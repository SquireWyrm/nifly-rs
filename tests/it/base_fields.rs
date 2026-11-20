//! Tests for the base classes' fields and lists: flags, transforms, velocity, bounding volumes,
//! group IDs, children, properties, extra data, controllers and collision.

use std::io::Cursor;

use nifly::{
    extra_data::{ExtraInteger, ExtraString, NiExtraData},
    file::{Index, NifFile, NifVersion, SaveOptions},
    geometry::NiShape,
    nodes::NiNode,
    object3d::{BoundingSphere, BoundingVolume, Matrix3, Transform, Vec3},
    objects::{NiAvObject, NiObject, NiObjectNet},
    shaders::NiShader,
};

use crate::common::{load, upstream};

/// Saves without optimizing or sorting, so block indices stay.
fn reload(nif: &mut NifFile) -> NifFile {
    let mut cur = Cursor::new(Vec::new());
    nif.save(&mut cur, Some(SaveOptions { optimize: false, sort_blocks: false })).unwrap();
    NifFile::load(cur.into_inner().as_slice(), None).unwrap()
}

fn root(nif: &NifFile) -> Index {
    nif.root_node().unwrap().index()
}

fn volume() -> BoundingVolume {
    let v = |x| Vec3::new(x, 2.0, 3.0);
    BoundingVolume::Union(vec![
        BoundingVolume::Sphere(BoundingSphere::new(v(1.0), 4.0)),
        BoundingVolume::Box { center: v(5.0), axes: [v(6.0), v(7.0), v(8.0)], extents: [9.0, 10.0, 11.0] },
        BoundingVolume::Capsule { center: v(12.0), origin: v(13.0), extent: 14.0, radius: 15.0 },
        BoundingVolume::Union(vec![BoundingVolume::Base, BoundingVolume::Other(3)]),
        BoundingVolume::HalfSpace { normal: v(16.0), constant: 17.0, center: v(18.0) },
    ])
}

#[test]
fn object_fields_round_trip() {
    let mut nif = NifFile::create(NifVersion::MORROWIND);
    let root = root(&nif);
    let transform = Transform::new(Vec3::new(1.0, 2.0, 3.0), Matrix3::IDENTITY, 2.0);
    let mut node = nif.block_of_mut::<NiAvObject>(root).unwrap();
    node.as_mut().set_raw_flags(0x1234);
    node.as_mut().set_transform(transform);
    node.as_mut().set_velocity(Vec3::new(0.5, 0.0, -1.0));
    node.as_mut().set_bounding_volume(Some(&volume()));
    let saved = reload(&mut nif);
    let node = saved.block_of::<NiAvObject>(root).unwrap();
    assert_eq!((node.raw_flags(), node.transform(), node.velocity()), (0x1234, transform, Vec3::new(0.5, 0.0, -1.0)));
    assert_eq!(node.bounding_volume(), Some(volume()));

    // Removing it, and files before Fallout 3 keep 16 bits of the flags.
    let mut node = nif.block_of_mut::<NiAvObject>(root).unwrap();
    node.as_mut().set_bounding_volume(None);
    node.set_raw_flags(0x1_0005);
    let node = reload(&mut nif);
    let node = node.block_of::<NiAvObject>(root).unwrap();
    assert_eq!((node.bounding_volume(), node.raw_flags()), (None, 5));
}

#[test]
fn group_id_by_version() {
    for (version, saved) in [(NifVersion::new(0x0A01_006A, 10, 0), 7), (NifVersion::OBLIVION, 0)] {
        let mut nif = NifFile::create(version);
        let root = root(&nif);
        nif.block_mut(root).unwrap().set_group_id(7);
        assert_eq!(nif.block(root).unwrap().group_id(), 7);
        assert_eq!(reload(&mut nif).block(root).unwrap().group_id(), saved, "{version:?}");
    }
}

#[test]
fn shape_transform_and_flags() {
    let mut nif = load("Cube");
    let shape = nif.shapes().indices().next().unwrap();
    let transform = Transform::new(Vec3::new(0.0, 0.0, 5.0), Matrix3::IDENTITY, 0.5);
    let mut block = nif.block_of_mut::<NiAvObject>(shape).unwrap();
    block.as_mut().set_transform(transform);
    block.set_raw_flags(0x0008_0000);
    let saved = reload(&mut nif);
    let shape = saved.block_of::<NiAvObject>(shape).unwrap();
    assert_eq!((shape.transform(), shape.raw_flags()), (transform, 0x0008_0000));
}

#[test]
fn set_children() {
    // Icosphere: root 0 -> Indirect1 1 -> Indirect2 2 -> shape 3.
    let mut nif = load("Icosphere");
    let mut indirect2 = nif.edit::<NiNode>(2).unwrap();
    assert!(!indirect2.set_children(&[0]), "an ancestor");
    assert!(!indirect2.set_children(&[2]), "itself");
    let extra = indirect2.assign_extra_data(ExtraInteger("", 1)).unwrap();
    assert!(!indirect2.set_children(&[3, extra]), "not an NiAVObject");
    assert!(!indirect2.set_children(&[usize::MAX]));
    assert_eq!(indirect2.view().children().indices().collect::<Vec<_>>(), [3]);

    // Moving the shape up keeps it under Indirect2 too.
    let mut root = nif.edit::<NiNode>(0).unwrap();
    assert!(root.set_children(&[3, 1]));
    let saved = reload(&mut nif);
    let children = |i| saved.view::<NiNode>(i).unwrap().children().indices().collect::<Vec<_>>();
    assert_eq!((children(0), children(2)), (vec![3, 1], vec![3]));
    assert!(nif.edit::<NiNode>(2).unwrap().set_children(&[]));
    assert_eq!(reload(&mut nif).view::<NiNode>(2).unwrap().children().count(), 0);
}

