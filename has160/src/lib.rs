#![no_std]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs, rust_2018_idioms)]

pub use digest::{self, Digest};

use core::{convert::TryInto, fmt, slice::from_ref};
use digest::{
    array::Array,
    block_buffer::Eager,
    core_api::{
        AlgorithmName, Block, BlockSizeUser, Buffer, BufferKindUser, CoreWrapper, FixedOutputCore,
        OutputSizeUser, Reset, UpdateCore,
    },
    crypto_common::hazmat::{DeserializeStateError, SerializableState, SerializedState},
    typenum::{Unsigned, U20, U28, U64},
    HashMarker, Output,
};

mod compress;

pub use compress::compress;

const STATE_LEN: usize = 5;
const BLOCK_SIZE: usize = <Has160Core as BlockSizeUser>::BlockSize::USIZE;

pub struct Has160Core {
    h: [u32; STATE_LEN],
    block_len: u64,
}

pub type Has160 = CoreWrapper<Has160Core>;

impl Default for Has160Core {
    #[inline]
    fn default() -> Self {
        Self {
            h: [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0],
            block_len: 0,
        }
    }
}

impl Reset for Has160Core {
    #[inline]
    fn reset(&mut self) {
        *self = Default::default();
    }
}

impl AlgorithmName for Has160Core {
    fn write_alg_name(f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Has160")
    }
}

impl fmt::Debug for Has160Core {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Has160Core { ... }")
    }
}

impl BlockSizeUser for Has160Core {
    type BlockSize = U64;
}
