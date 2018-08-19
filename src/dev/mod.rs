//! This module and all submodules are dedicated to implementing
//! the absolutely massive IDirect3DDevice9 interface.

mod device;
pub use self::device::*;

mod swapchain;
pub use self::swapchain::*;

mod resource;
pub use self::resource::*;

mod surface;
pub use self::surface::*;

mod texture;
pub use self::texture::*;

pub mod state;

mod shader;
pub use self::shader::*;

mod buffer;
pub use self::buffer::*;
