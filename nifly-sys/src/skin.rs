//! Skinning blocks from `Skin.hpp`. See `nodes.rs` for the upcast/downcast convention.

#[cxx::bridge]
pub mod ffi {
    #[namespace = "nifly"]
    unsafe extern "C++" {
        include!("Skin.hpp");

        type NiSkinData;
        type NiSkinPartition;
        type NiBoneContainer;
        type NiSkinInstance;
        type BSDismemberSkinInstance;
        type BSSkinInstance;
        type BSSkinBoneData;
    }

    #[namespace = "nifly"]
    extern "C++" {
        include!("Objects.hpp");

        type NiObject = crate::objects::ffi::NiObject;
    }

    extern "C++" {
        include!("nifly-sys/src/object_3d.rs.h");

        type Transform = crate::object_3d::ffi::Transform;
        type BoundingSphere = crate::object_3d::ffi::BoundingSphere;
        type Vec3 = crate::object_3d::ffi::Vec3;
    }

    #[namespace = "nifly_rs"]
    unsafe extern "C++" {
        include!("skin_shim.h");

        fn upcast_ni_skin_data(obj: &NiSkinData) -> &NiObject;
        fn downcast_ni_skin_data(obj: &NiObject) -> *const NiSkinData;
        fn upcast_ni_skin_partition(obj: &NiSkinPartition) -> &NiObject;
        fn downcast_ni_skin_partition(obj: &NiObject) -> *const NiSkinPartition;
        fn upcast_ni_bone_container(obj: &NiBoneContainer) -> &NiObject;
        fn downcast_ni_bone_container(obj: &NiObject) -> *const NiBoneContainer;
        fn upcast_ni_skin_instance(obj: &NiSkinInstance) -> &NiBoneContainer;
        fn downcast_ni_skin_instance(obj: &NiObject) -> *const NiSkinInstance;
        fn upcast_bs_dismember_skin_instance(obj: &BSDismemberSkinInstance) -> &NiSkinInstance;
        fn downcast_bs_dismember_skin_instance(obj: &NiObject) -> *const BSDismemberSkinInstance;
        fn upcast_bs_skin_instance(obj: &BSSkinInstance) -> &NiBoneContainer;
        fn downcast_bs_skin_instance(obj: &NiObject) -> *const BSSkinInstance;
        fn upcast_bs_skin_bone_data(obj: &BSSkinBoneData) -> &NiObject;
        fn downcast_bs_skin_bone_data(obj: &NiObject) -> *const BSSkinBoneData;

        /// `NiSkinData::skinTransform`: global to skin.
        fn skin_data_transform_global_to_skin(data: &NiSkinData) -> Transform;
        /// `NiSkinData::bones.size()`.
        fn skin_data_bone_count(data: &NiSkinData) -> usize;
        /// `NiSkinData::bones[index].boneTransform`: skin to bone. Returns false, leaving `out`
        /// unchanged, if `index` is out of range.
        fn skin_data_transform_skin_to_bone(
            data: &NiSkinData,
            index: usize,
            out: &mut Transform,
        ) -> bool;
        fn set_skin_data_transform_global_to_skin(data: Pin<&mut NiSkinData>, transform: Transform);
        /// `NiSkinData::hasVertWeights`.
        fn skin_data_has_vertex_weights(data: &NiSkinData) -> bool;
        /// `BSDismemberSkinInstance::partitions.size()`.
        fn dismember_partition_count(inst: &BSDismemberSkinInstance) -> usize;
        /// `partitions[index].flags`. 0 if `index` is out of range.
        fn dismember_partition_flags(inst: &BSDismemberSkinInstance, index: usize) -> u16;
        /// `partitions[index].partID`. 0 if `index` is out of range.
        fn dismember_partition_body_part(inst: &BSDismemberSkinInstance, index: usize) -> u16;
        /// Sets `partitions[index].flags`. Returns false if `index` is out of range.
        fn set_dismember_partition_flags(
            inst: Pin<&mut BSDismemberSkinInstance>,
            index: usize,
            flags: u16,
        ) -> bool;
        /// `NiSkinPartition::partitions.size()`.
        fn skin_partition_count(part: &NiSkinPartition) -> usize;
        /// `partitions[index].bones`. Empty if `index` is out of range.
        fn skin_partition_bones(part: &NiSkinPartition, index: usize) -> &[u16];
        /// `partitions[index].vertexMap`. Empty if `index` is out of range.
        fn skin_partition_vertex_map(part: &NiSkinPartition, index: usize) -> &[u16];
        /// `partitions[index].numTriangles`. 0 if `index` is out of range.
        fn skin_partition_triangle_count(part: &NiSkinPartition, index: usize) -> u16;

        /// `NiSkinData::bones[index].bounds`. Returns false, leaving `out` unchanged, if `index`
        /// is out of range.
        fn skin_data_bone_bounds(data: &NiSkinData, index: usize, out: &mut BoundingSphere)
        -> bool;
        fn set_skin_data_has_vertex_weights(data: Pin<&mut NiSkinData>, value: bool);
        /// `BSSkinBoneData::boneXforms`: its size, and an entry's transform and bounds. The
        /// getters return false, leaving `out` unchanged, if `index` is out of range.
        fn skin_bone_data_bone_count(data: &BSSkinBoneData) -> usize;
        fn skin_bone_data_transform_skin_to_bone(
            data: &BSSkinBoneData,
            index: usize,
            out: &mut Transform,
        ) -> bool;
        fn skin_bone_data_bounds(
            data: &BSSkinBoneData,
            index: usize,
            out: &mut BoundingSphere,
        ) -> bool;
        /// `BSSkinInstance::scales`.
        fn skin_instance_scales(inst: &BSSkinInstance) -> Vec<Vec3>;
        fn set_skin_instance_scales(inst: Pin<&mut BSSkinInstance>, scales: &[Vec3]);
        /// `targetRef` of an `NiSkinInstance` or `BSSkinInstance`; `NIF_NPOS` if empty or another
        /// type.
        fn bone_container_root(skin: &NiBoneContainer) -> u32;
        /// `partitions[index]` fields. Getters return empty or 0 if `index` is out of range, and
        /// setters false. Weights and bone indices are flattened 4 per vertex, triangles 3 per
        /// triangle.
        fn skin_partition_weights_per_vertex(part: &NiSkinPartition, index: usize) -> u16;
        fn skin_partition_vertex_weights(part: &NiSkinPartition, index: usize) -> &[f32];
        fn skin_partition_bone_indices(part: &NiSkinPartition, index: usize) -> &[u8];
        fn skin_partition_triangles(part: &NiSkinPartition, index: usize) -> &[u16];
        fn skin_partition_strip_count(part: &NiSkinPartition, index: usize) -> usize;
        fn skin_partition_strip(part: &NiSkinPartition, index: usize, strip: usize) -> &[u16];
        fn skin_partition_lod_level(part: &NiSkinPartition, index: usize) -> u8;
        fn set_skin_partition_lod_level(
            part: Pin<&mut NiSkinPartition>,
            index: usize,
            value: u8,
        ) -> bool;
        fn skin_partition_global_vb(part: &NiSkinPartition, index: usize) -> bool;
        fn set_skin_partition_global_vb(
            part: Pin<&mut NiSkinPartition>,
            index: usize,
            value: bool,
        ) -> bool;

        fn set_skin_data_transform_skin_to_bone(
            data: Pin<&mut NiSkinData>,
            index: usize,
            t: Transform,
        ) -> bool;
        fn set_skin_data_bone_bounds(
            data: Pin<&mut NiSkinData>,
            index: usize,
            b: BoundingSphere,
        ) -> bool;
        /// Bone `index`'s weights as parallel lists. False if `index` is out of range.
        fn skin_data_vertex_weights(
            data: &NiSkinData,
            index: usize,
            vertices: &mut Vec<u16>,
            weights: &mut Vec<f32>,
        ) -> bool;
        /// Keeps `numVertices` in step. False, changing nothing, if `index` is out of range, the
        /// lists differ in length or are longer than 65535.
        fn set_skin_data_vertex_weights(
            data: Pin<&mut NiSkinData>,
            index: usize,
            vertices: &[u16],
            weights: &[f32],
        ) -> bool;
        fn set_skin_bone_data_transform_skin_to_bone(
            data: Pin<&mut BSSkinBoneData>,
            index: usize,
            t: Transform,
        ) -> bool;
        fn set_skin_bone_data_bounds(
            data: Pin<&mut BSSkinBoneData>,
            index: usize,
            b: BoundingSphere,
        ) -> bool;
        fn set_dismember_partition_body_part(
            inst: Pin<&mut BSDismemberSkinInstance>,
            index: usize,
            part: u16,
        ) -> bool;
        /// `which`: 0 `hasVertexMap`, 1 `hasVertexWeights`, 2 `hasFaces`, 3 `hasBoneIndices`.
        fn skin_partition_flag(part: &NiSkinPartition, index: usize, which: u8) -> bool;
        fn set_skin_partition_flag(
            part: Pin<&mut NiSkinPartition>,
            index: usize,
            which: u8,
            value: bool,
        ) -> bool;
        fn skin_partition_vertex_desc(part: &NiSkinPartition) -> u64;
        fn set_skin_partition_vertex_desc(part: Pin<&mut NiSkinPartition>, bits: u64);
        fn skin_partition_block_vertex_desc(part: &NiSkinPartition, index: usize) -> u64;
        fn set_skin_partition_block_vertex_desc(
            part: Pin<&mut NiSkinPartition>,
            index: usize,
            bits: u64,
        ) -> bool;
        /// Flattened, 3 per triangle.
        fn skin_partition_true_triangles(part: &NiSkinPartition, index: usize) -> &[u16];
        /// False, changing nothing, unless there are `numTriangles` triangles.
        fn set_skin_partition_true_triangles(
            part: Pin<&mut NiSkinPartition>,
            index: usize,
            flat: &[u16],
        ) -> bool;
    }
}
