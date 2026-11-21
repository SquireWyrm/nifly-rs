//! Tests for NifFile::shape_partitions, set_shape_partitions, set_default_partition,
//! delete_partitions and remove_empty_partitions.

use crate::common::{load, reload, shape, shape_index, upstream};

use nifly::{
    file::{Index, NifFile},
    skin::{NiBoneContainer, NiSkinInstance, NiSkinPartition, ShapePartitions},
};

fn partitions(nif: &NifFile, name: &str) -> ShapePartitions {
    nif.view_of(shape(nif, name)).unwrap().partitions().unwrap()
}

/// (flags, body part) of each partition.
fn info(p: &ShapePartitions) -> Vec<(u16, u16)> {
    p.partitions
        .iter()
        .map(|i| (i.flags.bits(), i.body_part))
        .collect()
}

/// How many triangles each partition has.
fn counts(p: &ShapePartitions) -> Vec<usize> {
    (0..p.partitions.len())
        .map(|i| p.triangles.iter().filter(|t| **t == Some(i)).count())
        .collect()
}

/// Each triangle in partition 0 for the first half, `rest` for the second.
fn halves(n: usize, rest: Option<usize>) -> Vec<Option<usize>> {
    (0..n)
        .map(|t| if t < n / 2 { Some(0) } else { rest })
        .collect()
}

fn skin_partition<'a>(nif: &'a NifFile, name: &str) -> &'a NiSkinPartition {
    let inst = nif
        .child_blocks_of::<NiSkinInstance>(shape(nif, name))
        .next()
        .unwrap();
    nif.child_blocks_of::<NiSkinPartition>(inst).next().unwrap()
}

#[test]
fn reads_partitions() {
    let nif = upstream("TestNifFile_Skinned_SE");
    let p = partitions(&nif, "cylinder_1");
    assert_eq!(info(&p), [(257, 32)]);
    assert_eq!(p.triangles, vec![Some(0); 68]);

    // Without a dismember instance: default info (body part 0 before Skyrim).
    let nif = upstream("TestNifFile_Skinned_OB");
    let p = partitions(&nif, "Cuirass");
    assert_eq!(info(&p), [(1, 0)]);
    assert_eq!(counts(&p), [1638]);
    // Reading leaves the strip partition as it is.
    assert!(
        !nif.view_of(shape(&nif, "Cuirass"))
            .unwrap()
            .is_sse_compatible()
    );

    // Not skinned, or skinned without an NiSkinInstance (Fallout 4).
    let nif = load("Cube");
    assert!(
        nif.view_of(shape(&nif, "Cube"))
            .unwrap()
            .partitions()
            .is_none()
    );
    let nif = upstream("TestNifFile_Skinned_FO4");
    assert!(
        nif.shapes()
            .all(|s| nif.view_of(s).unwrap().partitions().is_none())
    );
}

#[test]
fn splits_and_rebuilds() {
    let mut nif = upstream("TestNifFile_Skinned_SE");
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape_index(&nif, "cylinder_1"))
            .is_some_and(|mut e| e.set_partitions(&[32, 34], &halves(68, Some(1))))
    );
    let saved = reload(&mut nif);
    for nif in [&nif, &saved] {
        let p = partitions(nif, "cylinder_1");
        // Flags are recalculated: the second partition shares the first's bones.
        assert_eq!(info(&p), [(257, 32), (1, 34)]);
        assert_eq!(counts(&p), [34, 34]);
        // The partitions themselves are rebuilt.
        let part = skin_partition(nif, "cylinder_1");
        assert_eq!(part.len(), 2);
        for i in 0..2 {
            assert_eq!(part.triangle_count(i), Some(34));
            assert!(!part.vertex_map(i).unwrap().is_empty());
            assert!(!part.bones(i).unwrap().is_empty());
        }
        // The other shape is untouched.
        assert_eq!(info(&partitions(nif, "cylinder_2")), [(257, 32)]);
    }
}

#[test]
fn unassigned_triangles_get_their_own_partition() {
    let mut nif = upstream("TestNifFile_Skinned_SE");
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape_index(&nif, "cylinder_1"))
            .is_some_and(|mut e| e.set_partitions(&[34], &halves(68, None)))
    );
    let p = partitions(&nif, "cylinder_1");
    assert_eq!(info(&p), [(257, 34), (1, 32)]);
    assert_eq!(counts(&p), [34, 34]);
}

