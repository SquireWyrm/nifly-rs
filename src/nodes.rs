//! `NiNode` and its subclasses.
//!
//! Each type derefs to its direct base class, so base-class methods are available
//! on every subclass. Use [`NiObject::downcast`](crate::objects::NiObject::downcast) to go the other way.

use std::pin::Pin;

use bitflags::bitflags;
use bstr::{BStr, BString, ByteSlice};
use int_enum::IntEnum;
use nifly_sys::{
    nif_file::ffi::{self as nif_ffi, RefKind, RefList},
    nodes::ffi,
};

use crate::{
    file::{block_index_to_ffi, index_from_ffi, optional_index_to_ffi, Index, NifFile},
    handle::{Edit, View},
    iter::{BlockIter, RefIndices},
    object3d::{BoundingSphere, Matrix3, Transform, Vec3},
    objects::{IsA, NiObject, NiType},
};

use crate::{collision::NiCollisionObject, extra_data::joined, objects::NiAvObject};

ni_types! { ffi;
    /// A scene graph node that can have children, from [`View::children`].
    NiNode(NiNode): NiAvObject => upcast_ni_node, downcast_ni_node;
    BsFadeNode(BSFadeNode): NiNode => upcast_bs_fade_node, downcast_bs_fade_node;
    /// Morrowind only. Marks a node holding the collision mesh.
    RootCollisionNode(RootCollisionNode): NiNode => upcast_root_collision_node, downcast_root_collision_node;
    /// Morrowind only. Marks a node that NPCs try to avoid.
    AvoidNode(AvoidNode): NiNode => upcast_avoid_node, downcast_avoid_node;
    /// Bethesda extension of `NiNode` with animation properties stored in the flags.
    NiBsAnimationNode(NiBSAnimationNode): NiNode => upcast_ni_bs_animation_node, downcast_ni_bs_animation_node;
    /// Bethesda extension of `NiNode` used together with `NiBSPArrayController`.
    NiBsParticleNode(NiBSParticleNode): NiNode => upcast_ni_bs_particle_node, downcast_ni_bs_particle_node;
    BsValueNode(BSValueNode): NiNode => upcast_bs_value_node, downcast_bs_value_node;
    BsLeafAnimNode(BSLeafAnimNode): NiNode => upcast_bs_leaf_anim_node, downcast_bs_leaf_anim_node;
    /// A Skyrim tree's root node, with its trunk and branch bones in [`View::bones_1`] and
    /// [`View::bones_2`], set by [`Edit::set_bones_1`] and [`Edit::set_bones_2`].
    BsTreeNode(BSTreeNode): NiNode => upcast_bs_tree_node, downcast_bs_tree_node;
    BsOrderedNode(BSOrderedNode): NiNode => upcast_bs_ordered_node, downcast_bs_ordered_node;
    /// A node culled by a [`BsMultiBound`], from [`View::multi_bound`].
    BsMultiBoundNode(BSMultiBoundNode): NiNode => upcast_bs_multi_bound_node, downcast_bs_multi_bound_node;
    /// A [`BsMultiBoundNode`] drawing many copies of distant objects (Fallout 76).
    BsDistantObjectInstancedNode(BSDistantObjectInstancedNode): BsMultiBoundNode => upcast_bs_distant_object_instanced_node, downcast_bs_distant_object_instanced_node;
    /// A node showing a range of its children.
    ///
    /// A `BSRangeNode` block itself is read as an unknown block when loading, so its fields can
    /// only be set on a node made with [`NifFile::add_node_of`]. The subclasses are read
    /// normally.
    BsRangeNode(BSRangeNode): NiNode => upcast_bs_range_node, downcast_bs_range_node;
    BsDebrisNode(BSDebrisNode): BsRangeNode => upcast_bs_debris_node, downcast_bs_debris_node;
    BsBlastNode(BSBlastNode): BsRangeNode => upcast_bs_blast_node, downcast_bs_blast_node;
    BsDamageStage(BSDamageStage): BsBlastNode => upcast_bs_damage_stage, downcast_bs_damage_stage;
    /// A node placing other models by reference (Starfield).
    BsWeakReferenceNode(BSWeakReferenceNode): NiNode => upcast_bs_weak_reference_node, downcast_bs_weak_reference_node;
    BsFaceGenNiNode(BSFaceGenNiNode): NiNode => upcast_bs_face_gen_ni_node, downcast_bs_face_gen_ni_node;
    NiBillboardNode(NiBillboardNode): NiNode => upcast_ni_billboard_node, downcast_ni_billboard_node;
    /// A node showing only its [active child](NiSwitchNode::active_index). Supported from
    /// Oblivion on: Morrowind files have no switch flags, which are read anyway, so such a file
    /// loads wrongly.
    NiSwitchNode(NiSwitchNode): NiNode => upcast_ni_switch_node, downcast_ni_switch_node;
    /// A switch node showing one child by distance or screen size, as set by its
    /// [LOD data](View::lod_data). Supported from Oblivion on: Morrowind stores the LOD levels
    /// in the node itself, which isn't read, so such a file loads wrongly.
    NiLodNode(NiLODNode): NiSwitchNode => upcast_ni_lod_node, downcast_ni_lod_node;
    NiBone(NiBone): NiNode => upcast_ni_bone, downcast_ni_bone;
    NiSortAdjustNode(NiSortAdjustNode): NiNode => upcast_ni_sort_adjust_node, downcast_ni_sort_adjust_node;
    BsMasterParticleSystem(BSMasterParticleSystem): NiNode => upcast_bs_master_particle_system, downcast_bs_master_particle_system;
    /// Base class of an [`NiLodNode`]'s LOD data, from [`View::lod_data`].
    NiLodData(NiLODData): NiObject => upcast_ni_lod_data, downcast_ni_lod_data;
    /// LOD data choosing a child by its distance from the camera.
    NiRangeLodData(NiRangeLODData): NiLodData => upcast_ni_range_lod_data, downcast_ni_range_lod_data;
    /// LOD data choosing a child by the proportion of the screen it covers.
    NiScreenLodData(NiScreenLODData): NiLodData => upcast_ni_screen_lod_data, downcast_ni_screen_lod_data;
    /// The culling bound of a [`BsMultiBoundNode`], from [`View::multi_bound`]. Its shape is in
    /// a [`BsMultiBoundData`], from [`View::data`].
    BsMultiBound(BSMultiBound): NiObject => upcast_bs_multi_bound, downcast_bs_multi_bound;
    /// Base class of a [`BsMultiBound`]'s shape, from [`View::data`].
    BsMultiBoundData(BSMultiBoundData): NiObject => upcast_bs_multi_bound_data, downcast_bs_multi_bound_data;
    /// A [`BsMultiBound`] box with any rotation.
    BsMultiBoundObb(BSMultiBoundOBB): BsMultiBoundData => upcast_bs_multi_bound_obb, downcast_bs_multi_bound_obb;
    /// A [`BsMultiBound`] box aligned with the axes.
    BsMultiBoundAabb(BSMultiBoundAABB): BsMultiBoundData => upcast_bs_multi_bound_aabb, downcast_bs_multi_bound_aabb;
    /// A [`BsMultiBound`] sphere.
    BsMultiBoundSphere(BSMultiBoundSphere): BsMultiBoundData => upcast_bs_multi_bound_sphere, downcast_bs_multi_bound_sphere;
    /// A viewpoint, with its frustum and viewport. The scene it renders is on [`View::scene`].
    /// Add one with [`NifFile::add_camera`].
    NiCamera(NiCamera): NiAvObject => upcast_ni_camera, downcast_ni_camera;
    /// Base of [`NiDefaultAvObjectPalette`], the only kind of palette.
    NiAvObjectPalette(NiAVObjectPalette): NiObject => upcast_ni_av_object_palette, downcast_ni_av_object_palette;
    /// Scene objects by name, where an animation's controller manager finds its targets. Its
    /// scene and objects are on [`View::scene`] and [`View::objects`], set by
    /// [`Edit::set_scene`] and [`Edit::set_objects`].
    NiDefaultAvObjectPalette(NiDefaultAVObjectPalette): NiAvObjectPalette => upcast_ni_default_av_object_palette, downcast_ni_default_av_object_palette;
}

