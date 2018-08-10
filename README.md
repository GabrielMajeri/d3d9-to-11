# Direct3D 9-to-11

## Scope of this project

This project is an attempt to convert [Direct3D 9](https://en.wikipedia.org/wiki/Direct3D#Direct3D_9) programs
to [Direct3D 11](https://en.wikipedia.org/wiki/Direct3D#Direct3D_11).
It reimplements the `d3d9.dll`, which contains the core D3D9 interfaces.

What is the point of this, considering Windows D3D9 drivers are already so optimised?
Well, on Linux, the situation isn't so good.

The whole idea is that this project would be combined with [DXVK](https://github.com/doitsujin/dxvk/), which would then translate D3D9 to D3D11.

## Why not VK9

You might have hear of the [VK9](https://github.com/disks86/VK9) project, which aims to convert D3D9 to Vulkan directly.

I would have loved to contribute to this project, but I don't know Vulkan very well.
Furthermore, I believe using Vulkan directly is error prone and there are already plenty of issues with such a project.

I respect Schaefer's work on VK9. However, I believe a more incremental approach, converting D3D9 to D3D11 is better,
since there is already a lot of work done for us by [DXVK](https://github.com/doitsujin/dxvk/).

Both VK9 and DXVK implement command stream multithreading, inspired by Wine's original [CSMT](https://github.com/wine-compholio/wine-staging/wiki/CSMT).
This feature reduces CPU utilisation and improves performance.
Instead of reimplementing this feature yet another time, this project assumes D3D11 (DXVK) does this for us.

## History of the project

I initially started work on my idea by forking DXVK and building on top of its infrastructure.
After a [discussion in a pull request](https://github.com/doitsujin/dxvk/pull/541),

I've realised it's better to keep the projects separate and allow DXVK's author to focus on the best D3D11 support,
while I work on a separate D3D9-to-D3D11 wrapper.

## Credits

- **Wine** for allowing us to run Windows programs on other OSes

- **DXVK** for making this project possible

- **VK9** for the original D3D9-to-Vulkan wrapper

## License

This project is licensed under the [Lesser GNU General Public License](LICENSE),
version 3 or (at your option) any later version.
