#pragma once
#include <array>
#include <cstdint>
#include <memory>
#include <set>
#include <string>
#include <type_traits>
#include <vector>

#include "NifFile.hpp"
#include "Particles.hpp"
#include "animation_shim.h"
#include "collision_shim.h"
#include "nifly-sys/src/object_3d.rs.h"
#include "nifly-sys/src/particles.rs.h"
#include "rust/cxx.h"
#include "transform.h"

// Upcasts and downcasts use NIFLY_RS_ANIMATION, and plain fields
// NIFLY_RS_ANIMATION_FIELD and NIFLY_RS_ANIMATION_VEC3 (animation_shim.h).

// A Color4 field as r, g, b, a.
#define NIFLY_RS_PARTICLE_COLOR(T, field, snake) \
    inline std::array<float, 4> snake(const nifly::T& obj) { \
        return {obj.field.r, obj.field.g, obj.field.b, obj.field.a}; \
    } \
    inline void set_##snake(nifly::T& obj, std::array<float, 4> value) { \
        obj.field = nifly::Color4(value[0], value[1], value[2], value[3]); \
    }

// A Vector4 field as x, y, z, w.
#define NIFLY_RS_PARTICLE_VEC4(T, field, snake) \
    inline std::array<float, 4> snake(const nifly::T& obj) { \
        return {obj.field.x, obj.field.y, obj.field.z, obj.field.w}; \
    } \
    inline void set_##snake(nifly::T& obj, std::array<float, 4> value) { \
        obj.field = nifly::Vector4(value[0], value[1], value[2], value[3]); \
    }

// A byte flag as a bool. Values other than 1 are kept unless the flag
// changes.
#define NIFLY_RS_PARTICLE_BYTE_BOOL(T, field, snake) \
    inline bool snake(const nifly::T& obj) { \
        return obj.field != 0; \
    } \
    inline void set_##snake(nifly::T& obj, bool value) { \
        if ((obj.field != 0) != value) \
            obj.field = value ? 1 : 0; \
    }

