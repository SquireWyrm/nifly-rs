//! Tests for NifFile::block_ref_count.

use crate::common::load;

use nifly::{file::NifFile, geometry::NiShape};

/// (refs, refs + pointers) for every block.
fn counts(nif: &NifFile) -> Vec<(usize, usize)> {
    (0..nif.blocks().count())
        .map(|i| (nif.block_ref_count(i, false), nif.block_ref_count(i, true)))
        .collect()
}

#[test]
fn bones_pointers() {
    // 0 root, 1-3 bones, 4 shape, 5 data, 6 skin instance, 7 skin data, 8 partition,
    // 9 shader, 10 texture set.
    let nif = load("Bones");
    let c = counts(&nif);
    // Nothing owns the root, but the skin instance points to it as the skeleton root.
    assert_eq!(c[0], (0, 1));
    // Each bone is a child of the root and in the skin instance's bone list.
    assert_eq!(c[1..=3], [(1, 2); 3]);
    // Everything else is owned by exactly one ref.
    assert!(c[4..].iter().all(|&rc| rc == (1, 1)), "{c:?}");
}

#[test]
fn collision_targets_are_pointers() {
    // bhkCollisionObject and bhkCompressedMeshShape both point back to the root.
    let nif = load("Suzanne");
    let c = counts(&nif);
    assert_eq!(c[0], (0, 2));
    assert!(c[1..].iter().all(|&rc| rc == (1, 1)), "{c:?}");
}

#[test]
fn unreferenced_blocks() {
    // Cone: 0 root, 1 unreferenced node, 2 unreferenced shader, 3 shape.
    let nif = load("Cone");
    assert_eq!(counts(&nif), [(0, 0), (0, 0), (0, 0), (1, 1)]);
}

#[test]
fn refs_from_outside_the_tree_count() {
    // Bones' skin data is only referenced by the skin instance, which isn't a node.
    let nif = load("Bones");
    let data = nif
        .blocks()
        .position(|b| b.block_name() == "NiSkinData")
        .unwrap();
    assert_eq!(nif.block_ref_count(data, false), 1);
}

#[test]
fn no_block_at_index() {
    let nif = load("Cube");
    let len = nif.blocks().count();
    for index in [len, len + 100, u32::MAX as usize, usize::MAX] {
        assert_eq!(nif.block_ref_count(index, true), 0, "{index}");
    }
}

#[test]
fn follows_edits() {
    let mut nif = load("Suzanne");
    let shape = nif
        .block_index(nif.blocks_of::<NiShape>().next().unwrap())
        .unwrap();
    // Attaching a node under the root adds a ref to it; the root itself is still unowned.
    let root = nif.root_node().unwrap().index();
    let node = nif
        .add_node("Extra", nif.root_node().unwrap().transform(), None)
        .unwrap();
    assert_eq!(nif.block_ref_count(node, false), 1);
    assert_eq!(nif.block_ref_count(root, false), 0);

    // Extra data on the shape is one more owned block.
    let extra = nif
        .edit::<nifly::objects::NiAvObject>(shape)
        .unwrap()
        .assign_extra_data(nifly::extra_data::ExtraInteger("Count", 3))
        .unwrap();
    assert_eq!(nif.block_ref_count(extra, false), 1);
}
