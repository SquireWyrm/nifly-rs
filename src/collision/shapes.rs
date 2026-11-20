//! Primitive, container and MOPP shapes, and shape refs.

use super::*;

impl BhkHeightFieldShape {
    fields! {
        /// The Havok material, as a raw value whose meaning depends on the game.
        material, set_material: u32 = height_field_material, set_height_field_material;
    }
}

/// Vectors are `[x, y, z, w]` in Havok units.
impl BhkPlaneShape {
    converted_fields! {
        /// A vector of unknown meaning.
        unknown_vector, set_unknown_vector: Vec3 =
            plane_shape_unknown_vector, set_plane_shape_unknown_vector, Vec3::from_ffi, Vec3::to_ffi;
    }

    /// The plane.
    pub fn plane(&self) -> Plane {
        Plane {
            normal: Vec3::from_ffi(ffi::plane_shape_normal(&self.0)),
            constant: ffi::plane_shape_constant(&self.0),
        }
    }

    /// Sets the [plane](Self::plane).
    pub fn set_plane(&mut self, plane: Plane) {
        ffi::set_plane_shape_normal(self.ffi_mut(), plane.normal.to_ffi());
        ffi::set_plane_shape_constant(self.ffi_mut(), plane.constant);
    }

    fields! {
        /// The half extents of the part of the plane that's used.
        half_extents, set_half_extents: [f32; 4] = plane_shape_half_extents, set_plane_shape_half_extents;
        /// The center of the part of the plane that's used.
        center, set_center: [f32; 4] = plane_shape_center, set_plane_shape_center;
    }
}

impl BhkSphereRepShape {
    fields! {
        /// The Havok material, as a raw value whose meaning depends on the game.
        material, set_material: u32 = sphere_rep_material, set_sphere_rep_material;
    }
}

impl BhkConvexShape {
    fields! {
        /// The radius around the shape, in Havok units. For a [`BhkSphereShape`], the sphere's
        /// radius.
        radius, set_radius: f32 = convex_shape_radius, set_convex_shape_radius;
    }
}

impl BhkMultiSphereShape {
    converted_fields! {
        shape_property, set_shape_property: CInfoProperty =
            multi_sphere_property, set_multi_sphere_property, CInfoProperty::from_ffi, CInfoProperty::to_ffi;
    }

    /// The spheres, in Havok units.
    pub fn spheres(&self) -> Vec<BoundingSphere> {
        ffi::multi_sphere_spheres(&self.0).into_iter().map(BoundingSphere::from_ffi).collect()
    }

    pub fn set_spheres(&mut self, spheres: &[BoundingSphere]) {
        let spheres: Vec<_> = spheres.iter().map(|s| s.to_ffi()).collect();
        ffi::set_multi_sphere_spheres(self.ffi_mut(), &spheres);
    }
}

/// Vectors are `[x, y, z, w]` in Havok units.
impl BhkConvexVerticesShape {
    converted_fields! {
        vertices_property, set_vertices_property: CInfoProperty =
            convex_vertices_vertices_property, set_convex_vertices_vertices_property,
            CInfoProperty::from_ffi, CInfoProperty::to_ffi;
        normals_property, set_normals_property: CInfoProperty =
            convex_vertices_normals_property, set_convex_vertices_normals_property,
            CInfoProperty::from_ffi, CInfoProperty::to_ffi;
    }

    /// The vertices whose convex hull is the shape.
    pub fn vertices(&self) -> Vec<[f32; 4]> {
        ffi::convex_vertices_list(&self.0, false).as_chunks::<4>().0.to_vec()
    }

    pub fn set_vertices(&mut self, vertices: &[[f32; 4]]) {
        ffi::set_convex_vertices_list(self.ffi_mut(), false, vertices.as_flattened());
    }

    /// The hull's faces as planes: an outward normal in `x`, `y` and `z`, and the plane's
    /// distance from the origin, negated, in `w`. They aren't updated when the
    /// [vertices](Self::vertices) change.
    pub fn normals(&self) -> Vec<[f32; 4]> {
        ffi::convex_vertices_list(&self.0, true).as_chunks::<4>().0.to_vec()
    }

