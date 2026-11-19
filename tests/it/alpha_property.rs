//! Tests for NifFile::alpha_property(_mut) and NiAlphaProperty.

use crate::common::{load, upstream};
use std::io::Cursor;

use nifly::{
    file::{Index, NifFile},
    shaders::{AlphaFlags, AlphaTest, BlendFactor, NiAlphaProperty},
};

/// For each shape: the block index, flags and threshold of its alpha property.
fn alphas(nif: &NifFile) -> Vec<Option<(Index, u16, u8)>> {
    nif.shapes()
        .map(|s| nif.view_of(s).unwrap().alpha_property().map(|a| (a.index(), a.flags().bits(), a.threshold())))
        .collect()
}

#[test]
fn from_the_shapes_ref() {
    // Skyrim SE BSTriShapes: seven shapes share one property, the last has its own.
    let nif = upstream("TestNifFile_OrderedNode_SE");
    let mut expected = vec![Some((9, 4333, 128)); 7];
    expected.push(Some((51, 4333, 65)));
    assert_eq!(alphas(&nif), expected);

    let nif = upstream("TestNifFile_Static_FO4_132");
    assert_eq!(alphas(&nif), [Some((6, 4844, 130))]);
}

#[test]
fn from_the_property_list() {
    let nif = upstream("TestNifFile_Skinned_MW");
    assert_eq!(alphas(&nif), [Some((6, 237, 0))]);
    let shape = nif.shapes().next().unwrap();
    let listed = nif.child_blocks_of::<NiAlphaProperty>(shape).next().unwrap();
    assert!(std::ptr::eq(listed, nif.view_of(shape).unwrap().alpha_property().unwrap().get()));

    // Skyrim LE NiTriShapes, only some with alpha.
    let nif = upstream("TestNifFile_Animated_LE");
    let found: Vec<_> = alphas(&nif).into_iter().map(|a| a.map(|(i, ..)| i)).collect();
    assert_eq!(found, [None, Some(56), None, Some(65), Some(70)]);
}

#[test]
fn none_without_one() {
    for asset in ["Bones", "Cone", "Cube", "Icosphere", "Suzanne"] {
        let nif = load(asset);
        assert!(alphas(&nif).iter().all(Option::is_none), "{asset}");
        let shape = nif.shapes().indices().next().unwrap();
        assert!(nif.view::<nifly::geometry::NiShape>(shape).and_then(|v| v.alpha_property()).is_none(), "{asset}");
    }
    let nif = upstream("TestNifFile_Static_FO4_132");
    let count = nif.blocks().count();
    // The alpha block itself isn't a shape.
    for index in [6, count, usize::MAX] {
        assert!(nif.view::<nifly::geometry::NiShape>(index).and_then(|v| v.alpha_property()).is_none(), "{index}");
    }
}

#[test]
fn setters_round_trip() {
    let mut nif = upstream("TestNifFile_Static_FO4_132");
    let shape = nif.shapes().indices().next().unwrap();
    let mut alpha_shape = nif.edit::<nifly::geometry::NiShape>(shape).unwrap();
    let mut alpha_edit = alpha_shape.alpha_property().unwrap();
    let mut alpha = alpha_edit.block();
    alpha.as_mut().set_flags(AlphaFlags::from_bits(4845));
    alpha.set_threshold(200);
    assert_eq!(alphas(&nif), [Some((6, 4845, 200))]);

    let mut cur = Cursor::new(Vec::new());
    nif.save(&mut cur, None).unwrap();
    let saved = NifFile::load(cur.into_inner().as_slice(), None).unwrap();
    let a = saved.shapes().views().next().unwrap().alpha_property().unwrap();
    assert_eq!((a.flags().bits(), a.threshold()), (4845, 200));
}

#[test]
fn reads_a_files_flags() {
    // Skyrim SE: blended, not tested.
    let nif = upstream("TestNifFile_OrderedNode_SE");
    let flags = nif.shapes().views().next().unwrap().alpha_property().unwrap().flags();
    assert_eq!(flags.bits(), 4333);
    assert!(flags.blending() && !flags.testing() && !flags.no_sorting());
    assert_eq!((flags.source_blend(), flags.destination_blend()), (Some(BlendFactor::SourceAlpha), Some(BlendFactor::InverseSourceAlpha)));
    assert_eq!(flags.test_function(), AlphaTest::Greater);
}

#[test]
fn alpha_flag_builders() {
    let f = AlphaFlags::from_bits(0)
        .with_source_blend(BlendFactor::SourceAlpha)
        .with_destination_blend(BlendFactor::InverseSourceAlpha)
        .with_testing(true)
        .with_test_function(AlphaTest::Greater);
    assert_eq!(f, AlphaFlags::DEFAULT);
    assert_eq!(AlphaFlags::default(), f);
    assert_eq!(f.with_blending(true).with_no_sorting(true).bits(), 4844 + 1 + 0x2000);
    // Factors 11 to 15 aren't blend factors.
    assert_eq!(AlphaFlags::from_bits(11 << 1).source_blend(), None);
}
