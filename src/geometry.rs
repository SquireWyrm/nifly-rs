//! `NiShape`, the base class of all geometry, and its subclasses.

use std::{collections::HashSet, io, pin::Pin};

use bitflags::bitflags;
use bstr::{BStr, BString, ByteSlice};
use int_enum::IntEnum;
use nifly_sys::{
    geometry::ffi,
    nif_file::ffi::{self as nif_ffi, RefKind},
    read_bridge::RustReader,
};

use crate::{
    error::{Error, LoadError, NifResult},
    file::{block_index_to_ffi, index_from_ffi, Index, NifFile},
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
        Self { smooth: true, smooth_threshold: 60.0, force_model_space: false }
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

impl NiGeometryData {
    pub fn vertex_count(&self) -> usize {
        ffi::geometry_data_vertex_count(&self.0) as usize
    }

    /// The number of triangles. Strips count each triangle they make, including degenerate
    /// ones; lines have none.
    pub fn triangle_count(&self) -> usize {
        ffi::geometry_data_triangle_count(&self.0) as usize
    }

    /// Whether vertex positions are stored.
    pub fn has_vertices(&self) -> bool {
        ffi::geometry_data_has_vertices(&self.0)
    }

    pub fn has_normals(&self) -> bool {
        ffi::geometry_data_has_normals(&self.0)
    }

    pub fn has_vertex_colors(&self) -> bool {
        ffi::geometry_data_has_vertex_colors(&self.0)
    }

    /// Whether texture coordinates are stored.
    pub fn has_uvs(&self) -> bool {
        ffi::geometry_data_has_uvs(&self.0)
    }

    /// Whether tangents and bitangents are stored.
    pub fn has_tangents(&self) -> bool {
        ffi::geometry_data_has_tangents(&self.0)
    }

    /// The sphere enclosing the vertices, as stored.
    pub fn bounds(&self) -> BoundingSphere {
        BoundingSphere::from_ffi(ffi::geometry_data_bounds(&self.0))
    }

    /// Sets the bounds. Saving with [`optimize`](crate::file::SaveOptions::optimize) on, the
    /// default, recalculates them as [`update_bounds`](Self::update_bounds) does.
    pub fn set_bounds(self: Pin<&mut Self>, bounds: BoundingSphere) {
        ffi::set_geometry_data_bounds(self.ffi_mut(), bounds.to_ffi());
    }

    /// Recalculates the [bounds](Self::bounds) from the vertices.
    pub fn update_bounds(self: Pin<&mut Self>) {
        ffi::update_geometry_data_bounds(self.ffi_mut());
    }

    /// How often the game may change the data, or `None` if the stored value isn't one of
    /// [`Consistency`]'s.
    pub fn consistency(&self) -> Option<Consistency> {
        Consistency::try_from(ffi::geometry_data_consistency(&self.0)).ok()
    }

    /// Sets how often the game may change the data. Files before version 10.0.1.0 (such as
    /// Morrowind's) and [`BsGeometryMeshData`] don't store it, so it's lost on save.
    pub fn set_consistency(self: Pin<&mut Self>, consistency: Consistency) {
        ffi::set_geometry_data_consistency(self.ffi_mut(), consistency.into());
    }

    /// The vertex positions. Particle data stores no arrays from version 20.2.0.7, so they're
    /// empty there.
    pub fn vertices(&self) -> Vec<Vec3> {
        vec3s(ffi::geometry_data_vectors(&self.0, 0))
    }

    /// The normals, or none.
    pub fn normals(&self) -> Vec<Vec3> {
        vec3s(ffi::geometry_data_vectors(&self.0, 1))
    }

    /// The tangents, or none.
    pub fn tangents(&self) -> Vec<Vec3> {
        vec3s(ffi::geometry_data_vectors(&self.0, 2))
    }

    /// The bitangents, or none.
    pub fn bitangents(&self) -> Vec<Vec3> {
        vec3s(ffi::geometry_data_vectors(&self.0, 3))
    }

    /// The vertex colors as RGBA, or none.
    pub fn vertex_colors(&self) -> Vec<[f32; 4]> {
        ffi::geometry_data_colors(&self.0).as_chunks::<4>().0.to_vec()
    }

    /// The texture coordinate sets.
    pub fn uv_sets(&self) -> Vec<Vec<[f32; 2]>> {
        (0..ffi::geometry_data_uv_set_count(&self.0))
            .map(|i| ffi::geometry_data_uv_set(&self.0, i).as_chunks::<2>().0.to_vec())
            .collect()
    }

    fn set_vectors(self: Pin<&mut Self>, which: u8, values: &[Vec3]) -> bool {
        let values: Vec<_> = values.iter().map(|v| v.to_ffi()).collect();
        ffi::set_geometry_data_vectors(self.ffi_mut(), which, &values)
    }

    /// Replaces the vertex positions. Returns `false`, changing nothing, unless there's one per
    /// vertex; the vertex count changes through the shape's
    /// [`set_vertices`](crate::handle::Edit::set_vertices).
    pub fn set_vertices(self: Pin<&mut Self>, vertices: &[Vec3]) -> bool {
        self.set_vectors(0, vertices)
    }

    /// Replaces the normals, adding them if the data has none. Returns `false`, changing
    /// nothing, unless there's one per vertex.
    pub fn set_normals(self: Pin<&mut Self>, normals: &[Vec3]) -> bool {
        self.set_vectors(1, normals)
    }

    /// Replaces the tangents, adding tangents and bitangents if the data has none. Returns
    /// `false`, changing nothing, unless there's one per vertex.
    pub fn set_tangents(self: Pin<&mut Self>, tangents: &[Vec3]) -> bool {
        self.set_vectors(2, tangents)
    }

    /// Replaces the bitangents, adding tangents and bitangents if the data has none. Returns
    /// `false`, changing nothing, unless there's one per vertex.
    pub fn set_bitangents(self: Pin<&mut Self>, bitangents: &[Vec3]) -> bool {
        self.set_vectors(3, bitangents)
    }

    /// Replaces the vertex colors, adding them if the data has none. Returns `false`, changing
    /// nothing, unless there's one per vertex.
    pub fn set_vertex_colors(self: Pin<&mut Self>, colors: &[[f32; 4]]) -> bool {
        ffi::set_geometry_data_colors(self.ffi_mut(), colors.as_flattened())
    }

    /// Adds zeroed normals, keeping any it has, or removes them.
    pub fn set_has_normals(self: Pin<&mut Self>, enable: bool) {
        ffi::set_geometry_data_has(self.ffi_mut(), 0, enable);
    }

    /// Adds white vertex colors, keeping any it has, or removes them.
    pub fn set_has_vertex_colors(self: Pin<&mut Self>, enable: bool) {
        ffi::set_geometry_data_has(self.ffi_mut(), 1, enable);
    }

    /// Adds tangents and bitangents, keeping any it has, or removes them. They're saved only
    /// with normals.
    pub fn set_has_tangents(self: Pin<&mut Self>, enable: bool) {
        ffi::set_geometry_data_has(self.ffi_mut(), 2, enable);
    }

    /// The data flags: the UV set count (bits 0–5, or bit 0 from Fallout 3 on) and the tangent
    /// method (bits 12–15). Set with [`Edit::set_data_raw_flags`](crate::handle::Edit::set_data_raw_flags).
    pub fn data_raw_flags(&self) -> u16 {
        ffi::geometry_data_flags(&self.0)
    }

    /// The data's group ID, saved in files from version 10.1.0.114.
    pub fn geometry_group_id(&self) -> i32 {
        ffi::geometry_data_group_id(&self.0)
    }

    pub fn set_geometry_group_id(self: Pin<&mut Self>, id: i32) {
        ffi::set_geometry_data_group_id(self.ffi_mut(), id);
    }

    /// Saved in files from version 10.1.0.0.
    pub fn keep_raw_flags(&self) -> u8 {
        ffi::geometry_data_byte(&self.0, 0)
    }

    pub fn set_keep_raw_flags(self: Pin<&mut Self>, flags: u8) {
        ffi::set_geometry_data_byte(self.ffi_mut(), 0, flags);
    }

    /// Saved in files from version 10.1.0.0.
    pub fn compress_raw_flags(&self) -> u8 {
        ffi::geometry_data_byte(&self.0, 1)
    }

    pub fn set_compress_raw_flags(self: Pin<&mut Self>, flags: u8) {
        ffi::set_geometry_data_byte(self.ffi_mut(), 1, flags);
    }

    /// Saved only in version 20.2.0.7 files after Fallout 3 (stream above 34).
    pub fn material_crc(&self) -> u32 {
        ffi::geometry_data_material_crc(&self.0)
    }

    pub fn set_material_crc(self: Pin<&mut Self>, crc: u32) {
        ffi::set_geometry_data_material_crc(self.ffi_mut(), crc);
    }
}

/// Joins `lists` for the bridge: the values and each list's length.
fn joined_u16<S: AsRef<[u16]>>(lists: &[S]) -> (Vec<u16>, Vec<usize>) {
    (lists.iter().flat_map(|l| l.as_ref().iter().copied()).collect(), lists.iter().map(|l| l.as_ref().len()).collect())
}

impl NiTriShapeData {
    /// The match groups: lists of vertices sharing a normal.
    pub fn match_groups(&self) -> Vec<Vec<u16>> {
        (0..ffi::tri_shape_data_match_group_count(&self.0))
            .map(|i| ffi::tri_shape_data_match_group(&self.0, i).to_vec())
            .collect()
    }

    /// Replaces the match groups. Returns `false`, changing nothing, if there are more than
    /// 65535 groups or vertices in one, or a vertex is out of range.
    pub fn set_match_groups<S: AsRef<[u16]>>(self: Pin<&mut Self>, groups: &[S]) -> bool {
        let (flat, lengths) = joined_u16(groups);
        ffi::set_tri_shape_data_match_groups(self.ffi_mut(), &flat, &lengths)
    }
}

impl NiTriStripsData {
    /// The triangle strips, as vertex indices.
    pub fn strips(&self) -> Vec<Vec<u16>> {
        (0..ffi::tri_strips_data_strip_count(&self.0)).map(|i| ffi::tri_strips_data_strip(&self.0, i).to_vec()).collect()
    }

    /// Replaces the strips, updating the triangle count. Returns `false`, changing nothing, if
    /// there are more than 65535 strips, vertices in one or triangles in all, or a vertex is
    /// out of range.
    pub fn set_strips<S: AsRef<[u16]>>(self: Pin<&mut Self>, strips: &[S]) -> bool {
        let (flat, lengths) = joined_u16(strips);
        ffi::set_tri_strips_data_strips(self.ffi_mut(), &flat, &lengths)
    }
}

/// The material and shader fields of an [`NiGeometry`]. Files from version 20.2.0.5 save the
/// materials, files from 10.0.1.0 to 20.2.0.4 the shader.
impl NiGeometry {
    /// The materials: each name with its extra data.
    pub fn materials(&self) -> Vec<(&BStr, u32)> {
        let extra = ffi::geometry_material_extra_data(&self.0);
        (0..ffi::geometry_material_count(&self.0))
            .map(|i| (ffi::geometry_material_name(&self.0, i).as_bstr(), extra.get(i).copied().unwrap_or(0)))
            .collect()
    }

    pub fn set_materials<N: AsRef<[u8]>>(self: Pin<&mut Self>, materials: &[(N, u32)]) {
        let (bytes, lengths) = crate::extra_data::joined(materials.iter().map(|(name, _)| name));
        let extra: Vec<u32> = materials.iter().map(|&(_, e)| e).collect();
        ffi::set_geometry_materials(self.ffi_mut(), &bytes, &lengths, &extra);
    }

    /// The index of the active material.
    pub fn active_material(&self) -> i32 {
        ffi::geometry_active_material(&self.0)
    }

    pub fn set_active_material(self: Pin<&mut Self>, index: i32) {
        ffi::set_geometry_active_material(self.ffi_mut(), index);
    }

    /// Whether the default material needs updating. Saved in files from version 20.2.0.7.
    pub fn default_material_update_flag(&self) -> u8 {
        ffi::geometry_material_update_flag(&self.0)
    }

    pub fn set_default_material_update_flag(self: Pin<&mut Self>, flag: u8) {
        ffi::set_geometry_material_update_flag(self.ffi_mut(), flag);
    }

    /// The shader's name, or `None` if there's no shader.
    pub fn shader_name(&self) -> Option<&BStr> {
        ffi::geometry_has_shader(&self.0).then(|| ffi::geometry_shader_name(&self.0).as_bstr())
    }

    /// Sets the shader's name, or removes the shader with `None`.
    pub fn set_shader_name(self: Pin<&mut Self>, name: Option<impl AsRef<[u8]>>) {
        match name {
            Some(name) => ffi::set_geometry_shader(self.ffi_mut(), true, name.as_ref()),
            None => ffi::set_geometry_shader(self.ffi_mut(), false, &[]),
        }
    }

    /// The shader's implementation, saved only with a shader.
    pub fn shader_implementation(&self) -> u32 {
        ffi::geometry_shader_implementation(&self.0)
    }

    pub fn set_shader_implementation(self: Pin<&mut Self>, implementation: u32) {
        ffi::set_geometry_shader_implementation(self.ffi_mut(), implementation);
    }
}

/// How often the game may change an [`NiGeometryData`]'s vertex data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u16)]
pub enum Consistency {
    /// May change. The default for new data.
    Mutable = 0x0000,
    /// Never changes.
    Static = 0x4000,
    /// Changes every frame.
    Volatile = 0x8000,
}

/// A channel of an [`AdditionalGeomData`]: one kind of per-vertex data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DataChannel {
    /// What the data is.
    pub data_type: u32,
    /// Bytes per vertex.
    pub unit_size: u32,
    /// Bytes for all vertices: `unit_size` times the vertex count.
    pub total_size: u32,
    /// Bytes per vertex of all the channels in the same block.
    pub stride: u32,
    /// The block holding the channel, by position.
    pub block: u32,
    /// Where the channel starts in each vertex's bytes.
    pub offset: u32,
    /// Bit 0 keeps the data; bits 1–2 are how often it changes (0 may change, 1 never, 2
    /// every frame).
    pub raw_flags: u8,
}

