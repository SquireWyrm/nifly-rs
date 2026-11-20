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
    nif_file_refs::ffi::{RefKind, RefList},
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

mod data;
mod system;
mod modifiers;
mod refs;
mod old;

pub use self::system::*;
pub use self::refs::*;
pub use self::old::*;
