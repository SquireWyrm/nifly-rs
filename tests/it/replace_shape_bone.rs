//! Tests for NifFile::replace_shape_bone.

use crate::common::{load, reload};

use nifly::{
    file::{Index, NifFile},
    geometry::NiShape,
    object3d::Transform,
};

fn limb(nif: &NifFile) -> (Index, &NiShape) {
    nif.shapes()
        .indexed()
        .find(|(_, s)| s.name() == "Limb")
        .unwrap()
}

fn bones(nif: &NifFile) -> Vec<String> {
    nif.view_of(limb(nif).1)
        .unwrap()
        .bone_list()
        .iter()
        .map(ToString::to_string)
        .collect()
}

#[test]
fn replaces_keeping_position() {
    let mut nif = load("Bones");
    let (shape, s) = limb(&nif);
    let weights = nif.view_of(s).unwrap().bone_weights(1);
    let root = nif.root_index().unwrap();
    let new = nif
        .add_node("NewBone", Transform::IDENTITY, Some(root))
        .unwrap();

    // Bone2 (block 2) is the second bone.
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.replace_bone(2, new))
    );
    let saved = reload(&mut nif);
    for nif in [&nif, &saved] {
        assert_eq!(bones(nif), ["Bone1", "NewBone", "Bone3"]);
        assert_eq!(nif.view_of(limb(nif).1).unwrap().bone_weights(1), weights);
    }
}

#[test]
fn only_the_first_match() {
    let mut nif = load("Bones");
    let shape = limb(&nif).0;
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.set_bone_ids(&[1, 2, 1]))
    );
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.replace_bone(1, 3))
    );
    assert_eq!(bones(&nif), ["Bone3", "Bone2", "Bone1"]);
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.replace_bone(1, 3))
    );
    assert_eq!(bones(&nif), ["Bone3", "Bone2", "Bone3"]);
}

#[test]
fn false_changes_nothing() {
    let mut nif = load("Bones");
    let shape = limb(&nif).0;
    let before = bones(&nif);
    for (old, new) in [
        (0, 3),  // the root isn't a bone of the shape
        (2, 4),  // block 4 is the shape, not a node
        (2, 99), // no such block
        (2, usize::MAX),
        (usize::MAX, 3),
    ] {
        assert!(
            !nif.edit::<nifly::geometry::NiShape>(shape)
                .is_some_and(|mut e| e.replace_bone(old, new)),
            "{old} -> {new}"
        );
    }
    assert_eq!(bones(&nif), before);
    for i in [0, nif.blocks().count(), usize::MAX] {
        assert!(
            !nif.edit::<nifly::geometry::NiShape>(i)
                .is_some_and(|mut e| e.replace_bone(2, 3)),
            "{i}"
        );
    }

    // An unskinned shape.
    let mut nif = load("Cube");
    let cube = nif.shapes().indices().next().unwrap();
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(cube)
            .is_some_and(|mut e| e.replace_bone(0, 0))
    );
}
