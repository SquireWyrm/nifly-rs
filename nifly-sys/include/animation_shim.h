#pragma once
#include <array>
#include <cstdint>
#include <cstring>
#include <memory>
#include <set>
#include <string>
#include <vector>

#include "Animation.hpp"
#include "NifFile.hpp"
#include "Particles.hpp"
#include "collision_shim.h"
#include "nifly-sys/src/animation.rs.h"
#include "nifly-sys/src/object_3d.rs.h"
#include "rust/cxx.h"

// For each animation type T with direct base Parent, defines:
// - upcast_<snake>: T -> Parent. Implicit derived-to-base conversion, always succeeds.
// - downcast_<snake>: NiObject -> T. Returns null unless obj is a T or a subclass of T.
#define NIFLY_RS_ANIMATION(T, Parent, snake) \
    inline const nifly::Parent& upcast_##snake(const nifly::T& obj) { \
        return obj; \
    } \
    inline const nifly::T* downcast_##snake(const nifly::NiObject& obj) { \
        return dynamic_cast<const nifly::T*>(&obj); \
    }

namespace nifly_rs {
NIFLY_RS_ANIMATION(NiKeyframeData, NiObject, ni_keyframe_data)
NIFLY_RS_ANIMATION(NiTransformData, NiKeyframeData, ni_transform_data)
NIFLY_RS_ANIMATION(NiPosData, NiObject, ni_pos_data)
NIFLY_RS_ANIMATION(NiBoolData, NiObject, ni_bool_data)
NIFLY_RS_ANIMATION(NiFloatData, NiObject, ni_float_data)
NIFLY_RS_ANIMATION(NiColorData, NiObject, ni_color_data)
NIFLY_RS_ANIMATION(NiUVData, NiObject, ni_uv_data)
NIFLY_RS_ANIMATION(NiVisData, NiObject, ni_vis_data)
NIFLY_RS_ANIMATION(NiMorphData, NiObject, ni_morph_data)
NIFLY_RS_ANIMATION(NiBSplineData, NiObject, ni_bspline_data)
NIFLY_RS_ANIMATION(NiBSplineBasisData, NiObject, ni_bspline_basis_data)
NIFLY_RS_ANIMATION(NiStringPalette, NiObject, ni_string_palette)
NIFLY_RS_ANIMATION(NiInterpolator, NiObject, ni_interpolator)
NIFLY_RS_ANIMATION(
    NiBSplineInterpolator,
    NiInterpolator,
    ni_bspline_interpolator
)
NIFLY_RS_ANIMATION(
    NiBSplineFloatInterpolator,
    NiBSplineInterpolator,
    ni_bspline_float_interpolator
)
NIFLY_RS_ANIMATION(
    NiBSplineCompFloatInterpolator,
    NiBSplineFloatInterpolator,
    ni_bspline_comp_float_interpolator
)
NIFLY_RS_ANIMATION(
    NiBSplinePoint3Interpolator,
    NiBSplineInterpolator,
    ni_bspline_point3_interpolator
)
NIFLY_RS_ANIMATION(
    NiBSplineCompPoint3Interpolator,
    NiBSplinePoint3Interpolator,
    ni_bspline_comp_point3_interpolator
)
NIFLY_RS_ANIMATION(
    NiBSplineTransformInterpolator,
    NiBSplineInterpolator,
    ni_bspline_transform_interpolator
)
NIFLY_RS_ANIMATION(
    NiBSplineCompTransformInterpolator,
    NiBSplineTransformInterpolator,
    ni_bspline_comp_transform_interpolator
)
NIFLY_RS_ANIMATION(NiBlendInterpolator, NiInterpolator, ni_blend_interpolator)
NIFLY_RS_ANIMATION(
    NiBlendBoolInterpolator,
    NiBlendInterpolator,
    ni_blend_bool_interpolator
)
NIFLY_RS_ANIMATION(
    NiBlendFloatInterpolator,
    NiBlendInterpolator,
    ni_blend_float_interpolator
)
NIFLY_RS_ANIMATION(
    NiBlendPoint3Interpolator,
    NiBlendInterpolator,
    ni_blend_point3_interpolator
)
NIFLY_RS_ANIMATION(
    NiBlendTransformInterpolator,
    NiBlendInterpolator,
    ni_blend_transform_interpolator
)
NIFLY_RS_ANIMATION(
    NiKeyBasedInterpolator,
    NiInterpolator,
    ni_key_based_interpolator
)
NIFLY_RS_ANIMATION(
    NiBoolInterpolator,
    NiKeyBasedInterpolator,
    ni_bool_interpolator
)
NIFLY_RS_ANIMATION(
    NiBoolTimelineInterpolator,
    NiBoolInterpolator,
    ni_bool_timeline_interpolator
)
NIFLY_RS_ANIMATION(
    NiFloatInterpolator,
    NiKeyBasedInterpolator,
    ni_float_interpolator
)
NIFLY_RS_ANIMATION(
    NiTransformInterpolator,
    NiKeyBasedInterpolator,
    ni_transform_interpolator
)
NIFLY_RS_ANIMATION(
    BSRotAccumTransfInterpolator,
    NiTransformInterpolator,
    bs_rot_accum_transf_interpolator
)
NIFLY_RS_ANIMATION(
    NiPoint3Interpolator,
    NiKeyBasedInterpolator,
    ni_point3_interpolator
)
NIFLY_RS_ANIMATION(
    NiPathInterpolator,
    NiKeyBasedInterpolator,
    ni_path_interpolator
)
NIFLY_RS_ANIMATION(
    NiLookAtInterpolator,
    NiInterpolator,
    ni_look_at_interpolator
)
NIFLY_RS_ANIMATION(
    BSTreadTransfInterpolator,
    NiInterpolator,
    bs_tread_transf_interpolator
)
NIFLY_RS_ANIMATION(NiTimeController, NiObject, ni_time_controller)
NIFLY_RS_ANIMATION(NiLookAtController, NiTimeController, ni_look_at_controller)
NIFLY_RS_ANIMATION(NiPathController, NiTimeController, ni_path_controller)
NIFLY_RS_ANIMATION(
    NiPSysResetOnLoopCtlr,
    NiTimeController,
    ni_psys_reset_on_loop_ctlr
)
NIFLY_RS_ANIMATION(NiUVController, NiTimeController, ni_uv_controller)
NIFLY_RS_ANIMATION(
    BSFrustumFOVController,
    NiTimeController,
    bs_frustum_fov_controller
)
NIFLY_RS_ANIMATION(
    BSLagBoneController,
    NiTimeController,
    bs_lag_bone_controller
)
NIFLY_RS_ANIMATION(
    BSProceduralLightningController,
    NiTimeController,
    bs_procedural_lightning_controller
)
NIFLY_RS_ANIMATION(
    NiBoneLODController,
    NiTimeController,
    ni_bone_lod_controller
)
NIFLY_RS_ANIMATION(
    NiBSBoneLODController,
    NiBoneLODController,
    ni_bs_bone_lod_controller
)
NIFLY_RS_ANIMATION(NiInterpController, NiTimeController, ni_interp_controller)
NIFLY_RS_ANIMATION(
    NiGeomMorpherController,
    NiInterpController,
    ni_geom_morpher_controller
)
NIFLY_RS_ANIMATION(
    NiSingleInterpController,
    NiInterpController,
    ni_single_interp_controller
)
NIFLY_RS_ANIMATION(
    NiRollController,
    NiSingleInterpController,
    ni_roll_controller
)
NIFLY_RS_ANIMATION(
    NiPoint3InterpController,
    NiSingleInterpController,
    ni_point3_interp_controller
)
NIFLY_RS_ANIMATION(
    NiMaterialColorController,
    NiPoint3InterpController,
    ni_material_color_controller
)
NIFLY_RS_ANIMATION(
    NiLightColorController,
    NiPoint3InterpController,
    ni_light_color_controller
)
NIFLY_RS_ANIMATION(
    NiExtraDataController,
    NiSingleInterpController,
    ni_extra_data_controller
)
NIFLY_RS_ANIMATION(
    NiFloatExtraDataController,
    NiExtraDataController,
    ni_float_extra_data_controller
)
NIFLY_RS_ANIMATION(
    NiBoolInterpController,
    NiSingleInterpController,
    ni_bool_interp_controller
)
NIFLY_RS_ANIMATION(NiVisController, NiBoolInterpController, ni_vis_controller)
NIFLY_RS_ANIMATION(
    NiFloatInterpController,
    NiSingleInterpController,
    ni_float_interp_controller
)
NIFLY_RS_ANIMATION(
    BSRefractionFirePeriodController,
    NiSingleInterpController,
    bs_refraction_fire_period_controller
)
NIFLY_RS_ANIMATION(
    NiFlipController,
    NiFloatInterpController,
    ni_flip_controller
)
NIFLY_RS_ANIMATION(
    NiTextureTransformController,
    NiFloatInterpController,
    ni_texture_transform_controller
)
NIFLY_RS_ANIMATION(
    NiLightDimmerController,
    NiFloatInterpController,
    ni_light_dimmer_controller
)
NIFLY_RS_ANIMATION(
    NiLightRadiusController,
    NiFloatInterpController,
    ni_light_radius_controller
)
NIFLY_RS_ANIMATION(
    NiAlphaController,
    NiFloatInterpController,
    ni_alpha_controller
)
NIFLY_RS_ANIMATION(NiPSysUpdateCtlr, NiTimeController, ni_psys_update_ctlr)
NIFLY_RS_ANIMATION(
    BSNiAlphaPropertyTestRefController,
    NiAlphaController,
    bs_ni_alpha_property_test_ref_controller
)
NIFLY_RS_ANIMATION(
    NiKeyframeController,
    NiSingleInterpController,
    ni_keyframe_controller
)
NIFLY_RS_ANIMATION(
    NiTransformController,
    NiKeyframeController,
    ni_transform_controller
)
NIFLY_RS_ANIMATION(
    BSMaterialEmittanceMultController,
    NiFloatInterpController,
    bs_material_emittance_mult_controller
)
NIFLY_RS_ANIMATION(
    BSRefractionStrengthController,
    NiFloatInterpController,
    bs_refraction_strength_controller
)
NIFLY_RS_ANIMATION(
    BSLightingShaderPropertyColorController,
    NiFloatInterpController,
    bs_lighting_shader_property_color_controller
)
NIFLY_RS_ANIMATION(
    BSLightingShaderPropertyFloatController,
    NiFloatInterpController,
    bs_lighting_shader_property_float_controller
)
NIFLY_RS_ANIMATION(
    BSLightingShaderPropertyUShortController,
    NiFloatInterpController,
    bs_lighting_shader_property_u_short_controller
)
NIFLY_RS_ANIMATION(
    BSEffectShaderPropertyColorController,
    NiFloatInterpController,
    bs_effect_shader_property_color_controller
)
NIFLY_RS_ANIMATION(
    BSEffectShaderPropertyFloatController,
    NiFloatInterpController,
    bs_effect_shader_property_float_controller
)
NIFLY_RS_ANIMATION(
    NiMultiTargetTransformController,
    NiInterpController,
    ni_multi_target_transform_controller
)
NIFLY_RS_ANIMATION(
    NiPSysModifierCtlr,
    NiSingleInterpController,
    ni_psys_modifier_ctlr
)
NIFLY_RS_ANIMATION(
    NiPSysModifierBoolCtlr,
    NiPSysModifierCtlr,
    ni_psys_modifier_bool_ctlr
)
NIFLY_RS_ANIMATION(
    NiPSysModifierActiveCtlr,
    NiPSysModifierBoolCtlr,
    ni_psys_modifier_active_ctlr
)
NIFLY_RS_ANIMATION(
    NiPSysModifierFloatCtlr,
    NiPSysModifierCtlr,
    ni_psys_modifier_float_ctlr
)
NIFLY_RS_ANIMATION(
    NiPSysEmitterLifeSpanCtlr,
    NiPSysModifierFloatCtlr,
    ni_psys_emitter_life_span_ctlr
)
NIFLY_RS_ANIMATION(
    NiPSysEmitterSpeedCtlr,
    NiPSysModifierFloatCtlr,
    ni_psys_emitter_speed_ctlr
)
NIFLY_RS_ANIMATION(
    NiPSysEmitterInitialRadiusCtlr,
    NiPSysModifierFloatCtlr,
    ni_psys_emitter_initial_radius_ctlr
)
NIFLY_RS_ANIMATION(
    NiPSysEmitterDeclinationCtlr,
    NiPSysModifierFloatCtlr,
    ni_psys_emitter_declination_ctlr
)
NIFLY_RS_ANIMATION(
    NiPSysGravityStrengthCtlr,
    NiPSysModifierFloatCtlr,
    ni_psys_gravity_strength_ctlr
)
NIFLY_RS_ANIMATION(
    NiPSysEmitterDeclinationVarCtlr,
    NiPSysModifierFloatCtlr,
    ni_psys_emitter_declination_var_ctlr
)
NIFLY_RS_ANIMATION(
    NiPSysFieldMagnitudeCtlr,
    NiPSysModifierFloatCtlr,
    ni_psys_field_magnitude_ctlr
)
NIFLY_RS_ANIMATION(
    NiPSysFieldAttenuationCtlr,
    NiPSysModifierFloatCtlr,
    ni_psys_field_attenuation_ctlr
)
NIFLY_RS_ANIMATION(
    NiPSysFieldMaxDistanceCtlr,
    NiPSysModifierFloatCtlr,
    ni_psys_field_max_distance_ctlr
)
NIFLY_RS_ANIMATION(
    NiPSysAirFieldAirFrictionCtlr,
    NiPSysModifierFloatCtlr,
    ni_psys_air_field_air_friction_ctlr
)
NIFLY_RS_ANIMATION(
    NiPSysAirFieldInheritVelocityCtlr,
    NiPSysModifierFloatCtlr,
    ni_psys_air_field_inherit_velocity_ctlr
)
NIFLY_RS_ANIMATION(
    NiPSysAirFieldSpreadCtlr,
    NiPSysModifierFloatCtlr,
    ni_psys_air_field_spread_ctlr
)
NIFLY_RS_ANIMATION(
    NiPSysInitialRotSpeedCtlr,
    NiPSysModifierFloatCtlr,
    ni_psys_initial_rot_speed_ctlr
)
NIFLY_RS_ANIMATION(
    NiPSysInitialRotSpeedVarCtlr,
    NiPSysModifierFloatCtlr,
    ni_psys_initial_rot_speed_var_ctlr
)
NIFLY_RS_ANIMATION(
    NiPSysInitialRotAngleCtlr,
    NiPSysModifierFloatCtlr,
    ni_psys_initial_rot_angle_ctlr
)
NIFLY_RS_ANIMATION(
    NiPSysInitialRotAngleVarCtlr,
    NiPSysModifierFloatCtlr,
    ni_psys_initial_rot_angle_var_ctlr
)
NIFLY_RS_ANIMATION(
    NiPSysEmitterPlanarAngleCtlr,
    NiPSysModifierFloatCtlr,
    ni_psys_emitter_planar_angle_ctlr
)
NIFLY_RS_ANIMATION(
    NiPSysEmitterPlanarAngleVarCtlr,
    NiPSysModifierFloatCtlr,
    ni_psys_emitter_planar_angle_var_ctlr
)
NIFLY_RS_ANIMATION(
    NiPSysRotDampeningCtlr,
    NiPSysModifierFloatCtlr,
    ni_psys_rot_dampening_ctlr
)
NIFLY_RS_ANIMATION(NiSequence, NiObject, ni_sequence)
NIFLY_RS_ANIMATION(BSAnimNote, NiObject, bs_anim_note)
NIFLY_RS_ANIMATION(BSAnimNotes, NiObject, bs_anim_notes)
NIFLY_RS_ANIMATION(NiControllerSequence, NiSequence, ni_controller_sequence)
NIFLY_RS_ANIMATION(NiControllerManager, NiTimeController, ni_controller_manager)
NIFLY_RS_ANIMATION(bhkBlendController, NiTimeController, bhk_blend_controller)

// Key values as four floats.
inline std::array<float, 4> key_value(float v) {
    return {v, 0.0f, 0.0f, 0.0f};
}

inline std::array<float, 4> key_value(uint8_t v) {
    return {float(v), 0.0f, 0.0f, 0.0f};
}

inline std::array<float, 4> key_value(const nifly::Vector3& v) {
    return {v.x, v.y, v.z, 0.0f};
}

inline std::array<float, 4> key_value(const nifly::Color4& c) {
    return {c.r, c.g, c.b, c.a};
}

inline std::array<float, 4> key_value(const nifly::Quaternion& q) {
    return {q.x, q.y, q.z, q.w};
}

inline void key_value_from(const std::array<float, 4>& a, float& v) {
    v = a[0];
}

inline void key_value_from(const std::array<float, 4>& a, uint8_t& v) {
    v = static_cast<uint8_t>(a[0]);
}

inline void key_value_from(const std::array<float, 4>& a, nifly::Vector3& v) {
    v = nifly::Vector3(a[0], a[1], a[2]);
}

inline void key_value_from(const std::array<float, 4>& a, nifly::Color4& c) {
    c = nifly::Color4(a[0], a[1], a[2], a[3]);
}

inline void
key_value_from(const std::array<float, 4>& a, nifly::Quaternion& q) {
    q.x = a[0];
    q.y = a[1];
    q.z = a[2];
    q.w = a[3];
}

template<class T>
KeyFfi key_to_ffi(const nifly::NiAnimationKey<T>& k) {
    return {
        k.time,
        key_value(k.value),
        key_value(k.forward),
        key_value(k.backward),
        {k.tbc.tension, k.tbc.bias, k.tbc.continuity}
    };
}

template<class T>
nifly::NiAnimationKey<T> key_from_ffi(const KeyFfi& f, nifly::NiKeyType type) {
    nifly::NiAnimationKey<T> k;
    k.type = type;
    k.time = f.time;
    key_value_from(f.value, k.value);
    key_value_from(f.forward, k.forward);
    key_value_from(f.backward, k.backward);
    k.tbc.tension = f.tbc[0];
    k.tbc.bias = f.tbc[1];
    k.tbc.continuity = f.tbc[2];
    return k;
}

// Linear, quadratic, TBC or constant; zero too for a group without keys.
inline bool valid_interpolation(uint32_t interpolation, bool has_keys) {
    switch (interpolation) {
        case nifly::LINEAR_KEY:
        case nifly::QUADRATIC_KEY:
        case nifly::TBC_KEY:
        case nifly::CONST_KEY:
            return true;
        case nifly::NO_INTERP:
            return !has_keys;
        default:
            return false;
    }
}

inline bool valid_group(const KeyGroupFfi& g) {
    return valid_interpolation(g.interpolation, !g.keys.empty());
}

template<class T>
KeyGroupFfi group_to_ffi(const nifly::NiAnimationKeyGroup<T>& g) {
    KeyGroupFfi out;
    out.interpolation = g.GetInterpolationType();
    for (uint32_t i = 0; i < g.GetNumKeys(); i++)
        out.keys.push_back(key_to_ffi(g.GetKey(i)));
    return out;
}

// Replaces the group. Callers check valid_group first.
template<class T>
void group_from_ffi(nifly::NiAnimationKeyGroup<T>& g, const KeyGroupFfi& f) {
    auto type = static_cast<nifly::NiKeyType>(f.interpolation);
    g.ClearKeys();
    g.SetInterpolationType(type);
    for (const auto& k : f.keys)
        g.AddKey(key_from_ffi<T>(k, type));
}

template<class T>
bool set_group(nifly::NiAnimationKeyGroup<T>& g, const KeyGroupFfi& f) {
    if (!valid_group(f))
        return false;
    group_from_ffi(g, f);
    return true;
}

#define NIFLY_RS_KEY_GROUP(T, field, snake) \
    inline KeyGroupFfi snake(const nifly::T& obj) { \
        return group_to_ffi(obj.field); \
    } \
    inline bool set_##snake(nifly::T& obj, const KeyGroupFfi& group) { \
        return set_group(obj.field, group); \
    }