    pub fn set_normals(&mut self, normals: &[[f32; 4]]) {
        ffi::set_convex_vertices_list(self.ffi_mut(), true, normals.as_flattened());
    }
}

impl BhkBoxShape {
    converted_fields! {
        /// Half the box's size along each axis, in Havok units.
        dimensions, set_dimensions: Vec3 = box_shape_dimensions, set_box_shape_dimensions, Vec3::from_ffi, Vec3::to_ffi;
    }
}

/// Points are `[x, y, z, w]` in Havok units.
impl BhkCylinderShape {
    fields! {
        /// The center of one end.
        vertex_a, set_vertex_a: [f32; 4] = cylinder_shape_vertex_a, set_cylinder_shape_vertex_a;
        /// The center of the other end.
        vertex_b, set_vertex_b: [f32; 4] = cylinder_shape_vertex_b, set_cylinder_shape_vertex_b;
        /// The cylinder's radius.
        cylinder_radius, set_cylinder_radius: f32 = cylinder_shape_radius, set_cylinder_shape_radius;
    }
}

/// The capsule's own radius is its [convex radius](BhkConvexShape::radius). Points are in Havok
/// units.
impl BhkCapsuleShape {
    converted_fields! {
        /// The center of one end.
        point_1, set_point_1: Vec3 = capsule_shape_point_1, set_capsule_shape_point_1, Vec3::from_ffi, Vec3::to_ffi;
        /// The center of the other end.
        point_2, set_point_2: Vec3 = capsule_shape_point_2, set_capsule_shape_point_2, Vec3::from_ffi, Vec3::to_ffi;
    }

    fields! {
        /// The radius at [`point_1`](Self::point_1).
        radius_1, set_radius_1: f32 = capsule_shape_radius_1, set_capsule_shape_radius_1;
        /// The radius at [`point_2`](Self::point_2).
        radius_2, set_radius_2: f32 = capsule_shape_radius_2, set_capsule_shape_radius_2;
    }
}

/// Its child shape is on [`View::shape`].
impl BhkTransformShape {
    fields! {
        /// The Havok material, as a raw value whose meaning depends on the game.
        material, set_material: u32 = transform_shape_material, set_transform_shape_material;
        /// The radius around the shape, in Havok units.
        radius, set_radius: f32 = transform_shape_radius, set_transform_shape_radius;
    }

    converted_fields! {
        /// The transform applied to the child shape, as four rows in Havok units.
        transform, set_transform: [[f32; 4]; 4] =
            transform_shape_transform, set_transform_shape_transform, rows, flatten;
    }
}

/// Its sub-shapes are on [`View::sub_shapes`].
impl BhkListShape {
    fields! {
        /// The Havok material, as a raw value whose meaning depends on the game.
        material, set_material: u32 = list_shape_material, set_list_shape_material;
    }

    converted_fields! {
        child_shape_property, set_child_shape_property: CInfoProperty =
            list_shape_child_shape_property, set_list_shape_child_shape_property,
            CInfoProperty::from_ffi, CInfoProperty::to_ffi;
        child_filter_property, set_child_filter_property: CInfoProperty =
            list_shape_child_filter_property, set_list_shape_child_filter_property,
            CInfoProperty::from_ffi, CInfoProperty::to_ffi;
    }

    /// The sub-shapes' collision filters, usually one per sub-shape.
    pub fn filters(&self) -> Vec<HavokFilter> {
        ffi::list_shape_filters(&self.0).into_iter().map(HavokFilter::from_ffi).collect()
    }

    /// Replaces the [filters](Self::filters). Returns `false`, changing nothing, unless there's
    /// one per sub-shape.
    pub fn set_filters(&mut self, filters: &[HavokFilter]) -> bool {
        let filters: Vec<_> = filters.iter().map(|f| f.to_ffi()).collect();
        ffi::set_list_shape_filters(self.ffi_mut(), &filters)
    }
}

