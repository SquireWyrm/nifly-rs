//! Tests for world objects, rigid bodies, phantoms and actions.

use nifly::{
    collision::{
        BhkAabbPhantom, BhkCollisionObject, BhkConstraint, BhkLiquidAction, BhkMoppBvTreeShape,
        BhkOrientHingedBodyAction, BhkRigidBody, BhkSimpleShapePhantom, BhkSpCollisionObject, BhkWorldObject,
        CInfoProperty, CollisionResponse, HavokFilter,
    },
    file::{Index, NifFile, NifVersion},
};

use crate::common::{load, reload_in_place as reload, upstream};

fn first<T: nifly::objects::NiType>(nif: &NifFile) -> Index {
    nif.blocks_of::<T>().indices().next().unwrap()
}

/// A new file of `version` whose root has a collision object with a new body of type `B`.
fn with_body<B: nifly::collision::NewBody>(version: NifVersion) -> (NifFile, Index) {
    let mut nif = NifFile::create(version);
    let collision = nif.root_node_mut().unwrap().assign_collision::<BhkCollisionObject, _>();
    let body = nif.edit::<BhkCollisionObject>(collision).unwrap().assign_body::<B, _>().unwrap();
    (nif, body)
}

#[test]
fn world_object_fields() {
    // Suzanne.nif: a rigid body with a MOPP shape.
    let mut nif = load("Suzanne");
    let body = first::<BhkRigidBody>(&nif);
    let shape = nif.view::<BhkRigidBody>(body).unwrap().shape().unwrap().index();
    assert!(nif.view::<BhkMoppBvTreeShape>(shape).is_some());
    let filter = HavokFilter { layer: 5, flags_and_parts: 0x81, group: 7 };
    let property = CInfoProperty { data: 1, size: 2, capacity_and_flags: 3 };

    let mut edit = nif.edit::<BhkRigidBody>(body).unwrap();
    assert!(!edit.set_shape(Some(body)), "not a shape");
    let mut world = edit.block().upcast_mut().upcast_mut();
    world.as_mut().set_collision_filter(filter);
    world.as_mut().set_broad_phase_type(2);
    world.as_mut().set_unknown_int(-4);
    world.as_mut().set_unknown_bytes([1, 2, 3]);
    world.set_property(property);

    let saved = reload(&mut nif);
    let world = saved.block_of::<BhkWorldObject>(body).unwrap();
    assert_eq!((world.collision_filter(), world.property()), (filter, property));
    assert_eq!((world.broad_phase_type(), world.unknown_int(), world.unknown_bytes()), (2, -4, [1, 2, 3]));
    assert_eq!(saved.view::<BhkWorldObject>(body).unwrap().shape().unwrap().index(), shape);

    let mut nif = saved;
    assert!(nif.edit::<BhkRigidBody>(body).unwrap().set_shape(None));
    assert!(reload(&mut nif).view::<BhkRigidBody>(body).unwrap().shape().is_none());
}

