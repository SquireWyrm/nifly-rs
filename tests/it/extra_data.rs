//! Tests for reading extra data and for NifFile::assign_extra_data.

use crate::common::load;
use std::io::Cursor;

use nifly::{
    extra_data::{
        BsClothExtraData, BsCollisionQueryProxyExtraData, BsExtraData, BsxBitflags, BsxFlags,
        ExtraBoolean, ExtraBsCloth, ExtraBsCollisionQueryProxy, ExtraBsxFlags, ExtraFloat,
        ExtraInteger, ExtraString, NiBooleanExtraData, NiExtraData, NiFloatExtraData,
        NiIntegerExtraData, NiStringExtraData,
    },
    file::{NifFile, NifVersion},
    geometry::NiShape,
    objects::NiObject,
};

fn root_index(nif: &NifFile) -> usize {
    nif.root_node().unwrap().index()
}

fn root_extra_data(nif: &NifFile) -> Vec<&NiObject> {
    nif.root_node().unwrap().extra_data().collect()
}

fn save_and_reload(nif: &mut NifFile) -> NifFile {
    let mut cur = Cursor::new(Vec::new());
    nif.save(&mut cur, None).unwrap();
    NifFile::load(cur.into_inner().as_slice(), None).unwrap()
}

#[test]
fn read_existing_bsx_flags() {
    let nif = load("Suzanne");
    let extra = root_extra_data(&nif);
    assert_eq!(extra.len(), 1);

    let bsx = extra[0].downcast::<BsxFlags>().unwrap();
    assert_eq!(bsx.name(), "BSX");
    assert_eq!(bsx.value(), 130);

    // BSXFlags is also an NiIntegerExtraData and an NiExtraData, but not a BSExtraData.
    assert_eq!(
        extra[0].downcast::<NiIntegerExtraData>().unwrap().value(),
        130
    );
    assert_eq!(extra[0].downcast::<NiExtraData>().unwrap().name(), b"BSX");
    assert!(extra[0].downcast::<NiStringExtraData>().is_none());
    assert!(extra[0].downcast::<BsExtraData>().is_none());
}

#[test]
fn assign_each_named_type() {
    let mut nif = load("Cube");
    let root = root_index(&nif);
    let before = nif.blocks().count();

    let s = nif
        .edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraString("Prn", "SHIELD")))
        .unwrap();
    let owned = nif
        .edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| {
            e.assign_extra_data(ExtraString(String::from("Owned"), String::from("owned")))
        })
        .unwrap();
    let i = nif
        .edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraInteger("Count", 3)))
        .unwrap();
    let f = nif
        .edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraFloat("Scale", 1.5)))
        .unwrap();
    let b = nif
        .edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraBoolean("Flag", true)))
        .unwrap();
    assert_eq!(
        [s, owned, i, f, b],
        [before, before + 1, before + 2, before + 3, before + 4]
    );

    let get = |index| nif.block(index).unwrap();
    let string = get(s).downcast::<NiStringExtraData>().unwrap();
    assert_eq!(
        (string.name(), string.value()),
        ("Prn".into(), "SHIELD".into())
    );
    let owned = get(owned).downcast::<NiStringExtraData>().unwrap();
    assert_eq!(
        (owned.name(), owned.value()),
        ("Owned".into(), "owned".into())
    );
    assert_eq!(get(i).downcast::<NiIntegerExtraData>().unwrap().value(), 3);
    assert_eq!(get(f).downcast::<NiFloatExtraData>().unwrap().value(), 1.5);
    assert!(get(b).downcast::<NiBooleanExtraData>().unwrap().value());

    // A plain integer isn't BSXFlags.
    assert!(get(i).downcast::<BsxFlags>().is_none());

    // All attached to the root, in order.
    let attached: Vec<_> = root_extra_data(&nif)
        .into_iter()
        .map(|e| nif.block_index(e).unwrap())
        .collect();
    let owned_index = nif.block_index(owned).unwrap();
    assert_eq!(attached, [s, owned_index, i, f, b]);
}

