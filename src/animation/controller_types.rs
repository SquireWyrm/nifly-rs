//! Fields and refs of each controller type.

use super::*;

impl NiLookAtController {
    fields! {
        /// Saved from 10.1.0.0 on.
        look_at_flags, set_look_at_flags: LookAtFlags = look_at_controller_flags, set_look_at_controller_flags, LookAtFlags::from_bits_retain, |f: LookAtFlags| f.bits();
    }
}

impl NiPathController {
    fields! {
        /// Saved from 10.1.0.0 on.
        path_flags, set_path_flags: PathFlags = path_controller_flags, set_path_controller_flags, PathFlags::from_bits_retain, |f: PathFlags| f.bits();
        /// Which way to bank: 1 or -1.
        bank_direction, set_bank_direction: i32 = path_controller_bank_direction, set_path_controller_bank_direction, same, same;
        /// In radians.
        max_bank_angle, set_max_bank_angle: f32 = path_controller_max_bank_angle, set_path_controller_max_bank_angle, same, same;
        smoothing, set_smoothing: f32 = path_controller_smoothing, set_path_controller_smoothing, same, same;
        /// The axis to face along the path when following it: 0 X, 1 Y, 2 Z.
        follow_axis, set_follow_axis: u16 = path_controller_follow_axis, set_path_controller_follow_axis, same, same;
    }
}

impl NiUvController {
    fields! {
        /// The texture coordinate set to animate.
        uv_set, set_uv_set: u16 = uv_controller_uv_set, set_uv_controller_uv_set, same, same;
    }
}

impl BsLagBoneController {
    fields! {
        /// How fast the bone returns to rest.
        linear_velocity, set_linear_velocity: f32 = lag_bone_linear_velocity, set_lag_bone_linear_velocity, same, same;
        /// How much the bone lags in rotation.
        linear_rotation, set_linear_rotation: f32 = lag_bone_linear_rotation, set_lag_bone_linear_rotation, same, same;
        /// How far the bone can trail.
        max_distance, set_max_distance: f32 = lag_bone_max_distance, set_lag_bone_max_distance, same, same;
    }
}

impl BsProceduralLightningController {
    fields! {
        subdivisions, set_subdivisions: u16 = lightning_subdivisions, set_lightning_subdivisions, same, same;
        branch_count, set_branch_count: u16 = lightning_branch_count, set_lightning_branch_count, same, same;
        branch_count_variation, set_branch_count_variation: u16 = lightning_branch_variation, set_lightning_branch_variation, same, same;
        /// How far the bolt reaches.
        length, set_length: f32 = lightning_length, set_lightning_length, same, same;
        length_variation, set_length_variation: f32 = lightning_length_variation, set_lightning_length_variation, same, same;
        width, set_width: f32 = lightning_width, set_lightning_width, same, same;
        /// The width of forks, relative to their parent.
        child_width_multiplier, set_child_width_multiplier: f32 = lightning_child_width_multiplier, set_lightning_child_width_multiplier, same, same;
        /// How far the bolt arcs: 0 is straight.
        arc_offset, set_arc_offset: f32 = lightning_arc_offset, set_lightning_arc_offset, same, same;
        fade_main_bolt, set_fade_main_bolt: bool = lightning_fade_main_bolt, set_lightning_fade_main_bolt, same, same;
        fade_child_bolts, set_fade_child_bolts: bool = lightning_fade_child_bolts, set_lightning_fade_child_bolts, same, same;
        animate_arc_offset, set_animate_arc_offset: bool = lightning_animate_arc_offset, set_lightning_animate_arc_offset, same, same;
    }
}

/// Which color of a material an [`NiMaterialColorController`] animates.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u16)]
pub enum MaterialColor {
    Ambient = 0,
    Diffuse = 1,
    Specular = 2,
    SelfIllumination = 3,
}

/// Which color of a light an [`NiLightColorController`] animates.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u16)]
pub enum LightColor {
    Diffuse = 0,
    Ambient = 1,
}

impl NiMaterialColorController {
    /// The color to animate, saved from 10.1.0.0 on. `None` if the stored value is 4 or more.
    pub fn target_color(&self) -> Option<MaterialColor> {
        let point3: &NiPoint3InterpController = self.upcast_ref();
        MaterialColor::try_from(ffi::point3_controller_target_color(&point3.0)).ok()
    }

