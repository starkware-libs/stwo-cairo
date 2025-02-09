#![feature(
    trait_upcasting,
    portable_simd,
    iter_array_chunks,
    array_chunks,
    raw_slice_split
)]
// TODO(Ohad): remove.
#![allow(clippy::too_many_arguments)]

pub mod adapter;
pub mod cairo_air;
pub mod components;
pub mod felt;
pub mod relations;
