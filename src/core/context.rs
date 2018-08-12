use std::{mem, ptr};

use comptr::ComPtr;
use winapi::ctypes::c_void;
use winapi::shared::d3d9::*;
use winapi::shared::d3d9caps::D3DCAPS9;
use winapi::shared::d3d9types::*;
use winapi::shared::dxgi;
use winapi::shared::windef::{HMONITOR, HWND};
use winapi::um::winuser;
use winapi::Interface;
use winapi::{
    shared::d3d9::{IDirect3D9, IDirect3D9Vtbl},
    um::unknwnbase::{IUnknown, IUnknownVtbl},
};

use com_impl::{implementation, interface};

use super::*;
use crate::core::format::D3DFormatExt;
use crate::{Error, Result};

/// D3D9 interface which stores all application context.
///
/// Similar in role to a DXGI factory.

#[interface(IUnknown, IDirect3D9)]
pub struct Context {
    factory: ComPtr<dxgi::IDXGIFactory>,
    adapters: Vec<Adapter>,
}

impl Context {
    /// Creates a new D3D9 context.
    pub fn new() -> Self {
        // We first have to create a factory, which is the equivalent of this interface in DXGI terms.
        let factory = unsafe {
            let uuid = dxgi::IDXGIFactory::uuidof();
            let mut factory: *mut dxgi::IDXGIFactory = ptr::null_mut();

            let result = dxgi::CreateDXGIFactory(&uuid, &mut factory as *mut _ as usize as *mut _);
            assert_eq!(result, 0, "Failed to create DXGI factory");

            ComPtr::new(factory)
        };

        // Now we can enumerate all the graphics adapters on the system.
        let mut adapter = ptr::null_mut();
        let adapters: Vec<_> = (0..)
            .take_while(|&id| unsafe { factory.EnumAdapters(id, &mut adapter) == 0 })
            .map(|id| Adapter::new(id, adapter))
            .collect();

        Self {
            __vtable: Self::create_vtable(),
            __refs: Self::create_refs(),
            factory,
            adapters,
        }
    }

    fn check_adapter(&self, adapter: u32) -> Result<&Adapter> {
        self.adapters
            .get(adapter as usize)
            .ok_or(Error::InvalidCall)
    }

    fn check_devty(&self, dev_ty: D3DDEVTYPE) -> Error {
        match dev_ty {
            D3DDEVTYPE_HAL => Error::Success,
            _ => Error::InvalidCall,
        }
    }
}

#[implementation(IUnknown, IDirect3D9)]
impl Context {
    /// Used to register a software rasterizer.
    fn register_software_device(&self, init_fn: *mut c_void) -> Error {
        check_not_null(init_fn)?;

        warn!("Application tried to register software device");

        // We don't suppor software rendering, but we report success here since
        // this call would simply allow software rasterization in cases where
        // the graphics adapter does not support it.
        Error::Success
    }

    /// Returns the number of GPUs installed on the system.
    fn get_adapter_count(&self) -> u32 {
        self.adapters.len() as u32
    }

    /// Returns a description of a GPU.
    fn get_adapter_identifier(
        &self,
        adapter: u32,
        // Note: we ignore the flag, since it's only possible value, D3DENUM_WHQL_LEVEL,
        // is deprecated and irrelevant on Wine / newer versions of Windows.
        _flags: u32,
        ident: *mut D3DADAPTER_IDENTIFIER9,
    ) -> Error {
        let adapter = self.check_adapter(adapter)?;
        let ident = check_mut_ref(ident)?;

        *ident = adapter.identifier();

        Error::Success
    }

    /// Returns the number of display modes with a certain format an adapter supports.
    fn get_adapter_mode_count(&mut self, adapter: u32, fmt: D3DFORMAT) -> u32 {
        self.adapters
            .get_mut(adapter as usize)
            .map(|adapter| adapter.mode_count(fmt))
            .unwrap_or_default()
    }

    /// Retrieves the list of display modes.
    fn enum_adapter_modes(
        &mut self,
        adapter: u32,
        fmt: D3DFORMAT,
        i: u32,
        mode: *mut D3DDISPLAYMODE,
    ) -> Error {
        let adapter = self.check_adapter(adapter)?;
        let mode = check_mut_ref(mode)?;

        *mode = adapter.mode(fmt, i).ok_or(Error::NotAvailable)?;

        Error::Success
    }

    /// Retrieve the current display mode of the GPU.
    fn get_adapter_display_mode(&self, adapter: u32, mode: *mut D3DDISPLAYMODE) -> Error {
        let monitor = self.get_adapter_monitor(adapter);
        let mode = check_mut_ref(mode)?;

        let mi = unsafe {
            let mut mi: winuser::MONITORINFO = mem::uninitialized();
            mi.cbSize = mem::size_of_val(&mi) as u32;
            let result = winuser::GetMonitorInfoW(monitor, &mut mi);
            assert_ne!(result, 0, "Failed to retrieve monitor info");
            mi
        };

        let rc = mi.rcMonitor;

        mode.Width = (rc.right - rc.left) as u32;
        mode.Height = (rc.bottom - rc.top) as u32;
        // 0 indicates an adapter-default rate.
        mode.RefreshRate = 0;
        // This format is usually what modern displays use internally.
        mode.Format = D3DFMT_X8R8G8B8;

        Error::Success
    }

    /// Checks if an adapter is hardware accelerated.
    fn check_device_type(
        &self,
        adapter: u32,
        ty: D3DDEVTYPE,
        adapter_fmt: D3DFORMAT,
        _bb_fmt: D3DFORMAT,
        _windowed: u32,
    ) -> Error {
        self.check_adapter(adapter)?;
        self.check_devty(ty)?;

        // We support hardware accel with all valid formats.
        if adapter_fmt.is_display_mode_format() {
            Error::Success
        } else {
            Error::NotAvailable
        }
    }

