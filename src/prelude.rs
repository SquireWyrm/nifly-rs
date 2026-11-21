//! Common types, for `use nifly::prelude::*`.
//!
//! Also brings [`ByteSlice`](bstr::ByteSlice)'s methods into scope, unnamed, for decoding names
//! with `to_str_lossy`.

pub use bstr::ByteSlice as _;

pub use crate::{
    file::{Index, LoadOptions, NifFile, NifVersion, SaveOptions},
    geometry::NiShape,
    handle::{Edit, View},
    nodes::NiNode,
    object3d::{Matrix3, Transform, Vec3, vec3},
    objects::{NiAvObject, NiObject, NiObjectNet, NiType},
};
