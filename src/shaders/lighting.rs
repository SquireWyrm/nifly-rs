//! BSLightingShaderProperty.

use super::*;

/// # Fields by game
///
/// Not every game stores every field. Each field's doc says which games save it ("Skyrim" means
/// both LE and SE). Elsewhere you can still read and set the field, but the value isn't written
/// when the file is saved.
///
/// Some fields are also only saved for one [shader type](Self::shader_type).
///
/// In Fallout 76 and Starfield, a shader with a name saves none of these fields. Its name is the
/// path of a material file, which holds them instead.
impl BsLightingShaderProperty {
    /// The shader type, which picks the shader program and which type-specific fields are saved,
    /// or `None` if the stored value isn't a [`LightingShaderType`].
    pub fn shader_type(&self) -> Option<LightingShaderType> {
        LightingShaderType::try_from(ffi::lighting_shader_type(&self.0)).ok()
    }

    /// Sets the [shader type](Self::shader_type).
    ///
    /// # Fallout 76 and Starfield
    ///
    /// These games dropped the [`Parallax`](LightingShaderType::Parallax) type and store every
    /// later type one lower (FaceTint as 3, SkinTint as 4, ...). Loading reads this numbering
    /// correctly, but saving doesn't write it. For a shader without a name, a type from
    /// `Parallax` on is saved wrong:
    ///
    /// - The stored type is the next one: `Parallax` reloads as `FaceTint`, `FaceTint` as
    ///   `SkinTint`, `Eye` as `Cloud`.
    /// - The type-specific fields written are those of the type before the one set, while
    ///   readers expect those of the stored type. Where the two have different fields, the
    ///   rest of the block is misread, and some types (such as `HairTint`) give a file that
    ///   can't be loaded again.
    ///
    /// Types before `Parallax` save correctly. A shader with a name (a material file path)
    /// isn't affected: it stores neither its type nor its other fields, which come from the
    /// material file.
    pub fn set_shader_type(&mut self, shader_type: LightingShaderType) {
        ffi::set_lighting_shader_type(self.ffi_mut(), shader_type.into());
    }

    /// How the textures repeat, or `None` if the stored value isn't a [`ClampMode`].
    pub fn texture_clamp_mode(&self) -> Option<ClampMode> {
        ClampMode::try_from(ffi::lighting_texture_clamp_mode(&self.0)).ok()
    }

    /// Sets the [texture clamp mode](Self::texture_clamp_mode).
    pub fn set_texture_clamp_mode(&mut self, mode: ClampMode) {
        ffi::set_lighting_texture_clamp_mode(self.ffi_mut(), mode.into());
    }

