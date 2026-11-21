//! Tests for the extra data types beyond the basic values (binary, lists, furniture, connect
//! points, markers, decals, bone data, ...): reading them and creating them with
//! NifFile::assign_extra_data.

use crate::common::{first, reload, upstream};

use nifly::{
    extra_data::{
        BoneLod, BoneTranslation, BoneTranslations, BsBehaviorGraphExtraData, BsBoneLodExtraData,
        BsBound, BsConnectPointChildren, BsConnectPointParents, BsDecalPlacementVectorExtraData,
        BsDistantObjectExtraData, BsDistantObjectLargeRefExtraData, BsEyeCenterExtraData,
        BsFurnitureMarker, BsFurnitureMarkerNode, BsInvMarker, BsPositionData, BsWArray,
        ConnectPoint, DecalPoint, EntryPoints, ExtraBehaviorGraph, ExtraBinary, ExtraBoneLods,
        ExtraBoneTranslations, ExtraBsBound, ExtraColor, ExtraConnectPointChildren,
        ExtraConnectPointParents, ExtraDecalPlacement, ExtraDistantObject,
        ExtraDistantObjectLargeRef, ExtraEyeCenter, ExtraFloats, ExtraFurnitureMarker,
        ExtraFurnitureMarkerNode, ExtraInvMarker, ExtraPositionData, ExtraSkinAttach, ExtraStrings,
        ExtraVector, ExtraVertWeights, ExtraWArray, FurniturePosition, NiBinaryExtraData,
        NiColorExtraData, NiFloatExtraData, NiFloatsExtraData, NiStringsExtraData,
        NiVectorExtraData, NiVertWeightsExtraData, SkinAttach,
    },
    file::{NifFile, NifVersion},
    object3d::Vec3,
};

#[test]
fn reads_existing() {
    let nif = upstream("TestNifFile_Furniture_Col_SE");
    let frn = first::<BsFurnitureMarkerNode>(&nif);
    assert_eq!(frn.name(), "FRN");
    let positions = frn.positions();
    assert_eq!(positions.len(), 1);
    assert_eq!(
        (positions[0].animation_type, positions[0].entry_points),
        (1, EntryPoints::BEHIND)
    );

    let nif = upstream("TestNifFile_Animated_LE");
    let graph = first::<BsBehaviorGraphExtraData>(&nif);
    assert_eq!(
        (graph.name(), graph.graph_file()),
        ("BGED".into(), "Magic\\BeginLoopEnd.hkx".into())
    );
    assert!(!graph.controls_base_skeleton());

    let nif = upstream("TestNifFile_Skinned_OB");
    let binary = first::<NiBinaryExtraData>(&nif);
    assert_eq!(binary.name(), "Tangent space (binormal & tangent vectors)");
    assert_eq!(binary.data().len(), 32496);
}

#[test]
fn skyrim_round_trip() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let root = nif.root_index().unwrap();
    let position = FurniturePosition {
        offset: Vec3::new(1.0, 2.0, 3.0),
        heading: 1.5,
        animation_type: 4,
        entry_points: EntryPoints::FRONT | EntryPoints::LEFT,
        ..Default::default()
    };
    nif.edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraBinary("Bin", [1u8, 2, 3])))
        .unwrap();
    nif.edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraFloats("Floats", [0.5, -1.0])))
        .unwrap();
    nif.edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraStrings("Strings", ["a", "", "ccc"])))
        .unwrap();
    nif.edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraFurnitureMarkerNode("FRN", [position])))
        .unwrap();
    nif.edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraInvMarker("INV", [4712, 6283, 100], 1.5)))
        .unwrap();
    nif.edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| {
            e.assign_extra_data(ExtraBsBound(
                "BBX",
                Vec3::new(0.0, 0.0, 10.0),
                Vec3::new(5.0, 6.0, 7.0),
            ))
        })
        .unwrap();
    nif.edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraBehaviorGraph("BGED", "Graph\\Test.hkx", true)))
        .unwrap();

    let saved = reload(&mut nif);
    assert_eq!(first::<NiBinaryExtraData>(&saved).data(), [1, 2, 3]);
    assert_eq!(first::<NiFloatsExtraData>(&saved).values(), [0.5, -1.0]);
    let strings: Vec<_> = first::<NiStringsExtraData>(&saved).values().collect();
    assert_eq!(strings, ["a", "", "ccc"]);
    let frn = first::<BsFurnitureMarker>(&saved);
    assert!(saved.blocks_of::<BsFurnitureMarkerNode>().next().is_some());
    assert_eq!(
        (frn.name(), frn.positions()),
        ("FRN".into(), vec![position])
    );
    let inv = first::<BsInvMarker>(&saved);
    assert_eq!((inv.rotation(), inv.zoom()), ([4712, 6283, 100], 1.5));
    let bound = first::<BsBound>(&saved);
    assert_eq!(
        (bound.center(), bound.half_extents()),
        (Vec3::new(0.0, 0.0, 10.0), Vec3::new(5.0, 6.0, 7.0))
    );
    let graph = first::<BsBehaviorGraphExtraData>(&saved);
    assert_eq!(
        (graph.graph_file(), graph.controls_base_skeleton()),
        ("Graph\\Test.hkx".into(), true)
    );
}

