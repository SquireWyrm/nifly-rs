//! Tests for NifFile::material_property.

use crate::common::{load, upstream};
use std::ptr;

use nifly::shaders::NiMaterialProperty;

#[test]
fn found_in_older_files() {
    for (file, expected) in [
        ("TestNifFile_Static_MW", [Some(3), None].as_slice()), // the second shape has no properties
        ("TestNifFile_Skinned_MW", &[Some(9)]),                // after alpha and texturing properties
        ("TestNifFile_Skinned_OB", &[Some(3)]),
    ] {
        let nif = upstream(file);
        let found: Vec<_> = nif.shapes().map(|s| nif.view_of(s).unwrap().material_property().map(|m| m.index())).collect();
        assert_eq!(found, expected, "{file}");
        for shape in nif.shapes() {
            // It's one of the shape's own refs.
            let child = nif.child_blocks_of::<NiMaterialProperty>(shape).next();
            assert_eq!(nif.view_of(shape).unwrap().material_property().map(|m| ptr::from_ref(m.get())), child.map(ptr::from_ref), "{file}");
        }
    }
}

#[test]
fn shader_falls_back_to_it() {
    // These files have no other shader, so the material is also the shader.
    for file in ["TestNifFile_Static_MW", "TestNifFile_Skinned_MW", "TestNifFile_Skinned_OB"] {
        let nif = upstream(file);
        for shape in nif.shapes() {
            let shader = nif.view_of(shape).unwrap().shader().map(|s| ptr::from_ref(s.cast::<NiMaterialProperty>().unwrap().get()));
            assert_eq!(nif.view_of(shape).unwrap().material_property().map(|m| ptr::from_ref(m.get())), shader, "{file}");
        }
    }
}

#[test]
fn newer_files_have_none() {
    for asset in ["Bones", "Cone", "Cube", "Icosphere", "Suzanne"] {
        let nif = load(asset);
        for shape in nif.shapes() {
            assert!(nif.view_of(shape).unwrap().material_property().is_none(), "{asset}");
        }
    }
}
