//! Handles pairing a block with its file.
//!
//! A block borrowed from a [`NifFile`] can't reach the file, so it can't follow its refs to other
//! blocks or check a block it's given. A [`View`] or [`Edit`] holds the file with the block's
//! index, and has the methods that need both: for example, `View<BsTreeNode>` lists a tree's
//! bones, and `Edit<BsTreeNode>` replaces them.
//!
//! Methods by block type. Those for a base type also work on its subtypes (see
//! [`IsA`]): a `View<BsFadeNode>` has `children`.
//!
//! | Type | [`View`] | [`Edit`] |
//! |---|---|---|
//! | [`NiObjectNet`](crate::objects::NiObjectNet) and subtypes | `extra_data`, `extra_data_chain`, `controller` | `assign_extra_data`, `set_extra_data`, `set_extra_data_chain`, `controller`, `set_controller` |
//! | [`NiAvObject`](crate::objects::NiAvObject) and subtypes | `parent`, `properties`, `collision` | `parent`, `set_parent`, `add_property`, `set_properties`, `collision`, `set_collision`, `assign_collision` |
//! | [`NiNode`](crate::nodes::NiNode) and subtypes | `children`, `children_of`, `effects` | `set_children`, `add_child`, `add_child_of`, `clone_node`, `set_effects`, `add_effect` |
//! | [`NiGeometryData`](crate::geometry::NiGeometryData) and subtypes | `additional_data` | `additional_data`, `assign_additional_data`, `set_additional_data`, `remove_additional_data`, `set_uv_sets`, `set_data_raw_flags` |
//! | [`NiExtraData`](crate::extra_data::NiExtraData) | `next_extra_data` | `next_extra_data`, `set_next_extra_data` |
//! | [`NiDynamicEffect`](crate::effects::NiDynamicEffect) and subtypes | `affected_nodes` | `set_affected_nodes` |
//! | [`NiTextureEffect`](crate::effects::NiTextureEffect) | `source_texture` | `source_texture`, `set_source_texture` |
//! | [`NiShape`](crate::geometry::NiShape) (not subtypes; `upcast` first) | vertex arrays (`vertices`, `normals`, `uvs`, ...), `triangles`, `segments`, `geometry_data`, skinning (`bone_list`, `bone_weights`, `partitions`, transforms), `skin_instance`, `shader` and properties, `texture_paths`, `texture_slot` | their setters (`set_vertices`, ...), `set_geometry_data`, `set_skin_instance`, `set_shader`, `set_alpha_property`, `delete`, `clone_shape`, `calc_normals`, `offset`/`scale`/`rotate`/`mirror`, skinning and partition edits, `assign_alpha_property`, `set_texture_slot` |
//! | [`BsTriShape`](crate::geometry::BsTriShape) and subtypes | | `set_vertex_flags`, `set_vertex_extra_floats` |
//! | [`BsTreeNode`](crate::nodes::BsTreeNode) | `bones_1`, `bones_2` | `set_bones_1`, `set_bones_2` |
//! | [`BsMasterParticleSystem`](crate::nodes::BsMasterParticleSystem) | `particle_systems` | `set_particle_systems` |
//! | [`BsMultiBoundNode`](crate::nodes::BsMultiBoundNode) | `multi_bound` | `multi_bound`, `assign_multi_bound`, `set_multi_bound`, `remove_multi_bound` |
//! | [`BsMultiBound`](crate::nodes::BsMultiBound) | `data` | `data`, `set_data` |
//! | [`NiLodNode`](crate::nodes::NiLodNode) | `lod_data` | `lod_data`, `assign_lod_data`, `set_lod_data`, `remove_lod_data` |
//! | [`NiCamera`](crate::nodes::NiCamera) | `scene` | `scene`, `set_scene` |
//! | [`NiDefaultAvObjectPalette`](crate::nodes::NiDefaultAvObjectPalette) | `scene`, `objects`, `object` | `scene`, `set_scene`, `set_objects` |
//! | [`NiBoneContainer`](crate::skin::NiBoneContainer) | `root`, `bones` | `root`, `set_root`, `set_bones` |
//! | [`NiSkinInstance`](crate::skin::NiSkinInstance) | `data`, `partition` | `data`, `partition`, `set_data`, `set_partition` |
//! | [`BsSkinInstance`](crate::skin::BsSkinInstance) | `data` | `data`, `set_data` |
//! | [`NiSkinData`](crate::skin::NiSkinData) | `partition` | `partition`, `set_partition` |
//! | [`NiCollisionObject`](crate::collision::NiCollisionObject) and subtypes | `target` | `target`, `set_target` |
//! | [`BhkNiCollisionObject`](crate::collision::BhkNiCollisionObject) and subtypes | `body` | `body`, `set_body`, `assign_body` |
//! | [`BhkWorldObject`](crate::collision::BhkWorldObject), [`BhkTransformShape`](crate::collision::BhkTransformShape), [`BhkMoppBvTreeShape`](crate::collision::BhkMoppBvTreeShape) and subtypes ([`HasShape`](crate::collision::HasShape)) | `shape` | `shape`, `set_shape`, `assign_shape` |
//! | [`BhkListShape`](crate::collision::BhkListShape), [`BhkConvexListShape`](crate::collision::BhkConvexListShape) ([`HasSubShapes`](crate::collision::HasSubShapes)) | `sub_shapes` | `set_sub_shapes`, `add_sub_shape` |
//! | [`BhkNiTriStripsShape`](crate::collision::BhkNiTriStripsShape) | `strips_data` | `set_strips_data` |
//! | [`BhkPackedNiTriStripsShape`](crate::collision::BhkPackedNiTriStripsShape) | `packed_data` | `packed_data`, `set_packed_data`, `assign_packed_data` |
//! | [`BhkCompressedMeshShape`](crate::collision::BhkCompressedMeshShape) | `mesh_target`, `compressed_data` | `mesh_target`, `set_mesh_target`, `compressed_data`, `set_compressed_data`, `assign_compressed_data` |
//! | [`BhkRigidBody`](crate::collision::BhkRigidBody) and subtypes | `constraints` | `set_constraints`, `add_constraint` |
//! | [`BhkConstraint`](crate::collision::BhkConstraint) and subtypes, [`BhkBallSocketConstraintChain`](crate::collision::BhkBallSocketConstraintChain) ([`HasEntities`](crate::collision::HasEntities)) | `entities` | `set_entities` |
//! | [`BhkBreakableConstraint`](crate::collision::BhkBreakableConstraint), [`BhkMalleableConstraint`](crate::collision::BhkMalleableConstraint) ([`WrapsConstraint`](crate::collision::WrapsConstraint)) | `wrapped_entities` | `set_wrapped_entities` |
//! | [`BhkBallSocketConstraintChain`](crate::collision::BhkBallSocketConstraintChain) | `chained_entities` | `set_chained_entities` |
//! | [`BhkRagdollTemplate`](crate::collision::BhkRagdollTemplate) | `ragdoll_bones` | `set_ragdoll_bones` |
//! | [`BhkOrientHingedBodyAction`](crate::collision::BhkOrientHingedBodyAction) | `hinged_body` | `hinged_body`, `set_hinged_body` |
//! | [`NiShader`](crate::shaders::NiShader) | `texture_set` | `texture_set`, `set_texture_set` |
//! | [`NiTexturingProperty`](crate::shaders::NiTexturingProperty) | `map_source` | `map_source`, `set_map_source` |
//! | [`NiSourceTexture`](crate::shaders::NiSourceTexture) | `pixel_data` | `pixel_data`, `assign_pixel_data`, `set_pixel_data` |
//! | [`TextureRenderData`](crate::shaders::TextureRenderData) | `palette` | `palette`, `assign_palette`, `set_palette` |
//!
//! ```
//! # use nifly::{file::NifFile, nodes::NiNode};
//! # let nif = NifFile::load(std::fs::File::open("assets/Cube.nif").unwrap(), None).unwrap();
//! let root = nif.view::<NiNode>(0).unwrap();
//! println!("{} at index {}", root.name(), root.index());
//! ```

