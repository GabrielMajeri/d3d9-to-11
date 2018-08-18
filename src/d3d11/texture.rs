use std::{mem, ptr};

use winapi::shared::d3d9types::*;
use winapi::um::d3d11::*;

use comptr::ComPtr;

use crate::core::{fmt::d3d_format_to_dxgi, msample::d3d9_to_dxgi_samples, *};
use crate::Result;

use super::util::d3d_usage_to_d3d11;

/// Wrapper for a D3D11 2D texture.
#[derive(Clone)]
pub struct Texture2D {
    texture: ComPtr<ID3D11Texture2D>,
}

impl Texture2D {
    /// Creates a new texture.
    pub fn new(
        device: &ID3D11Device,
        (width, height): (u32, u32),
        levels: u32,
        uflags: UsageFlags,
        fmt: D3DFORMAT,
        pool: MemoryPool,
    ) -> Result<Self> {
        let (usage, bind_flags, cpu_flags) = d3d_usage_to_d3d11(uflags, pool)?;

        let fmt = d3d_format_to_dxgi(fmt);

        let desc = D3D11_TEXTURE2D_DESC {
            Width: width,
            Height: height,
            MipLevels: levels,
            ArraySize: 1,
            Format: fmt,
            SampleDesc: d3d9_to_dxgi_samples(0, 0),
            Usage: usage,
            BindFlags: bind_flags,
            CPUAccessFlags: cpu_flags,
            MiscFlags: 0,
        };

        let texture = unsafe {
            let mut ptr = ptr::null_mut();

            let result = device.CreateTexture2D(&desc, ptr::null(), &mut ptr);
            check_hresult(result, "Failed to create 2D texture")?;

            ComPtr::new(ptr)
        };

        Ok(Self { texture })
    }

    /// Creates a new cube map texture.
    pub fn new_cube_texture(
        device: &ID3D11Device,
        dimension: u32,
        levels: u32,
        uflags: UsageFlags,
        fmt: D3DFORMAT,
        pool: MemoryPool,
    ) -> Result<Self> {
        let (usage, bind_flags, cpu_flags) = d3d_usage_to_d3d11(uflags, pool)?;
        let fmt = d3d_format_to_dxgi(fmt);

        let desc = D3D11_TEXTURE2D_DESC {
            Width: dimension,
            Height: dimension,
            MipLevels: levels,
            ArraySize: 6,
            Format: fmt,
            SampleDesc: d3d9_to_dxgi_samples(0, 0),
            Usage: usage,
            BindFlags: bind_flags,
            CPUAccessFlags: cpu_flags,
            MiscFlags: 0,
        };

        let texture = unsafe {
            let mut ptr = ptr::null_mut();

            let result = device.CreateTexture2D(&desc, ptr::null(), &mut ptr);
            check_hresult(result, "Failed to create cube texture")?;

            ComPtr::new(ptr)
        };

        Ok(Self { texture })
    }

    /// Creates a new render target.
    pub fn new_rt(
        device: &ID3D11Device,
        (width, height): (u32, u32),
        fmt: D3DFORMAT,
        ms_ty: D3DMULTISAMPLE_TYPE,
        ms_qlt: u32,
    ) -> Result<Self> {
        let fmt = d3d_format_to_dxgi(fmt);

        let desc = D3D11_TEXTURE2D_DESC {
            Width: width,
            Height: height,
            MipLevels: 1,
            ArraySize: 1,
            Format: fmt,
            SampleDesc: d3d9_to_dxgi_samples(ms_ty, ms_qlt),
            Usage: D3D11_USAGE_DEFAULT,
            BindFlags: D3D11_BIND_RENDER_TARGET,
            CPUAccessFlags: 0,
            MiscFlags: 0,
        };

        let texture = unsafe {
            let mut ptr = ptr::null_mut();

            let result = device.CreateTexture2D(&desc, ptr::null(), &mut ptr);
            check_hresult(result, "Failed to create render target texture")?;

            ComPtr::new(ptr)
        };

        Ok(Self { texture })
    }

    /// Creates a render target view from this texture.
    pub fn create_rt_view(&self, device: &ID3D11Device) -> Result<ComPtr<ID3D11RenderTargetView>> {
        let resource = self.as_resource();

        let view = unsafe {
            let mut ptr = ptr::null_mut();

            let result = device.CreateRenderTargetView(resource, ptr::null(), &mut ptr);
            check_hresult(result, "Failed to create render target view")?;

            ComPtr::new(ptr)
        };

        Ok(view)
    }

    /// Creates a new depth/stencil buffer.
    pub fn new_ds(
        device: &ID3D11Device,
        (width, height): (u32, u32),
        fmt: D3DFORMAT,
    ) -> Result<Self> {
        let fmt = d3d_format_to_dxgi(fmt);

        let desc = D3D11_TEXTURE2D_DESC {
            Width: width,
            Height: height,
            MipLevels: 1,
            ArraySize: 1,
            Format: fmt,
            SampleDesc: d3d9_to_dxgi_samples(0, 0),
            Usage: D3D11_USAGE_DEFAULT,
            BindFlags: D3D11_BIND_DEPTH_STENCIL,
            CPUAccessFlags: 0,
            MiscFlags: 0,
        };

        let texture = unsafe {
            let mut ptr = ptr::null_mut();

            let result = device.CreateTexture2D(&desc, ptr::null(), &mut ptr);
            check_hresult(result, "Failed to create depth/stencil texture")?;

            ComPtr::new(ptr)
        };

        Ok(Self { texture })
    }

    /// Creates a depth / stencil view from this texture.
    pub fn create_ds_view(&self, device: &ID3D11Device) -> Result<ComPtr<ID3D11DepthStencilView>> {
        let resource = self.as_resource();

        let view = unsafe {
            let mut ptr = ptr::null_mut();

            let result = device.CreateDepthStencilView(resource, ptr::null(), &mut ptr);
            check_hresult(result, "Failed to create depth / stencil view")?;

            ComPtr::new(ptr)
        };

        Ok(view)
    }

    /// Retrieves this texture as a resource.
    pub fn as_resource(&self) -> *mut ID3D11Resource {
        self.texture.upcast().as_mut()
    }

    /// Retrieves the description of this texture.
    pub fn desc(&self) -> D3D11_TEXTURE2D_DESC {
        unsafe {
            let mut desc = mem::uninitialized();
            self.texture.GetDesc(&mut desc);
            desc
        }
    }
}

impl From<ComPtr<ID3D11Texture2D>> for Texture2D {
    fn from(texture: ComPtr<ID3D11Texture2D>) -> Self {
        Self { texture }
    }
}
