//! Tests for the old (Morrowind and Oblivion) particle blocks.

use nifly::{
    effects::Plane,
    file::{NifFile, NifVersion},
    geometry::{NiAutoNormalParticles, NiParticleMeshes, NiParticles, NiRotatingParticles},
    object3d::{Transform, Vec3},
    particles::*,
};

use crate::common::{reload_in_place, upstream};

/// 10.0.1.0, the last version with old particles, which saves all their fields.
const LAST: NifVersion = NifVersion::new(0x0A000100, 0, 0);

/// A new file with old particles under the root, a controller on them, and their indices.
fn with_controller(version: NifVersion) -> (NifFile, usize, usize) {
    let mut nif = NifFile::create(version);
    let particles =
        nif.root_node_mut().unwrap().add_particle_system::<NiAutoNormalParticles, _>("Smoke", Transform::default());
    let controller =
        nif.edit::<NiAutoNormalParticles>(particles).unwrap().add_controller::<NiParticleSystemController, _>();
    (nif, particles, controller)
}

#[test]
fn upstream_files() {
    let nif = upstream("TestNifFile_Particles_MW");
    let c = nif.view::<NiParticleSystemController>(10).unwrap();
    assert_eq!(c.emitter().unwrap().index(), 1);
    assert_eq!(c.particle_modifiers().indices().collect::<Vec<_>>(), [11, 12]);
    assert_eq!(c.particle_colliders().count() + c.emitter_modifiers().count(), 0);
    assert_eq!((c.particles().len(), c.active_count()), (20, 13));
    assert!(c.use_birth_rate() && !c.reset_particle_system());
    let g = nif.view::<NiGravity>(11).unwrap();
    assert_eq!((g.next_modifier().unwrap().index(), g.particle_controller().unwrap().index()), (12, 10));
    assert_eq!(g.field_type(), Some(FieldType::Wind));
    assert!(nif.view::<NiParticleGrowFade>(12).unwrap().next_modifier().is_none());
    let d = nif.block_of::<NiParticlesData>(17).unwrap();
    assert_eq!((d.particle_radius(), d.sizes().len()), (100.0, 20));

    let nif = upstream("TestNifFile_RotatingParticles_MW");
    let r = nif.block_of::<NiParticleRotation>(16).unwrap();
    assert!(r.random_initial_axis());
    assert_eq!(r.initial_axis(), Vec3::new(1.0, 0.0, 0.0));
    let d = nif.block_of::<NiRotatingParticlesData>(22).unwrap();
    assert!(d.has_rotations_2());
    assert_eq!(d.rotations_2().len(), 8);
}

#[test]
fn add_old_particles() {
    let mut nif = NifFile::create(NifVersion::MORROWIND);
    let mut root = nif.root_node_mut().unwrap();
    let auto = root.add_particle_system::<NiAutoNormalParticles, _>("Auto", Transform::default());
    let rotating = root.add_particle_system::<NiRotatingParticles, _>("Rotating", Transform::default());
    let meshes = root.add_particle_system::<NiParticleMeshes, _>("Meshes", Transform::default());
    assert_eq!(nif.root_node().unwrap().children().indices().collect::<Vec<_>>(), [auto, rotating, meshes]);
    let block_name = |nif: &NifFile, i| {
        let d = nif.view::<NiParticles>(i).unwrap().particles_data().unwrap();
        d.block_name().to_string()
    };
    assert_eq!(block_name(&nif, auto), "NiAutoNormalParticlesData");
    assert_eq!(block_name(&nif, rotating), "NiRotatingParticlesData");
    assert_eq!(block_name(&nif, meshes), "NiParticleMeshesData");

    let rotating_data = nif.view::<NiParticles>(rotating).unwrap().particles_data().unwrap().index();
    let mut p = nif.edit::<NiAutoNormalParticles>(auto).unwrap();
    assert!(!p.set_particles_data(Some(rotating)), "not particle data");
    assert!(p.set_particles_data(Some(rotating_data)));
    let nif = reload_in_place(&mut nif);
    assert_eq!(nif.view::<NiAutoNormalParticles>(auto).unwrap().particles_data().unwrap().index(), rotating_data);
}

