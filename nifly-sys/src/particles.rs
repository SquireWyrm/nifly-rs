//! Particle systems: particle data, systems, modifiers, emitters, colliders and the old
//! particle blocks.
//!
//! For each type `T` with direct base `Parent`:
//! - `upcast_<t>(&T) -> &Parent`
//! - `downcast_<t>(&NiObject) -> *const T`. Null unless the object is a `T` or a subclass of `T`.

#[cxx::bridge]
pub mod ffi {
    /// An `NiParticleInfo`.
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct ParticleInfoFfi {
        velocity: Vec3,
        rotation_axis: Vec3,
        age: f32,
        life_span: f32,
        last_update: f32,
        spawn_generation: u16,
        code: u16,
    }

    #[namespace = "nifly"]
    unsafe extern "C++" {
        include!("particles_shim.h");
        include!("Particles.hpp");

        type NiParticlesData;
        type NiAutoNormalParticlesData;
        type NiRotatingParticlesData;
        type NiParticleMeshesData;
        type NiPSysData;
        type NiMeshPSysData;
        type BSStripPSysData;
        type NiPSysEmitterCtlrData;
        type NiParticleSystem;
        type NiMeshParticleSystem;
        type BSStripParticleSystem;
        type NiPSysModifier;
        type BSPSysStripUpdateModifier;
        type NiPSysSpawnModifier;
        type NiPSysAgeDeathModifier;
        type BSPSysLODModifier;
        type BSPSysSimpleColorModifier;
        type NiPSysRotationModifier;
        type BSPSysScaleModifier;
        type NiPSysGravityModifier;
        type NiPSysPositionModifier;
        type NiPSysBoundUpdateModifier;
        type NiPSysDragModifier;
        type BSPSysInheritVelocityModifier;
        type BSPSysSubTexModifier;
        type NiPSysBombModifier;
        type NiPSysColorModifier;
        type NiPSysGrowFadeModifier;
        type NiPSysMeshUpdateModifier;
        type NiPSysFieldModifier;
        type NiPSysVortexFieldModifier;
        type NiPSysGravityFieldModifier;
        type NiPSysDragFieldModifier;
        type NiPSysTurbulenceFieldModifier;
        type NiPSysAirFieldModifier;
        type NiPSysRadialFieldModifier;
        type BSWindModifier;
        type BSPSysRecycleBoundModifier;
        type BSPSysHavokUpdateModifier;
        type BSParentVelocityModifier;
        type NiPSysColliderManager;
        type NiPSysEmitter;
        type NiPSysVolumeEmitter;
        type NiPSysSphereEmitter;
        type NiPSysCylinderEmitter;
        type NiPSysBoxEmitter;
        type BSPSysArrayEmitter;
        type NiPSysMeshEmitter;
        type NiPSysCollider;
        type NiPSysSphericalCollider;
        type NiPSysPlanarCollider;
        type NiPSysEmitterCtlr;
        type BSPSysMultiTargetEmitterCtlr;
        type NiParticleModifier;
        type NiGravity;
        type NiParticleGrowFade;
        type NiParticleColorModifier;
        type NiParticleRotation;
        type NiParticleBomb;
        type NiParticleMeshModifier;
        type NiParticleCollider;
        type NiPlanarCollider;
        type NiSphericalCollider;
        type NiEmitterModifier;
        type NiParticleSystemController;
        type NiBSPArrayController;
    }

    #[namespace = "nifly"]
    unsafe extern "C++" {
        include!("NifFile.hpp");

        type NiObject = crate::objects::ffi::NiObject;
        type NiAVObject = crate::objects::ffi::NiAVObject;
        type NiGeometryData = crate::geometry::ffi::NiGeometryData;
        type NiTimeController = crate::animation::ffi::NiTimeController;
        type NiPSysModifierCtlr = crate::animation::ffi::NiPSysModifierCtlr;
        type NifFile = crate::nif_file::ffi::NifFile;
    }

    unsafe extern "C++" {
        include!("nifly-sys/src/object_3d.rs.h");

        type Vec3 = crate::object_3d::ffi::Vec3;
        type Transform = crate::object_3d::ffi::Transform;
        type BoundingSphere = crate::object_3d::ffi::BoundingSphere;
        type KeyFfi = crate::animation::ffi::KeyFfi;
        type KeyGroupFfi = crate::animation::ffi::KeyGroupFfi;
    }