NIFLY_RS_KEY_GROUP(NiPosData, data, pos_data_keys)
NIFLY_RS_KEY_GROUP(NiFloatData, data, float_data_keys)
NIFLY_RS_KEY_GROUP(NiBoolData, data, bool_data_keys)
NIFLY_RS_KEY_GROUP(NiColorData, data, color_data_keys)
NIFLY_RS_KEY_GROUP(NiKeyframeData, translations, keyframe_translations)
NIFLY_RS_KEY_GROUP(NiKeyframeData, scales, keyframe_scales)

inline const nifly::NiAnimationKeyGroup<float>*
uv_group(const nifly::NiUVData& d, size_t which) {
    switch (which) {
        case 0:
            return &d.uTrans;
        case 1:
            return &d.vTrans;
        case 2:
            return &d.uScale;
        case 3:
            return &d.vScale;
        default:
            return nullptr;
    }
}

inline KeyGroupFfi uv_data_keys(const nifly::NiUVData& d, size_t which) {
    auto g = uv_group(d, which);
    return g ? group_to_ffi(*g) : KeyGroupFfi {};
}

inline bool
set_uv_data_keys(nifly::NiUVData& d, size_t which, const KeyGroupFfi& group) {
    auto g = const_cast<nifly::NiAnimationKeyGroup<float>*>(uv_group(d, which));
    return g && set_group(*g, group);
}