    fields! {
        /// Emissive (glow) color as `[r, g, b]`.
        emissive_color, set_emissive_color: [f32; 3] =
            lighting_emissive_color, set_lighting_emissive_color, rgb, rgb_to_ffi;
        /// Multiplier for the emissive color.
        emissive_multiple, set_emissive_multiple: f32 =
            lighting_emissive_multiple, set_lighting_emissive_multiple, same, same;
        /// Opacity, from 0 (transparent) to 1.
        alpha, set_alpha: f32 = lighting_alpha, set_lighting_alpha, same, same;
        refraction_strength, set_refraction_strength: f32 =
            lighting_refraction_strength, set_lighting_refraction_strength, same, same;
        /// Specular glossiness (smoothness from Fallout 4 on).
        glossiness, set_glossiness: f32 = lighting_glossiness, set_lighting_glossiness, same, same;
        /// Specular color as `[r, g, b]`.
        specular_color, set_specular_color: [f32; 3] =
            lighting_specular_color, set_lighting_specular_color, rgb, rgb_to_ffi;
        specular_strength, set_specular_strength: f32 =
            lighting_specular_strength, set_lighting_specular_strength, same, same;

        /// Soft lighting (light wrap). Only saved in Skyrim.
        soft_lighting, set_soft_lighting: f32 =
            lighting_soft_lighting, set_lighting_soft_lighting, same, same;
        /// Rim lighting power. Only saved in Skyrim; Fallout 4 uses
        /// [`rimlight_power_2`](Self::rimlight_power_2).
        rimlight_power, set_rimlight_power: f32 =
            lighting_rimlight_power, set_lighting_rimlight_power, same, same;

        /// Subsurface scattering rolloff. Only saved in Fallout 4.
        subsurface_rolloff, set_subsurface_rolloff: f32 =
            lighting_subsurface_rolloff, set_lighting_subsurface_rolloff, same, same;
        /// Fallout 4's rim lighting power, kept apart from Skyrim's
        /// [`rimlight_power`](Self::rimlight_power). Only saved in Fallout 4. Defaults to
        /// `f32::MAX`, the only value that also saves [`backlight_power`](Self::backlight_power).
        rimlight_power_2, set_rimlight_power_2: f32 =
            lighting_rimlight_power_2, set_lighting_rimlight_power_2, same, same;
        /// Back lighting power. Only saved in Fallout 4, while
        /// [`rimlight_power_2`](Self::rimlight_power_2) is `f32::MAX`.
        backlight_power, set_backlight_power: f32 =
            lighting_backlight_power, set_lighting_backlight_power, same, same;

        /// Only saved from Fallout 4 on.
        grayscale_to_palette_scale, set_grayscale_to_palette_scale: f32 =
            lighting_grayscale_to_palette_scale, set_lighting_grayscale_to_palette_scale, same, same;
        /// Only saved from Fallout 4 on.
        fresnel_power, set_fresnel_power: f32 =
            lighting_fresnel_power, set_lighting_fresnel_power, same, same;
        /// Specular scale when wet. Only saved from Fallout 4 on.
        wetness_specular_scale, set_wetness_specular_scale: f32 =
            lighting_wetness_specular_scale, set_lighting_wetness_specular_scale, same, same;
        /// Specular power when wet. Only saved from Fallout 4 on.
        wetness_specular_power, set_wetness_specular_power: f32 =
            lighting_wetness_specular_power, set_lighting_wetness_specular_power, same, same;
        /// Minimum wetness. Only saved from Fallout 4 on.
        wetness_min_var, set_wetness_min_var: f32 =
            lighting_wetness_min_var, set_lighting_wetness_min_var, same, same;
        /// Environment map scale when wet. Only saved in Fallout 4 files with stream version 130.
        wetness_envmap_scale, set_wetness_envmap_scale: f32 =
            lighting_wetness_envmap_scale, set_lighting_wetness_envmap_scale, same, same;
        /// Fresnel power when wet. Only saved from Fallout 4 on.
        wetness_fresnel_power, set_wetness_fresnel_power: f32 =
            lighting_wetness_fresnel_power, set_lighting_wetness_fresnel_power, same, same;
        /// Metalness when wet. Only saved from Fallout 4 on.
        wetness_metalness, set_wetness_metalness: f32 =
            lighting_wetness_metalness, set_lighting_wetness_metalness, same, same;

        /// Luminance of the emittance. Only saved in Fallout 76 and Starfield.
        lum_emittance, set_lum_emittance: f32 =
            lighting_lum_emittance, set_lighting_lum_emittance, same, same;
        /// Only saved in Fallout 76 and Starfield.
        exposure_offset, set_exposure_offset: f32 =
            lighting_exposure_offset, set_lighting_exposure_offset, same, same;
        /// Only saved in Fallout 76 and Starfield.
        final_exposure_min, set_final_exposure_min: f32 =
            lighting_final_exposure_min, set_lighting_final_exposure_min, same, same;
        /// Only saved in Fallout 76 and Starfield.
        final_exposure_max, set_final_exposure_max: f32 =
            lighting_final_exposure_max, set_lighting_final_exposure_max, same, same;

        /// Uses screen-space reflections. Only saved in Fallout 4, for type
        /// [`EnvironmentMap`](LightingShaderType::EnvironmentMap).
        use_ssr, set_use_ssr: bool = lighting_use_ssr, set_lighting_use_ssr, same, same;
        /// Uses screen-space reflections when wet. Only saved in Fallout 4, for type
        /// [`EnvironmentMap`](LightingShaderType::EnvironmentMap).
        wetness_use_ssr, set_wetness_use_ssr: bool =
            lighting_wetness_use_ssr, set_lighting_wetness_use_ssr, same, same;
        /// Skin tint as `[r, g, b]`. Only saved for type
        /// [`SkinTint`](LightingShaderType::SkinTint).
        skin_tint_color, set_skin_tint_color: [f32; 3] =
            lighting_skin_tint_color, set_lighting_skin_tint_color, rgb, rgb_to_ffi;
        /// Skin tint opacity. Only saved from Fallout 4 on, for type
        /// [`SkinTint`](LightingShaderType::SkinTint).
        skin_tint_alpha, set_skin_tint_alpha: f32 =
            lighting_skin_tint_alpha, set_lighting_skin_tint_alpha, same, same;
        /// Hair tint as `[r, g, b]`. Only saved for type
        /// [`HairTint`](LightingShaderType::HairTint).
        hair_tint_color, set_hair_tint_color: [f32; 3] =
            lighting_hair_tint_color, set_lighting_hair_tint_color, rgb, rgb_to_ffi;
        /// Parallax occlusion passes. Only saved for type
        /// [`ParallaxOcclusion`](LightingShaderType::ParallaxOcclusion).
        parallax_max_passes, set_parallax_max_passes: f32 =
            lighting_max_passes, set_lighting_max_passes, same, same;
        /// Parallax occlusion height scale. Only saved for type
        /// [`ParallaxOcclusion`](LightingShaderType::ParallaxOcclusion).
        parallax_scale, set_parallax_scale: f32 = lighting_scale, set_lighting_scale, same, same;
        /// Only saved for type [`MultiLayerParallax`](LightingShaderType::MultiLayerParallax).
        parallax_inner_layer_thickness, set_parallax_inner_layer_thickness: f32 =
            lighting_parallax_inner_layer_thickness, set_lighting_parallax_inner_layer_thickness, same, same;
        /// Only saved for type [`MultiLayerParallax`](LightingShaderType::MultiLayerParallax).
        parallax_refraction_scale, set_parallax_refraction_scale: f32 =
            lighting_parallax_refraction_scale, set_lighting_parallax_refraction_scale, same, same;
        /// Inner layer texture scale as `[u, v]`. Only saved for type
        /// [`MultiLayerParallax`](LightingShaderType::MultiLayerParallax).
        parallax_inner_layer_texture_scale, set_parallax_inner_layer_texture_scale: [f32; 2] =
            lighting_parallax_inner_layer_texture_scale, set_lighting_parallax_inner_layer_texture_scale, uv, uv_to_ffi;
        /// Only saved for type [`MultiLayerParallax`](LightingShaderType::MultiLayerParallax).
        parallax_envmap_strength, set_parallax_envmap_strength: f32 =
            lighting_parallax_envmap_strength, set_lighting_parallax_envmap_strength, same, same;
        /// Snow sparkle parameters as four values. Only saved for type
        /// [`MultiIndexSnow`](LightingShaderType::MultiIndexSnow).
        sparkle_parameters, set_sparkle_parameters: [f32; 4] =
            lighting_sparkle_parameters, set_lighting_sparkle_parameters, rgba, rgba_to_ffi;
        /// Only saved for type [`Eye`](LightingShaderType::Eye).
        eye_cubemap_scale, set_eye_cubemap_scale: f32 =
            lighting_eye_cubemap_scale, set_lighting_eye_cubemap_scale, same, same;
        /// Center of the left eye's reflection. Only saved for type
        /// [`Eye`](LightingShaderType::Eye).
        eye_left_reflection_center, set_eye_left_reflection_center: Vec3 =
            lighting_eye_left_reflection_center, set_lighting_eye_left_reflection_center, Vec3::from_ffi, Vec3::to_ffi;
        /// Center of the right eye's reflection. Only saved for type
        /// [`Eye`](LightingShaderType::Eye).
        eye_right_reflection_center, set_eye_right_reflection_center: Vec3 =
            lighting_eye_right_reflection_center, set_lighting_eye_right_reflection_center, Vec3::from_ffi, Vec3::to_ffi;

        /// A value of unknown meaning stored with the wetness fields. Only saved in Fallout 4
        /// files with a stream version above 130, Fallout 76 and Starfield.
        wetness_unknown_1, set_wetness_unknown_1: f32 =
            lighting_wetness_unknown_1, set_lighting_wetness_unknown_1, same, same;
        /// A value of unknown meaning stored with the wetness fields. Only saved in Fallout 76
        /// and Starfield.
        wetness_unknown_2, set_wetness_unknown_2: f32 =
            lighting_wetness_unknown_2, set_lighting_wetness_unknown_2, same, same;
        /// A value of unknown meaning. Only saved in Starfield.
        unknown_float, set_unknown_float: f32 =
            lighting_unknown_float, set_lighting_unknown_float, same, same;
        /// A value of unknown meaning. Only saved in Starfield.
        unknown_float_1, set_unknown_float_1: f32 =
            lighting_unknown_float_1, set_lighting_unknown_float_1, same, same;
        /// A value of unknown meaning. Only saved in Starfield.
        unknown_float_2, set_unknown_float_2: f32 =
            lighting_unknown_float_2, set_lighting_unknown_float_2, same, same;
        /// A value of unknown meaning. Only saved in Starfield.
        unknown_short, set_unknown_short: u16 =
            lighting_unknown_short, set_lighting_unknown_short, same, same;
    }

