#![allow(unused)]
use std::array::from_fn;

use num_traits::{One, Zero};
use starknet_types_core::curve::ProjectivePoint;
use stwo::prover::backend::simd::conversion::{Pack, Unpack};
use stwo::prover::backend::simd::m31::PackedM31;
use stwo_cairo_common::preprocessed_columns::pedersen::{
    BITS_PER_WINDOW, NUM_WINDOWS, PEDERSEN_TABLE, ROWS_PER_WINDOW,
};
use stwo_cairo_common::prover_types::cpu::{Felt252, M31};
use stwo_cairo_common::prover_types::simd::PackedFelt252;

type PartialEcMulState = (Felt252, [Felt252; 2]);

pub struct PackedPedersenPointsTable {}
impl PackedPedersenPointsTable {
    pub fn deduce_output([input]: [PackedM31; 1]) -> [PackedFelt252; 2] {
        let arr = input
            .to_array()
            .map(|i| PEDERSEN_TABLE.get_row_coordinates(i.0 as usize));
        <_ as Pack>::pack(arr)
    }
}

#[derive(Debug)]
pub struct PartialEcMul {}
impl PartialEcMul {
    pub fn deduce_output(
        chain: M31,
        round: M31,
        (m_shifted, accumulator): PartialEcMulState,
    ) -> (M31, M31, PartialEcMulState) {
        let round_usize = round.0 as usize;
        assert!(round_usize < 2 * NUM_WINDOWS);

        let accumulator_point =
            ProjectivePoint::from_affine(accumulator[0].into(), accumulator[1].into())
                .expect("The accumulator should contain a curve point");

        let window_value = m_shifted.get_m31(0).0 as usize;
        let table_row = round_usize * ROWS_PER_WINDOW + window_value;
        let affine_point = PEDERSEN_TABLE.get_row(table_row);
        let table_point = ProjectivePoint::from_affine(affine_point.x, affine_point.y)
            .expect("Table point should be on curve");

        let new_accumulator_point = (accumulator_point + table_point)
            .to_affine()
            .expect("New accumulator is not infinity");
        let new_accumulator_x: Felt252 = new_accumulator_point.x().into();
        let new_accumulator_y: Felt252 = new_accumulator_point.y().into();

        let new_m_shifted = (m_shifted
            - Felt252 {
                limbs: [window_value as u64, 0, 0, 0],
            })
            / Felt252 {
                limbs: [1 << BITS_PER_WINDOW, 0, 0, 0],
            };

        (
            chain,
            round + M31::one(),
            (new_m_shifted, [new_accumulator_x, new_accumulator_y]),
        )
    }
}

pub struct PackedPartialEcMul {}
impl PackedPartialEcMul {
    pub fn deduce_output(
        input: (PackedM31, PackedM31, (PackedFelt252, [PackedFelt252; 2])),
    ) -> (PackedM31, PackedM31, (PackedFelt252, [PackedFelt252; 2])) {
        let unpacked_inputs = input.unpack();
        <_ as Pack>::pack(
            unpacked_inputs
                .map(|(chain, round, state)| PartialEcMul::deduce_output(chain, round, state)),
        )
    }
}

#[cfg(test)]
mod tests {
    use starknet_curve::curve_params::{PEDERSEN_P0, PEDERSEN_P1, PEDERSEN_P2, SHIFT_POINT};
    use starknet_types_core::curve::ProjectivePoint;
    use starknet_types_core::felt::Felt;
    use stwo_cairo_common::preprocessed_columns::pedersen::{BITS_PER_WINDOW, NUM_WINDOWS};
    use stwo_cairo_common::prover_types::cpu::{Felt252, M31};

    use super::PartialEcMul;

    #[test]
    fn test_deduce_output() {
        let chain = M31::from_u32_unchecked(1234);
        let round = M31::from_u32_unchecked((NUM_WINDOWS + 1) as u32);
        let m_shifted = Felt252 {
            limbs: [56 + (99 << 9), 0, 0, 0],
        };
        let accumulator = [PEDERSEN_P1.x().into(), PEDERSEN_P1.y().into()];

        let (new_chain, new_round, (new_m_shifted, new_accumulator)) =
            PartialEcMul::deduce_output(chain, round, (m_shifted, accumulator));

        let expected_new_m_shifted = Felt252 {
            limbs: [99, 0, 0, 0],
        };

        let p1 = ProjectivePoint::from_affine(PEDERSEN_P1.x(), PEDERSEN_P1.y()).unwrap();
        let p2 = ProjectivePoint::from_affine(PEDERSEN_P2.x(), PEDERSEN_P2.y()).unwrap();
        let shift_point = ProjectivePoint::from_affine(SHIFT_POINT.x(), SHIFT_POINT.y()).unwrap();
        let expected_new_accumulator = (p1 + &p2 * Felt::from(56 << BITS_PER_WINDOW) - shift_point)
            .to_affine()
            .unwrap();

        assert_eq!(new_chain, chain);
        assert_eq!(new_round, round + M31::from_u32_unchecked(1));
        assert_eq!(new_m_shifted, expected_new_m_shifted);
        assert_eq!(expected_new_accumulator.x(), new_accumulator[0].into());
        assert_eq!(expected_new_accumulator.y(), new_accumulator[1].into());
    }

    #[test]
    fn test_deduce_output_high_window() {
        let chain = M31::from_u32_unchecked(1234);
        let round = M31::from_u32_unchecked((NUM_WINDOWS - 1) as u32);
        let m_shifted = Felt252 {
            limbs: [5 + (2 << 5) + (99 << 9), 0, 0, 0],
        };
        let accumulator = [PEDERSEN_P1.x().into(), PEDERSEN_P1.y().into()];

        let (new_chain, new_round, (new_m_shifted, new_accumulator)) =
            PartialEcMul::deduce_output(chain, round, (m_shifted, accumulator));

        let expected_new_m_shifted = Felt252 {
            limbs: [99, 0, 0, 0],
        };

        let p0 = ProjectivePoint::from_affine(PEDERSEN_P0.x(), PEDERSEN_P0.y()).unwrap();
        let p1 = ProjectivePoint::from_affine(PEDERSEN_P1.x(), PEDERSEN_P1.y()).unwrap();
        let shift_point = ProjectivePoint::from_affine(SHIFT_POINT.x(), SHIFT_POINT.y()).unwrap();
        let shifted_p0 = &p0
            * (Felt::from(1u128 << (BITS_PER_WINDOW * 14))
                * Felt::from(1u128 << (BITS_PER_WINDOW * 13)));
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
}
