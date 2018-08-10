#pragma once

// Functions for converting between D3DFORMAT <-> DXGI_FORMAT.

DXGI_FORMAT d3d_format_to_dxgi_format(D3DFORMAT fmt) noexcept;
D3DFORMAT dxgi_format_to_d3d_format(DXGI_FORMAT fmt) noexcept;

// Functions for validating the various format types.

/// Checks if a given format is valid to be used when setting the mode of the display.
///
/// Note that on modern computers we cannot change the display's format,
/// this is just for sanity checking.
bool is_display_mode_format(D3DFORMAT fmt) noexcept;
