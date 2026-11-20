//! BSGeometry and its mesh data.

use super::*;

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

    pub fn set_bounds(&mut self, bounds: BoundingSphere) {
        ffi::set_bs_geometry_bounds(self.ffi_mut(), bounds.to_ffi());
    }

    /// The bounding box, as stored.
    pub fn bound_min_max(&self) -> [f32; 6] {
        let mut out = [0.0; 6];
        ffi::bs_geometry_bound_min_max(&self.0, &mut out);
        out
    }

    pub fn set_bound_min_max(&mut self, values: [f32; 6]) {
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
    pub fn set_internal_geometry(&mut self, internal: bool) {
        let flags = self.raw_flags() & !0x200 | if internal { 0x200 } else { 0 };
        self.set_raw_flags(flags);
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
    pub fn mesh_mut(&mut self, index: usize) -> Option<&mut BsGeometryMeshData> {
        let mesh = ffi::bs_geometry_mesh_mut(self.ffi_mut(), index);
        // SAFETY: null or a mesh this shape owns, exclusively borrowed with it. The wrapper is
        // `repr(transparent)`, and C++ objects never move.
        unsafe { mesh.cast::<BsGeometryMeshData>().as_mut() }
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
    pub fn set_mesh_info(&mut self, index: usize, info: MeshInfo) -> bool {
        ffi::set_bs_geometry_mesh_info(
            self.ffi_mut(),
            index,
            &[info.index_count, info.vertex_count, info.raw_flags],
        )
    }

    /// Adds an empty mesh at the end. Returns `false`, changing nothing, if there are already 4.
    pub fn add_mesh(&mut self) -> bool {
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

    pub fn set_version(&mut self, version: u32) {
        ffi::set_mesh_data_version(self.ffi_mut(), version);
    }

    /// Vertex positions are saved as 16-bit fractions of this times 69.969. With 0 or less,
    /// nothing after the triangles is saved.
    pub fn scale(&self) -> f32 {
        ffi::mesh_data_scale(&self.0)
    }

    /// Sets the [scale](Self::scale). Saving uses a larger one, without changing this, when
    /// a position wouldn't fit.
    pub fn set_scale(&mut self, scale: f32) {
        ffi::set_mesh_data_scale(self.ffi_mut(), scale);
    }

    pub fn triangles(&self) -> Vec<[u16; 3]> {
        triangles_of(ffi::mesh_data_triangles(&self.0))
    }

    /// Replaces the triangles. Returns `false`, changing nothing, if an index isn't below the
    /// vertex count.
    pub fn set_triangles(&mut self, triangles: &[[u16; 3]]) -> bool {
        ffi::set_mesh_data_triangles(self.ffi_mut(), triangles.as_flattened())
    }

    /// Replaces the vertex positions, changing the vertex count. Normals, tangents, UVs,
    /// colors and skin weights that aren't empty are resized to match, new entries zeroed
    /// (colors white). Returns `false`, changing nothing, if a triangle or LOD triangle uses a
    /// vertex past the new count.
    pub fn replace_vertices(&mut self, vertices: &[Vec3]) -> bool {
        let vertices: Vec<_> = vertices.iter().map(|v| v.to_ffi()).collect();
        ffi::replace_mesh_data_vertices(self.ffi_mut(), &vertices)
    }

    pub fn uvs(&self) -> Vec<[f32; 2]> {
        ffi::mesh_data_uvs(&self.0, 0).as_chunks::<2>().0.to_vec()
    }

    pub fn set_uvs(&mut self, uvs: &[[f32; 2]]) -> bool {
        ffi::set_mesh_data_uvs(self.ffi_mut(), 0, uvs.as_flattened())
    }

    pub fn second_uvs(&self) -> Vec<[f32; 2]> {
        ffi::mesh_data_uvs(&self.0, 1).as_chunks::<2>().0.to_vec()
    }

    pub fn set_second_uvs(&mut self, uvs: &[[f32; 2]]) -> bool {
        ffi::set_mesh_data_uvs(self.ffi_mut(), 1, uvs.as_flattened())
    }

    /// The vertex colors saved, as RGBA bytes.
    pub fn colors(&self) -> Vec<[u8; 4]> {
        ffi::mesh_data_colors(&self.0).as_chunks::<4>().0.to_vec()
    }

    pub fn set_colors(&mut self, colors: &[[u8; 4]]) -> bool {
        ffi::set_mesh_data_colors(self.ffi_mut(), colors.as_flattened())
    }

    /// Each tangent's 2-bit W component, the bitangent's sign.
    pub fn tangent_ws(&self) -> Vec<u8> {
        ffi::mesh_data_tangent_ws(&self.0).to_vec()
    }

    /// Replaces the [tangent W components](Self::tangent_ws). Returns `false`, changing
    /// nothing, unless there's one per tangent, each at most 3.
    pub fn set_tangent_ws(&mut self, ws: &[u8]) -> bool {
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
            .map(|v| {
                v.iter()
                    .map(|&[bone, weight]| MeshWeight { bone, weight })
                    .collect()
            })
            .collect()
    }

    /// Replaces the [skin weights](Self::skin_weights). Returns `false`, changing nothing,
    /// unless it's empty or has a list per vertex, all the same length, from 1 to 8.
    pub fn set_skin_weights<S: AsRef<[MeshWeight]>>(&mut self, weights: &[S]) -> bool {
        let per = weights.first().map_or(0, |w| w.as_ref().len());
        if weights.iter().any(|w| w.as_ref().len() != per) {
            return false;
        }
        let flat: Vec<u16> = weights
            .iter()
            .flat_map(|w| w.as_ref().iter().flat_map(|w| [w.bone, w.weight]))
            .collect();
        ffi::set_mesh_data_skin_weights(self.ffi_mut(), &flat, per as u32)
    }

    /// How many bones each vertex is weighted to: the length of each list in
    /// [`skin_weights`](Self::skin_weights), or 0 without weights.
    pub fn weights_per_vertex(&self) -> usize {
        ffi::mesh_data_weights_per_vertex(&self.0) as usize
    }

    /// Sets how many bones each vertex is weighted to, from 1 to 8. A vertex with more weights
    /// keeps its strongest ones, rescaled so they still sum to 65535 if a dropped weight was
    /// nonzero. One with fewer gets weights of 0 added.
    ///
    /// Returns `false`, changing nothing, unless `count` is from 1 to 8 and the mesh has
    /// [skin weights](Self::skin_weights).
    pub fn set_weights_per_vertex(&mut self, count: usize) -> bool {
        u32::try_from(count)
            .is_ok_and(|count| ffi::set_mesh_data_weights_per_vertex(self.ffi_mut(), count))
    }

    /// Lower-detail triangle lists over the same vertices.
    pub fn lods(&self) -> Vec<Vec<[u16; 3]>> {
        (0..ffi::mesh_data_lod_count(&self.0))
            .map(|i| triangles_of(ffi::mesh_data_lod(&self.0, i)))
            .collect()
    }

    /// Replaces the [LODs](Self::lods). Returns `false`, changing nothing, if an index isn't
    /// below the vertex count.
    pub fn set_lods<S: AsRef<[[u16; 3]]>>(&mut self, lods: &[S]) -> bool {
        let flat: Vec<u16> = lods
            .iter()
            .flat_map(|l| l.as_ref().as_flattened().iter().copied())
            .collect();
        let lengths: Vec<usize> = lods.iter().map(|l| l.as_ref().len() * 3).collect();
        ffi::set_mesh_data_lods(self.ffi_mut(), &flat, &lengths)
    }

    pub fn meshlets(&self) -> Vec<Meshlet> {
        ffi::mesh_data_meshlets(&self.0)
            .as_chunks::<4>()
            .0
            .iter()
            .map(
                |&[
                    vertex_count,
                    vertex_offset,
                    primitive_count,
                    primitive_offset,
                ]| Meshlet {
                    vertex_count,
                    vertex_offset,
                    primitive_count,
                    primitive_offset,
                },
            )
            .collect()
    }

    /// Replaces the [meshlets](Self::meshlets). Not checked against the triangles; see
    /// [`generate_meshlets`](Self::generate_meshlets).
    pub fn set_meshlets(&mut self, meshlets: &[Meshlet]) {
        let flat: Vec<u32> = meshlets
            .iter()
            .flat_map(|m| {
                [
                    m.vertex_count,
                    m.vertex_offset,
                    m.primitive_count,
                    m.primitive_offset,
                ]
            })
            .collect();
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

    pub fn set_cull_data(&mut self, cull_data: &[CullData]) {
        let flat: Vec<_> = cull_data
            .iter()
            .flat_map(|c| [c.center.to_ffi(), c.expand.to_ffi()])
            .collect();
        ffi::set_mesh_data_cull_data(self.ffi_mut(), &flat);
    }

    /// Replaces the meshlets and cull data with new ones covering the triangles, each with at
    /// most `max_vertices` vertices (at least 3) and `max_primitives` triangles (at least 1).
    /// Returns `false`, changing nothing, if there are no triangles or vertices.
    pub fn generate_meshlets(&mut self, max_vertices: u32, max_primitives: u32) -> bool {
        ffi::generate_mesh_data_meshlets(self.ffi_mut(), max_vertices, max_primitives)
    }
}
