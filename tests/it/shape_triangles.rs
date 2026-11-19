//! Tests for NifFile::shape_triangles and NifFile::set_shape_triangles.

use crate::common::{load, upstream, reload};

use nifly::{
    file::{Index, NifFile},
    geometry::NiShape,
};

fn shape<'a>(nif: &'a NifFile, name: &str) -> &'a NiShape {
    nif.shapes().find(|s| s.name() == name).unwrap()
}

fn index(nif: &NifFile, name: &str) -> Index {
    nif.shapes().indexed().find(|(_, s)| s.name() == name).unwrap().0
}

fn triangles(nif: &NifFile, name: &str) -> Option<Vec<[u16; 3]>> {
    nif.view_of(shape(nif, name)).unwrap().triangles()
}

#[test]
fn reads_triangles() {
    // NiTriShapeData, matching its triangle count and indexing its vertices.
    let nif = load("Bones");
    let tris = triangles(&nif, "Limb").unwrap();
    assert_eq!(tris.len(), nif.view_of(shape(&nif, "Limb")).unwrap().geometry_data().unwrap().triangle_count());
    assert_eq!(tris.len(), 252);
    assert!(tris.iter().flatten().all(|&i| i < 164));

    // BSTriShape: a cube is 12 triangles over 14 vertices.
    let nif = load("Cube");
    let tris = triangles(&nif, "Cube").unwrap();
    assert_eq!(tris.len(), 12);
    assert!(tris.iter().flatten().all(|&i| i < 14));
    assert!(tris.iter().all(|[a, b, c]| a != b && b != c && c != a));

    // Starfield geometry without its mesh file loaded has an empty list.
    let nif = upstream("TestNifFile_SF");
    assert_eq!(nif.shapes().views().next().unwrap().triangles(), Some(vec![]));

    // Particles have none.
    let nif = upstream("TestNifFile_Particles_MW");
    assert_eq!(triangles(&nif, "Blizzard"), None);
}

#[test]
fn set_round_trip() {
    // BSTriShape: flip every triangle's winding.
    let mut nif = load("Cube");
    let cube = index(&nif, "Cube");
    let flipped: Vec<[u16; 3]> = triangles(&nif, "Cube").unwrap().iter().map(|&[a, b, c]| [a, c, b]).collect();
    assert!(nif.edit::<nifly::geometry::NiShape>(cube).is_some_and(|mut e| e.set_triangles(&flipped)));
    assert_eq!(triangles(&nif, "Cube").unwrap(), flipped);
    assert_eq!(triangles(&reload(&mut nif), "Cube").unwrap(), flipped);

    // NiTriShapeData: keep half, which updates the stored count.
    let mut nif = load("Bones");
    let limb = index(&nif, "Limb");
    let half = triangles(&nif, "Limb").unwrap()[..126].to_vec();
    assert!(nif.edit::<nifly::geometry::NiShape>(limb).is_some_and(|mut e| e.set_triangles(&half)));
    assert_eq!(nif.view_of(shape(&nif, "Limb")).unwrap().geometry_data().unwrap().triangle_count(), 126);
    let saved = reload(&mut nif);
    assert_eq!(triangles(&saved, "Limb").unwrap(), half);
    assert_eq!(saved.view_of(shape(&saved, "Limb")).unwrap().geometry_data().unwrap().triangle_count(), 126);

    // An empty list is allowed.
    assert!(nif.edit::<nifly::geometry::NiShape>(limb).is_some_and(|mut e| e.set_triangles(&[])));
    assert_eq!(triangles(&nif, "Limb"), Some(vec![]));
}

#[test]
fn false_changes_nothing() {
    let mut nif = load("Bones");
    let limb = index(&nif, "Limb");
    let before = triangles(&nif, "Limb");
    // Index 164 is one past the last vertex.
    assert!(!nif.edit::<nifly::geometry::NiShape>(limb).is_some_and(|mut e| e.set_triangles(&[[0, 1, 2], [0, 1, 164]])));
    assert!(!nif.edit::<nifly::geometry::NiShape>(limb).is_some_and(|mut e| e.set_triangles(&[[u16::MAX, 0, 1]])));
    // NiTriShapeData counts triangles in 16 bits.
    assert!(!nif.edit::<nifly::geometry::NiShape>(limb).is_some_and(|mut e| e.set_triangles(&vec![[0, 1, 2]; 65536])));
    assert_eq!(triangles(&nif, "Limb"), before);

    // Not a shape, or out of range.
    let count = nif.blocks().count();
    for i in [0, 5, count, usize::MAX] {
        assert!(!nif.edit::<nifly::geometry::NiShape>(i).is_some_and(|mut e| e.set_triangles(&[])), "{i}");
    }

    // Skyrim SE BSTriShapes save at most 65535 triangles.
    let mut nif = load("Cube");
    let cube = index(&nif, "Cube");
    assert_eq!(nif.triangle_limit(), 65535);
    assert!(!nif.edit::<nifly::geometry::NiShape>(cube).is_some_and(|mut e| e.set_triangles(&vec![[0, 1, 2]; 65536])));
    assert_eq!(triangles(&nif, "Cube").unwrap().len(), 12);
    assert!(nif.edit::<nifly::geometry::NiShape>(cube).is_some_and(|mut e| e.set_triangles(&vec![[0, 1, 2]; 65535])));

    // Particles have no triangle list.
    let mut nif = upstream("TestNifFile_Particles_MW");
    let particles = index(&nif, "Blizzard");
    assert!(!nif.edit::<nifly::geometry::NiShape>(particles).is_some_and(|mut e| e.set_triangles(&[])));
}

#[test]
fn strips() {
    // Each shape is one 4-point strip.
    let mut nif = load("Strips_MW");
    let (index, s) = nif.shapes().indexed().next().unwrap();
    assert_eq!(s.block_name(), "NiTriStrips");
    assert_eq!(nif.view_of(s).unwrap().triangles().unwrap(), [[1, 2, 0], [2, 3, 0]]);
    // Strips can't be set or reordered as triangles.
    assert!(!nif.edit::<nifly::geometry::NiShape>(index).is_some_and(|mut e| e.set_triangles(&[[0, 1, 2]])));
    assert!(!nif.edit::<nifly::geometry::NiShape>(index).is_some_and(|mut e| e.reorder_triangles(&[1, 0])));
    assert_eq!(nif.shapes().views().next().unwrap().triangles().unwrap(), [[1, 2, 0], [2, 3, 0]]);
}
