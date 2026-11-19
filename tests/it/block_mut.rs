//! Tests for block, block_of, block_mut, block_of_mut, NiObject::downcast_mut, upcast_mut and
//! the other `_mut` lookups on NifFile.

use crate::common::{load, upstream};
use std::{io::Cursor, pin::Pin, ptr};

use nifly::{
    file::{Index, NifFile},
    geometry::NiShape,
    nodes::NiNode,
    objects::NiObject,
    shaders::{NiMaterialProperty, NiShader},
};

/// The address of a pinned block, to compare with a shared reference.
fn addr<T>(p: Option<Pin<&mut T>>) -> Option<*const T> {
    p.map(|p| ptr::from_ref(&*p))
}

fn shape_index(nif: &NifFile) -> Index {
    nif.shapes().indices().next().unwrap()
}

#[test]
fn same_block_as_shared_form() {
    let mut nif = load("Suzanne");
    let shape = shape_index(&nif);
    let root: *const NiNode = nif.root_node().unwrap().get();
    let named: *const NiShape = nif.find_block_by_name::<NiShape>("Suzanne:6").unwrap();
    let shader = nif.view::<NiShape>(shape).unwrap().shader().unwrap();
    let shader_index = shader.index();
    let shader = ptr::from_ref(shader.get());
    let set = ptr::from_ref(nif.block(9).unwrap());
    assert_eq!(shader_index, 8);

    assert_eq!(addr(nif.root_node_mut()), Some(root));
    assert_eq!(addr(nif.find_block_by_name_mut::<NiShape>("Suzanne:6")), Some(named));
    let mut shape_edit = nif.edit::<NiShape>(shape).unwrap();
    assert_eq!(ptr::from_ref(&*shape_edit.shader().unwrap().block()), shader);
    let mut edit = nif.edit::<NiShader>(shader_index).unwrap();
    let mut texture_set = edit.texture_set().unwrap();
    assert_eq!(ptr::from_ref(&*texture_set.block()).cast(), set);

    // Icosphere: shape 3's parent is Indirect2 (block 2).
    let mut ico = load("Icosphere");
    let indirect2: *const NiNode = ico.block(2).unwrap().downcast().unwrap();
    let mut shape = ico.edit::<NiShape>(3).unwrap();
    let mut parent = shape.parent().unwrap();
    assert_eq!(ptr::from_ref(&*parent.block()), indirect2);
}

#[test]
fn material_and_shader_in_older_files() {
    let mut nif = upstream("TestNifFile_Static_MW");
    let shape = shape_index(&nif);
    let material = ptr::from_ref(nif.view::<NiShape>(shape).unwrap().material_property().unwrap().get());
    let mut edit = nif.edit::<NiShape>(shape).unwrap();
    assert_eq!(ptr::from_ref(&*edit.material_property().unwrap().block()), material);
    // With no other shader, the shader is the material.
    assert_eq!(ptr::from_ref(&*edit.shader().unwrap().block()).cast::<u8>(), material.cast());
}

#[test]
fn none_cases() {
    let mut nif = load("Suzanne");
    let root = nif.root_node().unwrap().index();
    let count = nif.blocks().count();
    for index in [root, 9, count, usize::MAX] {
        assert!(nif.view::<nifly::geometry::NiShape>(index).and_then(|v| v.shader()).is_none(), "shader of {index}");
        assert!(nif.view::<nifly::geometry::NiShape>(index).and_then(|v| v.material_property()).is_none(), "material of {index}");
    }
    // Suzanne's shape has a shader but no material property.
    assert!(nif.view::<nifly::geometry::NiShape>(shape_index(&nif)).and_then(|v| v.material_property()).is_none());
    // The shape isn't a shader; block 9 is the texture set itself.
    assert!(nif.edit::<NiShader>(shape_index(&nif)).is_none());
    assert!(nif.edit::<NiShader>(9).is_none());
    assert!(nif.edit::<NiShader>(count).is_none());
    assert!(nif.find_block_by_name_mut::<NiShape>("missing").is_none());
    assert!(nif.find_block_by_name_mut::<NiNode>("Suzanne:6").is_none());
    assert!(nif.edit::<NiNode>(root).unwrap().parent().is_none());

    // The second Morrowind shape has no properties.
    let mw = upstream("TestNifFile_Static_MW");
    let second = mw.shapes().indices().nth(1).unwrap();
    assert!(mw.view::<nifly::geometry::NiShape>(second).and_then(|v| v.material_property()).is_none());
    assert!(mw.view::<nifly::geometry::NiShape>(second).and_then(|v| v.shader()).is_none());
}

