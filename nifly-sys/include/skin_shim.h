#pragma once

#include <cstring>

#include "Nodes.hpp"
#include "Skin.hpp"
#include "rust/cxx.h"
#include "transform.h"

// For each skin type T with direct base Parent, defines:
// - upcast_<snake>: T -> Parent. Implicit derived-to-base conversion, always succeeds.
// - downcast_<snake>: NiObject -> T. Returns null unless obj is a T or a subclass of T.
#define NIFLY_RS_SKIN(T, Parent, snake) \
    inline const nifly::Parent& upcast_##snake(const nifly::T& obj) { \
        return obj; \
    } \
    inline const nifly::T* downcast_##snake(const nifly::NiObject& obj) { \
        return dynamic_cast<const nifly::T*>(&obj); \
    }

namespace nifly_rs {
NIFLY_RS_SKIN(NiSkinData, NiObject, ni_skin_data)
NIFLY_RS_SKIN(NiSkinPartition, NiObject, ni_skin_partition)
NIFLY_RS_SKIN(NiBoneContainer, NiObject, ni_bone_container)
NIFLY_RS_SKIN(NiSkinInstance, NiBoneContainer, ni_skin_instance)
NIFLY_RS_SKIN(
    BSDismemberSkinInstance,
    NiSkinInstance,
    bs_dismember_skin_instance
)
NIFLY_RS_SKIN(BSSkinInstance, NiBoneContainer, bs_skin_instance)
NIFLY_RS_SKIN(BSSkinBoneData, NiObject, bs_skin_bone_data)

inline Transform
skin_data_transform_global_to_skin(const nifly::NiSkinData& data) {
    return to_ffi(data.skinTransform);
}

// bones.size() rather than numBones, so indexing can't go past the vector.
inline size_t skin_data_bone_count(const nifly::NiSkinData& data) {
    return data.bones.size();
}

inline bool skin_data_transform_skin_to_bone(
    const nifly::NiSkinData& data,
    size_t index,
    Transform& out
) {
    if (index >= data.bones.size())
        return false;
    out = to_ffi(data.bones[index].boneTransform);
    return true;
}

inline void set_skin_data_transform_global_to_skin(
    nifly::NiSkinData& data,
    Transform transform
) {
    data.skinTransform = from_ffi(transform);
}

inline bool skin_data_has_vertex_weights(const nifly::NiSkinData& data) {
    return data.hasVertWeights != 0;
}

inline size_t
dismember_partition_count(const nifly::BSDismemberSkinInstance& inst) {
    return inst.partitions.size();
}

// These return 0 if `index` is out of range. NiVector has no const operator[],
// so they cast away const to read.
inline uint16_t dismember_partition_flags(
    const nifly::BSDismemberSkinInstance& inst,
    size_t index
) {
    if (index >= inst.partitions.size())
        return 0;
    return const_cast<nifly::BSDismemberSkinInstance&>(inst)
        .partitions[index]
        .flags;
}

inline uint16_t dismember_partition_body_part(
    const nifly::BSDismemberSkinInstance& inst,
    size_t index
) {
    if (index >= inst.partitions.size())
        return 0;
    return const_cast<nifly::BSDismemberSkinInstance&>(inst)
        .partitions[index]
        .partID;
}

// Returns false if `index` is out of range.
inline bool set_dismember_partition_flags(
    nifly::BSDismemberSkinInstance& inst,
    size_t index,
    uint16_t flags
) {
    if (index >= inst.partitions.size())
        return false;
    inst.partitions[index].flags = nifly::PartitionFlags(flags);
    return true;
}

inline size_t skin_partition_count(const nifly::NiSkinPartition& part) {
    return part.partitions.size();
}

// These return empty or 0 if `index` is out of range.
inline rust::Slice<const uint16_t>
skin_partition_bones(const nifly::NiSkinPartition& part, size_t index) {
    if (index >= part.partitions.size())
        return {};
    const auto& bones = part.partitions[index].bones;
    return {bones.data(), bones.size()};
}

inline rust::Slice<const uint16_t>
skin_partition_vertex_map(const nifly::NiSkinPartition& part, size_t index) {
    if (index >= part.partitions.size())
        return {};
    const auto& map = part.partitions[index].vertexMap;
    return {map.data(), map.size()};
}

inline uint16_t skin_partition_triangle_count(
    const nifly::NiSkinPartition& part,
    size_t index
) {
    if (index >= part.partitions.size())
        return 0;
    return part.partitions[index].numTriangles;
}

// NiSkinData::bones[index].bounds. False, leaving `out` unchanged, if `index`
// is out of range.
inline bool skin_data_bone_bounds(
    const nifly::NiSkinData& data,
    size_t index,
    BoundingSphere& out
) {
    if (index >= data.bones.size())
        return false;
    out = to_ffi(data.bones[index].bounds);
    return true;
}

inline void
set_skin_data_has_vertex_weights(nifly::NiSkinData& data, bool value) {
    data.hasVertWeights = value ? 1 : 0;
}

inline size_t skin_bone_data_bone_count(const nifly::BSSkinBoneData& data) {
    return data.boneXforms.size();
}

// False, leaving `out` unchanged, if `index` is out of range.
inline bool skin_bone_data_transform_skin_to_bone(
    const nifly::BSSkinBoneData& data,
    size_t index,
    Transform& out
) {
    if (index >= data.boneXforms.size())
        return false;
    out = to_ffi(data.boneXforms[index].boneTransform);
    return true;
}

inline bool skin_bone_data_bounds(
    const nifly::BSSkinBoneData& data,
    size_t index,
    BoundingSphere& out
) {
    if (index >= data.boneXforms.size())
        return false;
    out = to_ffi(data.boneXforms[index].bounds);
    return true;
}

// BSSkinInstance::scales. NiVector has no const iteration, so this casts away
// const to read.
inline rust::Vec<Vec3> skin_instance_scales(const nifly::BSSkinInstance& inst) {
    auto& scales = const_cast<nifly::BSSkinInstance&>(inst).scales;
    rust::Vec<Vec3> out;
    for (const auto& v : scales)
        out.push_back({v.x, v.y, v.z});
    return out;
}

inline void set_skin_instance_scales(
    nifly::BSSkinInstance& inst,
    rust::Slice<const Vec3> scales
) {
    inst.scales.clear();
    for (const auto& v : scales) {
        nifly::Vector3 scale(v.x, v.y, v.z);
        inst.scales.push_back(scale);
    }
}

// The skeleton root pointer's block index of an NiSkinInstance or
// BSSkinInstance; NIF_NPOS if empty or another type.
inline uint32_t bone_container_root(const nifly::NiBoneContainer& skin) {
    if (auto inst = dynamic_cast<const nifly::NiSkinInstance*>(&skin))
        return inst->targetRef.index;
    if (auto inst = dynamic_cast<const nifly::BSSkinInstance*>(&skin))
        return inst->targetRef.index;
    return nifly::NIF_NPOS;
}

// Partition fields. Getters return empty or 0 if `index` is out of range, and
// setters false.
static_assert(sizeof(nifly::VertexWeight) == 4 * sizeof(float));
static_assert(sizeof(nifly::BoneIndices) == 4);
static_assert(sizeof(nifly::Triangle) == 3 * sizeof(uint16_t));

inline uint16_t skin_partition_weights_per_vertex(
    const nifly::NiSkinPartition& part,
    size_t index
) {
    if (index >= part.partitions.size())
        return 0;
    return part.partitions[index].numWeightsPerVertex;
}

// Flattened: 4 per vertex.
inline rust::Slice<const float> skin_partition_vertex_weights(
    const nifly::NiSkinPartition& part,
    size_t index
) {
    if (index >= part.partitions.size())
        return {};
    const auto& w = part.partitions[index].vertexWeights;
    return {reinterpret_cast<const float*>(w.data()), w.size() * 4};
}

// Flattened: 4 per vertex.
inline rust::Slice<const uint8_t>
skin_partition_bone_indices(const nifly::NiSkinPartition& part, size_t index) {
    if (index >= part.partitions.size())
        return {};
    const auto& b = part.partitions[index].boneIndices;
    return {reinterpret_cast<const uint8_t*>(b.data()), b.size() * 4};
}

// Flattened: 3 per triangle.
inline rust::Slice<const uint16_t>
skin_partition_triangles(const nifly::NiSkinPartition& part, size_t index) {
    if (index >= part.partitions.size())
        return {};
    const auto& t = part.partitions[index].triangles;
    return {reinterpret_cast<const uint16_t*>(t.data()), t.size() * 3};
}

inline size_t
skin_partition_strip_count(const nifly::NiSkinPartition& part, size_t index) {
    if (index >= part.partitions.size())
        return 0;
    return part.partitions[index].strips.size();
}

inline rust::Slice<const uint16_t> skin_partition_strip(
    const nifly::NiSkinPartition& part,
    size_t index,
    size_t strip
) {
    if (index >= part.partitions.size())
        return {};
    const auto& strips = part.partitions[index].strips;
    if (strip >= strips.size())
        return {};
    return {strips[strip].data(), strips[strip].size()};
}

inline uint8_t
skin_partition_lod_level(const nifly::NiSkinPartition& part, size_t index) {
    if (index >= part.partitions.size())
        return 0;
    return part.partitions[index].lodLevel;
}

inline bool set_skin_partition_lod_level(
    nifly::NiSkinPartition& part,
    size_t index,
    uint8_t value
) {
    if (index >= part.partitions.size())
        return false;
    part.partitions[index].lodLevel = value;
    return true;
}

inline bool
skin_partition_global_vb(const nifly::NiSkinPartition& part, size_t index) {
    if (index >= part.partitions.size())
        return false;
    return part.partitions[index].globalVB;
}

inline bool set_skin_partition_global_vb(
    nifly::NiSkinPartition& part,
    size_t index,
    bool value
) {
    if (index >= part.partitions.size())
        return false;
    part.partitions[index].globalVB = value;
    return true;
}

inline bool set_skin_data_transform_skin_to_bone(
    nifly::NiSkinData& data,
    size_t index,
    Transform t
) {
    if (index >= data.bones.size())
        return false;
    data.bones[index].boneTransform = from_ffi(t);
    return true;
}

inline bool set_skin_data_bone_bounds(
    nifly::NiSkinData& data,
    size_t index,
    BoundingSphere b
) {
    if (index >= data.bones.size())
        return false;
    data.bones[index].bounds = from_ffi(b);
    return true;
}

// Bone `index`'s weights as parallel vertex and weight lists. False if
// `index` is out of range.
inline bool skin_data_vertex_weights(
    const nifly::NiSkinData& data,
    size_t index,
    rust::Vec<uint16_t>& vertices,
    rust::Vec<float>& weights
) {
    if (index >= data.bones.size())
        return false;
    for (auto& w : data.bones[index].vertexWeights) {
        vertices.push_back(w.index);
        weights.push_back(w.weight);
    }
    return true;
}

// Replaces bone `index`'s weights, keeping numVertices in step. False,
// changing nothing, if `index` is out of range, the lists differ in length or
// there are more than 65535.
inline bool set_skin_data_vertex_weights(
    nifly::NiSkinData& data,
    size_t index,
    rust::Slice<const uint16_t> vertices,
    rust::Slice<const float> weights
) {
    if (index >= data.bones.size() || vertices.size() != weights.size()
        || vertices.size() > UINT16_MAX)
        return false;
    auto& bone = data.bones[index];
    bone.vertexWeights.clear();
    for (size_t i = 0; i < vertices.size(); i++)
        bone.vertexWeights.emplace_back(vertices[i], weights[i]);
    bone.numVertices = static_cast<uint16_t>(vertices.size());
    return true;
}

inline bool set_skin_bone_data_transform_skin_to_bone(
    nifly::BSSkinBoneData& data,
    size_t index,
    Transform t
) {
    if (index >= data.boneXforms.size())
        return false;
    data.boneXforms[index].boneTransform = from_ffi(t);
    return true;
}

inline bool set_skin_bone_data_bounds(
    nifly::BSSkinBoneData& data,
    size_t index,
    BoundingSphere b
) {
    if (index >= data.boneXforms.size())
        return false;
    data.boneXforms[index].bounds = from_ffi(b);
    return true;
}

inline bool set_dismember_partition_body_part(
    nifly::BSDismemberSkinInstance& inst,
    size_t index,
    uint16_t part
) {
    if (index >= inst.partitions.size())
        return false;
    inst.partitions[index].partID = part;
    return true;
}

// A partition's has* flags: 0 vertex map, 1 vertex weights, 2 faces, 3 bone
// indices. Null if `which` or `index` is out of range.
inline bool* partition_flag(
    const nifly::NiSkinPartition& part,
    size_t index,
    uint8_t which
) {
    if (index >= part.partitions.size())
        return nullptr;
    auto& p = const_cast<nifly::NiSkinPartition::PartitionBlock&>(
        part.partitions[index]
    );
    switch (which) {
        case 0:
            return &p.hasVertexMap;
        case 1:
            return &p.hasVertexWeights;
        case 2:
            return &p.hasFaces;
        case 3:
            return &p.hasBoneIndices;
        default:
            return nullptr;
    }
}

inline bool skin_partition_flag(
    const nifly::NiSkinPartition& part,
    size_t index,
    uint8_t which
) {
    bool* flag = partition_flag(part, index, which);
    return flag && *flag;
}

inline bool set_skin_partition_flag(
    nifly::NiSkinPartition& part,
    size_t index,
    uint8_t which,
    bool value
) {
    bool* flag = partition_flag(part, index, which);
    if (!flag)
        return false;
    *flag = value;
    return true;
}

// VertexDesc keeps its 64 bits private.
inline uint64_t skin_partition_vertex_desc(const nifly::NiSkinPartition& part) {
    uint64_t bits;
    std::memcpy(&bits, &part.vertexDesc, sizeof bits);
    return bits;
}

inline void
set_skin_partition_vertex_desc(nifly::NiSkinPartition& part, uint64_t bits) {
    std::memcpy(static_cast<void*>(&part.vertexDesc), &bits, sizeof bits);
}

inline uint64_t skin_partition_block_vertex_desc(
    const nifly::NiSkinPartition& part,
    size_t index
) {
    uint64_t bits = 0;
    if (index < part.partitions.size())
        std::memcpy(&bits, &part.partitions[index].vertexDesc, sizeof bits);
    return bits;
}

inline bool set_skin_partition_block_vertex_desc(
    nifly::NiSkinPartition& part,
    size_t index,
    uint64_t bits
) {
    if (index >= part.partitions.size())
        return false;
    std::memcpy(
        static_cast<void*>(&part.partitions[index].vertexDesc),
        &bits,
        sizeof bits
    );
    return true;
}

// Flattened (3 per triangle); empty if `index` is out of range.
inline rust::Slice<const uint16_t> skin_partition_true_triangles(
    const nifly::NiSkinPartition& part,
    size_t index
) {
    if (index >= part.partitions.size())
        return {};
    auto& tris = part.partitions[index].trueTriangles;
    return {reinterpret_cast<const uint16_t*>(tris.data()), tris.size() * 3};
}

// Replaces a partition's true triangles, one per mapped triangle. False,
// changing nothing, if `index` is out of range or the count differs from
// numTriangles.
inline bool set_skin_partition_true_triangles(
    nifly::NiSkinPartition& part,
    size_t index,
    rust::Slice<const uint16_t> flat
) {
    if (index >= part.partitions.size())
        return false;
    auto& p = part.partitions[index];
    if (flat.size() != size_t(p.numTriangles) * 3)
        return false;
    p.trueTriangles.resize(p.numTriangles);
    for (size_t i = 0; i < p.numTriangles; i++)
        p.trueTriangles[i] =
            nifly::Triangle(flat[i * 3], flat[i * 3 + 1], flat[i * 3 + 2]);
    return true;
}
} // namespace nifly_rs

#undef NIFLY_RS_SKIN
