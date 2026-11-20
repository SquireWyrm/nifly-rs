// Part of animation_shim.h: interpolator pose values and blend, path, look-at, tread and B-spline interpolators.
#pragma once

#include "animation_shim.h"

namespace nifly_rs {
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
} // namespace nifly_rs
