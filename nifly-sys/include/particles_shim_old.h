// Part of particles_shim.h: old (Morrowind and Oblivion) particle fields and creation.
#pragma once

#include "particles_shim.h"

namespace nifly_rs {
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
} // namespace nifly_rs
