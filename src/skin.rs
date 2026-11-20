//! Skinning blocks: skin instances, skin data and partitions.
//!
//! # Type hierarchy
//!
//! <!-- type-hierarchy:start -->
//!
//! - [`objects::NiObject`](crate::objects::NiObject)
//!   - [`NiSkinData`]
//!   - [`NiSkinPartition`]
//!   - [`NiBoneContainer`]
//!     - [`NiSkinInstance`]
//!       - [`BsDismemberSkinInstance`]
//!     - [`BsSkinInstance`]
//!   - [`BsSkinBoneData`]
//!
//! <!-- type-hierarchy:end -->

use std::collections::BTreeMap;

use bitflags::bitflags;
use bstr::BString;
use nifly_sys::{
    nif_file::ffi as nif_ffi,
    nif_file_refs::ffi::{RefKind, RefList},
    skin::ffi,
};

use crate::{
    file::{block_index_to_ffi, index_from_ffi, Index},
    geometry::{BsGeometry, NiShape, VertexWeight},
    object3d::BoundingSphere,
};

use crate::{
    handle::{Edit, View},
    iter::{BlockIter, RefIndices},
    nodes::NiNode,
    objects::{NiAvObject, NiObject},
};
use crate::object3d::{Transform, Vec3};

ni_types! { ffi;
    /// Skinning data for an [`NiSkinInstance`]: the global-to-skin transform and per-bone
    /// transforms. Reach it from a shape with [`NifFile::child_blocks_of`](crate::file::NifFile::child_blocks_of):
    ///
    /// ```no_run
    /// # use nifly::{file::NifFile, geometry::NiShape, skin::{NiSkinData, NiSkinInstance}};
    /// # fn f(nif: &NifFile, shape: &NiShape) -> Option<()> {
    /// let inst = nif.child_blocks_of::<NiSkinInstance>(shape).next()?;
    /// let data = nif.child_blocks_of::<NiSkinData>(inst).next()?;
    /// let t = data.transform_global_to_skin();
    /// # Some(())
    /// # }
    /// ```
    NiSkinData(NiSkinData): NiObject => upcast_ni_skin_data, downcast_ni_skin_data;
    /// Skinned geometry split into partitions of bones.
    NiSkinPartition(NiSkinPartition): NiObject => upcast_ni_skin_partition, downcast_ni_skin_partition;
    /// Base class of skin instances, which list a shape's bones. Its skeleton root is
    /// [`View::root`].
    NiBoneContainer(NiBoneContainer): NiObject => upcast_ni_bone_container, downcast_ni_bone_container;
    /// A shape's skin, with its data in an [`NiSkinData`] (Skyrim SE and earlier).
    NiSkinInstance(NiSkinInstance): NiBoneContainer => upcast_ni_skin_instance, downcast_ni_skin_instance;
    /// An [`NiSkinInstance`] with dismemberment partitions.
    BsDismemberSkinInstance(BSDismemberSkinInstance): NiSkinInstance => upcast_bs_dismember_skin_instance, downcast_bs_dismember_skin_instance;
    /// A shape's skin, with its data in a [`BsSkinBoneData`] (Fallout 4 and later).
    BsSkinInstance(BSSkinInstance): NiBoneContainer => upcast_bs_skin_instance, downcast_bs_skin_instance;
    /// Per-bone data for a [`BsSkinInstance`].
    BsSkinBoneData(BSSkinBoneData): NiObject => upcast_bs_skin_bone_data, downcast_bs_skin_bone_data;
}

impl NiSkinData {
    /// The transform from global coordinates to skin coordinates.
    pub fn transform_global_to_skin(&self) -> Transform {
        Transform::from_ffi(ffi::skin_data_transform_global_to_skin(&self.0))
    }

    /// Sets the [global-to-skin transform](View::transform_global_to_skin).
    pub fn set_transform_global_to_skin(&mut self, transform: Transform) {
        ffi::set_skin_data_transform_global_to_skin(self.ffi_mut(), transform.to_ffi());
    }

    pub fn bone_count(&self) -> usize {
        ffi::skin_data_bone_count(&self.0)
    }

    /// The transform from skin coordinates to the coordinates of bone `index`, or `None` if
    /// `index` is out of range. Bones are in the skin instance's order.
    pub fn transform_skin_to_bone(&self, index: usize) -> Option<Transform> {
        let mut out = Transform::IDENTITY.to_ffi();
        ffi::skin_data_transform_skin_to_bone(&self.0, index, &mut out).then(|| Transform::from_ffi(out))
    }

    /// Whether the block stores per-bone vertex weights.
    pub fn has_vertex_weights(&self) -> bool {
        ffi::skin_data_has_vertex_weights(&self.0)
    }

    /// Sets [`has_vertex_weights`](Self::has_vertex_weights). When off, the weights aren't
    /// saved, except in files before version 4.2.1.0, which always save them.
    pub fn set_has_vertex_weights(&mut self, value: bool) {
        ffi::set_skin_data_has_vertex_weights(self.ffi_mut(), value);
    }

    /// The bounds of the vertices bone `index` affects, in its coordinates, or `None` if `index`
    /// is out of range.
    pub fn bounds(&self, index: usize) -> Option<BoundingSphere> {
        let mut out = ZERO_BOUNDS.to_ffi();
        ffi::skin_data_bone_bounds(&self.0, index, &mut out).then(|| BoundingSphere::from_ffi(out))
    }

    /// Sets the [skin-to-bone transform](Self::transform_skin_to_bone) of bone `index`.
    /// Returns `false` if `index` is out of range.
    pub fn set_transform_skin_to_bone(&mut self, index: usize, transform: Transform) -> bool {
        ffi::set_skin_data_transform_skin_to_bone(self.ffi_mut(), index, transform.to_ffi())
    }

    /// Sets the [bounds](Self::bounds) of bone `index`. Returns `false` if `index` is out of
    /// range.
    pub fn set_bounds(&mut self, index: usize, bounds: BoundingSphere) -> bool {
        ffi::set_skin_data_bone_bounds(self.ffi_mut(), index, bounds.to_ffi())
    }

