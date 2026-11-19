//! Tests for NifFile::set_shape_bone_weights.

use std::{fs, io::Cursor};

use nifly::{
    file::{NifFile, NifVersion, SaveOptions},
    geometry::VertexWeight,
    skin::NiSkinData,
};

fn load(path: &str) -> NifFile {
    NifFile::load(fs::read(path).unwrap().as_slice(), None).unwrap()
}

fn raw(nif: &mut NifFile) -> Vec<u8> {
    let mut cur = Cursor::new(Vec::new());
    let options = SaveOptions { optimize: false, sort_blocks: false };
    nif.save(&mut cur, Some(options)).unwrap();
    cur.into_inner()
}

fn w(vertex: u16, weight: f32) -> VertexWeight {
    VertexWeight { vertex, weight }
}

fn weights(nif: &NifFile, bone: usize) -> Vec<VertexWeight> {
    nif.shapes().views().next().unwrap().bone_weights(bone)
}

#[test]
fn replaces_one_bones_weights() {
    let mut nif = load("assets/Bones.nif");
    let shape = nif.shapes().indices().next().unwrap();
    let (bone0, bone2) = (weights(&nif, 0), weights(&nif, 2));

    let new = [w(5, 0.25), w(1, 1.0), w(3, 0.5)];
    assert!(nif.edit::<nifly::geometry::NiShape>(shape).is_some_and(|mut e| e.set_bone_weights(1, new)));
    // Read back sorted by vertex.
    assert_eq!(weights(&nif, 1), [w(1, 1.0), w(3, 0.5), w(5, 0.25)]);
    assert_eq!(weights(&nif, 0), bone0);
    assert_eq!(weights(&nif, 2), bone2);

    let mut cur = Cursor::new(Vec::new());
    nif.save(&mut cur, None).unwrap();
    let saved = NifFile::load(cur.into_inner().as_slice(), None).unwrap();
    assert_eq!(weights(&saved, 1), [w(1, 1.0), w(3, 0.5), w(5, 0.25)]);
    assert!(saved.blocks_of::<NiSkinData>().next().unwrap().has_vertex_weights());
}

#[test]
fn duplicates_and_tiny_weights() {
    let mut nif = load("assets/Bones.nif");
    let shape = nif.shapes().indices().next().unwrap();
    // The last weight for a vertex wins; weights below 0.0001 are dropped.
    assert!(nif.edit::<nifly::geometry::NiShape>(shape).is_some_and(|mut e| e.set_bone_weights(0, [w(2, 0.1), w(2, 0.9), w(4, 0.00005), w(6, 0.5)])));
    assert_eq!(weights(&nif, 0), [w(2, 0.9), w(6, 0.5)]);

    // An empty list clears the bone.
    assert!(nif.edit::<nifly::geometry::NiShape>(shape).is_some_and(|mut e| e.set_bone_weights(0, [])));
    assert!(weights(&nif, 0).is_empty());
}

#[test]
fn rejects() {
    let mut nif = load("assets/Bones.nif");
    let before = raw(&mut nif);
    let shape = nif.shapes().indices().next().unwrap();
    let root = nif.nodes().indices().next().unwrap();
    let vertices = 164; // Bones.nif's mesh

    assert!(!nif.edit::<nifly::geometry::NiShape>(shape).is_some_and(|mut e| e.set_bone_weights(3, [w(0, 1.0)])), "bone out of range");
    assert!(!nif.edit::<nifly::geometry::NiShape>(shape).is_some_and(|mut e| e.set_bone_weights(0, [w(0, 1.0), w(vertices, 1.0)])), "vertex out of range");
    assert!(!nif.edit::<nifly::geometry::NiShape>(root).is_some_and(|mut e| e.set_bone_weights(0, [w(0, 1.0)])), "not a shape");
    assert!(!nif.edit::<nifly::geometry::NiShape>(usize::MAX).is_some_and(|mut e| e.set_bone_weights(0, [w(0, 1.0)])));
    assert_eq!(raw(&mut nif), before);

    let mut cube = load("assets/Cube.nif");
    let shape = cube.shapes().indices().next().unwrap();
    assert!(!cube.edit::<nifly::geometry::NiShape>(shape).is_some_and(|mut e| e.set_bone_weights(0, [w(0, 1.0)])), "unskinned");
}

#[test]
fn per_vertex_shapes_are_rejected() {
    // BSTriShape stores weights per vertex, which is where shape_bone_weights reads them.
    let mut nif = load("assets/Bones.nif");
    nif.optimize_for(NifVersion::SKYRIM_SE, None).unwrap();
    let before = raw(&mut nif);
    let shape = nif.shapes().indices().next().unwrap();
    assert!(!nif.edit::<nifly::geometry::NiShape>(shape).is_some_and(|mut e| e.set_bone_weights(0, [w(0, 1.0)])));
    assert_eq!(raw(&mut nif), before);
}