impl Default for DataChannel {
    /// All zero, except raw flags of 2: data that never changes.
    fn default() -> Self {
        Self { data_type: 0, unit_size: 0, total_size: 0, stride: 0, block: 0, offset: 0, raw_flags: 2 }
    }
}

impl DataChannel {
    fn from_ffi(c: ffi::AdditionalDataChannel) -> Self {
        Self {
            data_type: c.data_type as u32,
            unit_size: c.unit_size,
            total_size: c.total_size,
            stride: c.stride,
            block: c.block,
            offset: c.offset,
            raw_flags: c.flags,
        }
    }

    fn to_ffi(self) -> ffi::AdditionalDataChannel {
        ffi::AdditionalDataChannel {
            data_type: self.data_type as i32,
            unit_size: self.unit_size,
            total_size: self.total_size,
            stride: self.stride,
            block: self.block,
            offset: self.offset,
            flags: self.raw_flags,
        }
    }
}

/// A block of an [`NiAdditionalGeometryData`].
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AdditionalDataBlock {
    /// The size of each item of `data`.
    pub block_size: u32,
    pub block_offsets: Vec<u32>,
    /// One per item of `data`.
    pub data_sizes: Vec<u32>,
    /// Items of `block_size` bytes.
    pub data: Vec<Vec<u8>>,
}

/// A block of a [`BsPackedAdditionalGeometryData`].
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct PackedDataBlock {
    /// `None` if the block has no data.
    pub data: Option<PackedData>,
    pub shader_index: u32,
    pub total_size: u32,
}

/// The data of a [`PackedDataBlock`].
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct PackedData {
    pub block_offsets: Vec<u32>,
    pub atom_sizes: Vec<u32>,
    pub bytes: Vec<u8>,
}

impl AdditionalGeomData {
    /// Usually the vertex count of the [`NiGeometryData`] it belongs to.
    pub fn vertex_count(&self) -> u16 {
        ffi::agd_vertex_count(&self.0)
    }

    /// Sets the [vertex count](Self::vertex_count).
    pub fn set_vertex_count(self: Pin<&mut Self>, count: u16) {
        ffi::set_agd_vertex_count(self.ffi_mut(), count);
    }

    pub fn channels(&self) -> Vec<DataChannel> {
        ffi::agd_channels(&self.0).into_iter().map(DataChannel::from_ffi).collect()
    }

    /// Replaces the [channels](Self::channels). They aren't checked against the blocks.
    pub fn set_channels(self: Pin<&mut Self>, channels: &[DataChannel]) {
        let channels: Vec<_> = channels.iter().map(|c| c.to_ffi()).collect();
        ffi::set_agd_channels(self.ffi_mut(), &channels);
    }
}

impl NiAdditionalGeometryData {
    /// The data blocks, `None` for those without data.
    pub fn blocks(&self) -> Vec<Option<AdditionalDataBlock>> {
        let d = &self.0;
        (0..ffi::agd_block_count(d))
            .map(|i| {
                ffi::agd_block_has_data(d, i).then(|| {
                    let data_sizes = ffi::agd_block_data_sizes(d, i).to_vec();
                    AdditionalDataBlock {
                        block_size: ffi::agd_block_size(d, i),
                        block_offsets: ffi::agd_block_offsets(d, i).to_vec(),
                        data: (0..data_sizes.len()).map(|j| ffi::agd_block_data(d, i, j).to_vec()).collect(),
                        data_sizes,
                    }
                })
            })
            .collect()
    }

    /// Replaces the [blocks](Self::blocks). Returns `false`, changing nothing, if a block's
    /// `data` and `data_sizes` lengths differ or an item of `data` isn't `block_size` bytes.
    pub fn set_blocks(mut self: Pin<&mut Self>, blocks: &[Option<AdditionalDataBlock>]) -> bool {
        let valid = |b: &AdditionalDataBlock| {
            b.data.len() == b.data_sizes.len() && b.data.iter().all(|d| d.len() == b.block_size as usize)
        };
        if !blocks.iter().flatten().all(valid) {
            return false;
        }
        ffi::clear_agd_blocks(self.as_mut().ffi_mut());
        for block in blocks {
            match block {
                Some(b) => ffi::push_agd_block(
                    self.as_mut().ffi_mut(),
                    true,
                    b.block_size,
                    &b.block_offsets,
                    &b.data_sizes,
                    &b.data.concat(),
                ),
                None => ffi::push_agd_block(self.as_mut().ffi_mut(), false, 0, &[], &[], &[]),
            }
        }
        true
    }
}

impl BsPackedAdditionalGeometryData {
    pub fn blocks(&self) -> Vec<PackedDataBlock> {
        let d = &self.0;
        (0..ffi::packed_agd_block_count(d))
            .map(|i| {
                let mut ints = [0; 2];
                ffi::packed_agd_block_ints(d, i, &mut ints);
                PackedDataBlock {
                    data: ffi::packed_agd_block_has_data(d, i).then(|| PackedData {
                        block_offsets: ffi::packed_agd_block_offsets(d, i).to_vec(),
                        atom_sizes: ffi::packed_agd_block_atom_sizes(d, i).to_vec(),
                        bytes: ffi::packed_agd_block_data(d, i).to_vec(),
                    }),
                    shader_index: ints[0],
                    total_size: ints[1],
                }
            })
            .collect()
    }

    /// Replaces the [blocks](Self::blocks).
    pub fn set_blocks(mut self: Pin<&mut Self>, blocks: &[PackedDataBlock]) {
        ffi::clear_packed_agd_blocks(self.as_mut().ffi_mut());
        for b in blocks {
            let empty = PackedData::default();
            let data = b.data.as_ref().unwrap_or(&empty);
            ffi::push_packed_agd_block(
                self.as_mut().ffi_mut(),
                b.data.is_some(),
                &data.block_offsets,
                &data.atom_sizes,
                &data.bytes,
                b.shader_index,
                b.total_size,
            );
        }
    }
}

/// An additional data type that [`Edit::assign_additional_data`] can create.
///
/// This trait is sealed.
pub trait NewAdditionalData: NiType {
    #[doc(hidden)]
    const PACKED: bool;
}

impl NewAdditionalData for NiAdditionalGeometryData {
    const PACKED: bool = false;
}

impl NewAdditionalData for BsPackedAdditionalGeometryData {
    const PACKED: bool = true;
}

impl BsLodTriShape {
    /// The triangle counts of three levels of detail, as `[lod0, lod1, lod2]`.
    pub fn lod_sizes(&self) -> [u32; 3] {
        let mut out = [0; 3];
        ffi::lod_tri_shape_sizes(&self.0, &mut out);
        out
    }

    /// Sets the [LOD sizes](Self::lod_sizes). Not checked against the shape's triangles.
    pub fn set_lod_sizes(self: Pin<&mut Self>, sizes: [u32; 3]) {
        ffi::set_lod_tri_shape_sizes(self.ffi_mut(), &sizes);
    }
}

impl BsMeshLodTriShape {
    /// The triangle counts of three levels of detail, as `[lod0, lod1, lod2]`. Deleting
    /// vertices sets them to `[0, 0, triangle count]`.
    pub fn lod_sizes(&self) -> [u32; 3] {
        let mut out = [0; 3];
        ffi::mesh_lod_tri_shape_sizes(&self.0, &mut out);
        out
    }

    /// Sets the [LOD sizes](Self::lod_sizes). Not checked against the shape's triangles.
    pub fn set_lod_sizes(self: Pin<&mut Self>, sizes: [u32; 3]) {
        ffi::set_mesh_lod_tri_shape_sizes(self.ffi_mut(), &sizes);
    }
}

/// A segment of a [`BsSegmentedTriShape`] or a Skyrim SE [`BsSubIndexTriShape`]: a run of its
/// triangles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct GeometrySegment {
    pub raw_flags: u8,
    /// Where the segment starts in the triangle index list: 3 × its first triangle.
    pub index: u32,
    pub triangle_count: u32,
}

