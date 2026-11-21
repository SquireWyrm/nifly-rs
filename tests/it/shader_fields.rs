//! Tests for shader flag lists, legacy flags, fields of unknown meaning and texture set slots.

use nifly::{
    file::{Index, NifFile, NifVersion},
    geometry::NiShape,
    object3d::Vec3,
    objects::NiObjectNet,
    shaders::{
        BsEffectShaderProperty, BsLightingShaderProperty, BsShaderProperty, BsShaderTextureSet,
        NiMaterialProperty, NiShadeProperty, NiStencilProperty,
    },
};

use crate::common::{first_index, reload_in_place, upstream};

/// A new file of `version` with one triangle shape.
fn triangle(version: NifVersion) -> (NifFile, Index) {
    let mut nif = NifFile::create(version);
    let shape = nif
        .create_shape(
            "Tri",
            &[Vec3::ZERO, Vec3::X, Vec3::Y],
            &[[0, 1, 2]],
            None,
            None,
        )
        .unwrap();
    (nif, shape)
}

#[test]
fn crc_flag_lists() {
    // TestNifFile_FO76.nif: block 231 is an effect shader without a name.
    let mut nif = upstream("TestNifFile_FO76");
    let shader = nif.block_of::<BsShaderProperty>(231).unwrap();
    assert_eq!(
        (shader.crc_flags_1().len(), shader.crc_flags_2().len()),
        (4, 1)
    );
    let flags_1 = shader.raw_flags_1();

    let shader = nif.block_of_mut::<BsShaderProperty>(231).unwrap();
    shader.set_crc_flags_1(&[1, 2, 3, 4, 5, 6]);
    shader.set_crc_flags_2(&[]);
    shader.set_raw_flags_1(flags_1 ^ 1);
    let saved = reload_in_place(&mut nif);
    let shader = saved.block_of::<BsShaderProperty>(231).unwrap();
    assert_eq!(
        (shader.crc_flags_1(), shader.crc_flags_2()),
        (&[1, 2, 3, 4, 5, 6][..], &[][..])
    );
    assert_eq!(shader.raw_flags_1(), flags_1, "not saved in Fallout 76");
}

#[test]
fn shading_raw_flags() {
    let (mut nif, shape) = triangle(NifVersion::FALLOUT_3);
    let shader = nif
        .view::<NiShape>(shape)
        .unwrap()
        .shader()
        .unwrap()
        .index();
    assert_eq!(
        nif.block_of::<NiShadeProperty>(shader)
            .unwrap()
            .shading_raw_flags(),
        1
    );
    nif.block_of_mut::<NiShadeProperty>(shader)
        .unwrap()
        .set_shading_raw_flags(0);
    assert_eq!(
        reload_in_place(&mut nif)
            .block_of::<NiShadeProperty>(shader)
            .unwrap()
            .shading_raw_flags(),
        0
    );

    let (mut nif, shape) = triangle(NifVersion::SKYRIM_SE);
    let shader = nif
        .view::<NiShape>(shape)
        .unwrap()
        .shader()
        .unwrap()
        .index();
    nif.block_of_mut::<NiShadeProperty>(shader)
        .unwrap()
        .set_shading_raw_flags(0);
    assert_eq!(
        reload_in_place(&mut nif)
            .block_of::<NiShadeProperty>(shader)
            .unwrap()
            .shading_raw_flags(),
        1,
        "not saved"
    );
}

#[test]
fn legacy_raw_flags() {
    let mut nif = upstream("TestNifFile_Static_MW");
    let material = first_index::<NiMaterialProperty>(&nif);
    let shape = nif.shapes().indices().next().unwrap();
    let stencil = nif
        .edit::<NiShape>(shape)
        .unwrap()
        .add_property::<NiStencilProperty, _>();
    nif.block_of_mut::<NiMaterialProperty>(material)
        .unwrap()
        .set_legacy_raw_flags(5);
    nif.block_of_mut::<NiStencilProperty>(stencil)
        .unwrap()
        .set_legacy_raw_flags(7);
    let saved = reload_in_place(&mut nif);
    assert_eq!(
        saved
            .block_of::<NiMaterialProperty>(material)
            .unwrap()
            .legacy_raw_flags(),
        5
    );
    assert_eq!(
        saved
            .block_of::<NiStencilProperty>(stencil)
            .unwrap()
            .legacy_raw_flags(),
        7
    );
}

