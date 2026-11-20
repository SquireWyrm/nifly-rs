//! Tests for View::texture_set and BsShaderTextureSet.

use crate::common::load;

use nifly::{
    bstr::BStr,
    file::NifFile,
    handle::View,
    shaders::{BsShaderTextureSet, NiShader},
};

fn set_of(nif: &NifFile) -> Option<View<'_, BsShaderTextureSet>> {
    let shape = nif.shapes().next().unwrap();
    let shader = Some(nif.view_of(shape).unwrap().shader()?.index())?;
    nif.view::<NiShader>(shader)?.texture_set()
}

#[test]
fn suzanne_paths() {
    let nif = load("Suzanne");
    let set = set_of(&nif).unwrap();
    assert_eq!(set.index(), 9);
    assert_eq!(set.len(), 9);
    assert!(!set.is_empty());
    assert_eq!(
        set.path(0),
        Some(BStr::new(br"textures\clutter\ruins\RuinSconceWall.dds"))
    );
    assert_eq!(
        set.path(1),
        Some(BStr::new(br"textures\clutter\ruins\RuinSconceWall_n.dds"))
    );
    assert_eq!(set.path(2), Some(BStr::new(b"")));
    assert_eq!(set.path(9), None);
    assert_eq!(set.paths().len(), 9);
}

#[test]
fn shader_without_texture_set() {
    // Cone's only shader is unreferenced and has no texture set; Icosphere has no shader.
    let nif = load("Cone");
    let shader = nif.blocks_of::<NiShader>().views().next().unwrap();
    assert!(shader.texture_set().is_none());
    assert!(set_of(&load("Icosphere")).is_none());
}

#[test]
fn follows_set_texture_slot() {
    let mut nif = load("Suzanne");
    let shape = nif.block_index(nif.shapes().next().unwrap()).unwrap();
    nif.edit::<nifly::geometry::NiShape>(shape)
        .unwrap()
        .set_texture_slot(2, br"textures\glow.dds");
    assert_eq!(
        set_of(&nif).unwrap().path(2),
        Some(BStr::new(br"textures\glow.dds"))
    );
}
