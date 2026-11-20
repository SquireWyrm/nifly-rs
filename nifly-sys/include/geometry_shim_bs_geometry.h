// Part of geometry_shim.h: BSGeometry and its mesh data.
#pragma once

#include "geometry_shim.h"

namespace nifly_rs {
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
