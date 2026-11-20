//! Bethesda extra data: markers, connect points, bounds, bone data and others.

use super::*;

impl SkinAttach {
    /// The bone names, as stored.
    pub fn bones(&self) -> impl ExactSizeIterator<Item = &BStr> {
        (0..ffi::skin_attach_bone_count(&self.0))
            .map(|i| ffi::skin_attach_bone(&self.0, i).as_bstr())
    }

    pub fn set_bones<S: AsRef<[u8]>>(&mut self, bones: impl IntoIterator<Item = S>) {
        let (bytes, lengths) = joined(bones);
        ffi::set_skin_attach_bones(self.ffi_mut(), &bytes, &lengths);
    }
}

impl BsFurnitureMarker {
    pub fn positions(&self) -> Vec<FurniturePosition> {
        ffi::furniture_positions(&self.0)
            .into_iter()
            .map(FurniturePosition::from_ffi)
            .collect()
    }

    /// Replaces the [positions](Self::positions). Which fields are saved depends on the file
    /// version, as described on [`FurniturePosition`].
    pub fn set_positions(&mut self, positions: &[FurniturePosition]) {
        ffi::set_furniture_positions(self.ffi_mut(), &furniture_ffi(positions));
    }
}

impl BsConnectPointParents {
    pub fn connect_points(&self) -> Vec<ConnectPoint> {
        (0..ffi::connect_point_count(&self.0))
            .map(|i| {
                let t = ffi::connect_point_transform(&self.0, i);
                ConnectPoint {
                    parent: ffi::connect_point_parent(&self.0, i).into(),
                    name: ffi::connect_point_name(&self.0, i).into(),
                    rotation: [t.rotation_w, t.rotation_x, t.rotation_y, t.rotation_z],
                    translation: Vec3::from_ffi(t.translation),
                    scale: t.scale,
                }
            })
            .collect()
    }

    pub fn set_connect_points(&mut self, points: &[ConnectPoint]) {
        let (bytes, lengths, transforms) = connect_points_ffi(points);
        ffi::set_connect_points(self.ffi_mut(), &bytes, &lengths, &transforms);
    }
}

impl BsConnectPointChildren {
    /// Whether the model is skinned to the parent's skeleton.
    pub fn skinned(&self) -> bool {
        ffi::connect_point_children_skinned(&self.0)
    }

    /// The names of the parent's connect points this model attaches to.
    pub fn targets(&self) -> impl ExactSizeIterator<Item = &BStr> {
        (0..ffi::connect_point_children_len(&self.0))
            .map(|i| ffi::connect_point_child(&self.0, i).as_bstr())
    }

    pub fn set_skinned(&mut self, skinned: bool) {
        ffi::set_connect_point_skinned(self.ffi_mut(), skinned);
    }

    pub fn set_targets<S: AsRef<[u8]>>(&mut self, targets: impl IntoIterator<Item = S>) {
        let (bytes, lengths) = joined(targets);
        ffi::set_connect_point_targets(self.ffi_mut(), &bytes, &lengths);
    }
}

impl BsInvMarker {
    /// The rotation around the X, Y and Z axes, in thousandths of a radian.
    pub fn rotation(&self) -> [u16; 3] {
        let m = ffi::inv_marker(&self.0);
        [m.rotation_x, m.rotation_y, m.rotation_z]
    }

    pub fn zoom(&self) -> f32 {
        ffi::inv_marker(&self.0).zoom
    }

    pub fn set_rotation(&mut self, [rotation_x, rotation_y, rotation_z]: [u16; 3]) {
        let marker = types_ffi::InvMarkerFfi {
            rotation_x,
            rotation_y,
            rotation_z,
            ..ffi::inv_marker(&self.0)
        };
        ffi::set_inv_marker(self.ffi_mut(), marker);
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        let marker = types_ffi::InvMarkerFfi {
            zoom,
            ..ffi::inv_marker(&self.0)
        };
        ffi::set_inv_marker(self.ffi_mut(), marker);
    }
}

impl BsBound {
    pub fn center(&self) -> Vec3 {
        Vec3::from_ffi(ffi::bound_center(&self.0))
    }

    /// Half the box's size along each axis.
    pub fn half_extents(&self) -> Vec3 {
        Vec3::from_ffi(ffi::bound_half_extents(&self.0))
    }

    pub fn set_center(&mut self, center: Vec3) {
        ffi::set_bound_center(self.ffi_mut(), center.to_ffi());
    }

    pub fn set_half_extents(&mut self, half_extents: Vec3) {
        ffi::set_bound_half_extents(self.ffi_mut(), half_extents.to_ffi());
    }
}

impl BsBehaviorGraphExtraData {
    /// The behavior graph's path, as stored.
    pub fn graph_file(&self) -> &BStr {
        ffi::behavior_graph_file(&self.0).as_bstr()
    }

    /// Whether the graph controls the base skeleton.
    pub fn controls_base_skeleton(&self) -> bool {
        ffi::behavior_graph_controls_base_skeleton(&self.0)
    }

