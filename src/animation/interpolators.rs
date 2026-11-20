//! Interpolator fields and adding interpolators.

use super::*;

/// A translation, rotation and scale, as interpolators and B-splines store them.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QuatTransform {
    pub translation: Vec3,
    /// A quaternion, `[x, y, z, w]`.
    pub rotation: [f32; 4],
    pub scale: f32,
}

impl Default for QuatTransform {
    /// No translation or rotation, and a scale of 1.
    fn default() -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: [0.0, 0.0, 0.0, 1.0],
            scale: 1.0,
        }
    }
}

impl QuatTransform {
    fn from_ffi(t: ffi::QuatTransformFfi) -> Self {
        Self {
            translation: Vec3::from_ffi(t.translation),
            rotation: t.rotation,
            scale: t.scale,
        }
    }

    fn to_ffi(self) -> ffi::QuatTransformFfi {
        ffi::QuatTransformFfi {
            translation: self.translation.to_ffi(),
            rotation: self.rotation,
            scale: self.scale,
        }
    }
}

/// Getter and setter pairs for `QuatTransform` fields, whose shims take the value by reference.
macro_rules! transform_fields {
    ($($(#[$doc:meta])* $get:ident, $set:ident = $ffi_get:ident, $ffi_set:ident;)*) => {$(
        $(#[$doc])*
        pub fn $get(&self) -> QuatTransform {
            QuatTransform::from_ffi(ffi::$ffi_get(&self.0))
        }

        #[doc = concat!("Sets [`", stringify!($get), "`](Self::", stringify!($get), ").")]
        pub fn $set(&mut self, value: QuatTransform) {
            ffi::$ffi_set(self.ffi_mut(), &value.to_ffi());
        }
    )*};
}

/// The stored bool for an unset value.
const UNSET_BOOL: u8 = 2;

fn optional_bool(v: u8) -> Option<bool> {
    match v {
        0 => Some(false),
        1 => Some(true),
        _ => None,
    }
}

fn optional_bool_to_ffi(v: Option<bool>) -> u8 {
    v.map_or(UNSET_BOOL, u8::from)
}

fn optional_float(v: f32) -> Option<f32> {
    (v != f32::MIN).then_some(v)
}

fn optional_float_to_ffi(v: Option<f32>) -> f32 {
    v.unwrap_or(f32::MIN)
}

/// Unset if every component is `f32::MIN`.
fn optional_vec3(v: ffi::Vec3) -> Option<Vec3> {
    let v = Vec3::from_ffi(v);
    (v != Vec3::new(f32::MIN, f32::MIN, f32::MIN)).then_some(v)
}

fn optional_vec3_to_ffi(v: Option<Vec3>) -> ffi::Vec3 {
    v.unwrap_or(Vec3::new(f32::MIN, f32::MIN, f32::MIN))
        .to_ffi()
}

impl NiBoolInterpolator {
    fields! {
        /// The pose value, used without keys, or `None` if it's unset. Values other than 0 and
        /// 1 read as `None`.
        value, set_value: Option<bool> = bool_interpolator_value, set_bool_interpolator_value, optional_bool, optional_bool_to_ffi;
    }
}

impl NiFloatInterpolator {
    fields! {
        /// The pose value, used without keys, or `None` if it's unset (stored as `f32::MIN`).
        value, set_value: Option<f32> = float_interpolator_value, set_float_interpolator_value, optional_float, optional_float_to_ffi;
    }
}

impl NiPoint3Interpolator {
    fields! {
        /// The pose value, used without keys, or `None` if it's unset (every component stored
        /// as `f32::MIN`).
        value, set_value: Option<Vec3> = point3_interpolator_value, set_point3_interpolator_value, optional_vec3, optional_vec3_to_ffi;
    }
}

impl NiTransformInterpolator {
    transform_fields! {
        /// The pose, used without keys. Unset parts are `f32::MIN`: each translation
        /// component, the rotation's components and the scale.
        transform, set_transform = transform_interpolator_value, set_transform_interpolator_value;
    }
}

bitflags::bitflags! {
    /// The flags of an [`NiPathInterpolator`] or [`NiPathController`]. Other bits are kept as
    /// stored.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct PathFlags: u16 {
        /// The path's control points need updating.
        const CV_DATA_NEEDS_UPDATE = 1;
        /// The path is open rather than a closed loop.
        const CURVE_TYPE_OPEN = 1 << 1;
        const ALLOW_FLIP = 1 << 2;
        /// Bank into turns.
        const BANK = 1 << 3;
        const CONSTANT_VELOCITY = 1 << 4;
        /// Turn to face along the path.
        const FOLLOW = 1 << 5;
        const FLIP = 1 << 6;
        const _ = !0;
    }
}

impl NiPathInterpolator {
    fields! {
        flags, set_flags: PathFlags = path_flags, set_path_flags, PathFlags::from_bits_retain, |f: PathFlags| f.bits();
        /// Which way to bank: 1 or -1.
        bank_direction, set_bank_direction: i32 = path_bank_direction, set_path_bank_direction, same, same;
        /// In radians.
        max_bank_angle, set_max_bank_angle: f32 = path_max_bank_angle, set_path_max_bank_angle, same, same;
        smoothing, set_smoothing: f32 = path_smoothing, set_path_smoothing, same, same;
        /// The axis to face along the path when following it: 0 X, 1 Y, 2 Z.
        follow_axis, set_follow_axis: u16 = path_follow_axis, set_path_follow_axis, same, same;
    }
}

bitflags::bitflags! {
    /// The flags of an [`NiLookAtInterpolator`] or [`NiLookAtController`]: which axis looks at
    /// the target (X when neither axis flag is set). Other bits are kept as stored.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct LookAtFlags: u16 {
        /// Look away from the target instead.
        const FLIP = 1;
        const Y_AXIS = 1 << 1;
        const Z_AXIS = 1 << 2;
        const _ = !0;
    }
}

