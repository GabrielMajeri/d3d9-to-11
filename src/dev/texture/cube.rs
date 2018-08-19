use std::ptr;
use std::sync::atomic::{AtomicU32, Ordering};

use winapi::shared::{d3d9::*, d3d9types::*, windef::RECT};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

use com_impl::{implementation, interface, ComInterface};
use comptr::ComPtr;

use crate::dev::*;
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
    /// Returns the description of a mip map level of a face.
    fn get_level_desc(&self, level: u32, desc: *mut D3DSURFACE_DESC) -> Error {
        let surface = {
            let mut ptr = ptr::null_mut();
            // We can use any face, since they are all equal.
            self.get_cube_map_surface(0, level, &mut ptr)?;
            ComPtr::new(ptr)
        };

        surface.get_desc(desc)
    }

    /// Retrieves a face of this cube map.
    fn get_cube_map_surface(&self, face: u32, level: u32, ret: *mut *mut Surface) -> Error {
        let ret = check_mut_ref(ret)?;
        let levels = self.level_count();

        if face >= 6 {
            return Error::InvalidCall;
        }

        if level >= levels {
            return Error::InvalidCall;
        }

        let device = self.device();
        let texture = self.texture.clone();
        let usage = self.usage();
        let pool = self.pool();
        let subres = self.texture.calc_subresource(level, face, levels);
        let data = SurfaceData::SubResource(subres);

        *ret = Surface::new(device, texture, usage, pool, data).into();

        Error::Success
    }

    /// Maps a face of this cube map to memory.
    fn lock_rect(
        &self,
        face: u32,
        level: u32,
        ret: *mut D3DLOCKED_RECT,
        _r: *const RECT,
        flags: LockFlags,
    ) -> Error {
        let ret = check_mut_ref(ret)?;

        let resource = self.texture.as_resource();
        let levels = self.level_count();
        let subres = self.texture.calc_subresource(level, face, levels);
        let ctx = self.device_context();

        *ret = ctx.map(resource, subres, flags, self.usage())?;

        Error::Success
    }

    /// Unmaps a face of this cube map.
    fn unlock_rect(&self, face: u32, level: u32) -> Error {
        let resource = self.texture.as_resource();
        let levels = self.level_count();
        let subres = self.texture.calc_subresource(level, face, levels);
        let ctx = self.device_context();

        ctx.unmap(resource, subres);

        Error::Success
    }

    fn add_dirty_rect(&mut self, _face: u32, r: *const RECT) -> Error {
        let _r = check_ref(r)?;
        warn!("AddDirtyRect is not implemented");
        Error::Success
    }
}
