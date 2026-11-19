//! Tests for NifFile::set_shape_bone_ids and NifFile::set_shape_bone_list.

use crate::common::{load, upstream, reload};

use nifly::{
    file::{Index, NifFile},
    geometry::NiShape,
    object3d::Transform,
    skin::{NiSkinData, NiSkinInstance},
};

fn shape<'a>(nif: &'a NifFile, name: &str) -> &'a NiShape {
    nif.shapes().find(|s| s.name() == name).unwrap()
}

fn index(nif: &NifFile, name: &str) -> Index {
    nif.shapes().indexed().find(|(_, s)| s.name() == name).unwrap().0
}

fn bones(nif: &NifFile, name: &str) -> Vec<String> {
    nif.view_of(shape(nif, name)).unwrap().bone_list().iter().map(ToString::to_string).collect()
}

fn skin_data<'a>(nif: &'a NifFile, name: &str) -> &'a NiSkinData {
    let inst = nif.child_blocks_of::<NiSkinInstance>(shape(nif, name)).next().unwrap();
    nif.child_blocks_of::<NiSkinData>(inst).next().unwrap()
}

#[test]
fn same_count_keeps_data_by_position() {
    let mut nif = load("Bones");
    let limb = index(&nif, "Limb");
    let s = shape(&nif, "Limb");
    let weights: Vec<_> = (0..3).map(|b| nif.view_of(s).unwrap().bone_weights(b)).collect();
    let transforms: Vec<_> = (0..3).map(|b| nif.view_of(s).unwrap().transform_skin_to_bone(b)).collect();

    // Reverse the bones: Bone3, Bone2, Bone1.
    assert!(nif.edit::<nifly::geometry::NiShape>(limb).is_some_and(|mut e| e.set_bone_ids(&[3, 2, 1])));
    let saved = reload(&mut nif);
    for nif in [&nif, &saved] {
        assert_eq!(bones(nif, "Limb"), ["Bone3", "Bone2", "Bone1"]);
        let s = shape(nif, "Limb");
        // Weights and transforms stay with their positions.
        assert_eq!((0..3).map(|b| nif.view_of(s).unwrap().bone_weights(b)).collect::<Vec<_>>(), weights);
        assert_eq!((0..3).map(|b| nif.view_of(s).unwrap().transform_skin_to_bone(b)).collect::<Vec<_>>(), transforms);
    }
}

#[test]
fn new_count_resets_skin_data() {
    let mut nif = load("Bones");
    let limb = index(&nif, "Limb");
    assert!(!nif.view_of(shape(&nif, "Limb")).unwrap().bone_weights(0).is_empty());

    assert!(nif.edit::<nifly::geometry::NiShape>(limb).is_some_and(|mut e| e.set_bone_list(["Bone1", "Bone2"])));
    assert_eq!(bones(&nif, "Limb"), ["Bone1", "Bone2"]);
    assert_eq!(skin_data(&nif, "Limb").bone_count(), 2);
    let s = shape(&nif, "Limb");
    for bone in 0..2 {
        assert!(nif.view_of(s).unwrap().bone_weights(bone).is_empty(), "{bone}");
        assert_eq!(nif.view_of(s).unwrap().transform_skin_to_bone(bone), Some(Transform::IDENTITY), "{bone}");
    }
    let saved = reload(&mut nif);
    assert_eq!(bones(&saved, "Limb"), ["Bone1", "Bone2"]);

    // Bones can repeat, and the list can be emptied.
    assert!(nif.edit::<nifly::geometry::NiShape>(limb).is_some_and(|mut e| e.set_bone_ids(&[1, 1, 2, 3])));
    assert_eq!(bones(&nif, "Limb"), ["Bone1", "Bone1", "Bone2", "Bone3"]);
    assert!(nif.edit::<nifly::geometry::NiShape>(limb).is_some_and(|mut e| e.set_bone_ids(&[])));
    assert!(bones(&nif, "Limb").is_empty());
    assert_eq!(skin_data(&nif, "Limb").bone_count(), 0);
}

