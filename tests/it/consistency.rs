//! Tests for NiGeometryData::consistency and NiGeometryData::set_consistency.

use crate::common::{UPSTREAM, open, reload};

use nifly::{
    file::NifFile,
    geometry::{Consistency, NiGeometryData},
};

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
        data(&open("assets/Bones.nif")).consistency(),
        Some(Consistency::Mutable)
    );
    assert_eq!(
        data(&open(format!("{UPSTREAM}/TestNifFile_Skinned_OB.nif"))).consistency(),
        Some(Consistency::Static)
    );
}

#[test]
fn set_round_trip() {
    let mut nif = open(format!("{UPSTREAM}/TestNifFile_Skinned_OB.nif"));
    set(&mut nif, Consistency::Mutable);
    assert_eq!(data(&nif).consistency(), Some(Consistency::Mutable));
    assert_eq!(
        data(&reload(&mut nif)).consistency(),
        Some(Consistency::Mutable)
    );

    let mut nif = open("assets/Bones.nif");
    for c in [Consistency::Volatile, Consistency::Static] {
        set(&mut nif, c);
        assert_eq!(data(&reload(&mut nif)).consistency(), Some(c));
    }
}

#[test]
fn morrowind_doesnt_store_it() {
    let mut nif = open("assets/Stencil_MW.nif");
    set(&mut nif, Consistency::Static);
    assert_eq!(data(&nif).consistency(), Some(Consistency::Static));
    assert_eq!(
        data(&reload(&mut nif)).consistency(),
        Some(Consistency::Mutable)
    );
}
