//! Tests for NifFile::shape_vertices, shape_normals, shape_uvs, shape_colors, shape_tangents,
//! shape_bitangents and shape_eye_data.

use crate::common::{load, shape, upstream};

use nifly::{file::NifFile, geometry::NiShape, object3d::Vec3};

/// The length of each array, `None` for ones the shape doesn't store.
fn lengths(nif: &NifFile, s: &NiShape) -> [Option<usize>; 7] {
    [
        nif.view_of(s).unwrap().vertices().map(|v| v.len()),
        nif.view_of(s).unwrap().normals().map(|v| v.len()),
        nif.view_of(s).unwrap().uvs().map(|v| v.len()),
        nif.view_of(s).unwrap().vertex_colors().map(|v| v.len()),
        nif.view_of(s).unwrap().tangents().map(|v| v.len()),
        nif.view_of(s).unwrap().bitangents().map(|v| v.len()),
        nif.view_of(s).unwrap().eye_data().map(|v| v.len()),
    ]
}

#[test]
fn from_geometry_data() {
    let nif = load("Bones");
    let limb = shape(&nif, "Limb");
    let n = Some(164);
    assert_eq!(lengths(&nif, limb), [n, n, n, None, n, n, None]);
    assert_eq!(
        nif.view_of(limb)
            .unwrap()
            .geometry_data()
            .unwrap()
            .vertex_count(),
        164
    );

    assert_eq!(
        nif.view_of(limb).unwrap().vertices().unwrap()[0],
        Vec3::new(0.25, 0.25, 2.25)
    );
    let normal = nif.view_of(limb).unwrap().normals().unwrap()[0];
    assert!((normal.x - 0.57735026).abs() < 1e-6 && normal.x == normal.y && normal.y == normal.z);
    assert_eq!(nif.view_of(limb).unwrap().uvs().unwrap()[0], [0.625, 0.5]);

    // Vertex colors (Skyrim LE); no UVs, colors or tangents (Morrowind).
    let nif = upstream("TestNifFile_Animated_LE");
    let colors = nif
        .view_of(shape(&nif, "Low02:0"))
        .unwrap()
        .vertex_colors()
        .unwrap();
    assert_eq!(colors.len(), 10);
    assert_eq!(colors[0], [1.0, 1.0, 1.0, 1.0]);
    let nif = upstream("TestNifFile_Static_MW");
    let n = Some(4);
    assert_eq!(
        lengths(&nif, shape(&nif, "Tri EditorMarker_box_02")),
        [n, n, None, None, None, None, None]
    );
}

#[test]
fn from_bs_tri_shape() {
    let nif = load("Cube");
    let cube = shape(&nif, "Cube");
    assert_eq!(cube.block_name(), "BSTriShape");
    let n = Some(14);
    assert_eq!(lengths(&nif, cube), [n, n, n, None, n, n, None]);
    assert_eq!(
        nif.view_of(cube).unwrap().vertices().unwrap()[0],
        Vec3::new(1.0, 1.0, 1.0)
    );
    assert_eq!(nif.view_of(cube).unwrap().uvs().unwrap()[0], [0.625, 0.5]);

    // Stored at reduced precision, so only nearly unit length.
    for v in [
        nif.view_of(cube).unwrap().normals(),
        nif.view_of(cube).unwrap().tangents(),
    ]
    .into_iter()
    .flatten()
    .flatten()
    {
        assert!((v.length() - 1.0).abs() < 0.02, "{v:?}");
    }
    let normal = nif.view_of(cube).unwrap().normals().unwrap()[0];
    assert!((normal.x - 0.57735026).abs() < 0.01, "{normal:?}");

    // Suzanne has vertex colors.
    let nif = load("Suzanne");
    let colors = nif
        .view_of(shape(&nif, "Suzanne:6"))
        .unwrap()
        .vertex_colors()
        .unwrap();
    assert_eq!(colors.len(), 4567);
    assert!(colors.iter().flatten().all(|c| (0.0..=1.0).contains(c)));
}

#[test]
fn one_value_per_vertex() {
    let files = [
        load("Bones"),
        load("Cone"),
        load("Icosphere"),
        load("Suzanne"),
        upstream("TestNifFile_Animated_LE"),
        upstream("TestNifFile_FO76"),
        upstream("TestNifFile_Skinned_Dynamic_SE"),
        upstream("TestNifFile_Skinned_OB"),
        upstream("TestNifFile_Static_FO4"),
    ];
    for nif in &files {
        for s in nif.shapes() {
            let [vertices, rest @ ..] = lengths(nif, s);
            let vertices = vertices.unwrap();
            assert!(vertices > 0, "{}", s.name());
            assert!(
                rest.iter().flatten().all(|n| *n == vertices),
                "{}",
                s.name()
            );
        }
    }
}

#[test]
fn none_without_data() {
    // Particles aren't triangle-based.
    let nif = upstream("TestNifFile_Particles_MW");
    assert_eq!(lengths(&nif, shape(&nif, "Blizzard")), [None; 7]);

    // Starfield geometry without its mesh file loaded has no vertices.
    let nif = upstream("TestNifFile_SF");
    let geometry = nif.shapes().next().unwrap();
    assert_eq!(
        lengths(&nif, geometry),
        [Some(0), None, None, None, None, None, None]
    );
}
