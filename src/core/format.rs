//! Utility functions for validating and converting between D3DFORMAT <-> DXGI_FORMAT.
//!
//! See the following documentation links for more resources:
//! - D3D9 formats: https://docs.microsoft.com/en-us/windows/desktop/direct3d9/d3dformat
//! - DXGI formats: https://docs.microsoft.com/en-us/windows/desktop/api/dxgiformat/ne-dxgiformat-dxgi_format

use winapi::shared::d3d9types::*;
use winapi::shared::dxgiformat::*;

/// Extensions for the D3DFormat type.
pub trait D3DFormatExt {
    /// Converts a general resource format to a DXGI format.
    fn to_dxgi(self) -> DXGI_FORMAT;

    /// Converts a display mode format to its corresponding DXGI format.
    fn to_dxgi_display_format(self) -> DXGI_FORMAT;

    // Functions for validating the various format types.

    /// Checks if a given format is valid to be used when setting the mode of the display.
    ///
    /// Note that on modern computers we cannot change the display's format,
    /// this is just for sanity checking.
    fn is_display_mode_format(&self) -> bool;

    /// Checks if a given format is a valid D/S buffer format.
    fn is_depth_stencil_format(&self) -> bool;
}

impl D3DFormatExt for D3DFORMAT {
    fn to_dxgi(self) -> DXGI_FORMAT {
        // Note that a lot of RGB formats get transformed to BGR formats.
        // Due to GDI's choice BGR, most modern GPUs internally used BGR,
        // and converted RGB to it, which is why D3D11 uses it by default.
        match self {
            // 8 bit formats
            D3DFMT_R3G3B2 => DXGI_FORMAT_R8_UNORM,
            D3DFMT_A8 => DXGI_FORMAT_A8_UNORM,
            D3DFMT_L8 => DXGI_FORMAT_R8_UNORM,

            // 16 bit formats
            D3DFMT_A8R3G3B2 => DXGI_FORMAT_R8G8_UNORM,
            D3DFMT_R5G6B5 => DXGI_FORMAT_B5G6R5_UNORM,
            D3DFMT_X4R4G4B4 => DXGI_FORMAT_B4G4R4A4_UNORM,
            D3DFMT_A4R4G4B4 => DXGI_FORMAT_B4G4R4A4_UNORM,
            D3DFMT_X1R5G5B5 => DXGI_FORMAT_B5G5R5A1_UNORM,
            D3DFMT_A1R5G5B5 => DXGI_FORMAT_B5G5R5A1_UNORM,
            D3DFMT_L16 => DXGI_FORMAT_R16_UNORM,

            // 24 bit formats
            D3DFMT_R8G8B8 => DXGI_FORMAT_B8G8R8X8_UNORM,

            // 32 bit formats
            D3DFMT_X8R8G8B8 => DXGI_FORMAT_B8G8R8X8_UNORM,
            D3DFMT_A8R8G8B8 => DXGI_FORMAT_B8G8R8A8_UNORM,
            D3DFMT_A8B8G8R8 => DXGI_FORMAT_B8G8R8A8_UNORM,
            D3DFMT_X8B8G8R8 => DXGI_FORMAT_B8G8R8X8_UNORM,
            D3DFMT_G16R16 => DXGI_FORMAT_R16G16_UNORM,

            // Depth / stencil formats
            D3DFMT_S8_LOCKABLE => DXGI_FORMAT_R8_UNORM,
            D3DFMT_D16_LOCKABLE => DXGI_FORMAT_D16_UNORM,
            D3DFMT_D16 => DXGI_FORMAT_D16_UNORM,
            D3DFMT_D15S1 => DXGI_FORMAT_D16_UNORM,
            D3DFMT_D24S8 => DXGI_FORMAT_D24_UNORM_S8_UINT,
            D3DFMT_D24FS8 => DXGI_FORMAT_D24_UNORM_S8_UINT,
            D3DFMT_D24X8 => DXGI_FORMAT_D24_UNORM_S8_UINT,
            D3DFMT_D24X4S4 => DXGI_FORMAT_D24_UNORM_S8_UINT,
            D3DFMT_D32 => DXGI_FORMAT_D32_FLOAT,
            D3DFMT_D32F_LOCKABLE => DXGI_FORMAT_D32_FLOAT,
            D3DFMT_D32_LOCKABLE => DXGI_FORMAT_D32_FLOAT,

            // HDR formats
            D3DFMT_A2R10G10B10 | D3DFMT_A2B10G10R10 => DXGI_FORMAT_R10G10B10A2_UNORM,

            // Buffer formats
            D3DFMT_R16F => DXGI_FORMAT_R16_FLOAT,
            D3DFMT_G16R16F => DXGI_FORMAT_R16G16_FLOAT,
            D3DFMT_A16B16G16R16 => DXGI_FORMAT_R16G16B16A16_UINT,
            D3DFMT_A16B16G16R16F => DXGI_FORMAT_R16G16B16A16_FLOAT,
            D3DFMT_R32F => DXGI_FORMAT_R32_FLOAT,
            D3DFMT_G32R32F => DXGI_FORMAT_R32G32_FLOAT,
            D3DFMT_A32B32G32R32F => DXGI_FORMAT_R32G32B32A32_FLOAT,

            // Unknown formats
            D3DFMT_UNKNOWN => DXGI_FORMAT_UNKNOWN,
            _ => panic!("Unknown D3D9 format: {}", self),
        }
    }

    fn to_dxgi_display_format(self) -> DXGI_FORMAT {
        match self {
            // We have to map all these formats to a format DXGI supports.
            D3DFMT_R8G8B8..=D3DFMT_A1R5G5B5 | D3DFMT_UNKNOWN => DXGI_FORMAT_B8G8R8A8_UNORM,
            D3DFMT_A2R10G10B10 => DXGI_FORMAT_R10G10B10A2_UNORM,
            _ => panic!("Unknown D3D9 display format: {}", self),
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

    fn is_depth_stencil_format(&self) -> bool {
        match *self {
            // TODO: there are also some unused formats in this range.
            // Need to check all formats in this range to be valid.
            D3DFMT_D16_LOCKABLE..=D3DFMT_S8_LOCKABLE => true,
            _ => false,
        }
    }
}
