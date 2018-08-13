use std::{cmp, mem, ptr};

use winapi::{
    shared::d3d9::*,
    shared::d3d9types::*,
    shared::dxgi::*,
    shared::dxgitype::*,
    shared::windef::HWND,
    um::unknwnbase::{IUnknown, IUnknownVtbl},
    um::winuser,
};

use com_impl::{implementation, interface};
use comptr::ComPtr;

use crate::{core::format::D3DFormatExt, core::new_com_interface, Error, Result};

/// Represents a swap chain, which is a queue of buffers
/// on which the app can draw.
///
/// The swap chain handles the presentation of the buffers,
/// i.e. the way they are sent to the screen, and tries to
/// avoid tearing or input latency.
#[interface(IUnknown, IDirect3DSwapChain9)]
pub struct SwapChain {
    // Parent device of this interface.
    parent: ComPtr<IDirect3DDevice9>,
    // The equivalent DXGI interface.
    swap_chain: ComPtr<IDXGISwapChain>,
    // Store these for retrieving them later.
    pp: D3DPRESENT_PARAMETERS,
}

impl SwapChain {
    /// Creates a new swap chain with the given parameters, which presents into a window.
    pub fn new(
        parent: ComPtr<IDirect3DDevice9>,
        device: &mut IUnknown,
        factory: &mut IDXGIFactory,
        pp: &mut D3DPRESENT_PARAMETERS,
        window: HWND,
    ) -> Result<ComPtr<IDirect3DSwapChain9>> {
        // First we need to set up the description of this swap chain.
        let mut sc_desc = {
            // Fill in the description of the back buffer.
            let buffer_desc = {
                // Determine the dimensions of the back buffer.
                let (width, height) = {
                    if pp.BackBufferWidth * pp.BackBufferHeight == 0 {
                        // In full-screen modes, we need to be told the exact resolution.
                        if pp.Windowed == 0 {
                            return Err(Error::InvalidCall);
                        }

                        // If either of these was 0, the app requested us to use the window's size.
                        unsafe {
                            let mut r = mem::zeroed();

                            winuser::GetClientRect(window, &mut r);

                            let width = (r.right - r.left) as u32;
                            let height = (r.bottom - r.top) as u32;

                            // We have to update these elements of the structure.
                            pp.BackBufferWidth = width;
                            pp.BackBufferHeight = height;

                            (width, height)
                        }
                    } else {
                        (pp.BackBufferWidth, pp.BackBufferHeight)
                    }
                };

                let refresh_rate = DXGI_RATIONAL {
                    Numerator: pp.FullScreen_RefreshRateInHz,
                    Denominator: 1,
                };

                // Determine the back buffer format.
                // We take a mut reference because we have to update the format
                // with the actual format we will be using.
                let fmt = &mut pp.BackBufferFormat;

                // If it's unknown, use the display's one.
                if *fmt == D3DFMT_UNKNOWN {
                    *fmt = D3DFMT_X8R8G8B8;
                }

                DXGI_MODE_DESC {
                    Width: width,
                    Height: height,
                    RefreshRate: refresh_rate,
                    Format: fmt.to_dxgi(),
                    ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
                    Scaling: DXGI_MODE_SCALING_UNSPECIFIED,
                }
            };

            let sample_desc = {
                let count = if pp.SwapEffect != D3DSWAPEFFECT_DISCARD {
                    error!("Multisampling is only supported with the DISCARD swap effect");
                    error!("Disabling multisample antialiasing");
                    1
                } else {
                    // Clamp between 1 and 16.
                    cmp::min(16, cmp::max(1, pp.MultiSampleType))
                };

                DXGI_SAMPLE_DESC {
                    Count: count,
                    // We ignore the MS quality: we either enable MS, or we don't.
                    Quality: 0,
                }
            };

            let buffer_usage = DXGI_USAGE_BACK_BUFFER | DXGI_USAGE_RENDER_TARGET_OUTPUT;

            let buffer_count = {
                if pp.SwapEffect == D3DSWAPEFFECT_COPY {
                    error!("Application requested multiple back buffers, even though swap effect is COPY");
                    error!("Limiting to one back buffer");
                    pp.BackBufferCount = 1;
                }

                // We have to clamp this to be between 1 and 3.
                // TODO: with D3D9Ex the limit goes up to 30.
                std::cmp::min(std::cmp::max(pp.BackBufferCount, 1), 3)
            };

            let swap_effect = match pp.SwapEffect {
                D3DSWAPEFFECT_DISCARD => DXGI_SWAP_EFFECT_DISCARD,
                se => {
                    error!("Unsupported swap effect: {}", se);
                    error!("Falling back to DISCARD");
                    DXGI_SWAP_EFFECT_DISCARD
                }
            };

            // TODO: we currently ignore the pp.Flags
            if pp.Flags != 0 {
                warn!("Unsupported presentation flags: {}", pp.Flags);
            }

            // TODO: store the pp.PresentationInterval and use it when calling DXGI's Present.

            DXGI_SWAP_CHAIN_DESC {
                BufferDesc: buffer_desc,
                SampleDesc: sample_desc,
                BufferUsage: buffer_usage,
                BufferCount: buffer_count,
                OutputWindow: window,
                Windowed: pp.Windowed,
                SwapEffect: swap_effect,
                // We don't need any special flags.
                Flags: 0,
            }
        };

        let swap_chain = unsafe {
            let mut ptr = ptr::null_mut();

            let result = factory.CreateSwapChain(device, &mut sc_desc, &mut ptr);

            if result != 0 {
                error!("Failed to create swap chain: {:#X}", result);
                return Err(Error::NotAvailable);
            }

            ComPtr::new(ptr)
        };

        let pp = *pp;

        let swap_chain = Self {
            __vtable: Self::create_vtable(),
            __refs: Self::create_refs(),
            parent,
            swap_chain,
            pp,
        };

        Ok(unsafe { new_com_interface(swap_chain) })
    }
}

impl Drop for SwapChain {
    fn drop(&mut self) {
        unsafe {
            // According to the DXGI docs, if the swap chain was created as full-screen
            // we need to make it windowed right before destroying it.
            self.swap_chain.SetFullscreenState(0, ptr::null_mut());
        }
    }
}

#[implementation(IUnknown, IDirect3DSwapChain9)]
impl SwapChain {
    fn present() {
        unimplemented!()
    }

    fn get_front_buffer_data() {
        unimplemented!()
    }

    fn get_back_buffer() {
        unimplemented!()
    }

    fn get_raster_status() {
        unimplemented!()
    }

    fn get_display_mode() {
        unimplemented!()
    }

    fn get_device() {
        unimplemented!()
    }

    fn get_present_parameters() {
        unimplemented!()
    }
}
