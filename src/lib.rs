pub use bstr;

#[macro_use]
mod macros;

pub mod error;
pub mod extra_data;
pub mod handle;
pub mod iter;
pub mod object3d;
pub mod objects;

// The order of these sets the order of `NifFile`'s doc sections, one per module.
pub mod animation;
pub mod collision;
pub mod effects;
pub mod file;
pub mod geometry;
pub mod nodes;
pub mod particles;
pub mod shaders;
pub mod skin;
