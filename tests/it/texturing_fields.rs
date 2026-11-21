//! Tests for NiTexturingProperty's fields: flags, map descriptors, bump and parallax settings
//! and shader maps.

use nifly::{
    file::{Index, NifFile, NifVersion},
    geometry::NiShape,
    object3d::Vec3,
    shaders::{ClampMode, MapSlot, NiTexturingProperty, TextureMap, TextureTransform},
};

use crate::common::{first_index, reload_in_place, upstream};

const TRANSFORM: TextureTransform = TextureTransform {
    translation: [0.5, 0.25],
    scale: [2.0, 3.0],
    rotation: 1.5,
    transform_type: 1,
    center: [0.5, 0.5],
};

fn first(nif: &NifFile) -> Index {
    first_index::<NiTexturingProperty>(nif)
}

fn property(nif: &NifFile, index: Index) -> &NiTexturingProperty {
    nif.block_of::<NiTexturingProperty>(index).unwrap()
}

/// A new Fallout 3 file with one shape and a texturing property with a base map.
fn fallout_3() -> (NifFile, Index) {
    let mut nif = NifFile::create(NifVersion::FALLOUT_3);
    let shape = nif
        .create_shape(
            "Tri",
            &[Vec3::ZERO, Vec3::X, Vec3::Y],
            &[[0, 1, 2]],
            None,
            None,
        )
        .unwrap();
    let index = nif
        .edit::<NiShape>(shape)
        .unwrap()
        .add_property::<NiTexturingProperty, _>();
    let base = nif.add_source_texture(r"textures\base.dds");
    assert!(
        nif.edit::<NiTexturingProperty>(index)
            .unwrap()
            .set_map_source(TextureMap::Base, Some(base))
    );
    (nif, index)
}

#[test]
fn morrowind_map_fields() {
    let mut nif = upstream("TestNifFile_Skinned_MW");
    let index = first(&nif);
    assert!(property(&nif, index).has_map(TextureMap::Base));
    let p = nif.block_of_mut::<NiTexturingProperty>(index).unwrap();
    p.set_raw_flags(3);
    assert!(p.set_map_ps2(TextureMap::Base, [-3, 5]));
    assert!(p.set_map_unknown(TextureMap::Base, 9));
    assert!(p.set_map_transform(TextureMap::Base, Some(TRANSFORM)));
    assert!(!p.set_map_unknown(TextureMap::Glow, 9), "no glow map");
    assert_eq!(p.map_unknown(TextureMap::Glow), None);

    let saved = reload_in_place(&mut nif);
    let p = property(&saved, index);
    assert_eq!(p.raw_flags(), 3);
    assert_eq!(
        (p.map_ps2(TextureMap::Base), p.map_unknown(TextureMap::Base)),
        (Some([-3, 5]), Some(9))
    );
    assert_eq!(
        p.map_transform(TextureMap::Base),
        None,
        "not saved in Morrowind"
    );
}

#[test]
fn oblivion_map_fields() {
    let mut nif = upstream("TestNifFile_Skinned_OB");
    let index = first(&nif);
    let bump = nif.add_source_texture(r"textures\bump.dds");
    assert!(
        nif.edit::<NiTexturingProperty>(index)
            .unwrap()
            .set_map_source(TextureMap::Bump, Some(bump))
    );
    let p = nif.block_of_mut::<NiTexturingProperty>(index).unwrap();
    assert!(p.set_map_transform(TextureMap::Base, Some(TRANSFORM)));
    assert!(p.set_map_clamp_mode(TextureMap::Bump, ClampMode::ClampBoth));
    p.set_bump_luma_scale(0.5);
    p.set_bump_luma_offset(0.25);
    p.set_bump_matrix([1.0, 2.0, 3.0, 4.0]);
    assert!(p.set_map_ps2(TextureMap::Base, [-3, 5]));

    let saved = reload_in_place(&mut nif);
    let p = property(&saved, index);
    assert_eq!(p.map_transform(TextureMap::Base), Some(TRANSFORM));
    assert_eq!(
        p.map_clamp_mode(TextureMap::Bump),
        Some(ClampMode::ClampBoth)
    );
    assert_eq!(
        (p.bump_luma_scale(), p.bump_luma_offset(), p.bump_matrix()),
        (0.5, 0.25, [1.0, 2.0, 3.0, 4.0])
    );
    assert_eq!(
        p.map_ps2(TextureMap::Base),
        Some([0, -75]),
        "not saved in Oblivion"
    );

    let mut nif = saved;
    assert!(
        nif.block_of_mut::<NiTexturingProperty>(index)
            .unwrap()
            .set_map_transform(TextureMap::Base, None)
    );
    assert_eq!(
        property(&reload_in_place(&mut nif), index).map_transform(TextureMap::Base),
        None
    );
}

