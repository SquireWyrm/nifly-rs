#pragma once

#include <cmath>
#include <cstdint>
#include <memory>
#include <vector>

#include "Factory.hpp"
#include "Geometry.hpp"

namespace nifly_rs {

// Writes a BSGeometryMeshData as BSGeometryMeshData::Sync does, with two
// differences. Upstream calls SetUVs(true) while writing, which drops the
// second UV set (it's written as zeros and lost in memory) and cuts the first
// to 65535 entries. And it resizes normals and tangents to the 16-bit vertex
// count first; this writes the arrays as they are, so data saves as read.
// Nothing in `d` changes.
inline void write_mesh_data(
    const nifly::BSGeometryMeshData& d,
    nifly::NiStreamReversible& s
) {
    auto put = [&](auto value) { s.Sync(value); };
    put(d.version);
    if (d.version > 2)
        return;

    put(static_cast<uint32_t>(d.tris.size() * 3));
    for (auto t : d.tris)
        s.Sync(t);

    put(d.scale);
    if (d.scale <= 0.0f)
        return;

    put(d.nWeightsPerVert);

    put(static_cast<uint32_t>(d.vertices.size()));
    // BSGeometryMeshData::havokScale, which is private.
    constexpr float havok_scale = 69.969f;
    const float range = d.scale * havok_scale;
    for (const auto& v : d.vertices)
        for (float c : {v.x, v.y, v.z})
            put(static_cast<int16_t>(
                std::round((c / range) * (c < 0 ? 32768.0f : 32767.0f))
            ));

    for (size_t set = 0; set < 2; set++) {
        const auto* uvs = set < d.uvSets.size() ? &d.uvSets[set] : nullptr;
        put(static_cast<uint32_t>(uvs ? uvs->size() : 0));
        if (uvs)
            for (auto uv : *uvs) {
                s.SyncHalf(uv.u);
                s.SyncHalf(uv.v);
            }
    }

    put(static_cast<uint32_t>(d.vColors.size()));
    for (auto c : d.vColors)
        s.Sync(c);

    put(static_cast<uint32_t>(d.normals.size()));
    for (auto n : d.normals)
        s.SyncUDEC3(n);

    put(static_cast<uint32_t>(d.tangents.size()));
    for (size_t i = 0; i < d.tangents.size(); i++) {
        auto t = d.tangents[i];
        uint8_t w = i < d.tangentWs.size() ? d.tangentWs[i] : 1;
        s.SyncUDEC3(t, w);
    }

    const uint32_t per = d.nWeightsPerVert;
    put(static_cast<uint32_t>(per ? d.skinWeights.size() * per : 0));
    if (per)
        for (const auto& vertex : d.skinWeights)
            for (uint32_t j = 0; j < per; j++) {
                auto w = j < vertex.size()
                    ? vertex[j]
                    : nifly::BSGeometryMeshData::BoneWeight {};
                s.Sync(w);
            }

    put(static_cast<uint32_t>(d.lods.size()));
    for (const auto& lod : d.lods) {
        put(static_cast<uint32_t>(lod.size() * 3));
        for (auto t : lod)
            s.Sync(t);
    }

    put(static_cast<uint32_t>(d.meshletList.size()));
    for (const auto& m : d.meshletList)
        for (uint32_t v :
             {m.vertCount, m.vertOffset, m.primCount, m.primOffset})
            put(v);

    put(static_cast<uint32_t>(d.cullDataList.size()));
    for (auto c : d.cullDataList) {
        s.Sync(c.center);
        s.Sync(c.expand);
    }
}

// A BSGeometry whose internal meshes are written with write_mesh_data.
// GeometryFactory loads every BSGeometry as one. Clones stay Geometrys.
class Geometry final: public nifly::BSGeometry {
  public:
    void Put(nifly::NiOStream& stream) override {
        nifly::NiShape::Put(stream);
        nifly::NiStreamReversible s(
            nullptr,
            &stream,
            nifly::NiStreamReversible::Mode::Writing
        );
        s.Sync(bounds);
        for (float f : boundMinMax)
            s.Sync(f);
        skinInstanceRef.Sync(s);
        shaderPropertyRef.Sync(s);
        alphaPropertyRef.Sync(s);
        const bool internal = HasInternalGeomData();
        for (size_t i = 0; i < 4; i++) {
            uint8_t present = i < meshes.size();
            s.Sync(present);
            if (!present)
                continue;
            auto& mesh = meshes[i];
            mesh.internalGeom = internal;
            s.Sync(mesh.triSize);
            s.Sync(mesh.numVerts);
            s.Sync(mesh.flags);
            if (internal)
                write_mesh_data(mesh.meshData, s);
            else
                mesh.meshName.Sync(s, 4);
        }
    }

  private:
    Geometry* Clone_impl() const override {
        return new Geometry(*this);
    }
};

class GeometryFactory final: public nifly::NiFactory {
  public:
    std::unique_ptr<nifly::NiObject> Create() override {
        return std::make_unique<Geometry>();
    }

    std::unique_ptr<nifly::NiObject> Load(nifly::NiIStream& stream) override {
        auto geometry = std::make_unique<Geometry>();
        geometry->Get(stream);
        return geometry;
    }
};
} // namespace nifly_rs
