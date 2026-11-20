//! Tests for the BsLightingShaderProperty fields and BsShaderProperty's UV and environment map
//! fields.

use crate::common::{UPSTREAM, reload};
use std::{fs, io::Cursor};

use nifly::{
    file::{NifFile, NifVersion},
    object3d::Vec3,
    shaders::{BsLightingShaderProperty, ClampMode, LightingShaderType, Translucency},
};

fn load(path: &str) -> NifFile {
    NifFile::load(fs::read(path).unwrap().as_slice(), None).unwrap()
}

/// The first lighting shader.
fn shader(nif: &NifFile) -> &BsLightingShaderProperty {
    nif.blocks_of::<BsLightingShaderProperty>().next().unwrap()
}

fn shader_mut(nif: &mut NifFile) -> &mut BsLightingShaderProperty {
    let index = nif
        .blocks_of::<BsLightingShaderProperty>()
        .indices()
        .next()
        .unwrap();
    nif.block_of_mut::<BsLightingShaderProperty>(index).unwrap()
}

/// A Fallout 76 file with one new shape, whose shader has no name.
fn fallout_76() -> NifFile {
    let mut nif = NifFile::create(NifVersion::FALLOUT_76);
    let vertices = [Vec3::ZERO, Vec3::X, Vec3::Y];
    nif.create_shape("Tri", &vertices, &[[0, 1, 2]], None, None)
        .unwrap();
    assert_eq!(shader(&nif).name(), "");
    nif
}

#[test]
fn reads_fields() {
    let nif = load("assets/Cube.nif");
    let s = shader(&nif);
    assert_eq!(s.shader_type(), Some(LightingShaderType::Default));
    assert_eq!(s.texture_clamp_mode(), Some(ClampMode::WrapBoth));
    assert_eq!((s.uv_offset(), s.uv_scale()), ([0.0, 0.0], [1.0, 1.0]));
    assert_eq!(s.translucency(), None);
    assert!(s.texture_arrays().is_empty());

    let nif = load(&format!("{UPSTREAM}/TestNifFile_Static_FO4_132.nif"));
    let s = shader(&nif);
    assert_eq!(s.shader_type(), Some(LightingShaderType::EnvironmentMap));
    assert_eq!(s.environment_map_scale(), 0.7);
    assert_eq!(s.wet_material(), "template/CreatureTemplate_Wet.bgsm");
}

#[test]
fn shared_fields_round_trip() {
    let mut nif = load("assets/Cube.nif");
    let s = shader_mut(&mut nif);
    s.set_emissive_color([0.25, 0.5, 0.75]);
    s.set_emissive_multiple(2.0);
    s.set_alpha(0.5);
    s.set_refraction_strength(0.125);
    s.set_glossiness(40.0);
    s.set_specular_color([1.0, 0.5, 0.0]);
    s.set_specular_strength(3.0);
    s.set_soft_lighting(0.5);
    s.set_rimlight_power(4.0);
    s.set_texture_clamp_mode(ClampMode::ClampUWrapV);
    // Not saved in Skyrim.
    s.set_subsurface_rolloff(0.9);
    s.set_wet_material("wet.bgsm");
    s.set_lum_emittance(5.0);
    let base = shader_mut(&mut nif);
    base.set_uv_offset([0.5, -0.5]);
    base.set_uv_scale([2.0, 3.0]);

    let saved = reload(&mut nif);
    let s = shader(&saved);
    assert_eq!(s.emissive_color(), [0.25, 0.5, 0.75]);
    assert_eq!(s.emissive_multiple(), 2.0);
    assert_eq!(s.alpha(), 0.5);
    assert_eq!(s.refraction_strength(), 0.125);
    assert_eq!(s.glossiness(), 40.0);
    assert_eq!(s.specular_color(), [1.0, 0.5, 0.0]);
    assert_eq!(s.specular_strength(), 3.0);
    assert_eq!(s.soft_lighting(), 0.5);
    assert_eq!(s.rimlight_power(), 4.0);
    assert_eq!(s.texture_clamp_mode(), Some(ClampMode::ClampUWrapV));
    assert_eq!((s.uv_offset(), s.uv_scale()), ([0.5, -0.5], [2.0, 3.0]));
    assert_eq!(s.subsurface_rolloff(), 0.3);
    assert_eq!(s.wet_material(), "");
    assert_eq!(s.lum_emittance(), 100.0);
}

