//! Tests for NifFile::update_partition_flags, BsDismemberSkinInstance's partitions and
//! PartitionFlags.

use crate::common::{load, upstream, reload};

use nifly::{
    file::{Index, NifFile},
    skin::{BsDismemberSkinInstance, PartitionFlags},
};

/// Each dismembered shape's index and its skin instance's index.
fn dismembered(nif: &NifFile) -> Vec<(Index, Index)> {
    nif.shapes()
        .indexed()
        .filter_map(|(i, s)| {
            let inst = nif.child_blocks_of::<BsDismemberSkinInstance>(s).indices().next()?;
            Some((i, inst))
        })
        .collect()
}

/// (flag bits, body part) of each partition of the skin instance at `inst`.
fn partitions(nif: &NifFile, inst: Index) -> Vec<(u16, u16)> {
    let d = nif.block_of::<BsDismemberSkinInstance>(inst).unwrap();
    (0..d.len()).map(|i| (d.flags(i).unwrap().bits(), d.body_part(i).unwrap())).collect()
}

fn clear_flags(nif: &mut NifFile, inst: Index) {
    let d = nif.block_of_mut::<BsDismemberSkinInstance>(inst).unwrap();
    assert!(d.set_flags(0, PartitionFlags::default()));
    assert_eq!(partitions(nif, inst), [(0, 32)]);
}

#[test]
fn restores_flags() {
    let mut nif = upstream("TestNifFile_Skinned_SE");
    let shapes = dismembered(&nif);
    assert_eq!(shapes.len(), 2);
    for &(shape, inst) in &shapes {
        // One partition, visible and starting a bone set, for body part 32.
        assert_eq!(partitions(&nif, inst), [(257, 32)]);
        let flags = nif.block_of::<BsDismemberSkinInstance>(inst).unwrap().flags(0).unwrap();
        assert_eq!(flags, PartitionFlags::EDITOR_VISIBLE | PartitionFlags::START_BONE_SET);

        clear_flags(&mut nif, inst);
        assert!(nif.edit::<nifly::geometry::NiShape>(shape).is_some_and(|mut e| e.update_partition_flags()));
        assert_eq!(partitions(&nif, inst), [(257, 32)]);
    }

    // Set flags are saved as they are; saving doesn't recalculate them.
    let (_, inst) = shapes[0];
    clear_flags(&mut nif, inst);
    let mut saved = reload(&mut nif);
    let saved_shapes = dismembered(&saved);
    assert_eq!(partitions(&saved, saved_shapes[0].1), [(0, 32)]);
    assert_eq!(partitions(&saved, saved_shapes[1].1), [(257, 32)]);
    assert!(saved.edit::<nifly::geometry::NiShape>(saved_shapes[0].0).is_some_and(|mut e| e.update_partition_flags()));
    assert_eq!(partitions(&reload(&mut saved), saved_shapes[0].1), [(257, 32)]);
}

#[test]
fn update_skin_partitions_also_restores_them() {
    let mut nif = upstream("TestNifFile_Skinned_SE");
    let (shape, inst) = dismembered(&nif)[0];
    clear_flags(&mut nif, inst);
    assert!(nif.edit::<nifly::geometry::NiShape>(shape).is_some_and(|mut e| e.update_skin_partitions()));
    assert_eq!(partitions(&nif, inst), [(257, 32)]);
}

#[test]
fn false_without_dismember_partitions() {
    // Plain NiSkinInstance (Skyrim LE, Oblivion), Fallout 4 skins, unskinned shapes.
    for (file, mut nif) in [
        ("Bones", load("Bones")),
        ("Cube", load("Cube")),
        ("Skinned_OB", upstream("TestNifFile_Skinned_OB")),
        ("Skinned_FO4", upstream("TestNifFile_Skinned_FO4")),
    ] {
        let shapes: Vec<Index> = nif.shapes().indices().collect();
        assert!(!shapes.is_empty());
        for shape in shapes {
            assert!(!nif.edit::<nifly::geometry::NiShape>(shape).is_some_and(|mut e| e.update_partition_flags()), "{file} {shape}");
        }
    }

    let mut nif = upstream("TestNifFile_Skinned_SE");
    let (_, inst) = dismembered(&nif)[0];
    let count = nif.blocks().count();
    for index in [0, inst, count, usize::MAX] {
        assert!(!nif.edit::<nifly::geometry::NiShape>(index).is_some_and(|mut e| e.update_partition_flags()), "{index}");
    }
}

#[test]
fn partition_accessors_out_of_range() {
    let mut nif = upstream("TestNifFile_Skinned_SE");
    let (_, inst) = dismembered(&nif)[0];
    let d = nif.block_of::<BsDismemberSkinInstance>(inst).unwrap();
    assert_eq!(d.len(), 1);
    assert!(!d.is_empty());
    for index in [1, usize::MAX] {
        assert_eq!(d.flags(index), None);
        assert_eq!(d.body_part(index), None);
    }
    let d = nif.block_of_mut::<BsDismemberSkinInstance>(inst).unwrap();
    assert!(!d.set_flags(1, PartitionFlags::default()));
    assert!(!d.set_flags(usize::MAX, PartitionFlags::default()));
    assert_eq!(partitions(&nif, inst), [(257, 32)]);
}

#[test]
fn unnamed_partition_bits_are_kept() {
    let f = PartitionFlags::from_bits_retain(0x8002) | PartitionFlags::EDITOR_VISIBLE | PartitionFlags::START_BONE_SET;
    assert_eq!(f.bits(), 0x8103);
    assert_eq!((f - PartitionFlags::EDITOR_VISIBLE).bits(), 0x8102);
}
