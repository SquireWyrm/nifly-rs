//! Tests for the fixed-function properties and NifFile::add_property.

use crate::common::{UPSTREAM, reload};
use std::fs;

use nifly::{
    file::{Index, NifFile, NifVersion},
    objects::NiType,
    shaders::{
        DepthTest, FogFunction, LightingMode, NiDitherProperty, NiFogProperty, NiMaterialProperty,
        NiSpecularProperty, NiStencilProperty, NiVertexColorProperty, NiWireframeProperty, NiZBufferProperty,
        VertexColorFlags, VertexColorMode, ZBufferFlags,
    },
};

fn load(name: &str) -> NifFile {
    NifFile::load(fs::read(format!("{UPSTREAM}/{name}.nif")).unwrap().as_slice(), None).unwrap()
}

fn root_index(nif: &NifFile) -> Index {
    nif.root_node().unwrap().index()
}

fn first<T: NiType>(nif: &NifFile) -> &T {
    nif.blocks_of::<T>().next().unwrap()
}

fn add<T: nifly::shaders::NewProperty>(nif: &mut NifFile, target: Index) -> &mut T {
    let i = nif.edit::<nifly::objects::NiAvObject>(target).map(|mut e| e.add_property::<T, _>()).unwrap();
    nif.block_of_mut::<T>(i).unwrap()
}

#[test]
fn reads_morrowind() {
    let nif = load("TestNifFile_TextureEffect_MW");
    let v = first::<NiVertexColorProperty>(&nif);
    assert_eq!(v.vertex_mode(), Some(VertexColorMode::Ignore));
    assert_eq!(v.lighting_mode(), Some(LightingMode::EmissiveAmbientDiffuse));

    let nif = load("TestNifFile_Morph_MW");
    let flags = first::<NiZBufferProperty>(&nif).flags();
    assert!(flags.testing() && !flags.writing());
}

#[test]
fn oblivion_round_trip() {
    let mut nif = load("TestNifFile_Skinned_OB");
    let shape = nif.shapes().indices().next().unwrap();
    let before = nif.child_blocks(nif.block(shape).unwrap()).count();

    add::<NiSpecularProperty>(&mut nif, shape).set_enabled(true);
    add::<NiDitherProperty>(&mut nif, shape).set_enabled(true);
    add::<NiWireframeProperty>(&mut nif, shape).set_raw_flags(1);
    let fog = add::<NiFogProperty>(&mut nif, shape);
    fog.set_enabled(true);
    fog.set_function(FogFunction::RangeSquared);
    fog.set_depth(2.5);
    fog.set_color([0.25, 0.5, 0.75]);
    let vc = add::<NiVertexColorProperty>(&mut nif, shape);
    vc.set_vertex_mode(VertexColorMode::Emissive);
    vc.set_lighting_mode(LightingMode::Emissive);
    let z = add::<NiZBufferProperty>(&mut nif, shape);
    z.set_flags(ZBufferFlags::from_bits(0).with_testing(true));
    z.set_function(DepthTest::Greater);

    let saved = reload(&mut nif);
    let shape = saved.shapes().next().unwrap();
    assert_eq!(saved.child_blocks(shape).count(), before + 6);
    assert!(saved.child_blocks_of::<NiSpecularProperty>(shape).next().unwrap().enabled());
    assert!(saved.child_blocks_of::<NiDitherProperty>(shape).next().unwrap().enabled());
    assert!(saved.child_blocks_of::<NiWireframeProperty>(shape).next().unwrap().enabled());

    let fog = saved.child_blocks_of::<NiFogProperty>(shape).next().unwrap();
    assert!(fog.enabled());
    assert_eq!(fog.function(), Some(FogFunction::RangeSquared));
    assert_eq!(fog.raw_flags(), 0b011);
    assert_eq!(fog.depth(), 2.5);
    assert_eq!(fog.color(), [0.25, 0.5, 0.75]);

    let vc = saved.child_blocks_of::<NiVertexColorProperty>(shape).next().unwrap();
    assert_eq!(vc.vertex_mode(), Some(VertexColorMode::Emissive));
    assert_eq!(vc.lighting_mode(), Some(LightingMode::Emissive));

    let z = saved.child_blocks_of::<NiZBufferProperty>(shape).next().unwrap();
    assert!(z.flags().testing() && !z.flags().writing());
    assert_eq!(z.function(), Some(DepthTest::Greater));
}

