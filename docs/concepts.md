# Graphics concepts mapping

The purpose of this file is to give the reader a general idea of how D3D9 concepts are mapped to D3D11.

In order to understand the Direct3D 9 API, [consult the official documentation][d3d9].
The same applies for [Direct3D 11's documentation][d3d11].

[d3d9]: https://docs.microsoft.com/en-us/windows/desktop/direct3d9/dx9-graphics-programming-guide
[d3d11]: https://docs.microsoft.com/en-us/windows/desktop/direct3d11/dx-graphics-overviews

## Core (IDirect3D9)

- This is the first interface an application creates, and it is used to query information
  about the installed GPUs and create logical devices.

- Builds on top of `IDXGIFactory`, the equivalent interface for D3D11.

### Adapter

- Wraps a `IDXGIAdapter`.

- Represents a physical device.
  There is a one-to-one mapping of adapters and physical GPUs.

- Also creates a `ID3D11Device`, since we need it to query capabilties and surface format support.

## Device (IDirect3DDevice9)

- Logical view of a GPU.
