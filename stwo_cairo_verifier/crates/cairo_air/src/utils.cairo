use core::array::ToSpanTrait;
use core::iter::{IntoIterator, Iterator};
use core::num::traits::WrappingMul;
use core::traits::DivRem;
use stwo_verifier_core::TreeArray;
use stwo_verifier_core::fields::m31::M31;
use stwo_verifier_core::utils::pow2;
#[cfg(not(feature: "qm31_opcode"))]
use crate::{Invertible, QM31, Zero};
use super::components::memory_id_to_big;

#[generate_trait]
pub impl UsizeImpl of UsizeExTrait {
    /// Calculates the quotient of `self` and `other`, rounding the result towards positive
    /// infinity.
    ///
    /// # Panics
    ///
    /// This function will panic if `other` is zero.
    fn div_ceil(self: usize, other: usize) -> usize {
        let (d, r) = DivRem::div_rem(self, other.try_into().unwrap());
        if r > 0 {
            d + 1
        } else {
            d
        }
    }
}

pub fn tree_array_concat_cols(tree_array: Array<TreeArray<Span<u32>>>) -> TreeArray<Span<u32>> {
    let mut tree0 = array![];
    let mut tree1 = array![];
    let mut tree2 = array![];

    for curr_tree in tree_array.span() {
        // TODO: Instead of changing this to make it generic just refactor so the
        // whole function can be removed.
        assert!(curr_tree.len() <= 3);

        if curr_tree.len() > 0 {
            tree0.append_span(*curr_tree[0]);
        }
        if curr_tree.len() > 1 {
            tree1.append_span(*curr_tree[1]);
        }
        if curr_tree.len() > 2 {
            tree2.append_span(*curr_tree[2]);
        }
    }

    array![tree0.span(), tree1.span(), tree2.span()]
}

/// Splits a 252 bit dense representation into felts, each with `N_BITS_PER_FELT` bits.
pub fn split_f252(x: [u32; 8]) -> [M31; memory_id_to_big::N_M31_IN_FELT252] {
    let segments: [u32; memory_id_to_big::N_M31_IN_FELT252] = split(x);
    let mut m31_segments = array![];

    for segment in segments.span() {
        m31_segments.append((*segment).try_into().unwrap());
    }

    (*m31_segments.span().try_into().unwrap()).unbox()
}

/// Splits a 32N bit dense representation into felts, each with N_BITS_PER_FELT bits.
///
/// Parameters:
/// - `N`: the number of 32-bit words in the input.
/// - `M`: the number of felts in the output.
/// - `x`: the input dense representation.
fn split<
    const N: usize,
    const M: usize,
    impl FixedArrayToSpan: ToSpanTrait<[u32; N], u32>,
    impl SpanTryIntoFixedArray: TryInto<Span<u32>, @Box<[u32; M]>>,
>(
    x: [u32; N],
) -> [u32; M] {
    let mut res = array![];
    let mut n_bits_in_word = 32;
    let mut word_iter = FixedArrayToSpan::span(@x).into_iter();
    let mut word = *word_iter.next().unwrap_or(@0);

    let shift = pow2(memory_id_to_big::N_BITS_PER_FELT).try_into().unwrap();
    for _ in 0..M {
        if n_bits_in_word > memory_id_to_big::N_BITS_PER_FELT {
            let (high, low) = DivRem::div_rem(word, shift);
            res.append(low);
            word = high;
            n_bits_in_word -= memory_id_to_big::N_BITS_PER_FELT;
            continue;
        }

        let mut segment = word;
        // Fetch next word.
        word = *word_iter.next().unwrap_or(@0);

        // If we need more bits to fill, take from next word.
        if n_bits_in_word < memory_id_to_big::N_BITS_PER_FELT {
            let (_high, low) = DivRem::div_rem(
                WrappingMul::wrapping_mul(word, pow2(n_bits_in_word)), shift,
            );
            segment = segment + low;
            word /= pow2(memory_id_to_big::N_BITS_PER_FELT - n_bits_in_word);
        }

        res.append(segment);

        n_bits_in_word += 32 - memory_id_to_big::N_BITS_PER_FELT;
    }

    (*SpanTryIntoFixedArray::try_into(res.span()).unwrap()).unbox()
}

/// Sums the inverses of the given QM31 values.
/// Uses Montgomery's trick to compute the inverses efficiently.
#[cfg(not(feature: "qm31_opcode"))]
pub fn sum_inverses_qm31(values: @Array<QM31>) -> QM31 {
    let mut values = values.span();
    let first_value = *values.pop_front().expect('values cannot be empty');
    let mut prefix_mul = array![first_value];

    // First pass.
    let mut curr_mul = first_value;
    for value in values {
        let mul = curr_mul * *value;
        prefix_mul.append(mul);
        curr_mul = mul;
    }
    let mut prefix_mul = prefix_mul.span();

    // Inverse cumulative product.
    let mut curr_inverse = prefix_mul.pop_back().unwrap().inverse();

    // Second pass.
    let mut sum = Zero::zero();
    while let (Some(prefix), Some(value)) = (prefix_mul.pop_back(), values.pop_back()) {
        sum += *prefix * curr_inverse;
        curr_inverse *= *value;
    }

    sum + curr_inverse
}