    #[namespace = "nifly_rs"]
    unsafe extern "C++" {
        include!("particles_shim.h");

        fn upcast_ni_particles_data(obj: &NiParticlesData) -> &NiGeometryData;
        fn downcast_ni_particles_data(obj: &NiObject) -> *const NiParticlesData;
        fn upcast_ni_auto_normal_particles_data(
            obj: &NiAutoNormalParticlesData,
        ) -> &NiParticlesData;
        fn downcast_ni_auto_normal_particles_data(
            obj: &NiObject,
        ) -> *const NiAutoNormalParticlesData;
        fn upcast_ni_rotating_particles_data(obj: &NiRotatingParticlesData) -> &NiParticlesData;
        fn downcast_ni_rotating_particles_data(obj: &NiObject) -> *const NiRotatingParticlesData;
        fn upcast_ni_particle_meshes_data(obj: &NiParticleMeshesData) -> &NiRotatingParticlesData;
        fn downcast_ni_particle_meshes_data(obj: &NiObject) -> *const NiParticleMeshesData;
        fn upcast_ni_psys_data(obj: &NiPSysData) -> &NiRotatingParticlesData;
        fn downcast_ni_psys_data(obj: &NiObject) -> *const NiPSysData;
        fn upcast_ni_mesh_psys_data(obj: &NiMeshPSysData) -> &NiPSysData;
        fn downcast_ni_mesh_psys_data(obj: &NiObject) -> *const NiMeshPSysData;
        fn upcast_bs_strip_psys_data(obj: &BSStripPSysData) -> &NiPSysData;
        fn downcast_bs_strip_psys_data(obj: &NiObject) -> *const BSStripPSysData;
        fn upcast_ni_psys_emitter_ctlr_data(obj: &NiPSysEmitterCtlrData) -> &NiObject;
        fn downcast_ni_psys_emitter_ctlr_data(obj: &NiObject) -> *const NiPSysEmitterCtlrData;
        fn upcast_ni_particle_system(obj: &NiParticleSystem) -> &NiAVObject;
        fn downcast_ni_particle_system(obj: &NiObject) -> *const NiParticleSystem;
        fn upcast_ni_mesh_particle_system(obj: &NiMeshParticleSystem) -> &NiParticleSystem;
        fn downcast_ni_mesh_particle_system(obj: &NiObject) -> *const NiMeshParticleSystem;
        fn upcast_bs_strip_particle_system(obj: &BSStripParticleSystem) -> &NiParticleSystem;
        fn downcast_bs_strip_particle_system(obj: &NiObject) -> *const BSStripParticleSystem;
        fn upcast_ni_psys_modifier(obj: &NiPSysModifier) -> &NiObject;
        fn downcast_ni_psys_modifier(obj: &NiObject) -> *const NiPSysModifier;
        fn upcast_bs_psys_strip_update_modifier(obj: &BSPSysStripUpdateModifier)
        -> &NiPSysModifier;
        fn downcast_bs_psys_strip_update_modifier(
            obj: &NiObject,
        ) -> *const BSPSysStripUpdateModifier;
        fn upcast_ni_psys_spawn_modifier(obj: &NiPSysSpawnModifier) -> &NiPSysModifier;
        fn downcast_ni_psys_spawn_modifier(obj: &NiObject) -> *const NiPSysSpawnModifier;
        fn upcast_ni_psys_age_death_modifier(obj: &NiPSysAgeDeathModifier) -> &NiPSysModifier;
        fn downcast_ni_psys_age_death_modifier(obj: &NiObject) -> *const NiPSysAgeDeathModifier;
        fn upcast_bs_psys_lod_modifier(obj: &BSPSysLODModifier) -> &NiPSysModifier;
        fn downcast_bs_psys_lod_modifier(obj: &NiObject) -> *const BSPSysLODModifier;
        fn upcast_bs_psys_simple_color_modifier(obj: &BSPSysSimpleColorModifier)
        -> &NiPSysModifier;
        fn downcast_bs_psys_simple_color_modifier(
            obj: &NiObject,
        ) -> *const BSPSysSimpleColorModifier;
        fn upcast_ni_psys_rotation_modifier(obj: &NiPSysRotationModifier) -> &NiPSysModifier;
        fn downcast_ni_psys_rotation_modifier(obj: &NiObject) -> *const NiPSysRotationModifier;
        fn upcast_bs_psys_scale_modifier(obj: &BSPSysScaleModifier) -> &NiPSysModifier;
        fn downcast_bs_psys_scale_modifier(obj: &NiObject) -> *const BSPSysScaleModifier;
        fn upcast_ni_psys_gravity_modifier(obj: &NiPSysGravityModifier) -> &NiPSysModifier;
        fn downcast_ni_psys_gravity_modifier(obj: &NiObject) -> *const NiPSysGravityModifier;
        fn upcast_ni_psys_position_modifier(obj: &NiPSysPositionModifier) -> &NiPSysModifier;
        fn downcast_ni_psys_position_modifier(obj: &NiObject) -> *const NiPSysPositionModifier;
        fn upcast_ni_psys_bound_update_modifier(obj: &NiPSysBoundUpdateModifier)
        -> &NiPSysModifier;
        fn downcast_ni_psys_bound_update_modifier(
            obj: &NiObject,
        ) -> *const NiPSysBoundUpdateModifier;
        fn upcast_ni_psys_drag_modifier(obj: &NiPSysDragModifier) -> &NiPSysModifier;
        fn downcast_ni_psys_drag_modifier(obj: &NiObject) -> *const NiPSysDragModifier;
        fn upcast_bs_psys_inherit_velocity_modifier(
            obj: &BSPSysInheritVelocityModifier,
        ) -> &NiPSysModifier;
        fn downcast_bs_psys_inherit_velocity_modifier(
            obj: &NiObject,
        ) -> *const BSPSysInheritVelocityModifier;
        fn upcast_bs_psys_sub_tex_modifier(obj: &BSPSysSubTexModifier) -> &NiPSysModifier;
        fn downcast_bs_psys_sub_tex_modifier(obj: &NiObject) -> *const BSPSysSubTexModifier;
        fn upcast_ni_psys_bomb_modifier(obj: &NiPSysBombModifier) -> &NiPSysModifier;
        fn downcast_ni_psys_bomb_modifier(obj: &NiObject) -> *const NiPSysBombModifier;
        fn upcast_ni_psys_color_modifier(obj: &NiPSysColorModifier) -> &NiPSysModifier;
        fn downcast_ni_psys_color_modifier(obj: &NiObject) -> *const NiPSysColorModifier;
        fn upcast_ni_psys_grow_fade_modifier(obj: &NiPSysGrowFadeModifier) -> &NiPSysModifier;
        fn downcast_ni_psys_grow_fade_modifier(obj: &NiObject) -> *const NiPSysGrowFadeModifier;
        fn upcast_ni_psys_mesh_update_modifier(obj: &NiPSysMeshUpdateModifier) -> &NiPSysModifier;
        fn downcast_ni_psys_mesh_update_modifier(obj: &NiObject)
        -> *const NiPSysMeshUpdateModifier;
        fn upcast_ni_psys_field_modifier(obj: &NiPSysFieldModifier) -> &NiPSysModifier;
        fn downcast_ni_psys_field_modifier(obj: &NiObject) -> *const NiPSysFieldModifier;
        fn upcast_ni_psys_vortex_field_modifier(
            obj: &NiPSysVortexFieldModifier,
        ) -> &NiPSysFieldModifier;
        fn downcast_ni_psys_vortex_field_modifier(
            obj: &NiObject,
        ) -> *const NiPSysVortexFieldModifier;
        fn upcast_ni_psys_gravity_field_modifier(
            obj: &NiPSysGravityFieldModifier,
        ) -> &NiPSysFieldModifier;
        fn downcast_ni_psys_gravity_field_modifier(
            obj: &NiObject,
        ) -> *const NiPSysGravityFieldModifier;
        fn upcast_ni_psys_drag_field_modifier(
            obj: &NiPSysDragFieldModifier,
        ) -> &NiPSysFieldModifier;
        fn downcast_ni_psys_drag_field_modifier(obj: &NiObject) -> *const NiPSysDragFieldModifier;
        fn upcast_ni_psys_turbulence_field_modifier(
            obj: &NiPSysTurbulenceFieldModifier,
        ) -> &NiPSysFieldModifier;
        fn downcast_ni_psys_turbulence_field_modifier(
            obj: &NiObject,
        ) -> *const NiPSysTurbulenceFieldModifier;
        fn upcast_ni_psys_air_field_modifier(obj: &NiPSysAirFieldModifier) -> &NiPSysFieldModifier;
        fn downcast_ni_psys_air_field_modifier(obj: &NiObject) -> *const NiPSysAirFieldModifier;
        fn upcast_ni_psys_radial_field_modifier(
            obj: &NiPSysRadialFieldModifier,
        ) -> &NiPSysFieldModifier;
        fn downcast_ni_psys_radial_field_modifier(
            obj: &NiObject,
        ) -> *const NiPSysRadialFieldModifier;
        fn upcast_bs_wind_modifier(obj: &BSWindModifier) -> &NiPSysModifier;
        fn downcast_bs_wind_modifier(obj: &NiObject) -> *const BSWindModifier;
        fn upcast_bs_psys_recycle_bound_modifier(
            obj: &BSPSysRecycleBoundModifier,
        ) -> &NiPSysModifier;
        fn downcast_bs_psys_recycle_bound_modifier(
            obj: &NiObject,
        ) -> *const BSPSysRecycleBoundModifier;
        fn upcast_bs_psys_havok_update_modifier(obj: &BSPSysHavokUpdateModifier)
        -> &NiPSysModifier;
        fn downcast_bs_psys_havok_update_modifier(
            obj: &NiObject,
        ) -> *const BSPSysHavokUpdateModifier;
        fn upcast_bs_parent_velocity_modifier(obj: &BSParentVelocityModifier) -> &NiPSysModifier;
        fn downcast_bs_parent_velocity_modifier(obj: &NiObject) -> *const BSParentVelocityModifier;
        fn upcast_ni_psys_collider_manager(obj: &NiPSysColliderManager) -> &NiPSysModifier;
        fn downcast_ni_psys_collider_manager(obj: &NiObject) -> *const NiPSysColliderManager;
        fn upcast_ni_psys_emitter(obj: &NiPSysEmitter) -> &NiPSysModifier;
        fn downcast_ni_psys_emitter(obj: &NiObject) -> *const NiPSysEmitter;
        fn upcast_ni_psys_volume_emitter(obj: &NiPSysVolumeEmitter) -> &NiPSysEmitter;
        fn downcast_ni_psys_volume_emitter(obj: &NiObject) -> *const NiPSysVolumeEmitter;
        fn upcast_ni_psys_sphere_emitter(obj: &NiPSysSphereEmitter) -> &NiPSysVolumeEmitter;
        fn downcast_ni_psys_sphere_emitter(obj: &NiObject) -> *const NiPSysSphereEmitter;
        fn upcast_ni_psys_cylinder_emitter(obj: &NiPSysCylinderEmitter) -> &NiPSysVolumeEmitter;
        fn downcast_ni_psys_cylinder_emitter(obj: &NiObject) -> *const NiPSysCylinderEmitter;
        fn upcast_ni_psys_box_emitter(obj: &NiPSysBoxEmitter) -> &NiPSysVolumeEmitter;
        fn downcast_ni_psys_box_emitter(obj: &NiObject) -> *const NiPSysBoxEmitter;
        fn upcast_bs_psys_array_emitter(obj: &BSPSysArrayEmitter) -> &NiPSysVolumeEmitter;
        fn downcast_bs_psys_array_emitter(obj: &NiObject) -> *const BSPSysArrayEmitter;
        fn upcast_ni_psys_mesh_emitter(obj: &NiPSysMeshEmitter) -> &NiPSysEmitter;
        fn downcast_ni_psys_mesh_emitter(obj: &NiObject) -> *const NiPSysMeshEmitter;
        fn upcast_ni_psys_collider(obj: &NiPSysCollider) -> &NiObject;
        fn downcast_ni_psys_collider(obj: &NiObject) -> *const NiPSysCollider;
        fn upcast_ni_psys_spherical_collider(obj: &NiPSysSphericalCollider) -> &NiPSysCollider;
        fn downcast_ni_psys_spherical_collider(obj: &NiObject) -> *const NiPSysSphericalCollider;
        fn upcast_ni_psys_planar_collider(obj: &NiPSysPlanarCollider) -> &NiPSysCollider;
        fn downcast_ni_psys_planar_collider(obj: &NiObject) -> *const NiPSysPlanarCollider;
        fn upcast_ni_psys_emitter_ctlr(obj: &NiPSysEmitterCtlr) -> &NiPSysModifierCtlr;
        fn downcast_ni_psys_emitter_ctlr(obj: &NiObject) -> *const NiPSysEmitterCtlr;
        fn upcast_bs_psys_multi_target_emitter_ctlr(
            obj: &BSPSysMultiTargetEmitterCtlr,
        ) -> &NiPSysEmitterCtlr;
        fn downcast_bs_psys_multi_target_emitter_ctlr(
            obj: &NiObject,
        ) -> *const BSPSysMultiTargetEmitterCtlr;
        fn upcast_ni_particle_modifier(obj: &NiParticleModifier) -> &NiObject;
        fn downcast_ni_particle_modifier(obj: &NiObject) -> *const NiParticleModifier;
        fn upcast_ni_gravity(obj: &NiGravity) -> &NiParticleModifier;
        fn downcast_ni_gravity(obj: &NiObject) -> *const NiGravity;
        fn upcast_ni_particle_grow_fade(obj: &NiParticleGrowFade) -> &NiParticleModifier;
        fn downcast_ni_particle_grow_fade(obj: &NiObject) -> *const NiParticleGrowFade;
        fn upcast_ni_particle_color_modifier(obj: &NiParticleColorModifier) -> &NiParticleModifier;
        fn downcast_ni_particle_color_modifier(obj: &NiObject) -> *const NiParticleColorModifier;
        fn upcast_ni_particle_rotation(obj: &NiParticleRotation) -> &NiParticleModifier;
        fn downcast_ni_particle_rotation(obj: &NiObject) -> *const NiParticleRotation;
        fn upcast_ni_particle_bomb(obj: &NiParticleBomb) -> &NiParticleModifier;
        fn downcast_ni_particle_bomb(obj: &NiObject) -> *const NiParticleBomb;
        fn upcast_ni_particle_mesh_modifier(obj: &NiParticleMeshModifier) -> &NiParticleModifier;
        fn downcast_ni_particle_mesh_modifier(obj: &NiObject) -> *const NiParticleMeshModifier;
        fn upcast_ni_particle_collider(obj: &NiParticleCollider) -> &NiParticleModifier;
        fn downcast_ni_particle_collider(obj: &NiObject) -> *const NiParticleCollider;
        fn upcast_ni_planar_collider(obj: &NiPlanarCollider) -> &NiParticleCollider;
        fn downcast_ni_planar_collider(obj: &NiObject) -> *const NiPlanarCollider;
        fn upcast_ni_spherical_collider(obj: &NiSphericalCollider) -> &NiParticleCollider;
        fn downcast_ni_spherical_collider(obj: &NiObject) -> *const NiSphericalCollider;
        fn upcast_ni_emitter_modifier(obj: &NiEmitterModifier) -> &NiObject;
        fn downcast_ni_emitter_modifier(obj: &NiObject) -> *const NiEmitterModifier;
        fn upcast_ni_particle_system_controller(
            obj: &NiParticleSystemController,
        ) -> &NiTimeController;
        fn downcast_ni_particle_system_controller(
            obj: &NiObject,
        ) -> *const NiParticleSystemController;
        fn upcast_ni_bsp_array_controller(
            obj: &NiBSPArrayController,
        ) -> &NiParticleSystemController;
        fn downcast_ni_bsp_array_controller(obj: &NiObject) -> *const NiBSPArrayController;
        // Plain fields.
        fn particles_data_particle_radius(obj: &NiParticlesData) -> f32;
        fn set_particles_data_particle_radius(obj: Pin<&mut NiParticlesData>, value: f32);
        fn particles_data_active_count(obj: &NiParticlesData) -> u16;
        fn set_particles_data_active_count(obj: Pin<&mut NiParticlesData>, value: u16);
        fn particles_data_has_radii(obj: &NiParticlesData) -> bool;
        fn set_particles_data_has_radii(obj: Pin<&mut NiParticlesData>, value: bool);
        fn particles_data_has_sizes(obj: &NiParticlesData) -> bool;
        fn set_particles_data_has_sizes(obj: Pin<&mut NiParticlesData>, value: bool);
        fn particles_data_has_rotations(obj: &NiParticlesData) -> bool;
        fn set_particles_data_has_rotations(obj: Pin<&mut NiParticlesData>, value: bool);
        fn particles_data_has_rotation_angles(obj: &NiParticlesData) -> bool;
        fn set_particles_data_has_rotation_angles(obj: Pin<&mut NiParticlesData>, value: bool);
        fn particles_data_has_rotation_axes(obj: &NiParticlesData) -> bool;
        fn set_particles_data_has_rotation_axes(obj: Pin<&mut NiParticlesData>, value: bool);
        fn particles_data_has_texture_indices(obj: &NiParticlesData) -> bool;
        fn set_particles_data_has_texture_indices(obj: Pin<&mut NiParticlesData>, value: bool);
        fn particles_data_aspect_ratio(obj: &NiParticlesData) -> f32;
        fn set_particles_data_aspect_ratio(obj: Pin<&mut NiParticlesData>, value: f32);
        fn particles_data_aspect_flags(obj: &NiParticlesData) -> u16;
        fn set_particles_data_aspect_flags(obj: Pin<&mut NiParticlesData>, value: u16);
        fn particles_data_speed_to_aspect_aspect_2(obj: &NiParticlesData) -> f32;
        fn set_particles_data_speed_to_aspect_aspect_2(obj: Pin<&mut NiParticlesData>, value: f32);
        fn particles_data_speed_to_aspect_speed_1(obj: &NiParticlesData) -> f32;
        fn set_particles_data_speed_to_aspect_speed_1(obj: Pin<&mut NiParticlesData>, value: f32);
        fn particles_data_speed_to_aspect_speed_2(obj: &NiParticlesData) -> f32;
        fn set_particles_data_speed_to_aspect_speed_2(obj: Pin<&mut NiParticlesData>, value: f32);
        fn psys_data_has_rotation_speeds(obj: &NiPSysData) -> bool;
        fn set_psys_data_has_rotation_speeds(obj: Pin<&mut NiPSysData>, value: bool);
        fn psys_data_added_particle_count(obj: &NiPSysData) -> u16;
        fn set_psys_data_added_particle_count(obj: Pin<&mut NiPSysData>, value: u16);
        fn psys_data_added_particles_base(obj: &NiPSysData) -> u16;
        fn set_psys_data_added_particles_base(obj: Pin<&mut NiPSysData>, value: u16);
        fn psys_data_unknown_vector(obj: &NiPSysData) -> Vec3;
        fn set_psys_data_unknown_vector(obj: Pin<&mut NiPSysData>, value: Vec3);
        fn mesh_psys_data_default_pool_size(obj: &NiMeshPSysData) -> u32;
        fn set_mesh_psys_data_default_pool_size(obj: Pin<&mut NiMeshPSysData>, value: u32);
        fn mesh_psys_data_fill_pools_on_load(obj: &NiMeshPSysData) -> bool;
        fn set_mesh_psys_data_fill_pools_on_load(obj: Pin<&mut NiMeshPSysData>, value: bool);
        fn strip_psys_data_max_point_count(obj: &BSStripPSysData) -> u16;
        fn set_strip_psys_data_max_point_count(obj: Pin<&mut BSStripPSysData>, value: u16);
        fn strip_psys_data_start_cap_size(obj: &BSStripPSysData) -> u32;
        fn set_strip_psys_data_start_cap_size(obj: Pin<&mut BSStripPSysData>, value: u32);
        fn strip_psys_data_end_cap_size(obj: &BSStripPSysData) -> u32;
        fn set_strip_psys_data_end_cap_size(obj: Pin<&mut BSStripPSysData>, value: u32);
        fn strip_psys_data_do_z_prepass(obj: &BSStripPSysData) -> bool;
        fn set_strip_psys_data_do_z_prepass(obj: Pin<&mut BSStripPSysData>, value: bool);
        fn particle_system_world_space(obj: &NiParticleSystem) -> bool;
        fn set_particle_system_world_space(obj: Pin<&mut NiParticleSystem>, value: bool);
        fn particle_system_far_begin(obj: &NiParticleSystem) -> u16;
        fn set_particle_system_far_begin(obj: Pin<&mut NiParticleSystem>, value: u16);
        fn particle_system_far_end(obj: &NiParticleSystem) -> u16;
        fn set_particle_system_far_end(obj: Pin<&mut NiParticleSystem>, value: u16);
        fn particle_system_near_begin(obj: &NiParticleSystem) -> u16;
        fn set_particle_system_near_begin(obj: Pin<&mut NiParticleSystem>, value: u16);
        fn particle_system_near_end(obj: &NiParticleSystem) -> u16;
        fn set_particle_system_near_end(obj: Pin<&mut NiParticleSystem>, value: u16);
        fn particle_system_active_material(obj: &NiParticleSystem) -> i32;
        fn set_particle_system_active_material(obj: Pin<&mut NiParticleSystem>, value: i32);
        fn particle_system_default_material_update_flag(obj: &NiParticleSystem) -> u8;
        fn set_particle_system_default_material_update_flag(
            obj: Pin<&mut NiParticleSystem>,
            value: u8,
        );
        fn particle_system_shader_implementation(obj: &NiParticleSystem) -> u32;
        fn set_particle_system_shader_implementation(obj: Pin<&mut NiParticleSystem>, value: u32);
        fn psys_modifier_order(obj: &NiPSysModifier) -> u32;
        fn set_psys_modifier_order(obj: Pin<&mut NiPSysModifier>, value: u32);
        fn psys_modifier_active(obj: &NiPSysModifier) -> bool;
        fn set_psys_modifier_active(obj: Pin<&mut NiPSysModifier>, value: bool);
        fn strip_update_modifier_update_delta_time(obj: &BSPSysStripUpdateModifier) -> f32;
        fn set_strip_update_modifier_update_delta_time(
            obj: Pin<&mut BSPSysStripUpdateModifier>,
            value: f32,
        );
        fn spawn_modifier_spawn_generations(obj: &NiPSysSpawnModifier) -> u16;
        fn set_spawn_modifier_spawn_generations(obj: Pin<&mut NiPSysSpawnModifier>, value: u16);
        fn spawn_modifier_percent_spawned(obj: &NiPSysSpawnModifier) -> f32;
        fn set_spawn_modifier_percent_spawned(obj: Pin<&mut NiPSysSpawnModifier>, value: f32);
        fn spawn_modifier_min_spawned(obj: &NiPSysSpawnModifier) -> u16;
        fn set_spawn_modifier_min_spawned(obj: Pin<&mut NiPSysSpawnModifier>, value: u16);
        fn spawn_modifier_max_spawned(obj: &NiPSysSpawnModifier) -> u16;
        fn set_spawn_modifier_max_spawned(obj: Pin<&mut NiPSysSpawnModifier>, value: u16);
        fn spawn_modifier_speed_variation(obj: &NiPSysSpawnModifier) -> f32;
        fn set_spawn_modifier_speed_variation(obj: Pin<&mut NiPSysSpawnModifier>, value: f32);
        fn spawn_modifier_direction_variation(obj: &NiPSysSpawnModifier) -> f32;
        fn set_spawn_modifier_direction_variation(obj: Pin<&mut NiPSysSpawnModifier>, value: f32);
        fn spawn_modifier_life_span(obj: &NiPSysSpawnModifier) -> f32;
        fn set_spawn_modifier_life_span(obj: Pin<&mut NiPSysSpawnModifier>, value: f32);
        fn spawn_modifier_life_span_variation(obj: &NiPSysSpawnModifier) -> f32;
        fn set_spawn_modifier_life_span_variation(obj: Pin<&mut NiPSysSpawnModifier>, value: f32);
        fn age_death_modifier_spawn_on_death(obj: &NiPSysAgeDeathModifier) -> bool;
        fn set_age_death_modifier_spawn_on_death(
            obj: Pin<&mut NiPSysAgeDeathModifier>,
            value: bool,
        );
        fn lod_modifier_lod_begin_distance(obj: &BSPSysLODModifier) -> f32;
        fn set_lod_modifier_lod_begin_distance(obj: Pin<&mut BSPSysLODModifier>, value: f32);
        fn lod_modifier_lod_end_distance(obj: &BSPSysLODModifier) -> f32;
        fn set_lod_modifier_lod_end_distance(obj: Pin<&mut BSPSysLODModifier>, value: f32);
        fn lod_modifier_end_emit_scale(obj: &BSPSysLODModifier) -> f32;
        fn set_lod_modifier_end_emit_scale(obj: Pin<&mut BSPSysLODModifier>, value: f32);
        fn lod_modifier_end_size(obj: &BSPSysLODModifier) -> f32;
        fn set_lod_modifier_end_size(obj: Pin<&mut BSPSysLODModifier>, value: f32);
        fn simple_color_modifier_fade_in_percent(obj: &BSPSysSimpleColorModifier) -> f32;
        fn set_simple_color_modifier_fade_in_percent(
            obj: Pin<&mut BSPSysSimpleColorModifier>,
            value: f32,
        );
        fn simple_color_modifier_fade_out_percent(obj: &BSPSysSimpleColorModifier) -> f32;
        fn set_simple_color_modifier_fade_out_percent(
            obj: Pin<&mut BSPSysSimpleColorModifier>,
            value: f32,
        );
        fn simple_color_modifier_color_1_end_percent(obj: &BSPSysSimpleColorModifier) -> f32;
        fn set_simple_color_modifier_color_1_end_percent(
            obj: Pin<&mut BSPSysSimpleColorModifier>,
            value: f32,
        );
        fn simple_color_modifier_color_2_start_percent(obj: &BSPSysSimpleColorModifier) -> f32;
        fn set_simple_color_modifier_color_2_start_percent(
            obj: Pin<&mut BSPSysSimpleColorModifier>,
            value: f32,
        );
        fn simple_color_modifier_color_2_end_percent(obj: &BSPSysSimpleColorModifier) -> f32;
        fn set_simple_color_modifier_color_2_end_percent(
            obj: Pin<&mut BSPSysSimpleColorModifier>,
            value: f32,
        );
        fn simple_color_modifier_color_3_start_percent(obj: &BSPSysSimpleColorModifier) -> f32;
        fn set_simple_color_modifier_color_3_start_percent(
            obj: Pin<&mut BSPSysSimpleColorModifier>,
            value: f32,
        );
        fn simple_color_modifier_color_1(obj: &BSPSysSimpleColorModifier) -> [f32; 4];
        fn set_simple_color_modifier_color_1(
            obj: Pin<&mut BSPSysSimpleColorModifier>,
            value: [f32; 4],
        );
        fn simple_color_modifier_color_2(obj: &BSPSysSimpleColorModifier) -> [f32; 4];
        fn set_simple_color_modifier_color_2(
            obj: Pin<&mut BSPSysSimpleColorModifier>,
            value: [f32; 4],
        );
        fn simple_color_modifier_color_3(obj: &BSPSysSimpleColorModifier) -> [f32; 4];
        fn set_simple_color_modifier_color_3(
            obj: Pin<&mut BSPSysSimpleColorModifier>,
            value: [f32; 4],
        );
        fn rotation_modifier_initial_speed(obj: &NiPSysRotationModifier) -> f32;
        fn set_rotation_modifier_initial_speed(obj: Pin<&mut NiPSysRotationModifier>, value: f32);
        fn rotation_modifier_initial_speed_variation(obj: &NiPSysRotationModifier) -> f32;
        fn set_rotation_modifier_initial_speed_variation(
            obj: Pin<&mut NiPSysRotationModifier>,
            value: f32,
        );
        fn rotation_modifier_unknown_vector(obj: &NiPSysRotationModifier) -> [f32; 4];
        fn set_rotation_modifier_unknown_vector(
            obj: Pin<&mut NiPSysRotationModifier>,
            value: [f32; 4],
        );
        fn rotation_modifier_unknown_byte(obj: &NiPSysRotationModifier) -> u8;
        fn set_rotation_modifier_unknown_byte(obj: Pin<&mut NiPSysRotationModifier>, value: u8);
        fn rotation_modifier_initial_angle(obj: &NiPSysRotationModifier) -> f32;
        fn set_rotation_modifier_initial_angle(obj: Pin<&mut NiPSysRotationModifier>, value: f32);
        fn rotation_modifier_initial_angle_variation(obj: &NiPSysRotationModifier) -> f32;
        fn set_rotation_modifier_initial_angle_variation(
            obj: Pin<&mut NiPSysRotationModifier>,
            value: f32,
        );
        fn rotation_modifier_random_speed_sign(obj: &NiPSysRotationModifier) -> bool;
        fn set_rotation_modifier_random_speed_sign(
            obj: Pin<&mut NiPSysRotationModifier>,
            value: bool,
        );
        fn rotation_modifier_random_initial_axis(obj: &NiPSysRotationModifier) -> bool;
        fn set_rotation_modifier_random_initial_axis(
            obj: Pin<&mut NiPSysRotationModifier>,
            value: bool,
        );
        fn rotation_modifier_initial_axis(obj: &NiPSysRotationModifier) -> Vec3;
        fn set_rotation_modifier_initial_axis(obj: Pin<&mut NiPSysRotationModifier>, value: Vec3);
        fn gravity_modifier_gravity_axis(obj: &NiPSysGravityModifier) -> Vec3;
        fn set_gravity_modifier_gravity_axis(obj: Pin<&mut NiPSysGravityModifier>, value: Vec3);
        fn gravity_modifier_decay(obj: &NiPSysGravityModifier) -> f32;
        fn set_gravity_modifier_decay(obj: Pin<&mut NiPSysGravityModifier>, value: f32);
        fn gravity_modifier_strength(obj: &NiPSysGravityModifier) -> f32;
        fn set_gravity_modifier_strength(obj: Pin<&mut NiPSysGravityModifier>, value: f32);
        fn gravity_modifier_force_type(obj: &NiPSysGravityModifier) -> u32;
        fn set_gravity_modifier_force_type(obj: Pin<&mut NiPSysGravityModifier>, value: u32);
        fn gravity_modifier_turbulence(obj: &NiPSysGravityModifier) -> f32;
        fn set_gravity_modifier_turbulence(obj: Pin<&mut NiPSysGravityModifier>, value: f32);
        fn gravity_modifier_turbulence_scale(obj: &NiPSysGravityModifier) -> f32;
        fn set_gravity_modifier_turbulence_scale(obj: Pin<&mut NiPSysGravityModifier>, value: f32);
        fn gravity_modifier_world_aligned(obj: &NiPSysGravityModifier) -> bool;
        fn set_gravity_modifier_world_aligned(obj: Pin<&mut NiPSysGravityModifier>, value: bool);
        fn bound_update_modifier_update_skip(obj: &NiPSysBoundUpdateModifier) -> u16;
        fn set_bound_update_modifier_update_skip(
            obj: Pin<&mut NiPSysBoundUpdateModifier>,
            value: u16,
        );
        fn drag_modifier_drag_axis(obj: &NiPSysDragModifier) -> Vec3;
        fn set_drag_modifier_drag_axis(obj: Pin<&mut NiPSysDragModifier>, value: Vec3);
        fn drag_modifier_percentage(obj: &NiPSysDragModifier) -> f32;
        fn set_drag_modifier_percentage(obj: Pin<&mut NiPSysDragModifier>, value: f32);
        fn drag_modifier_range(obj: &NiPSysDragModifier) -> f32;
        fn set_drag_modifier_range(obj: Pin<&mut NiPSysDragModifier>, value: f32);
        fn drag_modifier_range_falloff(obj: &NiPSysDragModifier) -> f32;
        fn set_drag_modifier_range_falloff(obj: Pin<&mut NiPSysDragModifier>, value: f32);
        fn inherit_velocity_modifier_change_to_inherit(obj: &BSPSysInheritVelocityModifier) -> f32;
        fn set_inherit_velocity_modifier_change_to_inherit(
            obj: Pin<&mut BSPSysInheritVelocityModifier>,
            value: f32,
        );
        fn inherit_velocity_modifier_velocity_multiplier(
            obj: &BSPSysInheritVelocityModifier,
        ) -> f32;
        fn set_inherit_velocity_modifier_velocity_multiplier(
            obj: Pin<&mut BSPSysInheritVelocityModifier>,
            value: f32,
        );
        fn inherit_velocity_modifier_velocity_variation(obj: &BSPSysInheritVelocityModifier)
        -> f32;
        fn set_inherit_velocity_modifier_velocity_variation(
            obj: Pin<&mut BSPSysInheritVelocityModifier>,
            value: f32,
        );
        fn sub_tex_modifier_start_frame(obj: &BSPSysSubTexModifier) -> f32;
        fn set_sub_tex_modifier_start_frame(obj: Pin<&mut BSPSysSubTexModifier>, value: f32);
        fn sub_tex_modifier_start_frame_variation(obj: &BSPSysSubTexModifier) -> f32;
        fn set_sub_tex_modifier_start_frame_variation(
            obj: Pin<&mut BSPSysSubTexModifier>,
            value: f32,
        );
        fn sub_tex_modifier_end_frame(obj: &BSPSysSubTexModifier) -> f32;
        fn set_sub_tex_modifier_end_frame(obj: Pin<&mut BSPSysSubTexModifier>, value: f32);
        fn sub_tex_modifier_loop_start_frame(obj: &BSPSysSubTexModifier) -> f32;
        fn set_sub_tex_modifier_loop_start_frame(obj: Pin<&mut BSPSysSubTexModifier>, value: f32);
        fn sub_tex_modifier_loop_start_frame_variation(obj: &BSPSysSubTexModifier) -> f32;
        fn set_sub_tex_modifier_loop_start_frame_variation(
            obj: Pin<&mut BSPSysSubTexModifier>,
            value: f32,
        );
        fn sub_tex_modifier_frame_count(obj: &BSPSysSubTexModifier) -> f32;
        fn set_sub_tex_modifier_frame_count(obj: Pin<&mut BSPSysSubTexModifier>, value: f32);
        fn sub_tex_modifier_frame_count_variation(obj: &BSPSysSubTexModifier) -> f32;
        fn set_sub_tex_modifier_frame_count_variation(
            obj: Pin<&mut BSPSysSubTexModifier>,
            value: f32,
        );
        fn bomb_modifier_bomb_axis(obj: &NiPSysBombModifier) -> Vec3;
        fn set_bomb_modifier_bomb_axis(obj: Pin<&mut NiPSysBombModifier>, value: Vec3);
        fn bomb_modifier_decay(obj: &NiPSysBombModifier) -> f32;
        fn set_bomb_modifier_decay(obj: Pin<&mut NiPSysBombModifier>, value: f32);
        fn bomb_modifier_delta_v(obj: &NiPSysBombModifier) -> f32;
        fn set_bomb_modifier_delta_v(obj: Pin<&mut NiPSysBombModifier>, value: f32);
        fn bomb_modifier_decay_type(obj: &NiPSysBombModifier) -> u32;
        fn set_bomb_modifier_decay_type(obj: Pin<&mut NiPSysBombModifier>, value: u32);
        fn bomb_modifier_symmetry_type(obj: &NiPSysBombModifier) -> u32;
        fn set_bomb_modifier_symmetry_type(obj: Pin<&mut NiPSysBombModifier>, value: u32);
        fn grow_fade_modifier_grow_time(obj: &NiPSysGrowFadeModifier) -> f32;
        fn set_grow_fade_modifier_grow_time(obj: Pin<&mut NiPSysGrowFadeModifier>, value: f32);
        fn grow_fade_modifier_grow_generation(obj: &NiPSysGrowFadeModifier) -> u16;
        fn set_grow_fade_modifier_grow_generation(
            obj: Pin<&mut NiPSysGrowFadeModifier>,
            value: u16,
        );
        fn grow_fade_modifier_fade_time(obj: &NiPSysGrowFadeModifier) -> f32;
        fn set_grow_fade_modifier_fade_time(obj: Pin<&mut NiPSysGrowFadeModifier>, value: f32);
        fn grow_fade_modifier_fade_generation(obj: &NiPSysGrowFadeModifier) -> u16;
        fn set_grow_fade_modifier_fade_generation(
            obj: Pin<&mut NiPSysGrowFadeModifier>,
            value: u16,
        );
        fn grow_fade_modifier_base_scale(obj: &NiPSysGrowFadeModifier) -> f32;
        fn set_grow_fade_modifier_base_scale(obj: Pin<&mut NiPSysGrowFadeModifier>, value: f32);
        fn field_modifier_magnitude(obj: &NiPSysFieldModifier) -> f32;
        fn set_field_modifier_magnitude(obj: Pin<&mut NiPSysFieldModifier>, value: f32);
        fn field_modifier_attenuation(obj: &NiPSysFieldModifier) -> f32;
        fn set_field_modifier_attenuation(obj: Pin<&mut NiPSysFieldModifier>, value: f32);
        fn field_modifier_use_max_distance(obj: &NiPSysFieldModifier) -> bool;
        fn set_field_modifier_use_max_distance(obj: Pin<&mut NiPSysFieldModifier>, value: bool);
        fn field_modifier_max_distance(obj: &NiPSysFieldModifier) -> f32;
        fn set_field_modifier_max_distance(obj: Pin<&mut NiPSysFieldModifier>, value: f32);
        fn vortex_field_modifier_direction(obj: &NiPSysVortexFieldModifier) -> Vec3;
        fn set_vortex_field_modifier_direction(
            obj: Pin<&mut NiPSysVortexFieldModifier>,
            value: Vec3,
        );
        fn gravity_field_modifier_direction(obj: &NiPSysGravityFieldModifier) -> Vec3;
        fn set_gravity_field_modifier_direction(
            obj: Pin<&mut NiPSysGravityFieldModifier>,
            value: Vec3,
        );
        fn drag_field_modifier_use_direction(obj: &NiPSysDragFieldModifier) -> bool;
        fn set_drag_field_modifier_use_direction(
            obj: Pin<&mut NiPSysDragFieldModifier>,
            value: bool,
        );
        fn drag_field_modifier_direction(obj: &NiPSysDragFieldModifier) -> Vec3;
        fn set_drag_field_modifier_direction(obj: Pin<&mut NiPSysDragFieldModifier>, value: Vec3);
        fn turbulence_field_modifier_frequency(obj: &NiPSysTurbulenceFieldModifier) -> f32;
        fn set_turbulence_field_modifier_frequency(
            obj: Pin<&mut NiPSysTurbulenceFieldModifier>,
            value: f32,
        );
        fn air_field_modifier_direction(obj: &NiPSysAirFieldModifier) -> Vec3;
        fn set_air_field_modifier_direction(obj: Pin<&mut NiPSysAirFieldModifier>, value: Vec3);
        fn air_field_modifier_air_friction(obj: &NiPSysAirFieldModifier) -> f32;
        fn set_air_field_modifier_air_friction(obj: Pin<&mut NiPSysAirFieldModifier>, value: f32);
        fn air_field_modifier_inherit_velocity(obj: &NiPSysAirFieldModifier) -> f32;
        fn set_air_field_modifier_inherit_velocity(
            obj: Pin<&mut NiPSysAirFieldModifier>,
            value: f32,
        );
        fn air_field_modifier_inherit_rotation(obj: &NiPSysAirFieldModifier) -> bool;
        fn set_air_field_modifier_inherit_rotation(
            obj: Pin<&mut NiPSysAirFieldModifier>,
            value: bool,
        );
        fn air_field_modifier_component_only(obj: &NiPSysAirFieldModifier) -> bool;
        fn set_air_field_modifier_component_only(
            obj: Pin<&mut NiPSysAirFieldModifier>,
            value: bool,
        );
        fn air_field_modifier_enable_spread(obj: &NiPSysAirFieldModifier) -> bool;
        fn set_air_field_modifier_enable_spread(obj: Pin<&mut NiPSysAirFieldModifier>, value: bool);
        fn air_field_modifier_spread(obj: &NiPSysAirFieldModifier) -> f32;
        fn set_air_field_modifier_spread(obj: Pin<&mut NiPSysAirFieldModifier>, value: f32);
        fn radial_field_modifier_radial_type(obj: &NiPSysRadialFieldModifier) -> u32;
        fn set_radial_field_modifier_radial_type(
            obj: Pin<&mut NiPSysRadialFieldModifier>,
            value: u32,
        );
        fn wind_modifier_strength(obj: &BSWindModifier) -> f32;
        fn set_wind_modifier_strength(obj: Pin<&mut BSWindModifier>, value: f32);
        fn recycle_bound_modifier_bound_offset(obj: &BSPSysRecycleBoundModifier) -> Vec3;
        fn set_recycle_bound_modifier_bound_offset(
            obj: Pin<&mut BSPSysRecycleBoundModifier>,
            value: Vec3,
        );
        fn recycle_bound_modifier_bound_extent(obj: &BSPSysRecycleBoundModifier) -> Vec3;
        fn set_recycle_bound_modifier_bound_extent(
            obj: Pin<&mut BSPSysRecycleBoundModifier>,
            value: Vec3,
        );
        fn parent_velocity_modifier_damping(obj: &BSParentVelocityModifier) -> f32;
        fn set_parent_velocity_modifier_damping(
            obj: Pin<&mut BSParentVelocityModifier>,
            value: f32,
        );
        fn psys_emitter_speed(obj: &NiPSysEmitter) -> f32;
        fn set_psys_emitter_speed(obj: Pin<&mut NiPSysEmitter>, value: f32);
        fn psys_emitter_speed_variation(obj: &NiPSysEmitter) -> f32;
        fn set_psys_emitter_speed_variation(obj: Pin<&mut NiPSysEmitter>, value: f32);
        fn psys_emitter_declination(obj: &NiPSysEmitter) -> f32;
        fn set_psys_emitter_declination(obj: Pin<&mut NiPSysEmitter>, value: f32);
        fn psys_emitter_declination_variation(obj: &NiPSysEmitter) -> f32;
        fn set_psys_emitter_declination_variation(obj: Pin<&mut NiPSysEmitter>, value: f32);
        fn psys_emitter_planar_angle(obj: &NiPSysEmitter) -> f32;
        fn set_psys_emitter_planar_angle(obj: Pin<&mut NiPSysEmitter>, value: f32);
        fn psys_emitter_planar_angle_variation(obj: &NiPSysEmitter) -> f32;
        fn set_psys_emitter_planar_angle_variation(obj: Pin<&mut NiPSysEmitter>, value: f32);
        fn psys_emitter_initial_color(obj: &NiPSysEmitter) -> [f32; 4];
        fn set_psys_emitter_initial_color(obj: Pin<&mut NiPSysEmitter>, value: [f32; 4]);
        fn psys_emitter_initial_radius(obj: &NiPSysEmitter) -> f32;
        fn set_psys_emitter_initial_radius(obj: Pin<&mut NiPSysEmitter>, value: f32);
        fn psys_emitter_radius_variation(obj: &NiPSysEmitter) -> f32;
        fn set_psys_emitter_radius_variation(obj: Pin<&mut NiPSysEmitter>, value: f32);
        fn psys_emitter_life_span(obj: &NiPSysEmitter) -> f32;
        fn set_psys_emitter_life_span(obj: Pin<&mut NiPSysEmitter>, value: f32);
        fn psys_emitter_life_span_variation(obj: &NiPSysEmitter) -> f32;
        fn set_psys_emitter_life_span_variation(obj: Pin<&mut NiPSysEmitter>, value: f32);
        fn sphere_emitter_radius(obj: &NiPSysSphereEmitter) -> f32;
        fn set_sphere_emitter_radius(obj: Pin<&mut NiPSysSphereEmitter>, value: f32);
        fn cylinder_emitter_radius(obj: &NiPSysCylinderEmitter) -> f32;
        fn set_cylinder_emitter_radius(obj: Pin<&mut NiPSysCylinderEmitter>, value: f32);
        fn cylinder_emitter_height(obj: &NiPSysCylinderEmitter) -> f32;
        fn set_cylinder_emitter_height(obj: Pin<&mut NiPSysCylinderEmitter>, value: f32);
        fn box_emitter_width(obj: &NiPSysBoxEmitter) -> f32;
        fn set_box_emitter_width(obj: Pin<&mut NiPSysBoxEmitter>, value: f32);
        fn box_emitter_height(obj: &NiPSysBoxEmitter) -> f32;
        fn set_box_emitter_height(obj: Pin<&mut NiPSysBoxEmitter>, value: f32);
        fn box_emitter_depth(obj: &NiPSysBoxEmitter) -> f32;
        fn set_box_emitter_depth(obj: Pin<&mut NiPSysBoxEmitter>, value: f32);
        fn mesh_emitter_velocity_type(obj: &NiPSysMeshEmitter) -> u32;
        fn set_mesh_emitter_velocity_type(obj: Pin<&mut NiPSysMeshEmitter>, value: u32);
        fn mesh_emitter_emission_type(obj: &NiPSysMeshEmitter) -> u32;
        fn set_mesh_emitter_emission_type(obj: Pin<&mut NiPSysMeshEmitter>, value: u32);
        fn mesh_emitter_emission_axis(obj: &NiPSysMeshEmitter) -> Vec3;
        fn set_mesh_emitter_emission_axis(obj: Pin<&mut NiPSysMeshEmitter>, value: Vec3);
        fn psys_collider_bounce(obj: &NiPSysCollider) -> f32;
        fn set_psys_collider_bounce(obj: Pin<&mut NiPSysCollider>, value: f32);
        fn psys_collider_spawn_on_collide(obj: &NiPSysCollider) -> bool;
        fn set_psys_collider_spawn_on_collide(obj: Pin<&mut NiPSysCollider>, value: bool);
        fn psys_collider_die_on_collide(obj: &NiPSysCollider) -> bool;
        fn set_psys_collider_die_on_collide(obj: Pin<&mut NiPSysCollider>, value: bool);
        fn spherical_collider_radius(obj: &NiPSysSphericalCollider) -> f32;
        fn set_spherical_collider_radius(obj: Pin<&mut NiPSysSphericalCollider>, value: f32);
        fn planar_collider_width(obj: &NiPSysPlanarCollider) -> f32;
        fn set_planar_collider_width(obj: Pin<&mut NiPSysPlanarCollider>, value: f32);
        fn planar_collider_height(obj: &NiPSysPlanarCollider) -> f32;
        fn set_planar_collider_height(obj: Pin<&mut NiPSysPlanarCollider>, value: f32);
        fn planar_collider_x_axis(obj: &NiPSysPlanarCollider) -> Vec3;
        fn set_planar_collider_x_axis(obj: Pin<&mut NiPSysPlanarCollider>, value: Vec3);
        fn planar_collider_y_axis(obj: &NiPSysPlanarCollider) -> Vec3;
        fn set_planar_collider_y_axis(obj: Pin<&mut NiPSysPlanarCollider>, value: Vec3);
        fn multi_target_emitter_ctlr_max_emitters(obj: &BSPSysMultiTargetEmitterCtlr) -> u16;
        fn set_multi_target_emitter_ctlr_max_emitters(
            obj: Pin<&mut BSPSysMultiTargetEmitterCtlr>,
            value: u16,
        );
        // Per-particle lists.
        fn set_particles_data_vertex_count(obj: Pin<&mut NiParticlesData>, count: u16);
        fn particles_data_radii(obj: &NiParticlesData) -> &[f32];
        fn set_particles_data_radii(obj: Pin<&mut NiParticlesData>, values: &[f32]) -> bool;
        fn particles_data_sizes(obj: &NiParticlesData) -> &[f32];
        fn set_particles_data_sizes(obj: Pin<&mut NiParticlesData>, values: &[f32]) -> bool;
        fn particles_data_rotation_angles(obj: &NiParticlesData) -> &[f32];
        fn set_particles_data_rotation_angles(
            obj: Pin<&mut NiParticlesData>,
            values: &[f32],
        ) -> bool;
        fn particles_data_rotations(obj: &NiParticlesData) -> Vec<f32>;
        fn set_particles_data_rotations(obj: Pin<&mut NiParticlesData>, xyzw: &[f32]) -> bool;
        fn particles_data_rotation_axes(obj: &NiParticlesData) -> Vec<Vec3>;
        fn set_particles_data_rotation_axes(obj: Pin<&mut NiParticlesData>, axes: &[Vec3]) -> bool;
        fn particles_data_subtexture_offsets(obj: &NiParticlesData) -> Vec<f32>;
        fn set_particles_data_subtexture_offsets(obj: Pin<&mut NiParticlesData>, xyzw: &[f32]);
        fn psys_data_rotation_speeds(obj: &NiPSysData) -> &[f32];
        fn set_psys_data_rotation_speeds(obj: Pin<&mut NiPSysData>, values: &[f32]) -> bool;
        fn psys_data_particle_info(obj: &NiPSysData) -> Vec<ParticleInfoFfi>;
        fn set_psys_data_particle_info(obj: Pin<&mut NiPSysData>, info: &[ParticleInfoFfi])
        -> bool;
        fn mesh_psys_data_generation_pool_sizes(obj: &NiMeshPSysData) -> Vec<u32>;
        fn set_mesh_psys_data_generation_pool_sizes(obj: Pin<&mut NiMeshPSysData>, sizes: &[u32]);

        // NiPSysEmitterCtlrData.
        fn emitter_ctlr_data_keys(obj: &NiPSysEmitterCtlrData) -> KeyGroupFfi;
        fn set_emitter_ctlr_data_keys(
            obj: Pin<&mut NiPSysEmitterCtlrData>,
            group: &KeyGroupFfi,
        ) -> bool;
        fn emitter_ctlr_data_visibility_keys(obj: &NiPSysEmitterCtlrData) -> Vec<KeyFfi>;
        fn set_emitter_ctlr_data_visibility_keys(
            obj: Pin<&mut NiPSysEmitterCtlrData>,
            keys: &[KeyFfi],
        );

        // NiParticleSystem.
        fn particle_system_vertex_raw_flags(obj: &NiParticleSystem) -> [u8; 8];
        fn set_particle_system_vertex_raw_flags(obj: Pin<&mut NiParticleSystem>, flags: [u8; 8]);
        fn particle_system_bounds(obj: &NiParticleSystem) -> BoundingSphere;
        fn set_particle_system_bounds(obj: Pin<&mut NiParticleSystem>, bounds: BoundingSphere);
        fn particle_system_bound_min_max(obj: &NiParticleSystem) -> [f32; 6];
        fn set_particle_system_bound_min_max(obj: Pin<&mut NiParticleSystem>, values: [f32; 6]);
        fn particle_system_material_count(obj: &NiParticleSystem) -> usize;
        fn particle_system_material_name(obj: &NiParticleSystem, i: usize) -> &[u8];
        fn particle_system_material_extra_data(obj: &NiParticleSystem) -> &[u32];
        fn set_particle_system_materials(
            obj: Pin<&mut NiParticleSystem>,
            bytes: &[u8],
            lengths: &[usize],
            extra_data: &[u32],
        );
        fn particle_system_has_shader(obj: &NiParticleSystem) -> bool;
        fn particle_system_shader_name(obj: &NiParticleSystem) -> &[u8];
        fn set_particle_system_shader(obj: Pin<&mut NiParticleSystem>, has: bool, name: &[u8]);

        // Names.
        fn psys_modifier_name(obj: &NiPSysModifier) -> &[u8];
        fn set_psys_modifier_name(obj: Pin<&mut NiPSysModifier>, name: &[u8]);
        fn psys_modifier_ctlr_modifier_name(obj: &NiPSysModifierCtlr) -> &[u8];
        fn set_psys_modifier_ctlr_modifier_name(obj: Pin<&mut NiPSysModifierCtlr>, name: &[u8]);

        // Old particles.
        fn gravity_decay(obj: &NiGravity) -> f32;
        fn set_gravity_decay(obj: Pin<&mut NiGravity>, value: f32);
        fn gravity_force(obj: &NiGravity) -> f32;
        fn set_gravity_force(obj: Pin<&mut NiGravity>, value: f32);
        fn gravity_field_type(obj: &NiGravity) -> u32;
        fn set_gravity_field_type(obj: Pin<&mut NiGravity>, value: u32);
        fn gravity_position(obj: &NiGravity) -> Vec3;
        fn set_gravity_position(obj: Pin<&mut NiGravity>, value: Vec3);
        fn gravity_direction(obj: &NiGravity) -> Vec3;
        fn set_gravity_direction(obj: Pin<&mut NiGravity>, value: Vec3);
        fn grow_fade_grow(obj: &NiParticleGrowFade) -> f32;
        fn set_grow_fade_grow(obj: Pin<&mut NiParticleGrowFade>, value: f32);
        fn grow_fade_fade(obj: &NiParticleGrowFade) -> f32;
        fn set_grow_fade_fade(obj: Pin<&mut NiParticleGrowFade>, value: f32);
        fn particle_rotation_random_initial_axis(obj: &NiParticleRotation) -> bool;
        fn set_particle_rotation_random_initial_axis(
            obj: Pin<&mut NiParticleRotation>,
            value: bool,
        );
        fn particle_rotation_initial_axis(obj: &NiParticleRotation) -> Vec3;
        fn set_particle_rotation_initial_axis(obj: Pin<&mut NiParticleRotation>, value: Vec3);
        fn particle_rotation_rotation_speed(obj: &NiParticleRotation) -> f32;
        fn set_particle_rotation_rotation_speed(obj: Pin<&mut NiParticleRotation>, value: f32);
        fn particle_bomb_decay(obj: &NiParticleBomb) -> f32;
        fn set_particle_bomb_decay(obj: Pin<&mut NiParticleBomb>, value: f32);
        fn particle_bomb_duration(obj: &NiParticleBomb) -> f32;
        fn set_particle_bomb_duration(obj: Pin<&mut NiParticleBomb>, value: f32);
        fn particle_bomb_delta_v(obj: &NiParticleBomb) -> f32;
        fn set_particle_bomb_delta_v(obj: Pin<&mut NiParticleBomb>, value: f32);
        fn particle_bomb_start(obj: &NiParticleBomb) -> f32;
        fn set_particle_bomb_start(obj: Pin<&mut NiParticleBomb>, value: f32);
        fn particle_bomb_decay_type(obj: &NiParticleBomb) -> u32;
        fn set_particle_bomb_decay_type(obj: Pin<&mut NiParticleBomb>, value: u32);
        fn particle_bomb_symmetry_type(obj: &NiParticleBomb) -> u32;
        fn set_particle_bomb_symmetry_type(obj: Pin<&mut NiParticleBomb>, value: u32);
        fn particle_bomb_position(obj: &NiParticleBomb) -> Vec3;
        fn set_particle_bomb_position(obj: Pin<&mut NiParticleBomb>, value: Vec3);
        fn particle_bomb_direction(obj: &NiParticleBomb) -> Vec3;
        fn set_particle_bomb_direction(obj: Pin<&mut NiParticleBomb>, value: Vec3);
        fn particle_collider_bounce(obj: &NiParticleCollider) -> f32;
        fn set_particle_collider_bounce(obj: Pin<&mut NiParticleCollider>, value: f32);
        fn particle_collider_spawn_on_collide(obj: &NiParticleCollider) -> bool;
        fn set_particle_collider_spawn_on_collide(obj: Pin<&mut NiParticleCollider>, value: bool);
        fn particle_collider_die_on_collide(obj: &NiParticleCollider) -> bool;
        fn set_particle_collider_die_on_collide(obj: Pin<&mut NiParticleCollider>, value: bool);
        fn old_planar_collider_width(obj: &NiPlanarCollider) -> f32;
        fn set_old_planar_collider_width(obj: Pin<&mut NiPlanarCollider>, value: f32);
        fn old_planar_collider_height(obj: &NiPlanarCollider) -> f32;
        fn set_old_planar_collider_height(obj: Pin<&mut NiPlanarCollider>, value: f32);
        fn old_planar_collider_position(obj: &NiPlanarCollider) -> Vec3;
        fn set_old_planar_collider_position(obj: Pin<&mut NiPlanarCollider>, value: Vec3);
        fn old_planar_collider_x_axis(obj: &NiPlanarCollider) -> Vec3;
        fn set_old_planar_collider_x_axis(obj: Pin<&mut NiPlanarCollider>, value: Vec3);
        fn old_planar_collider_y_axis(obj: &NiPlanarCollider) -> Vec3;
        fn set_old_planar_collider_y_axis(obj: Pin<&mut NiPlanarCollider>, value: Vec3);
        fn old_spherical_collider_radius(obj: &NiSphericalCollider) -> f32;
        fn set_old_spherical_collider_radius(obj: Pin<&mut NiSphericalCollider>, value: f32);
        fn old_spherical_collider_position(obj: &NiSphericalCollider) -> Vec3;
        fn set_old_spherical_collider_position(obj: Pin<&mut NiSphericalCollider>, value: Vec3);
        fn particle_system_ctlr_speed(obj: &NiParticleSystemController) -> f32;
        fn set_particle_system_ctlr_speed(obj: Pin<&mut NiParticleSystemController>, value: f32);
        fn particle_system_ctlr_speed_variation(obj: &NiParticleSystemController) -> f32;
        fn set_particle_system_ctlr_speed_variation(
            obj: Pin<&mut NiParticleSystemController>,
            value: f32,
        );
        fn particle_system_ctlr_declination(obj: &NiParticleSystemController) -> f32;
        fn set_particle_system_ctlr_declination(
            obj: Pin<&mut NiParticleSystemController>,
            value: f32,
        );
        fn particle_system_ctlr_declination_variation(obj: &NiParticleSystemController) -> f32;
        fn set_particle_system_ctlr_declination_variation(
            obj: Pin<&mut NiParticleSystemController>,
            value: f32,
        );
        fn particle_system_ctlr_planar_angle(obj: &NiParticleSystemController) -> f32;
        fn set_particle_system_ctlr_planar_angle(
            obj: Pin<&mut NiParticleSystemController>,
            value: f32,
        );
        fn particle_system_ctlr_planar_angle_variation(obj: &NiParticleSystemController) -> f32;
        fn set_particle_system_ctlr_planar_angle_variation(
            obj: Pin<&mut NiParticleSystemController>,
            value: f32,
        );
        fn particle_system_ctlr_initial_normal(obj: &NiParticleSystemController) -> Vec3;
        fn set_particle_system_ctlr_initial_normal(
            obj: Pin<&mut NiParticleSystemController>,
            value: Vec3,
        );
        fn particle_system_ctlr_initial_color(obj: &NiParticleSystemController) -> [f32; 4];
        fn set_particle_system_ctlr_initial_color(
            obj: Pin<&mut NiParticleSystemController>,
            value: [f32; 4],
        );
        fn particle_system_ctlr_initial_size(obj: &NiParticleSystemController) -> f32;
        fn set_particle_system_ctlr_initial_size(
            obj: Pin<&mut NiParticleSystemController>,
            value: f32,
        );
        fn particle_system_ctlr_emit_start_time(obj: &NiParticleSystemController) -> f32;
        fn set_particle_system_ctlr_emit_start_time(
            obj: Pin<&mut NiParticleSystemController>,
            value: f32,
        );
        fn particle_system_ctlr_emit_stop_time(obj: &NiParticleSystemController) -> f32;
        fn set_particle_system_ctlr_emit_stop_time(
            obj: Pin<&mut NiParticleSystemController>,
            value: f32,
        );
        fn particle_system_ctlr_reset_particle_system(obj: &NiParticleSystemController) -> bool;
        fn set_particle_system_ctlr_reset_particle_system(
            obj: Pin<&mut NiParticleSystemController>,
            value: bool,
        );
        fn particle_system_ctlr_birth_rate(obj: &NiParticleSystemController) -> f32;
        fn set_particle_system_ctlr_birth_rate(
            obj: Pin<&mut NiParticleSystemController>,
            value: f32,
        );
        fn particle_system_ctlr_lifetime(obj: &NiParticleSystemController) -> f32;
        fn set_particle_system_ctlr_lifetime(obj: Pin<&mut NiParticleSystemController>, value: f32);
        fn particle_system_ctlr_lifetime_variation(obj: &NiParticleSystemController) -> f32;
        fn set_particle_system_ctlr_lifetime_variation(
            obj: Pin<&mut NiParticleSystemController>,
            value: f32,
        );
        fn particle_system_ctlr_use_birth_rate(obj: &NiParticleSystemController) -> bool;
        fn set_particle_system_ctlr_use_birth_rate(
            obj: Pin<&mut NiParticleSystemController>,
            value: bool,
        );
        fn particle_system_ctlr_spawn_on_death(obj: &NiParticleSystemController) -> bool;
        fn set_particle_system_ctlr_spawn_on_death(
            obj: Pin<&mut NiParticleSystemController>,
            value: bool,
        );
        fn particle_system_ctlr_emitter_dimensions(obj: &NiParticleSystemController) -> Vec3;
        fn set_particle_system_ctlr_emitter_dimensions(
            obj: Pin<&mut NiParticleSystemController>,
            value: Vec3,
        );
        fn particle_system_ctlr_spawn_generations(obj: &NiParticleSystemController) -> u16;
        fn set_particle_system_ctlr_spawn_generations(
            obj: Pin<&mut NiParticleSystemController>,
            value: u16,
        );
        fn particle_system_ctlr_percent_spawned(obj: &NiParticleSystemController) -> f32;
        fn set_particle_system_ctlr_percent_spawned(
            obj: Pin<&mut NiParticleSystemController>,
            value: f32,
        );
        fn particle_system_ctlr_spawn_multiplier(obj: &NiParticleSystemController) -> u16;
        fn set_particle_system_ctlr_spawn_multiplier(
            obj: Pin<&mut NiParticleSystemController>,
            value: u16,
        );
        fn particle_system_ctlr_spawn_speed_chaos(obj: &NiParticleSystemController) -> f32;
        fn set_particle_system_ctlr_spawn_speed_chaos(
            obj: Pin<&mut NiParticleSystemController>,
            value: f32,
        );
        fn particle_system_ctlr_spawn_direction_chaos(obj: &NiParticleSystemController) -> f32;
        fn set_particle_system_ctlr_spawn_direction_chaos(
            obj: Pin<&mut NiParticleSystemController>,
            value: f32,
        );
        fn particle_system_ctlr_active_count(obj: &NiParticleSystemController) -> u16;
        fn set_particle_system_ctlr_active_count(
            obj: Pin<&mut NiParticleSystemController>,
            value: u16,
        );
        fn particle_system_ctlr_static_target_bound(obj: &NiParticleSystemController) -> bool;
        fn set_particle_system_ctlr_static_target_bound(
            obj: Pin<&mut NiParticleSystemController>,
            value: bool,
        );
        fn old_planar_collider_plane_normal(obj: &NiPlanarCollider) -> Vec3;
        fn set_old_planar_collider_plane_normal(obj: Pin<&mut NiPlanarCollider>, value: Vec3);
        fn old_planar_collider_plane_constant(obj: &NiPlanarCollider) -> f32;
        fn set_old_planar_collider_plane_constant(obj: Pin<&mut NiPlanarCollider>, value: f32);
        fn rotating_particles_data_has_rotations_2(obj: &NiRotatingParticlesData) -> bool;
        fn set_rotating_particles_data_has_rotations_2(
            obj: Pin<&mut NiRotatingParticlesData>,
            value: bool,
        );
        fn rotating_particles_data_rotations_2(obj: &NiRotatingParticlesData) -> Vec<f32>;
        fn set_rotating_particles_data_rotations_2(
            obj: Pin<&mut NiRotatingParticlesData>,
            xyzw: &[f32],
        ) -> bool;
        fn particle_system_ctlr_particles(obj: &NiParticleSystemController)
        -> Vec<ParticleInfoFfi>;
        fn set_particle_system_ctlr_particles(
            obj: Pin<&mut NiParticleSystemController>,
            info: &[ParticleInfoFfi],
        ) -> bool;

        // Modifier lists.
        fn simple_color_modifier_unknown_shorts(obj: &BSPSysSimpleColorModifier) -> [u16; 26];
        fn set_simple_color_modifier_unknown_shorts(
            obj: Pin<&mut BSPSysSimpleColorModifier>,
            values: [u16; 26],
        );
        fn scale_modifier_scales(obj: &BSPSysScaleModifier) -> Vec<f32>;
        fn set_scale_modifier_scales(obj: Pin<&mut BSPSysScaleModifier>, scales: &[f32]);

        // Creation.
        fn add_ni_particle_system(
            file: Pin<&mut NifFile>,
            parent: u32,
            name: &[u8],
            transform: Transform,
        ) -> u32;
        fn add_ni_mesh_particle_system(
            file: Pin<&mut NifFile>,
            parent: u32,
            name: &[u8],
            transform: Transform,
        ) -> u32;
        fn add_bs_strip_particle_system(
            file: Pin<&mut NifFile>,
            parent: u32,
            name: &[u8],
            transform: Transform,
        ) -> u32;
        fn add_ni_psys_spherical_collider(file: Pin<&mut NifFile>, manager: u32) -> u32;
        fn add_ni_psys_planar_collider(file: Pin<&mut NifFile>, manager: u32) -> u32;
        fn add_ni_psys_emitter_ctlr_data(file: Pin<&mut NifFile>) -> u32;
        fn add_bs_psys_strip_update_modifier(
            file: Pin<&mut NifFile>,
            system: u32,
            name: &[u8],
        ) -> u32;
        fn add_ni_psys_spawn_modifier(file: Pin<&mut NifFile>, system: u32, name: &[u8]) -> u32;
        fn add_ni_psys_age_death_modifier(file: Pin<&mut NifFile>, system: u32, name: &[u8])
        -> u32;
        fn add_bs_psys_lod_modifier(file: Pin<&mut NifFile>, system: u32, name: &[u8]) -> u32;
        fn add_bs_psys_simple_color_modifier(
            file: Pin<&mut NifFile>,
            system: u32,
            name: &[u8],
        ) -> u32;
        fn add_ni_psys_rotation_modifier(file: Pin<&mut NifFile>, system: u32, name: &[u8]) -> u32;
        fn add_bs_psys_scale_modifier(file: Pin<&mut NifFile>, system: u32, name: &[u8]) -> u32;
        fn add_ni_psys_gravity_modifier(file: Pin<&mut NifFile>, system: u32, name: &[u8]) -> u32;
        fn add_ni_psys_position_modifier(file: Pin<&mut NifFile>, system: u32, name: &[u8]) -> u32;
        fn add_ni_psys_bound_update_modifier(
            file: Pin<&mut NifFile>,
            system: u32,
            name: &[u8],
        ) -> u32;
        fn add_ni_psys_drag_modifier(file: Pin<&mut NifFile>, system: u32, name: &[u8]) -> u32;
        fn add_bs_psys_inherit_velocity_modifier(
            file: Pin<&mut NifFile>,
            system: u32,
            name: &[u8],
        ) -> u32;
        fn add_bs_psys_sub_tex_modifier(file: Pin<&mut NifFile>, system: u32, name: &[u8]) -> u32;
        fn add_ni_psys_bomb_modifier(file: Pin<&mut NifFile>, system: u32, name: &[u8]) -> u32;
        fn add_ni_psys_color_modifier(file: Pin<&mut NifFile>, system: u32, name: &[u8]) -> u32;
        fn add_ni_psys_grow_fade_modifier(file: Pin<&mut NifFile>, system: u32, name: &[u8])
        -> u32;
        fn add_ni_psys_mesh_update_modifier(
            file: Pin<&mut NifFile>,
            system: u32,
            name: &[u8],
        ) -> u32;
        fn add_ni_psys_vortex_field_modifier(
            file: Pin<&mut NifFile>,
            system: u32,
            name: &[u8],
        ) -> u32;
        fn add_ni_psys_gravity_field_modifier(
            file: Pin<&mut NifFile>,
            system: u32,
            name: &[u8],
        ) -> u32;
        fn add_ni_psys_drag_field_modifier(
            file: Pin<&mut NifFile>,
            system: u32,
            name: &[u8],
        ) -> u32;
        fn add_ni_psys_turbulence_field_modifier(
            file: Pin<&mut NifFile>,
            system: u32,
            name: &[u8],
        ) -> u32;
        fn add_ni_psys_air_field_modifier(file: Pin<&mut NifFile>, system: u32, name: &[u8])
        -> u32;
        fn add_ni_psys_radial_field_modifier(
            file: Pin<&mut NifFile>,
            system: u32,
            name: &[u8],
        ) -> u32;
        fn add_bs_wind_modifier(file: Pin<&mut NifFile>, system: u32, name: &[u8]) -> u32;
        fn add_bs_psys_recycle_bound_modifier(
            file: Pin<&mut NifFile>,
            system: u32,
            name: &[u8],
        ) -> u32;
        fn add_bs_psys_havok_update_modifier(
            file: Pin<&mut NifFile>,
            system: u32,
            name: &[u8],
        ) -> u32;
        fn add_bs_parent_velocity_modifier(
            file: Pin<&mut NifFile>,
            system: u32,
            name: &[u8],
        ) -> u32;
        fn add_ni_psys_collider_manager(file: Pin<&mut NifFile>, system: u32, name: &[u8]) -> u32;
        fn add_ni_psys_sphere_emitter(file: Pin<&mut NifFile>, system: u32, name: &[u8]) -> u32;
        fn add_ni_psys_cylinder_emitter(file: Pin<&mut NifFile>, system: u32, name: &[u8]) -> u32;
        fn add_ni_psys_box_emitter(file: Pin<&mut NifFile>, system: u32, name: &[u8]) -> u32;
        fn add_bs_psys_array_emitter(file: Pin<&mut NifFile>, system: u32, name: &[u8]) -> u32;
        fn add_ni_psys_mesh_emitter(file: Pin<&mut NifFile>, system: u32, name: &[u8]) -> u32;
        fn add_ni_psys_emitter_ctlr(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_bs_psys_multi_target_emitter_ctlr(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_psys_update_ctlr(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_psys_reset_on_loop_ctlr(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_psys_modifier_active_ctlr(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_psys_emitter_life_span_ctlr(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_psys_emitter_speed_ctlr(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_psys_emitter_initial_radius_ctlr(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_psys_emitter_declination_ctlr(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_psys_gravity_strength_ctlr(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_psys_emitter_declination_var_ctlr(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_psys_field_magnitude_ctlr(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_psys_field_attenuation_ctlr(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_psys_field_max_distance_ctlr(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_psys_air_field_air_friction_ctlr(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_psys_air_field_inherit_velocity_ctlr(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_psys_air_field_spread_ctlr(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_psys_initial_rot_speed_ctlr(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_psys_initial_rot_speed_var_ctlr(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_psys_initial_rot_angle_ctlr(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_psys_initial_rot_angle_var_ctlr(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_psys_emitter_planar_angle_ctlr(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_psys_emitter_planar_angle_var_ctlr(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_psys_rot_dampening_ctlr(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_particles(
            file: Pin<&mut NifFile>,
            parent: u32,
            name: &[u8],
            transform: Transform,
        ) -> u32;
        fn add_ni_auto_normal_particles(
            file: Pin<&mut NifFile>,
            parent: u32,
            name: &[u8],
            transform: Transform,
        ) -> u32;
        fn add_ni_rotating_particles(
            file: Pin<&mut NifFile>,
            parent: u32,
            name: &[u8],
            transform: Transform,
        ) -> u32;
        fn add_ni_particle_meshes(
            file: Pin<&mut NifFile>,
            parent: u32,
            name: &[u8],
            transform: Transform,
        ) -> u32;
        fn add_ni_gravity(file: Pin<&mut NifFile>, controller: u32) -> u32;
        fn add_ni_particle_grow_fade(file: Pin<&mut NifFile>, controller: u32) -> u32;
        fn add_ni_particle_color_modifier(file: Pin<&mut NifFile>, controller: u32) -> u32;
        fn add_ni_particle_rotation(file: Pin<&mut NifFile>, controller: u32) -> u32;
        fn add_ni_particle_bomb(file: Pin<&mut NifFile>, controller: u32) -> u32;
        fn add_ni_particle_mesh_modifier(file: Pin<&mut NifFile>, controller: u32) -> u32;
        fn add_ni_planar_collider(file: Pin<&mut NifFile>, controller: u32) -> u32;
        fn add_ni_spherical_collider(file: Pin<&mut NifFile>, controller: u32) -> u32;
        fn add_ni_emitter_modifier(file: Pin<&mut NifFile>, controller: u32) -> u32;
        fn add_ni_particle_system_controller(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_bsp_array_controller(file: Pin<&mut NifFile>, owner: u32) -> u32;
    }
}
