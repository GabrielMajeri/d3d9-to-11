#include "core.hpp"

HRESULT Core::RegisterSoftwareDevice(void* pInitializeFunction) {
    METHOD_STUB;
}

UINT Core::GetAdapterCount() {
    METHOD_STUB;
}

HRESULT Core::GetAdapterIdentifier(UINT Adapter, DWORD Flags, D3DADAPTER_IDENTIFIER9* pIdentifier) {
    METHOD_STUB;
}

UINT Core::GetAdapterModeCount(UINT Adapter, D3DFORMAT Format) {
    METHOD_STUB;
}

HRESULT Core::EnumAdapterModes(UINT Adapter, D3DFORMAT Format, UINT Mode, D3DDISPLAYMODE* pMode) {
    METHOD_STUB;
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
    METHOD_STUB;
}

HRESULT Core::CheckDeviceMultiSampleType(UINT Adapter, D3DDEVTYPE DevType,
    D3DFORMAT SurfaceFormat, BOOL Windowed, D3DMULTISAMPLE_TYPE MultiSampleType, DWORD* pQualityLevels) {
    METHOD_STUB;
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
