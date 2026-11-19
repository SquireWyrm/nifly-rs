//! Tests for NifFile::apply_locked_normals, ExtraIntegers and NiIntegersExtraData, and
//! calc_shape_normals keeping locked normals.

use crate::common::{load, reload};

use nifly::{
    extra_data::{ExtraIntegers, NiIntegersExtraData},
    file::{Index, NifFile},
    geometry::NiShape,
    object3d::Vec3,
};

fn first_shape(nif: &NifFile) -> (Index, &NiShape) {
    nif.shapes().indexed().next().unwrap()
}

fn normals(nif: &NifFile) -> Vec<Vec3> {
    nif.view_of(first_shape(nif).1).unwrap().normals().unwrap()
}

/// The values of each `LOCKEDNORM` block on the first shape.
fn locked(nif: &NifFile) -> Vec<Vec<u32>> {
    let shape = first_shape(nif).1;
    nif.view_of(shape).unwrap().extra_data()
        .of::<NiIntegersExtraData>()
        .filter(|d| d.name() == "LOCKEDNORM")
        .map(|d| d.values().to_vec())
        .collect()
}

/// `name`.nif with every normal set to `normal` and `lock` locked.
fn source(name: &str, normal: Vec3, lock: &[u32]) -> NifFile {
    let mut nif = load(name);
    let (shape, s) = first_shape(&nif);
    let n = nif.view_of(s).unwrap().normals().unwrap().len();
    assert!(nif.edit::<nifly::geometry::NiShape>(shape).is_some_and(|mut e| e.set_normals(&vec![normal; n])));
    nif.edit::<nifly::objects::NiAvObject>(shape).and_then(|mut e| e.assign_extra_data(ExtraIntegers("LOCKEDNORM", lock))).unwrap();
    nif
}

#[test]
fn integers_extra_data() {
    let mut nif = load("Cube");
    let shape = first_shape(&nif).0;
    let index = nif.edit::<nifly::objects::NiAvObject>(shape).and_then(|mut e| e.assign_extra_data(ExtraIntegers("List", [7, 0, u32::MAX]))).unwrap();
    let block = nif.block_of::<NiIntegersExtraData>(index).unwrap();
    assert_eq!((block.name(), block.values()), ("List".into(), &[7, 0, u32::MAX][..]));
    let empty = nif.edit::<nifly::objects::NiAvObject>(shape).and_then(|mut e| e.assign_extra_data(ExtraIntegers("Empty", Vec::new()))).unwrap();
    assert!(nif.block_of::<NiIntegersExtraData>(empty).unwrap().values().is_empty());

    let saved = reload(&mut nif);
    let blocks: Vec<_> = saved.blocks_of::<NiIntegersExtraData>().map(|d| d.values().to_vec()).collect();
    assert_eq!(blocks, [vec![7, 0, u32::MAX], vec![]]);
}

#[test]
fn copies_only_locked_normals() {
    for (name, tolerance) in [("Bones", 0.0), ("Cube", 0.01)] {
        // Index 9999 is past the last vertex and ignored.
        let lock = [0, 3, 9999];
        let src = source(name, Vec3::X, &lock);
        let mut nif = load(name);
        let (shape, src_shape) = (first_shape(&nif).0, first_shape(&src).1);
        let before = normals(&nif);

        assert!(nif.edit::<nifly::geometry::NiShape>(shape).is_some_and(|mut e| e.apply_locked_normals(src.view_of(src_shape).unwrap())), "{name}");
        let after = normals(&nif);
        for (i, (a, b)) in after.iter().zip(&before).enumerate() {
            let want = if i == 0 || i == 3 { Vec3::X } else { *b };
            assert!(a.distance(want) <= tolerance, "{name} {i}: {a:?} != {want:?}");
        }
        assert_eq!(locked(&nif), [lock.to_vec()], "{name}");

        let saved = reload(&mut nif);
        assert_eq!(normals(&saved), after, "{name}");
        assert_eq!(locked(&saved), [lock.to_vec()], "{name}");
    }
}

#[test]
fn replaces_old_locked_data() {
    let src = source("Bones", Vec3::Y, &[1]);
    let mut nif = load("Bones");
    let shape = first_shape(&nif).0;
    nif.edit::<nifly::objects::NiAvObject>(shape).and_then(|mut e| e.assign_extra_data(ExtraIntegers("LOCKEDNORM", [5, 6]))).unwrap();
    let blocks = nif.blocks().count();

    assert!(nif.edit::<nifly::geometry::NiShape>(shape).is_some_and(|mut e| e.apply_locked_normals(src.view_of(first_shape(&src).1).unwrap())));
    // The old block is deleted and the copy added.
    assert_eq!(locked(&nif), [vec![1]]);
    assert_eq!(nif.blocks().count(), blocks);
    assert_eq!(nif.blocks_of::<NiIntegersExtraData>().count(), 1);
}

#[test]
fn calc_shape_normals_keeps_locked_normals() {
    let src = source("Bones", Vec3::X, &[0, 3]);
    let mut nif = load("Bones");
    let shape = first_shape(&nif).0;
    assert!(nif.edit::<nifly::geometry::NiShape>(shape).is_some_and(|mut e| e.apply_locked_normals(src.view_of(first_shape(&src).1).unwrap())));
    assert!(nif.edit::<nifly::geometry::NiShape>(shape).is_some_and(|mut e| e.calc_normals(None)));
    let after = normals(&nif);
    assert_eq!((after[0], after[3]), (Vec3::X, Vec3::X));
    assert_ne!(after[1], Vec3::X);
}

#[test]
fn false_changes_nothing() {
    let mut nif = load("Bones");
    let shape = first_shape(&nif).0;
    let before = (normals(&nif), nif.blocks().count());
    let unchanged = |nif: &NifFile| assert_eq!((normals(nif), nif.blocks().count()), before);

    // No LOCKEDNORM, an empty one, or one under another name.
    let plain = load("Bones");
    assert!(!nif.edit::<nifly::geometry::NiShape>(shape).is_some_and(|mut e| e.apply_locked_normals(plain.view_of(first_shape(&plain).1).unwrap())));
    let empty = source("Bones", Vec3::X, &[]);
    assert!(!nif.edit::<nifly::geometry::NiShape>(shape).is_some_and(|mut e| e.apply_locked_normals(empty.view_of(first_shape(&empty).1).unwrap())));
    let mut named = load("Bones");
    let named_shape = first_shape(&named).0;
    named.edit::<nifly::objects::NiAvObject>(named_shape).and_then(|mut e| e.assign_extra_data(ExtraIntegers("LOCKED", [0]))).unwrap();
    assert!(!nif.edit::<nifly::geometry::NiShape>(shape).is_some_and(|mut e| e.apply_locked_normals(named.view_of(first_shape(&named).1).unwrap())));
    unchanged(&nif);

    // Different vertex counts.
    let cube = source("Cube", Vec3::X, &[0]);
    assert!(!nif.edit::<nifly::geometry::NiShape>(shape).is_some_and(|mut e| e.apply_locked_normals(cube.view_of(first_shape(&cube).1).unwrap())));
    unchanged(&nif);

    // Not a shape, or out of range.
    let src = source("Bones", Vec3::X, &[0]);
    let count = nif.blocks().count();
    for i in [0, count, usize::MAX] {
        assert!(!nif.edit::<nifly::geometry::NiShape>(i).is_some_and(|mut e| e.apply_locked_normals(src.view_of(first_shape(&src).1).unwrap())), "{i}");
    }
    unchanged(&nif);
}
