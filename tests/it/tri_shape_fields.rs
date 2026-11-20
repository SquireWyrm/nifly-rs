//! Tests for BSTriShape's own fields: bounds, the vertex format, extra floats, particle data
//! and Skyrim SE segments.

use nifly::{
    file::{Index, NifFile},
    geometry::{BsSubIndexTriShape, BsTriShape, GeometrySegment, NiShape, ParticleData, VertexFlags},
    object3d::{BoundingSphere, Vec3},
};

use crate::common::{load, reload_in_place as reload, upstream};

const COLORS: VertexFlags = VertexFlags::COLORS;
const NORMALS: VertexFlags = VertexFlags::NORMALS;
const TANGENTS: VertexFlags = VertexFlags::TANGENTS;
const SECOND_UVS: VertexFlags = VertexFlags::UVS_2;
const FULL_PRECISION: VertexFlags = VertexFlags::FULL_PRECISION;

fn first(nif: &NifFile) -> Index {
    nif.blocks_of::<BsTriShape>().indices().next().unwrap()
}

fn shape(nif: &NifFile, index: Index) -> &BsTriShape {
    nif.block_of::<BsTriShape>(index).unwrap()
}

#[test]
fn bounds() {
    let mut nif = upstream("TestNifFile_FO76");
    let index = first(&nif);
    let sphere = BoundingSphere::new(Vec3::new(1.0, 2.0, 3.0), 4.0);
    let mut block = nif.block_of_mut::<BsTriShape>(index).unwrap();
    block.as_mut().set_bounds(sphere);
    block.set_bound_min_max([1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let saved = reload(&mut nif);
    assert_eq!(shape(&saved, index).bounds(), sphere);
    assert_eq!(shape(&saved, index).bound_min_max(), [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

    let mut nif = saved;
    nif.block_of_mut::<BsTriShape>(index).unwrap().update_bounds();
    assert_ne!(shape(&nif, index).bounds(), sphere);
}

#[test]
fn vertex_flags() {
    let mut nif = upstream("TestNifFile_Static_SE");
    let index = first(&nif);
    let flags = shape(&nif, index).vertex_flags();
    assert_eq!(shape(&nif, index).vertex_desc() >> 44, u64::from(flags.bits()));
    let mut edit = nif.edit::<BsTriShape>(index).unwrap();
    assert!(!edit.set_vertex_flags(flags | SECOND_UVS), "second UVs aren't stored");
    assert!(edit.set_vertex_flags(flags & !(NORMALS | TANGENTS) | COLORS));

    let saved = reload(&mut nif);
    assert_eq!(shape(&saved, index).vertex_flags(), flags & !(NORMALS | TANGENTS) | COLORS);
    let view = saved.view::<NiShape>(index).unwrap();
    assert_eq!(view.normals(), None);
    assert!(view.vertex_colors().unwrap().iter().all(|&c| c == [1.0; 4]), "new colors are white");
}

#[test]
fn extra_floats() {
    let mut nif = upstream("TestNifFile_Static_FO4");
    let index = first(&nif);
    let n = shape(&nif, index).vertex_extra_floats().len();
    let extra: Vec<Vec<f32>> = (0..n).map(|i| vec![i as f32, 0.5]).collect();
    let flags = shape(&nif, index).vertex_flags();
    let mut edit = nif.edit::<BsTriShape>(index).unwrap();
    assert!(edit.set_vertex_flags(flags & !FULL_PRECISION));
    assert!(!edit.set_vertex_extra_floats(&extra), "extra floats need full precision");
    assert!(edit.set_vertex_flags(flags | FULL_PRECISION));
    assert!(!edit.set_vertex_extra_floats(&extra[1..]));
    assert!(!edit.set_vertex_extra_floats(&vec![vec![0.0; 12]; n]), "a vertex is at most 60 bytes");
    assert!(edit.set_vertex_extra_floats(&extra));
    assert!(!edit.set_vertex_flags(flags & !FULL_PRECISION), "the extra floats need full precision");

    let saved = reload(&mut nif);
    assert_eq!(shape(&saved, index).vertex_extra_floats(), extra);
}

#[test]
fn particle_data() {
    let mut nif = upstream("TestNifFile_Static_SE");
    let index = first(&nif);
    assert_eq!(shape(&nif, index).particle_data(), None);
    let view = nif.view::<NiShape>(index).unwrap();
    let triangles = view.triangles().unwrap();
    let n = view.vertices().unwrap().len();
    let data = ParticleData {
        vertices: (0..n).map(|i| Vec3::new(i as f32, 0.5, -2.0)).collect(),
        normals: vec![Vec3::new(0.0, 0.0, 1.0); n],
        triangles: triangles.clone(),
    };
    let mut block = nif.block_of_mut::<BsTriShape>(index).unwrap();
    let mut bad = data.clone();
    bad.triangles[0][0] = n as u16;
    assert!(!block.as_mut().set_particle_data(Some(&bad)));
    assert!(!block.as_mut().set_particle_data(Some(&ParticleData { triangles: vec![], ..data.clone() })));
    assert!(block.set_particle_data(Some(&data)));

    let mut saved = reload(&mut nif);
    assert_eq!(shape(&saved, index).particle_data(), Some(data));
    assert!(saved.block_of_mut::<BsTriShape>(index).unwrap().set_particle_data(None));
    assert_eq!(shape(&reload(&mut saved), index).particle_data(), None);
}

#[test]
fn sse_segments() {
    // SubIndex_SE.nif: each shape has one segment holding its first triangle.
    let mut nif = load("SubIndex_SE");
    let index = nif.blocks_of::<BsSubIndexTriShape>().indices().next().unwrap();
    let block = nif.block_of::<BsSubIndexTriShape>(index).unwrap();
    assert_eq!(block.sse_segments(), [GeometrySegment { raw_flags: 1, index: 0, triangle_count: 1 }]);
    let count = nif.view::<NiShape>(index).unwrap().triangles().unwrap().len() as u32;

    let segments = [
        GeometrySegment { raw_flags: 0, index: 0, triangle_count: 1 },
        GeometrySegment { raw_flags: 2, index: 3, triangle_count: count - 1 },
    ];
    let mut block = nif.block_of_mut::<BsSubIndexTriShape>(index).unwrap();
    assert!(!block.as_mut().set_sse_segments(&[GeometrySegment { raw_flags: 0, index: 3, triangle_count: count }]));
    assert!(block.set_sse_segments(&segments));
    let saved = reload(&mut nif);
    assert_eq!(saved.block_of::<BsSubIndexTriShape>(index).unwrap().sse_segments(), segments);
}
