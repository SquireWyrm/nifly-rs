//! Triangle strip, packed and compressed mesh shapes.

use super::*;

/// Its [`NiTriStripsData`] blocks are on [`View::strips_data`].
impl BhkNiTriStripsShape {
    fields! {
        /// The Havok material, as a raw value whose meaning depends on the game.
        material, set_material: u32 = strips_shape_material, set_strips_shape_material;
        /// The radius around the shape, in Havok units.
        radius, set_radius: f32 = strips_shape_radius, set_strips_shape_radius;
        grow_by, set_grow_by: u32 = strips_shape_grow_by, set_strips_shape_grow_by;
        /// The scale, `[x, y, z, w]`.
        scale, set_scale: [f32; 4] = strips_shape_scale, set_strips_shape_scale;
    }

    /// Each strips data block's collision filter, as a raw value, usually one per block.
    pub fn filters(&self) -> &[u32] {
        ffi::strips_shape_filters(&self.0)
    }

    /// Replaces the [filters](Self::filters). Returns `false`, changing nothing, unless there's
    /// one per [strips data](View::strips_data) ref.
    pub fn set_filters(&mut self, filters: &[u32]) -> bool {
        ffi::set_strips_shape_filters(self.ffi_mut(), filters)
    }
}

/// A part of a packed mesh: a run of vertices with its own filter and material.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SubPart {
    pub filter: HavokFilter,
    pub vertex_count: u32,
    /// The Havok material, as a raw value whose meaning depends on the game.
    pub material: u32,
}

impl SubPart {
    pub(super) fn from_ffi(p: ffi::SubPartFfi) -> Self {
        Self {
            filter: HavokFilter::from_ffi(p.filter),
            vertex_count: p.vertex_count,
            material: p.material,
        }
    }

    pub(super) fn to_ffi(self) -> ffi::SubPartFfi {
        ffi::SubPartFfi {
            filter: self.filter.to_ffi(),
            vertex_count: self.vertex_count,
            material: self.material,
        }
    }
}

fn sub_parts_from_ffi(parts: Vec<ffi::SubPartFfi>) -> Vec<SubPart> {
    parts.into_iter().map(SubPart::from_ffi).collect()
}

fn sub_parts_to_ffi(parts: &[SubPart]) -> Vec<ffi::SubPartFfi> {
    parts.iter().map(|p| p.to_ffi()).collect()
}

/// Its data is on [`View::packed_data`].
impl BhkPackedNiTriStripsShape {
    /// The mesh's sub-parts. Only saved up to Oblivion; later files save the
    /// [data's](HkPackedNiTriStripsData::sub_parts) instead.
    pub fn sub_parts(&self) -> Vec<SubPart> {
        sub_parts_from_ffi(ffi::packed_shape_sub_parts(&self.0))
    }

    /// Replaces the [sub-parts](Self::sub_parts). Returns `false`, changing nothing, for more
    /// than 65535.
    pub fn set_sub_parts(&mut self, parts: &[SubPart]) -> bool {
        ffi::set_packed_shape_sub_parts(self.ffi_mut(), &sub_parts_to_ffi(parts))
    }

    fields! {
        user_data, set_user_data: u32 = packed_shape_user_data, set_packed_shape_user_data;
        /// The radius around the shape, in Havok units.
        radius, set_radius: f32 = packed_shape_radius, set_packed_shape_radius;
        /// The scale, `[x, y, z, w]`.
        scale, set_scale: [f32; 4] = packed_shape_scale, set_packed_shape_scale;
        /// Usually the same as [`radius`](Self::radius).
        radius_copy, set_radius_copy: f32 = packed_shape_radius_copy, set_packed_shape_radius_copy;
        /// Usually the same as [`scale`](Self::scale).
        scale_copy, set_scale_copy: [f32; 4] = packed_shape_scale_copy, set_packed_shape_scale_copy;
    }
}

/// A triangle of an [`HkPackedNiTriStripsData`].
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PackedTriangle {
    /// Vertex indices.
    pub triangle: [u16; 3],
    pub welding_info: u16,
    /// The face normal. Only saved up to Oblivion.
    pub normal: Vec3,
}

