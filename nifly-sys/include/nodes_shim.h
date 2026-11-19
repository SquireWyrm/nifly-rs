#pragma once
#include <cstddef>
#include <cstdint>
#include <cstring>

#include "NifFile.hpp"
#include "Nodes.hpp"
#include "Objects.hpp"
#include "Particles.hpp"
#include "rust/cxx.h"
#include "transform.h"

// For each node type T with direct base Parent, defines:
// - upcast_<snake>: T -> Parent. Implicit derived-to-base conversion, always succeeds.
// - downcast_<snake>: NiObject -> T. Returns null unless obj is a T or a subclass of T.
#define NIFLY_RS_NODE(T, Parent, snake) \
    inline const nifly::Parent& upcast_##snake(const nifly::T& obj) { \
        return obj; \
    } \
    inline const nifly::T* downcast_##snake(const nifly::NiObject& obj) { \
        return dynamic_cast<const nifly::T*>(&obj); \
    }

namespace nifly_rs {
NIFLY_RS_NODE(NiNode, NiAVObject, ni_node)
NIFLY_RS_NODE(BSFadeNode, NiNode, bs_fade_node)
NIFLY_RS_NODE(RootCollisionNode, NiNode, root_collision_node)
NIFLY_RS_NODE(AvoidNode, NiNode, avoid_node)
NIFLY_RS_NODE(NiBSAnimationNode, NiNode, ni_bs_animation_node)
NIFLY_RS_NODE(NiBSParticleNode, NiNode, ni_bs_particle_node)
NIFLY_RS_NODE(BSValueNode, NiNode, bs_value_node)
NIFLY_RS_NODE(BSLeafAnimNode, NiNode, bs_leaf_anim_node)
NIFLY_RS_NODE(BSTreeNode, NiNode, bs_tree_node)
NIFLY_RS_NODE(BSOrderedNode, NiNode, bs_ordered_node)
NIFLY_RS_NODE(BSMultiBoundNode, NiNode, bs_multi_bound_node)
NIFLY_RS_NODE(
    BSDistantObjectInstancedNode,
    BSMultiBoundNode,
    bs_distant_object_instanced_node
)
NIFLY_RS_NODE(BSRangeNode, NiNode, bs_range_node)
NIFLY_RS_NODE(BSDebrisNode, BSRangeNode, bs_debris_node)
NIFLY_RS_NODE(BSBlastNode, BSRangeNode, bs_blast_node)
NIFLY_RS_NODE(BSDamageStage, BSBlastNode, bs_damage_stage)
NIFLY_RS_NODE(BSWeakReferenceNode, NiNode, bs_weak_reference_node)
NIFLY_RS_NODE(BSFaceGenNiNode, NiNode, bs_face_gen_ni_node)
NIFLY_RS_NODE(NiBillboardNode, NiNode, ni_billboard_node)
NIFLY_RS_NODE(NiSwitchNode, NiNode, ni_switch_node)
NIFLY_RS_NODE(NiLODNode, NiSwitchNode, ni_lod_node)
NIFLY_RS_NODE(NiBone, NiNode, ni_bone)
NIFLY_RS_NODE(NiSortAdjustNode, NiNode, ni_sort_adjust_node)
NIFLY_RS_NODE(BSMasterParticleSystem, NiNode, bs_master_particle_system)
NIFLY_RS_NODE(BSMultiBound, NiObject, bs_multi_bound)
NIFLY_RS_NODE(NiLODData, NiObject, ni_lod_data)
NIFLY_RS_NODE(NiRangeLODData, NiLODData, ni_range_lod_data)
NIFLY_RS_NODE(NiScreenLODData, NiLODData, ni_screen_lod_data)
NIFLY_RS_NODE(BSMultiBoundData, NiObject, bs_multi_bound_data)
NIFLY_RS_NODE(BSMultiBoundOBB, BSMultiBoundData, bs_multi_bound_obb)
NIFLY_RS_NODE(BSMultiBoundAABB, BSMultiBoundData, bs_multi_bound_aabb)
NIFLY_RS_NODE(BSMultiBoundSphere, BSMultiBoundData, bs_multi_bound_sphere)
NIFLY_RS_NODE(NiCamera, NiAVObject, ni_camera)
NIFLY_RS_NODE(NiAVObjectPalette, NiObject, ni_av_object_palette)
NIFLY_RS_NODE(
    NiDefaultAVObjectPalette,
    NiAVObjectPalette,
    ni_default_av_object_palette
)

// Static NifFile::CanDeleteNode(NiNode*): true if every ref the node holds
// (children, effects, properties, collision, controller, extra data) is empty.
// It takes a non-const pointer because GetChildRefs isn't const, but it only
// reads.
inline bool can_delete_node(const nifly::NiNode& node) {
    return nifly::NifFile::CanDeleteNode(const_cast<nifly::NiNode*>(&node));
}

inline size_t child_count(const nifly::NiNode& node) {
    return node.childRefs.GetSize();
}

// Block indices of the child refs, in order. NIF_NPOS for empty refs.
inline rust::Vec<uint32_t> child_ref_indices(const nifly::NiNode& node) {
    rust::Vec<uint32_t> out;
    out.reserve(node.childRefs.GetSize());
    for (uint32_t i = 0; i < node.childRefs.GetSize(); i++)
        out.push_back(node.childRefs.GetBlockRef(i));
    return out;
}

// A getter `snake` and setter `set_snake` for the field `T::field`, converted
// with static_cast.
#define NIFLY_RS_NODE_FIELD(T, field, type, snake) \
    inline type snake(const nifly::T& node) { \
        return static_cast<type>(node.field); \
    } \
    inline void set_##snake(nifly::T& node, type value) { \
        node.field = static_cast<decltype(node.field)>(value); \
    }

