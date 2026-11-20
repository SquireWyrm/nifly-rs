// Part of shaders_shim.h: fixed-function and legacy shader property fields.
#pragma once

#include "shaders_shim.h"

namespace nifly_rs {
// Fixed-function properties.
#define NIFLY_RS_PROPERTY(T, prefix, name, Type, field) \
    NIFLY_RS_FIELD(T, prefix, name, Type, field, NIFLY_RS_SAME, NIFLY_RS_SAME)

NIFLY_RS_PROPERTY(NiSpecularProperty, specular, flags, uint16_t, flags)
NIFLY_RS_PROPERTY(NiDitherProperty, dither, flags, uint16_t, flags)
NIFLY_RS_PROPERTY(NiWireframeProperty, wireframe, flags, uint16_t, flags)
NIFLY_RS_PROPERTY(NiFogProperty, fog, flags, uint16_t, flags)
NIFLY_RS_PROPERTY(NiFogProperty, fog, depth, float, fogDepth)
NIFLY_RS_FIELD(
    NiFogProperty,
    fog,
    color,
    Vec3,
    fogColor,
    color3_to_ffi,
    color3_from_ffi
)
NIFLY_RS_PROPERTY(NiVertexColorProperty, vertex_color, flags, uint16_t, flags)
NIFLY_RS_PROPERTY(
    NiVertexColorProperty,
    vertex_color,
    vertex_mode,
    uint32_t,
    vertexMode
)
NIFLY_RS_PROPERTY(
    NiVertexColorProperty,
    vertex_color,
    lighting_mode,
    uint32_t,
    lightingMode
)
NIFLY_RS_PROPERTY(NiZBufferProperty, z_buffer, flags, uint16_t, flags)

inline uint32_t z_buffer_function(const nifly::NiZBufferProperty& z) {
    return z.testFunction;
}

inline void set_z_buffer_function(nifly::NiZBufferProperty& z, uint32_t v) {
    z.testFunction = static_cast<nifly::TestFunction>(v);
}

// Fallout 3 and New Vegas shaders, and BSWaterShaderProperty and
// BSSkyShaderProperty. Enums are passed as uint32_t.
#define NIFLY_RS_ENUM_FIELD(T, prefix, name, field) \
    inline uint32_t prefix##_##name(const nifly::T& s) { \
        return static_cast<uint32_t>(s.field); \
    } \
    inline void set_##prefix##_##name(nifly::T& s, uint32_t v) { \
        s.field = static_cast<decltype(s.field)>(v); \
    }
// An NiString field, passed as bytes.
#define NIFLY_RS_STRING_FIELD(T, prefix, name, field) \
    inline rust::Slice<const uint8_t> prefix##_##name(const nifly::T& s) { \
        const std::string& str = s.field.get(); \
        return {reinterpret_cast<const uint8_t*>(str.data()), str.size()}; \
    } \
    inline void set_##prefix##_##name( \
        nifly::T& s, \
        rust::Slice<const uint8_t> v \
    ) { \
        s.field.get().assign( \
            reinterpret_cast<const char*>(v.data()), \
            v.size() \
        ); \
    }

NIFLY_RS_ENUM_FIELD(BSShaderProperty, shader, legacy_type, shaderType)
NIFLY_RS_ENUM_FIELD(
    BSShaderLightingProperty,
    shader_lighting,
    clamp_mode,
    textureClampMode
)
NIFLY_RS_PROPERTY(
    BSShaderPPLightingProperty,
    pp_lighting,
    refraction_strength,
    float,
    refractionStrength
)
NIFLY_RS_PROPERTY(
    BSShaderPPLightingProperty,
    pp_lighting,
    refraction_fire_period,
    int32_t,
    refractionFirePeriod
)
NIFLY_RS_PROPERTY(
    BSShaderPPLightingProperty,
    pp_lighting,
    parallax_max_passes,
    float,
    parallaxMaxPasses
)
NIFLY_RS_PROPERTY(
    BSShaderPPLightingProperty,
    pp_lighting,
    parallax_scale,
    float,
    parallaxScale
)
NIFLY_RS_FIELD(
    BSShaderPPLightingProperty,
    pp_lighting,
    emissive_color,
    Rgba,
    emissiveColor,
    color4_to_ffi,
    color4_from_ffi
)
NIFLY_RS_STRING_FIELD(
    BSShaderNoLightingProperty,
    no_lighting,
    base_texture,
    baseTexture
)
NIFLY_RS_PROPERTY(
    BSShaderNoLightingProperty,
    no_lighting,
    falloff_start_angle,
    float,
    falloffStartAngle
)
NIFLY_RS_PROPERTY(
    BSShaderNoLightingProperty,
    no_lighting,
    falloff_stop_angle,
    float,
    falloffStopAngle
)
NIFLY_RS_PROPERTY(
    BSShaderNoLightingProperty,
    no_lighting,
    falloff_start_opacity,
    float,
    falloffStartOpacity
)
NIFLY_RS_PROPERTY(
    BSShaderNoLightingProperty,
    no_lighting,
    falloff_stop_opacity,
    float,
    falloffStopOpacity
)
NIFLY_RS_STRING_FIELD(SkyShaderProperty, sky, file_name, fileName)
NIFLY_RS_ENUM_FIELD(SkyShaderProperty, sky, object_type, skyObjectType)
NIFLY_RS_STRING_FIELD(TileShaderProperty, tile, file_name, fileName)
NIFLY_RS_STRING_FIELD(TallGrassShaderProperty, tall_grass, file_name, fileName)
NIFLY_RS_PROPERTY(BSWaterShaderProperty, bs_water, flags, uint32_t, waterFlags)
NIFLY_RS_STRING_FIELD(BSSkyShaderProperty, bs_sky, base_texture, baseTexture)
NIFLY_RS_ENUM_FIELD(BSSkyShaderProperty, bs_sky, object_type, skyFlags)

