// Part of shaders_shim.h: BSLightingShaderProperty and BSEffectShaderProperty fields.
#pragma once

#include "shaders_shim.h"

namespace nifly_rs {
// BSShaderProperty and BSLightingShaderProperty fields. Each macro defines a
// getter <prefix>_<name> and a setter set_<prefix>_<name>, converting between the
// nifly type and the FFI one.
#define NIFLY_RS_FIELD(T, prefix, name, FfiType, field, to_ffi, from_ffi) \
    inline FfiType prefix##_##name(const nifly::T& s) { \
        return to_ffi(s.field); \
    } \
    inline void set_##prefix##_##name(nifly::T& s, FfiType v) { \
        s.field = from_ffi(v); \
    }
#define NIFLY_RS_SAME(x) (x)
#define NIFLY_RS_LIGHTING(name, Type, field) \
    NIFLY_RS_FIELD( \
        BSLightingShaderProperty, \
        lighting, \
        name, \
        Type, \
        field, \
        NIFLY_RS_SAME, \
        NIFLY_RS_SAME \
    )
#define NIFLY_RS_LIGHTING_COLOR(name, field) \
    NIFLY_RS_FIELD( \
        BSLightingShaderProperty, \
        lighting, \
        name, \
        Vec3, \
        field, \
        color_to_ffi, \
        color_from_ffi \
    )

inline Vec2 vec2_to_ffi(const nifly::Vector2& v) {
    return {v.u, v.v};
}

inline nifly::Vector2 vec2_from_ffi(const Vec2& v) {
    return nifly::Vector2(v.x, v.y);
}

inline Rgba color4_to_ffi(const nifly::Color4& c) {
    return {c.r, c.g, c.b, c.a};
}

inline nifly::Color4 color4_from_ffi(const Rgba& c) {
    return nifly::Color4(c.r, c.g, c.b, c.a);
}

inline Vec3 color3_to_ffi(const nifly::Color3& c) {
    return {c.r, c.g, c.b};
}

inline nifly::Color3 color3_from_ffi(const Vec3& c) {
    return nifly::Color3(c.x, c.y, c.z);
}

NIFLY_RS_FIELD(
    BSShaderProperty,
    shader,
    uv_offset,
    Vec2,
    uvOffset,
    vec2_to_ffi,
    vec2_from_ffi
)
NIFLY_RS_FIELD(
    BSShaderProperty,
    shader,
    uv_scale,
    Vec2,
    uvScale,
    vec2_to_ffi,
    vec2_from_ffi
)
NIFLY_RS_FIELD(
    BSShaderProperty,
    shader,
    environment_map_scale,
    float,
    environmentMapScale,
    NIFLY_RS_SAME,
    NIFLY_RS_SAME
)

// The internal shader type numbering (with Parallax as 3) in every version.
inline void set_lighting_shader_type(
    nifly::BSLightingShaderProperty& shader,
    uint32_t type
) {
    shader.SetShaderType(type);
}

inline uint32_t
lighting_texture_clamp_mode(const nifly::BSLightingShaderProperty& s) {
    return s.textureClampMode;
}

inline void set_lighting_texture_clamp_mode(
    nifly::BSLightingShaderProperty& s,
    uint32_t mode
) {
    s.textureClampMode = static_cast<nifly::TexClampMode>(mode);
}

