// Part of shaders_shim.h: source textures, render data, pixel data and palettes.
#pragma once

#include "shaders_shim.h"

namespace nifly_rs {
// NiSourceTexture fields. The enums are passed as their raw values.
#define NIFLY_RS_SOURCE(name, Type, field) \
    NIFLY_RS_FIELD( \
        NiSourceTexture, \
        source, \
        name, \
        Type, \
        field, \
        NIFLY_RS_SAME, \
        NIFLY_RS_SAME \
    )
#define NIFLY_RS_SOURCE_ENUM(name, Enum, field) \
    inline uint32_t source_##name(const nifly::NiSourceTexture& s) { \
        return s.field; \
    } \
    inline void set_source_##name(nifly::NiSourceTexture& s, uint32_t v) { \
        s.field = static_cast<nifly::Enum>(v); \
    }

NIFLY_RS_SOURCE(use_external, bool, useExternal)
NIFLY_RS_SOURCE(use_internal, bool, useInternal)
NIFLY_RS_SOURCE(is_static, bool, isStatic)
NIFLY_RS_SOURCE(direct_render, bool, directRender)
NIFLY_RS_SOURCE(persistent_render_data, bool, persistentRenderData)
NIFLY_RS_SOURCE_ENUM(pixel_layout, PixelLayout, pixelLayout)
NIFLY_RS_SOURCE_ENUM(mip_map_format, MipMapFormat, mipMapFormat)
NIFLY_RS_SOURCE_ENUM(alpha_format, AlphaFormat, alphaFormat)

inline rust::Slice<const uint8_t>
source_file_name(const nifly::NiSourceTexture& s) {
    const std::string& str = s.fileName.get();
    return {reinterpret_cast<const uint8_t*>(str.data()), str.size()};
}

inline void set_source_file_name(
    nifly::NiSourceTexture& s,
    rust::Slice<const uint8_t> path
) {
    s.fileName.get().assign(
        reinterpret_cast<const char*>(path.data()),
        path.size()
    );
}

// The pixel data's block index; NIF_NPOS if the ref is empty.
inline uint32_t source_pixel_data(const nifly::NiSourceTexture& s) {
    return s.dataRef.index;
}

// TextureRenderData fields.
#define NIFLY_RS_RENDER_FIELD(field, type, snake) \
    inline type render_data_##snake(const nifly::TextureRenderData& d) { \
        return static_cast<type>(d.field); \
    } \
    inline void set_render_data_##snake(nifly::TextureRenderData& d, type v) { \
        d.field = static_cast<decltype(d.field)>(v); \
    }
NIFLY_RS_RENDER_FIELD(pixelFormat, uint32_t, pixel_format)
NIFLY_RS_RENDER_FIELD(bitsPerPixel, uint8_t, bits_per_pixel)
NIFLY_RS_RENDER_FIELD(bytesPerPixel, uint32_t, bytes_per_pixel)
NIFLY_RS_RENDER_FIELD(rendererHint, uint32_t, renderer_hint)
NIFLY_RS_RENDER_FIELD(extraData, uint32_t, extra_data)
NIFLY_RS_RENDER_FIELD(flags, uint8_t, flags)
NIFLY_RS_RENDER_FIELD(pixelTiling, uint32_t, tiling)

inline std::array<uint8_t, 8>
render_data_old_fast_compare(const nifly::TextureRenderData& d) {
    std::array<uint8_t, 8> out;
    std::copy(
        std::begin(d.oldFastCompare),
        std::end(d.oldFastCompare),
        out.begin()
    );
    return out;
}

inline void set_render_data_old_fast_compare(
    nifly::TextureRenderData& d,
    std::array<uint8_t, 8> v
) {
    std::copy(v.begin(), v.end(), std::begin(d.oldFastCompare));
}

#undef NIFLY_RS_RENDER_FIELD

// Red, green, blue and alpha masks. `out` and `v` have 4 elements.
inline void render_data_masks(
    const nifly::TextureRenderData& d,
    rust::Slice<uint32_t> out
) {
    out[0] = d.redMask;
    out[1] = d.greenMask;
    out[2] = d.blueMask;
    out[3] = d.alphaMask;
}

inline void set_render_data_masks(
    nifly::TextureRenderData& d,
    rust::Slice<const uint32_t> v
) {
    d.redMask = v[0];
    d.greenMask = v[1];
    d.blueMask = v[2];
    d.alphaMask = v[3];
}

// Channel `index` (0 to 3) as component, representation, bits and signedness.
// False if `index` is out of range.
inline bool render_data_channel(
    const nifly::TextureRenderData& d,
    size_t index,
    rust::Slice<uint32_t> out
) {
    if (index >= 4)
        return false;
    const auto& c = d.channels[index];
    out[0] = c.type;
    out[1] = c.convention;
    out[2] = c.bitsPerChannel;
    out[3] = c.isSigned;
    return true;
}

inline bool set_render_data_channel(
    nifly::TextureRenderData& d,
    size_t index,
    rust::Slice<const uint32_t> v
) {
    if (index >= 4)
        return false;
    auto& c = d.channels[index];
    c.type = static_cast<nifly::PixelComponent>(v[0]);
    c.convention = static_cast<nifly::PixelRepresentation>(v[1]);
    c.bitsPerChannel = static_cast<uint8_t>(v[2]);
    c.isSigned = v[3] != 0;
    return true;
}

