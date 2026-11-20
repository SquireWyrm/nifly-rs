//! Tests for collision objects: targets, bodies, NiCollisionData, flags, systems and creating
//! collision objects and bodies.

use nifly::{
    collision::{
        BhkBlendCollisionObject, BhkCollisionObject, BhkNiCollisionObject, BhkNpCollisionObject,
        BhkPhysicsSystem, BhkRagdollSystem, BhkRigidBody, BhkSimpleShapePhantom, CollisionMode,
        CollisionObjectFlags, NiCollisionData, NiCollisionObject, PropagationMode,
    },
    file::{Index, NifFile, NifVersion},
    nodes::NiNode,
    object3d::{BoundingSphere, BoundingVolume, Vec3},
    objects::NiAvObject,
};

use crate::common::{load, reload_in_place as reload};

fn first<T: nifly::objects::NiType>(nif: &NifFile) -> Index {
    nif.blocks_of::<T>().indices().next().unwrap()
}

/// A new file of `version` whose root has a new collision object of type `C`.
fn with_collision<C: nifly::collision::NewCollisionObject>(
    version: NifVersion,
) -> (NifFile, Index) {
    let mut nif = NifFile::create(version);
    let collision = nif.root_node_mut().unwrap().assign_collision::<C, _>();
    (nif, collision)
}

#[test]
fn reads_collision_object() {
    // Suzanne.nif: a collision object whose body is a rigid body.
    let nif = load("Suzanne");
    let index = first::<BhkCollisionObject>(&nif);
    let collision = nif.view::<BhkCollisionObject>(index).unwrap();
    let target = collision.target().unwrap();
    assert_eq!(target.collision().unwrap().index(), index);
    assert!(collision.body().unwrap().cast::<BhkRigidBody>().is_some());
    assert!(collision.flags().contains(CollisionObjectFlags::ACTIVE));
}

#[test]
fn target_and_body() {
    let mut nif = load("Suzanne");
    let index = first::<BhkCollisionObject>(&nif);
    let body = nif
        .view::<BhkCollisionObject>(index)
        .unwrap()
        .body()
        .unwrap()
        .index();
    let shape = nif.shapes().indices().next().unwrap();

    let mut edit = nif.edit::<BhkCollisionObject>(index).unwrap();
    assert!(!edit.set_target(Some(body)), "not an NiAvObject");
    assert!(!edit.set_body(Some(shape)), "not a world object");
    assert!(edit.set_target(Some(shape)));
    let flags = CollisionObjectFlags::ACTIVE
        | CollisionObjectFlags::SYNC_ON_UPDATE
        | CollisionObjectFlags::from_bits_retain(1 << 15);
    edit.set_flags(flags);
    let saved = reload(&mut nif);
    let collision = saved.view::<BhkCollisionObject>(index).unwrap();
    assert_eq!(collision.target().unwrap().index(), shape);
    assert_eq!(
        (collision.body().unwrap().index(), collision.flags()),
        (body, flags),
        "unnamed bits are kept"
    );

    let mut nif = saved;
    assert!(
        nif.edit::<BhkNiCollisionObject>(index)
            .unwrap()
            .set_body(None)
    );
    assert!(
        reload(&mut nif)
            .view::<BhkCollisionObject>(index)
            .unwrap()
            .body()
            .is_none()
    );
}

#[test]
fn assign_collision_and_body() {
    let (mut nif, collision) = with_collision::<BhkCollisionObject>(NifVersion::SKYRIM_SE);
    let root = nif.root_node().unwrap().index();
    assert_eq!(
        nif.view::<NiCollisionObject>(collision)
            .unwrap()
            .target()
            .unwrap()
            .index(),
        root
    );

    let mut edit = nif.edit::<BhkCollisionObject>(collision).unwrap();
    assert_eq!(
        edit.assign_body::<BhkPhysicsSystem, _>(),
        None,
        "systems are for NP collision objects"
    );
    let phantom = edit.assign_body::<BhkSimpleShapePhantom, _>().unwrap();
    let body = edit.assign_body::<BhkRigidBody, _>().unwrap();
    assert_ne!(phantom, body);

    let saved = reload(&mut nif);
    let root = saved.view::<NiNode>(root).unwrap();
    let collision = root.collision().unwrap();
    assert_eq!(
        collision
            .cast::<BhkCollisionObject>()
            .unwrap()
            .body()
            .unwrap()
            .index(),
        body
    );
}