#[test]
fn skyrim_le_gets_a_dismember_instance() {
    let mut nif = load("Bones");
    let skin_index = |nif: &NifFile| {
        let inst = nif
            .child_blocks_of::<NiBoneContainer>(shape(nif, "Limb"))
            .next()
            .unwrap();
        (
            nif.block_index(inst).unwrap(),
            inst.block_name().to_string(),
        )
    };
    let (before, name) = skin_index(&nif);
    assert_eq!(name, "NiSkinInstance");
    let count = nif.blocks().count();

    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape_index(&nif, "Limb"))
            .is_some_and(|mut e| e.set_partitions(&[32, 34], &halves(252, Some(1))))
    );
    assert_eq!(
        skin_index(&nif),
        (before, "BSDismemberSkinInstance".to_string())
    );
    assert_eq!(nif.blocks().count(), count);
    let saved = reload(&mut nif);
    assert_eq!(info(&partitions(&saved, "Limb")), [(257, 32), (1, 34)]);
    assert_eq!(
        saved
            .view_of(shape(&saved, "Limb"))
            .unwrap()
            .bone_list()
            .len(),
        3
    );
}

#[test]
fn oblivion_has_no_body_parts() {
    let mut nif = upstream("TestNifFile_Skinned_OB");
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape_index(&nif, "Cuirass"))
            .is_some_and(|mut e| e.set_partitions(&[5, 6], &halves(1638, Some(1))))
    );
    let p = partitions(&nif, "Cuirass");
    assert_eq!(info(&p), [(1, 0), (1, 0)]);
    assert_eq!(counts(&p), [819, 819]);
}

#[test]
fn false_changes_nothing() {
    let mut nif = upstream("TestNifFile_Skinned_SE");
    let cylinder = shape_index(&nif, "cylinder_1");
    let before = partitions(&nif, "cylinder_1");
    // Wrong length, or an index past the body parts.
    for tris in [vec![Some(0); 67], vec![Some(0); 69], halves(68, Some(2))] {
        assert!(
            !nif.edit::<nifly::geometry::NiShape>(cylinder)
                .is_some_and(|mut e| e.set_partitions(&[32, 34], &tris))
        );
    }
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(cylinder)
            .is_some_and(|mut e| e.set_partitions(&[], &vec![Some(0); 68]))
    );
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(cylinder)
            .is_some_and(|mut e| e.set_partitions(&[32], &[Some(usize::MAX); 68]))
    );
    assert_eq!(partitions(&nif, "cylinder_1"), before);

    // Partitions that can't be rebuilt: skin data without vertex weights.
    let mut nif = upstream("TestNifFile_Skinned_NoNiSkinDataWeights");
    let cylinder = shape_index(&nif, "cylinder_1");
    let before = partitions(&nif, "cylinder_1");
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(cylinder)
            .is_some_and(|mut e| e.set_partitions(&[32], &vec![Some(0); 68]))
    );
    assert_eq!(partitions(&nif, "cylinder_1"), before);

    // Not skinned, Fallout 4 skin, not a shape, out of range.
    let mut nif = load("Cube");
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(shape_index(&nif, "Cube"))
            .is_some_and(|mut e| e.set_partitions(&[32], &[Some(0); 12]))
    );
    let mut nif = upstream("TestNifFile_Skinned_FO4");
    let fo4 = nif.shapes().indices().next().unwrap();
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(fo4)
            .is_some_and(|mut e| e.set_partitions(&[32], &[]))
    );
    for i in [0, nif.blocks().count(), usize::MAX] {
        assert!(
            !nif.edit::<nifly::geometry::NiShape>(i)
                .is_some_and(|mut e| e.set_partitions(&[32], &[])),
            "{i}"
        );
    }
}

/// cylinder_1 split into thirds with body parts 32, 34 and 36.
fn thirds() -> (NifFile, Index) {
    let mut nif = upstream("TestNifFile_Skinned_SE");
    let cylinder = shape_index(&nif, "cylinder_1");
    let tris: Vec<Option<usize>> = (0..68).map(|t| Some(t * 3 / 68)).collect();
    assert!(
        nif.edit::<nifly::geometry::NiShape>(cylinder)
            .is_some_and(|mut e| e.set_partitions(&[32, 34, 36], &tris))
    );
    assert_eq!(counts(&partitions(&nif, "cylinder_1")), [23, 23, 22]);
    (nif, cylinder)
}

