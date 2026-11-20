//! Iterators over blocks.
//!
//! Every block iterator is a [`BlockIter`]: it walks block indices and yields the blocks of one
//! type. [`indexed`](BlockIter::indexed) and [`indices`](BlockIter::indices) give each block's
//! [`Index`] without a [`block_index`](NifFile::block_index) lookup.

use std::{collections::HashSet, iter::FusedIterator, marker::PhantomData, ops::Range, vec};

use crate::{
    file::{Index, NifFile},
    handle::View,
    objects::{NiObject, NiType},
};

/// Blocks of type `T`, found by walking a source of block indices `I`.
///
/// The iterator borrows the file. To pass the indices to a method that changes the file,
/// collect them first:
///
/// ```no_run
/// # use nifly::{file::NifFile, geometry::NiShape};
/// # fn f(nif: &mut NifFile) {
/// let old: Vec<_> = nif.shapes().indexed().filter(|(_, s)| s.name() == "Old").map(|(i, _)| i).collect();
/// for i in old {
///     nif.edit::<NiShape>(i).unwrap().calc_normals(None);
/// }
/// # }
/// ```
pub struct BlockIter<'a, T: NiType = NiObject, I = Range<usize>> {
    file: &'a NifFile,
    indices: I,
    _type: PhantomData<fn() -> &'a T>,
}

impl<'a, T: NiType, I> BlockIter<'a, T, I> {
    pub(crate) fn new(file: &'a NifFile, indices: I) -> Self {
        Self {
            file,
            indices,
            _type: PhantomData,
        }
    }

    /// `(index, block)` pairs instead of blocks.
    pub fn indexed(self) -> Indexed<'a, T, I> {
        Indexed(self)
    }

    /// The indices of the blocks this would yield.
    pub fn indices(self) -> Indices<'a, T, I> {
        Indices(self.indexed())
    }

    /// [`View`]s of the blocks, which pair each with the file and its index.
    pub fn views(self) -> Views<'a, T, I> {
        Views(self.indexed())
    }

    /// The same blocks, keeping only those that are a `U` or a subclass of `U`.
    pub fn of<U: NiType>(self) -> BlockIter<'a, U, I> {
        BlockIter::new(self.file, self.indices)
    }

    fn resolve(file: &'a NifFile, i: Index) -> Option<(Index, &'a T)> {
        Some((i, file.block_of::<T>(i)?))
    }
}

impl<T: NiType, I: Clone> Clone for BlockIter<'_, T, I> {
    fn clone(&self) -> Self {
        Self::new(self.file, self.indices.clone())
    }
}

impl<'a, T: NiType, I: Iterator<Item = Index>> Iterator for BlockIter<'a, T, I> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let file = self.file;
        self.indices
            .find_map(|i| Self::resolve(file, i))
            .map(|(_, b)| b)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.indices.size_hint().1)
    }
}

impl<'a, T: NiType, I: DoubleEndedIterator<Item = Index>> DoubleEndedIterator
    for BlockIter<'a, T, I>
{
    fn next_back(&mut self) -> Option<&'a T> {
        let file = self.file;
        self.indices
            .by_ref()
            .rev()
            .find_map(|i| Self::resolve(file, i))
            .map(|(_, b)| b)
    }
}

impl<T: NiType, I: FusedIterator<Item = Index>> FusedIterator for BlockIter<'_, T, I> {}

/// `(index, block)` pairs from [`BlockIter::indexed`].
#[derive(Clone)]
pub struct Indexed<'a, T: NiType, I>(BlockIter<'a, T, I>);

impl<'a, T: NiType, I: Iterator<Item = Index>> Iterator for Indexed<'a, T, I> {
    type Item = (Index, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let file = self.0.file;
        self.0
            .indices
            .find_map(|i| BlockIter::<T, I>::resolve(file, i))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a, T: NiType, I: DoubleEndedIterator<Item = Index>> DoubleEndedIterator
    for Indexed<'a, T, I>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let file = self.0.file;
        self.0
            .indices
            .by_ref()
            .rev()
            .find_map(|i| BlockIter::<T, I>::resolve(file, i))
    }
}