// NiKeyframeData rotations.
inline uint32_t keyframe_rotation_type(const nifly::NiKeyframeData& d) {
    return d.rotationType;
}

inline KeyGroupFfi keyframe_quaternion_keys(const nifly::NiKeyframeData& d) {
    KeyGroupFfi out;
    out.interpolation = d.rotationType;
    for (const auto& k : d.quaternionKeys)
        out.keys.push_back(key_to_ffi(k));
    return out;
}

inline bool set_keyframe_quaternion_keys(
    nifly::NiKeyframeData& d,
    const KeyGroupFfi& group
) {
    if (!valid_group(group))
        return false;
    auto type = static_cast<nifly::NiKeyType>(group.interpolation);
    d.rotationType = type;
    d.quaternionKeys.clear();
    for (const auto& k : group.keys)
        d.quaternionKeys.push_back(key_from_ffi<nifly::Quaternion>(k, type));
    d.xRotations.ClearKeys();
    d.yRotations.ClearKeys();
    d.zRotations.ClearKeys();
    return true;
}

inline KeyGroupFfi
keyframe_xyz_rotations(const nifly::NiKeyframeData& d, size_t axis) {
    switch (axis) {
        case 0:
            return group_to_ffi(d.xRotations);
        case 1:
            return group_to_ffi(d.yRotations);
        case 2:
            return group_to_ffi(d.zRotations);
        default:
            return {};
    }
}

inline bool set_keyframe_xyz_rotations(
    nifly::NiKeyframeData& d,
    const KeyGroupFfi& x,
    const KeyGroupFfi& y,
    const KeyGroupFfi& z
) {
    if (!valid_group(x) || !valid_group(y) || !valid_group(z))
        return false;
    group_from_ffi(d.xRotations, x);
    group_from_ffi(d.yRotations, y);
    group_from_ffi(d.zRotations, z);
    d.rotationType = nifly::XYZ_ROTATION_KEY;
    d.quaternionKeys.clear();
    return true;
}

// NiVisData.
inline rust::Vec<KeyFfi> vis_data_keys(const nifly::NiVisData& d) {
    rust::Vec<KeyFfi> out;
    for (const auto& k : const_items(d.keys))
        out.push_back(key_to_ffi(k));
    return out;
}

inline void
set_vis_data_keys(nifly::NiVisData& d, rust::Slice<const KeyFfi> keys) {
    d.keys.clear();
    for (const auto& k : keys) {
        auto key = key_from_ffi<uint8_t>(k, nifly::NO_INTERP);
        d.keys.push_back(key);
    }
}

// NiMorphData.
inline uint32_t morph_vertex_count(const nifly::NiMorphData& d) {
    return d.numVertices;
}

inline uint8_t morph_relative_targets(const nifly::NiMorphData& d) {
    return d.relativeTargets;
}

inline void set_morph_relative_targets(nifly::NiMorphData& d, uint8_t value) {
    d.relativeTargets = value;
}

inline rust::Vec<MorphFfi> morphs(const nifly::NiMorphData& d) {
    rust::Vec<MorphFfi> out;
    for (const auto& m : d.GetMorphs()) {
        MorphFfi f;
        for (char c : m.frameName.get())
            f.name.push_back(static_cast<uint8_t>(c));
        f.keys.interpolation = m.interpolation;
        for (const auto& k : m.keys)
            f.keys.keys.push_back(key_to_ffi(k));
        f.legacy_weight = m.legacyWeight;
        for (const auto& v : m.vectors)
            f.vectors.push_back(havok_vec3_to_ffi(v));
        out.push_back(std::move(f));
    }
    return out;
}

inline bool
set_morphs(nifly::NiMorphData& d, rust::Slice<const MorphFfi> values) {
    uint32_t count = d.numVertices;
    if (!values.empty())
        count = static_cast<uint32_t>(values[0].vectors.size());
    // A morph whose name is unchanged keeps its string reference, so an empty
    // name stays an empty string rather than becoming no string.
    auto old = d.GetMorphs();
    std::vector<nifly::Morph> list;
    for (const auto& f : values) {
        if (f.vectors.size() != count || !valid_group(f.keys))
            return false;
        nifly::Morph m;
        std::string name(
            reinterpret_cast<const char*>(f.name.data()),
            f.name.size()
        );
        if (list.size() < old.size()
            && old[list.size()].frameName.get() == name)
            m.frameName = old[list.size()].frameName;
        else
            m.frameName.get() = name;
        m.interpolation = static_cast<nifly::NiKeyType>(f.keys.interpolation);
        for (const auto& k : f.keys.keys)
            m.keys.push_back(key_from_ffi<float>(k, m.interpolation));
        m.legacyWeight = f.legacy_weight;
        for (const auto& v : f.vectors)
            m.vectors.push_back(havok_vec3_from_ffi(v));
        list.push_back(std::move(m));
    }
    d.SetMorphs(count, list);
    return true;
}

