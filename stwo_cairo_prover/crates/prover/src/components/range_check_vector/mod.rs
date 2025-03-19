use std::iter::zip;
use std::simd::Simd;

pub mod component;
pub mod component_prover;

use stwo_prover::core::backend::simd::m31::N_LANES;

pub const SIMD_ENUMERATION_0: Simd<u32, N_LANES> =
    Simd::from_array([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);

/// Partitions a number into 'N' bit segments.
///
/// For example: partition_into_bit_segments(0b110101010, [3, 4, 2]) -> [0b110, 0b1010, 0b10]
///
///
/// # Arguments
pub fn partition_into_bit_segments<const N: usize>(
    mut value: Simd<u32, N_LANES>,
    n_bits_per_segment: [u32; N],
) -> [Simd<u32, N_LANES>; N] {
    let mut segments = [Simd::splat(0); N];
    for (segment, segment_n_bits) in zip(&mut segments, n_bits_per_segment).rev() {
        let mask = Simd::splat((1 << segment_n_bits) - 1);
        *segment = value & mask;
        value >>= segment_n_bits;
    }
    segments
}

// Used for getting array sizes as consts.
#[macro_export]
macro_rules! count_elements {
    ($x:expr) => (1);
    ($x:expr, $($xs:expr),*) => (1 + $crate::count_elements!($($xs),*));
}

#[macro_export]
macro_rules! generate_range_check_code {
    ([$($log_range:expr),+]) => {
        paste::paste!{
            pub mod [<range_check_$($log_range)_*>] {
                $crate::range_check_eval!($($log_range),+);
                $crate::range_check_prover!($($log_range),+);
            }
        }
    };
}

generate_range_check_code!([6]);
generate_range_check_code!([11]);
generate_range_check_code!([12]);
generate_range_check_code!([18]);
generate_range_check_code!([19]);
generate_range_check_code!([3, 6]);
generate_range_check_code!([4, 3]);
generate_range_check_code!([4, 4]);
generate_range_check_code!([9, 9]);
generate_range_check_code!([7, 2, 5]);
generate_range_check_code!([3, 6, 6, 3]);
generate_range_check_code!([4, 4, 4, 4]);
generate_range_check_code!([3, 3, 3, 3, 3]);
