#pragma once
#include <algorithm>
#include <cmath>
#include <cstring>
#include <functional>
#include <iterator>
#include <limits>
#include <memory>
#include <set>
#include <sstream>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

#include "ExtraData.hpp"
#include "Factory.hpp"
#include "NifFile.hpp"
#include "Object3d.hpp"
#include "Particles.hpp"
#include "animation_shim.h"
#include "bhk.hpp"
#include "collision_shim.h"
#include "effects_shim.h"
#include "extra_data_shim.h"
#include "mesh_writer.h"
#include "nifly-sys/src/extra_data_types.rs.h"
#include "nifly-sys/src/nif_file.rs.h"
#include "nifly-sys/src/object_3d.rs.h"
#include "nifly-sys/src/read_bridge.rs.h"
#include "read_bridge.h"
#include "rust/cxx.h"
#include "sort_state.h"
#include "texturing_maps.h"
#include "transform.h"
#include "write_bridge.h"

namespace nifly_rs {
// Fallout 76 and Starfield shader types (upstream bug, read side).
//
// From stream 140 (Fallout 76 and Starfield), the games dropped Parallax (3) and
// store every later BSLightingShaderPropertyShaderType one lower: FaceTint is 3,
// SkinTint 4, ..., Eye 15. nifly keeps the old numbering internally and converts
// in BSLightingShaderProperty::Sync. Reading converts with `> 3` instead of
// `>= 3`, so stored 3 (FaceTint) is read as Parallax and every later value is
// right. This maps Parallax back to FaceTint after load; Parallax can't
// legitimately occur in these versions. Only shaders without a name are synced at
// all (named ones return early and keep type 0), so the fix only ever touches
// those.
inline void fix_read_lighting_shader_types(nifly::NifFile& file) {
    auto& hdr = file.GetHeader();
    auto version = hdr.GetVersion();
    if (version.User() != 12 || version.Stream() <= 139)
        return;
    for (uint32_t i = 0; i < hdr.GetNumBlocks(); i++) {
        auto s = hdr.GetBlock<nifly::BSLightingShaderProperty>(i);
        if (s && s->GetShaderType() == nifly::BSLSP_PARALLAX)
            s->SetShaderType(nifly::BSLSP_FACE);
    }
}

// NiSourceTexture syncs its pixel data ref, for a texture that isn't external,
// only while useInternal is false. From version 10.0.1.4 on the ref is always
// there and useInternal isn't stored, so it keeps its default true: loading
// skips the ref and misreads the rest of the block, and saving leaves it out.
// SourceTextureFactory loads those versions with useInternal false, and
// SourceTextureRestore saves them that way.
template<typename T>
class SourceTextureFactory final: public nifly::NiFactory {
  public:
    std::unique_ptr<nifly::NiObject> Create() override {
        return std::make_unique<T>();
    }

    std::unique_ptr<nifly::NiObject> Load(nifly::NiIStream& stream) override {
        auto texture = std::make_unique<T>();
        if (stream.GetVersion().File() > nifly::NiFileVersion::V10_0_1_3)
            texture->useInternal = false;
        texture->Get(stream);
        return texture;
    }
};

// Reaches NiFactoryRegister's protected registrations through a pointer to
// member; never instantiated.
struct FactoryAccess: nifly::NiFactoryRegister {
    using NiFactoryRegister::m_registrations;
};

// Swaps in SourceTextureFactory, SpotLightFactory (effects_shim.h),
// PackedSharedGeomDataFactory (extra_data_shim.h), GeometryFactory
// (mesh_writer.h), BlendInterpolatorFactory and FloatExtraDataControllerFactory
// (animation_shim.h).
inline void register_factories() {
    static const bool registered = [] {
        auto& regs =
            nifly::NiFactoryRegister::Get().*(&FactoryAccess::m_registrations);
        regs[nifly::NiSourceTexture::BlockName] =
            std::make_unique<SourceTextureFactory<nifly::NiSourceTexture>>();
        regs[nifly::NiSourceCubeMap::BlockName] =
            std::make_unique<SourceTextureFactory<nifly::NiSourceCubeMap>>();
        regs[nifly::NiSpotLight::BlockName] =
            std::make_unique<SpotLightFactory>();
        regs[nifly::BSPackedCombinedSharedGeomDataExtra::BlockName] =
            std::make_unique<PackedSharedGeomDataFactory>();
        regs[nifly::BSGeometry::BlockName] =
            std::make_unique<GeometryFactory>();
        regs[nifly::NiFloatExtraDataController::BlockName] =
            std::make_unique<FloatExtraDataControllerFactory>();
        regs[nifly::NiBlendBoolInterpolator::BlockName] = std::make_unique<
            BlendInterpolatorFactory<nifly::NiBlendBoolInterpolator>>();
        regs[nifly::NiBlendFloatInterpolator::BlockName] = std::make_unique<
            BlendInterpolatorFactory<nifly::NiBlendFloatInterpolator>>();
        regs[nifly::NiBlendPoint3Interpolator::BlockName] = std::make_unique<
            BlendInterpolatorFactory<nifly::NiBlendPoint3Interpolator>>();
        regs[nifly::NiBlendTransformInterpolator::BlockName] = std::make_unique<
            BlendInterpolatorFactory<nifly::NiBlendTransformInterpolator>>();
        return true;
    }();
    (void)registered;
}

// Sets useInternal false on every NiSourceTexture in files from 10.0.1.4 on
// (see SourceTextureFactory) and restores it when it goes out of scope, also
// if Save throws. Save may delete blocks, so textures are looked up again.
class SourceTextureRestore {
  public:
    explicit SourceTextureRestore(nifly::NifFile& file) : file(file) {
        auto& hdr = file.GetHeader();
        if (hdr.GetVersion().File() <= nifly::NiFileVersion::V10_0_1_3)
            return;
        for (uint32_t i = 0; i < hdr.GetNumBlocks(); i++)
            if (auto t = hdr.GetBlock<nifly::NiSourceTexture>(i)) {
                flags[t] = t->useInternal;
                t->useInternal = false;
            }
    }

    ~SourceTextureRestore() {
        if (flags.empty())
            return;
        auto& hdr = file.GetHeader();
        for (uint32_t i = 0; i < hdr.GetNumBlocks(); i++) {
            auto t = hdr.GetBlock<nifly::NiSourceTexture>(i);
            auto it = t ? flags.find(t) : flags.end();
            if (it != flags.end())
                t->useInternal = it->second;
        }
    }

    SourceTextureRestore(const SourceTextureRestore&) = delete;
    SourceTextureRestore& operator=(const SourceTextureRestore&) = delete;

  private:
    nifly::NifFile& file;
    std::unordered_map<const nifly::NiSourceTexture*, bool> flags;
};

// Reader errors propagate as the rust::Error thrown by the streambuf, so they
// aren't mistaken for a malformed file.
inline LoadResult
load_nif(read_bridge::RustReader& reader, LoadOptions options) {
    read_bridge::RustIStream in(reader);
    in.exceptions(std::ios::badbit);

    // Load directly into the heap allocation. NifFile has a user-declared copy
    // constructor and no move constructor, so building it on the stack and
    // handing it to make_unique would deep-clone every block.
    register_factories();
    auto nif = std::make_unique<nifly::NifFile>();
    nifly::NifLoadOptions nifly_options;
    nifly_options.isTerrain = options.is_terrain;
    int code = nif->Load(in, nifly_options);
    if (code == 0)
        fix_read_lighting_shader_types(*nif);

    LoadResult r;
    r.code = code;
    r.value = std::move(nif);

    return r;
}

// NifFile's copy constructor (CopyFrom) into a new heap allocation: clones every
// block and relinks geometry data.
inline std::unique_ptr<nifly::NifFile> clone_nif(const nifly::NifFile& file) {
    return std::make_unique<nifly::NifFile>(file);
}

// NifFile::Create on a new heap-allocated file (see load_nif for why it isn't
// built on the stack).
inline std::unique_ptr<nifly::NifFile> create_nif(NifVersion version) {
    auto nif = std::make_unique<nifly::NifFile>();
    nif->Create(
        nifly::NiVersion(
            static_cast<nifly::NiFileVersion>(version.file),
            version.user,
            version.stream
        )
    );
    return nif;
}

inline NifVersion nif_version(const nifly::NifFile& file) {
    const nifly::NiVersion& v = file.GetHeader().GetVersion();
    return NifVersion {static_cast<uint32_t>(v.File()), v.User(), v.Stream()};
}

inline std::unique_ptr<std::vector<std::string>>
names_to_ffi(std::vector<std::string>& names) {
    return std::make_unique<std::vector<std::string>>(std::move(names));
}

// NifFile::OptimizeFor. Takes the options by value since upstream wants a
// non-const reference but doesn't modify them.
inline OptimizeResult
optimize_for(nifly::NifFile& file, OptimizeOptions options) {
    nifly::OptOptions opts;
    opts.targetVersion = nifly::NiVersion(
        static_cast<nifly::NiFileVersion>(options.target.file),
        options.target.user,
        options.target.stream
    );
    opts.headParts = options.head_parts;
    opts.removeParallax = options.remove_parallax;
    opts.calcBounds = options.calc_bounds;
    opts.fixBSXFlags = options.fix_bsx_flags;
    opts.fixShaderFlags = options.fix_shader_flags;

    nifly::OptResult r = file.OptimizeFor(opts);
    return OptimizeResult {
        r.versionMismatch,
        r.dupesRenamed,
        names_to_ffi(r.shapesVColorsRemoved),
        names_to_ffi(r.shapesNormalsRemoved),
        names_to_ffi(r.shapesPartTriangulated),
        names_to_ffi(r.shapesTangentsAdded),
        names_to_ffi(r.shapesParallaxRemoved),
    };
}

// Fallout 76 and Starfield shader types (upstream bug, write side; see
// fix_read_lighting_shader_types for the numbering).
//
// Writing, BSLightingShaderProperty::Sync syncs the type before converting it:
// it writes the internal value as is, then decrements the in-memory value
// (`>= 3`), then writes the type-specific fields (the switch at the end of Sync)
// for that decremented type. Only shaders without a name are synced. For any
// type from 3 (Parallax) on:
// - the stored type is one too high (Eye reloads as Cloud);
// - the fields written belong to the type below the one set, so a reader
//   expecting the stored type's fields misreads the rest of the block; with some
//   types nifly can't load the file again (HairTint writes SkinTint's 4 floats
//   where ParallaxOcclusion's 2 are read);
// - the file being saved is changed, its types now one lower.
// It can't be worked around by pre-adjusting the in-memory type: the stored type
// and the switch see values one apart, so they can't both be right. The bad data
// is written as is and documented on the Rust side; only the in-memory change is
// undone (LightingTypeRestore).
//
// Restores the types of the lighting shaders still in the file when it goes out
// of scope, also if Save throws. Save may delete unreferenced blocks, so shaders
// are looked up again by address among the remaining blocks rather than
// dereferenced.
class LightingTypeRestore {
  public:
    explicit LightingTypeRestore(nifly::NifFile& file) : file(file) {
        auto& hdr = file.GetHeader();
        auto version = hdr.GetVersion();
        if (version.User() != 12 || version.Stream() <= 139)
            return;
        for (uint32_t i = 0; i < hdr.GetNumBlocks(); i++)
            if (auto s = hdr.GetBlock<nifly::BSLightingShaderProperty>(i))
                types[s] = s->GetShaderType();
    }

    ~LightingTypeRestore() {
        if (types.empty())
            return;
        auto& hdr = file.GetHeader();
        for (uint32_t i = 0; i < hdr.GetNumBlocks(); i++) {
            auto s = hdr.GetBlock<nifly::BSLightingShaderProperty>(i);
            auto it = s ? types.find(s) : types.end();
            if (it != types.end())
                s->SetShaderType(it->second);
        }
    }

    LightingTypeRestore(const LightingTypeRestore&) = delete;
    LightingTypeRestore& operator=(const LightingTypeRestore&) = delete;

  private:
    nifly::NifFile& file;
    std::unordered_map<const nifly::BSLightingShaderProperty*, uint32_t> types;
};

// Writing an Oblivion file clears each NiGeometryData's tangent flag (bit 12 of
// dataFlags) in memory, since Oblivion stores tangents in NiBinaryExtraData
// instead. The next save's FinalizeData then sees no tangents and deletes that
// extra data, losing the tangents and renumbering blocks. Restores the flag of
// the geometry data still in the file when it goes out of scope, also if Save
// throws; blocks are looked up again by address, as in LightingTypeRestore.
class TangentFlagRestore {
  public:
    explicit TangentFlagRestore(nifly::NifFile& file) : file(file) {
        auto& hdr = file.GetHeader();
        if (!hdr.GetVersion().IsOB())
            return;
        for (uint32_t i = 0; i < hdr.GetNumBlocks(); i++)
            if (auto d = hdr.GetBlock<nifly::NiGeometryData>(i))
                if (d->HasTangents())
                    with_tangents.insert(d);
    }

    ~TangentFlagRestore() {
        if (with_tangents.empty())
            return;
        auto& hdr = file.GetHeader();
        for (uint32_t i = 0; i < hdr.GetNumBlocks(); i++) {
            auto d = hdr.GetBlock<nifly::NiGeometryData>(i);
            if (d && with_tangents.count(d))
                d->dataFlags |= 1 << 12;
        }
    }

    TangentFlagRestore(const TangentFlagRestore&) = delete;
    TangentFlagRestore& operator=(const TangentFlagRestore&) = delete;

  private:
    nifly::NifFile& file;
    std::unordered_set<const nifly::NiGeometryData*> with_tangents;
};

// Mesh positions are saved as 16-bit fractions of `scale` (times havokScale),
// and upstream doesn't update `scale` when they change: positions outside the
// old range wrap around, and with no scale (0) no vertices are written. Widens
// it when needed, keeping it otherwise so unchanged data saves as read.
inline void fit_mesh_scale(nifly::BSGeometryMeshData& data) {
    float extent = 0.0f;
    for (const auto& v : data.vertices)
        extent =
            std::max({extent, std::abs(v.x), std::abs(v.y), std::abs(v.z)});
    // BSGeometryMeshData::havokScale, which is private.
    constexpr float havok_scale = 69.969f;
    float needed = extent / havok_scale;
    if (needed > data.scale)
        data.scale = needed;
    if (data.scale <= 0.0f && !data.vertices.empty())
        data.scale = 1.0f;
}

// Fits the scale of internal BSGeometry meshes for the save (see
// fit_mesh_scale), restoring it afterwards. Shapes are looked up again on
// restore, since optimizing may delete some.
class MeshScaleRestore {
  public:
    explicit MeshScaleRestore(nifly::NifFile& file) : file(file) {
        for (auto shape : file.GetShapes()) {
            auto g = dynamic_cast<nifly::BSGeometry*>(shape);
            if (!g || !g->HasInternalGeomData())
                continue;
            for (uint8_t i = 0; i < g->MeshCount(); i++) {
                auto& data = g->SelectMesh(i)->meshData;
                const float old = data.scale;
                if (data.version <= 2)
                    fit_mesh_scale(data);
                if (data.scale != old)
                    scales[g].emplace_back(i, old);
            }
            g->ReleaseMesh();
        }
    }

    ~MeshScaleRestore() {
        if (scales.empty())
            return;
        for (auto shape : file.GetShapes()) {
            auto g = dynamic_cast<nifly::BSGeometry*>(shape);
            auto found = scales.find(g);
            if (!g || found == scales.end())
                continue;
            for (auto [i, scale] : found->second)
                if (auto mesh = g->SelectMesh(i))
                    mesh->meshData.scale = scale;
            g->ReleaseMesh();
        }
    }

    MeshScaleRestore(const MeshScaleRestore&) = delete;
    MeshScaleRestore& operator=(const MeshScaleRestore&) = delete;

