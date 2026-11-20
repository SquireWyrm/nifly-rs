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
use nifly_sys::{collision::ffi, nif_file::ffi::{self as nif_ffi, RefKind}};

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
use nifly_sys::nif_file::ffi::RefList;

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
        pub fn $set(self: Pin<&mut Self>, value: $ty) {
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
        pub fn $set(self: Pin<&mut Self>, value: $ty) {
            ffi::$ffi_set(self.ffi_mut(), ($from)(value));
        }
    )*};
}

impl NiCollisionData {
    /// How collision tests pass on to children, or `None` if the stored value isn't a
    /// [`PropagationMode`].
    pub fn propagation_mode(&self) -> Option<PropagationMode> {
        PropagationMode::try_from(ffi::collision_data_propagation_mode(&self.0)).ok()
    }

    pub fn set_propagation_mode(self: Pin<&mut Self>, mode: PropagationMode) {
        ffi::set_collision_data_propagation_mode(self.ffi_mut(), mode.into());
    }

    /// What collision tests use, or `None` if the stored value isn't a [`CollisionMode`].
    pub fn collision_mode(&self) -> Option<CollisionMode> {
        CollisionMode::try_from(ffi::collision_data_collision_mode(&self.0)).ok()
    }

    pub fn set_collision_mode(self: Pin<&mut Self>, mode: CollisionMode) {
        ffi::set_collision_data_collision_mode(self.ffi_mut(), mode.into());
    }

    /// The bounding volume collision tests use, or `None` if it has none.
    pub fn bounding_volume(&self) -> Option<BoundingVolume> {
        BoundingVolume::from_ffi(&mut ffi::collision_data_bounding_volume(&self.0).into_iter())
    }

    /// Sets the [bounding volume](Self::bounding_volume), or removes it with `None`.
    pub fn set_bounding_volume(self: Pin<&mut Self>, volume: Option<&BoundingVolume>) {
        let mut records = Vec::new();
        if let Some(volume) = volume {
            volume.to_ffi(&mut records);
        }
        assert!(ffi::set_collision_data_bounding_volume(self.ffi_mut(), &records), "records of one volume");
    }
}

/// How an [`NiCollisionData`]'s collision tests pass on to its object's children.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum PropagationMode {
    OnSuccess = 0,
    OnFailure = 1,
    Always = 2,
    Never = 3,
}

/// What an [`NiCollisionData`]'s collision tests use.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum CollisionMode {
    /// An oriented bounding box.
    UseObb = 0,
    /// The triangles.
    UseTri = 1,
    /// The [bounding volume](NiCollisionData::bounding_volume).
    UseAbv = 2,
    /// No tests.
    NoTest = 3,
    /// The object's bounds.
    UseNiBound = 4,
}

impl BhkNiCollisionObject {
    converted_fields! {
        flags, set_flags: CollisionObjectFlags = bhk_collision_flags, set_bhk_collision_flags,
            CollisionObjectFlags::from_bits_retain, |f: CollisionObjectFlags| f.bits();
    }
}

bitflags! {
    /// The flags of a [`BhkNiCollisionObject`]. Other bits are kept as stored.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct CollisionObjectFlags: u16 {
        const ACTIVE = 1;
        const RESET_TRANSFORM = 1 << 1;
        const NOTIFY = 1 << 2;
        const SET_LOCAL = 1 << 3;
        const DEBUG_DISPLAY = 1 << 4;
        const USE_VELOCITY = 1 << 5;
        const RESET = 1 << 6;
        const SYNC_ON_UPDATE = 1 << 7;
        /// Only used by a [`BhkBlendCollisionObject`].
        const BLEND_POSITION = 1 << 8;
        /// Only used by a [`BhkBlendCollisionObject`].
        const ALWAYS_BLEND = 1 << 9;
        const ANIM_TARGETED = 1 << 10;
        const DISMEMBERED_LIMB = 1 << 11;
        const _ = !0;
    }
}

impl BhkNpCollisionObject {
    fields! {
        /// Which body of its [system](View::body) the object uses.
        body_id, set_body_id: u32 = np_collision_body_id, set_np_collision_body_id;
    }
}

impl BhkBlendCollisionObject {
    fields! {
        heir_gain, set_heir_gain: f32 = blend_collision_heir_gain, set_blend_collision_heir_gain;
        velocity_gain, set_velocity_gain: f32 = blend_collision_vel_gain, set_blend_collision_vel_gain;
    }
}

impl BhkPhysicsSystem {
    /// The packed Havok data.
    pub fn data(&self) -> &[u8] {
        ffi::physics_system_data(&self.0)
    }

    pub fn set_data(self: Pin<&mut Self>, data: &[u8]) {
        ffi::set_physics_system_data(self.ffi_mut(), data);
    }
}

impl BhkRagdollSystem {
    /// The packed Havok data.
    pub fn data(&self) -> &[u8] {
        ffi::ragdoll_system_data(&self.0)
    }

    pub fn set_data(self: Pin<&mut Self>, data: &[u8]) {
        ffi::set_ragdoll_system_data(self.ffi_mut(), data);
    }
}

/// Targets and bodies of collision objects.
impl<'a, T: NiType> View<'a, T> {
    /// The scene object the collision object belongs to, or `None` if it has none.
    pub fn target<I>(&self) -> Option<View<'a, NiAvObject>>
    where
        T: IsA<NiCollisionObject, I>,
    {
        self.follow(RefKind::CollisionTarget)
    }

    /// The collision object's body: a [`BhkWorldObject`] such as a [`BhkRigidBody`], or for a
    /// [`BhkNpCollisionObject`] a [`BhkPhysicsSystem`] or [`BhkRagdollSystem`]. `None` if it has
    /// none.
    pub fn body<I>(&self) -> Option<View<'a, NiObject>>
    where
        T: IsA<BhkNiCollisionObject, I>,
    {
        self.follow(RefKind::CollisionBody)
    }
}

impl<T: NiType> Edit<'_, T> {
    /// Like [`View::target`], for editing.
    pub fn target<I>(&mut self) -> Option<Edit<'_, NiAvObject>>
    where
        T: IsA<NiCollisionObject, I>,
    {
        self.follow(RefKind::CollisionTarget)
    }

    /// Sets the scene object the collision object belongs to, or clears it with `None`. Returns
    /// `false`, changing nothing, unless `target` is an [`NiAvObject`]. The object's own
    /// [collision ref](Edit::set_collision) isn't changed.
    pub fn set_target<I>(&mut self, target: Option<Index>) -> bool
    where
        T: IsA<NiCollisionObject, I>,
    {
        self.set_ref(RefKind::CollisionTarget, target)
    }

    /// Like [`View::body`], for editing.
    pub fn body<I>(&mut self) -> Option<Edit<'_, NiObject>>
    where
        T: IsA<BhkNiCollisionObject, I>,
    {
        self.follow(RefKind::CollisionBody)
    }

    /// Sets the collision object's [body](View::body), or clears it with `None`. Returns `false`,
    /// changing nothing, if `body` is the wrong type for this collision object.
    pub fn set_body<I>(&mut self, body: Option<Index>) -> bool
    where
        T: IsA<BhkNiCollisionObject, I>,
    {
        self.set_ref(RefKind::CollisionBody, body)
    }

    /// Adds a new collision object of type `C` with default settings as the object's
    /// [collision](View::collision), targeting the object, and returns its index. A previous
    /// collision object stays in the file until it's optimized away. Doesn't check that the
    /// file's game uses `C`.
    ///
    /// ```
    /// # use nifly::{collision::BhkCollisionObject, file::{NifFile, NifVersion}, nodes::NiNode};
    /// let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    /// let collision = nif.root_node_mut().unwrap().assign_collision::<BhkCollisionObject, _>();
    /// let root = nif.root_node().unwrap();
    /// assert_eq!(root.collision().unwrap().index(), collision);
    /// ```
    pub fn assign_collision<C: NewCollisionObject, I>(&mut self) -> Index
    where
        T: IsA<NiAvObject, I>,
    {
        let object = self.index() as u32;
        index_from_ffi(C::assign(self.file_mut().inner.pin_mut(), object)).expect("the block is an NiAvObject")
    }

    /// Adds a new body of type `B` with default settings as the collision object's
    /// [body](View::body), and returns its index. A previous body stays in the file until it's
    /// optimized away.
    ///
    /// Returns `None`, changing nothing, unless `B` fits: a [`BhkPhysicsSystem`] or
    /// [`BhkRagdollSystem`] for a [`BhkNpCollisionObject`], a [`BhkWorldObject`] for the others.
    pub fn assign_body<B: NewBody, I>(&mut self) -> Option<Index>
    where
        T: IsA<BhkNiCollisionObject, I>,
    {
        let collision = self.index() as u32;
        index_from_ffi(B::assign(self.file_mut().inner.pin_mut(), collision))
    }
}

/// A body type that [`Edit::assign_body`] can create.
///
/// This trait is sealed.
pub trait NewBody: NiType {
    /// Adds the body to the collision object at `collision` and returns its index (`u32::MAX`
    /// on failure).
    #[doc(hidden)]
    fn assign(file: Pin<&mut nif_ffi::NifFile>, collision: u32) -> u32;
}

macro_rules! new_body {
    ($($ty:ty => $assign:ident;)*) => {$(
        impl NewBody for $ty {
            fn assign(file: Pin<&mut nif_ffi::NifFile>, collision: u32) -> u32 {
                ffi::$assign(file, collision)
            }
        }
    )*};
}

new_body! {
    BhkRigidBody => assign_bhk_rigid_body;
    BhkRigidBodyT => assign_bhk_rigid_body_t;
    BhkSimpleShapePhantom => assign_bhk_simple_shape_phantom;
    BhkAabbPhantom => assign_bhk_aabb_phantom;
    BhkPhysicsSystem => assign_bhk_physics_system;
    BhkRagdollSystem => assign_bhk_ragdoll_system;
}

/// A collision object type that [`Edit::assign_collision`] can create.
///
/// This trait is sealed.
pub trait NewCollisionObject: NiType {
    /// Adds the collision object to the object at `object` and returns its index (`u32::MAX`
    /// on failure).
    #[doc(hidden)]
    fn assign(file: Pin<&mut nif_ffi::NifFile>, object: u32) -> u32;
}

macro_rules! new_collision_object {
    ($($ty:ty => $assign:ident;)*) => {$(
        impl NewCollisionObject for $ty {
            fn assign(file: Pin<&mut nif_ffi::NifFile>, object: u32) -> u32 {
                ffi::$assign(file, object)
            }
        }
    )*};
}

new_collision_object! {
    NiCollisionData => assign_ni_collision_data;
    BhkCollisionObject => assign_bhk_collision_object;
    BhkNpCollisionObject => assign_bhk_np_collision_object;
    BhkPCollisionObject => assign_bhk_p_collision_object;
    BhkSpCollisionObject => assign_bhk_sp_collision_object;
    BhkBlendCollisionObject => assign_bhk_blend_collision_object;
}

/// Which collision layer and group an object is in. The default is layer 1, static objects in
/// every game, as in new blocks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HavokFilter {
    pub layer: u8,
    /// Flags in the top three bits, and a body part number in the rest.
    pub flags_and_parts: u8,
    pub group: u16,
}

impl Default for HavokFilter {
    fn default() -> Self {
        Self { layer: 1, flags_and_parts: 0, group: 0 }
    }
}

impl HavokFilter {
    fn from_ffi(f: ffi::HavokFilterFfi) -> Self {
        Self { layer: f.layer, flags_and_parts: f.flags_and_parts, group: f.group }
    }

    fn to_ffi(self) -> ffi::HavokFilterFfi {
        ffi::HavokFilterFfi { layer: self.layer, flags_and_parts: self.flags_and_parts, group: self.group }
    }
}

