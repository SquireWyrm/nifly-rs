//! Animation: keys, animation data, interpolators, controllers and sequences.
//!
//! Data blocks hold [`KeyGroup`]s: keys of a value over time, and how values between them are
//! interpolated. Interpolators read data blocks, and controllers use interpolators to animate
//! their target.

use std::pin::Pin;

use bstr::{BStr, BString, ByteSlice};
use int_enum::IntEnum;
use nifly_sys::animation::ffi;

use crate::{
    file::{index_from_ffi, optional_index_to_ffi, Index, NifFile},
    handle::{Edit, View},
    nodes::NiNode,
    object3d::Vec3,
    objects::{IsA, NiObject, NiType, Sealed},
};
use nifly_sys::nif_file::ffi::RefKind;

ni_types! { ffi;
    /// Keyframes of a transform: rotation, translation and scale keys.
    NiKeyframeData(NiKeyframeData): NiObject => upcast_ni_keyframe_data, downcast_ni_keyframe_data;
    /// The keyframes of an [`NiTransformInterpolator`].
    NiTransformData(NiTransformData): NiKeyframeData => upcast_ni_transform_data, downcast_ni_transform_data;
    /// Position (3D vector) keys.
    NiPosData(NiPosData): NiObject => upcast_ni_pos_data, downcast_ni_pos_data;
    /// Boolean keys.
    NiBoolData(NiBoolData): NiObject => upcast_ni_bool_data, downcast_ni_bool_data;
    /// Float keys.
    NiFloatData(NiFloatData): NiObject => upcast_ni_float_data, downcast_ni_float_data;
    /// Color keys.
    NiColorData(NiColorData): NiObject => upcast_ni_color_data, downcast_ni_color_data;
    /// Texture coordinate offset and scale keys, for an [`NiUvController`].
    ///
    /// The `NiUVData` block.
    NiUvData(NiUVData): NiObject => upcast_ni_uv_data, downcast_ni_uv_data;
    /// Visibility keys, for an [`NiVisController`].
    NiVisData(NiVisData): NiObject => upcast_ni_vis_data, downcast_ni_vis_data;
    /// Morph targets for vertex morphing, for an [`NiGeomMorpherController`].
    NiMorphData(NiMorphData): NiObject => upcast_ni_morph_data, downcast_ni_morph_data;
    /// B-spline control points, shared by B-spline interpolators.
    NiBSplineData(NiBSplineData): NiObject => upcast_ni_bspline_data, downcast_ni_bspline_data;
    /// The number of control points of a B-spline.
    NiBSplineBasisData(NiBSplineBasisData): NiObject => upcast_ni_bspline_basis_data, downcast_ni_bspline_basis_data;
    /// Strings packed into one buffer, referred to by offset.
    NiStringPalette(NiStringPalette): NiObject => upcast_ni_string_palette, downcast_ni_string_palette;
    /// Base class of interpolators, which compute a value over time for a controller.
    NiInterpolator(NiInterpolator): NiObject => upcast_ni_interpolator, downcast_ni_interpolator;
    /// Base class of B-spline interpolators.
    NiBSplineInterpolator(NiBSplineInterpolator): NiInterpolator => upcast_ni_bspline_interpolator, downcast_ni_bspline_interpolator;
    /// Base class of B-spline float interpolators.
    NiBSplineFloatInterpolator(NiBSplineFloatInterpolator): NiBSplineInterpolator => upcast_ni_bspline_float_interpolator, downcast_ni_bspline_float_interpolator;
    /// A B-spline float interpolator with compressed control points.
    NiBSplineCompFloatInterpolator(NiBSplineCompFloatInterpolator): NiBSplineFloatInterpolator => upcast_ni_bspline_comp_float_interpolator, downcast_ni_bspline_comp_float_interpolator;
    /// Base class of B-spline 3D vector interpolators.
    NiBSplinePoint3Interpolator(NiBSplinePoint3Interpolator): NiBSplineInterpolator => upcast_ni_bspline_point3_interpolator, downcast_ni_bspline_point3_interpolator;
    /// A B-spline 3D vector interpolator with compressed control points.
    NiBSplineCompPoint3Interpolator(NiBSplineCompPoint3Interpolator): NiBSplinePoint3Interpolator => upcast_ni_bspline_comp_point3_interpolator, downcast_ni_bspline_comp_point3_interpolator;
    /// A B-spline transform interpolator.
    NiBSplineTransformInterpolator(NiBSplineTransformInterpolator): NiBSplineInterpolator => upcast_ni_bspline_transform_interpolator, downcast_ni_bspline_transform_interpolator;
    /// A B-spline transform interpolator with compressed control points.
    NiBSplineCompTransformInterpolator(NiBSplineCompTransformInterpolator): NiBSplineTransformInterpolator => upcast_ni_bspline_comp_transform_interpolator, downcast_ni_bspline_comp_transform_interpolator;
    /// Base class of interpolators that blend other interpolators.
    NiBlendInterpolator(NiBlendInterpolator): NiInterpolator => upcast_ni_blend_interpolator, downcast_ni_blend_interpolator;
    /// Blends boolean interpolators.
    NiBlendBoolInterpolator(NiBlendBoolInterpolator): NiBlendInterpolator => upcast_ni_blend_bool_interpolator, downcast_ni_blend_bool_interpolator;
    /// Blends float interpolators.
    NiBlendFloatInterpolator(NiBlendFloatInterpolator): NiBlendInterpolator => upcast_ni_blend_float_interpolator, downcast_ni_blend_float_interpolator;
    /// Blends 3D vector interpolators.
    NiBlendPoint3Interpolator(NiBlendPoint3Interpolator): NiBlendInterpolator => upcast_ni_blend_point3_interpolator, downcast_ni_blend_point3_interpolator;
    /// Blends transform interpolators.
    NiBlendTransformInterpolator(NiBlendTransformInterpolator): NiBlendInterpolator => upcast_ni_blend_transform_interpolator, downcast_ni_blend_transform_interpolator;
    /// Base class of interpolators driven by keys.
    NiKeyBasedInterpolator(NiKeyBasedInterpolator): NiInterpolator => upcast_ni_key_based_interpolator, downcast_ni_key_based_interpolator;
    /// Interpolates [`NiBoolData`] keys.
    NiBoolInterpolator(NiBoolInterpolator): NiKeyBasedInterpolator => upcast_ni_bool_interpolator, downcast_ni_bool_interpolator;
    /// A boolean interpolator that doesn't skip keys.
    NiBoolTimelineInterpolator(NiBoolTimelineInterpolator): NiBoolInterpolator => upcast_ni_bool_timeline_interpolator, downcast_ni_bool_timeline_interpolator;
    /// Interpolates [`NiFloatData`] keys.
    NiFloatInterpolator(NiFloatInterpolator): NiKeyBasedInterpolator => upcast_ni_float_interpolator, downcast_ni_float_interpolator;
    /// Interpolates [`NiTransformData`] keys.
    NiTransformInterpolator(NiTransformInterpolator): NiKeyBasedInterpolator => upcast_ni_transform_interpolator, downcast_ni_transform_interpolator;
    /// A transform interpolator that accumulates rotation.
    ///
    /// The `BSRotAccumTransfInterpolator` block.
    BsRotAccumTransfInterpolator(BSRotAccumTransfInterpolator): NiTransformInterpolator => upcast_bs_rot_accum_transf_interpolator, downcast_bs_rot_accum_transf_interpolator;
    /// Interpolates [`NiPosData`] keys.
    NiPoint3Interpolator(NiPoint3Interpolator): NiKeyBasedInterpolator => upcast_ni_point3_interpolator, downcast_ni_point3_interpolator;
    /// Moves an object along a path.
    NiPathInterpolator(NiPathInterpolator): NiKeyBasedInterpolator => upcast_ni_path_interpolator, downcast_ni_path_interpolator;
    /// Turns an object to face another.
    NiLookAtInterpolator(NiLookAtInterpolator): NiInterpolator => upcast_ni_look_at_interpolator, downcast_ni_look_at_interpolator;
    /// Interpolates the transforms of treads.
    ///
    /// The `BSTreadTransfInterpolator` block.
    BsTreadTransfInterpolator(BSTreadTransfInterpolator): NiInterpolator => upcast_bs_tread_transf_interpolator, downcast_bs_tread_transf_interpolator;
    /// Base class of controllers, which animate a target block over time.
    NiTimeController(NiTimeController): NiObject => upcast_ni_time_controller, downcast_ni_time_controller;
    /// Turns its target to face another object.
    NiLookAtController(NiLookAtController): NiTimeController => upcast_ni_look_at_controller, downcast_ni_look_at_controller;
    /// Moves its target along a path.
    NiPathController(NiPathController): NiTimeController => upcast_ni_path_controller, downcast_ni_path_controller;
    /// Resets a particle system when its animation loops.
    NiPSysResetOnLoopCtlr(NiPSysResetOnLoopCtlr): NiTimeController => upcast_ni_psys_reset_on_loop_ctlr, downcast_ni_psys_reset_on_loop_ctlr;
    /// Animates texture coordinates with [`NiUvData`].
    ///
    /// The `NiUVController` block.
    NiUvController(NiUVController): NiTimeController => upcast_ni_uv_controller, downcast_ni_uv_controller;
    /// Animates a camera's field of view.
    ///
    /// The `BSFrustumFOVController` block.
    BsFrustumFovController(BSFrustumFOVController): NiTimeController => upcast_bs_frustum_fov_controller, downcast_bs_frustum_fov_controller;
    /// Makes a bone lag behind its parent's motion.
    ///
    /// The `BSLagBoneController` block.
    BsLagBoneController(BSLagBoneController): NiTimeController => upcast_bs_lag_bone_controller, downcast_bs_lag_bone_controller;
    /// Animates procedural lightning.
    ///
    /// The `BSProceduralLightningController` block.
    BsProceduralLightningController(BSProceduralLightningController): NiTimeController => upcast_bs_procedural_lightning_controller, downcast_bs_procedural_lightning_controller;
    /// Switches bone levels of detail.
    ///
    /// The `NiBoneLODController` block.
    NiBoneLodController(NiBoneLODController): NiTimeController => upcast_ni_bone_lod_controller, downcast_ni_bone_lod_controller;
    /// Bethesda's bone level-of-detail controller.
    ///
    /// The `NiBSBoneLODController` block.
    NiBsBoneLodController(NiBSBoneLODController): NiBoneLodController => upcast_ni_bs_bone_lod_controller, downcast_ni_bs_bone_lod_controller;
    /// Base class of controllers driven by interpolators.
    NiInterpController(NiInterpController): NiTimeController => upcast_ni_interp_controller, downcast_ni_interp_controller;
    /// Morphs its target's vertices with [`NiMorphData`].
    NiGeomMorpherController(NiGeomMorpherController): NiInterpController => upcast_ni_geom_morpher_controller, downcast_ni_geom_morpher_controller;
    /// Base class of controllers with one interpolator.
    NiSingleInterpController(NiSingleInterpController): NiInterpController => upcast_ni_single_interp_controller, downcast_ni_single_interp_controller;
    /// Rolls its target around its forward axis.
    NiRollController(NiRollController): NiSingleInterpController => upcast_ni_roll_controller, downcast_ni_roll_controller;
    /// Base class of controllers driven by a 3D vector interpolator.
    NiPoint3InterpController(NiPoint3InterpController): NiSingleInterpController => upcast_ni_point3_interp_controller, downcast_ni_point3_interp_controller;
    /// Animates a material color.
    NiMaterialColorController(NiMaterialColorController): NiPoint3InterpController => upcast_ni_material_color_controller, downcast_ni_material_color_controller;
    /// Animates a light color.
    NiLightColorController(NiLightColorController): NiPoint3InterpController => upcast_ni_light_color_controller, downcast_ni_light_color_controller;
    /// Base class of controllers that animate extra data.
    NiExtraDataController(NiExtraDataController): NiSingleInterpController => upcast_ni_extra_data_controller, downcast_ni_extra_data_controller;
    /// Animates an [`NiFloatExtraData`](crate::extra_data::NiFloatExtraData) value.
    NiFloatExtraDataController(NiFloatExtraDataController): NiExtraDataController => upcast_ni_float_extra_data_controller, downcast_ni_float_extra_data_controller;
    /// Base class of controllers driven by a boolean interpolator.
    NiBoolInterpController(NiBoolInterpController): NiSingleInterpController => upcast_ni_bool_interp_controller, downcast_ni_bool_interp_controller;
    /// Animates its target's visibility.
    NiVisController(NiVisController): NiBoolInterpController => upcast_ni_vis_controller, downcast_ni_vis_controller;
    /// Base class of controllers driven by a float interpolator.
    NiFloatInterpController(NiFloatInterpController): NiSingleInterpController => upcast_ni_float_interp_controller, downcast_ni_float_interp_controller;
    /// Animates a shader's fire refraction period.
    ///
    /// The `BSRefractionFirePeriodController` block.
    BsRefractionFirePeriodController(BSRefractionFirePeriodController): NiSingleInterpController => upcast_bs_refraction_fire_period_controller, downcast_bs_refraction_fire_period_controller;
    /// Flips through a list of textures.
    NiFlipController(NiFlipController): NiFloatInterpController => upcast_ni_flip_controller, downcast_ni_flip_controller;
    /// Animates a texture transform.
    NiTextureTransformController(NiTextureTransformController): NiFloatInterpController => upcast_ni_texture_transform_controller, downcast_ni_texture_transform_controller;
    /// Animates a light's dimmer.
    NiLightDimmerController(NiLightDimmerController): NiFloatInterpController => upcast_ni_light_dimmer_controller, downcast_ni_light_dimmer_controller;
    /// Animates a light's radius.
    NiLightRadiusController(NiLightRadiusController): NiFloatInterpController => upcast_ni_light_radius_controller, downcast_ni_light_radius_controller;
    /// Animates a material's alpha.
    NiAlphaController(NiAlphaController): NiFloatInterpController => upcast_ni_alpha_controller, downcast_ni_alpha_controller;
    /// Updates a particle system each frame.
    NiPSysUpdateCtlr(NiPSysUpdateCtlr): NiTimeController => upcast_ni_psys_update_ctlr, downcast_ni_psys_update_ctlr;
    /// Animates an alpha property's test threshold.
    ///
    /// The `BSNiAlphaPropertyTestRefController` block.
    BsNiAlphaPropertyTestRefController(BSNiAlphaPropertyTestRefController): NiAlphaController => upcast_bs_ni_alpha_property_test_ref_controller, downcast_bs_ni_alpha_property_test_ref_controller;
    /// Animates its target's transform with [`NiKeyframeData`].
    NiKeyframeController(NiKeyframeController): NiSingleInterpController => upcast_ni_keyframe_controller, downcast_ni_keyframe_controller;
    /// Animates its target's transform with an interpolator.
    NiTransformController(NiTransformController): NiKeyframeController => upcast_ni_transform_controller, downcast_ni_transform_controller;
    /// Animates a shader's emittance multiplier.
    ///
    /// The `BSMaterialEmittanceMultController` block.
    BsMaterialEmittanceMultController(BSMaterialEmittanceMultController): NiFloatInterpController => upcast_bs_material_emittance_mult_controller, downcast_bs_material_emittance_mult_controller;
    /// Animates a shader's refraction strength.
    ///
    /// The `BSRefractionStrengthController` block.
    BsRefractionStrengthController(BSRefractionStrengthController): NiFloatInterpController => upcast_bs_refraction_strength_controller, downcast_bs_refraction_strength_controller;
    /// Animates a lighting shader color.
    ///
    /// The `BSLightingShaderPropertyColorController` block.
    BsLightingShaderPropertyColorController(BSLightingShaderPropertyColorController): NiFloatInterpController => upcast_bs_lighting_shader_property_color_controller, downcast_bs_lighting_shader_property_color_controller;
    /// Animates a lighting shader value.
    ///
    /// The `BSLightingShaderPropertyFloatController` block.
    BsLightingShaderPropertyFloatController(BSLightingShaderPropertyFloatController): NiFloatInterpController => upcast_bs_lighting_shader_property_float_controller, downcast_bs_lighting_shader_property_float_controller;
    /// Animates a lighting shader integer value.
    ///
    /// The `BSLightingShaderPropertyUShortController` block.
    BsLightingShaderPropertyUShortController(BSLightingShaderPropertyUShortController): NiFloatInterpController => upcast_bs_lighting_shader_property_u_short_controller, downcast_bs_lighting_shader_property_u_short_controller;
    /// Animates an effect shader color.
    ///
    /// The `BSEffectShaderPropertyColorController` block.
    BsEffectShaderPropertyColorController(BSEffectShaderPropertyColorController): NiFloatInterpController => upcast_bs_effect_shader_property_color_controller, downcast_bs_effect_shader_property_color_controller;
    /// Animates an effect shader value.
    ///
    /// The `BSEffectShaderPropertyFloatController` block.
    BsEffectShaderPropertyFloatController(BSEffectShaderPropertyFloatController): NiFloatInterpController => upcast_bs_effect_shader_property_float_controller, downcast_bs_effect_shader_property_float_controller;
    /// Animates the transforms of several objects.
    NiMultiTargetTransformController(NiMultiTargetTransformController): NiInterpController => upcast_ni_multi_target_transform_controller, downcast_ni_multi_target_transform_controller;
    /// Base class of controllers that animate a particle modifier.
    NiPSysModifierCtlr(NiPSysModifierCtlr): NiSingleInterpController => upcast_ni_psys_modifier_ctlr, downcast_ni_psys_modifier_ctlr;
    /// Base class of controllers that animate a particle modifier's switch.
    NiPSysModifierBoolCtlr(NiPSysModifierBoolCtlr): NiPSysModifierCtlr => upcast_ni_psys_modifier_bool_ctlr, downcast_ni_psys_modifier_bool_ctlr;
    /// Turns a particle modifier on and off.
    NiPSysModifierActiveCtlr(NiPSysModifierActiveCtlr): NiPSysModifierBoolCtlr => upcast_ni_psys_modifier_active_ctlr, downcast_ni_psys_modifier_active_ctlr;
    /// Base class of controllers that animate a particle modifier's value.
    NiPSysModifierFloatCtlr(NiPSysModifierFloatCtlr): NiPSysModifierCtlr => upcast_ni_psys_modifier_float_ctlr, downcast_ni_psys_modifier_float_ctlr;
    /// Animates a particle emitter's life span.
    NiPSysEmitterLifeSpanCtlr(NiPSysEmitterLifeSpanCtlr): NiPSysModifierFloatCtlr => upcast_ni_psys_emitter_life_span_ctlr, downcast_ni_psys_emitter_life_span_ctlr;
    /// Animates a particle emitter's speed.
    NiPSysEmitterSpeedCtlr(NiPSysEmitterSpeedCtlr): NiPSysModifierFloatCtlr => upcast_ni_psys_emitter_speed_ctlr, downcast_ni_psys_emitter_speed_ctlr;
    /// Animates a particle emitter's initial radius.
    NiPSysEmitterInitialRadiusCtlr(NiPSysEmitterInitialRadiusCtlr): NiPSysModifierFloatCtlr => upcast_ni_psys_emitter_initial_radius_ctlr, downcast_ni_psys_emitter_initial_radius_ctlr;
    /// Animates a particle emitter's declination.
    NiPSysEmitterDeclinationCtlr(NiPSysEmitterDeclinationCtlr): NiPSysModifierFloatCtlr => upcast_ni_psys_emitter_declination_ctlr, downcast_ni_psys_emitter_declination_ctlr;
    /// Animates a particle gravity modifier's strength.
    NiPSysGravityStrengthCtlr(NiPSysGravityStrengthCtlr): NiPSysModifierFloatCtlr => upcast_ni_psys_gravity_strength_ctlr, downcast_ni_psys_gravity_strength_ctlr;
    /// Animates a particle emitter's declination variation.
    NiPSysEmitterDeclinationVarCtlr(NiPSysEmitterDeclinationVarCtlr): NiPSysModifierFloatCtlr => upcast_ni_psys_emitter_declination_var_ctlr, downcast_ni_psys_emitter_declination_var_ctlr;
    /// Animates a particle field's magnitude.
    NiPSysFieldMagnitudeCtlr(NiPSysFieldMagnitudeCtlr): NiPSysModifierFloatCtlr => upcast_ni_psys_field_magnitude_ctlr, downcast_ni_psys_field_magnitude_ctlr;
    /// Animates a particle field's attenuation.
    NiPSysFieldAttenuationCtlr(NiPSysFieldAttenuationCtlr): NiPSysModifierFloatCtlr => upcast_ni_psys_field_attenuation_ctlr, downcast_ni_psys_field_attenuation_ctlr;
    /// Animates a particle field's maximum distance.
    NiPSysFieldMaxDistanceCtlr(NiPSysFieldMaxDistanceCtlr): NiPSysModifierFloatCtlr => upcast_ni_psys_field_max_distance_ctlr, downcast_ni_psys_field_max_distance_ctlr;
    /// Animates a particle air field's friction.
    NiPSysAirFieldAirFrictionCtlr(NiPSysAirFieldAirFrictionCtlr): NiPSysModifierFloatCtlr => upcast_ni_psys_air_field_air_friction_ctlr, downcast_ni_psys_air_field_air_friction_ctlr;
    /// Animates how much velocity a particle air field passes on.
    NiPSysAirFieldInheritVelocityCtlr(NiPSysAirFieldInheritVelocityCtlr): NiPSysModifierFloatCtlr => upcast_ni_psys_air_field_inherit_velocity_ctlr, downcast_ni_psys_air_field_inherit_velocity_ctlr;
    /// Animates a particle air field's spread.
    NiPSysAirFieldSpreadCtlr(NiPSysAirFieldSpreadCtlr): NiPSysModifierFloatCtlr => upcast_ni_psys_air_field_spread_ctlr, downcast_ni_psys_air_field_spread_ctlr;
    /// Animates particles' initial rotation speed.
    NiPSysInitialRotSpeedCtlr(NiPSysInitialRotSpeedCtlr): NiPSysModifierFloatCtlr => upcast_ni_psys_initial_rot_speed_ctlr, downcast_ni_psys_initial_rot_speed_ctlr;
    /// Animates the variation of particles' initial rotation speed.
    NiPSysInitialRotSpeedVarCtlr(NiPSysInitialRotSpeedVarCtlr): NiPSysModifierFloatCtlr => upcast_ni_psys_initial_rot_speed_var_ctlr, downcast_ni_psys_initial_rot_speed_var_ctlr;
    /// Animates particles' initial rotation angle.
    NiPSysInitialRotAngleCtlr(NiPSysInitialRotAngleCtlr): NiPSysModifierFloatCtlr => upcast_ni_psys_initial_rot_angle_ctlr, downcast_ni_psys_initial_rot_angle_ctlr;
    /// Animates the variation of particles' initial rotation angle.
    NiPSysInitialRotAngleVarCtlr(NiPSysInitialRotAngleVarCtlr): NiPSysModifierFloatCtlr => upcast_ni_psys_initial_rot_angle_var_ctlr, downcast_ni_psys_initial_rot_angle_var_ctlr;
    /// Animates a particle emitter's planar angle.
    NiPSysEmitterPlanarAngleCtlr(NiPSysEmitterPlanarAngleCtlr): NiPSysModifierFloatCtlr => upcast_ni_psys_emitter_planar_angle_ctlr, downcast_ni_psys_emitter_planar_angle_ctlr;
    /// Animates the variation of a particle emitter's planar angle.
    NiPSysEmitterPlanarAngleVarCtlr(NiPSysEmitterPlanarAngleVarCtlr): NiPSysModifierFloatCtlr => upcast_ni_psys_emitter_planar_angle_var_ctlr, downcast_ni_psys_emitter_planar_angle_var_ctlr;
    /// Animates particles' rotation dampening.
    NiPSysRotDampeningCtlr(NiPSysRotDampeningCtlr): NiPSysModifierFloatCtlr => upcast_ni_psys_rot_dampening_ctlr, downcast_ni_psys_rot_dampening_ctlr;
    /// A named animation: controlled blocks and their interpolators.
    NiSequence(NiSequence): NiObject => upcast_ni_sequence, downcast_ni_sequence;
    /// A note in an animation, such as a grab or look-at event.
    ///
    /// The `BSAnimNote` block.
    BsAnimNote(BSAnimNote): NiObject => upcast_bs_anim_note, downcast_bs_anim_note;
    /// A list of [`BsAnimNote`]s.
    ///
    /// The `BSAnimNotes` block.
    BsAnimNotes(BSAnimNotes): NiObject => upcast_bs_anim_notes, downcast_bs_anim_notes;
    /// An animation sequence played by an [`NiControllerManager`].
    NiControllerSequence(NiControllerSequence): NiSequence => upcast_ni_controller_sequence, downcast_ni_controller_sequence;
    /// Plays animation sequences on an object tree.
    NiControllerManager(NiControllerManager): NiTimeController => upcast_ni_controller_manager, downcast_ni_controller_manager;
}

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
        Self { time, value, forward: T::default(), backward: T::default(), tbc: Tbc::default() }
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
        Self { interpolation: None, keys: Vec::new() }
    }
}

