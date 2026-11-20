//! Helpers shared by the test modules.

use std::{fs, io::Cursor};

use nifly::file::{NifFile, SaveOptions};

/// nifly's own test files.
pub const UPSTREAM: &str = "nifly-sys/vendor/nifly/tests/input";

/// Loads `assets/{name}.nif`.
pub fn load(name: &str) -> NifFile {
    NifFile::load(
        fs::read(format!("assets/{name}.nif")).unwrap().as_slice(),
        None,
    )
    .unwrap()
}

/// Loads `{UPSTREAM}/{name}.nif`.
pub fn upstream(name: &str) -> NifFile {
    NifFile::load(
        fs::read(format!("{UPSTREAM}/{name}.nif"))
            .unwrap()
            .as_slice(),
        None,
    )
    .unwrap()
}

/// Saves with the default options.
pub fn save(nif: &mut NifFile) -> Vec<u8> {
    let mut cur = Cursor::new(Vec::new());
    nif.save(&mut cur, None).unwrap();
    cur.into_inner()
}

/// Saves with the default options and loads the result.
pub fn reload(nif: &mut NifFile) -> NifFile {
    NifFile::load(save(nif).as_slice(), None).unwrap()
}

/// Saves without optimizing or sorting, so block indices stay, and loads the result.
pub fn reload_in_place(nif: &mut NifFile) -> NifFile {
    let mut cur = Cursor::new(Vec::new());
    nif.save(
        &mut cur,
        Some(SaveOptions {
            optimize: false,
            sort_blocks: false,
        }),
    )
    .unwrap();
    NifFile::load(cur.into_inner().as_slice(), None).unwrap()
}