/// A Havok property record of unknown use, kept as stored.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct CInfoProperty {
    pub data: u32,
    pub size: u32,
    pub capacity_and_flags: u32,
}

impl CInfoProperty {
    fn from_ffi(p: ffi::CInfoPropertyFfi) -> Self {
        Self { data: p.data, size: p.size, capacity_and_flags: p.capacity_and_flags }
    }

    fn to_ffi(self) -> ffi::CInfoPropertyFfi {
        ffi::CInfoPropertyFfi { data: self.data, size: self.size, capacity_and_flags: self.capacity_and_flags }
    }
}

fn rows<const R: usize, const N: usize>(values: [f32; N]) -> [[f32; 4]; R] {
    std::array::from_fn(|r| std::array::from_fn(|c| values[r * 4 + c]))
}

fn flatten<const R: usize, const N: usize>(rows: [[f32; 4]; R]) -> [f32; N] {
    std::array::from_fn(|i| rows[i / 4][i % 4])
}

impl BhkWorldObject {
    converted_fields! {
        /// The collision layer and group.
        collision_filter, set_collision_filter: HavokFilter =
            world_object_filter, set_world_object_filter, HavokFilter::from_ffi, HavokFilter::to_ffi;
        property, set_property: CInfoProperty =
            world_object_property, set_world_object_property, CInfoProperty::from_ffi, CInfoProperty::to_ffi;
    }

    fields! {
        /// A value of unknown meaning.
        unknown_int, set_unknown_int: i32 = world_object_unknown_int, set_world_object_unknown_int;
        /// How the broad phase handles the object, as a raw value.
        broad_phase_type, set_broad_phase_type: u8 =
            world_object_broad_phase_type, set_world_object_broad_phase_type;
        /// Values of unknown meaning.
        unknown_bytes, set_unknown_bytes: [u8; 3] =
            world_object_unknown_bytes, set_world_object_unknown_bytes;
    }
}

/// # Fields by version
///
/// Oblivion and Fallout 3 files have stream version 34 or lower, Skyrim's 83 or 100, and Fallout
/// 4's 130. Fields a file's version doesn't store can still be read and set, but aren't written
/// when it's saved.
///
/// Vectors are `[x, y, z, w]` in Havok units, and the rotation is a quaternion `[x, y, z, w]`.
impl BhkRigidBody {
    /// How the body responds to collisions, or `None` if the stored value isn't a
    /// [`CollisionResponse`].
    pub fn collision_response(&self) -> Option<CollisionResponse> {
        CollisionResponse::try_from(ffi::rigid_body_collision_response(&self.0)).ok()
    }

    pub fn set_collision_response(self: Pin<&mut Self>, response: CollisionResponse) {
        ffi::set_rigid_body_collision_response(self.ffi_mut(), response.into());
    }

    converted_fields! {
        /// A copy of the [world object's filter](BhkWorldObject::collision_filter).
        collision_filter_copy, set_collision_filter_copy: HavokFilter =
            rigid_body_filter_copy, set_rigid_body_filter_copy, HavokFilter::from_ffi, HavokFilter::to_ffi;
        /// The inertia tensor as three rows of four values.
        inertia, set_inertia: [[f32; 4]; 3] = rigid_body_inertia, set_rigid_body_inertia, rows, flatten;
    }

    fields! {
        /// How many simulation steps pass between contact callbacks.
        process_contact_callback_delay, set_process_contact_callback_delay: u16 =
            rigid_body_callback_delay, set_rigid_body_callback_delay;
        /// A value of unknown meaning.
        unknown_int, set_unknown_int: u32 = rigid_body_unknown_int, set_rigid_body_unknown_int;
        /// Values of unknown meaning.
        unknown_shorts, set_unknown_shorts: [u16; 6] =
            rigid_body_unknown_shorts, set_rigid_body_unknown_shorts;
        translation, set_translation: [f32; 4] = rigid_body_translation, set_rigid_body_translation;
        rotation, set_rotation: [f32; 4] = rigid_body_rotation, set_rigid_body_rotation;
        linear_velocity, set_linear_velocity: [f32; 4] =
            rigid_body_linear_velocity, set_rigid_body_linear_velocity;
        angular_velocity, set_angular_velocity: [f32; 4] =
            rigid_body_angular_velocity, set_rigid_body_angular_velocity;
        /// The center of mass.
        center, set_center: [f32; 4] = rigid_body_center, set_rigid_body_center;
        mass, set_mass: f32 = rigid_body_mass, set_rigid_body_mass;
        linear_damping, set_linear_damping: f32 = rigid_body_linear_damping, set_rigid_body_linear_damping;
        angular_damping, set_angular_damping: f32 = rigid_body_angular_damping, set_rigid_body_angular_damping;
        /// Only saved in Skyrim (LE and SE).
        time_factor, set_time_factor: f32 = rigid_body_time_factor, set_rigid_body_time_factor;
        /// Only saved from Skyrim on.
        gravity_factor, set_gravity_factor: f32 = rigid_body_gravity_factor, set_rigid_body_gravity_factor;
        friction, set_friction: f32 = rigid_body_friction, set_rigid_body_friction;
        /// Only saved from Skyrim on.
        rolling_friction_multiplier, set_rolling_friction_multiplier: f32 =
            rigid_body_rolling_friction, set_rigid_body_rolling_friction;
        /// Bounciness.
        restitution, set_restitution: f32 = rigid_body_restitution, set_rigid_body_restitution;
        max_linear_velocity, set_max_linear_velocity: f32 =
            rigid_body_max_linear_velocity, set_rigid_body_max_linear_velocity;
        max_angular_velocity, set_max_angular_velocity: f32 =
            rigid_body_max_angular_velocity, set_rigid_body_max_angular_velocity;
        /// How deep other objects may go into the body.
        penetration_depth, set_penetration_depth: f32 =
            rigid_body_penetration_depth, set_rigid_body_penetration_depth;
        /// The motion system, as a raw value.
        motion_system, set_motion_system: u8 = rigid_body_motion_system, set_rigid_body_motion_system;
        /// The deactivator type, as a raw value.
        deactivator_type, set_deactivator_type: u8 = rigid_body_deactivator_type, set_rigid_body_deactivator_type;
        /// The solver deactivation, as a raw value.
        solver_deactivation, set_solver_deactivation: u8 =
            rigid_body_solver_deactivation, set_rigid_body_solver_deactivation;
        /// The motion quality, as a raw value.
        quality_type, set_quality_type: u8 = rigid_body_quality_type, set_rigid_body_quality_type;
        /// Only saved from Skyrim on.
        auto_remove_level, set_auto_remove_level: u8 =
            rigid_body_auto_remove_level, set_rigid_body_auto_remove_level;
        /// Only saved from Skyrim on.
        response_modifier_raw_flags, set_response_modifier_raw_flags: u8 =
            rigid_body_response_modifier_flags, set_rigid_body_response_modifier_flags;
        /// The number of shape keys in contact point properties. Only saved from Skyrim on.
        shape_keys_in_contact_points, set_shape_keys_in_contact_points: u8 =
            rigid_body_shape_keys, set_rigid_body_shape_keys;
        /// Only saved from Skyrim on.
        force_collide_onto_ppu, set_force_collide_onto_ppu: bool =
            rigid_body_force_collide_onto_ppu, set_rigid_body_force_collide_onto_ppu;
        /// The body flags. Only saved from Skyrim on; older files save
        /// [`body_raw_flags_32`](Self::body_raw_flags_32) instead.
        body_raw_flags, set_body_raw_flags: u16 = rigid_body_body_flags, set_rigid_body_body_flags;
        /// The body flags as 32 bits. Only saved before Skyrim. The two aren't converted into
        /// each other.
        body_raw_flags_32, set_body_raw_flags_32: u32 = rigid_body_body_flags_int, set_rigid_body_body_flags_int;
    }
}

/// How a [`BhkRigidBody`] responds to collisions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u8)]
pub enum CollisionResponse {
    Invalid = 0,
    /// Collides and bounces off.
    SimpleContact = 1,
    /// Only reports the contact.
    Reporting = 2,
    None = 3,
}

impl BhkSimpleShapePhantom {
    converted_fields! {
        /// The phantom's transform, as four rows.
        transform, set_transform: [[f32; 4]; 4] =
            simple_phantom_transform, set_simple_phantom_transform, rows, flatten;
    }
}

impl BhkAabbPhantom {
    fields! {
        /// The box's minimum corner, `[x, y, z, w]`.
        aabb_min, set_aabb_min: [f32; 4] = aabb_phantom_min, set_aabb_phantom_min;
        /// The box's maximum corner, `[x, y, z, w]`.
        aabb_max, set_aabb_max: [f32; 4] = aabb_phantom_max, set_aabb_phantom_max;
    }
}

impl BhkLiquidAction {
    fields! {
        user_data, set_user_data: u32 = liquid_user_data, set_liquid_user_data;
        /// A value of unknown meaning.
        unknown_int_1, set_unknown_int_1: u32 = liquid_unknown_int_1, set_liquid_unknown_int_1;
        /// A value of unknown meaning.
        unknown_int_2, set_unknown_int_2: u32 = liquid_unknown_int_2, set_liquid_unknown_int_2;
        initial_stick_force, set_initial_stick_force: f32 =
            liquid_initial_stick_force, set_liquid_initial_stick_force;
        stick_strength, set_stick_strength: f32 = liquid_stick_strength, set_liquid_stick_strength;
        neighbor_distance, set_neighbor_distance: f32 =
            liquid_neighbor_distance, set_liquid_neighbor_distance;
        neighbor_strength, set_neighbor_strength: f32 =
            liquid_neighbor_strength, set_liquid_neighbor_strength;
    }
}

/// Its body is on [`View::hinged_body`].
impl BhkOrientHingedBodyAction {
    fields! {
        /// A value of unknown meaning.
        unknown_int_1, set_unknown_int_1: u32 = hinged_action_unknown_int_1, set_hinged_action_unknown_int_1;
        /// A value of unknown meaning.
        unknown_int_2, set_unknown_int_2: u32 = hinged_action_unknown_int_2, set_hinged_action_unknown_int_2;
        /// The hinge axis in the body's space, `[x, y, z, w]`.
        hinge_axis, set_hinge_axis: [f32; 4] = hinged_action_hinge_axis, set_hinged_action_hinge_axis;
        /// The forward direction in the body's space, `[x, y, z, w]`.
        forward, set_forward: [f32; 4] = hinged_action_forward, set_hinged_action_forward;
        strength, set_strength: f32 = hinged_action_strength, set_hinged_action_strength;
        damping, set_damping: f32 = hinged_action_damping, set_hinged_action_damping;
    }
}

/// A block with a collision [shape](View::shape): a [`BhkWorldObject`], [`BhkTransformShape`] or
/// [`BhkMoppBvTreeShape`], or one of their subtypes.
///
/// This trait is sealed.
pub trait HasShape: NiType {}

impl HasShape for BhkWorldObject {}
impl HasShape for BhkPhantom {}
impl HasShape for BhkShapePhantom {}
impl HasShape for BhkSimpleShapePhantom {}
impl HasShape for BhkAabbPhantom {}
impl HasShape for BhkEntity {}
impl HasShape for BhkRigidBody {}
impl HasShape for BhkRigidBodyT {}

/// Shapes, constraints and hinged bodies.
impl<'a, T: NiType> View<'a, T> {
    /// The block's collision shape, or for a transform or MOPP shape its child shape. `None` if
    /// it has none.
    pub fn shape(&self) -> Option<View<'a, BhkShape>>
    where
        T: HasShape,
    {
        self.follow(RefKind::HavokShape)
    }

    /// The rigid body's constraints: [`BhkConstraint`]s and [`BhkBallSocketConstraintChain`]s.
    /// Skips empty refs.
    pub fn constraints<I>(&self) -> BlockIter<'a, BhkSerializable, RefIndices>
    where
        T: IsA<BhkRigidBody, I>,
    {
        self.follow_list(RefList::Constraints)
    }

