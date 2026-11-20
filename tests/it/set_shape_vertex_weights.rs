//! Tests for NifFile::set_shape_vertex_weights and NifFile::clear_shape_vertex_weights.

use crate::common::{load, reload, upstream};

use nifly::{
    file::{Index, NifFile},
    geometry::NiShape,
};

fn index(nif: &NifFile, name: &str) -> Index {
    nif.shapes()
        .indexed()
        .find(|(_, s)| s.name() == name)
        .unwrap()
        .0
}

/// The weight of each of `shape`'s bones on `vertex`, `None` where it has none.
fn vertex_weights(nif: &NifFile, shape: &NiShape, vertex: u16) -> Vec<Option<f32>> {
    (0..nif.view_of(shape).unwrap().bone_list().len())
        .map(|bone| {
            let weights = nif.view_of(shape).unwrap().bone_weights(bone);
            weights
                .iter()
                .find(|w| w.vertex == vertex)
                .map(|w| w.weight)
        })
        .collect()
}

fn weights_of(nif: &NifFile, name: &str, vertex: u16) -> Vec<Option<f32>> {
    vertex_weights(
        nif,
        nif.shapes().find(|s| s.name() == name).unwrap(),
        vertex,
    )
}

fn cylinders() -> NifFile {
    upstream("TestNifFile_Skinned_SE")
}

#[test]
fn sets_and_normalizes() {
    let mut nif = cylinders();
    let shape = index(&nif, "cylinder_1");
    assert_eq!(
        nif.view_of(nif.block_of::<NiShape>(shape).unwrap())
            .unwrap()
            .bone_list()
            .len(),
        2
    );

    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.set_vertex_weights(0, &[(1, 3.0), (0, 1.0)]))
    );
    assert_eq!(weights_of(&nif, "cylinder_1", 0), [Some(0.25), Some(0.75)]);
    assert_eq!(
        weights_of(&reload(&mut nif), "cylinder_1", 0),
        [Some(0.25), Some(0.75)]
    );

    // One bone, already normalized.
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.set_vertex_weights(135, &[(0, 1.0)]))
    );
    assert_eq!(weights_of(&nif, "cylinder_1", 135), [Some(1.0), None]);
}

#[test]
fn clearing() {
    let mut nif = cylinders();
    let shape = index(&nif, "cylinder_1");
    assert!(
        weights_of(&nif, "cylinder_1", 0)
            .iter()
            .any(Option::is_some)
    );

    // An empty slice, or all-zero weights, leaves the vertex unweighted.
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.set_vertex_weights(0, &[]))
    );
    assert_eq!(weights_of(&nif, "cylinder_1", 0), [None, None]);
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.set_vertex_weights(1, &[(0, 0.0), (1, 0.0)]))
    );
    assert_eq!(weights_of(&nif, "cylinder_1", 1), [None, None]);
}

#[test]
fn only_the_indexed_shape() {
    // Two shapes with the same name: only the one at the index changes.
    let mut nif = cylinders();
    let (first, second) = (index(&nif, "cylinder_1"), index(&nif, "cylinder_2"));
    nif.block_of_mut::<nifly::objects::NiObjectNet>(second)
        .unwrap()
        .set_name("cylinder_1");
    let before = vertex_weights(&nif, nif.block_of::<NiShape>(first).unwrap(), 0);

    assert!(
        nif.edit::<nifly::geometry::NiShape>(second)
            .is_some_and(|mut e| e.set_vertex_weights(0, &[(1, 1.0)]))
    );
    let first_shape = nif.block_of::<NiShape>(first).unwrap();
    let second_shape = nif.block_of::<NiShape>(second).unwrap();
    assert_eq!(vertex_weights(&nif, first_shape, 0), before);
    assert_eq!(vertex_weights(&nif, second_shape, 0), [None, Some(1.0)]);
}

