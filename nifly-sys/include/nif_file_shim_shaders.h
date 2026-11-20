// Part of nif_file_shim.h: shaders, properties and textures.
#pragma once

#include "nif_file_shim.h"

namespace nifly_rs {
// NifFile::GetShader. It takes a non-const shape but only reads it.
inline const nifly::NiShader*
shader(const nifly::NifFile& file, const nifly::NiShape& shape) {
    return file.GetShader(const_cast<nifly::NiShape*>(&shape));
}

// NifFile::GetAlphaProperty. It takes a non-const shape but only reads it.
inline const nifly::NiAlphaProperty*
alpha_property(const nifly::NifFile& file, const nifly::NiShape& shape) {
    return file.GetAlphaProperty(const_cast<nifly::NiShape*>(&shape));
}

// Whether the shape's alpha property belongs in its alpha property ref rather
// than its property list: NiGeometry only saves the ref from Skyrim on
// (stream > 34), the newer shape types always.
inline bool
stores_alpha_ref(const nifly::NifFile& file, nifly::NiShape& shape) {
    if (!shape.AlphaPropertyRef())
        return false;
    if (dynamic_cast<nifly::NiGeometry*>(&shape))
        return file.GetHeader().GetVersion().Stream() > 34;
    return true;
}

// Unlinks every alpha property of the shape (ref and property list) and
// deletes those no other block refers to. Upstream's RemoveAlphaProperty
// deletes them even when other shapes share them, which strips those shapes'
// alpha. Returns whether the shape had any.
inline bool
unlink_alpha_properties(nifly::NifFile& file, nifly::NiShape& shape) {
    auto& hdr = file.GetHeader();
    std::vector<uint32_t> ids;
    if (auto ref = shape.AlphaPropertyRef(); ref && hdr.GetBlock(ref)) {
        ids.push_back(ref->index);
        ref->Clear();
    }
    for (uint32_t i = 0; i < shape.propertyRefs.GetSize();) {
        uint32_t id = shape.propertyRefs.GetBlockRef(i);
        if (hdr.GetBlock<nifly::NiAlphaProperty>(id)) {
            ids.push_back(id);
            shape.propertyRefs.RemoveBlockRef(i);
        } else
            i++;
    }

    // Highest first, so deleting one doesn't shift the ids still to check.
    std::sort(ids.begin(), ids.end(), std::greater<>());
    ids.erase(std::unique(ids.begin(), ids.end()), ids.end());
    for (uint32_t id : ids)
        if (hdr.GetBlockRefCount(id, false) == 0)
            hdr.DeleteBlock(id);
    return !ids.empty();
}

// RemoveAlphaProperty on the shape at `shape_index`, keeping shared blocks.
// Returns false, changing nothing, if that block isn't a shape or has no alpha
// property.
inline bool remove_alpha_property(nifly::NifFile& file, uint32_t shape_index) {
    auto shape = file.GetHeader().GetBlock<nifly::NiShape>(shape_index);
    return shape && unlink_alpha_properties(file, *shape);
}

// AssignAlphaProperty with a new NiAlphaProperty, validated. Upstream removes
// the old property before checking it can add the new one, fails for shapes
// without a shader, and picks ref or property list by shader type, so a
// pre-Skyrim NiGeometry with another shader gets a ref that isn't saved.
// This places it by what the version saves (see stores_alpha_ref) and keeps
// shared old properties. Returns the new block's index, or NIF_NPOS (changing
// nothing) if `shape_index` isn't a shape.
inline uint32_t assign_alpha_property(
    nifly::NifFile& file,
    uint32_t shape_index,
    uint16_t flags,
    uint8_t threshold
) {
    auto& hdr = file.GetHeader();
    auto shape = hdr.GetBlock<nifly::NiShape>(shape_index);
    if (!shape)
        return nifly::NIF_NPOS;

    unlink_alpha_properties(file, *shape);
    auto alpha = std::make_unique<nifly::NiAlphaProperty>();
    alpha->flags = flags;
    alpha->threshold = threshold;
    uint32_t id = hdr.AddBlock(std::move(alpha));
    if (stores_alpha_ref(file, *shape))
        shape->AlphaPropertyRef()->index = id;
    else
        shape->propertyRefs.AddBlockRef(id);
    return id;
}

// Adds a default T to the property list of the NiAVObject at `target`.
// Returns the new block's index, or NIF_NPOS (changing nothing) if `target`
// isn't an NiAVObject.
template<typename T>
uint32_t add_property(nifly::NifFile& file, uint32_t target) {
    auto& hdr = file.GetHeader();
    auto obj = hdr.GetBlock<nifly::NiAVObject>(target);
    if (!obj)
        return nifly::NIF_NPOS;
    uint32_t id = hdr.AddBlock(std::make_unique<T>());
    obj->propertyRefs.AddBlockRef(id);
    return id;
}

#define NIFLY_RS_ADD_PROPERTY(T, snake) \
    inline uint32_t add_##snake(nifly::NifFile& file, uint32_t target) { \
        return add_property<nifly::T>(file, target); \
    }
