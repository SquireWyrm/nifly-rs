//! Tests for particle systems, modifiers, emitters, colliders and particle controllers.

use nifly::{
    animation::{
        Interpolation, Key, KeyGroup, NiBoolInterpolator, NiColorData, NiFloatInterpolator, NiPSysEmitterSpeedCtlr,
        NiPSysModifierActiveCtlr, NiPSysModifierCtlr, NiPSysUpdateCtlr,
    },
    file::{NifFile, NifVersion},
    geometry::NiGeometryData,
    nodes::{BsMasterParticleSystem, NiNode},
    object3d::{BoundingSphere, Transform, Vec3},
    particles::*,
    shaders::BsSkyShaderProperty,
};

use crate::common::reload_in_place;

/// Oblivion's 10.1.0.106 files.
const OLD: NifVersion = NifVersion::new(0x0A01006A, 10, 5);

/// A new file with a particle system under the root, and the system's index.
fn with_system(version: NifVersion) -> (NifFile, usize) {
    let mut nif = NifFile::create(version);
    let system = nif.root_node_mut().unwrap().add_particle_system::<NiParticleSystem, _>("Sparks", Transform::default());
    (nif, system)
}

#[test]
fn add_particle_system() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let mut root = nif.root_node_mut().unwrap();
    let plain = root.add_particle_system::<NiParticleSystem, _>("Plain", Transform::default());
    let mesh = root.add_particle_system::<NiMeshParticleSystem, _>("Mesh", Transform::default());
    let strip = root.add_particle_system::<BsStripParticleSystem, _>("Strip", Transform::default());
    let nif = reload_in_place(&mut nif);

    let root = nif.root_node().unwrap();
    assert_eq!(root.children().indices().collect::<Vec<_>>(), [plain, mesh, strip]);
    for (system, data) in [(plain, "NiPSysData"), (mesh, "NiMeshPSysData"), (strip, "BSStripPSysData")] {
        let v = nif.view::<NiParticleSystem>(system).unwrap();
        assert_eq!(v.data().unwrap().block_name(), data);
        assert!(v.world_space());
        assert_eq!(v.modifiers().count(), 0);
    }
    assert_eq!(nif.view::<NiParticleSystem>(plain).unwrap().name(), "Plain");
}

#[test]
fn system_refs() {
    let (mut nif, system) = with_system(NifVersion::SKYRIM_SE);
    let shader = nif.edit::<NiParticleSystem>(system).unwrap().add_property::<BsSkyShaderProperty, _>();
    let other = nif.root_node_mut().unwrap().add_particle_system::<NiParticleSystem, _>("Other", Transform::default());
    let other_data = nif.view::<NiParticleSystem>(other).unwrap().data().unwrap().index();

    let mut e = nif.edit::<NiParticleSystem>(system).unwrap();
    assert!(!e.set_data(Some(0)), "a node isn't particle data");
    assert!(!e.set_shader(Some(0)) && !e.set_alpha_property(Some(shader)) && !e.set_skin_instance(Some(0)));
    assert!(e.set_data(Some(other_data)) && e.set_shader(Some(shader)));
    let nif = reload_in_place(&mut nif);
    let v = nif.view::<NiParticleSystem>(system).unwrap();
    assert_eq!(v.data().unwrap().index(), other_data);
    assert_eq!(v.shader().unwrap().index(), shader);
    assert!(v.alpha_property().is_none() && v.skin_instance().is_none());

    // Older files save the data in the other ref, which follows.
    let (mut nif, system) = with_system(NifVersion::OBLIVION);
    let other = nif.root_node_mut().unwrap().add_particle_system::<NiParticleSystem, _>("Other", Transform::default());
    let other_data = nif.view::<NiParticleSystem>(other).unwrap().data().unwrap().index();
    assert!(nif.edit::<NiParticleSystem>(system).unwrap().set_data(Some(other_data)));
    let nif = reload_in_place(&mut nif);
    assert_eq!(nif.view::<NiParticleSystem>(system).unwrap().data().unwrap().index(), other_data);
}

