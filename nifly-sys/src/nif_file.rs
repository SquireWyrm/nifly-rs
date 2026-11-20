
// cxx's generated wrapper for `object_mut` doesn't carry its `# Safety` doc where clippy looks.
#[allow(clippy::missing_safety_doc)]
#[cxx::bridge]
pub mod ffi {
    struct LoadResult {
        code: i32,
        value: UniquePtr<NifFile>,
    }

    /// `nifly::NiVersion`: file version, user version and stream (user version 2).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct NifVersion {
        file: u32,
        user: u32,
        stream: u32,
    }

    /// Result of `NifFile::GetTextureSlot`: nifly's return code and the path it filled in.
    struct TextureSlot {
        /// 0 = not found, 1 = BSShaderTextureSet, 2 = BSEffectShaderProperty,
        /// 3 = NiTexturingProperty.
        source: u32,
        path: Vec<u8>,
    }

    /// `nifly::NifLoadOptions`.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct LoadOptions {
        is_terrain: bool,
    }

    /// `nifly::NifSaveOptions`.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct SaveOptions {
        optimize: bool,
        sort_blocks: bool,
    }

    /// `nifly::OptOptions`.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct OptimizeOptions {
        target: NifVersion,
        head_parts: bool,
        remove_parallax: bool,
        calc_bounds: bool,
        fix_bsx_flags: bool,
        fix_shader_flags: bool,
    }

    /// `nifly::OptResult`. The lists hold shape names.
    struct OptimizeResult {
        version_mismatch: bool,
        dupes_renamed: bool,
        vertex_colors_removed: UniquePtr<CxxVector<CxxString>>,
        normals_removed: UniquePtr<CxxVector<CxxString>>,
        partitions_triangulated: UniquePtr<CxxVector<CxxString>>,
        tangents_added: UniquePtr<CxxVector<CxxString>>,
        parallax_removed: UniquePtr<CxxVector<CxxString>>,
    }

    /// A texture coordinate.
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Uv {
        u: f32,
        v: f32,
    }

    /// An RGBA color.
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Color {
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    }

    /// A segment of a `BSSubIndexTriShape`: how many of the following subsegments are its.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct SegmentFfi {
        subsegments: u32,
    }

    /// A subsegment's data; `extra_len` floats of extra data follow in a separate list.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct SubsegmentFfi {
        user_slot: u32,
        material: u32,
        extra_len: u32,
    }

    /// A dismember partition's flags and body part.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct PartitionInfo {
        flags: u16,
        body_part: u16,
    }

    /// A triangle's three vertex indices.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Triangle {
        p1: u16,
        p2: u16,
        p3: u16,
    }

    /// One vertex's weight for a bone.
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct VertexWeight {
        vertex: u16,
        weight: f32,
    }
    
    #[namespace = "read_bridge"]
    extern "C++" {
        include!("nifly-sys/src/read_bridge.rs.h");

        type RustReader<'a> = crate::read_bridge::RustReader<'a>;
    }

    #[namespace = "write_bridge"]
    extern "C++" {
        include!("nifly-sys/src/write_bridge.rs.h");

        type RustWriter<'a> = crate::write_bridge::RustWriter<'a>;
    }

    extern "C++" {
        type Transform = crate::object_3d::ffi::Transform;
        type BoundingSphere = crate::object_3d::ffi::BoundingSphere;
        type Vec3 = crate::object_3d::ffi::Vec3;
        type FurniturePositionFfi = crate::extra_data_types::ffi::FurniturePositionFfi;
        type ConnectPointTransform = crate::extra_data_types::ffi::ConnectPointTransform;
        type InvMarkerFfi = crate::extra_data_types::ffi::InvMarkerFfi;
        type PackedGeomHeader = crate::extra_data_types::ffi::PackedGeomHeader;
        type PackedGeomObjectFfi = crate::extra_data_types::ffi::PackedGeomObjectFfi;
        type Float4 = crate::extra_data_types::ffi::Float4;
    }

    #[namespace = "nifly"]
    extern "C++" {
        include!("Objects.hpp");
        include!("Nodes.hpp");
        include!("Geometry.hpp");
        include!("Shaders.hpp");
        include!("Skin.hpp");

        type NiObject = crate::objects::ffi::NiObject;
        type NiObjectNET = crate::objects::ffi::NiObjectNET;
        type NiNode = crate::nodes::ffi::NiNode;
        type NiShape = crate::geometry::ffi::NiShape;
        type NiGeometryData = crate::geometry::ffi::NiGeometryData;
        type NiShader = crate::shaders::ffi::NiShader;
        type BSShaderTextureSet = crate::shaders::ffi::BSShaderTextureSet;
        type NiMaterialProperty = crate::shaders::ffi::NiMaterialProperty;
        type NiAlphaProperty = crate::shaders::ffi::NiAlphaProperty;
        type NiStencilProperty = crate::shaders::ffi::NiStencilProperty;
        type NiTexturingProperty = crate::shaders::ffi::NiTexturingProperty;
        type NiSkinData = crate::skin::ffi::NiSkinData;
    }

    #[namespace = "nifly"]
    unsafe extern "C++" {
        include!("NifFile.hpp");
        type NifFile;
        // GetHeader: not bound; shims use it, and header operations are bound individually
        // CopyFrom: bound through the copy constructor as `clone_nif` (shim), for `Clone`
        // Load: bound as `load_nif` (shim)
        // Save: bound as `save_nif` (shim)
        fn Optimize(self: Pin<&mut NifFile>);
        // OptimizeFor: bound as `optimize_for` (shim)
        fn TrimTexturePaths(self: Pin<&mut NifFile>);
        fn PrepareData(self: Pin<&mut NifFile>);
        fn FinalizeData(self: Pin<&mut NifFile>);
        fn IsValid(&self) -> bool;
        fn HasUnknown(&self) -> bool;
        fn IsTerrain(&self) -> bool;
        fn IsSSECompatible(&self) -> bool;
        // IsSSECompatible (shape): bound as `is_shape_sse_compatible` (shim)
        // Create: bound as `create_nif` (shim)
        fn Clear(self: Pin<&mut NifFile>);
        fn LinkGeomData(self: Pin<&mut NifFile>);
        fn RemoveInvalidTris(&self);
        // RenameDuplicateShapes
        fn RenameDuplicateShapes(self: Pin<&mut NifFile>) -> bool;
        #[Self = "NifFile"]
        fn GetVertexLimit() -> usize;
        fn GetTriangleLimit(&self) -> usize;
        // AddNode: bound as `add_node` (shim); `add_node_of` creates node subclasses the same way
        // DeleteNode: bound as `delete_node` (shim)
        // CanDeleteNode: bound as `nodes::can_delete_node` (shim)
        // CanDeleteNode (name): bound as `can_delete_node_by_name` (shim)
        // GetNodeName: not bound; read `NiObjectNet::name` (upstream also maps "" to "_unnamed_")
        // SetNodeName: not bound; use `NiObjectNet::set_name`
        // AssignExtraData: reimplemented for any NiObjectNET; bound as `assign_*_extra_data`, `assign_bsx_flags`,
        //   `assign_furniture_marker`, `assign_connect_point_*`, `assign_inv_marker` and
        //   `assign_bs_bound` (shims), one per value type; `assign_bs_*_extra_data` attach
        //   BSExtraData blocks the same way
        // SetShapeOrder: bound as `set_shape_order` (shim)
        // SetSortIndices: bound as `sort_visit` (shim), used by the safe crate's `BlockSorter`
        // SortNiObjectNET, SortAVObject, SortController, SortCollision, SortShape, SortGraph:
        //   not bound; the per-type steps SetSortIndices dispatches to, run by `sort_visit`
        fn PrettySortBlocks(self: Pin<&mut NifFile>);
        fn FixBSXFlags(self: Pin<&mut NifFile>);
        fn FixShaderFlags(self: Pin<&mut NifFile>);
        // DeleteUnreferencedBlocks: template; bound as `delete_unreferenced_blocks` (shim) with a
        //   Rust type check in place of the template parameter
        // DeleteUnreferencedNodes: bound as `delete_unreferenced_nodes` (shim)
        // FindBlockByName: template; reimplemented in Rust over `block` and `objects::object_name`
        // GetBlockID: bound as `block_index` (shim)
        // GetParentNode: bound as `parent_node_index` (shim), exposed as `View::parent`
        // SetParentNode: bound as `set_parent_node` (shim), exposed as `Edit::set_parent`, rejecting non-NiAVObject children and cycles
        // GetNodes: returns std::vector<NiNode*>, which cxx can't bind. Built in Rust from `block_count`/`block`.
        // GetShader: bound as `View::shader` (shim)
        // GetMaterialProperty: bound as `View::material_property` (shim)
        // GetStencilProperty: bound as `View::stencil_property` (shim)
        // GetTexturingProperty: bound as `View::texturing_property` (shim)
        // GetGeometryData: bound as `View::geometry_data` (shim)
        // GetExternalGeometryPathRefs: bound as `external_mesh_count` / `external_mesh_path` and
        //   `set_external_mesh_path` (shims)
        // SaveExternalShapeData: bound as `save_external_mesh` (shim), into a buffer, from a copy
        // LoadExternalShapeData: reimplemented as `load_external_mesh` (shim), replacing the mesh data
        //   only on success
        // GetTexturePathRefs: bound as `texture_path_count` / `texture_path` (shims), read-only
        // GetTextureSlot: bound as `View::texture_slot` (shim)
        // SetTextureSlot: bound as `Edit::set_texture_slot` (shim); the NiTexturingProperty part is
        //   reimplemented so it keeps higher maps and creates missing NiSourceTextures
        // CloneChildren: bound within a file as `clone_children` (shim); cross-file use is internal
        //   to the clone functions
        // CloneShape: bound as `Edit::clone_shape` / `clone_shape_from` (shims), cross-file only between
        //   files of the same game
        // CloneNamedNode: reimplemented on a given node as `clone_node` / `clone_node_from` (shims)
        // CreateShapeFromData: bound as `create_shape` (shim), with validation. Oblivion and
        //   Starfield are refused because of an upstream issue: their files get Fallout 3 blocks
        //   (see the shim).
        // GetShapeNames: `shape_names()` in Rust, over `block`
        // GetShapes: `shapes()` in Rust, over `block`
        // RenameShape: not bound; use `NiObjectNet::set_name`
        // TriangulateShape: bound as `Edit::triangulate` (shim), with validation
        // GetChildren: template returning std::vector<T*>, which cxx can't bind.
        // Built in Rust from `nodes::child_ref_indices`, `extra_data_indices` and `block`.
        /// Borrowed from the file's block list. Null if the file has no node blocks.
        fn GetRootNode(self: &NifFile) -> *mut NiNode;
        // GetTree: fills std::vector<NiObject*>, which cxx can't bind. Built in Rust from `objects::child_indices` and `block`.
        // GetNodeTransformToParent: built in Rust from `block`, `name_bytes` and `transform`
        // GetNodeTransform: deprecated alias of GetNodeTransformToParent
        // GetNodeTransformToGlobal: bound as `node_transform_to_global` (shim)
        // GetAbsoluteNodeTransform: deprecated alias of GetNodeTransformToGlobal
        // SetNodeTransformToParent: bound as `set_node_transform_to_parent` (shim)
        // SetNodeTransform: deprecated alias of SetNodeTransformToParent
        // GetShapeBoneList: bound as `View::bone_list` (shim)
        // GetShapeBoneIDList: bound as `View::bone_ids` (shim)
        // SetShapeBoneIDList: bound as `Edit::set_bone_ids` (shim), requiring node blocks
        // SetShapeBoneList: names resolved to nodes in Rust (rejecting missing ones) for
        // `Edit::set_bone_ids`; bound as `set_shape_bone_names` (shim) for BSGeometry
        // GetShapeBoneWeights: bound as `View::bone_weights` (shim)
        // CalcShapeTransformGlobalToSkin: bound as `View::calc_transform_global_to_skin` (shim)
        // GetShapeTransformGlobalToSkin: bound as `View::transform_global_to_skin` (shim)
        // SetShapeTransformGlobalToSkin: bound as the `NiSkinData` setter `set_transform_global_to_skin`
        //   (skin shim)
        // GetShapeTransformSkinToBone (name): the safe crate uses the index form
        // GetShapeTransformSkinToBone (index): bound as `View::transform_skin_to_bone` (shim)
        // SetShapeTransformSkinToBone: bound as `Edit::set_transform_skin_to_bone` (shim)
        // GetShapeBoneTransform (name, index): deprecated; dispatches to GetShapeTransformGlobalToSkin
        //   or GetShapeTransformSkinToBone
        // SetShapeBoneTransform: deprecated; dispatches to SetShapeTransformGlobalToSkin (bone index
        //   0xFFFFFFFF) or SetShapeTransformSkinToBone, both bound
        // SetShapeBoneBounds: bound as `Edit::set_bone_bounds` (shim), on a shape rather than a name
        // GetShapeBoneBounds: bound as `View::bone_bounds` (shim)
        // UpdateShapeBoneID: reimplemented as `Edit::replace_bone` (shim), on a shape index, with
        // validation
        // SetShapeBoneWeights: bound as `Edit::set_bone_weights` (shim), on a shape rather than a name
        // SetShapeVertWeights: reimplemented as `Edit::set_vertex_weights` (shim), on a shape index,
        // with validation
        // ClearShapeVertWeights: reimplemented as `Edit::clear_vertex_weights` (shim), on a shape
        // index, with validation
        // GetShapeSegments: bound as `View::segments` (shim), flattened
        // SetShapeSegments: bound as `Edit::set_segments` (shim), flattened, with validation
        // GetShapePartitions: reimplemented as `View::partitions` (shim), on a copy of the partition
        // SetShapePartitions: bound as `Edit::set_partitions` (shim), with validation, followed by
        //   UpdateSkinPartitions
        // SetDefaultPartition: reimplemented in Rust over `Edit::set_partitions`
        // DeletePartitions: bound as `Edit::delete_partitions` (shim), sorting the indices, with validation
        // ReorderTriangles: bound as `Edit::reorder_triangles` (shim), requiring a permutation
        // GetVertsForShape: pointer form not bound; the copy form is
        // GetNormalsForShape: reimplemented without caching as `View::normals` (shim)
        // GetUvsForShape: pointer form not bound; the copy form is
        // GetColorsForShape (name): not bound; use the shape form
        // GetColorsForShape (shape): pointer form not bound; the copy form is
        // GetTangentsForShape: pointer form not bound; the copy form is
        // GetBitangentsForShape: pointer form not bound; the copy form is
        // GetEyeDataForShape: pointer form not bound; the copy form is
        // GetVertsForShape (copy): bound as `View::vertices` (shim)
        // GetUvsForShape (copy): bound as `View::uvs` (shim)
        // GetColorsForShape (copy): bound as `View::vertex_colors` (shim)
        // GetTangentsForShape (copy): bound as `View::tangents` (shim)
        // GetBitangentsForShape (copy): bound as `View::bitangents` (shim)
        // GetEyeDataForShape (copy): bound as `View::eye_data` (shim)
        // SetVertsForShape: bound as `Edit::set_vertices` (shim), requiring one value per vertex
        // SetUvsForShape: bound as `Edit::set_uvs` (shim), requiring one value per vertex
        // SetColorsForShape (shape): bound as `Edit::set_vertex_colors` (shim), requiring one value per vertex
        // SetColorsForShape (name): not bound; use the shape form
        // SetTangentsForShape: bound as `Edit::set_tangents` (shim), requiring one value per vertex
        // SetBitangentsForShape: bound as `Edit::set_bitangents` (shim), requiring one value per vertex
        // SetEyeDataForShape: bound as `Edit::set_eye_data` (shim), requiring one value per vertex
        // GetBinaryTangentData, SetBinaryTangentData, DeleteBinaryTangentData: not bound. For
        //   Oblivion, PrepareData moves this NiBinaryExtraData blob into the shape's tangents and
        //   bitangents, and FinalizeData (on save) rebuilds it from them or deletes it, so
        //   `View::tangents` / `Edit::set_tangents` already cover it, and a direct write to the
        //   blob would be overwritten on save.
        // InvertUVsForShape: bound as `Edit::invert_uvs` (shim), with validation
        // MirrorShape: bound as `Edit::mirror` (shim), with validation
        // SetNormalsForShape: bound as `Edit::set_normals` (shim), requiring one value per vertex
        // CalcNormalsForShape: bound as `Edit::calc_normals` (shim), with validation
        // CalcTangentsForShape: bound as `Edit::calc_tangents` (shim), with validation
        // ApplyNormalsFromFile: reimplemented as `Edit::apply_locked_normals` (shim), on a shape index
        // and source shape, keeping shared LOCKEDNORM blocks
        // GetRootTranslation: reimplemented in Rust as `root_translation`, None without a root node
        // MoveVertex: bound as `Edit::set_vertex` (shim), with validation
        // OffsetShape: reimplemented in Rust as `Edit::offset`, with per-vertex weights
        // ScaleShape: reimplemented in Rust as `Edit::scale`. Upstream scales about the root's
        //   translation without adding it back (shifting the shape by it), gets the masked case
        //   backwards (a vertex masked 0 lands at 2v - target), and leaves normals as they were
        // RotateShape: reimplemented in Rust as `Edit::rotate`, for the same reasons as ScaleShape;
        //   same rotation order (X, then Y, then Z, about fixed axes)
        // GetAlphaProperty: bound as `View::alpha_property` (shim)
        // AssignAlphaProperty: bound as `Edit::assign_alpha_property` (shim), by version-correct placement and keeping shared blocks
        // RemoveAlphaProperty: bound as `Edit::remove_alpha_property` (shim), keeping shared blocks
        // DeleteShape: reimplemented as `Edit::delete` (shim), keeping shared blocks
        // DeleteShader: reimplemented as `Edit::delete_shader` (shim), keeping shared blocks
        // DeleteSkinning: bound as `Edit::delete_skinning` (shim), keeping shared blocks
        // RemoveEmptyPartitions: bound as `Edit::remove_empty_partitions` (shim), with validation, returning
        //   the number removed
        // DeleteVertsForShape: bound as `Edit::delete_vertices` (shim), sorting the indices, with
        //   validation
        // CalcShapeDiff: reimplemented in Rust as `View::vertex_diff`, without the scale (callers
        //   can scale the target) and past 65535 vertices (upstream's uint16_t loop truncates)
        // CalcUVDiff: reimplemented in Rust as `View::uv_diff`, for the same reasons
        // CreateSkinning: bound as `Edit::create_skinning` (shim), with validation
        // SetShapeDynamic: reimplemented as `NiGeometryData::set_consistency`, taking any
        //   `ConsistencyType` on data reached by index rather than shape name
        // UpdateSkinPartitions: bound as `Edit::update_skin_partitions` (shim), with validation
        // UpdatePartitionFlags: bound as `Edit::update_partition_flags` (shim), with validation
    }

    #[namespace = "nifly_rs"]
    unsafe extern "C++" {        
        include!("sort_state.h");
        include!("nif_file_shim.h");

        /// `NifFile::Load(std::istream&)` through a `RustIStream`. Reader errors come back as `Err`.
        fn load_nif(reader: &mut RustReader<'_>, options: LoadOptions) -> Result<LoadResult>;

        /// A deep copy, through `NifFile`'s copy constructor (`CopyFrom`).
        fn clone_nif(file: &NifFile) -> UniquePtr<NifFile>;

        /// New file with a root `NiNode` named "Scene Root", via `NifFile::Create`.
        fn create_nif(version: NifVersion) -> UniquePtr<NifFile>;

        /// The header's file version, user version and stream.
        fn nif_version(file: &NifFile) -> NifVersion;

        /// `NifFile::OptimizeFor`.
        fn optimize_for(file: Pin<&mut NifFile>, options: OptimizeOptions) -> OptimizeResult;

        /// `NifFile::Save(std::ostream&)` through a `RustOStream`.
        /// Returns nifly's code (0 = success). Writer errors come back as `Err`. Undoes
        /// Save's in-memory change to Fallout 76 and Starfield lighting shader types (see
        /// `LightingTypeRestore` in `nif_file_shim.h`).
        fn save_nif(
            nif: Pin<&mut NifFile>,
            writer: &mut RustWriter<'_>,
            options: SaveOptions,
        ) -> Result<i32>;

        /// `NifFile::AddNode`. `parent_index` = `u32::MAX` (`NIF_NPOS`) adds under the root.
        /// Returns the new node's block index, or `u32::MAX` if the parent isn't a node.
        fn add_node(
            file: Pin<&mut NifFile>,
            name: &[u8],
            transform: Transform,
            parent_index: u32,
        ) -> u32;

        /// Like `add_node`, creating the node subclass in the name.
        fn add_bs_fade_node(file: Pin<&mut NifFile>, name: &[u8], transform: Transform, parent_index: u32) -> u32;
        fn add_root_collision_node(file: Pin<&mut NifFile>, name: &[u8], transform: Transform, parent_index: u32) -> u32;
        fn add_avoid_node(file: Pin<&mut NifFile>, name: &[u8], transform: Transform, parent_index: u32) -> u32;
        fn add_ni_bs_animation_node(file: Pin<&mut NifFile>, name: &[u8], transform: Transform, parent_index: u32) -> u32;
        fn add_ni_bs_particle_node(file: Pin<&mut NifFile>, name: &[u8], transform: Transform, parent_index: u32) -> u32;
        fn add_bs_value_node(file: Pin<&mut NifFile>, name: &[u8], transform: Transform, parent_index: u32) -> u32;
        fn add_bs_leaf_anim_node(file: Pin<&mut NifFile>, name: &[u8], transform: Transform, parent_index: u32) -> u32;
        fn add_bs_tree_node(file: Pin<&mut NifFile>, name: &[u8], transform: Transform, parent_index: u32) -> u32;
        fn add_bs_ordered_node(file: Pin<&mut NifFile>, name: &[u8], transform: Transform, parent_index: u32) -> u32;
        fn add_bs_multi_bound_node(file: Pin<&mut NifFile>, name: &[u8], transform: Transform, parent_index: u32) -> u32;
        fn add_bs_weak_reference_node(file: Pin<&mut NifFile>, name: &[u8], transform: Transform, parent_index: u32) -> u32;
        fn add_bs_distant_object_instanced_node(file: Pin<&mut NifFile>, name: &[u8], transform: Transform, parent_index: u32) -> u32;
        fn add_bs_range_node(file: Pin<&mut NifFile>, name: &[u8], transform: Transform, parent_index: u32) -> u32;
        fn add_bs_debris_node(file: Pin<&mut NifFile>, name: &[u8], transform: Transform, parent_index: u32) -> u32;
        fn add_bs_blast_node(file: Pin<&mut NifFile>, name: &[u8], transform: Transform, parent_index: u32) -> u32;
        fn add_bs_damage_stage(file: Pin<&mut NifFile>, name: &[u8], transform: Transform, parent_index: u32) -> u32;
        fn add_bs_face_gen_ni_node(file: Pin<&mut NifFile>, name: &[u8], transform: Transform, parent_index: u32) -> u32;
        fn add_ni_billboard_node(file: Pin<&mut NifFile>, name: &[u8], transform: Transform, parent_index: u32) -> u32;
        fn add_ni_switch_node(file: Pin<&mut NifFile>, name: &[u8], transform: Transform, parent_index: u32) -> u32;
        fn add_ni_lod_node(file: Pin<&mut NifFile>, name: &[u8], transform: Transform, parent_index: u32) -> u32;
        fn add_ni_bone(file: Pin<&mut NifFile>, name: &[u8], transform: Transform, parent_index: u32) -> u32;
        fn add_ni_sort_adjust_node(file: Pin<&mut NifFile>, name: &[u8], transform: Transform, parent_index: u32) -> u32;
        fn add_bs_master_particle_system(file: Pin<&mut NifFile>, name: &[u8], transform: Transform, parent_index: u32) -> u32;

        /// `NifFile::GetBlockID`. `u32::MAX` (`NIF_NPOS`) if `obj` isn't in `file`. Linear search.
        fn block_index(file: &NifFile, obj: &NiObject) -> u32;

        /// `NifFile::DeleteNode`. Returns whether a node named `name` existed.
        fn delete_node(file: Pin<&mut NifFile>, name: &[u8]) -> bool;

        /// `NifFile::CanDeleteNode(name)`. False if no node has that name.
        fn can_delete_node_by_name(file: &NifFile, name: &[u8]) -> bool;


        /// `NifFile::SetShapeOrder` with the names of the shapes at block indices `shapes`.
        /// Silently does nothing in the cases upstream does; the safe crate checks first.
        fn set_shape_order(file: Pin<&mut NifFile>, shapes: &[u32]);

        /// `NifFile::SortState`, which C++ can't name from here because it's nested.
        type SortState;

        /// A sort state for `file` with no blocks visited yet.
        fn new_sort_state(file: &NifFile) -> UniquePtr<SortState>;
        /// `NifFile::SetSortIndices(index, state)`. Returns false, doing nothing, if there's no
        /// block at `index` or it was already visited.
        fn sort_visit(file: Pin<&mut NifFile>, state: Pin<&mut SortState>, index: u32) -> bool;
        fn sort_is_visited(state: &SortState, index: u32) -> bool;
        /// Gives unvisited blocks the remaining indices in their current order and calls
        /// `NiHeader::SetBlockOrder`, like the end of `PrettySortBlocks`.
        fn sort_apply(file: Pin<&mut NifFile>, state: Pin<&mut SortState>);

        /// `NifFile::DeleteUnreferencedBlocks<T>`, with `matches` deciding which blocks are a
        /// `T`. Returns the number of blocks deleted.
        fn delete_unreferenced_blocks(file: Pin<&mut NifFile>, matches: fn(&NiObject) -> bool) -> u32;

        /// `NifFile::DeleteUnreferencedNodes`. Returns the number of nodes deleted, or -1 if
        /// nifly refused (unknown blocks or no root node).
        fn delete_unreferenced_nodes(file: Pin<&mut NifFile>) -> i32;

        /// `NiHeader::GetBlockRefCount`: how many refs (and, with `include_ptrs`, pointers) in
        /// all blocks point to block `index`.
        fn block_ref_count(file: &NifFile, index: u32, include_ptrs: bool) -> i32;

        /// `NifFile::GetShader`. Null if the shape has no shader.
        fn shader(file: &NifFile, shape: &NiShape) -> *const NiShader;

        /// `NifFile::GetAlphaProperty`. Null if the shape has none.
        fn alpha_property(file: &NifFile, shape: &NiShape) -> *const NiAlphaProperty;

        /// `NifFile::AssignAlphaProperty` with a new property, on the shape at block
        /// `shape_index`. Placed in the ref or the property list by what the version saves, and
        /// shared old properties are kept. Returns the new block's index, or `u32::MAX` if that
        /// block isn't a shape.
        fn assign_alpha_property(file: Pin<&mut NifFile>, shape_index: u32, flags: u16, threshold: u8) -> u32;
        /// `NifFile::RemoveAlphaProperty` on the shape at block `shape_index`, deleting only
        /// properties no other block refers to. Returns whether it had one.
        fn remove_alpha_property(file: Pin<&mut NifFile>, shape_index: u32) -> bool;

        /// Gives the `BSMultiBoundNode` at `node` a new `BSMultiBound` with this data, replacing
        /// its ref. Returns the `BSMultiBound`'s index, or `u32::MAX` if that block isn't a
        /// `BSMultiBoundNode`. `rotation` has 3 rows.
        fn assign_multi_bound_aabb(file: Pin<&mut NifFile>, node: u32, center: Vec3, half_extent: Vec3) -> u32;
        fn assign_multi_bound_obb(
            file: Pin<&mut NifFile>,
            node: u32,
            center: Vec3,
            size: Vec3,
            rotation: &[Vec3],
        ) -> u32;
        fn assign_multi_bound_sphere(file: Pin<&mut NifFile>, node: u32, center: Vec3, radius: f32) -> u32;
        /// Clears the multi-bound ref of the `BSMultiBoundNode` at `node`. Returns false if that
        /// block isn't one or has no multi-bound.
        fn remove_multi_bound(file: Pin<&mut NifFile>, node: u32) -> bool;
        /// Gives the `NiLODNode` at `node` a new default LOD data block, replacing its ref.
        /// Returns the new block's index, or `u32::MAX` if that block isn't an `NiLODNode`.
        fn assign_range_lod_data(file: Pin<&mut NifFile>, node: u32) -> u32;
        fn assign_screen_lod_data(file: Pin<&mut NifFile>, node: u32) -> u32;
        /// Clears the LOD data ref of the `NiLODNode` at `node`. Returns false if that block isn't
        /// one or has no LOD data.
        fn remove_lod_data(file: Pin<&mut NifFile>, node: u32) -> bool;
        /// Gives the `NiGeometryData` at `data` a new `BSPackedAdditionalGeometryData` if
        /// `packed`, otherwise a new `NiAdditionalGeometryData`, with the data's vertex count.
        /// Returns its index, or `NIF_NPOS` (changing nothing) if that block isn't geometry data.
        fn assign_additional_geometry_data(file: Pin<&mut NifFile>, data: u32, packed: bool) -> u32;
        /// Clears the additional data ref of the `NiGeometryData` at `data`. False if that block
        /// isn't one or has no additional data.
        fn remove_additional_geometry_data(file: Pin<&mut NifFile>, data: u32) -> bool;

        /// Adds a new default property of that type to the property list of the `NiAVObject` at
        /// block `target`. Returns its index, or `u32::MAX`, changing nothing, if that block isn't
        /// an `NiAVObject`.
        fn add_material_property(file: Pin<&mut NifFile>, target: u32) -> u32;
        fn add_stencil_property(file: Pin<&mut NifFile>, target: u32) -> u32;
        fn add_texturing_property(file: Pin<&mut NifFile>, target: u32) -> u32;
        fn add_specular_property(file: Pin<&mut NifFile>, target: u32) -> u32;
        fn add_vertex_color_property(file: Pin<&mut NifFile>, target: u32) -> u32;
        fn add_dither_property(file: Pin<&mut NifFile>, target: u32) -> u32;
        fn add_fog_property(file: Pin<&mut NifFile>, target: u32) -> u32;
        fn add_wireframe_property(file: Pin<&mut NifFile>, target: u32) -> u32;
        fn add_z_buffer_property(file: Pin<&mut NifFile>, target: u32) -> u32;
        fn add_bs_shader_pp_lighting_property(file: Pin<&mut NifFile>, target: u32) -> u32;
        fn add_lighting30_shader_property(file: Pin<&mut NifFile>, target: u32) -> u32;
        fn add_bs_shader_no_lighting_property(file: Pin<&mut NifFile>, target: u32) -> u32;
        fn add_sky_shader_property(file: Pin<&mut NifFile>, target: u32) -> u32;
        fn add_tile_shader_property(file: Pin<&mut NifFile>, target: u32) -> u32;
        fn add_tall_grass_shader_property(file: Pin<&mut NifFile>, target: u32) -> u32;
        fn add_water_shader_property(file: Pin<&mut NifFile>, target: u32) -> u32;
        fn add_hair_shader_property(file: Pin<&mut NifFile>, target: u32) -> u32;
        fn add_distant_lod_shader_property(file: Pin<&mut NifFile>, target: u32) -> u32;
        fn add_bs_distant_tree_shader_property(file: Pin<&mut NifFile>, target: u32) -> u32;
        fn add_volumetric_fog_shader_property(file: Pin<&mut NifFile>, target: u32) -> u32;
        fn add_bs_water_shader_property(file: Pin<&mut NifFile>, target: u32) -> u32;
        fn add_bs_sky_shader_property(file: Pin<&mut NifFile>, target: u32) -> u32;

        /// `NifFile::GetStencilProperty`. Null if the shape's property list has none.
        fn stencil_property(file: &NifFile, shape: &NiShape) -> *const NiStencilProperty;

        /// The `Get*ForShape` copy getters. Each returns false, leaving `out` empty, if the shape
        /// doesn't store that data.
        fn shape_vertices(file: &NifFile, shape: &NiShape, out: &mut Vec<Vec3>) -> bool;
        fn shape_normals(file: &NifFile, shape: &NiShape, out: &mut Vec<Vec3>) -> bool;
        fn shape_uvs(file: &NifFile, shape: &NiShape, out: &mut Vec<Uv>) -> bool;
        fn shape_colors(file: &NifFile, shape: &NiShape, out: &mut Vec<Color>) -> bool;
        fn shape_tangents(file: &NifFile, shape: &NiShape, out: &mut Vec<Vec3>) -> bool;
        fn shape_bitangents(file: &NifFile, shape: &NiShape, out: &mut Vec<Vec3>) -> bool;
        fn shape_eye_data(file: &NifFile, shape: &NiShape, out: &mut Vec<f32>) -> bool;

        /// The `Set*ForShape` setters on the shape at block `shape`. Each returns false, changing
        /// nothing, unless the shape stores vertex data (geometry data or a `BSTriShape`'s own;
        /// for eye data, only a `BSTriShape`) and `values` has one value per vertex.
        fn set_shape_vertices(file: Pin<&mut NifFile>, shape: u32, values: &[Vec3]) -> bool;
        fn set_shape_normals(file: Pin<&mut NifFile>, shape: u32, values: &[Vec3]) -> bool;
        fn set_shape_uvs(file: Pin<&mut NifFile>, shape: u32, values: &[Uv]) -> bool;
        fn set_shape_colors(file: Pin<&mut NifFile>, shape: u32, values: &[Color]) -> bool;
        fn set_shape_tangents(file: Pin<&mut NifFile>, shape: u32, values: &[Vec3]) -> bool;
        fn set_shape_bitangents(file: Pin<&mut NifFile>, shape: u32, values: &[Vec3]) -> bool;
        fn set_shape_eye_data(file: Pin<&mut NifFile>, shape: u32, values: &[f32]) -> bool;

        /// `NiShape::GetTriangles`. Returns false, leaving `out` empty, if the shape stores none.
        /// Not an upstream `NifFile` method.
        fn shape_triangles(shape: &NiShape, out: &mut Vec<Triangle>) -> bool;
        /// `NiShape::SetTriangles` on the shape at block `shape`. Returns false, changing nothing,
        /// unless the shape stores settable triangles (`NiTriShapeData`, a `BSTriShape`, or a
        /// `BSGeometry` mesh), every index is below its vertex count, and the count fits.
        /// Not an upstream `NifFile` method.
        fn set_shape_triangles(file: Pin<&mut NifFile>, shape: u32, triangles: &[Triangle]) -> bool;

        /// `NifFile::CalcNormalsForShape` on the shape at block `shape`. Returns whether normals
        /// were calculated (see the shim).
        fn calc_shape_normals(
            file: Pin<&mut NifFile>,
            shape: u32,
            smooth: bool,
            smooth_threshold: f32,
            force: bool,
        ) -> bool;

        /// `NifFile::CalcTangentsForShape` on the shape at block `shape`. Returns whether tangents
        /// were calculated (see the shim).
        fn calc_shape_tangents(file: Pin<&mut NifFile>, shape: u32) -> bool;

        /// `NifFile::SetShapeVertWeights` on the shape at block `shape`, with `bones[i]` (positions
        /// in its bone list) weighted by `weights[i]`. Returns false, changing nothing, if the
        /// weights can't be stored (see the shim).
        fn set_shape_vertex_weights(
            file: Pin<&mut NifFile>,
            shape: u32,
            vertex: u32,
            bones: &[u32],
            weights: &[f32],
        ) -> bool;

        /// `NifFile::ClearShapeVertWeights` on the shape at block `shape`. Returns false, changing
        /// nothing, unless it stores weights per vertex (see the shim).
        fn clear_shape_vertex_weights(file: Pin<&mut NifFile>, shape: u32) -> bool;

        /// `NifFile::CreateSkinning` on the shape at block `shape`. Returns false, changing
        /// nothing, unless it's a shape type that can be skinned and has no skin instance.
        fn create_skinning(file: Pin<&mut NifFile>, shape: u32) -> bool;
        /// `NifFile::DeleteSkinning` on the shape at block `shape`, deleting only unshared
        /// blocks. Returns false, changing nothing, if it has no skin instance.
        fn delete_skinning(file: Pin<&mut NifFile>, shape: u32) -> bool;

        /// `NifFile::SetShapeBoneIDList` on the shape at block `shape`. Returns false, changing
        /// nothing, unless it has a skin instance, isn't a `BSGeometry`, and every id is a node.
        fn set_shape_bone_ids(file: Pin<&mut NifFile>, shape: u32, ids: &[u32]) -> bool;
        /// `NifFile::SetShapeBoneList` on the `BSGeometry` at block `shape`, with the names
        /// concatenated in `names` and their sizes in `lengths`. Returns false, changing nothing,
        /// unless it's a `BSGeometry` with a `BSSkin::Instance`.
        fn set_shape_bone_names(
            file: Pin<&mut NifFile>,
            shape: u32,
            names: &[u8],
            lengths: &[usize],
        ) -> bool;

        /// `NifFile::UpdateShapeBoneID` on the shape at block `shape`. Returns false, changing
        /// nothing, unless it's skinned, `new_id` is a node and a bone is `old_id`.
        fn replace_shape_bone(file: Pin<&mut NifFile>, shape: u32, old_id: u32, new_id: u32) -> bool;

        /// `NifFile::RemoveEmptyPartitions` on the shape at block `shape`. The number of partitions
        /// removed, or -1, changing nothing, if it can't run (see the shim).
        fn remove_empty_partitions(file: Pin<&mut NifFile>, shape: u32) -> i64;
        /// `NifFile::DeletePartitions` on the shape at block `shape`. Returns false, changing
        /// nothing, unless every index is a partition (see the shim).
        fn delete_partitions(file: Pin<&mut NifFile>, shape: u32, indices: &[u32]) -> bool;
        /// `NifFile::LoadExternalShapeData` for mesh `index` of the `BSGeometry` at block `shape`:
        /// 0 on success, 1 if there's no such external mesh, 2 if the data ended early, 3 for an
        /// unsupported mesh version. The mesh is only changed on success.
        fn load_external_mesh(
            file: Pin<&mut NifFile>,
            shape: u32,
            index: u32,
            reader: &mut RustReader<'_>,
        ) -> Result<i32>;
        /// `NifFile::SaveExternalShapeData` for mesh `index` of `shape`, into `out`. False,
        /// writing nothing, unless it's a `BSGeometry` with that external mesh.
        fn save_external_mesh(shape: &NiShape, index: u32, out: &mut Vec<u8>) -> Result<bool>;
        /// How many external meshes (LODs) a `BSGeometry` has; 0 for other shapes or internal data.
        fn external_mesh_count(shape: &NiShape) -> u32;
        /// The path of external mesh `index`; empty if out of range.
        fn external_mesh_path(shape: &NiShape, index: u32) -> &[u8];
        /// Sets the path of external mesh `index` of the `BSGeometry` at block `shape`. Returns
        /// false if there's no such mesh.
        fn set_external_mesh_path(file: Pin<&mut NifFile>, shape: u32, index: u32, path: &[u8]) -> bool;
        /// `NifFile::TriangulateShape` on the shape at block `shape`. Returns false, changing
        /// nothing, unless it's an `NiTriStrips` with triangles and unshared data.
        fn triangulate_shape(file: Pin<&mut NifFile>, shape: u32) -> bool;
        /// `NifFile::DeleteShape`, keeping blocks other blocks still use. False if not a shape.
        fn delete_shape(file: Pin<&mut NifFile>, shape: u32) -> bool;
        /// `NifFile::DeleteShader`, keeping blocks other blocks still use. False, changing
        /// nothing, if the block isn't a shape or has no shader, material or alpha property.
        fn delete_shader(file: Pin<&mut NifFile>, shape: u32) -> bool;
        /// `NifFile::DeleteVertsForShape` on the shape at block `shape`. Returns false, changing
        /// nothing, unless it stores vertices and every index is one of them.
        fn delete_shape_vertices(file: Pin<&mut NifFile>, shape: u32, vertices: &[u32]) -> bool;
        /// `NifFile::MoveVertex` on the shape at block `shape`. Returns false, changing nothing,
        /// unless it stores vertices and `vertex` is one of them.
        fn set_shape_vertex(file: Pin<&mut NifFile>, shape: u32, vertex: u32, position: Vec3) -> bool;
        /// `NifFile::MirrorShape` on the shape at block `shape`. Returns false, changing nothing,
        /// unless it can be mirrored (see the shim).
        fn mirror_shape(file: Pin<&mut NifFile>, shape: u32, x: bool, y: bool, z: bool) -> bool;
        /// `NifFile::InvertUVsForShape` on the shape at block `shape`. Returns false, changing
        /// nothing, unless it stores UVs.
        fn invert_shape_uvs(file: Pin<&mut NifFile>, shape: u32, u: bool, v: bool) -> bool;
        /// `NifFile::CreateShapeFromData`. The new shape's index, or `u32::MAX`, changing nothing,
        /// if the data doesn't fit (see the shim). Empty `View::uvs` or `View::normals` are left out.
        fn create_shape(
            file: Pin<&mut NifFile>,
            name: &[u8],
            vertices: &[Vec3],
            triangles: &[Triangle],
            uvs: &[Uv],
            normals: &[Vec3],
        ) -> u32;
        /// `NifFile::CloneShape` within the file. The clone's index, or `u32::MAX` if `shape`
        /// isn't a shape.
        fn clone_shape(file: Pin<&mut NifFile>, shape: u32, name: &[u8]) -> u32;
        /// `NifFile::CloneShape` from `source`. `u32::MAX` unless the versions match.
        fn clone_shape_from(file: Pin<&mut NifFile>, source: &NifFile, shape: &NiShape, name: &[u8]) -> u32;
        /// `NifFile::CloneNamedNode` on a given node within the file. `u32::MAX` if not a node.
        fn clone_node(file: Pin<&mut NifFile>, node: u32) -> u32;
        /// The same from `source`. `u32::MAX` unless the versions match.
        fn clone_node_from(file: Pin<&mut NifFile>, source: &NifFile, node: &NiNode) -> u32;
        /// `NifFile::CloneChildren` within the file. False if there's no such block.
        fn clone_children(file: Pin<&mut NifFile>, block: u32) -> bool;
        /// `NifFile::ReorderTriangles` on the shape at block `shape`. Returns false, changing
        /// nothing, unless `order` is a permutation of its triangles (see the shim).
        fn reorder_shape_triangles(file: Pin<&mut NifFile>, shape: u32, order: &[u32]) -> bool;
        /// `NifFile::GetShapeSegments`, flattened (see the shim). False unless the shape is a
        /// `BSSubIndexTriShape`.
        fn shape_segments(
            shape: &NiShape,
            ssf: &mut Vec<u8>,
            segs: &mut Vec<SegmentFfi>,
            subs: &mut Vec<SubsegmentFfi>,
            extra: &mut Vec<f32>,
            tri_parts: &mut Vec<i32>,
        ) -> bool;
        /// `NifFile::SetShapeSegments` on the shape at block `shape`, from the flattened form.
        /// Returns false, changing nothing, if it doesn't fit (see the shim).
        fn set_shape_segments(
            file: Pin<&mut NifFile>,
            shape: u32,
            ssf: &[u8],
            segs: &[SegmentFfi],
            subs: &[SubsegmentFfi],
            extra: &[f32],
            tri_parts: &[i32],
        ) -> bool;
        /// `NifFile::GetShapePartitions`, without changing the partition. Returns false unless the
        /// shape has an `NiSkinInstance` with a partition.
        fn shape_partitions(
            file: &NifFile,
            shape: &NiShape,
            info: &mut Vec<PartitionInfo>,
            tri_parts: &mut Vec<i32>,
        ) -> bool;
        /// `NifFile::SetShapePartitions` then `UpdateSkinPartitions` on the shape at block
        /// `shape`. Returns false, changing nothing, if the input doesn't fit (see the shim).
        fn set_shape_partitions(
            file: Pin<&mut NifFile>,
            shape: u32,
            info: &[PartitionInfo],
            tri_parts: &[i32],
        ) -> bool;

        /// `NifFile::ApplyNormalsFromFile` on the shape at block `shape`, from `source_shape` in
        /// `source`. Returns false, changing nothing, unless the source shape has `LOCKEDNORM`
        /// indices and both shapes have the same number of normals.
        fn apply_locked_normals(
            file: Pin<&mut NifFile>,
            shape: u32,
            source: &NifFile,
            source_shape: &NiShape,
        ) -> bool;

        /// `NifFile::GetGeometryData`. Null unless the shape is an `NiTriBasedGeom` with a data
        /// block or a `BSGeometry` with a mesh.
        fn geometry_data(file: &NifFile, shape: &NiShape) -> *const NiGeometryData;

        /// `NifFile::GetTexturingProperty`. Null if the shape's property list has none.
        fn texturing_property(file: &NifFile, shape: &NiShape) -> *const NiTexturingProperty;

        /// `NifFile::GetMaterialProperty`. Null if the shape's property list has none.
        fn material_property(file: &NifFile, shape: &NiShape) -> *const NiMaterialProperty;

        /// Whether `obj` is one of `file`'s blocks. Linear search.
        fn contains_block(file: &NifFile, obj: &NiObject) -> bool;

        /// `obj` as a mutable pointer, through `file` borrowed mutably.
        ///
        /// # Safety
        ///
        /// `obj` must be a block of `file` or an object inside one.
        unsafe fn object_mut(file: Pin<&mut NifFile>, obj: *const NiObject) -> *mut NiObject;

        fn block_count(file: &NifFile) -> usize;

        /// Null if `index` is out of range.
        fn block(file: &NifFile, index: usize) -> *const NiObject;

        /// Block indices of the `extraDataRefs` list, in order. `u32::MAX` (`NIF_NPOS`) for empty
        /// refs. Older files' linked extra data chain isn't included.
        fn extra_data_indices(obj: &NiObjectNET) -> Vec<u32>;

        /// `NifFile::CalcShapeTransformGlobalToSkin`. Returns false, leaving `out` unchanged, if
        /// the transform is neither stored nor calculable.
        fn calc_shape_transform_global_to_skin(file: &NifFile, shape: &NiShape, out: &mut Transform) -> bool;

        /// `NifFile::GetShapeTransformGlobalToSkin`. Returns false, leaving `out` unchanged, if
        /// the shape has no stored global-to-skin transform.
        fn shape_transform_global_to_skin(file: &NifFile, shape: &NiShape, out: &mut Transform) -> bool;


        /// Sets `targetRef` of the `NiSkinInstance` (to an `NiNode`) or `BSSkinInstance` (to an
        /// `NiAVObject`) at block `skin`. Returns false, changing nothing, if either block is
        /// missing or of the wrong type.
        fn set_skin_root(file: Pin<&mut NifFile>, skin: u32, root: u32) -> bool;

        /// `NifFile::SetShapeTransformSkinToBone` on the shape at block `shape`. Returns false,
        /// changing nothing, if that block isn't a shape, isn't skinned or has no such bone.
        fn set_shape_transform_skin_to_bone(
            file: Pin<&mut NifFile>,
            shape: u32,
            bone_index: u32,
            transform: Transform,
        ) -> bool;

        /// `NifFile::GetShapeBoneBounds`. Returns false, leaving `out` unchanged, if the shape
        /// isn't skinned or has no such bone.
        fn shape_bone_bounds(file: &NifFile, shape: &NiShape, bone_index: u32, out: &mut BoundingSphere) -> bool;

        /// `NifFile::SetShapeBoneBounds` on the shape at block `shape`. Returns false, changing
        /// nothing, if that block isn't a shape, isn't skinned or has no such bone.
        fn set_shape_bone_bounds(
            file: Pin<&mut NifFile>,
            shape: u32,
            bone_index: u32,
            bounds: BoundingSphere,
        ) -> bool;

        /// `NifFile::GetShapeTransformSkinToBone` by bone index. Returns false, leaving `out`
        /// unchanged, if the shape isn't skinned or has no such bone.
        fn shape_transform_skin_to_bone(
            file: &NifFile,
            shape: &NiShape,
            bone_index: u32,
            out: &mut Transform,
        ) -> bool;

        /// `NifFile::GetNodeTransformToGlobal`. Returns false, leaving `out` unchanged, if no node has that name.
        fn node_transform_to_global(file: &NifFile, name: &[u8], out: &mut Transform) -> bool;

        /// `NifFile::SetNodeTransformToParent`. Returns whether a node had that name.
        fn set_node_transform_to_parent(
            file: Pin<&mut NifFile>,
            name: &[u8],
            transform: Transform,
            root_children_only: bool,
        ) -> bool;

        /// `NifFile::GetParentNode` on the block at `child`, as the parent's block index;
        /// `u32::MAX` if there's no such block or it has no parent.
        fn parent_node_index(file: &NifFile, child: u32) -> u32;

        /// `NifFile::SetParentNode` by block index; `u32::MAX` for `parent_index` means the root.
        /// Returns false, changing nothing, if the child isn't a `NiAVObject`, the parent isn't a
        /// node, they're the same block, or the parent is in the child's subtree.
        fn set_parent_node(file: Pin<&mut NifFile>, child_index: u32, parent_index: u32) -> bool;

        /// Replaces `BSTreeNode::bones1` (`list` 1) or `bones2` (`list` 2) of the block at
        /// `node_index`. Returns false, changing nothing, if that block isn't a `BSTreeNode` or a
        /// bone isn't an `NiNode`.
        fn set_tree_node_bones(file: Pin<&mut NifFile>, node_index: u32, list: u8, bones: &[u32]) -> bool;

        /// `NifFile::GetShapeBoneList`. Empty if the shape isn't skinned.
        fn shape_bone_list(file: &NifFile, shape: &NiShape) -> UniquePtr<CxxVector<CxxString>>;

        /// `NifFile::GetShapeBoneIDList`. Empty if the shape isn't skinned.
        /// `NifFile::IsSSECompatible(NiShape*)`.
        fn is_shape_sse_compatible(file: &NifFile, shape: &NiShape) -> bool;

        /// `NifFile::GetTexturePathRefs`, read-only, as a count and a per-index getter.
        fn texture_path_count(file: &NifFile, shape: &NiShape) -> usize;
        /// Empty if `index` is out of range. Borrowed from the file's blocks.
        fn texture_path<'a>(file: &'a NifFile, shape: &NiShape, index: usize) -> &'a [u8];

        /// `NifFile::GetTextureSlot`.
        fn texture_slot(file: &NifFile, shape: &NiShape, index: u32) -> TextureSlot;
        /// `NifFile::SetTextureSlot` on the shape at block `shape_index` (ignored if it isn't one),
        /// with its `NiTexturingProperty` part reimplemented: the texture count only grows, and a
        /// map without a source texture gets a new `NiSourceTexture`.
        fn set_texture_slot(file: Pin<&mut NifFile>, shape_index: u32, path: &[u8], index: u32);
        /// Adds an unattached `NiSourceTexture` for the external file `path`; returns its index.
        fn add_source_texture(file: Pin<&mut NifFile>, path: &[u8]) -> u32;
        /// Sets the source of `map` of the `NiTexturingProperty` at `property`, adding the map, or
        /// removes the map if `source` is `NIF_NPOS`. Returns false, changing nothing, if a block
        /// has the wrong type or `map` is out of range.
        fn set_map_source(file: Pin<&mut NifFile>, property: u32, map: u32, source: u32) -> bool;
        /// Gives the `NiSourceTexture` at `texture` a new default pixel data block, replacing its
        /// ref. Returns the new block's index, or `u32::MAX` if that block isn't one.
        fn assign_ni_pixel_data(file: Pin<&mut NifFile>, texture: u32) -> u32;
        fn assign_persistent_pixel_data(file: Pin<&mut NifFile>, texture: u32) -> u32;
        /// Gives the `TextureRenderData` at `data` a new `NiPalette` with these colors (flattened
        /// r, g, b, a). Returns its index, or `u32::MAX` if that block isn't one or there aren't
        /// 16 or 256 colors.
        fn assign_palette(file: Pin<&mut NifFile>, data: u32, rgba: &[u8], has_alpha: bool) -> u32;

        fn shape_bone_ids(file: &NifFile, shape: &NiShape) -> Vec<u32>;

        /// `NifFile::GetShapeBoneWeights`, sorted by vertex. `bone_index` is the position in the
        /// shape's bone list. Empty if the shape isn't skinned or the index is out of range.
        fn shape_bone_weights(file: &NifFile, shape: &NiShape, bone_index: u32) -> Vec<VertexWeight>;

        /// `NifFile::SetShapeBoneWeights` on the shape at block `shape`, with `weights` sorted by
        /// vertex and free of duplicates. Returns false, changing nothing, if that block isn't a
        /// shape whose weights live in `NiSkinData`, the bone doesn't exist, or a vertex index is
        /// out of range.
        fn set_shape_bone_weights(
            file: Pin<&mut NifFile>,
            shape: u32,
            bone_index: u32,
            weights: &[VertexWeight],
        ) -> bool;

        /// `NifFile::UpdateSkinPartitions` on the shape at block `shape`. Returns false, changing
        /// nothing, if it would be skipped or read out of bounds (see the shim).
        fn update_skin_partitions(file: Pin<&mut NifFile>, shape: u32) -> bool;

        /// `NifFile::UpdatePartitionFlags` on the shape at block `shape`. Returns false, changing
        /// nothing, unless it has a `BSDismemberSkinInstance` and a skin partition with as many
        /// partitions.
        fn update_partition_flags(file: Pin<&mut NifFile>, shape: u32) -> bool;
    }
}
