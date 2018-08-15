//! This module and all submodules are dedicated to implementing
//! the absolutely massive IDirect3DDevice9 interface.

mod device;
pub use self::device::Device;

mod swapchain;
pub use self::swapchain::SwapChain;

#[macro_use]
pub(self) mod resource;

mod surface;
pub use self::surface::{Surface, SurfaceData};

mod texture;
pub use self::texture::Texture;
