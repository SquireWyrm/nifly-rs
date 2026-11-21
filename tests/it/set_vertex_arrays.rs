//! Tests for NifFile::set_shape_vertices, set_shape_normals, set_shape_uvs, set_shape_colors,
//! set_shape_tangents, set_shape_bitangents, set_shape_eye_data and set_shape_vertex.

use crate::common::{load, reload, shape, shape_index, upstream};

use nifly::{file::NifFile, object3d::Vec3};

/// `n` distinct unit vectors.
fn units(n: usize) -> Vec<Vec3> {
    (0..n)
        .map(|i| Vec3::new((i as f32).cos(), (i as f32).sin(), 0.0))
        .collect()
}

fn assert_close(a: &[Vec3], b: &[Vec3], tolerance: f32) {
    assert_eq!(a.len(), b.len());
    for (x, y) in a.iter().zip(b) {
        assert!(x.distance(*y) <= tolerance, "{x:?} != {y:?}");
    }
}

/// Every array of the shape named `name`.
type Arrays = (
    Option<Vec<Vec3>>,
    Option<Vec<Vec3>>,
    Option<Vec<[f32; 2]>>,
    Option<Vec<[f32; 4]>>,
    Option<Vec<Vec3>>,
    Option<Vec<Vec3>>,
    Option<Vec<f32>>,
);

fn arrays(nif: &NifFile, name: &str) -> Arrays {
    let s = shape(nif, name);
    (
        nif.view_of(s).unwrap().vertices(),
        nif.view_of(s).unwrap().normals(),
        nif.view_of(s).unwrap().uvs(),
        nif.view_of(s).unwrap().vertex_colors(),
        nif.view_of(s).unwrap().tangents(),
        nif.view_of(s).unwrap().bitangents(),
        nif.view_of(s).unwrap().eye_data(),
    )
}

#[test]
fn geometry_data_round_trip() {
    let mut nif = load("Bones");
    let limb = shape_index(&nif, "Limb");
    let n = 164;
    let vertices: Vec<Vec3> = nif
        .view_of(shape(&nif, "Limb"))
        .unwrap()
        .vertices()
        .unwrap()
        .iter()
        .map(|v| *v * 2.0)
        .collect();
    let normals = units(n);
    let tangents: Vec<Vec3> = normals.iter().map(|v| v.cross(Vec3::Z)).collect();
    let bitangents = vec![Vec3::Z; n];
    let uvs: Vec<[f32; 2]> = (0..n).map(|i| [i as f32 / n as f32, 0.5]).collect();
    let colors: Vec<[f32; 4]> = (0..n)
        .map(|i| [i as f32 / n as f32, 0.25, 0.5, 1.0])
        .collect();

    assert!(
        nif.edit::<nifly::geometry::NiShape>(limb)
            .is_some_and(|mut e| e.set_vertices(&vertices))
    );
    assert!(
        nif.edit::<nifly::geometry::NiShape>(limb)
            .is_some_and(|mut e| e.set_normals(&normals))
    );
    assert!(
        nif.edit::<nifly::geometry::NiShape>(limb)
            .is_some_and(|mut e| e.set_uvs(&uvs))
    );
    // Bones.nif has no vertex colors; setting them turns them on.
    assert!(
        nif.view_of(shape(&nif, "Limb"))
            .unwrap()
            .vertex_colors()
            .is_none()
    );
    assert!(
        nif.edit::<nifly::geometry::NiShape>(limb)
            .is_some_and(|mut e| e.set_vertex_colors(&colors))
    );
    assert!(
        nif.edit::<nifly::geometry::NiShape>(limb)
            .is_some_and(|mut e| e.set_tangents(&tangents))
    );
    assert!(
        nif.edit::<nifly::geometry::NiShape>(limb)
            .is_some_and(|mut e| e.set_bitangents(&bitangents))
    );

    // Stored as given.
    let expected = (
        Some(vertices),
        Some(normals),
        Some(uvs),
        Some(colors),
        Some(tangents),
        Some(bitangents),
        None,
    );
    assert_eq!(arrays(&nif, "Limb"), expected);
    assert_eq!(
        nif.view_of(shape(&nif, "Limb"))
            .unwrap()
            .geometry_data()
            .unwrap()
            .triangle_count(),
        252
    );
    assert_eq!(arrays(&reload(&mut nif), "Limb"), expected);
}

#[test]
fn turning_on_tangents_adds_bitangents() {
    // Replacing tangents a shape already has.
    let mut nif = upstream("TestNifFile_Animated_LE");
    let name = "Low02:0";
    let low = shape_index(&nif, name);
    let n = 10;
    assert!(
        nif.edit::<nifly::geometry::NiShape>(low)
            .is_some_and(|mut e| e.set_tangents(&vec![Vec3::X; n]))
    );
    assert_eq!(
        nif.view_of(shape(&nif, name)).unwrap().tangents(),
        Some(vec![Vec3::X; n])
    );

    // Morrowind's editor marker has only positions and normals.
    let mut nif = upstream("TestNifFile_Static_MW");
    let name = "Tri EditorMarker_box_02";
    let marker = shape_index(&nif, name);
    assert!(nif.view_of(shape(&nif, name)).unwrap().tangents().is_none());
    assert!(
        nif.edit::<nifly::geometry::NiShape>(marker)
            .is_some_and(|mut e| e.set_tangents(&[Vec3::X; 4]))
    );
    assert_eq!(
        nif.view_of(shape(&nif, name)).unwrap().tangents(),
        Some(vec![Vec3::X; 4])
    );
    assert_eq!(
        nif.view_of(shape(&nif, name)).unwrap().bitangents(),
        Some(vec![Vec3::ZERO; 4])
    );
}