impl GeometrySegment {
    fn from_ffi(s: ffi::GeometrySegment) -> Self {
        Self { raw_flags: s.flags, index: s.index, triangle_count: s.triangle_count }
    }

    fn to_ffi(self) -> ffi::GeometrySegment {
        ffi::GeometrySegment { flags: self.raw_flags, index: self.index, triangle_count: self.triangle_count }
    }
}

impl BsSubIndexTriShape {
    /// The segments Skyrim SE stores. Other versions store [`View::segments`] instead.
    pub fn sse_segments(&self) -> Vec<GeometrySegment> {
        ffi::sub_index_tri_shape_sse_segments(&self.0).into_iter().map(GeometrySegment::from_ffi).collect()
    }

    /// Replaces the [Skyrim SE segments](Self::sse_segments). Returns `false`, changing
    /// nothing, if a segment goes past the shape's triangles.
    pub fn set_sse_segments(self: Pin<&mut Self>, segments: &[GeometrySegment]) -> bool {
        let segments: Vec<_> = segments.iter().map(|s| s.to_ffi()).collect();
        ffi::set_sub_index_tri_shape_sse_segments(self.ffi_mut(), &segments)
    }
}

/// The counts and flags a [`BsGeometry`] stores for one of its meshes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct MeshInfo {
    /// The number of triangle indices: 3 per triangle.
    pub index_count: u32,
    pub vertex_count: u32,
    pub raw_flags: u32,
}

impl BsGeometry {
    /// The sphere enclosing the vertices, as stored.
    pub fn bounds(&self) -> BoundingSphere {
        BoundingSphere::from_ffi(ffi::bs_geometry_bounds(&self.0))
    }

    pub fn set_bounds(self: Pin<&mut Self>, bounds: BoundingSphere) {
        ffi::set_bs_geometry_bounds(self.ffi_mut(), bounds.to_ffi());
    }

    /// The bounding box, as stored.
    pub fn bound_min_max(&self) -> [f32; 6] {
        let mut out = [0.0; 6];
        ffi::bs_geometry_bound_min_max(&self.0, &mut out);
        out
    }

    pub fn set_bound_min_max(self: Pin<&mut Self>, values: [f32; 6]) {
        ffi::set_bs_geometry_bound_min_max(self.ffi_mut(), &values);
    }

    /// Whether the meshes' data is stored in the file (flag `0x200`) rather than in external
    /// `.mesh` files.
    pub fn has_internal_geometry(&self) -> bool {
        self.raw_flags() & 0x200 != 0
    }

    /// Sets where the meshes' data is saved. Mesh data never loaded (see
    /// [`load_mesh`](crate::handle::Edit::load_mesh)) is saved empty when made internal, and
    /// mesh paths are saved empty when made external.
    pub fn set_internal_geometry(self: Pin<&mut Self>, internal: bool) {
        let flags = self.raw_flags() & !0x200 | if internal { 0x200 } else { 0 };
        self.upcast_mut().upcast_mut().set_raw_flags(flags);
    }

    /// The number of meshes: one per level of detail, at most 4.
    pub fn mesh_count(&self) -> usize {
        ffi::bs_geometry_mesh_count(&self.0)
    }

    /// Mesh `index`'s data, or `None` if out of range.
    pub fn mesh(&self, index: usize) -> Option<&BsGeometryMeshData> {
        // SAFETY: null or a mesh this shape owns, living as long as it.
        unsafe { ffi::bs_geometry_mesh(&self.0, index).as_ref() }.map(BsGeometryMeshData::from_ffi)
    }

    /// Like [`mesh`](Self::mesh), for the data's setters.
    pub fn mesh_mut(self: Pin<&mut Self>, index: usize) -> Option<Pin<&mut BsGeometryMeshData>> {
        let mesh = ffi::bs_geometry_mesh_mut(self.ffi_mut(), index);
        // SAFETY: null or a mesh this shape owns, exclusively borrowed with it. The wrapper is
        // `repr(transparent)`, and C++ objects never move.
        unsafe { mesh.cast::<BsGeometryMeshData>().as_mut() }.map(|m| unsafe { Pin::new_unchecked(m) })
    }

    /// The counts and flags stored for mesh `index`, or `None` if out of range. Saving sets
    /// the counts from the mesh's data, if it has vertices.
    pub fn mesh_info(&self, index: usize) -> Option<MeshInfo> {
        let mut out = [0; 3];
        ffi::bs_geometry_mesh_info(&self.0, index, &mut out).then_some(MeshInfo {
            index_count: out[0],
            vertex_count: out[1],
            raw_flags: out[2],
        })
    }

    /// Sets the [mesh info](Self::mesh_info). Returns `false` if `index` is out of range.
    pub fn set_mesh_info(self: Pin<&mut Self>, index: usize, info: MeshInfo) -> bool {
        ffi::set_bs_geometry_mesh_info(self.ffi_mut(), index, &[info.index_count, info.vertex_count, info.raw_flags])
    }

    /// Adds an empty mesh at the end. Returns `false`, changing nothing, if there are already 4.
    pub fn add_mesh(self: Pin<&mut Self>) -> bool {
        ffi::add_bs_geometry_mesh(self.ffi_mut())
    }
}

/// One bone's weight for a vertex of a [`BsGeometryMeshData`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct MeshWeight {
    /// The bone's position in the shape's bone list.
    pub bone: u16,
    /// The weight as a fraction of 65535.
    pub weight: u16,
}

/// A group of triangles drawn together, from a [`BsGeometryMeshData`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Meshlet {
    pub vertex_count: u32,
    pub vertex_offset: u32,
    pub primitive_count: u32,
    pub primitive_offset: u32,
}

/// The bounding box of a [`Meshlet`].
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct CullData {
    pub center: Vec3,
    pub expand: Vec3,
}

fn triangles_of(flat: &[u16]) -> Vec<[u16; 3]> {
    flat.as_chunks::<3>().0.to_vec()
}

/// Lists that must be empty or have one entry per vertex return `false`, changing nothing,
/// otherwise.
impl BsGeometryMeshData {
    /// The mesh format version. Versions above 2 save nothing else.
    pub fn version(&self) -> u32 {
        ffi::mesh_data_version(&self.0)
    }

    pub fn set_version(self: Pin<&mut Self>, version: u32) {
        ffi::set_mesh_data_version(self.ffi_mut(), version);
    }

    /// Vertex positions are saved as 16-bit fractions of this times 69.969. With 0 or less,
    /// nothing after the triangles is saved.
    pub fn scale(&self) -> f32 {
        ffi::mesh_data_scale(&self.0)
    }

    /// Sets the [scale](Self::scale). Saving uses a larger one, without changing this, when
    /// a position wouldn't fit.
    pub fn set_scale(self: Pin<&mut Self>, scale: f32) {
        ffi::set_mesh_data_scale(self.ffi_mut(), scale);
    }

    pub fn triangles(&self) -> Vec<[u16; 3]> {
        triangles_of(ffi::mesh_data_triangles(&self.0))
    }

    /// Replaces the triangles. Returns `false`, changing nothing, if an index isn't below the
    /// vertex count.
    pub fn set_triangles(self: Pin<&mut Self>, triangles: &[[u16; 3]]) -> bool {
        ffi::set_mesh_data_triangles(self.ffi_mut(), triangles.as_flattened())
    }

    /// Replaces the vertex positions, changing the vertex count. Normals, tangents, UVs,
    /// colors and skin weights that aren't empty are resized to match, new entries zeroed
    /// (colors white). Returns `false`, changing nothing, if a triangle or LOD triangle uses a
    /// vertex past the new count.
    pub fn replace_vertices(self: Pin<&mut Self>, vertices: &[Vec3]) -> bool {
        let vertices: Vec<_> = vertices.iter().map(|v| v.to_ffi()).collect();
        ffi::replace_mesh_data_vertices(self.ffi_mut(), &vertices)
    }

    pub fn uvs(&self) -> Vec<[f32; 2]> {
        ffi::mesh_data_uvs(&self.0, 0).as_chunks::<2>().0.to_vec()
    }

    pub fn set_uvs(self: Pin<&mut Self>, uvs: &[[f32; 2]]) -> bool {
        ffi::set_mesh_data_uvs(self.ffi_mut(), 0, uvs.as_flattened())
    }

    pub fn second_uvs(&self) -> Vec<[f32; 2]> {
        ffi::mesh_data_uvs(&self.0, 1).as_chunks::<2>().0.to_vec()
    }

    pub fn set_second_uvs(self: Pin<&mut Self>, uvs: &[[f32; 2]]) -> bool {
        ffi::set_mesh_data_uvs(self.ffi_mut(), 1, uvs.as_flattened())
    }

    /// The vertex colors saved, as RGBA bytes.
    pub fn colors(&self) -> Vec<[u8; 4]> {
        ffi::mesh_data_colors(&self.0).as_chunks::<4>().0.to_vec()
    }

    pub fn set_colors(self: Pin<&mut Self>, colors: &[[u8; 4]]) -> bool {
        ffi::set_mesh_data_colors(self.ffi_mut(), colors.as_flattened())
    }

    /// Each tangent's 2-bit W component, the bitangent's sign.
    pub fn tangent_ws(&self) -> Vec<u8> {
        ffi::mesh_data_tangent_ws(&self.0).to_vec()
    }

    /// Replaces the [tangent W components](Self::tangent_ws). Returns `false`, changing
    /// nothing, unless there's one per tangent, each at most 3.
    pub fn set_tangent_ws(self: Pin<&mut Self>, ws: &[u8]) -> bool {
        ffi::set_mesh_data_tangent_ws(self.ffi_mut(), ws)
    }

    /// Each vertex's bone weights, the same number for every vertex.
    pub fn skin_weights(&self) -> Vec<Vec<MeshWeight>> {
        let per = ffi::mesh_data_weights_per_vertex(&self.0) as usize;
        let flat = ffi::mesh_data_skin_weights(&self.0);
        if per == 0 {
            return Vec::new();
        }
        flat.as_chunks::<2>()
            .0
            .chunks(per)
            .map(|v| v.iter().map(|&[bone, weight]| MeshWeight { bone, weight }).collect())
            .collect()
    }

    /// Replaces the [skin weights](Self::skin_weights). Returns `false`, changing nothing,
    /// unless it's empty or has a list per vertex, all the same, nonzero length.
    pub fn set_skin_weights<S: AsRef<[MeshWeight]>>(self: Pin<&mut Self>, weights: &[S]) -> bool {
        let per = weights.first().map_or(0, |w| w.as_ref().len());
        if weights.iter().any(|w| w.as_ref().len() != per) {
            return false;
        }
        let flat: Vec<u16> = weights.iter().flat_map(|w| w.as_ref().iter().flat_map(|w| [w.bone, w.weight])).collect();
        ffi::set_mesh_data_skin_weights(self.ffi_mut(), &flat, per as u32)
    }

    /// Lower-detail triangle lists over the same vertices.
    pub fn lods(&self) -> Vec<Vec<[u16; 3]>> {
        (0..ffi::mesh_data_lod_count(&self.0)).map(|i| triangles_of(ffi::mesh_data_lod(&self.0, i))).collect()
    }

