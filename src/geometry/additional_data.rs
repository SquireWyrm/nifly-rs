//! Additional geometry data.

use super::*;

/// A channel of an [`AdditionalGeomData`]: one kind of per-vertex data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DataChannel {
    /// What the data is.
    pub data_type: u32,
    /// Bytes per vertex.
    pub unit_size: u32,
    /// Bytes for all vertices: `unit_size` times the vertex count.
    pub total_size: u32,
    /// Bytes per vertex of all the channels in the same block.
    pub stride: u32,
    /// The block holding the channel, by position.
    pub block: u32,
    /// Where the channel starts in each vertex's bytes.
    pub offset: u32,
    /// Bit 0 keeps the data; bits 1–2 are how often it changes (0 may change, 1 never, 2
    /// every frame).
    pub raw_flags: u8,
}

impl Default for DataChannel {
    /// All zero, except raw flags of 2: data that never changes.
    fn default() -> Self {
        Self { data_type: 0, unit_size: 0, total_size: 0, stride: 0, block: 0, offset: 0, raw_flags: 2 }
    }
}

impl DataChannel {
    fn from_ffi(c: ffi::AdditionalDataChannel) -> Self {
        Self {
            data_type: c.data_type as u32,
            unit_size: c.unit_size,
            total_size: c.total_size,
            stride: c.stride,
            block: c.block,
            offset: c.offset,
            raw_flags: c.flags,
        }
    }

    fn to_ffi(self) -> ffi::AdditionalDataChannel {
        ffi::AdditionalDataChannel {
            data_type: self.data_type as i32,
            unit_size: self.unit_size,
            total_size: self.total_size,
            stride: self.stride,
            block: self.block,
            offset: self.offset,
            flags: self.raw_flags,
        }
    }
}

/// A block of an [`NiAdditionalGeometryData`].
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct AdditionalDataBlock {
    /// The size of each item of `data`.
    pub block_size: u32,
    pub block_offsets: Vec<u32>,
    /// One per item of `data`.
    pub data_sizes: Vec<u32>,
    /// Items of `block_size` bytes.
    pub data: Vec<Vec<u8>>,
}

/// A block of a [`BsPackedAdditionalGeometryData`].
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct PackedDataBlock {
    /// `None` if the block has no data.
    pub data: Option<PackedData>,
    pub shader_index: u32,
    pub total_size: u32,
}

/// The data of a [`PackedDataBlock`].
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct PackedData {
    pub block_offsets: Vec<u32>,
    pub atom_sizes: Vec<u32>,
    pub bytes: Vec<u8>,
}

impl AdditionalGeomData {
    /// Usually the vertex count of the [`NiGeometryData`] it belongs to.
    pub fn vertex_count(&self) -> u16 {
        ffi::agd_vertex_count(&self.0)
    }

    /// Sets the [vertex count](Self::vertex_count).
    pub fn set_vertex_count(&mut self, count: u16) {
        ffi::set_agd_vertex_count(self.ffi_mut(), count);
    }

    pub fn channels(&self) -> Vec<DataChannel> {
        ffi::agd_channels(&self.0).into_iter().map(DataChannel::from_ffi).collect()
    }

    /// Replaces the [channels](Self::channels). They aren't checked against the blocks.
    pub fn set_channels(&mut self, channels: &[DataChannel]) {
        let channels: Vec<_> = channels.iter().map(|c| c.to_ffi()).collect();
        ffi::set_agd_channels(self.ffi_mut(), &channels);
    }
}

impl NiAdditionalGeometryData {
    /// The data blocks, `None` for those without data.
    pub fn blocks(&self) -> Vec<Option<AdditionalDataBlock>> {
        let d = &self.0;
        (0..ffi::agd_block_count(d))
            .map(|i| {
                ffi::agd_block_has_data(d, i).then(|| {
                    let data_sizes = ffi::agd_block_data_sizes(d, i).to_vec();
                    AdditionalDataBlock {
                        block_size: ffi::agd_block_size(d, i),
                        block_offsets: ffi::agd_block_offsets(d, i).to_vec(),
                        data: (0..data_sizes.len()).map(|j| ffi::agd_block_data(d, i, j).to_vec()).collect(),
                        data_sizes,
                    }
                })
            })
            .collect()
    }

    /// Replaces the [blocks](Self::blocks). Returns `false`, changing nothing, if a block's
    /// `data` and `data_sizes` lengths differ or an item of `data` isn't `block_size` bytes.
    pub fn set_blocks(&mut self, blocks: &[Option<AdditionalDataBlock>]) -> bool {
        let valid = |b: &AdditionalDataBlock| {
            b.data.len() == b.data_sizes.len() && b.data.iter().all(|d| d.len() == b.block_size as usize)
        };
        if !blocks.iter().flatten().all(valid) {
            return false;
        }
        ffi::clear_agd_blocks(self.ffi_mut());
        for block in blocks {
            match block {
                Some(b) => ffi::push_agd_block(
                    self.ffi_mut(),
                    true,
                    b.block_size,
                    &b.block_offsets,
                    &b.data_sizes,
                    &b.data.concat(),
                ),
                None => ffi::push_agd_block(self.ffi_mut(), false, 0, &[], &[], &[]),
            }
        }
        true
    }
}

impl BsPackedAdditionalGeometryData {
    pub fn blocks(&self) -> Vec<PackedDataBlock> {
        let d = &self.0;
        (0..ffi::packed_agd_block_count(d))
            .map(|i| {
                let mut ints = [0; 2];
                ffi::packed_agd_block_ints(d, i, &mut ints);
                PackedDataBlock {
                    data: ffi::packed_agd_block_has_data(d, i).then(|| PackedData {
                        block_offsets: ffi::packed_agd_block_offsets(d, i).to_vec(),
                        atom_sizes: ffi::packed_agd_block_atom_sizes(d, i).to_vec(),
                        bytes: ffi::packed_agd_block_data(d, i).to_vec(),
                    }),
                    shader_index: ints[0],
                    total_size: ints[1],
                }
            })
            .collect()
    }

    /// Replaces the [blocks](Self::blocks).
    pub fn set_blocks(&mut self, blocks: &[PackedDataBlock]) {
        ffi::clear_packed_agd_blocks(self.ffi_mut());
        for b in blocks {
            let empty = PackedData::default();
            let data = b.data.as_ref().unwrap_or(&empty);
            ffi::push_packed_agd_block(
                self.ffi_mut(),
                b.data.is_some(),
                &data.block_offsets,
                &data.atom_sizes,
                &data.bytes,
                b.shader_index,
                b.total_size,
            );
        }
    }
}

/// An additional data type that [`Edit::assign_additional_data`] can create.
///
/// This trait is sealed.
pub trait NewAdditionalData: NiType {
    #[doc(hidden)]
    const PACKED: bool;
}

impl NewAdditionalData for NiAdditionalGeometryData {
    const PACKED: bool = false;
}

impl NewAdditionalData for BsPackedAdditionalGeometryData {
    const PACKED: bool = true;
}
