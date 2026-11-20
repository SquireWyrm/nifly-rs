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

// bhkWorldObject.
NIFLY_RS_COLLISION_CONVERT(
    bhkWorldObject,
    HavokFilterFfi,
    collisionFilter,
    world_object_filter,
    filter_to_ffi,
    filter_from_ffi
)
NIFLY_RS_COLLISION_FIELD(
    bhkWorldObject,
    int32_t,
    unkInt1,
    world_object_unknown_int
)
NIFLY_RS_COLLISION_FIELD(
    bhkWorldObject,
    uint8_t,
    broadPhaseType,
    world_object_broad_phase_type
)
NIFLY_RS_COLLISION_ARRAY(
    bhkWorldObject,
    uint8_t,
    3,
    unkBytes,
    world_object_unknown_bytes
)
NIFLY_RS_COLLISION_CONVERT(
    bhkWorldObject,
    CInfoPropertyFfi,
    prop,
    world_object_property,
    property_to_ffi,
    property_from_ffi
)

// bhkRigidBody.
inline uint8_t rigid_body_collision_response(const nifly::bhkRigidBody& b) {
    return b.collisionResponse;
}

inline void
set_rigid_body_collision_response(nifly::bhkRigidBody& b, uint8_t v) {
    b.collisionResponse = static_cast<nifly::hkResponseType>(v);
}

NIFLY_RS_COLLISION_FIELD(
    bhkRigidBody,
    uint16_t,
    processContactCallbackDelay,
    rigid_body_callback_delay
)
NIFLY_RS_COLLISION_FIELD(
    bhkRigidBody,
    uint32_t,
    unkInt1,
    rigid_body_unknown_int
)
NIFLY_RS_COLLISION_CONVERT(
    bhkRigidBody,
    HavokFilterFfi,
    collisionFilterCopy,
    rigid_body_filter_copy,
    filter_to_ffi,
    filter_from_ffi
)
NIFLY_RS_COLLISION_ARRAY(
    bhkRigidBody,
    uint16_t,
    6,
    unkShorts2,
    rigid_body_unknown_shorts
)
NIFLY_RS_COLLISION_VEC4(bhkRigidBody, translation, rigid_body_translation)
NIFLY_RS_COLLISION_CONVERT(
    bhkRigidBody,
    std::array<float NIFLY_RS_COMMA 4>,
    rotation,
    rigid_body_rotation,
    quat_to_ffi,
    quat_from_ffi
)
NIFLY_RS_COLLISION_VEC4(
    bhkRigidBody,
    linearVelocity,
    rigid_body_linear_velocity
)
NIFLY_RS_COLLISION_VEC4(
    bhkRigidBody,
    angularVelocity,
    rigid_body_angular_velocity
)
NIFLY_RS_COLLISION_ARRAY(
    bhkRigidBody,
    float,
    12,
    inertiaMatrix,
    rigid_body_inertia
)
NIFLY_RS_COLLISION_VEC4(bhkRigidBody, center, rigid_body_center)
NIFLY_RS_COLLISION_FIELD(bhkRigidBody, float, mass, rigid_body_mass)
NIFLY_RS_COLLISION_FIELD(
    bhkRigidBody,
    float,
    linearDamping,
    rigid_body_linear_damping
)
NIFLY_RS_COLLISION_FIELD(
    bhkRigidBody,
    float,
    angularDamping,
    rigid_body_angular_damping
)
NIFLY_RS_COLLISION_FIELD(
    bhkRigidBody,
    float,
    timeFactor,
    rigid_body_time_factor
)
NIFLY_RS_COLLISION_FIELD(
    bhkRigidBody,
    float,
    gravityFactor,
    rigid_body_gravity_factor
)
NIFLY_RS_COLLISION_FIELD(bhkRigidBody, float, friction, rigid_body_friction)
NIFLY_RS_COLLISION_FIELD(
    bhkRigidBody,
    float,
    rollingFrictionMult,
    rigid_body_rolling_friction
)
NIFLY_RS_COLLISION_FIELD(
    bhkRigidBody,
    float,
    restitution,
    rigid_body_restitution
)
NIFLY_RS_COLLISION_FIELD(
    bhkRigidBody,
    float,
    maxLinearVelocity,
    rigid_body_max_linear_velocity
)
NIFLY_RS_COLLISION_FIELD(
    bhkRigidBody,
    float,
    maxAngularVelocity,
    rigid_body_max_angular_velocity
)
NIFLY_RS_COLLISION_FIELD(
    bhkRigidBody,
    float,
    penetrationDepth,
    rigid_body_penetration_depth
)
NIFLY_RS_COLLISION_FIELD(
    bhkRigidBody,
    uint8_t,
    motionSystem,
    rigid_body_motion_system
)
NIFLY_RS_COLLISION_FIELD(
    bhkRigidBody,
    uint8_t,
    deactivatorType,
    rigid_body_deactivator_type
)
NIFLY_RS_COLLISION_FIELD(
    bhkRigidBody,
    uint8_t,
    solverDeactivation,
    rigid_body_solver_deactivation
)
NIFLY_RS_COLLISION_FIELD(
    bhkRigidBody,
    uint8_t,
    qualityType,
    rigid_body_quality_type
)
NIFLY_RS_COLLISION_FIELD(
    bhkRigidBody,
    uint8_t,
    autoRemoveLevel,
    rigid_body_auto_remove_level
)
NIFLY_RS_COLLISION_FIELD(
    bhkRigidBody,
    uint8_t,
    responseModifierFlag,
    rigid_body_response_modifier_flags
)
NIFLY_RS_COLLISION_FIELD(
    bhkRigidBody,
    uint8_t,
    numShapeKeysInContactPointProps,
    rigid_body_shape_keys
)
NIFLY_RS_COLLISION_FIELD(
    bhkRigidBody,
    bool,
    forceCollideOntoPpu,
    rigid_body_force_collide_onto_ppu
)
NIFLY_RS_COLLISION_FIELD(
    bhkRigidBody,
    uint32_t,
    bodyFlagsInt,
    rigid_body_body_flags_int
)
NIFLY_RS_COLLISION_FIELD(
    bhkRigidBody,
    uint16_t,
    bodyFlags,
    rigid_body_body_flags
)

// Phantoms.
NIFLY_RS_COLLISION_CONVERT(
    bhkSimpleShapePhantom,
    std::array<float NIFLY_RS_COMMA 16>,
    transform,
    simple_phantom_transform,
    matrix4_to_ffi,
    matrix4_from_ffi
)
NIFLY_RS_COLLISION_VEC4(bhkAabbPhantom, aabbMin, aabb_phantom_min)
NIFLY_RS_COLLISION_VEC4(bhkAabbPhantom, aabbMax, aabb_phantom_max)

// Actions.
NIFLY_RS_COLLISION_FIELD(bhkLiquidAction, uint32_t, userData, liquid_user_data)
NIFLY_RS_COLLISION_FIELD(
    bhkLiquidAction,
    uint32_t,
    unkInt1,
    liquid_unknown_int_1
)
NIFLY_RS_COLLISION_FIELD(
    bhkLiquidAction,
    uint32_t,
    unkInt2,
    liquid_unknown_int_2
)
NIFLY_RS_COLLISION_FIELD(
    bhkLiquidAction,
    float,
    initialStickForce,
    liquid_initial_stick_force
)
NIFLY_RS_COLLISION_FIELD(
    bhkLiquidAction,
    float,
    stickStrength,
    liquid_stick_strength
)
NIFLY_RS_COLLISION_FIELD(
    bhkLiquidAction,
    float,
    neighborDistance,
    liquid_neighbor_distance
)
NIFLY_RS_COLLISION_FIELD(
    bhkLiquidAction,
    float,
    neighborStrength,
    liquid_neighbor_strength
)
NIFLY_RS_COLLISION_FIELD(
    bhkOrientHingedBodyAction,
    uint32_t,
    unkInt1,
    hinged_action_unknown_int_1
)
NIFLY_RS_COLLISION_FIELD(
    bhkOrientHingedBodyAction,
    uint32_t,
    unkInt2,
    hinged_action_unknown_int_2
)
NIFLY_RS_COLLISION_VEC4(
    bhkOrientHingedBodyAction,
    hingeAxisLS,
    hinged_action_hinge_axis
)
NIFLY_RS_COLLISION_VEC4(
    bhkOrientHingedBodyAction,
    forwardLS,
    hinged_action_forward
)
NIFLY_RS_COLLISION_FIELD(
    bhkOrientHingedBodyAction,
    float,
    strength,
    hinged_action_strength
)
NIFLY_RS_COLLISION_FIELD(
    bhkOrientHingedBodyAction,
    float,
    damping,
    hinged_action_damping
)