    /// The rigid body the action keeps oriented, or `None` if it has none.
    pub fn hinged_body<I>(&self) -> Option<View<'a, BhkRigidBody>>
    where
        T: IsA<BhkOrientHingedBodyAction, I>,
    {
        self.follow(RefKind::HingedBody)
    }
}

impl<T: NiType> Edit<'_, T> {
    /// Like [`View::shape`], for editing.
    pub fn shape(&mut self) -> Option<Edit<'_, BhkShape>>
    where
        T: HasShape,
    {
        self.follow(RefKind::HavokShape)
    }

    /// Sets the block's [shape](View::shape), or clears it with `None`. Returns `false`, changing
    /// nothing, unless `shape` is a [`BhkShape`] that doesn't contain this block.
    pub fn set_shape(&mut self, shape: Option<Index>) -> bool
    where
        T: HasShape,
    {
        self.set_ref(RefKind::HavokShape, shape)
    }

    /// Replaces the rigid body's [constraints](View::constraints). Returns `false`, changing
    /// nothing, unless each is a [`BhkConstraint`] or [`BhkBallSocketConstraintChain`].
    pub fn set_constraints<I>(&mut self, constraints: &[Index]) -> bool
    where
        T: IsA<BhkRigidBody, I>,
    {
        self.set_ref_list(RefList::Constraints, constraints)
    }

    /// Like [`View::hinged_body`], for editing.
    pub fn hinged_body<I>(&mut self) -> Option<Edit<'_, BhkRigidBody>>
    where
        T: IsA<BhkOrientHingedBodyAction, I>,
    {
        self.follow(RefKind::HingedBody)
    }

    /// Sets the action's [hinged body](View::hinged_body), or clears it with `None`. Returns
    /// `false`, changing nothing, unless `body` is a [`BhkRigidBody`].
    pub fn set_hinged_body<I>(&mut self, body: Option<Index>) -> bool
    where
        T: IsA<BhkOrientHingedBodyAction, I>,
    {
        self.set_ref(RefKind::HingedBody, body)
    }
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

/// An action type that [`NifFile::add_action`] can create.
///
/// This trait is sealed.
pub trait NewAction: NiType {
    #[doc(hidden)]
    fn add(file: Pin<&mut nif_ffi::NifFile>) -> u32;
}

impl NewAction for BhkLiquidAction {
    fn add(file: Pin<&mut nif_ffi::NifFile>) -> u32 {
        ffi::add_bhk_liquid_action(file)
    }
}

impl NewAction for BhkOrientHingedBodyAction {
    fn add(file: Pin<&mut nif_ffi::NifFile>) -> u32 {
        ffi::add_bhk_orient_hinged_body_action(file)
    }
}

impl BhkHeightFieldShape {
    fields! {
        /// The Havok material, as a raw value whose meaning depends on the game.
        material, set_material: u32 = height_field_material, set_height_field_material;
    }
}

/// Vectors are `[x, y, z, w]` in Havok units.
impl BhkPlaneShape {
    converted_fields! {
        /// A vector of unknown meaning.
        unknown_vector, set_unknown_vector: Vec3 =
            plane_shape_unknown_vector, set_plane_shape_unknown_vector, Vec3::from_ffi, Vec3::to_ffi;
    }

    /// The plane.
    pub fn plane(&self) -> Plane {
        Plane {
            normal: Vec3::from_ffi(ffi::plane_shape_normal(&self.0)),
            constant: ffi::plane_shape_constant(&self.0),
        }
    }

    /// Sets the [plane](Self::plane).
    pub fn set_plane(mut self: Pin<&mut Self>, plane: Plane) {
        ffi::set_plane_shape_normal(self.as_mut().ffi_mut(), plane.normal.to_ffi());
        ffi::set_plane_shape_constant(self.ffi_mut(), plane.constant);
    }

    fields! {
        /// The half extents of the part of the plane that's used.
        half_extents, set_half_extents: [f32; 4] = plane_shape_half_extents, set_plane_shape_half_extents;
        /// The center of the part of the plane that's used.
        center, set_center: [f32; 4] = plane_shape_center, set_plane_shape_center;
    }
}

impl BhkSphereRepShape {
    fields! {
        /// The Havok material, as a raw value whose meaning depends on the game.
        material, set_material: u32 = sphere_rep_material, set_sphere_rep_material;
    }
}

impl BhkConvexShape {
    fields! {
        /// The radius around the shape, in Havok units. For a [`BhkSphereShape`], the sphere's
        /// radius.
        radius, set_radius: f32 = convex_shape_radius, set_convex_shape_radius;
    }
}

impl BhkMultiSphereShape {
    converted_fields! {
        shape_property, set_shape_property: CInfoProperty =
            multi_sphere_property, set_multi_sphere_property, CInfoProperty::from_ffi, CInfoProperty::to_ffi;
    }

    /// The spheres, in Havok units.
    pub fn spheres(&self) -> Vec<BoundingSphere> {
        ffi::multi_sphere_spheres(&self.0).into_iter().map(BoundingSphere::from_ffi).collect()
    }

    pub fn set_spheres(self: Pin<&mut Self>, spheres: &[BoundingSphere]) {
        let spheres: Vec<_> = spheres.iter().map(|s| s.to_ffi()).collect();
        ffi::set_multi_sphere_spheres(self.ffi_mut(), &spheres);
    }
}

/// Vectors are `[x, y, z, w]` in Havok units.
impl BhkConvexVerticesShape {
    converted_fields! {
        vertices_property, set_vertices_property: CInfoProperty =
            convex_vertices_vertices_property, set_convex_vertices_vertices_property,
            CInfoProperty::from_ffi, CInfoProperty::to_ffi;
        normals_property, set_normals_property: CInfoProperty =
            convex_vertices_normals_property, set_convex_vertices_normals_property,
            CInfoProperty::from_ffi, CInfoProperty::to_ffi;
    }

    /// The vertices whose convex hull is the shape.
    pub fn vertices(&self) -> Vec<[f32; 4]> {
        ffi::convex_vertices_list(&self.0, false).as_chunks::<4>().0.to_vec()
    }

    pub fn set_vertices(self: Pin<&mut Self>, vertices: &[[f32; 4]]) {
        ffi::set_convex_vertices_list(self.ffi_mut(), false, vertices.as_flattened());
    }

    /// The hull's faces as planes: an outward normal in `x`, `y` and `z`, and the plane's
    /// distance from the origin, negated, in `w`. They aren't updated when the
    /// [vertices](Self::vertices) change.
    pub fn normals(&self) -> Vec<[f32; 4]> {
        ffi::convex_vertices_list(&self.0, true).as_chunks::<4>().0.to_vec()
    }

    pub fn set_normals(self: Pin<&mut Self>, normals: &[[f32; 4]]) {
        ffi::set_convex_vertices_list(self.ffi_mut(), true, normals.as_flattened());
    }
}

impl BhkBoxShape {
    converted_fields! {
        /// Half the box's size along each axis, in Havok units.
        dimensions, set_dimensions: Vec3 = box_shape_dimensions, set_box_shape_dimensions, Vec3::from_ffi, Vec3::to_ffi;
    }
}

/// Points are `[x, y, z, w]` in Havok units.
impl BhkCylinderShape {
    fields! {
        /// The center of one end.
        vertex_a, set_vertex_a: [f32; 4] = cylinder_shape_vertex_a, set_cylinder_shape_vertex_a;
        /// The center of the other end.
        vertex_b, set_vertex_b: [f32; 4] = cylinder_shape_vertex_b, set_cylinder_shape_vertex_b;
        /// The cylinder's radius.
        cylinder_radius, set_cylinder_radius: f32 = cylinder_shape_radius, set_cylinder_shape_radius;
    }
}

/// The capsule's own radius is its [convex radius](BhkConvexShape::radius). Points are in Havok
/// units.
impl BhkCapsuleShape {
    converted_fields! {
        /// The center of one end.
        point_1, set_point_1: Vec3 = capsule_shape_point_1, set_capsule_shape_point_1, Vec3::from_ffi, Vec3::to_ffi;
        /// The center of the other end.
        point_2, set_point_2: Vec3 = capsule_shape_point_2, set_capsule_shape_point_2, Vec3::from_ffi, Vec3::to_ffi;
    }

    fields! {
        /// The radius at [`point_1`](Self::point_1).
        radius_1, set_radius_1: f32 = capsule_shape_radius_1, set_capsule_shape_radius_1;
        /// The radius at [`point_2`](Self::point_2).
        radius_2, set_radius_2: f32 = capsule_shape_radius_2, set_capsule_shape_radius_2;
    }
}

/// Its child shape is on [`View::shape`].
impl BhkTransformShape {
    fields! {
        /// The Havok material, as a raw value whose meaning depends on the game.
        material, set_material: u32 = transform_shape_material, set_transform_shape_material;
        /// The radius around the shape, in Havok units.
        radius, set_radius: f32 = transform_shape_radius, set_transform_shape_radius;
    }

    converted_fields! {
        /// The transform applied to the child shape, as four rows in Havok units.
        transform, set_transform: [[f32; 4]; 4] =
            transform_shape_transform, set_transform_shape_transform, rows, flatten;
    }
}

/// Its sub-shapes are on [`View::sub_shapes`].
impl BhkListShape {
    fields! {
        /// The Havok material, as a raw value whose meaning depends on the game.
        material, set_material: u32 = list_shape_material, set_list_shape_material;
    }

    converted_fields! {
        child_shape_property, set_child_shape_property: CInfoProperty =
            list_shape_child_shape_property, set_list_shape_child_shape_property,
            CInfoProperty::from_ffi, CInfoProperty::to_ffi;
        child_filter_property, set_child_filter_property: CInfoProperty =
            list_shape_child_filter_property, set_list_shape_child_filter_property,
            CInfoProperty::from_ffi, CInfoProperty::to_ffi;
    }

    /// The sub-shapes' collision filters, usually one per sub-shape.
    pub fn filters(&self) -> Vec<HavokFilter> {
        ffi::list_shape_filters(&self.0).into_iter().map(HavokFilter::from_ffi).collect()
    }

    /// Replaces the [filters](Self::filters). Returns `false`, changing nothing, unless there's
    /// one per sub-shape.
    pub fn set_filters(self: Pin<&mut Self>, filters: &[HavokFilter]) -> bool {
        let filters: Vec<_> = filters.iter().map(|f| f.to_ffi()).collect();
        ffi::set_list_shape_filters(self.ffi_mut(), &filters)
    }
}

/// Its sub-shapes are on [`View::sub_shapes`].
impl BhkConvexListShape {
    fields! {
        /// The Havok material, as a raw value whose meaning depends on the game.
        material, set_material: u32 = convex_list_material, set_convex_list_material;
        /// The radius around the shape, in Havok units.
        radius, set_radius: f32 = convex_list_radius, set_convex_list_radius;
        /// A value of unknown meaning.
        unknown_int, set_unknown_int: u32 = convex_list_unknown_int, set_convex_list_unknown_int;
        /// A value of unknown meaning.
        unknown_float, set_unknown_float: f32 = convex_list_unknown_float, set_convex_list_unknown_float;
        use_cached_aabb, set_use_cached_aabb: bool = convex_list_use_cached_aabb, set_convex_list_use_cached_aabb;
        closest_point_min_distance, set_closest_point_min_distance: f32 =
            convex_list_closest_point_min_distance, set_convex_list_closest_point_min_distance;
    }

    converted_fields! {
        child_shape_property, set_child_shape_property: CInfoProperty =
            convex_list_child_shape_property, set_convex_list_child_shape_property,
            CInfoProperty::from_ffi, CInfoProperty::to_ffi;
    }
}

