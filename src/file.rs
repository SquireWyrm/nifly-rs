//! [`NifFile`], a NIF file in memory, with its version and its load, save and optimize options.
//!
//! Blocks are addressed by [`Index`], their position in the file's block list.

use std::io;

use bstr::BString;
use cxx::UniquePtr;
use nifly_sys::{
    nif_file::ffi, nif_file_refs::ffi::RefList, objects::ffi as objects_ffi,
    read_bridge::RustReader, write_bridge::RustWriter,
};

use crate::{
    error::{Error, LoadError, NifResult, SaveError},
    extra_data::{IntoExtraData, NiExtraData},
    handle::{Edit, View},
    iter::{BlockIter, RefIndices, TreeIndices},
    objects::{IsA, NiObject, NiObjectNet, NiType},
};

/// A NIF format version: the file version plus the user version and stream (user version 2)
/// that Bethesda games use to tell their variants apart.
///
/// The associated constants are the versions each game writes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NifVersion {
    /// File version, one byte per component: 20.2.0.7 is `0x14020007`.
    pub file: u32,
    /// User version.
    pub user: u32,
    /// Stream, also called user version 2 or the Bethesda stream version.
    pub stream: u32,
}

impl NifVersion {
    /// Morrowind: 4.0.0.2.
    pub const MORROWIND: Self = Self::new(0x04000002, 0, 0);
    /// Oblivion: 20.0.0.5, user version 11, stream 11.
    pub const OBLIVION: Self = Self::new(0x14000005, 11, 11);
    /// Fallout 3 and New Vegas: 20.2.0.7, user version 11, stream 34.
    pub const FALLOUT_3: Self = Self::new(0x14020007, 11, 34);
    /// Skyrim (LE): 20.2.0.7, user version 12, stream 83.
    pub const SKYRIM: Self = Self::new(0x14020007, 12, 83);
    /// Skyrim Special Edition: 20.2.0.7, user version 12, stream 100.
    pub const SKYRIM_SE: Self = Self::new(0x14020007, 12, 100);
    /// Fallout 4: 20.2.0.7, user version 12, stream 130.
    pub const FALLOUT_4: Self = Self::new(0x14020007, 12, 130);
    /// Fallout 76: 20.2.0.7, user version 12, stream 155.
    pub const FALLOUT_76: Self = Self::new(0x14020007, 12, 155);
    /// Starfield: 20.2.0.7, user version 12, stream 172.
    pub const STARFIELD: Self = Self::new(0x14020007, 12, 172);

    /// A version from its file version, user version and stream.
    ///
    /// ```
    /// # use nifly::file::NifVersion;
    /// assert_eq!(NifVersion::new(0x14020007, 12, 100), NifVersion::SKYRIM_SE);
    /// ```
    pub const fn new(file: u32, user: u32, stream: u32) -> Self {
        Self { file, user, stream }
    }

    /// The file version as `[major, minor, patch, internal]`, e.g. `[20, 2, 0, 7]`.
    pub const fn file_parts(&self) -> [u8; 4] {
        self.file.to_be_bytes()
    }

    fn to_ffi(self) -> ffi::NifVersion {
        ffi::NifVersion {
            file: self.file,
            user: self.user,
            stream: self.stream,
        }
    }

    fn from_ffi(v: ffi::NifVersion) -> Self {
        Self::new(v.file, v.user, v.stream)
    }
}

/// Options for [`NifFile::load`]. `Default` disables all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LoadOptions {
    /// Load as a terrain file, which changes how texture paths are cleaned up and shapes are
    /// named. See [`NifFile::is_terrain`].
    pub is_terrain: bool,
}

/// Options for [`NifFile::save`]. `Default` enables both.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SaveOptions {
    /// Update bounds and delete unreferenced blocks first (see [`NifFile::optimize`]).
    pub optimize: bool,
    /// Sort all blocks into a logical order first. This changes block indices.
    pub sort_blocks: bool,
}

impl Default for SaveOptions {
    fn default() -> Self {
        Self {
            optimize: true,
            sort_blocks: true,
        }
    }
}

/// Options for [`NifFile::optimize_for`]. `Default` enables all except `head_parts`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OptimizeOptions {
    /// Use the mesh formats head parts need (`BSDynamicTriShape` for SE). Only for head parts.
    pub head_parts: bool,
    /// Remove parallax shader types, flags and texture paths.
    pub remove_parallax: bool,
    /// Recalculate bounding spheres for skinned meshes. Unskinned meshes keep their bounds.
    pub calc_bounds: bool,
    /// Fix the BSXFlags value based on the file's contents.
    pub fix_bsx_flags: bool,
    /// Fix shader flags based on the file's contents.
    pub fix_shader_flags: bool,
}

