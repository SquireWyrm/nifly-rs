//! Vectors, rotation matrices and transforms.
//!
//! With the `mint` feature, [`Vec3`] and [`Matrix3`] convert to and from the `mint` types.

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use nifly_sys::object_3d::ffi;

/// A 3D vector or point.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0);
    pub const ONE: Self = Self::new(1.0, 1.0, 1.0);
    pub const X: Self = Self::new(1.0, 0.0, 0.0);
    pub const Y: Self = Self::new(0.0, 1.0, 0.0);
    pub const Z: Self = Self::new(0.0, 0.0, 1.0);

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn distance(self, other: Self) -> f32 {
        (self - other).length()
    }

    /// The vector scaled to length 1, or unchanged if its length is 0.
    pub fn normalize(self) -> Self {
        let len = self.length();
        if len == 0.0 { self } else { self / len }
    }

    pub(crate) fn to_ffi(self) -> ffi::Vec3 {
        ffi::Vec3 { x: self.x, y: self.y, z: self.z }
    }

    pub(crate) fn from_ffi(v: ffi::Vec3) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from([x, y, z]: [f32; 3]) -> Self {
        Self::new(x, y, z)
    }
}

impl From<Vec3> for [f32; 3] {
    fn from(v: Vec3) -> Self {
        [v.x, v.y, v.z]
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, o: Self) -> Self {
        Self::new(self.x + o.x, self.y + o.y, self.z + o.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, o: Self) -> Self {
        Self::new(self.x - o.x, self.y - o.y, self.z - o.z)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, f: f32) -> Self {
        Self::new(self.x * f, self.y * f, self.z * f)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, f: f32) -> Self {
        Self::new(self.x / f, self.y / f, self.z / f)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, o: Self) {
        *self = *self + o;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, o: Self) {
        *self = *self - o;
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, f: f32) {
        *self = *self * f;
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, f: f32) {
        *self = *self / f;
    }
}

/// A 3×3 matrix stored as rows, applied to column vectors (`m * v`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix3 {
    pub rows: [Vec3; 3],
}

impl Default for Matrix3 {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Matrix3 {
    pub const IDENTITY: Self = Self::from_rows(Vec3::X, Vec3::Y, Vec3::Z);

    pub const fn from_rows(r0: Vec3, r1: Vec3, r2: Vec3) -> Self {
        Self { rows: [r0, r1, r2] }
    }

    pub fn transpose(&self) -> Self {
        let [a, b, c] = self.rows;
        Self::from_rows(
            Vec3::new(a.x, b.x, c.x),
            Vec3::new(a.y, b.y, c.y),
            Vec3::new(a.z, b.z, c.z),
        )
    }

    pub fn determinant(&self) -> f32 {
        let [a, b, c] = self.rows;
        a.x * (b.y * c.z - b.z * c.y) + a.y * (b.z * c.x - b.x * c.z) + a.z * (b.x * c.y - b.y * c.x)
    }

    /// The inverse, or `None` if the determinant is 0.
    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det == 0.0 {
            return None;
        }
        let i = 1.0 / det;
        let [a, b, c] = self.rows;
        Some(Self::from_rows(
            Vec3::new((b.y * c.z - b.z * c.y) * i, (c.y * a.z - c.z * a.y) * i, (a.y * b.z - a.z * b.y) * i),
            Vec3::new((b.z * c.x - b.x * c.z) * i, (c.z * a.x - c.x * a.z) * i, (a.z * b.x - a.x * b.z) * i),
            Vec3::new((b.x * c.y - b.y * c.x) * i, (c.x * a.y - c.y * a.x) * i, (a.x * b.y - a.y * b.x) * i),
        ))
    }

    fn column(&self, i: usize) -> Vec3 {
        let pick = |r: Vec3| <[f32; 3]>::from(r)[i];
        Vec3::new(pick(self.rows[0]), pick(self.rows[1]), pick(self.rows[2]))
    }
}

impl Mul for Matrix3 {
    type Output = Self;

    fn mul(self, o: Self) -> Self {
        let cols = [o.column(0), o.column(1), o.column(2)];
        let row = |r: Vec3| Vec3::new(r.dot(cols[0]), r.dot(cols[1]), r.dot(cols[2]));
        Self::from_rows(row(self.rows[0]), row(self.rows[1]), row(self.rows[2]))
    }
}

impl Mul<Vec3> for Matrix3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self.rows[0].dot(v), self.rows[1].dot(v), self.rows[2].dot(v))
    }
}

impl Mul<f32> for Matrix3 {
    type Output = Self;

