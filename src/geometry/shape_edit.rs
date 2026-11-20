//! Changing shapes through `Edit<NiShape>`, and generic shape handles.

use super::*;

/// Changing a shape.
impl Edit<'_, NiShape> {
    /// Like [`View::geometry_data`], for the data's setters.
    pub fn geometry_data(&mut self) -> Option<Pin<&mut NiGeometryData>> {
        let index = self.index();
        // SAFETY: the data came from this file.
        unsafe { self.file_mut().pin_block(|f| f.view::<NiShape>(index)?.geometry_data()) }
    }

    /// Points an `NiGeometry` shape at the [`NiGeometryData`] at `data`. Returns `false`,
    /// changing nothing, if the shape isn't an `NiGeometry` or the data's type doesn't match it,
    /// such as [`NiTriShapeData`] for an [`NiTriShape`] and its subtypes. The old data stays in
    /// the file until saving drops it, if nothing else uses it.
    pub fn set_geometry_data(&mut self, data: Index) -> bool {
        let (shape, file) = (self.index() as u32, self.file_mut());
        block_index_to_ffi(data).is_some_and(|data| refs_ffi::set_shape_geometry_data(file.inner.pin_mut(), shape, data))
    }

    /// Applies `position` to each vertex and `normal`/`tangent` to its directions, each weighted
    /// by `weights`; directions are renormalized.
    fn transform(
        &mut self,
        weights: Option<&[f32]>,
        position: impl Fn(Vec3) -> Vec3,
        normal: impl Fn(Vec3) -> Vec3,
        tangent: impl Fn(Vec3) -> Vec3,
    ) -> bool {
        let view = self.view();
        let Some(vertices) = view.vertices() else {
            return false;
        };
        if let Some(w) = weights
            && (w.len() != vertices.len() || !w.iter().all(|w| w.is_finite()))
        {
            return false;
        }
        let weight = |i: usize| weights.map_or(1.0, |w| w[i]);
        let blend = |items: Vec<Vec3>, f: &dyn Fn(Vec3) -> Vec3| -> Vec<Vec3> {
            items.into_iter().enumerate().map(|(i, d)| (d + (f(d).normalize() - d) * weight(i)).normalize()).collect()
        };
        let (normals, tangents, bitangents) = (view.normals(), view.tangents(), view.bitangents());

        let moved: Vec<Vec3> = vertices.iter().enumerate().map(|(i, &p)| p + (position(p) - p) * weight(i)).collect();
        if !self.set_vertices(&moved) {
            return false;
        }
        // Same vertex count, so these can't fail.
        if let Some(n) = normals {
            self.set_normals(&blend(n, &normal));
        }
        if let Some(t) = tangents {
            self.set_tangents(&blend(t, &tangent));
        }
        if let Some(b) = bitangents {
            self.set_bitangents(&blend(b, &tangent));
        }
        true
    }


    /// Replaces the triangles of the shape.
    ///
    /// Returns `false`, changing nothing, if the shape has no triangle list (an
    /// [`NiTriShape`] or other shape with [`NiTriShapeData`], a [`BsTriShape`], or a
    /// [`BsGeometry`] mesh; not strips), an index isn't below its vertex count, or there are
    /// more triangles than it can save: 65535 for [`NiTriShapeData`] and
    /// [`triangle_limit`](NifFile::triangle_limit) otherwise.
    ///
    /// Data that refers to the triangles, such as skin partitions (see
    /// [`update_skin_partitions`](Self::update_skin_partitions)) and the segments of a
    /// [`BsSubIndexTriShape`], isn't updated.
    pub fn set_triangles(&mut self, triangles: &[[u16; 3]]) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        let triangles: Vec<_> = triangles
            .iter()
            .map(|&[p1, p2, p3]| nif_ffi::Triangle { p1, p2, p3 })
            .collect();
        block_index_to_ffi(shape)
            .is_some_and(|shape| nif_ffi::set_shape_triangles(file.inner.pin_mut(), shape, &triangles))
    }

    /// Reads the data of external mesh `lod` (see [`mesh_paths`](View::mesh_paths))
    /// of the [`BsGeometry`] from `reader`, the contents of its `.mesh`
    /// file, replacing what the shape had for it. Its vertices, triangles and other vertex data
    /// are then available as for other shapes, from [`geometry_data`](View::geometry_data) and
    /// [`vertices`](View::vertices) for the first mesh.
    ///
    /// Returns `Ok(false)`, changing nothing, if there's no such external mesh. Reader errors are
    /// returned as [`Error::Io`], as is data that ends early ([`io::ErrorKind::UnexpectedEof`]).
    /// A mesh format version above 2 is [`LoadError::UnsupportedVersion`], and other malformed
    /// data [`Error::Exception`]. The mesh is only changed on success.
    pub fn load_mesh(&mut self, lod: usize, mut reader: impl io::Read) -> NifResult<bool> {
        let (shape, file) = (self.index(), self.file_mut());
        let (Some(shape), Ok(lod)) = (block_index_to_ffi(shape), u32::try_from(lod)) else {
            return Ok(false);
        };
        let mut reader = RustReader::new(&mut reader);
        let result = nif_ffi::load_external_mesh(file.inner.pin_mut(), shape, lod, &mut reader);

        // Prefer the reader's own error over the C++ exception it was turned into.
        if let Some(e) = reader.take_error() {
            return Err(Error::Io(e));
        }
        match result? {
            0 => Ok(true),
            1 => Ok(false),
            3 => Err(LoadError::UnsupportedVersion.into()),
            _ => Err(Error::Io(io::Error::new(io::ErrorKind::UnexpectedEof, "mesh data ended early"))),
        }
    }

    /// Sets the path of external mesh `lod` (see [`mesh_paths`](View::mesh_paths)) of
    /// the [`BsGeometry`]. Returns `false`, changing nothing, if there's no
    /// such external mesh.
    pub fn set_mesh_path(&mut self, lod: usize, path: impl AsRef<[u8]>) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        let (Some(shape), Ok(lod)) = (block_index_to_ffi(shape), u32::try_from(lod)) else {
            return false;
        };
        nif_ffi::set_external_mesh_path(file.inner.pin_mut(), shape, lod, path.as_ref())
    }

    /// Converts the [`NiTriStrips`] and its [`NiTriStripsData`] to an
    /// [`NiTriShape`] and [`NiTriShapeData`] with the same geometry, as triangles without the
    /// strips' degenerate ones. Both keep their block indices, and the shape everything else.
    /// A skin partition keeps its own strips; [`update_skin_partitions`](Self::update_skin_partitions)
    /// converts those.
    ///
    /// Returns `false`, changing nothing, if the shape isn't an [`NiTriStrips`] with triangles,
    /// or another block shares its data.
    pub fn triangulate(&mut self) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        block_index_to_ffi(shape).is_some_and(|shape| nif_ffi::triangulate_shape(file.inner.pin_mut(), shape))
    }

    /// Deletes the shape, removing it from its parent's children. The
    /// blocks it refers to, such as its data, shader, properties, skin, extra data, collision and
    /// controllers, are deleted too, along with what they refer to, unless another block still
    /// uses them. Bone nodes stay.
    ///
    /// Deleting blocks moves later blocks down, so block indices change. Returns `false` if that
    /// block isn't a shape.
    pub fn delete(mut self) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        block_index_to_ffi(shape).is_some_and(|shape| nif_ffi::delete_shape(file.inner.pin_mut(), shape))
    }

    /// Copies the shape, with deep copies of the blocks it refers to
    /// (geometry data, shader, skin, properties, extra data), and names the copy `name`. The
    /// copy is a child of the original's parent and uses the same bones. In Skyrim files, a copy
    /// whose shader uses model-space normals has its normals and tangents turned off.
    ///
    /// New blocks are added at the end, so existing block indices don't change. Returns the
    /// copy's index, or `None` if the shape can't be copied.
    pub fn clone_shape(&mut self, name: impl AsRef<[u8]>) -> Option<Index> {
        let (shape, file) = (self.index(), self.file_mut());
        let shape = block_index_to_ffi(shape)?;
        index_from_ffi(nif_ffi::clone_shape(file.inner.pin_mut(), shape, name.as_ref()))
    }

    /// Reorders the triangles of the shape: triangle `i` becomes the old
    /// triangle `order[i]`. The triangles themselves don't change.
    ///
    /// Data that refers to triangle positions isn't updated: the segments of a
    /// [`BsSubIndexTriShape`] (reset them with [`set_segments`](Self::set_segments))
    /// and the LOD sizes of a [`BsMeshLodTriShape`]. A skinned Skyrim SE [`BsTriShape`] saves its
    /// triangles in its skin partitions, so the new order is lost on save unless
    /// [`update_skin_partitions`](Self::update_skin_partitions) rebuilds them first.
    ///
    /// Returns `false`, changing nothing, if the shape has no triangle list (not
    /// [`NiTriStrips`]) or `order` isn't a permutation of its triangle indices.
    pub fn reorder_triangles(&mut self, order: &[usize]) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        let (Some(shape), Ok(order)) = (
            block_index_to_ffi(shape),
            order.iter().map(|&i| u32::try_from(i)).collect::<Result<Vec<u32>, _>>(),
        ) else {
            return false;
        };
        nif_ffi::reorder_shape_triangles(file.inner.pin_mut(), shape, &order)
    }

    /// Calculates the normals of the shape from its triangles, adding
    /// normals if it had none. Each vertex gets the normalized sum of its triangles' normals.
    /// Vertices listed in an `NiIntegersExtraData` named `LOCKEDNORM` on the shape keep their
    /// normals.
    ///
    /// Returns `false`, changing nothing, if the shape has no triangle data (an
    /// [`NiTriShape`], [`NiTriStrips`] or [`BsTriShape`]), or is a Skyrim shape whose shader
    /// uses model-space normals and `force_model_space` is off.
    ///
    /// # Compatibility
    ///
    /// `LOCKEDNORM` is a BodySlide and Outfit Studio convention; the games ignore it. See
    /// [`apply_locked_normals`](Self::apply_locked_normals).
    pub fn calc_normals(&mut self, options: Option<NormalOptions>) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        let o = options.unwrap_or_default();
        block_index_to_ffi(shape).is_some_and(|shape| {
            nif_ffi::calc_shape_normals(
                file.inner.pin_mut(),
                shape,
                o.smooth,
                o.smooth_threshold,
                o.force_model_space,
            )
        })
    }

    /// Calculates the tangents and bitangents of the shape from its
    /// triangles, positions, normals and first UV set, adding them if it had none. Tangents
    /// follow the texture's v direction and bitangents its u direction, both made perpendicular
    /// to the normal. Vertices whose triangles give no direction get a tangent built from the
    /// normal. Where the u direction is nearly parallel to the normal, the bitangent may end up
    /// skewed or zero.
    ///
    /// Returns `false`, changing nothing, if the shape has no triangle data (an
    /// [`NiTriShape`], [`NiTriStrips`] or [`BsTriShape`]) or it has no normals or UVs.
    pub fn calc_tangents(&mut self) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        block_index_to_ffi(shape).is_some_and(|shape| nif_ffi::calc_shape_tangents(file.inner.pin_mut(), shape))
    }

    /// Copies the locked normals of `source`, usually a shape in another file, to this shape.
    /// The locked vertices are the ones listed in an `NiIntegersExtraData` named `LOCKEDNORM` on
    /// `source` (see [`ExtraIntegers`](crate::extra_data::ExtraIntegers)); other normals are
    /// kept. This shape's own `LOCKEDNORM` data is replaced with a copy of the source's, so
    /// [`calc_normals`](Self::calc_normals) keeps these normals.
    ///
    /// Returns `false`, changing nothing, if the shape has no normals, `source` has no
    /// `LOCKEDNORM` indices or normals, or the two have different vertex counts. On success, a
    /// replaced `LOCKEDNORM` block no other block uses is deleted, which moves every later block
    /// down one index, and the copy is added as the last block.
    ///
    /// # Compatibility
    ///
    /// `LOCKEDNORM` is a BodySlide and Outfit Studio convention; the games ignore it. Outfit
    /// Studio's "Lock Normals" stores the locked vertices there, and BodySlide keeps their normals
    /// when it recalculates normals for a built mesh. It's used where separate meshes meet, such
    /// as a body's neck and wrists: the edge normals are tuned by hand so the lighting matches
    /// across the seam. To do the same here after reshaping a mesh that has the same vertices as
    /// a reference (a generated body variant, say), recalculate its normals, then copy the
    /// reference's seam normals back:
    ///
    /// ```no_run
    /// # use nifly::{file::NifFile, geometry::NiShape, object3d::Vec3};
    /// # fn f(nif: &mut NifFile, shape: usize, reshaped: &[Vec3], reference: &NifFile) {
    /// let reference_shape = reference.shapes().views().next().unwrap();
    /// let mut shape = nif.edit::<NiShape>(shape).unwrap();
    /// shape.set_vertices(reshaped);
    /// shape.calc_normals(None);
    /// shape.apply_locked_normals(reference_shape);
    /// shape.calc_tangents();
    /// # }
    /// ```
    pub fn apply_locked_normals(&mut self, source: View<'_, NiShape>) -> bool {
        let shape = self.index() as u32;
        let (source_file, source_shape) = (source.file(), source.get());
        nif_ffi::apply_locked_normals(self.file_mut().inner.pin_mut(), shape, &source_file.inner, source_shape.as_ffi())
    }

    /// Replaces the segments of the [`BsSubIndexTriShape`]. Triangles
    /// given `None` go into the first segment.
    ///
    /// The triangles are reordered so that each segment's, and within it each subsegment's,
    /// are together, keeping their relative order; triangle indices change. Subsegments without
    /// a `user_slot` are numbered within their segment, from 1.
    ///
    /// Returns `false`, changing nothing, if the shape isn't a [`BsSubIndexTriShape`],
    /// `segments.triangles` doesn't have one entry per triangle, an entry names a segment or
    /// subsegment that doesn't exist, or a `user_slot` is below 30.
    pub fn set_segments(&mut self, segments: &ShapeSegments) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        let Some(shape) = block_index_to_ffi(shape) else {
            return false;
        };
        // Number the parts as shape_segments does.
        let mut first_part = Vec::with_capacity(segments.segments.len());
        let (mut segs, mut subs, mut extra) = (Vec::new(), Vec::new(), Vec::new());
        let mut part = 0usize;
        for seg in &segments.segments {
            first_part.push(part);
            part += 1 + seg.subsegments.len();
            let Ok(count) = u32::try_from(seg.subsegments.len()) else {
                return false;
            };
            segs.push(nif_ffi::SegmentFfi { subsegments: count });
            for sub in &seg.subsegments {
                if sub.user_slot.is_some_and(|slot| slot < 30) {
                    return false;
                }
                let Ok(extra_len) = u32::try_from(sub.extra_data.len()) else {
                    return false;
                };
                subs.push(nif_ffi::SubsegmentFfi { user_slot: sub.user_slot.unwrap_or(0), material: sub.material, extra_len });
                extra.extend_from_slice(&sub.extra_data);
            }
        }
        let parts: Option<Vec<i32>> = segments
            .triangles
            .iter()
            .map(|t| match t {
                None => Some(-1),
                Some(SegmentIndex { segment, subsegment }) => {
                    let seg = segments.segments.get(*segment)?;
                    let offset = match subsegment {
                        None => 0,
                        Some(s) if *s < seg.subsegments.len() => s + 1,
                        Some(_) => return None,
                    };
                    i32::try_from(first_part[*segment] + offset).ok()
                }
            })
            .collect();
        let Some(parts) = parts else {
            return false;
        };
        nif_ffi::set_shape_segments(file.inner.pin_mut(), shape, &segments.ssf_file, &segs, &subs, &extra, &parts)
    }

    /// Replaces the vertex positions of the shape.
    ///
    /// These setters write wherever the shape keeps its vertex data: its
    /// [`geometry_data`](View::geometry_data) or, for a [`BsTriShape`], the shape itself. They
    /// turn the data on if the shape didn't store it. Each returns `false`, changing nothing, if
    /// the shape isn't such a shape or the slice doesn't have one value per vertex.
    ///
    /// A [`BsTriShape`] stores normals, tangents, the y and z of bitangents, and colors at
    /// reduced precision, so they read back rounded. Colors are clamped to `0.0..=1.0`.
    pub fn set_vertices(&mut self, vertices: &[Vec3]) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        let values: Vec<_> = vertices.iter().map(|v| v.to_ffi()).collect();
        file.set_vertex_array(shape, &values, nif_ffi::set_shape_vertices)
    }

    /// Deletes these vertices from the shape; order and repeats don't
    /// matter. Later vertices move down, and everything that refers to vertices follows: every
    /// vertex array, triangles (those using a deleted vertex are dropped), strips, segments, skin
    /// weights and partitions (empty partitions are removed), and `LOCKEDNORM` indices.
    ///
    /// A shape can be left with no vertices or triangles; delete it with
    /// [`delete`](Self::delete) if it's no longer needed. Returns `false`, changing
    /// nothing, if the shape doesn't store vertices or an index isn't one of its
    /// vertices.
    pub fn delete_vertices(&mut self, vertices: &[usize]) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        let (Some(shape), Ok(vertices)) =
            (block_index_to_ffi(shape), vertices.iter().map(|&v| u32::try_from(v)).collect::<Result<Vec<u32>, _>>())
        else {
            return false;
        };
        nif_ffi::delete_shape_vertices(file.inner.pin_mut(), shape, &vertices)
    }

    /// Moves vertex `vertex` of the shape to `position`. Returns `false`,
    /// changing nothing, if the shape doesn't store vertices or has no such vertex.
    /// See [`set_vertices`](Self::set_vertices).
    pub fn set_vertex(&mut self, vertex: usize, position: Vec3) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        let (Some(shape), Ok(vertex)) = (block_index_to_ffi(shape), u32::try_from(vertex)) else {
            return false;
        };
        nif_ffi::set_shape_vertex(file.inner.pin_mut(), shape, vertex, position.to_ffi())
    }

    /// Replaces the normals of the shape. See
    /// [`set_vertices`](Self::set_vertices).
    pub fn set_normals(&mut self, normals: &[Vec3]) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        let values: Vec<_> = normals.iter().map(|v| v.to_ffi()).collect();
        file.set_vertex_array(shape, &values, nif_ffi::set_shape_normals)
    }

    /// Moves the shape by `offset`, in its own coordinates.
    ///
    /// With `weights` (one per vertex), each vertex moves by that fraction of `offset`: 0 leaves
    /// it, 1 moves it fully. Returns `false`, changing nothing, if the shape has no vertices, or `weights` has a different length or a value that isn't finite.
    pub fn offset(&mut self, offset: Vec3, weights: Option<&[f32]>) -> bool {
        self.transform(weights, |p| p + offset, |n| n, |t| t)
    }

    /// Scales the shape by `scale` along each axis, about its own origin.
    /// Normals, tangents and bitangents follow, so lighting stays correct.
    ///
    /// With `weights` (one per vertex), each vertex moves that fraction of the way to its scaled
    /// position, and its normals and tangents turn that fraction of the way. Returns `false`,
    /// changing nothing, if the shape has no vertices, a scale component isn't
    /// positive and finite (use [`mirror`](Self::mirror) to mirror), or `weights` has
    /// a different length or a value that isn't finite.
    pub fn scale(&mut self, scale: Vec3, weights: Option<&[f32]>) -> bool {
        if ![scale.x, scale.y, scale.z].iter().all(|s| s.is_finite() && *s > 0.0) {
            return false;
        }
        // Normals transform by the cofactor matrix, which for a scale is this.
        let cofactor = Vec3::new(scale.y * scale.z, scale.x * scale.z, scale.x * scale.y);
        let mul = |a: Vec3, b: Vec3| Vec3::new(a.x * b.x, a.y * b.y, a.z * b.z);
        self.transform(weights, |p| mul(p, scale), |n| mul(n, cofactor), |t| mul(t, scale))
    }

    /// Rotates the shape about its own origin by `degrees` around the X,
    /// then Y, then Z axis (each counterclockwise looking down the axis towards the origin).
    /// Normals, tangents and bitangents turn with it.
    ///
    /// With `weights` (one per vertex), each vertex moves that fraction of the way to its rotated
    /// position, and its normals and tangents turn that fraction of the way. Returns `false`,
    /// changing nothing, if the shape has no vertices, an angle isn't finite, or
    /// `weights` has a different length or a value that isn't finite.
    pub fn rotate(&mut self, degrees: Vec3, weights: Option<&[f32]>) -> bool {
        if ![degrees.x, degrees.y, degrees.z].iter().all(|d| d.is_finite()) {
            return false;
        }
        let r = rotation(Vec3::Z, degrees.z) * rotation(Vec3::Y, degrees.y) * rotation(Vec3::X, degrees.x);
        self.transform(weights, |p| r * p, |n| r * n, |t| r * t)
    }

    /// Mirrors the shape along the chosen axes, in its own coordinates:
    /// positions, normals, tangents and bitangents have those components negated. When an odd
    /// number of axes is mirrored, each triangle's winding is reversed so the faces still point
    /// outwards. A [`BsTriShape`]'s tangents are recalculated instead.
    ///
    /// Skinning data isn't mirrored. A skinned Skyrim SE [`BsTriShape`] saves its triangles in
    /// its skin partitions, so the reversed winding is lost on save unless
    /// [`update_skin_partitions`](Self::update_skin_partitions) rebuilds them first.
    ///
    /// Returns `false`, changing nothing, if the shape has no vertices, or is an
    /// [`NiTriStrips`] and an odd number of axes is mirrored (its winding can't be reversed).
    pub fn mirror(&mut self, x: bool, y: bool, z: bool) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        block_index_to_ffi(shape).is_some_and(|shape| nif_ffi::mirror_shape(file.inner.pin_mut(), shape, x, y, z))
    }

    /// Flips the first UV set of the shape: each `u` becomes `1 - u` if
    /// `flip_u`, and each `v` becomes `1 - v` if `flip_v`, mirroring the texture horizontally or
    /// vertically. Returns `false`, changing nothing, if the shape has no UVs.
    pub fn invert_uvs(&mut self, flip_u: bool, flip_v: bool) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        block_index_to_ffi(shape).is_some_and(|shape| nif_ffi::invert_shape_uvs(file.inner.pin_mut(), shape, flip_u, flip_v))
    }

    /// Replaces the first UV set of the shape, as `[u, v]`. See
    /// [`set_vertices`](Self::set_vertices).
    pub fn set_uvs(&mut self, uvs: &[[f32; 2]]) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        let values: Vec<_> = uvs.iter().map(|&[u, v]| nif_ffi::Uv { u, v }).collect();
        file.set_vertex_array(shape, &values, nif_ffi::set_shape_uvs)
    }

    /// Replaces the vertex colors of the shape, as `[r, g, b, a]`. See
    /// [`set_vertices`](Self::set_vertices).
    pub fn set_vertex_colors(&mut self, colors: &[[f32; 4]]) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        let values: Vec<_> = colors.iter().map(|&[r, g, b, a]| nif_ffi::Color { r, g, b, a }).collect();
        file.set_vertex_array(shape, &values, nif_ffi::set_shape_colors)
    }

    /// Replaces the tangents of the shape. Turning tangents on also
    /// stores bitangents, zeroed if the shape had none. See
    /// [`set_vertices`](Self::set_vertices).
    pub fn set_tangents(&mut self, tangents: &[Vec3]) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        let values: Vec<_> = tangents.iter().map(|v| v.to_ffi()).collect();
        file.set_vertex_array(shape, &values, nif_ffi::set_shape_tangents)
    }

    /// Replaces the bitangents of the shape. Turning bitangents on also
    /// stores tangents, zeroed if the shape had none. See
    /// [`set_vertices`](Self::set_vertices).
    pub fn set_bitangents(&mut self, bitangents: &[Vec3]) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        let values: Vec<_> = bitangents.iter().map(|v| v.to_ffi()).collect();
        file.set_vertex_array(shape, &values, nif_ffi::set_shape_bitangents)
    }

    /// Replaces the eye data of the [`BsTriShape`]. Returns `false`,
    /// changing nothing, if the shape isn't a [`BsTriShape`] or `eye_data` doesn't have one
    /// value per vertex.
    pub fn set_eye_data(&mut self, eye_data: &[f32]) -> bool {
        let (shape, file) = (self.index(), self.file_mut());
        file.set_vertex_array(shape, eye_data, nif_ffi::set_shape_eye_data)
    }
}