impl Default for OptimizeOptions {
    fn default() -> Self {
        Self {
            head_parts: false,
            remove_parallax: true,
            calc_bounds: true,
            fix_bsx_flags: true,
            fix_shader_flags: true,
        }
    }
}

/// What [`NifFile::optimize_for`] changed. The lists hold shape names.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct OptimizeReport {
    /// Shapes with duplicate names were renamed.
    pub duplicates_renamed: bool,
    /// Shapes whose vertex colors were all white and were removed, with the shader's vertex
    /// color and alpha flags. Only with [`OptimizeOptions::remove_parallax`], and not for
    /// shaders with the tree animation flag.
    pub vertex_colors_removed: Vec<BString>,
    /// Shapes whose normals were removed because their shader uses model space normals.
    pub normals_removed: Vec<BString>,
    /// Shapes whose skin partitions were converted from strips to triangles.
    pub partitions_triangulated: Vec<BString>,
    /// Shapes that gained tangents.
    pub tangents_added: Vec<BString>,
    /// Shapes whose parallax shader type, flag and texture were removed
    /// ([`OptimizeOptions::remove_parallax`]).
    pub parallax_removed: Vec<BString>,
}

fn names_from_ffi(names: &cxx::CxxVector<cxx::CxxString>) -> Vec<BString> {
    names.iter().map(|n| BString::from(n.as_bytes())).collect()
}

/// A block's position in the file's block list, as returned by [`NifFile::block_index`] and
/// accepted by [`NifFile::block`].
///
/// Methods that change the file take blocks by index, since they can't also take a reference
/// borrowed from the file. Block iterators give each block's index through
/// [`indexed`](crate::iter::BlockIter::indexed) and [`indices`](crate::iter::BlockIter::indices).
/// Indices change when blocks are added, deleted or sorted (for example by [`NifFile::save`] or
/// [`NifFile::optimize_for`]).
pub type Index = usize;

/// A loaded or new NIF file: a header and a list of blocks, each an [`NiObject`] subclass.
///
/// The file owns its blocks. They're borrowed from it as `&T` (from [`block_of`](Self::block_of),
/// [`blocks_of`](Self::blocks_of), ...) or `&mut T` (from [`block_of_mut`](Self::block_of_mut))
/// for their fields, and as a [`View`] or [`Edit`] for what involves other blocks. Methods that
/// change the file take blocks by [`Index`].
///
/// ```
/// # use nifly::{file::NifFile, nodes::NiNode, shaders::NiMaterialProperty};
/// # use std::{fs::File, io::Cursor};
/// let mut nif = NifFile::load(
///     File::open("nifly-sys/vendor/nifly/tests/input/TestNifFile_Static_MW.nif")?,
///     None,
/// )?;
/// for node in nif.blocks_of::<NiNode>() {
///     println!("{}", node.name());
/// }
/// let index = nif.blocks_of::<NiMaterialProperty>().indices().next().unwrap();
/// nif.block_of_mut::<NiMaterialProperty>(index).unwrap().set_alpha(0.5);
/// nif.save(Cursor::new(Vec::new()), None)?;
/// # Ok::<(), nifly::error::Error>(())
/// ```
///
/// # Methods by topic
///
/// The main methods of each section below:
///
/// - **General:** [`create`](Self::create), [`load`](Self::load), [`save`](Self::save),
///   [`version`](Self::version), [`block`](Self::block), [`block_of`](Self::block_of),
///   [`block_mut`](Self::block_mut), [`block_of_mut`](Self::block_of_mut),
///   [`view`](Self::view), [`view_of`](Self::view_of), [`edit`](Self::edit),
///   [`block_index`](Self::block_index), [`blocks`](Self::blocks),
///   [`blocks_of`](Self::blocks_of), [`find_block_by_name`](Self::find_block_by_name),
///   [`child_blocks`](Self::child_blocks), [`tree`](Self::tree), [`subtree`](Self::subtree),
///   [`delete_unreferenced_blocks`](Self::delete_unreferenced_blocks),
///   [`sort_blocks`](Self::sort_blocks), [`optimize_for`](Self::optimize_for).
/// - **Nodes:** [`nodes`](Self::nodes), [`root_node`](Self::root_node),
///   [`add_node`](Self::add_node), [`add_node_of`](Self::add_node_of), [`add_camera`](Self::add_camera),
///   [`delete_node`](Self::delete_node),
///   [`node_transform_to_global`](Self::node_transform_to_global).
/// - **Shapes:** [`shapes`](Self::shapes), [`shape_names`](Self::shape_names),
///   [`set_shape_order`](Self::set_shape_order), [`create_shape`](Self::create_shape),
///   [`clone_shape_from`](Self::clone_shape_from).
/// - **Shaders, materials and textures:** [`add_source_texture`](Self::add_source_texture).
/// - **Collision:** [`add_action`](Self::add_action).
/// - **Animation:** [`add_animation_data`](Self::add_animation_data),
///   [`add_interpolator`](Self::add_interpolator), [`add_sequence`](Self::add_sequence).
///
/// Methods returning a block have a `_mut` form taking a block [`Index`]. It returns
/// `&mut T`, for the block's setters, or an [`Edit`] where the shared form returns a
/// [`View`].
///
/// Methods for one block, such as a shape's vertices, skin and shader, a node's children and
/// parent, or a tree node's bones, are on [`View`] and [`Edit`], from [`view`](Self::view) and
/// [`edit`](Self::edit), or [`BlockIter::views`](crate::iter::BlockIter::views) while
/// iterating.
pub struct NifFile {
    pub(crate) inner: UniquePtr<ffi::NifFile>,
}

