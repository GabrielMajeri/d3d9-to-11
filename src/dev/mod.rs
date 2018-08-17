//! This module and all submodules are dedicated to implementing
//! the absolutely massive IDirect3DDevice9 interface.

mod device;
pub use self::device::Device;

mod swapchain;
pub use self::swapchain::SwapChain;

mod resource;
pub use self::resource::Resource;

mod surface;
pub use self::surface::{Surface, SurfaceData};

mod texture;
pub use self::texture::Texture;

pub mod state;

pub mod shader;

mod buffer;
pub use self::buffer::{IndexBuffer, VertexBuffer};
