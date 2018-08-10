#include "format.hpp"

// See the following documentation links for more resources:
// - D3D9 formats: https://docs.microsoft.com/en-us/windows/desktop/direct3d9/d3dformat
// - DXGI formats: https://docs.microsoft.com/en-us/windows/desktop/api/dxgiformat/ne-dxgiformat-dxgi_format

// This is a list of static format mappings.
// A macro is used to make the list bidirectional.
#define FORMATS_LIST \
    FORMAT(D3DFMT_UNKNOWN, DXGI_FORMAT_UNKNOWN) \
    FORMAT(D3DFMT_A16B16G16R16F, DXGI_FORMAT_R16G16B16A16_FLOAT)

DXGI_FORMAT d3d_format_to_dxgi_format(UINT fmt) noexcept {
    switch (fmt) {
        #define FORMAT(a, b) case a: return b;
        FORMATS_LIST
        #undef FORMAT
    default:
        log::error("Unknown D3D9 format: ", fmt);
        return DXGI_FORMAT_UNKNOWN;
    }
}

UINT dxgi_format_to_d3d_format(DXGI_FORMAT fmt) noexcept {
    switch (fmt) {
        #define FORMAT(a, b) case b: return a;
        FORMATS_LIST
        #undef FORMAT
    default:
        log::error("Unknown DXGI format: ", fmt);
        return D3DFMT_UNKNOWN;
    }
}

bool is_display_mode_format(UINT fmt) noexcept {
    // Thankfully, these formats form a contiguous range.
    if (D3DFMT_A8R8G8B8 <= fmt && fmt <= D3DFMT_A1R5G5B5)
        return true;

    // This format is also supported.
    // It seems it's meant to be used with HDR displays.
    if (fmt == D3DFMT_A2R10G10B10)
        return true;

    return false;
}
