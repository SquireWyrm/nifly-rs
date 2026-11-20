//! `NiNode` and its subclasses.
//!
//! cxx can't expand macros inside a bridge, so each type is listed by hand.
//! For each type `T` with direct base `Parent`:
//! - `upcast_<t>(&T) -> &Parent`
//! - `downcast_<t>(&NiObject) -> *const T`. Null unless the object is a `T` or a subclass of `T`.

#[cxx::bridge]
pub mod ffi {
    #[namespace = "nifly"]
    unsafe extern "C++" {
        include!("Nodes.hpp");
        include!("Particles.hpp");

        type NiNode;
        type BSFadeNode;
        type RootCollisionNode;
        type AvoidNode;
        type NiBSAnimationNode;
        type NiBSParticleNode;
        type BSValueNode;
        type BSLeafAnimNode;
        type BSTreeNode;
        type BSOrderedNode;
        type BSMultiBoundNode;
        type BSDistantObjectInstancedNode;
        type BSRangeNode;
        type BSDebrisNode;
        type BSBlastNode;
        type BSDamageStage;
        type BSWeakReferenceNode;
        type BSFaceGenNiNode;
        type NiBillboardNode;
        type NiSwitchNode;
        type NiLODNode;
        type NiBone;
        type NiSortAdjustNode;
        type BSMasterParticleSystem;
        type BSMultiBound;
        type NiLODData;
        type NiRangeLODData;
        type NiScreenLODData;
        type BSMultiBoundData;
        type BSMultiBoundOBB;
        type BSMultiBoundAABB;
        type BSMultiBoundSphere;
        type NiCamera;
        type NiAVObjectPalette;
        type NiDefaultAVObjectPalette;
    }

    #[namespace = "nifly"]
    extern "C++" {
        include!("Objects.hpp");
        include!("NifFile.hpp");

        type NiObject = crate::objects::ffi::NiObject;
        type NiAVObject = crate::objects::ffi::NiAVObject;
        type NifFile = crate::nif_file::ffi::NifFile;
    }

    extern "C++" {
        include!("nifly-sys/src/object_3d.rs.h");

        type Vec3 = crate::object_3d::ffi::Vec3;
        type BoundingSphere = crate::object_3d::ffi::BoundingSphere;
        type Transform = crate::object_3d::ffi::Transform;
    }

