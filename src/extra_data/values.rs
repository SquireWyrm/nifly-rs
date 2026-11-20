//! The values Edit::assign_extra_data adds.

use super::*;

/// A value that [`Edit::assign_extra_data`](crate::handle::Edit::assign_extra_data) turns into
/// an extra data block. Each value type creates one block type and carries that block's name, if
/// it has one:
///
/// | Value | Block | Name |
/// |---|---|---|
/// | [`ExtraString`] | [`NiStringExtraData`] | given |
/// | [`ExtraInteger`] | [`NiIntegerExtraData`] | given |
/// | [`ExtraFloat`] | [`NiFloatExtraData`] | given |
/// | [`ExtraBoolean`] | [`NiBooleanExtraData`] | given |
/// | [`ExtraIntegers`] | [`NiIntegersExtraData`] | given |
/// | [`ExtraBinary`] | [`NiBinaryExtraData`] | given |
/// | [`ExtraFloats`] | [`NiFloatsExtraData`] | given |
/// | [`ExtraStrings`] | [`NiStringsExtraData`] | given |
/// | [`ExtraSkinAttach`] | [`SkinAttach`] | given |
/// | [`ExtraPackedSharedGeom`] | [`BsPackedCombinedSharedGeomDataExtra`] | given |
/// | [`ExtraFurnitureMarker`] | [`BsFurnitureMarker`] | given |
/// | [`ExtraFurnitureMarkerNode`] | [`BsFurnitureMarkerNode`] | given |
/// | [`ExtraConnectPointParents`] | [`BsConnectPointParents`] | given |
/// | [`ExtraConnectPointChildren`] | [`BsConnectPointChildren`] | given |
/// | [`ExtraInvMarker`] | [`BsInvMarker`] | given |
/// | [`ExtraBsBound`] | [`BsBound`] | given |
/// | [`ExtraBehaviorGraph`] | [`BsBehaviorGraphExtraData`] | given |
/// | [`ExtraColor`] | [`NiColorExtraData`] | given |
/// | [`ExtraVector`] | [`NiVectorExtraData`] | given |
/// | [`ExtraVertWeights`] | [`NiVertWeightsExtraData`] | given |
/// | [`ExtraEyeCenter`] | [`BsEyeCenterExtraData`] | given |
/// | [`ExtraPositionData`] | [`BsPositionData`] | given |
/// | [`ExtraWArray`] | [`BsWArray`] | given |
/// | [`ExtraDecalPlacement`] | [`BsDecalPlacementVectorExtraData`] | given |
/// | [`ExtraBoneLods`] | [`BsBoneLodExtraData`] | given |
/// | [`ExtraTextKeys`] | [`NiTextKeyExtraData`] | given |
/// | [`ExtraDistantObject`] | [`BsDistantObjectExtraData`] | given |
/// | [`ExtraDistantObjectLargeRef`] | [`BsDistantObjectLargeRefExtraData`] | given |
/// | [`ExtraBoneTranslations`] | [`BoneTranslations`] | given |
/// | [`ExtraBsxFlags`] | [`BsxFlags`] | always "BSX" |
/// | [`ExtraBsCloth`] | [`BsClothExtraData`] | none |
/// | [`ExtraBsCollisionQueryProxy`] | [`BsCollisionQueryProxyExtraData`] | none |
///
/// The block's setters change the values afterwards, and [`NiExtraData::set_name`] the name.
///
/// This trait is sealed.
pub trait IntoExtraData: Sealed {
    /// Creates the block, attaches it to `target` and returns its index (`u32::MAX` on failure).
    #[doc(hidden)]
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32;
}

/// An [`NiStringExtraData`] named `.0` with the string `.1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtraString<N: AsRef<[u8]>, V: AsRef<[u8]>>(pub N, pub V);

/// An [`NiIntegerExtraData`] named `.0` with the value `.1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtraInteger<N: AsRef<[u8]>>(pub N, pub u32);

/// An [`NiFloatExtraData`] named `.0` with the value `.1`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExtraFloat<N: AsRef<[u8]>>(pub N, pub f32);

