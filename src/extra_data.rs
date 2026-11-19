//! Extra data blocks: values attached to objects (see
//! [`View::extra_data`](crate::handle::View::extra_data) and
//! [`Edit::assign_extra_data`](crate::handle::Edit::assign_extra_data)).
//!
//! There are two families of types:
//! - **Blocks**, for reading, named after the block types: [`NiStringExtraData`], [`BsxFlags`],
//!   [`BsClothExtraData`], ... Get one from a block with
//!   [`NiObject::downcast`](crate::objects::NiObject::downcast). Each derefs to its base class, so
//!   [`BsxFlags`] has [`NiIntegerExtraData::value`] and every `NiExtraData` has `name()`.
//! - **Values**, for writing, prefixed `Extra`: [`ExtraString`], [`ExtraBsxFlags`],
//!   [`ExtraBsCloth`], ... Pass one to `assign_extra_data` to create the matching block.
//!
//! Most extra data derives from [`NiExtraData`] and has a name. Fallout 4's cloth and collision
//! query proxy data derive from [`BsExtraData`] instead and have none.

use std::pin::Pin;

use bstr::{BStr, BString, ByteSlice};

use nifly_sys::{extra_data::ffi, extra_data_types::ffi as types_ffi, nif_file::ffi as nif_ffi};

use crate::{
    file::{block_index_to_ffi, index_from_ffi, Index},
    handle::{Edit, View},
    object3d::{BoundingSphere, Matrix3, Transform, Vec3},
    objects::{NiObject, Sealed},
};

ni_types! { ffi;
    /// Base class of named extra data. Other `NiExtraData` types the crate doesn't wrap yet can
    /// still be downcast to this to read their name.
    NiExtraData(NiExtraData): NiObject => upcast_ni_extra_data, downcast_ni_extra_data;
    /// A string value.
    NiStringExtraData(NiStringExtraData): NiExtraData => upcast_ni_string_extra_data, downcast_ni_string_extra_data;
    /// An unsigned integer value.
    NiIntegerExtraData(NiIntegerExtraData): NiExtraData => upcast_ni_integer_extra_data, downcast_ni_integer_extra_data;
    /// Bethesda's per-file flags (havok, ragdoll, editor marker, ...), named "BSX". The flags are
    /// [`NiIntegerExtraData::value`].
    BsxFlags(BSXFlags): NiIntegerExtraData => upcast_bsx_flags, downcast_bsx_flags;
    /// A float value.
    NiFloatExtraData(NiFloatExtraData): NiExtraData => upcast_ni_float_extra_data, downcast_ni_float_extra_data;
    /// A boolean value.
    NiBooleanExtraData(NiBooleanExtraData): NiExtraData => upcast_ni_boolean_extra_data, downcast_ni_boolean_extra_data;
    /// A list of unsigned integers.
    NiIntegersExtraData(NiIntegersExtraData): NiExtraData => upcast_ni_integers_extra_data, downcast_ni_integers_extra_data;
    /// Raw bytes.
    NiBinaryExtraData(NiBinaryExtraData): NiExtraData => upcast_ni_binary_extra_data, downcast_ni_binary_extra_data;
    /// A list of floats.
    NiFloatsExtraData(NiFloatsExtraData): NiExtraData => upcast_ni_floats_extra_data, downcast_ni_floats_extra_data;
    /// A list of strings.
    NiStringsExtraData(NiStringsExtraData): NiExtraData => upcast_ni_strings_extra_data, downcast_ni_strings_extra_data;
    /// The bone names a Fallout 76 or Starfield shape's skin binds to. A Starfield
    /// [`BsGeometry`](crate::geometry::BsGeometry)'s [bone list](crate::handle::View::bone_list) reads
    /// the first one, and [`set_bone_list`](crate::handle::Edit::set_bone_list) writes it.
    SkinAttach(SkinAttach): NiExtraData => upcast_skin_attach, downcast_skin_attach;
    /// The objects of a Fallout 4 or 76 precombined mesh: where each one's geometry is in a CSG
    /// file, and the instances placing it. The vertices and triangles aren't in the file.
    BsPackedCombinedSharedGeomDataExtra(BSPackedCombinedSharedGeomDataExtra): NiExtraData => upcast_bs_packed_combined_shared_geom_data_extra, downcast_bs_packed_combined_shared_geom_data_extra;
    /// Where and how actors use a piece of furniture, usually named "FRN".
    BsFurnitureMarker(BSFurnitureMarker): NiExtraData => upcast_bs_furniture_marker, downcast_bs_furniture_marker;
    /// The [`BsFurnitureMarker`] type used from Skyrim on.
    BsFurnitureMarkerNode(BSFurnitureMarkerNode): BsFurnitureMarker => upcast_bs_furniture_marker_node, downcast_bs_furniture_marker_node;
    /// `BSConnectPoint::Parents`: points other models attach to (Fallout 4 and later), usually
    /// named "CPA".
    BsConnectPointParents(BSConnectPointParents): NiExtraData => upcast_bs_connect_point_parents, downcast_bs_connect_point_parents;
    /// `BSConnectPoint::Children`: the connect points this model attaches to (Fallout 4 and
    /// later), usually named "CPT".
    BsConnectPointChildren(BSConnectPointChildren): NiExtraData => upcast_bs_connect_point_children, downcast_bs_connect_point_children;
    /// How the inventory shows the model, usually named "INV".
    BsInvMarker(BSInvMarker): NiExtraData => upcast_bs_inv_marker, downcast_bs_inv_marker;
    /// A bounding box, usually named "BBX".
    BsBound(BSBound): NiExtraData => upcast_bs_bound, downcast_bs_bound;
    /// The model's behavior graph (`.hkx`), usually named "BGED".
    BsBehaviorGraphExtraData(BSBehaviorGraphExtraData): NiExtraData => upcast_bs_behavior_graph_extra_data, downcast_bs_behavior_graph_extra_data;
    /// A color.
    NiColorExtraData(NiColorExtraData): NiExtraData => upcast_ni_color_extra_data, downcast_ni_color_extra_data;
    /// A four-component vector.
    NiVectorExtraData(NiVectorExtraData): NiExtraData => upcast_ni_vector_extra_data, downcast_ni_vector_extra_data;
    /// A list of vertex weights.
    NiVertWeightsExtraData(NiVertWeightsExtraData): NiExtraData => upcast_ni_vert_weights_extra_data, downcast_ni_vert_weights_extra_data;
    /// The centers of the eyes, as a list of floats.
    BsEyeCenterExtraData(BSEyeCenterExtraData): NiExtraData => upcast_bs_eye_center_extra_data, downcast_bs_eye_center_extra_data;
    /// A list of values stored as 16-bit floats.
    BsPositionData(BSPositionData): NiExtraData => upcast_bs_position_data, downcast_bs_position_data;
    /// A list of unsigned integers.
    BsWArray(BSWArray): NiExtraData => upcast_bs_w_array, downcast_bs_w_array;
    /// Where decals can be placed: blocks of points with normals. Also an
    /// [`NiFloatExtraData`] with a float [`value`](NiFloatExtraData::value).
    BsDecalPlacementVectorExtraData(BSDecalPlacementVectorExtraData): NiFloatExtraData => upcast_bs_decal_placement_vector_extra_data, downcast_bs_decal_placement_vector_extra_data;
    /// Level-of-detail distances for bones.
    BsBoneLodExtraData(BSBoneLODExtraData): NiExtraData => upcast_bs_bone_lod_extra_data, downcast_bs_bone_lod_extra_data;
    /// Flags for the object's distant (LOD) version.
    BsDistantObjectExtraData(BSDistantObjectExtraData): NiExtraData => upcast_bs_distant_object_extra_data, downcast_bs_distant_object_extra_data;
    /// Whether the object is a large reference, drawn from further away.
    BsDistantObjectLargeRefExtraData(BSDistantObjectLargeRefExtraData): NiExtraData => upcast_bs_distant_object_large_ref_extra_data, downcast_bs_distant_object_large_ref_extra_data;
    /// Translations for named bones.
    BoneTranslations(BoneTranslations): NiExtraData => upcast_bone_translations, downcast_bone_translations;

    /// Base class of Bethesda's unnamed extra data (Fallout 4 and later). Unlike
    /// [`NiExtraData`], it has no name.
    BsExtraData(BSExtraData): NiObject => upcast_bs_extra_data, downcast_bs_extra_data;
    /// Fallout 4 cloth simulation data: a raw Havok (`.hkx`) blob.
    BsClothExtraData(BSClothExtraData): BsExtraData => upcast_bs_cloth_extra_data, downcast_bs_cloth_extra_data;
    /// Collision query proxy data: a raw blob.
    BsCollisionQueryProxyExtraData(BSCollisionQueryProxyExtraData): BsExtraData => upcast_bs_collision_query_proxy_extra_data, downcast_bs_collision_query_proxy_extra_data;
}

