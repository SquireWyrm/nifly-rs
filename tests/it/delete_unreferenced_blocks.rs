//! Tests for NifFile::delete_unreferenced_blocks.

use crate::common::{load, save_in_place};

use nifly::{
    file::{NifFile, NifVersion},
    geometry::NiShape,
    nodes::NiNode,
    objects::NiObject,
    shaders::NiShader,
};

fn types(nif: &NifFile) -> Vec<&str> {
    nif.blocks().map(|b| b.block_name()).collect()
}

// Cone: 0 root, 1 unreferenced NiNode, 2 unreferenced shader, 3 shape (the root's child).

#[test]
fn all_types() {
    let mut nif = load("Cone");
    assert_eq!(nif.delete_unreferenced_blocks::<NiObject>(), 2);
    assert_eq!(types(&nif), ["NiNode", "BSTriShape"]);
    // The shape's ref was remapped and still resolves.
    let root = nif.root_node().unwrap();
    assert_eq!(root.children().count(), 1);
}

#[test]
fn only_the_given_type() {
    let mut nif = load("Cone");
    assert_eq!(nif.delete_unreferenced_blocks::<NiShader>(), 1);
    assert_eq!(types(&nif), ["NiNode", "NiNode", "BSTriShape"]);

    let mut nif = load("Cone");
    assert_eq!(nif.delete_unreferenced_blocks::<NiNode>(), 1);
    assert_eq!(
        types(&nif),
        ["NiNode", "BSLightingShaderProperty", "BSTriShape"]
    );

    // The only shape is referenced.
    let mut nif = load("Cone");
    assert_eq!(nif.delete_unreferenced_blocks::<NiShape>(), 0);
}

#[test]
fn cascades_within_the_type() {
    // Deleting Indirect1 leaves Indirect2, and the shape under it, without an owner.
    let mut nif = load("Icosphere");
    assert!(nif.delete_node("Indirect1"));
    assert_eq!(types(&nif), ["BSFadeNode", "NiNode", "BSTriShape"]);

    // With every type, the shape goes too once Indirect2 is gone.
    let mut all = load("Icosphere");
    all.delete_node("Indirect1");
    assert_eq!(all.delete_unreferenced_blocks::<NiObject>(), 2);
    assert_eq!(types(&all), ["BSFadeNode"]);

    // With nodes only, the shape is left behind, unowned.
    assert_eq!(nif.delete_unreferenced_blocks::<NiNode>(), 1);
    assert_eq!(types(&nif), ["BSFadeNode", "BSTriShape"]);
    assert_eq!(nif.block_ref_count(1, false), 0);
    assert_eq!(nif.delete_unreferenced_blocks::<NiShape>(), 1);
    assert_eq!(types(&nif), ["BSFadeNode"]);
}

#[test]
fn nothing_to_delete() {
    for asset in ["Bones", "Cube", "Icosphere", "Suzanne"] {
        let mut nif = load(asset);
        let before = save_in_place(&mut nif);
        assert_eq!(nif.delete_unreferenced_blocks::<NiObject>(), 0, "{asset}");
        assert_eq!(save_in_place(&mut nif), before, "{asset}");
    }
}

#[test]
fn root_is_kept() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    assert_eq!(nif.delete_unreferenced_blocks::<NiObject>(), 0);
    assert!(nif.root_node().is_some());
}

#[test]
fn same_blocks_as_optimize() {
    let mut a = load("Cone");
    a.delete_unreferenced_blocks::<NiObject>();
    let mut b = load("Cone");
    b.optimize();
    assert_eq!(types(&a), types(&b));
}
