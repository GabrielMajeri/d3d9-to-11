#pragma once

/// This class abstracts a graphics adapter (GPU).
class Adapter final {
public:
    Adapter(UINT index, ComPtr<IDXGIAdapter>&& adapter) noexcept;

    void get_identifier(D3DADAPTER_IDENTIFIER9& id) const noexcept;

private:
    // Ordinal of this adapter in the list of GPUs.
    UINT index;

    // DXGI interface representing a physical device.
    ComPtr<IDXGIAdapter> adapter;
};
