//! Tests for keys, animation data blocks and text keys.

use std::fs;

use nifly::{
    animation::{
        Interpolation, Key, KeyGroup, Morph, NiBSplineBasisData, NiBSplineData, NiBoolData,
        NiColorData, NiFloatData, NiKeyframeData, NiMorphData, NiPosData, NiStringPalette,
        NiTransformData, NiUvData, NiVisData, Rotations, Tbc,
    },
    extra_data::{ExtraTextKeys, NiTextKeyExtraData, TextKey},
    file::{NifFile, NifVersion},
    object3d::Vec3,
    objects::NiAvObject,
};

use crate::common::{UPSTREAM, reload_in_place as reload, save, upstream};

const TBC: Tbc = Tbc {
    tension: 0.25,
    bias: -0.5,
    continuity: 0.75,
};

/// Three keys with tangents and TBC values set, whatever the interpolation saves.
fn keys<T: Copy>(values: [T; 3], tangent: T) -> Vec<Key<T>> {
    values
        .iter()
        .enumerate()
        .map(|(i, &value)| Key {
            time: i as f32,
            value,
            forward: tangent,
            backward: tangent,
            tbc: TBC,
        })
        .collect()
}

/// `group` as it reads back: only the fields its interpolation saves.
fn saved<T: Copy + Default>(group: &KeyGroup<T>) -> KeyGroup<T> {
    let keys = group
        .keys
        .iter()
        .map(|k| match group.interpolation {
            Some(Interpolation::Quadratic) => Key {
                tbc: Tbc::default(),
                ..*k
            },
            Some(Interpolation::Tbc) => Key {
                forward: T::default(),
                backward: T::default(),
                ..*k
            },
            _ => Key::new(k.time, k.value),
        })
        .collect();
    KeyGroup {
        interpolation: group.interpolation,
        keys,
    }
}

const ALL: [Interpolation; 4] = [
    Interpolation::Linear,
    Interpolation::Quadratic,
    Interpolation::Tbc,
    Interpolation::Constant,
];

#[test]
fn key_groups_by_interpolation() {
    for version in [NifVersion::MORROWIND, NifVersion::SKYRIM_SE] {
        for interpolation in ALL {
            let floats = KeyGroup::new(interpolation, keys([1.0, 2.0, 3.0], 0.5));
            let points = KeyGroup::new(
                interpolation,
                keys([Vec3::X, Vec3::Y, Vec3::Z], Vec3::new(1.0, 1.0, 1.0)),
            );
            let bools = KeyGroup::new(interpolation, keys([true, false, true], true));
            let colors = KeyGroup::new(interpolation, keys([[1.0, 0.5, 0.25, 1.0]; 3], [0.5; 4]));

            let mut nif = NifFile::create(version);
            let f = nif.add_animation_data::<NiFloatData>();
            let p = nif.add_animation_data::<NiPosData>();
            let b = nif.add_animation_data::<NiBoolData>();
            let c = nif.add_animation_data::<NiColorData>();
            assert!(
                nif.block_of_mut::<NiFloatData>(f)
                    .unwrap()
                    .set_keys(&floats)
            );
            assert!(nif.block_of_mut::<NiPosData>(p).unwrap().set_keys(&points));
            assert!(nif.block_of_mut::<NiBoolData>(b).unwrap().set_keys(&bools));
            assert!(
                nif.block_of_mut::<NiColorData>(c)
                    .unwrap()
                    .set_keys(&colors)
            );

            let nif = reload(&mut nif);
            assert_eq!(
                nif.block_of::<NiFloatData>(f).unwrap().keys(),
                saved(&floats),
                "{interpolation:?}"
            );
            assert_eq!(nif.block_of::<NiPosData>(p).unwrap().keys(), saved(&points));
            assert_eq!(nif.block_of::<NiBoolData>(b).unwrap().keys(), saved(&bools));
            assert_eq!(
                nif.block_of::<NiColorData>(c).unwrap().keys(),
                saved(&colors)
            );
        }
    }
}

