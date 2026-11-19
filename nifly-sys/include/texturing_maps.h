#pragma once

#include <cstdint>
#include <iterator>

#include "Shaders.hpp"

namespace nifly_rs {
// The members of NiTexturingProperty map `map`, in file order (base, dark,
// detail, gloss, glow, bump, normal, parallax, decals 0-3).
struct TexturingMap {
    nifly::NiBool nifly::NiTexturingProperty::* has;
    nifly::TexDesc nifly::NiTexturingProperty::* desc;
};

// Null if `map` is out of range.
inline const TexturingMap* texturing_map_members(uint32_t map) {
    using P = nifly::NiTexturingProperty;
    static const TexturingMap maps[] = {
        {&P::hasBaseTex, &P::baseTex},
        {&P::hasDarkTex, &P::darkTex},
        {&P::hasDetailTex, &P::detailTex},
        {&P::hasGlossTex, &P::glossTex},
        {&P::hasGlowTex, &P::glowTex},
        {&P::hasBumpTex, &P::bumpTex},
        {&P::hasNormalTex, &P::normalTex},
        {&P::hasParallaxTex, &P::parallaxTex},
        {&P::hasDecalTex0, &P::decalTex0},
        {&P::hasDecalTex1, &P::decalTex1},
        {&P::hasDecalTex2, &P::decalTex2},
        {&P::hasDecalTex3, &P::decalTex3},
    };
    return map < std::size(maps) ? &maps[map] : nullptr;
}

// Map numbers from this one on are shader maps, `map - texturing_shader_maps`.
constexpr uint32_t texturing_shader_maps = 12;

// Shader map `map - texturing_shader_maps`, used or not, or null if out of
// range.
inline nifly::ShaderTexDesc*
texturing_shader_map(nifly::NiTexturingProperty& t, uint32_t map) {
    if (map < texturing_shader_maps)
        return nullptr;
    uint32_t i = map - texturing_shader_maps;
    return i < t.shaderTex.size() ? &t.shaderTex.data()[i] : nullptr;
}

// The smallest textureCount that saves map `map` in a file of `version`, or 0
// if that version can't save it (normal and parallax maps before 20.2.0.5).
inline uint32_t
texturing_map_min_count(const nifly::NiVersion& version, uint32_t map) {
    if (map < 6)
        return map + 1;
    if (version.File() >= nifly::NiFileVersion::V20_2_0_5)
        return map < 12 ? map + 1 : 0;
    // No normal or parallax maps; decals are saved as the 7th to 10th.
    return map >= 8 && map < 12 ? map - 1 : 0;
}
} // namespace nifly_rs
