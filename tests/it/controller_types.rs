//! Tests for the fields and refs of specific controller types.

use nifly::{
    animation::{
        BhkBlendController, BsEffectShaderPropertyColorController, BsEffectShaderPropertyFloatController,
        BsFrustumFovController, BsLagBoneController, BsLightingShaderPropertyColorController,
        BsLightingShaderPropertyFloatController, BsLightingShaderPropertyUShortController,
        BsProceduralLightningController, EffectShaderControlledColor, EffectShaderControlledVariable,
        GeomMorpherFlags, LightColor, LightingShaderControlledColor, LightingShaderControlledFloat,
        LightningInterpolator, LookAtFlags, MaterialColor, MorphWeight, NiBoneLodController, NiBsBoneLodController,
        NiFloatData, NiFloatExtraDataController, NiFloatInterpolator, NiFlipController, NiGeomMorpherController,
        NiLightColorController, NiLookAtController, NiMaterialColorController, NiMultiTargetTransformController,
        NiPathController, NiPosData, NiTextureTransformController, NiTimeController, NiUvController, PathFlags,
        TransformMember,
    },
    file::{NifFile, NifVersion},
    shaders::{BsShaderProperty, MapSlot, NiSourceTexture, TextureMap},
};

use crate::common::{reload_in_place, upstream};

/// Oblivion's 10.1.0.106 files.
const OLD: NifVersion = NifVersion::new(0x0A01006A, 10, 5);

#[test]
fn path_controller() {
    let nif = upstream("TestNifFile_PathController_MW");
    let path = nif.view::<NiPathController>(91).unwrap();
    assert!(path.path_data().is_some() && path.percent_data().is_some());
    assert_eq!(path.bank_direction(), 1);

    for (version, flags_saved) in [(NifVersion::MORROWIND, false), (OLD, true)] {
        let mut nif = NifFile::create(version);
        let c = nif.root_node_mut().unwrap().add_controller::<NiPathController, _>();
        let pos = nif.add_animation_data::<NiPosData>();
        let float = nif.add_animation_data::<NiFloatData>();
        let mut e = nif.edit::<NiPathController>(c).unwrap();
        assert!(!e.set_path_data(Some(float)));
        assert!(e.set_path_data(Some(pos)) && e.set_percent_data(Some(float)));
        let mut b = nif.block_of_mut::<NiPathController>(c).unwrap();
        b.as_mut().set_path_flags(PathFlags::FOLLOW | PathFlags::BANK);
        b.as_mut().set_bank_direction(-1);
        b.as_mut().set_max_bank_angle(0.5);
        b.as_mut().set_smoothing(0.25);
        b.as_mut().set_follow_axis(2);

        let nif = reload_in_place(&mut nif);
        let v = nif.view::<NiPathController>(c).unwrap();
        assert_eq!((v.path_data().unwrap().index(), v.percent_data().unwrap().index()), (pos, float));
        let expected = if flags_saved { PathFlags::FOLLOW | PathFlags::BANK } else { PathFlags::empty() };
        assert_eq!(v.path_flags(), expected, "{version:?}");
        assert_eq!((v.bank_direction(), v.max_bank_angle(), v.smoothing(), v.follow_axis()), (-1, 0.5, 0.25, 2));
    }
}

#[test]
fn look_at_controller() {
    let mut nif = NifFile::create(OLD);
    let target = nif.root_node_mut().unwrap().add_child("Target", Default::default());
    let c = nif.root_node_mut().unwrap().add_controller::<NiLookAtController, _>();
    let data = nif.add_animation_data::<NiFloatData>();
    let mut e = nif.edit::<NiLookAtController>(c).unwrap();
    assert!(!e.set_look_at_node(Some(data)));
    assert!(e.set_look_at_node(Some(target)));
    nif.block_of_mut::<NiLookAtController>(c).unwrap().set_look_at_flags(LookAtFlags::FLIP | LookAtFlags::Z_AXIS);

    let nif = reload_in_place(&mut nif);
    let v = nif.view::<NiLookAtController>(c).unwrap();
    assert_eq!(v.look_at_node().unwrap().index(), target);
    assert_eq!(v.look_at_flags(), LookAtFlags::FLIP | LookAtFlags::Z_AXIS);
}

