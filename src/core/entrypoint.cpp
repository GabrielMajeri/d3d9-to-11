// API entrypoints.

#include <iostream>

#include "core.hpp"

extern "C" IDirect3D9* WINAPI Direct3DCreate9(UINT SdkVersion) {
    // Try to identify which version of the D3D9 the app was built against.
    // This could be used to implement compatibility workarounds if needed.
    switch (SdkVersion) {
    case 32:
        log::info("D3D9 version: 9.0c");
        break;
    default:
        log::warn("Unknown D3D9 SDK version: ", SdkVersion);
        break;
    }

    // Return our implementation of D3D9.
    return new Core();
}
