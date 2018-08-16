use std::sync::atomic::{AtomicU32, Ordering};

use winapi::shared::{d3d9::*, d3d9types::*};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

use com_impl::{implementation, interface, ComInterface};
use comptr::ComPtr;

use crate::dev::Device;
use crate::{core::*, Error, Result};

use super::{DeviceState, PixelState, VertexState};

/// Object which records some portions of a device's state.
#[interface(IDirect3DStateBlock9)]
pub struct StateBlock {
    refs: AtomicU32,
    device: *mut Device,
    state: RenderState,
}

/// Enum containing (possibly a part of) the render state.
enum RenderState {
    All(DeviceState),
    Pixel(PixelState),
    Vertex(VertexState),
}

impl StateBlock {
    /// Creates a new empty state block.
    pub fn new(device: &mut Device, ty: D3DSTATEBLOCKTYPE) -> Result<ComPtr<Self>> {
        let state = match ty {
            D3DSBT_ALL => RenderState::All(DeviceState::empty()),
            D3DSBT_PIXELSTATE => RenderState::Pixel(PixelState::empty()),
            D3DSBT_VERTEXSTATE => RenderState::Vertex(VertexState::empty()),
            _ => return Err(Error::InvalidCall),
        };

        let sb = Self {
            __vtable: Box::new(Self::create_vtable()),
            refs: AtomicU32::new(1),
            device,
            state,
        };

        Ok(unsafe { new_com_interface(sb) })
    }
}

impl_iunknown!(struct StateBlock: IUnknown, IDirect3DStateBlock9);

#[implementation(IDirect3DStateBlock9)]
impl StateBlock {
    /// Retrieves the device which owns this tate block.
    fn get_device(&self, ret: *mut *mut Device) -> Error {
        let ret = check_mut_ref(ret)?;
        *ret = com_ref(self.device);
        Error::Success
    }

    /// Captures the current values for the state which is already in this block.
    fn capture(&mut self) -> Error {
        unimplemented!()
    }

    /// Applies the contained state to the parent device.
    fn apply(&self) -> Error {
        unimplemented!()
    }
}