#[test]
fn bs_tri_shape_round_trip() {
    let mut nif = load("Cube");
    let cube = shape_index(&nif, "Cube");
    let n = 14;
    let vertices: Vec<Vec3> = nif
        .view_of(shape(&nif, "Cube"))
        .unwrap()
        .vertices()
        .unwrap()
        .iter()
        .map(|v| *v * 3.0)
        .collect();
    let normals = units(n);
    let uvs: Vec<[f32; 2]> = (0..n).map(|i| [i as f32 / 16.0, 0.25]).collect();
    let colors: Vec<[f32; 4]> = (0..n).map(|i| [i as f32 / 16.0, 0.25, 0.5, 1.0]).collect();
    let eye: Vec<f32> = (0..n).map(|i| i as f32 / 2.0).collect();

    assert!(
        nif.edit::<nifly::geometry::NiShape>(cube)
            .is_some_and(|mut e| e.set_vertices(&vertices))
    );
    assert!(
        nif.edit::<nifly::geometry::NiShape>(cube)
            .is_some_and(|mut e| e.set_normals(&normals))
    );
    assert!(
        nif.edit::<nifly::geometry::NiShape>(cube)
            .is_some_and(|mut e| e.set_uvs(&uvs))
    );
    assert!(
        nif.edit::<nifly::geometry::NiShape>(cube)
            .is_some_and(|mut e| e.set_vertex_colors(&colors))
    );
    assert!(
        nif.edit::<nifly::geometry::NiShape>(cube)
            .is_some_and(|mut e| e.set_tangents(&normals))
    );
    assert!(
        nif.edit::<nifly::geometry::NiShape>(cube)
            .is_some_and(|mut e| e.set_bitangents(&normals))
    );
    assert!(
        nif.edit::<nifly::geometry::NiShape>(cube)
            .is_some_and(|mut e| e.set_eye_data(&eye))
    );

    let saved = reload(&mut nif);
    for nif in [&nif, &saved] {
        let (v, nm, uv, c, t, b, e) = arrays(nif, "Cube");
        assert_close(&v.unwrap(), &vertices, 0.01);
        assert_eq!(uv.unwrap(), uvs);
        assert_eq!(e.unwrap(), eye);
        // Reduced precision: within one step of 1/127 per component.
        for got in [nm.unwrap(), t.unwrap(), b.unwrap()] {
            assert_close(&got, &normals, 0.02);
        }
        let c = c.unwrap();
        for (got, want) in c.iter().flatten().zip(colors.iter().flatten()) {
            assert!((got - want).abs() <= 1.0 / 255.0, "{got} != {want}");
        }
    }

    // Colors are clamped.
    assert!(
        nif.edit::<nifly::geometry::NiShape>(cube)
            .is_some_and(|mut e| e.set_vertex_colors(&vec![[2.0, -1.0, 0.5, 1.0]; n]))
    );
    let c = nif
        .view_of(shape(&nif, "Cube"))
        .unwrap()
        .vertex_colors()
        .unwrap();
    assert_eq!((c[0][0], c[0][1]), (1.0, 0.0));
}

