//! Base classes of the nifly object hierarchy:
//! `NiObject` -> `NiObjectNET` -> `NiAVObject`.
//!
//! cxx has no notion of inheritance, so each class is an unrelated opaque
//! type here. Upcasts are explicit shim functions.

#[cxx::bridge]
pub mod ffi {
    #[namespace = "nifly"]
    unsafe extern "C++" {
        include!("Objects.hpp");

        type NiObject;
        type NiObjectNET;
        type NiAVObject;
    }

    extern "C++" {
        include!("nifly-sys/src/object_3d.rs.h");

        type Transform = crate::object_3d::ffi::Transform;
        type Vec3 = crate::object_3d::ffi::Vec3;
        type BoundingVolumeFfi = crate::object_3d::ffi::BoundingVolumeFfi;
    }

    #[namespace = "nifly_rs"]
    unsafe extern "C++" {
        include!("objects_shim.h");

        fn av_as_net(obj: &NiAVObject) -> &NiObjectNET;
        fn net_as_obj(obj: &NiObjectNET) -> &NiObject;
        fn downcast_ni_object_net(obj: &NiObject) -> *const NiObjectNET;
        fn downcast_ni_av_object(obj: &NiObject) -> *const NiAVObject;

        fn block_name(obj: &NiObject) -> &str;
        fn name_bytes(obj: &NiObjectNET) -> &[u8];
        fn set_object_net_name(obj: Pin<&mut NiObjectNET>, name: &[u8]);
        /// Whether `obj` has a name: it is an `NiObjectNET` or an `NiExtraData`.
        fn is_named(obj: &NiObject) -> bool;
        /// The name of an `NiObjectNET` or `NiExtraData`, or empty for other blocks.
        fn object_name(obj: &NiObject) -> &[u8];
        fn transform(obj: &NiAVObject) -> Transform;
        fn flags(obj: &NiAVObject) -> u32;
        fn set_flags(obj: Pin<&mut NiAVObject>, flags: u32);
        fn set_transform(obj: Pin<&mut NiAVObject>, transform: Transform);
        fn velocity(obj: &NiAVObject) -> Vec3;
        fn set_velocity(obj: Pin<&mut NiAVObject>, velocity: Vec3);
        /// The bounding volume as records (see `BoundingVolumeFfi`); empty if the object has none.
        fn bounding_volume(obj: &NiAVObject) -> Vec<BoundingVolumeFfi>;
        /// Sets the bounding volume from records, or removes it if `records` is empty. Returns
        /// false, changing nothing, if the records don't form one volume.
        fn set_bounding_volume(obj: Pin<&mut NiAVObject>, records: &[BoundingVolumeFfi]) -> bool;
        /// `NiObject::groupID`, which is protected.
        fn group_id(obj: &NiObject) -> u32;
        fn set_group_id(obj: Pin<&mut NiObject>, id: u32);

        /// Block indices `obj` references. May contain `u32::MAX` (`NIF_NPOS`) for empty refs.
        fn child_indices(obj: &NiObject) -> Vec<u32>;
    }
}
