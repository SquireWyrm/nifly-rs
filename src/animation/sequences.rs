//! Sequences, controller managers and animation notes.

use super::*;
use crate::{extra_data::NiTextKeyExtraData, nodes::NiDefaultAvObjectPalette};

impl NiSequence {
    /// The name the sequence is played by.
    pub fn name(&self) -> &BStr {
        ffi::sequence_name(&self.0).as_bstr()
    }

    /// Sets [`name`](Self::name).
    pub fn set_name(&mut self, name: impl AsRef<[u8]>) {
        ffi::set_sequence_name(self.ffi_mut(), name.as_ref());
    }

    /// The name of the object that accumulates the animation's motion. A plain `NiSequence`
    /// saves it only before 10.1.0.104, an [`NiControllerSequence`] before 10.1.0.104 and from
    /// 10.1.0.106 on.
    pub fn accum_root_name(&self) -> &BStr {
        ffi::sequence_accum_root_name(&self.0).as_bstr()
    }

    /// Sets [`accum_root_name`](Self::accum_root_name).
    pub fn set_accum_root_name(&mut self, name: impl AsRef<[u8]>) {
        ffi::set_sequence_accum_root_name(self.ffi_mut(), name.as_ref());
    }

    fields! {
        /// Saved from 10.1.0.106 on.
        array_grow_by, set_array_grow_by: u32 = sequence_array_grow_by, set_sequence_array_grow_by, same, same;
    }
}

impl NiControllerSequence {
    enum_field! {
        /// How the sequence repeats, or `None` for a value it doesn't list. Saved from 10.1.0.106
        /// on, like the other fields here.
        cycle_type, set_cycle_type: CycleType = sequence_cycle_type, set_sequence_cycle_type
    }

    fields! {
        /// How the sequence blends with others of the same priority.
        weight, set_weight: f32 = sequence_weight, set_sequence_weight, same, same;
        /// The playback speed.
        frequency, set_frequency: f32 = sequence_frequency, set_sequence_frequency, same, same;
        /// Saved only up to 10.4.0.1.
        phase, set_phase: f32 = sequence_phase, set_sequence_phase, same, same;
        start_time, set_start_time: f32 = sequence_start_time, set_sequence_start_time, same, same;
        stop_time, set_stop_time: f32 = sequence_stop_time, set_sequence_stop_time, same, same;
        /// Saved only in 10.1.0.106.
        play_backwards, set_play_backwards: bool = sequence_play_backwards, set_sequence_play_backwards, same, same;
    }
}

impl NiControllerManager {
    fields! {
        /// Whether motion on the accumulation root adds up over cycles instead of being absolute.
        cumulative, set_cumulative: bool = manager_cumulative, set_manager_cumulative, same, same;
    }
}

/// The kind of a [`BsAnimNote`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, IntEnum)]
#[repr(u32)]
pub enum AnimNoteType {
    Invalid = 0,
    /// Grab IK, which saves an [arm](BsAnimNote::arm).
    GrabIk = 1,
    /// Look IK, which saves a [gain](BsAnimNote::gain) and [state](BsAnimNote::state).
    LookIk = 2,
}

impl BsAnimNote {
    enum_field! {
        /// The kind of note, or `None` for a value it doesn't list. It decides which of the other
        /// fields are saved.
        note_type, set_note_type: AnimNoteType = anim_note_type, set_anim_note_type
    }

    fields! {
        /// When the note happens.
        time, set_time: f32 = anim_note_time, set_anim_note_time, same, same;
        /// Saved only for [`GrabIk`](AnimNoteType::GrabIk) notes.
        arm, set_arm: u32 = anim_note_arm, set_anim_note_arm, same, same;
        /// Saved only for [`LookIk`](AnimNoteType::LookIk) notes.
        gain, set_gain: f32 = anim_note_gain, set_anim_note_gain, same, same;
        /// Saved only for [`LookIk`](AnimNoteType::LookIk) notes.
        state, set_state: u32 = anim_note_state, set_anim_note_state, same, same;
    }
}