#[test]
fn rigid_body_fields() {
    let (mut nif, body) = with_body::<BhkRigidBody>(NifVersion::SKYRIM_SE);
    let filter = HavokFilter { layer: 1, flags_and_parts: 2, group: 3 };
    let inertia = [[1.0, 0.0, 0.0, 0.0], [0.0, 2.0, 0.0, 0.0], [0.0, 0.0, 3.0, 0.0]];
    let mut b = nif.block_of_mut::<BhkRigidBody>(body).unwrap();
    b.as_mut().set_collision_response(CollisionResponse::Reporting);
    b.as_mut().set_process_contact_callback_delay(5);
    b.as_mut().set_unknown_int(6);
    b.as_mut().set_collision_filter_copy(filter);
    b.as_mut().set_unknown_shorts([1, 2, 3, 4, 5, 6]);
    b.as_mut().set_translation([1.0, 2.0, 3.0, 0.0]);
    b.as_mut().set_rotation([0.0, 0.0, 0.6, 0.8]);
    b.as_mut().set_linear_velocity([0.5, 0.0, 0.0, 0.0]);
    b.as_mut().set_angular_velocity([0.0, 0.5, 0.0, 0.0]);
    b.as_mut().set_inertia(inertia);
    b.as_mut().set_center([0.1, 0.2, 0.3, 0.0]);
    b.as_mut().set_mass(12.0);
    b.as_mut().set_linear_damping(0.2);
    b.as_mut().set_angular_damping(0.3);
    b.as_mut().set_time_factor(0.9);
    b.as_mut().set_gravity_factor(0.8);
    b.as_mut().set_friction(0.7);
    b.as_mut().set_rolling_friction_multiplier(0.6);
    b.as_mut().set_restitution(0.5);
    b.as_mut().set_max_linear_velocity(100.0);
    b.as_mut().set_max_angular_velocity(30.0);
    b.as_mut().set_penetration_depth(0.25);
    b.as_mut().set_motion_system(4);
    b.as_mut().set_deactivator_type(2);
    b.as_mut().set_solver_deactivation(3);
    b.as_mut().set_quality_type(5);
    b.as_mut().set_auto_remove_level(1);
    b.as_mut().set_response_modifier_flags(2);
    b.as_mut().set_shape_keys_in_contact_points(3);
    b.as_mut().set_force_collide_onto_ppu(true);
    b.as_mut().set_body_flags(9);
    b.set_body_flags_32(10);

    let saved = reload(&mut nif);
    let b = saved.block_of::<BhkRigidBody>(body).unwrap();
    assert_eq!(b.collision_response(), Some(CollisionResponse::Reporting));
    assert_eq!((b.process_contact_callback_delay(), b.unknown_int()), (5, 6));
    assert_eq!((b.collision_filter_copy(), b.unknown_shorts()), (filter, [1, 2, 3, 4, 5, 6]));
    assert_eq!((b.translation(), b.rotation()), ([1.0, 2.0, 3.0, 0.0], [0.0, 0.0, 0.6, 0.8]));
    assert_eq!((b.linear_velocity(), b.angular_velocity()), ([0.5, 0.0, 0.0, 0.0], [0.0, 0.5, 0.0, 0.0]));
    assert_eq!((b.inertia(), b.center()), (inertia, [0.1, 0.2, 0.3, 0.0]));
    assert_eq!((b.mass(), b.linear_damping(), b.angular_damping()), (12.0, 0.2, 0.3));
    assert_eq!((b.time_factor(), b.gravity_factor(), b.friction()), (0.9, 0.8, 0.7));
    assert_eq!((b.rolling_friction_multiplier(), b.restitution()), (0.6, 0.5));
    assert_eq!((b.max_linear_velocity(), b.max_angular_velocity(), b.penetration_depth()), (100.0, 30.0, 0.25));
    assert_eq!((b.motion_system(), b.deactivator_type(), b.solver_deactivation(), b.quality_type()), (4, 2, 3, 5));
    assert_eq!((b.auto_remove_level(), b.response_modifier_flags(), b.shape_keys_in_contact_points()), (1, 2, 3));
    assert!(b.force_collide_onto_ppu());
    assert_eq!((b.body_flags(), b.body_flags_32()), (9, 0), "only the 16-bit flags are saved in Skyrim");
}

#[test]
fn oblivion_rigid_body() {
    let (mut nif, body) = with_body::<BhkRigidBody>(NifVersion::OBLIVION);
    let mut b = nif.block_of_mut::<BhkRigidBody>(body).unwrap();
    b.as_mut().set_body_flags_32(0x10001);
    b.as_mut().set_gravity_factor(0.5);
    b.set_mass(3.0);
    let saved = reload(&mut nif);
    let b = saved.block_of::<BhkRigidBody>(body).unwrap();
    assert_eq!((b.body_flags_32(), b.mass()), (0x10001, 3.0));
    assert_eq!(b.gravity_factor(), 1.0, "not saved before Skyrim");
}

#[test]
fn constraints() {
    let mut nif = upstream("TestNifFile_Furniture_Col_SE");
    let (body, list) = nif
        .blocks_of::<BhkRigidBody>()
        .views()
        .map(|b| (b.index(), b.constraints().indices().collect::<Vec<_>>()))
        .find(|(_, c)| !c.is_empty())
        .expect("a rigid body with constraints");
    assert!(list.iter().all(|&c| nif.view::<BhkConstraint>(c).is_some()));

    let mut edit = nif.edit::<BhkRigidBody>(body).unwrap();
    assert!(!edit.set_constraints(&[body]), "not a constraint");
    assert!(edit.set_constraints(&[list[0], list[0]]));
    let saved = reload(&mut nif);
    let constraints: Vec<_> = saved.view::<BhkRigidBody>(body).unwrap().constraints().indices().collect();
    assert_eq!(constraints, [list[0], list[0]]);
}

