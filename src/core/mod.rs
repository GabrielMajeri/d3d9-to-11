//! Core function and type definitions.
//!
//! This module contains the fundamental building blocks on top of which the rest
//! of the library is built.

pub mod format;
pub mod str;

mod adapter;
pub(self) use self::adapter::Adapter;

mod context;
pub use self::context::Context;

use crate::{Error, Result};

/// Checks to make sure a given parameter passed in as a pointer is not null.
pub fn check_not_null<T>(ptr: *mut T) -> Error {
    if ptr.is_null() {
        Error::InvalidCall
    } else {
        Error::Success
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