namespace nifly_rs {
NIFLY_RS_ANIMATION(NiParticlesData, NiGeometryData, ni_particles_data)
NIFLY_RS_ANIMATION(
    NiAutoNormalParticlesData,
    NiParticlesData,
    ni_auto_normal_particles_data
)
NIFLY_RS_ANIMATION(
    NiRotatingParticlesData,
    NiParticlesData,
    ni_rotating_particles_data
)
NIFLY_RS_ANIMATION(
    NiParticleMeshesData,
    NiRotatingParticlesData,
    ni_particle_meshes_data
)
NIFLY_RS_ANIMATION(NiPSysData, NiRotatingParticlesData, ni_psys_data)
NIFLY_RS_ANIMATION(NiMeshPSysData, NiPSysData, ni_mesh_psys_data)
NIFLY_RS_ANIMATION(BSStripPSysData, NiPSysData, bs_strip_psys_data)
NIFLY_RS_ANIMATION(NiPSysEmitterCtlrData, NiObject, ni_psys_emitter_ctlr_data)
NIFLY_RS_ANIMATION(NiParticleSystem, NiAVObject, ni_particle_system)
NIFLY_RS_ANIMATION(
    NiMeshParticleSystem,
    NiParticleSystem,
    ni_mesh_particle_system
)
NIFLY_RS_ANIMATION(
    BSStripParticleSystem,
    NiParticleSystem,
    bs_strip_particle_system
)
NIFLY_RS_ANIMATION(NiPSysModifier, NiObject, ni_psys_modifier)
NIFLY_RS_ANIMATION(
    BSPSysStripUpdateModifier,
    NiPSysModifier,
    bs_psys_strip_update_modifier
)
NIFLY_RS_ANIMATION(NiPSysSpawnModifier, NiPSysModifier, ni_psys_spawn_modifier)
NIFLY_RS_ANIMATION(
    NiPSysAgeDeathModifier,
    NiPSysModifier,
    ni_psys_age_death_modifier
)
NIFLY_RS_ANIMATION(BSPSysLODModifier, NiPSysModifier, bs_psys_lod_modifier)
NIFLY_RS_ANIMATION(
    BSPSysSimpleColorModifier,
    NiPSysModifier,
    bs_psys_simple_color_modifier
)
NIFLY_RS_ANIMATION(
    NiPSysRotationModifier,
    NiPSysModifier,
    ni_psys_rotation_modifier
)
NIFLY_RS_ANIMATION(BSPSysScaleModifier, NiPSysModifier, bs_psys_scale_modifier)
NIFLY_RS_ANIMATION(
    NiPSysGravityModifier,
    NiPSysModifier,
    ni_psys_gravity_modifier
)
NIFLY_RS_ANIMATION(
    NiPSysPositionModifier,
    NiPSysModifier,
    ni_psys_position_modifier
)
NIFLY_RS_ANIMATION(
    NiPSysBoundUpdateModifier,
    NiPSysModifier,
    ni_psys_bound_update_modifier
)
NIFLY_RS_ANIMATION(NiPSysDragModifier, NiPSysModifier, ni_psys_drag_modifier)
NIFLY_RS_ANIMATION(
    BSPSysInheritVelocityModifier,
    NiPSysModifier,
    bs_psys_inherit_velocity_modifier
)
NIFLY_RS_ANIMATION(
    BSPSysSubTexModifier,
    NiPSysModifier,
    bs_psys_sub_tex_modifier
)
NIFLY_RS_ANIMATION(NiPSysBombModifier, NiPSysModifier, ni_psys_bomb_modifier)
NIFLY_RS_ANIMATION(NiPSysColorModifier, NiPSysModifier, ni_psys_color_modifier)
NIFLY_RS_ANIMATION(
    NiPSysGrowFadeModifier,
    NiPSysModifier,
    ni_psys_grow_fade_modifier
)
NIFLY_RS_ANIMATION(
    NiPSysMeshUpdateModifier,
    NiPSysModifier,
    ni_psys_mesh_update_modifier
)
NIFLY_RS_ANIMATION(NiPSysFieldModifier, NiPSysModifier, ni_psys_field_modifier)
NIFLY_RS_ANIMATION(
    NiPSysVortexFieldModifier,
    NiPSysFieldModifier,
    ni_psys_vortex_field_modifier
)
NIFLY_RS_ANIMATION(
    NiPSysGravityFieldModifier,
    NiPSysFieldModifier,
    ni_psys_gravity_field_modifier
)
NIFLY_RS_ANIMATION(
    NiPSysDragFieldModifier,
    NiPSysFieldModifier,
    ni_psys_drag_field_modifier
)
NIFLY_RS_ANIMATION(
    NiPSysTurbulenceFieldModifier,
    NiPSysFieldModifier,
    ni_psys_turbulence_field_modifier
)
NIFLY_RS_ANIMATION(
    NiPSysAirFieldModifier,
    NiPSysFieldModifier,
    ni_psys_air_field_modifier
)
NIFLY_RS_ANIMATION(
    NiPSysRadialFieldModifier,
    NiPSysFieldModifier,
    ni_psys_radial_field_modifier
)
NIFLY_RS_ANIMATION(BSWindModifier, NiPSysModifier, bs_wind_modifier)
NIFLY_RS_ANIMATION(
    BSPSysRecycleBoundModifier,
    NiPSysModifier,
    bs_psys_recycle_bound_modifier
)
NIFLY_RS_ANIMATION(
    BSPSysHavokUpdateModifier,
    NiPSysModifier,
    bs_psys_havok_update_modifier
)
NIFLY_RS_ANIMATION(
    BSParentVelocityModifier,
    NiPSysModifier,
    bs_parent_velocity_modifier
)
NIFLY_RS_ANIMATION(
    NiPSysColliderManager,
    NiPSysModifier,
    ni_psys_collider_manager
)
NIFLY_RS_ANIMATION(NiPSysEmitter, NiPSysModifier, ni_psys_emitter)
NIFLY_RS_ANIMATION(NiPSysVolumeEmitter, NiPSysEmitter, ni_psys_volume_emitter)
NIFLY_RS_ANIMATION(
    NiPSysSphereEmitter,
    NiPSysVolumeEmitter,
    ni_psys_sphere_emitter
)
NIFLY_RS_ANIMATION(
    NiPSysCylinderEmitter,
    NiPSysVolumeEmitter,
    ni_psys_cylinder_emitter
)
NIFLY_RS_ANIMATION(NiPSysBoxEmitter, NiPSysVolumeEmitter, ni_psys_box_emitter)
NIFLY_RS_ANIMATION(
    BSPSysArrayEmitter,
    NiPSysVolumeEmitter,
    bs_psys_array_emitter
)
NIFLY_RS_ANIMATION(NiPSysMeshEmitter, NiPSysEmitter, ni_psys_mesh_emitter)
NIFLY_RS_ANIMATION(NiPSysCollider, NiObject, ni_psys_collider)
NIFLY_RS_ANIMATION(
    NiPSysSphericalCollider,
    NiPSysCollider,
    ni_psys_spherical_collider
)
NIFLY_RS_ANIMATION(
    NiPSysPlanarCollider,
    NiPSysCollider,
    ni_psys_planar_collider
)
NIFLY_RS_ANIMATION(NiPSysEmitterCtlr, NiPSysModifierCtlr, ni_psys_emitter_ctlr)
NIFLY_RS_ANIMATION(
    BSPSysMultiTargetEmitterCtlr,
    NiPSysEmitterCtlr,
    bs_psys_multi_target_emitter_ctlr
)
NIFLY_RS_ANIMATION(NiParticleModifier, NiObject, ni_particle_modifier)
NIFLY_RS_ANIMATION(NiGravity, NiParticleModifier, ni_gravity)
NIFLY_RS_ANIMATION(
    NiParticleGrowFade,
    NiParticleModifier,
    ni_particle_grow_fade
)
NIFLY_RS_ANIMATION(
    NiParticleColorModifier,
    NiParticleModifier,
    ni_particle_color_modifier
)
NIFLY_RS_ANIMATION(NiParticleRotation, NiParticleModifier, ni_particle_rotation)
NIFLY_RS_ANIMATION(NiParticleBomb, NiParticleModifier, ni_particle_bomb)
NIFLY_RS_ANIMATION(
    NiParticleMeshModifier,
    NiParticleModifier,
    ni_particle_mesh_modifier
)
NIFLY_RS_ANIMATION(NiParticleCollider, NiParticleModifier, ni_particle_collider)
NIFLY_RS_ANIMATION(NiPlanarCollider, NiParticleCollider, ni_planar_collider)
NIFLY_RS_ANIMATION(
    NiSphericalCollider,
    NiParticleCollider,
    ni_spherical_collider
)
NIFLY_RS_ANIMATION(NiEmitterModifier, NiObject, ni_emitter_modifier)
NIFLY_RS_ANIMATION(
    NiParticleSystemController,
    NiTimeController,
    ni_particle_system_controller
)
NIFLY_RS_ANIMATION(
    NiBSPArrayController,
    NiParticleSystemController,
    ni_bsp_array_controller
)

NIFLY_RS_ANIMATION_FIELD(
    NiParticlesData,
    float,
    particleRadius,
    particles_data_particle_radius
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticlesData,
    uint16_t,
    numActive,
    particles_data_active_count
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticlesData,
    bool,
    hasRadii,
    particles_data_has_radii
)

// Morrowind files store the flag as a 32-bit value, not always 1, which is
// kept unless the flag changes.
inline bool particles_data_has_sizes(const nifly::NiParticlesData& obj) {
    return obj.hasSizes;
}

inline void
set_particles_data_has_sizes(nifly::NiParticlesData& obj, bool value) {
    if (static_cast<bool>(obj.hasSizes) != value)
        obj.hasSizes = value;
}
NIFLY_RS_ANIMATION_FIELD(
    NiParticlesData,
    bool,
    hasRotations,
    particles_data_has_rotations
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticlesData,
    bool,
    hasRotationAngles,
    particles_data_has_rotation_angles
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticlesData,
    bool,
    hasRotationAxes,
    particles_data_has_rotation_axes
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticlesData,
    bool,
    hasTextureIndices,
    particles_data_has_texture_indices
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticlesData,
    float,
    aspectRatio,
    particles_data_aspect_ratio
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticlesData,
    uint16_t,
    aspectFlags,
    particles_data_aspect_flags
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticlesData,
    float,
    speedToAspectAspect2,
    particles_data_speed_to_aspect_aspect_2
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticlesData,
    float,
    speedToAspectSpeed1,
    particles_data_speed_to_aspect_speed_1
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticlesData,
    float,
    speedToAspectSpeed2,
    particles_data_speed_to_aspect_speed_2
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysData,
    bool,
    hasRotationSpeeds,
    psys_data_has_rotation_speeds
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysData,
    uint16_t,
    numAddedParticles,
    psys_data_added_particle_count
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysData,
    uint16_t,
    addedParticlesBase,
    psys_data_added_particles_base
)
NIFLY_RS_ANIMATION_VEC3(NiPSysData, unknownVector, psys_data_unknown_vector)
NIFLY_RS_ANIMATION_FIELD(
    NiMeshPSysData,
    uint32_t,
    defaultPoolSize,
    mesh_psys_data_default_pool_size
)
NIFLY_RS_ANIMATION_FIELD(
    NiMeshPSysData,
    bool,
    fillPoolsOnLoad,
    mesh_psys_data_fill_pools_on_load
)
NIFLY_RS_ANIMATION_FIELD(
    BSStripPSysData,
    uint16_t,
    maxPointCount,
    strip_psys_data_max_point_count
)
NIFLY_RS_ANIMATION_FIELD(
    BSStripPSysData,
    uint32_t,
    startCapSize,
    strip_psys_data_start_cap_size
)
NIFLY_RS_ANIMATION_FIELD(
    BSStripPSysData,
    uint32_t,
    endCapSize,
    strip_psys_data_end_cap_size
)
NIFLY_RS_ANIMATION_FIELD(
    BSStripPSysData,
    bool,
    doZPrepass,
    strip_psys_data_do_z_prepass
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticleSystem,
    bool,
    isWorldSpace,
    particle_system_world_space
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticleSystem,
    uint16_t,
    farBegin,
    particle_system_far_begin
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticleSystem,
    uint16_t,
    farEnd,
    particle_system_far_end
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticleSystem,
    uint16_t,
    nearBegin,
    particle_system_near_begin
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticleSystem,
    uint16_t,
    nearEnd,
    particle_system_near_end
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticleSystem,
    int32_t,
    activeMaterial,
    particle_system_active_material
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticleSystem,
    uint8_t,
    defaultMatNeedsUpdate,
    particle_system_default_material_update_flag
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticleSystem,
    uint32_t,
    shaderExtraData,
    particle_system_shader_implementation
)
NIFLY_RS_ANIMATION_FIELD(NiPSysModifier, uint32_t, order, psys_modifier_order)
NIFLY_RS_ANIMATION_FIELD(NiPSysModifier, bool, isActive, psys_modifier_active)
NIFLY_RS_ANIMATION_FIELD(
    BSPSysStripUpdateModifier,
    float,
    updateDeltaTime,
    strip_update_modifier_update_delta_time
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysSpawnModifier,
    uint16_t,
    numSpawnGenerations,
    spawn_modifier_spawn_generations
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysSpawnModifier,
    float,
    percentSpawned,
    spawn_modifier_percent_spawned
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysSpawnModifier,
    uint16_t,
    minSpawned,
    spawn_modifier_min_spawned
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysSpawnModifier,
    uint16_t,
    maxSpawned,
    spawn_modifier_max_spawned
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysSpawnModifier,
    float,
    spawnSpeedVariation,
    spawn_modifier_speed_variation
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysSpawnModifier,
    float,
    spawnDirVariation,
    spawn_modifier_direction_variation
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysSpawnModifier,
    float,
    lifeSpan,
    spawn_modifier_life_span
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysSpawnModifier,
    float,
    lifeSpanVariation,
    spawn_modifier_life_span_variation
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysAgeDeathModifier,
    bool,
    spawnOnDeath,
    age_death_modifier_spawn_on_death
)
NIFLY_RS_ANIMATION_FIELD(
    BSPSysLODModifier,
    float,
    lodBeginDistance,
    lod_modifier_lod_begin_distance
)
NIFLY_RS_ANIMATION_FIELD(
    BSPSysLODModifier,
    float,
    lodEndDistance,
    lod_modifier_lod_end_distance
)
NIFLY_RS_ANIMATION_FIELD(
    BSPSysLODModifier,
    float,
    endEmitScale,
    lod_modifier_end_emit_scale
)
NIFLY_RS_ANIMATION_FIELD(
    BSPSysLODModifier,
    float,
    endSize,
    lod_modifier_end_size
)
NIFLY_RS_ANIMATION_FIELD(
    BSPSysSimpleColorModifier,
    float,
    fadeInPercent,
    simple_color_modifier_fade_in_percent
)
NIFLY_RS_ANIMATION_FIELD(
    BSPSysSimpleColorModifier,
    float,
    fadeOutPercent,
    simple_color_modifier_fade_out_percent
)
NIFLY_RS_ANIMATION_FIELD(
    BSPSysSimpleColorModifier,
    float,
    color1EndPercent,
    simple_color_modifier_color_1_end_percent
)
NIFLY_RS_ANIMATION_FIELD(
    BSPSysSimpleColorModifier,
    float,
    color2StartPercent,
    simple_color_modifier_color_2_start_percent
)
NIFLY_RS_ANIMATION_FIELD(
    BSPSysSimpleColorModifier,
    float,
    color2EndPercent,
    simple_color_modifier_color_2_end_percent
)
NIFLY_RS_ANIMATION_FIELD(
    BSPSysSimpleColorModifier,
    float,
    color3StartPercent,
    simple_color_modifier_color_3_start_percent
)
NIFLY_RS_PARTICLE_COLOR(
    BSPSysSimpleColorModifier,
    color1,
    simple_color_modifier_color_1
)
NIFLY_RS_PARTICLE_COLOR(
    BSPSysSimpleColorModifier,
    color2,
    simple_color_modifier_color_2
)
NIFLY_RS_PARTICLE_COLOR(
    BSPSysSimpleColorModifier,
    color3,
    simple_color_modifier_color_3
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysRotationModifier,
    float,
    initialSpeed,
    rotation_modifier_initial_speed
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysRotationModifier,
    float,
    initialSpeedVariation,
    rotation_modifier_initial_speed_variation
)
NIFLY_RS_PARTICLE_VEC4(
    NiPSysRotationModifier,
    unknownVector,
    rotation_modifier_unknown_vector
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysRotationModifier,
    uint8_t,
    unknownByte,
    rotation_modifier_unknown_byte
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysRotationModifier,
    float,
    initialAngle,
    rotation_modifier_initial_angle
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysRotationModifier,
    float,
    initialAngleVariation,
    rotation_modifier_initial_angle_variation
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysRotationModifier,
    bool,
    randomSpeedSign,
    rotation_modifier_random_speed_sign
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysRotationModifier,
    bool,
    randomInitialAxis,
    rotation_modifier_random_initial_axis
)
NIFLY_RS_ANIMATION_VEC3(
    NiPSysRotationModifier,
    initialAxis,
    rotation_modifier_initial_axis
)
NIFLY_RS_ANIMATION_VEC3(
    NiPSysGravityModifier,
    gravityAxis,
    gravity_modifier_gravity_axis
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysGravityModifier,
    float,
    decay,
    gravity_modifier_decay
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysGravityModifier,
    float,
    strength,
    gravity_modifier_strength
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysGravityModifier,
    uint32_t,
    forceType,
    gravity_modifier_force_type
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysGravityModifier,
    float,
    turbulence,
    gravity_modifier_turbulence
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysGravityModifier,
    float,
    turbulenceScale,
    gravity_modifier_turbulence_scale
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysGravityModifier,
    bool,
    worldAligned,
    gravity_modifier_world_aligned
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysBoundUpdateModifier,
    uint16_t,
    updateSkip,
    bound_update_modifier_update_skip
)
NIFLY_RS_ANIMATION_VEC3(NiPSysDragModifier, dragAxis, drag_modifier_drag_axis)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysDragModifier,
    float,
    percentage,
    drag_modifier_percentage
)
NIFLY_RS_ANIMATION_FIELD(NiPSysDragModifier, float, range, drag_modifier_range)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysDragModifier,
    float,
    rangeFalloff,
    drag_modifier_range_falloff
)
NIFLY_RS_ANIMATION_FIELD(
    BSPSysInheritVelocityModifier,
    float,
    changeToInherit,
    inherit_velocity_modifier_change_to_inherit
)
NIFLY_RS_ANIMATION_FIELD(
    BSPSysInheritVelocityModifier,
    float,
    velocityMult,
    inherit_velocity_modifier_velocity_multiplier
)
NIFLY_RS_ANIMATION_FIELD(
    BSPSysInheritVelocityModifier,
    float,
    velocityVar,
    inherit_velocity_modifier_velocity_variation
)
NIFLY_RS_ANIMATION_FIELD(
    BSPSysSubTexModifier,
    float,
    startFrame,
    sub_tex_modifier_start_frame
)
NIFLY_RS_ANIMATION_FIELD(
    BSPSysSubTexModifier,
    float,
    startFrameVariation,
    sub_tex_modifier_start_frame_variation
)
NIFLY_RS_ANIMATION_FIELD(
    BSPSysSubTexModifier,
    float,
    endFrame,
    sub_tex_modifier_end_frame
)
NIFLY_RS_ANIMATION_FIELD(
    BSPSysSubTexModifier,
    float,
    loopStartFrame,
    sub_tex_modifier_loop_start_frame
)
NIFLY_RS_ANIMATION_FIELD(
    BSPSysSubTexModifier,
    float,
    loopStartFrameVariation,
    sub_tex_modifier_loop_start_frame_variation
)
NIFLY_RS_ANIMATION_FIELD(
    BSPSysSubTexModifier,
    float,
    frameCount,
    sub_tex_modifier_frame_count
)
NIFLY_RS_ANIMATION_FIELD(
    BSPSysSubTexModifier,
    float,
    frameCountVariation,
    sub_tex_modifier_frame_count_variation
)
NIFLY_RS_ANIMATION_VEC3(NiPSysBombModifier, bombAxis, bomb_modifier_bomb_axis)
NIFLY_RS_ANIMATION_FIELD(NiPSysBombModifier, float, decay, bomb_modifier_decay)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysBombModifier,
    float,
    deltaV,
    bomb_modifier_delta_v
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysBombModifier,
    uint32_t,
    decayType,
    bomb_modifier_decay_type
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysBombModifier,
    uint32_t,
    symmetryType,
    bomb_modifier_symmetry_type
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysGrowFadeModifier,
    float,
    growTime,
    grow_fade_modifier_grow_time
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysGrowFadeModifier,
    uint16_t,
    growGeneration,
    grow_fade_modifier_grow_generation
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysGrowFadeModifier,
    float,
    fadeTime,
    grow_fade_modifier_fade_time
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysGrowFadeModifier,
    uint16_t,
    fadeGeneration,
    grow_fade_modifier_fade_generation
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysGrowFadeModifier,
    float,
    baseScale,
    grow_fade_modifier_base_scale
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysFieldModifier,
    float,
    magnitude,
    field_modifier_magnitude
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysFieldModifier,
    float,
    attenuation,
    field_modifier_attenuation
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysFieldModifier,
    bool,
    useMaxDistance,
    field_modifier_use_max_distance
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysFieldModifier,
    float,
    maxDistance,
    field_modifier_max_distance
)
NIFLY_RS_ANIMATION_VEC3(
    NiPSysVortexFieldModifier,
    direction,
    vortex_field_modifier_direction
)
NIFLY_RS_ANIMATION_VEC3(
    NiPSysGravityFieldModifier,
    direction,
    gravity_field_modifier_direction
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysDragFieldModifier,
    bool,
    useDirection,
    drag_field_modifier_use_direction
)
NIFLY_RS_ANIMATION_VEC3(
    NiPSysDragFieldModifier,
    direction,
    drag_field_modifier_direction
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysTurbulenceFieldModifier,
    float,
    frequency,
    turbulence_field_modifier_frequency
)
NIFLY_RS_ANIMATION_VEC3(
    NiPSysAirFieldModifier,
    direction,
    air_field_modifier_direction
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysAirFieldModifier,
    float,
    airFriction,
    air_field_modifier_air_friction
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysAirFieldModifier,
    float,
    inheritVelocity,
    air_field_modifier_inherit_velocity
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysAirFieldModifier,
    bool,
    inheritRotation,
    air_field_modifier_inherit_rotation
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysAirFieldModifier,
    bool,
    componentOnly,
    air_field_modifier_component_only
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysAirFieldModifier,
    bool,
    enableSpread,
    air_field_modifier_enable_spread
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysAirFieldModifier,
    float,
    spread,
    air_field_modifier_spread
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysRadialFieldModifier,
    uint32_t,
    radialType,
    radial_field_modifier_radial_type
)
NIFLY_RS_ANIMATION_FIELD(
    BSWindModifier,
    float,
    strength,
    wind_modifier_strength
)
NIFLY_RS_ANIMATION_VEC3(
    BSPSysRecycleBoundModifier,
    boundOffset,
    recycle_bound_modifier_bound_offset
)
NIFLY_RS_ANIMATION_VEC3(
    BSPSysRecycleBoundModifier,
    boundExtent,
    recycle_bound_modifier_bound_extent
)
NIFLY_RS_ANIMATION_FIELD(
    BSParentVelocityModifier,
    float,
    damping,
    parent_velocity_modifier_damping
)
NIFLY_RS_ANIMATION_FIELD(NiPSysEmitter, float, speed, psys_emitter_speed)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysEmitter,
    float,
    speedVariation,
    psys_emitter_speed_variation
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysEmitter,
    float,
    declination,
    psys_emitter_declination
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysEmitter,
    float,
    declinationVariation,
    psys_emitter_declination_variation
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysEmitter,
    float,
    planarAngle,
    psys_emitter_planar_angle
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysEmitter,
    float,
    planarAngleVariation,
    psys_emitter_planar_angle_variation
)
NIFLY_RS_PARTICLE_COLOR(NiPSysEmitter, color, psys_emitter_initial_color)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysEmitter,
    float,
    radius,
    psys_emitter_initial_radius
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysEmitter,
    float,
    radiusVariation,
    psys_emitter_radius_variation
)
NIFLY_RS_ANIMATION_FIELD(NiPSysEmitter, float, lifeSpan, psys_emitter_life_span)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysEmitter,
    float,
    lifeSpanVariation,
    psys_emitter_life_span_variation
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysSphereEmitter,
    float,
    radius,
    sphere_emitter_radius
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysCylinderEmitter,
    float,
    radius,
    cylinder_emitter_radius
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysCylinderEmitter,
    float,
    height,
    cylinder_emitter_height
)
NIFLY_RS_ANIMATION_FIELD(NiPSysBoxEmitter, float, width, box_emitter_width)
NIFLY_RS_ANIMATION_FIELD(NiPSysBoxEmitter, float, height, box_emitter_height)
NIFLY_RS_ANIMATION_FIELD(NiPSysBoxEmitter, float, depth, box_emitter_depth)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysMeshEmitter,
    uint32_t,
    velocityType,
    mesh_emitter_velocity_type
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysMeshEmitter,
    uint32_t,
    emissionType,
    mesh_emitter_emission_type
)
NIFLY_RS_ANIMATION_VEC3(
    NiPSysMeshEmitter,
    emissionAxis,
    mesh_emitter_emission_axis
)
NIFLY_RS_ANIMATION_FIELD(NiPSysCollider, float, bounce, psys_collider_bounce)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysCollider,
    bool,
    spawnOnCollide,
    psys_collider_spawn_on_collide
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysCollider,
    bool,
    dieOnCollide,
    psys_collider_die_on_collide
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysSphericalCollider,
    float,
    radius,
    spherical_collider_radius
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysPlanarCollider,
    float,
    width,
    planar_collider_width
)
NIFLY_RS_ANIMATION_FIELD(
    NiPSysPlanarCollider,
    float,
    height,
    planar_collider_height
)
NIFLY_RS_ANIMATION_VEC3(NiPSysPlanarCollider, xAxis, planar_collider_x_axis)
NIFLY_RS_ANIMATION_VEC3(NiPSysPlanarCollider, yAxis, planar_collider_y_axis)
NIFLY_RS_ANIMATION_FIELD(
    BSPSysMultiTargetEmitterCtlr,
    uint16_t,
    maxEmitters,
    multi_target_emitter_ctlr_max_emitters
)

