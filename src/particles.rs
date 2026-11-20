//! Particle systems: data, systems, modifiers, emitters, colliders and their controllers.
//!
//! An [`NiParticleSystem`] has [particle data](NiPSysData) and a list of
//! [modifiers](NiPSysModifier): emitters create particles, and other modifiers move, age, color
//! and kill them. Add a system with [`Edit::add_particle_system`] and its modifiers with
//! [`Edit::add_modifier`]. Controllers such as [`NiPSysEmitterCtlr`] animate the modifiers.
//!
//! Morrowind and Oblivion files may also use the old particle blocks: an
//! [`NiParticles`] shape, also added with
//! [`Edit::add_particle_system`], and an [`NiParticleSystemController`] that emits the particles
//! and runs chains of [`NiParticleModifier`]s, added with [`Edit::add_particle_modifier`].

use std::pin::Pin;

use bstr::{BStr, ByteSlice};
use int_enum::IntEnum;
use nifly_sys::{
    nif_file::ffi::{RefKind, RefList},
    particles::ffi,
};

use crate::{
    animation::{
        group_from_ffi, group_to_ffi, key_from_ffi, key_to_ffi, same, vec3, vec3_to_ffi, HasInterpolator, Key,
        KeyGroup, NewAnimationData, NewController, NiColorData, NiInterpolator, NiPSysModifierCtlr, NiTimeController,
    },
    effects::Plane,
    file::{index_from_ffi, Index},
    geometry::{NiAutoNormalParticles, NiGeometryData, NiParticleMeshes, NiParticles, NiRotatingParticles},
    handle::{Edit, View},
    iter::{BlockIter, RefIndices},
    nodes::{BsMasterParticleSystem, NiNode},
    object3d::{BoundingSphere, Transform, Vec3},
    objects::{IsA, NiAvObject, NiObject, NiType, Sealed},
    shaders::{BsShaderProperty, NiAlphaProperty},
    skin::NiBoneContainer,
};

ni_types! { ffi;
    /// Particle data: each particle's position, size and rotation, one per vertex.
    ///
    /// Vertices hold the positions. On save, the per-particle lists that are saved are cut or padded with zeros to the vertex count.
    NiParticlesData(NiParticlesData): NiGeometryData => upcast_ni_particles_data, downcast_ni_particles_data;
    /// Morrowind and Oblivion only. Data of an [`NiAutoNormalParticles`].
    NiAutoNormalParticlesData(NiAutoNormalParticlesData): NiParticlesData => upcast_ni_auto_normal_particles_data, downcast_ni_auto_normal_particles_data;
    /// Data of an [`NiRotatingParticles`], and the base of particle system data.
    NiRotatingParticlesData(NiRotatingParticlesData): NiParticlesData => upcast_ni_rotating_particles_data, downcast_ni_rotating_particles_data;
    /// Morrowind and Oblivion only. Data of an [`NiParticleMeshes`], with a [container node](crate::handle::View::container_node).
    NiParticleMeshesData(NiParticleMeshesData): NiRotatingParticlesData => upcast_ni_particle_meshes_data, downcast_ni_particle_meshes_data;
    /// The data of an [`NiParticleSystem`]: particle data plus each particle's motion.
    NiPSysData(NiPSysData): NiRotatingParticlesData => upcast_ni_psys_data, downcast_ni_psys_data;
    /// The data of an [`NiMeshParticleSystem`], whose particles are meshes.
    NiMeshPSysData(NiMeshPSysData): NiPSysData => upcast_ni_mesh_psys_data, downcast_ni_mesh_psys_data;
    /// The data of a [`BsStripParticleSystem`].
    ///
    /// The `BSStripPSysData` block.
    BsStripPSysData(BSStripPSysData): NiPSysData => upcast_bs_strip_psys_data, downcast_bs_strip_psys_data;
    /// Keys of an [`NiPSysEmitterCtlr`]: birth rate and emitter activity. Used up to 10.1.0.103; later controllers use interpolators.
    NiPSysEmitterCtlrData(NiPSysEmitterCtlrData): NiObject => upcast_ni_psys_emitter_ctlr_data, downcast_ni_psys_emitter_ctlr_data;
    /// A particle system: its [data](crate::handle::View::data) and the [modifiers](crate::handle::View::modifiers) that emit and move the particles. Add one with [`Edit::add_particle_system`](crate::handle::Edit::add_particle_system).
    NiParticleSystem(NiParticleSystem): NiAvObject => upcast_ni_particle_system, downcast_ni_particle_system;
    /// A particle system whose particles are meshes.
    NiMeshParticleSystem(NiMeshParticleSystem): NiParticleSystem => upcast_ni_mesh_particle_system, downcast_ni_mesh_particle_system;
    /// A particle system drawn as strips.
    ///
    /// The `BSStripParticleSystem` block.
    BsStripParticleSystem(BSStripParticleSystem): NiParticleSystem => upcast_bs_strip_particle_system, downcast_bs_strip_particle_system;
    /// Base class of particle modifiers, which emit, move and change the particles of their [particle system](crate::handle::View::particle_system).
    NiPSysModifier(NiPSysModifier): NiObject => upcast_ni_psys_modifier, downcast_ni_psys_modifier;
    /// Updates the strips of a [`BsStripParticleSystem`].
    ///
    /// The `BSPSysStripUpdateModifier` block.
    BsPSysStripUpdateModifier(BSPSysStripUpdateModifier): NiPSysModifier => upcast_bs_psys_strip_update_modifier, downcast_bs_psys_strip_update_modifier;
    /// Spawns new particles from existing ones, for an [`NiPSysAgeDeathModifier`] or [`NiPSysCollider`].
    NiPSysSpawnModifier(NiPSysSpawnModifier): NiPSysModifier => upcast_ni_psys_spawn_modifier, downcast_ni_psys_spawn_modifier;
    /// Ages particles and kills them at the end of their life span.
    NiPSysAgeDeathModifier(NiPSysAgeDeathModifier): NiPSysModifier => upcast_ni_psys_age_death_modifier, downcast_ni_psys_age_death_modifier;
    /// Emits fewer and smaller particles with distance.
    ///
    /// The `BSPSysLODModifier` block.
    BsPSysLodModifier(BSPSysLODModifier): NiPSysModifier => upcast_bs_psys_lod_modifier, downcast_bs_psys_lod_modifier;
    /// Fades particles in and out and through three colors over their life.
    ///
    /// The `BSPSysSimpleColorModifier` block.
    BsPSysSimpleColorModifier(BSPSysSimpleColorModifier): NiPSysModifier => upcast_bs_psys_simple_color_modifier, downcast_bs_psys_simple_color_modifier;
    /// Rotates particles.
    NiPSysRotationModifier(NiPSysRotationModifier): NiPSysModifier => upcast_ni_psys_rotation_modifier, downcast_ni_psys_rotation_modifier;
    /// Scales particles over their life.
    ///
    /// The `BSPSysScaleModifier` block.
    BsPSysScaleModifier(BSPSysScaleModifier): NiPSysModifier => upcast_bs_psys_scale_modifier, downcast_bs_psys_scale_modifier;
    /// Pulls particles along an axis or toward a point.
    NiPSysGravityModifier(NiPSysGravityModifier): NiPSysModifier => upcast_ni_psys_gravity_modifier, downcast_ni_psys_gravity_modifier;
    /// Moves particles by their velocity.
    NiPSysPositionModifier(NiPSysPositionModifier): NiPSysModifier => upcast_ni_psys_position_modifier, downcast_ni_psys_position_modifier;
    /// Updates the system's bounds.
    NiPSysBoundUpdateModifier(NiPSysBoundUpdateModifier): NiPSysModifier => upcast_ni_psys_bound_update_modifier, downcast_ni_psys_bound_update_modifier;
    /// Slows particles down along an axis.
    NiPSysDragModifier(NiPSysDragModifier): NiPSysModifier => upcast_ni_psys_drag_modifier, downcast_ni_psys_drag_modifier;
    /// Gives new particles the velocity of a node.
    ///
    /// The `BSPSysInheritVelocityModifier` block.
    BsPSysInheritVelocityModifier(BSPSysInheritVelocityModifier): NiPSysModifier => upcast_bs_psys_inherit_velocity_modifier, downcast_bs_psys_inherit_velocity_modifier;
    /// Animates particles through the [subtexture offsets](NiParticlesData::subtexture_offsets).
    ///
    /// The `BSPSysSubTexModifier` block.
    BsPSysSubTexModifier(BSPSysSubTexModifier): NiPSysModifier => upcast_bs_psys_sub_tex_modifier, downcast_bs_psys_sub_tex_modifier;
    /// Pushes particles away from a point, axis or plane.
    NiPSysBombModifier(NiPSysBombModifier): NiPSysModifier => upcast_ni_psys_bomb_modifier, downcast_ni_psys_bomb_modifier;
    /// Colors particles over their life from [color data](crate::handle::View::color_data).
    NiPSysColorModifier(NiPSysColorModifier): NiPSysModifier => upcast_ni_psys_color_modifier, downcast_ni_psys_color_modifier;
    /// Grows new particles and shrinks dying ones.
    NiPSysGrowFadeModifier(NiPSysGrowFadeModifier): NiPSysModifier => upcast_ni_psys_grow_fade_modifier, downcast_ni_psys_grow_fade_modifier;
    /// Updates the meshes of an [`NiMeshParticleSystem`].
    NiPSysMeshUpdateModifier(NiPSysMeshUpdateModifier): NiPSysModifier => upcast_ni_psys_mesh_update_modifier, downcast_ni_psys_mesh_update_modifier;
    /// Base class of modifiers that apply a force field around their [modifier object](crate::handle::View::modifier_object).
    NiPSysFieldModifier(NiPSysFieldModifier): NiPSysModifier => upcast_ni_psys_field_modifier, downcast_ni_psys_field_modifier;
    /// Spins particles around an axis.
    NiPSysVortexFieldModifier(NiPSysVortexFieldModifier): NiPSysFieldModifier => upcast_ni_psys_vortex_field_modifier, downcast_ni_psys_vortex_field_modifier;
    /// Pulls particles in a direction.
    NiPSysGravityFieldModifier(NiPSysGravityFieldModifier): NiPSysFieldModifier => upcast_ni_psys_gravity_field_modifier, downcast_ni_psys_gravity_field_modifier;
    /// Slows particles down.
    NiPSysDragFieldModifier(NiPSysDragFieldModifier): NiPSysFieldModifier => upcast_ni_psys_drag_field_modifier, downcast_ni_psys_drag_field_modifier;
    /// Moves particles at random.
    NiPSysTurbulenceFieldModifier(NiPSysTurbulenceFieldModifier): NiPSysFieldModifier => upcast_ni_psys_turbulence_field_modifier, downcast_ni_psys_turbulence_field_modifier;
    /// Blows particles along like wind.
    NiPSysAirFieldModifier(NiPSysAirFieldModifier): NiPSysFieldModifier => upcast_ni_psys_air_field_modifier, downcast_ni_psys_air_field_modifier;
    /// Pulls particles toward a point.
    NiPSysRadialFieldModifier(NiPSysRadialFieldModifier): NiPSysFieldModifier => upcast_ni_psys_radial_field_modifier, downcast_ni_psys_radial_field_modifier;
    /// Blows particles along with the game's wind.
    ///
    /// The `BSWindModifier` block.
    BsWindModifier(BSWindModifier): NiPSysModifier => upcast_bs_wind_modifier, downcast_bs_wind_modifier;
    /// Moves particles that leave a box back into it.
    ///
    /// The `BSPSysRecycleBoundModifier` block.
    BsPSysRecycleBoundModifier(BSPSysRecycleBoundModifier): NiPSysModifier => upcast_bs_psys_recycle_bound_modifier, downcast_bs_psys_recycle_bound_modifier;
    /// Moves particles with Havok physics.
    ///
    /// The `BSPSysHavokUpdateModifier` block.
    BsPSysHavokUpdateModifier(BSPSysHavokUpdateModifier): NiPSysModifier => upcast_bs_psys_havok_update_modifier, downcast_bs_psys_havok_update_modifier;
    /// Gives particles their parent's velocity.
    ///
    /// The `BSParentVelocityModifier` block.
    BsParentVelocityModifier(BSParentVelocityModifier): NiPSysModifier => upcast_bs_parent_velocity_modifier, downcast_bs_parent_velocity_modifier;
    /// Bounces particles off its [colliders](crate::handle::View::colliders).
    NiPSysColliderManager(NiPSysColliderManager): NiPSysModifier => upcast_ni_psys_collider_manager, downcast_ni_psys_collider_manager;
    /// Base class of particle emitters, which create particles.
    NiPSysEmitter(NiPSysEmitter): NiPSysModifier => upcast_ni_psys_emitter, downcast_ni_psys_emitter;
    /// Base class of emitters that create particles in a shape around their [modifier object](crate::handle::View::modifier_object).
    NiPSysVolumeEmitter(NiPSysVolumeEmitter): NiPSysEmitter => upcast_ni_psys_volume_emitter, downcast_ni_psys_volume_emitter;
    /// Emits particles from a sphere.
    NiPSysSphereEmitter(NiPSysSphereEmitter): NiPSysVolumeEmitter => upcast_ni_psys_sphere_emitter, downcast_ni_psys_sphere_emitter;
    /// Emits particles from a cylinder.
    NiPSysCylinderEmitter(NiPSysCylinderEmitter): NiPSysVolumeEmitter => upcast_ni_psys_cylinder_emitter, downcast_ni_psys_cylinder_emitter;
    /// Emits particles from a box.
    NiPSysBoxEmitter(NiPSysBoxEmitter): NiPSysVolumeEmitter => upcast_ni_psys_box_emitter, downcast_ni_psys_box_emitter;
    /// Emits particles from an array of points.
    ///
    /// The `BSPSysArrayEmitter` block.
    BsPSysArrayEmitter(BSPSysArrayEmitter): NiPSysVolumeEmitter => upcast_bs_psys_array_emitter, downcast_bs_psys_array_emitter;
    /// Emits particles from the surface of [meshes](crate::handle::View::particle_meshes).
    NiPSysMeshEmitter(NiPSysMeshEmitter): NiPSysEmitter => upcast_ni_psys_mesh_emitter, downcast_ni_psys_mesh_emitter;
    /// Base class of particle colliders, chained from an [`NiPSysColliderManager`].
    NiPSysCollider(NiPSysCollider): NiObject => upcast_ni_psys_collider, downcast_ni_psys_collider;
    /// Bounces particles off a sphere.
    NiPSysSphericalCollider(NiPSysSphericalCollider): NiPSysCollider => upcast_ni_psys_spherical_collider, downcast_ni_psys_spherical_collider;
    /// Bounces particles off a plane.
    NiPSysPlanarCollider(NiPSysPlanarCollider): NiPSysCollider => upcast_ni_psys_planar_collider, downcast_ni_psys_planar_collider;
    /// Animates an emitter's birth rate and switches it on and off.
    NiPSysEmitterCtlr(NiPSysEmitterCtlr): NiPSysModifierCtlr => upcast_ni_psys_emitter_ctlr, downcast_ni_psys_emitter_ctlr;
    /// An emitter controller of a [`BsMasterParticleSystem`].
    ///
    /// The `BSPSysMultiTargetEmitterCtlr` block.
    BsPSysMultiTargetEmitterCtlr(BSPSysMultiTargetEmitterCtlr): NiPSysEmitterCtlr => upcast_bs_psys_multi_target_emitter_ctlr, downcast_bs_psys_multi_target_emitter_ctlr;
    /// Morrowind and Oblivion only. Base class of the old particle modifiers, chained by [`next_modifier`](crate::handle::View::next_modifier) from an [`NiParticleSystemController`].
    NiParticleModifier(NiParticleModifier): NiObject => upcast_ni_particle_modifier, downcast_ni_particle_modifier;
    /// Morrowind and Oblivion only. Pulls old particles in a direction or toward a point.
    NiGravity(NiGravity): NiParticleModifier => upcast_ni_gravity, downcast_ni_gravity;
    /// Morrowind and Oblivion only. Grows and shrinks old particles.
    NiParticleGrowFade(NiParticleGrowFade): NiParticleModifier => upcast_ni_particle_grow_fade, downcast_ni_particle_grow_fade;
    /// Morrowind and Oblivion only. Colors old particles over their life.
    NiParticleColorModifier(NiParticleColorModifier): NiParticleModifier => upcast_ni_particle_color_modifier, downcast_ni_particle_color_modifier;
    /// Morrowind and Oblivion only. Rotates old particles.
    NiParticleRotation(NiParticleRotation): NiParticleModifier => upcast_ni_particle_rotation, downcast_ni_particle_rotation;
    /// Morrowind and Oblivion only. Pushes old particles away.
    NiParticleBomb(NiParticleBomb): NiParticleModifier => upcast_ni_particle_bomb, downcast_ni_particle_bomb;
    /// Morrowind and Oblivion only. Gives old particles meshes.
    NiParticleMeshModifier(NiParticleMeshModifier): NiParticleModifier => upcast_ni_particle_mesh_modifier, downcast_ni_particle_mesh_modifier;
    /// Morrowind and Oblivion only. Base class of the old particle colliders.
    NiParticleCollider(NiParticleCollider): NiParticleModifier => upcast_ni_particle_collider, downcast_ni_particle_collider;
    /// Morrowind and Oblivion only. Bounces old particles off a plane.
    NiPlanarCollider(NiPlanarCollider): NiParticleCollider => upcast_ni_planar_collider, downcast_ni_planar_collider;
    /// Morrowind and Oblivion only. Bounces old particles off a sphere.
    NiSphericalCollider(NiSphericalCollider): NiParticleCollider => upcast_ni_spherical_collider, downcast_ni_spherical_collider;
    /// Morrowind and Oblivion only. Changes an old particle emitter. Chained by [`next_modifier`](crate::handle::View::next_modifier) from an [`NiParticleSystemController`].
    NiEmitterModifier(NiEmitterModifier): NiObject => upcast_ni_emitter_modifier, downcast_ni_emitter_modifier;
    /// Morrowind and Oblivion only. Emits and updates old particles, from its [emitter](crate::handle::View::emitter), through its [particle modifiers](crate::handle::View::particle_modifiers), [colliders](crate::handle::View::particle_colliders) and [emitter modifiers](crate::handle::View::emitter_modifiers). Add one with [`Edit::add_controller`](crate::handle::Edit::add_controller).
    NiParticleSystemController(NiParticleSystemController): NiTimeController => upcast_ni_particle_system_controller, downcast_ni_particle_system_controller;
    /// Morrowind and Oblivion only. An old particle controller used with an [`NiBsParticleNode`](crate::nodes::NiBsParticleNode).
    ///
    /// The `NiBSPArrayController` block.
    NiBspArrayController(NiBSPArrayController): NiParticleSystemController => upcast_ni_bsp_array_controller, downcast_ni_bsp_array_controller;
}

