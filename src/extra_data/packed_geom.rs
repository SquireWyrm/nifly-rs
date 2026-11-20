//! Packed combined geometry extra data.

use super::*;

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
        Self {
            grayscale_to_palette_scale: 1.0,
            transform: Transform::IDENTITY,
            bounds: BoundingSphere::default(),
        }
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

    pub(super) fn push_floats(&self, out: &mut Vec<f32>) {
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

/// The fields of a [`BsPackedCombinedSharedGeomDataExtra`] besides its objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PackedGeomTotals {
    /// The vertex format, as in [`PackedGeomObject::vertex_desc`].
    pub vertex_desc: u64,
    pub vertex_count: u32,
    pub triangle_count: u32,
    pub raw_flags_1: u32,
    pub raw_flags_2: u32,
}

impl PackedGeomTotals {
    fn from_ffi(h: types_ffi::PackedGeomHeader) -> Self {
        let types_ffi::PackedGeomHeader {
            vertex_desc,
            vertex_count,
            triangle_count,
            flags_1,
            flags_2,
        } = h;
        Self {
            vertex_desc,
            vertex_count,
            triangle_count,
            raw_flags_1: flags_1,
            raw_flags_2: flags_2,
        }
    }

    pub(super) fn to_ffi(self) -> types_ffi::PackedGeomHeader {
        let Self {
            vertex_desc,
            vertex_count,
            triangle_count,
            raw_flags_1: flags_1,
            raw_flags_2: flags_2,
        } = self;
        types_ffi::PackedGeomHeader {
            vertex_desc,
            vertex_count,
            triangle_count,
            flags_1,
            flags_2,
        }
    }
}

impl BsPackedCombinedSharedGeomDataExtra {
    pub fn totals(&self) -> PackedGeomTotals {
        PackedGeomTotals::from_ffi(ffi::packed_shared_header(&self.0))
    }

    /// Sets the [totals](Self::totals).
    pub fn set_totals(&mut self, totals: PackedGeomTotals) {
        ffi::set_packed_shared_header(self.ffi_mut(), totals.to_ffi());
    }

    pub fn objects(&self) -> Vec<PackedGeomObject> {
        let floats = ffi::packed_shared_instances(&self.0);
        let mut instances = floats
            .as_chunks::<{ PackedGeomInstance::FLOATS }>()
            .0
            .iter()
            .map(PackedGeomInstance::from_floats);
        ffi::packed_shared_objects(&self.0)
            .into_iter()
            .map(|o| {
                let lod = |i: usize| TriangleRange {
                    count: o.lods[2 * i],
                    offset: o.lods[2 * i + 1],
                };
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
    pub fn set_objects(&mut self, objects: &[PackedGeomObject]) {
        let (objects, instances) = packed_objects_ffi(objects);
        let ok = ffi::set_packed_shared_objects(self.ffi_mut(), &objects, &instances);
        debug_assert!(ok, "instance counts match their floats");
    }
}
