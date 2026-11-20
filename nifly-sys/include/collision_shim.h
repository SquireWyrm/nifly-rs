#pragma once
#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <type_traits>

#include "ExtraData.hpp"
#include "NifFile.hpp"
#include "bhk.hpp"
#include "extra_data_shim.h"
#include "nifly-sys/src/collision.rs.h"
#include "nifly-sys/src/object_3d.rs.h"
#include "objects_shim.h"
#include "rust/cxx.h"

// For each collision type T with direct base Parent, defines:
// - upcast_<snake>: T -> Parent. Implicit derived-to-base conversion, always succeeds.
// - downcast_<snake>: NiObject -> T. Returns null unless obj is a T or a subclass of T.
#define NIFLY_RS_COLLISION(T, Parent, snake) \
    inline const nifly::Parent& upcast_##snake(const nifly::T& obj) { \
        return obj; \
    } \
    inline const nifly::T* downcast_##snake(const nifly::NiObject& obj) { \
        return dynamic_cast<const nifly::T*>(&obj); \
    }

// A getter and setter for a plain field.
#define NIFLY_RS_COLLISION_FIELD(T, type, field, snake) \
    inline type snake(const nifly::T& obj) { \
        return obj.field; \
    } \
    inline void set_##snake(nifly::T& obj, type value) { \
        obj.field = value; \
    }