#[test]
fn modifiers() {
    let (mut nif, system) = with_system(NifVersion::SKYRIM_SE);
    let mut e = nif.edit::<NiParticleSystem>(system).unwrap();
    let emitter = e.add_modifier::<NiPSysBoxEmitter>("Emitter");
    let age = e.add_modifier::<NiPSysAgeDeathModifier>("Age");
    let spawn = e.add_modifier::<NiPSysSpawnModifier>("Spawn");
    let position = e.add_modifier::<NiPSysPositionModifier>("Position");
    assert_eq!(e.view().modifiers().indices().collect::<Vec<_>>(), [emitter, age, spawn, position]);

    let m = nif.view::<NiPSysModifier>(age).unwrap();
    assert_eq!((m.name(), m.order(), m.active()), ("Age".into(), 0, true));
    assert_eq!(nif.view::<NiPSysAgeDeathModifier>(age).unwrap().particle_system().unwrap().index(), system);

    let mut e = nif.edit::<NiParticleSystem>(system).unwrap();
    assert!(!e.set_modifiers(&[emitter, 0]), "a node isn't a modifier");
    assert!(e.set_modifiers(&[position, emitter]));
    let mut m = nif.edit::<NiPSysAgeDeathModifier>(age).unwrap();
    assert!(!m.set_particle_system(Some(0)) && !m.set_spawn_modifier(Some(position)));
    assert!(m.set_spawn_modifier(Some(spawn)));
    let mut b = nif.block_of_mut::<NiPSysModifier>(age).unwrap();
    b.as_mut().set_name("Renamed");
    b.as_mut().set_order(3000);
    b.as_mut().set_active(false);

    let nif = reload_in_place(&mut nif);
    let v = nif.view::<NiParticleSystem>(system).unwrap();
    assert_eq!(v.modifiers().indices().collect::<Vec<_>>(), [position, emitter]);
    let m = nif.view::<NiPSysAgeDeathModifier>(age).unwrap();
    assert_eq!((m.name(), m.order(), m.active()), ("Renamed".into(), 3000, false));
    assert_eq!(m.spawn_modifier().unwrap().index(), spawn);
}

#[test]
fn modifier_objects() {
    let (mut nif, system) = with_system(NifVersion::SKYRIM_SE);
    let node = nif.root_node_mut().unwrap().add_child("Node", Transform::default());
    let mut e = nif.edit::<NiParticleSystem>(system).unwrap();
    let gravity = e.add_modifier::<NiPSysGravityModifier>("Gravity");
    let bomb = e.add_modifier::<NiPSysBombModifier>("Bomb");
    let field = e.add_modifier::<NiPSysAirFieldModifier>("Air");
    let sphere = e.add_modifier::<NiPSysSphereEmitter>("Sphere");

    // Gravity takes any scene object; bombs and volume emitters only nodes.
    assert!(nif.edit::<NiPSysGravityModifier>(gravity).unwrap().set_modifier_object(Some(system)));
    assert!(!nif.edit::<NiPSysBombModifier>(bomb).unwrap().set_modifier_object(Some(system)));
    assert!(nif.edit::<NiPSysBombModifier>(bomb).unwrap().set_modifier_object(Some(node)));
    assert!(!nif.edit::<NiPSysSphereEmitter>(sphere).unwrap().set_modifier_object(Some(gravity)));
    assert!(nif.edit::<NiPSysSphereEmitter>(sphere).unwrap().set_modifier_object(Some(node)));
    assert!(nif.edit::<NiPSysAirFieldModifier>(field).unwrap().set_modifier_object(Some(node)));

    let nif = reload_in_place(&mut nif);
    assert_eq!(nif.view::<NiPSysGravityModifier>(gravity).unwrap().modifier_object().unwrap().index(), system);
    assert_eq!(nif.view::<NiPSysBombModifier>(bomb).unwrap().modifier_object().unwrap().index(), node);
    assert_eq!(nif.view::<NiPSysSphereEmitter>(sphere).unwrap().modifier_object().unwrap().index(), node);
    assert_eq!(nif.view::<NiPSysAirFieldModifier>(field).unwrap().modifier_object().unwrap().index(), node);
}