inline rust::Slice<const uint8_t> string_bytes(const std::string& s) {
    return {reinterpret_cast<const uint8_t*>(s.data()), s.size()};
}

inline void assign_bytes(std::string& s, rust::Slice<const uint8_t> bytes) {
    s.assign(reinterpret_cast<const char*>(bytes.data()), bytes.size());
}

// Per-particle lists: empty, or one value per vertex.
template<class T>
bool per_particle(
    const nifly::NiGeometryData& d,
    const rust::Slice<const T>& v
) {
    return v.empty() || v.size() == d.GetNumVertices();
}

template<class T>
bool set_per_particle(
    const nifly::NiGeometryData& d,
    std::vector<T>& out,
    rust::Slice<const T> in
) {
    if (!per_particle(d, in))
        return false;
    out.assign(in.begin(), in.end());
    return true;
}

#define NIFLY_RS_PARTICLE_FLOATS(T, field, snake) \
    inline rust::Slice<const float> snake(const nifly::T& d) { \
        return {d.field.data(), d.field.size()}; \
    } \
    inline bool set_##snake(nifly::T& d, rust::Slice<const float> values) { \
        return set_per_particle(d, d.field, values); \
    }

NIFLY_RS_PARTICLE_FLOATS(NiParticlesData, radii, particles_data_radii)
NIFLY_RS_PARTICLE_FLOATS(NiParticlesData, sizes, particles_data_sizes)
NIFLY_RS_PARTICLE_FLOATS(
    NiParticlesData,
    rotationAngles,
    particles_data_rotation_angles
)
NIFLY_RS_PARTICLE_FLOATS(NiPSysData, rotationSpeeds, psys_data_rotation_speeds)