impl NiNode {
    pub(crate) fn as_ffi(&self) -> &ffi::NiNode {
        &self.0
    }

    /// Number of child refs, including empty or unresolvable ones.
    /// [`View::children`] skips those.
    pub fn child_count(&self) -> usize {
        ffi::child_count(&self.0)
    }

    /// Whether the node can be deleted without leaving blocks behind: true only if every ref
    /// it holds is empty (children, effects, properties, collision, controller and extra data).
    pub fn can_delete(&self) -> bool {
        ffi::can_delete_node(&self.0)
    }
}

/// Getter and setter pairs over the bridge's field shims, converting with `$to` and `$from`.
macro_rules! fields {
    ($($(#[$doc:meta])* $get:ident, $set:ident: $ty:ty = $ffi_get:ident, $ffi_set:ident;)*) => {$(
        $(#[$doc])*
        pub fn $get(&self) -> $ty {
            ffi::$ffi_get(&self.0)
        }

        #[doc = concat!("Sets [`", stringify!($get), "`](Self::", stringify!($get), ").")]
        pub fn $set(self: Pin<&mut Self>, value: $ty) {
            ffi::$ffi_set(self.ffi_mut(), value);
        }
    )*};
}

impl BsFaceGenNiNode {
    fields! {
        /// A value of unknown meaning.
        unknown, set_unknown: u16 = face_gen_unknown, set_face_gen_unknown;
    }
}

impl BsMasterParticleSystem {
    fields! {
        /// The most emitter objects at once.
        max_emitter_objects, set_max_emitter_objects: u16 = master_particle_max_emitters, set_master_particle_max_emitters;
    }
}

impl BsValueNode {
    fields! {
        /// The node's value, such as the add-on index of an `AddOnNode` node.
        value, set_value: i32 = value_node_value, set_value_node_value;
    }

    pub fn flags(&self) -> ValueNodeFlags {
        ValueNodeFlags::from_bits_retain(ffi::value_node_flags(&self.0))
    }

    pub fn set_flags(self: Pin<&mut Self>, flags: ValueNodeFlags) {
        ffi::set_value_node_flags(self.ffi_mut(), flags.bits());
    }
}

bitflags! {
    /// The flags of a [`BsValueNode`]. Other bits are kept as stored.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct ValueNodeFlags: u8 {
        /// Billboard around the world Z axis.
        const BILLBOARD_WORLD_Z = 1;
        /// Use the player adjustment.
        const USE_PLAYER_ADJUST = 1 << 1;
        const _ = !0;
    }
}

/// Getter and setter pairs for `Vec3` fields.
macro_rules! vec3_fields {
    ($($(#[$doc:meta])* $get:ident, $set:ident = $ffi_get:ident, $ffi_set:ident;)*) => {$(
        $(#[$doc])*
        pub fn $get(&self) -> Vec3 {
            Vec3::from_ffi(ffi::$ffi_get(&self.0))
        }

        #[doc = concat!("Sets [`", stringify!($get), "`](Self::", stringify!($get), ").")]
        pub fn $set(self: Pin<&mut Self>, value: Vec3) {
            ffi::$ffi_set(self.ffi_mut(), value.to_ffi());
        }
    )*};
}

impl BsMultiBoundNode {
    /// How the node is culled, or `None` if the stored value isn't a [`CullingMode`]. Saved from
    /// Skyrim on.
    pub fn culling_mode(&self) -> Option<CullingMode> {
        CullingMode::try_from(ffi::multi_bound_node_culling_mode(&self.0)).ok()
    }

    /// Sets the [culling mode](Self::culling_mode).
    pub fn set_culling_mode(self: Pin<&mut Self>, mode: CullingMode) {
        ffi::set_multi_bound_node_culling_mode(self.ffi_mut(), mode.into());
    }
}

/// How a [`BsMultiBoundNode`] is culled.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum CullingMode {
    /// By its bound and its children's.
    Normal = 0,
    /// Never culled.
    AllPass = 1,
    /// Always culled.
    AllFail = 2,
    /// Like `Normal`, ignoring multi-bounds.
    IgnoreMultiBounds = 3,
    /// Like `Normal`, without updating the multi-bound.
    ForceMultiBoundsNoUpdate = 4,
}

impl BsMultiBoundObb {
    vec3_fields! {
        center, set_center = multi_bound_obb_center, set_multi_bound_obb_center;
        size, set_size = multi_bound_obb_size, set_multi_bound_obb_size;
    }

    pub fn rotation(&self) -> Matrix3 {
        let mut rows = [Vec3::ZERO.to_ffi(); 3];
        ffi::multi_bound_obb_rotation(&self.0, &mut rows);
        Matrix3 { rows: rows.map(Vec3::from_ffi) }
    }

    /// Sets the [rotation](Self::rotation).
    pub fn set_rotation(self: Pin<&mut Self>, rotation: Matrix3) {
        ffi::set_multi_bound_obb_rotation(self.ffi_mut(), &rotation.rows.map(Vec3::to_ffi));
    }
}

impl BsMultiBoundAabb {
    vec3_fields! {
        center, set_center = multi_bound_aabb_center, set_multi_bound_aabb_center;
        /// Half the box's size along each axis.
        half_extent, set_half_extent = multi_bound_aabb_half_extent, set_multi_bound_aabb_half_extent;
    }
}

impl BsMultiBoundSphere {
    vec3_fields! {
        center, set_center = multi_bound_sphere_center, set_multi_bound_sphere_center;
    }

    fields! {
        radius, set_radius: f32 = multi_bound_sphere_radius, set_multi_bound_sphere_radius;
    }
}

/// Identifies a game file by hashes of its name and folder, and its extension.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct ResourceId {
    pub file_hash: u32,
    /// The extension's bytes, such as `*b"nif\0"`.
    pub extension: [u8; 4],
    pub dir_hash: u32,
}

/// The copies of one object in a [`BsDistantObjectInstancedNode`].
#[derive(Clone, Debug, PartialEq, Default)]
pub struct DistantObjectInstance {
    pub resource: ResourceId,
    /// Entries of unknown meaning.
    pub unknown: Vec<DistantObjectUnknown>,
    /// One 4×4 matrix per copy, row by row, as stored.
    pub transforms: Vec<[[f32; 4]; 4]>,
}

/// An entry of unknown meaning in a [`DistantObjectInstance`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DistantObjectUnknown {
    pub unknown_1: u64,
    pub unknown_2: u32,
}

impl ResourceId {
    fn to_ffi(self) -> [u32; 3] {
        [self.file_hash, u32::from_le_bytes(self.extension), self.dir_hash]
    }

    fn from_ffi(r: [u32; 3]) -> Self {
        Self { file_hash: r[0], extension: r[1].to_le_bytes(), dir_hash: r[2] }
    }
}

fn matrices_to_ffi(matrices: &[[[f32; 4]; 4]]) -> Vec<f32> {
    matrices.iter().flatten().flatten().copied().collect()
}

fn matrices_from_ffi(flat: &[f32]) -> Vec<[[f32; 4]; 4]> {
    flat.as_chunks::<16>().0.iter().map(|m| *m.as_chunks::<4>().0.as_array().unwrap()).collect()
}

impl DistantObjectInstance {
    fn ffi_parts(&self) -> ([u32; 3], Vec<f32>) {
        (self.resource.to_ffi(), matrices_to_ffi(&self.transforms))
    }

    fn unknown_ffi(&self) -> Vec<u64> {
        self.unknown.iter().flat_map(|u| [u.unknown_1, u64::from(u.unknown_2)]).collect()
    }
}

impl BsDistantObjectInstancedNode {
    pub fn instance_count(&self) -> usize {
        ffi::distant_instance_count(&self.0)
    }