NIFLY_RS_NODE_FIELD(BSValueNode, value, int32_t, value_node_value)
NIFLY_RS_NODE_FIELD(BSValueNode, valueFlags, uint8_t, value_node_flags)
NIFLY_RS_NODE_FIELD(
    BSOrderedNode,
    isStaticBound,
    bool,
    ordered_node_static_bound
)
NIFLY_RS_NODE_FIELD(BSRangeNode, min, uint8_t, range_node_min)
NIFLY_RS_NODE_FIELD(BSRangeNode, max, uint8_t, range_node_max)
NIFLY_RS_NODE_FIELD(BSRangeNode, current, uint8_t, range_node_current)
NIFLY_RS_NODE_FIELD(NiBillboardNode, billboardMode, uint16_t, billboard_mode)
NIFLY_RS_NODE_FIELD(NiSwitchNode, flags, uint16_t, switch_node_flags)
NIFLY_RS_NODE_FIELD(NiSwitchNode, index, uint32_t, switch_node_index)
NIFLY_RS_NODE_FIELD(NiSortAdjustNode, sortingMode, uint32_t, sorting_mode)
NIFLY_RS_NODE_FIELD(BSFaceGenNiNode, unkShort, uint16_t, face_gen_unknown)
NIFLY_RS_NODE_FIELD(BSWeakReferenceNode, unkInt1, uint32_t, weak_ref_unknown)
NIFLY_RS_NODE_FIELD(
    BSMasterParticleSystem,
    maxEmitterObjs,
    uint16_t,
    master_particle_max_emitters
)
NIFLY_RS_NODE_FIELD(
    NiCamera,
    numScreenPolygons,
    uint32_t,
    camera_screen_polygons
)
NIFLY_RS_NODE_FIELD(
    NiCamera,
    numScreenTextures,
    uint32_t,
    camera_screen_textures
)

#undef NIFLY_RS_NODE_FIELD

// BSOrderedNode::alphaSortBound as x, y, z, w. `out` and `v` have 4 elements.
inline void ordered_node_alpha_sort_bound(
    const nifly::BSOrderedNode& node,
    rust::Slice<float> out
) {
    const nifly::Vector4& v = node.alphaSortBound;
    out[0] = v.x;
    out[1] = v.y;
    out[2] = v.z;
    out[3] = v.w;
}

inline void set_ordered_node_alpha_sort_bound(
    nifly::BSOrderedNode& node,
    rust::Slice<const float> v
) {
    node.alphaSortBound = nifly::Vector4(v[0], v[1], v[2], v[3]);
}

