// Part of nif_file_shim.h: block access, sorting and deleting unreferenced blocks.
#pragma once

#include "nif_file_shim.h"

namespace nifly_rs {
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
} // namespace nifly_rs