/// A deep copy: every block is copied, at the same index. Blocks borrowed from one file don't
/// belong to the other.
impl Clone for NifFile {
    fn clone(&self) -> Self {
        Self {
            inner: ffi::clone_nif(&self.inner),
        }
    }
}

/// # General
///
/// Loading, saving, optimizing and sorting, and finding and walking blocks.
impl NifFile {
    /// Creates a new file for `version`, containing only a root `NiNode` named "Scene Root".
    ///
    /// ```
    /// # use nifly::file::{NifFile, NifVersion};
    /// let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    /// let root = nif.root_node().unwrap().index();
    /// # let transform = nif.root_node().unwrap().transform();
    /// nif.add_node("Child", transform, Some(root)).unwrap();
    /// ```
    pub fn create(version: NifVersion) -> Self {
        Self {
            inner: ffi::create_nif(version.to_ffi()),
        }
    }

    /// The file's format version.
    pub fn version(&self) -> NifVersion {
        NifVersion::from_ffi(ffi::nif_version(&self.inner))
    }

    /// Reads a file from `reader`. `None` uses [`LoadOptions::default`].
    ///
    /// Reader errors are returned as [`Error::Io`] with the reader's original [`io::Error`].
    /// Invalid file contents are returned as [`Error::LoadFile`].
    pub fn load(mut reader: impl io::Read, options: Option<LoadOptions>) -> NifResult<Self> {
        let options = options.unwrap_or_default();
        let options = ffi::LoadOptions {
            is_terrain: options.is_terrain,
        };

        let mut reader = RustReader::new(&mut reader);
        let result = ffi::load_nif(&mut reader, options);

        // Prefer the reader's own error over the C++ exception it was turned into.
        if let Some(e) = reader.take_error() {
            return Err(Error::Io(e));
        }

        let result = result?;
        match result.code {
            0 => Ok(Self {
                inner: result.value,
            }),
            code => Err(LoadError::try_from(code)
                .expect("unknown error code")
                .into()),
        }
    }

    /// Writes the file to `writer`. `None` uses [`SaveOptions::default`].
    ///
    /// This always finalizes the data first (see [`finalize_data`](Self::finalize_data)). With
    /// the default options it also optimizes the file and sorts the blocks into a logical order,
    /// so block indices, and with them the results of [`block`](Self::block) and
    /// [`bone_ids`](View::bone_ids), can change. Disable
    /// [`SaveOptions::sort_blocks`] to keep the block order.
    ///
    /// Writer errors are returned as [`Error::Io`] with the writer's original [`io::Error`].
    pub fn save(
        &mut self,
        mut writer: impl io::Write + io::Seek,
        options: Option<SaveOptions>,
    ) -> NifResult<()> {
        let options = options.unwrap_or_default();
        let options = ffi::SaveOptions {
            optimize: options.optimize,
            sort_blocks: options.sort_blocks,
        };

        let mut writer = RustWriter::new(&mut writer);
        let result = ffi::save_nif(self.inner.pin_mut(), &mut writer, options);

        // Prefer the writer's own error over the C++ exception it was turned into.
        if let Some(e) = writer.take_error() {
            return Err(Error::Io(e));
        }

        match result? {
            0 => Ok(()),
            code => Err(SaveError::try_from(code)
                .expect("unknown error code")
                .into()),
        }
    }

    /// Sets the [external emittance flag](crate::extra_data::BsxBitflags::EXTERNAL_EMITTANCE) of
    /// the "BSX" [`BsxFlags`] block if any
    /// [`BsShaderProperty`] in the file has external emittance, and clears it otherwise. Only the
    /// first "BSX" block is changed, and only that bit. Run by
    /// [`optimize_for`](Self::optimize_for) by default.
    ///
    /// [`BsxFlags`]: crate::extra_data::BsxFlags
    /// [`BsShaderProperty`]: crate::shaders::BsShaderProperty
    pub fn fix_bsx_flags(&mut self) {
        ffi::NifFile::FixBSXFlags(self.inner.pin_mut());
    }

