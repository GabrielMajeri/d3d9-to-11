use std::{mem, ops, ptr};

use winapi::shared::{d3d9types::*, winerror};
use winapi::um::d3d11::*;

use comptr::ComPtr;

use crate::core::*;
use crate::{Error, Result};

/// Wrapper for a D3D11 immediate context.
#[derive(Clone)]
pub struct DeviceContext {
    ctx: ComPtr<ID3D11DeviceContext>,
}

impl DeviceContext {
    /// Retrieve's a device's immediate context.
    pub fn new(device: &ID3D11Device) -> Self {
        let ctx = unsafe {
            let mut ptr = ptr::null_mut();
            device.GetImmediateContext(&mut ptr);
            ComPtr::new(ptr)
        };

        Self { ctx }
    }

    /// Maps a resource.
    pub fn map(
        &self,
        res: *mut ID3D11Resource,
        subres: u32,
        flags: u32,
        usage: u32,
    ) -> Result<D3DLOCKED_RECT> {
        let map_flags = if usage & D3DUSAGE_WRITEONLY != 0 {
            // NOOVERWRITE must come first, since in D3D11 it's a superset of discard.
            if flags & D3DLOCK_NOOVERWRITE != 0 {
                D3D11_MAP_WRITE_NO_OVERWRITE
            } else if flags & D3DLOCK_DISCARD != 0 {
                D3D11_MAP_WRITE_DISCARD
            } else {
                D3D11_MAP_WRITE
            }
        } else {
            // Either the app forgot to use writeonly, or it really wants to
            // read the data, in which case, we can only hope it works.

            // TODO: implement some stricter checks by checking the resource's memory pool,
            // then remove this warning.
            error!("Reading data from a resource might not work");

            if flags & D3DLOCK_READONLY != 0 {
                D3D11_MAP_READ
            } else {
                D3D11_MAP_READ_WRITE
            }
        };

        let gpu_flags = {
            let mut fl = 0;

            if flags & D3DLOCK_DONOTWAIT != 0 {
                fl |= D3D11_MAP_FLAG_DO_NOT_WAIT;
            }

            fl
        };

        // Try to map the subresource.
        let mapped = unsafe {
            let mut buf = mem::uninitialized();
            let result = self.Map(res, subres, map_flags, gpu_flags, &mut buf);

            match result {
                0 => Ok(buf),
                winerror::DXGI_ERROR_WAS_STILL_DRAWING => Err(Error::WasStillDrawing),
                hr => Err(check_hresult(hr, "Failed to map resource")),
            }
        }?;

        // TODO: we need special handling for pitch with DXT texture formats.

        let mapped = D3DLOCKED_RECT {
            Pitch: mapped.RowPitch as i32,
            pBits: mapped.pData,
        };

        Ok(mapped)
    }

    /// Unmaps a resource.
    pub fn unmap(&self, res: *mut ID3D11Resource, subres: u32) {
        unsafe {
            self.Unmap(res, subres);
        }
    }
}

impl ops::Deref for DeviceContext {
    type Target = ID3D11DeviceContext;
    fn deref(&self) -> &ID3D11DeviceContext {
        &self.ctx
    }
}
