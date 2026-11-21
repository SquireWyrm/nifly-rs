//! Tests for pointing refs at existing blocks: shaders, alpha properties, texture sets,
//! geometry data, skins, multi-bounds, LOD data, additional data, pixel data and palettes.

use nifly::{
    file::{Index, NifFile, NifVersion},
    geometry::{NiAdditionalGeometryData, NiGeometryData, NiShape},
    nodes::{BsMultiBound, BsMultiBoundNode, NiLodNode, NiRangeLodData},
    object3d::Transform,
    shaders::{AlphaFlags, NiPixelData, NiShader, NiSourceTexture, TextureRenderData},
    skin::{BsSkinInstance, NiSkinData, NiSkinInstance},
};

use crate::common::{first_index, load, reload_in_place, upstream};

fn first_shape(nif: &NifFile) -> Index {
    nif.shapes().indices().next().unwrap()
}

fn data_index(nif: &NifFile, shape: Index) -> Index {
    let data = nif.view::<NiShape>(shape).unwrap().geometry_data().unwrap();
    nif.blocks_of::<NiGeometryData>()
        .indexed()
        .find(|(_, d)| std::ptr::eq(*d, data))
        .unwrap()
        .0
}

#[test]
fn shader_alpha_and_texture_set() {
    let mut nif = load("Cube");
    let shape = first_shape(&nif);
    let mut edit = nif.edit::<NiShape>(shape).unwrap();
    let shader = edit.view().shader().unwrap().index();
    let set = edit.view().shader().unwrap().texture_set().unwrap().index();
    let alpha = edit.assign_alpha_property(AlphaFlags::DEFAULT, 128);

    assert!(!edit.set_shader(Some(set)));
    assert!(!edit.set_alpha_property(Some(shader)));
    assert!(edit.set_shader(None) && edit.set_alpha_property(None));
    let saved = reload_in_place(&mut nif);
    let view = saved.view::<NiShape>(shape).unwrap();
    assert!(view.shader().is_none() && view.alpha_property().is_none());

    let mut edit = nif.edit::<NiShape>(shape).unwrap();
    assert!(edit.set_shader(Some(shader)) && edit.set_alpha_property(Some(alpha)));
    let mut shader_edit = nif.edit::<NiShader>(shader).unwrap();
    assert!(!shader_edit.set_texture_set(Some(shader)));
    assert!(shader_edit.set_texture_set(None));
    assert!(
        reload_in_place(&mut nif)
            .view::<NiShader>(shader)
            .unwrap()
            .texture_set()
            .is_none()
    );
    assert!(
        nif.edit::<NiShader>(shader)
            .unwrap()
            .set_texture_set(Some(set))
    );

    let saved = reload_in_place(&mut nif);
    let view = saved.view::<NiShape>(shape).unwrap();
    assert_eq!(view.shader().map(|s| s.index()), Some(shader));
    assert_eq!(view.alpha_property().map(|a| a.index()), Some(alpha));
    assert_eq!(
        view.shader().unwrap().texture_set().map(|s| s.index()),
        Some(set)
    );
}

#[test]
fn geometry_data() {
    // Strips_MW: NiTriStrips `a` has its own data; `b` shares its data with a third shape.
    let mut nif = load("Strips_MW");
    let a = nif
        .shapes()
        .indexed()
        .find(|(_, s)| s.name() == "Tri EditorMarker_box_02")
        .unwrap()
        .0;
    let b = nif.shapes().indices().find(|&s| s != a).unwrap();
    let b_data = data_index(&nif, b);
    assert!(!nif.edit::<NiShape>(a).unwrap().set_geometry_data(a));
    assert!(nif.edit::<NiShape>(a).unwrap().set_geometry_data(b_data));
    let vertices = |nif: &NifFile, s| nif.view::<NiShape>(s).unwrap().vertices();
    assert_eq!(vertices(&nif, a), vertices(&nif, b));
    assert_eq!(data_index(&reload_in_place(&mut nif), a), b_data);

    // Each shape type takes only its own data type.
    let mut nif = load("Strips_MW");
    let a_data = data_index(&nif, a);
    assert!(nif.edit::<NiShape>(a).unwrap().triangulate());
    assert!(!nif.edit::<NiShape>(b).unwrap().set_geometry_data(a_data));
    assert!(!nif.edit::<NiShape>(a).unwrap().set_geometry_data(b_data));

    // BSTriShape has no data ref.
    let mut nif = load("Cube");
    let shape = first_shape(&nif);
    assert!(!nif.edit::<NiShape>(shape).unwrap().set_geometry_data(shape));
}

