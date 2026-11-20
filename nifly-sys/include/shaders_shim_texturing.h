// Part of shaders_shim.h: NiTexturingProperty maps and texture set paths.
#pragma once

#include "shaders_shim.h"

namespace nifly_rs {
inline uint32_t texturing_texture_count(const nifly::NiTexturingProperty& t) {
    return t.textureCount;
}

// The descriptor of map `map` in file order (base, dark, detail, gloss, glow,
// bump, normal, parallax, decals 0-3, then shader map `map - 12`), or null if
// it's absent or out of range.
inline nifly::TexDesc*
texturing_map(nifly::NiTexturingProperty& t, uint32_t map) {
    if (map >= texturing_shader_maps) {
        auto shader = texturing_shader_map(t, map);
        return shader && shader->isUsed ? &shader->data : nullptr;
    }
    auto members = texturing_map_members(map);
    if (!members || !(t.*members->has))
        return nullptr;
    return &(t.*members->desc);
}

// Only reads.
inline const nifly::TexDesc*
texturing_map(const nifly::NiTexturingProperty& t, uint32_t map) {
    return texturing_map(const_cast<nifly::NiTexturingProperty&>(t), map);
}

inline bool
texturing_has_map(const nifly::NiTexturingProperty& t, uint32_t map) {
    return texturing_map(t, map) != nullptr;
}

inline uint32_t
texturing_map_source(const nifly::NiTexturingProperty& t, uint32_t map) {
    auto desc = texturing_map(t, map);
    return desc ? desc->sourceRef.index : nifly::NIF_NPOS;
}

// A getter and setter for the uint32_t-like TexDesc field `field` of a map.
#define NIFLY_RS_MAP_FIELD(field, type, snake) \
    inline bool snake( \
        const nifly::NiTexturingProperty& t, \
        uint32_t map, \
        uint32_t& out \
    ) { \
        auto desc = texturing_map(t, map); \
        if (!desc) \
            return false; \
        out = static_cast<uint32_t>(desc->field); \
        return true; \
    } \
    inline bool set_##snake( \
        nifly::NiTexturingProperty& t, \
        uint32_t map, \
        uint32_t value \
    ) { \
        auto desc = texturing_map(t, map); \
        if (!desc) \
            return false; \
        desc->field = static_cast<type>(value); \
        return true; \
    }

NIFLY_RS_MAP_FIELD(clampMode, nifly::TexClampMode, texturing_map_clamp_mode)
NIFLY_RS_MAP_FIELD(filterMode, nifly::TexFilterMode, texturing_map_filter_mode)
NIFLY_RS_MAP_FIELD(uvSet, uint32_t, texturing_map_uv_set)
NIFLY_RS_MAP_FIELD(flags, uint16_t, texturing_map_flags)
NIFLY_RS_MAP_FIELD(maxAnisotropy, uint16_t, texturing_map_max_anisotropy)
NIFLY_RS_MAP_FIELD(unkShort1, uint16_t, texturing_map_unknown)
// int16_t values, sign-extended.
NIFLY_RS_MAP_FIELD(ps2_l, int16_t, texturing_map_ps2_l)
NIFLY_RS_MAP_FIELD(ps2_k, int16_t, texturing_map_ps2_k)

#undef NIFLY_RS_MAP_FIELD

// True if map `map` exists and has a transform, which goes to `out`.
inline bool texturing_map_transform(
    const nifly::NiTexturingProperty& t,
    uint32_t map,
    TextureTransform& out
) {
    auto desc = texturing_map(t, map);
    if (!desc || !desc->hasTexTransform)
        return false;
    const auto& x = desc->transform;
    out = {
        {x.translation.u, x.translation.v},
        {x.scale.u, x.scale.v},
        x.wRotation,
        x.transformType,
        {x.center.u, x.center.v},
    };
    return true;
}

// Gives map `map` the transform `value`, or none if `has` is false. False if
// the map is absent.
inline bool set_texturing_map_transform(
    nifly::NiTexturingProperty& t,
    uint32_t map,
    bool has,
    const TextureTransform& value
) {
    auto desc = texturing_map(t, map);
    if (!desc)
        return false;
    desc->hasTexTransform = has;
    if (has) {
        auto& x = desc->transform;
        x.translation =
            nifly::Vector2(value.translation.x, value.translation.y);
        x.scale = nifly::Vector2(value.scale.x, value.scale.y);
        x.wRotation = value.rotation;
        x.transformType = value.transform_type;
        x.center = nifly::Vector2(value.center.x, value.center.y);
    }
    return true;
}

inline uint16_t texturing_flags(const nifly::NiTexturingProperty& t) {
    return t.flags;
}

inline void set_texturing_flags(nifly::NiTexturingProperty& t, uint16_t v) {
    t.flags = v;
}

inline void
set_texturing_texture_count(nifly::NiTexturingProperty& t, uint32_t v) {
    t.textureCount = v;
}

inline float texturing_luma_scale(const nifly::NiTexturingProperty& t) {
    return t.lumaScale;
}

inline void set_texturing_luma_scale(nifly::NiTexturingProperty& t, float v) {
    t.lumaScale = v;
}

inline float texturing_luma_offset(const nifly::NiTexturingProperty& t) {
    return t.lumaOffset;
}

inline void set_texturing_luma_offset(nifly::NiTexturingProperty& t, float v) {
    t.lumaOffset = v;
}

inline std::array<float, 4>
texturing_bump_matrix(const nifly::NiTexturingProperty& t) {
    const auto& m = t.bumpMatrix;
    return {m.x, m.y, m.z, m.w};
}

inline void set_texturing_bump_matrix(
    nifly::NiTexturingProperty& t,
    std::array<float, 4> v
) {
    t.bumpMatrix = nifly::Vector4(v[0], v[1], v[2], v[3]);
}

inline float texturing_parallax_offset(const nifly::NiTexturingProperty& t) {
    return t.parallaxOffset;
}

inline void
set_texturing_parallax_offset(nifly::NiTexturingProperty& t, float v) {
    t.parallaxOffset = v;
}

inline size_t texturing_shader_map_count(const nifly::NiTexturingProperty& t) {
    return t.shaderTex.size();
}

// New shader maps are unused.
inline void
set_texturing_shader_map_count(nifly::NiTexturingProperty& t, size_t count) {
    t.shaderTex.resize(static_cast<uint32_t>(count));
}

// A used shader map's ID (`mapIndex`). False if shader map `i` is unused or
// out of range.
inline bool texturing_shader_map_id(
    const nifly::NiTexturingProperty& t,
    size_t i,
    uint32_t& out
) {
    if (i >= t.shaderTex.size() || !t.shaderTex.data()[i].isUsed)
        return false;
    out = t.shaderTex.data()[i].mapIndex;
    return true;
}

inline bool set_texturing_shader_map_id(
    nifly::NiTexturingProperty& t,
    size_t i,
    uint32_t id
) {
    if (i >= t.shaderTex.size() || !t.shaderTex.data()[i].isUsed)
        return false;
    t.shaderTex.data()[i].mapIndex = id;
    return true;
}

inline size_t texture_set_len(const nifly::BSShaderTextureSet& set) {
    return set.textures.size();
}

inline rust::Slice<const uint8_t>
texture_set_path(const nifly::BSShaderTextureSet& set, size_t index) {
    if (index >= set.textures.size())
        return {};
    const std::string& s = set.textures.data()[index].get();
    return {reinterpret_cast<const uint8_t*>(s.data()), s.size()};
}
} // namespace nifly_rs
