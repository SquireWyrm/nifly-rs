//! Tests for NifFile::reorder_shape_triangles.

use crate::common::{load, reload, upstream};

use nifly::file::{Index, NifFile};

fn first(nif: &NifFile) -> (Index, Vec<[u16; 3]>) {
    let (i, s) = nif.shapes().indexed().next().unwrap();
    (i, nif.view_of(s).unwrap().triangles().unwrap())
}

#[test]
fn reorders() {
    // BSTriShape and NiTriShape: reverse, then rotate by one.
    for mut nif in [load("Cube"), load("Bones")] {
        let (shape, before) = first(&nif);
        let n = before.len();
        let reversed: Vec<usize> = (0..n).rev().collect();
        assert!(
            nif.edit::<nifly::geometry::NiShape>(shape)
                .is_some_and(|mut e| e.reorder_triangles(&reversed))
        );
        let expected: Vec<_> = before.iter().rev().copied().collect();
        assert_eq!(first(&nif).1, expected);
        assert_eq!(first(&reload(&mut nif)).1, expected);

        let rotate: Vec<usize> = (1..n).chain([0]).collect();
        assert!(
            nif.edit::<nifly::geometry::NiShape>(shape)
                .is_some_and(|mut e| e.reorder_triangles(&rotate))
        );
        assert_eq!(first(&nif).1[n - 1], expected[0]);
    }
}

#[test]
fn skinned_skyrim_se_needs_partition_update() {
    let mut nif = upstream("TestNifFile_Skinned_SE");
    let (shape, before) = first(&nif);
    let reversed: Vec<usize> = (0..before.len()).rev().collect();
    let expected: Vec<_> = before.iter().rev().copied().collect();

    // The partitions keep the old order, and they're what's saved.
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.reorder_triangles(&reversed))
    );
    assert_eq!(first(&nif).1, expected);
    assert_eq!(first(&reload(&mut nif)).1, before);

    // Rebuilding them keeps the new order.
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.update_skin_partitions())
    );
    assert_eq!(first(&reload(&mut nif)).1, expected);
}

#[test]
fn false_changes_nothing() {
    let mut nif = load("Cube");
    let (shape, before) = first(&nif);
    let n = before.len();
    let identity: Vec<usize> = (0..n).collect();
    let mut repeated = identity.clone();
    repeated[1] = 0;
    for order in [
        &identity[..n - 1],
        &[identity.as_slice(), &[0]].concat(),
        &repeated,
        &[n; 12][..],
        &[usize::MAX; 12][..],
    ] {
        assert!(
            !nif.edit::<nifly::geometry::NiShape>(shape)
                .is_some_and(|mut e| e.reorder_triangles(order)),
            "{order:?}"
        );
    }
    assert_eq!(first(&nif).1, before);
    // The identity order is allowed and changes nothing.
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .is_some_and(|mut e| e.reorder_triangles(&identity))
    );
    assert_eq!(first(&nif).1, before);

    // Particles have no triangles; not a shape; out of range.
    let mut nif = upstream("TestNifFile_Particles_MW");
    let particles = nif
        .shapes()
        .indexed()
        .find(|(_, s)| s.name() == "Blizzard")
        .unwrap()
        .0;
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(particles)
            .is_some_and(|mut e| e.reorder_triangles(&[]))
    );
    for i in [0, nif.blocks().count(), usize::MAX] {
        assert!(
            !nif.edit::<nifly::geometry::NiShape>(i)
                .is_some_and(|mut e| e.reorder_triangles(&[])),
            "{i}"
        );
    }
}
