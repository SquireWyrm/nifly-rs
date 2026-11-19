//! Tests for NifFile::texture_paths, texture_slot and set_texture_slot.

use crate::common::{load};

use nifly::{
    bstr::BString,
    file::NifFile,
    geometry::{NiShape, TextureSource},
};

fn shape(nif: &NifFile) -> &NiShape {
    nif.blocks_of::<NiShape>().next().unwrap()
}

#[test]
fn texture_set_paths() {
    let nif = load("Suzanne");
    let paths = nif.view_of(shape(&nif)).unwrap().texture_paths();

    // BSShaderTextureSet has 9 slots; Suzanne fills the diffuse and normal map.
    assert_eq!(paths.len(), 9);
    assert_eq!(paths[0], br"textures\clutter\ruins\RuinSconceWall.dds");
    assert_eq!(paths[1], br"textures\clutter\ruins\RuinSconceWall_n.dds");
    assert!(paths[2..].iter().all(|p| p.is_empty()));
}

#[test]
fn empty_slots_are_included() {
    for name in ["Cube", "Bones"] {
        let nif = load(name);
        let paths = nif.view_of(shape(&nif)).unwrap().texture_paths();
        assert_eq!(paths.len(), 9, "{name}");
        assert!(paths.iter().all(|p| p.is_empty()), "{name}");
    }
}

#[test]
fn no_shader_means_no_paths() {
    // Icosphere's shape has no shader. Cone's BSLightingShaderProperty exists but isn't
    // referenced by the shape.
    for name in ["Icosphere", "Cone"] {
        let nif = load(name);
        assert!(nif.view_of(shape(&nif)).unwrap().texture_paths().is_empty(), "{name}");
    }
}

#[test]
fn paths_borrow_from_the_file() {
    let nif = load("Suzanne");
    let paths = nif.view_of(shape(&nif)).unwrap().texture_paths();
    // Borrowed, not copied: calling again returns the same memory.
    let again = nif.view_of(shape(&nif)).unwrap().texture_paths();
    assert!(std::ptr::eq(paths[0].as_ptr(), again[0].as_ptr()));
}

fn shape_index(nif: &NifFile) -> usize {
    nif.block_index(shape(nif)).unwrap()
}

fn save_and_reload(nif: &mut NifFile) -> NifFile {
    let mut cur = std::io::Cursor::new(Vec::new());
    nif.save(&mut cur, None).unwrap();
    NifFile::load(cur.into_inner().as_slice(), None).unwrap()
}

#[test]
fn texture_slot_reads_texture_set() {
    let nif = load("Suzanne");
    let shape = shape(&nif);

    assert_eq!(
        nif.view_of(shape).unwrap().texture_slot(0),
        Some((TextureSource::TextureSet, br"textures\clutter\ruins\RuinSconceWall.dds".into()))
    );
    assert_eq!(
        nif.view_of(shape).unwrap().texture_slot(1),
        Some((TextureSource::TextureSet, br"textures\clutter\ruins\RuinSconceWall_n.dds".into()))
    );
    // Empty slots are still slots.
    assert_eq!(nif.view_of(shape).unwrap().texture_slot(8), Some((TextureSource::TextureSet, "".into())));
}

#[test]
fn texture_slot_out_of_range_or_no_shader() {
    let suzanne = load("Suzanne");
    assert_eq!(suzanne.view_of(shape(&suzanne)).unwrap().texture_slot(9), None, "only 9 slots");
    assert_eq!(suzanne.view_of(shape(&suzanne)).unwrap().texture_slot(usize::MAX), None);

    let ico = load("Icosphere");
    assert_eq!(ico.view_of(shape(&ico)).unwrap().texture_slot(0), None, "no shader");
}

#[test]
fn texture_slot_matches_texture_paths() {
    for name in ["Cube", "Suzanne", "Bones"] {
        let nif = load(name);
        let shape = shape(&nif);
        for (i, path) in nif.view_of(shape).unwrap().texture_paths().into_iter().enumerate() {
            assert_eq!(nif.view_of(shape).unwrap().texture_slot(i), Some((TextureSource::TextureSet, path.to_owned())), "{name} slot {i}");
        }
    }
}

#[test]
fn set_texture_slot() {
    let mut nif = load("Suzanne");
    let index = shape_index(&nif);

    nif.edit::<nifly::geometry::NiShape>(index).unwrap().set_texture_slot(0, r"textures\new\diffuse.dds");
    nif.edit::<nifly::geometry::NiShape>(index).unwrap().set_texture_slot(2, br"textures\new\glow.dds");

    let shape = shape(&nif);
    assert_eq!(nif.view_of(shape).unwrap().texture_slot(0).unwrap().1, br"textures\new\diffuse.dds");
    assert_eq!(nif.view_of(shape).unwrap().texture_slot(2).unwrap().1, br"textures\new\glow.dds");
    // Other slots are untouched.
    assert_eq!(nif.view_of(shape).unwrap().texture_slot(1).unwrap().1, br"textures\clutter\ruins\RuinSconceWall_n.dds");
    assert_eq!(nif.view_of(shape).unwrap().texture_paths()[2], br"textures\new\glow.dds");
}

#[test]
fn set_texture_slot_clears_with_empty_path() {
    let mut nif = load("Suzanne");
    let index = shape_index(&nif);
    nif.edit::<nifly::geometry::NiShape>(index).unwrap().set_texture_slot(1, "");
    assert_eq!(nif.view_of(shape(&nif)).unwrap().texture_slot(1), Some((TextureSource::TextureSet, "".into())));
}

#[test]
fn set_texture_slot_ignores_invalid_targets() {
    let mut nif = load("Suzanne");
    let index = shape_index(&nif);
    let before: Vec<BString> = nif.view_of(shape(&nif)).unwrap().texture_paths().into_iter().map(ToOwned::to_owned).collect();

    nif.edit::<nifly::geometry::NiShape>(index).unwrap().set_texture_slot(9, "out of range");
    nif.edit::<nifly::geometry::NiShape>(index).unwrap().set_texture_slot(usize::MAX, "out of range");
    // The root and missing blocks aren't shapes.
    assert!(nif.edit::<nifly::geometry::NiShape>(0).is_none());
    assert!(nif.edit::<nifly::geometry::NiShape>(usize::MAX).is_none());

    let after: Vec<BString> = nif.view_of(shape(&nif)).unwrap().texture_paths().into_iter().map(ToOwned::to_owned).collect();
    assert_eq!(after, before);

    // A shape without a shader has no slots to set.
    let mut ico = load("Icosphere");
    let ico_shape = shape_index(&ico);
    ico.edit::<nifly::geometry::NiShape>(ico_shape).unwrap().set_texture_slot(0, "nowhere");
    assert_eq!(ico.view_of(shape(&ico)).unwrap().texture_slot(0), None);
}

#[test]
fn set_texture_slot_survives_save() {
    let mut nif = load("Cube");
    let index = shape_index(&nif);
    nif.edit::<nifly::geometry::NiShape>(index).unwrap().set_texture_slot(0, r"textures\cube\d.dds");
    nif.edit::<nifly::geometry::NiShape>(index).unwrap().set_texture_slot(1, r"textures\cube\n.dds");

    let saved = save_and_reload(&mut nif);
    let paths = saved.view_of(shape(&saved)).unwrap().texture_paths();
    assert_eq!(paths[0], br"textures\cube\d.dds");
    assert_eq!(paths[1], br"textures\cube\n.dds");
    assert!(paths[2..].iter().all(|p| p.is_empty()));
}
