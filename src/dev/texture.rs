use winapi::{
    shared::{d3d9::*, d3d9types::*, windef::RECT},
    um::{
        unknwnbase::IUnknownVtbl,
        d3d11::*,
    }
};
use com_impl::{implementation, interface, ComInterface};
use comptr::ComPtr;

use super::{Device, Surface, resource::Resource};
use crate::{core::*, Error};

/// Structure containing an image and its mip sub-levels.
///
/// Closely matches the `ID3D11Texture2D` interface.
#[interface(IDirect3DTexture9)]
pub struct Texture {
    resource: Resource,
    texture: ComPtr<ID3D11Texture2D>,
}

impl Texture {
    /// Creates a new texture object.
    pub fn new(device: *const Device, texture: ComPtr<ID3D11Texture2D>) -> ComPtr<Self> {
        let texture = Self {
            __vtable: Box::new(Self::create_vtable()),
            __refs: Self::create_refs(),
            resource: Resource::new(device, D3DRTYPE_TEXTURE),
            texture,
        };

        unsafe { new_com_interface(texture) }
    }
}

impl_resource!(Texture);

#[implementation(IDirect3DResource9, IDirect3DBaseTexture9)]
impl Texture {
    fn set_l_o_d(&mut self, _lod: u32) -> u32 {
        unimplemented!()
    }
    fn get_l_o_d(&self) -> u32 {
        unimplemented!()
    }
    fn get_level_count(&self) -> u32 {
        unimplemented!()
    }
    fn set_auto_gen_filter_type(&mut self, _filter: D3DTEXTUREFILTERTYPE) -> Error {
        unimplemented!()
    }
    fn get_auto_gen_filter_type(&self) -> D3DTEXTUREFILTERTYPE {
        unimplemented!()
    }
    fn generate_mip_sub_levels(&mut self) {
        unimplemented!()
    }
}

#[implementation(IDirect3DBaseTexture9, IDirect3DTexture9)]
impl Texture {
    fn get_level_desc(&self, _level: u32, _desc: *mut D3DSURFACE_DESC) -> Error {
        unimplemented!()
    }
    fn get_surface_level(&self, _level: u32, _ret: *mut *mut Surface) -> Error {
        unimplemented!()
    }
    fn lock_rect(&mut self, _level: u32, _locked_rect: *mut D3DLOCKED_RECT, _r: *const RECT, _flags: u32) -> Error {
        unimplemented!()
    }
    fn unlock_rect(&mut self, _level: u32) -> Error {
        unimplemented!()
    }
    fn add_dirty_rect(&mut self, _r: *const RECT) -> Error {
        unimplemented!()
    }
}
