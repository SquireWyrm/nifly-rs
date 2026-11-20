//! `NiProperty`, `NiShader` and the shader classes. See `nodes.rs` for the upcast/downcast
//! convention.

#[cxx::bridge]
pub mod ffi {
    /// A `Vector2`: UV offsets and scales.
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Vec2 {
        x: f32,
        y: f32,
    }

    /// A `Color4`.
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Rgba {
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    }

    /// A `TexTransform`.
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct TextureTransform {
        translation: Vec2,
        scale: Vec2,
        rotation: f32,
        transform_type: u32,
        center: Vec2,
    }

    #[namespace = "nifly"]
    unsafe extern "C++" {
        include!("Shaders.hpp");

        type NiProperty;
        type NiShader;
        type NiShadeProperty;
        type BSShaderProperty;
        type WaterShaderProperty;
        type HairShaderProperty;
        type DistantLODShaderProperty;
        type BSDistantTreeShaderProperty;
        type TallGrassShaderProperty;
        type VolumetricFogShaderProperty;
        type BSLightingShaderProperty;
        type BSEffectShaderProperty;
        type BSWaterShaderProperty;
        type BSSkyShaderProperty;
        type BSShaderLightingProperty;
        type SkyShaderProperty;
        type TileShaderProperty;
        type BSShaderNoLightingProperty;
        type BSShaderPPLightingProperty;
        type Lighting30ShaderProperty;
        type NiMaterialProperty;
        type NiAlphaProperty;
        type NiStencilProperty;
        type NiTexturingProperty;
        type NiSpecularProperty;
        type NiVertexColorProperty;
        type NiDitherProperty;
        type NiFogProperty;
        type NiWireframeProperty;
        type NiZBufferProperty;
        type NiTexture;
        type NiSourceTexture;
        type NiSourceCubeMap;
        type BSShaderTextureSet;
        type NiPalette;
        type TextureRenderData;
        type NiPixelData;
        type NiPersistentSrcTextureRendererData;
    }

    #[namespace = "nifly"]
    extern "C++" {
        include!("Objects.hpp");

        type NiObject = crate::objects::ffi::NiObject;
        type NiObjectNET = crate::objects::ffi::NiObjectNET;
    }

    extern "C++" {
        type Vec3 = crate::object_3d::ffi::Vec3;
    }