    /// Replaces the [LODs](Self::lods). Returns `false`, changing nothing, if an index isn't
    /// below the vertex count.
    pub fn set_lods<S: AsRef<[[u16; 3]]>>(self: Pin<&mut Self>, lods: &[S]) -> bool {
        let flat: Vec<u16> = lods.iter().flat_map(|l| l.as_ref().as_flattened().iter().copied()).collect();
        let lengths: Vec<usize> = lods.iter().map(|l| l.as_ref().len() * 3).collect();
        ffi::set_mesh_data_lods(self.ffi_mut(), &flat, &lengths)
    }

    pub fn meshlets(&self) -> Vec<Meshlet> {
        ffi::mesh_data_meshlets(&self.0)
            .as_chunks::<4>()
            .0
            .iter()
            .map(|&[vertex_count, vertex_offset, primitive_count, primitive_offset]| Meshlet {
                vertex_count,
                vertex_offset,
                primitive_count,
                primitive_offset,
            })
            .collect()
    }

    /// Replaces the [meshlets](Self::meshlets). Not checked against the triangles; see
    /// [`generate_meshlets`](Self::generate_meshlets).
    pub fn set_meshlets(self: Pin<&mut Self>, meshlets: &[Meshlet]) {
        let flat: Vec<u32> =
            meshlets.iter().flat_map(|m| [m.vertex_count, m.vertex_offset, m.primitive_count, m.primitive_offset]).collect();
        ffi::set_mesh_data_meshlets(self.ffi_mut(), &flat);
    }

    /// Each meshlet's bounding box.
    pub fn cull_data(&self) -> Vec<CullData> {
        vec3s(ffi::mesh_data_cull_data(&self.0))
            .as_chunks::<2>()
            .0
            .iter()
            .map(|&[center, expand]| CullData { center, expand })
            .collect()
    }

    pub fn set_cull_data(self: Pin<&mut Self>, cull_data: &[CullData]) {
        let flat: Vec<_> = cull_data.iter().flat_map(|c| [c.center.to_ffi(), c.expand.to_ffi()]).collect();
        ffi::set_mesh_data_cull_data(self.ffi_mut(), &flat);
    }

    /// Replaces the meshlets and cull data with new ones covering the triangles, each with at
    /// most `max_vertices` vertices (at least 3) and `max_primitives` triangles (at least 1).
    /// Returns `false`, changing nothing, if there are no triangles or vertices.
    pub fn generate_meshlets(self: Pin<&mut Self>, max_vertices: u32, max_primitives: u32) -> bool {
        ffi::generate_mesh_data_meshlets(self.ffi_mut(), max_vertices, max_primitives)
    }
}

/// The particle data of a Skyrim SE [`BsTriShape`]: a copy of its geometry, stored at half
/// precision.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ParticleData {
    /// One per vertex.
    pub vertices: Vec<Vec3>,
    /// One per vertex.
    pub normals: Vec<Vec3>,
    /// One per triangle.
    pub triangles: Vec<[u16; 3]>,
}

bitflags! {
    /// The vertex attributes a [`BsTriShape`] stores. Other bits are kept as stored.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct VertexFlags: u16 {
        const POSITIONS = 1;
        const UVS = 1 << 1;
        /// Second UVs, which aren't read or saved, only flagged.
        const UVS_2 = 1 << 2;
        const NORMALS = 1 << 3;
        const TANGENTS = 1 << 4;
        const COLORS = 1 << 5;
        const SKINNED = 1 << 6;
        /// Land data, which isn't read or saved, only flagged.
        const LAND_DATA = 1 << 7;
        const EYE_DATA = 1 << 8;
        const INSTANCE = 1 << 9;
        /// Positions saved as 32-bit floats rather than 16-bit.
        const FULL_PRECISION = 1 << 10;
        const _ = !0;
    }
}

impl BsTriShape {
    /// The sphere enclosing the vertices, as stored.
    pub fn bounds(&self) -> BoundingSphere {
        BoundingSphere::from_ffi(ffi::bs_tri_shape_bounds(&self.0))
    }

    /// Sets the bounds. Saving with [`optimize`](crate::file::SaveOptions::optimize) on, the
    /// default, recalculates them as [`update_bounds`](Self::update_bounds) does.
    pub fn set_bounds(self: Pin<&mut Self>, bounds: BoundingSphere) {
        ffi::set_bs_tri_shape_bounds(self.ffi_mut(), bounds.to_ffi());
    }

    /// Recalculates the [bounds](Self::bounds) from the vertices.
    pub fn update_bounds(self: Pin<&mut Self>) {
        ffi::update_bs_tri_shape_bounds(self.ffi_mut());
    }

    /// The bounding box, as stored: six floats saved from Fallout 76 on.
    pub fn bound_min_max(&self) -> [f32; 6] {
        let mut out = [0.0; 6];
        ffi::bs_tri_shape_bound_min_max(&self.0, &mut out);
        out
    }

    pub fn set_bound_min_max(self: Pin<&mut Self>, values: [f32; 6]) {
        ffi::set_bs_tri_shape_bound_min_max(self.ffi_mut(), &values);
    }

    /// The raw vertex format: the vertex size, each attribute's offset, and the
    /// [flags](Self::vertex_flags) in the top bits. Saving recalculates the size and offsets.
    pub fn vertex_desc(&self) -> u64 {
        ffi::bs_tri_shape_vertex_desc(&self.0)
    }

    /// The vertex attributes stored. Set with [`Edit::set_vertex_flags`].
    pub fn vertex_flags(&self) -> VertexFlags {
        VertexFlags::from_bits_retain(ffi::bs_tri_shape_vertex_flags(&self.0))
    }

    /// Each vertex's extra floats, stored between its position and bitangent.
    pub fn vertex_extra_floats(&self) -> Vec<Vec<f32>> {
        let mut per = 0;
        let flat = ffi::bs_tri_shape_extra_floats(&self.0, &mut per);
        if per == 0 {
            return vec![Vec::new(); ffi::bs_tri_shape_vertex_count(&self.0)];
        }
        flat.chunks_exact(per).map(<[f32]>::to_vec).collect()
    }

    /// The particle data, or `None` if there's none. Only Skyrim SE files store it.
    pub fn particle_data(&self) -> Option<ParticleData> {
        if !ffi::bs_tri_shape_has_particle_data(&self.0) {
            return None;
        }
        Some(ParticleData {
            vertices: vec3s(ffi::bs_tri_shape_particle_vectors(&self.0, false)),
            normals: vec3s(ffi::bs_tri_shape_particle_vectors(&self.0, true)),
            triangles: ffi::bs_tri_shape_particle_triangles(&self.0).as_chunks::<3>().0.to_vec(),
        })
    }

    /// Replaces the [particle data](Self::particle_data), or removes it with `None`. Returns
    /// `false`, changing nothing, unless it has a position and normal per vertex and a
    /// triangle per triangle, using only the shape's vertices.
    pub fn set_particle_data(self: Pin<&mut Self>, data: Option<&ParticleData>) -> bool {
        let Some(data) = data else {
            return ffi::set_bs_tri_shape_particle_data(self.ffi_mut(), false, &[], &[], &[]);
        };
        let vertices: Vec<_> = data.vertices.iter().map(|v| v.to_ffi()).collect();
        let normals: Vec<_> = data.normals.iter().map(|v| v.to_ffi()).collect();
        ffi::set_bs_tri_shape_particle_data(self.ffi_mut(), true, &vertices, &normals, data.triangles.as_flattened())
    }
}

/// Changing a [`BsTriShape`]'s vertex format.
impl<T: NiType> Edit<'_, T> {
    fn bs_tri_shape<I>(&mut self) -> (Pin<&mut BsTriShape>, bool)
    where
        T: IsA<BsTriShape, I>,
    {
        let sse = self.view().file().version().stream == 100;
        let index = self.index();
        (self.file_mut().block_of_mut::<BsTriShape>(index).expect("the block is a BSTriShape"), sse)
    }

    /// Sets the [vertex flags](BsTriShape::vertex_flags). Newly added colors start white;
    /// other new attributes start zeroed. Returns `false`, changing nothing, if they add
    /// second UVs, or the vertices couldn't be saved: [extra floats](BsTriShape::vertex_extra_floats)
    /// need full-precision positions (always the case in Skyrim SE), and a vertex is at most
    /// 60 bytes.
    pub fn set_vertex_flags<I>(&mut self, flags: VertexFlags) -> bool
    where
        T: IsA<BsTriShape, I>,
    {
        let (shape, sse) = self.bs_tri_shape();
        ffi::set_bs_tri_shape_vertex_flags(shape.ffi_mut(), flags.bits(), sse)
    }

    /// Replaces each vertex's [extra floats](BsTriShape::vertex_extra_floats). Returns
    /// `false`, changing nothing, unless there's a list per vertex, all the same length, and
    /// the vertices could be saved (see [`set_vertex_flags`](Self::set_vertex_flags)).
    pub fn set_vertex_extra_floats<S: AsRef<[f32]>, I>(&mut self, extra: &[S]) -> bool
    where
        T: IsA<BsTriShape, I>,
    {
        let per = extra.first().map_or(0, |e| e.as_ref().len());
        if extra.iter().any(|e| e.as_ref().len() != per) {
            return false;
        }
        let flat: Vec<f32> = extra.iter().flat_map(|e| e.as_ref().iter().copied()).collect();
        let (shape, sse) = self.bs_tri_shape();
        ffi::set_bs_tri_shape_extra_floats(shape.ffi_mut(), &flat, per, sse)
    }
}

impl BsSegmentedTriShape {
    pub fn segments(&self) -> Vec<GeometrySegment> {
        ffi::segmented_tri_shape_segments(&self.0)
            .into_iter()
            .map(GeometrySegment::from_ffi)
            .collect()
    }

    /// Replaces the segments. Returns `false`, changing nothing, if a segment goes past the
    /// shape's triangles. Segments may overlap or leave triangles out.
    pub fn set_segments(self: Pin<&mut Self>, segments: &[GeometrySegment]) -> bool {
        let segments: Vec<_> = segments.iter().map(|s| s.to_ffi()).collect();
        ffi::set_segmented_tri_shape_segments(self.ffi_mut(), &segments)
    }
}

impl NiLinesData {
    /// For each vertex, whether a line connects it to the next one.
    pub fn line_flags(&self) -> Vec<bool> {
        ffi::lines_data_flags(&self.0)
    }

    /// Sets the [line flags](Self::line_flags). Returns `false`, changing nothing, unless
    /// there's one per vertex.
    pub fn set_line_flags(self: Pin<&mut Self>, flags: &[bool]) -> bool {
        ffi::set_lines_data_flags(self.ffi_mut(), flags)
    }
}

/// A polygon of an [`NiScreenElementsData`]: a run of its vertices and triangles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ScreenPolygon {
    pub vertex_count: u16,
    pub vertex_offset: u16,
    pub triangle_count: u16,
    pub triangle_offset: u16,
}

