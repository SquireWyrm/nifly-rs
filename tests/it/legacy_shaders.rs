//! Tests for the Fallout 3 and New Vegas shader fields, and BSWaterShaderProperty and
//! BSSkyShaderProperty.

use crate::common::reload;

use nifly::{
    file::{Index, NifFile, NifVersion},
    objects::NiType,
    shaders::{
        BsShaderNoLightingProperty, BsShaderPpLightingProperty, BsSkyShaderProperty,
        BsWaterShaderProperty, ClampMode, LegacyShaderType, NewProperty, NiShader, SkyObjectType,
        SkyShaderProperty, TallGrassShaderProperty, TileShaderProperty, WaterShaderFlags,
    },
};

fn root_index(nif: &NifFile) -> Index {
    nif.root_node().unwrap().index()
}

fn first<T: NiType>(nif: &NifFile) -> &T {
    nif.blocks_of::<T>().next().unwrap()
}

fn add<T: NewProperty>(nif: &mut NifFile) -> &mut T {
    let i = nif
        .edit::<nifly::objects::NiAvObject>(root_index(nif))
        .map(|mut e| e.add_property::<T, _>())
        .unwrap();
    nif.block_of_mut::<T>(i).unwrap()
}

#[test]
fn fallout_3_round_trip() {
    let mut nif = NifFile::create(NifVersion::FALLOUT_3);

    let pp = add::<BsShaderPpLightingProperty>(&mut nif);
    pp.set_refraction_strength(0.5);
    pp.set_refraction_fire_period(-3);
    pp.set_parallax_max_passes(8.0);
    pp.set_parallax_scale(0.25);
    pp.set_emissive_color([1.0, 0.5, 0.25, 1.0]);
    let lighting = pp;
    lighting.set_texture_clamp_mode(ClampMode::ClampBoth);
    let base = lighting;
    base.set_legacy_shader_type(LegacyShaderType::Skin);
    base.set_environment_map_scale(2.0);

    let no_lighting = add::<BsShaderNoLightingProperty>(&mut nif);
    no_lighting.set_base_texture(b"textures\\effects\\glow.dds");
    no_lighting.set_falloff_start_angle(0.9);
    no_lighting.set_falloff_stop_angle(0.1);
    no_lighting.set_falloff_start_opacity(0.8);
    no_lighting.set_falloff_stop_opacity(0.2);

    let sky = add::<SkyShaderProperty>(&mut nif);
    sky.set_file_name("textures\\sky\\clouds.dds");
    sky.set_sky_object_type(SkyObjectType::Clouds);
    add::<TileShaderProperty>(&mut nif).set_file_name("tile.dds");
    add::<TallGrassShaderProperty>(&mut nif).set_file_name("grass.dds");

    let saved = reload(&mut nif);
    let pp = first::<BsShaderPpLightingProperty>(&saved);
    assert_eq!(pp.refraction_strength(), 0.5);
    assert_eq!(pp.refraction_fire_period(), -3);
    assert_eq!(pp.parallax_max_passes(), 8.0);
    assert_eq!(pp.parallax_scale(), 0.25);
    // Not saved in Fallout 3.
    assert_eq!(pp.emissive_color(), [0.0; 4]);
    assert_eq!(pp.texture_clamp_mode(), Some(ClampMode::ClampBoth));
    assert_eq!(pp.legacy_shader_type(), Some(LegacyShaderType::Skin));
    assert_eq!(pp.environment_map_scale(), 2.0);

    let no_lighting = first::<BsShaderNoLightingProperty>(&saved);
    assert_eq!(no_lighting.base_texture(), "textures\\effects\\glow.dds");
    assert_eq!(
        [
            no_lighting.falloff_start_angle(),
            no_lighting.falloff_stop_angle(),
            no_lighting.falloff_start_opacity(),
            no_lighting.falloff_stop_opacity()
        ],
        [0.9, 0.1, 0.8, 0.2]
    );

    let sky = first::<SkyShaderProperty>(&saved);
    assert_eq!(sky.file_name(), "textures\\sky\\clouds.dds");
    assert_eq!(sky.sky_object_type(), Some(SkyObjectType::Clouds));
    assert_eq!(first::<TileShaderProperty>(&saved).file_name(), "tile.dds");
    assert_eq!(
        first::<TallGrassShaderProperty>(&saved).file_name(),
        "grass.dds"
    );

    // They're attached to the root's property list, and found as shaders.
    let root = saved.root_node().unwrap();
    assert_eq!(
        saved
            .child_blocks_of::<nifly::shaders::NiProperty>(&root)
            .count(),
        5
    );
    assert_eq!(saved.child_blocks_of::<NiShader>(&root).count(), 5);
}

#[test]
fn skyrim_fields() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    add::<BsWaterShaderProperty>(&mut nif)
        .set_flags(WaterShaderFlags::DISPLACEMENT | WaterShaderFlags::DEPTH);
    let sky = add::<BsSkyShaderProperty>(&mut nif);
    sky.set_base_texture("textures\\sky\\moon.dds");
    sky.set_sky_object_type(SkyObjectType::Stars);
    let pp = add::<BsShaderPpLightingProperty>(&mut nif);
    pp.set_emissive_color([1.0, 0.5, 0.25, 1.0]);
    pp.set_refraction_strength(0.5);
    pp.set_texture_clamp_mode(ClampMode::ClampBoth);

    // Skyrim doesn't save property lists, so the blocks are saved unattached.
    let saved = reload(&mut nif);
    assert_eq!(
        first::<BsWaterShaderProperty>(&saved).flags(),
        WaterShaderFlags::DISPLACEMENT | WaterShaderFlags::DEPTH
    );
    let sky = first::<BsSkyShaderProperty>(&saved);
    assert_eq!(sky.base_texture(), "textures\\sky\\moon.dds");
    assert_eq!(sky.sky_object_type(), Some(SkyObjectType::Stars));

    // The Fallout 3 fields aren't saved; the emissive color is.
    let pp = first::<BsShaderPpLightingProperty>(&saved);
    assert_eq!(pp.emissive_color(), [1.0, 0.5, 0.25, 1.0]);
    assert_eq!(pp.refraction_strength(), 0.0);
    assert_eq!(pp.texture_clamp_mode(), Some(ClampMode::WrapBoth));
}

#[test]
fn defaults() {
    let mut nif = NifFile::create(NifVersion::FALLOUT_3);
    add::<SkyShaderProperty>(&mut nif);
    let sky = first::<SkyShaderProperty>(&nif);
    assert_eq!(sky.legacy_shader_type(), Some(LegacyShaderType::Default));
    assert_eq!(sky.texture_clamp_mode(), Some(ClampMode::WrapBoth));
    assert_eq!(sky.sky_object_type(), Some(SkyObjectType::Texture));
    assert_eq!(sky.file_name(), "");
}
