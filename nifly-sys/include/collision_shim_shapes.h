// Part of collision_shim.h: primitive, container and MOPP shapes, and shape creation.
#pragma once

#include "collision_shim.h"

namespace nifly_rs {
// Materials stored behind the virtual accessors.
#define NIFLY_RS_SHAPE_MATERIAL(T, snake) \
    inline uint32_t snake(const nifly::T& s) { \
        return s.GetMaterial(); \
    } \
    inline void set_##snake(nifly::T& s, uint32_t v) { \
        s.SetMaterial(v); \
    }

NIFLY_RS_SHAPE_MATERIAL(bhkHeightFieldShape, height_field_material)
NIFLY_RS_SHAPE_MATERIAL(bhkSphereRepShape, sphere_rep_material)
NIFLY_RS_SHAPE_MATERIAL(bhkNiTriStripsShape, strips_shape_material)
NIFLY_RS_SHAPE_MATERIAL(bhkListShape, list_shape_material)

#undef NIFLY_RS_SHAPE_MATERIAL

// Primitives.
NIFLY_RS_COLLISION_VEC3(bhkPlaneShape, unkVec, plane_shape_unknown_vector)
NIFLY_RS_COLLISION_VEC3(bhkPlaneShape, plane.normal, plane_shape_normal)
NIFLY_RS_COLLISION_FIELD(
    bhkPlaneShape,
    float,
    plane.constant,
    plane_shape_constant
)
NIFLY_RS_COLLISION_VEC4(bhkPlaneShape, halfExtents, plane_shape_half_extents)
NIFLY_RS_COLLISION_VEC4(bhkPlaneShape, center, plane_shape_center)
NIFLY_RS_COLLISION_FIELD(bhkConvexShape, float, radius, convex_shape_radius)
NIFLY_RS_COLLISION_CONVERT(
    bhkMultiSphereShape,
    CInfoPropertyFfi,
    shapeProperty,
    multi_sphere_property,
    property_to_ffi,
    property_from_ffi
)

inline rust::Vec<BoundingSphere>
multi_sphere_spheres(const nifly::bhkMultiSphereShape& s) {
    rust::Vec<BoundingSphere> out;
    for (auto& sphere : const_items(s.spheres))
        out.push_back({havok_vec3_to_ffi(sphere.center), sphere.radius});
    return out;
}

inline void set_multi_sphere_spheres(
    nifly::bhkMultiSphereShape& s,
    rust::Slice<const BoundingSphere> spheres
) {
    s.spheres.clear();
    for (auto& sphere : spheres) {
        nifly::BoundingSphere b;
        b.center = havok_vec3_from_ffi(sphere.center);
        b.radius = sphere.radius;
        s.spheres.push_back(b);
    }
}

NIFLY_RS_COLLISION_CONVERT(
    bhkConvexVerticesShape,
    CInfoPropertyFfi,
    vertsProp,
    convex_vertices_vertices_property,
    property_to_ffi,
    property_from_ffi
)
NIFLY_RS_COLLISION_CONVERT(
    bhkConvexVerticesShape,
    CInfoPropertyFfi,
    normalsProp,
    convex_vertices_normals_property,
    property_to_ffi,
    property_from_ffi
)

// Vertices (`normals` false) or normals as 4 floats each.
inline rust::Slice<const float>
convex_vertices_list(const nifly::bhkConvexVerticesShape& s, bool normals) {
    return vec4_list(normals ? s.normals : s.verts);
}

inline bool set_convex_vertices_list(
    nifly::bhkConvexVerticesShape& s,
    bool normals,
    rust::Slice<const float> values
) {
    return set_vec4_list(normals ? s.normals : s.verts, values);
}

NIFLY_RS_COLLISION_VEC3(bhkBoxShape, dimensions, box_shape_dimensions)
NIFLY_RS_COLLISION_VEC4(bhkCylinderShape, vertexA, cylinder_shape_vertex_a)
NIFLY_RS_COLLISION_VEC4(bhkCylinderShape, vertexB, cylinder_shape_vertex_b)
NIFLY_RS_COLLISION_FIELD(
    bhkCylinderShape,
    float,
    cylinderRadius,
    cylinder_shape_radius
)
NIFLY_RS_COLLISION_VEC3(bhkCapsuleShape, point1, capsule_shape_point_1)
NIFLY_RS_COLLISION_FIELD(
    bhkCapsuleShape,
    float,
    radius1,
    capsule_shape_radius_1
)
NIFLY_RS_COLLISION_VEC3(bhkCapsuleShape, point2, capsule_shape_point_2)
NIFLY_RS_COLLISION_FIELD(
    bhkCapsuleShape,
    float,
    radius2,
    capsule_shape_radius_2
)