    /// The vertex weights of bone `index` as stored, or `None` if `index` is out of range.
    pub fn vertex_weights(&self, index: usize) -> Option<Vec<VertexWeight>> {
        let (mut vertices, mut weights) = (Vec::new(), Vec::new());
        ffi::skin_data_vertex_weights(&self.0, index, &mut vertices, &mut weights).then_some(())?;
        Some(vertices.into_iter().zip(weights).map(|(vertex, weight)| VertexWeight { vertex, weight }).collect())
    }

    /// Replaces the [vertex weights](Self::vertex_weights) of bone `index` as given, without
    /// sorting or normalizing them. Returns `false`, changing nothing, if `index` is out of range
    /// or there are more than 65535. See also [`Edit::set_bone_weights`] on the shape.
    pub fn set_vertex_weights(&mut self, index: usize, weights: &[VertexWeight]) -> bool {
        let vertices: Vec<u16> = weights.iter().map(|w| w.vertex).collect();
        let values: Vec<f32> = weights.iter().map(|w| w.weight).collect();
        ffi::set_skin_data_vertex_weights(self.ffi_mut(), index, &vertices, &values)
    }
}

const ZERO_BOUNDS: BoundingSphere = BoundingSphere { center: Vec3::ZERO, radius: 0.0 };

impl BsSkinBoneData {
    pub fn bone_count(&self) -> usize {
        ffi::skin_bone_data_bone_count(&self.0)
    }

    /// The transform from skin coordinates to the coordinates of bone `index`, or `None` if
    /// `index` is out of range. Bones are in the skin instance's order.
    pub fn transform_skin_to_bone(&self, index: usize) -> Option<Transform> {
        let mut out = Transform::IDENTITY.to_ffi();
        ffi::skin_bone_data_transform_skin_to_bone(&self.0, index, &mut out).then(|| Transform::from_ffi(out))
    }

    /// The bounds of the vertices bone `index` affects, in its coordinates, or `None` if `index`
    /// is out of range.
    pub fn bounds(&self, index: usize) -> Option<BoundingSphere> {
        let mut out = ZERO_BOUNDS.to_ffi();
        ffi::skin_bone_data_bounds(&self.0, index, &mut out).then(|| BoundingSphere::from_ffi(out))
    }

    /// Sets the [skin-to-bone transform](Self::transform_skin_to_bone) of bone `index`.
    /// Returns `false` if `index` is out of range.
    pub fn set_transform_skin_to_bone(&mut self, index: usize, transform: Transform) -> bool {
        ffi::set_skin_bone_data_transform_skin_to_bone(self.ffi_mut(), index, transform.to_ffi())
    }

    /// Sets the [bounds](Self::bounds) of bone `index`. Returns `false` if `index` is out of
    /// range.
    pub fn set_bounds(&mut self, index: usize, bounds: BoundingSphere) -> bool {
        ffi::set_skin_bone_data_bounds(self.ffi_mut(), index, bounds.to_ffi())
    }
}

impl BsSkinInstance {
    /// Per-bone scales, usually one per bone, in the skin instance's order.
    pub fn scales(&self) -> Vec<Vec3> {
        ffi::skin_instance_scales(&self.0).into_iter().map(Vec3::from_ffi).collect()
    }

    /// Sets the [scales](Self::scales). Not checked against the bone count.
    pub fn set_scales(&mut self, scales: &[Vec3]) {
        let scales: Vec<_> = scales.iter().map(|v| v.to_ffi()).collect();
        ffi::set_skin_instance_scales(self.ffi_mut(), &scales);
    }
}

impl BsDismemberSkinInstance {
    /// The number of dismember partitions, one per [`NiSkinPartition`] partition.
    pub fn len(&self) -> usize {
        ffi::dismember_partition_count(&self.0)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The flags of partition `index`, or `None` if `index` is out of range.
    pub fn flags(&self, index: usize) -> Option<PartitionFlags> {
        (index < self.len()).then(|| PartitionFlags::from_bits_retain(ffi::dismember_partition_flags(&self.0, index)))
    }

    /// The body part of partition `index`, or `None` if `index` is out of range.
    pub fn body_part(&self, index: usize) -> Option<u16> {
        (index < self.len()).then(|| ffi::dismember_partition_body_part(&self.0, index))
    }

    /// Sets the flags of partition `index`. Returns `false` if `index` is out of range.
    pub fn set_flags(&mut self, index: usize, flags: PartitionFlags) -> bool {
        ffi::set_dismember_partition_flags(self.ffi_mut(), index, flags.bits())
    }

    /// Sets the body part of partition `index`. Returns `false` if `index` is out of range.
    pub fn set_body_part(&mut self, index: usize, part: u16) -> bool {
        ffi::set_dismember_partition_body_part(self.ffi_mut(), index, part)
    }
}

/// The partitions of a skinned shape, from
/// [`NifFile::partitions`](crate::handle::View::partitions).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShapePartitions {
    /// Each partition's flags and body part.
    pub partitions: Vec<PartitionInfo>,
    /// The partition of each triangle, in [`NifFile::triangles`](crate::handle::View::triangles)
    /// order, or `None` if it's in none.
    pub triangles: Vec<Option<usize>>,
}

/// A partition's flags and body part.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PartitionInfo {
    pub flags: PartitionFlags,
    /// The body part (dismemberment slot) the partition belongs to.
    pub body_part: u16,
}

bitflags! {
    /// The flags of a [`BsDismemberSkinInstance`] partition. Other bits are kept as stored.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct PartitionFlags: u16 {
        /// Visible in the editor.
        const EDITOR_VISIBLE = 1;
        /// Starts a new set of bones, rather than using the previous partition's.
        const START_BONE_SET = 1 << 8;
        const _ = !0;
    }
}