/// Its child shape is on [`View::shape`]. The MOPP code describes that shape, and isn't rebuilt
/// when it or the shape changes.
impl BhkMoppBvTreeShape {
    /// The MOPP code.
    pub fn mopp_code(&self) -> &[u8] {
        ffi::mopp_code(&self.0)
    }

    pub fn set_mopp_code(self: Pin<&mut Self>, code: &[u8]) {
        ffi::set_mopp_code(self.ffi_mut(), code);
    }

    fields! {
        /// The code's origin, `[x, y, z, w]` in Havok units.
        mopp_offset, set_mopp_offset: [f32; 4] = mopp_offset, set_mopp_offset;
        /// The code's scale.
        mopp_scale, set_mopp_scale: f32 = mopp_scale, set_mopp_scale;
        /// How the code was built, as a raw value. Only saved from Fallout 3 on.
        build_type, set_build_type: u8 = mopp_build_type, set_mopp_build_type;
    }
}

/// Its [`NiTriStripsData`] blocks are on [`View::strips_data`].
impl BhkNiTriStripsShape {
    fields! {
        /// The Havok material, as a raw value whose meaning depends on the game.
        material, set_material: u32 = strips_shape_material, set_strips_shape_material;
        /// The radius around the shape, in Havok units.
        radius, set_radius: f32 = strips_shape_radius, set_strips_shape_radius;
        grow_by, set_grow_by: u32 = strips_shape_grow_by, set_strips_shape_grow_by;
        /// The scale, `[x, y, z, w]`.
        scale, set_scale: [f32; 4] = strips_shape_scale, set_strips_shape_scale;
    }

    /// Each strips data block's collision filter, as a raw value, usually one per block.
    pub fn filters(&self) -> &[u32] {
        ffi::strips_shape_filters(&self.0)
    }

    /// Replaces the [filters](Self::filters). Returns `false`, changing nothing, unless there's
    /// one per [strips data](View::strips_data) ref.
    pub fn set_filters(self: Pin<&mut Self>, filters: &[u32]) -> bool {
        ffi::set_strips_shape_filters(self.ffi_mut(), filters)
    }
}

/// A part of a packed mesh: a run of vertices with its own filter and material.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SubPart {
    pub filter: HavokFilter,
    pub vertex_count: u32,
    /// The Havok material, as a raw value whose meaning depends on the game.
    pub material: u32,
}

impl SubPart {
    fn from_ffi(p: ffi::SubPartFfi) -> Self {
        Self { filter: HavokFilter::from_ffi(p.filter), vertex_count: p.vertex_count, material: p.material }
    }

    fn to_ffi(self) -> ffi::SubPartFfi {
        ffi::SubPartFfi { filter: self.filter.to_ffi(), vertex_count: self.vertex_count, material: self.material }
    }
}

fn sub_parts_from_ffi(parts: Vec<ffi::SubPartFfi>) -> Vec<SubPart> {
    parts.into_iter().map(SubPart::from_ffi).collect()
}

fn sub_parts_to_ffi(parts: &[SubPart]) -> Vec<ffi::SubPartFfi> {
    parts.iter().map(|p| p.to_ffi()).collect()
}

/// Its data is on [`View::packed_data`].
impl BhkPackedNiTriStripsShape {
    /// The mesh's sub-parts. Only saved up to Oblivion; later files save the
    /// [data's](HkPackedNiTriStripsData::sub_parts) instead.
    pub fn sub_parts(&self) -> Vec<SubPart> {
        sub_parts_from_ffi(ffi::packed_shape_sub_parts(&self.0))
    }

    /// Replaces the [sub-parts](Self::sub_parts). Returns `false`, changing nothing, for more
    /// than 65535.
    pub fn set_sub_parts(self: Pin<&mut Self>, parts: &[SubPart]) -> bool {
        ffi::set_packed_shape_sub_parts(self.ffi_mut(), &sub_parts_to_ffi(parts))
    }

    fields! {
        user_data, set_user_data: u32 = packed_shape_user_data, set_packed_shape_user_data;
        /// The radius around the shape, in Havok units.
        radius, set_radius: f32 = packed_shape_radius, set_packed_shape_radius;
        /// The scale, `[x, y, z, w]`.
        scale, set_scale: [f32; 4] = packed_shape_scale, set_packed_shape_scale;
        /// Usually the same as [`radius`](Self::radius).
        radius_copy, set_radius_copy: f32 = packed_shape_radius_copy, set_packed_shape_radius_copy;
        /// Usually the same as [`scale`](Self::scale).
        scale_copy, set_scale_copy: [f32; 4] = packed_shape_scale_copy, set_packed_shape_scale_copy;
    }
}

/// A triangle of an [`HkPackedNiTriStripsData`].
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PackedTriangle {
    /// Vertex indices.
    pub triangle: [u16; 3],
    pub welding_info: u16,
    /// The face normal. Only saved up to Oblivion.
    pub normal: Vec3,
}

/// Vertices are in Havok units.
impl HkPackedNiTriStripsData {
    /// The triangles. Normals are zero when read from a file that doesn't save them.
    pub fn triangles(&self) -> Vec<PackedTriangle> {
        ffi::packed_data_triangles(&self.0)
            .into_iter()
            .map(|t| PackedTriangle {
                triangle: [t.p1, t.p2, t.p3],
                welding_info: t.welding_info,
                normal: Vec3::from_ffi(t.normal),
            })
            .collect()
    }

    /// Replaces the [triangles](Self::triangles). Returns `false`, changing nothing, if an index
    /// isn't below the vertex count.
    pub fn set_triangles(self: Pin<&mut Self>, triangles: &[PackedTriangle]) -> bool {
        let triangles: Vec<_> = triangles
            .iter()
            .map(|t| ffi::PackedTriangleFfi {
                p1: t.triangle[0],
                p2: t.triangle[1],
                p3: t.triangle[2],
                welding_info: t.welding_info,
                normal: t.normal.to_ffi(),
            })
            .collect();
        ffi::set_packed_data_triangles(self.ffi_mut(), &triangles)
    }

    pub fn vertices(&self) -> Vec<Vec3> {
        ffi::packed_data_vertices(&self.0).into_iter().map(Vec3::from_ffi).collect()
    }

    /// Replaces the vertices, changing the vertex count. Returns `false`, changing nothing, if a
    /// triangle uses a vertex past the new count.
    pub fn set_vertices(self: Pin<&mut Self>, vertices: &[Vec3]) -> bool {
        let vertices: Vec<_> = vertices.iter().map(|v| v.to_ffi()).collect();
        ffi::set_packed_data_vertices(self.ffi_mut(), &vertices)
    }

    /// The mesh's sub-parts. Only saved after Oblivion; older files save the
    /// [shape's](BhkPackedNiTriStripsShape::sub_parts) instead.
    pub fn sub_parts(&self) -> Vec<SubPart> {
        sub_parts_from_ffi(ffi::packed_data_sub_parts(&self.0))
    }

    /// Replaces the [sub-parts](Self::sub_parts). Returns `false`, changing nothing, for more
    /// than 65535.
    pub fn set_sub_parts(self: Pin<&mut Self>, parts: &[SubPart]) -> bool {
        ffi::set_packed_data_sub_parts(self.ffi_mut(), &sub_parts_to_ffi(parts))
    }
}

/// Its data is on [`View::compressed_data`] and the scene object it was made from on
/// [`View::mesh_target`].
impl BhkCompressedMeshShape {
    fields! {
        user_data, set_user_data: u32 = compressed_shape_user_data, set_compressed_shape_user_data;
        /// The radius around the shape, in Havok units.
        radius, set_radius: f32 = compressed_shape_radius, set_compressed_shape_radius;
        /// A value of unknown meaning.
        unknown_float, set_unknown_float: f32 = compressed_shape_unknown_float, set_compressed_shape_unknown_float;
        /// The scale, `[x, y, z, w]`.
        scale, set_scale: [f32; 4] = compressed_shape_scale, set_compressed_shape_scale;
        /// Usually the same as [`radius`](Self::radius).
        radius_copy, set_radius_copy: f32 = compressed_shape_radius_copy, set_compressed_shape_radius_copy;
        /// Usually the same as [`scale`](Self::scale).
        scale_copy, set_scale_copy: [f32; 4] = compressed_shape_scale_copy, set_compressed_shape_scale_copy;
    }
}

/// A material of a [`BhkCompressedMeshShapeData`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ChunkMaterial {
    /// The Havok material, as a raw value whose meaning depends on the game.
    pub material: u32,
    pub filter: HavokFilter,
}

/// A transform of a [`BhkCompressedMeshShapeData`].
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ChunkTransform {
    /// `[x, y, z, w]` in Havok units.
    pub translation: [f32; 4],
    /// A quaternion, `[x, y, z, w]`.
    pub rotation: [f32; 4],
}

/// A triangle of a [`BhkCompressedMeshShapeData`] stored outside its chunks.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct BigTriangle {
    /// Indices into the [big vertices](BhkCompressedMeshShapeData::big_vertices).
    pub triangle: [u16; 3],
    /// The Havok material, as a raw value.
    pub material: u32,
    pub welding_info: u16,
}

/// A chunk of a [`BhkCompressedMeshShapeData`]: compressed vertices and the triangles made of
/// them.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MeshChunk {
    /// The offset added to the vertices, `[x, y, z, w]` in Havok units.
    pub translation: [f32; 4],
    /// An index into the [materials](BhkCompressedMeshShapeData::chunk_materials).
    pub material_index: u32,
    pub reference: u16,
    /// An index into the [transforms](BhkCompressedMeshShapeData::chunk_transforms).
    pub transform_index: u16,
    /// Vertex positions as `x`, `y`, `z` triples, in thousandths of a Havok unit from the
    /// translation.
    pub vertices: Vec<u16>,
    pub indices: Vec<u16>,
    /// The length of each strip at the start of the indices; the rest are plain triangles.
    pub strips: Vec<u16>,
    pub welding_info: Vec<u16>,
}

/// Lists and fields aren't checked against each other, except big triangles against big
/// vertices.
impl BhkCompressedMeshShapeData {
    fields! {
        bits_per_index, set_bits_per_index: u32 = cms_bits_per_index, set_cms_bits_per_index;
        bits_per_welding_index, set_bits_per_welding_index: u32 =
            cms_bits_per_welding_index, set_cms_bits_per_welding_index;
        mask_welding_index, set_mask_welding_index: u32 = cms_mask_welding_index, set_cms_mask_welding_index;
        mask_index, set_mask_index: u32 = cms_mask_index, set_cms_mask_index;
        /// The largest error compression introduced, in Havok units.
        error, set_error: f32 = cms_error, set_cms_error;
        /// The minimum corner of the bounding box, `[x, y, z, w]` in Havok units.
        aabb_min, set_aabb_min: [f32; 4] = cms_aabb_min, set_cms_aabb_min;
        /// The maximum corner of the bounding box, `[x, y, z, w]` in Havok units.
        aabb_max, set_aabb_max: [f32; 4] = cms_aabb_max, set_cms_aabb_max;
        /// The welding type, as a raw value.
        welding_type, set_welding_type: u8 = cms_welding_type, set_cms_welding_type;
        /// The material type, as a raw value.
        material_type, set_material_type: u8 = cms_material_type, set_cms_material_type;
        named_material_count, set_named_material_count: u32 =
            cms_named_material_count, set_cms_named_material_count;
        convex_piece_count, set_convex_piece_count: u32 = cms_convex_piece_count, set_cms_convex_piece_count;
    }

    /// The 32-bit material list, usually empty.
    pub fn materials_32(&self) -> &[u32] {
        ffi::cms_materials_n(&self.0, 0)
    }

    pub fn set_materials_32(self: Pin<&mut Self>, materials: &[u32]) {
        ffi::set_cms_materials_n(self.ffi_mut(), 0, materials);
    }

    /// The 16-bit material list, usually empty.
    pub fn materials_16(&self) -> &[u32] {
        ffi::cms_materials_n(&self.0, 1)
    }