/// How an [`NiPSysGravityModifier`] pulls.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum ForceType {
    /// Along the gravity axis.
    Planar = 0,
    /// Toward the modifier object.
    Spherical = 1,
    Unknown = 2,
}

/// How the force of an [`NiPSysBombModifier`] or [`NiParticleBomb`] falls off with distance.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum DecayType {
    None = 0,
    Linear = 1,
    Exponential = 2,
}

/// The shape of the force of an [`NiPSysBombModifier`] or [`NiParticleBomb`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum SymmetryType {
    /// Away from a point.
    Spherical = 0,
    /// Away from an axis.
    Cylindrical = 1,
    /// Away from a plane.
    Planar = 2,
}

/// How an [`NiGravity`] pulls.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum FieldType {
    /// Along the [direction](NiGravity::direction).
    Wind = 0,
    /// Toward the [position](NiGravity::position).
    Point = 1,
}

/// How an [`NiPSysMeshEmitter`] picks the velocity of new particles.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum VelocityType {
    /// Along the mesh normals.
    UseNormals = 0,
    /// In a random direction.
    UseRandom = 1,
    /// Along the [emission axis](NiPSysMeshEmitter::emission_axis).
    UseDirection = 2,
}

/// Where on its meshes an [`NiPSysMeshEmitter`] creates particles.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum EmitFrom {
    Vertices = 0,
    FaceCenter = 1,
    EdgeCenter = 2,
    FaceSurface = 3,
    EdgeSurface = 4,
}

bitflags::bitflags! {
    /// How particles of an [`NiParticlesData`] are stretched. Other bits are kept as stored.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct AspectFlags: u16 {
        const VELOCITY_ORIENTATION = 1;
        const INITIAL_ROTATION_FROM_VELOCITY = 1 << 1;
        const SPEED_TO_ASPECT = 1 << 8;
        const _ = !0;
    }
}

/// The motion of one particle of an [`NiPSysData`] or [`NiParticleSystemController`].
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ParticleInfo {
    pub velocity: Vec3,
    /// Saved up to 10.4.0.1.
    pub rotation_axis: Vec3,
    pub age: f32,
    pub life_span: f32,
    pub last_update: f32,
    pub spawn_generation: u16,
    pub code: u16,
}

impl ParticleInfo {
    fn from_ffi(p: ffi::ParticleInfoFfi) -> Self {
        ParticleInfo {
            velocity: Vec3::from_ffi(p.velocity),
            rotation_axis: Vec3::from_ffi(p.rotation_axis),
            age: p.age,
            life_span: p.life_span,
            last_update: p.last_update,
            spawn_generation: p.spawn_generation,
            code: p.code,
        }
    }

    fn to_ffi(self) -> ffi::ParticleInfoFfi {
        ffi::ParticleInfoFfi {
            velocity: self.velocity.to_ffi(),
            rotation_axis: self.rotation_axis.to_ffi(),
            age: self.age,
            life_span: self.life_span,
            last_update: self.last_update,
            spawn_generation: self.spawn_generation,
            code: self.code,
        }
    }
}

impl NiParticlesData {
    fields! {
        /// The particles' radius. Saved up to 10.0.1.0.
        particle_radius, set_particle_radius: f32 = particles_data_particle_radius, set_particles_data_particle_radius, same, same;
        /// The number of particles alive when the file was saved.
        active_count, set_active_count: u16 = particles_data_active_count, set_particles_data_active_count, same, same;
        /// Whether the particles have [radii](Self::radii). Saved from 10.1.0.0 on.
        has_radii, set_has_radii: bool = particles_data_has_radii, set_particles_data_has_radii, same, same;
        /// Whether the particles have [sizes](Self::sizes).
        has_sizes, set_has_sizes: bool = particles_data_has_sizes, set_particles_data_has_sizes, same, same;
        /// Whether the particles have [rotations](Self::rotations). Saved from 10.0.1.0 on.
        has_rotations, set_has_rotations: bool = particles_data_has_rotations, set_particles_data_has_rotations, same, same;
        /// Whether the particles have [rotation angles](Self::rotation_angles). Saved from 20.0.0.4 on.
        has_rotation_angles, set_has_rotation_angles: bool = particles_data_has_rotation_angles, set_particles_data_has_rotation_angles, same, same;
        /// Whether the particles have [rotation axes](Self::rotation_axes). Saved from 20.0.0.4 on.
        has_rotation_axes, set_has_rotation_axes: bool = particles_data_has_rotation_axes, set_particles_data_has_rotation_axes, same, same;
        /// Whether the particles use [subtexture offsets](Self::subtexture_offsets). Saved only in 20.2.0.7.
        has_texture_indices, set_has_texture_indices: bool = particles_data_has_texture_indices, set_particles_data_has_texture_indices, same, same;
        /// The aspect ratio of the subtexture quads. Saved only in 20.2.0.7, from Skyrim on.
        aspect_ratio, set_aspect_ratio: f32 = particles_data_aspect_ratio, set_particles_data_aspect_ratio, same, same;
        /// Saved only in 20.2.0.7, from Skyrim on.
        aspect_flags, set_aspect_flags: AspectFlags = particles_data_aspect_flags, set_particles_data_aspect_flags, AspectFlags::from_bits_retain, |f: AspectFlags| f.bits();
        /// Saved only in 20.2.0.7, from Skyrim on.
        speed_to_aspect_aspect_2, set_speed_to_aspect_aspect_2: f32 = particles_data_speed_to_aspect_aspect_2, set_particles_data_speed_to_aspect_aspect_2, same, same;
        /// Saved only in 20.2.0.7, from Skyrim on.
        speed_to_aspect_speed_1, set_speed_to_aspect_speed_1: f32 = particles_data_speed_to_aspect_speed_1, set_particles_data_speed_to_aspect_speed_1, same, same;
        /// Saved only in 20.2.0.7, from Skyrim on.
        speed_to_aspect_speed_2, set_speed_to_aspect_speed_2: f32 = particles_data_speed_to_aspect_speed_2, set_particles_data_speed_to_aspect_speed_2, same, same;
    }
}

impl NiPSysData {
    fields! {
        /// Whether the particles have [rotation speeds](Self::rotation_speeds). Saved from 20.0.0.2 on.
        has_rotation_speeds, set_has_rotation_speeds: bool = psys_data_has_rotation_speeds, set_psys_data_has_rotation_speeds, same, same;
        /// Saved except in 20.2.0.7.
        added_particle_count, set_added_particle_count: u16 = psys_data_added_particle_count, set_psys_data_added_particle_count, same, same;
        /// Saved except in 20.2.0.7.
        added_particles_base, set_added_particles_base: u16 = psys_data_added_particles_base, set_psys_data_added_particles_base, same, same;
        /// A value of unknown meaning, saved from Fallout 76 on.
        unknown_vector, set_unknown_vector: Vec3 = psys_data_unknown_vector, set_psys_data_unknown_vector, vec3, vec3_to_ffi;
    }
}

impl NiMeshPSysData {
    fields! {
        /// The default size of each generation's pool of meshes.
        default_pool_size, set_default_pool_size: u32 = mesh_psys_data_default_pool_size, set_mesh_psys_data_default_pool_size, same, same;
        /// Whether to fill the pools on load.
        fill_pools_on_load, set_fill_pools_on_load: bool = mesh_psys_data_fill_pools_on_load, set_mesh_psys_data_fill_pools_on_load, same, same;
    }
}

impl BsStripPSysData {
    fields! {
        /// The most points a strip has.
        max_point_count, set_max_point_count: u16 = strip_psys_data_max_point_count, set_strip_psys_data_max_point_count, same, same;
        start_cap_size, set_start_cap_size: f32 = strip_psys_data_start_cap_size, set_strip_psys_data_start_cap_size, f32::from_bits, f32::to_bits;
        end_cap_size, set_end_cap_size: f32 = strip_psys_data_end_cap_size, set_strip_psys_data_end_cap_size, f32::from_bits, f32::to_bits;
        do_z_prepass, set_do_z_prepass: bool = strip_psys_data_do_z_prepass, set_strip_psys_data_do_z_prepass, same, same;
    }
}

impl NiParticleSystem {
    fields! {
        /// Whether new particles are in world space rather than the system's space.
        world_space, set_world_space: bool = particle_system_world_space, set_particle_system_world_space, same, same;
        /// Saved from Skyrim on.
        far_begin, set_far_begin: u16 = particle_system_far_begin, set_particle_system_far_begin, same, same;
        /// Saved from Skyrim on.
        far_end, set_far_end: u16 = particle_system_far_end, set_particle_system_far_end, same, same;
        /// Saved from Skyrim on.
        near_begin, set_near_begin: u16 = particle_system_near_begin, set_particle_system_near_begin, same, same;
        /// Saved from Skyrim on.
        near_end, set_near_end: u16 = particle_system_near_end, set_particle_system_near_end, same, same;
        /// The index of the active material. Saved from 20.2.0.5 on.
        active_material, set_active_material: i32 = particle_system_active_material, set_particle_system_active_material, same, same;
        /// Whether the default material needs updating. Saved from 20.2.0.7 on.
        default_material_update_flag, set_default_material_update_flag: u8 = particle_system_default_material_update_flag, set_particle_system_default_material_update_flag, same, same;
        /// The shader's implementation, saved only with a shader.
        shader_implementation, set_shader_implementation: u32 = particle_system_shader_implementation, set_particle_system_shader_implementation, same, same;
    }
}