#[test]
fn uv_and_flip_controllers() {
    let nif = upstream("TestNifFile_UVController_MW");
    assert_eq!(nif.block_of::<NiUvController>(3).unwrap().uv_set(), 0);

    let mut nif = upstream("TestNifFile_Billboard_MW");
    let texture = nif.blocks_of::<NiSourceTexture>().indices().next().unwrap();
    let mut root = nif.root_node_mut().unwrap();
    let uv = root.add_controller::<NiUvController, _>();
    let flip = root.add_controller::<NiFlipController, _>();
    nif.block_of_mut::<NiUvController>(uv).unwrap().set_uv_set(1);
    let mut b = nif.block_of_mut::<NiFlipController>(flip).unwrap();
    b.as_mut().set_texture_map(TextureMap::Glow);
    b.as_mut().set_accum_time(1.5);
    b.as_mut().set_delta(0.1);
    let mut e = nif.edit::<NiFlipController>(flip).unwrap();
    assert!(!e.set_sources(&[uv]));
    assert!(e.set_sources(&[texture, texture]));

    let nif = reload_in_place(&mut nif);
    assert_eq!(nif.block_of::<NiUvController>(uv).unwrap().uv_set(), 1);
    let v = nif.view::<NiFlipController>(flip).unwrap();
    assert_eq!(v.texture_map(), Some(TextureMap::Glow));
    assert_eq!((v.accum_time(), v.delta()), (1.5, 0.1));
    assert_eq!(v.sources().indices().collect::<Vec<_>>(), [texture, texture]);
}

#[test]
fn texture_transform_controller() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let c = nif.root_node_mut().unwrap().add_controller::<NiTextureTransformController, _>();
    let mut b = nif.block_of_mut::<NiTextureTransformController>(c).unwrap();
    assert_eq!(b.map(), Some(MapSlot::Map(TextureMap::Base)));
    assert!(b.as_mut().set_map(MapSlot::Shader(3)));
    b.as_mut().set_operation(TransformMember::ScaleV);
    let nif2 = reload_in_place(&mut nif);
    let v = nif2.block_of::<NiTextureTransformController>(c).unwrap();
    assert_eq!(v.map(), Some(MapSlot::Shader(3)));
    assert_eq!(v.operation(), Some(TransformMember::ScaleV));

    let mut b = nif.block_of_mut::<NiTextureTransformController>(c).unwrap();
    assert!(b.as_mut().set_map(TextureMap::Detail));
    assert_eq!(b.map(), Some(MapSlot::Map(TextureMap::Detail)));
}

#[test]
fn color_controllers() {
    let nif = upstream("TestNifFile_Morph_MW");
    assert_eq!(nif.block_of::<NiMaterialColorController>(14).unwrap().target_color(), Some(MaterialColor::Ambient));

    for (version, saved) in [(NifVersion::MORROWIND, false), (OLD, true)] {
        let mut nif = NifFile::create(version);
        let mut root = nif.root_node_mut().unwrap();
        let material = root.add_controller::<NiMaterialColorController, _>();
        let light = root.add_controller::<NiLightColorController, _>();
        nif.block_of_mut::<NiMaterialColorController>(material).unwrap().set_target_color(MaterialColor::SelfIllumination);
        nif.block_of_mut::<NiLightColorController>(light).unwrap().set_target_color(LightColor::Ambient);
        let nif = reload_in_place(&mut nif);
        let m = nif.block_of::<NiMaterialColorController>(material).unwrap().target_color();
        let l = nif.block_of::<NiLightColorController>(light).unwrap().target_color();
        if saved {
            assert_eq!((m, l), (Some(MaterialColor::SelfIllumination), Some(LightColor::Ambient)));
        } else {
            assert_eq!((m, l), (Some(MaterialColor::Ambient), Some(LightColor::Diffuse)));
        }
    }
}

#[test]
fn shader_controllers() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let mut root = nif.root_node_mut().unwrap();
    let lf = root.add_controller::<BsLightingShaderPropertyFloatController, _>();
    let lc = root.add_controller::<BsLightingShaderPropertyColorController, _>();
    let lu = root.add_controller::<BsLightingShaderPropertyUShortController, _>();
    let ef = root.add_controller::<BsEffectShaderPropertyFloatController, _>();
    let ec = root.add_controller::<BsEffectShaderPropertyColorController, _>();
    nif.block_of_mut::<BsLightingShaderPropertyFloatController>(lf).unwrap().set_controlled_variable(LightingShaderControlledFloat::VScale);
    nif.block_of_mut::<BsLightingShaderPropertyColorController>(lc).unwrap().set_controlled_color(LightingShaderControlledColor::Emissive);
    nif.block_of_mut::<BsLightingShaderPropertyUShortController>(lu).unwrap().set_controlled_variable(1);
    nif.block_of_mut::<BsEffectShaderPropertyFloatController>(ef).unwrap().set_controlled_variable(EffectShaderControlledVariable::FalloffStopOpacity);
    nif.block_of_mut::<BsEffectShaderPropertyColorController>(ec).unwrap().set_controlled_color(EffectShaderControlledColor::Emissive);

    let nif = reload_in_place(&mut nif);
    assert_eq!(nif.block_of::<BsLightingShaderPropertyFloatController>(lf).unwrap().controlled_variable(), Some(LightingShaderControlledFloat::VScale));
    assert_eq!(nif.block_of::<BsLightingShaderPropertyColorController>(lc).unwrap().controlled_color(), Some(LightingShaderControlledColor::Emissive));
    assert_eq!(nif.block_of::<BsLightingShaderPropertyUShortController>(lu).unwrap().controlled_variable(), 1);
    assert_eq!(nif.block_of::<BsEffectShaderPropertyFloatController>(ef).unwrap().controlled_variable(), Some(EffectShaderControlledVariable::FalloffStopOpacity));
    assert_eq!(nif.block_of::<BsEffectShaderPropertyColorController>(ec).unwrap().controlled_color(), Some(EffectShaderControlledColor::Emissive));
}