#[test]
fn furniture_fields_by_version() {
    let position = FurniturePosition {
        offset: Vec3::new(1.0, 2.0, 3.0),
        orientation: 90,
        position_ref_1: 7,
        position_ref_2: 8,
        heading: 1.5,
        animation_type: 1,
        entry_points: EntryPoints::BEHIND | EntryPoints::from_bits_retain(1 << 15),
    };

    // Oblivion stores the orientation and position references.
    let mut nif = NifFile::create(NifVersion::OBLIVION);
    let root = nif.root_index().unwrap();
    nif.edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraFurnitureMarker("FRN", [position])))
        .unwrap();
    let saved = reload(&mut nif);
    let p = first::<BsFurnitureMarker>(&saved).positions()[0];
    assert!(saved.blocks_of::<BsFurnitureMarkerNode>().next().is_none());
    assert_eq!(
        (p.offset, p.orientation, p.position_ref_1, p.position_ref_2),
        (position.offset, 90, 7, 8)
    );
    assert_eq!(
        (p.heading, p.animation_type, p.entry_points),
        (0.0, 0, EntryPoints::empty())
    );

    // Skyrim stores the heading, animation type and entry points.
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let root = nif.root_index().unwrap();
    nif.edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraFurnitureMarkerNode("FRN", [position])))
        .unwrap();
    let saved = reload(&mut nif);
    let p = first::<BsFurnitureMarker>(&saved).positions()[0];
    assert_eq!(
        (p.offset, p.heading, p.animation_type, p.entry_points),
        (position.offset, 1.5, 1, position.entry_points),
        "unnamed bits are kept"
    );
    assert_eq!(
        (p.orientation, p.position_ref_1, p.position_ref_2),
        (0, 0, 0)
    );
}

#[test]
fn connect_points_round_trip() {
    let points = vec![
        ConnectPoint {
            parent: "WeaponMagazine".into(),
            name: "P-Mag".into(),
            rotation: [0.5, 0.5, -0.5, 0.5],
            translation: Vec3::new(1.0, -2.0, 3.0),
            scale: 1.0,
        },
        ConnectPoint {
            parent: "".into(),
            name: "P-Scope".into(),
            rotation: [1.0, 0.0, 0.0, 0.0],
            translation: Vec3::ZERO,
            scale: 0.5,
        },
    ];
    let mut nif = NifFile::create(NifVersion::FALLOUT_4);
    let root = nif.root_index().unwrap();
    nif.edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraConnectPointParents("CPA", &points)))
        .unwrap();
    nif.edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| {
            e.assign_extra_data(ExtraConnectPointChildren(
                "CPT",
                false,
                ["P-Mag", "P-Scope"],
            ))
        })
        .unwrap();

    let saved = reload(&mut nif);
    let parents = first::<BsConnectPointParents>(&saved);
    assert_eq!(
        (parents.name(), parents.connect_points()),
        ("CPA".into(), points)
    );
    let children = first::<BsConnectPointChildren>(&saved);
    assert!(!children.skinned());
    assert_eq!(children.targets().collect::<Vec<_>>(), ["P-Mag", "P-Scope"]);
}