// NiBSplineData and NiBSplineBasisData.
inline rust::Slice<const float>
bspline_float_points(const nifly::NiBSplineData& d) {
    return {d.floatControlPoints.data(), d.floatControlPoints.size()};
}

inline void set_bspline_float_points(
    nifly::NiBSplineData& d,
    rust::Slice<const float> points
) {
    d.floatControlPoints.clear();
    for (float p : points)
        d.floatControlPoints.push_back(p);
}

inline rust::Slice<const int16_t>
bspline_short_points(const nifly::NiBSplineData& d) {
    return {d.shortControlPoints.data(), d.shortControlPoints.size()};
}

inline void set_bspline_short_points(
    nifly::NiBSplineData& d,
    rust::Slice<const int16_t> points
) {
    d.shortControlPoints.clear();
    for (int16_t p : points)
        d.shortControlPoints.push_back(p);
}

inline uint32_t bspline_basis_count(const nifly::NiBSplineBasisData& d) {
    return d.numControlPoints;
}

inline void
set_bspline_basis_count(nifly::NiBSplineBasisData& d, uint32_t count) {
    d.numControlPoints = count;
}

// NiStringPalette.
inline rust::Slice<const uint8_t>
string_palette(const nifly::NiStringPalette& p) {
    const auto& s = p.palette.get();
    return {reinterpret_cast<const uint8_t*>(s.data()), s.size()};
}

inline void set_string_palette(
    nifly::NiStringPalette& p,
    rust::Slice<const uint8_t> palette
) {
    // A final zero byte is kept as the string's null output, as when read.
    size_t size = palette.size();
    bool null_output = size > 0 && palette[size - 1] == 0;
    if (null_output)
        size--;
    p.palette.get().assign(reinterpret_cast<const char*>(palette.data()), size);
    p.palette.SetNullOutput(null_output);
}

#define NIFLY_RS_ADD_ANIMATION(T, snake) \
    inline uint32_t add_##snake(nifly::NifFile& file) { \
        return file.GetHeader().AddBlock(std::make_unique<nifly::T>()); \
    }

NIFLY_RS_ADD_ANIMATION(NiKeyframeData, ni_keyframe_data)
NIFLY_RS_ADD_ANIMATION(NiTransformData, ni_transform_data)
NIFLY_RS_ADD_ANIMATION(NiPosData, ni_pos_data)
NIFLY_RS_ADD_ANIMATION(NiBoolData, ni_bool_data)
NIFLY_RS_ADD_ANIMATION(NiFloatData, ni_float_data)
NIFLY_RS_ADD_ANIMATION(NiColorData, ni_color_data)
NIFLY_RS_ADD_ANIMATION(NiUVData, ni_uv_data)
NIFLY_RS_ADD_ANIMATION(NiVisData, ni_vis_data)
NIFLY_RS_ADD_ANIMATION(NiMorphData, ni_morph_data)
NIFLY_RS_ADD_ANIMATION(NiBSplineData, ni_bspline_data)
NIFLY_RS_ADD_ANIMATION(NiBSplineBasisData, ni_bspline_basis_data)
NIFLY_RS_ADD_ANIMATION(NiStringPalette, ni_string_palette)

// Private NiPathInterpolator fields (private_access.cpp).
nifly::PathFlags& path_flags_field(nifly::NiPathInterpolator& p);
int& path_bank_dir_field(nifly::NiPathInterpolator& p);
float& path_max_bank_angle_field(nifly::NiPathInterpolator& p);
float& path_smoothing_field(nifly::NiPathInterpolator& p);
uint16_t& path_follow_axis_field(nifly::NiPathInterpolator& p);
nifly::NiBlockRef<nifly::NiPosData>&
path_data_field(nifly::NiPathInterpolator& p);
nifly::NiBlockRef<nifly::NiFloatData>&
path_percent_field(nifly::NiPathInterpolator& p);

// The fields are only read through const shims, so the casts never write.
template<class T>
T& unconst(const T& v) {
    return const_cast<T&>(v);
}

// A getter and setter for a plain field, converting between the field's type
// and `type`.
#define NIFLY_RS_ANIMATION_FIELD(T, type, field, snake) \
    inline type snake(const nifly::T& obj) { \
        return static_cast<type>(obj.field); \
    } \
    inline void set_##snake(nifly::T& obj, type value) { \
        obj.field = static_cast<decltype(obj.field)>(value); \
    }

// The same for a private NiPathInterpolator field.
#define NIFLY_RS_PATH_FIELD(type, accessor, snake) \
    inline type snake(const nifly::NiPathInterpolator& obj) { \
        return static_cast<type>(accessor(unconst(obj))); \
    } \
    inline void set_##snake(nifly::NiPathInterpolator& obj, type value) { \
        auto& f = accessor(obj); \
        f = static_cast<std::remove_reference_t<decltype(f)>>(value); \
    }

inline QuatTransformFfi quat_transform_to_ffi(
    const nifly::Vector3& translation,
    const nifly::Quaternion& rotation,
    float scale
) {
    return {havok_vec3_to_ffi(translation), key_value(rotation), scale};
}

template<class T>
QuatTransformFfi quat_transform_to_ffi(const T& t) {
    return quat_transform_to_ffi(t.translation, t.rotation, t.scale);
}

template<class T>
void quat_transform_from_ffi(T& t, const QuatTransformFfi& f) {
    t.translation = havok_vec3_from_ffi(f.translation);
    key_value_from(f.rotation, t.rotation);
    t.scale = f.scale;
}

#define NIFLY_RS_QUAT_TRANSFORM(T, snake) \
    inline QuatTransformFfi snake(const nifly::T& obj) { \
        return quat_transform_to_ffi(obj); \
    } \
    inline void set_##snake(nifly::T& obj, const QuatTransformFfi& value) { \
        quat_transform_from_ffi(obj, value); \
    }

#define NIFLY_RS_ANIMATION_VEC3(T, field, snake) \
    inline Vec3 snake(const nifly::T& obj) { \
        return havok_vec3_to_ffi(obj.field); \
    } \
    inline void set_##snake(nifly::T& obj, Vec3 value) { \
        obj.field = havok_vec3_from_ffi(value); \
    }

// Key-based interpolators' pose values.
NIFLY_RS_ANIMATION_FIELD(
    NiBoolInterpolator,
    uint8_t,
    boolValue,
    bool_interpolator_value
)
NIFLY_RS_ANIMATION_FIELD(
    NiFloatInterpolator,
    float,
    floatValue,
    float_interpolator_value
)
NIFLY_RS_ANIMATION_VEC3(
    NiPoint3Interpolator,
    point3Value,
    point3_interpolator_value
)
NIFLY_RS_QUAT_TRANSFORM(NiTransformInterpolator, transform_interpolator_value)

// Blend interpolators.
// The value is a bool, but files store 2 for unset, so it's read and written as
// its byte.
inline uint8_t blend_bool_value(const nifly::NiBlendBoolInterpolator& obj) {
    static_assert(sizeof(obj.value) == 1);
    uint8_t byte;
    std::memcpy(&byte, &obj.value, 1);
    return byte;
}

inline void
set_blend_bool_value(nifly::NiBlendBoolInterpolator& obj, uint8_t value) {
    std::memcpy(&obj.value, &value, 1);
}
NIFLY_RS_ANIMATION_FIELD(
    NiBlendFloatInterpolator,
    float,
    value,
    blend_float_value
)
NIFLY_RS_ANIMATION_VEC3(NiBlendPoint3Interpolator, point, blend_point3_value)

inline QuatTransformFfi
blend_transform_value(const nifly::NiBlendTransformInterpolator& obj) {
    return quat_transform_to_ffi(obj.value);
}

inline void set_blend_transform_value(
    nifly::NiBlendTransformInterpolator& obj,
    const QuatTransformFfi& value
) {
    quat_transform_from_ffi(obj.value, value);
}

inline std::array<bool, 3>
blend_transform_valid(const nifly::NiBlendTransformInterpolator& obj) {
    return {
        obj.value.trsValid[0],
        obj.value.trsValid[1],
        obj.value.trsValid[2]
    };
}

