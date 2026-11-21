//! Tests for NifFile::update_skin_partitions and NiSkinPartition's accessors.

use crate::common::{load, reload, shape_index, upstream};
use std::collections::BTreeSet;

use nifly::{
    file::{Index, NifFile},
    geometry::NiShape,
    skin::{NiSkinInstance, NiSkinPartition},
};

fn partition<'a>(nif: &'a NifFile, shape: &NiShape) -> Option<&'a NiSkinPartition> {
    let inst = nif.child_blocks_of::<NiSkinInstance>(shape).next()?;
    nif.child_blocks_of::<NiSkinPartition>(inst).next()
}

/// (bones, triangle count, vertex map length) of each partition.
type Summary = Vec<(Vec<u16>, usize, usize)>;

fn summary(part: &NiSkinPartition) -> Summary {
    (0..part.len())
        .map(|i| {
            (
                part.bones(i).unwrap().to_vec(),
                part.triangle_count(i).unwrap(),
                part.vertex_map(i).unwrap().len(),
            )
        })
        .collect()
}

/// The summary of the shape named `name`.
fn shape_summary(nif: &NifFile, name: &str) -> Summary {
    let shape = nif.shapes().find(|s| s.name() == name).unwrap();
    summary(partition(nif, shape).unwrap())
}

#[test]
fn rebuilds_after_weight_change() {
    let mut nif = load("Bones");
    let shape = shape_index(&nif, "Limb");
    assert_eq!(shape_summary(&nif, "Limb"), [(vec![0, 1, 2], 252, 164)]);

    // Unchanged weights give the same partition.
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.update_skin_partitions())
    );
    assert_eq!(shape_summary(&nif, "Limb"), [(vec![0, 1, 2], 252, 164)]);

    // Once bone 2 has no weights, the partition stops using it.
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.set_bone_weights(2, []))
    );
    assert_eq!(shape_summary(&nif, "Limb"), [(vec![0, 1, 2], 252, 164)]);
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.update_skin_partitions())
    );
    assert_eq!(shape_summary(&nif, "Limb"), [(vec![0, 1], 252, 164)]);

    let saved = reload(&mut nif);
    assert_eq!(shape_summary(&saved, "Limb"), [(vec![0, 1], 252, 164)]);
}

#[test]
fn converts_strips_to_triangles() {
    let mut nif = upstream("TestNifFile_Skinned_OB");
    let shape = shape_index(&nif, "Cuirass");
    let cuirass = nif.block_of::<NiShape>(shape).unwrap();
    assert!(!nif.view_of(cuirass).unwrap().is_sse_compatible());
    // Strip partitions count their degenerate triangles too.
    assert_eq!(shape_summary(&nif, "Cuirass")[0].1, 2851);

    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.update_skin_partitions())
    );
    let cuirass = nif.block_of::<NiShape>(shape).unwrap();
    assert!(nif.view_of(cuirass).unwrap().is_sse_compatible());
    let after = shape_summary(&nif, "Cuirass");
    assert_eq!(after.len(), 1);
    assert_eq!((after[0].1, after[0].2), (1638, 1354));
    // Every bone is still used (Oblivion allows 18 per partition; this shape has 10).
    assert_eq!(after[0].0, (0..10).collect::<Vec<u16>>());

    let saved = reload(&mut nif);
    assert_eq!(shape_summary(&saved, "Cuirass"), after);
}

#[test]
fn dismembered_bs_tri_shapes() {
    let mut nif = upstream("TestNifFile_Skinned_SE");
    let names = ["cylinder_1", "cylinder_2"];
    let before: Vec<Summary> = names.iter().map(|n| shape_summary(&nif, n)).collect();
    for name in names {
        assert!(
            nif.edit::<nifly::geometry::NiShape>(shape_index(&nif, name))
                .is_some_and(|mut e| e.update_skin_partitions()),
            "{name}"
        );
    }
    let after: Vec<Summary> = names.iter().map(|n| shape_summary(&nif, n)).collect();
    assert_eq!(after, before);
    assert_eq!(after[0], [(vec![0, 1], 68, 136)]);

    let saved = reload(&mut nif);
    let saved: Vec<Summary> = names.iter().map(|n| shape_summary(&saved, n)).collect();
    assert_eq!(saved, before);
}

#[test]
fn false_changes_nothing() {
    // Skin data without vertex weights would leave every partition without bones.
    let mut nif = upstream("TestNifFile_Skinned_NoNiSkinDataWeights");
    for name in ["cylinder_1", "cylinder_2"] {
        let before = shape_summary(&nif, name);
        assert_eq!(before[0].0, [0, 1]);
        assert!(
            !nif.edit::<nifly::geometry::NiShape>(shape_index(&nif, name))
                .is_some_and(|mut e| e.update_skin_partitions()),
            "{name}"
        );
        assert_eq!(shape_summary(&nif, name), before);
    }

    // No partition (Morrowind), a skin without NiSkinInstance (Fallout 4), unskinned shapes.
    for (file, mut nif) in [
        ("Skinned_MW", upstream("TestNifFile_Skinned_MW")),
        ("Skinned_FO4", upstream("TestNifFile_Skinned_FO4")),
        ("Cube", load("Cube")),
        ("Suzanne", load("Suzanne")),
    ] {
        let shapes: Vec<Index> = nif.shapes().indices().collect();
        assert!(!shapes.is_empty());
        for shape in shapes {
            assert!(
                !nif.edit::<nifly::geometry::NiShape>(shape)
                    .is_some_and(|mut e| e.update_skin_partitions()),
                "{file} {shape}"
            );
        }
    }

    // Not a shape, or out of range.
    let mut nif = load("Bones");
    let count = nif.blocks().count();
    for index in [0, count, usize::MAX] {
        assert!(
            !nif.edit::<nifly::geometry::NiShape>(index)
                .is_some_and(|mut e| e.update_skin_partitions()),
            "{index}"
        );
    }
}

#[test]
fn partition_accessors() {
    let nif = load("Bones");
    let shape = nif.shapes().find(|s| s.name() == "Limb").unwrap();
    let part = partition(&nif, shape).unwrap();
    assert_eq!(part.len(), 1);
    assert!(!part.is_empty());
    for index in [1, usize::MAX] {
        assert_eq!(part.bones(index), None);
        assert_eq!(part.vertex_map(index), None);
        assert_eq!(part.triangle_count(index), None);
    }

    // The vertex map lists each vertex once, and bones are positions in the bone list.
    let map = part.vertex_map(0).unwrap();
    assert_eq!(map.iter().collect::<BTreeSet<_>>().len(), map.len());
    let bone_count = nif.view_of(shape).unwrap().bone_list().len();
    assert!(
        part.bones(0)
            .unwrap()
            .iter()
            .all(|b| usize::from(*b) < bone_count)
    );
}
