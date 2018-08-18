use winapi::um::d3d11::*;

use crate::core::*;
use crate::Result;

/// Converts D3D9's buffer/texture usage and pool flags to corresponding D3D11 flags.
///
/// Returns an error if a certain combination is invalid.
pub fn d3d_usage_to_d3d11(
    uflags: UsageFlags,
    pool: MemoryPool,
) -> Result<(D3D11_USAGE, D3D11_BIND_FLAG, D3D11_CPU_ACCESS_FLAG)> {
    let mut usage = D3D11_USAGE_DEFAULT;
    let mut cpu_flags = 0;

    let write_to = UsageFlags::DYNAMIC | UsageFlags::WRITE_ONLY;

    match pool {
        MemoryPool::Default => {
            if uflags.intersects(write_to) {
                usage = D3D11_USAGE_DYNAMIC;
                cpu_flags = D3D11_CPU_ACCESS_WRITE;
            }
        }
        MemoryPool::Managed => {
            usage = D3D11_USAGE_DYNAMIC;
            cpu_flags = D3D11_CPU_ACCESS_WRITE;
        }
        MemoryPool::SystemMem => {
            if uflags.intersects(write_to) {
                usage = D3D11_USAGE_DYNAMIC;
                cpu_flags = D3D11_CPU_ACCESS_WRITE;
            } else {
                usage = D3D11_USAGE_STAGING;
                cpu_flags = D3D11_CPU_ACCESS_WRITE | D3D11_CPU_ACCESS_READ;
            }
        }
        _ => error!("Unsupported memory pool: {:?}", pool),
    }

    let bind_flags = if usage != D3D11_USAGE_STAGING {
        // Even if the app doesn't end up using this in a shader,
        // this is the only bind flag we could choose for it.
        D3D11_BIND_SHADER_RESOURCE
    } else {
        0
    };

    Ok((usage, bind_flags, cpu_flags))
}