#[test]
fn modifier_lists_and_data() {
    let (mut nif, system) = with_system(NifVersion::SKYRIM_SE);
    let node = nif.root_node_mut().unwrap().add_child("Node", Transform::default());
    let colors = nif.add_animation_data::<NiColorData>();
    let mut e = nif.edit::<NiParticleSystem>(system).unwrap();
    let color = e.add_modifier::<NiPSysColorModifier>("Color");
    let mesh_update = e.add_modifier::<NiPSysMeshUpdateModifier>("Meshes");
    let mesh_emitter = e.add_modifier::<NiPSysMeshEmitter>("MeshEmitter");
    let havok = e.add_modifier::<BsPSysHavokUpdateModifier>("Havok");

    let mut c = nif.edit::<NiPSysColorModifier>(color).unwrap();
    assert!(!c.set_color_data(Some(node)) && c.set_color_data(Some(colors)));
    assert!(nif.edit::<NiPSysMeshUpdateModifier>(mesh_update).unwrap().set_particle_meshes(&[node, system]));
    let mut m = nif.edit::<NiPSysMeshEmitter>(mesh_emitter).unwrap();
    assert!(!m.set_particle_meshes(&[colors]) && m.set_particle_meshes(&[node]));
    let mut h = nif.edit::<BsPSysHavokUpdateModifier>(havok).unwrap();
    assert!(!h.set_havok_nodes(&[system]) && h.set_havok_nodes(&[node]));
    assert!(!h.set_havok_modifier(Some(node)) && h.set_havok_modifier(Some(color)));

    let nif = reload_in_place(&mut nif);
    assert_eq!(nif.view::<NiPSysColorModifier>(color).unwrap().color_data().unwrap().index(), colors);
    let meshes: Vec<_> = nif.view::<NiPSysMeshUpdateModifier>(mesh_update).unwrap().particle_meshes().indices().collect();
    assert_eq!(meshes, [node, system]);
    assert_eq!(nif.view::<NiPSysMeshEmitter>(mesh_emitter).unwrap().particle_meshes().indices().collect::<Vec<_>>(), [node]);
    let h = nif.view::<BsPSysHavokUpdateModifier>(havok).unwrap();
    assert_eq!(h.havok_nodes().indices().collect::<Vec<_>>(), [node]);
    assert_eq!(h.havok_modifier().unwrap().index(), color);
}

#[test]
fn colliders() {
    let (mut nif, system) = with_system(NifVersion::SKYRIM_SE);
    let node = nif.root_node_mut().unwrap().add_child("Node", Transform::default());
    let mut e = nif.edit::<NiParticleSystem>(system).unwrap();
    let manager = e.add_modifier::<NiPSysColliderManager>("Colliders");
    let spawn = e.add_modifier::<NiPSysSpawnModifier>("Spawn");
    let mut m = nif.edit::<NiPSysColliderManager>(manager).unwrap();
    let sphere = m.add_collider::<NiPSysSphericalCollider, _>();
    let plane = m.add_collider::<NiPSysPlanarCollider, _>();
    assert_eq!(m.view().colliders().indices().collect::<Vec<_>>(), [sphere, plane]);

    let mut c = nif.edit::<NiPSysSphericalCollider>(sphere).unwrap();
    assert_eq!(c.view().collider_manager().unwrap().index(), manager);
    assert_eq!(c.view().next_collider().unwrap().index(), plane);
    assert!(!c.set_next_collider(Some(sphere)), "a collider can't follow itself");
    assert!(!nif.edit::<NiPSysPlanarCollider>(plane).unwrap().set_next_collider(Some(sphere)), "a loop");
    let mut c = nif.edit::<NiPSysSphericalCollider>(sphere).unwrap();
    assert!(c.set_spawn_modifier(Some(spawn)) && c.set_modifier_object(Some(node)));
    assert!(!c.set_collider_manager(Some(spawn)));

    let mut m = nif.edit::<NiPSysColliderManager>(manager).unwrap();
    assert!(!m.set_colliders(&[plane, plane]) && !m.set_colliders(&[node]));
    assert!(m.set_colliders(&[plane, sphere]));

    let mut b = nif.block_of_mut::<NiPSysCollider>(plane).unwrap();
    b.as_mut().set_bounce(0.5);
    b.as_mut().set_spawn_on_collide(true);
    b.as_mut().set_die_on_collide(true);
    let mut b = nif.block_of_mut::<NiPSysPlanarCollider>(plane).unwrap();
    b.as_mut().set_width(2.0);
    b.as_mut().set_x_axis(Vec3::new(1.0, 0.0, 0.0));
    nif.block_of_mut::<NiPSysSphericalCollider>(sphere).unwrap().set_radius(3.0);

    let nif = reload_in_place(&mut nif);
    let m = nif.view::<NiPSysColliderManager>(manager).unwrap();
    assert_eq!(m.colliders().indices().collect::<Vec<_>>(), [plane, sphere]);
    let c = nif.view::<NiPSysSphericalCollider>(sphere).unwrap();
    assert_eq!(c.radius(), 3.0);
    assert_eq!((c.spawn_modifier().unwrap().index(), c.modifier_object().unwrap().index()), (spawn, node));
    let p = nif.view::<NiPSysPlanarCollider>(plane).unwrap();
    assert_eq!((p.bounce(), p.spawn_on_collide(), p.die_on_collide()), (0.5, true, true));
    assert_eq!((p.width(), p.x_axis()), (2.0, Vec3::new(1.0, 0.0, 0.0)));
}

