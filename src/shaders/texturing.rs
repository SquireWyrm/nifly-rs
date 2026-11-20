//! NiTexturingProperty and its maps.

use super::*;

/// # Maps
///
/// A map is one of the fixed [`TextureMap`]s or a numbered [shader map](MapSlot::Shader). Its
/// fields are read and set through the `map_*` methods, which return `None` or `false`,
/// changing nothing, if the property doesn't have the map.
///
/// # Fields by version
///
/// Morrowind files are version 4.0.0.2, Oblivion's 20.0.0.4 or 20.0.0.5, and Fallout 3's and
/// New Vegas' 20.2.0.7. Fields a file's version doesn't store can still be read and set, but
/// aren't written when it's saved.
impl NiTexturingProperty {
    /// How the base map combines with the vertex and material colors, or `None` if the stored
    /// value isn't an apply mode. Saved only before Fallout 3; later files keep it in
    /// [`raw_flags`](Self::raw_flags).
    pub fn apply_mode(&self) -> Option<ApplyMode> {
        ApplyMode::try_from(ffi::texturing_apply_mode(&self.0)).ok()
    }

    pub fn set_apply_mode(self: Pin<&mut Self>, mode: ApplyMode) {
        ffi::set_texturing_apply_mode(self.ffi_mut(), mode.into());
    }

    fields! {
        /// The raw flags. From Fallout 3 on, bits 1–3 hold the apply mode. Saved in files up to
        /// version 10.0.1.2 and from 20.1.0.2 on.
        raw_flags, set_raw_flags: u16 = texturing_flags, set_texturing_flags, same, same;
        /// The number of map slots saved. The first five maps (base to glow) are always saved;
        /// later maps past the count aren't, and are lost on save. Files before Fallout 3 have no
        /// normal or parallax maps, so there the decals are slots 6 to 9.
        texture_count, set_texture_count: u32 = texturing_texture_count, set_texturing_texture_count, same, same;
        /// Scale of the bump map's luminance. Saved with the [bump map](TextureMap::Bump).
        bump_luma_scale, set_bump_luma_scale: f32 = texturing_luma_scale, set_texturing_luma_scale, same, same;
        /// Offset of the bump map's luminance. Saved with the [bump map](TextureMap::Bump).
        bump_luma_offset, set_bump_luma_offset: f32 =
            texturing_luma_offset, set_texturing_luma_offset, same, same;
        /// The bump map's 2×2 matrix, by rows. Saved with the [bump map](TextureMap::Bump).
        bump_matrix, set_bump_matrix: [f32; 4] = texturing_bump_matrix, set_texturing_bump_matrix, same, same;
        /// Offset of the parallax map. Saved with the [parallax map](TextureMap::Parallax).
        parallax_offset, set_parallax_offset: f32 =
            texturing_parallax_offset, set_texturing_parallax_offset, same, same;
    }

    /// Whether the property has `map`. A shader map counts while it's used.
    pub fn has_map(&self, map: impl Into<MapSlot>) -> bool {
        ffi::texturing_has_map(&self.0, map.into().to_ffi())
    }

    /// The block index of `map`'s source texture, or `None` if the property doesn't have `map`
    /// or its ref is empty. [`View::map_source`] gives the texture itself.
    pub fn map_source_index(&self, map: impl Into<MapSlot>) -> Option<Index> {
        index_from_ffi(ffi::texturing_map_source(&self.0, map.into().to_ffi()))
    }

    /// How `map` is repeated outside its UV range, or `None` if the stored value isn't a clamp
    /// mode. Only saved up to version 20.0.0.5 (Oblivion); later files keep it in
    /// [`map_raw_flags`](Self::map_raw_flags).
    pub fn map_clamp_mode(&self, map: impl Into<MapSlot>) -> Option<ClampMode> {
        ClampMode::try_from(self.map_field(map, ffi::texturing_map_clamp_mode)?).ok()
    }

    /// How `map` is filtered, or `None` if the stored value isn't a filter mode. Only saved up
    /// to version 20.0.0.5 (Oblivion); later files keep it in [`map_raw_flags`](Self::map_raw_flags).
    pub fn map_filter_mode(&self, map: impl Into<MapSlot>) -> Option<FilterMode> {
        FilterMode::try_from(self.map_field(map, ffi::texturing_map_filter_mode)?).ok()
    }

    /// The UV set `map` uses. Only saved up to version 20.0.0.5 (Oblivion); later files keep it
    /// in [`map_raw_flags`](Self::map_raw_flags).
    pub fn map_uv_set(&self, map: impl Into<MapSlot>) -> Option<u32> {
        self.map_field(map, ffi::texturing_map_uv_set)
    }

