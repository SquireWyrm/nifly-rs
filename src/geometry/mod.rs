//! `NiShape`, the base class of all geometry, and its subclasses.
//!
//! # Type hierarchy
//!
//! <!-- type-hierarchy:start -->
//!
//! - [`objects::NiAvObject`](crate::objects::NiAvObject)
//!   - [`NiShape`]
//!     - [`BsTriShape`]
//!       - [`BsSubIndexTriShape`]
//!       - [`BsMeshLodTriShape`]
//!       - [`BsDynamicTriShape`]
//!     - [`BsGeometry`]
//!     - [`NiGeometry`]
//!       - [`NiTriBasedGeom`]
//!         - [`NiTriShape`]
//!           - [`NiScreenElements`]
//!           - [`BsSegmentedTriShape`]
//!         - [`NiTriStrips`]
//!         - [`NiLines`]
//!         - [`BsLodTriShape`]
//!       - [`NiParticles`]
//!         - [`NiAutoNormalParticles`]
//!         - [`NiParticleMeshes`]
//!         - [`NiRotatingParticles`]
//! - [`objects::NiObject`](crate::objects::NiObject)
//!   - [`NiGeometryData`]
//!     - [`NiTriBasedGeomData`]
//!       - [`NiTriShapeData`]
//!         - [`NiScreenElementsData`]
//!       - [`NiTriStripsData`]
//!     - [`NiLinesData`]
//!     - [`BsGeometryMeshData`]
//!   - [`AdditionalGeomData`]
//!     - [`NiAdditionalGeometryData`]
//!     - [`BsPackedAdditionalGeometryData`]
//!
//! <!-- type-hierarchy:end -->

use std::{collections::HashSet, io, pin::Pin};

use bitflags::bitflags;
use bstr::{BStr, BString, ByteSlice};
use int_enum::IntEnum;
use nifly_sys::{
    geometry::ffi,
    nif_file::ffi as nif_ffi,
    nif_file_refs::ffi::{self as refs_ffi, RefKind},
    read_bridge::RustReader,
};

use crate::{
    error::{Error, LoadError, NifResult},
    file::{Index, NifFile, block_index_to_ffi, index_from_ffi},
    handle::{Edit, View},
    iter::BlockIter,
};

use crate::{
    object3d::{BoundingSphere, Matrix3, Vec3},
    objects::{IsA, NiAvObject, NiObject, NiType},
};

/// Where [`NifFile::texture_slot`](crate::handle::View::texture_slot) found a texture path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextureSource {
    /// A slot of the shader's `BSShaderTextureSet`.
    TextureSet,
    /// A texture built into a `BSEffectShaderProperty` (which has no texture set).
    EffectShader,
    /// An `NiSourceTexture` referenced by an `NiTexturingProperty` (Oblivion and older).
    TexturingProperty,
}

/// The segments of a [`BsSubIndexTriShape`], from
/// [`NifFile::segments`](crate::handle::View::segments).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ShapeSegments {
    /// The segment file (`.ssf`) path, as stored. Empty if there's none.
    pub ssf_file: BString,
    pub segments: Vec<Segment>,
    /// The segment of each triangle, in
    /// [`NifFile::triangles`](crate::handle::View::triangles) order, or `None` if
    /// it's in none.
    pub triangles: Vec<Option<SegmentIndex>>,
}

/// A segment of a [`BsSubIndexTriShape`].
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Segment {
    pub subsegments: Vec<Subsegment>,
}

/// A subsegment of a [`Segment`].
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Subsegment {
    /// The body part slot, 30 or more, or `None` for subsegments numbered automatically within
    /// their segment.
    pub user_slot: Option<u32>,
    /// The material (a hash), as stored.
    pub material: u32,
    pub extra_data: Vec<f32>,
}

/// Where a triangle is: a segment, and one of its subsegments if any.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SegmentIndex {
    pub segment: usize,
    pub subsegment: Option<usize>,
}

