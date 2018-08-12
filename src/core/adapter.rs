use std::{cell::RefCell, collections::HashMap, mem, ptr};

use comptr::ComPtr;

use winapi::shared::d3d9caps::*;
use winapi::shared::d3d9types::*;
use winapi::shared::dxgi::{IDXGIAdapter, IDXGIOutput, DXGI_OUTPUT_DESC};
use winapi::shared::dxgitype::DXGI_MODE_DESC;
use winapi::shared::windef::HMONITOR;
use winapi::um::{d3d11::*, d3dcommon};

use crate::core::format::D3DFormatExt;

/// This class represents a physical graphics adapter (GPU).
pub struct Adapter {
    // Ordinal of this adapter in the list of GPUs.
    index: u32,
    // DXGI interface representing a physical device.
    adapter: ComPtr<IDXGIAdapter>,
    // The display attached to this device.
    output: Option<ComPtr<IDXGIOutput>>,
    // Cache the display's properties.
    output_desc: Option<DXGI_OUTPUT_DESC>,
    // Caches the supported display modes compatible with a certain format.
    mode_cache: RefCell<HashMap<D3DFORMAT, Box<[DXGI_MODE_DESC]>>>,
    // With D3D11, obtaining a device's capabilities or checking for texture format support
    // requires us to create the device first.
    device: ComPtr<ID3D11Device>,
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