// Block indices of BSTreeNode's `bones1` (list 1) or `bones2` (list 2), in
// order. NIF_NPOS for empty refs.
inline rust::Vec<uint32_t>
tree_node_bones(const nifly::BSTreeNode& node, uint8_t list) {
    const auto& bones = list == 1 ? node.bones1 : node.bones2;
    rust::Vec<uint32_t> out;
    out.reserve(bones.GetSize());
    for (uint32_t i = 0; i < bones.GetSize(); i++)
        out.push_back(bones.GetBlockRef(i));
    return out;
}

inline Vec3 bound_vec3_to_ffi(const nifly::Vector3& v) {
    return {v.x, v.y, v.z};
}

inline nifly::Vector3 bound_vec3_from_ffi(const Vec3& v) {
    return nifly::Vector3(v.x, v.y, v.z);
}

// A getter `snake` and setter `set_snake` for the Vector3 field `T::field`.
#define NIFLY_RS_VEC3_FIELD(T, field, snake) \
    inline Vec3 snake(const nifly::T& b) { \
        return bound_vec3_to_ffi(b.field); \
    } \
    inline void set_##snake(nifly::T& b, Vec3 v) { \
        b.field = bound_vec3_from_ffi(v); \
    }
NIFLY_RS_VEC3_FIELD(BSMultiBoundOBB, center, multi_bound_obb_center)
NIFLY_RS_VEC3_FIELD(BSMultiBoundOBB, size, multi_bound_obb_size)
NIFLY_RS_VEC3_FIELD(BSMultiBoundAABB, center, multi_bound_aabb_center)
NIFLY_RS_VEC3_FIELD(BSMultiBoundAABB, halfExtent, multi_bound_aabb_half_extent)
NIFLY_RS_VEC3_FIELD(BSMultiBoundSphere, center, multi_bound_sphere_center)
#undef NIFLY_RS_VEC3_FIELD

// BSMultiBoundOBB::rotation as rows. `out` and `rows` have 3 elements.
inline void multi_bound_obb_rotation(
    const nifly::BSMultiBoundOBB& b,
    rust::Slice<Vec3> out
) {
    for (int r = 0; r < 3; r++)
        out[r] = bound_vec3_to_ffi(b.rotation[r]);
}

inline void set_multi_bound_obb_rotation(
    nifly::BSMultiBoundOBB& b,
    rust::Slice<const Vec3> rows
) {
    for (int r = 0; r < 3; r++)
        b.rotation[r] = bound_vec3_from_ffi(rows[r]);
}

inline float multi_bound_sphere_radius(const nifly::BSMultiBoundSphere& b) {
    return b.radius;
}

inline void
set_multi_bound_sphere_radius(nifly::BSMultiBoundSphere& b, float radius) {
    b.radius = radius;
}

inline uint32_t
multi_bound_node_culling_mode(const nifly::BSMultiBoundNode& node) {
    return node.cullingMode;
}

inline void
set_multi_bound_node_culling_mode(nifly::BSMultiBoundNode& node, uint32_t v) {
    node.cullingMode = static_cast<nifly::BSCPCullingType>(v);
}

// Block indices of the refs; NIF_NPOS if empty.
inline uint32_t multi_bound_node_bound(const nifly::BSMultiBoundNode& node) {
    return node.multiBoundRef.index;
}

inline uint32_t multi_bound_data(const nifly::BSMultiBound& bound) {
    return bound.dataRef.index;
}

inline uint32_t lod_node_data(const nifly::NiLODNode& node) {
    return node.lodLevelData.index;
}

inline Vec3 range_lod_center(const nifly::NiRangeLODData& d) {
    return bound_vec3_to_ffi(d.lodCenter);
}

inline void set_range_lod_center(nifly::NiRangeLODData& d, Vec3 v) {
    d.lodCenter = bound_vec3_from_ffi(v);
}

// lodLevels flattened as near, far pairs. NiVector has no const iteration, so
// this casts away const to read.
inline rust::Vec<float> range_lod_levels(const nifly::NiRangeLODData& d) {
    rust::Vec<float> out;
    for (const auto& r : const_cast<nifly::NiRangeLODData&>(d).lodLevels) {
        out.push_back(r.nearExtent);
        out.push_back(r.farExtent);
    }
    return out;
}

