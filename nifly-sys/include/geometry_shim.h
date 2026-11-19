#pragma once

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

// The two concrete AdditionalGeomData types have the same vertex count and
// channels, but no common base holding them.
template<typename F>
auto with_agd(const nifly::AdditionalGeomData& d, F f) {
    if (auto ni = dynamic_cast<const nifly::NiAdditionalGeometryData*>(&d))
        return f(*ni);
    return f(dynamic_cast<const nifly::BSPackedAdditionalGeometryData&>(d));
}

template<typename F>
void with_agd_mut(nifly::AdditionalGeomData& d, F f) {
    if (auto ni = dynamic_cast<nifly::NiAdditionalGeometryData*>(&d))
        f(*ni);
    else if (
        auto packed = dynamic_cast<nifly::BSPackedAdditionalGeometryData*>(&d)
    )
        f(*packed);
}

inline uint16_t agd_vertex_count(const nifly::AdditionalGeomData& d) {
    return with_agd(d, [](const auto& t) { return t.numVertices; });
}

inline void set_agd_vertex_count(nifly::AdditionalGeomData& d, uint16_t v) {
    with_agd_mut(d, [&](auto& t) { t.numVertices = v; });
}

inline rust::Vec<AdditionalDataChannel>
agd_channels(const nifly::AdditionalGeomData& d) {
    return with_agd(d, [](const auto& t) {
        rust::Vec<AdditionalDataChannel> out;
        const auto* infos = t.blockInfos.data();
        for (uint32_t i = 0; i < t.blockInfos.size(); ++i) {
            const auto& c = infos[i];
            out.push_back(
                {c.dataType,
                 c.numChannelBytesPerElement,
                 c.numChannelBytes,
                 c.numTotalBytesPerElement,
                 c.blockIndex,
                 c.channelOffset,
                 c.unkByte1}
            );
        }
        return out;
    });
}

inline void set_agd_channels(
    nifly::AdditionalGeomData& d,
    rust::Slice<const AdditionalDataChannel> channels
) {
    with_agd_mut(d, [&](auto& t) {
        t.blockInfos.clear();
        for (const auto& c : channels) {
            nifly::AdditionalDataInfo info;
            info.dataType = c.data_type;
            info.numChannelBytesPerElement = c.unit_size;
            info.numChannelBytes = c.total_size;
            info.numTotalBytesPerElement = c.stride;
            info.blockIndex = c.block;
            info.channelOffset = c.offset;
            info.unkByte1 = c.flags;
            t.blockInfos.push_back(info);
        }
    });
}

template<typename T>
rust::Slice<const T> slice_of(const std::vector<T>& v) {
    return {v.data(), v.size()};
}

inline size_t agd_block_count(const nifly::NiAdditionalGeometryData& d) {
    return d.blocks.size();
}

inline bool
agd_block_has_data(const nifly::NiAdditionalGeometryData& d, size_t i) {
    return d.blocks.data()[i].hasData;
}

inline uint32_t
agd_block_size(const nifly::NiAdditionalGeometryData& d, size_t i) {
    return d.blocks.data()[i].blockSize;
}

inline rust::Slice<const uint32_t>
agd_block_offsets(const nifly::NiAdditionalGeometryData& d, size_t i) {
    return slice_of(d.blocks.data()[i].blockOffsets);
}

inline rust::Slice<const uint32_t>
agd_block_data_sizes(const nifly::NiAdditionalGeometryData& d, size_t i) {
    return slice_of(d.blocks.data()[i].dataSizes);
}

inline rust::Slice<const uint8_t>
agd_block_data(const nifly::NiAdditionalGeometryData& d, size_t i, size_t j) {
    return slice_of(d.blocks.data()[i].data[j]);
}

inline void clear_agd_blocks(nifly::NiAdditionalGeometryData& d) {
    d.blocks.clear();
}

inline void push_agd_block(
    nifly::NiAdditionalGeometryData& d,
    bool has_data,
    uint32_t block_size,
    rust::Slice<const uint32_t> offsets,
    rust::Slice<const uint32_t> data_sizes,
    rust::Slice<const uint8_t> data
) {
    nifly::AdditionalDataBlock b;
    b.hasData = has_data;
    if (has_data) {
        b.blockSize = block_size;
        b.numBlocks = static_cast<uint32_t>(offsets.size());
        b.blockOffsets.assign(offsets.begin(), offsets.end());
        b.numData = static_cast<uint32_t>(data_sizes.size());
        b.dataSizes.assign(data_sizes.begin(), data_sizes.end());
        for (size_t j = 0; j < data_sizes.size(); ++j) {
            auto start = data.begin() + j * block_size;
            b.data.emplace_back(start, start + block_size);
        }
    }
    d.blocks.push_back(b);
}

inline size_t
packed_agd_block_count(const nifly::BSPackedAdditionalGeometryData& d) {
    return d.blocks.size();
}

inline bool packed_agd_block_has_data(
    const nifly::BSPackedAdditionalGeometryData& d,
    size_t i
) {
    return d.blocks.data()[i].hasData;
}

inline rust::Slice<const uint32_t> packed_agd_block_offsets(
    const nifly::BSPackedAdditionalGeometryData& d,
    size_t i
) {
    return slice_of(d.blocks.data()[i].blockOffsets);
}