#[test]
fn type_fields_round_trip() {
    let set = |shader_type, f: &dyn Fn(&mut BsLightingShaderProperty)| {
        let mut nif = load("assets/Cube.nif");
        let s = shader_mut(&mut nif);
        s.set_shader_type(shader_type);
        f(s);
        let saved = reload(&mut nif);
        assert_eq!(shader(&saved).shader_type(), Some(shader_type));
        saved
    };

    let mut nif = load("assets/Cube.nif");
    shader_mut(&mut nif).set_shader_type(LightingShaderType::EnvironmentMap);
    shader_mut(&mut nif).set_environment_map_scale(2.5);
    assert_eq!(shader(&reload(&mut nif)).environment_map_scale(), 2.5);
    let nif = set(LightingShaderType::SkinTint, &|s| {
        s.set_skin_tint_color([0.5, 0.25, 0.125])
    });
    assert_eq!(shader(&nif).skin_tint_color(), [0.5, 0.25, 0.125]);
    let nif = set(LightingShaderType::HairTint, &|s| {
        s.set_hair_tint_color([0.1, 0.2, 0.3])
    });
    assert_eq!(shader(&nif).hair_tint_color(), [0.1, 0.2, 0.3]);
    let nif = set(LightingShaderType::ParallaxOcclusion, &|s| {
        s.set_parallax_max_passes(8.0);
        s.set_parallax_scale(0.25);
    });
    assert_eq!(
        (
            shader(&nif).parallax_max_passes(),
            shader(&nif).parallax_scale()
        ),
        (8.0, 0.25)
    );
    let nif = set(LightingShaderType::MultiLayerParallax, &|s| {
        s.set_parallax_inner_layer_thickness(0.5);
        s.set_parallax_refraction_scale(0.75);
        s.set_parallax_inner_layer_texture_scale([2.0, 4.0]);
        s.set_parallax_envmap_strength(0.125);
    });
    let s = shader(&nif);
    assert_eq!(s.parallax_inner_layer_thickness(), 0.5);
    assert_eq!(s.parallax_refraction_scale(), 0.75);
    assert_eq!(s.parallax_inner_layer_texture_scale(), [2.0, 4.0]);
    assert_eq!(s.parallax_envmap_strength(), 0.125);
    let nif = set(LightingShaderType::MultiIndexSnow, &|s| {
        s.set_sparkle_parameters([1.0, 2.0, 3.0, 4.0])
    });
    assert_eq!(shader(&nif).sparkle_parameters(), [1.0, 2.0, 3.0, 4.0]);
    let nif = set(LightingShaderType::Eye, &|s| {
        s.set_eye_cubemap_scale(1.5);
        s.set_eye_left_reflection_center(Vec3::new(-1.0, 2.0, 3.0));
        s.set_eye_right_reflection_center(Vec3::new(1.0, 2.0, 3.0));
    });
    let s = shader(&nif);
    assert_eq!(s.eye_cubemap_scale(), 1.5);
    assert_eq!(s.eye_left_reflection_center(), Vec3::new(-1.0, 2.0, 3.0));
    assert_eq!(s.eye_right_reflection_center(), Vec3::new(1.0, 2.0, 3.0));

    // Another type's fields aren't saved.
    let nif = set(LightingShaderType::Default, &|s| {
        s.set_skin_tint_color([0.0; 3])
    });
    assert_eq!(shader(&nif).skin_tint_color(), [1.0; 3]);
}

