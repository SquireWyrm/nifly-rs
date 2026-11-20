//! Per-type node handles, adding nodes, and object palettes.

use super::*;

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
        BlockIter::new(
            self.file(),
            RefIndices::new(ffi::tree_node_bones(&self.get().0, 1)),
        )
    }

    /// Like [`bones_1`](Self::bones_1), for the second bone list (the branch bones of Skyrim
    /// trees).
    pub fn bones_2(&self) -> BlockIter<'a, NiNode, RefIndices> {
        BlockIter::new(
            self.file(),
            RefIndices::new(ffi::tree_node_bones(&self.get().0, 2)),
        )
    }
}

impl Edit<'_, BsTreeNode> {
    /// Replaces the first bone list with these nodes. Returns `false`, changing nothing, if a
    /// bone isn't a node.
    pub fn set_bones_1(&mut self, bones: &[Index]) -> bool {
        self.set_tree_bones(1, bones)
    }

    /// Like [`set_bones_1`](Self::set_bones_1), for the second bone list.
    pub fn set_bones_2(&mut self, bones: &[Index]) -> bool {
        self.set_tree_bones(2, bones)
    }

    fn set_tree_bones(&mut self, list: u8, bones: &[Index]) -> bool {
        let Some(bones) = bones
            .iter()
            .map(|&b| block_index_to_ffi(b))
            .collect::<Option<Vec<_>>>()
        else {
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
    /// let data = node.multi_bound().unwrap().multi_bound_data().unwrap();
    /// assert!(data.cast::<BsMultiBoundAabb>().is_some());
    /// ```
    pub fn assign_multi_bound(&mut self, shape: MultiBoundShape) -> Index {
        let node = self.index() as u32;
        let file = self.file_mut().inner.pin_mut();
        let index = match shape {
            MultiBoundShape::Aabb {
                center,
                half_extent,
            } => {
                nif_ffi::assign_multi_bound_aabb(file, node, center.to_ffi(), half_extent.to_ffi())
            }
            MultiBoundShape::Obb {
                center,
                size,
                rotation,
            } => nif_ffi::assign_multi_bound_obb(
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
    pub fn multi_bound_data(&self) -> Option<View<'a, BsMultiBoundData>> {
        self.resolve(index_from_ffi(ffi::multi_bound_data(&self.get().0)))
    }
}

impl Edit<'_, BsMultiBound> {
    /// Like [`View::multi_bound_data`], for editing.
    pub fn multi_bound_data(&mut self) -> Option<Edit<'_, BsMultiBoundData>> {
        let index = self.view().multi_bound_data()?.index();
        self.resolve(Some(index))
    }

    /// Points the multi-bound at the [`BsMultiBoundData`] at `data`, or clears it with `None`.
    /// Returns `false`, changing nothing, if that block isn't one.
    pub fn set_multi_bound_data(&mut self, data: Option<Index>) -> bool {
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
        index_from_ffi(T::assign(self.file_mut().inner.pin_mut(), node))
            .expect("the block is an NiLodNode")
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
        let Some(scene) = optional_index_to_ffi(scene) else {
            return false;
        };
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
    fn add(
        file: Pin<&mut nif_ffi::NifFile>,
        name: &[u8],
        transform: nif_ffi::Transform,
        parent: u32,
    ) -> u32;
}

impl NewNode for NiNode {
    fn add(
        file: Pin<&mut nif_ffi::NifFile>,
        name: &[u8],
        transform: nif_ffi::Transform,
        parent: u32,
    ) -> u32 {
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
    pub fn scene_root(&self) -> Option<View<'a, NiAvObject>> {
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
    /// Like [`View::scene_root`], for editing.
    pub fn scene_root(&mut self) -> Option<Edit<'_, NiAvObject>> {
        let index = self.view().scene_root()?.index();
        self.resolve(Some(index))
    }

    /// Sets the scene to the [`NiAvObject`] at block index `scene`, or none if `None`. Returns
    /// `false`, changing nothing, if that block isn't an `NiAvObject`.
    pub fn set_scene_root(&mut self, scene: Option<Index>) -> bool {
        let Some(scene) = optional_index_to_ffi(scene) else {
            return false;
        };
        let palette = self.index() as u32;
        ffi::set_palette_scene(self.file_mut().inner.pin_mut(), palette, scene)
    }

    /// Replaces the entries with `objects`: names and the block indices of the
    /// [`NiAvObject`]s they refer to, or `None` for empty entries. Names aren't checked against
    /// the objects. Returns `false`, changing nothing, if a block isn't an `NiAvObject`.
    pub fn set_objects<N: AsRef<[u8]>>(
        &mut self,
        objects: impl IntoIterator<Item = (N, Option<Index>)>,
    ) -> bool {
        let (mut names, mut indices) = (Vec::new(), Vec::new());
        for (name, index) in objects {
            let Some(index) = optional_index_to_ffi(index) else {
                return false;
            };
            names.push(name);
            indices.push(index);
        }
        let (bytes, lengths) = joined(names);
        let palette = self.index() as u32;
        ffi::set_palette_objects(
            self.file_mut().inner.pin_mut(),
            palette,
            &bytes,
            &lengths,
            &indices,
        )
    }
}
