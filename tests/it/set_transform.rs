//! Tests for NifFile::set_node_transform_to_parent.

use crate::common::{assert_close, first, load, reload};

use nifly::{
    file::{NifFile, NifVersion},
    geometry::NiShape,
    object3d::{Matrix3, Transform, Vec3, vec3},
};

const EPS: f32 = 1e-4;

fn transform(t: Vec3, scale: f32) -> Transform {
    Transform::new(t, Matrix3::IDENTITY, scale)
}

/// 90° about Z.
fn rotated(t: Vec3, scale: f32) -> Transform {
    Transform::new(
        t,
        Matrix3::from_rows(
            vec3(0.0, -1.0, 0.0),
            vec3(1.0, 0.0, 0.0),
            vec3(0.0, 0.0, 1.0),
        ),
        scale,
    )
}

#[test]
fn set_and_read_back() {
    let mut nif = load("Icosphere");
    let new = rotated(vec3(1.0, 2.0, 3.0), 2.0);

    assert!(nif.set_node_transform_to_parent("Indirect2", new, false));
    assert_eq!(nif.node_transform_to_parent("Indirect2"), Some(new));

    let node = nif.nodes().find(|n| n.name() == "Indirect2").unwrap();
    assert_eq!(node.transform(), new);
}

#[test]
fn global_follows_the_new_transform() {
    // Root -> Arm -> Hand, built from scratch so every transform is known.
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let identity = transform(vec3(0.0, 0.0, 0.0), 1.0);
    let arm = nif.add_node("Arm", identity, None).unwrap();
    nif.add_node("Hand", transform(vec3(1.0, 0.0, 0.0), 1.0), Some(arm))
        .unwrap();

    // Rotate Arm 90° about Z, scale it by 2 and move it up.
    assert!(nif.set_node_transform_to_parent("Arm", rotated(vec3(0.0, 0.0, 5.0), 2.0), false));

    // Hand's (1, 0, 0) is scaled to (2, 0, 0), rotated to (0, 2, 0), then offset by (0, 0, 5).
    let hand = nif.node_transform_to_global("Hand").unwrap();
    assert_close!(hand.translation, vec3(0.0, 2.0, 5.0), EPS);
    assert!((hand.scale - 2.0).abs() < EPS);
}

#[test]
fn missing_name_changes_nothing() {
    let mut nif = load("Icosphere");
    let before: Vec<_> = nif.nodes().map(|n| n.transform()).collect();

    let new = transform(vec3(9.0, 9.0, 9.0), 1.0);
    assert!(!nif.set_node_transform_to_parent("missing", new, false));
    // The shape has a name, but only nodes are searched.
    assert!(!nif.set_node_transform_to_parent("IcosphereMesh", new, false));

    let after: Vec<_> = nif.nodes().map(|n| n.transform()).collect();
    assert_eq!(after, before);
    let shape = first::<NiShape>(&nif);
    assert_ne!(shape.transform(), new);
}

#[test]
fn root_children_only() {
    let mut nif = load("Icosphere");
    let new = transform(vec3(4.0, 5.0, 6.0), 1.0);
    let indirect2_before = nif.node_transform_to_parent("Indirect2").unwrap();

    // Indirect2 is a grandchild of the root, so it isn't found.
    assert!(!nif.set_node_transform_to_parent("Indirect2", new, true));
    assert_eq!(
        nif.node_transform_to_parent("Indirect2"),
        Some(indirect2_before)
    );

    // Indirect1 is a direct child of the root.
    assert!(nif.set_node_transform_to_parent("Indirect1", new, true));
    assert_eq!(nif.node_transform_to_parent("Indirect1"), Some(new));
}

#[test]
fn root_children_only_skips_deeper_duplicate() {
    // Two nodes named "Dup": one deep (added first, so first in block order) and one under the root.
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let identity = transform(vec3(0.0, 0.0, 0.0), 1.0);
    let parent = nif.add_node("Parent", identity, None).unwrap();
    let deep = nif.add_node("Dup", identity, Some(parent)).unwrap();
    let shallow = nif.add_node("Dup", identity, None).unwrap();
    assert!(deep < shallow);

    let new = transform(vec3(1.0, 1.0, 1.0), 1.0);
    let get = |nif: &NifFile, i| {
        nif.block(i)
            .unwrap()
            .downcast::<nifly::nodes::NiNode>()
            .unwrap()
            .transform()
    };

    // Without the restriction, the first in block order (the deep one) is set.
    assert!(nif.set_node_transform_to_parent("Dup", new, false));
    assert_eq!(get(&nif, deep), new);
    assert_eq!(get(&nif, shallow), identity);

    // With it, only the root's child is considered.
    let other = transform(vec3(2.0, 2.0, 2.0), 1.0);
    assert!(nif.set_node_transform_to_parent("Dup", other, true));
    assert_eq!(get(&nif, deep), new);
    assert_eq!(get(&nif, shallow), other);
}

#[test]
fn set_transform_survives_save() {
    let mut nif = load("Bones");
    let new = rotated(vec3(0.5, -0.5, 2.0), 1.5);
    assert!(nif.set_node_transform_to_parent("Bone3", new, false));

    let saved = reload(&mut nif);
    assert_eq!(saved.node_transform_to_parent("Bone3"), Some(new));
}
