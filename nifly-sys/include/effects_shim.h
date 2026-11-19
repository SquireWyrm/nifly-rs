#pragma once
#include <cstdint>
#include <memory>

#include "Factory.hpp"
#include "NifFile.hpp"
#include "Objects.hpp"
#include "nifly-sys/src/object_3d.rs.h"
#include "rust/cxx.h"

// For each effect type T with direct base Parent, defines:
// - upcast_<snake>: T -> Parent. Implicit derived-to-base conversion, always succeeds.
// - downcast_<snake>: NiObject -> T. Returns null unless obj is a T or a subclass of T.
#define NIFLY_RS_EFFECT(T, Parent, snake) \
    inline const nifly::Parent& upcast_##snake(const nifly::T& obj) { \
        return obj; \
    } \
    inline const nifly::T* downcast_##snake(const nifly::NiObject& obj) { \
        return dynamic_cast<const nifly::T*>(&obj); \
    }

// A getter and setter for a plain field.
#define NIFLY_RS_EFFECT_FIELD(T, type, field, snake) \
    inline type snake(const nifly::T& obj) { \
        return obj.field; \
    } \
    inline void set_##snake(nifly::T& obj, type value) { \
        obj.field = value; \
    }

// A getter and setter for a Vector3 or Color3 field, as Vec3.
// `m0`..`m2` name the field's members.
#define NIFLY_RS_EFFECT_VEC3(T, field, snake, m0, m1, m2) \
    inline Vec3 snake(const nifly::T& obj) { \
        return {obj.field.m0, obj.field.m1, obj.field.m2}; \
    } \
    inline void set_##snake(nifly::T& obj, Vec3 v) { \
        obj.field.m0 = v.x; \
        obj.field.m1 = v.y; \
        obj.field.m2 = v.z; \
    }

namespace nifly_rs {
NIFLY_RS_EFFECT(NiDynamicEffect, NiAVObject, ni_dynamic_effect)
NIFLY_RS_EFFECT(NiTextureEffect, NiDynamicEffect, ni_texture_effect)
NIFLY_RS_EFFECT(NiLight, NiDynamicEffect, ni_light)
NIFLY_RS_EFFECT(NiAmbientLight, NiLight, ni_ambient_light)
NIFLY_RS_EFFECT(NiDirectionalLight, NiLight, ni_directional_light)
NIFLY_RS_EFFECT(NiPointLight, NiLight, ni_point_light)
NIFLY_RS_EFFECT(NiSpotLight, NiPointLight, ni_spot_light)

// NiSpotLight syncs its inner spot angle in every version, but it's only
// stored from 20.2.0.5 on: older files are misread from there. SpotLight syncs
// it by version; SpotLightFactory loads every NiSpotLight as one, and the shims
// below create them. Clones stay SpotLights.
class SpotLight final: public nifly::NiSpotLight {
  public:
    void Get(nifly::NiIStream& stream) override {
        nifly::NiPointLight::Get(stream);
        nifly::NiStreamReversible s(
            &stream,
            nullptr,
            nifly::NiStreamReversible::Mode::Reading
        );
        sync(s);
    }

    void Put(nifly::NiOStream& stream) override {
        nifly::NiPointLight::Put(stream);
        nifly::NiStreamReversible s(
            nullptr,
            &stream,
            nifly::NiStreamReversible::Mode::Writing
        );
        sync(s);
    }

  private:
    SpotLight* Clone_impl() const override {
        return new SpotLight(*this);
    }

    void sync(nifly::NiStreamReversible& s) {
        s.Sync(outerSpotAngle);
        if (s.GetVersion().File() >= nifly::NiFileVersion::V20_2_0_5)
            s.Sync(innerSpotAngle);
        s.Sync(exponent);
    }
};

class SpotLightFactory final: public nifly::NiFactory {
  public:
    std::unique_ptr<nifly::NiObject> Create() override {
        return std::make_unique<SpotLight>();
    }

