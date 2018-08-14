use std::mem;

use winapi::{
    shared::{d3d9::*, d3d9types::*},
    um::{d3d11::ID3D11DeviceContext, unknwnbase::IUnknownVtbl},
};

use com_impl::implementation;
use comptr::ComPtr;

use super::Device;
use crate::{core::*, Error};

/// Structure used as the base for all the D3D9 device resources.
pub struct Resource {
    // Need to hold a strong reference back to the parent device.
    device: ComPtr<IDirect3DDevice9>,
    // Priority of this resource.
    // Higher value indicates this resource should be evicted last from VRAM.
    priority: u32,
    /// The type of this resource.
    ty: D3DRESOURCETYPE,
}

impl Resource {
    /// Creates a new resource.
    /// Should only be called by structures which inherit from the IDirect3DResource9 interface.
    pub fn new(device: ComPtr<IDirect3DDevice9>, ty: D3DRESOURCETYPE) -> Self {
        Self {
            device,
            priority: 0,
            ty,
        }
    }

    /// Retrieves a reference to the D3D11 device which owns this resource.
    pub fn device(&self) -> &Device {
        // We know that the device that we were created with was from our own library.
        let dev_ptr = self.device.as_ref() as *const _ as *const Device;
        unsafe { &*dev_ptr }
    }

    /// Retrieves the immediate device context of the parent device.
    pub fn device_context(&self) -> &ID3D11DeviceContext {
        self.device().device_context()
    }

    #[allow(non_snake_case)]
    fn __create_IUnknownVtbl() -> IUnknownVtbl {
        unsafe { mem::uninitialized() }
    }
}

#[implementation(IUnknown, IDirect3DResource9)]
impl Resource {
    /// Retrieves the type of this resource.
    fn get_type(&self) -> D3DRESOURCETYPE {
        self.ty
    }

    /// Returns the parent device.
    fn get_device(&self, ret: *mut *mut IDirect3DDevice9) -> Error {
        let ret = check_mut_ref(ret)?;
        *ret = self.device.clone().into();
        Error::Success
    }

    fn set_private_data() {
        unimplemented!()
    }

    fn get_private_data() {
        unimplemented!()
    }

    fn free_private_data() {
        unimplemented!()
    }

    // TODO: the functions below could be used to improve performance.

    /// Updates this resource's priority.
    fn set_priority(&mut self, priority: u32) -> u32 {
        let old = self.priority;
        self.priority = priority;
        old
    }

    /// Returns the priority of this resource.
    fn get_priority(&self) -> u32 {
        self.priority
    }

    /// Pre loads resource to VRAM.
    fn pre_load() {
        info!("Resource pre-loading is not yet implemented");
    }
}