    /// Instance `index`, or `None` if `index` is out of range.
    pub fn instance(&self, index: usize) -> Option<DistantObjectInstance> {
        let mut r = [0; 3];
        ffi::distant_instance_resource(&self.0, index, &mut r).then_some(())?;
        let transforms = matrices_from_ffi(&ffi::distant_instance_transforms(&self.0, index));
        let unknown = ffi::distant_instance_unknown(&self.0, index)
            .as_chunks::<2>()
            .0
            .iter()
            .map(|&[unknown_1, unknown_2]| DistantObjectUnknown { unknown_1, unknown_2: unknown_2 as u32 })
            .collect();
        Some(DistantObjectInstance { resource: ResourceId::from_ffi(r), unknown, transforms })
    }

    /// Replaces instance `index`. Returns `false` if `index` is out of range.
    pub fn set_instance(mut self: Pin<&mut Self>, index: usize, instance: &DistantObjectInstance) -> bool {
        let (resource, transforms) = instance.ffi_parts();
        ffi::set_distant_instance(self.as_mut().ffi_mut(), index, &resource, &transforms)
            && ffi::set_distant_instance_unknown(self.ffi_mut(), index, &instance.unknown_ffi())
    }

    /// Appends an instance.
    pub fn push_instance(mut self: Pin<&mut Self>, instance: &DistantObjectInstance) {
        let (resource, transforms) = instance.ffi_parts();
        ffi::push_distant_instance(self.as_mut().ffi_mut(), &resource, &transforms);
        let index = self.instance_count() - 1;
        ffi::set_distant_instance_unknown(self.ffi_mut(), index, &instance.unknown_ffi());
    }

    /// A byte of unknown meaning stored with the texture arrays of slot `slot` (0 to 2), or
    /// `None` if `slot` is out of range.
    pub fn texture_arrays_unknown(&self, slot: usize) -> Option<u8> {
        (slot < 3).then(|| ffi::distant_texture_arrays_unknown(&self.0, slot))
    }

    /// Sets [`texture_arrays_unknown`](Self::texture_arrays_unknown). Returns `false` if `slot`
    /// is out of range.
    pub fn set_texture_arrays_unknown(self: Pin<&mut Self>, slot: usize, value: u8) -> bool {
        ffi::set_distant_texture_arrays_unknown(self.ffi_mut(), slot, value)
    }

    /// Removes instance `index`, moving later ones down. Returns `false` if `index` is out of
    /// range.
    pub fn remove_instance(self: Pin<&mut Self>, index: usize) -> bool {
        ffi::remove_distant_instance(self.ffi_mut(), index)
    }

    /// The texture arrays of slot `slot` (0 to 2), each a list of texture paths, or `None` if
    /// `slot` is out of range.
    pub fn texture_arrays(&self, slot: usize) -> Option<Vec<Vec<&BStr>>> {
        (slot < 3).then(|| {
            (0..ffi::distant_texture_array_count(&self.0, slot))
                .map(|a| {
                    (0..ffi::distant_texture_array_len(&self.0, slot, a))
                        .map(|i| ffi::distant_texture_array_path(&self.0, slot, a, i).as_bstr())
                        .collect()
                })
                .collect()
        })
    }

    /// Replaces the [texture arrays](Self::texture_arrays) of slot `slot`. Returns `false`,
    /// changing nothing, if `slot` is out of range.
    pub fn set_texture_arrays<A, P>(mut self: Pin<&mut Self>, slot: usize, arrays: impl IntoIterator<Item = A>) -> bool
    where
        A: IntoIterator<Item = P>,
        P: AsRef<[u8]>,
    {
        if slot >= 3 {
            return false;
        }
        ffi::clear_distant_texture_arrays(self.as_mut().ffi_mut(), slot);
        for array in arrays {
            ffi::add_distant_texture_array(self.as_mut().ffi_mut(), slot);
            for path in array {
                ffi::push_distant_texture_array_path(self.as_mut().ffi_mut(), slot, path.as_ref());
            }
        }
        true
    }
}

/// A model placed by a [`BsWeakReferenceNode`].
#[derive(Clone, Debug, PartialEq, Default)]
pub struct WeakReference {
    /// Saved from stream version 173 on, so not in [`NifVersion::STARFIELD`](crate::file::NifVersion::STARFIELD) files.
    pub form_id: u32,
    pub resource: ResourceId,
    /// One 4×4 matrix per copy, row by row, as stored.
    pub transforms: Vec<[[f32; 4]; 4]>,
    pub materials: Vec<WeakReferenceMaterial>,
}

/// A material of a [`WeakReference`].
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct WeakReferenceMaterial {
    pub biome_form_id: u32,
    pub dir_hash: u32,
    pub file_hash: u32,
    /// Saved null-terminated, so it can't contain a zero byte.
    pub name: BString,
}

/// A water plane placed by a [`BsWeakReferenceNode`].
#[derive(Clone, Debug, PartialEq, Default)]
pub struct WaterReference {
    /// A 4×4 matrix, row by row, as stored.
    pub transform: [[f32; 4]; 4],
    pub resource: ResourceId,
    /// A value of unknown meaning.
    pub unknown: u32,
    pub material: BString,
}

impl BsWeakReferenceNode {
    pub fn weak_references(&self) -> Vec<WeakReference> {
        (0..ffi::weak_ref_count(&self.0))
            .map(|i| {
                let mut r = [0; 3];
                ffi::weak_ref_resource(&self.0, i, &mut r);
                let materials = (0..ffi::weak_ref_material_count(&self.0, i))
                    .map(|m| {
                        let mut ids = [0; 3];
                        ffi::weak_ref_material(&self.0, i, m, &mut ids);
                        let name = ffi::weak_ref_material_name(&self.0, i, m).into();
                        WeakReferenceMaterial { biome_form_id: ids[0], dir_hash: ids[1], file_hash: ids[2], name }
                    })
                    .collect();
                WeakReference {
                    form_id: ffi::weak_ref_form_id(&self.0, i),
                    resource: ResourceId::from_ffi(r),
                    transforms: matrices_from_ffi(&ffi::weak_ref_transforms(&self.0, i)),
                    materials,
                }
            })
            .collect()
    }

    /// Replaces the [weak references](Self::weak_references). Returns `false`, changing nothing,
    /// if a material name contains a zero byte.
    pub fn set_weak_references(mut self: Pin<&mut Self>, references: &[WeakReference]) -> bool {
        if references.iter().flat_map(|r| &r.materials).any(|m| m.name.contains(&0)) {
            return false;
        }
        ffi::clear_weak_refs(self.as_mut().ffi_mut());
        for r in references {
            let transforms = matrices_to_ffi(&r.transforms);
            ffi::push_weak_ref(self.as_mut().ffi_mut(), r.form_id, &r.resource.to_ffi(), &transforms);
            for m in &r.materials {
                let ids = [m.biome_form_id, m.dir_hash, m.file_hash];
                ffi::push_weak_ref_material(self.as_mut().ffi_mut(), &ids, &m.name);
            }
        }
        true
    }

    pub fn water_references(&self) -> Vec<WaterReference> {
        (0..ffi::water_ref_count(&self.0))
            .map(|i| {
                let mut r = [0; 3];
                ffi::water_ref_resource(&self.0, i, &mut r);
                let transform = matrices_from_ffi(&ffi::water_ref_transform(&self.0, i))[0];
                WaterReference {
                    transform,
                    resource: ResourceId::from_ffi(r),
                    unknown: ffi::water_ref_unknown(&self.0, i),
                    material: ffi::water_ref_material(&self.0, i).into(),
                }
            })
            .collect()
    }

    /// Replaces the [water references](Self::water_references).
    pub fn set_water_references(mut self: Pin<&mut Self>, references: &[WaterReference]) {
        ffi::resize_water_refs(self.as_mut().ffi_mut(), references.len());
        for (i, r) in references.iter().enumerate() {
            let transform = matrices_to_ffi(&[r.transform]);
            ffi::set_water_ref(self.as_mut().ffi_mut(), i, &transform, &r.resource.to_ffi(), &r.material);
            ffi::set_water_ref_unknown(self.as_mut().ffi_mut(), i, r.unknown);
        }
    }

    fields! {
        /// A value of unknown meaning, stored between the weak and water references.
        unknown, set_unknown: u32 = weak_ref_unknown, set_weak_ref_unknown;
    }
}

