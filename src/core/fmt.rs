//! Utility functions for validating and converting between D3DFORMAT <-> DXGI_FORMAT.
//!
//! See the following documentation links for more resources:
//! - D3D9 formats: https://docs.microsoft.com/en-us/windows/desktop/direct3d9/d3dformat
//! - DXGI formats: https://docs.microsoft.com/en-us/windows/desktop/api/dxgiformat/ne-dxgiformat-dxgi_format

use winapi::shared::d3d9types::*;
use winapi::shared::dxgiformat::*;

/// Converts a display mode format to its corresponding DXGI format.
pub fn d3d_display_format_to_dxgi(fmt: D3DFORMAT) -> DXGI_FORMAT {
    match fmt {
        // We have to map all these formats to a format DXGI supports.
        D3DFMT_R8G8B8..=D3DFMT_A1R5G5B5 | D3DFMT_UNKNOWN => DXGI_FORMAT_B8G8R8A8_UNORM,
        D3DFMT_A2R10G10B10 => DXGI_FORMAT_R10G10B10A2_UNORM,
        _ => panic!("Unknown D3D9 display format: {}", fmt),
    }
}

/// Checks if a given format is valid to be used when setting the mode of the display.
///
/// Note that on modern computers we cannot change the display's format,
/// this is just for sanity checking.
pub fn is_display_mode_format(fmt: D3DFORMAT) -> bool {
    match fmt {
        // Thankfully, these formats form a contiguous range.
        D3DFMT_A8R8G8B8..=D3DFMT_A1R5G5B5 => true,
        // This format is also supported.
        // It seems it's meant to be used with HDR displays.
        D3DFMT_A2R10G10B10 => true,
        _ => false,
    }
}

/// Checks if a given format is a valid D/S buffer format.
pub fn is_depth_stencil_format(fmt: D3DFORMAT) -> bool {
    match fmt {
        // TODO: there are also some unused formats in this range.
        // Need to check all formats in this range to be valid.
        D3DFMT_D16_LOCKABLE..=D3DFMT_S8_LOCKABLE => true,
        _ => false,
    }
}

// This macro is used to generate bi-directional mapping between D3D and DXGI formats.
macro_rules! format_conv {
    ($($a:path => $b:path,)*) => {
        /// Converts a general resource format to a DXGI format.
        pub fn d3d_format_to_dxgi(fmt: D3DFORMAT) -> DXGI_FORMAT {
            #[allow(non_upper_case_globals)]
            match fmt {
                $($a => $b,)*

                _ => panic!("Unknown D3D9 format: {}", fmt),
            }
        }

        /// Converts a DXGI format back into a Direct3D format.
        pub fn dxgi_format_to_d3d(fmt: DXGI_FORMAT) -> D3DFORMAT {
            // Some DXGI formats map to multiple D3D9 formats.
            #[allow(unreachable_patterns)]
            match fmt {
                $($b => $a,)*
                _ => panic!("Unknown DXGI format: {}", fmt),
            }
        }
    }
}

