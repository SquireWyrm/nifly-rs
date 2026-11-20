//! Tests for effects and lights: NiDynamicEffect, NiTextureEffect, NiLight and its subclasses,
//! node effect lists, add_effect, and the spot light inner angle by version.

use crate::common::upstream;
use std::io::Cursor;

use nifly::{
    effects::{
        CoordinateGeneration, EffectType, NiAmbientLight, NiDirectionalLight, NiDynamicEffect, NiLight, NiPointLight,
        NiSpotLight, NiTextureEffect, Plane,
    },
    file::{Index, NifFile, SaveOptions},
    nodes::NiNode,
    object3d::{Matrix3, Vec3},
    shaders::{ClampMode, FilterMode, NiSourceTexture},
};

/// Saves without optimizing or sorting, so block indices stay the same.
fn reload(nif: &mut NifFile) -> NifFile {
    let mut cur = Cursor::new(Vec::new());
    nif.save(&mut cur, Some(SaveOptions { optimize: false, sort_blocks: false })).unwrap();
    NifFile::load(cur.into_inner().as_slice(), None).unwrap()
}

fn effects(nif: &NifFile, node: Index) -> Vec<Index> {
    nif.view::<NiNode>(node).unwrap().effects().indices().collect()
}

#[test]
fn reads_morrowind_texture_effects() {
    let nif = upstream("TestNifFile_TextureEffect_MW");
    assert_eq!(effects(&nif, 0), [19]);
    assert_eq!(effects(&nif, 2), [3]);
    // Each effect is also a child of the node listing it.
    assert!(nif.view::<NiNode>(2).unwrap().children().indices().any(|i| i == 3));

    let effect = nif.view::<NiTextureEffect>(3).unwrap();
    assert_eq!(effect.projection_matrix(), Matrix3::IDENTITY);
    assert_eq!(effect.projection_translation(), Vec3::ZERO);
    assert_eq!(effect.filter_mode(), Some(FilterMode::Trilinear));
    assert_eq!(effect.clamp_mode(), Some(ClampMode::WrapBoth));
    assert_eq!(effect.effect_type(), Some(EffectType::EnvironmentMap));
    assert_eq!(effect.coordinate_generation(), Some(CoordinateGeneration::SphereMap));
    assert_eq!(effect.source_texture().map(|t| t.index()), Some(4));
    assert!(!effect.clipping());
    assert_eq!(effect.clipping_plane(), Plane { normal: Vec3::new(1.0, 0.0, 0.0), constant: 0.0 });
    assert_eq!((effect.ps2_l(), effect.ps2_k()), (0, -75));
    // Morrowind stores affected nodes as pointers, not block indices.
    assert_eq!(effect.affected_node_pointers(), [12467848]);
    assert_eq!(effect.affected_nodes().count(), 0);
    assert!(effect.switch_state());
    assert_eq!(nif.view::<NiTextureEffect>(19).unwrap().affected_node_pointers(), [12333816]);
}

#[test]
fn texture_effect_fields_survive_save() {
    let mut nif = upstream("TestNifFile_TextureEffect_MW");
    let mut edit = nif.edit::<NiTextureEffect>(3).unwrap();
    let b = &mut *edit;
    let matrix = Matrix3 { rows: [Vec3::new(0.0, 1.0, 0.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 2.0)] };
    b.set_projection_matrix(matrix);
    b.set_projection_translation(Vec3::new(1.0, 2.0, 3.0));
    b.set_filter_mode(FilterMode::Bilinear);
    b.set_clamp_mode(ClampMode::ClampBoth);
    b.set_effect_type(EffectType::ProjectedShadow);
    b.set_coordinate_generation(CoordinateGeneration::WorldPerspective);
    b.set_clipping(true);
    b.set_clipping_plane(Plane { normal: Vec3::new(0.0, 0.0, 1.0), constant: 5.0 });
    b.set_ps2_l(3);
    b.set_ps2_k(-4);
    b.set_affected_node_pointers(&[1, 2]);
    assert!(edit.set_source_texture(Some(7)));

    let saved = reload(&mut nif);
    let effect = saved.view::<NiTextureEffect>(3).unwrap();
    assert_eq!(effect.projection_matrix(), matrix);
    assert_eq!(effect.projection_translation(), Vec3::new(1.0, 2.0, 3.0));
    assert_eq!(effect.filter_mode(), Some(FilterMode::Bilinear));
    assert_eq!(effect.clamp_mode(), Some(ClampMode::ClampBoth));
    assert_eq!(effect.effect_type(), Some(EffectType::ProjectedShadow));
    assert_eq!(effect.coordinate_generation(), Some(CoordinateGeneration::WorldPerspective));
    assert!(effect.clipping());
    assert_eq!(effect.clipping_plane(), Plane { normal: Vec3::new(0.0, 0.0, 1.0), constant: 5.0 });
    assert_eq!((effect.ps2_l(), effect.ps2_k()), (3, -4));
    assert_eq!(effect.affected_node_pointers(), [1, 2]);
    assert_eq!(effect.source_texture().map(|t| t.index()), Some(7));
}