#[test]
fn fallout_4_fields_round_trip() {
    let mut nif = load(&format!("{UPSTREAM}/TestNifFile_Static_FO4.nif"));
    let s = shader_mut(&mut nif);
    s.set_shader_type(LightingShaderType::EnvironmentMap);
    s.set_wet_material("wet.bgsm");
    s.set_subsurface_rolloff(0.5);
    s.set_backlight_power(2.5);
    s.set_grayscale_to_palette_scale(0.5);
    s.set_fresnel_power(4.0);
    s.set_wetness_specular_scale(0.25);
    s.set_wetness_specular_power(2.0);
    s.set_wetness_min_var(0.125);
    s.set_wetness_envmap_scale(0.75);
    s.set_wetness_fresnel_power(1.5);
    s.set_wetness_metalness(0.5);
    s.set_use_ssr(true);
    s.set_wetness_use_ssr(true);
    // Skyrim's.
    s.set_soft_lighting(0.9);

    let mut saved = reload(&mut nif);
    let s = shader(&saved);
    assert_eq!(s.wet_material(), "wet.bgsm");
    assert_eq!(s.subsurface_rolloff(), 0.5);
    assert_eq!((s.rimlight_power_2(), s.backlight_power()), (f32::MAX, 2.5));
    assert_eq!(s.grayscale_to_palette_scale(), 0.5);
    assert_eq!(s.fresnel_power(), 4.0);
    assert_eq!(s.wetness_specular_scale(), 0.25);
    assert_eq!(s.wetness_specular_power(), 2.0);
    assert_eq!(s.wetness_min_var(), 0.125);
    assert_eq!(s.wetness_envmap_scale(), 0.75);
    assert_eq!(s.wetness_fresnel_power(), 1.5);
    assert_eq!(s.wetness_metalness(), 0.5);
    assert!(s.use_ssr() && s.wetness_use_ssr());
    assert_eq!(s.soft_lighting(), 0.3);

    // Back lighting is only saved while rim lighting is f32::MAX.
    shader_mut(&mut saved).set_rimlight_power_2(3.0);
    let saved = reload(&mut saved);
    assert_eq!(
        (
            shader(&saved).rimlight_power_2(),
            shader(&saved).backlight_power()
        ),
        (3.0, 0.0)
    );

    // Skin tint alpha, from Fallout 4 on.
    let mut nif = load(&format!("{UPSTREAM}/TestNifFile_Static_FO4.nif"));
    let s = shader_mut(&mut nif);
    s.set_shader_type(LightingShaderType::SkinTint);
    s.set_skin_tint_alpha(0.5);
    assert_eq!(shader(&reload(&mut nif)).skin_tint_alpha(), 0.5);
}

#[test]
fn fallout_76_fields_round_trip() {
    let translucency = Translucency {
        subsurface_color: [1.0, 0.5, 0.25],
        transmissive_scale: 0.5,
        turbulence: 0.25,
        thick_object: true,
        mix_albedo: true,
    };
    let arrays = [vec!["a.dds", "b.dds"], vec![], vec!["c.dds"]];

    let mut nif = fallout_76();
    let s = shader_mut(&mut nif);
    s.set_lum_emittance(50.0);
    s.set_exposure_offset(12.0);
    s.set_final_exposure_min(1.5);
    s.set_final_exposure_max(2.5);
    s.set_translucency(Some(translucency));
    s.set_texture_arrays(arrays.clone());
    let mut saved = reload(&mut nif);
    let s = shader(&saved);
    assert_eq!(s.lum_emittance(), 50.0);
    assert_eq!(s.exposure_offset(), 12.0);
    assert_eq!((s.final_exposure_min(), s.final_exposure_max()), (1.5, 2.5));
    assert_eq!(s.translucency(), Some(translucency));
    assert_eq!(s.texture_arrays(), arrays);

    // Both can be turned off again.
    let s = shader_mut(&mut saved);
    s.set_translucency(None);
    s.set_texture_arrays(Vec::<Vec<&str>>::new());
    let saved = reload(&mut saved);
    assert_eq!(shader(&saved).translucency(), None);
    assert!(shader(&saved).texture_arrays().is_empty());
}

