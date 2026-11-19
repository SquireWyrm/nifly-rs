#include "write_bridge.h"

#include <algorithm>
#include <ios>
#include <streambuf>
#include <vector>

#include "nifly-sys/src/write_bridge.rs.h"

namespace write_bridge {

class RustOStream::Buf: public std::streambuf {
  public:
    Buf(RustWriter& writer, std::size_t buffer_size) :
        writer_(writer),
        buf_(std::max<std::size_t>(buffer_size, 1)) {
        setp(buf_.data(), buf_.data() + buf_.size());
    }

    // Best effort: errors can't be reported from a destructor.
    ~Buf() override {
        try {
            flush_buffer();
        } catch (...) {}
    }

  protected:
    int_type overflow(int_type c) override {
        flush_buffer();
        if (!traits_type::eq_int_type(c, traits_type::eof())) {
            *pptr() = traits_type::to_char_type(c);
            pbump(1);
        }
        return traits_type::not_eof(c);
    }

    // Bulk writes: fill the buffer, and write large chunks straight from the
    // caller's memory, skipping the intermediate copy.
    std::streamsize xsputn(const char* s, std::streamsize count) override {
        std::streamsize free = epptr() - pptr();
        if (count <= free) {
            std::copy(s, s + count, pptr());
            pbump(static_cast<int>(count));
            return count;
        }

        flush_buffer();
        if (count >= static_cast<std::streamsize>(buf_.size())) {
            write(s, static_cast<std::size_t>(count));
        } else {
            std::copy(s, s + count, pptr());
            pbump(static_cast<int>(count));
        }
        return count;
    }

    int sync() override {
        flush_buffer();
        writer_.flush_writer();
        return 0;
    }

    pos_type seekoff(
        off_type off,
        std::ios_base::seekdir dir,
        std::ios_base::openmode which
    ) override {
        if (!(which & std::ios_base::out))
            return pos_type(off_type(-1));

        flush_buffer();
        std::uint8_t whence = dir == std::ios_base::beg ? 0
            : dir == std::ios_base::cur                 ? 1
                                                        : 2;
        return pos_type(
            static_cast<off_type>(
                writer_.seek_to(static_cast<std::int64_t>(off), whence)
            )
        );
    }

    pos_type seekpos(pos_type pos, std::ios_base::openmode which) override {
        return seekoff(off_type(pos), std::ios_base::beg, which);
    }

  private:
    void flush_buffer() {
        std::size_t n = static_cast<std::size_t>(pptr() - pbase());
        // Reset first, so a failed write doesn't leave stale bytes to retry.
        setp(buf_.data(), buf_.data() + buf_.size());
        if (n > 0)
            write(buf_.data(), n);
    }

    void write(const char* data, std::size_t n) {
        writer_.write_bytes(
            rust::Slice<const std::uint8_t>(
                reinterpret_cast<const std::uint8_t*>(data),
                n
            )
        );
    }

    RustWriter& writer_;
    std::vector<char> buf_;
};

RustOStream::RustOStream(RustWriter& writer, std::size_t buffer_size) :
    std::ostream(nullptr),
    buf_(std::make_unique<Buf>(writer, buffer_size)) {
    rdbuf(buf_.get()); // also clears the badbit set by ostream(nullptr)
}

RustOStream::~RustOStream() = default;

void write_and_patch(
    RustWriter& writer,
    rust::Str text,
    std::uint64_t patch_at,
    rust::Str patch
) {
    RustOStream out(writer);
    out.exceptions(std::ios::badbit); // propagate Rust write errors
    out.write(text.data(), static_cast<std::streamsize>(text.size()));
    std::streampos end = out.tellp();
    out.seekp(static_cast<std::streamoff>(patch_at));
    out.write(patch.data(), static_cast<std::streamsize>(patch.size()));
    out.seekp(end);
    out.flush();
}

} // namespace write_bridge