impl NiPSysModifier {
    fields! {
        /// When the modifier runs relative to the system's others: lower first.
        order, set_order: u32 = psys_modifier_order, set_psys_modifier_order, same, same;
        /// Whether the modifier runs.
        active, set_active: bool = psys_modifier_active, set_psys_modifier_active, same, same;
    }
}

impl BsPSysStripUpdateModifier {
    fields! {
        /// The time between strip updates.
        update_delta_time, set_update_delta_time: f32 = strip_update_modifier_update_delta_time, set_strip_update_modifier_update_delta_time, same, same;
    }
}

impl NiPSysSpawnModifier {
    fields! {
        /// How many generations of particles can spawn.
        spawn_generations, set_spawn_generations: u16 = spawn_modifier_spawn_generations, set_spawn_modifier_spawn_generations, same, same;
        /// The fraction of particles that spawn, from 0 to 1.
        percent_spawned, set_percent_spawned: f32 = spawn_modifier_percent_spawned, set_spawn_modifier_percent_spawned, same, same;
        /// The fewest particles each one spawns.
        min_spawned, set_min_spawned: u16 = spawn_modifier_min_spawned, set_spawn_modifier_min_spawned, same, same;
        /// The most particles each one spawns.
        max_spawned, set_max_spawned: u16 = spawn_modifier_max_spawned, set_spawn_modifier_max_spawned, same, same;
        speed_variation, set_speed_variation: f32 = spawn_modifier_speed_variation, set_spawn_modifier_speed_variation, same, same;
        direction_variation, set_direction_variation: f32 = spawn_modifier_direction_variation, set_spawn_modifier_direction_variation, same, same;
        life_span, set_life_span: f32 = spawn_modifier_life_span, set_spawn_modifier_life_span, same, same;
        life_span_variation, set_life_span_variation: f32 = spawn_modifier_life_span_variation, set_spawn_modifier_life_span_variation, same, same;
    }
}

impl NiPSysAgeDeathModifier {
    fields! {
        /// Whether particles spawn new ones when they die, through the [spawn modifier](crate::handle::View::spawn_modifier).
        spawn_on_death, set_spawn_on_death: bool = age_death_modifier_spawn_on_death, set_age_death_modifier_spawn_on_death, same, same;
    }
}

impl BsPSysLodModifier {
    fields! {
        lod_begin_distance, set_lod_begin_distance: f32 = lod_modifier_lod_begin_distance, set_lod_modifier_lod_begin_distance, same, same;
        lod_end_distance, set_lod_end_distance: f32 = lod_modifier_lod_end_distance, set_lod_modifier_lod_end_distance, same, same;
        end_emit_scale, set_end_emit_scale: f32 = lod_modifier_end_emit_scale, set_lod_modifier_end_emit_scale, same, same;
        end_size, set_end_size: f32 = lod_modifier_end_size, set_lod_modifier_end_size, same, same;
    }
}

impl BsPSysSimpleColorModifier {
    fields! {
        /// The fraction of the life span spent fading in.
        fade_in_percent, set_fade_in_percent: f32 = simple_color_modifier_fade_in_percent, set_simple_color_modifier_fade_in_percent, same, same;
        /// The fraction of the life span spent fading out.
        fade_out_percent, set_fade_out_percent: f32 = simple_color_modifier_fade_out_percent, set_simple_color_modifier_fade_out_percent, same, same;
        color_1_end_percent, set_color_1_end_percent: f32 = simple_color_modifier_color_1_end_percent, set_simple_color_modifier_color_1_end_percent, same, same;
        color_2_start_percent, set_color_2_start_percent: f32 = simple_color_modifier_color_2_start_percent, set_simple_color_modifier_color_2_start_percent, same, same;
        color_2_end_percent, set_color_2_end_percent: f32 = simple_color_modifier_color_2_end_percent, set_simple_color_modifier_color_2_end_percent, same, same;
        color_3_start_percent, set_color_3_start_percent: f32 = simple_color_modifier_color_3_start_percent, set_simple_color_modifier_color_3_start_percent, same, same;
        /// RGBA.
        color_1, set_color_1: [f32; 4] = simple_color_modifier_color_1, set_simple_color_modifier_color_1, same, same;
        /// RGBA.
        color_2, set_color_2: [f32; 4] = simple_color_modifier_color_2, set_simple_color_modifier_color_2, same, same;
        /// RGBA.
        color_3, set_color_3: [f32; 4] = simple_color_modifier_color_3, set_simple_color_modifier_color_3, same, same;
    }
}

impl NiPSysRotationModifier {
    fields! {
        /// The rotation speed of new particles, in radians per second.
        initial_speed, set_initial_speed: f32 = rotation_modifier_initial_speed, set_rotation_modifier_initial_speed, same, same;
        initial_speed_variation, set_initial_speed_variation: f32 = rotation_modifier_initial_speed_variation, set_rotation_modifier_initial_speed_variation, same, same;
        /// A value of unknown meaning, saved from Fallout 76 on.
        unknown_vector, set_unknown_vector: [f32; 4] = rotation_modifier_unknown_vector, set_rotation_modifier_unknown_vector, same, same;
        /// A value of unknown meaning, saved from Fallout 76 on.
        unknown_byte, set_unknown_byte: u8 = rotation_modifier_unknown_byte, set_rotation_modifier_unknown_byte, same, same;
        /// The rotation angle of new particles, in radians.
        initial_angle, set_initial_angle: f32 = rotation_modifier_initial_angle, set_rotation_modifier_initial_angle, same, same;
        initial_angle_variation, set_initial_angle_variation: f32 = rotation_modifier_initial_angle_variation, set_rotation_modifier_initial_angle_variation, same, same;
        /// Whether to rotate new particles either way at random.
        random_speed_sign, set_random_speed_sign: bool = rotation_modifier_random_speed_sign, set_rotation_modifier_random_speed_sign, same, same;
        /// Whether to pick each new particle's axis at random instead of using [`initial_axis`](Self::initial_axis).
        random_initial_axis, set_random_initial_axis: bool = rotation_modifier_random_initial_axis, set_rotation_modifier_random_initial_axis, same, same;
        initial_axis, set_initial_axis: Vec3 = rotation_modifier_initial_axis, set_rotation_modifier_initial_axis, vec3, vec3_to_ffi;
    }
}

impl NiPSysGravityModifier {
    fields! {
        /// The direction of the force, in the [modifier object](crate::handle::View::modifier_object)'s space.
        gravity_axis, set_gravity_axis: Vec3 = gravity_modifier_gravity_axis, set_gravity_modifier_gravity_axis, vec3, vec3_to_ffi;
        /// How fast the force falls off with distance.
        decay, set_decay: f32 = gravity_modifier_decay, set_gravity_modifier_decay, same, same;
        strength, set_strength: f32 = gravity_modifier_strength, set_gravity_modifier_strength, same, same;
        turbulence, set_turbulence: f32 = gravity_modifier_turbulence, set_gravity_modifier_turbulence, same, same;
        turbulence_scale, set_turbulence_scale: f32 = gravity_modifier_turbulence_scale, set_gravity_modifier_turbulence_scale, same, same;
        /// Saved from Skyrim on (stream 17).
        world_aligned, set_world_aligned: bool = gravity_modifier_world_aligned, set_gravity_modifier_world_aligned, same, same;
    }

    enum_field! {
        /// The shape of the force, or `None` for a value it doesn't list.
        force_type, set_force_type: ForceType = gravity_modifier_force_type, set_gravity_modifier_force_type
    }
}

impl NiPSysBoundUpdateModifier {
    fields! {
        /// How many updates to skip between bound updates.
        update_skip, set_update_skip: u16 = bound_update_modifier_update_skip, set_bound_update_modifier_update_skip, same, same;
    }
}

impl NiPSysDragModifier {
    fields! {
        /// The direction of the force, in the [modifier object](crate::handle::View::modifier_object)'s space.
        drag_axis, set_drag_axis: Vec3 = drag_modifier_drag_axis, set_drag_modifier_drag_axis, vec3, vec3_to_ffi;
        /// How much to slow particles down.
        percentage, set_percentage: f32 = drag_modifier_percentage, set_drag_modifier_percentage, same, same;
        /// The distance up to which particles are fully affected.
        range, set_range: f32 = drag_modifier_range, set_drag_modifier_range, same, same;
        /// The distance at which particles are no longer affected.
        range_falloff, set_range_falloff: f32 = drag_modifier_range_falloff, set_drag_modifier_range_falloff, same, same;
    }
}

impl BsPSysInheritVelocityModifier {
    fields! {
        change_to_inherit, set_change_to_inherit: f32 = inherit_velocity_modifier_change_to_inherit, set_inherit_velocity_modifier_change_to_inherit, same, same;
        velocity_multiplier, set_velocity_multiplier: f32 = inherit_velocity_modifier_velocity_multiplier, set_inherit_velocity_modifier_velocity_multiplier, same, same;
        velocity_variation, set_velocity_variation: f32 = inherit_velocity_modifier_velocity_variation, set_inherit_velocity_modifier_velocity_variation, same, same;
    }
}

impl BsPSysSubTexModifier {
    fields! {
        start_frame, set_start_frame: f32 = sub_tex_modifier_start_frame, set_sub_tex_modifier_start_frame, same, same;
        start_frame_variation, set_start_frame_variation: f32 = sub_tex_modifier_start_frame_variation, set_sub_tex_modifier_start_frame_variation, same, same;
        end_frame, set_end_frame: f32 = sub_tex_modifier_end_frame, set_sub_tex_modifier_end_frame, same, same;
        loop_start_frame, set_loop_start_frame: f32 = sub_tex_modifier_loop_start_frame, set_sub_tex_modifier_loop_start_frame, same, same;
        loop_start_frame_variation, set_loop_start_frame_variation: f32 = sub_tex_modifier_loop_start_frame_variation, set_sub_tex_modifier_loop_start_frame_variation, same, same;
        frame_count, set_frame_count: f32 = sub_tex_modifier_frame_count, set_sub_tex_modifier_frame_count, same, same;
        frame_count_variation, set_frame_count_variation: f32 = sub_tex_modifier_frame_count_variation, set_sub_tex_modifier_frame_count_variation, same, same;
    }
}

impl NiPSysBombModifier {
    fields! {
        /// The direction of the force, in the [modifier object](crate::handle::View::modifier_object)'s space.
        bomb_axis, set_bomb_axis: Vec3 = bomb_modifier_bomb_axis, set_bomb_modifier_bomb_axis, vec3, vec3_to_ffi;
        /// How fast the force falls off with distance.
        decay, set_decay: f32 = bomb_modifier_decay, set_bomb_modifier_decay, same, same;
        /// The change in velocity.
        delta_v, set_delta_v: f32 = bomb_modifier_delta_v, set_bomb_modifier_delta_v, same, same;
    }

    enum_field! {
        /// How the force falls off, or `None` for a value it doesn't list.
        decay_type, set_decay_type: DecayType = bomb_modifier_decay_type, set_bomb_modifier_decay_type
    }

    enum_field! {
        /// The shape of the force, or `None` for a value it doesn't list.
        symmetry_type, set_symmetry_type: SymmetryType = bomb_modifier_symmetry_type, set_bomb_modifier_symmetry_type
    }
}

impl NiPSysGrowFadeModifier {
    fields! {
        /// How long new particles take to grow to full size.
        grow_time, set_grow_time: f32 = grow_fade_modifier_grow_time, set_grow_fade_modifier_grow_time, same, same;
        /// The spawn generation that grows.
        grow_generation, set_grow_generation: u16 = grow_fade_modifier_grow_generation, set_grow_fade_modifier_grow_generation, same, same;
        /// How long dying particles take to shrink away.
        fade_time, set_fade_time: f32 = grow_fade_modifier_fade_time, set_grow_fade_modifier_fade_time, same, same;
        /// The spawn generation that fades.
        fade_generation, set_fade_generation: u16 = grow_fade_modifier_fade_generation, set_grow_fade_modifier_fade_generation, same, same;
        /// Saved from Fallout 3 on (stream 34).
        base_scale, set_base_scale: f32 = grow_fade_modifier_base_scale, set_grow_fade_modifier_base_scale, same, same;
    }
}

impl NiPSysFieldModifier {
    fields! {
        /// The strength of the field.
        magnitude, set_magnitude: f32 = field_modifier_magnitude, set_field_modifier_magnitude, same, same;
        /// How fast the field falls off with distance.
        attenuation, set_attenuation: f32 = field_modifier_attenuation, set_field_modifier_attenuation, same, same;
        /// Whether the field ends at [`max_distance`](Self::max_distance).
        use_max_distance, set_use_max_distance: bool = field_modifier_use_max_distance, set_field_modifier_use_max_distance, same, same;
        max_distance, set_max_distance: f32 = field_modifier_max_distance, set_field_modifier_max_distance, same, same;
    }
}

impl NiPSysVortexFieldModifier {
    fields! {
        /// The vortex's axis.
        direction, set_direction: Vec3 = vortex_field_modifier_direction, set_vortex_field_modifier_direction, vec3, vec3_to_ffi;
    }
}

impl NiPSysGravityFieldModifier {
    fields! {
        /// The direction of the force.
        direction, set_direction: Vec3 = gravity_field_modifier_direction, set_gravity_field_modifier_direction, vec3, vec3_to_ffi;
    }
}

impl NiPSysDragFieldModifier {
    fields! {
        /// Whether drag applies only along [`direction`](Self::direction).
        use_direction, set_use_direction: bool = drag_field_modifier_use_direction, set_drag_field_modifier_use_direction, same, same;
        direction, set_direction: Vec3 = drag_field_modifier_direction, set_drag_field_modifier_direction, vec3, vec3_to_ffi;
    }
}

impl NiPSysTurbulenceFieldModifier {
    fields! {
        /// How often the turbulence changes.
        frequency, set_frequency: f32 = turbulence_field_modifier_frequency, set_turbulence_field_modifier_frequency, same, same;
    }
}

