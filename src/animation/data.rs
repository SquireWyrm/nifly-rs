//! Key data blocks and other animation data.

use super::*;

impl NiPosData {
    key_groups! {
        keys, set_keys: Vec3 = pos_data_keys, set_pos_data_keys;
    }
}

impl NiFloatData {
    key_groups! {
        keys, set_keys: f32 = float_data_keys, set_float_data_keys;
    }
}

impl NiBoolData {
    key_groups! {
        keys, set_keys: bool = bool_data_keys, set_bool_data_keys;
    }
}

impl NiColorData {
    key_groups! {
        /// Colors as RGBA.
        keys, set_keys: [f32; 4] = color_data_keys, set_color_data_keys;
    }
}

/// The rotation keys of an [`NiKeyframeData`].
#[derive(Clone, Debug, PartialEq)]
pub enum Rotations {
    /// Quaternions as `[x, y, z, w]`. Their tangents aren't saved, even for
    /// [`Quadratic`](Interpolation::Quadratic) keys.
    Quaternion(KeyGroup<[f32; 4]>),
    /// Angles around the X, Y and Z axes, in radians, each with its own keys.
    Xyz([KeyGroup<f32>; 3]),
}

/// The `XYZ_ROTATION_KEY` rotation type.
const XYZ_ROTATION: u32 = 4;

impl NiKeyframeData {
    pub fn rotations(&self) -> Rotations {
        if ffi::keyframe_rotation_type(&self.0) == XYZ_ROTATION {
            Rotations::Xyz([0, 1, 2].map(|axis| group_from_ffi(ffi::keyframe_xyz_rotations(&self.0, axis))))
        } else {
            Rotations::Quaternion(group_from_ffi(ffi::keyframe_quaternion_keys(&self.0)))
        }
    }

    /// Replaces the [rotations](Self::rotations). Returns `false`, changing nothing, if a
    /// group has keys but no interpolation.
    pub fn set_rotations(&mut self, rotations: &Rotations) -> bool {
        match rotations {
            Rotations::Quaternion(keys) => ffi::set_keyframe_quaternion_keys(self.ffi_mut(), &group_to_ffi(keys)),
            Rotations::Xyz([x, y, z]) => {
                ffi::set_keyframe_xyz_rotations(self.ffi_mut(), &group_to_ffi(x), &group_to_ffi(y), &group_to_ffi(z))
            }
        }
    }

    key_groups! {
        translations, set_translations: Vec3 = keyframe_translations, set_keyframe_translations;
        scales, set_scales: f32 = keyframe_scales, set_keyframe_scales;
    }
}

impl NiUvData {
    /// The key group `which` (0 u translation, 1 v translation, 2 u scale, 3 v scale).
    fn uv(&self, which: usize) -> KeyGroup<f32> {
        group_from_ffi(ffi::uv_data_keys(&self.0, which))
    }

    pub fn u_translations(&self) -> KeyGroup<f32> {
        self.uv(0)
    }

    pub fn v_translations(&self) -> KeyGroup<f32> {
        self.uv(1)
    }

    pub fn u_scales(&self) -> KeyGroup<f32> {
        self.uv(2)
    }

    pub fn v_scales(&self) -> KeyGroup<f32> {
        self.uv(3)
    }

    /// Replaces the [u translations](Self::u_translations). Returns `false`, changing nothing,
    /// if the group has keys but no interpolation. The same goes for the other setters.
    pub fn set_u_translations(&mut self, keys: &KeyGroup<f32>) -> bool {
        ffi::set_uv_data_keys(self.ffi_mut(), 0, &group_to_ffi(keys))
    }

    pub fn set_v_translations(&mut self, keys: &KeyGroup<f32>) -> bool {
        ffi::set_uv_data_keys(self.ffi_mut(), 1, &group_to_ffi(keys))
    }

    pub fn set_u_scales(&mut self, keys: &KeyGroup<f32>) -> bool {
        ffi::set_uv_data_keys(self.ffi_mut(), 2, &group_to_ffi(keys))
    }

    pub fn set_v_scales(&mut self, keys: &KeyGroup<f32>) -> bool {
        ffi::set_uv_data_keys(self.ffi_mut(), 3, &group_to_ffi(keys))
    }
}

impl NiVisData {
    /// The visibility keys. Only each key's time and value are saved.
    pub fn keys(&self) -> Vec<Key<bool>> {
        ffi::vis_data_keys(&self.0).iter().map(key_from_ffi).collect()
    }

    pub fn set_keys(&mut self, keys: &[Key<bool>]) {
        let keys: Vec<_> = keys.iter().map(key_to_ffi).collect();
        ffi::set_vis_data_keys(self.ffi_mut(), &keys);
    }
}

/// A morph target of an [`NiMorphData`]: an offset for each vertex.
///
/// Fields by version:
/// - `name`: saved from 10.1.0.106 on.
/// - `keys`: saved up to 10.1.0.0, with the interpolation even when there are no keys. Later
///   files keep a morph's weight in an interpolator instead.
/// - `legacy_weight`: saved from 10.1.0.104 up to 20.1.0.2, in files with a stream version
///   below 10 (not Bethesda's).
/// - `vectors`: always saved.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Morph {
    pub name: BString,
    /// The morph's weight over time.
    pub keys: KeyGroup<f32>,
    pub legacy_weight: f32,
    /// One per vertex.
    pub vectors: Vec<Vec3>,
}