#[test]
fn assign_bsx_flags() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let root = root_index(&nif);
    let index = nif
        .edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| {
            e.assign_extra_data(ExtraBsxFlags(BsxBitflags::HAVOK | BsxBitflags::ARTICULATED))
        })
        .unwrap();

    let bsx = nif.block_of::<BsxFlags>(index).unwrap();
    assert_eq!(bsx.block_name(), "BSXFlags");
    assert_eq!(bsx.name(), "BSX");
    assert_eq!(bsx.value(), 0x82);
}

#[test]
fn assign_cloth_and_collision_query_proxy() {
    let mut nif = NifFile::create(NifVersion::FALLOUT_4);
    let root = root_index(&nif);
    let cloth_bytes: Vec<u8> = (0..=255).collect();

    let cloth = nif
        .edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraBsCloth(&cloth_bytes)))
        .unwrap();
    let proxy = nif
        .edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraBsCollisionQueryProxy(b"proxy")))
        .unwrap();

    let cloth_block = nif.block(cloth).unwrap();
    assert_eq!(cloth_block.block_name(), "BSClothExtraData");
    assert_eq!(
        cloth_block.downcast::<BsClothExtraData>().unwrap().data(),
        cloth_bytes
    );
    // Unnamed: a BSExtraData, not an NiExtraData.
    assert!(cloth_block.downcast::<BsExtraData>().is_some());
    assert!(cloth_block.downcast::<NiExtraData>().is_none());

    let proxy_block = nif.block(proxy).unwrap();
    assert_eq!(
        proxy_block
            .downcast::<BsCollisionQueryProxyExtraData>()
            .unwrap()
            .data(),
        b"proxy"
    );

    // Both are in the root's extra data list.
    let attached: Vec<_> = root_extra_data(&nif)
        .into_iter()
        .map(|e| nif.block_index(e).unwrap())
        .collect();
    assert_eq!(attached, [cloth, proxy]);
}

#[test]
fn cloth_survives_save() {
    let mut nif = NifFile::create(NifVersion::FALLOUT_4);
    let root = root_index(&nif);
    let cloth_bytes = vec![0xAB; 10_000];
    nif.edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraBsCloth(cloth_bytes.clone())))
        .unwrap();
    nif.edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraString("Name", "after cloth")))
        .unwrap();

    let saved = save_and_reload(&mut nif);
    let extra = root_extra_data(&saved);
    assert_eq!(extra.len(), 2);
    assert_eq!(
        extra[0].downcast::<BsClothExtraData>().unwrap().data(),
        cloth_bytes
    );
    assert_eq!(
        extra[1].downcast::<NiStringExtraData>().unwrap().value(),
        "after cloth"
    );
}

#[test]
fn empty_cloth() {
    let mut nif = NifFile::create(NifVersion::FALLOUT_4);
    let root = root_index(&nif);
    let index = nif
        .edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraBsCloth([])))
        .unwrap();
    assert!(
        nif.block_of::<BsClothExtraData>(index)
            .unwrap()
            .data()
            .is_empty()
    );
}

#[test]
fn unnamed_blocks_rejected_in_linked_list_versions() {
    // Morrowind (4.0.0.2) stores extra data as a linked list through NiExtraData.
    let mut nif = NifFile::create(NifVersion::MORROWIND);
    let root = root_index(&nif);
    assert_eq!(
        nif.edit::<nifly::objects::NiAvObject>(root)
            .and_then(|mut e| e.assign_extra_data(ExtraBsCloth(b"x"))),
        None
    );
    assert_eq!(
        nif.edit::<nifly::objects::NiAvObject>(root)
            .and_then(|mut e| e.assign_extra_data(ExtraBsCollisionQueryProxy(b"x"))),
        None
    );
    assert_eq!(nif.blocks().count(), 1);

    // Named extra data still works there: nifly links it into the list.
    assert!(
        nif.edit::<nifly::objects::NiAvObject>(root)
            .and_then(|mut e| e.assign_extra_data(ExtraInteger("Count", 1)))
            .is_some()
    );
}

