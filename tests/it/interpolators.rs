//! Tests for interpolators: pose values, key data refs, path, look-at, blend, B-spline and tread
//! interpolators.

use std::io::Cursor;

use nifly::{
    animation::{
        BlendFlags, BlendItem, BsRotAccumTransfInterpolator, BsTreadTransfInterpolator, LookAtFlags,
        NiBSplineBasisData, NiBSplineCompFloatInterpolator, NiBSplineCompTransformInterpolator, NiBSplineData,
        NiBSplineInterpolator, NiBlendBoolInterpolator, NiBlendFloatInterpolator, NiBlendInterpolator,
        NiBlendPoint3Interpolator, NiBlendTransformInterpolator, NiBoolData, NiBoolInterpolator,
        NiBoolTimelineInterpolator, NiFloatData, NiFloatInterpolator, NiKeyframeData, NiLookAtInterpolator,
        NiPathInterpolator, NiPoint3Interpolator, NiPosData, NiTransformData, NiTransformInterpolator, PathFlags,
        QuatTransform, Tread,
    },
    file::{Index, NifFile, NifVersion, SaveOptions},
    object3d::Vec3,
};

use crate::common::{reload_in_place, save, upstream};

/// Oblivion's 10.1.0.106 files, which save the fields older than 10.1.0.110.
const OLD: NifVersion = NifVersion::new(0x0A01006A, 10, 5);

const POSE: QuatTransform = QuatTransform {
    translation: Vec3::new(1.0, 2.0, 3.0),
    rotation: [0.0, 0.0, 1.0, 0.0],
    scale: 2.0,
};

#[test]
fn pose_values() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let b = nif.add_interpolator::<NiBoolInterpolator>();
    let f = nif.add_interpolator::<NiFloatInterpolator>();
    let p = nif.add_interpolator::<NiPoint3Interpolator>();
    let t = nif.add_interpolator::<NiTransformInterpolator>();
    let unset_b = nif.add_interpolator::<NiBoolTimelineInterpolator>();
    let unset_f = nif.add_interpolator::<NiFloatInterpolator>();
    let unset_p = nif.add_interpolator::<NiPoint3Interpolator>();
    nif.block_of_mut::<NiBoolInterpolator>(b).unwrap().set_value(Some(true));
    nif.block_of_mut::<NiFloatInterpolator>(f).unwrap().set_value(Some(0.5));
    nif.block_of_mut::<NiPoint3Interpolator>(p).unwrap().set_value(Some(Vec3::X));
    nif.block_of_mut::<NiTransformInterpolator>(t).unwrap().set_transform(POSE);
    nif.block_of_mut::<NiBoolTimelineInterpolator>(unset_b).unwrap().set_value(None);
    nif.block_of_mut::<NiFloatInterpolator>(unset_f).unwrap().set_value(None);
    nif.block_of_mut::<NiPoint3Interpolator>(unset_p).unwrap().set_value(None);

    let nif = reload_in_place(&mut nif);
    assert_eq!(nif.block_of::<NiBoolInterpolator>(b).unwrap().value(), Some(true));
    assert_eq!(nif.block_of::<NiFloatInterpolator>(f).unwrap().value(), Some(0.5));
    assert_eq!(nif.block_of::<NiPoint3Interpolator>(p).unwrap().value(), Some(Vec3::X));
    assert_eq!(nif.block_of::<NiTransformInterpolator>(t).unwrap().transform(), POSE);
    assert_eq!(nif.block_of::<NiBoolTimelineInterpolator>(unset_b).unwrap().value(), None);
    assert_eq!(nif.block_of::<NiFloatInterpolator>(unset_f).unwrap().value(), None);
    assert_eq!(nif.block_of::<NiPoint3Interpolator>(unset_p).unwrap().value(), None);
}

