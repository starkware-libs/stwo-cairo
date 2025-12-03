#![allow(unused)]
use std::array::from_fn;

use num_traits::{One, Zero};
use starknet_types_core::curve::ProjectivePoint;
use stwo::prover::backend::simd::conversion::{Pack, Unpack};
use stwo::prover::backend::simd::m31::PackedM31;
use stwo_cairo_common::preprocessed_columns::pedersen::{PEDERSEN_TABLE_18, PEDERSEN_TABLE_9};
use stwo_cairo_common::prover_types::cpu::{Felt252, M31};
use stwo_cairo_common::prover_types::simd::PackedFelt252;

pub struct PackedPedersenPointsTableBitsPerWindow9 {}
impl PackedPedersenPointsTableBitsPerWindow9 {
    pub fn deduce_output([input]: [PackedM31; 1]) -> [PackedFelt252; 2] {
        let arr = input
            .to_array()
            .map(|i| PEDERSEN_TABLE_9.get_row_coordinates(i.0 as usize));
        <_ as Pack>::pack(arr)
    }
}

pub struct PackedPedersenPointsTableBitsPerWindow18 {}
impl PackedPedersenPointsTableBitsPerWindow18 {
    pub fn deduce_output([input]: [PackedM31; 1]) -> [PackedFelt252; 2] {
        let arr = input
            .to_array()
            .map(|i| PEDERSEN_TABLE_18.get_row_coordinates(i.0 as usize));
        <_ as Pack>::pack(arr)
    }
}

#[derive(Debug)]
pub struct PartialEcMul<const NUM_WINDOWS: usize> {}
impl<const NUM_WINDOWS: usize> PartialEcMul<NUM_WINDOWS> {
    type PartialEcMulState = ([M31; NUM_WINDOWS], [Felt252; 2]);

    pub fn deduce_output(
        chain: M31,
        round: M31,
        (m_shifted, accumulator): PartialEcMulState,
    ) -> (M31, M31, PartialEcMulState) {
        assert_eq!(252 % NUM_WINDOWS, 0);
        let bits_per_window = 252 / NUM_WINDOWS;
        let rows_per_window = 1 << bits_per_window;

        let round_usize = round.0 as usize;
        assert!(round_usize < 2 * NUM_WINDOWS);

        let accumulator_point =
            ProjectivePoint::from_affine(accumulator[0].into(), accumulator[1].into())
                .expect("The accumulator should contain a curve point");

        let window_value = m_shifted[0].0 as usize;
        let table_row = round_usize * rows_per_window + window_value;
        let affine_point = match bits_per_window {
            14 => PEDERSEN_TABLE_18.get_row(table_row),
            28 => PEDERSEN_TABLE_9.get_row(table_row),
            _ => panic!("Unsupported bits_per_window value {BITS_PER_WINDOW}"),
        };
        let table_point = ProjectivePoint::from_affine(affine_point.x, affine_point.y)
            .expect("Table point should be on curve");

        let new_accumulator_point = (accumulator_point + table_point)
            .to_affine()
            .expect("New accumulator is not infinity");
        let new_accumulator_x: Felt252 = new_accumulator_point.x().into();
        let new_accumulator_y: Felt252 = new_accumulator_point.y().into();

        let new_m_shifted: [M31; NUM_WINDOWS] = from_fn(|i| {
            if i < m_shifted.len() - 1 {
                m_shifted[i + 1]
            } else {
                M31::zero()
            }
        });

        (
            chain,
            round + M31::one(),
            (new_m_shifted, [new_accumulator_x, new_accumulator_y]),
        )
    }
}

pub struct PackedPartialEcMul<const NUM_WINDOWS: usize> {}
impl<const NUM_WINDOWS: usize> PackedPartialEcMul<NUM_WINDOWS> {
    pub fn deduce_output(
        input: (
            PackedM31,
            PackedM31,
            ([PackedM31; NUM_WINDOWS], [PackedFelt252; 2]),
        ),
    ) -> (
        PackedM31,
        PackedM31,
        ([PackedM31; NUM_WINDOWS], [PackedFelt252; 2]),
    ) {
        let unpacked_inputs = input.unpack();
        <_ as Pack>::pack(unpacked_inputs.map(|(chain, round, state)| {
            PartialEcMul::<NUM_WINDOWS>::deduce_output(chain, round, state)
        }))
    }
}

pub type PackedPartialEcMulBitsPerWindow18 = PackedPartialEcMul<14>;
pub type PackedPartialEcMulBitsPerWindow9 = PackedPartialEcMul<28>;

