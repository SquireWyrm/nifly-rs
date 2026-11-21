//! Tests for NifFile::add_node and NifFile::block_index.

use std::ptr;

use nifly::{
    file::NifFile,
    nodes::NiNode,
    object3d::{Transform, vec3},
};

use crate::common::{open, reload};

const EPS: f32 = 1e-4;

fn cube() -> NifFile {
    open("assets/Cube.nif")
}

fn node(nif: &NifFile, index: usize) -> &NiNode {
    nif.block_of::<NiNode>(index).unwrap()
}

fn child_names(nif: &NifFile, parent: &NiNode) -> Vec<String> {
    nif.view_of(parent)
        .unwrap()
        .children()
        .map(|c| c.name().to_string())
        .collect()
}

#[test]
fn block_index_is_inverse_of_block() {
    let nif = cube();
    for (i, b) in nif.blocks().enumerate() {
        assert_eq!(nif.block_index(b), Some(i));
    }

    let other = cube();
    assert_eq!(nif.block_index(other.root_node().unwrap().get()), None);
}

#[test]
fn add_under_root() {
    let mut nif = cube();
    let before = nif.blocks().count();

    let index = nif
        .add_node(
            "Added",
            Transform::from_translation(vec3(1.0, 2.0, 3.0)),
            None,
        )
        .unwrap();
    assert_eq!(index, before, "appended to the block list");

    let added = node(&nif, index);
    assert_eq!(added.block_name(), "NiNode");
    assert_eq!(added.name(), "Added");
    assert_eq!(
        added.transform(),
        Transform::from_translation(vec3(1.0, 2.0, 3.0))
    );
    assert_eq!(added.child_count(), 0);

    let root = nif.root_node().unwrap();
    assert_eq!(child_names(&nif, root.get()), ["Cube", "Added"]);
    assert_eq!(
        nif.view_of(added).and_then(|v| v.parent()).unwrap().index(),
        root.index()
    );
}

#[test]
fn existing_indices_stay_valid() {
    let mut nif = cube();
    let names: Vec<_> = nif.blocks().map(|b| b.block_name().to_owned()).collect();

    nif.add_node(
        "Added",
        Transform::from_translation(vec3(0.0, 0.0, 0.0)),
        None,
    )
    .unwrap();

    let after: Vec<_> = nif.blocks().map(|b| b.block_name().to_owned()).collect();
    assert_eq!(after[..names.len()], names);
}

#[test]
fn add_nested_and_compose_transforms() {
    let mut nif = cube();
    let arm = nif
        .add_node(
            "Arm",
            Transform::from_translation(vec3(1.0, 0.0, 0.0)),
            None,
        )
        .unwrap();
    let hand = nif
        .add_node(
            "Hand",
            Transform::from_translation(vec3(0.0, 2.0, 0.0)),
            Some(arm),
        )
        .unwrap();

    let arm_node = node(&nif, arm);
    let hand_node = node(&nif, hand);
    assert_eq!(child_names(&nif, arm_node), ["Hand"]);
    assert!(ptr::eq(
        nif.view_of(hand_node)
            .and_then(|v| v.parent())
            .unwrap()
            .get(),
        arm_node
    ));

    let global = nif.node_transform_to_global("Hand").unwrap();
    let t = global.translation;
    assert!(
        (t.x - 1.0).abs() < EPS && (t.y - 2.0).abs() < EPS && t.z.abs() < EPS,
        "{t:?}"
    );
}

#[test]
fn parent_by_index_from_reference() {
    // The usual pattern: get a node, turn it into an index, then mutate.
    let mut nif = cube();
    let root = nif.root_index().unwrap();
    let index = nif
        .add_node(
            "Child",
            Transform::from_translation(vec3(0.0, 0.0, 0.0)),
            Some(root),
        )
        .unwrap();
    assert_eq!(nif.block_index(node(&nif, index)), Some(index));
}

#[test]
fn invalid_parent_changes_nothing() {
    let mut nif = cube();
    let count = nif.blocks().count();

    // Block 1 is the BSTriShape, not a node.
    assert_eq!(nif.block(1).unwrap().block_name(), "BSTriShape");
    assert_eq!(
        nif.add_node(
            "Bad",
            Transform::from_translation(vec3(0.0, 0.0, 0.0)),
            Some(1)
        ),
        None
    );
    // Out of range, including values that don't fit nifly's u32 indices.
    assert_eq!(
        nif.add_node(
            "Bad",
            Transform::from_translation(vec3(0.0, 0.0, 0.0)),
            Some(count)
        ),
        None
    );
    assert_eq!(
        nif.add_node(
            "Bad",
            Transform::from_translation(vec3(0.0, 0.0, 0.0)),
            Some(u32::MAX as usize)
        ),
        None
    );
    assert_eq!(
        nif.add_node(
            "Bad",
            Transform::from_translation(vec3(0.0, 0.0, 0.0)),
            Some(usize::MAX)
        ),
        None
    );

    assert_eq!(nif.blocks().count(), count);
}

#[test]
fn non_utf8_name() {
    let mut nif = cube();
    let index = nif
        .add_node(
            b"Caf\xe9",
            Transform::from_translation(vec3(0.0, 0.0, 0.0)),
            None,
        )
        .unwrap();
    assert_eq!(node(&nif, index).name(), b"Caf\xe9");
}

#[test]
fn added_nodes_survive_save() {
    let mut nif = cube();
    let arm = nif
        .add_node(
            "Arm",
            Transform::from_translation(vec3(1.0, 0.0, 0.0)),
            None,
        )
        .unwrap();
    nif.add_node(
        "Hand",
        Transform::from_translation(vec3(0.0, 2.0, 0.0)),
        Some(arm),
    )
    .unwrap();

    let saved = reload(&mut nif);

    // Referenced from the root, so optimizing keeps them.
    let names: Vec<_> = saved.nodes().map(|n| n.name().to_string()).collect();
    assert_eq!(names, ["Scene Root", "Arm", "Hand"]);
    assert_eq!(
        saved.node_transform_to_parent("Hand"),
        Some(Transform::from_translation(vec3(0.0, 2.0, 0.0)))
    );
}

#[test]
fn add_child_from_handles() {
    use nifly::nodes::NiSwitchNode;

    let mut nif = cube();
    let before = nif.blocks().count();
    let arm = nif
        .root_node_mut()
        .unwrap()
        .add_child("Arm", Transform::from_translation(vec3(1.0, 2.0, 3.0)));
    assert_eq!(arm, before, "appended to the block list");
    nif.edit::<NiNode>(arm)
        .unwrap()
        .add_child_of::<NiSwitchNode, _>(
            "Switch",
            Transform::from_translation(vec3(0.0, 1.0, 0.0)),
        );

    let root = nif.root_node().unwrap();
    assert_eq!(
        child_names(&nif, root.get()),
        ["Cube", "Arm"],
        "the last child"
    );

    let saved = reload(&mut nif);
    let root = saved.root_node().unwrap();
    let arm = root
        .children_of::<NiNode, _>()
        .views()
        .find(|c| c.name() == "Arm")
        .unwrap();
    assert_eq!(
        arm.transform(),
        Transform::from_translation(vec3(1.0, 2.0, 3.0))
    );
    let switch_node = arm.children_of::<NiSwitchNode, _>().next().unwrap();
    assert_eq!(
        (switch_node.name(), switch_node.transform()),
        (
            "Switch".into(),
            Transform::from_translation(vec3(0.0, 1.0, 0.0))
        )
    );
}
