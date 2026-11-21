//! Tests for the NiShape subclasses.

use crate::common::{UPSTREAM, first, load};
use std::fs;

use nifly::{
    file::{NifFile, NifVersion, OptimizeOptions},
    geometry::*,
    objects::NiObject,
};

fn is<T: nifly::objects::NiType>(obj: &NiObject) -> bool {
    obj.downcast::<T>().is_some()
}

/// Whether `obj` downcasts to the type named `block_name`, or `None` for a name the table lacks.
fn downcasts_to_own_type(block_name: &str, obj: &NiObject) -> Option<bool> {
    Some(match block_name {
        "BSTriShape" => is::<BsTriShape>(obj),
        "BSSubIndexTriShape" => is::<BsSubIndexTriShape>(obj),
        "BSMeshLODTriShape" => is::<BsMeshLodTriShape>(obj),
        "BSDynamicTriShape" => is::<BsDynamicTriShape>(obj),
        "BSGeometry" => is::<BsGeometry>(obj),
        "NiTriShape" => is::<NiTriShape>(obj),
        "NiTriStrips" => is::<NiTriStrips>(obj),
        "NiLines" => is::<NiLines>(obj),
        "BSLODTriShape" => is::<BsLodTriShape>(obj),
        "NiScreenElements" => is::<NiScreenElements>(obj),
        "BSSegmentedTriShape" => is::<BsSegmentedTriShape>(obj),
        "NiParticles" => is::<NiParticles>(obj),
        "NiAutoNormalParticles" => is::<NiAutoNormalParticles>(obj),
        "NiParticleMeshes" => is::<NiParticleMeshes>(obj),
        "NiRotatingParticles" => is::<NiRotatingParticles>(obj),
        _ => return None,
    })
}

#[test]
fn deref_chain() {
    let nif = load("Bones");
    let tri = first::<NiTriShape>(&nif);
    let based: &NiTriBasedGeom = tri;
    let geom: &NiGeometry = based;
    let shape: &NiShape = geom;
    assert_eq!(tri.name(), "Limb");
    assert_eq!(shape.transform(), tri.transform());
    // A subclass view reaches the shape methods by upcasting.
    let tri = nif.view_of(tri).unwrap().upcast::<NiShape, _>();
    assert_eq!(tri.bone_list(), ["Bone1", "Bone2", "Bone3"]);
}

#[test]
fn conversion_changes_the_type() {
    let mut nif = load("Bones");
    nif.optimize_for(NifVersion::SKYRIM_SE, None).unwrap();
    let shape: &NiObject = nif.shapes().next().unwrap();
    assert!(is::<BsTriShape>(shape));
    assert!(!is::<BsDynamicTriShape>(shape));
    assert!(!is::<NiGeometry>(shape));

    let mut nif = load("Bones");
    let options = OptimizeOptions {
        head_parts: true,
        ..Default::default()
    };
    nif.optimize_for(NifVersion::SKYRIM_SE, Some(options))
        .unwrap();
    let shape: &NiObject = nif.shapes().next().unwrap();
    assert!(is::<BsDynamicTriShape>(shape));
    assert!(is::<BsTriShape>(shape));
}

#[test]
fn every_shape_in_upstream_files_downcasts_to_its_type() {
    let mut checked = std::collections::BTreeSet::new();
    for entry in fs::read_dir(UPSTREAM).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_none_or(|e| e != "nif") {
            continue;
        }
        // One input is deliberately corrupt.
        let Ok(nif) = NifFile::load(fs::read(&path).unwrap().as_slice(), None) else {
            continue;
        };
        for shape in nif.shapes() {
            let name = shape.block_name();
            let ok = downcasts_to_own_type(name, shape);
            assert_eq!(ok, Some(true), "{name} in {}", path.display());
            checked.insert(name.to_string());
        }
    }
    // These files cover Fallout 4, Starfield, Morrowind particles and dynamic SE skins.
    for name in [
        "BSDynamicTriShape",
        "BSGeometry",
        "BSSubIndexTriShape",
        "BSTriShape",
        "NiAutoNormalParticles",
        "NiRotatingParticles",
        "NiTriShape",
    ] {
        assert!(checked.contains(name), "no {name} in {UPSTREAM}");
    }
}