/// The volume an [`NiCamera`] sees, in its own coordinates: the edges of its view plane at
/// distance 1 (at any distance for an orthographic camera), and the near and far distances.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Frustum {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub near: f32,
    pub far: f32,
}

/// The part of the screen an [`NiCamera`] renders to, as fractions of its width and height
/// from the bottom left.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Viewport {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl NiCamera {
    fields! {
        /// Flags with no known use. Saved from Oblivion on.
        raw_flags, set_raw_flags: u16 = camera_flags, set_camera_flags;
        /// Whether the projection is orthographic rather than perspective. Saved from Oblivion
        /// on; Morrowind cameras are always perspective.
        orthographic, set_orthographic: bool = camera_orthographic, set_camera_orthographic;
        /// Scales the distances used to pick levels of detail.
        lod_adjust, set_lod_adjust: f32 = camera_lod_adjust, set_camera_lod_adjust;
        /// A count of screen polygons, which aren't stored; 0 in game files.
        screen_polygon_count, set_screen_polygon_count: u32 = camera_screen_polygons, set_camera_screen_polygons;
        /// A count of screen textures, which aren't stored; 0 in game files. Saved from version
        /// 4.2.1.0 on.
        screen_texture_count, set_screen_texture_count: u32 = camera_screen_textures, set_camera_screen_textures;
    }

    pub fn frustum(&self) -> Frustum {
        let mut v = [0.0; 6];
        ffi::camera_frustum(&self.0, &mut v);
        let [left, right, top, bottom, near, far] = v;
        Frustum { left, right, top, bottom, near, far }
    }

    /// Sets the [frustum](Self::frustum).
    pub fn set_frustum(self: Pin<&mut Self>, f: Frustum) {
        ffi::set_camera_frustum(self.ffi_mut(), &[f.left, f.right, f.top, f.bottom, f.near, f.far]);
    }

    pub fn viewport(&self) -> Viewport {
        let mut v = [0.0; 4];
        ffi::camera_viewport(&self.0, &mut v);
        let [left, right, top, bottom] = v;
        Viewport { left, right, top, bottom }
    }

    /// Sets the [viewport](Self::viewport).
    pub fn set_viewport(self: Pin<&mut Self>, v: Viewport) {
        ffi::set_camera_viewport(self.ffi_mut(), &[v.left, v.right, v.top, v.bottom]);
    }
}

/// The distances from the camera between which an [`NiLodNode`] child is shown.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct LodRange {
    pub near: f32,
    pub far: f32,
}

impl NiRangeLodData {
    vec3_fields! {
        /// The point distances are measured from, in the node's coordinates.
        center, set_center = range_lod_center, set_range_lod_center;
    }

    /// The range of each level, usually one per child, in child order.
    pub fn levels(&self) -> Vec<LodRange> {
        let flat = ffi::range_lod_levels(&self.0);
        flat.as_chunks().0.iter().map(|&[near, far]| LodRange { near, far }).collect()
    }

    /// Sets the [levels](Self::levels). Not checked against the child count.
    pub fn set_levels(self: Pin<&mut Self>, levels: &[LodRange]) {
        let flat: Vec<f32> = levels.iter().flat_map(|r| [r.near, r.far]).collect();
        ffi::set_range_lod_levels(self.ffi_mut(), &flat);
    }
}

impl NiScreenLodData {
    /// The bound whose screen size is measured, in the node's coordinates.
    pub fn bound(&self) -> BoundingSphere {
        BoundingSphere::from_ffi(ffi::screen_lod_bound(&self.0))
    }

    /// Sets the [bound](Self::bound).
    pub fn set_bound(self: Pin<&mut Self>, bound: BoundingSphere) {
        ffi::set_screen_lod_bound(self.ffi_mut(), bound.to_ffi());
    }

    /// The [bound](Self::bound) in world coordinates.
    pub fn world_bound(&self) -> BoundingSphere {
        BoundingSphere::from_ffi(ffi::screen_lod_world_bound(&self.0))
    }

    /// Sets the [world bound](Self::world_bound).
    pub fn set_world_bound(self: Pin<&mut Self>, bound: BoundingSphere) {
        ffi::set_screen_lod_world_bound(self.ffi_mut(), bound.to_ffi());
    }

    /// The screen proportion at which each level starts.
    pub fn proportion_levels(&self) -> Vec<f32> {
        ffi::screen_lod_proportion_levels(&self.0)
    }

    /// Sets the [proportion levels](Self::proportion_levels). Not checked against the child
    /// count.
    pub fn set_proportion_levels(self: Pin<&mut Self>, levels: &[f32]) {
        ffi::set_screen_lod_proportion_levels(self.ffi_mut(), levels);
    }
}

/// An LOD data type that [`Edit::assign_lod_data`] can create.
///
/// This trait is sealed.
pub trait NewLodData: NiType {
    /// Gives `node` a new default `Self` and returns its index (`u32::MAX` on failure).
    #[doc(hidden)]
    fn assign(file: Pin<&mut nif_ffi::NifFile>, node: u32) -> u32;
}

impl NewLodData for NiRangeLodData {
    fn assign(file: Pin<&mut nif_ffi::NifFile>, node: u32) -> u32 {
        nif_ffi::assign_range_lod_data(file, node)
    }
}

impl NewLodData for NiScreenLodData {
    fn assign(file: Pin<&mut nif_ffi::NifFile>, node: u32) -> u32 {
        nif_ffi::assign_screen_lod_data(file, node)
    }
}

/// The shape of a new [`BsMultiBound`], for [`Edit::assign_multi_bound`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MultiBoundShape {
    /// A [`BsMultiBoundAabb`].
    Aabb { center: Vec3, half_extent: Vec3 },
    /// A [`BsMultiBoundObb`].
    Obb { center: Vec3, size: Vec3, rotation: Matrix3 },
    /// A [`BsMultiBoundSphere`].
    Sphere { center: Vec3, radius: f32 },
}

impl BsOrderedNode {
    /// The bound used to sort children by alpha, as `[x, y, z, w]`.
    pub fn alpha_sort_bound(&self) -> [f32; 4] {
        let mut out = [0.0; 4];
        ffi::ordered_node_alpha_sort_bound(&self.0, &mut out);
        out
    }

    pub fn set_alpha_sort_bound(self: Pin<&mut Self>, bound: [f32; 4]) {
        ffi::set_ordered_node_alpha_sort_bound(self.ffi_mut(), &bound);
    }

    fields! {
        /// Whether [`alpha_sort_bound`](Self::alpha_sort_bound) is fixed rather than updated
        /// from the children.
        is_static_bound, set_static_bound: bool = ordered_node_static_bound, set_ordered_node_static_bound;
    }
}

impl BsRangeNode {
    fields! {
        /// The lowest child shown.
        min, set_min: u8 = range_node_min, set_range_node_min;
        /// The highest child shown.
        max, set_max: u8 = range_node_max, set_range_node_max;
        /// The current position in the range.
        current, set_current: u8 = range_node_current, set_range_node_current;
    }
}

impl NiBillboardNode {
    /// How the node turns to face the camera. `None` if the stored value isn't a mode.
    ///
    /// Not saved in Morrowind, whose files keep the mode in bits 5–6 of the object flags instead.
    pub fn mode(&self) -> Option<BillboardMode> {
        BillboardMode::try_from(ffi::billboard_mode(&self.0)).ok()
    }

    pub fn set_mode(self: Pin<&mut Self>, mode: BillboardMode) {
        ffi::set_billboard_mode(self.ffi_mut(), mode.into());
    }
}

/// How an [`NiBillboardNode`] turns to face the camera.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u16)]
pub enum BillboardMode {
    /// Faces the camera.
    AlwaysFaceCamera = 0,
    /// Turns only around its up axis.
    RotateAboutUp = 1,
    /// Faces the camera without rolling.
    RigidFaceCamera = 2,
    /// Faces the camera's position.
    AlwaysFaceCenter = 3,
    /// Faces the camera's position without rolling.
    RigidFaceCenter = 4,
    /// Turns only around its up axis (Bethesda variant).
    BsRotateAboutUp = 5,
    /// Turns only around its up axis.
    RotateAboutUp2 = 9,
}

