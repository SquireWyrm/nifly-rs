//! `NiExtraData` and the subclasses the safe crate supports. See `nodes.rs` for the
//! upcast/downcast convention.

#[cxx::bridge]
pub mod ffi {
    #[namespace = "nifly"]
    unsafe extern "C++" {
        include!("ExtraData.hpp");

        type NiExtraData;
        type NiStringExtraData;
        type NiIntegerExtraData;
        type BSXFlags;
        type NiFloatExtraData;
        type NiBooleanExtraData;
        type NiIntegersExtraData;
        type NiBinaryExtraData;
        type NiFloatsExtraData;
        type NiStringsExtraData;
        type SkinAttach;
        type BSPackedCombinedSharedGeomDataExtra;
        type BSFurnitureMarker;
        type BSFurnitureMarkerNode;
        type BSConnectPointParents;
        type BSConnectPointChildren;
        type BSInvMarker;
        type BSBound;
        type BSBehaviorGraphExtraData;
        type NiColorExtraData;
        type NiVectorExtraData;
        type NiVertWeightsExtraData;
        type BSEyeCenterExtraData;
        type BSPositionData;
        type BSWArray;
        type BSDecalPlacementVectorExtraData;
        type BSBoneLODExtraData;
        type BSDistantObjectExtraData;
        type BSDistantObjectLargeRefExtraData;
        type BoneTranslations;
        type NiTextKeyExtraData;

        // Derive from BSExtraData : NiObject, not NiExtraData.
        type BSExtraData;
        type BSClothExtraData;
        type BSCollisionQueryProxyExtraData;
    }

    #[namespace = "nifly"]
    extern "C++" {
        include!("Objects.hpp");

        type NiObject = crate::objects::ffi::NiObject;
    }

    extern "C++" {
        type Vec3 = crate::object_3d::ffi::Vec3;
        type FurniturePositionFfi = crate::extra_data_types::ffi::FurniturePositionFfi;
        type ConnectPointTransform = crate::extra_data_types::ffi::ConnectPointTransform;
        type InvMarkerFfi = crate::extra_data_types::ffi::InvMarkerFfi;
        type Float4 = crate::extra_data_types::ffi::Float4;
        type PackedGeomHeader = crate::extra_data_types::ffi::PackedGeomHeader;
        type PackedGeomObjectFfi = crate::extra_data_types::ffi::PackedGeomObjectFfi;
    }