namespace nifly_rs {
NIFLY_RS_COLLISION(NiCollisionObject, NiObject, ni_collision_object)
NIFLY_RS_COLLISION(NiCollisionData, NiCollisionObject, ni_collision_data)
NIFLY_RS_COLLISION(
    bhkNiCollisionObject,
    NiCollisionObject,
    bhk_ni_collision_object
)
NIFLY_RS_COLLISION(
    bhkCollisionObject,
    bhkNiCollisionObject,
    bhk_collision_object
)
NIFLY_RS_COLLISION(
    bhkNPCollisionObject,
    bhkCollisionObject,
    bhk_np_collision_object
)
NIFLY_RS_COLLISION(
    bhkPCollisionObject,
    bhkNiCollisionObject,
    bhk_p_collision_object
)
NIFLY_RS_COLLISION(
    bhkSPCollisionObject,
    bhkPCollisionObject,
    bhk_sp_collision_object
)
NIFLY_RS_COLLISION(
    bhkBlendCollisionObject,
    bhkCollisionObject,
    bhk_blend_collision_object
)
NIFLY_RS_COLLISION(bhkPhysicsSystem, BSExtraData, bhk_physics_system)
NIFLY_RS_COLLISION(bhkRagdollSystem, BSExtraData, bhk_ragdoll_system)
NIFLY_RS_COLLISION(bhkRefObject, NiObject, bhk_ref_object)
NIFLY_RS_COLLISION(bhkSerializable, bhkRefObject, bhk_serializable)
NIFLY_RS_COLLISION(bhkShape, bhkSerializable, bhk_shape)
NIFLY_RS_COLLISION(bhkHeightFieldShape, bhkShape, bhk_height_field_shape)
NIFLY_RS_COLLISION(bhkPlaneShape, bhkHeightFieldShape, bhk_plane_shape)
NIFLY_RS_COLLISION(bhkSphereRepShape, bhkShape, bhk_sphere_rep_shape)
NIFLY_RS_COLLISION(bhkConvexShape, bhkSphereRepShape, bhk_convex_shape)
NIFLY_RS_COLLISION(
    bhkMultiSphereShape,
    bhkSphereRepShape,
    bhk_multi_sphere_shape
)
NIFLY_RS_COLLISION(bhkConvexListShape, bhkShape, bhk_convex_list_shape)
NIFLY_RS_COLLISION(
    bhkConvexVerticesShape,
    bhkConvexShape,
    bhk_convex_vertices_shape
)
NIFLY_RS_COLLISION(bhkBoxShape, bhkConvexShape, bhk_box_shape)
NIFLY_RS_COLLISION(bhkSphereShape, bhkConvexShape, bhk_sphere_shape)
NIFLY_RS_COLLISION(bhkCylinderShape, bhkConvexShape, bhk_cylinder_shape)
NIFLY_RS_COLLISION(bhkTransformShape, bhkShape, bhk_transform_shape)
NIFLY_RS_COLLISION(
    bhkConvexTransformShape,
    bhkTransformShape,
    bhk_convex_transform_shape
)
NIFLY_RS_COLLISION(bhkCapsuleShape, bhkConvexShape, bhk_capsule_shape)
NIFLY_RS_COLLISION(bhkBvTreeShape, bhkShape, bhk_bv_tree_shape)
NIFLY_RS_COLLISION(bhkMoppBvTreeShape, bhkBvTreeShape, bhk_mopp_bv_tree_shape)
NIFLY_RS_COLLISION(bhkNiTriStripsShape, bhkShape, bhk_ni_tri_strips_shape)
NIFLY_RS_COLLISION(bhkShapeCollection, bhkShape, bhk_shape_collection)
NIFLY_RS_COLLISION(bhkListShape, bhkShapeCollection, bhk_list_shape)
NIFLY_RS_COLLISION(
    hkPackedNiTriStripsData,
    bhkShapeCollection,
    hk_packed_ni_tri_strips_data
)
NIFLY_RS_COLLISION(
    bhkPackedNiTriStripsShape,
    bhkShapeCollection,
    bhk_packed_ni_tri_strips_shape
)
NIFLY_RS_COLLISION(bhkLiquidAction, bhkSerializable, bhk_liquid_action)
NIFLY_RS_COLLISION(
    bhkOrientHingedBodyAction,
    bhkSerializable,
    bhk_orient_hinged_body_action
)
NIFLY_RS_COLLISION(bhkWorldObject, bhkSerializable, bhk_world_object)
NIFLY_RS_COLLISION(bhkPhantom, bhkWorldObject, bhk_phantom)
NIFLY_RS_COLLISION(bhkShapePhantom, bhkPhantom, bhk_shape_phantom)
NIFLY_RS_COLLISION(
    bhkSimpleShapePhantom,
    bhkShapePhantom,
    bhk_simple_shape_phantom
)
NIFLY_RS_COLLISION(bhkAabbPhantom, bhkShapePhantom, bhk_aabb_phantom)
NIFLY_RS_COLLISION(bhkEntity, bhkWorldObject, bhk_entity)
NIFLY_RS_COLLISION(bhkRigidBody, bhkEntity, bhk_rigid_body)
NIFLY_RS_COLLISION(bhkRigidBodyT, bhkRigidBody, bhk_rigid_body_t)
NIFLY_RS_COLLISION(bhkConstraint, bhkSerializable, bhk_constraint)
NIFLY_RS_COLLISION(bhkHingeConstraint, bhkConstraint, bhk_hinge_constraint)
NIFLY_RS_COLLISION(
    bhkLimitedHingeConstraint,
    bhkConstraint,
    bhk_limited_hinge_constraint
)
NIFLY_RS_COLLISION(
    bhkBreakableConstraint,
    bhkConstraint,
    bhk_breakable_constraint
)
NIFLY_RS_COLLISION(bhkRagdollConstraint, bhkConstraint, bhk_ragdoll_constraint)
NIFLY_RS_COLLISION(
    bhkStiffSpringConstraint,
    bhkConstraint,
    bhk_stiff_spring_constraint
)
NIFLY_RS_COLLISION(
    bhkPrismaticConstraint,
    bhkConstraint,
    bhk_prismatic_constraint
)
NIFLY_RS_COLLISION(
    bhkMalleableConstraint,
    bhkConstraint,
    bhk_malleable_constraint
)
NIFLY_RS_COLLISION(
    bhkBallAndSocketConstraint,
    bhkConstraint,
    bhk_ball_and_socket_constraint
)
NIFLY_RS_COLLISION(
    bhkBallSocketConstraintChain,
    bhkSerializable,
    bhk_ball_socket_constraint_chain
)
NIFLY_RS_COLLISION(
    bhkCompressedMeshShapeData,
    bhkRefObject,
    bhk_compressed_mesh_shape_data
)
NIFLY_RS_COLLISION(bhkCompressedMeshShape, bhkShape, bhk_compressed_mesh_shape)
NIFLY_RS_COLLISION(bhkPoseArray, NiObject, bhk_pose_array)
NIFLY_RS_COLLISION(bhkRagdollTemplate, NiExtraData, bhk_ragdoll_template)
NIFLY_RS_COLLISION(bhkRagdollTemplateData, NiObject, bhk_ragdoll_template_data)

// NiCollisionData. The enums are passed as their raw values.
inline uint32_t
collision_data_propagation_mode(const nifly::NiCollisionData& d) {
    return d.propagationMode;
}

inline void
set_collision_data_propagation_mode(nifly::NiCollisionData& d, uint32_t v) {
    d.propagationMode = static_cast<nifly::PropagationMode>(v);
}

inline uint32_t collision_data_collision_mode(const nifly::NiCollisionData& d) {
    return d.collisionMode;
}

inline void
set_collision_data_collision_mode(nifly::NiCollisionData& d, uint32_t v) {
    d.collisionMode = static_cast<nifly::CollisionMode>(v);
}

// Empty unless useABV is set.
inline rust::Vec<BoundingVolumeFfi>
collision_data_bounding_volume(const nifly::NiCollisionData& d) {
    rust::Vec<BoundingVolumeFfi> out;
    if (d.useABV)
        flatten_bounding_volume(d.boundingVolume, out);
    return out;
}

// Clears useABV for no records. False, changing nothing, unless the records
// are exactly one volume.
inline bool set_collision_data_bounding_volume(
    nifly::NiCollisionData& d,
    rust::Slice<const BoundingVolumeFfi> records
) {
    if (records.empty()) {
        d.useABV = false;
        return true;
    }
    nifly::BoundingVolume bv;
    size_t pos = 0;
    if (!build_bounding_volume(records, pos, bv) || pos != records.size())
        return false;
    d.boundingVolume = bv;
    d.useABV = true;
    return true;
}

NIFLY_RS_COLLISION_FIELD(
    bhkNiCollisionObject,
    uint16_t,
    flags,
    bhk_collision_flags
)
NIFLY_RS_COLLISION_FIELD(
    bhkNPCollisionObject,
    uint32_t,
    bodyID,
    np_collision_body_id
)
NIFLY_RS_COLLISION_FIELD(
    bhkBlendCollisionObject,
    float,
    heirGain,
    blend_collision_heir_gain
)
NIFLY_RS_COLLISION_FIELD(
    bhkBlendCollisionObject,
    float,
    velGain,
    blend_collision_vel_gain
)

template<class T>
rust::Slice<const uint8_t> system_bytes(const T& s) {
    return {reinterpret_cast<const uint8_t*>(s.data.data()), s.data.size()};
}

template<class T>
void set_system_bytes(T& s, rust::Slice<const uint8_t> bytes) {
    s.data.resize(static_cast<uint32_t>(bytes.size()));
    std::copy(
        bytes.begin(),
        bytes.end(),
        reinterpret_cast<uint8_t*>(s.data.data())
    );
}

inline rust::Slice<const uint8_t>
physics_system_data(const nifly::bhkPhysicsSystem& s) {
    return system_bytes(s);
}

inline void set_physics_system_data(
    nifly::bhkPhysicsSystem& s,
    rust::Slice<const uint8_t> bytes
) {
    set_system_bytes(s, bytes);
}

inline rust::Slice<const uint8_t>
ragdoll_system_data(const nifly::bhkRagdollSystem& s) {
    return system_bytes(s);
}

inline void set_ragdoll_system_data(
    nifly::bhkRagdollSystem& s,
    rust::Slice<const uint8_t> bytes
) {
    set_system_bytes(s, bytes);
}

// Adds `collision` as the collision object of the NiAVObject at `object`,
// targeting it, and returns its index. The old collision object stays in the
// file. NIF_NPOS, changing nothing, if `object` isn't an NiAVObject.
inline uint32_t assign_collision(
    nifly::NifFile& file,
    uint32_t object,
    std::unique_ptr<nifly::NiCollisionObject> collision
) {
    auto& hdr = file.GetHeader();
    auto av = hdr.GetBlock<nifly::NiAVObject>(object);
    if (!av)
        return nifly::NIF_NPOS;
    collision->targetRef.index = object;
    uint32_t id = hdr.AddBlock(std::move(collision));
    av->collisionRef.index = id;
    return id;
}

#define NIFLY_RS_ASSIGN_COLLISION(T, snake) \
    inline uint32_t assign_##snake(nifly::NifFile& file, uint32_t object) { \
        return assign_collision(file, object, std::make_unique<nifly::T>()); \
    }

