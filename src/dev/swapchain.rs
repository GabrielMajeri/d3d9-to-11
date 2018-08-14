use std::{cmp, mem, ptr};

use winapi::{
    shared::{d3d9::*, d3d9types::*, dxgi::*, dxgitype::*, windef::HWND, winerror},
    um::{
        d3d11::ID3D11Texture2D,
        unknwnbase::{IUnknown, IUnknownVtbl},
        winuser,
    },
    Interface,
};

use com_impl::{implementation, interface};
use comptr::ComPtr;

use crate::{
    core::{format::D3DFormatExt, *},
    Error, Result,
};

use super::Surface;

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
    // Determines how many vblanks to wait before presenting:
    // 0 -> no vsync
    // 1 through 4 -> vsync, with `refresh rate = (monitor Hz / sync_interval)`.
    sync_interval: u32,
}

impl SwapChain {
    /// Creates a new swap chain with the given parameters, which presents into a window.
    pub fn new(
        parent: ComPtr<IDirect3DDevice9>,
        device: &mut IUnknown,
        factory: &IDXGIFactory,
        pp: &mut D3DPRESENT_PARAMETERS,
        window: HWND,
    ) -> Result<*mut IDirect3DSwapChain9> {
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
                    *fmt = D3DFMT_A8R8G8B8;
                }

                DXGI_MODE_DESC {
                    Width: width,
                    Height: height,
                    RefreshRate: refresh_rate,
                    Format: fmt.to_dxgi_display_format(),
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

            check_hresult!(result, "Failed to create swap chain");

            ComPtr::new(ptr)
        };

        let pp = *pp;

        // Clamp this to 4.
        let sync_interval = cmp::min(pp.PresentationInterval, 4);

        let swap_chain = Self {
            __vtable: Self::create_vtable(),
            __refs: Self::create_refs(),
            parent,
            swap_chain,
            pp,
            sync_interval,
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
    /// Presents the back buffer to the screen, and moves to the next buffer in the chain.
    fn present(&self, src: usize, dest: usize, wnd: HWND, dirty: usize, flags: u32) -> Error {
        if src != 0 || dest != 0 || dirty != 0 {
            // Check if the app is even allowed to partially present.
            if self.pp.SwapEffect != D3DSWAPEFFECT_COPY {
                return Error::InvalidCall;
            }

            unimplemented!("Partial present is not yet supported");
        }

        if !wnd.is_null() {
            unimplemented!("Presenting to a different window is not supported");
        }

        let mut fl = 0;

        // These flags are missing from `winapi`.
        const DONOTWAIT: u32 = 1;
        const LINEAR_CONTENT: u32 = 2;

        if flags & DONOTWAIT != 0 {
            fl |= DXGI_PRESENT_DO_NOT_WAIT;
        }

        // TODO: determine what we have to do to support sRGB.
        if flags & LINEAR_CONTENT != 0 {
            warn!("sRGB / gamma correction not yet supported");
        }

        // Try to present.
        let result = unsafe { self.swap_chain.Present(self.sync_interval, fl) };

        match result {
            0 => Error::Success,
            winerror::DXGI_ERROR_WAS_STILL_DRAWING => Error::WasStillDrawing,
            hr => check_hresult!(hr, "Failed to present to screen"),
        }
    }

    /// Copies data from the front buffer into a surface.
    fn get_front_buffer_data(_fb: *mut IDirect3DSurface9) -> Error {
        // TODO: we need to get the front buffer, then copy its data into the passed-in surface.
        // We also need to ensure the format is converted to a format D3D9 supports.
        unimplemented!()
    }

    /// Retrieves the the back buffer's surface.
    fn get_back_buffer(
        &self,
        idx: u32,
        ty: D3DBACKBUFFER_TYPE,
        surf: *mut *mut IDirect3DSurface9,
    ) -> Error {
        let surf = check_mut_ref(surf)?;

        // Buffer indices start from 0.
        if idx >= self.pp.BackBufferCount {
            return Error::InvalidCall;
        }

        // The docs specify that mono is the only valid type.
        if ty != D3DBACKBUFFER_TYPE_MONO {
            return Error::InvalidCall;
        }

        // Retrieve the 2D texture representing this back buffer.
        let buffer = unsafe {
            let mut ptr = ptr::null_mut();
            let uuid = ID3D11Texture2D::uuidof();

            let result = self.swap_chain.GetBuffer(
                idx,
                &uuid,
                &mut ptr,
            );

            check_hresult!(result, "Failed to retrieve swap chain buffer");

            ComPtr::new(ptr as *mut ID3D11Texture2D)
        };

        // Create and return a pointer to the surface.
        *surf = Surface::from_texture(self.parent.clone(), buffer, 0).into();

        Error::Success
    }

    /// Gets the status of the current scanline the rasterizer is processing.
    fn get_raster_status(&self, rs: *mut D3DRASTER_STATUS) -> Error {
        check_mut_ref(rs)?;

        // We reported in the device caps that we don't support this.
        Error::NotAvailable
    }

    /// Retrieves the swap chain's display mode.
    fn get_display_mode(&self, dm: *mut D3DDISPLAYMODE) -> Error {
        let dm = check_mut_ref(dm)?;
        let pp = &self.pp;

        *dm = D3DDISPLAYMODE {
            Width: pp.BackBufferWidth,
            Height: pp.BackBufferHeight,
            Format: pp.BackBufferFormat,
            RefreshRate: pp.FullScreen_RefreshRateInHz,
        };

        Error::Success
    }

    /// Gets the device which created this object.
    fn get_device(&self, device: *mut *mut IDirect3DDevice9) -> Error {
        let device = check_mut_ref(device)?;
        *device = self.parent.clone().into();
        Error::Success
    }

    /// Retrieves the presentation parameters this swap chain was created with.
    fn get_present_parameters(&self, pp: *mut D3DPRESENT_PARAMETERS) -> Error {
        let pp = check_mut_ref(pp)?;
        *pp = self.pp;
        Error::Success
    }
}
