//! Controller flags, chains, targets and adding controllers.

use super::*;

/// How a controller maps time onto its keys, from its [flags](TimeControllerFlags).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum CycleType {
    /// Starts over after the stop time.
    Loop = 0,
    /// Plays forward, then backward.
    Reverse = 1,
    /// Holds the last value after the stop time.
    Clamp = 2,
}

/// When a controller's time starts, from its [flags](TimeControllerFlags).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum AnimType {
    /// Application time.
    AppTime = 0,
    /// Time since the controller was started.
    AppInit = 1,
}

/// The packed settings of an [`NiTimeController`].
///
/// Bits:
/// - 0: [`AnimType`]
/// - 1–2: [`CycleType`]
/// - 3: active
/// - 4: play backwards
/// - 5: manager-controlled
/// - 6: compute scaled time
/// - 7: forced update
///
/// The `with_*` methods replace one setting and keep the others.
///
/// ```
/// # use nifly::animation::{CycleType, TimeControllerFlags};
/// let flags = TimeControllerFlags::DEFAULT.with_cycle_type(CycleType::Loop);
/// assert!(flags.active());
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TimeControllerFlags(u16);

impl Default for TimeControllerFlags {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl TimeControllerFlags {
    /// The flags of a new controller: active, and [`Clamp`](CycleType::Clamp).
    pub const DEFAULT: Self = Self(0x000C);

    const ANIM_TYPE: BitField = (0, 1);
    const CYCLE_TYPE: BitField = (1, 2);
    const ACTIVE: BitField = (3, 1);
    const PLAY_BACKWARDS: BitField = (4, 1);
    const MANAGER_CONTROLLED: BitField = (5, 1);
    const COMPUTE_SCALED_TIME: BitField = (6, 1);
    const FORCED_UPDATE: BitField = (7, 1);

    pub const fn from_bits(bits: u16) -> Self {
        Self(bits)
    }

    pub const fn bits(self) -> u16 {
        self.0
    }

    fn field(self, field: BitField) -> u32 {
        get_bits(self.0, field)
    }

    fn flag(self, field: BitField) -> bool {
        self.field(field) != 0
    }

    fn with_field(self, field: BitField, value: u32) -> Self {
        Self(set_bits(self.0, field, value))
    }

    pub fn anim_type(self) -> AnimType {
        AnimType::try_from(self.field(Self::ANIM_TYPE)).expect("1 bit is always an anim type")
    }

    /// The cycle type. `None` if the bits hold 3.
    pub fn cycle_type(self) -> Option<CycleType> {
        CycleType::try_from(self.field(Self::CYCLE_TYPE)).ok()
    }

    /// Whether the controller runs.
    pub fn active(self) -> bool {
        self.flag(Self::ACTIVE)
    }

    pub fn play_backwards(self) -> bool {
        self.flag(Self::PLAY_BACKWARDS)
    }

    /// Whether an [`NiControllerManager`] drives the controller.
    pub fn manager_controlled(self) -> bool {
        self.flag(Self::MANAGER_CONTROLLED)
    }

    pub fn compute_scaled_time(self) -> bool {
        self.flag(Self::COMPUTE_SCALED_TIME)
    }

    pub fn forced_update(self) -> bool {
        self.flag(Self::FORCED_UPDATE)
    }

    #[must_use]
    pub fn with_anim_type(self, anim_type: AnimType) -> Self {
        self.with_field(Self::ANIM_TYPE, anim_type.into())
    }

    #[must_use]
    pub fn with_cycle_type(self, cycle_type: CycleType) -> Self {
        self.with_field(Self::CYCLE_TYPE, cycle_type.into())
    }

    #[must_use]
    pub fn with_active(self, on: bool) -> Self {
        self.with_field(Self::ACTIVE, on.into())
    }

    #[must_use]
    pub fn with_play_backwards(self, on: bool) -> Self {
        self.with_field(Self::PLAY_BACKWARDS, on.into())
    }

    #[must_use]
    pub fn with_manager_controlled(self, on: bool) -> Self {
        self.with_field(Self::MANAGER_CONTROLLED, on.into())
    }

