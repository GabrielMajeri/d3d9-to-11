use comptr::ComPtr;
use crate::core::Context;
use crate::Error;
use std::sync::Once;
use winapi::shared::d3d9::IDirect3D9Ex;

static INIT_LOGGER: Once = Once::new();

#[no_mangle]
pub unsafe extern "system" fn Direct3DCreate9(sdk_version: u32) -> Option<ComPtr<Context>> {
    // This function could be called multiple times during the lifetime of the DLL,
    // so we must protect the logger initializer.
    INIT_LOGGER.call_once(|| {
        env_logger::init();
    });

    // Try to identify which version of the D3D9 the app was built against.
    // This could be used to implement compatibility workarounds if needed.
    match sdk_version {
        32 => info!("D3D9 version 9.0c"),
        _ => warn!("Unknown D3D9 SDK version {}", sdk_version),
    }

    Context::new().ok()
}

#[no_mangle]
pub unsafe extern "system" fn Direct3DCreate9Ex(
    _sdk_version: u32,
    _ptr: *mut *mut IDirect3D9Ex,
) -> Error {
    error!("D3D9Ex is not yet supported");
    Error::NotAvailable
}
