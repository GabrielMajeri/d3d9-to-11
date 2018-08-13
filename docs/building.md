# Building from source

This page provides instructions on how to build the project from source.

## Prerequisites

### Rust

This project is written in [Rust](https://www.rust-lang.org). You also need to install the latest nightly Rust compiler.
The recommended way to do so is through [rustup](https://rustup.rs/).

When installing, make sure to pick `nightly` and `i686-pc-windows-gnu` as the default toolchain / target.
If you've already installed Rust, you also need to add the `i686-pc-windows-gnu` target to be able to cross-compile to Wine/Windows.

```sh
rustup target add i686-pc-windows-gnu
```

### MinGW

To be able to link the final DLL, you also need GCC and binutils provided by the [MinGW-w64 project](http://mingw-w64.org/doku.php).
See their website or your distro's documentation on how to install the latest MinGW-w64.

### Building with Cargo

Building the project is very simple. Cargo, Rust's package manager, will download and compile everything for you.

Simply run:

```sh
cargo build
```

And you're done. You can optionally append the `--release` flag to build an optimized release build.

The built file is stored in `target/i686-pc-windows-gnu/<debug or release>/d3d9.dll`.

### Building for 64-bit apps

Most D3D9 games are 32-bit. If you happen upon a 64-bit D3D9 game,
you will have to build the DLL for the corresponding target.

First, add the 64-bit target with `rustup`:

```sh
rustup target add x86_64-pc-windows-gnu
```

You can then build the DLL with:

```sh
cargo build --target x86_64-pc-windows-gnu
```
