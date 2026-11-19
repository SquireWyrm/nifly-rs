//! Tests against assets/Bones.nif: an NiTriShape skinned to three bones.
//!
//! Block layout:
//!   0 NiNode "Scene Root" -> [1, 2, 3, 4]
//!   1 NiNode "Bone1"                     t = (0, 0, 0)
//!   2 NiNode "Bone2"                     t = (0, 0, 0.75)
//!   3 NiNode "Bone3"                     t = (0, 0, 1.5)
//!   4 NiTriShape "Limb" -> [5, 6, 9]
//!   5 NiTriShapeData
//!   6 NiSkinInstance -> [7, 8]           bones are pointers, not child refs
//!   7 NiSkinData
//!   8 NiSkinPartition
//!   9 BSLightingShaderProperty -> [10]
//!  10 BSShaderTextureSet
//!
//! The bones are flat: all are direct children of the root.

use std::{fs::File, ptr};

use nifly::{file::NifFile, geometry::NiShape, nodes::NiNode, objects::NiObject};

const BONES: [&str; 3] = ["Bone1", "Bone2", "Bone3"];

fn bones() -> NifFile {
    NifFile::load(File::open("assets/Bones.nif").unwrap(), None).unwrap()
}

fn limb(nif: &NifFile) -> &NiShape {
    nif.blocks_of::<NiShape>().next().unwrap()
}

fn index_of(nif: &NifFile, obj: &NiObject) -> usize {
    nif.blocks().position(|b| ptr::eq(b, obj)).unwrap()
}

#[test]
fn bone_ids_resolve_to_named_nodes() {
    let nif = bones();
    let shape = limb(&nif);

    let names = nif.view_of(shape).unwrap().bone_list();
    let ids = nif.view_of(shape).unwrap().bone_ids();
    assert_eq!(names.len(), ids.len());

    for (id, name) in ids.into_iter().zip(names) {
        let bone = nif.block(id).unwrap().downcast::<NiNode>().expect("bone is a node");
        assert_eq!(bone.name(), name);
    }
}

#[test]
fn skin_blocks_reached_through_shape() {
    let nif = bones();
    let refs: Vec<_> = nif.child_blocks(limb(&nif)).map(|b| index_of(&nif, b)).collect();
    assert_eq!(refs, [5, 6, 9]);

    // The skin instance's bones are pointers, which GetChildIndices doesn't report.
    let skin = nif.block(6).unwrap();
    let skin_refs: Vec<_> = nif.child_blocks(skin).map(|b| index_of(&nif, b)).collect();
    assert_eq!(skin_refs, [7, 8]);
}

#[test]
fn bone_weights_per_bone() {
    let nif = bones();
    let shape = limb(&nif);

    let counts: Vec<_> = (0..BONES.len()).map(|b| nif.view_of(shape).unwrap().bone_weights(b).len()).collect();
    assert_eq!(counts, [102, 115, 87]);

    for b in 0..BONES.len() {
        let weights = nif.view_of(shape).unwrap().bone_weights(b);
        // Sorted by vertex, no duplicates, and every weight in (0, 1].
        assert!(weights.windows(2).all(|w| w[0].vertex < w[1].vertex), "bone {b} not sorted");
        assert!(weights.iter().all(|w| w.weight > 0.0 && w.weight <= 1.0 + 1e-6), "bone {b}");
    }
}

#[test]
fn bone_weights_sum_to_one_per_vertex() {
    let nif = bones();
    let shape = limb(&nif);

    let mut sums = std::collections::BTreeMap::new();
    for b in 0..BONES.len() {
        for w in nif.view_of(shape).unwrap().bone_weights(b) {
            *sums.entry(w.vertex).or_insert(0.0f32) += w.weight;
        }
    }

    // Every vertex (0..=163) is weighted, and each vertex's weights are normalized.
    assert_eq!(sums.len(), 164);
    assert_eq!(sums.keys().copied().collect::<Vec<_>>(), (0..164).collect::<Vec<u16>>());
    for (vertex, sum) in sums {
        assert!((sum - 1.0).abs() < 1e-4, "vertex {vertex} weights sum to {sum}");
    }
}

#[test]
fn bone_weights_out_of_range() {
    let nif = bones();
    let shape = limb(&nif);
    assert!(nif.view_of(shape).unwrap().bone_weights(BONES.len()).is_empty());
    assert!(nif.view_of(shape).unwrap().bone_weights(usize::MAX).is_empty());
}