// `levels` is near, far pairs.
inline void set_range_lod_levels(
    nifly::NiRangeLODData& d,
    rust::Slice<const float> levels
) {
    d.lodLevels.clear();
    for (size_t i = 0; i + 1 < levels.size(); i += 2) {
        nifly::LODRange r;
        r.nearExtent = levels[i];
        r.farExtent = levels[i + 1];
        d.lodLevels.push_back(r);
    }
}

inline BoundingSphere screen_lod_bound(const nifly::NiScreenLODData& d) {
    return to_ffi(nifly::BoundingSphere(d.boundCenter, d.boundRadius));
}

inline void set_screen_lod_bound(nifly::NiScreenLODData& d, BoundingSphere b) {
    d.boundCenter = bound_vec3_from_ffi(b.center);
    d.boundRadius = b.radius;
}

inline BoundingSphere screen_lod_world_bound(const nifly::NiScreenLODData& d) {
    return to_ffi(nifly::BoundingSphere(d.worldCenter, d.worldRadius));
}

inline void
set_screen_lod_world_bound(nifly::NiScreenLODData& d, BoundingSphere b) {
    d.worldCenter = bound_vec3_from_ffi(b.center);
    d.worldRadius = b.radius;
}

inline rust::Vec<float>
screen_lod_proportion_levels(const nifly::NiScreenLODData& d) {
    rust::Vec<float> out;
    for (float f : const_cast<nifly::NiScreenLODData&>(d).proportionLevels)
        out.push_back(f);
    return out;
}

inline void set_screen_lod_proportion_levels(
    nifly::NiScreenLODData& d,
    rust::Slice<const float> levels
) {
    d.proportionLevels.clear();
    for (float f : levels)
        d.proportionLevels.push_back(f);
}

// BSDistantObjectInstancedNode instances. A resource ID is passed as 3 u32s:
// file hash, extension bytes (little-endian) and directory hash. Transforms are
// flattened, 16 floats each. Getters return false or empty, and setters false,
// if `index` is out of range.
using DistantNode = nifly::BSDistantObjectInstancedNode;

inline size_t distant_instance_count(const DistantNode& node) {
    return node.instances.size();
}

inline bool distant_instance_resource(
    const DistantNode& node,
    size_t index,
    rust::Slice<uint32_t> out
) {
    if (index >= node.instances.size())
        return false;
    const auto& id = node.instances.data()[index].resourceID;
    out[0] = id.fileHash;
    std::memcpy(&out[1], id.extension, 4);
    out[2] = id.dirHash;
    return true;
}

inline rust::Vec<float>
distant_instance_transforms(const DistantNode& node, size_t index) {
    rust::Vec<float> out;
    if (index >= node.instances.size())
        return out;
    const auto& transforms = node.instances.data()[index].transforms;
    for (uint32_t t = 0; t < transforms.size(); t++)
        for (int i = 0; i < 16; i++)
            out.push_back(transforms.data()[t][i]);
    return out;
}

inline void fill_distant_instance(
    nifly::BSDistantObjectInstance& inst,
    rust::Slice<const uint32_t> resource,
    rust::Slice<const float> transforms
) {
    inst.resourceID.fileHash = resource[0];
    std::memcpy(inst.resourceID.extension, &resource[1], 4);
    inst.resourceID.dirHash = resource[2];
    inst.transforms.clear();
    for (size_t t = 0; t + 16 <= transforms.size(); t += 16) {
        nifly::Matrix4 m;
        for (int i = 0; i < 16; i++)
            m[i] = transforms[t + i];
        inst.transforms.push_back(m);
    }
}

// Keeps the instance's other data.
inline bool set_distant_instance(
    DistantNode& node,
    size_t index,
    rust::Slice<const uint32_t> resource,
    rust::Slice<const float> transforms
) {
    if (index >= node.instances.size())
        return false;
    fill_distant_instance(node.instances[index], resource, transforms);
    return true;
}

inline void push_distant_instance(
    DistantNode& node,
    rust::Slice<const uint32_t> resource,
    rust::Slice<const float> transforms
) {
    nifly::BSDistantObjectInstance inst;
    fill_distant_instance(inst, resource, transforms);
    node.instances.push_back(inst);
}

