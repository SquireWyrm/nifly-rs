//! Tests for Vec3, Matrix3 and Transform.

use std::fs;

use nifly::{
    file::NifFile,
    nodes::NiNode,
    object3d::{Matrix3, Transform, Vec3},
};

const EPS: f32 = 1e-5;

fn v(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, y, z)
}

fn assert_vec_close(a: Vec3, b: Vec3) {
    assert!(a.distance(b) < EPS, "{a:?} != {b:?}");
}

fn assert_matrix_close(a: Matrix3, b: Matrix3) {
    for (r, s) in a.rows.into_iter().zip(b.rows) {
        assert_vec_close(r, s);
    }
}

fn assert_transform_close(a: Transform, b: Transform) {
    assert_vec_close(a.translation, b.translation);
    assert_matrix_close(a.rotation, b.rotation);
    assert!(
        (a.scale - b.scale).abs() < EPS,
        "scale {} != {}",
        a.scale,
        b.scale
    );
}

/// 90° about Z: X -> Y, Y -> -X.
const ROT_Z: Matrix3 = Matrix3::from_rows(v_const(0.0, -1.0, 0.0), v_const(1.0, 0.0, 0.0), Vec3::Z);

const fn v_const(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, y, z)
}

/// A rotation that isn't about a single axis, with a translation and scale.
fn sample() -> Transform {
    // About X by 0.6 rad, then about Z by 90°.
    let (s, c) = 0.6f32.sin_cos();
    let about_x = Matrix3::from_rows(Vec3::X, v(0.0, c, -s), v(0.0, s, c));
    Transform::new(v(1.0, -2.0, 0.5), ROT_Z * about_x, 1.75)
}

#[test]
fn vec3_operators() {
    let a = v(1.0, 2.0, 3.0);
    let b = v(-4.0, 0.5, 2.0);
    assert_eq!(a + b, v(-3.0, 2.5, 5.0));
    assert_eq!(a - b, v(5.0, 1.5, 1.0));
    assert_eq!(-a, v(-1.0, -2.0, -3.0));
    assert_eq!(a * 2.0, v(2.0, 4.0, 6.0));
    assert_eq!(2.0 * a, a * 2.0);
    assert_eq!(a / 2.0, v(0.5, 1.0, 1.5));

    let mut c = a;
    c += b;
    c -= b;
    c *= 4.0;
    c /= 2.0;
    assert_eq!(c, a * 2.0);
}

#[test]
fn vec3_products_and_lengths() {
    let a = v(1.0, 2.0, 3.0);
    let b = v(-4.0, 0.5, 2.0);
    assert_eq!(a.dot(b), 3.0);
    assert_eq!(Vec3::X.cross(Vec3::Y), Vec3::Z);
    assert_eq!(a.cross(b).dot(a), 0.0);
    assert_eq!(v(3.0, 4.0, 0.0).length(), 5.0);
    assert_eq!(v(3.0, 4.0, 0.0).length_squared(), 25.0);
    assert_eq!(Vec3::ZERO.distance(v(0.0, 0.0, 2.0)), 2.0);

    assert_vec_close(v(3.0, 4.0, 0.0).normalize(), v(0.6, 0.8, 0.0));
    assert_eq!(Vec3::ZERO.normalize(), Vec3::ZERO);
}

#[test]
fn vec3_conversions_and_defaults() {
    assert_eq!(Vec3::from([1.0, 2.0, 3.0]), v(1.0, 2.0, 3.0));
    assert_eq!(<[f32; 3]>::from(v(1.0, 2.0, 3.0)), [1.0, 2.0, 3.0]);
    assert_eq!(Vec3::default(), Vec3::ZERO);
    assert_eq!(Vec3::ONE, v(1.0, 1.0, 1.0));
}

#[test]
fn matrix_products() {
    assert_eq!(ROT_Z * Vec3::X, Vec3::Y);
    assert_eq!(ROT_Z * Vec3::Y, -Vec3::X);
    assert_eq!(Matrix3::IDENTITY * ROT_Z, ROT_Z);
    assert_eq!(ROT_Z * Matrix3::IDENTITY, ROT_Z);
    // Four quarter turns are a full turn.
    assert_matrix_close(ROT_Z * ROT_Z * ROT_Z * ROT_Z, Matrix3::IDENTITY);
    // Applying a product is applying each factor, right to left.
    let (a, b, p) = (sample().rotation, ROT_Z, v(0.3, -1.0, 2.0));
    assert_vec_close((a * b) * p, a * (b * p));

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
    assert_matrix_close(m * m.inverse().unwrap(), Matrix3::IDENTITY);
    // A rotation's inverse is its transpose.
    let r = sample().rotation;
    assert_matrix_close(r.inverse().unwrap(), r.transpose());

    let singular = Matrix3::from_rows(Vec3::X, Vec3::X, Vec3::Z);
    assert_eq!(singular.determinant(), 0.0);
    assert_eq!(singular.inverse(), None);
}

#[test]
fn transform_apply() {
    let t = Transform::new(v(1.0, 0.0, 0.0), ROT_Z, 2.0);
    // (1, 0, 0) scaled by 2 -> (2, 0, 0), rotated -> (0, 2, 0), translated -> (1, 2, 0).
    assert_eq!(t.apply(Vec3::X), v(1.0, 2.0, 0.0));
    assert_eq!(t * Vec3::X, t.apply(Vec3::X));
    assert_eq!(t.apply_to_vector(Vec3::X), v(0.0, 2.0, 0.0));
    assert_eq!(
        Transform::IDENTITY.apply(v(4.0, 5.0, 6.0)),
        v(4.0, 5.0, 6.0)
    );
    assert_eq!(Transform::default(), Transform::IDENTITY);
}

#[test]
fn transform_compose_and_inverse() {
    let t = sample();
    assert_eq!(Transform::IDENTITY * t, t);
    assert_eq!(t * Transform::IDENTITY, t);

    // Composing then applying is applying the child, then the parent.
    let parent = Transform::new(v(0.0, 3.0, -1.0), ROT_Z, 0.5);
    let p = v(0.25, -1.5, 2.0);
    assert_vec_close((parent * t).apply(p), parent.apply(t.apply(p)));

    let inv = t.inverse().unwrap();
    assert_transform_close(t * inv, Transform::IDENTITY);
    assert_transform_close(inv * t, Transform::IDENTITY);
    assert_vec_close(inv.apply(t.apply(p)), p);

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
    let nif = NifFile::load(fs::read("assets/Icosphere.nif").unwrap().as_slice(), None).unwrap();
    let node = |name: &str| nif.find_block_by_name::<NiNode>(name).unwrap().transform();
    let root = nif.root_node().unwrap().transform();

    let global = root * node("Indirect1") * node("Indirect2");
    assert_transform_close(global, nif.node_transform_to_global("Indirect2").unwrap());
    assert_transform_close(
        root * node("Indirect1"),
        nif.node_transform_to_global("Indirect1").unwrap(),
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
    let a = v(1.0, 2.0, 3.0);
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
