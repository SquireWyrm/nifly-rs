//! Tests for sequences, controller managers and animation notes.

use nifly::{
    animation::{
        AnimNoteType, BsAnimNote, BsAnimNotes, ControlledBlock, CycleType, NiControllerManager, NiControllerSequence,
        NiFloatInterpolator, NiSequence, NiSequenceStreamHelper, NiStringPalette, NiTransformController,
    },
    extra_data::{ExtraTextKeys, NiTextKeyExtraData, TextKey},
    file::{NifFile, NifVersion},
    nodes::NiNode,
    objects::NiObjectNet,
};

use crate::common::{reload_in_place, upstream};

/// The versions with different controlled block layouts: target names only (10.0.1.0), strings
/// (10.1.0.106), palette offsets (10.2.0.0 and 20.0.0.5) and string table refs (20.2.0.7).
const SPECIAL: NifVersion = NifVersion::new(0x0A000100, 0, 0);
const OB_OLD: NifVersion = NifVersion::new(0x0A01006A, 10, 5);
const OB_PALETTE: NifVersion = NifVersion::new(0x0A020000, 10, 9);

fn block(node: &str, interpolator: Option<usize>, controller: Option<usize>) -> ControlledBlock {
    ControlledBlock {
        interpolator,
        controller,
        priority: 30,
        node_name: node.into(),
        property_type: "".into(),
        controller_type: "NiTransformController".into(),
        controller_id: "".into(),
        interpolator_id: "".into(),
        ..Default::default()
    }
}

#[test]
fn upstream_manager() {
    let nif = upstream("TestNifFile_Animated_LE");
    let manager = nif.view::<NiControllerManager>(3).unwrap();
    assert!(!manager.cumulative());
    assert_eq!(manager.sequences().indices().collect::<Vec<_>>(), [5, 25, 36]);
    assert_eq!(manager.object_palette().unwrap().index(), 47);
    let seq = nif.view::<NiControllerSequence>(5).unwrap();
    assert_eq!(seq.name(), "mBegin");
    assert_eq!(seq.accum_root_name(), "Mesh01");
    assert_eq!(seq.cycle_type(), Some(CycleType::Clamp));
    assert_eq!((seq.weight(), seq.frequency(), seq.start_time()), (1.0, 1.0, 0.0));
    assert_eq!(seq.sequence_manager().unwrap().index(), 3);
    assert_eq!(seq.text_key_data().unwrap().index(), 6);
    assert!(seq.string_palette().is_none() && seq.anim_notes().is_none());
    assert_eq!(seq.anim_note_arrays().count(), 0);
    let blocks = seq.controlled_blocks();
    assert_eq!(blocks.len(), 5);
    assert_eq!(blocks[0].node_name, "Low02");
    assert_eq!((blocks[0].interpolator, blocks[0].controller), (Some(7), Some(4)));
    assert_eq!(blocks[1].property_type, "BSEffectShaderProperty");
    assert_eq!(blocks[1].controller_id, "5");
    assert_eq!(blocks[4].controller_type, "NiPSysEmitterCtlr");
    assert_eq!(blocks[4].interpolator_id, "EmitterActive");
}

#[test]
fn upstream_stream_helper() {
    let nif = NifFile::load(
        std::fs::read(format!("{}/TestNifFile_Sequence_MW.kf", crate::common::UPSTREAM)).unwrap().as_slice(),
        None,
    )
    .unwrap();
    let helper = nif.view::<NiSequenceStreamHelper>(0).unwrap();
    assert_eq!(helper.controllers().indices().collect::<Vec<_>>(), [3]);
    assert!(nif.view::<NiObjectNet>(0).is_some());
}

#[test]
fn controlled_blocks_round_trip_unchanged() {
    let mut nif = upstream("TestNifFile_Animated_LE");
    let blocks = nif.view::<NiControllerSequence>(5).unwrap().controlled_blocks();
    assert!(nif.edit::<NiControllerSequence>(5).unwrap().set_controlled_blocks(&blocks));
    let nif = reload_in_place(&mut nif);
    assert_eq!(nif.view::<NiControllerSequence>(5).unwrap().controlled_blocks(), blocks);
}

