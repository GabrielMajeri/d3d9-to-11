use std::sync::atomic::{AtomicU32, Ordering};

use winapi::shared::{d3d9::*, d3d9types::*};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

use com_impl::{implementation, interface, ComInterface};
use comptr::ComPtr;

use crate::core::*;
use crate::d3d11;
use crate::Error;

use super::{Device, Resource};

/// Buffer holding vertex data.
#[interface(IDirect3DVertexBuffer9)]
pub struct VertexBuffer {
    resource: Resource,
    refs: AtomicU32,
    fvf: u32,
    buffer: d3d11::Buffer,
    usage: u32,
}

impl VertexBuffer {
    /// Creates a new vertex buffer.
    pub fn new(
        device: &Device,
        pool: D3DPOOL,
        fvf: u32,
        buffer: d3d11::Buffer,
        usage: u32,
    ) -> ComPtr<Self> {
        let vb = Self {
            __vtable: Box::new(Self::create_vtable()),
            resource: Resource::new(device, pool, D3DRTYPE_VERTEXBUFFER),
            refs: AtomicU32::new(1),
            fvf,
            buffer,
            usage,
        };

        unsafe { new_com_interface(vb) }
    }
}

impl_iunknown!(struct VertexBuffer: IUnknown, IDirect3DResource9, IDirect3DVertexBuffer9);

impl ComInterface<IDirect3DResource9Vtbl> for VertexBuffer {
    fn create_vtable() -> IDirect3DResource9Vtbl {
        let mut vtbl: IDirect3DResource9Vtbl = Resource::create_vtable();
        vtbl.parent = Self::create_vtable();
        vtbl
    }
}

#[implementation(IDirect3DVertexBuffer9)]
impl VertexBuffer {
    fn get_desc(&self, ret: *mut D3DVERTEXBUFFER_DESC) -> Error {
        let ret = check_mut_ref(ret)?;

        let desc = self.buffer.desc();

        ret.Type = D3DRTYPE_VERTEXBUFFER;
        ret.Size = desc.ByteWidth;
        ret.Format = D3DFMT_R32F;
        ret.FVF = self.fvf;
        ret.Pool = self.resource.pool();
        ret.Usage = self.usage;

        Error::Success
    }

    fn lock(&self, offset: u32, _size: u32, ret: *mut *mut u8, flags: u32) -> Error {
        let ret = check_mut_ref(ret)?;

        let resource = self.buffer.as_resource();
        let ctx = self.resource.device_context();
        let mapped = ctx.map(resource, 0, flags, self.usage)?;

        // TODO: allow buffers to be mapped multiple times.
        info!("Mapped vertex buffer");
        *ret = unsafe {
            let addr = mapped.pBits as *mut u8;
            addr.offset(offset as isize)
        };

        Error::Success
    }

    fn unlock(&self) -> Error {
        let resource = self.buffer.as_resource();
        let ctx = self.resource.device_context();
        ctx.unmap(resource, 0);
        Error::Success
    }
}

/// Buffer holding vertex indices.
#[interface(IDirect3DIndexBuffer9)]
pub struct IndexBuffer {
    resource: Resource,
    refs: AtomicU32,
    fmt: D3DFORMAT,
    buffer: d3d11::Buffer,
    usage: u32,
}

impl IndexBuffer {
    /// Creates a new index buffer.
    pub fn new(
        device: &Device,
        fmt: D3DFORMAT,
        pool: D3DPOOL,
        buffer: d3d11::Buffer,
        usage: u32,
    ) -> ComPtr<Self> {
        let vb = Self {
            __vtable: Box::new(Self::create_vtable()),
            resource: Resource::new(device, pool, D3DRTYPE_INDEXBUFFER),
            refs: AtomicU32::new(1),
            fmt,
            buffer,
            usage,
        };

        unsafe { new_com_interface(vb) }
    }
}

impl_iunknown!(struct IndexBuffer: IUnknown, IDirect3DResource9, IDirect3DIndexBuffer9);

impl ComInterface<IDirect3DResource9Vtbl> for IndexBuffer {
    fn create_vtable() -> IDirect3DResource9Vtbl {
        let mut vtbl: IDirect3DResource9Vtbl = Resource::create_vtable();
        vtbl.parent = Self::create_vtable();
        vtbl
    }
}

#[implementation(IDirect3DIndexBuffer9)]
impl IndexBuffer {
    fn get_desc(&self, ret: *mut D3DINDEXBUFFER_DESC) -> Error {
        let ret = check_mut_ref(ret)?;

        let desc = self.buffer.desc();

        ret.Type = D3DRTYPE_INDEXBUFFER;
        ret.Size = desc.ByteWidth;
        ret.Format = self.fmt;
        ret.Pool = self.resource.pool();
        ret.Usage = self.usage;

        Error::Success
    }

    fn lock(&self, offset: u32, _size: u32, ret: *mut *mut u8, flags: u32) -> Error {
        let ret = check_mut_ref(ret)?;

        let resource = self.buffer.as_resource();
        let ctx = self.resource.device_context();
        let mapped = ctx.map(resource, 0, flags, self.usage)?;

        // TODO: allow buffers to be mapped multiple times.
        info!("Mapped index buffer");
        *ret = unsafe {
            let addr = mapped.pBits as *mut u8;
            addr.offset(offset as isize)
        };

        Error::Success
    }

    fn unlock(&self) -> Error {
        let resource = self.buffer.as_resource();
        let ctx = self.resource.device_context();
        ctx.unmap(resource, 0);
        Error::Success
    }
}
