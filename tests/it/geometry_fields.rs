//! Tests for the fields of geometry data and NiGeometry blocks: arrays, UV sets, data flags,
//! match groups, strips, materials and the shader name.

use nifly::{
    file::{Index, NifFile},
    geometry::{NiGeometry, NiGeometryData, NiTriShapeData, NiTriStripsData},
    object3d::Vec3,
    objects::NiType,
};

use crate::common::{load, reload_in_place as reload, upstream};

fn first<T: NiType>(nif: &NifFile) -> Index {
    nif.blocks_of::<T>().indices().next().unwrap()
}

fn data(nif: &NifFile, index: Index) -> &NiGeometryData {
    nif.block_of::<NiGeometryData>(index).unwrap()
}

#[test]
fn arrays() {
    let mut nif = load("Bones");
    let index = first::<NiGeometryData>(&nif);
    let n = data(&nif, index).vertex_count();
    let vertices: Vec<Vec3> = (0..n).map(|i| Vec3::new(i as f32, 1.0, 2.0)).collect();
    let tangents = vec![Vec3::new(1.0, 0.0, 0.0); n];
    let colors = vec![[0.5, 0.25, 1.0, 1.0]; n];

    let block = nif.block_of_mut::<NiGeometryData>(index).unwrap();
    assert!(!block.set_vertices(&vertices[1..]));
    assert!(block.set_vertices(&vertices));
    assert!(block.set_tangents(&tangents));
    assert!(block.set_bitangents(&tangents));
    assert!(block.set_vertex_colors(&colors));
    block.set_has_normals(false);

    let saved = reload(&mut nif);
    let block = data(&saved, index);
    assert_eq!(block.vertices(), vertices);
    assert_eq!((block.tangents(), block.bitangents()), (vec![], vec![]), "not saved without normals");
    assert_eq!(block.vertex_colors(), colors);
    assert!(!block.has_normals() && block.normals().is_empty());

    let mut nif = saved;
    let block = nif.block_of_mut::<NiGeometryData>(index).unwrap();
    assert!(block.set_normals(&tangents));
    assert!(block.set_tangents(&tangents));
    block.set_has_vertex_colors(false);
    let saved = reload(&mut nif);
    let block = data(&saved, index);
    assert_eq!((block.normals(), block.tangents()), (tangents.clone(), tangents));
    assert!(block.vertex_colors().is_empty());
}

#[test]
fn uv_sets_and_data_flags() {
    let mut nif = upstream("TestNifFile_Skinned_OB");
    let index = first::<NiGeometryData>(&nif);
    let n = data(&nif, index).vertex_count();
    let sets = [vec![[0.5, 0.5]; n], vec![[1.0, 0.0]; n]];
    let mut edit = nif.edit::<NiGeometryData>(index).unwrap();
    assert!(!edit.set_uv_sets(&[&sets[0][1..]]));
    assert!(edit.set_uv_sets(&sets));
    let saved = reload(&mut nif);
    assert_eq!(data(&saved, index).uv_sets(), sets);
    assert_eq!(data(&saved, index).data_raw_flags() & 0x3F, 2);

    let mut nif = saved;
    let flags = data(&nif, index).data_raw_flags() & !0xF03F | 3;
    nif.edit::<NiGeometryData>(index).unwrap().set_data_raw_flags(flags);
    let block = data(&nif, index);
    assert_eq!(block.uv_sets().len(), 3);
    assert_eq!(block.uv_sets()[2], vec![[0.0, 0.0]; n]);
    assert!(!block.has_tangents() && block.tangents().is_empty());
    // Saving drops the tangents' extra data block, moving the data.
    let saved = reload(&mut nif);
    assert_eq!(data(&saved, first::<NiGeometryData>(&saved)).data_raw_flags(), flags);

    // Fallout 3 on stores one set at most.
    let mut nif = load("Bones");
    let index = first::<NiGeometryData>(&nif);
    let n = data(&nif, index).vertex_count();
    assert!(!nif.edit::<NiGeometryData>(index).unwrap().set_uv_sets(&[vec![[0.0, 0.0]; n], vec![[0.0, 0.0]; n]]));
    assert!(nif.edit::<NiGeometryData>(index).unwrap().set_uv_sets::<Vec<[f32; 2]>, _>(&[]));
    let saved = reload(&mut nif);
    assert!(!data(&saved, index).has_uvs() && data(&saved, index).uv_sets().is_empty());
}

