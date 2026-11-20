//! The prelude alone covers a load, edit and save.

use std::{fs::File, io::Cursor};

use nifly::prelude::*;

#[test]
fn prelude_covers_load_edit_save() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/Cube.nif");
    let mut nif = NifFile::load(File::open(path).unwrap(), None).unwrap();
    assert!(nif.version().file >= NifVersion::OBLIVION.file);

    // Names decode through the unnamed `ByteSlice` import.
    let root: View<NiNode> = nif.root_node().unwrap();
    let name = root.name().to_str_lossy().into_owned();
    let index: Index = root.index();
    assert!(root.children().count() > 0);

    let moved = Transform::new(Vec3::new(1.0, 2.0, 3.0), Matrix3::IDENTITY, 2.0);
    let mut edit: Edit<NiNode> = nif.edit::<NiNode>(index).unwrap();
    edit.set_transform(moved);

    let mut out = Cursor::new(Vec::new());
    let options = SaveOptions {
        sort_blocks: false,
        ..Default::default()
    };
    nif.save(&mut out, Some(options)).unwrap();
    out.set_position(0);
    let nif = NifFile::load(out, Some(LoadOptions::default())).unwrap();

    let root = nif.block_of::<NiNode>(index).unwrap();
    assert_eq!(root.name().to_str_lossy(), name);
    assert_eq!(root.transform(), moved);
    let base: &NiAvObject = root;
    let _: &NiObjectNet = base;
    let _: &NiObject = nif.block(index).unwrap();
    assert!(nif.blocks_of::<NiShape>().count() > 0);
}

/// The root re-exports name the same items as their modules.
#[test]
fn root_reexports() {
    let nif: nifly::NifFile = nifly::file::NifFile::create(nifly::NifVersion::SKYRIM_SE);
    let result: nifly::NifResult<()> = Err(nifly::Error::Io(std::io::ErrorKind::Other.into()));
    assert!(matches!(result, Err(nifly::error::Error::Io(_))));
    assert_eq!(nif.version(), nifly::file::NifVersion::SKYRIM_SE);
}

/// `NiType` is in the prelude for generic code over block types.
#[allow(dead_code)]
fn count<T: NiType>(nif: &NifFile) -> usize {
    nif.blocks_of::<T>().count()
}