/// Getter and setter pairs over the bridge's `u16` field shims.
macro_rules! u16_fields {
    ($($(#[$doc:meta])* $get:ident, $set:ident = $ffi_get:ident, $ffi_set:ident;)*) => {$(
        $(#[$doc])*
        pub fn $get(&self) -> u16 {
            ffi::$ffi_get(&self.0)
        }

        #[doc = concat!("Sets [`", stringify!($get), "`](Self::", stringify!($get), ").")]
        pub fn $set(self: Pin<&mut Self>, value: u16) {
            ffi::$ffi_set(self.ffi_mut(), value);
        }
    )*};
}

/// Deleting vertices clears the polygons and polygon indices, and sets the polygon count and
/// the maximum vertex and index counts to 0.
impl NiScreenElementsData {
    /// Every polygon slot, including unused ones after the [polygon
    /// count](Self::polygon_count). Its vertex and triangle ranges aren't checked.
    pub fn polygons(&self) -> Vec<ScreenPolygon> {
        ffi::screen_elements_polygons(&self.0)
            .into_iter()
            .map(|p| ScreenPolygon {
                vertex_count: p.vertex_count,
                vertex_offset: p.vertex_offset,
                triangle_count: p.triangle_count,
                triangle_offset: p.triangle_offset,
            })
            .collect()
    }

    /// The index of each polygon slot.
    pub fn polygon_indices(&self) -> Vec<u16> {
        ffi::screen_elements_polygon_indices(&self.0)
    }

    /// Replaces the polygon slots and their indices. Returns `false`, changing nothing, unless
    /// both have the same length, at most 65535, and the [polygon count](Self::polygon_count)
    /// fits in it.
    pub fn set_polygons(self: Pin<&mut Self>, polygons: &[ScreenPolygon], indices: &[u16]) -> bool {
        if polygons.len() < usize::from(self.polygon_count()) {
            return false;
        }
        let polygons: Vec<_> = polygons
            .iter()
            .map(|p| ffi::ScreenPolygon {
                vertex_count: p.vertex_count,
                vertex_offset: p.vertex_offset,
                triangle_count: p.triangle_count,
                triangle_offset: p.triangle_offset,
            })
            .collect();
        ffi::set_screen_elements_polygons(self.ffi_mut(), &polygons, indices)
    }

    /// The number of polygon slots in use.
    pub fn polygon_count(&self) -> u16 {
        ffi::screen_elements_polygon_count(&self.0)
    }

    /// Sets the [polygon count](Self::polygon_count). Returns `false`, changing nothing, if
    /// it's more than the number of [polygon slots](Self::polygons).
    pub fn set_polygon_count(self: Pin<&mut Self>, count: u16) -> bool {
        if usize::from(count) > self.polygons().len() {
            return false;
        }
        ffi::set_screen_elements_polygon_count(self.ffi_mut(), count);
        true
    }

    u16_fields! {
        /// How many polygon slots to add when growing.
        polygon_grow_by, set_polygon_grow_by = screen_elements_polygon_grow_by, set_screen_elements_polygon_grow_by;
        /// The vertex capacity.
        max_vertices, set_max_vertices = screen_elements_max_vertices, set_screen_elements_max_vertices;
        /// How many vertices to add when growing.
        vertices_grow_by, set_vertices_grow_by = screen_elements_vertices_grow_by, set_screen_elements_vertices_grow_by;
        /// The triangle index capacity.
        max_indices, set_max_indices = screen_elements_max_indices, set_screen_elements_max_indices;
        /// How many triangle indices to add when growing.
        indices_grow_by, set_indices_grow_by = screen_elements_indices_grow_by, set_screen_elements_indices_grow_by;
    }
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
    fn shape_order_indices<N: AsRef<[u8]>>(&self, order: impl IntoIterator<Item = N>) -> Option<Vec<u32>> {
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
        if uvs.is_some_and(|u| u.len() != vertices.len()) || normals.is_some_and(|n| n.len() != vertices.len()) {
            return None;
        }
        let vertices: Vec<_> = vertices.iter().map(|v| v.to_ffi()).collect();
        let triangles: Vec<_> = triangles.iter().map(|&[p1, p2, p3]| nif_ffi::Triangle { p1, p2, p3 }).collect();
        let uvs: Vec<_> = uvs.unwrap_or_default().iter().map(|&[u, v]| nif_ffi::Uv { u, v }).collect();
        let normals: Vec<_> = normals.unwrap_or_default().iter().map(|n| n.to_ffi()).collect();
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
    pub fn clone_shape_from(&mut self, shape: View<'_, NiShape>, name: impl AsRef<[u8]>) -> Option<Index> {
        let source = shape.file();
        index_from_ffi(nif_ffi::clone_shape_from(self.inner.pin_mut(), &source.inner, shape.get().as_ffi(), name.as_ref()))
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
        debug_assert!(self.contains(shape), "shape does not belong to this NifFile");

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
    }}

/// Reading a shape.
impl<'a> View<'a, NiShape> {
    /// The paths of the external `.mesh` files of a [`BsGeometry`], one per level of detail, as
    /// stored (a name or SHA-1 hash, relative to the game's `geometry` folder, without the
    /// extension). Empty if the shape isn't a [`BsGeometry`] or stores its mesh data in the file.
    pub fn mesh_paths(&self) -> Vec<&'a BStr> {
        let shape = self.get();
        (0..nif_ffi::external_mesh_count(shape.as_ffi()))
            .map(|i| nif_ffi::external_mesh_path(shape.as_ffi(), i).as_bstr())
            .collect()
    }

    /// The shape's vertex data: the [`NiGeometryData`] block of an [`NiTriBasedGeom`] (such as
    /// an [`NiTriShape`] or [`NiTriStrips`]), or the selected mesh of a [`BsGeometry`], which
    /// isn't a block of its own. `None` for other shapes, such as [`BsTriShape`], which stores
    /// its vertices itself, and particles, or if the data ref is empty.
    ///
    /// This is a plain reference rather than a [`View`], since a mesh has no block index.
    /// [`BsGeometry::mesh`] reaches a [`BsGeometry`]'s other meshes.
    pub fn geometry_data(&self) -> Option<&'a NiGeometryData> {
        let ptr = nif_ffi::geometry_data(&self.file().inner, self.get().as_ffi());
        // SAFETY: the data is owned by this file, which outlives `'a`.
        unsafe { ptr.as_ref() }.map(NiGeometryData::from_ffi)
    }


    /// Check if the shape is compatible with Skyrim SE: it isn't an `NiTriStrips`, and its skin
    /// partition (if any) has no triangle strips.
    pub fn is_sse_compatible(&self) -> bool {
        let (file, shape) = (self.file(), self.get());
        nif_ffi::is_shape_sse_compatible(&file.inner, shape.as_ffi())
    }

    /// The vertex positions of the shape, or `None` if it doesn't store them.
    ///
    /// These read the vertex data wherever the shape keeps it: in its [`geometry_data`]
    /// or, for a [`BsTriShape`], in the shape itself, where normals, tangents, bitangents and
    /// colors are stored at reduced precision and returned decoded. Particles return `None`.
    ///
    /// [`geometry_data`]: Self::geometry_data
    pub fn vertices(&self) -> Option<Vec<Vec3>> {
        let (file, shape) = (self.file(), self.get());
        file.vertex_array(shape, nif_ffi::shape_vertices).map(vec3s)
    }

    /// The normals of the shape, one per vertex, or `None` if it doesn't store them. See
    /// [`vertices`](Self::vertices).
    pub fn normals(&self) -> Option<Vec<Vec3>> {
        let (file, shape) = (self.file(), self.get());
        file.vertex_array(shape, nif_ffi::shape_normals).map(vec3s)
    }

    /// The texture coordinates of the shape's first UV set as `[u, v]`, one per vertex, or `None`
    /// if it doesn't store them. See [`vertices`](Self::vertices).
    pub fn uvs(&self) -> Option<Vec<[f32; 2]>> {
        let (file, shape) = (self.file(), self.get());
        let uvs = file.vertex_array(shape, nif_ffi::shape_uvs)?;
        Some(uvs.into_iter().map(|uv| [uv.u, uv.v]).collect())
    }

    /// The vertex colors of the shape as `[r, g, b, a]` in `0.0..=1.0`, one per vertex, or `None`
    /// if it doesn't store them. See [`vertices`](Self::vertices).
    pub fn vertex_colors(&self) -> Option<Vec<[f32; 4]>> {
        let (file, shape) = (self.file(), self.get());
        let colors = file.vertex_array(shape, nif_ffi::shape_colors)?;
        Some(colors.into_iter().map(|c| [c.r, c.g, c.b, c.a]).collect())
    }

    /// The tangents of the shape, one per vertex, or `None` if it doesn't store them. See
    /// [`vertices`](Self::vertices).
    ///
    /// Oblivion files keep tangents and bitangents in an `NiBinaryExtraData` named
    /// "Tangent space (binormal & tangent vectors)". Loading reads them from it, and saving
    /// rewrites it from the shape's tangents, or removes it if the shape has none.
    pub fn tangents(&self) -> Option<Vec<Vec3>> {
        let (file, shape) = (self.file(), self.get());
        file.vertex_array(shape, nif_ffi::shape_tangents).map(vec3s)
    }

    /// The bitangents of the shape, one per vertex, or `None` if it doesn't store them. See
    /// [`vertices`](Self::vertices).
    pub fn bitangents(&self) -> Option<Vec<Vec3>> {
        let (file, shape) = (self.file(), self.get());
        file.vertex_array(shape, nif_ffi::shape_bitangents).map(vec3s)
    }

    /// The eye data of a [`BsTriShape`], one value per vertex, or `None` if it isn't one or
    /// doesn't store any.
    pub fn eye_data(&self) -> Option<Vec<f32>> {
        let (file, shape) = (self.file(), self.get());
        file.vertex_array(shape, nif_ffi::shape_eye_data)
    }

    /// The offsets that move the shape's vertices to `target` (one position per vertex), as
    /// `(vertex, target - position)` in vertex order. Vertices that move less than 0.0001 along
    /// every axis are left out. Returns `None` if the shape has no vertices or `target` has a
    /// different length.
    ///
    /// # Compatibility
    ///
    /// BodySlide and Outfit Studio store slider data as these per-vertex offsets.
    pub fn vertex_diff(&self, target: &[Vec3]) -> Option<Vec<(usize, Vec3)>> {
        let vertices = self.vertices()?;
        (vertices.len() == target.len()).then(|| {
            let diffs = vertices.iter().zip(target).map(|(&v, &t)| t - v);
            diffs.enumerate().filter(|(_, d)| !nearly_zero(&[d.x, d.y, d.z])).collect()
        })
    }

    /// The offsets that move the shape's first UV set to `target` (one `[u, v]` per vertex), as
    /// `(vertex, target - uv)` in vertex order. Like
    /// [`vertex_diff`](Self::vertex_diff), but for UVs; `None` if the shape has no UVs.
    pub fn uv_diff(&self, target: &[[f32; 2]]) -> Option<Vec<(usize, [f32; 2])>> {
        let uvs = self.uvs()?;
        (uvs.len() == target.len()).then(|| {
            let diffs = uvs.iter().zip(target).map(|(uv, t)| [t[0] - uv[0], t[1] - uv[1]]);
            diffs.enumerate().filter(|(_, d)| !nearly_zero(d)).collect()
        })
    }

    /// The triangles of the shape, as three indices into its vertex arrays each, or `None` if it
    /// stores none. Triangle strips are returned as triangles, without degenerate ones. Particles
    /// and lines return `None`.
    pub fn triangles(&self) -> Option<Vec<[u16; 3]>> {
        let shape = self.get();
        let mut out = Vec::new();
        nif_ffi::shape_triangles(shape.as_ffi(), &mut out)
            .then(|| out.into_iter().map(|t| [t.p1, t.p2, t.p3]).collect())
    }

    /// Writes the data of external mesh `lod` (see [`mesh_paths`](Self::mesh_paths))
    /// of the [`BsGeometry`] the shape to `writer`, as the contents of its `.mesh` file. The
    /// counterpart of [`load_mesh`](Edit::load_mesh). Vertex positions are saved as
    /// 16-bit fractions of the mesh's extent, so they read back rounded.
    ///
    /// Returns `Ok(false)`, writing nothing, if the shape has no such external mesh. Writer errors
    /// are returned as [`Error::Io`], and data too large to write as [`Error::Exception`].
    pub fn save_mesh(&self, lod: usize, mut writer: impl io::Write) -> NifResult<bool> {
        let shape = self.get();
        let Ok(lod) = u32::try_from(lod) else {
            return Ok(false);
        };
        let mut bytes = Vec::new();
        if !nif_ffi::save_external_mesh(shape.as_ffi(), lod, &mut bytes)? {
            return Ok(false);
        }
        writer.write_all(&bytes)?;
        Ok(true)
    }

    /// The segments of the shape and the segment each triangle is in, or `None` if it isn't a
    /// [`BsSubIndexTriShape`]. Skyrim SE stores simpler segments, read with
    /// [`BsSubIndexTriShape::sse_segments`]; there this finds none.
    pub fn segments(&self) -> Option<ShapeSegments> {
        let shape = self.get();
        let (mut ssf, mut segs, mut subs, mut extra, mut parts) = (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());
        if !nif_ffi::shape_segments(shape.as_ffi(), &mut ssf, &mut segs, &mut subs, &mut extra, &mut parts) {
            return None;
        }

        // Part IDs number each segment, then its subsegments, in order.
        let mut by_part = Vec::new();
        let mut segments = Vec::with_capacity(segs.len());
        let (mut subs, mut extra) = (subs.into_iter(), extra.into_iter());
        for (segment, seg) in segs.iter().enumerate() {
            by_part.push(SegmentIndex { segment, subsegment: None });
            let subsegments = (0..seg.subsegments as usize)
                .map(|subsegment| {
                    by_part.push(SegmentIndex { segment, subsegment: Some(subsegment) });
                    let sub = subs.next().unwrap_or(nif_ffi::SubsegmentFfi { user_slot: 0, material: 0, extra_len: 0 });
                    Subsegment {
                        user_slot: (sub.user_slot >= 30).then_some(sub.user_slot),
                        material: sub.material,
                        extra_data: extra.by_ref().take(sub.extra_len as usize).collect(),
                    }
                })
                .collect();
            segments.push(Segment { subsegments });
        }
        let triangles = parts.iter().map(|&p| usize::try_from(p).ok().and_then(|p| by_part.get(p).copied())).collect();
        Some(ShapeSegments { ssf_file: BString::from(ssf), segments, triangles })
    }
}

/// Changing a shape.
impl Edit<'_, NiShape> {
    /// Like [`View::geometry_data`], for the data's setters.
    pub fn geometry_data(&mut self) -> Option<Pin<&mut NiGeometryData>> {
        let index = self.index();
        // SAFETY: the data came from this file.
        unsafe { self.file_mut().pin_block(|f| f.view::<NiShape>(index)?.geometry_data()) }
    }

    /// Points an `NiGeometry` shape at the [`NiGeometryData`] at `data`. Returns `false`,
    /// changing nothing, if the shape isn't an `NiGeometry` or the data's type doesn't match it,
    /// such as [`NiTriShapeData`] for an [`NiTriShape`] and its subtypes. The old data stays in
    /// the file until saving drops it, if nothing else uses it.
    pub fn set_geometry_data(&mut self, data: Index) -> bool {
        let (shape, file) = (self.index() as u32, self.file_mut());
        block_index_to_ffi(data).is_some_and(|data| nif_ffi::set_shape_geometry_data(file.inner.pin_mut(), shape, data))
    }

    /// Applies `position` to each vertex and `normal`/`tangent` to its directions, each weighted
    /// by `weights`; directions are renormalized.
    fn transform(
        &mut self,
        weights: Option<&[f32]>,
        position: impl Fn(Vec3) -> Vec3,
        normal: impl Fn(Vec3) -> Vec3,
        tangent: impl Fn(Vec3) -> Vec3,
    ) -> bool {
        let view = self.view();
        let Some(vertices) = view.vertices() else {
            return false;
        };
        if let Some(w) = weights
            && (w.len() != vertices.len() || !w.iter().all(|w| w.is_finite()))
        {
            return false;
        }
        let weight = |i: usize| weights.map_or(1.0, |w| w[i]);
        let blend = |items: Vec<Vec3>, f: &dyn Fn(Vec3) -> Vec3| -> Vec<Vec3> {
            items.into_iter().enumerate().map(|(i, d)| (d + (f(d).normalize() - d) * weight(i)).normalize()).collect()
        };
        let (normals, tangents, bitangents) = (view.normals(), view.tangents(), view.bitangents());

        let moved: Vec<Vec3> = vertices.iter().enumerate().map(|(i, &p)| p + (position(p) - p) * weight(i)).collect();
        if !self.set_vertices(&moved) {
            return false;
        }
        // Same vertex count, so these can't fail.
        if let Some(n) = normals {
            self.set_normals(&blend(n, &normal));
        }
        if let Some(t) = tangents {
            self.set_tangents(&blend(t, &tangent));
        }
        if let Some(b) = bitangents {
            self.set_bitangents(&blend(b, &tangent));
        }
        true
    }


    /// Replaces the triangles of the shape.
    ///
    /// Returns `false`, changing nothing, if the shape has no triangle list (an
    /// [`NiTriShape`] or other shape with [`NiTriShapeData`], a [`BsTriShape`], or a
    /// [`BsGeometry`] mesh; not strips), an index isn't below its vertex count, or there are
    /// more triangles than it can save: 65535 for [`NiTriShapeData`] and
    /// [`triangle_limit`](NifFile::triangle_limit) otherwise.
    ///
    /// Data that refers to the triangles, such as skin partitions (see
    /// [`update_skin_partitions`](Self::update_skin_partitions)) and the segments of a
    /// [`BsSubIndexTriShape`], isn't updated.
    pub fn set_triangles(&mut self, triangles: &[[u16; 3]]) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        let triangles: Vec<_> = triangles
            .iter()
            .map(|&[p1, p2, p3]| nif_ffi::Triangle { p1, p2, p3 })
            .collect();
        block_index_to_ffi(shape)
            .is_some_and(|shape| nif_ffi::set_shape_triangles(file.inner.pin_mut(), shape, &triangles))
    }

    /// Reads the data of external mesh `lod` (see [`mesh_paths`](View::mesh_paths))
    /// of the [`BsGeometry`] from `reader`, the contents of its `.mesh`
    /// file, replacing what the shape had for it. Its vertices, triangles and other vertex data
    /// are then available as for other shapes, from [`geometry_data`](View::geometry_data) and
    /// [`vertices`](View::vertices) for the first mesh.
    ///
    /// Returns `Ok(false)`, changing nothing, if there's no such external mesh. Reader errors are
    /// returned as [`Error::Io`], as is data that ends early ([`io::ErrorKind::UnexpectedEof`]).
    /// A mesh format version above 2 is [`LoadError::UnsupportedVersion`], and other malformed
    /// data [`Error::Exception`]. The mesh is only changed on success.
    pub fn load_mesh(&mut self, lod: usize, mut reader: impl io::Read) -> NifResult<bool> {
        let (shape, file) = (self.index(), self.file_mut());
        let (Some(shape), Ok(lod)) = (block_index_to_ffi(shape), u32::try_from(lod)) else {
            return Ok(false);
        };
        let mut reader = RustReader::new(&mut reader);
        let result = nif_ffi::load_external_mesh(file.inner.pin_mut(), shape, lod, &mut reader);

        // Prefer the reader's own error over the C++ exception it was turned into.
        if let Some(e) = reader.take_error() {
            return Err(Error::Io(e));
        }
        match result? {
            0 => Ok(true),
            1 => Ok(false),
            3 => Err(LoadError::UnsupportedVersion.into()),
            _ => Err(Error::Io(io::Error::new(io::ErrorKind::UnexpectedEof, "mesh data ended early"))),
        }
    }

    /// Sets the path of external mesh `lod` (see [`mesh_paths`](View::mesh_paths)) of
    /// the [`BsGeometry`]. Returns `false`, changing nothing, if there's no
    /// such external mesh.
    pub fn set_mesh_path(&mut self, lod: usize, path: impl AsRef<[u8]>) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        let (Some(shape), Ok(lod)) = (block_index_to_ffi(shape), u32::try_from(lod)) else {
            return false;
        };
        nif_ffi::set_external_mesh_path(file.inner.pin_mut(), shape, lod, path.as_ref())
    }

    /// Converts the [`NiTriStrips`] and its [`NiTriStripsData`] to an
    /// [`NiTriShape`] and [`NiTriShapeData`] with the same geometry, as triangles without the
    /// strips' degenerate ones. Both keep their block indices, and the shape everything else.
    /// A skin partition keeps its own strips; [`update_skin_partitions`](Self::update_skin_partitions)
    /// converts those.
    ///
    /// Returns `false`, changing nothing, if the shape isn't an [`NiTriStrips`] with triangles,
    /// or another block shares its data.
    pub fn triangulate(&mut self) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        block_index_to_ffi(shape).is_some_and(|shape| nif_ffi::triangulate_shape(file.inner.pin_mut(), shape))
    }

    /// Deletes the shape, removing it from its parent's children. The
    /// blocks it refers to, such as its data, shader, properties, skin, extra data, collision and
    /// controllers, are deleted too, along with what they refer to, unless another block still
    /// uses them. Bone nodes stay.
    ///
    /// Deleting blocks moves later blocks down, so block indices change. Returns `false` if that
    /// block isn't a shape.
    pub fn delete(mut self) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        block_index_to_ffi(shape).is_some_and(|shape| nif_ffi::delete_shape(file.inner.pin_mut(), shape))
    }

    /// Copies the shape, with deep copies of the blocks it refers to
    /// (geometry data, shader, skin, properties, extra data), and names the copy `name`. The
    /// copy is a child of the original's parent and uses the same bones. In Skyrim files, a copy
    /// whose shader uses model-space normals has its normals and tangents turned off.
    ///
    /// New blocks are added at the end, so existing block indices don't change. Returns the
    /// copy's index, or `None` if the shape can't be copied.
    pub fn clone_shape(&mut self, name: impl AsRef<[u8]>) -> Option<Index> {
        let (shape, file) = (self.index(), self.file_mut());
        let shape = block_index_to_ffi(shape)?;
        index_from_ffi(nif_ffi::clone_shape(file.inner.pin_mut(), shape, name.as_ref()))
    }

    /// Reorders the triangles of the shape: triangle `i` becomes the old
    /// triangle `order[i]`. The triangles themselves don't change.
    ///
    /// Data that refers to triangle positions isn't updated: the segments of a
    /// [`BsSubIndexTriShape`] (reset them with [`set_segments`](Self::set_segments))
    /// and the LOD sizes of a [`BsMeshLodTriShape`]. A skinned Skyrim SE [`BsTriShape`] saves its
    /// triangles in its skin partitions, so the new order is lost on save unless
    /// [`update_skin_partitions`](Self::update_skin_partitions) rebuilds them first.
    ///
    /// Returns `false`, changing nothing, if the shape has no triangle list (not
    /// [`NiTriStrips`]) or `order` isn't a permutation of its triangle indices.
    pub fn reorder_triangles(&mut self, order: &[usize]) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        let (Some(shape), Ok(order)) = (
            block_index_to_ffi(shape),
            order.iter().map(|&i| u32::try_from(i)).collect::<Result<Vec<u32>, _>>(),
        ) else {
            return false;
        };
        nif_ffi::reorder_shape_triangles(file.inner.pin_mut(), shape, &order)
    }

    /// Calculates the normals of the shape from its triangles, adding
    /// normals if it had none. Each vertex gets the normalized sum of its triangles' normals.
    /// Vertices listed in an `NiIntegersExtraData` named `LOCKEDNORM` on the shape keep their
    /// normals.
    ///
    /// Returns `false`, changing nothing, if the shape has no triangle data (an
    /// [`NiTriShape`], [`NiTriStrips`] or [`BsTriShape`]), or is a Skyrim shape whose shader
    /// uses model-space normals and `force_model_space` is off.
    ///
    /// # Compatibility
    ///
    /// `LOCKEDNORM` is a BodySlide and Outfit Studio convention; the games ignore it. See
    /// [`apply_locked_normals`](Self::apply_locked_normals).
    pub fn calc_normals(&mut self, options: Option<NormalOptions>) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        let o = options.unwrap_or_default();
        block_index_to_ffi(shape).is_some_and(|shape| {
            nif_ffi::calc_shape_normals(
                file.inner.pin_mut(),
                shape,
                o.smooth,
                o.smooth_threshold,
                o.force_model_space,
            )
        })
    }

    /// Calculates the tangents and bitangents of the shape from its
    /// triangles, positions, normals and first UV set, adding them if it had none. Tangents
    /// follow the texture's v direction and bitangents its u direction, both made perpendicular
    /// to the normal. Vertices whose triangles give no direction get a tangent built from the
    /// normal. Where the u direction is nearly parallel to the normal, the bitangent may end up
    /// skewed or zero.
    ///
    /// Returns `false`, changing nothing, if the shape has no triangle data (an
    /// [`NiTriShape`], [`NiTriStrips`] or [`BsTriShape`]) or it has no normals or UVs.
    pub fn calc_tangents(&mut self) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        block_index_to_ffi(shape).is_some_and(|shape| nif_ffi::calc_shape_tangents(file.inner.pin_mut(), shape))
    }

    /// Copies the locked normals of `source`, usually a shape in another file, to this shape.
    /// The locked vertices are the ones listed in an `NiIntegersExtraData` named `LOCKEDNORM` on
    /// `source` (see [`ExtraIntegers`](crate::extra_data::ExtraIntegers)); other normals are
    /// kept. This shape's own `LOCKEDNORM` data is replaced with a copy of the source's, so
    /// [`calc_normals`](Self::calc_normals) keeps these normals.
    ///
    /// Returns `false`, changing nothing, if the shape has no normals, `source` has no
    /// `LOCKEDNORM` indices or normals, or the two have different vertex counts. On success, a
    /// replaced `LOCKEDNORM` block no other block uses is deleted, which moves every later block
    /// down one index, and the copy is added as the last block.
    ///
    /// # Compatibility
    ///
    /// `LOCKEDNORM` is a BodySlide and Outfit Studio convention; the games ignore it. Outfit
    /// Studio's "Lock Normals" stores the locked vertices there, and BodySlide keeps their normals
    /// when it recalculates normals for a built mesh. It's used where separate meshes meet, such
    /// as a body's neck and wrists: the edge normals are tuned by hand so the lighting matches
    /// across the seam. To do the same here after reshaping a mesh that has the same vertices as
    /// a reference (a generated body variant, say), recalculate its normals, then copy the
    /// reference's seam normals back:
    ///
    /// ```no_run
    /// # use nifly::{file::NifFile, geometry::NiShape, object3d::Vec3};
    /// # fn f(nif: &mut NifFile, shape: usize, reshaped: &[Vec3], reference: &NifFile) {
    /// let reference_shape = reference.shapes().views().next().unwrap();
    /// let mut shape = nif.edit::<NiShape>(shape).unwrap();
    /// shape.set_vertices(reshaped);
    /// shape.calc_normals(None);
    /// shape.apply_locked_normals(reference_shape);
    /// shape.calc_tangents();
    /// # }
    /// ```
    pub fn apply_locked_normals(&mut self, source: View<'_, NiShape>) -> bool {
        let shape = self.index() as u32;
        let (source_file, source_shape) = (source.file(), source.get());
        nif_ffi::apply_locked_normals(self.file_mut().inner.pin_mut(), shape, &source_file.inner, source_shape.as_ffi())
    }

    /// Replaces the segments of the [`BsSubIndexTriShape`]. Triangles
    /// given `None` go into the first segment.
    ///
    /// The triangles are reordered so that each segment's, and within it each subsegment's,
    /// are together, keeping their relative order; triangle indices change. Subsegments without
    /// a `user_slot` are numbered within their segment, from 1.
    ///
    /// Returns `false`, changing nothing, if the shape isn't a [`BsSubIndexTriShape`],
    /// `segments.triangles` doesn't have one entry per triangle, an entry names a segment or
    /// subsegment that doesn't exist, or a `user_slot` is below 30.
    pub fn set_segments(&mut self, segments: &ShapeSegments) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        let Some(shape) = block_index_to_ffi(shape) else {
            return false;
        };
        // Number the parts as shape_segments does.
        let mut first_part = Vec::with_capacity(segments.segments.len());
        let (mut segs, mut subs, mut extra) = (Vec::new(), Vec::new(), Vec::new());
        let mut part = 0usize;
        for seg in &segments.segments {
            first_part.push(part);
            part += 1 + seg.subsegments.len();
            let Ok(count) = u32::try_from(seg.subsegments.len()) else {
                return false;
            };
            segs.push(nif_ffi::SegmentFfi { subsegments: count });
            for sub in &seg.subsegments {
                if sub.user_slot.is_some_and(|slot| slot < 30) {
                    return false;
                }
                let Ok(extra_len) = u32::try_from(sub.extra_data.len()) else {
                    return false;
                };
                subs.push(nif_ffi::SubsegmentFfi { user_slot: sub.user_slot.unwrap_or(0), material: sub.material, extra_len });
                extra.extend_from_slice(&sub.extra_data);
            }
        }
        let parts: Option<Vec<i32>> = segments
            .triangles
            .iter()
            .map(|t| match t {
                None => Some(-1),
                Some(SegmentIndex { segment, subsegment }) => {
                    let seg = segments.segments.get(*segment)?;
                    let offset = match subsegment {
                        None => 0,
                        Some(s) if *s < seg.subsegments.len() => s + 1,
                        Some(_) => return None,
                    };
                    i32::try_from(first_part[*segment] + offset).ok()
                }
            })
            .collect();
        let Some(parts) = parts else {
            return false;
        };
        nif_ffi::set_shape_segments(file.inner.pin_mut(), shape, &segments.ssf_file, &segs, &subs, &extra, &parts)
    }

    /// Replaces the vertex positions of the shape.
    ///
    /// These setters write wherever the shape keeps its vertex data: its
    /// [`geometry_data`](View::geometry_data) or, for a [`BsTriShape`], the shape itself. They
    /// turn the data on if the shape didn't store it. Each returns `false`, changing nothing, if
    /// the shape isn't such a shape or the slice doesn't have one value per vertex.
    ///
    /// A [`BsTriShape`] stores normals, tangents, the y and z of bitangents, and colors at
    /// reduced precision, so they read back rounded. Colors are clamped to `0.0..=1.0`.
    pub fn set_vertices(&mut self, vertices: &[Vec3]) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        let values: Vec<_> = vertices.iter().map(|v| v.to_ffi()).collect();
        file.set_vertex_array(shape, &values, nif_ffi::set_shape_vertices)
    }

    /// Deletes these vertices from the shape; order and repeats don't
    /// matter. Later vertices move down, and everything that refers to vertices follows: every
    /// vertex array, triangles (those using a deleted vertex are dropped), strips, segments, skin
    /// weights and partitions (empty partitions are removed), and `LOCKEDNORM` indices.
    ///
    /// A shape can be left with no vertices or triangles; delete it with
    /// [`delete`](Self::delete) if it's no longer needed. Returns `false`, changing
    /// nothing, if the shape doesn't store vertices or an index isn't one of its
    /// vertices.
    pub fn delete_vertices(&mut self, vertices: &[usize]) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        let (Some(shape), Ok(vertices)) =
            (block_index_to_ffi(shape), vertices.iter().map(|&v| u32::try_from(v)).collect::<Result<Vec<u32>, _>>())
        else {
            return false;
        };
        nif_ffi::delete_shape_vertices(file.inner.pin_mut(), shape, &vertices)
    }

    /// Moves vertex `vertex` of the shape to `position`. Returns `false`,
    /// changing nothing, if the shape doesn't store vertices or has no such vertex.
    /// See [`set_vertices`](Self::set_vertices).
    pub fn set_vertex(&mut self, vertex: usize, position: Vec3) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        let (Some(shape), Ok(vertex)) = (block_index_to_ffi(shape), u32::try_from(vertex)) else {
            return false;
        };
        nif_ffi::set_shape_vertex(file.inner.pin_mut(), shape, vertex, position.to_ffi())
    }

    /// Replaces the normals of the shape. See
    /// [`set_vertices`](Self::set_vertices).
    pub fn set_normals(&mut self, normals: &[Vec3]) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        let values: Vec<_> = normals.iter().map(|v| v.to_ffi()).collect();
        file.set_vertex_array(shape, &values, nif_ffi::set_shape_normals)
    }

    /// Moves the shape by `offset`, in its own coordinates.
    ///
    /// With `weights` (one per vertex), each vertex moves by that fraction of `offset`: 0 leaves
    /// it, 1 moves it fully. Returns `false`, changing nothing, if the shape has no vertices, or `weights` has a different length or a value that isn't finite.
    pub fn offset(&mut self, offset: Vec3, weights: Option<&[f32]>) -> bool {
        self.transform(weights, |p| p + offset, |n| n, |t| t)
    }

    /// Scales the shape by `scale` along each axis, about its own origin.
    /// Normals, tangents and bitangents follow, so lighting stays correct.
    ///
    /// With `weights` (one per vertex), each vertex moves that fraction of the way to its scaled
    /// position, and its normals and tangents turn that fraction of the way. Returns `false`,
    /// changing nothing, if the shape has no vertices, a scale component isn't
    /// positive and finite (use [`mirror`](Self::mirror) to mirror), or `weights` has
    /// a different length or a value that isn't finite.
    pub fn scale(&mut self, scale: Vec3, weights: Option<&[f32]>) -> bool {
        if ![scale.x, scale.y, scale.z].iter().all(|s| s.is_finite() && *s > 0.0) {
            return false;
        }
        // Normals transform by the cofactor matrix, which for a scale is this.
        let cofactor = Vec3::new(scale.y * scale.z, scale.x * scale.z, scale.x * scale.y);
        let mul = |a: Vec3, b: Vec3| Vec3::new(a.x * b.x, a.y * b.y, a.z * b.z);
        self.transform(weights, |p| mul(p, scale), |n| mul(n, cofactor), |t| mul(t, scale))
    }

    /// Rotates the shape about its own origin by `degrees` around the X,
    /// then Y, then Z axis (each counterclockwise looking down the axis towards the origin).
    /// Normals, tangents and bitangents turn with it.
    ///
    /// With `weights` (one per vertex), each vertex moves that fraction of the way to its rotated
    /// position, and its normals and tangents turn that fraction of the way. Returns `false`,
    /// changing nothing, if the shape has no vertices, an angle isn't finite, or
    /// `weights` has a different length or a value that isn't finite.
    pub fn rotate(&mut self, degrees: Vec3, weights: Option<&[f32]>) -> bool {
        if ![degrees.x, degrees.y, degrees.z].iter().all(|d| d.is_finite()) {
            return false;
        }
        let r = rotation(Vec3::Z, degrees.z) * rotation(Vec3::Y, degrees.y) * rotation(Vec3::X, degrees.x);
        self.transform(weights, |p| r * p, |n| r * n, |t| r * t)
    }

    /// Mirrors the shape along the chosen axes, in its own coordinates:
    /// positions, normals, tangents and bitangents have those components negated. When an odd
    /// number of axes is mirrored, each triangle's winding is reversed so the faces still point
    /// outwards. A [`BsTriShape`]'s tangents are recalculated instead.
    ///
    /// Skinning data isn't mirrored. A skinned Skyrim SE [`BsTriShape`] saves its triangles in
    /// its skin partitions, so the reversed winding is lost on save unless
    /// [`update_skin_partitions`](Self::update_skin_partitions) rebuilds them first.
    ///
    /// Returns `false`, changing nothing, if the shape has no vertices, or is an
    /// [`NiTriStrips`] and an odd number of axes is mirrored (its winding can't be reversed).
    pub fn mirror(&mut self, x: bool, y: bool, z: bool) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        block_index_to_ffi(shape).is_some_and(|shape| nif_ffi::mirror_shape(file.inner.pin_mut(), shape, x, y, z))
    }

    /// Flips the first UV set of the shape: each `u` becomes `1 - u` if
    /// `flip_u`, and each `v` becomes `1 - v` if `flip_v`, mirroring the texture horizontally or
    /// vertically. Returns `false`, changing nothing, if the shape has no UVs.
    pub fn invert_uvs(&mut self, flip_u: bool, flip_v: bool) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        block_index_to_ffi(shape).is_some_and(|shape| nif_ffi::invert_shape_uvs(file.inner.pin_mut(), shape, flip_u, flip_v))
    }

    /// Replaces the first UV set of the shape, as `[u, v]`. See
    /// [`set_vertices`](Self::set_vertices).
    pub fn set_uvs(&mut self, uvs: &[[f32; 2]]) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        let values: Vec<_> = uvs.iter().map(|&[u, v]| nif_ffi::Uv { u, v }).collect();
        file.set_vertex_array(shape, &values, nif_ffi::set_shape_uvs)
    }

    /// Replaces the vertex colors of the shape, as `[r, g, b, a]`. See
    /// [`set_vertices`](Self::set_vertices).
    pub fn set_vertex_colors(&mut self, colors: &[[f32; 4]]) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        let values: Vec<_> = colors.iter().map(|&[r, g, b, a]| nif_ffi::Color { r, g, b, a }).collect();
        file.set_vertex_array(shape, &values, nif_ffi::set_shape_colors)
    }

    /// Replaces the tangents of the shape. Turning tangents on also
    /// stores bitangents, zeroed if the shape had none. See
    /// [`set_vertices`](Self::set_vertices).
    pub fn set_tangents(&mut self, tangents: &[Vec3]) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        let values: Vec<_> = tangents.iter().map(|v| v.to_ffi()).collect();
        file.set_vertex_array(shape, &values, nif_ffi::set_shape_tangents)
    }

    /// Replaces the bitangents of the shape. Turning bitangents on also
    /// stores tangents, zeroed if the shape had none. See
    /// [`set_vertices`](Self::set_vertices).
    pub fn set_bitangents(&mut self, bitangents: &[Vec3]) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        let values: Vec<_> = bitangents.iter().map(|v| v.to_ffi()).collect();
        file.set_vertex_array(shape, &values, nif_ffi::set_shape_bitangents)
    }

    /// Replaces the eye data of the [`BsTriShape`]. Returns `false`,
    /// changing nothing, if the shape isn't a [`BsTriShape`] or `eye_data` doesn't have one
    /// value per vertex.
    pub fn set_eye_data(&mut self, eye_data: &[f32]) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        file.set_vertex_array(shape, eye_data, nif_ffi::set_shape_eye_data)
    }
}

