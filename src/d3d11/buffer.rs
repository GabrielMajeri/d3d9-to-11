use std::ptr;

use winapi::shared::d3d9types::*;
use winapi::um::d3d11::*;

use comptr::ComPtr;

use crate::core::*;
use crate::Result;

use super::util::d3d_usage_to_d3d11;

/// Wrapper for a vertex/index buffer.
#[derive(Clone)]
pub struct Buffer {
    buffer: ComPtr<ID3D11Buffer>,
}

impl Buffer {
    /// Creates a vertex/index buffer.
    pub fn new(
        device: &ID3D11Device,
        len: u32,
        usage: u32,
        pool: D3DPOOL,
        bind_flags: u32,
    ) -> Result<Self> {
        let (usage, cpu_flags) = d3d_usage_to_d3d11(usage, pool)?;

        let desc = D3D11_BUFFER_DESC {
            ByteWidth: len,
            Usage: usage,
            BindFlags: bind_flags,
            CPUAccessFlags: cpu_flags,
            MiscFlags: 0,
            StructureByteStride: 0,
        };

        let buffer = unsafe {
            let mut ptr = ptr::null_mut();

            let result = device.CreateBuffer(&desc, ptr::null(), &mut ptr);
            check_hresult(result, "Failed to create buffer")?;

            ComPtr::new(ptr)
        };

        Ok(Self { buffer })
    }
}
