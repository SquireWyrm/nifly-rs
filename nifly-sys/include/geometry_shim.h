#pragma once

#include <algorithm>
#include <cmath>
#include <cstring>

#include "Geometry.hpp"
#include "Particles.hpp"
#include "nifly-sys/src/geometry.rs.h"
#include "rust/cxx.h"
#include "transform.h"

// For each shape type T with direct base Parent, defines:
// - upcast_<snake>: T -> Parent. Implicit derived-to-base conversion, always succeeds.
// - downcast_<snake>: NiObject -> T. Returns null unless obj is a T or a subclass of T.
#define NIFLY_RS_SHAPE(T, Parent, snake) \
    inline const nifly::Parent& upcast_##snake(const nifly::T& obj) { \
        return obj; \
    } \
    inline const nifly::T* downcast_##snake(const nifly::NiObject& obj) { \
        return dynamic_cast<const nifly::T*>(&obj); \
    }

namespace nifly_rs {
NIFLY_RS_SHAPE(NiShape, NiAVObject, ni_shape)
NIFLY_RS_SHAPE(BSTriShape, NiShape, bs_tri_shape)
NIFLY_RS_SHAPE(BSSubIndexTriShape, BSTriShape, bs_sub_index_tri_shape)
NIFLY_RS_SHAPE(BSMeshLODTriShape, BSTriShape, bs_mesh_lod_tri_shape)
NIFLY_RS_SHAPE(BSDynamicTriShape, BSTriShape, bs_dynamic_tri_shape)
NIFLY_RS_SHAPE(BSGeometry, NiShape, bs_geometry)
NIFLY_RS_SHAPE(NiGeometry, NiShape, ni_geometry)
NIFLY_RS_SHAPE(NiTriBasedGeom, NiGeometry, ni_tri_based_geom)
NIFLY_RS_SHAPE(NiTriShape, NiTriBasedGeom, ni_tri_shape)
NIFLY_RS_SHAPE(NiTriStrips, NiTriBasedGeom, ni_tri_strips)
NIFLY_RS_SHAPE(NiLines, NiTriBasedGeom, ni_lines)
NIFLY_RS_SHAPE(BSLODTriShape, NiTriBasedGeom, bs_lod_tri_shape)
NIFLY_RS_SHAPE(NiScreenElements, NiTriShape, ni_screen_elements)
NIFLY_RS_SHAPE(BSSegmentedTriShape, NiTriShape, bs_segmented_tri_shape)
NIFLY_RS_SHAPE(NiParticles, NiGeometry, ni_particles)
NIFLY_RS_SHAPE(NiAutoNormalParticles, NiParticles, ni_auto_normal_particles)
NIFLY_RS_SHAPE(NiParticleMeshes, NiParticles, ni_particle_meshes)
NIFLY_RS_SHAPE(NiRotatingParticles, NiParticles, ni_rotating_particles)

NIFLY_RS_SHAPE(NiGeometryData, NiObject, ni_geometry_data)
NIFLY_RS_SHAPE(NiTriBasedGeomData, NiGeometryData, ni_tri_based_geom_data)
NIFLY_RS_SHAPE(NiTriShapeData, NiTriBasedGeomData, ni_tri_shape_data)
NIFLY_RS_SHAPE(NiTriStripsData, NiTriBasedGeomData, ni_tri_strips_data)
NIFLY_RS_SHAPE(NiLinesData, NiGeometryData, ni_lines_data)
NIFLY_RS_SHAPE(NiScreenElementsData, NiTriShapeData, ni_screen_elements_data)
NIFLY_RS_SHAPE(BSGeometryMeshData, NiGeometryData, bs_geometry_mesh_data)
NIFLY_RS_SHAPE(AdditionalGeomData, NiObject, additional_geom_data)
NIFLY_RS_SHAPE(
    NiAdditionalGeometryData,
    AdditionalGeomData,
    ni_additional_geometry_data
)
NIFLY_RS_SHAPE(
    BSPackedAdditionalGeometryData,
    AdditionalGeomData,
    bs_packed_additional_geometry_data
)

// GetNumVertices is 16 bits; a BSGeometryMeshData's count is 32, and it
// doesn't override GetNumTriangles.
inline uint32_t geometry_data_vertex_count(const nifly::NiGeometryData& d) {
    if (auto mesh = dynamic_cast<const nifly::BSGeometryMeshData*>(&d))
        return static_cast<uint32_t>(mesh->vertices.size());
    return d.GetNumVertices();
}

inline uint32_t geometry_data_triangle_count(const nifly::NiGeometryData& d) {
    if (auto mesh = dynamic_cast<const nifly::BSGeometryMeshData*>(&d))
        return static_cast<uint32_t>(mesh->tris.size());
    return d.GetNumTriangles();
}

inline bool geometry_data_has_vertices(const nifly::NiGeometryData& d) {
    return d.HasVertices();
}

inline bool geometry_data_has_normals(const nifly::NiGeometryData& d) {
    return d.HasNormals();
}

inline bool geometry_data_has_vertex_colors(const nifly::NiGeometryData& d) {
    return d.HasVertexColors();
}

inline bool geometry_data_has_uvs(const nifly::NiGeometryData& d) {
    return d.HasUVs();
}

inline bool geometry_data_has_tangents(const nifly::NiGeometryData& d) {
    return d.HasTangents();
}

inline BoundingSphere geometry_data_bounds(const nifly::NiGeometryData& d) {
    return to_ffi(d.GetBounds());
}

inline void
set_geometry_data_bounds(nifly::NiGeometryData& d, BoundingSphere bounds) {
    d.SetBounds(from_ffi(bounds));
}

inline void update_geometry_data_bounds(nifly::NiGeometryData& d) {
    d.UpdateBounds();
}

inline uint16_t geometry_data_consistency(const nifly::NiGeometryData& d) {
    return d.consistencyFlags;
}

inline void
set_geometry_data_consistency(nifly::NiGeometryData& d, uint16_t flags) {
    d.consistencyFlags = static_cast<nifly::ConsistencyType>(flags);
}

// BSLODTriShape level0..2 and BSMeshLODTriShape lodSize0..2. `out` and `v` have 3
// elements.
inline void
lod_tri_shape_sizes(const nifly::BSLODTriShape& s, rust::Slice<uint32_t> out) {
    out[0] = s.level0;
    out[1] = s.level1;
    out[2] = s.level2;
}

inline void set_lod_tri_shape_sizes(
    nifly::BSLODTriShape& s,
    rust::Slice<const uint32_t> v
) {
    s.level0 = v[0];
    s.level1 = v[1];
    s.level2 = v[2];
}

inline void mesh_lod_tri_shape_sizes(
    const nifly::BSMeshLODTriShape& s,
    rust::Slice<uint32_t> out
) {
    out[0] = s.lodSize0;
    out[1] = s.lodSize1;
    out[2] = s.lodSize2;
}

inline void set_mesh_lod_tri_shape_sizes(
    nifly::BSMeshLODTriShape& s,
    rust::Slice<const uint32_t> v
) {
    s.lodSize0 = v[0];
    s.lodSize1 = v[1];
    s.lodSize2 = v[2];
}

// BSSegmentedTriShape segments.
inline rust::Vec<GeometrySegment>
segmented_tri_shape_segments(const nifly::BSSegmentedTriShape& s) {
    rust::Vec<GeometrySegment> out;
    for (const auto& seg : s.GetSegments())
        out.push_back({seg.flags, seg.index, seg.numTris});
    return out;
}

// Replaces the segments. False, changing nothing, if a segment's triangles
// (`index` counts triangle corners, 3 per triangle) go past the shape's.
inline bool set_segmented_tri_shape_segments(
    nifly::BSSegmentedTriShape& s,
    rust::Slice<const GeometrySegment> segments
) {
    const uint64_t corners = uint64_t(s.GetNumTriangles()) * 3;
    std::vector<nifly::BSGeometrySegmentData> data;
    for (const auto& seg : segments) {
        if (uint64_t(seg.index) + uint64_t(seg.triangle_count) * 3 > corners)
            return false;
        nifly::BSGeometrySegmentData d;
        d.flags = seg.flags;
        d.index = seg.index;
        d.numTris = seg.triangle_count;
        data.push_back(d);
    }
    s.SetSegments(data);
    return true;
}

// NiLinesData::lineFlags, one per vertex.
inline rust::Vec<bool> lines_data_flags(const nifly::NiLinesData& d) {
    rust::Vec<bool> out;
    for (const auto& f : d.lineFlags)
        out.push_back(static_cast<bool>(f));
    return out;
}

// False, changing nothing, unless there's one flag per vertex.
inline bool
set_lines_data_flags(nifly::NiLinesData& d, rust::Slice<const bool> flags) {
    if (flags.size() != d.GetNumVertices())
        return false;
    d.lineFlags.assign(flags.begin(), flags.end());
    return true;
}

// NiScreenElementsData's fields are protected; pointers to members taken through
// a subclass reach them without instantiating it.
struct ScreenElementsAccess: nifly::NiScreenElementsData {
    using NiScreenElementsData::indicesGrowBy;
    using NiScreenElementsData::maxIndices;
    using NiScreenElementsData::maxPolygons;
    using NiScreenElementsData::maxVertices;
    using NiScreenElementsData::numPolygons;
    using NiScreenElementsData::polygonGrowBy;
    using NiScreenElementsData::polygonIndices;
    using NiScreenElementsData::polygons;
    using NiScreenElementsData::verticesGrowBy;
};

inline rust::Vec<ScreenPolygon>
screen_elements_polygons(const nifly::NiScreenElementsData& d) {
    rust::Vec<ScreenPolygon> out;
    for (const auto& p : d.*(&ScreenElementsAccess::polygons))
        out.push_back(
            {p.numVertices, p.vertexOffset, p.numTriangles, p.triangleOffset}
        );
    return out;
}

inline rust::Vec<uint16_t>
screen_elements_polygon_indices(const nifly::NiScreenElementsData& d) {
    rust::Vec<uint16_t> out;
    for (uint16_t i : d.*(&ScreenElementsAccess::polygonIndices))
        out.push_back(i);
    return out;
}

// Replaces the polygons and their indices, keeping the stored count in step.
// False, changing nothing, unless both have the same length, at most 65535.
inline bool set_screen_elements_polygons(
    nifly::NiScreenElementsData& d,
    rust::Slice<const ScreenPolygon> polygons,
    rust::Slice<const uint16_t> indices
) {
    if (polygons.size() != indices.size() || polygons.size() > 0xFFFF)
        return false;
    auto& out = d.*(&ScreenElementsAccess::polygons);
    out.clear();
    for (const auto& p : polygons) {
        nifly::PolygonInfo info;
        info.numVertices = p.vertex_count;
        info.vertexOffset = p.vertex_offset;
        info.numTriangles = p.triangle_count;
        info.triangleOffset = p.triangle_offset;
        out.push_back(info);
    }
    d.*(&ScreenElementsAccess::polygonIndices) =
        std::vector<uint16_t>(indices.begin(), indices.end());
    d.*(&ScreenElementsAccess::maxPolygons) =
        static_cast<uint16_t>(polygons.size());
    return true;
}

#define NIFLY_RS_SCREEN_FIELD(field, snake) \
    inline uint16_t screen_elements_##snake( \
        const nifly::NiScreenElementsData& d \
    ) { \
        return d.*(&ScreenElementsAccess::field); \
    } \
    inline void set_screen_elements_##snake( \
        nifly::NiScreenElementsData& d, \
        uint16_t v \
    ) { \
        d.*(&ScreenElementsAccess::field) = v; \
    }
NIFLY_RS_SCREEN_FIELD(numPolygons, polygon_count)
NIFLY_RS_SCREEN_FIELD(polygonGrowBy, polygon_grow_by)
NIFLY_RS_SCREEN_FIELD(maxVertices, max_vertices)
NIFLY_RS_SCREEN_FIELD(verticesGrowBy, vertices_grow_by)
NIFLY_RS_SCREEN_FIELD(maxIndices, max_indices)
NIFLY_RS_SCREEN_FIELD(indicesGrowBy, indices_grow_by)
#undef NIFLY_RS_SCREEN_FIELD
} // namespace nifly_rs

// The rest of the shims, by topic.
// In this order: later parts use earlier ones.
// clang-format off
#include "geometry_shim_additional_data.h"
#include "geometry_shim_data.h"
#include "geometry_shim_bs_tri_shape.h"
#include "geometry_shim_bs_geometry.h"
// clang-format on

#undef NIFLY_RS_SHAPE
