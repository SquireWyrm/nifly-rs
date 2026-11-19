//! Tests for NifFile::calc_shape_tangents.

use crate::common::{load, upstream, reload};

use nifly::{
    file::{Index, NifFile},
    geometry::NiShape,
    object3d::Vec3,
};

fn shape<'a>(nif: &'a NifFile, name: &str) -> &'a NiShape {
    nif.shapes().find(|s| s.name() == name).unwrap()
}

fn index(nif: &NifFile, name: &str) -> Index {
    nif.shapes().indexed().find(|(_, s)| s.name() == name).unwrap().0
}

/// (tangents, bitangents) of the shape named `name`.
fn frame(nif: &NifFile, name: &str) -> (Vec<Vec3>, Vec<Vec3>) {
    let s = shape(nif, name);
    (nif.view_of(s).unwrap().tangents().unwrap(), nif.view_of(s).unwrap().bitangents().unwrap())
}

/// Tangents calculated on a fresh copy of the file.
fn calculated(file: fn() -> NifFile, name: &str) -> (Vec<Vec3>, Vec<Vec3>) {
    let mut nif = file();
    assert!(nif.edit::<nifly::geometry::NiShape>(index(&nif, name)).is_some_and(|mut e| e.calc_tangents()));
    frame(&nif, name)
}

/// Checks that tangents are unit length and perpendicular to the normals, and returns how many
/// bitangents aren't unit length and perpendicular to both, all within `tol`.
fn check_frame(nif: &NifFile, name: &str, tol: f32) -> usize {
    let normals = nif.view_of(shape(nif, name)).unwrap().normals().unwrap();
    let (tangents, bitangents) = frame(nif, name);
    let mut skewed = 0;
    for ((n, t), b) in normals.iter().zip(&tangents).zip(&bitangents) {
        assert!((t.length() - 1.0).abs() < tol && t.dot(*n).abs() < tol, "{n:?} {t:?}");
        if (b.length() - 1.0).abs() >= tol || b.dot(*n).abs() >= tol || b.dot(*t).abs() >= tol {
            skewed += 1;
        }
    }
    skewed
}

fn cuirass() -> NifFile {
    upstream("TestNifFile_Skinned_OB")
}

fn cube() -> NifFile {
    load("Cube")
}

#[test]
fn recalculates() {
    // NiTriShapeData: overwrite them, then calculate them back.
    let mut nif = cuirass();
    let shape = index(&nif, "Cuirass");
    let expected = calculated(cuirass, "Cuirass");
    assert!(nif.edit::<nifly::geometry::NiShape>(shape).is_some_and(|mut e| e.set_tangents(&vec![Vec3::X; 1354])));
    assert!(nif.edit::<nifly::geometry::NiShape>(shape).is_some_and(|mut e| e.set_bitangents(&vec![Vec3::X; 1354])));
    assert!(nif.edit::<nifly::geometry::NiShape>(shape).is_some_and(|mut e| e.calc_tangents()));
    assert_eq!(frame(&nif, "Cuirass"), expected);
    // Where the u direction is nearly parallel to the normal, the bitangent can end up skewed or
    // zero.
    let skewed = check_frame(&nif, "Cuirass", 1e-3);
    assert!(skewed * 50 < 1354, "{skewed}");
    assert_eq!(frame(&reload(&mut nif), "Cuirass"), expected);

    // BSTriShape, stored at reduced precision.
    let mut nif = cube();
    let shape = index(&nif, "Cube");
    let expected = calculated(cube, "Cube");
    assert!(nif.edit::<nifly::geometry::NiShape>(shape).is_some_and(|mut e| e.set_tangents(&[Vec3::X; 14])));
    assert!(nif.edit::<nifly::geometry::NiShape>(shape).is_some_and(|mut e| e.calc_tangents()));
    assert_eq!(frame(&nif, "Cube"), expected);
    assert_eq!(check_frame(&nif, "Cube", 0.05), 0);
}

#[test]
fn adds_them_along_the_uvs() {
    // A flat Morrowind quad with normals and UVs but no tangents.
    let mut nif = upstream("TestNifFile_Billboard_MW");
    let name = "Tri BM_Snow_01";
    let s = shape(&nif, name);
    assert!(nif.view_of(s).unwrap().tangents().is_none());
    let (v, uv) = (nif.view_of(s).unwrap().vertices().unwrap(), nif.view_of(s).unwrap().uvs().unwrap());
    let [a, b, c] = nif.view_of(s).unwrap().triangles().unwrap()[0].map(usize::from);

    assert!(nif.edit::<nifly::geometry::NiShape>(index(&nif, name)).is_some_and(|mut e| e.calc_tangents()));
    assert_eq!(check_frame(&nif, name, 1e-4), 0);

    // Solve the first triangle for the directions of increasing u and v.
    let (e1, e2) = (v[b] - v[a], v[c] - v[a]);
    let (s1, s2, t1, t2) = (uv[b][0] - uv[a][0], uv[c][0] - uv[a][0], uv[b][1] - uv[a][1], uv[c][1] - uv[a][1]);
    let det = s1 * t2 - s2 * t1;
    let along_u = ((e1 * t2 - e2 * t1) / det).normalize();
    let along_v = ((e2 * s1 - e1 * s2) / det).normalize();
    let (tangents, bitangents) = frame(&nif, name);
    for (t, b) in tangents.iter().zip(&bitangents) {
        assert!(t.dot(along_v) > 0.999, "{t:?} vs {along_v:?}");
        assert!(b.dot(along_u) > 0.999, "{b:?} vs {along_u:?}");
    }
}

#[test]
fn false_without_normals_and_uvs() {
    // Morrowind's editor marker has normals but no UVs.
    let mut nif = upstream("TestNifFile_Static_MW");
    let name = "Tri EditorMarker_box_02";
    assert!(nif.view_of(shape(&nif, name)).unwrap().uvs().is_none());
    assert!(!nif.edit::<nifly::geometry::NiShape>(index(&nif, name)).is_some_and(|mut e| e.calc_tangents()));
    assert!(nif.view_of(shape(&nif, name)).unwrap().tangents().is_none());

    let mut nif = upstream("TestNifFile_Particles_MW");
    assert!(!nif.edit::<nifly::geometry::NiShape>(index(&nif, "Blizzard")).is_some_and(|mut e| e.calc_tangents()));

    let mut nif = upstream("TestNifFile_SF");
    let shape = nif.shapes().indices().next().unwrap();
    assert!(!nif.edit::<nifly::geometry::NiShape>(shape).is_some_and(|mut e| e.calc_tangents()));

    let mut nif = cube();
    let count = nif.blocks().count();
    for i in [0, count, usize::MAX] {
        assert!(!nif.edit::<nifly::geometry::NiShape>(i).is_some_and(|mut e| e.calc_tangents()), "{i}");
    }
}
