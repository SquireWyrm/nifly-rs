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
NIFLY_RS_ANIMATION(
    NiSequenceStreamHelper,
    NiObjectNET,
    ni_sequence_stream_helper
)

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
} // namespace nifly_rs

// The rest of the shims, by topic.
// In this order: later parts use earlier ones.
// clang-format off
#include "animation_shim_data.h"
#include "animation_shim_interpolators.h"
#include "animation_shim_controllers.h"
#include "animation_shim_sequences.h"
// clang-format on
