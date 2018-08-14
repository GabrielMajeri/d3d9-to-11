use std::mem;

use winapi::{
    shared::{d3d9::*, d3d9types::*, windef::RECT, winerror},
    um::{
        d3d11::*,
        unknwnbase::{IUnknown, IUnknownVtbl},
    },
};

use com_impl::{implementation, interface};
use comptr::ComPtr;

use super::Resource;
use crate::{core::*, Error};

/// Represents a 2D contiguous array of pixels.
#[interface(IUnknown, IDirect3DResource9, IDirect3DSurface9)]
pub struct Surface {
    parent: Resource,
    // Reference to the texture we own, or our parent texture.
    texture: ComPtr<ID3D11Texture2D>,
    // An index representing the sub-resource we are owning.
    // Can be 0 to indicate a top-level resource.
    subresource: u32,
}

impl Surface {
    /// Creates a new surface from a D3D11 2D texture.
    ///
    /// This constructor is usually called by all other surface constructors.
    pub fn from_texture(
        device: ComPtr<IDirect3DDevice9>,
        texture: ComPtr<ID3D11Texture2D>,
        subresource: u32,
    ) -> *mut IDirect3DSurface9 {
        let mut surface = Self {
            __vtable: Self::create_vtable(),
            __refs: Self::create_refs(),
            parent: Resource::new(device, D3DRTYPE_SURFACE),
            texture,
            subresource,
        };

        // Fix up the vtables.
        surface.__vtable.parent.parent = Self::__create_IUnknownVtbl();

        unsafe { new_com_interface(surface) }
    }

    /// Inherit Resource's implementation.
    #[allow(non_snake_case)]
    fn __create_IDirect3DResource9Vtbl() -> IDirect3DResource9Vtbl {
        Resource::__create_IDirect3DResource9Vtbl()
    }
}

#[implementation(IDirect3DResource9, IDirect3DSurface9)]
impl Surface {
    /// Gets the container of this resource.
    fn get_container() {
        unimplemented!()
    }

    /// Retrieves a description of this surface.
    fn get_desc(&self, _surface: *mut D3DSURFACE_DESC) -> Error {
        unimplemented!()
    }

    // -- Memory mapping functions --

    /// Locks a rectangular array of pixels and maps their memory.
    fn lock_rect(&mut self, ret: *mut D3DLOCKED_RECT, _r: *const RECT, flags: u32) -> Error {
        let ret = check_mut_ref(ret)?;
        // TODO: maybe track dirty regions for efficiency.

        // Try to map the subresource.
        let ctx = self.parent.device_context();
        let mapped = unsafe {
            let resource = self.texture.upcast().as_mut();

            let mut ty = 0;

            if flags & D3DLOCK_READONLY != 0 {
                ty |= D3D11_MAP_READ;
            } else {
                ty |= D3D11_MAP_READ_WRITE;
            }

            // Note: we do not validate that the texture was created
            // with dynamic usage, since D3D11 will validate that for us.
            if flags & D3DLOCK_DISCARD != 0 {
                ty |= D3D11_MAP_WRITE_DISCARD;
            }

            if flags & D3DLOCK_NOOVERWRITE != 0 {
                ty |= D3D11_MAP_WRITE_NO_OVERWRITE;
            }

            let mut fl = 0;

            if flags & D3DLOCK_DONOTWAIT != 0 {
                fl |= D3D11_MAP_FLAG_DO_NOT_WAIT;
            }

            let mut mapped = mem::uninitialized();

            let result = ctx.Map(resource, ty, fl, self.subresource, &mut mapped);

            match result {
                0 => Ok(mapped),
                winerror::DXGI_ERROR_WAS_STILL_DRAWING => Err(Error::WasStillDrawing),
                hr => Err(check_hresult!(hr, "Failed to map surface")),
            }
        }?;

        // TODO: we need special handling for pitch with DXT texture formats.
        *ret = D3DLOCKED_RECT {
            Pitch: mapped.RowPitch as i32,
            pBits: mapped.pData,
        };

        Error::Success
    }

    /// Unlocks the locked rectangle of memory.
    fn unlock_rect(&self) -> Error {
        let ctx = self.parent.device_context();

        let resource = self.texture.upcast().as_mut();

        unsafe {
            ctx.Unmap(resource, self.subresource);
        }

        Error::Success
    }

    // -- GDI interop functions --

    /// Retrieves the device context associated with this surface.
    fn get_d_c() {
        unimplemented!()
    }

    /// Releases a device context associated with this surface.
    fn release_d_c() {
        unimplemented!()
    }
}