inline void set_blend_transform_valid(
    nifly::NiBlendTransformInterpolator& obj,
    std::array<bool, 3> valid
) {
    for (size_t i = 0; i < 3; i++)
        obj.value.trsValid[i] = valid[i];
}

NIFLY_RS_ANIMATION_FIELD(NiBlendInterpolator, uint8_t, flags, blend_flags)
NIFLY_RS_ANIMATION_FIELD(
    NiBlendInterpolator,
    uint16_t,
    arraySize,
    blend_array_size
)
NIFLY_RS_ANIMATION_FIELD(
    NiBlendInterpolator,
    uint16_t,
    arrayGrowBy,
    blend_array_grow_by
)
NIFLY_RS_ANIMATION_FIELD(
    NiBlendInterpolator,
    float,
    weightThreshold,
    blend_weight_threshold
)
NIFLY_RS_ANIMATION_FIELD(
    NiBlendInterpolator,
    uint16_t,
    interpCount,
    blend_interp_count
)
NIFLY_RS_ANIMATION_FIELD(
    NiBlendInterpolator,
    uint8_t,
    singleIndex,
    blend_single_index
)
NIFLY_RS_ANIMATION_FIELD(
    NiBlendInterpolator,
    uint16_t,
    singleIndexShort,
    blend_single_index_16
)
NIFLY_RS_ANIMATION_FIELD(
    NiBlendInterpolator,
    int8_t,
    highPriority,
    blend_high_priority
)
NIFLY_RS_ANIMATION_FIELD(
    NiBlendInterpolator,
    int32_t,
    highPriorityInt,
    blend_high_priority_32
)
NIFLY_RS_ANIMATION_FIELD(
    NiBlendInterpolator,
    int8_t,
    nextHighPriority,
    blend_next_high_priority
)
NIFLY_RS_ANIMATION_FIELD(
    NiBlendInterpolator,
    int32_t,
    nextHighPriorityInt,
    blend_next_high_priority_32
)
NIFLY_RS_ANIMATION_FIELD(
    NiBlendInterpolator,
    float,
    singleTime,
    blend_single_time
)
NIFLY_RS_ANIMATION_FIELD(
    NiBlendInterpolator,
    float,
    highWeightsSum,
    blend_high_weights_sum
)
NIFLY_RS_ANIMATION_FIELD(
    NiBlendInterpolator,
    float,
    nextHighWeightsSum,
    blend_next_high_weights_sum
)
NIFLY_RS_ANIMATION_FIELD(
    NiBlendInterpolator,
    float,
    highEaseSpinner,
    blend_high_ease_spinner
)
NIFLY_RS_ANIMATION_FIELD(
    NiBlendInterpolator,
    bool,
    managerControlled,
    blend_manager_controlled
)
NIFLY_RS_ANIMATION_FIELD(
    NiBlendInterpolator,
    bool,
    onlyUseHighestWeight,
    blend_only_use_highest_weight
)

inline rust::Vec<BlendItemFfi>
blend_items(const nifly::NiBlendInterpolator& obj) {
    rust::Vec<BlendItemFfi> out;
    for (const auto& item : obj.interpItems)
        out.push_back(
            {item.interpolatorRef.index,
             item.weight,
             item.normalizedWeight,
             item.priority,
             item.priorityInt,
             item.easeSpinner}
        );
    return out;
}

// Path interpolators.
NIFLY_RS_PATH_FIELD(uint16_t, path_flags_field, path_flags)
NIFLY_RS_PATH_FIELD(int32_t, path_bank_dir_field, path_bank_direction)
NIFLY_RS_PATH_FIELD(float, path_max_bank_angle_field, path_max_bank_angle)
NIFLY_RS_PATH_FIELD(float, path_smoothing_field, path_smoothing)
NIFLY_RS_PATH_FIELD(uint16_t, path_follow_axis_field, path_follow_axis)

// Look-at interpolators.
NIFLY_RS_ANIMATION_FIELD(NiLookAtInterpolator, uint16_t, flags, look_at_flags)

inline rust::Slice<const uint8_t>
look_at_name(const nifly::NiLookAtInterpolator& obj) {
    const auto& s = obj.lookAtName.get();
    return {reinterpret_cast<const uint8_t*>(s.data()), s.size()};
}

inline void set_look_at_name(
    nifly::NiLookAtInterpolator& obj,
    rust::Slice<const uint8_t> name
) {
    obj.lookAtName.get().assign(
        reinterpret_cast<const char*>(name.data()),
        name.size()
    );
}

inline QuatTransformFfi
look_at_transform(const nifly::NiLookAtInterpolator& obj) {
    return quat_transform_to_ffi(obj.transform);
}

inline void set_look_at_transform(
    nifly::NiLookAtInterpolator& obj,
    const QuatTransformFfi& value
) {
    quat_transform_from_ffi(obj.transform, value);
}

inline std::array<bool, 3>
look_at_transform_valid(const nifly::NiLookAtInterpolator& obj) {
    return {
        obj.transform.trsValid[0],
        obj.transform.trsValid[1],
        obj.transform.trsValid[2]
    };
}

inline void set_look_at_transform_valid(
    nifly::NiLookAtInterpolator& obj,
    std::array<bool, 3> valid
) {
    for (size_t i = 0; i < 3; i++)
        obj.transform.trsValid[i] = valid[i];
}

// BSTreadTransfInterpolator.
inline rust::Vec<TreadFfi> treads(const nifly::BSTreadTransfInterpolator& obj) {
    rust::Vec<TreadFfi> out;
    for (const auto& t : const_items(obj.treadTransforms)) {
        TreadFfi f;
        for (char c : t.name.get())
            f.name.push_back(static_cast<uint8_t>(c));
        f.transform_1 = quat_transform_to_ffi(t.transform1);
        f.transform_2 = quat_transform_to_ffi(t.transform2);
        out.push_back(std::move(f));
    }
    return out;
}

// A tread whose name is unchanged keeps its string reference, as in
// set_text_keys.
inline void set_treads(
    nifly::BSTreadTransfInterpolator& obj,
    rust::Slice<const TreadFfi> values
) {
    std::vector<nifly::BSTreadTransform> old(
        obj.treadTransforms.begin(),
        obj.treadTransforms.end()
    );
    obj.treadTransforms.clear();
    for (size_t i = 0; i < values.size(); i++) {
        const auto& f = values[i];
        nifly::BSTreadTransform t;
        std::string name(
            reinterpret_cast<const char*>(f.name.data()),
            f.name.size()
        );
        if (i < old.size() && old[i].name.get() == name)
            t.name = old[i].name;
        else
            t.name.get() = name;
        quat_transform_from_ffi(t.transform1, f.transform_1);
        quat_transform_from_ffi(t.transform2, f.transform_2);
        obj.treadTransforms.push_back(t);
    }
}

// B-spline interpolators.
NIFLY_RS_ANIMATION_FIELD(
    NiBSplineInterpolator,
    float,
    startTime,
    bspline_start_time
)
NIFLY_RS_ANIMATION_FIELD(
    NiBSplineInterpolator,
    float,
    stopTime,
    bspline_stop_time
)
NIFLY_RS_ANIMATION_FIELD(
    NiBSplineCompFloatInterpolator,
    float,
    base,
    bspline_float_base
)
NIFLY_RS_ANIMATION_FIELD(
    NiBSplineCompFloatInterpolator,
    uint32_t,
    offset,
    bspline_float_offset
)
NIFLY_RS_ANIMATION_FIELD(
    NiBSplineCompFloatInterpolator,
    float,
    bias,
    bspline_float_bias
)
NIFLY_RS_ANIMATION_FIELD(
    NiBSplineCompFloatInterpolator,
    float,
    multiplier,
    bspline_float_multiplier
)
NIFLY_RS_ANIMATION_VEC3(
    NiBSplinePoint3Interpolator,
    value,
    bspline_point3_value
)
NIFLY_RS_ANIMATION_FIELD(
    NiBSplinePoint3Interpolator,
    uint32_t,
    handle,
    bspline_point3_handle
)
NIFLY_RS_ANIMATION_FIELD(
    NiBSplineCompPoint3Interpolator,
    float,
    positionOffset,
    bspline_point3_offset
)
NIFLY_RS_ANIMATION_FIELD(
    NiBSplineCompPoint3Interpolator,
    float,
    positionHalfRange,
    bspline_point3_half_range
)
NIFLY_RS_QUAT_TRANSFORM(NiBSplineTransformInterpolator, bspline_transform_value)
NIFLY_RS_ANIMATION_FIELD(
    NiBSplineTransformInterpolator,
    uint32_t,
    translationOffset,
    bspline_translation_offset
)
NIFLY_RS_ANIMATION_FIELD(
    NiBSplineTransformInterpolator,
    uint32_t,
    rotationOffset,
    bspline_rotation_offset
)
NIFLY_RS_ANIMATION_FIELD(
    NiBSplineTransformInterpolator,
    uint32_t,
    scaleOffset,
    bspline_scale_offset
)
NIFLY_RS_ANIMATION_FIELD(
    NiBSplineCompTransformInterpolator,
    float,
    translationBias,
    bspline_translation_bias
)
NIFLY_RS_ANIMATION_FIELD(
    NiBSplineCompTransformInterpolator,
    float,
    translationMultiplier,
    bspline_translation_multiplier
)
NIFLY_RS_ANIMATION_FIELD(
    NiBSplineCompTransformInterpolator,
    float,
    rotationBias,
    bspline_rotation_bias
)
NIFLY_RS_ANIMATION_FIELD(
    NiBSplineCompTransformInterpolator,
    float,
    rotationMultiplier,
    bspline_rotation_multiplier
)
NIFLY_RS_ANIMATION_FIELD(
    NiBSplineCompTransformInterpolator,
    float,
    scaleBias,
    bspline_scale_bias
)
NIFLY_RS_ANIMATION_FIELD(
    NiBSplineCompTransformInterpolator,
    float,
    scaleMultiplier,
    bspline_scale_multiplier
)

