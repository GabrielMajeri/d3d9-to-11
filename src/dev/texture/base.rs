use winapi::shared::{d3d9::*, d3d9types::*};

use com_impl::{implementation, ComInterface};

use crate::core::*;
use crate::dev::{Device, Resource};
use crate::Error;

/// The common interface for all texture interfaces.
#[repr(C)]
pub struct BaseTexture {
    resource: Resource,
    // Number of subresource levels in this textures.
    levels: u32,
}

impl BaseTexture {
    /// Initializes a new base texture.
    pub fn new(
        device: *const Device,
        usage: UsageFlags,
        pool: MemoryPool,
        rtype: ResourceType,
        levels: u32,
    ) -> Self {
        Self {
            resource: Resource::new(device, usage, pool, rtype),
            levels,
        }
    }
}

impl std::ops::Deref for BaseTexture {
    type Target = Resource;
    fn deref(&self) -> &Resource {
        &self.resource
    }
}

impl ComInterface<IDirect3DResource9Vtbl> for BaseTexture {
    fn create_vtable() -> IDirect3DResource9Vtbl {
        Resource::create_vtable()
    }
}

#[implementation(IDirect3DBaseTexture9)]
impl BaseTexture {
    fn set_l_o_d(&mut self, _lod: u32) -> u32 {
        unimplemented!()
    }
    fn get_l_o_d(&self) -> u32 {
        unimplemented!()
    }

    /// Returns the count of mip levels in this texture.
    pub fn get_level_count(&self) -> u32 {
        self.levels
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