#[test]
fn deletes_partitions() {
    let (mut nif, cylinder) = thirds();
    // Order and repeats don't matter.
    assert!(
        nif.edit::<nifly::geometry::NiShape>(cylinder)
            .is_some_and(|mut e| e.delete_partitions(&[2, 0, 2]))
    );
    let p = partitions(&nif, "cylinder_1");
    assert_eq!(info(&p), [(257, 34)]);
    assert_eq!(counts(&p), [23]);
    // The deleted partitions' triangles stay in the shape, in no partition.
    assert_eq!(p.triangles.iter().filter(|t| t.is_none()).count(), 45);
    assert_eq!(
        nif.view_of(shape(&nif, "cylinder_1"))
            .unwrap()
            .triangles()
            .unwrap()
            .len(),
        68
    );

    // A skinned Skyrim SE BSTriShape saves its triangles in its partitions, so they're dropped.
    let saved = reload(&mut nif);
    let p = partitions(&saved, "cylinder_1");
    assert_eq!(p.triangles, vec![Some(0); 23]);

    // Updating puts them in the first partition.
    assert!(
        nif.edit::<nifly::geometry::NiShape>(cylinder)
            .is_some_and(|mut e| e.update_skin_partitions())
    );
    assert_eq!(counts(&partitions(&nif, "cylinder_1")), [68]);
}

#[test]
fn deleted_partition_triangles_kept_by_shape_data() {
    // An NiTriShape keeps its triangles in its own data, so they survive a save.
    let mut nif = load("Bones");
    let limb = shape_index(&nif, "Limb");
    assert!(
        nif.edit::<nifly::geometry::NiShape>(limb)
            .is_some_and(|mut e| e.set_partitions(&[32, 34], &halves(252, Some(1))))
    );
    assert!(
        nif.edit::<nifly::geometry::NiShape>(limb)
            .is_some_and(|mut e| e.delete_partitions(&[1]))
    );
    let saved = reload(&mut nif);
    for nif in [&nif, &saved] {
        let p = partitions(nif, "Limb");
        assert_eq!(counts(&p), [126]);
        assert_eq!(p.triangles.iter().filter(|t| t.is_none()).count(), 126);
    }
}

#[test]
fn deleting_every_partition() {
    let mut nif = upstream("TestNifFile_Skinned_SE");
    let cylinder = shape_index(&nif, "cylinder_1");
    assert!(
        nif.edit::<nifly::geometry::NiShape>(cylinder)
            .is_some_and(|mut e| e.delete_partitions(&[0]))
    );
    let p = partitions(&nif, "cylinder_1");
    assert!(p.partitions.is_empty());
    assert!(p.triangles.iter().all(Option::is_none));
    // With triangles but no partitions there's nothing to update; a default partition restores one.
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(cylinder)
            .is_some_and(|mut e| e.update_skin_partitions())
    );
    assert!(
        nif.edit::<nifly::geometry::NiShape>(cylinder)
            .is_some_and(|mut e| e.set_default_partition())
    );
    assert_eq!(info(&partitions(&nif, "cylinder_1")), [(257, 32)]);
    assert_eq!(counts(&partitions(&nif, "cylinder_1")), [68]);
}

#[test]
fn delete_false_changes_nothing() {
    let (mut nif, cylinder) = thirds();
    let before = partitions(&nif, "cylinder_1");
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(cylinder)
            .is_some_and(|mut e| e.delete_partitions(&[0, 3]))
    );
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(cylinder)
            .is_some_and(|mut e| e.delete_partitions(&[usize::MAX]))
    );
    assert_eq!(partitions(&nif, "cylinder_1"), before);
    // An empty list deletes nothing.
    assert!(
        nif.edit::<nifly::geometry::NiShape>(cylinder)
            .is_some_and(|mut e| e.delete_partitions(&[]))
    );
    assert_eq!(partitions(&nif, "cylinder_1"), before);

    let mut nif = load("Cube");
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(shape_index(&nif, "Cube"))
            .is_some_and(|mut e| e.delete_partitions(&[]))
    );
    let mut nif = upstream("TestNifFile_Skinned_FO4");
    let fo4 = nif.shapes().indices().next().unwrap();
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(fo4)
            .is_some_and(|mut e| e.delete_partitions(&[]))
    );
    for i in [0, nif.blocks().count(), usize::MAX] {
        assert!(
            !nif.edit::<nifly::geometry::NiShape>(i)
                .is_some_and(|mut e| e.delete_partitions(&[])),
            "{i}"
        );
    }
}