/// Vertices are in Havok units.
impl HkPackedNiTriStripsData {
    /// The triangles. Normals are zero when read from a file that doesn't save them.
    pub fn triangles(&self) -> Vec<PackedTriangle> {
        ffi::packed_data_triangles(&self.0)
            .into_iter()
            .map(|t| PackedTriangle {
                triangle: [t.p1, t.p2, t.p3],
                welding_info: t.welding_info,
                normal: Vec3::from_ffi(t.normal),
            })
            .collect()
    }

    /// Replaces the [triangles](Self::triangles). Returns `false`, changing nothing, if an index
    /// isn't below the vertex count.
    pub fn set_triangles(&mut self, triangles: &[PackedTriangle]) -> bool {
        let triangles: Vec<_> = triangles
            .iter()
            .map(|t| ffi::PackedTriangleFfi {
                p1: t.triangle[0],
                p2: t.triangle[1],
                p3: t.triangle[2],
                welding_info: t.welding_info,
                normal: t.normal.to_ffi(),
            })
            .collect();
        ffi::set_packed_data_triangles(self.ffi_mut(), &triangles)
    }

    pub fn vertices(&self) -> Vec<Vec3> {
        ffi::packed_data_vertices(&self.0)
            .into_iter()
            .map(Vec3::from_ffi)
            .collect()
    }

    /// Replaces the vertices, changing the vertex count. Returns `false`, changing nothing, if a
    /// triangle uses a vertex past the new count.
    pub fn set_vertices(&mut self, vertices: &[Vec3]) -> bool {
        let vertices: Vec<_> = vertices.iter().map(|v| v.to_ffi()).collect();
        ffi::set_packed_data_vertices(self.ffi_mut(), &vertices)
    }

    /// The mesh's sub-parts. Only saved after Oblivion; older files save the
    /// [shape's](BhkPackedNiTriStripsShape::sub_parts) instead.
    pub fn sub_parts(&self) -> Vec<SubPart> {
        sub_parts_from_ffi(ffi::packed_data_sub_parts(&self.0))
    }

    /// Replaces the [sub-parts](Self::sub_parts). Returns `false`, changing nothing, for more
    /// than 65535.
    pub fn set_sub_parts(&mut self, parts: &[SubPart]) -> bool {
        ffi::set_packed_data_sub_parts(self.ffi_mut(), &sub_parts_to_ffi(parts))
    }
}

/// Its data is on [`View::compressed_data`] and the scene object it was made from on
/// [`View::mesh_target`].
impl BhkCompressedMeshShape {
    fields! {
        user_data, set_user_data: u32 = compressed_shape_user_data, set_compressed_shape_user_data;
        /// The radius around the shape, in Havok units.
        radius, set_radius: f32 = compressed_shape_radius, set_compressed_shape_radius;
        /// A value of unknown meaning.
        unknown_float, set_unknown_float: f32 = compressed_shape_unknown_float, set_compressed_shape_unknown_float;
        /// The scale, `[x, y, z, w]`.
        scale, set_scale: [f32; 4] = compressed_shape_scale, set_compressed_shape_scale;
        /// Usually the same as [`radius`](Self::radius).
        radius_copy, set_radius_copy: f32 = compressed_shape_radius_copy, set_compressed_shape_radius_copy;
        /// Usually the same as [`scale`](Self::scale).
        scale_copy, set_scale_copy: [f32; 4] = compressed_shape_scale_copy, set_compressed_shape_scale_copy;
    }
}

/// A material of a [`BhkCompressedMeshShapeData`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ChunkMaterial {
    /// The Havok material, as a raw value whose meaning depends on the game.
    pub material: u32,
    pub filter: HavokFilter,
}

/// A transform of a [`BhkCompressedMeshShapeData`].
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ChunkTransform {
    /// `[x, y, z, w]` in Havok units.
    pub translation: [f32; 4],
    /// A quaternion, `[x, y, z, w]`.
    pub rotation: [f32; 4],
}