impl NiSkinPartition {
    /// The number of partitions.
    pub fn len(&self) -> usize {
        ffi::skin_partition_count(&self.0)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The bones partition `index` uses, as positions in the shape's
    /// [bone list](View::bone_list), or `None` if `index` is out of range.
    pub fn bones(&self, index: usize) -> Option<&[u16]> {
        (index < self.len()).then(|| ffi::skin_partition_bones(&self.0, index))
    }

    /// The shape vertices partition `index` covers, in its own vertex order, or `None` if `index`
    /// is out of range.
    pub fn vertex_map(&self, index: usize) -> Option<&[u16]> {
        (index < self.len()).then(|| ffi::skin_partition_vertex_map(&self.0, index))
    }

    /// The number of triangles in partition `index`, or `None` if `index` is out of range.
    pub fn triangle_count(&self, index: usize) -> Option<usize> {
        (index < self.len()).then(|| ffi::skin_partition_triangle_count(&self.0, index).into())
    }

    /// The number of weights each vertex of partition `index` has, or `None` if `index` is out of
    /// range.
    pub fn weights_per_vertex(&self, index: usize) -> Option<usize> {
        (index < self.len()).then(|| ffi::skin_partition_weights_per_vertex(&self.0, index).into())
    }

    /// The weights of each vertex in partition `index`, in [vertex map](Self::vertex_map) order,
    /// or `None` if `index` is out of range. Empty if the partition stores none.
    pub fn vertex_weights(&self, index: usize) -> Option<&[[f32; 4]]> {
        (index < self.len()).then(|| ffi::skin_partition_vertex_weights(&self.0, index).as_chunks().0)
    }

    /// The bones of each weight in [`vertex_weights`](Self::vertex_weights), as positions in
    /// [`bones`](Self::bones), or `None` if `index` is out of range. Empty if the partition
    /// stores none.
    pub fn bone_indices(&self, index: usize) -> Option<&[[u8; 4]]> {
        (index < self.len()).then(|| ffi::skin_partition_bone_indices(&self.0, index).as_chunks().0)
    }

    /// The triangles of partition `index`, or `None` if `index` is out of range. Empty if the
    /// partition uses [strips](Self::strips). They index into the [vertex map](Self::vertex_map),
    /// except in Skyrim SE files, where they're shape vertex indices.
    pub fn triangles(&self, index: usize) -> Option<&[[u16; 3]]> {
        (index < self.len()).then(|| ffi::skin_partition_triangles(&self.0, index).as_chunks().0)
    }

    /// The triangle strips of partition `index`, indexing into the [vertex
    /// map](Self::vertex_map), or `None` if `index` is out of range. Most partitions use
    /// [triangles](View::triangles) instead.
    pub fn strips(&self, index: usize) -> Option<Vec<&[u16]>> {
        (index < self.len()).then(|| {
            (0..ffi::skin_partition_strip_count(&self.0, index))
                .map(|i| ffi::skin_partition_strip(&self.0, index, i))
                .collect()
        })
    }

    /// The level of detail of partition `index`, or `None` if `index` is out of range. Saved
    /// from Skyrim on.
    pub fn lod_level(&self, index: usize) -> Option<u8> {
        (index < self.len()).then(|| ffi::skin_partition_lod_level(&self.0, index))
    }

    /// Sets the [LOD level](Self::lod_level) of partition `index`. Returns `false` if `index` is
    /// out of range.
    pub fn set_lod_level(&mut self, index: usize, level: u8) -> bool {
        ffi::set_skin_partition_lod_level(self.ffi_mut(), index, level)
    }

    /// Whether partition `index` uses the global vertex buffer, or `None` if `index` is out of
    /// range. Saved from Skyrim on.
    pub fn global_vb(&self, index: usize) -> Option<bool> {
        (index < self.len()).then(|| ffi::skin_partition_global_vb(&self.0, index))
    }

    /// Sets [`global_vb`](Self::global_vb) for partition `index`. Returns `false` if `index` is
    /// out of range.
    pub fn set_global_vb(&mut self, index: usize, value: bool) -> bool {
        ffi::set_skin_partition_global_vb(self.ffi_mut(), index, value)
    }

    fn flag(&self, index: usize, which: u8) -> Option<bool> {
        (index < self.len()).then(|| ffi::skin_partition_flag(&self.0, index, which))
    }

    /// Whether partition `index` saves its [vertex map](Self::vertex_map), or `None` if `index`
    /// is out of range.
    pub fn has_vertex_map(&self, index: usize) -> Option<bool> {
        self.flag(index, 0)
    }

    /// Whether partition `index` saves its [vertex weights](Self::vertex_weights), or `None` if
    /// `index` is out of range.
    pub fn has_vertex_weights(&self, index: usize) -> Option<bool> {
        self.flag(index, 1)
    }

    /// Whether partition `index` saves its [triangles](Self::triangles) or
    /// [strips](Self::strips), or `None` if `index` is out of range.
    pub fn has_faces(&self, index: usize) -> Option<bool> {
        self.flag(index, 2)
    }

    /// Whether partition `index` saves its [bone indices](Self::bone_indices), or `None` if
    /// `index` is out of range.
    pub fn has_bone_indices(&self, index: usize) -> Option<bool> {
        self.flag(index, 3)
    }

    /// Sets [`has_vertex_map`](Self::has_vertex_map). A saved list shorter than the partition's
    /// vertex count is padded with zeros. Returns `false` if `index` is out of range.
    pub fn set_has_vertex_map(&mut self, index: usize, value: bool) -> bool {
        ffi::set_skin_partition_flag(self.ffi_mut(), index, 0, value)
    }

    /// Sets [`has_vertex_weights`](Self::has_vertex_weights), as
    /// [`set_has_vertex_map`](Self::set_has_vertex_map) does.
    pub fn set_has_vertex_weights(&mut self, index: usize, value: bool) -> bool {
        ffi::set_skin_partition_flag(self.ffi_mut(), index, 1, value)
    }

    /// Sets [`has_faces`](Self::has_faces), as [`set_has_vertex_map`](Self::set_has_vertex_map)
    /// does.
    pub fn set_has_faces(&mut self, index: usize, value: bool) -> bool {
        ffi::set_skin_partition_flag(self.ffi_mut(), index, 2, value)
    }

    /// Sets [`has_bone_indices`](Self::has_bone_indices), as
    /// [`set_has_vertex_map`](Self::set_has_vertex_map) does.
    pub fn set_has_bone_indices(&mut self, index: usize, value: bool) -> bool {
        ffi::set_skin_partition_flag(self.ffi_mut(), index, 3, value)
    }

    /// The vertex format of the partition's own vertex data, as raw bits. Saved in Skyrim SE
    /// files only, where saving copies it, with the vertex data, from the `BSTriShape` using the
    /// partition.
    pub fn vertex_desc(&self) -> u64 {
        ffi::skin_partition_vertex_desc(&self.0)
    }

    /// Sets the [vertex format](Self::vertex_desc). The vertex data isn't converted.
    pub fn set_vertex_desc(&mut self, bits: u64) {
        ffi::set_skin_partition_vertex_desc(self.ffi_mut(), bits);
    }

    /// The vertex format stored with partition `index`, as raw bits, or `None` if `index` is out
    /// of range. Saved in Skyrim SE files only, where saving copies it from the `BSTriShape`
    /// using the partition.
    pub fn partition_vertex_desc(&self, index: usize) -> Option<u64> {
        (index < self.len()).then(|| ffi::skin_partition_block_vertex_desc(&self.0, index))
    }

    /// Sets the [vertex format](Self::partition_vertex_desc) of partition `index`. Returns
    /// `false` if `index` is out of range.
    pub fn set_partition_vertex_desc(&mut self, index: usize, bits: u64) -> bool {
        ffi::set_skin_partition_block_vertex_desc(self.ffi_mut(), index, bits)
    }

    /// The triangles of partition `index` as shape vertex indices, one per
    /// [triangle](Self::triangles), or `None` if `index` is out of range. Saved in Skyrim SE
    /// files only.
    pub fn true_triangles(&self, index: usize) -> Option<&[[u16; 3]]> {
        (index < self.len()).then(|| ffi::skin_partition_true_triangles(&self.0, index).as_chunks().0)
    }

    /// Sets the [true triangles](Self::true_triangles) of partition `index`. Returns `false`,
    /// changing nothing, if `index` is out of range or the count differs from
    /// [`triangle_count`](Self::triangle_count).
    pub fn set_true_triangles(&mut self, index: usize, triangles: &[[u16; 3]]) -> bool {
        ffi::set_skin_partition_true_triangles(self.ffi_mut(), index, triangles.as_flattened())
    }
}

/// The skeleton root of an [`NiSkinInstance`] or [`BsSkinInstance`].
impl<'a> View<'a, NiBoneContainer> {
    /// The skin's skeleton root, or `None` if it has none or isn't a skin instance.
    pub fn root(&self) -> Option<View<'a, NiAvObject>> {
        self.resolve(index_from_ffi(ffi::bone_container_root(&self.get().0)))
    }
}