#[test]
fn key_data_types() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let float = nif.add_interpolator::<NiFloatInterpolator>();
    let transform = nif.add_interpolator::<BsRotAccumTransfInterpolator>();
    let bool_ = nif.add_interpolator::<NiBoolInterpolator>();
    let floats = nif.add_animation_data::<NiFloatData>();
    let points = nif.add_animation_data::<NiPosData>();
    let keyframes = nif.add_animation_data::<NiKeyframeData>();
    let transforms = nif.add_animation_data::<NiTransformData>();
    let bools = nif.add_animation_data::<NiBoolData>();

    let mut edit = nif.edit::<NiFloatInterpolator>(float).unwrap();
    assert!(!edit.set_key_data(Some(points)), "not float data");
    assert!(edit.set_key_data(Some(floats)));
    let mut edit = nif.edit::<BsRotAccumTransfInterpolator>(transform).unwrap();
    assert!(!edit.set_key_data(Some(keyframes)), "transform data only");
    assert!(edit.set_key_data(Some(transforms)));
    assert!(nif.edit::<NiBoolInterpolator>(bool_).unwrap().set_key_data(Some(bools)));

    let mut nif = reload_in_place(&mut nif);
    let float = nif.blocks_of::<NiFloatInterpolator>().views().next().unwrap();
    let data = float.key_data().unwrap().index();
    assert!(nif.block_of::<NiFloatData>(data).is_some());
    let transform = nif.blocks_of::<BsRotAccumTransfInterpolator>().views().next().unwrap();
    assert!(transform.key_data().is_some());
    let index = transform.index();
    let mut edit = nif.edit::<BsRotAccumTransfInterpolator>(index).unwrap();
    assert!(edit.set_key_data(None));
    assert!(edit.key_data().is_none());
}

#[test]
fn path_interpolator() {
    for version in [NifVersion::OBLIVION, NifVersion::SKYRIM_SE] {
        let mut nif = NifFile::create(version);
        let path = nif.add_interpolator::<NiPathInterpolator>();
        let points = nif.add_animation_data::<NiPosData>();
        let floats = nif.add_animation_data::<NiFloatData>();
        let flags = PathFlags::CURVE_TYPE_OPEN | PathFlags::FOLLOW | PathFlags::from_bits_retain(1 << 15);
        let p = nif.block_of_mut::<NiPathInterpolator>(path).unwrap();
        assert_eq!(p.flags(), PathFlags::CV_DATA_NEEDS_UPDATE | PathFlags::CURVE_TYPE_OPEN, "new block");
        p.set_flags(flags);
        p.set_bank_direction(-1);
        p.set_max_bank_angle(0.5);
        p.set_smoothing(0.25);
        p.set_follow_axis(2);
        let mut edit = nif.edit::<NiPathInterpolator>(path).unwrap();
        assert!(!edit.set_path_data(Some(floats)) && !edit.set_percent_data(Some(points)));
        assert!(edit.set_path_data(Some(points)) && edit.set_percent_data(Some(floats)));

        let nif = reload_in_place(&mut nif);
        let p = nif.view::<NiPathInterpolator>(path).unwrap();
        assert_eq!((p.flags(), p.bank_direction(), p.max_bank_angle(), p.smoothing(), p.follow_axis()), (flags, -1, 0.5, 0.25, 2));
        assert_eq!((p.path_data().unwrap().index(), p.percent_data().unwrap().index()), (points, floats));
    }
}

