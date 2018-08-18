use std::sync::atomic::{AtomicU32, Ordering};

use winapi::shared::{d3d9::*, windef::RECT};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

use com_impl::{implementation, interface, ComInterface};
use comptr::ComPtr;

use crate::dev::Device;
use crate::Error;
use crate::{core::*, d3d11};

use super::BaseTexture;

/// Cube map texture.
///
/// In other words, a texture array with 6 sub-textures.
#[interface(IDirect3DCubeTexture9)]
pub struct CubeTexture {
    base: BaseTexture,
    refs: AtomicU32,
    texture: d3d11::Texture2D,
}

impl CubeTexture {
    /// Creates a new cube texture.
    pub fn new(
        device: *const Device,
        texture: d3d11::Texture2D,
        levels: u32,
        usage: UsageFlags,
        pool: MemoryPool,
    ) -> ComPtr<Self> {
        let tc = Self {
            __vtable: Box::new(Self::create_vtable()),
            base: BaseTexture::new(device, usage, pool, ResourceType::CubeTexture, levels),
            refs: AtomicU32::new(1),
            texture,
        };

        unsafe { new_com_interface(tc) }
    }
}

impl std::ops::Deref for CubeTexture {
    type Target = BaseTexture;
    fn deref(&self) -> &BaseTexture {
        &self.base
    }
}

impl_iunknown!(struct CubeTexture: IUnknown, IDirect3DResource9, IDirect3DBaseTexture9, IDirect3DCubeTexture9);

impl ComInterface<IDirect3DBaseTexture9Vtbl> for CubeTexture {
    fn create_vtable() -> IDirect3DBaseTexture9Vtbl {
        let mut vtbl: IDirect3DBaseTexture9Vtbl = BaseTexture::create_vtable();
        vtbl.parent.parent = Self::create_vtable();
        vtbl
    }
}

#[implementation(IDirect3DCubeTexture9)]
impl CubeTexture {
    fn get_level_desc() {
        unimplemented!()
    }
    fn get_cube_map_surface() {
        unimplemented!()
    }
    fn lock_rect() {
        unimplemented!()
    }
    fn unlock_rect() {
        unimplemented!()
    }

    fn add_dirty_rect(&mut self, _face: u32, r: *const RECT) -> Error {
        let _r = check_ref(r)?;
        warn!("AddDirtyRect is not implemented");
        Error::Success
    }
}
