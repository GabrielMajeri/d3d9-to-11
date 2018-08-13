use winapi::{
    shared::{d3d9::*, d3d9caps::D3DCAPS9, d3d9types::*},
    um::d3d11::ID3D11Device,
    um::unknwnbase::{IUnknown, IUnknownVtbl},
};

use com_impl::{implementation, interface};
use comptr::ComPtr;

use crate::core::*;
use crate::{Error, Result};

/// Structure representing a logical graphics device.
#[interface(IUnknown, IDirect3DDevice9)]
pub struct Device {
    // Interface which created this device.
    parent: ComPtr<IDirect3D9>,
    // The adapter this device represents.
    //
    // Since D3D11 is thread-safe, we allow multiple logical devices
    // to share the same adapter.
    //
    // Since we own a handle to the parent, the ref is static.
    adapter: &'static Adapter,
    // The equivalent interface from D3D11.
    device: ComPtr<ID3D11Device>,
    // Store the creation params, since the app might request them later.
    creation_params: D3DDEVICE_CREATION_PARAMETERS,
}

impl Device {
    /// Creates a new device.
    pub fn new(
        parent: ComPtr<IDirect3D9>,
        adapter: &Adapter,
        cp: D3DDEVICE_CREATION_PARAMETERS,
        _pp: &mut D3DPRESENT_PARAMETERS,
    ) -> Result<Self> {
        // Need to work around the lifetime system,
        // Rust cannot know we share ownership of the device.
        let adapter = unsafe { &*(adapter as *const Adapter) };

        let device = adapter.device();

        let device = Self {
            __vtable: Self::create_vtable(),
            __refs: Self::create_refs(),
            parent,
            adapter,
            device,
            creation_params: cp,
        };

        Ok(device)
    }
}

#[implementation(IUnknown, IDirect3DDevice9)]
impl Device {
    // -- Device status functions --

    /// Checks that the device has not yet been lost / reset.
    fn test_cooperative_level() -> Error {
        // Even if the device were lost, we wouldn't be able to do much.
        Error::Success
    }

    /// Determines how much graphics memory is available.
    fn get_available_texture_mem(&self) -> u32 {
        self.adapter.available_memory()
    }

    /// Asks the driver to evict all managed resources from VRAM.
    fn evict_managed_resources() -> Error {
        // Do nothing. The D3D11 driver handles everything.
        Error::Success
    }

    // -- Creation parameters functions --

    /// Returns a reference to the parent interface.
    fn get_direct_3_d(&self, ptr: *mut *mut IDirect3D9) -> Error {
        let ptr = check_mut_ref(ptr)?;

        *ptr = self.parent.clone().into();

        Error::Success
    }

    /// Returns the caps of this device.
    fn get_device_caps(&self, caps: *mut D3DCAPS9) -> Error {
        let caps = check_mut_ref(caps)?;

        *caps = self.adapter.caps();

        Error::Success
    }

    /// Returns the creation parameters of this device.
    fn get_creation_parameters(&self, params: *mut D3DDEVICE_CREATION_PARAMETERS) -> Error {
        let params = check_mut_ref(params)?;
        *params = self.creation_params;
        Error::Success
    }

    // Function stubs:
    // these are functions which are defined, but not yet implemented.

