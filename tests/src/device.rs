use comptr::ComPtr;
use std::{mem, ptr, slice};
use winapi::shared::{d3d9::*, d3d9types::*, windef::*};

pub struct Device {
    device: ComPtr<IDirect3DDevice9>,
}

impl Device {
    /// Creates a new D3D9 device.
    pub fn new(ctx: &IDirect3D9, window: HWND) -> Device {
        let mut pp = D3DPRESENT_PARAMETERS {
            // Leave these all blank to automatically select them.
            BackBufferWidth: 0,
            BackBufferHeight: 0,
            BackBufferFormat: D3DFMT_UNKNOWN,
            BackBufferCount: 1,
            // No need to use multisampling in the tests.
            MultiSampleType: 0,
            MultiSampleQuality: 0,
            SwapEffect: D3DSWAPEFFECT_DISCARD,
            hDeviceWindow: window,
            Windowed: 1,
            // Have the library create the DS buffer for us.
            EnableAutoDepthStencil: 1,
            AutoDepthStencilFormat: D3DFMT_D24S8,
            Flags: 0,
            FullScreen_RefreshRateInHz: 0,
            // Enable VSync
            PresentationInterval: 1,
        };

        let mut device = ptr::null_mut();

        let result = unsafe {
            ctx.CreateDevice(
                D3DADAPTER_DEFAULT,
                D3DDEVTYPE_HAL,
                window,
                D3DCREATE_MULTITHREADED,
                &mut pp,
                &mut device,
            )
        };

        assert_eq!(result, 0, "Failed to create device");

        let device = ComPtr::new(device);

        Self { device }
    }

    pub fn present(&self) {
        unsafe {
            self.device.Present(
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            );
        }
    }

    /// Runs the device tests.
    pub fn run_tests(&mut self) {
        self.check_auto_rt_ds();
        self.fill_default_render_target();
    }

    fn get_render_target(&self, i: u32) -> Surface {
        let surface = unsafe {
            let mut ptr = ptr::null_mut();
            let result = self.device.GetRenderTarget(i, &mut ptr);
            assert_eq!(result, 0, "Failed to get default render target");
            ComPtr::new(ptr)
        };

        Surface { surface }
    }

    fn get_depth_stencil(&self) -> Surface {
        let surface = unsafe {
            let mut ptr = ptr::null_mut();
            let result = self.device.GetDepthStencilSurface(&mut ptr);
            assert_eq!(result, 0, "Failed to get depth buffer");
            ComPtr::new(ptr)
        };

        Surface { surface }
    }

    // Checks that the default render target / depth buffer is correctly constructed.
    fn check_auto_rt_ds(&self) {
        let rt = self.get_render_target(0);

        // Ensure that children objects have references to the parent device.
        let device = rt.device();
        assert_eq!(
            device.as_ref() as *const _,
            self.device.as_ref() as *const _
        );

        let rt_desc = rt.desc();
        assert_eq!(rt_desc.Usage, D3DUSAGE_RENDERTARGET);

        let ds = self.get_depth_stencil();

        let ds_desc = ds.desc();
        assert_eq!(ds_desc.Usage, D3DUSAGE_DEPTHSTENCIL);

        assert_eq!(rt_desc.Width, ds_desc.Width);
        assert_eq!(rt_desc.Height, ds_desc.Height);
    }

    // Creates a CPU-mappable texture, maps it, fills it with color manually,
    // then copies it onto the back buffer.
    pub fn fill_default_render_target(&self) {
        let rt = self.get_render_target(0);

        let rt_desc = rt.desc();

        let width = rt_desc.Width as usize;
        let height = rt_desc.Height as usize;

        let texture = Texture::new(
            &self.device,
            width,
            height,
            1,
            D3DUSAGE_DYNAMIC | D3DUSAGE_WRITEONLY,
            D3DFMT_A8R8G8B8,
            D3DPOOL_SYSTEMMEM,
        );

        let surface = texture.get_level(0);

        // Map the pixel data directly to memory.
        let (ptr, stride) = surface.map(D3DLOCK_DISCARD);

        let pixels = unsafe {
            let size = height as usize * stride;
            slice::from_raw_parts_mut(ptr as *mut u32, size)
        };

        // Fill out the raw texture data.
        for i in 0..height {
            for j in 0..width {
                let index = (i * stride) + j;

                let color = {
                    let r = ((i as f32 / height as f32) * 255.0) as u32;
                    let g = ((j as f32 / width as f32) * 255.0) as u32;
                    let b = 192;

                    b | (g << 8) | (r << 16)
                };

                pixels[index] = color;
            }
        }

        surface.unmap();

        unsafe {
            let dest = POINT { x: 0, y: 0 };

            self.device.UpdateSurface(
                surface.surface.as_mut(),
                ptr::null(),
                rt.surface.as_mut(),
                &dest,
            );
        }
    }
}

struct Surface {
    surface: ComPtr<IDirect3DSurface9>,
}

impl Surface {
    /// Retrieves the device which owns this surface.
    fn device(&self) -> ComPtr<IDirect3DDevice9> {
        unsafe {
            let mut ptr = ptr::null_mut();
            let result = self.surface.GetDevice(&mut ptr);
            assert_eq!(result, 0);
            ComPtr::new(ptr)
        }
    }

    /// Gets the description of a surface.
    fn desc(&self) -> D3DSURFACE_DESC {
        unsafe {
            let mut desc = mem::uninitialized();
            let result = self.surface.GetDesc(&mut desc);
            assert_eq!(result, 0, "Failed to get surface description");
            desc
        }
    }

    /// Maps the surface to CPU-accessible memory.
    /// Returns a pointer to the data and the data's stride.
    fn map<T>(&self, flags: u32) -> (*mut T, usize) {
        unsafe {
            let mut lr = mem::uninitialized();

            let result = self.surface.LockRect(&mut lr, ptr::null(), flags);
            assert_eq!(result, 0, "Failed to map surface");

            let ptr = lr.pBits as *mut T;
            let stride = lr.Pitch as usize / mem::size_of::<T>();

            (ptr, stride)
        }
    }

    fn unmap(&self) {
        unsafe {
            let result = self.surface.UnlockRect();
            assert_eq!(result, 0, "Failed to unmap surface");
        }
    }
}

struct Texture {
    texture: ComPtr<IDirect3DTexture9>,
}

impl Texture {
    fn new(
        device: &IDirect3DDevice9,
        width: usize,
        height: usize,
        lvls: u32,
        usage: u32,
        fmt: D3DFORMAT,
        pool: D3DPOOL,
    ) -> Self {
        let texture = unsafe {
            let mut ptr = ptr::null_mut();
            let result = device.CreateTexture(
                width as u32,
                height as u32,
                lvls,
                usage,
                fmt,
                pool,
                &mut ptr,
                ptr::null_mut(),
            );
            assert_eq!(result, 0, "Failed to create texture");
            ComPtr::new(ptr)
        };

        Self { texture }
    }

    /// Retrieves a mip map level of this texture.
    fn get_level(&self, lvl: u32) -> Surface {
        let surface = unsafe {
            let mut ptr = ptr::null_mut();
            let result = self.texture.GetSurfaceLevel(lvl, &mut ptr);
            assert_eq!(result, 0, "Failed to get texture mip map level");
            ComPtr::new(ptr)
        };

        Surface { surface }
    }
}
