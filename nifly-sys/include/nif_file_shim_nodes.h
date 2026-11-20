// Part of nif_file_shim.h: adding, deleting and cloning nodes, parents, transforms, bounds and LOD data.
#pragma once

#include "nif_file_shim.h"

namespace nifly_rs {
// NifFile::DeleteNode. Returns whether a node named `name` existed. Upstream
// doesn't report that, so the node is looked up first with the same
// FindBlockByName<NiNode> that DeleteNode uses.
inline bool delete_node(nifly::NifFile& file, rust::Slice<const uint8_t> name) {
    std::string c_name(reinterpret_cast<const char*>(name.data()), name.size());
    if (!file.FindBlockByName<nifly::NiNode>(c_name))
        return false;

    file.DeleteNode(c_name);
    return true;
}

// NifFile::CanDeleteNode(const std::string&). False if no node has that name.
inline bool can_delete_node_by_name(
    const nifly::NifFile& file,
    rust::Slice<const uint8_t> name
) {
    std::string c_name(reinterpret_cast<const char*>(name.data()), name.size());
    return file.CanDeleteNode(c_name);
}

// Gives the BSMultiBoundNode at `node_index` a new BSMultiBound holding `data`,
// replacing its ref. Returns the BSMultiBound's index, or NIF_NPOS (changing
// nothing) if that block isn't a BSMultiBoundNode.
template<typename D>
uint32_t assign_multi_bound(
    nifly::NifFile& file,
    uint32_t node_index,
    std::unique_ptr<D> data
) {
    auto& hdr = file.GetHeader();
    auto node = hdr.GetBlock<nifly::BSMultiBoundNode>(node_index);
    if (!node)
        return nifly::NIF_NPOS;
    auto bound = std::make_unique<nifly::BSMultiBound>();
    bound->dataRef.index = hdr.AddBlock(std::move(data));
    uint32_t id = hdr.AddBlock(std::move(bound));
    node->multiBoundRef.index = id;
    return id;
}

inline nifly::Vector3 multi_bound_vec3(const Vec3& v) {
    return nifly::Vector3(v.x, v.y, v.z);
}

inline uint32_t assign_multi_bound_aabb(
    nifly::NifFile& file,
    uint32_t node_index,
    Vec3 center,
    Vec3 half_extent
) {
    auto data = std::make_unique<nifly::BSMultiBoundAABB>();
    data->center = multi_bound_vec3(center);
    data->halfExtent = multi_bound_vec3(half_extent);
    return assign_multi_bound(file, node_index, std::move(data));
}

// `rotation` has 3 rows.
inline uint32_t assign_multi_bound_obb(
    nifly::NifFile& file,
    uint32_t node_index,
    Vec3 center,
    Vec3 size,
    rust::Slice<const Vec3> rotation
) {
    auto data = std::make_unique<nifly::BSMultiBoundOBB>();
    data->center = multi_bound_vec3(center);
    data->size = multi_bound_vec3(size);
    for (int r = 0; r < 3; r++)
        data->rotation[r] = multi_bound_vec3(rotation[r]);
    return assign_multi_bound(file, node_index, std::move(data));
}

inline uint32_t assign_multi_bound_sphere(
    nifly::NifFile& file,
    uint32_t node_index,
    Vec3 center,
    float radius
) {
    auto data = std::make_unique<nifly::BSMultiBoundSphere>();
    data->center = multi_bound_vec3(center);
    data->radius = radius;
    return assign_multi_bound(file, node_index, std::move(data));
}

// Gives the NiLODNode at `node_index` a new default T as its LOD data,
// replacing its ref. Returns the new block's index, or NIF_NPOS (changing
// nothing) if that block isn't an NiLODNode.
template<typename T>
uint32_t assign_lod_data(nifly::NifFile& file, uint32_t node_index) {
    auto& hdr = file.GetHeader();
    auto node = hdr.GetBlock<nifly::NiLODNode>(node_index);
    if (!node)
        return nifly::NIF_NPOS;
    uint32_t id = hdr.AddBlock(std::make_unique<T>());
    node->lodLevelData.index = id;
    return id;
}

inline uint32_t
assign_range_lod_data(nifly::NifFile& file, uint32_t node_index) {
    return assign_lod_data<nifly::NiRangeLODData>(file, node_index);
}

inline uint32_t
assign_screen_lod_data(nifly::NifFile& file, uint32_t node_index) {
    return assign_lod_data<nifly::NiScreenLODData>(file, node_index);
}

// Clears the LOD data ref of the NiLODNode at `node_index`. False if that
// block isn't one or has no LOD data.
inline bool remove_lod_data(nifly::NifFile& file, uint32_t node_index) {
    auto node = file.GetHeader().GetBlock<nifly::NiLODNode>(node_index);
    if (!node || node->lodLevelData.IsEmpty())
        return false;
    node->lodLevelData.Clear();
    return true;
}

// Clears the multi-bound ref of the BSMultiBoundNode at `node_index`. False if
// that block isn't one or has no multi-bound.
inline bool remove_multi_bound(nifly::NifFile& file, uint32_t node_index) {
    auto node = file.GetHeader().GetBlock<nifly::BSMultiBoundNode>(node_index);
    if (!node || node->multiBoundRef.IsEmpty())
        return false;
    node->multiBoundRef.Clear();
    return true;
}

// NifFile::AddNode. Adds a NiNode named `name` with `transform` (to parent)
// as the last child of the node at block `parent_index`, or of the root node
// if `parent_index` is NIF_NPOS. Returns the new node's block index, or
// NIF_NPOS if the parent isn't a node (or there is no root node).
inline uint32_t add_node(
    nifly::NifFile& file,
    rust::Slice<const uint8_t> name,
    Transform transform,
    uint32_t parent_index
) {
    nifly::NiNode* parent = nullptr;
    if (parent_index != nifly::NIF_NPOS) {
        parent = file.GetHeader().GetBlock<nifly::NiNode>(parent_index);
        if (!parent)
            return nifly::NIF_NPOS;
    }

    std::string c_name(reinterpret_cast<const char*>(name.data()), name.size());
    nifly::NiNode* node = file.AddNode(c_name, from_ffi(transform), parent);
    return node ? file.GetBlockID(node) : nifly::NIF_NPOS;
}

// add_node for a node subclass T, as NifFile::AddNode does for NiNode.
template<typename T>
uint32_t add_node_of(
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

    auto node = std::make_unique<T>();
    node->name.get().assign(
        reinterpret_cast<const char*>(name.data()),
        name.size()
    );
    node->SetTransformToParent(from_ffi(transform));
    uint32_t id = file.GetHeader().AddBlock(std::move(node));
    parent->childRefs.AddBlockRef(id);
    return id;
}

#define NIFLY_RS_ADD_NODE(T, snake) \
    inline uint32_t add_##snake( \
        nifly::NifFile& file, \
        rust::Slice<const uint8_t> name, \
        Transform transform, \
        uint32_t parent_index \
    ) { \
        return add_node_of<nifly::T>(file, name, transform, parent_index); \
    }
