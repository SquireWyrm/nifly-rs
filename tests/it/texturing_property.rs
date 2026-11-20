//! Tests for NifFile::texturing_property(_mut) and NiTexturingProperty.

use crate::common::{load, upstream, reload};

use nifly::{
    file::{Index, NifFile},
    shaders::{ApplyMode, ClampMode, FilterMode, NiTexturingProperty, TextureMap},
};

const MAPS: [TextureMap; 12] = [
    TextureMap::Base,
    TextureMap::Dark,
    TextureMap::Detail,
    TextureMap::Gloss,
    TextureMap::Glow,
    TextureMap::Bump,
    TextureMap::Normal,
    TextureMap::Parallax,
    TextureMap::Decal0,
    TextureMap::Decal1,
    TextureMap::Decal2,
    TextureMap::Decal3,
];

/// The texturing property of the shape named `name`.
fn texturing<'a>(nif: &'a NifFile, name: &str) -> &'a NiTexturingProperty {
    let shape = nif.shapes().find(|s| s.name() == name).unwrap();
    nif.view_of(shape).unwrap().texturing_property().unwrap().get()
}

fn shape_index(nif: &NifFile, name: &str) -> Index {
    nif.shapes().indexed().find(|(_, s)| s.name() == name).unwrap().0
}

/// (clamp, filter, UV set) of `map`.
fn map_fields(t: &NiTexturingProperty, map: TextureMap) -> (Option<ClampMode>, Option<FilterMode>, Option<u32>) {
    (t.map_clamp_mode(map), t.map_filter_mode(map), t.map_uv_set(map))
}

#[test]
fn reads_maps() {
    let nif = upstream("TestNifFile_Skinned_OB");
    let t = texturing(&nif, "Cuirass");
    assert_eq!(t.apply_mode(), Some(ApplyMode::Modulate));
    assert_eq!(t.texture_count(), 7);

    // Only the base map, whose source holds the shape's texture path.
    assert!(t.has_map(TextureMap::Base));
    let source = nif.block(t.map_source_index(TextureMap::Base).unwrap()).unwrap();
    assert_eq!(source.block_name(), "NiSourceTexture");
    let shape = nif.shapes().find(|s| s.name() == "Cuirass").unwrap();
    assert_eq!(nif.view_of(shape).unwrap().texture_paths(), [r"textures\Cuirass.dds"]);
    let expected = (Some(ClampMode::WrapBoth), Some(FilterMode::Trilinear), Some(0));
    assert_eq!(map_fields(t, TextureMap::Base), expected);
    for map in &MAPS[1..] {
        assert!(!t.has_map(*map), "{map:?}");
        assert_eq!(t.map_source_index(*map), None, "{map:?}");
        assert_eq!(map_fields(t, *map), (None, None, None), "{map:?}");
    }

    // Morrowind, clamped.
    let nif = upstream("TestNifFile_Skinned_MW");
    let t = texturing(&nif, "Tri Ex_Ashl_A_Banner_R");
    assert_eq!(t.map_clamp_mode(TextureMap::Base), Some(ClampMode::ClampBoth));
}

#[test]
fn setters_round_trip() {
    let mut nif = upstream("TestNifFile_Skinned_OB");
    let shape = shape_index(&nif, "Cuirass");
    let mut t_shape = nif.edit::<nifly::geometry::NiShape>(shape).unwrap();
    let mut t_edit = t_shape.texturing_property().unwrap();
    let t = &mut *t_edit;
    t.set_apply_mode(ApplyMode::Decal);
    assert!(t.set_map_clamp_mode(TextureMap::Base, ClampMode::ClampUWrapV));
    assert!(t.set_map_filter_mode(TextureMap::Base, FilterMode::Bilinear));
    assert!(t.set_map_uv_set(TextureMap::Base, 1));
    // Maps the property doesn't have aren't set.
    assert!(!t.set_map_clamp_mode(TextureMap::Dark, ClampMode::ClampBoth));
    assert!(!t.set_map_filter_mode(TextureMap::Glow, FilterMode::Nearest));
    assert!(!t.set_map_uv_set(TextureMap::Decal3, 2));

    let expected = (Some(ClampMode::ClampUWrapV), Some(FilterMode::Bilinear), Some(1));
    let t = texturing(&nif, "Cuirass");
    assert_eq!(t.apply_mode(), Some(ApplyMode::Decal));
    assert_eq!(map_fields(t, TextureMap::Base), expected);
    assert!(MAPS[1..].iter().all(|m| !t.has_map(*m)));

    let saved = reload(&mut nif);
    let t = texturing(&saved, "Cuirass");
    assert_eq!(t.apply_mode(), Some(ApplyMode::Decal));
    assert_eq!(map_fields(t, TextureMap::Base), expected);
    assert!(MAPS[1..].iter().all(|m| !t.has_map(*m)));
}

#[test]
fn none_without_one() {
    for asset in ["Bones", "Cone", "Cube", "Icosphere", "Suzanne"] {
        let nif = load(asset);
        assert!(nif.shapes().all(|s| nif.view_of(s).unwrap().texturing_property().is_none()), "{asset}");
    }
    let nif = upstream("TestNifFile_Skinned_SE");
    assert!(nif.shapes().all(|s| nif.view_of(s).unwrap().texturing_property().is_none()));

    let nif = upstream("TestNifFile_Skinned_OB");
    let property = nif.blocks_of::<NiTexturingProperty>().indices().next().unwrap();
    let count = nif.blocks().count();
    for index in [0, property, count, usize::MAX] {
        assert!(nif.view::<nifly::geometry::NiShape>(index).and_then(|v| v.texturing_property()).is_none(), "{index}");
    }
}
