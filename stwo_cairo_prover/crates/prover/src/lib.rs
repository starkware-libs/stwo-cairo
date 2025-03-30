#![feature(
    trait_upcasting,
    portable_simd,
    iter_array_chunks,
    array_chunks,
    raw_slice_split
)]
// TODO(Ohad): remove.
#![allow(clippy::too_many_arguments, clippy::module_inception)]

pub mod cairo_air;
mod felt;
pub mod prover;

pub use stwo_prover;
pub mod witness;
