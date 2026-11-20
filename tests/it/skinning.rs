//! Tests for NifFile::create_skinning and NifFile::delete_skinning.

use crate::common::{load, upstream, reload};

use nifly::{
    file::{Index, NifFile},
    geometry::NiShape,
    nodes::NiNode,
    shaders::BsShaderProperty,
    skin::{BsDismemberSkinInstance, NiBoneContainer, NiSkinPartition},
};

const SKINNED_FLAG: u32 = 1 << 1;

fn shape<'a>(nif: &'a NifFile, name: &str) -> &'a NiShape {
    nif.shapes().find(|s| s.name() == name).unwrap()
}

fn index(nif: &NifFile, name: &str) -> Index {
    nif.shapes().indexed().find(|(_, s)| s.name() == name).unwrap().0
}

/// The block names of the shape's skin instance and the blocks it refers to.
fn skin_blocks(nif: &NifFile, name: &str) -> Vec<String> {
    let s = shape(nif, name);
    let Some(skin) = nif.child_blocks_of::<NiBoneContainer>(s).next() else {
        return vec![];
    };
    let mut names = vec![skin.block_name().to_string()];
    names.extend(nif.child_blocks(skin).map(|b| b.block_name().to_string()));
    names
}

fn shader_skinned(nif: &NifFile, name: &str) -> bool {
    let shader = nif.view_of(shape(nif, name)).unwrap().shader().unwrap();
    shader.downcast::<BsShaderProperty>().unwrap().raw_flags_1() & SKINNED_FLAG != 0
}

#[test]
fn creates_for_skyrim_se() {
    let mut nif = load("Cube");
    let names: Vec<String> = nif.blocks().map(|b| b.block_name().to_string()).collect();
    assert!(skin_blocks(&nif, "Cube").is_empty());
    assert!(!shader_skinned(&nif, "Cube"));

    assert!(nif.edit::<nifly::geometry::NiShape>(index(&nif, "Cube")).is_some_and(|mut e| e.create_skinning()));
    // New blocks go at the end.
    let after: Vec<String> = nif.blocks().map(|b| b.block_name().to_string()).collect();
    assert_eq!(after[..names.len()], names[..]);

    let saved = reload(&mut nif);
    for nif in [&nif, &saved] {
        assert_eq!(skin_blocks(nif, "Cube"), ["BSDismemberSkinInstance", "NiSkinData", "NiSkinPartition"]);
        assert!(shader_skinned(nif, "Cube"));
        let s = shape(nif, "Cube");
        assert!(nif.view_of(s).unwrap().bone_list().is_empty());

        // One partition over the whole cube, for body part 32.
        let skin = nif.child_blocks_of::<BsDismemberSkinInstance>(s).next().unwrap();
        assert_eq!((skin.len(), skin.body_part(0)), (1, Some(32)));
        let part = nif.child_blocks_of::<NiSkinPartition>(skin).next().unwrap();
        assert_eq!((part.len(), part.triangle_count(0), part.vertex_map(0).map(<[u16]>::len)), (1, Some(12), Some(14)));
    }

    // The shape now stores weights per vertex (none, with no bones yet).
    let cube = index(&nif, "Cube");
    assert!(nif.edit::<nifly::geometry::NiShape>(cube).is_some_and(|mut e| e.set_vertex_weights(0, &[])));
}

#[test]
fn creates_for_other_versions() {
    // Morrowind: no partition is saved.
    let mut nif = upstream("TestNifFile_Static_MW");
    assert!(nif.edit::<nifly::geometry::NiShape>(index(&nif, "Tri EditorMarker_box_02")).is_some_and(|mut e| e.create_skinning()));
    let name = "Tri EditorMarker_box_02";
    assert_eq!(skin_blocks(&nif, name), ["NiSkinInstance", "NiSkinData", "NiSkinPartition"]);
    assert_eq!(skin_blocks(&reload(&mut nif), name), ["NiSkinInstance", "NiSkinData"]);

    // Fallout 4.
    let mut nif = upstream("TestNifFile_Static_FO4");
    assert!(nif.edit::<nifly::geometry::NiShape>(index(&nif, "cylinder_1")).is_some_and(|mut e| e.create_skinning()));
    let saved = reload(&mut nif);
    for nif in [&nif, &saved] {
        assert_eq!(skin_blocks(nif, "cylinder_1"), ["BSSkin::Instance", "BSSkin::BoneData"]);
        assert!(shader_skinned(nif, "cylinder_1"));
        assert!(skin_blocks(nif, "cylinder_2").is_empty());
    }
}