// Mipmaps flattened as width, height, offset.
inline rust::Vec<uint32_t>
render_data_mipmaps(const nifly::TextureRenderData& d) {
    rust::Vec<uint32_t> out;
    auto& mipmaps = const_cast<nifly::TextureRenderData&>(d).mipmaps;
    for (const auto& m : mipmaps) {
        out.push_back(m.width);
        out.push_back(m.height);
        out.push_back(m.offset);
    }
    return out;
}

inline void set_render_data_mipmaps(
    nifly::TextureRenderData& d,
    rust::Slice<const uint32_t> v
) {
    d.mipmaps.clear();
    for (size_t i = 0; i + 3 <= v.size(); i += 3) {
        nifly::MipMapInfo m;
        m.width = v[i];
        m.height = v[i + 1];
        m.offset = v[i + 2];
        d.mipmaps.push_back(m);
    }
}

inline uint32_t render_data_palette(const nifly::TextureRenderData& d) {
    return d.paletteRef.index;
}

// Pixel bytes of NiPixelData and NiPersistentSrcTextureRendererData, per face.
// Setters keep numPixels and numFaces in step.
template<typename T>
size_t pixel_face_count(const T& d) {
    return d.pixelData.size();
}

template<typename T>
rust::Slice<const uint8_t> pixel_face(const T& d, size_t face) {
    if (face >= d.pixelData.size())
        return {};
    return {d.pixelData[face].data(), d.pixelData[face].size()};
}

template<typename T>
void clear_pixel_faces(T& d) {
    d.pixelData.clear();
    d.numFaces = 0;
    d.numPixels = 0;
}

// The caller makes every face the same length.
template<typename T>
void push_pixel_face(T& d, rust::Slice<const uint8_t> bytes) {
    d.pixelData.emplace_back(bytes.begin(), bytes.end());
    d.numFaces = static_cast<uint32_t>(d.pixelData.size());
    d.numPixels = static_cast<uint32_t>(bytes.size());
}

#define NIFLY_RS_PIXELS(T, snake) \
    inline size_t snake##_face_count(const nifly::T& d) { \
        return pixel_face_count(d); \
    } \
    inline rust::Slice<const uint8_t> snake##_face( \
        const nifly::T& d, \
        size_t face \
    ) { \
        return pixel_face(d, face); \
    } \
    inline void clear_##snake##_faces(nifly::T& d) { \
        clear_pixel_faces(d); \
    } \
    inline void push_##snake##_face( \
        nifly::T& d, \
        rust::Slice<const uint8_t> bytes \
    ) { \
        push_pixel_face(d, bytes); \
    }
NIFLY_RS_PIXELS(NiPixelData, pixel_data)
NIFLY_RS_PIXELS(NiPersistentSrcTextureRendererData, persistent_data)
#undef NIFLY_RS_PIXELS

inline uint32_t
persistent_data_platform(const nifly::NiPersistentSrcTextureRendererData& d) {
    return d.platform;
}

inline void set_persistent_data_platform(
    nifly::NiPersistentSrcTextureRendererData& d,
    uint32_t v
) {
    d.platform = static_cast<nifly::PlatformID>(v);
}

inline uint32_t persistent_data_padded_pixel_count(
    const nifly::NiPersistentSrcTextureRendererData& d
) {
    return d.padNumPixels;
}

inline void set_persistent_data_padded_pixel_count(
    nifly::NiPersistentSrcTextureRendererData& d,
    uint32_t v
) {
    d.padNumPixels = v;
}

inline bool palette_has_alpha(const nifly::NiPalette& p) {
    return p.hasAlpha;
}

inline void set_palette_has_alpha(nifly::NiPalette& p, bool v) {
    p.hasAlpha = v;
}

// Colors flattened as r, g, b, a.
inline rust::Vec<uint8_t> palette_colors(const nifly::NiPalette& p) {
    rust::Vec<uint8_t> out;
    for (const auto& c : const_cast<nifly::NiPalette&>(p).palette) {
        out.push_back(c.r);
        out.push_back(c.g);
        out.push_back(c.b);
        out.push_back(c.a);
    }
    return out;
}

// False, changing nothing, unless there are 16 or 256 colors.
inline bool
set_palette_colors(nifly::NiPalette& p, rust::Slice<const uint8_t> rgba) {
    size_t n = rgba.size() / 4;
    if (rgba.size() % 4 != 0 || (n != 16 && n != 256))
        return false;
    p.palette.resize(static_cast<uint32_t>(n));
    for (size_t i = 0; i < n; i++) {
        auto& c = p.palette[static_cast<uint32_t>(i)];
        c.r = rgba[i * 4];
        c.g = rgba[i * 4 + 1];
        c.b = rgba[i * 4 + 2];
        c.a = rgba[i * 4 + 3];
    }
    return true;
}

// The block index of the shader's texture set ref; NIF_NPOS if the shader type
// has none or it's empty. Only BSLightingShaderProperty and
// BSShaderPPLightingProperty have one.
inline uint32_t shader_texture_set(const nifly::NiShader& shader) {
    auto ref = shader.TextureSetRef();
    return ref ? ref->index : nifly::NIF_NPOS;
}

#undef NIFLY_RS_STRING_FIELD
#undef NIFLY_RS_ENUM_FIELD
#undef NIFLY_RS_PROPERTY
#undef NIFLY_RS_SOURCE_ENUM
#undef NIFLY_RS_SOURCE
#undef NIFLY_RS_EFFECT_TEXTURE
#undef NIFLY_RS_EFFECT
#undef NIFLY_RS_LIGHTING_COLOR
#undef NIFLY_RS_LIGHTING
#undef NIFLY_RS_SAME
#undef NIFLY_RS_FIELD
} // namespace nifly_rs
