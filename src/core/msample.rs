use std::cmp;
use winapi::shared::{
    d3d9types::D3DMULTISAMPLE_TYPE,
    dxgitype::DXGI_SAMPLE_DESC,
};

/// Converts a D3D9 multisample type / quality pair to a DXGI_SAMPLE_DESC structure.
pub fn d3d9_to_dxgi_samples(ty: D3DMULTISAMPLE_TYPE, qlt: u32) -> DXGI_SAMPLE_DESC {
    // TODO: see how to handle non-maskable multisampling.

    let count = cmp::min(cmp::max(1, ty), 16);

    DXGI_SAMPLE_DESC {
        Count: count,
        Quality: qlt,
    }
}
