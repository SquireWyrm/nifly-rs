//! Tests for NifFile::shape_bone_bounds and NifFile::set_shape_bone_bounds.

use crate::common::UPSTREAM;
use std::{fs, io::Cursor};

use nifly::{
    file::{NifFile, SaveOptions},
    object3d::{BoundingSphere, Vec3},
    skin::NiSkinData,
};

fn load(path: &str) -> NifFile {
    NifFile::load(fs::read(path).unwrap().as_slice(), None).unwrap()
}

fn raw(nif: &mut NifFile) -> Vec<u8> {
    let mut cur = Cursor::new(Vec::new());
    let options = SaveOptions {
        optimize: false,
        sort_blocks: false,
    };
    nif.save(&mut cur, Some(options)).unwrap();
    cur.into_inner()
}

fn sample() -> BoundingSphere {
    BoundingSphere::new(Vec3::new(1.0, 2.0, 3.0), 4.5)
}

#[test]
fn bones_values() {
    // Each sphere is in its bone's coordinates, where Y points along the limb.
    let nif = load("assets/Bones.nif");
    let shape = nif.shapes().next().unwrap();
    for (i, (y, radius)) in [(0.516, 0.844), (0.371, 0.955), (0.105, 0.736)]
        .into_iter()
        .enumerate()
    {
        let b = nif.view_of(shape).unwrap().bone_bounds(i).unwrap();
        assert!(
            b.center.distance(Vec3::new(0.0, y, 0.0)) < 1e-3,
            "bone {i}: {b:?}"
        );
        assert!((b.radius - radius).abs() < 1e-3, "bone {i}: {b:?}");
    }
    assert_eq!(nif.view_of(shape).unwrap().bone_bounds(3), None);
    assert_eq!(nif.view_of(shape).unwrap().bone_bounds(usize::MAX), None);
}

#[test]
fn set_and_save_ni_skin_data() {
    let mut nif = load("assets/Bones.nif");
    let shape = nif.shapes().indices().next().unwrap();
    let before: Vec<_> = (0..3)
        .map(|i| nif.shapes().views().next().unwrap().bone_bounds(i))
        .collect();

    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.set_bone_bounds(1, sample()))
    );
    let s = nif.shapes().next().unwrap();
    assert_eq!(nif.view_of(s).unwrap().bone_bounds(1), Some(sample()));
    assert_eq!(nif.view_of(s).unwrap().bone_bounds(0), before[0]);
    assert_eq!(nif.view_of(s).unwrap().bone_bounds(2), before[2]);

    let mut cur = Cursor::new(Vec::new());
    nif.save(&mut cur, None).unwrap();
    let saved = NifFile::load(cur.into_inner().as_slice(), None).unwrap();
    assert_eq!(
        saved.shapes().views().next().unwrap().bone_bounds(1),
        Some(sample())
    );
    assert_eq!(saved.blocks_of::<NiSkinData>().count(), 1);
}

#[test]
fn set_bs_skin_bone_data() {
    // Fallout 4 stores bone bounds in BSSkin::BoneData.
    let mut nif = load(&format!("{UPSTREAM}/TestNifFile_Skinned_FO4.nif"));
    let shape = nif.shapes().indices().next().unwrap();
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.set_bone_bounds(0, sample()))
    );
    assert_eq!(
        nif.shapes().views().next().unwrap().bone_bounds(0),
        Some(sample())
    );
}

#[test]
fn duplicate_names_target_the_given_shape() {
    // Two shapes with the same name: setting bounds on the second must not touch the first.
    let mut nif = load(&format!("{UPSTREAM}/TestNifFile_Skinned_SE.nif"));
    let shapes: Vec<_> = nif.shapes().indices().collect();
    assert_eq!(shapes.len(), 2);
    nif.block_of_mut::<nifly::objects::NiObjectNet>(shapes[1])
        .unwrap()
        .set_name("cylinder_1");
    let first_before = nif.shapes().views().next().unwrap().bone_bounds(0);

    assert!(
        nif.edit::<nifly::geometry::NiShape>(shapes[1])
            .is_some_and(|mut e| e.set_bone_bounds(0, sample()))
    );
    let mut iter = nif.shapes();
    let (first, second) = (iter.next().unwrap(), iter.next().unwrap());
    assert_eq!(nif.view_of(first).unwrap().bone_bounds(0), first_before);
    assert_eq!(nif.view_of(second).unwrap().bone_bounds(0), Some(sample()));
}

#[test]
fn rejects() {
    let mut nif = load("assets/Bones.nif");
    let before = raw(&mut nif);
    let shape = nif.shapes().indices().next().unwrap();
    let root = nif.nodes().indices().next().unwrap();
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.set_bone_bounds(3, sample())),
        "bone out of range"
    );
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.set_bone_bounds(usize::MAX, sample()))
    );
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(root)
            .is_some_and(|mut e| e.set_bone_bounds(0, sample())),
        "not a shape"
    );
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(usize::MAX)
            .is_some_and(|mut e| e.set_bone_bounds(0, sample()))
    );
    assert_eq!(raw(&mut nif), before);

    let mut cube = load("assets/Cube.nif");
    let shape = cube.shapes().indices().next().unwrap();
    assert!(
        !cube
            .edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.set_bone_bounds(0, sample())),
        "unskinned"
    );
    assert_eq!(cube.shapes().views().next().unwrap().bone_bounds(0), None);
}