// An instance's unknown data as (unknown1, unknown2) pairs; empty if `index`
// is out of range.
inline rust::Vec<uint64_t>
distant_instance_unknown(const DistantNode& node, size_t index) {
    rust::Vec<uint64_t> out;
    if (index < node.instances.size())
        for (auto& u : const_cast<DistantNode&>(node)
                           .instances.data()[index]
                           .unknownData) {
            out.push_back(u.unknown1);
            out.push_back(u.unknown2);
        }
    return out;
}

// Sets an instance's unknown data from pairs. False if `index` is out of range
// or a value doesn't fit.
inline bool set_distant_instance_unknown(
    DistantNode& node,
    size_t index,
    rust::Slice<const uint64_t> pairs
) {
    if (index >= node.instances.size() || pairs.size() % 2 != 0)
        return false;
    for (size_t i = 1; i < pairs.size(); i += 2)
        if (pairs[i] > UINT32_MAX)
            return false;
    auto& data = node.instances.data()[index].unknownData;
    data.clear();
    for (size_t i = 0; i < pairs.size(); i += 2) {
        nifly::BSDistantObjectUnknown u;
        u.unknown1 = pairs[i];
        u.unknown2 = static_cast<uint32_t>(pairs[i + 1]);
        data.push_back(u);
    }
    return true;
}

inline uint8_t
distant_texture_arrays_unknown(const DistantNode& node, size_t slot) {
    return slot < 3 ? node.textureArrays[slot].unknownByte : 0;
}

inline bool set_distant_texture_arrays_unknown(
    DistantNode& node,
    size_t slot,
    uint8_t value
) {
    if (slot >= 3)
        return false;
    node.textureArrays[slot].unknownByte = value;
    return true;
}

inline bool remove_distant_instance(DistantNode& node, size_t index) {
    if (index >= node.instances.size())
        return false;
    node.instances.erase(static_cast<uint32_t>(index));
    return true;
}

// Texture arrays of slot `slot` (0 to 2). Getters return 0 or empty if out of
// range; setters do nothing.
inline size_t
distant_texture_array_count(const DistantNode& node, size_t slot) {
    return slot < 3 ? node.textureArrays[slot].textureArrays.size() : 0;
}

inline size_t
distant_texture_array_len(const DistantNode& node, size_t slot, size_t array) {
    if (array >= distant_texture_array_count(node, slot))
        return 0;
    return node.textureArrays[slot]
        .textureArrays.data()[array]
        .textureArray.size();
}

inline rust::Slice<const uint8_t> distant_texture_array_path(
    const DistantNode& node,
    size_t slot,
    size_t array,
    size_t index
) {
    if (index >= distant_texture_array_len(node, slot, array))
        return {};
    const auto& paths =
        node.textureArrays[slot].textureArrays.data()[array].textureArray;
    const std::string& str = paths.data()[index].get();
    return {reinterpret_cast<const uint8_t*>(str.data()), str.size()};
}

inline void clear_distant_texture_arrays(DistantNode& node, size_t slot) {
    if (slot < 3)
        node.textureArrays[slot].textureArrays.clear();
}

inline void add_distant_texture_array(DistantNode& node, size_t slot) {
    if (slot >= 3)
        return;
    nifly::BSTextureArray array;
    node.textureArrays[slot].textureArrays.push_back(array);
}

// Appends to the slot's last array; does nothing without one.
inline void push_distant_texture_array_path(
    DistantNode& node,
    size_t slot,
    rust::Slice<const uint8_t> path
) {
    if (slot >= 3 || node.textureArrays[slot].textureArrays.empty())
        return;
    auto& arrays = node.textureArrays[slot].textureArrays;
    nifly::NiString str;
    str.get().assign(reinterpret_cast<const char*>(path.data()), path.size());
    arrays[arrays.size() - 1].textureArray.push_back(str);
}

// Shared by BSWeakReferenceNode: resource IDs as 3 u32s and Matrix4s as 16
// floats, as for BSDistantObjectInstancedNode.
inline void
resource_to_ffi(const nifly::BSResourceID& id, rust::Slice<uint32_t> out) {
    out[0] = id.fileHash;
    std::memcpy(&out[1], id.extension, 4);
    out[2] = id.dirHash;
}

