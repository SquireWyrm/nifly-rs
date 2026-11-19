/// Defines safe wrappers for nifly classes that sit below `NiAVObject`.
///
/// For each `Name(FfiType): Parent => upcast_fn, downcast_fn;` this generates:
/// - `#[repr(transparent)] pub struct Name(ffi::FfiType)` with a crate-private `from_ffi`
/// - `Deref<Target = Parent>` through `ffi::upcast_fn`
/// - `upcast_mut`, the `Deref` for `Pin<&mut Name>`
/// - `NiType`, so `NiObject::downcast::<Name>()` works, through `ffi::downcast_fn`
///
/// `ffi` is the bridge module the types and shims live in.
macro_rules! ni_types {
    ($ffi_mod:ident; $(
        $(#[$meta:meta])*
        $name:ident($ffi:ident): $parent:ty => $upcast:ident, $downcast:ident;
    )*) => {$(
        $(#[$meta])*
        #[derive(::ref_cast::RefCastCustom)]
        #[repr(transparent)]
        pub struct $name($ffi_mod::$ffi);

        impl $name {
            #[::ref_cast::ref_cast_custom]
            #[allow(dead_code)]
            pub(crate) fn from_ffi(inner: &$ffi_mod::$ffi) -> &Self;

            /// The pinned C++ object, for calling mutating shims.
            #[allow(dead_code)]
            pub(crate) fn ffi_mut(self: ::std::pin::Pin<&mut Self>) -> ::std::pin::Pin<&mut $ffi_mod::$ffi> {
                // SAFETY: structural pin projection onto the only field of a
                // `repr(transparent)` struct.
                unsafe { self.map_unchecked_mut(|s| &mut s.0) }
            }

            #[doc = concat!(
                "This block as its direct base type, [`", stringify!($parent), "`], for that type's ",
                "setters. Chain to reach further bases."
            )]
            pub fn upcast_mut(self: ::std::pin::Pin<&mut Self>) -> ::std::pin::Pin<&mut $parent> {
                $crate::objects::upcast_mut(self)
            }
        }

        impl ::std::ops::Deref for $name {
            type Target = $parent;

            fn deref(&self) -> &$parent {
                <$parent>::from_ffi($ffi_mod::$upcast(&self.0))
            }
        }

        impl $crate::objects::Sealed for $name {}

        impl $crate::objects::HasParent for $name {
            type Parent = $parent;
        }

        impl $crate::objects::NiType for $name {
            fn downcast_from(obj: &$crate::objects::NiObject) -> Option<&Self> {
                // SAFETY: the shim returns either null or a pointer to the same
                // object as `obj`, so it has the same lifetime.
                unsafe { $ffi_mod::$downcast(obj.as_ffi()).as_ref() }.map(Self::from_ffi)
            }

            fn as_object(&self) -> &$crate::objects::NiObject {
                <$parent as $crate::objects::NiType>::as_object(::std::ops::Deref::deref(self))
            }
        }
    )*};
}