/// An [`NiBooleanExtraData`] named `.0` with the value `.1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtraBoolean<N: AsRef<[u8]>>(pub N, pub bool);

/// An [`NiIntegersExtraData`] named `.0` with the values `.1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtraIntegers<N: AsRef<[u8]>, V: AsRef<[u32]>>(pub N, pub V);

/// A [`BsxFlags`] block with these flags. Its name is always "BSX".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtraBsxFlags(pub BsxBitflags);

/// A [`BsClothExtraData`] block holding this raw Havok cloth data (e.g. read from a `.hkx` file).
/// It has no name.
///
/// Only valid in Fallout 4 and later files. Not checked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtraBsCloth<D: AsRef<[u8]>>(pub D);

/// A [`BsCollisionQueryProxyExtraData`] block holding this raw data. It has no name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtraBsCollisionQueryProxy<D: AsRef<[u8]>>(pub D);

impl<N: AsRef<[u8]>, V: AsRef<[u8]>> Sealed for ExtraString<N, V> {}
impl<N: AsRef<[u8]>, V: AsRef<[u8]>> IntoExtraData for ExtraString<N, V> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        assign_ffi::assign_string_extra_data(file, target, self.0.as_ref(), self.1.as_ref())
    }
}

impl<N: AsRef<[u8]>> Sealed for ExtraInteger<N> {}
impl<N: AsRef<[u8]>> IntoExtraData for ExtraInteger<N> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        assign_ffi::assign_integer_extra_data(file, target, self.0.as_ref(), self.1)
    }
}

impl<N: AsRef<[u8]>> Sealed for ExtraFloat<N> {}
impl<N: AsRef<[u8]>> IntoExtraData for ExtraFloat<N> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        assign_ffi::assign_float_extra_data(file, target, self.0.as_ref(), self.1)
    }
}

impl<N: AsRef<[u8]>> Sealed for ExtraBoolean<N> {}
impl<N: AsRef<[u8]>> IntoExtraData for ExtraBoolean<N> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        assign_ffi::assign_boolean_extra_data(file, target, self.0.as_ref(), self.1)
    }
}

impl<N: AsRef<[u8]>, V: AsRef<[u32]>> Sealed for ExtraIntegers<N, V> {}
impl<N: AsRef<[u8]>, V: AsRef<[u32]>> IntoExtraData for ExtraIntegers<N, V> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        assign_ffi::assign_integers_extra_data(file, target, self.0.as_ref(), self.1.as_ref())
    }
}

impl Sealed for ExtraBsxFlags {}
impl IntoExtraData for ExtraBsxFlags {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        assign_ffi::assign_bsx_flags(file, target, self.0.bits())
    }
}

impl<D: AsRef<[u8]>> Sealed for ExtraBsCloth<D> {}
impl<D: AsRef<[u8]>> IntoExtraData for ExtraBsCloth<D> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        assign_ffi::assign_bs_cloth_extra_data(file, target, self.0.as_ref())
    }
}

impl<D: AsRef<[u8]>> Sealed for ExtraBsCollisionQueryProxy<D> {}
impl<D: AsRef<[u8]>> IntoExtraData for ExtraBsCollisionQueryProxy<D> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        assign_ffi::assign_bs_collision_query_proxy_extra_data(file, target, self.0.as_ref())
    }
}

/// An [`NiBinaryExtraData`] named `.0` with the bytes `.1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtraBinary<N: AsRef<[u8]>, D: AsRef<[u8]>>(pub N, pub D);

/// An [`NiFloatsExtraData`] named `.0` with the values `.1`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExtraFloats<N: AsRef<[u8]>, V: AsRef<[f32]>>(pub N, pub V);

/// An [`NiStringsExtraData`] named `.0` with the strings `.1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtraStrings<N: AsRef<[u8]>, V: IntoIterator<Item: AsRef<[u8]>>>(pub N, pub V);