            match result {
                0 => Some(ComPtr::new(output)),
                _ => {
                    // Some GPUs might have no outputs attached.
                    warn!("No outputs detected for adapter {}", index);
                    None
                }
            }
        };

        let output_desc = output.as_ref().map(|output| unsafe {
            let mut desc = mem::uninitialized();
            let result = output.GetDesc(&mut desc);
            assert_eq!(result, 0);
            desc
        });

        // We need to also create the D3D11 device now.;
        let mut feature_level = 0;
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
                &mut feature_level,
                ptr::null_mut(),
            );
            assert_eq!(result, 0, "Failed to create D3D11 device");
            ComPtr::new(device)
        };

        if feature_level < d3dcommon::D3D_FEATURE_LEVEL_11_0 {
            warn!("Your GPU doesn't support all of D3D11's features");
        }

        Self {
            index,
            adapter,
            output,
            output_desc,
            mode_cache: RefCell::new(HashMap::new()),
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
            ptr::copy_nonoverlapping(
                driver.as_ptr(),
                id.Driver.as_mut_ptr() as *mut u8,
                driver.len(),
            );
        }

        // Human readable device description.
        let dxgi_desc = crate::core::str::wstr_to_string(&desc.Description);
        let description = format!("{} {}", dxgi_desc, "(D3D 9-to-11 Device)");

        unsafe {
            ptr::copy_nonoverlapping(
                description.as_ptr(),
                id.Description.as_mut_ptr() as *mut u8,
                description.len(),
            );
        }

        // Fake GDI device name
        let device_name = format!("DISPLAY{}", self.index);
        unsafe {
            ptr::copy_nonoverlapping(
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
    pub fn mode_count(&self, fmt: D3DFORMAT) -> u32 {
        if self.output.is_none() || !fmt.is_display_mode_format() {
            return 0;
        }

        // It's likely the app will also call `get_mode` soon after calling this function,
        // so we cache the mode list now.
        self.cache_display_modes(fmt);

        let mode_cache = self.mode_cache.borrow();
        let modes = &mode_cache[&fmt];

        modes.len() as u32
    }

    /// Retrieves the display mode of a certain index.
    pub fn mode(&self, fmt: D3DFORMAT, index: u32) -> Option<D3DDISPLAYMODE> {
        if self.output.is_none() || !fmt.is_display_mode_format() {
            return None;
        }

        // See if we need to update the cache.
        self.cache_display_modes(fmt);

        // Cache should contain an empty vector even if a format is not supported.
        let mode_cache = self.mode_cache.borrow();
        let modes = &mode_cache[&fmt];

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
        let lacks_support = check_rt(D3DRTYPE_SURFACE, D3D11_FORMAT_SUPPORT_TEXTURE2D) ||
            check_rt(D3DRTYPE_VOLUME, D3D11_FORMAT_SUPPORT_TEXTURE3D) ||
            check_rt(D3DRTYPE_TEXTURE, D3D11_FORMAT_SUPPORT_TEXTURE2D) ||
            check_rt(D3DRTYPE_VOLUMETEXTURE, D3D11_FORMAT_SUPPORT_TEXTURE3D) ||
            check_rt(D3DRTYPE_CUBETEXTURE, D3D11_FORMAT_SUPPORT_TEXTURECUBE) ||
            check_rt(D3DRTYPE_VERTEXBUFFER, D3D11_FORMAT_SUPPORT_IA_VERTEX_BUFFER) ||
            check_rt(D3DRTYPE_INDEXBUFFER, D3D11_FORMAT_SUPPORT_IA_INDEX_BUFFER) ||

            // Now we also need to check the proper usage.
            check_usage(D3DUSAGE_AUTOGENMIPMAP, D3D11_FORMAT_SUPPORT_MIP_AUTOGEN) ||
            check_usage(D3DUSAGE_RENDERTARGET, D3D11_FORMAT_SUPPORT_RENDER_TARGET) ||
            check_usage(D3DUSAGE_DEPTHSTENCIL, D3D11_FORMAT_SUPPORT_DEPTH_STENCIL);

        // Due to the way the check functions are written, we need to negate this result.
        !lacks_support
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

    /// Returns the capabilities of this device.
    pub fn caps(&self) -> D3DCAPS9 {
        D3DCAPS9 {
            DeviceType: D3DDEVTYPE_HAL,
            AdapterOrdinal: self.index,
            Caps: 0,
            // TODO: implement D3DCAPS2_CANSHARERESOURCE for D3D9Ex
            Caps2: D3DCAPS2_CANAUTOGENMIPMAP
                | D3DCAPS2_CANCALIBRATEGAMMA
                | D3DCAPS2_FULLSCREENGAMMA
                | D3DCAPS2_CANMANAGERESOURCE
                | D3DCAPS2_DYNAMICTEXTURES,
            Caps3: D3DCAPS3_ALPHA_FULLSCREEN_FLIP_OR_DISCARD
                | D3DCAPS3_COPY_TO_VIDMEM
                | D3DCAPS3_COPY_TO_SYSTEMMEM
                | D3DCAPS3_LINEAR_TO_SRGB_PRESENTATION,
            // A lot of these features are bitflags, so we set all bits.
            PresentationIntervals: !0,
            CursorCaps: !0,
            DevCaps: !0,
            PrimitiveMiscCaps: !0,
            RasterCaps: !0,
            ZCmpCaps: !0,
            SrcBlendCaps: !0,
            DestBlendCaps: !0,
            AlphaCmpCaps: !0,
            ShadeCaps: !0,
            // This cap indicates lack of support, so we mask it.
            TextureCaps: !D3DPTEXTURECAPS_NOPROJECTEDBUMPENV,
            TextureFilterCaps: !0,
            CubeTextureFilterCaps: !0,
            VolumeTextureFilterCaps: !0,
            TextureAddressCaps: !0,
            VolumeTextureAddressCaps: !0,
            LineCaps: !0,
            // The following caps are guaranteed on D3D11 hardware.
            MaxTextureWidth: 16384,
            MaxTextureHeight: 16384,
            MaxVolumeExtent: 2048,
            MaxTextureRepeat: 8192,
            MaxTextureAspectRatio: 16384,
            MaxAnisotropy: 16,
            // The depth buffer is at most a 32-bit float.
            MaxVertexW: std::f32::MAX,
            // Modern GPUs have really big guard bands
            GuardBandLeft: 0.0,
            GuardBandTop: 0.0,
            GuardBandRight: 0.0,
            GuardBandBottom: 0.0,
            ExtentsAdjust: 0.0,
            StencilCaps: !0,
            FVFCaps: !0,
            TextureOpCaps: !0,
            // These are pretty much unlimited on modern hardware.
            MaxSimultaneousTextures: std::u32::MAX,
            MaxTextureBlendStages: std::u32::MAX,
            MaxActiveLights: std::u32::MAX,
            MaxUserClipPlanes: std::u32::MAX,
            MaxPrimitiveCount: std::u32::MAX,
            MaxVertexIndex: std::u32::MAX,
            MaxVertexBlendMatrices: std::u32::MAX,
            MaxVertexBlendMatrixIndex: std::u32::MAX,
            VertexProcessingCaps: !0,
            MaxPointSize: std::f32::MAX,
            MaxStreams: 16,
            MaxStreamStride: 1 << 31,
            VertexShaderVersion: 0xFFFE_0000 | (3 << 8),
            MaxVertexShaderConst: 1 << 16,
            PixelShaderVersion: 0xFFFF_0000 | (3 << 8),
            PixelShader1xMaxValue: 8.0,
            DevCaps2: !0,
            MaxNpatchTessellationLevel: 256.0,
            Reserved5: 0,
            // TODO: multihead support
            MasterAdapterOrdinal: self.index,
            NumberOfAdaptersInGroup: 1,
            AdapterOrdinalInGroup: 0,
            DeclTypes: !0,
            NumSimultaneousRTs: 8,
            StretchRectFilterCaps: !0,
            VS20Caps: D3DVSHADERCAPS2_0 {
                Caps: !0,
                DynamicFlowControlDepth: 24,
                NumTemps: 16384,
                StaticFlowControlDepth: 1 << 31,
            },
            PS20Caps: D3DPSHADERCAPS2_0 {
                Caps: !0,
                DynamicFlowControlDepth: 24,
                NumTemps: !16384,
                StaticFlowControlDepth: 1 << 31,
                NumInstructionSlots: 1 << 31,
            },
            VertexTextureFilterCaps: !0,
            MaxVShaderInstructionsExecuted: !0,
            MaxPShaderInstructionsExecuted: !0,
            MaxVertexShader30InstructionSlots: 32768,
            MaxPixelShader30InstructionSlots: 32768,
        }
    }

    /// Returns the (primary) monitor of this adapter.
    pub fn monitor(&self) -> HMONITOR {
        self.output_desc
            .map(|desc| desc.Monitor)
            .unwrap_or(ptr::null_mut())
    }

    /// Retrieves the output's display modes and caches them.
    fn cache_display_modes(&self, fmt: D3DFORMAT) {
        let output = match self.output {
            Some(ref output) => output,
            None => return,
        };

        {
            let mode_cache = self.mode_cache.borrow();

            // Nothing to do if already in cache.
            if mode_cache.contains_key(&fmt) {
                return;
            }
        }

        let format = fmt.to_dxgi();
        let flags = 0;

        // Determine how big the list should be.
        let mut num = 0;
        unsafe {
            output.GetDisplayModeList(format, flags, &mut num, ptr::null_mut());
        }

        let mode_descs = unsafe {
            // Reserve space and store the mode descriptions.
            let mut mode_descs = {
                let sz = num as usize;
                let mut v = Vec::with_capacity(sz);
                v.set_len(sz);
                v.into_boxed_slice()
            };

            output.GetDisplayModeList(format, flags, &mut num, mode_descs.as_mut_ptr());

            mode_descs
        };

        let mut mode_cache = self.mode_cache.borrow_mut();

        // Even if the function calls fail, we still store the empty array
        // to determine if they're cached or not.
        mode_cache.insert(fmt, mode_descs);
    }
}
