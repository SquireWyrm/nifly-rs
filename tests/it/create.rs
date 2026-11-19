//! Tests for NifFile::create and NifFile::version.

use crate::common::{save};
use std::fs;

use nifly::{
    file::{NifFile, NifVersion},
    geometry::NiShape,
    nodes::NiNode,
};

const ALL: [(&str, NifVersion); 8] = [
    ("Morrowind", NifVersion::MORROWIND),
    ("Oblivion", NifVersion::OBLIVION),
    ("Fallout 3", NifVersion::FALLOUT_3),
    ("Skyrim", NifVersion::SKYRIM),
    ("Skyrim SE", NifVersion::SKYRIM_SE),
    ("Fallout 4", NifVersion::FALLOUT_4),
    ("Fallout 76", NifVersion::FALLOUT_76),
    ("Starfield", NifVersion::STARFIELD),
];

#[test]
fn create_has_only_root_node() {
    for (game, version) in ALL {
        let nif = NifFile::create(version);
        assert!(nif.is_valid(), "{game}");
        assert_eq!(nif.version(), version, "{game}");

        assert_eq!(nif.blocks().count(), 1, "{game}");
        let root = nif.root_node().expect(game);
        assert_eq!(root.block_name(), "NiNode", "{game}");
        assert_eq!(root.name(), "Scene Root", "{game}");
        assert_eq!(root.children().count(), 0, "{game}");
    }
    assert_eq!(NifVersion::SKYRIM_SE.file_parts(), [20, 2, 0, 7]);
}

#[test]
fn created_file_round_trips_through_save() {
    for (game, version) in ALL {
        let mut nif = NifFile::create(version);
        let root = nif.root_node().unwrap().index();
        let identity = nif.root_node().unwrap().transform();
        nif.add_node("Child", identity, Some(root)).unwrap();

        let bytes = save(&mut nif);
        let saved = NifFile::load(bytes.as_slice(), None).unwrap_or_else(|e| panic!("{game}: {e}"));
        assert_eq!(saved.version(), version, "{game}");

        let names: Vec<_> = saved.nodes().map(|n| n.name().to_string()).collect();
        assert_eq!(names, ["Scene Root", "Child"], "{game}");
    }
}

#[test]
fn version_of_loaded_assets() {
    let expected = [
        ("Cube", NifVersion::SKYRIM_SE),
        ("Suzanne", NifVersion::SKYRIM_SE),
        ("Icosphere", NifVersion::SKYRIM_SE),
        // Exported for Skyrim LE, which is why it uses NiTriShape and NiSkinInstance.
        ("Bones", NifVersion::SKYRIM),
        ("Cone", NifVersion::SKYRIM_SE),
    ];
    for (name, version) in expected {
        let nif = NifFile::load(fs::read(format!("assets/{name}.nif")).unwrap().as_slice(), None).unwrap();
        assert_eq!(nif.version(), version, "{name}");
    }
}

#[test]
fn create_then_build_a_small_tree() {
    let mut nif = NifFile::create(NifVersion::SKYRIM_SE);
    let identity = nif.root_node().unwrap().transform();
    let arm = nif.add_node("Arm", identity, None).unwrap();
    nif.add_node("Hand", identity, Some(arm)).unwrap();

    let arm_node = nif.block_of::<NiNode>(arm).unwrap();
    let hand: Vec<_> = nif.view_of(arm_node).unwrap().children().map(|c| c.name().to_string()).collect();
    assert_eq!(hand, ["Hand"]);
    assert!(nif.is_all_sse_compatible());
}

#[test]
fn sse_compatibility_per_shape() {
    for name in ["Cube", "Suzanne", "Icosphere", "Bones", "Cone"] {
        let nif = NifFile::load(fs::read(format!("assets/{name}.nif")).unwrap().as_slice(), None).unwrap();
        let shapes: Vec<_> = nif.blocks_of::<NiShape>().collect();
        assert!(!shapes.is_empty(), "{name}");

        // None of the assets use strips, including Bones' skin partition.
        for shape in &shapes {
            assert!(nif.view_of(*shape).unwrap().is_sse_compatible(), "{name}: {}", shape.name());
        }
        // The whole-file check agrees with checking each shape.
        assert_eq!(nif.is_all_sse_compatible(), shapes.iter().all(|s| nif.view_of(*s).unwrap().is_sse_compatible()), "{name}");
    }
}