#[test]
fn empty_lists() {
    let mut nif = NifFile::create(NifVersion::FALLOUT_4);
    let root = nif.root_index().unwrap();
    nif.edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraBinary("Bin", [])))
        .unwrap();
    nif.edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraFloats("Floats", [])))
        .unwrap();
    nif.edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraStrings("Strings", Vec::<&str>::new())))
        .unwrap();
    nif.edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraFurnitureMarkerNode("FRN", [])))
        .unwrap();
    nif.edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraConnectPointParents("CPA", [])))
        .unwrap();
    nif.edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| {
            e.assign_extra_data(ExtraConnectPointChildren("CPT", true, Vec::<&str>::new()))
        })
        .unwrap();

    let saved = reload(&mut nif);
    assert!(first::<NiBinaryExtraData>(&saved).data().is_empty());
    assert!(first::<NiFloatsExtraData>(&saved).values().is_empty());
    assert_eq!(first::<NiStringsExtraData>(&saved).values().len(), 0);
    assert!(first::<BsFurnitureMarker>(&saved).positions().is_empty());
    assert!(
        first::<BsConnectPointParents>(&saved)
            .connect_points()
            .is_empty()
    );
    let children = first::<BsConnectPointChildren>(&saved);
    assert!(children.skinned() && children.targets().len() == 0);
}

#[test]
fn invalid_target_changes_nothing() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let count = nif.blocks().count();
    for target in [count, usize::MAX] {
        assert_eq!(
            nif.edit::<nifly::objects::NiAvObject>(target)
                .and_then(|mut e| e.assign_extra_data(ExtraFloats("Floats", [1.0]))),
            None
        );
        assert_eq!(
            nif.edit::<nifly::objects::NiAvObject>(target)
                .and_then(|mut e| e.assign_extra_data(ExtraInvMarker("INV", [0; 3], 1.0))),
            None
        );
    }
    assert_eq!(nif.blocks().count(), count);
}

#[test]
fn value_and_list_types_round_trip() {
    let mut nif = NifFile::create(NifVersion::FALLOUT_4);
    let root = nif.root_index().unwrap();
    nif.edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraColor("Color", [0.25, 0.5, 0.75, 1.0])))
        .unwrap();
    nif.edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraVector("Vector", [1.0, -2.0, 3.0, -4.0])))
        .unwrap();
    nif.edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraVertWeights("Weights", [0.0, 0.5, 1.0])))
        .unwrap();
    nif.edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraEyeCenter("Eyes", [1.0, 2.0, 3.0, 4.0])))
        .unwrap();
    nif.edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraWArray("W", [7u32, 8, 9])))
        .unwrap();
    nif.edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraDistantObject("Distant", 0x1234)))
        .unwrap();
    nif.edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraDistantObjectLargeRef("Large", false)))
        .unwrap();

    let saved = reload(&mut nif);
    assert_eq!(
        first::<NiColorExtraData>(&saved).value(),
        [0.25, 0.5, 0.75, 1.0]
    );
    assert_eq!(
        first::<NiVectorExtraData>(&saved).value(),
        [1.0, -2.0, 3.0, -4.0]
    );
    assert_eq!(
        first::<NiVertWeightsExtraData>(&saved).weights(),
        [0.0, 0.5, 1.0]
    );
    assert_eq!(
        first::<BsEyeCenterExtraData>(&saved).values(),
        [1.0, 2.0, 3.0, 4.0]
    );
    assert_eq!(first::<BsWArray>(&saved).values(), [7, 8, 9]);
    assert_eq!(
        first::<BsDistantObjectExtraData>(&saved).raw_flags(),
        0x1234
    );
    assert!(!first::<BsDistantObjectLargeRefExtraData>(&saved).large_ref());
}

#[test]
fn position_data_rounds_to_half_floats() {
    let mut nif = NifFile::create(NifVersion::FALLOUT_4);
    let root = nif.root_index().unwrap();
    nif.edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraPositionData("Pos", [1.5, -0.25, 0.1, 70000.0])))
        .unwrap();
    let values = first::<BsPositionData>(&reload(&mut nif)).values();
    // Exact in 16 bits, rounded, and too large (infinity).
    assert_eq!(values[..2], [1.5, -0.25]);
    assert!((values[2] - 0.1).abs() < 1e-4 && values[2] != 0.1);
    assert!(values[3].is_infinite());
}

