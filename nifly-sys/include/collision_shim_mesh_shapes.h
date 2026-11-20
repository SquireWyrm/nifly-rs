// Part of collision_shim.h: triangle strip, packed and compressed mesh shapes and their data.
#pragma once

#include "collision_shim.h"

namespace nifly_rs {
// bhkNiTriStripsShape.
NIFLY_RS_COLLISION_FIELD(
    bhkNiTriStripsShape,
    float,
    radius,
    strips_shape_radius
)
NIFLY_RS_COLLISION_FIELD(
    bhkNiTriStripsShape,
    uint32_t,
    growBy,
    strips_shape_grow_by
)
NIFLY_RS_COLLISION_VEC4(bhkNiTriStripsShape, scale, strips_shape_scale)

inline rust::Slice<const uint32_t>
strips_shape_filters(const nifly::bhkNiTriStripsShape& s) {
    return {s.filters.data(), s.filters.size()};
}

// False, changing nothing, unless there's one filter per strips data ref.
inline bool set_strips_shape_filters(
    nifly::bhkNiTriStripsShape& s,
    rust::Slice<const uint32_t> filters
) {
    if (filters.size() != s.partRefs.GetSize())
        return false;
    s.filters.resize(static_cast<uint32_t>(filters.size()));
    std::copy(filters.begin(), filters.end(), s.filters.data());
    return true;
}

// Sub-parts, stored in the shape up to stream 11 and in the data after.
template<class V>
rust::Vec<SubPartFfi> sub_parts_to_ffi(const V& parts) {
    rust::Vec<SubPartFfi> out;
    for (auto& p : const_items(parts))
        out.push_back({filter_to_ffi(p.filter), p.numVerts, p.material});
    return out;
}

// False, changing nothing, if there are more than 65535.
template<class V>
bool set_sub_parts(V& parts, rust::Slice<const SubPartFfi> values) {
    if (values.size() > 0xFFFF)
        return false;
    parts.clear();
    for (auto& v : values) {
        nifly::hkSubPartData p;
        p.filter = filter_from_ffi(v.filter);
        p.numVerts = v.vertex_count;
        p.material = v.material;
        parts.push_back(p);
    }
    return true;
}

inline rust::Vec<SubPartFfi>
packed_shape_sub_parts(const nifly::bhkPackedNiTriStripsShape& s) {
    return sub_parts_to_ffi(s.subPartData);
}

inline bool set_packed_shape_sub_parts(
    nifly::bhkPackedNiTriStripsShape& s,
    rust::Slice<const SubPartFfi> parts
) {
    return set_sub_parts(s.subPartData, parts);
}

inline rust::Vec<SubPartFfi>
packed_data_sub_parts(const nifly::hkPackedNiTriStripsData& d) {
    return sub_parts_to_ffi(d.subPartData);
}

inline bool set_packed_data_sub_parts(
    nifly::hkPackedNiTriStripsData& d,
    rust::Slice<const SubPartFfi> parts
) {
    return set_sub_parts(d.subPartData, parts);
}

NIFLY_RS_COLLISION_FIELD(
    bhkPackedNiTriStripsShape,
    uint32_t,
    userData,
    packed_shape_user_data
)
NIFLY_RS_COLLISION_FIELD(
    bhkPackedNiTriStripsShape,
    float,
    radius,
    packed_shape_radius
)
NIFLY_RS_COLLISION_VEC4(bhkPackedNiTriStripsShape, scaling, packed_shape_scale)
NIFLY_RS_COLLISION_FIELD(
    bhkPackedNiTriStripsShape,
    float,
    radius2,
    packed_shape_radius_copy
)
NIFLY_RS_COLLISION_VEC4(
    bhkPackedNiTriStripsShape,
    scaling2,
    packed_shape_scale_copy
)

// hkPackedNiTriStripsData triangles. Both lists are filled on set, so either
// version saves them; a get reads the one with normals if it's complete.
inline rust::Vec<PackedTriangleFfi>
packed_data_triangles(const nifly::hkPackedNiTriStripsData& d) {
    rust::Vec<PackedTriangleFfi> out;
    if (d.triNormData.size() == d.keyCount) {
        for (auto& t : d.triNormData)
            out.push_back(
                {t.tri.p1,
                 t.tri.p2,
                 t.tri.p3,
                 t.weldingInfo,
                 havok_vec3_to_ffi(t.normal)}
            );
    } else {
        for (uint32_t i = 0; i < d.keyCount && i < d.triData.size(); i++) {
            auto& t = d.triData[i];
            out.push_back(
                {t.tri.p1, t.tri.p2, t.tri.p3, t.weldingInfo, {0, 0, 0}}
            );
        }
    }
    return out;
}

// False, changing nothing, if an index isn't below the vertex count.
inline bool set_packed_data_triangles(
    nifly::hkPackedNiTriStripsData& d,
    rust::Slice<const PackedTriangleFfi> tris
) {
    for (auto& t : tris)
        if (t.p1 >= d.numVerts || t.p2 >= d.numVerts || t.p3 >= d.numVerts)
            return false;
    d.triData.clear();
    d.triNormData.clear();
    for (auto& t : tris) {
        nifly::hkTriangleData tri;
        tri.tri = nifly::Triangle(t.p1, t.p2, t.p3);
        tri.weldingInfo = t.welding_info;
        d.triData.push_back(tri);
        nifly::hkTriangleNormalData norm;
        norm.tri = tri.tri;
        norm.weldingInfo = t.welding_info;
        norm.normal = havok_vec3_from_ffi(t.normal);
        d.triNormData.push_back(norm);
    }
    d.keyCount = static_cast<uint32_t>(tris.size());
    return true;
}

inline rust::Vec<Vec3>
packed_data_vertices(const nifly::hkPackedNiTriStripsData& d) {
    rust::Vec<Vec3> out;
    for (uint32_t i = 0; i < d.numVerts && i < d.compressedVertData.size(); i++)
        out.push_back(havok_vec3_to_ffi(d.compressedVertData[i]));
    return out;
}

// False, changing nothing, if a triangle uses a vertex past the new count.
inline bool set_packed_data_vertices(
    nifly::hkPackedNiTriStripsData& d,
    rust::Slice<const Vec3> verts
) {
    for (auto& t : packed_data_triangles(d))
        if (t.p1 >= verts.size() || t.p2 >= verts.size()
            || t.p3 >= verts.size())
            return false;
    d.compressedVertData.clear();
    for (auto& v : verts)
        d.compressedVertData.push_back(havok_vec3_from_ffi(v));
    d.numVerts = static_cast<uint32_t>(verts.size());
    return true;
}

// bhkCompressedMeshShape.
NIFLY_RS_COLLISION_FIELD(
    bhkCompressedMeshShape,
    uint32_t,
    userData,
    compressed_shape_user_data
)
NIFLY_RS_COLLISION_FIELD(
    bhkCompressedMeshShape,
    float,
    radius,
    compressed_shape_radius
)
NIFLY_RS_COLLISION_FIELD(
    bhkCompressedMeshShape,
    float,
    unkFloat,
    compressed_shape_unknown_float
)
NIFLY_RS_COLLISION_VEC4(bhkCompressedMeshShape, scaling, compressed_shape_scale)
NIFLY_RS_COLLISION_FIELD(
    bhkCompressedMeshShape,
    float,
    radius2,
    compressed_shape_radius_copy
)
NIFLY_RS_COLLISION_VEC4(
    bhkCompressedMeshShape,
    scaling2,
    compressed_shape_scale_copy
)

// bhkCompressedMeshShapeData.
NIFLY_RS_COLLISION_FIELD(
    bhkCompressedMeshShapeData,
    uint32_t,
    bitsPerIndex,
    cms_bits_per_index
)
NIFLY_RS_COLLISION_FIELD(
    bhkCompressedMeshShapeData,
    uint32_t,
    bitsPerWIndex,
    cms_bits_per_welding_index
)
NIFLY_RS_COLLISION_FIELD(
    bhkCompressedMeshShapeData,
    uint32_t,
    maskWIndex,
    cms_mask_welding_index
)
NIFLY_RS_COLLISION_FIELD(
    bhkCompressedMeshShapeData,
    uint32_t,
    maskIndex,
    cms_mask_index
)
NIFLY_RS_COLLISION_FIELD(bhkCompressedMeshShapeData, float, error, cms_error)
NIFLY_RS_COLLISION_VEC4(bhkCompressedMeshShapeData, aabbBoundMin, cms_aabb_min)
NIFLY_RS_COLLISION_VEC4(bhkCompressedMeshShapeData, aabbBoundMax, cms_aabb_max)
NIFLY_RS_COLLISION_FIELD(
    bhkCompressedMeshShapeData,
    uint8_t,
    weldingType,
    cms_welding_type
)
NIFLY_RS_COLLISION_FIELD(
    bhkCompressedMeshShapeData,
    uint8_t,
    materialType,
    cms_material_type
)
NIFLY_RS_COLLISION_FIELD(
    bhkCompressedMeshShapeData,
    uint32_t,
    numNamedMat,
    cms_named_material_count
)
NIFLY_RS_COLLISION_FIELD(
    bhkCompressedMeshShapeData,
    uint32_t,
    numConvexPieceA,
    cms_convex_piece_count
)

// Material lists 32 (0), 16 (1) and 8 (2).
inline nifly::NiVector<uint32_t>&
cms_material_list(nifly::bhkCompressedMeshShapeData& d, uint8_t which) {
    return which == 0 ? d.mat32 : which == 1 ? d.mat16 : d.mat8;
}

inline rust::Slice<const uint32_t>
cms_materials_n(const nifly::bhkCompressedMeshShapeData& d, uint8_t which) {
    auto& list = cms_material_list(
        const_cast<nifly::bhkCompressedMeshShapeData&>(d),
        which
    );
    return {list.data(), list.size()};
}

inline void set_cms_materials_n(
    nifly::bhkCompressedMeshShapeData& d,
    uint8_t which,
    rust::Slice<const uint32_t> values
) {
    auto& list = cms_material_list(d, which);
    list.resize(static_cast<uint32_t>(values.size()));
    std::copy(values.begin(), values.end(), list.data());
}

inline rust::Vec<CmsMaterialFfi>
cms_materials(const nifly::bhkCompressedMeshShapeData& d) {
    rust::Vec<CmsMaterialFfi> out;
    for (auto& m : const_items(d.materials))
        out.push_back({m.material, filter_to_ffi(m.layer)});
    return out;
}

inline void set_cms_materials(
    nifly::bhkCompressedMeshShapeData& d,
    rust::Slice<const CmsMaterialFfi> values
) {
    d.materials.clear();
    for (auto& v : values) {
        nifly::bhkCMSDMaterial m;
        m.material = v.material;
        m.layer = filter_from_ffi(v.filter);
        d.materials.push_back(m);
    }
}

inline rust::Vec<CmsTransformFfi>
cms_transforms(const nifly::bhkCompressedMeshShapeData& d) {
    rust::Vec<CmsTransformFfi> out;
    for (auto& t : const_items(d.transforms))
        out.push_back({vec4_to_ffi(t.translation), quat_to_ffi(t.rotation)});
    return out;
}

inline void set_cms_transforms(
    nifly::bhkCompressedMeshShapeData& d,
    rust::Slice<const CmsTransformFfi> values
) {
    d.transforms.clear();
    for (auto& v : values) {
        nifly::bhkCMSDTransform t;
        t.translation = vec4_from_ffi(v.translation);
        t.rotation = quat_from_ffi(v.rotation);
        d.transforms.push_back(t);
    }
}

inline rust::Slice<const float>
cms_big_vertices(const nifly::bhkCompressedMeshShapeData& d) {
    return vec4_list(d.bigVerts);
}

// False, changing nothing, unless there are 4 floats per vertex and the big
// triangles only use vertices below the new count.
inline bool set_cms_big_vertices(
    nifly::bhkCompressedMeshShapeData& d,
    rust::Slice<const float> values
) {
    size_t count = values.size() / 4;
    for (auto& t : const_items(d.bigTris))
        if (t.triangle1 >= count || t.triangle2 >= count
            || t.triangle3 >= count)
            return false;
    return set_vec4_list(d.bigVerts, values);
}

inline rust::Vec<CmsBigTriangleFfi>
cms_big_triangles(const nifly::bhkCompressedMeshShapeData& d) {
    rust::Vec<CmsBigTriangleFfi> out;
    for (auto& t : const_items(d.bigTris))
        out.push_back(
            {t.triangle1, t.triangle2, t.triangle3, t.material, t.weldingInfo}
        );
    return out;
}

// False, changing nothing, if an index isn't below the big vertex count.
inline bool set_cms_big_triangles(
    nifly::bhkCompressedMeshShapeData& d,
    rust::Slice<const CmsBigTriangleFfi> values
) {
    uint32_t count = d.bigVerts.size();
    for (auto& v : values)
        if (v.p1 >= count || v.p2 >= count || v.p3 >= count)
            return false;
    d.bigTris.clear();
    for (auto& v : values) {
        nifly::bhkCMSDBigTris t;
        t.triangle1 = v.p1;
        t.triangle2 = v.p2;
        t.triangle3 = v.p3;
        t.material = v.material;
        t.weldingInfo = v.welding_info;
        d.bigTris.push_back(t);
    }
    return true;
}

inline size_t cms_chunk_count(const nifly::bhkCompressedMeshShapeData& d) {
    return d.chunks.size();
}

inline CmsChunkFfi
cms_chunk(const nifly::bhkCompressedMeshShapeData& d, size_t index) {
    auto& c = const_cast<nifly::bhkCompressedMeshShapeData&>(d)
                  .chunks[static_cast<uint32_t>(index)];
    return {
        vec4_to_ffi(c.translation),
        c.matIndex,
        c.reference,
        c.transformIndex
    };
}

// A chunk's vertices (0), indices (1), strips (2) or welding info (3).
inline rust::Slice<const uint16_t> cms_chunk_list(
    const nifly::bhkCompressedMeshShapeData& d,
    size_t index,
    uint8_t which
) {
    auto& c = const_cast<nifly::bhkCompressedMeshShapeData&>(d)
                  .chunks[static_cast<uint32_t>(index)];
    auto& list = which == 0 ? c.verts
        : which == 1        ? c.indices
        : which == 2        ? c.strips
                            : c.weldingInfo;
    return {list.data(), list.size()};
}

// Replaces the chunks. `lists` holds each chunk's vertices, indices, strips
// and welding info in turn, with their lengths in `lengths` (4 per chunk).
// False, changing nothing, unless the lengths match.
inline bool set_cms_chunks(
    nifly::bhkCompressedMeshShapeData& d,
    rust::Slice<const CmsChunkFfi> chunks,
    rust::Slice<const uint16_t> lists,
    rust::Slice<const uint32_t> lengths
) {
    if (lengths.size() != chunks.size() * 4)
        return false;
    size_t total = 0;
    for (uint32_t n : lengths)
        total += n;
    if (total != lists.size())
        return false;

    d.chunks.clear();
    size_t pos = 0;
    for (size_t i = 0; i < chunks.size(); i++) {
        nifly::bhkCMSDChunk c;
        c.translation = vec4_from_ffi(chunks[i].translation);
        c.matIndex = chunks[i].material_index;
        c.reference = chunks[i].reference;
        c.transformIndex = chunks[i].transform_index;
        nifly::NiVector<uint16_t>* targets[4] =
            {&c.verts, &c.indices, &c.strips, &c.weldingInfo};
        for (int k = 0; k < 4; k++) {
            uint32_t n = lengths[i * 4 + k];
            targets[k]->resize(n);
            std::copy(
                lists.begin() + pos,
                lists.begin() + pos + n,
                targets[k]->data()
            );
            pos += n;
        }
        d.chunks.push_back(c);
    }
    return true;
}
} // namespace nifly_rs