#[test]
fn particle_data() {
    let (mut nif, system) = with_system(NifVersion::OBLIVION);
    let data = nif.view::<NiParticleSystem>(system).unwrap().data().unwrap().index();
    let mut d = nif.block_of_mut::<NiParticlesData>(data).unwrap();
    d.as_mut().set_vertex_count(3);
    assert!(!d.as_mut().set_radii(&[1.0, 2.0]), "one per vertex");
    assert!(d.as_mut().set_radii(&[1.0, 2.0, 3.0]));
    d.as_mut().set_has_radii(true);
    assert!(d.as_mut().set_sizes(&[0.5; 3]));
    d.as_mut().set_has_sizes(true);
    let q = [0.0, 0.0, 0.0, 1.0];
    assert!(!d.as_mut().set_rotations(&[q]) && d.as_mut().set_rotations(&[q; 3]));
    d.as_mut().set_has_rotations(true);
    assert!(d.as_mut().set_rotation_angles(&[0.1, 0.2, 0.3]));
    d.as_mut().set_has_rotation_angles(true);
    assert!(d.as_mut().set_rotation_axes(&[Vec3::new(0.0, 0.0, 1.0); 3]));
    d.as_mut().set_has_rotation_axes(true);
    d.as_mut().set_active_count(2);
    let info = ParticleInfo { velocity: Vec3::new(1.0, 2.0, 3.0), age: 0.5, life_span: 2.0, code: 7, ..Default::default() };
    let mut p = nif.block_of_mut::<NiPSysData>(data).unwrap();
    assert!(!p.as_mut().set_particle_info(&[info]) && p.as_mut().set_particle_info(&[info; 3]));
    assert!(p.as_mut().set_rotation_speeds(&[4.0; 3]));
    p.as_mut().set_has_rotation_speeds(true);
    p.as_mut().set_added_particle_count(1);

    let mut nif = reload_in_place(&mut nif);
    let d = nif.block_of::<NiPSysData>(data).unwrap();
    assert_eq!((d.vertex_count(), d.active_count(), d.added_particle_count()), (3, 2, 1));
    assert_eq!((d.radii(), d.sizes(), d.rotation_angles()), (&[1.0, 2.0, 3.0][..], &[0.5; 3][..], &[0.1, 0.2, 0.3][..]));
    assert_eq!((d.rotations(), d.rotation_axes()), (vec![q; 3], vec![Vec3::new(0.0, 0.0, 1.0); 3]));
    assert_eq!((d.particle_info(), d.rotation_speeds()), (vec![info; 3], &[4.0; 3][..]));

    // Growing pads the lists that have values with zeros.
    nif.block_of_mut::<NiParticlesData>(data).unwrap().set_vertex_count(4);
    let d = nif.block_of::<NiPSysData>(data).unwrap();
    assert_eq!((d.radii(), d.particle_info().len()), (&[1.0, 2.0, 3.0, 0.0][..], 4));
}

