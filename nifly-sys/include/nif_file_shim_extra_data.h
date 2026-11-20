// Part of nif_file_shim.h: assigning extra data.
#pragma once

#include "nif_file_shim.h"

namespace nifly_rs {
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
} // namespace nifly_rs
