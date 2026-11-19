//! Tests for NifFile::create_shape.

use crate::common::{UPSTREAM, reload};
use std::fs;

use nifly::{
    file::{Index, NifFile},
    geometry::NiShape,
    object3d::Vec3,
};

const VERTICES: [Vec3; 4] = [
    Vec3::new(0.0, 0.0, 0.0),
    Vec3::new(1.0, 0.0, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
    Vec3::new(1.0, 1.0, 0.0),
];
const TRIANGLES: [[u16; 3]; 2] = [[0, 1, 2], [2, 1, 3]];
const UVS: [[f32; 2]; 4] = [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]];
const NORMALS: [Vec3; 4] = [Vec3::Z; 4];

/// Arguments to `create_shape` after the name.
type Geometry<'a> = (&'a [Vec3], &'a [[u16; 3]], Option<&'a [[f32; 2]]>, Option<&'a [Vec3]>);

fn load(path: &str) -> NifFile {
    NifFile::load(fs::read(path).unwrap().as_slice(), None).unwrap()
}

fn quad(nif: &mut NifFile) -> Option<Index> {
    nif.create_shape("Quad", &VERTICES, &TRIANGLES, Some(&UVS), Some(&NORMALS))
}

fn shape<'a>(nif: &'a NifFile, name: &str) -> &'a NiShape {
    nif.shapes().find(|s| s.name() == name).unwrap()
}

#[test]
fn creates_for_each_version() {
    for (path, block, shading) in [
        ("assets/Cube.nif", "BSTriShape", "BSLightingShaderProperty"),
        ("assets/Bones.nif", "NiTriShape", "BSLightingShaderProperty"),
        (&format!("{UPSTREAM}/TestNifFile_Static_FO4.nif"), "BSSubIndexTriShape", "BSLightingShaderProperty"),
        (&format!("{UPSTREAM}/TestNifFile_Static_MW.nif"), "NiTriShape", "NiMaterialProperty"),
    ] {
        let mut nif = load(path);
        let names: Vec<String> = nif.blocks().map(|b| b.block_name().to_string()).collect();
        let index = quad(&mut nif).unwrap();
        // New blocks go at the end.
        assert!(index >= names.len(), "{path}");
        let after: Vec<String> = nif.blocks().map(|b| b.block_name().to_string()).collect();
        assert_eq!(after[..names.len()], names[..], "{path}");

        let saved = reload(&mut nif);
        for nif in [&nif, &saved] {
            let s = shape(nif, "Quad");
            assert_eq!(s.block_name(), block, "{path}");
            assert_eq!(nif.view_of(s).unwrap().shader().unwrap().block_name(), shading, "{path}");
            assert_eq!(nif.view_of(s).and_then(|v| v.parent()).map(|p| p.index()), nif.root_node().map(|r| r.index()), "{path}");
            assert!(nif.view_of(s).unwrap().bone_list().is_empty(), "{path}");
            assert_eq!(nif.view_of(s).unwrap().vertices().unwrap(), VERTICES, "{path}");
            assert_eq!(nif.view_of(s).unwrap().triangles().unwrap(), TRIANGLES, "{path}");
            assert_eq!(nif.view_of(s).unwrap().uvs().unwrap(), UVS, "{path}");
            assert_eq!(nif.view_of(s).unwrap().normals().unwrap().len(), 4, "{path}");
        }
    }
}

#[test]
fn optional_uvs_and_normals() {
    let mut nif = load("assets/Bones.nif");
    let index = nif.create_shape("Bare", &VERTICES, &TRIANGLES, None, None).unwrap();
    let s = shape(&nif, "Bare");
    assert!(nif.view_of(s).unwrap().normals().is_none() && nif.view_of(s).unwrap().uvs().is_none());
    // Normals can be added afterwards.
    assert!(nif.edit::<nifly::geometry::NiShape>(index).is_some_and(|mut e| e.calc_normals(None)));
    assert!(nif.view_of(shape(&nif, "Bare")).unwrap().normals().unwrap().iter().all(|n| (n.z.abs() - 1.0).abs() < 1e-5));

    // A BSTriShape always has UVs.
    let mut nif = load("assets/Cube.nif");
    nif.create_shape("Bare", &VERTICES, &TRIANGLES, None, None).unwrap();
    assert_eq!(nif.view_of(shape(&nif, "Bare")).unwrap().uvs().unwrap(), [[0.0; 2]; 4]);

    // No triangles is allowed.
    assert!(nif.create_shape("Points", &VERTICES, &[], None, None).is_some());
}

#[test]
fn none_changes_nothing() {
    let mut nif = load("assets/Cube.nif");
    let count = nif.blocks().count();
    let bad: [Geometry; 4] = [
        (&VERTICES, &[[0, 1, 4]], None, None),
        (&VERTICES, &TRIANGLES, Some(&UVS[..3]), None),
        (&VERTICES, &TRIANGLES, None, Some(&NORMALS[..3])),
        (&VERTICES, &TRIANGLES, Some(&[]), None),
    ];
    for (i, (v, t, uv, n)) in bad.into_iter().enumerate() {
        assert_eq!(nif.create_shape("x", v, t, uv, n), None, "{i}");
    }
    let too_many = vec![Vec3::ZERO; 65536];
    assert_eq!(nif.create_shape("x", &too_many, &[], None, None), None);
    assert_eq!(nif.create_shape("x", &VERTICES, &vec![[0, 1, 2]; 65536], None, None), None);
    assert_eq!(nif.blocks().count(), count);

    // Versions it can't build shapes for.
    for file in ["TestNifFile_Skinned_OB", "TestNifFile_SF"] {
        let mut nif = load(&format!("{UPSTREAM}/{file}.nif"));
        let count = nif.blocks().count();
        assert_eq!(quad(&mut nif), None, "{file}");
        assert_eq!(nif.blocks().count(), count, "{file}");
    }
}