// Adds a new, unattached action and returns its index.
inline uint32_t add_bhk_liquid_action(nifly::NifFile& file) {
    return file.GetHeader().AddBlock(
        std::make_unique<nifly::bhkLiquidAction>()
    );
}

inline uint32_t add_bhk_orient_hinged_body_action(nifly::NifFile& file) {
    return file.GetHeader().AddBlock(
        std::make_unique<nifly::bhkOrientHingedBodyAction>()
    );
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

// Materials stored behind the virtual accessors.
#define NIFLY_RS_SHAPE_MATERIAL(T, snake) \
    inline uint32_t snake(const nifly::T& s) { \
        return s.GetMaterial(); \
    } \
    inline void set_##snake(nifly::T& s, uint32_t v) { \
        s.SetMaterial(v); \
    }

NIFLY_RS_SHAPE_MATERIAL(bhkHeightFieldShape, height_field_material)
NIFLY_RS_SHAPE_MATERIAL(bhkSphereRepShape, sphere_rep_material)
NIFLY_RS_SHAPE_MATERIAL(bhkNiTriStripsShape, strips_shape_material)
NIFLY_RS_SHAPE_MATERIAL(bhkListShape, list_shape_material)

#undef NIFLY_RS_SHAPE_MATERIAL

// Primitives.
NIFLY_RS_COLLISION_VEC3(bhkPlaneShape, unkVec, plane_shape_unknown_vector)
NIFLY_RS_COLLISION_VEC3(bhkPlaneShape, plane.normal, plane_shape_normal)
NIFLY_RS_COLLISION_FIELD(
    bhkPlaneShape,
    float,
    plane.constant,
    plane_shape_constant
)
NIFLY_RS_COLLISION_VEC4(bhkPlaneShape, halfExtents, plane_shape_half_extents)
NIFLY_RS_COLLISION_VEC4(bhkPlaneShape, center, plane_shape_center)
NIFLY_RS_COLLISION_FIELD(bhkConvexShape, float, radius, convex_shape_radius)
NIFLY_RS_COLLISION_CONVERT(
    bhkMultiSphereShape,
    CInfoPropertyFfi,
    shapeProperty,
    multi_sphere_property,
    property_to_ffi,
    property_from_ffi
)

inline rust::Vec<BoundingSphere>
multi_sphere_spheres(const nifly::bhkMultiSphereShape& s) {
    rust::Vec<BoundingSphere> out;
    for (auto& sphere : const_items(s.spheres))
        out.push_back({havok_vec3_to_ffi(sphere.center), sphere.radius});
    return out;
}

inline void set_multi_sphere_spheres(
    nifly::bhkMultiSphereShape& s,
    rust::Slice<const BoundingSphere> spheres
) {
    s.spheres.clear();
    for (auto& sphere : spheres) {
        nifly::BoundingSphere b;
        b.center = havok_vec3_from_ffi(sphere.center);
        b.radius = sphere.radius;
        s.spheres.push_back(b);
    }
}

NIFLY_RS_COLLISION_CONVERT(
    bhkConvexVerticesShape,
    CInfoPropertyFfi,
    vertsProp,
    convex_vertices_vertices_property,
    property_to_ffi,
    property_from_ffi
)
NIFLY_RS_COLLISION_CONVERT(
    bhkConvexVerticesShape,
    CInfoPropertyFfi,
    normalsProp,
    convex_vertices_normals_property,
    property_to_ffi,
    property_from_ffi
)

// Vertices (`normals` false) or normals as 4 floats each.
inline rust::Slice<const float>
convex_vertices_list(const nifly::bhkConvexVerticesShape& s, bool normals) {
    return vec4_list(normals ? s.normals : s.verts);
}

inline bool set_convex_vertices_list(
    nifly::bhkConvexVerticesShape& s,
    bool normals,
    rust::Slice<const float> values
) {
    return set_vec4_list(normals ? s.normals : s.verts, values);
}

NIFLY_RS_COLLISION_VEC3(bhkBoxShape, dimensions, box_shape_dimensions)
NIFLY_RS_COLLISION_VEC4(bhkCylinderShape, vertexA, cylinder_shape_vertex_a)
NIFLY_RS_COLLISION_VEC4(bhkCylinderShape, vertexB, cylinder_shape_vertex_b)
NIFLY_RS_COLLISION_FIELD(
    bhkCylinderShape,
    float,
    cylinderRadius,
    cylinder_shape_radius
)
NIFLY_RS_COLLISION_VEC3(bhkCapsuleShape, point1, capsule_shape_point_1)
NIFLY_RS_COLLISION_FIELD(
    bhkCapsuleShape,
    float,
    radius1,
    capsule_shape_radius_1
)
NIFLY_RS_COLLISION_VEC3(bhkCapsuleShape, point2, capsule_shape_point_2)
NIFLY_RS_COLLISION_FIELD(
    bhkCapsuleShape,
    float,
    radius2,
    capsule_shape_radius_2
)

// Containers.
NIFLY_RS_COLLISION_FIELD(
    bhkTransformShape,
    uint32_t,
    material,
    transform_shape_material
)
NIFLY_RS_COLLISION_FIELD(
    bhkTransformShape,
    float,
    radius,
    transform_shape_radius
)
NIFLY_RS_COLLISION_CONVERT(
    bhkTransformShape,
    std::array<float NIFLY_RS_COMMA 16>,
    xform,
    transform_shape_transform,
    matrix4_to_ffi,
    matrix4_from_ffi
)
NIFLY_RS_COLLISION_CONVERT(
    bhkListShape,
    CInfoPropertyFfi,
    childShapeProp,
    list_shape_child_shape_property,
    property_to_ffi,
    property_from_ffi
)
NIFLY_RS_COLLISION_CONVERT(
    bhkListShape,
    CInfoPropertyFfi,
    childFilterProp,
    list_shape_child_filter_property,
    property_to_ffi,
    property_from_ffi
)

inline rust::Vec<HavokFilterFfi>
list_shape_filters(const nifly::bhkListShape& s) {
    rust::Vec<HavokFilterFfi> out;
    for (auto& f : const_items(s.filters))
        out.push_back(filter_to_ffi(f));
    return out;
}

// False, changing nothing, unless there's one filter per sub-shape.
inline bool set_list_shape_filters(
    nifly::bhkListShape& s,
    rust::Slice<const HavokFilterFfi> filters
) {
    if (filters.size() != s.subShapeRefs.GetSize())
        return false;
    s.filters.clear();
    for (auto f : filters) {
        auto filter = filter_from_ffi(f);
        s.filters.push_back(filter);
    }
    return true;
}