/// A [`BsPackedCombinedSharedGeomDataExtra`] named `.0` with the totals `.1` and objects `.2`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExtraPackedSharedGeom<N: AsRef<[u8]>, O: AsRef<[PackedGeomObject]>>(
    pub N,
    pub PackedGeomTotals,
    pub O,
);

/// A [`SkinAttach`] named `.0` (usually "SkinBMP") with the bone names `.1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtraSkinAttach<N: AsRef<[u8]>, B: IntoIterator<Item: AsRef<[u8]>>>(pub N, pub B);

/// A [`BsFurnitureMarker`] named `.0` (usually "FRN") with the positions `.1`. Skyrim and later
/// use [`ExtraFurnitureMarkerNode`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExtraFurnitureMarker<N: AsRef<[u8]>, P: AsRef<[FurniturePosition]>>(pub N, pub P);

/// A [`BsFurnitureMarkerNode`] named `.0` (usually "FRN") with the positions `.1`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExtraFurnitureMarkerNode<N: AsRef<[u8]>, P: AsRef<[FurniturePosition]>>(pub N, pub P);

/// A [`BsConnectPointParents`] named `.0` (usually "CPA") with the connect points `.1`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExtraConnectPointParents<N: AsRef<[u8]>, P: AsRef<[ConnectPoint]>>(pub N, pub P);

/// A [`BsConnectPointChildren`] named `.0` (usually "CPT"), [skinned](BsConnectPointChildren::skinned)
/// if `.1`, attaching to the connect points named in `.2`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtraConnectPointChildren<N: AsRef<[u8]>, T: IntoIterator<Item: AsRef<[u8]>>>(
    pub N,
    pub bool,
    pub T,
);

/// A [`BsInvMarker`] named `.0` (usually "INV") with the [rotation](BsInvMarker::rotation) `.1`
/// and zoom `.2`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExtraInvMarker<N: AsRef<[u8]>>(pub N, pub [u16; 3], pub f32);

/// A [`BsBound`] named `.0` (usually "BBX") with the center `.1` and half extents `.2`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExtraBsBound<N: AsRef<[u8]>>(pub N, pub Vec3, pub Vec3);

/// A [`BsBehaviorGraphExtraData`] named `.0` (usually "BGED") with the graph file `.1`,
/// [controlling the base skeleton](BsBehaviorGraphExtraData::controls_base_skeleton) if `.2`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtraBehaviorGraph<N: AsRef<[u8]>, F: AsRef<[u8]>>(pub N, pub F, pub bool);

impl<N: AsRef<[u8]>, D: AsRef<[u8]>> Sealed for ExtraBinary<N, D> {}
impl<N: AsRef<[u8]>, D: AsRef<[u8]>> IntoExtraData for ExtraBinary<N, D> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        assign_ffi::assign_binary_extra_data(file, target, self.0.as_ref(), self.1.as_ref())
    }
}

impl<N: AsRef<[u8]>, V: AsRef<[f32]>> Sealed for ExtraFloats<N, V> {}
impl<N: AsRef<[u8]>, V: AsRef<[f32]>> IntoExtraData for ExtraFloats<N, V> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        assign_ffi::assign_floats_extra_data(file, target, self.0.as_ref(), self.1.as_ref())
    }
}

impl<N: AsRef<[u8]>, V: IntoIterator<Item: AsRef<[u8]>>> Sealed for ExtraStrings<N, V> {}
impl<N: AsRef<[u8]>, V: IntoIterator<Item: AsRef<[u8]>>> IntoExtraData for ExtraStrings<N, V> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        let (bytes, lengths) = joined(self.1);
        assign_ffi::assign_strings_extra_data(file, target, self.0.as_ref(), &bytes, &lengths)
    }
}

impl<N: AsRef<[u8]>, O: AsRef<[PackedGeomObject]>> Sealed for ExtraPackedSharedGeom<N, O> {}
impl<N: AsRef<[u8]>, O: AsRef<[PackedGeomObject]>> IntoExtraData for ExtraPackedSharedGeom<N, O> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        let (objects, instances) = packed_objects_ffi(self.2.as_ref());
        assign_ffi::assign_packed_shared_geom(
            file,
            target,
            self.0.as_ref(),
            self.1.to_ffi(),
            &objects,
            &instances,
        )
    }
}

