/// Auto-generated file. Any changes will be overwritten.
pub use anyhow::Result;
pub use shared::{
    constants::*, structs::{Address, MetadataLayer, Region, Size},
    traits::{Decoder, Encoder},
};
pub use std::{path::Path, sync::Arc};
pub use zarrs::{
    array::{codec::GzipCodec, Array, ArrayBuilder, DataType, FillValue},
    array_subset::ArraySubset, filesystem::FilesystemStore, group::GroupBuilder,
};
pub fn interleave(channels: &[u8], output: &mut Vec<u8>) {
    let rs = &channels[..TILE_LENGTH];
    let gs = &channels[TILE_LENGTH..TILE_LENGTH * 2];
    let bs = &channels[TILE_LENGTH * 2..];
    output.extend(rs.iter().zip(gs).zip(bs).flat_map(|((&r, &g), &b)| [r, g, b]));
}
