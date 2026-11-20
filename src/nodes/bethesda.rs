//! Bethesda node fields: multi-bounds, distant objects, weak references and others.

use super::*;

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