    pub fn set_graph_file(&mut self, graph_file: impl AsRef<[u8]>) {
        ffi::set_behavior_graph_file(self.ffi_mut(), graph_file.as_ref());
    }

    pub fn set_controls_base_skeleton(&mut self, controls: bool) {
        ffi::set_behavior_graph_controls_base_skeleton(self.ffi_mut(), controls);
    }
}

impl NiColorExtraData {
    /// The color as `[r, g, b, a]`.
    pub fn value(&self) -> [f32; 4] {
        let c = ffi::color_extra_data_value(&self.0);
        [c.x, c.y, c.z, c.w]
    }

    pub fn set_value(&mut self, value: [f32; 4]) {
        ffi::set_color_extra_data(self.ffi_mut(), float4(value));
    }
}

impl NiVectorExtraData {
    pub fn value(&self) -> [f32; 4] {
        let v = ffi::vector_extra_data_value(&self.0);
        [v.x, v.y, v.z, v.w]
    }

    pub fn set_value(&mut self, value: [f32; 4]) {
        ffi::set_vector_extra_data(self.ffi_mut(), float4(value));
    }
}

impl NiVertWeightsExtraData {
    pub fn weights(&self) -> &[f32] {
        ffi::vert_weights_extra_data_values(&self.0)
    }

    /// Replaces the [weights](Self::weights). Returns `false`, changing nothing, for more than
    /// 65535, as the count is stored in 16 bits.
    pub fn set_weights(&mut self, weights: &[f32]) -> bool {
        ffi::set_vert_weights_extra_data(self.ffi_mut(), weights)
    }
}

impl BsEyeCenterExtraData {
    pub fn values(&self) -> &[f32] {
        ffi::eye_center_extra_data_values(&self.0)
    }

    pub fn set_values(&mut self, values: &[f32]) {
        ffi::set_eye_center_extra_data(self.ffi_mut(), values);
    }
}

impl BsPositionData {
    /// The values, widened from 16-bit floats.
    pub fn values(&self) -> Vec<f32> {
        ffi::position_data_values(&self.0)
    }

    /// Replaces the [values](Self::values), rounded to 16-bit floats.
    pub fn set_values(&mut self, values: &[f32]) {
        ffi::set_position_data(self.ffi_mut(), values);
    }
}

impl BsWArray {
    pub fn values(&self) -> &[u32] {
        ffi::w_array_values(&self.0)
    }

    pub fn set_values(&mut self, values: &[u32]) {
        ffi::set_w_array(self.ffi_mut(), values);
    }
}

impl BsDecalPlacementVectorExtraData {
    /// The blocks of points, each with its normal.
    pub fn blocks(&self) -> Vec<Vec<DecalPoint>> {
        (0..ffi::decal_block_count(&self.0))
            .map(|b| {
                let points = ffi::decal_block_points(&self.0, b);
                let normals = ffi::decal_block_normals(&self.0, b);
                points
                    .into_iter()
                    .zip(normals)
                    .map(|(p, n)| DecalPoint {
                        position: Vec3::from_ffi(p),
                        normal: Vec3::from_ffi(n),
                    })
                    .collect()
            })
            .collect()
    }

    /// Replaces the [blocks](Self::blocks). Its float value is set with
    /// [`NiFloatExtraData::set_value`]. Returns `false`, changing nothing, for more than 65535
    /// blocks or points in a block, as the counts are stored in 16 bits.
    pub fn set_blocks<B: AsRef<[DecalPoint]>>(
        &mut self,
        blocks: impl IntoIterator<Item = B>,
    ) -> bool {
        let (lengths, points, normals) = decal_ffi(blocks);
        ffi::set_decal_blocks(self.ffi_mut(), &lengths, &points, &normals)
    }
}

impl NiTextKeyExtraData {
    pub fn text_keys(&self) -> Vec<TextKey> {
        (0..ffi::text_key_count(&self.0))
            .map(|i| TextKey {
                time: ffi::text_key_time(&self.0, i),
                text: ffi::text_key_text(&self.0, i).into(),
            })
            .collect()
    }

    pub fn set_text_keys(&mut self, keys: &[TextKey]) {
        let (bytes, lengths) = joined(keys.iter().map(|k| &k.text));
        let times: Vec<f32> = keys.iter().map(|k| k.time).collect();
        ffi::set_text_keys(self.ffi_mut(), &bytes, &lengths, &times);
    }
}

/// A named point in an animation's time, in an [`NiTextKeyExtraData`].
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TextKey {
    pub time: f32,
    pub text: BString,
}

impl BsBoneLodExtraData {
    pub fn bone_lods(&self) -> Vec<BoneLod> {
        (0..ffi::bone_lod_count(&self.0))
            .map(|i| BoneLod {
                distance: ffi::bone_lod_distance(&self.0, i),
                bone: ffi::bone_lod_bone(&self.0, i).into(),
            })
            .collect()
    }

    pub fn set_bone_lods(&mut self, lods: &[BoneLod]) {
        let (bytes, lengths) = joined(lods.iter().map(|l| &l.bone));
        let distances: Vec<u32> = lods.iter().map(|l| l.distance).collect();
        ffi::set_bone_lods(self.ffi_mut(), &bytes, &lengths, &distances);
    }
}

