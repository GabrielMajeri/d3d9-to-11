use std::ptr;

use winapi::shared::d3d9types::*;

use crate::dev::shader::VertexDeclaration;

impl_state! {
    /// Structure containing all state related to vertex processing.
    ///
    /// For a list of things contained within the vertex state, see:
    /// https://docs.microsoft.com/en-us/windows/desktop/direct3d9/saving-vertex-states-with-a-stateblock
    pub struct VertexState {
        // Vertex-related render state
        cull_mode: D3DRS_CULLMODE = D3DCULL_CCW,
        fog_color: D3DRS_FOGCOLOR = 0,
        fog_table_mode: D3DRS_FOGTABLEMODE = D3DFOG_NONE,
        fog_start: D3DRS_FOGSTART = 0,
        fog_end: D3DRS_FOGEND = 1,
        fog_density: D3DRS_FOGDENSITY = 1,
        range_fog_enable: D3DRS_RANGEFOGENABLE = 0,
        ambient: D3DRS_AMBIENT = 0,
        color_vertex: D3DRS_COLORVERTEX = 1,
        fog_vertex_mode: D3DRS_FOGVERTEXMODE = D3DFOG_NONE,
        clipping: D3DRS_CLIPPING = 1,
        lighting: D3DRS_LIGHTING = 1,
        local_viewer: D3DRS_LOCALVIEWER = 1,
        emissive_material_source: D3DRS_EMISSIVEMATERIALSOURCE = D3DMCS_MATERIAL,
        ambient_material_source: D3DRS_AMBIENTMATERIALSOURCE = D3DMCS_MATERIAL,
        diffuse_material_source: D3DRS_DIFFUSEMATERIALSOURCE = D3DMCS_COLOR1,
        specular_material_source: D3DRS_SPECULARMATERIALSOURCE = D3DMCS_COLOR2,
        vertex_blend: D3DRS_VERTEXBLEND = D3DVBF_DISABLE,
        clip_plane_enable: D3DRS_CLIPPLANEENABLE = 0,
        // This one is driver dependent by default.
        point_size: D3DRS_POINTSIZE = 1,
        point_size_min: D3DRS_POINTSIZE_MIN = 1,
        point_sprite_enable: D3DRS_POINTSPRITEENABLE = 0,
        point_scale_enable: D3DRS_POINTSCALEENABLE = 0,
        point_scale_a: D3DRS_POINTSCALE_A = 1,
        point_scale_b: D3DRS_POINTSCALE_B = 0,
        point_scale_c: D3DRS_POINTSCALE_C = 0,
        multisample_antialias: D3DRS_MULTISAMPLEANTIALIAS = 1,
        multisample_mask: D3DRS_MULTISAMPLEMASK = 0xffff_ffff,
        patch_edge_style: D3DRS_PATCHEDGESTYLE = D3DPATCHEDGE_DISCRETE,
        point_size_max: D3DRS_POINTSIZE_MAX = 1,
        index_vertex_blend_enable: D3DRS_INDEXEDVERTEXBLENDENABLE = 0,
        tween_factor: D3DRS_TWEENFACTOR = 0,
        position_degree: D3DRS_POSITIONDEGREE = D3DDEGREE_CUBIC,
        normal_degree: D3DRS_NORMALDEGREE = D3DDEGREE_LINEAR,
        min_tess_lvl: D3DRS_MINTESSELLATIONLEVEL = 1,
        max_tess_lvl: D3DRS_MAXTESSELLATIONLEVEL = 1,
        adaptive_tess_x: D3DRS_ADAPTIVETESS_X = 0,
        adaptive_tess_y: D3DRS_ADAPTIVETESS_Y = 0,
        adaptive_tess_z: D3DRS_ADAPTIVETESS_Z = 1,
        adaptive_tess_w: D3DRS_ADAPTIVETESS_W = 0,
        enable_adaptive_tess: D3DRS_ENABLEADAPTIVETESSELLATION = 0;
        // Sampler state
        MAX_SAMPLERS = 4;
        dmap_offset: D3DSAMP_DMAPOFFSET = 256;
        // Texture state
        tex_coord_index: D3DTSS_TEXCOORDINDEX = 0,
        texture_transform_flags: D3DTSS_TEXTURETRANSFORMFLAGS = D3DTTFF_DISABLE;
        // Extra state variables
        vertex_decl: *const VertexDeclaration = ptr::null_mut(),
    }
}
