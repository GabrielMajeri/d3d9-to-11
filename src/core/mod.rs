//! Core function and type definitions.
//!
//! This module contains the fundamental building blocks on top of which the rest
//! of the library is built.

pub mod fmt;

pub mod msample;

pub mod str;

mod adapter;
pub use self::adapter::Adapter;

mod context;
pub use self::context::Context;

use comptr::ComPtr;

use crate::{Error, Result};

/// Checks to make sure a given parameter passed in as a pointer is not null.
pub fn check_not_null<T>(ptr: *mut T) -> Error {
    if ptr.is_null() {
        Error::InvalidCall
    } else {
        Error::Success
    }
}

/// Checks that a HRESULT returned successfully, otherwise returns an error.
#[must_use]
pub fn check_hresult(hr: i32, msg: &'static str) -> Error {
    if hr != 0 {
        let err = std::io::Error::from_raw_os_error(hr);
        error!("{}: {}", msg, err);
        crate::Error::DriverInternalError
    } else {
        crate::Error::Success
    }
}

/// Tries to convert pointer parameter into a reference.
pub fn check_ref<'a, T>(ptr: *const T) -> Result<&'a T> {
    unsafe { ptr.as_ref().ok_or(Error::InvalidCall) }
}

/// Tries to convert a pointer parameter into a mutable reference.
pub fn check_mut_ref<'a, T>(ptr: *mut T) -> Result<&'a mut T> {
    unsafe { ptr.as_mut().ok_or(Error::InvalidCall) }
}

/// Creates a new heap-allocated COM interface from a Rust structure.
///
/// Unsafe because there is no way of checking if `this` implements the desired interface.
pub unsafe fn new_com_interface<T, I>(this: T) -> ComPtr<I> {
    // Danger right here.
    ComPtr::new(Box::into_raw(Box::new(this)) as *mut _)
}