#[test]
fn scalar_fields() {
    let mut nif = load("Bones");
    let index = first::<NiGeometryData>(&nif);
    let block = nif.block_of_mut::<NiGeometryData>(index).unwrap();
    block.set_geometry_group_id(-3);
    block.set_keep_raw_flags(1);
    block.set_compress_raw_flags(2);
    block.set_material_crc(0xDEAD);
    let saved = reload(&mut nif);
    let block = data(&saved, index);
    assert_eq!(
        (block.geometry_group_id(), block.keep_raw_flags(), block.compress_raw_flags(), block.material_crc()),
        (-3, 1, 2, 0xDEAD)
    );
}

#[test]
fn match_groups() {
    let mut nif = load("Bones");
    let index = first::<NiTriShapeData>(&nif);
    let n = data(&nif, index).vertex_count() as u16;
    let block = nif.block_of_mut::<NiTriShapeData>(index).unwrap();
    assert!(!block.set_match_groups(&[vec![0, n]]));
    assert!(block.set_match_groups(&[vec![0, 1], vec![2]]));
    let saved = reload(&mut nif);
    assert_eq!(saved.block_of::<NiTriShapeData>(index).unwrap().match_groups(), [vec![0, 1], vec![2]]);
}

#[test]
fn strips() {
    let mut nif = load("Strips_MW");
    let index = first::<NiTriStripsData>(&nif);
    let n = data(&nif, index).vertex_count() as u16;
    let block = nif.block_of_mut::<NiTriStripsData>(index).unwrap();
    assert!(!block.set_strips(&[vec![0, 1, n]]));
    assert!(block.set_strips(&[vec![0, 1, 2, 3], vec![1, 2]]));
    let saved = reload(&mut nif);
    let block = saved.block_of::<NiTriStripsData>(index).unwrap();
    assert_eq!(block.strips(), [vec![0, 1, 2, 3], vec![1, 2]]);
    assert_eq!(block.triangle_count(), 2);
}

#[test]
fn materials() {
    let mut nif = load("Bones");
    let shape = first::<NiGeometry>(&nif);
    let block = nif.block_of_mut::<NiGeometry>(shape).unwrap();
    block.set_materials(&[("First", 5), ("Second", 6)]);
    block.set_active_material(1);
    block.set_default_material_update_flag(1);
    let saved = reload(&mut nif);
    let block = saved.block_of::<NiGeometry>(shape).unwrap();
    assert_eq!(block.materials(), [("First".into(), 5), ("Second".into(), 6)]);
    assert_eq!((block.active_material(), block.default_material_update_flag()), (1, 1));
}

#[test]
fn shader_name() {
    let mut nif = upstream("TestNifFile_Skinned_OB");
    let shape = first::<NiGeometry>(&nif);
    let block = nif.block_of_mut::<NiGeometry>(shape).unwrap();
    block.set_shader_name(Some("Shader"));
    block.set_shader_implementation(7);
    let mut saved = reload(&mut nif);
    let block = saved.block_of::<NiGeometry>(shape).unwrap();
    assert_eq!((block.shader_name(), block.shader_implementation()), (Some("Shader".into()), 7));

    saved.block_of_mut::<NiGeometry>(shape).unwrap().set_shader_name(None::<&str>);
    assert_eq!(reload(&mut saved).block_of::<NiGeometry>(shape).unwrap().shader_name(), None);
}