impl NiPSysAirFieldModifier {
    fields! {
        /// The direction of the air flow.
        direction, set_direction: Vec3 = air_field_modifier_direction, set_air_field_modifier_direction, vec3, vec3_to_ffi;
        air_friction, set_air_friction: f32 = air_field_modifier_air_friction, set_air_field_modifier_air_friction, same, same;
        inherit_velocity, set_inherit_velocity: f32 = air_field_modifier_inherit_velocity, set_air_field_modifier_inherit_velocity, same, same;
        inherit_rotation, set_inherit_rotation: bool = air_field_modifier_inherit_rotation, set_air_field_modifier_inherit_rotation, same, same;
        component_only, set_component_only: bool = air_field_modifier_component_only, set_air_field_modifier_component_only, same, same;
        enable_spread, set_enable_spread: bool = air_field_modifier_enable_spread, set_air_field_modifier_enable_spread, same, same;
        spread, set_spread: f32 = air_field_modifier_spread, set_air_field_modifier_spread, same, same;
    }
}

impl NiPSysRadialFieldModifier {
    fields! {
        /// How the force falls off with distance: 0 for not at all.
        radial_type, set_radial_type: f32 = radial_field_modifier_radial_type, set_radial_field_modifier_radial_type, f32::from_bits, f32::to_bits;
    }
}

impl BsWindModifier {
    fields! {
        strength, set_strength: f32 = wind_modifier_strength, set_wind_modifier_strength, same, same;
    }
}

impl BsPSysRecycleBoundModifier {
    fields! {
        bound_offset, set_bound_offset: Vec3 = recycle_bound_modifier_bound_offset, set_recycle_bound_modifier_bound_offset, vec3, vec3_to_ffi;
        bound_extent, set_bound_extent: Vec3 = recycle_bound_modifier_bound_extent, set_recycle_bound_modifier_bound_extent, vec3, vec3_to_ffi;
    }
}

impl BsParentVelocityModifier {
    fields! {
        damping, set_damping: f32 = parent_velocity_modifier_damping, set_parent_velocity_modifier_damping, same, same;
    }
}

impl NiPSysEmitter {
    fields! {
        /// The speed of new particles.
        speed, set_speed: f32 = psys_emitter_speed, set_psys_emitter_speed, same, same;
        speed_variation, set_speed_variation: f32 = psys_emitter_speed_variation, set_psys_emitter_speed_variation, same, same;
        /// The vertical direction of new particles, in radians: 0 is up, pi down.
        declination, set_declination: f32 = psys_emitter_declination, set_psys_emitter_declination, same, same;
        declination_variation, set_declination_variation: f32 = psys_emitter_declination_variation, set_psys_emitter_declination_variation, same, same;
        /// The horizontal direction of new particles, in radians.
        planar_angle, set_planar_angle: f32 = psys_emitter_planar_angle, set_psys_emitter_planar_angle, same, same;
        planar_angle_variation, set_planar_angle_variation: f32 = psys_emitter_planar_angle_variation, set_psys_emitter_planar_angle_variation, same, same;
        /// The color of new particles, RGBA.
        initial_color, set_initial_color: [f32; 4] = psys_emitter_initial_color, set_psys_emitter_initial_color, same, same;
        /// The radius of new particles.
        initial_radius, set_initial_radius: f32 = psys_emitter_initial_radius, set_psys_emitter_initial_radius, same, same;
        radius_variation, set_radius_variation: f32 = psys_emitter_radius_variation, set_psys_emitter_radius_variation, same, same;
        /// How long new particles live, in seconds.
        life_span, set_life_span: f32 = psys_emitter_life_span, set_psys_emitter_life_span, same, same;
        life_span_variation, set_life_span_variation: f32 = psys_emitter_life_span_variation, set_psys_emitter_life_span_variation, same, same;
    }
}

impl NiPSysSphereEmitter {
    fields! {
        /// The sphere's radius.
        radius, set_radius: f32 = sphere_emitter_radius, set_sphere_emitter_radius, same, same;
    }
}

impl NiPSysCylinderEmitter {
    fields! {
        /// The cylinder's radius.
        radius, set_radius: f32 = cylinder_emitter_radius, set_cylinder_emitter_radius, same, same;
        /// The cylinder's height.
        height, set_height: f32 = cylinder_emitter_height, set_cylinder_emitter_height, same, same;
    }
}

impl NiPSysBoxEmitter {
    fields! {
        /// The box's size along x.
        width, set_width: f32 = box_emitter_width, set_box_emitter_width, same, same;
        /// The box's size along y.
        height, set_height: f32 = box_emitter_height, set_box_emitter_height, same, same;
        /// The box's size along z.
        depth, set_depth: f32 = box_emitter_depth, set_box_emitter_depth, same, same;
    }
}

impl NiPSysMeshEmitter {
    fields! {
        /// The direction of new particles with [`VelocityType::UseDirection`].
        emission_axis, set_emission_axis: Vec3 = mesh_emitter_emission_axis, set_mesh_emitter_emission_axis, vec3, vec3_to_ffi;
    }

    enum_field! {
        /// How new particles' velocity is chosen, or `None` for a value it doesn't list.
        velocity_type, set_velocity_type: VelocityType = mesh_emitter_velocity_type, set_mesh_emitter_velocity_type
    }

    enum_field! {
        /// Where on the meshes particles start, or `None` for a value it doesn't list.
        emission_type, set_emission_type: EmitFrom = mesh_emitter_emission_type, set_mesh_emitter_emission_type
    }
}

impl NiPSysCollider {
    fields! {
        /// How much speed particles keep when they bounce off, from 0 to 1.
        bounce, set_bounce: f32 = psys_collider_bounce, set_psys_collider_bounce, same, same;
        /// Whether particles spawn new ones when they hit, through the [spawn modifier](crate::handle::View::spawn_modifier).
        spawn_on_collide, set_spawn_on_collide: bool = psys_collider_spawn_on_collide, set_psys_collider_spawn_on_collide, same, same;
        /// Whether particles die when they hit.
        die_on_collide, set_die_on_collide: bool = psys_collider_die_on_collide, set_psys_collider_die_on_collide, same, same;
    }
}

impl NiPSysSphericalCollider {
    fields! {
        /// The sphere's radius.
        radius, set_radius: f32 = spherical_collider_radius, set_spherical_collider_radius, same, same;
    }
}

impl NiPSysPlanarCollider {
    fields! {
        /// The plane's size along [`x_axis`](Self::x_axis).
        width, set_width: f32 = planar_collider_width, set_planar_collider_width, same, same;
        /// The plane's size along [`y_axis`](Self::y_axis).
        height, set_height: f32 = planar_collider_height, set_planar_collider_height, same, same;
        x_axis, set_x_axis: Vec3 = planar_collider_x_axis, set_planar_collider_x_axis, vec3, vec3_to_ffi;
        y_axis, set_y_axis: Vec3 = planar_collider_y_axis, set_planar_collider_y_axis, vec3, vec3_to_ffi;
    }
}

impl BsPSysMultiTargetEmitterCtlr {
    fields! {
        /// The most emitters the controller drives.
        max_emitters, set_max_emitters: u16 = multi_target_emitter_ctlr_max_emitters, set_multi_target_emitter_ctlr_max_emitters, same, same;
    }
}

impl NiGravity {
    fields! {
        /// How fast the force falls off with distance.
        decay, set_decay: f32 = gravity_decay, set_gravity_decay, same, same;
        /// The force's strength.
        force, set_force: f32 = gravity_force, set_gravity_force, same, same;
        /// The point particles are pulled toward, in the system's space.
        position, set_position: Vec3 = gravity_position, set_gravity_position, vec3, vec3_to_ffi;
        /// The direction particles are pulled in.
        direction, set_direction: Vec3 = gravity_direction, set_gravity_direction, vec3, vec3_to_ffi;
    }

    enum_field! {
        /// The kind of force, or `None` for a value it doesn't list.
        field_type, set_field_type: FieldType = gravity_field_type, set_gravity_field_type
    }
}

impl NiParticleGrowFade {
    fields! {
        /// The time from a particle's birth during which it grows.
        grow, set_grow: f32 = grow_fade_grow, set_grow_fade_grow, same, same;
        /// The time before a particle's death during which it shrinks.
        fade, set_fade: f32 = grow_fade_fade, set_grow_fade_fade, same, same;
    }
}

impl NiParticleRotation {
    fields! {
        /// Whether each particle starts with a random axis instead of [`initial_axis`](Self::initial_axis).
        random_initial_axis, set_random_initial_axis: bool = particle_rotation_random_initial_axis, set_particle_rotation_random_initial_axis, same, same;
        initial_axis, set_initial_axis: Vec3 = particle_rotation_initial_axis, set_particle_rotation_initial_axis, vec3, vec3_to_ffi;
        /// The speed of rotation, in radians per second.
        rotation_speed, set_rotation_speed: f32 = particle_rotation_rotation_speed, set_particle_rotation_rotation_speed, same, same;
    }
}

impl NiParticleBomb {
    fields! {
        /// How fast the force falls off with distance.
        decay, set_decay: f32 = particle_bomb_decay, set_particle_bomb_decay, same, same;
        /// How long the bomb pushes.
        duration, set_duration: f32 = particle_bomb_duration, set_particle_bomb_duration, same, same;
        /// The change in speed the bomb gives.
        delta_v, set_delta_v: f32 = particle_bomb_delta_v, set_particle_bomb_delta_v, same, same;
        /// When the bomb goes off.
        start, set_start: f32 = particle_bomb_start, set_particle_bomb_start, same, same;
        position, set_position: Vec3 = particle_bomb_position, set_particle_bomb_position, vec3, vec3_to_ffi;
        direction, set_direction: Vec3 = particle_bomb_direction, set_particle_bomb_direction, vec3, vec3_to_ffi;
    }

    enum_field! {
        /// How the force falls off, or `None` for a value it doesn't list.
        decay_type, set_decay_type: DecayType = particle_bomb_decay_type, set_particle_bomb_decay_type
    }

    enum_field! {
        /// The force's shape, or `None` for a value it doesn't list. Saved from 4.1.0.12 on.
        symmetry_type, set_symmetry_type: SymmetryType = particle_bomb_symmetry_type, set_particle_bomb_symmetry_type
    }
}

impl NiParticleCollider {
    fields! {
        /// How much speed particles keep when they bounce off, from 0 to 1.
        bounce, set_bounce: f32 = particle_collider_bounce, set_particle_collider_bounce, same, same;
        /// Whether particles spawn new ones when they hit. Saved from 4.2.0.2 on.
        spawn_on_collide, set_spawn_on_collide: bool = particle_collider_spawn_on_collide, set_particle_collider_spawn_on_collide, same, same;
        /// Whether particles die when they hit. Saved from 4.2.0.2 on.
        die_on_collide, set_die_on_collide: bool = particle_collider_die_on_collide, set_particle_collider_die_on_collide, same, same;
    }
}

impl NiPlanarCollider {
    fields! {
        /// The plane's size along [`x_axis`](Self::x_axis).
        width, set_width: f32 = old_planar_collider_width, set_old_planar_collider_width, same, same;
        /// The plane's size along [`y_axis`](Self::y_axis).
        height, set_height: f32 = old_planar_collider_height, set_old_planar_collider_height, same, same;
        /// The plane's center.
        position, set_position: Vec3 = old_planar_collider_position, set_old_planar_collider_position, vec3, vec3_to_ffi;
        x_axis, set_x_axis: Vec3 = old_planar_collider_x_axis, set_old_planar_collider_x_axis, vec3, vec3_to_ffi;
        y_axis, set_y_axis: Vec3 = old_planar_collider_y_axis, set_old_planar_collider_y_axis, vec3, vec3_to_ffi;
    }
}

impl NiSphericalCollider {
    fields! {
        /// The sphere's radius.
        radius, set_radius: f32 = old_spherical_collider_radius, set_old_spherical_collider_radius, same, same;
        /// The sphere's center.
        position, set_position: Vec3 = old_spherical_collider_position, set_old_spherical_collider_position, vec3, vec3_to_ffi;
    }
}

