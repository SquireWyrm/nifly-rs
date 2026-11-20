//! Animation: keys, animation data, interpolators, controllers and sequences.
//!
//! Data blocks hold [`KeyGroup`]s: keys of a value over time, and how values between them are
//! interpolated. Interpolators read data blocks, and controllers use interpolators to animate
//! their target. Sequences name the controllers and interpolators of one animation, and a
//! controller manager plays sequences.

use std::pin::Pin;

use bstr::{BStr, BString, ByteSlice};
use int_enum::IntEnum;
use nifly_sys::animation::ffi;

use crate::{
    file::{block_index_to_ffi, index_from_ffi, optional_index_to_ffi, Index, NifFile},
    handle::{Edit, View},
    iter::{BlockIter, RefIndices},
    nodes::NiNode,
    object3d::Vec3,
    objects::{IsA, NiObject, NiObjectNet, NiType, Sealed},
    shaders::{get_bits, set_bits, BitField, BsShaderProperty, MapSlot, NiSourceTexture, TextureMap},
};
use nifly_sys::nif_file_refs::ffi::{self as refs_ffi, RefKind, RefList};

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
    ///
    /// Controllers form a chain on the object they animate, from
    /// [`View::controllers`]; [`Edit::add_controller`] adds one. Through handles: the
    /// [next controller](View::next_controller) and [target](View::controller_target).
    NiTimeController(NiTimeController): NiObject => upcast_ni_time_controller, downcast_ni_time_controller;
    /// Turns its target to face another object, the [look-at node](View::look_at_node).
    NiLookAtController(NiLookAtController): NiTimeController => upcast_ni_look_at_controller, downcast_ni_look_at_controller;
    /// Moves its target along a path, from its [path data](View::path_data).
    NiPathController(NiPathController): NiTimeController => upcast_ni_path_controller, downcast_ni_path_controller;
    /// Resets a particle system when its animation loops.
    NiPSysResetOnLoopCtlr(NiPSysResetOnLoopCtlr): NiTimeController => upcast_ni_psys_reset_on_loop_ctlr, downcast_ni_psys_reset_on_loop_ctlr;
    /// Animates texture coordinates with [`NiUvData`].
    ///
    /// The `NiUVController` block.
    NiUvController(NiUVController): NiTimeController => upcast_ni_uv_controller, downcast_ni_uv_controller;
    /// Animates a camera's field of view with its [interpolator](View::interpolator).
    ///
    /// The `BSFrustumFOVController` block.
    BsFrustumFovController(BSFrustumFOVController): NiTimeController => upcast_bs_frustum_fov_controller, downcast_bs_frustum_fov_controller;
    /// Makes a bone lag behind its parent's motion.
    ///
    /// The `BSLagBoneController` block.
    BsLagBoneController(BSLagBoneController): NiTimeController => upcast_bs_lag_bone_controller, downcast_bs_lag_bone_controller;
    /// Animates procedural lightning, with nine [interpolators](View::lightning_interpolator) and
    /// a [shader](View::lightning_shader).
    ///
    /// The `BSProceduralLightningController` block.
    BsProceduralLightningController(BSProceduralLightningController): NiTimeController => upcast_bs_procedural_lightning_controller, downcast_bs_procedural_lightning_controller;
    /// Switches bone levels of detail, each a [node group](View::node_groups).
    ///
    /// The `NiBoneLODController` block.
    NiBoneLodController(NiBoneLODController): NiTimeController => upcast_ni_bone_lod_controller, downcast_ni_bone_lod_controller;
    /// Bethesda's bone level-of-detail controller.
    ///
    /// The `NiBSBoneLODController` block.
    NiBsBoneLodController(NiBSBoneLODController): NiBoneLodController => upcast_ni_bs_bone_lod_controller, downcast_ni_bs_bone_lod_controller;
    /// Base class of controllers driven by interpolators.
    NiInterpController(NiInterpController): NiTimeController => upcast_ni_interp_controller, downcast_ni_interp_controller;
    /// Morphs its target's vertices with [`NiMorphData`], from [`View::key_data`], and
    /// [interpolators](View::morph_interpolators).
    NiGeomMorpherController(NiGeomMorpherController): NiInterpController => upcast_ni_geom_morpher_controller, downcast_ni_geom_morpher_controller;
    /// Base class of controllers with one interpolator, from [`View::interpolator`].
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
    /// Flips through a list of [textures](View::sources).
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
    /// Animates the transforms of several objects: its target and
    /// [extra targets](View::extra_targets).
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
    /// A named animation: the objects it animates, with their controllers and interpolators.
    ///
    /// Through handles: [controlled blocks](View::controlled_blocks) and
    /// [text keys](View::text_key_data). [`NifFile::add_sequence`] adds one.
    NiSequence(NiSequence): NiObject => upcast_ni_sequence, downcast_ni_sequence;
    /// A note in an animation, such as a grab or look-at event.
    ///
    /// The `BSAnimNote` block.
    BsAnimNote(BSAnimNote): NiObject => upcast_bs_anim_note, downcast_bs_anim_note;
    /// A list of [`BsAnimNote`]s, from [`View::notes`].
    ///
    /// The `BSAnimNotes` block.
    BsAnimNotes(BSAnimNotes): NiObject => upcast_bs_anim_notes, downcast_bs_anim_notes;
    /// An animation sequence, played by an [`NiControllerManager`] or stored alone in a `.kf`
    /// file.
    ///
    /// Through handles: the [manager](View::sequence_manager),
    /// [string palette](View::string_palette) and notes ([`View::anim_notes`],
    /// [`View::anim_note_arrays`]).
    NiControllerSequence(NiControllerSequence): NiSequence => upcast_ni_controller_sequence, downcast_ni_controller_sequence;
    /// Plays animation sequences on an object tree.
    ///
    /// Through handles: the [sequences](View::sequences), added with [`Edit::add_sequence`], and
    /// the [object palette](View::object_palette).
    NiControllerManager(NiControllerManager): NiTimeController => upcast_ni_controller_manager, downcast_ni_controller_manager;
    /// A Havok blend controller, found in skeletons.
    ///
    /// The `bhkBlendController` block.
    BhkBlendController(bhkBlendController): NiTimeController => upcast_bhk_blend_controller, downcast_bhk_blend_controller;
    /// The root of a Morrowind `.kf` file, holding its text keys as extra data and its
    /// controllers.
    NiSequenceStreamHelper(NiSequenceStreamHelper): NiObjectNet => upcast_ni_sequence_stream_helper, downcast_ni_sequence_stream_helper;
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

    /// Adds a new `T` named `name` and returns its index. An [`NiControllerSequence`] has weight
    /// and frequency 1 and loops. Nothing refers to it, so it suits the root of a `.kf` file;
    /// [`Edit::add_sequence`] adds one to a manager.
    pub fn add_sequence<T: NewSequence>(&mut self, name: impl AsRef<[u8]>) -> Index {
        index_from_ffi(T::add(self.inner.pin_mut(), name.as_ref())).expect("adding a block succeeds")
    }
}

pub(crate) fn same<T>(x: T) -> T {
    x
}

pub(crate) fn vec3(v: ffi::Vec3) -> Vec3 {
    Vec3::from_ffi(v)
}

pub(crate) fn vec3_to_ffi(v: Vec3) -> ffi::Vec3 {
    v.to_ffi()
}

mod keys;
mod data;
mod interpolators;
mod interpolator_refs;
mod controllers;
mod controller_types;
mod sequences;

pub use self::keys::*;
pub use self::data::*;
pub use self::interpolators::*;
pub use self::interpolator_refs::*;
pub use self::controllers::*;
pub use self::controller_types::*;
pub use self::sequences::*;