  private:
    nifly::NifFile& file;
    std::unordered_map<
        const nifly::BSGeometry*,
        std::vector<std::pair<uint8_t, float>>>
        scales;
};

// NifFile::Save through a RustOStream. Writer errors
// propagate as the rust::Error thrown by the streambuf; otherwise returns
// nifly's code (0 = success, 1 = stream not writable).
inline int32_t save_nif(
    nifly::NifFile& nif,
    write_bridge::RustWriter& writer,
    SaveOptions options
) {
    LightingTypeRestore restore(nif);
    SourceTextureRestore restore_textures(nif);
    TangentFlagRestore restore_tangents(nif);
    MeshScaleRestore restore_scales(nif);
    write_bridge::RustOStream out(writer);
    out.exceptions(std::ios::badbit);
    nifly::NifSaveOptions nifly_options;
    nifly_options.optimize = options.optimize;
    nifly_options.sortBlocks = options.sort_blocks;
    int code = nif.Save(out, nifly_options);
    out.flush();
    return code;
}

// NifFile::DeleteNode. Returns whether a node named `name` existed. Upstream
// doesn't report that, so the node is looked up first with the same
// FindBlockByName<NiNode> that DeleteNode uses.
inline bool delete_node(nifly::NifFile& file, rust::Slice<const uint8_t> name) {
    std::string c_name(reinterpret_cast<const char*>(name.data()), name.size());
    if (!file.FindBlockByName<nifly::NiNode>(c_name))
        return false;

    file.DeleteNode(c_name);
    return true;
}

// NifFile::CanDeleteNode(const std::string&). False if no node has that name.
inline bool can_delete_node_by_name(
    const nifly::NifFile& file,
    rust::Slice<const uint8_t> name
) {
    std::string c_name(reinterpret_cast<const char*>(name.data()), name.size());
    return file.CanDeleteNode(c_name);
}

// NifFile::AssignExtraData with a newly constructed T, for any NiObjectNET
// (upstream takes an NiAVObject). Sets the extra data's name, lets `set` fill
// in the value, and attaches it to the NiObjectNET at block `target`: at the
// head of its extra data chain up to 4.2.2.0, at the end of its list after.
// Returns the new block's index, or NIF_NPOS (adding nothing) if `target`
// isn't a NiObjectNET.
template<class T, class Set>
uint32_t assign_extra_data(
    nifly::NifFile& file,
    uint32_t target,
    rust::Slice<const uint8_t> name,
    Set set
) {
    auto& hdr = file.GetHeader();
    auto* obj = hdr.GetBlock<nifly::NiObjectNET>(target);
    if (!obj)
        return nifly::NIF_NPOS;

    auto data = std::make_unique<T>();
    data->name.get().assign(
        reinterpret_cast<const char*>(name.data()),
        name.size()
    );
    set(*data);
    bool chained = hdr.GetVersion().File() <= nifly::V4_2_2_0;
    if (chained)
        data->nextExtraDataRef.index = obj->extraDataRef.index;
    uint32_t id = hdr.AddBlock(std::move(data));
    if (chained)
        obj->extraDataRef.index = id;
    else
        obj->extraDataRefs.AddBlockRef(id);
    return id;
}

#define NIFLY_RS_ASSIGN(T, setter) [&](nifly::T& d) { setter; }

inline uint32_t assign_string_extra_data(
    nifly::NifFile& file,
    uint32_t target,
    rust::Slice<const uint8_t> name,
    rust::Slice<const uint8_t> value
) {
    return assign_extra_data<nifly::NiStringExtraData>(
        file,
        target,
        name,
        NIFLY_RS_ASSIGN(NiStringExtraData, set_string_extra_data(d, value))
    );
}

inline uint32_t assign_integer_extra_data(
    nifly::NifFile& file,
    uint32_t target,
    rust::Slice<const uint8_t> name,
    uint32_t value
) {
    return assign_extra_data<nifly::NiIntegerExtraData>(
        file,
        target,
        name,
        NIFLY_RS_ASSIGN(NiIntegerExtraData, set_integer_extra_data(d, value))
    );
}

inline uint32_t assign_integers_extra_data(
    nifly::NifFile& file,
    uint32_t target,
    rust::Slice<const uint8_t> name,
    rust::Slice<const uint32_t> values
) {
    return assign_extra_data<nifly::NiIntegersExtraData>(
        file,
        target,
        name,
        NIFLY_RS_ASSIGN(NiIntegersExtraData, set_integers_extra_data(d, values))
    );
}

inline uint32_t assign_float_extra_data(
    nifly::NifFile& file,
    uint32_t target,
    rust::Slice<const uint8_t> name,
    float value
) {
    return assign_extra_data<nifly::NiFloatExtraData>(
        file,
        target,
        name,
        NIFLY_RS_ASSIGN(NiFloatExtraData, set_float_extra_data(d, value))
    );
}

inline uint32_t assign_boolean_extra_data(
    nifly::NifFile& file,
    uint32_t target,
    rust::Slice<const uint8_t> name,
    bool value
) {
    return assign_extra_data<nifly::NiBooleanExtraData>(
        file,
        target,
        name,
        NIFLY_RS_ASSIGN(NiBooleanExtraData, set_boolean_extra_data(d, value))
    );
}

// For BSExtraData blocks, which AssignExtraData can't take because they aren't
// NiExtraData. Does what AssignExtraData does for file versions after 4.2.2.0:
// adds the block and appends it to the target's extra data list. Older
// versions store extra data as a linked list through NiExtraData, which these
// blocks can't be part of, so they're rejected. Returns the new block's index,
// or NIF_NPOS (adding nothing) if `target` isn't a NiObjectNET.
template<class T, class Set>
uint32_t assign_bs_extra_data(nifly::NifFile& file, uint32_t target, Set set) {
    auto& hdr = file.GetHeader();
    if (hdr.GetVersion().File() <= nifly::V4_2_2_0)
        return nifly::NIF_NPOS;

    auto* obj = hdr.GetBlock<nifly::NiObjectNET>(target);
    if (!obj)
        return nifly::NIF_NPOS;

    auto data = std::make_unique<T>();
    set(*data);
    uint32_t id = hdr.AddBlock(std::move(data));
    obj->extraDataRefs.AddBlockRef(id);
    return id;
}

inline uint32_t assign_bs_cloth_extra_data(
    nifly::NifFile& file,
    uint32_t target,
    rust::Slice<const uint8_t> bytes
) {
    return assign_bs_extra_data<nifly::BSClothExtraData>(
        file,
        target,
        NIFLY_RS_ASSIGN(BSClothExtraData, set_cloth_extra_data(d, bytes))
    );
}

inline uint32_t assign_bs_collision_query_proxy_extra_data(
    nifly::NifFile& file,
    uint32_t target,
    rust::Slice<const uint8_t> bytes
) {
    return assign_bs_extra_data<nifly::BSCollisionQueryProxyExtraData>(
        file,
        target,
        NIFLY_RS_ASSIGN(
            BSCollisionQueryProxyExtraData,
            set_collision_query_proxy_extra_data(d, bytes)
        )
    );
}

// BSXFlags is always named "BSX".
inline uint32_t
assign_bsx_flags(nifly::NifFile& file, uint32_t target, uint32_t flags) {
    static const uint8_t bsx[] = {'B', 'S', 'X'};
    return assign_extra_data<nifly::BSXFlags>(
        file,
        target,
        rust::Slice<const uint8_t>(bsx, 3),
        NIFLY_RS_ASSIGN(BSXFlags, set_integer_extra_data(d, flags))
    );
}

inline uint32_t assign_color_extra_data(
    nifly::NifFile& file,
    uint32_t target,
    rust::Slice<const uint8_t> name,
    Float4 color
) {
    return assign_extra_data<nifly::NiColorExtraData>(
        file,
        target,
        name,
        NIFLY_RS_ASSIGN(NiColorExtraData, set_color_extra_data(d, color))
    );
}

inline uint32_t assign_vector_extra_data(
    nifly::NifFile& file,
    uint32_t target,
    rust::Slice<const uint8_t> name,
    Float4 vector
) {
    return assign_extra_data<nifly::NiVectorExtraData>(
        file,
        target,
        name,
        NIFLY_RS_ASSIGN(NiVectorExtraData, set_vector_extra_data(d, vector))
    );
}

inline uint32_t assign_vert_weights_extra_data(
    nifly::NifFile& file,
    uint32_t target,
    rust::Slice<const uint8_t> name,
    rust::Slice<const float> weights
) {
    if (!vert_weights_fit(weights))
        return nifly::NIF_NPOS;
    return assign_extra_data<nifly::NiVertWeightsExtraData>(
        file,
        target,
        name,
        NIFLY_RS_ASSIGN(
            NiVertWeightsExtraData,
            set_vert_weights_extra_data(d, weights)
        )
    );
}

inline uint32_t assign_eye_center_extra_data(
    nifly::NifFile& file,
    uint32_t target,
    rust::Slice<const uint8_t> name,
    rust::Slice<const float> values
) {
    return assign_extra_data<nifly::BSEyeCenterExtraData>(
        file,
        target,
        name,
        NIFLY_RS_ASSIGN(
            BSEyeCenterExtraData,
            set_eye_center_extra_data(d, values)
        )
    );
}

inline uint32_t assign_position_data(
    nifly::NifFile& file,
    uint32_t target,
    rust::Slice<const uint8_t> name,
    rust::Slice<const float> values
) {
    return assign_extra_data<nifly::BSPositionData>(
        file,
        target,
        name,
        NIFLY_RS_ASSIGN(BSPositionData, set_position_data(d, values))
    );
}

inline uint32_t assign_w_array(
    nifly::NifFile& file,
    uint32_t target,
    rust::Slice<const uint8_t> name,
    rust::Slice<const uint32_t> values
) {
    return assign_extra_data<nifly::BSWArray>(
        file,
        target,
        name,
        NIFLY_RS_ASSIGN(BSWArray, set_w_array(d, values))
    );
}

inline uint32_t assign_decal_placement_vector_extra_data(
    nifly::NifFile& file,
    uint32_t target,
    rust::Slice<const uint8_t> name,
    float value,
    rust::Slice<const size_t> block_lengths,
    rust::Slice<const Vec3> points,
    rust::Slice<const Vec3> normals
) {
    if (!decal_blocks_fit(block_lengths, points, normals))
        return nifly::NIF_NPOS;
    return assign_extra_data<nifly::BSDecalPlacementVectorExtraData>(
        file,
        target,
        name,
        [&](nifly::BSDecalPlacementVectorExtraData& d) {
            d.floatData = value;
            set_decal_blocks(d, block_lengths, points, normals);
        }
    );
}

inline uint32_t assign_distant_object_extra_data(
    nifly::NifFile& file,
    uint32_t target,
    rust::Slice<const uint8_t> name,
    uint32_t flags
) {
    return assign_extra_data<nifly::BSDistantObjectExtraData>(
        file,
        target,
        name,
        NIFLY_RS_ASSIGN(
            BSDistantObjectExtraData,
            set_distant_object_flags(d, flags)
        )
    );
}

inline uint32_t assign_distant_object_large_ref_extra_data(
    nifly::NifFile& file,
    uint32_t target,
    rust::Slice<const uint8_t> name,
    bool large_ref
) {
    return assign_extra_data<nifly::BSDistantObjectLargeRefExtraData>(
        file,
        target,
        name,
        NIFLY_RS_ASSIGN(
            BSDistantObjectLargeRefExtraData,
            set_distant_object_large_ref(d, large_ref)
        )
    );
}

inline uint32_t assign_binary_extra_data(
    nifly::NifFile& file,
    uint32_t target,
    rust::Slice<const uint8_t> name,
    rust::Slice<const uint8_t> bytes
) {
    return assign_extra_data<nifly::NiBinaryExtraData>(
        file,
        target,
        name,
        NIFLY_RS_ASSIGN(NiBinaryExtraData, set_binary_extra_data(d, bytes))
    );
}

inline uint32_t assign_floats_extra_data(
    nifly::NifFile& file,
    uint32_t target,
    rust::Slice<const uint8_t> name,
    rust::Slice<const float> values
) {
    return assign_extra_data<nifly::NiFloatsExtraData>(
        file,
        target,
        name,
        NIFLY_RS_ASSIGN(NiFloatsExtraData, set_floats_extra_data(d, values))
    );
}

inline uint32_t assign_strings_extra_data(
    nifly::NifFile& file,
    uint32_t target,
    rust::Slice<const uint8_t> name,
    rust::Slice<const uint8_t> bytes,
    rust::Slice<const size_t> lengths
) {
    return assign_extra_data<nifly::NiStringsExtraData>(
        file,
        target,
        name,
        NIFLY_RS_ASSIGN(
            NiStringsExtraData,
            set_strings_extra_data(d, bytes, lengths)
        )
    );
}

inline uint32_t assign_packed_shared_geom(
    nifly::NifFile& file,
    uint32_t target,
    rust::Slice<const uint8_t> name,
    PackedGeomHeader header,
    rust::Slice<const PackedGeomObjectFfi> objects,
    rust::Slice<const float> instances
) {
    auto& hdr = file.GetHeader();
    if (!hdr.GetBlock<nifly::NiObjectNET>(target))
        return nifly::NIF_NPOS;
    PackedSharedGeomData data;
    if (!set_packed_shared_objects(data, objects, instances))
        return nifly::NIF_NPOS;
    return assign_extra_data<PackedSharedGeomData>(
        file,
        target,
        name,
        [&](PackedSharedGeomData& d) {
            d.objects = data.objects;
            d.data = data.data;
            d.numData = data.numData;
            set_packed_shared_header(d, header);
        }
    );
}

inline uint32_t assign_skin_attach(
    nifly::NifFile& file,
    uint32_t target,
    rust::Slice<const uint8_t> name,
    rust::Slice<const uint8_t> bytes,
    rust::Slice<const size_t> lengths
) {
    return assign_extra_data<nifly::SkinAttach>(
        file,
        target,
        name,
        NIFLY_RS_ASSIGN(SkinAttach, set_skin_attach_bones(d, bytes, lengths))
    );
}

// A BSFurnitureMarkerNode if `node` (Skyrim on), otherwise a BSFurnitureMarker.
inline uint32_t assign_furniture_marker(
    nifly::NifFile& file,
    uint32_t target,
    rust::Slice<const uint8_t> name,
    rust::Slice<const FurniturePositionFfi> positions,
    bool node
) {
    auto set = [&](nifly::BSFurnitureMarker& d) {
        set_furniture_positions(d, positions);
    };
    if (node)
        return assign_extra_data<nifly::BSFurnitureMarkerNode>(
            file,
            target,
            name,
            set
        );
    return assign_extra_data<nifly::BSFurnitureMarker>(file, target, name, set);
}

inline uint32_t assign_connect_point_parents(
    nifly::NifFile& file,
    uint32_t target,
    rust::Slice<const uint8_t> name,
    rust::Slice<const uint8_t> bytes,
    rust::Slice<const size_t> lengths,
    rust::Slice<const ConnectPointTransform> transforms
) {
    return assign_extra_data<nifly::BSConnectPointParents>(
        file,
        target,
        name,
        NIFLY_RS_ASSIGN(
            BSConnectPointParents,
            set_connect_points(d, bytes, lengths, transforms)
        )
    );
}

inline uint32_t assign_connect_point_children(
    nifly::NifFile& file,
    uint32_t target,
    rust::Slice<const uint8_t> name,
    bool skinned,
    rust::Slice<const uint8_t> bytes,
    rust::Slice<const size_t> lengths
) {
    return assign_extra_data<nifly::BSConnectPointChildren>(
        file,
        target,
        name,
        [&](nifly::BSConnectPointChildren& d) {
            set_connect_point_skinned(d, skinned);
            set_connect_point_targets(d, bytes, lengths);
        }
    );
}

inline uint32_t assign_inv_marker(
    nifly::NifFile& file,
    uint32_t target,
    rust::Slice<const uint8_t> name,
    InvMarkerFfi marker
) {
    return assign_extra_data<nifly::BSInvMarker>(
        file,
        target,
        name,
        NIFLY_RS_ASSIGN(BSInvMarker, set_inv_marker(d, marker))
    );
}

inline uint32_t assign_bs_bound(
    nifly::NifFile& file,
    uint32_t target,
    rust::Slice<const uint8_t> name,
    Vec3 center,
    Vec3 half_extents
) {
    return assign_extra_data<nifly::BSBound>(
        file,
        target,
        name,
        [&](nifly::BSBound& d) {
            set_bound_center(d, center);
            set_bound_half_extents(d, half_extents);
        }
    );
}

inline uint32_t assign_text_key_extra_data(
    nifly::NifFile& file,
    uint32_t target,
    rust::Slice<const uint8_t> name,
    rust::Slice<const uint8_t> bytes,
    rust::Slice<const size_t> lengths,
    rust::Slice<const float> times
) {
    return assign_extra_data<nifly::NiTextKeyExtraData>(
        file,
        target,
        name,
        NIFLY_RS_ASSIGN(
            NiTextKeyExtraData,
            set_text_keys(d, bytes, lengths, times)
        )
    );
}

inline uint32_t assign_bone_lod_extra_data(
    nifly::NifFile& file,
    uint32_t target,
    rust::Slice<const uint8_t> name,
    rust::Slice<const uint8_t> bytes,
    rust::Slice<const size_t> lengths,
    rust::Slice<const uint32_t> distances
) {
    return assign_extra_data<nifly::BSBoneLODExtraData>(
        file,
        target,
        name,
        NIFLY_RS_ASSIGN(
            BSBoneLODExtraData,
            set_bone_lods(d, bytes, lengths, distances)
        )
    );
}

inline uint32_t assign_bone_translations(
    nifly::NifFile& file,
    uint32_t target,
    rust::Slice<const uint8_t> name,
    rust::Slice<const uint8_t> bytes,
    rust::Slice<const size_t> lengths,
    rust::Slice<const Vec3> translations
) {
    return assign_extra_data<nifly::BoneTranslations>(
        file,
        target,
        name,
        NIFLY_RS_ASSIGN(
            BoneTranslations,
            set_bone_translations(d, bytes, lengths, translations)
        )
    );
}

inline uint32_t assign_behavior_graph_extra_data(
    nifly::NifFile& file,
    uint32_t target,
    rust::Slice<const uint8_t> name,
    rust::Slice<const uint8_t> graph_file,
    bool controls_base_skeleton
) {
    return assign_extra_data<nifly::BSBehaviorGraphExtraData>(
        file,
        target,
        name,
        [&](nifly::BSBehaviorGraphExtraData& d) {
            set_behavior_graph_file(d, graph_file);
            set_behavior_graph_controls_base_skeleton(
                d,
                controls_base_skeleton
            );
        }
    );
}

#undef NIFLY_RS_ASSIGN

// Sets the next block in the extra data chain of the NiExtraData at `data`
// (NIF_NPOS clears it). False, changing nothing, if a block has the wrong
// type, `next` is `data`, or the chain from `next` leads back to `data`.
inline bool
set_extra_data_next(nifly::NifFile& file, uint32_t data_index, uint32_t next) {
    auto& hdr = file.GetHeader();
    auto data = hdr.GetBlock<nifly::NiExtraData>(data_index);
    if (!data)
        return false;
    if (next != nifly::NIF_NPOS) {
        std::set<uint32_t> seen;
        for (uint32_t at = next; at != nifly::NIF_NPOS;) {
            auto link = hdr.GetBlock<nifly::NiExtraData>(at);
            if (!link || at == data_index)
                return false;
            if (!seen.insert(at).second)
                break;
            at = link->nextExtraDataRef.index;
        }
    }
    data->nextExtraDataRef.index = next;
    return true;
}

// NifFile::SetShapeOrder, taking shapes by block index. Upstream looks the
// names up again with FindBlockByName, so the names must be unique.
inline void
set_shape_order(nifly::NifFile& file, rust::Slice<const uint32_t> shapes) {
    std::vector<std::string> order;
    order.reserve(shapes.size());
    for (uint32_t index : shapes) {
        auto shape = file.GetHeader().GetBlock<nifly::NiShape>(index);
        if (!shape)
            return;
        order.push_back(shape->name.get());
    }
    file.SetShapeOrder(order);
}

inline std::unique_ptr<SortState> new_sort_state(const nifly::NifFile& file) {
    auto state = std::make_unique<SortState>();
    uint32_t count = file.GetHeader().GetNumBlocks();
    state->inner.newIndices.resize(count);
    for (uint32_t i = 0; i < count; i++)
        state->inner.newIndices[i] = i;
    return state;
}

inline bool sort_visit(nifly::NifFile& file, SortState& state, uint32_t index) {
    if (!file.GetHeader().GetBlock<nifly::NiObject>(index))
        return false;
    if (state.inner.visitedIndices.count(index) > 0)
        return false;
    file.SetSortIndices(index, state.inner);
    return true;
}

inline bool sort_is_visited(const SortState& state, uint32_t index) {
    return state.inner.visitedIndices.count(index) > 0;
}

// The end of PrettySortBlocks: blocks no visit reached keep their relative
// order after the visited ones.
inline void sort_apply(nifly::NifFile& file, SortState& state) {
    auto& s = state.inner;
    for (size_t i = 0; i < s.newIndices.size(); i++) {
        uint32_t index = static_cast<uint32_t>(i);
        if (s.visitedIndices.count(index) == 0) {
            s.newIndices[i] = s.newIndex++;
            s.visitedIndices.insert(index);
        }
    }
    file.GetHeader().SetBlockOrder(s.newIndices);
}

// NifFile::DeleteUnreferencedBlocks<T> and the NiHeader functions it calls,
// with the template's GetBlock<T> type check done by `matches`. Same loop as
// NiHeader::DeleteUnreferencedBlocksInternal, repeated until a pass deletes
// nothing, since deleting a block can leave the blocks it referenced unowned.
inline uint32_t delete_unreferenced_blocks(
    nifly::NifFile& file,
    rust::Fn<bool(const nifly::NiObject&)> matches
) {
    if (file.HasUnknown())
        return 0;

    auto& hdr = file.GetHeader();
    uint32_t rootId = file.GetBlockID(file.GetRootNode());
    if (rootId == nifly::NIF_NPOS)
        return 0;

    uint32_t count = 0;
    bool anyDeleted;
    do {
        anyDeleted = false;
        for (uint32_t i = 0; i < hdr.GetNumBlocks(); i++) {
            if (i == rootId)
                continue;

            auto block = hdr.GetBlock<nifly::NiObject>(i);
            if (block && matches(*block) && !hdr.IsBlockReferenced(i, false)) {
                hdr.DeleteBlock(i);
                anyDeleted = true;
                count++;
                if (rootId > i)
                    rootId--;
            }
        }
    } while (anyDeleted);

    return count;
}

// NifFile::DeleteUnreferencedNodes, with its bool result and deletion count
// folded into one value: -1 if it refused, otherwise the count.
inline int32_t delete_unreferenced_nodes(nifly::NifFile& file) {
    int count = 0;
    if (!file.DeleteUnreferencedNodes(&count))
        return -1;
    return count;
}

// NiHeader::GetBlockRefCount. It isn't const, but only reads.
inline int32_t
block_ref_count(const nifly::NifFile& file, uint32_t index, bool include_ptrs) {
    return const_cast<nifly::NiHeader&>(file.GetHeader())
        .GetBlockRefCount(index, include_ptrs);
}

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

// Gives the BSMultiBoundNode at `node_index` a new BSMultiBound holding `data`,
// replacing its ref. Returns the BSMultiBound's index, or NIF_NPOS (changing
// nothing) if that block isn't a BSMultiBoundNode.
template<typename D>
uint32_t assign_multi_bound(
    nifly::NifFile& file,
    uint32_t node_index,
    std::unique_ptr<D> data
) {
    auto& hdr = file.GetHeader();
    auto node = hdr.GetBlock<nifly::BSMultiBoundNode>(node_index);
    if (!node)
        return nifly::NIF_NPOS;
    auto bound = std::make_unique<nifly::BSMultiBound>();
    bound->dataRef.index = hdr.AddBlock(std::move(data));
    uint32_t id = hdr.AddBlock(std::move(bound));
    node->multiBoundRef.index = id;
    return id;
}

inline nifly::Vector3 multi_bound_vec3(const Vec3& v) {
    return nifly::Vector3(v.x, v.y, v.z);
}

inline uint32_t assign_multi_bound_aabb(
    nifly::NifFile& file,
    uint32_t node_index,
    Vec3 center,
    Vec3 half_extent
) {
    auto data = std::make_unique<nifly::BSMultiBoundAABB>();
    data->center = multi_bound_vec3(center);
    data->halfExtent = multi_bound_vec3(half_extent);
    return assign_multi_bound(file, node_index, std::move(data));
}

// `rotation` has 3 rows.
inline uint32_t assign_multi_bound_obb(
    nifly::NifFile& file,
    uint32_t node_index,
    Vec3 center,
    Vec3 size,
    rust::Slice<const Vec3> rotation
) {
    auto data = std::make_unique<nifly::BSMultiBoundOBB>();
    data->center = multi_bound_vec3(center);
    data->size = multi_bound_vec3(size);
    for (int r = 0; r < 3; r++)
        data->rotation[r] = multi_bound_vec3(rotation[r]);
    return assign_multi_bound(file, node_index, std::move(data));
}

inline uint32_t assign_multi_bound_sphere(
    nifly::NifFile& file,
    uint32_t node_index,
    Vec3 center,
    float radius
) {
    auto data = std::make_unique<nifly::BSMultiBoundSphere>();
    data->center = multi_bound_vec3(center);
    data->radius = radius;
    return assign_multi_bound(file, node_index, std::move(data));
}

// Gives the NiLODNode at `node_index` a new default T as its LOD data,
// replacing its ref. Returns the new block's index, or NIF_NPOS (changing
// nothing) if that block isn't an NiLODNode.
template<typename T>
uint32_t assign_lod_data(nifly::NifFile& file, uint32_t node_index) {
    auto& hdr = file.GetHeader();
    auto node = hdr.GetBlock<nifly::NiLODNode>(node_index);
    if (!node)
        return nifly::NIF_NPOS;
    uint32_t id = hdr.AddBlock(std::make_unique<T>());
    node->lodLevelData.index = id;
    return id;
}

inline uint32_t
assign_range_lod_data(nifly::NifFile& file, uint32_t node_index) {
    return assign_lod_data<nifly::NiRangeLODData>(file, node_index);
}

inline uint32_t
assign_screen_lod_data(nifly::NifFile& file, uint32_t node_index) {
    return assign_lod_data<nifly::NiScreenLODData>(file, node_index);
}

// Clears the LOD data ref of the NiLODNode at `node_index`. False if that
// block isn't one or has no LOD data.
inline bool remove_lod_data(nifly::NifFile& file, uint32_t node_index) {
    auto node = file.GetHeader().GetBlock<nifly::NiLODNode>(node_index);
    if (!node || node->lodLevelData.IsEmpty())
        return false;
    node->lodLevelData.Clear();
    return true;
}

template<typename T>
uint32_t
new_additional_data(nifly::NifFile& file, nifly::NiGeometryData& data) {
    auto block = std::make_unique<T>();
    block->numVertices = data.GetNumVertices();
    uint32_t id = file.GetHeader().AddBlock(std::move(block));
    data.additionalDataRef.index = id;
    return id;
}

inline uint32_t assign_additional_geometry_data(
    nifly::NifFile& file,
    uint32_t data_index,
    bool packed
) {
    auto data = file.GetHeader().GetBlock<nifly::NiGeometryData>(data_index);
    if (!data)
        return nifly::NIF_NPOS;
    if (packed)
        return new_additional_data<nifly::BSPackedAdditionalGeometryData>(
            file,
            *data
        );
    return new_additional_data<nifly::NiAdditionalGeometryData>(file, *data);
}

inline bool
remove_additional_geometry_data(nifly::NifFile& file, uint32_t data_index) {
    auto data = file.GetHeader().GetBlock<nifly::NiGeometryData>(data_index);
    if (!data || data->additionalDataRef.IsEmpty())
        return false;
    data->additionalDataRef.Clear();
    return true;
}

// Clears the multi-bound ref of the BSMultiBoundNode at `node_index`. False if
// that block isn't one or has no multi-bound.
inline bool remove_multi_bound(nifly::NifFile& file, uint32_t node_index) {
    auto node = file.GetHeader().GetBlock<nifly::BSMultiBoundNode>(node_index);
    if (!node || node->multiBoundRef.IsEmpty())
        return false;
    node->multiBoundRef.Clear();
    return true;
}

// NifFile::GetStencilProperty. It takes a non-const shape but only reads it.
inline const nifly::NiStencilProperty*
stencil_property(const nifly::NifFile& file, const nifly::NiShape& shape) {
    return file.GetStencilProperty(const_cast<nifly::NiShape*>(&shape));
}

inline Vec3 vec3_to_ffi(const nifly::Vector3& v) {
    return Vec3 {v.x, v.y, v.z};
}

// Copies a std::vector into `out` through `convert`, returning `ok`.
template<typename T, typename U, typename F>
bool copy_out(bool ok, const std::vector<T>& in, rust::Vec<U>& out, F convert) {
    if (!ok)
        return false;
    out.reserve(in.size());
    for (const auto& x : in)
        out.push_back(convert(x));
    return true;
}

// NifFile::Get*ForShape (the copying overloads) for each kind of vertex data.
// They take a non-const shape but only read it.
inline bool shape_vertices(
    const nifly::NifFile& file,
    const nifly::NiShape& shape,
    rust::Vec<Vec3>& out
) {
    std::vector<nifly::Vector3> v;
    bool ok = file.GetVertsForShape(const_cast<nifly::NiShape*>(&shape), v);
    return copy_out(ok, v, out, vec3_to_ffi);
}

// GetNormalsForShape has no copying overload, and the pointer one refreshes a
// BSTriShape's cache of decoded normals. This decodes them the same way
// without writing to the shape.
inline bool read_normals(
    const nifly::NifFile& file,
    const nifly::NiShape& shape,
    std::vector<nifly::Vector3>& out
) {
    if (!shape.HasNormals())
        return false;
    auto s = const_cast<nifly::NiShape*>(&shape);
    if (auto data = file.GetGeometryData(s)) {
        out = data->normals;
        return true;
    }
    auto tri = dynamic_cast<const nifly::BSTriShape*>(&shape);
    if (!tri)
        return false;
    auto unpack = [](uint8_t c) { return c / 255.0f * 2.0f - 1.0f; };
    out.reserve(tri->vertData.size());
    for (const auto& v : tri->vertData)
        out.emplace_back(
            unpack(v.normal[0]),
            unpack(v.normal[1]),
            unpack(v.normal[2])
        );
    return true;
}

inline bool shape_normals(
    const nifly::NifFile& file,
    const nifly::NiShape& shape,
    rust::Vec<Vec3>& out
) {
    std::vector<nifly::Vector3> v;
    bool ok = read_normals(file, shape, v);
    return copy_out(ok, v, out, vec3_to_ffi);
}

inline bool shape_uvs(
    const nifly::NifFile& file,
    const nifly::NiShape& shape,
    rust::Vec<Uv>& out
) {
    std::vector<nifly::Vector2> v;
    bool ok = file.GetUvsForShape(const_cast<nifly::NiShape*>(&shape), v);
    return copy_out(ok, v, out, [](const nifly::Vector2& x) {
        return Uv {x.u, x.v};
    });
}

inline bool shape_colors(
    const nifly::NifFile& file,
    const nifly::NiShape& shape,
    rust::Vec<Color>& out
) {
    std::vector<nifly::Color4> v;
    bool ok = file.GetColorsForShape(const_cast<nifly::NiShape*>(&shape), v);
    return copy_out(ok, v, out, [](const nifly::Color4& c) {
        return Color {c.r, c.g, c.b, c.a};
    });
}

inline bool shape_tangents(
    const nifly::NifFile& file,
    const nifly::NiShape& shape,
    rust::Vec<Vec3>& out
) {
    std::vector<nifly::Vector3> v;
    bool ok = file.GetTangentsForShape(const_cast<nifly::NiShape*>(&shape), v);
    return copy_out(ok, v, out, vec3_to_ffi);
}

inline bool shape_bitangents(
    const nifly::NifFile& file,
    const nifly::NiShape& shape,
    rust::Vec<Vec3>& out
) {
    std::vector<nifly::Vector3> v;
    bool ok =
        file.GetBitangentsForShape(const_cast<nifly::NiShape*>(&shape), v);
    return copy_out(ok, v, out, vec3_to_ffi);
}

// GetEyeDataForShape (copying) is non-const but only reads.
inline bool shape_eye_data(
    const nifly::NifFile& file,
    const nifly::NiShape& shape,
    rust::Vec<float>& out
) {
    std::vector<float> v;
    bool ok = const_cast<nifly::NifFile&>(file).GetEyeDataForShape(
        const_cast<nifly::NiShape*>(&shape),
        v
    );
    return copy_out(ok, v, out, [](float x) { return x; });
}

// The vertex count of the shape at `shape_index` if it stores vertex data the
// Set*ForShape setters write: geometry data, or a BSTriShape's own vertices.
// -1 otherwise.
inline int32_t
settable_vertex_count(nifly::NifFile& file, uint32_t shape_index) {
    auto shape = file.GetHeader().GetBlock<nifly::NiShape>(shape_index);
    if (!shape)
        return -1;
    if (auto data = file.GetGeometryData(shape))
        return data->GetNumVertices();
    if (auto tri = dynamic_cast<nifly::BSTriShape*>(shape))
        return tri->GetNumVertices();
    return -1;
}

// Converts `in` and calls `set` with the shape, unless the shape has no
// settable vertex data or `in` doesn't have one value per vertex. Upstream
// rebuilds the geometry (dropping its triangles) when the vertex count differs,
// or copies mismatched arrays as they are.
template<typename In, typename Out, typename Convert, typename Set>
bool set_vertex_array(
    nifly::NifFile& file,
    uint32_t shape_index,
    rust::Slice<const In> in,
    Convert convert,
    Set set
) {
    int32_t count = settable_vertex_count(file, shape_index);
    if (count < 0 || in.size() != static_cast<size_t>(count))
        return false;
    std::vector<Out> values;
    values.reserve(in.size());
    for (const auto& x : in)
        values.push_back(convert(x));
    set(file.GetHeader().GetBlock<nifly::NiShape>(shape_index), values);
    return true;
}

inline nifly::Vector3 vec3_from_ffi(const Vec3& v) {
    return nifly::Vector3(v.x, v.y, v.z);
}

inline bool set_shape_vertices(
    nifly::NifFile& file,
    uint32_t shape,
    rust::Slice<const Vec3> values
) {
    return set_vertex_array<Vec3, nifly::Vector3>(
        file,
        shape,
        values,
        vec3_from_ffi,
        [&](nifly::NiShape* s, const std::vector<nifly::Vector3>& v) {
            file.SetVertsForShape(s, v);
        }
    );
}

inline bool set_shape_normals(
    nifly::NifFile& file,
    uint32_t shape,
    rust::Slice<const Vec3> values
) {
    return set_vertex_array<Vec3, nifly::Vector3>(
        file,
        shape,
        values,
        vec3_from_ffi,
        [&](nifly::NiShape* s, const std::vector<nifly::Vector3>& v) {
            file.SetNormalsForShape(s, v);
        }
    );
}

inline bool set_shape_uvs(
    nifly::NifFile& file,
    uint32_t shape,
    rust::Slice<const Uv> values
) {
    return set_vertex_array<Uv, nifly::Vector2>(
        file,
        shape,
        values,
        [](const Uv& uv) { return nifly::Vector2(uv.u, uv.v); },
        [&](nifly::NiShape* s, const std::vector<nifly::Vector2>& v) {
            file.SetUvsForShape(s, v);
        }
    );
}

inline bool set_shape_colors(
    nifly::NifFile& file,
    uint32_t shape,
    rust::Slice<const Color> values
) {
    return set_vertex_array<Color, nifly::Color4>(
        file,
        shape,
        values,
        [](const Color& c) { return nifly::Color4(c.r, c.g, c.b, c.a); },
        [&](nifly::NiShape* s, const std::vector<nifly::Color4>& v) {
            file.SetColorsForShape(s, v);
        }
    );
}

inline bool set_shape_tangents(
    nifly::NifFile& file,
    uint32_t shape,
    rust::Slice<const Vec3> values
) {
    return set_vertex_array<Vec3, nifly::Vector3>(
        file,
        shape,
        values,
        vec3_from_ffi,
        [&](nifly::NiShape* s, const std::vector<nifly::Vector3>& v) {
            file.SetTangentsForShape(s, v);
        }
    );
}

inline bool set_shape_bitangents(
    nifly::NifFile& file,
    uint32_t shape,
    rust::Slice<const Vec3> values
) {
    return set_vertex_array<Vec3, nifly::Vector3>(
        file,
        shape,
        values,
        vec3_from_ffi,
        [&](nifly::NiShape* s, const std::vector<nifly::Vector3>& v) {
            file.SetBitangentsForShape(s, v);
        }
    );
}

// Eye data only exists on BSTriShape.
inline bool set_shape_eye_data(
    nifly::NifFile& file,
    uint32_t shape,
    rust::Slice<const float> values
) {
    auto block = file.GetHeader().GetBlock<nifly::BSTriShape>(shape);
    if (!block)
        return false;
    return set_vertex_array<float, float>(
        file,
        shape,
        values,
        [](float x) { return x; },
        [&](nifly::NiShape* s, const std::vector<float>& v) {
            file.SetEyeDataForShape(s, v);
        }
    );
}

// NiShape::GetTriangles, which strips convert to triangles.
inline bool
shape_triangles(const nifly::NiShape& shape, rust::Vec<Triangle>& out) {
    std::vector<nifly::Triangle> tris;
    if (!shape.GetTriangles(tris))
        return false;
    out.reserve(tris.size());
    for (const auto& t : tris)
        out.push_back(Triangle {t.p1, t.p2, t.p3});
    return true;
}

// NiShape::SetTriangles, which upstream ignores for strips, lines and other
// data without a triangle list. Also rejects indices past the vertex count and
// more triangles than the shape can save: NiTriShapeData counts them in 16
// bits; BSTriShape and BSGeometry as GetTriangleLimit says.
inline bool set_shape_triangles(
    nifly::NifFile& file,
    uint32_t shape_index,
    rust::Slice<const Triangle> triangles
) {
    auto shape = file.GetHeader().GetBlock<nifly::NiShape>(shape_index);
    if (!shape)
        return false;

    size_t limit = file.GetTriangleLimit();
    uint32_t vertices = 0;
    auto data = file.GetGeometryData(shape);
    if (dynamic_cast<nifly::NiTriShapeData*>(data)) {
        limit = std::numeric_limits<uint16_t>::max();
        vertices = data->GetNumVertices();
    } else if (dynamic_cast<nifly::BSGeometryMeshData*>(data))
        vertices = data->GetNumVertices();
    else if (auto tri = dynamic_cast<nifly::BSTriShape*>(shape); tri && !data)
        vertices = tri->GetNumVertices();
    else
        return false;

    if (triangles.size() > limit)
        return false;
    std::vector<nifly::Triangle> tris;
    tris.reserve(triangles.size());
    for (const auto& t : triangles) {
        if (t.p1 >= vertices || t.p2 >= vertices || t.p3 >= vertices)
            return false;
        tris.emplace_back(t.p1, t.p2, t.p3);
    }
    shape->SetTriangles(tris);
    return true;
}

// NifFile::CalcNormalsForShape on the shape at `shape_index`, which returns
// nothing. Returns whether normals were calculated. Upstream only recalculates
// normals NiTriShapeData and NiTriStripsData already have, and for other
// geometry data (lines, BSGeometry meshes) just turns them on, zeroed; this
// turns them on for the former first and rejects the latter. It also skips
// Skyrim model-space shaders unless `force`.
inline bool calc_shape_normals(
    nifly::NifFile& file,
    uint32_t shape_index,
    bool smooth,
    float smooth_threshold,
    bool force
) {
    auto& hdr = file.GetHeader();
    auto shape = hdr.GetBlock<nifly::NiShape>(shape_index);
    if (!shape)
        return false;

    auto data = file.GetGeometryData(shape);
    if (data) {
        if (!dynamic_cast<nifly::NiTriShapeData*>(data)
            && !dynamic_cast<nifly::NiTriStripsData*>(data))
            return false;
    } else if (!dynamic_cast<nifly::BSTriShape*>(shape))
        return false;

    auto version = hdr.GetVersion();
    if ((version.IsSK() || version.IsSSE()) && !force) {
        auto shader = file.GetShader(shape);
        if (shader && shader->IsModelSpace())
            return false;
    }

    if (data && !data->HasNormals())
        data->SetNormals(true);
    file.CalcNormalsForShape(shape, force, smooth, smooth_threshold);
    return true;
}

// NifFile::CalcTangentsForShape on the shape at `shape_index`, which returns
// nothing. Returns whether tangents were calculated. Upstream does nothing
// without normals and UVs, and for other geometry data (lines, BSGeometry
// meshes) only turns tangents on, zeroed; both are rejected here.
inline bool calc_shape_tangents(nifly::NifFile& file, uint32_t shape_index) {
    auto shape = file.GetHeader().GetBlock<nifly::NiShape>(shape_index);
    if (!shape)
        return false;

    if (auto data = file.GetGeometryData(shape)) {
        if (!dynamic_cast<nifly::NiTriShapeData*>(data)
            && !dynamic_cast<nifly::NiTriStripsData*>(data))
            return false;
        if (!data->HasNormals() || !data->HasUVs())
            return false;
    } else if (auto tri = dynamic_cast<nifly::BSTriShape*>(shape)) {
        if (!tri->HasNormals() || !tri->HasUVs())
            return false;
    } else
        return false;

    file.CalcTangentsForShape(shape);
    return true;
}

// NifFile::ApplyNormalsFromFile, reimplemented on a shape index and a source
// shape rather than a name looked up in both files. Upstream deletes the
// target's old LOCKEDNORM blocks even when other blocks share them, and reads
// the source normals through a getter that writes to a cache. Returns false,
// changing nothing, unless the source shape has LOCKEDNORM indices, both shapes
// have normals, and they have the same number.
inline bool apply_locked_normals(
    nifly::NifFile& file,
    uint32_t shape_index,
    const nifly::NifFile& source,
    const nifly::NiShape& source_shape
) {
    auto& hdr = file.GetHeader();
    auto shape = hdr.GetBlock<nifly::NiShape>(shape_index);
    if (!shape)
        return false;

    // Upstream unions every LOCKEDNORM's indices and copies the last block.
    auto& src_hdr = const_cast<nifly::NifFile&>(source).GetHeader();
    const nifly::NiIntegersExtraData* locked_data = nullptr;
    std::unordered_set<uint32_t> locked;
    // NiBlockRefArray has no const iteration; this only reads.
    auto& src_refs = const_cast<nifly::NiShape&>(source_shape).extraDataRefs;
    for (auto& ref : src_refs) {
        auto data = src_hdr.GetBlock<nifly::NiIntegersExtraData>(ref);
        if (data && data->name == "LOCKEDNORM") {
            locked_data = data;
            for (auto i : data->integersData)
                locked.insert(i);
        }
    }
    if (!locked_data || locked.empty())
        return false;

    std::vector<nifly::Vector3> normals, src_normals;
    if (!read_normals(file, *shape, normals)
        || !read_normals(source, source_shape, src_normals)
        || normals.size() != src_normals.size())
        return false;
    for (auto i : locked)
        if (i < normals.size())
            normals[i] = src_normals[i];
    file.SetNormalsForShape(shape, normals);

    // Replace the shape's LOCKEDNORM blocks, deleting only unshared ones.
    std::vector<uint32_t> ids;
    for (uint32_t i = 0; i < shape->extraDataRefs.GetSize();) {
        uint32_t id = shape->extraDataRefs.GetBlockRef(i);
        auto data = hdr.GetBlock<nifly::NiIntegersExtraData>(id);
        if (data && data->name == "LOCKEDNORM") {
            ids.push_back(id);
            shape->extraDataRefs.RemoveBlockRef(i);
        } else
            i++;
    }
    std::sort(ids.begin(), ids.end(), std::greater<>());
    ids.erase(std::unique(ids.begin(), ids.end()), ids.end());
    for (uint32_t id : ids)
        if (hdr.GetBlockRefCount(id, false) == 0)
            hdr.DeleteBlock(id);

    file.AssignExtraData(shape, locked_data->Clone());
    return true;
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

// NifFile::GetBlockID: the block index of `obj`, or NIF_NPOS if it isn't one
// of `file`'s blocks. Linear search. GetBlockID takes a non-const pointer but
// only compares addresses, so the const_cast is safe.
inline uint32_t
block_index(const nifly::NifFile& file, const nifly::NiObject& obj) {
    return file.GetBlockID(const_cast<nifly::NiObject*>(&obj));
}

inline bool
contains_block(const nifly::NifFile& file, const nifly::NiObject& obj) {
    return block_index(file, obj) != nifly::NIF_NPOS;
}

// The object at `obj`, mutable, reached through the file borrowed mutably. Rust
// finds objects with the shared lookups, then pins the pointer this returns, so
// a pinned `&mut` always comes from a mutable borrow of the file rather than a
// shared reference. `obj` must be a block of `file`, or an object inside one (a
// BSGeometry's mesh data); the Rust caller guarantees it, so it isn't searched
// for.
inline nifly::NiObject*
object_mut(nifly::NifFile& file, const nifly::NiObject* obj) {
    (void)file;
    return const_cast<nifly::NiObject*>(obj);
}

// NifFile::AddNode. Adds a NiNode named `name` with `transform` (to parent)
// as the last child of the node at block `parent_index`, or of the root node
// if `parent_index` is NIF_NPOS. Returns the new node's block index, or
// NIF_NPOS if the parent isn't a node (or there is no root node).
inline uint32_t add_node(
    nifly::NifFile& file,
    rust::Slice<const uint8_t> name,
    Transform transform,
    uint32_t parent_index
) {
    nifly::NiNode* parent = nullptr;
    if (parent_index != nifly::NIF_NPOS) {
        parent = file.GetHeader().GetBlock<nifly::NiNode>(parent_index);
        if (!parent)
            return nifly::NIF_NPOS;
    }

    std::string c_name(reinterpret_cast<const char*>(name.data()), name.size());
    nifly::NiNode* node = file.AddNode(c_name, from_ffi(transform), parent);
    return node ? file.GetBlockID(node) : nifly::NIF_NPOS;
}

// add_node for a node subclass T, as NifFile::AddNode does for NiNode.
template<typename T>
uint32_t add_node_of(
    nifly::NifFile& file,
    rust::Slice<const uint8_t> name,
    Transform transform,
    uint32_t parent_index
) {
    nifly::NiNode* parent = parent_index == nifly::NIF_NPOS
        ? file.GetRootNode()
        : file.GetHeader().GetBlock<nifly::NiNode>(parent_index);
    if (!parent)
        return nifly::NIF_NPOS;

    auto node = std::make_unique<T>();
    node->name.get().assign(
        reinterpret_cast<const char*>(name.data()),
        name.size()
    );
    node->SetTransformToParent(from_ffi(transform));
    uint32_t id = file.GetHeader().AddBlock(std::move(node));
    parent->childRefs.AddBlockRef(id);
    return id;
}

#define NIFLY_RS_ADD_NODE(T, snake) \
    inline uint32_t add_##snake( \
        nifly::NifFile& file, \
        rust::Slice<const uint8_t> name, \
        Transform transform, \
        uint32_t parent_index \
    ) { \
        return add_node_of<nifly::T>(file, name, transform, parent_index); \
    }
NIFLY_RS_ADD_NODE(BSFadeNode, bs_fade_node)
NIFLY_RS_ADD_NODE(RootCollisionNode, root_collision_node)
NIFLY_RS_ADD_NODE(AvoidNode, avoid_node)
NIFLY_RS_ADD_NODE(NiBSAnimationNode, ni_bs_animation_node)
NIFLY_RS_ADD_NODE(NiBSParticleNode, ni_bs_particle_node)
NIFLY_RS_ADD_NODE(BSValueNode, bs_value_node)
NIFLY_RS_ADD_NODE(BSLeafAnimNode, bs_leaf_anim_node)
NIFLY_RS_ADD_NODE(BSTreeNode, bs_tree_node)
NIFLY_RS_ADD_NODE(BSOrderedNode, bs_ordered_node)
NIFLY_RS_ADD_NODE(BSMultiBoundNode, bs_multi_bound_node)
NIFLY_RS_ADD_NODE(
    BSDistantObjectInstancedNode,
    bs_distant_object_instanced_node
)
NIFLY_RS_ADD_NODE(BSRangeNode, bs_range_node)
NIFLY_RS_ADD_NODE(BSDebrisNode, bs_debris_node)
NIFLY_RS_ADD_NODE(BSBlastNode, bs_blast_node)
NIFLY_RS_ADD_NODE(BSDamageStage, bs_damage_stage)
NIFLY_RS_ADD_NODE(BSWeakReferenceNode, bs_weak_reference_node)
NIFLY_RS_ADD_NODE(BSFaceGenNiNode, bs_face_gen_ni_node)
NIFLY_RS_ADD_NODE(NiBillboardNode, ni_billboard_node)
NIFLY_RS_ADD_NODE(NiSwitchNode, ni_switch_node)
NIFLY_RS_ADD_NODE(NiLODNode, ni_lod_node)
NIFLY_RS_ADD_NODE(NiBone, ni_bone)
NIFLY_RS_ADD_NODE(NiSortAdjustNode, ni_sort_adjust_node)
NIFLY_RS_ADD_NODE(BSMasterParticleSystem, bs_master_particle_system)
#undef NIFLY_RS_ADD_NODE

inline size_t block_count(const nifly::NifFile& file) {
    return file.GetHeader().GetNumBlocks();
}

// Null if `index` is out of range.
inline const nifly::NiObject* block(const nifly::NifFile& file, size_t index) {
    if (index >= file.GetHeader().GetNumBlocks())
        return nullptr;

    return file.GetHeader().GetBlock<nifly::NiObject>(
        static_cast<uint32_t>(index)
    );
}

// Only the extraDataRefs list (file versions 10.0.1.0 and later), like
// NifFile::GetChildren. The extraDataRef chain from older versions isn't followed.
// Block indices, in order. NIF_NPOS for empty refs.
inline rust::Vec<uint32_t> extra_data_indices(const nifly::NiObjectNET& obj) {
    rust::Vec<uint32_t> out;
    out.reserve(obj.extraDataRefs.GetSize());
    for (uint32_t i = 0; i < obj.extraDataRefs.GetSize(); i++)
        out.push_back(obj.extraDataRefs.GetBlockRef(i));
    return out;
}

// NifFile::SetNodeTransformToParent. Returns whether a node had that name.
inline bool set_node_transform_to_parent(
    nifly::NifFile& file,
    rust::Slice<const uint8_t> name,
    Transform transform,
    bool root_children_only
) {
    std::string c_name(reinterpret_cast<const char*>(name.data()), name.size());
    return file.SetNodeTransformToParent(
        c_name,
        from_ffi(transform),
        root_children_only
    );
}

// NifFile::GetNodeTransformToGlobal. Returns false, leaving `out` unchanged,
// if no node has that name.
// NifFile::CalcShapeTransformGlobalToSkin. Takes a non-const shape but only
// reads it.
inline bool calc_shape_transform_global_to_skin(
    const nifly::NifFile& file,
    const nifly::NiShape& shape,
    Transform& out
) {
    nifly::MatTransform xform;
    if (!file.CalcShapeTransformGlobalToSkin(
            const_cast<nifly::NiShape*>(&shape),
            xform
        ))
        return false;

    out = to_ffi(xform);
    return true;
}

// NifFile::GetShapeTransformGlobalToSkin. Takes a non-const shape but only
// reads it.
inline bool shape_transform_global_to_skin(
    const nifly::NifFile& file,
    const nifly::NiShape& shape,
    Transform& out
) {
    nifly::MatTransform xform;
    if (!file.GetShapeTransformGlobalToSkin(
            const_cast<nifly::NiShape*>(&shape),
            xform
        ))
        return false;

    out = to_ffi(xform);
    return true;
}

// Sets the skeleton root of the NiSkinInstance (to an NiNode) or BSSkinInstance
// (to an NiAVObject) at `skin_index`. False, changing nothing, if either block
// is missing or of the wrong type.
inline bool
set_skin_root(nifly::NifFile& file, uint32_t skin_index, uint32_t root_index) {
    auto& hdr = file.GetHeader();
    if (auto inst = hdr.GetBlock<nifly::NiSkinInstance>(skin_index)) {
        if (!hdr.GetBlock<nifly::NiNode>(root_index))
            return false;
        inst->targetRef.index = root_index;
        return true;
    }
    if (auto inst = hdr.GetBlock<nifly::BSSkinInstance>(skin_index)) {
        if (!hdr.GetBlock<nifly::NiAVObject>(root_index))
            return false;
        inst->targetRef.index = root_index;
        return true;
    }
    return false;
}

// NifFile::SetShapeTransformSkinToBone, which doesn't report failure. The
// getter has the same conditions (skin data found, index in range), so it
// decides whether the set happens.
inline bool set_shape_transform_skin_to_bone(
    nifly::NifFile& file,
    uint32_t shape_index,
    uint32_t bone_index,
    Transform transform
) {
    auto shape = file.GetHeader().GetBlock<nifly::NiShape>(shape_index);
    nifly::MatTransform current;
    if (!shape || !file.GetShapeTransformSkinToBone(shape, bone_index, current))
        return false;
    file.SetShapeTransformSkinToBone(shape, bone_index, from_ffi(transform));
    return true;
}

// NifFile::GetShapeBoneBounds. Takes a non-const shape but only reads it.
inline bool shape_bone_bounds(
    const nifly::NifFile& file,
    const nifly::NiShape& shape,
    uint32_t bone_index,
    BoundingSphere& out
) {
    nifly::BoundingSphere bounds;
    if (!file.GetShapeBoneBounds(
            const_cast<nifly::NiShape*>(&shape),
            bone_index,
            bounds
        ))
        return false;

    out = to_ffi(bounds);
    return true;
}

// NifFile::SetShapeBoneBounds, but on the shape at `shape_index`. Upstream finds
// the shape by name, which picks the first shape with that name when names
// repeat. Same logic otherwise: BSSkin::BoneData first, then NiSkinData.
inline bool set_shape_bone_bounds(
    nifly::NifFile& file,
    uint32_t shape_index,
    uint32_t bone_index,
    BoundingSphere bounds
) {
    auto& hdr = file.GetHeader();
    auto shape = hdr.GetBlock<nifly::NiShape>(shape_index);
    if (!shape)
        return false;

    if (auto bsSkin =
            hdr.GetBlock<nifly::BSSkinInstance>(shape->SkinInstanceRef())) {
        auto data = hdr.GetBlock(bsSkin->dataRef);
        if (!data || bone_index >= data->nBones)
            return false;
        data->boneXforms[bone_index].bounds = from_ffi(bounds);
        return true;
    }

    auto skinInst =
        hdr.GetBlock<nifly::NiSkinInstance>(shape->SkinInstanceRef());
    if (!skinInst)
        return false;
    auto data = hdr.GetBlock(skinInst->dataRef);
    if (!data || bone_index >= data->numBones)
        return false;
    data->bones[bone_index].bounds = from_ffi(bounds);
    return true;
}

// NifFile::GetShapeTransformSkinToBone (bone index). Takes a non-const shape
// but only reads it.
inline bool shape_transform_skin_to_bone(
    const nifly::NifFile& file,
    const nifly::NiShape& shape,
    uint32_t bone_index,
    Transform& out
) {
    nifly::MatTransform xform;
    if (!file.GetShapeTransformSkinToBone(
            const_cast<nifly::NiShape*>(&shape),
            bone_index,
            xform
        ))
        return false;

    out = to_ffi(xform);
    return true;
}

inline bool node_transform_to_global(
    const nifly::NifFile& file,
    rust::Slice<const uint8_t> name,
    Transform& out
) {
    std::string c_name(reinterpret_cast<const char*>(name.data()), name.size());
    nifly::MatTransform xform;
    if (!file.GetNodeTransformToGlobal(c_name, xform))
        return false;

    out = to_ffi(xform);
    return true;
}

// NifFile::IsSSECompatible(NiShape*): false if the shape is NiTriStrips or its
// skin partition has strips. Takes a non-const pointer but only reads.
inline bool is_shape_sse_compatible(
    const nifly::NifFile& file,
    const nifly::NiShape& shape
) {
    return file.IsSSECompatible(const_cast<nifly::NiShape*>(&shape));
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

// NifFile::GetShapeBoneList. Names of the bone nodes used by the shape's skin
// instance, or from its SkinAttach extra data (Starfield). Empty if the shape
// isn't skinned. GetShapeBoneList takes a non-const pointer but only reads.
inline std::unique_ptr<std::vector<std::string>>
shape_bone_list(const nifly::NifFile& file, const nifly::NiShape& shape) {
    auto out = std::make_unique<std::vector<std::string>>();
    file.GetShapeBoneList(const_cast<nifly::NiShape*>(&shape), *out);
    return out;
}

// NifFile::GetShapeBoneIDList. Block indices of the bone nodes in the shape's
// skin instance, skipping empty refs. Empty if the shape isn't skinned. Unlike
// GetShapeBoneList, this doesn't read Starfield SkinAttach names.
// GetShapeBoneIDList takes a non-const pointer but only reads.
inline rust::Vec<uint32_t>
shape_bone_ids(const nifly::NifFile& file, const nifly::NiShape& shape) {
    std::vector<int> ids;
    file.GetShapeBoneIDList(const_cast<nifly::NiShape*>(&shape), ids);

    rust::Vec<uint32_t> out;
    out.reserve(ids.size());
    for (int id : ids)
        out.push_back(static_cast<uint32_t>(id));
    return out;
}

// NifFile::SetShapeVertWeights, reimplemented on the shape at `shape_index`:
// upstream looks the shape up by name, which finds the wrong one when names
// repeat. Upstream also silently ignores shapes without
// per-vertex weights and out-of-range vertices, and truncates extra weights
// after normalizing over all of them. This returns false, changing nothing,
// unless the shape is a skinned BSTriShape or a BSGeometry mesh with weights,
// the vertex exists, there are at most as many weights as the vertex holds,
// every bone is a position in the skin instance's bone list that fits the
// stored size (a byte for BSTriShape, 16 bits for BSGeometry), and every
// weight is finite and not negative.
inline bool set_shape_vertex_weights(
    nifly::NifFile& file,
    uint32_t shape_index,
    uint32_t vertex,
    rust::Slice<const uint32_t> bones,
    rust::Slice<const float> weights
) {
    auto& hdr = file.GetHeader();
    auto shape = hdr.GetBlock<nifly::NiShape>(shape_index);
    if (!shape || bones.size() != weights.size())
        return false;
    auto skin = hdr.GetBlock<nifly::NiBoneContainer>(shape->SkinInstanceRef());
    if (!skin)
        return false;

    size_t slots = 0;
    uint32_t bone_limit = skin->boneRefs.GetSize();
    if (auto geom = dynamic_cast<nifly::BSGeometry*>(shape)) {
        auto data =
            dynamic_cast<nifly::BSGeometryMeshData*>(geom->GetGeomData());
        if (!data || vertex >= data->skinWeights.size())
            return false;
        slots = data->nWeightsPerVert ? data->nWeightsPerVert : 4;
        bone_limit = std::min<uint32_t>(bone_limit, 65536);
    } else if (auto tri = dynamic_cast<nifly::BSTriShape*>(shape)) {
        if (!tri->IsSkinned() || vertex >= tri->vertData.size())
            return false;
        slots = 4;
        bone_limit = std::min<uint32_t>(bone_limit, 256);
    } else
        return false;

    if (bones.size() > slots)
        return false;
    for (size_t i = 0; i < bones.size(); i++)
        if (bones[i] >= bone_limit || !std::isfinite(weights[i])
            || weights[i] < 0.0f)
            return false;

    // As upstream: normalize to a sum of 1 and clear the unused slots.
    float sum = 0.0f;
    for (float w : weights)
        sum += w;
    if (sum <= 0.0f)
        sum = 1.0f;

    if (auto geom = dynamic_cast<nifly::BSGeometry*>(shape)) {
        auto data =
            static_cast<nifly::BSGeometryMeshData*>(geom->GetGeomData());
        auto& slots_out = data->skinWeights[vertex];
        slots_out.assign(slots, nifly::BSGeometryMeshData::BoneWeight {});
        for (size_t i = 0; i < bones.size(); i++) {
            slots_out[i].boneIndex = static_cast<uint16_t>(bones[i]);
            slots_out[i].weight =
                static_cast<uint16_t>(std::lround(weights[i] / sum * 65535.0f));
        }
        return true;
    }

    auto& v = static_cast<nifly::BSTriShape*>(shape)->vertData[vertex];
    std::memset(v.weights, 0, sizeof(v.weights));
    std::memset(v.weightBones, 0, sizeof(v.weightBones));
    for (size_t i = 0; i < bones.size(); i++) {
        v.weightBones[i] = static_cast<uint8_t>(bones[i]);
        v.weights[i] = weights[i] / sum;
    }
    return true;
}

// NifFile::ClearShapeVertWeights, reimplemented on the shape at `shape_index`
// for the same reason as set_shape_vertex_weights. Returns false, changing
// nothing, unless the shape is a skinned BSTriShape or a skinned BSGeometry
// with mesh data; upstream silently ignores other shapes.
inline bool
clear_shape_vertex_weights(nifly::NifFile& file, uint32_t shape_index) {
    auto& hdr = file.GetHeader();
    auto shape = hdr.GetBlock<nifly::NiShape>(shape_index);
    if (!shape
        || !hdr.GetBlock<nifly::NiBoneContainer>(shape->SkinInstanceRef()))
        return false;

    if (auto geom = dynamic_cast<nifly::BSGeometry*>(shape)) {
        auto data =
            dynamic_cast<nifly::BSGeometryMeshData*>(geom->GetGeomData());
        if (!data)
            return false;
        // As upstream: one zeroed entry of nWeightsPerVert slots per vertex.
        if (data->nWeightsPerVert == 0)
            data->nWeightsPerVert = 4;
        data->skinWeights.assign(
            data->vertices.size(),
            std::vector<nifly::BSGeometryMeshData::BoneWeight>(
                data->nWeightsPerVert
            )
        );
        return true;
    }

    auto tri = dynamic_cast<nifly::BSTriShape*>(shape);
    if (!tri || !tri->IsSkinned())
        return false;
    for (auto& v : tri->vertData) {
        std::memset(v.weights, 0, sizeof(v.weights));
        std::memset(v.weightBones, 0, sizeof(v.weightBones));
    }
    return true;
}

// NifFile::CreateSkinning on the shape at `shape_index`, which returns nothing.
// Returns false, changing nothing, unless the shape is one upstream knows how to
// skin (NiTriShape, NiTriStrips, BSTriShape, BSGeometry) and has no skin
// instance yet; upstream silently skips the rest.
inline bool create_skinning(nifly::NifFile& file, uint32_t shape_index) {
    auto shape = file.GetHeader().GetBlock<nifly::NiShape>(shape_index);
    if (!shape || !shape->SkinInstanceRef()
        || !shape->SkinInstanceRef()->IsEmpty())
        return false;
    if (!shape->HasType<nifly::NiTriShape>()
        && !shape->HasType<nifly::NiTriStrips>()
        && !shape->HasType<nifly::BSTriShape>()
        && !shape->HasType<nifly::BSGeometry>())
        return false;
    file.CreateSkinning(shape);
    return true;
}

// NifFile::DeleteSkinning on the shape at `shape_index`. Upstream deletes the
// skin instance, its skin data, partition and bone data even when other blocks
// share them. This unlinks the shape and deletes only the ones nothing else
// refers to, then lets upstream clear the skinning flags (and, for BSGeometry,
// its SkinAttach and mesh weights). Returns false, changing nothing, if the
// block isn't a shape with a skin instance.
inline bool delete_skinning(nifly::NifFile& file, uint32_t shape_index) {
    auto& hdr = file.GetHeader();
    auto shape = hdr.GetBlock<nifly::NiShape>(shape_index);
    if (!shape || !shape->SkinInstanceRef()
        || shape->SkinInstanceRef()->IsEmpty())
        return false;

    uint32_t inst_id = shape->SkinInstanceRef()->index;
    std::vector<uint32_t> owned;
    if (auto inst = hdr.GetBlock<nifly::NiSkinInstance>(inst_id)) {
        owned.push_back(inst->dataRef.index);
        owned.push_back(inst->skinPartitionRef.index);
    } else if (auto inst = hdr.GetBlock<nifly::BSSkinInstance>(inst_id))
        owned.push_back(inst->dataRef.index);
    shape->SkinInstanceRef()->Clear();

    // The instance first: until it's gone, it still refers to its data.
    if (hdr.GetBlock<nifly::NiObject>(inst_id)
        && hdr.GetBlockRefCount(inst_id, false) == 0) {
        hdr.DeleteBlock(inst_id);
        for (auto& id : owned)
            if (id != nifly::NIF_NPOS && id > inst_id)
                id--;
        std::sort(owned.begin(), owned.end(), std::greater<>());
        for (uint32_t id : owned)
            if (id != nifly::NIF_NPOS && hdr.GetBlock<nifly::NiObject>(id)
                && hdr.GetBlockRefCount(id, false) == 0)
                hdr.DeleteBlock(id);
    }

    file.DeleteSkinning(shape);
    return true;
}

// NifFile::SetShapeBoneIDList on the shape at `shape_index`. Returns false,
// changing nothing, unless the shape has a skin instance, isn't a BSGeometry
// (which binds bones by name; see set_shape_bone_names), and every id is a
// node block. Upstream stores any index it's given.
inline bool set_shape_bone_ids(
    nifly::NifFile& file,
    uint32_t shape_index,
    rust::Slice<const uint32_t> ids
) {
    auto& hdr = file.GetHeader();
    auto shape = hdr.GetBlock<nifly::NiShape>(shape_index);
    if (!shape || shape->HasType<nifly::BSGeometry>()
        || !hdr.GetBlock<nifly::NiBoneContainer>(shape->SkinInstanceRef()))
        return false;
    std::vector<int> list;
    list.reserve(ids.size());
    for (uint32_t id : ids) {
        if (!hdr.GetBlock<nifly::NiNode>(id))
            return false;
        list.push_back(static_cast<int>(id));
    }
    file.SetShapeBoneIDList(shape, list);
    return true;
}

// NifFile::SetShapeBoneList for a BSGeometry at `shape_index`, which writes the
// names to its SkinAttach extra data. `names` holds the names back to back,
// `lengths` their sizes. Returns false, changing nothing, unless the shape is a
// BSGeometry with a BSSkin::Instance and the lengths add up.
inline bool set_shape_bone_names(
    nifly::NifFile& file,
    uint32_t shape_index,
    rust::Slice<const uint8_t> names,
    rust::Slice<const size_t> lengths
) {
    auto& hdr = file.GetHeader();
    auto shape = hdr.GetBlock<nifly::BSGeometry>(shape_index);
    if (!shape
        || !hdr.GetBlock<nifly::BSSkinInstance>(shape->SkinInstanceRef()))
        return false;
    std::vector<std::string> list;
    list.reserve(lengths.size());
    size_t pos = 0;
    for (size_t len : lengths) {
        if (len > names.size() - pos)
            return false;
        list.emplace_back(
            reinterpret_cast<const char*>(names.data() + pos),
            len
        );
        pos += len;
    }
    if (pos != names.size())
        return false;
    file.SetShapeBoneList(shape, list);
    return true;
}

// NifFile::UpdateShapeBoneID, reimplemented on the shape at `shape_index`
// rather than the first shape with its name: points the first of its bones that
// is `old_id` to `new_id`. Returns false, changing nothing, unless the shape is
// skinned, `new_id` is a node block and some bone is `old_id`; upstream checks
// neither and stores any index.
inline bool replace_shape_bone(
    nifly::NifFile& file,
    uint32_t shape_index,
    uint32_t old_id,
    uint32_t new_id
) {
    auto& hdr = file.GetHeader();
    auto shape = hdr.GetBlock<nifly::NiShape>(shape_index);
    if (!shape || !hdr.GetBlock<nifly::NiNode>(new_id))
        return false;
    auto bones = hdr.GetBlock<nifly::NiBoneContainer>(shape->SkinInstanceRef());
    if (!bones)
        return false;
    for (auto& bone : bones->boneRefs) {
        if (!bone.IsEmpty() && bone.index == old_id) {
            bone.index = new_id;
            return true;
        }
    }
    return false;
}

// NifFile::GetShapeBoneWeights, as a vector sorted by vertex index instead of
// an unordered_map. `bone_index` is the position in the shape's bone list.
// GetShapeBoneWeights takes a non-const pointer but only reads.
// NifFile::SetShapeBoneWeights, but on the shape at `shape_index` rather than
// the first shape with its name. Only NiSkinData weights are set, as upstream;
// BSTriShape and BSGeometry are rejected because GetShapeBoneWeights reads
// their per-vertex weights instead. Weights below 0.0001 are dropped, as upstream.
inline bool set_shape_bone_weights(
    nifly::NifFile& file,
    uint32_t shape_index,
    uint32_t bone_index,
    rust::Slice<const VertexWeight> weights
) {
    auto& hdr = file.GetHeader();
    auto shape = hdr.GetBlock<nifly::NiShape>(shape_index);
    if (!shape || dynamic_cast<nifly::BSTriShape*>(shape)
        || dynamic_cast<nifly::BSGeometry*>(shape))
        return false;

    auto skinInst =
        hdr.GetBlock<nifly::NiSkinInstance>(shape->SkinInstanceRef());
    if (!skinInst)
        return false;
    auto data = hdr.GetBlock(skinInst->dataRef);
    if (!data || bone_index >= data->numBones)
        return false;

    uint16_t numVertices = shape->GetNumVertices();
    for (const auto& w : weights)
        if (w.vertex >= numVertices)
            return false;

    data->hasVertWeights = true;
    auto& bone = data->bones[bone_index];
    bone.vertexWeights.clear();
    for (const auto& w : weights)
        if (w.weight >= 0.0001f)
            bone.vertexWeights.emplace_back(
                nifly::SkinWeight(w.vertex, w.weight)
            );
    bone.numVertices = static_cast<uint16_t>(bone.vertexWeights.size());
    return true;
}

// NifFile::UpdateSkinPartitions on the shape at `shape_index`, which returns
// nothing. Checks first what it would silently skip, or where it would read or
// write out of bounds:
// - the block isn't a shape with an NiSkinInstance, NiSkinData and
//   NiSkinPartition, or its triangles can't be read;
// - the NiSkinData has no vertex weights (BSTriShape files may keep them only
//   per vertex): every partition would lose its bones;
// - the shape has triangles but the partition has no partitions (every
//   triangle goes to partition 0, indexing an empty vector);
// - a BSDismemberSkinInstance lists a different number of partitions than the
//   NiSkinPartition (splitting indexes and inserts by partition);
// - with no per-partition bone limit (Skyrim LE and others), more than 256
//   bones, whose partition-local indices would wrap in a uint8_t.
// The shape at `shape_index` if UpdateSkinPartitions can run on it, apart from
// the partition-count checks below (which set_shape_partitions satisfies by
// construction), with its triangles. Null otherwise.
inline nifly::NiShape* updatable_skin(
    nifly::NifFile& file,
    uint32_t shape_index,
    std::vector<nifly::Triangle>& tris
) {
    auto& hdr = file.GetHeader();
    auto shape = hdr.GetBlock<nifly::NiShape>(shape_index);
    if (!shape || !shape->SkinInstanceRef())
        return nullptr;
    auto skinInst =
        hdr.GetBlock<nifly::NiSkinInstance>(shape->SkinInstanceRef());
    if (!skinInst)
        return nullptr;
    auto data = hdr.GetBlock(skinInst->dataRef);
    auto part = hdr.GetBlock(skinInst->skinPartitionRef);
    if (!data || !part || !data->hasVertWeights)
        return nullptr;
    if (!shape->GetTriangles(tris))
        return nullptr;

    auto version = hdr.GetVersion();
    bool limited = version.IsOB() || version.IsFO3() || version.IsSSE();
    if (!limited && data->bones.size() > 256)
        return nullptr;
    return shape;
}

inline bool update_skin_partitions(nifly::NifFile& file, uint32_t shape_index) {
    std::vector<nifly::Triangle> tris;
    auto shape = updatable_skin(file, shape_index, tris);
    if (!shape)
        return false;
    auto& hdr = file.GetHeader();
    auto skinInst =
        hdr.GetBlock<nifly::NiSkinInstance>(shape->SkinInstanceRef());
    auto part = hdr.GetBlock(skinInst->skinPartitionRef);
    if (!tris.empty() && part->partitions.empty())
        return false;

    auto dismember = dynamic_cast<nifly::BSDismemberSkinInstance*>(skinInst);
    if (dismember && dismember->partitions.size() != part->partitions.size())
        return false;

    // Upstream reuses a cached triangle-to-partition list when its length
    // matches, which keeps triangles in no partition out (after
    // delete_partitions) until the file is reloaded. Rebuilding it from the
    // partitions puts them in the first one every time.
    part->triParts.clear();
    file.UpdateSkinPartitions(shape);
    return true;
}

// NifFile::DeletePartitions on the shape at `shape_index`, with `indices` sorted
// and deduplicated here (upstream requires both). Returns false, changing
// nothing, unless the shape has an NiSkinInstance with an NiSkinPartition, every
// index is a partition, and a BSDismemberSkinInstance has as many partitions;
// upstream erases out of range otherwise.
inline bool delete_partitions(
    nifly::NifFile& file,
    uint32_t shape_index,
    rust::Slice<const uint32_t> indices
) {
    auto& hdr = file.GetHeader();
    auto shape = hdr.GetBlock<nifly::NiShape>(shape_index);
    if (!shape)
        return false;
    auto skinInst =
        hdr.GetBlock<nifly::NiSkinInstance>(shape->SkinInstanceRef());
    if (!skinInst)
        return false;
    auto part = hdr.GetBlock(skinInst->skinPartitionRef);
    if (!part)
        return false;
    auto dismember = dynamic_cast<nifly::BSDismemberSkinInstance*>(skinInst);
    if (dismember && dismember->partitions.size() != part->partitions.size())
        return false;

    std::vector<uint32_t> sorted(indices.begin(), indices.end());
    std::sort(sorted.begin(), sorted.end());
    sorted.erase(std::unique(sorted.begin(), sorted.end()), sorted.end());
    if (!sorted.empty() && sorted.back() >= part->partitions.size())
        return false;
    file.DeletePartitions(shape, sorted);
    return true;
}

// NifFile::GetExternalGeometryPathRefs, as a count and a getter. The meshes are
// only reachable through SelectMesh, so this selects each and then releases,
// leaving the first selected as before (nothing else in the bindings selects
// another). 0 unless the shape is a BSGeometry with external mesh data.
inline uint32_t external_mesh_count(const nifly::NiShape& shape) {
    auto geom =
        dynamic_cast<nifly::BSGeometry*>(const_cast<nifly::NiShape*>(&shape));
    if (!geom || geom->HasInternalGeomData())
        return 0;
    return geom->MeshCount();
}

// Empty if `index` is out of range.
inline rust::Slice<const uint8_t>
external_mesh_path(const nifly::NiShape& shape, uint32_t index) {
    if (index >= external_mesh_count(shape))
        return {};
    auto geom =
        static_cast<nifly::BSGeometry*>(const_cast<nifly::NiShape*>(&shape));
    const std::string& name =
        geom->SelectMesh(static_cast<uint8_t>(index))->meshName.get();
    geom->ReleaseMesh();
    return {reinterpret_cast<const uint8_t*>(name.data()), name.size()};
}

// NifFile::LoadExternalShapeData, reimplemented to read into fresh mesh data
// and only replace the mesh's on success: upstream reads into the mesh itself,
// leaving it half-overwritten when the data is short or malformed, and returns
// true whether or not the mesh exists. Reader errors propagate as the
// rust::Error thrown by the streambuf. Returns 0 on success, 1 if the shape
// isn't a BSGeometry with external mesh `index`, 2 if the data ended early, 3
// if its version is one upstream can't read (above 2).
inline int32_t load_external_mesh(
    nifly::NifFile& file,
    uint32_t shape_index,
    uint32_t index,
    read_bridge::RustReader& reader
) {
    auto shape = file.GetHeader().GetBlock<nifly::BSGeometry>(shape_index);
    if (!shape || index >= external_mesh_count(*shape))
        return 1;

    read_bridge::RustIStream in(reader);
    in.exceptions(std::ios::badbit);
    std::string bytes(std::istreambuf_iterator<char>(in), {});

    // Mesh data can't be assigned (it has a const member), so parse into a
    // scratch copy first and only parse into the mesh once that succeeds.
    auto parse = [&](nifly::BSGeometryMeshData& data) {
        std::istringstream buffer(bytes);
        nifly::NiIStream stream(&buffer, nullptr);
        nifly::NiStreamReversible sync(
            &stream,
            nullptr,
            nifly::NiStreamReversible::Mode::Reading
        );
        data.Sync(sync);
        return static_cast<bool>(buffer);
    };
    nifly::BSGeometryMeshData scratch;
    if (!parse(scratch))
        return 2;
    // Upstream stops after the version when it's one it can't read, as if the
    // mesh were empty.
    if (scratch.version > 2)
        return 3;

    parse(shape->SelectMesh(static_cast<uint8_t>(index))->meshData);
    shape->ReleaseMesh();
    return 0;
}

// NifFile::SaveExternalShapeData into `out`, from a copy of the mesh data:
// writing updates the data's counters, and this shouldn't change the file. The
// copy's position scale is widened first if the positions need it.
// Returns false, writing nothing, unless the shape is a BSGeometry with external
// mesh `index`.
inline bool save_external_mesh(
    const nifly::NiShape& shape,
    uint32_t index,
    rust::Vec<uint8_t>& out
) {
    if (index >= external_mesh_count(shape))
        return false;
    auto geom =
        static_cast<nifly::BSGeometry*>(const_cast<nifly::NiShape*>(&shape));
    auto copy = geom->SelectMesh(static_cast<uint8_t>(index))->meshData.Clone();
    geom->ReleaseMesh();

    fit_mesh_scale(*copy);

    std::ostringstream buffer;
    nifly::NiOStream stream(&buffer, nullptr);
    nifly::NiStreamReversible sync(
        nullptr,
        &stream,
        nifly::NiStreamReversible::Mode::Writing
    );
    write_mesh_data(*copy, sync);
    const std::string bytes = buffer.str();
    out.reserve(bytes.size());
    for (char c : bytes)
        out.push_back(static_cast<uint8_t>(c));
    return true;
}

// Writes through GetExternalGeometryPathRefs' reference for mesh `index` of the
// BSGeometry at `shape_index`. Returns false if there's no such external mesh.
inline bool set_external_mesh_path(
    nifly::NifFile& file,
    uint32_t shape_index,
    uint32_t index,
    rust::Slice<const uint8_t> path
) {
    auto shape = file.GetHeader().GetBlock<nifly::NiShape>(shape_index);
    if (!shape)
        return false;
    auto refs = file.GetExternalGeometryPathRefs(shape);
    if (index >= refs.size())
        return false;
    refs[index].get().assign(
        reinterpret_cast<const char*>(path.data()),
        path.size()
    );
    return true;
}

// NifFile::CreateShapeFromData. Returns the new shape's index, or NIF_NPOS,
// changing nothing, unless: the file has a root node and is a version upstream
// builds shapes for (Morrowind, Fallout 3/NV, Skyrim LE/SE, Fallout 4/76; for
// others it builds Fallout 3 blocks), there are at most 65535 vertices, every
// triangle index is below the vertex count, the triangle count fits (65535 for
// NiTriShapeData), and `uvs` and `normals` are empty or have one per vertex
// (upstream reads them per vertex without checking).
//
// The version restriction works around an upstream issue: CreateShapeFromData's
// final branch only tells Skyrim LE apart, so every other version, Oblivion
// included, gets Fallout 3 blocks (NiTriShape with a BSShaderPPLightingProperty,
// which Oblivion doesn't have; it needs NiMaterialProperty and
// NiTexturingProperty, as the Morrowind branch builds). It never checks IsOB().
// Starfield is a missing feature instead: no branch builds a BSGeometry.
inline uint32_t create_shape(
    nifly::NifFile& file,
    rust::Slice<const uint8_t> name,
    rust::Slice<const Vec3> vertices,
    rust::Slice<const Triangle> triangles,
    rust::Slice<const Uv> uvs,
    rust::Slice<const Vec3> normals
) {
    auto version = file.GetHeader().GetVersion();
    bool bs_tri_shape = version.IsSSE() || version.IsFO4() || version.IsFO76();
    if (!bs_tri_shape && !version.IsMW() && !version.IsFO3() && !version.IsSK())
        return nifly::NIF_NPOS;
    if (!file.GetRootNode() || vertices.size() > 0xFFFF)
        return nifly::NIF_NPOS;
    if ((!uvs.empty() && uvs.size() != vertices.size())
        || (!normals.empty() && normals.size() != vertices.size()))
        return nifly::NIF_NPOS;
    size_t limit = bs_tri_shape ? file.GetTriangleLimit() : 0xFFFF;
    if (triangles.size() > limit)
        return nifly::NIF_NPOS;

    std::vector<nifly::Vector3> v;
    v.reserve(vertices.size());
    for (const auto& p : vertices)
        v.emplace_back(p.x, p.y, p.z);
    std::vector<nifly::Triangle> t;
    t.reserve(triangles.size());
    for (const auto& tri : triangles) {
        if (tri.p1 >= v.size() || tri.p2 >= v.size() || tri.p3 >= v.size())
            return nifly::NIF_NPOS;
        t.emplace_back(tri.p1, tri.p2, tri.p3);
    }
    std::vector<nifly::Vector2> uv;
    for (const auto& u : uvs)
        uv.emplace_back(u.u, u.v);
    std::vector<nifly::Vector3> n;
    for (const auto& p : normals)
        n.emplace_back(p.x, p.y, p.z);

    std::string shape_name(
        reinterpret_cast<const char*>(name.data()),
        name.size()
    );
    auto shape = file.CreateShapeFromData(
        shape_name,
        &v,
        &t,
        uv.empty() ? nullptr : &uv,
        n.empty() ? nullptr : &n
    );
    return shape ? file.GetBlockID(shape) : nifly::NIF_NPOS;
}

// NifFile::TriangulateShape on the shape at `shape_index`, which returns
// nothing. Returns false, changing nothing, unless the shape is an NiTriStrips
// with triangles whose NiTriStripsData no other block refers to: upstream
// replaces the data block in place, which would leave another strips shape
// sharing it with NiTriShapeData.
inline bool triangulate_shape(nifly::NifFile& file, uint32_t shape_index) {
    auto& hdr = file.GetHeader();
    auto shape = hdr.GetBlock<nifly::NiTriStrips>(shape_index);
    if (!shape)
        return false;
    auto data = hdr.GetBlock<nifly::NiTriStripsData>(shape->DataRef());
    if (!data || data->StripsToTris().empty())
        return false;
    if (hdr.GetBlockRefCount(shape->DataRef()->index, false) != 1)
        return false;
    file.TriangulateShape(shape);
    return true;
}

// NifFile::InvertUVsForShape on the shape at `shape_index`, which returns
// nothing. Returns false, changing nothing, unless the shape stores UVs;
// upstream silently skips the rest.
inline bool
invert_shape_uvs(nifly::NifFile& file, uint32_t shape_index, bool u, bool v) {
    auto shape = file.GetHeader().GetBlock<nifly::NiShape>(shape_index);
    std::vector<nifly::Vector2> uvs;
    if (!shape || !file.GetUvsForShape(shape, uvs))
        return false;
    file.InvertUVsForShape(shape, u, v);
    return true;
}

// NifFile::MirrorShape on the shape at `shape_index`, which returns nothing.
// Returns false, changing nothing, unless the shape has vertices it mirrors
// (geometry data or a BSTriShape's own), and, when an odd number of axes is
// mirrored, triangles it can rewind: upstream can't set strips' triangles, so it
// would mirror a strips shape without flipping its winding, turning it inside
// out.
inline bool mirror_shape(
    nifly::NifFile& file,
    uint32_t shape_index,
    bool x,
    bool y,
    bool z
) {
    auto shape = file.GetHeader().GetBlock<nifly::NiShape>(shape_index);
    if (!shape)
        return false;
    auto data = file.GetGeometryData(shape);
    if (!data && !dynamic_cast<nifly::BSTriShape*>(shape))
        return false;
    bool flip = (x != y) != z;
    if (flip && shape->HasType<nifly::NiTriStrips>())
        return false;
    file.MirrorShape(shape, x, y, z);
    return true;
}

// NifFile::MoveVertex on the shape at `shape_index`, which returns nothing.
// Returns false, changing nothing, unless the shape stores vertices (see
// settable_vertex_count) and `vertex` is one of them. Upstream compares a
// signed index, so a negative one would write out of bounds.
inline bool set_shape_vertex(
    nifly::NifFile& file,
    uint32_t shape_index,
    uint32_t vertex,
    Vec3 position
) {
    int32_t count = settable_vertex_count(file, shape_index);
    if (count < 0 || vertex >= static_cast<uint32_t>(count))
        return false;
    auto shape = file.GetHeader().GetBlock<nifly::NiShape>(shape_index);
    file.MoveVertex(shape, vec3_from_ffi(position), static_cast<int>(vertex));
    return true;
}

// Deletes the blocks in `roots`, and the blocks they refer to through child
// refs, recursively, but only those no other block refers to. Callers unlink
// the roots first. Repeats until a pass deletes nothing, since deleting a block
// can leave the blocks it referred to unreferenced.
inline void delete_if_unreferenced(
    nifly::NifFile& file,
    const std::vector<uint32_t>& roots
) {
    auto& hdr = file.GetHeader();
    std::set<uint32_t> candidates;
    std::vector<uint32_t> stack(roots.begin(), roots.end());
    while (!stack.empty()) {
        uint32_t id = stack.back();
        stack.pop_back();
        auto block = hdr.GetBlock<nifly::NiObject>(id);
        if (!block || !candidates.insert(id).second)
            continue;
        std::vector<uint32_t> children;
        block->GetChildIndices(children);
        for (uint32_t c : children)
            if (c != nifly::NIF_NPOS)
                stack.push_back(c);
    }

    bool deleted = true;
    while (deleted) {
        deleted = false;
        // Highest first, then shift the rest down past each deleted block.
        for (auto it = candidates.rbegin(); it != candidates.rend(); ++it) {
            uint32_t id = *it;
            if (hdr.GetBlockRefCount(id, false) != 0)
                continue;
            hdr.DeleteBlock(id);
            std::set<uint32_t> rest;
            for (uint32_t c : candidates)
                if (c != id)
                    rest.insert(c > id ? c - 1 : c);
            candidates = std::move(rest);
            deleted = true;
            break;
        }
    }
}

// NifFile::DeleteShape, reimplemented: upstream deletes the shape's data,
// properties, extra data, skin and shader even when other blocks share them,
// and leaves its collision and controllers behind. This removes the shape from
// every node's children, deletes it, then deletes whatever it referred to that
// nothing else does. Returns false if the block isn't a shape.
inline bool delete_shape(nifly::NifFile& file, uint32_t shape_index) {
    auto& hdr = file.GetHeader();
    auto shape = hdr.GetBlock<nifly::NiShape>(shape_index);
    if (!shape)
        return false;
    for (auto node : file.GetNodes())
        for (uint32_t i = node->childRefs.GetSize(); i-- > 0;)
            if (node->childRefs.GetBlockRef(i) == shape_index)
                node->childRefs.RemoveBlockRef(i);
    delete_if_unreferenced(file, {shape_index});
    return true;
}

// NifFile::DeleteShader, reimplemented: unlinks the shape's shader, the shaders
// and materials in its property list, and its alpha property, then deletes
// those (and their texture sets and controllers) only if nothing else refers to
// them. Upstream deletes them even when shared, and only removes two shader
// types from property lists. Returns false, changing nothing, if the block
// isn't a shape or has none of them.
inline bool delete_shader(nifly::NifFile& file, uint32_t shape_index) {
    auto& hdr = file.GetHeader();
    auto shape = hdr.GetBlock<nifly::NiShape>(shape_index);
    if (!shape)
        return false;
    std::vector<uint32_t> unlinked;
    if (auto ref = shape->ShaderPropertyRef(); ref && hdr.GetBlock(ref)) {
        unlinked.push_back(ref->index);
        ref->Clear();
    }
    if (auto ref = shape->AlphaPropertyRef(); ref && hdr.GetBlock(ref)) {
        unlinked.push_back(ref->index);
        ref->Clear();
    }
    for (uint32_t i = shape->propertyRefs.GetSize(); i-- > 0;) {
        uint32_t id = shape->propertyRefs.GetBlockRef(i);
        if (hdr.GetBlock<nifly::NiShader>(id)
            || hdr.GetBlock<nifly::NiAlphaProperty>(id)) {
            unlinked.push_back(id);
            shape->propertyRefs.RemoveBlockRef(i);
        }
    }
    if (unlinked.empty())
        return false;
    delete_if_unreferenced(file, unlinked);
    return true;
}

// NifFile::DeleteVertsForShape on the shape at `shape_index`, with `vertices`
// sorted and deduplicated here (upstream requires both and erases out of
// bounds otherwise). Returns false, changing nothing, unless the shape stores
// vertices and every index is one of them; a BSGeometry mesh must have at most
// 65535 vertices, as upstream requires. Upstream's own return value (whether the
// shape was left without vertices or triangles) isn't passed on.
inline bool delete_shape_vertices(
    nifly::NifFile& file,
    uint32_t shape_index,
    rust::Slice<const uint32_t> vertices
) {
    int32_t count = settable_vertex_count(file, shape_index);
    if (count < 0)
        return false;
    auto shape = file.GetHeader().GetBlock<nifly::NiShape>(shape_index);
    if (auto geom = dynamic_cast<nifly::BSGeometry*>(shape)) {
        auto data =
            dynamic_cast<nifly::BSGeometryMeshData*>(geom->GetGeomData());
        if (!data || data->vertices.size() > 0xFFFF)
            return false;
    }
    std::vector<uint16_t> sorted;
    sorted.reserve(vertices.size());
    for (uint32_t v : vertices) {
        if (v >= static_cast<uint32_t>(count))
            return false;
        sorted.push_back(static_cast<uint16_t>(v));
    }
    std::sort(sorted.begin(), sorted.end());
    sorted.erase(std::unique(sorted.begin(), sorted.end()), sorted.end());
    if (!sorted.empty())
        file.DeleteVertsForShape(shape, sorted);
    return true;
}

// Whether two files are from the same game (by NiVersion's Is* ranges), so the
// block types cloned from one exist in the other. Saving writes the copies in the
// target's version, dropping fields it doesn't store. Files in none of the ranges
// must match exactly. Upstream clones across any versions, which can put block
// types a game doesn't have (BSTriShape in Skyrim LE) into its files.
inline bool same_game(const nifly::NifFile& a, const nifly::NifFile& b) {
    const auto& va = a.GetHeader().GetVersion();
    const auto& vb = b.GetHeader().GetVersion();
    using Is = bool (nifly::NiVersion::*)() const;
    for (Is is :
         {&nifly::NiVersion::IsMW,
          &nifly::NiVersion::IsOB,
          &nifly::NiVersion::IsFO3,
          &nifly::NiVersion::IsSK,
          &nifly::NiVersion::IsSSE,
          &nifly::NiVersion::IsFO4,
          &nifly::NiVersion::IsFO76,
          &nifly::NiVersion::IsSF}) {
        if ((va.*is)() || (vb.*is)())
            return (va.*is)() && (vb.*is)();
    }
    return va.File() == vb.File() && va.User() == vb.User()
        && va.Stream() == vb.Stream();
}

inline uint32_t clone_shape_impl(
    nifly::NifFile& file,
    nifly::NiShape* shape,
    rust::Slice<const uint8_t> name,
    nifly::NifFile* source
) {
    std::string n(reinterpret_cast<const char*>(name.data()), name.size());
    auto clone = file.CloneShape(shape, n, source);
    return clone ? file.GetBlockID(clone) : nifly::NIF_NPOS;
}

// NifFile::CloneShape within the file, on the shape at `shape_index`. Returns
// the clone's index, or NIF_NPOS if that block isn't a shape.
inline uint32_t clone_shape(
    nifly::NifFile& file,
    uint32_t shape_index,
    rust::Slice<const uint8_t> name
) {
    auto shape = file.GetHeader().GetBlock<nifly::NiShape>(shape_index);
    return shape ? clone_shape_impl(file, shape, name, nullptr)
                 : nifly::NIF_NPOS;
}

// NifFile::CloneShape from `source`, which it only reads. NIF_NPOS unless both
// files are from the same game.
inline uint32_t clone_shape_from(
    nifly::NifFile& file,
    const nifly::NifFile& source,
    const nifly::NiShape& shape,
    rust::Slice<const uint8_t> name
) {
    if (!same_game(file, source))
        return nifly::NIF_NPOS;
    return clone_shape_impl(
        file,
        const_cast<nifly::NiShape*>(&shape),
        name,
        const_cast<nifly::NifFile*>(&source)
    );
}

// NifFile::CloneNamedNode, reimplemented on a given node rather than the first
// node with a name: copies it without its children, effects, collision or
// controller, adds it as an unparented block, then clones what it still refers
// to (extra data, properties).
inline uint32_t clone_node_impl(
    nifly::NifFile& file,
    const nifly::NiNode& node,
    nifly::NifFile* source
) {
    auto copy = node.Clone();
    copy->collisionRef.Clear();
    copy->controllerRef.Clear();
    copy->childRefs.Clear();
    copy->effectRefs.Clear();
    uint32_t id = file.GetHeader().AddBlock(std::move(copy));
    file.CloneChildren(file.GetHeader().GetBlock<nifly::NiNode>(id), source);
    return id;
}

// Within the file. NIF_NPOS if `node_index` isn't a node.
inline uint32_t clone_node(nifly::NifFile& file, uint32_t node_index) {
    auto node = file.GetHeader().GetBlock<nifly::NiNode>(node_index);
    return node ? clone_node_impl(file, *node, nullptr) : nifly::NIF_NPOS;
}

// From `source`, which it only reads. NIF_NPOS unless both files are from the
// same game.
inline uint32_t clone_node_from(
    nifly::NifFile& file,
    const nifly::NifFile& source,
    const nifly::NiNode& node
) {
    if (!same_game(file, source))
        return nifly::NIF_NPOS;
    return clone_node_impl(file, node, const_cast<nifly::NifFile*>(&source));
}

// NifFile::CloneChildren within the file: replaces each child ref of the block
// at `block_index` with a ref to a deep copy. False if there's no such block.
inline bool clone_children(nifly::NifFile& file, uint32_t block_index) {
    auto block = file.GetHeader().GetBlock<nifly::NiObject>(block_index);
    if (!block)
        return false;
    file.CloneChildren(block, nullptr);
    return true;
}

// NifFile::ReorderTriangles on the shape at `shape_index`: triangle i becomes
// the old triangle `order[i]`. Returns false, changing nothing, unless the
// shape has triangles it can set (not strips) and `order` is a permutation of
// them; upstream only checks the length and range, so a repeated index
// duplicates one triangle and drops another.
inline bool reorder_shape_triangles(
    nifly::NifFile& file,
    uint32_t shape_index,
    rust::Slice<const uint32_t> order
) {
    auto shape = file.GetHeader().GetBlock<nifly::NiShape>(shape_index);
    if (!shape || shape->HasType<nifly::NiTriStrips>())
        return false;
    std::vector<nifly::Triangle> tris;
    if (!shape->GetTriangles(tris) || order.size() != tris.size())
        return false;
    std::vector<bool> seen(tris.size());
    for (uint32_t i : order) {
        if (i >= tris.size() || seen[i])
            return false;
        seen[i] = true;
    }
    std::vector<uint32_t> list(order.begin(), order.end());
    return nifly::NifFile::ReorderTriangles(shape, list);
}

// NifFile::GetShapeSegments. Part IDs are numbered in order: each segment, then
// its subsegments. Flattens the segmentation: `segs` has each segment's
// subsegment count, `subs` each subsegment's data with `extra_len` of its floats
// in `extra`, and `tri_parts` each triangle's part ID (-1 for none). Returns
// false unless the shape is a BSSubIndexTriShape.
inline bool shape_segments(
    const nifly::NiShape& shape,
    rust::Vec<uint8_t>& ssf,
    rust::Vec<SegmentFfi>& segs,
    rust::Vec<SubsegmentFfi>& subs,
    rust::Vec<float>& extra,
    rust::Vec<int32_t>& tri_parts
) {
    auto bssits = dynamic_cast<const nifly::BSSubIndexTriShape*>(&shape);
    if (!bssits)
        return false;
    nifly::NifSegmentationInfo inf;
    std::vector<int> parts;
    bssits->GetSegmentation(inf, parts);

    for (char c : inf.ssfFile)
        ssf.push_back(static_cast<uint8_t>(c));
    for (const auto& seg : inf.segs) {
        segs.push_back(SegmentFfi {static_cast<uint32_t>(seg.subs.size())});
        for (const auto& sub : seg.subs) {
            subs.push_back(
                SubsegmentFfi {
                    sub.userSlotID,
                    sub.material,
                    static_cast<uint32_t>(sub.extraData.size())
                }
            );
            for (float f : sub.extraData)
                extra.push_back(f);
        }
    }
    for (int p : parts)
        tri_parts.push_back(p);
    return true;
}

// NifFile::SetShapeSegments on the shape at `shape_index`, from the flattened
// form shape_segments returns, with part IDs numbered the same way. Returns
// false, changing nothing, unless the shape is a BSSubIndexTriShape,
// `tri_parts` has one entry per triangle, each is a part ID or -1, and the
// subsegments and floats add up. Upstream indexes its part-ID table with any
// value it's given, and silently does nothing on a length mismatch.
inline bool set_shape_segments(
    nifly::NifFile& file,
    uint32_t shape_index,
    rust::Slice<const uint8_t> ssf,
    rust::Slice<const SegmentFfi> segs,
    rust::Slice<const SubsegmentFfi> subs,
    rust::Slice<const float> extra,
    rust::Slice<const int32_t> tri_parts
) {
    auto shape =
        file.GetHeader().GetBlock<nifly::BSSubIndexTriShape>(shape_index);
    if (!shape || tri_parts.size() != shape->GetNumTriangles())
        return false;

    nifly::NifSegmentationInfo inf;
    inf.ssfFile.assign(reinterpret_cast<const char*>(ssf.data()), ssf.size());
    int part = 0;
    size_t sub_i = 0, extra_i = 0;
    for (const auto& seg : segs) {
        nifly::NifSegmentInfo info;
        info.partID = part++;
        for (uint32_t k = 0; k < seg.subsegments; k++, sub_i++) {
            if (sub_i >= subs.size())
                return false;
            const auto& s = subs[sub_i];
            if (s.extra_len > extra.size() - extra_i)
                return false;
            nifly::NifSubSegmentInfo sub;
            sub.partID = part++;
            sub.userSlotID = s.user_slot;
            sub.material = s.material;
            sub.extraData.assign(
                extra.begin() + extra_i,
                extra.begin() + extra_i + s.extra_len
            );
            extra_i += s.extra_len;
            info.subs.push_back(std::move(sub));
        }
        inf.segs.push_back(std::move(info));
    }
    if (sub_i != subs.size() || extra_i != extra.size())
        return false;
    for (int32_t p : tri_parts)
        if (p < -1 || p >= part)
            return false;

    std::vector<int> parts(tri_parts.begin(), tri_parts.end());
    file.SetShapeSegments(shape, inf, parts);
    return true;
}

// NifFile::RemoveEmptyPartitions on the shape at `shape_index`, which returns
// nothing. Returns how many partitions were removed, or -1, changing nothing,
// unless the shape has an NiSkinInstance with an NiSkinPartition and a
// BSDismemberSkinInstance has as many partitions (upstream erases its entries
// by partition index).
inline int64_t
remove_empty_partitions(nifly::NifFile& file, uint32_t shape_index) {
    auto& hdr = file.GetHeader();
    auto shape = hdr.GetBlock<nifly::NiShape>(shape_index);
    if (!shape)
        return -1;
    auto skinInst =
        hdr.GetBlock<nifly::NiSkinInstance>(shape->SkinInstanceRef());
    if (!skinInst)
        return -1;
    auto part = hdr.GetBlock(skinInst->skinPartitionRef);
    if (!part)
        return -1;
    auto dismember = dynamic_cast<nifly::BSDismemberSkinInstance*>(skinInst);
    if (dismember && dismember->partitions.size() != part->partitions.size())
        return -1;

    size_t before = part->partitions.size();
    file.RemoveEmptyPartitions(shape);
    return static_cast<int64_t>(before - part->partitions.size());
}

// NifFile::GetShapePartitions, reimplemented on a copy of the NiSkinPartition:
// upstream is const but fills the partition's cached triangle lists, and
// converts strip partitions to triangles. Fills `info` with each partition's
// flags and body part (defaults where there's no BSDismemberSkinInstance entry)
// and `tri_parts` with each triangle's partition, -1 for none. Returns false
// unless the shape has an NiSkinInstance with an NiSkinPartition.
inline bool shape_partitions(
    const nifly::NifFile& file,
    const nifly::NiShape& shape,
    rust::Vec<PartitionInfo>& info,
    rust::Vec<int32_t>& tri_parts
) {
    auto& hdr = const_cast<nifly::NifFile&>(file).GetHeader();
    auto skinInst =
        hdr.GetBlock<nifly::NiSkinInstance>(shape.SkinInstanceRef());
    if (!skinInst)
        return false;
    auto part = hdr.GetBlock(skinInst->skinPartitionRef);
    if (!part)
        return false;

    auto dismember = dynamic_cast<nifly::BSDismemberSkinInstance*>(skinInst);
    size_t known = 0;
    if (dismember) {
        for (auto& p : dismember->partitions) {
            info.push_back(
                PartitionInfo {static_cast<uint16_t>(p.flags), p.partID}
            );
            known++;
        }
    }
    uint16_t body = hdr.GetVersion().User() >= 12 ? 32 : 0;
    for (size_t i = known; i < part->partitions.size(); i++)
        info.push_back(PartitionInfo {nifly::PF_EDITOR_VISIBLE, body});

    // Upstream's GenerateTriPartsFromTrueTriangles, but with -1 rather than 0
    // for triangles in no partition.
    std::vector<nifly::Triangle> tris;
    shape.GetTriangles(tris);
    auto copy = part->Clone();
    copy->PrepareTrueTriangles();
    std::unordered_map<nifly::Triangle, int> index;
    for (size_t i = 0; i < tris.size(); i++) {
        auto t = tris[i];
        t.rot();
        index[t] = static_cast<int>(i);
    }
    std::vector<int> parts(tris.size(), -1);
    for (size_t p = 0; p < copy->partitions.size(); p++) {
        for (auto t : copy->partitions[p].trueTriangles) {
            t.rot();
            if (auto it = index.find(t); it != index.end())
                parts[it->second] = static_cast<int>(p);
        }
    }
    for (int p : parts)
        tri_parts.push_back(p);
    return true;
}

// NifFile::SetShapePartitions followed by UpdateSkinPartitions, which
// SetShapePartitions needs: it leaves every partition without its vertex map,
// bones and weights. Returns false, changing nothing, unless the partitions can
// be rebuilt (see updatable_skin), `tri_parts` has one entry per triangle, and
// each is below the number of partitions or -1 (unassigned, which upstream puts
// in an extra partition). Upstream checks neither, and leaves the partitions
// inconsistent with the triangles.
inline bool set_shape_partitions(
    nifly::NifFile& file,
    uint32_t shape_index,
    rust::Slice<const PartitionInfo> info,
    rust::Slice<const int32_t> tri_parts
) {
    std::vector<nifly::Triangle> tris;
    auto shape = updatable_skin(file, shape_index, tris);
    if (!shape || tri_parts.size() != tris.size())
        return false;
    for (int32_t p : tri_parts)
        if (p < -1 || (p >= 0 && static_cast<size_t>(p) >= info.size()))
            return false;

    nifly::NiVector<nifly::BSDismemberSkinInstance::PartitionInfo> partitions;
    for (const auto& i : info) {
        nifly::BSDismemberSkinInstance::PartitionInfo p;
        p.flags = static_cast<nifly::PartitionFlags>(i.flags);
        p.partID = i.body_part;
        partitions.push_back(p);
    }
    std::vector<int> parts(tri_parts.begin(), tri_parts.end());
    file.SetShapePartitions(shape, partitions, parts, true);
    file.UpdateSkinPartitions(shape);
    return true;
}

// NifFile::UpdatePartitionFlags on the shape at `shape_index`, which returns
// nothing. Returns false unless the shape has a BSDismemberSkinInstance and an
// NiSkinPartition with as many partitions: upstream indexes the skin
// partition by dismember partition.
inline bool update_partition_flags(nifly::NifFile& file, uint32_t shape_index) {
    auto& hdr = file.GetHeader();
    auto shape = hdr.GetBlock<nifly::NiShape>(shape_index);
    if (!shape || !shape->SkinInstanceRef())
        return false;
    auto dismember =
        hdr.GetBlock<nifly::BSDismemberSkinInstance>(shape->SkinInstanceRef());
    if (!dismember)
        return false;
    auto part = hdr.GetBlock(dismember->skinPartitionRef);
    if (!part || part->partitions.size() != dismember->partitions.size())
        return false;

    file.UpdatePartitionFlags(shape);
    return true;
}

inline rust::Vec<VertexWeight> shape_bone_weights(
    const nifly::NifFile& file,
    const nifly::NiShape& shape,
    uint32_t bone_index
) {
    std::unordered_map<uint16_t, float> weights;
    file.GetShapeBoneWeights(
        const_cast<nifly::NiShape*>(&shape),
        bone_index,
        weights
    );

    std::vector<VertexWeight> sorted;
    sorted.reserve(weights.size());
    for (const auto& [vertex, weight] : weights)
        sorted.push_back(VertexWeight {vertex, weight});
    std::sort(
        sorted.begin(),
        sorted.end(),
        [](const VertexWeight& a, const VertexWeight& b) {
            return a.vertex < b.vertex;
        }
    );

    rust::Vec<VertexWeight> out;
    out.reserve(sorted.size());
    for (const auto& w : sorted)
        out.push_back(w);
    return out;
}

// GetParentNode takes a non-const pointer but doesn't mutate the object.
// NifFile::GetParentNode on the block at `child_index`, as the parent's block
// index; NIF_NPOS if there's no such block or it has no parent.
inline uint32_t
parent_node_index(const nifly::NifFile& file, uint32_t child_index) {
    auto& hdr = file.GetHeader();
    auto child = hdr.GetBlock<nifly::NiObject>(child_index);
    if (!child)
        return nifly::NIF_NPOS;
    auto parent = file.GetParentNode(child);
    return parent ? hdr.GetBlockID(parent) : nifly::NIF_NPOS;
}

// SetParentNode, validated. Upstream adds any block to childRefs (which only
// hold NiAVObjects), creates cycles when the parent is in the child's subtree,
// and silently does nothing for bad pointers. NIF_NPOS for `parent_index`
// means the root node, as a null parent does upstream. Returns false, changing
// nothing, if the child isn't a NiAVObject, the parent isn't a node, they're
// the same block, or the parent is in the child's subtree.
inline bool set_parent_node(
    nifly::NifFile& file,
    uint32_t child_index,
    uint32_t parent_index
) {
    auto& hdr = file.GetHeader();
    auto child = hdr.GetBlock<nifly::NiAVObject>(child_index);
    nifly::NiNode* parent = parent_index == nifly::NIF_NPOS
        ? file.GetRootNode()
        : hdr.GetBlock<nifly::NiNode>(parent_index);
    if (!child || !parent || static_cast<nifly::NiObject*>(child) == parent)
        return false;

    // Walk the child's node subtree through child refs, looking for the parent.
    uint32_t parent_id = file.GetBlockID(parent);
    std::vector<bool> seen(hdr.GetNumBlocks());
    std::vector<uint32_t> stack {child_index};
    while (!stack.empty()) {
        uint32_t id = stack.back();
        stack.pop_back();
        if (id == parent_id)
            return false;
        if (id >= seen.size() || seen[id])
            continue;
        seen[id] = true;
        if (auto node = hdr.GetBlock<nifly::NiNode>(id))
            for (uint32_t i = 0; i < node->childRefs.GetSize(); i++)
                stack.push_back(node->childRefs.GetBlockRef(i));
    }

    file.SetParentNode(child, parent);
    return true;
}

// Replaces BSTreeNode `node_index`'s `bones1` (list 1) or `bones2` (list 2)
// with refs to `bones`. Returns false, changing nothing, if that block isn't a
// BSTreeNode or a bone isn't an NiNode.
inline bool set_tree_node_bones(
    nifly::NifFile& file,
    uint32_t node_index,
    uint8_t list,
    rust::Slice<const uint32_t> bones
) {
    auto& hdr = file.GetHeader();
    auto node = hdr.GetBlock<nifly::BSTreeNode>(node_index);
    if (!node)
        return false;
    for (uint32_t bone : bones)
        if (!hdr.GetBlock<nifly::NiNode>(bone))
            return false;

    auto& refs = list == 1 ? node->bones1 : node->bones2;
    refs.Clear();
    for (uint32_t bone : bones)
        refs.AddBlockRef(bone);
    return true;
}

// Refs to existing blocks, by RefKind (nif_file.rs).

// The ref of kind `kind` on `obj`, or null if it has none of that kind.
// `target_ok` checks a block can go in it.
struct RefSlot {
    nifly::NiRef* ref = nullptr;
    bool (*target_ok)(nifly::NiObject*) = nullptr;
};

template<class T>
bool is_a(nifly::NiObject* obj) {
    return dynamic_cast<T*>(obj) != nullptr;
}

template<class O, class T, class Get>
RefSlot ref_slot(nifly::NiObject& obj, Get get) {
    auto owner = dynamic_cast<O*>(&obj);
    if (!owner)
        return {};
    return {get(*owner), &is_a<T>};
}

// Slot `i` of a constraint's entity array, or none if it has fewer slots.
inline RefSlot
entity_slot(nifly::NiBlockRefArray<nifly::bhkEntity>& refs, uint32_t i) {
    if (refs.GetSize() <= i)
        return {};
    return {&*(refs.begin() + i), &is_a<nifly::bhkEntity>};
}

// Gives the entity arrays behind `kind` on `obj` their two slots, which every
// save writes, so the slots exist to set.
inline void make_entity_slots(nifly::NiObject& obj, RefKind kind) {
    using namespace nifly;
    if (kind == RefKind::EntityA || kind == RefKind::EntityB) {
        if (auto c = dynamic_cast<bhkConstraint*>(&obj))
            two_entity_slots(c->entityRefs);
    } else if (
        kind == RefKind::WrappedEntityA || kind == RefKind::WrappedEntityB
    ) {
        if (auto c = dynamic_cast<bhkBreakableConstraint*>(&obj))
            two_entity_slots(c->subConstraint.entityRefs);
        if (auto c = dynamic_cast<bhkMalleableConstraint*>(&obj))
            two_entity_slots(c->subConstraint.entityRefs);
    }
}

inline RefSlot find_ref(nifly::NiObject& obj, RefKind kind) {
    using namespace nifly;
    switch (kind) {
        case RefKind::Controller:
            return ref_slot<NiObjectNET, NiTimeController>(obj, [](auto& o) {
                return &o.controllerRef;
            });
        case RefKind::Collision:
            return ref_slot<NiAVObject, NiCollisionObject>(obj, [](auto& o) {
                return &o.collisionRef;
            });
        case RefKind::Shader:
            return ref_slot<NiShape, NiShader>(obj, [](auto& o) {
                return o.ShaderPropertyRef();
            });
        case RefKind::AlphaProperty:
            return ref_slot<NiShape, NiAlphaProperty>(obj, [](auto& o) {
                return o.AlphaPropertyRef();
            });
        case RefKind::SkinInstance:
            return ref_slot<NiShape, NiBoneContainer>(obj, [](auto& o) {
                return o.SkinInstanceRef();
            });
        case RefKind::TextureSet:
            return ref_slot<NiShader, BSShaderTextureSet>(obj, [](auto& o) {
                return o.TextureSetRef();
            });
        case RefKind::MultiBound:
            return ref_slot<BSMultiBoundNode, BSMultiBound>(obj, [](auto& o) {
                return &o.multiBoundRef;
            });
        case RefKind::MultiBoundData:
            return ref_slot<BSMultiBound, BSMultiBoundData>(obj, [](auto& o) {
                return &o.dataRef;
            });
        case RefKind::LodData:
            return ref_slot<NiLODNode, NiLODData>(obj, [](auto& o) {
                return &o.lodLevelData;
            });
        case RefKind::SkinData:
            return ref_slot<NiSkinInstance, NiSkinData>(obj, [](auto& o) {
                return &o.dataRef;
            });
        case RefKind::SkinPartition:
            return ref_slot<NiSkinInstance, NiSkinPartition>(obj, [](auto& o) {
                return &o.skinPartitionRef;
            });
        case RefKind::SkinBoneData:
            return ref_slot<BSSkinInstance, BSSkinBoneData>(obj, [](auto& o) {
                return &o.dataRef;
            });
        case RefKind::SkinDataPartition:
            return ref_slot<NiSkinData, NiSkinPartition>(obj, [](auto& o) {
                return &o.skinPartitionRef;
            });
        case RefKind::AdditionalData:
            return ref_slot<NiGeometryData, AdditionalGeomData>(
                obj,
                [](auto& o) { return &o.additionalDataRef; }
            );
        case RefKind::PixelData:
            return ref_slot<NiSourceTexture, TextureRenderData>(
                obj,
                [](auto& o) { return &o.dataRef; }
            );
        case RefKind::Palette:
            return ref_slot<TextureRenderData, NiPalette>(obj, [](auto& o) {
                return &o.paletteRef;
            });
        case RefKind::CollisionTarget:
            return ref_slot<NiCollisionObject, NiAVObject>(obj, [](auto& o) {
                return &o.targetRef;
            });
        case RefKind::CollisionBody:
            if (auto np = dynamic_cast<bhkNPCollisionObject*>(&obj))
                return {&np->bodyRef, [](NiObject* target) {
                            return is_a<bhkPhysicsSystem>(target)
                                || is_a<bhkRagdollSystem>(target);
                        }};
            return ref_slot<bhkNiCollisionObject, bhkWorldObject>(
                obj,
                [](auto& o) { return &o.bodyRef; }
            );
        case RefKind::HavokShape:
            if (auto ref = child_shape_ref(&obj))
                return {ref, &is_a<bhkShape>};
            return {};
        case RefKind::HingedBody:
            return ref_slot<bhkOrientHingedBodyAction, bhkRigidBody>(
                obj,
                [](auto& o) { return &o.bodyRef; }
            );
        case RefKind::CompressedMeshTarget:
            return ref_slot<bhkCompressedMeshShape, NiAVObject>(
                obj,
                [](auto& o) { return &o.targetRef; }
            );
        case RefKind::CompressedMeshData:
            return ref_slot<bhkCompressedMeshShape, bhkCompressedMeshShapeData>(
                obj,
                [](auto& o) { return &o.dataRef; }
            );
        case RefKind::EntityA:
        case RefKind::EntityB: {
            uint32_t i = kind == RefKind::EntityA ? 0 : 1;
            if (auto chain = dynamic_cast<bhkBallSocketConstraintChain*>(&obj))
                return {
                    i == 0 ? &chain->entityARef : &chain->entityBRef,
                    &is_a<bhkEntity>
                };
            if (auto c = dynamic_cast<bhkConstraint*>(&obj))
                return entity_slot(c->entityRefs, i);
            return {};
        }
        case RefKind::WrappedEntityA:
        case RefKind::WrappedEntityB: {
            uint32_t i = kind == RefKind::WrappedEntityA ? 0 : 1;
            if (auto c = dynamic_cast<bhkBreakableConstraint*>(&obj))
                return entity_slot(c->subConstraint.entityRefs, i);
            if (auto c = dynamic_cast<bhkMalleableConstraint*>(&obj))
                return entity_slot(c->subConstraint.entityRefs, i);
            return {};
        }
        case RefKind::PackedData:
            return ref_slot<bhkPackedNiTriStripsShape, hkPackedNiTriStripsData>(
                obj,
                [](auto& o) { return &o.dataRef; }
            );
        case RefKind::KeyData:
            if (auto i = dynamic_cast<NiBoolInterpolator*>(&obj))
                return {&i->dataRef, &is_a<NiBoolData>};
            if (auto i = dynamic_cast<NiFloatInterpolator*>(&obj))
                return {&i->dataRef, &is_a<NiFloatData>};
            if (auto i = dynamic_cast<NiPoint3Interpolator*>(&obj))
                return {&i->dataRef, &is_a<NiPosData>};
            if (auto i = dynamic_cast<NiTransformInterpolator*>(&obj))
                return {&i->dataRef, &is_a<NiTransformData>};
            if (auto i = dynamic_cast<BSTreadTransfInterpolator*>(&obj))
                return {&i->dataRef, &is_a<NiFloatData>};
            if (auto c = dynamic_cast<NiKeyframeController*>(&obj))
                return {&c->dataRef, &is_a<NiKeyframeData>};
            if (auto c = dynamic_cast<NiMaterialColorController*>(&obj))
                return {&c->dataRef, &is_a<NiPosData>};
            if (auto c = dynamic_cast<NiLightColorController*>(&obj))
                return {&c->dataRef, &is_a<NiPosData>};
            if (auto c = dynamic_cast<NiVisController*>(&obj))
                return {&c->dataRef, &is_a<NiVisData>};
            if (auto c = dynamic_cast<NiAlphaController*>(&obj))
                return {&c->dataRef, &is_a<NiFloatData>};
            if (auto c = dynamic_cast<NiRollController*>(&obj))
                return {&c->dataRef, &is_a<NiFloatData>};
            if (auto c = dynamic_cast<NiUVController*>(&obj))
                return {&c->dataRef, &is_a<NiUVData>};
            if (auto c = dynamic_cast<NiGeomMorpherController*>(&obj))
                return {&c->dataRef, &is_a<NiMorphData>};
            return {};
        case RefKind::PathData:
            if (auto i = dynamic_cast<NiPathInterpolator*>(&obj))
                return {&path_data_field(*i), &is_a<NiPosData>};
            return ref_slot<NiPathController, NiPosData>(obj, [](auto& o) {
                return &o.pathDataRef;
            });
        case RefKind::PercentData:
            if (auto i = dynamic_cast<NiPathInterpolator*>(&obj))
                return {&path_percent_field(*i), &is_a<NiFloatData>};
            return ref_slot<NiPathController, NiFloatData>(obj, [](auto& o) {
                return &o.percentDataRef;
            });
        case RefKind::LookAtNode:
            if (auto c = dynamic_cast<NiLookAtController*>(&obj))
                return {&c->lookAtNodePtr, &is_a<NiNode>};
            return ref_slot<NiLookAtInterpolator, NiNode>(obj, [](auto& o) {
                return &o.lookAtRef;
            });
        case RefKind::LookAtTranslation:
            return ref_slot<NiLookAtInterpolator, NiPoint3Interpolator>(
                obj,
                [](auto& o) { return &o.translateInterpRef; }
            );
        case RefKind::LookAtRoll:
            return ref_slot<NiLookAtInterpolator, NiFloatInterpolator>(
                obj,
                [](auto& o) { return &o.rollInterpRef; }
            );
        case RefKind::LookAtScale:
            return ref_slot<NiLookAtInterpolator, NiFloatInterpolator>(
                obj,
                [](auto& o) { return &o.scaleInterpRef; }
            );
        case RefKind::SingleInterpolator:
            return ref_slot<NiBlendInterpolator, NiInterpolator>(
                obj,
                [](auto& o) { return &o.singleInterpolatorRef; }
            );
        case RefKind::SplineData:
            return ref_slot<NiBSplineInterpolator, NiBSplineData>(
                obj,
                [](auto& o) { return &o.splineDataRef; }
            );
        case RefKind::SplineBasis:
            return ref_slot<NiBSplineInterpolator, NiBSplineBasisData>(
                obj,
                [](auto& o) { return &o.basisDataRef; }
            );
        case RefKind::NextController:
            return ref_slot<NiTimeController, NiTimeController>(
                obj,
                [](auto& o) { return &o.nextControllerRef; }
            );
        case RefKind::ControllerTarget:
            return ref_slot<NiTimeController, NiObjectNET>(obj, [](auto& o) {
                return &o.targetRef;
            });
        case RefKind::Interpolator:
            if (auto c = dynamic_cast<BSFrustumFOVController*>(&obj))
                return {&c->interpolatorRef, &is_a<NiInterpolator>};
            return ref_slot<NiSingleInterpController, NiInterpolator>(
                obj,
                [](auto& o) { return &o.interpolatorRef; }
            );
        case RefKind::LightningGeneration:
            return ref_slot<BSProceduralLightningController, NiInterpolator>(
                obj,
                [](auto& o) { return &o.generationInterpRef; }
            );
        case RefKind::LightningMutation:
            return ref_slot<BSProceduralLightningController, NiInterpolator>(
                obj,
                [](auto& o) { return &o.mutationInterpRef; }
            );
        case RefKind::LightningSubdivision:
            return ref_slot<BSProceduralLightningController, NiInterpolator>(
                obj,
                [](auto& o) { return &o.subdivisionInterpRef; }
            );
        case RefKind::LightningBranches:
            return ref_slot<BSProceduralLightningController, NiInterpolator>(
                obj,
                [](auto& o) { return &o.numBranchesInterpRef; }
            );
        case RefKind::LightningBranchVariation:
            return ref_slot<BSProceduralLightningController, NiInterpolator>(
                obj,
                [](auto& o) { return &o.numBranchesVarInterpRef; }
            );
        case RefKind::LightningLength:
            return ref_slot<BSProceduralLightningController, NiInterpolator>(
                obj,
                [](auto& o) { return &o.lengthInterpRef; }
            );
        case RefKind::LightningLengthVariation:
            return ref_slot<BSProceduralLightningController, NiInterpolator>(
                obj,
                [](auto& o) { return &o.lengthVarInterpRef; }
            );
        case RefKind::LightningWidth:
            return ref_slot<BSProceduralLightningController, NiInterpolator>(
                obj,
                [](auto& o) { return &o.widthInterpRef; }
            );
        case RefKind::LightningArcOffset:
            return ref_slot<BSProceduralLightningController, NiInterpolator>(
                obj,
                [](auto& o) { return &o.arcOffsetInterpRef; }
            );
        case RefKind::LightningShader:
            return ref_slot<BSProceduralLightningController, BSShaderProperty>(
                obj,
                [](auto& o) { return &o.shaderPropertyRef; }
            );
        default:
            return {};
    }
}

// Whether the shape at `target` is the one at `root` or contained in it,
// through child shape and sub-shape refs.
inline bool
in_shape_tree(const nifly::NiHeader& hdr, uint32_t root, uint32_t target) {
    using namespace nifly;
    std::vector<bool> seen(hdr.GetNumBlocks());
    std::vector<uint32_t> stack {root};
    while (!stack.empty()) {
        uint32_t id = stack.back();
        stack.pop_back();
        if (id == target)
            return true;
        if (id >= seen.size() || seen[id])
            continue;
        seen[id] = true;
        auto obj = hdr.GetBlock<bhkShape>(id);
        if (!obj)
            continue;
        if (auto ref = child_shape_ref(obj))
            stack.push_back(ref->index);
        if (auto list = dynamic_cast<bhkListShape*>(obj))
            for (uint32_t i = 0; i < list->subShapeRefs.GetSize(); i++)
                stack.push_back(list->subShapeRefs.GetBlockRef(i));
        if (auto list = dynamic_cast<bhkConvexListShape*>(obj))
            for (uint32_t i = 0; i < list->shapeRefs.GetSize(); i++)
                stack.push_back(list->shapeRefs.GetBlockRef(i));
    }
    return false;
}

// Whether the controller chain from `from`, through next controller refs,
// reaches the block at `target`. Stops at a repeat.
inline bool controller_chain_reaches(
    const nifly::NiHeader& hdr,
    uint32_t from,
    uint32_t target
) {
    std::set<uint32_t> seen;
    for (uint32_t at = from; at != nifly::NIF_NPOS;) {
        if (at == target)
            return true;
        auto c = hdr.GetBlock<nifly::NiTimeController>(at);
        if (!c || !seen.insert(at).second)
            return false;
        at = c->nextControllerRef.index;
    }
    return false;
}

// The block index in the `kind` ref of the block at `owner`; NIF_NPOS if it's
// empty or the block has no such ref.
inline uint32_t
block_ref(const nifly::NifFile& file, uint32_t owner, RefKind kind) {
    auto obj = file.GetHeader().GetBlock<nifly::NiObject>(owner);
    if (!obj)
        return nifly::NIF_NPOS;
    RefSlot slot = find_ref(*obj, kind);
    return slot.ref ? slot.ref->index : nifly::NIF_NPOS;
}

// Points the `kind` ref of the block at `owner` at `target`, or empties it if
// `target` is NIF_NPOS. Returns false, changing nothing, if the block has no
// such ref or `target` is the wrong type.
inline bool set_block_ref(
    nifly::NifFile& file,
    uint32_t owner,
    RefKind kind,
    uint32_t target
) {
    auto& hdr = file.GetHeader();
    auto obj = hdr.GetBlock<nifly::NiObject>(owner);
    if (!obj)
        return false;
    // Entity slots may need creating, so check the target first.
    bool entity = kind == RefKind::EntityA || kind == RefKind::EntityB
        || kind == RefKind::WrappedEntityA || kind == RefKind::WrappedEntityB;
    if (entity && target != nifly::NIF_NPOS
        && !is_a<nifly::bhkEntity>(hdr.GetBlock<nifly::NiObject>(target)))
        return false;
    if (entity)
        make_entity_slots(*obj, kind);
    RefSlot slot = find_ref(*obj, kind);
    if (!slot.ref)
        return false;
    if (target != nifly::NIF_NPOS) {
        auto block = hdr.GetBlock<nifly::NiObject>(target);
        if (!block || !slot.target_ok(block))
            return false;
        if (kind == RefKind::HavokShape && in_shape_tree(hdr, target, owner))
            return false;
        if (kind == RefKind::SingleInterpolator
            && interpolator_uses(hdr, target, owner))
            return false;
        if (kind == RefKind::NextController
            && controller_chain_reaches(hdr, target, owner))
            return false;
    }
    slot.ref->index = target;
    return true;
}

// Points an NiGeometry's data ref at the NiGeometryData at `target`. Shapes
// that cache their data (NiTriShape, NiTriStrips, NiLines, BSLODTriShape and
// subtypes) only take their own data type, and the cache follows. Returns
// false, changing nothing, otherwise.
inline bool
set_shape_geometry_data(nifly::NifFile& file, uint32_t shape, uint32_t target) {
    using namespace nifly;
    auto& hdr = file.GetHeader();
    auto geom = hdr.GetBlock<NiGeometry>(shape);
    auto data = hdr.GetBlock<NiGeometryData>(target);
    if (!geom || !data)
        return false;
    bool cached = geom->HasType<NiTriShape>() || geom->HasType<NiTriStrips>()
        || geom->HasType<NiLines>() || geom->HasType<BSLODTriShape>();
    if (cached) {
        geom->SetGeomData(data);
        if (geom->GetGeomData() != data)
            return false;
    }
    geom->DataRef()->index = target;
    return true;
}

// Whether `target` is the node at `root` or in its node subtree, through
// child refs.
inline bool
in_node_subtree(const nifly::NiHeader& hdr, uint32_t root, uint32_t target) {
    std::vector<bool> seen(hdr.GetNumBlocks());
    std::vector<uint32_t> stack {root};
    while (!stack.empty()) {
        uint32_t id = stack.back();
        stack.pop_back();
        if (id == target)
            return true;
        if (id >= seen.size() || seen[id])
            continue;
        seen[id] = true;
        if (auto node = hdr.GetBlock<nifly::NiNode>(id))
            for (uint32_t i = 0; i < node->childRefs.GetSize(); i++)
                stack.push_back(node->childRefs.GetBlockRef(i));
    }
    return false;
}

template<class T>
rust::Vec<uint32_t> ref_array(const nifly::NiBlockRefArray<T>& refs) {
    rust::Vec<uint32_t> out;
    for (uint32_t i = 0; i < refs.GetSize(); i++)
        out.push_back(refs.GetBlockRef(i));
    return out;
}

template<class T>
void set_ref_array(
    nifly::NiBlockRefArray<T>& refs,
    rust::Slice<const uint32_t> targets
) {
    refs.Clear();
    for (uint32_t id : targets)
        refs.AddBlockRef(id);
}

// The `list` refs of the block at `owner`, in order, empty ones included.
// The extra data chain is followed from its head until a ref is empty, isn't
// extra data or repeats. Empty if the block has no such list.
inline rust::Vec<uint32_t>
block_ref_list(const nifly::NifFile& file, uint32_t owner, RefList list) {
    using namespace nifly;
    auto& hdr = file.GetHeader();
    auto obj = hdr.GetBlock<NiObject>(owner);
    switch (list) {
        case RefList::Children:
            if (auto node = dynamic_cast<NiNode*>(obj))
                return ref_array(node->childRefs);
            break;
        case RefList::Properties:
            if (auto av = dynamic_cast<NiAVObject*>(obj))
                return ref_array(av->propertyRefs);
            break;
        case RefList::ExtraData:
            if (auto net = dynamic_cast<NiObjectNET*>(obj))
                return ref_array(net->extraDataRefs);
            break;
        case RefList::ExtraDataChain:
            if (auto net = dynamic_cast<NiObjectNET*>(obj)) {
                rust::Vec<uint32_t> out;
                std::set<uint32_t> seen;
                auto extra = hdr.GetBlock<NiExtraData>(net->extraDataRef);
                while (extra && seen.insert(hdr.GetBlockID(extra)).second) {
                    out.push_back(hdr.GetBlockID(extra));
                    extra = hdr.GetBlock<NiExtraData>(extra->nextExtraDataRef);
                }
                return out;
            }
            break;
        case RefList::Bones:
            if (auto skin = dynamic_cast<NiBoneContainer*>(obj))
                return ref_array(skin->boneRefs);
            break;
        case RefList::ParticleSystems:
            if (auto master = dynamic_cast<BSMasterParticleSystem*>(obj))
                return ref_array(master->particleSysRefs);
            break;
        case RefList::Constraints:
            if (auto body = dynamic_cast<bhkRigidBody*>(obj))
                return ref_array(body->constraintRefs);
            break;
        case RefList::SubShapes:
            if (auto list = dynamic_cast<bhkListShape*>(obj))
                return ref_array(list->subShapeRefs);
            if (auto list = dynamic_cast<bhkConvexListShape*>(obj))
                return ref_array(list->shapeRefs);
            break;
        case RefList::StripsData:
            if (auto shape = dynamic_cast<bhkNiTriStripsShape*>(obj))
                return ref_array(shape->partRefs);
            break;
        case RefList::ChainedEntities:
            if (auto chain = dynamic_cast<bhkBallSocketConstraintChain*>(obj))
                return ref_array(chain->chainedEntityRefs);
            break;
        case RefList::RagdollBones:
            if (auto t = dynamic_cast<bhkRagdollTemplate*>(obj))
                return ref_array(t->boneRefs);
            break;
        case RefList::FlipSources:
            if (auto c = dynamic_cast<NiFlipController*>(obj))
                return ref_array(c->sourceRefs);
            break;
        case RefList::ExtraTargets:
            if (auto c = dynamic_cast<NiMultiTargetTransformController*>(obj))
                return ref_array(c->targetRefs);
            break;
        case RefList::Controllers:
            if (auto net = dynamic_cast<NiObjectNET*>(obj)) {
                rust::Vec<uint32_t> out;
                std::set<uint32_t> seen;
                auto c = hdr.GetBlock<NiTimeController>(net->controllerRef);
                while (c && seen.insert(hdr.GetBlockID(c)).second) {
                    out.push_back(hdr.GetBlockID(c));
                    c = hdr.GetBlock<NiTimeController>(c->nextControllerRef);
                }
                return out;
            }
            break;
        default:
            break;
    }
    return {};
}

// The bone count of a skin's data, or -1 if it has none.
inline int64_t
skin_data_bone_count(const nifly::NiHeader& hdr, nifly::NiBoneContainer& skin) {
    if (auto inst = dynamic_cast<nifly::NiSkinInstance*>(&skin))
        if (auto data = hdr.GetBlock<nifly::NiSkinData>(inst->dataRef))
            return static_cast<int64_t>(data->bones.size());
    if (auto inst = dynamic_cast<nifly::BSSkinInstance*>(&skin))
        if (auto data = hdr.GetBlock<nifly::BSSkinBoneData>(inst->dataRef))
            return static_cast<int64_t>(data->boneXforms.size());
    return -1;
}

template<class T>
bool all_are(const nifly::NiHeader& hdr, rust::Slice<const uint32_t> ids) {
    for (uint32_t id : ids)
        if (!hdr.GetBlock<T>(id))
            return false;
    return true;
}

// Replaces the `list` refs of the block at `owner` with `targets`. Children
// must be NiAVObjects that aren't the node or above it; properties
// NiProperty; extra data NiExtraData or BSExtraData, and the chain
// NiExtraData, and the controller chain NiTimeController. Chains link each target to the
// next, so they must be distinct. Bones are nodes, as many as the skin's data
// has. Returns false, changing nothing, if a target doesn't fit or the block
// has no such list.
inline bool set_block_ref_list(
    nifly::NifFile& file,
    uint32_t owner,
    RefList list,
    rust::Slice<const uint32_t> targets
) {
    using namespace nifly;
    auto& hdr = file.GetHeader();
    auto obj = hdr.GetBlock<NiObject>(owner);
    switch (list) {
        case RefList::Children: {
            auto node = dynamic_cast<NiNode*>(obj);
            if (!node || !all_are<NiAVObject>(hdr, targets))
                return false;
            for (uint32_t id : targets)
                if (in_node_subtree(hdr, id, owner))
                    return false;
            set_ref_array(node->childRefs, targets);
            return true;
        }
        case RefList::Properties: {
            auto av = dynamic_cast<NiAVObject*>(obj);
            if (!av || !all_are<NiProperty>(hdr, targets))
                return false;
            set_ref_array(av->propertyRefs, targets);
            return true;
        }
        case RefList::ExtraData: {
            auto net = dynamic_cast<NiObjectNET*>(obj);
            if (!net)
                return false;
            for (uint32_t id : targets)
                if (!hdr.GetBlock<NiExtraData>(id)
                    && !hdr.GetBlock<BSExtraData>(id))
                    return false;
            set_ref_array(net->extraDataRefs, targets);
            return true;
        }
        case RefList::ExtraDataChain: {
            auto net = dynamic_cast<NiObjectNET*>(obj);
            if (!net || !all_are<NiExtraData>(hdr, targets))
                return false;
            std::set<uint32_t> distinct(targets.begin(), targets.end());
            if (distinct.size() != targets.size())
                return false;
            net->extraDataRef.index = targets.empty() ? NIF_NPOS : targets[0];
            for (size_t i = 0; i < targets.size(); i++)
                hdr.GetBlock<NiExtraData>(targets[i])->nextExtraDataRef.index =
                    i + 1 < targets.size() ? targets[i + 1] : NIF_NPOS;
            return true;
        }
        case RefList::Bones: {
            auto skin = dynamic_cast<NiBoneContainer*>(obj);
            if (!skin || !all_are<NiNode>(hdr, targets))
                return false;
            int64_t count = skin_data_bone_count(hdr, *skin);
            if (count >= 0 && size_t(count) != targets.size())
                return false;
            set_ref_array(skin->boneRefs, targets);
            return true;
        }
        case RefList::ParticleSystems: {
            auto master = dynamic_cast<BSMasterParticleSystem*>(obj);
            if (!master || !all_are<NiAVObject>(hdr, targets))
                return false;
            set_ref_array(master->particleSysRefs, targets);
            return true;
        }
        case RefList::Constraints: {
            auto body = dynamic_cast<bhkRigidBody*>(obj);
            if (!body)
                return false;
            for (uint32_t id : targets)
                if (!hdr.GetBlock<bhkConstraint>(id)
                    && !hdr.GetBlock<bhkBallSocketConstraintChain>(id))
                    return false;
            set_ref_array(body->constraintRefs, targets);
            return true;
        }
        case RefList::SubShapes: {
            for (uint32_t id : targets)
                if (in_shape_tree(hdr, id, owner))
                    return false;
            if (auto list = dynamic_cast<bhkListShape*>(obj)) {
                if (!all_are<bhkShape>(hdr, targets))
                    return false;
                if (list->filters.size() == list->subShapeRefs.GetSize())
                    list->filters.resize(static_cast<uint32_t>(targets.size()));
                set_ref_array(list->subShapeRefs, targets);
                return true;
            }
            if (auto list = dynamic_cast<bhkConvexListShape*>(obj)) {
                if (!all_are<bhkConvexShape>(hdr, targets))
                    return false;
                set_ref_array(list->shapeRefs, targets);
                return true;
            }
            return false;
        }
        case RefList::StripsData: {
            auto shape = dynamic_cast<bhkNiTriStripsShape*>(obj);
            if (!shape || !all_are<NiTriStripsData>(hdr, targets))
                return false;
            if (shape->filters.size() == shape->partRefs.GetSize())
                shape->filters.resize(static_cast<uint32_t>(targets.size()));
            set_ref_array(shape->partRefs, targets);
            return true;
        }
        case RefList::ChainedEntities: {
            auto chain = dynamic_cast<bhkBallSocketConstraintChain*>(obj);
            if (!chain || !all_are<bhkRigidBody>(hdr, targets))
                return false;
            set_ref_array(chain->chainedEntityRefs, targets);
            return true;
        }
        case RefList::RagdollBones: {
            auto t = dynamic_cast<bhkRagdollTemplate*>(obj);
            if (!t || !all_are<bhkRagdollTemplateData>(hdr, targets))
                return false;
            set_ref_array(t->boneRefs, targets);
            return true;
        }
        case RefList::FlipSources: {
            auto c = dynamic_cast<NiFlipController*>(obj);
            if (!c || !all_are<NiSourceTexture>(hdr, targets))
                return false;
            set_ref_array(c->sourceRefs, targets);
            return true;
        }
        case RefList::ExtraTargets: {
            auto c = dynamic_cast<NiMultiTargetTransformController*>(obj);
            if (!c || targets.size() > UINT16_MAX)
                return false;
            for (uint32_t id : targets)
                if (id != NIF_NPOS && !hdr.GetBlock<NiAVObject>(id))
                    return false;
            set_ref_array(c->targetRefs, targets);
            return true;
        }
        case RefList::Controllers: {
            auto net = dynamic_cast<NiObjectNET*>(obj);
            if (!net || !all_are<NiTimeController>(hdr, targets))
                return false;
            std::set<uint32_t> distinct(targets.begin(), targets.end());
            if (distinct.size() != targets.size())
                return false;
            net->controllerRef.index = targets.empty() ? NIF_NPOS : targets[0];
            for (size_t i = 0; i < targets.size(); i++)
                hdr.GetBlock<NiTimeController>(targets[i])
                    ->nextControllerRef.index =
                    i + 1 < targets.size() ? targets[i + 1] : NIF_NPOS;
            return true;
        }
        default:
            return false;
    }
}
} // namespace nifly_rs
