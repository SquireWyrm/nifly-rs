//! Tests for NifFile::offset_shape, scale_shape and rotate_shape.

use crate::common::reload;
use std::fs;

use nifly::{
    file::{Index, NifFile},
    object3d::Vec3,
};

fn load(path: &str) -> NifFile {
    NifFile::load(fs::read(path).unwrap().as_slice(), None).unwrap()
}

fn vertices(nif: &NifFile, shape: Index) -> Vec<Vec3> {
    nif.view::<nifly::geometry::NiShape>(shape)
        .unwrap()
        .vertices()
        .unwrap()
}

fn normals(nif: &NifFile, shape: Index) -> Vec<Vec3> {
    nif.view::<nifly::geometry::NiShape>(shape)
        .unwrap()
        .normals()
        .unwrap()
}

fn close(a: &[Vec3], b: &[Vec3], tolerance: f32) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x.distance(*y) <= tolerance)
}

/// A quad tilted 45 degrees about Y, with normals (1, 0, 1)/sqrt(2), in Bones.nif.
fn tilted_quad() -> (NifFile, Index) {
    let mut nif = load("assets/Bones.nif");
    let v = [
        Vec3::new(1.0, -1.0, -1.0),
        Vec3::new(1.0, 1.0, -1.0),
        Vec3::new(-1.0, 1.0, 1.0),
        Vec3::new(-1.0, -1.0, 1.0),
    ];
    let n = Vec3::new(1.0, 0.0, 1.0).normalize();
    let quad = nif
        .create_shape("Quad", &v, &[[0, 1, 2], [0, 2, 3]], None, Some(&[n; 4]))
        .unwrap();
    (nif, quad)
}

#[test]
fn offsets() {
    let mut nif = load("assets/Bones.nif");
    let limb = nif.shapes().indices().next().unwrap();
    let (before, normals_before) = (vertices(&nif, limb), normals(&nif, limb));
    let offset = Vec3::new(1.0, -2.0, 3.0);
    assert!(
        nif.edit::<nifly::geometry::NiShape>(limb)
            .is_some_and(|mut e| e.offset(offset, None))
    );
    let expected: Vec<_> = before.iter().map(|v| *v + offset).collect();
    assert!(close(&vertices(&nif, limb), &expected, 1e-5));
    // Directions don't change when moving.
    assert!(close(&normals(&nif, limb), &normals_before, 1e-6));
    let saved = reload(&mut nif);
    assert!(close(
        &vertices(&saved, saved.shapes().indices().next().unwrap()),
        &expected,
        1e-5
    ));

    // Weighted: 0 stays, 0.5 moves halfway, 1 moves fully.
    let mut nif = load("assets/Bones.nif");
    let n = before.len();
    let weights: Vec<f32> = (0..n).map(|i| [0.0, 0.5, 1.0][i % 3]).collect();
    assert!(
        nif.edit::<nifly::geometry::NiShape>(limb)
            .is_some_and(|mut e| e.offset(offset, Some(&weights)))
    );
    let expected: Vec<_> = before
        .iter()
        .zip(&weights)
        .map(|(v, w)| *v + offset * *w)
        .collect();
    assert!(close(&vertices(&nif, limb), &expected, 1e-5));
}

#[test]
fn scales_with_normals() {
    // Uniform: positions double, normals stay.
    let mut nif = load("assets/Bones.nif");
    let limb = nif.shapes().indices().next().unwrap();
    let (before, normals_before) = (vertices(&nif, limb), normals(&nif, limb));
    assert!(
        nif.edit::<nifly::geometry::NiShape>(limb)
            .is_some_and(|mut e| e.scale(Vec3::new(2.0, 2.0, 2.0), None))
    );
    assert!(close(
        &vertices(&nif, limb),
        &before.iter().map(|v| *v * 2.0).collect::<Vec<_>>(),
        1e-5
    ));
    assert!(close(&normals(&nif, limb), &normals_before, 1e-5));

    // Non-uniform: normals stay perpendicular to the scaled surface.
    let (mut nif, quad) = tilted_quad();
    assert!(
        nif.edit::<nifly::geometry::NiShape>(quad)
            .is_some_and(|mut e| e.scale(Vec3::new(3.0, 1.0, 0.5), None))
    );
    let v = vertices(&nif, quad);
    let (e1, e2) = (v[1] - v[0], v[3] - v[0]);
    for n in normals(&nif, quad) {
        assert!((n.length() - 1.0).abs() < 1e-5);
        assert!(n.dot(e1).abs() < 1e-5 && n.dot(e2).abs() < 1e-5, "{n:?}");
    }
}

