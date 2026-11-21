//! Tests for Vec3, Matrix3, Transform and the bounding volumes.

use nifly::{
    file::NifFile,
    nodes::NiNode,
    object3d::{BoundingSphere, BoundingVolume, Matrix3, Transform, Vec3, vec3},
};

use crate::common::{assert_close, open};

const EPS: f32 = 1e-5;

/// 90° about Z: X -> Y, Y -> -X.
const ROT_Z: Matrix3 = Matrix3::from_rows(vec3(0.0, -1.0, 0.0), vec3(1.0, 0.0, 0.0), Vec3::Z);

/// A rotation that isn't about a single axis, with a translation and scale.
fn sample() -> Transform {
    // About X by 0.6 rad, then about Z by 90°.
    let (s, c) = 0.6f32.sin_cos();
    let about_x = Matrix3::from_rows(Vec3::X, vec3(0.0, c, -s), vec3(0.0, s, c));
    Transform::new(vec3(1.0, -2.0, 0.5), ROT_Z * about_x, 1.75)
}

#[test]
fn vec3_operators() {
    let a = vec3(1.0, 2.0, 3.0);
    let b = vec3(-4.0, 0.5, 2.0);
    assert_eq!(a + b, vec3(-3.0, 2.5, 5.0));
    assert_eq!(a - b, vec3(5.0, 1.5, 1.0));
    assert_eq!(-a, vec3(-1.0, -2.0, -3.0));
    assert_eq!(a * 2.0, vec3(2.0, 4.0, 6.0));
    assert_eq!(2.0 * a, a * 2.0);
    assert_eq!(a / 2.0, vec3(0.5, 1.0, 1.5));

    let mut c = a;
    c += b;
    c -= b;
    c *= 4.0;
    c /= 2.0;
    assert_eq!(c, a * 2.0);
}

#[test]
fn vec3_products_and_lengths() {
    let a = vec3(1.0, 2.0, 3.0);
    let b = vec3(-4.0, 0.5, 2.0);
    assert_eq!(a.dot(b), 3.0);
    assert_eq!(Vec3::X.cross(Vec3::Y), Vec3::Z);
    assert_eq!(a.cross(b).dot(a), 0.0);
    assert_eq!(vec3(3.0, 4.0, 0.0).length(), 5.0);
    assert_eq!(vec3(3.0, 4.0, 0.0).length_squared(), 25.0);
    assert_eq!(Vec3::ZERO.distance(vec3(0.0, 0.0, 2.0)), 2.0);

    assert_close!(vec3(3.0, 4.0, 0.0).normalize(), vec3(0.6, 0.8, 0.0), EPS);
    assert_eq!(Vec3::ZERO.normalize(), Vec3::ZERO);
}

#[test]
fn vec3_conversions_and_defaults() {
    assert_eq!(Vec3::from([1.0, 2.0, 3.0]), vec3(1.0, 2.0, 3.0));
    assert_eq!(<[f32; 3]>::from(vec3(1.0, 2.0, 3.0)), [1.0, 2.0, 3.0]);
    assert_eq!(Vec3::default(), Vec3::ZERO);
    assert_eq!(Vec3::ONE, vec3(1.0, 1.0, 1.0));
}

#[test]
fn matrix_products() {
    assert_eq!(ROT_Z * Vec3::X, Vec3::Y);
    assert_eq!(ROT_Z * Vec3::Y, -Vec3::X);
    assert_eq!(Matrix3::IDENTITY * ROT_Z, ROT_Z);
    assert_eq!(ROT_Z * Matrix3::IDENTITY, ROT_Z);
    // Four quarter turns are a full turn.
    assert_close!(ROT_Z * ROT_Z * ROT_Z * ROT_Z, Matrix3::IDENTITY, EPS);
    // Applying a product is applying each factor, right to left.
    let (a, b, p) = (sample().rotation, ROT_Z, vec3(0.3, -1.0, 2.0));
    assert_close!((a * b) * p, a * (b * p), EPS);

    assert_eq!(
        Matrix3::IDENTITY * 2.0 - Matrix3::IDENTITY,
        Matrix3::IDENTITY
    );
    assert_eq!(
        Matrix3::IDENTITY + Matrix3::IDENTITY,
        Matrix3::IDENTITY * 2.0
    );
    assert_eq!(Matrix3::default(), Matrix3::IDENTITY);
}