#[test]
fn look_at_interpolator() {
    for version in [OLD, NifVersion::SKYRIM_SE] {
        let mut nif = NifFile::create(version);
        let root = nif.root_node().unwrap().index();
        let look = nif.add_interpolator::<NiLookAtInterpolator>();
        let point = nif.add_interpolator::<NiPoint3Interpolator>();
        let roll = nif.add_interpolator::<NiFloatInterpolator>();
        let scale = nif.add_interpolator::<NiFloatInterpolator>();
        let l = nif.block_of_mut::<NiLookAtInterpolator>(look).unwrap();
        l.set_flags(LookAtFlags::FLIP | LookAtFlags::Z_AXIS);
        l.set_look_at_name("Target");
        l.set_transform(POSE);
        l.set_transform_valid([true, false, true]);
        let mut edit = nif.edit::<NiLookAtInterpolator>(look).unwrap();
        assert!(!edit.set_look_at_node(Some(point)), "not a node");
        assert!(!edit.set_look_at_translation(Some(roll)) && !edit.set_look_at_roll(Some(point)));
        assert!(edit.set_look_at_node(Some(root)) && edit.set_look_at_translation(Some(point)));
        assert!(edit.set_look_at_roll(Some(roll)) && edit.set_look_at_scale(Some(scale)));

        let nif = reload_in_place(&mut nif);
        let l = nif.view::<NiLookAtInterpolator>(look).unwrap();
        assert_eq!((l.flags(), l.look_at_name(), l.transform()), (LookAtFlags::FLIP | LookAtFlags::Z_AXIS, "Target".into(), POSE));
        let valid = if version == OLD { [true, false, true] } else { [false; 3] };
        assert_eq!(l.transform_valid(), valid, "saved before 10.1.0.110");
        assert_eq!(l.look_at_node().unwrap().index(), root);
        let subs = [l.look_at_translation().map(|v| v.index()), l.look_at_roll().map(|v| v.index()), l.look_at_scale().map(|v| v.index())];
        assert_eq!(subs, [Some(point), Some(roll), Some(scale)]);
    }
}

fn item(interpolator: Index, weight: f32) -> BlendItem {
    BlendItem { interpolator: Some(interpolator), weight, normalized_weight: 0.5, priority: 3, priority_32: 7, ease_spinner: 1.0 }
}

#[test]
fn blend_items() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    nif.add_node("Gone", Default::default(), None).unwrap();
    let blend = nif.add_interpolator::<NiBlendFloatInterpolator>();
    let inner = nif.add_interpolator::<NiBlendFloatInterpolator>();
    let a = nif.add_interpolator::<NiFloatInterpolator>();
    let b = nif.add_interpolator::<NiFloatInterpolator>();
    let data = nif.add_animation_data::<NiFloatData>();
    assert_eq!(nif.block_of::<NiBlendInterpolator>(blend).unwrap().flags(), BlendFlags::MANAGER_CONTROLLED, "new block");

    // Managed blends don't save their items, so the array size stays.
    let mut edit = nif.edit::<NiBlendFloatInterpolator>(blend).unwrap();
    edit.set_array_size(2);
    assert!(edit.set_blend_items(&[item(a, 1.0)]));
    assert_eq!(edit.view().array_size(), 2);

    let mut edit = nif.edit::<NiBlendFloatInterpolator>(blend).unwrap();
    edit.set_flags(BlendFlags::empty());
    assert!(!edit.set_blend_items(&[item(data, 1.0)]), "not an interpolator");
    assert!(!edit.set_blend_items(&[item(blend, 1.0)]), "itself");
    assert!(!edit.set_blend_items(&vec![BlendItem::default(); 256]), "a byte holds the count");
    assert!(nif.edit::<NiBlendFloatInterpolator>(inner).unwrap().set_blend_items(&[item(blend, 1.0)]));
    let mut edit = nif.edit::<NiBlendFloatInterpolator>(blend).unwrap();
    assert!(!edit.set_blend_items(&[item(inner, 1.0)]), "the inner blend uses this one");
    assert!(!edit.set_single_interpolator(Some(inner)));
    assert!(nif.edit::<NiBlendFloatInterpolator>(inner).unwrap().set_blend_items(&[]));
    let items = [item(a, 0.25), BlendItem::default(), item(b, 0.75)];
    let mut edit = nif.edit::<NiBlendFloatInterpolator>(blend).unwrap();
    assert!(edit.set_blend_items(&items));
    assert_eq!(edit.view().array_size(), 3);
    edit.set_value(Some(0.5));

    assert_eq!((nif.block_ref_count(a, false), nif.block_ref_count(b, false)), (1, 1), "items are child refs");

    let mut nif = reload_in_place(&mut nif);
    let read = nif.view::<NiBlendFloatInterpolator>(blend).unwrap().blend_items();
    let expected: Vec<_> = items.iter().map(|i| BlendItem { priority_32: 0, ..*i }).collect();
    assert_eq!(read, expected, "priority_32 is saved before 10.1.0.110");
    assert_eq!(nif.block_of::<NiBlendFloatInterpolator>(blend).unwrap().value(), Some(0.5));

    // Deleting an earlier block renumbers the items' refs.
    assert!(nif.delete_node("Gone"));
    let read = nif.view::<NiBlendFloatInterpolator>(blend - 1).unwrap().blend_items();
    assert_eq!((read[0].interpolator, read[2].interpolator), (Some(a - 1), Some(b - 1)));
}

