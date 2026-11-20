//! Tests for the skin root, BSSkin::Instance scales, per-bone data and partition fields.

use crate::common::{UPSTREAM, reload};
use std::fs;

use nifly::{
    file::NifFile,
    geometry::NiShape,
    object3d::{Transform, Vec3},
    skin::{BsSkinBoneData, BsSkinInstance, NiBoneContainer, NiSkinData, NiSkinPartition},
};

fn load(name: &str) -> NifFile {
    NifFile::load(fs::read(format!("{UPSTREAM}/{name}.nif")).unwrap().as_slice(), None).unwrap()
}

fn root_name(nif: &NifFile) -> String {
    let skin = nif.blocks_of::<NiBoneContainer>().views().next().unwrap();
    skin.root().unwrap().name().to_string()
}

fn first_skin(nif: &NifFile) -> usize {
    nif.blocks_of::<NiBoneContainer>().indices().next().unwrap()
}

#[test]
fn skin_root() {
    let mut nif = load("TestNifFile_Skinned_SE");
    assert_eq!(root_name(&nif), "Scene Root");

    let skin = first_skin(&nif);
    let skeleton = nif.add_node("Skeleton", Transform::default(), None).unwrap();
    assert!(nif.edit::<NiBoneContainer>(skin).unwrap().set_root(skeleton));
    assert_eq!(root_name(&reload(&mut nif)), "Skeleton");

    // Saving sorted the blocks.
    let skin = first_skin(&nif);
    let shape = nif.shapes().indices().next().unwrap();
    let mut edit = nif.edit::<NiBoneContainer>(skin).unwrap();
    assert!(edit.root().unwrap().into_cast::<nifly::nodes::NiNode>().is_ok());

    // An NiSkinInstance's root must be a node.
    assert!(!edit.set_root(shape));
    assert!(!edit.set_root(usize::MAX));
    assert_eq!(root_name(&nif), "Skeleton");
    // Only skin instances have a root.
    assert!(nif.view::<NiBoneContainer>(shape).is_none());
}

#[test]
fn bs_skin_instance_root_can_be_a_shape() {
    let mut nif = load("TestNifFile_Skinned_FO4");
    let skin = nif.blocks_of::<BsSkinInstance>().indices().next().unwrap();
    let shape = nif.shapes().indices().next().unwrap();
    let name = nif.block_of::<NiShape>(shape).unwrap().name().to_string();
    assert!(nif.edit::<NiBoneContainer>(skin).unwrap().set_root(shape));
    assert_eq!(root_name(&reload(&mut nif)), name);
}

#[test]
fn bs_skin_instance_scales() {
    let mut nif = load("TestNifFile_Skinned_FO4");
    let skin = nif.blocks_of::<BsSkinInstance>().indices().next().unwrap();
    assert!(nif.block_of::<BsSkinInstance>(skin).unwrap().scales().is_empty());

    let scales = [Vec3::new(1.0, 1.0, 1.0), Vec3::new(2.0, 0.5, 1.0)];
    nif.block_of_mut::<BsSkinInstance>(skin).unwrap().set_scales(&scales);
    let saved = reload(&mut nif);
    assert_eq!(saved.block_of::<BsSkinInstance>(skin).unwrap().scales(), scales);
}

#[test]
fn per_bone_data() {
    let nif = load("TestNifFile_Skinned_FO4");
    let shape = nif.shapes().next().unwrap();
    let skin = nif.child_blocks_of::<BsSkinInstance>(shape).next().unwrap();
    let data = nif.child_blocks_of::<BsSkinBoneData>(skin).next().unwrap();
    assert_eq!(data.bone_count(), 2);
    assert_eq!(data.transform_skin_to_bone(1), nif.view_of(shape).unwrap().transform_skin_to_bone(1));
    assert_eq!(data.bounds(1), nif.view_of(shape).unwrap().bone_bounds(1));
    assert_eq!((data.transform_skin_to_bone(2), data.bounds(2)), (None, None));

    let nif = load("TestNifFile_Skinned_OB");
    let shape = nif.shapes().next().unwrap();
    let data = nif.blocks_of::<NiSkinData>().next().unwrap();
    assert_eq!(data.bounds(3), nif.view_of(shape).unwrap().bone_bounds(3));
    assert_eq!(data.bounds(data.bone_count()), None);
}

#[test]
fn has_vertex_weights() {
    let mut nif = load("TestNifFile_Skinned_OB");
    let index = nif.blocks_of::<NiSkinData>().indices().next().unwrap();
    nif.block_of_mut::<NiSkinData>(index).unwrap().set_has_vertex_weights(false);
    let saved = reload(&mut nif);
    assert!(!saved.block_of::<NiSkinData>(index).unwrap().has_vertex_weights());
    let shape = saved.shapes().next().unwrap();
    assert!(saved.view_of(shape).unwrap().bone_weights(0).is_empty());
}

#[test]
fn partitions() {
    let nif = load("TestNifFile_Skinned_SE");
    let part = nif.blocks_of::<NiSkinPartition>().next().unwrap();
    let vertices = part.vertex_map(0).unwrap().len();
    assert_eq!(part.weights_per_vertex(0), Some(4));
    assert_eq!(part.vertex_weights(0).unwrap().len(), vertices);
    assert_eq!(part.vertex_weights(0).unwrap()[0], [1.0, 0.0, 0.0, 0.0]);
    assert_eq!(part.bone_indices(0).unwrap().len(), vertices);
    assert_eq!(part.triangles(0).unwrap().len(), part.triangle_count(0).unwrap());
    assert_eq!(part.triangles(0).unwrap()[0], [0, 1, 2]);
    assert!(part.strips(0).unwrap().is_empty());
    assert_eq!((part.lod_level(0), part.global_vb(0)), (Some(0), Some(false)));
    assert_eq!(part.vertex_weights(1), None);
    assert_eq!(part.strips(1), None);

    // Oblivion's partition is one strip.
    let nif = load("TestNifFile_Skinned_OB");
    let part = nif.blocks_of::<NiSkinPartition>().next().unwrap();
    assert!(part.triangles(0).unwrap().is_empty());
    let strips = part.strips(0).unwrap();
    assert_eq!(strips.len(), 1);
    assert!(strips[0].len() > 2);
}

#[test]
fn partition_lod_and_global_vb() {
    let mut nif = load("TestNifFile_Skinned_SE");
    let index = nif.blocks_of::<NiSkinPartition>().indices().next().unwrap();
    let part = nif.block_of_mut::<NiSkinPartition>(index).unwrap();
    assert!(part.set_lod_level(0, 2));
    assert!(part.set_global_vb(0, true));
    assert!(!part.set_lod_level(1, 2));
    assert!(!part.set_global_vb(1, true));
    let saved = reload(&mut nif);
    let part = saved.block_of::<NiSkinPartition>(index).unwrap();
    assert_eq!((part.lod_level(0), part.global_vb(0)), (Some(2), Some(true)));

    // Not saved before Skyrim.
    let mut nif = load("TestNifFile_Skinned_OB");
    let index = nif.blocks_of::<NiSkinPartition>().indices().next().unwrap();
    nif.block_of_mut::<NiSkinPartition>(index).unwrap().set_lod_level(0, 2);
    let saved = reload(&mut nif);
    assert_eq!(saved.block_of::<NiSkinPartition>(index).unwrap().lod_level(0), Some(0));
}