inline nifly::BSResourceID resource_from_ffi(rust::Slice<const uint32_t> r) {
    nifly::BSResourceID id;
    id.fileHash = r[0];
    std::memcpy(id.extension, &r[1], 4);
    id.dirHash = r[2];
    return id;
}

inline void push_matrix4(rust::Vec<float>& out, const nifly::Matrix4& m) {
    for (int i = 0; i < 16; i++)
        out.push_back(m[i]);
}

inline nifly::Matrix4 matrix4_from_ffi(rust::Slice<const float> f, size_t at) {
    nifly::Matrix4 m;
    for (int i = 0; i < 16; i++)
        m[i] = f[at + i];
    return m;
}

using WeakNode = nifly::BSWeakReferenceNode;

// Weak references. Getters return 0, false or empty if an index is out of
// range.
inline size_t weak_ref_count(const WeakNode& node) {
    return node.weakRefs.size();
}

inline bool weak_ref_resource(
    const WeakNode& node,
    size_t index,
    rust::Slice<uint32_t> out
) {
    if (index >= node.weakRefs.size())
        return false;
    resource_to_ffi(node.weakRefs[index].resourceID, out);
    return true;
}

inline uint32_t weak_ref_form_id(const WeakNode& node, size_t index) {
    return index < node.weakRefs.size() ? node.weakRefs[index].formID : 0;
}

inline rust::Vec<float>
weak_ref_transforms(const WeakNode& node, size_t index) {
    rust::Vec<float> out;
    if (index < node.weakRefs.size())
        for (const auto& m : node.weakRefs[index].transforms)
            push_matrix4(out, m);
    return out;
}

inline size_t weak_ref_material_count(const WeakNode& node, size_t index) {
    return index < node.weakRefs.size()
        ? node.weakRefs[index].unkMaterials.size()
        : 0;
}

// biome form ID, directory hash, file hash.
inline bool weak_ref_material(
    const WeakNode& node,
    size_t index,
    size_t material,
    rust::Slice<uint32_t> out
) {
    if (material >= weak_ref_material_count(node, index))
        return false;
    const auto& m = node.weakRefs[index].unkMaterials[material];
    out[0] = m.biomeFormID;
    out[1] = m.dirHash;
    out[2] = m.fileHash;
    return true;
}

inline rust::Slice<const uint8_t>
weak_ref_material_name(const WeakNode& node, size_t index, size_t material) {
    if (material >= weak_ref_material_count(node, index))
        return {};
    const std::string& str = node.weakRefs[index].unkMaterials[material].mat;
    return {reinterpret_cast<const uint8_t*>(str.data()), str.size()};
}

inline void clear_weak_refs(WeakNode& node) {
    node.weakRefs.clear();
    node.numWeakRefs = 0;
}

// `transforms` is 16 floats per matrix.
inline void push_weak_ref(
    WeakNode& node,
    uint32_t form_id,
    rust::Slice<const uint32_t> resource,
    rust::Slice<const float> transforms
) {
    nifly::BSWeakReference ref;
    ref.formID = form_id;
    ref.resourceID = resource_from_ffi(resource);
    for (size_t t = 0; t + 16 <= transforms.size(); t += 16)
        ref.transforms.push_back(matrix4_from_ffi(transforms, t));
    ref.numTransforms = static_cast<uint32_t>(ref.transforms.size());
    node.weakRefs.push_back(ref);
    node.numWeakRefs = static_cast<uint32_t>(node.weakRefs.size());
}

// Appends a material to the last weak reference; does nothing without one.
// `ids` is biome form ID, directory hash, file hash.
inline void push_weak_ref_material(
    WeakNode& node,
    rust::Slice<const uint32_t> ids,
    rust::Slice<const uint8_t> name
) {
    if (node.weakRefs.empty())
        return;
    auto& ref = node.weakRefs.back();
    nifly::UnkMaterialStruct m;
    m.biomeFormID = ids[0];
    m.dirHash = ids[1];
    m.fileHash = ids[2];
    m.mat.assign(reinterpret_cast<const char*>(name.data()), name.size());
    ref.unkMaterials.push_back(m);
    ref.numMaterials = static_cast<uint32_t>(ref.unkMaterials.size());
}