#[test]
fn float_extra_data_controller() {
    for (version, saved) in [(NifVersion::SKYRIM_SE, true), (OLD, false)] {
        let mut nif = NifFile::create(version);
        let mut root = nif.root_node_mut().unwrap();
        let c = root.add_controller::<NiFloatExtraDataController, _>();
        let after = root.add_controller::<BsLagBoneController, _>();
        let interpolator = nif.add_interpolator::<NiFloatInterpolator>();
        nif.block_of_mut::<NiFloatExtraDataController>(c).unwrap().set_extra_data_name("Fade");
        assert!(nif.edit::<NiFloatExtraDataController>(c).unwrap().set_interpolator(Some(interpolator)));
        nif.block_of_mut::<BsLagBoneController>(after).unwrap().set_max_distance(7.0);

        // The blocks after the controller still read correctly.
        let nif = reload_in_place(&mut nif);
        let v = nif.view::<NiFloatExtraDataController>(c).unwrap();
        assert_eq!(v.extra_data_name(), if saved { "Fade" } else { "" }, "{version:?}");
        assert_eq!(v.interpolator().unwrap().index(), interpolator);
        assert_eq!(nif.block_of::<BsLagBoneController>(after).unwrap().max_distance(), 7.0);
    }
}

#[test]
fn geom_morpher_controller() {
    let nif = upstream("TestNifFile_Morph_MW");
    let morpher = nif.view::<NiGeomMorpherController>(10).unwrap();
    assert!(morpher.morph_interpolators().is_empty());

    let items = |i, j| [MorphWeight { interpolator: Some(i), weight: 0.5 }, MorphWeight { interpolator: Some(j), weight: 1.0 }];
    for (version, weights_saved) in [(NifVersion::SKYRIM_SE, true), (NifVersion::OBLIVION, false)] {
        let mut nif = NifFile::create(version);
        let c = nif.root_node_mut().unwrap().add_controller::<NiGeomMorpherController, _>();
        let a = nif.add_interpolator::<NiFloatInterpolator>();
        let b = nif.add_interpolator::<NiFloatInterpolator>();
        let data = nif.add_animation_data::<NiFloatData>();
        let mut e = nif.edit::<NiGeomMorpherController>(c).unwrap();
        assert!(!e.set_morph_interpolators(&items(a, data)));
        assert!(e.set_morph_interpolators(&items(a, b)));
        assert_eq!(e.view().morph_interpolators(), items(a, b), "kept in memory");
        let mut block = nif.block_of_mut::<NiGeomMorpherController>(c).unwrap();
        block.as_mut().set_morpher_flags(GeomMorpherFlags::UPDATE_NORMALS);
        block.as_mut().set_always_update(true);
        block.as_mut().set_unknown_ints(&[7, 8]);

        let nif = reload_in_place(&mut nif);
        let v = nif.view::<NiGeomMorpherController>(c).unwrap();
        let read = v.morph_interpolators();
        assert_eq!(read.iter().map(|w| w.interpolator).collect::<Vec<_>>(), [Some(a), Some(b)], "{version:?}");
        assert_eq!(read[1].weight, if weights_saved { 1.0 } else { 0.0 });
        assert_eq!(v.morpher_flags(), GeomMorpherFlags::UPDATE_NORMALS);
        assert!(v.always_update());
        assert_eq!(v.unknown_ints(), if weights_saved { vec![] } else { vec![7, 8] });
    }
}