/// Options for [`NifFile::calc_normals`](crate::handle::Edit::calc_normals).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NormalOptions {
    /// Average the normals of vertices at the same position, such as along UV seams, where they
    /// differ by less than `smooth_threshold`.
    pub smooth: bool,
    /// The largest angle in degrees between normals that `smooth` averages.
    pub smooth_threshold: f32,
    /// Also calculate normals for Skyrim shapes whose shader uses model-space normal maps, which
    /// don't use vertex normals.
    pub force_model_space: bool,
}

impl Default for NormalOptions {
    fn default() -> Self {
        Self {
            smooth: true,
            smooth_threshold: 60.0,
            force_model_space: false,
        }
    }
}

/// One vertex's weight for a bone, from [`NifFile::bone_weights`](crate::handle::View::bone_weights).
pub type VertexWeight = nifly_sys::nif_file::ffi::VertexWeight;

ni_types! { ffi;
    /// Base class of all geometry. Use [`block_name`](crate::objects::NiObject::block_name) to see
    /// the concrete type.
    NiShape(NiShape): NiAvObject => upcast_ni_shape, downcast_ni_shape;
    /// Geometry with its vertex data in the shape itself.
    BsTriShape(BSTriShape): NiShape => upcast_bs_tri_shape, downcast_bs_tri_shape;
    /// A [`BsTriShape`] split into segments.
    BsSubIndexTriShape(BSSubIndexTriShape): BsTriShape => upcast_bs_sub_index_tri_shape, downcast_bs_sub_index_tri_shape;
    /// `BSMeshLODTriShape`.
    BsMeshLodTriShape(BSMeshLODTriShape): BsTriShape => upcast_bs_mesh_lod_tri_shape, downcast_bs_mesh_lod_tri_shape;
    /// A [`BsTriShape`] whose vertices can be updated at runtime, used for head parts.
    /// Loading reads its dynamic vertex data into the vertex positions and the bitangents' X,
    /// and saving writes it back from them.
    BsDynamicTriShape(BSDynamicTriShape): BsTriShape => upcast_bs_dynamic_tri_shape, downcast_bs_dynamic_tri_shape;
    /// Geometry stored in external mesh files (Starfield).
    BsGeometry(BSGeometry): NiShape => upcast_bs_geometry, downcast_bs_geometry;
    /// Geometry with its vertex data in a separate `NiGeometryData` block.
    NiGeometry(NiGeometry): NiShape => upcast_ni_geometry, downcast_ni_geometry;
    NiTriBasedGeom(NiTriBasedGeom): NiGeometry => upcast_ni_tri_based_geom, downcast_ni_tri_based_geom;
    NiTriShape(NiTriShape): NiTriBasedGeom => upcast_ni_tri_shape, downcast_ni_tri_shape;
    /// Geometry stored as triangle strips. Not supported by Skyrim SE.
    NiTriStrips(NiTriStrips): NiTriBasedGeom => upcast_ni_tri_strips, downcast_ni_tri_strips;
    NiLines(NiLines): NiTriBasedGeom => upcast_ni_lines, downcast_ni_lines;
    /// `BSLODTriShape`.
    BsLodTriShape(BSLODTriShape): NiTriBasedGeom => upcast_bs_lod_tri_shape, downcast_bs_lod_tri_shape;
    NiScreenElements(NiScreenElements): NiTriShape => upcast_ni_screen_elements, downcast_ni_screen_elements;
    /// An [`NiTriShape`] split into segments.
    BsSegmentedTriShape(BSSegmentedTriShape): NiTriShape => upcast_bs_segmented_tri_shape, downcast_bs_segmented_tri_shape;
    NiParticles(NiParticles): NiGeometry => upcast_ni_particles, downcast_ni_particles;
    NiAutoNormalParticles(NiAutoNormalParticles): NiParticles => upcast_ni_auto_normal_particles, downcast_ni_auto_normal_particles;
    NiParticleMeshes(NiParticleMeshes): NiParticles => upcast_ni_particle_meshes, downcast_ni_particle_meshes;
    NiRotatingParticles(NiRotatingParticles): NiParticles => upcast_ni_rotating_particles, downcast_ni_rotating_particles;

    /// The vertex data of a shape, from
    /// [`NifFile::geometry_data`](crate::handle::View::geometry_data).
    NiGeometryData(NiGeometryData): NiObject => upcast_ni_geometry_data, downcast_ni_geometry_data;
    NiTriBasedGeomData(NiTriBasedGeomData): NiGeometryData => upcast_ni_tri_based_geom_data, downcast_ni_tri_based_geom_data;
    /// The data of an [`NiTriShape`].
    NiTriShapeData(NiTriShapeData): NiTriBasedGeomData => upcast_ni_tri_shape_data, downcast_ni_tri_shape_data;
    /// The data of an [`NiTriStrips`].
    NiTriStripsData(NiTriStripsData): NiTriBasedGeomData => upcast_ni_tri_strips_data, downcast_ni_tri_strips_data;
    /// The data of an [`NiLines`].
    NiLinesData(NiLinesData): NiGeometryData => upcast_ni_lines_data, downcast_ni_lines_data;
    /// The data of an [`NiScreenElements`].
    NiScreenElementsData(NiScreenElementsData): NiTriShapeData => upcast_ni_screen_elements_data, downcast_ni_screen_elements_data;
    /// The selected mesh of a [`BsGeometry`]. It's part of the shape, not a block of its own.
    BsGeometryMeshData(BSGeometryMeshData): NiGeometryData => upcast_bs_geometry_mesh_data, downcast_bs_geometry_mesh_data;

    /// Extra per-vertex data of an [`NiGeometryData`], from [`View::additional_data`]: an
    /// [`NiAdditionalGeometryData`] or [`BsPackedAdditionalGeometryData`]. Its
    /// [channels](Self::channels) describe where each kind of data is in the type's blocks.
    AdditionalGeomData(AdditionalGeomData): NiObject => upcast_additional_geom_data, downcast_additional_geom_data;
    /// Additional data in [blocks](Self::blocks) of same-size items.
    NiAdditionalGeometryData(NiAdditionalGeometryData): AdditionalGeomData => upcast_ni_additional_geometry_data, downcast_ni_additional_geometry_data;
    /// Additional data in packed [blocks](Self::blocks). Fallout 3 and New Vegas.
    BsPackedAdditionalGeometryData(BSPackedAdditionalGeometryData): AdditionalGeomData => upcast_bs_packed_additional_geometry_data, downcast_bs_packed_additional_geometry_data;
}

