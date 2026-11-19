//! Tests for NifFile::shape_mesh_paths and NifFile::set_shape_mesh_path.

use crate::common::{UPSTREAM, load, reload};
use std::fs;

use nifly::{file::NifFile, geometry::BsGeometryMeshData};

/// A hashed mesh path, as Starfield stores them.
const PATH: &str = r"abcdef0123456789\0123456789abcdef01234567";

fn starfield() -> NifFile {
    NifFile::load(fs::read(format!("{UPSTREAM}/TestNifFile_SF.nif")).unwrap().as_slice(), None).unwrap()
}

fn paths(nif: &NifFile) -> Vec<String> {
    nif.shapes().views().next().unwrap().mesh_paths().iter().map(ToString::to_string).collect()
}

#[test]
fn reads_paths() {
    let nif = starfield();
    assert_eq!(paths(&nif), ["TestNifFile_SF"]);
    // Reading leaves the geometry data reachable as before.
    let s = nif.shapes().next().unwrap();
    assert!(nif.view_of(s).unwrap().geometry_data().unwrap().downcast::<BsGeometryMeshData>().is_some());

    // Other shapes have none.
    let nif = load("Cube");
    assert!(nif.shapes().views().next().unwrap().mesh_paths().is_empty());
}

#[test]
fn sets_paths() {
    let mut nif = starfield();
    let shape = nif.shapes().indices().next().unwrap();
    assert!(nif.edit::<nifly::geometry::NiShape>(shape).is_some_and(|mut e| e.set_mesh_path(0, PATH)));
    let saved = reload(&mut nif);
    for nif in [&nif, &saved] {
        assert_eq!(paths(nif), [PATH]);
    }

    // Only existing LODs, on a BSGeometry.
    assert!(!nif.edit::<nifly::geometry::NiShape>(shape).is_some_and(|mut e| e.set_mesh_path(1, "x")));
    assert!(!nif.edit::<nifly::geometry::NiShape>(shape).is_some_and(|mut e| e.set_mesh_path(usize::MAX, "x")));
    assert_eq!(paths(&nif), [PATH]);
    for i in [0, nif.blocks().count(), usize::MAX] {
        assert!(!nif.edit::<nifly::geometry::NiShape>(i).is_some_and(|mut e| e.set_mesh_path(0, "x")), "{i}");
    }
    let mut cube = load("Cube");
    let cube_shape = cube.shapes().indices().next().unwrap();
    assert!(!cube.edit::<nifly::geometry::NiShape>(cube_shape).is_some_and(|mut e| e.set_mesh_path(0, "x")));
}
