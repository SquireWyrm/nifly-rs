// Part of collision_shim.h: world objects, rigid bodies, phantoms and actions.
#pragma once

#include "collision_shim.h"

namespace nifly_rs {
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
} // namespace nifly_rs
