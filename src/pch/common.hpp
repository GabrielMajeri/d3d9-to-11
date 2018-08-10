// Common includes for the project.
// This file is meant to be used as a Pre-Compiled Header to speed up compilation.

// Please only include things which rarely change in this header.
// This includes Windows API files and the C++ standard library.

#include "api.hpp"
#include "windows.hpp"

#include <cassert>

#include <atomic>
#include <iostream>
#include <type_traits>
#include <utility>
#include <vector>

#include <d3d9.h>

#include <dxgi.h>
#include <d3d11.h>

#include "../util/log.hpp"
#include "../util/com/ptr.hpp"
#include "../util/com/impl.hpp"

/// This macro can be used in methods which are not yet implemented.
/// The app will crash if they get called.
#define METHOD_STUB \
    std::cerr << __func__ << " is not implemented\n"; \
    std::abort()

/// Since we often have to validate pointer parameters, this macro encapsulates the check.
#define CHECK_NOT_NULL(ptr) { if ((ptr) == nullptr) { return D3DERR_INVALIDCALL; } }