#[test]
fn default_partition() {
    let (mut nif, cylinder) = thirds();
    assert!(
        nif.edit::<nifly::geometry::NiShape>(cylinder)
            .is_some_and(|mut e| e.set_default_partition())
    );
    let saved = reload(&mut nif);
    for nif in [&nif, &saved] {
        let p = partitions(nif, "cylinder_1");
        assert_eq!(info(&p), [(257, 32)]);
        assert_eq!(p.triangles, vec![Some(0); 68]);
        assert_eq!(skin_partition(nif, "cylinder_1").len(), 1);
    }

    // Body part 0 before Skyrim; Skyrim LE gets a dismember instance.
    let mut nif = upstream("TestNifFile_Skinned_OB");
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape_index(&nif, "Cuirass"))
            .is_some_and(|mut e| e.set_partitions(&[0, 0], &halves(1638, Some(1))))
    );
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape_index(&nif, "Cuirass"))
            .is_some_and(|mut e| e.set_default_partition())
    );
    assert_eq!(counts(&partitions(&nif, "Cuirass")), [1638]);
    let mut nif = load("Bones");
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape_index(&nif, "Limb"))
            .is_some_and(|mut e| e.set_default_partition())
    );
    assert_eq!(info(&partitions(&nif, "Limb")), [(257, 32)]);
    let inst = nif
        .child_blocks_of::<NiBoneContainer>(shape(&nif, "Limb"))
        .next()
        .unwrap();
    assert_eq!(inst.block_name(), "BSDismemberSkinInstance");

    // Same conditions as set_shape_partitions.
    let mut nif = upstream("TestNifFile_Skinned_NoNiSkinDataWeights");
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(shape_index(&nif, "cylinder_1"))
            .is_some_and(|mut e| e.set_default_partition())
    );
    let mut nif = load("Cube");
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(shape_index(&nif, "Cube"))
            .is_some_and(|mut e| e.set_default_partition())
    );
    for i in [0, nif.blocks().count(), usize::MAX] {
        assert!(
            !nif.edit::<nifly::geometry::NiShape>(i)
                .is_some_and(|mut e| e.set_default_partition()),
            "{i}"
        );
    }
}

#[test]
fn removes_empty_partitions() {
    // Partition 1 gets no triangles.
    let mut nif = upstream("TestNifFile_Skinned_SE");
    let cylinder = shape_index(&nif, "cylinder_1");
    let tris: Vec<Option<usize>> = (0..68).map(|t| Some(if t < 34 { 0 } else { 2 })).collect();
    assert!(
        nif.edit::<nifly::geometry::NiShape>(cylinder)
            .is_some_and(|mut e| e.set_partitions(&[32, 34, 36], &tris))
    );
    let p = partitions(&nif, "cylinder_1");
    assert_eq!((info(&p).len(), counts(&p)), (3, vec![34, 0, 34]));
    assert_eq!(
        skin_partition(&nif, "cylinder_1").triangle_count(1),
        Some(0)
    );

    assert_eq!(
        nif.edit::<nifly::geometry::NiShape>(cylinder)
            .and_then(|mut e| e.remove_empty_partitions()),
        Some(1)
    );
    let saved = reload(&mut nif);
    for nif in [&nif, &saved] {
        let p = partitions(nif, "cylinder_1");
        assert_eq!(info(&p).iter().map(|i| i.1).collect::<Vec<_>>(), [32, 36]);
        assert_eq!(counts(&p), [34, 34]);
    }
    // Nothing more to remove.
    assert_eq!(
        nif.edit::<nifly::geometry::NiShape>(cylinder)
            .and_then(|mut e| e.remove_empty_partitions()),
        Some(0)
    );
    assert_eq!(
        nif.edit::<nifly::geometry::NiShape>(shape_index(&nif, "cylinder_2"))
            .and_then(|mut e| e.remove_empty_partitions()),
        Some(0)
    );
}

#[test]
fn remove_empty_needs_partitions() {
    let mut nif = load("Cube");
    assert_eq!(
        nif.edit::<nifly::geometry::NiShape>(shape_index(&nif, "Cube"))
            .and_then(|mut e| e.remove_empty_partitions()),
        None
    );
    let mut nif = upstream("TestNifFile_Skinned_FO4");
    let fo4 = nif.shapes().indices().next().unwrap();
    assert_eq!(
        nif.edit::<nifly::geometry::NiShape>(fo4)
            .and_then(|mut e| e.remove_empty_partitions()),
        None
    );
    for i in [0, nif.blocks().count(), usize::MAX] {
        assert_eq!(
            nif.edit::<nifly::geometry::NiShape>(i)
                .and_then(|mut e| e.remove_empty_partitions()),
            None,
            "{i}"
        );
    }
}