// NiBlendInterpolator doesn't list its items' interpolator refs as child refs,
// so saving drops the interpolators as unreferenced and renumbering blocks
// leaves the refs stale. BlendInterpolatorFactory loads each blend type as a
// BlendInterpolator, which lists them, and the add shim creates them. Clones
// stay BlendInterpolators.
template<class Base>
class BlendInterpolator final: public Base {
  public:
    void GetChildRefs(std::set<nifly::NiRef*>& refs) override {
        Base::GetChildRefs(refs);
        for (auto& item : this->interpItems)
            refs.insert(&item.interpolatorRef);
    }

    void GetChildIndices(std::vector<uint32_t>& indices) override {
        Base::GetChildIndices(indices);
        for (auto& item : this->interpItems)
            indices.push_back(item.interpolatorRef.index);
    }

  private:
    BlendInterpolator* Clone_impl() const override {
        return new BlendInterpolator(*this);
    }
};

template<class Base>
class BlendInterpolatorFactory final: public nifly::NiFactory {
  public:
    std::unique_ptr<nifly::NiObject> Create() override {
        return std::make_unique<BlendInterpolator<Base>>();
    }

    std::unique_ptr<nifly::NiObject> Load(nifly::NiIStream& stream) override {
        auto obj = std::make_unique<BlendInterpolator<Base>>();
        obj->Get(stream);
        return obj;
    }
};

// Whether the interpolator at `target` is the one at `root` or used by it,
// through blend items, single interpolators and look-at sub-interpolators.
inline bool
interpolator_uses(const nifly::NiHeader& hdr, uint32_t root, uint32_t target) {
    using namespace nifly;
    std::vector<bool> seen(hdr.GetNumBlocks());
    std::vector<uint32_t> stack {root};
    while (!stack.empty()) {
        uint32_t id = stack.back();
        stack.pop_back();
        if (id == target)
            return true;
        if (id >= seen.size() || seen[id])
            continue;
        seen[id] = true;
        if (auto blend = hdr.GetBlock<NiBlendInterpolator>(id)) {
            stack.push_back(blend->singleInterpolatorRef.index);
            for (const auto& item : blend->interpItems)
                stack.push_back(item.interpolatorRef.index);
        }
        if (auto look = hdr.GetBlock<NiLookAtInterpolator>(id)) {
            stack.push_back(look->translateInterpRef.index);
            stack.push_back(look->rollInterpRef.index);
            stack.push_back(look->scaleInterpRef.index);
        }
    }
    return false;
}

// Replaces the blend interpolator's items. False, changing nothing, unless each
// interpolator is empty or an NiInterpolator that doesn't use the blend
// interpolator, and the version can store the count (a byte from 10.1.0.110).
// Where the items are saved (before 10.1.0.112, or without
// INTERP_BLEND_MANAGER_CONTROLLED), arraySize becomes their count.
inline bool set_blend_items(
    nifly::NifFile& file,
    uint32_t owner,
    rust::Slice<const BlendItemFfi> items
) {
    using namespace nifly;
    auto& hdr = file.GetHeader();
    auto blend = hdr.GetBlock<NiBlendInterpolator>(owner);
    if (!blend)
        return false;
    auto version = hdr.GetVersion().File();
    size_t limit = version >= V10_1_0_110 ? 0xFF : 0xFFFF;
    if (items.size() > limit)
        return false;
    for (const auto& item : items) {
        if (item.interpolator == NIF_NPOS)
            continue;
        if (!hdr.GetBlock<NiInterpolator>(item.interpolator)
            || interpolator_uses(hdr, item.interpolator, owner))
            return false;
    }
    blend->interpItems.clear();
    for (const auto& f : items) {
        InterpBlendItem item;
        item.interpolatorRef.index = f.interpolator;
        item.weight = f.weight;
        item.normalizedWeight = f.normalized_weight;
        item.priority = f.priority;
        item.priorityInt = f.priority_32;
        item.easeSpinner = f.ease_spinner;
        blend->interpItems.push_back(item);
    }
    bool saved = version < V10_1_0_112
        || (blend->flags & INTERP_BLEND_MANAGER_CONTROLLED) == 0;
    if (saved)
        blend->arraySize = static_cast<uint16_t>(items.size());
    return true;
}

#define NIFLY_RS_ADD_BLEND(T, snake) \
    inline uint32_t add_##snake(nifly::NifFile& file) { \
        return file.GetHeader().AddBlock( \
            std::make_unique<BlendInterpolator<nifly::T>>() \
        ); \
    }
NIFLY_RS_ADD_ANIMATION(NiBoolInterpolator, ni_bool_interpolator)
NIFLY_RS_ADD_ANIMATION(
    NiBoolTimelineInterpolator,
    ni_bool_timeline_interpolator
)
NIFLY_RS_ADD_ANIMATION(NiFloatInterpolator, ni_float_interpolator)
NIFLY_RS_ADD_ANIMATION(NiTransformInterpolator, ni_transform_interpolator)
NIFLY_RS_ADD_ANIMATION(
    BSRotAccumTransfInterpolator,
    bs_rot_accum_transf_interpolator
)
NIFLY_RS_ADD_ANIMATION(NiPoint3Interpolator, ni_point3_interpolator)
NIFLY_RS_ADD_ANIMATION(NiPathInterpolator, ni_path_interpolator)
NIFLY_RS_ADD_ANIMATION(NiLookAtInterpolator, ni_look_at_interpolator)
NIFLY_RS_ADD_ANIMATION(BSTreadTransfInterpolator, bs_tread_transf_interpolator)
NIFLY_RS_ADD_ANIMATION(
    NiBSplineCompFloatInterpolator,
    ni_bspline_comp_float_interpolator
)
NIFLY_RS_ADD_ANIMATION(
    NiBSplineCompPoint3Interpolator,
    ni_bspline_comp_point3_interpolator
)
NIFLY_RS_ADD_ANIMATION(
    NiBSplineTransformInterpolator,
    ni_bspline_transform_interpolator
)
NIFLY_RS_ADD_ANIMATION(
    NiBSplineCompTransformInterpolator,
    ni_bspline_comp_transform_interpolator
)
NIFLY_RS_ADD_BLEND(NiBlendBoolInterpolator, ni_blend_bool_interpolator)
NIFLY_RS_ADD_BLEND(NiBlendFloatInterpolator, ni_blend_float_interpolator)
NIFLY_RS_ADD_BLEND(NiBlendPoint3Interpolator, ni_blend_point3_interpolator)
NIFLY_RS_ADD_BLEND(
    NiBlendTransformInterpolator,
    ni_blend_transform_interpolator
)
// Controller types.