fn vec3s(v: Vec<nifly_sys::object_3d::ffi::Vec3>) -> Vec<Vec3> {
    v.into_iter().map(Vec3::from_ffi).collect()
}

/// Joins `lists` for the bridge: the values and each list's length.
fn joined_u16<S: AsRef<[u16]>>(lists: &[S]) -> (Vec<u16>, Vec<usize>) {
    (
        lists
            .iter()
            .flat_map(|l| l.as_ref().iter().copied())
            .collect(),
        lists.iter().map(|l| l.as_ref().len()).collect(),
    )
}

impl NiShape {
    pub(crate) fn as_ffi(&self) -> &ffi::NiShape {
        &self.0
    }
}

/// # Shapes
///
/// Listing, naming and ordering shapes, their geometry data and vertex arrays, and geometry
/// checks.
impl NifFile {
    /// Every shape in the file, in block order, including subclasses such as `BSTriShape`.
    pub fn shapes(&self) -> BlockIter<'_, NiShape> {
        self.blocks_of::<NiShape>()
    }

    /// The name of every shape, in block order. Names can repeat; see
    /// [`rename_duplicate_shapes`](Self::rename_duplicate_shapes).
    pub fn shape_names(&self) -> impl Iterator<Item = &BStr> {
        self.shapes().map(|s| s.name())
    }

    /// Renames shapes whose name another child of the same node already has, by appending
    /// `_1`, `_2`, …. Returns whether any shape was renamed. Run by
    /// [`optimize_for`](Self::optimize_for).
    ///
    /// Only the root node and its direct child nodes are checked; shapes deeper in the tree keep
    /// their names. In each node, the first shape child is never renamed, and later ones are
    /// renamed even if the name they share belongs to a node rather than a shape.
    pub fn rename_duplicate_shapes(&mut self) -> bool {
        nif_ffi::NifFile::RenameDuplicateShapes(self.inner.pin_mut())
    }

    /// Reorders the root node's shapes to match `order`, a list of shape names, then sorts all
    /// blocks as [`pretty_sort_blocks`](Self::pretty_sort_blocks) does. Block indices change.
    ///
    /// Returns `false`, changing nothing, unless every shape is a direct child of the root,
    /// `order` names each exactly once, shape names are unique and the file has no
    /// [unknown blocks](Self::has_unknown).
    pub fn set_shape_order<N: AsRef<[u8]>>(&mut self, order: impl IntoIterator<Item = N>) -> bool {
        let Some(indices) = self.shape_order_indices(order) else {
            return false;
        };
        nif_ffi::set_shape_order(self.inner.pin_mut(), &indices);
        true
    }

    /// Block indices of the shapes named in `order`, if `set_shape_order` can apply it.
    fn shape_order_indices<N: AsRef<[u8]>>(
        &self,
        order: impl IntoIterator<Item = N>,
    ) -> Option<Vec<u32>> {
        if self.has_unknown() {
            return None;
        }
        let root = self.root_node()?;
        let root_children: HashSet<_> = root.children().indices().collect();

        let mut seen = HashSet::new();
        let mut indices = Vec::new();
        for name in order {
            let shape = self.find_block_by_name::<NiShape>(name)?;
            let index = self.block_index(shape)?;
            if !root_children.contains(&index) || !seen.insert(index) {
                return None;
            }
            indices.push(block_index_to_ffi(index)?);
        }

        // Every shape is listed. Since each name found a different shape, the names are unique.
        (!indices.is_empty() && indices.len() == self.shapes().count()).then_some(indices)
    }

    /// Check if all shapes are compatible with SSE (no strips in geometry or skin partition).
    /// See [`is_sse_compatible`](View::is_sse_compatible) for a single shape.
    pub fn is_all_sse_compatible(&self) -> bool {
        nif_ffi::NifFile::IsSSECompatible(&self.inner)
    }

    /// Adds an unskinned shape named `name` with this geometry as a child of the root node, with
    /// a default shader and texture set where the version has them. `triangles` index into
    /// `vertices`; `uvs` and `normals`, if given, have one entry per vertex. Without them the
    /// shape has no normals (see [`calc_normals`](Edit::calc_normals)), and no UVs
    /// except on a [`BsTriShape`], which always stores them (as zeros).
    ///
    /// | Version | Shape | Shading |
    /// |---|---|---|
    /// | Skyrim SE | [`BsTriShape`] | `BSLightingShaderProperty` |
    /// | Fallout 4 and 76 | [`BsSubIndexTriShape`] | `BSLightingShaderProperty` |
    /// | Skyrim LE | [`NiTriShape`] | `BSLightingShaderProperty` |
    /// | Fallout 3 and New Vegas | [`NiTriShape`] | `BSShaderPPLightingProperty` |
    /// | Morrowind | [`NiTriShape`] | `NiMaterialProperty`, `NiTexturingProperty` |
    ///
    /// Other versions aren't supported:
    ///
    /// - **Oblivion** shades shapes with an `NiMaterialProperty` and `NiTexturingProperty`, like
    ///   Morrowind, but this would give it the Fallout 3 shader, which Oblivion doesn't have.
    /// - **Starfield** keeps geometry in [`BsGeometry`] shapes whose mesh data lives in separate
    ///   `.mesh` files, which this doesn't create.
    ///
    /// New blocks are added at the end, so existing block indices don't change. Returns the
    /// shape's index, or `None`, changing nothing, if the version isn't supported, the file has no
    /// root node, there are more vertices than [`vertex_limit`](Self::vertex_limit) or more
    /// triangles than the shape can save, a triangle index isn't below the vertex count, or `uvs`
    /// or `normals` has a different length.
    pub fn create_shape(
        &mut self,
        name: impl AsRef<[u8]>,
        vertices: &[Vec3],
        triangles: &[[u16; 3]],
        uvs: Option<&[[f32; 2]]>,
        normals: Option<&[Vec3]>,
    ) -> Option<Index> {
        // Empty lists mean "none" across the bridge, so an empty given list must match no vertices.
        if uvs.is_some_and(|u| u.len() != vertices.len())
            || normals.is_some_and(|n| n.len() != vertices.len())
        {
            return None;
        }
        let vertices: Vec<_> = vertices.iter().map(|v| v.to_ffi()).collect();
        let triangles: Vec<_> = triangles
            .iter()
            .map(|&[p1, p2, p3]| nif_ffi::Triangle { p1, p2, p3 })
            .collect();
        let uvs: Vec<_> = uvs
            .unwrap_or_default()
            .iter()
            .map(|&[u, v]| nif_ffi::Uv { u, v })
            .collect();
        let normals: Vec<_> = normals
            .unwrap_or_default()
            .iter()
            .map(|n| n.to_ffi())
            .collect();
        index_from_ffi(nif_ffi::create_shape(
            self.inner.pin_mut(),
            name.as_ref(),
            &vertices,
            &triangles,
            &uvs,
            &normals,
        ))
    }

    /// Copies `shape`, usually from another file, into this file, as [`Edit::clone_shape`] does,
    /// as a child of the root node.
    ///
    /// Bones are matched by name. Every node below the source's root that this file has no node
    /// of that name for is copied in, under the node named like its source parent, or the root.
    /// A node this file has under a different parent than in the source is moved there, with the
    /// source's transform to its parent. The source file isn't changed.
    ///
    /// Returns the copy's index, or `None`, changing nothing, unless both files are from the
    /// same game. Skyrim LE and SE are different games, Fallout 3 and New Vegas one; files of
    /// other versions must have the same [version](Self::version). Between versions of a game,
    /// the copy is saved in this file's version, so fields it doesn't store are dropped, and
    /// unknown blocks are copied as they are.
    pub fn clone_shape_from(
        &mut self,
        shape: View<'_, NiShape>,
        name: impl AsRef<[u8]>,
    ) -> Option<Index> {
        let source = shape.file();
        index_from_ffi(nif_ffi::clone_shape_from(
            self.inner.pin_mut(),
            &source.inner,
            shape.get().as_ffi(),
            name.as_ref(),
        ))
    }

    fn set_vertex_array<T>(
        &mut self,
        shape: Index,
        values: &[T],
        set: fn(Pin<&mut nif_ffi::NifFile>, u32, &[T]) -> bool,
    ) -> bool {
        block_index_to_ffi(shape).is_some_and(|shape| set(self.inner.pin_mut(), shape, values))
    }

    fn vertex_array<T>(
        &self,
        shape: &NiShape,
        get: fn(&nif_ffi::NifFile, &ffi::NiShape, &mut Vec<T>) -> bool,
    ) -> Option<Vec<T>> {
        debug_assert!(
            self.contains(shape),
            "shape does not belong to this NifFile"
        );

        let mut out = Vec::new();
        get(&self.inner, shape.as_ffi(), &mut out).then_some(out)
    }

    /// Links each `NiGeometry` to its `NiGeometryData` block. Doesn't affect `BSTriShape` blocks.
    pub fn link_geom_data(&mut self) {
        nif_ffi::NifFile::LinkGeomData(self.inner.pin_mut())
    }

    /// Removes triangles with vertex indices that don't exist.
    pub fn remove_invalid_tris(&mut self) {
        nif_ffi::NifFile::RemoveInvalidTris(&self.inner)
    }

    /// The most vertices a shape can have: 65535, in every version.
    pub fn vertex_limit() -> usize {
        nif_ffi::NifFile::GetVertexLimit()
    }

    /// The most triangles a shape can have in this file's version: 65535 before Fallout 4,
    /// 4294967295 from Fallout 4 on.
    pub fn triangle_limit(&self) -> usize {
        nif_ffi::NifFile::GetTriangleLimit(&self.inner)
    }
}

/// Whether every component is under 0.0001 in magnitude, the threshold for an unchanged vertex.
fn nearly_zero(components: &[f32]) -> bool {
    components.iter().all(|c| c.abs() < 1e-4)
}

mod additional_data;
mod bs_geometry;
mod bs_shapes;
mod data;
mod shape_edit;
mod shape_view;

pub use self::additional_data::*;
pub use self::bs_geometry::*;
pub use self::bs_shapes::*;
pub use self::data::*;
