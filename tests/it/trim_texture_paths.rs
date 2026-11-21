//! Tests for NifFile::trim_texture_paths.

use nifly::file::{Index, NifFile};

use crate::common::open;

fn first(nif: &NifFile) -> Index {
    nif.shapes().indices().next().unwrap()
}

fn slot(nif: &NifFile, index: usize) -> String {
    nif.shapes()
        .views()
        .next()
        .unwrap()
        .texture_slot(index)
        .unwrap()
        .1
        .to_string()
}

#[test]
fn normalizes_skyrim_paths() {
    let mut nif = open("assets/Cube.nif");
    let shape = first(&nif);
    let cases = [
        (
            "  C:/Games/Skyrim/Data/textures//Foo/bar.dds \n",
            r"textures\Foo\bar.dds",
        ),
        ("foo/bar_n.dds", r"textures\foo\bar_n.dds"),
        (r"\textures\x.dds", r"textures\x.dds"),
        (r"Textures\Kept.dds", r"Textures\Kept.dds"),
        ("", ""),
    ];
    for (i, (input, _)) in cases.iter().enumerate() {
        nif.edit::<nifly::geometry::NiShape>(shape)
            .unwrap()
            .set_texture_slot(i, input);
    }
    assert_eq!(slot(&nif, 0), cases[0].0);

    nif.trim_texture_paths();
    for (i, (input, expected)) in cases.iter().enumerate() {
        assert_eq!(slot(&nif, i), *expected, "{input:?}");
    }
    // Already normalized paths don't change.
    nif.trim_texture_paths();
    assert_eq!(slot(&nif, 0), cases[0].1);
}

#[test]
fn no_prefix_for_morrowind() {
    let mut nif = open("nifly-sys/vendor/nifly/tests/input/TestNifFile_Skinned_MW.nif");
    let shape = first(&nif);
    nif.edit::<nifly::geometry::NiShape>(shape)
        .unwrap()
        .set_texture_slot(0, r"C:\Morrowind\Data Files\Textures\tx_a.dds ");
    nif.trim_texture_paths();
    assert_eq!(slot(&nif, 0), "tx_a.dds");
    nif.edit::<nifly::geometry::NiShape>(shape)
        .unwrap()
        .set_texture_slot(0, "sub/tx_b.dds");
    nif.trim_texture_paths();
    assert_eq!(slot(&nif, 0), r"sub\tx_b.dds");
}