impl<'a> View<'a, NiBoneContainer> {
    /// The skin's bones, in order. Skips empty refs.
    pub fn bones(&self) -> BlockIter<'a, NiNode, RefIndices> {
        self.follow_list(RefList::Bones)
    }
}

impl Edit<'_, NiBoneContainer> {
    /// Replaces the skin's [bones](View::bones), keeping their count. Returns `false`, changing
    /// nothing, unless each block is a node and there are as many as the skin's data has, if it
    /// has data. To change the count, use [`Edit::set_bone_ids`] on the shape.
    pub fn set_bones(&mut self, bones: &[Index]) -> bool {
        self.set_ref_list(RefList::Bones, bones)
    }

    /// Like [`View::root`], for editing.
    pub fn root(&mut self) -> Option<Edit<'_, NiAvObject>> {
        let index = self.view().root()?.index();
        self.resolve(Some(index))
    }

    /// Sets the skin's skeleton root. Returns `false`, changing nothing, unless the skin is an
    /// [`NiSkinInstance`] and `root` a node, or the skin a [`BsSkinInstance`] and `root` an
    /// [`NiAvObject`].
    pub fn set_root(&mut self, root: Index) -> bool {
        let skin = self.index() as u32;
        block_index_to_ffi(root).is_some_and(|root| nif_ffi::set_skin_root(self.file_mut().inner.pin_mut(), skin, root))
    }
}

/// The data and partition of an [`NiSkinInstance`].
impl<'a> View<'a, NiSkinInstance> {
    /// The skin's data, or `None` if it has none.
    pub fn data(&self) -> Option<View<'a, NiSkinData>> {
        self.follow(RefKind::SkinData)
    }

    /// The skin's partition, or `None` if it has none.
    pub fn partition(&self) -> Option<View<'a, NiSkinPartition>> {
        self.follow(RefKind::SkinPartition)
    }
}

impl Edit<'_, NiSkinInstance> {
    /// Like [`View::data`], for editing.
    pub fn data(&mut self) -> Option<Edit<'_, NiSkinData>> {
        self.follow(RefKind::SkinData)
    }

    /// Like [`View::partition`], for editing.
    pub fn partition(&mut self) -> Option<Edit<'_, NiSkinPartition>> {
        self.follow(RefKind::SkinPartition)
    }

    /// Points the skin at the [`NiSkinData`] at `data`, or clears it with `None`. The data
    /// should have one entry per bone. Returns `false`, changing nothing, if that block isn't
    /// one.
    pub fn set_data(&mut self, data: Option<Index>) -> bool {
        self.set_ref(RefKind::SkinData, data)
    }

    /// Points the skin at the [`NiSkinPartition`] at `partition`, or clears it with `None`.
    /// Returns `false`, changing nothing, if that block isn't one.
    pub fn set_partition(&mut self, partition: Option<Index>) -> bool {
        self.set_ref(RefKind::SkinPartition, partition)
    }
}

/// The partition of an [`NiSkinData`], saved in files from version 4.0.0.2 to 10.1.0.0.
impl<'a> View<'a, NiSkinData> {
    /// The data's partition, or `None` if it has none.
    pub fn partition(&self) -> Option<View<'a, NiSkinPartition>> {
        self.follow(RefKind::SkinDataPartition)
    }
}

impl Edit<'_, NiSkinData> {
    /// Like [`View::partition`], for editing.
    pub fn partition(&mut self) -> Option<Edit<'_, NiSkinPartition>> {
        self.follow(RefKind::SkinDataPartition)
    }

    /// Points the data at the [`NiSkinPartition`] at `partition`, or clears it with `None`.
    /// Returns `false`, changing nothing, if that block isn't one.
    pub fn set_partition(&mut self, partition: Option<Index>) -> bool {
        self.set_ref(RefKind::SkinDataPartition, partition)
    }
}

/// The bone data of a [`BsSkinInstance`].
impl<'a> View<'a, BsSkinInstance> {
    /// The skin's bone data, or `None` if it has none.
    pub fn data(&self) -> Option<View<'a, BsSkinBoneData>> {
        self.follow(RefKind::SkinBoneData)
    }
}

