# Direct3D 9-to-11

## Scope of this project

This project is an attempt to convert [Direct3D 9](https://en.wikipedia.org/wiki/Direct3D#Direct3D_9) programs
to [Direct3D 11](https://en.wikipedia.org/wiki/Direct3D#Direct3D_11).
It reimplements the `d3d9.dll`, which contains the core D3D9 interfaces.

Most D3D9 games are CPU limited on modern PCs, since GPU power increased exponentially while CPUs fell behind.
Furthermore, most (old) games lack multithreading support, draining the CPU resource even more.

This project uplifts the games D3D9 graphics API calls to D3D11.

**Important**: this project ought be used together with [DXVK](https://github.com/doitsujin/dxvk/), which would then translate D3D11 to Vulkan.

## Building

The project's uses [Meson](https://mesonbuild.com/) as its build system.
You should install the latest stable version of Meson for best compatibility and performance.

[Ninja](https://ninja-build.org/) is also recommended to speed up builds compared to Makefiles.

To cross-compile from Linux to Wine/Windows, you need a cross-compiler which is provided by the [MinGW-w64 project](http://mingw-w64.org/doku.php).

Here is how to build the project using MinGW's GCC:

```sh
meson 'build.w32' --cross-file 'toolchain/gcc-w32.txt'
cd 'build.w32'
ninja
```

However, you needn't use `mingw-w64-gcc` when developing: in fact, the developer uses (and recommends) Clang instead.
Clang is a native cross-compiler. **You still need to install MinGW fully** to obtain their linker / libraries / headers,
but please use `toolchain/clang-w32.txt` files for a better development experience.

## Using

The result of the build process is a `D3D9.dll` file which should be placed in a game's executable directory.
Use `winecfg` to set this DLL override to "Native".

**Warning**: currently, you must also copy the `libwinpthread-1.dll`, `libstdc++-6.dll`, `libgcc_s_sjlj-1.dll` files from your MinGW installation
(usually installed somewhere like `/usr/i686-w64-mingw32/bin`) into the same directory as `D3D9.dll`.
This is an issue with the current build system and will get fixed in the future.

## Why not VK9

[VK9](https://github.com/disks86/VK9) is an alternative to this project, which aims to convert D3D9 to Vulkan directly.

I respect Schaefer's work on VK9.
I would have loved to contribute to this project, but I don't know Vulkan very well.
Furthermore, I believe using Vulkan directly is error prone. It's hard to fix both D3D9 bugs and Vulkan bugs at the same time.

I believe a more layered approach, i.e. converting D3D9 to D3D11, is better,
since there is already a lot of work done for us by [DXVK](https://github.com/doitsujin/dxvk/).

Both VK9 and DXVK implement command stream multithreading, inspired by Wine's original [CSMT](https://github.com/wine-compholio/wine-staging/wiki/CSMT).
This feature reduces CPU utilisation and improves performance.
Instead of reimplementing this feature yet another time, this project assumes D3D11 (DXVK) does this for us.

## History of the project

I initially started work on my idea by forking DXVK and building on top of its infrastructure.

After a [discussion in a pull request](https://github.com/doitsujin/dxvk/pull/541),
I've realised it's better to keep the projects separate and allow DXVK to focus on the best D3D11 support,
while I work on my separate D3D9-to-D3D11 wrapper.

## Credits

- **Wine** for allowing us to run Windows programs on other OSes

- **DXVK** for inspiration and making this project possible

- **VK9** for the original D3D9-to-Vulkan wrapper

## License

This project is licensed under the [Lesser GNU General Public License](LICENSE),
version 3 or (at your option) any later version.