impl NiParticleSystemController {
    fields! {
        /// The speed of new particles.
        speed, set_speed: f32 = particle_system_ctlr_speed, set_particle_system_ctlr_speed, same, same;
        speed_variation, set_speed_variation: f32 = particle_system_ctlr_speed_variation, set_particle_system_ctlr_speed_variation, same, same;
        /// The angle of new particles' direction from the emitter's up axis, in radians.
        declination, set_declination: f32 = particle_system_ctlr_declination, set_particle_system_ctlr_declination, same, same;
        declination_variation, set_declination_variation: f32 = particle_system_ctlr_declination_variation, set_particle_system_ctlr_declination_variation, same, same;
        /// The angle of new particles' direction around the emitter's up axis, in radians.
        planar_angle, set_planar_angle: f32 = particle_system_ctlr_planar_angle, set_particle_system_ctlr_planar_angle, same, same;
        planar_angle_variation, set_planar_angle_variation: f32 = particle_system_ctlr_planar_angle_variation, set_particle_system_ctlr_planar_angle_variation, same, same;
        /// The normal of new particles.
        initial_normal, set_initial_normal: Vec3 = particle_system_ctlr_initial_normal, set_particle_system_ctlr_initial_normal, vec3, vec3_to_ffi;
        /// The color of new particles, RGBA.
        initial_color, set_initial_color: [f32; 4] = particle_system_ctlr_initial_color, set_particle_system_ctlr_initial_color, same, same;
        /// The size of new particles.
        initial_size, set_initial_size: f32 = particle_system_ctlr_initial_size, set_particle_system_ctlr_initial_size, same, same;
        /// When emitting starts.
        emit_start_time, set_emit_start_time: f32 = particle_system_ctlr_emit_start_time, set_particle_system_ctlr_emit_start_time, same, same;
        /// When emitting stops.
        emit_stop_time, set_emit_stop_time: f32 = particle_system_ctlr_emit_stop_time, set_particle_system_ctlr_emit_stop_time, same, same;
        /// Whether the system resets when the animation loops.
        reset_particle_system, set_reset_particle_system: bool = particle_system_ctlr_reset_particle_system, set_particle_system_ctlr_reset_particle_system, same, same;
        /// New particles per second, with [`use_birth_rate`](Self::use_birth_rate).
        birth_rate, set_birth_rate: f32 = particle_system_ctlr_birth_rate, set_particle_system_ctlr_birth_rate, same, same;
        /// How long particles live.
        lifetime, set_lifetime: f32 = particle_system_ctlr_lifetime, set_particle_system_ctlr_lifetime, same, same;
        lifetime_variation, set_lifetime_variation: f32 = particle_system_ctlr_lifetime_variation, set_particle_system_ctlr_lifetime_variation, same, same;
        /// Whether to emit at [`birth_rate`](Self::birth_rate) rather than to keep every particle alive.
        use_birth_rate, set_use_birth_rate: bool = particle_system_ctlr_use_birth_rate, set_particle_system_ctlr_use_birth_rate, same, same;
        /// Whether particles spawn new ones when they die.
        spawn_on_death, set_spawn_on_death: bool = particle_system_ctlr_spawn_on_death, set_particle_system_ctlr_spawn_on_death, same, same;
        /// The size of the box particles start in, around the [emitter](crate::handle::View::emitter).
        emitter_dimensions, set_emitter_dimensions: Vec3 = particle_system_ctlr_emitter_dimensions, set_particle_system_ctlr_emitter_dimensions, vec3, vec3_to_ffi;
        /// How many generations of particles can spawn.
        spawn_generations, set_spawn_generations: u16 = particle_system_ctlr_spawn_generations, set_particle_system_ctlr_spawn_generations, same, same;
        /// The fraction of particles that spawn, from 0 to 1.
        percent_spawned, set_percent_spawned: f32 = particle_system_ctlr_percent_spawned, set_particle_system_ctlr_percent_spawned, same, same;
        /// How many particles each one spawns.
        spawn_multiplier, set_spawn_multiplier: u16 = particle_system_ctlr_spawn_multiplier, set_particle_system_ctlr_spawn_multiplier, same, same;
        /// The speed variation of spawned particles.
        spawn_speed_chaos, set_spawn_speed_chaos: f32 = particle_system_ctlr_spawn_speed_chaos, set_particle_system_ctlr_spawn_speed_chaos, same, same;
        /// The direction variation of spawned particles.
        spawn_direction_chaos, set_spawn_direction_chaos: f32 = particle_system_ctlr_spawn_direction_chaos, set_particle_system_ctlr_spawn_direction_chaos, same, same;
        /// The number of [particles](Self::particles) alive when the file was saved.
        active_count, set_active_count: u16 = particle_system_ctlr_active_count, set_particle_system_ctlr_active_count, same, same;
        static_target_bound, set_static_target_bound: bool = particle_system_ctlr_static_target_bound, set_particle_system_ctlr_static_target_bound, same, same;
    }
}

impl NiPlanarCollider {
    /// The plane particles bounce off.
    pub fn plane(&self) -> Plane {
        Plane {
            normal: Vec3::from_ffi(ffi::old_planar_collider_plane_normal(&self.0)),
            constant: ffi::old_planar_collider_plane_constant(&self.0),
        }
    }

    /// Sets [`plane`](Self::plane).
    pub fn set_plane(mut self: Pin<&mut Self>, plane: Plane) {
        ffi::set_old_planar_collider_plane_normal(self.as_mut().ffi_mut(), plane.normal.to_ffi());
        ffi::set_old_planar_collider_plane_constant(self.ffi_mut(), plane.constant);
    }
}

/// Morrowind rotations, saved up to 4.2.2.0. Later files use
/// [`NiParticlesData::rotations`].
impl NiRotatingParticlesData {
    fields! {
        /// Whether the particles have [rotations](Self::rotations_2).
        has_rotations_2, set_has_rotations_2: bool = rotating_particles_data_has_rotations_2, set_rotating_particles_data_has_rotations_2, same, same;
    }

    /// The particles' rotations as quaternions, `[x, y, z, w]`.
    pub fn rotations_2(&self) -> Vec<[f32; 4]> {
        ffi::rotating_particles_data_rotations_2(&self.0).as_chunks::<4>().0.to_vec()
    }

    /// Replaces [`rotations_2`](Self::rotations_2). Returns `false`, changing nothing, unless
    /// there are none or one per vertex.
    pub fn set_rotations_2(self: Pin<&mut Self>, rotations: &[[f32; 4]]) -> bool {
        ffi::set_rotating_particles_data_rotations_2(self.ffi_mut(), rotations.as_flattened())
    }
}

impl NiParticleSystemController {
    /// The particles' records: the most particles alive at once, and each one's motion.
    pub fn particles(&self) -> Vec<ParticleInfo> {
        ffi::particle_system_ctlr_particles(&self.0).into_iter().map(ParticleInfo::from_ffi).collect()
    }

    /// Replaces [`particles`](Self::particles). Returns `false`, changing nothing, if there are
    /// more than 65535.
    pub fn set_particles(self: Pin<&mut Self>, particles: &[ParticleInfo]) -> bool {
        let particles: Vec<_> = particles.iter().copied().map(ParticleInfo::to_ffi).collect();
        ffi::set_particle_system_ctlr_particles(self.ffi_mut(), &particles)
    }
}

/// Per-particle lists. Each holds nothing or one value per vertex, and files from 20.2.0.7 save
/// only whether it's there.
impl NiParticlesData {
    /// Sets the [vertex count](NiGeometryData::vertex_count), the most particles the system
    /// holds, and resizes every per-vertex list that has values to match, padding with zeros.
    pub fn set_vertex_count(self: Pin<&mut Self>, count: u16) {
        ffi::set_particles_data_vertex_count(self.ffi_mut(), count);
    }

    /// The particles' radii.
    pub fn radii(&self) -> &[f32] {
        ffi::particles_data_radii(&self.0)
    }

    /// Replaces [`radii`](Self::radii). Returns `false`, changing nothing, unless there are none
    /// or one per vertex.
    pub fn set_radii(self: Pin<&mut Self>, radii: &[f32]) -> bool {
        ffi::set_particles_data_radii(self.ffi_mut(), radii)
    }

    /// The particles' sizes.
    pub fn sizes(&self) -> &[f32] {
        ffi::particles_data_sizes(&self.0)
    }

    /// Replaces [`sizes`](Self::sizes). Returns `false`, changing nothing, unless there are none
    /// or one per vertex.
    pub fn set_sizes(self: Pin<&mut Self>, sizes: &[f32]) -> bool {
        ffi::set_particles_data_sizes(self.ffi_mut(), sizes)
    }

    /// The particles' rotations as quaternions, `[x, y, z, w]`.
    pub fn rotations(&self) -> Vec<[f32; 4]> {
        ffi::particles_data_rotations(&self.0).as_chunks::<4>().0.to_vec()
    }

    /// Replaces [`rotations`](Self::rotations). Returns `false`, changing nothing, unless there
    /// are none or one per vertex.
    pub fn set_rotations(self: Pin<&mut Self>, rotations: &[[f32; 4]]) -> bool {
        ffi::set_particles_data_rotations(self.ffi_mut(), rotations.as_flattened())
    }

    /// The particles' rotation angles, in radians.
    pub fn rotation_angles(&self) -> &[f32] {
        ffi::particles_data_rotation_angles(&self.0)
    }

    /// Replaces [`rotation_angles`](Self::rotation_angles). Returns `false`, changing nothing,
    /// unless there are none or one per vertex.
    pub fn set_rotation_angles(self: Pin<&mut Self>, angles: &[f32]) -> bool {
        ffi::set_particles_data_rotation_angles(self.ffi_mut(), angles)
    }

    /// The particles' rotation axes.
    pub fn rotation_axes(&self) -> Vec<Vec3> {
        ffi::particles_data_rotation_axes(&self.0).into_iter().map(Vec3::from_ffi).collect()
    }

    /// Replaces [`rotation_axes`](Self::rotation_axes). Returns `false`, changing nothing, unless
    /// there are none or one per vertex.
    pub fn set_rotation_axes(self: Pin<&mut Self>, axes: &[Vec3]) -> bool {
        let axes: Vec<_> = axes.iter().map(|a| a.to_ffi()).collect();
        ffi::set_particles_data_rotation_axes(self.ffi_mut(), &axes)
    }

    /// The UV offsets of the subtextures a [`BsPSysSubTexModifier`] cycles through. Saved only in
    /// 20.2.0.7; before Skyrim, at most 255 are saved.
    pub fn subtexture_offsets(&self) -> Vec<[f32; 4]> {
        ffi::particles_data_subtexture_offsets(&self.0).as_chunks::<4>().0.to_vec()
    }

    /// Replaces [`subtexture_offsets`](Self::subtexture_offsets).
    pub fn set_subtexture_offsets(self: Pin<&mut Self>, offsets: &[[f32; 4]]) {
        ffi::set_particles_data_subtexture_offsets(self.ffi_mut(), offsets.as_flattened());
    }
}

impl NiPSysData {
    /// The particles' rotation speeds. Saved when
    /// [`has_rotation_speeds`](Self::has_rotation_speeds), except in 20.2.0.7.
    pub fn rotation_speeds(&self) -> &[f32] {
        ffi::psys_data_rotation_speeds(&self.0)
    }

    /// Replaces [`rotation_speeds`](Self::rotation_speeds). Returns `false`, changing nothing,
    /// unless there are none or one per vertex.
    pub fn set_rotation_speeds(self: Pin<&mut Self>, speeds: &[f32]) -> bool {
        ffi::set_psys_data_rotation_speeds(self.ffi_mut(), speeds)
    }

    /// Each particle's motion. Saved except in 20.2.0.7.
    pub fn particle_info(&self) -> Vec<ParticleInfo> {
        ffi::psys_data_particle_info(&self.0).into_iter().map(ParticleInfo::from_ffi).collect()
    }

    /// Replaces [`particle_info`](Self::particle_info). Returns `false`, changing nothing, unless
    /// there is none or one per vertex.
    pub fn set_particle_info(self: Pin<&mut Self>, info: &[ParticleInfo]) -> bool {
        let info: Vec<_> = info.iter().copied().map(ParticleInfo::to_ffi).collect();
        ffi::set_psys_data_particle_info(self.ffi_mut(), &info)
    }
}

/// The pool fields are saved from 10.2.0.0 on.
impl NiMeshPSysData {
    /// The pool size of each spawn generation.
    pub fn generation_pool_sizes(&self) -> Vec<u32> {
        ffi::mesh_psys_data_generation_pool_sizes(&self.0)
    }

    /// Replaces [`generation_pool_sizes`](Self::generation_pool_sizes).
    pub fn set_generation_pool_sizes(self: Pin<&mut Self>, sizes: &[u32]) {
        ffi::set_mesh_psys_data_generation_pool_sizes(self.ffi_mut(), sizes);
    }
}

impl NiPSysEmitterCtlrData {
    /// The birth rate keys.
    pub fn keys(&self) -> KeyGroup<f32> {
        group_from_ffi(ffi::emitter_ctlr_data_keys(&self.0))
    }

    /// Replaces [`keys`](Self::keys). Returns `false`, changing nothing, if the group has keys but
    /// no interpolation.
    pub fn set_keys(self: Pin<&mut Self>, keys: &KeyGroup<f32>) -> bool {
        ffi::set_emitter_ctlr_data_keys(self.ffi_mut(), &group_to_ffi(keys))
    }

    /// When the emitter is on. Only each key's time and value are saved.
    pub fn visibility_keys(&self) -> Vec<Key<bool>> {
        ffi::emitter_ctlr_data_visibility_keys(&self.0).iter().map(key_from_ffi).collect()
    }

    /// Replaces [`visibility_keys`](Self::visibility_keys).
    pub fn set_visibility_keys(self: Pin<&mut Self>, keys: &[Key<bool>]) {
        let keys: Vec<_> = keys.iter().map(key_to_ffi).collect();
        ffi::set_emitter_ctlr_data_visibility_keys(self.ffi_mut(), &keys);
    }
}

/// Bethesda fields, saved from Skyrim SE on: the vertex flags and bounds.
impl NiParticleSystem {
    /// The eight vertex flag bytes.
    pub fn vertex_raw_flags(&self) -> [u8; 8] {
        ffi::particle_system_vertex_raw_flags(&self.0)
    }

    /// Sets [`vertex_raw_flags`](Self::vertex_raw_flags).
    pub fn set_vertex_raw_flags(self: Pin<&mut Self>, flags: [u8; 8]) {
        ffi::set_particle_system_vertex_raw_flags(self.ffi_mut(), flags);
    }

    /// The bounding sphere.
    pub fn bounds(&self) -> BoundingSphere {
        BoundingSphere::from_ffi(ffi::particle_system_bounds(&self.0))
    }

    /// Sets [`bounds`](Self::bounds).
    pub fn set_bounds(self: Pin<&mut Self>, bounds: BoundingSphere) {
        ffi::set_particle_system_bounds(self.ffi_mut(), bounds.to_ffi());
    }

    /// The bounding box's minimum and maximum. Saved only from Fallout 76 on.
    pub fn bound_min_max(&self) -> [f32; 6] {
        ffi::particle_system_bound_min_max(&self.0)
    }

    /// Sets [`bound_min_max`](Self::bound_min_max).
    pub fn set_bound_min_max(self: Pin<&mut Self>, values: [f32; 6]) {
        ffi::set_particle_system_bound_min_max(self.ffi_mut(), values);
    }
}

/// Material and shader fields, saved before Skyrim SE: files from version 20.2.0.5 save the
/// materials, files from 10.0.1.0 to 20.1.0.3 the shader.
impl NiParticleSystem {
    /// The materials: each name with its extra data.
    pub fn materials(&self) -> Vec<(&BStr, u32)> {
        let extra = ffi::particle_system_material_extra_data(&self.0);
        (0..ffi::particle_system_material_count(&self.0))
            .map(|i| (ffi::particle_system_material_name(&self.0, i).as_bstr(), extra.get(i).copied().unwrap_or(0)))
            .collect()
    }

    /// Replaces [`materials`](Self::materials).
    pub fn set_materials<N: AsRef<[u8]>>(self: Pin<&mut Self>, materials: &[(N, u32)]) {
        let (bytes, lengths) = crate::extra_data::joined(materials.iter().map(|(name, _)| name));
        let extra: Vec<u32> = materials.iter().map(|&(_, e)| e).collect();
        ffi::set_particle_system_materials(self.ffi_mut(), &bytes, &lengths, &extra);
    }

