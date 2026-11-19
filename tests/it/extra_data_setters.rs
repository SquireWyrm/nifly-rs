//! Tests for changing existing extra data: the block setters, NiExtraData::set_name and the old
//! extra data chain.

use std::{fs, io::Cursor};

use nifly::{
    extra_data::{
        BoneLod, BoneTranslation, BoneTranslations, BsBehaviorGraphExtraData, BsBoneLodExtraData, BsBound,
        BsClothExtraData, BsCollisionQueryProxyExtraData, BsConnectPointChildren, BsConnectPointParents,
        BsDecalPlacementVectorExtraData, BsDistantObjectExtraData, BsDistantObjectLargeRefExtraData,
        BsEyeCenterExtraData, BsFurnitureMarker, BsInvMarker, BsPositionData, BsWArray, BsxFlags, ConnectPoint,
        DecalPoint, ExtraBehaviorGraph, ExtraBinary, ExtraBoneLods, ExtraBoneTranslations, ExtraBoolean, ExtraBsBound,
        ExtraBsCloth, ExtraBsCollisionQueryProxy, ExtraBsxFlags, ExtraColor, ExtraConnectPointChildren,
        ExtraConnectPointParents, ExtraDecalPlacement, ExtraDistantObject, ExtraDistantObjectLargeRef, ExtraEyeCenter,
        ExtraFloat, ExtraFloats, ExtraFurnitureMarkerNode, ExtraInteger, ExtraIntegers, ExtraInvMarker,
        ExtraPositionData, ExtraSkinAttach, ExtraString, ExtraStrings, ExtraVector, ExtraVertWeights, ExtraWArray,
        FurniturePosition, IntoExtraData, NiBinaryExtraData, NiBooleanExtraData, NiColorExtraData, NiExtraData,
        NiFloatExtraData, NiFloatsExtraData, NiIntegerExtraData, NiIntegersExtraData, NiStringExtraData,
        NiStringsExtraData, NiVectorExtraData, NiVertWeightsExtraData, SkinAttach,
    },
    file::{Index, NifFile, NifVersion, SaveOptions},
    object3d::Vec3,
    objects::{NiAvObject, NiType},
};

fn reload(nif: &mut NifFile) -> NifFile {
    let mut cur = Cursor::new(Vec::new());
    nif.save(&mut cur, Some(SaveOptions { optimize: false, sort_blocks: false })).unwrap();
    NifFile::load(cur.into_inner().as_slice(), None).unwrap()
}

/// Attaches `value` to the root and returns its index.
fn add(nif: &mut NifFile, value: impl IntoExtraData) -> Index {
    let root = nif.root_node().unwrap().index();
    nif.edit::<NiAvObject>(root).unwrap().assign_extra_data(value).unwrap()
}

fn set<T: NiType>(nif: &mut NifFile, index: Index, f: impl FnOnce(std::pin::Pin<&mut T>)) {
    f(nif.block_of_mut::<T>(index).unwrap());
}

fn get<T: NiType>(nif: &NifFile, index: Index) -> &T {
    nif.block_of::<T>(index).unwrap()
}

