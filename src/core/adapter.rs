use std::{collections::HashMap, mem, ptr};

use comptr::ComPtr;

use winapi::shared::d3d9types::*;
use winapi::shared::dxgi::{IDXGIAdapter, IDXGIOutput};
use winapi::shared::dxgitype::DXGI_MODE_DESC;
use winapi::um::{d3d11::*, d3dcommon};

use crate::core::format::D3DFormatExt;

/// This class represents a physical graphics adapter (GPU).
pub struct Adapter {
    // Ordinal of this adapter in the list of GPUs.
    index: u32,
    // DXGI interface representing a physical device.
    adapter: ComPtr<IDXGIAdapter>,
    // The display attached to this device.
    output: ComPtr<IDXGIOutput>,
    // Caches the supported display modes compatible with a certain format.
    mode_cache: HashMap<D3DFORMAT, Box<[DXGI_MODE_DESC]>>,
    // With D3D11, obtaining a device's capabilities or checking for texture format support
    // requires us to create the device first.
    device: ComPtr<ID3D11Device>,
    // The highest-supported feature level of this device.
    //feature_level: d3dcommon::D3D_FEATURE_LEVEL,
}

impl Adapter {
    /// Creates a new adapter.
    pub fn new(index: u32, adapter: *mut IDXGIAdapter) -> Self {
        let adapter = ComPtr::new(adapter);

        // D3D9 only supports one monitor per adapter.
        // TODO: allow user to choose which monitor they want to use.
        let output = unsafe {
            let mut output = ptr::null_mut();
            let result = adapter.EnumOutputs(0, &mut output);
            assert_eq!(result, 0, "Failed to retrieve adapter's output");
            ComPtr::new(output)
        };

        // We need to also create the D3D11 device now.;
        let mut _feature_level = 0;
        let device = unsafe {
            let mut device = ptr::null_mut();
            let result = D3D11CreateDevice(
                // Create a device for the adapter we own.
                adapter.get_mut(),
                d3dcommon::D3D_DRIVER_TYPE_UNKNOWN,
                ptr::null_mut(),
                // No additional flags.
                0,
                // We will use whichever feature level is supported.
                ptr::null_mut(),
                0,
                D3D11_SDK_VERSION,
                &mut device,
                &mut _feature_level,
                ptr::null_mut(),
            );
            assert_eq!(result, 0, "Failed to create D3D11 device");
            ComPtr::new(device)
        };

        Self {
            index,
            adapter,
            output,
            mode_cache: HashMap::new(),
            device,
        }
    }

    /// Retrieves a description of this adapter.
    pub fn identifier(&self) -> D3DADAPTER_IDENTIFIER9 {
        let desc = unsafe {
            let mut desc = mem::uninitialized();
            let result = self.adapter.GetDesc(&mut desc);
            assert_eq!(result, 0, "Failed to get adapter description");
            desc
        };

        let mut id: D3DADAPTER_IDENTIFIER9 = unsafe { mem::zeroed() };

        // Internal identifier of the driver.
        let driver = "D3D 9-to-11 Driver";
        unsafe {
            ptr::copy(
                driver.as_ptr(),
                id.Driver.as_mut_ptr() as *mut u8,
                driver.len(),
            );
        }

        // Human readable device description.
        let dxgi_desc = crate::core::str::wstr_to_string(&desc.Description);
        let description = format!("{} {}", dxgi_desc, "(D3D 9-to-11 Device)");
        unsafe {
            ptr::copy(
                description.as_ptr(),
                id.Description.as_mut_ptr() as *mut u8,
                description.len(),
            );
        }

        // Fake GDI device name
        let device_name = format!("DISPLAY{}", self.index);
        unsafe {
            ptr::copy(
                device_name.as_ptr(),
                id.DeviceName.as_mut_ptr() as *mut u8,
                device_name.len(),
            );
        }

        unsafe {
            *id.DriverVersion.QuadPart_mut() = 1;
        }

        // These fields are passed-through.
        id.VendorId = desc.VendorId;
        id.DeviceId = desc.DeviceId;
        id.SubSysId = desc.SubSysId;
        id.Revision = desc.Revision;

        // D3D9 wants a 128-bit unique adapter identifier.
        // We don't have anything like that available, so we combine a 64-bit LUID with the adapter's index.
        // TODO: need to find a safer way to do this. Rust doesn't like raw pointer aliasing like C++ does.

        id.WHQLLevel = 1;

        id
    }

    /// Retrieves the number of display modes which match the requested format.
    pub fn mode_count(&mut self, fmt: D3DFORMAT) -> u32 {
        // It's likely the app will also call `get_mode` soon after calling this function,
        // so we cache the mode list now.
        self.cache_display_modes(fmt);

        let modes = &self.mode_cache[&fmt];

        modes.len() as u32
    }

