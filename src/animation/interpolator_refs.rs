//! Interpolator and controller refs to their data and other interpolators.

use super::*;

/// An interpolator or controller with a key data block.
///
/// Interpolators: [`NiBoolInterpolator`] ([`NiBoolData`]), [`NiFloatInterpolator`]
/// ([`NiFloatData`]), [`NiPoint3Interpolator`] ([`NiPosData`]), [`NiTransformInterpolator`]
/// ([`NiTransformData`]), [`BsTreadTransfInterpolator`] ([`NiFloatData`]), and their subtypes.
///
/// Controllers that keep their keys before 10.1.0.104, when [interpolators](View::interpolator)
/// replaced them: [`NiKeyframeController`] ([`NiKeyframeData`]),
/// [`NiMaterialColorController`] and [`NiLightColorController`] ([`NiPosData`]),
/// [`NiVisController`] ([`NiVisData`]), [`NiAlphaController`] ([`NiFloatData`]), and their
/// subtypes.
///
/// Controllers that keep their keys in every version: [`NiRollController`] ([`NiFloatData`]),
/// [`NiUvController`] ([`NiUvData`]) and [`NiGeomMorpherController`] ([`NiMorphData`]).
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
impl HasKeyData for NiKeyframeController {
    type Data = NiKeyframeData;
}
impl HasKeyData for NiTransformController {
    type Data = NiKeyframeData;
}
impl HasKeyData for NiMaterialColorController {
    type Data = NiPosData;
}
impl HasKeyData for NiLightColorController {
    type Data = NiPosData;
}
impl HasKeyData for NiVisController {
    type Data = NiVisData;
}
impl HasKeyData for NiAlphaController {
    type Data = NiFloatData;
}
impl HasKeyData for BsNiAlphaPropertyTestRefController {
    type Data = NiFloatData;
}
impl HasKeyData for NiRollController {
    type Data = NiFloatData;
}
impl HasKeyData for NiUvController {
    type Data = NiUvData;
}
impl HasKeyData for NiGeomMorpherController {
    type Data = NiMorphData;
}

/// Interpolator refs.
impl<'a, T: NiType> View<'a, T> {
    /// The block's key data, or `None` if it has none.
    pub fn key_data(&self) -> Option<View<'a, T::Data>>
    where
        T: HasKeyData,
    {
        self.follow(RefKind::KeyData)
    }

    /// The path interpolator's path, or `None` if it has none.
    pub fn path_data(&self) -> Option<View<'a, NiPosData>>
    where
        T: HasPath,
    {
        self.follow(RefKind::PathData)
    }

    /// The path interpolator's position along the path over time, as percentages, or `None` if
    /// it has none.
    pub fn percent_data(&self) -> Option<View<'a, NiFloatData>>
    where
        T: HasPath,
    {
        self.follow(RefKind::PercentData)
    }

    /// The node the look-at interpolator faces, or `None` if it has none.
    pub fn look_at_node(&self) -> Option<View<'a, NiNode>>
    where
        T: LooksAt,
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

    /// Sets the block's [key data](View::key_data), or clears it with `None`. Returns `false`,
    /// changing nothing, unless `data` is the block's [data type](HasKeyData::Data).
    pub fn set_key_data(&mut self, data: Option<Index>) -> bool
    where
        T: HasKeyData,
    {
        self.set_ref(RefKind::KeyData, data)
    }

    /// Like [`View::path_data`], for editing.
    pub fn path_data(&mut self) -> Option<Edit<'_, NiPosData>>
    where
        T: HasPath,
    {
        self.follow(RefKind::PathData)
    }

    /// Sets the [path](View::path_data), or clears it with `None`. Returns `false`, changing
    /// nothing, unless `data` is an [`NiPosData`].
    pub fn set_path_data(&mut self, data: Option<Index>) -> bool
    where
        T: HasPath,
    {
        self.set_ref(RefKind::PathData, data)
    }

    /// Like [`View::percent_data`], for editing.
    pub fn percent_data(&mut self) -> Option<Edit<'_, NiFloatData>>
    where
        T: HasPath,
    {
        self.follow(RefKind::PercentData)
    }

    /// Sets the [percent data](View::percent_data), or clears it with `None`. Returns `false`,
    /// changing nothing, unless `data` is an [`NiFloatData`].
    pub fn set_percent_data(&mut self, data: Option<Index>) -> bool
    where
        T: HasPath,
    {
        self.set_ref(RefKind::PercentData, data)
    }

    /// Like [`View::look_at_node`], for editing.
    pub fn look_at_node(&mut self) -> Option<Edit<'_, NiNode>>
    where
        T: LooksAt,
    {
        self.follow(RefKind::LookAtNode)
    }

    /// Sets the [look-at node](View::look_at_node), or clears it with `None`. Returns `false`,
    /// changing nothing, unless `node` is an [`NiNode`]. The
    /// [look-at name](NiLookAtInterpolator::look_at_name) isn't changed.
    pub fn set_look_at_node(&mut self, node: Option<Index>) -> bool
    where
        T: LooksAt,
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