/// A counterclockwise rotation by `degrees` about the unit `axis` (X, Y or Z).
fn rotation(axis: Vec3, degrees: f32) -> Matrix3 {
    let (s, c) = degrees.to_radians().sin_cos();
    if axis == Vec3::X {
        Matrix3::from_rows(Vec3::X, Vec3::new(0.0, c, -s), Vec3::new(0.0, s, c))
    } else if axis == Vec3::Y {
        Matrix3::from_rows(Vec3::new(c, 0.0, s), Vec3::Y, Vec3::new(-s, 0.0, c))
    } else {
        Matrix3::from_rows(Vec3::new(c, -s, 0.0), Vec3::new(s, c, 0.0), Vec3::Z)
    }
}

/// Whether every component is under 0.0001 in magnitude, the threshold for an unchanged vertex.
fn nearly_zero(components: &[f32]) -> bool {
    components.iter().all(|c| c.abs() < 1e-4)
}

/// The additional data of an [`NiGeometryData`].
impl<'a, T: NiType> View<'a, T> {
    /// The data's [`AdditionalGeomData`], or `None` if it has none. Saved from version 20.0.0.4
    /// on, so not in Morrowind or early Oblivion files.
    pub fn additional_data<I>(&self) -> Option<View<'a, AdditionalGeomData>>
    where
        T: IsA<NiGeometryData, I>,
    {
        let data: &NiGeometryData = self.get().upcast_ref();
        self.resolve(index_from_ffi(ffi::geometry_data_additional_data(&data.0)))
    }
}