impl<N: AsRef<[u8]>, B: IntoIterator<Item: AsRef<[u8]>>> Sealed for ExtraSkinAttach<N, B> {}
impl<N: AsRef<[u8]>, B: IntoIterator<Item: AsRef<[u8]>>> IntoExtraData for ExtraSkinAttach<N, B> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        let (bytes, lengths) = joined(self.1);
        assign_ffi::assign_skin_attach(file, target, self.0.as_ref(), &bytes, &lengths)
    }
}

impl<N: AsRef<[u8]>, P: AsRef<[FurniturePosition]>> Sealed for ExtraFurnitureMarker<N, P> {}
impl<N: AsRef<[u8]>, P: AsRef<[FurniturePosition]>> IntoExtraData for ExtraFurnitureMarker<N, P> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        assign_ffi::assign_furniture_marker(
            file,
            target,
            self.0.as_ref(),
            &furniture_ffi(self.1.as_ref()),
            false,
        )
    }
}

impl<N: AsRef<[u8]>, P: AsRef<[FurniturePosition]>> Sealed for ExtraFurnitureMarkerNode<N, P> {}
impl<N: AsRef<[u8]>, P: AsRef<[FurniturePosition]>> IntoExtraData
    for ExtraFurnitureMarkerNode<N, P>
{
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        assign_ffi::assign_furniture_marker(
            file,
            target,
            self.0.as_ref(),
            &furniture_ffi(self.1.as_ref()),
            true,
        )
    }
}

impl<N: AsRef<[u8]>, P: AsRef<[ConnectPoint]>> Sealed for ExtraConnectPointParents<N, P> {}
impl<N: AsRef<[u8]>, P: AsRef<[ConnectPoint]>> IntoExtraData for ExtraConnectPointParents<N, P> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        let (bytes, lengths, transforms) = connect_points_ffi(self.1.as_ref());
        assign_ffi::assign_connect_point_parents(
            file,
            target,
            self.0.as_ref(),
            &bytes,
            &lengths,
            &transforms,
        )
    }
}

impl<N: AsRef<[u8]>, T: IntoIterator<Item: AsRef<[u8]>>> Sealed
    for ExtraConnectPointChildren<N, T>
{
}
impl<N: AsRef<[u8]>, T: IntoIterator<Item: AsRef<[u8]>>> IntoExtraData
    for ExtraConnectPointChildren<N, T>
{
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        let (bytes, lengths) = joined(self.2);
        assign_ffi::assign_connect_point_children(
            file,
            target,
            self.0.as_ref(),
            self.1,
            &bytes,
            &lengths,
        )
    }
}

impl<N: AsRef<[u8]>> Sealed for ExtraInvMarker<N> {}
impl<N: AsRef<[u8]>> IntoExtraData for ExtraInvMarker<N> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        let [rotation_x, rotation_y, rotation_z] = self.1;
        let marker = types_ffi::InvMarkerFfi {
            rotation_x,
            rotation_y,
            rotation_z,
            zoom: self.2,
        };
        assign_ffi::assign_inv_marker(file, target, self.0.as_ref(), marker)
    }
}

impl<N: AsRef<[u8]>> Sealed for ExtraBsBound<N> {}
impl<N: AsRef<[u8]>> IntoExtraData for ExtraBsBound<N> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        assign_ffi::assign_bs_bound(
            file,
            target,
            self.0.as_ref(),
            self.1.to_ffi(),
            self.2.to_ffi(),
        )
    }
}

impl<N: AsRef<[u8]>, F: AsRef<[u8]>> Sealed for ExtraBehaviorGraph<N, F> {}
impl<N: AsRef<[u8]>, F: AsRef<[u8]>> IntoExtraData for ExtraBehaviorGraph<N, F> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        assign_ffi::assign_behavior_graph_extra_data(
            file,
            target,
            self.0.as_ref(),
            self.1.as_ref(),
            self.2,
        )
    }
}

