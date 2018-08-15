use comptr::ComPtr;
use winapi::shared::d3d9::*;

pub fn create_context() -> ComPtr<IDirect3D9> {
    let sdk_version = D3D_SDK_VERSION;

    let d3d9 = unsafe { Direct3DCreate9(sdk_version) };
    assert!(!d3d9.is_null(), "Failed to create IDirect3D9");

    ComPtr::new(d3d9)
}

pub fn run_tests(ctx: &ComPtr<IDirect3D9>) {
    lifetime(ctx.clone());
}

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