/// One object a sequence animates, from [`View::controlled_blocks`].
///
/// The strings are saved by version: before 10.1.0.104 only `node_name`; from 10.2.0.0 to
/// 20.1.0.0 in the sequence's [string palette](View::string_palette); otherwise as strings.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ControlledBlock {
    /// Saved from 10.1.0.106 on.
    pub interpolator: Option<Index>,
    pub controller: Option<Index>,
    /// Saved only from 10.1.0.104 to 10.1.0.110.
    pub blend_interpolator: Option<Index>,
    /// Saved only from 10.1.0.104 to 10.1.0.110.
    pub blend_index: u16,
    /// Saved from 10.1.0.106 on, in Bethesda files.
    pub priority: u8,
    /// The name of the animated object.
    pub node_name: BString,
    /// The type of the animated property, if a property is animated. Saved from 10.1.0.104 on,
    /// like the other IDs.
    pub property_type: BString,
    pub controller_type: BString,
    pub controller_id: BString,
    pub interpolator_id: BString,
}

/// A sequence block [`NifFile::add_sequence`] can add.
pub trait NewSequence: NiType + Sealed {
    #[doc(hidden)]
    fn add(file: Pin<&mut nifly_sys::nif_file::ffi::NifFile>, name: &[u8]) -> u32;
}

macro_rules! new_sequence {
    ($($t:ident => $add:ident;)*) => {$(
        impl NewSequence for $t {
            fn add(file: Pin<&mut nifly_sys::nif_file::ffi::NifFile>, name: &[u8]) -> u32 {
                ffi::$add(file, name)
            }
        }
    )*};
}

new_sequence! {
    NiSequence => add_ni_sequence;
    NiControllerSequence => add_ni_controller_sequence;
    NiSequenceStreamHelper => add_ni_sequence_stream_helper;
}

/// Sequence and manager refs.
impl<'a, T: NiType> View<'a, T> {
    /// The sequence's text keys, or `None` if it has none. A plain `NiSequence` saves them only
    /// before 10.1.0.104, an [`NiControllerSequence`] before 10.1.0.104 and from 10.1.0.106 on.
    pub fn text_key_data<I>(&self) -> Option<View<'a, NiTextKeyExtraData>>
    where
        T: IsA<NiSequence, I>,
    {
        self.follow(RefKind::SequenceTextKeys)
    }

    /// The objects the sequence animates.
    pub fn controlled_blocks<I>(&self) -> Vec<ControlledBlock>
    where
        T: IsA<NiSequence, I>,
    {
        let text = |b: Vec<u8>| BString::from(b);
        ffi::controlled_blocks(&self.file().inner, self.index() as u32)
            .into_iter()
            .map(|b| ControlledBlock {
                interpolator: index_from_ffi(b.interpolator),
                controller: index_from_ffi(b.controller),
                blend_interpolator: index_from_ffi(b.blend_interpolator),
                blend_index: b.blend_index,
                priority: b.priority,
                node_name: text(b.node_name),
                property_type: text(b.property_type),
                controller_type: text(b.controller_type),
                controller_id: text(b.controller_id),
                interpolator_id: text(b.interpolator_id),
            })
            .collect()
    }

    /// The manager that plays the sequence, or `None` if it has none. Saved from 10.1.0.106 on.
    pub fn sequence_manager<I>(&self) -> Option<View<'a, NiControllerManager>>
    where
        T: IsA<NiControllerSequence, I>,
    {
        self.follow(RefKind::SequenceManager)
    }

    /// The palette holding the sequence's [controlled block](View::controlled_blocks) strings,
    /// or `None` if it has none. Saved from 10.1.0.113 to 20.1.0.0.
    pub fn string_palette<I>(&self) -> Option<View<'a, NiStringPalette>>
    where
        T: IsA<NiControllerSequence, I>,
    {
        self.follow(RefKind::SequencePalette)
    }

    /// The sequence's notes, or `None` if it has none. Saved only in Bethesda files with stream
    /// version 24 to 28; later ones save [`anim_note_arrays`](View::anim_note_arrays).
    pub fn anim_notes<I>(&self) -> Option<View<'a, BsAnimNotes>>
    where
        T: IsA<NiControllerSequence, I>,
    {
        self.follow(RefKind::AnimNotes)
    }

