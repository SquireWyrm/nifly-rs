//! `NiNode` and its subclasses.
//!
//! Each type derefs to its direct base class, so base-class methods are available
//! on every subclass. Use [`NiObject::downcast`](crate::objects::NiObject::downcast) to go the other way.

use std::pin::Pin;

use bitflags::bitflags;
use bstr::{BStr, BString, ByteSlice};
use int_enum::IntEnum;
use nifly_sys::{
    nif_file::ffi as nif_ffi,
    nif_file_refs::ffi::{RefKind, RefList},
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
        pub fn $set(&mut self, value: $ty) {
            ffi::$ffi_set(self.ffi_mut(), value);
        }
    )*};
}

/// Getter and setter pairs for `Vec3` fields.
macro_rules! vec3_fields {
    ($($(#[$doc:meta])* $get:ident, $set:ident = $ffi_get:ident, $ffi_set:ident;)*) => {$(
        $(#[$doc])*
        pub fn $get(&self) -> Vec3 {
            Vec3::from_ffi(ffi::$ffi_get(&self.0))
        }

        #[doc = concat!("Sets [`", stringify!($get), "`](Self::", stringify!($get), ").")]
        pub fn $set(&mut self, value: Vec3) {
            ffi::$ffi_set(self.ffi_mut(), value.to_ffi());
        }
    )*};
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

mod bethesda;
mod gamebryo;
mod handles;

pub use self::bethesda::*;
pub use self::gamebryo::*;
pub use self::handles::*;
