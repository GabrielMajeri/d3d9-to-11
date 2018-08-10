// API entrypoints.

#include <iostream>

extern "C" IDirect3D9* WINAPI Direct3DCreate9(UINT SdkVersion) {
    std::cerr << "D3D9 SDK version: " << SdkVersion << '\n';

    std::cerr << __func__ << " stub\n";
    std::exit(1);

    return nullptr;
}