NIFLY_RS_ADD_PROPERTY(NiMaterialProperty, material_property)
NIFLY_RS_ADD_PROPERTY(NiStencilProperty, stencil_property)
NIFLY_RS_ADD_PROPERTY(NiTexturingProperty, texturing_property)
NIFLY_RS_ADD_PROPERTY(NiSpecularProperty, specular_property)
NIFLY_RS_ADD_PROPERTY(NiVertexColorProperty, vertex_color_property)
NIFLY_RS_ADD_PROPERTY(NiDitherProperty, dither_property)
NIFLY_RS_ADD_PROPERTY(NiFogProperty, fog_property)
NIFLY_RS_ADD_PROPERTY(NiWireframeProperty, wireframe_property)
NIFLY_RS_ADD_PROPERTY(NiZBufferProperty, z_buffer_property)
NIFLY_RS_ADD_PROPERTY(
    BSShaderPPLightingProperty,
    bs_shader_pp_lighting_property
)
NIFLY_RS_ADD_PROPERTY(Lighting30ShaderProperty, lighting30_shader_property)
NIFLY_RS_ADD_PROPERTY(
    BSShaderNoLightingProperty,
    bs_shader_no_lighting_property
)
NIFLY_RS_ADD_PROPERTY(SkyShaderProperty, sky_shader_property)
NIFLY_RS_ADD_PROPERTY(TileShaderProperty, tile_shader_property)
NIFLY_RS_ADD_PROPERTY(TallGrassShaderProperty, tall_grass_shader_property)
NIFLY_RS_ADD_PROPERTY(WaterShaderProperty, water_shader_property)
NIFLY_RS_ADD_PROPERTY(HairShaderProperty, hair_shader_property)
NIFLY_RS_ADD_PROPERTY(DistantLODShaderProperty, distant_lod_shader_property)
NIFLY_RS_ADD_PROPERTY(
    BSDistantTreeShaderProperty,
    bs_distant_tree_shader_property
)
NIFLY_RS_ADD_PROPERTY(
    VolumetricFogShaderProperty,
    volumetric_fog_shader_property
)
NIFLY_RS_ADD_PROPERTY(BSWaterShaderProperty, bs_water_shader_property)
NIFLY_RS_ADD_PROPERTY(BSSkyShaderProperty, bs_sky_shader_property)
#undef NIFLY_RS_ADD_PROPERTY

// NifFile::GetStencilProperty. It takes a non-const shape but only reads it.
inline const nifly::NiStencilProperty*
stencil_property(const nifly::NifFile& file, const nifly::NiShape& shape) {
    return file.GetStencilProperty(const_cast<nifly::NiShape*>(&shape));
}

// NifFile::GetGeometryData. It takes a non-const shape but only reads it.
inline const nifly::NiGeometryData*
geometry_data(const nifly::NifFile& file, const nifly::NiShape& shape) {
    return file.GetGeometryData(const_cast<nifly::NiShape*>(&shape));
}

// NifFile::GetTexturingProperty. It takes a non-const shape but only reads it.
inline const nifly::NiTexturingProperty*
texturing_property(const nifly::NifFile& file, const nifly::NiShape& shape) {
    return file.GetTexturingProperty(const_cast<nifly::NiShape*>(&shape));
}

