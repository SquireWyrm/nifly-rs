//! Tests for NifFile::fix_shader_flags and the shader flag getters.

use crate::common::{load};
use std::io::Cursor;

use nifly::{
    file::{NifFile, SaveOptions},
    shaders::BsLightingShaderProperty,
};

const ENVIRONMENT_MAPPING: u32 = 1 << 7; // flags 1
const EYE_ENVIRONMENT_MAPPING: u32 = 1 << 17; // flags 1

/// The file as it is in memory: no optimizing or sorting on save.
fn raw(nif: &mut NifFile) -> Vec<u8> {
    let mut cur = Cursor::new(Vec::new());
    let options = SaveOptions { optimize: false, sort_blocks: false };
    nif.save(&mut cur, Some(options)).unwrap();
    cur.into_inner()
}

fn lighting(nif: &NifFile) -> &BsLightingShaderProperty {
    nif.blocks_of::<BsLightingShaderProperty>().next().unwrap()
}

#[test]
fn consistent_shaders_are_unchanged() {
    // Every asset's shader is a default shader without either environment flag.
    for asset in ["Bones", "Cone", "Cube", "Suzanne"] {
        let mut nif = load(asset);
        let f1 = lighting(&nif).raw_flags_1();
        assert_eq!(f1 & (ENVIRONMENT_MAPPING | EYE_ENVIRONMENT_MAPPING), 0, "{asset}");

        let before = raw(&mut nif);
        nif.fix_shader_flags();
        assert_eq!(raw(&mut nif), before, "{asset}");
    }
}

#[test]
fn no_shaders_does_nothing() {
    let mut nif = load("Icosphere");
    let before = raw(&mut nif);
    nif.fix_shader_flags();
    assert_eq!(raw(&mut nif), before);
}