#[test]
fn lighting_unknowns() {
    // The Starfield shader has a name, which saves no fields: clear it.
    let mut nif = upstream("TestNifFile_SF");
    let index = first_index::<BsLightingShaderProperty>(&nif);
    nif.block_of_mut::<NiObjectNet>(index).unwrap().set_name("");
    let s = nif.block_of_mut::<BsLightingShaderProperty>(index).unwrap();
    s.set_wetness_unknown_1(1.5);
    s.set_wetness_unknown_2(2.5);
    s.set_unknown_float(3.5);
    s.set_unknown_float_1(4.5);
    s.set_unknown_float_2(5.5);
    s.set_unknown_short(6);
    let saved = reload_in_place(&mut nif);
    let s = saved.block_of::<BsLightingShaderProperty>(index).unwrap();
    assert_eq!((s.wetness_unknown_1(), s.wetness_unknown_2()), (1.5, 2.5));
    assert_eq!(
        (s.unknown_float(), s.unknown_float_1(), s.unknown_float_2()),
        (3.5, 4.5, 5.5)
    );
    assert_eq!(s.unknown_short(), 6);
}

#[test]
fn effect_unknowns() {
    // No Starfield test file has an effect shader without a name; Fallout 76 doesn't save these.
    let mut nif = upstream("TestNifFile_FO76");
    let s = nif.block_of_mut::<BsEffectShaderProperty>(231).unwrap();
    s.set_unknown_float(1.5);
    s.set_unknown_bytes([1, 2, 3, 4, 5, 6, 7]);
    s.set_unknown_floats([1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    s.set_unknown_byte(9);
    s.set_lighting_influence(11);
    s.set_clamp_unknown(12);
    let s = nif.block_of::<BsEffectShaderProperty>(231).unwrap();
    assert_eq!(
        (s.unknown_float(), s.unknown_bytes(), s.unknown_byte()),
        (1.5, [1, 2, 3, 4, 5, 6, 7], 9)
    );
    assert_eq!(s.unknown_floats(), [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

    let saved = reload_in_place(&mut nif);
    let s = saved.block_of::<BsEffectShaderProperty>(231).unwrap();
    assert_eq!(
        (s.unknown_float(), s.unknown_bytes(), s.unknown_byte()),
        (0.0, [0; 7], 0)
    );
    assert_eq!((s.lighting_influence(), s.clamp_unknown()), (11, 12));
}

#[test]
fn texture_set_slots() {
    let mut nif = upstream("TestNifFile_Static_SE");
    let index = first_index::<BsShaderTextureSet>(&nif);
    let set = nif.block_of_mut::<BsShaderTextureSet>(index).unwrap();
    assert!(!set.set_path(9, "x.dds"), "Skyrim sets have 9 slots");
    assert!(set.set_path(1, "textures\\n.dds"));
    let saved = reload_in_place(&mut nif);
    assert_eq!(
        saved
            .block_of::<BsShaderTextureSet>(index)
            .unwrap()
            .path(1)
            .unwrap(),
        "textures\\n.dds"
    );

    let mut nif = saved;
    let paths = [r"textures\a.dds", "", r"textures\c.dds"];
    nif.block_of_mut::<BsShaderTextureSet>(index)
        .unwrap()
        .set_paths(paths);
    let saved = reload_in_place(&mut nif);
    assert!(
        saved
            .block_of::<BsShaderTextureSet>(index)
            .unwrap()
            .paths()
            .eq(paths)
    );
}