impl<T> KeyGroup<T> {
    pub fn new(interpolation: Interpolation, keys: Vec<Key<T>>) -> Self {
        Self { interpolation: Some(interpolation), keys }
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

fn key_from_ffi<T: KeyValue>(k: &ffi::KeyFfi) -> Key<T> {
    let [tension, bias, continuity] = k.tbc;
    Key {
        time: k.time,
        value: T::from_floats(k.value),
        forward: T::from_floats(k.forward),
        backward: T::from_floats(k.backward),
        tbc: Tbc { tension, bias, continuity },
    }
}

fn key_to_ffi<T: KeyValue>(k: &Key<T>) -> ffi::KeyFfi {
    ffi::KeyFfi {
        time: k.time,
        value: k.value.to_floats(),
        forward: k.forward.to_floats(),
        backward: k.backward.to_floats(),
        tbc: [k.tbc.tension, k.tbc.bias, k.tbc.continuity],
    }
}

fn group_from_ffi<T: KeyValue>(g: ffi::KeyGroupFfi) -> KeyGroup<T> {
    KeyGroup {
        interpolation: Interpolation::try_from(g.interpolation).ok(),
        keys: g.keys.iter().map(key_from_ffi).collect(),
    }
}

fn group_to_ffi<T: KeyValue>(g: &KeyGroup<T>) -> ffi::KeyGroupFfi {
    ffi::KeyGroupFfi {
        interpolation: g.interpolation.map_or(0, u32::from),
        keys: g.keys.iter().map(key_to_ffi).collect(),
    }
}

/// Getter and setter pairs for key groups.
macro_rules! key_groups {
    ($($(#[$doc:meta])* $get:ident, $set:ident: $ty:ty = $ffi_get:ident, $ffi_set:ident;)*) => {$(
        $(#[$doc])*
        pub fn $get(&self) -> KeyGroup<$ty> {
            group_from_ffi(ffi::$ffi_get(&self.0))
        }

        #[doc = concat!(
            "Replaces [`", stringify!($get), "`](Self::", stringify!($get), "). Returns `false`, ",
            "changing nothing, if the group has keys but no interpolation."
        )]
        pub fn $set(self: Pin<&mut Self>, keys: &KeyGroup<$ty>) -> bool {
            ffi::$ffi_set(self.ffi_mut(), &group_to_ffi(keys))
        }
    )*};
}

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
    pub fn set_rotations(self: Pin<&mut Self>, rotations: &Rotations) -> bool {
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
    pub fn set_u_translations(self: Pin<&mut Self>, keys: &KeyGroup<f32>) -> bool {
        ffi::set_uv_data_keys(self.ffi_mut(), 0, &group_to_ffi(keys))
    }

    pub fn set_v_translations(self: Pin<&mut Self>, keys: &KeyGroup<f32>) -> bool {
        ffi::set_uv_data_keys(self.ffi_mut(), 1, &group_to_ffi(keys))
    }

    pub fn set_u_scales(self: Pin<&mut Self>, keys: &KeyGroup<f32>) -> bool {
        ffi::set_uv_data_keys(self.ffi_mut(), 2, &group_to_ffi(keys))
    }

    pub fn set_v_scales(self: Pin<&mut Self>, keys: &KeyGroup<f32>) -> bool {
        ffi::set_uv_data_keys(self.ffi_mut(), 3, &group_to_ffi(keys))
    }
}

impl NiVisData {
    /// The visibility keys. Only each key's time and value are saved.
    pub fn keys(&self) -> Vec<Key<bool>> {
        ffi::vis_data_keys(&self.0).iter().map(key_from_ffi).collect()
    }

    pub fn set_keys(self: Pin<&mut Self>, keys: &[Key<bool>]) {
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

    pub fn set_relative_targets(self: Pin<&mut Self>, relative: bool) {
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
    pub fn set_morphs(self: Pin<&mut Self>, morphs: &[Morph]) -> bool {
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

    pub fn set_float_control_points(self: Pin<&mut Self>, points: &[f32]) {
        ffi::set_bspline_float_points(self.ffi_mut(), points);
    }

    /// The compressed control points, which compressed interpolators scale by their offsets and
    /// half-ranges.
    pub fn short_control_points(&self) -> &[i16] {
        ffi::bspline_short_points(&self.0)
    }

    pub fn set_short_control_points(self: Pin<&mut Self>, points: &[i16]) {
        ffi::set_bspline_short_points(self.ffi_mut(), points);
    }
}

impl NiBSplineBasisData {
    pub fn control_point_count(&self) -> u32 {
        ffi::bspline_basis_count(&self.0)
    }

    pub fn set_control_point_count(self: Pin<&mut Self>, count: u32) {
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
    pub fn set_palette(self: Pin<&mut Self>, palette: impl AsRef<[u8]>) {
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
}

/// # Animation
///
/// Adding animation blocks that don't belong to another block.
impl NifFile {
    /// Adds a new, empty `T` and returns its index. Nothing refers to it, so saving with
    /// [`SaveOptions::optimize`](crate::file::SaveOptions::optimize) drops it.
    ///
    /// ```
    /// # use nifly::{animation::{Interpolation, Key, KeyGroup, NiFloatData}, file::{NifFile, NifVersion}};
    /// let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    /// let data = nif.add_animation_data::<NiFloatData>();
    /// let keys = KeyGroup::new(Interpolation::Linear, vec![Key::new(0.0, 0.0), Key::new(1.0, 2.5)]);
    /// assert!(nif.block_of_mut::<NiFloatData>(data).unwrap().set_keys(&keys));
    /// ```
    pub fn add_animation_data<T: NewAnimationData>(&mut self) -> Index {
        index_from_ffi(T::add(self.inner.pin_mut())).expect("adding a block succeeds")
    }

    /// Adds a new `T` with default settings and returns its index. Nothing refers to it, so
    /// saving with [`SaveOptions::optimize`](crate::file::SaveOptions::optimize) drops it.
    pub fn add_interpolator<T: NewInterpolator>(&mut self) -> Index {
        index_from_ffi(T::add(self.inner.pin_mut())).expect("adding a block succeeds")
    }
}

/// Getter and setter pairs over the bridge's field shims, converting with `$to` and `$from`.
macro_rules! fields {
    ($($(#[$doc:meta])* $get:ident, $set:ident: $ty:ty = $ffi_get:ident, $ffi_set:ident, $to:expr, $from:expr;)*) => {$(
        $(#[$doc])*
        pub fn $get(&self) -> $ty {
            ($to)(ffi::$ffi_get(&self.0))
        }

        #[doc = concat!("Sets [`", stringify!($get), "`](Self::", stringify!($get), ").")]
        pub fn $set(self: Pin<&mut Self>, value: $ty) {
            ffi::$ffi_set(self.ffi_mut(), ($from)(value));
        }
    )*};
}

fn same<T>(x: T) -> T {
    x
}

fn vec3(v: ffi::Vec3) -> Vec3 {
    Vec3::from_ffi(v)
}

fn vec3_to_ffi(v: Vec3) -> ffi::Vec3 {
    v.to_ffi()
}

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
        Self { translation: Vec3::ZERO, rotation: [0.0, 0.0, 0.0, 1.0], scale: 1.0 }
    }
}

impl QuatTransform {
    fn from_ffi(t: ffi::QuatTransformFfi) -> Self {
        Self { translation: Vec3::from_ffi(t.translation), rotation: t.rotation, scale: t.scale }
    }

    fn to_ffi(self) -> ffi::QuatTransformFfi {
        ffi::QuatTransformFfi { translation: self.translation.to_ffi(), rotation: self.rotation, scale: self.scale }
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
        pub fn $set(self: Pin<&mut Self>, value: QuatTransform) {
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
    v.unwrap_or(Vec3::new(f32::MIN, f32::MIN, f32::MIN)).to_ffi()
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
    /// The flags of an [`NiPathInterpolator`]. Other bits are kept as stored.
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
    /// The flags of an [`NiLookAtInterpolator`]: which axis looks at the target (X when neither
    /// axis flag is set). Other bits are kept as stored.
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

    pub fn set_look_at_name(self: Pin<&mut Self>, name: impl AsRef<[u8]>) {
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

    pub fn set_treads(self: Pin<&mut Self>, treads: &[Tread]) {
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

/// An interpolator with a key data block: [`NiBoolInterpolator`] ([`NiBoolData`]),
/// [`NiFloatInterpolator`] ([`NiFloatData`]), [`NiPoint3Interpolator`] ([`NiPosData`]),
/// [`NiTransformInterpolator`] ([`NiTransformData`]), [`BsTreadTransfInterpolator`]
/// ([`NiFloatData`]), or one of their subtypes.
///
/// This trait is sealed.
pub trait HasKeyData: NiType {
    /// The data block type.
    type Data: NiType;
}

impl HasKeyData for NiBoolInterpolator {
    type Data = NiBoolData;
}
impl HasKeyData for NiBoolTimelineInterpolator {
    type Data = NiBoolData;
}
impl HasKeyData for NiFloatInterpolator {
    type Data = NiFloatData;
}
impl HasKeyData for NiPoint3Interpolator {
    type Data = NiPosData;
}
impl HasKeyData for NiTransformInterpolator {
    type Data = NiTransformData;
}
impl HasKeyData for BsRotAccumTransfInterpolator {
    type Data = NiTransformData;
}
impl HasKeyData for BsTreadTransfInterpolator {
    type Data = NiFloatData;
}

/// Interpolator refs.
impl<'a, T: NiType> View<'a, T> {
    /// The interpolator's key data, or `None` if it has none.
    pub fn key_data(&self) -> Option<View<'a, T::Data>>
    where
        T: HasKeyData,
    {
        self.follow(RefKind::KeyData)
    }

    /// The path interpolator's path, or `None` if it has none.
    pub fn path_data<I>(&self) -> Option<View<'a, NiPosData>>
    where
        T: IsA<NiPathInterpolator, I>,
    {
        self.follow(RefKind::PathData)
    }

    /// The path interpolator's position along the path over time, as percentages, or `None` if
    /// it has none.
    pub fn percent_data<I>(&self) -> Option<View<'a, NiFloatData>>
    where
        T: IsA<NiPathInterpolator, I>,
    {
        self.follow(RefKind::PercentData)
    }

    /// The node the look-at interpolator faces, or `None` if it has none.
    pub fn look_at_node<I>(&self) -> Option<View<'a, NiNode>>
    where
        T: IsA<NiLookAtInterpolator, I>,
    {
        self.follow(RefKind::LookAtNode)
    }

    /// The look-at interpolator's translation interpolator, or `None` if it has none.
    pub fn look_at_translation<I>(&self) -> Option<View<'a, NiPoint3Interpolator>>
    where
        T: IsA<NiLookAtInterpolator, I>,
    {
        self.follow(RefKind::LookAtTranslation)
    }

    /// The look-at interpolator's roll interpolator, or `None` if it has none.
    pub fn look_at_roll<I>(&self) -> Option<View<'a, NiFloatInterpolator>>
    where
        T: IsA<NiLookAtInterpolator, I>,
    {
        self.follow(RefKind::LookAtRoll)
    }

    /// The look-at interpolator's scale interpolator, or `None` if it has none.
    pub fn look_at_scale<I>(&self) -> Option<View<'a, NiFloatInterpolator>>
    where
        T: IsA<NiLookAtInterpolator, I>,
    {
        self.follow(RefKind::LookAtScale)
    }

    /// The blend interpolator's single interpolator, used when only one item is active, or
    /// `None` if it has none. Saved from 10.1.0.108 up to 10.1.0.111.
    pub fn single_interpolator<I>(&self) -> Option<View<'a, NiInterpolator>>
    where
        T: IsA<NiBlendInterpolator, I>,
    {
        self.follow(RefKind::SingleInterpolator)
    }

    /// The blend interpolator's items. See [`NiBlendInterpolator`] for when they're saved.
    pub fn blend_items<I>(&self) -> Vec<BlendItem>
    where
        T: IsA<NiBlendInterpolator, I>,
    {
        let blend: &NiBlendInterpolator = self.get().upcast_ref();
        ffi::blend_items(&blend.0)
            .into_iter()
            .map(|i| BlendItem {
                interpolator: index_from_ffi(i.interpolator),
                weight: i.weight,
                normalized_weight: i.normalized_weight,
                priority: i.priority,
                priority_32: i.priority_32,
                ease_spinner: i.ease_spinner,
            })
            .collect()
    }

    /// The B-spline interpolator's control points, or `None` if it has none.
    pub fn spline_data<I>(&self) -> Option<View<'a, NiBSplineData>>
    where
        T: IsA<NiBSplineInterpolator, I>,
    {
        self.follow(RefKind::SplineData)
    }

    /// The B-spline interpolator's basis, or `None` if it has none.
    pub fn spline_basis<I>(&self) -> Option<View<'a, NiBSplineBasisData>>
    where
        T: IsA<NiBSplineInterpolator, I>,
    {
        self.follow(RefKind::SplineBasis)
    }
}

impl<T: NiType> Edit<'_, T> {
    /// Like [`View::key_data`], for editing.
    pub fn key_data(&mut self) -> Option<Edit<'_, T::Data>>
    where
        T: HasKeyData,
    {
        self.follow(RefKind::KeyData)
    }

    /// Sets the interpolator's [key data](View::key_data), or clears it with `None`. Returns
    /// `false`, changing nothing, unless `data` is the interpolator's data type.
    pub fn set_key_data(&mut self, data: Option<Index>) -> bool
    where
        T: HasKeyData,
    {
        self.set_ref(RefKind::KeyData, data)
    }

    /// Like [`View::path_data`], for editing.
    pub fn path_data<I>(&mut self) -> Option<Edit<'_, NiPosData>>
    where
        T: IsA<NiPathInterpolator, I>,
    {
        self.follow(RefKind::PathData)
    }

    /// Sets the [path](View::path_data), or clears it with `None`. Returns `false`, changing
    /// nothing, unless `data` is an [`NiPosData`].
    pub fn set_path_data<I>(&mut self, data: Option<Index>) -> bool
    where
        T: IsA<NiPathInterpolator, I>,
    {
        self.set_ref(RefKind::PathData, data)
    }

    /// Like [`View::percent_data`], for editing.
    pub fn percent_data<I>(&mut self) -> Option<Edit<'_, NiFloatData>>
    where
        T: IsA<NiPathInterpolator, I>,
    {
        self.follow(RefKind::PercentData)
    }

    /// Sets the [percent data](View::percent_data), or clears it with `None`. Returns `false`,
    /// changing nothing, unless `data` is an [`NiFloatData`].
    pub fn set_percent_data<I>(&mut self, data: Option<Index>) -> bool
    where
        T: IsA<NiPathInterpolator, I>,
    {
        self.set_ref(RefKind::PercentData, data)
    }

    /// Like [`View::look_at_node`], for editing.
    pub fn look_at_node<I>(&mut self) -> Option<Edit<'_, NiNode>>
    where
        T: IsA<NiLookAtInterpolator, I>,
    {
        self.follow(RefKind::LookAtNode)
    }

    /// Sets the [look-at node](View::look_at_node), or clears it with `None`. Returns `false`,
    /// changing nothing, unless `node` is an [`NiNode`]. The
    /// [look-at name](NiLookAtInterpolator::look_at_name) isn't changed.
    pub fn set_look_at_node<I>(&mut self, node: Option<Index>) -> bool
    where
        T: IsA<NiLookAtInterpolator, I>,
    {
        self.set_ref(RefKind::LookAtNode, node)
    }

    /// Like [`View::look_at_translation`], for editing.
    pub fn look_at_translation<I>(&mut self) -> Option<Edit<'_, NiPoint3Interpolator>>
    where
        T: IsA<NiLookAtInterpolator, I>,
    {
        self.follow(RefKind::LookAtTranslation)
    }

    /// Sets the [translation interpolator](View::look_at_translation), or clears it with `None`.
    /// Returns `false`, changing nothing, unless `interpolator` is an [`NiPoint3Interpolator`].
    pub fn set_look_at_translation<I>(&mut self, interpolator: Option<Index>) -> bool
    where
        T: IsA<NiLookAtInterpolator, I>,
    {
        self.set_ref(RefKind::LookAtTranslation, interpolator)
    }

    /// Like [`View::look_at_roll`], for editing.
    pub fn look_at_roll<I>(&mut self) -> Option<Edit<'_, NiFloatInterpolator>>
    where
        T: IsA<NiLookAtInterpolator, I>,
    {
        self.follow(RefKind::LookAtRoll)
    }

    /// Sets the [roll interpolator](View::look_at_roll), or clears it with `None`. Returns
    /// `false`, changing nothing, unless `interpolator` is an [`NiFloatInterpolator`].
    pub fn set_look_at_roll<I>(&mut self, interpolator: Option<Index>) -> bool
    where
        T: IsA<NiLookAtInterpolator, I>,
    {
        self.set_ref(RefKind::LookAtRoll, interpolator)
    }

    /// Like [`View::look_at_scale`], for editing.
    pub fn look_at_scale<I>(&mut self) -> Option<Edit<'_, NiFloatInterpolator>>
    where
        T: IsA<NiLookAtInterpolator, I>,
    {
        self.follow(RefKind::LookAtScale)
    }

    /// Sets the [scale interpolator](View::look_at_scale), or clears it with `None`. Returns
    /// `false`, changing nothing, unless `interpolator` is an [`NiFloatInterpolator`].
    pub fn set_look_at_scale<I>(&mut self, interpolator: Option<Index>) -> bool
    where
        T: IsA<NiLookAtInterpolator, I>,
    {
        self.set_ref(RefKind::LookAtScale, interpolator)
    }

    /// Like [`View::single_interpolator`], for editing.
    pub fn single_interpolator<I>(&mut self) -> Option<Edit<'_, NiInterpolator>>
    where
        T: IsA<NiBlendInterpolator, I>,
    {
        self.follow(RefKind::SingleInterpolator)
    }

    /// Sets the [single interpolator](View::single_interpolator), or clears it with `None`.
    /// Returns `false`, changing nothing, unless `interpolator` is an [`NiInterpolator`] that
    /// doesn't use this one.
    pub fn set_single_interpolator<I>(&mut self, interpolator: Option<Index>) -> bool
    where
        T: IsA<NiBlendInterpolator, I>,
    {
        self.set_ref(RefKind::SingleInterpolator, interpolator)
    }

    /// Replaces the [blend items](View::blend_items). Where the version and flags save them, the
    /// [array size](NiBlendInterpolator::array_size) becomes their number.
    ///
    /// Returns `false`, changing nothing, unless each interpolator is empty or an
    /// [`NiInterpolator`] that doesn't use this one, and the version can store the number of
    /// items (255 from 10.1.0.110 on, 65535 before).
    pub fn set_blend_items<I>(&mut self, items: &[BlendItem]) -> bool
    where
        T: IsA<NiBlendInterpolator, I>,
    {
        let Some(items) = items
            .iter()
            .map(|i| {
                Some(ffi::BlendItemFfi {
                    interpolator: optional_index_to_ffi(i.interpolator)?,
                    weight: i.weight,
                    normalized_weight: i.normalized_weight,
                    priority: i.priority,
                    priority_32: i.priority_32,
                    ease_spinner: i.ease_spinner,
                })
            })
            .collect::<Option<Vec<_>>>()
        else {
            return false;
        };
        let owner = self.index() as u32;
        ffi::set_blend_items(self.file_mut().inner.pin_mut(), owner, &items)
    }

    /// Like [`View::spline_data`], for editing.
    pub fn spline_data<I>(&mut self) -> Option<Edit<'_, NiBSplineData>>
    where
        T: IsA<NiBSplineInterpolator, I>,
    {
        self.follow(RefKind::SplineData)
    }

    /// Sets the [control points](View::spline_data), or clears them with `None`. Returns
    /// `false`, changing nothing, unless `data` is an [`NiBSplineData`].
    pub fn set_spline_data<I>(&mut self, data: Option<Index>) -> bool
    where
        T: IsA<NiBSplineInterpolator, I>,
    {
        self.set_ref(RefKind::SplineData, data)
    }

    /// Like [`View::spline_basis`], for editing.
    pub fn spline_basis<I>(&mut self) -> Option<Edit<'_, NiBSplineBasisData>>
    where
        T: IsA<NiBSplineInterpolator, I>,
    {
        self.follow(RefKind::SplineBasis)
    }

    /// Sets the [basis](View::spline_basis), or clears it with `None`. Returns `false`, changing
    /// nothing, unless `basis` is an [`NiBSplineBasisData`].
    pub fn set_spline_basis<I>(&mut self, basis: Option<Index>) -> bool
    where
        T: IsA<NiBSplineInterpolator, I>,
    {
        self.set_ref(RefKind::SplineBasis, basis)
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