NIFLY_RS_COLLISION_FIELD(
    bhkConvexListShape,
    uint32_t,
    material,
    convex_list_material
)
NIFLY_RS_COLLISION_FIELD(bhkConvexListShape, float, radius, convex_list_radius)
NIFLY_RS_COLLISION_FIELD(
    bhkConvexListShape,
    uint32_t,
    unkInt1,
    convex_list_unknown_int
)
NIFLY_RS_COLLISION_FIELD(
    bhkConvexListShape,
    float,
    unkFloat1,
    convex_list_unknown_float
)
NIFLY_RS_COLLISION_CONVERT(
    bhkConvexListShape,
    CInfoPropertyFfi,
    childShapeProp,
    convex_list_child_shape_property,
    property_to_ffi,
    property_from_ffi
)
NIFLY_RS_COLLISION_FIELD(
    bhkConvexListShape,
    bool,
    useCachedAABB,
    convex_list_use_cached_aabb
)
NIFLY_RS_COLLISION_FIELD(
    bhkConvexListShape,
    float,
    closestPointMinDistance,
    convex_list_closest_point_min_distance
)

// MOPP.
inline rust::Slice<const uint8_t>
mopp_code(const nifly::bhkMoppBvTreeShape& s) {
    return {s.data.data(), s.data.size()};
}

inline void
set_mopp_code(nifly::bhkMoppBvTreeShape& s, rust::Slice<const uint8_t> bytes) {
    s.data.resize(static_cast<uint32_t>(bytes.size()));
    std::copy(bytes.begin(), bytes.end(), s.data.data());
}

NIFLY_RS_COLLISION_VEC4(bhkMoppBvTreeShape, offset, mopp_offset)
NIFLY_RS_COLLISION_FIELD(bhkMoppBvTreeShape, float, scale, mopp_scale)
NIFLY_RS_COLLISION_FIELD(
    bhkMoppBvTreeShape,
    uint8_t,
    buildType,
    mopp_build_type
)

// bhkNiTriStripsShape.
NIFLY_RS_COLLISION_FIELD(
    bhkNiTriStripsShape,
    float,
    radius,
    strips_shape_radius
)
NIFLY_RS_COLLISION_FIELD(
    bhkNiTriStripsShape,
    uint32_t,
    growBy,
    strips_shape_grow_by
)
NIFLY_RS_COLLISION_VEC4(bhkNiTriStripsShape, scale, strips_shape_scale)

inline rust::Slice<const uint32_t>
strips_shape_filters(const nifly::bhkNiTriStripsShape& s) {
    return {s.filters.data(), s.filters.size()};
}

// False, changing nothing, unless there's one filter per strips data ref.
inline bool set_strips_shape_filters(
    nifly::bhkNiTriStripsShape& s,
    rust::Slice<const uint32_t> filters
) {
    if (filters.size() != s.partRefs.GetSize())
        return false;
    s.filters.resize(static_cast<uint32_t>(filters.size()));
    std::copy(filters.begin(), filters.end(), s.filters.data());
    return true;
}

// Sub-parts, stored in the shape up to stream 11 and in the data after.
template<class V>
rust::Vec<SubPartFfi> sub_parts_to_ffi(const V& parts) {
    rust::Vec<SubPartFfi> out;
    for (auto& p : const_items(parts))
        out.push_back({filter_to_ffi(p.filter), p.numVerts, p.material});
    return out;
}

// False, changing nothing, if there are more than 65535.
template<class V>
bool set_sub_parts(V& parts, rust::Slice<const SubPartFfi> values) {
    if (values.size() > 0xFFFF)
        return false;
    parts.clear();
    for (auto& v : values) {
        nifly::hkSubPartData p;
        p.filter = filter_from_ffi(v.filter);
        p.numVerts = v.vertex_count;
        p.material = v.material;
        parts.push_back(p);
    }
    return true;
}

inline rust::Vec<SubPartFfi>
packed_shape_sub_parts(const nifly::bhkPackedNiTriStripsShape& s) {
    return sub_parts_to_ffi(s.subPartData);
}

inline bool set_packed_shape_sub_parts(
    nifly::bhkPackedNiTriStripsShape& s,
    rust::Slice<const SubPartFfi> parts
) {
    return set_sub_parts(s.subPartData, parts);
}

inline rust::Vec<SubPartFfi>
packed_data_sub_parts(const nifly::hkPackedNiTriStripsData& d) {
    return sub_parts_to_ffi(d.subPartData);
}

inline bool set_packed_data_sub_parts(
    nifly::hkPackedNiTriStripsData& d,
    rust::Slice<const SubPartFfi> parts
) {
    return set_sub_parts(d.subPartData, parts);
}

NIFLY_RS_COLLISION_FIELD(
    bhkPackedNiTriStripsShape,
    uint32_t,
    userData,
    packed_shape_user_data
)
NIFLY_RS_COLLISION_FIELD(
    bhkPackedNiTriStripsShape,
    float,
    radius,
    packed_shape_radius
)
NIFLY_RS_COLLISION_VEC4(bhkPackedNiTriStripsShape, scaling, packed_shape_scale)
NIFLY_RS_COLLISION_FIELD(
    bhkPackedNiTriStripsShape,
    float,
    radius2,
    packed_shape_radius_copy
)
NIFLY_RS_COLLISION_VEC4(
    bhkPackedNiTriStripsShape,
    scaling2,
    packed_shape_scale_copy
)

// hkPackedNiTriStripsData triangles. Both lists are filled on set, so either
// version saves them; a get reads the one with normals if it's complete.
inline rust::Vec<PackedTriangleFfi>
packed_data_triangles(const nifly::hkPackedNiTriStripsData& d) {
    rust::Vec<PackedTriangleFfi> out;
    if (d.triNormData.size() == d.keyCount) {
        for (auto& t : d.triNormData)
            out.push_back(
                {t.tri.p1,
                 t.tri.p2,
                 t.tri.p3,
                 t.weldingInfo,
                 havok_vec3_to_ffi(t.normal)}
            );
    } else {
        for (uint32_t i = 0; i < d.keyCount && i < d.triData.size(); i++) {
            auto& t = d.triData[i];
            out.push_back(
                {t.tri.p1, t.tri.p2, t.tri.p3, t.weldingInfo, {0, 0, 0}}
            );
        }
    }
    return out;
}

// False, changing nothing, if an index isn't below the vertex count.
inline bool set_packed_data_triangles(
    nifly::hkPackedNiTriStripsData& d,
    rust::Slice<const PackedTriangleFfi> tris
) {
    for (auto& t : tris)
        if (t.p1 >= d.numVerts || t.p2 >= d.numVerts || t.p3 >= d.numVerts)
            return false;
    d.triData.clear();
    d.triNormData.clear();
    for (auto& t : tris) {
        nifly::hkTriangleData tri;
        tri.tri = nifly::Triangle(t.p1, t.p2, t.p3);
        tri.weldingInfo = t.welding_info;
        d.triData.push_back(tri);
        nifly::hkTriangleNormalData norm;
        norm.tri = tri.tri;
        norm.weldingInfo = t.welding_info;
        norm.normal = havok_vec3_from_ffi(t.normal);
        d.triNormData.push_back(norm);
    }
    d.keyCount = static_cast<uint32_t>(tris.size());
    return true;
}

inline rust::Vec<Vec3>
packed_data_vertices(const nifly::hkPackedNiTriStripsData& d) {
    rust::Vec<Vec3> out;
    for (uint32_t i = 0; i < d.numVerts && i < d.compressedVertData.size(); i++)
        out.push_back(havok_vec3_to_ffi(d.compressedVertData[i]));
    return out;
}