#[test]
fn matrix_transpose_determinant_inverse() {
    assert_eq!(
        ROT_Z.transpose(),
        Matrix3::from_rows(Vec3::Y, -Vec3::X, Vec3::Z)
    );
    assert_eq!(ROT_Z.determinant(), 1.0);
    assert_eq!((Matrix3::IDENTITY * 2.0).determinant(), 8.0);

    let m = sample().rotation * 3.0;
    assert_close!(m * m.inverse().unwrap(), Matrix3::IDENTITY, EPS);
    // A rotation's inverse is its transpose.
    let r = sample().rotation;
    assert_close!(r.inverse().unwrap(), r.transpose(), EPS);

    let singular = Matrix3::from_rows(Vec3::X, Vec3::X, Vec3::Z);
    assert_eq!(singular.determinant(), 0.0);
    assert_eq!(singular.inverse(), None);
}

#[test]
fn transform_apply() {
    let t = Transform::new(vec3(1.0, 0.0, 0.0), ROT_Z, 2.0);
    // (1, 0, 0) scaled by 2 -> (2, 0, 0), rotated -> (0, 2, 0), translated -> (1, 2, 0).
    assert_eq!(t.apply(Vec3::X), vec3(1.0, 2.0, 0.0));
    assert_eq!(t * Vec3::X, t.apply(Vec3::X));
    assert_eq!(t.apply_to_vector(Vec3::X), vec3(0.0, 2.0, 0.0));
    assert_eq!(
        Transform::IDENTITY.apply(vec3(4.0, 5.0, 6.0)),
        vec3(4.0, 5.0, 6.0)
    );
    assert_eq!(Transform::default(), Transform::IDENTITY);
}

#[test]
fn transform_compose_and_inverse() {
    let t = sample();
    assert_eq!(Transform::IDENTITY * t, t);
    assert_eq!(t * Transform::IDENTITY, t);

    // Composing then applying is applying the child, then the parent.
    let parent = Transform::new(vec3(0.0, 3.0, -1.0), ROT_Z, 0.5);
    let p = vec3(0.25, -1.5, 2.0);
    assert_close!((parent * t).apply(p), parent.apply(t.apply(p)), EPS);

    let inv = t.inverse().unwrap();
    assert_close!(t * inv, Transform::IDENTITY, EPS);
    assert_close!(inv * t, Transform::IDENTITY, EPS);
    assert_close!(inv.apply(t.apply(p)), p, EPS);

    assert_eq!(
        Transform::new(Vec3::ZERO, Matrix3::IDENTITY, 0.0).inverse(),
        None
    );
    let singular = Matrix3::from_rows(Vec3::X, Vec3::X, Vec3::Z);
    assert_eq!(Transform::new(Vec3::ZERO, singular, 1.0).inverse(), None);
}

#[test]
fn compose_matches_global_transform() {
    // Icosphere: root -> Indirect1 -> Indirect2 -> shape, each with its own transform.
    let nif = open("assets/Icosphere.nif");
    let node = |name: &str| nif.find_block_by_name::<NiNode>(name).unwrap().transform();
    let root = nif.root_node().unwrap().transform();

    let global = root * node("Indirect1") * node("Indirect2");
    assert_close!(
        global,
        nif.node_transform_to_global("Indirect2").unwrap(),
        EPS
    );
    assert_close!(
        root * node("Indirect1"),
        nif.node_transform_to_global("Indirect1").unwrap(),
        EPS
    );
}

#[test]
fn round_trips_through_the_file() {
    let mut nif = NifFile::create(nifly::file::NifVersion::SKYRIM_SE);
    let t = sample();
    let i = nif.add_node("Node", t, None).unwrap();
    let node = nif.block_of::<NiNode>(i).unwrap();
    assert_eq!(node.transform(), t);
}

#[cfg(feature = "mint")]
#[test]
fn mint_conversions() {
    let a = vec3(1.0, 2.0, 3.0);
    let mv: mint::Vector3<f32> = a.into();
    assert_eq!((mv.x, mv.y, mv.z), (1.0, 2.0, 3.0));
    assert_eq!(Vec3::from(mv), a);
    let mp: mint::Point3<f32> = a.into();
    assert_eq!(Vec3::from(mp), a);

    // Rows stay rows; columns are the transpose.
    let rows: mint::RowMatrix3<f32> = ROT_Z.into();
    assert_eq!((rows.x.x, rows.x.y, rows.y.x), (0.0, -1.0, 1.0));
    assert_eq!(Matrix3::from(rows), ROT_Z);
    let cols: mint::ColumnMatrix3<f32> = ROT_Z.into();
    // Column 0 of ROT_Z is (0, 1, 0).
    assert_eq!((cols.x.x, cols.x.y, cols.x.z), (0.0, 1.0, 0.0));
    assert_eq!(Matrix3::from(cols), ROT_Z);
}

