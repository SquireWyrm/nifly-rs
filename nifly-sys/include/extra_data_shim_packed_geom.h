// Part of extra_data_shim.h: BSPackedCombinedSharedGeomDataExtra, its replacement class and factory, and its fields.
#pragma once

#include "extra_data_shim.h"

namespace nifly_rs {
// BSPackedCombinedSharedGeomDataExtra reads each object's data with vertices
// and triangles, as BSPackedCombinedGeomDataExtra stores them. The shared
// block keeps those in the CSG file and stores the data only up to the vertex
// description, so files are misread from the first object on.
// PackedSharedGeomData syncs that layout; PackedSharedGeomDataFactory loads
// every BSPackedCombinedSharedGeomDataExtra as one, and assign_packed_shared_geom
// creates them. Clones stay PackedSharedGeomData.
class PackedSharedGeomData final:
    public nifly::BSPackedCombinedSharedGeomDataExtra {
  public:
    void Get(nifly::NiIStream& stream) override {
        nifly::NiExtraData::Get(stream);
        nifly::NiStreamReversible s(
            &stream,
            nullptr,
            nifly::NiStreamReversible::Mode::Reading
        );
        sync(s);
    }

    void Put(nifly::NiOStream& stream) override {
        nifly::NiExtraData::Put(stream);
        nifly::NiStreamReversible s(
            nullptr,
            &stream,
            nifly::NiStreamReversible::Mode::Writing
        );
        numData = static_cast<uint32_t>(objects.size());
        sync(s);
    }

  private:
    PackedSharedGeomData* Clone_impl() const override {
        return new PackedSharedGeomData(*this);
    }

    void sync(nifly::NiStreamReversible& s) {
        vertexDesc.Sync(s);
        s.Sync(numVertices);
        s.Sync(numTriangles);
        s.Sync(unkFlags1);
        s.Sync(unkFlags2);
        s.Sync(numData);
        if (numData > nifly::NIF_ARRAY_SIZE_LIMIT)
            throw std::length_error("IO: Array size is too large.");
        objects.resize(numData);
        data.resize(numData);
        for (auto& o : objects)
            s.Sync(o);
        for (auto& d : data) {
            s.Sync(d.numVertices);
            s.Sync(d.lodLevels);
            s.Sync(d.triCountLod0);
            s.Sync(d.triOffsetLod0);
            s.Sync(d.triCountLod1);
            s.Sync(d.triOffsetLod1);
            s.Sync(d.triCountLod2);
            s.Sync(d.triOffsetLod2);
            d.combined.Sync(s);
            d.vertexDesc.Sync(s);
        }
    }
};

class PackedSharedGeomDataFactory final: public nifly::NiFactory {
  public:
    std::unique_ptr<nifly::NiObject> Create() override {
        return std::make_unique<PackedSharedGeomData>();
    }

    std::unique_ptr<nifly::NiObject> Load(nifly::NiIStream& stream) override {
        auto block = std::make_unique<PackedSharedGeomData>();
        block->Get(stream);
        return block;
    }
};

inline PackedGeomHeader
packed_shared_header(const nifly::BSPackedCombinedSharedGeomDataExtra& obj) {
    return {
        vertex_desc_bits(obj.vertexDesc),
        obj.numVertices,
        obj.numTriangles,
        obj.unkFlags1,
        obj.unkFlags2
    };
}

inline void set_packed_shared_header(
    nifly::BSPackedCombinedSharedGeomDataExtra& obj,
    PackedGeomHeader h
) {
    set_vertex_desc_bits(obj.vertexDesc, h.vertex_desc);
    obj.numVertices = h.vertex_count;
    obj.numTriangles = h.triangle_count;
    obj.unkFlags1 = h.flags_1;
    obj.unkFlags2 = h.flags_2;
}

inline rust::Vec<PackedGeomObjectFfi>
packed_shared_objects(const nifly::BSPackedCombinedSharedGeomDataExtra& obj) {
    rust::Vec<PackedGeomObjectFfi> out;
    for (size_t i = 0; i < obj.objects.size() && i < obj.data.size(); i++) {
        const auto& o = obj.objects[i];
        const auto& d = obj.data[i];
        out.push_back(
            {o.fileNameHash,
             o.dataOffset,
             d.numVertices,
             d.lodLevels,
             {d.triCountLod0,
              d.triOffsetLod0,
              d.triCountLod1,
              d.triOffsetLod1,
              d.triCountLod2,
              d.triOffsetLod2},
             vertex_desc_bits(d.vertexDesc),
             d.combined.size()}
        );
    }
    return out;
}

inline rust::Vec<float>
packed_shared_instances(const nifly::BSPackedCombinedSharedGeomDataExtra& obj) {
    static_assert(
        sizeof(nifly::BSPackedGeomDataCombined) == 18 * sizeof(float)
    );
    rust::Vec<float> out;
    for (size_t i = 0; i < obj.objects.size() && i < obj.data.size(); i++) {
        const auto& combined = obj.data[i].combined;
        const float* f = reinterpret_cast<const float*>(combined.data());
        for (size_t j = 0; j < combined.size() * 18; j++)
            out.push_back(f[j]);
    }
    return out;
}

inline bool set_packed_shared_objects(
    nifly::BSPackedCombinedSharedGeomDataExtra& obj,
    rust::Slice<const PackedGeomObjectFfi> objects,
    rust::Slice<const float> instances
) {
    size_t total = 0;
    for (const auto& o : objects)
        total += o.instance_count;
    if (total * 18 != instances.size())
        return false;

    obj.objects.clear();
    obj.data.clear();
    const float* f = instances.data();
    for (const auto& o : objects) {
        nifly::BSPackedGeomObject object;
        object.fileNameHash = o.filename_hash;
        object.dataOffset = o.data_offset;
        obj.objects.push_back(object);

        nifly::BSPackedGeomData d;
        d.numVertices = o.vertex_count;
        d.lodLevels = o.lod_levels;
        d.triCountLod0 = o.lods[0];
        d.triOffsetLod0 = o.lods[1];
        d.triCountLod1 = o.lods[2];
        d.triOffsetLod1 = o.lods[3];
        d.triCountLod2 = o.lods[4];
        d.triOffsetLod2 = o.lods[5];
        set_vertex_desc_bits(d.vertexDesc, o.vertex_desc);
        d.combined.resize(static_cast<uint32_t>(o.instance_count));
        std::memcpy(
            d.combined.data(),
            f,
            o.instance_count * 18 * sizeof(float)
        );
        f += o.instance_count * 18;
        obj.data.push_back(std::move(d));
    }
    obj.numData = static_cast<uint32_t>(objects.size());
    return true;
}
} // namespace nifly_rs
