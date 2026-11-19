#pragma once

#include "Object3d.hpp"
#include "nifly-sys/src/object_3d.rs.h"

namespace nifly_rs {
inline nifly::MatTransform from_ffi(const Transform& t) {
    nifly::MatTransform m;
    m.translation =
        nifly::Vector3(t.translation.x, t.translation.y, t.translation.z);
    for (int r = 0; r < 3; ++r) {
        m.rotation[r] =
            nifly::Vector3(t.rotation[r].x, t.rotation[r].y, t.rotation[r].z);
    }
    m.scale = t.scale;
    return m;
}

inline Transform to_ffi(const nifly::MatTransform& m) {
    Transform t;
    t.translation = {m.translation.x, m.translation.y, m.translation.z};
    for (int r = 0; r < 3; ++r) {
        t.rotation[r] = {m.rotation[r].x, m.rotation[r].y, m.rotation[r].z};
    }
    t.scale = m.scale;
    return t;
}

inline nifly::BoundingSphere from_ffi(const BoundingSphere& b) {
    return nifly::BoundingSphere(
        nifly::Vector3(b.center.x, b.center.y, b.center.z),
        b.radius
    );
}

inline BoundingSphere to_ffi(const nifly::BoundingSphere& b) {
    BoundingSphere out;
    out.center = {b.center.x, b.center.y, b.center.z};
    out.radius = b.radius;
    return out;
}
} // namespace nifly_rs
