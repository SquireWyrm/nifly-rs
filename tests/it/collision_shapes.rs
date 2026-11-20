//! Tests for collision shapes: primitives, containers, MOPP and mesh shapes, and creating them.

use nifly::{
    collision::{
        BhkBoxShape, BhkCapsuleShape, BhkCollisionObject, BhkCompressedMeshShape, BhkCompressedMeshShapeData,
        BhkConvexListShape, BhkConvexShape, BhkConvexTransformShape, BhkConvexVerticesShape, BhkCylinderShape,
        BhkListShape, BhkMoppBvTreeShape, BhkMultiSphereShape, BhkNiTriStripsShape, BhkPackedNiTriStripsShape,
        BhkPlaneShape, BhkRigidBody, BhkSphereShape, BhkTransformShape, BigTriangle, CInfoProperty, ChunkMaterial,
        ChunkTransform, HavokFilter, HkPackedNiTriStripsData, MeshChunk, PackedTriangle, SubPart,
    },
    effects::Plane,
    file::{Index, NifFile, NifVersion},
    geometry::NiTriStripsData,
    object3d::{BoundingSphere, Vec3},
};

use crate::common::{load, reload_in_place as reload};

/// A new file of `version` whose root has a collision object with a new rigid body.
fn with_body(version: NifVersion) -> (NifFile, Index) {
    let mut nif = NifFile::create(version);
    let collision = nif.root_node_mut().unwrap().assign_collision::<BhkCollisionObject, _>();
    let body = nif.edit::<BhkCollisionObject>(collision).unwrap().assign_body::<BhkRigidBody, _>().unwrap();
    (nif, body)
}

#[test]
fn convex_shapes() {
    let (mut nif, body) = with_body(NifVersion::SKYRIM_SE);
    let list = nif.edit::<BhkRigidBody>(body).unwrap().assign_shape::<BhkListShape>();
    let mut edit = nif.edit::<BhkListShape>(list).unwrap();
    let cube = edit.add_sub_shape::<BhkBoxShape>().unwrap();
    let sphere = edit.add_sub_shape::<BhkSphereShape>().unwrap();
    let capsule = edit.add_sub_shape::<BhkCapsuleShape>().unwrap();
    let cylinder = edit.add_sub_shape::<BhkCylinderShape>().unwrap();
    let multi = edit.add_sub_shape::<BhkMultiSphereShape>().unwrap();
    let hull = edit.add_sub_shape::<BhkConvexVerticesShape>().unwrap();

    let mut b = nif.block_of_mut::<BhkBoxShape>(cube).unwrap();
    b.as_mut().set_dimensions(Vec3::new(1.0, 2.0, 3.0));
    let mut convex = b.upcast_mut();
    convex.as_mut().set_radius(0.1);
    convex.upcast_mut().set_material(42);
    nif.block_of_mut::<BhkConvexShape>(sphere).unwrap().set_radius(0.5);
    let mut c = nif.block_of_mut::<BhkCapsuleShape>(capsule).unwrap();
    c.as_mut().set_point_1(Vec3::new(0.0, 0.0, -1.0));
    c.as_mut().set_point_2(Vec3::new(0.0, 0.0, 1.0));
    c.as_mut().set_radius_1(0.25);
    c.set_radius_2(0.5);
    let mut c = nif.block_of_mut::<BhkCylinderShape>(cylinder).unwrap();
    c.as_mut().set_vertex_a([0.0, 0.0, -2.0, 0.0]);
    c.as_mut().set_vertex_b([0.0, 0.0, 2.0, 0.0]);
    c.set_cylinder_radius(0.75);
    let spheres = [BoundingSphere::new(Vec3::new(1.0, 0.0, 0.0), 0.5), BoundingSphere::new(Vec3::ZERO, 1.0)];
    let mut m = nif.block_of_mut::<BhkMultiSphereShape>(multi).unwrap();
    m.as_mut().set_spheres(&spheres);
    m.set_shape_property(CInfoProperty { data: 1, size: 2, capacity_and_flags: 3 });
    let vertices = [[0.0, 0.0, 0.0, 0.0], [1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0]];
    let normals = [[0.0, 0.0, 1.0, -0.5]];
    let mut h = nif.block_of_mut::<BhkConvexVerticesShape>(hull).unwrap();
    h.as_mut().set_vertices(&vertices);
    h.as_mut().set_normals(&normals);
    h.set_vertices_property(CInfoProperty { data: 4, size: 5, capacity_and_flags: 6 });

    let saved = reload(&mut nif);
    let shapes: Vec<_> = saved.view::<BhkListShape>(list).unwrap().sub_shapes().indices().collect();
    assert_eq!(shapes, [cube, sphere, capsule, cylinder, multi, hull]);
    let b = saved.block_of::<BhkBoxShape>(cube).unwrap();
    assert_eq!((b.dimensions(), b.radius(), b.material()), (Vec3::new(1.0, 2.0, 3.0), 0.1, 42));
    assert_eq!(saved.block_of::<BhkConvexShape>(sphere).unwrap().radius(), 0.5);
    let c = saved.block_of::<BhkCapsuleShape>(capsule).unwrap();
    assert_eq!((c.point_1(), c.point_2()), (Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 0.0, 1.0)));
    assert_eq!((c.radius_1(), c.radius_2()), (0.25, 0.5));
    let c = saved.block_of::<BhkCylinderShape>(cylinder).unwrap();
    assert_eq!((c.vertex_a()[2], c.vertex_b()[2], c.cylinder_radius()), (-2.0, 2.0, 0.75));
    let m = saved.block_of::<BhkMultiSphereShape>(multi).unwrap();
    assert_eq!((m.spheres(), m.shape_property().size), (spheres.to_vec(), 2));
    let h = saved.block_of::<BhkConvexVerticesShape>(hull).unwrap();
    assert_eq!((h.vertices(), h.normals()), (vertices.to_vec(), normals.to_vec()));
    assert_eq!(h.vertices_property().capacity_and_flags, 6);
}