/// A triangle of a [`BhkCompressedMeshShapeData`] stored outside its chunks.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct BigTriangle {
    /// Indices into the [big vertices](BhkCompressedMeshShapeData::big_vertices).
    pub triangle: [u16; 3],
    /// The Havok material, as a raw value.
    pub material: u32,
    pub welding_info: u16,
}

/// A chunk of a [`BhkCompressedMeshShapeData`]: compressed vertices and the triangles made of
/// them.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MeshChunk {
    /// The offset added to the vertices, `[x, y, z, w]` in Havok units.
    pub translation: [f32; 4],
    /// An index into the [materials](BhkCompressedMeshShapeData::chunk_materials).
    pub material_index: u32,
    pub reference: u16,
    /// An index into the [transforms](BhkCompressedMeshShapeData::chunk_transforms).
    pub transform_index: u16,
    /// Vertex positions as `x`, `y`, `z` triples, in thousandths of a Havok unit from the
    /// translation.
    pub vertices: Vec<u16>,
    pub indices: Vec<u16>,
    /// The length of each strip at the start of the indices; the rest are plain triangles.
    pub strips: Vec<u16>,
    pub welding_info: Vec<u16>,
}

/// Lists and fields aren't checked against each other, except big triangles against big
/// vertices.
impl BhkCompressedMeshShapeData {
    fields! {
        bits_per_index, set_bits_per_index: u32 = cms_bits_per_index, set_cms_bits_per_index;
        bits_per_welding_index, set_bits_per_welding_index: u32 =
            cms_bits_per_welding_index, set_cms_bits_per_welding_index;
        mask_welding_index, set_mask_welding_index: u32 = cms_mask_welding_index, set_cms_mask_welding_index;
        mask_index, set_mask_index: u32 = cms_mask_index, set_cms_mask_index;
        /// The largest error compression introduced, in Havok units.
        error, set_error: f32 = cms_error, set_cms_error;
        /// The minimum corner of the bounding box, `[x, y, z, w]` in Havok units.
        aabb_min, set_aabb_min: [f32; 4] = cms_aabb_min, set_cms_aabb_min;
        /// The maximum corner of the bounding box, `[x, y, z, w]` in Havok units.
        aabb_max, set_aabb_max: [f32; 4] = cms_aabb_max, set_cms_aabb_max;
        /// The welding type, as a raw value.
        welding_type, set_welding_type: u8 = cms_welding_type, set_cms_welding_type;
        /// The material type, as a raw value.
        material_type, set_material_type: u8 = cms_material_type, set_cms_material_type;
        named_material_count, set_named_material_count: u32 =
            cms_named_material_count, set_cms_named_material_count;
        convex_piece_count, set_convex_piece_count: u32 = cms_convex_piece_count, set_cms_convex_piece_count;
    }

    /// The 32-bit material list, usually empty.
    pub fn materials_32(&self) -> &[u32] {
        ffi::cms_materials_n(&self.0, 0)
    }

    pub fn set_materials_32(&mut self, materials: &[u32]) {
        ffi::set_cms_materials_n(self.ffi_mut(), 0, materials);
    }

    /// The 16-bit material list, usually empty.
    pub fn materials_16(&self) -> &[u32] {
        ffi::cms_materials_n(&self.0, 1)
    }

    pub fn set_materials_16(&mut self, materials: &[u32]) {
        ffi::set_cms_materials_n(self.ffi_mut(), 1, materials);
    }

    /// The 8-bit material list, usually empty.
    pub fn materials_8(&self) -> &[u32] {
        ffi::cms_materials_n(&self.0, 2)
    }

    pub fn set_materials_8(&mut self, materials: &[u32]) {
        ffi::set_cms_materials_n(self.ffi_mut(), 2, materials);
    }

    /// The materials chunks refer to.
    pub fn chunk_materials(&self) -> Vec<ChunkMaterial> {
        ffi::cms_materials(&self.0)
            .into_iter()
            .map(|m| ChunkMaterial {
                material: m.material,
                filter: HavokFilter::from_ffi(m.filter),
            })
            .collect()
    }

