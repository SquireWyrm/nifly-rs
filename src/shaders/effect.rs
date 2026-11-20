//! BSEffectShaderProperty.

use super::*;

/// # Fields by game
///
/// Not every game stores every field. Each field's doc says which games save it. Elsewhere you
/// can still read and set the field, but the value isn't written when the file is saved.
///
/// In Fallout 76 and Starfield, and in Fallout 4 files with a stream version above 130, a shader
/// with a name saves none of these fields. Its name is the path of a material file, which holds
/// them instead.
impl BsEffectShaderProperty {
    /// How the textures repeat, or `None` if the stored value isn't a [`ClampMode`].
    pub fn texture_clamp_mode(&self) -> Option<ClampMode> {
        ClampMode::try_from(u32::from(self.clamp_byte(0))).ok()
    }

    /// Sets the [texture clamp mode](Self::texture_clamp_mode).
    pub fn set_texture_clamp_mode(&mut self, mode: ClampMode) {
        self.set_clamp_byte(0, u32::from(mode) as u8);
    }

    /// How much scene lighting affects the shader, from 0 (none) to 255.
    pub fn lighting_influence(&self) -> u8 {
        self.clamp_byte(1)
    }

    /// Sets the [lighting influence](Self::lighting_influence).
    pub fn set_lighting_influence(&mut self, influence: u8) {
        self.set_clamp_byte(1, influence);
    }

    /// The smallest mipmap level used for the environment map.
    pub fn env_map_min_lod(&self) -> u8 {
        self.clamp_byte(2)
    }

    /// Sets the [environment map's minimum LOD](Self::env_map_min_lod).
    pub fn set_env_map_min_lod(&mut self, lod: u8) {
        self.set_clamp_byte(2, lod);
    }

    /// The last byte of the field holding the [clamp mode](Self::texture_clamp_mode), of unknown
    /// meaning.
    pub fn clamp_unknown(&self) -> u8 {
        self.clamp_byte(3)
    }

    /// Sets [`clamp_unknown`](Self::clamp_unknown).
    pub fn set_clamp_unknown(&mut self, value: u8) {
        self.set_clamp_byte(3, value);
    }

    /// Byte `i` of the field holding the clamp mode, lighting influence, environment map
    /// minimum LOD and an unknown byte, lowest first.
    fn clamp_byte(&self, i: usize) -> u8 {
        ffi::effect_texture_clamp_bytes(&self.0).to_le_bytes()[i]
    }

    fn set_clamp_byte(&mut self, i: usize, value: u8) {
        let mut bytes = ffi::effect_texture_clamp_bytes(&self.0).to_le_bytes();
        bytes[i] = value;
        ffi::set_effect_texture_clamp_bytes(self.ffi_mut(), u32::from_le_bytes(bytes));
    }

    texture_paths! {
        /// The base texture.
        source_texture, set_source_texture = effect_source_texture, set_effect_source_texture;
        /// The palette the base texture's greyscale is mapped through.
        greyscale_texture, set_greyscale_texture = effect_greyscale_texture, set_effect_greyscale_texture;
        /// Only saved from Fallout 4 on.
        env_map_texture, set_env_map_texture = effect_env_map_texture, set_effect_env_map_texture;
        /// Only saved from Fallout 4 on.
        normal_texture, set_normal_texture = effect_normal_texture, set_effect_normal_texture;
        /// Only saved from Fallout 4 on.
        env_mask_texture, set_env_mask_texture = effect_env_mask_texture, set_effect_env_mask_texture;
        /// Only saved in Fallout 76 and Starfield.
        reflectance_texture, set_reflectance_texture = effect_reflectance_texture, set_effect_reflectance_texture;
        /// Only saved in Fallout 76 and Starfield.
        lighting_texture, set_lighting_texture = effect_lighting_texture, set_effect_lighting_texture;
        /// Only saved in Fallout 76 and Starfield.
        emit_gradient_texture, set_emit_gradient_texture =
            effect_emit_gradient_texture, set_effect_emit_gradient_texture;
    }

    fields! {
        /// Angle in degrees, between the view and the surface normal, where falloff starts.
        falloff_start_angle, set_falloff_start_angle: f32 =
            effect_falloff_start_angle, set_effect_falloff_start_angle, same, same;
        /// Angle in degrees where falloff stops.
        falloff_stop_angle, set_falloff_stop_angle: f32 =
            effect_falloff_stop_angle, set_effect_falloff_stop_angle, same, same;
        /// Opacity where falloff starts.
        falloff_start_opacity, set_falloff_start_opacity: f32 =
            effect_falloff_start_opacity, set_effect_falloff_start_opacity, same, same;
        /// Opacity where falloff stops.
        falloff_stop_opacity, set_falloff_stop_opacity: f32 =
            effect_falloff_stop_opacity, set_effect_falloff_stop_opacity, same, same;
        /// Only saved in Fallout 76.
        refraction_power, set_refraction_power: f32 =
            effect_refraction_power, set_effect_refraction_power, same, same;
        /// Base (emissive) color as `[r, g, b, a]`.
        base_color, set_base_color: [f32; 4] = effect_base_color, set_effect_base_color, rgba, rgba_to_ffi;
        /// Multiplier for the base color.
        base_color_scale, set_base_color_scale: f32 =
            effect_base_color_scale, set_effect_base_color_scale, same, same;
        /// Depth over which soft particles fade where they meet geometry.
        soft_falloff_depth, set_soft_falloff_depth: f32 =
            effect_soft_falloff_depth, set_effect_soft_falloff_depth, same, same;
        /// Strength of the environment map. Only saved from Fallout 4 on. This shader doesn't
        /// save [`BsShaderProperty::environment_map_scale`].
        env_map_scale, set_env_map_scale: f32 = effect_env_map_scale, set_effect_env_map_scale, same, same;
        /// Emittance color as `[r, g, b]`. Only saved in Fallout 76 and Starfield.
        emittance_color, set_emittance_color: [f32; 3] =
            effect_emittance_color, set_effect_emittance_color, rgb, rgb_to_ffi;
        /// Luminance of the emittance. Only saved in Fallout 76 and Starfield.
        lum_emittance, set_lum_emittance: f32 =
            effect_lum_emittance, set_effect_lum_emittance, same, same;
        /// Only saved in Fallout 76 and Starfield.
        exposure_offset, set_exposure_offset: f32 =
            effect_exposure_offset, set_effect_exposure_offset, same, same;
        /// Only saved in Fallout 76 and Starfield.
        final_exposure_min, set_final_exposure_min: f32 =
            effect_final_exposure_min, set_effect_final_exposure_min, same, same;
        /// Only saved in Fallout 76 and Starfield.
        final_exposure_max, set_final_exposure_max: f32 =
            effect_final_exposure_max, set_effect_final_exposure_max, same, same;
        /// A value of unknown meaning. Only saved in Starfield.
        unknown_float, set_unknown_float: f32 = effect_unknown_float, set_effect_unknown_float, same, same;
        /// Values of unknown meaning. Only saved in Starfield.
        unknown_bytes, set_unknown_bytes: [u8; 7] = effect_unknown_bytes, set_effect_unknown_bytes, same, same;
        /// Values of unknown meaning. Only saved in Starfield.
        unknown_floats, set_unknown_floats: [f32; 6] =
            effect_unknown_floats, set_effect_unknown_floats, same, same;
        /// A value of unknown meaning. Only saved in Starfield.
        unknown_byte, set_unknown_byte: u8 = effect_unknown_byte, set_effect_unknown_byte, same, same;
    }
}
