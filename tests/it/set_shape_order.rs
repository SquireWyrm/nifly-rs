//! Tests for NifFile::set_shape_order.

use crate::common::{load, save_in_place};
use std::io::Cursor;

use nifly::file::{NifFile, SaveOptions};

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
    let before = save_in_place(&mut nif);

    let cases: [&[&[u8]]; 5] = [
        &[],                  // empty
        &[b"missing"],        // unknown name
        &[b"Suzanne"],        // a node, not a shape
        &[&name, &name],      // repeated
        &[&name, b"missing"], // too many
    ];
    for order in cases {
        assert!(!nif.set_shape_order(order), "{order:?}");
        assert_eq!(save_in_place(&mut nif), before, "{order:?}");
    }
}

#[test]
fn shapes_must_be_root_children() {
    // Icosphere's shape is under Root -> Indirect1 -> Indirect2.
    let mut nif = load("Icosphere");
    let before = save_in_place(&mut nif);
    assert!(!nif.set_shape_order([shape_name(&nif)]));
    assert_eq!(save_in_place(&mut nif), before);
}

#[test]
fn sorts_blocks() {
    // Cone has unreferenced blocks out of the tree; a valid order sorts the blocks like saving
    // with sort_blocks does.
    let mut a = load("Cone");
    let mut b = load("Cone");
    assert!(a.set_shape_order([shape_name(&a)]));

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
