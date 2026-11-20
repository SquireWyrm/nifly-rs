//! Tests for NifFile::fix_bsx_flags.

use crate::common::{load};
use std::io::Cursor;

use nifly::{
    extra_data::{BsxBitflags, BsxFlags, ExtraBsxFlags},
    file::{NifFile, NifVersion, OptimizeOptions, SaveOptions},
};

const EMITTANCE: u32 = 1 << 9;

/// The file as it is in memory: no optimizing or sorting on save.
fn raw(nif: &mut NifFile) -> Vec<u8> {
    let mut cur = Cursor::new(Vec::new());
    let options = SaveOptions { optimize: false, sort_blocks: false };
    nif.save(&mut cur, Some(options)).unwrap();
    cur.into_inner()
}

/// Values of every BSXFlags block, in block order.
fn bsx_values(nif: &NifFile) -> Vec<u32> {
    nif.blocks_of::<BsxFlags>().map(|b| b.value()).collect()
}

/// A new file whose root has a BSX block with `flags`.
fn with_bsx(version: NifVersion, flags: u32) -> NifFile {
    let mut nif = NifFile::create(version);
    let root = nif.root_node().unwrap().index();
    nif.edit::<nifly::objects::NiAvObject>(root).and_then(|mut e| e.assign_extra_data(ExtraBsxFlags(BsxBitflags::from_bits_retain(flags)))).unwrap();
    nif
}

#[test]
fn consistent_flags_are_unchanged() {
    // Suzanne's BSX is 130 (havok | articulated) and its shader has no external emittance.
    let mut nif = load("Suzanne");
    let before = raw(&mut nif);
    nif.fix_bsx_flags();
    assert_eq!(bsx_values(&nif), [130]);
    assert_eq!(raw(&mut nif), before);
}

#[test]
fn unnecessary_emittance_is_cleared() {
    // No shaders at all, so the flag isn't needed. Other bits stay.
    let mut nif = with_bsx(NifVersion::SKYRIM_SE, 0x3ff);
    nif.fix_bsx_flags();
    assert_eq!(bsx_values(&nif), [0x3ff & !EMITTANCE]);
}

#[test]
fn no_bsx_block_does_nothing() {
    for asset in ["Bones", "Cone", "Cube", "Icosphere"] {
        let mut nif = load(asset);
        let before = raw(&mut nif);
        nif.fix_bsx_flags();
        assert_eq!(raw(&mut nif), before, "{asset}");
    }
}

#[test]
fn only_the_first_bsx_block_is_fixed() {
    let mut nif = with_bsx(NifVersion::SKYRIM_SE, EMITTANCE | 2);
    let root = nif.root_node().unwrap().index();
    nif.edit::<nifly::objects::NiAvObject>(root).and_then(|mut e| e.assign_extra_data(ExtraBsxFlags(BsxBitflags::EXTERNAL_EMITTANCE | BsxBitflags::RAGDOLL))).unwrap();

    nif.fix_bsx_flags();
    assert_eq!(bsx_values(&nif), [2, EMITTANCE | 4]);
}

#[test]
fn optimize_for_runs_it_by_default() {
    let mut nif = with_bsx(NifVersion::SKYRIM, EMITTANCE | 2);
    nif.optimize_for(NifVersion::SKYRIM_SE, None).unwrap();
    assert_eq!(bsx_values(&nif), [2]);

    let mut nif = with_bsx(NifVersion::SKYRIM, EMITTANCE | 2);
    let options = OptimizeOptions { fix_bsx_flags: false, ..Default::default() };
    nif.optimize_for(NifVersion::SKYRIM_SE, Some(options)).unwrap();
    assert_eq!(bsx_values(&nif), [EMITTANCE | 2]);
}
