//! Assigning extra data blocks to objects.

#[cxx::bridge]
pub mod ffi {
    extern "C++" {
        type Vec3 = crate::object_3d::ffi::Vec3;
        type FurniturePositionFfi = crate::extra_data_types::ffi::FurniturePositionFfi;
        type ConnectPointTransform = crate::extra_data_types::ffi::ConnectPointTransform;
        type InvMarkerFfi = crate::extra_data_types::ffi::InvMarkerFfi;
        type PackedGeomHeader = crate::extra_data_types::ffi::PackedGeomHeader;
        type PackedGeomObjectFfi = crate::extra_data_types::ffi::PackedGeomObjectFfi;
        type Float4 = crate::extra_data_types::ffi::Float4;
    }

    #[namespace = "nifly"]
    unsafe extern "C++" {
        include!("NifFile.hpp");

        type NifFile = crate::nif_file::ffi::NifFile;
    }

    #[namespace = "nifly_rs"]
    unsafe extern "C++" {
        include!("nif_file_shim.h");

        /// `NifFile::AssignExtraData` with a new extra data block of each type. `target` is the
        /// block index of a `NiObjectNET`. Return the new block's index, or `u32::MAX`
        /// (`NIF_NPOS`), adding nothing, if `target` isn't a `NiObjectNET`.
        fn assign_string_extra_data(
            file: Pin<&mut NifFile>,
            target: u32,
            name: &[u8],
            value: &[u8],
        ) -> u32;
        fn assign_integer_extra_data(
            file: Pin<&mut NifFile>,
            target: u32,
            name: &[u8],
            value: u32,
        ) -> u32;
        fn assign_integers_extra_data(
            file: Pin<&mut NifFile>,
            target: u32,
            name: &[u8],
            values: &[u32],
        ) -> u32;
        fn assign_float_extra_data(
            file: Pin<&mut NifFile>,
            target: u32,
            name: &[u8],
            value: f32,
        ) -> u32;
        fn assign_boolean_extra_data(
            file: Pin<&mut NifFile>,
            target: u32,
            name: &[u8],
            value: bool,
        ) -> u32;
        /// A `BSXFlags` block, always named "BSX".
        fn assign_bsx_flags(file: Pin<&mut NifFile>, target: u32, flags: u32) -> u32;
        fn assign_binary_extra_data(
            file: Pin<&mut NifFile>,
            target: u32,
            name: &[u8],
            bytes: &[u8],
        ) -> u32;
        fn assign_floats_extra_data(
            file: Pin<&mut NifFile>,
            target: u32,
            name: &[u8],
            values: &[f32],
        ) -> u32;
        /// The strings are `bytes` split into `lengths`.
        fn assign_strings_extra_data(
            file: Pin<&mut NifFile>,
            target: u32,
            name: &[u8],
            bytes: &[u8],
            lengths: &[usize],
        ) -> u32;
        /// A `BSPackedCombinedSharedGeomDataExtra` with the header and objects, as in
        /// `set_packed_shared_header` and `set_packed_shared_objects`.
        fn assign_packed_shared_geom(
            file: Pin<&mut NifFile>,
            target: u32,
            name: &[u8],
            header: PackedGeomHeader,
            objects: &[PackedGeomObjectFfi],
            instances: &[f32],
        ) -> u32;
        /// Sets the next block in the extra data chain of the `NiExtraData` at `data`
        /// (`NIF_NPOS` clears it). False, changing nothing, if a block has the wrong type or the
        /// chain would loop back to `data`.
        fn set_extra_data_next(file: Pin<&mut NifFile>, data: u32, next: u32) -> bool;
        /// A `SkinAttach`; the bone names are `bytes` split into `lengths`.
        fn assign_skin_attach(
            file: Pin<&mut NifFile>,
            target: u32,
            name: &[u8],
            bytes: &[u8],
            lengths: &[usize],
        ) -> u32;
        /// A `BSFurnitureMarkerNode` if `node`, otherwise a `BSFurnitureMarker`.
        fn assign_furniture_marker(
            file: Pin<&mut NifFile>,
            target: u32,
            name: &[u8],
            positions: &[FurniturePositionFfi],
            node: bool,
        ) -> u32;
        /// Connect point i: parent and name are strings 2i and 2i + 1 of `bytes` split into
        /// `lengths`, as in `assign_strings_extra_data`, and its transform is `transforms[i]`.
        fn assign_connect_point_parents(
            file: Pin<&mut NifFile>,
            target: u32,
            name: &[u8],
            bytes: &[u8],
            lengths: &[usize],
            transforms: &[ConnectPointTransform],
        ) -> u32;
        fn assign_connect_point_children(
            file: Pin<&mut NifFile>,
            target: u32,
            name: &[u8],
            skinned: bool,
            bytes: &[u8],
            lengths: &[usize],
        ) -> u32;
        fn assign_inv_marker(
            file: Pin<&mut NifFile>,
            target: u32,
            name: &[u8],
            marker: InvMarkerFfi,
        ) -> u32;
        fn assign_bs_bound(
            file: Pin<&mut NifFile>,
            target: u32,
            name: &[u8],
            center: Vec3,
            half_extents: Vec3,
        ) -> u32;
        fn assign_color_extra_data(
            file: Pin<&mut NifFile>,
            target: u32,
            name: &[u8],
            color: Float4,
        ) -> u32;
        fn assign_vector_extra_data(
            file: Pin<&mut NifFile>,
            target: u32,
            name: &[u8],
            vector: Float4,
        ) -> u32;
        /// `u32::MAX` also for more than 65535 weights.
        fn assign_vert_weights_extra_data(
            file: Pin<&mut NifFile>,
            target: u32,
            name: &[u8],
            weights: &[f32],
        ) -> u32;
        fn assign_eye_center_extra_data(
            file: Pin<&mut NifFile>,
            target: u32,
            name: &[u8],
            values: &[f32],
        ) -> u32;
        fn assign_position_data(
            file: Pin<&mut NifFile>,
            target: u32,
            name: &[u8],
            values: &[f32],
        ) -> u32;
        fn assign_w_array(file: Pin<&mut NifFile>, target: u32, name: &[u8], values: &[u32])
        -> u32;
        /// Block i takes the next `block_lengths[i]` points and normals. `u32::MAX` also for
        /// more than 65535 blocks or points in a block, or lengths that don't add up.
        fn assign_decal_placement_vector_extra_data(
            file: Pin<&mut NifFile>,
            target: u32,
            name: &[u8],
            value: f32,
            block_lengths: &[usize],
            points: &[Vec3],
            normals: &[Vec3],
        ) -> u32;
        fn assign_distant_object_extra_data(
            file: Pin<&mut NifFile>,
            target: u32,
            name: &[u8],
            flags: u32,
        ) -> u32;
        fn assign_distant_object_large_ref_extra_data(
            file: Pin<&mut NifFile>,
            target: u32,
            name: &[u8],
            large_ref: bool,
        ) -> u32;
        /// Key i is `bytes` split into `lengths`, at `times[i]`.
        fn assign_text_key_extra_data(
            file: Pin<&mut NifFile>,
            target: u32,
            name: &[u8],
            bytes: &[u8],
            lengths: &[usize],
            times: &[f32],
        ) -> u32;
        /// Bone i is `bytes` split into `lengths`, at `distances[i]`.
        fn assign_bone_lod_extra_data(
            file: Pin<&mut NifFile>,
            target: u32,
            name: &[u8],
            bytes: &[u8],
            lengths: &[usize],
            distances: &[u32],
        ) -> u32;
        /// Bone i is `bytes` split into `lengths`, with `translations[i]`.
        fn assign_bone_translations(
            file: Pin<&mut NifFile>,
            target: u32,
            name: &[u8],
            bytes: &[u8],
            lengths: &[usize],
            translations: &[Vec3],
        ) -> u32;
        fn assign_behavior_graph_extra_data(
            file: Pin<&mut NifFile>,
            target: u32,
            name: &[u8],
            graph_file: &[u8],
            controls_base_skeleton: bool,
        ) -> u32;
        /// Attach a `BSExtraData` block (no name). `u32::MAX` also if the file version is
        /// 4.2.2.0 or older, where extra data is a linked list these blocks can't join.
        fn assign_bs_cloth_extra_data(file: Pin<&mut NifFile>, target: u32, bytes: &[u8]) -> u32;
        fn assign_bs_collision_query_proxy_extra_data(
            file: Pin<&mut NifFile>,
            target: u32,
            bytes: &[u8],
        ) -> u32;
    }
}
