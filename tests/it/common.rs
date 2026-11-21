//! Helpers shared by the test modules.

use std::path::Path;

use nifly::{
    file::{Index, NifFile, SaveOptions},
    geometry::NiShape,
    objects::NiType,
};

/// nifly's own test files.
pub const UPSTREAM: &str = "nifly-sys/vendor/nifly/tests/input";

/// Saves without optimizing or sorting, so block indices stay.
pub const IN_PLACE: SaveOptions = SaveOptions {
    optimize: false,
    sort_blocks: false,
};

/// Loads the file at `path`.
pub fn open(path: impl AsRef<Path>) -> NifFile {
    NifFile::open(path, None).unwrap()
}

/// Loads `assets/{name}.nif`.
pub fn load(name: &str) -> NifFile {
    open(format!("assets/{name}.nif"))
}

/// Loads `{UPSTREAM}/{name}.nif`.
pub fn upstream(name: &str) -> NifFile {
    open(format!("{UPSTREAM}/{name}.nif"))
}

/// Saves with the default options.
pub fn save(nif: &mut NifFile) -> Vec<u8> {
    nif.to_bytes(None).unwrap()
}

/// Saves without optimizing or sorting, so block indices stay.
pub fn save_in_place(nif: &mut NifFile) -> Vec<u8> {
    nif.to_bytes(Some(IN_PLACE)).unwrap()
}

/// Saves with `options` and loads the result.
pub fn reload_with(nif: &mut NifFile, options: Option<SaveOptions>) -> NifFile {
    NifFile::load(nif.to_bytes(options).unwrap().as_slice(), None).unwrap()
}

/// Saves with the default options and loads the result.
pub fn reload(nif: &mut NifFile) -> NifFile {
    reload_with(nif, None)
}

/// Saves without optimizing or sorting, so block indices stay, and loads the result.
pub fn reload_in_place(nif: &mut NifFile) -> NifFile {
    reload_with(nif, Some(IN_PLACE))
}

/// The first block of type `T`.
pub fn first<T: NiType>(nif: &NifFile) -> &T {
    nif.blocks_of::<T>().next().unwrap()
}

/// The index of the first block of type `T`.
pub fn first_index<T: NiType>(nif: &NifFile) -> Index {
    nif.blocks_of::<T>().indices().next().unwrap()
}

/// The shape named `name`.
pub fn shape<'a>(nif: &'a NifFile, name: &str) -> &'a NiShape {
    nif.shapes().find(|s| s.name() == name).unwrap()
}

/// The index of the shape named `name`.
pub fn shape_index(nif: &NifFile, name: &str) -> Index {
    nif.shapes()
        .indexed()
        .find(|(_, s)| s.name() == name)
        .unwrap()
        .0
}

/// Asserts that `a.abs_diff_eq(&b, epsilon)`, for the `object3d` types.
macro_rules! assert_close {
    ($a:expr, $b:expr, $epsilon:expr $(,)?) => {{
        let (a, b) = (&$a, &$b);
        assert!(a.abs_diff_eq(b, $epsilon), "{a:?} != {b:?}");
    }};
}
pub(crate) use assert_close;