impl<T: NiType, I: FusedIterator<Item = Index>> FusedIterator for Indexed<'_, T, I> {}

/// Block indices from [`BlockIter::indices`].
#[derive(Clone)]
pub struct Indices<'a, T: NiType, I>(Indexed<'a, T, I>);

impl<T: NiType, I: Iterator<Item = Index>> Iterator for Indices<'_, T, I> {
    type Item = Index;

    fn next(&mut self) -> Option<Index> {
        self.0.next().map(|(i, _)| i)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<T: NiType, I: DoubleEndedIterator<Item = Index>> DoubleEndedIterator for Indices<'_, T, I> {
    fn next_back(&mut self) -> Option<Index> {
        self.0.next_back().map(|(i, _)| i)
    }
}

impl<T: NiType, I: FusedIterator<Item = Index>> FusedIterator for Indices<'_, T, I> {}

/// [`View`]s from [`BlockIter::views`].
#[derive(Clone)]
pub struct Views<'a, T: NiType, I>(Indexed<'a, T, I>);

impl<'a, T: NiType, I: Iterator<Item = Index>> Iterator for Views<'a, T, I> {
    type Item = View<'a, T>;

    fn next(&mut self) -> Option<View<'a, T>> {
        let file = self.0.0.file;
        self.0.next().map(|(i, b)| View::from_parts(file, i, b))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl<'a, T: NiType, I: DoubleEndedIterator<Item = Index>> DoubleEndedIterator for Views<'a, T, I> {
    fn next_back(&mut self) -> Option<View<'a, T>> {
        let file = self.0.0.file;
        self.0
            .next_back()
            .map(|(i, b)| View::from_parts(file, i, b))
    }
}

impl<T: NiType, I: FusedIterator<Item = Index>> FusedIterator for Views<'_, T, I> {}

/// The block indices of a list of refs, such as a node's children. Empty refs are skipped.
#[derive(Clone)]
pub struct RefIndices(vec::IntoIter<u32>);

impl RefIndices {
    pub(crate) fn new(refs: Vec<u32>) -> Self {
        Self(refs.into_iter())
    }
}

impl Iterator for RefIndices {
    type Item = Index;

    fn next(&mut self) -> Option<Index> {
        self.0.find(|&i| i != u32::MAX).map(|i| i as Index)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.0.size_hint().1)
    }
}

impl DoubleEndedIterator for RefIndices {
    fn next_back(&mut self) -> Option<Index> {
        self.0.rfind(|&i| i != u32::MAX).map(|i| i as Index)
    }
}

impl FusedIterator for RefIndices {}

/// The block indices reachable from a start block, depth-first. Each block appears once, the
/// first time it's reached.
#[derive(Clone)]
pub struct TreeIndices<'a> {
    file: &'a NifFile,
    stack: Vec<Index>,
    visited: HashSet<Index>,
}

impl<'a> TreeIndices<'a> {
    pub(crate) fn new(file: &'a NifFile, start: Option<Index>) -> Self {
        Self {
            file,
            stack: start.into_iter().collect(),
            visited: HashSet::new(),
        }
    }
}

impl Iterator for TreeIndices<'_> {
    type Item = Index;

    // Marking blocks as visited when they're popped, not when they're pushed, visits each child
    // just before descending into it, as a recursive walk would.
    fn next(&mut self) -> Option<Index> {
        while let Some(i) = self.stack.pop() {
            let Some(block) = self.file.block(i) else {
                continue;
            };
            if !self.visited.insert(i) {
                continue;
            }
            self.stack.extend(self.file.child_ref_indices(block).rev());
            return Some(i);
        }
        None
    }
}

impl FusedIterator for TreeIndices<'_> {}
