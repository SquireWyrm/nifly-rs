// Part of nif_file_shim.h: shape vertex data, triangles, normals and additional data.
#pragma once

#include "nif_file_shim.h"

namespace nifly_rs {
template<typename T>
uint32_t
new_additional_data(nifly::NifFile& file, nifly::NiGeometryData& data) {
    auto block = std::make_unique<T>();
    block->numVertices = data.GetNumVertices();
    uint32_t id = file.GetHeader().AddBlock(std::move(block));
    data.additionalDataRef.index = id;
    return id;
}

inline uint32_t assign_additional_geometry_data(
    nifly::NifFile& file,
    uint32_t data_index,
    bool packed
) {
    auto data = file.GetHeader().GetBlock<nifly::NiGeometryData>(data_index);
    if (!data)
        return nifly::NIF_NPOS;
    if (packed)
        return new_additional_data<nifly::BSPackedAdditionalGeometryData>(
            file,
            *data
        );
    return new_additional_data<nifly::NiAdditionalGeometryData>(file, *data);
}

inline bool
remove_additional_geometry_data(nifly::NifFile& file, uint32_t data_index) {
    auto data = file.GetHeader().GetBlock<nifly::NiGeometryData>(data_index);
    if (!data || data->additionalDataRef.IsEmpty())
        return false;
    data->additionalDataRef.Clear();
    return true;
}

// Copies a std::vector into `out` through `convert`, returning `ok`.
template<typename T, typename U, typename F>
bool copy_out(bool ok, const std::vector<T>& in, rust::Vec<U>& out, F convert) {
    if (!ok)
        return false;
    out.reserve(in.size());
    for (const auto& x : in)
        out.push_back(convert(x));
    return true;
}

// NifFile::Get*ForShape (the copying overloads) for each kind of vertex data.
// They take a non-const shape but only read it.
inline bool shape_vertices(
    const nifly::NifFile& file,
    const nifly::NiShape& shape,
    rust::Vec<Vec3>& out
) {
    std::vector<nifly::Vector3> v;
    bool ok = file.GetVertsForShape(const_cast<nifly::NiShape*>(&shape), v);
    return copy_out(ok, v, out, vec3_to_ffi);
}

// GetNormalsForShape has no copying overload, and the pointer one refreshes a
// BSTriShape's cache of decoded normals. This decodes them the same way
// without writing to the shape.
inline bool read_normals(
    const nifly::NifFile& file,
    const nifly::NiShape& shape,
    std::vector<nifly::Vector3>& out
) {
    if (!shape.HasNormals())
        return false;
    auto s = const_cast<nifly::NiShape*>(&shape);
    if (auto data = file.GetGeometryData(s)) {
        out = data->normals;
        return true;
    }
    auto tri = dynamic_cast<const nifly::BSTriShape*>(&shape);
    if (!tri)
        return false;
    auto unpack = [](uint8_t c) { return c / 255.0f * 2.0f - 1.0f; };
    out.reserve(tri->vertData.size());
    for (const auto& v : tri->vertData)
        out.emplace_back(
            unpack(v.normal[0]),
            unpack(v.normal[1]),
            unpack(v.normal[2])
        );
    return true;
}

inline bool shape_normals(
    const nifly::NifFile& file,
    const nifly::NiShape& shape,
    rust::Vec<Vec3>& out
) {
    std::vector<nifly::Vector3> v;
    bool ok = read_normals(file, shape, v);
    return copy_out(ok, v, out, vec3_to_ffi);
}

inline bool shape_uvs(
    const nifly::NifFile& file,
    const nifly::NiShape& shape,
    rust::Vec<Uv>& out
) {
    std::vector<nifly::Vector2> v;
    bool ok = file.GetUvsForShape(const_cast<nifly::NiShape*>(&shape), v);
    return copy_out(ok, v, out, [](const nifly::Vector2& x) {
        return Uv {x.u, x.v};
    });
}

inline bool shape_colors(
    const nifly::NifFile& file,
    const nifly::NiShape& shape,
    rust::Vec<Color>& out
) {
    std::vector<nifly::Color4> v;
    bool ok = file.GetColorsForShape(const_cast<nifly::NiShape*>(&shape), v);
    return copy_out(ok, v, out, [](const nifly::Color4& c) {
        return Color {c.r, c.g, c.b, c.a};
    });
}

inline bool shape_tangents(
    const nifly::NifFile& file,
    const nifly::NiShape& shape,
    rust::Vec<Vec3>& out
) {
    std::vector<nifly::Vector3> v;
    bool ok = file.GetTangentsForShape(const_cast<nifly::NiShape*>(&shape), v);
    return copy_out(ok, v, out, vec3_to_ffi);
}

inline bool shape_bitangents(
    const nifly::NifFile& file,
    const nifly::NiShape& shape,
    rust::Vec<Vec3>& out
) {
    std::vector<nifly::Vector3> v;
    bool ok =
        file.GetBitangentsForShape(const_cast<nifly::NiShape*>(&shape), v);
    return copy_out(ok, v, out, vec3_to_ffi);
}

// GetEyeDataForShape (copying) is non-const but only reads.
inline bool shape_eye_data(
    const nifly::NifFile& file,
    const nifly::NiShape& shape,
    rust::Vec<float>& out
) {
    std::vector<float> v;
    bool ok = const_cast<nifly::NifFile&>(file).GetEyeDataForShape(
        const_cast<nifly::NiShape*>(&shape),
        v
    );
    return copy_out(ok, v, out, [](float x) { return x; });
}

// The vertex count of the shape at `shape_index` if it stores vertex data the
// Set*ForShape setters write: geometry data, or a BSTriShape's own vertices.
// -1 otherwise.
inline int32_t
settable_vertex_count(nifly::NifFile& file, uint32_t shape_index) {
    auto shape = file.GetHeader().GetBlock<nifly::NiShape>(shape_index);
    if (!shape)
        return -1;
    if (auto data = file.GetGeometryData(shape))
        return data->GetNumVertices();
    if (auto tri = dynamic_cast<nifly::BSTriShape*>(shape))
        return tri->GetNumVertices();
    return -1;
}

// Converts `in` and calls `set` with the shape, unless the shape has no
// settable vertex data or `in` doesn't have one value per vertex. Upstream
// rebuilds the geometry (dropping its triangles) when the vertex count differs,
// or copies mismatched arrays as they are.
template<typename In, typename Out, typename Convert, typename Set>
bool set_vertex_array(
    nifly::NifFile& file,
    uint32_t shape_index,
    rust::Slice<const In> in,
    Convert convert,
    Set set
) {
    int32_t count = settable_vertex_count(file, shape_index);
    if (count < 0 || in.size() != static_cast<size_t>(count))
        return false;
    std::vector<Out> values;
    values.reserve(in.size());
    for (const auto& x : in)
        values.push_back(convert(x));
    set(file.GetHeader().GetBlock<nifly::NiShape>(shape_index), values);
    return true;
}

inline nifly::Vector3 vec3_from_ffi(const Vec3& v) {
    return nifly::Vector3(v.x, v.y, v.z);
}

inline bool set_shape_vertices(
    nifly::NifFile& file,
    uint32_t shape,
    rust::Slice<const Vec3> values
) {
    return set_vertex_array<Vec3, nifly::Vector3>(
        file,
        shape,
        values,
        vec3_from_ffi,
        [&](nifly::NiShape* s, const std::vector<nifly::Vector3>& v) {
            file.SetVertsForShape(s, v);
        }
    );
}

inline bool set_shape_normals(
    nifly::NifFile& file,
    uint32_t shape,
    rust::Slice<const Vec3> values
) {
    return set_vertex_array<Vec3, nifly::Vector3>(
        file,
        shape,
        values,
        vec3_from_ffi,
        [&](nifly::NiShape* s, const std::vector<nifly::Vector3>& v) {
            file.SetNormalsForShape(s, v);
        }
    );
}

inline bool set_shape_uvs(
    nifly::NifFile& file,
    uint32_t shape,
    rust::Slice<const Uv> values
) {
    return set_vertex_array<Uv, nifly::Vector2>(
        file,
        shape,
        values,
        [](const Uv& uv) { return nifly::Vector2(uv.u, uv.v); },
        [&](nifly::NiShape* s, const std::vector<nifly::Vector2>& v) {
            file.SetUvsForShape(s, v);
        }
    );
}

inline bool set_shape_colors(
    nifly::NifFile& file,
    uint32_t shape,
    rust::Slice<const Color> values
) {
    return set_vertex_array<Color, nifly::Color4>(
        file,
        shape,
        values,
        [](const Color& c) { return nifly::Color4(c.r, c.g, c.b, c.a); },
        [&](nifly::NiShape* s, const std::vector<nifly::Color4>& v) {
            file.SetColorsForShape(s, v);
        }
    );
}

inline bool set_shape_tangents(
    nifly::NifFile& file,
    uint32_t shape,
    rust::Slice<const Vec3> values
) {
    return set_vertex_array<Vec3, nifly::Vector3>(
        file,
        shape,
        values,
        vec3_from_ffi,
        [&](nifly::NiShape* s, const std::vector<nifly::Vector3>& v) {
            file.SetTangentsForShape(s, v);
        }
    );
}

inline bool set_shape_bitangents(
    nifly::NifFile& file,
    uint32_t shape,
    rust::Slice<const Vec3> values
) {
    return set_vertex_array<Vec3, nifly::Vector3>(
        file,
        shape,
        values,
        vec3_from_ffi,
        [&](nifly::NiShape* s, const std::vector<nifly::Vector3>& v) {
            file.SetBitangentsForShape(s, v);
        }
    );
}

// Eye data only exists on BSTriShape.
inline bool set_shape_eye_data(
    nifly::NifFile& file,
    uint32_t shape,
    rust::Slice<const float> values
) {
    auto block = file.GetHeader().GetBlock<nifly::BSTriShape>(shape);
    if (!block)
        return false;
    return set_vertex_array<float, float>(
        file,
        shape,
        values,
        [](float x) { return x; },
        [&](nifly::NiShape* s, const std::vector<float>& v) {
            file.SetEyeDataForShape(s, v);
        }
    );
}

// NiShape::GetTriangles, which strips convert to triangles.
inline bool
shape_triangles(const nifly::NiShape& shape, rust::Vec<Triangle>& out) {
    std::vector<nifly::Triangle> tris;
    if (!shape.GetTriangles(tris))
        return false;
    out.reserve(tris.size());
    for (const auto& t : tris)
        out.push_back(Triangle {t.p1, t.p2, t.p3});
    return true;
}

// NiShape::SetTriangles, which upstream ignores for strips, lines and other
// data without a triangle list. Also rejects indices past the vertex count and
// more triangles than the shape can save: NiTriShapeData counts them in 16
// bits; BSTriShape and BSGeometry as GetTriangleLimit says.
inline bool set_shape_triangles(
    nifly::NifFile& file,
    uint32_t shape_index,
    rust::Slice<const Triangle> triangles
) {
    auto shape = file.GetHeader().GetBlock<nifly::NiShape>(shape_index);
    if (!shape)
        return false;

    size_t limit = file.GetTriangleLimit();
    uint32_t vertices = 0;
    auto data = file.GetGeometryData(shape);
    if (dynamic_cast<nifly::NiTriShapeData*>(data)) {
        limit = std::numeric_limits<uint16_t>::max();
        vertices = data->GetNumVertices();
    } else if (dynamic_cast<nifly::BSGeometryMeshData*>(data))
        vertices = data->GetNumVertices();
    else if (auto tri = dynamic_cast<nifly::BSTriShape*>(shape); tri && !data)
        vertices = tri->GetNumVertices();
    else
        return false;

    if (triangles.size() > limit)
        return false;
    std::vector<nifly::Triangle> tris;
    tris.reserve(triangles.size());
    for (const auto& t : triangles) {
        if (t.p1 >= vertices || t.p2 >= vertices || t.p3 >= vertices)
            return false;
        tris.emplace_back(t.p1, t.p2, t.p3);
    }
    shape->SetTriangles(tris);
    return true;
}

// NifFile::CalcNormalsForShape on the shape at `shape_index`, which returns
// nothing. Returns whether normals were calculated. Upstream only recalculates
// normals NiTriShapeData and NiTriStripsData already have, and for other
// geometry data (lines, BSGeometry meshes) just turns them on, zeroed; this
// turns them on for the former first and rejects the latter. It also skips
// Skyrim model-space shaders unless `force`.
inline bool calc_shape_normals(
    nifly::NifFile& file,
    uint32_t shape_index,
    bool smooth,
    float smooth_threshold,
    bool force
) {
    auto& hdr = file.GetHeader();
    auto shape = hdr.GetBlock<nifly::NiShape>(shape_index);
    if (!shape)
        return false;

    auto data = file.GetGeometryData(shape);
    if (data) {
        if (!dynamic_cast<nifly::NiTriShapeData*>(data)
            && !dynamic_cast<nifly::NiTriStripsData*>(data))
            return false;
    } else if (!dynamic_cast<nifly::BSTriShape*>(shape))
        return false;

    auto version = hdr.GetVersion();
    if ((version.IsSK() || version.IsSSE()) && !force) {
        auto shader = file.GetShader(shape);
        if (shader && shader->IsModelSpace())
            return false;
    }

    if (data && !data->HasNormals())
        data->SetNormals(true);
    file.CalcNormalsForShape(shape, force, smooth, smooth_threshold);
    return true;
}

// NifFile::CalcTangentsForShape on the shape at `shape_index`, which returns
// nothing. Returns whether tangents were calculated. Upstream does nothing
// without normals and UVs, and for other geometry data (lines, BSGeometry
// meshes) only turns tangents on, zeroed; both are rejected here.
inline bool calc_shape_tangents(nifly::NifFile& file, uint32_t shape_index) {
    auto shape = file.GetHeader().GetBlock<nifly::NiShape>(shape_index);
    if (!shape)
        return false;

    if (auto data = file.GetGeometryData(shape)) {
        if (!dynamic_cast<nifly::NiTriShapeData*>(data)
            && !dynamic_cast<nifly::NiTriStripsData*>(data))
            return false;
        if (!data->HasNormals() || !data->HasUVs())
            return false;
    } else if (auto tri = dynamic_cast<nifly::BSTriShape*>(shape)) {
        if (!tri->HasNormals() || !tri->HasUVs())
            return false;
    } else
        return false;

    file.CalcTangentsForShape(shape);
    return true;
}

// NifFile::ApplyNormalsFromFile, reimplemented on a shape index and a source
// shape rather than a name looked up in both files. Upstream deletes the
// target's old LOCKEDNORM blocks even when other blocks share them, and reads
// the source normals through a getter that writes to a cache. Returns false,
// changing nothing, unless the source shape has LOCKEDNORM indices, both shapes
// have normals, and they have the same number.
inline bool apply_locked_normals(
    nifly::NifFile& file,
    uint32_t shape_index,
    const nifly::NifFile& source,
    const nifly::NiShape& source_shape
) {
    auto& hdr = file.GetHeader();
    auto shape = hdr.GetBlock<nifly::NiShape>(shape_index);
    if (!shape)
        return false;

    // Upstream unions every LOCKEDNORM's indices and copies the last block.
    auto& src_hdr = const_cast<nifly::NifFile&>(source).GetHeader();
    const nifly::NiIntegersExtraData* locked_data = nullptr;
    std::unordered_set<uint32_t> locked;
    // NiBlockRefArray has no const iteration; this only reads.
    auto& src_refs = const_cast<nifly::NiShape&>(source_shape).extraDataRefs;
    for (auto& ref : src_refs) {
        auto data = src_hdr.GetBlock<nifly::NiIntegersExtraData>(ref);
        if (data && data->name == "LOCKEDNORM") {
            locked_data = data;
            for (auto i : data->integersData)
                locked.insert(i);
        }
    }
    if (!locked_data || locked.empty())
        return false;

    std::vector<nifly::Vector3> normals, src_normals;
    if (!read_normals(file, *shape, normals)
        || !read_normals(source, source_shape, src_normals)
        || normals.size() != src_normals.size())
        return false;
    for (auto i : locked)
        if (i < normals.size())
            normals[i] = src_normals[i];
    file.SetNormalsForShape(shape, normals);

    // Replace the shape's LOCKEDNORM blocks, deleting only unshared ones.
    std::vector<uint32_t> ids;
    for (uint32_t i = 0; i < shape->extraDataRefs.GetSize();) {
        uint32_t id = shape->extraDataRefs.GetBlockRef(i);
        auto data = hdr.GetBlock<nifly::NiIntegersExtraData>(id);
        if (data && data->name == "LOCKEDNORM") {
            ids.push_back(id);
            shape->extraDataRefs.RemoveBlockRef(i);
        } else
            i++;
    }
    std::sort(ids.begin(), ids.end(), std::greater<>());
    ids.erase(std::unique(ids.begin(), ids.end()), ids.end());
    for (uint32_t id : ids)
        if (hdr.GetBlockRefCount(id, false) == 0)
            hdr.DeleteBlock(id);

    file.AssignExtraData(shape, locked_data->Clone());
    return true;
}
} // namespace nifly_rs
