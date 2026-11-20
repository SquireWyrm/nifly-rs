//! Tests for the controller chain: controller fields and flags, chains, targets, interpolators
//! and old key data refs.

use nifly::{
    animation::{
        AnimType, CycleType, NiAlphaController, NiFloatData, NiFloatInterpolator, NiGeomMorpherController,
        NiInterpController, NiKeyframeController, NiKeyframeData, NiMaterialColorController, NiMorphData,
        NiPosData, NiSingleInterpController, NiTimeController, NiTransformController, NiUvController, NiUvData, NiVisController,
        TimeControllerFlags,
    },
    file::{NifFile, NifVersion},
    objects::NiObjectNet,
};

use crate::common::{reload_in_place, upstream};

/// Oblivion's 10.1.0.106 files, which save the fields older than 10.1.0.110.
const OLD: NifVersion = NifVersion::new(0x0A01006A, 10, 5);

fn chain(nif: &NifFile, owner: usize) -> Vec<usize> {
    nif.view::<NiObjectNet>(owner).unwrap().controllers().indices().collect()
}

#[test]
fn upstream_chains() {
    let nif = upstream("TestNifFile_Morph_MW");
    let remap = nif.view::<NiObjectNet>(13).unwrap();
    assert_eq!(remap.controller().unwrap().index(), 14);
    assert_eq!(chain(&nif, 13), [14, 15]);
    let color = nif.view::<NiMaterialColorController>(14).unwrap();
    assert_eq!(color.next_controller().unwrap().index(), 15);
    assert_eq!(color.controller_target().unwrap().index(), 13);
    assert!(color.key_data().is_some_and(|d| d.cast::<NiPosData>().is_some()));
    let flags = color.flags();
    assert_eq!(flags.bits(), 0x3c);
    assert_eq!(flags.anim_type(), AnimType::AppTime);
    assert_eq!(flags.cycle_type(), Some(CycleType::Clamp));
    assert!(flags.active() && flags.play_backwards() && flags.manager_controlled());
    assert!(!flags.compute_scaled_time() && !flags.forced_update());
    assert_eq!((color.frequency(), color.phase(), color.start_time(), color.stop_time()), (1.0, 0.0, 0.0, 0.6));
    assert!(nif.view::<NiAlphaController>(15).unwrap().next_controller().is_none());

    assert!(nif.view::<NiKeyframeController>(6).unwrap().key_data().is_some());
    assert!(nif.view::<NiGeomMorpherController>(10).unwrap().key_data().is_some_and(|d| d.cast::<NiMorphData>().is_some()));
    assert!(nif.view::<NiUvController>(42).unwrap().key_data().is_some_and(|d| d.cast::<NiUvData>().is_some()));

    let nif = upstream("TestNifFile_Animated_LE");
    assert_eq!(chain(&nif, 0), [3, 4]);
    let shader_controller = nif.view::<NiTimeController>(11).unwrap();
    assert_eq!(shader_controller.controller_target().unwrap().index(), 64);
    let shader_controller = shader_controller.cast::<NiSingleInterpController>().unwrap();
    assert!(shader_controller.interpolator().is_some());
}

#[test]
fn add_controllers() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let mut root = nif.root_node_mut().unwrap();
    let alpha = root.add_controller::<NiAlphaController, _>();
    let vis = root.add_controller::<NiVisController, _>();
    assert_eq!(chain(&nif, 0), [alpha, vis]);
    let c = nif.view::<NiTimeController>(vis).unwrap();
    assert_eq!(c.controller_target().unwrap().index(), 0);
    assert_eq!(c.flags(), TimeControllerFlags::DEFAULT);
    assert_eq!((c.frequency(), c.phase(), c.start_time(), c.stop_time()), (1.0, 0.0, f32::MAX, f32::MIN));

    let flags = TimeControllerFlags::DEFAULT.with_cycle_type(CycleType::Loop).with_compute_scaled_time(true);
    let base = nif.block_of_mut::<NiTimeController>(alpha).unwrap();
    base.set_flags(flags);
    base.set_frequency(2.0);
    base.set_phase(0.5);
    base.set_start_time(0.25);
    base.set_stop_time(3.0);

    let nif = reload_in_place(&mut nif);
    assert_eq!(chain(&nif, 0), [alpha, vis]);
    let c = nif.view::<NiTimeController>(alpha).unwrap();
    assert_eq!(c.flags(), flags);
    assert_eq!((c.frequency(), c.phase(), c.start_time(), c.stop_time()), (2.0, 0.5, 0.25, 3.0));
    assert_eq!(c.controller_target().unwrap().index(), 0);
}