    /// `map`'s raw flags: bits 0–7 are the UV set, 8–11 the [filter mode](FilterMode) and 12–15
    /// the [clamp mode](ClampMode). Only saved from version 20.1.0.3 (Fallout 3) on; the
    /// separate fields aren't converted into them.
    pub fn map_raw_flags(&self, map: impl Into<MapSlot>) -> Option<u16> {
        self.map_field(map, ffi::texturing_map_flags).map(|v| v as u16)
    }

    /// `map`'s maximum anisotropy. Only saved from version 20.5.0.4 on.
    pub fn map_max_anisotropy(&self, map: impl Into<MapSlot>) -> Option<u16> {
        self.map_field(map, ffi::texturing_map_max_anisotropy).map(|v| v as u16)
    }

    /// `map`'s PlayStation 2 L and K values, as `[l, k]`. Only saved before version 10.4.0.1.
    pub fn map_ps2(&self, map: impl Into<MapSlot>) -> Option<[i16; 2]> {
        let map = map.into();
        let l = self.map_field(map, ffi::texturing_map_ps2_l)?;
        let k = self.map_field(map, ffi::texturing_map_ps2_k)?;
        Some([l as i16, k as i16])
    }

    /// A value of unknown meaning stored with `map`. Only saved up to version 4.1.0.12.
    pub fn map_unknown(&self, map: impl Into<MapSlot>) -> Option<u16> {
        self.map_field(map, ffi::texturing_map_unknown).map(|v| v as u16)
    }

    /// `map`'s texture coordinate transform, or `None` if the property doesn't have `map` or
    /// `map` has no transform. Only saved from version 10.1.0.0 on.
    pub fn map_transform(&self, map: impl Into<MapSlot>) -> Option<TextureTransform> {
        let mut out = TextureTransform::default().to_ffi();
        ffi::texturing_map_transform(&self.0, map.into().to_ffi(), &mut out)
            .then(|| TextureTransform::from_ffi(out))
    }

    pub fn set_map_clamp_mode(self: Pin<&mut Self>, map: impl Into<MapSlot>, mode: ClampMode) -> bool {
        ffi::set_texturing_map_clamp_mode(self.ffi_mut(), map.into().to_ffi(), mode.into())
    }

    pub fn set_map_filter_mode(self: Pin<&mut Self>, map: impl Into<MapSlot>, mode: FilterMode) -> bool {
        ffi::set_texturing_map_filter_mode(self.ffi_mut(), map.into().to_ffi(), mode.into())
    }

    pub fn set_map_uv_set(self: Pin<&mut Self>, map: impl Into<MapSlot>, uv_set: u32) -> bool {
        ffi::set_texturing_map_uv_set(self.ffi_mut(), map.into().to_ffi(), uv_set)
    }

    pub fn set_map_raw_flags(self: Pin<&mut Self>, map: impl Into<MapSlot>, flags: u16) -> bool {
        ffi::set_texturing_map_flags(self.ffi_mut(), map.into().to_ffi(), flags.into())
    }

    pub fn set_map_max_anisotropy(self: Pin<&mut Self>, map: impl Into<MapSlot>, value: u16) -> bool {
        ffi::set_texturing_map_max_anisotropy(self.ffi_mut(), map.into().to_ffi(), value.into())
    }

    pub fn set_map_ps2(mut self: Pin<&mut Self>, map: impl Into<MapSlot>, [l, k]: [i16; 2]) -> bool {
        let map = map.into().to_ffi();
        ffi::set_texturing_map_ps2_l(self.as_mut().ffi_mut(), map, l as u32)
            && ffi::set_texturing_map_ps2_k(self.ffi_mut(), map, k as u32)
    }

    pub fn set_map_unknown(self: Pin<&mut Self>, map: impl Into<MapSlot>, value: u16) -> bool {
        ffi::set_texturing_map_unknown(self.ffi_mut(), map.into().to_ffi(), value.into())
    }

    /// Gives `map` a texture coordinate transform, or removes it with `None`.
    pub fn set_map_transform(
        self: Pin<&mut Self>,
        map: impl Into<MapSlot>,
        transform: Option<TextureTransform>,
    ) -> bool {
        let value = transform.unwrap_or_default().to_ffi();
        ffi::set_texturing_map_transform(self.ffi_mut(), map.into().to_ffi(), transform.is_some(), &value)
    }

    /// The number of shader maps, used or not. Only saved from version 10.0.1.0 on.
    pub fn shader_map_count(&self) -> usize {
        ffi::texturing_shader_map_count(&self.0)
    }

    /// Changes the number of shader maps. New ones are unused; give them a texture with
    /// [`Edit::set_map_source`].
    pub fn set_shader_map_count(self: Pin<&mut Self>, count: usize) {
        ffi::set_texturing_shader_map_count(self.ffi_mut(), count);
    }