#[test]
fn create_false_changes_nothing() {
    for (file, mut nif) in [
        ("Skinned_SE", upstream("TestNifFile_Skinned_SE")),
        ("SF", upstream("TestNifFile_SF")),
        ("Bones", load("Bones")),
    ] {
        let count = nif.blocks().count();
        let shapes: Vec<Index> = nif.shapes().indices().collect();
        for s in shapes {
            assert!(!nif.edit::<nifly::geometry::NiShape>(s).is_some_and(|mut e| e.create_skinning()), "{file} {s}: already skinned");
        }
        assert_eq!(nif.blocks().count(), count, "{file}");
    }

    let mut nif = upstream("TestNifFile_Particles_MW");
    assert!(!nif.edit::<nifly::geometry::NiShape>(index(&nif, "Blizzard")).is_some_and(|mut e| e.create_skinning()));

    let mut nif = load("Cube");
    let count = nif.blocks().count();
    for i in [0, count, usize::MAX] {
        assert!(!nif.edit::<nifly::geometry::NiShape>(i).is_some_and(|mut e| e.create_skinning()), "{i}");
    }
    assert_eq!(nif.blocks().count(), count);
}

#[test]
fn deletes_only_this_shapes_skin() {
    let mut nif = upstream("TestNifFile_Skinned_SE");
    let (count, nodes) = (nif.blocks().count(), nif.nodes().count());
    let other = skin_blocks(&nif, "cylinder_2");
    assert!(!other.is_empty());

    assert!(nif.edit::<nifly::geometry::NiShape>(index(&nif, "cylinder_1")).is_some_and(|mut e| e.delete_skinning()));
    let saved = reload(&mut nif);
    for nif in [&nif, &saved] {
        assert!(skin_blocks(nif, "cylinder_1").is_empty());
        assert!(!shader_skinned(nif, "cylinder_1"));
        assert!(nif.view_of(shape(nif, "cylinder_1")).unwrap().bone_list().is_empty());
        assert_eq!(skin_blocks(nif, "cylinder_2"), other);
        assert!(shader_skinned(nif, "cylinder_2"));
    }
    // The skin instance, data and partition are gone; bone nodes stay.
    assert_eq!(nif.blocks().count(), count - 3);
    assert_eq!(nif.nodes().count(), nodes);
    assert_eq!(nif.blocks_of::<NiNode>().count(), nodes);

    // The unskinned shape no longer takes per-vertex weights.
    let cylinder = index(&nif, "cylinder_1");
    assert!(!nif.edit::<nifly::geometry::NiShape>(cylinder).is_some_and(|mut e| e.set_vertex_weights(0, &[])));
}

#[test]
fn delete_other_kinds() {
    // NiTriShape (Skyrim LE).
    let mut nif = load("Bones");
    let count = nif.blocks().count();
    assert!(nif.edit::<nifly::geometry::NiShape>(index(&nif, "Limb")).is_some_and(|mut e| e.delete_skinning()));
    assert!(skin_blocks(&nif, "Limb").is_empty());
    assert_eq!(nif.blocks().count(), count - 3);

    // Starfield: the skin instance, bone data and SkinAttach extra data.
    let mut nif = upstream("TestNifFile_SF");
    let count = nif.blocks().count();
    let extra = |nif: &NifFile| -> Vec<String> {
        let s = nif.shapes().next().unwrap();
        nif.view_of(s).unwrap().extra_data().map(|b| b.block_name().to_string()).collect()
    };
    assert_eq!(extra(&nif), ["NiIntegerExtraData", "SkinAttach"]);
    let geometry = nif.shapes().indices().next().unwrap();
    assert!(nif.edit::<nifly::geometry::NiShape>(geometry).is_some_and(|mut e| e.delete_skinning()));
    let s = nif.shapes().next().unwrap();
    assert!(nif.child_blocks_of::<NiBoneContainer>(s).next().is_none());
    // Other extra data stays.
    assert_eq!(extra(&nif), ["NiIntegerExtraData"]);
    assert_eq!(nif.blocks().count(), count - 3);
}

#[test]
fn create_after_delete() {
    let mut nif = load("Cube");
    let cube = index(&nif, "Cube");
    let count = nif.blocks().count();
    assert!(nif.edit::<nifly::geometry::NiShape>(cube).is_some_and(|mut e| e.create_skinning()));
    assert!(nif.edit::<nifly::geometry::NiShape>(cube).is_some_and(|mut e| e.delete_skinning()));
    assert_eq!(nif.blocks().count(), count);
    assert!(!shader_skinned(&nif, "Cube"));
    assert!(nif.edit::<nifly::geometry::NiShape>(cube).is_some_and(|mut e| e.create_skinning()));
    assert_eq!(skin_blocks(&nif, "Cube"), ["BSDismemberSkinInstance", "NiSkinData", "NiSkinPartition"]);
}

#[test]
fn delete_false_changes_nothing() {
    let mut nif = load("Cube");
    let count = nif.blocks().count();
    assert!(!nif.edit::<nifly::geometry::NiShape>(index(&nif, "Cube")).is_some_and(|mut e| e.delete_skinning()));
    for i in [0, count, usize::MAX] {
        assert!(!nif.edit::<nifly::geometry::NiShape>(i).is_some_and(|mut e| e.delete_skinning()), "{i}");
    }
    assert_eq!(nif.blocks().count(), count);
}