impl NiExtraData {
    /// The extra data's name, as stored. Empty for file versions before 10.0.1.0, which don't
    /// store one.
    pub fn name(&self) -> &BStr {
        ffi::extra_data_name(&self.0).as_bstr()
    }

    /// Sets the [name](Self::name).
    pub fn set_name(self: Pin<&mut Self>, name: impl AsRef<[u8]>) {
        ffi::set_extra_data_name(self.ffi_mut(), name.as_ref());
    }

    /// The size field of the old extra data chain (see [`View::next_extra_data`]). Saved only in
    /// files up to version 4.2.2.0.
    pub fn num_bytes(&self) -> u32 {
        ffi::extra_data_num_bytes(&self.0)
    }

    /// Sets [`num_bytes`](Self::num_bytes).
    pub fn set_num_bytes(self: Pin<&mut Self>, v: u32) {
        ffi::set_extra_data_num_bytes(self.ffi_mut(), v);
    }
}

impl NiStringExtraData {
    /// The string value, as stored.
    pub fn value(&self) -> &BStr {
        ffi::string_extra_data_value(&self.0).as_bstr()
    }

    pub fn set_value(self: Pin<&mut Self>, value: impl AsRef<[u8]>) {
        ffi::set_string_extra_data(self.ffi_mut(), value.as_ref());
    }
}

impl NiIntegerExtraData {
    pub fn value(&self) -> u32 {
        ffi::integer_extra_data_value(&self.0)
    }

    pub fn set_value(self: Pin<&mut Self>, value: u32) {
        ffi::set_integer_extra_data(self.ffi_mut(), value);
    }
}

impl NiFloatExtraData {
    pub fn value(&self) -> f32 {
        ffi::float_extra_data_value(&self.0)
    }

    pub fn set_value(self: Pin<&mut Self>, value: f32) {
        ffi::set_float_extra_data(self.ffi_mut(), value);
    }
}