    /// The wet material's path. Only saved from Fallout 4 on.
    pub fn wet_material(&self) -> &BStr {
        ffi::lighting_root_material(&self.0).as_bstr()
    }

    /// Sets the [wet material](Self::wet_material).
    pub fn set_wet_material(&mut self, path: impl AsRef<[u8]>) {
        ffi::set_lighting_root_material(self.ffi_mut(), path.as_ref());
    }

    /// The translucency settings, or `None` if translucency is off. Only saved in Fallout 76.
    pub fn translucency(&self) -> Option<Translucency> {
        ffi::lighting_translucency(&self.0).then(|| Translucency {
            subsurface_color: rgb(ffi::lighting_subsurface_color(&self.0)),
            transmissive_scale: ffi::lighting_transmissive_scale(&self.0),
            turbulence: ffi::lighting_turbulence(&self.0),
            thick_object: ffi::lighting_thick_object(&self.0),
            mix_albedo: ffi::lighting_mix_albedo(&self.0),
        })
    }

    /// Turns translucency on with these settings, or off with `None`.
    pub fn set_translucency(&mut self, translucency: Option<Translucency>) {
        ffi::set_lighting_translucency(self.ffi_mut(), translucency.is_some());
        if let Some(t) = translucency {
            ffi::set_lighting_subsurface_color(self.ffi_mut(), rgb_to_ffi(t.subsurface_color));
            ffi::set_lighting_transmissive_scale(self.ffi_mut(), t.transmissive_scale);
            ffi::set_lighting_turbulence(self.ffi_mut(), t.turbulence);
            ffi::set_lighting_thick_object(self.ffi_mut(), t.thick_object);
            ffi::set_lighting_mix_albedo(self.ffi_mut(), t.mix_albedo);
        }
    }

