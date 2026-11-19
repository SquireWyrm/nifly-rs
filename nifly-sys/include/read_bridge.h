#pragma once
#include <cstddef>
#include <cstdint>
#include <istream>
#include <memory>

#include "nifly-sys/src/read_bridge.rs.h"
#include "rust/cxx.h"

namespace read_bridge {

// Opaque Rust type, defined in nifly-sys/src/read_bridge.rs (the cxx bridge
// generates its complete definition in "nifly-sys/src/read_bridge.rs.h").
struct RustReader;

// A std::istream that pulls its bytes from a borrowed Rust `impl Read`.
//
// Usable anywhere a std::istream& is expected:
//
//     // reader: RustReader&
//     read_bridge::RustIStream in(reader);
//     std::string line;
//     while (std::getline(in, line)) { ... }
//
// Errors returned by the Rust reader surface as a rust::Error exception
// inside the streambuf. As with any streambuf, std::istream catches it and
// sets badbit; call `in.exceptions(std::ios::badbit)` to have the original
// rust::Error rethrown instead.
//
// The RustReader is borrowed and must outlive the stream. Seeking isn't
// supported.
class RustIStream: public std::istream {
  public:
    explicit RustIStream(RustReader& reader, std::size_t buffer_size = 8192);
    ~RustIStream() override;

    RustIStream(const RustIStream&) = delete;
    RustIStream& operator=(const RustIStream&) = delete;

  private:
    class Buf;
    std::unique_ptr<Buf> buf_;
};

// Test helper: runs ordinary istream-based C++ code over a Rust reader. Used by
// the tests in read_bridge.rs to exercise chunked reads and error propagation.
std::uint64_t count_lines(RustReader& reader);

} // namespace read_bridge