// False, changing nothing, if a triangle uses a vertex past the new count.
inline bool set_packed_data_vertices(
    nifly::hkPackedNiTriStripsData& d,
    rust::Slice<const Vec3> verts
) {
    for (auto& t : packed_data_triangles(d))
        if (t.p1 >= verts.size() || t.p2 >= verts.size()
            || t.p3 >= verts.size())
            return false;
    d.compressedVertData.clear();
    for (auto& v : verts)
        d.compressedVertData.push_back(havok_vec3_from_ffi(v));
    d.numVerts = static_cast<uint32_t>(verts.size());
    return true;
}

// bhkCompressedMeshShape.
NIFLY_RS_COLLISION_FIELD(
    bhkCompressedMeshShape,
    uint32_t,
    userData,
    compressed_shape_user_data
)
NIFLY_RS_COLLISION_FIELD(
    bhkCompressedMeshShape,
    float,
    radius,
    compressed_shape_radius
)
NIFLY_RS_COLLISION_FIELD(
    bhkCompressedMeshShape,
    float,
    unkFloat,
    compressed_shape_unknown_float
)
NIFLY_RS_COLLISION_VEC4(bhkCompressedMeshShape, scaling, compressed_shape_scale)
NIFLY_RS_COLLISION_FIELD(
    bhkCompressedMeshShape,
    float,
    radius2,
    compressed_shape_radius_copy
)
NIFLY_RS_COLLISION_VEC4(
    bhkCompressedMeshShape,
    scaling2,
    compressed_shape_scale_copy
)

// bhkCompressedMeshShapeData.
NIFLY_RS_COLLISION_FIELD(
    bhkCompressedMeshShapeData,
    uint32_t,
    bitsPerIndex,
    cms_bits_per_index
)
NIFLY_RS_COLLISION_FIELD(
    bhkCompressedMeshShapeData,
    uint32_t,
    bitsPerWIndex,
    cms_bits_per_welding_index
)
NIFLY_RS_COLLISION_FIELD(
    bhkCompressedMeshShapeData,
    uint32_t,
    maskWIndex,
    cms_mask_welding_index
)
NIFLY_RS_COLLISION_FIELD(
    bhkCompressedMeshShapeData,
    uint32_t,
    maskIndex,
    cms_mask_index
)
NIFLY_RS_COLLISION_FIELD(bhkCompressedMeshShapeData, float, error, cms_error)
NIFLY_RS_COLLISION_VEC4(bhkCompressedMeshShapeData, aabbBoundMin, cms_aabb_min)
NIFLY_RS_COLLISION_VEC4(bhkCompressedMeshShapeData, aabbBoundMax, cms_aabb_max)
NIFLY_RS_COLLISION_FIELD(
    bhkCompressedMeshShapeData,
    uint8_t,
    weldingType,
    cms_welding_type
)
NIFLY_RS_COLLISION_FIELD(
    bhkCompressedMeshShapeData,
    uint8_t,
    materialType,
    cms_material_type
)
NIFLY_RS_COLLISION_FIELD(
    bhkCompressedMeshShapeData,
    uint32_t,
    numNamedMat,
    cms_named_material_count
)
NIFLY_RS_COLLISION_FIELD(
    bhkCompressedMeshShapeData,
    uint32_t,
    numConvexPieceA,
    cms_convex_piece_count
)

// Material lists 32 (0), 16 (1) and 8 (2).
inline nifly::NiVector<uint32_t>&
cms_material_list(nifly::bhkCompressedMeshShapeData& d, uint8_t which) {
    return which == 0 ? d.mat32 : which == 1 ? d.mat16 : d.mat8;
}

inline rust::Slice<const uint32_t>
cms_materials_n(const nifly::bhkCompressedMeshShapeData& d, uint8_t which) {
    auto& list = cms_material_list(
        const_cast<nifly::bhkCompressedMeshShapeData&>(d),
        which
    );
    return {list.data(), list.size()};
}

inline void set_cms_materials_n(
    nifly::bhkCompressedMeshShapeData& d,
    uint8_t which,
    rust::Slice<const uint32_t> values
) {
    auto& list = cms_material_list(d, which);
    list.resize(static_cast<uint32_t>(values.size()));
    std::copy(values.begin(), values.end(), list.data());
}

inline rust::Vec<CmsMaterialFfi>
cms_materials(const nifly::bhkCompressedMeshShapeData& d) {
    rust::Vec<CmsMaterialFfi> out;
    for (auto& m : const_items(d.materials))
        out.push_back({m.material, filter_to_ffi(m.layer)});
    return out;
}

inline void set_cms_materials(
    nifly::bhkCompressedMeshShapeData& d,
    rust::Slice<const CmsMaterialFfi> values
) {
    d.materials.clear();
    for (auto& v : values) {
        nifly::bhkCMSDMaterial m;
        m.material = v.material;
        m.layer = filter_from_ffi(v.filter);
        d.materials.push_back(m);
    }
}

inline rust::Vec<CmsTransformFfi>
cms_transforms(const nifly::bhkCompressedMeshShapeData& d) {
    rust::Vec<CmsTransformFfi> out;
    for (auto& t : const_items(d.transforms))
        out.push_back({vec4_to_ffi(t.translation), quat_to_ffi(t.rotation)});
    return out;
}

inline void set_cms_transforms(
    nifly::bhkCompressedMeshShapeData& d,
    rust::Slice<const CmsTransformFfi> values
) {
    d.transforms.clear();
    for (auto& v : values) {
        nifly::bhkCMSDTransform t;
        t.translation = vec4_from_ffi(v.translation);
        t.rotation = quat_from_ffi(v.rotation);
        d.transforms.push_back(t);
    }
}

inline rust::Slice<const float>
cms_big_vertices(const nifly::bhkCompressedMeshShapeData& d) {
    return vec4_list(d.bigVerts);
}

// False, changing nothing, unless there are 4 floats per vertex and the big
// triangles only use vertices below the new count.
inline bool set_cms_big_vertices(
    nifly::bhkCompressedMeshShapeData& d,
    rust::Slice<const float> values
) {
    size_t count = values.size() / 4;
    for (auto& t : const_items(d.bigTris))
        if (t.triangle1 >= count || t.triangle2 >= count
            || t.triangle3 >= count)
            return false;
    return set_vec4_list(d.bigVerts, values);
}

inline rust::Vec<CmsBigTriangleFfi>
cms_big_triangles(const nifly::bhkCompressedMeshShapeData& d) {
    rust::Vec<CmsBigTriangleFfi> out;
    for (auto& t : const_items(d.bigTris))
        out.push_back(
            {t.triangle1, t.triangle2, t.triangle3, t.material, t.weldingInfo}
        );
    return out;
}

// False, changing nothing, if an index isn't below the big vertex count.
inline bool set_cms_big_triangles(
    nifly::bhkCompressedMeshShapeData& d,
    rust::Slice<const CmsBigTriangleFfi> values
) {
    uint32_t count = d.bigVerts.size();
    for (auto& v : values)
        if (v.p1 >= count || v.p2 >= count || v.p3 >= count)
            return false;
    d.bigTris.clear();
    for (auto& v : values) {
        nifly::bhkCMSDBigTris t;
        t.triangle1 = v.p1;
        t.triangle2 = v.p2;
        t.triangle3 = v.p3;
        t.material = v.material;
        t.weldingInfo = v.welding_info;
        d.bigTris.push_back(t);
    }
    return true;
}

inline size_t cms_chunk_count(const nifly::bhkCompressedMeshShapeData& d) {
    return d.chunks.size();
}

inline CmsChunkFfi
cms_chunk(const nifly::bhkCompressedMeshShapeData& d, size_t index) {
    auto& c = const_cast<nifly::bhkCompressedMeshShapeData&>(d)
                  .chunks[static_cast<uint32_t>(index)];
    return {
        vec4_to_ffi(c.translation),
        c.matIndex,
        c.reference,
        c.transformIndex
    };
}