    /// Makes two flags of every [`BsLightingShaderProperty`] in the file match its
    /// [shader type](crate::shaders::BsLightingShaderProperty::shader_type):
    /// [environment mapping](crate::shaders::SkyrimShaderFlags1::ENVIRONMENT_MAPPING) is set
    /// only for environment map shaders (type 1), and
    /// [eye environment mapping](crate::shaders::SkyrimShaderFlags1::EYE_ENVIRONMENT_MAPPING) only
    /// for eye shaders (type 16). Run by [`optimize_for`](Self::optimize_for) by default.
    ///
    /// [`BsLightingShaderProperty`]: crate::shaders::BsLightingShaderProperty
    pub fn fix_shader_flags(&mut self) {
        ffi::NifFile::FixShaderFlags(self.inner.pin_mut());
    }

    /// Deletes every block of type `T` that no block references, and returns how many were
    /// deleted. Use [`NiObject`] as `T` for any type. Block indices change.
    ///
    /// Blocks left unreferenced by a deletion are deleted too if they're a `T`. Pointers don't
    /// count as references (see [`block_ref_count`](Self::block_ref_count)). The
    /// [root node](Self::root_node) is kept. Does nothing if the file has
    /// [unknown blocks](Self::has_unknown) or no root node.
    ///
    /// ```no_run
    /// # use nifly::{file::NifFile, objects::NiObject, shaders::NiShader};
    /// # fn f(nif: &mut NifFile) {
    /// let shaders = nif.delete_unreferenced_blocks::<NiShader>();
    /// let all = nif.delete_unreferenced_blocks::<NiObject>();
    /// # }
    /// ```
    pub fn delete_unreferenced_blocks<T: NiType>(&mut self) -> usize {
        fn matches<T: NiType>(obj: &objects_ffi::NiObject) -> bool {
            T::downcast_from(NiObject::from_ffi(obj)).is_some()
        }
        ffi::delete_unreferenced_blocks(self.inner.pin_mut(), matches::<T>) as usize
    }

    /// Updates every shape's bounds and deletes unreferenced blocks. Block indices change. Run
    /// by [`save`](Self::save) when [`SaveOptions::optimize`] is set.
    pub fn optimize(&mut self) {
        ffi::NifFile::Optimize(self.inner.pin_mut());
    }

    /// Converts the file between Skyrim LE ([`NifVersion::SKYRIM`]) and Skyrim SE
    /// ([`NifVersion::SKYRIM_SE`]). `None` uses [`OptimizeOptions::default`].
    ///
    /// Returns `None` and changes nothing unless the conversion is LE to SE or SE to LE.
    ///
    /// Shapes are replaced with the target's shape type (`NiTriShape` ⇄ `BSTriShape`),
    /// unreferenced blocks are deleted and blocks are re-sorted, so block indices change.
    pub fn optimize_for(
        &mut self,
        target: NifVersion,
        options: Option<OptimizeOptions>,
    ) -> Option<OptimizeReport> {
        let o = options.unwrap_or_default();
        let options = ffi::OptimizeOptions {
            target: target.to_ffi(),
            head_parts: o.head_parts,
            remove_parallax: o.remove_parallax,
            calc_bounds: o.calc_bounds,
            fix_bsx_flags: o.fix_bsx_flags,
            fix_shader_flags: o.fix_shader_flags,
        };
        let r = ffi::optimize_for(self.inner.pin_mut(), options);
        if r.version_mismatch {
            return None;
        }
        Some(OptimizeReport {
            duplicates_renamed: r.dupes_renamed,
            vertex_colors_removed: names_from_ffi(&r.vertex_colors_removed),
            normals_removed: names_from_ffi(&r.normals_removed),
            partitions_triangulated: names_from_ffi(&r.partitions_triangulated),
            tangents_added: names_from_ffi(&r.tangents_added),
            parallax_removed: names_from_ffi(&r.parallax_removed),
        })
    }

    /// Fills in what loading leaves out: resolves string refs, links geometry data to its
    /// shapes, cleans up texture paths and removes invalid triangles. Skinned `BSTriShape`s get
    /// their mesh data from their skin partitions. Run by [`load`](Self::load).
    pub fn prepare_data(&mut self) {
        ffi::NifFile::PrepareData(self.inner.pin_mut());
    }

    /// Updates the sizes and counts saving writes. Skinned `BSTriShape`s copy their mesh data
    /// back to their skin partitions. Run by [`save`](Self::save).
    pub fn finalize_data(&mut self) {
        ffi::NifFile::FinalizeData(self.inner.pin_mut());
    }

    /// Whether the file was loaded or created, and not [cleared](Self::clear) since.
    pub fn is_valid(&self) -> bool {
        ffi::NifFile::IsValid(&self.inner)
    }

