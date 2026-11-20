use std::io;

#[cxx::bridge(namespace = "read_bridge")]
pub mod ffi {
    extern "Rust" {
        type RustReader<'a>;

        // `Result` => C++ sees a rust::Error exception on failure.
        // `&mut [u8]` => rust::Slice<uint8_t>.
        fn read_some(self: &mut RustReader<'_>, buf: &mut [u8]) -> Result<usize>;
    }

    unsafe extern "C++" {
        include!("read_bridge.h");

        fn count_lines(reader: &mut RustReader<'_>) -> Result<u64>;
    }
}

// Lets other bridges (nif_file.rs) refer to RustReader as an extern type.
unsafe impl<'a> cxx::ExternType for RustReader<'a> {
    type Id = cxx::type_id!("read_bridge::RustReader");
    type Kind = cxx::kind::Opaque;
}

/// Type-erased, borrowed reader. cxx can't expose generics or `dyn Trait` directly, so any
/// `impl Read` is borrowed inside this concrete opaque type.
///
/// The first I/O error is kept, so callers can return the original [`io::Error`] instead of the
/// C++ exception message it was turned into.
pub struct RustReader<'a> {
    inner: &'a mut dyn io::Read,
    error: Option<io::Error>,
}

impl<'a> RustReader<'a> {
    pub fn new(reader: &'a mut impl io::Read) -> Self {
        Self {
            inner: reader,
            error: None,
        }
    }

    /// Takes the first I/O error that was reported to C++, if any.
    pub fn take_error(&mut self) -> Option<io::Error> {
        self.error.take()
    }

    /// One `read` call. Returns 0 only at EOF (or for an empty buffer).
    /// `Interrupted` is retried, matching what `Read::read_exact` does.
    fn read_some(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        loop {
            match self.inner.read(buf) {
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Ok(n) => return Ok(n),
                Err(e) => {
                    // Keep the original; C++ gets a new error with the same message.
                    let for_cpp = io::Error::new(e.kind(), e.to_string());
                    if self.error.is_none() {
                        self.error = Some(e);
                    }
                    return Err(for_cpp);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    /// Runs C++ `std::istream` code over a Rust reader. The ffi declaration can't be test-only,
    /// because build.rs generates the C++ side without `cfg(test)`.
    fn count_lines(mut reader: impl io::Read) -> Result<u64, cxx::Exception> {
        ffi::count_lines(&mut RustReader::new(&mut reader))
    }

    #[test]
    fn counts_lines() {
        assert_eq!(count_lines(Cursor::new("a\nb\nc")).unwrap(), 3);
    }

    #[test]
    fn large_input_crosses_buffer_boundaries() {
        let data = "line\n".repeat(100_000);
        assert_eq!(count_lines(Cursor::new(data)).unwrap(), 100_000);
    }

    #[test]
    fn reads_from_borrowed_slice() {
        let data = b"a\nb\n".to_vec();
        let mut slice: &[u8] = &data;
        assert_eq!(
            ffi::count_lines(&mut RustReader::new(&mut slice)).unwrap(),
            2
        );
    }

    #[test]
    fn errors_propagate_with_original_kind() {
        struct Failing;
        impl io::Read for Failing {
            fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
                Err(io::Error::new(io::ErrorKind::ConnectionReset, "boom"))
            }
        }

        let mut f = Failing;
        let mut r = RustReader::new(&mut f);
        let err = ffi::count_lines(&mut r).unwrap_err();
        assert!(err.what().contains("boom"));
        assert_eq!(
            r.take_error().unwrap().kind(),
            io::ErrorKind::ConnectionReset
        );
    }
}