#[test]
fn old_blend_fields() {
    let mut nif = NifFile::create(OLD);
    let blend = nif.add_interpolator::<NiBlendTransformInterpolator>();
    let single = nif.add_interpolator::<NiTransformInterpolator>();
    let others = [
        nif.add_interpolator::<NiBlendBoolInterpolator>(),
        nif.add_interpolator::<NiBlendPoint3Interpolator>(),
    ];
    let mut edit = nif.edit::<NiBlendTransformInterpolator>(blend).unwrap();
    assert!(edit.set_blend_items(&[item(single, 1.0)]));
    assert!(edit.set_single_interpolator(Some(single)));
    let b = &mut *edit;
    b.set_transform(POSE);
    b.set_transform_valid([true, true, false]);
    let base = b;
    base.set_manager_controlled(true);
    base.set_only_use_highest_weight(true);
    base.set_weight_threshold(0.125);
    base.set_array_grow_by(4);
    base.set_interpolator_count(1);
    base.set_single_index_16(0);
    base.set_high_priority_32(-5);
    base.set_next_high_priority_32(-6);
    base.set_single_time(1.5);
    nif.block_of_mut::<NiBlendBoolInterpolator>(others[0]).unwrap().set_value(Some(true));
    nif.block_of_mut::<NiBlendPoint3Interpolator>(others[1]).unwrap().set_value(Some(Vec3::Y));

    let nif = reload_in_place(&mut nif);
    let b = nif.view::<NiBlendTransformInterpolator>(blend).unwrap();
    assert_eq!((b.transform(), b.transform_valid()), (POSE, [true, true, false]));
    assert!(b.manager_controlled() && b.only_use_highest_weight());
    assert_eq!((b.weight_threshold(), b.array_grow_by(), b.interpolator_count()), (0.125, 4, 1));
    assert_eq!((b.single_index_16(), b.high_priority_32(), b.next_high_priority_32()), (0, -5, -6));
    assert_eq!(b.single_interpolator().map(|v| v.index()), None, "saved from 10.1.0.108 to 10.1.0.111");
    assert_eq!(b.blend_items()[0].priority_32, 7, "priority_32 before 10.1.0.110");
    assert_eq!(nif.block_of::<NiBlendBoolInterpolator>(others[0]).unwrap().value(), Some(true));
    assert_eq!(nif.block_of::<NiBlendPoint3Interpolator>(others[1]).unwrap().value(), Some(Vec3::Y));
}

