//! Tests for NifFile::delete_shape and NifFile::delete_shader.

use crate::common::{UPSTREAM, reload};
use std::fs;

use nifly::{
    file::{Index, NifFile},
    geometry::NiShape,
};

fn load(path: &str) -> NifFile {
    NifFile::load(fs::read(path).unwrap().as_slice(), None).unwrap()
}

fn index(nif: &NifFile, name: &str) -> Index {
    nif.shapes().indexed().find(|(_, s)| s.name() == name).unwrap().0
}

fn shape<'a>(nif: &'a NifFile, name: &str) -> &'a NiShape {
    nif.shapes().find(|s| s.name() == name).unwrap()
}

fn block_names(nif: &NifFile) -> Vec<String> {
    let mut names: Vec<_> = nif.blocks().map(|b| b.block_name().to_string()).collect();
    names.sort();
    names
}

/// Block names of the shape's property list and refs.
fn children(nif: &NifFile, name: &str) -> Vec<String> {
    nif.child_blocks(shape(nif, name)).map(|b| b.block_name().to_string()).collect()
}

#[test]
fn deletes_a_shape_and_what_it_owns() {
    let mut nif = load("assets/Cube.nif");
    assert!(nif.edit::<nifly::geometry::NiShape>(index(&nif, "Cube")).is_some_and(|e| e.delete()));
    assert_eq!(block_names(&nif), ["NiNode"]);
    assert_eq!(nif.root_node().unwrap().children().count(), 0);
    assert_eq!(reload(&mut nif).blocks().count(), 1);

    // A skinned shape: its data, skin and shader go; the bone nodes stay.
    let mut nif = load("assets/Bones.nif");
    assert!(nif.edit::<nifly::geometry::NiShape>(index(&nif, "Limb")).is_some_and(|e| e.delete()));
    assert_eq!(block_names(&nif), ["NiNode"; 4]);
    let saved = reload(&mut nif);
    assert_eq!(saved.nodes().map(|n| n.name().to_string()).collect::<Vec<_>>(), ["Scene Root", "Bone1", "Bone2", "Bone3"]);
}

#[test]
fn shared_blocks_stay() {
    // "" and "Shared" share their data; all three shapes share a texturing property.
    let mut nif = load("assets/Strips_MW.nif");
    let count = nif.blocks().count();
    assert!(nif.edit::<nifly::geometry::NiShape>(index(&nif, "")).is_some_and(|e| e.delete()));
    assert_eq!(nif.blocks().count(), count - 1);
    assert_eq!(children(&nif, "Shared"), ["NiTexturingProperty", "NiTriStripsData"]);
    assert_eq!(nif.view_of(shape(&nif, "Shared")).unwrap().triangles().unwrap().len(), 2);

    // Now the data is "Shared"'s alone, so it goes with it.
    assert!(nif.edit::<nifly::geometry::NiShape>(index(&nif, "Shared")).is_some_and(|e| e.delete()));
    assert_eq!(nif.blocks().count(), count - 3);
    assert_eq!(nif.shapes().count(), 1);
    assert_eq!(children(&reload(&mut nif), "Tri EditorMarker_box_02"), [
        "NiMaterialProperty",
        "NiTexturingProperty",
        "NiTriStripsData"
    ]);
}

#[test]
fn deletes_a_shader() {
    let mut nif = load("assets/Cube.nif");
    let cube = index(&nif, "Cube");
    assert!(nif.edit::<nifly::geometry::NiShape>(cube).is_some_and(|mut e| e.delete_shader()));
    assert!(nif.view_of(shape(&nif, "Cube")).unwrap().shader().is_none());
    // The shader and its texture set are gone; the geometry stays.
    assert_eq!(block_names(&nif), ["BSTriShape", "NiNode"]);
    assert_eq!(nif.view_of(shape(&nif, "Cube")).unwrap().triangles().unwrap().len(), 12);
    assert!(!nif.edit::<nifly::geometry::NiShape>(cube).is_some_and(|mut e| e.delete_shader()));
    assert_eq!(reload(&mut nif).blocks().count(), 2);

    // Morrowind: the material goes; texturing and stencil properties stay.
    let mut nif = load("assets/Stencil_MW.nif");
    let marker = index(&nif, "Tri EditorMarker_box_02");
    assert!(nif.edit::<nifly::geometry::NiShape>(marker).is_some_and(|mut e| e.delete_shader()));
    assert_eq!(children(&nif, "Tri EditorMarker_box_02"), ["NiStencilProperty", "NiTexturingProperty", "NiTriShapeData"]);
}

#[test]
fn shared_alpha_stays() {
    // Seven shapes share one alpha property; the last has its own.
    let mut nif = load(&format!("{UPSTREAM}/TestNifFile_OrderedNode_SE.nif"));
    let shapes: Vec<Index> = nif.shapes().indices().collect();
    let first = nif.shapes().next().unwrap();
    let alpha = nif.view_of(first).unwrap().alpha_property().unwrap().index();
    let (users, count) = (nif.block_ref_count(alpha, false), nif.blocks().count());
    assert_eq!(users, 7);

    assert!(nif.edit::<nifly::geometry::NiShape>(shapes[0]).is_some_and(|mut e| e.delete_shader()));
    assert!(nif.shapes().views().next().unwrap().alpha_property().is_none());
    // The shared alpha property stays for the other six.
    let second = nif.shapes().nth(1).unwrap();
    let alpha_after = nif.view_of(second).unwrap().alpha_property().unwrap().index();
    assert_eq!(nif.block_ref_count(alpha_after, false), 6);
    assert!(nif.blocks().count() < count);
}

#[test]
fn false_without_a_shape() {
    let mut nif = load("assets/Bones.nif");
    let count = nif.blocks().count();
    for i in [0, 1, count, usize::MAX] {
        assert!(!nif.edit::<nifly::geometry::NiShape>(i).is_some_and(|e| e.delete()), "{i}");
        assert!(!nif.edit::<nifly::geometry::NiShape>(i).is_some_and(|mut e| e.delete_shader()), "{i}");
    }
    assert_eq!(nif.blocks().count(), count);
}
