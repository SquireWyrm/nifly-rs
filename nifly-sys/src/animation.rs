//! Keys, animation data, interpolators, controllers and sequences.
//!
//! For each type `T` with direct base `Parent`:
//! - `upcast_<t>(&T) -> &Parent`
//! - `downcast_<t>(&NiObject) -> *const T`. Null unless the object is a `T` or a subclass of `T`.

#[cxx::bridge]
pub mod ffi {
    /// An `NiAnimationKey` of any value type. Values are four floats: a float or bool uses the
    /// first, a `Vector3` the first three, a `Color4` all four (r, g, b, a) and a `Quaternion` all
    /// four (x, y, z, w). `tbc` is tension, bias and continuity.
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct KeyFfi {
        time: f32,
        value: [f32; 4],
        forward: [f32; 4],
        backward: [f32; 4],
        tbc: [f32; 3],
    }

    /// An `NiAnimationKeyGroup`: its `NiKeyType` and keys.
    #[derive(Debug, Clone, PartialEq)]
    struct KeyGroupFfi {
        interpolation: u32,
        keys: Vec<KeyFfi>,
    }

    /// A `Morph` of an `NiMorphData`.
    #[derive(Debug, Clone, PartialEq)]
    struct MorphFfi {
        name: Vec<u8>,
        keys: KeyGroupFfi,
        legacy_weight: f32,
        vectors: Vec<Vec3>,
    }

    /// A `QuatTransform`, or the translation, rotation and scale of a block with no
    /// `trsValid`. The rotation is x, y, z, w.
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct QuatTransformFfi {
        translation: Vec3,
        rotation: [f32; 4],
        scale: f32,
    }

    /// An `InterpBlendItem`.
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct BlendItemFfi {
        interpolator: u32,
        weight: f32,
        normalized_weight: f32,
        priority: u8,
        priority_32: u32,
        ease_spinner: f32,
    }

    /// A `MorphWeight` of an `NiGeomMorpherController`.
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct MorphWeightFfi {
        interpolator: u32,
        weight: f32,
    }

    /// One `NiBoneLODController` node group.
    #[derive(Debug, Clone, PartialEq)]
    struct NodeGroupFfi {
        nodes: Vec<u32>,
    }

    /// A `BSTreadTransform`.
    #[derive(Debug, Clone, PartialEq)]
    struct TreadFfi {
        name: Vec<u8>,
        transform_1: QuatTransformFfi,
        transform_2: QuatTransformFfi,
    }

    #[namespace = "nifly"]
    unsafe extern "C++" {
        include!("Animation.hpp");
        include!("Particles.hpp");

        type NiKeyframeData;
        type NiTransformData;
        type NiPosData;
        type NiBoolData;
        type NiFloatData;
        type NiColorData;
        type NiUVData;
        type NiVisData;
        type NiMorphData;
        type NiBSplineData;
        type NiBSplineBasisData;
        type NiStringPalette;
        type NiInterpolator;
        type NiBSplineInterpolator;
        type NiBSplineFloatInterpolator;
        type NiBSplineCompFloatInterpolator;
        type NiBSplinePoint3Interpolator;
        type NiBSplineCompPoint3Interpolator;
        type NiBSplineTransformInterpolator;
        type NiBSplineCompTransformInterpolator;
        type NiBlendInterpolator;
        type NiBlendBoolInterpolator;
        type NiBlendFloatInterpolator;
        type NiBlendPoint3Interpolator;
        type NiBlendTransformInterpolator;
        type NiKeyBasedInterpolator;
        type NiBoolInterpolator;
        type NiBoolTimelineInterpolator;
        type NiFloatInterpolator;
        type NiTransformInterpolator;
        type BSRotAccumTransfInterpolator;
        type NiPoint3Interpolator;
        type NiPathInterpolator;
        type NiLookAtInterpolator;
        type BSTreadTransfInterpolator;
        type NiTimeController;
        type NiLookAtController;
        type NiPathController;
        type NiPSysResetOnLoopCtlr;
        type NiUVController;
        type BSFrustumFOVController;
        type BSLagBoneController;
        type BSProceduralLightningController;
        type NiBoneLODController;
        type NiBSBoneLODController;
        type NiInterpController;
        type NiGeomMorpherController;
        type NiSingleInterpController;
        type NiRollController;
        type NiPoint3InterpController;
        type NiMaterialColorController;
        type NiLightColorController;
        type NiExtraDataController;
        type NiFloatExtraDataController;
        type NiBoolInterpController;
        type NiVisController;
        type NiFloatInterpController;
        type BSRefractionFirePeriodController;
        type NiFlipController;
        type NiTextureTransformController;
        type NiLightDimmerController;
        type NiLightRadiusController;
        type NiAlphaController;
        type NiPSysUpdateCtlr;
        type BSNiAlphaPropertyTestRefController;
        type NiKeyframeController;
        type NiTransformController;
        type BSMaterialEmittanceMultController;
        type BSRefractionStrengthController;
        type BSLightingShaderPropertyColorController;
        type BSLightingShaderPropertyFloatController;
        type BSLightingShaderPropertyUShortController;
        type BSEffectShaderPropertyColorController;
        type BSEffectShaderPropertyFloatController;
        type NiMultiTargetTransformController;
        type NiPSysModifierCtlr;
        type NiPSysModifierBoolCtlr;
        type NiPSysModifierActiveCtlr;
        type NiPSysModifierFloatCtlr;
        type NiPSysEmitterLifeSpanCtlr;
        type NiPSysEmitterSpeedCtlr;
        type NiPSysEmitterInitialRadiusCtlr;
        type NiPSysEmitterDeclinationCtlr;
        type NiPSysGravityStrengthCtlr;
        type NiPSysEmitterDeclinationVarCtlr;
        type NiPSysFieldMagnitudeCtlr;
        type NiPSysFieldAttenuationCtlr;
        type NiPSysFieldMaxDistanceCtlr;
        type NiPSysAirFieldAirFrictionCtlr;
        type NiPSysAirFieldInheritVelocityCtlr;
        type NiPSysAirFieldSpreadCtlr;
        type NiPSysInitialRotSpeedCtlr;
        type NiPSysInitialRotSpeedVarCtlr;
        type NiPSysInitialRotAngleCtlr;
        type NiPSysInitialRotAngleVarCtlr;
        type NiPSysEmitterPlanarAngleCtlr;
        type NiPSysEmitterPlanarAngleVarCtlr;
        type NiPSysRotDampeningCtlr;
        type NiSequence;
        type BSAnimNote;
        type BSAnimNotes;
        type NiControllerSequence;
        type NiControllerManager;
        type bhkBlendController;
    }

    #[namespace = "nifly"]
    unsafe extern "C++" {
        include!("NifFile.hpp");

        type NiObject = crate::objects::ffi::NiObject;
        type NifFile = crate::nif_file::ffi::NifFile;
    }

    unsafe extern "C++" {
        include!("nifly-sys/src/object_3d.rs.h");

        type Vec3 = crate::object_3d::ffi::Vec3;
    }