impl Edit<'_, BsSkinInstance> {
    /// Like [`View::data`], for editing.
    pub fn data(&mut self) -> Option<Edit<'_, BsSkinBoneData>> {
        self.follow(RefKind::SkinBoneData)
    }

    /// Points the skin at the [`BsSkinBoneData`] at `data`, or clears it with `None`. The data
    /// should have one entry per bone. Returns `false`, changing nothing, if that block isn't
    /// one.
    pub fn set_data(&mut self, data: Option<Index>) -> bool {
        self.set_ref(RefKind::SkinBoneData, data)
    }
}

/// Reading a shape.
impl<'a> View<'a, NiShape> {
    /// The shape's skin instance, or `None` if it isn't skinned. Cast it to an
    /// [`NiSkinInstance`] or [`BsSkinInstance`].
    pub fn skin_instance(&self) -> Option<View<'a, NiBoneContainer>> {
        self.follow(RefKind::SkinInstance)
    }

    /// Block indices of the bones (nodes) the shape is skinned to, in skin instance order. Empty if
    /// the shape isn't skinned. Empty refs are skipped. Use [`block`](crate::file::NifFile::block) to resolve them.
    ///
    /// Unlike [`bone_list`](Self::bone_list), this only reads the skin instance, so Starfield `BSGeometry` bones bound by `SkinAttach`
    /// names aren't included.
    pub fn bone_ids(&self) -> Vec<Index> {
        let (file, shape) = (self.file(), self.get());
        nif_ffi::shape_bone_ids(&file.inner, shape.as_ffi())
            .into_iter()
            .map(|id| id as Index)
            .collect()
    }

    /// Names of the bones (nodes) the shape is skinned to, in skin instance order. Empty if the
    /// shape isn't skinned. Bones whose refs don't resolve are skipped. Starfield `BSGeometry`
    /// reads the names from its `SkinAttach` extra data instead.
    pub fn bone_list(&self) -> Vec<BString> {
        let (file, shape) = (self.file(), self.get());
        nif_ffi::shape_bone_list(&file.inner, shape.as_ffi())
            .iter()
            .map(|name| BString::from(name.as_bytes()))
            .collect()
    }

    /// Vertex weights for one of the shape's bones, sorted by vertex index. Only vertices with a
    /// non-zero weight are included. Empty if the shape isn't skinned or `bone_index` is out of range.
    ///
    /// `bone_index` is the bone's position in [`bone_list`](Self::bone_list) /
    /// [`bone_ids`](Self::bone_ids), not its block index.
    ///
    /// Weights are read from wherever the shape stores them: per-vertex data (`BSTriShape`), mesh
    /// data (Starfield `BSGeometry`) or `NiSkinData`, where weights below 0.0001 are dropped.
    pub fn bone_weights(&self, bone_index: usize) -> Vec<VertexWeight> {
        let (file, shape) = (self.file(), self.get());
        match u32::try_from(bone_index) {
            Ok(bone_index) => nif_ffi::shape_bone_weights(&file.inner, shape.as_ffi(), bone_index),
            Err(_) => Vec::new(),
        }
    }

    /// The partitions of the shape and the partition each triangle is in, or `None` if it isn't
    /// skinned with an [`NiSkinInstance`] and [`NiSkinPartition`]. Without a
    /// [`BsDismemberSkinInstance`], every partition has default info: visible in the editor, body
    /// part 32 for Skyrim and later, 0 before.
    pub fn partitions(&self) -> Option<ShapePartitions> {
        let (file, shape) = (self.file(), self.get());
        let (mut info, mut tri_parts) = (Vec::new(), Vec::new());
        if !nif_ffi::shape_partitions(&file.inner, shape.as_ffi(), &mut info, &mut tri_parts) {
            return None;
        }
        Some(ShapePartitions {
            partitions: info
                .into_iter()
                .map(|p| PartitionInfo { flags: PartitionFlags::from_bits_retain(p.flags), body_part: p.body_part })
                .collect(),
            triangles: tri_parts.into_iter().map(|p| usize::try_from(p).ok()).collect(),
        })
    }

    /// The transform from global coordinates to the shape's skin coordinates, or `None` if the shape
    /// has no [`NiSkinData`]. Fallout 4 and later skins don't have one. A missing transform isn't
    /// necessarily the identity.
    pub fn transform_global_to_skin(&self) -> Option<Transform> {
        let (file, shape) = (self.file(), self.get());
        let mut out = Transform::IDENTITY.to_ffi();
        nif_ffi::shape_transform_global_to_skin(&file.inner, shape.as_ffi(), &mut out)
            .then(|| Transform::from_ffi(out))
    }

    /// Like [`transform_global_to_skin`](Self::transform_global_to_skin), but if the
    /// shape doesn't store the transform (Fallout 4 and later), calculates it from the first bone
    /// in [`bone_list`](Self::bone_list): the inverse of the bone's
    /// [global transform](crate::file::NifFile::node_transform_to_global) composed with its
    /// [skin-to-bone transform](Self::transform_skin_to_bone). The result is only right if
    /// that bone is in its bind pose. `None` if the shape isn't skinned or has no bones.
    pub fn calc_transform_global_to_skin(&self) -> Option<Transform> {
        let (file, shape) = (self.file(), self.get());
        let mut out = Transform::IDENTITY.to_ffi();
        nif_ffi::calc_shape_transform_global_to_skin(&file.inner, shape.as_ffi(), &mut out)
            .then(|| Transform::from_ffi(out))
    }

    /// The transform from the shape's skin coordinates to the coordinates of one of its bones, or
    /// `None` if the shape isn't skinned or `bone_index` is out of range. `bone_index` is the
    /// bone's position in [`bone_list`](Self::bone_list) /
    /// [`bone_ids`](Self::bone_ids).
    pub fn transform_skin_to_bone(&self, bone_index: usize) -> Option<Transform> {
        let (file, shape) = (self.file(), self.get());
        let bone_index = u32::try_from(bone_index).ok()?;
        let mut out = Transform::IDENTITY.to_ffi();
        nif_ffi::shape_transform_skin_to_bone(&file.inner, shape.as_ffi(), bone_index, &mut out)
            .then(|| Transform::from_ffi(out))
    }

    /// The bounding sphere of the vertices bone `bone_index` influences, in that bone's coordinates,
    /// or `None` if the shape isn't skinned or has no such bone. `bone_index` is the bone's
    /// position in [`bone_list`](Self::bone_list).
    pub fn bone_bounds(&self, bone_index: usize) -> Option<BoundingSphere> {
        let (file, shape) = (self.file(), self.get());
        let bone_index = u32::try_from(bone_index).ok()?;
        let mut out = BoundingSphere::default().to_ffi();
        nif_ffi::shape_bone_bounds(&file.inner, shape.as_ffi(), bone_index, &mut out)
            .then(|| BoundingSphere::from_ffi(out))
    }
}