#[test]
fn modifier_chains() {
    let (mut nif, _, controller) = with_controller(NifVersion::MORROWIND);
    let mut c = nif.edit::<NiParticleSystemController>(controller).unwrap();
    let gravity = c.add_particle_modifier::<NiGravity, _>();
    let plane = c.add_particle_modifier::<NiPlanarCollider, _>();
    let grow = c.add_particle_modifier::<NiParticleGrowFade, _>();
    let sphere = c.add_particle_modifier::<NiSphericalCollider, _>();
    let emitter = c.add_particle_modifier::<NiEmitterModifier, _>();
    let emitter_2 = c.add_particle_modifier::<NiEmitterModifier, _>();
    let c = c.view();
    assert_eq!(c.particle_modifiers().indices().collect::<Vec<_>>(), [gravity, grow]);
    assert_eq!(c.particle_colliders().indices().collect::<Vec<_>>(), [plane, sphere]);
    assert_eq!(c.emitter_modifiers().indices().collect::<Vec<_>>(), [emitter, emitter_2]);
    for i in [gravity, plane, grow, sphere] {
        assert_eq!(nif.view::<NiParticleModifier>(i).unwrap().particle_controller().unwrap().index(), controller);
    }
    assert_eq!(nif.view::<NiEmitterModifier>(emitter).unwrap().particle_controller().unwrap().index(), controller);

    let mut g = nif.edit::<NiGravity>(gravity).unwrap();
    assert!(!g.set_next_modifier(Some(gravity)), "a modifier can't follow itself");
    assert!(!g.set_next_modifier(Some(emitter)), "an emitter modifier isn't a particle modifier");
    assert!(!g.set_particle_controller(Some(gravity)));
    assert!(!nif.edit::<NiParticleGrowFade>(grow).unwrap().set_next_modifier(Some(gravity)), "a loop");
    let mut e = nif.edit::<NiEmitterModifier>(emitter).unwrap();
    assert!(!e.set_next_modifier(Some(gravity)), "a particle modifier isn't an emitter modifier");
    assert!(e.set_next_modifier(None));
    assert!(e.view().next_modifier().is_none());

    let mut c = nif.edit::<NiParticleSystemController>(controller).unwrap();
    assert!(!c.set_particle_modifiers(&[grow, grow]) && !c.set_particle_modifiers(&[emitter]));
    assert!(!c.set_particle_colliders(&[gravity, plane]), "the first must be a collider");
    assert!(!c.set_emitter_modifiers(&[gravity]));
    assert!(c.set_particle_modifiers(&[grow, gravity]));
    assert!(c.set_particle_colliders(&[sphere, plane, gravity]));
    assert!(c.set_emitter_modifiers(&[emitter_2]));
    assert!(!c.set_emitter(Some(controller)));
    assert!(c.set_emitter(Some(0)));

    let nif = reload_in_place(&mut nif);
    let c = nif.view::<NiParticleSystemController>(controller).unwrap();
    assert_eq!(c.emitter().unwrap().index(), 0);
    assert_eq!(c.particle_modifiers().indices().collect::<Vec<_>>(), [grow, gravity]);
    assert_eq!(c.particle_colliders().indices().collect::<Vec<_>>(), [sphere, plane, gravity]);
    assert_eq!(c.emitter_modifiers().indices().collect::<Vec<_>>(), [emitter_2]);
}

#[test]
fn modifier_refs() {
    let (mut nif, particles, controller) = with_controller(NifVersion::MORROWIND);
    let mut c = nif.edit::<NiParticleSystemController>(controller).unwrap();
    let color = c.add_particle_modifier::<NiParticleColorModifier, _>();
    let meshes = c.add_particle_modifier::<NiParticleMeshModifier, _>();
    let color_data = nif.add_animation_data::<nifly::animation::NiColorData>();
    let mut m = nif.edit::<NiParticleColorModifier>(color).unwrap();
    assert!(!m.set_color_data(Some(color)) && m.set_color_data(Some(color_data)));
    let mut m = nif.edit::<NiParticleMeshModifier>(meshes).unwrap();
    assert!(!m.set_particle_meshes(&[controller]) && m.set_particle_meshes(&[particles, 0]));

    let mesh_particles =
        nif.root_node_mut().unwrap().add_particle_system::<NiParticleMeshes, _>("Meshes", Transform::default());
    let data = nif.view::<NiParticles>(mesh_particles).unwrap().particles_data().unwrap().index();
    let mut d = nif.edit::<NiParticleMeshesData>(data).unwrap();
    assert!(!d.set_container_node(Some(particles)), "the container must be a node");
    assert!(d.set_container_node(Some(0)));

    let nif = reload_in_place(&mut nif);
    assert_eq!(nif.view::<NiParticleColorModifier>(color).unwrap().color_data().unwrap().index(), color_data);
    let m = nif.view::<NiParticleMeshModifier>(meshes).unwrap();
    assert_eq!(m.particle_meshes().indices().collect::<Vec<_>>(), [particles, 0]);
    assert_eq!(nif.view::<NiParticleMeshesData>(data).unwrap().container_node().unwrap().index(), 0);
}

