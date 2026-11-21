//! Tests for NifFile::fix_shader_flags and the shader flag getters.

use crate::common::{first, load, save_in_place};

use nifly::{file::NifFile, shaders::BsLightingShaderProperty};

const ENVIRONMENT_MAPPING: u32 = 1 << 7; // flags 1
const EYE_ENVIRONMENT_MAPPING: u32 = 1 << 17; // flags 1

fn lighting(nif: &NifFile) -> &BsLightingShaderProperty {
    first::<BsLightingShaderProperty>(nif)
}

#[test]
fn consistent_shaders_are_unchanged() {
    // Every asset's shader is a default shader without either environment flag.
    for asset in ["Bones", "Cone", "Cube", "Suzanne"] {
        let mut nif = load(asset);
        let f1 = lighting(&nif).raw_flags_1();
        assert_eq!(
            f1 & (ENVIRONMENT_MAPPING | EYE_ENVIRONMENT_MAPPING),
            0,
            "{asset}"
        );

        let before = save_in_place(&mut nif);
        nif.fix_shader_flags();
        assert_eq!(save_in_place(&mut nif), before, "{asset}");
    }
}

#[test]
fn no_shaders_does_nothing() {
    let mut nif = load("Icosphere");
    let before = save_in_place(&mut nif);
    nif.fix_shader_flags();
    assert_eq!(save_in_place(&mut nif), before);
}