NIFLY_RS_ADD_NODE(BSFadeNode, bs_fade_node)
NIFLY_RS_ADD_NODE(RootCollisionNode, root_collision_node)
NIFLY_RS_ADD_NODE(AvoidNode, avoid_node)
NIFLY_RS_ADD_NODE(NiBSAnimationNode, ni_bs_animation_node)
NIFLY_RS_ADD_NODE(NiBSParticleNode, ni_bs_particle_node)
NIFLY_RS_ADD_NODE(BSValueNode, bs_value_node)
NIFLY_RS_ADD_NODE(BSLeafAnimNode, bs_leaf_anim_node)
NIFLY_RS_ADD_NODE(BSTreeNode, bs_tree_node)
NIFLY_RS_ADD_NODE(BSOrderedNode, bs_ordered_node)
NIFLY_RS_ADD_NODE(BSMultiBoundNode, bs_multi_bound_node)
NIFLY_RS_ADD_NODE(
    BSDistantObjectInstancedNode,
    bs_distant_object_instanced_node
)
NIFLY_RS_ADD_NODE(BSRangeNode, bs_range_node)
NIFLY_RS_ADD_NODE(BSDebrisNode, bs_debris_node)
NIFLY_RS_ADD_NODE(BSBlastNode, bs_blast_node)
NIFLY_RS_ADD_NODE(BSDamageStage, bs_damage_stage)
NIFLY_RS_ADD_NODE(BSWeakReferenceNode, bs_weak_reference_node)
NIFLY_RS_ADD_NODE(BSFaceGenNiNode, bs_face_gen_ni_node)
NIFLY_RS_ADD_NODE(NiBillboardNode, ni_billboard_node)
NIFLY_RS_ADD_NODE(NiSwitchNode, ni_switch_node)
NIFLY_RS_ADD_NODE(NiLODNode, ni_lod_node)
NIFLY_RS_ADD_NODE(NiBone, ni_bone)
NIFLY_RS_ADD_NODE(NiSortAdjustNode, ni_sort_adjust_node)
NIFLY_RS_ADD_NODE(BSMasterParticleSystem, bs_master_particle_system)
#undef NIFLY_RS_ADD_NODE

