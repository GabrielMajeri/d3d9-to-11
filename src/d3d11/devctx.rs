use std::{ops, ptr};

use winapi::um::d3d11::*;

use comptr::ComPtr;

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
}

impl ops::Deref for DeviceContext {
    type Target = ID3D11DeviceContext;
    fn deref(&self) -> &ID3D11DeviceContext {
        &self.ctx
    }
}
