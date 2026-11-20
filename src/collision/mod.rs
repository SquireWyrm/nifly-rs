//! Collision: [`NiCollisionObject`] and the Havok (`bhk`) blocks.
//!
//! A scene object's collision object ([`View::collision`]) links it to a Havok body
//! ([`View::body`]), usually a [`BhkRigidBody`], which has a [`BhkShape`]. Add one with
//! [`Edit::assign_collision`].
//!
//! ```text
//! NiAvObject -> BhkCollisionObject -> BhkRigidBody -> BhkMoppBvTreeShape -> BhkCompressedMeshShape
//! ```
//!
//! Havok values are stored as they are: positions are in Havok units, which are game units
//! divided by 69.99 in Skyrim and by 7 in Oblivion and Fallout 3.

use std::pin::Pin;

use bitflags::bitflags;
use bstr::{BStr, ByteSlice};
use int_enum::IntEnum;
use nifly_sys::{collision::ffi, nif_file::ffi as nif_ffi, nif_file_refs::ffi::RefKind};

use crate::{
    effects::Plane,
    extra_data::{joined, BsExtraData, NiExtraData},
    file::{index_from_ffi, Index, NifFile},
    geometry::NiTriStripsData,
    handle::{Edit, View},
    iter::{BlockIter, RefIndices},
    object3d::{BoundingSphere, BoundingVolume, Vec3},
    objects::{IsA, NiAvObject, NiObject, NiType},
};
use nifly_sys::nif_file_refs::ffi::RefList;

