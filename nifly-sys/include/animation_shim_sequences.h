// Part of animation_shim.h: sequences, controller managers and animation
// notes.
#pragma once

#include "animation_shim.h"

namespace nifly_rs {
// Files before 10.1.0.104 store a sequence's accumulation root name and text
// keys ref after its name, which NiSequence doesn't read. Sequence (for a
// plain NiSequence, which has no such fields) and ControllerSequence read and
// write them there in those versions and like their base from 10.1.0.104 on.
// load_nif loads both types as these, the add shims create them, and clones
// stay replacements.

// The fields of a sequence before 10.1.0.104: name, accumulation root name,
// text keys and controlled blocks.
template<class T>
void sync_old_sequence(T& seq, nifly::NiStreamReversible& s) {
    seq.name.Sync(s);
    seq.accumRootName.Sync(s);
    seq.textKeyRef.Sync(s);
    uint32_t size = seq.controlledBlocks.SyncSize(s);
    seq.controlledBlocks.SyncData(s, size);
}

class Sequence final: public nifly::NiSequence {
  public:
    nifly::NiStringRef accumRootName;
    nifly::NiBlockRef<nifly::NiTextKeyExtraData> textKeyRef;

    void Get(nifly::NiIStream& stream) override {
        if (stream.GetVersion().File() >= nifly::NiFileVersion::V10_1_0_104) {
            nifly::NiSequence::Get(stream);
            return;
        }
        nifly::NiObject::Get(stream);
        nifly::NiStreamReversible s(
            &stream,
            nullptr,
            nifly::NiStreamReversible::Mode::Reading
        );
        sync_old_sequence(*this, s);
    }

    void Put(nifly::NiOStream& stream) override {
        if (stream.GetVersion().File() >= nifly::NiFileVersion::V10_1_0_104) {
            nifly::NiSequence::Put(stream);
            return;
        }
        nifly::NiObject::Put(stream);
        nifly::NiStreamReversible s(
            nullptr,
            &stream,
            nifly::NiStreamReversible::Mode::Writing
        );
        sync_old_sequence(*this, s);
    }

    void GetStringRefs(std::vector<nifly::NiStringRef*>& refs) override {
        nifly::NiSequence::GetStringRefs(refs);
        refs.emplace_back(&accumRootName);
    }

    void GetChildRefs(std::set<nifly::NiRef*>& refs) override {
        nifly::NiSequence::GetChildRefs(refs);
        refs.insert(&textKeyRef);
    }

    void GetChildIndices(std::vector<uint32_t>& indices) override {
        nifly::NiSequence::GetChildIndices(indices);
        indices.push_back(textKeyRef.index);
    }

  private:
    Sequence* Clone_impl() const override {
        return new Sequence(*this);
    }
};

class ControllerSequence final: public nifly::NiControllerSequence {
  public:
    void Get(nifly::NiIStream& stream) override {
        if (stream.GetVersion().File() >= nifly::NiFileVersion::V10_1_0_104) {
            nifly::NiControllerSequence::Get(stream);
            return;
        }
        nifly::NiObject::Get(stream);
        nifly::NiStreamReversible s(
            &stream,
            nullptr,
            nifly::NiStreamReversible::Mode::Reading
        );
        sync_old_sequence(*this, s);
        nifly::NiControllerSequence::Sync(s);
    }

    void Put(nifly::NiOStream& stream) override {
        if (stream.GetVersion().File() >= nifly::NiFileVersion::V10_1_0_104) {
            nifly::NiControllerSequence::Put(stream);
            return;
        }
        nifly::NiObject::Put(stream);
        nifly::NiStreamReversible s(
            nullptr,
            &stream,
            nifly::NiStreamReversible::Mode::Writing
        );
        sync_old_sequence(*this, s);
        nifly::NiControllerSequence::Sync(s);
    }

  private:
    ControllerSequence* Clone_impl() const override {
        return new ControllerSequence(*this);
    }
};

// BSAnimNote reads and writes the gain and state of grab IK notes too, but
// files store them only for look IK notes. AnimNote syncs them by type.
class AnimNote final: public nifly::BSAnimNote {
  public:
    void Get(nifly::NiIStream& stream) override {
        nifly::NiObject::Get(stream);
        nifly::NiStreamReversible s(
            &stream,
            nullptr,
            nifly::NiStreamReversible::Mode::Reading
        );
        sync(s);
    }

