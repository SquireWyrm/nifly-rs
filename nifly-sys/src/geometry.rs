//! `NiShape` and its subclasses. See `nodes.rs` for the upcast/downcast convention.

#[cxx::bridge]
pub mod ffi {
    /// A `BSGeometrySegmentData`.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct GeometrySegment {
        flags: u8,
        index: u32,
        triangle_count: u32,
    }

    /// A `PolygonInfo`.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct ScreenPolygon {
        vertex_count: u16,
        vertex_offset: u16,
        triangle_count: u16,
        triangle_offset: u16,
    }

    /// An `AdditionalDataInfo`.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct AdditionalDataChannel {
        data_type: i32,
        unit_size: u32,
        total_size: u32,
        stride: u32,
        block: u32,
        offset: u32,
        flags: u8,
    }

    #[namespace = "nifly"]
    unsafe extern "C++" {
        include!("Geometry.hpp");
        include!("Particles.hpp");

        type NiShape;
        type BSTriShape;
        type BSSubIndexTriShape;
        type BSMeshLODTriShape;
        type BSDynamicTriShape;
        type BSGeometry;
        type NiGeometry;
        type NiTriBasedGeom;
        type NiTriShape;
        type NiTriStrips;
        type NiLines;
        type BSLODTriShape;
        type NiScreenElements;
        type BSSegmentedTriShape;
        type NiParticles;
        type NiAutoNormalParticles;
        type NiParticleMeshes;
        type NiRotatingParticles;
        type NiGeometryData;
        type NiTriBasedGeomData;
        type NiTriShapeData;
        type NiTriStripsData;
        type NiLinesData;
        type NiScreenElementsData;
        type BSGeometryMeshData;
        type AdditionalGeomData;
        type NiAdditionalGeometryData;
        type BSPackedAdditionalGeometryData;
    }

    extern "C++" {
        include!("nifly-sys/src/object_3d.rs.h");

        type BoundingSphere = crate::object_3d::ffi::BoundingSphere;
        type Vec3 = crate::object_3d::ffi::Vec3;
    }

    #[namespace = "nifly"]
    extern "C++" {
        include!("Objects.hpp");

        type NiObject = crate::objects::ffi::NiObject;
        type NiAVObject = crate::objects::ffi::NiAVObject;
    }

    #[namespace = "nifly_rs"]
    unsafe extern "C++" {
        include!("geometry_shim.h");

        fn upcast_ni_shape(obj: &NiShape) -> &NiAVObject;
        fn downcast_ni_shape(obj: &NiObject) -> *const NiShape;
        fn upcast_bs_tri_shape(obj: &BSTriShape) -> &NiShape;
        fn downcast_bs_tri_shape(obj: &NiObject) -> *const BSTriShape;
        fn upcast_bs_sub_index_tri_shape(obj: &BSSubIndexTriShape) -> &BSTriShape;
        fn downcast_bs_sub_index_tri_shape(obj: &NiObject) -> *const BSSubIndexTriShape;
        fn upcast_bs_mesh_lod_tri_shape(obj: &BSMeshLODTriShape) -> &BSTriShape;
        fn downcast_bs_mesh_lod_tri_shape(obj: &NiObject) -> *const BSMeshLODTriShape;
        fn upcast_bs_dynamic_tri_shape(obj: &BSDynamicTriShape) -> &BSTriShape;
        fn downcast_bs_dynamic_tri_shape(obj: &NiObject) -> *const BSDynamicTriShape;
        fn upcast_bs_geometry(obj: &BSGeometry) -> &NiShape;
        fn downcast_bs_geometry(obj: &NiObject) -> *const BSGeometry;
        fn upcast_ni_geometry(obj: &NiGeometry) -> &NiShape;
        fn downcast_ni_geometry(obj: &NiObject) -> *const NiGeometry;
        fn upcast_ni_tri_based_geom(obj: &NiTriBasedGeom) -> &NiGeometry;
        fn downcast_ni_tri_based_geom(obj: &NiObject) -> *const NiTriBasedGeom;
        fn upcast_ni_tri_shape(obj: &NiTriShape) -> &NiTriBasedGeom;
        fn downcast_ni_tri_shape(obj: &NiObject) -> *const NiTriShape;
        fn upcast_ni_tri_strips(obj: &NiTriStrips) -> &NiTriBasedGeom;
        fn downcast_ni_tri_strips(obj: &NiObject) -> *const NiTriStrips;
        fn upcast_ni_lines(obj: &NiLines) -> &NiTriBasedGeom;
        fn downcast_ni_lines(obj: &NiObject) -> *const NiLines;
        fn upcast_bs_lod_tri_shape(obj: &BSLODTriShape) -> &NiTriBasedGeom;
        fn downcast_bs_lod_tri_shape(obj: &NiObject) -> *const BSLODTriShape;
        fn upcast_ni_screen_elements(obj: &NiScreenElements) -> &NiTriShape;
        fn downcast_ni_screen_elements(obj: &NiObject) -> *const NiScreenElements;
        fn upcast_bs_segmented_tri_shape(obj: &BSSegmentedTriShape) -> &NiTriShape;
        fn downcast_bs_segmented_tri_shape(obj: &NiObject) -> *const BSSegmentedTriShape;
        fn upcast_ni_particles(obj: &NiParticles) -> &NiGeometry;
        fn downcast_ni_particles(obj: &NiObject) -> *const NiParticles;
        fn upcast_ni_auto_normal_particles(obj: &NiAutoNormalParticles) -> &NiParticles;
        fn downcast_ni_auto_normal_particles(obj: &NiObject) -> *const NiAutoNormalParticles;
        fn upcast_ni_particle_meshes(obj: &NiParticleMeshes) -> &NiParticles;
        fn downcast_ni_particle_meshes(obj: &NiObject) -> *const NiParticleMeshes;
        fn upcast_ni_rotating_particles(obj: &NiRotatingParticles) -> &NiParticles;
        fn downcast_ni_rotating_particles(obj: &NiObject) -> *const NiRotatingParticles;

        fn upcast_ni_geometry_data(obj: &NiGeometryData) -> &NiObject;
        fn downcast_ni_geometry_data(obj: &NiObject) -> *const NiGeometryData;
        fn upcast_ni_tri_based_geom_data(obj: &NiTriBasedGeomData) -> &NiGeometryData;
        fn downcast_ni_tri_based_geom_data(obj: &NiObject) -> *const NiTriBasedGeomData;
        fn upcast_ni_tri_shape_data(obj: &NiTriShapeData) -> &NiTriBasedGeomData;
        fn downcast_ni_tri_shape_data(obj: &NiObject) -> *const NiTriShapeData;
        fn upcast_ni_tri_strips_data(obj: &NiTriStripsData) -> &NiTriBasedGeomData;
        fn downcast_ni_tri_strips_data(obj: &NiObject) -> *const NiTriStripsData;
        fn upcast_ni_lines_data(obj: &NiLinesData) -> &NiGeometryData;
        fn downcast_ni_lines_data(obj: &NiObject) -> *const NiLinesData;
        fn upcast_ni_screen_elements_data(obj: &NiScreenElementsData) -> &NiTriShapeData;
        fn downcast_ni_screen_elements_data(obj: &NiObject) -> *const NiScreenElementsData;
        fn upcast_bs_geometry_mesh_data(obj: &BSGeometryMeshData) -> &NiGeometryData;
        fn downcast_bs_geometry_mesh_data(obj: &NiObject) -> *const BSGeometryMeshData;

        /// `NiGeometryData::GetNumVertices`, or a `BSGeometryMeshData`'s 32-bit count.
        fn geometry_data_vertex_count(d: &NiGeometryData) -> u32;
        /// `NiGeometryData::GetNumTriangles` (virtual; 0 for types without triangles), or a
        /// `BSGeometryMeshData`'s triangle count.
        fn geometry_data_triangle_count(d: &NiGeometryData) -> u32;
        /// `NiGeometryData::HasVertices` and friends.
        fn geometry_data_has_vertices(d: &NiGeometryData) -> bool;
        fn geometry_data_has_normals(d: &NiGeometryData) -> bool;
        fn geometry_data_has_vertex_colors(d: &NiGeometryData) -> bool;
        fn geometry_data_has_uvs(d: &NiGeometryData) -> bool;
        fn geometry_data_has_tangents(d: &NiGeometryData) -> bool;
        /// `NiGeometryData::GetBounds`, `SetBounds` and `UpdateBounds`.
        fn geometry_data_bounds(d: &NiGeometryData) -> BoundingSphere;
        fn set_geometry_data_bounds(d: Pin<&mut NiGeometryData>, bounds: BoundingSphere);
        fn update_geometry_data_bounds(d: Pin<&mut NiGeometryData>);
        /// `NiGeometryData::consistencyFlags`, as the raw `ConsistencyType` value.
        fn geometry_data_consistency(d: &NiGeometryData) -> u16;
        fn set_geometry_data_consistency(d: Pin<&mut NiGeometryData>, flags: u16);

        /// `BSLODTriShape::level0..2` and `BSMeshLODTriShape::lodSize0..2`; slices have 3
        /// elements.
        fn lod_tri_shape_sizes(s: &BSLODTriShape, out: &mut [u32]);
        fn set_lod_tri_shape_sizes(s: Pin<&mut BSLODTriShape>, v: &[u32]);
        fn mesh_lod_tri_shape_sizes(s: &BSMeshLODTriShape, out: &mut [u32]);
        fn set_mesh_lod_tri_shape_sizes(s: Pin<&mut BSMeshLODTriShape>, v: &[u32]);
        /// `BSSegmentedTriShape::GetSegments` and `SetSegments`, the setter checking ranges.
        fn segmented_tri_shape_segments(s: &BSSegmentedTriShape) -> Vec<GeometrySegment>;
        fn set_segmented_tri_shape_segments(
            s: Pin<&mut BSSegmentedTriShape>,
            segments: &[GeometrySegment],
        ) -> bool;
        /// `NiLinesData::lineFlags`; the setter needs one per vertex.
        fn lines_data_flags(d: &NiLinesData) -> Vec<bool>;
        fn set_lines_data_flags(d: Pin<&mut NiLinesData>, flags: &[bool]) -> bool;
        /// `NiScreenElementsData`'s protected fields.
        fn screen_elements_polygons(d: &NiScreenElementsData) -> Vec<ScreenPolygon>;
        fn screen_elements_polygon_indices(d: &NiScreenElementsData) -> Vec<u16>;
        fn set_screen_elements_polygons(
            d: Pin<&mut NiScreenElementsData>,
            polygons: &[ScreenPolygon],
            indices: &[u16],
        ) -> bool;
        fn screen_elements_polygon_count(d: &NiScreenElementsData) -> u16;
        fn set_screen_elements_polygon_count(d: Pin<&mut NiScreenElementsData>, v: u16);
        fn screen_elements_polygon_grow_by(d: &NiScreenElementsData) -> u16;
        fn set_screen_elements_polygon_grow_by(d: Pin<&mut NiScreenElementsData>, v: u16);
        fn screen_elements_max_vertices(d: &NiScreenElementsData) -> u16;
        fn set_screen_elements_max_vertices(d: Pin<&mut NiScreenElementsData>, v: u16);
        fn screen_elements_vertices_grow_by(d: &NiScreenElementsData) -> u16;
        fn set_screen_elements_vertices_grow_by(d: Pin<&mut NiScreenElementsData>, v: u16);
        fn screen_elements_max_indices(d: &NiScreenElementsData) -> u16;
        fn set_screen_elements_max_indices(d: Pin<&mut NiScreenElementsData>, v: u16);
        fn screen_elements_indices_grow_by(d: &NiScreenElementsData) -> u16;
        fn set_screen_elements_indices_grow_by(d: Pin<&mut NiScreenElementsData>, v: u16);

        fn upcast_additional_geom_data(obj: &AdditionalGeomData) -> &NiObject;
        fn downcast_additional_geom_data(obj: &NiObject) -> *const AdditionalGeomData;
        fn upcast_ni_additional_geometry_data(
            obj: &NiAdditionalGeometryData,
        ) -> &AdditionalGeomData;
        fn downcast_ni_additional_geometry_data(obj: &NiObject) -> *const NiAdditionalGeometryData;
        fn upcast_bs_packed_additional_geometry_data(
            obj: &BSPackedAdditionalGeometryData,
        ) -> &AdditionalGeomData;
        fn downcast_bs_packed_additional_geometry_data(
            obj: &NiObject,
        ) -> *const BSPackedAdditionalGeometryData;
        /// `numVertices` and `blockInfos` of either type.
        fn agd_vertex_count(d: &AdditionalGeomData) -> u16;
        fn set_agd_vertex_count(d: Pin<&mut AdditionalGeomData>, v: u16);
        fn agd_channels(d: &AdditionalGeomData) -> Vec<AdditionalDataChannel>;
        fn set_agd_channels(d: Pin<&mut AdditionalGeomData>, channels: &[AdditionalDataChannel]);
        /// `NiAdditionalGeometryData::blocks`. `i` and `j` must be in range. Pushed blocks'
        /// `data` is `data_sizes.len()` items of `block_size` bytes; the caller checks that.
        fn agd_block_count(d: &NiAdditionalGeometryData) -> usize;
        fn agd_block_has_data(d: &NiAdditionalGeometryData, i: usize) -> bool;
        fn agd_block_size(d: &NiAdditionalGeometryData, i: usize) -> u32;
        fn agd_block_offsets(d: &NiAdditionalGeometryData, i: usize) -> &[u32];
        fn agd_block_data_sizes(d: &NiAdditionalGeometryData, i: usize) -> &[u32];
        fn agd_block_data(d: &NiAdditionalGeometryData, i: usize, j: usize) -> &[u8];
        fn clear_agd_blocks(d: Pin<&mut NiAdditionalGeometryData>);
        fn push_agd_block(
            d: Pin<&mut NiAdditionalGeometryData>,
            has_data: bool,
            block_size: u32,
            offsets: &[u32],
            data_sizes: &[u32],
            data: &[u8],
        );
        /// `BSPackedAdditionalGeometryData::blocks`. `i` must be in range.
        fn packed_agd_block_count(d: &BSPackedAdditionalGeometryData) -> usize;
        fn packed_agd_block_has_data(d: &BSPackedAdditionalGeometryData, i: usize) -> bool;
        fn packed_agd_block_offsets(d: &BSPackedAdditionalGeometryData, i: usize) -> &[u32];
        fn packed_agd_block_atom_sizes(d: &BSPackedAdditionalGeometryData, i: usize) -> &[u32];
        fn packed_agd_block_data(d: &BSPackedAdditionalGeometryData, i: usize) -> &[u8];
        /// `unkInt1` and `numTotalBytesPerElement`.
        fn packed_agd_block_ints(d: &BSPackedAdditionalGeometryData, i: usize, out: &mut [u32]);
        fn clear_packed_agd_blocks(d: Pin<&mut BSPackedAdditionalGeometryData>);
        #[allow(clippy::too_many_arguments)]
        fn push_packed_agd_block(
            d: Pin<&mut BSPackedAdditionalGeometryData>,
            has_data: bool,
            offsets: &[u32],
            atom_sizes: &[u32],
            data: &[u8],
            shader_index: u32,
            total_size: u32,
        );
        /// `NiGeometryData::additionalDataRef`, a block index (`NIF_NPOS` if empty).
        fn geometry_data_additional_data(d: &NiGeometryData) -> u32;

        /// `NiGeometryData`'s arrays: `which` is 0 vertices, 1 normals, 2 tangents,
        /// 3 bitangents. The setter needs one per vertex and enables normals or tangents.
        fn geometry_data_vectors(d: &NiGeometryData, which: u8) -> Vec<Vec3>;
        fn set_geometry_data_vectors(
            d: Pin<&mut NiGeometryData>,
            which: u8,
            values: &[Vec3],
        ) -> bool;
        /// `vertexColors`, 4 floats each.
        fn geometry_data_colors(d: &NiGeometryData) -> &[f32];
        fn set_geometry_data_colors(d: Pin<&mut NiGeometryData>, flat: &[f32]) -> bool;
        /// `SetNormals`, `SetVertexColors` or `SetTangents` for `which` 0, 1 or 2.
        fn set_geometry_data_has(d: Pin<&mut NiGeometryData>, which: u8, enable: bool);
        /// `uvSets`, 2 floats per UV. The setter takes `count` sets and updates `dataFlags`.
        fn geometry_data_uv_set_count(d: &NiGeometryData) -> usize;
        fn geometry_data_uv_set(d: &NiGeometryData, i: usize) -> &[f32];
        fn set_geometry_data_uv_sets(
            d: Pin<&mut NiGeometryData>,
            flat: &[f32],
            count: usize,
            bethesda: bool,
        ) -> bool;
        /// `dataFlags`; the setter resizes the UV sets and tangents to match.
        fn geometry_data_flags(d: &NiGeometryData) -> u16;
        fn set_geometry_data_flags(d: Pin<&mut NiGeometryData>, flags: u16, bethesda: bool);
        /// `groupID`, `keepFlags` (`which` 0), `compressFlags` (1) and `materialCRC`.
        fn geometry_data_group_id(d: &NiGeometryData) -> i32;
        fn set_geometry_data_group_id(d: Pin<&mut NiGeometryData>, v: i32);
        fn geometry_data_byte(d: &NiGeometryData, which: u8) -> u8;
        fn set_geometry_data_byte(d: Pin<&mut NiGeometryData>, which: u8, v: u8);
        fn geometry_data_material_crc(d: &NiGeometryData) -> u32;
        fn set_geometry_data_material_crc(d: Pin<&mut NiGeometryData>, v: u32);

        /// `NiTriShapeData`'s match groups and `NiTriStripsData`'s strips. Setters take the
        /// lists joined, with their lengths, and check counts and vertex indices.
        fn tri_shape_data_match_group_count(d: &NiTriShapeData) -> usize;
        fn tri_shape_data_match_group(d: &NiTriShapeData, i: usize) -> &[u16];
        fn set_tri_shape_data_match_groups(
            d: Pin<&mut NiTriShapeData>,
            flat: &[u16],
            lengths: &[usize],
        ) -> bool;
        fn tri_strips_data_strip_count(d: &NiTriStripsData) -> usize;
        fn tri_strips_data_strip(d: &NiTriStripsData, i: usize) -> &[u16];
        fn set_tri_strips_data_strips(
            d: Pin<&mut NiTriStripsData>,
            flat: &[u16],
            lengths: &[usize],
        ) -> bool;

        /// `NiGeometry`'s `materialNames` and `materialExtraData`, `activeMaterial`,
        /// `defaultMatNeedsUpdateFlag`, and `shader`, `shaderName` and `implementation`.
        fn geometry_material_count(g: &NiGeometry) -> usize;
        fn geometry_material_name(g: &NiGeometry, i: usize) -> &[u8];
        fn geometry_material_extra_data(g: &NiGeometry) -> &[u32];
        fn set_geometry_materials(
            g: Pin<&mut NiGeometry>,
            bytes: &[u8],
            lengths: &[usize],
            extra_data: &[u32],
        );
        fn geometry_active_material(g: &NiGeometry) -> i32;
        fn set_geometry_active_material(g: Pin<&mut NiGeometry>, v: i32);
        fn geometry_material_update_flag(g: &NiGeometry) -> u8;
        fn set_geometry_material_update_flag(g: Pin<&mut NiGeometry>, v: u8);
        fn geometry_has_shader(g: &NiGeometry) -> bool;
        fn geometry_shader_name(g: &NiGeometry) -> &[u8];
        fn set_geometry_shader(g: Pin<&mut NiGeometry>, has: bool, name: &[u8]);
        fn geometry_shader_implementation(g: &NiGeometry) -> u32;
        fn set_geometry_shader_implementation(g: Pin<&mut NiGeometry>, v: u32);

        /// `BSTriShape`'s bounds and `boundMinMax` (6 elements).
        fn bs_tri_shape_bounds(s: &BSTriShape) -> BoundingSphere;
        fn set_bs_tri_shape_bounds(s: Pin<&mut BSTriShape>, b: BoundingSphere);
        fn update_bs_tri_shape_bounds(s: Pin<&mut BSTriShape>);
        fn bs_tri_shape_bound_min_max(s: &BSTriShape, out: &mut [f32]);
        fn set_bs_tri_shape_bound_min_max(s: Pin<&mut BSTriShape>, v: &[f32]);
        /// The raw `vertexDesc` and its flags. The setter rejects layouts that can't be
        /// saved; `sse` is whether the file is Skyrim SE.
        fn bs_tri_shape_vertex_desc(s: &BSTriShape) -> u64;
        fn bs_tri_shape_vertex_flags(s: &BSTriShape) -> u16;
        fn set_bs_tri_shape_vertex_flags(s: Pin<&mut BSTriShape>, flags: u16, sse: bool) -> bool;
        /// Each vertex's `extra` floats, `per` per vertex.
        fn bs_tri_shape_vertex_count(s: &BSTriShape) -> usize;
        fn bs_tri_shape_extra_floats(s: &BSTriShape, per: &mut usize) -> Vec<f32>;
        fn set_bs_tri_shape_extra_floats(
            s: Pin<&mut BSTriShape>,
            flat: &[f32],
            per: usize,
            sse: bool,
        ) -> bool;
        /// The Skyrim SE particle data.
        fn bs_tri_shape_has_particle_data(s: &BSTriShape) -> bool;
        fn bs_tri_shape_particle_vectors(s: &BSTriShape, normals: bool) -> Vec<Vec3>;
        fn bs_tri_shape_particle_triangles(s: &BSTriShape) -> &[u16];
        fn set_bs_tri_shape_particle_data(
            s: Pin<&mut BSTriShape>,
            has: bool,
            vertices: &[Vec3],
            normals: &[Vec3],
            triangles: &[u16],
        ) -> bool;
        /// `BSSubIndexTriShape::GetSegments` and `SetSegments` (Skyrim SE), the setter
        /// checking ranges.
        fn sub_index_tri_shape_sse_segments(s: &BSSubIndexTriShape) -> Vec<GeometrySegment>;
        fn set_sub_index_tri_shape_sse_segments(
            s: Pin<&mut BSSubIndexTriShape>,
            segments: &[GeometrySegment],
        ) -> bool;

        /// `BSGeometry`'s bounds, `boundMinMax` (6 elements) and meshes. Mesh pointers are
        /// null if `i` is out of range. Mesh info is `triSize`, `numVerts` and `flags`.
        fn bs_geometry_bounds(g: &BSGeometry) -> BoundingSphere;
        fn set_bs_geometry_bounds(g: Pin<&mut BSGeometry>, b: BoundingSphere);
        fn bs_geometry_bound_min_max(g: &BSGeometry, out: &mut [f32]);
        fn set_bs_geometry_bound_min_max(g: Pin<&mut BSGeometry>, v: &[f32]);
        fn bs_geometry_mesh_count(g: &BSGeometry) -> usize;
        fn bs_geometry_mesh(g: &BSGeometry, i: usize) -> *const BSGeometryMeshData;
        fn bs_geometry_mesh_mut(g: Pin<&mut BSGeometry>, i: usize) -> *mut BSGeometryMeshData;
        fn bs_geometry_mesh_info(g: &BSGeometry, i: usize, out: &mut [u32]) -> bool;
        fn set_bs_geometry_mesh_info(g: Pin<&mut BSGeometry>, i: usize, v: &[u32]) -> bool;
        fn add_bs_geometry_mesh(g: Pin<&mut BSGeometry>) -> bool;

        /// `BSGeometryMeshData`'s own fields. Lists are flattened; setters check counts and
        /// vertex indices (see the shims).
        fn mesh_data_version(d: &BSGeometryMeshData) -> u32;
        fn set_mesh_data_version(d: Pin<&mut BSGeometryMeshData>, v: u32);
        fn mesh_data_scale(d: &BSGeometryMeshData) -> f32;
        fn set_mesh_data_scale(d: Pin<&mut BSGeometryMeshData>, v: f32);
        fn mesh_data_triangles(d: &BSGeometryMeshData) -> &[u16];
        fn set_mesh_data_triangles(d: Pin<&mut BSGeometryMeshData>, flat: &[u16]) -> bool;
        fn replace_mesh_data_vertices(d: Pin<&mut BSGeometryMeshData>, values: &[Vec3]) -> bool;
        fn mesh_data_uvs(d: &BSGeometryMeshData, which: usize) -> &[f32];
        fn set_mesh_data_uvs(d: Pin<&mut BSGeometryMeshData>, which: usize, flat: &[f32]) -> bool;
        fn mesh_data_colors(d: &BSGeometryMeshData) -> &[u8];
        fn set_mesh_data_colors(d: Pin<&mut BSGeometryMeshData>, flat: &[u8]) -> bool;
        fn mesh_data_tangent_ws(d: &BSGeometryMeshData) -> &[u8];
        fn set_mesh_data_tangent_ws(d: Pin<&mut BSGeometryMeshData>, ws: &[u8]) -> bool;
        fn mesh_data_weights_per_vertex(d: &BSGeometryMeshData) -> u32;
        fn mesh_data_skin_weights(d: &BSGeometryMeshData) -> Vec<u16>;
        fn set_mesh_data_skin_weights(
            d: Pin<&mut BSGeometryMeshData>,
            flat: &[u16],
            per: u32,
        ) -> bool;
        fn set_mesh_data_weights_per_vertex(d: Pin<&mut BSGeometryMeshData>, count: u32) -> bool;
        fn mesh_data_lod_count(d: &BSGeometryMeshData) -> usize;
        fn mesh_data_lod(d: &BSGeometryMeshData, i: usize) -> &[u16];
        fn set_mesh_data_lods(
            d: Pin<&mut BSGeometryMeshData>,
            flat: &[u16],
            lengths: &[usize],
        ) -> bool;
        fn mesh_data_meshlets(d: &BSGeometryMeshData) -> &[u32];
        fn set_mesh_data_meshlets(d: Pin<&mut BSGeometryMeshData>, flat: &[u32]);
        fn mesh_data_cull_data(d: &BSGeometryMeshData) -> Vec<Vec3>;
        fn set_mesh_data_cull_data(d: Pin<&mut BSGeometryMeshData>, flat: &[Vec3]);
        fn generate_mesh_data_meshlets(
            d: Pin<&mut BSGeometryMeshData>,
            max_vertices: u32,
            max_primitives: u32,
        ) -> bool;
    }
}
