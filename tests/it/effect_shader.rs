//! Tests for the BsEffectShaderProperty fields.

use crate::common::{UPSTREAM, reload};
use std::fs;

use nifly::{
    file::{Index, NifFile},
    shaders::{BsEffectShaderProperty, ClampMode},
};

fn load(name: &str) -> NifFile {
    NifFile::load(fs::read(format!("{UPSTREAM}/{name}.nif")).unwrap().as_slice(), None).unwrap()
}

/// The first effect shader with (or without) a name.
fn index(nif: &NifFile, named: bool) -> Index {
    nif.blocks_of::<BsEffectShaderProperty>().indexed().find(|(_, s)| s.name().is_empty() != named).unwrap().0
}

fn shader(nif: &NifFile, named: bool) -> &BsEffectShaderProperty {
    nif.block_of::<BsEffectShaderProperty>(index(nif, named)).unwrap()
}

fn shader_mut(nif: &mut NifFile, named: bool) -> &mut BsEffectShaderProperty {
    let i = index(nif, named);
    nif.block_of_mut::<BsEffectShaderProperty>(i).unwrap()
}

#[test]
fn reads_fields() {
    let nif = load("TestNifFile_LooseBlocks_SE");
    let s = shader(&nif, false);
    assert_eq!(s.source_texture(), "textures\\Tile.dds");
    assert_eq!(s.greyscale_texture(), "textures\\Gradient.dds");
    assert_eq!(s.base_color(), [0.982, 1.0, 0.837, 0.77]);
    assert_eq!(s.texture_clamp_mode(), Some(ClampMode::WrapBoth));
    assert_eq!((s.lighting_influence(), s.env_map_min_lod()), (255, 0));
}

#[test]
fn skyrim_fields_round_trip() {
    let mut nif = load("TestNifFile_LooseBlocks_SE");
    let s = shader_mut(&mut nif, false);
    s.set_source_texture("textures\\a.dds");
    s.set_greyscale_texture("textures\\b.dds");
    s.set_falloff_start_angle(0.25);
    s.set_falloff_stop_angle(0.5);
    s.set_falloff_start_opacity(0.75);
    s.set_falloff_stop_opacity(0.125);
    s.set_base_color([0.5, 0.25, 0.125, 1.0]);
    s.set_base_color_scale(2.0);
    s.set_soft_falloff_depth(10.0);
    s.set_texture_clamp_mode(ClampMode::ClampUWrapV);
    s.set_lighting_influence(128);
    s.set_env_map_min_lod(3);
    // Not saved in Skyrim.
    s.set_env_map_texture("textures\\env.dds");
    s.set_env_map_scale(2.0);
    s.set_lum_emittance(5.0);

    let saved = reload(&mut nif);
    let s = shader(&saved, false);
    assert_eq!((s.source_texture(), s.greyscale_texture()), ("textures\\a.dds".into(), "textures\\b.dds".into()));
    assert_eq!(
        (s.falloff_start_angle(), s.falloff_stop_angle(), s.falloff_start_opacity(), s.falloff_stop_opacity()),
        (0.25, 0.5, 0.75, 0.125)
    );
    assert_eq!((s.base_color(), s.base_color_scale()), ([0.5, 0.25, 0.125, 1.0], 2.0));
    assert_eq!(s.soft_falloff_depth(), 10.0);
    assert_eq!(s.texture_clamp_mode(), Some(ClampMode::ClampUWrapV));
    assert_eq!((s.lighting_influence(), s.env_map_min_lod()), (128, 3));
    assert_eq!((s.env_map_texture(), s.env_map_scale(), s.lum_emittance()), ("".into(), 1.0, 100.0));
}

#[test]
fn clamp_bytes_are_independent() {
    let mut nif = load("TestNifFile_LooseBlocks_SE");
    let s = shader_mut(&mut nif, false);
    s.set_lighting_influence(7);
    s.set_env_map_min_lod(9);
    s.set_texture_clamp_mode(ClampMode::ClampBoth);
    s.set_lighting_influence(8);
    assert_eq!((s.texture_clamp_mode(), s.lighting_influence(), s.env_map_min_lod()), (Some(ClampMode::ClampBoth), 8, 9));
}

#[test]
fn fallout_4_fields_round_trip() {
    let mut nif = load("TestNifFile_Static_FO4_139");
    let s = shader_mut(&mut nif, false);
    s.set_env_map_texture("textures\\env.dds");
    s.set_normal_texture("textures\\n.dds");
    s.set_env_mask_texture("textures\\mask.dds");
    s.set_env_map_scale(0.5);
    // Fallout 76's.
    s.set_reflectance_texture("textures\\r.dds");
    s.set_refraction_power(2.0);

    let saved = reload(&mut nif);
    let s = shader(&saved, false);
    assert_eq!(s.env_map_texture(), "textures\\env.dds");
    assert_eq!(s.normal_texture(), "textures\\n.dds");
    assert_eq!(s.env_mask_texture(), "textures\\mask.dds");
    assert_eq!(s.env_map_scale(), 0.5);
    assert_eq!((s.reflectance_texture(), s.refraction_power()), ("".into(), 0.0));
}

#[test]
fn fallout_76_fields_round_trip() {
    let mut nif = load("TestNifFile_FO76");
    let s = shader_mut(&mut nif, false);
    s.set_reflectance_texture("textures\\r.dds");
    s.set_lighting_texture("textures\\l.dds");
    s.set_emit_gradient_texture("textures\\g.dds");
    s.set_emittance_color([0.25, 0.5, 0.75]);
    s.set_lum_emittance(50.0);
    s.set_exposure_offset(12.0);
    s.set_final_exposure_min(1.5);
    s.set_final_exposure_max(2.5);
    s.set_refraction_power(3.0);

    let saved = reload(&mut nif);
    let s = shader(&saved, false);
    assert_eq!(s.reflectance_texture(), "textures\\r.dds");
    assert_eq!(s.lighting_texture(), "textures\\l.dds");
    assert_eq!(s.emit_gradient_texture(), "textures\\g.dds");
    assert_eq!(s.emittance_color(), [0.25, 0.5, 0.75]);
    assert_eq!(s.lum_emittance(), 50.0);
    assert_eq!(s.exposure_offset(), 12.0);
    assert_eq!((s.final_exposure_min(), s.final_exposure_max()), (1.5, 2.5));
    assert_eq!(s.refraction_power(), 3.0);
}

#[test]
fn named_fallout_76_shader_saves_no_fields() {
    let mut nif = load("TestNifFile_FO76");
    shader_mut(&mut nif, true).set_source_texture("textures\\x.dds");
    assert_eq!(shader(&reload(&mut nif), true).source_texture(), "");
}