    #[namespace = "nifly_rs"]
    unsafe extern "C++" {
        include!("extra_data_shim.h");

        fn upcast_ni_extra_data(obj: &NiExtraData) -> &NiObject;
        fn upcast_ni_string_extra_data(obj: &NiStringExtraData) -> &NiExtraData;
        fn upcast_ni_integer_extra_data(obj: &NiIntegerExtraData) -> &NiExtraData;
        fn upcast_bsx_flags(obj: &BSXFlags) -> &NiIntegerExtraData;
        fn upcast_ni_float_extra_data(obj: &NiFloatExtraData) -> &NiExtraData;
        fn upcast_ni_boolean_extra_data(obj: &NiBooleanExtraData) -> &NiExtraData;
        fn upcast_ni_integers_extra_data(obj: &NiIntegersExtraData) -> &NiExtraData;

        fn downcast_ni_extra_data(obj: &NiObject) -> *const NiExtraData;
        fn downcast_ni_string_extra_data(obj: &NiObject) -> *const NiStringExtraData;
        fn downcast_ni_integer_extra_data(obj: &NiObject) -> *const NiIntegerExtraData;
        fn downcast_bsx_flags(obj: &NiObject) -> *const BSXFlags;
        fn downcast_ni_float_extra_data(obj: &NiObject) -> *const NiFloatExtraData;
        fn downcast_ni_boolean_extra_data(obj: &NiObject) -> *const NiBooleanExtraData;
        fn downcast_ni_integers_extra_data(obj: &NiObject) -> *const NiIntegersExtraData;

        fn extra_data_name(obj: &NiExtraData) -> &[u8];
        fn string_extra_data_value(obj: &NiStringExtraData) -> &[u8];
        fn integer_extra_data_value(obj: &NiIntegerExtraData) -> u32;
        fn float_extra_data_value(obj: &NiFloatExtraData) -> f32;
        fn boolean_extra_data_value(obj: &NiBooleanExtraData) -> bool;
        fn integers_extra_data_values(obj: &NiIntegersExtraData) -> &[u32];

        fn upcast_bs_extra_data(obj: &BSExtraData) -> &NiObject;
        fn upcast_bs_cloth_extra_data(obj: &BSClothExtraData) -> &BSExtraData;
        fn upcast_bs_collision_query_proxy_extra_data(
            obj: &BSCollisionQueryProxyExtraData,
        ) -> &BSExtraData;

        fn downcast_bs_extra_data(obj: &NiObject) -> *const BSExtraData;
        fn downcast_bs_cloth_extra_data(obj: &NiObject) -> *const BSClothExtraData;
        fn downcast_bs_collision_query_proxy_extra_data(
            obj: &NiObject,
        ) -> *const BSCollisionQueryProxyExtraData;

        fn cloth_extra_data_bytes(obj: &BSClothExtraData) -> &[u8];
        fn collision_query_proxy_extra_data_bytes(obj: &BSCollisionQueryProxyExtraData) -> &[u8];

        // More NiExtraData subclasses.
        fn upcast_ni_binary_extra_data(obj: &NiBinaryExtraData) -> &NiExtraData;
        fn downcast_ni_binary_extra_data(obj: &NiObject) -> *const NiBinaryExtraData;
        fn upcast_ni_floats_extra_data(obj: &NiFloatsExtraData) -> &NiExtraData;
        fn downcast_ni_floats_extra_data(obj: &NiObject) -> *const NiFloatsExtraData;
        fn upcast_ni_strings_extra_data(obj: &NiStringsExtraData) -> &NiExtraData;
        fn downcast_ni_strings_extra_data(obj: &NiObject) -> *const NiStringsExtraData;
        fn upcast_skin_attach(obj: &SkinAttach) -> &NiExtraData;
        fn downcast_skin_attach(obj: &NiObject) -> *const SkinAttach;
        fn upcast_bs_furniture_marker(obj: &BSFurnitureMarker) -> &NiExtraData;
        fn downcast_bs_furniture_marker(obj: &NiObject) -> *const BSFurnitureMarker;
        fn upcast_bs_furniture_marker_node(obj: &BSFurnitureMarkerNode) -> &BSFurnitureMarker;
        fn downcast_bs_furniture_marker_node(obj: &NiObject) -> *const BSFurnitureMarkerNode;
        fn upcast_bs_connect_point_parents(obj: &BSConnectPointParents) -> &NiExtraData;
        fn downcast_bs_connect_point_parents(obj: &NiObject) -> *const BSConnectPointParents;
        fn upcast_bs_connect_point_children(obj: &BSConnectPointChildren) -> &NiExtraData;
        fn downcast_bs_connect_point_children(obj: &NiObject) -> *const BSConnectPointChildren;
        fn upcast_bs_inv_marker(obj: &BSInvMarker) -> &NiExtraData;
        fn downcast_bs_inv_marker(obj: &NiObject) -> *const BSInvMarker;
        fn upcast_bs_bound(obj: &BSBound) -> &NiExtraData;
        fn downcast_bs_bound(obj: &NiObject) -> *const BSBound;
        fn upcast_bs_behavior_graph_extra_data(obj: &BSBehaviorGraphExtraData) -> &NiExtraData;
        fn downcast_bs_behavior_graph_extra_data(obj: &NiObject)
        -> *const BSBehaviorGraphExtraData;

        fn binary_extra_data_bytes(obj: &NiBinaryExtraData) -> &[u8];
        fn floats_extra_data_values(obj: &NiFloatsExtraData) -> &[f32];
        fn strings_extra_data_len(obj: &NiStringsExtraData) -> usize;
        /// Empty if `index` is out of range.
        fn strings_extra_data_value(obj: &NiStringsExtraData, index: usize) -> &[u8];
        fn upcast_bs_packed_combined_shared_geom_data_extra(
            obj: &BSPackedCombinedSharedGeomDataExtra,
        ) -> &NiExtraData;
        fn downcast_bs_packed_combined_shared_geom_data_extra(
            obj: &NiObject,
        ) -> *const BSPackedCombinedSharedGeomDataExtra;
        /// Each instance is 18 floats: the grayscale to palette scale, the rotation (row-major),
        /// translation and scale, and the bounding sphere's center and radius.
        fn packed_shared_header(obj: &BSPackedCombinedSharedGeomDataExtra) -> PackedGeomHeader;
        fn set_packed_shared_header(
            obj: Pin<&mut BSPackedCombinedSharedGeomDataExtra>,
            header: PackedGeomHeader,
        );
        fn packed_shared_objects(
            obj: &BSPackedCombinedSharedGeomDataExtra,
        ) -> Vec<PackedGeomObjectFfi>;
        fn packed_shared_instances(obj: &BSPackedCombinedSharedGeomDataExtra) -> Vec<f32>;
        /// Returns false, changing nothing, if the instance counts don't add up to `instances`.
        fn set_packed_shared_objects(
            obj: Pin<&mut BSPackedCombinedSharedGeomDataExtra>,
            objects: &[PackedGeomObjectFfi],
            instances: &[f32],
        ) -> bool;
        // Setters for existing blocks; the assign_* shims fill new blocks through them.
        // Strings are `bytes` split into `lengths`.
        fn set_extra_data_name(obj: Pin<&mut NiExtraData>, name: &[u8]);
        /// `nextExtraDataRef`, a block index (`NIF_NPOS` if empty), and `numBytes`.
        fn extra_data_next(obj: &NiExtraData) -> u32;
        fn extra_data_num_bytes(obj: &NiExtraData) -> u32;
        fn set_extra_data_num_bytes(obj: Pin<&mut NiExtraData>, v: u32);
        fn set_string_extra_data(obj: Pin<&mut NiStringExtraData>, value: &[u8]);
        fn set_integer_extra_data(obj: Pin<&mut NiIntegerExtraData>, v: u32);
        fn set_float_extra_data(obj: Pin<&mut NiFloatExtraData>, v: f32);
        fn set_boolean_extra_data(obj: Pin<&mut NiBooleanExtraData>, v: bool);
        fn set_integers_extra_data(obj: Pin<&mut NiIntegersExtraData>, values: &[u32]);
        fn set_binary_extra_data(obj: Pin<&mut NiBinaryExtraData>, bytes: &[u8]);
        fn set_floats_extra_data(obj: Pin<&mut NiFloatsExtraData>, values: &[f32]);
        fn set_strings_extra_data(
            obj: Pin<&mut NiStringsExtraData>,
            bytes: &[u8],
            lengths: &[usize],
        );
        fn set_skin_attach_bones(obj: Pin<&mut SkinAttach>, bytes: &[u8], lengths: &[usize]);
        fn set_cloth_extra_data(obj: Pin<&mut BSClothExtraData>, bytes: &[u8]);
        fn set_collision_query_proxy_extra_data(
            obj: Pin<&mut BSCollisionQueryProxyExtraData>,
            bytes: &[u8],
        );
        fn set_color_extra_data(obj: Pin<&mut NiColorExtraData>, c: Float4);
        fn set_vector_extra_data(obj: Pin<&mut NiVectorExtraData>, v: Float4);
        /// False, changing nothing, for more than 65535 weights.
        fn set_vert_weights_extra_data(
            obj: Pin<&mut NiVertWeightsExtraData>,
            weights: &[f32],
        ) -> bool;
        fn set_eye_center_extra_data(obj: Pin<&mut BSEyeCenterExtraData>, values: &[f32]);
        fn set_position_data(obj: Pin<&mut BSPositionData>, values: &[f32]);
        fn set_w_array(obj: Pin<&mut BSWArray>, values: &[u32]);
        /// As in `assign_decal_placement_vector_extra_data`; false, changing nothing, if the
        /// lengths don't add up or exceed 65535.
        fn set_decal_blocks(
            obj: Pin<&mut BSDecalPlacementVectorExtraData>,
            block_lengths: &[usize],
            points: &[Vec3],
            normals: &[Vec3],
        ) -> bool;
        fn set_distant_object_flags(obj: Pin<&mut BSDistantObjectExtraData>, flags: u32);
        fn set_distant_object_large_ref(
            obj: Pin<&mut BSDistantObjectLargeRefExtraData>,
            large_ref: bool,
        );
        fn set_furniture_positions(
            obj: Pin<&mut BSFurnitureMarker>,
            positions: &[FurniturePositionFfi],
        );
        fn set_connect_points(
            obj: Pin<&mut BSConnectPointParents>,
            bytes: &[u8],
            lengths: &[usize],
            transforms: &[ConnectPointTransform],
        );
        fn set_connect_point_skinned(obj: Pin<&mut BSConnectPointChildren>, skinned: bool);
        fn set_connect_point_targets(
            obj: Pin<&mut BSConnectPointChildren>,
            bytes: &[u8],
            lengths: &[usize],
        );
        fn set_inv_marker(obj: Pin<&mut BSInvMarker>, marker: InvMarkerFfi);
        fn set_bound_center(obj: Pin<&mut BSBound>, v: Vec3);
        fn set_bound_half_extents(obj: Pin<&mut BSBound>, v: Vec3);
        fn set_behavior_graph_file(obj: Pin<&mut BSBehaviorGraphExtraData>, graph_file: &[u8]);
        fn set_behavior_graph_controls_base_skeleton(
            obj: Pin<&mut BSBehaviorGraphExtraData>,
            controls: bool,
        );
        fn set_bone_lods(
            obj: Pin<&mut BSBoneLODExtraData>,
            bytes: &[u8],
            lengths: &[usize],
            distances: &[u32],
        );
        /// Key i is `bytes` split into `lengths`, at `times[i]`.
        fn set_text_keys(
            obj: Pin<&mut NiTextKeyExtraData>,
            bytes: &[u8],
            lengths: &[usize],
            times: &[f32],
        );
        /// Keeps the stored count in step.
        fn set_bone_translations(
            obj: Pin<&mut BoneTranslations>,
            bytes: &[u8],
            lengths: &[usize],
            translations: &[Vec3],
        );
        fn skin_attach_bone_count(obj: &SkinAttach) -> usize;
        /// Empty if `index` is out of range.
        fn skin_attach_bone(obj: &SkinAttach, index: usize) -> &[u8];
        fn furniture_positions(obj: &BSFurnitureMarker) -> Vec<FurniturePositionFfi>;
        fn connect_point_count(obj: &BSConnectPointParents) -> usize;
        /// `root` and `variableName` of connect point `index`; empty if out of range.
        fn connect_point_parent(obj: &BSConnectPointParents, index: usize) -> &[u8];
        fn connect_point_name(obj: &BSConnectPointParents, index: usize) -> &[u8];
        /// Identity if out of range.
        fn connect_point_transform(
            obj: &BSConnectPointParents,
            index: usize,
        ) -> ConnectPointTransform;
        fn connect_point_children_skinned(obj: &BSConnectPointChildren) -> bool;
        fn connect_point_children_len(obj: &BSConnectPointChildren) -> usize;
        /// Empty if `index` is out of range.
        fn connect_point_child(obj: &BSConnectPointChildren, index: usize) -> &[u8];
        fn inv_marker(obj: &BSInvMarker) -> InvMarkerFfi;
        fn bound_center(obj: &BSBound) -> Vec3;
        fn bound_half_extents(obj: &BSBound) -> Vec3;
        fn behavior_graph_file(obj: &BSBehaviorGraphExtraData) -> &[u8];
        fn behavior_graph_controls_base_skeleton(obj: &BSBehaviorGraphExtraData) -> bool;

        fn upcast_ni_color_extra_data(obj: &NiColorExtraData) -> &NiExtraData;
        fn downcast_ni_color_extra_data(obj: &NiObject) -> *const NiColorExtraData;
        fn upcast_ni_vector_extra_data(obj: &NiVectorExtraData) -> &NiExtraData;
        fn downcast_ni_vector_extra_data(obj: &NiObject) -> *const NiVectorExtraData;
        fn upcast_ni_vert_weights_extra_data(obj: &NiVertWeightsExtraData) -> &NiExtraData;
        fn downcast_ni_vert_weights_extra_data(obj: &NiObject) -> *const NiVertWeightsExtraData;
        fn upcast_bs_eye_center_extra_data(obj: &BSEyeCenterExtraData) -> &NiExtraData;
        fn downcast_bs_eye_center_extra_data(obj: &NiObject) -> *const BSEyeCenterExtraData;
        fn upcast_bs_position_data(obj: &BSPositionData) -> &NiExtraData;
        fn downcast_bs_position_data(obj: &NiObject) -> *const BSPositionData;
        fn upcast_bs_w_array(obj: &BSWArray) -> &NiExtraData;
        fn downcast_bs_w_array(obj: &NiObject) -> *const BSWArray;
        fn upcast_bs_decal_placement_vector_extra_data(
            obj: &BSDecalPlacementVectorExtraData,
        ) -> &NiFloatExtraData;
        fn downcast_bs_decal_placement_vector_extra_data(
            obj: &NiObject,
        ) -> *const BSDecalPlacementVectorExtraData;
        fn upcast_bs_bone_lod_extra_data(obj: &BSBoneLODExtraData) -> &NiExtraData;
        fn downcast_bs_bone_lod_extra_data(obj: &NiObject) -> *const BSBoneLODExtraData;
        fn upcast_bs_distant_object_extra_data(obj: &BSDistantObjectExtraData) -> &NiExtraData;
        fn downcast_bs_distant_object_extra_data(obj: &NiObject)
        -> *const BSDistantObjectExtraData;
        fn upcast_bs_distant_object_large_ref_extra_data(
            obj: &BSDistantObjectLargeRefExtraData,
        ) -> &NiExtraData;
        fn downcast_bs_distant_object_large_ref_extra_data(
            obj: &NiObject,
        ) -> *const BSDistantObjectLargeRefExtraData;
        fn upcast_bone_translations(obj: &BoneTranslations) -> &NiExtraData;
        fn downcast_bone_translations(obj: &NiObject) -> *const BoneTranslations;
        fn upcast_ni_text_key_extra_data(obj: &NiTextKeyExtraData) -> &NiExtraData;
        fn downcast_ni_text_key_extra_data(obj: &NiObject) -> *const NiTextKeyExtraData;

        fn color_extra_data_value(obj: &NiColorExtraData) -> Float4;
        fn vector_extra_data_value(obj: &NiVectorExtraData) -> Float4;
        fn vert_weights_extra_data_values(obj: &NiVertWeightsExtraData) -> &[f32];
        fn eye_center_extra_data_values(obj: &BSEyeCenterExtraData) -> &[f32];
        /// The half floats, widened.
        fn position_data_values(obj: &BSPositionData) -> Vec<f32>;
        fn w_array_values(obj: &BSWArray) -> &[u32];
        fn decal_block_count(obj: &BSDecalPlacementVectorExtraData) -> usize;
        /// Empty if `block` is out of range.
        fn decal_block_points(obj: &BSDecalPlacementVectorExtraData, block: usize) -> Vec<Vec3>;
        fn decal_block_normals(obj: &BSDecalPlacementVectorExtraData, block: usize) -> Vec<Vec3>;
        fn bone_lod_count(obj: &BSBoneLODExtraData) -> usize;
        /// 0 and empty if `index` is out of range.
        fn bone_lod_distance(obj: &BSBoneLODExtraData, index: usize) -> u32;
        fn bone_lod_bone(obj: &BSBoneLODExtraData, index: usize) -> &[u8];
        fn text_key_count(obj: &NiTextKeyExtraData) -> usize;
        /// Panics if `index` is out of range.
        fn text_key_time(obj: &NiTextKeyExtraData, index: usize) -> f32;
        /// Panics if `index` is out of range.
        fn text_key_text(obj: &NiTextKeyExtraData, index: usize) -> &[u8];
        fn distant_object_flags(obj: &BSDistantObjectExtraData) -> u32;
        fn distant_object_large_ref(obj: &BSDistantObjectLargeRefExtraData) -> bool;
        fn bone_translation_count(obj: &BoneTranslations) -> usize;
        /// Empty and zero if `index` is out of range.
        fn bone_translation_bone(obj: &BoneTranslations, index: usize) -> &[u8];
        fn bone_translation(obj: &BoneTranslations, index: usize) -> Vec3;
    }
}