#[test]
fn keys_need_an_interpolation() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let f = nif.add_animation_data::<NiFloatData>();
    let before = KeyGroup::new(Interpolation::Linear, vec![Key::new(0.0, 1.0)]);
    let data = nif.block_of_mut::<NiFloatData>(f).unwrap();
    assert!(data.set_keys(&before));
    let none = KeyGroup {
        interpolation: None,
        keys: vec![Key::new(0.0, 2.0)],
    };
    assert!(!data.set_keys(&none));
    assert_eq!(data.keys(), before);
    assert!(
        data.set_keys(&KeyGroup::default()),
        "an empty group needs none"
    );
    assert_eq!(data.keys(), KeyGroup::default());
}

#[test]
fn keyframe_rotations() {
    let quaternions = KeyGroup::new(
        Interpolation::Tbc,
        keys(
            [
                [0.0, 0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0, 0.0],
                [1.0, 0.0, 0.0, 0.0],
            ],
            [0.0; 4],
        ),
    );
    let xyz = [
        KeyGroup::new(Interpolation::Linear, keys([0.0, 1.0, 2.0], 0.0)),
        KeyGroup::new(Interpolation::Quadratic, keys([1.0, 2.0, 3.0], 0.5)),
        KeyGroup::default(),
    ];
    let translations = KeyGroup::new(
        Interpolation::Quadratic,
        keys([Vec3::X, Vec3::Y, Vec3::Z], Vec3::Z),
    );
    let scales = KeyGroup::new(Interpolation::Constant, keys([1.0, 2.0, 1.0], 0.0));

    for version in [NifVersion::MORROWIND, NifVersion::SKYRIM] {
        let mut nif = NifFile::create(version);
        let q = nif.add_animation_data::<NiKeyframeData>();
        let x = nif.add_animation_data::<NiTransformData>();
        let data = nif.block_of_mut::<NiKeyframeData>(q).unwrap();
        assert!(data.set_rotations(&Rotations::Quaternion(quaternions.clone())));
        assert!(data.set_translations(&translations));
        assert!(data.set_scales(&scales));
        let rotations = Rotations::Xyz(xyz.clone());
        assert!(
            nif.block_of_mut::<NiKeyframeData>(x)
                .unwrap()
                .set_rotations(&rotations)
        );

        let nif = reload(&mut nif);
        let data = nif.block_of::<NiKeyframeData>(q).unwrap();
        let Rotations::Quaternion(read) = data.rotations() else {
            panic!("quaternions")
        };
        let only_tbc = |g: &KeyGroup<[f32; 4]>| {
            g.keys
                .iter()
                .map(|k| (k.time, k.value, k.tbc))
                .collect::<Vec<_>>()
        };
        assert_eq!(
            only_tbc(&read),
            only_tbc(&quaternions),
            "quaternions save no tangents"
        );
        assert_eq!(
            (data.translations(), data.scales()),
            (saved(&translations), saved(&scales))
        );
        let data = nif.block_of::<NiTransformData>(x).unwrap();
        assert_eq!(data.rotations(), Rotations::Xyz(xyz.each_ref().map(saved)));
    }
}

#[test]
fn rotations_need_interpolations() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let index = nif.add_animation_data::<NiKeyframeData>();
    let data = nif.block_of_mut::<NiKeyframeData>(index).unwrap();
    let bad = KeyGroup {
        interpolation: None,
        keys: vec![Key::new(0.0, 1.0)],
    };
    assert!(!data.set_rotations(&Rotations::Xyz([
        KeyGroup::default(),
        bad,
        KeyGroup::default()
    ])));
    assert!(!data.set_rotations(&Rotations::Quaternion(KeyGroup {
        interpolation: None,
        keys: vec![Key::new(0.0, [0.0; 4])]
    })));
    assert_eq!(data.rotations(), Rotations::Quaternion(KeyGroup::default()));
}

