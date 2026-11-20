// Part of geometry_shim.h: NiGeometryData arrays, triangle data and NiGeometry materials and shaders.
#pragma once

#include "geometry_shim.h"

namespace nifly_rs {
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
} // namespace nifly_rs