impl BsDistantObjectExtraData {
    pub fn raw_flags(&self) -> u32 {
        ffi::distant_object_flags(&self.0)
    }

    pub fn set_raw_flags(&mut self, flags: u32) {
        ffi::set_distant_object_flags(self.ffi_mut(), flags);
    }
}

impl BsDistantObjectLargeRefExtraData {
    pub fn large_ref(&self) -> bool {
        ffi::distant_object_large_ref(&self.0)
    }

    pub fn set_large_ref(&mut self, large_ref: bool) {
        ffi::set_distant_object_large_ref(self.ffi_mut(), large_ref);
    }
}

impl BoneTranslations {
    pub fn translations(&self) -> Vec<BoneTranslation> {
        (0..ffi::bone_translation_count(&self.0))
            .map(|i| BoneTranslation {
                bone: ffi::bone_translation_bone(&self.0, i).into(),
                translation: Vec3::from_ffi(ffi::bone_translation(&self.0, i)),
            })
            .collect()
    }

    pub fn set_translations(&mut self, translations: &[BoneTranslation]) {
        let (bytes, lengths, values) = bone_translations_ffi(translations);
        ffi::set_bone_translations(self.ffi_mut(), &bytes, &lengths, &values);
    }
}

/// A point in a [`BsDecalPlacementVectorExtraData`] block.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct DecalPoint {
    pub position: Vec3,
    pub normal: Vec3,
}

/// A bone's level-of-detail distance, in a [`BsBoneLodExtraData`].
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BoneLod {
    pub distance: u32,
    /// The bone's name.
    pub bone: BString,
}

/// A bone's translation, in a [`BoneTranslations`].
#[derive(Debug, Clone, PartialEq, Default)]
pub struct BoneTranslation {
    /// The bone's name.
    pub bone: BString,
    pub translation: Vec3,
}

/// A position in a [`BsFurnitureMarker`]. Oblivion and Fallout 3 files store `orientation` and
/// the two position references; Skyrim and later files store `heading`, `animation_type` and
/// `entry_points` instead. The other fields aren't saved.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct FurniturePosition {
    pub offset: Vec3,
    pub orientation: u16,
    pub position_ref_1: u8,
    pub position_ref_2: u8,
    /// Rotation around the Z axis, in radians.
    pub heading: f32,
    /// 1 sit, 2 sleep, 4 lean.
    pub animation_type: u16,
    /// Which sides the position can be entered from.
    pub entry_points: EntryPoints,
}

bitflags! {
    /// The sides a [`FurniturePosition`] can be entered from. Other bits are kept as stored.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct EntryPoints: u16 {
        const FRONT = 1;
        const BEHIND = 1 << 1;
        const RIGHT = 1 << 2;
        const LEFT = 1 << 3;
        const UP = 1 << 4;
        const _ = !0;
    }
}

impl FurniturePosition {
    fn from_ffi(p: types_ffi::FurniturePositionFfi) -> Self {
        Self {
            offset: Vec3::from_ffi(p.offset),
            orientation: p.orientation,
            position_ref_1: p.position_ref_1,
            position_ref_2: p.position_ref_2,
            heading: p.heading,
            animation_type: p.animation_type,
            entry_points: EntryPoints::from_bits_retain(p.entry_points),
        }
    }

    pub(super) fn to_ffi(self) -> types_ffi::FurniturePositionFfi {
        types_ffi::FurniturePositionFfi {
            offset: self.offset.to_ffi(),
            orientation: self.orientation,
            position_ref_1: self.position_ref_1,
            position_ref_2: self.position_ref_2,
            heading: self.heading,
            animation_type: self.animation_type,
            entry_points: self.entry_points.bits(),
        }
    }
}

/// A point in a [`BsConnectPointParents`] that other models attach to.
#[derive(Debug, Clone, PartialEq)]
pub struct ConnectPoint {
    /// The name of the node the point is relative to.
    pub parent: BString,
    /// The point's name, which a [`BsConnectPointChildren`] refers to.
    pub name: BString,
    /// The rotation as a quaternion `[w, x, y, z]`.
    pub rotation: [f32; 4],
    pub translation: Vec3,
    pub scale: f32,
}

impl BsClothExtraData {
    /// The raw Havok cloth data, as stored in a `.hkx` file.
    pub fn data(&self) -> &[u8] {
        ffi::cloth_extra_data_bytes(&self.0)
    }

    pub fn set_data(&mut self, data: &[u8]) {
        ffi::set_cloth_extra_data(self.ffi_mut(), data);
    }
}

impl BsCollisionQueryProxyExtraData {
    /// The raw data.
    pub fn data(&self) -> &[u8] {
        ffi::collision_query_proxy_extra_data_bytes(&self.0)
    }

    pub fn set_data(&mut self, data: &[u8]) {
        ffi::set_collision_query_proxy_extra_data(self.ffi_mut(), data);
    }
}
