// Declares the rust::Vec<T> default constructors that cxx defines in each bridge's
// generated .cc, for the shared structs the bridges put in a Vec. The shims construct
// these Vecs in inline functions, which GCC treats as instantiating the primary
// template, and then rejects the specialization defined later in the same file.
// build.rs force-includes this header, so it comes before any shim.
#pragma once

// Its std::hash<Triangle> names Triangle after `using namespace nifly`, which the
// global Triangle declared below would make ambiguous.
#include "Object3d.hpp"
#include "rust/cxx.h"

#define NIFLY_RS_RUST_VEC(T) \
    struct T; \
    namespace rust { \
    inline namespace cxxbridge1 { \
        template<> \
        Vec<::T>::Vec() noexcept; \
    } \
    }

NIFLY_RS_RUST_VEC(AdditionalDataChannel)
NIFLY_RS_RUST_VEC(BlendItemFfi)
NIFLY_RS_RUST_VEC(BoneTransformFfi)
NIFLY_RS_RUST_VEC(BoundingSphere)
NIFLY_RS_RUST_VEC(BoundingVolumeFfi)
NIFLY_RS_RUST_VEC(CmsBigTriangleFfi)
NIFLY_RS_RUST_VEC(CmsMaterialFfi)
NIFLY_RS_RUST_VEC(CmsTransformFfi)
NIFLY_RS_RUST_VEC(Color)
NIFLY_RS_RUST_VEC(ControlledBlockFfi)
NIFLY_RS_RUST_VEC(FurniturePositionFfi)
NIFLY_RS_RUST_VEC(GeometrySegment)
NIFLY_RS_RUST_VEC(HavokFilterFfi)
NIFLY_RS_RUST_VEC(JointConstraintFfi)
NIFLY_RS_RUST_VEC(KeyFfi)
NIFLY_RS_RUST_VEC(MorphFfi)
NIFLY_RS_RUST_VEC(MorphWeightFfi)
NIFLY_RS_RUST_VEC(NodeGroupFfi)
NIFLY_RS_RUST_VEC(PackedGeomObjectFfi)
NIFLY_RS_RUST_VEC(PackedTriangleFfi)
NIFLY_RS_RUST_VEC(ParticleInfoFfi)
NIFLY_RS_RUST_VEC(PartitionInfo)
NIFLY_RS_RUST_VEC(ScreenPolygon)
NIFLY_RS_RUST_VEC(SegmentFfi)
NIFLY_RS_RUST_VEC(SubPartFfi)
NIFLY_RS_RUST_VEC(SubsegmentFfi)
NIFLY_RS_RUST_VEC(TreadFfi)
NIFLY_RS_RUST_VEC(Triangle)
NIFLY_RS_RUST_VEC(Uv)
NIFLY_RS_RUST_VEC(Vec3)
NIFLY_RS_RUST_VEC(VertexWeight)

#undef NIFLY_RS_RUST_VEC