#[test]
fn ni_skin_instance() {
    let mut nif = upstream("TestNifFile_Skinned_OB");
    let shape = first_shape(&nif);
    let skin = nif
        .view::<NiShape>(shape)
        .unwrap()
        .skin_instance()
        .unwrap()
        .index();
    let view = nif.view::<NiSkinInstance>(skin).unwrap();
    let (data, partition) = (
        view.skin_data().unwrap().index(),
        view.skin_partition().unwrap().index(),
    );

    let mut edit = nif.edit::<NiSkinInstance>(skin).unwrap();
    assert!(!edit.set_skin_data(Some(partition)));
    assert!(!edit.set_skin_partition(Some(data)));
    assert!(edit.set_skin_partition(None));
    assert!(
        reload_in_place(&mut nif)
            .view::<NiSkinInstance>(skin)
            .unwrap()
            .skin_partition()
            .is_none()
    );
    let mut edit = nif.edit::<NiSkinInstance>(skin).unwrap();
    assert!(edit.set_skin_partition(Some(partition)) && edit.set_skin_data(Some(data)));
    assert!(edit.skin_data().is_some() && edit.skin_partition().is_some());

    let mut shape_edit = nif.edit::<NiShape>(shape).unwrap();
    assert!(!shape_edit.set_skin_instance(Some(data)));
    assert!(shape_edit.set_skin_instance(None));
    assert!(shape_edit.skin_instance().is_none());
    assert!(
        reload_in_place(&mut nif)
            .view::<NiShape>(shape)
            .unwrap()
            .bone_ids()
            .is_empty()
    );
    assert!(
        nif.edit::<NiShape>(shape)
            .unwrap()
            .set_skin_instance(Some(skin))
    );
    assert_eq!(
        reload_in_place(&mut nif)
            .view::<NiShape>(shape)
            .unwrap()
            .skin_instance()
            .unwrap()
            .index(),
        skin
    );
}

#[test]
fn skin_data_partition() {
    let mut nif = upstream("TestNifFile_Skinned_MW");
    let data = first_index::<NiSkinData>(&nif);
    let before = nif
        .view::<NiSkinData>(data)
        .unwrap()
        .partition()
        .map(|p| p.index());
    let mut edit = nif.edit::<NiSkinData>(data).unwrap();
    assert!(!edit.set_partition(Some(data)));
    assert!(edit.set_partition(None));
    assert!(edit.partition().is_none());
    assert!(
        reload_in_place(&mut nif)
            .view::<NiSkinData>(data)
            .unwrap()
            .partition()
            .is_none()
    );
    assert!(nif.edit::<NiSkinData>(data).unwrap().set_partition(before));
}

#[test]
fn bs_skin_instance() {
    let mut nif = upstream("TestNifFile_Skinned_FO4");
    let skin = first_index::<BsSkinInstance>(&nif);
    let mut edit = nif.edit::<BsSkinInstance>(skin).unwrap();
    let data = edit.bone_data().unwrap().index();
    assert!(!edit.set_bone_data(Some(skin)));
    assert!(edit.set_bone_data(None));
    assert!(
        reload_in_place(&mut nif)
            .view::<BsSkinInstance>(skin)
            .unwrap()
            .bone_data()
            .is_none()
    );
    assert!(
        nif.edit::<BsSkinInstance>(skin)
            .unwrap()
            .set_bone_data(Some(data))
    );
    assert_eq!(
        reload_in_place(&mut nif)
            .view::<BsSkinInstance>(skin)
            .unwrap()
            .bone_data()
            .unwrap()
            .index(),
        data
    );
}

