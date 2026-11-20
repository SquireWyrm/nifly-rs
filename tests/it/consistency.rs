//! Tests for NiGeometryData::consistency and NiGeometryData::set_consistency.

use crate::common::{UPSTREAM, reload};
use std::fs;

use nifly::{
    file::NifFile,
    geometry::{Consistency, NiGeometryData},
};

fn load(path: &str) -> NifFile {
    NifFile::load(fs::read(path).unwrap().as_slice(), None).unwrap()
}

fn data(nif: &NifFile) -> &NiGeometryData {
    nif.shapes()
        .views()
        .next()
        .unwrap()
        .geometry_data()
        .unwrap()
}

fn set(nif: &mut NifFile, consistency: Consistency) {
    let shape = nif.shapes().indices().next().unwrap();
    nif.edit::<nifly::geometry::NiShape>(shape)
        .unwrap()
        .geometry_data()
        .unwrap()
        .set_consistency(consistency);
}

#[test]
fn reads_consistency() {
    assert_eq!(
        data(&load("assets/Bones.nif")).consistency(),
        Some(Consistency::Mutable)
    );
    assert_eq!(
        data(&load(&format!("{UPSTREAM}/TestNifFile_Skinned_OB.nif"))).consistency(),
        Some(Consistency::Static)
    );
}

#[test]
fn set_round_trip() {
    let mut nif = load(&format!("{UPSTREAM}/TestNifFile_Skinned_OB.nif"));
    set(&mut nif, Consistency::Mutable);
    assert_eq!(data(&nif).consistency(), Some(Consistency::Mutable));
    assert_eq!(
        data(&reload(&mut nif)).consistency(),
        Some(Consistency::Mutable)
    );

    let mut nif = load("assets/Bones.nif");
    for c in [Consistency::Volatile, Consistency::Static] {
        set(&mut nif, c);
        assert_eq!(data(&reload(&mut nif)).consistency(), Some(c));
    }
}

#[test]
fn morrowind_doesnt_store_it() {
    let mut nif = load("assets/Stencil_MW.nif");
    set(&mut nif, Consistency::Static);
    assert_eq!(data(&nif).consistency(), Some(Consistency::Static));
    assert_eq!(
        data(&reload(&mut nif)).consistency(),
        Some(Consistency::Mutable)
    );
}