// Rotations as x, y, z, w.
inline rust::Vec<float>
particles_data_rotations(const nifly::NiParticlesData& d) {
    rust::Vec<float> out;
    for (const auto& q : d.rotations)
        for (float f : key_value(q))
            out.push_back(f);
    return out;
}

inline bool set_particles_data_rotations(
    nifly::NiParticlesData& d,
    rust::Slice<const float> xyzw
) {
    size_t n = xyzw.size() / 4;
    if (xyzw.size() % 4 != 0 || (n != 0 && n != d.GetNumVertices()))
        return false;
    d.rotations.resize(n);
    for (size_t i = 0; i < n; i++)
        key_value_from(
            {xyzw[i * 4], xyzw[i * 4 + 1], xyzw[i * 4 + 2], xyzw[i * 4 + 3]},
            d.rotations[i]
        );
    return true;
}

inline rust::Vec<Vec3>
particles_data_rotation_axes(const nifly::NiParticlesData& d) {
    rust::Vec<Vec3> out;
    for (const auto& a : d.rotationAxes)
        out.push_back(havok_vec3_to_ffi(a));
    return out;
}

inline bool set_particles_data_rotation_axes(
    nifly::NiParticlesData& d,
    rust::Slice<const Vec3> axes
) {
    if (!per_particle(d, axes))
        return false;
    d.rotationAxes.clear();
    for (const auto& a : axes)
        d.rotationAxes.push_back(havok_vec3_from_ffi(a));
    return true;
}

// Resizes a list that has values to `n`, padding with zeros.
template<class V>
void resize_if_any(V& v, size_t n) {
    if (!v.empty())
        v.resize(n);
}

// Reaches NiGeometryData's protected vertex count through a derived class.
struct VertexCountAccess: nifly::NiParticlesData {
    static uint16_t& of(nifly::NiGeometryData& d) {
        return d.*(&VertexCountAccess::numVertices);
    }
};

// Sets the vertex (particle) count and resizes every per-vertex list that has
// values to match, padding with zeros.
inline void
set_particles_data_vertex_count(nifly::NiParticlesData& d, uint16_t n) {
    VertexCountAccess::of(d) = n;
    d.numParticles = n;
    resize_if_any(d.vertices, n);
    resize_if_any(d.normals, n);
    resize_if_any(d.tangents, n);
    resize_if_any(d.bitangents, n);
    resize_if_any(d.vertexColors, n);
    for (auto& uvs : d.uvSets)
        resize_if_any(uvs, n);
    resize_if_any(d.radii, n);
    resize_if_any(d.sizes, n);
    resize_if_any(d.rotations, n);
    resize_if_any(d.rotationAngles, n);
    resize_if_any(d.rotationAxes, n);
    if (auto r = dynamic_cast<nifly::NiRotatingParticlesData*>(&d))
        resize_if_any(r->rotations2, n);
    if (auto p = dynamic_cast<nifly::NiPSysData*>(&d)) {
        resize_if_any(p->particleInfo, n);
        resize_if_any(p->rotationSpeeds, n);
    }
}

// Subtexture offsets as four floats each.
inline rust::Vec<float>
particles_data_subtexture_offsets(const nifly::NiParticlesData& d) {
    rust::Vec<float> out;
    for (const auto& v : const_items(d.subtexOffsets)) {
        out.push_back(v.x);
        out.push_back(v.y);
        out.push_back(v.z);
        out.push_back(v.w);
    }
    return out;
}

inline void set_particles_data_subtexture_offsets(
    nifly::NiParticlesData& d,
    rust::Slice<const float> xyzw
) {
    d.subtexOffsets.clear();
    for (size_t i = 0; i + 3 < xyzw.size(); i += 4) {
        nifly::Vector4 v(xyzw[i], xyzw[i + 1], xyzw[i + 2], xyzw[i + 3]);
        d.subtexOffsets.push_back(v);
    }
}

inline ParticleInfoFfi particle_info_to_ffi(const nifly::NiParticleInfo& p) {
    return {
        havok_vec3_to_ffi(p.velocity),
        havok_vec3_to_ffi(p.rotationAxis),
        p.age,
        p.lifeSpan,
        p.lastUpdate,
        p.spawnGeneration,
        p.code
    };
}

inline nifly::NiParticleInfo particle_info_from_ffi(const ParticleInfoFfi& f) {
    nifly::NiParticleInfo p;
    p.velocity = havok_vec3_from_ffi(f.velocity);
    p.rotationAxis = havok_vec3_from_ffi(f.rotation_axis);
    p.age = f.age;
    p.lifeSpan = f.life_span;
    p.lastUpdate = f.last_update;
    p.spawnGeneration = f.spawn_generation;
    p.code = f.code;
    return p;
}

inline rust::Vec<ParticleInfoFfi>
psys_data_particle_info(const nifly::NiPSysData& d) {
    rust::Vec<ParticleInfoFfi> out;
    for (const auto& p : d.particleInfo)
        out.push_back(particle_info_to_ffi(p));
    return out;
}

inline bool set_psys_data_particle_info(
    nifly::NiPSysData& d,
    rust::Slice<const ParticleInfoFfi> info
) {
    if (!per_particle(d, info))
        return false;
    d.particleInfo.clear();
    for (const auto& f : info)
        d.particleInfo.push_back(particle_info_from_ffi(f));
    return true;
}

inline rust::Vec<uint32_t>
mesh_psys_data_generation_pool_sizes(const nifly::NiMeshPSysData& d) {
    rust::Vec<uint32_t> out;
    for (uint32_t v : const_items(d.generationPoolSize))
        out.push_back(v);
    return out;
}

inline void set_mesh_psys_data_generation_pool_sizes(
    nifly::NiMeshPSysData& d,
    rust::Slice<const uint32_t> sizes
) {
    d.generationPoolSize.clear();
    for (uint32_t v : sizes)
        d.generationPoolSize.push_back(v);
}

// NiPSysEmitterCtlrData.
NIFLY_RS_KEY_GROUP(NiPSysEmitterCtlrData, floatKeys, emitter_ctlr_data_keys)

inline rust::Vec<KeyFfi>
emitter_ctlr_data_visibility_keys(const nifly::NiPSysEmitterCtlrData& d) {
    rust::Vec<KeyFfi> out;
    for (const auto& k : const_items(d.visibilityKeys))
        out.push_back(key_to_ffi(k));
    return out;
}

inline void set_emitter_ctlr_data_visibility_keys(
    nifly::NiPSysEmitterCtlrData& d,
    rust::Slice<const KeyFfi> keys
) {
    d.visibilityKeys.clear();
    for (const auto& k : keys) {
        auto key = key_from_ffi<uint8_t>(k, nifly::NO_INTERP);
        d.visibilityKeys.push_back(key);
    }
}

// NiParticleSystem.
inline std::array<uint8_t, 8>
particle_system_vertex_raw_flags(const nifly::NiParticleSystem& s) {
    return {
        s.vertFlags1,
        s.vertFlags2,
        s.vertFlags3,
        s.vertFlags4,
        s.vertFlags5,
        s.vertFlags6,
        s.vertFlags7,
        s.vertFlags8
    };
}