#[test]
fn false_changes_nothing() {
    let mut nif = cylinders();
    let shape = index(&nif, "cylinder_1");
    let before = weights_of(&nif, "cylinder_1", 0);
    let five = [(0, 1.0), (1, 1.0), (0, 1.0), (1, 1.0), (0, 1.0)];
    for (vertex, weights) in [
        (136, &[(0, 1.0)][..]), // past the last vertex
        (0, &five[..]),         // more than 4
        (0, &[(2, 1.0)][..]),   // only 2 bones
        (0, &[(0, -1.0)][..]),
        (0, &[(0, f32::NAN)][..]),
        (0, &[(0, f32::INFINITY)][..]),
        (0, &[(usize::MAX, 1.0)][..]),
    ] {
        assert!(
            !nif.edit::<nifly::geometry::NiShape>(shape)
                .is_some_and(|mut e| e.set_vertex_weights(vertex, weights)),
            "{vertex} {weights:?}"
        );
    }
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.set_vertex_weights(usize::MAX, &[]))
    );
    assert_eq!(weights_of(&nif, "cylinder_1", 0), before);

    // Not a shape, or out of range.
    let count = nif.blocks().count();
    for i in [0, count, usize::MAX] {
        assert!(
            !nif.edit::<nifly::geometry::NiShape>(i)
                .is_some_and(|mut e| e.set_vertex_weights(0, &[])),
            "{i}"
        );
    }

    // An unskinned BSTriShape, and an NiTriShape, whose weights are per bone.
    let mut nif = load("Cube");
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(index(&nif, "Cube"))
            .is_some_and(|mut e| e.set_vertex_weights(0, &[]))
    );
    let mut nif = load("Bones");
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(index(&nif, "Limb"))
            .is_some_and(|mut e| e.set_vertex_weights(0, &[(0, 1.0)]))
    );

    // Starfield geometry without its mesh file loaded has no vertices.
    let mut nif = upstream("TestNifFile_SF");
    let geometry = nif.shapes().indices().next().unwrap();
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(geometry)
            .is_some_and(|mut e| e.set_vertex_weights(0, &[]))
    );
}

#[test]
fn clear_all() {
    let mut nif = cylinders();
    let (first, second) = (index(&nif, "cylinder_1"), index(&nif, "cylinder_2"));
    let other = weights_of(&nif, "cylinder_2", 0);
    assert!(
        nif.edit::<nifly::geometry::NiShape>(first)
            .is_some_and(|mut e| e.clear_vertex_weights())
    );

    let s = nif.block_of::<NiShape>(first).unwrap();
    assert!(
        (0..nif.view_of(s).unwrap().bone_list().len()).all(|bone| nif
            .view_of(s)
            .unwrap()
            .bone_weights(bone)
            .is_empty())
    );
    // Other shapes keep theirs.
    assert_eq!(weights_of(&nif, "cylinder_2", 0), other);

    // Weights can be set again, and the cleared shape stays cleared through a save.
    assert!(
        nif.edit::<nifly::geometry::NiShape>(first)
            .is_some_and(|mut e| e.set_vertex_weights(7, &[(0, 1.0)]))
    );
    let saved = reload(&mut nif);
    let s = saved.shapes().find(|s| s.name() == "cylinder_1").unwrap();
    assert_eq!(saved.view_of(s).unwrap().bone_weights(0).len(), 1);
    assert!(saved.view_of(s).unwrap().bone_weights(1).is_empty());
    assert!(
        nif.edit::<nifly::geometry::NiShape>(second)
            .is_some_and(|mut e| e.clear_vertex_weights())
    );
}

#[test]
fn clear_false_without_vertex_weights() {
    let mut nif = load("Cube");
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(index(&nif, "Cube"))
            .is_some_and(|mut e| e.clear_vertex_weights())
    );
    let mut nif = load("Bones");
    let before = nif.shapes().views().next().unwrap().bone_weights(0);
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(index(&nif, "Limb"))
            .is_some_and(|mut e| e.clear_vertex_weights())
    );
    assert_eq!(nif.shapes().views().next().unwrap().bone_weights(0), before);

    let mut nif = cylinders();
    let count = nif.blocks().count();
    for i in [0, count, usize::MAX] {
        assert!(
            !nif.edit::<nifly::geometry::NiShape>(i)
                .is_some_and(|mut e| e.clear_vertex_weights()),
            "{i}"
        );
    }
}
