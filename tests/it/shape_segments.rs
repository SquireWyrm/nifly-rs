//! Tests for NifFile::shape_segments and NifFile::set_shape_segments.

use crate::common::{UPSTREAM, load, reload};
use std::{collections::BTreeMap, fs};

use nifly::{
    file::{Index, NifFile},
    geometry::{NiShape, Segment, SegmentIndex, ShapeSegments, Subsegment},
};

fn fo4() -> NifFile {
    NifFile::load(fs::read(format!("{UPSTREAM}/TestNifFile_Skinned_FO4.nif")).unwrap().as_slice(), None).unwrap()
}

fn shape<'a>(nif: &'a NifFile, name: &str) -> &'a NiShape {
    nif.shapes().find(|s| s.name() == name).unwrap()
}

fn index(nif: &NifFile, name: &str) -> Index {
    nif.shapes().indexed().find(|(_, s)| s.name() == name).unwrap().0
}

fn segments(nif: &NifFile, name: &str) -> ShapeSegments {
    nif.view_of(shape(nif, name)).unwrap().segments().unwrap()
}

fn at(segment: usize, subsegment: Option<usize>) -> Option<SegmentIndex> {
    Some(SegmentIndex { segment, subsegment })
}

/// Sorted triangles by (segment, subsegment), `None` for triangles in no segment.
type Groups = BTreeMap<Option<(usize, Option<usize>)>, Vec<[u16; 3]>>;

/// The sorted triangles in each segment or subsegment.
fn groups(nif: &NifFile, name: &str) -> Groups {
    let tris = nif.view_of(shape(nif, name)).unwrap().triangles().unwrap();
    let mut out: BTreeMap<_, Vec<_>> = BTreeMap::new();
    for (t, seg) in tris.iter().zip(segments(nif, name).triangles) {
        out.entry(seg.map(|s| (s.segment, s.subsegment))).or_default().push(*t);
    }
    out.values_mut().for_each(|v| v.sort());
    out
}

#[test]
fn reads_segments() {
    let nif = fo4();
    let s = segments(&nif, "cylinder_1");
    assert_eq!(s.ssf_file, "");
    assert_eq!(s.segments, vec![Segment::default(); 4]);
    let counts: Vec<usize> = (0..4).map(|i| s.triangles.iter().filter(|t| **t == at(i, None)).count()).collect();
    assert_eq!(counts, [20, 14, 16, 18]);
    assert_eq!(s.triangles.len(), nif.view_of(shape(&nif, "cylinder_1")).unwrap().triangles().unwrap().len());

    // Only BSSubIndexTriShapes have segments.
    let nif = load("Cube");
    assert!(nif.view_of(shape(&nif, "Cube")).unwrap().segments().is_none());
}

#[test]
fn sets_subsegments() {
    let mut nif = fo4();
    let cylinder = index(&nif, "cylinder_1");
    let before = nif.view_of(shape(&nif, "cylinder_1")).unwrap().triangles().unwrap();
    let n = before.len();
    let segs = vec![
        Segment {
            subsegments: vec![
                Subsegment { user_slot: Some(32), material: 0xABCD_1234, extra_data: vec![1.0, 2.5] },
                Subsegment { user_slot: None, material: 7, extra_data: vec![] },
            ],
        },
        Segment::default(),
    ];
    // Interleaved, so setting has to reorder them.
    let triangles: Vec<_> = (0..n)
        .map(|t| match t % 4 {
            0 => at(0, Some(0)),
            1 => at(0, Some(1)),
            2 => at(0, None),
            _ => at(1, None),
        })
        .collect();
    let input = ShapeSegments { ssf_file: r"Meshes\Test.ssf".into(), segments: segs.clone(), triangles };
    let mut expected = Groups::new();
    for (t, seg) in before.iter().zip(&input.triangles) {
        expected.entry(seg.map(|s| (s.segment, s.subsegment))).or_default().push(*t);
    }
    expected.values_mut().for_each(|v| v.sort());

    assert!(nif.edit::<nifly::geometry::NiShape>(cylinder).is_some_and(|mut e| e.set_segments(&input)));
    let saved = reload(&mut nif);
    for nif in [&nif, &saved] {
        let s = segments(nif, "cylinder_1");
        assert_eq!(s.ssf_file, r"Meshes\Test.ssf");
        assert_eq!(s.segments, segs);
        // Each group keeps its triangles, now together: segment 0, its subsegments, segment 1.
        assert_eq!(groups(nif, "cylinder_1"), expected);
        let order: Vec<_> = s.triangles.iter().map(|t| t.map(|t| (t.segment, t.subsegment))).collect();
        let mut sorted = order.clone();
        sorted.sort_by_key(|t| match t {
            Some((0, None)) => 0,
            Some((0, Some(0))) => 1,
            Some((0, Some(1))) => 2,
            _ => 3,
        });
        assert_eq!(order, sorted);
        // The other shape is untouched.
        assert_eq!(segments(nif, "cylinder_2").segments.len(), 4);
    }
}

#[test]
fn unassigned_go_to_the_first_segment() {
    let mut nif = fo4();
    let cylinder = index(&nif, "cylinder_1");
    let n = segments(&nif, "cylinder_1").triangles.len();
    let input = ShapeSegments { segments: vec![Segment::default(); 2], triangles: vec![None; n], ..Default::default() };
    assert!(nif.edit::<nifly::geometry::NiShape>(cylinder).is_some_and(|mut e| e.set_segments(&input)));
    assert_eq!(segments(&nif, "cylinder_1").triangles, vec![at(0, None); n]);
}

#[test]
fn false_changes_nothing() {
    let mut nif = fo4();
    let cylinder = index(&nif, "cylinder_1");
    let before = segments(&nif, "cylinder_1");
    let n = before.triangles.len();
    let one_sub = Segment { subsegments: vec![Subsegment::default()] };
    let bad = [
        ShapeSegments { segments: vec![Segment::default()], triangles: vec![at(0, None); n - 1], ..Default::default() },
        ShapeSegments { segments: vec![Segment::default()], triangles: vec![at(1, None); n], ..Default::default() },
        ShapeSegments { segments: vec![one_sub.clone()], triangles: vec![at(0, Some(1)); n], ..Default::default() },
        ShapeSegments {
            segments: vec![Segment { subsegments: vec![Subsegment { user_slot: Some(29), ..Default::default() }] }],
            triangles: vec![at(0, None); n],
            ..Default::default()
        },
    ];
    for (i, input) in bad.iter().enumerate() {
        assert!(!nif.edit::<nifly::geometry::NiShape>(cylinder).is_some_and(|mut e| e.set_segments(input)), "{i}");
    }
    assert_eq!(segments(&nif, "cylinder_1"), before);

    // Not a BSSubIndexTriShape, not a shape, out of range.
    let empty = ShapeSegments::default();
    let mut cube = load("Cube");
    let cube_shape = index(&cube, "Cube");
    assert!(!cube.edit::<nifly::geometry::NiShape>(cube_shape).is_some_and(|mut e| e.set_segments(&empty)));
    for i in [0, nif.blocks().count(), usize::MAX] {
        assert!(!nif.edit::<nifly::geometry::NiShape>(i).is_some_and(|mut e| e.set_segments(&empty)), "{i}");
    }
}
