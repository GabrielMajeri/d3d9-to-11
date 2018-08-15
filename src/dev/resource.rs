use winapi::shared::d3d9types::D3DRESOURCETYPE;

use super::Device;

/// Structure used as the base for all the D3D9 device resources.
/// Use the `impl_resource` macro to implement its functions in inherited classes.
pub struct Resource {
    /// Need to hold a reference back to the parent device.
    pub device: *const Device,
    /// Priority of this resource.
    /// Higher value indicates this resource should be evicted last from VRAM.
    pub priority: u32,
    /// The type of this resource.
    pub ty: D3DRESOURCETYPE,
}

impl Resource {
    /// Creates a new base resource structure.
    pub fn new(device: *const Device, ty: D3DRESOURCETYPE) -> Self {
        Self {
            device,
            priority: 0,
            ty,
        }
    }
}

macro_rules! impl_resource {
    ($name: ident) => {
        impl $name {
            /// Returns the parent device of this resource.
            pub fn device(&self) -> &Device {
                unsafe { &*self.resource.device }
            }

            /// Retrieves the immediate device context of the parent device.
            pub fn device_context(&self) -> &ID3D11DeviceContext {
                self.device().device_context()
            }
        }

        #[implementation(IUnknown, IDirect3DResource9)]
        impl $name {
            /// Retrieves the type of this resource.
            fn get_type(&self) -> D3DRESOURCETYPE {
                self.resource.ty
            }

            /// Returns the parent device.
            fn get_device(&self, ret: *mut *mut Device) -> Error {
                let ret = check_mut_ref(ret)?;
                *ret = com_ref(self.resource.device);
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
                let old = self.resource.priority;
                self.resource.priority = priority;
                old
            }

            /// Returns the priority of this resource.
            fn get_priority(&self) -> u32 {
                self.resource.priority
            }

            /// Pre loads resource to VRAM.
            fn pre_load(&self) {
                info!("Resource pre-loading is not yet implemented");
            }
        }
    }
}