// Raw flags and fields of unknown meaning.
// legacyFlags is protected; this reaches it through a pointer to member.
struct MaterialAccess: nifly::NiMaterialProperty {
    using nifly::NiMaterialProperty::legacyFlags;
};

inline uint16_t material_legacy_flags(const nifly::NiMaterialProperty& m) {
    return m.*(&MaterialAccess::legacyFlags);
}

inline void
set_material_legacy_flags(nifly::NiMaterialProperty& m, uint16_t v) {
    m.*(&MaterialAccess::legacyFlags) = v;
}

NIFLY_RS_PROPERTY(
    NiStencilProperty,
    stencil,
    legacy_flags,
    uint16_t,
    legacyFlags
)
NIFLY_RS_LIGHTING(unknown_float, float, unkFloat)
NIFLY_RS_LIGHTING(unknown_float_1, float, unkFloat1)
NIFLY_RS_LIGHTING(unknown_float_2, float, unkFloat2)
NIFLY_RS_LIGHTING(unknown_short, uint16_t, unkShort1)
NIFLY_RS_LIGHTING(wetness_unknown_1, float, wetnessUnknown1)
NIFLY_RS_LIGHTING(wetness_unknown_2, float, wetnessUnknown2)
NIFLY_RS_EFFECT(unknown_float, float, unkFloat)
NIFLY_RS_EFFECT(unknown_byte, uint8_t, unkByte1)

// FO3ShadingFlags, passed as its raw value.
inline uint16_t shade_flags(const nifly::NiShadeProperty& s) {
    return static_cast<uint16_t>(s.shadingFlags);
}

inline void set_shade_flags(nifly::NiShadeProperty& s, uint16_t v) {
    s.shadingFlags = static_cast<nifly::FO3ShadingFlags>(v);
}

inline std::array<uint8_t, 7>
effect_unknown_bytes(const nifly::BSEffectShaderProperty& s) {
    std::array<uint8_t, 7> out;
    std::copy(std::begin(s.unkBytes), std::end(s.unkBytes), out.begin());
    return out;
}

inline void set_effect_unknown_bytes(
    nifly::BSEffectShaderProperty& s,
    std::array<uint8_t, 7> v
) {
    std::copy(v.begin(), v.end(), std::begin(s.unkBytes));
}

inline std::array<float, 6>
effect_unknown_floats(const nifly::BSEffectShaderProperty& s) {
    std::array<float, 6> out;
    std::copy(std::begin(s.unkFloats), std::end(s.unkFloats), out.begin());
    return out;
}

inline void set_effect_unknown_floats(
    nifly::BSEffectShaderProperty& s,
    std::array<float, 6> v
) {
    std::copy(v.begin(), v.end(), std::begin(s.unkFloats));
}

// SF1 (list 0) or SF2 (list 1): shader flags as CRC32s of their names.
inline const std::vector<uint32_t>&
shader_crc_flags_list(const nifly::BSShaderProperty& s, uint32_t list) {
    return list == 0 ? s.SF1 : s.SF2;
}

inline rust::Slice<const uint32_t>
shader_crc_flags(const nifly::BSShaderProperty& s, uint32_t list) {
    const auto& v = shader_crc_flags_list(s, list);
    return {v.data(), v.size()};
}

// Keeps the count saved with the list in step.
inline void set_shader_crc_flags(
    nifly::BSShaderProperty& s,
    uint32_t list,
    rust::Slice<const uint32_t> flags
) {
    auto& v = list == 0 ? s.SF1 : s.SF2;
    v.assign(flags.begin(), flags.end());
    (list == 0 ? s.numSF1 : s.numSF2) = static_cast<uint32_t>(v.size());
}

// Resizes the set; new slots are empty.
inline void set_texture_set_len(nifly::BSShaderTextureSet& set, size_t len) {
    set.textures.resize(static_cast<uint32_t>(len));
}

// False if `index` is out of range.
inline bool set_texture_set_path(
    nifly::BSShaderTextureSet& set,
    size_t index,
    rust::Slice<const uint8_t> path
) {
    if (index >= set.textures.size())
        return false;
    set.textures.data()[index].get().assign(
        reinterpret_cast<const char*>(path.data()),
        path.size()
    );
    return true;
}
} // namespace nifly_rs