    pub fn set_materials_16(self: Pin<&mut Self>, materials: &[u32]) {
        ffi::set_cms_materials_n(self.ffi_mut(), 1, materials);
    }

    /// The 8-bit material list, usually empty.
    pub fn materials_8(&self) -> &[u32] {
        ffi::cms_materials_n(&self.0, 2)
    }

    pub fn set_materials_8(self: Pin<&mut Self>, materials: &[u32]) {
        ffi::set_cms_materials_n(self.ffi_mut(), 2, materials);
    }

    /// The materials chunks refer to.
    pub fn chunk_materials(&self) -> Vec<ChunkMaterial> {
        ffi::cms_materials(&self.0)
            .into_iter()
            .map(|m| ChunkMaterial { material: m.material, filter: HavokFilter::from_ffi(m.filter) })
            .collect()
    }

    pub fn set_chunk_materials(self: Pin<&mut Self>, materials: &[ChunkMaterial]) {
        let materials: Vec<_> = materials
            .iter()
            .map(|m| ffi::CmsMaterialFfi { material: m.material, filter: m.filter.to_ffi() })
            .collect();
        ffi::set_cms_materials(self.ffi_mut(), &materials);
    }

    /// The transforms chunks refer to.
    pub fn chunk_transforms(&self) -> Vec<ChunkTransform> {
        ffi::cms_transforms(&self.0)
            .into_iter()
            .map(|t| ChunkTransform { translation: t.translation, rotation: t.rotation })
            .collect()
    }

    pub fn set_chunk_transforms(self: Pin<&mut Self>, transforms: &[ChunkTransform]) {
        let transforms: Vec<_> = transforms
            .iter()
            .map(|t| ffi::CmsTransformFfi { translation: t.translation, rotation: t.rotation })
            .collect();
        ffi::set_cms_transforms(self.ffi_mut(), &transforms);
    }

    /// Uncompressed vertices, `[x, y, z, w]` in Havok units, for the
    /// [big triangles](Self::big_triangles).
    pub fn big_vertices(&self) -> Vec<[f32; 4]> {
        ffi::cms_big_vertices(&self.0).as_chunks::<4>().0.to_vec()
    }

    /// Replaces the [big vertices](Self::big_vertices). Returns `false`, changing nothing, if a
    /// big triangle uses a vertex past the new count.
    pub fn set_big_vertices(self: Pin<&mut Self>, vertices: &[[f32; 4]]) -> bool {
        ffi::set_cms_big_vertices(self.ffi_mut(), vertices.as_flattened())
    }

    /// Triangles too large for a chunk.
    pub fn big_triangles(&self) -> Vec<BigTriangle> {
        ffi::cms_big_triangles(&self.0)
            .into_iter()
            .map(|t| BigTriangle { triangle: [t.p1, t.p2, t.p3], material: t.material, welding_info: t.welding_info })
            .collect()
    }

    /// Replaces the [big triangles](Self::big_triangles). Returns `false`, changing nothing, if
    /// an index isn't below the big vertex count.
    pub fn set_big_triangles(self: Pin<&mut Self>, triangles: &[BigTriangle]) -> bool {
        let triangles: Vec<_> = triangles
            .iter()
            .map(|t| ffi::CmsBigTriangleFfi {
                p1: t.triangle[0],
                p2: t.triangle[1],
                p3: t.triangle[2],
                material: t.material,
                welding_info: t.welding_info,
            })
            .collect();
        ffi::set_cms_big_triangles(self.ffi_mut(), &triangles)
    }

    pub fn chunks(&self) -> Vec<MeshChunk> {
        (0..ffi::cms_chunk_count(&self.0))
            .map(|i| {
                let c = ffi::cms_chunk(&self.0, i);
                let list = |which| ffi::cms_chunk_list(&self.0, i, which).to_vec();
                MeshChunk {
                    translation: c.translation,
                    material_index: c.material_index,
                    reference: c.reference,
                    transform_index: c.transform_index,
                    vertices: list(0),
                    indices: list(1),
                    strips: list(2),
                    welding_info: list(3),
                }
            })
            .collect()
    }

    pub fn set_chunks(self: Pin<&mut Self>, chunks: &[MeshChunk]) {
        let mut headers = Vec::with_capacity(chunks.len());
        let mut lists = Vec::new();
        let mut lengths = Vec::with_capacity(chunks.len() * 4);
        for c in chunks {
            headers.push(ffi::CmsChunkFfi {
                translation: c.translation,
                material_index: c.material_index,
                reference: c.reference,
                transform_index: c.transform_index,
            });
            for list in [&c.vertices, &c.indices, &c.strips, &c.welding_info] {
                lists.extend_from_slice(list);
                lengths.push(u32::try_from(list.len()).expect("a chunk list fits in u32"));
            }
        }
        assert!(ffi::set_cms_chunks(self.ffi_mut(), &headers, &lists, &lengths), "lengths add up");
    }
}

impl HasShape for BhkTransformShape {}
impl HasShape for BhkConvexTransformShape {}
impl HasShape for BhkMoppBvTreeShape {}

/// A shape with a list of sub-shapes: a [`BhkListShape`] or [`BhkConvexListShape`].
///
/// This trait is sealed.
pub trait HasSubShapes: NiType {}

impl HasSubShapes for BhkListShape {}
impl HasSubShapes for BhkConvexListShape {}

/// Sub-shapes and mesh data.
impl<'a, T: NiType> View<'a, T> {
    /// The list shape's sub-shapes. Skips empty refs.
    pub fn sub_shapes(&self) -> BlockIter<'a, BhkShape, RefIndices>
    where
        T: HasSubShapes,
    {
        self.follow_list(RefList::SubShapes)
    }

    /// The strips shape's triangle data. Skips empty refs.
    pub fn strips_data<I>(&self) -> BlockIter<'a, NiTriStripsData, RefIndices>
    where
        T: IsA<BhkNiTriStripsShape, I>,
    {
        self.follow_list(RefList::StripsData)
    }

    /// The scene object the compressed mesh shape was made from, or `None` if it has none.
    pub fn mesh_target<I>(&self) -> Option<View<'a, NiAvObject>>
    where
        T: IsA<BhkCompressedMeshShape, I>,
    {
        self.follow(RefKind::CompressedMeshTarget)
    }

    /// The compressed mesh shape's data, or `None` if it has none.
    pub fn compressed_data<I>(&self) -> Option<View<'a, BhkCompressedMeshShapeData>>
    where
        T: IsA<BhkCompressedMeshShape, I>,
    {
        self.follow(RefKind::CompressedMeshData)
    }

    /// The packed shape's data, or `None` if it has none.
    pub fn packed_data<I>(&self) -> Option<View<'a, HkPackedNiTriStripsData>>
    where
        T: IsA<BhkPackedNiTriStripsShape, I>,
    {
        self.follow(RefKind::PackedData)
    }
}

impl<T: NiType> Edit<'_, T> {
    /// Adds a new shape of type `S` with default settings as the block's [shape](View::shape),
    /// and returns its index. A previous shape stays in the file until it's optimized away.
    ///
    /// ```
    /// # use nifly::{collision::*, file::{NifFile, NifVersion}};
    /// let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    /// let collision = nif.root_node_mut().unwrap().assign_collision::<BhkCollisionObject, _>();
    /// let mut collision = nif.edit::<BhkCollisionObject>(collision).unwrap();
    /// let body = collision.assign_body::<BhkRigidBody, _>().unwrap();
    /// let shape = nif.edit::<BhkRigidBody>(body).unwrap().assign_shape::<BhkBoxShape>();
    /// assert_eq!(nif.view::<BhkRigidBody>(body).unwrap().shape().unwrap().index(), shape);
    /// ```
    pub fn assign_shape<S: NewShape>(&mut self) -> Index
    where
        T: HasShape,
    {
        let owner = self.index() as u32;
        index_from_ffi(S::assign(self.file_mut().inner.pin_mut(), owner)).expect("the block has a shape ref")
    }

    /// Replaces the list shape's [sub-shapes](View::sub_shapes). A [`BhkListShape`]'s
    /// [filters](BhkListShape::filters) are resized to match if there was one per sub-shape,
    /// new ones default.
    ///
    /// Returns `false`, changing nothing, unless each is a [`BhkShape`] (a [`BhkConvexShape`]
    /// for a [`BhkConvexListShape`]) that doesn't contain this shape.
    pub fn set_sub_shapes(&mut self, shapes: &[Index]) -> bool
    where
        T: HasSubShapes,
    {
        self.set_ref_list(RefList::SubShapes, shapes)
    }

    /// Adds a new shape of type `S` with default settings at the end of the list shape's
    /// [sub-shapes](View::sub_shapes), and returns its index. A [`BhkListShape`] with one
    /// [filter](BhkListShape::filters) per sub-shape gets a default one for it.
    ///
    /// Returns `None`, changing nothing, if this is a [`BhkConvexListShape`] and `S` isn't a
    /// [`BhkConvexShape`].
    pub fn add_sub_shape<S: NewShape>(&mut self) -> Option<Index>
    where
        T: HasSubShapes,
    {
        let owner = self.index() as u32;
        index_from_ffi(S::add_sub(self.file_mut().inner.pin_mut(), owner))
    }

    /// Replaces the strips shape's [triangle data](View::strips_data). Its
    /// [filters](BhkNiTriStripsShape::filters) are resized to match if there was one per ref,
    /// new ones zero. Returns `false`, changing nothing, unless each is an [`NiTriStripsData`].
    pub fn set_strips_data<I>(&mut self, data: &[Index]) -> bool
    where
        T: IsA<BhkNiTriStripsShape, I>,
    {
        self.set_ref_list(RefList::StripsData, data)
    }

    /// Like [`View::mesh_target`], for editing.
    pub fn mesh_target<I>(&mut self) -> Option<Edit<'_, NiAvObject>>
    where
        T: IsA<BhkCompressedMeshShape, I>,
    {
        self.follow(RefKind::CompressedMeshTarget)
    }

    /// Sets the compressed mesh shape's [target](View::mesh_target), or clears it with `None`.
    /// Returns `false`, changing nothing, unless `target` is an [`NiAvObject`].
    pub fn set_mesh_target<I>(&mut self, target: Option<Index>) -> bool
    where
        T: IsA<BhkCompressedMeshShape, I>,
    {
        self.set_ref(RefKind::CompressedMeshTarget, target)
    }

    /// Like [`View::compressed_data`], for editing.
    pub fn compressed_data<I>(&mut self) -> Option<Edit<'_, BhkCompressedMeshShapeData>>
    where
        T: IsA<BhkCompressedMeshShape, I>,
    {
        self.follow(RefKind::CompressedMeshData)
    }

    /// Sets the compressed mesh shape's [data](View::compressed_data), or clears it with
    /// `None`. Returns `false`, changing nothing, unless `data` is a
    /// [`BhkCompressedMeshShapeData`].
    pub fn set_compressed_data<I>(&mut self, data: Option<Index>) -> bool
    where
        T: IsA<BhkCompressedMeshShape, I>,
    {
        self.set_ref(RefKind::CompressedMeshData, data)
    }

    /// Adds new, empty [data](View::compressed_data) to the compressed mesh shape and returns
    /// its index. The previous data stays in the file until it's optimized away.
    pub fn assign_compressed_data<I>(&mut self) -> Index
    where
        T: IsA<BhkCompressedMeshShape, I>,
    {
        let shape = self.index() as u32;
        index_from_ffi(ffi::assign_compressed_data(self.file_mut().inner.pin_mut(), shape))
            .expect("the block is a compressed mesh shape")
    }

    /// Like [`View::packed_data`], for editing.
    pub fn packed_data<I>(&mut self) -> Option<Edit<'_, HkPackedNiTriStripsData>>
    where
        T: IsA<BhkPackedNiTriStripsShape, I>,
    {
        self.follow(RefKind::PackedData)
    }

    /// Sets the packed shape's [data](View::packed_data), or clears it with `None`. Returns
    /// `false`, changing nothing, unless `data` is an [`HkPackedNiTriStripsData`].
    pub fn set_packed_data<I>(&mut self, data: Option<Index>) -> bool
    where
        T: IsA<BhkPackedNiTriStripsShape, I>,
    {
        self.set_ref(RefKind::PackedData, data)
    }

    /// Adds new, empty [data](View::packed_data) to the packed shape and returns its index. The
    /// previous data stays in the file until it's optimized away.
    pub fn assign_packed_data<I>(&mut self) -> Index
    where
        T: IsA<BhkPackedNiTriStripsShape, I>,
    {
        let shape = self.index() as u32;
        index_from_ffi(ffi::assign_packed_data(self.file_mut().inner.pin_mut(), shape))
            .expect("the block is a packed shape")
    }
}