#[test]
fn phantoms() {
    let transform = [[1.0, 0.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, -1.0, 0.0, 0.0], [1.0, 2.0, 3.0, 1.0]];
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let collision = nif.root_node_mut().unwrap().assign_collision::<BhkSpCollisionObject, _>();
    let mut edit = nif.edit::<BhkSpCollisionObject>(collision).unwrap();
    let simple = edit.assign_body::<BhkSimpleShapePhantom, _>().unwrap();
    nif.block_of_mut::<BhkSimpleShapePhantom>(simple).unwrap().set_transform(transform);
    let (mut nif2, aabb) = with_body::<BhkAabbPhantom>(NifVersion::SKYRIM_SE);
    let mut phantom = nif2.block_of_mut::<BhkAabbPhantom>(aabb).unwrap();
    phantom.as_mut().set_aabb_min([-1.0, -2.0, -3.0, 0.0]);
    phantom.set_aabb_max([1.0, 2.0, 3.0, 0.0]);

    assert_eq!(reload(&mut nif).block_of::<BhkSimpleShapePhantom>(simple).unwrap().transform(), transform);
    let saved = reload(&mut nif2);
    let phantom = saved.block_of::<BhkAabbPhantom>(aabb).unwrap();
    assert_eq!((phantom.aabb_min(), phantom.aabb_max()), ([-1.0, -2.0, -3.0, 0.0], [1.0, 2.0, 3.0, 0.0]));
}

#[test]
fn actions() {
    let (mut nif, body) = with_body::<BhkRigidBody>(NifVersion::SKYRIM_SE);
    let liquid = nif.add_action::<BhkLiquidAction>();
    let hinged = nif.add_action::<BhkOrientHingedBodyAction>();
    let mut l = nif.block_of_mut::<BhkLiquidAction>(liquid).unwrap();
    l.as_mut().set_user_data(1);
    l.as_mut().set_unknown_int_1(2);
    l.as_mut().set_unknown_int_2(3);
    l.as_mut().set_initial_stick_force(0.5);
    l.as_mut().set_stick_strength(0.25);
    l.as_mut().set_neighbor_distance(4.0);
    l.set_neighbor_strength(0.125);
    let mut h = nif.block_of_mut::<BhkOrientHingedBodyAction>(hinged).unwrap();
    h.as_mut().set_unknown_int_1(5);
    h.as_mut().set_unknown_int_2(6);
    h.as_mut().set_hinge_axis([0.0, 0.0, 1.0, 0.0]);
    h.as_mut().set_forward([1.0, 0.0, 0.0, 0.0]);
    h.as_mut().set_strength(2.0);
    h.set_damping(0.5);
    let mut edit = nif.edit::<BhkOrientHingedBodyAction>(hinged).unwrap();
    assert!(!edit.set_hinged_body(Some(liquid)), "not a rigid body");
    assert!(edit.set_hinged_body(Some(body)));

    let saved = reload(&mut nif);
    let l = saved.block_of::<BhkLiquidAction>(liquid).unwrap();
    assert_eq!((l.user_data(), l.unknown_int_1(), l.unknown_int_2()), (1, 2, 3));
    assert_eq!((l.initial_stick_force(), l.stick_strength()), (0.5, 0.25));
    assert_eq!((l.neighbor_distance(), l.neighbor_strength()), (4.0, 0.125));
    let h = saved.view::<BhkOrientHingedBodyAction>(hinged).unwrap();
    assert_eq!((h.unknown_int_1(), h.unknown_int_2()), (5, 6));
    assert_eq!((h.hinge_axis(), h.forward()), ([0.0, 0.0, 1.0, 0.0], [1.0, 0.0, 0.0, 0.0]));
    assert_eq!((h.strength(), h.damping()), (2.0, 0.5));
    assert_eq!(h.hinged_body().unwrap().index(), body);
}