    /// Whether the file has blocks of unknown types. They're kept and saved as they were read,
    /// but their refs aren't known, so [`delete_unreferenced_blocks`](Self::delete_unreferenced_blocks)
    /// and block sorting do nothing.
    pub fn has_unknown(&self) -> bool {
        ffi::NifFile::HasUnknown(&self.inner)
    }

    /// Whether the file was loaded with [`LoadOptions::is_terrain`].
    pub fn is_terrain(&self) -> bool {
        ffi::NifFile::IsTerrain(&self.inner)
    }

    /// Deletes every block and header string, leaving an empty file that isn't
    /// [valid](Self::is_valid).
    pub fn clear(&mut self) {
        ffi::NifFile::Clear(self.inner.pin_mut())
    }

    /// How many refs to the block at `index` the file's blocks hold, including blocks outside the
    /// tree, or 0 if there's no such block. A block listed twice counts twice.
    ///
    /// Refs are owning links: children, a shape's data and shader, extra data, collision. With
    /// `include_ptrs`, back-references such as skin bones, the skeleton root and collision
    /// targets count too.
    pub fn block_ref_count(&self, index: Index, include_ptrs: bool) -> usize {
        block_index_to_ffi(index).map_or(0, |i| {
            ffi::block_ref_count(&self.inner, i, include_ptrs) as usize
        })
    }

    /// Whether `obj` is one of this file's blocks. O(number of blocks).
    pub fn contains(&self, obj: &NiObject) -> bool {
        ffi::contains_block(&self.inner, obj.as_ffi())
    }

    /// The block index of `obj`, or `None` if it isn't one of this file's blocks. The inverse of
    /// [`block`](Self::block). O(number of blocks).
    ///
    /// Useful for methods that take `&mut self`, such as [`add_node`](Self::add_node), which
    /// can't also take a reference borrowed from the file.
    pub fn block_index(&self, obj: &NiObject) -> Option<Index> {
        match ffi::block_index(&self.inner, obj.as_ffi()) {
            u32::MAX => None,
            index => Some(index as Index),
        }
    }

    /// Sorts all blocks into a logical order, based on child references, block types and version.
    /// Block indices change. Run by [`save`](Self::save) when [`SaveOptions::sort_blocks`] is set.
    /// Does nothing if the file has [unknown blocks](Self::has_unknown).
    pub fn pretty_sort_blocks(&mut self) {
        ffi::NifFile::PrettySortBlocks(self.inner.pin_mut());
    }