    fn mul(self, f: f32) -> Self {
        Self { rows: self.rows.map(|r| r * f) }
    }
}

impl Add for Matrix3 {
    type Output = Self;

    fn add(self, o: Self) -> Self {
        let [a, b, c] = self.rows;
        let [d, e, f] = o.rows;
        Self::from_rows(a + d, b + e, c + f)
    }
}

impl Sub for Matrix3 {
    type Output = Self;

    fn sub(self, o: Self) -> Self {
        let [a, b, c] = self.rows;
        let [d, e, f] = o.rows;
        Self::from_rows(a - d, b - e, c - f)
    }
}

/// Translation, rotation and uniform scale from an object's coordinate system to its parent's.
///
/// `parent * child` composes two transforms, and `transform * point` applies one.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Matrix3,
    pub scale: f32,
}

impl Default for Transform {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Transform {
    pub const IDENTITY: Self = Self::new(Vec3::ZERO, Matrix3::IDENTITY, 1.0);

    pub const fn new(translation: Vec3, rotation: Matrix3, scale: f32) -> Self {
        Self { translation, rotation, scale }
    }

    /// Maps a point into the parent's coordinate system.
    pub fn apply(&self, point: Vec3) -> Vec3 {
        self.translation + self.rotation * (point * self.scale)
    }

    /// Maps a direction or offset into the parent's coordinate system, without translating it.
    pub fn apply_to_vector(&self, v: Vec3) -> Vec3 {
        self.rotation * (v * self.scale)
    }

    /// The transform back from the parent's coordinate system, or `None` if the rotation can't
    /// be inverted or the scale is 0.
    pub fn inverse(&self) -> Option<Self> {
        if self.scale == 0.0 {
            return None;
        }
        let rotation = self.rotation.inverse()?;
        let scale = 1.0 / self.scale;
        let translation = -((rotation * self.translation) * scale);
        Some(Self::new(translation, rotation, scale))
    }

    pub(crate) fn to_ffi(self) -> ffi::Transform {
        ffi::Transform {
            translation: self.translation.to_ffi(),
            rotation: self.rotation.rows.map(Vec3::to_ffi),
            scale: self.scale,
        }
    }

    pub(crate) fn from_ffi(t: ffi::Transform) -> Self {
        Self::new(
            Vec3::from_ffi(t.translation),
            Matrix3 { rows: t.rotation.map(Vec3::from_ffi) },
            t.scale,
        )
    }
}

impl Mul for Transform {
    type Output = Self;

    /// `self` is the parent: the result maps from `child`'s coordinate system to `self`'s parent's.
    fn mul(self, child: Self) -> Self {
        Self::new(
            self.translation + self.rotation * (child.translation * self.scale),
            self.rotation * child.rotation,
            self.scale * child.scale,
        )
    }
}

impl Mul<Vec3> for Transform {
    type Output = Vec3;

    fn mul(self, point: Vec3) -> Vec3 {
        self.apply(point)
    }
}

/// A sphere enclosing a set of points, such as the vertices a bone influences.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct BoundingSphere {
    pub center: Vec3,
    pub radius: f32,
}

impl BoundingSphere {
    pub const fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }

    pub(crate) fn to_ffi(self) -> ffi::BoundingSphere {
        ffi::BoundingSphere { center: self.center.to_ffi(), radius: self.radius }
    }

    pub(crate) fn from_ffi(b: ffi::BoundingSphere) -> Self {
        Self::new(Vec3::from_ffi(b.center), b.radius)
    }
}

/// An object's bounding volume, saved in files up to version 4.2.2.0. See
/// [`NiAvObject::bounding_volume`](crate::objects::NiAvObject::bounding_volume).
#[derive(Debug, Clone, PartialEq)]
pub enum BoundingVolume {
    /// A volume with no shape.
    Base,
    Sphere(BoundingSphere),
    Box {
        center: Vec3,
        axes: [Vec3; 3],
        extents: [f32; 3],
    },
    Capsule {
        center: Vec3,
        origin: Vec3,
        extent: f32,
        radius: f32,
    },
    Union(Vec<BoundingVolume>),
    HalfSpace {
        normal: Vec3,
        constant: f32,
        center: Vec3,
    },
    /// A type with no data of its own, by its stored value.
    Other(u32),
}

impl BoundingVolume {
    const BASE: u32 = u32::MAX;
    const SPHERE: u32 = 0;
    const BOX: u32 = 1;
    const CAPSULE: u32 = 2;
    const UNION: u32 = 4;
    const HALF_SPACE: u32 = 5;