impl NiBooleanExtraData {
    pub fn value(&self) -> bool {
        ffi::boolean_extra_data_value(&self.0)
    }

    pub fn set_value(self: Pin<&mut Self>, value: bool) {
        ffi::set_boolean_extra_data(self.ffi_mut(), value);
    }
}

impl NiIntegersExtraData {
    pub fn values(&self) -> &[u32] {
        ffi::integers_extra_data_values(&self.0)
    }

    pub fn set_values(self: Pin<&mut Self>, values: &[u32]) {
        ffi::set_integers_extra_data(self.ffi_mut(), values);
    }
}

impl NiBinaryExtraData {
    pub fn data(&self) -> &[u8] {
        ffi::binary_extra_data_bytes(&self.0)
    }

    pub fn set_data(self: Pin<&mut Self>, data: &[u8]) {
        ffi::set_binary_extra_data(self.ffi_mut(), data);
    }
}

impl NiFloatsExtraData {
    pub fn values(&self) -> &[f32] {
        ffi::floats_extra_data_values(&self.0)
    }

    pub fn set_values(self: Pin<&mut Self>, values: &[f32]) {
        ffi::set_floats_extra_data(self.ffi_mut(), values);
    }
}

impl NiStringsExtraData {
    /// The strings, as stored.
    pub fn values(&self) -> impl ExactSizeIterator<Item = &BStr> {
        (0..ffi::strings_extra_data_len(&self.0)).map(|i| ffi::strings_extra_data_value(&self.0, i).as_bstr())
    }

    pub fn set_values<S: AsRef<[u8]>>(self: Pin<&mut Self>, values: impl IntoIterator<Item = S>) {
        let (bytes, lengths) = joined(values);
        ffi::set_strings_extra_data(self.ffi_mut(), &bytes, &lengths);
    }
}

/// An object of a [`BsPackedCombinedSharedGeomDataExtra`].
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PackedGeomObject {
    /// CRC32 of the CSG file's name, without its extension.
    pub filename_hash: u32,
    /// Where the geometry starts in the CSG file.
    pub data_offset: u32,
    pub vertex_count: u32,
    pub lod_levels: u32,
    /// The triangles of LODs 0 to 2.
    pub lods: [TriangleRange; 3],
    /// The geometry's vertex format, as stored: attribute flags from bit 44, and sizes and
    /// offsets below.
    pub vertex_desc: u64,
    /// The copies of the geometry.
    pub instances: Vec<PackedGeomInstance>,
}

/// A run of triangles of a [`PackedGeomObject`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TriangleRange {
    pub count: u32,
    pub offset: u32,
}

/// A copy of a [`PackedGeomObject`]'s geometry.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PackedGeomInstance {
    pub grayscale_to_palette_scale: f32,
    /// From the geometry's coordinates to the mesh's.
    pub transform: Transform,
    pub bounds: BoundingSphere,
}

impl Default for PackedGeomInstance {
    /// A scale of 1, the identity transform and empty bounds.
    fn default() -> Self {
        Self { grayscale_to_palette_scale: 1.0, transform: Transform::IDENTITY, bounds: BoundingSphere::default() }
    }
}

impl PackedGeomInstance {
    const FLOATS: usize = 18;

    fn from_floats(f: &[f32; Self::FLOATS]) -> Self {
        let v = |i: usize| Vec3::new(f[i], f[i + 1], f[i + 2]);
        Self {
            grayscale_to_palette_scale: f[0],
            transform: Transform::new(v(10), Matrix3::from_rows(v(1), v(4), v(7)), f[13]),
            bounds: BoundingSphere::new(v(14), f[17]),
        }
    }

    fn push_floats(&self, out: &mut Vec<f32>) {
        let v = |v: Vec3| [v.x, v.y, v.z];
        let t = &self.transform;
        out.push(self.grayscale_to_palette_scale);
        for row in t.rotation.rows {
            out.extend(v(row));
        }
        out.extend(v(t.translation));
        out.push(t.scale);
        out.extend(v(self.bounds.center));
        out.push(self.bounds.radius);
    }
}

/// Objects as the bridge takes them: one struct each, and their instances' floats.
fn packed_objects_ffi(objects: &[PackedGeomObject]) -> (Vec<types_ffi::PackedGeomObjectFfi>, Vec<f32>) {
    let mut floats = Vec::new();
    let objects = objects
        .iter()
        .map(|o| {
            o.instances.iter().for_each(|i| i.push_floats(&mut floats));
            let [l0, l1, l2] = o.lods;
            types_ffi::PackedGeomObjectFfi {
                filename_hash: o.filename_hash,
                data_offset: o.data_offset,
                vertex_count: o.vertex_count,
                lod_levels: o.lod_levels,
                lods: [l0.count, l0.offset, l1.count, l1.offset, l2.count, l2.offset],
                vertex_desc: o.vertex_desc,
                instance_count: o.instances.len(),
            }
        })
        .collect();
    (objects, floats)
}

/// The fields of a [`BsPackedCombinedSharedGeomDataExtra`] besides its objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PackedGeomTotals {
    /// The vertex format, as in [`PackedGeomObject::vertex_desc`].
    pub vertex_desc: u64,
    pub vertex_count: u32,
    pub triangle_count: u32,
    pub flags_1: u32,
    pub flags_2: u32,
}

