//! Tests for NifFile::root_translation.

use std::fs;

use nifly::{
    file::NifFile,
    object3d::{Transform, Vec3},
};

fn load(path: &str) -> NifFile {
    NifFile::load(fs::read(path).unwrap().as_slice(), None).unwrap()
}

#[test]
fn reads_the_root_translation() {
    // Morrowind's billboard root is raised by 200.
    let nif = load("nifly-sys/vendor/nifly/tests/input/TestNifFile_Billboard_MW.nif");
    let t = nif.root_translation().unwrap();
    assert!(
        (t.z - 200.0).abs() < 1e-3 && t.x == 0.0 && t.y.abs() < 1e-5,
        "{t:?}"
    );
    assert_eq!(Some(t), nif.root_node().map(|r| r.transform().translation));

    assert_eq!(load("assets/Cube.nif").root_translation(), Some(Vec3::ZERO));
}

#[test]
fn follows_changes() {
    let mut nif = load("assets/Cube.nif");
    let moved = Transform {
        translation: Vec3::new(1.0, 2.0, 3.0),
        ..Transform::IDENTITY
    };
    assert!(nif.set_node_transform_to_parent("Scene Root", moved, false));
    assert_eq!(nif.root_translation(), Some(Vec3::new(1.0, 2.0, 3.0)));
}
