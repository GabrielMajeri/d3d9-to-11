//! Module containing texture-related interfaces.
//!
//! This means 2D textures, 3D (volume) textures, or cube maps.

mod base;
pub use self::base::BaseTexture;

mod texture2d;
pub use self::texture2d::Texture;

mod cube;
pub use self::cube::CubeTexture;
