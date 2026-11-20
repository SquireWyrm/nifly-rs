//! Tests for NifFile::find_block_by_name.

use crate::common::load;

use nifly::{
    extra_data::{BsxFlags, NiExtraData},
    file::{NifFile, NifVersion},
    geometry::NiShape,
    nodes::{BsFadeNode, NiNode},
    object3d::Transform,
    objects::NiObject,
};

fn index(nif: &NifFile, obj: &NiObject) -> usize {
    nif.block_index(obj).unwrap()
}

#[test]
fn finds_each_kind_of_named_block() {
    let nif = load("Suzanne");

    let root = nif.find_block_by_name::<BsFadeNode>("Suzanne").unwrap();
    assert_eq!(index(&nif, root), 0);

    let shape = nif.find_block_by_name::<NiShape>("Suzanne:6").unwrap();
    assert_eq!(index(&nif, shape), 7);

    // Extra data isn't an NiObjectNET but has a name too.
    let bsx = nif.find_block_by_name::<BsxFlags>("BSX").unwrap();
    assert_eq!(index(&nif, bsx), 1);
    assert!(nif.find_block_by_name::<NiExtraData>("BSX").is_some());
}

#[test]
fn type_must_match() {
    let nif = load("Suzanne");
    // Right name, wrong type.
    assert!(nif.find_block_by_name::<NiShape>("Suzanne").is_none());
    assert!(nif.find_block_by_name::<NiNode>("Suzanne:6").is_none());
    assert!(nif.find_block_by_name::<NiNode>("BSX").is_none());
    // A base type matches subclasses.
    assert!(nif.find_block_by_name::<NiNode>("Suzanne").is_some());
}

#[test]
fn ni_object_searches_every_named_block() {
    let nif = load("Suzanne");
    for (name, i) in [("Suzanne", 0), ("BSX", 1), ("Suzanne:6", 7)] {
        let block = nif.find_block_by_name::<NiObject>(name).unwrap();
        assert_eq!(index(&nif, block), i, "{name}");
    }
}

#[test]
fn missing_name() {
    let nif = load("Icosphere");
    assert!(nif.find_block_by_name::<NiObject>("missing").is_none());
    // Names are matched exactly.
    assert!(nif.find_block_by_name::<NiNode>("indirect1").is_none());
    assert!(nif.find_block_by_name::<NiNode>("Indirect").is_none());
    assert!(nif.find_block_by_name::<NiNode>("Indirect1").is_some());
}

#[test]
fn first_in_block_order() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let identity = Transform::IDENTITY;
    let first = nif.add_node("Dup", identity, None).unwrap();
    let parent = nif.add_node("Parent", identity, None).unwrap();
    nif.add_node("Dup", identity, Some(parent)).unwrap();

    let found = nif.find_block_by_name::<NiNode>("Dup").unwrap();
    assert_eq!(index(&nif, found), first);
}

#[test]
fn agrees_with_bone_names() {
    let nif = load("Bones");
    let shape = nif.blocks_of::<NiShape>().next().unwrap();
    let ids = nif.view_of(shape).unwrap().bone_ids();
    for (name, id) in nif.view_of(shape).unwrap().bone_list().iter().zip(ids) {
        let bone = nif.find_block_by_name::<NiNode>(name).unwrap();
        assert_eq!(index(&nif, bone), id, "{name}");
    }
}