#[test]
fn plane_shape() {
    let (mut nif, body) = with_body(NifVersion::SKYRIM_SE);
    let plane = nif.edit::<BhkRigidBody>(body).unwrap().assign_shape::<BhkPlaneShape>();
    let value = Plane { normal: Vec3::new(0.0, 0.0, 1.0), constant: 2.0 };
    let mut p = nif.block_of_mut::<BhkPlaneShape>(plane).unwrap();
    p.as_mut().set_plane(value);
    p.as_mut().set_unknown_vector(Vec3::new(1.0, 2.0, 3.0));
    p.as_mut().set_half_extents([5.0, 5.0, 0.0, 0.0]);
    p.as_mut().set_center([1.0, 0.0, 0.0, 0.0]);
    p.upcast_mut().set_material(7);

    let saved = reload(&mut nif);
    let p = saved.block_of::<BhkPlaneShape>(plane).unwrap();
    assert_eq!((p.plane(), p.unknown_vector()), (value, Vec3::new(1.0, 2.0, 3.0)));
    assert_eq!((p.half_extents(), p.center(), p.material()), ([5.0, 5.0, 0.0, 0.0], [1.0, 0.0, 0.0, 0.0], 7));
}

#[test]
fn list_shapes() {
    let (mut nif, body) = with_body(NifVersion::SKYRIM_SE);
    let list = nif.edit::<BhkRigidBody>(body).unwrap().assign_shape::<BhkListShape>();
    let mut edit = nif.edit::<BhkListShape>(list).unwrap();
    let cube = edit.add_sub_shape::<BhkBoxShape>().unwrap();
    let convex = edit.add_sub_shape::<BhkConvexListShape>().unwrap();
    let filter = HavokFilter { layer: 9, flags_and_parts: 0, group: 1 };
    assert_eq!(edit.block().filters(), [HavokFilter::default(); 2], "one default filter per added shape");
    assert!(!edit.block().set_filters(&[filter]), "not one per sub-shape");
    assert!(edit.block().set_filters(&[filter, filter]));
    assert!(!edit.set_sub_shapes(&[list]), "contains itself");
    assert!(!edit.set_sub_shapes(&[body]), "not a shape");
    assert!(edit.set_sub_shapes(&[convex]));
    assert_eq!(edit.block().filters(), [filter], "resized with the sub-shapes");

    let mut edit = nif.edit::<BhkConvexListShape>(convex).unwrap();
    assert_eq!(edit.add_sub_shape::<BhkListShape>(), None, "not convex");
    assert!(!edit.set_sub_shapes(&[list]), "not convex");
    let sphere = edit.add_sub_shape::<BhkSphereShape>().unwrap();
    assert!(edit.set_sub_shapes(&[cube, sphere]));
    let mut c = edit.block();
    c.as_mut().set_material(3);
    c.as_mut().set_radius(0.05);
    c.as_mut().set_unknown_int(4);
    c.as_mut().set_unknown_float(0.5);
    c.as_mut().set_use_cached_aabb(true);
    c.as_mut().set_closest_point_min_distance(0.25);
    c.set_child_shape_property(CInfoProperty { data: 0, size: 0, capacity_and_flags: 7 });
    let mut l = nif.block_of_mut::<BhkListShape>(list).unwrap();
    l.as_mut().set_material(11);
    l.as_mut().set_child_shape_property(CInfoProperty { data: 1, size: 0, capacity_and_flags: 0 });
    l.set_child_filter_property(CInfoProperty { data: 2, size: 0, capacity_and_flags: 0 });

    let saved = reload(&mut nif);
    let l = saved.view::<BhkListShape>(list).unwrap();
    assert_eq!(l.sub_shapes().indices().collect::<Vec<_>>(), [convex]);
    assert_eq!((l.filters(), l.material()), (vec![filter], 11));
    assert_eq!((l.child_shape_property().data, l.child_filter_property().data), (1, 2));
    let c = saved.view::<BhkConvexListShape>(convex).unwrap();
    assert_eq!(c.sub_shapes().indices().collect::<Vec<_>>(), [cube, sphere]);
    assert_eq!((c.material(), c.radius(), c.unknown_int(), c.unknown_float()), (3, 0.05, 4, 0.5));
    assert_eq!((c.use_cached_aabb(), c.closest_point_min_distance()), (true, 0.25));
    assert_eq!(c.child_shape_property().capacity_and_flags, 7);
}