/// Its sub-shapes are on [`View::sub_shapes`].
impl BhkConvexListShape {
    fields! {
        /// The Havok material, as a raw value whose meaning depends on the game.
        material, set_material: u32 = convex_list_material, set_convex_list_material;
        /// The radius around the shape, in Havok units.
        radius, set_radius: f32 = convex_list_radius, set_convex_list_radius;
        /// A value of unknown meaning.
        unknown_int, set_unknown_int: u32 = convex_list_unknown_int, set_convex_list_unknown_int;
        /// A value of unknown meaning.
        unknown_float, set_unknown_float: f32 = convex_list_unknown_float, set_convex_list_unknown_float;
        use_cached_aabb, set_use_cached_aabb: bool = convex_list_use_cached_aabb, set_convex_list_use_cached_aabb;
        closest_point_min_distance, set_closest_point_min_distance: f32 =
            convex_list_closest_point_min_distance, set_convex_list_closest_point_min_distance;
    }

    converted_fields! {
        child_shape_property, set_child_shape_property: CInfoProperty =
            convex_list_child_shape_property, set_convex_list_child_shape_property,
            CInfoProperty::from_ffi, CInfoProperty::to_ffi;
    }
}

/// Its child shape is on [`View::shape`]. The MOPP code describes that shape, and isn't rebuilt
/// when it or the shape changes.
impl BhkMoppBvTreeShape {
    /// The MOPP code.
    pub fn mopp_code(&self) -> &[u8] {
        ffi::mopp_code(&self.0)
    }

    pub fn set_mopp_code(&mut self, code: &[u8]) {
        ffi::set_mopp_code(self.ffi_mut(), code);
    }

    fields! {
        /// The code's origin, `[x, y, z, w]` in Havok units.
        mopp_offset, set_mopp_offset: [f32; 4] = mopp_offset, set_mopp_offset;
        /// The code's scale.
        mopp_scale, set_mopp_scale: f32 = mopp_scale, set_mopp_scale;
        /// How the code was built, as a raw value. Only saved from Fallout 3 on.
        build_type, set_build_type: u8 = mopp_build_type, set_mopp_build_type;
    }
}

impl HasShape for BhkTransformShape {}
impl HasShape for BhkConvexTransformShape {}
impl HasShape for BhkMoppBvTreeShape {}

/// A shape with a list of sub-shapes: a [`BhkListShape`] or [`BhkConvexListShape`].
///
/// This trait is sealed.
pub trait HasSubShapes: NiType {}

impl HasSubShapes for BhkListShape {}
impl HasSubShapes for BhkConvexListShape {}

/// Sub-shapes and mesh data.
impl<'a, T: NiType> View<'a, T> {
    /// The list shape's sub-shapes. Skips empty refs.
    pub fn sub_shapes(&self) -> BlockIter<'a, BhkShape, RefIndices>
    where
        T: HasSubShapes,
    {
        self.follow_list(RefList::SubShapes)
    }

    /// The strips shape's triangle data. Skips empty refs.
    pub fn strips_data<I>(&self) -> BlockIter<'a, NiTriStripsData, RefIndices>
    where
        T: IsA<BhkNiTriStripsShape, I>,
    {
        self.follow_list(RefList::StripsData)
    }

    /// The scene object the compressed mesh shape was made from, or `None` if it has none.
    pub fn mesh_target<I>(&self) -> Option<View<'a, NiAvObject>>
    where
        T: IsA<BhkCompressedMeshShape, I>,
    {
        self.follow(RefKind::CompressedMeshTarget)
    }

    /// The compressed mesh shape's data, or `None` if it has none.
    pub fn compressed_data<I>(&self) -> Option<View<'a, BhkCompressedMeshShapeData>>
    where
        T: IsA<BhkCompressedMeshShape, I>,
    {
        self.follow(RefKind::CompressedMeshData)
    }

    /// The packed shape's data, or `None` if it has none.
    pub fn packed_data<I>(&self) -> Option<View<'a, HkPackedNiTriStripsData>>
    where
        T: IsA<BhkPackedNiTriStripsShape, I>,
    {
        self.follow(RefKind::PackedData)
    }
}

