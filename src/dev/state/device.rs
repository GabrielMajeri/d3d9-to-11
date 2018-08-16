use winapi::shared::d3d9types::*;

use crate::Error;

/// Structure containing all render state.
/// This includes pixel and vertex state.
///
/// For a list of all state we must keep track of, see:
/// https://docs.microsoft.com/en-us/windows/desktop/direct3d9/saving-vertex-states-with-a-stateblock
pub struct DeviceState {}

impl DeviceState {
    /// Creates a new empty device state.
    pub(super) fn empty() -> Self {
        Self {}
    }

    pub fn set_render_state(&mut self, _state: D3DRENDERSTATETYPE, _value: u32) -> Error {
        unimplemented!()
    }

    pub fn get_render_state(&self, _state: D3DRENDERSTATETYPE) -> u32 {
        unimplemented!()
    }
}

impl Default for DeviceState {
    fn default() -> Self {
        Self {}
    }
}