/// Builds a sequence animating `Bone` and returns the file and the sequence's index.
fn sequence_file(version: NifVersion) -> (NifFile, usize) {
    let mut nif = NifFile::create(version);
    let root = nif.root_node().unwrap().index();
    let keys = nif
        .edit::<NiNode>(root)
        .unwrap()
        .assign_extra_data(ExtraTextKeys("", [TextKey { time: 0.0, text: "start".into() }]))
        .unwrap();
    let controller = nif.edit::<NiNode>(root).unwrap().add_controller::<NiTransformController, _>();
    let interpolator = nif.add_interpolator::<NiFloatInterpolator>();
    let seq = nif.add_sequence::<NiControllerSequence>("Idle");
    let mut edit = nif.edit::<NiControllerSequence>(seq).unwrap();
    edit.block().upcast_mut().set_accum_root_name("Root");
    assert!(edit.set_text_key_data(Some(keys)));
    let mut ui = block("Bone", Some(interpolator), Some(controller));
    ui.controller_id = "ID".into();
    assert!(edit.set_controlled_blocks(&[ui, block("Other", None, Some(controller))]));
    (nif, seq)
}

#[test]
fn layouts_by_version() {
    for version in [SPECIAL, OB_OLD, OB_PALETTE, NifVersion::OBLIVION, NifVersion::SKYRIM_SE] {
        let (mut nif, seq) = sequence_file(version);
        let before = nif.view::<NiControllerSequence>(seq).unwrap().controlled_blocks();
        let nif = reload_in_place(&mut nif);
        let seq = nif.view::<NiControllerSequence>(seq).unwrap();
        assert_eq!(seq.name(), "Idle", "{version:?}");
        assert_eq!(seq.accum_root_name(), "Root", "{version:?}");
        assert!(seq.text_key_data().is_some(), "{version:?}");
        let blocks = seq.controlled_blocks();
        assert_eq!(blocks.len(), 2, "{version:?}");
        assert_eq!(blocks[0].node_name, "Bone", "{version:?}");
        assert_eq!(blocks[1].node_name, "Other", "{version:?}");
        assert_eq!(blocks[0].controller, before[0].controller, "{version:?}");
        if version == SPECIAL {
            // Only target names and controllers.
            assert_eq!(blocks[0].controller_type, "");
            assert_eq!(blocks[0].interpolator, None);
            assert_eq!(seq.frequency(), 0.0);
        } else {
            assert_eq!(blocks[0].controller_type, "NiTransformController", "{version:?}");
            assert_eq!(blocks[0].controller_id, "ID", "{version:?}");
            assert_eq!(blocks[0].interpolator, before[0].interpolator, "{version:?}");
            assert_eq!(seq.frequency(), 1.0, "{version:?}");
        }
        let priority = if version == SPECIAL { 0 } else { 30 };
        assert_eq!(blocks[0].priority, priority, "{version:?}");
        let palette = seq.string_palette();
        assert_eq!(palette.is_some(), [OB_PALETTE, NifVersion::OBLIVION].contains(&version), "{version:?}");
    }
}

#[test]
fn palette_offsets() {
    let (mut nif, seq) = sequence_file(OB_PALETTE);
    let palette = nif.view::<NiControllerSequence>(seq).unwrap().string_palette().unwrap().index();
    let bytes = nif.block_of::<NiStringPalette>(palette).unwrap().palette().to_owned();
    assert_eq!(bytes, "Bone\0NiTransformController\0ID\0Other");

    // Unchanged blocks keep their offsets, so the palette doesn't change.
    let mut blocks = nif.view::<NiControllerSequence>(seq).unwrap().controlled_blocks();
    assert!(nif.edit::<NiControllerSequence>(seq).unwrap().set_controlled_blocks(&blocks));
    assert_eq!(nif.block_of::<NiStringPalette>(palette).unwrap().palette(), bytes);

    // New strings are appended; strings already there are reused.
    blocks[1].node_name = "Third".into();
    blocks[1].interpolator_id = "Bone".into();
    assert!(nif.edit::<NiControllerSequence>(seq).unwrap().set_controlled_blocks(&blocks));
    let palette = nif.block_of::<NiStringPalette>(palette).unwrap();
    assert_eq!(palette.palette(), "Bone\0NiTransformController\0ID\0Other\0Third");
    assert_eq!(nif.view::<NiControllerSequence>(seq).unwrap().controlled_blocks(), blocks);
    let nif = reload_in_place(&mut nif);
    assert_eq!(nif.view::<NiControllerSequence>(seq).unwrap().controlled_blocks(), blocks);
}

