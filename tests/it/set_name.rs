//! Tests for NiObjectNet::set_name.

use crate::common::load;
use std::io::Cursor;

use nifly::{file::NifFile, geometry::NiShape, nodes::NiNode, objects::NiObjectNet};

fn rename(nif: &mut NifFile, index: usize, name: impl AsRef<[u8]>) {
    nif.block_of_mut::<NiObjectNet>(index)
        .unwrap()
        .set_name(name);
}

#[test]
fn rename_node() {
    let mut nif = load("Bones");
    rename(&mut nif, 2, "Elbow");

    assert_eq!(nif.block_of::<NiNode>(2).unwrap().name(), "Elbow");
    // Name lookups see the new name.
    assert!(nif.node_transform_to_parent("Elbow").is_some());
    assert!(nif.node_transform_to_parent("Bone2").is_none());
    // Bone names come from the bone nodes, so the skin's bone list follows.
    let shape = nif.blocks_of::<NiShape>().next().unwrap();
    assert_eq!(
        nif.view_of(shape).unwrap().bone_list(),
        ["Bone1", "Elbow", "Bone3"]
    );
}

#[test]
fn rename_shape_and_reach_setter_by_upcast() {
    let mut nif = load("Bones");
    // Block 4 is the NiTriShape "Limb".
    let shape = nif.block_of_mut::<NiShape>(4).unwrap();
    shape.set_name("Arm");
    assert_eq!(nif.block_of::<NiShape>(4).unwrap().name(), "Arm");
    // Blocks without a name can't be reached as NiObjectNet.
    assert!(
        nif.block_of_mut::<NiObjectNet>(nif.blocks().count())
            .is_none()
    );
}

#[test]
fn rename_to_empty_and_non_utf8() {
    let mut nif = load("Bones");
    rename(&mut nif, 1, "");
    assert_eq!(nif.block_of::<NiNode>(1).unwrap().name(), "");
    rename(&mut nif, 1, b"Caf\xe9");
    assert_eq!(nif.block_of::<NiNode>(1).unwrap().name(), &b"Caf\xe9"[..]);
}

#[test]
fn renamed_node_survives_save() {
    let mut nif = load("Icosphere");
    let index = nif
        .nodes()
        .indexed()
        .find(|(_, n)| n.name() == "Indirect2")
        .unwrap()
        .0;
    rename(&mut nif, index, "Renamed");

    let mut cur = Cursor::new(Vec::new());
    nif.save(&mut cur, None).unwrap();
    let saved = NifFile::load(cur.into_inner().as_slice(), None).unwrap();

    let names: Vec<_> = saved.nodes().map(|n| n.name().to_string()).collect();
    assert_eq!(names, ["Icosphere", "Indirect1", "Renamed"]);
    assert_eq!(
        saved.node_transform_to_global("Renamed"),
        nif.node_transform_to_global("Renamed")
    );
}