#[test]
fn modifier_fields() {
    let (mut nif, _, controller) = with_controller(LAST);
    let mut c = nif.edit::<NiParticleSystemController>(controller).unwrap();
    let gravity = c.add_particle_modifier::<NiGravity, _>();
    let grow = c.add_particle_modifier::<NiParticleGrowFade, _>();
    let rotation = c.add_particle_modifier::<NiParticleRotation, _>();
    let bomb = c.add_particle_modifier::<NiParticleBomb, _>();
    let plane = c.add_particle_modifier::<NiPlanarCollider, _>();
    let sphere = c.add_particle_modifier::<NiSphericalCollider, _>();
    let v = Vec3::new(1.0, 2.0, 3.0);

    let g = nif.block_of_mut::<NiGravity>(gravity).unwrap();
    g.set_decay(0.5);
    g.set_force(-9.8);
    g.set_field_type(FieldType::Point);
    g.set_position(v);
    g.set_direction(v);
    let g = nif.block_of_mut::<NiParticleGrowFade>(grow).unwrap();
    g.set_grow(0.25);
    g.set_fade(0.75);
    let r = nif.block_of_mut::<NiParticleRotation>(rotation).unwrap();
    r.set_random_initial_axis(true);
    r.set_initial_axis(v);
    r.set_rotation_speed(2.0);
    let b = nif.block_of_mut::<NiParticleBomb>(bomb).unwrap();
    b.set_decay(1.0);
    b.set_duration(2.0);
    b.set_delta_v(3.0);
    b.set_start(4.0);
    b.set_decay_type(DecayType::Exponential);
    b.set_symmetry_type(SymmetryType::Planar);
    b.set_position(v);
    b.set_direction(v);
    let p = nif.block_of_mut::<NiParticleCollider>(plane).unwrap();
    p.set_bounce(0.5);
    p.set_spawn_on_collide(true);
    p.set_die_on_collide(true);
    let p = nif.block_of_mut::<NiPlanarCollider>(plane).unwrap();
    p.set_width(2.0);
    p.set_height(3.0);
    p.set_position(v);
    p.set_x_axis(Vec3::new(1.0, 0.0, 0.0));
    p.set_y_axis(Vec3::new(0.0, 1.0, 0.0));
    p.set_plane(Plane { normal: Vec3::new(0.0, 0.0, 1.0), constant: 5.0 });
    let s = nif.block_of_mut::<NiSphericalCollider>(sphere).unwrap();
    s.set_radius(4.0);
    s.set_position(v);

    let nif = reload_in_place(&mut nif);
    let g = nif.block_of::<NiGravity>(gravity).unwrap();
    assert_eq!((g.decay(), g.force(), g.field_type()), (0.5, -9.8, Some(FieldType::Point)));
    assert_eq!((g.position(), g.direction()), (v, v));
    let g = nif.block_of::<NiParticleGrowFade>(grow).unwrap();
    assert_eq!((g.grow(), g.fade()), (0.25, 0.75));
    let r = nif.block_of::<NiParticleRotation>(rotation).unwrap();
    assert_eq!((r.random_initial_axis(), r.initial_axis(), r.rotation_speed()), (true, v, 2.0));
    let b = nif.block_of::<NiParticleBomb>(bomb).unwrap();
    assert_eq!((b.decay(), b.duration(), b.delta_v(), b.start()), (1.0, 2.0, 3.0, 4.0));
    assert_eq!((b.decay_type(), b.symmetry_type()), (Some(DecayType::Exponential), Some(SymmetryType::Planar)));
    assert_eq!((b.position(), b.direction()), (v, v));
    let p = nif.block_of::<NiPlanarCollider>(plane).unwrap();
    assert_eq!((p.bounce(), p.spawn_on_collide(), p.die_on_collide()), (0.5, true, true));
    assert_eq!((p.width(), p.height(), p.position()), (2.0, 3.0, v));
    assert_eq!((p.x_axis(), p.y_axis()), (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)));
    assert_eq!(p.plane(), Plane { normal: Vec3::new(0.0, 0.0, 1.0), constant: 5.0 });
    let s = nif.block_of::<NiSphericalCollider>(sphere).unwrap();
    assert_eq!((s.radius(), s.position()), (4.0, v));
}