inline void set_particle_system_vertex_raw_flags(
    nifly::NiParticleSystem& s,
    std::array<uint8_t, 8> f
) {
    s.vertFlags1 = f[0];
    s.vertFlags2 = f[1];
    s.vertFlags3 = f[2];
    s.vertFlags4 = f[3];
    s.vertFlags5 = f[4];
    s.vertFlags6 = f[5];
    s.vertFlags7 = f[6];
    s.vertFlags8 = f[7];
}

inline BoundingSphere particle_system_bounds(const nifly::NiParticleSystem& s) {
    return {havok_vec3_to_ffi(s.bounds.center), s.bounds.radius};
}

inline void
set_particle_system_bounds(nifly::NiParticleSystem& s, BoundingSphere b) {
    s.bounds.center = havok_vec3_from_ffi(b.center);
    s.bounds.radius = b.radius;
}

inline std::array<float, 6>
particle_system_bound_min_max(const nifly::NiParticleSystem& s) {
    std::array<float, 6> out;
    std::copy(std::begin(s.boundMinMax), std::end(s.boundMinMax), out.begin());
    return out;
}

inline void set_particle_system_bound_min_max(
    nifly::NiParticleSystem& s,
    std::array<float, 6> v
) {
    std::copy(v.begin(), v.end(), std::begin(s.boundMinMax));
}

inline size_t particle_system_material_count(const nifly::NiParticleSystem& s) {
    return s.materialNames.size();
}

// Empty if `i` is out of range.
inline rust::Slice<const uint8_t>
particle_system_material_name(const nifly::NiParticleSystem& s, size_t i) {
    if (i >= s.materialNames.size())
        return {};
    return string_bytes(s.materialNames.data()[i].get());
}

inline rust::Slice<const uint32_t>
particle_system_material_extra_data(const nifly::NiParticleSystem& s) {
    return {s.materialExtraData.data(), s.materialExtraData.size()};
}

// Replaces the materials: names joined in `bytes`, one extra data value each.
inline void set_particle_system_materials(
    nifly::NiParticleSystem& s,
    rust::Slice<const uint8_t> bytes,
    rust::Slice<const size_t> lengths,
    rust::Slice<const uint32_t> extra_data
) {
    s.materialNames.clear();
    s.materialExtraData.clear();
    size_t at = 0;
    for (size_t i = 0; i < lengths.size(); i++) {
        nifly::NiStringRef name(
            std::string(
                reinterpret_cast<const char*>(bytes.data()) + at,
                lengths[i]
            )
        );
        at += lengths[i];
        s.materialNames.push_back(name);
        uint32_t extra = extra_data[i];
        s.materialExtraData.push_back(extra);
    }
}

inline bool particle_system_has_shader(const nifly::NiParticleSystem& s) {
    return s.hasShader;
}

inline rust::Slice<const uint8_t>
particle_system_shader_name(const nifly::NiParticleSystem& s) {
    return string_bytes(s.shaderName.get());
}

inline void set_particle_system_shader(
    nifly::NiParticleSystem& s,
    bool has,
    rust::Slice<const uint8_t> name
) {
    s.hasShader = has;
    assign_bytes(s.shaderName.get(), name);
}

// Names of modifiers and the modifiers controllers animate.
inline rust::Slice<const uint8_t>
psys_modifier_name(const nifly::NiPSysModifier& m) {
    return string_bytes(m.name.get());
}

inline void set_psys_modifier_name(
    nifly::NiPSysModifier& m,
    rust::Slice<const uint8_t> name
) {
    assign_bytes(m.name.get(), name);
}

inline rust::Slice<const uint8_t>
psys_modifier_ctlr_modifier_name(const nifly::NiPSysModifierCtlr& c) {
    return string_bytes(c.modifierName.get());
}

inline void set_psys_modifier_ctlr_modifier_name(
    nifly::NiPSysModifierCtlr& c,
    rust::Slice<const uint8_t> name
) {
    assign_bytes(c.modifierName.get(), name);
}

inline std::array<uint16_t, 26> simple_color_modifier_unknown_shorts(
    const nifly::BSPSysSimpleColorModifier& m
) {
    std::array<uint16_t, 26> out;
    std::copy(
        std::begin(m.unknownShorts),
        std::end(m.unknownShorts),
        out.begin()
    );
    return out;
}

inline void set_simple_color_modifier_unknown_shorts(
    nifly::BSPSysSimpleColorModifier& m,
    std::array<uint16_t, 26> v
) {
    std::copy(v.begin(), v.end(), std::begin(m.unknownShorts));
}

inline rust::Vec<float>
scale_modifier_scales(const nifly::BSPSysScaleModifier& m) {
    rust::Vec<float> out;
    for (float f : const_items(m.floats))
        out.push_back(f);
    return out;
}

inline void set_scale_modifier_scales(
    nifly::BSPSysScaleModifier& m,
    rust::Slice<const float> scales
) {
    m.floats.clear();
    for (float f : scales)
        m.floats.push_back(f);
}

// Replacements for blocks that are misread in some versions. Each reads and
// writes like its base from the version that stores all its fields, and skips
// the fields older files don't store. The factories load the blocks as these,
// the add shims create them, and clones stay replacements.

// NiPSysEmitter reads and writes radiusVariation in every version, but files
// store it only from 10.4.0.1. Base is a concrete emitter type.
template<class Base>
class Emitter final: public Base {
  public:
    void Get(nifly::NiIStream& stream) override {
        if (stream.GetVersion().File() >= nifly::NiFileVersion::V10_4_0_1) {
            Base::Get(stream);
            return;
        }
        nifly::NiPSysModifier::Get(stream);
        nifly::NiStreamReversible s(
            &stream,
            nullptr,
            nifly::NiStreamReversible::Mode::Reading
        );
        sync_old(s);
    }

    void Put(nifly::NiOStream& stream) override {
        if (stream.GetVersion().File() >= nifly::NiFileVersion::V10_4_0_1) {
            Base::Put(stream);
            return;
        }
        nifly::NiPSysModifier::Put(stream);
        nifly::NiStreamReversible s(
            nullptr,
            &stream,
            nifly::NiStreamReversible::Mode::Writing
        );
        sync_old(s);
    }

  private:
    // The emitter fields without radiusVariation, then the subclass fields.
    void sync_old(nifly::NiStreamReversible& s) {
        s.Sync(this->speed);
        s.Sync(this->speedVariation);
        s.Sync(this->declination);
        s.Sync(this->declinationVariation);
        s.Sync(this->planarAngle);
        s.Sync(this->planarAngleVariation);
        s.Sync(this->color);
        s.Sync(this->radius);
        s.Sync(this->lifeSpan);
        s.Sync(this->lifeSpanVariation);
        if constexpr (std::is_base_of_v<nifly::NiPSysVolumeEmitter, Base>)
            this->nifly::NiPSysVolumeEmitter::Sync(s);
        if constexpr (std::is_same_v<Base, nifly::NiPSysSphereEmitter>)
            this->nifly::NiPSysSphereEmitter::Sync(s);
        if constexpr (std::is_same_v<Base, nifly::NiPSysCylinderEmitter>)
            this->nifly::NiPSysCylinderEmitter::Sync(s);
        if constexpr (std::is_same_v<Base, nifly::NiPSysBoxEmitter>)
            this->nifly::NiPSysBoxEmitter::Sync(s);
        if constexpr (std::is_same_v<Base, nifly::NiPSysMeshEmitter>)
            this->nifly::NiPSysMeshEmitter::Sync(s);
    }

    Emitter* Clone_impl() const override {
        return new Emitter(*this);
    }
};

// NiPSysRotationModifier reads and writes the speed variation, angle, angle
// variation and random sign in every version, but files store them only from
// 20.0.0.2.
class RotationModifier final: public nifly::NiPSysRotationModifier {
  public:
    void Get(nifly::NiIStream& stream) override {
        if (stream.GetVersion().File() >= nifly::NiFileVersion::V20_0_0_2) {
            nifly::NiPSysRotationModifier::Get(stream);
            return;
        }
        nifly::NiPSysModifier::Get(stream);
        nifly::NiStreamReversible s(
            &stream,
            nullptr,
            nifly::NiStreamReversible::Mode::Reading
        );
        sync_old(s);
    }

    void Put(nifly::NiOStream& stream) override {
        if (stream.GetVersion().File() >= nifly::NiFileVersion::V20_0_0_2) {
            nifly::NiPSysRotationModifier::Put(stream);
            return;
        }
        nifly::NiPSysModifier::Put(stream);
        nifly::NiStreamReversible s(
            nullptr,
            &stream,
            nifly::NiStreamReversible::Mode::Writing
        );
        sync_old(s);
    }

  private:
    void sync_old(nifly::NiStreamReversible& s) {
        s.Sync(initialSpeed);
        s.Sync(randomInitialAxis);
        s.Sync(initialAxis);
    }

    RotationModifier* Clone_impl() const override {
        return new RotationModifier(*this);
    }
};

// NiMeshPSysData reads and writes its pool fields in every version, but files
// store them only from 10.2.0.0.
class MeshPSysData final: public nifly::NiMeshPSysData {
  public:
    void Get(nifly::NiIStream& stream) override {
        if (stream.GetVersion().File() >= nifly::NiFileVersion::V10_2_0_0) {
            nifly::NiMeshPSysData::Get(stream);
            return;
        }
        nifly::NiPSysData::Get(stream);
        nifly::NiStreamReversible s(
            &stream,
            nullptr,
            nifly::NiStreamReversible::Mode::Reading
        );
        nodeRef.Sync(s);
    }

