//! Tests for NifFile::clone_shape, clone_shape_from, clone_node, clone_node_from and
//! clone_children.

use crate::common::{UPSTREAM, load, reload};
use std::{fs, ptr};

use nifly::{
    file::{Index, NifFile},
    geometry::NiShape,
    nodes::NiNode,
};

fn skinned_se() -> NifFile {
    NifFile::load(fs::read(format!("{UPSTREAM}/TestNifFile_Skinned_SE.nif")).unwrap().as_slice(), None).unwrap()
}

fn shape<'a>(nif: &'a NifFile, name: &str) -> &'a NiShape {
    nif.shapes().find(|s| s.name() == name).unwrap()
}

fn index(nif: &NifFile, name: &str) -> Index {
    nif.shapes().indexed().find(|(_, s)| s.name() == name).unwrap().0
}

fn node(nif: &NifFile, name: &str) -> Index {
    nif.nodes().indexed().find(|(_, n)| n.name() == name).unwrap().0
}

fn bones(nif: &NifFile, name: &str) -> Vec<String> {
    nif.view_of(shape(nif, name)).unwrap().bone_list().iter().map(ToString::to_string).collect()
}

fn parent_name(nif: &NifFile, name: &str) -> Option<String> {
    nif.view_of(shape(nif, name)).and_then(|v| v.parent()).map(|p| p.name().to_string())
}

#[test]
fn clones_a_shape_in_place() {
    let mut nif = load("Bones");
    let limb = index(&nif, "Limb");
    let count = nif.blocks().count();
    let copy = nif.edit::<nifly::geometry::NiShape>(limb).and_then(|mut e| e.clone_shape("Limb Copy")).unwrap();
    assert!(nif.blocks().count() > count);
    assert_eq!(nif.block_of::<NiShape>(copy).unwrap().name(), "Limb Copy");

    let saved = reload(&mut nif);
    for nif in [&nif, &saved] {
        let (a, b) = (shape(nif, "Limb"), shape(nif, "Limb Copy"));
        // Same parent, bones, geometry and weights...
        assert_eq!(parent_name(nif, "Limb Copy"), parent_name(nif, "Limb"));
        assert_eq!(bones(nif, "Limb Copy"), bones(nif, "Limb"));
        assert_eq!(nif.view_of(b).unwrap().bone_ids(), nif.view_of(a).unwrap().bone_ids());
        assert_eq!(nif.view_of(b).unwrap().vertices(), nif.view_of(a).unwrap().vertices());
        assert_eq!(nif.view_of(b).unwrap().triangles(), nif.view_of(a).unwrap().triangles());
        assert_eq!(nif.view_of(b).unwrap().bone_weights(0), nif.view_of(a).unwrap().bone_weights(0));
        // ...in blocks of its own.
        assert!(!ptr::eq(nif.view_of(a).unwrap().geometry_data().unwrap(), nif.view_of(b).unwrap().geometry_data().unwrap()));
    }
}

#[test]
fn clones_a_shape_from_another_file() {
    let source = skinned_se();
    let mut nif = load("Cube");
    let count = (nif.blocks().count(), source.blocks().count());
    let copy = nif.clone_shape_from(source.view_of(shape(&source, "cylinder_1")).unwrap(), "Cylinder").unwrap();
    assert_eq!(nif.block_of::<NiShape>(copy).unwrap().name(), "Cylinder");
    assert_eq!(source.blocks().count(), count.1);
    assert!(nif.blocks().count() > count.0);

    let saved = reload(&mut nif);
    for nif in [&nif, &saved] {
        assert_eq!(parent_name(nif, "Cylinder").as_deref(), Some("Scene Root"));
        // The source's bone nodes are copied in and bound by name.
        assert_eq!(bones(nif, "Cylinder"), ["BoneA", "BoneB"]);
        assert!(nif.nodes().any(|n| n.name() == "BoneA"));
        let (a, b) = (shape(&source, "cylinder_1"), shape(nif, "Cylinder"));
        assert_eq!(nif.view_of(b).unwrap().vertices(), source.view_of(a).unwrap().vertices());
        assert_eq!(nif.view_of(b).unwrap().triangles(), source.view_of(a).unwrap().triangles());
        assert_eq!(nif.view_of(b).unwrap().bone_weights(1), source.view_of(a).unwrap().bone_weights(1));
        // The original cube is untouched.
        assert_eq!(nif.view_of(shape(nif, "Cube")).unwrap().triangles().unwrap().len(), 12);
    }

    // A second shape reuses the bones already there.
    let nodes = nif.nodes().count();
    nif.clone_shape_from(source.view_of(shape(&source, "cylinder_2")).unwrap(), "Cylinder 2").unwrap();
    assert_eq!(nif.nodes().count(), nodes);
    assert_eq!(bones(&nif, "Cylinder 2"), ["BoneA", "BoneB"]);
}

