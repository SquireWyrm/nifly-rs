//! Errors from loading and saving files and meshes.

use std::io;

use int_enum::IntEnum;
use thiserror::Error;

/// A `Result` with [`Error`](enum@Error) as the default error type.
pub type NifResult<T, E = Error> = Result<T, E>;

/// An error from loading or saving.
#[derive(Error, Debug)]
pub enum Error {
    /// The data isn't a file that can be loaded.
    #[error("could not load file: {0}")]
    LoadFile(#[from] LoadError),
    /// The file couldn't be written.
    #[error("could not save file: {0}")]
    SaveFile(#[from] SaveError),
    /// The reader or writer failed, with its original error.
    #[error("i/o error: {0}")]
    Io(#[from] io::Error),
    /// Reading or writing failed on malformed or out-of-range data, such as a list longer than
    /// the data holds or a mesh too large to write.
    #[error("encountered c++ exception: {0}")]
    Exception(#[from] cxx::Exception)
}

/// Why data couldn't be loaded.
#[derive(Error, Debug, IntEnum)]
#[repr(i32)]
pub enum LoadError {
    /// The header is missing or isn't a NIF header.
    #[error("invalid/missing header")]
    InvalidHeader = 1,
    /// The version isn't one of the games' ([`NifVersion`](crate::file::NifVersion)'s
    /// constants) or 10.0.1.0 with user version 0. For a mesh, its format version is above 2.
    #[error("unsupported file version")]
    UnsupportedVersion = 2,
    /// The file has a block of an unknown type, and its version (before 20.2.0.5) doesn't store
    /// block sizes, so the block can't be skipped.
    #[error("unknown blocks without block sizes is unsupported")]
    UnknownUnsizedBlocks = 3,
}

/// Why a file couldn't be saved.
#[derive(Error, Debug, IntEnum)]
#[repr(i32)]
pub enum SaveError {
    /// The writer was unusable before anything was written.
    #[error("output stream was not writable")]
    StreamNotWritable = 1,
}