impl NiSwitchNode {
    pub fn flags(&self) -> SwitchNodeFlags {
        SwitchNodeFlags::from_bits_retain(ffi::switch_node_flags(&self.0))
    }

    pub fn set_flags(self: Pin<&mut Self>, flags: SwitchNodeFlags) {
        ffi::set_switch_node_flags(self.ffi_mut(), flags.bits());
    }

    fields! {
        /// The position of the active child among the node's children. Not checked against the
        /// number of children.
        active_index, set_active_index: u32 = switch_node_index, set_switch_node_index;
    }
}

bitflags! {
    /// The flags of an [`NiSwitchNode`]. Other bits are kept as stored.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct SwitchNodeFlags: u16 {
        /// Update only the active child.
        const UPDATE_ONLY_ACTIVE_CHILD = 1;
        const UPDATE_CONTROLLERS = 1 << 1;
        const _ = !0;
    }
}

impl NiSortAdjustNode {
    /// Whether the children are sorted. `None` if the stored value isn't a mode.
    pub fn sorting_mode(&self) -> Option<SortingMode> {
        SortingMode::try_from(ffi::sorting_mode(&self.0)).ok()
    }

    pub fn set_sorting_mode(self: Pin<&mut Self>, mode: SortingMode) {
        ffi::set_sorting_mode(self.ffi_mut(), mode.into());
    }
}

/// Whether an [`NiSortAdjustNode`]'s children are sorted.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum SortingMode {
    /// As the parent sorts.
    Inherit = 0,
    /// Not sorted.
    Off = 1,
}

/// # Nodes
///
/// Finding, adding, deleting and moving nodes, and their transforms.
impl NifFile {
    /// Every node in the file, in block order, including subclasses such as `BSFadeNode`.
    pub fn nodes(&self) -> BlockIter<'_, NiNode> {
        self.blocks_of::<NiNode>()
    }

    /// Returns the root node: block 0 if it's a node, otherwise the first node block.
    /// Returns `None` if the file has no node blocks.
    pub fn root_node(&self) -> Option<View<'_, NiNode>> {
        self.view::<NiNode>(0).or_else(|| self.nodes().views().next())
    }

    /// Like [`root_node`](Self::root_node), for editing.
    pub fn root_node_mut(&mut self) -> Option<Edit<'_, NiNode>> {
        let index = self.root_node()?.index();
        self.edit(index)
    }

    /// The root node's translation relative to its parent, or `None` if there's no root node
    /// (see [`root_node`](Self::root_node)).
    pub fn root_translation(&self) -> Option<Vec3> {
        Some(self.root_node()?.transform().translation)
    }

    /// Copies `node`, usually from another file, into this file, as [`Edit::clone_node`] does.
    /// The source file isn't changed. Returns `None`, changing nothing, unless both files are
    /// from the same game. Skyrim LE and SE are different games, Fallout 3 and New Vegas one;
    /// files of other versions must have the same [version](Self::version). Between versions of
    /// a game, the copy is saved in this file's version, so fields it doesn't store are dropped,
    /// and unknown blocks are copied as they are.
    pub fn clone_node_from(&mut self, node: View<'_, NiNode>) -> Option<Index> {
        index_from_ffi(nif_ffi::clone_node_from(self.inner.pin_mut(), &node.file().inner, node.get().as_ffi()))
    }

    /// Adds a new `NiNode` named `name` with the given transform to its parent, as the last child
    /// of the node at block index `parent`, or of the [root node](Self::root_node) if `parent` is
    /// `None`. Returns the new node's block index.
    ///
    /// Returns `None` without changing the file if `parent` isn't the index of a node, or if
    /// `parent` is `None` and the file has no root node.
    ///
    /// The new node is appended to the block list, so existing block indices stay valid.
    /// [`Edit::add_child`] does the same from a node's handle.
    ///
    /// ```no_run
    /// # use nifly::{file::NifFile, object3d::Transform};
    /// # let mut nif: NifFile = todo!();
    /// # let transform: Transform = todo!();
    /// let arm = nif.add_node("Arm", transform, None).unwrap();
    /// let hand = nif.add_node("Hand", transform, Some(arm)).unwrap();
    /// ```
    pub fn add_node(
        &mut self,
        name: impl AsRef<[u8]>,
        transform: Transform,
        parent: Option<Index>,
    ) -> Option<Index> {
        let parent_index = match parent {
            None => u32::MAX,
            // Indices that don't fit in u32 can't be blocks, and u32::MAX would mean "root".
            Some(index) => match u32::try_from(index) {
                Ok(index) if index != u32::MAX => index,
                _ => return None,
            },
        };

        match nif_ffi::add_node(self.inner.pin_mut(), name.as_ref(), transform.to_ffi(), parent_index) {
            u32::MAX => None,
            index => Some(index as Index),
        }
    }

    /// Like [`add_node`](Self::add_node), but adds a `T` with default settings. Doesn't check
    /// that the file's game uses `T`.
    ///
    /// ```
    /// # use nifly::{file::{NifFile, NifVersion}, nodes::NiSwitchNode, object3d::Transform};
    /// let mut nif = NifFile::create(NifVersion::OBLIVION);
    /// let switch = nif.add_node_of::<NiSwitchNode>("Switch", Transform::default(), None).unwrap();
    /// nif.block_of_mut::<NiSwitchNode>(switch).unwrap().set_active_index(1);
    /// ```
    pub fn add_node_of<T: NewNode>(
        &mut self,
        name: impl AsRef<[u8]>,
        transform: Transform,
        parent: Option<Index>,
    ) -> Option<Index> {
        let parent_index = match parent {
            None => u32::MAX,
            Some(index) => block_index_to_ffi(index)?,
        };
        index_from_ffi(T::add(self.inner.pin_mut(), name.as_ref(), transform.to_ffi(), parent_index))
    }

    /// Like [`add_node`](Self::add_node), but adds an [`NiCamera`]. It has a perspective frustum
    /// and the full screen as its viewport.
    ///
    /// ```
    /// # use nifly::{file::{NifFile, NifVersion}, nodes::NiCamera, object3d::Transform};
    /// let mut nif = NifFile::create(NifVersion::OBLIVION);
    /// let camera = nif.add_camera("Camera", Transform::default(), None).unwrap();
    /// nif.block_of_mut::<NiCamera>(camera).unwrap().set_orthographic(true);
    /// ```
    pub fn add_camera(&mut self, name: impl AsRef<[u8]>, transform: Transform, parent: Option<Index>) -> Option<Index> {
        let parent_index = match parent {
            None => u32::MAX,
            Some(index) => block_index_to_ffi(index)?,
        };
        index_from_ffi(ffi::add_camera(self.inner.pin_mut(), name.as_ref(), transform.to_ffi(), parent_index))
    }

    /// Deletes the first node (in block order) named `name`. Returns whether a node had that
    /// name. Only nodes are searched, and names are compared byte for byte.
    ///
    /// [`NiNode::can_delete`] isn't checked first:
    /// - Refs to the deleted node are cleared, e.g. in its parent's children or a skin's bones.
    /// - Anything the node referenced (children, properties, extra data, ...) stays in the file,
    ///   no longer linked into the tree. Saving with [`SaveOptions::optimize`](crate::file::SaveOptions::optimize) removes it.
    /// - Every block after the deleted one moves down one index, so block indices you're holding
    ///   from before the call (e.g. from [`add_node`](Self::add_node) or
    ///   [`block_index`](Self::block_index)) may now refer to a different block.
    ///
    /// **Don't delete bones of skinned shapes this way.** The bone is removed from the skin
    /// instance's bone list, but `NiSkinData` keeps its weights, so the weights of later bones
    /// shift onto the wrong bones (see [`bone_weights`](View::bone_weights)).
    pub fn delete_node(&mut self, name: impl AsRef<[u8]>) -> bool {
        nif_ffi::delete_node(self.inner.pin_mut(), name.as_ref())
    }

    /// Whether the first node (in block order) named `name` can be deleted without leaving
    /// blocks behind (see [`NiNode::can_delete`]). `false` if no node has that name.
    pub fn can_delete_node(&self, name: impl AsRef<[u8]>) -> bool {
        nif_ffi::can_delete_node_by_name(&self.inner, name.as_ref())
    }

    /// Deletes empty nodes and returns how many were deleted, or `None` if the file has
    /// [unknown blocks](Self::has_unknown) or no root node. Block indices change.
    ///
    /// A node is empty if it owns no blocks (children, extra data, collision, properties,
    /// controllers) and is referenced at most once, counting pointers. Empty branches are deleted
    /// whole. The root and bones are kept.
    ///
    /// Empty nodes can still matter to the game, e.g. `AddOnNode*` and `InvMarker` attachment
    /// points.
    pub fn delete_unreferenced_nodes(&mut self) -> Option<usize> {
        usize::try_from(nif_ffi::delete_unreferenced_nodes(self.inner.pin_mut())).ok()
    }

    /// Returns the transform to its parent of the first node (in block order) named `name`, or
    /// `None` if no node has that name. Only nodes are searched; shapes with the name are ignored.
    /// Names are compared byte for byte.
    ///
    /// If you already have the node, call [`NiAvObject::transform`] instead.
    pub fn node_transform_to_parent(&self, name: impl AsRef<[u8]>) -> Option<Transform> {
        let name = name.as_ref();
        self.nodes()
            .find(|node| node.name() == name)
            .map(|node| node.transform())
    }

    /// Returns the transform from the first node (in block order) named `name` to the root's
    /// coordinate system, by composing transforms up the parent chain. `None` if no node has that name.
    /// Only nodes are searched, and names are compared byte for byte.
    ///
    /// O(depth × blocks × children), since each parent is found by scanning every node.
    pub fn node_transform_to_global(&self, name: impl AsRef<[u8]>) -> Option<Transform> {
        let mut out = Transform::IDENTITY.to_ffi();
        nif_ffi::node_transform_to_global(&self.inner, name.as_ref(), &mut out)
            .then(|| Transform::from_ffi(out))
    }

    /// Sets the transform to its parent of the first node (in block order) named `name`. Returns
    /// whether a node had that name. Only nodes are searched, and names are compared byte for
    /// byte.
    ///
    /// With `root_children_only`, only the root node's direct children are searched (in child
    /// order), so a deeper node with the same name is left alone.
    ///
    /// The transform is stored as given; NIF has a single uniform scale.
    pub fn set_node_transform_to_parent(
        &mut self,
        name: impl AsRef<[u8]>,
        transform: Transform,
        root_children_only: bool,
    ) -> bool {
        nif_ffi::set_node_transform_to_parent(
            self.inner.pin_mut(),
            name.as_ref(),
            transform.to_ffi(),
            root_children_only,
        )
    }
}

