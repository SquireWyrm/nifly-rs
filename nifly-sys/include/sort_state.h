#pragma once

#include "NifFile.hpp"

namespace nifly_rs {
// Wraps NifFile::SortState, the state PrettySortBlocks and SetShapeOrder pass
// through SetSortIndices. newIndices maps each old block index to its new one.
// In its own header because the generated nif_file.rs.h needs it before
// nif_file_shim.h is complete.
struct SortState {
    nifly::NifFile::SortState inner;
};
} // namespace nifly_rs
