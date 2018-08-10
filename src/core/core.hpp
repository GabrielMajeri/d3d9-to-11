#pragma once

#include "../util/com/impl.hpp"
#include "adapter.hpp"

class Core final: public ComImpl<IDirect3D9> {
public:
    Core();

    HRESULT WINAPI RegisterSoftwareDevice(void* pInitializeFunction) override;

    UINT WINAPI GetAdapterCount() override;

    HRESULT WINAPI GetAdapterIdentifier(UINT Adapter, DWORD Flags,
      D3DADAPTER_IDENTIFIER9* pIdentifier) override;

    UINT WINAPI GetAdapterModeCount(UINT Adapter,
      D3DFORMAT Format) override;

    HRESULT WINAPI EnumAdapterModes(UINT Adapter, D3DFORMAT Format,
      UINT Mode, D3DDISPLAYMODE* pMode) override;

    HRESULT WINAPI GetAdapterDisplayMode(UINT Adapter,
      D3DDISPLAYMODE* pMode) override;

    HRESULT WINAPI CheckDeviceType(UINT Adapter, D3DDEVTYPE DevType,
      D3DFORMAT AdapterFormat, D3DFORMAT BackBufferFormat,
      BOOL bWindowed) override;

    HRESULT WINAPI CheckDeviceFormat(UINT Adapter, D3DDEVTYPE DevType,
      D3DFORMAT AdapterFormat, DWORD Usage,
      D3DRESOURCETYPE RType, D3DFORMAT CheckFormat) override;

    HRESULT WINAPI CheckDeviceMultiSampleType(UINT Adapter, D3DDEVTYPE DevType,
      D3DFORMAT SurfaceFormat, BOOL Windowed,
      D3DMULTISAMPLE_TYPE MultiSampleType, DWORD* pQualityLevels) override;

    HRESULT WINAPI CheckDepthStencilMatch(UINT Adapter, D3DDEVTYPE DevType,
      D3DFORMAT AdapterFormat, D3DFORMAT RenderTargetFormat,
      D3DFORMAT DepthStencilFormat) override;

    HRESULT WINAPI CheckDeviceFormatConversion(UINT Adapter, D3DDEVTYPE DevType,
      D3DFORMAT SourceFormat, D3DFORMAT TargetFormat) override;

    HRESULT WINAPI GetDeviceCaps(UINT Adapter, D3DDEVTYPE DevType,
      D3DCAPS9* pCaps) override;

    HMONITOR WINAPI GetAdapterMonitor(UINT Adapter) override;

    HRESULT WINAPI CreateDevice(UINT Adapter, D3DDEVTYPE DevType,
      HWND hFocusWindow, DWORD BehaviorFlags,
      D3DPRESENT_PARAMETERS* pPresentationParameters,
      IDirect3DDevice9** ppReturnedDeviceInterface) override;

private:
    ComPtr<IDXGIFactory> m_factory;
    std::vector<Adapter> m_adapters;
};
