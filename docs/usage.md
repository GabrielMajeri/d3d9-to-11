# Usage instructions

The result of [building](building.md) is a DLL file called `d3d9.dll`.

## Installation

The DLL should be placed in a game's executable directory.
Use `winecfg` to set this DLL override to "Native".

## Enable logging

The library has extensive logging capabilities.

```sh
export RUST_LOG=d3d9=info
wine my-game.exe
```

You can replace `info` with your desired logging level: error, warn, info, debug, trace.