/// Children and copies of nodes.
impl<'a, T: NiType> View<'a, T> {
    /// The node's direct children. Skips empty refs and refs that don't resolve to an
    /// [`NiAvObject`].
    pub fn children<I>(&self) -> BlockIter<'a, NiAvObject, RefIndices>
    where
        T: IsA<NiNode, I>,
    {
        let node: &NiNode = self.get().upcast_ref();
        BlockIter::new(self.file(), RefIndices::new(ffi::child_ref_indices(node.as_ffi())))
    }

    /// The node's direct children that are a `U` or a subclass of `U`.
    pub fn children_of<U: NiType, I>(&self) -> BlockIter<'a, U, RefIndices>
    where
        T: IsA<NiNode, I>,
    {
        self.children().of::<U>()
    }

    /// The object's collision object, or `None` if it has none.
    pub fn collision<I>(&self) -> Option<View<'a, NiCollisionObject>>
    where
        T: IsA<NiAvObject, I>,
    {
        self.follow(RefKind::Collision)
    }

    /// The first node that lists this object as a child, or `None` if there isn't one (e.g. for
    /// the root). O(blocks × children).
    pub fn parent<I>(&self) -> Option<View<'a, NiNode>>
    where
        T: IsA<NiAvObject, I>,
    {
        let index = self.index() as u32;
        self.resolve(index_from_ffi(nif_ffi::parent_node_index(&self.file().inner, index)))
    }
}

impl<T: NiType> Edit<'_, T> {
    /// Like [`View::parent`], for editing.
    pub fn parent<I>(&mut self) -> Option<Edit<'_, NiNode>>
    where
        T: IsA<NiAvObject, I>,
    {
        let index = self.view().parent()?.index();
        self.resolve(Some(index))
    }

    /// Like [`View::collision`], for editing.
    pub fn collision<I>(&mut self) -> Option<Edit<'_, NiCollisionObject>>
    where
        T: IsA<NiAvObject, I>,
    {
        self.follow(RefKind::Collision)
    }

    /// Sets the object's collision object, or removes it with `None`. Returns `false`, changing
    /// nothing, unless `collision` is an [`NiCollisionObject`]. The collision object's
    /// [target](View::target) isn't changed. [`Edit::assign_collision`] adds a new one.
    pub fn set_collision<I>(&mut self, collision: Option<Index>) -> bool
    where
        T: IsA<NiAvObject, I>,
    {
        self.set_ref(RefKind::Collision, collision)
    }

    /// Replaces the node's children, in order. Other nodes listing them keep them too. Returns
    /// `false`, changing nothing, if a block isn't an [`NiAvObject`], or is this node or above
    /// it.
    pub fn set_children<I>(&mut self, children: &[Index]) -> bool
    where
        T: IsA<NiNode, I>,
    {
        self.set_ref_list(RefList::Children, children)
    }

    /// Moves the object under the node at block index `parent`, or under the
    /// [root node](NifFile::root_node) if `parent` is `None`. The object is appended to the new
    /// parent's children and removed from its old parent, the first node in block order listing
    /// it. Its transform is kept, so it's now relative to the new parent. Does nothing more if
    /// `parent` already lists it.
    ///
    /// Returns `false`, changing nothing, if `parent` isn't a node (or there's no root node), is
    /// this object, or is in its subtree.
    pub fn set_parent<I>(&mut self, parent: Option<Index>) -> bool
    where
        T: IsA<NiAvObject, I>,
    {
        let parent = match parent {
            None => u32::MAX,
            Some(parent) => match block_index_to_ffi(parent) {
                Some(parent) => parent,
                None => return false,
            },
        };
        let child = self.index() as u32;
        nif_ffi::set_parent_node(self.file_mut().inner.pin_mut(), child, parent)
    }

    /// Copies the node, without its children, effects, collision or controller, and with deep
    /// copies of its extra data and properties, and returns the copy's index. The copy has the
    /// same name and transform but no parent; see [`set_parent`](Self::set_parent). New blocks
    /// are added at the end, so existing block indices don't change.
    pub fn clone_node<I>(&mut self) -> Index
    where
        T: IsA<NiNode, I>,
    {
        let node = self.index() as u32;
        index_from_ffi(nif_ffi::clone_node(self.file_mut().inner.pin_mut(), node)).expect("the block is a node")
    }

    /// Adds a new `NiNode` named `name` with the given transform to its parent as the node's
    /// last child, and returns its index. It's added at the end, so existing block indices don't
    /// change.
    ///
    /// ```
    /// # use nifly::{file::{NifFile, NifVersion}, nodes::{NiNode, NiSwitchNode}, object3d::Transform};
    /// let mut nif = NifFile::create(NifVersion::OBLIVION);
    /// let arm = nif.root_node_mut().unwrap().add_child("Arm", Transform::default());
    /// let mut arm = nif.edit::<NiNode>(arm).unwrap();
    /// let switch = arm.add_child_of::<NiSwitchNode, _>("Switch", Transform::default());
    /// nif.block_of_mut::<NiSwitchNode>(switch).unwrap().set_active_index(1);
    /// ```
    pub fn add_child<I>(&mut self, name: impl AsRef<[u8]>, transform: Transform) -> Index
    where
        T: IsA<NiNode, I>,
    {
        self.add_child_of::<NiNode, I>(name, transform)
    }

    /// Like [`add_child`](Self::add_child), but adds an `N` with default settings. Doesn't check
    /// that the file's game uses `N`.
    pub fn add_child_of<N: NewNode, I>(&mut self, name: impl AsRef<[u8]>, transform: Transform) -> Index
    where
        T: IsA<NiNode, I>,
    {
        let parent = self.index() as u32;
        index_from_ffi(N::add(self.file_mut().inner.pin_mut(), name.as_ref(), transform.to_ffi(), parent))
            .expect("the block is a node")
    }
}