    /// Starts a custom block sort, or returns `None` if the file has
    /// [unknown blocks](Self::has_unknown).
    ///
    /// [`BlockSorter::visit`] sorts one subtree at a time, in the order you visit them, and
    /// [`BlockSorter::apply`] puts the rest after them and reorders the file.
    /// [`pretty_sort_blocks`](Self::pretty_sort_blocks) is the same as visiting every node
    /// without a parent, in block order.
    ///
    /// ```no_run
    /// # use nifly::file::NifFile;
    /// # fn f(nif: &mut NifFile, weapon: usize, root: usize) {
    /// let mut sorter = nif.sort_blocks().unwrap();
    /// sorter.visit(weapon); // the weapon's subtree comes first
    /// sorter.visit(root);
    /// sorter.apply();
    /// # }
    /// ```
    pub fn sort_blocks(&mut self) -> Option<BlockSorter<'_>> {
        if self.has_unknown() {
            return None;
        }
        let state = ffi::new_sort_state(&self.inner);
        Some(BlockSorter { file: self, state })
    }

    /// Replaces every block the block at `block` refers to (its [child blocks](Self::child_blocks),
    /// recursively) with a deep copy, so it no longer shares them with other blocks. The
    /// originals stay; [`optimize`](Self::optimize) deletes those nothing refers to any more.
    ///
    /// New blocks are added at the end, so existing block indices don't change. Returns `false`
    /// if there's no block at `block`.
    pub fn clone_children(&mut self, block: Index) -> bool {
        block_index_to_ffi(block)
            .is_some_and(|block| ffi::clone_children(self.inner.pin_mut(), block))
    }

    /// Returns the block at `index`, or `None` if `index` is out of range.
    pub fn block(&self, index: Index) -> Option<&NiObject> {
        let ptr = ffi::block(&self.inner, index);
        // SAFETY: as in `root_node`, the block is owned by this file.
        unsafe { ptr.as_ref() }.map(NiObject::from_ffi)
    }

    /// The block at `index` as a `T`, or `None` if there's no such block or it isn't a `T` or a
    /// subclass of it.
    pub fn block_of<T: NiType>(&self, index: Index) -> Option<&T> {
        self.block(index)?.downcast::<T>()
    }

    /// Like [`block`](Self::block), but mutable. Use [`NiObject::downcast_mut`] or
    /// [`block_of_mut`](Self::block_of_mut) to reach a type's setters.
    pub fn block_mut(&mut self, index: Index) -> Option<&mut NiObject> {
        self.block_of_mut::<NiObject>(index)
    }

    /// The block at `index` as a mutable `T`, or `None` if there's no such block or it isn't a
    /// `T`. A block's `&mut` also reaches its base types' setters.
    ///
    /// ```
    /// # use nifly::{file::NifFile, shaders::NiMaterialProperty};
    /// # let mut nif = NifFile::load(std::fs::File::open(
    /// #     "nifly-sys/vendor/nifly/tests/input/TestNifFile_Static_MW.nif").unwrap(), None).unwrap();
    /// let (index, _) = nif.blocks_of::<NiMaterialProperty>().indexed().next().unwrap();
    /// let material = nif.block_of_mut::<NiMaterialProperty>(index).unwrap();
    /// material.set_alpha(0.5);
    /// material.set_emissive([1.0, 0.5, 0.0]);
    /// assert_eq!(material.alpha(), 0.5);
    /// ```
    pub fn block_of_mut<T: NiType>(&mut self, index: Index) -> Option<&mut T> {
        // SAFETY: the block came from this file.
        unsafe { self.find_mut(|f| f.block_of::<T>(index)) }
    }

    /// The block at `index` as a [`View`] of type `T`, or `None` if there's no such block or it
    /// isn't a `T`.
    pub fn view<T: NiType>(&self, index: Index) -> Option<View<'_, T>> {
        View::new(self, index)
    }

    /// `block` as a [`View`], or `None` if it's from another file. Finds its index with
    /// [`block_index`](Self::block_index), an O(blocks) search; prefer
    /// [`BlockIter::views`](crate::iter::BlockIter::views) when iterating.
    pub fn view_of<T: NiType>(&self, block: &T) -> Option<View<'_, T>> {
        self.view(self.block_index(block.as_object())?)
    }

    /// The block at `index` as an [`Edit`] of type `T`, or `None` if there's no such block or it
    /// isn't a `T`.
    pub fn edit<T: NiType>(&mut self, index: Index) -> Option<Edit<'_, T>> {
        Edit::new(self, index)
    }

    /// Borrows the object `find` finds in this file for mutation, borrowing the file mutably for as
    /// long as the result lives. `None` if `find` finds nothing.
    ///
    /// `find` only locates the object. The mutable reference is built from the pointer the
    /// `object_mut` shim returns for it through the file borrowed mutably, so no `&mut` is made
    /// from a shared reference.
    ///
    /// # Safety
    ///
    /// `find` must return a block owned by this file (or a base-class view of one), or an object
    /// inside one, such as a [`BsGeometry`](crate::geometry::BsGeometry)'s mesh data.
    pub(crate) unsafe fn find_mut<T: NiType>(
        &mut self,
        find: impl FnOnce(&Self) -> Option<&T>,
    ) -> Option<&mut T> {
        let found: *const T = find(self)?;
        // SAFETY: `found` is valid until the file is next borrowed mutably, below.
        let obj: *const objects_ffi::NiObject = unsafe { &*found }.as_object().as_ffi();
        // SAFETY: `obj` is this file's, as the caller guarantees.
        let obj_mut = unsafe { ffi::object_mut(self.inner.pin_mut(), obj) };
        // Where `T` sits in the object (the same address under single inheritance), applied to
        // the mutable pointer so the result keeps its provenance.
        let offset = (found as *const u8 as isize).wrapping_sub(obj as *const u8 as isize);
        let found = obj_mut.cast::<u8>().wrapping_offset(offset).cast::<T>();
        // SAFETY: `found` is the object `find` returned, reached through `&mut self`, which stays
        // borrowed for as long as the result lives, so no other reference to it exists. The
        // object is only accessed through C++, and never moves (see `objects::assert_zero_sized`).
        Some(unsafe { &mut *found })
    }

    /// Iterates over every block in the file, in block order.
    pub fn blocks(&self) -> BlockIter<'_> {
        BlockIter::new(self, 0..ffi::block_count(&self.inner))
    }

    /// Iterates over every block that is a `T` or a subclass of `T`, in block order.
    ///
    /// ```no_run
    /// # use nifly::{file::NifFile, nodes::BsFadeNode};
    /// # let nif: NifFile = todo!();
    /// for fade in nif.blocks_of::<BsFadeNode>() {
    ///     println!("{}", fade.name());
    /// }
    /// ```
    pub fn blocks_of<T: NiType>(&self) -> BlockIter<'_, T> {
        self.blocks().of::<T>()
    }

    /// The first block in block order that is a `T` and is named `name`. Use [`NiObject`] as `T`
    /// to search every named block.
    ///
    /// Only blocks with a name are searched: nodes, shapes and other [`NiObjectNet`]s, and
    /// extra data. Use [`block_index`](Self::block_index) to get the result's index.
    ///
    /// ```no_run
    /// # use nifly::{file::NifFile, nodes::NiNode};
    /// # fn f(nif: &NifFile) {
    /// let bone = nif.find_block_by_name::<NiNode>("NPC Head [Head]");
    /// # }
    /// ```
    pub fn find_block_by_name<T: NiType>(&self, name: impl AsRef<[u8]>) -> Option<&T> {
        let name = name.as_ref();
        self.blocks()
            .filter(|b| b.name_if_named() == Some(name))
            .find_map(NiObject::downcast::<T>)
    }

    /// Like [`find_block_by_name`](Self::find_block_by_name), but mutable.
    pub fn find_block_by_name_mut<T: NiType>(&mut self, name: impl AsRef<[u8]>) -> Option<&mut T> {
        // SAFETY: the block came from this file.
        unsafe { self.find_mut(|f| f.find_block_by_name::<T>(name)) }
    }

    /// Iterates over every block `obj` references directly: children, properties,
    /// controllers, extra data, shaders, and so on. Skips empty refs.
    ///
    /// Panics in debug builds if `obj` is from another file; in release builds the result is
    /// meaningless.
    pub fn child_blocks(&self, obj: &NiObject) -> BlockIter<'_, NiObject, RefIndices> {
        debug_assert!(self.contains(obj), "object does not belong to this NifFile");
        BlockIter::new(self, self.child_ref_indices(obj))
    }

    /// Every block `obj` references directly that is a `T` or a subclass of `T`, in field order.
    /// Only refs are followed, not pointers: a skin instance's bones and a collision object's
    /// target aren't child blocks. Empty refs are skipped; a block referenced twice appears twice.
    ///
    /// Panics in debug builds if `obj` is from another file; in release builds the result is
    /// meaningless.
    pub fn child_blocks_of<T: NiType>(&self, obj: &NiObject) -> BlockIter<'_, T, RefIndices> {
        self.child_blocks(obj).of::<T>()
    }

    /// The indices of the blocks `obj` references, without the foreign-object check.
    pub(crate) fn child_ref_indices(&self, obj: &NiObject) -> RefIndices {
        RefIndices::new(objects_ffi::child_indices(obj.as_ffi()))
    }

    /// Walks every block reachable from the root node, depth-first. Each block appears once, the
    /// first time it's reached. Empty if the file has no root node.
    pub fn tree(&self) -> BlockIter<'_, NiObject, TreeIndices<'_>> {
        let root = self.root_node().map(|node| node.index());
        BlockIter::new(self, TreeIndices::new(self, root))
    }

    /// Like [`tree`](Self::tree), but starts at `obj` instead of the root.
    ///
    /// Panics in debug builds if `obj` is from another file; in release builds the result is
    /// meaningless.
    pub fn subtree(&self, obj: &NiObject) -> BlockIter<'_, NiObject, TreeIndices<'_>> {
        debug_assert!(self.contains(obj), "object does not belong to this NifFile");
        BlockIter::new(self, TreeIndices::new(self, self.block_index(obj)))
    }
}

