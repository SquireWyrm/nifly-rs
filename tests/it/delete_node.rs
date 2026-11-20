//! Tests for NifFile::delete_node, NifFile::can_delete_node and NiNode::can_delete.

use crate::common::load;
use std::io::Cursor;

use nifly::{file::NifFile, geometry::NiShape, nodes::NiNode, objects::NiObject};

fn block_names(nif: &NifFile) -> Vec<String> {
    nif.blocks().map(|b| b.block_name().to_owned()).collect()
}

fn indices<'a>(nif: &NifFile, objs: impl Iterator<Item = &'a NiObject>) -> Vec<usize> {
    objs.map(|o| nif.block_index(o).unwrap()).collect()
}

fn node<'a>(nif: &'a NifFile, name: &str) -> &'a NiNode {
    nif.nodes().find(|n| n.name() == name).unwrap()
}

#[test]
fn can_delete_only_nodes_without_refs() {
    let bones = load("Bones");
    assert!(!bones.can_delete_node("Scene Root"), "has children");
    for bone in ["Bone1", "Bone2", "Bone3"] {
        assert!(bones.can_delete_node(bone), "{bone}");
        assert!(node(&bones, bone).can_delete(), "{bone}");
    }

    let ico = load("Icosphere");
    assert!(!ico.can_delete_node("Indirect1"));
    assert!(
        !ico.can_delete_node("Indirect2"),
        "has the shape as a child"
    );

    // Suzanne's root has extra data and collision as well as children.
    let suzanne = load("Suzanne");
    assert!(!suzanne.root_node().unwrap().can_delete());
}

#[test]
fn can_delete_missing_name_is_false() {
    let nif = load("Bones");
    assert!(!nif.can_delete_node("missing"));
    // The shape is named "Limb", but only nodes are searched.
    assert!(!nif.can_delete_node("Limb"));
}

#[test]
fn delete_missing_name_changes_nothing() {
    let mut nif = load("Bones");
    let before = block_names(&nif);
    assert!(!nif.delete_node("missing"));
    assert!(!nif.delete_node("Limb"), "shapes aren't nodes");
    assert_eq!(block_names(&nif), before);
}

#[test]
fn delete_leaf_node() {
    // Cone's block 1 is an unreferenced node with an empty name.
    let mut nif = load("Cone");
    assert!(nif.can_delete_node(""));
    assert!(nif.delete_node(""));

    assert_eq!(
        block_names(&nif),
        ["NiNode", "BSLightingShaderProperty", "BSTriShape"]
    );
    // The root's child ref moved from index 3 to 2 along with the shape.
    let root = nif.root_node().unwrap();
    assert_eq!(indices(&nif, nif.child_blocks(&root)), [2]);
    assert_eq!(root.children().next().unwrap().name(), "Cone");
}

#[test]
fn delete_clears_parent_ref_and_shifts_indices() {
    let mut nif = load("Bones");
    assert!(nif.delete_node("Bone2"));

    assert_eq!(nif.blocks().count(), 10);
    let root = nif.root_node().unwrap();
    let children: Vec<_> = root.children().map(|c| c.name().to_string()).collect();
    assert_eq!(children, ["Bone1", "Bone3", "Limb"]);
    // Bone3 and everything after it moved down one index.
    assert_eq!(indices(&nif, nif.child_blocks(&root)), [1, 2, 3]);
    assert_eq!(nif.block_of::<NiNode>(2).unwrap().name(), "Bone3");
}

#[test]
fn delete_node_with_children_orphans_them() {
    let mut nif = load("Icosphere");
    assert!(nif.delete_node("Indirect1"));

    // Indirect2 and the shape are still in the file, but no longer in the tree.
    assert_eq!(block_names(&nif), ["BSFadeNode", "NiNode", "BSTriShape"]);
    assert_eq!(indices(&nif, nif.tree()), [0]);

    let indirect2 = node(&nif, "Indirect2");
    assert!(nif.view_of(indirect2).and_then(|v| v.parent()).is_none());
    assert_eq!(nif.view_of(indirect2).unwrap().children().count(), 1);

    // Saving with optimize removes the orphans.
    let mut cur = Cursor::new(Vec::new());
    nif.save(&mut cur, None).unwrap();
    let saved = NifFile::load(cur.into_inner().as_slice(), None).unwrap();
    assert_eq!(block_names(&saved), ["BSFadeNode"]);
}

#[test]
fn deleting_a_bone_leaves_skin_data_out_of_sync() {
    // Documents upstream behavior: DeleteNode removes the bone from the skin instance, but
    // NiSkinData keeps all three bones' weights, so later bones pick up the wrong weights.
    let mut nif = load("Bones");
    let shape = nif.blocks_of::<NiShape>().next().unwrap();
    let bone2_weights = nif.view_of(shape).unwrap().bone_weights(1);
    assert_eq!(bone2_weights.len(), 115);

    assert!(nif.delete_node("Bone2"));

    let shape = nif.blocks_of::<NiShape>().next().unwrap();
    assert_eq!(nif.view_of(shape).unwrap().bone_list(), ["Bone1", "Bone3"]);
    assert_eq!(nif.view_of(shape).unwrap().bone_ids(), [1, 2]);
    // Bone index 1 is now Bone3, but its weights are still Bone2's.
    assert_eq!(nif.view_of(shape).unwrap().bone_weights(1), bone2_weights);
}

#[test]
fn delete_then_add_reuses_nothing() {
    // New nodes are appended after the shifted blocks.
    let mut nif = load("Cone");
    nif.delete_node("");
    let identity = nif.root_node().unwrap().transform();
    let index = nif.add_node("New", identity, None).unwrap();
    assert_eq!(index, 3);
    assert_eq!(
        block_names(&nif),
        ["NiNode", "BSLightingShaderProperty", "BSTriShape", "NiNode"]
    );
}