    std::unique_ptr<nifly::NiObject> Load(nifly::NiIStream& stream) override {
        auto light = std::make_unique<SpotLight>();
        light->Get(stream);
        return light;
    }
};

// Reads a ref array's block indices; GetBlockRef isn't const.
template<typename Refs>
inline rust::Vec<uint32_t> ref_indices(const Refs& refs) {
    rust::Vec<uint32_t> out;
    out.reserve(refs.GetSize());
    for (uint32_t i = 0; i < refs.GetSize(); i++)
        out.push_back(refs.GetBlockRef(i));
    return out;
}

NIFLY_RS_EFFECT_FIELD(NiDynamicEffect, bool, switchState, effect_switch_state)

inline rust::Vec<uint32_t>
effect_affected_nodes(const nifly::NiDynamicEffect& e) {
    return ref_indices(e.affectedNodes);
}

// NiVector has no const iteration, so this casts away const to read.
inline rust::Vec<uint32_t>
effect_affected_node_pointers(const nifly::NiDynamicEffect& e) {
    rust::Vec<uint32_t> out;
    for (uint32_t p :
         const_cast<nifly::NiDynamicEffect&>(e).affectedNodePointers)
        out.push_back(p);
    return out;
}

inline void set_effect_affected_node_pointers(
    nifly::NiDynamicEffect& e,
    rust::Slice<const uint32_t> v
) {
    e.affectedNodePointers.clear();
    for (uint32_t p : v)
        e.affectedNodePointers.push_back(p);
}

inline void
texture_effect_matrix(const nifly::NiTextureEffect& e, rust::Slice<Vec3> out) {
    for (int r = 0; r < 3; r++) {
        const auto& row = e.modelProjectionMatrix[r];
        out[r] = {row.x, row.y, row.z};
    }
}

inline void set_texture_effect_matrix(
    nifly::NiTextureEffect& e,
    rust::Slice<const Vec3> rows
) {
    for (int r = 0; r < 3; r++)
        e.modelProjectionMatrix[r] =
            nifly::Vector3(rows[r].x, rows[r].y, rows[r].z);
}

NIFLY_RS_EFFECT_VEC3(
    NiTextureEffect,
    modelProjectionTranslation,
    texture_effect_translation,
    x,
    y,
    z
)

inline uint32_t texture_effect_filtering(const nifly::NiTextureEffect& e) {
    return e.textureFiltering;
}

inline void
set_texture_effect_filtering(nifly::NiTextureEffect& e, uint32_t v) {
    e.textureFiltering = static_cast<nifly::TexFilterMode>(v);
}

inline uint32_t texture_effect_clamping(const nifly::NiTextureEffect& e) {
    return e.textureClamping;
}

inline void set_texture_effect_clamping(nifly::NiTextureEffect& e, uint32_t v) {
    e.textureClamping = static_cast<nifly::TexClampMode>(v);
}

inline uint32_t texture_effect_type(const nifly::NiTextureEffect& e) {
    return e.textureType;
}

inline void set_texture_effect_type(nifly::NiTextureEffect& e, uint32_t v) {
    e.textureType = static_cast<nifly::EffectType>(v);
}

inline uint32_t texture_effect_coordinates(const nifly::NiTextureEffect& e) {
    return e.coordinateGenerationType;
}

inline void
set_texture_effect_coordinates(nifly::NiTextureEffect& e, uint32_t v) {
    e.coordinateGenerationType = static_cast<nifly::CoordGenType>(v);
}

inline uint32_t texture_effect_source(const nifly::NiTextureEffect& e) {
    return e.sourceTexture.index;
}

NIFLY_RS_EFFECT_FIELD(
    NiTextureEffect,
    uint8_t,
    clippingPlane,
    texture_effect_clipping
)
NIFLY_RS_EFFECT_VEC3(
    NiTextureEffect,
    plane.normal,
    texture_effect_plane_normal,
    x,
    y,
    z
)
NIFLY_RS_EFFECT_FIELD(
    NiTextureEffect,
    float,
    plane.constant,
    texture_effect_plane_constant
)
NIFLY_RS_EFFECT_FIELD(NiTextureEffect, int16_t, ps2_l, texture_effect_ps2_l)
NIFLY_RS_EFFECT_FIELD(NiTextureEffect, int16_t, ps2_k, texture_effect_ps2_k)
NIFLY_RS_EFFECT_FIELD(
    NiTextureEffect,
    uint16_t,
    unkShort1,
    texture_effect_unknown
)

NIFLY_RS_EFFECT_FIELD(NiLight, float, dimmer, light_dimmer)
NIFLY_RS_EFFECT_VEC3(NiLight, ambientColor, light_ambient, r, g, b)
NIFLY_RS_EFFECT_VEC3(NiLight, diffuseColor, light_diffuse, r, g, b)
NIFLY_RS_EFFECT_VEC3(NiLight, specularColor, light_specular, r, g, b)
NIFLY_RS_EFFECT_FIELD(
    NiPointLight,
    float,
    constantAttenuation,
    point_light_constant
)
NIFLY_RS_EFFECT_FIELD(
    NiPointLight,
    float,
    linearAttenuation,
    point_light_linear
)
NIFLY_RS_EFFECT_FIELD(
    NiPointLight,
    float,
    quadraticAttenuation,
    point_light_quadratic
)
NIFLY_RS_EFFECT_FIELD(NiSpotLight, float, outerSpotAngle, spot_light_outer)
NIFLY_RS_EFFECT_FIELD(NiSpotLight, float, innerSpotAngle, spot_light_inner)
NIFLY_RS_EFFECT_FIELD(NiSpotLight, float, exponent, spot_light_exponent)

inline rust::Vec<uint32_t> node_effects(const nifly::NiNode& node) {
    return ref_indices(node.effectRefs);
}

// Replaces `refs` with `indices` if every one is a T; otherwise returns false.
template<typename T, typename Refs>
inline bool set_refs_of(
    nifly::NiHeader& hdr,
    Refs& refs,
    rust::Slice<const uint32_t> indices
) {
    for (uint32_t i : indices)
        if (!hdr.GetBlock<T>(i))
            return false;
    refs.Clear();
    for (uint32_t i : indices)
        refs.AddBlockRef(i);
    return true;
}

inline bool set_node_effects(
    nifly::NifFile& file,
    uint32_t node_index,
    rust::Slice<const uint32_t> effects
) {
    auto& hdr = file.GetHeader();
    auto node = hdr.GetBlock<nifly::NiNode>(node_index);
    return node
        && set_refs_of<nifly::NiDynamicEffect>(hdr, node->effectRefs, effects);
}

inline bool set_effect_affected_nodes(
    nifly::NifFile& file,
    uint32_t effect_index,
    rust::Slice<const uint32_t> nodes
) {
    auto& hdr = file.GetHeader();
    auto effect = hdr.GetBlock<nifly::NiDynamicEffect>(effect_index);
    return effect
        && set_refs_of<nifly::NiNode>(hdr, effect->affectedNodes, nodes);
}

inline bool set_texture_effect_source(
    nifly::NifFile& file,
    uint32_t effect_index,
    uint32_t texture
) {
    auto& hdr = file.GetHeader();
    auto effect = hdr.GetBlock<nifly::NiTextureEffect>(effect_index);
    if (!effect)
        return false;
    if (texture != nifly::NIF_NPOS
        && !hdr.GetBlock<nifly::NiSourceTexture>(texture))
        return false;
    effect->sourceTexture.index = texture;
    return true;
}

// Adds `effect` to the effects of the node at `node_index`; see add_texture_effect.
inline uint32_t add_effect(
    nifly::NifFile& file,
    uint32_t node_index,
    std::unique_ptr<nifly::NiDynamicEffect> effect
) {
    auto& hdr = file.GetHeader();
    auto node = hdr.GetBlock<nifly::NiNode>(node_index);
    if (!node)
        return nifly::NIF_NPOS;
    uint32_t id = hdr.AddBlock(std::move(effect));
    node->effectRefs.AddBlockRef(id);
    return id;
}

// New lights start at full brightness with linear falloff, as stored by
// default in the file format.
template<typename T>
inline std::unique_ptr<T> new_light() {
    auto light = std::make_unique<T>();
    light->dimmer = 1.0f;
    if constexpr (std::is_base_of_v<nifly::NiPointLight, T>)
        light->linearAttenuation = 1.0f;
    return light;
}

inline uint32_t add_texture_effect(nifly::NifFile& file, uint32_t node) {
    return add_effect(file, node, std::make_unique<nifly::NiTextureEffect>());
}

inline uint32_t add_ambient_light(nifly::NifFile& file, uint32_t node) {
    return add_effect(file, node, new_light<nifly::NiAmbientLight>());
}

inline uint32_t add_directional_light(nifly::NifFile& file, uint32_t node) {
    return add_effect(file, node, new_light<nifly::NiDirectionalLight>());
}

inline uint32_t add_point_light(nifly::NifFile& file, uint32_t node) {
    return add_effect(file, node, new_light<nifly::NiPointLight>());
}

inline uint32_t add_spot_light(nifly::NifFile& file, uint32_t node) {
    return add_effect(file, node, new_light<SpotLight>());
}
} // namespace nifly_rs