    pub fn set_chunk_materials(&mut self, materials: &[ChunkMaterial]) {
        let materials: Vec<_> = materials
            .iter()
            .map(|m| ffi::CmsMaterialFfi {
                material: m.material,
                filter: m.filter.to_ffi(),
            })
            .collect();
        ffi::set_cms_materials(self.ffi_mut(), &materials);
    }

    /// The transforms chunks refer to.
    pub fn chunk_transforms(&self) -> Vec<ChunkTransform> {
        ffi::cms_transforms(&self.0)
            .into_iter()
            .map(|t| ChunkTransform {
                translation: t.translation,
                rotation: t.rotation,
            })
            .collect()
    }

    pub fn set_chunk_transforms(&mut self, transforms: &[ChunkTransform]) {
        let transforms: Vec<_> = transforms
            .iter()
            .map(|t| ffi::CmsTransformFfi {
                translation: t.translation,
                rotation: t.rotation,
            })
            .collect();
        ffi::set_cms_transforms(self.ffi_mut(), &transforms);
    }

    /// Uncompressed vertices, `[x, y, z, w]` in Havok units, for the
    /// [big triangles](Self::big_triangles).
    pub fn big_vertices(&self) -> Vec<[f32; 4]> {
        ffi::cms_big_vertices(&self.0).as_chunks::<4>().0.to_vec()
    }

    /// Replaces the [big vertices](Self::big_vertices). Returns `false`, changing nothing, if a
    /// big triangle uses a vertex past the new count.
    pub fn set_big_vertices(&mut self, vertices: &[[f32; 4]]) -> bool {
        ffi::set_cms_big_vertices(self.ffi_mut(), vertices.as_flattened())
    }

    /// Triangles too large for a chunk.
    pub fn big_triangles(&self) -> Vec<BigTriangle> {
        ffi::cms_big_triangles(&self.0)
            .into_iter()
            .map(|t| BigTriangle {
                triangle: [t.p1, t.p2, t.p3],
                material: t.material,
                welding_info: t.welding_info,
            })
            .collect()
    }

    /// Replaces the [big triangles](Self::big_triangles). Returns `false`, changing nothing, if
    /// an index isn't below the big vertex count.
    pub fn set_big_triangles(&mut self, triangles: &[BigTriangle]) -> bool {
        let triangles: Vec<_> = triangles
            .iter()
            .map(|t| ffi::CmsBigTriangleFfi {
                p1: t.triangle[0],
                p2: t.triangle[1],
                p3: t.triangle[2],
                material: t.material,
                welding_info: t.welding_info,
            })
            .collect();
        ffi::set_cms_big_triangles(self.ffi_mut(), &triangles)
    }

    pub fn chunks(&self) -> Vec<MeshChunk> {
        (0..ffi::cms_chunk_count(&self.0))
            .map(|i| {
                let c = ffi::cms_chunk(&self.0, i);
                let list = |which| ffi::cms_chunk_list(&self.0, i, which).to_vec();
                MeshChunk {
                    translation: c.translation,
                    material_index: c.material_index,
                    reference: c.reference,
                    transform_index: c.transform_index,
                    vertices: list(0),
                    indices: list(1),
                    strips: list(2),
                    welding_info: list(3),
                }
            })
            .collect()
    }

    pub fn set_chunks(&mut self, chunks: &[MeshChunk]) {
        let mut headers = Vec::with_capacity(chunks.len());
        let mut lists = Vec::new();
        let mut lengths = Vec::with_capacity(chunks.len() * 4);
        for c in chunks {
            headers.push(ffi::CmsChunkFfi {
                translation: c.translation,
                material_index: c.material_index,
                reference: c.reference,
                transform_index: c.transform_index,
            });
            for list in [&c.vertices, &c.indices, &c.strips, &c.welding_info] {
                lists.extend_from_slice(list);
                lengths.push(u32::try_from(list.len()).expect("a chunk list fits in u32"));
            }
        }
        assert!(
            ffi::set_cms_chunks(self.ffi_mut(), &headers, &lists, &lengths),
            "lengths add up"
        );
    }
}
