#![allow(unused)]
use std::array::from_fn;

use cairo_air::pedersen::const_columns::PEDERSEN_TABLE;
use num_traits::{One, Zero};
use starknet_types_core::curve::ProjectivePoint;
use stwo_cairo_common::preprocessed_consts::pedersen::{NUM_WINDOWS, ROWS_PER_WINDOW};
use stwo_cairo_common::prover_types::cpu::{Felt252, M31};

type PartialEcMulState = (M31, [M31; 14], [Felt252; 2]);

#[derive(Debug)]
pub struct PartialEcMul {}
impl PartialEcMul {
    pub fn deduce_output(
        chain: M31,
        round: M31,
        (table_offset, m_shifted, accumulator): PartialEcMulState,
    ) -> (M31, M31, PartialEcMulState) {
        let round_usize = round.0 as usize;
        let table_offset_usize = table_offset.0 as usize;
        assert!(round_usize < NUM_WINDOWS);

        let accumulator_point =
            ProjectivePoint::from_affine(accumulator[0].into(), accumulator[1].into())
                .expect("The accumulator should contain a curve point");

        let window_value = m_shifted[0].0 as usize;
        let table_row = table_offset_usize + round_usize * ROWS_PER_WINDOW + window_value;
        let table_point: ProjectivePoint = PEDERSEN_TABLE
            .get_row(table_row)
            .try_into()
            .expect("Table point should be on curve");

        let new_accumulator_point = (accumulator_point + table_point)
            .to_affine()
            .expect("New accumulator is not infinity");
        let new_accumulator_x: Felt252 = new_accumulator_point.x().into();
        let new_accumulator_y: Felt252 = new_accumulator_point.y().into();

        let new_m_shifted: [M31; 14] = from_fn(|i| {
            if i < m_shifted.len() - 1 {
                m_shifted[i + 1]
            } else {
                M31::zero()
            }
        });

        (
            chain,
            round + M31::one(),
            (
                table_offset,
                new_m_shifted,
                [new_accumulator_x, new_accumulator_y],
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use starknet_curve::curve_params::{PEDERSEN_P1, PEDERSEN_P2, SHIFT_POINT};
    use starknet_types_core::curve::ProjectivePoint;
    use starknet_types_core::felt::Felt;
    use stwo_cairo_common::preprocessed_consts::pedersen::{BITS_PER_WINDOW, P2_SECTION_START};
    use stwo_cairo_common::prover_types::cpu::M31;

    use super::PartialEcMul;

    #[test]
    fn test_deduce_output() {
        let chain = M31::from_u32_unchecked(1234);
        let round = M31::from_u32_unchecked(1);
        let table_offset = M31::from_u32_unchecked(P2_SECTION_START as u32);
        let mut m_shifted = [M31::from_u32_unchecked(0); 14];
        m_shifted[0] = M31::from_u32_unchecked(5678);
        m_shifted[1] = M31::from_u32_unchecked(9999);
        let accumulator = [PEDERSEN_P1.x().into(), PEDERSEN_P1.y().into()];

        let (new_chain, new_round, (new_table_offset, new_m_shifted, new_accumulator)) =
            PartialEcMul::deduce_output(chain, round, (table_offset, m_shifted, accumulator));

        let mut expected_new_m_shifted = [M31::from_u32_unchecked(0); 14];
        expected_new_m_shifted[0] = M31::from_u32_unchecked(9999);

        let p1 = ProjectivePoint::from_affine(PEDERSEN_P1.x(), PEDERSEN_P1.y()).unwrap();
        let p2 = ProjectivePoint::from_affine(PEDERSEN_P2.x(), PEDERSEN_P2.y()).unwrap();
        let shift_point = ProjectivePoint::from_affine(SHIFT_POINT.x(), SHIFT_POINT.y()).unwrap();
        let expected_new_accumulator = (p1 + &p2 * Felt::from(5678 << BITS_PER_WINDOW)
            - shift_point)
            .to_affine()
            .unwrap();

        assert_eq!(new_chain, chain);
        assert_eq!(new_round, round + M31::from_u32_unchecked(1));
        assert_eq!(new_table_offset, table_offset);
        assert_eq!(new_m_shifted, expected_new_m_shifted);
        assert_eq!(expected_new_accumulator.x(), new_accumulator[0].into());
        assert_eq!(expected_new_accumulator.y(), new_accumulator[1].into());
    }
}
