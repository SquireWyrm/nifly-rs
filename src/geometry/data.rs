//! Geometry data: NiGeometryData, triangle, strip, line and screen element data, and NiGeometry.

use super::*;

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
    pub fn set_bounds(&mut self, bounds: BoundingSphere) {
        ffi::set_geometry_data_bounds(self.ffi_mut(), bounds.to_ffi());
    }

    /// Recalculates the [bounds](Self::bounds) from the vertices.
    pub fn update_bounds(&mut self) {
        ffi::update_geometry_data_bounds(self.ffi_mut());
    }

    /// How often the game may change the data, or `None` if the stored value isn't one of
    /// [`Consistency`]'s.
    pub fn consistency(&self) -> Option<Consistency> {
        Consistency::try_from(ffi::geometry_data_consistency(&self.0)).ok()
    }

    /// Sets how often the game may change the data. Files before version 10.0.1.0 (such as
    /// Morrowind's) and [`BsGeometryMeshData`] don't store it, so it's lost on save.
    pub fn set_consistency(&mut self, consistency: Consistency) {
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

    fn set_vectors(&mut self, which: u8, values: &[Vec3]) -> bool {
        let values: Vec<_> = values.iter().map(|v| v.to_ffi()).collect();
        ffi::set_geometry_data_vectors(self.ffi_mut(), which, &values)
    }

    /// Replaces the vertex positions. Returns `false`, changing nothing, unless there's one per
    /// vertex; the vertex count changes through the shape's
    /// [`set_vertices`](crate::handle::Edit::set_vertices).
    pub fn set_vertices(&mut self, vertices: &[Vec3]) -> bool {
        self.set_vectors(0, vertices)
    }

    /// Replaces the normals, adding them if the data has none. Returns `false`, changing
    /// nothing, unless there's one per vertex.
    pub fn set_normals(&mut self, normals: &[Vec3]) -> bool {
        self.set_vectors(1, normals)
    }

    /// Replaces the tangents, adding tangents and bitangents if the data has none. Returns
    /// `false`, changing nothing, unless there's one per vertex.
    pub fn set_tangents(&mut self, tangents: &[Vec3]) -> bool {
        self.set_vectors(2, tangents)
    }

    /// Replaces the bitangents, adding tangents and bitangents if the data has none. Returns
    /// `false`, changing nothing, unless there's one per vertex.
    pub fn set_bitangents(&mut self, bitangents: &[Vec3]) -> bool {
        self.set_vectors(3, bitangents)
    }

    /// Replaces the vertex colors, adding them if the data has none. Returns `false`, changing
    /// nothing, unless there's one per vertex.
    pub fn set_vertex_colors(&mut self, colors: &[[f32; 4]]) -> bool {
        ffi::set_geometry_data_colors(self.ffi_mut(), colors.as_flattened())
    }

    /// Adds zeroed normals, keeping any it has, or removes them.
    pub fn set_has_normals(&mut self, enable: bool) {
        ffi::set_geometry_data_has(self.ffi_mut(), 0, enable);
    }

    /// Adds white vertex colors, keeping any it has, or removes them.
    pub fn set_has_vertex_colors(&mut self, enable: bool) {
        ffi::set_geometry_data_has(self.ffi_mut(), 1, enable);
    }

    /// Adds tangents and bitangents, keeping any it has, or removes them. They're saved only
    /// with normals.
    pub fn set_has_tangents(&mut self, enable: bool) {
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

    pub fn set_geometry_group_id(&mut self, id: i32) {
        ffi::set_geometry_data_group_id(self.ffi_mut(), id);
    }

    /// Saved in files from version 10.1.0.0.
    pub fn keep_raw_flags(&self) -> u8 {
        ffi::geometry_data_byte(&self.0, 0)
    }

    pub fn set_keep_raw_flags(&mut self, flags: u8) {
        ffi::set_geometry_data_byte(self.ffi_mut(), 0, flags);
    }

    /// Saved in files from version 10.1.0.0.
    pub fn compress_raw_flags(&self) -> u8 {
        ffi::geometry_data_byte(&self.0, 1)
    }

    pub fn set_compress_raw_flags(&mut self, flags: u8) {
        ffi::set_geometry_data_byte(self.ffi_mut(), 1, flags);
    }

    /// Saved only in version 20.2.0.7 files after Fallout 3 (stream above 34).
    pub fn material_crc(&self) -> u32 {
        ffi::geometry_data_material_crc(&self.0)
    }

    pub fn set_material_crc(&mut self, crc: u32) {
        ffi::set_geometry_data_material_crc(self.ffi_mut(), crc);
    }
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
    pub fn set_match_groups<S: AsRef<[u16]>>(&mut self, groups: &[S]) -> bool {
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
    pub fn set_strips<S: AsRef<[u16]>>(&mut self, strips: &[S]) -> bool {
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

    pub fn set_materials<N: AsRef<[u8]>>(&mut self, materials: &[(N, u32)]) {
        let (bytes, lengths) = crate::extra_data::joined(materials.iter().map(|(name, _)| name));
        let extra: Vec<u32> = materials.iter().map(|&(_, e)| e).collect();
        ffi::set_geometry_materials(self.ffi_mut(), &bytes, &lengths, &extra);
    }

    /// The index of the active material.
    pub fn active_material(&self) -> i32 {
        ffi::geometry_active_material(&self.0)
    }

    pub fn set_active_material(&mut self, index: i32) {
        ffi::set_geometry_active_material(self.ffi_mut(), index);
    }

    /// Whether the default material needs updating. Saved in files from version 20.2.0.7.
    pub fn default_material_update_flag(&self) -> u8 {
        ffi::geometry_material_update_flag(&self.0)
    }

    pub fn set_default_material_update_flag(&mut self, flag: u8) {
        ffi::set_geometry_material_update_flag(self.ffi_mut(), flag);
    }

    /// The shader's name, or `None` if there's no shader.
    pub fn shader_name(&self) -> Option<&BStr> {
        ffi::geometry_has_shader(&self.0).then(|| ffi::geometry_shader_name(&self.0).as_bstr())
    }

    /// Sets the shader's name, or removes the shader with `None`.
    pub fn set_shader_name(&mut self, name: Option<impl AsRef<[u8]>>) {
        match name {
            Some(name) => ffi::set_geometry_shader(self.ffi_mut(), true, name.as_ref()),
            None => ffi::set_geometry_shader(self.ffi_mut(), false, &[]),
        }
    }

    /// The shader's implementation, saved only with a shader.
    pub fn shader_implementation(&self) -> u32 {
        ffi::geometry_shader_implementation(&self.0)
    }

    pub fn set_shader_implementation(&mut self, implementation: u32) {
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

impl NiLinesData {
    /// For each vertex, whether a line connects it to the next one.
    pub fn line_flags(&self) -> Vec<bool> {
        ffi::lines_data_flags(&self.0)
    }

    /// Sets the [line flags](Self::line_flags). Returns `false`, changing nothing, unless
    /// there's one per vertex.
    pub fn set_line_flags(&mut self, flags: &[bool]) -> bool {
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
        pub fn $set(&mut self, value: u16) {
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
    pub fn set_polygons(&mut self, polygons: &[ScreenPolygon], indices: &[u16]) -> bool {
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
    pub fn set_polygon_count(&mut self, count: u16) -> bool {
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
