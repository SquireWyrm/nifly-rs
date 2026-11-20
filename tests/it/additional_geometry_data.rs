//! Tests for additional geometry data: NiAdditionalGeometryData, BSPackedAdditionalGeometryData
//! and NiGeometryData's ref to them.

use crate::common::upstream;
use std::io::Cursor;

use nifly::{
    file::{NifFile, SaveOptions},
    geometry::{
        AdditionalDataBlock, AdditionalGeomData, BsPackedAdditionalGeometryData, DataChannel, NiAdditionalGeometryData,
        NiGeometryData, PackedData, PackedDataBlock,
    },
};

fn reload(nif: &mut NifFile, options: Option<SaveOptions>) -> NifFile {
    let mut cur = Cursor::new(Vec::new());
    nif.save(&mut cur, options).unwrap();
    NifFile::load(cur.into_inner().as_slice(), None).unwrap()
}

/// Keeps block indices the same.
const KEEP: Option<SaveOptions> = Some(SaveOptions { optimize: false, sort_blocks: false });

fn first_data(nif: &NifFile) -> usize {
    nif.blocks_of::<NiGeometryData>().indices().next().unwrap()
}

fn channels() -> Vec<DataChannel> {
    let position = DataChannel { data_type: 7, unit_size: 12, total_size: 36, stride: 16, ..DataChannel::default() };
    vec![position, DataChannel { unit_size: 4, total_size: 12, offset: 12, raw_flags: 3, ..position }]
}

fn blocks() -> Vec<Option<AdditionalDataBlock>> {
    vec![
        Some(AdditionalDataBlock {
            block_size: 3,
            block_offsets: vec![0, 48],
            data_sizes: vec![3, 1],
            data: vec![vec![1, 2, 3], vec![4, 5, 6]],
        }),
        None,
        Some(AdditionalDataBlock::default()),
    ]
}

#[test]
fn assign_and_remove() {
    let mut nif = upstream("TestNifFile_Skinned_OB");
    let data = first_data(&nif);
    let vertices = nif.view::<NiGeometryData>(data).unwrap().vertex_count();
    let count = nif.blocks().count();
    let mut edit = nif.edit::<NiGeometryData>(data).unwrap();
    assert!(edit.additional_data().is_none());
    assert!(!edit.remove_additional_data());

    let extra = edit.assign_additional_data::<NiAdditionalGeometryData, _>();
    assert_eq!(extra, count);
    let view = edit.view().additional_data().unwrap();
    assert_eq!(view.index(), extra);
    assert_eq!(view.vertex_count() as usize, vertices);
    assert!(view.channels().is_empty());
    assert!(view.cast::<NiAdditionalGeometryData>().unwrap().blocks().is_empty());

    // Replacing it adds a new block.
    let packed = edit.assign_additional_data::<BsPackedAdditionalGeometryData, _>();
    assert_eq!(packed, count + 1);
    edit.additional_data().unwrap().block().set_vertex_count(5);
    assert_eq!(nif.view::<BsPackedAdditionalGeometryData>(packed).unwrap().vertex_count(), 5);

    let mut edit = nif.edit::<NiGeometryData>(data).unwrap();
    assert!(edit.remove_additional_data());
    assert!(edit.view().additional_data().is_none());
    assert!(!edit.remove_additional_data());
    // Saving drops the unused blocks.
    assert_eq!(reload(&mut nif, None).blocks().count(), count);
}

#[test]
fn only_geometry_data() {
    let mut nif = upstream("TestNifFile_Skinned_OB");
    assert!(nif.edit::<NiGeometryData>(0).is_none());
    assert!(nif.view::<AdditionalGeomData>(first_data(&nif)).is_none());
}

#[test]
fn fields_survive_save() {
    let mut nif = upstream("TestNifFile_Skinned_OB");
    let data = first_data(&nif);
    let extra = nif.edit::<NiGeometryData>(data).unwrap().assign_additional_data::<NiAdditionalGeometryData, _>();
    let mut block = nif.block_of_mut::<NiAdditionalGeometryData>(extra).unwrap();
    block.as_mut().upcast_mut().set_vertex_count(3);
    block.as_mut().upcast_mut().set_channels(&channels());
    assert!(block.as_mut().set_blocks(&blocks()));

    let saved = reload(&mut nif, KEEP);
    let view = saved.view::<NiGeometryData>(data).unwrap().additional_data().unwrap();
    assert_eq!(view.index(), extra);
    assert_eq!((view.vertex_count(), view.channels()), (3, channels()));
    assert_eq!(view.cast::<NiAdditionalGeometryData>().unwrap().blocks(), blocks());
    // A large data type keeps its value.
    let mut block = nif.block_of_mut::<NiAdditionalGeometryData>(extra).unwrap();
    let big = [DataChannel { data_type: u32::MAX, ..DataChannel::default() }];
    block.as_mut().upcast_mut().set_channels(&big);
    // Saving again keeps the block indices.
    assert_eq!(reload(&mut nif, KEEP).block_of::<AdditionalGeomData>(extra).unwrap().channels(), big);
}

#[test]
fn set_blocks_checks_sizes() {
    let mut nif = upstream("TestNifFile_Skinned_OB");
    let data = first_data(&nif);
    let extra = nif.edit::<NiGeometryData>(data).unwrap().assign_additional_data::<NiAdditionalGeometryData, _>();
    let mut block = nif.block_of_mut::<NiAdditionalGeometryData>(extra).unwrap();
    assert!(block.as_mut().set_blocks(&blocks()));

    let good = AdditionalDataBlock { block_size: 2, block_offsets: vec![], data_sizes: vec![2], data: vec![vec![1, 2]] };
    let short_item = AdditionalDataBlock { data: vec![vec![1]], ..good.clone() };
    let missing_size = AdditionalDataBlock { data_sizes: vec![], ..good.clone() };
    for bad in [short_item, missing_size] {
        assert!(!block.as_mut().set_blocks(&[Some(good.clone()), Some(bad)]));
    }
    assert_eq!(block.blocks(), blocks());
}

#[test]
fn packed_blocks_survive_save() {
    let mut nif = upstream("TestNifFile_Skinned_OB");
    let data = first_data(&nif);
    let extra = nif.edit::<NiGeometryData>(data).unwrap().assign_additional_data::<BsPackedAdditionalGeometryData, _>();
    let blocks = vec![
        PackedDataBlock {
            data: Some(PackedData { block_offsets: vec![0], atom_sizes: vec![4, 4], bytes: (0..24).collect() }),
            shader_index: 1,
            total_size: 8,
        },
        // The two numbers are kept without data.
        PackedDataBlock { data: None, shader_index: 2, total_size: 3 },
    ];
    let mut block = nif.block_of_mut::<BsPackedAdditionalGeometryData>(extra).unwrap();
    block.as_mut().upcast_mut().set_channels(&channels());
    block.as_mut().set_blocks(&blocks);

    let saved = reload(&mut nif, KEEP);
    let view = saved.view::<NiGeometryData>(data).unwrap().additional_data().unwrap();
    assert_eq!(view.channels(), channels());
    assert_eq!(view.cast::<BsPackedAdditionalGeometryData>().unwrap().blocks(), blocks);
}

#[test]
fn morrowind_drops_the_ref() {
    let mut nif = upstream("TestNifFile_Static_MW");
    let data = first_data(&nif);
    nif.edit::<NiGeometryData>(data).unwrap().assign_additional_data::<NiAdditionalGeometryData, _>();
    let saved = reload(&mut nif, KEEP);
    assert!(saved.view::<NiGeometryData>(data).unwrap().additional_data().is_none());
}
