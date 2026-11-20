// Part of collision_shim.h: constraints, motors and descriptors.
#pragma once

#include "collision_shim.h"

namespace nifly_rs {
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