// Water references. Getters return false or empty if `index` is out of range.
inline size_t water_ref_count(const WeakNode& node) {
    return node.waterRefs.size();
}

inline rust::Vec<float>
water_ref_transform(const WeakNode& node, size_t index) {
    rust::Vec<float> out;
    if (index < node.waterRefs.size())
        push_matrix4(out, node.waterRefs[index].transform);
    return out;
}

inline bool water_ref_resource(
    const WeakNode& node,
    size_t index,
    rust::Slice<uint32_t> out
) {
    if (index >= node.waterRefs.size())
        return false;
    resource_to_ffi(node.waterRefs[index].resourceID, out);
    return true;
}

inline rust::Slice<const uint8_t>
water_ref_material(const WeakNode& node, size_t index) {
    if (index >= node.waterRefs.size())
        return {};
    const std::string& str = node.waterRefs[index].material.get();
    return {reinterpret_cast<const uint8_t*>(str.data()), str.size()};
}

inline uint32_t water_ref_unknown(const WeakNode& node, size_t index) {
    return index < node.waterRefs.size() ? node.waterRefs[index].unkInt1 : 0;
}

inline bool
set_water_ref_unknown(WeakNode& node, size_t index, uint32_t value) {
    if (index >= node.waterRefs.size())
        return false;
    node.waterRefs[index].unkInt1 = value;
    return true;
}

// Keeps existing entries' other data; new ones are default.
inline void resize_water_refs(WeakNode& node, size_t count) {
    node.waterRefs.resize(count);
    node.numWaterRefs = static_cast<uint32_t>(count);
}

// `transform` has 16 floats. False if `index` is out of range.
inline bool set_water_ref(
    WeakNode& node,
    size_t index,
    rust::Slice<const float> transform,
    rust::Slice<const uint32_t> resource,
    rust::Slice<const uint8_t> material
) {
    if (index >= node.waterRefs.size())
        return false;
    auto& ref = node.waterRefs[index];
    ref.transform = matrix4_from_ffi(transform, 0);
    ref.resourceID = resource_from_ffi(resource);
    ref.material.get().assign(
        reinterpret_cast<const char*>(material.data()),
        material.size()
    );
    return true;
}

inline uint16_t camera_flags(const nifly::NiCamera& c) {
    return c.obsoleteFlags;
}

inline void set_camera_flags(nifly::NiCamera& c, uint16_t v) {
    c.obsoleteFlags = v;
}

inline void camera_frustum(const nifly::NiCamera& c, rust::Slice<float> out) {
    out[0] = c.frustumLeft;
    out[1] = c.frustumRight;
    out[2] = c.frustumTop;
    out[3] = c.frustomBottom;
    out[4] = c.frustumNear;
    out[5] = c.frustumFar;
}

inline void set_camera_frustum(nifly::NiCamera& c, rust::Slice<const float> v) {
    c.frustumLeft = v[0];
    c.frustumRight = v[1];
    c.frustumTop = v[2];
    c.frustomBottom = v[3];
    c.frustumNear = v[4];
    c.frustumFar = v[5];
}

inline bool camera_orthographic(const nifly::NiCamera& c) {
    return c.useOrtho;
}

inline void set_camera_orthographic(nifly::NiCamera& c, bool v) {
    c.useOrtho = v;
}

inline void camera_viewport(const nifly::NiCamera& c, rust::Slice<float> out) {
    out[0] = c.viewportLeft;
    out[1] = c.viewportRight;
    out[2] = c.viewportTop;
    out[3] = c.viewportBottom;
}

inline void
set_camera_viewport(nifly::NiCamera& c, rust::Slice<const float> v) {
    c.viewportLeft = v[0];
    c.viewportRight = v[1];
    c.viewportTop = v[2];
    c.viewportBottom = v[3];
}

inline float camera_lod_adjust(const nifly::NiCamera& c) {
    return c.lodAdjust;
}

inline void set_camera_lod_adjust(nifly::NiCamera& c, float v) {
    c.lodAdjust = v;
}

