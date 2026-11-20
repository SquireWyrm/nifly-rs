// Part of particles_shim.h: modifier, emitter, collider and particle controller fields.
#pragma once

#include "particles_shim.h"

namespace nifly_rs {
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
} // namespace nifly_rs