// Based upon the following reference:
// https://docs.microsoft.com/en-us/windows/desktop/direct3d10/d3d10-graphics-programming-guide-resources-legacy-formats
format_conv! {
    // 8 bit formats
    D3DFMT_A8 => DXGI_FORMAT_A8_UNORM,
    D3DFMT_L8 => DXGI_FORMAT_R8_UNORM,

    // 16 bit formats
    D3DFMT_R5G6B5 => DXGI_FORMAT_B5G6R5_UNORM,
    D3DFMT_X4R4G4B4 => DXGI_FORMAT_B4G4R4A4_UNORM,
    D3DFMT_A4R4G4B4 => DXGI_FORMAT_B4G4R4A4_UNORM,
    D3DFMT_X1R5G5B5 => DXGI_FORMAT_B5G5R5A1_UNORM,
    D3DFMT_A1R5G5B5 => DXGI_FORMAT_B5G5R5A1_UNORM,
    D3DFMT_A8L8 => DXGI_FORMAT_R8G8_UNORM,
    D3DFMT_L16 => DXGI_FORMAT_R16_UNORM,

    // 32 bit formats
    D3DFMT_X8B8G8R8 => DXGI_FORMAT_R8G8B8A8_UNORM,
    D3DFMT_X8R8G8B8 => DXGI_FORMAT_B8G8R8X8_UNORM,
    D3DFMT_A8R8G8B8 => DXGI_FORMAT_B8G8R8A8_UNORM,
    D3DFMT_A8B8G8R8 => DXGI_FORMAT_R8G8B8A8_UNORM,
    D3DFMT_G16R16 => DXGI_FORMAT_R16G16_UNORM,

    // HDR formats
    D3DFMT_A2R10G10B10 => DXGI_FORMAT_R10G10B10A2_UNORM,
    D3DFMT_A2B10G10R10 => DXGI_FORMAT_R10G10B10A2_UNORM,

    // Depth / stencil formats
    D3DFMT_D16_LOCKABLE => DXGI_FORMAT_D16_UNORM,
    D3DFMT_D16 => DXGI_FORMAT_D16_UNORM,
    D3DFMT_D24S8 => DXGI_FORMAT_D24_UNORM_S8_UINT,
    D3DFMT_D24X8 => DXGI_FORMAT_D24_UNORM_S8_UINT,
    D3DFMT_D32 => DXGI_FORMAT_D32_FLOAT,
    D3DFMT_D32F_LOCKABLE => DXGI_FORMAT_D32_FLOAT,

    // Compressed formats
    D3DFMT_DXT1 => DXGI_FORMAT_BC1_UNORM,
    D3DFMT_DXT2 => DXGI_FORMAT_BC2_UNORM,
    D3DFMT_DXT3 => DXGI_FORMAT_BC3_UNORM,
    D3DFMT_DXT4 => DXGI_FORMAT_BC4_UNORM,
    D3DFMT_DXT5 => DXGI_FORMAT_BC5_UNORM,

    // Special formats: mostly used for hardware video.
    D3DFMT_R8G8_B8G8 => DXGI_FORMAT_G8R8_G8B8_UNORM,
    D3DFMT_G8R8_G8B8 => DXGI_FORMAT_R8G8_B8G8_UNORM,

    // Signed formats
    D3DFMT_V8U8 => DXGI_FORMAT_R8G8_SNORM,
    D3DFMT_Q8W8V8U8 => DXGI_FORMAT_R8G8B8A8_SNORM,
    D3DFMT_V16U16 => DXGI_FORMAT_R16G16_SNORM,

    // Buffer formats
    D3DFMT_R16F => DXGI_FORMAT_R16_FLOAT,
    D3DFMT_G16R16F => DXGI_FORMAT_R16G16_FLOAT,
    D3DFMT_A16B16G16R16 => DXGI_FORMAT_R16G16B16A16_UNORM,
    D3DFMT_A16B16G16R16F => DXGI_FORMAT_R16G16B16A16_FLOAT,
    D3DFMT_R32F => DXGI_FORMAT_R32_FLOAT,
    D3DFMT_G32R32F => DXGI_FORMAT_R32G32_FLOAT,
    D3DFMT_A32B32G32R32F => DXGI_FORMAT_R32G32B32A32_FLOAT,

    // Unknown format
    D3DFMT_UNKNOWN => DXGI_FORMAT_UNKNOWN,

    // Unsupported formats
    // TODO: some formats have no support in modern DXGI.
    // We might still be able to approximate them with some other formats though.
    D3DFMT_P8 => DXGI_FORMAT_UNKNOWN,
    D3DFMT_A8P8 => DXGI_FORMAT_UNKNOWN,
    D3DFMT_A4L4 => DXGI_FORMAT_UNKNOWN,
    D3DFMT_R3G3B2 => DXGI_FORMAT_UNKNOWN,
    D3DFMT_A8R3G3B2 => DXGI_FORMAT_UNKNOWN,
    D3DFMT_R8G8B8 => DXGI_FORMAT_UNKNOWN,
    D3DFMT_D15S1 => DXGI_FORMAT_UNKNOWN,
    D3DFMT_D24FS8 => DXGI_FORMAT_UNKNOWN,
    D3DFMT_D24X4S4 => DXGI_FORMAT_UNKNOWN,
    D3DFMT_S8_LOCKABLE => DXGI_FORMAT_UNKNOWN,
    D3DFMT_CxV8U8 => DXGI_FORMAT_UNKNOWN,
    D3DFMT_L6V5U5 => DXGI_FORMAT_UNKNOWN,
    D3DFMT_X8L8V8U8 => DXGI_FORMAT_UNKNOWN,
    D3DFMT_A2W10V10U10 => DXGI_FORMAT_UNKNOWN,
    D3DFMT_Q16W16V16U16 => DXGI_FORMAT_UNKNOWN,
    D3DFMT_UYVY => DXGI_FORMAT_UNKNOWN,
    D3DFMT_YUY2 => DXGI_FORMAT_UNKNOWN,
}