NIFLY_RS_ASSIGN_COLLISION(NiCollisionData, ni_collision_data)
NIFLY_RS_ASSIGN_COLLISION(bhkCollisionObject, bhk_collision_object)
NIFLY_RS_ASSIGN_COLLISION(bhkNPCollisionObject, bhk_np_collision_object)
NIFLY_RS_ASSIGN_COLLISION(bhkPCollisionObject, bhk_p_collision_object)
NIFLY_RS_ASSIGN_COLLISION(bhkSPCollisionObject, bhk_sp_collision_object)
NIFLY_RS_ASSIGN_COLLISION(bhkBlendCollisionObject, bhk_blend_collision_object)

#undef NIFLY_RS_ASSIGN_COLLISION

// Adds a new B as the body of the bhkNiCollisionObject at `collision` and
// returns its index. A bhkNPCollisionObject takes a physics or ragdoll system,
// other collision objects a bhkWorldObject. NIF_NPOS, changing nothing,
// otherwise. The old body stays in the file.
template<class B>
uint32_t assign_body(nifly::NifFile& file, uint32_t collision) {
    auto& hdr = file.GetHeader();
    auto c = hdr.GetBlock<nifly::bhkNiCollisionObject>(collision);
    if (!c)
        return nifly::NIF_NPOS;
    bool np = dynamic_cast<nifly::bhkNPCollisionObject*>(c) != nullptr;
    constexpr bool system = std::is_base_of_v<nifly::bhkPhysicsSystem, B>
        || std::is_base_of_v<nifly::bhkRagdollSystem, B>;
    if (np != system)
        return nifly::NIF_NPOS;
    uint32_t id = hdr.AddBlock(std::make_unique<B>());
    c->bodyRef.index = id;
    return id;
}