NIFLY_RS_LIGHTING_COLOR(emissive_color, emissiveColor)
NIFLY_RS_LIGHTING(emissive_multiple, float, emissiveMultiple)
NIFLY_RS_LIGHTING(alpha, float, alpha)
NIFLY_RS_LIGHTING(refraction_strength, float, refractionStrength)
NIFLY_RS_LIGHTING(glossiness, float, glossiness)
NIFLY_RS_LIGHTING_COLOR(specular_color, specularColor)
NIFLY_RS_LIGHTING(specular_strength, float, specularStrength)
NIFLY_RS_LIGHTING(soft_lighting, float, softlighting)
NIFLY_RS_LIGHTING(rimlight_power, float, rimlightPower)
NIFLY_RS_LIGHTING(subsurface_rolloff, float, subsurfaceRolloff)
NIFLY_RS_LIGHTING(rimlight_power_2, float, rimlightPower2)
NIFLY_RS_LIGHTING(backlight_power, float, backlightPower)
NIFLY_RS_LIGHTING(grayscale_to_palette_scale, float, grayscaleToPaletteScale)
NIFLY_RS_LIGHTING(fresnel_power, float, fresnelPower)
NIFLY_RS_LIGHTING(wetness_specular_scale, float, wetnessSpecScale)
NIFLY_RS_LIGHTING(wetness_specular_power, float, wetnessSpecPower)
NIFLY_RS_LIGHTING(wetness_min_var, float, wetnessMinVar)
NIFLY_RS_LIGHTING(wetness_envmap_scale, float, wetnessEnvmapScale)
NIFLY_RS_LIGHTING(wetness_fresnel_power, float, wetnessFresnelPower)
NIFLY_RS_LIGHTING(wetness_metalness, float, wetnessMetalness)
NIFLY_RS_LIGHTING(lum_emittance, float, lumEmittance)
NIFLY_RS_LIGHTING(exposure_offset, float, exposureOffset)
NIFLY_RS_LIGHTING(final_exposure_min, float, finalExposureMin)
NIFLY_RS_LIGHTING(final_exposure_max, float, finalExposureMax)
NIFLY_RS_LIGHTING(translucency, bool, doTranslucency)
NIFLY_RS_FIELD(
    BSLightingShaderProperty,
    lighting,
    subsurface_color,
    Vec3,
    subsurfaceColor,
    color3_to_ffi,
    color3_from_ffi
)
NIFLY_RS_LIGHTING(transmissive_scale, float, transmissiveScale)
NIFLY_RS_LIGHTING(turbulence, float, turbulence)
NIFLY_RS_LIGHTING(thick_object, bool, thickObject)
NIFLY_RS_LIGHTING(mix_albedo, bool, mixAlbedo)
NIFLY_RS_LIGHTING(use_ssr, bool, useSSR)
NIFLY_RS_LIGHTING(wetness_use_ssr, bool, wetnessUseSSR)
NIFLY_RS_LIGHTING_COLOR(skin_tint_color, skinTintColor)
NIFLY_RS_LIGHTING(skin_tint_alpha, float, skinTintAlpha)
NIFLY_RS_LIGHTING_COLOR(hair_tint_color, hairTintColor)
NIFLY_RS_LIGHTING(max_passes, float, maxPasses)
NIFLY_RS_LIGHTING(scale, float, scale)
NIFLY_RS_LIGHTING(
    parallax_inner_layer_thickness,
    float,
    parallaxInnerLayerThickness
)
NIFLY_RS_LIGHTING(parallax_refraction_scale, float, parallaxRefractionScale)
NIFLY_RS_FIELD(
    BSLightingShaderProperty,
    lighting,
    parallax_inner_layer_texture_scale,
    Vec2,
    parallaxInnerLayerTextureScale,
    vec2_to_ffi,
    vec2_from_ffi
)
NIFLY_RS_LIGHTING(parallax_envmap_strength, float, parallaxEnvmapStrength)
NIFLY_RS_FIELD(
    BSLightingShaderProperty,
    lighting,
    sparkle_parameters,
    Rgba,
    sparkleParameters,
    color4_to_ffi,
    color4_from_ffi
)
NIFLY_RS_LIGHTING(eye_cubemap_scale, float, eyeCubemapScale)
NIFLY_RS_LIGHTING_COLOR(eye_left_reflection_center, eyeLeftReflectionCenter)
NIFLY_RS_LIGHTING_COLOR(eye_right_reflection_center, eyeRightReflectionCenter)

inline rust::Slice<const uint8_t>
lighting_root_material(const nifly::BSLightingShaderProperty& s) {
    const std::string& str = s.rootMaterialName.get();
    return {reinterpret_cast<const uint8_t*>(str.data()), str.size()};
}

inline void set_lighting_root_material(
    nifly::BSLightingShaderProperty& s,
    rust::Slice<const uint8_t> name
) {
    s.rootMaterialName.get().assign(
        reinterpret_cast<const char*>(name.data()),
        name.size()
    );
}

// Texture arrays. hasTextureArrays and numTextureArrays are kept in step with the
// vector, since writing trusts numTextureArrays.
inline size_t
lighting_texture_array_count(const nifly::BSLightingShaderProperty& s) {
    return s.textureArrays.size();
}

inline size_t lighting_texture_array_len(
    const nifly::BSLightingShaderProperty& s,
    size_t array
) {
    return array < s.textureArrays.size()
        ? s.textureArrays[array].textureArray.size()
        : 0;
}

inline rust::Slice<const uint8_t> lighting_texture_array_path(
    const nifly::BSLightingShaderProperty& s,
    size_t array,
    size_t index
) {
    if (array >= s.textureArrays.size())
        return {};
    const auto& paths = s.textureArrays[array].textureArray;
    if (index >= paths.size())
        return {};
    const std::string& str = paths.data()[index].get();
    return {reinterpret_cast<const uint8_t*>(str.data()), str.size()};
}

