#pragma once

#include <algorithm>
#include <array>
#include <iterator>
#include <vector>

#include "Shaders.hpp"
#include "nifly-sys/src/object_3d.rs.h"
#include "nifly-sys/src/shaders.rs.h"
#include "rust/cxx.h"
#include "texturing_maps.h"

// For each shader type T with direct base Parent, defines:
// - upcast_<snake>: T -> Parent. Implicit derived-to-base conversion, always succeeds.
// - downcast_<snake>: NiObject -> T. Returns null unless obj is a T or a subclass of T.
#define NIFLY_RS_SHADER(T, Parent, snake) \
    inline const nifly::Parent& upcast_##snake(const nifly::T& obj) { \
        return obj; \
    } \
    inline const nifly::T* downcast_##snake(const nifly::NiObject& obj) { \
        return dynamic_cast<const nifly::T*>(&obj); \
    }

namespace nifly_rs {
NIFLY_RS_SHADER(NiProperty, NiObjectNET, ni_property)
NIFLY_RS_SHADER(NiShader, NiProperty, ni_shader)
NIFLY_RS_SHADER(NiShadeProperty, NiShader, ni_shade_property)
NIFLY_RS_SHADER(BSShaderProperty, NiShadeProperty, bs_shader_property)
NIFLY_RS_SHADER(WaterShaderProperty, BSShaderProperty, water_shader_property)
NIFLY_RS_SHADER(HairShaderProperty, BSShaderProperty, hair_shader_property)
NIFLY_RS_SHADER(
    DistantLODShaderProperty,
    BSShaderProperty,
    distant_lod_shader_property
)
NIFLY_RS_SHADER(
    BSDistantTreeShaderProperty,
    BSShaderProperty,
    bs_distant_tree_shader_property
)
NIFLY_RS_SHADER(
    TallGrassShaderProperty,
    BSShaderProperty,
    tall_grass_shader_property
)
NIFLY_RS_SHADER(
    VolumetricFogShaderProperty,
    BSShaderProperty,
    volumetric_fog_shader_property
)
NIFLY_RS_SHADER(
    BSLightingShaderProperty,
    BSShaderProperty,
    bs_lighting_shader_property
)
NIFLY_RS_SHADER(
    BSEffectShaderProperty,
    BSShaderProperty,
    bs_effect_shader_property
)
NIFLY_RS_SHADER(
    BSWaterShaderProperty,
    BSShaderProperty,
    bs_water_shader_property
)
NIFLY_RS_SHADER(BSSkyShaderProperty, BSShaderProperty, bs_sky_shader_property)
NIFLY_RS_SHADER(
    BSShaderLightingProperty,
    BSShaderProperty,
    bs_shader_lighting_property
)
NIFLY_RS_SHADER(
    SkyShaderProperty,
    BSShaderLightingProperty,
    sky_shader_property
)
NIFLY_RS_SHADER(
    TileShaderProperty,
    BSShaderLightingProperty,
    tile_shader_property
)
NIFLY_RS_SHADER(
    BSShaderNoLightingProperty,
    BSShaderLightingProperty,
    bs_shader_no_lighting_property
)
NIFLY_RS_SHADER(
    BSShaderPPLightingProperty,
    BSShaderLightingProperty,
    bs_shader_pp_lighting_property
)
NIFLY_RS_SHADER(
    Lighting30ShaderProperty,
    BSShaderPPLightingProperty,
    lighting30_shader_property
)
NIFLY_RS_SHADER(NiMaterialProperty, NiShader, ni_material_property)
NIFLY_RS_SHADER(NiAlphaProperty, NiProperty, ni_alpha_property)
NIFLY_RS_SHADER(NiStencilProperty, NiProperty, ni_stencil_property)
NIFLY_RS_SHADER(NiTexturingProperty, NiProperty, ni_texturing_property)
NIFLY_RS_SHADER(NiSpecularProperty, NiProperty, ni_specular_property)
NIFLY_RS_SHADER(NiVertexColorProperty, NiProperty, ni_vertex_color_property)
NIFLY_RS_SHADER(NiDitherProperty, NiProperty, ni_dither_property)
NIFLY_RS_SHADER(NiFogProperty, NiProperty, ni_fog_property)
NIFLY_RS_SHADER(NiWireframeProperty, NiProperty, ni_wireframe_property)
NIFLY_RS_SHADER(NiZBufferProperty, NiProperty, ni_z_buffer_property)
NIFLY_RS_SHADER(NiTexture, NiObjectNET, ni_texture)
NIFLY_RS_SHADER(NiSourceTexture, NiTexture, ni_source_texture)
NIFLY_RS_SHADER(NiPalette, NiObject, ni_palette)
NIFLY_RS_SHADER(TextureRenderData, NiObject, texture_render_data)
NIFLY_RS_SHADER(NiPixelData, TextureRenderData, ni_pixel_data)
NIFLY_RS_SHADER(
    NiPersistentSrcTextureRendererData,
    TextureRenderData,
    ni_persistent_src_texture_renderer_data
)
NIFLY_RS_SHADER(NiSourceCubeMap, NiSourceTexture, ni_source_cube_map)

NIFLY_RS_SHADER(BSShaderTextureSet, NiObject, bs_shader_texture_set)

inline uint32_t shader_flags_1(const nifly::BSShaderProperty& shader) {
    return shader.shaderFlags1;
}

inline uint32_t shader_flags_2(const nifly::BSShaderProperty& shader) {
    return shader.shaderFlags2;
}

inline void
set_shader_flags_1(nifly::BSShaderProperty& shader, uint32_t flags) {
    shader.shaderFlags1 = flags;
}

inline void
set_shader_flags_2(nifly::BSShaderProperty& shader, uint32_t flags) {
    shader.shaderFlags2 = flags;
}

inline uint32_t
lighting_shader_type(const nifly::BSLightingShaderProperty& shader) {
    return shader.GetShaderType();
}

// NiMaterialProperty. Colors are passed as Vec3 (r, g, b in x, y, z).
inline Vec3 color_to_ffi(const nifly::Vector3& c) {
    return {c.x, c.y, c.z};
}

inline nifly::Vector3 color_from_ffi(const Vec3& c) {
    return nifly::Vector3(c.x, c.y, c.z);
}

inline Vec3 material_ambient(const nifly::NiMaterialProperty& m) {
    return color_to_ffi(m.colorAmbient);
}

inline Vec3 material_diffuse(const nifly::NiMaterialProperty& m) {
    return color_to_ffi(m.colorDiffuse);
}

inline Vec3 material_specular(const nifly::NiMaterialProperty& m) {
    return color_to_ffi(m.GetSpecularColor());
}

// GetEmissiveColor returns a Color4 whose alpha is unused.
inline Vec3 material_emissive(const nifly::NiMaterialProperty& m) {
    nifly::Color4 c = m.GetEmissiveColor();
    return {c.r, c.g, c.b};
}

inline float material_glossiness(const nifly::NiMaterialProperty& m) {
    return m.GetGlossiness();
}

inline float material_alpha(const nifly::NiMaterialProperty& m) {
    return m.GetAlpha();
}

inline float material_emissive_multiple(const nifly::NiMaterialProperty& m) {
    return m.GetEmissiveMultiple();
}

inline bool material_is_emissive(const nifly::NiMaterialProperty& m) {
    return m.IsEmissive();
}

inline bool material_has_specular(const nifly::NiMaterialProperty& m) {
    return m.HasSpecular();
}

inline void set_material_ambient(nifly::NiMaterialProperty& m, Vec3 c) {
    m.colorAmbient = color_from_ffi(c);
}

inline void set_material_diffuse(nifly::NiMaterialProperty& m, Vec3 c) {
    m.colorDiffuse = color_from_ffi(c);
}

inline void set_material_specular(nifly::NiMaterialProperty& m, Vec3 c) {
    m.SetSpecularColor(color_from_ffi(c));
}

// SetEmissiveColor only reads r, g and b.
inline void set_material_emissive(nifly::NiMaterialProperty& m, Vec3 c) {
    m.SetEmissiveColor(nifly::Color4(c.x, c.y, c.z, 1.0f));
}

inline void set_material_glossiness(nifly::NiMaterialProperty& m, float v) {
    m.SetGlossiness(v);
}

inline void set_material_alpha(nifly::NiMaterialProperty& m, float v) {
    m.SetAlpha(v);
}

inline void
set_material_emissive_multiple(nifly::NiMaterialProperty& m, float v) {
    m.SetEmissiveMultiple(v);
}

inline uint16_t alpha_flags(const nifly::NiAlphaProperty& a) {
    return a.flags;
}

inline uint8_t alpha_threshold(const nifly::NiAlphaProperty& a) {
    return a.threshold;
}

inline void set_alpha_flags(nifly::NiAlphaProperty& a, uint16_t flags) {
    a.flags = flags;
}

inline void set_alpha_threshold(nifly::NiAlphaProperty& a, uint8_t threshold) {
    a.threshold = threshold;
}

// A getter `snake` and setter `set_snake` for the public field `T::field`.
#define NIFLY_RS_FIELD(T, field, type, snake) \
    inline type snake(const nifly::T& obj) { \
        return obj.field; \
    } \
    inline void set_##snake(nifly::T& obj, type value) { \
        obj.field = value; \
    }

