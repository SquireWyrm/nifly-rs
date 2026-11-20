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
//!
//! # Type hierarchy
//!
//! <!-- type-hierarchy:start -->
//!
//! - [`objects::NiObject`](crate::objects::NiObject)
//!   - [`NiExtraData`]
//!     - [`NiStringExtraData`]
//!     - [`NiIntegerExtraData`]
//!       - [`BsxFlags`]
//!     - [`NiFloatExtraData`]
//!       - [`BsDecalPlacementVectorExtraData`]
//!     - [`NiBooleanExtraData`]
//!     - [`NiIntegersExtraData`]
//!     - [`NiBinaryExtraData`]
//!     - [`NiFloatsExtraData`]
//!     - [`NiStringsExtraData`]
//!     - [`SkinAttach`]
//!     - [`BsPackedCombinedSharedGeomDataExtra`]
//!     - [`BsFurnitureMarker`]
//!       - [`BsFurnitureMarkerNode`]
//!     - [`BsConnectPointParents`]
//!     - [`BsConnectPointChildren`]
//!     - [`BsInvMarker`]
//!     - [`BsBound`]
//!     - [`BsBehaviorGraphExtraData`]
//!     - [`NiColorExtraData`]
//!     - [`NiVectorExtraData`]
//!     - [`NiVertWeightsExtraData`]
//!     - [`BsEyeCenterExtraData`]
//!     - [`BsPositionData`]
//!     - [`BsWArray`]
//!     - [`BsBoneLodExtraData`]
//!     - [`BsDistantObjectExtraData`]
//!     - [`BsDistantObjectLargeRefExtraData`]
//!     - [`BoneTranslations`]
//!     - [`NiTextKeyExtraData`]
//!   - [`BsExtraData`]
//!     - [`BsClothExtraData`]
//!     - [`BsCollisionQueryProxyExtraData`]
//!
//! <!-- type-hierarchy:end -->

use std::pin::Pin;

use bitflags::bitflags;
use bstr::{BStr, BString, ByteSlice};

use nifly_sys::{
    extra_data::ffi, extra_data_types::ffi as types_ffi, nif_file::ffi as nif_ffi,
    nif_file_extra_data::ffi as assign_ffi,
};

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
    /// Named points in an animation's time, such as "start" and "end".
    NiTextKeyExtraData(NiTextKeyExtraData): NiExtraData => upcast_ni_text_key_extra_data, downcast_ni_text_key_extra_data;

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
    pub fn set_name(&mut self, name: impl AsRef<[u8]>) {
        ffi::set_extra_data_name(self.ffi_mut(), name.as_ref());
    }

    /// The size field of the old extra data chain (see [`View::next_extra_data`]). Saved only in
    /// files up to version 4.2.2.0.
    pub fn num_bytes(&self) -> u32 {
        ffi::extra_data_num_bytes(&self.0)
    }

    /// Sets [`num_bytes`](Self::num_bytes).
    pub fn set_num_bytes(&mut self, v: u32) {
        ffi::set_extra_data_num_bytes(self.ffi_mut(), v);
    }
}

impl NiStringExtraData {
    /// The string value, as stored.
    pub fn value(&self) -> &BStr {
        ffi::string_extra_data_value(&self.0).as_bstr()
    }

    pub fn set_value(&mut self, value: impl AsRef<[u8]>) {
        ffi::set_string_extra_data(self.ffi_mut(), value.as_ref());
    }
}

impl NiIntegerExtraData {
    pub fn value(&self) -> u32 {
        ffi::integer_extra_data_value(&self.0)
    }

    pub fn set_value(&mut self, value: u32) {
        ffi::set_integer_extra_data(self.ffi_mut(), value);
    }
}

impl BsxFlags {
    /// The flags, from the [value](NiIntegerExtraData::value).
    pub fn flags(&self) -> BsxBitflags {
        BsxBitflags::from_bits_retain(self.value())
    }

    pub fn set_flags(&mut self, flags: BsxBitflags) {
        self.set_value(flags.bits());
    }
}