// A chunk's vertices (0), indices (1), strips (2) or welding info (3).
inline rust::Slice<const uint16_t> cms_chunk_list(
    const nifly::bhkCompressedMeshShapeData& d,
    size_t index,
    uint8_t which
) {
    auto& c = const_cast<nifly::bhkCompressedMeshShapeData&>(d)
                  .chunks[static_cast<uint32_t>(index)];
    auto& list = which == 0 ? c.verts
        : which == 1        ? c.indices
        : which == 2        ? c.strips
                            : c.weldingInfo;
    return {list.data(), list.size()};
}

// Replaces the chunks. `lists` holds each chunk's vertices, indices, strips
// and welding info in turn, with their lengths in `lengths` (4 per chunk).
// False, changing nothing, unless the lengths match.
inline bool set_cms_chunks(
    nifly::bhkCompressedMeshShapeData& d,
    rust::Slice<const CmsChunkFfi> chunks,
    rust::Slice<const uint16_t> lists,
    rust::Slice<const uint32_t> lengths
) {
    if (lengths.size() != chunks.size() * 4)
        return false;
    size_t total = 0;
    for (uint32_t n : lengths)
        total += n;
    if (total != lists.size())
        return false;

    d.chunks.clear();
    size_t pos = 0;
    for (size_t i = 0; i < chunks.size(); i++) {
        nifly::bhkCMSDChunk c;
        c.translation = vec4_from_ffi(chunks[i].translation);
        c.matIndex = chunks[i].material_index;
        c.reference = chunks[i].reference;
        c.transformIndex = chunks[i].transform_index;
        nifly::NiVector<uint16_t>* targets[4] =
            {&c.verts, &c.indices, &c.strips, &c.weldingInfo};
        for (int k = 0; k < 4; k++) {
            uint32_t n = lengths[i * 4 + k];
            targets[k]->resize(n);
            std::copy(
                lists.begin() + pos,
                lists.begin() + pos + n,
                targets[k]->data()
            );
            pos += n;
        }
        d.chunks.push_back(c);
    }
    return true;
}

// Shape creation.
// The shape of `owner` (a world object, transform shape or MOPP shape), or
// null.
inline nifly::NiBlockRef<nifly::bhkShape>*
child_shape_ref(nifly::NiObject* owner) {
    if (auto o = dynamic_cast<nifly::bhkWorldObject*>(owner))
        return &o->shapeRef;
    if (auto o = dynamic_cast<nifly::bhkTransformShape*>(owner))
        return &o->shapeRef;
    if (auto o = dynamic_cast<nifly::bhkMoppBvTreeShape*>(owner))
        return &o->shapeRef;
    return nullptr;
}

// Adds a new S as the shape of the block at `owner` and returns its index.
// NIF_NPOS, changing nothing, if the block has no shape ref.
template<class S>
uint32_t assign_shape(nifly::NifFile& file, uint32_t owner) {
    auto& hdr = file.GetHeader();
    auto ref = child_shape_ref(hdr.GetBlock<nifly::NiObject>(owner));
    if (!ref)
        return nifly::NIF_NPOS;
    uint32_t id = hdr.AddBlock(std::make_unique<S>());
    ref->index = id;
    return id;
}

// Adds a new S at the end of the sub-shapes of the list or convex list shape
// at `owner` and returns its index. A list shape with one filter per
// sub-shape gets a default filter for it. NIF_NPOS, changing nothing, if the
// block isn't such a shape or is a convex list and S isn't convex.
template<class S>
uint32_t add_sub_shape(nifly::NifFile& file, uint32_t owner) {
    auto& hdr = file.GetHeader();
    auto obj = hdr.GetBlock<nifly::NiObject>(owner);
    if (auto list = dynamic_cast<nifly::bhkListShape*>(obj)) {
        bool in_step = list->filters.size() == list->subShapeRefs.GetSize();
        uint32_t id = hdr.AddBlock(std::make_unique<S>());
        list->subShapeRefs.AddBlockRef(id);
        if (in_step) {
            nifly::HavokFilter filter;
            list->filters.push_back(filter);
        }
        return id;
    }
    if (auto list = dynamic_cast<nifly::bhkConvexListShape*>(obj)) {
        if constexpr (!std::is_base_of_v<nifly::bhkConvexShape, S>)
            return nifly::NIF_NPOS;
        else {
            uint32_t id = hdr.AddBlock(std::make_unique<S>());
            list->shapeRefs.AddBlockRef(id);
            return id;
        }
    }
    return nifly::NIF_NPOS;
}

#define NIFLY_RS_NEW_SHAPE(T, snake) \
    inline uint32_t assign_##snake(nifly::NifFile& file, uint32_t owner) { \
        return assign_shape<nifly::T>(file, owner); \
    } \
    inline uint32_t add_sub_##snake(nifly::NifFile& file, uint32_t owner) { \
        return add_sub_shape<nifly::T>(file, owner); \
    }

NIFLY_RS_NEW_SHAPE(bhkPlaneShape, bhk_plane_shape)
NIFLY_RS_NEW_SHAPE(bhkMultiSphereShape, bhk_multi_sphere_shape)
NIFLY_RS_NEW_SHAPE(bhkConvexListShape, bhk_convex_list_shape)
NIFLY_RS_NEW_SHAPE(bhkConvexVerticesShape, bhk_convex_vertices_shape)
NIFLY_RS_NEW_SHAPE(bhkBoxShape, bhk_box_shape)
NIFLY_RS_NEW_SHAPE(bhkSphereShape, bhk_sphere_shape)
NIFLY_RS_NEW_SHAPE(bhkCylinderShape, bhk_cylinder_shape)
NIFLY_RS_NEW_SHAPE(bhkTransformShape, bhk_transform_shape)
NIFLY_RS_NEW_SHAPE(bhkConvexTransformShape, bhk_convex_transform_shape)
NIFLY_RS_NEW_SHAPE(bhkCapsuleShape, bhk_capsule_shape)
NIFLY_RS_NEW_SHAPE(bhkMoppBvTreeShape, bhk_mopp_bv_tree_shape)
NIFLY_RS_NEW_SHAPE(bhkNiTriStripsShape, bhk_ni_tri_strips_shape)
NIFLY_RS_NEW_SHAPE(bhkListShape, bhk_list_shape)
NIFLY_RS_NEW_SHAPE(bhkPackedNiTriStripsShape, bhk_packed_ni_tri_strips_shape)
NIFLY_RS_NEW_SHAPE(bhkCompressedMeshShape, bhk_compressed_mesh_shape)

#undef NIFLY_RS_NEW_SHAPE

// Adds new data to the packed or compressed mesh shape at `shape` and returns
// its index. NIF_NPOS, changing nothing, if the block isn't one.
inline uint32_t assign_packed_data(nifly::NifFile& file, uint32_t shape) {
    auto& hdr = file.GetHeader();
    auto s = hdr.GetBlock<nifly::bhkPackedNiTriStripsShape>(shape);
    if (!s)
        return nifly::NIF_NPOS;
    uint32_t id =
        hdr.AddBlock(std::make_unique<nifly::hkPackedNiTriStripsData>());
    s->dataRef.index = id;
    return id;
}

inline uint32_t assign_compressed_data(nifly::NifFile& file, uint32_t shape) {
    auto& hdr = file.GetHeader();
    auto s = hdr.GetBlock<nifly::bhkCompressedMeshShape>(shape);
    if (!s)
        return nifly::NIF_NPOS;
    uint32_t id =
        hdr.AddBlock(std::make_unique<nifly::bhkCompressedMeshShapeData>());
    s->dataRef.index = id;
    return id;
}

// Constraints.
NIFLY_RS_COLLISION_FIELD(bhkConstraint, uint32_t, priority, constraint_priority)