/// Changing a shape.
impl Edit<'_, NiShape> {
    /// Like [`View::skin_instance`], for editing.
    pub fn skin_instance(&mut self) -> Option<Edit<'_, NiBoneContainer>> {
        self.follow(RefKind::SkinInstance)
    }

    /// Points the shape at the skin instance at `skin`, or clears it with `None`. The shape's
    /// vertex format and the skin's bones aren't changed. Returns `false`, changing nothing, if
    /// that block isn't an [`NiSkinInstance`] or [`BsSkinInstance`].
    pub fn set_skin_instance(&mut self, skin: Option<Index>) -> bool {
        self.set_ref(RefKind::SkinInstance, skin)
    }

    /// Replaces the vertex weights of bone `bone_index` (its position in
    /// [`bone_list`](View::bone_list)), stored in the shape's [`NiSkinData`]. If a vertex appears
    /// more than once, the last weight wins. Weights below 0.0001 are dropped, as
    /// [`bone_weights`](View::bone_weights) does when reading them.
    ///
    /// Returns `false`, changing nothing, if the shape isn't skinned with `NiSkinData`, the bone
    /// doesn't exist, or a vertex index is out of range. [`BsTriShape`] and [`BsGeometry`] are
    /// rejected: they store weights per vertex (see [`set_vertex_weights`](Self::set_vertex_weights)).
    ///
    /// The shape's [`NiSkinPartition`], if it has one, keeps its own copy of the weights. Update it
    /// with [`update_skin_partitions`](Self::update_skin_partitions).
    ///
    /// [`BsTriShape`]: crate::geometry::BsTriShape
    /// [`BsGeometry`]: crate::geometry::BsGeometry
    pub fn set_bone_weights(&mut self, bone_index: usize, weights: impl IntoIterator<Item = VertexWeight>) -> bool {
        let shape = self.index() as u32;
        let Ok(bone_index) = u32::try_from(bone_index) else {
            return false;
        };
        let by_vertex: BTreeMap<u16, f32> = weights.into_iter().map(|w| (w.vertex, w.weight)).collect();
        let weights: Vec<VertexWeight> =
            by_vertex.into_iter().map(|(vertex, weight)| VertexWeight { vertex, weight }).collect();
        nif_ffi::set_shape_bone_weights(self.file_mut().inner.pin_mut(), shape, bone_index, &weights)
    }


    /// Replaces the bones of the skinned shape with the nodes at these
    /// block indices, in order.
    ///
    /// Per-bone data is kept by position. If the bone count changes, it's reset instead: the
    /// [`NiSkinData`] bones lose their transforms, bounds and weights, and `BSSkin::BoneData`
    /// bones their transforms. Per-vertex weights in a [`BsTriShape`] refer to bone positions
    /// and aren't changed, and neither is the [`NiSkinPartition`]; see
    /// [`update_skin_partitions`](Self::update_skin_partitions).
    ///
    /// Returns `false`, changing nothing, if the shape isn't skinned, is a
    /// [`BsGeometry`] (which binds bones by name; use
    /// [`set_bone_list`](Self::set_bone_list)), or a block in `bones` isn't a node.
    ///
    /// [`BsTriShape`]: crate::geometry::BsTriShape
    pub fn set_bone_ids(&mut self, bones: &[Index]) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        let Some(shape) = block_index_to_ffi(shape) else {
            return false;
        };
        let Some(ids) = bones.iter().map(|&b| block_index_to_ffi(b)).collect::<Option<Vec<u32>>>() else {
            return false;
        };
        nif_ffi::set_shape_bone_ids(file.inner.pin_mut(), shape, &ids)
    }

    /// Replaces the bones of the skinned shape with the nodes of these
    /// names, in order, each the first node in block order with that name. Otherwise the same as
    /// [`set_bone_ids`](Self::set_bone_ids).
    ///
    /// A [`BsGeometry`] binds bones by name instead, so the names are stored in its `SkinAttach`
    /// extra data (created if missing) and needn't be nodes in the file.
    ///
    /// Returns `false`, changing nothing, if the shape isn't skinned or, except for a
    /// [`BsGeometry`], a name isn't a node's.
    pub fn set_bone_list<N: AsRef<[u8]>>(&mut self, names: impl IntoIterator<Item = N>) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        let names: Vec<N> = names.into_iter().collect();
        if file.block_of::<BsGeometry>(shape).is_some() {
            let Some(index) = block_index_to_ffi(shape) else {
                return false;
            };
            let bytes: Vec<u8> = names.iter().flat_map(|n| n.as_ref().iter().copied()).collect();
            let lengths: Vec<usize> = names.iter().map(|n| n.as_ref().len()).collect();
            return nif_ffi::set_shape_bone_names(file.inner.pin_mut(), index, &bytes, &lengths);
        }
        let ids: Option<Vec<Index>> = names
            .iter()
            .map(|name| file.nodes().indexed().find(|(_, n)| n.name() == name.as_ref()).map(|(i, _)| i))
            .collect();
        ids.is_some_and(|ids| self.set_bone_ids(&ids))
    }

    /// Makes the skinned shape use the node at block index `new` in place
    /// of the node at `old`. Only the first bone that is `old` changes, keeping its position, so
    /// its weights and transforms now apply to `new`.
    ///
    /// Returns `false`, changing nothing, if the shape isn't skinned, `new` isn't a node,
    /// or no bone is `old`. A [`BsGeometry`] binds bones by name, so it has no bones to replace.
    pub fn replace_bone(&mut self, old: Index, new: Index) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        let (Some(shape), Some(old), Some(new)) = (block_index_to_ffi(shape), block_index_to_ffi(old), block_index_to_ffi(new))
        else {
            return false;
        };
        nif_ffi::replace_shape_bone(file.inner.pin_mut(), shape, old, new)
    }

    /// Rebuilds the [`NiSkinPartition`] of the shape from its triangles
    /// and the vertex weights in its [`NiSkinData`], for example after
    /// [`set_bone_weights`](Self::set_bone_weights). Saving doesn't do this.
    ///
    /// Each vertex keeps its four largest weights, normalized. Triangles stay in their partition
    /// (ones in none go to the first). A partition that would use more bones than the game allows
    /// (18 in Oblivion and Fallout 3, 80 in Skyrim SE) is split, and a
    /// [`BsDismemberSkinInstance`] gets a partition with the same body part for each new one.
    /// Partitions are stored as triangles, not strips.
    ///
    /// Returns `false`, changing nothing, if the shape has no [`NiSkinInstance`],
    /// [`NiSkinData`] that stores vertex weights, and an [`NiSkinPartition`] with at least one
    /// partition; if a [`BsDismemberSkinInstance`] lists a different number of partitions; or if
    /// a file without a bone limit has more than 256 bones.
    pub fn update_skin_partitions(&mut self) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        let Some(shape) = block_index_to_ffi(shape) else {
            return false;
        };
        nif_ffi::update_skin_partitions(file.inner.pin_mut(), shape)
    }

    /// Recalculates the [flags](BsDismemberSkinInstance::flags) of the dismember partitions of
    /// the shape. Every partition becomes visible in the editor, except
    /// Fallout 3 and New Vegas meat caps (body parts 100 to 999). The first partition, and each
    /// one whose bones differ from the previous one's, starts a new bone set.
    /// [`update_skin_partitions`](Self::update_skin_partitions) does this too.
    ///
    /// Returns `false`, changing nothing, unless the shape has a [`BsDismemberSkinInstance`] and
    /// an [`NiSkinPartition`] with as many partitions.
    pub fn update_partition_flags(&mut self) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        let Some(shape) = block_index_to_ffi(shape) else {
            return false;
        };
        nif_ffi::update_partition_flags(file.inner.pin_mut(), shape)
    }

    /// Replaces the bone weights of vertex `vertex` of the shape, as
    /// `(bone, weight)` pairs where `bone` is a position in
    /// [`bone_list`](View::bone_list). Weights are scaled to add up to 1 (unless
    /// they're all 0), and the vertex's unused slots are cleared. An empty slice clears the
    /// vertex's weights.
    ///
    /// Only shapes that store weights per vertex are supported: a skinned [`BsTriShape`], which
    /// holds up to 4 weights per vertex, and a [`BsGeometry`] mesh. Returns `false`, changing
    /// nothing, if the shape isn't one, `vertex` is out of range, there are more pairs than the
    /// vertex holds, a bone isn't in the bone list (a [`BsTriShape`] stores at most 256), or a
    /// weight is negative or not finite. Use [`set_bone_weights`](Self::set_bone_weights)
    /// for other shapes.
    ///
    /// The shape's [`NiSkinData`], if it has one, keeps its own per-bone weights and isn't
    /// updated.
    ///
    /// [`BsTriShape`]: crate::geometry::BsTriShape
    /// [`BsGeometry`]: crate::geometry::BsGeometry
    pub fn set_vertex_weights(&mut self, vertex: usize, weights: &[(usize, f32)]) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        let (Some(shape), Ok(vertex)) = (block_index_to_ffi(shape), u32::try_from(vertex)) else {
            return false;
        };
        let Ok(bones) = weights.iter().map(|&(bone, _)| u32::try_from(bone)).collect::<Result<Vec<_>, _>>() else {
            return false;
        };
        let weights: Vec<f32> = weights.iter().map(|&(_, w)| w).collect();
        nif_ffi::set_shape_vertex_weights(file.inner.pin_mut(), shape, vertex, &bones, &weights)
    }

    /// Clears the bone weights of every vertex of the shape, leaving it
    /// unweighted. Set new ones with [`set_vertex_weights`](Self::set_vertex_weights).
    ///
    /// Returns `false`, changing nothing, unless that block is a shape that stores weights per
    /// vertex: a skinned [`BsTriShape`] or [`BsGeometry`] mesh. The shape's [`NiSkinData`], if it
    /// has one, isn't updated.
    ///
    /// [`BsTriShape`]: crate::geometry::BsTriShape
    /// [`BsGeometry`]: crate::geometry::BsGeometry
    pub fn clear_vertex_weights(&mut self) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        block_index_to_ffi(shape).is_some_and(|shape| nif_ffi::clear_shape_vertex_weights(file.inner.pin_mut(), shape))
    }

    /// Adds the blocks the shape needs to be skinned, and turns on the
    /// skinning flags of the shape and its shader. Which blocks depends on the shape and version:
    ///
    /// - [`NiTriShape`] and [`NiTriStrips`]: an [`NiSkinInstance`] ([`BsDismemberSkinInstance`]
    ///   in Fallout 3 and Skyrim LE files), [`NiSkinData`] and an [`NiSkinPartition`] with one
    ///   partition covering every triangle. Morrowind files don't save the partition.
    /// - Skyrim SE [`BsTriShape`]: the same, with a [`BsDismemberSkinInstance`].
    /// - Fallout 4 and later [`BsTriShape`], and [`BsGeometry`]: a [`BsSkinInstance`] and
    ///   [`BsSkinBoneData`]; a [`BsGeometry`] also gets `SkinAttach` extra data for bone names.
    ///
    /// The skin starts with no bones; add them with [`set_bone_list`](Self::set_bone_list)
    /// or [`set_bone_ids`](Self::set_bone_ids). New blocks are added at the end, so existing block indices
    /// don't change. Returns `false`, changing nothing, if the shape isn't one of those
    /// or is already skinned.
    ///
    /// [`NiTriShape`]: crate::geometry::NiTriShape
    /// [`NiTriStrips`]: crate::geometry::NiTriStrips
    /// [`BsTriShape`]: crate::geometry::BsTriShape
    /// [`BsGeometry`]: crate::geometry::BsGeometry
    pub fn create_skinning(&mut self) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        block_index_to_ffi(shape).is_some_and(|shape| nif_ffi::create_skinning(file.inner.pin_mut(), shape))
    }

    /// Removes the skinning of the shape: unlinks its skin instance, and
    /// turns off the skinning flags of the shape and its shader. The skin instance and its skin
    /// data, partition or bone data are deleted unless other blocks use them, which moves every
    /// later block down. Bone nodes stay. For a [`BsGeometry`], its `SkinAttach` extra data
    /// (deleted unless shared) and per-vertex weights go too.
    ///
    /// Returns `false`, changing nothing, if the shape has no skin instance.
    ///
    /// [`BsGeometry`]: crate::geometry::BsGeometry
    pub fn delete_skinning(&mut self) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        block_index_to_ffi(shape).is_some_and(|shape| nif_ffi::delete_skinning(file.inner.pin_mut(), shape))
    }

    /// Splits the triangles of the shape into partitions with these body
    /// parts: `triangles` gives each triangle's partition, as an index into `body_parts`, in
    /// [`triangles`](View::triangles) order. Triangles given `None` go into an extra
    /// partition after the others, with the default body part (see
    /// [`partitions`](View::partitions)).
    ///
    /// The partitions are then rebuilt as [`update_skin_partitions`](Self::update_skin_partitions)
    /// does: one that would use too many bones is split, and flags are recalculated. In Fallout 3
    /// and Skyrim LE files, a plain [`NiSkinInstance`] is replaced with a
    /// [`BsDismemberSkinInstance`] at the same block index to hold the body parts; other files
    /// without one don't store body parts.
    ///
    /// Returns `false`, changing nothing, if the shape isn't one whose partitions can be
    /// rebuilt (see [`update_skin_partitions`](Self::update_skin_partitions)), `triangles` doesn't
    /// have one entry per triangle, or an entry isn't below `body_parts.len()`.
    pub fn set_partitions(&mut self, body_parts: &[u16], triangles: &[Option<usize>]) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        let Some(shape) = block_index_to_ffi(shape) else {
            return false;
        };
        let Some(tri_parts) = triangles
            .iter()
            .map(|p| match p {
                None => Some(-1),
                Some(p) => i32::try_from(*p).ok(),
            })
            .collect::<Option<Vec<i32>>>()
        else {
            return false;
        };
        let info: Vec<_> = body_parts.iter().map(|&body_part| nif_ffi::PartitionInfo { flags: 0, body_part }).collect();
        nif_ffi::set_shape_partitions(file.inner.pin_mut(), shape, &info, &tri_parts)
    }

    /// Puts every triangle of the shape into one partition with the
    /// default body part: 32 for Skyrim and later, 0 before. Otherwise the same as
    /// [`set_partitions`](Self::set_partitions), including when it returns `false`.
    pub fn set_default_partition(&mut self) -> bool {
        let Some(count) = self.view().triangles().map(|t| t.len()) else {
            return false;
        };
        let body_part = if self.view().file().version().user >= 12 { 32 } else { 0 };
        self.set_partitions(&[body_part], &vec![Some(0); count])
    }

    /// Deletes the partitions at these positions from the shape, along with
    /// their [`BsDismemberSkinInstance`] entries; later partitions move down. Order and repeats
    /// don't matter. Their triangles stay in the shape but are in no partition until
    /// [`update_skin_partitions`](Self::update_skin_partitions) puts them in the first one, or
    /// [`set_partitions`](Self::set_partitions) assigns them. A skinned Skyrim SE
    /// [`BsTriShape`] saves its triangles in its partitions, so saving drops them. Partition flags
    /// are recalculated as [`update_partition_flags`](Self::update_partition_flags) does.
    ///
    /// Returns `false`, changing nothing, if the shape has no [`NiSkinInstance`]
    /// and [`NiSkinPartition`], a position isn't a partition, or a [`BsDismemberSkinInstance`]
    /// lists a different number of partitions.
    ///
    /// [`BsTriShape`]: crate::geometry::BsTriShape
    pub fn delete_partitions(&mut self, partitions: &[usize]) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        let Some(shape) = block_index_to_ffi(shape) else {
            return false;
        };
        let Ok(indices) = partitions.iter().map(|&p| u32::try_from(p)).collect::<Result<Vec<u32>, _>>() else {
            return false;
        };
        nif_ffi::delete_partitions(file.inner.pin_mut(), shape, &indices)
    }

    /// Deletes the partitions of the shape that have no triangles, as
    /// [`delete_partitions`](Self::delete_partitions) does, and returns how many there were.
    ///
    /// Returns `None`, changing nothing, if the shape has no [`NiSkinInstance`]
    /// and [`NiSkinPartition`], or a [`BsDismemberSkinInstance`] lists a different number of
    /// partitions.
    pub fn remove_empty_partitions(&mut self) -> Option<usize> {
        let (shape, file) = (self.index(), self.file_mut());
        let shape = block_index_to_ffi(shape)?;
        usize::try_from(nif_ffi::remove_empty_partitions(file.inner.pin_mut(), shape)).ok()
    }

    /// Sets the transform from the skin coordinates of the shape to the
    /// coordinates of bone `bone_index` (its position in [`bone_list`](View::bone_list)).
    /// Returns `false`, changing nothing, if the shape isn't skinned or has no
    /// such bone. Works for both [`NiSkinData`] and
    /// [`BsSkinBoneData`] skins.
    pub fn set_transform_skin_to_bone(&mut self, bone_index: usize, transform: Transform) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        let (Some(shape), Ok(bone_index)) = (block_index_to_ffi(shape), u32::try_from(bone_index)) else {
            return false;
        };
        nif_ffi::set_shape_transform_skin_to_bone(file.inner.pin_mut(), shape, bone_index, transform.to_ffi())
    }

    /// Sets the bounding sphere of bone `bone_index` of the shape.
    /// Returns `false`, changing nothing, if the shape isn't skinned or has no
    /// such bone. Works for both [`NiSkinData`] and
    /// [`BsSkinBoneData`] skins.
    pub fn set_bone_bounds(&mut self, bone_index: usize, bounds: BoundingSphere) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        let (Some(shape), Ok(bone_index)) = (block_index_to_ffi(shape), u32::try_from(bone_index)) else {
            return false;
        };
        nif_ffi::set_shape_bone_bounds(file.inner.pin_mut(), shape, bone_index, bounds.to_ffi())
    }
}