NIFLY_RS_ANIMATION_FIELD(
    NiLookAtController,
    uint16_t,
    lookAtFlags,
    look_at_controller_flags
)
NIFLY_RS_ANIMATION_FIELD(
    NiPathController,
    uint16_t,
    pathFlags,
    path_controller_flags
)
NIFLY_RS_ANIMATION_FIELD(
    NiPathController,
    int32_t,
    bankDir,
    path_controller_bank_direction
)
NIFLY_RS_ANIMATION_FIELD(
    NiPathController,
    float,
    maxBankAngle,
    path_controller_max_bank_angle
)
NIFLY_RS_ANIMATION_FIELD(
    NiPathController,
    float,
    smoothing,
    path_controller_smoothing
)
NIFLY_RS_ANIMATION_FIELD(
    NiPathController,
    uint16_t,
    followAxis,
    path_controller_follow_axis
)
NIFLY_RS_ANIMATION_FIELD(
    NiUVController,
    uint16_t,
    textureSet,
    uv_controller_uv_set
)
NIFLY_RS_ANIMATION_FIELD(
    BSLagBoneController,
    float,
    linearVelocity,
    lag_bone_linear_velocity
)
NIFLY_RS_ANIMATION_FIELD(
    BSLagBoneController,
    float,
    linearRotation,
    lag_bone_linear_rotation
)
NIFLY_RS_ANIMATION_FIELD(
    BSLagBoneController,
    float,
    maxDistance,
    lag_bone_max_distance
)
NIFLY_RS_ANIMATION_FIELD(
    BSProceduralLightningController,
    uint16_t,
    subdivisions,
    lightning_subdivisions
)
NIFLY_RS_ANIMATION_FIELD(
    BSProceduralLightningController,
    uint16_t,
    numBranches,
    lightning_branch_count
)
NIFLY_RS_ANIMATION_FIELD(
    BSProceduralLightningController,
    uint16_t,
    numBranchesPerVariation,
    lightning_branch_variation
)
NIFLY_RS_ANIMATION_FIELD(
    BSProceduralLightningController,
    float,
    length,
    lightning_length
)
NIFLY_RS_ANIMATION_FIELD(
    BSProceduralLightningController,
    float,
    lengthVariation,
    lightning_length_variation
)
NIFLY_RS_ANIMATION_FIELD(
    BSProceduralLightningController,
    float,
    width,
    lightning_width
)
NIFLY_RS_ANIMATION_FIELD(
    BSProceduralLightningController,
    float,
    childWidthMult,
    lightning_child_width_multiplier
)
NIFLY_RS_ANIMATION_FIELD(
    BSProceduralLightningController,
    float,
    arcOffset,
    lightning_arc_offset
)
NIFLY_RS_ANIMATION_FIELD(
    BSProceduralLightningController,
    bool,
    fadeMainBolt,
    lightning_fade_main_bolt
)
NIFLY_RS_ANIMATION_FIELD(
    BSProceduralLightningController,
    bool,
    fadeChildBolts,
    lightning_fade_child_bolts
)
NIFLY_RS_ANIMATION_FIELD(
    BSProceduralLightningController,
    bool,
    animateArcOffset,
    lightning_animate_arc_offset
)
NIFLY_RS_ANIMATION_FIELD(
    NiPoint3InterpController,
    uint16_t,
    targetColor,
    point3_controller_target_color
)
NIFLY_RS_ANIMATION_FIELD(
    NiFlipController,
    uint32_t,
    textureSlot,
    flip_texture_slot
)
NIFLY_RS_ANIMATION_FIELD(NiFlipController, float, accumTime, flip_accum_time)
NIFLY_RS_ANIMATION_FIELD(NiFlipController, float, delta, flip_delta)
NIFLY_RS_ANIMATION_FIELD(
    NiTextureTransformController,
    bool,
    shaderMap,
    texture_transform_shader_map
)
NIFLY_RS_ANIMATION_FIELD(
    NiTextureTransformController,
    uint32_t,
    textureSlot,
    texture_transform_slot
)
NIFLY_RS_ANIMATION_FIELD(
    NiTextureTransformController,
    uint32_t,
    operation,
    texture_transform_operation
)
NIFLY_RS_ANIMATION_FIELD(
    BSLightingShaderPropertyColorController,
    uint32_t,
    typeOfControlledColor,
    lighting_color_controller_color
)
NIFLY_RS_ANIMATION_FIELD(
    BSLightingShaderPropertyFloatController,
    uint32_t,
    typeOfControlledVariable,
    lighting_float_controller_variable
)
NIFLY_RS_ANIMATION_FIELD(
    BSLightingShaderPropertyUShortController,
    uint32_t,
    typeOfControlledVariable,
    lighting_ushort_controller_variable
)
NIFLY_RS_ANIMATION_FIELD(
    BSEffectShaderPropertyColorController,
    uint32_t,
    typeOfControlledColor,
    effect_color_controller_color
)
NIFLY_RS_ANIMATION_FIELD(
    BSEffectShaderPropertyFloatController,
    uint32_t,
    typeOfControlledVariable,
    effect_float_controller_variable
)
NIFLY_RS_ANIMATION_FIELD(
    NiGeomMorpherController,
    uint16_t,
    morpherFlags,
    geom_morpher_flags
)
NIFLY_RS_ANIMATION_FIELD(
    NiGeomMorpherController,
    bool,
    alwaysUpdate,
    geom_morpher_always_update
)
NIFLY_RS_ANIMATION_FIELD(NiBoneLODController, uint32_t, lod, bone_lod_lod)
NIFLY_RS_ANIMATION_FIELD(
    bhkBlendController,
    uint32_t,
    keys,
    blend_controller_keys
)

inline uint32_t
bone_lod_controller_count(const nifly::NiBoneLODController& obj) {
    return obj.numLODs;
}

inline rust::Slice<const uint8_t>
float_extra_data_controller_name(const nifly::NiFloatExtraDataController& obj) {
    auto& name = obj.extraData.get();
    return {reinterpret_cast<const uint8_t*>(name.data()), name.size()};
}

inline void set_float_extra_data_controller_name(
    nifly::NiFloatExtraDataController& obj,
    rust::Slice<const uint8_t> name
) {
    obj.extraData.get().assign(
        reinterpret_cast<const char*>(name.data()),
        name.size()
    );
}

inline rust::Vec<uint32_t>
geom_morpher_unknown_ints(const nifly::NiGeomMorpherController& obj) {
    rust::Vec<uint32_t> out;
    for (uint32_t v : unconst(obj).unknownInts)
        out.push_back(v);
    return out;
}

inline void set_geom_morpher_unknown_ints(
    nifly::NiGeomMorpherController& obj,
    rust::Slice<const uint32_t> ints
) {
    obj.unknownInts.clear();
    for (uint32_t v : ints)
        obj.unknownInts.push_back(v);
}

// The interpolators of the NiGeomMorpherController at `owner`, from the list
// its file version saves: weighted from 20.1.0.3 on, plain refs before (with
// the weights kept in memory, or 0). Empty if the block isn't one.
inline rust::Vec<MorphWeightFfi>
morph_weights(const nifly::NifFile& file, uint32_t owner) {
    using namespace nifly;
    rust::Vec<MorphWeightFfi> out;
    auto& hdr = file.GetHeader();
    auto c = hdr.GetBlock<NiGeomMorpherController>(owner);
    if (!c)
        return out;
    if (hdr.GetVersion().File() >= NiFileVersion::V20_1_0_3) {
        for (auto& w : c->interpWeights)
            out.push_back({w.interpRef.index, w.weight});
        return out;
    }
    for (uint32_t i = 0; i < c->interpolatorRefs.GetSize(); i++) {
        float weight =
            i < c->interpWeights.size() ? c->interpWeights[i].weight : 0.0f;
        out.push_back({c->interpolatorRefs.GetBlockRef(i), weight});
    }
    return out;
}

// Replaces both interpolator lists of the NiGeomMorpherController at `owner`.
// False, changing nothing, unless each interpolator is empty or an
// NiInterpolator.
inline bool set_morph_weights(
    nifly::NifFile& file,
    uint32_t owner,
    rust::Slice<const MorphWeightFfi> weights
) {
    using namespace nifly;
    auto& hdr = file.GetHeader();
    auto c = hdr.GetBlock<NiGeomMorpherController>(owner);
    if (!c)
        return false;
    for (auto& w : weights)
        if (w.interpolator != NIF_NPOS
            && !hdr.GetBlock<NiInterpolator>(w.interpolator))
            return false;
    c->interpolatorRefs.Clear();
    c->interpWeights.clear();
    for (auto& w : weights) {
        c->interpolatorRefs.AddBlockRef(w.interpolator);
        MorphWeight m;
        m.interpRef.index = w.interpolator;
        m.weight = w.weight;
        c->interpWeights.push_back(m);
    }
    return true;
}

inline rust::Vec<NodeGroupFfi>
bone_lod_groups(const nifly::NiBoneLODController& obj) {
    rust::Vec<NodeGroupFfi> out;
    for (auto& group : unconst(obj).boneArrays) {
        NodeGroupFfi g;
        for (uint32_t i = 0; i < group.GetSize(); i++)
            g.nodes.push_back(group.GetBlockRef(i));
        out.push_back(std::move(g));
    }
    return out;
}

