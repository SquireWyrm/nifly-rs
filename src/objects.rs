//! Base classes of the object hierarchy.
//!
//! Each block type derefs to its base class:
//!
//! ```text
//! NiNode -> NiAvObject -> NiObjectNet -> NiObject
//! ```
//!
//! This means base-class methods are callable on any subclass, e.g.
//! `node.name()` or `node.block_name()`, and through a `&mut` block their
//! setters too, e.g. `node.set_name("Root")`. Downcasting goes through
//! [`NiObject::downcast`].
//!
//! All of these types are only ever handed out as references borrowed from a
//! [`NifFile`](crate::file::NifFile), so they can't outlive the file or
//! observe it being mutated. Blocks can't be moved out of the file: swapping
//! two blocks with [`std::mem::swap`] does nothing.

use std::{
    ops::{Deref, DerefMut},
    pin::Pin,
};

use bstr::{BStr, ByteSlice};

use nifly_sys::objects::ffi;
use ref_cast::{RefCastCustom, ref_cast_custom};

use crate::object3d::{BoundingVolume, Transform, Vec3};

/// Root of the object hierarchy. Every block in a NIF file is a `NiObject`.
#[derive(RefCastCustom)]
#[repr(transparent)]
pub struct NiObject(ffi::NiObject);

impl NiObject {
    #[ref_cast_custom]
    pub(crate) fn from_ffi(inner: &ffi::NiObject) -> &Self;

    /// The block type name, e.g. `"NiNode"` or `"BSFadeNode"`.
    pub fn block_name(&self) -> &str {
        ffi::block_name(&self.0)
    }

    /// Returns `self` as `T` if the block is a `T` or a subclass of it.
    pub fn downcast<T: NiType>(&self) -> Option<&T> {
        T::downcast_from(self)
    }

    /// Like [`downcast`](Self::downcast), for a mutable block.
    ///
    /// ```
    /// # use nifly::{file::NifFile, shaders::NiMaterialProperty};
    /// # let mut nif = NifFile::load(std::fs::File::open(
    /// #     "nifly-sys/vendor/nifly/tests/input/TestNifFile_Static_MW.nif").unwrap(), None).unwrap();
    /// # let index = nif.blocks_of::<NiMaterialProperty>().indices().next().unwrap();
    /// let block = nif.block_mut(index).unwrap();
    /// if let Some(material) = block.downcast_mut::<NiMaterialProperty>() {
    ///     material.set_alpha(0.5);
    /// }
    /// ```
    pub fn downcast_mut<T: NiType>(&mut self) -> Option<&mut T> {
        reborrow_mut(self, |obj| obj.downcast::<T>())
    }

    pub(crate) fn as_ffi(&self) -> &ffi::NiObject {
        &self.0
    }

    /// The block's group ID, saved in files from version 10.0.0.0 to 10.1.0.113.
    pub fn group_id(&self) -> u32 {
        ffi::group_id(&self.0)
    }

    pub fn set_group_id(&mut self, id: u32) {
        ffi::set_group_id(self.ffi_mut(), id);
    }

    fn ffi_mut(&mut self) -> Pin<&mut ffi::NiObject> {
        // SAFETY: the C++ object never moves (see `assert_zero_sized`).
        unsafe { Pin::new_unchecked(&mut self.0) }
    }

    /// The block's name if it has one (it is an [`NiObjectNet`] or an
    /// [`NiExtraData`](crate::extra_data::NiExtraData)).
    pub(crate) fn name_if_named(&self) -> Option<&[u8]> {
        ffi::is_named(&self.0).then(|| ffi::object_name(&self.0))
    }
}

/// An object with a name, extra data and controllers.
#[derive(RefCastCustom)]
#[repr(transparent)]
pub struct NiObjectNet(ffi::NiObjectNET);

impl NiObjectNet {
    #[ref_cast_custom]
    pub(crate) fn from_ffi(inner: &ffi::NiObjectNET) -> &Self;

    pub(crate) fn as_ffi(&self) -> &ffi::NiObjectNET {
        &self.0
    }

    /// The object's name, as stored. Bethesda files commonly use Windows-1252.
    pub fn name(&self) -> &BStr {
        ffi::name_bytes(&self.0).as_bstr()
    }

