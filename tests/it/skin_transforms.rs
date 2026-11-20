//! Tests for NifFile::shape_transform_global_to_skin and the skin-to-bone getter and setter.

use crate::common::load;
use std::fs;

use nifly::{
    file::{NifFile, NifVersion},
    object3d::{Matrix3, Transform, Vec3},
};

const EPS: f32 = 1e-5;

fn assert_close(a: Transform, b: Transform) {
    assert!(
        a.translation.distance(b.translation) < EPS,
        "{a:?} != {b:?}"
    );
    for (r, s) in a.rotation.rows.into_iter().zip(b.rotation.rows) {
        assert!(r.distance(s) < EPS, "{a:?} != {b:?}");
    }
    assert!((a.scale - b.scale).abs() < EPS, "{a:?} != {b:?}");
}

/// In the bind pose, each bone's global transform times its skin-to-bone transform is the
/// skin-to-global transform.
fn assert_bind_pose(nif: &NifFile) {
    let shape = nif.shapes().next().unwrap();
    let skin_to_global = nif
        .view_of(shape)
        .unwrap()
        .transform_global_to_skin()
        .unwrap()
        .inverse()
        .unwrap();
    let bones = nif.view_of(shape).unwrap().bone_list();
    assert_eq!(bones.len(), 3);
    for (i, name) in bones.iter().enumerate() {
        let bone_global = nif.node_transform_to_global(name).unwrap();
        let skin_to_bone = nif
            .view_of(shape)
            .unwrap()
            .transform_skin_to_bone(i)
            .unwrap();
        assert_close(bone_global * skin_to_bone, skin_to_global);
    }
}

#[test]
fn bones_values() {
    let nif = load("Bones");
    let shape = nif.shapes().next().unwrap();
    assert_close(
        nif.view_of(shape)
            .unwrap()
            .transform_global_to_skin()
            .unwrap(),
        Transform::IDENTITY,
    );

    // The bones stand along Z at 0, 0.75 and 1.5, each rotated 90° about X so its Y axis points
    // up. Skin-to-bone undoes that: global Z becomes bone Y.
    let to_bone = Matrix3::from_rows(Vec3::X, Vec3::Z, -Vec3::Y);
    for (i, height) in [0.0, 0.75, 1.5].into_iter().enumerate() {
        let skin_to_bone = nif
            .view_of(shape)
            .unwrap()
            .transform_skin_to_bone(i)
            .unwrap();
        let expected = Transform::new(Vec3::new(0.0, -height, 0.0), to_bone, 1.0);
        assert_close(skin_to_bone, expected);
        assert_close(
            nif.node_transform_to_global(format!("Bone{}", i + 1))
                .unwrap()
                * skin_to_bone,
            Transform::IDENTITY,
        );
    }
}

#[test]
fn bind_pose_relation() {
    assert_bind_pose(&load("Bones"));
}

#[test]
fn survives_conversion_to_se() {
    let mut nif = load("Bones");
    nif.optimize_for(NifVersion::SKYRIM_SE, None).unwrap();
    assert_bind_pose(&nif);
}

#[test]
fn out_of_range_bone() {
    let nif = load("Bones");
    let shape = nif.shapes().next().unwrap();
    assert!(
        nif.view_of(shape)
            .unwrap()
            .transform_skin_to_bone(3)
            .is_none()
    );
    assert!(
        nif.view_of(shape)
            .unwrap()
            .transform_skin_to_bone(usize::MAX)
            .is_none()
    );
}

#[test]
fn unskinned_shapes() {
    for asset in ["Cone", "Cube", "Icosphere", "Suzanne"] {
        let nif = load(asset);
        let shape = nif.shapes().next().unwrap();
        assert!(
            nif.view_of(shape)
                .unwrap()
                .transform_global_to_skin()
                .is_none(),
            "{asset}"
        );
        assert!(
            nif.view_of(shape)
                .unwrap()
                .transform_skin_to_bone(0)
                .is_none(),
            "{asset}"
        );
    }
}

fn sample() -> Transform {
    Transform::new(
        Vec3::new(0.25, -1.0, 2.0),
        Matrix3::from_rows(Vec3::Y, -Vec3::X, Vec3::Z),
        1.5,
    )
}

fn shape_index(nif: &NifFile) -> usize {
    nif.block_index(nif.shapes().next().unwrap()).unwrap()
}

#[test]
fn set_skin_to_bone_ni_skin_data() {
    let mut nif = load("Bones");
    let shape = shape_index(&nif);
    let before: Vec<_> = (0..3)
        .map(|i| {
            nif.shapes()
                .views()
                .next()
                .unwrap()
                .transform_skin_to_bone(i)
                .unwrap()
        })
        .collect();

    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.set_transform_skin_to_bone(1, sample()))
    );
    let s = nif.shapes().next().unwrap();
    assert_close(
        nif.view_of(s).unwrap().transform_skin_to_bone(1).unwrap(),
        sample(),
    );
    // The other bones are untouched.
    assert_eq!(
        nif.view_of(s).unwrap().transform_skin_to_bone(0),
        Some(before[0])
    );
    assert_eq!(
        nif.view_of(s).unwrap().transform_skin_to_bone(2),
        Some(before[2])
    );

    // It's stored in the NiSkinData block and survives a save.
    let mut cur = std::io::Cursor::new(Vec::new());
    nif.save(&mut cur, None).unwrap();
    let saved = NifFile::load(cur.into_inner().as_slice(), None).unwrap();
    let data = saved.blocks_of::<nifly::skin::NiSkinData>().next().unwrap();
    assert_close(data.transform_skin_to_bone(1).unwrap(), sample());
}

