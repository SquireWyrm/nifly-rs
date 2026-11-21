//! Tests for NifFile::assign_alpha_property and NifFile::remove_alpha_property.

use crate::common::{load, reload, save_in_place, upstream};

use nifly::{
    file::{Index, NifFile},
    shaders::{AlphaFlags, NiAlphaProperty},
};

/// Flags and threshold of each shape's alpha property.
fn alphas(nif: &NifFile) -> Vec<Option<(u16, u8)>> {
    nif.shapes()
        .map(|s| {
            nif.view_of(s)
                .unwrap()
                .alpha_property()
                .map(|a| (a.flags().bits(), a.threshold()))
        })
        .collect()
}

fn nth_shape(nif: &NifFile, n: usize) -> Index {
    nif.shapes().indices().nth(n).unwrap()
}

#[test]
fn assigned_and_saved_in_every_layout() {
    // (file, shape): BSTriShape (SE), NiTriShape with the alpha ref (Skyrim LE), NiTriShape
    // with a property list (Oblivion), BSGeometry (Starfield), and a Morrowind shape with no
    // properties or shader at all.
    for (file, shape) in [
        ("assets/Cube", 0),
        ("assets/Bones", 0),
        ("upstream/TestNifFile_Skinned_OB", 0),
        ("upstream/TestNifFile_SF", 0),
        ("upstream/TestNifFile_Static_MW", 1),
    ] {
        let mut nif = match file.split_once('/').unwrap() {
            ("assets", name) => load(name),
            (_, name) => upstream(name),
        };
        let count = nif.blocks().count();
        let index = nth_shape(&nif, shape);
        let new = nif
            .edit::<nifly::geometry::NiShape>(index)
            .map(|mut e| e.assign_alpha_property(AlphaFlags::from_bits(4845), 77))
            .unwrap();
        assert_eq!(new, count, "{file}: appended");
        let alpha = nif
            .view::<nifly::geometry::NiShape>(index)
            .unwrap()
            .alpha_property()
            .unwrap();
        assert_eq!(alpha.index(), new, "{file}");

        // Saving sorts blocks, which can reorder shapes, so look for the values on any shape.
        let saved = reload(&mut nif);
        let with = alphas(&saved)
            .into_iter()
            .filter(|a| *a == Some((4845, 77)))
            .count();
        assert_eq!(with, 1, "{file}: lost on save");
    }
}

#[test]
fn replaces_the_old_property() {
    let mut nif = upstream("TestNifFile_Skinned_MW");
    let shape = nth_shape(&nif, 0);
    let count = nif.blocks().count();
    assert_eq!(alphas(&nif), [Some((237, 0))]);

    // The old block is deleted, so the new one takes the last index.
    let new = nif
        .edit::<nifly::geometry::NiShape>(shape)
        .map(|mut e| e.assign_alpha_property(AlphaFlags::DEFAULT, 9))
        .unwrap();
    assert_eq!(new, count - 1);
    assert_eq!(nif.blocks().count(), count);
    assert_eq!(nif.blocks_of::<NiAlphaProperty>().count(), 1);
    assert_eq!(alphas(&nif), [Some((AlphaFlags::DEFAULT.bits(), 9))]);
    let s = nif.block(shape).unwrap().downcast().unwrap();
    assert_eq!(nif.child_blocks_of::<NiAlphaProperty>(s).count(), 1);
}

#[test]
fn shared_properties_are_kept() {
    // Shapes 0 to 6 share alpha block 9; shape 7 has block 51.
    let mut nif = upstream("TestNifFile_OrderedNode_SE");
    let count = nif.blocks().count();
    let shapes: Vec<_> = nif.shapes().indices().collect();

    assert!(
        nif.edit::<nifly::geometry::NiShape>(shapes[0])
            .is_some_and(|mut e| e.remove_alpha_property())
    );
    assert_eq!(nif.blocks().count(), count, "still used by six shapes");
    let mut expected = vec![Some((4333, 128)); 7];
    expected.push(Some((4333, 65)));
    expected[0] = None;
    assert_eq!(alphas(&nif), expected);

    // Replacing on a shared one gives that shape its own.
    let new = nif
        .edit::<nifly::geometry::NiShape>(shapes[1])
        .map(|mut e| e.assign_alpha_property(AlphaFlags::from_bits(1), 2))
        .unwrap();
    assert_eq!(new, count);
    expected[1] = Some((1, 2));
    assert_eq!(alphas(&nif), expected);

    // Once the last user lets go, block 9 is deleted.
    for &shape in &shapes[2..7] {
        assert!(
            nif.edit::<nifly::geometry::NiShape>(shape)
                .is_some_and(|mut e| e.remove_alpha_property())
        );
    }
    assert_eq!(nif.blocks().count(), count, "9 deleted, the new one added");
    assert_eq!(
        alphas(&nif)[2..],
        [None, None, None, None, None, Some((4333, 65))]
    );
    assert_eq!(nif.blocks_of::<NiAlphaProperty>().count(), 2);
}

#[test]
fn remove_from_the_property_list() {
    let mut nif = upstream("TestNifFile_Skinned_MW");
    let count = nif.blocks().count();
    assert!(
        nif.edit::<nifly::geometry::NiShape>(nth_shape(&nif, 0))
            .is_some_and(|mut e| e.remove_alpha_property())
    );
    assert_eq!(alphas(&nif), [None]);
    assert_eq!(nif.blocks().count(), count - 1);
    assert_eq!(alphas(&reload(&mut nif)), [None]);
}

#[test]
fn rejects_without_changes() {
    let mut nif = upstream("TestNifFile_OrderedNode_SE");
    let before = save_in_place(&mut nif);
    let count = nif.blocks().count();
    // Block 9 is an alpha property, not a shape; the root isn't one either.
    for index in [0, 9, count, usize::MAX] {
        assert_eq!(
            nif.edit::<nifly::geometry::NiShape>(index)
                .map(|mut e| e.assign_alpha_property(AlphaFlags::from_bits(0), 0)),
            None,
            "{index}"
        );
        assert!(
            !nif.edit::<nifly::geometry::NiShape>(index)
                .is_some_and(|mut e| e.remove_alpha_property()),
            "{index}"
        );
    }
    assert_eq!(save_in_place(&mut nif), before);

    // A shape without one.
    let mut cube = load("Cube");
    let before = save_in_place(&mut cube);
    assert!(
        !cube
            .edit::<nifly::geometry::NiShape>(nth_shape(&cube, 0))
            .is_some_and(|mut e| e.remove_alpha_property())
    );
    assert_eq!(save_in_place(&mut cube), before);
}