/// A counterclockwise rotation by `degrees` about the unit `axis` (X, Y or Z).
fn rotation(axis: Vec3, degrees: f32) -> Matrix3 {
    let (s, c) = degrees.to_radians().sin_cos();
    if axis == Vec3::X {
        Matrix3::from_rows(Vec3::X, Vec3::new(0.0, c, -s), Vec3::new(0.0, s, c))
    } else if axis == Vec3::Y {
        Matrix3::from_rows(Vec3::new(c, 0.0, s), Vec3::Y, Vec3::new(-s, 0.0, c))
    } else {
        Matrix3::from_rows(Vec3::new(c, -s, 0.0), Vec3::new(s, c, 0.0), Vec3::Z)
    }
}

/// The additional data of an [`NiGeometryData`].
impl<'a, T: NiType> View<'a, T> {
    /// The data's [`AdditionalGeomData`], or `None` if it has none. Saved from version 20.0.0.4
    /// on, so not in Morrowind or early Oblivion files.
    pub fn additional_data<I>(&self) -> Option<View<'a, AdditionalGeomData>>
    where
        T: IsA<NiGeometryData, I>,
    {
        let data: &NiGeometryData = self.get().upcast_ref();
        self.resolve(index_from_ffi(ffi::geometry_data_additional_data(&data.0)))
    }
}

