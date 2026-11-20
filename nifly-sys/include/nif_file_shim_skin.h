// Part of nif_file_shim.h: skin bones, weights, transforms and partitions.
#pragma once

#include "nif_file_shim.h"

namespace nifly_rs {
// NifFile::GetNodeTransformToGlobal. Returns false, leaving `out` unchanged,
// if no node has that name.
// NifFile::CalcShapeTransformGlobalToSkin. Takes a non-const shape but only
// reads it.
inline bool calc_shape_transform_global_to_skin(
    const nifly::NifFile& file,
    const nifly::NiShape& shape,
    Transform& out
) {
    nifly::MatTransform xform;
    if (!file.CalcShapeTransformGlobalToSkin(
            const_cast<nifly::NiShape*>(&shape),
            xform
        ))
        return false;

    out = to_ffi(xform);
    return true;
}

// NifFile::GetShapeTransformGlobalToSkin. Takes a non-const shape but only
// reads it.
inline bool shape_transform_global_to_skin(
    const nifly::NifFile& file,
    const nifly::NiShape& shape,
    Transform& out
) {
    nifly::MatTransform xform;
    if (!file.GetShapeTransformGlobalToSkin(
            const_cast<nifly::NiShape*>(&shape),
            xform
        ))
        return false;

    out = to_ffi(xform);
    return true;
}

// Sets the skeleton root of the NiSkinInstance (to an NiNode) or BSSkinInstance
// (to an NiAVObject) at `skin_index`. False, changing nothing, if either block
// is missing or of the wrong type.
inline bool
set_skin_root(nifly::NifFile& file, uint32_t skin_index, uint32_t root_index) {
    auto& hdr = file.GetHeader();
    if (auto inst = hdr.GetBlock<nifly::NiSkinInstance>(skin_index)) {
        if (!hdr.GetBlock<nifly::NiNode>(root_index))
            return false;
        inst->targetRef.index = root_index;
        return true;
    }
    if (auto inst = hdr.GetBlock<nifly::BSSkinInstance>(skin_index)) {
        if (!hdr.GetBlock<nifly::NiAVObject>(root_index))
            return false;
        inst->targetRef.index = root_index;
        return true;
    }
    return false;
}

// NifFile::SetShapeTransformSkinToBone, which doesn't report failure. The
// getter has the same conditions (skin data found, index in range), so it
// decides whether the set happens.
inline bool set_shape_transform_skin_to_bone(
    nifly::NifFile& file,
    uint32_t shape_index,
    uint32_t bone_index,
    Transform transform
) {
    auto shape = file.GetHeader().GetBlock<nifly::NiShape>(shape_index);
    nifly::MatTransform current;
    if (!shape || !file.GetShapeTransformSkinToBone(shape, bone_index, current))
        return false;
    file.SetShapeTransformSkinToBone(shape, bone_index, from_ffi(transform));
    return true;
}

// NifFile::GetShapeBoneBounds. Takes a non-const shape but only reads it.
inline bool shape_bone_bounds(
    const nifly::NifFile& file,
    const nifly::NiShape& shape,
    uint32_t bone_index,
    BoundingSphere& out
) {
    nifly::BoundingSphere bounds;
    if (!file.GetShapeBoneBounds(
            const_cast<nifly::NiShape*>(&shape),
            bone_index,
            bounds
        ))
        return false;

    out = to_ffi(bounds);
    return true;
}

// NifFile::SetShapeBoneBounds, but on the shape at `shape_index`. Upstream finds
// the shape by name, which picks the first shape with that name when names
// repeat. Same logic otherwise: BSSkin::BoneData first, then NiSkinData.
inline bool set_shape_bone_bounds(
    nifly::NifFile& file,
    uint32_t shape_index,
    uint32_t bone_index,
    BoundingSphere bounds
) {
    auto& hdr = file.GetHeader();
    auto shape = hdr.GetBlock<nifly::NiShape>(shape_index);
    if (!shape)
        return false;

    if (auto bsSkin =
            hdr.GetBlock<nifly::BSSkinInstance>(shape->SkinInstanceRef())) {
        auto data = hdr.GetBlock(bsSkin->dataRef);
        if (!data || bone_index >= data->nBones)
            return false;
        data->boneXforms[bone_index].bounds = from_ffi(bounds);
        return true;
    }

    auto skinInst =
        hdr.GetBlock<nifly::NiSkinInstance>(shape->SkinInstanceRef());
    if (!skinInst)
        return false;
    auto data = hdr.GetBlock(skinInst->dataRef);
    if (!data || bone_index >= data->numBones)
        return false;
    data->bones[bone_index].bounds = from_ffi(bounds);
    return true;
}

// NifFile::GetShapeTransformSkinToBone (bone index). Takes a non-const shape
// but only reads it.
inline bool shape_transform_skin_to_bone(
    const nifly::NifFile& file,
    const nifly::NiShape& shape,
    uint32_t bone_index,
    Transform& out
) {
    nifly::MatTransform xform;
    if (!file.GetShapeTransformSkinToBone(
            const_cast<nifly::NiShape*>(&shape),
            bone_index,
            xform
        ))
        return false;

    out = to_ffi(xform);
    return true;
}

// NifFile::GetShapeBoneList. Names of the bone nodes used by the shape's skin
// instance, or from its SkinAttach extra data (Starfield). Empty if the shape
// isn't skinned. GetShapeBoneList takes a non-const pointer but only reads.
inline std::unique_ptr<std::vector<std::string>>
shape_bone_list(const nifly::NifFile& file, const nifly::NiShape& shape) {
    auto out = std::make_unique<std::vector<std::string>>();
    file.GetShapeBoneList(const_cast<nifly::NiShape*>(&shape), *out);
    return out;
}

// NifFile::GetShapeBoneIDList. Block indices of the bone nodes in the shape's
// skin instance, skipping empty refs. Empty if the shape isn't skinned. Unlike
// GetShapeBoneList, this doesn't read Starfield SkinAttach names.
// GetShapeBoneIDList takes a non-const pointer but only reads.
inline rust::Vec<uint32_t>
shape_bone_ids(const nifly::NifFile& file, const nifly::NiShape& shape) {
    std::vector<int> ids;
    file.GetShapeBoneIDList(const_cast<nifly::NiShape*>(&shape), ids);

    rust::Vec<uint32_t> out;
    out.reserve(ids.size());
    for (int id : ids)
        out.push_back(static_cast<uint32_t>(id));
    return out;
}

// NifFile::SetShapeVertWeights, reimplemented on the shape at `shape_index`:
// upstream looks the shape up by name, which finds the wrong one when names
// repeat. Upstream also silently ignores shapes without
// per-vertex weights and out-of-range vertices, and drops extra weights. This
// returns false, changing nothing,
// unless the shape is a skinned BSTriShape or a BSGeometry mesh with weights,
// the vertex exists, there are at most as many weights as the vertex holds,
// every bone is a position in the skin instance's bone list that fits the
// stored size (a byte for BSTriShape, 16 bits for BSGeometry), and every
// weight is finite and not negative.
inline bool set_shape_vertex_weights(
    nifly::NifFile& file,
    uint32_t shape_index,
    uint32_t vertex,
    rust::Slice<const uint32_t> bones,
    rust::Slice<const float> weights
) {
    auto& hdr = file.GetHeader();
    auto shape = hdr.GetBlock<nifly::NiShape>(shape_index);
    if (!shape || bones.size() != weights.size())
        return false;
    auto skin = hdr.GetBlock<nifly::NiBoneContainer>(shape->SkinInstanceRef());
    if (!skin)
        return false;

    size_t slots = 0;
    uint32_t bone_limit = skin->boneRefs.GetSize();
    if (auto geom = dynamic_cast<nifly::BSGeometry*>(shape)) {
        auto data =
            dynamic_cast<nifly::BSGeometryMeshData*>(geom->GetGeomData());
        if (!data || vertex >= data->skinWeights.size())
            return false;
        slots = data->nWeightsPerVert ? data->nWeightsPerVert : 4;
        bone_limit = std::min<uint32_t>(bone_limit, 65536);
    } else if (auto tri = dynamic_cast<nifly::BSTriShape*>(shape)) {
        if (!tri->IsSkinned() || vertex >= tri->vertData.size())
            return false;
        slots = 4;
        bone_limit = std::min<uint32_t>(bone_limit, 256);
    } else
        return false;

    if (bones.size() > slots)
        return false;
    for (size_t i = 0; i < bones.size(); i++)
        if (bones[i] >= bone_limit || !std::isfinite(weights[i])
            || weights[i] < 0.0f)
            return false;

    // As upstream: normalize to a sum of 1 and clear the unused slots.
    float sum = 0.0f;
    for (float w : weights)
        sum += w;
    if (sum <= 0.0f)
        sum = 1.0f;

    if (auto geom = dynamic_cast<nifly::BSGeometry*>(shape)) {
        auto data =
            static_cast<nifly::BSGeometryMeshData*>(geom->GetGeomData());
        auto& slots_out = data->skinWeights[vertex];
        slots_out.assign(slots, nifly::BSGeometryMeshData::BoneWeight {});
        for (size_t i = 0; i < bones.size(); i++) {
            slots_out[i].boneIndex = static_cast<uint16_t>(bones[i]);
            slots_out[i].weight =
                static_cast<uint16_t>(std::lround(weights[i] / sum * 65535.0f));
        }
        return true;
    }

    auto& v = static_cast<nifly::BSTriShape*>(shape)->vertData[vertex];
    std::memset(v.weights, 0, sizeof(v.weights));
    std::memset(v.weightBones, 0, sizeof(v.weightBones));
    for (size_t i = 0; i < bones.size(); i++) {
        v.weightBones[i] = static_cast<uint8_t>(bones[i]);
        v.weights[i] = weights[i] / sum;
    }
    return true;
}

// NifFile::ClearShapeVertWeights, reimplemented on the shape at `shape_index`
// for the same reason as set_shape_vertex_weights. Returns false, changing
// nothing, unless the shape is a skinned BSTriShape or a skinned BSGeometry
// with mesh data; upstream silently ignores other shapes.
inline bool
clear_shape_vertex_weights(nifly::NifFile& file, uint32_t shape_index) {
    auto& hdr = file.GetHeader();
    auto shape = hdr.GetBlock<nifly::NiShape>(shape_index);
    if (!shape
        || !hdr.GetBlock<nifly::NiBoneContainer>(shape->SkinInstanceRef()))
        return false;

    if (auto geom = dynamic_cast<nifly::BSGeometry*>(shape)) {
        auto data =
            dynamic_cast<nifly::BSGeometryMeshData*>(geom->GetGeomData());
        if (!data)
            return false;
        // As upstream: one zeroed entry of nWeightsPerVert slots per vertex.
        if (data->nWeightsPerVert == 0)
            data->nWeightsPerVert = 4;
        data->skinWeights.assign(
            data->vertices.size(),
            std::vector<nifly::BSGeometryMeshData::BoneWeight>(
                data->nWeightsPerVert
            )
        );
        return true;
    }

    auto tri = dynamic_cast<nifly::BSTriShape*>(shape);
    if (!tri || !tri->IsSkinned())
        return false;
    for (auto& v : tri->vertData) {
        std::memset(v.weights, 0, sizeof(v.weights));
        std::memset(v.weightBones, 0, sizeof(v.weightBones));
    }
    return true;
}

// NifFile::CreateSkinning on the shape at `shape_index`, which returns nothing.
// Returns false, changing nothing, unless the shape is one upstream knows how to
// skin (NiTriShape, NiTriStrips, BSTriShape, BSGeometry) and has no skin
// instance yet; upstream silently skips the rest.
inline bool create_skinning(nifly::NifFile& file, uint32_t shape_index) {
    auto shape = file.GetHeader().GetBlock<nifly::NiShape>(shape_index);
    if (!shape || !shape->SkinInstanceRef()
        || !shape->SkinInstanceRef()->IsEmpty())
        return false;
    if (!shape->HasType<nifly::NiTriShape>()
        && !shape->HasType<nifly::NiTriStrips>()
        && !shape->HasType<nifly::BSTriShape>()
        && !shape->HasType<nifly::BSGeometry>())
        return false;
    file.CreateSkinning(shape);
    return true;
}

// NifFile::DeleteSkinning on the shape at `shape_index`. Upstream deletes the
// skin instance, its skin data, partition and bone data even when other blocks
// share them. This unlinks the shape and deletes only the ones nothing else
// refers to, then lets upstream clear the skinning flags (and, for BSGeometry,
// its SkinAttach and mesh weights). Returns false, changing nothing, if the
// block isn't a shape with a skin instance.
inline bool delete_skinning(nifly::NifFile& file, uint32_t shape_index) {
    auto& hdr = file.GetHeader();
    auto shape = hdr.GetBlock<nifly::NiShape>(shape_index);
    if (!shape || !shape->SkinInstanceRef()
        || shape->SkinInstanceRef()->IsEmpty())
        return false;

    uint32_t inst_id = shape->SkinInstanceRef()->index;
    std::vector<uint32_t> owned;
    if (auto inst = hdr.GetBlock<nifly::NiSkinInstance>(inst_id)) {
        owned.push_back(inst->dataRef.index);
        owned.push_back(inst->skinPartitionRef.index);
    } else if (auto inst = hdr.GetBlock<nifly::BSSkinInstance>(inst_id))
        owned.push_back(inst->dataRef.index);
    shape->SkinInstanceRef()->Clear();

    // The instance first: until it's gone, it still refers to its data.
    if (hdr.GetBlock<nifly::NiObject>(inst_id)
        && hdr.GetBlockRefCount(inst_id, false) == 0) {
        hdr.DeleteBlock(inst_id);
        for (auto& id : owned)
            if (id != nifly::NIF_NPOS && id > inst_id)
                id--;
        std::sort(owned.begin(), owned.end(), std::greater<>());
        for (uint32_t id : owned)
            if (id != nifly::NIF_NPOS && hdr.GetBlock<nifly::NiObject>(id)
                && hdr.GetBlockRefCount(id, false) == 0)
                hdr.DeleteBlock(id);
    }

    file.DeleteSkinning(shape);
    return true;
}

// NifFile::SetShapeBoneIDList on the shape at `shape_index`. Returns false,
// changing nothing, unless the shape has a skin instance, isn't a BSGeometry
// (which binds bones by name; see set_shape_bone_names), and every id is a
// node block. Upstream stores any index it's given.
inline bool set_shape_bone_ids(
    nifly::NifFile& file,
    uint32_t shape_index,
    rust::Slice<const uint32_t> ids
) {
    auto& hdr = file.GetHeader();
    auto shape = hdr.GetBlock<nifly::NiShape>(shape_index);
    if (!shape || shape->HasType<nifly::BSGeometry>()
        || !hdr.GetBlock<nifly::NiBoneContainer>(shape->SkinInstanceRef()))
        return false;
    std::vector<int> list;
    list.reserve(ids.size());
    for (uint32_t id : ids) {
        if (!hdr.GetBlock<nifly::NiNode>(id))
            return false;
        list.push_back(static_cast<int>(id));
    }
    file.SetShapeBoneIDList(shape, list);
    return true;
}

// NifFile::SetShapeBoneList for a BSGeometry at `shape_index`, which writes the
// names to its SkinAttach extra data. `names` holds the names back to back,
// `lengths` their sizes. Returns false, changing nothing, unless the shape is a
// BSGeometry with a BSSkin::Instance and the lengths add up.
inline bool set_shape_bone_names(
    nifly::NifFile& file,
    uint32_t shape_index,
    rust::Slice<const uint8_t> names,
    rust::Slice<const size_t> lengths
) {
    auto& hdr = file.GetHeader();
    auto shape = hdr.GetBlock<nifly::BSGeometry>(shape_index);
    if (!shape
        || !hdr.GetBlock<nifly::BSSkinInstance>(shape->SkinInstanceRef()))
        return false;
    std::vector<std::string> list;
    list.reserve(lengths.size());
    size_t pos = 0;
    for (size_t len : lengths) {
        if (len > names.size() - pos)
            return false;
        list.emplace_back(
            reinterpret_cast<const char*>(names.data() + pos),
            len
        );
        pos += len;
    }
    if (pos != names.size())
        return false;
    file.SetShapeBoneList(shape, list);
    return true;
}

// NifFile::UpdateShapeBoneID, reimplemented on the shape at `shape_index`
// rather than the first shape with its name: points the first of its bones that
// is `old_id` to `new_id`. Returns false, changing nothing, unless the shape is
// skinned, `new_id` is a node block and some bone is `old_id`; upstream checks
// neither and stores any index.
inline bool replace_shape_bone(
    nifly::NifFile& file,
    uint32_t shape_index,
    uint32_t old_id,
    uint32_t new_id
) {
    auto& hdr = file.GetHeader();
    auto shape = hdr.GetBlock<nifly::NiShape>(shape_index);
    if (!shape || !hdr.GetBlock<nifly::NiNode>(new_id))
        return false;
    auto bones = hdr.GetBlock<nifly::NiBoneContainer>(shape->SkinInstanceRef());
    if (!bones)
        return false;
    for (auto& bone : bones->boneRefs) {
        if (!bone.IsEmpty() && bone.index == old_id) {
            bone.index = new_id;
            return true;
        }
    }
    return false;
}

// NifFile::GetShapeBoneWeights, as a vector sorted by vertex index instead of
// an unordered_map. `bone_index` is the position in the shape's bone list.
// GetShapeBoneWeights takes a non-const pointer but only reads.
// NifFile::SetShapeBoneWeights, but on the shape at `shape_index` rather than
// the first shape with its name. Only NiSkinData weights are set, as upstream;
// BSTriShape and BSGeometry are rejected because GetShapeBoneWeights reads
// their per-vertex weights instead. Weights below 0.0001 are dropped, as upstream.
inline bool set_shape_bone_weights(
    nifly::NifFile& file,
    uint32_t shape_index,
    uint32_t bone_index,
    rust::Slice<const VertexWeight> weights
) {
    auto& hdr = file.GetHeader();
    auto shape = hdr.GetBlock<nifly::NiShape>(shape_index);
    if (!shape || dynamic_cast<nifly::BSTriShape*>(shape)
        || dynamic_cast<nifly::BSGeometry*>(shape))
        return false;

    auto skinInst =
        hdr.GetBlock<nifly::NiSkinInstance>(shape->SkinInstanceRef());
    if (!skinInst)
        return false;
    auto data = hdr.GetBlock(skinInst->dataRef);
    if (!data || bone_index >= data->numBones)
        return false;

    uint16_t numVertices = shape->GetNumVertices();
    for (const auto& w : weights)
        if (w.vertex >= numVertices)
            return false;

    data->hasVertWeights = true;
    auto& bone = data->bones[bone_index];
    bone.vertexWeights.clear();
    for (const auto& w : weights)
        if (w.weight >= 0.0001f)
            bone.vertexWeights.emplace_back(
                nifly::SkinWeight(w.vertex, w.weight)
            );
    bone.numVertices = static_cast<uint16_t>(bone.vertexWeights.size());
    return true;
}

// NifFile::UpdateSkinPartitions on the shape at `shape_index`, which returns
// nothing. Checks first what it would silently skip, or where it would read or
// write out of bounds:
// - the block isn't a shape with an NiSkinInstance, NiSkinData and
//   NiSkinPartition, or its triangles can't be read;
// - the NiSkinData has no vertex weights (BSTriShape files may keep them only
//   per vertex): every partition would lose its bones;
// - the shape has triangles but the partition has no partitions (every
//   triangle goes to partition 0, indexing an empty vector);
// - a BSDismemberSkinInstance lists a different number of partitions than the
//   NiSkinPartition (splitting indexes and inserts by partition);
// - with no per-partition bone limit (Skyrim LE and others), more than 256
//   bones, whose partition-local indices would wrap in a uint8_t.
// The shape at `shape_index` if UpdateSkinPartitions can run on it, apart from
// the partition-count checks below (which set_shape_partitions satisfies by
// construction), with its triangles. Null otherwise.
inline nifly::NiShape* updatable_skin(
    nifly::NifFile& file,
    uint32_t shape_index,
    std::vector<nifly::Triangle>& tris
) {
    auto& hdr = file.GetHeader();
    auto shape = hdr.GetBlock<nifly::NiShape>(shape_index);
    if (!shape || !shape->SkinInstanceRef())
        return nullptr;
    auto skinInst =
        hdr.GetBlock<nifly::NiSkinInstance>(shape->SkinInstanceRef());
    if (!skinInst)
        return nullptr;
    auto data = hdr.GetBlock(skinInst->dataRef);
    auto part = hdr.GetBlock(skinInst->skinPartitionRef);
    if (!data || !part || !data->hasVertWeights)
        return nullptr;
    if (!shape->GetTriangles(tris))
        return nullptr;

    auto version = hdr.GetVersion();
    bool limited = version.IsOB() || version.IsFO3() || version.IsSSE();
    if (!limited && data->bones.size() > 256)
        return nullptr;
    return shape;
}

inline bool update_skin_partitions(nifly::NifFile& file, uint32_t shape_index) {
    std::vector<nifly::Triangle> tris;
    auto shape = updatable_skin(file, shape_index, tris);
    if (!shape)
        return false;
    auto& hdr = file.GetHeader();
    auto skinInst =
        hdr.GetBlock<nifly::NiSkinInstance>(shape->SkinInstanceRef());
    auto part = hdr.GetBlock(skinInst->skinPartitionRef);
    if (!tris.empty() && part->partitions.empty())
        return false;

    auto dismember = dynamic_cast<nifly::BSDismemberSkinInstance*>(skinInst);
    if (dismember && dismember->partitions.size() != part->partitions.size())
        return false;

    // Upstream reuses a cached triangle-to-partition list when its length
    // matches, which keeps triangles in no partition out (after
    // delete_partitions) until the file is reloaded. Rebuilding it from the
    // partitions puts them in the first one every time.
    part->triParts.clear();
    file.UpdateSkinPartitions(shape);
    return true;
}

// NifFile::DeletePartitions on the shape at `shape_index`, with `indices` sorted
// and deduplicated here (upstream requires both). Returns false, changing
// nothing, unless the shape has an NiSkinInstance with an NiSkinPartition, every
// index is a partition, and a BSDismemberSkinInstance has as many partitions;
// upstream erases out of range otherwise.
inline bool delete_partitions(
    nifly::NifFile& file,
    uint32_t shape_index,
    rust::Slice<const uint32_t> indices
) {
    auto& hdr = file.GetHeader();
    auto shape = hdr.GetBlock<nifly::NiShape>(shape_index);
    if (!shape)
        return false;
    auto skinInst =
        hdr.GetBlock<nifly::NiSkinInstance>(shape->SkinInstanceRef());
    if (!skinInst)
        return false;
    auto part = hdr.GetBlock(skinInst->skinPartitionRef);
    if (!part)
        return false;
    auto dismember = dynamic_cast<nifly::BSDismemberSkinInstance*>(skinInst);
    if (dismember && dismember->partitions.size() != part->partitions.size())
        return false;

    std::vector<uint32_t> sorted(indices.begin(), indices.end());
    std::sort(sorted.begin(), sorted.end());
    sorted.erase(std::unique(sorted.begin(), sorted.end()), sorted.end());
    if (!sorted.empty() && sorted.back() >= part->partitions.size())
        return false;
    file.DeletePartitions(shape, sorted);
    return true;
}

// NifFile::GetShapeMaxBonesPerVertex.
inline uint8_t shape_max_bones_per_vertex(
    const nifly::NifFile& file,
    const nifly::NiShape& shape
) {
    return file.GetShapeMaxBonesPerVertex(const_cast<nifly::NiShape*>(&shape));
}

// NifFile::GetShapeBonesPerVertex. A BSGeometry's count is its first mesh's.
inline uint8_t shape_bones_per_vertex(
    const nifly::NifFile& file,
    const nifly::NiShape& shape
) {
    return file.GetShapeBonesPerVertex(const_cast<nifly::NiShape*>(&shape));
}

inline rust::Vec<VertexWeight> shape_bone_weights(
    const nifly::NifFile& file,
    const nifly::NiShape& shape,
    uint32_t bone_index
) {
    std::unordered_map<uint16_t, float> weights;
    file.GetShapeBoneWeights(
        const_cast<nifly::NiShape*>(&shape),
        bone_index,
        weights
    );

    std::vector<VertexWeight> sorted;
    sorted.reserve(weights.size());
    for (const auto& [vertex, weight] : weights)
        sorted.push_back(VertexWeight {vertex, weight});
    std::sort(
        sorted.begin(),
        sorted.end(),
        [](const VertexWeight& a, const VertexWeight& b) {
            return a.vertex < b.vertex;
        }
    );

    rust::Vec<VertexWeight> out;
    out.reserve(sorted.size());
    for (const auto& w : sorted)
        out.push_back(w);
    return out;
}
} // namespace nifly_rs