#[test]
fn transform_and_mopp() {
    let (mut nif, body) = with_body(NifVersion::SKYRIM_SE);
    let mopp = nif.edit::<BhkRigidBody>(body).unwrap().assign_shape::<BhkMoppBvTreeShape>();
    let transform = nif.edit::<BhkMoppBvTreeShape>(mopp).unwrap().assign_shape::<BhkConvexTransformShape>();
    let mut edit = nif.edit::<BhkConvexTransformShape>(transform).unwrap();
    let cube = edit.assign_shape::<BhkBoxShape>();
    assert!(!edit.set_shape(Some(mopp)), "contains the transform");
    assert!(!edit.set_shape(Some(transform)), "itself");
    assert_eq!(edit.shape().unwrap().index(), cube);

    let matrix = [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [1.0, 2.0, 3.0, 1.0]];
    let mut t = nif.block_of_mut::<BhkTransformShape>(transform).unwrap();
    t.as_mut().set_transform(matrix);
    t.as_mut().set_material(5);
    t.set_radius(0.01);
    let mut m = nif.block_of_mut::<BhkMoppBvTreeShape>(mopp).unwrap();
    m.as_mut().set_mopp_code(&[1, 2, 3]);
    m.as_mut().set_mopp_offset([1.0, 2.0, 3.0, 0.0]);
    m.as_mut().set_mopp_scale(4.0);
    m.set_build_type(1);

    let saved = reload(&mut nif);
    let t = saved.view::<BhkTransformShape>(transform).unwrap();
    assert_eq!(t.shape().unwrap().index(), cube);
    assert_eq!((t.transform(), t.material(), t.radius()), (matrix, 5, 0.01));
    let m = saved.view::<BhkMoppBvTreeShape>(mopp).unwrap();
    assert_eq!(m.shape().unwrap().index(), transform);
    assert_eq!((m.mopp_code(), m.mopp_offset(), m.mopp_scale(), m.build_type()), (&[1, 2, 3][..], [1.0, 2.0, 3.0, 0.0], 4.0, 1));
}