    /// Renames the object. Names are stored as given; saving adds them to the string table.
    pub fn set_name(&mut self, name: impl AsRef<[u8]>) {
        // SAFETY: the C++ object never moves (see `assert_zero_sized`).
        let inner = unsafe { Pin::new_unchecked(&mut self.0) };
        ffi::set_object_net_name(inner, name.as_ref());
    }
}

impl Deref for NiObjectNet {
    type Target = NiObject;

    fn deref(&self) -> &NiObject {
        NiObject::from_ffi(ffi::net_as_obj(&self.0))
    }
}

/// Fails the build unless `T` is zero-sized. Every block wrapper is, because the bridge's C++
/// types are opaque and zero-sized on the Rust side. So safe code can't move a C++ object
/// through a `&mut` to it (`mem::swap` and `mem::replace` copy nothing), and handing out
/// `&mut` blocks is sound although C++ objects must not move.
pub(crate) const fn assert_zero_sized<T>() {
    assert!(size_of::<T>() == 0, "block wrappers must be zero-sized");
}

const _: () = assert_zero_sized::<NiObject>();
const _: () = assert_zero_sized::<NiObjectNet>();
const _: () = assert_zero_sized::<NiAvObject>();

/// Reborrows the block as the `U` that `f` finds for it (the same C++ object seen as a base
/// or derived class), or `None` if `f` finds none. Backs each type's `DerefMut` and
/// [`NiObject::downcast_mut`].
///
/// `f` only locates the object: its address is applied to the `&mut`'s own pointer, so the
/// result keeps that pointer's provenance and no `&mut` is made from a shared reference.
pub(crate) fn reborrow_mut<T, U>(this: &mut T, f: impl FnOnce(&T) -> Option<&U>) -> Option<&mut U> {
    let this: *mut T = this;
    // SAFETY: `this` is valid for the duration of the call, and no `&mut` to it is live.
    let found: *const U = f(unsafe { &*this })?;
    // Where the shim puts the other class (the same address under single inheritance).
    let offset = (found as *const u8 as isize).wrapping_sub(this as *const u8 as isize);
    let found = this.cast::<u8>().wrapping_offset(offset).cast::<U>();
    // SAFETY: `found` is the same C++ object, so the exclusive borrow of `this` carries over to
    // it. The object is only accessed through C++ and never moves.
    Some(unsafe { &mut *found })
}

/// [`DerefMut`] for a block: the same object as its direct base type.
pub(crate) fn upcast_mut<T: Deref<Target: Sized>>(this: &mut T) -> &mut T::Target {
    reborrow_mut(this, |t| Some(&**t)).expect("an upcast always succeeds")
}

impl DerefMut for NiObjectNet {
    fn deref_mut(&mut self) -> &mut NiObject {
        upcast_mut(self)
    }
}

impl DerefMut for NiAvObject {
    fn deref_mut(&mut self) -> &mut NiObjectNet {
        upcast_mut(self)
    }
}

/// An object that lives in the scene graph, with a transform relative to its parent.
#[derive(RefCastCustom)]
#[repr(transparent)]
pub struct NiAvObject(ffi::NiAVObject);

impl NiAvObject {
    #[ref_cast_custom]
    pub(crate) fn from_ffi(inner: &ffi::NiAVObject) -> &Self;

    /// Transform from this object's coordinate system to its parent's.
    pub fn transform(&self) -> Transform {
        Transform::from_ffi(ffi::transform(&self.0))
    }

    /// Sets the transform to the parent. See also
    /// [`NifFile::set_node_transform_to_parent`](crate::file::NifFile::set_node_transform_to_parent),
    /// which finds a node by name.
    pub fn set_transform(&mut self, transform: Transform) {
        ffi::set_transform(self.ffi_mut(), transform.to_ffi());
    }

    pub fn raw_flags(&self) -> u32 {
        ffi::flags(&self.0)
    }

    /// Sets the flags. Files before Fallout 3 save only the low 16 bits.
    pub fn set_raw_flags(&mut self, flags: u32) {
        ffi::set_flags(self.ffi_mut(), flags);
    }

    /// The object's velocity, saved in files up to version 4.2.2.0.
    pub fn velocity(&self) -> Vec3 {
        Vec3::from_ffi(ffi::velocity(&self.0))
    }

    pub fn set_velocity(&mut self, velocity: Vec3) {
        ffi::set_velocity(self.ffi_mut(), velocity.to_ffi());
    }

