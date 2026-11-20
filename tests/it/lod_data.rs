//! Tests for NiLODNode's LOD data: NiRangeLODData and NiScreenLODData.

use crate::common::reload;

use nifly::{
    file::{Index, NifFile, NifVersion},
    handle::View,
    nodes::{LodRange, NiLodData, NiLodNode, NiRangeLodData, NiScreenLodData},
    object3d::{BoundingSphere, Transform, Vec3},
};

fn lod_file() -> (NifFile, Index) {
    let mut nif = NifFile::create(NifVersion::OBLIVION);
    let node = nif
        .add_node_of::<NiLodNode>("LOD", Transform::default(), None)
        .unwrap();
    (nif, node)
}

fn data(nif: &NifFile) -> View<'_, NiLodData> {
    nif.blocks_of::<NiLodNode>()
        .views()
        .next()
        .unwrap()
        .lod_data()
        .unwrap()
}

#[test]
fn range_lod_data() {
    let (mut nif, node) = lod_file();
    let index = nif
        .edit::<NiLodNode>(node)
        .unwrap()
        .assign_lod_data::<NiRangeLodData>();
    let levels = [
        LodRange {
            near: 0.0,
            far: 1000.0,
        },
        LodRange {
            near: 1000.0,
            far: 4000.0,
        },
    ];
    let d = nif.block_of_mut::<NiRangeLodData>(index).unwrap();
    d.set_center(Vec3::new(1.0, 2.0, 3.0));
    d.set_levels(&levels);

    let saved = reload(&mut nif);
    let d = data(&saved).cast::<NiRangeLodData>().unwrap();
    assert_eq!(d.center(), Vec3::new(1.0, 2.0, 3.0));
    assert_eq!(d.levels(), levels);
}

#[test]
fn screen_lod_data() {
    let (mut nif, node) = lod_file();
    let mut edit = nif.edit::<NiLodNode>(node).unwrap();
    edit.assign_lod_data::<NiScreenLodData>();
    let bound = BoundingSphere {
        center: Vec3::new(1.0, 0.0, 0.0),
        radius: 5.0,
    };
    let world = BoundingSphere {
        center: Vec3::new(0.0, 2.0, 0.0),
        radius: 10.0,
    };
    let mut screen = edit
        .lod_data()
        .unwrap()
        .into_cast::<NiScreenLodData>()
        .unwrap();
    screen.set_bound(bound);
    screen.set_world_bound(world);
    screen.set_proportion_levels(&[0.5, 0.1]);

    let saved = reload(&mut nif);
    let d = data(&saved).cast::<NiScreenLodData>().unwrap();
    assert_eq!((d.bound(), d.world_bound()), (bound, world));
    assert_eq!(d.proportion_levels(), [0.5, 0.1]);
}

#[test]
fn replaces_and_removes() {
    let (mut nif, node) = lod_file();
    let mut edit = nif.edit::<NiLodNode>(node).unwrap();
    edit.assign_lod_data::<NiRangeLodData>();
    edit.assign_lod_data::<NiScreenLodData>();
    let saved = reload(&mut nif);
    assert!(data(&saved).cast::<NiScreenLodData>().is_some());
    // The replaced data was dropped on save.
    assert_eq!(saved.blocks_of::<NiLodData>().count(), 1);

    let node = nif.blocks_of::<NiLodNode>().indices().next().unwrap();
    let mut edit = nif.edit::<NiLodNode>(node).unwrap();
    assert!(edit.remove_lod_data());
    assert!(!edit.remove_lod_data());
    assert!(edit.lod_data().is_none());
    assert_eq!(reload(&mut nif).blocks_of::<NiLodData>().count(), 0);
}

#[test]
fn only_lod_nodes() {
    let (nif, _) = lod_file();
    let root = nif.root_node().unwrap().index();
    assert!(nif.view::<NiLodNode>(root).is_none());
}