impl<T: NiType> Edit<'_, T> {
    /// Adds a new shape of type `S` with default settings as the block's [shape](View::shape),
    /// and returns its index. A previous shape stays in the file until it's optimized away.
    ///
    /// ```
    /// # use nifly::{collision::*, file::{NifFile, NifVersion}};
    /// let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    /// let collision = nif.root_node_mut().unwrap().assign_collision::<BhkCollisionObject, _>();
    /// let mut collision = nif.edit::<BhkCollisionObject>(collision).unwrap();
    /// let body = collision.assign_body::<BhkRigidBody, _>().unwrap();
    /// let shape = nif.edit::<BhkRigidBody>(body).unwrap().assign_shape::<BhkBoxShape>();
    /// assert_eq!(nif.view::<BhkRigidBody>(body).unwrap().shape().unwrap().index(), shape);
    /// ```
    pub fn assign_shape<S: NewShape>(&mut self) -> Index
    where
        T: HasShape,
    {
        let owner = self.index() as u32;
        index_from_ffi(S::assign(self.file_mut().inner.pin_mut(), owner)).expect("the block has a shape ref")
    }

    /// Replaces the list shape's [sub-shapes](View::sub_shapes). A [`BhkListShape`]'s
    /// [filters](BhkListShape::filters) are resized to match if there was one per sub-shape,
    /// new ones default.
    ///
    /// Returns `false`, changing nothing, unless each is a [`BhkShape`] (a [`BhkConvexShape`]
    /// for a [`BhkConvexListShape`]) that doesn't contain this shape.
    pub fn set_sub_shapes(&mut self, shapes: &[Index]) -> bool
    where
        T: HasSubShapes,
    {
        self.set_ref_list(RefList::SubShapes, shapes)
    }

    /// Adds a new shape of type `S` with default settings at the end of the list shape's
    /// [sub-shapes](View::sub_shapes), and returns its index. A [`BhkListShape`] with one
    /// [filter](BhkListShape::filters) per sub-shape gets a default one for it.
    ///
    /// Returns `None`, changing nothing, if this is a [`BhkConvexListShape`] and `S` isn't a
    /// [`BhkConvexShape`].
    pub fn add_sub_shape<S: NewShape>(&mut self) -> Option<Index>
    where
        T: HasSubShapes,
    {
        let owner = self.index() as u32;
        index_from_ffi(S::add_sub(self.file_mut().inner.pin_mut(), owner))
    }

    /// Replaces the strips shape's [triangle data](View::strips_data). Its
    /// [filters](BhkNiTriStripsShape::filters) are resized to match if there was one per ref,
    /// new ones zero. Returns `false`, changing nothing, unless each is an [`NiTriStripsData`].
    pub fn set_strips_data<I>(&mut self, data: &[Index]) -> bool
    where
        T: IsA<BhkNiTriStripsShape, I>,
    {
        self.set_ref_list(RefList::StripsData, data)
    }

    /// Like [`View::mesh_target`], for editing.
    pub fn mesh_target<I>(&mut self) -> Option<Edit<'_, NiAvObject>>
    where
        T: IsA<BhkCompressedMeshShape, I>,
    {
        self.follow(RefKind::CompressedMeshTarget)
    }

    /// Sets the compressed mesh shape's [target](View::mesh_target), or clears it with `None`.
    /// Returns `false`, changing nothing, unless `target` is an [`NiAvObject`].
    pub fn set_mesh_target<I>(&mut self, target: Option<Index>) -> bool
    where
        T: IsA<BhkCompressedMeshShape, I>,
    {
        self.set_ref(RefKind::CompressedMeshTarget, target)
    }

    /// Like [`View::compressed_data`], for editing.
    pub fn compressed_data<I>(&mut self) -> Option<Edit<'_, BhkCompressedMeshShapeData>>
    where
        T: IsA<BhkCompressedMeshShape, I>,
    {
        self.follow(RefKind::CompressedMeshData)
    }

    /// Sets the compressed mesh shape's [data](View::compressed_data), or clears it with
    /// `None`. Returns `false`, changing nothing, unless `data` is a
    /// [`BhkCompressedMeshShapeData`].
    pub fn set_compressed_data<I>(&mut self, data: Option<Index>) -> bool
    where
        T: IsA<BhkCompressedMeshShape, I>,
    {
        self.set_ref(RefKind::CompressedMeshData, data)
    }

    /// Adds new, empty [data](View::compressed_data) to the compressed mesh shape and returns
    /// its index. The previous data stays in the file until it's optimized away.
    pub fn assign_compressed_data<I>(&mut self) -> Index
    where
        T: IsA<BhkCompressedMeshShape, I>,
    {
        let shape = self.index() as u32;
        index_from_ffi(ffi::assign_compressed_data(self.file_mut().inner.pin_mut(), shape))
            .expect("the block is a compressed mesh shape")
    }

    /// Like [`View::packed_data`], for editing.
    pub fn packed_data<I>(&mut self) -> Option<Edit<'_, HkPackedNiTriStripsData>>
    where
        T: IsA<BhkPackedNiTriStripsShape, I>,
    {
        self.follow(RefKind::PackedData)
    }

    /// Sets the packed shape's [data](View::packed_data), or clears it with `None`. Returns
    /// `false`, changing nothing, unless `data` is an [`HkPackedNiTriStripsData`].
    pub fn set_packed_data<I>(&mut self, data: Option<Index>) -> bool
    where
        T: IsA<BhkPackedNiTriStripsShape, I>,
    {
        self.set_ref(RefKind::PackedData, data)
    }

    /// Adds new, empty [data](View::packed_data) to the packed shape and returns its index. The
    /// previous data stays in the file until it's optimized away.
    pub fn assign_packed_data<I>(&mut self) -> Index
    where
        T: IsA<BhkPackedNiTriStripsShape, I>,
    {
        let shape = self.index() as u32;
        index_from_ffi(ffi::assign_packed_data(self.file_mut().inner.pin_mut(), shape))
            .expect("the block is a packed shape")
    }
}

