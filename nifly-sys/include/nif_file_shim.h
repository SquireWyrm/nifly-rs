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
#include "particles_shim.h"
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
// (mesh_writer.h), BlendInterpolatorFactory, FloatExtraDataControllerFactory
// (animation_shim.h) and the ReplacementFactory replacements.
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
        regs[nifly::NiPSysSphereEmitter::BlockName] = std::make_unique<
            ReplacementFactory<Emitter<nifly::NiPSysSphereEmitter>>>();
        regs[nifly::NiPSysCylinderEmitter::BlockName] = std::make_unique<
            ReplacementFactory<Emitter<nifly::NiPSysCylinderEmitter>>>();
        regs[nifly::NiPSysBoxEmitter::BlockName] = std::make_unique<
            ReplacementFactory<Emitter<nifly::NiPSysBoxEmitter>>>();
        regs[nifly::BSPSysArrayEmitter::BlockName] = std::make_unique<
            ReplacementFactory<Emitter<nifly::BSPSysArrayEmitter>>>();
        regs[nifly::NiPSysMeshEmitter::BlockName] = std::make_unique<
            ReplacementFactory<Emitter<nifly::NiPSysMeshEmitter>>>();
        regs[nifly::NiPSysRotationModifier::BlockName] =
            std::make_unique<ReplacementFactory<RotationModifier>>();
        regs[nifly::NiMeshPSysData::BlockName] =
            std::make_unique<ReplacementFactory<MeshPSysData>>();
        regs[nifly::NiBlendBoolInterpolator::BlockName] = std::make_unique<
            BlendInterpolatorFactory<nifly::NiBlendBoolInterpolator>>();
        regs[nifly::NiBlendFloatInterpolator::BlockName] = std::make_unique<
            BlendInterpolatorFactory<nifly::NiBlendFloatInterpolator>>();
        regs[nifly::NiBlendPoint3Interpolator::BlockName] = std::make_unique<
            BlendInterpolatorFactory<nifly::NiBlendPoint3Interpolator>>();
        regs[nifly::NiBlendTransformInterpolator::BlockName] = std::make_unique<
            BlendInterpolatorFactory<nifly::NiBlendTransformInterpolator>>();
        regs[nifly::NiSequence::BlockName] =
            std::make_unique<ReplacementFactory<Sequence>>();
        regs[nifly::NiControllerSequence::BlockName] =
            std::make_unique<ReplacementFactory<ControllerSequence>>();
        regs[nifly::BSAnimNote::BlockName] =
            std::make_unique<ReplacementFactory<AnimNote>>();
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
    write_bridge::RustOStream out(writer);
    out.exceptions(std::ios::badbit);
    nifly::NifSaveOptions nifly_options;
    nifly_options.optimize = options.optimize;
    nifly_options.sortBlocks = options.sort_blocks;
    int code = nif.Save(out, nifly_options);
    out.flush();
    return code;
}

inline Vec3 vec3_to_ffi(const nifly::Vector3& v) {
    return Vec3 {v.x, v.y, v.z};
}
} // namespace nifly_rs

// The rest of the shims, by topic.
// In this order: later parts use earlier ones.
// clang-format off
#include "nif_file_shim_blocks.h"
#include "nif_file_shim_extra_data.h"
#include "nif_file_shim_nodes.h"
#include "nif_file_shim_shaders.h"
#include "nif_file_shim_shape_data.h"
#include "nif_file_shim_skin.h"
#include "nif_file_shim_shapes.h"
#include "nif_file_shim_segments.h"
#include "nif_file_shim_refs.h"
#include "nif_file_shim_ref_lists.h"
// clang-format on
