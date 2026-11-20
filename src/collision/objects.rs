//! Collision objects, collision data and physics systems.

use super::*;

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
