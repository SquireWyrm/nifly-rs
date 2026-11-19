//! Tests for BSMultiBound, its data blocks and the BSMultiBoundNode fields.

use crate::common::{reload};
use std::fs;

use nifly::{
    file::{Index, NifFile, NifVersion},
    handle::View,
    nodes::{
        BsMultiBound, BsMultiBoundAabb, BsMultiBoundData, BsMultiBoundNode, BsMultiBoundObb, BsMultiBoundSphere,
        CullingMode, MultiBoundShape,
    },
    object3d::{Matrix3, Transform, Vec3},
};

fn load() -> NifFile {
    let path = "nifly-sys/vendor/nifly/tests/input/TestNifFile_MultiBound_SE.nif";
    NifFile::load(fs::read(path).unwrap().as_slice(), None).unwrap()
}

fn node_index(nif: &NifFile) -> Index {
    nif.blocks_of::<BsMultiBoundNode>().indices().next().unwrap()
}

fn data(nif: &NifFile) -> View<'_, BsMultiBoundData> {
    let node = nif.blocks_of::<BsMultiBoundNode>().views().next().unwrap();
    node.multi_bound().unwrap().data().unwrap()
}

fn assign(nif: &mut NifFile, shape: MultiBoundShape) -> Index {
    let node = node_index(nif);
    nif.edit::<BsMultiBoundNode>(node).unwrap().assign_multi_bound(shape)
}

#[test]
fn reads_existing() {
    let nif = load();
    let node = nif.blocks_of::<BsMultiBoundNode>().next().unwrap();
    assert_eq!(node.name(), "FadeNode Anim");
    assert_eq!(node.culling_mode(), Some(CullingMode::Normal));
    let obb = data(&nif).cast::<BsMultiBoundObb>().unwrap();
    assert_eq!(obb.center(), Vec3::new(163.23926, 54.239258, 100.44919));
    assert_eq!(obb.size(), Vec3::new(604.15845, 469.32007, 512.7313));
    assert_eq!(obb.rotation(), Matrix3::IDENTITY);
}

#[test]
fn edits_round_trip() {
    let mut nif = load();
    let node = node_index(&nif);
    let rotation = Matrix3::from_rows(Vec3::Y, Vec3::Z, Vec3::X);
    let mut edit = nif.edit::<BsMultiBoundNode>(node).unwrap();
    edit.block().set_culling_mode(CullingMode::AllFail);
    let mut bound = edit.multi_bound().unwrap();
    let mut obb = bound.data().unwrap().into_cast::<BsMultiBoundObb>().unwrap();
    obb.block().set_center(Vec3::new(1.0, 2.0, 3.0));
    obb.block().set_size(Vec3::new(4.0, 5.0, 6.0));
    obb.block().set_rotation(rotation);

    let saved = reload(&mut nif);
    let node = saved.blocks_of::<BsMultiBoundNode>().next().unwrap();
    assert_eq!(node.culling_mode(), Some(CullingMode::AllFail));
    let obb = data(&saved).cast::<BsMultiBoundObb>().unwrap();
    assert_eq!((obb.center(), obb.size(), obb.rotation()), (Vec3::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0), rotation));
}

#[test]
fn assigns_each_shape() {
    let mut nif = load();
    let center = Vec3::new(1.0, 2.0, 3.0);

    let bound = assign(&mut nif, MultiBoundShape::Aabb { center, half_extent: Vec3::new(4.0, 5.0, 6.0) });
    assert!(nif.block_of::<BsMultiBound>(bound).is_some());
    let saved = reload(&mut nif);
    let aabb = data(&saved).cast::<BsMultiBoundAabb>().unwrap();
    assert_eq!((aabb.center(), aabb.half_extent()), (center, Vec3::new(4.0, 5.0, 6.0)));
    // The old multi-bound and its OBB were dropped on save.
    assert_eq!(saved.blocks_of::<BsMultiBound>().count(), 1);
    assert_eq!(saved.blocks_of::<BsMultiBoundObb>().count(), 0);

    assign(&mut nif, MultiBoundShape::Sphere { center, radius: 7.0 });
    let saved = reload(&mut nif);
    let sphere = data(&saved).cast::<BsMultiBoundSphere>().unwrap();
    assert_eq!((sphere.center(), sphere.radius()), (center, 7.0));

    let rotation = Matrix3::from_rows(Vec3::Z, Vec3::X, Vec3::Y);
    assign(&mut nif, MultiBoundShape::Obb { center, size: Vec3::new(8.0, 9.0, 10.0), rotation });
    let saved = reload(&mut nif);
    let obb = data(&saved).cast::<BsMultiBoundObb>().unwrap();
    assert_eq!((obb.center(), obb.size(), obb.rotation()), (center, Vec3::new(8.0, 9.0, 10.0), rotation));
}

#[test]
fn removes() {
    let mut nif = load();
    let node = node_index(&nif);
    let mut edit = nif.edit::<BsMultiBoundNode>(node).unwrap();
    assert!(edit.remove_multi_bound());
    assert!(!edit.remove_multi_bound());
    assert!(edit.multi_bound().is_none());
    let saved = reload(&mut nif);
    assert!(saved.blocks_of::<BsMultiBoundNode>().views().next().unwrap().multi_bound().is_none());
    assert_eq!(saved.blocks_of::<BsMultiBound>().count(), 0);
}

#[test]
fn only_multi_bound_nodes() {
    let nif = load();
    let root = nif.root_node().unwrap().index();
    assert!(nif.view::<BsMultiBoundNode>(root).is_none());
    assert!(nif.view::<BsMultiBound>(root).is_none());
}

#[test]
fn culling_mode_not_saved_in_fallout_3() {
    let mut nif = NifFile::create(NifVersion::FALLOUT_3);
    let node = nif.add_node_of::<BsMultiBoundNode>("Bound", Transform::default(), None).unwrap();
    nif.block_of_mut::<BsMultiBoundNode>(node).unwrap().set_culling_mode(CullingMode::AllPass);
    let saved = reload(&mut nif);
    let node = saved.blocks_of::<BsMultiBoundNode>().next().unwrap();
    assert_eq!(node.culling_mode(), Some(CullingMode::Normal));
}
