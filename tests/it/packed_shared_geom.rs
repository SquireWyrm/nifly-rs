//! Tests for BSPackedCombinedSharedGeomDataExtra, Fallout 4 precombined mesh objects.

use std::io::Cursor;

use nifly::{
    extra_data::{
        BsPackedCombinedSharedGeomDataExtra, ExtraPackedSharedGeom, PackedGeomInstance,
        PackedGeomObject, PackedGeomTotals, TriangleRange,
    },
    file::{Index, NifFile, NifVersion, SaveOptions},
    geometry::NiShape,
    nodes::NiNode,
    object3d::{BoundingSphere, Matrix3, Transform, Vec3},
    objects::NiAvObject,
};

use crate::common::open;

fn save(nif: &mut NifFile) -> Vec<u8> {
    let mut cur = Cursor::new(Vec::new());
    nif.save(
        &mut cur,
        Some(SaveOptions {
            optimize: false,
            sort_blocks: false,
        }),
    )
    .unwrap();
    cur.into_inner()
}

fn reload(nif: &mut NifFile) -> NifFile {
    NifFile::load(save(nif).as_slice(), None).unwrap()
}

fn object() -> PackedGeomObject {
    let instance = |i: f32| PackedGeomInstance {
        grayscale_to_palette_scale: 0.5 + i,
        transform: Transform::new(
            Vec3::new(10.0 * i, 20.0, -30.0),
            Matrix3::from_rows(
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(-1.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
            ),
            1.5,
        ),
        bounds: BoundingSphere::new(Vec3::new(1.0, 2.0, i), 64.0),
    };
    PackedGeomObject {
        filename_hash: 0xDEAD_BEEF,
        data_offset: 4096,
        vertex_count: 100,
        lod_levels: 3,
        lods: [
            TriangleRange {
                count: 50,
                offset: 0,
            },
            TriangleRange {
                count: 20,
                offset: 50,
            },
            TriangleRange {
                count: 5,
                offset: 70,
            },
        ],
        vertex_desc: 0x0043_B000_0005_4408,
        instances: vec![instance(0.0), instance(1.0)],
    }
}

/// A Fallout 4 file with a packed geometry block holding `objects` on its root.
fn with_objects(objects: &[PackedGeomObject]) -> (NifFile, Index) {
    let mut nif = NifFile::create(NifVersion::FALLOUT_4);
    let root = nif.root_index().unwrap();
    let block = nif
        .edit::<NiAvObject>(root)
        .unwrap()
        .assign_extra_data(ExtraPackedSharedGeom(
            "",
            PackedGeomTotals::default(),
            objects,
        ))
        .unwrap();
    (nif, block)
}

#[test]
fn objects_survive_save() {
    let (mut nif, block) = with_objects(&[object(), PackedGeomObject::default()]);
    let totals = PackedGeomTotals {
        vertex_desc: 0x0001_2345_6789_ABCD,
        vertex_count: 100,
        triangle_count: 75,
        raw_flags_1: 1,
        raw_flags_2: 2,
    };
    nif.block_of_mut::<BsPackedCombinedSharedGeomDataExtra>(block)
        .unwrap()
        .set_totals(totals);

    let saved = reload(&mut nif);
    let b = saved
        .block_of::<BsPackedCombinedSharedGeomDataExtra>(block)
        .unwrap();
    assert_eq!(b.objects(), [object(), PackedGeomObject::default()]);
    assert_eq!(b.totals(), totals);
    // The file still ends with the right blocks after it.
    assert!(saved.root_node().is_some());
}

#[test]
fn objects_store_no_geometry() {
    // An object takes its hash and offset (8 bytes), its counts and LOD ranges (36), its
    // instances (72 each) and its vertex format (8). Its vertices and triangles are in the CSG.
    let empty = save(&mut with_objects(&[]).0).len();
    let one = save(&mut with_objects(&[object()]).0).len();
    assert_eq!(one - empty, 8 + 36 + 2 * 72 + 8);
}

#[test]
fn set_objects() {
    let (mut nif, block) = with_objects(&[]);
    let b = nif
        .block_of_mut::<BsPackedCombinedSharedGeomDataExtra>(block)
        .unwrap();
    assert!(b.objects().is_empty());
    b.set_objects(&[object(), object()]);
    assert_eq!(b.objects().len(), 2);
    b.set_objects(&[PackedGeomObject {
        instances: vec![],
        ..object()
    }]);
    assert_eq!(
        reload(&mut nif)
            .block_of::<BsPackedCombinedSharedGeomDataExtra>(block)
            .unwrap()
            .objects(),
        [PackedGeomObject {
            instances: vec![],
            ..object()
        }]
    );
}

#[test]
fn copies_keep_the_layout() {
    let (mut nif, block) = with_objects(&[object()]);
    let root = nif.root_index().unwrap();
    let copy = nif.edit::<NiNode>(root).unwrap().clone_node();
    let count = nif.blocks().count();
    let saved = reload(&mut nif);
    assert_eq!(saved.blocks().count(), count);
    let objects: Vec<_> = saved
        .blocks_of::<BsPackedCombinedSharedGeomDataExtra>()
        .map(|b| b.objects())
        .collect();
    assert_eq!(objects, [vec![object()], vec![object()]], "{block} {copy}");
}

#[test]
fn needs_an_object_target() {
    let (mut nif, block) = with_objects(&[]);
    assert!(nif.edit::<NiAvObject>(block).is_none());
    // A shape works too.
    let mut nif = open("nifly-sys/vendor/nifly/tests/input/TestNifFile_Static_FO4.nif");
    let shape = nif.shapes().indices().next().unwrap();
    let block = nif
        .edit::<NiShape>(shape)
        .unwrap()
        .into_upcast::<NiAvObject, _>()
        .assign_extra_data(ExtraPackedSharedGeom(
            "P",
            PackedGeomTotals::default(),
            [object()],
        ));
    assert!(block.is_some());
}

#[test]
fn totals_at_creation() {
    let mut nif = NifFile::create(NifVersion::FALLOUT_4);
    let root = nif.root_index().unwrap();
    let totals = PackedGeomTotals {
        vertex_desc: 7,
        vertex_count: 3,
        triangle_count: 1,
        raw_flags_1: 4,
        raw_flags_2: 5,
    };
    let block = nif
        .edit::<NiAvObject>(root)
        .unwrap()
        .assign_extra_data(ExtraPackedSharedGeom("", totals, [object()]))
        .unwrap();
    let saved = reload(&mut nif);
    let b = saved
        .block_of::<BsPackedCombinedSharedGeomDataExtra>(block)
        .unwrap();
    assert_eq!((b.totals(), b.objects()), (totals, vec![object()]));
}