impl NiLookAtInterpolator {
    fields! {
        flags, set_flags: LookAtFlags = look_at_flags, set_look_at_flags, LookAtFlags::from_bits_retain, |f: LookAtFlags| f.bits();
    }

    /// The name of the [look-at node](crate::handle::View::look_at_node).
    pub fn look_at_name(&self) -> &BStr {
        ffi::look_at_name(&self.0).as_bstr()
    }

    pub fn set_look_at_name(&mut self, name: impl AsRef<[u8]>) {
        ffi::set_look_at_name(self.ffi_mut(), name.as_ref());
    }

    transform_fields! {
        /// The pose.
        transform, set_transform = look_at_transform, set_look_at_transform;
    }

    fields! {
        /// Whether the [transform](Self::transform)'s translation, rotation and scale are set.
        /// Saved only before 10.1.0.110.
        transform_valid, set_transform_valid: [bool; 3] = look_at_transform_valid, set_look_at_transform_valid, same, same;
    }
}

bitflags::bitflags! {
    /// The flags of an [`NiBlendInterpolator`], saved from 10.1.0.112 on. Other bits are kept as
    /// stored.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct BlendFlags: u8 {
        /// Blended by a controller manager, which keeps the items, so they aren't saved.
        const MANAGER_CONTROLLED = 1;
        const _ = !0;
    }
}

/// An interpolator blended by an [`NiBlendInterpolator`], with its weight and priority.
///
/// `priority` is saved from 10.1.0.110 on and `priority_32` before; the two aren't converted
/// into each other.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BlendItem {
    /// The [`NiInterpolator`], or `None`.
    pub interpolator: Option<Index>,
    pub weight: f32,
    pub normalized_weight: f32,
    pub priority: u8,
    pub priority_32: u32,
    pub ease_spinner: f32,
}

