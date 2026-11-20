// Part of animation_shim.h: keyframe, visibility, morph and B-spline data, string palettes, and adding data blocks.
#pragma once

#include "animation_shim.h"

namespace nifly_rs {
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
} // namespace nifly_rs