    #[must_use]
    pub fn with_compute_scaled_time(self, on: bool) -> Self {
        self.with_field(Self::COMPUTE_SCALED_TIME, on.into())
    }

    #[must_use]
    pub fn with_forced_update(self, on: bool) -> Self {
        self.with_field(Self::FORCED_UPDATE, on.into())
    }
}

impl NiTimeController {
    fields! {
        flags, set_flags: TimeControllerFlags = controller_flags, set_controller_flags, TimeControllerFlags::from_bits, TimeControllerFlags::bits;
        /// The speed of time for the controller: 1 is normal.
        frequency, set_frequency: f32 = controller_frequency, set_controller_frequency, same, same;
        /// The time offset.
        phase, set_phase: f32 = controller_phase, set_controller_phase, same, same;
        /// The time the animation starts, in seconds. A new controller has `f32::MAX`.
        start_time, set_start_time: f32 = controller_start_time, set_controller_start_time, same, same;
        /// The time the animation stops, in seconds. A new controller has `f32::MIN`.
        stop_time, set_stop_time: f32 = controller_stop_time, set_controller_stop_time, same, same;
    }
}

impl NiInterpController {
    fields! {
        /// Whether an [`NiControllerManager`] drives the controller. Saved only from 10.1.0.104
        /// to 10.1.0.108.
        manager_controlled, set_manager_controlled: bool = interp_controller_manager_controlled, set_interp_controller_manager_controlled, same, same;
    }
}

/// Controller chains of named objects.
impl<'a, T: NiType> View<'a, T> {
    /// The object's first controller, or `None` if it has none.
    pub fn controller<I>(&self) -> Option<View<'a, NiTimeController>>
    where
        T: IsA<NiObjectNet, I>,
    {
        self.follow(RefKind::Controller)
    }

    /// The object's controllers: the first, then each one's
    /// [next controller](View::next_controller). Stops at an empty ref or a controller already
    /// listed.
    pub fn controllers<I>(&self) -> BlockIter<'a, NiTimeController, RefIndices>
    where
        T: IsA<NiObjectNet, I>,
    {
        self.follow_list(RefList::Controllers)
    }
}

impl<T: NiType> Edit<'_, T> {
    /// Like [`View::controller`], for editing.
    pub fn controller<I>(&mut self) -> Option<Edit<'_, NiTimeController>>
    where
        T: IsA<NiObjectNet, I>,
    {
        self.follow(RefKind::Controller)
    }

    /// Sets the object's first controller, or removes it with `None`, keeping that controller's
    /// next controllers. Returns `false`, changing nothing, unless `controller` is an
    /// [`NiTimeController`].
    pub fn set_controller<I>(&mut self, controller: Option<Index>) -> bool
    where
        T: IsA<NiObjectNet, I>,
    {
        self.set_ref(RefKind::Controller, controller)
    }

    /// Replaces the object's [controllers](View::controllers), linking each to the next. Their
    /// [targets](View::controller_target) don't change. Returns `false`, changing nothing,
    /// unless the blocks are distinct [`NiTimeController`]s.
    pub fn set_controllers<I>(&mut self, controllers: &[Index]) -> bool
    where
        T: IsA<NiObjectNet, I>,
    {
        self.set_ref_list(RefList::Controllers, controllers)
    }

    /// Adds a new `C` with default settings to the end of the object's
    /// [controllers](View::controllers), targeting the object, and returns its index.
    ///
    /// ```
    /// # use nifly::{animation::{NiAlphaController, NiFloatInterpolator}, file::{NifFile, NifVersion}, nodes::NiNode};
    /// let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    /// let mut root = nif.root_node_mut().unwrap();
    /// let controller = root.add_controller::<NiAlphaController, _>();
    /// let interpolator = nif.add_interpolator::<NiFloatInterpolator>();
    /// assert!(nif.edit::<NiAlphaController>(controller).unwrap().set_interpolator(Some(interpolator)));
    /// ```
    pub fn add_controller<C: NewController, I>(&mut self) -> Index
    where
        T: IsA<NiObjectNet, I>,
    {
        let owner = self.index() as u32;
        index_from_ffi(C::add(self.file_mut().inner.pin_mut(), owner))
            .expect("the owner is an NiObjectNet")
    }
}

