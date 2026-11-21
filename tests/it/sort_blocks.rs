//! Tests for NifFile::pretty_sort_blocks, NifFile::sort_blocks and BlockSorter.

use crate::common::{load, reload_in_place, save_in_place};
use std::io::Cursor;

use nifly::file::{NifFile, SaveOptions};

fn types(nif: &NifFile) -> Vec<&str> {
    nif.blocks().map(|b| b.block_name()).collect()
}

#[test]
fn pretty_sort_matches_save() {
    let mut a = load("Cone");
    a.pretty_sort_blocks();
    assert_eq!(
        types(&a),
        ["NiNode", "BSTriShape", "NiNode", "BSLightingShaderProperty"]
    );

    let mut b = load("Cone");
    let mut cur = Cursor::new(Vec::new());
    b.save(
        &mut cur,
        Some(SaveOptions {
            optimize: false,
            sort_blocks: true,
        }),
    )
    .unwrap();
    assert_eq!(save_in_place(&mut a), cur.into_inner());
}

#[test]
fn visiting_parentless_nodes_is_pretty_sort() {
    for asset in ["Cone", "Icosphere", "Suzanne", "Bones"] {
        let mut a = load(asset);
        let roots: Vec<_> = a
            .nodes()
            .views()
            .filter(|n| n.parent().is_none())
            .map(|n| n.index())
            .collect();
        let mut sorter = a.sort_blocks().unwrap();
        for root in roots {
            assert!(sorter.visit(root));
        }
        sorter.apply();

        let mut b = load(asset);
        b.pretty_sort_blocks();
        assert_eq!(save_in_place(&mut a), save_in_place(&mut b), "{asset}");
    }
}

#[test]
fn visit_order_decides_block_order() {
    // Suzanne: 0 root, 1 BSXFlags, 2-6 collision, 7 shape, 8 shader, 9 texture set.
    let mut nif = load("Suzanne");
    let mut sorter = nif.sort_blocks().unwrap();
    assert!(sorter.visit(7)); // the shape's subtree first
    assert!(sorter.visit(0));
    sorter.apply();

    assert_eq!(
        types(&nif),
        [
            "BSTriShape",
            "BSLightingShaderProperty",
            "BSShaderTextureSet",
            "BSFadeNode",
            "BSXFlags",
            // Collision blocks are placed dependencies first.
            "bhkCompressedMeshShapeData",
            "bhkCompressedMeshShape",
            "bhkMoppBvTreeShape",
            "bhkRigidBody",
            "bhkCollisionObject",
        ]
    );
    // Refs were remapped: the shape is still the root's child and keeps its shader.
    let root = nif
        .find_block_by_name::<nifly::nodes::NiNode>("Suzanne")
        .unwrap();
    let child = nif.view_of(root).unwrap().children().next().unwrap();
    assert_eq!(child.name(), "Suzanne:6");
    assert_eq!(nif.block_index(child), Some(0));
}

#[test]
fn unvisited_blocks_keep_their_order_after_visited_ones() {
    // Cone: 0 root, 1 unreferenced NiNode, 2 unreferenced shader, 3 shape (the root's child).
    let mut nif = load("Cone");
    let mut sorter = nif.sort_blocks().unwrap();
    assert!(sorter.visit(3));
    sorter.apply();
    assert_eq!(
        types(&nif),
        ["BSTriShape", "NiNode", "NiNode", "BSLightingShaderProperty"]
    );
    // The real root is now block 1, but it's still the shape's parent.
    let shape = nif.shapes().next().unwrap();
    let parent = nif.view_of(shape).and_then(|v| v.parent()).unwrap();
    assert_eq!(parent.index(), 1);
}

#[test]
fn visit_and_is_visited() {
    let mut nif = load("Suzanne");
    let mut sorter = nif.sort_blocks().unwrap();
    assert!(!sorter.is_visited(7));

    assert!(sorter.visit(7));
    // The shape's shader and texture set come with it.
    for i in [7, 8, 9] {
        assert!(sorter.is_visited(i), "block {i}");
    }
    assert!(!sorter.is_visited(0));

    assert!(!sorter.visit(7), "already visited");
    assert!(!sorter.visit(8), "visited as part of the shape");
    assert!(!sorter.visit(10), "out of range");
    assert!(!sorter.visit(usize::MAX));
    assert!(!sorter.is_visited(usize::MAX));
}

#[test]
fn dropping_without_apply_keeps_block_order() {
    let mut nif = load("Cone");
    let before = types(&nif).join(",");
    let mut sorter = nif.sort_blocks().unwrap();
    sorter.visit(3);
    sorter.visit(0);
    drop(sorter);
    assert_eq!(types(&nif).join(","), before);
}

#[test]
fn sorted_file_round_trips() {
    let mut nif = load("Bones");
    let mut sorter = nif.sort_blocks().unwrap();
    sorter.visit(4); // the skinned shape before the bones
    sorter.visit(0);
    sorter.apply();

    let saved = reload_in_place(&mut nif);
    assert_eq!(saved.blocks().next().unwrap().block_name(), "NiTriShape");
    let shape = saved.shapes().next().unwrap();
    assert_eq!(
        saved.view_of(shape).unwrap().bone_list(),
        ["Bone1", "Bone2", "Bone3"]
    );
}
