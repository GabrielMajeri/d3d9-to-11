use winapi::shared::d3d9types::*;
use winapi::um::d3d11::*;

use crate::{Error, Result};

/// Converts D3D9's buffer/texture usage and pool flags to corresponding D3D11 flags.
///
/// Returns an error if a certain combination is invalid.
pub fn d3d_usage_to_d3d11(
    uflags: u32,
    pool: D3DPOOL,
) -> Result<(D3D11_USAGE, D3D11_CPU_ACCESS_FLAG)> {
    let mut usage = D3D11_USAGE_DEFAULT;
    let mut cpu_flags = 0;

    match pool {
        // Default resources are placed in VRAM.
        D3DPOOL_DEFAULT => {
            if uflags & D3DUSAGE_DYNAMIC != 0 {
                usage = D3D11_USAGE_DYNAMIC;
            }

            if uflags & D3DUSAGE_WRITEONLY != 0 {
                cpu_flags = D3D11_CPU_ACCESS_WRITE;
            } else {
                error!("Resource readback are not yet supported");
                Error::InvalidCall?
            }
        }
        // Managed resources are placed in VRAM if possible, and are backed by system RAM.
        D3DPOOL_MANAGED => {
            usage = D3D11_USAGE_DYNAMIC;
            cpu_flags = D3D11_CPU_ACCESS_WRITE;
        }
        // SystemMem resources are stored in RAM.
        // Because of this, they are not accessible in shaders.
        D3DPOOL_SYSTEMMEM => {
            usage = D3D11_USAGE_STAGING;
            cpu_flags = D3D11_CPU_ACCESS_WRITE | D3D11_CPU_ACCESS_READ;
        }
        _ => error!("Unsupported memory pool: {}", pool),
    }

    Ok((usage, cpu_flags))
}