    /// Sets [`target_color`](Self::target_color).
    pub fn set_target_color(&mut self, color: MaterialColor) {
        ffi::set_point3_controller_target_color((**self).ffi_mut(), color.into());
    }
}

impl NiLightColorController {
    /// The color to animate, saved from 10.1.0.0 on. `None` if the stored value is 2 or more.
    pub fn target_color(&self) -> Option<LightColor> {
        let point3: &NiPoint3InterpController = self.upcast_ref();
        LightColor::try_from(ffi::point3_controller_target_color(&point3.0)).ok()
    }

    /// Sets [`target_color`](Self::target_color).
    pub fn set_target_color(&mut self, color: LightColor) {
        ffi::set_point3_controller_target_color((**self).ffi_mut(), color.into());
    }
}

impl NiFlipController {
    /// The map whose texture flips. `None` if the stored value is 12 or more.
    pub fn texture_map(&self) -> Option<TextureMap> {
        TextureMap::try_from(ffi::flip_texture_slot(&self.0)).ok()
    }

    /// Sets [`texture_map`](Self::texture_map).
    pub fn set_texture_map(&mut self, map: TextureMap) {
        ffi::set_flip_texture_slot(self.ffi_mut(), map.into());
    }

    fields! {
        /// Saved only before 10.1.0.104.
        accum_time, set_accum_time: f32 = flip_accum_time, set_flip_accum_time, same, same;
        /// The time between flips, saved only before 10.1.0.104.
        delta, set_delta: f32 = flip_delta, set_flip_delta, same, same;
    }
}

/// The part of a texture transform an [`NiTextureTransformController`] animates.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum TransformMember {
    TranslateU = 0,
    TranslateV = 1,
    Rotate = 2,
    ScaleU = 3,
    ScaleV = 4,
}

impl NiTextureTransformController {
    /// The map whose transform is animated. `None` for a fixed map whose stored value is 12 or
    /// more.
    pub fn map(&self) -> Option<MapSlot> {
        let slot = ffi::texture_transform_slot(&self.0);
        if ffi::texture_transform_shader_map(&self.0) {
            usize::try_from(slot).ok().map(MapSlot::Shader)
        } else {
            TextureMap::try_from(slot).ok().map(MapSlot::Map)
        }
    }

    /// Sets [`map`](Self::map). Returns `false`, changing nothing, for a shader map past
    /// `u32::MAX`.
    pub fn set_map(&mut self, map: impl Into<MapSlot>) -> bool {
        let (shader, slot) = match map.into() {
            MapSlot::Map(map) => (false, map.into()),
            MapSlot::Shader(i) => match u32::try_from(i) {
                Ok(i) => (true, i),
                Err(_) => return false,
            },
        };
        ffi::set_texture_transform_shader_map(self.ffi_mut(), shader);
        ffi::set_texture_transform_slot(self.ffi_mut(), slot);
        true
    }

    /// The part of the transform to animate. `None` if the stored value is 5 or more.
    pub fn operation(&self) -> Option<TransformMember> {
        TransformMember::try_from(ffi::texture_transform_operation(&self.0)).ok()
    }

    /// Sets [`operation`](Self::operation).
    pub fn set_operation(&mut self, operation: TransformMember) {
        ffi::set_texture_transform_operation(self.ffi_mut(), operation.into());
    }
}

impl NiFloatExtraDataController {
    /// The name of the [`NiFloatExtraData`](crate::extra_data::NiFloatExtraData) to animate.
    /// Saved from 10.2.0.0 on.
    pub fn extra_data_name(&self) -> &BStr {
        ffi::float_extra_data_controller_name(&self.0).as_bstr()
    }

    /// Sets [`extra_data_name`](Self::extra_data_name).
    pub fn set_extra_data_name(&mut self, name: impl AsRef<[u8]>) {
        ffi::set_float_extra_data_controller_name(self.ffi_mut(), name.as_ref());
    }
}

impl NiPSysModifierCtlr {
    /// The [name](crate::particles::NiPSysModifier::name) of the modifier to animate, in the controller's target.
    pub fn modifier_name(&self) -> &BStr {
        nifly_sys::particles::ffi::psys_modifier_ctlr_modifier_name(&self.0).as_bstr()
    }

    /// Sets [`modifier_name`](Self::modifier_name).
    pub fn set_modifier_name(&mut self, name: impl AsRef<[u8]>) {
        nifly_sys::particles::ffi::set_psys_modifier_ctlr_modifier_name(
            self.ffi_mut(),
            name.as_ref(),
        );
    }
}

