//! This module and all submodules are dedicated to implementing
//! the absolutely massive IDirect3DDevice9 interface.

mod device;
pub use self::device::Device;

mod swapchain;
pub use self::swapchain::SwapChain;

mod resource;
pub use self::resource::Resource;

mod surface;
pub use self::surface::Surface;