/// Extra data of named objects.
impl<'a, T: NiType> View<'a, T> {
    /// The extra data blocks attached to the object. Skips empty refs. Only reads the extra data
    /// list from file version 10.0.1.0 and later; the linked extra data chain in older files
    /// isn't followed.
    pub fn extra_data<I>(&self) -> BlockIter<'a, NiObject, RefIndices>
    where
        T: IsA<NiObjectNet, I>,
    {
        let obj: &NiObjectNet = self.get().upcast_ref();
        BlockIter::new(
            self.file(),
            RefIndices::new(ffi::extra_data_indices(obj.as_ffi())),
        )
    }

    /// The extra data chain of files up to version 4.2.2.0: the first block, then each block's
    /// [next](View::next_extra_data). Stops at an empty ref or a block already listed.
    pub fn extra_data_chain<I>(&self) -> BlockIter<'a, NiExtraData, RefIndices>
    where
        T: IsA<NiObjectNet, I>,
    {
        self.follow_list(RefList::ExtraDataChain)
    }
}

impl<T: NiType> Edit<'_, T> {
    /// Creates an extra data block from `value` and attaches it to the object. The value's type
    /// picks the block type and carries its name, if it has one (see [`IntoExtraData`]). Returns
    /// the new block's index. The block is appended, so existing block indices stay valid. A
    /// block is always added, even if the object already has one of the same kind.
    ///
    /// Returns `None`, changing nothing, for unnamed Fallout 4 blocks
    /// ([`ExtraBsCloth`](crate::extra_data::ExtraBsCloth),
    /// [`ExtraBsCollisionQueryProxy`](crate::extra_data::ExtraBsCollisionQueryProxy)) in files
    /// of version 4.2.2.0 or older, which store extra data as a linked list they can't join.
    ///
    /// ```no_run
    /// # use nifly::{file::NifFile, extra_data::*, nodes::NiNode};
    /// # let mut nif: NifFile = todo!();
    /// # let cloth_hkx: Vec<u8> = todo!();
    /// let root = nif.root_node().unwrap().index();
    /// let mut root = nif.edit::<NiNode>(root).unwrap();
    /// root.assign_extra_data(ExtraString("Prn", "SHIELD")).unwrap();
    /// root.assign_extra_data(ExtraFloat("Scale", 1.5)).unwrap();
    /// root.assign_extra_data(ExtraBsxFlags(BsxBitflags::HAVOK | BsxBitflags::ARTICULATED)).unwrap();
    /// root.assign_extra_data(ExtraBsCloth(&cloth_hkx)).unwrap();
    /// ```
    pub fn assign_extra_data<I>(&mut self, value: impl IntoExtraData) -> Option<Index>
    where
        T: IsA<NiObjectNet, I>,
    {
        let target = self.index() as u32;
        index_from_ffi(value.assign(self.file_mut().inner.pin_mut(), target))
    }

    /// Replaces the object's extra data list, saved in files from version 10.0.1.0 on. Returns
    /// `false`, changing nothing, unless each block is extra data: an [`NiExtraData`] or a
    /// Fallout 4 block such as [`BsClothExtraData`](crate::extra_data::BsClothExtraData).
    pub fn set_extra_data<I>(&mut self, extra_data: &[Index]) -> bool
    where
        T: IsA<NiObjectNet, I>,
    {
        self.set_ref_list(RefList::ExtraData, extra_data)
    }

    /// Replaces the extra data chain, saved in files up to version 4.2.2.0, linking each block to
    /// the next. Returns `false`, changing nothing, unless the blocks are distinct
    /// [`NiExtraData`] blocks.
    pub fn set_extra_data_chain<I>(&mut self, extra_data: &[Index]) -> bool
    where
        T: IsA<NiObjectNet, I>,
    {
        self.set_ref_list(RefList::ExtraDataChain, extra_data)
    }
}