// NifFile::GetMaterialProperty. It takes a non-const shape but only reads it.
inline const nifly::NiMaterialProperty*
material_property(const nifly::NifFile& file, const nifly::NiShape& shape) {
    return file.GetMaterialProperty(const_cast<nifly::NiShape*>(&shape));
}

// NifFile::GetTexturePathRefs, as a count and a per-index getter, because cxx
// can't return a vector of references. Each call rebuilds upstream's list,
// which is short (a texture set has at most ~10 slots). The returned bytes are
// owned by the file's blocks. GetTexturePathRefs takes a non-const pointer but
// only reads.
inline size_t
texture_path_count(const nifly::NifFile& file, const nifly::NiShape& shape) {
    return file.GetTexturePathRefs(const_cast<nifly::NiShape*>(&shape)).size();
}

// Empty if `index` is out of range.
inline rust::Slice<const uint8_t> texture_path(
    const nifly::NifFile& file,
    const nifly::NiShape& shape,
    size_t index
) {
    auto refs = file.GetTexturePathRefs(const_cast<nifly::NiShape*>(&shape));
    if (index >= refs.size())
        return {};

    const std::string& s = refs[index].get();
    return {reinterpret_cast<const uint8_t*>(s.data()), s.size()};
}

// NifFile::GetTextureSlot. GetTextureSlot takes a non-const pointer but only
// reads.
inline TextureSlot texture_slot(
    const nifly::NifFile& file,
    const nifly::NiShape& shape,
    uint32_t index
) {
    std::string path;
    uint32_t source =
        file.GetTextureSlot(const_cast<nifly::NiShape*>(&shape), path, index);

    TextureSlot slot;
    slot.source = source;
    slot.path.reserve(path.size());
    for (char c : path)
        slot.path.push_back(static_cast<uint8_t>(c));
    return slot;
}

// Adds an NiSourceTexture for the external file `path`, attached to nothing.
inline uint32_t
add_source_texture(nifly::NifFile& file, rust::Slice<const uint8_t> path) {
    auto tex = std::make_unique<nifly::NiSourceTexture>();
    tex->fileName.get().assign(
        reinterpret_cast<const char*>(path.data()),
        path.size()
    );
    return file.GetHeader().AddBlock(std::move(tex));
}

// Gives the NiSourceTexture at `texture_index` a new default T as its pixel
// data, replacing its ref. Returns the new block's index, or NIF_NPOS (changing
// nothing) if that block isn't an NiSourceTexture.
template<typename T>
uint32_t assign_pixel_data(nifly::NifFile& file, uint32_t texture_index) {
    auto& hdr = file.GetHeader();
    auto tex = hdr.GetBlock<nifly::NiSourceTexture>(texture_index);
    if (!tex)
        return nifly::NIF_NPOS;
    uint32_t id = hdr.AddBlock(std::make_unique<T>());
    tex->dataRef.index = id;
    return id;
}

inline uint32_t assign_ni_pixel_data(nifly::NifFile& file, uint32_t texture) {
    return assign_pixel_data<nifly::NiPixelData>(file, texture);
}

inline uint32_t
assign_persistent_pixel_data(nifly::NifFile& file, uint32_t texture) {
    return assign_pixel_data<nifly::NiPersistentSrcTextureRendererData>(
        file,
        texture
    );
}

// Gives the TextureRenderData at `data_index` a new NiPalette with these
// colors (flattened r, g, b, a). Returns its index, or NIF_NPOS (changing
// nothing) if that block isn't a TextureRenderData or there aren't 16 or 256
// colors.
inline uint32_t assign_palette(
    nifly::NifFile& file,
    uint32_t data_index,
    rust::Slice<const uint8_t> rgba,
    bool has_alpha
) {
    auto& hdr = file.GetHeader();
    auto data = hdr.GetBlock<nifly::TextureRenderData>(data_index);
    if (!data)
        return nifly::NIF_NPOS;
    size_t n = rgba.size() / 4;
    if (rgba.size() % 4 != 0 || (n != 16 && n != 256))
        return nifly::NIF_NPOS;
    auto palette = std::make_unique<nifly::NiPalette>();
    palette->palette.resize(static_cast<uint32_t>(n));
    for (uint32_t i = 0; i < n; i++)
        palette->palette[i] =
            {rgba[i * 4], rgba[i * 4 + 1], rgba[i * 4 + 2], rgba[i * 4 + 3]};
    palette->hasAlpha = has_alpha;
    uint32_t id = hdr.AddBlock(std::move(palette));
    data->paletteRef.index = id;
    return id;
}

