//! Implements multisampling-related conversion functions.

use std::cmp;
use winapi::shared::{d3d9types::D3DMULTISAMPLE_TYPE, dxgitype::DXGI_SAMPLE_DESC};

/// Converts a D3D9 multisample type / quality pair to a DXGI_SAMPLE_DESC structure.
pub fn d3d9_to_dxgi_samples(ty: D3DMULTISAMPLE_TYPE, qlt: u32) -> DXGI_SAMPLE_DESC {
    // TODO: see how to handle non-maskable multisampling.

    let count = cmp::min(cmp::max(1, ty), 16);

    DXGI_SAMPLE_DESC {
        Count: count,
        Quality: qlt,
    }
}

/// Converts a DXGI_SAMPLE_DESC structure to a pair of MS type and quality.
pub fn dxgi_samples_to_d3d9(d: DXGI_SAMPLE_DESC) -> (D3DMULTISAMPLE_TYPE, u32) {
    (d.Count, d.Quality)
}
