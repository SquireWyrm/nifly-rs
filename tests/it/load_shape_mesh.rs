//! Tests for NifFile::load_shape_mesh and NifFile::save_shape_mesh.

use crate::common::{UPSTREAM, open};
use std::{fs, io};

use nifly::{
    error::{Error, LoadError},
    file::NifFile,
};

fn starfield() -> NifFile {
    open(format!("{UPSTREAM}/TestNifFile_SF.nif"))
}

fn mesh() -> Vec<u8> {
    fs::read(format!("{UPSTREAM}/TestNifFile_SF.mesh")).unwrap()
}

/// (vertices, triangles, normals, UVs) of the first shape.
fn counts(nif: &NifFile) -> (usize, usize, usize, usize) {
    let s = nif.shapes().next().unwrap();
    (
        nif.view_of(s)
            .unwrap()
            .geometry_data()
            .unwrap()
            .vertex_count(),
        nif.view_of(s).unwrap().triangles().unwrap().len(),
        nif.view_of(s).unwrap().normals().map_or(0, |v| v.len()),
        nif.view_of(s).unwrap().uvs().map_or(0, |v| v.len()),
    )
}

#[test]
fn loads_mesh_data() {
    let mut nif = starfield();
    let shape = nif.shapes().indices().next().unwrap();
    assert_eq!(counts(&nif), (0, 0, 0, 0));

    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .unwrap()
            .load_mesh(0, mesh().as_slice())
            .unwrap()
    );
    assert_eq!(counts(&nif), (6616, 12132, 6616, 6616));
    let s = nif.shapes().next().unwrap();
    let tris = nif.view_of(s).unwrap().triangles().unwrap();
    assert!(tris.iter().flatten().all(|&i| usize::from(i) < 6616));
    // Per-vertex skin weights come with it.
    assert!(!nif.view_of(s).unwrap().bone_weights(0).is_empty());
    assert_eq!(nif.view_of(s).unwrap().mesh_paths(), ["TestNifFile_SF"]);
}

/// A reader that fails after a few bytes.
struct Failing(usize);

impl io::Read for Failing {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.0 == 0 {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "denied"));
        }
        let n = buf.len().min(self.0);
        buf[..n].fill(0);
        self.0 -= n;
        Ok(n)
    }
}

#[test]
fn errors_change_nothing() {
    let mut nif = starfield();
    let shape = nif.shapes().indices().next().unwrap();
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .unwrap()
            .load_mesh(0, mesh().as_slice())
            .unwrap()
    );
    let loaded = counts(&nif);

    // Cut short, or empty.
    let data = mesh();
    for bytes in [&data[..data.len() / 2], &[][..]] {
        match nif
            .edit::<nifly::geometry::NiShape>(shape)
            .unwrap()
            .load_mesh(0, bytes)
        {
            Err(Error::Io(e)) => assert_eq!(e.kind(), io::ErrorKind::UnexpectedEof),
            other => panic!("{other:?}"),
        }
    }
    // A version that can't be read.
    let mut bad = data.clone();
    bad[..4].copy_from_slice(&3u32.to_le_bytes());
    assert!(matches!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .unwrap()
            .load_mesh(0, bad.as_slice()),
        Err(Error::LoadFile(LoadError::UnsupportedVersion))
    ));
    // The reader's own error.
    match nif
        .edit::<nifly::geometry::NiShape>(shape)
        .unwrap()
        .load_mesh(0, Failing(10))
    {
        Err(Error::Io(e)) => assert_eq!(e.kind(), io::ErrorKind::PermissionDenied),
        other => panic!("{other:?}"),
    }
    assert_eq!(counts(&nif), loaded);
}

#[test]
fn false_without_external_mesh() {
    let mut nif = starfield();
    let shape = nif.shapes().indices().next().unwrap();
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(shape)
            .unwrap()
            .load_mesh(1, mesh().as_slice())
            .unwrap()
    );
    for i in [0, nif.blocks().count(), usize::MAX] {
        assert!(nif.edit::<nifly::geometry::NiShape>(i).is_none(), "{i}");
    }
    assert_eq!(counts(&nif), (0, 0, 0, 0));

    let mut cube = open("assets/Cube.nif");
    let cube_shape = cube.shapes().indices().next().unwrap();
    assert!(
        !cube
            .edit::<nifly::geometry::NiShape>(cube_shape)
            .unwrap()
            .load_mesh(0, mesh().as_slice())
            .unwrap()
    );
}

#[test]
fn saves_what_it_loaded() {
    let mut nif = starfield();
    let shape = nif.shapes().indices().next().unwrap();
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .unwrap()
            .load_mesh(0, mesh().as_slice())
            .unwrap()
    );

    let mut out = Vec::new();
    assert!(
        nif.shapes()
            .views()
            .next()
            .unwrap()
            .save_mesh(0, &mut out)
            .unwrap()
    );
    assert_eq!(out, mesh());

    // Loading what was saved gives the same mesh, and saving changed nothing.
    let before = counts(&nif);
    let mut again = starfield();
    assert!(
        again
            .edit::<nifly::geometry::NiShape>(shape)
            .unwrap()
            .load_mesh(0, out.as_slice())
            .unwrap()
    );
    assert_eq!(counts(&again), before);
    let mut twice = Vec::new();
    assert!(
        nif.shapes()
            .views()
            .next()
            .unwrap()
            .save_mesh(0, &mut twice)
            .unwrap()
    );
    assert_eq!(twice, out);
}

#[test]
fn saves_edits() {
    let mut nif = starfield();
    let shape = nif.shapes().indices().next().unwrap();
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .unwrap()
            .load_mesh(0, mesh().as_slice())
            .unwrap()
    );
    let s = nif.shapes().next().unwrap();
    let moved: Vec<_> = nif
        .view_of(s)
        .unwrap()
        .vertices()
        .unwrap()
        .iter()
        .map(|v| *v * 2.0)
        .collect();
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.set_vertices(&moved))
    );

    let mut out = Vec::new();
    assert!(
        nif.shapes()
            .views()
            .next()
            .unwrap()
            .save_mesh(0, &mut out)
            .unwrap()
    );
    let mut again = starfield();
    assert!(
        again
            .edit::<nifly::geometry::NiShape>(shape)
            .unwrap()
            .load_mesh(0, out.as_slice())
            .unwrap()
    );
    let reread = again.shapes().views().next().unwrap().vertices().unwrap();
    assert_eq!(reread.len(), moved.len());
    // Positions are stored compressed, so compare loosely.
    assert!(
        reread
            .iter()
            .zip(&moved)
            .all(|(a, b)| a.distance(*b) < 0.01 * b.length().max(1.0))
    );
}

/// A writer that always fails.
struct Broken;

impl io::Write for Broken {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
        Err(io::Error::new(io::ErrorKind::BrokenPipe, "broken"))
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn save_without_external_mesh_or_writer() {
    let nif = starfield();
    let s = nif.shapes().next().unwrap();
    let mut out = Vec::new();
    assert!(!nif.view_of(s).unwrap().save_mesh(1, &mut out).unwrap());
    assert!(out.is_empty());
    match nif.view_of(s).unwrap().save_mesh(0, Broken) {
        Err(Error::Io(e)) => assert_eq!(e.kind(), io::ErrorKind::BrokenPipe),
        other => panic!("{other:?}"),
    }
    let cube = open("assets/Cube.nif");
    assert!(
        !cube
            .shapes()
            .views()
            .next()
            .unwrap()
            .save_mesh(0, &mut out)
            .unwrap()
    );
}
