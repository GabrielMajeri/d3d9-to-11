#include "core.hpp"

#include "format.hpp"

#define CHECK_ADAPTER(adapter) { if ((adapter) >= m_adapters.size()) return D3DERR_INVALIDCALL; }
#define CHECK_DEVTYPE(dev_ty) { if ((dev_ty) != D3DDEVTYPE_HAL) return D3DERR_INVALIDCALL; }

Core::Core() {
    // We first have to create a factory, which is the equivalent of this interface in DXGI terms.
    const auto result = CreateDXGIFactory(m_factory.uuid(), (void**)&m_factory);
    assert(SUCCEEDED(result) && "Failed to create DXGI factory");

    // Now we can enumerate all the graphics adapters on the system.
    UINT id = 0;
    ComPtr<IDXGIAdapter> adapter;
    while (m_factory->EnumAdapters(id, &adapter) != DXGI_ERROR_NOT_FOUND) {
        m_adapters.emplace_back(id++, std::move(adapter));
    }
}

HRESULT Core::RegisterSoftwareDevice(void* pInitializeFunction) {
    CHECK_NOT_NULL(pInitializeFunction);

    log::warn("Application tried to register software device");

    return D3D_OK;
}

UINT Core::GetAdapterCount() {
    return m_adapters.size();
}

HRESULT Core::GetAdapterIdentifier(UINT Adapter, DWORD Flags, D3DADAPTER_IDENTIFIER9* pIdentifier) {
    CHECK_ADAPTER(Adapter);
    CHECK_NOT_NULL(pIdentifier);

    // Note: we ignore the flag, since it's only possible value, D3DENUM_WHQL_LEVEL,
    // is deprecated and irrelevant on Wine / newer versions of Windows.

    auto& adapter = m_adapters[Adapter];
    auto& id = *pIdentifier;

    adapter.get_identifier(id);

    return D3D_OK;
}

UINT Core::GetAdapterModeCount(UINT Adapter, D3DFORMAT Format) {
    CHECK_ADAPTER(Adapter);

    // Modern GPUs support back-buffers in any format, but the display's format cannot be changed.
    // The back-buffer will be converted to the right format on the fly.
    if (!is_display_mode_format(Format))
        return D3DERR_NOTAVAILABLE;

    return m_adapters[Adapter].get_mode_count(Format);
}

HRESULT Core::EnumAdapterModes(UINT Adapter, D3DFORMAT Format, UINT Mode, D3DDISPLAYMODE* pMode) {
    CHECK_ADAPTER(Adapter);
    CHECK_NOT_NULL(pMode);

    if (!is_display_mode_format(Format))
        return D3DERR_NOTAVAILABLE;

    return m_adapters[Adapter].get_mode(Format, Mode, *pMode);
}

HRESULT Core::GetAdapterDisplayMode(UINT Adapter, D3DDISPLAYMODE* pMode) {
    METHOD_STUB;
}

HRESULT Core::CheckDeviceType(UINT Adapter, D3DDEVTYPE DevType,
    D3DFORMAT AdapterFormat, D3DFORMAT BackBufferFormat, BOOL bWindowed) {
    METHOD_STUB;
}

HRESULT Core::CheckDeviceFormat(UINT Adapter, D3DDEVTYPE DevType,
    D3DFORMAT AdapterFormat, DWORD Usage, D3DRESOURCETYPE RType, D3DFORMAT CheckFormat) {
    CHECK_ADAPTER(Adapter);
    CHECK_DEVTYPE(DevType);

    // We ignore AdapterFormat, see the comment in GetAdapterModeCount.

    return m_adapters[Adapter].check_format_support(Usage, RType, CheckFormat);
}

HRESULT Core::CheckDeviceMultiSampleType(UINT Adapter, D3DDEVTYPE DevType,
    D3DFORMAT SurfaceFormat, BOOL Windowed,
    D3DMULTISAMPLE_TYPE MultiSampleType, DWORD* pQualityLevels) {
    CHECK_ADAPTER(Adapter);
    CHECK_DEVTYPE(DevType);

    // Ask D3D11 to tell us if it supports MS for this format.
    UINT quality = 0;
    m_adapters[Adapter].check_multisample_support(SurfaceFormat,
        MultiSampleType, quality);

    // Return the maximum quality level, if requested.
    if (pQualityLevels) {
        *pQualityLevels = quality;
    }

    // Quality of 0 would mean no support for MS.
    return quality ? D3D_OK : D3DERR_NOTAVAILABLE;
}

HRESULT Core::CheckDepthStencilMatch(UINT Adapter, D3DDEVTYPE DevType,
    D3DFORMAT AdapterFormat, D3DFORMAT RenderTargetFormat,
    D3DFORMAT DepthStencilFormat) {
    METHOD_STUB;
}

HRESULT Core::CheckDeviceFormatConversion(UINT Adapter, D3DDEVTYPE DevType,
    D3DFORMAT SourceFormat, D3DFORMAT TargetFormat) {
    METHOD_STUB;
}

HRESULT Core::GetDeviceCaps(UINT Adapter, D3DDEVTYPE DevType,
    D3DCAPS9* pCaps) {
    METHOD_STUB;
}

HMONITOR Core::GetAdapterMonitor(UINT Adapter) {
    METHOD_STUB;
}

HRESULT Core::CreateDevice(UINT Adapter, D3DDEVTYPE DevType,
    HWND hFocusWindow, DWORD BehaviorFlags,
    D3DPRESENT_PARAMETERS* pPresentationParameters,
    IDirect3DDevice9** ppReturnedDeviceInterface) {
    METHOD_STUB;
}