#[test]
fn rotates_with_normals() {
    let (mut nif, quad) = tilted_quad();
    // A quarter turn about Z: +X goes to +Y.
    assert!(
        nif.edit::<nifly::geometry::NiShape>(quad)
            .is_some_and(|mut e| e.rotate(Vec3::new(0.0, 0.0, 90.0), None))
    );
    assert!(close(
        &normals(&nif, quad),
        &[Vec3::new(0.0, 1.0, 1.0).normalize(); 4],
        1e-5
    ));
    assert!(close(
        &vertices(&nif, quad)[..1],
        &[Vec3::new(1.0, 1.0, -1.0)],
        1e-5
    ));

    // X first, then Y: +Y goes to +Z, then to +X.
    let (mut nif, quad) = tilted_quad();
    let probe = Vec3::new(0.0, 1.0, 0.0);
    assert!(
        nif.edit::<nifly::geometry::NiShape>(quad)
            .is_some_and(|mut e| e.set_vertex(0, probe))
    );
    assert!(
        nif.edit::<nifly::geometry::NiShape>(quad)
            .is_some_and(|mut e| e.rotate(Vec3::new(90.0, 90.0, 0.0), None))
    );
    assert!(vertices(&nif, quad)[0].distance(Vec3::X) < 1e-5);

    // A BSTriShape's stored normals turn too, within their precision.
    let mut nif = load("assets/Cube.nif");
    let cube = nif.shapes().indices().next().unwrap();
    let before = normals(&nif, cube);
    assert!(
        nif.edit::<nifly::geometry::NiShape>(cube)
            .is_some_and(|mut e| e.rotate(Vec3::new(0.0, 0.0, 180.0), None))
    );
    let turned: Vec<_> = before.iter().map(|n| Vec3::new(-n.x, -n.y, n.z)).collect();
    assert!(close(&normals(&nif, cube), &turned, 0.02));
}

#[test]
fn weighted_rotation() {
    let (mut nif, quad) = tilted_quad();
    let before = vertices(&nif, quad);
    assert!(
        nif.edit::<nifly::geometry::NiShape>(quad)
            .is_some_and(|mut e| e.rotate(Vec3::new(0.0, 0.0, 90.0), Some(&[0.0, 1.0, 0.0, 1.0])))
    );
    let after = vertices(&nif, quad);
    assert_eq!((after[0], after[2]), (before[0], before[2]));
    assert!(after[1].distance(Vec3::new(-1.0, 1.0, -1.0)) < 1e-5);
}

#[test]
fn false_changes_nothing() {
    let mut nif = load("assets/Bones.nif");
    let limb = nif.shapes().indices().next().unwrap();
    let before = vertices(&nif, limb);
    let n = before.len();
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(limb)
            .is_some_and(|mut e| e.offset(Vec3::X, Some(&vec![1.0; n - 1])))
    );
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(limb)
            .is_some_and(|mut e| e.offset(Vec3::X, Some(&vec![f32::NAN; n])))
    );
    for scale in [
        Vec3::new(0.0, 1.0, 1.0),
        Vec3::new(-1.0, 1.0, 1.0),
        Vec3::new(f32::INFINITY, 1.0, 1.0),
    ] {
        assert!(
            !nif.edit::<nifly::geometry::NiShape>(limb)
                .is_some_and(|mut e| e.scale(scale, None)),
            "{scale:?}"
        );
    }
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(limb)
            .is_some_and(|mut e| e.rotate(Vec3::new(f32::NAN, 0.0, 0.0), None))
    );
    assert_eq!(vertices(&nif, limb), before);

    let mut nif = load("nifly-sys/vendor/nifly/tests/input/TestNifFile_Particles_MW.nif");
    let particles = nif
        .shapes()
        .indexed()
        .find(|(_, s)| s.name() == "Blizzard")
        .unwrap()
        .0;
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(particles)
            .is_some_and(|mut e| e.offset(Vec3::X, None))
    );
    for i in [0, nif.blocks().count(), usize::MAX] {
        assert!(
            !nif.edit::<nifly::geometry::NiShape>(i)
                .is_some_and(|mut e| e.rotate(Vec3::X, None)),
            "{i}"
        );
        assert!(
            !nif.edit::<nifly::geometry::NiShape>(i)
                .is_some_and(|mut e| e.scale(Vec3::ONE, None)),
            "{i}"
        );
    }
}