NIFLY_RS_FIELD(NiStencilProperty, flags, uint16_t, stencil_flags)
NIFLY_RS_FIELD(NiStencilProperty, stencilEnabled, bool, stencil_enabled)
NIFLY_RS_FIELD(NiStencilProperty, stencilFunction, uint32_t, stencil_function)
NIFLY_RS_FIELD(NiStencilProperty, stencilRef, uint32_t, stencil_reference)
NIFLY_RS_FIELD(NiStencilProperty, stencilMask, uint32_t, stencil_mask)
NIFLY_RS_FIELD(NiStencilProperty, failAction, uint32_t, stencil_fail_action)
NIFLY_RS_FIELD(NiStencilProperty, zFailAction, uint32_t, stencil_z_fail_action)
NIFLY_RS_FIELD(NiStencilProperty, passAction, uint32_t, stencil_pass_action)
NIFLY_RS_FIELD(NiStencilProperty, drawMode, uint32_t, stencil_draw_mode)

NIFLY_RS_FIELD(NiTexturingProperty, applyMode, uint32_t, texturing_apply_mode)

#undef NIFLY_RS_FIELD

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

// NiSourceTexture fields. The enums are passed as their raw values.
#define NIFLY_RS_SOURCE(name, Type, field) \
    NIFLY_RS_FIELD( \
        NiSourceTexture, \
        source, \
        name, \
        Type, \
        field, \
        NIFLY_RS_SAME, \
        NIFLY_RS_SAME \
    )
