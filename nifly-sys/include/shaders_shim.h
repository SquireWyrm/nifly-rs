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
} // namespace nifly_rs

// The rest of the shims, by topic.
// In this order: later parts use earlier ones.
// clang-format off
#include "shaders_shim_texturing.h"
#include "shaders_shim_bethesda.h"
#include "shaders_shim_properties.h"
#include "shaders_shim_textures.h"
// clang-format on

#undef NIFLY_RS_SHADER
