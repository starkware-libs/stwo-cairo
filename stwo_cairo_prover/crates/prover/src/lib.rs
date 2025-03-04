#![feature(
    trait_upcasting,
    portable_simd,
    iter_array_chunks,
    array_chunks,
    raw_slice_split
)]
// TODO(Ohad): remove.
#![allow(clippy::too_many_arguments)]

pub mod cairo_air;
mod components;
mod felt;
pub use stwo_prover;