/// The particle systems of a [`BsMasterParticleSystem`].
impl<'a> View<'a, BsMasterParticleSystem> {
    /// The particle systems it manages. Skips empty refs.
    pub fn particle_systems(&self) -> BlockIter<'a, NiAvObject, RefIndices> {
        self.follow_list(RefList::ParticleSystems)
    }
}

impl Edit<'_, BsMasterParticleSystem> {
    /// Replaces the [particle systems](View::particle_systems). Returns `false`, changing
    /// nothing, unless each block is an [`NiAvObject`].
    pub fn set_particle_systems(&mut self, systems: &[Index]) -> bool {
        self.set_ref_list(RefList::ParticleSystems, systems)
    }
}

/// Bone lists of a [`BsTreeNode`].
impl<'a> View<'a, BsTreeNode> {
    /// The nodes in the first bone list (the trunk bones of Skyrim trees). Skips empty refs and
    /// refs that don't resolve to a node.
    pub fn bones_1(&self) -> BlockIter<'a, NiNode, RefIndices> {
        BlockIter::new(self.file(), RefIndices::new(ffi::tree_node_bones(&self.get().0, 1)))
    }

    /// Like [`bones_1`](Self::bones_1), for the second bone list (the branch bones of Skyrim
    /// trees).
    pub fn bones_2(&self) -> BlockIter<'a, NiNode, RefIndices> {
        BlockIter::new(self.file(), RefIndices::new(ffi::tree_node_bones(&self.get().0, 2)))
    }
}

impl Edit<'_, BsTreeNode> {
    /// Replaces the first bone list with these nodes. Returns `false`, changing nothing, if a
    /// bone isn't a node.
    pub fn set_bones_1(&mut self, bones: &[Index]) -> bool {
        self.set_bones(1, bones)
    }

    /// Like [`set_bones_1`](Self::set_bones_1), for the second bone list.
    pub fn set_bones_2(&mut self, bones: &[Index]) -> bool {
        self.set_bones(2, bones)
    }

    fn set_bones(&mut self, list: u8, bones: &[Index]) -> bool {
        let Some(bones) = bones.iter().map(|&b| block_index_to_ffi(b)).collect::<Option<Vec<_>>>() else {
            return false;
        };
        let node = self.index() as u32;
        nif_ffi::set_tree_node_bones(self.file_mut().inner.pin_mut(), node, list, &bones)
    }
}

/// The multi-bound of a [`BsMultiBoundNode`].
impl<'a> View<'a, BsMultiBoundNode> {
    /// The node's multi-bound, or `None` if it has none.
    pub fn multi_bound(&self) -> Option<View<'a, BsMultiBound>> {
        self.resolve(index_from_ffi(ffi::multi_bound_node_bound(&self.get().0)))
    }
}

impl Edit<'_, BsMultiBoundNode> {
    /// Like [`View::multi_bound`], for editing.
    pub fn multi_bound(&mut self) -> Option<Edit<'_, BsMultiBound>> {
        let index = self.view().multi_bound()?.index();
        self.resolve(Some(index))
    }

    /// Gives the node a new [`BsMultiBound`] of this shape, replacing any it had, and returns the
    /// new multi-bound's index. The new blocks are added at the end. A replaced multi-bound stays
    /// in the file until saving drops it, if nothing else uses it.
    ///
    /// ```
    /// # use nifly::{file::NifFile, nodes::{BsMultiBoundAabb, BsMultiBoundNode, MultiBoundShape}, object3d::Vec3};
    /// # let mut nif = NifFile::load(std::fs::File::open("nifly-sys/vendor/nifly/tests/input/TestNifFile_MultiBound_SE.nif").unwrap(), None).unwrap();
    /// # let node = nif.blocks_of::<BsMultiBoundNode>().indices().next().unwrap();
    /// let shape = MultiBoundShape::Aabb { center: Vec3::ZERO, half_extent: Vec3::new(64.0, 64.0, 32.0) };
    /// nif.edit::<BsMultiBoundNode>(node).unwrap().assign_multi_bound(shape);
    /// let node = nif.view::<BsMultiBoundNode>(node).unwrap();
    /// let data = node.multi_bound().unwrap().data().unwrap();
    /// assert!(data.cast::<BsMultiBoundAabb>().is_some());
    /// ```
    pub fn assign_multi_bound(&mut self, shape: MultiBoundShape) -> Index {
        let node = self.index() as u32;
        let file = self.file_mut().inner.pin_mut();
        let index = match shape {
            MultiBoundShape::Aabb { center, half_extent } => {
                nif_ffi::assign_multi_bound_aabb(file, node, center.to_ffi(), half_extent.to_ffi())
            }
            MultiBoundShape::Obb { center, size, rotation } => nif_ffi::assign_multi_bound_obb(
                file,
                node,
                center.to_ffi(),
                size.to_ffi(),
                &rotation.rows.map(Vec3::to_ffi),
            ),
            MultiBoundShape::Sphere { center, radius } => {
                nif_ffi::assign_multi_bound_sphere(file, node, center.to_ffi(), radius)
            }
        };
        index_from_ffi(index).expect("the block is a BsMultiBoundNode")
    }

    /// Clears the node's multi-bound. It stays in the file until saving drops it, if nothing else
    /// uses it. Returns `false` if the node had none.
    pub fn remove_multi_bound(&mut self) -> bool {
        let node = self.index() as u32;
        nif_ffi::remove_multi_bound(self.file_mut().inner.pin_mut(), node)
    }

    /// Points the node at the [`BsMultiBound`] at `multi_bound`, or clears it with `None`.
    /// Returns `false`, changing nothing, if that block isn't one.
    pub fn set_multi_bound(&mut self, multi_bound: Option<Index>) -> bool {
        self.set_ref(RefKind::MultiBound, multi_bound)
    }
}

/// The shape of a [`BsMultiBound`].
impl<'a> View<'a, BsMultiBound> {
    /// The multi-bound's shape, or `None` if it has none. Cast it to a [`BsMultiBoundAabb`],
    /// [`BsMultiBoundObb`] or [`BsMultiBoundSphere`].
    pub fn data(&self) -> Option<View<'a, BsMultiBoundData>> {
        self.resolve(index_from_ffi(ffi::multi_bound_data(&self.get().0)))
    }
}

impl Edit<'_, BsMultiBound> {
    /// Like [`View::data`], for editing.
    pub fn data(&mut self) -> Option<Edit<'_, BsMultiBoundData>> {
        let index = self.view().data()?.index();
        self.resolve(Some(index))
    }

    /// Points the multi-bound at the [`BsMultiBoundData`] at `data`, or clears it with `None`.
    /// Returns `false`, changing nothing, if that block isn't one.
    pub fn set_data(&mut self, data: Option<Index>) -> bool {
        self.set_ref(RefKind::MultiBoundData, data)
    }
}

/// The LOD data of an [`NiLodNode`].
impl<'a> View<'a, NiLodNode> {
    /// The node's LOD data, or `None` if it has none. Cast it to an [`NiRangeLodData`] or
    /// [`NiScreenLodData`].
    pub fn lod_data(&self) -> Option<View<'a, NiLodData>> {
        self.resolve(index_from_ffi(ffi::lod_node_data(&self.get().0)))
    }
}

