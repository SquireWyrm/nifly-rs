//! Tests for NiDefaultAVObjectPalette's scene and objects.

use crate::common::upstream;
use std::io::Cursor;

use nifly::{
    bstr::BString,
    file::{NifFile, SaveOptions},
    nodes::{NiAvObjectPalette, NiDefaultAvObjectPalette},
    objects::NiObject,
};

/// The palette in `TestNifFile_Animated_LE`.
const PALETTE: usize = 47;

fn reload(nif: &mut NifFile, options: Option<SaveOptions>) -> NifFile {
    let mut cur = Cursor::new(Vec::new());
    nif.save(&mut cur, options).unwrap();
    NifFile::load(cur.into_inner().as_slice(), None).unwrap()
}

/// Keeps block indices the same.
const KEEP: Option<SaveOptions> = Some(SaveOptions { optimize: false, sort_blocks: false });

/// Each entry's name, with its object's index and name.
fn entries(nif: &NifFile) -> Vec<(BString, Option<(usize, BString)>)> {
    let palette = nif.blocks_of::<NiDefaultAvObjectPalette>().views().next().unwrap();
    palette.objects().map(|o| (o.name.to_owned(), o.object.map(|v| (v.index(), v.name().to_owned())))).collect()
}

#[test]
fn reads_palette() {
    let nif = upstream("TestNifFile_Animated_LE");
    assert!(nif.block(PALETTE).unwrap().downcast::<NiAvObjectPalette>().is_some());
    let palette = nif.view::<NiDefaultAvObjectPalette>(PALETTE).unwrap();
    assert_eq!(palette.scene().map(|s| (s.index(), s.name().to_owned())), Some((0, "Scene Root".into())));

    let objects = entries(&nif);
    assert_eq!(objects.len(), 11);
    assert_eq!(objects[0], ("PCloud06-Emitter".into(), Some((74, "PCloud06-Emitter".into()))));
    // Names needn't match the objects'.
    assert_eq!(objects[4], ("IceGlow".into(), Some((66, "Glow".into()))));

    assert_eq!(palette.object("GlowCenter:0").map(|o| o.index()), Some(62));
    assert_eq!(palette.object(b"IceGlow").map(|o| o.index()), Some(66));
    assert!(palette.object("Glow").is_none());
}

#[test]
fn survives_save() {
    for name in ["TestNifFile_Animated_LE", "TestNifFile_FO76"] {
        let mut nif = upstream(name);
        let named = |e: Vec<(BString, Option<(usize, BString)>)>| {
            e.into_iter().map(|(n, o)| (n, o.map(|(_, o)| o))).collect::<Vec<_>>()
        };
        let before = named(entries(&nif));
        // Sorting renumbers the blocks; the refs follow them.
        let saved = reload(&mut nif, None);
        assert_eq!(named(entries(&saved)), before, "{name}");
        let palette = saved.blocks_of::<NiDefaultAvObjectPalette>().views().next().unwrap();
        assert_eq!(palette.scene().map(|s| s.index()), saved.root_node().map(|r| r.index()), "{name}");
    }
}

#[test]
fn set_scene() {
    let mut nif = upstream("TestNifFile_Animated_LE");
    let count = nif.blocks().count();
    let mut edit = nif.edit::<NiDefaultAvObjectPalette>(PALETTE).unwrap();
    // The palette isn't an object; the others don't exist.
    for bad in [PALETTE, count, usize::MAX] {
        assert!(!edit.set_scene(Some(bad)), "{bad}");
    }
    assert_eq!(edit.view().scene().map(|s| s.index()), Some(0));

    assert!(edit.set_scene(Some(48)));
    edit.scene().unwrap().block().upcast_mut().set_name("Scene");
    assert_eq!(nif.view::<NiDefaultAvObjectPalette>(PALETTE).unwrap().scene().unwrap().name(), "Scene");
    let saved = reload(&mut nif, KEEP);
    assert_eq!(saved.view::<NiDefaultAvObjectPalette>(PALETTE).unwrap().scene().map(|s| s.index()), Some(48));

    let mut edit = nif.edit::<NiDefaultAvObjectPalette>(PALETTE).unwrap();
    assert!(edit.set_scene(None));
    assert!(edit.scene().is_none());
}

#[test]
fn set_objects() {
    let mut nif = upstream("TestNifFile_Animated_LE");
    let before = entries(&nif);
    let count = nif.blocks().count();
    let mut edit = nif.edit::<NiDefaultAvObjectPalette>(PALETTE).unwrap();
    // Block 1 isn't an object; the others don't exist.
    for bad in [1, count, usize::MAX] {
        assert!(!edit.set_objects([("A", Some(48)), ("B", Some(bad))]), "{bad}");
    }
    assert_eq!(entries(&nif), before);

    let mut edit = nif.edit::<NiDefaultAvObjectPalette>(PALETTE).unwrap();
    assert!(edit.set_objects([(&b"Low02"[..], Some(48)), (b"", None), (b"Shape", Some(49))]));
    let expected: Vec<(BString, _)> =
        vec![("Low02".into(), Some((48, "Low02".into()))), ("".into(), None), ("Shape".into(), Some((49, "Low02:0".into())))];
    assert_eq!(entries(&nif), expected);
    assert_eq!(entries(&reload(&mut nif, KEEP)), expected);

    let mut edit = nif.edit::<NiDefaultAvObjectPalette>(PALETTE).unwrap();
    assert!(edit.set_objects(Vec::<(String, Option<usize>)>::new()));
    assert_eq!(edit.view().objects().len(), 0);
    assert!(edit.view().object("").is_none());
}

#[test]
fn only_palettes() {
    let mut nif = upstream("TestNifFile_Animated_LE");
    assert!(nif.edit::<NiDefaultAvObjectPalette>(0).is_none());
    assert!(nif.block(0).unwrap().downcast::<NiAvObjectPalette>().is_none());
    let palette = nif.view::<NiDefaultAvObjectPalette>(PALETTE).unwrap().upcast::<NiAvObjectPalette, _>();
    let _: &NiObject = &palette;
}