    void Put(nifly::NiOStream& stream) override {
        if (stream.GetVersion().File() >= nifly::NiFileVersion::V10_2_0_0) {
            nifly::NiMeshPSysData::Put(stream);
            return;
        }
        nifly::NiPSysData::Put(stream);
        nifly::NiStreamReversible s(
            nullptr,
            &stream,
            nifly::NiStreamReversible::Mode::Writing
        );
        nodeRef.Sync(s);
    }

  private:
    MeshPSysData* Clone_impl() const override {
        return new MeshPSysData(*this);
    }
};

// Loads every block of a type as the replacement T.
template<class T>
class ReplacementFactory final: public nifly::NiFactory {
  public:
    std::unique_ptr<nifly::NiObject> Create() override {
        return std::make_unique<T>();
    }

    std::unique_ptr<nifly::NiObject> Load(nifly::NiIStream& stream) override {
        auto b = std::make_unique<T>();
        b->Get(stream);
        return b;
    }
};

// Old particles (Morrowind and Oblivion).

NIFLY_RS_ANIMATION_FIELD(NiGravity, float, decay, gravity_decay)
NIFLY_RS_ANIMATION_FIELD(NiGravity, float, force, gravity_force)
NIFLY_RS_ANIMATION_FIELD(NiGravity, uint32_t, fieldType, gravity_field_type)
NIFLY_RS_ANIMATION_VEC3(NiGravity, position, gravity_position)
NIFLY_RS_ANIMATION_VEC3(NiGravity, direction, gravity_direction)
NIFLY_RS_ANIMATION_FIELD(NiParticleGrowFade, float, grow, grow_fade_grow)
NIFLY_RS_ANIMATION_FIELD(NiParticleGrowFade, float, fade, grow_fade_fade)
NIFLY_RS_PARTICLE_BYTE_BOOL(
    NiParticleRotation,
    randomInitialAxis,
    particle_rotation_random_initial_axis
)
NIFLY_RS_ANIMATION_VEC3(
    NiParticleRotation,
    initialAxis,
    particle_rotation_initial_axis
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticleRotation,
    float,
    rotationSpeed,
    particle_rotation_rotation_speed
)
NIFLY_RS_ANIMATION_FIELD(NiParticleBomb, float, decay, particle_bomb_decay)
NIFLY_RS_ANIMATION_FIELD(
    NiParticleBomb,
    float,
    duration,
    particle_bomb_duration
)
NIFLY_RS_ANIMATION_FIELD(NiParticleBomb, float, deltaV, particle_bomb_delta_v)
NIFLY_RS_ANIMATION_FIELD(NiParticleBomb, float, start, particle_bomb_start)
NIFLY_RS_ANIMATION_FIELD(
    NiParticleBomb,
    uint32_t,
    decayType,
    particle_bomb_decay_type
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticleBomb,
    uint32_t,
    symmetryType,
    particle_bomb_symmetry_type
)
NIFLY_RS_ANIMATION_VEC3(NiParticleBomb, position, particle_bomb_position)
NIFLY_RS_ANIMATION_VEC3(NiParticleBomb, direction, particle_bomb_direction)
NIFLY_RS_ANIMATION_FIELD(
    NiParticleCollider,
    float,
    bounce,
    particle_collider_bounce
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticleCollider,
    bool,
    spawnOnCollide,
    particle_collider_spawn_on_collide
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticleCollider,
    bool,
    dieOnCollide,
    particle_collider_die_on_collide
)
NIFLY_RS_ANIMATION_FIELD(
    NiPlanarCollider,
    float,
    width,
    old_planar_collider_width
)
NIFLY_RS_ANIMATION_FIELD(
    NiPlanarCollider,
    float,
    height,
    old_planar_collider_height
)
NIFLY_RS_ANIMATION_VEC3(
    NiPlanarCollider,
    position,
    old_planar_collider_position
)
NIFLY_RS_ANIMATION_VEC3(NiPlanarCollider, xVector, old_planar_collider_x_axis)
NIFLY_RS_ANIMATION_VEC3(NiPlanarCollider, yVector, old_planar_collider_y_axis)
NIFLY_RS_ANIMATION_FIELD(
    NiSphericalCollider,
    float,
    radius,
    old_spherical_collider_radius
)
NIFLY_RS_ANIMATION_VEC3(
    NiSphericalCollider,
    position,
    old_spherical_collider_position
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticleSystemController,
    float,
    speed,
    particle_system_ctlr_speed
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticleSystemController,
    float,
    speedVariation,
    particle_system_ctlr_speed_variation
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticleSystemController,
    float,
    declination,
    particle_system_ctlr_declination
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticleSystemController,
    float,
    declinationVariation,
    particle_system_ctlr_declination_variation
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticleSystemController,
    float,
    planarAngle,
    particle_system_ctlr_planar_angle
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticleSystemController,
    float,
    planarAngleVariation,
    particle_system_ctlr_planar_angle_variation
)
NIFLY_RS_ANIMATION_VEC3(
    NiParticleSystemController,
    initialNormal,
    particle_system_ctlr_initial_normal
)
NIFLY_RS_PARTICLE_COLOR(
    NiParticleSystemController,
    initialColor,
    particle_system_ctlr_initial_color
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticleSystemController,
    float,
    initialSize,
    particle_system_ctlr_initial_size
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticleSystemController,
    float,
    emitStartTime,
    particle_system_ctlr_emit_start_time
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticleSystemController,
    float,
    emitStopTime,
    particle_system_ctlr_emit_stop_time
)
NIFLY_RS_PARTICLE_BYTE_BOOL(
    NiParticleSystemController,
    resetParticleSystem,
    particle_system_ctlr_reset_particle_system
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticleSystemController,
    float,
    birthRate,
    particle_system_ctlr_birth_rate
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticleSystemController,
    float,
    lifetime,
    particle_system_ctlr_lifetime
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticleSystemController,
    float,
    lifetimeVariation,
    particle_system_ctlr_lifetime_variation
)
NIFLY_RS_PARTICLE_BYTE_BOOL(
    NiParticleSystemController,
    useBirthRate,
    particle_system_ctlr_use_birth_rate
)
NIFLY_RS_PARTICLE_BYTE_BOOL(
    NiParticleSystemController,
    spawnOnDeath,
    particle_system_ctlr_spawn_on_death
)
NIFLY_RS_ANIMATION_VEC3(
    NiParticleSystemController,
    emitterDimensions,
    particle_system_ctlr_emitter_dimensions
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticleSystemController,
    uint16_t,
    numSpawnGenerations,
    particle_system_ctlr_spawn_generations
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticleSystemController,
    float,
    percentageSpawned,
    particle_system_ctlr_percent_spawned
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticleSystemController,
    uint16_t,
    spawnMultiplier,
    particle_system_ctlr_spawn_multiplier
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticleSystemController,
    float,
    spawnSpeedChaos,
    particle_system_ctlr_spawn_speed_chaos
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticleSystemController,
    float,
    spawnDirChaos,
    particle_system_ctlr_spawn_direction_chaos
)
NIFLY_RS_ANIMATION_FIELD(
    NiParticleSystemController,
    uint16_t,
    numValid,
    particle_system_ctlr_active_count
)
NIFLY_RS_PARTICLE_BYTE_BOOL(
    NiParticleSystemController,
    staticTargetBound,
    particle_system_ctlr_static_target_bound
)
NIFLY_RS_ANIMATION_VEC3(
    NiPlanarCollider,
    plane.normal,
    old_planar_collider_plane_normal
)
NIFLY_RS_ANIMATION_FIELD(
    NiPlanarCollider,
    float,
    plane.constant,
    old_planar_collider_plane_constant
)

// Morrowind files may store the flag as a 32-bit value other than 1, which is
// kept unless the flag changes.
inline bool rotating_particles_data_has_rotations_2(
    const nifly::NiRotatingParticlesData& d
) {
    return d.hasRotations2;
}

inline void set_rotating_particles_data_has_rotations_2(
    nifly::NiRotatingParticlesData& d,
    bool value
) {
    if (static_cast<bool>(d.hasRotations2) != value)
        d.hasRotations2 = value;
}

// Rotations as x, y, z, w.
inline rust::Vec<float>
rotating_particles_data_rotations_2(const nifly::NiRotatingParticlesData& d) {
    rust::Vec<float> out;
    for (const auto& q : d.rotations2)
        for (float f : key_value(q))
            out.push_back(f);
    return out;
}

inline bool set_rotating_particles_data_rotations_2(
    nifly::NiRotatingParticlesData& d,
    rust::Slice<const float> xyzw
) {
    size_t n = xyzw.size() / 4;
    if (xyzw.size() % 4 != 0 || (n != 0 && n != d.GetNumVertices()))
        return false;
    d.rotations2.resize(n);
    for (size_t i = 0; i < n; i++)
        key_value_from(
            {xyzw[i * 4], xyzw[i * 4 + 1], xyzw[i * 4 + 2], xyzw[i * 4 + 3]},
            d.rotations2[i]
        );
    return true;
}

inline rust::Vec<ParticleInfoFfi>
particle_system_ctlr_particles(const nifly::NiParticleSystemController& c) {
    rust::Vec<ParticleInfoFfi> out;
    for (const auto& p : c.particles)
        out.push_back(particle_info_to_ffi(p));
    return out;
}

