#pragma once

/// This class abstracts a graphics adapter (GPU).
class Adapter final {
public:
    Adapter(UINT index, ComPtr<IDXGIAdapter>&& adapter) noexcept;

    // Retrieves a description of this adapter.
    void get_identifier(D3DADAPTER_IDENTIFIER9& id) const noexcept;

    // Checks if a given format is supported for a specific resource usage.
    HRESULT check_format_support(DWORD usage, D3DRESOURCETYPE rt, D3DFORMAT format) const noexcept;

    // Checks if we support multisampling for a given format.
    void check_multisample_support(D3DFORMAT fmt,
        D3DMULTISAMPLE_TYPE ms, UINT& quality) const noexcept;

private:
    // Ordinal of this adapter in the list of GPUs.
    UINT m_index;

    // DXGI interface representing a physical device.
    ComPtr<IDXGIAdapter> m_adapter;

    // With D3D11, obtaining a device's capabilities or checking for texture format support
    // requires us to create the device first.
    ComPtr<ID3D11Device> m_device;

    // The highest-supported feature level of this device.
    D3D_FEATURE_LEVEL m_feature_level;
};
