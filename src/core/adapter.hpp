#pragma once

#include <unordered_map>

/// This class abstracts a graphics adapter (GPU).
class Adapter final {
public:
    Adapter(UINT index, ComPtr<IDXGIAdapter>&& adapter) noexcept;

    // Retrieves a description of this adapter.
    void get_identifier(D3DADAPTER_IDENTIFIER9& id) const noexcept;

    // Retrieves the number of display modes which match the requested format.
    UINT get_mode_count(D3DFORMAT fmt) const noexcept;
    // Retrieves the display mode of a certain index.
    HRESULT get_mode(D3DFORMAT fmt, UINT index, D3DDISPLAYMODE& mode) const noexcept;

    // Checks if a given format is supported for a specific resource usage.
    HRESULT check_format_support(DWORD usage, D3DRESOURCETYPE rt, D3DFORMAT format) const noexcept;

    // Checks if we support multisampling for a given format.
    void check_multisample_support(D3DFORMAT fmt,
        D3DMULTISAMPLE_TYPE ms, UINT& quality) const noexcept;

private:
    // Retrieves the output's display modes and caches them.
    void cache_display_modes(D3DFORMAT fmt) const noexcept;

    // Ordinal of this adapter in the list of GPUs.
    UINT m_index;

    // DXGI interface representing a physical device.
    ComPtr<IDXGIAdapter> m_adapter;

    // The display attached to this device.
    ComPtr<IDXGIOutput> m_output;

    // Caches the supported display modes compatible with a certain format.
    mutable std::unordered_map<D3DFORMAT, std::vector<DXGI_MODE_DESC>> m_modes;

    // With D3D11, obtaining a device's capabilities or checking for texture format support
    // requires us to create the device first.
    ComPtr<ID3D11Device> m_device;

    // The highest-supported feature level of this device.
    D3D_FEATURE_LEVEL m_feature_level;
};
