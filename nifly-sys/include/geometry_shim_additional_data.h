// Part of geometry_shim.h: additional geometry data.
#pragma once

#include "geometry_shim.h"

namespace nifly_rs {
// The two concrete AdditionalGeomData types have the same vertex count and
// channels, but no common base holding them.
template<typename F>
auto with_agd(const nifly::AdditionalGeomData& d, F f) {
    if (auto ni = dynamic_cast<const nifly::NiAdditionalGeometryData*>(&d))
        return f(*ni);
    return f(dynamic_cast<const nifly::BSPackedAdditionalGeometryData&>(d));
}

template<typename F>
void with_agd_mut(nifly::AdditionalGeomData& d, F f) {
    if (auto ni = dynamic_cast<nifly::NiAdditionalGeometryData*>(&d))
        f(*ni);
    else if (
        auto packed = dynamic_cast<nifly::BSPackedAdditionalGeometryData*>(&d)
    )
        f(*packed);
}

inline uint16_t agd_vertex_count(const nifly::AdditionalGeomData& d) {
    return with_agd(d, [](const auto& t) { return t.numVertices; });
}

inline void set_agd_vertex_count(nifly::AdditionalGeomData& d, uint16_t v) {
    with_agd_mut(d, [&](auto& t) { t.numVertices = v; });
}

inline rust::Vec<AdditionalDataChannel>
agd_channels(const nifly::AdditionalGeomData& d) {
    return with_agd(d, [](const auto& t) {
        rust::Vec<AdditionalDataChannel> out;
        const auto* infos = t.blockInfos.data();
        for (uint32_t i = 0; i < t.blockInfos.size(); ++i) {
            const auto& c = infos[i];
            out.push_back(
                {c.dataType,
                 c.numChannelBytesPerElement,
                 c.numChannelBytes,
                 c.numTotalBytesPerElement,
                 c.blockIndex,
                 c.channelOffset,
                 c.unkByte1}
            );
        }
        return out;
    });
}

inline void set_agd_channels(
    nifly::AdditionalGeomData& d,
    rust::Slice<const AdditionalDataChannel> channels
) {
    with_agd_mut(d, [&](auto& t) {
        t.blockInfos.clear();
        for (const auto& c : channels) {
            nifly::AdditionalDataInfo info;
            info.dataType = c.data_type;
            info.numChannelBytesPerElement = c.unit_size;
            info.numChannelBytes = c.total_size;
            info.numTotalBytesPerElement = c.stride;
            info.blockIndex = c.block;
            info.channelOffset = c.offset;
            info.unkByte1 = c.flags;
            t.blockInfos.push_back(info);
        }
    });
}

template<typename T>
rust::Slice<const T> slice_of(const std::vector<T>& v) {
    return {v.data(), v.size()};
}

inline size_t agd_block_count(const nifly::NiAdditionalGeometryData& d) {
    return d.blocks.size();
}

inline bool
agd_block_has_data(const nifly::NiAdditionalGeometryData& d, size_t i) {
    return d.blocks.data()[i].hasData;
}

inline uint32_t
agd_block_size(const nifly::NiAdditionalGeometryData& d, size_t i) {
    return d.blocks.data()[i].blockSize;
}

inline rust::Slice<const uint32_t>
agd_block_offsets(const nifly::NiAdditionalGeometryData& d, size_t i) {
    return slice_of(d.blocks.data()[i].blockOffsets);
}

inline rust::Slice<const uint32_t>
agd_block_data_sizes(const nifly::NiAdditionalGeometryData& d, size_t i) {
    return slice_of(d.blocks.data()[i].dataSizes);
}

inline rust::Slice<const uint8_t>
agd_block_data(const nifly::NiAdditionalGeometryData& d, size_t i, size_t j) {
    return slice_of(d.blocks.data()[i].data[j]);
}

inline void clear_agd_blocks(nifly::NiAdditionalGeometryData& d) {
    d.blocks.clear();
}

inline void push_agd_block(
    nifly::NiAdditionalGeometryData& d,
    bool has_data,
    uint32_t block_size,
    rust::Slice<const uint32_t> offsets,
    rust::Slice<const uint32_t> data_sizes,
    rust::Slice<const uint8_t> data
) {
    nifly::AdditionalDataBlock b;
    b.hasData = has_data;
    if (has_data) {
        b.blockSize = block_size;
        b.numBlocks = static_cast<uint32_t>(offsets.size());
        b.blockOffsets.assign(offsets.begin(), offsets.end());
        b.numData = static_cast<uint32_t>(data_sizes.size());
        b.dataSizes.assign(data_sizes.begin(), data_sizes.end());
        for (size_t j = 0; j < data_sizes.size(); ++j) {
            auto start = data.begin() + j * block_size;
            b.data.emplace_back(start, start + block_size);
        }
    }
    d.blocks.push_back(b);
}

inline size_t
packed_agd_block_count(const nifly::BSPackedAdditionalGeometryData& d) {
    return d.blocks.size();
}

inline bool packed_agd_block_has_data(
    const nifly::BSPackedAdditionalGeometryData& d,
    size_t i
) {
    return d.blocks.data()[i].hasData;
}

inline rust::Slice<const uint32_t> packed_agd_block_offsets(
    const nifly::BSPackedAdditionalGeometryData& d,
    size_t i
) {
    return slice_of(d.blocks.data()[i].blockOffsets);
}

inline rust::Slice<const uint32_t> packed_agd_block_atom_sizes(
    const nifly::BSPackedAdditionalGeometryData& d,
    size_t i
) {
    return slice_of(d.blocks.data()[i].atomSizes);
}

inline rust::Slice<const uint8_t> packed_agd_block_data(
    const nifly::BSPackedAdditionalGeometryData& d,
    size_t i
) {
    return slice_of(d.blocks.data()[i].data);
}

inline void packed_agd_block_ints(
    const nifly::BSPackedAdditionalGeometryData& d,
    size_t i,
    rust::Slice<uint32_t> out
) {
    out[0] = d.blocks.data()[i].unkInt1;
    out[1] = d.blocks.data()[i].numTotalBytesPerElement;
}

inline void clear_packed_agd_blocks(nifly::BSPackedAdditionalGeometryData& d) {
    d.blocks.clear();
}

inline void push_packed_agd_block(
    nifly::BSPackedAdditionalGeometryData& d,
    bool has_data,
    rust::Slice<const uint32_t> offsets,
    rust::Slice<const uint32_t> atom_sizes,
    rust::Slice<const uint8_t> data,
    uint32_t shader_index,
    uint32_t total_size
) {
    nifly::BSPackedAdditionalDataBlock b;
    b.hasData = has_data;
    if (has_data) {
        b.numTotalBytes = static_cast<uint32_t>(data.size());
        b.numBlocks = static_cast<uint32_t>(offsets.size());
        b.blockOffsets.assign(offsets.begin(), offsets.end());
        b.numAtoms = static_cast<uint32_t>(atom_sizes.size());
        b.atomSizes.assign(atom_sizes.begin(), atom_sizes.end());
        b.data.assign(data.begin(), data.end());
    }
    b.unkInt1 = shader_index;
    b.numTotalBytesPerElement = total_size;
    d.blocks.push_back(b);
}

inline uint32_t geometry_data_additional_data(const nifly::NiGeometryData& d) {
    return d.additionalDataRef.index;
}
} // namespace nifly_rs