/// A shape type that [`Edit::assign_shape`] and [`Edit::add_sub_shape`] can create.
///
/// This trait is sealed.
pub trait NewShape: NiType {
    #[doc(hidden)]
    fn assign(file: Pin<&mut nif_ffi::NifFile>, owner: u32) -> u32;
    #[doc(hidden)]
    fn add_sub(file: Pin<&mut nif_ffi::NifFile>, owner: u32) -> u32;
}

macro_rules! new_shape {
    ($($ty:ty => $assign:ident, $add_sub:ident;)*) => {$(
        impl NewShape for $ty {
            fn assign(file: Pin<&mut nif_ffi::NifFile>, owner: u32) -> u32 {
                ffi::$assign(file, owner)
            }

            fn add_sub(file: Pin<&mut nif_ffi::NifFile>, owner: u32) -> u32 {
                ffi::$add_sub(file, owner)
            }
        }
    )*};
}

new_shape! {
    BhkPlaneShape => assign_bhk_plane_shape, add_sub_bhk_plane_shape;
    BhkMultiSphereShape => assign_bhk_multi_sphere_shape, add_sub_bhk_multi_sphere_shape;
    BhkConvexListShape => assign_bhk_convex_list_shape, add_sub_bhk_convex_list_shape;
    BhkConvexVerticesShape => assign_bhk_convex_vertices_shape, add_sub_bhk_convex_vertices_shape;
    BhkBoxShape => assign_bhk_box_shape, add_sub_bhk_box_shape;
    BhkSphereShape => assign_bhk_sphere_shape, add_sub_bhk_sphere_shape;
    BhkCylinderShape => assign_bhk_cylinder_shape, add_sub_bhk_cylinder_shape;
    BhkTransformShape => assign_bhk_transform_shape, add_sub_bhk_transform_shape;
    BhkConvexTransformShape => assign_bhk_convex_transform_shape, add_sub_bhk_convex_transform_shape;
    BhkCapsuleShape => assign_bhk_capsule_shape, add_sub_bhk_capsule_shape;
    BhkMoppBvTreeShape => assign_bhk_mopp_bv_tree_shape, add_sub_bhk_mopp_bv_tree_shape;
    BhkNiTriStripsShape => assign_bhk_ni_tri_strips_shape, add_sub_bhk_ni_tri_strips_shape;
    BhkListShape => assign_bhk_list_shape, add_sub_bhk_list_shape;
    BhkPackedNiTriStripsShape => assign_bhk_packed_ni_tri_strips_shape, add_sub_bhk_packed_ni_tri_strips_shape;
    BhkCompressedMeshShape => assign_bhk_compressed_mesh_shape, add_sub_bhk_compressed_mesh_shape;
}
