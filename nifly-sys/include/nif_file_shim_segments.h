// Part of nif_file_shim.h: segments and dismember partitions.
#pragma once

#include "nif_file_shim.h"

namespace nifly_rs {
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
} // namespace nifly_rs