#define NIFLY_RS_SOURCE_ENUM(name, Enum, field) \
    inline uint32_t source_##name(const nifly::NiSourceTexture& s) { \
        return s.field; \
    } \
    inline void set_source_##name(nifly::NiSourceTexture& s, uint32_t v) { \
        s.field = static_cast<nifly::Enum>(v); \
    }

NIFLY_RS_SOURCE(use_external, bool, useExternal)
NIFLY_RS_SOURCE(use_internal, bool, useInternal)
NIFLY_RS_SOURCE(is_static, bool, isStatic)
NIFLY_RS_SOURCE(direct_render, bool, directRender)
NIFLY_RS_SOURCE(persistent_render_data, bool, persistentRenderData)
NIFLY_RS_SOURCE_ENUM(pixel_layout, PixelLayout, pixelLayout)
NIFLY_RS_SOURCE_ENUM(mip_map_format, MipMapFormat, mipMapFormat)
NIFLY_RS_SOURCE_ENUM(alpha_format, AlphaFormat, alphaFormat)

inline rust::Slice<const uint8_t>
source_file_name(const nifly::NiSourceTexture& s) {
    const std::string& str = s.fileName.get();
    return {reinterpret_cast<const uint8_t*>(str.data()), str.size()};
}

inline void set_source_file_name(
    nifly::NiSourceTexture& s,
    rust::Slice<const uint8_t> path
) {
    s.fileName.get().assign(
        reinterpret_cast<const char*>(path.data()),
        path.size()
    );
}

// The pixel data's block index; NIF_NPOS if the ref is empty.
inline uint32_t source_pixel_data(const nifly::NiSourceTexture& s) {
    return s.dataRef.index;
}

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

// TextureRenderData fields.
#define NIFLY_RS_RENDER_FIELD(field, type, snake) \
    inline type render_data_##snake(const nifly::TextureRenderData& d) { \
        return static_cast<type>(d.field); \
    } \
    inline void set_render_data_##snake(nifly::TextureRenderData& d, type v) { \
        d.field = static_cast<decltype(d.field)>(v); \
    }
