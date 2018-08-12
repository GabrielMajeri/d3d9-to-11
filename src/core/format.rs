//! Utility functions for validating and converting between D3DFORMAT <-> DXGI_FORMAT.
//!
//! See the following documentation links for more resources:
//! - D3D9 formats: https://docs.microsoft.com/en-us/windows/desktop/direct3d9/d3dformat
//! - DXGI formats: https://docs.microsoft.com/en-us/windows/desktop/api/dxgiformat/ne-dxgiformat-dxgi_format

use winapi::shared::d3d9types::*;
use winapi::shared::dxgiformat::*;

/// Extensions for the D3DFormat type.
pub trait D3DFormatExt {
    fn to_dxgi(self) -> DXGI_FORMAT;

    // Functions for validating the various format types.

    /// Checks if a given format is valid to be used when setting the mode of the display.
    ///
    /// Note that on modern computers we cannot change the display's format,
    /// this is just for sanity checking.
    fn is_display_mode_format(&self) -> bool;
}

impl D3DFormatExt for D3DFORMAT {
    fn to_dxgi(self) -> DXGI_FORMAT {
        match self {
            D3DFMT_UNKNOWN => DXGI_FORMAT_UNKNOWN,
            D3DFMT_X8R8G8B8 => DXGI_FORMAT_B8G8R8X8_UNORM,
            D3DFMT_A16B16G16R16F => DXGI_FORMAT_R16G16B16A16_FLOAT,
            _ => panic!("Unknown D3D9 format: {}", self),
        }
    }

    fn is_display_mode_format(&self) -> bool {
        match *self {
            // Thankfully, these formats form a contiguous range.
            D3DFMT_A8R8G8B8..=D3DFMT_A1R5G5B5 => true,
            // This format is also supported.
            // It seems it's meant to be used with HDR displays.
            D3DFMT_A2R10G10B10 => true,
            _ => false,
        }
    }
}
