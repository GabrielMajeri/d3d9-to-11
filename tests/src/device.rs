use comptr::ComPtr;
use std::ptr;
use winapi::shared::{d3d9::*, d3d9types::*, windef::HWND};

/// Creates a new D3D9 device.
pub fn create_device(ctx: &IDirect3D9, window: HWND) -> ComPtr<IDirect3DDevice9> {
    let mut pp = D3DPRESENT_PARAMETERS {
        // Leave these all blank to automatically select them.
        BackBufferWidth: 0,
        BackBufferHeight: 0,
        BackBufferFormat: D3DFMT_UNKNOWN,
        BackBufferCount: 1,
        // No need to use multisampling in the tests.
        MultiSampleType: 0,
        MultiSampleQuality: 0,
        SwapEffect: D3DSWAPEFFECT_DISCARD,
        hDeviceWindow: window,
        Windowed: 1,
        // Have the library create the DS buffer for us.
        EnableAutoDepthStencil: 1,
        AutoDepthStencilFormat: D3DFMT_D24S8,
        Flags: 0,
        FullScreen_RefreshRateInHz: 0,
        // Enable VSync
        PresentationInterval: 1,
    };

    let mut device = ptr::null_mut();

    let result = unsafe {
        ctx.CreateDevice(
            D3DADAPTER_DEFAULT,
            D3DDEVTYPE_HAL,
            window,
            D3DCREATE_MULTITHREADED,
            &mut pp,
            &mut device,
        )
    };

    assert_eq!(result, 0, "Failed to create device");

    ComPtr::new(device)
}