    /// The ID the shader uses for used shader map `index`, or `None` if it's unused or out of
    /// range.
    pub fn shader_map_id(&self, index: usize) -> Option<u32> {
        let mut out = 0;
        ffi::texturing_shader_map_id(&self.0, index, &mut out).then_some(out)
    }

    /// Returns `false`, changing nothing, if shader map `index` is unused or out of range.
    pub fn set_shader_map_id(self: Pin<&mut Self>, index: usize, id: u32) -> bool {
        ffi::set_texturing_shader_map_id(self.ffi_mut(), index, id)
    }

    fn map_field(
        &self,
        map: impl Into<MapSlot>,
        get: fn(&ffi::NiTexturingProperty, u32, &mut u32) -> bool,
    ) -> Option<u32> {
        let mut out = 0;
        get(&self.0, map.into().to_ffi(), &mut out).then_some(out)
    }
}

/// A map of an [`NiTexturingProperty`]: one of the fixed maps or a shader map. A
/// [`TextureMap`] converts into it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MapSlot {
    Map(TextureMap),
    /// The shader map at this position, below
    /// [`shader_map_count`](NiTexturingProperty::shader_map_count).
    Shader(usize),
}

impl From<TextureMap> for MapSlot {
    fn from(map: TextureMap) -> Self {
        MapSlot::Map(map)
    }
}

impl MapSlot {
    /// The bridge's map number: the fixed maps, then shader maps from 12. Out of range for a
    /// shader map that can't exist.
    pub(super) fn to_ffi(self) -> u32 {
        match self {
            MapSlot::Map(map) => map.into(),
            MapSlot::Shader(i) => u32::try_from(i).ok().and_then(|i| i.checked_add(12)).unwrap_or(u32::MAX),
        }
    }
}

/// A texture coordinate transform of an [`NiTexturingProperty`] map.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TextureTransform {
    pub translation: [f32; 2],
    pub scale: [f32; 2],
    /// Rotation in radians.
    pub rotation: f32,
    /// How the parts combine: 0 and 2 for Maya's order, 1 for 3ds Max's.
    pub transform_type: u32,
    /// The center of rotation and scaling.
    pub center: [f32; 2],
}

impl TextureTransform {
    fn from_ffi(t: ffi::TextureTransform) -> Self {
        Self {
            translation: uv(t.translation),
            scale: uv(t.scale),
            rotation: t.rotation,
            transform_type: t.transform_type,
            center: uv(t.center),
        }
    }

    pub(super) fn to_ffi(self) -> ffi::TextureTransform {
        ffi::TextureTransform {
            translation: uv_to_ffi(self.translation),
            scale: uv_to_ffi(self.scale),
            rotation: self.rotation,
            transform_type: self.transform_type,
            center: uv_to_ffi(self.center),
        }
    }
}

/// A map of an [`NiTexturingProperty`], in the order they're saved.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum TextureMap {
    /// The main color texture.
    Base = 0,
    /// Darkens the base, e.g. a light map.
    Dark = 1,
    /// Fine detail, usually tiled.
    Detail = 2,
    /// Where the surface is shiny.
    Gloss = 3,
    /// Light the surface emits.
    Glow = 4,
    /// A bump map.
    Bump = 5,
    /// A normal map (Fallout 3 and later).
    Normal = 6,
    /// A parallax map (Fallout 3 and later).
    Parallax = 7,
    Decal0 = 8,
    Decal1 = 9,
    Decal2 = 10,
    Decal3 = 11,
}

/// How an [`NiTexturingProperty`]'s base map combines with the vertex and material colors.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum ApplyMode {
    /// The texture replaces the color.
    Replace = 0,
    /// The texture is blended over the color by its alpha.
    Decal = 1,
    /// The texture and color are multiplied.
    Modulate = 2,
    /// Highlighting (PlayStation 2 only).
    Hilight = 3,
    /// A second highlighting mode (PlayStation 2 only).
    Hilight2 = 4,
}

/// How a texture repeats outside its UV range, along U (S) and V (T).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum ClampMode {
    /// Clamped along both.
    ClampBoth = 0,
    /// Clamped along U, repeated along V.
    ClampUWrapV = 1,
    /// Repeated along U, clamped along V.
    WrapUClampV = 2,
    /// Repeated along both.
    WrapBoth = 3,
}

/// How a texture is filtered when scaled.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum FilterMode {
    /// Nearest texel, no mipmaps.
    Nearest = 0,
    /// Bilinear, no mipmaps.
    Bilinear = 1,
    /// Bilinear between the two nearest mipmaps.
    Trilinear = 2,
    /// Nearest texel of the nearest mipmap.
    NearestMipNearest = 3,
    /// Nearest texel, blended between the two nearest mipmaps.
    NearestMipLinear = 4,
    /// Bilinear within the nearest mipmap.
    BilinearMipNearest = 5,
}
