// Part of particles_shim.h: NiParticleSystem fields, materials and shader.
#pragma once

#include "particles_shim.h"

namespace nifly_rs {
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
} // namespace nifly_rs