#[test]
fn compressed_mesh() {
    // Suzanne.nif: a MOPP shape over a compressed mesh shape.
    let mut nif = load("Suzanne");
    let shape = nif.blocks_of::<BhkCompressedMeshShape>().indices().next().unwrap();
    let view = nif.view::<BhkCompressedMeshShape>(shape).unwrap();
    let data = view.compressed_data().unwrap().index();
    assert!(view.mesh_target().is_some());
    let d = nif.block_of::<BhkCompressedMeshShapeData>(data).unwrap();
    let chunks = d.chunks();
    assert!(!chunks.is_empty() && chunks.iter().all(|c| c.vertices.len() % 3 == 0));

    let root = nif.root_node().unwrap().index();
    let mut edit = nif.edit::<BhkCompressedMeshShape>(shape).unwrap();
    assert!(!edit.set_mesh_target(Some(data)), "not an NiAvObject");
    assert!(!edit.set_compressed_data(Some(root)), "not compressed mesh data");
    assert!(edit.set_mesh_target(Some(root)));
    let mut s = edit.block();
    s.as_mut().set_radius(0.02);
    s.as_mut().set_radius_copy(0.03);
    s.as_mut().set_scale([2.0, 2.0, 2.0, 0.0]);
    s.set_user_data(9);

    let chunk = MeshChunk {
        translation: [1.0, 2.0, 3.0, 0.0],
        material_index: 0,
        reference: 0xFFFF,
        transform_index: 0,
        vertices: vec![0, 0, 0, 1000, 0, 0, 0, 1000, 0],
        indices: vec![0, 1, 2],
        strips: vec![],
        welding_info: vec![0],
    };
    let big = BigTriangle { triangle: [0, 1, 2], material: 0, welding_info: 5 };
    let mut d = nif.block_of_mut::<BhkCompressedMeshShapeData>(data).unwrap();
    assert!(d.as_mut().set_big_triangles(&[]));
    assert!(d.as_mut().set_big_vertices(&[[0.0; 4], [1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0]]));
    assert!(d.as_mut().set_big_triangles(&[big]));
    assert!(!d.as_mut().set_big_vertices(&[[0.0; 4]]), "the big triangle uses vertex 2");
    assert!(!d.as_mut().set_big_triangles(&[BigTriangle { triangle: [0, 1, 3], ..big }]));
    d.as_mut().set_chunks(std::slice::from_ref(&chunk));
    d.as_mut().set_chunk_materials(&[ChunkMaterial { material: 0x1234, filter: HavokFilter::default() }]);
    d.as_mut().set_chunk_transforms(&[ChunkTransform { translation: [0.0; 4], rotation: [0.0, 0.0, 0.0, 1.0] }]);
    d.as_mut().set_materials_8(&[8]);
    d.as_mut().set_error(0.001);
    d.as_mut().set_aabb_min([-1.0; 4]);
    d.set_bits_per_index(17);

    let saved = reload(&mut nif);
    let s = saved.view::<BhkCompressedMeshShape>(shape).unwrap();
    assert_eq!(s.mesh_target().unwrap().index(), root);
    assert_eq!((s.radius(), s.radius_copy(), s.scale(), s.user_data()), (0.02, 0.03, [2.0, 2.0, 2.0, 0.0], 9));
    let d = s.compressed_data().unwrap();
    assert_eq!((d.chunks(), d.big_triangles()), (vec![chunk], vec![big]));
    assert_eq!(d.big_vertices().len(), 3);
    assert_eq!(d.chunk_materials()[0].material, 0x1234);
    assert_eq!(d.chunk_transforms()[0].rotation, [0.0, 0.0, 0.0, 1.0]);
    assert_eq!((d.materials_8(), d.error(), d.aabb_min(), d.bits_per_index()), (&[8][..], 0.001, [-1.0; 4], 17));
}

