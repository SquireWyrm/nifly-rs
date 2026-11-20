//! `NiCollisionObject` and the Havok (`bhk*`) collision blocks.
//!
//! For each type `T` with direct base `Parent`:
//! - `upcast_<t>(&T) -> &Parent`
//! - `downcast_<t>(&NiObject) -> *const T`. Null unless the object is a `T` or a subclass of `T`.

#[cxx::bridge]
pub mod ffi {
    /// A `HavokFilter`.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct HavokFilterFfi {
        layer: u8,
        flags_and_parts: u8,
        group: u16,
    }

    /// An `hkWorldObjCInfoProperty`.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct CInfoPropertyFfi {
        data: u32,
        size: u32,
        capacity_and_flags: u32,
    }

    /// An `hkSubPartData`.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct SubPartFfi {
        filter: HavokFilterFfi,
        vertex_count: u32,
        material: u32,
    }

    /// An `hkTriangleNormalData`; the normal is zero when read from an `hkTriangleData`.
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct PackedTriangleFfi {
        p1: u16,
        p2: u16,
        p3: u16,
        welding_info: u16,
        normal: Vec3,
    }

    /// A `bhkCMSDMaterial`.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct CmsMaterialFfi {
        material: u32,
        filter: HavokFilterFfi,
    }

    /// A `bhkCMSDTransform`; the rotation is x, y, z, w.
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct CmsTransformFfi {
        translation: [f32; 4],
        rotation: [f32; 4],
    }

    /// A `bhkCMSDBigTris`.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct CmsBigTriangleFfi {
        p1: u16,
        p2: u16,
        p3: u16,
        material: u32,
        welding_info: u16,
    }

    /// A `bhkCMSDChunk` without its lists.
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct CmsChunkFfi {
        translation: [f32; 4],
        material_index: u32,
        reference: u16,
        transform_index: u16,
    }

    /// A `MotorDesc`: the type and the fields of that type's motor. Fields other types don't
    /// have are zero.
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct MotorFfi {
        motor_type: u8,
        min_force: f32,
        max_force: f32,
        tau: f32,
        damping: f32,
        proportional_recovery_velocity: f32,
        constant_recovery_velocity: f32,
        velocity_target: f32,
        use_velocity_target: bool,
        spring_constant: f32,
        spring_damping: f32,
        enabled: bool,
    }

    /// A constraint descriptor: up to 8 `Vector4`s (4 floats each) and 6 floats in the order
    /// of the descriptor's members, and its motor. `constraint_type` is the `hkConstraintType`
    /// of a wrapped constraint.
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct DescriptorFfi {
        constraint_type: u32,
        vectors: [f32; 32],
        floats: [f32; 6],
        motor: MotorFfi,
    }

    /// A `BoneMatrix`; the rotation is x, y, z, w.
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct BoneTransformFfi {
        translation: Vec3,
        rotation: [f32; 4],
        scale: Vec3,
    }

    /// A `ConstraintData` of `bhkRagdollTemplateData`, with its raw entity indices.
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct JointConstraintFfi {
        descriptor: DescriptorFfi,
        priority: u32,
        tau: f32,
        damping: f32,
        strength: f32,
        entity_a: u32,
        entity_b: u32,
    }

    #[namespace = "nifly"]
    unsafe extern "C++" {
        include!("bhk.hpp");

        type NiCollisionObject;
        type NiCollisionData;
        type bhkNiCollisionObject;
        type bhkCollisionObject;
        type bhkNPCollisionObject;
        type bhkPCollisionObject;
        type bhkSPCollisionObject;
        type bhkBlendCollisionObject;
        type bhkPhysicsSystem;
        type bhkRagdollSystem;
        type bhkRefObject;
        type bhkSerializable;
        type bhkShape;
        type bhkHeightFieldShape;
        type bhkPlaneShape;
        type bhkSphereRepShape;
        type bhkConvexShape;
        type bhkMultiSphereShape;
        type bhkConvexListShape;
        type bhkConvexVerticesShape;
        type bhkBoxShape;
        type bhkSphereShape;
        type bhkCylinderShape;
        type bhkTransformShape;
        type bhkConvexTransformShape;
        type bhkCapsuleShape;
        type bhkBvTreeShape;
        type bhkMoppBvTreeShape;
        type bhkNiTriStripsShape;
        type bhkShapeCollection;
        type bhkListShape;
        type hkPackedNiTriStripsData;
        type bhkPackedNiTriStripsShape;
        type bhkLiquidAction;
        type bhkOrientHingedBodyAction;
        type bhkWorldObject;
        type bhkPhantom;
        type bhkShapePhantom;
        type bhkSimpleShapePhantom;
        type bhkAabbPhantom;
        type bhkEntity;
        type bhkRigidBody;
        type bhkRigidBodyT;
        type bhkConstraint;
        type bhkHingeConstraint;
        type bhkLimitedHingeConstraint;
        type bhkBreakableConstraint;
        type bhkRagdollConstraint;
        type bhkStiffSpringConstraint;
        type bhkPrismaticConstraint;
        type bhkMalleableConstraint;
        type bhkBallAndSocketConstraint;
        type bhkBallSocketConstraintChain;
        type bhkCompressedMeshShapeData;
        type bhkCompressedMeshShape;
        type bhkPoseArray;
        type bhkRagdollTemplate;
        type bhkRagdollTemplateData;
    }

    #[namespace = "nifly"]
    extern "C++" {
        include!("NifFile.hpp");

        type NiObject = crate::objects::ffi::NiObject;
        type NiExtraData = crate::extra_data::ffi::NiExtraData;
        type BSExtraData = crate::extra_data::ffi::BSExtraData;
        type NifFile = crate::nif_file::ffi::NifFile;
    }

    extern "C++" {
        include!("nifly-sys/src/object_3d.rs.h");

        type BoundingVolumeFfi = crate::object_3d::ffi::BoundingVolumeFfi;
        type Vec3 = crate::object_3d::ffi::Vec3;
        type BoundingSphere = crate::object_3d::ffi::BoundingSphere;
    }

    #[namespace = "nifly_rs"]
    unsafe extern "C++" {
        include!("collision_shim.h");

        fn upcast_ni_collision_object(obj: &NiCollisionObject) -> &NiObject;
        fn downcast_ni_collision_object(obj: &NiObject) -> *const NiCollisionObject;
        fn upcast_ni_collision_data(obj: &NiCollisionData) -> &NiCollisionObject;
        fn downcast_ni_collision_data(obj: &NiObject) -> *const NiCollisionData;
        fn upcast_bhk_ni_collision_object(obj: &bhkNiCollisionObject) -> &NiCollisionObject;
        fn downcast_bhk_ni_collision_object(obj: &NiObject) -> *const bhkNiCollisionObject;
        fn upcast_bhk_collision_object(obj: &bhkCollisionObject) -> &bhkNiCollisionObject;
        fn downcast_bhk_collision_object(obj: &NiObject) -> *const bhkCollisionObject;
        fn upcast_bhk_np_collision_object(obj: &bhkNPCollisionObject) -> &bhkCollisionObject;
        fn downcast_bhk_np_collision_object(obj: &NiObject) -> *const bhkNPCollisionObject;
        fn upcast_bhk_p_collision_object(obj: &bhkPCollisionObject) -> &bhkNiCollisionObject;
        fn downcast_bhk_p_collision_object(obj: &NiObject) -> *const bhkPCollisionObject;
        fn upcast_bhk_sp_collision_object(obj: &bhkSPCollisionObject) -> &bhkPCollisionObject;
        fn downcast_bhk_sp_collision_object(obj: &NiObject) -> *const bhkSPCollisionObject;
        fn upcast_bhk_blend_collision_object(obj: &bhkBlendCollisionObject) -> &bhkCollisionObject;
        fn downcast_bhk_blend_collision_object(obj: &NiObject) -> *const bhkBlendCollisionObject;
        fn upcast_bhk_physics_system(obj: &bhkPhysicsSystem) -> &BSExtraData;
        fn downcast_bhk_physics_system(obj: &NiObject) -> *const bhkPhysicsSystem;
        fn upcast_bhk_ragdoll_system(obj: &bhkRagdollSystem) -> &BSExtraData;
        fn downcast_bhk_ragdoll_system(obj: &NiObject) -> *const bhkRagdollSystem;
        fn upcast_bhk_ref_object(obj: &bhkRefObject) -> &NiObject;
        fn downcast_bhk_ref_object(obj: &NiObject) -> *const bhkRefObject;
        fn upcast_bhk_serializable(obj: &bhkSerializable) -> &bhkRefObject;
        fn downcast_bhk_serializable(obj: &NiObject) -> *const bhkSerializable;
        fn upcast_bhk_shape(obj: &bhkShape) -> &bhkSerializable;
        fn downcast_bhk_shape(obj: &NiObject) -> *const bhkShape;
        fn upcast_bhk_height_field_shape(obj: &bhkHeightFieldShape) -> &bhkShape;
        fn downcast_bhk_height_field_shape(obj: &NiObject) -> *const bhkHeightFieldShape;
        fn upcast_bhk_plane_shape(obj: &bhkPlaneShape) -> &bhkHeightFieldShape;
        fn downcast_bhk_plane_shape(obj: &NiObject) -> *const bhkPlaneShape;
        fn upcast_bhk_sphere_rep_shape(obj: &bhkSphereRepShape) -> &bhkShape;
        fn downcast_bhk_sphere_rep_shape(obj: &NiObject) -> *const bhkSphereRepShape;
        fn upcast_bhk_convex_shape(obj: &bhkConvexShape) -> &bhkSphereRepShape;
        fn downcast_bhk_convex_shape(obj: &NiObject) -> *const bhkConvexShape;
        fn upcast_bhk_multi_sphere_shape(obj: &bhkMultiSphereShape) -> &bhkSphereRepShape;
        fn downcast_bhk_multi_sphere_shape(obj: &NiObject) -> *const bhkMultiSphereShape;
        fn upcast_bhk_convex_list_shape(obj: &bhkConvexListShape) -> &bhkShape;
        fn downcast_bhk_convex_list_shape(obj: &NiObject) -> *const bhkConvexListShape;
        fn upcast_bhk_convex_vertices_shape(obj: &bhkConvexVerticesShape) -> &bhkConvexShape;
        fn downcast_bhk_convex_vertices_shape(obj: &NiObject) -> *const bhkConvexVerticesShape;
        fn upcast_bhk_box_shape(obj: &bhkBoxShape) -> &bhkConvexShape;
        fn downcast_bhk_box_shape(obj: &NiObject) -> *const bhkBoxShape;
        fn upcast_bhk_sphere_shape(obj: &bhkSphereShape) -> &bhkConvexShape;
        fn downcast_bhk_sphere_shape(obj: &NiObject) -> *const bhkSphereShape;
        fn upcast_bhk_cylinder_shape(obj: &bhkCylinderShape) -> &bhkConvexShape;
        fn downcast_bhk_cylinder_shape(obj: &NiObject) -> *const bhkCylinderShape;
        fn upcast_bhk_transform_shape(obj: &bhkTransformShape) -> &bhkShape;
        fn downcast_bhk_transform_shape(obj: &NiObject) -> *const bhkTransformShape;
        fn upcast_bhk_convex_transform_shape(obj: &bhkConvexTransformShape) -> &bhkTransformShape;
        fn downcast_bhk_convex_transform_shape(obj: &NiObject) -> *const bhkConvexTransformShape;
        fn upcast_bhk_capsule_shape(obj: &bhkCapsuleShape) -> &bhkConvexShape;
        fn downcast_bhk_capsule_shape(obj: &NiObject) -> *const bhkCapsuleShape;
        fn upcast_bhk_bv_tree_shape(obj: &bhkBvTreeShape) -> &bhkShape;
        fn downcast_bhk_bv_tree_shape(obj: &NiObject) -> *const bhkBvTreeShape;
        fn upcast_bhk_mopp_bv_tree_shape(obj: &bhkMoppBvTreeShape) -> &bhkBvTreeShape;
        fn downcast_bhk_mopp_bv_tree_shape(obj: &NiObject) -> *const bhkMoppBvTreeShape;
        fn upcast_bhk_ni_tri_strips_shape(obj: &bhkNiTriStripsShape) -> &bhkShape;
        fn downcast_bhk_ni_tri_strips_shape(obj: &NiObject) -> *const bhkNiTriStripsShape;
        fn upcast_bhk_shape_collection(obj: &bhkShapeCollection) -> &bhkShape;
        fn downcast_bhk_shape_collection(obj: &NiObject) -> *const bhkShapeCollection;
        fn upcast_bhk_list_shape(obj: &bhkListShape) -> &bhkShapeCollection;
        fn downcast_bhk_list_shape(obj: &NiObject) -> *const bhkListShape;
        fn upcast_hk_packed_ni_tri_strips_data(obj: &hkPackedNiTriStripsData) -> &bhkShapeCollection;
        fn downcast_hk_packed_ni_tri_strips_data(obj: &NiObject) -> *const hkPackedNiTriStripsData;
        fn upcast_bhk_packed_ni_tri_strips_shape(obj: &bhkPackedNiTriStripsShape) -> &bhkShapeCollection;
        fn downcast_bhk_packed_ni_tri_strips_shape(obj: &NiObject) -> *const bhkPackedNiTriStripsShape;
        fn upcast_bhk_liquid_action(obj: &bhkLiquidAction) -> &bhkSerializable;
        fn downcast_bhk_liquid_action(obj: &NiObject) -> *const bhkLiquidAction;
        fn upcast_bhk_orient_hinged_body_action(obj: &bhkOrientHingedBodyAction) -> &bhkSerializable;
        fn downcast_bhk_orient_hinged_body_action(obj: &NiObject) -> *const bhkOrientHingedBodyAction;
        fn upcast_bhk_world_object(obj: &bhkWorldObject) -> &bhkSerializable;
        fn downcast_bhk_world_object(obj: &NiObject) -> *const bhkWorldObject;
        fn upcast_bhk_phantom(obj: &bhkPhantom) -> &bhkWorldObject;
        fn downcast_bhk_phantom(obj: &NiObject) -> *const bhkPhantom;
        fn upcast_bhk_shape_phantom(obj: &bhkShapePhantom) -> &bhkPhantom;
        fn downcast_bhk_shape_phantom(obj: &NiObject) -> *const bhkShapePhantom;
        fn upcast_bhk_simple_shape_phantom(obj: &bhkSimpleShapePhantom) -> &bhkShapePhantom;
        fn downcast_bhk_simple_shape_phantom(obj: &NiObject) -> *const bhkSimpleShapePhantom;
        fn upcast_bhk_aabb_phantom(obj: &bhkAabbPhantom) -> &bhkShapePhantom;
        fn downcast_bhk_aabb_phantom(obj: &NiObject) -> *const bhkAabbPhantom;
        fn upcast_bhk_entity(obj: &bhkEntity) -> &bhkWorldObject;
        fn downcast_bhk_entity(obj: &NiObject) -> *const bhkEntity;
        fn upcast_bhk_rigid_body(obj: &bhkRigidBody) -> &bhkEntity;
        fn downcast_bhk_rigid_body(obj: &NiObject) -> *const bhkRigidBody;
        fn upcast_bhk_rigid_body_t(obj: &bhkRigidBodyT) -> &bhkRigidBody;
        fn downcast_bhk_rigid_body_t(obj: &NiObject) -> *const bhkRigidBodyT;
        fn upcast_bhk_constraint(obj: &bhkConstraint) -> &bhkSerializable;
        fn downcast_bhk_constraint(obj: &NiObject) -> *const bhkConstraint;
        fn upcast_bhk_hinge_constraint(obj: &bhkHingeConstraint) -> &bhkConstraint;
        fn downcast_bhk_hinge_constraint(obj: &NiObject) -> *const bhkHingeConstraint;
        fn upcast_bhk_limited_hinge_constraint(obj: &bhkLimitedHingeConstraint) -> &bhkConstraint;
        fn downcast_bhk_limited_hinge_constraint(obj: &NiObject) -> *const bhkLimitedHingeConstraint;
        fn upcast_bhk_breakable_constraint(obj: &bhkBreakableConstraint) -> &bhkConstraint;
        fn downcast_bhk_breakable_constraint(obj: &NiObject) -> *const bhkBreakableConstraint;
        fn upcast_bhk_ragdoll_constraint(obj: &bhkRagdollConstraint) -> &bhkConstraint;
        fn downcast_bhk_ragdoll_constraint(obj: &NiObject) -> *const bhkRagdollConstraint;
        fn upcast_bhk_stiff_spring_constraint(obj: &bhkStiffSpringConstraint) -> &bhkConstraint;
        fn downcast_bhk_stiff_spring_constraint(obj: &NiObject) -> *const bhkStiffSpringConstraint;
        fn upcast_bhk_prismatic_constraint(obj: &bhkPrismaticConstraint) -> &bhkConstraint;
        fn downcast_bhk_prismatic_constraint(obj: &NiObject) -> *const bhkPrismaticConstraint;
        fn upcast_bhk_malleable_constraint(obj: &bhkMalleableConstraint) -> &bhkConstraint;
        fn downcast_bhk_malleable_constraint(obj: &NiObject) -> *const bhkMalleableConstraint;
        fn upcast_bhk_ball_and_socket_constraint(obj: &bhkBallAndSocketConstraint) -> &bhkConstraint;
        fn downcast_bhk_ball_and_socket_constraint(obj: &NiObject) -> *const bhkBallAndSocketConstraint;
        fn upcast_bhk_ball_socket_constraint_chain(obj: &bhkBallSocketConstraintChain) -> &bhkSerializable;
        fn downcast_bhk_ball_socket_constraint_chain(obj: &NiObject) -> *const bhkBallSocketConstraintChain;
        fn upcast_bhk_compressed_mesh_shape_data(obj: &bhkCompressedMeshShapeData) -> &bhkRefObject;
        fn downcast_bhk_compressed_mesh_shape_data(obj: &NiObject) -> *const bhkCompressedMeshShapeData;
        fn upcast_bhk_compressed_mesh_shape(obj: &bhkCompressedMeshShape) -> &bhkShape;
        fn downcast_bhk_compressed_mesh_shape(obj: &NiObject) -> *const bhkCompressedMeshShape;
        fn upcast_bhk_pose_array(obj: &bhkPoseArray) -> &NiObject;
        fn downcast_bhk_pose_array(obj: &NiObject) -> *const bhkPoseArray;
        fn upcast_bhk_ragdoll_template(obj: &bhkRagdollTemplate) -> &NiExtraData;
        fn downcast_bhk_ragdoll_template(obj: &NiObject) -> *const bhkRagdollTemplate;
        fn upcast_bhk_ragdoll_template_data(obj: &bhkRagdollTemplateData) -> &NiObject;
        fn downcast_bhk_ragdoll_template_data(obj: &NiObject) -> *const bhkRagdollTemplateData;

        /// `NiCollisionData` fields. The enums are raw values. The bounding volume is empty
        /// unless `useABV` is set; setting no records clears it, and the setter returns false,
        /// changing nothing, unless the records are one volume.
        fn collision_data_propagation_mode(d: &NiCollisionData) -> u32;
        fn set_collision_data_propagation_mode(d: Pin<&mut NiCollisionData>, v: u32);
        fn collision_data_collision_mode(d: &NiCollisionData) -> u32;
        fn set_collision_data_collision_mode(d: Pin<&mut NiCollisionData>, v: u32);
        fn collision_data_bounding_volume(d: &NiCollisionData) -> Vec<BoundingVolumeFfi>;
        fn set_collision_data_bounding_volume(d: Pin<&mut NiCollisionData>, records: &[BoundingVolumeFfi]) -> bool;

        /// `bhkNiCollisionObject::flags`, `bhkNPCollisionObject::bodyID` and the
        /// `bhkBlendCollisionObject` gains.
        fn bhk_collision_flags(c: &bhkNiCollisionObject) -> u16;
        fn set_bhk_collision_flags(c: Pin<&mut bhkNiCollisionObject>, v: u16);
        fn np_collision_body_id(c: &bhkNPCollisionObject) -> u32;
        fn set_np_collision_body_id(c: Pin<&mut bhkNPCollisionObject>, v: u32);
        fn blend_collision_heir_gain(c: &bhkBlendCollisionObject) -> f32;
        fn set_blend_collision_heir_gain(c: Pin<&mut bhkBlendCollisionObject>, v: f32);
        fn blend_collision_vel_gain(c: &bhkBlendCollisionObject) -> f32;
        fn set_blend_collision_vel_gain(c: Pin<&mut bhkBlendCollisionObject>, v: f32);

        /// The `data` blobs of `bhkPhysicsSystem` and `bhkRagdollSystem`.
        fn physics_system_data(s: &bhkPhysicsSystem) -> &[u8];
        fn set_physics_system_data(s: Pin<&mut bhkPhysicsSystem>, bytes: &[u8]);
        fn ragdoll_system_data(s: &bhkRagdollSystem) -> &[u8];
        fn set_ragdoll_system_data(s: Pin<&mut bhkRagdollSystem>, bytes: &[u8]);

        /// Adds a new collision object of the named type to the `NiAVObject` at `object`, as
        /// its collision object and targeting it, and returns its index. `NIF_NPOS`, changing
        /// nothing, if `object` isn't an `NiAVObject`.
        fn assign_ni_collision_data(file: Pin<&mut NifFile>, object: u32) -> u32;
        fn assign_bhk_collision_object(file: Pin<&mut NifFile>, object: u32) -> u32;
        fn assign_bhk_np_collision_object(file: Pin<&mut NifFile>, object: u32) -> u32;
        fn assign_bhk_p_collision_object(file: Pin<&mut NifFile>, object: u32) -> u32;
        fn assign_bhk_sp_collision_object(file: Pin<&mut NifFile>, object: u32) -> u32;
        fn assign_bhk_blend_collision_object(file: Pin<&mut NifFile>, object: u32) -> u32;

        /// Adds a new body of the named type to the `bhkNiCollisionObject` at `collision` and
        /// returns its index. `NIF_NPOS`, changing nothing, unless the type fits: a physics or
        /// ragdoll system for a `bhkNPCollisionObject`, a world object for the others.
        fn assign_bhk_rigid_body(file: Pin<&mut NifFile>, collision: u32) -> u32;
        fn assign_bhk_rigid_body_t(file: Pin<&mut NifFile>, collision: u32) -> u32;
        fn assign_bhk_simple_shape_phantom(file: Pin<&mut NifFile>, collision: u32) -> u32;
        fn assign_bhk_aabb_phantom(file: Pin<&mut NifFile>, collision: u32) -> u32;
        fn assign_bhk_physics_system(file: Pin<&mut NifFile>, collision: u32) -> u32;
        fn assign_bhk_ragdoll_system(file: Pin<&mut NifFile>, collision: u32) -> u32;

        /// Fields of world objects, rigid bodies, phantoms and actions; the field names are in
        /// `collision_shim.h`. Vectors are `Vector4`s, rotations `QuaternionXYZW`s (x, y, z, w)
        /// and the phantom transform a `Matrix4`'s 16 values. The response type is raw.
        fn world_object_unknown_int(o: &bhkWorldObject) -> i32;
        fn set_world_object_unknown_int(o: Pin<&mut bhkWorldObject>, v: i32);
        fn world_object_broad_phase_type(o: &bhkWorldObject) -> u8;
        fn set_world_object_broad_phase_type(o: Pin<&mut bhkWorldObject>, v: u8);
        fn rigid_body_callback_delay(o: &bhkRigidBody) -> u16;
        fn set_rigid_body_callback_delay(o: Pin<&mut bhkRigidBody>, v: u16);
        fn rigid_body_unknown_int(o: &bhkRigidBody) -> u32;
        fn set_rigid_body_unknown_int(o: Pin<&mut bhkRigidBody>, v: u32);
        fn rigid_body_mass(o: &bhkRigidBody) -> f32;
        fn set_rigid_body_mass(o: Pin<&mut bhkRigidBody>, v: f32);
        fn rigid_body_linear_damping(o: &bhkRigidBody) -> f32;
        fn set_rigid_body_linear_damping(o: Pin<&mut bhkRigidBody>, v: f32);
        fn rigid_body_angular_damping(o: &bhkRigidBody) -> f32;
        fn set_rigid_body_angular_damping(o: Pin<&mut bhkRigidBody>, v: f32);
        fn rigid_body_time_factor(o: &bhkRigidBody) -> f32;
        fn set_rigid_body_time_factor(o: Pin<&mut bhkRigidBody>, v: f32);
        fn rigid_body_gravity_factor(o: &bhkRigidBody) -> f32;
        fn set_rigid_body_gravity_factor(o: Pin<&mut bhkRigidBody>, v: f32);
        fn rigid_body_friction(o: &bhkRigidBody) -> f32;
        fn set_rigid_body_friction(o: Pin<&mut bhkRigidBody>, v: f32);
        fn rigid_body_rolling_friction(o: &bhkRigidBody) -> f32;
        fn set_rigid_body_rolling_friction(o: Pin<&mut bhkRigidBody>, v: f32);
        fn rigid_body_restitution(o: &bhkRigidBody) -> f32;
        fn set_rigid_body_restitution(o: Pin<&mut bhkRigidBody>, v: f32);
        fn rigid_body_max_linear_velocity(o: &bhkRigidBody) -> f32;
        fn set_rigid_body_max_linear_velocity(o: Pin<&mut bhkRigidBody>, v: f32);
        fn rigid_body_max_angular_velocity(o: &bhkRigidBody) -> f32;
        fn set_rigid_body_max_angular_velocity(o: Pin<&mut bhkRigidBody>, v: f32);
        fn rigid_body_penetration_depth(o: &bhkRigidBody) -> f32;
        fn set_rigid_body_penetration_depth(o: Pin<&mut bhkRigidBody>, v: f32);
        fn rigid_body_motion_system(o: &bhkRigidBody) -> u8;
        fn set_rigid_body_motion_system(o: Pin<&mut bhkRigidBody>, v: u8);
        fn rigid_body_deactivator_type(o: &bhkRigidBody) -> u8;
        fn set_rigid_body_deactivator_type(o: Pin<&mut bhkRigidBody>, v: u8);
        fn rigid_body_solver_deactivation(o: &bhkRigidBody) -> u8;
        fn set_rigid_body_solver_deactivation(o: Pin<&mut bhkRigidBody>, v: u8);
        fn rigid_body_quality_type(o: &bhkRigidBody) -> u8;
        fn set_rigid_body_quality_type(o: Pin<&mut bhkRigidBody>, v: u8);
        fn rigid_body_auto_remove_level(o: &bhkRigidBody) -> u8;
        fn set_rigid_body_auto_remove_level(o: Pin<&mut bhkRigidBody>, v: u8);
        fn rigid_body_response_modifier_flags(o: &bhkRigidBody) -> u8;
        fn set_rigid_body_response_modifier_flags(o: Pin<&mut bhkRigidBody>, v: u8);
        fn rigid_body_shape_keys(o: &bhkRigidBody) -> u8;
        fn set_rigid_body_shape_keys(o: Pin<&mut bhkRigidBody>, v: u8);
        fn rigid_body_force_collide_onto_ppu(o: &bhkRigidBody) -> bool;
        fn set_rigid_body_force_collide_onto_ppu(o: Pin<&mut bhkRigidBody>, v: bool);
        fn rigid_body_body_flags_int(o: &bhkRigidBody) -> u32;
        fn set_rigid_body_body_flags_int(o: Pin<&mut bhkRigidBody>, v: u32);
        fn rigid_body_body_flags(o: &bhkRigidBody) -> u16;
        fn set_rigid_body_body_flags(o: Pin<&mut bhkRigidBody>, v: u16);
        fn liquid_user_data(o: &bhkLiquidAction) -> u32;
        fn set_liquid_user_data(o: Pin<&mut bhkLiquidAction>, v: u32);
        fn liquid_unknown_int_1(o: &bhkLiquidAction) -> u32;
        fn set_liquid_unknown_int_1(o: Pin<&mut bhkLiquidAction>, v: u32);
        fn liquid_unknown_int_2(o: &bhkLiquidAction) -> u32;
        fn set_liquid_unknown_int_2(o: Pin<&mut bhkLiquidAction>, v: u32);
        fn liquid_initial_stick_force(o: &bhkLiquidAction) -> f32;
        fn set_liquid_initial_stick_force(o: Pin<&mut bhkLiquidAction>, v: f32);
        fn liquid_stick_strength(o: &bhkLiquidAction) -> f32;
        fn set_liquid_stick_strength(o: Pin<&mut bhkLiquidAction>, v: f32);
        fn liquid_neighbor_distance(o: &bhkLiquidAction) -> f32;
        fn set_liquid_neighbor_distance(o: Pin<&mut bhkLiquidAction>, v: f32);
        fn liquid_neighbor_strength(o: &bhkLiquidAction) -> f32;
        fn set_liquid_neighbor_strength(o: Pin<&mut bhkLiquidAction>, v: f32);
        fn hinged_action_unknown_int_1(o: &bhkOrientHingedBodyAction) -> u32;
        fn set_hinged_action_unknown_int_1(o: Pin<&mut bhkOrientHingedBodyAction>, v: u32);
        fn hinged_action_unknown_int_2(o: &bhkOrientHingedBodyAction) -> u32;
        fn set_hinged_action_unknown_int_2(o: Pin<&mut bhkOrientHingedBodyAction>, v: u32);
        fn hinged_action_strength(o: &bhkOrientHingedBodyAction) -> f32;
        fn set_hinged_action_strength(o: Pin<&mut bhkOrientHingedBodyAction>, v: f32);
        fn hinged_action_damping(o: &bhkOrientHingedBodyAction) -> f32;
        fn set_hinged_action_damping(o: Pin<&mut bhkOrientHingedBodyAction>, v: f32);
        fn rigid_body_translation(o: &bhkRigidBody) -> [f32; 4];
        fn set_rigid_body_translation(o: Pin<&mut bhkRigidBody>, v: [f32; 4]);
        fn rigid_body_linear_velocity(o: &bhkRigidBody) -> [f32; 4];
        fn set_rigid_body_linear_velocity(o: Pin<&mut bhkRigidBody>, v: [f32; 4]);
        fn rigid_body_angular_velocity(o: &bhkRigidBody) -> [f32; 4];
        fn set_rigid_body_angular_velocity(o: Pin<&mut bhkRigidBody>, v: [f32; 4]);
        fn rigid_body_center(o: &bhkRigidBody) -> [f32; 4];
        fn set_rigid_body_center(o: Pin<&mut bhkRigidBody>, v: [f32; 4]);
        fn aabb_phantom_min(o: &bhkAabbPhantom) -> [f32; 4];
        fn set_aabb_phantom_min(o: Pin<&mut bhkAabbPhantom>, v: [f32; 4]);
        fn aabb_phantom_max(o: &bhkAabbPhantom) -> [f32; 4];
        fn set_aabb_phantom_max(o: Pin<&mut bhkAabbPhantom>, v: [f32; 4]);
        fn hinged_action_hinge_axis(o: &bhkOrientHingedBodyAction) -> [f32; 4];
        fn set_hinged_action_hinge_axis(o: Pin<&mut bhkOrientHingedBodyAction>, v: [f32; 4]);
        fn hinged_action_forward(o: &bhkOrientHingedBodyAction) -> [f32; 4];
        fn set_hinged_action_forward(o: Pin<&mut bhkOrientHingedBodyAction>, v: [f32; 4]);
        fn world_object_unknown_bytes(o: &bhkWorldObject) -> [u8; 3];
        fn set_world_object_unknown_bytes(o: Pin<&mut bhkWorldObject>, v: [u8; 3]);
        fn rigid_body_unknown_shorts(o: &bhkRigidBody) -> [u16; 6];
        fn set_rigid_body_unknown_shorts(o: Pin<&mut bhkRigidBody>, v: [u16; 6]);
        fn rigid_body_inertia(o: &bhkRigidBody) -> [f32; 12];
        fn set_rigid_body_inertia(o: Pin<&mut bhkRigidBody>, v: [f32; 12]);
        fn world_object_filter(o: &bhkWorldObject) -> HavokFilterFfi;
        fn set_world_object_filter(o: Pin<&mut bhkWorldObject>, v: HavokFilterFfi);
        fn world_object_property(o: &bhkWorldObject) -> CInfoPropertyFfi;
        fn set_world_object_property(o: Pin<&mut bhkWorldObject>, v: CInfoPropertyFfi);
        fn rigid_body_filter_copy(o: &bhkRigidBody) -> HavokFilterFfi;
        fn set_rigid_body_filter_copy(o: Pin<&mut bhkRigidBody>, v: HavokFilterFfi);
        fn rigid_body_rotation(o: &bhkRigidBody) -> [f32; 4];
        fn set_rigid_body_rotation(o: Pin<&mut bhkRigidBody>, v: [f32; 4]);
        fn simple_phantom_transform(o: &bhkSimpleShapePhantom) -> [f32; 16];
        fn set_simple_phantom_transform(o: Pin<&mut bhkSimpleShapePhantom>, v: [f32; 16]);
        fn rigid_body_collision_response(o: &bhkRigidBody) -> u8;
        fn set_rigid_body_collision_response(o: Pin<&mut bhkRigidBody>, v: u8);

        /// Adds a new, unattached action and returns its index.
        fn add_bhk_liquid_action(file: Pin<&mut NifFile>) -> u32;
        fn add_bhk_orient_hinged_body_action(file: Pin<&mut NifFile>) -> u32;

        /// Shape fields; the field names are in `collision_shim.h`. Materials go through
        /// `GetMaterial`/`SetMaterial` where the class overrides them. Vector3s are `Vec3`s,
        /// Vector4s `[f32; 4]` and the transform a `Matrix4`'s 16 values.
        fn height_field_material(o: &bhkHeightFieldShape) -> u32;
        fn set_height_field_material(o: Pin<&mut bhkHeightFieldShape>, v: u32);
        fn sphere_rep_material(o: &bhkSphereRepShape) -> u32;
        fn set_sphere_rep_material(o: Pin<&mut bhkSphereRepShape>, v: u32);
        fn strips_shape_material(o: &bhkNiTriStripsShape) -> u32;
        fn set_strips_shape_material(o: Pin<&mut bhkNiTriStripsShape>, v: u32);
        fn list_shape_material(o: &bhkListShape) -> u32;
        fn set_list_shape_material(o: Pin<&mut bhkListShape>, v: u32);
        fn transform_shape_material(o: &bhkTransformShape) -> u32;
        fn set_transform_shape_material(o: Pin<&mut bhkTransformShape>, v: u32);
        fn convex_list_material(o: &bhkConvexListShape) -> u32;
        fn set_convex_list_material(o: Pin<&mut bhkConvexListShape>, v: u32);
        fn convex_list_unknown_int(o: &bhkConvexListShape) -> u32;
        fn set_convex_list_unknown_int(o: Pin<&mut bhkConvexListShape>, v: u32);
        fn strips_shape_grow_by(o: &bhkNiTriStripsShape) -> u32;
        fn set_strips_shape_grow_by(o: Pin<&mut bhkNiTriStripsShape>, v: u32);
        fn packed_shape_user_data(o: &bhkPackedNiTriStripsShape) -> u32;
        fn set_packed_shape_user_data(o: Pin<&mut bhkPackedNiTriStripsShape>, v: u32);
        fn compressed_shape_user_data(o: &bhkCompressedMeshShape) -> u32;
        fn set_compressed_shape_user_data(o: Pin<&mut bhkCompressedMeshShape>, v: u32);
        fn cms_bits_per_index(o: &bhkCompressedMeshShapeData) -> u32;
        fn set_cms_bits_per_index(o: Pin<&mut bhkCompressedMeshShapeData>, v: u32);
        fn cms_bits_per_welding_index(o: &bhkCompressedMeshShapeData) -> u32;
        fn set_cms_bits_per_welding_index(o: Pin<&mut bhkCompressedMeshShapeData>, v: u32);
        fn cms_mask_welding_index(o: &bhkCompressedMeshShapeData) -> u32;
        fn set_cms_mask_welding_index(o: Pin<&mut bhkCompressedMeshShapeData>, v: u32);
        fn cms_mask_index(o: &bhkCompressedMeshShapeData) -> u32;
        fn set_cms_mask_index(o: Pin<&mut bhkCompressedMeshShapeData>, v: u32);
        fn cms_named_material_count(o: &bhkCompressedMeshShapeData) -> u32;
        fn set_cms_named_material_count(o: Pin<&mut bhkCompressedMeshShapeData>, v: u32);
        fn cms_convex_piece_count(o: &bhkCompressedMeshShapeData) -> u32;
        fn set_cms_convex_piece_count(o: Pin<&mut bhkCompressedMeshShapeData>, v: u32);
        fn plane_shape_constant(o: &bhkPlaneShape) -> f32;
        fn set_plane_shape_constant(o: Pin<&mut bhkPlaneShape>, v: f32);
        fn convex_shape_radius(o: &bhkConvexShape) -> f32;
        fn set_convex_shape_radius(o: Pin<&mut bhkConvexShape>, v: f32);
        fn cylinder_shape_radius(o: &bhkCylinderShape) -> f32;
        fn set_cylinder_shape_radius(o: Pin<&mut bhkCylinderShape>, v: f32);
        fn capsule_shape_radius_1(o: &bhkCapsuleShape) -> f32;
        fn set_capsule_shape_radius_1(o: Pin<&mut bhkCapsuleShape>, v: f32);
        fn capsule_shape_radius_2(o: &bhkCapsuleShape) -> f32;
        fn set_capsule_shape_radius_2(o: Pin<&mut bhkCapsuleShape>, v: f32);
        fn transform_shape_radius(o: &bhkTransformShape) -> f32;
        fn set_transform_shape_radius(o: Pin<&mut bhkTransformShape>, v: f32);
        fn convex_list_radius(o: &bhkConvexListShape) -> f32;
        fn set_convex_list_radius(o: Pin<&mut bhkConvexListShape>, v: f32);
        fn convex_list_unknown_float(o: &bhkConvexListShape) -> f32;
        fn set_convex_list_unknown_float(o: Pin<&mut bhkConvexListShape>, v: f32);
        fn convex_list_closest_point_min_distance(o: &bhkConvexListShape) -> f32;
        fn set_convex_list_closest_point_min_distance(o: Pin<&mut bhkConvexListShape>, v: f32);
        fn mopp_scale(o: &bhkMoppBvTreeShape) -> f32;
        fn set_mopp_scale(o: Pin<&mut bhkMoppBvTreeShape>, v: f32);
        fn strips_shape_radius(o: &bhkNiTriStripsShape) -> f32;
        fn set_strips_shape_radius(o: Pin<&mut bhkNiTriStripsShape>, v: f32);
        fn packed_shape_radius(o: &bhkPackedNiTriStripsShape) -> f32;
        fn set_packed_shape_radius(o: Pin<&mut bhkPackedNiTriStripsShape>, v: f32);
        fn packed_shape_radius_copy(o: &bhkPackedNiTriStripsShape) -> f32;
        fn set_packed_shape_radius_copy(o: Pin<&mut bhkPackedNiTriStripsShape>, v: f32);
        fn compressed_shape_radius(o: &bhkCompressedMeshShape) -> f32;
        fn set_compressed_shape_radius(o: Pin<&mut bhkCompressedMeshShape>, v: f32);
        fn compressed_shape_unknown_float(o: &bhkCompressedMeshShape) -> f32;
        fn set_compressed_shape_unknown_float(o: Pin<&mut bhkCompressedMeshShape>, v: f32);
        fn compressed_shape_radius_copy(o: &bhkCompressedMeshShape) -> f32;
        fn set_compressed_shape_radius_copy(o: Pin<&mut bhkCompressedMeshShape>, v: f32);
        fn cms_error(o: &bhkCompressedMeshShapeData) -> f32;
        fn set_cms_error(o: Pin<&mut bhkCompressedMeshShapeData>, v: f32);
        fn mopp_build_type(o: &bhkMoppBvTreeShape) -> u8;
        fn set_mopp_build_type(o: Pin<&mut bhkMoppBvTreeShape>, v: u8);
        fn cms_welding_type(o: &bhkCompressedMeshShapeData) -> u8;
        fn set_cms_welding_type(o: Pin<&mut bhkCompressedMeshShapeData>, v: u8);
        fn cms_material_type(o: &bhkCompressedMeshShapeData) -> u8;
        fn set_cms_material_type(o: Pin<&mut bhkCompressedMeshShapeData>, v: u8);
        fn convex_list_use_cached_aabb(o: &bhkConvexListShape) -> bool;
        fn set_convex_list_use_cached_aabb(o: Pin<&mut bhkConvexListShape>, v: bool);
        fn plane_shape_unknown_vector(o: &bhkPlaneShape) -> Vec3;
        fn set_plane_shape_unknown_vector(o: Pin<&mut bhkPlaneShape>, v: Vec3);
        fn plane_shape_normal(o: &bhkPlaneShape) -> Vec3;
        fn set_plane_shape_normal(o: Pin<&mut bhkPlaneShape>, v: Vec3);
        fn box_shape_dimensions(o: &bhkBoxShape) -> Vec3;
        fn set_box_shape_dimensions(o: Pin<&mut bhkBoxShape>, v: Vec3);
        fn capsule_shape_point_1(o: &bhkCapsuleShape) -> Vec3;
        fn set_capsule_shape_point_1(o: Pin<&mut bhkCapsuleShape>, v: Vec3);
        fn capsule_shape_point_2(o: &bhkCapsuleShape) -> Vec3;
        fn set_capsule_shape_point_2(o: Pin<&mut bhkCapsuleShape>, v: Vec3);
        fn plane_shape_half_extents(o: &bhkPlaneShape) -> [f32; 4];
        fn set_plane_shape_half_extents(o: Pin<&mut bhkPlaneShape>, v: [f32; 4]);
        fn plane_shape_center(o: &bhkPlaneShape) -> [f32; 4];
        fn set_plane_shape_center(o: Pin<&mut bhkPlaneShape>, v: [f32; 4]);
        fn cylinder_shape_vertex_a(o: &bhkCylinderShape) -> [f32; 4];
        fn set_cylinder_shape_vertex_a(o: Pin<&mut bhkCylinderShape>, v: [f32; 4]);
        fn cylinder_shape_vertex_b(o: &bhkCylinderShape) -> [f32; 4];
        fn set_cylinder_shape_vertex_b(o: Pin<&mut bhkCylinderShape>, v: [f32; 4]);
        fn mopp_offset(o: &bhkMoppBvTreeShape) -> [f32; 4];
        fn set_mopp_offset(o: Pin<&mut bhkMoppBvTreeShape>, v: [f32; 4]);
        fn strips_shape_scale(o: &bhkNiTriStripsShape) -> [f32; 4];
        fn set_strips_shape_scale(o: Pin<&mut bhkNiTriStripsShape>, v: [f32; 4]);
        fn packed_shape_scale(o: &bhkPackedNiTriStripsShape) -> [f32; 4];
        fn set_packed_shape_scale(o: Pin<&mut bhkPackedNiTriStripsShape>, v: [f32; 4]);
        fn packed_shape_scale_copy(o: &bhkPackedNiTriStripsShape) -> [f32; 4];
        fn set_packed_shape_scale_copy(o: Pin<&mut bhkPackedNiTriStripsShape>, v: [f32; 4]);
        fn compressed_shape_scale(o: &bhkCompressedMeshShape) -> [f32; 4];
        fn set_compressed_shape_scale(o: Pin<&mut bhkCompressedMeshShape>, v: [f32; 4]);
        fn compressed_shape_scale_copy(o: &bhkCompressedMeshShape) -> [f32; 4];
        fn set_compressed_shape_scale_copy(o: Pin<&mut bhkCompressedMeshShape>, v: [f32; 4]);
        fn cms_aabb_min(o: &bhkCompressedMeshShapeData) -> [f32; 4];
        fn set_cms_aabb_min(o: Pin<&mut bhkCompressedMeshShapeData>, v: [f32; 4]);
        fn cms_aabb_max(o: &bhkCompressedMeshShapeData) -> [f32; 4];
        fn set_cms_aabb_max(o: Pin<&mut bhkCompressedMeshShapeData>, v: [f32; 4]);
        fn multi_sphere_property(o: &bhkMultiSphereShape) -> CInfoPropertyFfi;
        fn set_multi_sphere_property(o: Pin<&mut bhkMultiSphereShape>, v: CInfoPropertyFfi);
        fn convex_vertices_vertices_property(o: &bhkConvexVerticesShape) -> CInfoPropertyFfi;
        fn set_convex_vertices_vertices_property(o: Pin<&mut bhkConvexVerticesShape>, v: CInfoPropertyFfi);
        fn convex_vertices_normals_property(o: &bhkConvexVerticesShape) -> CInfoPropertyFfi;
        fn set_convex_vertices_normals_property(o: Pin<&mut bhkConvexVerticesShape>, v: CInfoPropertyFfi);
        fn list_shape_child_shape_property(o: &bhkListShape) -> CInfoPropertyFfi;
        fn set_list_shape_child_shape_property(o: Pin<&mut bhkListShape>, v: CInfoPropertyFfi);
        fn list_shape_child_filter_property(o: &bhkListShape) -> CInfoPropertyFfi;
        fn set_list_shape_child_filter_property(o: Pin<&mut bhkListShape>, v: CInfoPropertyFfi);
        fn convex_list_child_shape_property(o: &bhkConvexListShape) -> CInfoPropertyFfi;
        fn set_convex_list_child_shape_property(o: Pin<&mut bhkConvexListShape>, v: CInfoPropertyFfi);
        fn transform_shape_transform(o: &bhkTransformShape) -> [f32; 16];
        fn set_transform_shape_transform(o: Pin<&mut bhkTransformShape>, v: [f32; 16]);

        /// The multi-sphere shape's spheres.
        fn multi_sphere_spheres(s: &bhkMultiSphereShape) -> Vec<BoundingSphere>;
        fn set_multi_sphere_spheres(s: Pin<&mut bhkMultiSphereShape>, spheres: &[BoundingSphere]);
        /// The convex vertices shape's vertices, or with `normals` its normals, as 4 floats
        /// each. The setter returns false, changing nothing, unless there are 4 per entry.
        fn convex_vertices_list(s: &bhkConvexVerticesShape, normals: bool) -> &[f32];
        fn set_convex_vertices_list(s: Pin<&mut bhkConvexVerticesShape>, normals: bool, values: &[f32]) -> bool;
        /// The list shape's filters. The setter returns false, changing nothing, unless there's
        /// one per sub-shape.
        fn list_shape_filters(s: &bhkListShape) -> Vec<HavokFilterFfi>;
        fn set_list_shape_filters(s: Pin<&mut bhkListShape>, filters: &[HavokFilterFfi]) -> bool;
        /// The MOPP code bytes.
        fn mopp_code(s: &bhkMoppBvTreeShape) -> &[u8];
        fn set_mopp_code(s: Pin<&mut bhkMoppBvTreeShape>, bytes: &[u8]);
        /// The strips shape's filters. The setter returns false, changing nothing, unless
        /// there's one per strips data ref.
        fn strips_shape_filters(s: &bhkNiTriStripsShape) -> &[u32];
        fn set_strips_shape_filters(s: Pin<&mut bhkNiTriStripsShape>, filters: &[u32]) -> bool;
        /// Sub-parts of the packed shape and data. The setters return false, changing nothing,
        /// for more than 65535.
        fn packed_shape_sub_parts(s: &bhkPackedNiTriStripsShape) -> Vec<SubPartFfi>;
        fn set_packed_shape_sub_parts(s: Pin<&mut bhkPackedNiTriStripsShape>, parts: &[SubPartFfi]) -> bool;
        fn packed_data_sub_parts(d: &hkPackedNiTriStripsData) -> Vec<SubPartFfi>;
        fn set_packed_data_sub_parts(d: Pin<&mut hkPackedNiTriStripsData>, parts: &[SubPartFfi]) -> bool;
        /// The packed data's triangles, from `triNormData` if it has `keyCount` entries and
        /// `triData` otherwise. The setter fills both and `keyCount`, and returns false,
        /// changing nothing, if an index isn't below the vertex count.
        fn packed_data_triangles(d: &hkPackedNiTriStripsData) -> Vec<PackedTriangleFfi>;
        fn set_packed_data_triangles(d: Pin<&mut hkPackedNiTriStripsData>, tris: &[PackedTriangleFfi]) -> bool;
        /// The packed data's vertices. The setter also sets `numVerts`, and returns false,
        /// changing nothing, if a triangle uses a vertex past the new count.
        fn packed_data_vertices(d: &hkPackedNiTriStripsData) -> Vec<Vec3>;
        fn set_packed_data_vertices(d: Pin<&mut hkPackedNiTriStripsData>, verts: &[Vec3]) -> bool;

        /// The compressed mesh data's lists. `which` picks material list 32 (0), 16 (1) or 8
        /// (2). Big vertices are 4 floats each; their setter returns false, changing nothing,
        /// unless there are 4 per vertex and the big triangles only use vertices below the new
        /// count. The big triangle setter returns false, changing nothing, if an index isn't
        /// below the big vertex count.
        fn cms_materials_n(d: &bhkCompressedMeshShapeData, which: u8) -> &[u32];
        fn set_cms_materials_n(d: Pin<&mut bhkCompressedMeshShapeData>, which: u8, values: &[u32]);
        fn cms_materials(d: &bhkCompressedMeshShapeData) -> Vec<CmsMaterialFfi>;
        fn set_cms_materials(d: Pin<&mut bhkCompressedMeshShapeData>, values: &[CmsMaterialFfi]);
        fn cms_transforms(d: &bhkCompressedMeshShapeData) -> Vec<CmsTransformFfi>;
        fn set_cms_transforms(d: Pin<&mut bhkCompressedMeshShapeData>, values: &[CmsTransformFfi]);
        fn cms_big_vertices(d: &bhkCompressedMeshShapeData) -> &[f32];
        fn set_cms_big_vertices(d: Pin<&mut bhkCompressedMeshShapeData>, values: &[f32]) -> bool;
        fn cms_big_triangles(d: &bhkCompressedMeshShapeData) -> Vec<CmsBigTriangleFfi>;
        fn set_cms_big_triangles(d: Pin<&mut bhkCompressedMeshShapeData>, values: &[CmsBigTriangleFfi]) -> bool;
        /// The chunks. `cms_chunk_list` returns a chunk's vertices (0), indices (1), strips (2)
        /// or welding info (3). `set_cms_chunks` takes each chunk's four lists in turn in
        /// `lists`, with their lengths in `lengths`, and returns false, changing nothing, unless
        /// the lengths add up. Indices must be below `cms_chunk_count`.
        fn cms_chunk_count(d: &bhkCompressedMeshShapeData) -> usize;
        fn cms_chunk(d: &bhkCompressedMeshShapeData, index: usize) -> CmsChunkFfi;
        fn cms_chunk_list(d: &bhkCompressedMeshShapeData, index: usize, which: u8) -> &[u16];
        fn set_cms_chunks(
            d: Pin<&mut bhkCompressedMeshShapeData>,
            chunks: &[CmsChunkFfi],
            lists: &[u16],
            lengths: &[u32],
        ) -> bool;

        /// Adds a new shape of the named type as the shape of the world object, transform shape
        /// or MOPP shape at `owner` (`assign_*`), or at the end of the list or convex list shape
        /// at `owner` (`add_sub_*`), and returns its index. `NIF_NPOS`, changing nothing, if
        /// the block isn't one of those, or for `add_sub_*` it's a convex list and the type
        /// isn't convex. A list shape with one filter per sub-shape gets a default one.
        fn assign_bhk_plane_shape(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_sub_bhk_plane_shape(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn assign_bhk_multi_sphere_shape(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_sub_bhk_multi_sphere_shape(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn assign_bhk_convex_list_shape(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_sub_bhk_convex_list_shape(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn assign_bhk_convex_vertices_shape(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_sub_bhk_convex_vertices_shape(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn assign_bhk_box_shape(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_sub_bhk_box_shape(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn assign_bhk_sphere_shape(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_sub_bhk_sphere_shape(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn assign_bhk_cylinder_shape(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_sub_bhk_cylinder_shape(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn assign_bhk_transform_shape(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_sub_bhk_transform_shape(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn assign_bhk_convex_transform_shape(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_sub_bhk_convex_transform_shape(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn assign_bhk_capsule_shape(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_sub_bhk_capsule_shape(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn assign_bhk_mopp_bv_tree_shape(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_sub_bhk_mopp_bv_tree_shape(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn assign_bhk_ni_tri_strips_shape(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_sub_bhk_ni_tri_strips_shape(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn assign_bhk_list_shape(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_sub_bhk_list_shape(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn assign_bhk_packed_ni_tri_strips_shape(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_sub_bhk_packed_ni_tri_strips_shape(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn assign_bhk_compressed_mesh_shape(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_sub_bhk_compressed_mesh_shape(file: Pin<&mut NifFile>, owner: u32) -> u32;

        /// Adds new data to the packed or compressed mesh shape at `shape` and returns its
        /// index. `NIF_NPOS`, changing nothing, if the block isn't that shape.
        fn assign_packed_data(file: Pin<&mut NifFile>, shape: u32) -> u32;
        fn assign_compressed_data(file: Pin<&mut NifFile>, shape: u32) -> u32;

        /// Constraint fields. The descriptors go through `DescriptorFfi`. A wrapped constraint
        /// reads the descriptor of its type (none for an unknown type), and its setter sets the
        /// type and that descriptor, returning false, changing nothing, for an unknown type.
        fn constraint_priority(o: &bhkConstraint) -> u32;
        fn set_constraint_priority(o: Pin<&mut bhkConstraint>, v: u32);
        fn hinge_descriptor(c: &bhkHingeConstraint) -> DescriptorFfi;
        fn set_hinge_descriptor(c: Pin<&mut bhkHingeConstraint>, d: &DescriptorFfi);
        fn limited_hinge_descriptor(c: &bhkLimitedHingeConstraint) -> DescriptorFfi;
        fn set_limited_hinge_descriptor(c: Pin<&mut bhkLimitedHingeConstraint>, d: &DescriptorFfi);
        fn ragdoll_descriptor(c: &bhkRagdollConstraint) -> DescriptorFfi;
        fn set_ragdoll_descriptor(c: Pin<&mut bhkRagdollConstraint>, d: &DescriptorFfi);
        fn stiff_spring_descriptor(c: &bhkStiffSpringConstraint) -> DescriptorFfi;
        fn set_stiff_spring_descriptor(c: Pin<&mut bhkStiffSpringConstraint>, d: &DescriptorFfi);
        fn prismatic_descriptor(c: &bhkPrismaticConstraint) -> DescriptorFfi;
        fn set_prismatic_descriptor(c: Pin<&mut bhkPrismaticConstraint>, d: &DescriptorFfi);
        fn ball_and_socket_descriptor(c: &bhkBallAndSocketConstraint) -> DescriptorFfi;
        fn set_ball_and_socket_descriptor(c: Pin<&mut bhkBallAndSocketConstraint>, d: &DescriptorFfi);
        fn breakable_wrapped(c: &bhkBreakableConstraint) -> DescriptorFfi;
        fn set_breakable_wrapped(c: Pin<&mut bhkBreakableConstraint>, d: &DescriptorFfi) -> bool;
        fn breakable_wrapped_type(c: &bhkBreakableConstraint) -> u32;
        fn breakable_wrapped_priority(c: &bhkBreakableConstraint) -> u32;
        fn set_breakable_wrapped_priority(c: Pin<&mut bhkBreakableConstraint>, v: u32);
        fn breakable_tau(c: &bhkBreakableConstraint) -> f32;
        fn set_breakable_tau(c: Pin<&mut bhkBreakableConstraint>, v: f32);
        fn breakable_damping(c: &bhkBreakableConstraint) -> f32;
        fn set_breakable_damping(c: Pin<&mut bhkBreakableConstraint>, v: f32);
        fn breakable_strength(c: &bhkBreakableConstraint) -> f32;
        fn set_breakable_strength(c: Pin<&mut bhkBreakableConstraint>, v: f32);
        fn malleable_wrapped(c: &bhkMalleableConstraint) -> DescriptorFfi;
        fn set_malleable_wrapped(c: Pin<&mut bhkMalleableConstraint>, d: &DescriptorFfi) -> bool;
        fn malleable_wrapped_type(c: &bhkMalleableConstraint) -> u32;
        fn malleable_wrapped_priority(c: &bhkMalleableConstraint) -> u32;
        fn set_malleable_wrapped_priority(c: Pin<&mut bhkMalleableConstraint>, v: u32);
        fn malleable_tau(c: &bhkMalleableConstraint) -> f32;
        fn set_malleable_tau(c: Pin<&mut bhkMalleableConstraint>, v: f32);
        fn malleable_damping(c: &bhkMalleableConstraint) -> f32;
        fn set_malleable_damping(c: Pin<&mut bhkMalleableConstraint>, v: f32);
        fn malleable_strength(c: &bhkMalleableConstraint) -> f32;
        fn set_malleable_strength(c: Pin<&mut bhkMalleableConstraint>, v: f32);
        fn breakable_remove_when_broken(c: &bhkBreakableConstraint) -> bool;
        fn set_breakable_remove_when_broken(c: Pin<&mut bhkBreakableConstraint>, v: bool);
        /// The chain's pivots, 4 floats each; the setter returns false, changing nothing,
        /// unless there are 4 per pivot.
        fn chain_pivots(c: &bhkBallSocketConstraintChain) -> &[f32];
        fn set_chain_pivots(c: Pin<&mut bhkBallSocketConstraintChain>, values: &[f32]) -> bool;
        fn chain_tau(c: &bhkBallSocketConstraintChain) -> f32;
        fn set_chain_tau(c: Pin<&mut bhkBallSocketConstraintChain>, v: f32);
        fn chain_damping(c: &bhkBallSocketConstraintChain) -> f32;
        fn set_chain_damping(c: Pin<&mut bhkBallSocketConstraintChain>, v: f32);
        fn chain_cfm(c: &bhkBallSocketConstraintChain) -> f32;
        fn set_chain_cfm(c: Pin<&mut bhkBallSocketConstraintChain>, v: f32);
        fn chain_max_error_distance(c: &bhkBallSocketConstraintChain) -> f32;
        fn set_chain_max_error_distance(c: Pin<&mut bhkBallSocketConstraintChain>, v: f32);
        fn chain_priority(c: &bhkBallSocketConstraintChain) -> u32;
        fn set_chain_priority(c: Pin<&mut bhkBallSocketConstraintChain>, v: u32);

        /// Adds a new constraint of the named type to the rigid body at `body`'s constraints,
        /// with the body as entity A, and returns its index.
        /// `NIF_NPOS`, changing nothing, if the block isn't a rigid body.
        fn add_bhk_hinge_constraint(file: Pin<&mut NifFile>, body: u32) -> u32;
        fn add_bhk_limited_hinge_constraint(file: Pin<&mut NifFile>, body: u32) -> u32;
        fn add_bhk_ragdoll_constraint(file: Pin<&mut NifFile>, body: u32) -> u32;
        fn add_bhk_stiff_spring_constraint(file: Pin<&mut NifFile>, body: u32) -> u32;
        fn add_bhk_prismatic_constraint(file: Pin<&mut NifFile>, body: u32) -> u32;
        fn add_bhk_ball_and_socket_constraint(file: Pin<&mut NifFile>, body: u32) -> u32;
        fn add_bhk_breakable_constraint(file: Pin<&mut NifFile>, body: u32) -> u32;
        fn add_bhk_malleable_constraint(file: Pin<&mut NifFile>, body: u32) -> u32;
        fn add_bhk_ball_socket_constraint_chain(file: Pin<&mut NifFile>, body: u32) -> u32;

        /// The pose array's bone names and poses. `set_pose_array_bones` takes the names
        /// joined, with their lengths. `set_pose_array_poses` takes each pose's transforms in
        /// turn, with their counts, and returns false, changing nothing, unless they add up.
        fn pose_array_bone_count(p: &bhkPoseArray) -> usize;
        fn pose_array_bone(p: &bhkPoseArray, index: usize) -> &[u8];
        fn set_pose_array_bones(p: Pin<&mut bhkPoseArray>, bytes: &[u8], lengths: &[usize]);
        fn pose_array_pose_count(p: &bhkPoseArray) -> usize;
        fn pose_array_pose(p: &bhkPoseArray, index: usize) -> Vec<BoneTransformFfi>;
        fn set_pose_array_poses(p: Pin<&mut bhkPoseArray>, transforms: &[BoneTransformFfi], lengths: &[usize]) -> bool;

        /// `bhkRagdollTemplateData` fields. The constraint setter keeps each position's entities
        /// and returns false, changing nothing, if a type is unknown.
        fn template_data_name(d: &bhkRagdollTemplateData) -> &[u8];
        fn set_template_data_name(d: Pin<&mut bhkRagdollTemplateData>, name: &[u8]);
        fn template_data_mass(d: &bhkRagdollTemplateData) -> f32;
        fn set_template_data_mass(d: Pin<&mut bhkRagdollTemplateData>, v: f32);
        fn template_data_restitution(d: &bhkRagdollTemplateData) -> f32;
        fn set_template_data_restitution(d: Pin<&mut bhkRagdollTemplateData>, v: f32);
        fn template_data_friction(d: &bhkRagdollTemplateData) -> f32;
        fn set_template_data_friction(d: Pin<&mut bhkRagdollTemplateData>, v: f32);
        fn template_data_radius(d: &bhkRagdollTemplateData) -> f32;
        fn set_template_data_radius(d: Pin<&mut bhkRagdollTemplateData>, v: f32);
        fn template_data_material(d: &bhkRagdollTemplateData) -> u32;
        fn set_template_data_material(d: Pin<&mut bhkRagdollTemplateData>, v: u32);
        fn template_data_constraints(d: &bhkRagdollTemplateData) -> Vec<JointConstraintFfi>;
        fn set_template_data_constraints(d: Pin<&mut bhkRagdollTemplateData>, values: &[JointConstraintFfi]) -> bool;

        /// Adds a new, unattached block of the named type and returns its index.
        fn add_bhk_pose_array(file: Pin<&mut NifFile>) -> u32;
        fn add_bhk_ragdoll_template_data(file: Pin<&mut NifFile>) -> u32;
        fn add_bhk_ragdoll_template(file: Pin<&mut NifFile>) -> u32;
    }
}