#[test]
fn controller_fields() {
    let (mut nif, _, controller) = with_controller(LAST);
    let v = Vec3::new(1.0, 2.0, 3.0);
    let info = ParticleInfo {
        velocity: v,
        rotation_axis: Vec3::new(0.0, 0.0, 1.0),
        age: 0.5,
        life_span: 2.0,
        last_update: 0.25,
        spawn_generation: 1,
        code: 7,
    };
    let c = nif.block_of_mut::<NiParticleSystemController>(controller).unwrap();
    c.set_speed(10.0);
    c.set_speed_variation(1.0);
    c.set_declination(0.5);
    c.set_declination_variation(0.1);
    c.set_planar_angle(0.2);
    c.set_planar_angle_variation(0.3);
    c.set_initial_normal(v);
    c.set_initial_color([0.5, 0.25, 1.0, 0.75]);
    c.set_initial_size(4.0);
    c.set_emit_start_time(1.0);
    c.set_emit_stop_time(5.0);
    c.set_reset_particle_system(true);
    c.set_birth_rate(12.0);
    c.set_lifetime(3.0);
    c.set_lifetime_variation(0.5);
    c.set_use_birth_rate(true);
    c.set_spawn_on_death(true);
    c.set_emitter_dimensions(v);
    c.set_spawn_generations(2);
    c.set_percent_spawned(0.5);
    c.set_spawn_multiplier(3);
    c.set_spawn_speed_chaos(0.1);
    c.set_spawn_direction_chaos(0.2);
    c.set_active_count(1);
    c.set_static_target_bound(true);
    assert!(c.set_particles(&[info; 2]));
    assert!(!c.set_particles(&vec![info; 65536]), "a 16-bit count");

    let nif = reload_in_place(&mut nif);
    let c = nif.block_of::<NiParticleSystemController>(controller).unwrap();
    assert_eq!((c.speed(), c.speed_variation(), c.declination(), c.declination_variation()), (10.0, 1.0, 0.5, 0.1));
    assert_eq!((c.planar_angle(), c.planar_angle_variation(), c.initial_normal()), (0.2, 0.3, v));
    assert_eq!((c.initial_color(), c.initial_size()), ([0.5, 0.25, 1.0, 0.75], 4.0));
    assert_eq!((c.emit_start_time(), c.emit_stop_time(), c.birth_rate()), (1.0, 5.0, 12.0));
    assert_eq!((c.lifetime(), c.lifetime_variation(), c.emitter_dimensions()), (3.0, 0.5, v));
    assert!(c.reset_particle_system() && c.use_birth_rate() && c.spawn_on_death() && c.static_target_bound());
    assert_eq!((c.spawn_generations(), c.percent_spawned(), c.spawn_multiplier()), (2, 0.5, 3));
    assert_eq!((c.spawn_speed_chaos(), c.spawn_direction_chaos(), c.active_count()), (0.1, 0.2, 1));
    assert_eq!(c.particles(), [info; 2]);
}

#[test]
fn bsp_array_controller() {
    let (mut nif, particles, _) = with_controller(NifVersion::MORROWIND);
    let c = nif.edit::<NiAutoNormalParticles>(particles).unwrap().add_controller::<NiBspArrayController, _>();
    let mut c = nif.edit::<NiBspArrayController>(c).unwrap();
    let gravity = c.add_particle_modifier::<NiGravity, _>();
    assert_eq!(c.view().particle_modifiers().indices().collect::<Vec<_>>(), [gravity]);
}

#[test]
fn rotating_particles_data() {
    let mut nif = NifFile::create(NifVersion::MORROWIND);
    let particles =
        nif.root_node_mut().unwrap().add_particle_system::<NiRotatingParticles, _>("Spin", Transform::default());
    let data = nif.view::<NiParticles>(particles).unwrap().particles_data().unwrap().index();
    let d = nif.block_of_mut::<NiRotatingParticlesData>(data).unwrap();
    d.set_vertex_count(2);
    let q = [0.0, 0.0, 0.0, 1.0];
    assert!(!d.set_rotations_2(&[q]), "one per vertex");
    assert!(d.set_rotations_2(&[q; 2]));
    d.set_has_rotations_2(true);

    let nif = reload_in_place(&mut nif);
    let d = nif.block_of::<NiRotatingParticlesData>(data).unwrap();
    assert!(d.has_rotations_2());
    assert_eq!(d.rotations_2(), [q; 2]);
}

#[test]
fn morrowind_drops_later_fields() {
    let (mut nif, _, controller) = with_controller(NifVersion::MORROWIND);
    let mut c = nif.edit::<NiParticleSystemController>(controller).unwrap();
    let bomb = c.add_particle_modifier::<NiParticleBomb, _>();
    let sphere = c.add_particle_modifier::<NiSphericalCollider, _>();
    nif.block_of_mut::<NiParticleBomb>(bomb).unwrap().set_symmetry_type(SymmetryType::Planar);
    nif.block_of_mut::<NiParticleCollider>(sphere).unwrap().set_die_on_collide(true);

    let nif = reload_in_place(&mut nif);
    assert_eq!(nif.block_of::<NiParticleBomb>(bomb).unwrap().symmetry_type(), Some(SymmetryType::Spherical));
    assert!(!nif.block_of::<NiParticleCollider>(sphere).unwrap().die_on_collide());
}
