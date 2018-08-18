//! Type-safe enums and bitflags for D3D9.

use bitflags::*;

bitflags! {
    /// Resource usage flags.
    pub struct UsageFlags: u32 {
        /// Indicates this resource is dynamic, i.e. will be mapped and
        /// written to frequently.
        const DYNAMIC = 1 << 9;
        /// Resource will only be written to.
        const WRITE_ONLY = 1 << 3;

        /// Buffer will be used for drawing points.
        const POINTS = 1 << 6;
        /// Buffer will be used for drawing higher-order primitives.
        const RT_PATCHES = 1 << 7;
        /// Buffer will be used for drawing N-patches.
        const N_PATCHES = 1 << 8;

        /// Automatically generated and manage mip maps.
        const AUTO_GEN_MIP_MAP = 1 << 10;

        /// Resource is a render target.
        const RENDER_TARGET = 1;
        /// Resource is a depth / stencil buffer.
        const DEPTH_STENCIL = 2;
        /// Resource is a displacement map.
        const DISPLACEMENT_MAP = 1 << 14;

        // The values in this buffer do not require clipping.
        // No way to disable it in D3D11.
        // const DO_NOT_CLIP = 1 << 5;

        // Used to indicate software processing should be used.
        // We always use hardware acceleration.
        // const SOFTWARE_PROCESSING = 1 << 4;

        // We ignore the usage query flags.

        // We also ignore the D3D9Ex flags.
    }
}

bitflags! {
    /// Resource locking (mapping) flags.
    pub struct LockFlags: u32 {
        /// Resource will only be read from.
        const READ_ONLY = 1 << 4;
        /// The initialized parts of the resoruce will not be overwritten.
        const NO_OVERWRITE = 1 << 12;
        /// Discard the existing contents of the resource.
        const DISCARD = 1 << 13;
        /// Return immediately if the resource is in use.
        const DO_NOT_WAIT = 1 << 14;
        /// Do not update the dirty area with the locked rectangle.
        const NO_DIRTY_UPDATE = 1 << 15;

        // NO_SYS_LOCK: not applicable to us.
    }
}

/// A pool of memory from which the resource was allocated.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum MemoryPool {
    // Default resources are placed in VRAM.
    Default,
    // Managed resources are placed in VRAM if possible, and are backed by system RAM.
    Managed,
    // SystemMem resources are stored in RAM.
    // Because of this, they are not accessible in shaders.
    SystemMem,
    // Memory allocated from RAM, not accessible by Direct3D9.
    Scratch,
}

/// The kind of resource an interface represents.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum ResourceType {
    Surface = 1,
    Volume = 2,
    Texture = 3,
    VolumeTexture = 4,
    CubeTexture = 5,
    VertexBuffer = 6,
    IndexBuffer = 7,
}
