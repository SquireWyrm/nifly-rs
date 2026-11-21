//! Tests for NifFile::triangulate_shape, on assets/Strips_MW.nif.

use crate::common::{open, reload, shape, shape_index};
use std::ptr;

use nifly::file::NifFile;

fn load() -> NifFile {
    open("assets/Strips_MW.nif")
}

const MARKER: &str = "Tri EditorMarker_box_02";

#[test]
fn the_asset_has_strips() {
    let nif = load();
    for name in [MARKER, "", "Shared"] {
        let s = shape(&nif, name);
        assert_eq!(s.block_name(), "NiTriStrips", "{name:?}");
        assert_eq!(
            nif.view_of(s)
                .unwrap()
                .geometry_data()
                .unwrap()
                .block_name(),
            "NiTriStripsData",
            "{name:?}"
        );
        assert_eq!(
            nif.view_of(s).unwrap().triangles().unwrap(),
            [[1, 2, 0], [2, 3, 0]],
            "{name:?}"
        );
    }
    // "" and "Shared" share their data.
    assert!(ptr::eq(
        nif.view_of(shape(&nif, ""))
            .unwrap()
            .geometry_data()
            .unwrap(),
        nif.view_of(shape(&nif, "Shared"))
            .unwrap()
            .geometry_data()
            .unwrap()
    ));
    assert!(
        !nif.view_of(shape(&nif, MARKER))
            .unwrap()
            .is_sse_compatible()
    );
}

#[test]
fn converts_to_triangles() {
    let mut nif = load();
    let marker = shape_index(&nif, MARKER);
    let data = nif
        .block_index(
            nif.view_of(shape(&nif, MARKER))
                .unwrap()
                .geometry_data()
                .unwrap(),
        )
        .unwrap();
    let s = shape(&nif, MARKER);
    let (vertices, normals, transform) = (
        nif.view_of(s).unwrap().vertices(),
        nif.view_of(s).unwrap().normals(),
        s.transform(),
    );
    let count = nif.blocks().count();

    assert!(
        nif.edit::<nifly::geometry::NiShape>(marker)
            .is_some_and(|mut e| e.triangulate())
    );
    // Same block indices, new types.
    assert_eq!(nif.blocks().count(), count);
    assert_eq!(nif.block(marker).unwrap().block_name(), "NiTriShape");
    assert_eq!(nif.block(data).unwrap().block_name(), "NiTriShapeData");

    let saved = reload(&mut nif);
    for nif in [&nif, &saved] {
        let s = shape(nif, MARKER);
        assert_eq!(s.block_name(), "NiTriShape");
        assert_eq!(
            nif.view_of(s).unwrap().triangles().unwrap(),
            [[1, 2, 0], [2, 3, 0]]
        );
        assert_eq!(
            nif.view_of(s)
                .unwrap()
                .geometry_data()
                .unwrap()
                .triangle_count(),
            2
        );
        assert_eq!(
            (
                nif.view_of(s).unwrap().vertices(),
                nif.view_of(s).unwrap().normals(),
                s.transform()
            ),
            (vertices.clone(), normals.clone(), transform)
        );
        assert!(nif.view_of(s).unwrap().is_sse_compatible());
        // Triangles can now be set.
        assert_eq!(shape(nif, "").block_name(), "NiTriStrips");
    }
    assert!(
        nif.edit::<nifly::geometry::NiShape>(marker)
            .is_some_and(|mut e| e.set_triangles(&[[0, 1, 2]]))
    );
}

#[test]
fn false_changes_nothing() {
    let mut nif = load();
    let count = nif.blocks().count();
    // Shared data.
    for name in ["", "Shared"] {
        assert!(
            !nif.edit::<nifly::geometry::NiShape>(shape_index(&nif, name))
                .is_some_and(|mut e| e.triangulate()),
            "{name:?}"
        );
        assert_eq!(shape(&nif, name).block_name(), "NiTriStrips");
    }
    // Already triangles.
    let marker = shape_index(&nif, MARKER);
    assert!(
        nif.edit::<nifly::geometry::NiShape>(marker)
            .is_some_and(|mut e| e.triangulate())
    );
    assert!(
        !nif.edit::<nifly::geometry::NiShape>(marker)
            .is_some_and(|mut e| e.triangulate())
    );
    // Not a shape, or out of range.
    for i in [0, nif.blocks().count(), usize::MAX] {
        assert!(
            !nif.edit::<nifly::geometry::NiShape>(i)
                .is_some_and(|mut e| e.triangulate()),
            "{i}"
        );
    }
    assert_eq!(nif.blocks().count(), count);
}