#define NIFLY_RS_ASSIGN_BODY(T, snake) \
    inline uint32_t assign_##snake(nifly::NifFile& file, uint32_t collision) { \
        return assign_body<nifly::T>(file, collision); \
    }

NIFLY_RS_ASSIGN_BODY(bhkRigidBody, bhk_rigid_body)
NIFLY_RS_ASSIGN_BODY(bhkRigidBodyT, bhk_rigid_body_t)
NIFLY_RS_ASSIGN_BODY(bhkSimpleShapePhantom, bhk_simple_shape_phantom)
NIFLY_RS_ASSIGN_BODY(bhkAabbPhantom, bhk_aabb_phantom)
NIFLY_RS_ASSIGN_BODY(bhkPhysicsSystem, bhk_physics_system)
NIFLY_RS_ASSIGN_BODY(bhkRagdollSystem, bhk_ragdoll_system)

#undef NIFLY_RS_ASSIGN_BODY

// Conversions for Havok values.
inline std::array<float, 4> vec4_to_ffi(const nifly::Vector4& v) {
    return {v.x, v.y, v.z, v.w};
}

inline nifly::Vector4 vec4_from_ffi(std::array<float, 4> v) {
    return nifly::Vector4(v[0], v[1], v[2], v[3]);
}

inline std::array<float, 4> quat_to_ffi(const nifly::QuaternionXYZW& q) {
    return {q.x, q.y, q.z, q.w};
}

inline nifly::QuaternionXYZW quat_from_ffi(std::array<float, 4> v) {
    nifly::QuaternionXYZW q;
    q.x = v[0];
    q.y = v[1];
    q.z = v[2];
    q.w = v[3];
    return q;
}

inline HavokFilterFfi filter_to_ffi(const nifly::HavokFilter& f) {
    return {f.layer, f.flagsAndParts, f.group};
}

inline nifly::HavokFilter filter_from_ffi(HavokFilterFfi f) {
    nifly::HavokFilter out;
    out.layer = f.layer;
    out.flagsAndParts = f.flags_and_parts;
    out.group = f.group;
    return out;
}

inline CInfoPropertyFfi
property_to_ffi(const nifly::hkWorldObjCInfoProperty& p) {
    return {p.data, p.size, p.capacityAndFlags};
}