#[test]
fn chain_links() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let mut root = nif.root_node_mut().unwrap();
    let a = root.add_controller::<NiAlphaController, _>();
    let b = root.add_controller::<NiVisController, _>();
    let c = root.add_controller::<NiTransformController, _>();
    let data = nif.add_animation_data::<NiFloatData>();

    let mut last = nif.edit::<NiTimeController>(c).unwrap();
    assert!(!last.set_next_controller(Some(a)), "a cycle");
    assert!(!last.set_next_controller(Some(c)), "a cycle to itself");
    assert!(!last.set_next_controller(Some(data)), "not a controller");
    assert!(!last.set_next_controller(Some(1000)));
    let mut first = nif.edit::<NiTimeController>(a).unwrap();
    assert!(first.set_next_controller(Some(c)));
    assert_eq!(chain(&nif, 0), [a, c]);
    assert!(nif.edit::<NiTimeController>(b).unwrap().set_next_controller(Some(a)));
    assert!(nif.edit::<NiTimeController>(a).unwrap().set_next_controller(None));
    assert_eq!(chain(&nif, 0), [a]);

    let mut root = nif.root_node_mut().unwrap();
    assert!(!root.set_controller(Some(data)));
    assert!(root.set_controller(Some(b)));
    assert_eq!(chain(&nif, 0), [b, a]);

    let mut root = nif.root_node_mut().unwrap();
    assert!(!root.set_controllers(&[c, c]), "duplicates");
    assert!(!root.set_controllers(&[c, data]), "not a controller");
    assert_eq!(chain(&nif, 0), [b, a]);
    assert!(nif.root_node_mut().unwrap().set_controllers(&[c, a, b]));
    assert_eq!(chain(&nif, 0), [c, a, b]);
    assert!(nif.view::<NiTimeController>(b).unwrap().next_controller().is_none());
    assert!(nif.root_node_mut().unwrap().set_controllers(&[]));
    assert!(nif.root_node().unwrap().controller().is_none());

    assert!(nif.edit::<NiTimeController>(c).unwrap().set_next_controller(None));
    assert!(nif.root_node_mut().unwrap().set_controllers(&[a, b]));
    assert!(nif.edit::<NiTimeController>(b).unwrap().set_next_controller(Some(c)));
    assert_eq!(chain(&nif, 0), [a, b, c]);
    let d = nif.root_node_mut().unwrap().add_controller::<NiAlphaController, _>();
    assert_eq!(chain(&nif, 0), [a, b, c, d]);
}

#[test]
fn targets() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let child = nif.root_node_mut().unwrap().add_child("Child", Default::default());
    let c = nif.root_node_mut().unwrap().add_controller::<NiAlphaController, _>();
    let data = nif.add_animation_data::<NiFloatData>();
    let mut e = nif.edit::<NiAlphaController>(c).unwrap();
    assert!(!e.set_controller_target(Some(data)));
    assert!(e.set_controller_target(Some(child)));
    assert_eq!(e.controller_target().unwrap().index(), child);
    let nif2 = reload_in_place(&mut nif);
    assert_eq!(nif2.view::<NiTimeController>(c).unwrap().controller_target().unwrap().index(), child);
    assert!(nif.edit::<NiAlphaController>(c).unwrap().set_controller_target(None));
    assert!(nif.view::<NiTimeController>(c).unwrap().controller_target().is_none());
}

#[test]
fn interpolators() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let c = nif.root_node_mut().unwrap().add_controller::<NiAlphaController, _>();
    let interpolator = nif.add_interpolator::<NiFloatInterpolator>();
    let data = nif.add_animation_data::<NiFloatData>();
    let mut e = nif.edit::<NiAlphaController>(c).unwrap();
    assert!(!e.set_interpolator(Some(data)));
    assert!(e.set_interpolator(Some(interpolator)));
    assert_eq!(e.interpolator().unwrap().index(), interpolator);

    let nif = reload_in_place(&mut nif);
    assert_eq!(nif.view::<NiAlphaController>(c).unwrap().interpolator().unwrap().index(), interpolator);
}

#[test]
fn old_key_data() {
    let mut nif = NifFile::create(NifVersion::MORROWIND);
    let mut root = nif.root_node_mut().unwrap();
    let keyframes = root.add_controller::<NiTransformController, _>();
    let alpha = root.add_controller::<NiAlphaController, _>();
    let keyframe_data = nif.add_animation_data::<NiKeyframeData>();
    let float_data = nif.add_animation_data::<NiFloatData>();
    let interpolator = nif.add_interpolator::<NiFloatInterpolator>();
    let mut e = nif.edit::<NiTransformController>(keyframes).unwrap();
    assert!(!e.set_key_data(Some(float_data)));
    assert!(e.set_key_data(Some(keyframe_data)));
    let mut e = nif.edit::<NiAlphaController>(alpha).unwrap();
    assert!(e.set_key_data(Some(float_data)));
    assert!(e.set_interpolator(Some(interpolator)));

    let nif = reload_in_place(&mut nif);
    assert_eq!(nif.view::<NiTransformController>(keyframes).unwrap().key_data().unwrap().index(), keyframe_data);
    let alpha = nif.view::<NiAlphaController>(alpha).unwrap();
    assert_eq!(alpha.key_data().unwrap().index(), float_data);
    assert!(alpha.interpolator().is_none(), "interpolators aren't saved before 10.1.0.104");
}

#[test]
fn manager_controlled() {
    for (version, saved) in [(OLD, true), (NifVersion::SKYRIM_SE, false)] {
        let mut nif = NifFile::create(version);
        let c = nif.root_node_mut().unwrap().add_controller::<NiVisController, _>();
        nif.block_of_mut::<NiInterpController>(c).unwrap().set_manager_controlled(true);
        let nif = reload_in_place(&mut nif);
        assert_eq!(nif.block_of::<NiInterpController>(c).unwrap().manager_controlled(), saved, "{version:?}");
    }
}

#[test]
fn flags() {
    let flags = TimeControllerFlags::from_bits(0xff06);
    assert_eq!(flags.cycle_type(), None);
    let flags = flags.with_cycle_type(CycleType::Reverse).with_active(true).with_anim_type(AnimType::AppInit);
    assert_eq!(flags.bits(), 0xff0b);
    assert_eq!(flags.cycle_type(), Some(CycleType::Reverse));
    assert!(!flags.with_forced_update(false).forced_update());
    assert_eq!(TimeControllerFlags::default(), TimeControllerFlags::DEFAULT);

    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let c = nif.root_node_mut().unwrap().add_controller::<NiVisController, _>();
    nif.block_of_mut::<NiTimeController>(c).unwrap().set_flags(flags);
    let nif = reload_in_place(&mut nif);
    assert_eq!(nif.block_of::<NiTimeController>(c).unwrap().flags(), flags);
}
