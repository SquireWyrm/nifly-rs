//! Tests for NifFile::delete_shape_vertices.

use crate::common::reload;
use std::{collections::BTreeMap, fs};

use nifly::{
    extra_data::{ExtraIntegers, NiIntegersExtraData},
    file::{Index, NifFile},
    skin::{NiSkinInstance, NiSkinPartition},
};

fn load(path: &str) -> NifFile {
    NifFile::load(fs::read(path).unwrap().as_slice(), None).unwrap()
}

fn first(nif: &NifFile) -> Index {
    nif.shapes().indices().next().unwrap()
}

/// The new index of each old vertex, `None` for deleted ones.
fn remap(count: usize, deleted: &[usize]) -> Vec<Option<u16>> {
    let mut next = 0;
    (0..count)
        .map(|v| {
            if deleted.contains(&v) {
                None
            } else {
                next += 1;
                Some(next - 1)
            }
        })
        .collect()
}

/// Triangles after deleting, dropping those that used a deleted vertex.
fn remapped(tris: &[[u16; 3]], map: &[Option<u16>]) -> Vec<[u16; 3]> {
    tris.iter()
        .filter_map(|t| {
            Some([
                map[t[0] as usize]?,
                map[t[1] as usize]?,
                map[t[2] as usize]?,
            ])
        })
        .collect()
}

#[test]
fn deletes_and_reindexes() {
    let mut nif = load("assets/Cube.nif");
    let cube = first(&nif);
    let s = nif.block_of::<nifly::geometry::NiShape>(cube).unwrap();
    let (vertices, uvs, tris) = (
        nif.view_of(s).unwrap().vertices().unwrap(),
        nif.view_of(s).unwrap().uvs().unwrap(),
        nif.view_of(s).unwrap().triangles().unwrap(),
    );
    let deleted = [0, 5];
    let map = remap(vertices.len(), &deleted);
    // Order and repeats don't matter.
    assert!(
        nif.edit::<nifly::geometry::NiShape>(cube)
            .is_some_and(|mut e| e.delete_vertices(&[5, 0, 5]))
    );

    let keep = |i: &usize| !deleted.contains(i);
    let saved = reload(&mut nif);
    for nif in [&nif, &saved] {
        let s = nif
            .block_of::<nifly::geometry::NiShape>(first(nif))
            .unwrap();
        let v: Vec<_> = vertices
            .iter()
            .enumerate()
            .filter(|(i, _)| keep(i))
            .map(|(_, v)| *v)
            .collect();
        let uv: Vec<_> = uvs
            .iter()
            .enumerate()
            .filter(|(i, _)| keep(i))
            .map(|(_, v)| *v)
            .collect();
        assert_eq!(nif.view_of(s).unwrap().vertices().unwrap(), v);
        assert_eq!(nif.view_of(s).unwrap().uvs().unwrap(), uv);
        assert_eq!(
            nif.view_of(s).unwrap().triangles().unwrap(),
            remapped(&tris, &map)
        );
    }
}

#[test]
fn skin_weights_and_partitions_follow() {
    let mut nif = load("assets/Bones.nif");
    let limb = first(&nif);
    let s = nif.block_of::<nifly::geometry::NiShape>(limb).unwrap();
    let count = nif.view_of(s).unwrap().vertices().unwrap().len();
    let weights: Vec<_> = (0..3)
        .map(|b| nif.view_of(s).unwrap().bone_weights(b))
        .collect();
    let deleted: Vec<usize> = (0..count).step_by(7).collect();
    let map = remap(count, &deleted);
    assert!(
        nif.edit::<nifly::geometry::NiShape>(limb)
            .is_some_and(|mut e| e.delete_vertices(&deleted))
    );

    let saved = reload(&mut nif);
    for nif in [&nif, &saved] {
        let s = nif
            .block_of::<nifly::geometry::NiShape>(first(nif))
            .unwrap();
        let left = count - deleted.len();
        assert_eq!(nif.view_of(s).unwrap().vertices().unwrap().len(), left);
        // Each bone keeps its weights, re-indexed, without the deleted vertices.
        for (bone, before) in weights.iter().enumerate() {
            let expected: BTreeMap<u16, f32> = before
                .iter()
                .filter_map(|w| Some((map[w.vertex as usize]?, w.weight)))
                .collect();
            let got: BTreeMap<u16, f32> = nif
                .view_of(s)
                .unwrap()
                .bone_weights(bone)
                .iter()
                .map(|w| (w.vertex, w.weight))
                .collect();
            assert_eq!(got, expected, "bone {bone}");
        }
        // The partition only maps existing vertices.
        let inst = nif.child_blocks_of::<NiSkinInstance>(s).next().unwrap();
        let part = nif.child_blocks_of::<NiSkinPartition>(inst).next().unwrap();
        assert!(
            part.vertex_map(0)
                .unwrap()
                .iter()
                .all(|&v| usize::from(v) < left)
        );
        assert!(
            nif.view_of(s)
                .unwrap()
                .triangles()
                .unwrap()
                .iter()
                .flatten()
                .all(|&v| usize::from(v) < left)
        );
    }
}

