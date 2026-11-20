//! BSTriShape and its LOD, sub-index and segmented subclasses.

use super::*;

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