#[test]
fn fallout_3_saves_packed_flags_only() {
    let mut nif = NifFile::create(NifVersion::FALLOUT_3);
    let root = root_index(&nif);

    let flags = VertexColorFlags::from_bits(0)
        .with_vertex_mode(VertexColorMode::AmbientDiffuse)
        .with_lighting_mode(LightingMode::EmissiveAmbientDiffuse);
    let vc = add::<NiVertexColorProperty>(&mut nif, root);
    vc.set_flags(flags);
    // Not saved from Fallout 3 on.
    vc.set_vertex_mode(VertexColorMode::Emissive);

    let flags = ZBufferFlags::from_bits(0).with_testing(true).with_writing(true).with_function(DepthTest::Less);
    let z = add::<NiZBufferProperty>(&mut nif, root);
    z.set_flags(flags);
    // Not saved from Fallout 3 on.
    z.set_function(DepthTest::Never);

    let saved = reload(&mut nif);
    let vc = first::<NiVertexColorProperty>(&saved);
    assert_eq!(vc.flags().vertex_mode(), Some(VertexColorMode::AmbientDiffuse));
    assert_eq!(vc.flags().lighting_mode(), LightingMode::EmissiveAmbientDiffuse);
    assert_eq!(vc.vertex_mode(), Some(VertexColorMode::Ignore));

    let z = first::<NiZBufferProperty>(&saved);
    assert_eq!(z.flags().function(), Some(DepthTest::Less));
    assert!(z.flags().testing() && z.flags().writing());
    assert_eq!(z.function(), Some(DepthTest::LessOrEqual));
}

#[test]
fn morrowind_has_no_z_function() {
    let mut nif = load("TestNifFile_Static_MW");
    let root = root_index(&nif);
    add::<NiZBufferProperty>(&mut nif, root).set_function(DepthTest::Never);
    let saved = reload(&mut nif);
    let root = saved.root_node().unwrap();
    let z = saved.child_blocks_of::<NiZBufferProperty>(&root).next().unwrap();
    assert_eq!(z.function(), Some(DepthTest::LessOrEqual));
}

#[test]
fn skyrim_saves_no_property_list() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let root = root_index(&nif);
    assert!(nif.edit::<nifly::objects::NiAvObject>(root).map(|mut e| e.add_property::<NiFogProperty, _>()).is_some());
    let root = nif.root_node().unwrap();
    assert_eq!(nif.child_blocks_of::<NiFogProperty>(&root).count(), 1);

    let saved = reload(&mut nif);
    let root = saved.root_node().unwrap();
    assert_eq!(saved.child_blocks_of::<NiFogProperty>(&root).count(), 0);
}

#[test]
fn adds_material_and_stencil() {
    let mut nif = load("TestNifFile_Skinned_OB");
    let shape = nif.shapes().indices().next().unwrap();
    assert!(nif.view::<nifly::geometry::NiShape>(shape).unwrap().stencil_property().is_none());
    let materials = nif.blocks_of::<NiMaterialProperty>().count();

    let stencil = nif.edit::<nifly::objects::NiAvObject>(shape).map(|mut e| e.add_property::<NiStencilProperty, _>()).unwrap();
    nif.edit::<nifly::objects::NiAvObject>(shape).map(|mut e| e.add_property::<NiMaterialProperty, _>()).unwrap();

    let saved = reload(&mut nif);
    let shape = saved.shapes().next().unwrap();
    assert!(saved.view_of(shape).unwrap().stencil_property().is_some());
    assert_eq!(saved.blocks_of::<NiMaterialProperty>().count(), materials + 1);
    assert!(stencil > 0);
}

#[test]
fn invalid_target_changes_nothing() {
    let mut nif = load("TestNifFile_Skinned_OB");
    let blocks = nif.blocks().count();
    // Geometry data isn't a shape or node.
    let shape = nif.shapes().next().unwrap();
    let data = nif.block_index(nif.view_of(shape).unwrap().geometry_data().unwrap()).unwrap();

    assert_eq!(nif.edit::<nifly::objects::NiAvObject>(data).map(|mut e| e.add_property::<NiFogProperty, _>()), None);
    assert_eq!(nif.edit::<nifly::objects::NiAvObject>(blocks).map(|mut e| e.add_property::<NiFogProperty, _>()), None);
    assert_eq!(nif.edit::<nifly::objects::NiAvObject>(usize::MAX).map(|mut e| e.add_property::<NiFogProperty, _>()), None);
    assert_eq!(nif.blocks().count(), blocks);
}