#[test]
fn set_skin_to_bone_bs_skin_bone_data() {
    // Fallout 4 stores bone transforms in BSSkin::BoneData.
    let path = "nifly-sys/vendor/nifly/tests/input/TestNifFile_Skinned_FO4.nif";
    let mut nif = NifFile::load(fs::read(path).unwrap().as_slice(), None).unwrap();
    let shape = shape_index(&nif);
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.set_transform_skin_to_bone(0, sample()))
    );
    assert_close(
        nif.shapes()
            .views()
            .next()
            .unwrap()
            .transform_skin_to_bone(0)
            .unwrap(),
        sample(),
    );
}

#[test]
fn set_skin_to_bone_rejects() {
    let mut nif = load("Bones");
    let shape = shape_index(&nif);
    let root = nif.root_node().unwrap().index();
    let save = |nif: &mut NifFile| {
        let mut cur = std::io::Cursor::new(Vec::new());
        let options = nifly::file::SaveOptions {
            optimize: false,
            sort_blocks: false,
        };
        nif.save(&mut cur, Some(options)).unwrap();
        cur.into_inner()
    };
    let before = save(&mut nif);
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.set_transform_skin_to_bone(3, sample())),
        "bone out of range"
    );
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.set_transform_skin_to_bone(usize::MAX, sample()))
    );
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(root)
            .is_some_and(|mut e| e.set_transform_skin_to_bone(0, sample())),
        "not a shape"
    );
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(usize::MAX)
            .is_some_and(|mut e| e.set_transform_skin_to_bone(0, sample()))
    );
    assert_eq!(save(&mut nif), before);

    // An unskinned shape.
    let mut cube = load("Cube");
    let shape = shape_index(&cube);
    assert!(
        !cube
            .edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.set_transform_skin_to_bone(0, sample()))
    );
}

const FO4: &str = "nifly-sys/vendor/nifly/tests/input/TestNifFile_Skinned_FO4.nif";

fn node_index(nif: &NifFile, name: &str) -> usize {
    nif.block_index(
        nif.find_block_by_name::<nifly::nodes::NiNode>(name)
            .unwrap(),
    )
    .unwrap()
}

/// `(bone_to_global · skin_to_bone)⁻¹` for bone `i` of the first shape.
fn expected_from_bone(nif: &NifFile, i: usize) -> Transform {
    let shape = nif.shapes().next().unwrap();
    let name = &nif.view_of(shape).unwrap().bone_list()[i];
    let global = nif.node_transform_to_global(name).unwrap();
    (global
        * nif
            .view_of(shape)
            .unwrap()
            .transform_skin_to_bone(i)
            .unwrap())
    .inverse()
    .unwrap()
}

#[test]
fn calc_returns_stored_transform() {
    let mut nif = load("Bones");
    // Moving a bone doesn't matter: the stored transform wins.
    nif.set_node_transform_to_parent("Bone1", sample(), false);
    let shape = nif.shapes().next().unwrap();
    assert_eq!(
        nif.view_of(shape).unwrap().calc_transform_global_to_skin(),
        nif.view_of(shape).unwrap().transform_global_to_skin()
    );
}

#[test]
fn calc_from_first_bone() {
    let mut nif = NifFile::load(fs::read(FO4).unwrap().as_slice(), None).unwrap();
    // Move both bones away from the bind pose, so the result isn't the identity.
    nif.set_node_transform_to_parent("BoneA", sample(), false);
    nif.set_node_transform_to_parent(
        "BoneB",
        Transform::new(Vec3::new(3.0, 0.0, 1.0), Matrix3::IDENTITY, 0.5),
        false,
    );
    let shape = nif.shapes().next().unwrap();
    assert_eq!(nif.view_of(shape).unwrap().transform_global_to_skin(), None);

    // The first bone is used. With the bones moved apart, the second would give another result.
    let calc = nif
        .view_of(shape)
        .unwrap()
        .calc_transform_global_to_skin()
        .unwrap();
    assert_close(calc, expected_from_bone(&nif, 0));
    let from_b = expected_from_bone(&nif, 1);
    assert!(
        calc.translation.distance(from_b.translation) > 0.1,
        "{calc:?} vs {from_b:?}"
    );

    // Renaming the bone's node renames the bone, so it's still found.
    let bone_a = node_index(&nif, "BoneA");
    nif.block_of_mut::<nifly::objects::NiObjectNet>(bone_a)
        .unwrap()
        .set_name("Renamed");
    let shape = nif.shapes().next().unwrap();
    assert_eq!(nif.view_of(shape).unwrap().bone_list()[0], "Renamed");
    assert_close(
        nif.view_of(shape)
            .unwrap()
            .calc_transform_global_to_skin()
            .unwrap(),
        calc,
    );
}

#[test]
fn calc_unskinned_is_none() {
    for asset in ["Cube", "Suzanne"] {
        let nif = load(asset);
        assert_eq!(
            nif.shapes()
                .views()
                .next()
                .unwrap()
                .calc_transform_global_to_skin(),
            None,
            "{asset}"
        );
    }
}
