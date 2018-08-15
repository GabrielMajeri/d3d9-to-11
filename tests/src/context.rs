use comptr::ComPtr;
use winapi::shared::{d3d9::*, d3d9types::*};

pub fn create_context() -> ComPtr<IDirect3D9> {
    let sdk_version = D3D_SDK_VERSION;

    let d3d9 = unsafe { Direct3DCreate9(sdk_version) };
    assert!(!d3d9.is_null(), "Failed to create IDirect3D9");

    ComPtr::new(d3d9)
}

pub fn run_tests(ctx: &ComPtr<IDirect3D9>) {
    lifetime(ctx.clone());
    check_common_formats(&ctx);
}

// Tests the Context's reference counting mechanisms.
fn lifetime(ctx: ComPtr<IDirect3D9>) {
    let count = unsafe { ctx.GetAdapterCount() };

    let clone;

    {
        clone = ctx.clone();
        std::mem::drop(ctx);
    }

    let new_count = unsafe { clone.GetAdapterCount() };

    assert_eq!(count, new_count);
}

// Ensures that some basic formats are supported.
fn check_common_formats(ctx: &IDirect3D9) {
    let check_support = |fmt| unsafe {
        let result = ctx.CheckDeviceFormat(
            D3DADAPTER_DEFAULT,
            D3DDEVTYPE_HAL,
            D3DFMT_A8R8G8B8,
            0,
            D3DRTYPE_TEXTURE,
            fmt,
        );
        assert_eq!(result, 0, "Format is not supported: {}", fmt);
    };

    check_support(D3DFMT_A8B8G8R8);
    check_support(D3DFMT_A16B16G16R16F);
    check_support(D3DFMT_DXT1);
}
