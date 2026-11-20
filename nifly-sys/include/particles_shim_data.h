// Part of particles_shim.h: particle data fields and per-particle lists.
#pragma once

#include "particles_shim.h"

namespace nifly_rs {
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
} // namespace nifly_rs