    /// The sequence's note lists. Saved only in Bethesda files with stream version above 28.
    /// Skips empty refs.
    pub fn anim_note_arrays<I>(&self) -> BlockIter<'a, BsAnimNotes, RefIndices>
    where
        T: IsA<NiControllerSequence, I>,
    {
        self.follow_list(RefList::AnimNoteArrays)
    }

    /// The notes in the list. Skips empty refs.
    pub fn notes<I>(&self) -> BlockIter<'a, BsAnimNote, RefIndices>
    where
        T: IsA<BsAnimNotes, I>,
    {
        self.follow_list(RefList::AnimNoteList)
    }

    /// The manager's sequences. Skips empty refs.
    pub fn sequences<I>(&self) -> BlockIter<'a, NiControllerSequence, RefIndices>
    where
        T: IsA<NiControllerManager, I>,
    {
        self.follow_list(RefList::Sequences)
    }

    /// The manager's object palette, or `None` if it has none.
    pub fn object_palette<I>(&self) -> Option<View<'a, NiDefaultAvObjectPalette>>
    where
        T: IsA<NiControllerManager, I>,
    {
        self.follow(RefKind::ObjectPalette)
    }
}

impl<T: NiType> Edit<'_, T> {
    /// Like [`View::text_key_data`], for editing.
    pub fn text_key_data<I>(&mut self) -> Option<Edit<'_, NiTextKeyExtraData>>
    where
        T: IsA<NiSequence, I>,
    {
        self.follow(RefKind::SequenceTextKeys)
    }

    /// Sets the [text keys](View::text_key_data), or clears them with `None`. Returns `false`,
    /// changing nothing, unless `keys` is an [`NiTextKeyExtraData`].
    pub fn set_text_key_data<I>(&mut self, keys: Option<Index>) -> bool
    where
        T: IsA<NiSequence, I>,
    {
        self.set_ref(RefKind::SequenceTextKeys, keys)
    }

    /// Replaces the [controlled blocks](View::controlled_blocks). Returns `false`, changing
    /// nothing, unless each interpolator is empty or an [`NiInterpolator`], each controller
    /// empty or an [`NiTimeController`], and each blend interpolator empty or an
    /// [`NiBlendInterpolator`].
    ///
    /// From 10.2.0.0 to 20.1.0.0, new strings are added to the sequence's
    /// [string palette](View::string_palette), or the one its blocks use; a palette is added if
    /// there's none. Strings already in it are reused, and other offsets stay valid. Empty
    /// strings are saved as no string.
    pub fn set_controlled_blocks<I>(&mut self, blocks: &[ControlledBlock]) -> bool
    where
        T: IsA<NiSequence, I>,
    {
        let Some(blocks) = blocks
            .iter()
            .map(|b| {
                Some(ffi::ControlledBlockFfi {
                    interpolator: optional_index_to_ffi(b.interpolator)?,
                    controller: optional_index_to_ffi(b.controller)?,
                    blend_interpolator: optional_index_to_ffi(b.blend_interpolator)?,
                    blend_index: b.blend_index,
                    priority: b.priority,
                    node_name: b.node_name.to_vec(),
                    property_type: b.property_type.to_vec(),
                    controller_type: b.controller_type.to_vec(),
                    controller_id: b.controller_id.to_vec(),
                    interpolator_id: b.interpolator_id.to_vec(),
                })
            })
            .collect::<Option<Vec<_>>>()
        else {
            return false;
        };
        let owner = self.index() as u32;
        ffi::set_controlled_blocks(self.file_mut().inner.pin_mut(), owner, &blocks)
    }

    /// Like [`View::sequence_manager`], for editing.
    pub fn sequence_manager<I>(&mut self) -> Option<Edit<'_, NiControllerManager>>
    where
        T: IsA<NiControllerSequence, I>,
    {
        self.follow(RefKind::SequenceManager)
    }

    /// Sets the [sequence manager](View::sequence_manager), or clears it with `None`. Returns
    /// `false`, changing nothing, unless `manager` is an [`NiControllerManager`].
    pub fn set_sequence_manager<I>(&mut self, manager: Option<Index>) -> bool
    where
        T: IsA<NiControllerSequence, I>,
    {
        self.set_ref(RefKind::SequenceManager, manager)
    }

    /// Like [`View::string_palette`], for editing.
    pub fn string_palette<I>(&mut self) -> Option<Edit<'_, NiStringPalette>>
    where
        T: IsA<NiControllerSequence, I>,
    {
        self.follow(RefKind::SequencePalette)
    }

    /// Sets the [string palette](View::string_palette), or clears it with `None`. The
    /// controlled blocks keep theirs. Returns `false`, changing nothing, unless `palette` is an
    /// [`NiStringPalette`].
    pub fn set_string_palette<I>(&mut self, palette: Option<Index>) -> bool
    where
        T: IsA<NiControllerSequence, I>,
    {
        self.set_ref(RefKind::SequencePalette, palette)
    }

    /// Like [`View::anim_notes`], for editing.
    pub fn anim_notes<I>(&mut self) -> Option<Edit<'_, BsAnimNotes>>
    where
        T: IsA<NiControllerSequence, I>,
    {
        self.follow(RefKind::AnimNotes)
    }

    /// Sets the [notes](View::anim_notes), or clears them with `None`. Returns `false`, changing
    /// nothing, unless `notes` is a [`BsAnimNotes`].
    pub fn set_anim_notes<I>(&mut self, notes: Option<Index>) -> bool
    where
        T: IsA<NiControllerSequence, I>,
    {
        self.set_ref(RefKind::AnimNotes, notes)
    }

    /// Replaces the [note lists](View::anim_note_arrays). Returns `false`, changing nothing,
    /// unless each is a [`BsAnimNotes`] and there are at most 65535.
    pub fn set_anim_note_arrays<I>(&mut self, lists: &[Index]) -> bool
    where
        T: IsA<NiControllerSequence, I>,
    {
        self.set_ref_list(RefList::AnimNoteArrays, lists)
    }

    /// Replaces the [notes](View::notes). Returns `false`, changing nothing, unless each is a
    /// [`BsAnimNote`] and there are at most 65535.
    pub fn set_notes<I>(&mut self, notes: &[Index]) -> bool
    where
        T: IsA<BsAnimNotes, I>,
    {
        self.set_ref_list(RefList::AnimNoteList, notes)
    }

    /// Replaces the manager's [sequences](View::sequences). Their
    /// [managers](View::sequence_manager) are unchanged. Returns `false`, changing nothing,
    /// unless each is an [`NiControllerSequence`].
    pub fn set_sequences<I>(&mut self, sequences: &[Index]) -> bool
    where
        T: IsA<NiControllerManager, I>,
    {
        self.set_ref_list(RefList::Sequences, sequences)
    }

    /// Adds a new [`NiControllerSequence`] named `name` to the end of the manager's
    /// [sequences](View::sequences), with the manager as its
    /// [sequence manager](View::sequence_manager), and returns its index. It has weight and
    /// frequency 1 and loops.
    pub fn add_sequence<I>(&mut self, name: impl AsRef<[u8]>) -> Index
    where
        T: IsA<NiControllerManager, I>,
    {
        let owner = self.index() as u32;
        let added = ffi::add_manager_sequence(self.file_mut().inner.pin_mut(), owner, name.as_ref());
        index_from_ffi(added).expect("the owner is an NiControllerManager")
    }

    /// Like [`View::object_palette`], for editing.
    pub fn object_palette<I>(&mut self) -> Option<Edit<'_, NiDefaultAvObjectPalette>>
    where
        T: IsA<NiControllerManager, I>,
    {
        self.follow(RefKind::ObjectPalette)
    }

    /// Sets the [object palette](View::object_palette), or clears it with `None`. Returns
    /// `false`, changing nothing, unless `palette` is an [`NiDefaultAvObjectPalette`].
    pub fn set_object_palette<I>(&mut self, palette: Option<Index>) -> bool
    where
        T: IsA<NiControllerManager, I>,
    {
        self.set_ref(RefKind::ObjectPalette, palette)
    }
}