    /// Checks if a certain format can be used for something.
    fn check_device_format(
        &self,
        adapter: u32,
        ty: D3DDEVTYPE,
        _adapter_fmt: D3DFORMAT,
        usage: u32,
        rt: D3DRESOURCETYPE,
        check_fmt: D3DFORMAT,
    ) -> Error {
        let adapter = self.check_adapter(adapter)?;
        self.check_devty(ty)?;

        if adapter.is_format_supported(check_fmt, rt, usage) {
            Error::Success
        } else {
            Error::NotAvailable
        }
    }

    /// Checks if a format can be used with multisampling.
    fn check_device_multi_sample_type(
        &self,
        adapter: u32,
        ty: D3DDEVTYPE,
        surface_fmt: D3DFORMAT,
        _windowed: u32,
        mst: D3DMULTISAMPLE_TYPE,
        quality: *mut u32,
    ) -> Error {
        let adapter = self.check_adapter(adapter)?;
        self.check_devty(ty)?;

        let quality = check_mut_ref(quality);

        let q = adapter.is_multisampling_supported(surface_fmt, mst);

        // Return the maximum quality level, if requested.
        if let Ok(quality) = quality {
            *quality = q;
        }

        // Max quality of 0 would mean no support for MS.
        if q == 0 {
            Error::NotAvailable
        } else {
            Error::Success
        }
    }

    /// Checks if a depth/stencil format can be used with a RT format.
    fn check_depth_stencil_match(
        &self,
        adapter: u32,
        ty: D3DDEVTYPE,
        _adapter_fmt: D3DFORMAT,
        _rt_fmt: D3DFORMAT,
        ds_fmt: D3DFORMAT,
    ) -> Error {
        self.check_adapter(adapter)?;
        self.check_devty(ty)?;

        // We don't check the adapter fmt / render target fmt since on modern GPUs
        // basically any valid combination of formats is allowed.

        // We only have to check that the format which was passed in
        // can be used with d/s buffers.
        if ds_fmt.is_depth_stencil_format() {
            Error::Success
        } else {
            Error::NotAvailable
        }
    }

    /// Checks if a conversion between two given formats is supported.
    fn check_device_format_conversion(
        &self,
        adapter: u32,
        ty: D3DDEVTYPE,
        _src_fmt: D3DFORMAT,
        _tgt_fmt: D3DFORMAT,
    ) -> Error {
        self.check_adapter(adapter)?;
        self.check_devty(ty)?;

        // For most types we can simply convert them to the right format on-the-fly.
        // TODO: we should at least validate the formats to make sure they are valid for back buffers.

        Error::Success
    }

    /// Returns a structure describing the features and limits of an adapter.
    fn get_device_caps(&self, adapter: u32, ty: D3DDEVTYPE, caps: *mut D3DCAPS9) -> Error {
        let adapter = self.check_adapter(adapter)?;
        self.check_devty(ty)?;
        let caps = check_mut_ref(caps)?;

        *caps = adapter.caps();

        Error::Success
    }

    /// Retrieves the monitor associated with an adapter.
    fn get_adapter_monitor(&self, adapter: u32) -> HMONITOR {
        self.check_adapter(adapter)
            .map(|adapter| adapter.monitor())
            .unwrap_or(ptr::null_mut())
    }

    /// Creates a logical device from an adapter.
    fn create_device(
        &mut self,
        adapter: u32,
        ty: D3DDEVTYPE,
        focus: HWND,
        flags: u32,
        pp: *mut D3DPRESENT_PARAMETERS,
        device: *mut *mut IDirect3DDevice9,
    ) -> Error {
        self.check_devty(ty)?;
        let ret = check_mut_ref(device)?;

        // TODO: support using multiple GPUs
        if flags & D3DCREATE_ADAPTERGROUP_DEVICE != 0 {
            warn!("Application requested the creation of a multi-GPU logical device");
        }

        // The device will need to hold a strong reference back to this interface.
        let parent = unsafe {
            let ptr = ComPtr::new(self as *mut _ as *mut IDirect3D9);
            ptr.AddRef();
            ptr
        };

        // Pass on the D3D11 device.
        // Since ID3D11Device is thread safe, it doesn't matter if the app
        // create multiple devices from the same adapter.
        let d3d11_device = self.check_adapter(adapter)?.device();

        // This struct stores the original device creation parameters.
        let cp = D3DDEVICE_CREATION_PARAMETERS {
            AdapterOrdinal: adapter,
            DeviceType: D3DDEVTYPE_HAL,
            hFocusWindow: focus,
            BehaviorFlags: flags,
        };

        // This structure describes some settings for the back buffer(s).
        // Since we don't support multiple adapters, we only use the first param in the array.
        let pp = check_mut_ref(pp)?;

        // Create the actual device.
        let device = crate::Device::new(parent, d3d11_device, cp, pp)?;

        // Now convert it to a raw pointer and return it.
        let ptr = Box::into_raw(Box::new(device));

        *ret = ptr as *mut IDirect3DDevice9;

        Error::Success
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    fn new_context() -> ComPtr<IDirect3D9> {
        let ptr = Box::into_raw(Box::new(Context::new()));
        ComPtr::new(ptr as *mut IDirect3D9)
    }

    #[test]
    fn context_lifetime() {
        let ctx = new_context();

        let original_count = unsafe { ctx.GetAdapterCount() };
        assert!(original_count > 0, "No GPUs found on the system.");

        let copy = ctx.clone();

        mem::drop(ctx);

        let count = unsafe { copy.GetAdapterCount() };

        assert_eq!(original_count, count);
    }
}