#[test]
fn bethesda_particle_data() {
    let (mut nif, system) = with_system(NifVersion::SKYRIM_SE);
    let data = nif.view::<NiParticleSystem>(system).unwrap().data().unwrap().index();
    let mut d = nif.block_of_mut::<NiParticlesData>(data).unwrap();
    d.as_mut().set_vertex_count(2);
    assert!(d.as_mut().set_radii(&[1.0, 2.0]));
    d.as_mut().set_has_radii(true);
    d.as_mut().set_has_texture_indices(true);
    d.as_mut().set_subtexture_offsets(&[[0.0, 0.0, 0.5, 0.5], [0.5, 0.0, 1.0, 0.5]]);
    d.as_mut().set_aspect_ratio(2.0);
    d.as_mut().set_aspect_flags(AspectFlags::SPEED_TO_ASPECT);

    let nif = reload_in_place(&mut nif);
    let d = nif.block_of::<NiParticlesData>(data).unwrap();
    assert!(d.has_radii() && d.radii().is_empty(), "20.2.0.7 saves only the flag");
    assert!(d.has_texture_indices());
    assert_eq!(d.subtexture_offsets(), [[0.0, 0.0, 0.5, 0.5], [0.5, 0.0, 1.0, 0.5]]);
    assert_eq!((d.aspect_ratio(), d.aspect_flags()), (2.0, AspectFlags::SPEED_TO_ASPECT));
}

