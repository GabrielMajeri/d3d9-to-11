use winapi::shared::d3d9types::*;

use crate::dev::shader::VertexDeclaration;

use super::{PixelState, VertexState};

/// Structure containing all render state.
/// This includes pixel and vertex state.
///
/// For a list of all state we must keep track of, see:
/// https://docs.microsoft.com/en-us/windows/desktop/direct3d9/saving-vertex-states-with-a-stateblock
pub struct DeviceState {
    vertex: VertexState,
    pixel: PixelState,
}

impl DeviceState {
    /// Creates a new empty device state.
    pub(super) fn empty() -> Self {
        Self {
            vertex: VertexState::empty(),
            pixel: PixelState::empty(),
        }
    }

    pub fn set_render_state(&mut self, state: D3DRENDERSTATETYPE, value: u32) {
        self.vertex.set_render_state(state, value);
        self.pixel.set_render_state(state, value);
    }

    pub fn get_render_state(&self, state: D3DRENDERSTATETYPE) -> u32 {
        self.vertex
            .get_render_state(state)
            .or_else(|| self.pixel.get_render_state(state))
            .unwrap_or_default()
    }

    pub fn set_vertex_declaration(&mut self, decl: &VertexDeclaration) {
        self.vertex.vertex_decl = Some(decl);
    }

    pub fn get_vertex_declaration(&self) -> *const VertexDeclaration {
        self.vertex.vertex_decl.unwrap()
    }
}

impl Default for DeviceState {
    fn default() -> Self {
        Self {
            vertex: VertexState::default(),
            pixel: PixelState::default(),
        }
    }
}