bitflags! {
    /// The flags of a [`BsxFlags`] block. The names are Skyrim's; Oblivion files use bits 0–5
    /// for havok, collision, skeleton, animation, flame nodes and editor markers. Other bits
    /// are kept as stored.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct BsxBitflags: u32 {
        const ANIMATED = 1;
        const HAVOK = 1 << 1;
        const RAGDOLL = 1 << 2;
        const COMPLEX = 1 << 3;
        const ADDON = 1 << 4;
        const EDITOR_MARKER = 1 << 5;
        const DYNAMIC = 1 << 6;
        const ARTICULATED = 1 << 7;
        const NEEDS_TRANSFORM_UPDATES = 1 << 8;
        const EXTERNAL_EMITTANCE = 1 << 9;
        const MAGIC_SHADER_PARTICLES = 1 << 10;
        const LIGHTS = 1 << 11;
        const BREAKABLE = 1 << 12;
        const SEARCHED_BREAKABLE = 1 << 13;
        const _ = !0;
    }
}

impl NiFloatExtraData {
    pub fn value(&self) -> f32 {
        ffi::float_extra_data_value(&self.0)
    }

    pub fn set_value(&mut self, value: f32) {
        ffi::set_float_extra_data(self.ffi_mut(), value);
    }
}

impl NiBooleanExtraData {
    pub fn value(&self) -> bool {
        ffi::boolean_extra_data_value(&self.0)
    }

    pub fn set_value(&mut self, value: bool) {
        ffi::set_boolean_extra_data(self.ffi_mut(), value);
    }
}

impl NiIntegersExtraData {
    pub fn values(&self) -> &[u32] {
        ffi::integers_extra_data_values(&self.0)
    }

    pub fn set_values(&mut self, values: &[u32]) {
        ffi::set_integers_extra_data(self.ffi_mut(), values);
    }
}

impl NiBinaryExtraData {
    pub fn data(&self) -> &[u8] {
        ffi::binary_extra_data_bytes(&self.0)
    }

    pub fn set_data(&mut self, data: &[u8]) {
        ffi::set_binary_extra_data(self.ffi_mut(), data);
    }
}

impl NiFloatsExtraData {
    pub fn values(&self) -> &[f32] {
        ffi::floats_extra_data_values(&self.0)
    }

    pub fn set_values(&mut self, values: &[f32]) {
        ffi::set_floats_extra_data(self.ffi_mut(), values);
    }
}

impl NiStringsExtraData {
    /// The strings, as stored.
    pub fn values(&self) -> impl ExactSizeIterator<Item = &BStr> {
        (0..ffi::strings_extra_data_len(&self.0)).map(|i| ffi::strings_extra_data_value(&self.0, i).as_bstr())
    }

    pub fn set_values<S: AsRef<[u8]>>(&mut self, values: impl IntoIterator<Item = S>) {
        let (bytes, lengths) = joined(values);
        ffi::set_strings_extra_data(self.ffi_mut(), &bytes, &lengths);
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

/// Strings as one byte list and each one's length, as the bridge takes them.
pub(crate) fn joined<S: AsRef<[u8]>>(strings: impl IntoIterator<Item = S>) -> (Vec<u8>, Vec<usize>) {
    let (mut bytes, mut lengths) = (Vec::new(), Vec::new());
    for s in strings {
        bytes.extend_from_slice(s.as_ref());
        lengths.push(s.as_ref().len());
    }
    (bytes, lengths)
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
        assign_ffi::set_extra_data_next(self.file_mut().inner.pin_mut(), data, next)
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

fn float4([x, y, z, w]: [f32; 4]) -> types_ffi::Float4 {
    types_ffi::Float4 { x, y, z, w }
}

fn furniture_ffi(positions: &[FurniturePosition]) -> Vec<types_ffi::FurniturePositionFfi> {
    positions.iter().map(|p| p.to_ffi()).collect()
}

mod packed_geom;
mod markers;
mod values;

pub use self::packed_geom::*;
pub use self::markers::*;
pub use self::values::*;