// Containers.
NIFLY_RS_COLLISION_FIELD(
    bhkTransformShape,
    uint32_t,
    material,
    transform_shape_material
)
NIFLY_RS_COLLISION_FIELD(
    bhkTransformShape,
    float,
    radius,
    transform_shape_radius
)
NIFLY_RS_COLLISION_CONVERT(
    bhkTransformShape,
    std::array<float NIFLY_RS_COMMA 16>,
    xform,
    transform_shape_transform,
    matrix4_to_ffi,
    matrix4_from_ffi
)
NIFLY_RS_COLLISION_CONVERT(
    bhkListShape,
    CInfoPropertyFfi,
    childShapeProp,
    list_shape_child_shape_property,
    property_to_ffi,
    property_from_ffi
)
NIFLY_RS_COLLISION_CONVERT(
    bhkListShape,
    CInfoPropertyFfi,
    childFilterProp,
    list_shape_child_filter_property,
    property_to_ffi,
    property_from_ffi
)

inline rust::Vec<HavokFilterFfi>
list_shape_filters(const nifly::bhkListShape& s) {
    rust::Vec<HavokFilterFfi> out;
    for (auto& f : const_items(s.filters))
        out.push_back(filter_to_ffi(f));
    return out;
}

// False, changing nothing, unless there's one filter per sub-shape.
inline bool set_list_shape_filters(
    nifly::bhkListShape& s,
    rust::Slice<const HavokFilterFfi> filters
) {
    if (filters.size() != s.subShapeRefs.GetSize())
        return false;
    s.filters.clear();
    for (auto f : filters) {
        auto filter = filter_from_ffi(f);
        s.filters.push_back(filter);
    }
    return true;
}

NIFLY_RS_COLLISION_FIELD(
    bhkConvexListShape,
    uint32_t,
    material,
    convex_list_material
)
NIFLY_RS_COLLISION_FIELD(bhkConvexListShape, float, radius, convex_list_radius)
NIFLY_RS_COLLISION_FIELD(
    bhkConvexListShape,
    uint32_t,
    unkInt1,
    convex_list_unknown_int
)
NIFLY_RS_COLLISION_FIELD(
    bhkConvexListShape,
    float,
    unkFloat1,
    convex_list_unknown_float
)
NIFLY_RS_COLLISION_CONVERT(
    bhkConvexListShape,
    CInfoPropertyFfi,
    childShapeProp,
    convex_list_child_shape_property,
    property_to_ffi,
    property_from_ffi
)
NIFLY_RS_COLLISION_FIELD(
    bhkConvexListShape,
    bool,
    useCachedAABB,
    convex_list_use_cached_aabb
)
NIFLY_RS_COLLISION_FIELD(
    bhkConvexListShape,
    float,
    closestPointMinDistance,
    convex_list_closest_point_min_distance
)

// MOPP.
inline rust::Slice<const uint8_t>
mopp_code(const nifly::bhkMoppBvTreeShape& s) {
    return {s.data.data(), s.data.size()};
}

inline void
set_mopp_code(nifly::bhkMoppBvTreeShape& s, rust::Slice<const uint8_t> bytes) {
    s.data.resize(static_cast<uint32_t>(bytes.size()));
    std::copy(bytes.begin(), bytes.end(), s.data.data());
}

NIFLY_RS_COLLISION_VEC4(bhkMoppBvTreeShape, offset, mopp_offset)
NIFLY_RS_COLLISION_FIELD(bhkMoppBvTreeShape, float, scale, mopp_scale)
NIFLY_RS_COLLISION_FIELD(
    bhkMoppBvTreeShape,
    uint8_t,
    buildType,
    mopp_build_type
)

// Shape creation.
// The shape of `owner` (a world object, transform shape or MOPP shape), or
// null.
inline nifly::NiBlockRef<nifly::bhkShape>*
child_shape_ref(nifly::NiObject* owner) {
    if (auto o = dynamic_cast<nifly::bhkWorldObject*>(owner))
        return &o->shapeRef;
    if (auto o = dynamic_cast<nifly::bhkTransformShape*>(owner))
        return &o->shapeRef;
    if (auto o = dynamic_cast<nifly::bhkMoppBvTreeShape*>(owner))
        return &o->shapeRef;
    return nullptr;
}

// Adds a new S as the shape of the block at `owner` and returns its index.
// NIF_NPOS, changing nothing, if the block has no shape ref.
template<class S>
uint32_t assign_shape(nifly::NifFile& file, uint32_t owner) {
    auto& hdr = file.GetHeader();
    auto ref = child_shape_ref(hdr.GetBlock<nifly::NiObject>(owner));
    if (!ref)
        return nifly::NIF_NPOS;
    uint32_t id = hdr.AddBlock(std::make_unique<S>());
    ref->index = id;
    return id;
}