#[test]
fn uv_and_visibility() {
    let mut nif = NifFile::create(NifVersion::MORROWIND);
    let uv = nif.add_animation_data::<NiUvData>();
    let vis = nif.add_animation_data::<NiVisData>();
    let groups: [KeyGroup<f32>; 4] = ALL.map(|i| KeyGroup::new(i, keys([0.0, 0.5, 1.0], 0.25)));
    let data = nif.block_of_mut::<NiUvData>(uv).unwrap();
    assert!(data.set_u_translations(&groups[0]));
    assert!(data.set_v_translations(&groups[1]));
    assert!(data.set_u_scales(&groups[2]));
    assert!(data.set_v_scales(&groups[3]));
    let visible = vec![Key::new(0.0, true), Key::new(1.5, false)];
    nif.block_of_mut::<NiVisData>(vis)
        .unwrap()
        .set_keys(&visible);

    let nif = reload(&mut nif);
    let data = nif.block_of::<NiUvData>(uv).unwrap();
    let read = [
        data.u_translations(),
        data.v_translations(),
        data.u_scales(),
        data.v_scales(),
    ];
    assert_eq!(read, groups.each_ref().map(saved));
    assert_eq!(nif.block_of::<NiVisData>(vis).unwrap().keys(), visible);
}

#[test]
fn morphs_by_version() {
    let morph = |name: &str, n: usize| Morph {
        name: name.into(),
        keys: KeyGroup::new(
            Interpolation::Linear,
            vec![Key::new(0.0, 0.0), Key::new(1.0, 1.0)],
        ),
        legacy_weight: 0.5,
        vectors: vec![Vec3::new(1.0, 2.0, 3.0); n],
    };
    let morphs = [morph("Base", 3), morph("Smile", 3)];
    let legacy = NifVersion::new(0x0A01006A, 10, 5);
    for version in [
        NifVersion::MORROWIND,
        legacy,
        NifVersion::OBLIVION,
        NifVersion::SKYRIM,
    ] {
        let mut nif = NifFile::create(version);
        let index = nif.add_animation_data::<NiMorphData>();
        let data = nif.block_of_mut::<NiMorphData>(index).unwrap();
        assert!(
            !data.set_morphs(&[morph("A", 3), morph("B", 2)]),
            "vectors per vertex"
        );
        assert_eq!(data.vertex_count(), 0);
        assert!(data.set_morphs(&morphs));
        data.set_relative_targets(false);

        let nif = reload(&mut nif);
        let data = nif.block_of::<NiMorphData>(index).unwrap();
        assert_eq!((data.vertex_count(), data.relative_targets()), (3, false));
        let morrowind = version == NifVersion::MORROWIND;
        let expected: Vec<_> = morphs
            .iter()
            .map(|m| Morph {
                name: if morrowind { "".into() } else { m.name.clone() },
                keys: if morrowind {
                    m.keys.clone()
                } else {
                    KeyGroup::default()
                },
                legacy_weight: if version == legacy {
                    m.legacy_weight
                } else {
                    0.0
                },
                vectors: m.vectors.clone(),
            })
            .collect();
        assert_eq!(data.morphs(), expected, "{version:?}");
    }
}

#[test]
fn bsplines_and_palettes() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let spline = nif.add_animation_data::<NiBSplineData>();
    let basis = nif.add_animation_data::<NiBSplineBasisData>();
    let palette = nif.add_animation_data::<NiStringPalette>();
    let data = nif.block_of_mut::<NiBSplineData>(spline).unwrap();
    data.set_float_control_points(&[0.5, 1.5]);
    data.set_short_control_points(&[-3, 7, 32767]);
    nif.block_of_mut::<NiBSplineBasisData>(basis)
        .unwrap()
        .set_control_point_count(4);
    nif.block_of_mut::<NiStringPalette>(palette)
        .unwrap()
        .set_palette(b"Bip01\0NiTransformController\0");

    let nif = reload(&mut nif);
    let data = nif.block_of::<NiBSplineData>(spline).unwrap();
    assert_eq!(
        (data.float_control_points(), data.short_control_points()),
        (&[0.5, 1.5][..], &[-3, 7, 32767][..])
    );
    assert_eq!(
        nif.block_of::<NiBSplineBasisData>(basis)
            .unwrap()
            .control_point_count(),
        4
    );
    let p = nif.block_of::<NiStringPalette>(palette).unwrap();
    assert_eq!(
        p.palette(),
        "Bip01\0NiTransformController",
        "the final zero byte isn't included"
    );
    assert_eq!(
        (p.string_at(0), p.string_at(6)),
        (Some("Bip01".into()), Some("NiTransformController".into()))
    );
    assert_eq!((p.string_at(27), p.string_at(28)), (Some("".into()), None));
}