    /// The shader's name, or `None` if there's no shader.
    pub fn shader_name(&self) -> Option<&BStr> {
        ffi::particle_system_has_shader(&self.0).then(|| ffi::particle_system_shader_name(&self.0).as_bstr())
    }

    /// Sets the shader's name, or removes the shader with `None`.
    pub fn set_shader_name(self: Pin<&mut Self>, name: Option<impl AsRef<[u8]>>) {
        match name {
            Some(name) => ffi::set_particle_system_shader(self.ffi_mut(), true, name.as_ref()),
            None => ffi::set_particle_system_shader(self.ffi_mut(), false, &[]),
        }
    }
}

impl NiPSysModifier {
    /// The modifier's name, which controllers use to find it.
    pub fn name(&self) -> &BStr {
        ffi::psys_modifier_name(&self.0).as_bstr()
    }

    /// Sets [`name`](Self::name).
    pub fn set_name(self: Pin<&mut Self>, name: impl AsRef<[u8]>) {
        ffi::set_psys_modifier_name(self.ffi_mut(), name.as_ref());
    }
}

impl BsPSysSimpleColorModifier {
    /// Values of unknown meaning, saved from Fallout 76 on.
    pub fn unknown_shorts(&self) -> [u16; 26] {
        ffi::simple_color_modifier_unknown_shorts(&self.0)
    }

    /// Sets [`unknown_shorts`](Self::unknown_shorts).
    pub fn set_unknown_shorts(self: Pin<&mut Self>, values: [u16; 26]) {
        ffi::set_simple_color_modifier_unknown_shorts(self.ffi_mut(), values);
    }
}

impl BsPSysScaleModifier {
    /// The scales over the particles' life.
    pub fn scales(&self) -> Vec<f32> {
        ffi::scale_modifier_scales(&self.0)
    }

    /// Replaces [`scales`](Self::scales).
    pub fn set_scales(self: Pin<&mut Self>, scales: &[f32]) {
        ffi::set_scale_modifier_scales(self.ffi_mut(), scales);
    }
}

/// The refs of a particle system. These are on the exact type: for a subtype, `upcast` first.
impl<'a> View<'a, NiParticleSystem> {
    /// The particle data, or `None` if it has none.
    pub fn data(&self) -> Option<View<'a, NiPSysData>> {
        self.follow(RefKind::ParticleData)
    }

    /// The skin instance, or `None` if it has none.
    pub fn skin_instance(&self) -> Option<View<'a, NiBoneContainer>> {
        self.follow(RefKind::ParticleSkinInstance)
    }

    /// The shader, or `None` if it has none. Saved from Skyrim on.
    pub fn shader(&self) -> Option<View<'a, BsShaderProperty>> {
        self.follow(RefKind::ParticleShader)
    }

    /// The alpha property, or `None` if it has none. Saved from Skyrim on.
    pub fn alpha_property(&self) -> Option<View<'a, NiAlphaProperty>> {
        self.follow(RefKind::ParticleAlpha)
    }

    /// The system's modifiers, in order. Skips empty refs.
    pub fn modifiers(&self) -> BlockIter<'a, NiPSysModifier, RefIndices> {
        self.follow_list(RefList::Modifiers)
    }
}

impl Edit<'_, NiParticleSystem> {
    /// Like [`View::data`], for editing.
    pub fn data(&mut self) -> Option<Edit<'_, NiPSysData>> {
        self.follow(RefKind::ParticleData)
    }

    /// Sets the [data](View::data), or clears it with `None`. Returns `false`, changing nothing,
    /// unless `data` is an [`NiPSysData`].
    pub fn set_data(&mut self, data: Option<Index>) -> bool {
        self.set_ref(RefKind::ParticleData, data)
    }

    /// Like [`View::skin_instance`], for editing.
    pub fn skin_instance(&mut self) -> Option<Edit<'_, NiBoneContainer>> {
        self.follow(RefKind::ParticleSkinInstance)
    }

    /// Sets the [skin instance](View::skin_instance), or clears it with `None`. Returns `false`,
    /// changing nothing, unless `skin` is an [`NiBoneContainer`].
    pub fn set_skin_instance(&mut self, skin: Option<Index>) -> bool {
        self.set_ref(RefKind::ParticleSkinInstance, skin)
    }

    /// Like [`View::shader`], for editing.
    pub fn shader(&mut self) -> Option<Edit<'_, BsShaderProperty>> {
        self.follow(RefKind::ParticleShader)
    }

    /// Sets the [shader](View::shader), or clears it with `None`. Returns `false`, changing
    /// nothing, unless `shader` is a [`BsShaderProperty`].
    pub fn set_shader(&mut self, shader: Option<Index>) -> bool {
        self.set_ref(RefKind::ParticleShader, shader)
    }

    /// Like [`View::alpha_property`], for editing.
    pub fn alpha_property(&mut self) -> Option<Edit<'_, NiAlphaProperty>> {
        self.follow(RefKind::ParticleAlpha)
    }

    /// Sets the [alpha property](View::alpha_property), or clears it with `None`. Returns
    /// `false`, changing nothing, unless `alpha` is an [`NiAlphaProperty`].
    pub fn set_alpha_property(&mut self, alpha: Option<Index>) -> bool {
        self.set_ref(RefKind::ParticleAlpha, alpha)
    }

    /// Replaces the [modifiers](View::modifiers). Returns `false`, changing nothing, unless each
    /// is an [`NiPSysModifier`].
    pub fn set_modifiers(&mut self, modifiers: &[Index]) -> bool {
        self.set_ref_list(RefList::Modifiers, modifiers)
    }

    /// Adds a new, active `M` named `name` to the end of the [modifiers](View::modifiers),
    /// targeting the system, and returns its index. Its [order](NiPSysModifier::order) is 0.
    ///
    /// ```
    /// # use nifly::{file::{NifFile, NifVersion}, object3d::Transform, particles::*};
    /// let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    /// let mut root = nif.root_node_mut().unwrap();
    /// let system = root.add_particle_system::<NiParticleSystem, _>("Sparks", Transform::default());
    /// let mut system = nif.edit::<NiParticleSystem>(system).unwrap();
    /// let emitter = system.add_modifier::<NiPSysBoxEmitter>("Emitter");
    /// system.add_modifier::<NiPSysPositionModifier>("Position");
    /// let emitter = nif.block_of_mut::<NiPSysEmitter>(emitter).unwrap();
    /// emitter.set_life_span(2.0);
    /// ```
    pub fn add_modifier<M: NewModifier>(&mut self, name: impl AsRef<[u8]>) -> Index {
        let system = self.index() as u32;
        index_from_ffi(M::add(self.file_mut().inner.pin_mut(), system, name.as_ref()))
            .expect("the block is an NiParticleSystem")
    }
}

impl<T: NiType> Edit<'_, T> {
    /// Adds a new `P` named `name`, with `transform` relative to this node, as the node's last
    /// child, and returns its index. The system gets new data of its type ([`NiPSysData`],
    /// [`NiAutoNormalParticlesData`], ...). An [`NiParticleSystem`] is in
    /// [world space](NiParticleSystem::world_space). Doesn't check that the file's game uses `P`.
    pub fn add_particle_system<P: NewParticleSystem, I>(&mut self, name: impl AsRef<[u8]>, transform: Transform) -> Index
    where
        T: IsA<NiNode, I>,
    {
        let parent = self.index() as u32;
        index_from_ffi(P::add(self.file_mut().inner.pin_mut(), parent, name.as_ref(), transform.to_ffi()))
            .expect("the block is a node")
    }
}

/// Modifier, collider and emitter controller refs.
impl<'a, T: NiType> View<'a, T> {
    /// The particle system the modifier belongs to, or `None` if it has none.
    pub fn particle_system<I>(&self) -> Option<View<'a, NiParticleSystem>>
    where
        T: IsA<NiPSysModifier, I>,
    {
        self.follow(RefKind::ParticleSystem)
    }

    /// The modifier that spawns new particles, or `None` if it has none.
    pub fn spawn_modifier(&self) -> Option<View<'a, NiPSysSpawnModifier>>
    where
        T: HasSpawnModifier,
    {
        self.follow(RefKind::SpawnModifier)
    }

    /// The object whose position and orientation the modifier or collider works around, or
    /// `None` if it has none: the emitter's node for volume emitters, the field's object for
    /// field modifiers.
    pub fn modifier_object(&self) -> Option<View<'a, NiAvObject>>
    where
        T: HasModifierObject,
    {
        self.follow(RefKind::ModifierObject)
    }

    /// The colors over the particles' life, or `None` if there are none.
    pub fn color_data(&self) -> Option<View<'a, NiColorData>>
    where
        T: HasColorData,
    {
        self.follow(RefKind::ColorData)
    }

    /// The meshes the modifier updates or emits from. Skips empty refs.
    pub fn particle_meshes(&self) -> BlockIter<'a, NiAvObject, RefIndices>
    where
        T: HasParticleMeshes,
    {
        self.follow_list(RefList::ParticleMeshes)
    }

    /// The colliders of a collider manager: the first, then each one's
    /// [next collider](View::next_collider). Stops at an empty ref or a collider already listed.
    pub fn colliders<I>(&self) -> BlockIter<'a, NiPSysCollider, RefIndices>
    where
        T: IsA<NiPSysColliderManager, I>,
    {
        self.follow_list(RefList::Colliders)
    }

    /// The collider after this one, or `None` if it's the last.
    pub fn next_collider<I>(&self) -> Option<View<'a, NiPSysCollider>>
    where
        T: IsA<NiPSysCollider, I>,
    {
        self.follow(RefKind::NextCollider)
    }

    /// The manager the collider belongs to, or `None` if it has none.
    pub fn collider_manager<I>(&self) -> Option<View<'a, NiPSysColliderManager>>
    where
        T: IsA<NiPSysCollider, I>,
    {
        self.follow(RefKind::ColliderManager)
    }

    /// The emitter controller's keys, or `None` if it has none. Saved up to 10.1.0.103; later
    /// controllers use the [interpolator](View::interpolator) and
    /// [visibility interpolator](View::visibility_interpolator).
    pub fn emitter_data<I>(&self) -> Option<View<'a, NiPSysEmitterCtlrData>>
    where
        T: IsA<NiPSysEmitterCtlr, I>,
    {
        self.follow(RefKind::EmitterCtlrData)
    }

    /// The interpolator that switches the emitter on and off, or `None` if it has none. Saved
    /// from 10.1.0.104 on.
    pub fn visibility_interpolator<I>(&self) -> Option<View<'a, NiInterpolator>>
    where
        T: IsA<NiPSysEmitterCtlr, I>,
    {
        self.follow(RefKind::VisibilityInterpolator)
    }
}

