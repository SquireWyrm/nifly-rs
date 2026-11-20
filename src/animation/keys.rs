//! Keys and key groups.

use super::*;

/// How values between two keys are computed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum Interpolation {
    /// A straight line between the values.
    Linear = 1,
    /// A curve through each key's [`forward`](Key::forward) and [`backward`](Key::backward)
    /// tangents.
    Quadratic = 2,
    /// A curve shaped by each key's [tension, bias and continuity](Key::tbc).
    Tbc = 3,
    /// The value of the earlier key, until the next.
    Constant = 5,
}

/// Tension, bias and continuity, which shape a [`Tbc`](Interpolation::Tbc) curve.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Tbc {
    pub tension: f32,
    pub bias: f32,
    pub continuity: f32,
}

/// A value at a point in time.
///
/// Only the fields its group's [`interpolation`](KeyGroup::interpolation) uses are saved: the
/// tangents for [`Quadratic`](Interpolation::Quadratic) and `tbc` for
/// [`Tbc`](Interpolation::Tbc).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Key<T> {
    pub time: f32,
    pub value: T,
    /// The tangent leaving the key.
    pub forward: T,
    /// The tangent arriving at the key.
    pub backward: T,
    pub tbc: Tbc,
}

impl<T: Default> Key<T> {
    /// A key with no tangents or TBC values.
    pub fn new(time: f32, value: T) -> Self {
        Self {
            time,
            value,
            forward: T::default(),
            backward: T::default(),
            tbc: Tbc::default(),
        }
    }
}

/// Keys and their interpolation.
///
/// A group with keys needs an `interpolation`; setters return `false`, changing nothing,
/// without one. An empty group saves none, except in [`Morph::keys`].
#[derive(Clone, Debug, PartialEq)]
pub struct KeyGroup<T> {
    /// `None` if the stored value isn't an [`Interpolation`].
    pub interpolation: Option<Interpolation>,
    pub keys: Vec<Key<T>>,
}

impl<T> Default for KeyGroup<T> {
    fn default() -> Self {
        Self {
            interpolation: None,
            keys: Vec::new(),
        }
    }
}

impl<T> KeyGroup<T> {
    pub fn new(interpolation: Interpolation, keys: Vec<Key<T>>) -> Self {
        Self {
            interpolation: Some(interpolation),
            keys,
        }
    }
}

/// A type key values can have: `f32`, `bool`, [`Vec3`] or `[f32; 4]` (a color as RGBA, or a
/// quaternion as `[x, y, z, w]`).
pub trait KeyValue: Sealed + Copy + Default {
    #[doc(hidden)]
    fn to_floats(self) -> [f32; 4];
    #[doc(hidden)]
    fn from_floats(v: [f32; 4]) -> Self;
}

impl Sealed for f32 {}
impl KeyValue for f32 {
    fn to_floats(self) -> [f32; 4] {
        [self, 0.0, 0.0, 0.0]
    }

    fn from_floats(v: [f32; 4]) -> Self {
        v[0]
    }
}

impl Sealed for bool {}
impl KeyValue for bool {
    fn to_floats(self) -> [f32; 4] {
        [f32::from(u8::from(self)), 0.0, 0.0, 0.0]
    }

    fn from_floats(v: [f32; 4]) -> Self {
        v[0] != 0.0
    }
}

impl Sealed for Vec3 {}
impl KeyValue for Vec3 {
    fn to_floats(self) -> [f32; 4] {
        [self.x, self.y, self.z, 0.0]
    }

    fn from_floats(v: [f32; 4]) -> Self {
        Vec3::new(v[0], v[1], v[2])
    }
}

impl Sealed for [f32; 4] {}
impl KeyValue for [f32; 4] {
    fn to_floats(self) -> [f32; 4] {
        self
    }

    fn from_floats(v: [f32; 4]) -> Self {
        v
    }
}

pub(crate) fn key_from_ffi<T: KeyValue>(k: &ffi::KeyFfi) -> Key<T> {
    let [tension, bias, continuity] = k.tbc;
    Key {
        time: k.time,
        value: T::from_floats(k.value),
        forward: T::from_floats(k.forward),
        backward: T::from_floats(k.backward),
        tbc: Tbc {
            tension,
            bias,
            continuity,
        },
    }
}

pub(crate) fn key_to_ffi<T: KeyValue>(k: &Key<T>) -> ffi::KeyFfi {
    ffi::KeyFfi {
        time: k.time,
        value: k.value.to_floats(),
        forward: k.forward.to_floats(),
        backward: k.backward.to_floats(),
        tbc: [k.tbc.tension, k.tbc.bias, k.tbc.continuity],
    }
}

pub(crate) fn group_from_ffi<T: KeyValue>(g: ffi::KeyGroupFfi) -> KeyGroup<T> {
    KeyGroup {
        interpolation: Interpolation::try_from(g.interpolation).ok(),
        keys: g.keys.iter().map(key_from_ffi).collect(),
    }
}

pub(crate) fn group_to_ffi<T: KeyValue>(g: &KeyGroup<T>) -> ffi::KeyGroupFfi {
    ffi::KeyGroupFfi {
        interpolation: g.interpolation.map_or(0, u32::from),
        keys: g.keys.iter().map(key_to_ffi).collect(),
    }
}
