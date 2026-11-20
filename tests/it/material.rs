//! Tests for NiMaterialProperty's accessors and setters, and NifFile::block_of_mut.

use crate::common::{reload, upstream};

use nifly::{
    file::{Index, NifFile},
    nodes::NiNode,
    objects::NiObject,
    shaders::{NiMaterialProperty, NiShader},
};

fn material(nif: &NifFile) -> (Index, &NiMaterialProperty) {
    nif.blocks_of::<NiMaterialProperty>()
        .indexed()
        .next()
        .unwrap()
}

#[test]
fn reads_stored_values() {
    let teal = [0.34509805, 0.7803922, 0.882353];
    let white = [1.0; 3];
    let black = [0.0; 3];
    // (file, ambient, diffuse, specular, glossiness)
    for (file, ambient, diffuse, specular, glossiness) in [
        ("TestNifFile_Static_MW", teal, teal, teal, 0.0),
        ("TestNifFile_Skinned_MW", white, white, black, 0.0),
        ("TestNifFile_Skinned_OB", white, white, black, 10.0),
    ] {
        let nif = upstream(file);
        let (_, m) = material(&nif);
        assert_eq!(m.ambient(), ambient, "{file}");
        assert_eq!(m.diffuse(), diffuse, "{file}");
        assert_eq!(m.specular(), specular, "{file}");
        assert_eq!(m.has_specular(), specular != black, "{file}");
        assert_eq!(m.emissive(), black, "{file}");
        assert!(!m.is_emissive(), "{file}");
        assert_eq!(m.glossiness(), glossiness, "{file}");
        assert_eq!(m.alpha(), 1.0, "{file}");
        assert_eq!(m.emissive_multiple(), 1.0, "{file}");
    }
}

#[test]
fn setters_round_trip() {
    let mut nif = upstream("TestNifFile_Skinned_OB");
    let (index, _) = material(&nif);
    let m = nif.block_of_mut::<NiMaterialProperty>(index).unwrap();
    m.set_ambient([0.1, 0.2, 0.3]);
    m.set_diffuse([0.4, 0.5, 0.6]);
    m.set_specular([0.7, 0.8, 0.9]);
    m.set_emissive([1.0, 0.5, 0.0]);
    m.set_glossiness(42.0);
    m.set_alpha(0.25);
    m.set_emissive_multiple(3.0);

    let check = |m: &NiMaterialProperty, multiple: f32| {
        assert_eq!(m.ambient(), [0.1, 0.2, 0.3]);
        assert_eq!(m.diffuse(), [0.4, 0.5, 0.6]);
        assert_eq!(m.specular(), [0.7, 0.8, 0.9]);
        assert!(m.has_specular());
        assert_eq!(m.emissive(), [1.0, 0.5, 0.0]);
        assert!(m.is_emissive());
        assert_eq!(m.glossiness(), 42.0);
        assert_eq!(m.alpha(), 0.25);
        assert_eq!(m.emissive_multiple(), multiple);
    };
    // Visible through the shared borrow, and through the shape.
    check(material(&nif).1, 3.0);
    let shape = nif.shapes().next().unwrap();
    check(
        nif.view_of(shape)
            .unwrap()
            .material_property()
            .unwrap()
            .get(),
        3.0,
    );

    // Oblivion files don't store the emissive multiple.
    let saved = reload(&mut nif);
    check(material(&saved).1, 1.0);
}

#[test]
fn black_clears_the_flags() {
    let mut nif = upstream("TestNifFile_Static_MW");
    let (index, m) = material(&nif);
    assert!(m.has_specular());
    let m = nif.block_of_mut::<NiMaterialProperty>(index).unwrap();
    m.set_specular([0.0; 3]);
    m.set_emissive([0.0, 0.0, 0.001]);
    assert!(!m.has_specular());
    assert!(m.is_emissive());
}

#[test]
fn block_mut_checks_index_and_type() {
    let mut nif = upstream("TestNifFile_Skinned_OB");
    let (index, _) = material(&nif);
    let count = nif.blocks().count();
    assert!(nif.block_of_mut::<NiMaterialProperty>(index).is_some());
    // Base classes work too.
    assert!(nif.block_of_mut::<NiShader>(index).is_some());
    assert!(nif.block_of_mut::<NiObject>(index).is_some());
    assert!(nif.block_of_mut::<NiNode>(index).is_none());
    assert!(nif.block_of_mut::<NiObject>(count).is_none());
    assert!(nif.block_of_mut::<NiObject>(usize::MAX).is_none());

    // Reading through the mutable borrow follows Deref like a shared one.
    let m = nif.block_of_mut::<NiMaterialProperty>(index).unwrap();
    assert_eq!(m.name(), "Cuirass");
    assert_eq!(m.block_name(), "NiMaterialProperty");
}
