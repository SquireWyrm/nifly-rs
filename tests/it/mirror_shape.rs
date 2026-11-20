//! Tests for NifFile::mirror_shape.

use crate::common::{UPSTREAM, reload};
use std::fs;

use nifly::{
    file::{Index, NifFile},
    object3d::Vec3,
};

fn load(path: &str) -> NifFile {
    NifFile::load(fs::read(path).unwrap().as_slice(), None).unwrap()
}

/// (index, vertices, normals, triangles) of the first shape.
fn first(nif: &NifFile) -> (Index, Vec<Vec3>, Vec<Vec3>, Vec<[u16; 3]>) {
    let (i, s) = nif.shapes().indexed().next().unwrap();
    (
        i,
        nif.view_of(s).unwrap().vertices().unwrap(),
        nif.view_of(s).unwrap().normals().unwrap(),
        nif.view_of(s).unwrap().triangles().unwrap(),
    )
}

fn close(a: &[Vec3], b: &[Vec3], tolerance: f32) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x.distance(*y) <= tolerance)
}

/// Each triangle rotated to start at its smallest index, which keeps its winding.
fn canonical(tris: &[[u16; 3]]) -> Vec<[u16; 3]> {
    tris.iter()
        .map(|&t| {
            let i = (0..3).min_by_key(|&i| t[i]).unwrap();
            [t[i], t[(i + 1) % 3], t[(i + 2) % 3]]
        })
        .collect()
}

fn reversed(tris: &[[u16; 3]]) -> Vec<[u16; 3]> {
    tris.iter().map(|&[a, b, c]| [c, b, a]).collect()
}

#[test]
fn mirrors_one_axis() {
    // NiTriShapeData (Bones) and BSTriShape (Cube, normals stored at reduced precision).
    for (path, tolerance) in [("assets/Bones.nif", 0.0), ("assets/Cube.nif", 0.02)] {
        let mut nif = load(path);
        let (shape, vertices, normals, tris) = first(&nif);
        assert!(
            nif.edit::<nifly::geometry::NiShape>(shape)
                .is_some_and(|mut e| e.mirror(true, false, false))
        );

        let flip_x = |v: &Vec3| Vec3::new(-v.x, v.y, v.z);
        let saved = reload(&mut nif);
        for nif in [&nif, &saved] {
            let (_, v, n, t) = first(nif);
            assert_eq!(v, vertices.iter().map(flip_x).collect::<Vec<_>>(), "{path}");
            assert!(
                close(
                    &n,
                    &normals.iter().map(flip_x).collect::<Vec<_>>(),
                    tolerance
                ),
                "{path}"
            );
            // The winding is reversed so faces still point out.
            assert_eq!(t, reversed(&tris), "{path}");
        }

        // Mirroring back restores it.
        assert!(
            nif.edit::<nifly::geometry::NiShape>(shape)
                .is_some_and(|mut e| e.mirror(true, false, false))
        );
        let (_, v, _, t) = first(&nif);
        assert_eq!((v, t), (vertices, tris), "{path}");
    }
}

#[test]
fn even_axes_keep_winding() {
    let mut nif = load("assets/Bones.nif");
    let (shape, vertices, _, tris) = first(&nif);
    // Two axes are a rotation, so triangles keep their winding.
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.mirror(false, true, true))
    );
    let (_, v, _, t) = first(&nif);
    assert_eq!(
        v,
        vertices
            .iter()
            .map(|v| Vec3::new(v.x, -v.y, -v.z))
            .collect::<Vec<_>>()
    );
    assert_eq!(t, tris);
    // All three reverse it.
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.mirror(true, true, true))
    );
    assert_eq!(first(&nif).3, reversed(&tris));
    // None changes nothing.
    let before = first(&nif);
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.mirror(false, false, false))
    );
    assert_eq!(first(&nif), before);
}

#[test]
fn skinned_skyrim_se_needs_partition_update() {
    let mut nif = load(&format!("{UPSTREAM}/TestNifFile_Skinned_SE.nif"));
    let (shape, _, _, tris) = first(&nif);
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.mirror(true, false, false))
    );
    assert_eq!(first(&nif).3, reversed(&tris));
    // The partitions keep the old winding, and they're what's saved.
    assert_eq!(first(&reload(&mut nif)).3, tris);
    // Rebuilding them keeps it (the partitions may start each triangle at another corner).
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.update_skin_partitions())
    );
    assert_eq!(
        canonical(&first(&reload(&mut nif)).3),
        canonical(&reversed(&tris))
    );
}

#[test]
fn strips_only_with_even_axes() {
    let mut nif = load("assets/Strips_MW.nif");
    let (shape, vertices, _, tris) = first(&nif);
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.mirror(true, false, false))
    );
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.mirror(true, true, true))
    );
    assert_eq!(first(&nif).1, vertices);
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.mirror(true, true, false))
    );
    let (_, v, _, t) = first(&nif);
    assert_eq!(
        v,
        vertices
            .iter()
            .map(|v| Vec3::new(-v.x, -v.y, v.z))
            .collect::<Vec<_>>()
    );
    assert_eq!(t, tris);
}

#[test]
fn false_without_vertices() {
    let mut nif = load(&format!("{UPSTREAM}/TestNifFile_Particles_MW.nif"));
    let particles = nif
        .shapes()
        .indexed()
        .find(|(_, s)| s.name() == "Blizzard")
        .unwrap()
        .0;
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(particles)
            .is_some_and(|mut e| e.mirror(true, false, false))
    );
    for i in [0, nif.blocks().count(), usize::MAX] {
        assert!(
            !nif.edit::<nifly::geometry::NiShape>(i)
                .is_some_and(|mut e| e.mirror(true, false, false)),
            "{i}"
        );
    }
}