/// Which float of a [`BsLightingShaderProperty`](crate::shaders::BsLightingShaderProperty) a
/// [`BsLightingShaderPropertyFloatController`] animates.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum LightingShaderControlledFloat {
    RefractionStrength = 0,
    /// Found in files; meaning unknown.
    Unknown3 = 3,
    /// Found in files; meaning unknown.
    Unknown4 = 4,
    EnvironmentMapScale = 8,
    Glossiness = 9,
    SpecularStrength = 10,
    EmissiveMultiple = 11,
    Alpha = 12,
    /// Found in files; meaning unknown.
    Unknown13 = 13,
    /// Found in files; meaning unknown.
    Unknown14 = 14,
    UOffset = 20,
    UScale = 21,
    VOffset = 22,
    VScale = 23,
}

/// Which color of a [`BsLightingShaderProperty`](crate::shaders::BsLightingShaderProperty) a
/// [`BsLightingShaderPropertyColorController`] animates.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum LightingShaderControlledColor {
    Specular = 0,
    Emissive = 1,
}

/// Which float of a [`BsEffectShaderProperty`](crate::shaders::BsEffectShaderProperty) a
/// [`BsEffectShaderPropertyFloatController`] animates.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum EffectShaderControlledVariable {
    EmissiveMultiple = 0,
    /// In degrees.
    FalloffStartAngle = 1,
    /// In degrees.
    FalloffStopAngle = 2,
    FalloffStartOpacity = 3,
    FalloffStopOpacity = 4,
    Alpha = 5,
    UOffset = 6,
    UScale = 7,
    VOffset = 8,
    VScale = 9,
    /// Found in files; meaning unknown.
    Unknown11 = 11,
    /// Found in files; meaning unknown.
    Unknown12 = 12,
    /// Found in files; meaning unknown.
    Unknown13 = 13,
    /// Found in files; meaning unknown.
    Unknown14 = 14,
}

/// Which color of a [`BsEffectShaderProperty`](crate::shaders::BsEffectShaderProperty) a
/// [`BsEffectShaderPropertyColorController`] animates.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum EffectShaderControlledColor {
    Emissive = 0,
}

impl BsLightingShaderPropertyFloatController {
    enum_field! {
        /// The float to animate, or `None` for a value it doesn't list.
        controlled_variable, set_controlled_variable: LightingShaderControlledFloat = lighting_float_controller_variable, set_lighting_float_controller_variable
    }
}

impl BsLightingShaderPropertyUShortController {
    fields! {
        /// The value to animate; its meaning is unknown.
        controlled_variable, set_controlled_variable: u32 = lighting_ushort_controller_variable, set_lighting_ushort_controller_variable, same, same;
    }
}

impl BsLightingShaderPropertyColorController {
    enum_field! {
        /// The color to animate, or `None` for a value it doesn't list.
        controlled_color, set_controlled_color: LightingShaderControlledColor = lighting_color_controller_color, set_lighting_color_controller_color
    }
}

impl BsEffectShaderPropertyFloatController {
    enum_field! {
        /// The float to animate, or `None` for a value it doesn't list.
        controlled_variable, set_controlled_variable: EffectShaderControlledVariable = effect_float_controller_variable, set_effect_float_controller_variable
    }
}

impl BsEffectShaderPropertyColorController {
    enum_field! {
        /// The color to animate, or `None` for a value it doesn't list.
        controlled_color, set_controlled_color: EffectShaderControlledColor = effect_color_controller_color, set_effect_color_controller_color
    }
}

bitflags::bitflags! {
    /// The flags of an [`NiGeomMorpherController`], saved from 10.0.1.2 on. Other bits are kept
    /// as stored.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct GeomMorpherFlags: u16 {
        /// Recompute normals as the vertices morph.
        const UPDATE_NORMALS = 1;
        const _ = !0;
    }
}

impl NiGeomMorpherController {
    fields! {
        morpher_flags, set_morpher_flags: GeomMorpherFlags = geom_morpher_flags, set_geom_morpher_flags, GeomMorpherFlags::from_bits_retain, |f: GeomMorpherFlags| f.bits();
        /// Update even when the target isn't visible.
        always_update, set_always_update: bool = geom_morpher_always_update, set_geom_morpher_always_update, same, same;
    }

    /// Values of unknown meaning, saved from 10.2.0.0 to 20.0.0.5 in Bethesda files.
    pub fn unknown_ints(&self) -> Vec<u32> {
        ffi::geom_morpher_unknown_ints(&self.0)
    }