#[test]
fn values_and_names() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let string = add(&mut nif, ExtraString("S", "a"));
    let integer = add(&mut nif, ExtraInteger("I", 1));
    let float = add(&mut nif, ExtraFloat("F", 1.0));
    let boolean = add(&mut nif, ExtraBoolean("B", false));
    let integers = add(&mut nif, ExtraIntegers("Is", [1]));
    let binary = add(&mut nif, ExtraBinary("Bin", [1]));
    let floats = add(&mut nif, ExtraFloats("Fs", [1.0]));
    let strings = add(&mut nif, ExtraStrings("Ss", ["a"]));
    let color = add(&mut nif, ExtraColor("C", [0.0; 4]));
    let vector = add(&mut nif, ExtraVector("V", [0.0; 4]));
    let bsx = add(&mut nif, ExtraBsxFlags(1));

    set::<NiStringExtraData>(&mut nif, string, |b| b.set_value("changed"));
    set::<NiIntegerExtraData>(&mut nif, integer, |b| b.set_value(42));
    set::<NiFloatExtraData>(&mut nif, float, |b| b.set_value(2.5));
    set::<NiBooleanExtraData>(&mut nif, boolean, |b| b.set_value(true));
    set::<NiIntegersExtraData>(&mut nif, integers, |b| b.set_values(&[3, 4, 5]));
    set::<NiBinaryExtraData>(&mut nif, binary, |b| b.set_data(&[9, 8]));
    set::<NiFloatsExtraData>(&mut nif, floats, |b| b.set_values(&[]));
    set::<NiStringsExtraData>(&mut nif, strings, |b| b.set_values(["x", "", "yz"]));
    set::<NiColorExtraData>(&mut nif, color, |b| b.set_value([0.1, 0.2, 0.3, 0.4]));
    set::<NiVectorExtraData>(&mut nif, vector, |b| b.set_value([1.0, 2.0, 3.0, 4.0]));
    // BSX flags are integers; their block can be renamed too.
    set::<NiIntegerExtraData>(&mut nif, bsx, |b| b.set_value(0x82));
    set::<NiExtraData>(&mut nif, bsx, |b| b.set_name("Renamed"));
    set::<NiExtraData>(&mut nif, string, |b| b.set_name(b"S2"));

    let nif = reload(&mut nif);
    assert_eq!(get::<NiStringExtraData>(&nif, string).value(), "changed");
    assert_eq!(get::<NiExtraData>(&nif, string).name(), "S2");
    assert_eq!(get::<NiIntegerExtraData>(&nif, integer).value(), 42);
    assert_eq!(get::<NiFloatExtraData>(&nif, float).value(), 2.5);
    assert!(get::<NiBooleanExtraData>(&nif, boolean).value());
    assert_eq!(get::<NiIntegersExtraData>(&nif, integers).values(), [3, 4, 5]);
    assert_eq!(get::<NiBinaryExtraData>(&nif, binary).data(), [9, 8]);
    assert!(get::<NiFloatsExtraData>(&nif, floats).values().is_empty());
    assert_eq!(get::<NiStringsExtraData>(&nif, strings).values().collect::<Vec<_>>(), ["x", "", "yz"]);
    assert_eq!(get::<NiColorExtraData>(&nif, color).value(), [0.1, 0.2, 0.3, 0.4]);
    assert_eq!(get::<NiVectorExtraData>(&nif, vector).value(), [1.0, 2.0, 3.0, 4.0]);
    let bsx_block = get::<BsxFlags>(&nif, bsx);
    assert_eq!((bsx_block.name(), bsx_block.value()), ("Renamed".into(), 0x82));
}

