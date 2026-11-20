//! Reading shapes through `View<NiShape>`.

use super::*;

/// Reading a shape.
impl<'a> View<'a, NiShape> {
    /// The paths of the external `.mesh` files of a [`BsGeometry`], one per level of detail, as
    /// stored (a name or SHA-1 hash, relative to the game's `geometry` folder, without the
    /// extension). Empty if the shape isn't a [`BsGeometry`] or stores its mesh data in the file.
    pub fn mesh_paths(&self) -> Vec<&'a BStr> {
        let shape = self.get();
        (0..nif_ffi::external_mesh_count(shape.as_ffi()))
            .map(|i| nif_ffi::external_mesh_path(shape.as_ffi(), i).as_bstr())
            .collect()
    }

    /// The shape's vertex data: the [`NiGeometryData`] block of an [`NiTriBasedGeom`] (such as
    /// an [`NiTriShape`] or [`NiTriStrips`]), or the selected mesh of a [`BsGeometry`], which
    /// isn't a block of its own. `None` for other shapes, such as [`BsTriShape`], which stores
    /// its vertices itself, and particles, or if the data ref is empty.
    ///
    /// This is a plain reference rather than a [`View`], since a mesh has no block index.
    /// [`BsGeometry::mesh`] reaches a [`BsGeometry`]'s other meshes.
    pub fn geometry_data(&self) -> Option<&'a NiGeometryData> {
        let ptr = nif_ffi::geometry_data(&self.file().inner, self.get().as_ffi());
        // SAFETY: the data is owned by this file, which outlives `'a`.
        unsafe { ptr.as_ref() }.map(NiGeometryData::from_ffi)
    }

    /// Check if the shape is compatible with Skyrim SE: it isn't an `NiTriStrips`, and its skin
    /// partition (if any) has no triangle strips.
    pub fn is_sse_compatible(&self) -> bool {
        let (file, shape) = (self.file(), self.get());
        nif_ffi::is_shape_sse_compatible(&file.inner, shape.as_ffi())
    }

    /// The vertex positions of the shape, or `None` if it doesn't store them.
    ///
    /// These read the vertex data wherever the shape keeps it: in its [`geometry_data`]
    /// or, for a [`BsTriShape`], in the shape itself, where normals, tangents, bitangents and
    /// colors are stored at reduced precision and returned decoded. Particles return `None`.
    ///
    /// [`geometry_data`]: Self::geometry_data
    pub fn vertices(&self) -> Option<Vec<Vec3>> {
        let (file, shape) = (self.file(), self.get());
        file.vertex_array(shape, nif_ffi::shape_vertices).map(vec3s)
    }

    /// The normals of the shape, one per vertex, or `None` if it doesn't store them. See
    /// [`vertices`](Self::vertices).
    pub fn normals(&self) -> Option<Vec<Vec3>> {
        let (file, shape) = (self.file(), self.get());
        file.vertex_array(shape, nif_ffi::shape_normals).map(vec3s)
    }

    /// The texture coordinates of the shape's first UV set as `[u, v]`, one per vertex, or `None`
    /// if it doesn't store them. See [`vertices`](Self::vertices).
    pub fn uvs(&self) -> Option<Vec<[f32; 2]>> {
        let (file, shape) = (self.file(), self.get());
        let uvs = file.vertex_array(shape, nif_ffi::shape_uvs)?;
        Some(uvs.into_iter().map(|uv| [uv.u, uv.v]).collect())
    }

    /// The vertex colors of the shape as `[r, g, b, a]` in `0.0..=1.0`, one per vertex, or `None`
    /// if it doesn't store them. See [`vertices`](Self::vertices).
    pub fn vertex_colors(&self) -> Option<Vec<[f32; 4]>> {
        let (file, shape) = (self.file(), self.get());
        let colors = file.vertex_array(shape, nif_ffi::shape_colors)?;
        Some(colors.into_iter().map(|c| [c.r, c.g, c.b, c.a]).collect())
    }

    /// The tangents of the shape, one per vertex, or `None` if it doesn't store them. See
    /// [`vertices`](Self::vertices).
    ///
    /// Oblivion files keep tangents and bitangents in an `NiBinaryExtraData` named
    /// "Tangent space (binormal & tangent vectors)". Loading reads them from it, and saving
    /// rewrites it from the shape's tangents, or removes it if the shape has none.
    pub fn tangents(&self) -> Option<Vec<Vec3>> {
        let (file, shape) = (self.file(), self.get());
        file.vertex_array(shape, nif_ffi::shape_tangents).map(vec3s)
    }

    /// The bitangents of the shape, one per vertex, or `None` if it doesn't store them. See
    /// [`vertices`](Self::vertices).
    pub fn bitangents(&self) -> Option<Vec<Vec3>> {
        let (file, shape) = (self.file(), self.get());
        file.vertex_array(shape, nif_ffi::shape_bitangents)
            .map(vec3s)
    }

    /// The eye data of a [`BsTriShape`], one value per vertex, or `None` if it isn't one or
    /// doesn't store any.
    pub fn eye_data(&self) -> Option<Vec<f32>> {
        let (file, shape) = (self.file(), self.get());
        file.vertex_array(shape, nif_ffi::shape_eye_data)
    }

    /// The offsets that move the shape's vertices to `target` (one position per vertex), as
    /// `(vertex, target - position)` in vertex order. Vertices that move less than 0.0001 along
    /// every axis are left out. Returns `None` if the shape has no vertices or `target` has a
    /// different length.
    ///
    /// # Compatibility
    ///
    /// BodySlide and Outfit Studio store slider data as these per-vertex offsets.
    pub fn vertex_diff(&self, target: &[Vec3]) -> Option<Vec<(usize, Vec3)>> {
        let vertices = self.vertices()?;
        (vertices.len() == target.len()).then(|| {
            let diffs = vertices.iter().zip(target).map(|(&v, &t)| t - v);
            diffs
                .enumerate()
                .filter(|(_, d)| !nearly_zero(&[d.x, d.y, d.z]))
                .collect()
        })
    }

    /// The offsets that move the shape's first UV set to `target` (one `[u, v]` per vertex), as
    /// `(vertex, target - uv)` in vertex order. Like
    /// [`vertex_diff`](Self::vertex_diff), but for UVs; `None` if the shape has no UVs.
    pub fn uv_diff(&self, target: &[[f32; 2]]) -> Option<Vec<(usize, [f32; 2])>> {
        let uvs = self.uvs()?;
        (uvs.len() == target.len()).then(|| {
            let diffs = uvs
                .iter()
                .zip(target)
                .map(|(uv, t)| [t[0] - uv[0], t[1] - uv[1]]);
            diffs.enumerate().filter(|(_, d)| !nearly_zero(d)).collect()
        })
    }

    /// The triangles of the shape, as three indices into its vertex arrays each, or `None` if it
    /// stores none. Triangle strips are returned as triangles, without degenerate ones. Particles
    /// and lines return `None`.
    pub fn triangles(&self) -> Option<Vec<[u16; 3]>> {
        let shape = self.get();
        let mut out = Vec::new();
        nif_ffi::shape_triangles(shape.as_ffi(), &mut out)
            .then(|| out.into_iter().map(|t| [t.p1, t.p2, t.p3]).collect())
    }

    /// Writes the data of external mesh `lod` (see [`mesh_paths`](Self::mesh_paths))
    /// of the [`BsGeometry`] the shape to `writer`, as the contents of its `.mesh` file. The
    /// counterpart of [`load_mesh`](Edit::load_mesh). Vertex positions are saved as
    /// 16-bit fractions of the mesh's extent, so they read back rounded.
    ///
    /// Returns `Ok(false)`, writing nothing, if the shape has no such external mesh. Writer errors
    /// are returned as [`Error::Io`], and data too large to write as [`Error::Exception`].
    pub fn save_mesh(&self, lod: usize, mut writer: impl io::Write) -> NifResult<bool> {
        let shape = self.get();
        let Ok(lod) = u32::try_from(lod) else {
            return Ok(false);
        };
        let mut bytes = Vec::new();
        if !nif_ffi::save_external_mesh(shape.as_ffi(), lod, &mut bytes)? {
            return Ok(false);
        }
        writer.write_all(&bytes)?;
        Ok(true)
    }

    /// The segments of the shape and the segment each triangle is in, or `None` if it isn't a
    /// [`BsSubIndexTriShape`]. Skyrim SE stores simpler segments, read with
    /// [`BsSubIndexTriShape::sse_segments`]; there this finds none.
    pub fn segments(&self) -> Option<ShapeSegments> {
        let shape = self.get();
        let (mut ssf, mut segs, mut subs, mut extra, mut parts) =
            (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());
        if !nif_ffi::shape_segments(
            shape.as_ffi(),
            &mut ssf,
            &mut segs,
            &mut subs,
            &mut extra,
            &mut parts,
        ) {
            return None;
        }

        // Part IDs number each segment, then its subsegments, in order.
        let mut by_part = Vec::new();
        let mut segments = Vec::with_capacity(segs.len());
        let (mut subs, mut extra) = (subs.into_iter(), extra.into_iter());
        for (segment, seg) in segs.iter().enumerate() {
            by_part.push(SegmentIndex {
                segment,
                subsegment: None,
            });
            let subsegments = (0..seg.subsegments as usize)
                .map(|subsegment| {
                    by_part.push(SegmentIndex {
                        segment,
                        subsegment: Some(subsegment),
                    });
                    let sub = subs.next().unwrap_or(nif_ffi::SubsegmentFfi {
                        user_slot: 0,
                        material: 0,
                        extra_len: 0,
                    });
                    Subsegment {
                        user_slot: (sub.user_slot >= 30).then_some(sub.user_slot),
                        material: sub.material,
                        extra_data: extra.by_ref().take(sub.extra_len as usize).collect(),
                    }
                })
                .collect();
            segments.push(Segment { subsegments });
        }
        let triangles = parts
            .iter()
            .map(|&p| {
                usize::try_from(p)
                    .ok()
                    .and_then(|p| by_part.get(p).copied())
            })
            .collect();
        Some(ShapeSegments {
            ssf_file: BString::from(ssf),
            segments,
            triangles,
        })
    }
}