impl PackedGeomTotals {
    fn from_ffi(h: types_ffi::PackedGeomHeader) -> Self {
        let types_ffi::PackedGeomHeader { vertex_desc, vertex_count, triangle_count, flags_1, flags_2 } = h;
        Self { vertex_desc, vertex_count, triangle_count, flags_1, flags_2 }
    }

    fn to_ffi(self) -> types_ffi::PackedGeomHeader {
        let Self { vertex_desc, vertex_count, triangle_count, flags_1, flags_2 } = self;
        types_ffi::PackedGeomHeader { vertex_desc, vertex_count, triangle_count, flags_1, flags_2 }
    }
}

impl BsPackedCombinedSharedGeomDataExtra {
    pub fn totals(&self) -> PackedGeomTotals {
        PackedGeomTotals::from_ffi(ffi::packed_shared_header(&self.0))
    }

    /// Sets the [totals](Self::totals).
    pub fn set_totals(self: Pin<&mut Self>, totals: PackedGeomTotals) {
        ffi::set_packed_shared_header(self.ffi_mut(), totals.to_ffi());
    }

    pub fn objects(&self) -> Vec<PackedGeomObject> {
        let floats = ffi::packed_shared_instances(&self.0);
        let mut instances = floats.as_chunks::<{ PackedGeomInstance::FLOATS }>().0.iter().map(PackedGeomInstance::from_floats);
        ffi::packed_shared_objects(&self.0)
            .into_iter()
            .map(|o| {
                let lod = |i: usize| TriangleRange { count: o.lods[2 * i], offset: o.lods[2 * i + 1] };
                PackedGeomObject {
                    filename_hash: o.filename_hash,
                    data_offset: o.data_offset,
                    vertex_count: o.vertex_count,
                    lod_levels: o.lod_levels,
                    lods: [lod(0), lod(1), lod(2)],
                    vertex_desc: o.vertex_desc,
                    instances: instances.by_ref().take(o.instance_count).collect(),
                }
            })
            .collect()
    }

    /// Replaces the [objects](Self::objects). The [totals](Self::totals) aren't updated.
    pub fn set_objects(self: Pin<&mut Self>, objects: &[PackedGeomObject]) {
        let (objects, instances) = packed_objects_ffi(objects);
        let ok = ffi::set_packed_shared_objects(self.ffi_mut(), &objects, &instances);
        debug_assert!(ok, "instance counts match their floats");
    }
}

impl SkinAttach {
    /// The bone names, as stored.
    pub fn bones(&self) -> impl ExactSizeIterator<Item = &BStr> {
        (0..ffi::skin_attach_bone_count(&self.0)).map(|i| ffi::skin_attach_bone(&self.0, i).as_bstr())
    }

    pub fn set_bones<S: AsRef<[u8]>>(self: Pin<&mut Self>, bones: impl IntoIterator<Item = S>) {
        let (bytes, lengths) = joined(bones);
        ffi::set_skin_attach_bones(self.ffi_mut(), &bytes, &lengths);
    }
}

impl BsFurnitureMarker {
    pub fn positions(&self) -> Vec<FurniturePosition> {
        ffi::furniture_positions(&self.0).into_iter().map(FurniturePosition::from_ffi).collect()
    }

