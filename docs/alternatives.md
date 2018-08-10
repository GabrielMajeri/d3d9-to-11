# Alternatives

This document describes the principal alternatives users can pick from.

## Native Windows Direct3D 9 driver

If you have a computer with Windows installed, you can and should use the official driver which came with your GPU.
While D3D9 is considered a legacy technology, it will be supported for a long time to come.

## WineD3D

This is Wine's D3D9 implementation which converts the API calls to OpenGL.

However, historically OpenGL had issues with CPU / driver overhead, perhaps even more so than D3D9,
and while the GPU performance is good, in most benchmarks this implementation is CPU limited.

There are a few tehniques for reducing the CPU overhead. For example, Wine implements
[Command Stream Multithreading](https://github.com/wine-compholio/wine-staging/wiki/CSMT),
which allows WineD3D to take better advantage of the multiple cores found in modern PCs.
Unfortunately, performance is still somewhat lacking.

## VK9

[VK9](https://github.com/disks86/VK9) is an alternative to this project, which aims to convert D3D9 to Vulkan directly.

With all due respect for Mr. Schaefer's work on VK9, I believe it's a very hard task to convert a legacy API like D3D9
directly to Vulkan.

Even DXVK, which converts D3D11 calls to Vulkan, is a very complex library.

I believe a more layered approach, i.e. converting D3D9 to D3D11, is better, since there is already a lot of work done for us
by [DXVK](https://github.com/doitsujin/dxvk/), the layer on which we build, which performs the D3D11-to-Vulkan conversion.