#[test]
fn after_create_skinning() {
    // Skyrim SE: a new skin, bones by name, then per-vertex weights.
    let mut nif = load("Cube");
    let cube = index(&nif, "Cube");
    let root = nif.root_node().map(|r| r.index()).unwrap();
    let bone = nif.add_node("Bone", Transform::IDENTITY, Some(root)).unwrap();
    assert!(nif.edit::<nifly::geometry::NiShape>(cube).is_some_and(|mut e| e.create_skinning()));
    assert!(nif.edit::<nifly::geometry::NiShape>(cube).is_some_and(|mut e| e.set_bone_list(["Bone"])));
    assert_eq!(nif.view_of(shape(&nif, "Cube")).unwrap().bone_ids(), [bone]);
    assert!(nif.edit::<nifly::geometry::NiShape>(cube).is_some_and(|mut e| e.set_vertex_weights(0, &[(0, 1.0)])));
    let saved = reload(&mut nif);
    assert_eq!(bones(&saved, "Cube"), ["Bone"]);
    assert_eq!(saved.view_of(shape(&saved, "Cube")).unwrap().bone_weights(0).len(), 1);

    // Fallout 4: BSSkin::Instance, using the file's unused bone nodes.
    let mut nif = upstream("TestNifFile_Static_FO4");
    let cylinder = index(&nif, "cylinder_1");
    assert!(nif.edit::<nifly::geometry::NiShape>(cylinder).is_some_and(|mut e| e.create_skinning()));
    assert!(nif.edit::<nifly::geometry::NiShape>(cylinder).is_some_and(|mut e| e.set_bone_list(["BoneA", "BoneB"])));
    let saved = reload(&mut nif);
    for nif in [&nif, &saved] {
        assert_eq!(bones(nif, "cylinder_1"), ["BoneA", "BoneB"]);
        assert!(bones(nif, "cylinder_2").is_empty());
    }
}

#[test]
fn bs_geometry_by_name() {
    let mut nif = upstream("TestNifFile_SF");
    let geometry = nif.shapes().indices().next().unwrap();
    assert_eq!(nif.shapes().views().next().unwrap().bone_list().len(), 38);

    // Names needn't be nodes in the file.
    assert!(nif.edit::<nifly::geometry::NiShape>(geometry).is_some_and(|mut e| e.set_bone_list(["C_Hips", "", "Not A Node"])));
    let saved = reload(&mut nif);
    for nif in [&nif, &saved] {
        let s = nif.shapes().next().unwrap();
        assert_eq!(nif.view_of(s).unwrap().bone_list(), ["C_Hips", "", "Not A Node"]);
    }
    assert!(nif.edit::<nifly::geometry::NiShape>(geometry).is_some_and(|mut e| e.set_bone_list(Vec::<&str>::new())));
    assert!(nif.shapes().views().next().unwrap().bone_list().is_empty());

    // It binds by name, not by node.
    assert!(!nif.edit::<nifly::geometry::NiShape>(geometry).is_some_and(|mut e| e.set_bone_ids(&[0])));
}

#[test]
fn false_changes_nothing() {
    let mut nif = load("Bones");
    let limb = index(&nif, "Limb");
    let count = nif.blocks().count();
    let before = bones(&nif, "Limb");

    assert!(!nif.edit::<nifly::geometry::NiShape>(limb).is_some_and(|mut e| e.set_bone_list(["Bone1", "Missing"])));
    // Block 4 is the shape, and 99 doesn't exist.
    for ids in [&[1, 4][..], &[99], &[usize::MAX]] {
        assert!(!nif.edit::<nifly::geometry::NiShape>(limb).is_some_and(|mut e| e.set_bone_ids(ids)), "{ids:?}");
    }
    assert_eq!(bones(&nif, "Limb"), before);
    assert_eq!(nif.blocks().count(), count);

    // Not skinned, not a shape, or out of range.
    let mut nif = load("Cube");
    let cube = index(&nif, "Cube");
    assert!(!nif.edit::<nifly::geometry::NiShape>(cube).is_some_and(|mut e| e.set_bone_ids(&[0])));
    assert!(!nif.edit::<nifly::geometry::NiShape>(cube).is_some_and(|mut e| e.set_bone_list(["Scene Root"])));
    for i in [0, nif.blocks().count(), usize::MAX] {
        assert!(!nif.edit::<nifly::geometry::NiShape>(i).is_some_and(|mut e| e.set_bone_ids(&[])), "{i}");
        assert!(!nif.edit::<nifly::geometry::NiShape>(i).is_some_and(|mut e| e.set_bone_list(Vec::<&str>::new())), "{i}");
    }
}