// False, changing nothing, if there are more than a 16-bit count holds.
inline bool set_particle_system_ctlr_particles(
    nifly::NiParticleSystemController& c,
    rust::Slice<const ParticleInfoFfi> info
) {
    if (info.size() > UINT16_MAX)
        return false;
    c.particles.clear();
    for (const auto& f : info)
        c.particles.push_back(particle_info_from_ffi(f));
    return true;
}

// Adds a new old particle shape of type T with new data of type Data, named
// `name`, with `transform` (to parent), as the last child of the node at
// `parent`. NIF_NPOS, adding nothing, if `parent` isn't a node.
template<class T, class Data>
uint32_t add_old_particles(
    nifly::NifFile& file,
    uint32_t parent_index,
    rust::Slice<const uint8_t> name,
    Transform transform
) {
    auto& hdr = file.GetHeader();
    auto parent = hdr.GetBlock<nifly::NiNode>(parent_index);
    if (!parent)
        return nifly::NIF_NPOS;
    uint32_t data = hdr.AddBlock(std::make_unique<Data>());
    auto shape = std::make_unique<T>();
    assign_bytes(shape->name.get(), name);
    shape->SetTransformToParent(from_ffi(transform));
    shape->DataRef()->index = data;
    uint32_t id = hdr.AddBlock(std::move(shape));
    parent->childRefs.AddBlockRef(id);
    return id;
}

#define NIFLY_RS_ADD_OLD_PARTICLES(T, Data, snake) \
    inline uint32_t add_##snake( \
        nifly::NifFile& file, \
        uint32_t parent, \
        rust::Slice<const uint8_t> name, \
        Transform transform \
    ) { \
        return add_old_particles<nifly::T, nifly::Data>( \
            file, \
            parent, \
            name, \
            transform \
        ); \
    }

NIFLY_RS_ADD_OLD_PARTICLES(NiParticles, NiParticlesData, ni_particles)
NIFLY_RS_ADD_OLD_PARTICLES(
    NiAutoNormalParticles,
    NiAutoNormalParticlesData,
    ni_auto_normal_particles
)
NIFLY_RS_ADD_OLD_PARTICLES(
    NiRotatingParticles,
    NiRotatingParticlesData,
    ni_rotating_particles
)
NIFLY_RS_ADD_OLD_PARTICLES(
    NiParticleMeshes,
    NiParticleMeshesData,
    ni_particle_meshes
)
#undef NIFLY_RS_ADD_OLD_PARTICLES

// The ref at the end of a chain of `Link`s through next modifier refs,
// starting at `head`. A ref that isn't a `Link`, or that repeats one already
// passed, ends the chain.
template<class Link>
nifly::NiRef*
modifier_chain_end(const nifly::NiHeader& hdr, nifly::NiRef* head) {
    std::set<uint32_t> seen;
    nifly::NiRef* link = head;
    while (auto m = hdr.GetBlock<Link>(link->index)) {
        if (!seen.insert(link->index).second)
            break;
        link = &m->nextModifierRef;
    }
    return link;
}

// Adds a new T to the end of a chain of the NiParticleSystemController at
// `controller`, pointing back at it: emitter modifiers to the emitter
// modifier chain, colliders to the collider chain, and other modifiers to the
// particle modifier chain. A ref in the chain that isn't a modifier of the
// chain's kind, or that repeats one, ends the chain and is replaced. NIF_NPOS,
// adding nothing, if `controller` isn't one.
template<class T>
uint32_t
add_particle_modifier(nifly::NifFile& file, uint32_t controller_index) {
    using namespace nifly;
    auto& hdr = file.GetHeader();
    auto controller =
        hdr.GetBlock<NiParticleSystemController>(controller_index);
    if (!controller)
        return NIF_NPOS;
    auto modifier = std::make_unique<T>();
    modifier->controllerRef.index = controller_index;
    uint32_t id = hdr.AddBlock(std::move(modifier));
    NiRef* link;
    if constexpr (std::is_same_v<T, NiEmitterModifier>)
        link = modifier_chain_end<NiEmitterModifier>(
            hdr,
            &controller->emitterModifierRef
        );
    else if constexpr (std::is_base_of_v<NiParticleCollider, T>)
        link = modifier_chain_end<NiParticleModifier>(
            hdr,
            &controller->particleColliderRef
        );
    else
        link = modifier_chain_end<NiParticleModifier>(
            hdr,
            &controller->particleModifierRef
        );
    link->index = id;
    return id;
}

#define NIFLY_RS_ADD_PARTICLE_MODIFIER(T, snake) \
    inline uint32_t add_##snake(nifly::NifFile& file, uint32_t controller) { \
        return add_particle_modifier<nifly::T>(file, controller); \
    }

NIFLY_RS_ADD_PARTICLE_MODIFIER(NiGravity, ni_gravity)
NIFLY_RS_ADD_PARTICLE_MODIFIER(NiParticleGrowFade, ni_particle_grow_fade)
NIFLY_RS_ADD_PARTICLE_MODIFIER(
    NiParticleColorModifier,
    ni_particle_color_modifier
)
NIFLY_RS_ADD_PARTICLE_MODIFIER(NiParticleRotation, ni_particle_rotation)
NIFLY_RS_ADD_PARTICLE_MODIFIER(NiParticleBomb, ni_particle_bomb)
NIFLY_RS_ADD_PARTICLE_MODIFIER(
    NiParticleMeshModifier,
    ni_particle_mesh_modifier
)
NIFLY_RS_ADD_PARTICLE_MODIFIER(NiPlanarCollider, ni_planar_collider)
NIFLY_RS_ADD_PARTICLE_MODIFIER(NiSphericalCollider, ni_spherical_collider)
NIFLY_RS_ADD_PARTICLE_MODIFIER(NiEmitterModifier, ni_emitter_modifier)
#undef NIFLY_RS_ADD_PARTICLE_MODIFIER

NIFLY_RS_ADD_CONTROLLER(
    NiParticleSystemController,
    ni_particle_system_controller
)
NIFLY_RS_ADD_CONTROLLER(NiBSPArrayController, ni_bsp_array_controller)

// Particle systems.

// Adds a new particle system of type T with new data of type Data, named
// `name`, with `transform` (to parent), as the last child of the node at
// `parent`. New systems are in world space. NIF_NPOS, adding nothing, if
// `parent` isn't a node.
template<class T, class Data>
uint32_t add_particle_system(
    nifly::NifFile& file,
    uint32_t parent_index,
    rust::Slice<const uint8_t> name,
    Transform transform
) {
    auto& hdr = file.GetHeader();
    auto parent = hdr.GetBlock<nifly::NiNode>(parent_index);
    if (!parent)
        return nifly::NIF_NPOS;
    uint32_t data = hdr.AddBlock(std::make_unique<Data>());
    auto system = std::make_unique<T>();
    assign_bytes(system->name.get(), name);
    system->SetTransformToParent(from_ffi(transform));
    system->isWorldSpace = true;
    system->dataRef.index = data;
    system->psysDataRef.index = data;
    uint32_t id = hdr.AddBlock(std::move(system));
    parent->childRefs.AddBlockRef(id);
    return id;
}

inline uint32_t add_ni_particle_system(
    nifly::NifFile& file,
    uint32_t parent,
    rust::Slice<const uint8_t> name,
    Transform transform
) {
    return add_particle_system<nifly::NiParticleSystem, nifly::NiPSysData>(
        file,
        parent,
        name,
        transform
    );
}

inline uint32_t add_ni_mesh_particle_system(
    nifly::NifFile& file,
    uint32_t parent,
    rust::Slice<const uint8_t> name,
    Transform transform
) {
    return add_particle_system<nifly::NiMeshParticleSystem, MeshPSysData>(
        file,
        parent,
        name,
        transform
    );
}

inline uint32_t add_bs_strip_particle_system(
    nifly::NifFile& file,
    uint32_t parent,
    rust::Slice<const uint8_t> name,
    Transform transform
) {
    return add_particle_system<
        nifly::BSStripParticleSystem,
        nifly::BSStripPSysData>(file, parent, name, transform);
}

// Adds a new modifier of type Make, named `name` and active, to the end of the
// modifiers of the NiParticleSystem at `system`, targeting it. NIF_NPOS,
// adding nothing, if `system` isn't one.
template<class Make>
uint32_t add_modifier(
    nifly::NifFile& file,
    uint32_t system_index,
    rust::Slice<const uint8_t> name
) {
    auto& hdr = file.GetHeader();
    auto system = hdr.GetBlock<nifly::NiParticleSystem>(system_index);
    if (!system)
        return nifly::NIF_NPOS;
    auto modifier = std::make_unique<Make>();
    assign_bytes(modifier->name.get(), name);
    modifier->targetRef.index = system_index;
    modifier->isActive = true;
    uint32_t id = hdr.AddBlock(std::move(modifier));
    system->modifierRefs.AddBlockRef(id);
    return id;
}

#define NIFLY_RS_ADD_MODIFIER_AS(Make, snake) \
    inline uint32_t add_##snake( \
        nifly::NifFile& file, \
        uint32_t system, \
        rust::Slice<const uint8_t> name \
    ) { \
        return add_modifier<Make>(file, system, name); \
    }
#define NIFLY_RS_ADD_MODIFIER(T, snake) \
    NIFLY_RS_ADD_MODIFIER_AS(nifly::T, snake)