/// An [`NiColorExtraData`] named `.0` with the color `.1`, as `[r, g, b, a]`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExtraColor<N: AsRef<[u8]>>(pub N, pub [f32; 4]);

/// An [`NiVectorExtraData`] named `.0` with the vector `.1`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExtraVector<N: AsRef<[u8]>>(pub N, pub [f32; 4]);

/// An [`NiVertWeightsExtraData`] named `.0` with the weights `.1`. At most 65535, as the count is
/// stored in 16 bits; `assign_extra_data` returns `None` for more.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExtraVertWeights<N: AsRef<[u8]>, V: AsRef<[f32]>>(pub N, pub V);

/// A [`BsEyeCenterExtraData`] named `.0` with the values `.1`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExtraEyeCenter<N: AsRef<[u8]>, V: AsRef<[f32]>>(pub N, pub V);

/// A [`BsPositionData`] named `.0` with the values `.1`, rounded to 16-bit floats.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExtraPositionData<N: AsRef<[u8]>, V: AsRef<[f32]>>(pub N, pub V);

/// A [`BsWArray`] named `.0` with the values `.1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtraWArray<N: AsRef<[u8]>, V: AsRef<[u32]>>(pub N, pub V);

/// A [`BsDecalPlacementVectorExtraData`] named `.0` with the float value `.1` and the blocks
/// `.2`. At most 65535 blocks of at most 65535 points, as the counts are stored in 16 bits;
/// `assign_extra_data` returns `None` for more.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExtraDecalPlacement<N: AsRef<[u8]>, B: IntoIterator<Item: AsRef<[DecalPoint]>>>(
    pub N,
    pub f32,
    pub B,
);

/// An [`NiTextKeyExtraData`] named `.0` with the text keys `.1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtraTextKeys<N: AsRef<[u8]>, K: AsRef<[TextKey]>>(pub N, pub K);

/// A [`BsBoneLodExtraData`] named `.0` with the bone LODs `.1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtraBoneLods<N: AsRef<[u8]>, L: AsRef<[BoneLod]>>(pub N, pub L);

/// A [`BsDistantObjectExtraData`] named `.0` with the flags `.1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtraDistantObject<N: AsRef<[u8]>>(pub N, pub u32);

/// A [`BsDistantObjectLargeRefExtraData`] named `.0`, a large reference if `.1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtraDistantObjectLargeRef<N: AsRef<[u8]>>(pub N, pub bool);

/// A [`BoneTranslations`] named `.0` with the translations `.1`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExtraBoneTranslations<N: AsRef<[u8]>, T: AsRef<[BoneTranslation]>>(pub N, pub T);

impl<N: AsRef<[u8]>> Sealed for ExtraColor<N> {}
impl<N: AsRef<[u8]>> IntoExtraData for ExtraColor<N> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        assign_ffi::assign_color_extra_data(file, target, self.0.as_ref(), float4(self.1))
    }
}

impl<N: AsRef<[u8]>> Sealed for ExtraVector<N> {}
impl<N: AsRef<[u8]>> IntoExtraData for ExtraVector<N> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        assign_ffi::assign_vector_extra_data(file, target, self.0.as_ref(), float4(self.1))
    }
}

impl<N: AsRef<[u8]>, V: AsRef<[f32]>> Sealed for ExtraVertWeights<N, V> {}
impl<N: AsRef<[u8]>, V: AsRef<[f32]>> IntoExtraData for ExtraVertWeights<N, V> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        assign_ffi::assign_vert_weights_extra_data(file, target, self.0.as_ref(), self.1.as_ref())
    }
}

impl<N: AsRef<[u8]>, V: AsRef<[f32]>> Sealed for ExtraEyeCenter<N, V> {}
impl<N: AsRef<[u8]>, V: AsRef<[f32]>> IntoExtraData for ExtraEyeCenter<N, V> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        assign_ffi::assign_eye_center_extra_data(file, target, self.0.as_ref(), self.1.as_ref())
    }
}

