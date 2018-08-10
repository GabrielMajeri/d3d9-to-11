#pragma once

// Functions for converting between D3DFORMAT <-> DXGI_FORMAT.

DXGI_FORMAT d3d_format_to_dxgi_format(UINT fmt) noexcept;

UINT dxgi_format_to_d3d_format(DXGI_FORMAT fmt) noexcept;
