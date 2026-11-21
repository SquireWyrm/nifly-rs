//! Tests for NifFile::optimize_for.

use crate::common::{first, open, reload, save};
use std::fs;

use nifly::{
    file::{NifFile, NifVersion, OptimizeOptions, OptimizeReport},
    geometry::NiShape,
};

const UPSTREAM: &str = "nifly-sys/vendor/nifly/tests";

fn load_path(path: &str) -> NifFile {
    open(path)
}

fn shape_types(nif: &NifFile) -> Vec<String> {
    nif.blocks_of::<NiShape>()
        .map(|s| s.block_name().to_string())
        .collect()
}

/// Mirrors nifly's own tests: optimize, save and compare with its expected output.
fn upstream_case(name: &str, target: NifVersion, head_parts: bool) {
    let mut nif = load_path(&format!("{UPSTREAM}/input/{name}.nif"));
    let options = OptimizeOptions {
        head_parts,
        ..Default::default()
    };
    assert!(nif.optimize_for(target, Some(options)).is_some());
    assert_eq!(nif.version(), target);
    let expected = fs::read(format!("{UPSTREAM}/expected/{name}.nif")).unwrap();
    assert!(
        save(&mut nif) == expected,
        "{name} differs from nifly's expected output"
    );
}

#[test]
fn upstream_le_to_se() {
    upstream_case(
        "TestNifFile_Optimize_LE_to_SE",
        NifVersion::SKYRIM_SE,
        false,
    );
}

#[test]
fn upstream_le_to_se_head_parts() {
    upstream_case(
        "TestNifFile_Optimize_Dynamic_LE_to_SE",
        NifVersion::SKYRIM_SE,
        true,
    );
}

#[test]
fn upstream_se_to_le() {
    upstream_case("TestNifFile_Optimize_SE_to_LE", NifVersion::SKYRIM, false);
}

#[test]
fn upstream_se_to_le_head_parts() {
    upstream_case(
        "TestNifFile_Optimize_Dynamic_SE_to_LE",
        NifVersion::SKYRIM,
        true,
    );
}

#[test]
fn bones_le_to_se_keeps_skinning() {
    let mut nif = load_path("assets/Bones.nif");
    assert_eq!(nif.version(), NifVersion::SKYRIM);
    let shape = first::<NiShape>(&nif);
    let bones = nif.view_of(shape).unwrap().bone_list();
    let weights: Vec<_> = (0..bones.len())
        .map(|i| nif.view_of(shape).unwrap().bone_weights(i).len())
        .collect();
    assert_eq!(shape_types(&nif), ["NiTriShape"]);

    // Nothing to strip or fix: no vertex colors, parallax or strips, and it already has tangents.
    let report = nif.optimize_for(NifVersion::SKYRIM_SE, None).unwrap();
    assert_eq!(report, OptimizeReport::default());
    assert_eq!(nif.version(), NifVersion::SKYRIM_SE);
    assert_eq!(shape_types(&nif), ["BSTriShape"]);

    // Still skinned to the same bones after a save and reload.
    let saved = reload(&mut nif);
    assert_eq!(saved.version(), NifVersion::SKYRIM_SE);
    let shape = first::<NiShape>(&saved);
    assert_eq!(saved.view_of(shape).unwrap().bone_list(), bones);
    let after: Vec<_> = (0..bones.len())
        .map(|i| saved.view_of(shape).unwrap().bone_weights(i).len())
        .collect();
    assert_eq!(after, weights);
    assert!(saved.is_all_sse_compatible());
}

#[test]
fn round_trip_le_se_le() {
    let mut nif = load_path("assets/Bones.nif");
    nif.optimize_for(NifVersion::SKYRIM_SE, None).unwrap();
    nif.optimize_for(NifVersion::SKYRIM, None).unwrap();
    assert_eq!(nif.version(), NifVersion::SKYRIM);
    assert_eq!(shape_types(&nif), ["NiTriShape"]);
}

#[test]
fn unsupported_conversion_changes_nothing() {
    for (asset, target) in [
        ("Suzanne", NifVersion::SKYRIM_SE), // SE to SE
        ("Bones", NifVersion::SKYRIM),      // LE to LE
        ("Suzanne", NifVersion::FALLOUT_4), // SE to FO4
    ] {
        let mut nif = load_path(&format!("assets/{asset}.nif"));
        let before = save(&mut nif);
        assert!(
            nif.optimize_for(target, None).is_none(),
            "{asset} -> {target:?}"
        );
        assert_eq!(save(&mut nif), before, "{asset} -> {target:?}");
    }
}