inline void clear_lighting_texture_arrays(nifly::BSLightingShaderProperty& s) {
    s.textureArrays.clear();
    s.numTextureArrays = 0;
    s.hasTextureArrays = false;
}

// Appends an empty array.
inline void add_lighting_texture_array(nifly::BSLightingShaderProperty& s) {
    s.textureArrays.emplace_back();
    s.numTextureArrays = static_cast<uint32_t>(s.textureArrays.size());
    s.hasTextureArrays = true;
}

// Appends a path to the last array; does nothing without one.
inline void push_lighting_texture_array_path(
    nifly::BSLightingShaderProperty& s,
    rust::Slice<const uint8_t> path
) {
    if (s.textureArrays.empty())
        return;
    nifly::NiString str;
    str.get().assign(reinterpret_cast<const char*>(path.data()), path.size());
    s.textureArrays.back().textureArray.push_back(str);
}

// BSEffectShaderProperty fields. Texture paths are NiStrings, passed as bytes.
#define NIFLY_RS_EFFECT(name, Type, field) \
    NIFLY_RS_FIELD( \
        BSEffectShaderProperty, \
        effect, \
        name, \
        Type, \
        field, \
        NIFLY_RS_SAME, \
        NIFLY_RS_SAME \
    )
#define NIFLY_RS_EFFECT_TEXTURE(name, field) \
    inline rust::Slice<const uint8_t> effect_##name( \
        const nifly::BSEffectShaderProperty& s \
    ) { \
        const std::string& str = s.field.get(); \
        return {reinterpret_cast<const uint8_t*>(str.data()), str.size()}; \
    } \
    inline void set_effect_##name( \
        nifly::BSEffectShaderProperty& s, \
        rust::Slice<const uint8_t> path \
    ) { \
        s.field.get().assign( \
            reinterpret_cast<const char*>(path.data()), \
            path.size() \
        ); \
    }

// textureClampMode holds four bytes: clamp mode, lighting influence, environment
// map minimum LOD and an unused byte, lowest first.
NIFLY_RS_EFFECT(texture_clamp_bytes, uint32_t, textureClampMode)
NIFLY_RS_EFFECT(falloff_start_angle, float, falloffStartAngle)
NIFLY_RS_EFFECT(falloff_stop_angle, float, falloffStopAngle)
NIFLY_RS_EFFECT(falloff_start_opacity, float, falloffStartOpacity)
NIFLY_RS_EFFECT(falloff_stop_opacity, float, falloffStopOpacity)
NIFLY_RS_EFFECT(refraction_power, float, refractionPower)
NIFLY_RS_FIELD(
    BSEffectShaderProperty,
    effect,
    base_color,
    Rgba,
    baseColor,
    color4_to_ffi,
    color4_from_ffi
)
NIFLY_RS_EFFECT(base_color_scale, float, baseColorScale)
NIFLY_RS_EFFECT(soft_falloff_depth, float, softFalloffDepth)
NIFLY_RS_EFFECT(env_map_scale, float, envMapScale)
NIFLY_RS_FIELD(
    BSEffectShaderProperty,
    effect,
    emittance_color,
    Vec3,
    emittanceColor,
    color3_to_ffi,
    color3_from_ffi
)
NIFLY_RS_EFFECT(lum_emittance, float, lumEmittance)
NIFLY_RS_EFFECT(exposure_offset, float, exposureOffset)
NIFLY_RS_EFFECT(final_exposure_min, float, finalExposureMin)
NIFLY_RS_EFFECT(final_exposure_max, float, finalExposureMax)
NIFLY_RS_EFFECT_TEXTURE(source_texture, sourceTexture)
NIFLY_RS_EFFECT_TEXTURE(greyscale_texture, greyscaleTexture)
NIFLY_RS_EFFECT_TEXTURE(env_map_texture, envMapTexture)
NIFLY_RS_EFFECT_TEXTURE(normal_texture, normalTexture)
NIFLY_RS_EFFECT_TEXTURE(env_mask_texture, envMaskTexture)
NIFLY_RS_EFFECT_TEXTURE(reflectance_texture, reflectanceTexture)
NIFLY_RS_EFFECT_TEXTURE(lighting_texture, lightingTexture)
NIFLY_RS_EFFECT_TEXTURE(emit_gradient_texture, emitGradientTexture)
} // namespace nifly_rs