inline MotorFfi motor_to_ffi(const nifly::MotorDesc& m) {
    MotorFfi out {};
    out.motor_type = m.motorType;
    switch (m.motorType) {
        case nifly::MOTOR_POSITION: {
            auto& p = m.motorPosition;
            out.min_force = p.minForce;
            out.max_force = p.maxForce;
            out.tau = p.tau;
            out.damping = p.damping;
            out.proportional_recovery_velocity = p.proportionalRecoveryVelocity;
            out.constant_recovery_velocity = p.constantRecoveryVelocity;
            out.enabled = p.motorEnabled;
            break;
        }
        case nifly::MOTOR_VELOCITY: {
            auto& v = m.motorVelocity;
            out.min_force = v.minForce;
            out.max_force = v.maxForce;
            out.tau = v.tau;
            out.velocity_target = v.velocityTarget;
            out.use_velocity_target = v.useVelocityTargetFromConstraintTargets;
            out.enabled = v.motorEnabled;
            break;
        }
        case nifly::MOTOR_SPRING: {
            auto& s = m.motorSpringDamper;
            out.min_force = s.minForce;
            out.max_force = s.maxForce;
            out.spring_constant = s.springConstant;
            out.spring_damping = s.springDamping;
            out.enabled = s.motorEnabled;
            break;
        }
        default:
            break;
    }
    return out;
}

// Sets the motor type and the fields of that type's motor.
inline void motor_from_ffi(nifly::MotorDesc& m, const MotorFfi& f) {
    m.motorType = static_cast<nifly::MotorType>(f.motor_type);
    switch (m.motorType) {
        case nifly::MOTOR_POSITION: {
            auto& p = m.motorPosition;
            p.minForce = f.min_force;
            p.maxForce = f.max_force;
            p.tau = f.tau;
            p.damping = f.damping;
            p.proportionalRecoveryVelocity = f.proportional_recovery_velocity;
            p.constantRecoveryVelocity = f.constant_recovery_velocity;
            p.motorEnabled = f.enabled;
            break;
        }
        case nifly::MOTOR_VELOCITY: {
            auto& v = m.motorVelocity;
            v.minForce = f.min_force;
            v.maxForce = f.max_force;
            v.tau = f.tau;
            v.velocityTarget = f.velocity_target;
            v.useVelocityTargetFromConstraintTargets = f.use_velocity_target;
            v.motorEnabled = f.enabled;
            break;
        }
        case nifly::MOTOR_SPRING: {
            auto& s = m.motorSpringDamper;
            s.minForce = f.min_force;
            s.maxForce = f.max_force;
            s.springConstant = f.spring_constant;
            s.springDamping = f.spring_damping;
            s.motorEnabled = f.enabled;
            break;
        }
        default:
            break;
    }
}

// Descriptors as DescriptorFfi: up to 8 vectors, 6 floats and a motor, in the
// order of the descriptor's members.
inline void put_vec(DescriptorFfi& d, int i, const nifly::Vector4& v) {
    d.vectors[i * 4] = v.x;
    d.vectors[i * 4 + 1] = v.y;
    d.vectors[i * 4 + 2] = v.z;
    d.vectors[i * 4 + 3] = v.w;
}

inline nifly::Vector4 get_vec(const DescriptorFfi& d, int i) {
    return nifly::Vector4(
        d.vectors[i * 4],
        d.vectors[i * 4 + 1],
        d.vectors[i * 4 + 2],
        d.vectors[i * 4 + 3]
    );
}

inline DescriptorFfi desc_to_ffi(const nifly::BallAndSocketDesc& c) {
    DescriptorFfi d {};
    put_vec(d, 0, c.translationA);
    put_vec(d, 1, c.translationB);
    return d;
}

inline void desc_from_ffi(nifly::BallAndSocketDesc& c, const DescriptorFfi& d) {
    c.translationA = get_vec(d, 0);
    c.translationB = get_vec(d, 1);
}

inline DescriptorFfi desc_to_ffi(const nifly::StiffSpringDesc& c) {
    DescriptorFfi d {};
    put_vec(d, 0, c.pivotA);
    put_vec(d, 1, c.pivotB);
    d.floats[0] = c.length;
    return d;
}

inline void desc_from_ffi(nifly::StiffSpringDesc& c, const DescriptorFfi& d) {
    c.pivotA = get_vec(d, 0);
    c.pivotB = get_vec(d, 1);
    c.length = d.floats[0];
}

// Hinge-like descriptors with eight vectors.
template<class D, class V>
DescriptorFfi eight_to_ffi(const D& c, V members) {
    DescriptorFfi d {};
    auto ptrs = members(const_cast<D&>(c));
    for (int i = 0; i < 8; i++)
        put_vec(d, i, *ptrs[i]);
    return d;
}

template<class D, class V>
void eight_from_ffi(D& c, const DescriptorFfi& d, V members) {
    auto ptrs = members(c);
    for (int i = 0; i < 8; i++)
        *ptrs[i] = get_vec(d, i);
}

inline std::array<nifly::Vector4*, 8> hinge_vecs(nifly::HingeDesc& c) {
    return {
        &c.axleA,
        &c.axleInA1,
        &c.axleInA2,
        &c.pivotA,
        &c.axleB,
        &c.axleInB1,
        &c.axleInB2,
        &c.pivotB
    };
}

inline DescriptorFfi desc_to_ffi(const nifly::HingeDesc& c) {
    return eight_to_ffi(c, hinge_vecs);
}

inline void desc_from_ffi(nifly::HingeDesc& c, const DescriptorFfi& d) {
    eight_from_ffi(c, d, hinge_vecs);
}

inline std::array<nifly::Vector4*, 8>
limited_hinge_vecs(nifly::LimitedHingeDesc& c) {
    return {
        &c.axleA,
        &c.axleInA1,
        &c.axleInA2,
        &c.pivotA,
        &c.axleB,
        &c.axleInB1,
        &c.axleInB2,
        &c.pivotB
    };
}

inline DescriptorFfi desc_to_ffi(const nifly::LimitedHingeDesc& c) {
    DescriptorFfi d = eight_to_ffi(c, limited_hinge_vecs);
    d.floats[0] = c.minAngle;
    d.floats[1] = c.maxAngle;
    d.floats[2] = c.maxFriction;
    d.motor = motor_to_ffi(c.motorDesc);
    return d;
}

inline void desc_from_ffi(nifly::LimitedHingeDesc& c, const DescriptorFfi& d) {
    eight_from_ffi(c, d, limited_hinge_vecs);
    c.minAngle = d.floats[0];
    c.maxAngle = d.floats[1];
    c.maxFriction = d.floats[2];
    motor_from_ffi(c.motorDesc, d.motor);
}

inline std::array<nifly::Vector4*, 8> ragdoll_vecs(nifly::RagdollDesc& c) {
    return {
        &c.twistA,
        &c.planeA,
        &c.motorA,
        &c.pivotA,
        &c.twistB,
        &c.planeB,
        &c.motorB,
        &c.pivotB
    };
}

inline DescriptorFfi desc_to_ffi(const nifly::RagdollDesc& c) {
    DescriptorFfi d = eight_to_ffi(c, ragdoll_vecs);
    d.floats[0] = c.coneMaxAngle;
    d.floats[1] = c.planeMinAngle;
    d.floats[2] = c.planeMaxAngle;
    d.floats[3] = c.twistMinAngle;
    d.floats[4] = c.twistMaxAngle;
    d.floats[5] = c.maxFriction;
    d.motor = motor_to_ffi(c.motorDesc);
    return d;
}

