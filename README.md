# Direct3D 9-to-11

[![AppVeyor build status](https://ci.appveyor.com/api/projects/status/30d6a8gwk4w0u4h8?svg=true)](https://ci.appveyor.com/project/GuildMasterInfinite/d3d9-to-11)

## Scope of this project

This project is an attempt to convert [Direct3D 9](https://en.wikipedia.org/wiki/Direct3D#Direct3D_9) programs
to [Direct3D 11](https://en.wikipedia.org/wiki/Direct3D#Direct3D_11).
It reimplements the `d3d9.dll`, which contains the core D3D9 interfaces.

Most D3D9 games are CPU limited on modern PCs, since GPU power increased exponentially while CPUs fell behind.
Furthermore, most (old) games lack multithreading support, draining the CPU resource even more.

This project uplifts the games D3D9 graphics API calls to D3D11.

**Important**: this project ought be used together with [DXVK](https://github.com/doitsujin/dxvk/), which would then translate D3D11 to Vulkan.

## Documentation

All of the project's documentation is stored in the [docs](docs/index.md) directory and is checked into the repository.
Everyone is welcome to contribute to the docs. The docs are licensed under the [GNU Free Documentation License](docs/license.md).

## Building

See the [documentation on how to build from source](docs/building.md).

## Using

After building, see the [usage instructions](docs/usage.md).

## Credits

- **Wine** for allowing us to run Windows programs on other OSes

- **DXVK** for inspiration and making this project possible

- **VK9** for the original D3D9-to-Vulkan wrapper

## License

This project is licensed under the [Lesser GNU General Public License](LICENSE), version 3 or (at your option) any later version.