// Makes `map` of `t` present with source `source` (NIF_NPOS for an empty ref),
// raising textureCount so it's saved where the version can save it.
inline void attach_texturing_map(
    const nifly::NiVersion& version,
    nifly::NiTexturingProperty& t,
    const TexturingMap& members,
    uint32_t map,
    uint32_t source
) {
    // Files can store any nonzero value; keep it.
    if (!(t.*members.has))
        t.*members.has = true;
    (t.*members.desc).sourceRef.index = source;
    t.textureCount =
        std::max(t.textureCount, texturing_map_min_count(version, map));
}

// Sets the source texture of `map` of the NiTexturingProperty at
// `property_index`, adding the map if it's absent, or removes the map if
// `source_index` is NIF_NPOS. False, changing nothing, if the property or
// source has the wrong type or `map` is out of range.
inline bool set_map_source(
    nifly::NifFile& file,
    uint32_t property_index,
    uint32_t map,
    uint32_t source_index
) {
    auto& hdr = file.GetHeader();
    auto t = hdr.GetBlock<nifly::NiTexturingProperty>(property_index);
    if (!t)
        return false;
    if (map >= texturing_shader_maps) {
        auto shader = texturing_shader_map(*t, map);
        if (!shader)
            return false;
        if (source_index == nifly::NIF_NPOS) {
            shader->isUsed = false;
            shader->data.sourceRef.Clear();
            return true;
        }
        if (!hdr.GetBlock<nifly::NiSourceTexture>(source_index))
            return false;
        shader->isUsed = true;
        shader->data.sourceRef.index = source_index;
        return true;
    }
    auto members = texturing_map_members(map);
    if (!members)
        return false;

    if (source_index == nifly::NIF_NPOS) {
        t->*members->has = false;
        (t->*members->desc).sourceRef.Clear();
        return true;
    }
    if (!hdr.GetBlock<nifly::NiSourceTexture>(source_index))
        return false;
    attach_texturing_map(hdr.GetVersion(), *t, *members, map, source_index);
    return true;
}

// NifFile::SetTextureSlot, with its NiTexturingProperty part replaced. That
// part set textureCount to `index + 1`, which could drop higher maps on save,
// and set decal paths in files from 20.2.0.5 on without a count that saves
// them. Here the count only grows, and a map without a source texture gets a
// new one. Does nothing if `shape_index` isn't a shape.
inline void set_texture_slot(
    nifly::NifFile& file,
    uint32_t shape_index,
    rust::Slice<const uint8_t> path,
    uint32_t index
) {
    auto& hdr = file.GetHeader();
    auto* shape = hdr.GetBlock<nifly::NiShape>(shape_index);
    if (!shape)
        return;

    std::string c_path(reinterpret_cast<const char*>(path.data()), path.size());

    // The shader cases SetTextureSlot handles: a texture set slot in range, or
    // an effect shader without a texture set.
    if (auto shader = file.GetShader(shape)) {
        auto set_ref = shader->TextureSetRef();
        auto set = set_ref ? hdr.GetBlock(*set_ref) : nullptr;
        if ((set && index < set->textures.size())
            || (!set && dynamic_cast<nifly::BSEffectShaderProperty*>(shader))) {
            file.SetTextureSlot(shape, c_path, index);
            return;
        }
    }

    auto t = file.GetTexturingProperty(shape);
    if (!t || index > 9)
        return;
    // Slots 6 to 9 are the decals.
    uint32_t map = index < 6 ? index : index + 2;
    const auto& members = *texturing_map_members(map);

    auto& source_ref = (t->*members.desc).sourceRef;
    auto source = hdr.GetBlock<nifly::NiSourceTexture>(source_ref);
    uint32_t source_index = source_ref.index;
    if (!source)
        source_index = add_source_texture(file, path);
    else
        source->fileName.get() = c_path;
    attach_texturing_map(hdr.GetVersion(), *t, members, map, source_index);
}
} // namespace nifly_rs