/// A shape type that [`Edit::assign_shape`] and [`Edit::add_sub_shape`] can create.
///
/// This trait is sealed.
pub trait NewShape: NiType {
    #[doc(hidden)]
    fn assign(file: Pin<&mut nif_ffi::NifFile>, owner: u32) -> u32;
    #[doc(hidden)]
    fn add_sub(file: Pin<&mut nif_ffi::NifFile>, owner: u32) -> u32;
}

macro_rules! new_shape {
    ($($ty:ty => $assign:ident, $add_sub:ident;)*) => {$(
        impl NewShape for $ty {
            fn assign(file: Pin<&mut nif_ffi::NifFile>, owner: u32) -> u32 {
                ffi::$assign(file, owner)
            }

            fn add_sub(file: Pin<&mut nif_ffi::NifFile>, owner: u32) -> u32 {
                ffi::$add_sub(file, owner)
            }
        }
    )*};
}

new_shape! {
    BhkPlaneShape => assign_bhk_plane_shape, add_sub_bhk_plane_shape;
    BhkMultiSphereShape => assign_bhk_multi_sphere_shape, add_sub_bhk_multi_sphere_shape;
    BhkConvexListShape => assign_bhk_convex_list_shape, add_sub_bhk_convex_list_shape;
    BhkConvexVerticesShape => assign_bhk_convex_vertices_shape, add_sub_bhk_convex_vertices_shape;
    BhkBoxShape => assign_bhk_box_shape, add_sub_bhk_box_shape;
    BhkSphereShape => assign_bhk_sphere_shape, add_sub_bhk_sphere_shape;
    BhkCylinderShape => assign_bhk_cylinder_shape, add_sub_bhk_cylinder_shape;
    BhkTransformShape => assign_bhk_transform_shape, add_sub_bhk_transform_shape;
    BhkConvexTransformShape => assign_bhk_convex_transform_shape, add_sub_bhk_convex_transform_shape;
    BhkCapsuleShape => assign_bhk_capsule_shape, add_sub_bhk_capsule_shape;
    BhkMoppBvTreeShape => assign_bhk_mopp_bv_tree_shape, add_sub_bhk_mopp_bv_tree_shape;
    BhkNiTriStripsShape => assign_bhk_ni_tri_strips_shape, add_sub_bhk_ni_tri_strips_shape;
    BhkListShape => assign_bhk_list_shape, add_sub_bhk_list_shape;
    BhkPackedNiTriStripsShape => assign_bhk_packed_ni_tri_strips_shape, add_sub_bhk_packed_ni_tri_strips_shape;
    BhkCompressedMeshShape => assign_bhk_compressed_mesh_shape, add_sub_bhk_compressed_mesh_shape;
}

/// Its entities are on [`View::entities`].
impl BhkConstraint {
    fields! {
        /// The solver priority, as a raw value.
        priority, set_priority: u32 = constraint_priority, set_constraint_priority;
    }
}

/// A constraint motor, which drives the constraint toward a target. Only saved from Fallout 3
/// on, and only by [limited hinge](LimitedHingeDescriptor), [ragdoll](RagdollDescriptor) and
/// [prismatic](PrismaticDescriptor) descriptors.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Motor {
    #[default]
    None,
    Position(PositionMotor),
    Velocity(VelocityMotor),
    SpringDamper(SpringDamperMotor),
}

/// A [`Motor`] driving toward a position.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PositionMotor {
    pub min_force: f32,
    pub max_force: f32,
    pub tau: f32,
    pub damping: f32,
    pub proportional_recovery_velocity: f32,
    pub constant_recovery_velocity: f32,
    pub enabled: bool,
}

impl Default for PositionMotor {
    fn default() -> Self {
        Self {
            min_force: -1_000_000.0,
            max_force: 1_000_000.0,
            tau: 0.8,
            damping: 1.0,
            proportional_recovery_velocity: 2.0,
            constant_recovery_velocity: 1.0,
            enabled: false,
        }
    }
}

/// A [`Motor`] driving toward a velocity.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VelocityMotor {
    pub min_force: f32,
    pub max_force: f32,
    pub tau: f32,
    pub velocity_target: f32,
    pub use_velocity_target_from_constraint_targets: bool,
    pub enabled: bool,
}

impl Default for VelocityMotor {
    fn default() -> Self {
        Self {
            min_force: -1_000_000.0,
            max_force: 1_000_000.0,
            tau: 0.0,
            velocity_target: 0.0,
            use_velocity_target_from_constraint_targets: false,
            enabled: false,
        }
    }
}

/// A [`Motor`] acting as a spring.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpringDamperMotor {
    pub min_force: f32,
    pub max_force: f32,
    pub spring_constant: f32,
    pub spring_damping: f32,
    pub enabled: bool,
}

impl Default for SpringDamperMotor {
    fn default() -> Self {
        Self { min_force: -1_000_000.0, max_force: 1_000_000.0, spring_constant: 0.0, spring_damping: 0.0, enabled: false }
    }
}

impl Motor {
    fn from_ffi(m: &ffi::MotorFfi) -> Self {
        match m.motor_type {
            1 => Self::Position(PositionMotor {
                min_force: m.min_force,
                max_force: m.max_force,
                tau: m.tau,
                damping: m.damping,
                proportional_recovery_velocity: m.proportional_recovery_velocity,
                constant_recovery_velocity: m.constant_recovery_velocity,
                enabled: m.enabled,
            }),
            2 => Self::Velocity(VelocityMotor {
                min_force: m.min_force,
                max_force: m.max_force,
                tau: m.tau,
                velocity_target: m.velocity_target,
                use_velocity_target_from_constraint_targets: m.use_velocity_target,
                enabled: m.enabled,
            }),
            3 => Self::SpringDamper(SpringDamperMotor {
                min_force: m.min_force,
                max_force: m.max_force,
                spring_constant: m.spring_constant,
                spring_damping: m.spring_damping,
                enabled: m.enabled,
            }),
            _ => Self::None,
        }
    }

    fn to_ffi(self) -> ffi::MotorFfi {
        let mut m = ffi::MotorFfi {
            motor_type: 0,
            min_force: 0.0,
            max_force: 0.0,
            tau: 0.0,
            damping: 0.0,
            proportional_recovery_velocity: 0.0,
            constant_recovery_velocity: 0.0,
            velocity_target: 0.0,
            use_velocity_target: false,
            spring_constant: 0.0,
            spring_damping: 0.0,
            enabled: false,
        };
        match self {
            Self::None => {}
            Self::Position(p) => {
                m.motor_type = 1;
                (m.min_force, m.max_force, m.tau, m.damping) = (p.min_force, p.max_force, p.tau, p.damping);
                m.proportional_recovery_velocity = p.proportional_recovery_velocity;
                m.constant_recovery_velocity = p.constant_recovery_velocity;
                m.enabled = p.enabled;
            }
            Self::Velocity(v) => {
                m.motor_type = 2;
                (m.min_force, m.max_force, m.tau, m.velocity_target) = (v.min_force, v.max_force, v.tau, v.velocity_target);
                m.use_velocity_target = v.use_velocity_target_from_constraint_targets;
                m.enabled = v.enabled;
            }
            Self::SpringDamper(s) => {
                m.motor_type = 3;
                (m.min_force, m.max_force) = (s.min_force, s.max_force);
                (m.spring_constant, m.spring_damping) = (s.spring_constant, s.spring_damping);
                m.enabled = s.enabled;
            }
        }
        m
    }
}

/// Builds a `DescriptorFfi` from vectors, floats and a motor.
fn descriptor_ffi(vectors: &[[f32; 4]], floats: &[f32], motor: Motor) -> ffi::DescriptorFfi {
    let mut d = ffi::DescriptorFfi { constraint_type: 0, vectors: [0.0; 32], floats: [0.0; 6], motor: motor.to_ffi() };
    d.vectors[..vectors.len() * 4].copy_from_slice(vectors.as_flattened());
    d.floats[..floats.len()].copy_from_slice(floats);
    d
}

fn vector(d: &ffi::DescriptorFfi, i: usize) -> [f32; 4] {
    d.vectors[i * 4..i * 4 + 4].try_into().unwrap()
}

/// A [`BhkBallAndSocketConstraint`]'s settings: a point on each entity, held together.
/// Vectors are `[x, y, z, w]` in each entity's space, in Havok units.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BallAndSocketDescriptor {
    pub translation_a: [f32; 4],
    pub translation_b: [f32; 4],
}

impl BallAndSocketDescriptor {
    fn from_ffi(d: &ffi::DescriptorFfi) -> Self {
        Self { translation_a: vector(d, 0), translation_b: vector(d, 1) }
    }

    fn to_ffi(self) -> ffi::DescriptorFfi {
        descriptor_ffi(&[self.translation_a, self.translation_b], &[], Motor::None)
    }
}

/// A [`BhkStiffSpringConstraint`]'s settings: a point on each entity, kept `length` apart.
/// Vectors are `[x, y, z, w]` in each entity's space, in Havok units.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StiffSpringDescriptor {
    pub pivot_a: [f32; 4],
    pub pivot_b: [f32; 4],
    pub length: f32,
}

impl StiffSpringDescriptor {
    fn from_ffi(d: &ffi::DescriptorFfi) -> Self {
        Self { pivot_a: vector(d, 0), pivot_b: vector(d, 1), length: d.floats[0] }
    }

    fn to_ffi(self) -> ffi::DescriptorFfi {
        descriptor_ffi(&[self.pivot_a, self.pivot_b], &[self.length], Motor::None)
    }
}

/// A [`BhkHingeConstraint`]'s settings. Vectors are `[x, y, z, w]` in each entity's space, in
/// Havok units.
///
/// Oblivion files save only `pivot_a`, `axle_in_a1`, `axle_in_a2`, `pivot_b` and `axle_b`.
/// Files between 20.0.0.5 and 20.2.0.7 save none.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HingeDescriptor {
    pub axle_a: [f32; 4],
    pub axle_in_a1: [f32; 4],
    pub axle_in_a2: [f32; 4],
    pub pivot_a: [f32; 4],
    pub axle_b: [f32; 4],
    pub axle_in_b1: [f32; 4],
    pub axle_in_b2: [f32; 4],
    pub pivot_b: [f32; 4],
}

impl HingeDescriptor {
    fn from_ffi(d: &ffi::DescriptorFfi) -> Self {
        Self {
            axle_a: vector(d, 0),
            axle_in_a1: vector(d, 1),
            axle_in_a2: vector(d, 2),
            pivot_a: vector(d, 3),
            axle_b: vector(d, 4),
            axle_in_b1: vector(d, 5),
            axle_in_b2: vector(d, 6),
            pivot_b: vector(d, 7),
        }
    }

    fn to_ffi(self) -> ffi::DescriptorFfi {
        let v = [
            self.axle_a,
            self.axle_in_a1,
            self.axle_in_a2,
            self.pivot_a,
            self.axle_b,
            self.axle_in_b1,
            self.axle_in_b2,
            self.pivot_b,
        ];
        descriptor_ffi(&v, &[], Motor::None)
    }
}