// NifFile::SetNodeTransformToParent. Returns whether a node had that name.
inline bool set_node_transform_to_parent(
    nifly::NifFile& file,
    rust::Slice<const uint8_t> name,
    Transform transform,
    bool root_children_only
) {
    std::string c_name(reinterpret_cast<const char*>(name.data()), name.size());
    return file.SetNodeTransformToParent(
        c_name,
        from_ffi(transform),
        root_children_only
    );
}

inline bool node_transform_to_global(
    const nifly::NifFile& file,
    rust::Slice<const uint8_t> name,
    Transform& out
) {
    std::string c_name(reinterpret_cast<const char*>(name.data()), name.size());
    nifly::MatTransform xform;
    if (!file.GetNodeTransformToGlobal(c_name, xform))
        return false;

    out = to_ffi(xform);
    return true;
}

// GetParentNode takes a non-const pointer but doesn't mutate the object.
// NifFile::GetParentNode on the block at `child_index`, as the parent's block
// index; NIF_NPOS if there's no such block or it has no parent.
inline uint32_t
parent_node_index(const nifly::NifFile& file, uint32_t child_index) {
    auto& hdr = file.GetHeader();
    auto child = hdr.GetBlock<nifly::NiObject>(child_index);
    if (!child)
        return nifly::NIF_NPOS;
    auto parent = file.GetParentNode(child);
    return parent ? hdr.GetBlockID(parent) : nifly::NIF_NPOS;
}

// SetParentNode, validated. Upstream adds any block to childRefs (which only
// hold NiAVObjects), creates cycles when the parent is in the child's subtree,
// and silently does nothing for bad pointers. NIF_NPOS for `parent_index`
// means the root node, as a null parent does upstream. Returns false, changing
// nothing, if the child isn't a NiAVObject, the parent isn't a node, they're
// the same block, or the parent is in the child's subtree.
inline bool set_parent_node(
    nifly::NifFile& file,
    uint32_t child_index,
    uint32_t parent_index
) {
    auto& hdr = file.GetHeader();
    auto child = hdr.GetBlock<nifly::NiAVObject>(child_index);
    nifly::NiNode* parent = parent_index == nifly::NIF_NPOS
        ? file.GetRootNode()
        : hdr.GetBlock<nifly::NiNode>(parent_index);
    if (!child || !parent || static_cast<nifly::NiObject*>(child) == parent)
        return false;

    // Walk the child's node subtree through child refs, looking for the parent.
    uint32_t parent_id = file.GetBlockID(parent);
    std::vector<bool> seen(hdr.GetNumBlocks());
    std::vector<uint32_t> stack {child_index};
    while (!stack.empty()) {
        uint32_t id = stack.back();
        stack.pop_back();
        if (id == parent_id)
            return false;
        if (id >= seen.size() || seen[id])
            continue;
        seen[id] = true;
        if (auto node = hdr.GetBlock<nifly::NiNode>(id))
            for (uint32_t i = 0; i < node->childRefs.GetSize(); i++)
                stack.push_back(node->childRefs.GetBlockRef(i));
    }

    file.SetParentNode(child, parent);
    return true;
}

// Replaces BSTreeNode `node_index`'s `bones1` (list 1) or `bones2` (list 2)
// with refs to `bones`. Returns false, changing nothing, if that block isn't a
// BSTreeNode or a bone isn't an NiNode.
inline bool set_tree_node_bones(
    nifly::NifFile& file,
    uint32_t node_index,
    uint8_t list,
    rust::Slice<const uint32_t> bones
) {
    auto& hdr = file.GetHeader();
    auto node = hdr.GetBlock<nifly::BSTreeNode>(node_index);
    if (!node)
        return false;
    for (uint32_t bone : bones)
        if (!hdr.GetBlock<nifly::NiNode>(bone))
            return false;

    auto& refs = list == 1 ? node->bones1 : node->bones2;
    refs.Clear();
    for (uint32_t bone : bones)
        refs.AddBlockRef(bone);
    return true;
}
} // namespace nifly_rs