NIFLY_RS_ADD_MODIFIER(BSPSysStripUpdateModifier, bs_psys_strip_update_modifier)
NIFLY_RS_ADD_MODIFIER(NiPSysSpawnModifier, ni_psys_spawn_modifier)
NIFLY_RS_ADD_MODIFIER(NiPSysAgeDeathModifier, ni_psys_age_death_modifier)
NIFLY_RS_ADD_MODIFIER(BSPSysLODModifier, bs_psys_lod_modifier)
NIFLY_RS_ADD_MODIFIER(BSPSysSimpleColorModifier, bs_psys_simple_color_modifier)
NIFLY_RS_ADD_MODIFIER_AS(RotationModifier, ni_psys_rotation_modifier)
NIFLY_RS_ADD_MODIFIER(BSPSysScaleModifier, bs_psys_scale_modifier)
NIFLY_RS_ADD_MODIFIER(NiPSysGravityModifier, ni_psys_gravity_modifier)
NIFLY_RS_ADD_MODIFIER(NiPSysPositionModifier, ni_psys_position_modifier)
NIFLY_RS_ADD_MODIFIER(NiPSysBoundUpdateModifier, ni_psys_bound_update_modifier)
NIFLY_RS_ADD_MODIFIER(NiPSysDragModifier, ni_psys_drag_modifier)
NIFLY_RS_ADD_MODIFIER(
    BSPSysInheritVelocityModifier,
    bs_psys_inherit_velocity_modifier
)
NIFLY_RS_ADD_MODIFIER(BSPSysSubTexModifier, bs_psys_sub_tex_modifier)
NIFLY_RS_ADD_MODIFIER(NiPSysBombModifier, ni_psys_bomb_modifier)
NIFLY_RS_ADD_MODIFIER(NiPSysColorModifier, ni_psys_color_modifier)
NIFLY_RS_ADD_MODIFIER(NiPSysGrowFadeModifier, ni_psys_grow_fade_modifier)
NIFLY_RS_ADD_MODIFIER(NiPSysMeshUpdateModifier, ni_psys_mesh_update_modifier)
NIFLY_RS_ADD_MODIFIER(NiPSysVortexFieldModifier, ni_psys_vortex_field_modifier)
NIFLY_RS_ADD_MODIFIER(
    NiPSysGravityFieldModifier,
    ni_psys_gravity_field_modifier
)
NIFLY_RS_ADD_MODIFIER(NiPSysDragFieldModifier, ni_psys_drag_field_modifier)
NIFLY_RS_ADD_MODIFIER(
    NiPSysTurbulenceFieldModifier,
    ni_psys_turbulence_field_modifier
)
NIFLY_RS_ADD_MODIFIER(NiPSysAirFieldModifier, ni_psys_air_field_modifier)
NIFLY_RS_ADD_MODIFIER(NiPSysRadialFieldModifier, ni_psys_radial_field_modifier)
NIFLY_RS_ADD_MODIFIER(BSWindModifier, bs_wind_modifier)
NIFLY_RS_ADD_MODIFIER(
    BSPSysRecycleBoundModifier,
    bs_psys_recycle_bound_modifier
)
NIFLY_RS_ADD_MODIFIER(BSPSysHavokUpdateModifier, bs_psys_havok_update_modifier)
NIFLY_RS_ADD_MODIFIER(BSParentVelocityModifier, bs_parent_velocity_modifier)
NIFLY_RS_ADD_MODIFIER(NiPSysColliderManager, ni_psys_collider_manager)
NIFLY_RS_ADD_MODIFIER_AS(
    Emitter<nifly::NiPSysSphereEmitter>,
    ni_psys_sphere_emitter
)
NIFLY_RS_ADD_MODIFIER_AS(
    Emitter<nifly::NiPSysCylinderEmitter>,
    ni_psys_cylinder_emitter
)
NIFLY_RS_ADD_MODIFIER_AS(Emitter<nifly::NiPSysBoxEmitter>, ni_psys_box_emitter)
NIFLY_RS_ADD_MODIFIER_AS(
    Emitter<nifly::BSPSysArrayEmitter>,
    bs_psys_array_emitter
)
NIFLY_RS_ADD_MODIFIER_AS(
    Emitter<nifly::NiPSysMeshEmitter>,
    ni_psys_mesh_emitter
)
#undef NIFLY_RS_ADD_MODIFIER
#undef NIFLY_RS_ADD_MODIFIER_AS

// Adds a new collider of type T to the end of the collider chain of the
// NiPSysColliderManager at `manager`, pointing back at it. A ref that isn't a
// collider, or that repeats one already passed, ends the chain and is
// replaced. NIF_NPOS, adding nothing, if `manager` isn't one.
template<class T>
uint32_t add_collider(nifly::NifFile& file, uint32_t manager_index) {
    using namespace nifly;
    auto& hdr = file.GetHeader();
    auto manager = hdr.GetBlock<NiPSysColliderManager>(manager_index);
    if (!manager)
        return NIF_NPOS;
    auto collider = std::make_unique<T>();
    collider->managerRef.index = manager_index;
    uint32_t id = hdr.AddBlock(std::move(collider));
    NiRef* link = &manager->colliderRef;
    std::set<uint32_t> seen;
    while (auto c = hdr.GetBlock<NiPSysCollider>(link->index)) {
        if (!seen.insert(link->index).second)
            break;
        link = &c->nextColliderRef;
    }
    link->index = id;
    return id;
}

inline uint32_t
add_ni_psys_spherical_collider(nifly::NifFile& file, uint32_t manager) {
    return add_collider<nifly::NiPSysSphericalCollider>(file, manager);
}

inline uint32_t
add_ni_psys_planar_collider(nifly::NifFile& file, uint32_t manager) {
    return add_collider<nifly::NiPSysPlanarCollider>(file, manager);
}

// Particle controllers and their data.
NIFLY_RS_ADD_CONTROLLER(NiPSysEmitterCtlr, ni_psys_emitter_ctlr)
NIFLY_RS_ADD_CONTROLLER(
    BSPSysMultiTargetEmitterCtlr,
    bs_psys_multi_target_emitter_ctlr
)
NIFLY_RS_ADD_CONTROLLER(NiPSysUpdateCtlr, ni_psys_update_ctlr)
NIFLY_RS_ADD_CONTROLLER(NiPSysResetOnLoopCtlr, ni_psys_reset_on_loop_ctlr)
NIFLY_RS_ADD_CONTROLLER(NiPSysModifierActiveCtlr, ni_psys_modifier_active_ctlr)
NIFLY_RS_ADD_CONTROLLER(
    NiPSysEmitterLifeSpanCtlr,
    ni_psys_emitter_life_span_ctlr
)
NIFLY_RS_ADD_CONTROLLER(NiPSysEmitterSpeedCtlr, ni_psys_emitter_speed_ctlr)
NIFLY_RS_ADD_CONTROLLER(
    NiPSysEmitterInitialRadiusCtlr,
    ni_psys_emitter_initial_radius_ctlr
)
NIFLY_RS_ADD_CONTROLLER(
    NiPSysEmitterDeclinationCtlr,
    ni_psys_emitter_declination_ctlr
)
NIFLY_RS_ADD_CONTROLLER(
    NiPSysGravityStrengthCtlr,
    ni_psys_gravity_strength_ctlr
)
NIFLY_RS_ADD_CONTROLLER(
    NiPSysEmitterDeclinationVarCtlr,
    ni_psys_emitter_declination_var_ctlr
)
NIFLY_RS_ADD_CONTROLLER(NiPSysFieldMagnitudeCtlr, ni_psys_field_magnitude_ctlr)
NIFLY_RS_ADD_CONTROLLER(
    NiPSysFieldAttenuationCtlr,
    ni_psys_field_attenuation_ctlr
)
NIFLY_RS_ADD_CONTROLLER(
    NiPSysFieldMaxDistanceCtlr,
    ni_psys_field_max_distance_ctlr
)
NIFLY_RS_ADD_CONTROLLER(
    NiPSysAirFieldAirFrictionCtlr,
    ni_psys_air_field_air_friction_ctlr
)
NIFLY_RS_ADD_CONTROLLER(
    NiPSysAirFieldInheritVelocityCtlr,
    ni_psys_air_field_inherit_velocity_ctlr
)
NIFLY_RS_ADD_CONTROLLER(NiPSysAirFieldSpreadCtlr, ni_psys_air_field_spread_ctlr)
NIFLY_RS_ADD_CONTROLLER(
    NiPSysInitialRotSpeedCtlr,
    ni_psys_initial_rot_speed_ctlr
)
NIFLY_RS_ADD_CONTROLLER(
    NiPSysInitialRotSpeedVarCtlr,
    ni_psys_initial_rot_speed_var_ctlr
)
NIFLY_RS_ADD_CONTROLLER(
    NiPSysInitialRotAngleCtlr,
    ni_psys_initial_rot_angle_ctlr
)
NIFLY_RS_ADD_CONTROLLER(
    NiPSysInitialRotAngleVarCtlr,
    ni_psys_initial_rot_angle_var_ctlr
)
NIFLY_RS_ADD_CONTROLLER(
    NiPSysEmitterPlanarAngleCtlr,
    ni_psys_emitter_planar_angle_ctlr
)
NIFLY_RS_ADD_CONTROLLER(
    NiPSysEmitterPlanarAngleVarCtlr,
    ni_psys_emitter_planar_angle_var_ctlr
)
NIFLY_RS_ADD_CONTROLLER(NiPSysRotDampeningCtlr, ni_psys_rot_dampening_ctlr)
NIFLY_RS_ADD_ANIMATION(NiPSysEmitterCtlrData, ni_psys_emitter_ctlr_data)
} // namespace nifly_rs
