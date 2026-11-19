#include "read_bridge.h"

#include <algorithm>
#include <cstring>
#include <ios>
#include <streambuf>
#include <string>
#include <vector>

namespace read_bridge {

class RustIStream::Buf: public std::streambuf {
  public:
    Buf(RustReader& reader, std::size_t buffer_size) :
        reader_(reader),
        buf_(std::max<std::size_t>(buffer_size, 1)) {
        // Empty get area: the first read triggers underflow().
        setg(buf_.data(), buf_.data(), buf_.data());
    }

  protected:
    int_type underflow() override {
        if (gptr() < egptr()) {
            return traits_type::to_int_type(*gptr());
        }
        std::size_t n = reader_.read_some(
            rust::Slice<std::uint8_t>(
                reinterpret_cast<std::uint8_t*>(buf_.data()),
                buf_.size()
            )
        );
        if (n == 0) {
            return traits_type::eof();
        }
        setg(buf_.data(), buf_.data(), buf_.data() + n);
        return traits_type::to_int_type(*gptr());
    }

    // Bulk reads: drain the buffer, then read large chunks straight into the
    // caller's memory, skipping the intermediate copy.
    std::streamsize xsgetn(char* s, std::streamsize count) override {
        std::streamsize total = 0;
        while (total < count) {
            std::streamsize avail = egptr() - gptr();
            if (avail > 0) {
                std::streamsize n = std::min(avail, count - total);
                std::memcpy(s + total, gptr(), static_cast<std::size_t>(n));
                gbump(static_cast<int>(n));
                total += n;
                continue;
            }
            std::streamsize remaining = count - total;
            if (remaining >= static_cast<std::streamsize>(buf_.size())) {
                std::size_t n = reader_.read_some(
                    rust::Slice<std::uint8_t>(
                        reinterpret_cast<std::uint8_t*>(s + total),
                        static_cast<std::size_t>(remaining)
                    )
                );
                if (n == 0)
                    break;
                total += static_cast<std::streamsize>(n);
            } else {
                if (traits_type::eq_int_type(underflow(), traits_type::eof()))
                    break;
            }
        }
        return total;
    }

  private:
    RustReader& reader_;
    std::vector<char> buf_;
};

RustIStream::RustIStream(RustReader& reader, std::size_t buffer_size) :
    std::istream(nullptr),
    buf_(std::make_unique<Buf>(reader, buffer_size)) {
    rdbuf(buf_.get()); // also clears the badbit set by istream(nullptr)
}

RustIStream::~RustIStream() = default;

std::uint64_t count_lines(RustReader& reader) {
    RustIStream in(reader);
    in.exceptions(std::ios::badbit); // propagate Rust read errors
    std::uint64_t n = 0;
    std::string line;
    while (std::getline(in, line)) {
        ++n;
    }
    return n;
}

} // namespace read_bridge