#[test]
fn text_keys() {
    let keys = vec![
        TextKey {
            time: 0.0,
            text: "start".into(),
        },
        TextKey {
            time: 2.5,
            text: "end".into(),
        },
    ];
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let root = nif.root_node().unwrap().index();
    let index = nif
        .edit::<NiAvObject>(root)
        .unwrap()
        .assign_extra_data(ExtraTextKeys("Text", &keys))
        .unwrap();
    let nif2 = reload(&mut nif);
    let data = nif2.block_of::<NiTextKeyExtraData>(index).unwrap();
    assert_eq!((data.name(), data.text_keys()), ("Text".into(), keys));

    nif.block_of_mut::<NiTextKeyExtraData>(index)
        .unwrap()
        .set_text_keys(&[]);
    assert!(
        reload(&mut nif)
            .block_of::<NiTextKeyExtraData>(index)
            .unwrap()
            .text_keys()
            .is_empty()
    );
}

/// Every key group and text key list set to itself saves unchanged.
#[test]
fn upstream_files_round_trip() {
    for name in [
        "TestNifFile_Sequence_MW.kf",
        "TestNifFile_Morph_MW.nif",
        "TestNifFile_UVController_MW.nif",
        "TestNifFile_PathController_MW.nif",
        "TestNifFile_Animated_LE.nif",
    ] {
        let mut nif = NifFile::load(
            fs::read(format!("{UPSTREAM}/{name}")).unwrap().as_slice(),
            None,
        )
        .unwrap();
        let before = save(&mut nif);
        let indices: Vec<_> = nif.blocks().indices().collect();
        for i in indices {
            if let Some(d) = nif.block_of_mut::<NiKeyframeData>(i) {
                let (r, t, s) = (d.rotations(), d.translations(), d.scales());
                assert!(d.set_rotations(&r) && d.set_translations(&t) && d.set_scales(&s));
            } else if let Some(d) = nif.block_of_mut::<NiFloatData>(i) {
                let k = d.keys();
                assert!(d.set_keys(&k));
            } else if let Some(d) = nif.block_of_mut::<NiPosData>(i) {
                let k = d.keys();
                assert!(d.set_keys(&k));
            } else if let Some(d) = nif.block_of_mut::<NiBoolData>(i) {
                let k = d.keys();
                assert!(d.set_keys(&k));
            } else if let Some(d) = nif.block_of_mut::<NiUvData>(i) {
                let g = [
                    d.u_translations(),
                    d.v_translations(),
                    d.u_scales(),
                    d.v_scales(),
                ];
                assert!(d.set_u_translations(&g[0]) && d.set_v_translations(&g[1]));
                assert!(d.set_u_scales(&g[2]) && d.set_v_scales(&g[3]));
            } else if let Some(d) = nif.block_of_mut::<NiMorphData>(i) {
                let m = d.morphs();
                assert!(d.set_morphs(&m));
            } else if let Some(d) = nif.block_of_mut::<NiTextKeyExtraData>(i) {
                let k = d.text_keys();
                d.set_text_keys(&k);
            }
        }
        assert!(save(&mut nif) == before, "{name}");
    }
    let morph = upstream("TestNifFile_Morph_MW");
    let data = morph.blocks_of::<NiMorphData>().next().unwrap();
    let first = &data.morphs()[0];
    assert_eq!(
        (first.keys.interpolation, first.keys.keys.len()),
        (Some(Interpolation::Linear), 0),
        "kept without keys"
    );
}
