//! Error type definition.

use std::{ops, result};

pub type Result<T> = result::Result<T, Error>;

const D3D9_FACILITY: u32 = 0x876;

const fn make_hresult(sev: u32, fac: u32, code: u32) -> u32 {
    (sev << 31) | (fac << 16) | code
}

const fn make_result(code: u32) -> u32 {
    make_hresult(1, D3D9_FACILITY, code)
}

const fn make_status(code: u32) -> u32 {
    make_hresult(0, D3D9_FACILITY, code)
}

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum Error {
    Success = 0,
    NotFound = make_result(2150),
    MoreData = make_result(2151),
    NotAvailable = make_result(2154),
    InvalidCall = make_result(2156),

    // Device errors
    InvalidDevice = make_result(2155),
    DeviceHung = make_result(2164),
    DeviceLost = make_result(2152),
    DeviceNotReset = make_result(2153),
    DeviceRemoved = make_result(2160),

    // Driver errors
    DriverInvalidCall = make_result(2157),
    DriverInternalError = make_result(2087),

    // Resource exhaustion errors
    OutOfVideoMemory = make_result(380),
    TooManyOperations = make_result(2077),

    // Resource usage errors
    WasStillDrawing = make_result(540),
    WrongTextureFormat = make_result(2072),
    UnsupportedTextureFilter = make_result(2082),
    UnsupportedColorOperation = make_result(2073),
    UnsupportedColorArg = make_result(2074),
    UnsupportedAlphaOperation = make_result(2075),
    UnsupportedAlphaArg = make_result(2076),
    UnsupportedFactorValue = make_result(2079),
    UnsupportedOverlay = make_result(2171),
    UnsupportedOverlayFormat = make_result(2172),
    NoAutoGen = make_status(2159),

    // Conflicting state
    ConflictingTextureFilter = make_result(2078),
    ConflictingTexturePalette = make_result(2086),
    ConflictingRenderState = make_result(2081),

    // Protected content
    CanNotProtectContent = make_result(2173),
    UnsupportedCrypto = make_result(2174),

    // Misc errors
    NotResident = make_status(2165),
    ResidentInSharedMemory = make_status(2166),
    PresentModeChanged = make_status(2167),
    PresentOccluded = make_status(2168),
    PresentStatisticsDisjoint = make_status(2180),
}

impl ops::Try for Error {
    type Ok = ();
    type Error = Error;

    fn into_result(self) -> Result<()> {
        match self {
            Error::Success => Ok(()),
            _ => Err(self),
        }
    }

    fn from_ok(_: ()) -> Self {
        Error::Success
    }

    fn from_error(err: Self) -> Self {
        err
    }
}