inline rust::Slice<const uint32_t> packed_agd_block_atom_sizes(
    const nifly::BSPackedAdditionalGeometryData& d,
    size_t i
) {
    return slice_of(d.blocks.data()[i].atomSizes);
}

inline rust::Slice<const uint8_t> packed_agd_block_data(
    const nifly::BSPackedAdditionalGeometryData& d,
    size_t i
) {
    return slice_of(d.blocks.data()[i].data);
}

inline void packed_agd_block_ints(
    const nifly::BSPackedAdditionalGeometryData& d,
    size_t i,
    rust::Slice<uint32_t> out
) {
    out[0] = d.blocks.data()[i].unkInt1;
    out[1] = d.blocks.data()[i].numTotalBytesPerElement;
}

inline void clear_packed_agd_blocks(nifly::BSPackedAdditionalGeometryData& d) {
    d.blocks.clear();
}

inline void push_packed_agd_block(
    nifly::BSPackedAdditionalGeometryData& d,
    bool has_data,
    rust::Slice<const uint32_t> offsets,
    rust::Slice<const uint32_t> atom_sizes,
    rust::Slice<const uint8_t> data,
    uint32_t shader_index,
    uint32_t total_size
) {
    nifly::BSPackedAdditionalDataBlock b;
    b.hasData = has_data;
    if (has_data) {
        b.numTotalBytes = static_cast<uint32_t>(data.size());
        b.numBlocks = static_cast<uint32_t>(offsets.size());
        b.blockOffsets.assign(offsets.begin(), offsets.end());
        b.numAtoms = static_cast<uint32_t>(atom_sizes.size());
        b.atomSizes.assign(atom_sizes.begin(), atom_sizes.end());
        b.data.assign(data.begin(), data.end());
    }
    b.unkInt1 = shader_index;
    b.numTotalBytesPerElement = total_size;
    d.blocks.push_back(b);
}

inline uint32_t geometry_data_additional_data(const nifly::NiGeometryData& d) {
    return d.additionalDataRef.index;
}

// NiGeometryData's arrays: 0 vertices, 1 normals, 2 tangents, 3 bitangents.
inline std::vector<nifly::Vector3>&
geometry_data_array(const nifly::NiGeometryData& d, uint8_t which) {
    auto& m = const_cast<nifly::NiGeometryData&>(d);
    switch (which) {
        case 1:
            return m.normals;
        case 2:
            return m.tangents;
        case 3:
            return m.bitangents;
        default:
            return m.vertices;
    }
}

inline rust::Vec<Vec3>
geometry_data_vectors(const nifly::NiGeometryData& d, uint8_t which) {
    rust::Vec<Vec3> out;
    for (const auto& v : geometry_data_array(d, which))
        out.push_back({v.x, v.y, v.z});
    return out;
}

// Replaces an array, enabling normals or tangents if needed. False, changing
// nothing, unless there's one value per vertex.
inline bool set_geometry_data_vectors(
    nifly::NiGeometryData& d,
    uint8_t which,
    rust::Slice<const Vec3> values
) {
    if (values.size() != geometry_data_vertex_count(d))
        return false;
    if (which == 1 && !d.HasNormals())
        d.SetNormals(true);
    if ((which == 2 || which == 3) && !d.HasTangents())
        d.SetTangents(true);
    auto& out = geometry_data_array(d, which);
    out.clear();
    for (const auto& v : values)
        out.emplace_back(v.x, v.y, v.z);
    return true;
}

// Flattened, 4 floats per color.
inline rust::Slice<const float>
geometry_data_colors(const nifly::NiGeometryData& d) {
    return {
        reinterpret_cast<const float*>(d.vertexColors.data()),
        d.vertexColors.size() * 4
    };
}

inline bool set_geometry_data_colors(
    nifly::NiGeometryData& d,
    rust::Slice<const float> flat
) {
    if (flat.size() != size_t(geometry_data_vertex_count(d)) * 4)
        return false;
    d.SetVertexColors(true);
    for (size_t i = 0; i < d.vertexColors.size(); i++)
        d.vertexColors[i] = nifly::Color4(
            flat[i * 4],
            flat[i * 4 + 1],
            flat[i * 4 + 2],
            flat[i * 4 + 3]
        );
    return true;
}

// 0 normals, 1 vertex colors, 2 tangents.
inline void
set_geometry_data_has(nifly::NiGeometryData& d, uint8_t which, bool enable) {
    switch (which) {
        case 0:
            d.SetNormals(enable);
            break;
        case 1:
            d.SetVertexColors(enable);
            break;
        default:
            d.SetTangents(enable);
            break;
    }
}

inline size_t geometry_data_uv_set_count(const nifly::NiGeometryData& d) {
    return d.uvSets.size();
}

// Flattened, 2 floats per UV; empty if `i` is out of range.
inline rust::Slice<const float>
geometry_data_uv_set(const nifly::NiGeometryData& d, size_t i) {
    if (i >= d.uvSets.size())
        return {};
    return {
        reinterpret_cast<const float*>(d.uvSets[i].data()),
        d.uvSets[i].size() * 2
    };
}

// The UV set count bits of dataFlags: bit 0 from stream 34, else bits 0-5.
inline uint16_t uv_count_mask(bool bethesda) {
    return bethesda ? 0x1 : 0x3F;
}