inline void desc_from_ffi(nifly::RagdollDesc& c, const DescriptorFfi& d) {
    eight_from_ffi(c, d, ragdoll_vecs);
    c.coneMaxAngle = d.floats[0];
    c.planeMinAngle = d.floats[1];
    c.planeMaxAngle = d.floats[2];
    c.twistMinAngle = d.floats[3];
    c.twistMaxAngle = d.floats[4];
    c.maxFriction = d.floats[5];
    motor_from_ffi(c.motorDesc, d.motor);
}

inline std::array<nifly::Vector4*, 8> prismatic_vecs(nifly::PrismaticDesc& c) {
    return {
        &c.slidingA,
        &c.rotationA,
        &c.planeA,
        &c.pivotA,
        &c.slidingB,
        &c.rotationB,
        &c.planeB,
        &c.pivotB
    };
}

inline DescriptorFfi desc_to_ffi(const nifly::PrismaticDesc& c) {
    DescriptorFfi d = eight_to_ffi(c, prismatic_vecs);
    d.floats[0] = c.minDistance;
    d.floats[1] = c.maxDistance;
    d.floats[2] = c.friction;
    d.motor = motor_to_ffi(c.motorDesc);
    return d;
}

inline void desc_from_ffi(nifly::PrismaticDesc& c, const DescriptorFfi& d) {
    eight_from_ffi(c, d, prismatic_vecs);
    c.minDistance = d.floats[0];
    c.maxDistance = d.floats[1];
    c.friction = d.floats[2];
    motor_from_ffi(c.motorDesc, d.motor);
}

#define NIFLY_RS_DESCRIPTOR(T, field, snake) \
    inline DescriptorFfi snake(const nifly::T& c) { \
        return desc_to_ffi(c.field); \
    } \
    inline void set_##snake(nifly::T& c, const DescriptorFfi& d) { \
        desc_from_ffi(c.field, d); \
    }

NIFLY_RS_DESCRIPTOR(bhkHingeConstraint, hinge, hinge_descriptor)
NIFLY_RS_DESCRIPTOR(
    bhkLimitedHingeConstraint,
    limitedHinge,
    limited_hinge_descriptor
)
NIFLY_RS_DESCRIPTOR(bhkRagdollConstraint, ragdoll, ragdoll_descriptor)
NIFLY_RS_DESCRIPTOR(
    bhkStiffSpringConstraint,
    stiffSpring,
    stiff_spring_descriptor
)
NIFLY_RS_DESCRIPTOR(bhkPrismaticConstraint, prismatic, prismatic_descriptor)
NIFLY_RS_DESCRIPTOR(
    bhkBallAndSocketConstraint,
    ballAndSocket,
    ball_and_socket_descriptor
)

#undef NIFLY_RS_DESCRIPTOR

// The descriptor of a wrapped constraint's type, with `constraint_type` set.
// Only the type's descriptor is filled; an unknown type fills none.
inline DescriptorFfi wrapped_to_ffi(const nifly::ConstraintData& c) {
    DescriptorFfi d {};
    switch (c.type) {
        case nifly::BallAndSocket:
            d = desc_to_ffi(c.desc1);
            break;
        case nifly::Hinge:
            d = desc_to_ffi(c.desc2);
            break;
        case nifly::LimitedHinge:
            d = desc_to_ffi(c.desc3);
            break;
        case nifly::Prismatic:
            d = desc_to_ffi(c.desc4);
            break;
        case nifly::Ragdoll:
            d = desc_to_ffi(c.desc5);
            break;
        case nifly::StiffSpring:
            d = desc_to_ffi(c.desc6);
            break;
        default:
            break;
    }
    d.constraint_type = c.type;
    return d;
}

// Sets the type and that type's descriptor. False, changing nothing, for an
// unknown type.
inline bool wrapped_from_ffi(nifly::ConstraintData& c, const DescriptorFfi& d) {
    switch (d.constraint_type) {
        case nifly::BallAndSocket:
            desc_from_ffi(c.desc1, d);
            break;
        case nifly::Hinge:
            desc_from_ffi(c.desc2, d);
            break;
        case nifly::LimitedHinge:
            desc_from_ffi(c.desc3, d);
            break;
        case nifly::Prismatic:
            desc_from_ffi(c.desc4, d);
            break;
        case nifly::Ragdoll:
            desc_from_ffi(c.desc5, d);
            break;
        case nifly::StiffSpring:
            desc_from_ffi(c.desc6, d);
            break;
        default:
            return false;
    }
    c.type = static_cast<nifly::hkConstraintType>(d.constraint_type);
    return true;
}