#[test]
fn false_changes_nothing() {
    let mut nif = load("Bones");
    let limb = shape_index(&nif, "Limb");
    let before = arrays(&nif, "Limb");
    // One value too few or too many.
    for n in [163, 165, 0] {
        assert!(
            !nif.edit::<nifly::geometry::NiShape>(limb)
                .is_some_and(|mut e| e.set_vertices(&vec![Vec3::ONE; n])),
            "{n}"
        );
        assert!(
            !nif.edit::<nifly::geometry::NiShape>(limb)
                .is_some_and(|mut e| e.set_normals(&vec![Vec3::Z; n])),
            "{n}"
        );
        assert!(
            !nif.edit::<nifly::geometry::NiShape>(limb)
                .is_some_and(|mut e| e.set_uvs(&vec![[0.0; 2]; n])),
            "{n}"
        );
        assert!(
            !nif.edit::<nifly::geometry::NiShape>(limb)
                .is_some_and(|mut e| e.set_vertex_colors(&vec![[1.0; 4]; n])),
            "{n}"
        );
        assert!(
            !nif.edit::<nifly::geometry::NiShape>(limb)
                .is_some_and(|mut e| e.set_tangents(&vec![Vec3::X; n])),
            "{n}"
        );
        assert!(
            !nif.edit::<nifly::geometry::NiShape>(limb)
                .is_some_and(|mut e| e.set_bitangents(&vec![Vec3::Y; n])),
            "{n}"
        );
    }
    // Eye data only exists on BSTriShapes.
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(limb)
            .is_some_and(|mut e| e.set_eye_data(&[0.0; 164]))
    );
    assert_eq!(arrays(&nif, "Limb"), before);
    assert_eq!(
        nif.view_of(shape(&nif, "Limb"))
            .unwrap()
            .geometry_data()
            .unwrap()
            .triangle_count(),
        252
    );

    // Not a shape, or out of range.
    let count = nif.blocks().count();
    for i in [0, 5, count, usize::MAX] {
        assert!(
            !nif.edit::<nifly::geometry::NiShape>(i)
                .is_some_and(|mut e| e.set_vertices(&[])),
            "{i}"
        );
        assert!(
            !nif.edit::<nifly::geometry::NiShape>(i)
                .is_some_and(|mut e| e.set_eye_data(&[])),
            "{i}"
        );
    }

    // Particles store no settable vertex data.
    let mut nif = upstream("TestNifFile_Particles_MW");
    let particles = shape_index(&nif, "Blizzard");
    for len in [0, 1, 40] {
        assert!(
            !nif.edit::<nifly::geometry::NiShape>(particles)
                .is_some_and(|mut e| e.set_vertices(&vec![Vec3::ZERO; len]))
        );
    }
}

#[test]
fn oblivion_tangents_round_trip_through_extra_data() {
    let blob = |nif: &NifFile| {
        nif.shapes()
            .views()
            .next()
            .unwrap()
            .extra_data()
            .filter_map(|b| b.downcast::<nifly::extra_data::NiExtraData>())
            .filter(|e| e.name() == "Tangent space (binormal & tangent vectors)")
            .count()
    };
    let mut nif = upstream("TestNifFile_Skinned_OB");
    assert_eq!(blob(&nif), 1);
    let cuirass = shape_index(&nif, "Cuirass");
    let n = nif
        .view_of(shape(&nif, "Cuirass"))
        .unwrap()
        .tangents()
        .unwrap()
        .len();
    assert_eq!(n, 1354);
    assert!(
        nif.edit::<nifly::geometry::NiShape>(cuirass)
            .is_some_and(|mut e| e.set_tangents(&vec![Vec3::X; n]))
    );
    assert!(
        nif.edit::<nifly::geometry::NiShape>(cuirass)
            .is_some_and(|mut e| e.set_bitangents(&vec![Vec3::Y; n]))
    );

    let saved = reload(&mut nif);
    assert_eq!(blob(&saved), 1);
    assert_eq!(
        saved
            .view_of(shape(&saved, "Cuirass"))
            .unwrap()
            .tangents()
            .unwrap(),
        vec![Vec3::X; n]
    );
    assert_eq!(
        saved
            .view_of(shape(&saved, "Cuirass"))
            .unwrap()
            .bitangents()
            .unwrap(),
        vec![Vec3::Y; n]
    );
}

#[test]
fn moves_one_vertex() {
    // NiTriShapeData and BSTriShape.
    for (asset, name) in [("Bones", "Limb"), ("Cube", "Cube")] {
        let mut nif = load(asset);
        let i = shape_index(&nif, name);
        let mut expected = nif.view_of(shape(&nif, name)).unwrap().vertices().unwrap();
        let last = expected.len() - 1;
        for v in [0, last] {
            assert!(
                nif.edit::<nifly::geometry::NiShape>(i)
                    .is_some_and(|mut e| e.set_vertex(v, Vec3::new(5.0, -6.0, 7.5))),
                "{asset} {v}"
            );
            expected[v] = Vec3::new(5.0, -6.0, 7.5);
        }
        assert_eq!(
            nif.view_of(shape(&nif, name)).unwrap().vertices().unwrap(),
            expected,
            "{asset}"
        );
        let saved = reload(&mut nif);
        assert_eq!(
            saved
                .view_of(shape(&saved, name))
                .unwrap()
                .vertices()
                .unwrap(),
            expected,
            "{asset}"
        );

        // Past the last vertex.
        for v in [last + 1, usize::MAX] {
            assert!(
                !nif.edit::<nifly::geometry::NiShape>(i)
                    .is_some_and(|mut e| e.set_vertex(v, Vec3::ZERO)),
                "{asset} {v}"
            );
        }
        assert_eq!(
            nif.view_of(shape(&nif, name)).unwrap().vertices().unwrap(),
            expected,
            "{asset}"
        );
    }

    // No vertices to move: particles, not a shape, out of range.
    let mut nif = upstream("TestNifFile_Particles_MW");
    let particles = shape_index(&nif, "Blizzard");
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(particles)
            .is_some_and(|mut e| e.set_vertex(0, Vec3::ZERO))
    );
    for i in [0, nif.blocks().count(), usize::MAX] {
        assert!(
            !nif.edit::<nifly::geometry::NiShape>(i)
                .is_some_and(|mut e| e.set_vertex(0, Vec3::ZERO)),
            "{i}"
        );
    }
}