use std::{fmt, marker::PhantomData, ops::Deref, pin::Pin};

use nifly_sys::nif_file::ffi::{self as nif_ffi, RefKind, RefList};

use crate::{
    file::{Index, NifFile, block_index_to_ffi, index_from_ffi, optional_index_to_ffi},
    iter::{BlockIter, RefIndices},
    objects::{IsA, NiType},
};

/// A block of type `T` with the file it belongs to, from [`NifFile::view`] or
/// [`BlockIter::views`](crate::iter::BlockIter::views). Derefs to the block.
pub struct View<'a, T: NiType> {
    file: &'a NifFile,
    index: Index,
    block: &'a T,
}

impl<T: NiType> Clone for View<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: NiType> Copy for View<'_, T> {}

impl<'a, T: NiType> View<'a, T> {
    pub(crate) fn new(file: &'a NifFile, index: Index) -> Option<Self> {
        Some(Self { file, index, block: file.block_of::<T>(index)? })
    }

    pub(crate) fn from_parts(file: &'a NifFile, index: Index, block: &'a T) -> Self {
        Self { file, index, block }
    }

    /// The block's index in the file.
    pub fn index(&self) -> Index {
        self.index
    }

    pub fn file(&self) -> &'a NifFile {
        self.file
    }

    /// The block, borrowed for as long as the file.
    pub fn get(&self) -> &'a T {
        self.block
    }

    /// The same block as a `U`, or `None` if it isn't one. Works for base and derived types.
    pub fn cast<U: NiType>(&self) -> Option<View<'a, U>> {
        View::new(self.file, self.index)
    }

    /// The same block as its base type `B`.
    pub fn upcast<B: NiType, I>(&self) -> View<'a, B>
    where
        T: IsA<B, I>,
    {
        View { file: self.file, index: self.index, block: self.block.upcast_ref() }
    }

    /// The block at `index` in the same file, if it's a `U`.
    pub(crate) fn resolve<U: NiType>(&self, index: Option<Index>) -> Option<View<'a, U>> {
        View::new(self.file, index?)
    }

    /// The block in this block's `kind` ref, if it's a `U`.
    pub(crate) fn follow<U: NiType>(&self, kind: RefKind) -> Option<View<'a, U>> {
        self.resolve(index_from_ffi(nif_ffi::block_ref(&self.file.inner, self.index as u32, kind)))
    }

    /// The blocks in this block's `list` refs that are a `U`.
    pub(crate) fn follow_list<U: NiType>(&self, list: RefList) -> BlockIter<'a, U, RefIndices> {
        BlockIter::new(self.file, RefIndices::new(nif_ffi::block_ref_list(&self.file.inner, self.index as u32, list)))
    }
}

