//! Round-trip tests for NifFile::save.

use std::{
    fs::File,
    io::{self, Cursor, Seek, SeekFrom, Write},
};

use nifly::{
    error::Error,
    file::{NifFile, SaveOptions},
    geometry::NiShape,
};

const ASSETS: [&str; 4] = ["Cube", "Suzanne", "Icosphere", "Bones"];

fn load(name: &str) -> NifFile {
    NifFile::load(File::open(format!("assets/{name}.nif")).unwrap(), None).unwrap()
}

fn save_to_vec(nif: &mut NifFile) -> Vec<u8> {
    let mut cur = Cursor::new(Vec::new());
    nif.save(&mut cur, None).unwrap();
    cur.into_inner()
}

fn reload(bytes: Vec<u8>) -> NifFile {
    NifFile::load(Cursor::new(bytes), None).unwrap()
}

/// Everything the bindings can read, as comparable values.
fn summary(nif: &NifFile) -> Vec<String> {
    let mut out = Vec::new();
    out.extend(nif.blocks().map(|b| b.block_name().to_owned()));
    for node in nif.nodes() {
        out.push(format!("{} {:?} {:#x}", node.name(), node.transform(), node.flags()));
        out.push(format!("global {:?}", nif.node_transform_to_global(node.name())));
    }
    for shape in nif.blocks_of::<NiShape>() {
        out.push(format!("{} {:?}", shape.name(), shape.transform()));
        let bones = nif.view_of(shape).unwrap().bone_list();
        out.push(format!("bones {bones:?} {:?}", nif.view_of(shape).unwrap().bone_ids()));
        for b in 0..bones.len() {
            out.push(format!("weights {b} {:?}", nif.view_of(shape).unwrap().bone_weights(b)));
        }
    }
    out
}

#[test]
fn round_trip_preserves_everything() {
    for name in ASSETS {
        let mut nif = load(name);
        let before = summary(&nif);

        let saved = reload(save_to_vec(&mut nif));
        assert!(saved.is_valid(), "{name}");
        assert!(!saved.has_unknown(), "{name}");
        assert_eq!(summary(&saved), before, "{name}");
    }
}

#[test]
fn second_save_is_byte_identical() {
    // The first save may optimize and reorder blocks; after that, saving is stable.
    for name in ASSETS {
        let first = save_to_vec(&mut load(name));
        let second = save_to_vec(&mut reload(first.clone()));
        assert_eq!(first, second, "{name}");
    }
}

#[test]
fn saving_again_keeps_oblivion_tangents() {
    // Writing an Oblivion file used to clear its tangents in memory, so the next save
    // dropped them and their extra data block.
    let path = "nifly-sys/vendor/nifly/tests/input/TestNifFile_Skinned_OB.nif";
    let mut nif = NifFile::load(File::open(path).unwrap(), None).unwrap();
    let shape = nif.shapes().views().next().unwrap().index();
    let tangents = nif.view::<NiShape>(shape).unwrap().tangents();
    assert!(tangents.is_some());
    let first = save_to_vec(&mut nif);
    assert_eq!(nif.view::<NiShape>(shape).unwrap().tangents(), tangents);
    assert_eq!(save_to_vec(&mut nif), first);
}

#[test]
fn save_writes_after_existing_position() {
    // nifly seeks to absolute stream positions it recorded with tellp, so output that doesn't
    // start at position 0 must still come out right.
    let mut nif = load("Bones");
    let expected = save_to_vec(&mut nif);

    let mut cur = Cursor::new(b"PREFIX".to_vec());
    cur.seek(SeekFrom::End(0)).unwrap();
    nif.save(&mut cur, None).unwrap();

    let bytes = cur.into_inner();
    assert_eq!(&bytes[..6], b"PREFIX");
    assert_eq!(&bytes[6..], expected);
}

#[test]
fn save_to_file() {
    let path = std::env::temp_dir().join(format!("nifly-save-test-{}.nif", std::process::id()));
    let mut nif = load("Suzanne");
    let before = summary(&nif);

    let mut file = File::create(&path).unwrap();
    nif.save(&mut file, None).unwrap();
    // The writer is borrowed, so it's still usable afterwards.
    file.flush().unwrap();
    drop(file);

    let saved = NifFile::load(File::open(&path).unwrap(), None).unwrap();
    std::fs::remove_file(&path).unwrap();
    assert_eq!(summary(&saved), before);
}

#[test]
fn writer_error_returns_original_io_error() {
    struct Failing;
    impl Write for Failing {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::StorageFull, "disk full"))
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    impl Seek for Failing {
        fn seek(&mut self, _: SeekFrom) -> io::Result<u64> {
            Ok(0)
        }
    }

    let mut nif = load("Cube");
    match nif.save(Failing, None) {
        Err(Error::Io(e)) => {
            assert_eq!(e.kind(), io::ErrorKind::StorageFull);
            assert_eq!(e.to_string(), "disk full");
        }
        other => panic!("expected Error::Io, got {other:?}"),
    }
}

#[test]
fn seek_error_returns_original_io_error() {
    let mut nif = load("Cube");

    struct NoSeek(Vec<u8>);
    impl Write for NoSeek {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.write(buf)
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    impl Seek for NoSeek {
        fn seek(&mut self, _: SeekFrom) -> io::Result<u64> {
            Err(io::Error::new(io::ErrorKind::Unsupported, "cannot seek"))
        }
    }

    match nif.save(NoSeek(Vec::new()), None) {
        Err(Error::Io(e)) => assert_eq!(e.kind(), io::ErrorKind::Unsupported),
        other => panic!("expected Error::Io, got {other:?}"),
    }
}

#[test]
fn save_reproduces_original_bytes() {
    // The assets were written by nifly (through PyNifly), so they're already optimized and
    // sorted, and saving them reproduces the original file exactly.
    for name in ASSETS {
        let original = std::fs::read(format!("assets/{name}.nif")).unwrap();
        assert_eq!(save_to_vec(&mut load(name)), original, "{name}");
    }
}

#[test]
fn every_option_combination_round_trips() {
    // These assets are already optimized and sorted, so the options can't change the output;
    // tests/cone.rs covers a file where they do.
    // This checks every combination is accepted and produces a valid, identical file.
    for name in ASSETS {
        let original = std::fs::read(format!("assets/{name}.nif")).unwrap();
        for optimize in [false, true] {
            for sort_blocks in [false, true] {
                let mut nif = load(name);
                let mut cur = Cursor::new(Vec::new());
                nif.save(&mut cur, Some(SaveOptions { optimize, sort_blocks })).unwrap();
                assert_eq!(cur.into_inner(), original, "{name} optimize={optimize} sort={sort_blocks}");
            }
        }
    }
}

#[test]
fn save_options_default_matches_nifly() {
    let d = SaveOptions::default();
    assert!(d.optimize);
    assert!(d.sort_blocks);
}
