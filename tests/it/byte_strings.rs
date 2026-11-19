//! Tests for names and paths returned as bstr types.

use crate::common::{load};

use nifly::{
    bstr::{BStr, BString, ByteSlice},
    file::NifVersion,
    geometry::NiShape,
    nodes::NiNode,
};

#[test]
fn non_utf8_names_are_kept() {
    let mut nif = load("Cube");
    let shape = nif.shapes().indices().next().unwrap();
    nif.block_of_mut::<nifly::objects::NiObjectNet>(shape).unwrap().set_name(b"Caf\xe9");

    let name = nif.shapes().next().unwrap().name();
    assert_eq!(name, b"Caf\xe9");
    assert!(name.to_str().is_err());
    assert_eq!(format!("{name}"), "Caf\u{FFFD}");
    assert_eq!(nif.shape_names().collect::<Vec<_>>(), [b"Caf\xe9"]);
}

#[test]
fn bone_list_keeps_raw_bytes() {
    // Bone names come from the bone nodes, so a non-UTF-8 node name must come back unchanged.
    let mut nif = load("Bones");
    let bone = nif.blocks_of::<NiNode>().indexed().find(|(_, n)| n.name() == "Bone1").unwrap().0;
    nif.block_of_mut::<nifly::objects::NiObjectNet>(bone).unwrap().set_name(b"Kn\xe9e");

    let shape = nif.shapes().next().unwrap();
    let bones = nif.view_of(shape).unwrap().bone_list();
    assert_eq!(bones, [&b"Kn\xe9e"[..], b"Bone2", b"Bone3"]);
    // The raw name finds the node again.
    assert!(nif.find_block_by_name::<NiNode>(&bones[0]).is_some());
}

#[test]
fn parameters_accept_every_byte_string_type() {
    let nif = load("Cube");
    let owned = String::from("Cube");
    let bytes: Vec<u8> = b"Cube".to_vec();
    let bstring = BString::from("Cube");
    assert!(nif.find_block_by_name::<NiShape>("Cube").is_some());
    assert!(nif.find_block_by_name::<NiShape>(&owned).is_some());
    assert!(nif.find_block_by_name::<NiShape>(owned.clone()).is_some());
    assert!(nif.find_block_by_name::<NiShape>(b"Cube").is_some());
    assert!(nif.find_block_by_name::<NiShape>(&bytes).is_some());
    assert!(nif.find_block_by_name::<NiShape>(BStr::new("Cube")).is_some());
    assert!(nif.find_block_by_name::<NiShape>(&bstring).is_some());

    // A name returned by one method can be passed straight to another.
    let mut nif = load("Cube");
    let name = nif.shapes().next().unwrap().name().to_owned();
    let shape = nif.shapes().indices().next().unwrap();
    nif.block_of_mut::<nifly::objects::NiObjectNet>(shape).unwrap().set_name(&name);
}

#[test]
fn optimize_report_lists_are_byte_strings() {
    let mut nif = load("Bones");
    let report = nif.optimize_for(NifVersion::SKYRIM_SE, None).unwrap();
    let lists: [&Vec<BString>; 5] = [
        &report.vertex_colors_removed,
        &report.normals_removed,
        &report.partitions_triangulated,
        &report.tangents_added,
        &report.parallax_removed,
    ];
    assert!(lists.iter().all(|l| l.is_empty()));
}