#[test]
fn systems() {
    let (mut nif, collision) = with_collision::<BhkNpCollisionObject>(NifVersion::FALLOUT_4);
    let mut edit = nif.edit::<BhkNpCollisionObject>(collision).unwrap();
    assert_eq!(
        edit.assign_body::<BhkRigidBody, _>(),
        None,
        "NP collision objects take systems"
    );
    let ragdoll = edit.assign_body::<BhkRagdollSystem, _>().unwrap();
    nif.block_of_mut::<BhkRagdollSystem>(ragdoll)
        .unwrap()
        .set_data(&[9, 8]);
    let mut edit = nif.edit::<BhkNpCollisionObject>(collision).unwrap();
    let physics = edit.assign_body::<BhkPhysicsSystem, _>().unwrap();
    edit.set_body_id(3);
    nif.block_of_mut::<BhkPhysicsSystem>(physics)
        .unwrap()
        .set_data(&[1, 2, 3, 4]);

    let saved = reload(&mut nif);
    let collision = saved.view::<BhkNpCollisionObject>(collision).unwrap();
    assert_eq!(collision.body_id(), 3);
    assert_eq!(
        collision
            .body()
            .unwrap()
            .cast::<BhkPhysicsSystem>()
            .unwrap()
            .data(),
        [1, 2, 3, 4]
    );
    assert_eq!(
        saved.block_of::<BhkRagdollSystem>(ragdoll).unwrap().data(),
        [9, 8]
    );

    let collision = collision.index();
    let mut nif = saved;
    let mut edit = nif.edit::<BhkNpCollisionObject>(collision).unwrap();
    assert!(edit.set_body(Some(ragdoll)));
    assert!(!edit.set_body(Some(0)), "not a system");
}

#[test]
fn blend_gains() {
    let (mut nif, collision) = with_collision::<BhkBlendCollisionObject>(NifVersion::SKYRIM_SE);
    let blend = nif
        .block_of_mut::<BhkBlendCollisionObject>(collision)
        .unwrap();
    blend.set_heir_gain(0.5);
    blend.set_velocity_gain(0.25);
    let saved = reload(&mut nif);
    let blend = saved
        .block_of::<BhkBlendCollisionObject>(collision)
        .unwrap();
    assert_eq!((blend.heir_gain(), blend.velocity_gain()), (0.5, 0.25));
}

#[test]
fn collision_data() {
    let (mut nif, collision) = with_collision::<NiCollisionData>(NifVersion::OBLIVION);
    let volume = BoundingVolume::Sphere(BoundingSphere::new(Vec3::new(1.0, 2.0, 3.0), 4.0));
    let data = nif.block_of::<NiCollisionData>(collision).unwrap();
    assert_eq!(data.bounding_volume(), None);
    let data = nif.block_of_mut::<NiCollisionData>(collision).unwrap();
    data.set_propagation_mode(PropagationMode::Never);
    data.set_collision_mode(CollisionMode::UseAbv);
    data.set_bounding_volume(Some(&volume));

    let saved = reload(&mut nif);
    let data = saved.block_of::<NiCollisionData>(collision).unwrap();
    assert_eq!(data.propagation_mode(), Some(PropagationMode::Never));
    assert_eq!(data.collision_mode(), Some(CollisionMode::UseAbv));
    assert_eq!(data.bounding_volume(), Some(volume));
    let target = saved
        .view::<NiCollisionData>(collision)
        .unwrap()
        .target()
        .unwrap();
    assert!(target.cast::<NiAvObject>().is_some());
}