impl<T: NiType> Edit<'_, T> {
    /// Like [`View::particle_system`], for editing.
    pub fn particle_system<I>(&mut self) -> Option<Edit<'_, NiParticleSystem>>
    where
        T: IsA<NiPSysModifier, I>,
    {
        self.follow(RefKind::ParticleSystem)
    }

    /// Sets the [particle system](View::particle_system), or clears it with `None`. Returns
    /// `false`, changing nothing, unless `system` is an [`NiParticleSystem`].
    pub fn set_particle_system<I>(&mut self, system: Option<Index>) -> bool
    where
        T: IsA<NiPSysModifier, I>,
    {
        self.set_ref(RefKind::ParticleSystem, system)
    }

    /// Like [`View::spawn_modifier`], for editing.
    pub fn spawn_modifier(&mut self) -> Option<Edit<'_, NiPSysSpawnModifier>>
    where
        T: HasSpawnModifier,
    {
        self.follow(RefKind::SpawnModifier)
    }

    /// Sets the [spawn modifier](View::spawn_modifier), or clears it with `None`. Returns
    /// `false`, changing nothing, unless `modifier` is an [`NiPSysSpawnModifier`].
    pub fn set_spawn_modifier(&mut self, modifier: Option<Index>) -> bool
    where
        T: HasSpawnModifier,
    {
        self.set_ref(RefKind::SpawnModifier, modifier)
    }

    /// Like [`View::modifier_object`], for editing.
    pub fn modifier_object(&mut self) -> Option<Edit<'_, NiAvObject>>
    where
        T: HasModifierObject,
    {
        self.follow(RefKind::ModifierObject)
    }

    /// Sets the [modifier object](View::modifier_object), or clears it with `None`. Returns
    /// `false`, changing nothing, unless `object` is an [`NiNode`] for an [`NiPSysBombModifier`]
    /// or [`NiPSysVolumeEmitter`], or an [`NiAvObject`] for the others.
    pub fn set_modifier_object(&mut self, object: Option<Index>) -> bool
    where
        T: HasModifierObject,
    {
        self.set_ref(RefKind::ModifierObject, object)
    }

    /// Like [`View::color_data`], for editing.
    pub fn color_data(&mut self) -> Option<Edit<'_, NiColorData>>
    where
        T: HasColorData,
    {
        self.follow(RefKind::ColorData)
    }

    /// Sets the [color data](View::color_data), or clears it with `None`. Returns `false`,
    /// changing nothing, unless `data` is an [`NiColorData`].
    pub fn set_color_data(&mut self, data: Option<Index>) -> bool
    where
        T: HasColorData,
    {
        self.set_ref(RefKind::ColorData, data)
    }

    /// Replaces the [particle meshes](View::particle_meshes). Returns `false`, changing nothing,
    /// unless each is an [`NiAvObject`].
    pub fn set_particle_meshes(&mut self, meshes: &[Index]) -> bool
    where
        T: HasParticleMeshes,
    {
        self.set_ref_list(RefList::ParticleMeshes, meshes)
    }

    /// Replaces the [colliders](View::colliders), linking each to the next. Returns `false`,
    /// changing nothing, unless they're distinct [`NiPSysCollider`]s. Doesn't change their
    /// [manager](View::collider_manager).
    pub fn set_colliders<I>(&mut self, colliders: &[Index]) -> bool
    where
        T: IsA<NiPSysColliderManager, I>,
    {
        self.set_ref_list(RefList::Colliders, colliders)
    }

    /// Adds a new `C` to the end of the [colliders](View::colliders), managed by this manager,
    /// and returns its index. A ref in the chain that isn't a collider, or that repeats one, ends
    /// the chain and is replaced.
    pub fn add_collider<C: NewCollider, I>(&mut self) -> Index
    where
        T: IsA<NiPSysColliderManager, I>,
    {
        let manager = self.index() as u32;
        index_from_ffi(C::add(self.file_mut().inner.pin_mut(), manager)).expect("the block is an NiPSysColliderManager")
    }

    /// Like [`View::next_collider`], for editing.
    pub fn next_collider<I>(&mut self) -> Option<Edit<'_, NiPSysCollider>>
    where
        T: IsA<NiPSysCollider, I>,
    {
        self.follow(RefKind::NextCollider)
    }

    /// Sets the [next collider](View::next_collider), or ends the chain with `None`. Returns
    /// `false`, changing nothing, unless `next` is an [`NiPSysCollider`] whose chain doesn't lead
    /// back to this one.
    pub fn set_next_collider<I>(&mut self, next: Option<Index>) -> bool
    where
        T: IsA<NiPSysCollider, I>,
    {
        self.set_ref(RefKind::NextCollider, next)
    }

    /// Like [`View::collider_manager`], for editing.
    pub fn collider_manager<I>(&mut self) -> Option<Edit<'_, NiPSysColliderManager>>
    where
        T: IsA<NiPSysCollider, I>,
    {
        self.follow(RefKind::ColliderManager)
    }

    /// Sets the [collider manager](View::collider_manager), or clears it with `None`. Returns
    /// `false`, changing nothing, unless `manager` is an [`NiPSysColliderManager`].
    pub fn set_collider_manager<I>(&mut self, manager: Option<Index>) -> bool
    where
        T: IsA<NiPSysCollider, I>,
    {
        self.set_ref(RefKind::ColliderManager, manager)
    }

    /// Like [`View::emitter_data`], for editing.
    pub fn emitter_data<I>(&mut self) -> Option<Edit<'_, NiPSysEmitterCtlrData>>
    where
        T: IsA<NiPSysEmitterCtlr, I>,
    {
        self.follow(RefKind::EmitterCtlrData)
    }

    /// Sets the [emitter data](View::emitter_data), or clears it with `None`. Returns `false`,
    /// changing nothing, unless `data` is an [`NiPSysEmitterCtlrData`].
    pub fn set_emitter_data<I>(&mut self, data: Option<Index>) -> bool
    where
        T: IsA<NiPSysEmitterCtlr, I>,
    {
        self.set_ref(RefKind::EmitterCtlrData, data)
    }

    /// Like [`View::visibility_interpolator`], for editing.
    pub fn visibility_interpolator<I>(&mut self) -> Option<Edit<'_, NiInterpolator>>
    where
        T: IsA<NiPSysEmitterCtlr, I>,
    {
        self.follow(RefKind::VisibilityInterpolator)
    }

    /// Sets the [visibility interpolator](View::visibility_interpolator), or clears it with
    /// `None`. Returns `false`, changing nothing, unless `interpolator` is an
    /// [`NiInterpolator`].
    pub fn set_visibility_interpolator<I>(&mut self, interpolator: Option<Index>) -> bool
    where
        T: IsA<NiPSysEmitterCtlr, I>,
    {
        self.set_ref(RefKind::VisibilityInterpolator, interpolator)
    }
}

/// The refs of a [`BsPSysHavokUpdateModifier`].
impl<'a> View<'a, BsPSysHavokUpdateModifier> {
    /// The nodes whose Havok bodies move the particles. Skips empty refs.
    pub fn havok_nodes(&self) -> BlockIter<'a, NiNode, RefIndices> {
        self.follow_list(RefList::HavokUpdateNodes)
    }

    /// The modifier it works with, or `None` if it has none.
    pub fn havok_modifier(&self) -> Option<View<'a, NiPSysModifier>> {
        self.follow(RefKind::HavokUpdateModifier)
    }
}

impl Edit<'_, BsPSysHavokUpdateModifier> {
    /// Replaces the [Havok nodes](View::havok_nodes). Returns `false`, changing nothing, unless
    /// each is an [`NiNode`].
    pub fn set_havok_nodes(&mut self, nodes: &[Index]) -> bool {
        self.set_ref_list(RefList::HavokUpdateNodes, nodes)
    }

    /// Like [`View::havok_modifier`], for editing.
    pub fn havok_modifier(&mut self) -> Option<Edit<'_, NiPSysModifier>> {
        self.follow(RefKind::HavokUpdateModifier)
    }

    /// Sets the [Havok modifier](View::havok_modifier), or clears it with `None`. Returns
    /// `false`, changing nothing, unless `modifier` is an [`NiPSysModifier`].
    pub fn set_havok_modifier(&mut self, modifier: Option<Index>) -> bool {
        self.set_ref(RefKind::HavokUpdateModifier, modifier)
    }
}

/// The node of an [`NiMeshPSysData`].
impl<'a> View<'a, NiMeshPSysData> {
    /// The node holding the particle meshes, or `None` if it has none.
    pub fn mesh_node(&self) -> Option<View<'a, NiNode>> {
        self.follow(RefKind::MeshPSysNode)
    }
}

impl Edit<'_, NiMeshPSysData> {
    /// Like [`View::mesh_node`], for editing.
    pub fn mesh_node(&mut self) -> Option<Edit<'_, NiNode>> {
        self.follow(RefKind::MeshPSysNode)
    }

    /// Sets the [mesh node](View::mesh_node), or clears it with `None`. Returns `false`,
    /// changing nothing, unless `node` is an [`NiNode`].
    pub fn set_mesh_node(&mut self, node: Option<Index>) -> bool {
        self.set_ref(RefKind::MeshPSysNode, node)
    }
}

/// The master system of a [`BsPSysMultiTargetEmitterCtlr`].
impl<'a> View<'a, BsPSysMultiTargetEmitterCtlr> {
    /// The master particle system, or `None` if it has none.
    pub fn master_particle_system(&self) -> Option<View<'a, BsMasterParticleSystem>> {
        self.follow(RefKind::MasterParticleSystem)
    }
}

impl Edit<'_, BsPSysMultiTargetEmitterCtlr> {
    /// Like [`View::master_particle_system`], for editing.
    pub fn master_particle_system(&mut self) -> Option<Edit<'_, BsMasterParticleSystem>> {
        self.follow(RefKind::MasterParticleSystem)
    }

    /// Sets the [master particle system](View::master_particle_system), or clears it with
    /// `None`. Returns `false`, changing nothing, unless `system` is a
    /// [`BsMasterParticleSystem`].
    pub fn set_master_particle_system(&mut self, system: Option<Index>) -> bool {
        self.set_ref(RefKind::MasterParticleSystem, system)
    }
}

/// Old particle, modifier and controller refs.
impl<'a, T: NiType> View<'a, T> {
    /// The modifier after this one in its chain, or `None` if it's the last.
    pub fn next_modifier(&self) -> Option<View<'a, T::Next>>
    where
        T: ChainedModifier,
    {
        self.follow(RefKind::NextParticleModifier)
    }

    /// The controller the modifier belongs to, or `None` if it has none.
    pub fn particle_controller(&self) -> Option<View<'a, NiParticleSystemController>>
    where
        T: ChainedModifier,
    {
        self.follow(RefKind::ParticleController)
    }

    /// The particles' data, or `None` if they have none.
    pub fn particles_data<I>(&self) -> Option<View<'a, NiParticlesData>>
    where
        T: IsA<NiParticles, I>,
    {
        self.follow(RefKind::ParticlesData)
    }

    /// The object particles start around, or `None` if it has none.
    pub fn emitter<I>(&self) -> Option<View<'a, NiAvObject>>
    where
        T: IsA<NiParticleSystemController, I>,
    {
        self.follow(RefKind::ParticleEmitter)
    }

    /// The controller's particle modifiers: the first, then each one's
    /// [next modifier](View::next_modifier). Stops at an empty ref or a modifier already listed.
    pub fn particle_modifiers<I>(&self) -> BlockIter<'a, NiParticleModifier, RefIndices>
    where
        T: IsA<NiParticleSystemController, I>,
    {
        self.follow_list(RefList::ParticleModifiers)
    }

    /// The controller's colliders: the first, which is an [`NiParticleCollider`], then each one's
    /// [next modifier](View::next_modifier). Stops at an empty ref or a modifier already listed.
    pub fn particle_colliders<I>(&self) -> BlockIter<'a, NiParticleModifier, RefIndices>
    where
        T: IsA<NiParticleSystemController, I>,
    {
        self.follow_list(RefList::ParticleColliders)
    }

    /// The controller's emitter modifiers: the first, then each one's
    /// [next modifier](View::next_modifier). Stops at an empty ref or a modifier already listed.
    pub fn emitter_modifiers<I>(&self) -> BlockIter<'a, NiEmitterModifier, RefIndices>
    where
        T: IsA<NiParticleSystemController, I>,
    {
        self.follow_list(RefList::EmitterModifiers)
    }
}

impl<T: NiType> Edit<'_, T> {
    /// Like [`View::next_modifier`], for editing.
    pub fn next_modifier(&mut self) -> Option<Edit<'_, T::Next>>
    where
        T: ChainedModifier,
    {
        self.follow(RefKind::NextParticleModifier)
    }

    /// Sets the [next modifier](View::next_modifier), or ends the chain with `None`. Returns
    /// `false`, changing nothing, unless `next` is an [`NiParticleModifier`] (an
    /// [`NiEmitterModifier`] for one) whose chain doesn't lead back to this one.
    pub fn set_next_modifier(&mut self, next: Option<Index>) -> bool
    where
        T: ChainedModifier,
    {
        self.set_ref(RefKind::NextParticleModifier, next)
    }

    /// Like [`View::particle_controller`], for editing.
    pub fn particle_controller(&mut self) -> Option<Edit<'_, NiParticleSystemController>>
    where
        T: ChainedModifier,
    {
        self.follow(RefKind::ParticleController)
    }

    /// Sets the [particle controller](View::particle_controller), or clears it with `None`.
    /// Returns `false`, changing nothing, unless `controller` is an
    /// [`NiParticleSystemController`].
    pub fn set_particle_controller(&mut self, controller: Option<Index>) -> bool
    where
        T: ChainedModifier,
    {
        self.set_ref(RefKind::ParticleController, controller)
    }

    /// Like [`View::particles_data`], for editing.
    pub fn particles_data<I>(&mut self) -> Option<Edit<'_, NiParticlesData>>
    where
        T: IsA<NiParticles, I>,
    {
        self.follow(RefKind::ParticlesData)
    }

    /// Sets the [particles' data](View::particles_data), or clears it with `None`. Returns
    /// `false`, changing nothing, unless `data` is an [`NiParticlesData`]. Use the data type that
    /// matches the particles' type, such as [`NiAutoNormalParticlesData`] for
    /// [`NiAutoNormalParticles`].
    pub fn set_particles_data<I>(&mut self, data: Option<Index>) -> bool
    where
        T: IsA<NiParticles, I>,
    {
        self.set_ref(RefKind::ParticlesData, data)
    }

    /// Like [`View::emitter`], for editing.
    pub fn emitter<I>(&mut self) -> Option<Edit<'_, NiAvObject>>
    where
        T: IsA<NiParticleSystemController, I>,
    {
        self.follow(RefKind::ParticleEmitter)
    }

    /// Sets the [emitter](View::emitter), or clears it with `None`. Returns `false`, changing
    /// nothing, unless `emitter` is an [`NiAvObject`].
    pub fn set_emitter<I>(&mut self, emitter: Option<Index>) -> bool
    where
        T: IsA<NiParticleSystemController, I>,
    {
        self.set_ref(RefKind::ParticleEmitter, emitter)
    }

    /// Replaces the [particle modifiers](View::particle_modifiers), linking them in order.
    /// Returns `false`, changing nothing, unless they're distinct [`NiParticleModifier`]s.
    pub fn set_particle_modifiers<I>(&mut self, modifiers: &[Index]) -> bool
    where
        T: IsA<NiParticleSystemController, I>,
    {
        self.set_ref_list(RefList::ParticleModifiers, modifiers)
    }

    /// Replaces the [particle colliders](View::particle_colliders), linking them in order.
    /// Returns `false`, changing nothing, unless they're distinct [`NiParticleModifier`]s and the
    /// first is an [`NiParticleCollider`].
    pub fn set_particle_colliders<I>(&mut self, colliders: &[Index]) -> bool
    where
        T: IsA<NiParticleSystemController, I>,
    {
        self.set_ref_list(RefList::ParticleColliders, colliders)
    }

    /// Replaces the [emitter modifiers](View::emitter_modifiers), linking them in order. Returns
    /// `false`, changing nothing, unless they're distinct [`NiEmitterModifier`]s.
    pub fn set_emitter_modifiers<I>(&mut self, modifiers: &[Index]) -> bool
    where
        T: IsA<NiParticleSystemController, I>,
    {
        self.set_ref_list(RefList::EmitterModifiers, modifiers)
    }

    /// Adds a new `M` belonging to this controller to the end of a chain and returns its index:
    /// colliders to the [particle colliders](View::particle_colliders), an [`NiEmitterModifier`]
    /// to the [emitter modifiers](View::emitter_modifiers), and others to the
    /// [particle modifiers](View::particle_modifiers). A ref in the chain that isn't of the
    /// chain's type, or that repeats one, ends the chain and is replaced.
    ///
    /// ```
    /// # use nifly::{file::{NifFile, NifVersion}, object3d::Transform, particles::*, geometry::NiAutoNormalParticles};
    /// let mut nif = NifFile::create(NifVersion::MORROWIND);
    /// let mut root = nif.root_node_mut().unwrap();
    /// let particles = root.add_particle_system::<NiAutoNormalParticles, _>("Smoke", Transform::default());
    /// let mut particles = nif.edit::<NiAutoNormalParticles>(particles).unwrap();
    /// let controller = particles.add_controller::<NiParticleSystemController, _>();
    /// let mut controller = nif.edit::<NiParticleSystemController>(controller).unwrap();
    /// controller.set_emitter(Some(0));
    /// let gravity = controller.add_particle_modifier::<NiGravity, _>();
    /// controller.add_particle_modifier::<NiParticleGrowFade, _>();
    /// nif.block_of_mut::<NiGravity>(gravity).unwrap().set_force(-9.8);
    /// ```
    pub fn add_particle_modifier<M: NewParticleModifier, I>(&mut self) -> Index
    where
        T: IsA<NiParticleSystemController, I>,
    {
        let controller = self.index() as u32;
        index_from_ffi(M::add(self.file_mut().inner.pin_mut(), controller))
            .expect("the block is an NiParticleSystemController")
    }
}