// Replaces the UV sets (`count` sets, flattened) and the count in dataFlags.
// False, changing nothing, unless each set has one UV per vertex and the count
// fits in the flags.
inline bool set_geometry_data_uv_sets(
    nifly::NiGeometryData& d,
    rust::Slice<const float> flat,
    size_t count,
    bool bethesda
) {
    const size_t n = geometry_data_vertex_count(d);
    const uint16_t mask = uv_count_mask(bethesda);
    if (count > mask || flat.size() != count * n * 2)
        return false;
    d.SetUVs(count > 0);
    d.uvSets.assign(count, std::vector<nifly::Vector2>(n));
    for (size_t s = 0; s < count; s++)
        for (size_t i = 0; i < n; i++) {
            const float* uv = flat.data() + (s * n + i) * 2;
            d.uvSets[s][i] = nifly::Vector2(uv[0], uv[1]);
        }
    d.dataFlags = static_cast<uint16_t>((d.dataFlags & ~mask) | count);
    return true;
}

inline uint16_t geometry_data_flags(const nifly::NiGeometryData& d) {
    return d.dataFlags;
}

// Sets dataFlags, adding or removing UV sets and tangents to match.
inline void set_geometry_data_flags(
    nifly::NiGeometryData& d,
    uint16_t flags,
    bool bethesda
) {
    const size_t n = geometry_data_vertex_count(d);
    const size_t count = flags & uv_count_mask(bethesda);
    d.SetUVs(count > 0);
    d.uvSets.resize(count);
    for (auto& set : d.uvSets)
        set.resize(n);
    if (flags & 0xF000) {
        d.tangents.resize(n);
        d.bitangents.resize(n);
    } else {
        d.tangents.clear();
        d.bitangents.clear();
    }
    d.dataFlags = flags;
}

inline int32_t geometry_data_group_id(const nifly::NiGeometryData& d) {
    return d.groupID;
}

inline void set_geometry_data_group_id(nifly::NiGeometryData& d, int32_t v) {
    d.groupID = v;
}

// 0 keepFlags, 1 compressFlags.
inline uint8_t
geometry_data_byte(const nifly::NiGeometryData& d, uint8_t which) {
    return which ? d.compressFlags : d.keepFlags;
}

inline void
set_geometry_data_byte(nifly::NiGeometryData& d, uint8_t which, uint8_t v) {
    (which ? d.compressFlags : d.keepFlags) = v;
}

inline uint32_t geometry_data_material_crc(const nifly::NiGeometryData& d) {
    return d.materialCRC;
}

inline void
set_geometry_data_material_crc(nifly::NiGeometryData& d, uint32_t v) {
    d.materialCRC = v;
}

struct TriShapeDataAccess: nifly::NiTriShapeData {
    using NiTriShapeData::matchGroups;
};

struct TriBasedGeomDataAccess: nifly::NiTriBasedGeomData {
    using NiTriBasedGeomData::numTriangles;
};

inline size_t tri_shape_data_match_group_count(const nifly::NiTriShapeData& d) {
    return (d.*(&TriShapeDataAccess::matchGroups)).size();
}

// Empty if `i` is out of range.
inline rust::Slice<const uint16_t>
tri_shape_data_match_group(const nifly::NiTriShapeData& d, size_t i) {
    const auto& groups = d.*(&TriShapeDataAccess::matchGroups);
    if (i >= groups.size())
        return {};
    return slice_of(groups[i].matches);
}

// Splits `flat` into lists of `lengths`. False if a length or the list count
// is over 65535, a value isn't below `limit` or the lengths don't add up.
inline bool split_u16_lists(
    rust::Slice<const uint16_t> flat,
    rust::Slice<const size_t> lengths,
    size_t limit,
    std::vector<std::vector<uint16_t>>& out
) {
    if (lengths.size() > 0xFFFF)
        return false;
    size_t at = 0;
    for (size_t len : lengths) {
        if (len > 0xFFFF || len > flat.size() - at)
            return false;
        std::vector<uint16_t> list(flat.begin() + at, flat.begin() + at + len);
        for (uint16_t v : list)
            if (v >= limit)
                return false;
        out.push_back(std::move(list));
        at += len;
    }
    return at == flat.size();
}

// Replaces the match groups. False, changing nothing, if there are more than
// 65535 groups or entries in one, or a vertex is out of range.
inline bool set_tri_shape_data_match_groups(
    nifly::NiTriShapeData& d,
    rust::Slice<const uint16_t> flat,
    rust::Slice<const size_t> lengths
) {
    std::vector<std::vector<uint16_t>> lists;
    if (!split_u16_lists(flat, lengths, d.GetNumVertices(), lists))
        return false;
    std::vector<nifly::MatchGroup> groups;
    for (auto& list : lists) {
        nifly::MatchGroup g;
        g.count = static_cast<uint16_t>(list.size());
        g.matches = std::move(list);
        groups.push_back(std::move(g));
    }
    d.SetMatchGroups(groups);
    return true;
}

inline size_t tri_strips_data_strip_count(const nifly::NiTriStripsData& d) {
    return d.stripsInfo.points.size();
}

// Empty if `i` is out of range.
inline rust::Slice<const uint16_t>
tri_strips_data_strip(const nifly::NiTriStripsData& d, size_t i) {
    if (i >= d.stripsInfo.points.size())
        return {};
    return slice_of(d.stripsInfo.points[i]);
}

