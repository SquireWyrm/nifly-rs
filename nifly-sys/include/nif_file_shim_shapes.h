// Part of nif_file_shim.h: creating, changing, deleting and cloning shapes, and external meshes.
#pragma once

#include "nif_file_shim.h"

namespace nifly_rs {
// NifFile::IsSSECompatible(NiShape*): false if the shape is NiTriStrips or its
// skin partition has strips. Takes a non-const pointer but only reads.
inline bool is_shape_sse_compatible(
    const nifly::NifFile& file,
    const nifly::NiShape& shape
) {
    return file.IsSSECompatible(const_cast<nifly::NiShape*>(&shape));
}

// NifFile::GetExternalGeometryPathRefs, as a count and a getter. The meshes are
// only reachable through SelectMesh, so this selects each and then releases,
// leaving the first selected as before (nothing else in the bindings selects
// another). 0 unless the shape is a BSGeometry with external mesh data.
inline uint32_t external_mesh_count(const nifly::NiShape& shape) {
    auto geom =
        dynamic_cast<nifly::BSGeometry*>(const_cast<nifly::NiShape*>(&shape));
    if (!geom || geom->HasInternalGeomData())
        return 0;
    return geom->MeshCount();
}

// Empty if `index` is out of range.
inline rust::Slice<const uint8_t>
external_mesh_path(const nifly::NiShape& shape, uint32_t index) {
    if (index >= external_mesh_count(shape))
        return {};
    auto geom =
        static_cast<nifly::BSGeometry*>(const_cast<nifly::NiShape*>(&shape));
    const std::string& name =
        geom->SelectMesh(static_cast<uint8_t>(index))->meshName.get();
    geom->ReleaseMesh();
    return {reinterpret_cast<const uint8_t*>(name.data()), name.size()};
}

// NifFile::LoadExternalShapeData, reimplemented to read into fresh mesh data
// and only replace the mesh's on success: upstream reads into the mesh itself,
// leaving it half-overwritten when the data is short or malformed, and returns
// true whether or not the mesh exists. Reader errors propagate as the
// rust::Error thrown by the streambuf. Returns 0 on success, 1 if the shape
// isn't a BSGeometry with external mesh `index`, 2 if the data ended early, 3
// if its version is one upstream can't read (above 2).
inline int32_t load_external_mesh(
    nifly::NifFile& file,
    uint32_t shape_index,
    uint32_t index,
    read_bridge::RustReader& reader
) {
    auto shape = file.GetHeader().GetBlock<nifly::BSGeometry>(shape_index);
    if (!shape || index >= external_mesh_count(*shape))
        return 1;

    read_bridge::RustIStream in(reader);
    in.exceptions(std::ios::badbit);
    std::string bytes(std::istreambuf_iterator<char>(in), {});

    // Mesh data can't be assigned (it has a const member), so parse into a
    // scratch copy first and only parse into the mesh once that succeeds.
    auto parse = [&](nifly::BSGeometryMeshData& data) {
        std::istringstream buffer(bytes);
        nifly::NiIStream stream(&buffer, nullptr);
        nifly::NiStreamReversible sync(
            &stream,
            nullptr,
            nifly::NiStreamReversible::Mode::Reading
        );
        data.Sync(sync);
        return static_cast<bool>(buffer);
    };
    nifly::BSGeometryMeshData scratch;
    if (!parse(scratch))
        return 2;
    // Upstream stops after the version when it's one it can't read, as if the
    // mesh were empty.
    if (scratch.version > 2)
        return 3;

    parse(shape->SelectMesh(static_cast<uint8_t>(index))->meshData);
    shape->ReleaseMesh();
    return 0;
}

// NifFile::SaveExternalShapeData into `out`, written with write_mesh_data,
// which changes nothing in the file.
// Returns false, writing nothing, unless the shape is a BSGeometry with external
// mesh `index`.
inline bool save_external_mesh(
    const nifly::NiShape& shape,
    uint32_t index,
    rust::Vec<uint8_t>& out
) {
    if (index >= external_mesh_count(shape))
        return false;
    auto geom =
        static_cast<nifly::BSGeometry*>(const_cast<nifly::NiShape*>(&shape));
    std::ostringstream buffer;
    nifly::NiOStream stream(&buffer, nullptr);
    nifly::NiStreamReversible sync(
        nullptr,
        &stream,
        nifly::NiStreamReversible::Mode::Writing
    );
    write_mesh_data(
        geom->SelectMesh(static_cast<uint8_t>(index))->meshData,
        sync
    );
    geom->ReleaseMesh();
    const std::string bytes = buffer.str();
    out.reserve(bytes.size());
    for (char c : bytes)
        out.push_back(static_cast<uint8_t>(c));
    return true;
}

// Writes through GetExternalGeometryPathRefs' reference for mesh `index` of the
// BSGeometry at `shape_index`. Returns false if there's no such external mesh.
inline bool set_external_mesh_path(
    nifly::NifFile& file,
    uint32_t shape_index,
    uint32_t index,
    rust::Slice<const uint8_t> path
) {
    auto shape = file.GetHeader().GetBlock<nifly::NiShape>(shape_index);
    if (!shape)
        return false;
    auto refs = file.GetExternalGeometryPathRefs(shape);
    if (index >= refs.size())
        return false;
    refs[index].get().assign(
        reinterpret_cast<const char*>(path.data()),
        path.size()
    );
    return true;
}

// NifFile::CreateShapeFromData. Returns the new shape's index, or NIF_NPOS,
// changing nothing, unless: the file has a root node and is a version upstream
// builds shapes for (Morrowind, Fallout 3/NV, Skyrim LE/SE, Fallout 4/76; for
// others it builds Fallout 3 blocks), there are at most 65535 vertices, every
// triangle index is below the vertex count, the triangle count fits (65535 for
// NiTriShapeData), and `uvs` and `normals` are empty or have one per vertex
// (upstream reads them per vertex without checking).
//
// The version restriction works around an upstream issue: CreateShapeFromData's
// final branch only tells Skyrim LE apart, so every other version, Oblivion
// included, gets Fallout 3 blocks (NiTriShape with a BSShaderPPLightingProperty,
// which Oblivion doesn't have; it needs NiMaterialProperty and
// NiTexturingProperty, as the Morrowind branch builds). It never checks IsOB().
// Starfield is a missing feature instead: no branch builds a BSGeometry.
inline uint32_t create_shape(
    nifly::NifFile& file,
    rust::Slice<const uint8_t> name,
    rust::Slice<const Vec3> vertices,
    rust::Slice<const Triangle> triangles,
    rust::Slice<const Uv> uvs,
    rust::Slice<const Vec3> normals
) {
    auto version = file.GetHeader().GetVersion();
    bool bs_tri_shape = version.IsSSE() || version.IsFO4() || version.IsFO76();
    if (!bs_tri_shape && !version.IsMW() && !version.IsFO3() && !version.IsSK())
        return nifly::NIF_NPOS;
    if (!file.GetRootNode() || vertices.size() > 0xFFFF)
        return nifly::NIF_NPOS;
    if ((!uvs.empty() && uvs.size() != vertices.size())
        || (!normals.empty() && normals.size() != vertices.size()))
        return nifly::NIF_NPOS;
    size_t limit = bs_tri_shape ? file.GetTriangleLimit() : 0xFFFF;
    if (triangles.size() > limit)
        return nifly::NIF_NPOS;

    std::vector<nifly::Vector3> v;
    v.reserve(vertices.size());
    for (const auto& p : vertices)
        v.emplace_back(p.x, p.y, p.z);
    std::vector<nifly::Triangle> t;
    t.reserve(triangles.size());
    for (const auto& tri : triangles) {
        if (tri.p1 >= v.size() || tri.p2 >= v.size() || tri.p3 >= v.size())
            return nifly::NIF_NPOS;
        t.emplace_back(tri.p1, tri.p2, tri.p3);
    }
    std::vector<nifly::Vector2> uv;
    for (const auto& u : uvs)
        uv.emplace_back(u.u, u.v);
    std::vector<nifly::Vector3> n;
    for (const auto& p : normals)
        n.emplace_back(p.x, p.y, p.z);

    std::string shape_name(
        reinterpret_cast<const char*>(name.data()),
        name.size()
    );
    auto shape = file.CreateShapeFromData(
        shape_name,
        &v,
        &t,
        uv.empty() ? nullptr : &uv,
        n.empty() ? nullptr : &n
    );
    return shape ? file.GetBlockID(shape) : nifly::NIF_NPOS;
}

// NifFile::TriangulateShape on the shape at `shape_index`, which returns
// nothing. Returns false, changing nothing, unless the shape is an NiTriStrips
// with triangles whose NiTriStripsData no other block refers to: upstream
// replaces the data block in place, which would leave another strips shape
// sharing it with NiTriShapeData.
inline bool triangulate_shape(nifly::NifFile& file, uint32_t shape_index) {
    auto& hdr = file.GetHeader();
    auto shape = hdr.GetBlock<nifly::NiTriStrips>(shape_index);
    if (!shape)
        return false;
    auto data = hdr.GetBlock<nifly::NiTriStripsData>(shape->DataRef());
    if (!data || data->StripsToTris().empty())
        return false;
    if (hdr.GetBlockRefCount(shape->DataRef()->index, false) != 1)
        return false;
    file.TriangulateShape(shape);
    return true;
}

// NifFile::InvertUVsForShape on the shape at `shape_index`, which returns
// nothing. Returns false, changing nothing, unless the shape stores UVs;
// upstream silently skips the rest.
inline bool
invert_shape_uvs(nifly::NifFile& file, uint32_t shape_index, bool u, bool v) {
    auto shape = file.GetHeader().GetBlock<nifly::NiShape>(shape_index);
    std::vector<nifly::Vector2> uvs;
    if (!shape || !file.GetUvsForShape(shape, uvs))
        return false;
    file.InvertUVsForShape(shape, u, v);
    return true;
}

// NifFile::MirrorShape on the shape at `shape_index`, which returns nothing.
// Returns false, changing nothing, unless the shape has vertices it mirrors
// (geometry data or a BSTriShape's own), and, when an odd number of axes is
// mirrored, triangles it can rewind: upstream can't set strips' triangles, so it
// would mirror a strips shape without flipping its winding, turning it inside
// out.
inline bool mirror_shape(
    nifly::NifFile& file,
    uint32_t shape_index,
    bool x,
    bool y,
    bool z
) {
    auto shape = file.GetHeader().GetBlock<nifly::NiShape>(shape_index);
    if (!shape)
        return false;
    auto data = file.GetGeometryData(shape);
    if (!data && !dynamic_cast<nifly::BSTriShape*>(shape))
        return false;
    bool flip = (x != y) != z;
    if (flip && shape->HasType<nifly::NiTriStrips>())
        return false;
    file.MirrorShape(shape, x, y, z);
    return true;
}

// NifFile::MoveVertex on the shape at `shape_index`, which returns nothing.
// Returns false, changing nothing, unless the shape stores vertices (see
// settable_vertex_count) and `vertex` is one of them. Upstream compares a
// signed index, so a negative one would write out of bounds.
inline bool set_shape_vertex(
    nifly::NifFile& file,
    uint32_t shape_index,
    uint32_t vertex,
    Vec3 position
) {
    int32_t count = settable_vertex_count(file, shape_index);
    if (count < 0 || vertex >= static_cast<uint32_t>(count))
        return false;
    auto shape = file.GetHeader().GetBlock<nifly::NiShape>(shape_index);
    file.MoveVertex(shape, vec3_from_ffi(position), static_cast<int>(vertex));
    return true;
}

// NifFile::DeleteShape, reimplemented: upstream deletes the shape's data,
// properties, extra data, skin and shader even when other blocks share them,
// and leaves its collision and controllers behind. This removes the shape from
// every node's children, deletes it, then deletes whatever it referred to that
// nothing else does. Returns false if the block isn't a shape.
inline bool delete_shape(nifly::NifFile& file, uint32_t shape_index) {
    auto& hdr = file.GetHeader();
    auto shape = hdr.GetBlock<nifly::NiShape>(shape_index);
    if (!shape)
        return false;
    for (auto node : file.GetNodes())
        for (uint32_t i = node->childRefs.GetSize(); i-- > 0;)
            if (node->childRefs.GetBlockRef(i) == shape_index)
                node->childRefs.RemoveBlockRef(i);
    delete_if_unreferenced(file, {shape_index});
    return true;
}

// NifFile::DeleteShader, reimplemented: unlinks the shape's shader, the shaders
// and materials in its property list, and its alpha property, then deletes
// those (and their texture sets and controllers) only if nothing else refers to
// them. Upstream deletes them even when shared, and only removes two shader
// types from property lists. Returns false, changing nothing, if the block
// isn't a shape or has none of them.
inline bool delete_shader(nifly::NifFile& file, uint32_t shape_index) {
    auto& hdr = file.GetHeader();
    auto shape = hdr.GetBlock<nifly::NiShape>(shape_index);
    if (!shape)
        return false;
    std::vector<uint32_t> unlinked;
    if (auto ref = shape->ShaderPropertyRef(); ref && hdr.GetBlock(ref)) {
        unlinked.push_back(ref->index);
        ref->Clear();
    }
    if (auto ref = shape->AlphaPropertyRef(); ref && hdr.GetBlock(ref)) {
        unlinked.push_back(ref->index);
        ref->Clear();
    }
    for (uint32_t i = shape->propertyRefs.GetSize(); i-- > 0;) {
        uint32_t id = shape->propertyRefs.GetBlockRef(i);
        if (hdr.GetBlock<nifly::NiShader>(id)
            || hdr.GetBlock<nifly::NiAlphaProperty>(id)) {
            unlinked.push_back(id);
            shape->propertyRefs.RemoveBlockRef(i);
        }
    }
    if (unlinked.empty())
        return false;
    delete_if_unreferenced(file, unlinked);
    return true;
}

// NifFile::DeleteVertsForShape on the shape at `shape_index`, with `vertices`
// sorted and deduplicated here (upstream requires both and erases out of
// bounds otherwise). Returns false, changing nothing, unless the shape stores
// vertices and every index is one of them; a BSGeometry mesh must have at most
// 65535 vertices, as upstream requires. Upstream's own return value (whether the
// shape was left without vertices or triangles) isn't passed on.
inline bool delete_shape_vertices(
    nifly::NifFile& file,
    uint32_t shape_index,
    rust::Slice<const uint32_t> vertices
) {
    int32_t count = settable_vertex_count(file, shape_index);
    if (count < 0)
        return false;
    auto shape = file.GetHeader().GetBlock<nifly::NiShape>(shape_index);
    if (auto geom = dynamic_cast<nifly::BSGeometry*>(shape)) {
        auto data =
            dynamic_cast<nifly::BSGeometryMeshData*>(geom->GetGeomData());
        if (!data || data->vertices.size() > 0xFFFF)
            return false;
    }
    std::vector<uint16_t> sorted;
    sorted.reserve(vertices.size());
    for (uint32_t v : vertices) {
        if (v >= static_cast<uint32_t>(count))
            return false;
        sorted.push_back(static_cast<uint16_t>(v));
    }
    std::sort(sorted.begin(), sorted.end());
    sorted.erase(std::unique(sorted.begin(), sorted.end()), sorted.end());
    if (!sorted.empty())
        file.DeleteVertsForShape(shape, sorted);
    return true;
}

// Whether two files are from the same game (by NiVersion's Is* ranges), so the
// block types cloned from one exist in the other. Saving writes the copies in the
// target's version, dropping fields it doesn't store. Files in none of the ranges
// must match exactly. Upstream clones across any versions, which can put block
// types a game doesn't have (BSTriShape in Skyrim LE) into its files.
inline bool same_game(const nifly::NifFile& a, const nifly::NifFile& b) {
    const auto& va = a.GetHeader().GetVersion();
    const auto& vb = b.GetHeader().GetVersion();
    using Is = bool (nifly::NiVersion::*)() const;
    for (Is is :
         {&nifly::NiVersion::IsMW,
          &nifly::NiVersion::IsOB,
          &nifly::NiVersion::IsFO3,
          &nifly::NiVersion::IsSK,
          &nifly::NiVersion::IsSSE,
          &nifly::NiVersion::IsFO4,
          &nifly::NiVersion::IsFO76,
          &nifly::NiVersion::IsSF}) {
        if ((va.*is)() || (vb.*is)())
            return (va.*is)() && (vb.*is)();
    }
    return va.File() == vb.File() && va.User() == vb.User()
        && va.Stream() == vb.Stream();
}

inline uint32_t clone_shape_impl(
    nifly::NifFile& file,
    nifly::NiShape* shape,
    rust::Slice<const uint8_t> name,
    nifly::NifFile* source
) {
    std::string n(reinterpret_cast<const char*>(name.data()), name.size());
    auto clone = file.CloneShape(shape, n, source);
    return clone ? file.GetBlockID(clone) : nifly::NIF_NPOS;
}

// NifFile::CloneShape within the file, on the shape at `shape_index`. Returns
// the clone's index, or NIF_NPOS if that block isn't a shape.
inline uint32_t clone_shape(
    nifly::NifFile& file,
    uint32_t shape_index,
    rust::Slice<const uint8_t> name
) {
    auto shape = file.GetHeader().GetBlock<nifly::NiShape>(shape_index);
    return shape ? clone_shape_impl(file, shape, name, nullptr)
                 : nifly::NIF_NPOS;
}

// NifFile::CloneShape from `source`, which it only reads. NIF_NPOS unless both
// files are from the same game.
inline uint32_t clone_shape_from(
    nifly::NifFile& file,
    const nifly::NifFile& source,
    const nifly::NiShape& shape,
    rust::Slice<const uint8_t> name
) {
    if (!same_game(file, source))
        return nifly::NIF_NPOS;
    return clone_shape_impl(
        file,
        const_cast<nifly::NiShape*>(&shape),
        name,
        const_cast<nifly::NifFile*>(&source)
    );
}

// NifFile::CloneNamedNode, reimplemented on a given node rather than the first
// node with a name: copies it without its children, effects, collision or
// controller, adds it as an unparented block, then clones what it still refers
// to (extra data, properties).
inline uint32_t clone_node_impl(
    nifly::NifFile& file,
    const nifly::NiNode& node,
    nifly::NifFile* source
) {
    auto copy = node.Clone();
    copy->collisionRef.Clear();
    copy->controllerRef.Clear();
    copy->childRefs.Clear();
    copy->effectRefs.Clear();
    uint32_t id = file.GetHeader().AddBlock(std::move(copy));
    file.CloneChildren(file.GetHeader().GetBlock<nifly::NiNode>(id), source);
    return id;
}

// Within the file. NIF_NPOS if `node_index` isn't a node.
inline uint32_t clone_node(nifly::NifFile& file, uint32_t node_index) {
    auto node = file.GetHeader().GetBlock<nifly::NiNode>(node_index);
    return node ? clone_node_impl(file, *node, nullptr) : nifly::NIF_NPOS;
}

// From `source`, which it only reads. NIF_NPOS unless both files are from the
// same game.
inline uint32_t clone_node_from(
    nifly::NifFile& file,
    const nifly::NifFile& source,
    const nifly::NiNode& node
) {
    if (!same_game(file, source))
        return nifly::NIF_NPOS;
    return clone_node_impl(file, node, const_cast<nifly::NifFile*>(&source));
}

// NifFile::CloneChildren within the file: replaces each child ref of the block
// at `block_index` with a ref to a deep copy. False if there's no such block.
inline bool clone_children(nifly::NifFile& file, uint32_t block_index) {
    auto block = file.GetHeader().GetBlock<nifly::NiObject>(block_index);
    if (!block)
        return false;
    file.CloneChildren(block, nullptr);
    return true;
}

// NifFile::ReorderTriangles on the shape at `shape_index`: triangle i becomes
// the old triangle `order[i]`. Returns false, changing nothing, unless the
// shape has triangles it can set (not strips) and `order` is a permutation of
// them; upstream only checks the length and range, so a repeated index
// duplicates one triangle and drops another.
inline bool reorder_shape_triangles(
    nifly::NifFile& file,
    uint32_t shape_index,
    rust::Slice<const uint32_t> order
) {
    auto shape = file.GetHeader().GetBlock<nifly::NiShape>(shape_index);
    if (!shape || shape->HasType<nifly::NiTriStrips>())
        return false;
    std::vector<nifly::Triangle> tris;
    if (!shape->GetTriangles(tris) || order.size() != tris.size())
        return false;
    std::vector<bool> seen(tris.size());
    for (uint32_t i : order) {
        if (i >= tris.size() || seen[i])
            return false;
        seen[i] = true;
    }
    std::vector<uint32_t> list(order.begin(), order.end());
    return nifly::NifFile::ReorderTriangles(shape, list);
}
} // namespace nifly_rs