    #[namespace = "nifly_rs"]
    unsafe extern "C++" {
        include!("animation_shim.h");

        fn upcast_ni_keyframe_data(obj: &NiKeyframeData) -> &NiObject;
        fn downcast_ni_keyframe_data(obj: &NiObject) -> *const NiKeyframeData;
        fn upcast_ni_transform_data(obj: &NiTransformData) -> &NiKeyframeData;
        fn downcast_ni_transform_data(obj: &NiObject) -> *const NiTransformData;
        fn upcast_ni_pos_data(obj: &NiPosData) -> &NiObject;
        fn downcast_ni_pos_data(obj: &NiObject) -> *const NiPosData;
        fn upcast_ni_bool_data(obj: &NiBoolData) -> &NiObject;
        fn downcast_ni_bool_data(obj: &NiObject) -> *const NiBoolData;
        fn upcast_ni_float_data(obj: &NiFloatData) -> &NiObject;
        fn downcast_ni_float_data(obj: &NiObject) -> *const NiFloatData;
        fn upcast_ni_color_data(obj: &NiColorData) -> &NiObject;
        fn downcast_ni_color_data(obj: &NiObject) -> *const NiColorData;
        fn upcast_ni_uv_data(obj: &NiUVData) -> &NiObject;
        fn downcast_ni_uv_data(obj: &NiObject) -> *const NiUVData;
        fn upcast_ni_vis_data(obj: &NiVisData) -> &NiObject;
        fn downcast_ni_vis_data(obj: &NiObject) -> *const NiVisData;
        fn upcast_ni_morph_data(obj: &NiMorphData) -> &NiObject;
        fn downcast_ni_morph_data(obj: &NiObject) -> *const NiMorphData;
        fn upcast_ni_bspline_data(obj: &NiBSplineData) -> &NiObject;
        fn downcast_ni_bspline_data(obj: &NiObject) -> *const NiBSplineData;
        fn upcast_ni_bspline_basis_data(obj: &NiBSplineBasisData) -> &NiObject;
        fn downcast_ni_bspline_basis_data(obj: &NiObject) -> *const NiBSplineBasisData;
        fn upcast_ni_string_palette(obj: &NiStringPalette) -> &NiObject;
        fn downcast_ni_string_palette(obj: &NiObject) -> *const NiStringPalette;
        fn upcast_ni_interpolator(obj: &NiInterpolator) -> &NiObject;
        fn downcast_ni_interpolator(obj: &NiObject) -> *const NiInterpolator;
        fn upcast_ni_bspline_interpolator(obj: &NiBSplineInterpolator) -> &NiInterpolator;
        fn downcast_ni_bspline_interpolator(obj: &NiObject) -> *const NiBSplineInterpolator;
        fn upcast_ni_bspline_float_interpolator(obj: &NiBSplineFloatInterpolator) -> &NiBSplineInterpolator;
        fn downcast_ni_bspline_float_interpolator(obj: &NiObject) -> *const NiBSplineFloatInterpolator;
        fn upcast_ni_bspline_comp_float_interpolator(obj: &NiBSplineCompFloatInterpolator) -> &NiBSplineFloatInterpolator;
        fn downcast_ni_bspline_comp_float_interpolator(obj: &NiObject) -> *const NiBSplineCompFloatInterpolator;
        fn upcast_ni_bspline_point3_interpolator(obj: &NiBSplinePoint3Interpolator) -> &NiBSplineInterpolator;
        fn downcast_ni_bspline_point3_interpolator(obj: &NiObject) -> *const NiBSplinePoint3Interpolator;
        fn upcast_ni_bspline_comp_point3_interpolator(obj: &NiBSplineCompPoint3Interpolator) -> &NiBSplinePoint3Interpolator;
        fn downcast_ni_bspline_comp_point3_interpolator(obj: &NiObject) -> *const NiBSplineCompPoint3Interpolator;
        fn upcast_ni_bspline_transform_interpolator(obj: &NiBSplineTransformInterpolator) -> &NiBSplineInterpolator;
        fn downcast_ni_bspline_transform_interpolator(obj: &NiObject) -> *const NiBSplineTransformInterpolator;
        fn upcast_ni_bspline_comp_transform_interpolator(obj: &NiBSplineCompTransformInterpolator) -> &NiBSplineTransformInterpolator;
        fn downcast_ni_bspline_comp_transform_interpolator(obj: &NiObject) -> *const NiBSplineCompTransformInterpolator;
        fn upcast_ni_blend_interpolator(obj: &NiBlendInterpolator) -> &NiInterpolator;
        fn downcast_ni_blend_interpolator(obj: &NiObject) -> *const NiBlendInterpolator;
        fn upcast_ni_blend_bool_interpolator(obj: &NiBlendBoolInterpolator) -> &NiBlendInterpolator;
        fn downcast_ni_blend_bool_interpolator(obj: &NiObject) -> *const NiBlendBoolInterpolator;
        fn upcast_ni_blend_float_interpolator(obj: &NiBlendFloatInterpolator) -> &NiBlendInterpolator;
        fn downcast_ni_blend_float_interpolator(obj: &NiObject) -> *const NiBlendFloatInterpolator;
        fn upcast_ni_blend_point3_interpolator(obj: &NiBlendPoint3Interpolator) -> &NiBlendInterpolator;
        fn downcast_ni_blend_point3_interpolator(obj: &NiObject) -> *const NiBlendPoint3Interpolator;
        fn upcast_ni_blend_transform_interpolator(obj: &NiBlendTransformInterpolator) -> &NiBlendInterpolator;
        fn downcast_ni_blend_transform_interpolator(obj: &NiObject) -> *const NiBlendTransformInterpolator;
        fn upcast_ni_key_based_interpolator(obj: &NiKeyBasedInterpolator) -> &NiInterpolator;
        fn downcast_ni_key_based_interpolator(obj: &NiObject) -> *const NiKeyBasedInterpolator;
        fn upcast_ni_bool_interpolator(obj: &NiBoolInterpolator) -> &NiKeyBasedInterpolator;
        fn downcast_ni_bool_interpolator(obj: &NiObject) -> *const NiBoolInterpolator;
        fn upcast_ni_bool_timeline_interpolator(obj: &NiBoolTimelineInterpolator) -> &NiBoolInterpolator;
        fn downcast_ni_bool_timeline_interpolator(obj: &NiObject) -> *const NiBoolTimelineInterpolator;
        fn upcast_ni_float_interpolator(obj: &NiFloatInterpolator) -> &NiKeyBasedInterpolator;
        fn downcast_ni_float_interpolator(obj: &NiObject) -> *const NiFloatInterpolator;
        fn upcast_ni_transform_interpolator(obj: &NiTransformInterpolator) -> &NiKeyBasedInterpolator;
        fn downcast_ni_transform_interpolator(obj: &NiObject) -> *const NiTransformInterpolator;
        fn upcast_bs_rot_accum_transf_interpolator(obj: &BSRotAccumTransfInterpolator) -> &NiTransformInterpolator;
        fn downcast_bs_rot_accum_transf_interpolator(obj: &NiObject) -> *const BSRotAccumTransfInterpolator;
        fn upcast_ni_point3_interpolator(obj: &NiPoint3Interpolator) -> &NiKeyBasedInterpolator;
        fn downcast_ni_point3_interpolator(obj: &NiObject) -> *const NiPoint3Interpolator;
        fn upcast_ni_path_interpolator(obj: &NiPathInterpolator) -> &NiKeyBasedInterpolator;
        fn downcast_ni_path_interpolator(obj: &NiObject) -> *const NiPathInterpolator;
        fn upcast_ni_look_at_interpolator(obj: &NiLookAtInterpolator) -> &NiInterpolator;
        fn downcast_ni_look_at_interpolator(obj: &NiObject) -> *const NiLookAtInterpolator;
        fn upcast_bs_tread_transf_interpolator(obj: &BSTreadTransfInterpolator) -> &NiInterpolator;
        fn downcast_bs_tread_transf_interpolator(obj: &NiObject) -> *const BSTreadTransfInterpolator;
        fn upcast_ni_time_controller(obj: &NiTimeController) -> &NiObject;
        fn downcast_ni_time_controller(obj: &NiObject) -> *const NiTimeController;
        fn upcast_ni_look_at_controller(obj: &NiLookAtController) -> &NiTimeController;
        fn downcast_ni_look_at_controller(obj: &NiObject) -> *const NiLookAtController;
        fn upcast_ni_path_controller(obj: &NiPathController) -> &NiTimeController;
        fn downcast_ni_path_controller(obj: &NiObject) -> *const NiPathController;
        fn upcast_ni_psys_reset_on_loop_ctlr(obj: &NiPSysResetOnLoopCtlr) -> &NiTimeController;
        fn downcast_ni_psys_reset_on_loop_ctlr(obj: &NiObject) -> *const NiPSysResetOnLoopCtlr;
        fn upcast_ni_uv_controller(obj: &NiUVController) -> &NiTimeController;
        fn downcast_ni_uv_controller(obj: &NiObject) -> *const NiUVController;
        fn upcast_bs_frustum_fov_controller(obj: &BSFrustumFOVController) -> &NiTimeController;
        fn downcast_bs_frustum_fov_controller(obj: &NiObject) -> *const BSFrustumFOVController;
        fn upcast_bs_lag_bone_controller(obj: &BSLagBoneController) -> &NiTimeController;
        fn downcast_bs_lag_bone_controller(obj: &NiObject) -> *const BSLagBoneController;
        fn upcast_bs_procedural_lightning_controller(obj: &BSProceduralLightningController) -> &NiTimeController;
        fn downcast_bs_procedural_lightning_controller(obj: &NiObject) -> *const BSProceduralLightningController;
        fn upcast_ni_bone_lod_controller(obj: &NiBoneLODController) -> &NiTimeController;
        fn downcast_ni_bone_lod_controller(obj: &NiObject) -> *const NiBoneLODController;
        fn upcast_ni_bs_bone_lod_controller(obj: &NiBSBoneLODController) -> &NiBoneLODController;
        fn downcast_ni_bs_bone_lod_controller(obj: &NiObject) -> *const NiBSBoneLODController;
        fn upcast_ni_interp_controller(obj: &NiInterpController) -> &NiTimeController;
        fn downcast_ni_interp_controller(obj: &NiObject) -> *const NiInterpController;
        fn upcast_ni_geom_morpher_controller(obj: &NiGeomMorpherController) -> &NiInterpController;
        fn downcast_ni_geom_morpher_controller(obj: &NiObject) -> *const NiGeomMorpherController;
        fn upcast_ni_single_interp_controller(obj: &NiSingleInterpController) -> &NiInterpController;
        fn downcast_ni_single_interp_controller(obj: &NiObject) -> *const NiSingleInterpController;
        fn upcast_ni_roll_controller(obj: &NiRollController) -> &NiSingleInterpController;
        fn downcast_ni_roll_controller(obj: &NiObject) -> *const NiRollController;
        fn upcast_ni_point3_interp_controller(obj: &NiPoint3InterpController) -> &NiSingleInterpController;
        fn downcast_ni_point3_interp_controller(obj: &NiObject) -> *const NiPoint3InterpController;
        fn upcast_ni_material_color_controller(obj: &NiMaterialColorController) -> &NiPoint3InterpController;
        fn downcast_ni_material_color_controller(obj: &NiObject) -> *const NiMaterialColorController;
        fn upcast_ni_light_color_controller(obj: &NiLightColorController) -> &NiPoint3InterpController;
        fn downcast_ni_light_color_controller(obj: &NiObject) -> *const NiLightColorController;
        fn upcast_ni_extra_data_controller(obj: &NiExtraDataController) -> &NiSingleInterpController;
        fn downcast_ni_extra_data_controller(obj: &NiObject) -> *const NiExtraDataController;
        fn upcast_ni_float_extra_data_controller(obj: &NiFloatExtraDataController) -> &NiExtraDataController;
        fn downcast_ni_float_extra_data_controller(obj: &NiObject) -> *const NiFloatExtraDataController;
        fn upcast_ni_bool_interp_controller(obj: &NiBoolInterpController) -> &NiSingleInterpController;
        fn downcast_ni_bool_interp_controller(obj: &NiObject) -> *const NiBoolInterpController;
        fn upcast_ni_vis_controller(obj: &NiVisController) -> &NiBoolInterpController;
        fn downcast_ni_vis_controller(obj: &NiObject) -> *const NiVisController;
        fn upcast_ni_float_interp_controller(obj: &NiFloatInterpController) -> &NiSingleInterpController;
        fn downcast_ni_float_interp_controller(obj: &NiObject) -> *const NiFloatInterpController;
        fn upcast_bs_refraction_fire_period_controller(obj: &BSRefractionFirePeriodController) -> &NiSingleInterpController;
        fn downcast_bs_refraction_fire_period_controller(obj: &NiObject) -> *const BSRefractionFirePeriodController;
        fn upcast_ni_flip_controller(obj: &NiFlipController) -> &NiFloatInterpController;
        fn downcast_ni_flip_controller(obj: &NiObject) -> *const NiFlipController;
        fn upcast_ni_texture_transform_controller(obj: &NiTextureTransformController) -> &NiFloatInterpController;
        fn downcast_ni_texture_transform_controller(obj: &NiObject) -> *const NiTextureTransformController;
        fn upcast_ni_light_dimmer_controller(obj: &NiLightDimmerController) -> &NiFloatInterpController;
        fn downcast_ni_light_dimmer_controller(obj: &NiObject) -> *const NiLightDimmerController;
        fn upcast_ni_light_radius_controller(obj: &NiLightRadiusController) -> &NiFloatInterpController;
        fn downcast_ni_light_radius_controller(obj: &NiObject) -> *const NiLightRadiusController;
        fn upcast_ni_alpha_controller(obj: &NiAlphaController) -> &NiFloatInterpController;
        fn downcast_ni_alpha_controller(obj: &NiObject) -> *const NiAlphaController;
        fn upcast_ni_psys_update_ctlr(obj: &NiPSysUpdateCtlr) -> &NiTimeController;
        fn downcast_ni_psys_update_ctlr(obj: &NiObject) -> *const NiPSysUpdateCtlr;
        fn upcast_bs_ni_alpha_property_test_ref_controller(obj: &BSNiAlphaPropertyTestRefController) -> &NiAlphaController;
        fn downcast_bs_ni_alpha_property_test_ref_controller(obj: &NiObject) -> *const BSNiAlphaPropertyTestRefController;
        fn upcast_ni_keyframe_controller(obj: &NiKeyframeController) -> &NiSingleInterpController;
        fn downcast_ni_keyframe_controller(obj: &NiObject) -> *const NiKeyframeController;
        fn upcast_ni_transform_controller(obj: &NiTransformController) -> &NiKeyframeController;
        fn downcast_ni_transform_controller(obj: &NiObject) -> *const NiTransformController;
        fn upcast_bs_material_emittance_mult_controller(obj: &BSMaterialEmittanceMultController) -> &NiFloatInterpController;
        fn downcast_bs_material_emittance_mult_controller(obj: &NiObject) -> *const BSMaterialEmittanceMultController;
        fn upcast_bs_refraction_strength_controller(obj: &BSRefractionStrengthController) -> &NiFloatInterpController;
        fn downcast_bs_refraction_strength_controller(obj: &NiObject) -> *const BSRefractionStrengthController;
        fn upcast_bs_lighting_shader_property_color_controller(obj: &BSLightingShaderPropertyColorController) -> &NiFloatInterpController;
        fn downcast_bs_lighting_shader_property_color_controller(obj: &NiObject) -> *const BSLightingShaderPropertyColorController;
        fn upcast_bs_lighting_shader_property_float_controller(obj: &BSLightingShaderPropertyFloatController) -> &NiFloatInterpController;
        fn downcast_bs_lighting_shader_property_float_controller(obj: &NiObject) -> *const BSLightingShaderPropertyFloatController;
        fn upcast_bs_lighting_shader_property_u_short_controller(obj: &BSLightingShaderPropertyUShortController) -> &NiFloatInterpController;
        fn downcast_bs_lighting_shader_property_u_short_controller(obj: &NiObject) -> *const BSLightingShaderPropertyUShortController;
        fn upcast_bs_effect_shader_property_color_controller(obj: &BSEffectShaderPropertyColorController) -> &NiFloatInterpController;
        fn downcast_bs_effect_shader_property_color_controller(obj: &NiObject) -> *const BSEffectShaderPropertyColorController;
        fn upcast_bs_effect_shader_property_float_controller(obj: &BSEffectShaderPropertyFloatController) -> &NiFloatInterpController;
        fn downcast_bs_effect_shader_property_float_controller(obj: &NiObject) -> *const BSEffectShaderPropertyFloatController;
        fn upcast_ni_multi_target_transform_controller(obj: &NiMultiTargetTransformController) -> &NiInterpController;
        fn downcast_ni_multi_target_transform_controller(obj: &NiObject) -> *const NiMultiTargetTransformController;
        fn upcast_ni_psys_modifier_ctlr(obj: &NiPSysModifierCtlr) -> &NiSingleInterpController;
        fn downcast_ni_psys_modifier_ctlr(obj: &NiObject) -> *const NiPSysModifierCtlr;
        fn upcast_ni_psys_modifier_bool_ctlr(obj: &NiPSysModifierBoolCtlr) -> &NiPSysModifierCtlr;
        fn downcast_ni_psys_modifier_bool_ctlr(obj: &NiObject) -> *const NiPSysModifierBoolCtlr;
        fn upcast_ni_psys_modifier_active_ctlr(obj: &NiPSysModifierActiveCtlr) -> &NiPSysModifierBoolCtlr;
        fn downcast_ni_psys_modifier_active_ctlr(obj: &NiObject) -> *const NiPSysModifierActiveCtlr;
        fn upcast_ni_psys_modifier_float_ctlr(obj: &NiPSysModifierFloatCtlr) -> &NiPSysModifierCtlr;
        fn downcast_ni_psys_modifier_float_ctlr(obj: &NiObject) -> *const NiPSysModifierFloatCtlr;
        fn upcast_ni_psys_emitter_life_span_ctlr(obj: &NiPSysEmitterLifeSpanCtlr) -> &NiPSysModifierFloatCtlr;
        fn downcast_ni_psys_emitter_life_span_ctlr(obj: &NiObject) -> *const NiPSysEmitterLifeSpanCtlr;
        fn upcast_ni_psys_emitter_speed_ctlr(obj: &NiPSysEmitterSpeedCtlr) -> &NiPSysModifierFloatCtlr;
        fn downcast_ni_psys_emitter_speed_ctlr(obj: &NiObject) -> *const NiPSysEmitterSpeedCtlr;
        fn upcast_ni_psys_emitter_initial_radius_ctlr(obj: &NiPSysEmitterInitialRadiusCtlr) -> &NiPSysModifierFloatCtlr;
        fn downcast_ni_psys_emitter_initial_radius_ctlr(obj: &NiObject) -> *const NiPSysEmitterInitialRadiusCtlr;
        fn upcast_ni_psys_emitter_declination_ctlr(obj: &NiPSysEmitterDeclinationCtlr) -> &NiPSysModifierFloatCtlr;
        fn downcast_ni_psys_emitter_declination_ctlr(obj: &NiObject) -> *const NiPSysEmitterDeclinationCtlr;
        fn upcast_ni_psys_gravity_strength_ctlr(obj: &NiPSysGravityStrengthCtlr) -> &NiPSysModifierFloatCtlr;
        fn downcast_ni_psys_gravity_strength_ctlr(obj: &NiObject) -> *const NiPSysGravityStrengthCtlr;
        fn upcast_ni_psys_emitter_declination_var_ctlr(obj: &NiPSysEmitterDeclinationVarCtlr) -> &NiPSysModifierFloatCtlr;
        fn downcast_ni_psys_emitter_declination_var_ctlr(obj: &NiObject) -> *const NiPSysEmitterDeclinationVarCtlr;
        fn upcast_ni_psys_field_magnitude_ctlr(obj: &NiPSysFieldMagnitudeCtlr) -> &NiPSysModifierFloatCtlr;
        fn downcast_ni_psys_field_magnitude_ctlr(obj: &NiObject) -> *const NiPSysFieldMagnitudeCtlr;
        fn upcast_ni_psys_field_attenuation_ctlr(obj: &NiPSysFieldAttenuationCtlr) -> &NiPSysModifierFloatCtlr;
        fn downcast_ni_psys_field_attenuation_ctlr(obj: &NiObject) -> *const NiPSysFieldAttenuationCtlr;
        fn upcast_ni_psys_field_max_distance_ctlr(obj: &NiPSysFieldMaxDistanceCtlr) -> &NiPSysModifierFloatCtlr;
        fn downcast_ni_psys_field_max_distance_ctlr(obj: &NiObject) -> *const NiPSysFieldMaxDistanceCtlr;
        fn upcast_ni_psys_air_field_air_friction_ctlr(obj: &NiPSysAirFieldAirFrictionCtlr) -> &NiPSysModifierFloatCtlr;
        fn downcast_ni_psys_air_field_air_friction_ctlr(obj: &NiObject) -> *const NiPSysAirFieldAirFrictionCtlr;
        fn upcast_ni_psys_air_field_inherit_velocity_ctlr(obj: &NiPSysAirFieldInheritVelocityCtlr) -> &NiPSysModifierFloatCtlr;
        fn downcast_ni_psys_air_field_inherit_velocity_ctlr(obj: &NiObject) -> *const NiPSysAirFieldInheritVelocityCtlr;
        fn upcast_ni_psys_air_field_spread_ctlr(obj: &NiPSysAirFieldSpreadCtlr) -> &NiPSysModifierFloatCtlr;
        fn downcast_ni_psys_air_field_spread_ctlr(obj: &NiObject) -> *const NiPSysAirFieldSpreadCtlr;
        fn upcast_ni_psys_initial_rot_speed_ctlr(obj: &NiPSysInitialRotSpeedCtlr) -> &NiPSysModifierFloatCtlr;
        fn downcast_ni_psys_initial_rot_speed_ctlr(obj: &NiObject) -> *const NiPSysInitialRotSpeedCtlr;
        fn upcast_ni_psys_initial_rot_speed_var_ctlr(obj: &NiPSysInitialRotSpeedVarCtlr) -> &NiPSysModifierFloatCtlr;
        fn downcast_ni_psys_initial_rot_speed_var_ctlr(obj: &NiObject) -> *const NiPSysInitialRotSpeedVarCtlr;
        fn upcast_ni_psys_initial_rot_angle_ctlr(obj: &NiPSysInitialRotAngleCtlr) -> &NiPSysModifierFloatCtlr;
        fn downcast_ni_psys_initial_rot_angle_ctlr(obj: &NiObject) -> *const NiPSysInitialRotAngleCtlr;
        fn upcast_ni_psys_initial_rot_angle_var_ctlr(obj: &NiPSysInitialRotAngleVarCtlr) -> &NiPSysModifierFloatCtlr;
        fn downcast_ni_psys_initial_rot_angle_var_ctlr(obj: &NiObject) -> *const NiPSysInitialRotAngleVarCtlr;
        fn upcast_ni_psys_emitter_planar_angle_ctlr(obj: &NiPSysEmitterPlanarAngleCtlr) -> &NiPSysModifierFloatCtlr;
        fn downcast_ni_psys_emitter_planar_angle_ctlr(obj: &NiObject) -> *const NiPSysEmitterPlanarAngleCtlr;
        fn upcast_ni_psys_emitter_planar_angle_var_ctlr(obj: &NiPSysEmitterPlanarAngleVarCtlr) -> &NiPSysModifierFloatCtlr;
        fn downcast_ni_psys_emitter_planar_angle_var_ctlr(obj: &NiObject) -> *const NiPSysEmitterPlanarAngleVarCtlr;
        fn upcast_ni_psys_rot_dampening_ctlr(obj: &NiPSysRotDampeningCtlr) -> &NiPSysModifierFloatCtlr;
        fn downcast_ni_psys_rot_dampening_ctlr(obj: &NiObject) -> *const NiPSysRotDampeningCtlr;
        fn upcast_ni_sequence(obj: &NiSequence) -> &NiObject;
        fn downcast_ni_sequence(obj: &NiObject) -> *const NiSequence;
        fn upcast_bs_anim_note(obj: &BSAnimNote) -> &NiObject;
        fn downcast_bs_anim_note(obj: &NiObject) -> *const BSAnimNote;
        fn upcast_bs_anim_notes(obj: &BSAnimNotes) -> &NiObject;
        fn downcast_bs_anim_notes(obj: &NiObject) -> *const BSAnimNotes;
        fn upcast_ni_controller_sequence(obj: &NiControllerSequence) -> &NiSequence;
        fn downcast_ni_controller_sequence(obj: &NiObject) -> *const NiControllerSequence;
        fn upcast_ni_controller_manager(obj: &NiControllerManager) -> &NiTimeController;
        fn downcast_ni_controller_manager(obj: &NiObject) -> *const NiControllerManager;
        fn upcast_bhk_blend_controller(obj: &bhkBlendController) -> &NiTimeController;
        fn downcast_bhk_blend_controller(obj: &NiObject) -> *const bhkBlendController;

        // Key groups. Setters return false, changing nothing, if the group has keys but its
        // interpolation isn't linear, quadratic, TBC or constant, or if it has none and the
        // interpolation isn't one of those or zero.
        fn pos_data_keys(obj: &NiPosData) -> KeyGroupFfi;
        fn set_pos_data_keys(obj: Pin<&mut NiPosData>, group: &KeyGroupFfi) -> bool;
        fn float_data_keys(obj: &NiFloatData) -> KeyGroupFfi;
        fn set_float_data_keys(obj: Pin<&mut NiFloatData>, group: &KeyGroupFfi) -> bool;
        fn bool_data_keys(obj: &NiBoolData) -> KeyGroupFfi;
        fn set_bool_data_keys(obj: Pin<&mut NiBoolData>, group: &KeyGroupFfi) -> bool;
        fn color_data_keys(obj: &NiColorData) -> KeyGroupFfi;
        fn set_color_data_keys(obj: Pin<&mut NiColorData>, group: &KeyGroupFfi) -> bool;
        fn keyframe_translations(obj: &NiKeyframeData) -> KeyGroupFfi;
        fn set_keyframe_translations(obj: Pin<&mut NiKeyframeData>, group: &KeyGroupFfi) -> bool;
        fn keyframe_scales(obj: &NiKeyframeData) -> KeyGroupFfi;
        fn set_keyframe_scales(obj: Pin<&mut NiKeyframeData>, group: &KeyGroupFfi) -> bool;
        /// `which`: 0 u translation, 1 v translation, 2 u scale, 3 v scale. Out of range reads
        /// an empty group and sets nothing.
        fn uv_data_keys(obj: &NiUVData, which: usize) -> KeyGroupFfi;
        fn set_uv_data_keys(obj: Pin<&mut NiUVData>, which: usize, group: &KeyGroupFfi) -> bool;

        /// The rotation key type: 4 (`XYZ_ROTATION_KEY`) means three float groups.
        fn keyframe_rotation_type(obj: &NiKeyframeData) -> u32;
        /// The quaternion keys, with the rotation type as their interpolation.
        fn keyframe_quaternion_keys(obj: &NiKeyframeData) -> KeyGroupFfi;
        /// Sets quaternion keys and the rotation type, clearing the XYZ groups.
        fn set_keyframe_quaternion_keys(obj: Pin<&mut NiKeyframeData>, group: &KeyGroupFfi) -> bool;
        /// XYZ rotation group `axis` (0 to 2).
        fn keyframe_xyz_rotations(obj: &NiKeyframeData, axis: usize) -> KeyGroupFfi;
        /// Sets the three XYZ groups and the rotation type, clearing the quaternion keys.
        fn set_keyframe_xyz_rotations(
            obj: Pin<&mut NiKeyframeData>,
            x: &KeyGroupFfi,
            y: &KeyGroupFfi,
            z: &KeyGroupFfi,
        ) -> bool;

        /// Visibility keys: time and value only.
        fn vis_data_keys(obj: &NiVisData) -> Vec<KeyFfi>;
        fn set_vis_data_keys(obj: Pin<&mut NiVisData>, keys: &[KeyFfi]);

        fn morph_vertex_count(obj: &NiMorphData) -> u32;
        fn morph_relative_targets(obj: &NiMorphData) -> u8;
        fn set_morph_relative_targets(obj: Pin<&mut NiMorphData>, value: u8);
        fn morphs(obj: &NiMorphData) -> Vec<MorphFfi>;
        /// False, changing nothing, unless every morph has the same number of vectors and
        /// valid keys. The vertex count becomes that number, or stays if there are no morphs.
        fn set_morphs(obj: Pin<&mut NiMorphData>, morphs: &[MorphFfi]) -> bool;

        fn bspline_float_points(obj: &NiBSplineData) -> &[f32];
        fn set_bspline_float_points(obj: Pin<&mut NiBSplineData>, points: &[f32]);
        fn bspline_short_points(obj: &NiBSplineData) -> &[i16];
        fn set_bspline_short_points(obj: Pin<&mut NiBSplineData>, points: &[i16]);
        fn bspline_basis_count(obj: &NiBSplineBasisData) -> u32;
        fn set_bspline_basis_count(obj: Pin<&mut NiBSplineBasisData>, count: u32);

        fn string_palette(obj: &NiStringPalette) -> &[u8];
        fn set_string_palette(obj: Pin<&mut NiStringPalette>, palette: &[u8]);

        /// Adds a new block of the type and returns its index.
        fn add_ni_keyframe_data(file: Pin<&mut NifFile>) -> u32;
        fn add_ni_transform_data(file: Pin<&mut NifFile>) -> u32;
        fn add_ni_pos_data(file: Pin<&mut NifFile>) -> u32;
        fn add_ni_bool_data(file: Pin<&mut NifFile>) -> u32;
        fn add_ni_float_data(file: Pin<&mut NifFile>) -> u32;
        fn add_ni_color_data(file: Pin<&mut NifFile>) -> u32;
        fn add_ni_uv_data(file: Pin<&mut NifFile>) -> u32;
        fn add_ni_vis_data(file: Pin<&mut NifFile>) -> u32;
        fn add_ni_morph_data(file: Pin<&mut NifFile>) -> u32;
        fn add_ni_bspline_data(file: Pin<&mut NifFile>) -> u32;
        fn add_ni_bspline_basis_data(file: Pin<&mut NifFile>) -> u32;
        fn add_ni_string_palette(file: Pin<&mut NifFile>) -> u32;

        // Interpolators.
        fn bool_interpolator_value(obj: &NiBoolInterpolator) -> u8;
        fn set_bool_interpolator_value(obj: Pin<&mut NiBoolInterpolator>, value: u8);
        fn float_interpolator_value(obj: &NiFloatInterpolator) -> f32;
        fn set_float_interpolator_value(obj: Pin<&mut NiFloatInterpolator>, value: f32);
        fn blend_bool_value(obj: &NiBlendBoolInterpolator) -> u8;
        fn set_blend_bool_value(obj: Pin<&mut NiBlendBoolInterpolator>, value: u8);
        fn blend_float_value(obj: &NiBlendFloatInterpolator) -> f32;
        fn set_blend_float_value(obj: Pin<&mut NiBlendFloatInterpolator>, value: f32);
        fn blend_flags(obj: &NiBlendInterpolator) -> u8;
        fn set_blend_flags(obj: Pin<&mut NiBlendInterpolator>, value: u8);
        fn blend_array_size(obj: &NiBlendInterpolator) -> u16;
        fn set_blend_array_size(obj: Pin<&mut NiBlendInterpolator>, value: u16);
        fn blend_array_grow_by(obj: &NiBlendInterpolator) -> u16;
        fn set_blend_array_grow_by(obj: Pin<&mut NiBlendInterpolator>, value: u16);
        fn blend_weight_threshold(obj: &NiBlendInterpolator) -> f32;
        fn set_blend_weight_threshold(obj: Pin<&mut NiBlendInterpolator>, value: f32);
        fn blend_interp_count(obj: &NiBlendInterpolator) -> u16;
        fn set_blend_interp_count(obj: Pin<&mut NiBlendInterpolator>, value: u16);
        fn blend_single_index(obj: &NiBlendInterpolator) -> u8;
        fn set_blend_single_index(obj: Pin<&mut NiBlendInterpolator>, value: u8);
        fn blend_single_index_16(obj: &NiBlendInterpolator) -> u16;
        fn set_blend_single_index_16(obj: Pin<&mut NiBlendInterpolator>, value: u16);
        fn blend_high_priority(obj: &NiBlendInterpolator) -> i8;
        fn set_blend_high_priority(obj: Pin<&mut NiBlendInterpolator>, value: i8);
        fn blend_high_priority_32(obj: &NiBlendInterpolator) -> i32;
        fn set_blend_high_priority_32(obj: Pin<&mut NiBlendInterpolator>, value: i32);
        fn blend_next_high_priority(obj: &NiBlendInterpolator) -> i8;
        fn set_blend_next_high_priority(obj: Pin<&mut NiBlendInterpolator>, value: i8);
        fn blend_next_high_priority_32(obj: &NiBlendInterpolator) -> i32;
        fn set_blend_next_high_priority_32(obj: Pin<&mut NiBlendInterpolator>, value: i32);
        fn blend_single_time(obj: &NiBlendInterpolator) -> f32;
        fn set_blend_single_time(obj: Pin<&mut NiBlendInterpolator>, value: f32);
        fn blend_high_weights_sum(obj: &NiBlendInterpolator) -> f32;
        fn set_blend_high_weights_sum(obj: Pin<&mut NiBlendInterpolator>, value: f32);
        fn blend_next_high_weights_sum(obj: &NiBlendInterpolator) -> f32;
        fn set_blend_next_high_weights_sum(obj: Pin<&mut NiBlendInterpolator>, value: f32);
        fn blend_high_ease_spinner(obj: &NiBlendInterpolator) -> f32;
        fn set_blend_high_ease_spinner(obj: Pin<&mut NiBlendInterpolator>, value: f32);
        fn blend_manager_controlled(obj: &NiBlendInterpolator) -> bool;
        fn set_blend_manager_controlled(obj: Pin<&mut NiBlendInterpolator>, value: bool);
        fn blend_only_use_highest_weight(obj: &NiBlendInterpolator) -> bool;
        fn set_blend_only_use_highest_weight(obj: Pin<&mut NiBlendInterpolator>, value: bool);
        fn look_at_flags(obj: &NiLookAtInterpolator) -> u16;
        fn set_look_at_flags(obj: Pin<&mut NiLookAtInterpolator>, value: u16);
        fn bspline_start_time(obj: &NiBSplineInterpolator) -> f32;
        fn set_bspline_start_time(obj: Pin<&mut NiBSplineInterpolator>, value: f32);
        fn bspline_stop_time(obj: &NiBSplineInterpolator) -> f32;
        fn set_bspline_stop_time(obj: Pin<&mut NiBSplineInterpolator>, value: f32);
        fn bspline_float_base(obj: &NiBSplineCompFloatInterpolator) -> f32;
        fn set_bspline_float_base(obj: Pin<&mut NiBSplineCompFloatInterpolator>, value: f32);
        fn bspline_float_offset(obj: &NiBSplineCompFloatInterpolator) -> u32;
        fn set_bspline_float_offset(obj: Pin<&mut NiBSplineCompFloatInterpolator>, value: u32);
        fn bspline_float_bias(obj: &NiBSplineCompFloatInterpolator) -> f32;
        fn set_bspline_float_bias(obj: Pin<&mut NiBSplineCompFloatInterpolator>, value: f32);
        fn bspline_float_multiplier(obj: &NiBSplineCompFloatInterpolator) -> f32;
        fn set_bspline_float_multiplier(obj: Pin<&mut NiBSplineCompFloatInterpolator>, value: f32);
        fn bspline_point3_handle(obj: &NiBSplinePoint3Interpolator) -> u32;
        fn set_bspline_point3_handle(obj: Pin<&mut NiBSplinePoint3Interpolator>, value: u32);
        fn bspline_point3_offset(obj: &NiBSplineCompPoint3Interpolator) -> f32;
        fn set_bspline_point3_offset(obj: Pin<&mut NiBSplineCompPoint3Interpolator>, value: f32);
        fn bspline_point3_half_range(obj: &NiBSplineCompPoint3Interpolator) -> f32;
        fn set_bspline_point3_half_range(obj: Pin<&mut NiBSplineCompPoint3Interpolator>, value: f32);
        fn bspline_translation_offset(obj: &NiBSplineTransformInterpolator) -> u32;
        fn set_bspline_translation_offset(obj: Pin<&mut NiBSplineTransformInterpolator>, value: u32);
        fn bspline_rotation_offset(obj: &NiBSplineTransformInterpolator) -> u32;
        fn set_bspline_rotation_offset(obj: Pin<&mut NiBSplineTransformInterpolator>, value: u32);
        fn bspline_scale_offset(obj: &NiBSplineTransformInterpolator) -> u32;
        fn set_bspline_scale_offset(obj: Pin<&mut NiBSplineTransformInterpolator>, value: u32);
        fn bspline_translation_bias(obj: &NiBSplineCompTransformInterpolator) -> f32;
        fn set_bspline_translation_bias(obj: Pin<&mut NiBSplineCompTransformInterpolator>, value: f32);
        fn bspline_translation_multiplier(obj: &NiBSplineCompTransformInterpolator) -> f32;
        fn set_bspline_translation_multiplier(obj: Pin<&mut NiBSplineCompTransformInterpolator>, value: f32);
        fn bspline_rotation_bias(obj: &NiBSplineCompTransformInterpolator) -> f32;
        fn set_bspline_rotation_bias(obj: Pin<&mut NiBSplineCompTransformInterpolator>, value: f32);
        fn bspline_rotation_multiplier(obj: &NiBSplineCompTransformInterpolator) -> f32;
        fn set_bspline_rotation_multiplier(obj: Pin<&mut NiBSplineCompTransformInterpolator>, value: f32);
        fn bspline_scale_bias(obj: &NiBSplineCompTransformInterpolator) -> f32;
        fn set_bspline_scale_bias(obj: Pin<&mut NiBSplineCompTransformInterpolator>, value: f32);
        fn bspline_scale_multiplier(obj: &NiBSplineCompTransformInterpolator) -> f32;
        fn set_bspline_scale_multiplier(obj: Pin<&mut NiBSplineCompTransformInterpolator>, value: f32);
        fn point3_interpolator_value(obj: &NiPoint3Interpolator) -> Vec3;
        fn set_point3_interpolator_value(obj: Pin<&mut NiPoint3Interpolator>, value: Vec3);
        fn blend_point3_value(obj: &NiBlendPoint3Interpolator) -> Vec3;
        fn set_blend_point3_value(obj: Pin<&mut NiBlendPoint3Interpolator>, value: Vec3);
        fn bspline_point3_value(obj: &NiBSplinePoint3Interpolator) -> Vec3;
        fn set_bspline_point3_value(obj: Pin<&mut NiBSplinePoint3Interpolator>, value: Vec3);
        fn transform_interpolator_value(obj: &NiTransformInterpolator) -> QuatTransformFfi;
        fn set_transform_interpolator_value(obj: Pin<&mut NiTransformInterpolator>, value: &QuatTransformFfi);
        fn bspline_transform_value(obj: &NiBSplineTransformInterpolator) -> QuatTransformFfi;
        fn set_bspline_transform_value(obj: Pin<&mut NiBSplineTransformInterpolator>, value: &QuatTransformFfi);
        fn path_flags(obj: &NiPathInterpolator) -> u16;
        fn set_path_flags(obj: Pin<&mut NiPathInterpolator>, value: u16);
        fn path_bank_direction(obj: &NiPathInterpolator) -> i32;
        fn set_path_bank_direction(obj: Pin<&mut NiPathInterpolator>, value: i32);
        fn path_max_bank_angle(obj: &NiPathInterpolator) -> f32;
        fn set_path_max_bank_angle(obj: Pin<&mut NiPathInterpolator>, value: f32);
        fn path_smoothing(obj: &NiPathInterpolator) -> f32;
        fn set_path_smoothing(obj: Pin<&mut NiPathInterpolator>, value: f32);
        fn path_follow_axis(obj: &NiPathInterpolator) -> u16;
        fn set_path_follow_axis(obj: Pin<&mut NiPathInterpolator>, value: u16);
        fn blend_transform_value(obj: &NiBlendTransformInterpolator) -> QuatTransformFfi;
        fn set_blend_transform_value(obj: Pin<&mut NiBlendTransformInterpolator>, value: &QuatTransformFfi);
        fn blend_transform_valid(obj: &NiBlendTransformInterpolator) -> [bool; 3];
        fn set_blend_transform_valid(obj: Pin<&mut NiBlendTransformInterpolator>, valid: [bool; 3]);
        fn blend_items(obj: &NiBlendInterpolator) -> Vec<BlendItemFfi>;
        /// Replaces the items of the blend interpolator at `owner`. False, changing nothing,
        /// unless each interpolator is empty or an `NiInterpolator` that doesn't use the
        /// owner, and the version can store the count. Where the items are saved, sets
        /// `arraySize` to their count.
        fn set_blend_items(file: Pin<&mut NifFile>, owner: u32, items: &[BlendItemFfi]) -> bool;
        fn look_at_name(obj: &NiLookAtInterpolator) -> &[u8];
        fn set_look_at_name(obj: Pin<&mut NiLookAtInterpolator>, name: &[u8]);
        fn look_at_transform(obj: &NiLookAtInterpolator) -> QuatTransformFfi;
        fn set_look_at_transform(obj: Pin<&mut NiLookAtInterpolator>, value: &QuatTransformFfi);
        fn look_at_transform_valid(obj: &NiLookAtInterpolator) -> [bool; 3];
        fn set_look_at_transform_valid(obj: Pin<&mut NiLookAtInterpolator>, valid: [bool; 3]);
        fn treads(obj: &BSTreadTransfInterpolator) -> Vec<TreadFfi>;
        fn set_treads(obj: Pin<&mut BSTreadTransfInterpolator>, treads: &[TreadFfi]);
        fn add_ni_bool_interpolator(file: Pin<&mut NifFile>) -> u32;
        fn add_ni_bool_timeline_interpolator(file: Pin<&mut NifFile>) -> u32;
        fn add_ni_float_interpolator(file: Pin<&mut NifFile>) -> u32;
        fn add_ni_transform_interpolator(file: Pin<&mut NifFile>) -> u32;
        fn add_bs_rot_accum_transf_interpolator(file: Pin<&mut NifFile>) -> u32;
        fn add_ni_point3_interpolator(file: Pin<&mut NifFile>) -> u32;
        fn add_ni_path_interpolator(file: Pin<&mut NifFile>) -> u32;
        fn add_ni_look_at_interpolator(file: Pin<&mut NifFile>) -> u32;
        fn add_bs_tread_transf_interpolator(file: Pin<&mut NifFile>) -> u32;
        fn add_ni_bspline_comp_float_interpolator(file: Pin<&mut NifFile>) -> u32;
        fn add_ni_bspline_comp_point3_interpolator(file: Pin<&mut NifFile>) -> u32;
        fn add_ni_bspline_transform_interpolator(file: Pin<&mut NifFile>) -> u32;
        fn add_ni_bspline_comp_transform_interpolator(file: Pin<&mut NifFile>) -> u32;
        fn add_ni_blend_bool_interpolator(file: Pin<&mut NifFile>) -> u32;
        fn add_ni_blend_float_interpolator(file: Pin<&mut NifFile>) -> u32;
        fn add_ni_blend_point3_interpolator(file: Pin<&mut NifFile>) -> u32;
        fn add_ni_blend_transform_interpolator(file: Pin<&mut NifFile>) -> u32;
        fn controller_flags(obj: &NiTimeController) -> u16;
        fn set_controller_flags(obj: Pin<&mut NiTimeController>, value: u16);
        fn controller_frequency(obj: &NiTimeController) -> f32;
        fn set_controller_frequency(obj: Pin<&mut NiTimeController>, value: f32);
        fn controller_phase(obj: &NiTimeController) -> f32;
        fn set_controller_phase(obj: Pin<&mut NiTimeController>, value: f32);
        fn controller_start_time(obj: &NiTimeController) -> f32;
        fn set_controller_start_time(obj: Pin<&mut NiTimeController>, value: f32);
        fn controller_stop_time(obj: &NiTimeController) -> f32;
        fn set_controller_stop_time(obj: Pin<&mut NiTimeController>, value: f32);
        fn interp_controller_manager_controlled(obj: &NiInterpController) -> bool;
        fn set_interp_controller_manager_controlled(obj: Pin<&mut NiInterpController>, value: bool);
        fn look_at_controller_flags(obj: &NiLookAtController) -> u16;
        fn set_look_at_controller_flags(obj: Pin<&mut NiLookAtController>, value: u16);
        fn path_controller_flags(obj: &NiPathController) -> u16;
        fn set_path_controller_flags(obj: Pin<&mut NiPathController>, value: u16);
        fn path_controller_bank_direction(obj: &NiPathController) -> i32;
        fn set_path_controller_bank_direction(obj: Pin<&mut NiPathController>, value: i32);
        fn path_controller_max_bank_angle(obj: &NiPathController) -> f32;
        fn set_path_controller_max_bank_angle(obj: Pin<&mut NiPathController>, value: f32);
        fn path_controller_smoothing(obj: &NiPathController) -> f32;
        fn set_path_controller_smoothing(obj: Pin<&mut NiPathController>, value: f32);
        fn path_controller_follow_axis(obj: &NiPathController) -> u16;
        fn set_path_controller_follow_axis(obj: Pin<&mut NiPathController>, value: u16);
        fn uv_controller_uv_set(obj: &NiUVController) -> u16;
        fn set_uv_controller_uv_set(obj: Pin<&mut NiUVController>, value: u16);
        fn lag_bone_linear_velocity(obj: &BSLagBoneController) -> f32;
        fn set_lag_bone_linear_velocity(obj: Pin<&mut BSLagBoneController>, value: f32);
        fn lag_bone_linear_rotation(obj: &BSLagBoneController) -> f32;
        fn set_lag_bone_linear_rotation(obj: Pin<&mut BSLagBoneController>, value: f32);
        fn lag_bone_max_distance(obj: &BSLagBoneController) -> f32;
        fn set_lag_bone_max_distance(obj: Pin<&mut BSLagBoneController>, value: f32);
        fn lightning_subdivisions(obj: &BSProceduralLightningController) -> u16;
        fn set_lightning_subdivisions(obj: Pin<&mut BSProceduralLightningController>, value: u16);
        fn lightning_branch_count(obj: &BSProceduralLightningController) -> u16;
        fn set_lightning_branch_count(obj: Pin<&mut BSProceduralLightningController>, value: u16);
        fn lightning_branch_variation(obj: &BSProceduralLightningController) -> u16;
        fn set_lightning_branch_variation(obj: Pin<&mut BSProceduralLightningController>, value: u16);
        fn lightning_length(obj: &BSProceduralLightningController) -> f32;
        fn set_lightning_length(obj: Pin<&mut BSProceduralLightningController>, value: f32);
        fn lightning_length_variation(obj: &BSProceduralLightningController) -> f32;
        fn set_lightning_length_variation(obj: Pin<&mut BSProceduralLightningController>, value: f32);
        fn lightning_width(obj: &BSProceduralLightningController) -> f32;
        fn set_lightning_width(obj: Pin<&mut BSProceduralLightningController>, value: f32);
        fn lightning_child_width_multiplier(obj: &BSProceduralLightningController) -> f32;
        fn set_lightning_child_width_multiplier(obj: Pin<&mut BSProceduralLightningController>, value: f32);
        fn lightning_arc_offset(obj: &BSProceduralLightningController) -> f32;
        fn set_lightning_arc_offset(obj: Pin<&mut BSProceduralLightningController>, value: f32);
        fn lightning_fade_main_bolt(obj: &BSProceduralLightningController) -> bool;
        fn set_lightning_fade_main_bolt(obj: Pin<&mut BSProceduralLightningController>, value: bool);
        fn lightning_fade_child_bolts(obj: &BSProceduralLightningController) -> bool;
        fn set_lightning_fade_child_bolts(obj: Pin<&mut BSProceduralLightningController>, value: bool);
        fn lightning_animate_arc_offset(obj: &BSProceduralLightningController) -> bool;
        fn set_lightning_animate_arc_offset(obj: Pin<&mut BSProceduralLightningController>, value: bool);
        fn point3_controller_target_color(obj: &NiPoint3InterpController) -> u16;
        fn set_point3_controller_target_color(obj: Pin<&mut NiPoint3InterpController>, value: u16);
        fn flip_texture_slot(obj: &NiFlipController) -> u32;
        fn set_flip_texture_slot(obj: Pin<&mut NiFlipController>, value: u32);
        fn flip_accum_time(obj: &NiFlipController) -> f32;
        fn set_flip_accum_time(obj: Pin<&mut NiFlipController>, value: f32);
        fn flip_delta(obj: &NiFlipController) -> f32;
        fn set_flip_delta(obj: Pin<&mut NiFlipController>, value: f32);
        fn texture_transform_shader_map(obj: &NiTextureTransformController) -> bool;
        fn set_texture_transform_shader_map(obj: Pin<&mut NiTextureTransformController>, value: bool);
        fn texture_transform_slot(obj: &NiTextureTransformController) -> u32;
        fn set_texture_transform_slot(obj: Pin<&mut NiTextureTransformController>, value: u32);
        fn texture_transform_operation(obj: &NiTextureTransformController) -> u32;
        fn set_texture_transform_operation(obj: Pin<&mut NiTextureTransformController>, value: u32);
        fn lighting_color_controller_color(obj: &BSLightingShaderPropertyColorController) -> u32;
        fn set_lighting_color_controller_color(obj: Pin<&mut BSLightingShaderPropertyColorController>, value: u32);
        fn lighting_float_controller_variable(obj: &BSLightingShaderPropertyFloatController) -> u32;
        fn set_lighting_float_controller_variable(obj: Pin<&mut BSLightingShaderPropertyFloatController>, value: u32);
        fn lighting_ushort_controller_variable(obj: &BSLightingShaderPropertyUShortController) -> u32;
        fn set_lighting_ushort_controller_variable(obj: Pin<&mut BSLightingShaderPropertyUShortController>, value: u32);
        fn effect_color_controller_color(obj: &BSEffectShaderPropertyColorController) -> u32;
        fn set_effect_color_controller_color(obj: Pin<&mut BSEffectShaderPropertyColorController>, value: u32);
        fn effect_float_controller_variable(obj: &BSEffectShaderPropertyFloatController) -> u32;
        fn set_effect_float_controller_variable(obj: Pin<&mut BSEffectShaderPropertyFloatController>, value: u32);
        fn geom_morpher_flags(obj: &NiGeomMorpherController) -> u16;
        fn set_geom_morpher_flags(obj: Pin<&mut NiGeomMorpherController>, value: u16);
        fn geom_morpher_always_update(obj: &NiGeomMorpherController) -> bool;
        fn set_geom_morpher_always_update(obj: Pin<&mut NiGeomMorpherController>, value: bool);
        fn bone_lod_lod(obj: &NiBoneLODController) -> u32;
        fn set_bone_lod_lod(obj: Pin<&mut NiBoneLODController>, value: u32);
        fn blend_controller_keys(obj: &bhkBlendController) -> u32;
        fn set_blend_controller_keys(obj: Pin<&mut bhkBlendController>, value: u32);
        fn bone_lod_controller_count(obj: &NiBoneLODController) -> u32;
        fn float_extra_data_controller_name(obj: &NiFloatExtraDataController) -> &[u8];
        fn set_float_extra_data_controller_name(obj: Pin<&mut NiFloatExtraDataController>, name: &[u8]);
        fn geom_morpher_unknown_ints(obj: &NiGeomMorpherController) -> Vec<u32>;
        fn set_geom_morpher_unknown_ints(obj: Pin<&mut NiGeomMorpherController>, ints: &[u32]);
        /// The interpolators of the `NiGeomMorpherController` at `owner`, from the list the file
        /// version saves, with weights kept in memory before 20.1.0.3 (or 0).
        fn morph_weights(file: &NifFile, owner: u32) -> Vec<MorphWeightFfi>;
        /// Replaces both interpolator lists. False, changing nothing, unless each interpolator
        /// is empty or an `NiInterpolator`.
        fn set_morph_weights(file: Pin<&mut NifFile>, owner: u32, weights: &[MorphWeightFfi]) -> bool;
        fn bone_lod_groups(obj: &NiBoneLODController) -> Vec<NodeGroupFfi>;
        /// Replaces the node groups and sets the LOD count to their number. False, changing
        /// nothing, unless each node is an `NiNode`.
        fn set_bone_lod_groups(file: Pin<&mut NifFile>, owner: u32, groups: &[NodeGroupFfi]) -> bool;
        /// Adds a controller to the end of the chain of the `NiObjectNET` at `owner`, targeting
        /// it. `NIF_NPOS`, adding nothing, if `owner` isn't one.
        fn add_ni_look_at_controller(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_path_controller(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_uv_controller(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_bs_frustum_fov_controller(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_bs_lag_bone_controller(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_bs_procedural_lightning_controller(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_bone_lod_controller(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_bs_bone_lod_controller(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_geom_morpher_controller(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_roll_controller(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_material_color_controller(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_light_color_controller(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_float_extra_data_controller(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_bhk_blend_controller(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_vis_controller(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_flip_controller(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_texture_transform_controller(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_light_dimmer_controller(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_light_radius_controller(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_alpha_controller(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_bs_ni_alpha_property_test_ref_controller(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_keyframe_controller(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_transform_controller(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_bs_material_emittance_mult_controller(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_bs_refraction_strength_controller(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_bs_refraction_fire_period_controller(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_bs_lighting_shader_property_color_controller(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_bs_lighting_shader_property_float_controller(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_bs_lighting_shader_property_u_short_controller(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_bs_effect_shader_property_color_controller(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_bs_effect_shader_property_float_controller(file: Pin<&mut NifFile>, owner: u32) -> u32;
        fn add_ni_multi_target_transform_controller(file: Pin<&mut NifFile>, owner: u32) -> u32;
    }
}