#[test]
fn bone_lod_controller() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let a = nif.root_node_mut().unwrap().add_child("A", Default::default());
    let b = nif.root_node_mut().unwrap().add_child("B", Default::default());
    let c = nif.root_node_mut().unwrap().add_controller::<NiBsBoneLodController, _>();
    let data = nif.add_animation_data::<NiFloatData>();
    let mut e = nif.edit::<NiBsBoneLodController>(c).unwrap();
    assert!(!e.set_node_groups(&[vec![a, data]]));
    assert!(e.set_node_groups(&[vec![0, a, b], vec![a]]));
    nif.block_of_mut::<NiBoneLodController>(c).unwrap().set_lod(1);

    let nif = reload_in_place(&mut nif);
    let v = nif.view::<NiBoneLodController>(c).unwrap();
    assert_eq!(v.node_groups(), [vec![0, a, b], vec![a]]);
    assert_eq!((v.lod(), v.lod_count()), (1, 2));
}

#[test]
fn lightning_controller() {
    let mut nif = upstream("TestNifFile_Static_SE");
    let shader = nif.blocks_of::<BsShaderProperty>().indices().next().unwrap();
    let c = nif.root_node_mut().unwrap().add_controller::<BsProceduralLightningController, _>();
    let width = nif.add_interpolator::<NiFloatInterpolator>();
    let arc = nif.add_interpolator::<NiFloatInterpolator>();
    let mut e = nif.edit::<BsProceduralLightningController>(c).unwrap();
    assert!(!e.set_lightning_shader(Some(width)));
    assert!(e.set_lightning_shader(Some(shader)));
    assert!(!e.set_lightning_interpolator(LightningInterpolator::Width, Some(shader)));
    assert!(e.set_lightning_interpolator(LightningInterpolator::Width, Some(width)));
    assert!(e.set_lightning_interpolator(LightningInterpolator::ArcOffset, Some(arc)));
    let mut b = nif.block_of_mut::<BsProceduralLightningController>(c).unwrap();
    b.as_mut().set_branch_count(3);
    b.as_mut().set_child_width_multiplier(0.75);
    b.as_mut().set_fade_child_bolts(true);

    let nif = reload_in_place(&mut nif);
    let v = nif.view::<BsProceduralLightningController>(c).unwrap();
    assert_eq!(v.lightning_shader().unwrap().index(), shader);
    assert_eq!(v.lightning_interpolator(LightningInterpolator::Width).unwrap().index(), width);
    assert_eq!(v.lightning_interpolator(LightningInterpolator::ArcOffset).unwrap().index(), arc);
    assert!(v.lightning_interpolator(LightningInterpolator::Generation).is_none());
    assert_eq!((v.branch_count(), v.child_width_multiplier(), v.fade_child_bolts()), (3, 0.75, true));
}

#[test]
fn multi_target_controller() {
    let nif = upstream("TestNifFile_Animated_LE");
    assert!(nif.view::<NiMultiTargetTransformController>(4).unwrap().extra_targets().iter().any(Option::is_some));

    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let a = nif.root_node_mut().unwrap().add_child("A", Default::default());
    let c = nif.root_node_mut().unwrap().add_controller::<NiMultiTargetTransformController, _>();
    let mut e = nif.edit::<NiMultiTargetTransformController>(c).unwrap();
    assert!(!e.set_extra_targets(&[Some(a), Some(c)]));
    assert!(!e.set_extra_targets(&vec![Some(a); 65536]));
    assert!(e.set_extra_targets(&[Some(a), None, Some(0)]));
    let nif = reload_in_place(&mut nif);
    assert_eq!(nif.view::<NiMultiTargetTransformController>(c).unwrap().extra_targets(), [Some(a), None, Some(0)]);
}

#[test]
fn other_controllers() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let mut root = nif.root_node_mut().unwrap();
    let lag = root.add_controller::<BsLagBoneController, _>();
    let blend = root.add_controller::<BhkBlendController, _>();
    let fov = root.add_controller::<BsFrustumFovController, _>();
    let interpolator = nif.add_interpolator::<NiFloatInterpolator>();
    let mut b = nif.block_of_mut::<BsLagBoneController>(lag).unwrap();
    b.as_mut().set_linear_velocity(3.0);
    b.as_mut().set_linear_rotation(1.0);
    nif.block_of_mut::<BhkBlendController>(blend).unwrap().set_keys(4);
    assert!(nif.edit::<BsFrustumFovController>(fov).unwrap().set_interpolator(Some(interpolator)));

    let nif = reload_in_place(&mut nif);
    let b = nif.block_of::<BsLagBoneController>(lag).unwrap();
    assert_eq!((b.linear_velocity(), b.linear_rotation()), (3.0, 1.0));
    assert_eq!(nif.block_of::<BhkBlendController>(blend).unwrap().keys(), 4);
    assert_eq!(nif.view::<BsFrustumFovController>(fov).unwrap().interpolator().unwrap().index(), interpolator);
    assert_eq!(nif.view::<NiTimeController>(fov).unwrap().controller_target().unwrap().index(), 0);
}