    fn begin_scene() {
        unimplemented!()
    }
    fn begin_state_block() {
        unimplemented!()
    }
    fn clear() {
        unimplemented!()
    }
    fn color_fill() {
        unimplemented!()
    }
    fn create_additional_swap_chain() {
        unimplemented!()
    }
    fn create_cube_texture() {
        unimplemented!()
    }
    fn create_depth_stencil_surface() {
        unimplemented!()
    }
    fn create_index_buffer() {
        unimplemented!()
    }
    fn create_offscreen_plain_surface() {
        unimplemented!()
    }
    fn create_pixel_shader() {
        unimplemented!()
    }
    fn create_query() {
        unimplemented!()
    }
    fn create_render_target() {
        unimplemented!()
    }
    fn create_state_block() {
        unimplemented!()
    }
    fn create_texture() {
        unimplemented!()
    }
    fn create_vertex_buffer() {
        unimplemented!()
    }
    fn create_vertex_declaration() {
        unimplemented!()
    }
    fn create_vertex_shader() {
        unimplemented!()
    }
    fn create_volume_texture() {
        unimplemented!()
    }
    fn delete_patch() {
        unimplemented!()
    }
    fn draw_indexed_primitive() {
        unimplemented!()
    }
    fn draw_indexed_primitive_u_p() {
        unimplemented!()
    }
    fn draw_primitive() {
        unimplemented!()
    }
    fn draw_primitive_u_p() {
        unimplemented!()
    }
    fn draw_rect_patch() {
        unimplemented!()
    }
    fn draw_tri_patch() {
        unimplemented!()
    }
    fn end_scene() {
        unimplemented!()
    }
    fn end_state_block() {
        unimplemented!()
    }
    fn get_back_buffer() {
        unimplemented!()
    }
    fn get_clip_plane() {
        unimplemented!()
    }
    fn get_clip_status() {
        unimplemented!()
    }
    fn get_current_texture_palette() {
        unimplemented!()
    }
    fn get_depth_stencil_surface() {
        unimplemented!()
    }
    fn get_display_mode() {
        unimplemented!()
    }
    fn get_front_buffer_data() {
        unimplemented!()
    }
    fn get_f_v_f() {
        unimplemented!()
    }
    fn get_gamma_ramp() {
        unimplemented!()
    }
    fn get_indices() {
        unimplemented!()
    }
    fn get_light() {
        unimplemented!()
    }
    fn get_light_enable() {
        unimplemented!()
    }
    fn get_material() {
        unimplemented!()
    }
    fn get_n_patch_mode() {
        unimplemented!()
    }
    fn get_number_of_swap_chains() {
        unimplemented!()
    }
    fn get_palette_entries() {
        unimplemented!()
    }
    fn get_pixel_shader() {
        unimplemented!()
    }
    fn get_pixel_shader_constant_b() {
        unimplemented!()
    }
    fn get_pixel_shader_constant_f() {
        unimplemented!()
    }
    fn get_pixel_shader_constant_i() {
        unimplemented!()
    }
    fn get_raster_status() {
        unimplemented!()
    }
    fn get_render_state() {
        unimplemented!()
    }
    fn get_render_target() {
        unimplemented!()
    }
    fn get_render_target_data() {
        unimplemented!()
    }
    fn get_sampler_state() {
        unimplemented!()
    }
    fn get_scissor_rect() {
        unimplemented!()
    }
    fn get_software_vertex_processing() {
        unimplemented!()
    }
    fn get_stream_source() {
        unimplemented!()
    }
    fn get_stream_source_freq() {
        unimplemented!()
    }
    fn get_swap_chain() {
        unimplemented!()
    }
    fn get_texture() {
        unimplemented!()
    }
    fn get_texture_stage_state() {
        unimplemented!()
    }
    fn get_transform() {
        unimplemented!()
    }
    fn get_vertex_declaration() {
        unimplemented!()
    }
    fn get_vertex_shader() {
        unimplemented!()
    }
    fn get_vertex_shader_constant_b() {
        unimplemented!()
    }
    fn get_vertex_shader_constant_f() {
        unimplemented!()
    }
    fn get_vertex_shader_constant_i() {
        unimplemented!()
    }
    fn get_viewport() {
        unimplemented!()
    }
    fn light_enable() {
        unimplemented!()
    }
    fn multiply_transform() {
        unimplemented!()
    }
    fn present() {
        unimplemented!()
    }
    fn process_vertices() {
        unimplemented!()
    }
    fn reset() {
        unimplemented!()
    }
    fn set_clip_plane() {
        unimplemented!()
    }
    fn set_clip_status() {
        unimplemented!()
    }
    fn set_current_texture_palette() {
        unimplemented!()
    }
    fn set_cursor_position() {
        unimplemented!()
    }
    fn set_cursor_properties() {
        unimplemented!()
    }
    fn set_depth_stencil_surface() {
        unimplemented!()
    }
    fn set_dialog_box_mode() {
        unimplemented!()
    }
    fn set_f_v_f() {
        unimplemented!()
    }
    fn set_gamma_ramp() {
        unimplemented!()
    }
    fn set_indices() {
        unimplemented!()
    }
    fn set_light() {
        unimplemented!()
    }
    fn set_material() {
        unimplemented!()
    }
    fn set_n_patch_mode() {
        unimplemented!()
    }
    fn set_palette_entries() {
        unimplemented!()
    }
    fn set_pixel_shader() {
        unimplemented!()
    }
    fn set_pixel_shader_constant_b() {
        unimplemented!()
    }
    fn set_pixel_shader_constant_f() {
        unimplemented!()
    }
    fn set_pixel_shader_constant_i() {
        unimplemented!()
    }
    fn set_render_state() {
        unimplemented!()
    }
    fn set_render_target() {
        unimplemented!()
    }
    fn set_sampler_state() {
        unimplemented!()
    }
    fn set_scissor_rect() {
        unimplemented!()
    }
    fn set_software_vertex_processing() {
        unimplemented!()
    }
    fn set_stream_source() {
        unimplemented!()
    }
    fn set_stream_source_freq() {
        unimplemented!()
    }
    fn set_texture() {
        unimplemented!()
    }
    fn set_texture_stage_state() {
        unimplemented!()
    }
    fn set_transform() {
        unimplemented!()
    }
    fn set_vertex_declaration() {
        unimplemented!()
    }
    fn set_vertex_shader() {
        unimplemented!()
    }
    fn set_vertex_shader_constant_b() {
        unimplemented!()
    }
    fn set_vertex_shader_constant_f() {
        unimplemented!()
    }
    fn set_vertex_shader_constant_i() {
        unimplemented!()
    }
    fn set_viewport() {
        unimplemented!()
    }
    fn show_cursor() {
        unimplemented!()
    }
    fn stretch_rect() {
        unimplemented!()
    }
    fn update_surface() {
        unimplemented!()
    }
    fn update_texture() {
        unimplemented!()
    }
    fn validate_device() {
        unimplemented!()
    }
}