#[test]
fn bspline_interpolators() {
    let mut nif = NifFile::create(NifVersion::OBLIVION);
    let float = nif.add_interpolator::<NiBSplineCompFloatInterpolator>();
    let transform = nif.add_interpolator::<NiBSplineCompTransformInterpolator>();
    let data = nif.add_animation_data::<NiBSplineData>();
    let basis = nif.add_animation_data::<NiBSplineBasisData>();
    let f = nif.block_of_mut::<NiBSplineCompFloatInterpolator>(float).unwrap();
    f.set_base(1.0);
    f.set_offset(4);
    f.set_bias(0.5);
    f.set_multiplier(2.0);
    let s = f;
    s.set_start_time(0.25);
    s.set_stop_time(3.0);
    let t = nif.block_of_mut::<NiBSplineCompTransformInterpolator>(transform).unwrap();
    t.set_rotation_bias(0.5);
    t.set_scale_multiplier(3.0);
    let base = t;
    base.set_transform(POSE);
    base.set_rotation_offset(8);
    base.set_scale_offset(u32::MAX);
    let mut edit = nif.edit::<NiBSplineCompFloatInterpolator>(float).unwrap();
    assert!(!edit.set_spline_data(Some(basis)) && !edit.set_spline_basis(Some(data)));
    assert!(edit.set_spline_data(Some(data)) && edit.set_spline_basis(Some(basis)));

    let nif = reload_in_place(&mut nif);
    let f = nif.view::<NiBSplineCompFloatInterpolator>(float).unwrap();
    assert_eq!((f.base(), f.offset(), f.bias(), f.multiplier()), (1.0, 4, 0.5, 2.0));
    let s: &NiBSplineInterpolator = &f;
    assert_eq!((s.start_time(), s.stop_time()), (0.25, 3.0));
    assert_eq!((f.spline_data().unwrap().index(), f.spline_basis().unwrap().index()), (data, basis));
    let t = nif.block_of::<NiBSplineCompTransformInterpolator>(transform).unwrap();
    assert_eq!((t.rotation_bias(), t.scale_multiplier(), t.transform(), t.rotation_offset(), t.scale_offset()), (0.5, 3.0, POSE, 8, u32::MAX));
}

#[test]
fn treads() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let index = nif.add_interpolator::<BsTreadTransfInterpolator>();
    let data = nif.add_animation_data::<NiFloatData>();
    let treads = vec![Tread { name: "Tread01".into(), transform_1: POSE, transform_2: QuatTransform::default() }];
    nif.block_of_mut::<BsTreadTransfInterpolator>(index).unwrap().set_treads(&treads);
    assert!(nif.edit::<BsTreadTransfInterpolator>(index).unwrap().set_key_data(Some(data)));
    let nif = reload_in_place(&mut nif);
    let t = nif.view::<BsTreadTransfInterpolator>(index).unwrap();
    assert_eq!((t.treads(), t.key_data().unwrap().index()), (treads, data));
}

/// Every interpolator field and ref set to itself saves unchanged.
#[test]
fn upstream_round_trip() {
    let mut nif = upstream("TestNifFile_Animated_LE");
    let raw = |nif: &mut NifFile| {
        let mut c = Cursor::new(Vec::new());
        nif.save(&mut c, Some(SaveOptions { optimize: false, sort_blocks: false })).unwrap();
        c.into_inner()
    };
    let before = raw(&mut nif);
    let indices: Vec<_> = nif.blocks().indices().collect();
    for i in indices {
        if let Some(mut e) = nif.edit::<NiFloatInterpolator>(i) {
            let d = e.view().key_data().map(|d| d.index());
            assert!(e.set_key_data(d));
            let v = e.view().value();
            e.set_value(v);
        } else if let Some(mut e) = nif.edit::<NiBoolInterpolator>(i) {
            let d = e.view().key_data().map(|d| d.index());
            assert!(e.set_key_data(d));
            let v = e.view().value();
            e.set_value(v);
        } else if let Some(mut e) = nif.edit::<NiTransformInterpolator>(i) {
            let d = e.view().key_data().map(|d| d.index());
            assert!(e.set_key_data(d));
            let t = e.view().transform();
            e.set_transform(t);
        } else if let Some(mut e) = nif.edit::<NiBlendInterpolator>(i) {
            let items = e.view().blend_items();
            assert!(e.set_blend_items(&items));
            let (f, s) = (e.view().flags(), e.view().array_size());
            let b = &mut *e;
            b.set_flags(f);
            b.set_array_size(s);
        }
    }
    assert!(raw(&mut nif) == before);
    assert!(!save(&mut nif).is_empty());
}