// Adds a new S at the end of the sub-shapes of the list or convex list shape
// at `owner` and returns its index. A list shape with one filter per
// sub-shape gets a default filter for it. NIF_NPOS, changing nothing, if the
// block isn't such a shape or is a convex list and S isn't convex.
template<class S>
uint32_t add_sub_shape(nifly::NifFile& file, uint32_t owner) {
    auto& hdr = file.GetHeader();
    auto obj = hdr.GetBlock<nifly::NiObject>(owner);
    if (auto list = dynamic_cast<nifly::bhkListShape*>(obj)) {
        bool in_step = list->filters.size() == list->subShapeRefs.GetSize();
        uint32_t id = hdr.AddBlock(std::make_unique<S>());
        list->subShapeRefs.AddBlockRef(id);
        if (in_step) {
            nifly::HavokFilter filter;
            list->filters.push_back(filter);
        }
        return id;
    }
    if (auto list = dynamic_cast<nifly::bhkConvexListShape*>(obj)) {
        if constexpr (!std::is_base_of_v<nifly::bhkConvexShape, S>)
            return nifly::NIF_NPOS;
        else {
            uint32_t id = hdr.AddBlock(std::make_unique<S>());
            list->shapeRefs.AddBlockRef(id);
            return id;
        }
    }
    return nifly::NIF_NPOS;
}

#define NIFLY_RS_NEW_SHAPE(T, snake) \
    inline uint32_t assign_##snake(nifly::NifFile& file, uint32_t owner) { \
        return assign_shape<nifly::T>(file, owner); \
    } \
    inline uint32_t add_sub_##snake(nifly::NifFile& file, uint32_t owner) { \
        return add_sub_shape<nifly::T>(file, owner); \
    }

NIFLY_RS_NEW_SHAPE(bhkPlaneShape, bhk_plane_shape)
NIFLY_RS_NEW_SHAPE(bhkMultiSphereShape, bhk_multi_sphere_shape)
NIFLY_RS_NEW_SHAPE(bhkConvexListShape, bhk_convex_list_shape)
NIFLY_RS_NEW_SHAPE(bhkConvexVerticesShape, bhk_convex_vertices_shape)
NIFLY_RS_NEW_SHAPE(bhkBoxShape, bhk_box_shape)
NIFLY_RS_NEW_SHAPE(bhkSphereShape, bhk_sphere_shape)
NIFLY_RS_NEW_SHAPE(bhkCylinderShape, bhk_cylinder_shape)
NIFLY_RS_NEW_SHAPE(bhkTransformShape, bhk_transform_shape)
NIFLY_RS_NEW_SHAPE(bhkConvexTransformShape, bhk_convex_transform_shape)
NIFLY_RS_NEW_SHAPE(bhkCapsuleShape, bhk_capsule_shape)
NIFLY_RS_NEW_SHAPE(bhkMoppBvTreeShape, bhk_mopp_bv_tree_shape)
NIFLY_RS_NEW_SHAPE(bhkNiTriStripsShape, bhk_ni_tri_strips_shape)
NIFLY_RS_NEW_SHAPE(bhkListShape, bhk_list_shape)
NIFLY_RS_NEW_SHAPE(bhkPackedNiTriStripsShape, bhk_packed_ni_tri_strips_shape)
NIFLY_RS_NEW_SHAPE(bhkCompressedMeshShape, bhk_compressed_mesh_shape)

#undef NIFLY_RS_NEW_SHAPE

// Adds new data to the packed or compressed mesh shape at `shape` and returns
// its index. NIF_NPOS, changing nothing, if the block isn't one.
inline uint32_t assign_packed_data(nifly::NifFile& file, uint32_t shape) {
    auto& hdr = file.GetHeader();
    auto s = hdr.GetBlock<nifly::bhkPackedNiTriStripsShape>(shape);
    if (!s)
        return nifly::NIF_NPOS;
    uint32_t id =
        hdr.AddBlock(std::make_unique<nifly::hkPackedNiTriStripsData>());
    s->dataRef.index = id;
    return id;
}

inline uint32_t assign_compressed_data(nifly::NifFile& file, uint32_t shape) {
    auto& hdr = file.GetHeader();
    auto s = hdr.GetBlock<nifly::bhkCompressedMeshShape>(shape);
    if (!s)
        return nifly::NIF_NPOS;
    uint32_t id =
        hdr.AddBlock(std::make_unique<nifly::bhkCompressedMeshShapeData>());
    s->dataRef.index = id;
    return id;
}
} // namespace nifly_rs