inline uint32_t camera_scene(const nifly::NiCamera& c) {
    return c.sceneRef.index;
}

inline bool
set_camera_scene(nifly::NifFile& file, uint32_t camera_index, uint32_t scene) {
    auto& hdr = file.GetHeader();
    auto camera = hdr.GetBlock<nifly::NiCamera>(camera_index);
    if (!camera)
        return false;
    if (scene != nifly::NIF_NPOS && !hdr.GetBlock<nifly::NiAVObject>(scene))
        return false;
    camera->sceneRef.index = scene;
    return true;
}

inline uint32_t add_camera(
    nifly::NifFile& file,
    rust::Slice<const uint8_t> name,
    Transform transform,
    uint32_t parent_index
) {
    nifly::NiNode* parent = parent_index == nifly::NIF_NPOS
        ? file.GetRootNode()
        : file.GetHeader().GetBlock<nifly::NiNode>(parent_index);
    if (!parent)
        return nifly::NIF_NPOS;

    // The file format's defaults; the block's own are all zero.
    auto camera = std::make_unique<nifly::NiCamera>();
    camera->name.get().assign(
        reinterpret_cast<const char*>(name.data()),
        name.size()
    );
    camera->SetTransformToParent(from_ffi(transform));
    camera->frustumLeft = -0.63707f;
    camera->frustumRight = 0.63707f;
    camera->frustumTop = 0.385714f;
    camera->frustomBottom = -0.385714f;
    camera->frustumNear = 1.0f;
    camera->frustumFar = 5000.0f;
    camera->viewportRight = 1.0f;
    camera->viewportTop = 1.0f;
    camera->lodAdjust = 1.0f;
    uint32_t id = file.GetHeader().AddBlock(std::move(camera));
    parent->childRefs.AddBlockRef(id);
    return id;
}

inline uint32_t palette_scene(const nifly::NiDefaultAVObjectPalette& p) {
    return p.sceneRef.index;
}

inline size_t palette_object_count(const nifly::NiDefaultAVObjectPalette& p) {
    return p.objects.size();
}

inline rust::Slice<const uint8_t>
palette_object_name(const nifly::NiDefaultAVObjectPalette& p, size_t i) {
    const std::string& s = p.objects.data()[i].name.get();
    return {reinterpret_cast<const uint8_t*>(s.data()), s.size()};
}

inline uint32_t
palette_object(const nifly::NiDefaultAVObjectPalette& p, size_t i) {
    return p.objects.data()[i].objectRef.index;
}

inline bool set_palette_scene(
    nifly::NifFile& file,
    uint32_t palette_index,
    uint32_t scene
) {
    auto& hdr = file.GetHeader();
    auto palette = hdr.GetBlock<nifly::NiDefaultAVObjectPalette>(palette_index);
    if (!palette)
        return false;
    if (scene != nifly::NIF_NPOS && !hdr.GetBlock<nifly::NiAVObject>(scene))
        return false;
    palette->sceneRef.index = scene;
    return true;
}

inline bool set_palette_objects(
    nifly::NifFile& file,
    uint32_t palette_index,
    rust::Slice<const uint8_t> names,
    rust::Slice<const size_t> lengths,
    rust::Slice<const uint32_t> objects
) {
    auto& hdr = file.GetHeader();
    auto palette = hdr.GetBlock<nifly::NiDefaultAVObjectPalette>(palette_index);
    if (!palette || lengths.size() != objects.size())
        return false;
    size_t total = 0;
    for (size_t len : lengths)
        total += len;
    if (total != names.size())
        return false;
    for (uint32_t o : objects)
        if (o != nifly::NIF_NPOS && !hdr.GetBlock<nifly::NiAVObject>(o))
            return false;

    palette->objects.clear();
    size_t offset = 0;
    for (size_t i = 0; i < objects.size(); ++i) {
        nifly::AVObject entry;
        entry.name.get().assign(
            reinterpret_cast<const char*>(names.data()) + offset,
            lengths[i]
        );
        entry.objectRef.index = objects[i];
        palette->objects.push_back(entry);
        offset += lengths[i];
    }
    return true;
}
} // namespace nifly_rs

#undef NIFLY_RS_NODE