    /// The texture arrays, each a list of texture paths. Only saved in Fallout 76.
    pub fn texture_arrays(&self) -> Vec<Vec<&BStr>> {
        (0..ffi::lighting_texture_array_count(&self.0))
            .map(|a| {
                (0..ffi::lighting_texture_array_len(&self.0, a))
                    .map(|i| ffi::lighting_texture_array_path(&self.0, a, i).as_bstr())
                    .collect()
            })
            .collect()
    }

    /// Replaces the [texture arrays](Self::texture_arrays).
    pub fn set_texture_arrays<A, P>(&mut self, arrays: impl IntoIterator<Item = A>)
    where
        A: IntoIterator<Item = P>,
        P: AsRef<[u8]>,
    {
        ffi::clear_lighting_texture_arrays(self.ffi_mut());
        for array in arrays {
            ffi::add_lighting_texture_array(self.ffi_mut());
            for path in array {
                ffi::push_lighting_texture_array_path(self.ffi_mut(), path.as_ref());
            }
        }
    }
}

/// The shader program of a [`BsLightingShaderProperty`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum LightingShaderType {
    Default = 0,
    EnvironmentMap = 1,
    GlowMap = 2,
    /// Not in Fallout 76 and Starfield, which store the later types one lower. See
    /// [`set_shader_type`](BsLightingShaderProperty::set_shader_type).
    Parallax = 3,
    FaceTint = 4,
    SkinTint = 5,
    HairTint = 6,
    ParallaxOcclusion = 7,
    MultiTextureLandscape = 8,
    LodLandscape = 9,
    Snow = 10,
    MultiLayerParallax = 11,
    TreeAnim = 12,
    LodObjects = 13,
    MultiIndexSnow = 14,
    LodObjectsHd = 15,
    Eye = 16,
    Cloud = 17,
    LodLandscapeNoise = 18,
    MultiTextureLandscapeLodBlend = 19,
    Dismemberment = 20,
}
