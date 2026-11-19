//! Plain structs shared by the `extra_data` and `nif_file` bridges. They live in a bridge of their
//! own, without a shim, so either shim can include their header without an include cycle.

#[cxx::bridge]
pub mod ffi {
    /// A `FurniturePosition`. Which fields a file stores depends on its version.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct FurniturePositionFfi {
        pub offset: Vec3,
        pub orientation: u16,
        pub position_ref_1: u8,
        pub position_ref_2: u8,
        pub heading: f32,
        pub animation_type: u16,
        pub entry_points: u16,
    }

    /// A `BSConnectPoint`'s transform; its two strings are passed separately.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct ConnectPointTransform {
        pub rotation_w: f32,
        pub rotation_x: f32,
        pub rotation_y: f32,
        pub rotation_z: f32,
        pub translation: Vec3,
        pub scale: f32,
    }

    /// A `BSInvMarker`'s values.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct InvMarkerFfi {
        pub rotation_x: u16,
        pub rotation_y: u16,
        pub rotation_z: u16,
        pub zoom: f32,
    }

    /// A `Color4` or `Vector4`.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Float4 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub w: f32,
    }

    /// A `BSPackedCombinedSharedGeomDataExtra`'s fields besides its objects.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PackedGeomHeader {
        pub vertex_desc: u64,
        pub vertex_count: u32,
        pub triangle_count: u32,
        pub flags_1: u32,
        pub flags_2: u32,
    }

    /// A `BSPackedGeomObject` with its `BSPackedGeomData`. `lods` is the triangle count and
    /// offset of LODs 0 to 2; the object's `instance_count` instances are passed separately.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PackedGeomObjectFfi {
        pub filename_hash: u32,
        pub data_offset: u32,
        pub vertex_count: u32,
        pub lod_levels: u32,
        pub lods: [u32; 6],
        pub vertex_desc: u64,
        pub instance_count: usize,
    }

    extern "C++" {
        // Defines Vec3 before the structs above use it.
        include!("nifly-sys/src/object_3d.rs.h");
        type Vec3 = crate::object_3d::ffi::Vec3;
    }

    // Lets the extra_data bridge return `Vec<FurniturePositionFfi>`.
    impl Vec<FurniturePositionFfi> {}
    impl Vec<PackedGeomObjectFfi> {}
}