#[test]
fn set_through_material_property_mut() {
    let mut nif = upstream("TestNifFile_Skinned_OB");
    let shape = shape_index(&nif);
    nif.edit::<nifly::geometry::NiShape>(shape).unwrap().material_property().unwrap().block().set_alpha(0.5);
    assert_eq!(nif.shapes().views().next().unwrap().material_property().unwrap().alpha(), 0.5);

    let mut cur = Cursor::new(Vec::new());
    nif.save(&mut cur, None).unwrap();
    let saved = NifFile::load(cur.into_inner().as_slice(), None).unwrap();
    assert_eq!(saved.shapes().views().next().unwrap().material_property().unwrap().alpha(), 0.5);
}

#[test]
fn block_of_matches_downcast() {
    let nif = upstream("TestNifFile_Static_MW");
    let index = nif.blocks_of::<NiMaterialProperty>().indices().next().unwrap();
    let block = nif.block(index).unwrap();
    let material = nif.block_of::<NiMaterialProperty>(index).unwrap();
    assert!(ptr::eq(material, block.downcast::<NiMaterialProperty>().unwrap()));
    // Base types work too; other types and missing blocks give `None`.
    assert!(ptr::eq(nif.block_of::<NiObject>(index).unwrap(), block));
    assert!(nif.block_of::<NiShader>(index).is_some());
    assert!(nif.block_of::<NiNode>(index).is_none());
    let count = nif.blocks().count();
    for index in [count, usize::MAX] {
        assert!(nif.block_of::<NiObject>(index).is_none(), "{index}");
    }
}

#[test]
fn block_mut_and_downcast_mut() {
    let mut nif = upstream("TestNifFile_Static_MW");
    let index = nif.blocks_of::<NiMaterialProperty>().indices().next().unwrap();
    let shared: *const NiObject = nif.block(index).unwrap();
    assert_eq!(addr(nif.block_mut(index)), Some(shared));
    let count = nif.blocks().count();
    for i in [count, usize::MAX] {
        assert!(nif.block_mut(i).is_none(), "{i}");
    }

    // Downcasting reaches the type's setters, and is the same block as block_of_mut.
    let material: *const NiMaterialProperty = nif.block_of::<NiMaterialProperty>(index).unwrap();
    assert_eq!(addr(nif.block_mut(index).unwrap().downcast_mut::<NiMaterialProperty>()), Some(material));
    assert_eq!(addr(nif.block_of_mut::<NiMaterialProperty>(index)), Some(material));
    nif.block_mut(index).unwrap().downcast_mut::<NiMaterialProperty>().unwrap().set_alpha(0.25);
    assert_eq!(nif.block_of::<NiMaterialProperty>(index).unwrap().alpha(), 0.25);

    // The wrong type gives `None`; `NiObject` always succeeds.
    assert!(nif.block_mut(index).unwrap().downcast_mut::<NiNode>().is_none());
    for i in 0..count {
        let block: *const NiObject = nif.block(i).unwrap();
        assert_eq!(addr(nif.block_mut(i).unwrap().downcast_mut::<NiObject>()), Some(block), "{i}");
    }
}

#[test]
fn upcast_mut() {
    use nifly::{
        objects::{NiAvObject, NiObjectNet},
        shaders::{BsLightingShaderProperty, BsShaderProperty},
    };

    // A subclass reaches its base type's setters, on the same object.
    let mut nif = load("Cube");
    let index = nif.blocks_of::<BsLightingShaderProperty>().indices().next().unwrap();
    let mut shader = nif.block_of_mut::<BsLightingShaderProperty>(index).unwrap();
    shader.as_mut().set_alpha(0.5);
    let base: Pin<&mut BsShaderProperty> = shader.upcast_mut();
    let base_addr = ptr::from_ref(&*base);
    base.set_uv_scale([2.0, 3.0]);
    let s = nif.block_of::<BsLightingShaderProperty>(index).unwrap();
    assert_eq!((s.alpha(), s.uv_scale()), (0.5, [2.0, 3.0]));
    let shared: &BsShaderProperty = s;
    assert!(ptr::eq(base_addr, shared));

    // Chained, up to NiObject, each the same object as Deref gives.
    let node = nif.root_node().map(|r| r.index()).unwrap();
    let n = nif.block_of_mut::<NiNode>(node).unwrap();
    let av: Pin<&mut NiAvObject> = n.upcast_mut();
    let net: Pin<&mut NiObjectNet> = av.upcast_mut();
    let obj: Pin<&mut NiObject> = net.upcast_mut();
    let obj_addr = ptr::from_ref(&*obj);
    assert!(ptr::eq(obj_addr, nif.block(node).unwrap()));
}
