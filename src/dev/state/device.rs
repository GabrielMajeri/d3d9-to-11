use std::collections::HashMap;
use std::{mem, ptr};

use winapi::shared::d3d9types::*;

use nalgebra::{self as na, Matrix4};

use crate::dev::shader::VertexDeclaration;
use crate::dev::*;

use super::*;

/// Structure containing all render state.
/// This includes pixel and vertex state.
///
/// For a list of all state we must keep track of, see:
/// https://docs.microsoft.com/en-us/windows/desktop/direct3d9/saving-vertex-states-with-a-stateblock
pub struct DeviceState {
    vertex: VertexState,
    pixel: PixelState,
    textures: [*mut BaseTexture; 20],
    viewport: D3DVIEWPORT9,
    transforms: HashMap<D3DTRANSFORMSTATETYPE, Matrix4<f32>>,
    material: D3DMATERIAL9,
}

impl DeviceState {
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

    pub fn set_sampler_state(&mut self, sampler: u32, ty: D3DSAMPLERSTATETYPE, value: u32) {
        // The 4 vertex texture samplers are in the 257-260 range.
        if D3DVERTEXTEXTURESAMPLER0 <= sampler && sampler <= D3DVERTEXTEXTURESAMPLER3 {
            // Map the vertex sampler to the [0; 3] range.
            let sampler = sampler - D3DVERTEXTEXTURESAMPLER0;
            self.vertex.set_sampler_state(sampler, ty, value);
        } else {
            self.pixel.set_sampler_state(sampler, ty, value)
        }
    }

    pub fn get_sampler_state(&self, sampler: u32, ty: D3DSAMPLERSTATETYPE) -> u32 {
        if D3DVERTEXTEXTURESAMPLER0 <= sampler && sampler <= D3DVERTEXTEXTURESAMPLER3 {
            let sampler = sampler - D3DVERTEXTEXTURESAMPLER0;
            self.vertex.get_sampler_state(sampler, ty)
        } else {
            self.pixel.get_sampler_state(sampler, ty)
        }
    }

    pub fn set_texture_stage_state(
        &mut self,
        stage: u32,
        ty: D3DTEXTURESTAGESTATETYPE,
        value: u32,
    ) {
        if D3DVERTEXTEXTURESAMPLER0 <= stage && stage <= D3DVERTEXTEXTURESAMPLER3 {
            let stage = stage - D3DVERTEXTEXTURESAMPLER0;
            self.vertex.set_texture_stage_state(stage, ty, value);
        } else {
            self.pixel.set_texture_stage_state(stage, ty, value);
        }
    }

    pub fn get_texture_stage_state(&self, stage: u32, ty: D3DTEXTURESTAGESTATETYPE) -> u32 {
        if D3DVERTEXTEXTURESAMPLER0 <= stage && stage <= D3DVERTEXTEXTURESAMPLER3 {
            let stage = stage - D3DVERTEXTEXTURESAMPLER0;
            self.vertex.get_texture_stage_state(stage, ty)
        } else {
            self.pixel.get_texture_stage_state(stage, ty)
        }
    }

    pub fn set_texture(&mut self, mut stage: u32, texture: *mut BaseTexture) {
        if D3DVERTEXTEXTURESAMPLER0 <= stage && stage <= D3DVERTEXTEXTURESAMPLER3 {
            stage = 16 + stage - D3DVERTEXTEXTURESAMPLER0;
        }

        if let Some(tx) = self.textures.get_mut(stage as usize) {
            *tx = texture;
        }
    }

    pub fn get_texture(&self, mut stage: u32) -> *mut BaseTexture {
        if D3DVERTEXTEXTURESAMPLER0 <= stage && stage <= D3DVERTEXTEXTURESAMPLER3 {
            stage = 16 + stage - D3DVERTEXTEXTURESAMPLER0;
        }

        self.textures
            .get(stage as usize)
            .cloned()
            .unwrap_or(ptr::null_mut())
    }

    pub fn set_vertex_declaration(&mut self, decl: &VertexDeclaration) {
        self.vertex.vertex_decl = decl;
    }

    pub fn get_vertex_declaration(&self) -> *const VertexDeclaration {
        self.vertex.vertex_decl
    }

    pub fn set_viewport(&mut self, vp: &D3DVIEWPORT9) {
        self.viewport = *vp;
    }

    pub fn get_viewport(&self) -> D3DVIEWPORT9 {
        self.viewport
    }

    pub fn set_transform(&mut self, ty: D3DTRANSFORMSTATETYPE, value: Matrix4<f32>) {
        self.transforms.insert(ty, value);
    }

    pub fn get_transform(&self, ty: D3DTRANSFORMSTATETYPE) -> Matrix4<f32> {
        self.transforms.get(&ty).cloned().unwrap_or_else(na::one)
    }

    pub fn set_material(&mut self, mat: &D3DMATERIAL9) {
        self.material = *mat;
    }

    pub fn get_material(&self) -> D3DMATERIAL9 {
        self.material
    }
}

impl Default for DeviceState {
    fn default() -> Self {
        let mut state = Self {
            vertex: VertexState::default(),
            pixel: PixelState::default(),
            textures: [ptr::null_mut(); 20],
            // The default viewport depends on the default render target's size.
            viewport: unsafe { mem::zeroed() },
            transforms: HashMap::with_capacity(4),
            material: unsafe { mem::zeroed() },
        };

        // The first texture stage has a different default state.
        state.pixel.ts[0].color_op = D3DTOP_MODULATE;
        state.pixel.ts[0].alpha_op = D3DTOP_SELECTARG1;

        state
    }
}