    /// Replaces the [positions](Self::positions). Which fields are saved depends on the file
    /// version, as described on [`FurniturePosition`].
    pub fn set_positions(self: Pin<&mut Self>, positions: &[FurniturePosition]) {
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

    pub fn set_connect_points(self: Pin<&mut Self>, points: &[ConnectPoint]) {
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
        (0..ffi::connect_point_children_len(&self.0)).map(|i| ffi::connect_point_child(&self.0, i).as_bstr())
    }

    pub fn set_skinned(self: Pin<&mut Self>, skinned: bool) {
        ffi::set_connect_point_skinned(self.ffi_mut(), skinned);
    }

    pub fn set_targets<S: AsRef<[u8]>>(self: Pin<&mut Self>, targets: impl IntoIterator<Item = S>) {
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

    pub fn set_rotation(self: Pin<&mut Self>, [rotation_x, rotation_y, rotation_z]: [u16; 3]) {
        let marker = types_ffi::InvMarkerFfi { rotation_x, rotation_y, rotation_z, ..ffi::inv_marker(&self.0) };
        ffi::set_inv_marker(self.ffi_mut(), marker);
    }

    pub fn set_zoom(self: Pin<&mut Self>, zoom: f32) {
        let marker = types_ffi::InvMarkerFfi { zoom, ..ffi::inv_marker(&self.0) };
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

    pub fn set_center(self: Pin<&mut Self>, center: Vec3) {
        ffi::set_bound_center(self.ffi_mut(), center.to_ffi());
    }

    pub fn set_half_extents(self: Pin<&mut Self>, half_extents: Vec3) {
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

    pub fn set_graph_file(self: Pin<&mut Self>, graph_file: impl AsRef<[u8]>) {
        ffi::set_behavior_graph_file(self.ffi_mut(), graph_file.as_ref());
    }

    pub fn set_controls_base_skeleton(self: Pin<&mut Self>, controls: bool) {
        ffi::set_behavior_graph_controls_base_skeleton(self.ffi_mut(), controls);
    }
}

impl NiColorExtraData {
    /// The color as `[r, g, b, a]`.
    pub fn value(&self) -> [f32; 4] {
        let c = ffi::color_extra_data_value(&self.0);
        [c.x, c.y, c.z, c.w]
    }

    pub fn set_value(self: Pin<&mut Self>, value: [f32; 4]) {
        ffi::set_color_extra_data(self.ffi_mut(), float4(value));
    }
}

impl NiVectorExtraData {
    pub fn value(&self) -> [f32; 4] {
        let v = ffi::vector_extra_data_value(&self.0);
        [v.x, v.y, v.z, v.w]
    }

    pub fn set_value(self: Pin<&mut Self>, value: [f32; 4]) {
        ffi::set_vector_extra_data(self.ffi_mut(), float4(value));
    }
}

impl NiVertWeightsExtraData {
    pub fn weights(&self) -> &[f32] {
        ffi::vert_weights_extra_data_values(&self.0)
    }

    /// Replaces the [weights](Self::weights). Returns `false`, changing nothing, for more than
    /// 65535, as the count is stored in 16 bits.
    pub fn set_weights(self: Pin<&mut Self>, weights: &[f32]) -> bool {
        ffi::set_vert_weights_extra_data(self.ffi_mut(), weights)
    }
}

impl BsEyeCenterExtraData {
    pub fn values(&self) -> &[f32] {
        ffi::eye_center_extra_data_values(&self.0)
    }

    pub fn set_values(self: Pin<&mut Self>, values: &[f32]) {
        ffi::set_eye_center_extra_data(self.ffi_mut(), values);
    }
}

impl BsPositionData {
    /// The values, widened from 16-bit floats.
    pub fn values(&self) -> Vec<f32> {
        ffi::position_data_values(&self.0)
    }

    /// Replaces the [values](Self::values), rounded to 16-bit floats.
    pub fn set_values(self: Pin<&mut Self>, values: &[f32]) {
        ffi::set_position_data(self.ffi_mut(), values);
    }
}

impl BsWArray {
    pub fn values(&self) -> &[u32] {
        ffi::w_array_values(&self.0)
    }

    pub fn set_values(self: Pin<&mut Self>, values: &[u32]) {
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
                    .map(|(p, n)| DecalPoint { position: Vec3::from_ffi(p), normal: Vec3::from_ffi(n) })
                    .collect()
            })
            .collect()
    }

    /// Replaces the [blocks](Self::blocks). Its float value is set with
    /// [`NiFloatExtraData::set_value`]. Returns `false`, changing nothing, for more than 65535
    /// blocks or points in a block, as the counts are stored in 16 bits.
    pub fn set_blocks<B: AsRef<[DecalPoint]>>(self: Pin<&mut Self>, blocks: impl IntoIterator<Item = B>) -> bool {
        let (lengths, points, normals) = decal_ffi(blocks);
        ffi::set_decal_blocks(self.ffi_mut(), &lengths, &points, &normals)
    }
}

impl BsBoneLodExtraData {
    pub fn bone_lods(&self) -> Vec<BoneLod> {
        (0..ffi::bone_lod_count(&self.0))
            .map(|i| BoneLod { distance: ffi::bone_lod_distance(&self.0, i), bone: ffi::bone_lod_bone(&self.0, i).into() })
            .collect()
    }

    pub fn set_bone_lods(self: Pin<&mut Self>, lods: &[BoneLod]) {
        let (bytes, lengths) = joined(lods.iter().map(|l| &l.bone));
        let distances: Vec<u32> = lods.iter().map(|l| l.distance).collect();
        ffi::set_bone_lods(self.ffi_mut(), &bytes, &lengths, &distances);
    }
}

impl BsDistantObjectExtraData {
    pub fn flags(&self) -> u32 {
        ffi::distant_object_flags(&self.0)
    }

    pub fn set_flags(self: Pin<&mut Self>, flags: u32) {
        ffi::set_distant_object_flags(self.ffi_mut(), flags);
    }
}

impl BsDistantObjectLargeRefExtraData {
    pub fn large_ref(&self) -> bool {
        ffi::distant_object_large_ref(&self.0)
    }

    pub fn set_large_ref(self: Pin<&mut Self>, large_ref: bool) {
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

    pub fn set_translations(self: Pin<&mut Self>, translations: &[BoneTranslation]) {
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
    /// Which sides the position can be entered from, as flags: 1 front, 2 behind, 4 right, 8
    /// left, 16 up.
    pub entry_points: u16,
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
            entry_points: p.entry_points,
        }
    }

    fn to_ffi(self) -> types_ffi::FurniturePositionFfi {
        types_ffi::FurniturePositionFfi {
            offset: self.offset.to_ffi(),
            orientation: self.orientation,
            position_ref_1: self.position_ref_1,
            position_ref_2: self.position_ref_2,
            heading: self.heading,
            animation_type: self.animation_type,
            entry_points: self.entry_points,
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

    pub fn set_data(self: Pin<&mut Self>, data: &[u8]) {
        ffi::set_cloth_extra_data(self.ffi_mut(), data);
    }
}

impl BsCollisionQueryProxyExtraData {
    /// The raw data.
    pub fn data(&self) -> &[u8] {
        ffi::collision_query_proxy_extra_data_bytes(&self.0)
    }

    pub fn set_data(self: Pin<&mut Self>, data: &[u8]) {
        ffi::set_collision_query_proxy_extra_data(self.ffi_mut(), data);
    }
}

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
pub struct ExtraBsxFlags(pub u32);

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
        nif_ffi::assign_string_extra_data(file, target, self.0.as_ref(), self.1.as_ref())
    }
}

impl<N: AsRef<[u8]>> Sealed for ExtraInteger<N> {}
impl<N: AsRef<[u8]>> IntoExtraData for ExtraInteger<N> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        nif_ffi::assign_integer_extra_data(file, target, self.0.as_ref(), self.1)
    }
}