NIFLY_RS_RENDER_FIELD(pixelFormat, uint32_t, pixel_format)
NIFLY_RS_RENDER_FIELD(bitsPerPixel, uint8_t, bits_per_pixel)
NIFLY_RS_RENDER_FIELD(bytesPerPixel, uint32_t, bytes_per_pixel)
NIFLY_RS_RENDER_FIELD(rendererHint, uint32_t, renderer_hint)
NIFLY_RS_RENDER_FIELD(extraData, uint32_t, extra_data)
NIFLY_RS_RENDER_FIELD(flags, uint8_t, flags)
NIFLY_RS_RENDER_FIELD(pixelTiling, uint32_t, tiling)

inline std::array<uint8_t, 8>
render_data_old_fast_compare(const nifly::TextureRenderData& d) {
    std::array<uint8_t, 8> out;
    std::copy(
        std::begin(d.oldFastCompare),
        std::end(d.oldFastCompare),
        out.begin()
    );
    return out;
}

inline void set_render_data_old_fast_compare(
    nifly::TextureRenderData& d,
    std::array<uint8_t, 8> v
) {
    std::copy(v.begin(), v.end(), std::begin(d.oldFastCompare));
}

#undef NIFLY_RS_RENDER_FIELD

// Red, green, blue and alpha masks. `out` and `v` have 4 elements.
inline void render_data_masks(
    const nifly::TextureRenderData& d,
    rust::Slice<uint32_t> out
) {
    out[0] = d.redMask;
    out[1] = d.greenMask;
    out[2] = d.blueMask;
    out[3] = d.alphaMask;
}

inline void set_render_data_masks(
    nifly::TextureRenderData& d,
    rust::Slice<const uint32_t> v
) {
    d.redMask = v[0];
    d.greenMask = v[1];
    d.blueMask = v[2];
    d.alphaMask = v[3];
}

// Channel `index` (0 to 3) as component, representation, bits and signedness.
// False if `index` is out of range.
inline bool render_data_channel(
    const nifly::TextureRenderData& d,
    size_t index,
    rust::Slice<uint32_t> out
) {
    if (index >= 4)
        return false;
    const auto& c = d.channels[index];
    out[0] = c.type;
    out[1] = c.convention;
    out[2] = c.bitsPerChannel;
    out[3] = c.isSigned;
    return true;
}

inline bool set_render_data_channel(
    nifly::TextureRenderData& d,
    size_t index,
    rust::Slice<const uint32_t> v
) {
    if (index >= 4)
        return false;
    auto& c = d.channels[index];
    c.type = static_cast<nifly::PixelComponent>(v[0]);
    c.convention = static_cast<nifly::PixelRepresentation>(v[1]);
    c.bitsPerChannel = static_cast<uint8_t>(v[2]);
    c.isSigned = v[3] != 0;
    return true;
}

// Mipmaps flattened as width, height, offset.
inline rust::Vec<uint32_t>
render_data_mipmaps(const nifly::TextureRenderData& d) {
    rust::Vec<uint32_t> out;
    auto& mipmaps = const_cast<nifly::TextureRenderData&>(d).mipmaps;
    for (const auto& m : mipmaps) {
        out.push_back(m.width);
        out.push_back(m.height);
        out.push_back(m.offset);
    }
    return out;
}

inline void set_render_data_mipmaps(
    nifly::TextureRenderData& d,
    rust::Slice<const uint32_t> v
) {
    d.mipmaps.clear();
    for (size_t i = 0; i + 3 <= v.size(); i += 3) {
        nifly::MipMapInfo m;
        m.width = v[i];
        m.height = v[i + 1];
        m.offset = v[i + 2];
        d.mipmaps.push_back(m);
    }
}

inline uint32_t render_data_palette(const nifly::TextureRenderData& d) {
    return d.paletteRef.index;
}

// Pixel bytes of NiPixelData and NiPersistentSrcTextureRendererData, per face.
// Setters keep numPixels and numFaces in step.
template<typename T>
size_t pixel_face_count(const T& d) {
    return d.pixelData.size();
}

template<typename T>
rust::Slice<const uint8_t> pixel_face(const T& d, size_t face) {
    if (face >= d.pixelData.size())
        return {};
    return {d.pixelData[face].data(), d.pixelData[face].size()};
}

