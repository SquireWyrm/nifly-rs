// Part of particles_shim.h: adding particle systems, modifiers, colliders and controllers.
#pragma once

#include "particles_shim.h"

namespace nifly_rs {
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
