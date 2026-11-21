//! Tests for NifFile::load with borrowed readers and error reporting, and NifFile::open.

use std::{
    fs::{self, File},
    io::{self, Read, Seek, SeekFrom},
};

use nifly::{
    error::{Error, LoadError},
    file::{LoadOptions, NifFile},
};

#[test]
fn load_from_borrowed_file_and_reuse_it() {
    let mut file = File::open("assets/Cube.nif").unwrap();
    let nif = NifFile::load(&mut file, None).unwrap();
    assert_eq!(nif.blocks().count(), 4);

    // Still ours: rewind and read it again.
    file.seek(SeekFrom::Start(0)).unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();
    assert_eq!(
        bytes.len() as u64,
        fs::metadata("assets/Cube.nif").unwrap().len()
    );
}

#[test]
fn load_from_byte_slice() {
    let bytes = fs::read("assets/Suzanne.nif").unwrap();
    let nif = NifFile::load(bytes.as_slice(), None).unwrap();
    assert_eq!(nif.root_node().unwrap().name(), "Suzanne");
}

#[test]
fn reader_error_returns_original_io_error() {
    struct Failing;
    impl Read for Failing {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::new(
                io::ErrorKind::ConnectionReset,
                "connection reset",
            ))
        }
    }

    match NifFile::load(Failing, None) {
        Err(Error::Io(e)) => {
            assert_eq!(e.kind(), io::ErrorKind::ConnectionReset);
            assert_eq!(e.to_string(), "connection reset");
        }
        Err(other) => panic!("expected Error::Io, got {other:?}"),
        Ok(_) => panic!("expected Error::Io, got Ok"),
    }
}

#[test]
fn reader_error_mid_file_returns_io_error() {
    // Fails after the header, so the error happens partway through nifly's parsing.
    struct FailAfter<'a> {
        data: &'a [u8],
    }
    impl Read for FailAfter<'_> {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            if self.data.is_empty() {
                return Err(io::Error::new(io::ErrorKind::TimedOut, "timed out"));
            }
            self.data.read(buf)
        }
    }

    let bytes = fs::read("assets/Suzanne.nif").unwrap();
    match NifFile::load(
        FailAfter {
            data: &bytes[..200],
        },
        None,
    ) {
        Err(Error::Io(e)) => assert_eq!(e.kind(), io::ErrorKind::TimedOut),
        Err(other) => panic!("expected Error::Io, got {other:?}"),
        Ok(_) => panic!("expected Error::Io, got Ok"),
    }
}

#[test]
fn invalid_data_returns_load_error() {
    match NifFile::load(&b"definitely not a nif file"[..], None) {
        Err(Error::LoadFile(LoadError::InvalidHeader)) => {}
        Err(other) => panic!("expected InvalidHeader, got {other:?}"),
        Ok(_) => panic!("expected InvalidHeader, got Ok"),
    }
}

#[test]
fn terrain_option() {
    let bytes = fs::read("assets/Cube.nif").unwrap();

    let default = NifFile::load(bytes.as_slice(), None).unwrap();
    assert!(!default.is_terrain());

    let explicit_default = NifFile::load(bytes.as_slice(), Some(LoadOptions::default())).unwrap();
    assert!(!explicit_default.is_terrain());

    let terrain = NifFile::load(bytes.as_slice(), Some(LoadOptions { is_terrain: true })).unwrap();
    assert!(terrain.is_terrain());
}

#[test]
fn open_reads_a_path() {
    let opened = NifFile::open("assets/Suzanne.nif", None).unwrap();
    let loaded = NifFile::load(fs::read("assets/Suzanne.nif").unwrap().as_slice(), None).unwrap();
    assert_eq!(opened.blocks().count(), loaded.blocks().count());
    assert_eq!(opened.root_node().unwrap().name(), "Suzanne");

    let terrain = NifFile::open("assets/Suzanne.nif", Some(LoadOptions { is_terrain: true }));
    assert!(terrain.is_ok());
}

#[test]
fn open_missing_file_is_an_io_error() {
    match NifFile::open("assets/Missing.nif", None) {
        Err(Error::Io(e)) => assert_eq!(e.kind(), io::ErrorKind::NotFound),
        other => panic!("expected an i/o error, got {:?}", other.err()),
    }
}
