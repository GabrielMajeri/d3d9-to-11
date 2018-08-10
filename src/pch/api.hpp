#pragma once

// This header file defines a macro which ensures
// we properly export symbols from the library.

// On 32-bit, we need to use the __stdcall calling convention.
#ifndef WINAPI
    #ifdef _WIN64
        #define WINAPI
    #else
        #define WINAPI __stdcall
    #endif
#endif
