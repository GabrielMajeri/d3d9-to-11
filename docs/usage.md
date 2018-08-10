# Usage instructions

The result of [building](building.md) is a DLL file called `d3d9.dll`.

The DLL should be placed in a game's executable directory.
Use `winecfg` to set this DLL override to "Native".

**Warning**: currently, you must also copy/symlink the `libwinpthread-1.dll`, `libstdc++-6.dll`, `libgcc_s_sjlj-1.dll` files from your MinGW installation
(usually installed somewhere like `/usr/i686-w64-mingw32/bin`) into the same directory as `D3D9.dll`.
This is an issue with the current build system and will get fixed in the future.