    /// Appends the volume's records in pre-order.
    pub(crate) fn to_ffi(&self, out: &mut Vec<ffi::BoundingVolumeFfi>) {
        let mut values = [0.0; 15];
        let mut put = |at: usize, v: &[f32]| values[at..at + v.len()].copy_from_slice(v);
        let (kind, children) = match self {
            Self::Base => (Self::BASE, 0),
            Self::Sphere(s) => {
                put(0, &<[f32; 3]>::from(s.center));
                put(3, &[s.radius]);
                (Self::SPHERE, 0)
            }
            Self::Box { center, axes, extents } => {
                put(0, &<[f32; 3]>::from(*center));
                for (i, axis) in axes.iter().enumerate() {
                    put(3 + 3 * i, &<[f32; 3]>::from(*axis));
                }
                put(12, extents);
                (Self::BOX, 0)
            }
            Self::Capsule { center, origin, extent, radius } => {
                put(0, &<[f32; 3]>::from(*center));
                put(3, &<[f32; 3]>::from(*origin));
                put(6, &[*extent, *radius]);
                (Self::CAPSULE, 0)
            }
            Self::Union(children) => (Self::UNION, children.len() as u32),
            Self::HalfSpace { normal, constant, center } => {
                put(0, &<[f32; 3]>::from(*normal));
                put(3, &[*constant]);
                put(4, &<[f32; 3]>::from(*center));
                (Self::HALF_SPACE, 0)
            }
            Self::Other(kind) => (*kind, 0),
        };
        out.push(ffi::BoundingVolumeFfi { kind, values, children });
        if let Self::Union(children) = self {
            children.iter().for_each(|c| c.to_ffi(out));
        }
    }

    /// Reads one volume from pre-order records, or `None` if they run out.
    pub(crate) fn from_ffi(records: &mut impl Iterator<Item = ffi::BoundingVolumeFfi>) -> Option<Self> {
        let r = records.next()?;
        let v = r.values;
        let vec3 = |at: usize| Vec3::new(v[at], v[at + 1], v[at + 2]);
        Some(match r.kind {
            Self::BASE => Self::Base,
            Self::SPHERE => Self::Sphere(BoundingSphere::new(vec3(0), v[3])),
            Self::BOX => Self::Box { center: vec3(0), axes: [vec3(3), vec3(6), vec3(9)], extents: [v[12], v[13], v[14]] },
            Self::CAPSULE => Self::Capsule { center: vec3(0), origin: vec3(3), extent: v[6], radius: v[7] },
            Self::UNION => Self::Union((0..r.children).map(|_| Self::from_ffi(records)).collect::<Option<_>>()?),
            Self::HALF_SPACE => Self::HalfSpace { normal: vec3(0), constant: v[3], center: vec3(4) },
            kind => Self::Other(kind),
        })
    }
}

#[cfg(feature = "mint")]
mod mint_impls {
    use super::{Matrix3, Vec3};

    impl From<mint::Vector3<f32>> for Vec3 {
        fn from(v: mint::Vector3<f32>) -> Self {
            Self::new(v.x, v.y, v.z)
        }
    }

    impl From<Vec3> for mint::Vector3<f32> {
        fn from(v: Vec3) -> Self {
            Self { x: v.x, y: v.y, z: v.z }
        }
    }

    impl From<mint::Point3<f32>> for Vec3 {
        fn from(p: mint::Point3<f32>) -> Self {
            Self::new(p.x, p.y, p.z)
        }
    }

    impl From<Vec3> for mint::Point3<f32> {
        fn from(v: Vec3) -> Self {
            Self { x: v.x, y: v.y, z: v.z }
        }
    }

    impl From<mint::RowMatrix3<f32>> for Matrix3 {
        fn from(m: mint::RowMatrix3<f32>) -> Self {
            Self::from_rows(m.x.into(), m.y.into(), m.z.into())
        }
    }

    impl From<Matrix3> for mint::RowMatrix3<f32> {
        fn from(m: Matrix3) -> Self {
            let [x, y, z] = m.rows.map(Into::into);
            Self { x, y, z }
        }
    }

    impl From<mint::ColumnMatrix3<f32>> for Matrix3 {
        fn from(m: mint::ColumnMatrix3<f32>) -> Self {
            Self::from_rows(m.x.into(), m.y.into(), m.z.into()).transpose()
        }
    }

    impl From<Matrix3> for mint::ColumnMatrix3<f32> {
        fn from(m: Matrix3) -> Self {
            let [x, y, z] = m.transpose().rows.map(Into::into);
            Self { x, y, z }
        }
    }
}