// Replaces the strips and the stored triangle count. False, changing nothing,
// if there are more than 65535 strips, points in one or triangles in all, or a
// vertex is out of range.
inline bool set_tri_strips_data_strips(
    nifly::NiTriStripsData& d,
    rust::Slice<const uint16_t> flat,
    rust::Slice<const size_t> lengths
) {
    std::vector<std::vector<uint16_t>> lists;
    if (!split_u16_lists(flat, lengths, d.GetNumVertices(), lists))
        return false;
    size_t triangles = 0;
    for (const auto& list : lists)
        if (list.size() > 2)
            triangles += list.size() - 2;
    if (triangles > 0xFFFF)
        return false;
    auto& info = d.stripsInfo;
    info.stripLengths.clear();
    for (const auto& list : lists) {
        uint16_t len = static_cast<uint16_t>(list.size());
        info.stripLengths.push_back(len);
    }
    info.points = std::move(lists);
    info.hasPoints = true;
    d.*(&TriBasedGeomDataAccess::numTriangles) =
        static_cast<uint16_t>(triangles);
    return true;
}

inline size_t geometry_material_count(const nifly::NiGeometry& g) {
    return g.materialNames.size();
}

// Empty if `i` is out of range.
inline rust::Slice<const uint8_t>
geometry_material_name(const nifly::NiGeometry& g, size_t i) {
    if (i >= g.materialNames.size())
        return {};
    const auto& s = g.materialNames.data()[i].get();
    return {reinterpret_cast<const uint8_t*>(s.data()), s.size()};
}

inline rust::Slice<const uint32_t>
geometry_material_extra_data(const nifly::NiGeometry& g) {
    return {g.materialExtraData.data(), g.materialExtraData.size()};
}

// Replaces the materials: names joined in `bytes`, one extra data value each.
inline void set_geometry_materials(
    nifly::NiGeometry& g,
    rust::Slice<const uint8_t> bytes,
    rust::Slice<const size_t> lengths,
    rust::Slice<const uint32_t> extra_data
) {
    g.materialNames.clear();
    g.materialExtraData.clear();
    size_t at = 0;
    for (size_t i = 0; i < lengths.size(); i++) {
        nifly::NiStringRef name(
            std::string(
                reinterpret_cast<const char*>(bytes.data()) + at,
                lengths[i]
            )
        );
        at += lengths[i];
        g.materialNames.push_back(name);
        uint32_t extra = extra_data[i];
        g.materialExtraData.push_back(extra);
    }
}

inline int32_t geometry_active_material(const nifly::NiGeometry& g) {
    return g.activeMaterial;
}

inline void set_geometry_active_material(nifly::NiGeometry& g, int32_t v) {
    g.activeMaterial = v;
}

inline uint8_t geometry_material_update_flag(const nifly::NiGeometry& g) {
    return g.defaultMatNeedsUpdateFlag;
}

inline void set_geometry_material_update_flag(nifly::NiGeometry& g, uint8_t v) {
    g.defaultMatNeedsUpdateFlag = v;
}

inline bool geometry_has_shader(const nifly::NiGeometry& g) {
    return g.shader;
}

inline rust::Slice<const uint8_t>
geometry_shader_name(const nifly::NiGeometry& g) {
    const auto& s = g.shaderName.get();
    return {reinterpret_cast<const uint8_t*>(s.data()), s.size()};
}

inline void set_geometry_shader(
    nifly::NiGeometry& g,
    bool has,
    rust::Slice<const uint8_t> name
) {
    g.shader = has;
    g.shaderName.get().assign(
        reinterpret_cast<const char*>(name.data()),
        name.size()
    );
}

inline uint32_t geometry_shader_implementation(const nifly::NiGeometry& g) {
    return g.implementation;
}

inline void
set_geometry_shader_implementation(nifly::NiGeometry& g, uint32_t v) {
    g.implementation = v;
}

inline BoundingSphere bs_tri_shape_bounds(const nifly::BSTriShape& s) {
    return to_ffi(s.GetBounds());
}

inline void set_bs_tri_shape_bounds(nifly::BSTriShape& s, BoundingSphere b) {
    s.SetBounds(from_ffi(b));
}

inline void update_bs_tri_shape_bounds(nifly::BSTriShape& s) {
    s.UpdateBounds();
}

struct BSTriShapeAccess: nifly::BSTriShape {
    using BSTriShape::boundMinMax;
};

// `out` and `v` have 6 elements.
inline void
bs_tri_shape_bound_min_max(const nifly::BSTriShape& s, rust::Slice<float> out) {
    const auto& m = s.*(&BSTriShapeAccess::boundMinMax);
    for (size_t i = 0; i < 6; i++)
        out[i] = m[i];
}

inline void set_bs_tri_shape_bound_min_max(
    nifly::BSTriShape& s,
    rust::Slice<const float> v
) {
    auto& m = s.*(&BSTriShapeAccess::boundMinMax);
    for (size_t i = 0; i < 6; i++)
        m[i] = v[i];
}

// VertexDesc keeps its 64 bits private.
inline uint64_t bs_tri_shape_vertex_desc(const nifly::BSTriShape& s) {
    uint64_t bits;
    std::memcpy(&bits, &s.vertexDesc, sizeof bits);
    return bits;
}

