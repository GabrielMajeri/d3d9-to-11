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
        dims: (u32, u32, u32),
        uflags: u32,
        fmt: D3DFORMAT,
        pool: D3DPOOL,
        mut ms_ty: D3DMULTISAMPLE_TYPE,
        mut ms_qlt: u32,
    ) -> Result<Self> {
        let mut bind_flags = 0;

        let (usage, cpu_flags) = if uflags & D3DUSAGE_RENDERTARGET != 0 {
            bind_flags |= D3D11_BIND_RENDER_TARGET;
            (D3D11_USAGE_DEFAULT, 0)
        } else if uflags & D3DUSAGE_DEPTHSTENCIL != 0 {
            bind_flags |= D3D11_BIND_DEPTH_STENCIL;
            ms_ty = 0;
            ms_qlt = 0;
            (D3D11_USAGE_DEFAULT, 0)
        } else {
            d3d_usage_to_d3d11(uflags, pool)?
        };

        let fmt = d3d_format_to_dxgi(fmt);

        if bind_flags == 0 && usage != D3D11_USAGE_STAGING {
            // Even if the app doesn't end up using this in a shader,
            // this is the only bind flag we could choose for it.
            bind_flags |= D3D11_BIND_SHADER_RESOURCE;
        }

        let desc = D3D11_TEXTURE2D_DESC {
            Width: dims.0,
            Height: dims.1,
            MipLevels: dims.2,
            ArraySize: 1,
            Format: fmt,
            SampleDesc: d3d9_to_dxgi_samples(ms_ty, ms_qlt),
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