#[test]
fn set_source_texture_checks_type() {
    let mut nif = upstream("TestNifFile_TextureEffect_MW");
    let mut edit = nif.edit::<NiTextureEffect>(3).unwrap();
    // Block 5 is a shape; the others don't exist.
    for bad in [5, 27, usize::MAX] {
        assert!(!edit.set_source_texture(Some(bad)), "{bad}");
    }
    assert_eq!(edit.source_texture().map(|t| t.index()), Some(4));
    edit.source_texture().unwrap().set_file_name("textures\\new.dds");
    assert_eq!(nif.view::<NiSourceTexture>(4).unwrap().file_name(), "textures\\new.dds");

    let mut edit = nif.edit::<NiTextureEffect>(3).unwrap();
    assert!(edit.set_source_texture(None));
    assert!(edit.view().source_texture().is_none());
    assert!(edit.source_texture().is_none());
}

#[test]
fn set_effects_checks_type() {
    let mut nif = upstream("TestNifFile_TextureEffect_MW");
    let mut node = nif.edit::<NiNode>(2).unwrap();
    // Block 4 is a texture, 5 a shape.
    for bad in [&[3, 4][..], &[5], &[usize::MAX]] {
        assert!(!node.set_effects(bad), "{bad:?}");
    }
    assert_eq!(effects(&nif, 2), [3]);

    let mut node = nif.edit::<NiNode>(2).unwrap();
    assert!(node.set_effects(&[19, 3]));
    assert_eq!(effects(&nif, 2), [19, 3]);
    assert_eq!(effects(&reload(&mut nif), 2), [19, 3]);

    let mut node = nif.edit::<NiNode>(2).unwrap();
    assert!(node.set_effects(&[]));
    assert!(effects(&nif, 2).is_empty());
}

#[test]
fn affected_nodes() {
    let mut nif = upstream("TestNifFile_Skinned_OB");
    let light = nif.edit::<NiNode>(0).unwrap().add_effect::<NiAmbientLight, _>();
    let shape = nif.shapes().indices().next().unwrap();
    let mut edit = nif.edit::<NiAmbientLight>(light).unwrap();
    assert!(!edit.set_affected_nodes(&[shape]), "a shape isn't a node");
    assert!(!edit.set_affected_nodes(&[usize::MAX]));
    assert_eq!(edit.view().affected_nodes().count(), 0);
    assert!(edit.set_affected_nodes(&[0]));
    assert_eq!(edit.view().affected_nodes().indices().collect::<Vec<_>>(), [0]);

    let saved = reload(&mut nif);
    let light = saved.blocks_of::<NiAmbientLight>().views().next().unwrap();
    assert_eq!(light.affected_nodes().map(|n| n.name().to_owned()).collect::<Vec<_>>(), [saved.root_node().unwrap().name()]);
    // A node can reach them through its effects, as the base type.
    let effect = saved.root_node().unwrap().effects().views().next().unwrap();
    assert_eq!(effect.index(), light.index());
    assert_eq!(effect.affected_nodes().count(), 1);
}

fn set_light(nif: &mut NifFile, index: Index) {
    let mut edit = nif.edit::<NiLight>(index).unwrap();
    let light = &mut *edit;
    light.set_dimmer(0.5);
    light.set_ambient_color([0.1, 0.2, 0.3]);
    light.set_diffuse_color([0.4, 0.5, 0.6]);
    light.set_specular_color([0.7, 0.8, 0.9]);
    light.set_switch_state(false);
}

fn check_light(light: &NiLight) {
    assert_eq!(light.dimmer(), 0.5);
    assert_eq!(light.ambient_color(), [0.1, 0.2, 0.3]);
    assert_eq!(light.diffuse_color(), [0.4, 0.5, 0.6]);
    assert_eq!(light.specular_color(), [0.7, 0.8, 0.9]);
}

#[test]
fn new_lights_have_defaults() {
    let mut nif = upstream("TestNifFile_Skinned_OB");
    let root = nif.root_node().unwrap().index();
    let count = nif.blocks().count();
    let point = nif.edit::<NiNode>(root).unwrap().add_effect::<NiPointLight, _>();
    let texture = nif.edit::<NiNode>(root).unwrap().add_effect::<NiTextureEffect, _>();
    assert_eq!((point, texture), (count, count + 1));
    assert_eq!(effects(&nif, root), [point, texture]);

    let light = nif.view::<NiPointLight>(point).unwrap();
    assert_eq!(light.dimmer(), 1.0);
    assert_eq!(light.diffuse_color(), [0.0; 3]);
    assert!(light.switch_state());
    assert_eq!(
        (light.constant_attenuation(), light.linear_attenuation(), light.quadratic_attenuation()),
        (0.0, 1.0, 0.0)
    );
    let effect = nif.view::<NiTextureEffect>(texture).unwrap();
    assert_eq!(effect.projection_matrix(), Matrix3::IDENTITY);
    assert_eq!(effect.effect_type(), Some(EffectType::EnvironmentMap));
    assert!(effect.source_texture().is_none());
}