inline nifly::hkWorldObjCInfoProperty property_from_ffi(CInfoPropertyFfi p) {
    nifly::hkWorldObjCInfoProperty out;
    out.data = p.data;
    out.size = p.size;
    out.capacityAndFlags = p.capacity_and_flags;
    return out;
}

inline std::array<float, 16> matrix4_to_ffi(const nifly::Matrix4& m) {
    std::array<float, 16> out;
    for (int i = 0; i < 16; i++)
        out[i] = m[i];
    return out;
}

inline nifly::Matrix4 matrix4_from_ffi(std::array<float, 16> v) {
    nifly::Matrix4 m;
    for (int i = 0; i < 16; i++)
        m[i] = v[i];
    return m;
}

// A getter and setter for `field`, converting with `to` and `from`.
#define NIFLY_RS_COLLISION_CONVERT(T, Ffi, field, snake, to, from) \
    inline Ffi snake(const nifly::T& obj) { \
        return to(obj.field); \
    } \
    inline void set_##snake(nifly::T& obj, Ffi value) { \
        obj.field = from(value); \
    }
#define NIFLY_RS_COLLISION_VEC4(T, field, snake) \
    NIFLY_RS_COLLISION_CONVERT( \
        T, \
        std::array<float NIFLY_RS_COMMA 4>, \
        field, \
        snake, \
        vec4_to_ffi, \
        vec4_from_ffi \
    )
#define NIFLY_RS_COMMA ,

// A getter and setter for a C array field of N elements of type E.
#define NIFLY_RS_COLLISION_ARRAY(T, E, N, field, snake) \
    inline std::array<E, N> snake(const nifly::T& obj) { \
        std::array<E, N> out; \
        std::copy(std::begin(obj.field), std::end(obj.field), out.begin()); \
        return out; \
    } \
    inline void set_##snake(nifly::T& obj, std::array<E, N> value) { \
        std::copy(value.begin(), value.end(), std::begin(obj.field)); \
    }

// Shapes.
inline Vec3 havok_vec3_to_ffi(const nifly::Vector3& v) {
    return {v.x, v.y, v.z};
}

inline nifly::Vector3 havok_vec3_from_ffi(Vec3 v) {
    return nifly::Vector3(v.x, v.y, v.z);
}

#define NIFLY_RS_COLLISION_VEC3(T, field, snake) \
    NIFLY_RS_COLLISION_CONVERT( \
        T, \
        Vec3, \
        field, \
        snake, \
        havok_vec3_to_ffi, \
        havok_vec3_from_ffi \
    )

// Const iteration over an NiVector, which only iterates mutably.
template<class T>
struct ConstItems {
    const T* first;
    const T* last;

    const T* begin() const {
        return first;
    }

    const T* end() const {
        return last;
    }
};

template<class V>
auto const_items(const V& list) {
    using T = std::remove_cv_t<std::remove_pointer_t<decltype(list.data())>>;
    return ConstItems<T> {list.data(), list.data() + list.size()};
}

// Lists of Vector4s, as 4 floats each.
template<class V>
rust::Slice<const float> vec4_list(const V& list) {
    return {
        reinterpret_cast<const float*>(list.data()),
        size_t(list.size()) * 4
    };
}

// False, changing nothing, unless there are 4 floats per entry.
template<class V>
bool set_vec4_list(V& list, rust::Slice<const float> values) {
    if (values.size() % 4 != 0)
        return false;
    list.resize(static_cast<uint32_t>(values.size() / 4));
    for (size_t i = 0; i < values.size() / 4; i++)
        list[i] = nifly::Vector4(
            values[i * 4],
            values[i * 4 + 1],
            values[i * 4 + 2],
            values[i * 4 + 3]
        );
    return true;
}
} // namespace nifly_rs

// The rest of the shims, by topic.
// In this order: later parts use earlier ones.
// clang-format off
#include "collision_shim_bodies.h"
#include "collision_shim_shapes.h"
#include "collision_shim_mesh_shapes.h"
#include "collision_shim_constraints.h"
// clang-format on

#undef NIFLY_RS_COLLISION_ARRAY
#undef NIFLY_RS_COMMA
#undef NIFLY_RS_COLLISION_VEC4
#undef NIFLY_RS_COLLISION_CONVERT

#undef NIFLY_RS_COLLISION_FIELD
#undef NIFLY_RS_COLLISION
