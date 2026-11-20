//! Tests against assets/Suzanne.nif: a BSFadeNode root with BSXFlags extra data,
//! a Havok collision chain and one BSTriShape.
//!
//! Block layout:
//!   0 BSFadeNode "Suzanne" -> [1, 6, 7]
//!   1 BSXFlags
//!   2 bhkCompressedMeshShapeData
//!   3 bhkCompressedMeshShape -> [2]
//!   4 bhkMoppBvTreeShape -> [3]
//!   5 bhkRigidBody -> [4]
//!   6 bhkCollisionObject -> [5]
//!   7 BSTriShape "Suzanne:6" -> [8]
//!   8 BSLightingShaderProperty -> [9]
//!   9 BSShaderTextureSet

use std::{fs::File, ptr};

use nifly::{file::NifFile, objects::NiObject};

const BLOCK_NAMES: [&str; 10] = [
    "BSFadeNode",
    "BSXFlags",
    "bhkCompressedMeshShapeData",
    "bhkCompressedMeshShape",
    "bhkMoppBvTreeShape",
    "bhkRigidBody",
    "bhkCollisionObject",
    "BSTriShape",
    "BSLightingShaderProperty",
    "BSShaderTextureSet",
];

fn suzanne() -> NifFile {
    NifFile::load(File::open("assets/Suzanne.nif").unwrap(), None).unwrap()
}

fn block(nif: &NifFile, i: usize) -> &NiObject {
    nif.blocks().nth(i).unwrap()
}

/// Block index of `obj`, by address.
fn index_of(nif: &NifFile, obj: &NiObject) -> usize {
    nif.blocks().position(|b| ptr::eq(b, obj)).unwrap()
}

fn indices<'a>(nif: &NifFile, objs: impl Iterator<Item = &'a NiObject>) -> Vec<usize> {
    objs.map(|o| index_of(nif, o)).collect()
}

#[test]
fn child_blocks_of_every_block() {
    let nif = suzanne();
    let expected: [&[usize]; 10] = [
        &[1, 6, 7],
        &[],
        &[],
        &[2],
        &[3],
        &[4],
        &[5],
        &[8],
        &[9],
        &[],
    ];

    for (i, refs) in expected.iter().enumerate() {
        let actual = indices(&nif, nif.child_blocks(block(&nif, i)));
        assert_eq!(actual, *refs, "child blocks of {i} ({})", BLOCK_NAMES[i]);
    }
}

#[test]
fn tree_order() {
    let nif = suzanne();
    // Depth-first: extra data, then the collision chain, then the shape and its shader.
    assert_eq!(indices(&nif, nif.tree()), [0, 1, 6, 5, 4, 3, 2, 7, 8, 9]);
}

#[test]
fn subtrees() {
    let nif = suzanne();
    assert_eq!(indices(&nif, nif.subtree(block(&nif, 7))), [7, 8, 9]);
    assert_eq!(indices(&nif, nif.subtree(block(&nif, 6))), [6, 5, 4, 3, 2]);
    assert_eq!(indices(&nif, nif.subtree(block(&nif, 9))), [9]);
}

#[test]
fn parent_node_only_follows_child_refs() {
    let nif = suzanne();
    let root = nif.root_node().unwrap();

    let shape = block(&nif, 7);
    let shape = nif
        .view_of(shape)
        .and_then(|v| v.cast::<nifly::objects::NiAvObject>())
        .unwrap();
    assert_eq!(shape.parent().unwrap().index(), root.index());
    assert!(root.parent().is_none());

    // Extra data and collision are referenced by the root, but not as children,
    // so nifly's GetParentNode doesn't find a parent for them.
    assert!(
        nif.view_of(block(&nif, 1))
            .and_then(|v| v.cast::<nifly::objects::NiAvObject>())
            .and_then(|v| v.parent())
            .is_none()
    );
    assert!(
        nif.view_of(block(&nif, 6))
            .and_then(|v| v.cast::<nifly::objects::NiAvObject>())
            .and_then(|v| v.parent())
            .is_none()
    );
}

#[test]
fn block_by_index() {
    let nif = suzanne();
    for (i, name) in BLOCK_NAMES.iter().enumerate() {
        let b = nif.block(i).unwrap();
        assert_eq!(b.block_name(), *name);
        assert!(ptr::eq(b, block(&nif, i)));
    }
    assert!(nif.block(BLOCK_NAMES.len()).is_none());
    assert!(nif.block(u32::MAX as usize).is_none());
}