    /// Sets [`unknown_ints`](Self::unknown_ints).
    pub fn set_unknown_ints(&mut self, ints: &[u32]) {
        ffi::set_geom_morpher_unknown_ints(self.ffi_mut(), ints);
    }
}

impl NiBoneLodController {
    fields! {
        /// The current level of detail.
        lod, set_lod: u32 = bone_lod_lod, set_bone_lod_lod, same, same;
    }

    /// The number of levels of detail, kept as the number of
    /// [node groups](crate::handle::View::node_groups).
    pub fn lod_count(&self) -> u32 {
        ffi::bone_lod_controller_count(&self.0)
    }
}

impl BhkBlendController {
    fields! {
        /// Usually 0.
        keys, set_keys: u32 = blend_controller_keys, set_blend_controller_keys, same, same;
    }
}

/// One of the nine interpolators of a [`BsProceduralLightningController`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LightningInterpolator {
    Generation,
    Mutation,
    Subdivision,
    BranchCount,
    BranchCountVariation,
    Length,
    LengthVariation,
    Width,
    ArcOffset,
}

impl LightningInterpolator {
    fn ref_kind(self) -> RefKind {
        match self {
            Self::Generation => RefKind::LightningGeneration,
            Self::Mutation => RefKind::LightningMutation,
            Self::Subdivision => RefKind::LightningSubdivision,
            Self::BranchCount => RefKind::LightningBranches,
            Self::BranchCountVariation => RefKind::LightningBranchVariation,
            Self::Length => RefKind::LightningLength,
            Self::LengthVariation => RefKind::LightningLengthVariation,
            Self::Width => RefKind::LightningWidth,
            Self::ArcOffset => RefKind::LightningArcOffset,
        }
    }
}

/// An interpolator of an [`NiGeomMorpherController`], one per morph, with its weight.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MorphWeight {
    pub interpolator: Option<Index>,
    /// Saved from 20.1.0.3 on.
    pub weight: f32,
}

/// Controller type refs.
impl<'a, T: NiType> View<'a, T> {
    /// One of the lightning controller's interpolators, or `None` if it has none.
    pub fn lightning_interpolator<I>(
        &self,
        which: LightningInterpolator,
    ) -> Option<View<'a, NiInterpolator>>
    where
        T: IsA<BsProceduralLightningController, I>,
    {
        self.follow(which.ref_kind())
    }

    /// The shader the lightning controller's bolts use, or `None` if it has none.
    pub fn lightning_shader<I>(&self) -> Option<View<'a, BsShaderProperty>>
    where
        T: IsA<BsProceduralLightningController, I>,
    {
        self.follow(RefKind::LightningShader)
    }

    /// The flip controller's textures. Skips empty refs.
    pub fn sources<I>(&self) -> BlockIter<'a, NiSourceTexture, RefIndices>
    where
        T: IsA<NiFlipController, I>,
    {
        self.follow_list(RefList::FlipSources)
    }

    /// The objects a multi-target controller animates, besides its target. Empty refs are
    /// `None`.
    pub fn extra_targets<I>(&self) -> Vec<Option<Index>>
    where
        T: IsA<NiMultiTargetTransformController, I>,
    {
        refs_ffi::block_ref_list(
            &self.file().inner,
            self.index() as u32,
            RefList::ExtraTargets,
        )
        .into_iter()
        .map(index_from_ffi)
        .collect()
    }

    /// The geometry morpher's interpolators, one per morph. Saved from 10.1.0.106 on; the
    /// weights only from 20.1.0.3.
    pub fn morph_interpolators<I>(&self) -> Vec<MorphWeight>
    where
        T: IsA<NiGeomMorpherController, I>,
    {
        ffi::morph_weights(&self.file().inner, self.index() as u32)
            .into_iter()
            .map(|w| MorphWeight {
                interpolator: index_from_ffi(w.interpolator),
                weight: w.weight,
            })
            .collect()
    }

    /// The bone LOD controller's node groups, one per level of detail. Skips empty refs.
    pub fn node_groups<I>(&self) -> Vec<Vec<Index>>
    where
        T: IsA<NiBoneLodController, I>,
    {
        let lod: &NiBoneLodController = self.get().upcast_ref();
        ffi::bone_lod_groups(&lod.0)
            .into_iter()
            .map(|g| g.nodes.into_iter().filter_map(index_from_ffi).collect())
            .collect()
    }
}