/// A [`BhkLimitedHingeConstraint`]'s settings: a [hinge](HingeDescriptor) with angle limits, in
/// radians. Vectors are `[x, y, z, w]` in each entity's space, in Havok units.
///
/// Oblivion files don't save `axle_in_b1` or the motor.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LimitedHingeDescriptor {
    pub axle_a: [f32; 4],
    pub axle_in_a1: [f32; 4],
    pub axle_in_a2: [f32; 4],
    pub pivot_a: [f32; 4],
    pub axle_b: [f32; 4],
    pub axle_in_b1: [f32; 4],
    pub axle_in_b2: [f32; 4],
    pub pivot_b: [f32; 4],
    pub min_angle: f32,
    pub max_angle: f32,
    pub max_friction: f32,
    pub motor: Motor,
}

impl LimitedHingeDescriptor {
    fn from_ffi(d: &ffi::DescriptorFfi) -> Self {
        Self {
            axle_a: vector(d, 0),
            axle_in_a1: vector(d, 1),
            axle_in_a2: vector(d, 2),
            pivot_a: vector(d, 3),
            axle_b: vector(d, 4),
            axle_in_b1: vector(d, 5),
            axle_in_b2: vector(d, 6),
            pivot_b: vector(d, 7),
            min_angle: d.floats[0],
            max_angle: d.floats[1],
            max_friction: d.floats[2],
            motor: Motor::from_ffi(&d.motor),
        }
    }

    fn to_ffi(self) -> ffi::DescriptorFfi {
        let v = [
            self.axle_a,
            self.axle_in_a1,
            self.axle_in_a2,
            self.pivot_a,
            self.axle_b,
            self.axle_in_b1,
            self.axle_in_b2,
            self.pivot_b,
        ];
        descriptor_ffi(&v, &[self.min_angle, self.max_angle, self.max_friction], self.motor)
    }
}

/// A [`BhkRagdollConstraint`]'s settings: a joint with a cone and twist limits, in radians.
/// Vectors are `[x, y, z, w]` in each entity's space, in Havok units.
///
/// Oblivion files don't save `motor_a`, `motor_b` or the motor.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RagdollDescriptor {
    pub twist_a: [f32; 4],
    pub plane_a: [f32; 4],
    pub motor_a: [f32; 4],
    pub pivot_a: [f32; 4],
    pub twist_b: [f32; 4],
    pub plane_b: [f32; 4],
    pub motor_b: [f32; 4],
    pub pivot_b: [f32; 4],
    pub cone_max_angle: f32,
    pub plane_min_angle: f32,
    pub plane_max_angle: f32,
    pub twist_min_angle: f32,
    pub twist_max_angle: f32,
    pub max_friction: f32,
    pub motor: Motor,
}

impl RagdollDescriptor {
    fn from_ffi(d: &ffi::DescriptorFfi) -> Self {
        Self {
            twist_a: vector(d, 0),
            plane_a: vector(d, 1),
            motor_a: vector(d, 2),
            pivot_a: vector(d, 3),
            twist_b: vector(d, 4),
            plane_b: vector(d, 5),
            motor_b: vector(d, 6),
            pivot_b: vector(d, 7),
            cone_max_angle: d.floats[0],
            plane_min_angle: d.floats[1],
            plane_max_angle: d.floats[2],
            twist_min_angle: d.floats[3],
            twist_max_angle: d.floats[4],
            max_friction: d.floats[5],
            motor: Motor::from_ffi(&d.motor),
        }
    }

    fn to_ffi(self) -> ffi::DescriptorFfi {
        let v = [
            self.twist_a,
            self.plane_a,
            self.motor_a,
            self.pivot_a,
            self.twist_b,
            self.plane_b,
            self.motor_b,
            self.pivot_b,
        ];
        let f = [
            self.cone_max_angle,
            self.plane_min_angle,
            self.plane_max_angle,
            self.twist_min_angle,
            self.twist_max_angle,
            self.max_friction,
        ];
        descriptor_ffi(&v, &f, self.motor)
    }
}

/// A [`BhkPrismaticConstraint`]'s settings: a sliding joint with distance limits. Vectors are
/// `[x, y, z, w]` in each entity's space, in Havok units.
///
/// Files between 20.0.0.5 and 20.2.0.7 save none of the vectors, and Oblivion files don't
/// save the motor.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PrismaticDescriptor {
    pub sliding_a: [f32; 4],
    pub rotation_a: [f32; 4],
    pub plane_a: [f32; 4],
    pub pivot_a: [f32; 4],
    pub sliding_b: [f32; 4],
    pub rotation_b: [f32; 4],
    pub plane_b: [f32; 4],
    pub pivot_b: [f32; 4],
    pub min_distance: f32,
    pub max_distance: f32,
    pub friction: f32,
    pub motor: Motor,
}

impl PrismaticDescriptor {
    fn from_ffi(d: &ffi::DescriptorFfi) -> Self {
        Self {
            sliding_a: vector(d, 0),
            rotation_a: vector(d, 1),
            plane_a: vector(d, 2),
            pivot_a: vector(d, 3),
            sliding_b: vector(d, 4),
            rotation_b: vector(d, 5),
            plane_b: vector(d, 6),
            pivot_b: vector(d, 7),
            min_distance: d.floats[0],
            max_distance: d.floats[1],
            friction: d.floats[2],
            motor: Motor::from_ffi(&d.motor),
        }
    }

    fn to_ffi(self) -> ffi::DescriptorFfi {
        let v = [
            self.sliding_a,
            self.rotation_a,
            self.plane_a,
            self.pivot_a,
            self.sliding_b,
            self.rotation_b,
            self.plane_b,
            self.pivot_b,
        ];
        descriptor_ffi(&v, &[self.min_distance, self.max_distance, self.friction], self.motor)
    }
}

/// The settings of a constraint wrapped by a [`BhkBreakableConstraint`] or
/// [`BhkMalleableConstraint`]. The variant is the wrapped constraint's type.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConstraintDescriptor {
    BallAndSocket(BallAndSocketDescriptor),
    Hinge(HingeDescriptor),
    LimitedHinge(LimitedHingeDescriptor),
    Prismatic(PrismaticDescriptor),
    Ragdoll(RagdollDescriptor),
    StiffSpring(StiffSpringDescriptor),
}

impl ConstraintDescriptor {
    fn from_ffi(d: &ffi::DescriptorFfi) -> Option<Self> {
        Some(match d.constraint_type {
            0 => Self::BallAndSocket(BallAndSocketDescriptor::from_ffi(d)),
            1 => Self::Hinge(HingeDescriptor::from_ffi(d)),
            2 => Self::LimitedHinge(LimitedHingeDescriptor::from_ffi(d)),
            6 => Self::Prismatic(PrismaticDescriptor::from_ffi(d)),
            7 => Self::Ragdoll(RagdollDescriptor::from_ffi(d)),
            8 => Self::StiffSpring(StiffSpringDescriptor::from_ffi(d)),
            _ => return None,
        })
    }

    fn to_ffi(self) -> ffi::DescriptorFfi {
        let (constraint_type, mut d) = match self {
            Self::BallAndSocket(c) => (0, c.to_ffi()),
            Self::Hinge(c) => (1, c.to_ffi()),
            Self::LimitedHinge(c) => (2, c.to_ffi()),
            Self::Prismatic(c) => (6, c.to_ffi()),
            Self::Ragdoll(c) => (7, c.to_ffi()),
            Self::StiffSpring(c) => (8, c.to_ffi()),
        };
        d.constraint_type = constraint_type;
        d
    }
}

/// A descriptor getter and setter for each constraint type.
macro_rules! descriptors {
    ($($ty:ty => $desc:ty, $get:ident, $set:ident;)*) => {$(
        impl $ty {
            /// The constraint's settings.
            pub fn descriptor(&self) -> $desc {
                <$desc>::from_ffi(&ffi::$get(&self.0))
            }

            /// Sets the [descriptor](Self::descriptor).
            pub fn set_descriptor(self: Pin<&mut Self>, descriptor: $desc) {
                ffi::$set(self.ffi_mut(), &descriptor.to_ffi());
            }
        }
    )*};
}

descriptors! {
    BhkHingeConstraint => HingeDescriptor, hinge_descriptor, set_hinge_descriptor;
    BhkLimitedHingeConstraint => LimitedHingeDescriptor, limited_hinge_descriptor, set_limited_hinge_descriptor;
    BhkRagdollConstraint => RagdollDescriptor, ragdoll_descriptor, set_ragdoll_descriptor;
    BhkStiffSpringConstraint => StiffSpringDescriptor, stiff_spring_descriptor, set_stiff_spring_descriptor;
    BhkPrismaticConstraint => PrismaticDescriptor, prismatic_descriptor, set_prismatic_descriptor;
    BhkBallAndSocketConstraint => BallAndSocketDescriptor, ball_and_socket_descriptor, set_ball_and_socket_descriptor;
}

/// The wrapped constraint's fields, for each wrapping type.
macro_rules! wrapped {
    ($($ty:ty => $get:ident, $set:ident, $ty_get:ident, $prio:ident, $set_prio:ident,
        $tau:ident, $set_tau:ident, $damping:ident, $set_damping:ident,
        $strength:ident, $set_strength:ident;)*) => {$(
        /// It wraps another constraint, whose entities are on [`View::wrapped_entities`].
        /// [`tau`](Self::tau) and [`damping`](Self::damping) are only saved up to Oblivion, and
        /// [`strength`](Self::strength) only from Fallout 3 on.
        impl $ty {
            /// The wrapped constraint's settings, or `None` if its type is unknown.
            pub fn wrapped(&self) -> Option<ConstraintDescriptor> {
                ConstraintDescriptor::from_ffi(&ffi::$get(&self.0))
            }

            /// The wrapped constraint's type as stored, for a [`wrapped`](Self::wrapped) of
            /// `None`.
            pub fn wrapped_type(&self) -> u32 {
                ffi::$ty_get(&self.0)
            }

            /// Sets the wrapped constraint's type and settings.
            pub fn set_wrapped(self: Pin<&mut Self>, descriptor: ConstraintDescriptor) {
                assert!(ffi::$set(self.ffi_mut(), &descriptor.to_ffi()), "a known type");
            }

            fields! {
                /// The wrapped constraint's solver priority, as a raw value.
                wrapped_priority, set_wrapped_priority: u32 = $prio, $set_prio;
                tau, set_tau: f32 = $tau, $set_tau;
                damping, set_damping: f32 = $damping, $set_damping;
                /// The strength; for a breakable constraint, the force that breaks it.
                strength, set_strength: f32 = $strength, $set_strength;
            }
        }
    )*};
}

wrapped! {
    BhkBreakableConstraint => breakable_wrapped, set_breakable_wrapped, breakable_wrapped_type,
        breakable_wrapped_priority, set_breakable_wrapped_priority, breakable_tau, set_breakable_tau,
        breakable_damping, set_breakable_damping, breakable_strength, set_breakable_strength;
    BhkMalleableConstraint => malleable_wrapped, set_malleable_wrapped, malleable_wrapped_type,
        malleable_wrapped_priority, set_malleable_wrapped_priority, malleable_tau, set_malleable_tau,
        malleable_damping, set_malleable_damping, malleable_strength, set_malleable_strength;
}

impl BhkBreakableConstraint {
    fields! {
        remove_when_broken, set_remove_when_broken: bool =
            breakable_remove_when_broken, set_breakable_remove_when_broken;
    }
}

/// Its entities are on [`View::entities`] and the bodies along it on
/// [`View::chained_entities`].
impl BhkBallSocketConstraintChain {
    /// The pivots, `[x, y, z, w]` in Havok units.
    pub fn pivots(&self) -> Vec<[f32; 4]> {
        ffi::chain_pivots(&self.0).as_chunks::<4>().0.to_vec()
    }

