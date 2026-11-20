//! Handles pairing a block with its file.
//!
//! A block borrowed from a [`NifFile`] can't reach the file, so it can't follow its refs to other
//! blocks or check a block it's given. A [`View`] or [`Edit`] holds the file with the block's
//! index, and has the methods that need both: for example, `View<BsTreeNode>` lists a tree's
//! bones, and `Edit<BsTreeNode>` replaces them.
//!
//! Each block type's docs end with a *Handle methods* section listing the methods its handles
//! have, including those for its base types. Methods for every block, such as
//! [`index`](View::index), [`cast`](View::cast) and [`upcast`](View::upcast), aren't listed.
//!
//! ```
//! # use nifly::{file::NifFile, nodes::NiNode};
//! # let nif = NifFile::load(std::fs::File::open("assets/Cube.nif").unwrap(), None).unwrap();
//! let root = nif.view::<NiNode>(0).unwrap();
//! println!("{} at index {}", root.name(), root.index());
//! ```

use std::{
    fmt,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use nifly_sys::nif_file_refs::ffi::{self as refs_ffi, RefKind, RefList};

use crate::{
    file::{Index, NifFile, block_index_to_ffi, index_from_ffi, optional_index_to_ffi},
    iter::{BlockIter, RefIndices},
    objects::{IsA, NiType},
};

/// A block of type `T` with the file it belongs to, from [`NifFile::view`] or
/// [`BlockIter::views`](crate::iter::BlockIter::views). Derefs to the block.
pub struct View<'a, T: NiType> {
    file: &'a NifFile,
    index: Index,
    block: &'a T,
}

impl<T: NiType> Clone for View<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: NiType> Copy for View<'_, T> {}

impl<'a, T: NiType> View<'a, T> {
    pub(crate) fn new(file: &'a NifFile, index: Index) -> Option<Self> {
        Some(Self {
            file,
            index,
            block: file.block_of::<T>(index)?,
        })
    }

    pub(crate) fn from_parts(file: &'a NifFile, index: Index, block: &'a T) -> Self {
        Self { file, index, block }
    }

    /// The block's index in the file.
    pub fn index(&self) -> Index {
        self.index
    }

    pub fn file(&self) -> &'a NifFile {
        self.file
    }

    /// The block, borrowed for as long as the file.
    pub fn get(&self) -> &'a T {
        self.block
    }

    /// The same block as a `U`, or `None` if it isn't one. Works for base and derived types.
    pub fn cast<U: NiType>(&self) -> Option<View<'a, U>> {
        View::new(self.file, self.index)
    }

    /// The same block as its base type `B`.
    pub fn upcast<B: NiType, I>(&self) -> View<'a, B>
    where
        T: IsA<B, I>,
    {
        View {
            file: self.file,
            index: self.index,
            block: self.block.upcast_ref(),
        }
    }

    /// The block at `index` in the same file, if it's a `U`.
    pub(crate) fn resolve<U: NiType>(&self, index: Option<Index>) -> Option<View<'a, U>> {
        View::new(self.file, index?)
    }

    /// The block in this block's `kind` ref, if it's a `U`.
    pub(crate) fn follow<U: NiType>(&self, kind: RefKind) -> Option<View<'a, U>> {
        self.resolve(index_from_ffi(refs_ffi::block_ref(
            &self.file.inner,
            self.index as u32,
            kind,
        )))
    }

    /// The blocks in this block's `list` refs that are a `U`.
    pub(crate) fn follow_list<U: NiType>(&self, list: RefList) -> BlockIter<'a, U, RefIndices> {
        BlockIter::new(
            self.file,
            RefIndices::new(refs_ffi::block_ref_list(
                &self.file.inner,
                self.index as u32,
                list,
            )),
        )
    }
}

impl<T: NiType> Deref for View<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.block
    }
}

impl<T: NiType> fmt::Debug for View<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("View")
            .field("index", &self.index)
            .field("block", &self.block.as_object().block_name())
            .finish()
    }
}

/// A block of type `T` in a file borrowed mutably, from [`NifFile::edit`]. Derefs mutably to the
/// block, for its setters. Methods that change other blocks, or check blocks they're given, go
/// here; the [`View`] methods are on [`view`](Self::view).
pub struct Edit<'a, T: NiType> {
    file: &'a mut NifFile,
    index: Index,
    _type: PhantomData<fn() -> T>,
}

impl<'a, T: NiType> Edit<'a, T> {
    pub(crate) fn new(file: &'a mut NifFile, index: Index) -> Option<Self> {
        file.block_of::<T>(index)?;
        Some(Self {
            file,
            index,
            _type: PhantomData,
        })
    }

    /// The block's index in the file.
    pub fn index(&self) -> Index {
        self.index
    }

    /// The block, for reading.
    pub fn view(&self) -> View<'_, T> {
        View::new(self.file, self.index).expect("the block was checked when the edit was made")
    }

    /// The same block as a `U`, or the edit back if it isn't one. Works for base and derived
    /// types.
    pub fn into_cast<U: NiType>(self) -> Result<Edit<'a, U>, Self> {
        if self.file.block_of::<U>(self.index).is_some() {
            Ok(Edit {
                file: self.file,
                index: self.index,
                _type: PhantomData,
            })
        } else {
            Err(self)
        }
    }

    /// The same block as its base type `B`.
    pub fn into_upcast<B: NiType, I>(self) -> Edit<'a, B>
    where
        T: IsA<B, I>,
    {
        Edit {
            file: self.file,
            index: self.index,
            _type: PhantomData,
        }
    }

    pub(crate) fn file_mut(&mut self) -> &mut NifFile {
        self.file
    }

    /// The block at `index` in the same file, if it's a `U`.
    pub(crate) fn resolve<U: NiType>(&mut self, index: Option<Index>) -> Option<Edit<'_, U>> {
        Edit::new(self.file, index?)
    }

    /// The block in this block's `kind` ref, if it's a `U`, for editing.
    pub(crate) fn follow<U: NiType>(&mut self, kind: RefKind) -> Option<Edit<'_, U>> {
        let index = index_from_ffi(refs_ffi::block_ref(
            &self.file.inner,
            self.index as u32,
            kind,
        ));
        self.resolve(index)
    }

    /// Points this block's `kind` ref at `target`, or empties it. `false` if the shim rejects it.
    pub(crate) fn set_ref(&mut self, kind: RefKind, target: Option<Index>) -> bool {
        let Some(target) = optional_index_to_ffi(target) else {
            return false;
        };
        refs_ffi::set_block_ref(self.file.inner.pin_mut(), self.index as u32, kind, target)
    }

    /// Replaces this block's `list` refs. `false` if the shim rejects them.
    pub(crate) fn set_ref_list(&mut self, list: RefList, targets: &[Index]) -> bool {
        let Some(targets) = targets
            .iter()
            .map(|&i| block_index_to_ffi(i))
            .collect::<Option<Vec<_>>>()
        else {
            return false;
        };
        refs_ffi::set_block_ref_list(self.file.inner.pin_mut(), self.index as u32, list, &targets)
    }
}

impl<T: NiType> Deref for Edit<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.file
            .block_of::<T>(self.index)
            .expect("the block was checked when the edit was made")
    }
}

impl<T: NiType> DerefMut for Edit<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.file
            .block_of_mut::<T>(self.index)
            .expect("the block was checked when the edit was made")
    }
}

impl<T: NiType> fmt::Debug for Edit<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Edit")
            .field("index", &self.index)
            .field("block", &self.as_object().block_name())
            .finish()
    }
}
