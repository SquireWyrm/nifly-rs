//! Tests for NifFile::set_parent_node, on assets/Icosphere.nif:
//! 0 BSFadeNode -> [1], 1 NiNode "Indirect1" -> [2], 2 NiNode "Indirect2" -> [3], 3 BSTriShape.

use std::{fs, io::Cursor};

use nifly::{
    extra_data::ExtraString,
    file::{Index, NifFile, NifVersion, SaveOptions},
    nodes::NiNode,
    object3d::Transform,
    objects::NiAvObject,
};

fn icosphere() -> NifFile {
    NifFile::load(fs::read("assets/Icosphere.nif").unwrap().as_slice(), None).unwrap()
}

/// The file as it is in memory: no optimizing or sorting on save.
fn raw(nif: &mut NifFile) -> Vec<u8> {
    let mut cur = Cursor::new(Vec::new());
    let options = SaveOptions { optimize: false, sort_blocks: false };
    nif.save(&mut cur, Some(options)).unwrap();
    cur.into_inner()
}

fn node(nif: &NifFile, i: Index) -> &NiNode {
    nif.block(i).unwrap().downcast().unwrap()
}

/// Child indices of the three nodes.
fn hierarchy(nif: &NifFile) -> Vec<Vec<Index>> {
    (0..3).map(|i| nif.view_of(node(nif, i)).unwrap().children().indices().collect()).collect()
}

#[test]
fn none_moves_under_the_root() {
    let mut nif = icosphere();
    let before = nif.block_of::<NiAvObject>(3).unwrap().transform();
    assert!(nif.edit::<nifly::objects::NiAvObject>(3).is_some_and(|mut e| e.set_parent(None)));
    assert_eq!(hierarchy(&nif), [vec![1, 3], vec![2], vec![]]);
    // The local transform is kept.
    assert_eq!(nif.block_of::<NiAvObject>(3).unwrap().transform(), before);

    let mut cur = Cursor::new(Vec::new());
    nif.save(&mut cur, None).unwrap();
    let saved = NifFile::load(cur.into_inner().as_slice(), None).unwrap();
    let root = saved.root_node().unwrap();
    let names: Vec<_> = root.children().map(|c| c.name()).collect();
    assert_eq!(names, ["Indirect1", "IcosphereMesh"]);
}

#[test]
fn explicit_parent() {
    let mut nif = icosphere();
    assert!(nif.edit::<nifly::objects::NiAvObject>(2).is_some_and(|mut e| e.set_parent(Some(0))));
    assert_eq!(hierarchy(&nif), [vec![1, 2], vec![], vec![3]]);
    assert_eq!(nif.view::<NiNode>(2).unwrap().parent().unwrap().index(), 0);

    // Moving it back down restores the original layout, apart from child order.
    assert!(nif.edit::<nifly::objects::NiAvObject>(2).is_some_and(|mut e| e.set_parent(Some(1))));
    assert_eq!(hierarchy(&nif), [vec![1], vec![2], vec![3]]);
}

#[test]
fn current_parent_changes_nothing() {
    let mut nif = icosphere();
    let before = raw(&mut nif);
    assert!(nif.edit::<nifly::objects::NiAvObject>(1).is_some_and(|mut e| e.set_parent(Some(0))));
    assert!(nif.edit::<nifly::objects::NiAvObject>(1).is_some_and(|mut e| e.set_parent(None)));
    assert!(nif.edit::<nifly::objects::NiAvObject>(3).is_some_and(|mut e| e.set_parent(Some(2))));
    assert_eq!(raw(&mut nif), before);
}

#[test]
fn rejects() {
    let mut nif = icosphere();
    let not_av = nif.edit::<nifly::objects::NiAvObject>(1).and_then(|mut e| e.assign_extra_data(ExtraString("Note", "x"))).unwrap();
    let before = raw(&mut nif);
    let count = nif.blocks().count();
    let cases: [(Index, Option<Index>); 11] = [
        (1, Some(2)),                // into its own subtree
        (0, Some(2)),                // the root into its subtree
        (0, None),                   // the root under itself
        (1, Some(1)),                // itself
        (1, Some(3)),                // the parent is a shape
        (not_av, Some(0)),           // the child isn't an NiAVObject
        (1, Some(not_av)),           // the parent isn't an NiAVObject
        (count, Some(0)),            // no such child
        (1, Some(count)),            // no such parent
        (usize::MAX, None),          // child out of range
        (1, Some(u32::MAX as Index)), // not taken as "the root"
    ];
    for (child, parent) in cases {
        assert!(!nif.edit::<nifly::objects::NiAvObject>(child).is_some_and(|mut e| e.set_parent(parent)), "{child} under {parent:?}");
    }
    assert_eq!(raw(&mut nif), before);
}

#[test]
fn nodes_added_to_a_new_file() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let arm = nif.add_node("Arm", Transform::IDENTITY, None).unwrap();
    let hand = nif.add_node("Hand", Transform::IDENTITY, Some(arm)).unwrap();
    assert!(!nif.edit::<nifly::objects::NiAvObject>(arm).is_some_and(|mut e| e.set_parent(Some(hand))));
    assert!(nif.edit::<nifly::objects::NiAvObject>(hand).is_some_and(|mut e| e.set_parent(None)));
    let root = nif.root_node().unwrap();
    let names: Vec<_> = root.children().map(|c| c.name()).collect();
    assert_eq!(names, ["Arm", "Hand"]);
    assert_eq!(nif.view_of(node(&nif, arm)).unwrap().children().count(), 0);
}