#[cfg(test)]
#[generic_tests::define]
mod tests {
    use starknet_curve::curve_params::{PEDERSEN_P0, PEDERSEN_P1, PEDERSEN_P2, SHIFT_POINT};
    use starknet_types_core::curve::ProjectivePoint;
    use starknet_types_core::felt::Felt;
    use stwo_cairo_common::prover_types::cpu::M31;

    use super::PartialEcMul;

    #[test]
    fn test_deduce_output<const NUM_WINDOWS: usize>() {
        let bits_per_window = 252 / NUM_WINDOWS;

        let chain = M31::from_u32_unchecked(1234);
        let round = M31::from_u32_unchecked((NUM_WINDOWS + 1) as u32);
        let mut m_shifted = [M31::from_u32_unchecked(0); NUM_WINDOWS];
        m_shifted[0] = M31::from_u32_unchecked(56);
        m_shifted[1] = M31::from_u32_unchecked(99);
        let accumulator = [PEDERSEN_P1.x().into(), PEDERSEN_P1.y().into()];

        let (new_chain, new_round, (new_m_shifted, new_accumulator)) =
            PartialEcMul::<NUM_WINDOWS>::deduce_output(chain, round, (m_shifted, accumulator));

        let mut expected_new_m_shifted = [M31::from_u32_unchecked(0); NUM_WINDOWS];
        expected_new_m_shifted[0] = M31::from_u32_unchecked(99);

        let p1 = ProjectivePoint::from_affine(PEDERSEN_P1.x(), PEDERSEN_P1.y()).unwrap();
        let p2 = ProjectivePoint::from_affine(PEDERSEN_P2.x(), PEDERSEN_P2.y()).unwrap();
        let shift_point = ProjectivePoint::from_affine(SHIFT_POINT.x(), SHIFT_POINT.y()).unwrap();
        let expected_new_accumulator = (p1 + &p2 * Felt::from(56 << bits_per_window) - shift_point)
            .to_affine()
            .unwrap();

        assert_eq!(new_chain, chain);
        assert_eq!(new_round, round + M31::from_u32_unchecked(1));
        assert_eq!(new_m_shifted, expected_new_m_shifted);
        assert_eq!(expected_new_accumulator.x(), new_accumulator[0].into());
        assert_eq!(expected_new_accumulator.y(), new_accumulator[1].into());
    }

    #[test]
    fn test_deduce_output_high_window<const NUM_WINDOWS: usize>() {
        let bits_per_window = 252 / NUM_WINDOWS;
        let bits_in_last_window = bits_per_window - 4;
        let chain = M31::from_u32_unchecked(1234);
        let round = M31::from_u32_unchecked((NUM_WINDOWS - 1) as u32);
        let mut m_shifted = [M31::from_u32_unchecked(0); NUM_WINDOWS];
        m_shifted[0] = M31::from_u32_unchecked(((2 << bits_in_last_window) + 5) as u32);
        m_shifted[1] = M31::from_u32_unchecked(99);
        let accumulator = [PEDERSEN_P1.x().into(), PEDERSEN_P1.y().into()];

        let (new_chain, new_round, (new_m_shifted, new_accumulator)) =
            PartialEcMul::<NUM_WINDOWS>::deduce_output(chain, round, (m_shifted, accumulator));

        let mut expected_new_m_shifted = [M31::from_u32_unchecked(0); NUM_WINDOWS];
        expected_new_m_shifted[0] = M31::from_u32_unchecked(99);

        let p0 = ProjectivePoint::from_affine(PEDERSEN_P0.x(), PEDERSEN_P0.y()).unwrap();
        let p1 = ProjectivePoint::from_affine(PEDERSEN_P1.x(), PEDERSEN_P1.y()).unwrap();
        let shift_point = ProjectivePoint::from_affine(SHIFT_POINT.x(), SHIFT_POINT.y()).unwrap();
        let shifted_p0 = &p0
            * (Felt::from(1u128 << (bits_per_window * (NUM_WINDOWS / 2)))
                * Felt::from(1u128 << (bits_per_window * (NUM_WINDOWS / 2 - 1))));
        let expected_new_accumulator = (&p1 * Felt::from(3) + &shifted_p0 * Felt::from(5)
            - shift_point)
            .to_affine()
            .unwrap();

        assert_eq!(new_chain, chain);
        assert_eq!(new_round, round + M31::from_u32_unchecked(1));
        assert_eq!(new_m_shifted, expected_new_m_shifted);
        assert_eq!(expected_new_accumulator.x(), new_accumulator[0].into());
        assert_eq!(expected_new_accumulator.y(), new_accumulator[1].into());
    }

    #[instantiate_tests(<28>)]
    mod small_window {}

    #[instantiate_tests(<14>)]
    mod large_window {}
}
