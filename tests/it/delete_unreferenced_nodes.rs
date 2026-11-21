//! Tests for NifFile::delete_unreferenced_nodes.

use crate::common::{load, save_in_place};

use nifly::{
    extra_data::ExtraInteger,
    file::{NifFile, NifVersion},
    nodes::NiNode,
    object3d::Transform,
};

fn node_names(nif: &NifFile) -> Vec<String> {
    nif.nodes().map(|n| n.name().to_string()).collect()
}

#[test]
fn deletes_loose_node() {
    // Cone: 0 root, 1 unreferenced NiNode "", 2 unreferenced shader, 3 shape.
    let mut nif = load("Cone");
    assert_eq!(nif.delete_unreferenced_nodes(), Some(1));
    let types: Vec<_> = nif.blocks().map(|b| b.block_name()).collect();
    // Only nodes are deleted; the loose shader stays.
    assert_eq!(types, ["NiNode", "BSLightingShaderProperty", "BSTriShape"]);
}

#[test]
fn deletes_empty_child_nodes_too() {
    let mut nif = load("Suzanne");
    nif.add_node("Empty", Transform::IDENTITY, None).unwrap();
    assert_eq!(nif.delete_unreferenced_nodes(), Some(1));
    assert_eq!(node_names(&nif), ["Suzanne"]);
    // The root's child list no longer resolves to the deleted node.
    let root = nif.root_node().unwrap();
    let children: Vec<_> = root.children().map(|c| c.name().to_string()).collect();
    assert_eq!(children, ["Suzanne:6"]);
}

#[test]
fn deletes_empty_branches() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let a = nif.add_node("A", Transform::IDENTITY, None).unwrap();
    let b = nif.add_node("B", Transform::IDENTITY, Some(a)).unwrap();
    nif.add_node("C", Transform::IDENTITY, Some(b)).unwrap();
    // C is deleted first, which empties B, then A.
    assert_eq!(nif.delete_unreferenced_nodes(), Some(3));
    assert_eq!(node_names(&nif), ["Scene Root"]);
}

#[test]
fn keeps_nodes_that_own_something() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let tagged = nif.add_node("Tagged", Transform::IDENTITY, None).unwrap();
    nif.edit::<nifly::objects::NiAvObject>(tagged)
        .and_then(|mut e| e.assign_extra_data(ExtraInteger("Id", 7)))
        .unwrap();
    nif.add_node("Empty", Transform::IDENTITY, None).unwrap();

    assert_eq!(nif.delete_unreferenced_nodes(), Some(1));
    assert_eq!(node_names(&nif), ["Scene Root", "Tagged"]);
}

#[test]
fn bones_survive() {
    // Bones are empty leaves, but the skin instance points to them.
    let mut nif = load("Bones");
    let before = save_in_place(&mut nif);
    assert_eq!(nif.delete_unreferenced_nodes(), Some(0));
    assert_eq!(save_in_place(&mut nif), before);
    assert!(nif.find_block_by_name::<NiNode>("Bone1").is_some());
}

#[test]
fn nodes_with_children_are_kept() {
    for asset in ["Cube", "Icosphere", "Suzanne"] {
        let mut nif = load(asset);
        let before = save_in_place(&mut nif);
        assert_eq!(nif.delete_unreferenced_nodes(), Some(0), "{asset}");
        assert_eq!(save_in_place(&mut nif), before, "{asset}");
    }
}

#[test]
fn root_is_kept() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    assert_eq!(nif.delete_unreferenced_nodes(), Some(0));
    assert_eq!(node_names(&nif), ["Scene Root"]);
}