    /// Retrieves the display mode of a certain index.
    pub fn mode(&mut self, fmt: D3DFORMAT, index: u32) -> Option<D3DDISPLAYMODE> {
        // See if we need to update the cache.
        self.cache_display_modes(fmt);

        // Cache should contain an empty vector even if a format is not supported.
        let modes = &self.mode_cache[&fmt];

        modes.get(index as usize)
            // Fill in the structure if it was found.
            .map(|mode| D3DDISPLAYMODE {
                Width: mode.Width,
                Height: mode.Height,
                RefreshRate: {
                    let rf = mode.RefreshRate;
                    if rf.Denominator == 0 {
                        0
                    } else {
                        rf.Numerator / rf.Denominator
                    }
                },
                Format: fmt,
            })
    }

    /// Checks if a given format is supported for a specific resource usage.
    pub fn is_format_supported(&self, fmt: D3DFORMAT, rt: D3DRESOURCETYPE, usage: u32) -> bool {
        let fmt = fmt.to_dxgi();

        let support = unsafe {
            let mut sp = 0;
            if self.device.CheckFormatSupport(fmt, &mut sp) != 0 {
                return false;
            }
            sp
        };

        // Returns true if a resource type is _not_ supported.
        let check_rt = |d3d9_rt, sp| (rt == d3d9_rt) && ((support & sp) == 0);
        let check_usage = |d3d9_usage, uf| ((usage & d3d9_usage) != 0) && ((support & uf) == 0);

        // First we have to check the resource type.
        check_rt(D3DRTYPE_SURFACE, D3D11_FORMAT_SUPPORT_TEXTURE2D) ||
        check_rt(D3DRTYPE_VOLUME, D3D11_FORMAT_SUPPORT_TEXTURE3D) ||
        check_rt(D3DRTYPE_TEXTURE, D3D11_FORMAT_SUPPORT_TEXTURE2D) ||
        check_rt(D3DRTYPE_VOLUMETEXTURE, D3D11_FORMAT_SUPPORT_TEXTURE3D) ||
        check_rt(D3DRTYPE_CUBETEXTURE, D3D11_FORMAT_SUPPORT_TEXTURECUBE) ||
        check_rt(D3DRTYPE_VERTEXBUFFER, D3D11_FORMAT_SUPPORT_IA_VERTEX_BUFFER) ||
        check_rt(D3DRTYPE_INDEXBUFFER, D3D11_FORMAT_SUPPORT_IA_INDEX_BUFFER) ||

        // Now we also need to check the proper usage.
        check_usage(D3DUSAGE_AUTOGENMIPMAP, D3D11_FORMAT_SUPPORT_MIP_AUTOGEN) ||
        check_usage(D3DUSAGE_RENDERTARGET, D3D11_FORMAT_SUPPORT_RENDER_TARGET) ||
        check_usage(D3DUSAGE_DEPTHSTENCIL, D3D11_FORMAT_SUPPORT_DEPTH_STENCIL)
    }

    /// Checks if we support multisampling for a given format.
    /// Returns the maximum quality level supported for a given format.
    pub fn is_multisampling_supported(&self, fmt: D3DFORMAT, ms: D3DMULTISAMPLE_TYPE) -> u32 {
        let fmt = fmt.to_dxgi();

        let mut quality = 0;
        unsafe {
            // Even if this fails, quality is initialized to 0.
            self.device
                .CheckMultisampleQualityLevels(fmt, ms, &mut quality);
        }

        quality
    }

    /// Retrieves the output's display modes and caches them.
    fn cache_display_modes(&mut self, fmt: D3DFORMAT) {
        // Nothing to do if already in cache.
        if self.mode_cache.contains_key(&fmt) {
            return;
        }

        let format = fmt.to_dxgi();
        let flags = 0;

        // Determine how big the list should be.
        let mut num = 0;
        unsafe {
            self.output
                .GetDisplayModeList(format, flags, &mut num, ptr::null_mut());
        }

        let mode_descs = unsafe {
            // Reserve space and store the mode descriptions.
            let mut mode_descs = {
                let sz = num as usize;
                let mut v = Vec::with_capacity(sz);
                v.set_len(sz);
                v.into_boxed_slice()
            };

            self.output
                .GetDisplayModeList(format, flags, &mut num, mode_descs.as_mut_ptr());

            mode_descs
        };

        // Even if the function calls fail, we still store the empty array
        // to determine if they're cached or not.
        self.mode_cache.insert(fmt, mode_descs);
    }
}