/// Controller refs.
impl<'a, T: NiType> View<'a, T> {
    /// The controller after this one in its object's chain, or `None` if it's the last.
    pub fn next_controller<I>(&self) -> Option<View<'a, NiTimeController>>
    where
        T: IsA<NiTimeController, I>,
    {
        self.follow(RefKind::NextController)
    }

    /// The object the controller animates, or `None` if it has none.
    pub fn controller_target<I>(&self) -> Option<View<'a, NiObjectNet>>
    where
        T: IsA<NiTimeController, I>,
    {
        self.follow(RefKind::ControllerTarget)
    }

    /// The controller's interpolator, or `None` if it has none. Saved from 10.1.0.104 on; older
    /// controllers keep their keys in [key data](View::key_data).
    pub fn interpolator(&self) -> Option<View<'a, NiInterpolator>>
    where
        T: HasInterpolator,
    {
        self.follow(RefKind::Interpolator)
    }
}

impl<T: NiType> Edit<'_, T> {
    /// Like [`View::next_controller`], for editing.
    pub fn next_controller<I>(&mut self) -> Option<Edit<'_, NiTimeController>>
    where
        T: IsA<NiTimeController, I>,
    {
        self.follow(RefKind::NextController)
    }

    /// Sets the [next controller](View::next_controller), or ends the chain with `None`.
    /// Returns `false`, changing nothing, unless `next` is an [`NiTimeController`] whose chain
    /// doesn't lead back to this one.
    pub fn set_next_controller<I>(&mut self, next: Option<Index>) -> bool
    where
        T: IsA<NiTimeController, I>,
    {
        self.set_ref(RefKind::NextController, next)
    }

    /// Like [`View::controller_target`], for editing.
    pub fn controller_target<I>(&mut self) -> Option<Edit<'_, NiObjectNet>>
    where
        T: IsA<NiTimeController, I>,
    {
        self.follow(RefKind::ControllerTarget)
    }

    /// Sets the [target](View::controller_target), or clears it with `None`. Returns `false`,
    /// changing nothing, unless `target` is an [`NiObjectNet`].
    pub fn set_controller_target<I>(&mut self, target: Option<Index>) -> bool
    where
        T: IsA<NiTimeController, I>,
    {
        self.set_ref(RefKind::ControllerTarget, target)
    }

    /// Like [`View::interpolator`], for editing.
    pub fn interpolator(&mut self) -> Option<Edit<'_, NiInterpolator>>
    where
        T: HasInterpolator,
    {
        self.follow(RefKind::Interpolator)
    }

    /// Sets the [interpolator](View::interpolator), or clears it with `None`. Returns `false`,
    /// changing nothing, unless `interpolator` is an [`NiInterpolator`].
    pub fn set_interpolator(&mut self, interpolator: Option<Index>) -> bool
    where
        T: HasInterpolator,
    {
        self.set_ref(RefKind::Interpolator, interpolator)
    }
}

/// A controller [`Edit::add_controller`] can add.
pub trait NewController: NiType + Sealed {
    #[doc(hidden)]
    fn add(file: Pin<&mut nifly_sys::nif_file::ffi::NifFile>, owner: u32) -> u32;
}

macro_rules! new_controller {
    ($($t:ident => $add:ident;)*) => {$(
        impl NewController for $t {
            fn add(file: Pin<&mut nifly_sys::nif_file::ffi::NifFile>, owner: u32) -> u32 {
                ffi::$add(file, owner)
            }
        }
    )*};
}