// Replaces the node groups of the NiBoneLODController at `owner` and sets its
// LOD count to their number. False, changing nothing, unless each node is an
// NiNode.
inline bool set_bone_lod_groups(
    nifly::NifFile& file,
    uint32_t owner,
    rust::Slice<const NodeGroupFfi> groups
) {
    using namespace nifly;
    auto& hdr = file.GetHeader();
    auto c = hdr.GetBlock<NiBoneLODController>(owner);
    if (!c)
        return false;
    for (auto& g : groups)
        for (uint32_t id : g.nodes)
            if (!hdr.GetBlock<NiNode>(id))
                return false;
    c->boneArrays.clear();
    for (auto& g : groups) {
        NiBlockPtrArray<NiNode> nodes;
        for (uint32_t id : g.nodes)
            nodes.AddBlockRef(id);
        c->boneArrays.push_back(std::move(nodes));
    }
    c->numLODs = static_cast<uint32_t>(groups.size());
    return true;
}

// NiFloatExtraDataController reads and writes the extra data name in every
// version, but files store it only from 10.2.0.0 on: older files are misread
// from there. FloatExtraDataController syncs it by version;
// FloatExtraDataControllerFactory loads every NiFloatExtraDataController as
// one, and add_ni_float_extra_data_controller creates them.
class FloatExtraDataController final: public nifly::NiFloatExtraDataController {
  public:
    void Get(nifly::NiIStream& stream) override {
        nifly::NiExtraDataController::Get(stream);
        if (stream.GetVersion().File() >= nifly::NiFileVersion::V10_2_0_0)
            extraData.Read(stream);
    }

    void Put(nifly::NiOStream& stream) override {
        nifly::NiExtraDataController::Put(stream);
        if (stream.GetVersion().File() >= nifly::NiFileVersion::V10_2_0_0)
            extraData.Write(stream);
    }

  private:
    FloatExtraDataController* Clone_impl() const override {
        return new FloatExtraDataController(*this);
    }
};

class FloatExtraDataControllerFactory final: public nifly::NiFactory {
  public:
    std::unique_ptr<nifly::NiObject> Create() override {
        return std::make_unique<FloatExtraDataController>();
    }

    std::unique_ptr<nifly::NiObject> Load(nifly::NiIStream& stream) override {
        auto c = std::make_unique<FloatExtraDataController>();
        c->Get(stream);
        return c;
    }
};

// Controllers.

NIFLY_RS_ANIMATION_FIELD(NiTimeController, uint16_t, flags, controller_flags)
NIFLY_RS_ANIMATION_FIELD(
    NiTimeController,
    float,
    frequency,
    controller_frequency
)
NIFLY_RS_ANIMATION_FIELD(NiTimeController, float, phase, controller_phase)
NIFLY_RS_ANIMATION_FIELD(
    NiTimeController,
    float,
    startTime,
    controller_start_time
)
NIFLY_RS_ANIMATION_FIELD(
    NiTimeController,
    float,
    stopTime,
    controller_stop_time
)
NIFLY_RS_ANIMATION_FIELD(
    NiInterpController,
    bool,
    managerControlled,
    interp_controller_manager_controlled
)

// Appends `controller` to the controller chain of the NiObjectNET at `owner`
// and points its target at the owner. A ref that isn't a controller, or that
// repeats one already passed, ends the chain and is replaced.
inline void append_controller(
    nifly::NiHeader& hdr,
    nifly::NiObjectNET& owner,
    uint32_t owner_index,
    nifly::NiTimeController& controller,
    uint32_t index
) {
    using namespace nifly;
    controller.targetRef.index = owner_index;
    NiRef* link = &owner.controllerRef;
    std::set<uint32_t> seen;
    while (auto c = hdr.GetBlock<NiTimeController>(link->index)) {
        if (!seen.insert(link->index).second)
            break;
        link = &c->nextControllerRef;
    }
    link->index = index;
}

// Adds a new T to the end of the controller chain of the NiObjectNET at
// `owner`, targeting it. NIF_NPOS, adding nothing, if `owner` isn't one.
#define NIFLY_RS_ADD_CONTROLLER_AS(T, Make, snake) \
    inline uint32_t add_##snake(nifly::NifFile& file, uint32_t owner) { \
        auto& hdr = file.GetHeader(); \
        auto net = hdr.GetBlock<nifly::NiObjectNET>(owner); \
        if (!net) \
            return nifly::NIF_NPOS; \
        auto block = std::make_unique<Make>(); \
        auto& controller = *block; \
        uint32_t index = hdr.AddBlock(std::move(block)); \
        append_controller(hdr, *net, owner, controller, index); \
        return index; \
    }
#define NIFLY_RS_ADD_CONTROLLER(T, snake) \
    NIFLY_RS_ADD_CONTROLLER_AS(T, nifly::T, snake)

NIFLY_RS_ADD_CONTROLLER(NiLookAtController, ni_look_at_controller)
NIFLY_RS_ADD_CONTROLLER(NiPathController, ni_path_controller)
NIFLY_RS_ADD_CONTROLLER(NiUVController, ni_uv_controller)
NIFLY_RS_ADD_CONTROLLER(BSFrustumFOVController, bs_frustum_fov_controller)
NIFLY_RS_ADD_CONTROLLER(BSLagBoneController, bs_lag_bone_controller)
NIFLY_RS_ADD_CONTROLLER(
    BSProceduralLightningController,
    bs_procedural_lightning_controller
)
NIFLY_RS_ADD_CONTROLLER(NiBoneLODController, ni_bone_lod_controller)
NIFLY_RS_ADD_CONTROLLER(NiBSBoneLODController, ni_bs_bone_lod_controller)
NIFLY_RS_ADD_CONTROLLER(NiGeomMorpherController, ni_geom_morpher_controller)
NIFLY_RS_ADD_CONTROLLER(NiRollController, ni_roll_controller)
NIFLY_RS_ADD_CONTROLLER(NiMaterialColorController, ni_material_color_controller)
NIFLY_RS_ADD_CONTROLLER(NiLightColorController, ni_light_color_controller)
NIFLY_RS_ADD_CONTROLLER_AS(
    NiFloatExtraDataController,
    FloatExtraDataController,
    ni_float_extra_data_controller
)
NIFLY_RS_ADD_CONTROLLER(bhkBlendController, bhk_blend_controller)
NIFLY_RS_ADD_CONTROLLER(NiVisController, ni_vis_controller)
NIFLY_RS_ADD_CONTROLLER(NiFlipController, ni_flip_controller)
NIFLY_RS_ADD_CONTROLLER(
    NiTextureTransformController,
    ni_texture_transform_controller
)
NIFLY_RS_ADD_CONTROLLER(NiLightDimmerController, ni_light_dimmer_controller)
NIFLY_RS_ADD_CONTROLLER(NiLightRadiusController, ni_light_radius_controller)
NIFLY_RS_ADD_CONTROLLER(NiAlphaController, ni_alpha_controller)
NIFLY_RS_ADD_CONTROLLER(
    BSNiAlphaPropertyTestRefController,
    bs_ni_alpha_property_test_ref_controller
)
NIFLY_RS_ADD_CONTROLLER(NiKeyframeController, ni_keyframe_controller)
NIFLY_RS_ADD_CONTROLLER(NiTransformController, ni_transform_controller)
NIFLY_RS_ADD_CONTROLLER(
    BSMaterialEmittanceMultController,
    bs_material_emittance_mult_controller
)
NIFLY_RS_ADD_CONTROLLER(
    BSRefractionStrengthController,
    bs_refraction_strength_controller
)
NIFLY_RS_ADD_CONTROLLER(
    BSRefractionFirePeriodController,
    bs_refraction_fire_period_controller
)
NIFLY_RS_ADD_CONTROLLER(
    BSLightingShaderPropertyColorController,
    bs_lighting_shader_property_color_controller
)
NIFLY_RS_ADD_CONTROLLER(
    BSLightingShaderPropertyFloatController,
    bs_lighting_shader_property_float_controller
)
NIFLY_RS_ADD_CONTROLLER(
    BSLightingShaderPropertyUShortController,
    bs_lighting_shader_property_u_short_controller
)
NIFLY_RS_ADD_CONTROLLER(
    BSEffectShaderPropertyColorController,
    bs_effect_shader_property_color_controller
)
NIFLY_RS_ADD_CONTROLLER(
    BSEffectShaderPropertyFloatController,
    bs_effect_shader_property_float_controller
)
NIFLY_RS_ADD_CONTROLLER(
    NiMultiTargetTransformController,
    ni_multi_target_transform_controller
)
} // namespace nifly_rs
