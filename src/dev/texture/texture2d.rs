use std::ptr;
use std::sync::atomic::{AtomicU32, Ordering};

use winapi::shared::{d3d9::*, d3d9types::*, windef::RECT};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

use com_impl::{implementation, interface, ComInterface};
use comptr::ComPtr;

use crate::dev::*;
use crate::{core::*, d3d11, Error};

use super::BaseTexture;

/// Structure containing an image and its mip sub-levels.
///
/// Closely matches the `ID3D11Texture2D` interface.
#[interface(IDirect3DTexture9)]
pub struct Texture {
    base: BaseTexture,
    refs: AtomicU32,
    texture: d3d11::Texture2D,
}

impl Texture {
    /// Creates a new texture object.
    pub fn new(
        device: *const Device,
        pool: MemoryPool,
        texture: d3d11::Texture2D,
        levels: u32,
        usage: UsageFlags,
    ) -> ComPtr<Self> {
        let texture = Self {
            __vtable: Box::new(Self::create_vtable()),
            base: BaseTexture::new(device, usage, pool, ResourceType::Texture, levels),
            refs: AtomicU32::new(1),
            texture,
        };

        unsafe { new_com_interface(texture) }
    }
}

impl std::ops::Deref for Texture {
    type Target = BaseTexture;
    fn deref(&self) -> &BaseTexture {
        &self.base
    }
}

impl_iunknown!(struct Texture: IUnknown, IDirect3DResource9, IDirect3DBaseTexture9, IDirect3DTexture9);

impl ComInterface<IDirect3DBaseTexture9Vtbl> for Texture {
    fn create_vtable() -> IDirect3DBaseTexture9Vtbl {
        let mut vtbl: IDirect3DBaseTexture9Vtbl = BaseTexture::create_vtable();
        vtbl.parent.parent = Self::create_vtable();
        vtbl
    }
}

#[implementation(IDirect3DTexture9)]
impl Texture {
    /// Retrieves the description of a certain mip level.
    fn get_level_desc(&self, level: u32, desc: *mut D3DSURFACE_DESC) -> Error {
        let surface = {
            let mut ptr = ptr::null_mut();
            self.get_surface_level(level, &mut ptr)?;
            ComPtr::new(ptr)
        };

        surface.get_desc(desc)
    }

    /// Retrieves a surface representing a mip level of this texture.
    fn get_surface_level(&self, level: u32, ret: *mut *mut Surface) -> Error {
        let ret = check_mut_ref(ret)?;

        if level >= self.level_count() {
            return Error::InvalidCall;
        }

        let device = self.device();
        let texture = self.texture.clone();
        let usage = self.usage();
        let pool = self.pool();
        let data = SurfaceData::SubResource(level);

        *ret = Surface::new(device, texture, usage, pool, data).into();

        Error::Success
    }

    /// Locks a texture and maps its memory.
    pub fn lock_rect(
        &self,
        level: u32,
        ret: *mut D3DLOCKED_RECT,
        // TODO: maybe track dirty regions for efficiency.
        _r: *const RECT,
        flags: LockFlags,
    ) -> Error {
        let ret = check_mut_ref(ret)?;

        let resource = self.texture.as_resource();
        let ctx = self.device_context();

        *ret = ctx.map(resource, level, flags, self.usage())?;

        Error::Success
    }

    /// Unlocks the locked rectangle of memory.
    pub fn unlock_rect(&self, level: u32) -> Error {
        let resource = self.texture.as_resource();
        let ctx = self.device_context();

        ctx.unmap(resource, level);

        Error::Success
    }

    fn add_dirty_rect(&mut self, r: *const RECT) -> Error {
        let _r = check_ref(r)?;
        warn!("AddDirtyRect is not implemented");
        Error::Success
    }
}