ni_types! { ffi;
    /// Base class of collision objects, the link between a scene object and its collision. Its target is on [`View::target`].
    NiCollisionObject(NiCollisionObject): NiObject => upcast_ni_collision_object, downcast_ni_collision_object;
    /// Collision from a bounding volume (Morrowind and Oblivion era files).
    NiCollisionData(NiCollisionData): NiCollisionObject => upcast_ni_collision_data, downcast_ni_collision_data;
    /// Base class of Havok collision objects. Its body is on [`View::body`].
    BhkNiCollisionObject(bhkNiCollisionObject): NiCollisionObject => upcast_bhk_ni_collision_object, downcast_bhk_ni_collision_object;
    /// The usual Havok collision object, from Oblivion to Skyrim.
    BhkCollisionObject(bhkCollisionObject): BhkNiCollisionObject => upcast_bhk_collision_object, downcast_bhk_collision_object;
    /// A Havok collision object whose body is a [`BhkPhysicsSystem`] or [`BhkRagdollSystem`] (Fallout 4 and later).
    ///
    /// The `bhkNPCollisionObject` block.
    BhkNpCollisionObject(bhkNPCollisionObject): BhkCollisionObject => upcast_bhk_np_collision_object, downcast_bhk_np_collision_object;
    /// A Havok collision object for a phantom.
    BhkPCollisionObject(bhkPCollisionObject): BhkNiCollisionObject => upcast_bhk_p_collision_object, downcast_bhk_p_collision_object;
    /// A Havok collision object for a simple shape phantom.
    ///
    /// The `bhkSPCollisionObject` block.
    BhkSpCollisionObject(bhkSPCollisionObject): BhkPCollisionObject => upcast_bhk_sp_collision_object, downcast_bhk_sp_collision_object;
    /// A Havok collision object blended with animation, used for ragdolls.
    BhkBlendCollisionObject(bhkBlendCollisionObject): BhkCollisionObject => upcast_bhk_blend_collision_object, downcast_bhk_blend_collision_object;
    /// Packed Havok physics data (Fallout 4 and later), the body of a [`BhkNpCollisionObject`].
    BhkPhysicsSystem(bhkPhysicsSystem): BsExtraData => upcast_bhk_physics_system, downcast_bhk_physics_system;
    /// Packed Havok ragdoll data (Fallout 4 and later), the body of a [`BhkNpCollisionObject`].
    BhkRagdollSystem(bhkRagdollSystem): BsExtraData => upcast_bhk_ragdoll_system, downcast_bhk_ragdoll_system;
    /// Base class of Havok blocks.
    BhkRefObject(bhkRefObject): NiObject => upcast_bhk_ref_object, downcast_bhk_ref_object;
    /// Base class of Havok blocks stored with the scene.
    BhkSerializable(bhkSerializable): BhkRefObject => upcast_bhk_serializable, downcast_bhk_serializable;
    /// Base class of collision shapes.
    BhkShape(bhkShape): BhkSerializable => upcast_bhk_shape, downcast_bhk_shape;
    /// Base class of height field shapes.
    BhkHeightFieldShape(bhkHeightFieldShape): BhkShape => upcast_bhk_height_field_shape, downcast_bhk_height_field_shape;
    /// An infinite plane.
    BhkPlaneShape(bhkPlaneShape): BhkHeightFieldShape => upcast_bhk_plane_shape, downcast_bhk_plane_shape;
    /// Base class of shapes made of spheres.
    BhkSphereRepShape(bhkSphereRepShape): BhkShape => upcast_bhk_sphere_rep_shape, downcast_bhk_sphere_rep_shape;
    /// Base class of convex shapes.
    BhkConvexShape(bhkConvexShape): BhkSphereRepShape => upcast_bhk_convex_shape, downcast_bhk_convex_shape;
    /// A set of spheres.
    BhkMultiSphereShape(bhkMultiSphereShape): BhkSphereRepShape => upcast_bhk_multi_sphere_shape, downcast_bhk_multi_sphere_shape;
    /// A list of convex shapes treated as one convex shape.
    BhkConvexListShape(bhkConvexListShape): BhkShape => upcast_bhk_convex_list_shape, downcast_bhk_convex_list_shape;
    /// The convex hull of a set of vertices.
    BhkConvexVerticesShape(bhkConvexVerticesShape): BhkConvexShape => upcast_bhk_convex_vertices_shape, downcast_bhk_convex_vertices_shape;
    /// A box.
    BhkBoxShape(bhkBoxShape): BhkConvexShape => upcast_bhk_box_shape, downcast_bhk_box_shape;
    /// A sphere.
    BhkSphereShape(bhkSphereShape): BhkConvexShape => upcast_bhk_sphere_shape, downcast_bhk_sphere_shape;
    /// A cylinder.
    BhkCylinderShape(bhkCylinderShape): BhkConvexShape => upcast_bhk_cylinder_shape, downcast_bhk_cylinder_shape;
    /// A shape with a transform applied.
    BhkTransformShape(bhkTransformShape): BhkShape => upcast_bhk_transform_shape, downcast_bhk_transform_shape;
    /// A convex shape with a transform applied.
    BhkConvexTransformShape(bhkConvexTransformShape): BhkTransformShape => upcast_bhk_convex_transform_shape, downcast_bhk_convex_transform_shape;
    /// A capsule: a cylinder with rounded ends.
    BhkCapsuleShape(bhkCapsuleShape): BhkConvexShape => upcast_bhk_capsule_shape, downcast_bhk_capsule_shape;
    /// Base class of shapes with a bounding volume tree.
    BhkBvTreeShape(bhkBvTreeShape): BhkShape => upcast_bhk_bv_tree_shape, downcast_bhk_bv_tree_shape;
    /// A shape with a MOPP tree that speeds up collision tests against it.
    BhkMoppBvTreeShape(bhkMoppBvTreeShape): BhkBvTreeShape => upcast_bhk_mopp_bv_tree_shape, downcast_bhk_mopp_bv_tree_shape;
    /// A mesh shape made from `NiTriStripsData` blocks (Oblivion).
    BhkNiTriStripsShape(bhkNiTriStripsShape): BhkShape => upcast_bhk_ni_tri_strips_shape, downcast_bhk_ni_tri_strips_shape;
    /// Base class of shapes made of other shapes.
    BhkShapeCollection(bhkShapeCollection): BhkShape => upcast_bhk_shape_collection, downcast_bhk_shape_collection;
    /// A list of shapes.
    BhkListShape(bhkListShape): BhkShapeCollection => upcast_bhk_list_shape, downcast_bhk_list_shape;
    /// The triangles and vertices of a [`BhkPackedNiTriStripsShape`].
    ///
    /// The `hkPackedNiTriStripsData` block.
    HkPackedNiTriStripsData(hkPackedNiTriStripsData): BhkShapeCollection => upcast_hk_packed_ni_tri_strips_data, downcast_hk_packed_ni_tri_strips_data;
    /// A mesh shape (Oblivion and Fallout 3).
    BhkPackedNiTriStripsShape(bhkPackedNiTriStripsShape): BhkShapeCollection => upcast_bhk_packed_ni_tri_strips_shape, downcast_bhk_packed_ni_tri_strips_shape;
    /// A liquid action.
    BhkLiquidAction(bhkLiquidAction): BhkSerializable => upcast_bhk_liquid_action, downcast_bhk_liquid_action;
    /// An action keeping a hinged body oriented.
    BhkOrientHingedBodyAction(bhkOrientHingedBodyAction): BhkSerializable => upcast_bhk_orient_hinged_body_action, downcast_bhk_orient_hinged_body_action;
    /// Base class of objects in the Havok world, which have a shape.
    BhkWorldObject(bhkWorldObject): BhkSerializable => upcast_bhk_world_object, downcast_bhk_world_object;
    /// Base class of phantoms, which detect overlaps without colliding.
    BhkPhantom(bhkPhantom): BhkWorldObject => upcast_bhk_phantom, downcast_bhk_phantom;
    /// Base class of phantoms with a shape.
    BhkShapePhantom(bhkShapePhantom): BhkPhantom => upcast_bhk_shape_phantom, downcast_bhk_shape_phantom;
    /// A phantom with a shape and a transform.
    BhkSimpleShapePhantom(bhkSimpleShapePhantom): BhkShapePhantom => upcast_bhk_simple_shape_phantom, downcast_bhk_simple_shape_phantom;
    /// A phantom over an axis-aligned box.
    BhkAabbPhantom(bhkAabbPhantom): BhkShapePhantom => upcast_bhk_aabb_phantom, downcast_bhk_aabb_phantom;
    /// Base class of entities, which constraints connect.
    BhkEntity(bhkEntity): BhkWorldObject => upcast_bhk_entity, downcast_bhk_entity;
    /// A rigid body: a shape with mass and motion.
    BhkRigidBody(bhkRigidBody): BhkEntity => upcast_bhk_rigid_body, downcast_bhk_rigid_body;
    /// A rigid body whose translation and rotation apply to its shape.
    BhkRigidBodyT(bhkRigidBodyT): BhkRigidBody => upcast_bhk_rigid_body_t, downcast_bhk_rigid_body_t;
    /// Base class of constraints between two entities.
    BhkConstraint(bhkConstraint): BhkSerializable => upcast_bhk_constraint, downcast_bhk_constraint;
    /// A hinge.
    BhkHingeConstraint(bhkHingeConstraint): BhkConstraint => upcast_bhk_hinge_constraint, downcast_bhk_hinge_constraint;
    /// A hinge with angle limits.
    BhkLimitedHingeConstraint(bhkLimitedHingeConstraint): BhkConstraint => upcast_bhk_limited_hinge_constraint, downcast_bhk_limited_hinge_constraint;
    /// A constraint that breaks under enough force.
    BhkBreakableConstraint(bhkBreakableConstraint): BhkConstraint => upcast_bhk_breakable_constraint, downcast_bhk_breakable_constraint;
    /// A ragdoll joint.
    BhkRagdollConstraint(bhkRagdollConstraint): BhkConstraint => upcast_bhk_ragdoll_constraint, downcast_bhk_ragdoll_constraint;
    /// A stiff spring keeping two points at a distance.
    BhkStiffSpringConstraint(bhkStiffSpringConstraint): BhkConstraint => upcast_bhk_stiff_spring_constraint, downcast_bhk_stiff_spring_constraint;
    /// A sliding joint.
    BhkPrismaticConstraint(bhkPrismaticConstraint): BhkConstraint => upcast_bhk_prismatic_constraint, downcast_bhk_prismatic_constraint;
    /// A constraint with adjustable strength.
    BhkMalleableConstraint(bhkMalleableConstraint): BhkConstraint => upcast_bhk_malleable_constraint, downcast_bhk_malleable_constraint;
    /// A ball and socket joint.
    BhkBallAndSocketConstraint(bhkBallAndSocketConstraint): BhkConstraint => upcast_bhk_ball_and_socket_constraint, downcast_bhk_ball_and_socket_constraint;
    /// A chain of ball and socket joints.
    BhkBallSocketConstraintChain(bhkBallSocketConstraintChain): BhkSerializable => upcast_bhk_ball_socket_constraint_chain, downcast_bhk_ball_socket_constraint_chain;
    /// The compressed triangles and vertices of a [`BhkCompressedMeshShape`].
    BhkCompressedMeshShapeData(bhkCompressedMeshShapeData): BhkRefObject => upcast_bhk_compressed_mesh_shape_data, downcast_bhk_compressed_mesh_shape_data;
    /// A compressed mesh shape (Skyrim).
    BhkCompressedMeshShape(bhkCompressedMeshShape): BhkShape => upcast_bhk_compressed_mesh_shape, downcast_bhk_compressed_mesh_shape;
    /// Bone poses.
    BhkPoseArray(bhkPoseArray): NiObject => upcast_bhk_pose_array, downcast_bhk_pose_array;
    /// Extra data listing a ragdoll's bones.
    BhkRagdollTemplate(bhkRagdollTemplate): NiExtraData => upcast_bhk_ragdoll_template, downcast_bhk_ragdoll_template;
    /// A ragdoll bone's physical settings and constraints.
    BhkRagdollTemplateData(bhkRagdollTemplateData): NiObject => upcast_bhk_ragdoll_template_data, downcast_bhk_ragdoll_template_data;
}