impl<N: AsRef<[u8]>> Sealed for ExtraFloat<N> {}
impl<N: AsRef<[u8]>> IntoExtraData for ExtraFloat<N> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        nif_ffi::assign_float_extra_data(file, target, self.0.as_ref(), self.1)
    }
}

impl<N: AsRef<[u8]>> Sealed for ExtraBoolean<N> {}
impl<N: AsRef<[u8]>> IntoExtraData for ExtraBoolean<N> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        nif_ffi::assign_boolean_extra_data(file, target, self.0.as_ref(), self.1)
    }
}

impl<N: AsRef<[u8]>, V: AsRef<[u32]>> Sealed for ExtraIntegers<N, V> {}
impl<N: AsRef<[u8]>, V: AsRef<[u32]>> IntoExtraData for ExtraIntegers<N, V> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        nif_ffi::assign_integers_extra_data(file, target, self.0.as_ref(), self.1.as_ref())
    }
}

impl Sealed for ExtraBsxFlags {}
impl IntoExtraData for ExtraBsxFlags {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        nif_ffi::assign_bsx_flags(file, target, self.0)
    }
}

impl<D: AsRef<[u8]>> Sealed for ExtraBsCloth<D> {}
impl<D: AsRef<[u8]>> IntoExtraData for ExtraBsCloth<D> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        nif_ffi::assign_bs_cloth_extra_data(file, target, self.0.as_ref())
    }
}

impl<D: AsRef<[u8]>> Sealed for ExtraBsCollisionQueryProxy<D> {}
impl<D: AsRef<[u8]>> IntoExtraData for ExtraBsCollisionQueryProxy<D> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        nif_ffi::assign_bs_collision_query_proxy_extra_data(file, target, self.0.as_ref())
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
pub struct ExtraPackedSharedGeom<N: AsRef<[u8]>, O: AsRef<[PackedGeomObject]>>(pub N, pub PackedGeomTotals, pub O);

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
pub struct ExtraConnectPointChildren<N: AsRef<[u8]>, T: IntoIterator<Item: AsRef<[u8]>>>(pub N, pub bool, pub T);

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

/// Strings as one byte list and each one's length, as the bridge takes them.
pub(crate) fn joined<S: AsRef<[u8]>>(strings: impl IntoIterator<Item = S>) -> (Vec<u8>, Vec<usize>) {
    let (mut bytes, mut lengths) = (Vec::new(), Vec::new());
    for s in strings {
        bytes.extend_from_slice(s.as_ref());
        lengths.push(s.as_ref().len());
    }
    (bytes, lengths)
}

impl<N: AsRef<[u8]>, D: AsRef<[u8]>> Sealed for ExtraBinary<N, D> {}
impl<N: AsRef<[u8]>, D: AsRef<[u8]>> IntoExtraData for ExtraBinary<N, D> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        nif_ffi::assign_binary_extra_data(file, target, self.0.as_ref(), self.1.as_ref())
    }
}

impl<N: AsRef<[u8]>, V: AsRef<[f32]>> Sealed for ExtraFloats<N, V> {}
impl<N: AsRef<[u8]>, V: AsRef<[f32]>> IntoExtraData for ExtraFloats<N, V> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        nif_ffi::assign_floats_extra_data(file, target, self.0.as_ref(), self.1.as_ref())
    }
}

impl<N: AsRef<[u8]>, V: IntoIterator<Item: AsRef<[u8]>>> Sealed for ExtraStrings<N, V> {}
impl<N: AsRef<[u8]>, V: IntoIterator<Item: AsRef<[u8]>>> IntoExtraData for ExtraStrings<N, V> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        let (bytes, lengths) = joined(self.1);
        nif_ffi::assign_strings_extra_data(file, target, self.0.as_ref(), &bytes, &lengths)
    }
}

impl<N: AsRef<[u8]>, O: AsRef<[PackedGeomObject]>> Sealed for ExtraPackedSharedGeom<N, O> {}
impl<N: AsRef<[u8]>, O: AsRef<[PackedGeomObject]>> IntoExtraData for ExtraPackedSharedGeom<N, O> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        let (objects, instances) = packed_objects_ffi(self.2.as_ref());
        nif_ffi::assign_packed_shared_geom(file, target, self.0.as_ref(), self.1.to_ffi(), &objects, &instances)
    }
}

impl<N: AsRef<[u8]>, B: IntoIterator<Item: AsRef<[u8]>>> Sealed for ExtraSkinAttach<N, B> {}
impl<N: AsRef<[u8]>, B: IntoIterator<Item: AsRef<[u8]>>> IntoExtraData for ExtraSkinAttach<N, B> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        let (bytes, lengths) = joined(self.1);
        nif_ffi::assign_skin_attach(file, target, self.0.as_ref(), &bytes, &lengths)
    }
}

fn furniture_ffi(positions: &[FurniturePosition]) -> Vec<types_ffi::FurniturePositionFfi> {
    positions.iter().map(|p| p.to_ffi()).collect()
}

impl<N: AsRef<[u8]>, P: AsRef<[FurniturePosition]>> Sealed for ExtraFurnitureMarker<N, P> {}
impl<N: AsRef<[u8]>, P: AsRef<[FurniturePosition]>> IntoExtraData for ExtraFurnitureMarker<N, P> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        nif_ffi::assign_furniture_marker(file, target, self.0.as_ref(), &furniture_ffi(self.1.as_ref()), false)
    }
}

