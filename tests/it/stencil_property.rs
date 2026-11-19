//! Tests for NifFile::stencil_property(_mut) and NiStencilProperty, on assets/Stencil_MW.nif.

use crate::common::{load, reload};
use std::{fs, ptr};

use nifly::{
    file::NifFile,
    shaders::{
        DrawMode, NiStencilProperty, StencilAction, StencilFlags, StencilTest,
    },
};

/// The stencil property of the shape named "Tri EditorMarker_box_02".
fn stencil(nif: &NifFile) -> &NiStencilProperty {
    let shape = nif.shapes().find(|s| s.name() == "Tri EditorMarker_box_02").unwrap();
    nif.view_of(shape).unwrap().stencil_property().unwrap().get()
}

type Fields = (
    bool,
    Option<StencilTest>,
    u32,
    u32,
    Option<StencilAction>,
    Option<StencilAction>,
    Option<StencilAction>,
    Option<DrawMode>,
);

/// (enabled, function, reference, mask, fail, z-fail, pass, draw mode)
fn fields(s: &NiStencilProperty) -> Fields {
    (
        s.enabled(),
        s.function(),
        s.reference(),
        s.mask(),
        s.fail_action(),
        s.z_fail_action(),
        s.pass_action(),
        s.draw_mode(),
    )
}

#[test]
fn reads_stored_fields() {
    let nif = load("Stencil_MW");
    let s = stencil(&nif);
    let expected = (
        true,
        Some(StencilTest::Greater),
        7,
        0xFF,
        Some(StencilAction::Zero),
        Some(StencilAction::Replace),
        Some(StencilAction::Increment),
        Some(DrawMode::Clockwise),
    );
    assert_eq!(fields(s), expected);
    // Morrowind files don't store the packed flags, so they keep their default.
    assert_eq!(s.flags(), StencilFlags::DEFAULT);

    // It's in the shape's property list; the collision shape has none.
    let shapes: Vec<_> = nif.shapes().collect();
    let with: Vec<_> = shapes.iter().map(|s| nif.view_of(*s).unwrap().stencil_property().is_some()).collect();
    assert_eq!(with.iter().filter(|w| **w).count(), 1);
    let shape = nif.shapes().find(|s| s.name() == "Tri EditorMarker_box_02").unwrap();
    let listed = nif.child_blocks_of::<NiStencilProperty>(shape).next().unwrap();
    assert!(ptr::eq(listed, s));
}

#[test]
fn setters_round_trip() {
    let mut nif = load("Stencil_MW");
    let shape = nif.shapes().indexed().find(|(_, s)| s.name() == "Tri EditorMarker_box_02").unwrap().0;
    let mut s_shape = nif.edit::<nifly::geometry::NiShape>(shape).unwrap();
    let mut s_edit = s_shape.stencil_property().unwrap();
    let mut s = s_edit.block();
    s.as_mut().set_enabled(false);
    s.as_mut().set_function(StencilTest::Always);
    s.as_mut().set_reference(1);
    s.as_mut().set_mask(0xF0);
    s.as_mut().set_fail_action(StencilAction::Invert);
    s.as_mut().set_z_fail_action(StencilAction::Decrement);
    s.as_mut().set_pass_action(StencilAction::Keep);
    s.as_mut().set_draw_mode(DrawMode::Both);
    s.set_flags(StencilFlags::from_bits(1));
    let expected = (
        false,
        Some(StencilTest::Always),
        1,
        0xF0,
        Some(StencilAction::Invert),
        Some(StencilAction::Decrement),
        Some(StencilAction::Keep),
        Some(DrawMode::Both),
    );
    assert_eq!(fields(stencil(&nif)), expected);
    assert_eq!(stencil(&nif).flags().bits(), 1);

    // The separate fields are saved in Morrowind files; the packed flags aren't.
    let saved = reload(&mut nif);
    assert_eq!(fields(stencil(&saved)), expected);
    assert_eq!(stencil(&saved).flags(), StencilFlags::DEFAULT);
}

#[test]
fn none_without_one() {
    for asset in ["Bones", "Cone", "Cube", "Icosphere", "Suzanne"] {
        let nif = load(asset);
        assert!(nif.shapes().all(|s| nif.view_of(s).unwrap().stencil_property().is_none()), "{asset}");
    }
    let upstream = "nifly-sys/vendor/nifly/tests/input/TestNifFile_Static_MW.nif";
    let nif = NifFile::load(fs::read(upstream).unwrap().as_slice(), None).unwrap();
    assert!(nif.shapes().all(|s| nif.view_of(s).unwrap().stencil_property().is_none()));

    let nif = load("Stencil_MW");
    let stencil_index = nif.blocks_of::<NiStencilProperty>().indices().next().unwrap();
    let count = nif.blocks().count();
    for index in [0, stencil_index, count, usize::MAX] {
        assert!(nif.view::<nifly::geometry::NiShape>(index).and_then(|v| v.stencil_property()).is_none(), "{index}");
    }
}

#[test]
fn stencil_flag_builders() {
    let parts = StencilFlags::from_bits(0)
        .with_function(StencilTest::Greater)
        .with_pass_action(StencilAction::Increment)
        .with_draw_mode(DrawMode::Both);
    assert_eq!(parts, StencilFlags::DEFAULT);
    let f = parts.with_enabled(true).with_fail_action(StencilAction::Zero).with_z_fail_action(StencilAction::Replace);
    assert_eq!(f.bits(), 19840 + 1 + (1 << 1) + (2 << 4));
    assert_eq!((f.enabled(), f.fail_action(), f.z_fail_action()), (true, Some(StencilAction::Zero), Some(StencilAction::Replace)));
    // Action bits holding 6 aren't an action.
    assert_eq!(StencilFlags::from_bits(6 << 1).fail_action(), None);
}
