//! Tests for NifFile::rename_duplicate_shapes.

use crate::common::{load, save};

use nifly::{file::NifFile, object3d::Transform};

const ASSETS: [&str; 5] = ["Bones", "Cone", "Cube", "Icosphere", "Suzanne"];

fn identity() -> Transform {
    Transform::IDENTITY
}

fn shape_name(nif: &NifFile) -> Vec<u8> {
    nif.shapes().next().unwrap().name().to_vec()
}

#[test]
fn no_duplicates_changes_nothing() {
    for asset in ASSETS {
        let mut nif = load(asset);
        let before = save(&mut nif);
        assert!(!nif.rename_duplicate_shapes(), "{asset}");
        assert_eq!(save(&mut nif), before, "{asset}");
    }
}

#[test]
fn first_shape_keeps_its_name() {
    // Suzanne's only shape is a child of the root. A sibling node with the same name doesn't
    // rename it, because the first shape child of each node is skipped.
    let mut nif = load("Suzanne");
    let name = shape_name(&nif);
    nif.add_node(&name, identity(), None).unwrap();

    assert!(!nif.rename_duplicate_shapes());
    assert_eq!(shape_name(&nif), name);
}

#[test]
fn shapes_below_root_children_are_not_checked() {
    // Icosphere's shape is under Root -> Indirect1 -> Indirect2, deeper than nifly looks.
    let mut nif = load("Icosphere");
    let name = shape_name(&nif);
    let indirect2 = nif.block_index(
        nif.find_block_by_name::<nifly::nodes::NiNode>("Indirect2")
            .unwrap(),
    );
    nif.add_node(&name, identity(), indirect2).unwrap();

    assert!(!nif.rename_duplicate_shapes());
    assert_eq!(shape_name(&nif), name);
}
