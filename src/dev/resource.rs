use winapi::shared::d3d9::*;
use winapi::um::unknwnbase::IUnknownVtbl;

use com_impl::{implementation, ComInterface};

use crate::core::*;
use crate::d3d11;
use crate::Error;

use super::Device;

/// Structure used as the base for all the D3D9 device resources.
/// Use the `impl_resource` macro to implement its functions in inherited classes.
pub struct Resource {
    /// Need to hold a reference back to the parent device.
    device: *const Device,
    /// Usage flags of this resource.
    usage: UsageFlags,
    /// Memory pool from which this resource was allocated.
    pool: MemoryPool,
    /// The type of this resource.
    ty: ResourceType,
    /// Priority of this resource.
    /// Higher value indicates this resource should be evicted last from VRAM.
    priority: u32,
}

impl Resource {
    /// Creates a new base resource structure.
    pub fn new(
        device: *const Device,
        usage: UsageFlags,
        pool: MemoryPool,
        ty: ResourceType,
    ) -> Self {
        Self {
            device,
            usage,
            pool,
            ty,
            priority: 0,
        }
    }

    /// Returns the parent device of this resource.
    pub fn device(&self) -> &Device {
        unsafe { &*self.device }
    }

    /// Retrieves the immediate device context of the parent device.
    pub fn device_context(&self) -> &d3d11::DeviceContext {
        self.device().device_context()
    }

    /// Retrieves the usage flags of this resource.
    pub fn usage(&self) -> UsageFlags {
        self.usage
    }

    /// Retrieves the memory pool in which this resource belongs.
    pub fn pool(&self) -> MemoryPool {
        self.pool
    }
}

impl ComInterface<IUnknownVtbl> for Resource {
    fn create_vtable() -> IUnknownVtbl {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
struct Thunk {
    __vtbl: usize,
    rsrc: Resource,
}

impl std::ops::Deref for Thunk {
    type Target = Resource;
    fn deref(&self) -> &Resource {
        &self.rsrc
    }
}

impl std::ops::DerefMut for Thunk {
    type Target = Resource;
    fn deref_mut(&mut self) -> &mut Resource {
        &mut self.rsrc
    }
}

#[implementation(IDirect3DResource9)]
impl Resource {
    /// Retrieves the type of this resource.
    fn get_type(self: &Thunk) -> ResourceType {
        self.ty
    }

    /// Returns the parent device.
    fn get_device(self: &Thunk, ret: *mut *mut Device) -> Error {
        let ret = check_mut_ref(ret)?;
        *ret = com_ref(self.device);
        Error::Success
    }

    fn set_private_data(self: &Thunk) {
        unimplemented!()
    }

    fn get_private_data(self: &Thunk) {
        unimplemented!()
    }

    fn free_private_data(self: &Thunk) {
        unimplemented!()
    }

    // TODO: the functions below could be used to improve performance.

    /// Updates this resource's priority.
    fn set_priority(self: &mut Thunk, priority: u32) -> u32 {
        let old = self.priority;
        self.priority = priority;
        old
    }

    /// Returns the priority of this resource.
    fn get_priority(self: &mut Thunk) -> u32 {
        self.priority
    }

    /// Pre loads resource to VRAM.
    fn pre_load(self: &Thunk) {
        info!("Resource pre-loading is not yet implemented");
    }
}