#[test]
fn set_controlled_blocks_checks_types() {
    let (mut nif, seq) = sequence_file(NifVersion::SKYRIM_SE);
    let before = nif.view::<NiControllerSequence>(seq).unwrap().controlled_blocks();
    let controller = before[0].controller;
    let mut edit = nif.edit::<NiControllerSequence>(seq).unwrap();
    assert!(!edit.set_controlled_blocks(&[block("Bone", controller, None)]));
    assert!(!edit.set_controlled_blocks(&[block("Bone", None, Some(seq))]));
    let wrong_blend = ControlledBlock { blend_interpolator: before[0].interpolator, ..Default::default() };
    assert!(!edit.set_controlled_blocks(&[wrong_blend]));
    assert!(!edit.set_controlled_blocks(&[block("Bone", Some(usize::MAX - 1), None)]));
    assert_eq!(nif.view::<NiControllerSequence>(seq).unwrap().controlled_blocks(), before);
}

#[test]
fn sequence_refs() {
    let (mut nif, seq) = sequence_file(NifVersion::OBLIVION);
    let palette = nif.view::<NiControllerSequence>(seq).unwrap().string_palette().unwrap().index();
    let mut edit = nif.edit::<NiControllerSequence>(seq).unwrap();
    assert!(!edit.set_text_key_data(Some(palette)));
    assert!(!edit.set_string_palette(Some(seq)));
    assert!(!edit.set_sequence_manager(Some(seq)));
    assert!(edit.set_string_palette(None));
    assert!(edit.string_palette().is_none());
    assert!(edit.set_string_palette(Some(palette)));
    assert!(edit.set_text_key_data(None));
    assert!(edit.text_key_data().is_none());
}

#[test]
fn sequence_fields() {
    let mut nif = NifFile::create(OB_OLD);
    let seq = nif.add_sequence::<NiControllerSequence>("Walk");
    let mut block = nif.block_of_mut::<NiControllerSequence>(seq).unwrap();
    block.as_mut().set_weight(0.5);
    block.as_mut().set_cycle_type(CycleType::Reverse);
    block.as_mut().set_frequency(2.0);
    block.as_mut().set_phase(0.25);
    block.as_mut().set_start_time(1.0);
    block.as_mut().set_stop_time(3.0);
    block.as_mut().set_play_backwards(true);
    block.as_mut().upcast_mut().set_array_grow_by(4);
    block.upcast_mut().set_name("Run");
    let nif = reload_in_place(&mut nif);
    let seq = nif.block_of::<NiControllerSequence>(seq).unwrap();
    assert_eq!(seq.name(), "Run");
    assert_eq!(seq.cycle_type(), Some(CycleType::Reverse));
    assert_eq!((seq.weight(), seq.frequency(), seq.phase()), (0.5, 2.0, 0.25));
    assert_eq!((seq.start_time(), seq.stop_time()), (1.0, 3.0));
    assert!(seq.play_backwards());
    assert_eq!(seq.array_grow_by(), 4);
}

#[test]
fn fields_lost_by_version() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let seq = nif.add_sequence::<NiControllerSequence>("Walk");
    let mut block = nif.block_of_mut::<NiControllerSequence>(seq).unwrap();
    block.as_mut().set_phase(0.25);
    block.set_play_backwards(true);
    let nif = reload_in_place(&mut nif);
    let seq = nif.block_of::<NiControllerSequence>(seq).unwrap();
    assert_eq!(seq.phase(), 0.0);
    assert!(!seq.play_backwards());
}

#[test]
fn plain_sequence() {
    for version in [SPECIAL, NifVersion::SKYRIM_SE] {
        let (mut nif, _) = sequence_file(version);
        let keys = nif.blocks_of::<NiTextKeyExtraData>().indices().next().unwrap();
        let seq = nif.add_sequence::<NiSequence>("Old");
        let mut edit = nif.edit::<NiSequence>(seq).unwrap();
        edit.block().set_accum_root_name("Root");
        assert!(edit.set_text_key_data(Some(keys)));
        assert!(edit.set_controlled_blocks(&[block("Bone", None, None)]));
        let nif = reload_in_place(&mut nif);
        let seq = nif.view::<NiSequence>(seq).unwrap();
        assert_eq!(seq.name(), "Old");
        assert_eq!(seq.controlled_blocks()[0].node_name, "Bone");
        // Saved only before 10.1.0.104.
        let old = version == SPECIAL;
        assert_eq!(seq.accum_root_name() == "Root", old);
        assert_eq!(seq.text_key_data().is_some(), old);
    }
}