    #[namespace = "nifly_rs"]
    unsafe extern "C++" {
        include!("shaders_shim.h");

        fn upcast_ni_property(obj: &NiProperty) -> &NiObjectNET;
        fn downcast_ni_property(obj: &NiObject) -> *const NiProperty;
        fn upcast_ni_shader(obj: &NiShader) -> &NiProperty;
        fn downcast_ni_shader(obj: &NiObject) -> *const NiShader;
        fn upcast_ni_shade_property(obj: &NiShadeProperty) -> &NiShader;
        fn downcast_ni_shade_property(obj: &NiObject) -> *const NiShadeProperty;
        fn upcast_bs_shader_property(obj: &BSShaderProperty) -> &NiShadeProperty;
        fn downcast_bs_shader_property(obj: &NiObject) -> *const BSShaderProperty;
        fn upcast_water_shader_property(obj: &WaterShaderProperty) -> &BSShaderProperty;
        fn downcast_water_shader_property(obj: &NiObject) -> *const WaterShaderProperty;
        fn upcast_hair_shader_property(obj: &HairShaderProperty) -> &BSShaderProperty;
        fn downcast_hair_shader_property(obj: &NiObject) -> *const HairShaderProperty;
        fn upcast_distant_lod_shader_property(obj: &DistantLODShaderProperty) -> &BSShaderProperty;
        fn downcast_distant_lod_shader_property(obj: &NiObject) -> *const DistantLODShaderProperty;
        fn upcast_bs_distant_tree_shader_property(
            obj: &BSDistantTreeShaderProperty,
        ) -> &BSShaderProperty;
        fn downcast_bs_distant_tree_shader_property(
            obj: &NiObject,
        ) -> *const BSDistantTreeShaderProperty;
        fn upcast_tall_grass_shader_property(obj: &TallGrassShaderProperty) -> &BSShaderProperty;
        fn downcast_tall_grass_shader_property(obj: &NiObject) -> *const TallGrassShaderProperty;
        fn upcast_volumetric_fog_shader_property(
            obj: &VolumetricFogShaderProperty,
        ) -> &BSShaderProperty;
        fn downcast_volumetric_fog_shader_property(
            obj: &NiObject,
        ) -> *const VolumetricFogShaderProperty;
        fn upcast_bs_lighting_shader_property(obj: &BSLightingShaderProperty) -> &BSShaderProperty;
        fn downcast_bs_lighting_shader_property(obj: &NiObject) -> *const BSLightingShaderProperty;
        fn upcast_bs_effect_shader_property(obj: &BSEffectShaderProperty) -> &BSShaderProperty;
        fn downcast_bs_effect_shader_property(obj: &NiObject) -> *const BSEffectShaderProperty;
        fn upcast_bs_water_shader_property(obj: &BSWaterShaderProperty) -> &BSShaderProperty;
        fn downcast_bs_water_shader_property(obj: &NiObject) -> *const BSWaterShaderProperty;
        fn upcast_bs_sky_shader_property(obj: &BSSkyShaderProperty) -> &BSShaderProperty;
        fn downcast_bs_sky_shader_property(obj: &NiObject) -> *const BSSkyShaderProperty;
        fn upcast_bs_shader_lighting_property(obj: &BSShaderLightingProperty) -> &BSShaderProperty;
        fn downcast_bs_shader_lighting_property(obj: &NiObject) -> *const BSShaderLightingProperty;
        fn upcast_sky_shader_property(obj: &SkyShaderProperty) -> &BSShaderLightingProperty;
        fn downcast_sky_shader_property(obj: &NiObject) -> *const SkyShaderProperty;
        fn upcast_tile_shader_property(obj: &TileShaderProperty) -> &BSShaderLightingProperty;
        fn downcast_tile_shader_property(obj: &NiObject) -> *const TileShaderProperty;
        fn upcast_bs_shader_no_lighting_property(
            obj: &BSShaderNoLightingProperty,
        ) -> &BSShaderLightingProperty;
        fn downcast_bs_shader_no_lighting_property(
            obj: &NiObject,
        ) -> *const BSShaderNoLightingProperty;
        fn upcast_bs_shader_pp_lighting_property(
            obj: &BSShaderPPLightingProperty,
        ) -> &BSShaderLightingProperty;
        fn downcast_bs_shader_pp_lighting_property(
            obj: &NiObject,
        ) -> *const BSShaderPPLightingProperty;
        fn upcast_lighting30_shader_property(
            obj: &Lighting30ShaderProperty,
        ) -> &BSShaderPPLightingProperty;
        fn downcast_lighting30_shader_property(obj: &NiObject) -> *const Lighting30ShaderProperty;
        fn upcast_ni_material_property(obj: &NiMaterialProperty) -> &NiShader;
        fn downcast_ni_material_property(obj: &NiObject) -> *const NiMaterialProperty;
        fn upcast_ni_alpha_property(obj: &NiAlphaProperty) -> &NiProperty;
        fn downcast_ni_alpha_property(obj: &NiObject) -> *const NiAlphaProperty;
        fn upcast_ni_stencil_property(obj: &NiStencilProperty) -> &NiProperty;
        fn downcast_ni_stencil_property(obj: &NiObject) -> *const NiStencilProperty;
        fn upcast_ni_texturing_property(obj: &NiTexturingProperty) -> &NiProperty;
        fn downcast_ni_texturing_property(obj: &NiObject) -> *const NiTexturingProperty;
        fn upcast_ni_specular_property(obj: &NiSpecularProperty) -> &NiProperty;
        fn downcast_ni_specular_property(obj: &NiObject) -> *const NiSpecularProperty;
        fn upcast_ni_vertex_color_property(obj: &NiVertexColorProperty) -> &NiProperty;
        fn downcast_ni_vertex_color_property(obj: &NiObject) -> *const NiVertexColorProperty;
        fn upcast_ni_dither_property(obj: &NiDitherProperty) -> &NiProperty;
        fn downcast_ni_dither_property(obj: &NiObject) -> *const NiDitherProperty;
        fn upcast_ni_fog_property(obj: &NiFogProperty) -> &NiProperty;
        fn downcast_ni_fog_property(obj: &NiObject) -> *const NiFogProperty;
        fn upcast_ni_wireframe_property(obj: &NiWireframeProperty) -> &NiProperty;
        fn downcast_ni_wireframe_property(obj: &NiObject) -> *const NiWireframeProperty;
        fn upcast_ni_z_buffer_property(obj: &NiZBufferProperty) -> &NiProperty;
        fn downcast_ni_z_buffer_property(obj: &NiObject) -> *const NiZBufferProperty;
        fn upcast_ni_texture(obj: &NiTexture) -> &NiObjectNET;
        fn downcast_ni_texture(obj: &NiObject) -> *const NiTexture;
        fn upcast_ni_source_texture(obj: &NiSourceTexture) -> &NiTexture;
        fn downcast_ni_source_texture(obj: &NiObject) -> *const NiSourceTexture;
        fn upcast_ni_source_cube_map(obj: &NiSourceCubeMap) -> &NiSourceTexture;
        fn downcast_ni_source_cube_map(obj: &NiObject) -> *const NiSourceCubeMap;
        fn upcast_bs_shader_texture_set(obj: &BSShaderTextureSet) -> &NiObject;
        fn downcast_bs_shader_texture_set(obj: &NiObject) -> *const BSShaderTextureSet;

        /// `BSShaderProperty::shaderFlags1` and `shaderFlags2`.
        fn shader_flags_1(shader: &BSShaderProperty) -> u32;
        fn shader_flags_2(shader: &BSShaderProperty) -> u32;
        fn set_shader_flags_1(shader: Pin<&mut BSShaderProperty>, flags: u32);
        fn set_shader_flags_2(shader: Pin<&mut BSShaderProperty>, flags: u32);
        /// `BSLightingShaderProperty::GetShaderType`.
        fn lighting_shader_type(shader: &BSLightingShaderProperty) -> u32;
        /// `BSShaderProperty` fields.
        fn shader_uv_offset(s: &BSShaderProperty) -> Vec2;
        fn set_shader_uv_offset(s: Pin<&mut BSShaderProperty>, value: Vec2);
        fn shader_uv_scale(s: &BSShaderProperty) -> Vec2;
        fn set_shader_uv_scale(s: Pin<&mut BSShaderProperty>, value: Vec2);
        fn shader_environment_map_scale(s: &BSShaderProperty) -> f32;
        fn set_shader_environment_map_scale(s: Pin<&mut BSShaderProperty>, value: f32);
        /// `BSLightingShaderProperty::SetShaderType`, in the internal numbering (Parallax is 3).
        fn set_lighting_shader_type(shader: Pin<&mut BSLightingShaderProperty>, shader_type: u32);
        /// `BSLightingShaderProperty` fields. Colors are r, g, b in x, y, z; the field
        /// names are in `shaders_shim_bethesda.h`. Not bound: the virtual getters, which read the
        /// same fields.
        fn lighting_texture_clamp_mode(s: &BSLightingShaderProperty) -> u32;
        fn set_lighting_texture_clamp_mode(s: Pin<&mut BSLightingShaderProperty>, value: u32);
        fn lighting_emissive_multiple(s: &BSLightingShaderProperty) -> f32;
        fn set_lighting_emissive_multiple(s: Pin<&mut BSLightingShaderProperty>, value: f32);
        fn lighting_alpha(s: &BSLightingShaderProperty) -> f32;
        fn set_lighting_alpha(s: Pin<&mut BSLightingShaderProperty>, value: f32);
        fn lighting_refraction_strength(s: &BSLightingShaderProperty) -> f32;
        fn set_lighting_refraction_strength(s: Pin<&mut BSLightingShaderProperty>, value: f32);
        fn lighting_glossiness(s: &BSLightingShaderProperty) -> f32;
        fn set_lighting_glossiness(s: Pin<&mut BSLightingShaderProperty>, value: f32);
        fn lighting_specular_strength(s: &BSLightingShaderProperty) -> f32;
        fn set_lighting_specular_strength(s: Pin<&mut BSLightingShaderProperty>, value: f32);
        fn lighting_soft_lighting(s: &BSLightingShaderProperty) -> f32;
        fn set_lighting_soft_lighting(s: Pin<&mut BSLightingShaderProperty>, value: f32);
        fn lighting_rimlight_power(s: &BSLightingShaderProperty) -> f32;
        fn set_lighting_rimlight_power(s: Pin<&mut BSLightingShaderProperty>, value: f32);
        fn lighting_subsurface_rolloff(s: &BSLightingShaderProperty) -> f32;
        fn set_lighting_subsurface_rolloff(s: Pin<&mut BSLightingShaderProperty>, value: f32);
        fn lighting_rimlight_power_2(s: &BSLightingShaderProperty) -> f32;
        fn set_lighting_rimlight_power_2(s: Pin<&mut BSLightingShaderProperty>, value: f32);
        fn lighting_backlight_power(s: &BSLightingShaderProperty) -> f32;
        fn set_lighting_backlight_power(s: Pin<&mut BSLightingShaderProperty>, value: f32);
        fn lighting_grayscale_to_palette_scale(s: &BSLightingShaderProperty) -> f32;
        fn set_lighting_grayscale_to_palette_scale(
            s: Pin<&mut BSLightingShaderProperty>,
            value: f32,
        );
        fn lighting_fresnel_power(s: &BSLightingShaderProperty) -> f32;
        fn set_lighting_fresnel_power(s: Pin<&mut BSLightingShaderProperty>, value: f32);
        fn lighting_wetness_specular_scale(s: &BSLightingShaderProperty) -> f32;
        fn set_lighting_wetness_specular_scale(s: Pin<&mut BSLightingShaderProperty>, value: f32);
        fn lighting_wetness_specular_power(s: &BSLightingShaderProperty) -> f32;
        fn set_lighting_wetness_specular_power(s: Pin<&mut BSLightingShaderProperty>, value: f32);
        fn lighting_wetness_min_var(s: &BSLightingShaderProperty) -> f32;
        fn set_lighting_wetness_min_var(s: Pin<&mut BSLightingShaderProperty>, value: f32);
        fn lighting_wetness_envmap_scale(s: &BSLightingShaderProperty) -> f32;
        fn set_lighting_wetness_envmap_scale(s: Pin<&mut BSLightingShaderProperty>, value: f32);
        fn lighting_wetness_fresnel_power(s: &BSLightingShaderProperty) -> f32;
        fn set_lighting_wetness_fresnel_power(s: Pin<&mut BSLightingShaderProperty>, value: f32);
        fn lighting_wetness_metalness(s: &BSLightingShaderProperty) -> f32;
        fn set_lighting_wetness_metalness(s: Pin<&mut BSLightingShaderProperty>, value: f32);
        fn lighting_lum_emittance(s: &BSLightingShaderProperty) -> f32;
        fn set_lighting_lum_emittance(s: Pin<&mut BSLightingShaderProperty>, value: f32);
        fn lighting_exposure_offset(s: &BSLightingShaderProperty) -> f32;
        fn set_lighting_exposure_offset(s: Pin<&mut BSLightingShaderProperty>, value: f32);
        fn lighting_final_exposure_min(s: &BSLightingShaderProperty) -> f32;
        fn set_lighting_final_exposure_min(s: Pin<&mut BSLightingShaderProperty>, value: f32);
        fn lighting_final_exposure_max(s: &BSLightingShaderProperty) -> f32;
        fn set_lighting_final_exposure_max(s: Pin<&mut BSLightingShaderProperty>, value: f32);
        fn lighting_transmissive_scale(s: &BSLightingShaderProperty) -> f32;
        fn set_lighting_transmissive_scale(s: Pin<&mut BSLightingShaderProperty>, value: f32);
        fn lighting_turbulence(s: &BSLightingShaderProperty) -> f32;
        fn set_lighting_turbulence(s: Pin<&mut BSLightingShaderProperty>, value: f32);
        fn lighting_skin_tint_alpha(s: &BSLightingShaderProperty) -> f32;
        fn set_lighting_skin_tint_alpha(s: Pin<&mut BSLightingShaderProperty>, value: f32);
        fn lighting_max_passes(s: &BSLightingShaderProperty) -> f32;
        fn set_lighting_max_passes(s: Pin<&mut BSLightingShaderProperty>, value: f32);
        fn lighting_scale(s: &BSLightingShaderProperty) -> f32;
        fn set_lighting_scale(s: Pin<&mut BSLightingShaderProperty>, value: f32);
        fn lighting_parallax_inner_layer_thickness(s: &BSLightingShaderProperty) -> f32;
        fn set_lighting_parallax_inner_layer_thickness(
            s: Pin<&mut BSLightingShaderProperty>,
            value: f32,
        );
        fn lighting_parallax_refraction_scale(s: &BSLightingShaderProperty) -> f32;
        fn set_lighting_parallax_refraction_scale(
            s: Pin<&mut BSLightingShaderProperty>,
            value: f32,
        );
        fn lighting_parallax_envmap_strength(s: &BSLightingShaderProperty) -> f32;
        fn set_lighting_parallax_envmap_strength(s: Pin<&mut BSLightingShaderProperty>, value: f32);
        fn lighting_eye_cubemap_scale(s: &BSLightingShaderProperty) -> f32;
        fn set_lighting_eye_cubemap_scale(s: Pin<&mut BSLightingShaderProperty>, value: f32);
        fn lighting_translucency(s: &BSLightingShaderProperty) -> bool;
        fn set_lighting_translucency(s: Pin<&mut BSLightingShaderProperty>, value: bool);
        fn lighting_thick_object(s: &BSLightingShaderProperty) -> bool;
        fn set_lighting_thick_object(s: Pin<&mut BSLightingShaderProperty>, value: bool);
        fn lighting_mix_albedo(s: &BSLightingShaderProperty) -> bool;
        fn set_lighting_mix_albedo(s: Pin<&mut BSLightingShaderProperty>, value: bool);
        fn lighting_use_ssr(s: &BSLightingShaderProperty) -> bool;
        fn set_lighting_use_ssr(s: Pin<&mut BSLightingShaderProperty>, value: bool);
        fn lighting_wetness_use_ssr(s: &BSLightingShaderProperty) -> bool;
        fn set_lighting_wetness_use_ssr(s: Pin<&mut BSLightingShaderProperty>, value: bool);
        fn lighting_emissive_color(s: &BSLightingShaderProperty) -> Vec3;
        fn set_lighting_emissive_color(s: Pin<&mut BSLightingShaderProperty>, value: Vec3);
        fn lighting_specular_color(s: &BSLightingShaderProperty) -> Vec3;
        fn set_lighting_specular_color(s: Pin<&mut BSLightingShaderProperty>, value: Vec3);
        fn lighting_subsurface_color(s: &BSLightingShaderProperty) -> Vec3;
        fn set_lighting_subsurface_color(s: Pin<&mut BSLightingShaderProperty>, value: Vec3);
        fn lighting_skin_tint_color(s: &BSLightingShaderProperty) -> Vec3;
        fn set_lighting_skin_tint_color(s: Pin<&mut BSLightingShaderProperty>, value: Vec3);
        fn lighting_hair_tint_color(s: &BSLightingShaderProperty) -> Vec3;
        fn set_lighting_hair_tint_color(s: Pin<&mut BSLightingShaderProperty>, value: Vec3);
        fn lighting_eye_left_reflection_center(s: &BSLightingShaderProperty) -> Vec3;
        fn set_lighting_eye_left_reflection_center(
            s: Pin<&mut BSLightingShaderProperty>,
            value: Vec3,
        );
        fn lighting_eye_right_reflection_center(s: &BSLightingShaderProperty) -> Vec3;
        fn set_lighting_eye_right_reflection_center(
            s: Pin<&mut BSLightingShaderProperty>,
            value: Vec3,
        );
        fn lighting_parallax_inner_layer_texture_scale(s: &BSLightingShaderProperty) -> Vec2;
        fn set_lighting_parallax_inner_layer_texture_scale(
            s: Pin<&mut BSLightingShaderProperty>,
            value: Vec2,
        );
        fn lighting_sparkle_parameters(s: &BSLightingShaderProperty) -> Rgba;
        fn set_lighting_sparkle_parameters(s: Pin<&mut BSLightingShaderProperty>, value: Rgba);
        /// `rootMaterialName`, the wet material.
        fn lighting_root_material(s: &BSLightingShaderProperty) -> &[u8];
        fn set_lighting_root_material(s: Pin<&mut BSLightingShaderProperty>, name: &[u8]);
        /// `textureArrays`, keeping `hasTextureArrays` and `numTextureArrays` in step.
        fn lighting_texture_array_count(s: &BSLightingShaderProperty) -> usize;
        fn lighting_texture_array_len(s: &BSLightingShaderProperty, array: usize) -> usize;
        fn lighting_texture_array_path(
            s: &BSLightingShaderProperty,
            array: usize,
            index: usize,
        ) -> &[u8];
        fn clear_lighting_texture_arrays(s: Pin<&mut BSLightingShaderProperty>);
        fn add_lighting_texture_array(s: Pin<&mut BSLightingShaderProperty>);
        fn push_lighting_texture_array_path(s: Pin<&mut BSLightingShaderProperty>, path: &[u8]);

        /// `BSEffectShaderProperty` fields; the field names are in `shaders_shim_bethesda.h`. Not bound:
        /// the fields of unknown meaning (`unkFloat`, `unkBytes`, `unkFloats`, `unkByte1`).
        /// `textureClampMode` is passed whole: four bytes, split in the safe crate.
        fn effect_texture_clamp_bytes(s: &BSEffectShaderProperty) -> u32;
        fn set_effect_texture_clamp_bytes(s: Pin<&mut BSEffectShaderProperty>, value: u32);
        fn effect_falloff_start_angle(s: &BSEffectShaderProperty) -> f32;
        fn set_effect_falloff_start_angle(s: Pin<&mut BSEffectShaderProperty>, value: f32);
        fn effect_falloff_stop_angle(s: &BSEffectShaderProperty) -> f32;
        fn set_effect_falloff_stop_angle(s: Pin<&mut BSEffectShaderProperty>, value: f32);
        fn effect_falloff_start_opacity(s: &BSEffectShaderProperty) -> f32;
        fn set_effect_falloff_start_opacity(s: Pin<&mut BSEffectShaderProperty>, value: f32);
        fn effect_falloff_stop_opacity(s: &BSEffectShaderProperty) -> f32;
        fn set_effect_falloff_stop_opacity(s: Pin<&mut BSEffectShaderProperty>, value: f32);
        fn effect_refraction_power(s: &BSEffectShaderProperty) -> f32;
        fn set_effect_refraction_power(s: Pin<&mut BSEffectShaderProperty>, value: f32);
        fn effect_base_color_scale(s: &BSEffectShaderProperty) -> f32;
        fn set_effect_base_color_scale(s: Pin<&mut BSEffectShaderProperty>, value: f32);
        fn effect_soft_falloff_depth(s: &BSEffectShaderProperty) -> f32;
        fn set_effect_soft_falloff_depth(s: Pin<&mut BSEffectShaderProperty>, value: f32);
        fn effect_env_map_scale(s: &BSEffectShaderProperty) -> f32;
        fn set_effect_env_map_scale(s: Pin<&mut BSEffectShaderProperty>, value: f32);
        fn effect_lum_emittance(s: &BSEffectShaderProperty) -> f32;
        fn set_effect_lum_emittance(s: Pin<&mut BSEffectShaderProperty>, value: f32);
        fn effect_exposure_offset(s: &BSEffectShaderProperty) -> f32;
        fn set_effect_exposure_offset(s: Pin<&mut BSEffectShaderProperty>, value: f32);
        fn effect_final_exposure_min(s: &BSEffectShaderProperty) -> f32;
        fn set_effect_final_exposure_min(s: Pin<&mut BSEffectShaderProperty>, value: f32);
        fn effect_final_exposure_max(s: &BSEffectShaderProperty) -> f32;
        fn set_effect_final_exposure_max(s: Pin<&mut BSEffectShaderProperty>, value: f32);
        fn effect_base_color(s: &BSEffectShaderProperty) -> Rgba;
        fn set_effect_base_color(s: Pin<&mut BSEffectShaderProperty>, value: Rgba);
        fn effect_emittance_color(s: &BSEffectShaderProperty) -> Vec3;
        fn set_effect_emittance_color(s: Pin<&mut BSEffectShaderProperty>, value: Vec3);
        fn effect_source_texture(s: &BSEffectShaderProperty) -> &[u8];
        fn set_effect_source_texture(s: Pin<&mut BSEffectShaderProperty>, path: &[u8]);
        fn effect_greyscale_texture(s: &BSEffectShaderProperty) -> &[u8];
        fn set_effect_greyscale_texture(s: Pin<&mut BSEffectShaderProperty>, path: &[u8]);
        fn effect_env_map_texture(s: &BSEffectShaderProperty) -> &[u8];
        fn set_effect_env_map_texture(s: Pin<&mut BSEffectShaderProperty>, path: &[u8]);
        fn effect_normal_texture(s: &BSEffectShaderProperty) -> &[u8];
        fn set_effect_normal_texture(s: Pin<&mut BSEffectShaderProperty>, path: &[u8]);
        fn effect_env_mask_texture(s: &BSEffectShaderProperty) -> &[u8];
        fn set_effect_env_mask_texture(s: Pin<&mut BSEffectShaderProperty>, path: &[u8]);
        fn effect_reflectance_texture(s: &BSEffectShaderProperty) -> &[u8];
        fn set_effect_reflectance_texture(s: Pin<&mut BSEffectShaderProperty>, path: &[u8]);
        fn effect_lighting_texture(s: &BSEffectShaderProperty) -> &[u8];
        fn set_effect_lighting_texture(s: Pin<&mut BSEffectShaderProperty>, path: &[u8]);
        fn effect_emit_gradient_texture(s: &BSEffectShaderProperty) -> &[u8];
        fn set_effect_emit_gradient_texture(s: Pin<&mut BSEffectShaderProperty>, path: &[u8]);

        /// `NiSourceTexture` fields; the enums (`PixelLayout`, `MipMapFormat`, `AlphaFormat`)
        /// are raw values.
        fn source_use_external(s: &NiSourceTexture) -> bool;
        fn set_source_use_external(s: Pin<&mut NiSourceTexture>, value: bool);
        fn source_use_internal(s: &NiSourceTexture) -> bool;
        fn set_source_use_internal(s: Pin<&mut NiSourceTexture>, value: bool);
        fn source_is_static(s: &NiSourceTexture) -> bool;
        fn set_source_is_static(s: Pin<&mut NiSourceTexture>, value: bool);
        fn source_direct_render(s: &NiSourceTexture) -> bool;
        fn set_source_direct_render(s: Pin<&mut NiSourceTexture>, value: bool);
        fn source_persistent_render_data(s: &NiSourceTexture) -> bool;
        fn set_source_persistent_render_data(s: Pin<&mut NiSourceTexture>, value: bool);
        fn source_pixel_layout(s: &NiSourceTexture) -> u32;
        fn set_source_pixel_layout(s: Pin<&mut NiSourceTexture>, value: u32);
        fn source_mip_map_format(s: &NiSourceTexture) -> u32;
        fn set_source_mip_map_format(s: Pin<&mut NiSourceTexture>, value: u32);
        fn source_alpha_format(s: &NiSourceTexture) -> u32;
        fn set_source_alpha_format(s: Pin<&mut NiSourceTexture>, value: u32);
        fn source_file_name(s: &NiSourceTexture) -> &[u8];
        fn set_source_file_name(s: Pin<&mut NiSourceTexture>, path: &[u8]);
        /// `dataRef`'s block index; `NIF_NPOS` if empty.
        fn source_pixel_data(s: &NiSourceTexture) -> u32;

        /// `NiMaterialProperty` fields. Colors are r, g, b in x, y, z.
        fn material_ambient(m: &NiMaterialProperty) -> Vec3;
        fn material_diffuse(m: &NiMaterialProperty) -> Vec3;
        fn material_specular(m: &NiMaterialProperty) -> Vec3;
        fn material_emissive(m: &NiMaterialProperty) -> Vec3;
        fn material_glossiness(m: &NiMaterialProperty) -> f32;
        fn material_alpha(m: &NiMaterialProperty) -> f32;
        fn material_emissive_multiple(m: &NiMaterialProperty) -> f32;
        /// `NiMaterialProperty::IsEmissive` and `HasSpecular`: the color isn't exactly zero.
        fn material_is_emissive(m: &NiMaterialProperty) -> bool;
        fn material_has_specular(m: &NiMaterialProperty) -> bool;
        fn set_material_ambient(m: Pin<&mut NiMaterialProperty>, c: Vec3);
        fn set_material_diffuse(m: Pin<&mut NiMaterialProperty>, c: Vec3);
        fn set_material_specular(m: Pin<&mut NiMaterialProperty>, c: Vec3);
        fn set_material_emissive(m: Pin<&mut NiMaterialProperty>, c: Vec3);
        fn set_material_glossiness(m: Pin<&mut NiMaterialProperty>, v: f32);
        fn set_material_alpha(m: Pin<&mut NiMaterialProperty>, v: f32);
        fn set_material_emissive_multiple(m: Pin<&mut NiMaterialProperty>, v: f32);

        /// `NiAlphaProperty::flags` and `threshold`.
        fn alpha_flags(a: &NiAlphaProperty) -> u16;
        fn alpha_threshold(a: &NiAlphaProperty) -> u8;
        fn set_alpha_flags(a: Pin<&mut NiAlphaProperty>, flags: u16);
        fn set_alpha_threshold(a: Pin<&mut NiAlphaProperty>, threshold: u8);

        /// `NiTexturingProperty::applyMode`.
        fn texturing_apply_mode(t: &NiTexturingProperty) -> u32;
        fn set_texturing_apply_mode(t: Pin<&mut NiTexturingProperty>, value: u32);
        /// `NiTexturingProperty::textureCount`.
        fn texturing_texture_count(t: &NiTexturingProperty) -> u32;
        /// Whether map `map` (base, dark, detail, gloss, glow, bump, normal, parallax, decals
        /// 0-3, then shader maps from 12) is present. False if `map` is out of range.
        fn texturing_has_map(t: &NiTexturingProperty, map: u32) -> bool;
        /// The `NiSourceTexture` block index of map `map`; `NIF_NPOS` if it's absent or empty.
        fn texturing_map_source(t: &NiTexturingProperty, map: u32) -> u32;
        /// `TexDesc` fields of map `map`. The getters return false if the map is absent,
        /// leaving `out` unchanged; the setters return false, changing nothing.
        fn texturing_map_clamp_mode(t: &NiTexturingProperty, map: u32, out: &mut u32) -> bool;
        fn set_texturing_map_clamp_mode(
            t: Pin<&mut NiTexturingProperty>,
            map: u32,
            value: u32,
        ) -> bool;
        fn texturing_map_filter_mode(t: &NiTexturingProperty, map: u32, out: &mut u32) -> bool;
        fn set_texturing_map_filter_mode(
            t: Pin<&mut NiTexturingProperty>,
            map: u32,
            value: u32,
        ) -> bool;
        fn texturing_map_uv_set(t: &NiTexturingProperty, map: u32, out: &mut u32) -> bool;
        fn set_texturing_map_uv_set(t: Pin<&mut NiTexturingProperty>, map: u32, value: u32)
        -> bool;
        fn texturing_map_flags(t: &NiTexturingProperty, map: u32, out: &mut u32) -> bool;
        fn set_texturing_map_flags(t: Pin<&mut NiTexturingProperty>, map: u32, value: u32) -> bool;
        fn texturing_map_max_anisotropy(t: &NiTexturingProperty, map: u32, out: &mut u32) -> bool;
        fn set_texturing_map_max_anisotropy(
            t: Pin<&mut NiTexturingProperty>,
            map: u32,
            value: u32,
        ) -> bool;
        fn texturing_map_unknown(t: &NiTexturingProperty, map: u32, out: &mut u32) -> bool;
        fn set_texturing_map_unknown(
            t: Pin<&mut NiTexturingProperty>,
            map: u32,
            value: u32,
        ) -> bool;
        /// `ps2_l` and `ps2_k`, `int16_t`s passed sign-extended.
        fn texturing_map_ps2_l(t: &NiTexturingProperty, map: u32, out: &mut u32) -> bool;
        fn set_texturing_map_ps2_l(t: Pin<&mut NiTexturingProperty>, map: u32, value: u32) -> bool;
        fn texturing_map_ps2_k(t: &NiTexturingProperty, map: u32, out: &mut u32) -> bool;
        fn set_texturing_map_ps2_k(t: Pin<&mut NiTexturingProperty>, map: u32, value: u32) -> bool;
        /// True if the map exists and has a transform.
        fn texturing_map_transform(
            t: &NiTexturingProperty,
            map: u32,
            out: &mut TextureTransform,
        ) -> bool;
        /// False if the map is absent.
        fn set_texturing_map_transform(
            t: Pin<&mut NiTexturingProperty>,
            map: u32,
            has: bool,
            value: &TextureTransform,
        ) -> bool;
        fn texturing_flags(t: &NiTexturingProperty) -> u16;
        fn set_texturing_flags(t: Pin<&mut NiTexturingProperty>, value: u16);
        fn set_texturing_texture_count(t: Pin<&mut NiTexturingProperty>, value: u32);
        fn texturing_luma_scale(t: &NiTexturingProperty) -> f32;
        fn set_texturing_luma_scale(t: Pin<&mut NiTexturingProperty>, value: f32);
        fn texturing_luma_offset(t: &NiTexturingProperty) -> f32;
        fn set_texturing_luma_offset(t: Pin<&mut NiTexturingProperty>, value: f32);
        fn texturing_bump_matrix(t: &NiTexturingProperty) -> [f32; 4];
        fn set_texturing_bump_matrix(t: Pin<&mut NiTexturingProperty>, value: [f32; 4]);
        fn texturing_parallax_offset(t: &NiTexturingProperty) -> f32;
        fn set_texturing_parallax_offset(t: Pin<&mut NiTexturingProperty>, value: f32);
        /// `shaderTex`, whose maps are map numbers 12 and up.
        fn texturing_shader_map_count(t: &NiTexturingProperty) -> usize;
        fn set_texturing_shader_map_count(t: Pin<&mut NiTexturingProperty>, count: usize);
        /// A used shader map's `mapIndex`. False if it's unused or out of range.
        fn texturing_shader_map_id(t: &NiTexturingProperty, i: usize, out: &mut u32) -> bool;
        fn set_texturing_shader_map_id(t: Pin<&mut NiTexturingProperty>, i: usize, id: u32)
        -> bool;

        /// `NiStencilProperty` fields: `flags`, `stencilEnabled`, `stencilFunction`,
        /// `stencilRef`, `stencilMask`, `failAction`, `zFailAction`, `passAction`, `drawMode`.
        fn stencil_flags(s: &NiStencilProperty) -> u16;
        fn set_stencil_flags(s: Pin<&mut NiStencilProperty>, value: u16);
        fn stencil_enabled(s: &NiStencilProperty) -> bool;
        fn set_stencil_enabled(s: Pin<&mut NiStencilProperty>, value: bool);
        fn stencil_function(s: &NiStencilProperty) -> u32;
        fn set_stencil_function(s: Pin<&mut NiStencilProperty>, value: u32);
        fn stencil_reference(s: &NiStencilProperty) -> u32;
        fn set_stencil_reference(s: Pin<&mut NiStencilProperty>, value: u32);
        fn stencil_mask(s: &NiStencilProperty) -> u32;
        fn set_stencil_mask(s: Pin<&mut NiStencilProperty>, value: u32);
        fn stencil_fail_action(s: &NiStencilProperty) -> u32;
        fn set_stencil_fail_action(s: Pin<&mut NiStencilProperty>, value: u32);
        fn stencil_z_fail_action(s: &NiStencilProperty) -> u32;
        fn set_stencil_z_fail_action(s: Pin<&mut NiStencilProperty>, value: u32);
        fn stencil_pass_action(s: &NiStencilProperty) -> u32;
        fn set_stencil_pass_action(s: Pin<&mut NiStencilProperty>, value: u32);
        fn stencil_draw_mode(s: &NiStencilProperty) -> u32;
        fn set_stencil_draw_mode(s: Pin<&mut NiStencilProperty>, value: u32);

        /// Fixed-function property fields: `flags` on each, `NiFogProperty::fogDepth` and
        /// `fogColor`, `NiVertexColorProperty::vertexMode` and `lightingMode`,
        /// `NiZBufferProperty::testFunction`.
        fn specular_flags(p: &NiSpecularProperty) -> u16;
        fn set_specular_flags(p: Pin<&mut NiSpecularProperty>, value: u16);
        fn dither_flags(p: &NiDitherProperty) -> u16;
        fn set_dither_flags(p: Pin<&mut NiDitherProperty>, value: u16);
        fn wireframe_flags(p: &NiWireframeProperty) -> u16;
        fn set_wireframe_flags(p: Pin<&mut NiWireframeProperty>, value: u16);
        fn fog_flags(p: &NiFogProperty) -> u16;
        fn set_fog_flags(p: Pin<&mut NiFogProperty>, value: u16);
        fn fog_depth(p: &NiFogProperty) -> f32;
        fn set_fog_depth(p: Pin<&mut NiFogProperty>, value: f32);
        fn fog_color(p: &NiFogProperty) -> Vec3;
        fn set_fog_color(p: Pin<&mut NiFogProperty>, value: Vec3);
        fn vertex_color_flags(p: &NiVertexColorProperty) -> u16;
        fn set_vertex_color_flags(p: Pin<&mut NiVertexColorProperty>, value: u16);
        fn vertex_color_vertex_mode(p: &NiVertexColorProperty) -> u32;
        fn set_vertex_color_vertex_mode(p: Pin<&mut NiVertexColorProperty>, value: u32);
        fn vertex_color_lighting_mode(p: &NiVertexColorProperty) -> u32;
        fn set_vertex_color_lighting_mode(p: Pin<&mut NiVertexColorProperty>, value: u32);
        fn z_buffer_flags(p: &NiZBufferProperty) -> u16;
        fn set_z_buffer_flags(p: Pin<&mut NiZBufferProperty>, value: u16);
        fn z_buffer_function(p: &NiZBufferProperty) -> u32;
        fn set_z_buffer_function(p: Pin<&mut NiZBufferProperty>, value: u32);

        fn upcast_ni_palette(obj: &NiPalette) -> &NiObject;
        fn downcast_ni_palette(obj: &NiObject) -> *const NiPalette;
        fn upcast_texture_render_data(obj: &TextureRenderData) -> &NiObject;
        fn downcast_texture_render_data(obj: &NiObject) -> *const TextureRenderData;
        fn upcast_ni_pixel_data(obj: &NiPixelData) -> &TextureRenderData;
        fn downcast_ni_pixel_data(obj: &NiObject) -> *const NiPixelData;
        fn upcast_ni_persistent_src_texture_renderer_data(
            obj: &NiPersistentSrcTextureRendererData,
        ) -> &TextureRenderData;
        fn downcast_ni_persistent_src_texture_renderer_data(
            obj: &NiObject,
        ) -> *const NiPersistentSrcTextureRendererData;

        /// `TextureRenderData` fields; enums are passed as `u32`.
        fn render_data_pixel_format(d: &TextureRenderData) -> u32;
        fn set_render_data_pixel_format(d: Pin<&mut TextureRenderData>, v: u32);
        fn render_data_bits_per_pixel(d: &TextureRenderData) -> u8;
        fn set_render_data_bits_per_pixel(d: Pin<&mut TextureRenderData>, v: u8);
        fn render_data_bytes_per_pixel(d: &TextureRenderData) -> u32;
        fn set_render_data_bytes_per_pixel(d: Pin<&mut TextureRenderData>, v: u32);
        fn render_data_renderer_hint(d: &TextureRenderData) -> u32;
        fn set_render_data_renderer_hint(d: Pin<&mut TextureRenderData>, v: u32);
        fn render_data_extra_data(d: &TextureRenderData) -> u32;
        fn set_render_data_extra_data(d: Pin<&mut TextureRenderData>, v: u32);
        fn render_data_flags(d: &TextureRenderData) -> u8;
        fn set_render_data_flags(d: Pin<&mut TextureRenderData>, v: u8);
        fn render_data_tiling(d: &TextureRenderData) -> u32;
        fn set_render_data_tiling(d: Pin<&mut TextureRenderData>, v: u32);
        fn render_data_old_fast_compare(d: &TextureRenderData) -> [u8; 8];
        fn set_render_data_old_fast_compare(d: Pin<&mut TextureRenderData>, v: [u8; 8]);
        /// Red, green, blue and alpha masks; slices have 4 elements.
        fn render_data_masks(d: &TextureRenderData, out: &mut [u32]);
        fn set_render_data_masks(d: Pin<&mut TextureRenderData>, v: &[u32]);
        /// Channel `index` as component, representation, bits and signedness. Returns false if
        /// `index` isn't below 4.
        fn render_data_channel(d: &TextureRenderData, index: usize, out: &mut [u32]) -> bool;
        fn set_render_data_channel(d: Pin<&mut TextureRenderData>, index: usize, v: &[u32])
        -> bool;
        /// Mipmaps flattened as width, height, offset.
        fn render_data_mipmaps(d: &TextureRenderData) -> Vec<u32>;
        fn set_render_data_mipmaps(d: Pin<&mut TextureRenderData>, v: &[u32]);
        /// `paletteRef` block index; `NIF_NPOS` if empty.
        fn render_data_palette(d: &TextureRenderData) -> u32;

        /// Pixel bytes per face. The push setter keeps `numPixels` and `numFaces` in step; the
        /// caller makes every face the same length.
        fn pixel_data_face_count(d: &NiPixelData) -> usize;
        fn pixel_data_face(d: &NiPixelData, face: usize) -> &[u8];
        fn clear_pixel_data_faces(d: Pin<&mut NiPixelData>);
        fn push_pixel_data_face(d: Pin<&mut NiPixelData>, bytes: &[u8]);
        fn persistent_data_face_count(d: &NiPersistentSrcTextureRendererData) -> usize;
        fn persistent_data_face(d: &NiPersistentSrcTextureRendererData, face: usize) -> &[u8];
        fn clear_persistent_data_faces(d: Pin<&mut NiPersistentSrcTextureRendererData>);
        fn push_persistent_data_face(d: Pin<&mut NiPersistentSrcTextureRendererData>, bytes: &[u8]);
        fn persistent_data_platform(d: &NiPersistentSrcTextureRendererData) -> u32;
        fn set_persistent_data_platform(d: Pin<&mut NiPersistentSrcTextureRendererData>, v: u32);
        fn persistent_data_padded_pixel_count(d: &NiPersistentSrcTextureRendererData) -> u32;
        fn set_persistent_data_padded_pixel_count(
            d: Pin<&mut NiPersistentSrcTextureRendererData>,
            v: u32,
        );

        /// `NiPalette` fields. Colors are flattened r, g, b, a; the setter returns false, changing
        /// nothing, unless there are 16 or 256.
        fn palette_has_alpha(p: &NiPalette) -> bool;
        /// `TextureSetRef()` block index; `NIF_NPOS` if the shader type has none or it's empty.
        fn shader_texture_set(shader: &NiShader) -> u32;
        fn set_palette_has_alpha(p: Pin<&mut NiPalette>, v: bool);
        fn palette_colors(p: &NiPalette) -> Vec<u8>;
        fn set_palette_colors(p: Pin<&mut NiPalette>, rgba: &[u8]) -> bool;

        /// Fallout 3 and New Vegas shader fields, and those of `BSWaterShaderProperty` and
        /// `BSSkyShaderProperty`; the field names are in `shaders_shim_properties.h`. Enums are passed as
        /// `u32`, `NiString`s as bytes.
        fn shader_legacy_type(s: &BSShaderProperty) -> u32;
        fn set_shader_legacy_type(s: Pin<&mut BSShaderProperty>, value: u32);
        fn shader_lighting_clamp_mode(s: &BSShaderLightingProperty) -> u32;
        fn set_shader_lighting_clamp_mode(s: Pin<&mut BSShaderLightingProperty>, value: u32);
        fn pp_lighting_refraction_strength(s: &BSShaderPPLightingProperty) -> f32;
        fn set_pp_lighting_refraction_strength(s: Pin<&mut BSShaderPPLightingProperty>, value: f32);
        fn pp_lighting_refraction_fire_period(s: &BSShaderPPLightingProperty) -> i32;
        fn set_pp_lighting_refraction_fire_period(
            s: Pin<&mut BSShaderPPLightingProperty>,
            value: i32,
        );
        fn pp_lighting_parallax_max_passes(s: &BSShaderPPLightingProperty) -> f32;
        fn set_pp_lighting_parallax_max_passes(s: Pin<&mut BSShaderPPLightingProperty>, value: f32);
        fn pp_lighting_parallax_scale(s: &BSShaderPPLightingProperty) -> f32;
        fn set_pp_lighting_parallax_scale(s: Pin<&mut BSShaderPPLightingProperty>, value: f32);
        fn pp_lighting_emissive_color(s: &BSShaderPPLightingProperty) -> Rgba;
        fn set_pp_lighting_emissive_color(s: Pin<&mut BSShaderPPLightingProperty>, value: Rgba);
        fn no_lighting_base_texture(s: &BSShaderNoLightingProperty) -> &[u8];
        fn set_no_lighting_base_texture(s: Pin<&mut BSShaderNoLightingProperty>, value: &[u8]);
        fn no_lighting_falloff_start_angle(s: &BSShaderNoLightingProperty) -> f32;
        fn set_no_lighting_falloff_start_angle(s: Pin<&mut BSShaderNoLightingProperty>, value: f32);
        fn no_lighting_falloff_stop_angle(s: &BSShaderNoLightingProperty) -> f32;
        fn set_no_lighting_falloff_stop_angle(s: Pin<&mut BSShaderNoLightingProperty>, value: f32);
        fn no_lighting_falloff_start_opacity(s: &BSShaderNoLightingProperty) -> f32;
        fn set_no_lighting_falloff_start_opacity(
            s: Pin<&mut BSShaderNoLightingProperty>,
            value: f32,
        );
        fn no_lighting_falloff_stop_opacity(s: &BSShaderNoLightingProperty) -> f32;
        fn set_no_lighting_falloff_stop_opacity(
            s: Pin<&mut BSShaderNoLightingProperty>,
            value: f32,
        );
        fn sky_file_name(s: &SkyShaderProperty) -> &[u8];
        fn set_sky_file_name(s: Pin<&mut SkyShaderProperty>, value: &[u8]);
        fn sky_object_type(s: &SkyShaderProperty) -> u32;
        fn set_sky_object_type(s: Pin<&mut SkyShaderProperty>, value: u32);
        fn tile_file_name(s: &TileShaderProperty) -> &[u8];
        fn set_tile_file_name(s: Pin<&mut TileShaderProperty>, value: &[u8]);
        fn tall_grass_file_name(s: &TallGrassShaderProperty) -> &[u8];
        fn set_tall_grass_file_name(s: Pin<&mut TallGrassShaderProperty>, value: &[u8]);
        fn bs_water_flags(s: &BSWaterShaderProperty) -> u32;
        fn set_bs_water_flags(s: Pin<&mut BSWaterShaderProperty>, value: u32);
        fn bs_sky_base_texture(s: &BSSkyShaderProperty) -> &[u8];
        fn set_bs_sky_base_texture(s: Pin<&mut BSSkyShaderProperty>, value: &[u8]);
        fn bs_sky_object_type(s: &BSSkyShaderProperty) -> u32;
        fn set_bs_sky_object_type(s: Pin<&mut BSSkyShaderProperty>, value: u32);

        /// Number of texture slots in the set (13 for Skyrim SE and Fallout 4).
        fn texture_set_len(set: &BSShaderTextureSet) -> usize;
        /// The path in slot `index`. Empty if out of range.
        fn texture_set_path(set: &BSShaderTextureSet, index: usize) -> &[u8];
        fn set_texture_set_len(set: Pin<&mut BSShaderTextureSet>, len: usize);
        /// False if `index` is out of range.
        fn set_texture_set_path(
            set: Pin<&mut BSShaderTextureSet>,
            index: usize,
            path: &[u8],
        ) -> bool;

        /// `NiShadeProperty::shadingFlags`.
        fn shade_flags(s: &NiShadeProperty) -> u16;
        fn set_shade_flags(s: Pin<&mut NiShadeProperty>, value: u16);
        /// `legacyFlags` of the material and stencil properties.
        fn material_legacy_flags(s: &NiMaterialProperty) -> u16;
        fn set_material_legacy_flags(s: Pin<&mut NiMaterialProperty>, value: u16);
        fn stencil_legacy_flags(s: &NiStencilProperty) -> u16;
        fn set_stencil_legacy_flags(s: Pin<&mut NiStencilProperty>, value: u16);
        /// `BSShaderProperty::SF1` (list 0) or `SF2` (list 1). The setter keeps `numSF1`/`numSF2`.
        fn shader_crc_flags(s: &BSShaderProperty, list: u32) -> &[u32];
        fn set_shader_crc_flags(s: Pin<&mut BSShaderProperty>, list: u32, flags: &[u32]);
        /// `BSLightingShaderProperty` fields of unknown meaning.
        fn lighting_unknown_float(s: &BSLightingShaderProperty) -> f32;
        fn set_lighting_unknown_float(s: Pin<&mut BSLightingShaderProperty>, value: f32);
        fn lighting_unknown_float_1(s: &BSLightingShaderProperty) -> f32;
        fn set_lighting_unknown_float_1(s: Pin<&mut BSLightingShaderProperty>, value: f32);
        fn lighting_unknown_float_2(s: &BSLightingShaderProperty) -> f32;
        fn set_lighting_unknown_float_2(s: Pin<&mut BSLightingShaderProperty>, value: f32);
        fn lighting_unknown_short(s: &BSLightingShaderProperty) -> u16;
        fn set_lighting_unknown_short(s: Pin<&mut BSLightingShaderProperty>, value: u16);
        fn lighting_wetness_unknown_1(s: &BSLightingShaderProperty) -> f32;
        fn set_lighting_wetness_unknown_1(s: Pin<&mut BSLightingShaderProperty>, value: f32);
        fn lighting_wetness_unknown_2(s: &BSLightingShaderProperty) -> f32;
        fn set_lighting_wetness_unknown_2(s: Pin<&mut BSLightingShaderProperty>, value: f32);
        /// `BSEffectShaderProperty` fields of unknown meaning.
        fn effect_unknown_float(s: &BSEffectShaderProperty) -> f32;
        fn set_effect_unknown_float(s: Pin<&mut BSEffectShaderProperty>, value: f32);
        fn effect_unknown_byte(s: &BSEffectShaderProperty) -> u8;
        fn set_effect_unknown_byte(s: Pin<&mut BSEffectShaderProperty>, value: u8);
        fn effect_unknown_bytes(s: &BSEffectShaderProperty) -> [u8; 7];
        fn set_effect_unknown_bytes(s: Pin<&mut BSEffectShaderProperty>, value: [u8; 7]);
        fn effect_unknown_floats(s: &BSEffectShaderProperty) -> [f32; 6];
        fn set_effect_unknown_floats(s: Pin<&mut BSEffectShaderProperty>, value: [f32; 6]);
    }
}
