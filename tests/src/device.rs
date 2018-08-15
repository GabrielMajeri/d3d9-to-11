use comptr::ComPtr;
use std::{mem, ptr};
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

pub fn run_tests(dev: &ComPtr<IDirect3DDevice9>) {
    check_auto_rt_ds(&dev);
}

/// Gets the description of a surface.
fn surface_get_desc(surf: &IDirect3DSurface9) -> D3DSURFACE_DESC {
    unsafe {
        let mut desc = mem::uninitialized();
        let result = surf.GetDesc(&mut desc);
        assert_eq!(result, 0, "Failed to get surface description");
        desc
    }
}

// Checks that the default render target / depth buffer is correctly constructed.
fn check_auto_rt_ds(dev: &IDirect3DDevice9) {
    let rt = unsafe {
        let mut ptr = ptr::null_mut();
        let result = dev.GetRenderTarget(0, &mut ptr);
        assert_eq!(result, 0, "Failed to get default render target");
        ComPtr::new(ptr)
    };

    let rt_desc = surface_get_desc(&rt);
    assert_eq!(rt_desc.Usage, D3DUSAGE_RENDERTARGET);

    let ds = unsafe {
        let mut ptr = ptr::null_mut();
        let result = dev.GetDepthStencilSurface(&mut ptr);
        assert_eq!(result, 0, "Failed to get default depth buffer");
        ComPtr::new(ptr)
    };

    let ds_desc = surface_get_desc(&ds);
    assert_eq!(ds_desc.Usage, D3DUSAGE_DEPTHSTENCIL);

    assert_eq!(rt_desc.Width, ds_desc.Width);
    assert_eq!(rt_desc.Height, ds_desc.Height);
}