    #[namespace = "nifly_rs"]
    unsafe extern "C++" {
        include!("nodes_shim.h");

        fn upcast_bs_multi_bound(obj: &BSMultiBound) -> &NiObject;
        fn downcast_bs_multi_bound(obj: &NiObject) -> *const BSMultiBound;
        fn upcast_bs_multi_bound_data(obj: &BSMultiBoundData) -> &NiObject;
        fn downcast_bs_multi_bound_data(obj: &NiObject) -> *const BSMultiBoundData;
        fn upcast_bs_multi_bound_obb(obj: &BSMultiBoundOBB) -> &BSMultiBoundData;
        fn downcast_bs_multi_bound_obb(obj: &NiObject) -> *const BSMultiBoundOBB;
        fn upcast_bs_multi_bound_aabb(obj: &BSMultiBoundAABB) -> &BSMultiBoundData;
        fn downcast_bs_multi_bound_aabb(obj: &NiObject) -> *const BSMultiBoundAABB;
        fn upcast_bs_multi_bound_sphere(obj: &BSMultiBoundSphere) -> &BSMultiBoundData;
        fn downcast_bs_multi_bound_sphere(obj: &NiObject) -> *const BSMultiBoundSphere;

        /// Multi-bound data fields; rotations are 3 rows.
        fn multi_bound_obb_center(b: &BSMultiBoundOBB) -> Vec3;
        fn set_multi_bound_obb_center(b: Pin<&mut BSMultiBoundOBB>, v: Vec3);
        fn multi_bound_obb_size(b: &BSMultiBoundOBB) -> Vec3;
        fn set_multi_bound_obb_size(b: Pin<&mut BSMultiBoundOBB>, v: Vec3);
        fn multi_bound_obb_rotation(b: &BSMultiBoundOBB, out: &mut [Vec3]);
        fn set_multi_bound_obb_rotation(b: Pin<&mut BSMultiBoundOBB>, rows: &[Vec3]);
        fn multi_bound_aabb_center(b: &BSMultiBoundAABB) -> Vec3;
        fn set_multi_bound_aabb_center(b: Pin<&mut BSMultiBoundAABB>, v: Vec3);
        fn multi_bound_aabb_half_extent(b: &BSMultiBoundAABB) -> Vec3;
        fn set_multi_bound_aabb_half_extent(b: Pin<&mut BSMultiBoundAABB>, v: Vec3);
        fn multi_bound_sphere_center(b: &BSMultiBoundSphere) -> Vec3;
        fn set_multi_bound_sphere_center(b: Pin<&mut BSMultiBoundSphere>, v: Vec3);
        fn multi_bound_sphere_radius(b: &BSMultiBoundSphere) -> f32;
        fn set_multi_bound_sphere_radius(b: Pin<&mut BSMultiBoundSphere>, radius: f32);
        /// `BSMultiBoundNode::cullingMode`, as the raw `BSCPCullingType` value.
        fn multi_bound_node_culling_mode(node: &BSMultiBoundNode) -> u32;
        fn set_multi_bound_node_culling_mode(node: Pin<&mut BSMultiBoundNode>, v: u32);
        /// `BSMultiBoundNode::multiBoundRef` and `BSMultiBound::dataRef` block indices;
        /// `NIF_NPOS` if empty.
        fn multi_bound_node_bound(node: &BSMultiBoundNode) -> u32;
        fn multi_bound_data(bound: &BSMultiBound) -> u32;

        fn upcast_ni_camera(obj: &NiCamera) -> &NiAVObject;
        fn downcast_ni_camera(obj: &NiObject) -> *const NiCamera;
        /// `NiCamera` fields. The frustum is left, right, top, bottom, near, far; the viewport
        /// left, right, top, bottom. `sceneRef` is a block index (`NIF_NPOS` if empty).
        fn camera_flags(c: &NiCamera) -> u16;
        fn set_camera_flags(c: Pin<&mut NiCamera>, v: u16);
        fn camera_frustum(c: &NiCamera, out: &mut [f32]);
        fn set_camera_frustum(c: Pin<&mut NiCamera>, v: &[f32]);
        fn camera_orthographic(c: &NiCamera) -> bool;
        fn set_camera_orthographic(c: Pin<&mut NiCamera>, v: bool);
        fn camera_viewport(c: &NiCamera, out: &mut [f32]);
        fn set_camera_viewport(c: Pin<&mut NiCamera>, v: &[f32]);
        fn camera_lod_adjust(c: &NiCamera) -> f32;
        fn set_camera_lod_adjust(c: Pin<&mut NiCamera>, v: f32);
        fn camera_scene(c: &NiCamera) -> u32;
        /// Sets the scene of the camera at `camera` to the `NiAVObject` at `scene` (`NIF_NPOS`
        /// clears it). Returns false, changing nothing, if a block has the wrong type.
        fn set_camera_scene(file: Pin<&mut NifFile>, camera: u32, scene: u32) -> bool;
        /// Adds a new `NiCamera`, with the file format's default settings, as the last child of
        /// the node at `parent_index` (`NIF_NPOS`: the root). Returns its index, or `NIF_NPOS`
        /// (changing nothing) if there's no such node.
        fn add_camera(
            file: Pin<&mut NifFile>,
            name: &[u8],
            transform: Transform,
            parent_index: u32,
        ) -> u32;

        fn upcast_ni_av_object_palette(obj: &NiAVObjectPalette) -> &NiObject;
        fn downcast_ni_av_object_palette(obj: &NiObject) -> *const NiAVObjectPalette;
        fn upcast_ni_default_av_object_palette(
            obj: &NiDefaultAVObjectPalette,
        ) -> &NiAVObjectPalette;
        fn downcast_ni_default_av_object_palette(obj: &NiObject)
        -> *const NiDefaultAVObjectPalette;
        /// `NiDefaultAVObjectPalette::sceneRef` and the objects' `objectRef`s are block indices
        /// (`NIF_NPOS` if empty). `i` must be below the count.
        fn palette_scene(p: &NiDefaultAVObjectPalette) -> u32;
        fn palette_object_count(p: &NiDefaultAVObjectPalette) -> usize;
        fn palette_object_name(p: &NiDefaultAVObjectPalette, i: usize) -> &[u8];
        fn palette_object(p: &NiDefaultAVObjectPalette, i: usize) -> u32;
        /// Sets the scene of the palette at `palette` to the `NiAVObject` at `scene` (`NIF_NPOS`
        /// clears it). Returns false, changing nothing, if a block has the wrong type.
        fn set_palette_scene(file: Pin<&mut NifFile>, palette: u32, scene: u32) -> bool;
        /// Replaces the objects of the palette at `palette`: object i is named by the next
        /// `lengths[i]` bytes of `names` and refers to `objects[i]` (`NIF_NPOS`: none). Returns
        /// false, changing nothing, if a block has the wrong type or the lengths don't match.
        fn set_palette_objects(
            file: Pin<&mut NifFile>,
            palette: u32,
            names: &[u8],
            lengths: &[usize],
            objects: &[u32],
        ) -> bool;

        fn upcast_ni_lod_data(obj: &NiLODData) -> &NiObject;
        fn downcast_ni_lod_data(obj: &NiObject) -> *const NiLODData;
        fn upcast_ni_range_lod_data(obj: &NiRangeLODData) -> &NiLODData;
        fn downcast_ni_range_lod_data(obj: &NiObject) -> *const NiRangeLODData;
        fn upcast_ni_screen_lod_data(obj: &NiScreenLODData) -> &NiLODData;
        fn downcast_ni_screen_lod_data(obj: &NiObject) -> *const NiScreenLODData;
        /// `NiLODNode::lodLevelData` block index; `NIF_NPOS` if empty.
        fn lod_node_data(node: &NiLODNode) -> u32;
        /// LOD data fields. Range levels are flattened near, far pairs.
        fn range_lod_center(d: &NiRangeLODData) -> Vec3;
        fn set_range_lod_center(d: Pin<&mut NiRangeLODData>, v: Vec3);
        fn range_lod_levels(d: &NiRangeLODData) -> Vec<f32>;
        fn set_range_lod_levels(d: Pin<&mut NiRangeLODData>, levels: &[f32]);
        fn screen_lod_bound(d: &NiScreenLODData) -> BoundingSphere;
        fn set_screen_lod_bound(d: Pin<&mut NiScreenLODData>, b: BoundingSphere);
        fn screen_lod_world_bound(d: &NiScreenLODData) -> BoundingSphere;
        fn set_screen_lod_world_bound(d: Pin<&mut NiScreenLODData>, b: BoundingSphere);
        fn screen_lod_proportion_levels(d: &NiScreenLODData) -> Vec<f32>;
        fn set_screen_lod_proportion_levels(d: Pin<&mut NiScreenLODData>, levels: &[f32]);

        /// `BSDistantObjectInstancedNode` instances. Resource IDs are 3 `u32`s (file hash,
        /// extension bytes little-endian, directory hash); transforms are flattened, 16 floats
        /// each. Getters return false or empty, and setters false, if `index` is out of range.
        fn distant_instance_count(node: &BSDistantObjectInstancedNode) -> usize;
        fn distant_instance_resource(
            node: &BSDistantObjectInstancedNode,
            index: usize,
            out: &mut [u32],
        ) -> bool;
        fn distant_instance_transforms(
            node: &BSDistantObjectInstancedNode,
            index: usize,
        ) -> Vec<f32>;
        fn set_distant_instance(
            node: Pin<&mut BSDistantObjectInstancedNode>,
            index: usize,
            resource: &[u32],
            transforms: &[f32],
        ) -> bool;
        fn push_distant_instance(
            node: Pin<&mut BSDistantObjectInstancedNode>,
            resource: &[u32],
            transforms: &[f32],
        );
        fn remove_distant_instance(
            node: Pin<&mut BSDistantObjectInstancedNode>,
            index: usize,
        ) -> bool;
        /// An instance's unknown data as (unknown1, unknown2) pairs.
        fn distant_instance_unknown(node: &BSDistantObjectInstancedNode, index: usize) -> Vec<u64>;
        /// False if `index` is out of range or a second value doesn't fit in `u32`.
        fn set_distant_instance_unknown(
            node: Pin<&mut BSDistantObjectInstancedNode>,
            index: usize,
            pairs: &[u64],
        ) -> bool;
        fn distant_texture_arrays_unknown(node: &BSDistantObjectInstancedNode, slot: usize) -> u8;
        fn set_distant_texture_arrays_unknown(
            node: Pin<&mut BSDistantObjectInstancedNode>,
            slot: usize,
            v: u8,
        ) -> bool;
        /// Texture arrays of `slot` (0 to 2). Getters return 0 or empty if out of range; setters
        /// do nothing.
        fn distant_texture_array_count(node: &BSDistantObjectInstancedNode, slot: usize) -> usize;
        fn distant_texture_array_len(
            node: &BSDistantObjectInstancedNode,
            slot: usize,
            array: usize,
        ) -> usize;
        fn distant_texture_array_path(
            node: &BSDistantObjectInstancedNode,
            slot: usize,
            array: usize,
            index: usize,
        ) -> &[u8];
        fn clear_distant_texture_arrays(node: Pin<&mut BSDistantObjectInstancedNode>, slot: usize);
        fn add_distant_texture_array(node: Pin<&mut BSDistantObjectInstancedNode>, slot: usize);
        fn push_distant_texture_array_path(
            node: Pin<&mut BSDistantObjectInstancedNode>,
            slot: usize,
            path: &[u8],
        );

        /// `BSWeakReferenceNode` weak references, with resource IDs and matrices passed as for
        /// `BSDistantObjectInstancedNode`. Material IDs are biome form ID, directory hash and file
        /// hash. Getters return 0, false or empty if an index is out of range. Setters keep the
        /// stored counts in step.
        fn weak_ref_count(node: &BSWeakReferenceNode) -> usize;
        fn weak_ref_resource(node: &BSWeakReferenceNode, index: usize, out: &mut [u32]) -> bool;
        fn weak_ref_form_id(node: &BSWeakReferenceNode, index: usize) -> u32;
        fn weak_ref_transforms(node: &BSWeakReferenceNode, index: usize) -> Vec<f32>;
        fn weak_ref_material_count(node: &BSWeakReferenceNode, index: usize) -> usize;
        fn weak_ref_material(
            node: &BSWeakReferenceNode,
            index: usize,
            material: usize,
            out: &mut [u32],
        ) -> bool;
        fn weak_ref_material_name(
            node: &BSWeakReferenceNode,
            index: usize,
            material: usize,
        ) -> &[u8];
        fn clear_weak_refs(node: Pin<&mut BSWeakReferenceNode>);
        fn push_weak_ref(
            node: Pin<&mut BSWeakReferenceNode>,
            form_id: u32,
            resource: &[u32],
            transforms: &[f32],
        );
        fn push_weak_ref_material(node: Pin<&mut BSWeakReferenceNode>, ids: &[u32], name: &[u8]);
        /// Water references. `resize_water_refs` keeps existing entries' other data.
        fn water_ref_count(node: &BSWeakReferenceNode) -> usize;
        fn water_ref_transform(node: &BSWeakReferenceNode, index: usize) -> Vec<f32>;
        fn water_ref_resource(node: &BSWeakReferenceNode, index: usize, out: &mut [u32]) -> bool;
        fn water_ref_material(node: &BSWeakReferenceNode, index: usize) -> &[u8];
        fn water_ref_unknown(node: &BSWeakReferenceNode, index: usize) -> u32;
        fn set_water_ref_unknown(node: Pin<&mut BSWeakReferenceNode>, index: usize, v: u32)
        -> bool;
        fn resize_water_refs(node: Pin<&mut BSWeakReferenceNode>, count: usize);
        fn set_water_ref(
            node: Pin<&mut BSWeakReferenceNode>,
            index: usize,
            transform: &[f32],
            resource: &[u32],
            material: &[u8],
        ) -> bool;

        fn upcast_ni_node(obj: &NiNode) -> &NiAVObject;
        fn upcast_bs_fade_node(obj: &BSFadeNode) -> &NiNode;
        fn upcast_root_collision_node(obj: &RootCollisionNode) -> &NiNode;
        fn upcast_avoid_node(obj: &AvoidNode) -> &NiNode;
        fn upcast_ni_bs_animation_node(obj: &NiBSAnimationNode) -> &NiNode;
        fn upcast_ni_bs_particle_node(obj: &NiBSParticleNode) -> &NiNode;
        fn upcast_bs_value_node(obj: &BSValueNode) -> &NiNode;
        fn upcast_bs_leaf_anim_node(obj: &BSLeafAnimNode) -> &NiNode;
        fn upcast_bs_tree_node(obj: &BSTreeNode) -> &NiNode;
        fn upcast_bs_ordered_node(obj: &BSOrderedNode) -> &NiNode;
        fn upcast_bs_multi_bound_node(obj: &BSMultiBoundNode) -> &NiNode;
        fn upcast_bs_distant_object_instanced_node(
            obj: &BSDistantObjectInstancedNode,
        ) -> &BSMultiBoundNode;
        fn upcast_bs_range_node(obj: &BSRangeNode) -> &NiNode;
        fn upcast_bs_debris_node(obj: &BSDebrisNode) -> &BSRangeNode;
        fn upcast_bs_blast_node(obj: &BSBlastNode) -> &BSRangeNode;
        fn upcast_bs_damage_stage(obj: &BSDamageStage) -> &BSBlastNode;
        fn upcast_bs_weak_reference_node(obj: &BSWeakReferenceNode) -> &NiNode;
        fn upcast_bs_face_gen_ni_node(obj: &BSFaceGenNiNode) -> &NiNode;
        fn upcast_ni_billboard_node(obj: &NiBillboardNode) -> &NiNode;
        fn upcast_ni_switch_node(obj: &NiSwitchNode) -> &NiNode;
        fn upcast_ni_lod_node(obj: &NiLODNode) -> &NiSwitchNode;
        fn upcast_ni_bone(obj: &NiBone) -> &NiNode;
        fn upcast_ni_sort_adjust_node(obj: &NiSortAdjustNode) -> &NiNode;
        fn upcast_bs_master_particle_system(obj: &BSMasterParticleSystem) -> &NiNode;

        fn downcast_ni_node(obj: &NiObject) -> *const NiNode;
        fn downcast_bs_fade_node(obj: &NiObject) -> *const BSFadeNode;
        fn downcast_root_collision_node(obj: &NiObject) -> *const RootCollisionNode;
        fn downcast_avoid_node(obj: &NiObject) -> *const AvoidNode;
        fn downcast_ni_bs_animation_node(obj: &NiObject) -> *const NiBSAnimationNode;
        fn downcast_ni_bs_particle_node(obj: &NiObject) -> *const NiBSParticleNode;
        fn downcast_bs_value_node(obj: &NiObject) -> *const BSValueNode;
        fn downcast_bs_leaf_anim_node(obj: &NiObject) -> *const BSLeafAnimNode;
        fn downcast_bs_tree_node(obj: &NiObject) -> *const BSTreeNode;
        fn downcast_bs_ordered_node(obj: &NiObject) -> *const BSOrderedNode;
        fn downcast_bs_multi_bound_node(obj: &NiObject) -> *const BSMultiBoundNode;
        fn downcast_bs_distant_object_instanced_node(
            obj: &NiObject,
        ) -> *const BSDistantObjectInstancedNode;
        fn downcast_bs_range_node(obj: &NiObject) -> *const BSRangeNode;
        fn downcast_bs_debris_node(obj: &NiObject) -> *const BSDebrisNode;
        fn downcast_bs_blast_node(obj: &NiObject) -> *const BSBlastNode;
        fn downcast_bs_damage_stage(obj: &NiObject) -> *const BSDamageStage;
        fn downcast_bs_weak_reference_node(obj: &NiObject) -> *const BSWeakReferenceNode;
        fn downcast_bs_face_gen_ni_node(obj: &NiObject) -> *const BSFaceGenNiNode;
        fn downcast_ni_billboard_node(obj: &NiObject) -> *const NiBillboardNode;
        fn downcast_ni_switch_node(obj: &NiObject) -> *const NiSwitchNode;
        fn downcast_ni_lod_node(obj: &NiObject) -> *const NiLODNode;
        fn downcast_ni_bone(obj: &NiObject) -> *const NiBone;
        fn downcast_ni_sort_adjust_node(obj: &NiObject) -> *const NiSortAdjustNode;
        fn downcast_bs_master_particle_system(obj: &NiObject) -> *const BSMasterParticleSystem;

        /// Static `NifFile::CanDeleteNode(NiNode*)`: true if all of the node's refs are empty.
        fn can_delete_node(node: &NiNode) -> bool;

        fn child_count(node: &NiNode) -> usize;

        /// Block indices of the node's child refs, in order. `u32::MAX` (`NIF_NPOS`) for empty refs.
        fn child_ref_indices(node: &NiNode) -> Vec<u32>;

        /// Node subclass fields: `BSValueNode::value` and `valueFlags`,
        /// `BSOrderedNode::isStaticBound`, `BSRangeNode::min`, `max` and `current`,
        /// `NiBillboardNode::billboardMode`, `NiSwitchNode::flags` and `index`,
        /// `NiSortAdjustNode::sortingMode`.
        fn value_node_value(node: &BSValueNode) -> i32;
        fn set_value_node_value(node: Pin<&mut BSValueNode>, value: i32);
        fn value_node_flags(node: &BSValueNode) -> u8;
        fn set_value_node_flags(node: Pin<&mut BSValueNode>, value: u8);
        fn ordered_node_static_bound(node: &BSOrderedNode) -> bool;
        fn set_ordered_node_static_bound(node: Pin<&mut BSOrderedNode>, value: bool);
        fn range_node_min(node: &BSRangeNode) -> u8;
        fn set_range_node_min(node: Pin<&mut BSRangeNode>, value: u8);
        fn range_node_max(node: &BSRangeNode) -> u8;
        fn set_range_node_max(node: Pin<&mut BSRangeNode>, value: u8);
        fn range_node_current(node: &BSRangeNode) -> u8;
        fn set_range_node_current(node: Pin<&mut BSRangeNode>, value: u8);
        fn billboard_mode(node: &NiBillboardNode) -> u16;
        fn set_billboard_mode(node: Pin<&mut NiBillboardNode>, value: u16);
        fn switch_node_flags(node: &NiSwitchNode) -> u16;
        fn set_switch_node_flags(node: Pin<&mut NiSwitchNode>, value: u16);
        fn switch_node_index(node: &NiSwitchNode) -> u32;
        fn set_switch_node_index(node: Pin<&mut NiSwitchNode>, value: u32);
        fn sorting_mode(node: &NiSortAdjustNode) -> u32;
        fn set_sorting_mode(node: Pin<&mut NiSortAdjustNode>, value: u32);
        fn face_gen_unknown(node: &BSFaceGenNiNode) -> u16;
        fn set_face_gen_unknown(node: Pin<&mut BSFaceGenNiNode>, value: u16);
        fn weak_ref_unknown(node: &BSWeakReferenceNode) -> u32;
        fn set_weak_ref_unknown(node: Pin<&mut BSWeakReferenceNode>, value: u32);
        fn master_particle_max_emitters(node: &BSMasterParticleSystem) -> u16;
        fn set_master_particle_max_emitters(node: Pin<&mut BSMasterParticleSystem>, value: u16);
        fn camera_screen_polygons(node: &NiCamera) -> u32;
        fn set_camera_screen_polygons(node: Pin<&mut NiCamera>, value: u32);
        fn camera_screen_textures(node: &NiCamera) -> u32;
        fn set_camera_screen_textures(node: Pin<&mut NiCamera>, value: u32);

        /// `BSOrderedNode::alphaSortBound` as x, y, z, w. Both slices have 4 elements.
        fn ordered_node_alpha_sort_bound(node: &BSOrderedNode, out: &mut [f32]);
        fn set_ordered_node_alpha_sort_bound(node: Pin<&mut BSOrderedNode>, v: &[f32]);

        /// Block indices of `BSTreeNode::bones1` (`list` 1) or `bones2` (`list` 2), in order.
        /// `u32::MAX` for empty refs.
        fn tree_node_bones(node: &BSTreeNode, list: u8) -> Vec<u32>;
    }
}