inline uint16_t bs_tri_shape_vertex_flags(const nifly::BSTriShape& s) {
    return static_cast<uint16_t>(bs_tri_shape_vertex_desc(s) >> 44);
}

inline size_t bs_tri_shape_extra_count(const nifly::BSTriShape& s) {
    return s.vertData.empty() ? 0 : s.vertData.front().extra.size();
}

// Whether vertices with `flags` and `extra` floats can be saved: extra floats
// need full-precision positions, and a vertex is at most 15 dwords. Skyrim SE
// (`sse`) always saves full-precision positions.
inline bool bs_layout_ok(uint16_t flags, size_t extra, bool sse) {
    auto has = [&](uint16_t f) { return (flags & f) != 0; };
    const bool full = has(nifly::VF_FULLPREC) || sse;
    if (extra > 0 && !(has(nifly::VF_VERTEX) && full))
        return false;
    size_t dwords = extra;
    if (has(nifly::VF_VERTEX))
        dwords += full ? 4 : 2;
    if (has(nifly::VF_UV))
        dwords++;
    if (has(nifly::VF_UV_2))
        dwords++;
    if (has(nifly::VF_NORMAL)) {
        dwords++;
        if (has(nifly::VF_TANGENT))
            dwords++;
    }
    if (has(nifly::VF_COLORS))
        dwords++;
    if (has(nifly::VF_SKINNED))
        dwords += 3;
    if (has(nifly::VF_EYEDATA))
        dwords++;
    return dwords <= 15;
}

// Sets the vertex flags. False, changing nothing, if they add second UVs (whose
// data isn't stored) or the layout can't be saved (see bs_layout_ok). Newly
// enabled colors start white.
inline bool
set_bs_tri_shape_vertex_flags(nifly::BSTriShape& s, uint16_t flags, bool sse) {
    const uint16_t old = bs_tri_shape_vertex_flags(s);
    if ((flags & nifly::VF_UV_2) && !(old & nifly::VF_UV_2))
        return false;
    if (!bs_layout_ok(flags, bs_tri_shape_extra_count(s), sse))
        return false;
    if ((flags & nifly::VF_COLORS) && !(old & nifly::VF_COLORS))
        s.SetVertexColors(true);
    uint64_t bits = bs_tri_shape_vertex_desc(s);
    bits = (bits & ~(uint64_t(0xFFFF) << 44)) | (uint64_t(flags) << 44);
    std::memcpy(&s.vertexDesc, &bits, sizeof bits);
    return true;
}

inline size_t bs_tri_shape_vertex_count(const nifly::BSTriShape& s) {
    return s.vertData.size();
}

// The extra floats of every vertex, flattened; `per` is the count per vertex.
inline rust::Vec<float>
bs_tri_shape_extra_floats(const nifly::BSTriShape& s, size_t& per) {
    per = bs_tri_shape_extra_count(s);
    rust::Vec<float> out;
    for (const auto& v : s.vertData)
        for (size_t i = 0; i < per; i++)
            out.push_back(i < v.extra.size() ? v.extra[i] : 0.0f);
    return out;
}

// Replaces the extra floats, `per` per vertex. False, changing nothing, unless
// there are `per` for each vertex and the layout can be saved.
inline bool set_bs_tri_shape_extra_floats(
    nifly::BSTriShape& s,
    rust::Slice<const float> flat,
    size_t per,
    bool sse
) {
    if (s.vertData.size() != s.GetNumVertices()
        || flat.size() != per * s.vertData.size())
        return false;
    if (!bs_layout_ok(bs_tri_shape_vertex_flags(s), per, sse))
        return false;
    for (size_t i = 0; i < s.vertData.size(); i++)
        s.vertData[i].extra.assign(
            flat.begin() + i * per,
            flat.begin() + (i + 1) * per
        );
    return true;
}

inline bool bs_tri_shape_has_particle_data(const nifly::BSTriShape& s) {
    return s.particleDataSize > 0;
}

inline rust::Vec<Vec3>
bs_tri_shape_particle_vectors(const nifly::BSTriShape& s, bool normals) {
    rust::Vec<Vec3> out;
    for (const auto& v : normals ? s.particleNorms : s.particleVerts)
        out.push_back({v.x, v.y, v.z});
    return out;
}

// Flattened, 3 per triangle.
inline rust::Slice<const uint16_t>
bs_tri_shape_particle_triangles(const nifly::BSTriShape& s) {
    return {
        reinterpret_cast<const uint16_t*>(s.particleTris.data()),
        s.particleTris.size() * 3
    };
}

// Replaces the particle data, or removes it if `has` is false. False, changing
// nothing, unless there's a position and normal per vertex and a triangle per
// triangle, using only the shape's vertices.
inline bool set_bs_tri_shape_particle_data(
    nifly::BSTriShape& s,
    bool has,
    rust::Slice<const Vec3> vertices,
    rust::Slice<const Vec3> normals,
    rust::Slice<const uint16_t> triangles
) {
    if (!has) {
        s.particleDataSize = 0;
        s.particleVerts.clear();
        s.particleNorms.clear();
        s.particleTris.clear();
        return true;
    }
    const size_t n = s.GetNumVertices();
    const size_t t = s.GetNumTriangles();
    if (vertices.size() != n || normals.size() != n
        || triangles.size() != t * 3)
        return false;
    for (uint16_t i : triangles)
        if (i >= n)
            return false;
    s.particleVerts.clear();
    s.particleNorms.clear();
    s.particleTris.clear();
    for (size_t i = 0; i < n; i++) {
        s.particleVerts
            .emplace_back(vertices[i].x, vertices[i].y, vertices[i].z);
        s.particleNorms.emplace_back(normals[i].x, normals[i].y, normals[i].z);
    }
    for (size_t i = 0; i < t; i++)
        s.particleTris.emplace_back(
            triangles[i * 3],
            triangles[i * 3 + 1],
            triangles[i * 3 + 2]
        );
    // Counted in 16-bit values: 6 per vertex, 3 per triangle.
    s.particleDataSize = static_cast<uint32_t>(n * 6 + t * 3);
    return true;
}

