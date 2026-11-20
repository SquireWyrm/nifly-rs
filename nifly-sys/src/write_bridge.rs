use std::io::{self, Seek, SeekFrom, Write};

#[cxx::bridge(namespace = "write_bridge")]
pub mod ffi {
    extern "Rust" {
        type RustWriter<'a>;

        // `Result` => C++ sees a rust::Error exception on failure.
        // `&[u8]` => rust::Slice<const uint8_t>.
        fn write_bytes(self: &mut RustWriter<'_>, buf: &[u8]) -> Result<()>;
        fn flush_writer(self: &mut RustWriter<'_>) -> Result<()>;

        /// `whence`: 0 = start, 1 = current, 2 = end. Returns the new position from the start.
        fn seek_to(self: &mut RustWriter<'_>, offset: i64, whence: u8) -> Result<u64>;
    }

    unsafe extern "C++" {
        include!("write_bridge.h");

        /// Test helper: writes `text`, seeks back to `patch_at` and overwrites with `patch`.
        fn write_and_patch<'a>(
            writer: &mut RustWriter<'a>,
            text: &str,
            patch_at: u64,
            patch: &str,
        ) -> Result<()>;
    }
}

// Lets other bridges (nif_file.rs) refer to RustWriter as an extern type.
unsafe impl<'a> cxx::ExternType for RustWriter<'a> {
    type Id = cxx::type_id!("write_bridge::RustWriter");
    type Kind = cxx::kind::Opaque;
}

trait WriteSeek: Write + Seek {}
impl<T: Write + Seek + ?Sized> WriteSeek for T {}

/// Type-erased, borrowed writer. cxx can't expose generics or `dyn Trait` directly, so any
/// `impl Write + Seek` is borrowed inside this concrete opaque type.
///
/// The first I/O error is kept, so callers can return the original [`io::Error`] instead of the
/// C++ exception message it was turned into.
pub struct RustWriter<'a> {
    inner: &'a mut dyn WriteSeek,
    error: Option<io::Error>,
}

impl<'a> RustWriter<'a> {
    pub fn new(writer: &'a mut (impl Write + Seek)) -> Self {
        Self {
            inner: writer,
            error: None,
        }
    }

    /// Takes the first I/O error that was reported to C++, if any.
    pub fn take_error(&mut self) -> Option<io::Error> {
        self.error.take()
    }

    fn write_bytes(&mut self, buf: &[u8]) -> io::Result<()> {
        let result = self.inner.write_all(buf);
        self.record_original(result)
    }

    fn flush_writer(&mut self) -> io::Result<()> {
        let result = self.inner.flush();
        self.record_original(result)
    }

    fn seek_to(&mut self, offset: i64, whence: u8) -> io::Result<u64> {
        let pos = match whence {
            0 => match u64::try_from(offset) {
                Ok(o) => SeekFrom::Start(o),
                Err(_) => {
                    let e = io::Error::new(io::ErrorKind::InvalidInput, "negative seek from start");
                    return self.record_original(Err(e));
                }
            },
            1 => SeekFrom::Current(offset),
            2 => SeekFrom::End(offset),
            _ => {
                let e = io::Error::new(io::ErrorKind::InvalidInput, "invalid seek direction");
                return self.record_original(Err(e));
            }
        };
        let result = self.inner.seek(pos);
        self.record_original(result)
    }

    /// Stores the original error and hands C++ a new error with the same message.
    fn record_original<T>(&mut self, result: io::Result<T>) -> io::Result<T> {
        match result {
            Ok(v) => Ok(v),
            Err(e) => {
                let for_cpp = io::Error::new(e.kind(), e.to_string());
                if self.error.is_none() {
                    self.error = Some(e);
                }
                Err(for_cpp)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn writes_and_seeks() {
        let mut cur = Cursor::new(Vec::new());
        let mut w = RustWriter::new(&mut cur);
        ffi::write_and_patch(&mut w, "hello world", 6, "WORLD").unwrap();
        assert!(w.take_error().is_none());
        assert_eq!(cur.into_inner(), b"hello WORLD");
    }

    #[test]
    fn large_writes_bypass_buffer() {
        let text = "x".repeat(100_000);
        let mut cur = Cursor::new(Vec::new());
        let mut w = RustWriter::new(&mut cur);
        ffi::write_and_patch(&mut w, &text, 50_000, "y").unwrap();

        let out = cur.into_inner();
        assert_eq!(out.len(), 100_000);
        assert_eq!(out[50_000], b'y');
        assert_eq!(out.iter().filter(|&&b| b == b'x').count(), 99_999);
    }

    #[test]
    fn errors_propagate_with_original_kind() {
        struct Failing;
        impl Write for Failing {
            fn write(&mut self, _: &[u8]) -> io::Result<usize> {
                Err(io::Error::new(io::ErrorKind::PermissionDenied, "boom"))
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

        let mut f = Failing;
        let mut w = RustWriter::new(&mut f);
        let err = ffi::write_and_patch(&mut w, "hello", 0, "j").unwrap_err();
        assert!(err.what().contains("boom"));
        assert_eq!(
            w.take_error().unwrap().kind(),
            io::ErrorKind::PermissionDenied
        );
    }
}