impl<T: NiType> Edit<'_, T> {
    /// Like [`View::additional_data`], for editing.
    pub fn additional_data<I>(&mut self) -> Option<Edit<'_, AdditionalGeomData>>
    where
        T: IsA<NiGeometryData, I>,
    {
        let index = self.view().additional_data()?.index();
        self.resolve(Some(index))
    }

    /// Gives the data a new `A`, with the data's vertex count and no channels or blocks, as its
    /// additional data, replacing any it had, and returns the new block's index. It's added at
    /// the end. Replaced additional data stays in the file until saving drops it, if nothing
    /// else uses it.
    ///
    /// ```
    /// # use nifly::{file::NifFile, geometry::{NiAdditionalGeometryData, NiGeometryData}};
    /// # let file = std::fs::read("nifly-sys/vendor/nifly/tests/input/TestNifFile_Skinned_OB.nif").unwrap();
    /// # let mut nif = NifFile::load(file.as_slice(), None).unwrap();
    /// let data = nif.blocks_of::<NiGeometryData>().indices().next().unwrap();
    /// let extra = nif.edit::<NiGeometryData>(data).unwrap().assign_additional_data::<NiAdditionalGeometryData, _>();
    /// ```
    pub fn assign_additional_data<A: NewAdditionalData, I>(&mut self) -> Index
    where
        T: IsA<NiGeometryData, I>,
    {
        let data = self.index() as u32;
        index_from_ffi(nif_ffi::assign_additional_geometry_data(self.file_mut().inner.pin_mut(), data, A::PACKED))
            .expect("the block is geometry data")
    }

    /// Clears the data's additional data. It stays in the file until saving drops it, if
    /// nothing else uses it. Returns `false` if the data had none.
    pub fn remove_additional_data<I>(&mut self) -> bool
    where
        T: IsA<NiGeometryData, I>,
    {
        let data = self.index() as u32;
        nif_ffi::remove_additional_geometry_data(self.file_mut().inner.pin_mut(), data)
    }

    /// Replaces the texture coordinate sets and their count in the
    /// [data flags](NiGeometryData::data_raw_flags). Returns `false`, changing nothing, unless each
    /// set has one UV per vertex and there are at most 63 sets, or one from Fallout 3 on.
    pub fn set_uv_sets<S: AsRef<[[f32; 2]]>, I>(&mut self, sets: &[S]) -> bool
    where
        T: IsA<NiGeometryData, I>,
    {
        let bethesda = self.view().file().version().stream >= 34;
        let flat: Vec<f32> = sets.iter().flat_map(|s| s.as_ref().as_flattened().iter().copied()).collect();
        let index = self.index();
        let data = self.file_mut().block_of_mut::<NiGeometryData>(index).expect("the block is geometry data");
        ffi::set_geometry_data_uv_sets(data.ffi_mut(), &flat, sets.len(), bethesda)
    }

    /// Sets the [data flags](NiGeometryData::data_raw_flags), adding zeroed UV sets or removing
    /// them to match the count, and adding zeroed tangents and bitangents or removing them to
    /// match the tangent method.
    pub fn set_data_raw_flags<I>(&mut self, flags: u16)
    where
        T: IsA<NiGeometryData, I>,
    {
        let bethesda = self.view().file().version().stream >= 34;
        let index = self.index();
        let data = self.file_mut().block_of_mut::<NiGeometryData>(index).expect("the block is geometry data");
        ffi::set_geometry_data_flags(data.ffi_mut(), flags, bethesda);
    }

    /// Points the data at the [`AdditionalGeomData`] at `extra`, or clears it with `None`.
    /// Returns `false`, changing nothing, if that block isn't one.
    pub fn set_additional_data<I>(&mut self, extra: Option<Index>) -> bool
    where
        T: IsA<NiGeometryData, I>,
    {
        self.set_ref(RefKind::AdditionalData, extra)
    }
}