impl<T: NiType> Edit<'_, T> {
    /// Like [`View::additional_data`], for editing.
    pub fn additional_data<I>(&mut self) -> Option<Edit<'_, AdditionalGeomData>>
    where
        T: IsA<NiGeometryData, I>,
    {
        let index = self.view().additional_data()?.index();
        self.resolve(Some(index))
    }

    /// Gives the data a new `A`, with the data's vertex count and no channels or blocks, as its
    /// additional data, replacing any it had, and returns the new block's index. It's added at
    /// the end. Replaced additional data stays in the file until saving drops it, if nothing
    /// else uses it.
    ///
    /// ```
    /// # use nifly::{file::NifFile, geometry::{NiAdditionalGeometryData, NiGeometryData}};
    /// # let file = std::fs::read("nifly-sys/vendor/nifly/tests/input/TestNifFile_Skinned_OB.nif").unwrap();
    /// # let mut nif = NifFile::load(file.as_slice(), None).unwrap();
    /// let data = nif.blocks_of::<NiGeometryData>().indices().next().unwrap();
    /// let extra = nif.edit::<NiGeometryData>(data).unwrap().assign_additional_data::<NiAdditionalGeometryData, _>();
    /// ```
    pub fn assign_additional_data<A: NewAdditionalData, I>(&mut self) -> Index
    where
        T: IsA<NiGeometryData, I>,
    {
        let data = self.index() as u32;
        index_from_ffi(nif_ffi::assign_additional_geometry_data(self.file_mut().inner.pin_mut(), data, A::PACKED))
            .expect("the block is geometry data")
    }

    /// Clears the data's additional data. It stays in the file until saving drops it, if
    /// nothing else uses it. Returns `false` if the data had none.
    pub fn remove_additional_data<I>(&mut self) -> bool
    where
        T: IsA<NiGeometryData, I>,
    {
        let data = self.index() as u32;
        nif_ffi::remove_additional_geometry_data(self.file_mut().inner.pin_mut(), data)
    }

    /// Replaces the texture coordinate sets and their count in the
    /// [data flags](NiGeometryData::data_raw_flags). Returns `false`, changing nothing, unless each
    /// set has one UV per vertex and there are at most 63 sets, or one from Fallout 3 on.
    pub fn set_uv_sets<S: AsRef<[[f32; 2]]>, I>(&mut self, sets: &[S]) -> bool
    where
        T: IsA<NiGeometryData, I>,
    {
        let bethesda = self.view().file().version().stream >= 34;
        let flat: Vec<f32> = sets.iter().flat_map(|s| s.as_ref().as_flattened().iter().copied()).collect();
        let index = self.index();
        let data = self.file_mut().block_of_mut::<NiGeometryData>(index).expect("the block is geometry data");
        ffi::set_geometry_data_uv_sets(data.ffi_mut(), &flat, sets.len(), bethesda)
    }

    /// Sets the [data flags](NiGeometryData::data_raw_flags), adding zeroed UV sets or removing
    /// them to match the count, and adding zeroed tangents and bitangents or removing them to
    /// match the tangent method.
    pub fn set_data_raw_flags<I>(&mut self, flags: u16)
    where
        T: IsA<NiGeometryData, I>,
    {
        let bethesda = self.view().file().version().stream >= 34;
        let index = self.index();
        let data = self.file_mut().block_of_mut::<NiGeometryData>(index).expect("the block is geometry data");
        ffi::set_geometry_data_flags(data.ffi_mut(), flags, bethesda);
    }

    /// Points the data at the [`AdditionalGeomData`] at `extra`, or clears it with `None`.
    /// Returns `false`, changing nothing, if that block isn't one.
    pub fn set_additional_data<I>(&mut self, extra: Option<Index>) -> bool
    where
        T: IsA<NiGeometryData, I>,
    {
        self.set_ref(RefKind::AdditionalData, extra)
    }
}