#[test]
fn lists_and_records() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let weights = add(&mut nif, ExtraVertWeights("W", [1.0]));
    let eye = add(&mut nif, ExtraEyeCenter("E", [1.0]));
    let position = add(&mut nif, ExtraPositionData("P", [1.0]));
    let w_array = add(&mut nif, ExtraWArray("WA", [1]));
    let decal = add(&mut nif, ExtraDecalPlacement("D", 1.0, Vec::<Vec<DecalPoint>>::new()));
    let bone_lods = add(&mut nif, ExtraBoneLods("BL", Vec::new()));
    let distant = add(&mut nif, ExtraDistantObject("DO", 0));
    let large_ref = add(&mut nif, ExtraDistantObjectLargeRef("LR", false));
    let translations = add(&mut nif, ExtraBoneTranslations("BT", Vec::new()));
    let furniture = add(&mut nif, ExtraFurnitureMarkerNode("FRN", Vec::new()));
    let parents = add(&mut nif, ExtraConnectPointParents("CPA", Vec::new()));
    let children = add(&mut nif, ExtraConnectPointChildren("CPT", false, Vec::<&str>::new()));
    let inv = add(&mut nif, ExtraInvMarker("INV", [0; 3], 1.0));
    let bound = add(&mut nif, ExtraBsBound("BBX", Vec3::ZERO, Vec3::ZERO));
    let graph = add(&mut nif, ExtraBehaviorGraph("BGED", "a.hkx", false));
    let attach = add(&mut nif, ExtraSkinAttach("SkinBMP", Vec::<&str>::new()));

    let point = |x: f32| DecalPoint { position: Vec3::new(x, 0.0, 0.0), normal: Vec3::new(0.0, 0.0, 1.0) };
    let lods = vec![BoneLod { distance: 10, bone: "A".into() }, BoneLod { distance: 20, bone: "B".into() }];
    let bone_translations = vec![BoneTranslation { bone: "Spine".into(), translation: Vec3::new(1.0, 2.0, 3.0) }];
    let positions = vec![FurniturePosition { offset: Vec3::new(1.0, 0.0, 0.0), heading: 1.5, animation_type: 1, entry_points: 3, ..Default::default() }];
    let points = vec![ConnectPoint {
        parent: "Root".into(),
        name: "P-Point".into(),
        rotation: [1.0, 0.0, 0.0, 0.0],
        translation: Vec3::new(4.0, 5.0, 6.0),
        scale: 2.0,
    }];

    set::<NiVertWeightsExtraData>(&mut nif, weights, |b| assert!(b.set_weights(&[0.25, 0.75])));
    set::<BsEyeCenterExtraData>(&mut nif, eye, |b| b.set_values(&[1.0, 2.0, 3.0, 4.0]));
    set::<BsPositionData>(&mut nif, position, |b| b.set_values(&[0.5, 2.0]));
    set::<BsWArray>(&mut nif, w_array, |b| b.set_values(&[7, 8]));
    set::<BsDecalPlacementVectorExtraData>(&mut nif, decal, |b| assert!(b.set_blocks([vec![point(1.0), point(2.0)], vec![point(3.0)]])));
    set::<NiFloatExtraData>(&mut nif, decal, |b| b.set_value(0.5));
    set::<BsBoneLodExtraData>(&mut nif, bone_lods, |b| b.set_bone_lods(&lods));
    set::<BsDistantObjectExtraData>(&mut nif, distant, |b| b.set_flags(0x13));
    set::<BsDistantObjectLargeRefExtraData>(&mut nif, large_ref, |b| b.set_large_ref(true));
    set::<BoneTranslations>(&mut nif, translations, |b| b.set_translations(&bone_translations));
    set::<BsFurnitureMarker>(&mut nif, furniture, |b| b.set_positions(&positions));
    set::<BsConnectPointParents>(&mut nif, parents, |b| b.set_connect_points(&points));
    set::<BsConnectPointChildren>(&mut nif, children, |mut b| {
        b.as_mut().set_skinned(true);
        b.as_mut().set_targets(["P-Point", "P-Other"]);
    });
    set::<BsInvMarker>(&mut nif, inv, |mut b| {
        b.as_mut().set_rotation([100, 200, 300]);
        b.as_mut().set_zoom(0.75);
    });
    set::<BsBound>(&mut nif, bound, |mut b| {
        b.as_mut().set_center(Vec3::new(1.0, 2.0, 3.0));
        b.as_mut().set_half_extents(Vec3::new(4.0, 5.0, 6.0));
    });
    set::<BsBehaviorGraphExtraData>(&mut nif, graph, |mut b| {
        b.as_mut().set_graph_file("Actors\\Graph.hkx");
        b.as_mut().set_controls_base_skeleton(true);
    });
    set::<SkinAttach>(&mut nif, attach, |b| b.set_bones(["Bone1", "Bone2"]));

    let nif = reload(&mut nif);
    assert_eq!(get::<NiVertWeightsExtraData>(&nif, weights).weights(), [0.25, 0.75]);
    assert_eq!(get::<BsEyeCenterExtraData>(&nif, eye).values(), [1.0, 2.0, 3.0, 4.0]);
    assert_eq!(get::<BsPositionData>(&nif, position).values(), [0.5, 2.0]);
    assert_eq!(get::<BsWArray>(&nif, w_array).values(), [7, 8]);
    assert_eq!(get::<BsDecalPlacementVectorExtraData>(&nif, decal).blocks(), [vec![point(1.0), point(2.0)], vec![point(3.0)]]);
    assert_eq!(get::<NiFloatExtraData>(&nif, decal).value(), 0.5);
    assert_eq!(get::<BsBoneLodExtraData>(&nif, bone_lods).bone_lods(), lods);
    assert_eq!(get::<BsDistantObjectExtraData>(&nif, distant).flags(), 0x13);
    assert!(get::<BsDistantObjectLargeRefExtraData>(&nif, large_ref).large_ref());
    assert_eq!(get::<BoneTranslations>(&nif, translations).translations(), bone_translations);
    assert_eq!(get::<BsFurnitureMarker>(&nif, furniture).positions(), positions);
    assert_eq!(get::<BsConnectPointParents>(&nif, parents).connect_points(), points);
    let c = get::<BsConnectPointChildren>(&nif, children);
    assert!(c.skinned());
    assert_eq!(c.targets().collect::<Vec<_>>(), ["P-Point", "P-Other"]);
    let m = get::<BsInvMarker>(&nif, inv);
    assert_eq!((m.rotation(), m.zoom()), ([100, 200, 300], 0.75));
    let b = get::<BsBound>(&nif, bound);
    assert_eq!((b.center(), b.half_extents()), (Vec3::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0)));
    let g = get::<BsBehaviorGraphExtraData>(&nif, graph);
    assert_eq!((g.graph_file(), g.controls_base_skeleton()), ("Actors\\Graph.hkx".into(), true));
    assert_eq!(get::<SkinAttach>(&nif, attach).bones().collect::<Vec<_>>(), ["Bone1", "Bone2"]);
}