impl<N: AsRef<[u8]>, P: AsRef<[FurniturePosition]>> Sealed for ExtraFurnitureMarkerNode<N, P> {}
impl<N: AsRef<[u8]>, P: AsRef<[FurniturePosition]>> IntoExtraData for ExtraFurnitureMarkerNode<N, P> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        nif_ffi::assign_furniture_marker(file, target, self.0.as_ref(), &furniture_ffi(self.1.as_ref()), true)
    }
}

/// Connect points as the bridge takes them: parent and name strings, then transforms.
fn connect_points_ffi(points: &[ConnectPoint]) -> (Vec<u8>, Vec<usize>, Vec<types_ffi::ConnectPointTransform>) {
    let (bytes, lengths) = joined(points.iter().flat_map(|p| [&p.parent, &p.name]));
    let transforms = points
        .iter()
        .map(|p| {
            let [w, x, y, z] = p.rotation;
            types_ffi::ConnectPointTransform {
                rotation_w: w,
                rotation_x: x,
                rotation_y: y,
                rotation_z: z,
                translation: p.translation.to_ffi(),
                scale: p.scale,
            }
        })
        .collect();
    (bytes, lengths, transforms)
}

impl<N: AsRef<[u8]>, P: AsRef<[ConnectPoint]>> Sealed for ExtraConnectPointParents<N, P> {}
impl<N: AsRef<[u8]>, P: AsRef<[ConnectPoint]>> IntoExtraData for ExtraConnectPointParents<N, P> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        let (bytes, lengths, transforms) = connect_points_ffi(self.1.as_ref());
        nif_ffi::assign_connect_point_parents(file, target, self.0.as_ref(), &bytes, &lengths, &transforms)
    }
}

impl<N: AsRef<[u8]>, T: IntoIterator<Item: AsRef<[u8]>>> Sealed for ExtraConnectPointChildren<N, T> {}
impl<N: AsRef<[u8]>, T: IntoIterator<Item: AsRef<[u8]>>> IntoExtraData for ExtraConnectPointChildren<N, T> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        let (bytes, lengths) = joined(self.2);
        nif_ffi::assign_connect_point_children(file, target, self.0.as_ref(), self.1, &bytes, &lengths)
    }
}

impl<N: AsRef<[u8]>> Sealed for ExtraInvMarker<N> {}
impl<N: AsRef<[u8]>> IntoExtraData for ExtraInvMarker<N> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        let [rotation_x, rotation_y, rotation_z] = self.1;
        let marker = types_ffi::InvMarkerFfi { rotation_x, rotation_y, rotation_z, zoom: self.2 };
        nif_ffi::assign_inv_marker(file, target, self.0.as_ref(), marker)
    }
}

impl<N: AsRef<[u8]>> Sealed for ExtraBsBound<N> {}
impl<N: AsRef<[u8]>> IntoExtraData for ExtraBsBound<N> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        nif_ffi::assign_bs_bound(file, target, self.0.as_ref(), self.1.to_ffi(), self.2.to_ffi())
    }
}

impl<N: AsRef<[u8]>, F: AsRef<[u8]>> Sealed for ExtraBehaviorGraph<N, F> {}
impl<N: AsRef<[u8]>, F: AsRef<[u8]>> IntoExtraData for ExtraBehaviorGraph<N, F> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        nif_ffi::assign_behavior_graph_extra_data(file, target, self.0.as_ref(), self.1.as_ref(), self.2)
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
pub struct ExtraDecalPlacement<N: AsRef<[u8]>, B: IntoIterator<Item: AsRef<[DecalPoint]>>>(pub N, pub f32, pub B);

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

fn float4([x, y, z, w]: [f32; 4]) -> types_ffi::Float4 {
    types_ffi::Float4 { x, y, z, w }
}

impl<N: AsRef<[u8]>> Sealed for ExtraColor<N> {}
impl<N: AsRef<[u8]>> IntoExtraData for ExtraColor<N> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        nif_ffi::assign_color_extra_data(file, target, self.0.as_ref(), float4(self.1))
    }
}

impl<N: AsRef<[u8]>> Sealed for ExtraVector<N> {}
impl<N: AsRef<[u8]>> IntoExtraData for ExtraVector<N> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        nif_ffi::assign_vector_extra_data(file, target, self.0.as_ref(), float4(self.1))
    }
}

impl<N: AsRef<[u8]>, V: AsRef<[f32]>> Sealed for ExtraVertWeights<N, V> {}
impl<N: AsRef<[u8]>, V: AsRef<[f32]>> IntoExtraData for ExtraVertWeights<N, V> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        nif_ffi::assign_vert_weights_extra_data(file, target, self.0.as_ref(), self.1.as_ref())
    }
}

impl<N: AsRef<[u8]>, V: AsRef<[f32]>> Sealed for ExtraEyeCenter<N, V> {}
impl<N: AsRef<[u8]>, V: AsRef<[f32]>> IntoExtraData for ExtraEyeCenter<N, V> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        nif_ffi::assign_eye_center_extra_data(file, target, self.0.as_ref(), self.1.as_ref())
    }
}

impl<N: AsRef<[u8]>, V: AsRef<[f32]>> Sealed for ExtraPositionData<N, V> {}
impl<N: AsRef<[u8]>, V: AsRef<[f32]>> IntoExtraData for ExtraPositionData<N, V> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        nif_ffi::assign_position_data(file, target, self.0.as_ref(), self.1.as_ref())
    }
}