/// A block sort in progress, from [`NifFile::sort_blocks`].
///
/// Visiting only computes the new order; [`apply`](Self::apply) moves the blocks. Dropping the
/// sorter without applying it leaves the block order as it was, but visiting a node already
/// reorders its child refs: nodes, then shapes, then other blocks, then empty refs (in
/// Oblivion and Fallout 3 files, only nodes with children come before shapes).
/// `BSOrderedNode` children keep their order.
pub struct BlockSorter<'a> {
    file: &'a mut NifFile,
    state: UniquePtr<ffi::SortState>,
}

impl BlockSorter<'_> {
    /// Gives the block at `index` the next new index, then the blocks it references in this
    /// order: extra data, controllers, properties and collision, then a node's children or a
    /// shape's data, skin, shader and alpha property. Collision blocks are placed
    /// dependencies first. Blocks already visited keep their place.
    ///
    /// Returns `false`, doing nothing, if there's no block at `index` or it was already visited.
    /// Indices are the file's indices from before the sort.
    pub fn visit(&mut self, index: Index) -> bool {
        let Some(index) = block_index_to_ffi(index) else {
            return false;
        };
        ffi::sort_visit(self.file.inner.pin_mut(), self.state.pin_mut(), index)
    }

    /// Whether the block at `index` has been placed by a [`visit`](Self::visit), directly or
    /// as part of a visited subtree.
    pub fn is_visited(&self, index: Index) -> bool {
        block_index_to_ffi(index).is_some_and(|i| ffi::sort_is_visited(&self.state, i))
    }

    /// Puts the blocks no visit reached after the visited ones, keeping their relative order,
    /// and reorders the file. Block indices change.
    pub fn apply(mut self) {
        ffi::sort_apply(self.file.inner.pin_mut(), self.state.pin_mut());
    }
}

/// A block index as nifly's `u32`, or `None` if it can't be one. `u32::MAX` is nifly's "no block"
/// (`NIF_NPOS`), so it's rejected too.
pub(crate) fn block_index_to_ffi(index: Index) -> Option<u32> {
    u32::try_from(index).ok().filter(|&i| i != u32::MAX)
}

/// `None` as `NIF_NPOS`; `None` for an index that doesn't fit.
pub(crate) fn optional_index_to_ffi(index: Option<Index>) -> Option<u32> {
    match index {
        Some(i) => block_index_to_ffi(i),
        None => Some(u32::MAX),
    }
}

/// A block index returned by nifly, where `u32::MAX` (`NIF_NPOS`) means "none".
pub(crate) fn index_from_ffi(index: u32) -> Option<Index> {
    (index != u32::MAX).then_some(index as Index)
}
