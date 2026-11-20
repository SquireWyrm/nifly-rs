#pragma once
#include <algorithm>
#include <cstdint>
#include <cstring>
#include <stdexcept>
#include <string>
#include <vector>

#include "ExtraData.hpp"
#include "Factory.hpp"
#include "nifly-sys/src/extra_data_types.rs.h"
#include "nifly-sys/src/object_3d.rs.h"
#include "rust/cxx.h"

// Same upcast/downcast convention as nodes_shim.h.
#define NIFLY_RS_EXTRA_DATA(T, Parent, snake) \
    inline const nifly::Parent& upcast_##snake(const nifly::T& obj) { \
        return obj; \
    } \
    inline const nifly::T* downcast_##snake(const nifly::NiObject& obj) { \
        return dynamic_cast<const nifly::T*>(&obj); \
    }

namespace nifly_rs {
NIFLY_RS_EXTRA_DATA(NiExtraData, NiObject, ni_extra_data)
NIFLY_RS_EXTRA_DATA(NiStringExtraData, NiExtraData, ni_string_extra_data)
NIFLY_RS_EXTRA_DATA(NiIntegerExtraData, NiExtraData, ni_integer_extra_data)
NIFLY_RS_EXTRA_DATA(BSXFlags, NiIntegerExtraData, bsx_flags)
NIFLY_RS_EXTRA_DATA(NiFloatExtraData, NiExtraData, ni_float_extra_data)
NIFLY_RS_EXTRA_DATA(NiBooleanExtraData, NiExtraData, ni_boolean_extra_data)
NIFLY_RS_EXTRA_DATA(NiIntegersExtraData, NiExtraData, ni_integers_extra_data)

// BSExtraData derives from NiObject, not NiExtraData, and has no name.
NIFLY_RS_EXTRA_DATA(BSExtraData, NiObject, bs_extra_data)
NIFLY_RS_EXTRA_DATA(BSClothExtraData, BSExtraData, bs_cloth_extra_data)
NIFLY_RS_EXTRA_DATA(
    BSCollisionQueryProxyExtraData,
    BSExtraData,
    bs_collision_query_proxy_extra_data
)

inline rust::Slice<const uint8_t>
cloth_extra_data_bytes(const nifly::BSClothExtraData& obj) {
    return {reinterpret_cast<const uint8_t*>(obj.data.data()), obj.data.size()};
}

inline rust::Slice<const uint32_t>
integers_extra_data_values(const nifly::NiIntegersExtraData& obj) {
    return {obj.integersData.data(), obj.integersData.size()};
}

inline rust::Slice<const uint8_t> collision_query_proxy_extra_data_bytes(
    const nifly::BSCollisionQueryProxyExtraData& obj
) {
    return {reinterpret_cast<const uint8_t*>(obj.data.data()), obj.data.size()};
}

inline rust::Slice<const uint8_t>
extra_data_name(const nifly::NiExtraData& obj) {
    const std::string& s = obj.name.get();
    return {reinterpret_cast<const uint8_t*>(s.data()), s.size()};
}

inline rust::Slice<const uint8_t>
string_extra_data_value(const nifly::NiStringExtraData& obj) {
    const std::string& s = obj.stringData.get();
    return {reinterpret_cast<const uint8_t*>(s.data()), s.size()};
}

inline uint32_t integer_extra_data_value(const nifly::NiIntegerExtraData& obj) {
    return obj.integerData;
}

inline float float_extra_data_value(const nifly::NiFloatExtraData& obj) {
    return obj.floatData;
}

inline bool boolean_extra_data_value(const nifly::NiBooleanExtraData& obj) {
    return obj.booleanData;
}

// More NiExtraData subclasses.
NIFLY_RS_EXTRA_DATA(NiBinaryExtraData, NiExtraData, ni_binary_extra_data)
NIFLY_RS_EXTRA_DATA(NiFloatsExtraData, NiExtraData, ni_floats_extra_data)
NIFLY_RS_EXTRA_DATA(NiStringsExtraData, NiExtraData, ni_strings_extra_data)
NIFLY_RS_EXTRA_DATA(SkinAttach, NiExtraData, skin_attach)
NIFLY_RS_EXTRA_DATA(
    BSPackedCombinedSharedGeomDataExtra,
    NiExtraData,
    bs_packed_combined_shared_geom_data_extra
)

// VertexDesc keeps its 64 bits private.
static_assert(sizeof(nifly::VertexDesc) == sizeof(uint64_t));

inline uint64_t vertex_desc_bits(const nifly::VertexDesc& d) {
    uint64_t bits;
    std::memcpy(&bits, &d, sizeof bits);
    return bits;
}

inline void set_vertex_desc_bits(nifly::VertexDesc& d, uint64_t bits) {
    std::memcpy(&d, &bits, sizeof bits);
}

NIFLY_RS_EXTRA_DATA(BSFurnitureMarker, NiExtraData, bs_furniture_marker)
NIFLY_RS_EXTRA_DATA(
    BSFurnitureMarkerNode,
    BSFurnitureMarker,
    bs_furniture_marker_node
)
NIFLY_RS_EXTRA_DATA(
    BSConnectPointParents,
    NiExtraData,
    bs_connect_point_parents
)
NIFLY_RS_EXTRA_DATA(
    BSConnectPointChildren,
    NiExtraData,
    bs_connect_point_children
)
NIFLY_RS_EXTRA_DATA(BSInvMarker, NiExtraData, bs_inv_marker)
NIFLY_RS_EXTRA_DATA(BSBound, NiExtraData, bs_bound)
NIFLY_RS_EXTRA_DATA(
    BSBehaviorGraphExtraData,
    NiExtraData,
    bs_behavior_graph_extra_data
)

inline rust::Slice<const uint8_t> extra_bytes(const std::string& s) {
    return {reinterpret_cast<const uint8_t*>(s.data()), s.size()};
}

inline rust::Slice<const uint8_t>
binary_extra_data_bytes(const nifly::NiBinaryExtraData& obj) {
    return {obj.data.data(), obj.data.size()};
}

inline rust::Slice<const float>
floats_extra_data_values(const nifly::NiFloatsExtraData& obj) {
    return {obj.floatsData.data(), obj.floatsData.size()};
}

inline size_t strings_extra_data_len(const nifly::NiStringsExtraData& obj) {
    return obj.stringsData.size();
}

inline rust::Slice<const uint8_t>
strings_extra_data_value(const nifly::NiStringsExtraData& obj, size_t index) {
    if (index >= obj.stringsData.size())
        return {};
    return extra_bytes(obj.stringsData.data()[index].get());
}

inline size_t skin_attach_bone_count(const nifly::SkinAttach& obj) {
    return obj.bones.size();
}

inline rust::Slice<const uint8_t>
skin_attach_bone(const nifly::SkinAttach& obj, size_t index) {
    if (index >= obj.bones.size())
        return {};
    return extra_bytes(obj.bones.data()[index].get());
}

inline rust::Vec<FurniturePositionFfi>
furniture_positions(const nifly::BSFurnitureMarker& obj) {
    rust::Vec<FurniturePositionFfi> out;
    for (uint32_t i = 0; i < obj.positions.size(); i++) {
        const auto& p = obj.positions.data()[i];
        out.push_back(
            FurniturePositionFfi {
                Vec3 {p.offset.x, p.offset.y, p.offset.z},
                p.orientation,
                p.posRef1,
                p.posRef2,
                p.heading,
                p.animationType,
                p.entryPoints,
            }
        );
    }
    return out;
}

inline size_t connect_point_count(const nifly::BSConnectPointParents& obj) {
    return obj.connectPoints.size();
}

inline rust::Slice<const uint8_t>
connect_point_parent(const nifly::BSConnectPointParents& obj, size_t index) {
    if (index >= obj.connectPoints.size())
        return {};
    return extra_bytes(obj.connectPoints.data()[index].root.get());
}

inline rust::Slice<const uint8_t>
connect_point_name(const nifly::BSConnectPointParents& obj, size_t index) {
    if (index >= obj.connectPoints.size())
        return {};
    return extra_bytes(obj.connectPoints.data()[index].variableName.get());
}

inline ConnectPointTransform
connect_point_transform(const nifly::BSConnectPointParents& obj, size_t index) {
    if (index >= obj.connectPoints.size())
        return ConnectPointTransform {
            1.0f,
            0.0f,
            0.0f,
            0.0f,
            Vec3 {0.0f, 0.0f, 0.0f},
            1.0f
        };
    const auto& c = obj.connectPoints.data()[index];
    return ConnectPointTransform {
        c.rotation.w,
        c.rotation.x,
        c.rotation.y,
        c.rotation.z,
        Vec3 {c.translation.x, c.translation.y, c.translation.z},
        c.scale,
    };
}

inline bool
connect_point_children_skinned(const nifly::BSConnectPointChildren& obj) {
    return obj.skinned;
}

inline size_t
connect_point_children_len(const nifly::BSConnectPointChildren& obj) {
    return obj.targets.size();
}

inline rust::Slice<const uint8_t>
connect_point_child(const nifly::BSConnectPointChildren& obj, size_t index) {
    if (index >= obj.targets.size())
        return {};
    return extra_bytes(obj.targets.data()[index].get());
}

inline InvMarkerFfi inv_marker(const nifly::BSInvMarker& obj) {
    return InvMarkerFfi {obj.rotationX, obj.rotationY, obj.rotationZ, obj.zoom};
}

inline Vec3 bound_center(const nifly::BSBound& obj) {
    return {obj.center.x, obj.center.y, obj.center.z};
}

inline Vec3 bound_half_extents(const nifly::BSBound& obj) {
    return {obj.halfExtents.x, obj.halfExtents.y, obj.halfExtents.z};
}

inline rust::Slice<const uint8_t>
behavior_graph_file(const nifly::BSBehaviorGraphExtraData& obj) {
    return extra_bytes(obj.behaviorGraphFile.get());
}

inline bool behavior_graph_controls_base_skeleton(
    const nifly::BSBehaviorGraphExtraData& obj
) {
    return obj.controlsBaseSkel;
}
NIFLY_RS_EXTRA_DATA(NiColorExtraData, NiExtraData, ni_color_extra_data)
NIFLY_RS_EXTRA_DATA(NiVectorExtraData, NiExtraData, ni_vector_extra_data)
NIFLY_RS_EXTRA_DATA(
    NiVertWeightsExtraData,
    NiExtraData,
    ni_vert_weights_extra_data
)
NIFLY_RS_EXTRA_DATA(BSEyeCenterExtraData, NiExtraData, bs_eye_center_extra_data)
NIFLY_RS_EXTRA_DATA(BSPositionData, NiExtraData, bs_position_data)
NIFLY_RS_EXTRA_DATA(BSWArray, NiExtraData, bs_w_array)
NIFLY_RS_EXTRA_DATA(
    BSDecalPlacementVectorExtraData,
    NiFloatExtraData,
    bs_decal_placement_vector_extra_data
)
NIFLY_RS_EXTRA_DATA(BSBoneLODExtraData, NiExtraData, bs_bone_lod_extra_data)
NIFLY_RS_EXTRA_DATA(
    BSDistantObjectExtraData,
    NiExtraData,
    bs_distant_object_extra_data
)
NIFLY_RS_EXTRA_DATA(
    BSDistantObjectLargeRefExtraData,
    NiExtraData,
    bs_distant_object_large_ref_extra_data
)
NIFLY_RS_EXTRA_DATA(BoneTranslations, NiExtraData, bone_translations)
NIFLY_RS_EXTRA_DATA(NiTextKeyExtraData, NiExtraData, ni_text_key_extra_data)

inline Float4 color_extra_data_value(const nifly::NiColorExtraData& obj) {
    const auto& c = obj.colorData;
    return {c.r, c.g, c.b, c.a};
}

inline Float4 vector_extra_data_value(const nifly::NiVectorExtraData& obj) {
    const auto& v = obj.vectorData;
    return {v.x, v.y, v.z, v.w};
}

inline rust::Slice<const float>
vert_weights_extra_data_values(const nifly::NiVertWeightsExtraData& obj) {
    return {obj.weights.data(), obj.weights.size()};
}

inline rust::Slice<const float>
eye_center_extra_data_values(const nifly::BSEyeCenterExtraData& obj) {
    return {obj.data.data(), obj.data.size()};
}

inline rust::Vec<float> position_data_values(const nifly::BSPositionData& obj) {
    rust::Vec<float> out;
    for (uint32_t i = 0; i < obj.data.size(); i++)
        out.push_back(static_cast<float>(obj.data.data()[i]));
    return out;
}

inline rust::Slice<const uint32_t> w_array_values(const nifly::BSWArray& obj) {
    return {obj.data.data(), obj.data.size()};
}

inline size_t
decal_block_count(const nifly::BSDecalPlacementVectorExtraData& obj) {
    return obj.decalVectorBlocks.size();
}

template<class List>
rust::Vec<Vec3> vec3s_to_ffi(const List& list) {
    rust::Vec<Vec3> out;
    for (uint32_t i = 0; i < list.size(); i++) {
        const auto& v = list.data()[i];
        out.push_back(Vec3 {v.x, v.y, v.z});
    }
    return out;
}

inline rust::Vec<Vec3> decal_block_points(
    const nifly::BSDecalPlacementVectorExtraData& obj,
    size_t block
) {
    if (block >= obj.decalVectorBlocks.size())
        return {};
    return vec3s_to_ffi(obj.decalVectorBlocks.data()[block].points);
}

inline rust::Vec<Vec3> decal_block_normals(
    const nifly::BSDecalPlacementVectorExtraData& obj,
    size_t block
) {
    if (block >= obj.decalVectorBlocks.size())
        return {};
    return vec3s_to_ffi(obj.decalVectorBlocks.data()[block].normals);
}

inline size_t bone_lod_count(const nifly::BSBoneLODExtraData& obj) {
    return obj.boneLODs.size();
}

inline uint32_t
bone_lod_distance(const nifly::BSBoneLODExtraData& obj, size_t index) {
    return index < obj.boneLODs.size() ? obj.boneLODs.data()[index].distance
                                       : 0;
}

inline rust::Slice<const uint8_t>
bone_lod_bone(const nifly::BSBoneLODExtraData& obj, size_t index) {
    if (index >= obj.boneLODs.size())
        return {};
    return extra_bytes(obj.boneLODs.data()[index].boneName.get());
}

inline uint32_t
distant_object_flags(const nifly::BSDistantObjectExtraData& obj) {
    return obj.distantObjectFlags;
}

inline bool
distant_object_large_ref(const nifly::BSDistantObjectLargeRefExtraData& obj) {
    return obj.largeRef;
}

inline size_t bone_translation_count(const nifly::BoneTranslations& obj) {
    return obj.translations.size();
}

inline rust::Slice<const uint8_t>
bone_translation_bone(const nifly::BoneTranslations& obj, size_t index) {
    if (index >= obj.translations.size())
        return {};
    return extra_bytes(obj.translations[index].bone.get());
}

inline Vec3 bone_translation(const nifly::BoneTranslations& obj, size_t index) {
    if (index >= obj.translations.size())
        return {0.0f, 0.0f, 0.0f};
    const auto& t = obj.translations[index].trans;
    return {t.x, t.y, t.z};
}

// Setters for existing extra data. The assign_* shims in nif_file_shim.h fill
// new blocks through these too. Lists are replaced; checks that can fail run
// first, so a false return changes nothing.

inline void
set_extra_data_name(nifly::NiExtraData& obj, rust::Slice<const uint8_t> name) {
    obj.name.get().assign(
        reinterpret_cast<const char*>(name.data()),
        name.size()
    );
}

inline uint32_t extra_data_next(const nifly::NiExtraData& obj) {
    return obj.nextExtraDataRef.index;
}

inline uint32_t extra_data_num_bytes(const nifly::NiExtraData& obj) {
    return obj.numBytes;
}

inline void set_extra_data_num_bytes(nifly::NiExtraData& obj, uint32_t v) {
    obj.numBytes = v;
}

// Splits `bytes` into strings of `lengths` bytes each, in order. The safe crate
// builds both from the same strings, so they always add up.
inline std::vector<std::string> split_strings(
    rust::Slice<const uint8_t> bytes,
    rust::Slice<const size_t> lengths
) {
    std::vector<std::string> out;
    out.reserve(lengths.size());
    size_t at = 0;
    for (size_t len : lengths) {
        len = std::min(len, bytes.size() - at);
        out.emplace_back(reinterpret_cast<const char*>(bytes.data()) + at, len);
        at += len;
    }
    return out;
}

inline nifly::NiString to_ni_string(const std::string& s) {
    nifly::NiString out;
    out.get() = s;
    return out;
}

inline void set_string_extra_data(
    nifly::NiStringExtraData& d,
    rust::Slice<const uint8_t> value
) {
    d.stringData.get().assign(
        reinterpret_cast<const char*>(value.data()),
        value.size()
    );
}

inline void set_integer_extra_data(nifly::NiIntegerExtraData& d, uint32_t v) {
    d.integerData = v;
}

inline void set_float_extra_data(nifly::NiFloatExtraData& d, float v) {
    d.floatData = v;
}

inline void set_boolean_extra_data(nifly::NiBooleanExtraData& d, bool v) {
    d.booleanData = v;
}

inline void set_integers_extra_data(
    nifly::NiIntegersExtraData& d,
    rust::Slice<const uint32_t> values
) {
    d.integersData.clear();
    for (uint32_t v : values)
        d.integersData.push_back(v);
}

inline void set_binary_extra_data(
    nifly::NiBinaryExtraData& d,
    rust::Slice<const uint8_t> bytes
) {
    d.data.resize(static_cast<uint32_t>(bytes.size()));
    std::copy(bytes.begin(), bytes.end(), d.data.data());
}

inline void set_floats_extra_data(
    nifly::NiFloatsExtraData& d,
    rust::Slice<const float> values
) {
    d.floatsData.resize(static_cast<uint32_t>(values.size()));
    std::copy(values.begin(), values.end(), d.floatsData.data());
}

inline void set_strings_extra_data(
    nifly::NiStringsExtraData& d,
    rust::Slice<const uint8_t> bytes,
    rust::Slice<const size_t> lengths
) {
    d.stringsData.clear();
    for (const auto& s : split_strings(bytes, lengths)) {
        auto str = to_ni_string(s);
        d.stringsData.push_back(str);
    }
}

inline void set_skin_attach_bones(
    nifly::SkinAttach& d,
    rust::Slice<const uint8_t> bytes,
    rust::Slice<const size_t> lengths
) {
    d.bones.clear();
    for (const auto& s : split_strings(bytes, lengths)) {
        auto str = to_ni_string(s);
        d.bones.push_back(str);
    }
}

inline void set_cloth_extra_data(
    nifly::BSClothExtraData& d,
    rust::Slice<const uint8_t> bytes
) {
    d.data.resize(static_cast<uint32_t>(bytes.size()));
    std::copy(
        bytes.begin(),
        bytes.end(),
        reinterpret_cast<uint8_t*>(d.data.data())
    );
}

inline void set_collision_query_proxy_extra_data(
    nifly::BSCollisionQueryProxyExtraData& d,
    rust::Slice<const uint8_t> bytes
) {
    d.data.resize(static_cast<uint32_t>(bytes.size()));
    std::copy(
        bytes.begin(),
        bytes.end(),
        reinterpret_cast<uint8_t*>(d.data.data())
    );
}

inline void set_color_extra_data(nifly::NiColorExtraData& d, Float4 c) {
    d.colorData = nifly::Color4(c.x, c.y, c.z, c.w);
}

inline void set_vector_extra_data(nifly::NiVectorExtraData& d, Float4 v) {
    d.vectorData = nifly::Vector4(v.x, v.y, v.z, v.w);
}

// The weights' count is stored in 16 bits, so more than 65535 are rejected.
inline bool vert_weights_fit(rust::Slice<const float> weights) {
    return weights.size() <= 0xFFFF;
}

inline bool set_vert_weights_extra_data(
    nifly::NiVertWeightsExtraData& d,
    rust::Slice<const float> weights
) {
    if (!vert_weights_fit(weights))
        return false;
    d.weights.resize(static_cast<uint16_t>(weights.size()));
    std::copy(weights.begin(), weights.end(), d.weights.data());
    return true;
}

inline void set_eye_center_extra_data(
    nifly::BSEyeCenterExtraData& d,
    rust::Slice<const float> values
) {
    d.data.resize(static_cast<uint32_t>(values.size()));
    std::copy(values.begin(), values.end(), d.data.data());
}

// Values are narrowed to half floats.
inline void
set_position_data(nifly::BSPositionData& d, rust::Slice<const float> values) {
    d.data.resize(static_cast<uint32_t>(values.size()));
    for (size_t i = 0; i < values.size(); i++)
        d.data.data()[i] = half_float::half(values[i]);
}

inline void
set_w_array(nifly::BSWArray& d, rust::Slice<const uint32_t> values) {
    d.data.resize(static_cast<uint32_t>(values.size()));
    std::copy(values.begin(), values.end(), d.data.data());
}

// Block i holds points[j] and normals[j] for the next block_lengths[i] j.
// Writing stores one normal per point, so each point comes with its normal.
// Block and point counts are stored in 16 bits, so more than 65535 are
// rejected.
inline bool decal_blocks_fit(
    rust::Slice<const size_t> block_lengths,
    rust::Slice<const Vec3> points,
    rust::Slice<const Vec3> normals
) {
    if (block_lengths.size() > 0xFFFF || points.size() != normals.size())
        return false;
    size_t total = 0;
    for (size_t len : block_lengths) {
        if (len > 0xFFFF)
            return false;
        total += len;
    }
    return total == points.size();
}

inline bool set_decal_blocks(
    nifly::BSDecalPlacementVectorExtraData& d,
    rust::Slice<const size_t> block_lengths,
    rust::Slice<const Vec3> points,
    rust::Slice<const Vec3> normals
) {
    if (!decal_blocks_fit(block_lengths, points, normals))
        return false;
    d.decalVectorBlocks.clear();
    size_t at = 0;
    for (size_t len : block_lengths) {
        nifly::DecalVectorBlock block;
        block.points.resize(static_cast<uint16_t>(len));
        block.normals.resize(static_cast<uint16_t>(len));
        for (size_t j = 0; j < len; j++, at++) {
            block.points.data()[j] =
                nifly::Vector3(points[at].x, points[at].y, points[at].z);
            block.normals.data()[j] =
                nifly::Vector3(normals[at].x, normals[at].y, normals[at].z);
        }
        d.decalVectorBlocks.push_back(block);
    }
    return true;
}

inline void
set_distant_object_flags(nifly::BSDistantObjectExtraData& d, uint32_t flags) {
    d.distantObjectFlags = flags;
}

inline void set_distant_object_large_ref(
    nifly::BSDistantObjectLargeRefExtraData& d,
    bool large_ref
) {
    d.largeRef = large_ref;
}

inline void set_furniture_positions(
    nifly::BSFurnitureMarker& d,
    rust::Slice<const FurniturePositionFfi> positions
) {
    d.positions.clear();
    for (const auto& p : positions) {
        nifly::FurniturePosition pos;
        pos.offset = nifly::Vector3(p.offset.x, p.offset.y, p.offset.z);
        pos.orientation = p.orientation;
        pos.posRef1 = p.position_ref_1;
        pos.posRef2 = p.position_ref_2;
        pos.heading = p.heading;
        pos.animationType = p.animation_type;
        pos.entryPoints = p.entry_points;
        d.positions.push_back(pos);
    }
}

// Connect point i has parent strings[2i], name strings[2i + 1] and transform
// transforms[i]; the safe crate builds both lists from the same points.
inline void set_connect_points(
    nifly::BSConnectPointParents& d,
    rust::Slice<const uint8_t> bytes,
    rust::Slice<const size_t> lengths,
    rust::Slice<const ConnectPointTransform> transforms
) {
    d.connectPoints.clear();
    auto strings = split_strings(bytes, lengths);
    size_t n = std::min(strings.size() / 2, transforms.size());
    for (size_t i = 0; i < n; i++) {
        const auto& t = transforms[i];
        nifly::BSConnectPoint c;
        c.root.get() = strings[2 * i];
        c.variableName.get() = strings[2 * i + 1];
        c.rotation = nifly::Quaternion(
            t.rotation_w,
            t.rotation_x,
            t.rotation_y,
            t.rotation_z
        );
        c.translation =
            nifly::Vector3(t.translation.x, t.translation.y, t.translation.z);
        c.scale = t.scale;
        d.connectPoints.push_back(c);
    }
}

inline void
set_connect_point_skinned(nifly::BSConnectPointChildren& d, bool skinned) {
    d.skinned = skinned;
}

inline void set_connect_point_targets(
    nifly::BSConnectPointChildren& d,
    rust::Slice<const uint8_t> bytes,
    rust::Slice<const size_t> lengths
) {
    d.targets.clear();
    for (const auto& s : split_strings(bytes, lengths)) {
        auto str = to_ni_string(s);
        d.targets.push_back(str);
    }
}

inline void set_inv_marker(nifly::BSInvMarker& d, InvMarkerFfi marker) {
    d.rotationX = marker.rotation_x;
    d.rotationY = marker.rotation_y;
    d.rotationZ = marker.rotation_z;
    d.zoom = marker.zoom;
}

inline void set_bound_center(nifly::BSBound& d, Vec3 v) {
    d.center = nifly::Vector3(v.x, v.y, v.z);
}

inline void set_bound_half_extents(nifly::BSBound& d, Vec3 v) {
    d.halfExtents = nifly::Vector3(v.x, v.y, v.z);
}

inline void set_behavior_graph_file(
    nifly::BSBehaviorGraphExtraData& d,
    rust::Slice<const uint8_t> graph_file
) {
    d.behaviorGraphFile.get().assign(
        reinterpret_cast<const char*>(graph_file.data()),
        graph_file.size()
    );
}

inline void set_behavior_graph_controls_base_skeleton(
    nifly::BSBehaviorGraphExtraData& d,
    bool controls
) {
    d.controlsBaseSkel = controls;
}

// Bone i is `bytes` split into `lengths`, as in set_strings_extra_data, at
// `distances[i]`.
inline size_t text_key_count(const nifly::NiTextKeyExtraData& obj) {
    return obj.textKeys.size();
}

inline float text_key_time(const nifly::NiTextKeyExtraData& obj, size_t index) {
    if (index >= obj.textKeys.size())
        throw std::out_of_range("text key index");
    return obj.textKeys.data()[index].time;
}

inline rust::Slice<const uint8_t>
text_key_text(const nifly::NiTextKeyExtraData& obj, size_t index) {
    if (index >= obj.textKeys.size())
        throw std::out_of_range("text key index");
    return extra_bytes(obj.textKeys.data()[index].value.get());
}

// Key i is `bytes` split into `lengths`, as in set_strings_extra_data, at
// `times[i]`. A key whose text is unchanged keeps its string reference, so an
// empty text stays an empty string rather than becoming no string.
inline void set_text_keys(
    nifly::NiTextKeyExtraData& d,
    rust::Slice<const uint8_t> bytes,
    rust::Slice<const size_t> lengths,
    rust::Slice<const float> times
) {
    std::vector<nifly::NiTextKey> old(d.textKeys.begin(), d.textKeys.end());
    d.textKeys.clear();
    auto texts = split_strings(bytes, lengths);
    for (size_t i = 0; i < std::min(texts.size(), times.size()); i++) {
        nifly::NiTextKey key;
        if (i < old.size() && old[i].value.get() == texts[i])
            key.value = old[i].value;
        else
            key.value.get() = texts[i];
        key.time = times[i];
        d.textKeys.push_back(key);
    }
}

inline void set_bone_lods(
    nifly::BSBoneLODExtraData& d,
    rust::Slice<const uint8_t> bytes,
    rust::Slice<const size_t> lengths,
    rust::Slice<const uint32_t> distances
) {
    d.boneLODs.clear();
    auto bones = split_strings(bytes, lengths);
    for (size_t i = 0; i < std::min(bones.size(), distances.size()); i++) {
        nifly::BoneLOD lod;
        lod.distance = distances[i];
        lod.boneName.get() = bones[i];
        d.boneLODs.push_back(lod);
    }
}

// Bone i is `bytes` split into `lengths` with `translations[i]`. Keeps
// numTranslations, which is written as the count, in step.
inline void set_bone_translations(
    nifly::BoneTranslations& d,
    rust::Slice<const uint8_t> bytes,
    rust::Slice<const size_t> lengths,
    rust::Slice<const Vec3> translations
) {
    d.translations.clear();
    auto bones = split_strings(bytes, lengths);
    for (size_t i = 0; i < std::min(bones.size(), translations.size()); i++) {
        nifly::BoneTranslation t;
        t.bone.get() = bones[i];
        t.trans = nifly::Vector3(
            translations[i].x,
            translations[i].y,
            translations[i].z
        );
        d.translations.push_back(t);
    }
    d.numTranslations = static_cast<uint32_t>(d.translations.size());
}
} // namespace nifly_rs

// The rest of the shims, by topic.
// In this order: later parts use earlier ones.
// clang-format off
#include "extra_data_shim_packed_geom.h"
// clang-format on

#undef NIFLY_RS_EXTRA_DATA