new_controller! {
    NiLookAtController => add_ni_look_at_controller;
    NiPathController => add_ni_path_controller;
    NiUvController => add_ni_uv_controller;
    BsFrustumFovController => add_bs_frustum_fov_controller;
    BsLagBoneController => add_bs_lag_bone_controller;
    BsProceduralLightningController => add_bs_procedural_lightning_controller;
    NiBoneLodController => add_ni_bone_lod_controller;
    NiBsBoneLodController => add_ni_bs_bone_lod_controller;
    NiGeomMorpherController => add_ni_geom_morpher_controller;
    NiRollController => add_ni_roll_controller;
    NiMaterialColorController => add_ni_material_color_controller;
    NiLightColorController => add_ni_light_color_controller;
    NiFloatExtraDataController => add_ni_float_extra_data_controller;
    NiVisController => add_ni_vis_controller;
    NiFlipController => add_ni_flip_controller;
    NiTextureTransformController => add_ni_texture_transform_controller;
    NiLightDimmerController => add_ni_light_dimmer_controller;
    NiLightRadiusController => add_ni_light_radius_controller;
    NiAlphaController => add_ni_alpha_controller;
    BsNiAlphaPropertyTestRefController => add_bs_ni_alpha_property_test_ref_controller;
    NiKeyframeController => add_ni_keyframe_controller;
    NiTransformController => add_ni_transform_controller;
    BsMaterialEmittanceMultController => add_bs_material_emittance_mult_controller;
    BsRefractionStrengthController => add_bs_refraction_strength_controller;
    BsRefractionFirePeriodController => add_bs_refraction_fire_period_controller;
    BsLightingShaderPropertyColorController => add_bs_lighting_shader_property_color_controller;
    BsLightingShaderPropertyFloatController => add_bs_lighting_shader_property_float_controller;
    BsLightingShaderPropertyUShortController => add_bs_lighting_shader_property_u_short_controller;
    BsEffectShaderPropertyColorController => add_bs_effect_shader_property_color_controller;
    BsEffectShaderPropertyFloatController => add_bs_effect_shader_property_float_controller;
    NiMultiTargetTransformController => add_ni_multi_target_transform_controller;
    BhkBlendController => add_bhk_blend_controller;
    NiControllerManager => add_ni_controller_manager;
}

/// An [`NiPathInterpolator`] or [`NiPathController`]: a block with path data refs.
///
/// This trait is sealed.
pub trait HasPath: NiType {}

impl HasPath for NiPathInterpolator {}
impl HasPath for NiPathController {}

/// An [`NiLookAtInterpolator`] or [`NiLookAtController`]: a block with a look-at node.
///
/// This trait is sealed.
pub trait LooksAt: NiType {}

impl LooksAt for NiLookAtInterpolator {}
impl LooksAt for NiLookAtController {}

/// A controller with one interpolator: an [`NiSingleInterpController`] or subtype, or a
/// [`BsFrustumFovController`].
///
/// This trait is sealed.
pub trait HasInterpolator: NiType {}

impl_marker! { HasInterpolator:
    NiSingleInterpController NiRollController NiPoint3InterpController NiMaterialColorController
    NiLightColorController NiExtraDataController NiFloatExtraDataController NiBoolInterpController
    NiVisController NiFloatInterpController BsRefractionFirePeriodController NiFlipController
    NiTextureTransformController NiLightDimmerController NiLightRadiusController NiAlphaController
    BsNiAlphaPropertyTestRefController NiKeyframeController NiTransformController
    BsMaterialEmittanceMultController BsRefractionStrengthController
    BsLightingShaderPropertyColorController BsLightingShaderPropertyFloatController
    BsLightingShaderPropertyUShortController BsEffectShaderPropertyColorController
    BsEffectShaderPropertyFloatController NiPSysModifierCtlr NiPSysModifierBoolCtlr
    NiPSysModifierActiveCtlr NiPSysModifierFloatCtlr NiPSysEmitterLifeSpanCtlr
    NiPSysEmitterSpeedCtlr NiPSysEmitterInitialRadiusCtlr NiPSysEmitterDeclinationCtlr
    NiPSysGravityStrengthCtlr NiPSysEmitterDeclinationVarCtlr NiPSysFieldMagnitudeCtlr
    NiPSysFieldAttenuationCtlr NiPSysFieldMaxDistanceCtlr NiPSysAirFieldAirFrictionCtlr
    NiPSysAirFieldInheritVelocityCtlr NiPSysAirFieldSpreadCtlr NiPSysInitialRotSpeedCtlr
    NiPSysInitialRotSpeedVarCtlr NiPSysInitialRotAngleCtlr NiPSysInitialRotAngleVarCtlr
    NiPSysEmitterPlanarAngleCtlr NiPSysEmitterPlanarAngleVarCtlr NiPSysRotDampeningCtlr
    BsFrustumFovController
}
