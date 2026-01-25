use core::array::ToSpanTrait;
use core::iter::{IntoIterator, Iterator};
use core::num::traits::WrappingMul;
use core::traits::DivRem;
use stwo_verifier_core::TreeArray;
use stwo_verifier_core::fields::m31::M31;
use stwo_verifier_core::fields::qm31::{QM31, QM31Trait, QM31_EXTENSION_DEGREE};
use stwo_verifier_core::utils::pow2;
#[cfg(not(feature: "qm31_opcode"))]
use crate::{Invertible, Zero};
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

/// Assumes all values are reduced mod M31 and packs them into QM31 elements.
pub fn pack_into_qm31s(mut values: Span<u32>) -> Span<QM31> {
    let mut res = array![];

    while let Some(chunk) = values.multi_pop_front::<QM31_EXTENSION_DEGREE>() {
        append_chunk(ref res, chunk.unbox());
    }

    if !values.is_empty() {
        let mut chunk = array![];
        let chunk_size = values.len();
        chunk.append_span(values);
        for _ in chunk_size..QM31_EXTENSION_DEGREE {
            chunk.append(0_u32);
        }
        let fixed_arr: [u32; QM31_EXTENSION_DEGREE] = (*chunk.span().try_into().unwrap()).unbox();
        append_chunk(ref res, fixed_arr);
    }

    res.span()
}

fn append_chunk(ref array: Array<QM31>, chunk: [u32; QM31_EXTENSION_DEGREE]) {
    let [v0, v1, v2, v3] = chunk;

    let new_qm31 = QM31Trait::from_fixed_array(
        [
            v0.try_into().unwrap(), v1.try_into().unwrap(), v2.try_into().unwrap(),
            v3.try_into().unwrap(),
        ],
    );
    array.append(new_qm31);
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
pub fn split<
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

/// Interpret the mask values as a single `QM31` value.
pub fn as_qm31(mask_values: @Box<[Span<QM31>; 4]>) -> QM31 {
    let [column_0, column_1, column_2, column_3]: [Span<QM31>; 4] = mask_values.unbox();

    let [coeff_0]: [QM31; 1] = (*column_0.try_into().unwrap()).unbox();
    let [coeff_1]: [QM31; 1] = (*column_1.try_into().unwrap()).unbox();
    let [coeff_2]: [QM31; 1] = (*column_2.try_into().unwrap()).unbox();
    let [coeff_3]: [QM31; 1] = (*column_3.try_into().unwrap()).unbox();

    QM31Trait::from_partial_evals([coeff_0, coeff_1, coeff_2, coeff_3])
}

/// Interpret the mask values as two neighboring `QM31` values.
pub fn as_neighboring_qm31s(mask_values: @Box<[Span<QM31>; 4]>) -> [QM31; 2] {
    let [column_0, column_1, column_2, column_3]: [Span<QM31>; 4] = mask_values.unbox();

    let [coeff_0_first, coeff_0_second]: [QM31; 2] = (*column_0.try_into().unwrap()).unbox();
    let [coeff_1_first, coeff_1_second]: [QM31; 2] = (*column_1.try_into().unwrap()).unbox();
    let [coeff_2_first, coeff_2_second]: [QM31; 2] = (*column_2.try_into().unwrap()).unbox();
    let [coeff_3_first, coeff_3_second]: [QM31; 2] = (*column_3.try_into().unwrap()).unbox();

    [
        QM31Trait::from_partial_evals([coeff_0_first, coeff_1_first, coeff_2_first, coeff_3_first]),
        QM31Trait::from_partial_evals(
            [coeff_0_second, coeff_1_second, coeff_2_second, coeff_3_second],
        ),
    ]
}
