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
} // namespace nifly_rs

// The rest of the shims, by topic.
// In this order: later parts use earlier ones.
// clang-format off
#include "particles_shim_data.h"
#include "particles_shim_system.h"
#include "particles_shim_modifiers.h"
#include "particles_shim_replacements.h"
#include "particles_shim_old.h"
#include "particles_shim_add.h"
// clang-format on