/// Getter and setter pairs over the bridge's field shims.
macro_rules! fields {
    ($($(#[$doc:meta])* $get:ident, $set:ident: $ty:ty = $ffi_get:ident, $ffi_set:ident;)*) => {$(
        $(#[$doc])*
        pub fn $get(&self) -> $ty {
            ffi::$ffi_get(&self.0)
        }

        #[doc = concat!("Sets [`", stringify!($get), "`](Self::", stringify!($get), ").")]
        pub fn $set(&mut self, value: $ty) {
            ffi::$ffi_set(self.ffi_mut(), value);
        }
    )*};
}

/// Like `fields!`, converting with `$to` and `$from`.
macro_rules! converted_fields {
    ($($(#[$doc:meta])* $get:ident, $set:ident: $ty:ty = $ffi_get:ident, $ffi_set:ident, $to:expr, $from:expr;)*) => {$(
        $(#[$doc])*
        pub fn $get(&self) -> $ty {
            ($to)(ffi::$ffi_get(&self.0))
        }

        #[doc = concat!("Sets [`", stringify!($get), "`](Self::", stringify!($get), ").")]
        pub fn $set(&mut self, value: $ty) {
            ffi::$ffi_set(self.ffi_mut(), ($from)(value));
        }
    )*};
}

fn rows<const R: usize, const N: usize>(values: [f32; N]) -> [[f32; 4]; R] {
    std::array::from_fn(|r| std::array::from_fn(|c| values[r * 4 + c]))
}

fn flatten<const R: usize, const N: usize>(rows: [[f32; 4]; R]) -> [f32; N] {
    std::array::from_fn(|i| rows[i / 4][i % 4])
}

/// # Collision
///
/// Adding collision blocks that don't belong to another block. Collision objects, bodies and
/// shapes are added through the block they belong to, such as [`Edit::assign_collision`].
impl NifFile {
    /// Adds a new action of type `A` with default settings and returns its index. Nothing refers
    /// to it, so saving with [`SaveOptions::optimize`](crate::file::SaveOptions::optimize) drops
    /// it.
    pub fn add_action<A: NewAction>(&mut self) -> Index {
        index_from_ffi(A::add(self.inner.pin_mut())).expect("adding a block succeeds")
    }

    /// Adds a new, empty [`BhkPoseArray`] and returns its index. Nothing refers to it, so saving
    /// with [`SaveOptions::optimize`](crate::file::SaveOptions::optimize) drops it.
    pub fn add_pose_array(&mut self) -> Index {
        index_from_ffi(ffi::add_bhk_pose_array(self.inner.pin_mut())).expect("adding a block succeeds")
    }

    /// Adds a new [`BhkRagdollTemplate`] with no bones and returns its index. Attach it to an
    /// object with [`Edit::set_extra_data`].
    pub fn add_ragdoll_template(&mut self) -> Index {
        index_from_ffi(ffi::add_bhk_ragdoll_template(self.inner.pin_mut())).expect("adding a block succeeds")
    }

    /// Adds a new [`BhkRagdollTemplateData`] with default settings and returns its index. Add it
    /// to a template with [`Edit::set_ragdoll_bones`].
    pub fn add_ragdoll_template_data(&mut self) -> Index {
        index_from_ffi(ffi::add_bhk_ragdoll_template_data(self.inner.pin_mut())).expect("adding a block succeeds")
    }
}

mod objects;
mod bodies;
mod shapes;
mod mesh_shapes;
mod motors;
mod constraints;
mod ragdolls;

pub use self::objects::*;
pub use self::bodies::*;
pub use self::shapes::*;
pub use self::mesh_shapes::*;
pub use self::motors::*;
pub use self::constraints::*;
pub use self::ragdolls::*;