#[test]
fn strips_follow() {
    // Each shape is the strip 1, 2, 0, 3: deleting vertex 3 leaves one triangle.
    let mut nif = load("assets/Strips_MW.nif");
    let shape = first(&nif);
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.delete_vertices(&[3]))
    );
    let s = nif.block_of::<nifly::geometry::NiShape>(shape).unwrap();
    assert_eq!(nif.view_of(s).unwrap().vertices().unwrap().len(), 3);
    assert_eq!(nif.view_of(s).unwrap().triangles().unwrap(), [[1, 2, 0]]);
}

#[test]
fn locked_normals_follow() {
    let mut nif = load("assets/Cube.nif");
    let cube = first(&nif);
    nif.edit::<nifly::objects::NiAvObject>(cube)
        .and_then(|mut e| e.assign_extra_data(ExtraIntegers("LOCKEDNORM", [0, 5, 10])))
        .unwrap();
    assert!(
        nif.edit::<nifly::geometry::NiShape>(cube)
            .is_some_and(|mut e| e.delete_vertices(&[5]))
    );
    let locked = nif.blocks_of::<NiIntegersExtraData>().next().unwrap();
    assert_eq!(locked.values(), [0, 9]);
}

#[test]
fn deleting_everything_leaves_an_empty_shape() {
    let mut nif = load("assets/Cube.nif");
    let cube = first(&nif);
    let all: Vec<usize> = (0..14).collect();
    assert!(
        nif.edit::<nifly::geometry::NiShape>(cube)
            .is_some_and(|mut e| e.delete_vertices(&all))
    );
    let s = nif.block_of::<nifly::geometry::NiShape>(cube).unwrap();
    assert!(nif.view_of(s).unwrap().vertices().unwrap().is_empty());
    assert!(nif.view_of(s).unwrap().triangles().unwrap().is_empty());
    assert!(
        nif.edit::<nifly::geometry::NiShape>(cube)
            .is_some_and(|e| e.delete())
    );
}

#[test]
fn false_changes_nothing() {
    let mut nif = load("assets/Cube.nif");
    let cube = first(&nif);
    let before = nif
        .view::<nifly::geometry::NiShape>(cube)
        .unwrap()
        .vertices()
        .unwrap();
    for bad in [&[14][..], &[0, 14], &[usize::MAX]] {
        assert!(
            !nif.edit::<nifly::geometry::NiShape>(cube)
                .is_some_and(|mut e| e.delete_vertices(bad)),
            "{bad:?}"
        );
    }
    assert_eq!(
        nif.view::<nifly::geometry::NiShape>(cube)
            .unwrap()
            .vertices()
            .unwrap(),
        before
    );
    // An empty list deletes nothing.
    assert!(
        nif.edit::<nifly::geometry::NiShape>(cube)
            .is_some_and(|mut e| e.delete_vertices(&[]))
    );
    assert_eq!(
        nif.view::<nifly::geometry::NiShape>(cube)
            .unwrap()
            .vertices()
            .unwrap(),
        before
    );

    let mut nif = load("nifly-sys/vendor/nifly/tests/input/TestNifFile_Particles_MW.nif");
    let particles = nif
        .shapes()
        .indexed()
        .find(|(_, s)| s.name() == "Blizzard")
        .unwrap()
        .0;
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(particles)
            .is_some_and(|mut e| e.delete_vertices(&[0]))
    );
    for i in [0, nif.blocks().count(), usize::MAX] {
        assert!(
            !nif.edit::<nifly::geometry::NiShape>(i)
                .is_some_and(|mut e| e.delete_vertices(&[0])),
            "{i}"
        );
    }
}
