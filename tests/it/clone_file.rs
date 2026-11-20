//! Tests for `impl Clone for NifFile`.

use crate::common::UPSTREAM;
use std::{fs, io::Cursor, ptr};

use nifly::{
    file::{LoadOptions, NifFile, SaveOptions},
    object3d::Vec3,
};

fn load(path: &str) -> NifFile {
    NifFile::load(fs::read(path).unwrap().as_slice(), None).unwrap()
}

fn bytes(nif: &mut NifFile) -> Vec<u8> {
    let mut cur = Cursor::new(Vec::new());
    nif.save(
        &mut cur,
        Some(SaveOptions {
            optimize: false,
            sort_blocks: false,
        }),
    )
    .unwrap();
    cur.into_inner()
}

/// Every test file, from both the crate's assets and nifly's inputs.
fn all_files() -> Vec<String> {
    let mut paths = Vec::new();
    for dir in ["assets", UPSTREAM] {
        for entry in fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_some_and(|e| e == "nif")
                && !path.ends_with("TestNifFile_Corrupted.nif")
            {
                paths.push(path.to_string_lossy().into_owned());
            }
        }
    }
    paths
}

#[test]
fn saves_identically() {
    for path in all_files() {
        let mut nif = load(&path);
        let mut copy = nif.clone();
        assert_eq!(copy.version(), nif.version(), "{path}");
        assert_eq!(copy.blocks().count(), nif.blocks().count(), "{path}");
        assert_eq!(copy.has_unknown(), nif.has_unknown(), "{path}");
        assert_eq!(bytes(&mut copy), bytes(&mut nif), "{path}");
    }
}

#[test]
fn copies_are_independent() {
    let mut nif = load("assets/Cube.nif");
    let original = bytes(&mut nif.clone());
    let mut copy = nif.clone();
    let cube = copy.shapes().indices().next().unwrap();
    copy.block_of_mut::<nifly::objects::NiObjectNet>(cube)
        .unwrap()
        .set_name("Copy");
    assert!(
        copy.edit::<nifly::geometry::NiShape>(cube)
            .is_some_and(|mut e| e.offset(Vec3::X, None))
    );

    assert_eq!(nif.shapes().next().unwrap().name(), "Cube");
    assert_eq!(bytes(&mut nif), original);
    assert_eq!(copy.shapes().next().unwrap().name(), "Copy");

    // Blocks, and the geometry data shapes link to, are the copy's own.
    let (a, b) = (nif.shapes().next().unwrap(), copy.shapes().next().unwrap());
    assert!(!ptr::eq(a, b));
    assert!(!copy.contains(a) && !nif.contains(b));
    let nif = load("assets/Bones.nif");
    let copy = nif.clone();
    let s = copy.shapes().next().unwrap();
    assert!(copy.contains(copy.view_of(s).unwrap().geometry_data().unwrap()));
}

#[test]
fn keeps_terrain_and_loaded_meshes() {
    let nif = NifFile::load(
        fs::read("assets/Cube.nif").unwrap().as_slice(),
        Some(LoadOptions { is_terrain: true }),
    )
    .unwrap();
    assert!(nif.clone().is_terrain());

    // A Starfield shape's loaded mesh data is copied.
    let mut nif = load(&format!("{UPSTREAM}/TestNifFile_SF.nif"));
    let shape = nif.shapes().indices().next().unwrap();
    let mesh = fs::read(format!("{UPSTREAM}/TestNifFile_SF.mesh")).unwrap();
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .unwrap()
            .load_mesh(0, mesh.as_slice())
            .unwrap()
    );
    let copy = nif.clone();
    let (a, b) = (nif.shapes().next().unwrap(), copy.shapes().next().unwrap());
    let vertices = nif.view_of(a).unwrap().vertices().unwrap();
    assert!(!vertices.is_empty());
    assert_eq!(copy.view_of(b).unwrap().vertices().unwrap(), vertices);
    assert_eq!(
        copy.view_of(b).unwrap().triangles(),
        nif.view_of(a).unwrap().triangles()
    );
}
