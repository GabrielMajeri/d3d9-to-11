# Building from source

This page provides instructions on how to build the project from source.

## Prerequisites

The project uses [Meson](https://mesonbuild.com/) as its build system.
You should install the latest stable version of Meson for best compatibility and performance.

[Ninja](https://ninja-build.org/) is also recommended to speed up builds compared to Makefiles.

## Compiling with MinGW

To cross-compile from Linux to Wine/Windows, you need a cross-compiler which is provided by the [MinGW-w64 project](http://mingw-w64.org/doku.php).

### Building with GCC

Here is how to build the project using MinGW's GCC:

```sh
meson 'build.w32' --cross-file 'toolchain/gcc-w32.txt'
ninja -C 'build.w32'
```

### Building with Clang (recommended)

However, you needn't use `mingw-w64-gcc` when developing: in fact, the developer uses (and recommends) Clang instead.
Clang is natively a cross-compiler, meaning you just need to [download it from its website](http://releases.llvm.org/download.html).

**You still need to install MinGW fully** to obtain their linker / libraries / headers, but please use Clang for a better development experience.

```sh
meson 'build.w32' --cross-file 'toolchain/clang-w32.txt'
ninja -C 'build.w32'
```

## Compiling with Microsoft Visual C++

At the moment, only the MinGW-w64 toolchain is supported.
You can download and install [MinGW-w64 for Windows](https://sourceforge.net/projects/mingw-w64/).
