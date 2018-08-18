use std::sync::atomic::{AtomicU32, Ordering};

use winapi::shared::{d3d9::*, d3d9types::*, guiddef::GUID, windef::RECT};
use winapi::um::d3d11::*;
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

use com_impl::{implementation, interface, ComInterface};
use comptr::ComPtr;

use crate::core::{fmt::dxgi_format_to_d3d, msample::dxgi_samples_to_d3d9, *};
use crate::d3d11;
use crate::Error;

use super::{Device, Resource, Texture};

/// Represents a 2D contiguous array of pixels.
#[interface(IDirect3DSurface9)]
pub struct Surface {
    resource: Resource,
    refs: AtomicU32,
    // Reference to the texture we own, or our parent texture.
    texture: d3d11::Texture2D,
    // An index representing the sub-resource we are owning.
    // Can be 0 to indicate a top-level resource.
    subresource: u32,
    // Extra data required for this surface type.
    data: SurfaceData,
}

/// Extra information required to fully describe a surface.
///
/// In D3D9, a surface can represent quite a lot of things,
/// so this enum is used to store the data required for each kind.
pub enum SurfaceData {
    /// This is an ordinary surface.
    None,
    /// This surface is owning a render target.
    RenderTarget(ComPtr<ID3D11RenderTargetView>),
    /// This surface is owning a depth / stencil buffer.
    DepthStencil(ComPtr<ID3D11DepthStencilView>),
    /// This surface is a mip level of a bigger 2D texture.
    SubTexture(*const Texture),
}

impl Surface {
    /// Creates a new surface from a D3D11 2D texture, and possibly some extra data.
    pub fn new(
        device: *const Device,
        texture: d3d11::Texture2D,
        subresource: u32,
        pool: D3DPOOL,
        data: SurfaceData,
    ) -> ComPtr<Self> {
        let surface = Self {
            __vtable: Box::new(Self::create_vtable()),
            resource: Resource::new(device, pool, D3DRTYPE_SURFACE),
            refs: AtomicU32::new(1),
            texture,
            subresource,
            data,
        };

        unsafe { new_com_interface(surface) }
    }

    /// Retrieves a reference to the subresource this surface represents.
    pub fn subresource(&mut self) -> (*mut ID3D11Resource, u32) {
        let resource = self.texture.as_resource();
        (resource, self.subresource)
    }

    /// If this surface is a render target, retrieves the associated RT view.
    pub fn render_target_view(&self) -> Option<&mut ID3D11RenderTargetView> {
        if let SurfaceData::RenderTarget(ref view) = self.data {
            Some(view.as_mut())
        } else {
            None
        }
    }

    /// If this surface is a depth / stencil buffer, retrieves the associated DS view.
    pub fn depth_stencil_view(&self) -> Option<&mut ID3D11DepthStencilView> {
        if let SurfaceData::DepthStencil(ref view) = self.data {
            Some(view.as_mut())
        } else {
            None
        }
    }

    /// If this surface is a mip level of a texture, retrieves a reference to the texture.
    pub fn subtexture(&self) -> Option<&Texture> {
        if let SurfaceData::SubTexture(texture) = self.data {
            Some(unsafe { &*texture })
        } else {
            None
        }
    }
}

impl std::ops::Deref for Surface {
    type Target = Resource;
    fn deref(&self) -> &Resource {
        &self.resource
    }
}

impl_iunknown!(struct Surface: IUnknown, IDirect3DResource9, IDirect3DSurface9);

impl ComInterface<IDirect3DResource9Vtbl> for Surface {
    fn create_vtable() -> IDirect3DResource9Vtbl {
        let mut vtbl: IDirect3DResource9Vtbl = Resource::create_vtable();
        vtbl.parent = Self::create_vtable();
        vtbl
    }
}

#[implementation(IDirect3DSurface9)]
impl Surface {
    /// Gets the container of this resource.
    fn get_container(&self, _riid: &GUID, ret: *mut usize) -> Error {
        let ret = check_mut_ref(ret)?;

        *ret = if let SurfaceData::SubTexture(texture) = self.data {
            com_ref(texture) as usize
        } else {
            com_ref(self.device()) as usize
        };

        Error::Success
    }

    /// Retrieves a description of this surface.
    pub fn get_desc(&self, ret: *mut D3DSURFACE_DESC) -> Error {
        let ret = check_mut_ref(ret)?;

        let desc = self.texture.desc();

        ret.Width = desc.Width;
        ret.Height = desc.Height;

        ret.Format = dxgi_format_to_d3d(desc.Format);
        ret.Type = D3DRTYPE_SURFACE;

        use self::SurfaceData::*;
        ret.Usage = match self.data {
            RenderTarget(_) => D3DUSAGE_RENDERTARGET,
            DepthStencil(_) => D3DUSAGE_DEPTHSTENCIL,
            SubTexture(tex) => unsafe { (*tex).usage() },
            None => 0,
        };

        ret.Pool = self.pool();

        let (ms_ty, ms_qlt) = dxgi_samples_to_d3d9(desc.SampleDesc);
        ret.MultiSampleType = ms_ty;
        ret.MultiSampleQuality = ms_qlt;

        Error::Success
    }

    // -- Memory mapping functions --

    fn lock_rect(&mut self, ret: *mut D3DLOCKED_RECT, r: *const RECT, flags: u32) -> Error {
        self.subtexture()
            .map(|tex| tex.lock_rect(self.subresource, ret, r, flags))
            .ok_or_else(|| {
                error!("Memory mapping is only implemented for (sub)textures");
                Error::InvalidCall
            })?
    }

    fn unlock_rect(&self) -> Error {
        self.subtexture()
            .map(|tex| tex.unlock_rect(self.subresource))
            .ok_or(Error::InvalidCall)?
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
