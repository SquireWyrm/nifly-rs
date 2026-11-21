// Part of geometry_shim.h: BSTriShape and its vertex data.
#pragma once

#include "geometry_shim.h"

namespace nifly_rs {
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
    std::memcpy(static_cast<void*>(&s.vertexDesc), &bits, sizeof bits);
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
} // namespace nifly_rs