#[test]
fn lights_survive_save() {
    let mut nif = upstream("TestNifFile_Static_SE");
    let root = nif.root_node().unwrap().index();
    let ambient = nif.edit::<NiNode>(root).unwrap().add_effect::<NiAmbientLight, _>();
    let directional = nif.edit::<NiNode>(root).unwrap().add_effect::<NiDirectionalLight, _>();
    let point = nif.edit::<NiNode>(root).unwrap().add_effect::<NiPointLight, _>();
    let spot = nif.edit::<NiNode>(root).unwrap().add_effect::<NiSpotLight, _>();
    for i in [ambient, directional, point, spot] {
        set_light(&mut nif, i);
        assert!(nif.edit::<NiDynamicEffect>(i).unwrap().set_parent(Some(root)));
    }
    let mut edit = nif.edit::<NiPointLight>(point).unwrap();
    let p = &mut *edit;
    p.set_constant_attenuation(1.0);
    p.set_linear_attenuation(2.0);
    p.set_quadratic_attenuation(3.0);
    let mut edit = nif.edit::<NiSpotLight>(spot).unwrap();
    let s = &mut *edit;
    s.set_outer_angle(0.75);
    s.set_inner_angle(0.25);
    s.set_exponent(2.0);

    let saved = reload(&mut nif);
    let root = saved.root_node().unwrap();
    let lights: Vec<_> = root.effects().of::<NiLight>().collect();
    assert_eq!(lights.len(), 4);
    lights.iter().for_each(|l| check_light(l));
    // Skyrim saves the switch state.
    assert!(lights.iter().all(|l| !l.switch_state()));
    assert_eq!(root.effects().of::<NiAmbientLight>().count(), 1);
    assert_eq!(root.effects().of::<NiDirectionalLight>().count(), 1);
    assert_eq!(root.children().of::<NiLight>().count(), 4);

    let p = root.effects().of::<NiPointLight>().next().unwrap();
    assert_eq!((p.constant_attenuation(), p.linear_attenuation(), p.quadratic_attenuation()), (1.0, 2.0, 3.0));
    let s = root.effects().of::<NiSpotLight>().next().unwrap();
    assert_eq!((s.outer_angle(), s.inner_angle(), s.exponent()), (0.75, 0.25, 2.0));
    check_light(s);
}

/// A spot light added to `name`, saved and reloaded in its version, as
/// `(outer, inner, exponent)`. A copy of the file writes it the same way.
fn spot_light_round_trip(name: &str) -> (f32, f32, f32) {
    let mut nif = upstream(name);
    let root = nif.root_node().unwrap().index();
    let spot = nif.edit::<NiNode>(root).unwrap().add_effect::<NiSpotLight, _>();
    let mut edit = nif.edit::<NiSpotLight>(spot).unwrap();
    let s = &mut *edit;
    s.set_outer_angle(0.75);
    s.set_inner_angle(0.25);
    s.set_exponent(2.0);
    set_light(&mut nif, spot);
    let mut copy = nif.clone();

    let results = [&mut nif, &mut copy].map(|file| {
        let saved = reload(file);
        // Everything after the angle is read where it was written.
        let s = saved.blocks_of::<NiSpotLight>().next().unwrap();
        check_light(s);
        assert_eq!(saved.root_node().unwrap().effects().count(), 1, "{name}");
        (s.outer_angle(), s.inner_angle(), s.exponent())
    });
    assert_eq!(results[0], results[1], "{name}");
    results[0]
}

#[test]
fn spot_light_inner_angle_by_version() {
    // Not stored before Fallout 3: the exponent after it is still read correctly.
    assert_eq!(spot_light_round_trip("TestNifFile_Static_MW"), (0.75, 0.0, 2.0));
    assert_eq!(spot_light_round_trip("TestNifFile_Skinned_OB"), (0.75, 0.0, 2.0));
    assert_eq!(spot_light_round_trip("TestNifFile_Static_SE"), (0.75, 0.25, 2.0));
}

#[test]
fn fallout_4_drops_effect_lists() {
    let mut nif = upstream("TestNifFile_Static_FO4");
    let root = nif.root_node().unwrap().index();
    let light = nif.edit::<NiNode>(root).unwrap().add_effect::<NiPointLight, _>();
    assert_eq!(effects(&nif, root), [light]);
    assert!(nif.edit::<NiPointLight>(light).unwrap().set_parent(Some(root)));
    set_light(&mut nif, light);

    let saved = reload(&mut nif);
    let root = saved.root_node().unwrap();
    assert_eq!(root.effects().count(), 0);
    // As a child, the light itself is kept.
    let light = root.children().of::<NiPointLight>().next().unwrap();
    check_light(light);
}