    void Put(nifly::NiOStream& stream) override {
        nifly::NiObject::Put(stream);
        nifly::NiStreamReversible s(
            nullptr,
            &stream,
            nifly::NiStreamReversible::Mode::Writing
        );
        sync(s);
    }

  private:
    void sync(nifly::NiStreamReversible& s) {
        s.Sync(type);
        s.Sync(time);
        if (type == ANT_GRABIK)
            s.Sync(arm);
        if (type == ANT_LOOKIK) {
            s.Sync(gain);
            s.Sync(state);
        }
    }

    AnimNote* Clone_impl() const override {
        return new AnimNote(*this);
    }
};

// The accumulation root name of a sequence: a controller sequence's own, or
// the old one of a plain sequence. Null for another block.
inline const nifly::NiStringRef* accum_root_ref(const nifly::NiSequence& obj) {
    if (auto c = dynamic_cast<const nifly::NiControllerSequence*>(&obj))
        return &c->accumRootName;
    if (auto s = dynamic_cast<const Sequence*>(&obj))
        return &s->accumRootName;
    return nullptr;
}

// The text keys ref of a sequence, like accum_root_ref.
inline nifly::NiBlockRef<nifly::NiTextKeyExtraData>*
sequence_text_keys_ref(nifly::NiObject& obj) {
    if (auto c = dynamic_cast<nifly::NiControllerSequence*>(&obj))
        return &c->textKeyRef;
    if (auto s = dynamic_cast<Sequence*>(&obj))
        return &s->textKeyRef;
    return nullptr;
}

inline rust::Slice<const uint8_t> str_slice(const std::string& s) {
    return {reinterpret_cast<const uint8_t*>(s.data()), s.size()};
}

inline std::string str_from(rust::Slice<const uint8_t> s) {
    return std::string(reinterpret_cast<const char*>(s.data()), s.size());
}

inline std::string str_from(const rust::Vec<uint8_t>& s) {
    return std::string(reinterpret_cast<const char*>(s.data()), s.size());
}

inline rust::Slice<const uint8_t> sequence_name(const nifly::NiSequence& obj) {
    return str_slice(obj.name.get());
}

inline void
set_sequence_name(nifly::NiSequence& obj, rust::Slice<const uint8_t> name) {
    obj.name.get() = str_from(name);
}

inline rust::Slice<const uint8_t>
sequence_accum_root_name(const nifly::NiSequence& obj) {
    auto ref = accum_root_ref(obj);
    return ref ? str_slice(ref->get()) : rust::Slice<const uint8_t> {};
}

inline void set_sequence_accum_root_name(
    nifly::NiSequence& obj,
    rust::Slice<const uint8_t> name
) {
    if (auto ref = accum_root_ref(obj))
        const_cast<nifly::NiStringRef*>(ref)->get() = str_from(name);
}

NIFLY_RS_ANIMATION_FIELD(
    NiSequence,
    uint32_t,
    arrayGrowBy,
    sequence_array_grow_by
)
NIFLY_RS_ANIMATION_FIELD(NiControllerSequence, float, weight, sequence_weight)
NIFLY_RS_ANIMATION_FIELD(
    NiControllerSequence,
    uint32_t,
    cycleType,
    sequence_cycle_type
)
NIFLY_RS_ANIMATION_FIELD(
    NiControllerSequence,
    float,
    frequency,
    sequence_frequency
)
NIFLY_RS_ANIMATION_FIELD(NiControllerSequence, float, phase, sequence_phase)
NIFLY_RS_ANIMATION_FIELD(
    NiControllerSequence,
    float,
    startTime,
    sequence_start_time
)
NIFLY_RS_ANIMATION_FIELD(
    NiControllerSequence,
    float,
    stopTime,
    sequence_stop_time
)
NIFLY_RS_ANIMATION_FIELD(
    NiControllerSequence,
    bool,
    playBackwards,
    sequence_play_backwards
)
NIFLY_RS_ANIMATION_FIELD(
    NiControllerManager,
    bool,
    cumulative,
    manager_cumulative
)
NIFLY_RS_ANIMATION_FIELD(BSAnimNote, uint32_t, type, anim_note_type)
NIFLY_RS_ANIMATION_FIELD(BSAnimNote, float, time, anim_note_time)
NIFLY_RS_ANIMATION_FIELD(BSAnimNote, uint32_t, arm, anim_note_arm)
NIFLY_RS_ANIMATION_FIELD(BSAnimNote, float, gain, anim_note_gain)
NIFLY_RS_ANIMATION_FIELD(BSAnimNote, uint32_t, state, anim_note_state)

// Controlled blocks. Their strings are stored by version: before 10.1.0.104
// only the target name, then strings up to 10.1.0.113, offsets into a string
// palette from 10.2.0.0 to 20.1.0.0, and strings again (string table refs)
// from 20.1.0.1.

inline bool uses_string_palette(const nifly::NiVersion& v) {
    return v.File() >= nifly::NiFileVersion::V10_2_0_0
        && v.File() < nifly::NiFileVersion::V20_1_0_1;
}

// A palette's bytes as saved: its string, plus the final zero byte if any.
inline std::string palette_bytes(const nifly::NiStringPalette& p) {
    std::string out = p.palette.get();
    if (p.palette.GetNullOutput())
        out.push_back('\0');
    return out;
}

// The string at `offset` in `bytes`, up to the next zero byte. Empty if
// `offset` is past the end.
inline std::string palette_string(const std::string& bytes, uint32_t offset) {
    if (offset >= bytes.size())
        return {};
    size_t end = bytes.find('\0', offset);
    return bytes.substr(offset, end == std::string::npos ? end : end - offset);
}

// The five ID strings of a controlled block, in order, and their offsets.
inline std::array<nifly::NiStringRef*, 5>
link_strings(nifly::ControllerLink& link) {
    return {
        &link.nodeName,
        &link.propType,
        &link.ctrlType,
        &link.ctrlID,
        &link.interpID
    };
}

inline std::array<uint32_t*, 5> link_offsets(nifly::ControllerLink& link) {
    return {
        &link.nodeNameOffset,
        &link.propertyTypeOffset,
        &link.controllerTypeOffset,
        &link.controllerIDOffset,
        &link.interpIDOffset
    };
}

inline void set_ffi_string(rust::Vec<uint8_t>& out, const std::string& s) {
    out.clear();
    for (char c : s)
        out.push_back(static_cast<uint8_t>(c));
}

inline rust::Vec<uint8_t>& ffi_string(ControlledBlockFfi& f, size_t i) {
    switch (i) {
        case 0:
            return f.node_name;
        case 1:
            return f.property_type;
        case 2:
            return f.controller_type;
        case 3:
            return f.controller_id;
        default:
            return f.interpolator_id;
    }
}

inline const rust::Vec<uint8_t>&
ffi_string(const ControlledBlockFfi& f, size_t i) {
    return ffi_string(const_cast<ControlledBlockFfi&>(f), i);
}

// The controlled blocks of the sequence at `owner`, with the strings the
// file's version stores. Empty if it isn't a sequence.
inline rust::Vec<ControlledBlockFfi>
controlled_blocks(const nifly::NifFile& file, uint32_t owner) {
    using namespace nifly;
    auto& hdr = file.GetHeader();
    rust::Vec<ControlledBlockFfi> out;
    auto seq = hdr.GetBlock<NiSequence>(owner);
    if (!seq)
        return out;
    const auto& v = hdr.GetVersion();
    for (auto& link : seq->controlledBlocks) {
        ControlledBlockFfi f;
        f.interpolator = link.interpolatorRef.index;
        f.controller = link.controllerRef.index;
        f.blend_interpolator = link.blendInterpolatorRef.index;
        f.blend_index = link.blendIndex;
        f.priority = link.priority;
        auto strings = link_strings(link);
        auto offsets = link_offsets(link);
        if (uses_string_palette(v)) {
            auto palette = hdr.GetBlock<NiStringPalette>(link.stringPaletteRef);
            std::string bytes = palette ? palette_bytes(*palette) : "";
            for (size_t i = 0; i < 5; i++)
                set_ffi_string(
                    ffi_string(f, i),
                    palette_string(bytes, *offsets[i])
                );
        } else if (v.File() < NiFileVersion::V10_1_0_104) {
            set_ffi_string(f.node_name, link.targetName.get());
        } else {
            for (size_t i = 0; i < 5; i++)
                set_ffi_string(ffi_string(f, i), strings[i]->get());
        }
        out.push_back(std::move(f));
    }
    return out;
}

// The offset of `s` as a whole string in palette `bytes`, appending it with a
// zero byte if it isn't there.
inline uint32_t palette_offset(std::string& bytes, const std::string& s) {
    for (size_t at = 0; at <= bytes.size();) {
        size_t end = bytes.find('\0', at);
        if (end == std::string::npos)
            end = bytes.size();
        if (bytes.compare(at, end - at, s) == 0 && at < bytes.size())
            return static_cast<uint32_t>(at);
        at = end + 1;
    }
    // End an unterminated last string first, so it doesn't run into `s`.
    if (!bytes.empty() && bytes.back() != '\0')
        bytes.push_back('\0');
    auto at = static_cast<uint32_t>(bytes.size());
    bytes += s;
    bytes.push_back('\0');
    return at;
}

// The palette the controlled blocks of `seq` use: the controller sequence's,
// or else the first one a block refers to. NIF_NPOS if there's none.
inline uint32_t
sequence_palette(const nifly::NiHeader& hdr, nifly::NiSequence& seq) {
    using namespace nifly;
    if (auto c = dynamic_cast<NiControllerSequence*>(&seq))
        if (hdr.GetBlock<NiStringPalette>(c->stringPaletteRef))
            return c->stringPaletteRef.index;
    for (auto& link : seq.controlledBlocks)
        if (hdr.GetBlock<NiStringPalette>(link.stringPaletteRef))
            return link.stringPaletteRef.index;
    return NIF_NPOS;
}

template<class T>
bool empty_or(const nifly::NiHeader& hdr, uint32_t id) {
    return id == nifly::NIF_NPOS || hdr.GetBlock<T>(id);
}

// Replaces the controlled blocks of the sequence at `owner`. The node name is
// also the target name; strings are set in every form, and from 10.2.0.0 to
// 20.1.0.0 also written to the sequence's palette (see sequence_palette),
// which is added if needed. An unchanged string keeps its string reference
// and palette offset; others are appended to the palette unless it holds them.
// False, changing nothing, unless each ref is empty or the right type.
inline bool set_controlled_blocks(
    nifly::NifFile& file,
    uint32_t owner,
    rust::Slice<const ControlledBlockFfi> blocks
) {
    using namespace nifly;
    auto& hdr = file.GetHeader();
    auto seq = hdr.GetBlock<NiSequence>(owner);
    if (!seq)
        return false;
    for (const auto& b : blocks)
        if (!empty_or<NiInterpolator>(hdr, b.interpolator)
            || !empty_or<NiTimeController>(hdr, b.controller)
            || !empty_or<NiBlendInterpolator>(hdr, b.blend_interpolator))
            return false;

    std::vector<ControllerLink> old(
        seq->controlledBlocks.begin(),
        seq->controlledBlocks.end()
    );
    bool palette_form = uses_string_palette(hdr.GetVersion());
    uint32_t palette_id = palette_form ? sequence_palette(hdr, *seq) : NIF_NPOS;
    std::string bytes;
    if (auto p = hdr.GetBlock<NiStringPalette>(palette_id))
        bytes = palette_bytes(*p);
    size_t old_size = bytes.size();

    std::vector<ControllerLink> links;
    for (size_t i = 0; i < blocks.size(); i++) {
        const auto& b = blocks[i];
        ControllerLink link;
        if (i < old.size())
            link = old[i];
        link.interpolatorRef.index = b.interpolator;
        link.controllerRef.index = b.controller;
        link.blendInterpolatorRef.index = b.blend_interpolator;
        link.blendIndex = b.blend_index;
        link.priority = b.priority;
        link.targetName.get() = str_from(ffi_string(b, 0));
        auto strings = link_strings(link);
        auto offsets = link_offsets(link);
        std::string old_bytes;
        if (i < old.size())
            if (auto p = hdr.GetBlock<NiStringPalette>(old[i].stringPaletteRef))
                old_bytes = palette_bytes(*p);
        for (size_t s = 0; s < 5; s++) {
            std::string text = str_from(ffi_string(b, s));
            if (i >= old.size() || strings[s]->get() != text) {
                // A new reference, so an empty string saves as none.
                *strings[s] = NiStringRef();
                strings[s]->get() = text;
            }
            if (!palette_form)
                continue;
            bool kept = i < old.size()
                && old[i].stringPaletteRef.index == palette_id
                && palette_id != NIF_NPOS
                && palette_string(old_bytes, *offsets[s]) == text;
            if (kept)
                continue;
            *offsets[s] = text.empty() ? NIF_NPOS : palette_offset(bytes, text);
        }
        links.push_back(std::move(link));
    }

    if (palette_form && bytes.size() != old_size) {
        if (palette_id == NIF_NPOS) {
            palette_id = hdr.AddBlock(std::make_unique<NiStringPalette>());
            if (auto c = dynamic_cast<NiControllerSequence*>(seq))
                c->stringPaletteRef.index = palette_id;
        }
        auto p = hdr.GetBlock<NiStringPalette>(palette_id);
        bool null_output = !bytes.empty() && bytes.back() == '\0';
        if (null_output)
            bytes.pop_back();
        p->palette.get() = bytes;
        p->palette.SetNullOutput(null_output);
    }
    seq->controlledBlocks.clear();
    for (auto& link : links) {
        if (palette_form)
            link.stringPaletteRef.index = palette_id;
        seq->controlledBlocks.push_back(link);
    }
    return true;
}

// Adding sequences and notes.

inline uint32_t
add_ni_sequence(nifly::NifFile& file, rust::Slice<const uint8_t> name) {
    auto seq = std::make_unique<Sequence>();
    seq->name.get() = str_from(name);
    return file.GetHeader().AddBlock(std::move(seq));
}

inline std::unique_ptr<ControllerSequence>
new_controller_sequence(rust::Slice<const uint8_t> name) {
    auto seq = std::make_unique<ControllerSequence>();
    seq->name.get() = str_from(name);
    seq->frequency = 1.0f;
    return seq;
}

inline uint32_t add_ni_controller_sequence(
    nifly::NifFile& file,
    rust::Slice<const uint8_t> name
) {
    return file.GetHeader().AddBlock(new_controller_sequence(name));
}

inline uint32_t add_ni_sequence_stream_helper(
    nifly::NifFile& file,
    rust::Slice<const uint8_t> name
) {
    auto helper = std::make_unique<nifly::NiSequenceStreamHelper>();
    helper->name.get() = str_from(name);
    return file.GetHeader().AddBlock(std::move(helper));
}

// Adds a controller sequence named `name` to the end of the sequences of the
// manager at `owner`, managed by it. NIF_NPOS, adding nothing, if `owner`
// isn't a manager.
inline uint32_t add_manager_sequence(
    nifly::NifFile& file,
    uint32_t owner,
    rust::Slice<const uint8_t> name
) {
    auto& hdr = file.GetHeader();
    auto manager = hdr.GetBlock<nifly::NiControllerManager>(owner);
    if (!manager)
        return nifly::NIF_NPOS;
    auto seq = new_controller_sequence(name);
    seq->managerRef.index = owner;
    uint32_t index = hdr.AddBlock(std::move(seq));
    manager->controllerSequenceRefs.AddBlockRef(index);
    return index;
}

inline uint32_t add_bs_anim_note(nifly::NifFile& file) {
    return file.GetHeader().AddBlock(std::make_unique<AnimNote>());
}

NIFLY_RS_ADD_ANIMATION(BSAnimNotes, bs_anim_notes)
NIFLY_RS_ADD_CONTROLLER(NiControllerManager, ni_controller_manager)
} // namespace nifly_rs