#[test]
fn properties() {
    let mut nif = upstream("TestNifFile_Skinned_OB");
    let shape = nif.shapes().indices().next().unwrap();
    let props: Vec<_> = nif.view::<NiShape>(shape).unwrap().properties().indices().collect();
    assert!(props.len() >= 2, "{props:?}");

    let mut edit = nif.edit::<NiShape>(shape).unwrap();
    assert!(!edit.set_properties(&[props[0], shape]));
    let reversed: Vec<_> = props.iter().rev().copied().collect();
    assert!(edit.set_properties(&reversed));
    let saved = reload(&mut nif);
    assert_eq!(saved.view::<NiShape>(shape).unwrap().properties().indices().collect::<Vec<_>>(), reversed);
    assert!(nif.edit::<NiShape>(shape).unwrap().set_properties(&[]));
    assert_eq!(reload(&mut nif).view::<NiShape>(shape).unwrap().properties().count(), 0);
}

#[test]
fn extra_data_list() {
    let mut nif = load("Suzanne");
    let root = root(&nif);
    let mut edit = nif.edit::<NiNode>(root).unwrap();
    let bsx = edit.view().extra_data().indices().next().unwrap();
    let string = edit.assign_extra_data(ExtraString("Prn", "SHIELD")).unwrap();
    assert!(!edit.set_extra_data(&[string, root]));
    assert!(edit.set_extra_data(&[string, bsx]));
    let saved = reload(&mut nif);
    assert_eq!(saved.view::<NiNode>(root).unwrap().extra_data().indices().collect::<Vec<_>>(), [string, bsx]);
}

#[test]
fn extra_data_on_any_named_object() {
    let mut nif = load("Cube");
    let shape = nif.shapes().indices().next().unwrap();
    let shader = nif.view::<NiShape>(shape).unwrap().shader().unwrap().index();
    let data = nif.edit::<NiShader>(shader).unwrap().assign_extra_data(ExtraInteger("Count", 3)).unwrap();
    let saved = reload(&mut nif);
    let view = saved.view::<NiObjectNet>(shader).unwrap();
    assert_eq!(view.extra_data().indices().collect::<Vec<_>>(), [data]);
}

#[test]
fn extra_data_chain() {
    let mut nif = NifFile::create(NifVersion::MORROWIND);
    let root = root(&nif);
    let mut edit = nif.edit::<NiNode>(root).unwrap();
    let a = edit.assign_extra_data(ExtraString("", "a")).unwrap();
    let b = edit.assign_extra_data(ExtraString("", "b")).unwrap();
    assert_eq!(edit.view().extra_data_chain().indices().collect::<Vec<_>>(), [b, a]);

    assert!(!edit.set_extra_data_chain(&[a, a]), "a cycle");
    assert!(!edit.set_extra_data_chain(&[a, root]));
    assert!(edit.set_extra_data_chain(&[a, b]));
    let saved = reload(&mut nif);
    let chain: Vec<_> = saved.view::<NiNode>(root).unwrap().extra_data_chain().indices().collect();
    assert_eq!(chain, [a, b]);
    assert_eq!(saved.view::<NiExtraData>(b).unwrap().next_extra_data().map(|v| v.index()), None);
    assert!(nif.edit::<NiNode>(root).unwrap().set_extra_data_chain(&[]));
    assert_eq!(reload(&mut nif).view::<NiNode>(root).unwrap().extra_data_chain().count(), 0);
}

#[test]
fn controller() {
    let mut nif = upstream("TestNifFile_Morph_MW");
    let (owner, controller) = nif
        .blocks_of::<NiObjectNet>()
        .views()
        .find_map(|v| Some((v.index(), v.controller()?.index())))
        .unwrap();
    assert!(nif.block(controller).unwrap().block_name().ends_with("Controller"));

    let mut edit = nif.edit::<NiObjectNet>(owner).unwrap();
    assert!(!edit.set_controller(Some(owner)));
    assert!(edit.set_controller(None));
    assert!(edit.controller().is_none());
    assert!(edit.set_controller(Some(controller)));
    assert_eq!(reload(&mut nif).view::<NiObjectNet>(owner).unwrap().controller().unwrap().index(), controller);
}

#[test]
fn collision() {
    // Suzanne's root has a bhkCollisionObject at 6.
    let mut nif = load("Suzanne");
    let root = root(&nif);
    let mut edit = nif.edit::<NiNode>(root).unwrap();
    assert_eq!(edit.collision().map(|c| c.index()), Some(6));
    assert!(!edit.set_collision(Some(5)), "a rigid body");
    assert!(edit.set_collision(None));
    assert!(reload(&mut nif).view::<NiNode>(root).unwrap().collision().is_none());
    assert!(nif.edit::<NiNode>(root).unwrap().set_collision(Some(6)));
    let saved = reload(&mut nif);
    let collision: Option<_> = saved.view::<NiNode>(root).unwrap().collision();
    assert_eq!(collision.map(|c| c.get().block_name()), Some("bhkCollisionObject"));
    assert!(saved.view::<NiObject>(6).is_some());
}
