#pragma once
#include <cstddef>
#include <cstdint>
#include <memory>
#include <ostream>

#include "rust/cxx.h"

namespace write_bridge {

// Opaque Rust type, defined in nifly-sys/src/write_bridge.rs (the cxx bridge
// generates its complete definition in "nifly-sys/src/write_bridge.rs.h").
struct RustWriter;

// A std::ostream that pushes its bytes into a borrowed Rust `impl Write + Seek`.
//
// Usable anywhere a std::ostream& is expected, including code that seeks back
// to patch earlier output (seekp/tellp go to Rust's Seek after flushing):
//
//     // writer: RustWriter&
//     write_bridge::RustOStream out(writer);
//     out.exceptions(std::ios::badbit);
//     out << ...;
//     out.flush();
//
// Errors returned by the Rust writer surface as a rust::Error exception
// inside the streambuf. As with any streambuf, std::ostream catches it and
// sets badbit; call `out.exceptions(std::ios::badbit)` to have the original
// rust::Error rethrown instead.
//
// Call flush() before the stream is destroyed. The destructor also flushes,
// but it can't report errors, so any failure there is silently dropped.
//
// The RustWriter is borrowed and must outlive the stream.
class RustOStream: public std::ostream {
  public:
    explicit RustOStream(RustWriter& writer, std::size_t buffer_size = 8192);
    ~RustOStream() override;

    RustOStream(const RustOStream&) = delete;
    RustOStream& operator=(const RustOStream&) = delete;

  private:
    class Buf;
    std::unique_ptr<Buf> buf_;
};

// Test helper: writes `text`, seeks back to `patch_at` and overwrites with
// `patch`. Used by the tests in write_bridge.rs to exercise buffering, seeking
// and error propagation.
void write_and_patch(
    RustWriter& writer,
    rust::Str text,
    std::uint64_t patch_at,
    rust::Str patch
);

} // namespace write_bridge