template<typename T>
void clear_pixel_faces(T& d) {
    d.pixelData.clear();
    d.numFaces = 0;
    d.numPixels = 0;
}

// The caller makes every face the same length.
template<typename T>
void push_pixel_face(T& d, rust::Slice<const uint8_t> bytes) {
    d.pixelData.emplace_back(bytes.begin(), bytes.end());
    d.numFaces = static_cast<uint32_t>(d.pixelData.size());
    d.numPixels = static_cast<uint32_t>(bytes.size());
}

#define NIFLY_RS_PIXELS(T, snake) \
    inline size_t snake##_face_count(const nifly::T& d) { \
        return pixel_face_count(d); \
    } \
    inline rust::Slice<const uint8_t> snake##_face( \
        const nifly::T& d, \
        size_t face \
    ) { \
        return pixel_face(d, face); \
    } \
    inline void clear_##snake##_faces(nifly::T& d) { \
        clear_pixel_faces(d); \
    } \
    inline void push_##snake##_face( \
        nifly::T& d, \
        rust::Slice<const uint8_t> bytes \
    ) { \
        push_pixel_face(d, bytes); \
    }
NIFLY_RS_PIXELS(NiPixelData, pixel_data)
NIFLY_RS_PIXELS(NiPersistentSrcTextureRendererData, persistent_data)
#undef NIFLY_RS_PIXELS

inline uint32_t
persistent_data_platform(const nifly::NiPersistentSrcTextureRendererData& d) {
    return d.platform;
}

inline void set_persistent_data_platform(
    nifly::NiPersistentSrcTextureRendererData& d,
    uint32_t v
) {
    d.platform = static_cast<nifly::PlatformID>(v);
}

inline uint32_t persistent_data_padded_pixel_count(
    const nifly::NiPersistentSrcTextureRendererData& d
) {
    return d.padNumPixels;
}

inline void set_persistent_data_padded_pixel_count(
    nifly::NiPersistentSrcTextureRendererData& d,
    uint32_t v
) {
    d.padNumPixels = v;
}

inline bool palette_has_alpha(const nifly::NiPalette& p) {
    return p.hasAlpha;
}

inline void set_palette_has_alpha(nifly::NiPalette& p, bool v) {
    p.hasAlpha = v;
}

// Colors flattened as r, g, b, a.
inline rust::Vec<uint8_t> palette_colors(const nifly::NiPalette& p) {
    rust::Vec<uint8_t> out;
    for (const auto& c : const_cast<nifly::NiPalette&>(p).palette) {
        out.push_back(c.r);
        out.push_back(c.g);
        out.push_back(c.b);
        out.push_back(c.a);
    }
    return out;
}

// False, changing nothing, unless there are 16 or 256 colors.
inline bool
set_palette_colors(nifly::NiPalette& p, rust::Slice<const uint8_t> rgba) {
    size_t n = rgba.size() / 4;
    if (rgba.size() % 4 != 0 || (n != 16 && n != 256))
        return false;
    p.palette.resize(static_cast<uint32_t>(n));
    for (size_t i = 0; i < n; i++) {
        auto& c = p.palette[static_cast<uint32_t>(i)];
        c.r = rgba[i * 4];
        c.g = rgba[i * 4 + 1];
        c.b = rgba[i * 4 + 2];
        c.a = rgba[i * 4 + 3];
    }
    return true;
}

// The block index of the shader's texture set ref; NIF_NPOS if the shader type
// has none or it's empty. Only BSLightingShaderProperty and
// BSShaderPPLightingProperty have one.
inline uint32_t shader_texture_set(const nifly::NiShader& shader) {
    auto ref = shader.TextureSetRef();
    return ref ? ref->index : nifly::NIF_NPOS;
}

#undef NIFLY_RS_STRING_FIELD
#undef NIFLY_RS_ENUM_FIELD
#undef NIFLY_RS_PROPERTY
#undef NIFLY_RS_SOURCE_ENUM
#undef NIFLY_RS_SOURCE
#undef NIFLY_RS_EFFECT_TEXTURE
#undef NIFLY_RS_EFFECT
#undef NIFLY_RS_LIGHTING_COLOR
#undef NIFLY_RS_LIGHTING
#undef NIFLY_RS_SAME
#undef NIFLY_RS_FIELD
} // namespace nifly_rs

#undef NIFLY_RS_SHADER
