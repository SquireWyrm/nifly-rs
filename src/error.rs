use std::io;

use int_enum::IntEnum;
use thiserror::Error;

pub type NifResult<T, E = Error> = Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("could not load file: {0}")]
    LoadFile(#[from] LoadError),
    #[error("could not save file: {0}")]
    SaveFile(#[from] SaveError),
    #[error("i/o error: {0}")]
    Io(#[from] io::Error),
    #[error("encountered c++ exception: {0}")]
    Exception(#[from] cxx::Exception)
}

#[derive(Error, Debug, IntEnum)]
#[repr(i32)]
pub enum LoadError {
    #[error("invalid/missing header")]
    InvalidHeader = 1,
    #[error("unsupported file version")]
    UnsupportedVersion = 2,
    #[error("unknown blocks without block sizes is unsupported")]
    UnknownUnsizedBlocks = 3,
}

#[derive(Error, Debug, IntEnum)]
#[repr(i32)]
pub enum SaveError {
    #[error("output stream was not writable")]
    StreamNotWritable = 1,
}
