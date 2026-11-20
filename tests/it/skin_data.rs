//! Tests for the skin blocks, NifFile::child_blocks_of and NiSkinData::set_transform_global_to_skin.

use crate::common::{load, upstream};
use std::io::Cursor;

use nifly::{
    file::NifFile,
    geometry::NiShape,
    nodes::NiNode,
    object3d::{Matrix3, Transform, Vec3},
    objects::NiObject,
    shaders::{NiProperty, NiShader},
    skin::*,
};

fn skin_data<'a>(nif: &'a NifFile, shape: &NiShape) -> Option<&'a NiSkinData> {
    let inst = nif.child_blocks_of::<NiSkinInstance>(shape).next()?;
    nif.child_blocks_of::<NiSkinData>(inst).next()
}

#[test]
fn child_blocks_of_filters_by_type() {
    let nif = load("Suzanne");
    let shape = nif.shapes().next().unwrap();
    let shader = nif.child_blocks_of::<NiShader>(shape).next().unwrap();
    assert_eq!(shader.block_name(), "BSLightingShaderProperty");
    let property = nif.child_blocks_of::<NiProperty>(shape).next().unwrap();
    assert_eq!(nif.block_index(property), nif.block_index(shader));

    let all: Vec<_> = nif
        .child_blocks(shape)
        .map(|b| nif.block_index(b))
        .collect();
    let any: Vec<_> = nif
        .child_blocks_of::<NiObject>(shape)
        .map(|b| nif.block_index(b))
        .collect();
    assert_eq!(all, any);

    // Suzanne's root has no child nodes; Icosphere's has Indirect1.
    let root = nif.root_node().unwrap();
    assert_eq!(nif.child_blocks_of::<NiNode>(&root).count(), 0);
    let ico = load("Icosphere");
    let names: Vec<_> = ico
        .child_blocks_of::<NiNode>(ico.root_node().unwrap().get())
        .map(|n| n.name())
        .collect();
    assert_eq!(names, ["Indirect1"]);
}

#[test]
fn bones_are_pointers_not_child_blocks() {
    let nif = load("Bones");
    let inst = nif
        .child_blocks_of::<NiSkinInstance>(nif.shapes().next().unwrap())
        .next()
        .unwrap();
    assert_eq!(nif.child_blocks_of::<NiNode>(inst).count(), 0);
    // Its refs are the data and the partition.
    let types: Vec<_> = nif.child_blocks(inst).map(|b| b.block_name()).collect();
    assert_eq!(types, ["NiSkinData", "NiSkinPartition"]);
}

#[test]
fn bones_skin_data() {
    let nif = load("Bones");
    let shape = nif.shapes().next().unwrap();
    let inst = nif
        .child_blocks_of::<NiBoneContainer>(shape)
        .next()
        .unwrap();
    assert!(inst.downcast::<NiSkinInstance>().is_some());
    assert!(inst.downcast::<BsSkinInstance>().is_none());

    let data = skin_data(&nif, shape).unwrap();
    assert_eq!(data.bone_count(), 3);
    assert!(data.has_vertex_weights());
    assert_eq!(
        Some(data.transform_global_to_skin()),
        nif.view_of(shape).unwrap().transform_global_to_skin()
    );
    for i in 0..3 {
        assert_eq!(
            data.transform_skin_to_bone(i),
            nif.view_of(shape).unwrap().transform_skin_to_bone(i)
        );
    }
    assert_eq!(data.transform_skin_to_bone(3), None);
}

#[test]
fn set_transform_global_to_skin() {
    let mut nif = load("Bones");
    let shape = nif.shapes().next().unwrap();
    let index = nif.block_index(skin_data(&nif, shape).unwrap()).unwrap();

    let new = Transform::new(Vec3::new(0.0, 0.0, -1.0), Matrix3::IDENTITY, 2.0);
    nif.block_of_mut::<NiSkinData>(index)
        .unwrap()
        .set_transform_global_to_skin(new);
    let shape = nif.shapes().next().unwrap();
    assert_eq!(
        nif.view_of(shape).unwrap().transform_global_to_skin(),
        Some(new)
    );

    let mut cur = Cursor::new(Vec::new());
    nif.save(&mut cur, None).unwrap();
    let saved = NifFile::load(cur.into_inner().as_slice(), None).unwrap();
    assert_eq!(
        saved
            .shapes()
            .views()
            .next()
            .unwrap()
            .transform_global_to_skin(),
        Some(new)
    );
}

#[test]
fn skinned_se_uses_ni_skin_data() {
    let nif = upstream("TestNifFile_Skinned_SE");
    for shape in nif.shapes() {
        let inst = nif
            .child_blocks_of::<NiBoneContainer>(shape)
            .next()
            .unwrap();
        assert!(inst.downcast::<BsDismemberSkinInstance>().is_some());
        let data = skin_data(&nif, shape).unwrap();
        assert_eq!(
            data.bone_count(),
            nif.view_of(shape).unwrap().bone_list().len()
        );
        assert_eq!(
            Some(data.transform_global_to_skin()),
            nif.view_of(shape).unwrap().transform_global_to_skin()
        );
    }
}

#[test]
fn skinned_fo4_has_no_ni_skin_data() {
    let nif = upstream("TestNifFile_Skinned_FO4");
    assert_eq!(nif.blocks_of::<NiSkinData>().count(), 0);
    for shape in nif.shapes() {
        let inst = nif
            .child_blocks_of::<NiBoneContainer>(shape)
            .next()
            .unwrap();
        assert!(inst.downcast::<BsSkinInstance>().is_some());
        assert!(nif.child_blocks_of::<BsSkinBoneData>(inst).next().is_some());
        assert_eq!(skin_data(&nif, shape).map(|_| ()), None);
        assert_eq!(nif.view_of(shape).unwrap().transform_global_to_skin(), None);
        // Skin-to-bone still works, through BSSkin::BoneData.
        for i in 0..nif.view_of(shape).unwrap().bone_list().len() {
            assert!(
                nif.view_of(shape)
                    .unwrap()
                    .transform_skin_to_bone(i)
                    .is_some(),
                "bone {i}"
            );
        }
    }
}

#[test]
fn older_versions_use_ni_skin_data() {
    for (file, bones) in [
        ("TestNifFile_Skinned_OB", 10),
        ("TestNifFile_Skinned_MW", 3),
    ] {
        let nif = upstream(file);
        let shape = nif.shapes().next().unwrap();
        let data = skin_data(&nif, shape).unwrap();
        assert_eq!(data.bone_count(), bones, "{file}");
        assert_eq!(
            Some(data.transform_global_to_skin()),
            nif.view_of(shape).unwrap().transform_global_to_skin(),
            "{file}"
        );
    }
}

#[test]
fn skin_data_without_vertex_weights() {
    let nif = upstream("TestNifFile_Skinned_NoNiSkinDataWeights");
    for shape in nif.shapes() {
        assert!(!skin_data(&nif, shape).unwrap().has_vertex_weights());
    }
}
