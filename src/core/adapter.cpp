#include "adapter.hpp"

#include <cstring>
#include "../util/str.hpp"
#include "format.hpp"

Adapter::Adapter(UINT index, ComPtr<IDXGIAdapter>&& adapter) noexcept
    : m_index{ index }, m_adapter { std::move(adapter) } {
    const auto result = D3D11CreateDevice(
        // Create a device for the adapter we own.
        m_adapter.as_raw(),
        D3D_DRIVER_TYPE_UNKNOWN, nullptr,
        // No additional flags.
        0,
        // We will use whichever feature level is supported.
        nullptr, 0,
        D3D11_SDK_VERSION,
        &m_device,
        &m_feature_level,
        // We do not need a context for now.
        nullptr
    );

    assert(SUCCEEDED(result) && "Failed to create D3D11 device");
}

void Adapter::get_identifier(D3DADAPTER_IDENTIFIER9& id) const noexcept {
    DXGI_ADAPTER_DESC desc;
    assert(SUCCEEDED(m_adapter->GetDesc(&desc)));

    // Internal identifier of the driver.
    std::strcpy(id.Driver, "D3D 9-to-11 Driver");

    // Human readable device description.
    const auto description = str::join(str::convert(desc.Description), " (D3D 9-to-11 Device)");
    std::strcpy(id.Description, description.data());

    // Fake GDI device name
    const auto device_name = str::join("DISPLAY", m_index);
    std::strcpy(id.DeviceName, device_name.data());

    id.DriverVersion.QuadPart = 1;

    // These fields are passed-through.
    id.VendorId = desc.VendorId;
    id.DeviceId = desc.DeviceId;
    id.SubSysId = desc.SubSysId;
    id.Revision = desc.Revision;

    // D3D9 wants a 128-bit unique adapter identifier.
    // We don't have anything like that available, so we combine a 64-bit LUID with the adapter's index.
    std::memcpy(&id.DeviceIdentifier.Data1, &desc.AdapterLuid, sizeof(LUID));
    std::memcpy(&id.DeviceIdentifier.Data4[0], &m_index, sizeof(UINT));

    id.WHQLLevel = 1;
}

HRESULT Adapter::check_format_support(DWORD usage, D3DRESOURCETYPE rt, D3DFORMAT format) const noexcept {
    DXGI_FORMAT fmt = d3d_format_to_dxgi_format(format);

    UINT support = 0;
    if (FAILED(m_device->CheckFormatSupport(fmt, &support)))
        return D3DERR_NOTAVAILABLE;

    // Macro to check if the resource type is supported.
    // Returns true if a resource type is _not_ supported.
    #define CHECK_RT_SUPPORT(a, b) (rt == D3DRTYPE_ ## a && (support & D3D11_FORMAT_SUPPORT_ ## b) == 0)

    if (CHECK_RT_SUPPORT(SURFACE, TEXTURE2D) ||
        CHECK_RT_SUPPORT(VOLUME, TEXTURE3D) ||
        CHECK_RT_SUPPORT(TEXTURE, TEXTURE2D) ||
        CHECK_RT_SUPPORT(VOLUMETEXTURE, TEXTURE3D) ||
        CHECK_RT_SUPPORT(CUBETEXTURE, TEXTURECUBE) ||
        CHECK_RT_SUPPORT(VERTEXBUFFER, IA_VERTEX_BUFFER) ||
        CHECK_RT_SUPPORT(INDEXBUFFER, IA_INDEX_BUFFER)) {
        return D3DERR_NOTAVAILABLE;
    }

    // Similar to macro above.
    #define CHECK_USAGE(a, b) (((usage & D3DUSAGE_##a) != 0) && ((support & D3D11_FORMAT_SUPPORT_##b) == 0))

    if (CHECK_USAGE(AUTOGENMIPMAP, MIP_AUTOGEN) ||
        CHECK_USAGE(RENDERTARGET, RENDER_TARGET) ||
        CHECK_USAGE(DEPTHSTENCIL, DEPTH_STENCIL)) {
        return D3DERR_NOTAVAILABLE;
    }

    return D3D_OK;
}
