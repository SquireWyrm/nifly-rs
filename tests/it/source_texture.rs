//! Tests for NifFile::add_source_texture, Edit::set_map_source, and set_texture_slot on texturing
//! properties.

use crate::common::{UPSTREAM, open, reload};

use nifly::{
    file::{Index, NifFile, NifVersion},
    geometry::TextureSource,
    object3d::Vec3,
    shaders::{NiSourceTexture, NiTexturingProperty, TextureMap},
};

fn load(name: &str) -> NifFile {
    open(format!("{UPSTREAM}/{name}.nif"))
}

fn shape(nif: &NifFile) -> Index {
    nif.shapes().indices().next().unwrap()
}

fn property(nif: &NifFile) -> &NiTexturingProperty {
    nif.view::<nifly::geometry::NiShape>(shape(nif))
        .unwrap()
        .texturing_property()
        .unwrap()
        .get()
}

fn property_index(nif: &NifFile) -> Index {
    nif.block_index(property(nif)).unwrap()
}

fn slot(nif: &NifFile, index: usize) -> Option<String> {
    let (source, path) = nif
        .view::<nifly::geometry::NiShape>(shape(nif))
        .unwrap()
        .texture_slot(index)?;
    assert_eq!(source, TextureSource::TexturingProperty);
    Some(path.to_string())
}

fn source_path(nif: &NifFile, map: TextureMap) -> Option<String> {
    let property = nif.view::<NiTexturingProperty>(property_index(nif))?;
    Some(property.map_source(map)?.file_name().to_string())
}

fn set_source(nif: &mut NifFile, map: TextureMap, source: Option<Index>) -> bool {
    let property = property_index(nif);
    nif.edit::<NiTexturingProperty>(property)
        .unwrap()
        .set_map_source(map, source)
}

#[test]
fn set_texture_slot_creates_source() {
    let mut nif = load("TestNifFile_Skinned_OB");
    assert!(!property(&nif).has_map(TextureMap::Glow));
    let blocks = nif.blocks().count();

    nif.edit::<nifly::geometry::NiShape>(shape(&nif))
        .unwrap()
        .set_texture_slot(4, "textures\\glow.dds");
    assert_eq!(nif.blocks().count(), blocks + 1);
    assert_eq!(
        source_path(&nif, TextureMap::Glow).unwrap(),
        "textures\\glow.dds"
    );

    // Setting it again reuses the texture.
    nif.edit::<nifly::geometry::NiShape>(shape(&nif))
        .unwrap()
        .set_texture_slot(4, "textures\\glow2.dds");
    assert_eq!(nif.blocks().count(), blocks + 1);

    let saved = reload(&mut nif);
    assert_eq!(slot(&saved, 4).unwrap(), "textures\\glow2.dds");
    assert_eq!(slot(&saved, 0).unwrap(), "textures\\Cuirass.dds");
}

#[test]
fn set_texture_slot_keeps_higher_maps() {
    let mut nif = load("TestNifFile_Skinned_OB");
    nif.edit::<nifly::geometry::NiShape>(shape(&nif))
        .unwrap()
        .set_texture_slot(4, "textures\\glow.dds");
    nif.edit::<nifly::geometry::NiShape>(shape(&nif))
        .unwrap()
        .set_texture_slot(0, "textures\\base.dds");
    assert_eq!(property(&nif).texture_count(), 7);

    let saved = reload(&mut nif);
    assert_eq!(slot(&saved, 0).unwrap(), "textures\\base.dds");
    assert_eq!(slot(&saved, 4).unwrap(), "textures\\glow.dds");
}

#[test]
fn decals_from_fallout_3() {
    // Decals are saved after the normal and parallax maps, so need a count of 9.
    let mut nif = NifFile::create(NifVersion::FALLOUT_3);
    let vertices = [
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    ];
    let shape = nif
        .create_shape("Tri", &vertices, &[[0, 1, 2]], None, None)
        .unwrap();
    nif.edit::<nifly::objects::NiAvObject>(shape)
        .map(|mut e| e.add_property::<NiTexturingProperty, _>())
        .unwrap();

    nif.edit::<nifly::geometry::NiShape>(shape)
        .unwrap()
        .set_texture_slot(6, "textures\\decal.dds");
    assert_eq!(property(&nif).texture_count(), 9);
    assert_eq!(slot(&reload(&mut nif), 6).unwrap(), "textures\\decal.dds");
}

#[test]
fn decals_before_fallout_3() {
    let mut nif = load("TestNifFile_Skinned_OB");
    nif.edit::<nifly::geometry::NiShape>(shape(&nif))
        .unwrap()
        .set_texture_slot(9, "textures\\decal3.dds");
    assert_eq!(property(&nif).texture_count(), 10);
    assert_eq!(slot(&reload(&mut nif), 9).unwrap(), "textures\\decal3.dds");
}

#[test]
fn set_map_source() {
    let mut nif = load("TestNifFile_Skinned_OB");
    let texture = nif.add_source_texture("textures\\detail.dds");
    assert!(set_source(&mut nif, TextureMap::Detail, Some(texture)));
    assert!(set_source(&mut nif, TextureMap::Gloss, Some(texture)));
    assert!(set_source(&mut nif, TextureMap::Base, None));

    let saved = reload(&mut nif);
    let p = property(&saved);
    assert!(!p.has_map(TextureMap::Base));
    assert_eq!(
        p.map_source_index(TextureMap::Detail),
        p.map_source_index(TextureMap::Gloss)
    );
    assert_eq!(
        source_path(&saved, TextureMap::Detail).unwrap(),
        "textures\\detail.dds"
    );
    // The old base texture is dropped, being unused.
    assert_eq!(saved.blocks_of::<NiSourceTexture>().count(), 1);
}

#[test]
fn set_map_source_rejects_invalid() {
    let mut nif = load("TestNifFile_Skinned_OB");
    let shape = shape(&nif);
    assert!(!set_source(&mut nif, TextureMap::Base, Some(shape)));
    assert!(!set_source(&mut nif, TextureMap::Base, Some(usize::MAX)));
    assert!(nif.edit::<NiTexturingProperty>(shape).is_none());
    assert_eq!(
        source_path(&nif, TextureMap::Base).unwrap(),
        "textures\\Cuirass.dds"
    );
}

#[test]
fn normal_map_not_saved_before_fallout_3() {
    let mut nif = load("TestNifFile_Skinned_OB");
    let texture = nif.add_source_texture("textures\\normal.dds");
    assert!(set_source(&mut nif, TextureMap::Normal, Some(texture)));
    assert!(property_has(&nif, TextureMap::Normal));
    assert!(!property_has(&reload(&mut nif), TextureMap::Normal));
}

fn property_has(nif: &NifFile, map: TextureMap) -> bool {
    property(nif).has_map(map)
}