    /// The object's bounding volume, or `None` if it has none. Saved in files up to version
    /// 4.2.2.0.
    pub fn bounding_volume(&self) -> Option<BoundingVolume> {
        BoundingVolume::from_ffi(&mut ffi::bounding_volume(&self.0).into_iter())
    }

    /// Sets the bounding volume, or removes it with `None`.
    pub fn set_bounding_volume(&mut self, volume: Option<&BoundingVolume>) {
        let mut records = Vec::new();
        if let Some(volume) = volume {
            volume.to_ffi(&mut records);
        }
        assert!(ffi::set_bounding_volume(self.ffi_mut(), &records), "records of one volume");
    }

    fn ffi_mut(&mut self) -> Pin<&mut ffi::NiAVObject> {
        // SAFETY: the C++ object never moves (see `assert_zero_sized`).
        unsafe { Pin::new_unchecked(&mut self.0) }
    }
}

impl Deref for NiAvObject {
    type Target = NiObjectNet;

    fn deref(&self) -> &NiObjectNet {
        NiObjectNet::from_ffi(ffi::av_as_net(&self.0))
    }
}

mod sealed {
    pub trait Sealed {}
}

/// A concrete type that a [`NiObject`] can be downcast to.
///
/// This trait is sealed. Implementations are provided by this crate.
pub trait NiType: sealed::Sealed + 'static {
    #[doc(hidden)]
    fn downcast_from(obj: &NiObject) -> Option<&Self>;

    /// The same object as an [`NiObject`], through each base type's `Deref`.
    #[doc(hidden)]
    fn as_object(&self) -> &NiObject;
}

pub(crate) use sealed::Sealed;

/// A block type's direct base type, which its `Deref` goes to.
///
/// This trait is sealed.
#[doc(hidden)]
pub trait HasParent: NiType + std::ops::Deref<Target = Self::Parent> {
    type Parent: NiType;
}

/// Marks a type as `B` or a subclass of `B`, so a [`View`](crate::handle::View) or
/// [`Edit`](crate::handle::Edit) of it has `B`'s methods. `I` records how far up `B` is; it's
/// always inferred.
///
/// This trait is sealed.
pub trait IsA<B: NiType, I>: NiType {
    /// The same block as a `B`.
    #[doc(hidden)]
    fn upcast_ref(&self) -> &B;
}

/// `IsA` index: the type itself.
#[doc(hidden)]
pub struct Here;

/// `IsA` index: `I` levels above the direct base type.
#[doc(hidden)]
pub struct There<I>(std::marker::PhantomData<I>);

impl<T: NiType> IsA<T, Here> for T {
    fn upcast_ref(&self) -> &T {
        self
    }
}

impl<T: HasParent, B: NiType, I> IsA<B, There<I>> for T
where
    T::Parent: IsA<B, I>,
{
    fn upcast_ref(&self) -> &B {
        let parent: &T::Parent = self;
        parent.upcast_ref()
    }
}

impl HasParent for NiObjectNet {
    type Parent = NiObject;
}

impl HasParent for NiAvObject {
    type Parent = NiObjectNet;
}

impl Sealed for NiObject {}

/// Every block is an `NiObject`, so this downcast always succeeds.
impl NiType for NiObject {
    fn downcast_from(obj: &NiObject) -> Option<&Self> {
        Some(obj)
    }

    fn as_object(&self) -> &NiObject {
        self
    }
}

impl Sealed for NiObjectNet {}

impl NiType for NiObjectNet {
    fn downcast_from(obj: &NiObject) -> Option<&Self> {
        // SAFETY: the shim returns either null or a pointer to the same object as `obj`.
        unsafe { ffi::downcast_ni_object_net(obj.as_ffi()).as_ref() }.map(Self::from_ffi)
    }

    fn as_object(&self) -> &NiObject {
        self
    }
}

impl Sealed for NiAvObject {}

impl NiType for NiAvObject {
    fn downcast_from(obj: &NiObject) -> Option<&Self> {
        // SAFETY: the shim returns either null or a pointer to the same object as `obj`.
        unsafe { ffi::downcast_ni_av_object(obj.as_ffi()).as_ref() }.map(Self::from_ffi)
    }

    fn as_object(&self) -> &NiObject {
        let net: &NiObjectNet = self;
        net
    }
}
