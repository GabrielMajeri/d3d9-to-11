#include "adapter.hpp"

#include <cstring>
#include "../util/str.hpp"

Adapter::Adapter(UINT index, ComPtr<IDXGIAdapter>&& adapter) noexcept
    : m_index{ index }, m_adapter { std::move(adapter) } {
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
