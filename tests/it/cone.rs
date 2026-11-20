//! Tests against assets/Cone.nif: a file that isn't optimized or sorted, to check that the save
//! options reach nifly.
//!
//! Block layout:
//!   0 NiNode "Scene Root" -> [3]
//!   1 NiNode ""                     unreferenced
//!   2 BSLightingShaderProperty      unreferenced (the shape doesn't use it)
//!   3 BSTriShape "Cone"

use std::io::Cursor;

use nifly::file::{NifFile, SaveOptions};

const ORIGINAL: [&str; 4] = ["NiNode", "NiNode", "BSLightingShaderProperty", "BSTriShape"];

fn cone_bytes() -> Vec<u8> {
    std::fs::read("assets/Cone.nif").unwrap()
}

fn block_names(nif: &NifFile) -> Vec<String> {
    nif.blocks().map(|b| b.block_name().to_owned()).collect()
}

/// Saves with the given options. Returns the saved bytes and the in-memory file after saving.
fn save(optimize: bool, sort_blocks: bool) -> (Vec<u8>, NifFile) {
    let mut nif = NifFile::load(cone_bytes().as_slice(), None).unwrap();
    let mut cur = Cursor::new(Vec::new());
    nif.save(
        &mut cur,
        Some(SaveOptions {
            optimize,
            sort_blocks,
        }),
    )
    .unwrap();
    (cur.into_inner(), nif)
}

fn reload(bytes: &[u8]) -> NifFile {
    NifFile::load(bytes, None).unwrap()
}

#[test]
fn layout_has_unreferenced_blocks() {
    let nif = NifFile::load(cone_bytes().as_slice(), None).unwrap();
    assert_eq!(block_names(&nif), ORIGINAL);

    let reachable: Vec<_> = nif
        .tree()
        .map(|b| nif.blocks().position(|x| std::ptr::eq(x, b)).unwrap())
        .collect();
    assert_eq!(reachable, [0, 3]);

    let shape = nif.block(3).unwrap();
    assert_eq!(
        nif.child_blocks(shape).count(),
        0,
        "shape doesn't reference the shader"
    );
}

#[test]
fn no_options_keeps_file_unchanged() {
    let (bytes, nif) = save(false, false);
    assert_eq!(bytes, cone_bytes());
    assert_eq!(block_names(&nif), ORIGINAL);
}

#[test]
fn optimize_removes_unreferenced_blocks() {
    let (bytes, nif) = save(true, false);
    assert!(bytes.len() < cone_bytes().len());

    let saved = reload(&bytes);
    assert_eq!(block_names(&saved), ["NiNode", "BSTriShape"]);
    assert_eq!(saved.nodes().count(), 1);
    assert_eq!(saved.root_node().unwrap().name(), "Scene Root");
    assert_eq!(
        saved.root_node().unwrap().children().next().unwrap().name(),
        "Cone"
    );

    // Saving modifies the file in memory too.
    assert_eq!(block_names(&nif), ["NiNode", "BSTriShape"]);
}

#[test]
fn sort_moves_unreferenced_blocks_last() {
    let (bytes, nif) = save(false, true);
    assert_eq!(bytes.len(), cone_bytes().len());

    // Referenced blocks first, in tree order, then the unreferenced ones.
    let order = ["NiNode", "BSTriShape", "NiNode", "BSLightingShaderProperty"];
    assert_eq!(block_names(&reload(&bytes)), order);
    assert_eq!(block_names(&nif), order);

    // The root's child ref was updated to the shape's new index.
    let saved = reload(&bytes);
    let root = saved.root_node().unwrap();
    assert_eq!(root.children().next().unwrap().name(), "Cone");
}

#[test]
fn default_options_optimize_and_sort() {
    let mut nif = NifFile::load(cone_bytes().as_slice(), None).unwrap();
    let mut cur = Cursor::new(Vec::new());
    nif.save(&mut cur, None).unwrap();
    let default_bytes = cur.into_inner();

    let (both, _) = save(true, true);
    assert_eq!(default_bytes, both);
    assert_eq!(
        block_names(&reload(&default_bytes)),
        ["NiNode", "BSTriShape"]
    );
}

#[test]
fn optimized_file_is_stable() {
    let (first, _) = save(true, true);
    let mut nif = reload(&first);
    let mut cur = Cursor::new(Vec::new());
    nif.save(&mut cur, None).unwrap();
    assert_eq!(cur.into_inner(), first);
}