#[test]
fn manager() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let root = nif.root_node().unwrap().index();
    let manager = nif.edit::<NiNode>(root).unwrap().add_controller::<NiControllerManager, _>();
    let palette = nif.edit::<NiNode>(root).unwrap().add_controller::<NiTransformController, _>();
    let mut edit = nif.edit::<NiControllerManager>(manager).unwrap();
    edit.block().set_cumulative(true);
    let first = edit.add_sequence("First");
    let second = edit.add_sequence(b"Second");
    assert_eq!(edit.view().sequences().indices().collect::<Vec<_>>(), [first, second]);
    assert!(!edit.set_sequences(&[first, root]));
    assert!(edit.set_sequences(&[second, first]));
    assert!(!edit.set_object_palette(Some(palette)));
    assert!(edit.set_object_palette(None));

    let seq = nif.view::<NiControllerSequence>(first).unwrap();
    assert_eq!(seq.name(), "First");
    assert_eq!(seq.sequence_manager().unwrap().index(), manager);
    assert_eq!((seq.weight(), seq.frequency(), seq.cycle_type()), (1.0, 1.0, Some(CycleType::Loop)));
    let nif = reload_in_place(&mut nif);
    let manager = nif.view::<NiControllerManager>(manager).unwrap();
    assert!(manager.cumulative());
    assert_eq!(manager.controller_target().unwrap().index(), root);
    assert_eq!(manager.sequences().indices().collect::<Vec<_>>(), [second, first]);
}

#[test]
fn anim_notes() {
    // Fallout 3 saves note lists; stream 24 to 28 a single list.
    for (version, arrays) in [(NifVersion::FALLOUT_3, true), (NifVersion::new(0x14020007, 11, 24), false)] {
        let mut nif = NifFile::create(version);
        let seq = nif.add_sequence::<NiControllerSequence>("Grab");
        let grab = nif.add_animation_data::<BsAnimNote>();
        let look = nif.add_animation_data::<BsAnimNote>();
        let list = nif.add_animation_data::<BsAnimNotes>();
        let mut note = nif.block_of_mut::<BsAnimNote>(grab).unwrap();
        note.as_mut().set_note_type(AnimNoteType::GrabIk);
        note.as_mut().set_time(0.5);
        note.as_mut().set_arm(1);
        note.as_mut().set_gain(2.0);
        let mut note = nif.block_of_mut::<BsAnimNote>(look).unwrap();
        note.as_mut().set_note_type(AnimNoteType::LookIk);
        note.as_mut().set_arm(1);
        note.as_mut().set_gain(2.0);
        note.as_mut().set_state(3);
        let mut notes = nif.edit::<BsAnimNotes>(list).unwrap();
        assert!(!notes.set_notes(&[grab, list]));
        assert!(notes.set_notes(&[grab, look]));
        let mut edit = nif.edit::<NiControllerSequence>(seq).unwrap();
        assert!(!edit.set_anim_notes(Some(grab)));
        assert!(!edit.set_anim_note_arrays(&[grab]));
        assert!(edit.set_anim_notes(Some(list)));
        assert!(edit.set_anim_note_arrays(&[list]));

        let nif = reload_in_place(&mut nif);
        let seq = nif.view::<NiControllerSequence>(seq).unwrap();
        assert_eq!(seq.anim_notes().is_some(), !arrays);
        assert_eq!(seq.anim_note_arrays().count(), usize::from(arrays));
        let notes = nif.view::<BsAnimNotes>(list).unwrap();
        assert_eq!(notes.notes().indices().collect::<Vec<_>>(), [grab, look]);
        // Each type saves only its own fields.
        let note = nif.block_of::<BsAnimNote>(grab).unwrap();
        assert_eq!(note.note_type(), Some(AnimNoteType::GrabIk));
        assert_eq!((note.time(), note.arm(), note.gain()), (0.5, 1, 0.0));
        let note = nif.block_of::<BsAnimNote>(look).unwrap();
        assert_eq!((note.arm(), note.gain(), note.state()), (0, 2.0, 3));
    }
}

#[test]
fn stream_helper() {
    let mut nif = NifFile::create(NifVersion::MORROWIND);
    let helper = nif.add_sequence::<NiSequenceStreamHelper>("Kf");
    let nif = reload_in_place(&mut nif);
    assert_eq!(nif.block_of::<NiSequenceStreamHelper>(helper).unwrap().name(), "Kf");
}