impl Edit<'_, NiLodNode> {
    /// Like [`View::lod_data`], for editing.
    pub fn lod_data(&mut self) -> Option<Edit<'_, NiLodData>> {
        let index = self.view().lod_data()?.index();
        self.resolve(Some(index))
    }

    /// Gives the node a new `T` with default settings as its LOD data, replacing any it had, and
    /// returns the new block's index. It's added at the end. Replaced LOD data stays in the file
    /// until saving drops it, if nothing else uses it.
    ///
    /// ```
    /// # use nifly::{file::{NifFile, NifVersion}, nodes::{LodRange, NiLodNode, NiRangeLodData}, object3d::Transform};
    /// let mut nif = NifFile::create(NifVersion::OBLIVION);
    /// let node = nif.add_node_of::<NiLodNode>("LOD", Transform::default(), None).unwrap();
    /// let data = nif.edit::<NiLodNode>(node).unwrap().assign_lod_data::<NiRangeLodData>();
    /// let levels = [LodRange { near: 0.0, far: 1000.0 }, LodRange { near: 1000.0, far: 4000.0 }];
    /// nif.block_of_mut::<NiRangeLodData>(data).unwrap().set_levels(&levels);
    /// ```
    pub fn assign_lod_data<T: NewLodData>(&mut self) -> Index {
        let node = self.index() as u32;
        index_from_ffi(T::assign(self.file_mut().inner.pin_mut(), node)).expect("the block is an NiLodNode")
    }

    /// Clears the node's LOD data. It stays in the file until saving drops it, if nothing else
    /// uses it. Returns `false` if the node had none.
    pub fn remove_lod_data(&mut self) -> bool {
        let node = self.index() as u32;
        nif_ffi::remove_lod_data(self.file_mut().inner.pin_mut(), node)
    }

    /// Points the node at the [`NiLodData`] at `data`, or clears it with `None`. Returns
    /// `false`, changing nothing, if that block isn't one.
    pub fn set_lod_data(&mut self, data: Option<Index>) -> bool {
        self.set_ref(RefKind::LodData, data)
    }
}

/// The scene of an [`NiCamera`].
impl<'a> View<'a, NiCamera> {
    /// The object the camera renders, or `None` if it has none.
    pub fn scene(&self) -> Option<View<'a, NiAvObject>> {
        self.resolve(index_from_ffi(ffi::camera_scene(&self.get().0)))
    }
}

impl Edit<'_, NiCamera> {
    /// Like [`View::scene`], for editing.
    pub fn scene(&mut self) -> Option<Edit<'_, NiAvObject>> {
        let index = self.view().scene()?.index();
        self.resolve(Some(index))
    }

    /// Sets the scene to the [`NiAvObject`] at block index `scene`, or none if `None`. Returns
    /// `false`, changing nothing, if that block isn't an `NiAvObject`.
    pub fn set_scene(&mut self, scene: Option<Index>) -> bool {
        let Some(scene) = optional_index_to_ffi(scene) else { return false };
        let camera = self.index() as u32;
        ffi::set_camera_scene(self.file_mut().inner.pin_mut(), camera, scene)
    }
}

/// A node type that [`NifFile::add_node_of`] can create.
///
/// This trait is sealed.
pub trait NewNode: NiType {
    /// Adds the node and returns its index (`u32::MAX` on failure).
    #[doc(hidden)]
    fn add(file: Pin<&mut nif_ffi::NifFile>, name: &[u8], transform: nif_ffi::Transform, parent: u32) -> u32;
}

impl NewNode for NiNode {
    fn add(file: Pin<&mut nif_ffi::NifFile>, name: &[u8], transform: nif_ffi::Transform, parent: u32) -> u32 {
        nif_ffi::add_node(file, name, transform, parent)
    }
}

macro_rules! new_node {
    ($($ty:ty => $add:ident;)*) => {$(
        impl NewNode for $ty {
            fn add(file: Pin<&mut nif_ffi::NifFile>, name: &[u8], transform: nif_ffi::Transform, parent: u32) -> u32 {
                nif_ffi::$add(file, name, transform, parent)
            }
        }
    )*};
}

new_node! {
    BsFadeNode => add_bs_fade_node;
    RootCollisionNode => add_root_collision_node;
    AvoidNode => add_avoid_node;
    NiBsAnimationNode => add_ni_bs_animation_node;
    NiBsParticleNode => add_ni_bs_particle_node;
    BsValueNode => add_bs_value_node;
    BsLeafAnimNode => add_bs_leaf_anim_node;
    BsTreeNode => add_bs_tree_node;
    BsOrderedNode => add_bs_ordered_node;
    BsMultiBoundNode => add_bs_multi_bound_node;
    BsWeakReferenceNode => add_bs_weak_reference_node;
    BsDistantObjectInstancedNode => add_bs_distant_object_instanced_node;
    BsRangeNode => add_bs_range_node;
    BsDebrisNode => add_bs_debris_node;
    BsBlastNode => add_bs_blast_node;
    BsDamageStage => add_bs_damage_stage;
    BsFaceGenNiNode => add_bs_face_gen_ni_node;
    NiBillboardNode => add_ni_billboard_node;
    NiSwitchNode => add_ni_switch_node;
    NiLodNode => add_ni_lod_node;
    NiBone => add_ni_bone;
    NiSortAdjustNode => add_ni_sort_adjust_node;
    BsMasterParticleSystem => add_bs_master_particle_system;
}

/// An entry of an [`NiDefaultAvObjectPalette`], from [`View::objects`].
#[derive(Clone, Copy, Debug)]
pub struct PaletteObject<'a> {
    /// Usually the object's name.
    pub name: &'a BStr,
    /// `None` if the entry is empty or doesn't refer to an [`NiAvObject`].
    pub object: Option<View<'a, NiAvObject>>,
}

/// The scene and objects of an [`NiDefaultAvObjectPalette`].
impl<'a> View<'a, NiDefaultAvObjectPalette> {
    /// The root of the objects, or `None` if it has none.
    pub fn scene(&self) -> Option<View<'a, NiAvObject>> {
        self.resolve(index_from_ffi(ffi::palette_scene(&self.get().0)))
    }

    /// The palette's entries, in order.
    pub fn objects(&self) -> impl ExactSizeIterator<Item = PaletteObject<'a>> + 'a {
        let (view, palette) = (*self, &self.get().0);
        (0..ffi::palette_object_count(palette)).map(move |i| PaletteObject {
            name: ffi::palette_object_name(palette, i).as_bstr(),
            object: view.resolve(index_from_ffi(ffi::palette_object(palette, i))),
        })
    }

    /// The object of the first entry named `name`, or `None` if there isn't one or it's empty.
    pub fn object(&self, name: impl AsRef<[u8]>) -> Option<View<'a, NiAvObject>> {
        self.objects().find(|o| o.name == name.as_ref())?.object
    }
}

impl Edit<'_, NiDefaultAvObjectPalette> {
    /// Like [`View::scene`], for editing.
    pub fn scene(&mut self) -> Option<Edit<'_, NiAvObject>> {
        let index = self.view().scene()?.index();
        self.resolve(Some(index))
    }

    /// Sets the scene to the [`NiAvObject`] at block index `scene`, or none if `None`. Returns
    /// `false`, changing nothing, if that block isn't an `NiAvObject`.
    pub fn set_scene(&mut self, scene: Option<Index>) -> bool {
        let Some(scene) = optional_index_to_ffi(scene) else { return false };
        let palette = self.index() as u32;
        ffi::set_palette_scene(self.file_mut().inner.pin_mut(), palette, scene)
    }

    /// Replaces the entries with `objects`: names and the block indices of the
    /// [`NiAvObject`]s they refer to, or `None` for empty entries. Names aren't checked against
    /// the objects. Returns `false`, changing nothing, if a block isn't an `NiAvObject`.
    pub fn set_objects<N: AsRef<[u8]>>(&mut self, objects: impl IntoIterator<Item = (N, Option<Index>)>) -> bool {
        let (mut names, mut indices) = (Vec::new(), Vec::new());
        for (name, index) in objects {
            let Some(index) = optional_index_to_ffi(index) else { return false };
            names.push(name);
            indices.push(index);
        }
        let (bytes, lengths) = joined(names);
        let palette = self.index() as u32;
        ffi::set_palette_objects(self.file_mut().inner.pin_mut(), palette, &bytes, &lengths, &indices)
    }
}
