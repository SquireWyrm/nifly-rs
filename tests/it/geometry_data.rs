//! Tests for NifFile::geometry_data(_mut) and NiGeometryData.

use crate::common::{load, upstream, reload};
use std::{io::Cursor, ptr};

use nifly::{
    file::{Index, NifFile, SaveOptions},
    geometry::{BsGeometryMeshData, NiGeometryData, NiShape, NiTriBasedGeomData, NiTriShapeData},
    object3d::{BoundingSphere, Vec3},
    objects::NiObject,
};

fn shape_index(nif: &NifFile, name: &str) -> Index {
    nif.shapes().indexed().find(|(_, s)| s.name() == name).unwrap().0
}

fn data<'a>(nif: &'a NifFile, name: &str) -> &'a NiGeometryData {
    let shape = nif.shapes().find(|s| s.name() == name).unwrap();
    nif.view_of(shape).unwrap().geometry_data().unwrap()
}

/// (vertices, triangles, has vertices, normals, colors, UVs, tangents)
fn summary(d: &NiGeometryData) -> (usize, usize, bool, bool, bool, bool, bool) {
    (
        d.vertex_count(),
        d.triangle_count(),
        d.has_vertices(),
        d.has_normals(),
        d.has_vertex_colors(),
        d.has_uvs(),
        d.has_tangents(),
    )
}

#[test]
fn tri_shape_data_block() {
    let nif = load("Bones");
    let d = data(&nif, "Limb");
    // The shape's data ref, an NiTriShapeData block.
    let shape = nif.shapes().find(|s| s.name() == "Limb").unwrap();
    let index = nif.block_index(d).unwrap();
    assert_eq!(index, 5);
    assert!(nif.child_blocks(shape).indices().any(|i| i == index));
    assert!(d.downcast::<NiTriShapeData>().is_some());
    assert!(d.downcast::<NiTriBasedGeomData>().is_some());
    assert_eq!(summary(d), (164, 252, true, true, false, true, true));
    assert_eq!(d.bounds(), BoundingSphere::new(Vec3::new(0.0, 0.0, 1.0), 1.299038));

    // Older files: vertex colors (Skyrim LE) and no UVs (Morrowind).
    let nif = upstream("TestNifFile_Animated_LE");
    assert_eq!(summary(data(&nif, "Low02:0")), (10, 4, true, true, true, true, true));
    let nif = upstream("TestNifFile_Static_MW");
    assert_eq!(summary(data(&nif, "Tri EditorMarker_box_02")), (4, 2, true, true, false, false, false));
}

#[test]
fn bounds_setters_round_trip() {
    let mut nif = upstream("TestNifFile_Static_MW");
    let name = "Tri EditorMarker_box_02";
    let shape = shape_index(&nif, name);
    let stored = data(&nif, name).bounds();
    assert_eq!(stored.radius, 9354.119);

    let bounds = BoundingSphere::new(Vec3::new(1.0, 2.0, 3.0), 4.0);
    nif.edit::<nifly::geometry::NiShape>(shape).unwrap().geometry_data().unwrap().set_bounds(bounds);
    assert_eq!(data(&nif, name).bounds(), bounds);

    // Saving without optimizing keeps them.
    let mut cur = Cursor::new(Vec::new());
    nif.save(&mut cur, Some(SaveOptions { optimize: false, sort_blocks: true })).unwrap();
    let mut kept = NifFile::load(cur.into_inner().as_slice(), None).unwrap();
    assert_eq!(data(&kept, name).bounds(), bounds);

    // Recalculating from the vertices gives the stored sphere back.
    let shape = shape_index(&kept, name);
    kept.edit::<nifly::geometry::NiShape>(shape).unwrap().geometry_data().unwrap().update_bounds();
    assert_close(data(&kept, name).bounds(), stored);

    // A default save recalculates them too.
    nif.edit::<nifly::geometry::NiShape>(shape_index(&nif, name)).unwrap().geometry_data().unwrap().set_bounds(bounds);
    assert_close(data(&reload(&mut nif), name).bounds(), stored);
}

fn assert_close(a: BoundingSphere, b: BoundingSphere) {
    let close = |x: f32, y: f32| (x - y).abs() < 0.01;
    assert!(
        close(a.radius, b.radius) && close(a.center.x, b.center.x) && close(a.center.y, b.center.y) && close(a.center.z, b.center.z),
        "{a:?} != {b:?}"
    );
}

#[test]
fn bs_geometry_mesh_isnt_a_block() {
    let nif = upstream("TestNifFile_SF");
    let shape = nif.shapes().next().unwrap();
    assert_eq!(shape.block_name(), "BSGeometry");
    let d = nif.view_of(shape).unwrap().geometry_data().unwrap();
    assert!(d.downcast::<BsGeometryMeshData>().is_some());
    assert_eq!(nif.block_index(d), None);
}

#[test]
fn none_for_other_shapes() {
    // BSTriShapes keep their vertices themselves.
    for asset in ["Cone", "Cube", "Icosphere", "Suzanne"] {
        let nif = load(asset);
        assert!(nif.shapes().all(|s| nif.view_of(s).unwrap().geometry_data().is_none()), "{asset}");
    }
    // Particles aren't triangle-based.
    let nif = upstream("TestNifFile_Particles_MW");
    let particles = nif.shapes().find(|s| s.name() == "Blizzard").unwrap();
    assert_eq!(particles.block_name(), "NiAutoNormalParticles");
    assert!(nif.view_of(particles).unwrap().geometry_data().is_none());

    let nif = load("Bones");
    let count = nif.blocks().count();
    for index in [0, 5, count, usize::MAX] {
        assert!(nif.view::<nifly::geometry::NiShape>(index).and_then(|v| v.geometry_data()).is_none(), "{index}");
    }
}

#[test]
fn mut_is_the_same_data() {
    let mut nif = upstream("TestNifFile_Skinned_OB");
    let shape = shape_index(&nif, "Cuirass");
    let shared: *const NiGeometryData = nif.view_of(nif.block_of::<NiShape>(shape).unwrap()).unwrap().geometry_data().unwrap();
    let mut pinned_shape = nif.edit::<nifly::geometry::NiShape>(shape).unwrap();
    let pinned = pinned_shape.geometry_data().unwrap();
    assert!(ptr::eq(&*pinned, shared));
    let block: &NiObject = data(&nif, "Cuirass");
    assert_eq!(block.block_name(), "NiTriShapeData");
}