impl<T: NiType> Edit<'_, T> {
    /// Like [`View::lightning_interpolator`], for editing.
    pub fn lightning_interpolator<I>(
        &mut self,
        which: LightningInterpolator,
    ) -> Option<Edit<'_, NiInterpolator>>
    where
        T: IsA<BsProceduralLightningController, I>,
    {
        self.follow(which.ref_kind())
    }

    /// Sets one of the [lightning interpolators](View::lightning_interpolator), or clears it
    /// with `None`. Returns `false`, changing nothing, unless `interpolator` is an
    /// [`NiInterpolator`].
    pub fn set_lightning_interpolator<I>(
        &mut self,
        which: LightningInterpolator,
        interpolator: Option<Index>,
    ) -> bool
    where
        T: IsA<BsProceduralLightningController, I>,
    {
        self.set_ref(which.ref_kind(), interpolator)
    }

    /// Like [`View::lightning_shader`], for editing.
    pub fn lightning_shader<I>(&mut self) -> Option<Edit<'_, BsShaderProperty>>
    where
        T: IsA<BsProceduralLightningController, I>,
    {
        self.follow(RefKind::LightningShader)
    }

    /// Sets the [lightning shader](View::lightning_shader), or clears it with `None`. Returns
    /// `false`, changing nothing, unless `shader` is a [`BsShaderProperty`].
    pub fn set_lightning_shader<I>(&mut self, shader: Option<Index>) -> bool
    where
        T: IsA<BsProceduralLightningController, I>,
    {
        self.set_ref(RefKind::LightningShader, shader)
    }

    /// Replaces the flip controller's [textures](View::sources). Returns `false`, changing
    /// nothing, unless each is an [`NiSourceTexture`].
    pub fn set_sources<I>(&mut self, sources: &[Index]) -> bool
    where
        T: IsA<NiFlipController, I>,
    {
        self.set_ref_list(RefList::FlipSources, sources)
    }

    /// Replaces the [extra targets](View::extra_targets). Returns `false`, changing nothing,
    /// unless each is empty or an [`NiAvObject`](crate::objects::NiAvObject) and there are at
    /// most 65535.
    pub fn set_extra_targets<I>(&mut self, targets: &[Option<Index>]) -> bool
    where
        T: IsA<NiMultiTargetTransformController, I>,
    {
        let Some(targets) = targets
            .iter()
            .map(|&t| optional_index_to_ffi(t))
            .collect::<Option<Vec<_>>>()
        else {
            return false;
        };
        let owner = self.index() as u32;
        refs_ffi::set_block_ref_list(
            self.file_mut().inner.pin_mut(),
            owner,
            RefList::ExtraTargets,
            &targets,
        )
    }

    /// Replaces the [morph interpolators](View::morph_interpolators). Returns `false`, changing
    /// nothing, unless each interpolator is empty or an [`NiInterpolator`].
    pub fn set_morph_interpolators<I>(&mut self, interpolators: &[MorphWeight]) -> bool
    where
        T: IsA<NiGeomMorpherController, I>,
    {
        let Some(weights) = interpolators
            .iter()
            .map(|w| {
                Some(ffi::MorphWeightFfi {
                    interpolator: optional_index_to_ffi(w.interpolator)?,
                    weight: w.weight,
                })
            })
            .collect::<Option<Vec<_>>>()
        else {
            return false;
        };
        let owner = self.index() as u32;
        ffi::set_morph_weights(self.file_mut().inner.pin_mut(), owner, &weights)
    }

    /// Replaces the [node groups](View::node_groups) and sets the
    /// [LOD count](NiBoneLodController::lod_count) to their number. Returns `false`, changing
    /// nothing, unless each node is an [`NiNode`].
    pub fn set_node_groups<I>(&mut self, groups: &[Vec<Index>]) -> bool
    where
        T: IsA<NiBoneLodController, I>,
    {
        let Some(groups) = groups
            .iter()
            .map(|g| {
                Some(ffi::NodeGroupFfi {
                    nodes: g
                        .iter()
                        .map(|&i| block_index_to_ffi(i))
                        .collect::<Option<_>>()?,
                })
            })
            .collect::<Option<Vec<_>>>()
        else {
            return false;
        };
        let owner = self.index() as u32;
        ffi::set_bone_lod_groups(self.file_mut().inner.pin_mut(), owner, &groups)
    }
}