#[test]
fn multi_bound() {
    let mut nif = upstream("TestNifFile_MultiBound_SE");
    let node = first_index::<BsMultiBoundNode>(&nif);
    let mut edit = nif.edit::<BsMultiBoundNode>(node).unwrap();
    let bound = edit.view().multi_bound().unwrap().index();
    let data = edit
        .view()
        .multi_bound()
        .unwrap()
        .multi_bound_data()
        .unwrap()
        .index();
    assert!(!edit.set_multi_bound(Some(data)));
    assert!(edit.set_multi_bound(None));
    let mut bound_edit = nif.edit::<BsMultiBound>(bound).unwrap();
    assert!(!bound_edit.set_multi_bound_data(Some(bound)));
    assert!(bound_edit.set_multi_bound_data(None));
    let saved = reload_in_place(&mut nif);
    assert!(
        saved
            .view::<BsMultiBoundNode>(node)
            .unwrap()
            .multi_bound()
            .is_none()
    );
    assert!(
        saved
            .view::<BsMultiBound>(bound)
            .unwrap()
            .multi_bound_data()
            .is_none()
    );

    assert!(
        nif.edit::<BsMultiBound>(bound)
            .unwrap()
            .set_multi_bound_data(Some(data))
    );
    assert!(
        nif.edit::<BsMultiBoundNode>(node)
            .unwrap()
            .set_multi_bound(Some(bound))
    );
    let saved = reload_in_place(&mut nif);
    let bound = saved
        .view::<BsMultiBoundNode>(node)
        .unwrap()
        .multi_bound()
        .unwrap();
    assert_eq!(bound.multi_bound_data().map(|d| d.index()), Some(data));
}

#[test]
fn lod_data() {
    let mut nif = NifFile::create(NifVersion::OBLIVION);
    let node = nif
        .add_node_of::<NiLodNode>("LOD", Transform::default(), None)
        .unwrap();
    let mut edit = nif.edit::<NiLodNode>(node).unwrap();
    let data = edit.assign_lod_data::<NiRangeLodData>();
    assert!(!edit.set_lod_data(Some(node)));
    assert!(edit.set_lod_data(None));
    assert!(edit.lod_data().is_none());
    assert!(edit.set_lod_data(Some(data)));
    assert_eq!(
        reload_in_place(&mut nif)
            .view::<NiLodNode>(node)
            .unwrap()
            .lod_data()
            .unwrap()
            .index(),
        data
    );
}

#[test]
fn additional_data() {
    let mut nif = upstream("TestNifFile_Skinned_OB");
    let data = first_index::<NiGeometryData>(&nif);
    let mut edit = nif.edit::<NiGeometryData>(data).unwrap();
    let extra = edit.assign_additional_data::<NiAdditionalGeometryData, _>();
    assert!(!edit.set_additional_data(Some(data)));
    assert!(edit.set_additional_data(None));
    assert!(edit.additional_data().is_none());
    assert!(edit.set_additional_data(Some(extra)));
    assert_eq!(
        reload_in_place(&mut nif)
            .view::<NiGeometryData>(data)
            .unwrap()
            .additional_data()
            .unwrap()
            .index(),
        extra
    );
}

#[test]
fn pixel_data_and_palette() {
    let mut nif = NifFile::create(NifVersion::MORROWIND);
    let texture = nif.add_source_texture("embedded.dds");
    nif.block_of_mut::<NiSourceTexture>(texture)
        .unwrap()
        .set_use_external(false);
    let mut edit = nif.edit::<NiSourceTexture>(texture).unwrap();
    let data = edit.assign_pixel_data::<NiPixelData>();
    let mut data_edit = nif.edit::<TextureRenderData>(data).unwrap();
    let palette = data_edit
        .assign_palette(&[[0, 0, 0, 255]; 16], false)
        .unwrap();
    assert!(!data_edit.set_palette(Some(data)));
    assert!(data_edit.set_palette(None));
    assert!(data_edit.palette().is_none());
    assert!(data_edit.set_palette(Some(palette)));

    let mut edit = nif.edit::<NiSourceTexture>(texture).unwrap();
    assert!(!edit.set_pixel_data(Some(palette)));
    assert!(edit.set_pixel_data(None));
    assert!(
        reload_in_place(&mut nif)
            .view::<NiSourceTexture>(texture)
            .unwrap()
            .pixel_data()
            .is_none()
    );
    assert!(
        nif.edit::<NiSourceTexture>(texture)
            .unwrap()
            .set_pixel_data(Some(data))
    );
    let saved = reload_in_place(&mut nif);
    let data_view = saved
        .view::<NiSourceTexture>(texture)
        .unwrap()
        .pixel_data()
        .unwrap();
    assert_eq!(
        (data_view.index(), data_view.palette().map(|p| p.index())),
        (data, Some(palette))
    );
}
