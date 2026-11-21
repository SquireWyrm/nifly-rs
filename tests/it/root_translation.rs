//! Tests for NifFile::root_translation and NifFile::root_index.

use nifly::{
    file::{NifFile, NifVersion},
    object3d::{Transform, Vec3},
};

use crate::common::open;

#[test]
fn reads_the_root_translation() {
    // Morrowind's billboard root is raised by 200.
    let nif = open("nifly-sys/vendor/nifly/tests/input/TestNifFile_Billboard_MW.nif");
    let t = nif.root_translation().unwrap();
    assert!(
        (t.z - 200.0).abs() < 1e-3 && t.x == 0.0 && t.y.abs() < 1e-5,
        "{t:?}"
    );
    assert_eq!(Some(t), nif.root_node().map(|r| r.transform().translation));

    assert_eq!(open("assets/Cube.nif").root_translation(), Some(Vec3::ZERO));
}

#[test]
fn follows_changes() {
    let mut nif = open("assets/Cube.nif");
    let moved = Transform {
        translation: Vec3::new(1.0, 2.0, 3.0),
        ..Transform::IDENTITY
    };
    assert!(nif.set_node_transform_to_parent("Scene Root", moved, false));
    assert_eq!(nif.root_translation(), Some(Vec3::new(1.0, 2.0, 3.0)));
}

#[test]
fn root_index_is_the_root_nodes() {
    let nif = open("assets/Cube.nif");
    assert_eq!(nif.root_index(), Some(nif.root_node().unwrap().index()));
    assert_eq!(NifFile::create(NifVersion::SKYRIM_SE).root_index(), Some(0));

    // A file with no nodes has no root.
    let mut empty = NifFile::create(NifVersion::SKYRIM_SE);
    assert!(empty.delete_node("Scene Root"));
    assert_eq!(empty.root_index(), None);
}