#[test]
fn decals_and_bones_round_trip() {
    let decals = vec![
        vec![
            DecalPoint {
                position: Vec3::new(1.0, 2.0, 3.0),
                normal: Vec3::Z,
            },
            DecalPoint {
                position: Vec3::new(-1.0, 0.0, 0.5),
                normal: Vec3::X,
            },
        ],
        vec![],
        vec![DecalPoint {
            position: Vec3::ZERO,
            normal: Vec3::Y,
        }],
    ];
    let lods = vec![
        BoneLod {
            distance: 100,
            bone: "NPC Head".into(),
        },
        BoneLod {
            distance: 0,
            bone: "".into(),
        },
    ];
    let translations = vec![BoneTranslation {
        bone: "Bone1".into(),
        translation: Vec3::new(0.5, 0.25, 0.125),
    }];

    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let root = nif.root_index().unwrap();
    nif.edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraDecalPlacement("DVPG", 0.5, &decals)))
        .unwrap();
    nif.edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraBoneLods("BSBoneLOD", &lods)))
        .unwrap();
    nif.edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraBoneTranslations("Translations", &translations)))
        .unwrap();

    let saved = reload(&mut nif);
    let decal = first::<BsDecalPlacementVectorExtraData>(&saved);
    assert_eq!(decal.blocks(), decals);
    // It's also a float extra data.
    let value: &NiFloatExtraData = decal;
    assert_eq!(value.value(), 0.5);
    assert_eq!(first::<BsBoneLodExtraData>(&saved).bone_lods(), lods);
    assert_eq!(
        first::<BoneTranslations>(&saved).translations(),
        translations
    );
}

#[test]
fn sixteen_bit_counts_are_limited() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let root = nif.root_index().unwrap();
    let count = nif.blocks().count();
    assert_eq!(
        nif.edit::<nifly::objects::NiAvObject>(root)
            .and_then(|mut e| e.assign_extra_data(ExtraVertWeights("W", vec![0.0; 65536]))),
        None
    );
    let point = DecalPoint::default();
    assert_eq!(
        nif.edit::<nifly::objects::NiAvObject>(root).and_then(
            |mut e| e.assign_extra_data(ExtraDecalPlacement("D", 0.0, [vec![point; 65536]]))
        ),
        None
    );
    assert_eq!(
        nif.edit::<nifly::objects::NiAvObject>(root).and_then(
            |mut e| e.assign_extra_data(ExtraDecalPlacement("D", 0.0, vec![[point]; 65536]))
        ),
        None
    );
    assert_eq!(nif.blocks().count(), count);

    // The largest counts are accepted.
    let index = nif
        .edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraVertWeights("W", vec![1.0; 65535])))
        .unwrap();
    assert_eq!(
        nif.block_of::<NiVertWeightsExtraData>(index)
            .unwrap()
            .weights()
            .len(),
        65535
    );
}

#[test]
fn skin_attach() {
    // A Starfield shape's bone list comes from its SkinAttach.
    let mut nif = upstream("TestNifFile_SF");
    let shape = nif.shapes().views().next().unwrap();
    let attach = shape
        .extra_data()
        .of::<SkinAttach>()
        .views()
        .next()
        .unwrap();
    let bones: Vec<_> = attach.bones().map(|b| b.to_owned()).collect();
    assert_eq!(attach.name(), "SkinBMP");
    assert_eq!((bones.len(), bones[0].as_slice()), (38, &b"C_Chest"[..]));
    assert_eq!(shape.bone_list(), bones);

    // Setting the bone list rewrites it.
    let (shape, attach) = (shape.index(), attach.index());
    assert!(
        nif.edit::<nifly::geometry::NiShape>(shape)
            .unwrap()
            .set_bone_list(["A", "B"])
    );
    assert_eq!(
        nif.view::<SkinAttach>(attach)
            .unwrap()
            .bones()
            .collect::<Vec<_>>(),
        ["A", "B"]
    );

    // A new one.
    let mut nif = NifFile::create(NifVersion::FALLOUT_76);
    let root = nif.root_index().unwrap();
    let new = nif
        .edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraSkinAttach("SkinBMP", ["Bone", "", "Bone 2"])))
        .unwrap();
    let empty = nif
        .edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraSkinAttach("Empty", Vec::<&str>::new())))
        .unwrap();
    let saved = reload(&mut nif);
    let attaches: Vec<_> = saved.blocks_of::<SkinAttach>().collect();
    assert_eq!(attaches.len(), 2, "{new} {empty}");
    assert_eq!(attaches[0].name(), "SkinBMP");
    assert_eq!(
        attaches[0].bones().collect::<Vec<_>>(),
        ["Bone", "", "Bone 2"]
    );
    assert_eq!(attaches[1].name(), "Empty");
    assert_eq!(attaches[1].bones().len(), 0);
}
