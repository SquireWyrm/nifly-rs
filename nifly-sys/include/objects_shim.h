#pragma once
#include <cstdint>

#include "ExtraData.hpp"
#include "Objects.hpp"
#include "rust/cxx.h"
#include "transform.h"

namespace nifly_rs {
// Upcasts. These are implicit derived-to-base conversions, so they always succeed.
inline const nifly::NiObjectNET& av_as_net(const nifly::NiAVObject& obj) {
    return obj;
}

inline const nifly::NiObject& net_as_obj(const nifly::NiObjectNET& obj) {
    return obj;
}

// Downcasts. Return null unless obj is a T or a subclass of it.
inline const nifly::NiObjectNET*
downcast_ni_object_net(const nifly::NiObject& obj) {
    return dynamic_cast<const nifly::NiObjectNET*>(&obj);
}

inline const nifly::NiAVObject*
downcast_ni_av_object(const nifly::NiObject& obj) {
    return dynamic_cast<const nifly::NiAVObject*>(&obj);
}

// GetBlockName is virtual but not const. It only returns a static string, so
// casting away const is safe.
inline rust::Str block_name(const nifly::NiObject& obj) {
    return const_cast<nifly::NiObject&>(obj).GetBlockName();
}

// Names are raw bytes (often Windows-1252, not UTF-8), so they are returned
// as a byte slice and decoded on the Rust side.
inline rust::Slice<const uint8_t> name_bytes(const nifly::NiObjectNET& obj) {
    const std::string& s = obj.name.get();
    return {reinterpret_cast<const uint8_t*>(s.data()), s.size()};
}

inline void
set_object_net_name(nifly::NiObjectNET& obj, rust::Slice<const uint8_t> name) {
    obj.name.get().assign(
        reinterpret_cast<const char*>(name.data()),
        name.size()
    );
}

// The blocks with a `name` field, which is what FindBlockByName<T> compares.
// NiExtraData isn't an NiObjectNET, so both are checked.
inline const nifly::NiStringRef* object_name_ref(const nifly::NiObject& obj) {
    if (auto net = dynamic_cast<const nifly::NiObjectNET*>(&obj))
        return &net->name;
    if (auto extra = dynamic_cast<const nifly::NiExtraData*>(&obj))
        return &extra->name;
    return nullptr;
}

inline bool is_named(const nifly::NiObject& obj) {
    return object_name_ref(obj) != nullptr;
}

inline rust::Slice<const uint8_t> object_name(const nifly::NiObject& obj) {
    const nifly::NiStringRef* ref = object_name_ref(obj);
    if (!ref)
        return {};
    const std::string& s = ref->get();
    return {reinterpret_cast<const uint8_t*>(s.data()), s.size()};
}

inline Transform transform(const nifly::NiAVObject& obj) {
    return to_ffi(obj.GetTransformToParent());
}

// Indices of every block `obj` references (children, properties, controllers,
// extra data, ...). May contain NIF_NPOS for empty refs. GetChildIndices is
// virtual and not const, but it only reads.
inline rust::Vec<uint32_t> child_indices(const nifly::NiObject& obj) {
    std::vector<uint32_t> indices;
    const_cast<nifly::NiObject&>(obj).GetChildIndices(indices);

    rust::Vec<uint32_t> out;
    out.reserve(indices.size());
    for (uint32_t i : indices)
        out.push_back(i);
    return out;
}

inline uint32_t flags(const nifly::NiAVObject& obj) {
    return obj.flags;
}

inline void set_flags(nifly::NiAVObject& obj, uint32_t flags) {
    obj.flags = flags;
}

inline void set_transform(nifly::NiAVObject& obj, Transform t) {
    obj.SetTransformToParent(from_ffi(t));
}

inline Vec3 velocity(const nifly::NiAVObject& obj) {
    return {obj.velocity.x, obj.velocity.y, obj.velocity.z};
}

inline void set_velocity(nifly::NiAVObject& obj, Vec3 v) {
    obj.velocity = nifly::Vector3(v.x, v.y, v.z);
}

// Reaches NiObject's protected groupID through a pointer to member; never
// instantiated.
struct ObjectAccess: nifly::NiObject {
    using NiObject::groupID;
};

inline uint32_t group_id(const nifly::NiObject& obj) {
    return obj.*(&ObjectAccess::groupID);
}

inline void set_group_id(nifly::NiObject& obj, uint32_t id) {
    obj.*(&ObjectAccess::groupID) = id;
}

inline void put_vec3(float* out, const nifly::Vector3& v) {
    out[0] = v.x;
    out[1] = v.y;
    out[2] = v.z;
}

inline nifly::Vector3 get_vec3(const float* in) {
    return nifly::Vector3(in[0], in[1], in[2]);
}

inline void flatten_bounding_volume(
    const nifly::BoundingVolume& bv,
    rust::Vec<BoundingVolumeFfi>& out
) {
    BoundingVolumeFfi r {};
    r.kind = bv.collisionType;
    float* v = r.values.data();
    switch (bv.collisionType) {
        case nifly::SPHERE_BV:
            put_vec3(v, bv.bvSphere.center);
            v[3] = bv.bvSphere.radius;
            break;
        case nifly::BOX_BV:
            put_vec3(v, bv.bvBox.center);
            put_vec3(v + 3, bv.bvBox.axis1);
            put_vec3(v + 6, bv.bvBox.axis2);
            put_vec3(v + 9, bv.bvBox.axis3);
            v[12] = bv.bvBox.extent1;
            v[13] = bv.bvBox.extent2;
            v[14] = bv.bvBox.extent3;
            break;
        case nifly::CAPSULE_BV:
            put_vec3(v, bv.bvCapsule.center);
            put_vec3(v + 3, bv.bvCapsule.origin);
            v[6] = bv.bvCapsule.extent;
            v[7] = bv.bvCapsule.radius;
            break;
        case nifly::HALFSPACE_BV:
            put_vec3(v, bv.bvHalfSpace.plane.normal);
            v[3] = bv.bvHalfSpace.plane.constant;
            put_vec3(v + 4, bv.bvHalfSpace.center);
            break;
        case nifly::UNION_BV:
            r.children =
                static_cast<uint32_t>(bv.bvUnion->boundingVolumes.size());
            break;
        default:
            break;
    }
    out.push_back(r);
    if (bv.collisionType == nifly::UNION_BV)
        for (auto& child : bv.bvUnion->boundingVolumes)
            flatten_bounding_volume(child, out);
}

inline rust::Vec<BoundingVolumeFfi>
bounding_volume(const nifly::NiAVObject& obj) {
    rust::Vec<BoundingVolumeFfi> out;
    if (obj.hasBoundingVolume)
        flatten_bounding_volume(obj.boundingVolume, out);
    return out;
}

// Reads the volume starting at records[pos], advancing pos. False if the
// records run out.
inline bool build_bounding_volume(
    rust::Slice<const BoundingVolumeFfi> records,
    size_t& pos,
    nifly::BoundingVolume& bv
) {
    if (pos >= records.size())
        return false;
    const BoundingVolumeFfi& r = records[pos++];
    const float* v = r.values.data();
    bv.collisionType = static_cast<nifly::BoundVolumeType>(r.kind);
    bv.bvUnion->boundingVolumes.clear();
    switch (bv.collisionType) {
        case nifly::SPHERE_BV:
            bv.bvSphere = nifly::BoundingSphere(get_vec3(v), v[3]);
            break;
        case nifly::BOX_BV:
            bv.bvBox.center = get_vec3(v);
            bv.bvBox.axis1 = get_vec3(v + 3);
            bv.bvBox.axis2 = get_vec3(v + 6);
            bv.bvBox.axis3 = get_vec3(v + 9);
            bv.bvBox.extent1 = v[12];
            bv.bvBox.extent2 = v[13];
            bv.bvBox.extent3 = v[14];
            break;
        case nifly::CAPSULE_BV:
            bv.bvCapsule.center = get_vec3(v);
            bv.bvCapsule.origin = get_vec3(v + 3);
            bv.bvCapsule.extent = v[6];
            bv.bvCapsule.radius = v[7];
            break;
        case nifly::HALFSPACE_BV:
            bv.bvHalfSpace.plane.normal = get_vec3(v);
            bv.bvHalfSpace.plane.constant = v[3];
            bv.bvHalfSpace.center = get_vec3(v + 4);
            break;
        case nifly::UNION_BV:
            if (r.children > records.size() - pos)
                return false;
            bv.bvUnion->boundingVolumes.resize(r.children);
            for (auto& child : bv.bvUnion->boundingVolumes)
                if (!build_bounding_volume(records, pos, child))
                    return false;
            break;
        default:
            break;
    }
    return true;
}

inline bool set_bounding_volume(
    nifly::NiAVObject& obj,
    rust::Slice<const BoundingVolumeFfi> records
) {
    if (records.empty()) {
        obj.hasBoundingVolume = false;
        return true;
    }
    nifly::BoundingVolume bv;
    size_t pos = 0;
    if (!build_bounding_volume(records, pos, bv) || pos != records.size())
        return false;
    obj.boundingVolume = bv;
    obj.hasBoundingVolume = true;
    return true;
}
} // namespace nifly_rs