#[test]
fn packed_mesh_by_version() {
    let vertices = [Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)];
    let triangle = PackedTriangle { triangle: [0, 1, 2], welding_info: 3, normal: Vec3::new(0.0, 0.0, 1.0) };
    let part = SubPart { filter: HavokFilter { layer: 2, flags_and_parts: 0, group: 0 }, vertex_count: 3, material: 6 };

    for version in [NifVersion::OBLIVION, NifVersion::FALLOUT_3] {
        let (mut nif, body) = with_body(version);
        let shape = nif.edit::<BhkRigidBody>(body).unwrap().assign_shape::<BhkPackedNiTriStripsShape>();
        let mut edit = nif.edit::<BhkPackedNiTriStripsShape>(shape).unwrap();
        let data = edit.assign_packed_data();
        assert!(!edit.set_packed_data(Some(body)), "not packed data");
        let mut s = edit.block();
        assert!(s.as_mut().set_sub_parts(&[part]));
        s.as_mut().set_radius(0.1);
        s.set_scale_copy([1.0; 4]);
        let mut d = nif.block_of_mut::<HkPackedNiTriStripsData>(data).unwrap();
        assert!(!d.as_mut().set_triangles(&[triangle]), "no vertices yet");
        assert!(d.as_mut().set_vertices(&vertices));
        assert!(d.as_mut().set_triangles(&[triangle]));
        assert!(!d.as_mut().set_vertices(&vertices[..2]), "the triangle uses vertex 2");
        assert!(d.set_sub_parts(&[part]));

        let saved = reload(&mut nif);
        let s = saved.view::<BhkPackedNiTriStripsShape>(shape).unwrap();
        assert_eq!((s.radius(), s.scale_copy()), (0.1, [1.0; 4]));
        let d = s.packed_data().unwrap();
        assert_eq!(d.vertices(), vertices);
        let oblivion = version == NifVersion::OBLIVION;
        let normal = if oblivion { triangle.normal } else { Vec3::ZERO };
        assert_eq!(d.triangles(), [PackedTriangle { normal, ..triangle }], "normals only saved up to Oblivion");
        let parts = |saved: bool| if saved { vec![part] } else { vec![] };
        assert_eq!((s.sub_parts(), d.sub_parts()), (parts(oblivion), parts(!oblivion)));
    }
}

#[test]
fn strips_shape() {
    // Strips_MW.nif: two NiTriStripsData blocks.
    let mut nif = load("Strips_MW");
    let data: Vec<_> = nif.blocks_of::<NiTriStripsData>().indices().collect();
    let root = nif.root_node().unwrap().index();
    let collision = nif.root_node_mut().unwrap().assign_collision::<BhkCollisionObject, _>();
    let body = nif.edit::<BhkCollisionObject>(collision).unwrap().assign_body::<BhkRigidBody, _>().unwrap();
    let shape = nif.edit::<BhkRigidBody>(body).unwrap().assign_shape::<BhkNiTriStripsShape>();

    let mut edit = nif.edit::<BhkNiTriStripsShape>(shape).unwrap();
    assert!(!edit.set_strips_data(&[root]), "not strips data");
    assert!(edit.set_strips_data(&data));
    assert_eq!(edit.block().filters(), [0, 0], "one per ref, as there were before");
    assert!(edit.block().set_filters(&[1, 2]));
    assert!(!edit.block().set_filters(&[1]));
    assert!(edit.set_strips_data(&data[..1]));
    assert_eq!(edit.block().filters(), [1]);
    let mut s = edit.block();
    s.as_mut().set_material(4);
    s.as_mut().set_grow_by(2);
    s.set_scale([2.0; 4]);

    let view = nif.view::<BhkNiTriStripsShape>(shape).unwrap();
    assert_eq!(view.strips_data().indices().collect::<Vec<_>>(), data[..1]);
    assert_eq!((view.material(), view.grow_by(), view.scale()), (4, 2, [2.0; 4]));
}
