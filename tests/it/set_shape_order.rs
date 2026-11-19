//! Tests for NifFile::set_shape_order.

use crate::common::{load};
use std::io::Cursor;

use nifly::file::{NifFile, SaveOptions};

/// The file as it is in memory: no optimizing or sorting on save.
fn raw(nif: &mut NifFile) -> Vec<u8> {
    let mut cur = Cursor::new(Vec::new());
    let options = SaveOptions { optimize: false, sort_blocks: false };
    nif.save(&mut cur, Some(options)).unwrap();
    cur.into_inner()
}

fn shape_name(nif: &NifFile) -> Vec<u8> {
    nif.shapes().next().unwrap().name().to_vec()
}

#[test]
fn single_root_shape() {
    for asset in ["Bones", "Cube", "Suzanne"] {
        let mut nif = load(asset);
        let name = shape_name(&nif);
        assert!(nif.set_shape_order([&name]), "{asset}");
        assert_eq!(shape_name(&nif), name, "{asset}");
    }
}

#[test]
fn invalid_orders_change_nothing() {
    let mut nif = load("Suzanne");
    let name = shape_name(&nif);
    let before = raw(&mut nif);

    let cases: [&[&[u8]]; 5] = [
        &[],                            // empty
        &[b"missing"],                  // unknown name
        &[b"Suzanne"],                  // a node, not a shape
        &[&name, &name],                // repeated
        &[&name, b"missing"],           // too many
    ];
    for order in cases {
        assert!(!nif.set_shape_order(order), "{order:?}");
        assert_eq!(raw(&mut nif), before, "{order:?}");
    }
}

#[test]
fn shapes_must_be_root_children() {
    // Icosphere's shape is under Root -> Indirect1 -> Indirect2.
    let mut nif = load("Icosphere");
    let before = raw(&mut nif);
    assert!(!nif.set_shape_order([shape_name(&nif)]));
    assert_eq!(raw(&mut nif), before);
}

#[test]
fn sorts_blocks() {
    // Cone has unreferenced blocks out of the tree; a valid order sorts the blocks like saving
    // with sort_blocks does.
    let mut a = load("Cone");
    let mut b = load("Cone");
    assert!(a.set_shape_order([shape_name(&a)]));

    let mut cur = Cursor::new(Vec::new());
    b.save(&mut cur, Some(SaveOptions { optimize: false, sort_blocks: true })).unwrap();
    assert_eq!(raw(&mut a), cur.into_inner());
}
