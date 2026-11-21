//! Tests for NifFile::invert_shape_uvs.

use crate::common::{open, reload};

use nifly::file::{Index, NifFile};

fn first(nif: &NifFile) -> (Index, Vec<[f32; 2]>) {
    let (i, s) = nif.shapes().indexed().next().unwrap();
    (i, nif.view_of(s).unwrap().uvs().unwrap_or_default())
}

#[test]
fn flips_each_axis() {
    // NiTriShapeData (Bones) and BSTriShape (Cube).
    for path in ["assets/Bones.nif", "assets/Cube.nif"] {
        let mut nif = open(path);
        let (shape, before) = first(&nif);
        assert!(!before.is_empty());

        assert!(
            nif.edit::<nifly::geometry::NiShape>(shape)
                .is_some_and(|mut e| e.invert_uvs(true, false))
        );
        let flipped: Vec<_> = before.iter().map(|[u, v]| [1.0 - u, *v]).collect();
        assert_eq!(first(&nif).1, flipped, "{path}");

        assert!(
            nif.edit::<nifly::geometry::NiShape>(shape)
                .is_some_and(|mut e| e.invert_uvs(false, true))
        );
        let both: Vec<_> = before.iter().map(|[u, v]| [1.0 - u, 1.0 - v]).collect();
        assert_eq!(first(&nif).1, both, "{path}");
        assert_eq!(first(&reload(&mut nif)).1, both, "{path}");

        // Flipping again restores them; neither axis changes nothing.
        assert!(
            nif.edit::<nifly::geometry::NiShape>(shape)
                .is_some_and(|mut e| e.invert_uvs(true, true))
        );
        assert!(
            nif.edit::<nifly::geometry::NiShape>(shape)
                .is_some_and(|mut e| e.invert_uvs(false, false))
        );
        let restored = first(&nif).1;
        assert!(
            restored
                .iter()
                .zip(&before)
                .all(|(a, b)| (a[0] - b[0]).abs() < 1e-6 && (a[1] - b[1]).abs() < 1e-6),
            "{path}"
        );
    }
}

#[test]
fn false_without_uvs() {
    // Morrowind's editor marker has no UVs.
    let mut nif = open("nifly-sys/vendor/nifly/tests/input/TestNifFile_Static_MW.nif");
    let marker = nif
        .shapes()
        .indexed()
        .find(|(_, s)| s.name() == "Tri EditorMarker_box_02")
        .unwrap()
        .0;
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(marker)
            .is_some_and(|mut e| e.invert_uvs(true, true))
    );
    for i in [0, nif.blocks().count(), usize::MAX] {
        assert!(
            !nif.edit::<nifly::geometry::NiShape>(i)
                .is_some_and(|mut e| e.invert_uvs(true, true)),
            "{i}"
        );
    }
}