    pub fn set_pivots(self: Pin<&mut Self>, pivots: &[[f32; 4]]) {
        ffi::set_chain_pivots(self.ffi_mut(), pivots.as_flattened());
    }

    fields! {
        tau, set_tau: f32 = chain_tau, set_chain_tau;
        damping, set_damping: f32 = chain_damping, set_chain_damping;
        /// The constraint force mixing.
        cfm, set_cfm: f32 = chain_cfm, set_chain_cfm;
        max_error_distance, set_max_error_distance: f32 = chain_max_error_distance, set_chain_max_error_distance;
        /// The solver priority, as a raw value.
        priority, set_priority: u32 = chain_priority, set_chain_priority;
    }
}

/// A constraint with two [entities](View::entities): a [`BhkConstraint`] or one of its
/// subtypes, or a [`BhkBallSocketConstraintChain`].
///
/// This trait is sealed.
pub trait HasEntities: NiType {}

impl HasEntities for BhkConstraint {}
impl HasEntities for BhkHingeConstraint {}
impl HasEntities for BhkLimitedHingeConstraint {}
impl HasEntities for BhkBreakableConstraint {}
impl HasEntities for BhkRagdollConstraint {}
impl HasEntities for BhkStiffSpringConstraint {}
impl HasEntities for BhkPrismaticConstraint {}
impl HasEntities for BhkMalleableConstraint {}
impl HasEntities for BhkBallAndSocketConstraint {}
impl HasEntities for BhkBallSocketConstraintChain {}

/// A constraint that wraps another: a [`BhkBreakableConstraint`] or [`BhkMalleableConstraint`].
///
/// This trait is sealed.
pub trait WrapsConstraint: NiType {}

impl WrapsConstraint for BhkBreakableConstraint {}
impl WrapsConstraint for BhkMalleableConstraint {}

/// Constraint entities.
impl<'a, T: NiType> View<'a, T> {
    /// The two entities the constraint connects, `None` where a slot is empty.
    pub fn entities(&self) -> [Option<View<'a, BhkEntity>>; 2]
    where
        T: HasEntities,
    {
        [self.follow(RefKind::EntityA), self.follow(RefKind::EntityB)]
    }

    /// The two entities of the wrapped constraint, `None` where a slot is empty. Game files
    /// leave both empty.
    pub fn wrapped_entities(&self) -> [Option<View<'a, BhkEntity>>; 2]
    where
        T: WrapsConstraint,
    {
        [self.follow(RefKind::WrappedEntityA), self.follow(RefKind::WrappedEntityB)]
    }

    /// The rigid bodies along the chain. Skips empty refs.
    pub fn chained_entities<I>(&self) -> BlockIter<'a, BhkRigidBody, RefIndices>
    where
        T: IsA<BhkBallSocketConstraintChain, I>,
    {
        self.follow_list(RefList::ChainedEntities)
    }
}

impl<T: NiType> Edit<'_, T> {
    /// Whether each of `entities` is empty or a [`BhkEntity`] in this file.
    fn entities_ok(&self, entities: [Option<Index>; 2]) -> bool {
        let file = self.view().file();
        entities.iter().all(|e| e.is_none_or(|e| file.block_of::<BhkEntity>(e).is_some()))
    }

    /// Sets the two entities the constraint connects, `None` emptying a slot. Returns `false`,
    /// changing nothing, unless each is a [`BhkEntity`].
    pub fn set_entities(&mut self, entities: [Option<Index>; 2]) -> bool
    where
        T: HasEntities,
    {
        self.entities_ok(entities)
            && self.set_ref(RefKind::EntityA, entities[0])
            && self.set_ref(RefKind::EntityB, entities[1])
    }

    /// Sets the wrapped constraint's two [entities](View::wrapped_entities), like
    /// [`set_entities`](Self::set_entities).
    pub fn set_wrapped_entities(&mut self, entities: [Option<Index>; 2]) -> bool
    where
        T: WrapsConstraint,
    {
        self.entities_ok(entities)
            && self.set_ref(RefKind::WrappedEntityA, entities[0])
            && self.set_ref(RefKind::WrappedEntityB, entities[1])
    }

    /// Replaces the chain's [rigid bodies](View::chained_entities). Returns `false`, changing
    /// nothing, unless each is a [`BhkRigidBody`].
    pub fn set_chained_entities<I>(&mut self, bodies: &[Index]) -> bool
    where
        T: IsA<BhkBallSocketConstraintChain, I>,
    {
        self.set_ref_list(RefList::ChainedEntities, bodies)
    }

    /// Adds a new constraint of type `C` with default settings at the end of the rigid body's
    /// [constraints](View::constraints), with this body as its first [entity](View::entities),
    /// and returns its index.
    pub fn add_constraint<C: NewConstraint, I>(&mut self) -> Index
    where
        T: IsA<BhkRigidBody, I>,
    {
        let body = self.index() as u32;
        index_from_ffi(C::add(self.file_mut().inner.pin_mut(), body)).expect("the block is a rigid body")
    }
}

/// A constraint type that [`Edit::add_constraint`] can create.
///
/// This trait is sealed.
pub trait NewConstraint: NiType {
    #[doc(hidden)]
    fn add(file: Pin<&mut nif_ffi::NifFile>, body: u32) -> u32;
}

macro_rules! new_constraint {
    ($($ty:ty => $add:ident;)*) => {$(
        impl NewConstraint for $ty {
            fn add(file: Pin<&mut nif_ffi::NifFile>, body: u32) -> u32 {
                ffi::$add(file, body)
            }
        }
    )*};
}

new_constraint! {
    BhkHingeConstraint => add_bhk_hinge_constraint;
    BhkLimitedHingeConstraint => add_bhk_limited_hinge_constraint;
    BhkRagdollConstraint => add_bhk_ragdoll_constraint;
    BhkStiffSpringConstraint => add_bhk_stiff_spring_constraint;
    BhkPrismaticConstraint => add_bhk_prismatic_constraint;
    BhkBallAndSocketConstraint => add_bhk_ball_and_socket_constraint;
    BhkBreakableConstraint => add_bhk_breakable_constraint;
    BhkMalleableConstraint => add_bhk_malleable_constraint;
    BhkBallSocketConstraintChain => add_bhk_ball_socket_constraint_chain;
}

/// A bone's transform in a [`BhkPoseArray`] pose.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoneTransform {
    pub translation: Vec3,
    /// A quaternion, `[x, y, z, w]`.
    pub rotation: [f32; 4],
    pub scale: Vec3,
}

impl Default for BoneTransform {
    fn default() -> Self {
        Self { translation: Vec3::ZERO, rotation: [0.0, 0.0, 0.0, 1.0], scale: Vec3::new(1.0, 1.0, 1.0) }
    }
}

impl BhkPoseArray {
    /// The bone names, as stored.
    pub fn bones(&self) -> impl ExactSizeIterator<Item = &BStr> {
        (0..ffi::pose_array_bone_count(&self.0)).map(|i| ffi::pose_array_bone(&self.0, i).as_bstr())
    }

    pub fn set_bones<S: AsRef<[u8]>>(self: Pin<&mut Self>, bones: impl IntoIterator<Item = S>) {
        let (bytes, lengths) = joined(bones);
        ffi::set_pose_array_bones(self.ffi_mut(), &bytes, &lengths);
    }

    /// The poses, each usually with one transform per [bone](Self::bones).
    pub fn poses(&self) -> Vec<Vec<BoneTransform>> {
        (0..ffi::pose_array_pose_count(&self.0))
            .map(|i| {
                ffi::pose_array_pose(&self.0, i)
                    .into_iter()
                    .map(|t| BoneTransform {
                        translation: Vec3::from_ffi(t.translation),
                        rotation: t.rotation,
                        scale: Vec3::from_ffi(t.scale),
                    })
                    .collect()
            })
            .collect()
    }

    pub fn set_poses<P: AsRef<[BoneTransform]>>(self: Pin<&mut Self>, poses: &[P]) {
        let lengths: Vec<_> = poses.iter().map(|p| p.as_ref().len()).collect();
        let transforms: Vec<_> = poses
            .iter()
            .flat_map(|p| p.as_ref())
            .map(|t| ffi::BoneTransformFfi {
                translation: t.translation.to_ffi(),
                rotation: t.rotation,
                scale: t.scale.to_ffi(),
            })
            .collect();
        assert!(ffi::set_pose_array_poses(self.ffi_mut(), &transforms, &lengths), "lengths add up");
    }
}

/// A constraint of a [`BhkRagdollTemplateData`], applied to the bone's joint.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct JointConstraint {
    pub descriptor: ConstraintDescriptor,
    /// The solver priority, as a raw value.
    pub priority: u32,
    /// Only saved up to Oblivion.
    pub tau: f32,
    /// Only saved up to Oblivion.
    pub damping: f32,
    /// Only saved from Fallout 3 on.
    pub strength: f32,
}

/// One bone of a ragdoll: its name, physical settings and joint constraints.
impl BhkRagdollTemplateData {
    /// The bone's name, as stored.
    pub fn name(&self) -> &BStr {
        ffi::template_data_name(&self.0).as_bstr()
    }

    pub fn set_name(self: Pin<&mut Self>, name: impl AsRef<[u8]>) {
        ffi::set_template_data_name(self.ffi_mut(), name.as_ref());
    }

    fields! {
        mass, set_mass: f32 = template_data_mass, set_template_data_mass;
        /// Bounciness.
        restitution, set_restitution: f32 = template_data_restitution, set_template_data_restitution;
        friction, set_friction: f32 = template_data_friction, set_template_data_friction;
        radius, set_radius: f32 = template_data_radius, set_template_data_radius;
        /// The Havok material, as a raw value whose meaning depends on the game.
        material, set_material: u32 = template_data_material, set_template_data_material;
    }

    /// The joint's constraints. Constraints of an unknown type are left out.
    pub fn joint_constraints(&self) -> Vec<JointConstraint> {
        ffi::template_data_constraints(&self.0)
            .into_iter()
            .filter_map(|c| {
                Some(JointConstraint {
                    descriptor: ConstraintDescriptor::from_ffi(&c.descriptor)?,
                    priority: c.priority,
                    tau: c.tau,
                    damping: c.damping,
                    strength: c.strength,
                })
            })
            .collect()
    }

    /// Replaces the [joint constraints](Self::joint_constraints). Each keeps the entities of the
    /// one it replaces; new ones have none.
    pub fn set_joint_constraints(self: Pin<&mut Self>, constraints: &[JointConstraint]) {
        let constraints: Vec<_> = constraints
            .iter()
            .map(|c| ffi::JointConstraintFfi {
                descriptor: c.descriptor.to_ffi(),
                priority: c.priority,
                tau: c.tau,
                damping: c.damping,
                strength: c.strength,
                entity_a: u32::MAX,
                entity_b: u32::MAX,
            })
            .collect();
        assert!(ffi::set_template_data_constraints(self.ffi_mut(), &constraints), "known types");
    }
}

/// Ragdoll template bones.
impl<'a, T: NiType> View<'a, T> {
    /// The ragdoll template's bones. Skips empty refs.
    pub fn ragdoll_bones<I>(&self) -> BlockIter<'a, BhkRagdollTemplateData, RefIndices>
    where
        T: IsA<BhkRagdollTemplate, I>,
    {
        self.follow_list(RefList::RagdollBones)
    }
}

impl<T: NiType> Edit<'_, T> {
    /// Replaces the ragdoll template's [bones](View::ragdoll_bones). Returns `false`, changing
    /// nothing, unless each is a [`BhkRagdollTemplateData`].
    pub fn set_ragdoll_bones<I>(&mut self, bones: &[Index]) -> bool
    where
        T: IsA<BhkRagdollTemplate, I>,
    {
        self.set_ref_list(RefList::RagdollBones, bones)
    }
}