/// Fields by version:
/// - [`flags`](Self::flags) and [`weight_threshold`](Self::weight_threshold): from 10.1.0.112.
///   Older files keep the threshold separately and the flags in
///   [`manager_controlled`](Self::manager_controlled) and
///   [`only_use_highest_weight`](Self::only_use_highest_weight).
/// - The items ([`View::blend_items`](crate::handle::View::blend_items)): before 10.1.0.112, and
///   later only without [`MANAGER_CONTROLLED`](BlendFlags::MANAGER_CONTROLLED). The blending
///   state fields are saved with them from 10.1.0.112 on.
/// - Fields with a `_16` or `_32` variant: the plain one from 10.1.0.110, the variant before.
///   The two aren't converted into each other.
impl NiBlendInterpolator {
    fields! {
        flags, set_flags: BlendFlags = blend_flags, set_blend_flags, BlendFlags::from_bits_retain, |f: BlendFlags| f.bits();
        /// The number of items saved, as a byte from 10.1.0.110 on. Items are cut or padded with
        /// empty ones to this many when saved;
        /// [`Edit::set_blend_items`](crate::handle::Edit::set_blend_items) keeps it in step.
        array_size, set_array_size: u16 = blend_array_size, set_blend_array_size, same, same;
        /// Saved before 10.1.0.110.
        array_grow_by, set_array_grow_by: u16 = blend_array_grow_by, set_blend_array_grow_by, same, same;
        weight_threshold, set_weight_threshold: f32 = blend_weight_threshold, set_blend_weight_threshold, same, same;
        /// Saved before 10.1.0.112.
        manager_controlled, set_manager_controlled: bool = blend_manager_controlled, set_blend_manager_controlled, same, same;
        /// Saved before 10.1.0.112.
        only_use_highest_weight, set_only_use_highest_weight: bool =
            blend_only_use_highest_weight, set_blend_only_use_highest_weight, same, same;
        /// The number of items in use, as a byte from 10.1.0.110 on.
        interpolator_count, set_interpolator_count: u16 = blend_interp_count, set_blend_interp_count, same, same;
        single_index, set_single_index: u8 = blend_single_index, set_blend_single_index, same, same;
        single_index_16, set_single_index_16: u16 = blend_single_index_16, set_blend_single_index_16, same, same;
        high_priority, set_high_priority: i8 = blend_high_priority, set_blend_high_priority, same, same;
        high_priority_32, set_high_priority_32: i32 = blend_high_priority_32, set_blend_high_priority_32, same, same;
        next_high_priority, set_next_high_priority: i8 =
            blend_next_high_priority, set_blend_next_high_priority, same, same;
        next_high_priority_32, set_next_high_priority_32: i32 =
            blend_next_high_priority_32, set_blend_next_high_priority_32, same, same;
        /// Saved from 10.1.0.108 up to 10.1.0.111, and later with the items.
        single_time, set_single_time: f32 = blend_single_time, set_blend_single_time, same, same;
        /// Saved from 10.1.0.112 on.
        high_weights_sum, set_high_weights_sum: f32 = blend_high_weights_sum, set_blend_high_weights_sum, same, same;
        /// Saved from 10.1.0.112 on.
        next_high_weights_sum, set_next_high_weights_sum: f32 =
            blend_next_high_weights_sum, set_blend_next_high_weights_sum, same, same;
        /// Saved from 10.1.0.112 on.
        high_ease_spinner, set_high_ease_spinner: f32 = blend_high_ease_spinner, set_blend_high_ease_spinner, same, same;
    }
}

impl NiBlendBoolInterpolator {
    fields! {
        /// The blended value, or `None` if it's unset. Values other than 0 and 1 read as `None`.
        value, set_value: Option<bool> = blend_bool_value, set_blend_bool_value, optional_bool, optional_bool_to_ffi;
    }
}

impl NiBlendFloatInterpolator {
    fields! {
        /// The blended value, or `None` if it's unset (stored as `f32::MIN`).
        value, set_value: Option<f32> = blend_float_value, set_blend_float_value, optional_float, optional_float_to_ffi;
    }
}

impl NiBlendPoint3Interpolator {
    fields! {
        /// The blended value, or `None` if it's unset (every component stored as `f32::MIN`).
        value, set_value: Option<Vec3> = blend_point3_value, set_blend_point3_value, optional_vec3, optional_vec3_to_ffi;
    }
}

impl NiBlendTransformInterpolator {
    transform_fields! {
        /// The blended transform. Saved before 10.1.0.110.
        transform, set_transform = blend_transform_value, set_blend_transform_value;
    }

    fields! {
        /// Whether the [transform](Self::transform)'s translation, rotation and scale are set.
        /// Saved before 10.1.0.110.
        transform_valid, set_transform_valid: [bool; 3] = blend_transform_valid, set_blend_transform_valid, same, same;
    }
}

/// A tread of a [`BsTreadTransfInterpolator`]: a name and two transforms.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Tread {
    pub name: BString,
    pub transform_1: QuatTransform,
    pub transform_2: QuatTransform,
}

impl BsTreadTransfInterpolator {
    pub fn treads(&self) -> Vec<Tread> {
        ffi::treads(&self.0)
            .into_iter()
            .map(|t| Tread {
                name: t.name.into(),
                transform_1: QuatTransform::from_ffi(t.transform_1),
                transform_2: QuatTransform::from_ffi(t.transform_2),
            })
            .collect()
    }

    pub fn set_treads(&mut self, treads: &[Tread]) {
        let treads: Vec<_> = treads
            .iter()
            .map(|t| ffi::TreadFfi {
                name: t.name.to_vec(),
                transform_1: t.transform_1.to_ffi(),
                transform_2: t.transform_2.to_ffi(),
            })
            .collect();
        ffi::set_treads(self.ffi_mut(), &treads);
    }
}

impl NiBSplineInterpolator {
    fields! {
        start_time, set_start_time: f32 = bspline_start_time, set_bspline_start_time, same, same;
        stop_time, set_stop_time: f32 = bspline_stop_time, set_bspline_stop_time, same, same;
    }
}