impl<T: NiType> Deref for View<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.block
    }
}

impl<T: NiType> fmt::Debug for View<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("View")
            .field("index", &self.index)
            .field("block", &self.block.as_object().block_name())
            .finish()
    }
}

/// A block of type `T` in a file borrowed mutably, from [`NifFile::edit`]. Derefs to the block.
/// Methods that change other blocks, or check blocks they're given, go here; the block's own
/// setters are on [`block`](Self::block), and the [`View`] methods on [`view`](Self::view).
pub struct Edit<'a, T: NiType> {
    file: &'a mut NifFile,
    index: Index,
    _type: PhantomData<fn() -> T>,
}

impl<'a, T: NiType> Edit<'a, T> {
    pub(crate) fn new(file: &'a mut NifFile, index: Index) -> Option<Self> {
        file.block_of::<T>(index)?;
        Some(Self { file, index, _type: PhantomData })
    }

    /// The block's index in the file.
    pub fn index(&self) -> Index {
        self.index
    }

    /// The block, for reading.
    pub fn view(&self) -> View<'_, T> {
        View::new(self.file, self.index).expect("the block was checked when the edit was made")
    }

    /// The block, for its own setters.
    pub fn block(&mut self) -> Pin<&mut T> {
        self.file.block_of_mut::<T>(self.index).expect("the block was checked when the edit was made")
    }

    /// The same block as a `U`, or the edit back if it isn't one. Works for base and derived
    /// types.
    pub fn into_cast<U: NiType>(self) -> Result<Edit<'a, U>, Self> {
        if self.file.block_of::<U>(self.index).is_some() {
            Ok(Edit { file: self.file, index: self.index, _type: PhantomData })
        } else {
            Err(self)
        }
    }

    /// The same block as its base type `B`.
    pub fn into_upcast<B: NiType, I>(self) -> Edit<'a, B>
    where
        T: IsA<B, I>,
    {
        Edit { file: self.file, index: self.index, _type: PhantomData }
    }

    pub(crate) fn file_mut(&mut self) -> &mut NifFile {
        self.file
    }

    /// The block at `index` in the same file, if it's a `U`.
    pub(crate) fn resolve<U: NiType>(&mut self, index: Option<Index>) -> Option<Edit<'_, U>> {
        Edit::new(self.file, index?)
    }

    /// The block in this block's `kind` ref, if it's a `U`, for editing.
    pub(crate) fn follow<U: NiType>(&mut self, kind: RefKind) -> Option<Edit<'_, U>> {
        let index = index_from_ffi(nif_ffi::block_ref(&self.file.inner, self.index as u32, kind));
        self.resolve(index)
    }

    /// Points this block's `kind` ref at `target`, or empties it. `false` if the shim rejects it.
    pub(crate) fn set_ref(&mut self, kind: RefKind, target: Option<Index>) -> bool {
        let Some(target) = optional_index_to_ffi(target) else {
            return false;
        };
        nif_ffi::set_block_ref(self.file.inner.pin_mut(), self.index as u32, kind, target)
    }

    /// Replaces this block's `list` refs. `false` if the shim rejects them.
    pub(crate) fn set_ref_list(&mut self, list: RefList, targets: &[Index]) -> bool {
        let Some(targets) = targets.iter().map(|&i| block_index_to_ffi(i)).collect::<Option<Vec<_>>>() else {
            return false;
        };
        nif_ffi::set_block_ref_list(self.file.inner.pin_mut(), self.index as u32, list, &targets)
    }
}

impl<T: NiType> Deref for Edit<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.file.block_of::<T>(self.index).expect("the block was checked when the edit was made")
    }
}

impl<T: NiType> fmt::Debug for Edit<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Edit").field("index", &self.index).field("block", &self.as_object().block_name()).finish()
    }
}