#define NIFLY_RS_WRAPPED(T, snake) \
    inline DescriptorFfi snake##_wrapped(const nifly::T& c) { \
        return wrapped_to_ffi(c.subConstraint); \
    } \
    inline bool set_##snake##_wrapped(nifly::T& c, const DescriptorFfi& d) { \
        return wrapped_from_ffi(c.subConstraint, d); \
    } \
    inline uint32_t snake##_wrapped_type(const nifly::T& c) { \
        return c.subConstraint.type; \
    } \
    NIFLY_RS_COLLISION_FIELD( \
        T, \
        uint32_t, \
        subConstraint.priority, \
        snake##_wrapped_priority \
    ) \
    NIFLY_RS_COLLISION_FIELD(T, float, subConstraint.tau, snake##_tau) \
    NIFLY_RS_COLLISION_FIELD(T, float, subConstraint.damping, snake##_damping) \
    NIFLY_RS_COLLISION_FIELD(T, float, subConstraint.strength, snake##_strength)

NIFLY_RS_WRAPPED(bhkBreakableConstraint, breakable)
NIFLY_RS_WRAPPED(bhkMalleableConstraint, malleable)

#undef NIFLY_RS_WRAPPED

NIFLY_RS_COLLISION_FIELD(
    bhkBreakableConstraint,
    bool,
    removeWhenBroken,
    breakable_remove_when_broken
)

// bhkBallSocketConstraintChain.
inline rust::Slice<const float>
chain_pivots(const nifly::bhkBallSocketConstraintChain& c) {
    return vec4_list(c.pivots);
}

inline bool set_chain_pivots(
    nifly::bhkBallSocketConstraintChain& c,
    rust::Slice<const float> values
) {
    return set_vec4_list(c.pivots, values);
}

NIFLY_RS_COLLISION_FIELD(bhkBallSocketConstraintChain, float, tau, chain_tau)
NIFLY_RS_COLLISION_FIELD(
    bhkBallSocketConstraintChain,
    float,
    damping,
    chain_damping
)
NIFLY_RS_COLLISION_FIELD(bhkBallSocketConstraintChain, float, cfm, chain_cfm)
NIFLY_RS_COLLISION_FIELD(
    bhkBallSocketConstraintChain,
    float,
    maxErrorDistance,
    chain_max_error_distance
)
NIFLY_RS_COLLISION_FIELD(
    bhkBallSocketConstraintChain,
    uint32_t,
    priority,
    chain_priority
)

// Gives a constraint's entity array its two slots.
inline void two_entity_slots(nifly::NiBlockRefArray<nifly::bhkEntity>& refs) {
    refs.SetKeepEmptyRefs();
    if (refs.GetSize() != 2)
        refs.SetSize(2);
}

// Adds a new C to the rigid body at `body`'s constraints, with the body as
// entity A, and returns its index. NIF_NPOS,
// changing nothing, if the block isn't a rigid body.
template<class C>
uint32_t add_constraint(nifly::NifFile& file, uint32_t body) {
    auto& hdr = file.GetHeader();
    auto b = hdr.GetBlock<nifly::bhkRigidBody>(body);
    if (!b)
        return nifly::NIF_NPOS;
    auto c = std::make_unique<C>();
    if constexpr (std::is_same_v<C, nifly::bhkBallSocketConstraintChain>)
        c->entityARef.index = body;
    else {
        two_entity_slots(c->entityRefs);
        c->entityRefs.SetBlockRef(0, body);
    }
    uint32_t id = hdr.AddBlock(std::move(c));
    b->constraintRefs.AddBlockRef(id);
    return id;
}

#define NIFLY_RS_ADD_CONSTRAINT(T, snake) \
    inline uint32_t add_##snake(nifly::NifFile& file, uint32_t body) { \
        return add_constraint<nifly::T>(file, body); \
    }

NIFLY_RS_ADD_CONSTRAINT(bhkHingeConstraint, bhk_hinge_constraint)
NIFLY_RS_ADD_CONSTRAINT(bhkLimitedHingeConstraint, bhk_limited_hinge_constraint)
NIFLY_RS_ADD_CONSTRAINT(bhkRagdollConstraint, bhk_ragdoll_constraint)
NIFLY_RS_ADD_CONSTRAINT(bhkStiffSpringConstraint, bhk_stiff_spring_constraint)
NIFLY_RS_ADD_CONSTRAINT(bhkPrismaticConstraint, bhk_prismatic_constraint)
NIFLY_RS_ADD_CONSTRAINT(
    bhkBallAndSocketConstraint,
    bhk_ball_and_socket_constraint
)
NIFLY_RS_ADD_CONSTRAINT(bhkBreakableConstraint, bhk_breakable_constraint)
NIFLY_RS_ADD_CONSTRAINT(bhkMalleableConstraint, bhk_malleable_constraint)
NIFLY_RS_ADD_CONSTRAINT(
    bhkBallSocketConstraintChain,
    bhk_ball_socket_constraint_chain
)

#undef NIFLY_RS_ADD_CONSTRAINT

// bhkPoseArray.
inline size_t pose_array_bone_count(const nifly::bhkPoseArray& p) {
    return p.bones.size();
}

inline rust::Slice<const uint8_t>
pose_array_bone(const nifly::bhkPoseArray& p, size_t index) {
    if (index >= p.bones.size())
        return {};
    return extra_bytes(p.bones.data()[index].get());
}

inline void set_pose_array_bones(
    nifly::bhkPoseArray& p,
    rust::Slice<const uint8_t> bytes,
    rust::Slice<const size_t> lengths
) {
    p.bones.clear();
    for (const auto& s : split_strings(bytes, lengths)) {
        nifly::NiStringRef str(s);
        p.bones.push_back(str);
    }
}

inline size_t pose_array_pose_count(const nifly::bhkPoseArray& p) {
    return p.poses.size();
}

inline rust::Vec<BoneTransformFfi>
pose_array_pose(const nifly::bhkPoseArray& p, size_t index) {
    rust::Vec<BoneTransformFfi> out;
    if (index >= p.poses.size())
        return out;
    auto& pose = p.poses.data()[index];
    for (auto& m : const_items(pose.matrices))
        out.push_back(
            {havok_vec3_to_ffi(m.translation),
             quat_to_ffi(m.rotation),
             havok_vec3_to_ffi(m.scale)}
        );
    return out;
}

// Replaces the poses: `lengths` holds each pose's transform count, and
// `transforms` the transforms in turn. False, changing nothing, unless the
// lengths add up.
inline bool set_pose_array_poses(
    nifly::bhkPoseArray& p,
    rust::Slice<const BoneTransformFfi> transforms,
    rust::Slice<const size_t> lengths
) {
    size_t total = 0;
    for (size_t n : lengths)
        total += n;
    if (total != transforms.size())
        return false;
    p.poses.clear();
    size_t pos = 0;
    for (size_t n : lengths) {
        nifly::BonePose pose;
        for (size_t i = 0; i < n; i++, pos++) {
            nifly::BoneMatrix m;
            m.translation = havok_vec3_from_ffi(transforms[pos].translation);
            m.rotation = quat_from_ffi(transforms[pos].rotation);
            m.scale = havok_vec3_from_ffi(transforms[pos].scale);
            pose.matrices.push_back(m);
        }
        p.poses.push_back(pose);
    }
    return true;
}

// bhkRagdollTemplateData.
inline rust::Slice<const uint8_t>
template_data_name(const nifly::bhkRagdollTemplateData& d) {
    return extra_bytes(d.name.get());
}

inline void set_template_data_name(
    nifly::bhkRagdollTemplateData& d,
    rust::Slice<const uint8_t> name
) {
    d.name.get().assign(
        reinterpret_cast<const char*>(name.data()),
        name.size()
    );
}

NIFLY_RS_COLLISION_FIELD(
    bhkRagdollTemplateData,
    float,
    mass,
    template_data_mass
)
NIFLY_RS_COLLISION_FIELD(
    bhkRagdollTemplateData,
    float,
    restitution,
    template_data_restitution
)
NIFLY_RS_COLLISION_FIELD(
    bhkRagdollTemplateData,
    float,
    friction,
    template_data_friction
)
NIFLY_RS_COLLISION_FIELD(
    bhkRagdollTemplateData,
    float,
    radius,
    template_data_radius
)
NIFLY_RS_COLLISION_FIELD(
    bhkRagdollTemplateData,
    uint32_t,
    material,
    template_data_material
)

// The joint constraints, with their raw entity indices.
inline rust::Vec<JointConstraintFfi>
template_data_constraints(const nifly::bhkRagdollTemplateData& d) {
    rust::Vec<JointConstraintFfi> out;
    for (auto& c : const_items(d.constraints)) {
        auto entity = [&](uint32_t i) {
            return c.entityRefs.GetSize() > i ? c.entityRefs.GetBlockRef(i)
                                              : nifly::NIF_NPOS;
        };
        out.push_back(
            {wrapped_to_ffi(c),
             c.priority,
             c.tau,
             c.damping,
             c.strength,
             entity(0),
             entity(1)}
        );
    }
    return out;
}

// Replaces the joint constraints. Each keeps the entities of the one at its
// position, if any. False, changing nothing, if a type is unknown.
inline bool set_template_data_constraints(
    nifly::bhkRagdollTemplateData& d,
    rust::Slice<const JointConstraintFfi> values
) {
    std::vector<nifly::ConstraintData> list;
    for (size_t i = 0; i < values.size(); i++) {
        auto& v = values[i];
        nifly::ConstraintData c;
        if (!wrapped_from_ffi(c, v.descriptor))
            return false;
        if (i < d.constraints.size())
            c.entityRefs = d.constraints.data()[i].entityRefs;
        c.priority = v.priority;
        c.tau = v.tau;
        c.damping = v.damping;
        c.strength = v.strength;
        list.push_back(std::move(c));
    }
    d.constraints.clear();
    for (auto& c : list)
        d.constraints.push_back(c);
    return true;
}

// Adds a new pose array or ragdoll template data block and returns its index.
inline uint32_t add_bhk_pose_array(nifly::NifFile& file) {
    return file.GetHeader().AddBlock(std::make_unique<nifly::bhkPoseArray>());
}

inline uint32_t add_bhk_ragdoll_template_data(nifly::NifFile& file) {
    return file.GetHeader().AddBlock(
        std::make_unique<nifly::bhkRagdollTemplateData>()
    );
}

inline uint32_t add_bhk_ragdoll_template(nifly::NifFile& file) {
    return file.GetHeader().AddBlock(
        std::make_unique<nifly::bhkRagdollTemplate>()
    );
}

#undef NIFLY_RS_COLLISION_VEC3
} // namespace nifly_rs

#undef NIFLY_RS_COLLISION_ARRAY
#undef NIFLY_RS_COMMA
#undef NIFLY_RS_COLLISION_VEC4
#undef NIFLY_RS_COLLISION_CONVERT

#undef NIFLY_RS_COLLISION_FIELD
#undef NIFLY_RS_COLLISION