impl<N: AsRef<[u8]>, V: AsRef<[u32]>> Sealed for ExtraWArray<N, V> {}
impl<N: AsRef<[u8]>, V: AsRef<[u32]>> IntoExtraData for ExtraWArray<N, V> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        nif_ffi::assign_w_array(file, target, self.0.as_ref(), self.1.as_ref())
    }
}

/// Decal blocks as the bridge takes them: each block's length, then all points and normals.
type DecalFfi = (Vec<usize>, Vec<nifly_sys::object_3d::ffi::Vec3>, Vec<nifly_sys::object_3d::ffi::Vec3>);

fn decal_ffi<B: AsRef<[DecalPoint]>>(blocks: impl IntoIterator<Item = B>) -> DecalFfi {
    let (mut lengths, mut points, mut normals) = (Vec::new(), Vec::new(), Vec::new());
    for block in blocks {
        let block = block.as_ref();
        lengths.push(block.len());
        points.extend(block.iter().map(|p| p.position.to_ffi()));
        normals.extend(block.iter().map(|p| p.normal.to_ffi()));
    }
    (lengths, points, normals)
}

impl<N: AsRef<[u8]>, B: IntoIterator<Item: AsRef<[DecalPoint]>>> Sealed for ExtraDecalPlacement<N, B> {}
impl<N: AsRef<[u8]>, B: IntoIterator<Item: AsRef<[DecalPoint]>>> IntoExtraData for ExtraDecalPlacement<N, B> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        let (lengths, points, normals) = decal_ffi(self.2);
        nif_ffi::assign_decal_placement_vector_extra_data(
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

impl<N: AsRef<[u8]>, L: AsRef<[BoneLod]>> Sealed for ExtraBoneLods<N, L> {}
impl<N: AsRef<[u8]>, L: AsRef<[BoneLod]>> IntoExtraData for ExtraBoneLods<N, L> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        let lods = self.1.as_ref();
        let (bytes, lengths) = joined(lods.iter().map(|l| &l.bone));
        let distances: Vec<u32> = lods.iter().map(|l| l.distance).collect();
        nif_ffi::assign_bone_lod_extra_data(file, target, self.0.as_ref(), &bytes, &lengths, &distances)
    }
}

impl<N: AsRef<[u8]>> Sealed for ExtraDistantObject<N> {}
impl<N: AsRef<[u8]>> IntoExtraData for ExtraDistantObject<N> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        nif_ffi::assign_distant_object_extra_data(file, target, self.0.as_ref(), self.1)
    }
}

impl<N: AsRef<[u8]>> Sealed for ExtraDistantObjectLargeRef<N> {}
impl<N: AsRef<[u8]>> IntoExtraData for ExtraDistantObjectLargeRef<N> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        nif_ffi::assign_distant_object_large_ref_extra_data(file, target, self.0.as_ref(), self.1)
    }
}

impl<N: AsRef<[u8]>, T: AsRef<[BoneTranslation]>> Sealed for ExtraBoneTranslations<N, T> {}
impl<N: AsRef<[u8]>, T: AsRef<[BoneTranslation]>> IntoExtraData for ExtraBoneTranslations<N, T> {
    fn assign(self, file: Pin<&mut nif_ffi::NifFile>, target: u32) -> u32 {
        let (bytes, lengths, vectors) = bone_translations_ffi(self.1.as_ref());
        nif_ffi::assign_bone_translations(file, target, self.0.as_ref(), &bytes, &lengths, &vectors)
    }
}

/// Bone translations as the bridge takes them: bone names, then translations.
fn bone_translations_ffi(
    translations: &[BoneTranslation],
) -> (Vec<u8>, Vec<usize>, Vec<nifly_sys::object_3d::ffi::Vec3>) {
    let (bytes, lengths) = joined(translations.iter().map(|t| &t.bone));
    let vectors = translations.iter().map(|t| t.translation.to_ffi()).collect();
    (bytes, lengths, vectors)
}

/// The old extra data chain of an [`NiExtraData`].
impl<'a> View<'a, NiExtraData> {
    /// The next extra data in the chain files up to version 4.2.2.0 use instead of a list, or
    /// `None` if this is the last.
    pub fn next_extra_data(&self) -> Option<View<'a, NiExtraData>> {
        self.resolve(index_from_ffi(ffi::extra_data_next(&self.get().0)))
    }
}

impl Edit<'_, NiExtraData> {
    /// Like [`View::next_extra_data`], for editing.
    pub fn next_extra_data(&mut self) -> Option<Edit<'_, NiExtraData>> {
        let index = self.view().next_extra_data()?.index();
        self.resolve(Some(index))
    }

    /// Sets the [next extra data](View::next_extra_data) to the block at `next`, or none if
    /// `None`. Saved only in files up to version 4.2.2.0. Returns `false`, changing nothing, if
    /// that block isn't an [`NiExtraData`] or the chain would loop back to this one.
    pub fn set_next_extra_data(&mut self, next: Option<Index>) -> bool {
        let next = match next {
            Some(n) => match block_index_to_ffi(n) {
                Some(n) => n,
                None => return false,
            },
            None => u32::MAX,
        };
        let data = self.index() as u32;
        nif_ffi::set_extra_data_next(self.file_mut().inner.pin_mut(), data, next)
    }
}