inline rust::Vec<GeometrySegment>
sub_index_tri_shape_sse_segments(const nifly::BSSubIndexTriShape& s) {
    rust::Vec<GeometrySegment> out;
    for (const auto& seg : s.GetSegments())
        out.push_back({seg.flags, seg.index, seg.numTris});
    return out;
}

// Replaces the Skyrim SE segments. False, changing nothing, if a segment's
// triangles go past the shape's.
inline bool set_sub_index_tri_shape_sse_segments(
    nifly::BSSubIndexTriShape& s,
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

struct BSGeometryAccess: nifly::BSGeometry {
    using BSGeometry::boundMinMax;
    using BSGeometry::bounds;
    using BSGeometry::meshes;
};

inline BoundingSphere bs_geometry_bounds(const nifly::BSGeometry& g) {
    return to_ffi(g.*(&BSGeometryAccess::bounds));
}

inline void set_bs_geometry_bounds(nifly::BSGeometry& g, BoundingSphere b) {
    g.*(&BSGeometryAccess::bounds) = from_ffi(b);
}

// `out` and `v` have 6 elements.
inline void
bs_geometry_bound_min_max(const nifly::BSGeometry& g, rust::Slice<float> out) {
    const auto& m = g.*(&BSGeometryAccess::boundMinMax);
    for (size_t i = 0; i < 6; i++)
        out[i] = m[i];
}

inline void set_bs_geometry_bound_min_max(
    nifly::BSGeometry& g,
    rust::Slice<const float> v
) {
    auto& m = g.*(&BSGeometryAccess::boundMinMax);
    for (size_t i = 0; i < 6; i++)
        m[i] = v[i];
}

inline size_t bs_geometry_mesh_count(const nifly::BSGeometry& g) {
    return (g.*(&BSGeometryAccess::meshes)).size();
}

// Null if `i` is out of range.
inline const nifly::BSGeometryMeshData*
bs_geometry_mesh(const nifly::BSGeometry& g, size_t i) {
    const auto& meshes = g.*(&BSGeometryAccess::meshes);
    return i < meshes.size() ? &meshes[i].meshData : nullptr;
}

inline nifly::BSGeometryMeshData*
bs_geometry_mesh_mut(nifly::BSGeometry& g, size_t i) {
    auto& meshes = g.*(&BSGeometryAccess::meshes);
    return i < meshes.size() ? &meshes[i].meshData : nullptr;
}

// triSize, numVerts and flags of mesh `i` (3 elements). False if out of range.
inline bool bs_geometry_mesh_info(
    const nifly::BSGeometry& g,
    size_t i,
    rust::Slice<uint32_t> out
) {
    const auto& meshes = g.*(&BSGeometryAccess::meshes);
    if (i >= meshes.size())
        return false;
    out[0] = meshes[i].triSize;
    out[1] = meshes[i].numVerts;
    out[2] = meshes[i].flags;
    return true;
}

inline bool set_bs_geometry_mesh_info(
    nifly::BSGeometry& g,
    size_t i,
    rust::Slice<const uint32_t> v
) {
    auto& meshes = g.*(&BSGeometryAccess::meshes);
    if (i >= meshes.size())
        return false;
    meshes[i].triSize = v[0];
    meshes[i].numVerts = v[1];
    meshes[i].flags = v[2];
    return true;
}

// Adds an empty mesh. False if there are already 4, the most a file stores.
inline bool add_bs_geometry_mesh(nifly::BSGeometry& g) {
    if (bs_geometry_mesh_count(g) >= 4)
        return false;
    g.AddMesh();
    g.ReleaseMesh();
    return true;
}

inline uint32_t mesh_data_version(const nifly::BSGeometryMeshData& d) {
    return d.version;
}

inline void set_mesh_data_version(nifly::BSGeometryMeshData& d, uint32_t v) {
    d.version = v;
}

inline float mesh_data_scale(const nifly::BSGeometryMeshData& d) {
    return d.scale;
}

inline void set_mesh_data_scale(nifly::BSGeometryMeshData& d, float v) {
    d.scale = v;
}

// Flattened, 3 per triangle.
inline rust::Slice<const uint16_t>
mesh_data_triangles(const nifly::BSGeometryMeshData& d) {
    return {
        reinterpret_cast<const uint16_t*>(d.tris.data()),
        d.tris.size() * 3
    };
}

inline bool
flat_triangles_in_range(rust::Slice<const uint16_t> flat, size_t n) {
    if (flat.size() % 3 != 0)
        return false;
    for (uint16_t i : flat)
        if (i >= n)
            return false;
    return true;
}

inline std::vector<nifly::Triangle>
to_triangles(rust::Slice<const uint16_t> flat) {
    std::vector<nifly::Triangle> out;
    for (size_t i = 0; i + 2 < flat.size(); i += 3)
        out.emplace_back(flat[i], flat[i + 1], flat[i + 2]);
    return out;
}

// False, changing nothing, unless every index is below the vertex count.
inline bool set_mesh_data_triangles(
    nifly::BSGeometryMeshData& d,
    rust::Slice<const uint16_t> flat
) {
    if (!flat_triangles_in_range(flat, d.vertices.size()))
        return false;
    d.tris = to_triangles(flat);
    d.nTriIndices = static_cast<uint32_t>(d.tris.size() * 3);
    return true;
}

// Replaces the positions. Per-vertex arrays that aren't empty are resized to
// match, new entries zeroed (colors white). False, changing nothing, if a
// triangle or LOD triangle would use a removed vertex.
inline bool replace_mesh_data_vertices(
    nifly::BSGeometryMeshData& d,
    rust::Slice<const Vec3> values
) {
    const size_t n = values.size();
    if (n > UINT32_MAX)
        return false;
    for (const auto& t : d.tris)
        if (t.p1 >= n || t.p2 >= n || t.p3 >= n)
            return false;
    for (const auto& lod : d.lods)
        for (const auto& t : lod)
            if (t.p1 >= n || t.p2 >= n || t.p3 >= n)
                return false;
    d.vertices.clear();
    for (const auto& v : values)
        d.vertices.emplace_back(v.x, v.y, v.z);
    auto fit = [n](auto& v, auto fill) {
        if (!v.empty())
            v.resize(n, fill);
    };
    fit(d.normals, nifly::Vector3());
    fit(d.tangents, nifly::Vector3());
    fit(d.bitangents, nifly::Vector3());
    fit(d.tangentWs, uint8_t(1));
    fit(d.vertexColors, nifly::Color4(1.0f, 1.0f, 1.0f, 1.0f));
    nifly::ByteColor4 white;
    white.r = white.g = white.b = white.a = 255;
    fit(d.vColors, white);
    for (auto& set : d.uvSets)
        fit(set, nifly::Vector2());
    fit(d.skinWeights,
        std::vector<nifly::BSGeometryMeshData::BoneWeight>(d.nWeightsPerVert));
    d.nVertices = static_cast<uint32_t>(n);

    struct Access: nifly::NiGeometryData {
        using NiGeometryData::numVertices;
    };

    d.*(&Access::numVertices) =
        static_cast<uint16_t>(std::min(n, size_t(0xFFFF)));
    return true;
}

// UV set `which` (0 or 1), flattened.
inline rust::Slice<const float>
mesh_data_uvs(const nifly::BSGeometryMeshData& d, size_t which) {
    if (which >= d.uvSets.size())
        return {};
    return {
        reinterpret_cast<const float*>(d.uvSets[which].data()),
        d.uvSets[which].size() * 2
    };
}

// Replaces UV set `which` (0 or 1). False, changing nothing, unless it's empty
// or has one UV per vertex.
inline bool set_mesh_data_uvs(
    nifly::BSGeometryMeshData& d,
    size_t which,
    rust::Slice<const float> flat
) {
    const size_t n = d.vertices.size();
    if (which > 1 || (!flat.empty() && flat.size() != n * 2))
        return false;
    if (d.uvSets.size() < 2)
        d.uvSets.resize(2);
    auto& set = d.uvSets[which];
    set.clear();
    for (size_t i = 0; i + 1 < flat.size(); i += 2)
        set.emplace_back(flat[i], flat[i + 1]);
    // SetUVs would drop the second set.
    if (which == 0) {
        if (set.empty())
            d.dataFlags &= ~uint16_t(1);
        else
            d.dataFlags |= 1;
    }
    return true;
}

// Flattened, 4 bytes per color.
inline rust::Slice<const uint8_t>
mesh_data_colors(const nifly::BSGeometryMeshData& d) {
    return {
        reinterpret_cast<const uint8_t*>(d.vColors.data()),
        d.vColors.size() * 4
    };
}

inline bool set_mesh_data_colors(
    nifly::BSGeometryMeshData& d,
    rust::Slice<const uint8_t> flat
) {
    if (!flat.empty() && flat.size() != d.vertices.size() * 4)
        return false;
    d.vColors.clear();
    for (size_t i = 0; i + 3 < flat.size(); i += 4) {
        nifly::ByteColor4 c;
        c.r = flat[i];
        c.g = flat[i + 1];
        c.b = flat[i + 2];
        c.a = flat[i + 3];
        d.vColors.push_back(c);
    }
    return true;
}

inline rust::Slice<const uint8_t>
mesh_data_tangent_ws(const nifly::BSGeometryMeshData& d) {
    return {d.tangentWs.data(), d.tangentWs.size()};
}

// One per tangent, each 0-3. False, changing nothing, otherwise.
inline bool set_mesh_data_tangent_ws(
    nifly::BSGeometryMeshData& d,
    rust::Slice<const uint8_t> ws
) {
    if (ws.size() != d.tangents.size())
        return false;
    for (uint8_t w : ws)
        if (w > 3)
            return false;
    d.tangentWs.assign(ws.begin(), ws.end());
    return true;
}

inline uint32_t
mesh_data_weights_per_vertex(const nifly::BSGeometryMeshData& d) {
    return d.nWeightsPerVert;
}

// Flattened bone index and weight pairs, nWeightsPerVert per vertex.
inline rust::Vec<uint16_t>
mesh_data_skin_weights(const nifly::BSGeometryMeshData& d) {
    rust::Vec<uint16_t> out;
    for (const auto& vertex : d.skinWeights)
        for (const auto& w : vertex) {
            out.push_back(w.boneIndex);
            out.push_back(w.weight);
        }
    return out;
}

// Replaces the skin weights, `per` per vertex for every vertex, or none.
// False, changing nothing, otherwise.
inline bool set_mesh_data_skin_weights(
    nifly::BSGeometryMeshData& d,
    rust::Slice<const uint16_t> flat,
    uint32_t per
) {
    const size_t n = d.vertices.size();
    if (!flat.empty() && (per == 0 || flat.size() != n * per * 2))
        return false;
    d.skinWeights.clear();
    d.nWeightsPerVert = flat.empty() ? 0 : per;
    for (size_t v = 0; v < (flat.empty() ? 0 : n); v++) {
        std::vector<nifly::BSGeometryMeshData::BoneWeight> weights(per);
        for (size_t j = 0; j < per; j++) {
            weights[j].boneIndex = flat[(v * per + j) * 2];
            weights[j].weight = flat[(v * per + j) * 2 + 1];
        }
        d.skinWeights.push_back(std::move(weights));
    }
    d.nTotalWeights = static_cast<uint32_t>(d.skinWeights.size() * per);
    return true;
}

inline size_t mesh_data_lod_count(const nifly::BSGeometryMeshData& d) {
    return d.lods.size();
}

// Flattened, 3 per triangle; empty if `i` is out of range.
inline rust::Slice<const uint16_t>
mesh_data_lod(const nifly::BSGeometryMeshData& d, size_t i) {
    if (i >= d.lods.size())
        return {};
    return {
        reinterpret_cast<const uint16_t*>(d.lods[i].data()),
        d.lods[i].size() * 3
    };
}

// Replaces the LOD triangle lists, given joined with each one's index count.
// False, changing nothing, unless every index is below the vertex count.
inline bool set_mesh_data_lods(
    nifly::BSGeometryMeshData& d,
    rust::Slice<const uint16_t> flat,
    rust::Slice<const size_t> lengths
) {
    if (!flat_triangles_in_range(flat, d.vertices.size()))
        return false;
    std::vector<std::vector<nifly::Triangle>> lods;
    size_t at = 0;
    for (size_t len : lengths) {
        if (len % 3 != 0 || len > flat.size() - at)
            return false;
        lods.push_back(to_triangles({flat.data() + at, len}));
        at += len;
    }
    if (at != flat.size())
        return false;
    d.lods = std::move(lods);
    d.nLODS = static_cast<uint32_t>(d.lods.size());
    return true;
}

// Flattened, 4 per meshlet: vertex count and offset, primitive count and offset.
inline rust::Slice<const uint32_t>
mesh_data_meshlets(const nifly::BSGeometryMeshData& d) {
    return {
        reinterpret_cast<const uint32_t*>(d.meshletList.data()),
        d.meshletList.size() * 4
    };
}

inline void set_mesh_data_meshlets(
    nifly::BSGeometryMeshData& d,
    rust::Slice<const uint32_t> flat
) {
    d.meshletList.clear();
    for (size_t i = 0; i + 3 < flat.size(); i += 4) {
        nifly::BSGeometryMeshData::Meshlet m;
        m.vertCount = flat[i];
        m.vertOffset = flat[i + 1];
        m.primCount = flat[i + 2];
        m.primOffset = flat[i + 3];
        d.meshletList.push_back(m);
    }
    d.nMeshlets = static_cast<uint32_t>(d.meshletList.size());
}

// Flattened, 2 per entry: center and expansion.
inline rust::Vec<Vec3> mesh_data_cull_data(const nifly::BSGeometryMeshData& d) {
    rust::Vec<Vec3> out;
    for (const auto& c : d.cullDataList) {
        out.push_back({c.center.x, c.center.y, c.center.z});
        out.push_back({c.expand.x, c.expand.y, c.expand.z});
    }
    return out;
}

inline void set_mesh_data_cull_data(
    nifly::BSGeometryMeshData& d,
    rust::Slice<const Vec3> flat
) {
    d.cullDataList.clear();
    for (size_t i = 0; i + 1 < flat.size(); i += 2) {
        nifly::BSGeometryMeshData::CullData c;
        c.center = nifly::Vector3(flat[i].x, flat[i].y, flat[i].z);
        c.expand = nifly::Vector3(flat[i + 1].x, flat[i + 1].y, flat[i + 1].z);
        d.cullDataList.push_back(c);
    }
    d.nCullData = static_cast<uint32_t>(d.cullDataList.size());
}

// GenerateMeshlets. False, changing nothing, if there are no triangles.
inline bool generate_mesh_data_meshlets(
    nifly::BSGeometryMeshData& d,
    uint32_t max_vertices,
    uint32_t max_primitives
) {
    if (d.tris.empty() || d.vertices.empty())
        return false;
    d.GenerateMeshlets(max_vertices, max_primitives);
    return true;
}
} // namespace nifly_rs

#undef NIFLY_RS_SHAPE