#[test]
fn clones_nodes() {
    let mut nif = load("Bones");
    let bone2 = node(&nif, "Bone2");
    nif.add_node("Tip", nifly::object3d::Transform::IDENTITY, Some(bone2)).unwrap();
    let original = nif.block_of::<NiNode>(bone2).unwrap();
    let (transform, children) = (original.transform(), original.child_count());
    assert!(children > 0);

    let copy = nif.edit::<nifly::nodes::NiNode>(bone2).map(|mut e| e.clone_node()).unwrap();
    let c = nif.block_of::<NiNode>(copy).unwrap();
    assert_eq!((c.name(), c.transform(), c.child_count()), ("Bone2".into(), transform, 0));
    assert!(nif.view_of(c).and_then(|v| v.parent()).is_none());
    // The original keeps its children.
    assert_eq!(nif.block_of::<NiNode>(bone2).unwrap().child_count(), children);

    // From another file of the same game.
    let source = load("Bones");
    let mut other = NifFile::load(fs::read("assets/Bones.nif").unwrap().as_slice(), None).unwrap();
    let copy = other.clone_node_from(source.view_of(source.nodes().find(|n| n.name() == "Bone3").unwrap()).unwrap()).unwrap();
    assert_eq!(other.block_of::<NiNode>(copy).unwrap().name(), "Bone3");
}

#[test]
fn versions_of_one_game() {
    // Fallout 4 stream 130 into stream 132, saved in the target's version.
    let read = |name: &str| NifFile::load(fs::read(format!("{UPSTREAM}/{name}.nif")).unwrap().as_slice(), None).unwrap();
    let source = read("TestNifFile_Static_FO4");
    let mut nif = read("TestNifFile_Static_FO4_132");
    assert_ne!(source.version(), nif.version());
    let s = source.shapes().next().unwrap();
    let copy = nif.clone_shape_from(source.view_of(s).unwrap(), "Copy").unwrap();
    assert!(nif.clone_node_from(source.view_of(source.nodes().next().unwrap()).unwrap()).is_some());
    let vertices = source.view_of(s).unwrap().vertices();
    assert_eq!(nif.view_of(nif.block_of::<NiShape>(copy).unwrap()).unwrap().vertices(), vertices);
    let saved = reload(&mut nif);
    assert_eq!(saved.version(), read("TestNifFile_Static_FO4_132").version());
    assert_eq!(saved.view_of(shape(&saved, "Copy")).unwrap().vertices(), vertices);
}

#[test]
fn different_games_are_refused() {
    // Skyrim LE into Skyrim SE.
    let source = load("Bones");
    let mut nif = load("Cube");
    let count = nif.blocks().count();
    assert_eq!(nif.clone_shape_from(source.view_of(shape(&source, "Limb")).unwrap(), "Limb"), None);
    assert_eq!(nif.clone_node_from(source.view_of(source.nodes().next().unwrap()).unwrap()), None);
    assert_eq!(nif.blocks().count(), count);
}

#[test]
fn clones_children() {
    // The cube's shader becomes a block of its own; the original is left unreferenced.
    let mut nif = load("Cube");
    let cube = index(&nif, "Cube");
    let shader_index = |nif: &NifFile| nif.view_of(shape(nif, "Cube")).unwrap().shader().unwrap().index();
    let before = shader_index(&nif);
    let count = nif.blocks().count();
    assert!(nif.clone_children(cube));
    let after = shader_index(&nif);
    assert_ne!(after, before);
    assert!(after >= count);
    assert_eq!(nif.block_ref_count(before, false), 0);
    let saved = reload(&mut nif);
    assert_eq!(saved.blocks().count(), count);
}

#[test]
fn none_without_a_source_block() {
    let mut nif = load("Bones");
    let count = nif.blocks().count();
    let root = node(&nif, "Scene Root");
    assert_eq!(nif.edit::<nifly::geometry::NiShape>(root).and_then(|mut e| e.clone_shape("x")), None);
    assert_eq!(nif.edit::<nifly::nodes::NiNode>(index(&nif, "Limb")).map(|mut e| e.clone_node()), None);
    for i in [count, usize::MAX] {
        assert_eq!(nif.edit::<nifly::geometry::NiShape>(i).and_then(|mut e| e.clone_shape("x")), None, "{i}");
        assert_eq!(nif.edit::<nifly::nodes::NiNode>(i).map(|mut e| e.clone_node()), None, "{i}");
        assert!(!nif.clone_children(i), "{i}");
    }
    assert_eq!(nif.blocks().count(), count);
}