#[test]
fn assign_to_shape() {
    let mut nif = load("Cube");
    let shape = nif
        .block_index(nif.blocks_of::<NiShape>().next().unwrap())
        .unwrap();
    let index = nif
        .edit::<nifly::objects::NiAvObject>(shape)
        .and_then(|mut e| e.assign_extra_data(ExtraString("Note", "on the shape")))
        .unwrap();

    let shape = nif.blocks_of::<NiShape>().next().unwrap();
    let attached: Vec<_> = nif
        .view_of(shape)
        .unwrap()
        .extra_data()
        .map(|e| nif.block_index(e).unwrap())
        .collect();
    assert_eq!(attached, [index]);
}

#[test]
fn invalid_target_changes_nothing() {
    let mut nif = load("Suzanne");
    let count = nif.blocks().count();

    // Block 1 is the existing BSXFlags: extra data, not an NiAVObject.
    assert_eq!(nif.block(1).unwrap().block_name(), "BSXFlags");
    assert_eq!(
        nif.edit::<nifly::objects::NiAvObject>(1)
            .and_then(|mut e| e.assign_extra_data(ExtraInteger("Bad", 1))),
        None
    );
    assert_eq!(
        nif.edit::<nifly::objects::NiAvObject>(1)
            .and_then(|mut e| e.assign_extra_data(ExtraBsxFlags(BsxBitflags::ANIMATED))),
        None
    );
    assert_eq!(
        nif.edit::<nifly::objects::NiAvObject>(1)
            .and_then(|mut e| e.assign_extra_data(ExtraBsCloth(b"x"))),
        None
    );
    assert_eq!(
        nif.edit::<nifly::objects::NiAvObject>(count)
            .and_then(|mut e| e.assign_extra_data(ExtraBoolean("Bad", true))),
        None
    );
    assert_eq!(
        nif.edit::<nifly::objects::NiAvObject>(u32::MAX as usize)
            .and_then(|mut e| e.assign_extra_data(ExtraBoolean("Bad", true))),
        None
    );
    assert_eq!(
        nif.edit::<nifly::objects::NiAvObject>(usize::MAX)
            .and_then(|mut e| e.assign_extra_data(ExtraBoolean("Bad", true))),
        None
    );

    assert_eq!(nif.blocks().count(), count);
}

#[test]
fn non_utf8_name_and_value() {
    let mut nif = load("Cube");
    let root = root_index(&nif);
    let index = nif
        .edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraString(b"Caf\xe9", b"na\xefve")))
        .unwrap();
    let extra = nif.block_of::<NiStringExtraData>(index).unwrap();
    assert_eq!(extra.name(), b"Caf\xe9");
    assert_eq!(extra.value(), b"na\xefve");
}

#[test]
fn assigned_extra_data_survives_save() {
    let mut nif = load("Cube");
    let root = root_index(&nif);
    nif.edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraBsxFlags(BsxBitflags::HAVOK)))
        .unwrap();
    nif.edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraString("Prn", "SHIELD")))
        .unwrap();
    nif.edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraFloat("Scale", 0.25)))
        .unwrap();

    let saved = save_and_reload(&mut nif);
    let extra = root_extra_data(&saved);
    let names: Vec<_> = extra.iter().map(|e| e.block_name()).collect();
    assert_eq!(names, ["BSXFlags", "NiStringExtraData", "NiFloatExtraData"]);

    assert_eq!(extra[0].downcast::<BsxFlags>().unwrap().value(), 0x2);
    assert_eq!(
        extra[1].downcast::<NiStringExtraData>().unwrap().value(),
        "SHIELD"
    );
    assert_eq!(
        extra[2].downcast::<NiFloatExtraData>().unwrap().value(),
        0.25
    );
}

#[test]
fn blocks_of_finds_extra_data() {
    let mut nif = load("Suzanne");
    let root = root_index(&nif);
    nif.edit::<nifly::objects::NiAvObject>(root)
        .and_then(|mut e| e.assign_extra_data(ExtraInteger("Count", 7)))
        .unwrap();

    // The existing BSXFlags plus the new integer: both are NiIntegerExtraData.
    assert_eq!(nif.blocks_of::<NiIntegerExtraData>().count(), 2);
    assert_eq!(nif.blocks_of::<BsxFlags>().count(), 1);
    assert_eq!(nif.blocks_of::<NiExtraData>().count(), 2);
    assert_eq!(nif.blocks_of::<BsExtraData>().count(), 0);
}
