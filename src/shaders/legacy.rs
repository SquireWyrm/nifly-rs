//! Shader properties of Oblivion and Fallout 3.

use super::*;

/// The shader program of a Fallout 3 or New Vegas shader, from
/// [`BsShaderProperty::legacy_shader_type`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum LegacyShaderType {
    TallGrass = 0,
    Default = 1,
    Sky = 10,
    Skin = 14,
    Water = 17,
    Lighting30 = 29,
    Tile = 32,
    NoLighting = 33,
}

impl BsShaderLightingProperty {
    /// How the textures repeat, or `None` if the stored value isn't a [`ClampMode`]. Only saved
    /// in Fallout 3 and New Vegas files.
    pub fn texture_clamp_mode(&self) -> Option<ClampMode> {
        ClampMode::try_from(ffi::shader_lighting_clamp_mode(&self.0)).ok()
    }

    /// Sets the [texture clamp mode](Self::texture_clamp_mode).
    pub fn set_texture_clamp_mode(&mut self, mode: ClampMode) {
        ffi::set_shader_lighting_clamp_mode(self.ffi_mut(), mode.into());
    }
}

/// Its texture set is [`View::texture_set`].
impl BsShaderPpLightingProperty {
    fields! {
        /// Refraction strength. Saved in Fallout 3 and New Vegas files.
        refraction_strength, set_refraction_strength: f32 =
            pp_lighting_refraction_strength, set_pp_lighting_refraction_strength, same, same;
        /// Refraction fire period. Saved in Fallout 3 and New Vegas files.
        refraction_fire_period, set_refraction_fire_period: i32 =
            pp_lighting_refraction_fire_period, set_pp_lighting_refraction_fire_period, same, same;
        /// Maximum number of parallax passes. Saved in Fallout 3 and New Vegas files.
        parallax_max_passes, set_parallax_max_passes: f32 =
            pp_lighting_parallax_max_passes, set_pp_lighting_parallax_max_passes, same, same;
        /// Parallax scale. Saved in Fallout 3 and New Vegas files.
        parallax_scale, set_parallax_scale: f32 =
            pp_lighting_parallax_scale, set_pp_lighting_parallax_scale, same, same;
        /// Emissive color as `[r, g, b, a]`. Only saved in Skyrim and later files, not in Fallout
        /// 3 and New Vegas ones.
        emissive_color, set_emissive_color: [f32; 4] =
            pp_lighting_emissive_color, set_pp_lighting_emissive_color, rgba, rgba_to_ffi;
    }
}

impl BsShaderNoLightingProperty {
    texture_paths! {
        /// The texture path. Paths are as stored.
        base_texture, set_base_texture = no_lighting_base_texture, set_no_lighting_base_texture;
    }

    fields! {
        /// The view angle's cosine where falloff starts.
        falloff_start_angle, set_falloff_start_angle: f32 =
            no_lighting_falloff_start_angle, set_no_lighting_falloff_start_angle, same, same;
        /// The view angle's cosine where falloff stops.
        falloff_stop_angle, set_falloff_stop_angle: f32 =
            no_lighting_falloff_stop_angle, set_no_lighting_falloff_stop_angle, same, same;
        /// Opacity where falloff starts.
        falloff_start_opacity, set_falloff_start_opacity: f32 =
            no_lighting_falloff_start_opacity, set_no_lighting_falloff_start_opacity, same, same;
        /// Opacity where falloff stops.
        falloff_stop_opacity, set_falloff_stop_opacity: f32 =
            no_lighting_falloff_stop_opacity, set_no_lighting_falloff_stop_opacity, same, same;
    }
}

impl SkyShaderProperty {
    texture_paths! {
        /// The texture path. Paths are as stored.
        file_name, set_file_name = sky_file_name, set_sky_file_name;
    }

    /// What part of the sky this is, or `None` if the stored value isn't a [`SkyObjectType`].
    pub fn sky_object_type(&self) -> Option<SkyObjectType> {
        SkyObjectType::try_from(ffi::sky_object_type(&self.0)).ok()
    }

    /// Sets the [sky object type](Self::sky_object_type).
    pub fn set_sky_object_type(&mut self, object_type: SkyObjectType) {
        ffi::set_sky_object_type(self.ffi_mut(), object_type.into());
    }
}

/// What part of the sky a [`SkyShaderProperty`] or [`BsSkyShaderProperty`] draws.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum SkyObjectType {
    Texture = 0,
    Sunglare = 1,
    Sky = 2,
    Clouds = 3,
    Stars = 5,
    MoonStarsMask = 7,
}

impl TileShaderProperty {
    texture_paths! {
        /// The texture path. Paths are as stored.
        file_name, set_file_name = tile_file_name, set_tile_file_name;
    }
}

impl TallGrassShaderProperty {
    texture_paths! {
        /// The texture path. Paths are as stored.
        file_name, set_file_name = tall_grass_file_name, set_tall_grass_file_name;
    }
}

/// In Fallout 76 and Starfield, a shader with a name saves none of its fields.
impl BsWaterShaderProperty {
    fields! {
        flags, set_flags: WaterShaderFlags = bs_water_flags, set_bs_water_flags,
            WaterShaderFlags::from_bits_retain, |f: WaterShaderFlags| f.bits();
    }
}

bitflags! {
    /// The flags of a [`BsWaterShaderProperty`]. Other bits are kept as stored.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct WaterShaderFlags: u32 {
        const DISPLACEMENT = 1;
        const LOD = 1 << 1;
        const DEPTH = 1 << 2;
        const ACTOR_IN_WATER = 1 << 3;
        const ACTOR_IN_WATER_IS_MOVING = 1 << 4;
        const UNDERWATER = 1 << 5;
        const REFLECTIONS = 1 << 6;
        const REFRACTIONS = 1 << 7;
        const VERTEX_UV = 1 << 8;
        const VERTEX_ALPHA_DEPTH = 1 << 9;
        const PROCEDURAL = 1 << 10;
        const FOG = 1 << 11;
        const UPDATE_CONSTANTS = 1 << 12;
        const CUBEMAP = 1 << 13;
        const _ = !0;
    }
}

/// In Fallout 76 and Starfield, a shader with a name saves none of its fields.
impl BsSkyShaderProperty {
    texture_paths! {
        /// The texture path. Paths are as stored.
        base_texture, set_base_texture = bs_sky_base_texture, set_bs_sky_base_texture;
    }

    /// What part of the sky this is, or `None` if the stored value isn't a [`SkyObjectType`].
    pub fn sky_object_type(&self) -> Option<SkyObjectType> {
        SkyObjectType::try_from(ffi::bs_sky_object_type(&self.0)).ok()
    }

    /// Sets the [sky object type](Self::sky_object_type).
    pub fn set_sky_object_type(&mut self, object_type: SkyObjectType) {
        ffi::set_bs_sky_object_type(self.ffi_mut(), object_type.into());
    }
}
