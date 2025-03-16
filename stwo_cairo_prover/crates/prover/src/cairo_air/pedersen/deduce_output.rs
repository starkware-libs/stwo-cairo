use std::array::from_fn;

use num_traits::{One, Zero};
use starknet_types_core::curve::ProjectivePoint;
use stwo_cairo_common::preprocessed_consts::pedersen::{NUM_WINDOWS, ROWS_PER_WINDOW};
use stwo_cairo_common::prover_types::cpu::{Felt252, M31};
use stwo_cairo_common::prover_types::simd::{PackedFelt252, N_LANES};
// use stwo_prover::core::backend::simd::conversion::Unpack;
use stwo_prover::core::backend::simd::m31::PackedM31;

use super::const_columns::PEDERSEN_TABLE;

type PartialEcMulState = (M31, [M31; 14], [Felt252; 2]);
// type PackedPartialEcMulState = (PackedM31, [PackedM31; 14], [PackedFelt252; 2]);

// impl Unpack for PackedPartialEcMulState {
//     type CpuType = PartialEcMulState;
//     fn unpack(self) -> [PartialEcMulState; N_LANES] {
//         self.to_array()
//     }
// }

#[derive(Copy, Clone, Debug)]
pub struct WrapperPartialEcMul(
    pub PackedM31,
    pub PackedM31,
    pub (PackedM31, [PackedM31; 14], [PackedFelt252; 2]),
);

// impl Unpack for WrapperPartialEcMul {
//     type CpuType = (M31, M31, (M31, [M31; 14], [Felt252; 2]));
//     fn unpack(self) -> [(M31, M31, (M31, [M31; 14], [Felt252; 2])); N_LANES] {
//         let chain_arr = self.0.to_array();
//         let round_arr = self.1.to_array();
//         let table_offset_arr = self.2 .0.to_array();
//         let m_shifted_arr1: [[M31; N_LANES]; 14] = from_fn(|i| self.2 .1[i].to_array()); //
// N_LANES         let m_shifted_arr2: [[M31; 14]; N_LANES] =
//             from_fn(|i| from_fn(|j| m_shifted_arr1[j][i].clone())); // N_LANES
//         let accumulator_arr1: [[Felt252; N_LANES]; 2] = from_fn(|i| self.2 .2[i].to_array()); //
// N_LANES         let accumulator_arr2: [[Felt252; 2]; N_LANES] =
//             from_fn(|i| from_fn(|j| accumulator_arr1[j][i].clone())); // N_LANES
//         from_fn(|i| {
//             (
//                 chain_arr[i].clone(),
//                 round_arr[i].clone(),
//                 (
//                     table_offset_arr[i].clone(),
//                     m_shifted_arr2[i].clone(),
//                     accumulator_arr2[i].clone(),
//                     // from_fn(|j| m_shifted_arr2[j][i].clone()),
//                     // from_fn(|j| accumulator_arr2[j][i].clone()),
//                 ),
//             )
//         })
//     }
// }

// impl Unpack
//     for (
//         PackedM31,
//         PackedM31,
//         (PackedM31, [PackedM31; 14], [PackedFelt252; 2]),
//     )
// {
//     type CpuType = (M31, M31, (M31, [M31; 14], [Felt252; 2]));
//     fn unpack(self) -> [(M31, M31, (M31, [M31; 14], [Felt252; 2])); N_LANES] {
//         self.to_array()
//     }
// }
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

    pub fn packed_deduce_output(
        chain: PackedM31,
        round: PackedM31,
        (table_offset, m_shifted, accumulator): (PackedM31, [PackedM31; 14], [PackedFelt252; 2]),
    ) -> (
        PackedM31,
        PackedM31,
        (PackedM31, [PackedM31; 14], [PackedFelt252; 2]),
    ) {
        let chain_arr = chain.to_array();
        let round_arr = round.to_array();
        let table_offset_arr = table_offset.to_array();
        let m_shifted_arr1: [[M31; N_LANES]; 14] = from_fn(|i| m_shifted[i].to_array()); // N_LANES
        let m_shifted_arr2: [[M31; 14]; N_LANES] = from_fn(|i| from_fn(|j| m_shifted_arr1[j][i])); // N_LANES
        let accumulator_arr1: [[Felt252; N_LANES]; 2] = from_fn(|i| accumulator[i].to_array()); // N_LANES
        let accumulator_arr2: [[Felt252; 2]; N_LANES] =
            from_fn(|i| from_fn(|j| accumulator_arr1[j][i])); // N_LANES

        let mut chain_output_vec = vec![];
        let mut round_output_vec = vec![];
        let mut table_offset_output = vec![];
        let mut m_shifted_output_vec = vec![];
        let mut accumulator_output_vec = vec![];

        for i in 0..N_LANES {
            let (
                curr_chain_output,
                curr_round_output,
                (curr_table_offset_output, curr_m_shifted_output, curr_accumulator_output),
            ) = PartialEcMul::deduce_output(
                chain_arr[i],
                round_arr[i],
                (table_offset_arr[i], m_shifted_arr2[i], accumulator_arr2[i]),
            );

            chain_output_vec.push(curr_chain_output);
            round_output_vec.push(curr_round_output);
            table_offset_output.push(curr_table_offset_output);
            m_shifted_output_vec.push(curr_m_shifted_output);
            accumulator_output_vec.push(curr_accumulator_output);
        }

        let chain_output = PackedM31::from_array(chain_output_vec.try_into().unwrap());
        let round_output = PackedM31::from_array(round_output_vec.try_into().unwrap());
        let table_offset_output = PackedM31::from_array(table_offset_output.try_into().unwrap());
        let m_shifted_output =
            from_fn(|i| PackedM31::from_array(from_fn(|j| m_shifted_output_vec[j][i])));
        let accumulator_output =
            from_fn(|i| PackedFelt252::from_array(&from_fn(|j| accumulator_output_vec[j][i])));

        (
            chain_output,
            round_output,
            (table_offset_output, m_shifted_output, accumulator_output),
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