impl<N: AsRef<[u8]>, V: AsRef<[f32]>> Sealed for ExtraPositionData<N, V> {}
impl<N: AsRef<[u8]>, V: AsRef<[f32]>> IntoExtraData for ExtraPositionData<N, V> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        assign_ffi::assign_position_data(file, target, self.0.as_ref(), self.1.as_ref())
    }
}

impl<N: AsRef<[u8]>, V: AsRef<[u32]>> Sealed for ExtraWArray<N, V> {}
impl<N: AsRef<[u8]>, V: AsRef<[u32]>> IntoExtraData for ExtraWArray<N, V> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        assign_ffi::assign_w_array(file, target, self.0.as_ref(), self.1.as_ref())
    }
}

impl<N: AsRef<[u8]>, B: IntoIterator<Item: AsRef<[DecalPoint]>>> Sealed
    for ExtraDecalPlacement<N, B>
{
}
impl<N: AsRef<[u8]>, B: IntoIterator<Item: AsRef<[DecalPoint]>>> IntoExtraData
    for ExtraDecalPlacement<N, B>
{
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        let (lengths, points, normals) = decal_ffi(self.2);
        assign_ffi::assign_decal_placement_vector_extra_data(
            file,
            target,
            self.0.as_ref(),
            self.1,
            &lengths,
            &points,
            &normals,
        )
    }
}

impl<N: AsRef<[u8]>, K: AsRef<[TextKey]>> Sealed for ExtraTextKeys<N, K> {}
impl<N: AsRef<[u8]>, K: AsRef<[TextKey]>> IntoExtraData for ExtraTextKeys<N, K> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        let keys = self.1.as_ref();
        let (bytes, lengths) = joined(keys.iter().map(|k| &k.text));
        let times: Vec<f32> = keys.iter().map(|k| k.time).collect();
        assign_ffi::assign_text_key_extra_data(
            file,
            target,
            self.0.as_ref(),
            &bytes,
            &lengths,
            &times,
        )
    }
}

impl<N: AsRef<[u8]>, L: AsRef<[BoneLod]>> Sealed for ExtraBoneLods<N, L> {}
impl<N: AsRef<[u8]>, L: AsRef<[BoneLod]>> IntoExtraData for ExtraBoneLods<N, L> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        let lods = self.1.as_ref();
        let (bytes, lengths) = joined(lods.iter().map(|l| &l.bone));
        let distances: Vec<u32> = lods.iter().map(|l| l.distance).collect();
        assign_ffi::assign_bone_lod_extra_data(
            file,
            target,
            self.0.as_ref(),
            &bytes,
            &lengths,
            &distances,
        )
    }
}

impl<N: AsRef<[u8]>> Sealed for ExtraDistantObject<N> {}
impl<N: AsRef<[u8]>> IntoExtraData for ExtraDistantObject<N> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        assign_ffi::assign_distant_object_extra_data(file, target, self.0.as_ref(), self.1)
    }
}

impl<N: AsRef<[u8]>> Sealed for ExtraDistantObjectLargeRef<N> {}
impl<N: AsRef<[u8]>> IntoExtraData for ExtraDistantObjectLargeRef<N> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        assign_ffi::assign_distant_object_large_ref_extra_data(
            file,
            target,
            self.0.as_ref(),
            self.1,
        )
    }
}

impl<N: AsRef<[u8]>, T: AsRef<[BoneTranslation]>> Sealed for ExtraBoneTranslations<N, T> {}
impl<N: AsRef<[u8]>, T: AsRef<[BoneTranslation]>> IntoExtraData for ExtraBoneTranslations<N, T> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        let (bytes, lengths, vectors) = bone_translations_ffi(self.1.as_ref());
        assign_ffi::assign_bone_translations(
            file,
            target,
            self.0.as_ref(),
            &bytes,
            &lengths,
            &vectors,
        )
    }
}
