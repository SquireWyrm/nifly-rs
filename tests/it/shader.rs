//! Tests for NifFile::shader and the shader types.

use crate::common::{load};

use nifly::{
    file::{NifFile, NifVersion},
    shaders::{BsEffectShaderProperty, BsLightingShaderProperty, BsShaderProperty, NiMaterialProperty, NiProperty, NiShader},
};

/// Block index and type of the first shape's shader.
fn shader_of(nif: &NifFile) -> Option<(usize, &str)> {
    let shape = nif.shapes().next().unwrap();
    nif.view_of(shape).unwrap().shader().map(|s| (s.index(), s.get().block_name()))
}

#[test]
fn shaders_of_assets() {
    for (asset, expected) in [
        ("Bones", Some((9, "BSLightingShaderProperty"))), // NiTriShape (Skyrim LE)
        ("Cube", Some((2, "BSLightingShaderProperty"))),
        ("Suzanne", Some((8, "BSLightingShaderProperty"))),
        ("Icosphere", None),
    ] {
        assert_eq!(shader_of(&load(asset)), expected, "{asset}");
    }
}

#[test]
fn unreferenced_shader_isnt_the_shapes() {
    // Cone has a BSLightingShaderProperty block, but its shape doesn't reference it.
    let nif = load("Cone");
    assert_eq!(nif.blocks_of::<NiShader>().count(), 1);
    assert_eq!(shader_of(&nif), None);
}

#[test]
fn shader_hierarchy() {
    let nif = load("Suzanne");
    let shader = nif.shapes().views().next().unwrap().shader().unwrap();

    // Down to the concrete type and back up through Deref.
    let lighting = shader.downcast::<BsLightingShaderProperty>().unwrap();
    let bs: &BsShaderProperty = lighting;
    let property: &NiProperty = bs;
    assert_eq!(property.name(), "");
    assert_eq!(property.block_name(), "BSLightingShaderProperty");

    assert!(shader.downcast::<BsEffectShaderProperty>().is_none());
    assert!(shader.downcast::<NiMaterialProperty>().is_none());

    // The texture set is a block of its own, not a shader or property.
    let texture_set = nif.block(9).unwrap();
    assert_eq!(texture_set.block_name(), "BSShaderTextureSet");
    assert!(texture_set.downcast::<NiProperty>().is_none());
}

#[test]
fn shader_types_in_blocks_of() {
    let nif = load("Suzanne");
    let types: Vec<_> = nif.blocks_of::<NiProperty>().map(|p| p.block_name()).collect();
    assert_eq!(types, ["BSLightingShaderProperty"]);
    assert_eq!(nif.blocks_of::<BsLightingShaderProperty>().count(), 1);
}

#[test]
fn shader_survives_optimize_for() {
    // LE to SE replaces the shape, which keeps its shader ref.
    let mut nif = load("Bones");
    nif.optimize_for(NifVersion::SKYRIM_SE, None).unwrap();
    let (_, ty) = shader_of(&nif).unwrap();
    assert_eq!(ty, "BSLightingShaderProperty");
}