#[test]
fn constructors() {
    assert_eq!(vec3(1.0, 2.0, 3.0), Vec3::new(1.0, 2.0, 3.0));
    let t = vec3(1.0, 2.0, 3.0);
    assert_eq!(
        Transform::from_translation(t),
        Transform::new(t, Matrix3::IDENTITY, 1.0)
    );
    assert_eq!(
        Transform::from_rotation(ROT_Z),
        Transform::new(Vec3::ZERO, ROT_Z, 1.0)
    );
    assert_eq!(
        Transform::from_scale(2.0),
        Transform::new(Vec3::ZERO, Matrix3::IDENTITY, 2.0)
    );
}

#[test]
fn rotations_about_axes() {
    let quarter = std::f32::consts::FRAC_PI_2;
    assert_close!(Matrix3::from_rotation_z(quarter), ROT_Z, EPS);
    // Counterclockwise looking down each axis: X -> Y, Y -> Z, Z -> X.
    assert_close!(Matrix3::from_rotation_z(quarter) * Vec3::X, Vec3::Y, EPS);
    assert_close!(Matrix3::from_rotation_x(quarter) * Vec3::Y, Vec3::Z, EPS);
    assert_close!(Matrix3::from_rotation_y(quarter) * Vec3::Z, Vec3::X, EPS);
    for angle in [0.0, 0.3, -1.2, 2.5] {
        for r in [
            Matrix3::from_rotation_x(angle),
            Matrix3::from_rotation_y(angle),
            Matrix3::from_rotation_z(angle),
        ] {
            assert_close!(r * r.transpose(), Matrix3::IDENTITY, EPS);
            assert!((r.determinant() - 1.0).abs() < EPS);
        }
    }
    assert_eq!(Matrix3::from_rotation_x(0.0), Matrix3::IDENTITY);
}

#[test]
fn abs_diff_eq_compares_each_component() {
    let a = vec3(1.0, 2.0, 3.0);
    assert!(a.abs_diff_eq(&vec3(1.05, 1.95, 3.0), 0.1));
    assert!(!a.abs_diff_eq(&vec3(1.0, 2.0, 3.2), 0.1));
    assert!(a.abs_diff_eq(&a, 0.0));

    let m = sample().rotation;
    let mut n = m;
    n.rows[2].z += 0.05;
    assert!(m.abs_diff_eq(&n, 0.1) && !m.abs_diff_eq(&n, 0.01));

    let t = sample();
    let moved = Transform {
        translation: t.translation + vec3(0.0, 0.05, 0.0),
        ..t
    };
    let scaled = Transform {
        scale: t.scale + 0.05,
        ..t
    };
    let rotated = Transform { rotation: n, ..t };
    for other in [moved, scaled, rotated] {
        assert!(t.abs_diff_eq(&other, 0.1) && !t.abs_diff_eq(&other, 0.01));
    }

    let s = BoundingSphere::new(a, 2.0);
    assert!(s.abs_diff_eq(&BoundingSphere::new(vec3(1.05, 2.0, 3.0), 2.05), 0.1));
    assert!(!s.abs_diff_eq(&BoundingSphere::new(a, 2.5), 0.1));
}

#[test]
fn bounding_volume_abs_diff_eq() {
    let sphere = |r: f32| BoundingVolume::Sphere(BoundingSphere::new(Vec3::ZERO, r));
    let capsule = |extent: f32| BoundingVolume::Capsule {
        center: Vec3::ZERO,
        origin: Vec3::X,
        extent,
        radius: 1.0,
    };
    let boxed = |e: f32| BoundingVolume::Box {
        center: Vec3::ZERO,
        axes: [Vec3::X, Vec3::Y, Vec3::Z],
        extents: [1.0, 1.0, e],
    };
    let half = |c: f32| BoundingVolume::HalfSpace {
        normal: Vec3::Z,
        constant: c,
        center: Vec3::ZERO,
    };
    for (a, b) in [
        (sphere(1.0), sphere(1.05)),
        (capsule(1.0), capsule(1.05)),
        (boxed(1.0), boxed(1.05)),
        (half(1.0), half(1.05)),
    ] {
        assert!(a.abs_diff_eq(&b, 0.1) && !a.abs_diff_eq(&b, 0.01), "{a:?}");
    }
    let union = |r: f32| BoundingVolume::Union(vec![sphere(r), BoundingVolume::Base]);
    assert!(union(1.0).abs_diff_eq(&union(1.05), 0.1));
    assert!(!union(1.0).abs_diff_eq(&BoundingVolume::Union(vec![sphere(1.0)]), 0.1));

    // Different kinds never match.
    assert!(!sphere(1.0).abs_diff_eq(&capsule(1.0), 10.0));
    assert!(BoundingVolume::Other(3).abs_diff_eq(&BoundingVolume::Other(3), 0.0));
    assert!(!BoundingVolume::Other(3).abs_diff_eq(&BoundingVolume::Other(6), 10.0));
}