impl NiBSplineCompFloatInterpolator {
    fields! {
        /// The value without control points.
        base, set_base: f32 = bspline_float_base, set_bspline_float_base, same, same;
        /// Where the control points start in the [`NiBSplineData`], or `u32::MAX` for none.
        offset, set_offset: u32 = bspline_float_offset, set_bspline_float_offset, same, same;
        bias, set_bias: f32 = bspline_float_bias, set_bspline_float_bias, same, same;
        multiplier, set_multiplier: f32 = bspline_float_multiplier, set_bspline_float_multiplier, same, same;
    }
}

impl NiBSplinePoint3Interpolator {
    fields! {
        /// The value without control points.
        value, set_value: Vec3 = bspline_point3_value, set_bspline_point3_value, vec3, vec3_to_ffi;
        /// Where the control points start in the [`NiBSplineData`], or `u32::MAX` for none.
        handle, set_handle: u32 = bspline_point3_handle, set_bspline_point3_handle, same, same;
    }
}

impl NiBSplineCompPoint3Interpolator {
    fields! {
        position_offset, set_position_offset: f32 = bspline_point3_offset, set_bspline_point3_offset, same, same;
        position_half_range, set_position_half_range: f32 =
            bspline_point3_half_range, set_bspline_point3_half_range, same, same;
    }
}

impl NiBSplineTransformInterpolator {
    transform_fields! {
        /// The transform without control points.
        transform, set_transform = bspline_transform_value, set_bspline_transform_value;
    }

    fields! {
        /// Where the translation control points start in the [`NiBSplineData`], or `u32::MAX`
        /// for none. The same goes for the rotation and scale offsets.
        translation_offset, set_translation_offset: u32 =
            bspline_translation_offset, set_bspline_translation_offset, same, same;
        rotation_offset, set_rotation_offset: u32 = bspline_rotation_offset, set_bspline_rotation_offset, same, same;
        scale_offset, set_scale_offset: u32 = bspline_scale_offset, set_bspline_scale_offset, same, same;
    }
}

impl NiBSplineCompTransformInterpolator {
    fields! {
        translation_bias, set_translation_bias: f32 = bspline_translation_bias, set_bspline_translation_bias, same, same;
        translation_multiplier, set_translation_multiplier: f32 =
            bspline_translation_multiplier, set_bspline_translation_multiplier, same, same;
        rotation_bias, set_rotation_bias: f32 = bspline_rotation_bias, set_bspline_rotation_bias, same, same;
        rotation_multiplier, set_rotation_multiplier: f32 =
            bspline_rotation_multiplier, set_bspline_rotation_multiplier, same, same;
        scale_bias, set_scale_bias: f32 = bspline_scale_bias, set_bspline_scale_bias, same, same;
        scale_multiplier, set_scale_multiplier: f32 = bspline_scale_multiplier, set_bspline_scale_multiplier, same, same;
    }
}

/// An interpolator [`NifFile::add_interpolator`] can add.
pub trait NewInterpolator: NiType + Sealed {
    #[doc(hidden)]
    fn add(file: Pin<&mut nifly_sys::nif_file::ffi::NifFile>) -> u32;
}

macro_rules! new_interpolator {
    ($($t:ident => $add:ident;)*) => {$(
        impl NewInterpolator for $t {
            fn add(file: Pin<&mut nifly_sys::nif_file::ffi::NifFile>) -> u32 {
                ffi::$add(file)
            }
        }
    )*};
}

new_interpolator! {
    NiBoolInterpolator => add_ni_bool_interpolator;
    NiBoolTimelineInterpolator => add_ni_bool_timeline_interpolator;
    NiFloatInterpolator => add_ni_float_interpolator;
    NiTransformInterpolator => add_ni_transform_interpolator;
    BsRotAccumTransfInterpolator => add_bs_rot_accum_transf_interpolator;
    NiPoint3Interpolator => add_ni_point3_interpolator;
    NiPathInterpolator => add_ni_path_interpolator;
    NiLookAtInterpolator => add_ni_look_at_interpolator;
    BsTreadTransfInterpolator => add_bs_tread_transf_interpolator;
    NiBSplineCompFloatInterpolator => add_ni_bspline_comp_float_interpolator;
    NiBSplineCompPoint3Interpolator => add_ni_bspline_comp_point3_interpolator;
    NiBSplineTransformInterpolator => add_ni_bspline_transform_interpolator;
    NiBSplineCompTransformInterpolator => add_ni_bspline_comp_transform_interpolator;
    NiBlendBoolInterpolator => add_ni_blend_bool_interpolator;
    NiBlendFloatInterpolator => add_ni_blend_float_interpolator;
    NiBlendPoint3Interpolator => add_ni_blend_point3_interpolator;
    NiBlendTransformInterpolator => add_ni_blend_transform_interpolator;
}
