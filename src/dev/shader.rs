use std::slice;
use std::sync::atomic::{AtomicU32, Ordering};

use winapi::shared::{d3d9::*, d3d9types::*};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

use com_impl::{implementation, interface, ComInterface};
use comptr::ComPtr;

use crate::{core::*, Error, Result};

use super::Device;

/// Given a pointer to an array of tokens (forming up a shader),
/// returns a box containing the tokens.
fn tokens_to_box(tokens: *const u32) -> Box<[u32]> {
    let tokens = unsafe {
        // We don't know how long the shader will be.
        let mut len = 0;

        loop {
            len += 1;
            // We go until we find the end token.
            if *tokens.offset(len) == 0x0000_FFFF {
                break;
            }
        }

        std::slice::from_raw_parts(tokens, len as usize)
    };

    tokens.into()
}

macro_rules! impl_shader {
    ($name:ident, $iface:ident) => {
        #[implementation($iface)]
        impl $name {
            /// Retrieves the device which created this shader.
            fn get_device(&self, ret: *mut *mut Device) -> Error {
                let ret = check_mut_ref(ret)?;
                *ret = com_ref(self.device);
                Error::Success
            }

            /// Retrieve the shader's byte code.
            fn get_function(&self, ret: *mut u32, num: *mut u32) -> Error {
                if ret.is_null() {
                    let num = check_mut_ref(num)?;

                    *num = self.code.len() as u32;
                } else {
                    let code = unsafe {
                        let ret = check_mut_ref(ret)?;
                        slice::from_raw_parts_mut(ret, self.code.len())
                    };

                    code.copy_from_slice(&self.code);
                }
                Error::Success
            }
        }
    };
}

/// Vertex shader class.
#[interface(IDirect3DVertexShader9)]
pub struct VertexShader {
    refs: AtomicU32,
    device: *const Device,
    code: Box<[u32]>,
}

impl VertexShader {
    /// Create a new vertex shader.
    pub fn new(device: &Device, func: *const u32) -> Result<ComPtr<Self>> {
        let code = tokens_to_box(func);

        let vs = Self {
            __vtable: Box::new(Self::create_vtable()),
            refs: AtomicU32::new(1),
            device,
            code,
        };

        Ok(unsafe { new_com_interface(vs) })
    }
}

impl_iunknown!(struct VertexShader: IUnknown, IDirect3DVertexShader9);

impl_shader!(VertexShader, IDirect3DVertexShader9);

/// Pixel shader class.
#[interface(IDirect3DPixelShader9)]
pub struct PixelShader {
    refs: AtomicU32,
    device: *const Device,
    code: Box<[u32]>,
}

impl PixelShader {
    /// Create a new pixel shader.
    pub fn new(device: &Device, func: *const u32) -> Result<ComPtr<Self>> {
        let code = tokens_to_box(func);

        let ps = Self {
            __vtable: Box::new(Self::create_vtable()),
            refs: AtomicU32::new(1),
            device,
            code,
        };

        Ok(unsafe { new_com_interface(ps) })
    }
}

impl_iunknown!(struct PixelShader: IUnknown, IDirect3DPixelShader9);

impl_shader!(PixelShader, IDirect3DPixelShader9);

/// Declaration of a vertex shader's inputs.
#[interface(IDirect3DVertexDeclaration9)]
pub struct VertexDeclaration {
    refs: AtomicU32,
    device: *const Device,
    elems: Box<[D3DVERTEXELEMENT9]>,
}

impl VertexDeclaration {
    /// Creates a new vertex declaration.
    pub fn new(device: &Device, elems: *const D3DVERTEXELEMENT9) -> ComPtr<Self> {
        let elems = unsafe {
            let mut count = 0;
            let mut ptr = elems;

            fn is_end(ve: D3DVERTEXELEMENT9) -> bool {
                let end = D3DDECL_END;
                ve.Stream == end.Stream
                    && ve.Offset == end.Offset
                    && ve.Type == end.Type
                    && ve.Method == end.Method
                    && ve.Usage == end.Usage
                    && ve.UsageIndex == end.UsageIndex
            }

            // Input is a variable-length array terminated by END.
            while !is_end(*ptr) {
                ptr = ptr.offset(1);
                count += 1;

                // It's possible some apps forgot the terminator, in which case
                // we try to avoid looping forever.
                if count == 64 {
                    error!("Maximum vertex elements reached, but no terminator found.")
                }
            }

            let elems = slice::from_raw_parts(elems, count);

            Box::from(elems)
        };

        let vd = Self {
            __vtable: Box::new(Self::create_vtable()),
            refs: AtomicU32::new(1),
            device,
            elems,
        };

        unsafe { new_com_interface(vd) }
    }
}

impl_iunknown!(struct VertexDeclaration: IUnknown, IDirect3DVertexDeclaration9);

#[implementation(IDirect3DVertexDeclaration9)]
impl VertexDeclaration {
    /// Retrieves the device which owns this vertex declaration.
    fn get_device(&self, ret: *mut *mut Device) -> Error {
        let ret = check_mut_ref(ret)?;
        *ret = com_ref(self.device);
        Error::Success
    }

    /// Retrieves the elements which make up this declaration.
    fn get_declaration(&self, elems: *mut D3DVERTEXELEMENT9, num: *mut u32) -> Error {
        if elems.is_null() {
            let num = check_mut_ref(num)?;

            *num = self.elems.len() as u32;
        } else {
            let elems = unsafe {
                let elems = check_mut_ref(elems)?;
                slice::from_raw_parts_mut(elems, self.elems.len())
            };

            elems.copy_from_slice(&self.elems);
        }

        Error::Success
    }
}