/// The container node of an [`NiParticleMeshesData`].
impl<'a> View<'a, NiParticleMeshesData> {
    /// The node holding the particle meshes, or `None` if it has none.
    pub fn container_node(&self) -> Option<View<'a, NiNode>> {
        self.follow(RefKind::ContainerNode)
    }
}

impl Edit<'_, NiParticleMeshesData> {
    /// Like [`View::container_node`], for editing.
    pub fn container_node(&mut self) -> Option<Edit<'_, NiNode>> {
        self.follow(RefKind::ContainerNode)
    }

    /// Sets the [container node](View::container_node), or clears it with `None`. Returns
    /// `false`, changing nothing, unless `node` is an [`NiNode`].
    pub fn set_container_node(&mut self, node: Option<Index>) -> bool {
        self.set_ref(RefKind::ContainerNode, node)
    }
}

/// An [`NiPSysAgeDeathModifier`] or [`NiPSysCollider`]: a block with a spawn modifier.
///
/// This trait is sealed.
pub trait HasSpawnModifier: NiType {}

/// A particle modifier or collider that works around an object: gravity, drag, inherit
/// velocity, bomb, field and recycle bound modifiers, volume emitters and colliders.
///
/// This trait is sealed.
pub trait HasModifierObject: NiType {}

/// An [`NiPSysColorModifier`] or [`NiParticleColorModifier`]: a block with color data.
///
/// This trait is sealed.
pub trait HasColorData: NiType {}

/// An [`NiPSysMeshUpdateModifier`], [`NiPSysMeshEmitter`] or [`NiParticleMeshModifier`]: a block
/// with a list of meshes.
///
/// This trait is sealed.
pub trait HasParticleMeshes: NiType {}

macro_rules! impl_marker {
    ($tr:ident: $($t:ident)*) => {$(impl $tr for $t {})*};
}

impl_marker! { HasSpawnModifier:
    NiPSysAgeDeathModifier NiPSysCollider NiPSysSphericalCollider NiPSysPlanarCollider
}

impl_marker! { HasModifierObject:
    NiPSysGravityModifier NiPSysDragModifier BsPSysInheritVelocityModifier NiPSysBombModifier
    NiPSysFieldModifier NiPSysVortexFieldModifier NiPSysGravityFieldModifier NiPSysDragFieldModifier
    NiPSysTurbulenceFieldModifier NiPSysAirFieldModifier NiPSysRadialFieldModifier
    BsPSysRecycleBoundModifier NiPSysVolumeEmitter NiPSysSphereEmitter NiPSysCylinderEmitter
    NiPSysBoxEmitter BsPSysArrayEmitter NiPSysCollider NiPSysSphericalCollider NiPSysPlanarCollider
}

impl_marker! { HasColorData: NiPSysColorModifier NiParticleColorModifier }

impl_marker! { HasParticleMeshes: NiPSysMeshUpdateModifier NiPSysMeshEmitter NiParticleMeshModifier }

impl_marker! { HasInterpolator: NiPSysEmitterCtlr BsPSysMultiTargetEmitterCtlr }

/// An old particle modifier or an [`NiEmitterModifier`]: a block in a chain of an
/// [`NiParticleSystemController`].
///
/// This trait is sealed.
pub trait ChainedModifier: NiType {
    /// The type of the next block in the chain.
    type Next: NiType;
}

macro_rules! chained_modifier {
    ($next:ident: $($t:ident)*) => {$(impl ChainedModifier for $t { type Next = $next; })*};
}

chained_modifier! { NiParticleModifier:
    NiParticleModifier NiGravity NiParticleGrowFade NiParticleColorModifier NiParticleRotation
    NiParticleBomb NiParticleMeshModifier NiParticleCollider NiPlanarCollider NiSphericalCollider
}

chained_modifier! { NiEmitterModifier: NiEmitterModifier }

/// An old particle modifier [`Edit::add_particle_modifier`] can add.
pub trait NewParticleModifier: NiType + Sealed {
    #[doc(hidden)]
    fn add(file: Pin<&mut nifly_sys::nif_file::ffi::NifFile>, controller: u32) -> u32;
}

macro_rules! new_particle_modifier {
    ($($t:ident => $add:ident;)*) => {$(
        impl NewParticleModifier for $t {
            fn add(file: Pin<&mut nifly_sys::nif_file::ffi::NifFile>, controller: u32) -> u32 {
                ffi::$add(file, controller)
            }
        }
    )*};
}

new_particle_modifier! {
    NiGravity => add_ni_gravity;
    NiParticleGrowFade => add_ni_particle_grow_fade;
    NiParticleColorModifier => add_ni_particle_color_modifier;
    NiParticleRotation => add_ni_particle_rotation;
    NiParticleBomb => add_ni_particle_bomb;
    NiParticleMeshModifier => add_ni_particle_mesh_modifier;
    NiPlanarCollider => add_ni_planar_collider;
    NiSphericalCollider => add_ni_spherical_collider;
    NiEmitterModifier => add_ni_emitter_modifier;
}

/// A particle system [`Edit::add_particle_system`] can add.
pub trait NewParticleSystem: NiType + Sealed {
    #[doc(hidden)]
    fn add(file: Pin<&mut nifly_sys::nif_file::ffi::NifFile>, parent: u32, name: &[u8], transform: ffi::Transform) -> u32;
}

macro_rules! new_particle_system {
    ($($t:ident => $add:ident;)*) => {$(
        impl NewParticleSystem for $t {
            fn add(file: Pin<&mut nifly_sys::nif_file::ffi::NifFile>, parent: u32, name: &[u8], transform: ffi::Transform) -> u32 {
                ffi::$add(file, parent, name, transform)
            }
        }
    )*};
}

new_particle_system! {
    NiParticleSystem => add_ni_particle_system;
    NiMeshParticleSystem => add_ni_mesh_particle_system;
    BsStripParticleSystem => add_bs_strip_particle_system;
    NiParticles => add_ni_particles;
    NiAutoNormalParticles => add_ni_auto_normal_particles;
    NiRotatingParticles => add_ni_rotating_particles;
    NiParticleMeshes => add_ni_particle_meshes;
}

/// A modifier [`Edit::add_modifier`] can add.
pub trait NewModifier: NiType + Sealed {
    #[doc(hidden)]
    fn add(file: Pin<&mut nifly_sys::nif_file::ffi::NifFile>, system: u32, name: &[u8]) -> u32;
}

macro_rules! new_modifier {
    ($($t:ident => $add:ident;)*) => {$(
        impl NewModifier for $t {
            fn add(file: Pin<&mut nifly_sys::nif_file::ffi::NifFile>, system: u32, name: &[u8]) -> u32 {
                ffi::$add(file, system, name)
            }
        }
    )*};
}

/// A collider [`Edit::add_collider`] can add.
pub trait NewCollider: NiType + Sealed {
    #[doc(hidden)]
    fn add(file: Pin<&mut nifly_sys::nif_file::ffi::NifFile>, manager: u32) -> u32;
}

impl NewCollider for NiPSysSphericalCollider {
    fn add(file: Pin<&mut nifly_sys::nif_file::ffi::NifFile>, manager: u32) -> u32 {
        ffi::add_ni_psys_spherical_collider(file, manager)
    }
}

impl NewCollider for NiPSysPlanarCollider {
    fn add(file: Pin<&mut nifly_sys::nif_file::ffi::NifFile>, manager: u32) -> u32 {
        ffi::add_ni_psys_planar_collider(file, manager)
    }
}

macro_rules! new_controller {
    ($($t:ty => $add:ident;)*) => {$(
        impl NewController for $t {
            fn add(file: Pin<&mut nifly_sys::nif_file::ffi::NifFile>, owner: u32) -> u32 {
                ffi::$add(file, owner)
            }
        }
    )*};
}

impl NewAnimationData for NiPSysEmitterCtlrData {
    fn add(file: Pin<&mut nifly_sys::nif_file::ffi::NifFile>) -> u32 {
        ffi::add_ni_psys_emitter_ctlr_data(file)
    }
}

new_modifier! {
    BsPSysStripUpdateModifier => add_bs_psys_strip_update_modifier;
    NiPSysSpawnModifier => add_ni_psys_spawn_modifier;
    NiPSysAgeDeathModifier => add_ni_psys_age_death_modifier;
    BsPSysLodModifier => add_bs_psys_lod_modifier;
    BsPSysSimpleColorModifier => add_bs_psys_simple_color_modifier;
    NiPSysRotationModifier => add_ni_psys_rotation_modifier;
    BsPSysScaleModifier => add_bs_psys_scale_modifier;
    NiPSysGravityModifier => add_ni_psys_gravity_modifier;
    NiPSysPositionModifier => add_ni_psys_position_modifier;
    NiPSysBoundUpdateModifier => add_ni_psys_bound_update_modifier;
    NiPSysDragModifier => add_ni_psys_drag_modifier;
    BsPSysInheritVelocityModifier => add_bs_psys_inherit_velocity_modifier;
    BsPSysSubTexModifier => add_bs_psys_sub_tex_modifier;
    NiPSysBombModifier => add_ni_psys_bomb_modifier;
    NiPSysColorModifier => add_ni_psys_color_modifier;
    NiPSysGrowFadeModifier => add_ni_psys_grow_fade_modifier;
    NiPSysMeshUpdateModifier => add_ni_psys_mesh_update_modifier;
    NiPSysVortexFieldModifier => add_ni_psys_vortex_field_modifier;
    NiPSysGravityFieldModifier => add_ni_psys_gravity_field_modifier;
    NiPSysDragFieldModifier => add_ni_psys_drag_field_modifier;
    NiPSysTurbulenceFieldModifier => add_ni_psys_turbulence_field_modifier;
    NiPSysAirFieldModifier => add_ni_psys_air_field_modifier;
    NiPSysRadialFieldModifier => add_ni_psys_radial_field_modifier;
    BsWindModifier => add_bs_wind_modifier;
    BsPSysRecycleBoundModifier => add_bs_psys_recycle_bound_modifier;
    BsPSysHavokUpdateModifier => add_bs_psys_havok_update_modifier;
    BsParentVelocityModifier => add_bs_parent_velocity_modifier;
    NiPSysColliderManager => add_ni_psys_collider_manager;
    NiPSysSphereEmitter => add_ni_psys_sphere_emitter;
    NiPSysCylinderEmitter => add_ni_psys_cylinder_emitter;
    NiPSysBoxEmitter => add_ni_psys_box_emitter;
    BsPSysArrayEmitter => add_bs_psys_array_emitter;
    NiPSysMeshEmitter => add_ni_psys_mesh_emitter;
}

new_controller! {
    NiParticleSystemController => add_ni_particle_system_controller;
    NiBspArrayController => add_ni_bsp_array_controller;
    NiPSysEmitterCtlr => add_ni_psys_emitter_ctlr;
    BsPSysMultiTargetEmitterCtlr => add_bs_psys_multi_target_emitter_ctlr;
    crate::animation::NiPSysUpdateCtlr => add_ni_psys_update_ctlr;
    crate::animation::NiPSysResetOnLoopCtlr => add_ni_psys_reset_on_loop_ctlr;
    crate::animation::NiPSysModifierActiveCtlr => add_ni_psys_modifier_active_ctlr;
    crate::animation::NiPSysEmitterLifeSpanCtlr => add_ni_psys_emitter_life_span_ctlr;
    crate::animation::NiPSysEmitterSpeedCtlr => add_ni_psys_emitter_speed_ctlr;
    crate::animation::NiPSysEmitterInitialRadiusCtlr => add_ni_psys_emitter_initial_radius_ctlr;
    crate::animation::NiPSysEmitterDeclinationCtlr => add_ni_psys_emitter_declination_ctlr;
    crate::animation::NiPSysGravityStrengthCtlr => add_ni_psys_gravity_strength_ctlr;
    crate::animation::NiPSysEmitterDeclinationVarCtlr => add_ni_psys_emitter_declination_var_ctlr;
    crate::animation::NiPSysFieldMagnitudeCtlr => add_ni_psys_field_magnitude_ctlr;
    crate::animation::NiPSysFieldAttenuationCtlr => add_ni_psys_field_attenuation_ctlr;
    crate::animation::NiPSysFieldMaxDistanceCtlr => add_ni_psys_field_max_distance_ctlr;
    crate::animation::NiPSysAirFieldAirFrictionCtlr => add_ni_psys_air_field_air_friction_ctlr;
    crate::animation::NiPSysAirFieldInheritVelocityCtlr => add_ni_psys_air_field_inherit_velocity_ctlr;
    crate::animation::NiPSysAirFieldSpreadCtlr => add_ni_psys_air_field_spread_ctlr;
    crate::animation::NiPSysInitialRotSpeedCtlr => add_ni_psys_initial_rot_speed_ctlr;
    crate::animation::NiPSysInitialRotSpeedVarCtlr => add_ni_psys_initial_rot_speed_var_ctlr;
    crate::animation::NiPSysInitialRotAngleCtlr => add_ni_psys_initial_rot_angle_ctlr;
    crate::animation::NiPSysInitialRotAngleVarCtlr => add_ni_psys_initial_rot_angle_var_ctlr;
    crate::animation::NiPSysEmitterPlanarAngleCtlr => add_ni_psys_emitter_planar_angle_ctlr;
    crate::animation::NiPSysEmitterPlanarAngleVarCtlr => add_ni_psys_emitter_planar_angle_var_ctlr;
    crate::animation::NiPSysRotDampeningCtlr => add_ni_psys_rot_dampening_ctlr;
}