#[test]
fn shader_types_by_version() {
    // Every type saves in Skyrim.
    let mut nif = load("assets/Cube.nif");
    shader_mut(&mut nif).set_shader_type(LightingShaderType::Parallax);
    assert_eq!(
        shader(&reload(&mut nif)).shader_type(),
        Some(LightingShaderType::Parallax)
    );

    // Fallout 76 saves the types before Parallax.
    let mut nif = fallout_76();
    shader_mut(&mut nif).set_shader_type(LightingShaderType::EnvironmentMap);
    shader_mut(&mut nif).set_environment_map_scale(2.0);
    let saved = reload(&mut nif);
    assert_eq!(
        shader(&saved).shader_type(),
        Some(LightingShaderType::EnvironmentMap)
    );
    assert_eq!(shader(&saved).environment_map_scale(), 2.0);

    // Later ones are saved as the next type, with the previous type's fields. The file being
    // saved keeps the type that was set.
    for (set, stored) in [
        (LightingShaderType::Parallax, LightingShaderType::FaceTint),
        (LightingShaderType::FaceTint, LightingShaderType::SkinTint),
        (LightingShaderType::Eye, LightingShaderType::Cloud),
    ] {
        shader_mut(&mut nif).set_shader_type(set);
        assert_eq!(
            shader(&reload(&mut nif)).shader_type(),
            Some(stored),
            "{set:?}"
        );
        assert_eq!(shader(&nif).shader_type(), Some(set));
    }
    // HairTint writes SkinTint's fields where ParallaxOcclusion's are read.
    shader_mut(&mut nif).set_shader_type(LightingShaderType::HairTint);
    let mut out = Cursor::new(Vec::new());
    nif.save(&mut out, None).unwrap();
    assert!(NifFile::load(out.into_inner().as_slice(), None).is_err());
    assert_eq!(
        shader(&nif).shader_type(),
        Some(LightingShaderType::HairTint)
    );

    // A named shader saves none of its fields, so it isn't affected.
    let mut nif = load(&format!("{UPSTREAM}/TestNifFile_FO76.nif"));
    shader_mut(&mut nif).set_shader_type(LightingShaderType::Eye);
    assert_eq!(
        shader(&reload(&mut nif)).shader_type(),
        Some(LightingShaderType::Default)
    );
}

#[test]
fn fallout_76_reads_face_tint() {
    // Default (0) and GlowMap (2) save without type fields, so the files differ only in the
    // stored type. Storing 3 there gives Fallout 76's FaceTint.
    let bytes = |t| {
        let mut nif = fallout_76();
        shader_mut(&mut nif).set_shader_type(t);
        let mut out = Cursor::new(Vec::new());
        nif.save(&mut out, None).unwrap();
        out.into_inner()
    };
    let (a, mut b) = (
        bytes(LightingShaderType::Default),
        bytes(LightingShaderType::GlowMap),
    );
    let diff: Vec<usize> = (0..a.len()).filter(|&i| a[i] != b[i]).collect();
    assert_eq!((a.len(), diff.len()), (b.len(), 1));
    b[diff[0]] = 3;
    let nif = NifFile::load(b.as_slice(), None).unwrap();
    assert_eq!(
        shader(&nif).shader_type(),
        Some(LightingShaderType::FaceTint)
    );
}

#[test]
fn named_starfield_shader_saves_no_fields() {
    let mut nif = load(&format!("{UPSTREAM}/TestNifFile_SF.nif"));
    let before = shader(&nif).glossiness();
    shader_mut(&mut nif).set_glossiness(before + 1.0);
    assert_eq!(shader(&reload(&mut nif)).glossiness(), before);
}