#[test]
fn system_fields() {
    let (mut nif, system) = with_system(NifVersion::SKYRIM_SE);
    let mut s = nif.block_of_mut::<NiParticleSystem>(system).unwrap();
    s.as_mut().set_world_space(false);
    s.as_mut().set_vertex_raw_flags([1, 2, 3, 4, 5, 6, 7, 8]);
    s.as_mut().set_bounds(BoundingSphere::new(Vec3::new(1.0, 2.0, 3.0), 4.0));
    s.as_mut().set_far_begin(10);
    s.as_mut().set_near_end(20);
    let nif = reload_in_place(&mut nif);
    let s = nif.block_of::<NiParticleSystem>(system).unwrap();
    assert!(!s.world_space());
    assert_eq!(s.vertex_raw_flags(), [1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(s.bounds(), BoundingSphere::new(Vec3::new(1.0, 2.0, 3.0), 4.0));
    assert_eq!((s.far_begin(), s.near_end()), (10, 20));

    let (mut nif, system) = with_system(NifVersion::OBLIVION);
    nif.block_of_mut::<NiParticleSystem>(system).unwrap().set_shader_name(Some("Shader"));
    let nif = reload_in_place(&mut nif);
    assert_eq!(nif.block_of::<NiParticleSystem>(system).unwrap().shader_name(), Some("Shader".into()));
}

#[test]
fn modifier_fields() {
    let (mut nif, system) = with_system(NifVersion::SKYRIM_SE);
    let mut e = nif.edit::<NiParticleSystem>(system).unwrap();
    let gravity = e.add_modifier::<NiPSysGravityModifier>("Gravity");
    let bomb = e.add_modifier::<NiPSysBombModifier>("Bomb");
    let color = e.add_modifier::<BsPSysSimpleColorModifier>("Color");
    let scale = e.add_modifier::<BsPSysScaleModifier>("Scale");
    let rotation = e.add_modifier::<NiPSysRotationModifier>("Rotation");
    let radial = e.add_modifier::<NiPSysRadialFieldModifier>("Radial");
    let mesh = e.add_modifier::<NiPSysMeshEmitter>("Mesh");

    let mut g = nif.block_of_mut::<NiPSysGravityModifier>(gravity).unwrap();
    g.as_mut().set_force_type(ForceType::Spherical);
    g.as_mut().set_strength(9.8);
    g.as_mut().set_world_aligned(true);
    let mut b = nif.block_of_mut::<NiPSysBombModifier>(bomb).unwrap();
    b.as_mut().set_decay_type(DecayType::Exponential);
    b.as_mut().set_symmetry_type(SymmetryType::Planar);
    let mut c = nif.block_of_mut::<BsPSysSimpleColorModifier>(color).unwrap();
    c.as_mut().set_color_2([0.1, 0.2, 0.3, 0.4]);
    c.as_mut().set_fade_out_percent(0.75);
    nif.block_of_mut::<BsPSysScaleModifier>(scale).unwrap().set_scales(&[1.0, 0.5, 0.0]);
    let mut r = nif.block_of_mut::<NiPSysRotationModifier>(rotation).unwrap();
    r.as_mut().set_initial_speed_variation(0.25);
    r.as_mut().set_random_initial_axis(true);
    nif.block_of_mut::<NiPSysRadialFieldModifier>(radial).unwrap().set_radial_type(1.5);
    let mut m = nif.block_of_mut::<NiPSysMeshEmitter>(mesh).unwrap();
    m.as_mut().set_velocity_type(VelocityType::UseDirection);
    m.as_mut().set_emission_type(EmitFrom::FaceSurface);
    let mut em = nif.block_of_mut::<NiPSysEmitter>(mesh).unwrap();
    em.as_mut().set_initial_color([1.0, 0.5, 0.25, 1.0]);
    em.as_mut().set_radius_variation(0.1);

    let nif = reload_in_place(&mut nif);
    let g = nif.block_of::<NiPSysGravityModifier>(gravity).unwrap();
    assert_eq!((g.force_type(), g.strength(), g.world_aligned()), (Some(ForceType::Spherical), 9.8, true));
    let b = nif.block_of::<NiPSysBombModifier>(bomb).unwrap();
    assert_eq!((b.decay_type(), b.symmetry_type()), (Some(DecayType::Exponential), Some(SymmetryType::Planar)));
    let c = nif.block_of::<BsPSysSimpleColorModifier>(color).unwrap();
    assert_eq!((c.color_2(), c.fade_out_percent()), ([0.1, 0.2, 0.3, 0.4], 0.75));
    assert_eq!(nif.block_of::<BsPSysScaleModifier>(scale).unwrap().scales(), [1.0, 0.5, 0.0]);
    let r = nif.block_of::<NiPSysRotationModifier>(rotation).unwrap();
    assert_eq!((r.initial_speed_variation(), r.random_initial_axis()), (0.25, true));
    assert_eq!(nif.block_of::<NiPSysRadialFieldModifier>(radial).unwrap().radial_type(), 1.5);
    let m = nif.block_of::<NiPSysMeshEmitter>(mesh).unwrap();
    assert_eq!((m.velocity_type(), m.emission_type()), (Some(VelocityType::UseDirection), Some(EmitFrom::FaceSurface)));
    assert_eq!((m.initial_color(), m.radius_variation()), ([1.0, 0.5, 0.25, 1.0], 0.1));
}

#[test]
fn particle_controllers() {
    let (mut nif, system) = with_system(NifVersion::SKYRIM_SE);
    let mut e = nif.edit::<NiParticleSystem>(system).unwrap();
    let emitter = e.add_modifier::<NiPSysBoxEmitter>("Emitter");
    let update = e.add_controller::<NiPSysUpdateCtlr, _>();
    let ctlr = e.add_controller::<NiPSysEmitterCtlr, _>();
    let speed = e.add_controller::<NiPSysEmitterSpeedCtlr, _>();
    let active = e.add_controller::<NiPSysModifierActiveCtlr, _>();
    assert_eq!(e.view().controllers().indices().collect::<Vec<_>>(), [update, ctlr, speed, active]);
    let birth = nif.add_interpolator::<NiFloatInterpolator>();
    let visible = nif.add_interpolator::<NiBoolInterpolator>();
    let data = nif.add_animation_data::<NiPSysEmitterCtlrData>();

    let mut c = nif.edit::<NiPSysEmitterCtlr>(ctlr).unwrap();
    assert!(c.set_interpolator(Some(birth)));
    assert!(!c.set_visibility_interpolator(Some(data)) && c.set_visibility_interpolator(Some(visible)));
    assert!(!c.set_emitter_data(Some(birth)) && c.set_emitter_data(Some(data)));
    nif.block_of_mut::<NiPSysModifierCtlr>(ctlr).unwrap().set_modifier_name("Emitter");
    let mut d = nif.block_of_mut::<NiPSysEmitterCtlrData>(data).unwrap();
    assert!(d.as_mut().set_keys(&KeyGroup::new(Interpolation::Linear, vec![Key::new(0.0, 5.0), Key::new(1.0, 10.0)])));
    d.as_mut().set_visibility_keys(&[Key::new(0.0, true), Key::new(2.0, false)]);

    let nif = reload_in_place(&mut nif);
    let c = nif.view::<NiPSysEmitterCtlr>(ctlr).unwrap();
    assert_eq!(c.modifier_name(), "Emitter");
    assert_eq!(nif.view::<NiPSysBoxEmitter>(emitter).unwrap().name(), c.modifier_name());
    assert_eq!(c.interpolator().unwrap().index(), birth);
    assert_eq!(c.visibility_interpolator().unwrap().index(), visible);
    // Saved only before 10.1.0.104.
    assert!(c.emitter_data().is_none());
    let d = nif.block_of::<NiPSysEmitterCtlrData>(data).unwrap();
    assert_eq!(d.keys().keys.len(), 2);
    assert_eq!(d.visibility_keys().iter().map(|k| (k.time, k.value)).collect::<Vec<_>>(), [(0.0, true), (2.0, false)]);
}

#[test]
fn multi_target_emitter_controller() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let master = nif.root_node_mut().unwrap().add_child_of::<BsMasterParticleSystem, _>("Master", Transform::default());
    let mut m = nif.edit::<NiNode>(master).unwrap();
    let system = m.add_particle_system::<NiParticleSystem, _>("Sparks", Transform::default());
    let ctlr = nif.edit::<NiParticleSystem>(system).unwrap().add_controller::<BsPSysMultiTargetEmitterCtlr, _>();
    let mut c = nif.edit::<BsPSysMultiTargetEmitterCtlr>(ctlr).unwrap();
    assert!(!c.set_master_particle_system(Some(system)) && c.set_master_particle_system(Some(master)));
    nif.block_of_mut::<BsPSysMultiTargetEmitterCtlr>(ctlr).unwrap().set_max_emitters(4);
    let nif = reload_in_place(&mut nif);
    let c = nif.view::<BsPSysMultiTargetEmitterCtlr>(ctlr).unwrap();
    assert_eq!((c.master_particle_system().unwrap().index(), c.max_emitters()), (master, 4));
}

#[test]
fn mesh_particle_data() {
    let mut nif = NifFile::create(NifVersion::OBLIVION);
    let node = nif.root_node_mut().unwrap().add_child("Meshes", Transform::default());
    let system =
        nif.root_node_mut().unwrap().add_particle_system::<NiMeshParticleSystem, _>("Mesh", Transform::default());
    let system = nif.view::<NiMeshParticleSystem>(system).unwrap().upcast::<NiParticleSystem, _>().index();
    let data = nif.view::<NiParticleSystem>(system).unwrap().data().unwrap().index();
    let mut d = nif.edit::<NiMeshPSysData>(data).unwrap();
    assert!(!d.set_mesh_node(Some(system)) && d.set_mesh_node(Some(node)));
    let mut b = nif.block_of_mut::<NiMeshPSysData>(data).unwrap();
    b.as_mut().set_default_pool_size(8);
    b.as_mut().set_generation_pool_sizes(&[4, 2]);
    let nif = reload_in_place(&mut nif);
    let d = nif.view::<NiMeshPSysData>(data).unwrap();
    assert_eq!(d.mesh_node().unwrap().index(), node);
    assert_eq!((d.default_pool_size(), d.generation_pool_sizes()), (8, vec![4, 2]));
}

/// Early Oblivion files don't store some emitter, rotation modifier and mesh data fields; they
/// read back as defaults and the fields after them stay intact.
#[test]
fn old_versions() {
    for (version, stored) in [(OLD, false), (NifVersion::OBLIVION, true)] {
        let mut nif = NifFile::create(version);
        let node = nif.root_node_mut().unwrap().add_child("Node", Transform::default());
        let system =
            nif.root_node_mut().unwrap().add_particle_system::<NiMeshParticleSystem, _>("Mesh", Transform::default());
        let data = nif.view::<NiParticleSystem>(system).unwrap().data().unwrap().index();
        let mut e = nif.edit::<NiParticleSystem>(system).unwrap();
        let emitters = [
            e.add_modifier::<NiPSysSphereEmitter>("Sphere"),
            e.add_modifier::<NiPSysCylinderEmitter>("Cylinder"),
            e.add_modifier::<NiPSysBoxEmitter>("Box"),
            e.add_modifier::<BsPSysArrayEmitter>("Array"),
            e.add_modifier::<NiPSysMeshEmitter>("Mesh"),
        ];
        let rotation = e.add_modifier::<NiPSysRotationModifier>("Rotation");
        let after = e.add_modifier::<NiPSysPositionModifier>("After");
        for &emitter in &emitters {
            let mut b = nif.block_of_mut::<NiPSysEmitter>(emitter).unwrap();
            b.as_mut().set_radius_variation(0.5);
            b.as_mut().set_life_span(3.0);
            b.as_mut().set_life_span_variation(1.0);
        }
        nif.block_of_mut::<NiPSysSphereEmitter>(emitters[0]).unwrap().set_radius(7.0);
        nif.block_of_mut::<NiPSysMeshEmitter>(emitters[4]).unwrap().set_emission_axis(Vec3::new(0.0, 1.0, 0.0));
        let mut r = nif.block_of_mut::<NiPSysRotationModifier>(rotation).unwrap();
        r.as_mut().set_initial_speed(2.0);
        r.as_mut().set_initial_angle(1.0);
        r.as_mut().set_initial_axis(Vec3::new(0.0, 0.0, 1.0));
        nif.block_of_mut::<NiMeshPSysData>(data).unwrap().set_default_pool_size(8);
        assert!(nif.edit::<NiMeshPSysData>(data).unwrap().set_mesh_node(Some(node)));

        let nif = reload_in_place(&mut nif);
        for &emitter in &emitters {
            let b = nif.block_of::<NiPSysEmitter>(emitter).unwrap();
            assert_eq!(b.radius_variation(), if stored { 0.5 } else { 0.0 }, "{version:?}");
            assert_eq!((b.life_span(), b.life_span_variation()), (3.0, 1.0), "{version:?}");
        }
        assert_eq!(nif.block_of::<NiPSysSphereEmitter>(emitters[0]).unwrap().radius(), 7.0);
        assert_eq!(nif.block_of::<NiPSysMeshEmitter>(emitters[4]).unwrap().emission_axis(), Vec3::new(0.0, 1.0, 0.0));
        let r = nif.block_of::<NiPSysRotationModifier>(rotation).unwrap();
        assert_eq!(r.initial_angle(), if stored { 1.0 } else { 0.0 }, "{version:?}");
        assert_eq!((r.initial_speed(), r.initial_axis()), (2.0, Vec3::new(0.0, 0.0, 1.0)));
        assert_eq!(nif.block_of::<NiPSysModifier>(after).unwrap().name(), "After");
        let d = nif.view::<NiMeshPSysData>(data).unwrap();
        assert_eq!(d.default_pool_size(), if stored { 8 } else { 0 }, "{version:?}");
        assert_eq!(d.mesh_node().unwrap().index(), node);
    }
}

#[test]
fn geometry_data_base() {
    // Particle data is geometry data, so its vertices work as for shapes.
    let (mut nif, system) = with_system(NifVersion::OBLIVION);
    let data = nif.view::<NiParticleSystem>(system).unwrap().data().unwrap().index();
    nif.block_of_mut::<NiParticlesData>(data).unwrap().set_vertex_count(2);
    let mut g = nif.block_of_mut::<NiGeometryData>(data).unwrap();
    assert!(g.as_mut().set_vertices(&[Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)]));
    let nif = reload_in_place(&mut nif);
    assert_eq!(nif.block_of::<NiGeometryData>(data).unwrap().vertices(), [Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)]);
}
