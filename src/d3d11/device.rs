use std::ops;

use winapi::um::d3d11::ID3D11Device;

use comptr::ComPtr;

/// Wraps a D3D11 device.
#[derive(Clone)]
pub struct Device {
    device: ComPtr<ID3D11Device>,
}

impl Device {
    /// Creates a new D3D11 device wrapper.
    pub fn new(device: ComPtr<ID3D11Device>) -> Self {
        Self { device }
    }
}

impl ops::Deref for Device {
    type Target = ID3D11Device;

    fn deref(&self) -> &ID3D11Device {
        &self.device
    }
}