impl NiMorphData {
    /// The number of vertices, which each morph has a vector for.
    pub fn vertex_count(&self) -> u32 {
        ffi::morph_vertex_count(&self.0)
    }

    /// Whether the morphs are offsets from the first one (the base), rather than positions.
    /// Other stored values read as `true`.
    pub fn relative_targets(&self) -> bool {
        ffi::morph_relative_targets(&self.0) != 0
    }

    pub fn set_relative_targets(&mut self, relative: bool) {
        ffi::set_morph_relative_targets(self.ffi_mut(), relative.into());
    }

    pub fn morphs(&self) -> Vec<Morph> {
        ffi::morphs(&self.0)
            .into_iter()
            .map(|m| Morph {
                name: m.name.into(),
                keys: group_from_ffi(m.keys),
                legacy_weight: m.legacy_weight,
                vectors: m.vectors.into_iter().map(Vec3::from_ffi).collect(),
            })
            .collect()
    }

    /// Replaces the morphs, setting the [vertex count](Self::vertex_count) to their number of
    /// vectors. Returns `false`, changing nothing, unless every morph has the same number of
    /// vectors and its keys have an interpolation (if any).
    pub fn set_morphs(&mut self, morphs: &[Morph]) -> bool {
        let morphs: Vec<_> = morphs
            .iter()
            .map(|m| ffi::MorphFfi {
                name: m.name.to_vec(),
                keys: group_to_ffi(&m.keys),
                legacy_weight: m.legacy_weight,
                vectors: m.vectors.iter().map(|v| v.to_ffi()).collect(),
            })
            .collect();
        ffi::set_morphs(self.ffi_mut(), &morphs)
    }
}

impl NiBSplineData {
    /// The full-precision control points, shared by the interpolators that use this block.
    pub fn float_control_points(&self) -> &[f32] {
        ffi::bspline_float_points(&self.0)
    }

    pub fn set_float_control_points(&mut self, points: &[f32]) {
        ffi::set_bspline_float_points(self.ffi_mut(), points);
    }

    /// The compressed control points, which compressed interpolators scale by their offsets and
    /// half-ranges.
    pub fn short_control_points(&self) -> &[i16] {
        ffi::bspline_short_points(&self.0)
    }

    pub fn set_short_control_points(&mut self, points: &[i16]) {
        ffi::set_bspline_short_points(self.ffi_mut(), points);
    }
}

impl NiBSplineBasisData {
    pub fn control_point_count(&self) -> u32 {
        ffi::bspline_basis_count(&self.0)
    }

    pub fn set_control_point_count(&mut self, count: u32) {
        ffi::set_bspline_basis_count(self.ffi_mut(), count);
    }
}

impl NiStringPalette {
    /// The packed strings, separated by zero bytes. The last string's zero byte is saved but
    /// not included.
    pub fn palette(&self) -> &BStr {
        ffi::string_palette(&self.0).as_bstr()
    }

    /// Replaces the [palette](Self::palette). A final zero byte is dropped from it and saved
    /// anyway. Offsets into the palette, such as a sequence's, aren't updated.
    pub fn set_palette(&mut self, palette: impl AsRef<[u8]>) {
        ffi::set_string_palette(self.ffi_mut(), palette.as_ref());
    }

    /// The string starting at `offset`, up to the next zero byte, or `None` if `offset` is past
    /// the end.
    pub fn string_at(&self, offset: usize) -> Option<&BStr> {
        let rest = self.palette().get(offset..)?;
        Some(rest.split(|&b| b == 0).next().unwrap_or_default().as_bstr())
    }
}

/// An animation data block [`NifFile::add_animation_data`] can add.
pub trait NewAnimationData: NiType + Sealed {
    #[doc(hidden)]
    fn add(file: Pin<&mut nifly_sys::nif_file::ffi::NifFile>) -> u32;
}

macro_rules! new_animation_data {
    ($($t:ident => $add:ident;)*) => {$(
        impl NewAnimationData for $t {
            fn add(file: Pin<&mut nifly_sys::nif_file::ffi::NifFile>) -> u32 {
                ffi::$add(file)
            }
        }
    )*};
}

new_animation_data! {
    NiKeyframeData => add_ni_keyframe_data;
    NiTransformData => add_ni_transform_data;
    NiPosData => add_ni_pos_data;
    NiBoolData => add_ni_bool_data;
    NiFloatData => add_ni_float_data;
    NiColorData => add_ni_color_data;
    NiUvData => add_ni_uv_data;
    NiVisData => add_ni_vis_data;
    NiMorphData => add_ni_morph_data;
    NiBSplineData => add_ni_bspline_data;
    NiBSplineBasisData => add_ni_bspline_basis_data;
    NiStringPalette => add_ni_string_palette;
    BsAnimNote => add_bs_anim_note;
    BsAnimNotes => add_bs_anim_notes;
}