#[test]
fn blobs() {
    let mut nif = NifFile::create(NifVersion::FALLOUT_4);
    let cloth = add(&mut nif, ExtraBsCloth([1, 2]));
    let proxy = add(&mut nif, ExtraBsCollisionQueryProxy([3]));
    set::<BsClothExtraData>(&mut nif, cloth, |b| b.set_data(&[5, 6, 7]));
    set::<BsCollisionQueryProxyExtraData>(&mut nif, proxy, |b| b.set_data(&[]));
    let nif = reload(&mut nif);
    assert_eq!(get::<BsClothExtraData>(&nif, cloth).data(), [5, 6, 7]);
    assert!(get::<BsCollisionQueryProxyExtraData>(&nif, proxy).data().is_empty());
}

#[test]
fn rejected_values_change_nothing() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let weights = add(&mut nif, ExtraVertWeights("W", [1.0]));
    let decal = add(&mut nif, ExtraDecalPlacement("D", 1.0, [vec![DecalPoint::default()]]));

    set::<NiVertWeightsExtraData>(&mut nif, weights, |b| assert!(!b.set_weights(&vec![0.0; 0x10000])));
    assert_eq!(get::<NiVertWeightsExtraData>(&nif, weights).weights(), [1.0]);
    set::<BsDecalPlacementVectorExtraData>(&mut nif, decal, |mut b| {
        assert!(!b.as_mut().set_blocks([vec![DecalPoint::default(); 0x10000]]));
        assert!(!b.as_mut().set_blocks(vec![Vec::<DecalPoint>::new(); 0x10000]));
    });
    assert_eq!(get::<BsDecalPlacementVectorExtraData>(&nif, decal).blocks(), [vec![DecalPoint::default()]]);
}

#[test]
fn setting_existing_blocks() {
    // Blocks read from a file change the same way.
    let file = fs::read("nifly-sys/vendor/nifly/tests/input/TestNifFile_SF.nif").unwrap();
    let mut nif = NifFile::load(file.as_slice(), None).unwrap();
    let attach = nif.blocks_of::<SkinAttach>().indices().next().unwrap();
    set::<SkinAttach>(&mut nif, attach, |b| b.set_bones(["Only"]));
    assert_eq!(reload(&mut nif).block_of::<SkinAttach>(attach).unwrap().bones().collect::<Vec<_>>(), ["Only"]);

    // The translation count is kept in step with the list.
    let mut nif = NifFile::create(NifVersion::FALLOUT_76);
    let translations = add(&mut nif, ExtraBoneTranslations("BT", &[]));
    let three = vec![BoneTranslation::default(); 3];
    set::<BoneTranslations>(&mut nif, translations, |b| b.set_translations(&three));
    assert_eq!(reload(&mut nif).block_of::<BoneTranslations>(translations).unwrap().translations(), three);
}

#[test]
fn extra_data_chain() {
    // Morrowind chains extra data; new blocks go at the head, so the chain runs b -> a.
    let mut nif = NifFile::create(NifVersion::MORROWIND);
    let a = add(&mut nif, ExtraString("", "a"));
    let b = add(&mut nif, ExtraString("", "b"));
    let root = nif.root_node().unwrap().index();
    assert_eq!(nif.view::<NiExtraData>(b).unwrap().next_extra_data().map(|n| n.index()), Some(a));

    let mut edit = nif.edit::<NiExtraData>(a).unwrap();
    // Not extra data, itself, missing, or looping back through b.
    for bad in [root, a, 100, usize::MAX, b] {
        assert!(!edit.set_next_extra_data(Some(bad)), "{bad}");
    }
    assert!(edit.next_extra_data().is_none());

    // Unlinking a and linking it again.
    let mut edit = nif.edit::<NiExtraData>(b).unwrap();
    assert!(edit.set_next_extra_data(None));
    assert!(edit.view().next_extra_data().is_none());
    assert!(edit.set_next_extra_data(Some(a)));
    edit.next_extra_data().unwrap().block().set_num_bytes(5);
    assert_eq!(nif.block_of::<NiExtraData>(a).unwrap().num_bytes(), 5);
    let saved = reload(&mut nif);
    assert_eq!(saved.view::<NiExtraData>(b).unwrap().next_extra_data().map(|n| n.index()), Some(a));
    assert_eq!(saved.block_of::<NiExtraData>(a).unwrap().num_bytes(), 5);
}
