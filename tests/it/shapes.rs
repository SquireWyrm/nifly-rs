//! Tests for NifFile::shapes and NifFile::shape_names.

use crate::common::{load};

use nifly::geometry::NiShape;

#[test]
fn same_as_blocks_of() {
    for asset in ["Bones", "Cone", "Cube", "Icosphere", "Suzanne"] {
        let nif = load(asset);
        let a: Vec<_> = nif.shapes().map(|s| s as *const NiShape).collect();
        let b: Vec<_> = nif.blocks_of::<NiShape>().map(|s| s as *const NiShape).collect();
        assert_eq!(a, b, "{asset}");
    }
}

#[test]
fn shape_names_follow_renames() {
    let mut nif = load("Bones");
    let i = nif.block_index(nif.shapes().next().unwrap()).unwrap();
    assert_eq!(nif.shape_names().collect::<Vec<_>>(), [b"Limb"]);
    nif.block_of_mut::<nifly::objects::NiObjectNet>(i).unwrap().set_name(b"Caf\xe9");
    assert_eq!(nif.shape_names().collect::<Vec<_>>(), [b"Caf\xe9"]);
}