#[test]
fn fallout_3_fields() {
    let (mut nif, index) = fallout_3();
    let parallax = nif.add_source_texture(r"textures\parallax.dds");
    assert!(
        nif.edit::<NiTexturingProperty>(index)
            .unwrap()
            .set_map_source(TextureMap::Parallax, Some(parallax))
    );
    let p = nif.block_of_mut::<NiTexturingProperty>(index).unwrap();
    p.set_raw_flags(0x5);
    p.set_parallax_offset(0.125);
    assert!(p.set_map_raw_flags(TextureMap::Base, 0x3201));
    assert!(p.set_map_max_anisotropy(TextureMap::Base, 4));
    p.set_map_uv_set(TextureMap::Base, 3);

    let saved = reload_in_place(&mut nif);
    let p = property(&saved, index);
    assert_eq!((p.raw_flags(), p.parallax_offset()), (0x5, 0.125));
    assert_eq!(p.map_raw_flags(TextureMap::Base), Some(0x3201));
    assert_eq!(
        p.map_uv_set(TextureMap::Base),
        Some(0),
        "kept in the flags from Fallout 3 on"
    );
    assert_eq!(
        p.map_max_anisotropy(TextureMap::Base),
        Some(0),
        "not saved in Fallout 3"
    );
    assert_eq!(p.texture_count(), 8);

    let mut nif = saved;
    nif.block_of_mut::<NiTexturingProperty>(index)
        .unwrap()
        .set_texture_count(7);
    let saved = reload_in_place(&mut nif);
    assert!(
        !property(&saved, index).has_map(TextureMap::Parallax),
        "past the count"
    );
    assert!(property(&saved, index).has_map(TextureMap::Base));
}

#[test]
fn shader_maps() {
    let mut nif = upstream("TestNifFile_Skinned_OB");
    let index = first(&nif);
    let source = nif.add_source_texture(r"textures\shader.dds");
    nif.block_of_mut::<NiTexturingProperty>(index)
        .unwrap()
        .set_shader_map_count(2);
    let mut edit = nif.edit::<NiTexturingProperty>(index).unwrap();
    assert!(
        !edit.set_map_source(MapSlot::Shader(2), Some(source)),
        "out of range"
    );
    assert!(
        !edit.set_map_source(MapSlot::Shader(1), Some(0)),
        "not a source texture"
    );
    assert!(edit.set_map_source(MapSlot::Shader(1), Some(source)));

    let p = nif.block_of_mut::<NiTexturingProperty>(index).unwrap();
    assert!(!p.set_shader_map_id(0, 5), "unused");
    assert!(p.set_shader_map_id(1, 5));
    assert!(p.set_map_uv_set(MapSlot::Shader(1), 2));

    let saved = reload_in_place(&mut nif);
    let p = property(&saved, index);
    assert_eq!(p.shader_map_count(), 2);
    assert!(!p.has_map(MapSlot::Shader(0)));
    assert_eq!(
        (p.shader_map_id(1), p.map_uv_set(MapSlot::Shader(1))),
        (Some(5), Some(2))
    );
    let view = saved.view::<NiTexturingProperty>(index).unwrap();
    assert_eq!(
        view.map_source(MapSlot::Shader(1)).unwrap().file_name(),
        r"textures\shader.dds"
    );

    let mut nif = saved;
    assert!(
        nif.edit::<NiTexturingProperty>(index)
            .unwrap()
            .set_map_source(MapSlot::Shader(1), None)
    );
    let saved = reload_in_place(&mut nif);
    assert!(!property(&saved, index).has_map(MapSlot::Shader(1)));
    assert_eq!(property(&saved, index).shader_map_id(1), None);
}
