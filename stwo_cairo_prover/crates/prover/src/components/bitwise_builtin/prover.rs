#![allow(unused_parens)]
#![allow(unused_imports)]
use std::iter::zip;

use itertools::{chain, zip_eq, Itertools};
use num_traits::{One, Zero};
use prover_types::cpu::*;
use prover_types::simd::*;
use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};
use stwo_air_utils::trace::component_trace::ComponentTrace;
use stwo_air_utils_derive::{IterMut, ParIterMut, Uninitialized};
use stwo_prover::constraint_framework::logup::LogupTraceGenerator;
use stwo_prover::constraint_framework::Relation;
use stwo_prover::core::air::Component;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::conversion::Unpack;
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::qm31::PackedQM31;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::{BackendForChannel, Col, Column};
use stwo_prover::core::channel::{Channel, MerkleChannel};
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::FieldExpOps;
use stwo_prover::core::pcs::TreeBuilder;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::utils::{
    bit_reverse_coset_to_circle_domain_order, bit_reverse_index, coset_index_to_circle_domain_index,
};

use super::component::{Claim, InteractionClaim};
use crate::cairo_air::preprocessed::{PreProcessedColumn, Seq};
use crate::components::utils::AtomicMultiplicityColumn;
use crate::components::{memory_address_to_id, memory_id_to_big, verify_bitwise_xor_9};
use crate::relations;

const N_TRACE_COLUMNS: usize = 89;

#[derive(Default)]
pub struct ClaimGenerator {
    pub log_size: u32,
    pub bitwise_builtin_segment_start: u32,
}
impl ClaimGenerator {
    pub fn new(log_size: u32, bitwise_builtin_segment_start: u32) -> Self {
        assert!(log_size >= LOG_N_LANES);
        Self {
            log_size,
            bitwise_builtin_segment_start,
        }
    }

    pub fn write_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
        memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
        verify_bitwise_xor_9_state: &verify_bitwise_xor_9::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator)
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let log_size = self.log_size;

        let (trace, lookup_data) = write_trace_simd(
            log_size,
            self.bitwise_builtin_segment_start,
            memory_address_to_id_state,
            memory_id_to_big_state,
            verify_bitwise_xor_9_state,
        );

        tree_builder.extend_evals(trace.to_evals());

        (
            Claim {
                log_size,
                bitwise_builtin_segment_start: self.bitwise_builtin_segment_start,
            },
            InteractionClaimGenerator {
                log_size,
                lookup_data,
            },
        )
    }
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    log_size: u32,
    bitwise_builtin_segment_start: u32,
    memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
    memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
    verify_bitwise_xor_9_state: &verify_bitwise_xor_9::ClaimGenerator,
) -> (ComponentTrace<N_TRACE_COLUMNS>, LookupData) {
    let log_n_packed_rows = log_size - LOG_N_LANES;
    let (mut trace, mut lookup_data) = unsafe {
        (
            ComponentTrace::<N_TRACE_COLUMNS>::uninitialized(log_size),
            LookupData::uninitialized(log_n_packed_rows),
        )
    };

    let M31_0 = PackedM31::broadcast(M31::from(0));
    let M31_1 = PackedM31::broadcast(M31::from(1));
    let M31_1073741824 = PackedM31::broadcast(M31::from(1073741824));
    let M31_2 = PackedM31::broadcast(M31::from(2));
    let M31_3 = PackedM31::broadcast(M31::from(3));
    let M31_4 = PackedM31::broadcast(M31::from(4));
    let M31_5 = PackedM31::broadcast(M31::from(5));

    trace
        .par_iter_mut()
        .enumerate()
        .zip(lookup_data.par_iter_mut())
        .for_each(|((row_index, row), lookup_data)| {
            let seq = Seq::new(log_size).packed_at(row_index);

            // Read Positive Num Bits 252.

            let memory_address_to_id_value_tmp_efb2a_0 = memory_address_to_id_state.deduce_output(
                (((PackedM31::broadcast(M31::from(bitwise_builtin_segment_start)))
                    + ((seq) * (M31_5)))
                    + (M31_0)),
            );
            let memory_id_to_big_value_tmp_efb2a_1 =
                memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_efb2a_0);
            let op0_id_col0 = memory_address_to_id_value_tmp_efb2a_0;
            *row[0] = op0_id_col0;
            let memory_address_to_id_inputs_0 =
                (((PackedM31::broadcast(M31::from(bitwise_builtin_segment_start)))
                    + ((seq) * (M31_5)))
                    + (M31_0))
                    .unpack();
            *lookup_data.memory_address_to_id_0 = [
                (((PackedM31::broadcast(M31::from(bitwise_builtin_segment_start)))
                    + ((seq) * (M31_5)))
                    + (M31_0)),
                op0_id_col0,
            ];
            let op0_limb_0_col1 = memory_id_to_big_value_tmp_efb2a_1.get_m31(0);
            *row[1] = op0_limb_0_col1;
            let op0_limb_1_col2 = memory_id_to_big_value_tmp_efb2a_1.get_m31(1);
            *row[2] = op0_limb_1_col2;
            let op0_limb_2_col3 = memory_id_to_big_value_tmp_efb2a_1.get_m31(2);
            *row[3] = op0_limb_2_col3;
            let op0_limb_3_col4 = memory_id_to_big_value_tmp_efb2a_1.get_m31(3);
            *row[4] = op0_limb_3_col4;
            let op0_limb_4_col5 = memory_id_to_big_value_tmp_efb2a_1.get_m31(4);
            *row[5] = op0_limb_4_col5;
            let op0_limb_5_col6 = memory_id_to_big_value_tmp_efb2a_1.get_m31(5);
            *row[6] = op0_limb_5_col6;
            let op0_limb_6_col7 = memory_id_to_big_value_tmp_efb2a_1.get_m31(6);
            *row[7] = op0_limb_6_col7;
            let op0_limb_7_col8 = memory_id_to_big_value_tmp_efb2a_1.get_m31(7);
            *row[8] = op0_limb_7_col8;
            let op0_limb_8_col9 = memory_id_to_big_value_tmp_efb2a_1.get_m31(8);
            *row[9] = op0_limb_8_col9;
            let op0_limb_9_col10 = memory_id_to_big_value_tmp_efb2a_1.get_m31(9);
            *row[10] = op0_limb_9_col10;
            let op0_limb_10_col11 = memory_id_to_big_value_tmp_efb2a_1.get_m31(10);
            *row[11] = op0_limb_10_col11;
            let op0_limb_11_col12 = memory_id_to_big_value_tmp_efb2a_1.get_m31(11);
            *row[12] = op0_limb_11_col12;
            let op0_limb_12_col13 = memory_id_to_big_value_tmp_efb2a_1.get_m31(12);
            *row[13] = op0_limb_12_col13;
            let op0_limb_13_col14 = memory_id_to_big_value_tmp_efb2a_1.get_m31(13);
            *row[14] = op0_limb_13_col14;
            let op0_limb_14_col15 = memory_id_to_big_value_tmp_efb2a_1.get_m31(14);
            *row[15] = op0_limb_14_col15;
            let op0_limb_15_col16 = memory_id_to_big_value_tmp_efb2a_1.get_m31(15);
            *row[16] = op0_limb_15_col16;
            let op0_limb_16_col17 = memory_id_to_big_value_tmp_efb2a_1.get_m31(16);
            *row[17] = op0_limb_16_col17;
            let op0_limb_17_col18 = memory_id_to_big_value_tmp_efb2a_1.get_m31(17);
            *row[18] = op0_limb_17_col18;
            let op0_limb_18_col19 = memory_id_to_big_value_tmp_efb2a_1.get_m31(18);
            *row[19] = op0_limb_18_col19;
            let op0_limb_19_col20 = memory_id_to_big_value_tmp_efb2a_1.get_m31(19);
            *row[20] = op0_limb_19_col20;
            let op0_limb_20_col21 = memory_id_to_big_value_tmp_efb2a_1.get_m31(20);
            *row[21] = op0_limb_20_col21;
            let op0_limb_21_col22 = memory_id_to_big_value_tmp_efb2a_1.get_m31(21);
            *row[22] = op0_limb_21_col22;
            let op0_limb_22_col23 = memory_id_to_big_value_tmp_efb2a_1.get_m31(22);
            *row[23] = op0_limb_22_col23;
            let op0_limb_23_col24 = memory_id_to_big_value_tmp_efb2a_1.get_m31(23);
            *row[24] = op0_limb_23_col24;
            let op0_limb_24_col25 = memory_id_to_big_value_tmp_efb2a_1.get_m31(24);
            *row[25] = op0_limb_24_col25;
            let op0_limb_25_col26 = memory_id_to_big_value_tmp_efb2a_1.get_m31(25);
            *row[26] = op0_limb_25_col26;
            let op0_limb_26_col27 = memory_id_to_big_value_tmp_efb2a_1.get_m31(26);
            *row[27] = op0_limb_26_col27;
            let op0_limb_27_col28 = memory_id_to_big_value_tmp_efb2a_1.get_m31(27);
            *row[28] = op0_limb_27_col28;
            let memory_id_to_big_inputs_0 = op0_id_col0.unpack();
            *lookup_data.memory_id_to_big_0 = [
                op0_id_col0,
                op0_limb_0_col1,
                op0_limb_1_col2,
                op0_limb_2_col3,
                op0_limb_3_col4,
                op0_limb_4_col5,
                op0_limb_5_col6,
                op0_limb_6_col7,
                op0_limb_7_col8,
                op0_limb_8_col9,
                op0_limb_9_col10,
                op0_limb_10_col11,
                op0_limb_11_col12,
                op0_limb_12_col13,
                op0_limb_13_col14,
                op0_limb_14_col15,
                op0_limb_15_col16,
                op0_limb_16_col17,
                op0_limb_17_col18,
                op0_limb_18_col19,
                op0_limb_19_col20,
                op0_limb_20_col21,
                op0_limb_21_col22,
                op0_limb_22_col23,
                op0_limb_23_col24,
                op0_limb_24_col25,
                op0_limb_25_col26,
                op0_limb_26_col27,
                op0_limb_27_col28,
            ];

            // Read Positive Num Bits 252.

            let memory_address_to_id_value_tmp_efb2a_2 = memory_address_to_id_state.deduce_output(
                (((PackedM31::broadcast(M31::from(bitwise_builtin_segment_start)))
                    + ((seq) * (M31_5)))
                    + (M31_1)),
            );
            let memory_id_to_big_value_tmp_efb2a_3 =
                memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_efb2a_2);
            let op1_id_col29 = memory_address_to_id_value_tmp_efb2a_2;
            *row[29] = op1_id_col29;
            let memory_address_to_id_inputs_1 =
                (((PackedM31::broadcast(M31::from(bitwise_builtin_segment_start)))
                    + ((seq) * (M31_5)))
                    + (M31_1))
                    .unpack();
            *lookup_data.memory_address_to_id_1 = [
                (((PackedM31::broadcast(M31::from(bitwise_builtin_segment_start)))
                    + ((seq) * (M31_5)))
                    + (M31_1)),
                op1_id_col29,
            ];
            let op1_limb_0_col30 = memory_id_to_big_value_tmp_efb2a_3.get_m31(0);
            *row[30] = op1_limb_0_col30;
            let op1_limb_1_col31 = memory_id_to_big_value_tmp_efb2a_3.get_m31(1);
            *row[31] = op1_limb_1_col31;
            let op1_limb_2_col32 = memory_id_to_big_value_tmp_efb2a_3.get_m31(2);
            *row[32] = op1_limb_2_col32;
            let op1_limb_3_col33 = memory_id_to_big_value_tmp_efb2a_3.get_m31(3);
            *row[33] = op1_limb_3_col33;
            let op1_limb_4_col34 = memory_id_to_big_value_tmp_efb2a_3.get_m31(4);
            *row[34] = op1_limb_4_col34;
            let op1_limb_5_col35 = memory_id_to_big_value_tmp_efb2a_3.get_m31(5);
            *row[35] = op1_limb_5_col35;
            let op1_limb_6_col36 = memory_id_to_big_value_tmp_efb2a_3.get_m31(6);
            *row[36] = op1_limb_6_col36;
            let op1_limb_7_col37 = memory_id_to_big_value_tmp_efb2a_3.get_m31(7);
            *row[37] = op1_limb_7_col37;
            let op1_limb_8_col38 = memory_id_to_big_value_tmp_efb2a_3.get_m31(8);
            *row[38] = op1_limb_8_col38;
            let op1_limb_9_col39 = memory_id_to_big_value_tmp_efb2a_3.get_m31(9);
            *row[39] = op1_limb_9_col39;
            let op1_limb_10_col40 = memory_id_to_big_value_tmp_efb2a_3.get_m31(10);
            *row[40] = op1_limb_10_col40;
            let op1_limb_11_col41 = memory_id_to_big_value_tmp_efb2a_3.get_m31(11);
            *row[41] = op1_limb_11_col41;
            let op1_limb_12_col42 = memory_id_to_big_value_tmp_efb2a_3.get_m31(12);
            *row[42] = op1_limb_12_col42;
            let op1_limb_13_col43 = memory_id_to_big_value_tmp_efb2a_3.get_m31(13);
            *row[43] = op1_limb_13_col43;
            let op1_limb_14_col44 = memory_id_to_big_value_tmp_efb2a_3.get_m31(14);
            *row[44] = op1_limb_14_col44;
            let op1_limb_15_col45 = memory_id_to_big_value_tmp_efb2a_3.get_m31(15);
            *row[45] = op1_limb_15_col45;
            let op1_limb_16_col46 = memory_id_to_big_value_tmp_efb2a_3.get_m31(16);
            *row[46] = op1_limb_16_col46;
            let op1_limb_17_col47 = memory_id_to_big_value_tmp_efb2a_3.get_m31(17);
            *row[47] = op1_limb_17_col47;
            let op1_limb_18_col48 = memory_id_to_big_value_tmp_efb2a_3.get_m31(18);
            *row[48] = op1_limb_18_col48;
            let op1_limb_19_col49 = memory_id_to_big_value_tmp_efb2a_3.get_m31(19);
            *row[49] = op1_limb_19_col49;
            let op1_limb_20_col50 = memory_id_to_big_value_tmp_efb2a_3.get_m31(20);
            *row[50] = op1_limb_20_col50;
            let op1_limb_21_col51 = memory_id_to_big_value_tmp_efb2a_3.get_m31(21);
            *row[51] = op1_limb_21_col51;
            let op1_limb_22_col52 = memory_id_to_big_value_tmp_efb2a_3.get_m31(22);
            *row[52] = op1_limb_22_col52;
            let op1_limb_23_col53 = memory_id_to_big_value_tmp_efb2a_3.get_m31(23);
            *row[53] = op1_limb_23_col53;
            let op1_limb_24_col54 = memory_id_to_big_value_tmp_efb2a_3.get_m31(24);
            *row[54] = op1_limb_24_col54;
            let op1_limb_25_col55 = memory_id_to_big_value_tmp_efb2a_3.get_m31(25);
            *row[55] = op1_limb_25_col55;
            let op1_limb_26_col56 = memory_id_to_big_value_tmp_efb2a_3.get_m31(26);
            *row[56] = op1_limb_26_col56;
            let op1_limb_27_col57 = memory_id_to_big_value_tmp_efb2a_3.get_m31(27);
            *row[57] = op1_limb_27_col57;
            let memory_id_to_big_inputs_1 = op1_id_col29.unpack();
            *lookup_data.memory_id_to_big_1 = [
                op1_id_col29,
                op1_limb_0_col30,
                op1_limb_1_col31,
                op1_limb_2_col32,
                op1_limb_3_col33,
                op1_limb_4_col34,
                op1_limb_5_col35,
                op1_limb_6_col36,
                op1_limb_7_col37,
                op1_limb_8_col38,
                op1_limb_9_col39,
                op1_limb_10_col40,
                op1_limb_11_col41,
                op1_limb_12_col42,
                op1_limb_13_col43,
                op1_limb_14_col44,
                op1_limb_15_col45,
                op1_limb_16_col46,
                op1_limb_17_col47,
                op1_limb_18_col48,
                op1_limb_19_col49,
                op1_limb_20_col50,
                op1_limb_21_col51,
                op1_limb_22_col52,
                op1_limb_23_col53,
                op1_limb_24_col54,
                op1_limb_25_col55,
                op1_limb_26_col56,
                op1_limb_27_col57,
            ];

            // Bitwise Xor Num Bits 9.

            let xor_tmp_efb2a_4 = ((PackedUInt16::from_m31(op0_limb_0_col1))
                ^ (PackedUInt16::from_m31(op1_limb_0_col30)));
            let xor_col58 = xor_tmp_efb2a_4.as_m31();
            *row[58] = xor_col58;
            let verify_bitwise_xor_9_inputs_0 =
                [op0_limb_0_col1, op1_limb_0_col30, xor_col58].unpack();
            *lookup_data.verify_bitwise_xor_9_0 = [op0_limb_0_col1, op1_limb_0_col30, xor_col58];

            let and_tmp_efb2a_5 =
                ((M31_1073741824) * (((op0_limb_0_col1) + (op1_limb_0_col30)) - (xor_col58)));

            // Bitwise Xor Num Bits 9.

            let xor_tmp_efb2a_6 = ((PackedUInt16::from_m31(op0_limb_1_col2))
                ^ (PackedUInt16::from_m31(op1_limb_1_col31)));
            let xor_col59 = xor_tmp_efb2a_6.as_m31();
            *row[59] = xor_col59;
            let verify_bitwise_xor_9_inputs_1 =
                [op0_limb_1_col2, op1_limb_1_col31, xor_col59].unpack();
            *lookup_data.verify_bitwise_xor_9_1 = [op0_limb_1_col2, op1_limb_1_col31, xor_col59];

            let and_tmp_efb2a_7 =
                ((M31_1073741824) * (((op0_limb_1_col2) + (op1_limb_1_col31)) - (xor_col59)));

            // Bitwise Xor Num Bits 9.

            let xor_tmp_efb2a_8 = ((PackedUInt16::from_m31(op0_limb_2_col3))
                ^ (PackedUInt16::from_m31(op1_limb_2_col32)));
            let xor_col60 = xor_tmp_efb2a_8.as_m31();
            *row[60] = xor_col60;
            let verify_bitwise_xor_9_inputs_2 =
                [op0_limb_2_col3, op1_limb_2_col32, xor_col60].unpack();
            *lookup_data.verify_bitwise_xor_9_2 = [op0_limb_2_col3, op1_limb_2_col32, xor_col60];

            let and_tmp_efb2a_9 =
                ((M31_1073741824) * (((op0_limb_2_col3) + (op1_limb_2_col32)) - (xor_col60)));

            // Bitwise Xor Num Bits 9.

            let xor_tmp_efb2a_10 = ((PackedUInt16::from_m31(op0_limb_3_col4))
                ^ (PackedUInt16::from_m31(op1_limb_3_col33)));
            let xor_col61 = xor_tmp_efb2a_10.as_m31();
            *row[61] = xor_col61;
            let verify_bitwise_xor_9_inputs_3 =
                [op0_limb_3_col4, op1_limb_3_col33, xor_col61].unpack();
            *lookup_data.verify_bitwise_xor_9_3 = [op0_limb_3_col4, op1_limb_3_col33, xor_col61];

            let and_tmp_efb2a_11 =
                ((M31_1073741824) * (((op0_limb_3_col4) + (op1_limb_3_col33)) - (xor_col61)));

            // Bitwise Xor Num Bits 9.

            let xor_tmp_efb2a_12 = ((PackedUInt16::from_m31(op0_limb_4_col5))
                ^ (PackedUInt16::from_m31(op1_limb_4_col34)));
            let xor_col62 = xor_tmp_efb2a_12.as_m31();
            *row[62] = xor_col62;
            let verify_bitwise_xor_9_inputs_4 =
                [op0_limb_4_col5, op1_limb_4_col34, xor_col62].unpack();
            *lookup_data.verify_bitwise_xor_9_4 = [op0_limb_4_col5, op1_limb_4_col34, xor_col62];

            let and_tmp_efb2a_13 =
                ((M31_1073741824) * (((op0_limb_4_col5) + (op1_limb_4_col34)) - (xor_col62)));

            // Bitwise Xor Num Bits 9.

            let xor_tmp_efb2a_14 = ((PackedUInt16::from_m31(op0_limb_5_col6))
                ^ (PackedUInt16::from_m31(op1_limb_5_col35)));
            let xor_col63 = xor_tmp_efb2a_14.as_m31();
            *row[63] = xor_col63;
            let verify_bitwise_xor_9_inputs_5 =
                [op0_limb_5_col6, op1_limb_5_col35, xor_col63].unpack();
            *lookup_data.verify_bitwise_xor_9_5 = [op0_limb_5_col6, op1_limb_5_col35, xor_col63];

            let and_tmp_efb2a_15 =
                ((M31_1073741824) * (((op0_limb_5_col6) + (op1_limb_5_col35)) - (xor_col63)));

            // Bitwise Xor Num Bits 9.

            let xor_tmp_efb2a_16 = ((PackedUInt16::from_m31(op0_limb_6_col7))
                ^ (PackedUInt16::from_m31(op1_limb_6_col36)));
            let xor_col64 = xor_tmp_efb2a_16.as_m31();
            *row[64] = xor_col64;
            let verify_bitwise_xor_9_inputs_6 =
                [op0_limb_6_col7, op1_limb_6_col36, xor_col64].unpack();
            *lookup_data.verify_bitwise_xor_9_6 = [op0_limb_6_col7, op1_limb_6_col36, xor_col64];

            let and_tmp_efb2a_17 =
                ((M31_1073741824) * (((op0_limb_6_col7) + (op1_limb_6_col36)) - (xor_col64)));

            // Bitwise Xor Num Bits 9.

            let xor_tmp_efb2a_18 = ((PackedUInt16::from_m31(op0_limb_7_col8))
                ^ (PackedUInt16::from_m31(op1_limb_7_col37)));
            let xor_col65 = xor_tmp_efb2a_18.as_m31();
            *row[65] = xor_col65;
            let verify_bitwise_xor_9_inputs_7 =
                [op0_limb_7_col8, op1_limb_7_col37, xor_col65].unpack();
            *lookup_data.verify_bitwise_xor_9_7 = [op0_limb_7_col8, op1_limb_7_col37, xor_col65];

            let and_tmp_efb2a_19 =
                ((M31_1073741824) * (((op0_limb_7_col8) + (op1_limb_7_col37)) - (xor_col65)));

            // Bitwise Xor Num Bits 9.

            let xor_tmp_efb2a_20 = ((PackedUInt16::from_m31(op0_limb_8_col9))
                ^ (PackedUInt16::from_m31(op1_limb_8_col38)));
            let xor_col66 = xor_tmp_efb2a_20.as_m31();
            *row[66] = xor_col66;
            let verify_bitwise_xor_9_inputs_8 =
                [op0_limb_8_col9, op1_limb_8_col38, xor_col66].unpack();
            *lookup_data.verify_bitwise_xor_9_8 = [op0_limb_8_col9, op1_limb_8_col38, xor_col66];

            let and_tmp_efb2a_21 =
                ((M31_1073741824) * (((op0_limb_8_col9) + (op1_limb_8_col38)) - (xor_col66)));

            // Bitwise Xor Num Bits 9.

            let xor_tmp_efb2a_22 = ((PackedUInt16::from_m31(op0_limb_9_col10))
                ^ (PackedUInt16::from_m31(op1_limb_9_col39)));
            let xor_col67 = xor_tmp_efb2a_22.as_m31();
            *row[67] = xor_col67;
            let verify_bitwise_xor_9_inputs_9 =
                [op0_limb_9_col10, op1_limb_9_col39, xor_col67].unpack();
            *lookup_data.verify_bitwise_xor_9_9 = [op0_limb_9_col10, op1_limb_9_col39, xor_col67];

            let and_tmp_efb2a_23 =
                ((M31_1073741824) * (((op0_limb_9_col10) + (op1_limb_9_col39)) - (xor_col67)));

            // Bitwise Xor Num Bits 9.

            let xor_tmp_efb2a_24 = ((PackedUInt16::from_m31(op0_limb_10_col11))
                ^ (PackedUInt16::from_m31(op1_limb_10_col40)));
            let xor_col68 = xor_tmp_efb2a_24.as_m31();
            *row[68] = xor_col68;
            let verify_bitwise_xor_9_inputs_10 =
                [op0_limb_10_col11, op1_limb_10_col40, xor_col68].unpack();
            *lookup_data.verify_bitwise_xor_9_10 =
                [op0_limb_10_col11, op1_limb_10_col40, xor_col68];

            let and_tmp_efb2a_25 =
                ((M31_1073741824) * (((op0_limb_10_col11) + (op1_limb_10_col40)) - (xor_col68)));

            // Bitwise Xor Num Bits 9.

            let xor_tmp_efb2a_26 = ((PackedUInt16::from_m31(op0_limb_11_col12))
                ^ (PackedUInt16::from_m31(op1_limb_11_col41)));
            let xor_col69 = xor_tmp_efb2a_26.as_m31();
            *row[69] = xor_col69;
            let verify_bitwise_xor_9_inputs_11 =
                [op0_limb_11_col12, op1_limb_11_col41, xor_col69].unpack();
            *lookup_data.verify_bitwise_xor_9_11 =
                [op0_limb_11_col12, op1_limb_11_col41, xor_col69];

            let and_tmp_efb2a_27 =
                ((M31_1073741824) * (((op0_limb_11_col12) + (op1_limb_11_col41)) - (xor_col69)));

            // Bitwise Xor Num Bits 9.

            let xor_tmp_efb2a_28 = ((PackedUInt16::from_m31(op0_limb_12_col13))
                ^ (PackedUInt16::from_m31(op1_limb_12_col42)));
            let xor_col70 = xor_tmp_efb2a_28.as_m31();
            *row[70] = xor_col70;
            let verify_bitwise_xor_9_inputs_12 =
                [op0_limb_12_col13, op1_limb_12_col42, xor_col70].unpack();
            *lookup_data.verify_bitwise_xor_9_12 =
                [op0_limb_12_col13, op1_limb_12_col42, xor_col70];

            let and_tmp_efb2a_29 =
                ((M31_1073741824) * (((op0_limb_12_col13) + (op1_limb_12_col42)) - (xor_col70)));

            // Bitwise Xor Num Bits 9.

            let xor_tmp_efb2a_30 = ((PackedUInt16::from_m31(op0_limb_13_col14))
                ^ (PackedUInt16::from_m31(op1_limb_13_col43)));
            let xor_col71 = xor_tmp_efb2a_30.as_m31();
            *row[71] = xor_col71;
            let verify_bitwise_xor_9_inputs_13 =
                [op0_limb_13_col14, op1_limb_13_col43, xor_col71].unpack();
            *lookup_data.verify_bitwise_xor_9_13 =
                [op0_limb_13_col14, op1_limb_13_col43, xor_col71];

            let and_tmp_efb2a_31 =
                ((M31_1073741824) * (((op0_limb_13_col14) + (op1_limb_13_col43)) - (xor_col71)));

            // Bitwise Xor Num Bits 9.

            let xor_tmp_efb2a_32 = ((PackedUInt16::from_m31(op0_limb_14_col15))
                ^ (PackedUInt16::from_m31(op1_limb_14_col44)));
            let xor_col72 = xor_tmp_efb2a_32.as_m31();
            *row[72] = xor_col72;
            let verify_bitwise_xor_9_inputs_14 =
                [op0_limb_14_col15, op1_limb_14_col44, xor_col72].unpack();
            *lookup_data.verify_bitwise_xor_9_14 =
                [op0_limb_14_col15, op1_limb_14_col44, xor_col72];

            let and_tmp_efb2a_33 =
                ((M31_1073741824) * (((op0_limb_14_col15) + (op1_limb_14_col44)) - (xor_col72)));

            // Bitwise Xor Num Bits 9.

            let xor_tmp_efb2a_34 = ((PackedUInt16::from_m31(op0_limb_15_col16))
                ^ (PackedUInt16::from_m31(op1_limb_15_col45)));
            let xor_col73 = xor_tmp_efb2a_34.as_m31();
            *row[73] = xor_col73;
            let verify_bitwise_xor_9_inputs_15 =
                [op0_limb_15_col16, op1_limb_15_col45, xor_col73].unpack();
            *lookup_data.verify_bitwise_xor_9_15 =
                [op0_limb_15_col16, op1_limb_15_col45, xor_col73];

            let and_tmp_efb2a_35 =
                ((M31_1073741824) * (((op0_limb_15_col16) + (op1_limb_15_col45)) - (xor_col73)));

            // Bitwise Xor Num Bits 9.

            let xor_tmp_efb2a_36 = ((PackedUInt16::from_m31(op0_limb_16_col17))
                ^ (PackedUInt16::from_m31(op1_limb_16_col46)));
            let xor_col74 = xor_tmp_efb2a_36.as_m31();
            *row[74] = xor_col74;
            let verify_bitwise_xor_9_inputs_16 =
                [op0_limb_16_col17, op1_limb_16_col46, xor_col74].unpack();
            *lookup_data.verify_bitwise_xor_9_16 =
                [op0_limb_16_col17, op1_limb_16_col46, xor_col74];

            let and_tmp_efb2a_37 =
                ((M31_1073741824) * (((op0_limb_16_col17) + (op1_limb_16_col46)) - (xor_col74)));

            // Bitwise Xor Num Bits 9.

            let xor_tmp_efb2a_38 = ((PackedUInt16::from_m31(op0_limb_17_col18))
                ^ (PackedUInt16::from_m31(op1_limb_17_col47)));
            let xor_col75 = xor_tmp_efb2a_38.as_m31();
            *row[75] = xor_col75;
            let verify_bitwise_xor_9_inputs_17 =
                [op0_limb_17_col18, op1_limb_17_col47, xor_col75].unpack();
            *lookup_data.verify_bitwise_xor_9_17 =
                [op0_limb_17_col18, op1_limb_17_col47, xor_col75];

            let and_tmp_efb2a_39 =
                ((M31_1073741824) * (((op0_limb_17_col18) + (op1_limb_17_col47)) - (xor_col75)));

            // Bitwise Xor Num Bits 9.

            let xor_tmp_efb2a_40 = ((PackedUInt16::from_m31(op0_limb_18_col19))
                ^ (PackedUInt16::from_m31(op1_limb_18_col48)));
            let xor_col76 = xor_tmp_efb2a_40.as_m31();
            *row[76] = xor_col76;
            let verify_bitwise_xor_9_inputs_18 =
                [op0_limb_18_col19, op1_limb_18_col48, xor_col76].unpack();
            *lookup_data.verify_bitwise_xor_9_18 =
                [op0_limb_18_col19, op1_limb_18_col48, xor_col76];

            let and_tmp_efb2a_41 =
                ((M31_1073741824) * (((op0_limb_18_col19) + (op1_limb_18_col48)) - (xor_col76)));

            // Bitwise Xor Num Bits 9.

            let xor_tmp_efb2a_42 = ((PackedUInt16::from_m31(op0_limb_19_col20))
                ^ (PackedUInt16::from_m31(op1_limb_19_col49)));
            let xor_col77 = xor_tmp_efb2a_42.as_m31();
            *row[77] = xor_col77;
            let verify_bitwise_xor_9_inputs_19 =
                [op0_limb_19_col20, op1_limb_19_col49, xor_col77].unpack();
            *lookup_data.verify_bitwise_xor_9_19 =
                [op0_limb_19_col20, op1_limb_19_col49, xor_col77];

            let and_tmp_efb2a_43 =
                ((M31_1073741824) * (((op0_limb_19_col20) + (op1_limb_19_col49)) - (xor_col77)));

            // Bitwise Xor Num Bits 9.

            let xor_tmp_efb2a_44 = ((PackedUInt16::from_m31(op0_limb_20_col21))
                ^ (PackedUInt16::from_m31(op1_limb_20_col50)));
            let xor_col78 = xor_tmp_efb2a_44.as_m31();
            *row[78] = xor_col78;
            let verify_bitwise_xor_9_inputs_20 =
                [op0_limb_20_col21, op1_limb_20_col50, xor_col78].unpack();
            *lookup_data.verify_bitwise_xor_9_20 =
                [op0_limb_20_col21, op1_limb_20_col50, xor_col78];

            let and_tmp_efb2a_45 =
                ((M31_1073741824) * (((op0_limb_20_col21) + (op1_limb_20_col50)) - (xor_col78)));

            // Bitwise Xor Num Bits 9.

            let xor_tmp_efb2a_46 = ((PackedUInt16::from_m31(op0_limb_21_col22))
                ^ (PackedUInt16::from_m31(op1_limb_21_col51)));
            let xor_col79 = xor_tmp_efb2a_46.as_m31();
            *row[79] = xor_col79;
            let verify_bitwise_xor_9_inputs_21 =
                [op0_limb_21_col22, op1_limb_21_col51, xor_col79].unpack();
            *lookup_data.verify_bitwise_xor_9_21 =
                [op0_limb_21_col22, op1_limb_21_col51, xor_col79];

            let and_tmp_efb2a_47 =
                ((M31_1073741824) * (((op0_limb_21_col22) + (op1_limb_21_col51)) - (xor_col79)));

            // Bitwise Xor Num Bits 9.

            let xor_tmp_efb2a_48 = ((PackedUInt16::from_m31(op0_limb_22_col23))
                ^ (PackedUInt16::from_m31(op1_limb_22_col52)));
            let xor_col80 = xor_tmp_efb2a_48.as_m31();
            *row[80] = xor_col80;
            let verify_bitwise_xor_9_inputs_22 =
                [op0_limb_22_col23, op1_limb_22_col52, xor_col80].unpack();
            *lookup_data.verify_bitwise_xor_9_22 =
                [op0_limb_22_col23, op1_limb_22_col52, xor_col80];

            let and_tmp_efb2a_49 =
                ((M31_1073741824) * (((op0_limb_22_col23) + (op1_limb_22_col52)) - (xor_col80)));

            // Bitwise Xor Num Bits 9.

            let xor_tmp_efb2a_50 = ((PackedUInt16::from_m31(op0_limb_23_col24))
                ^ (PackedUInt16::from_m31(op1_limb_23_col53)));
            let xor_col81 = xor_tmp_efb2a_50.as_m31();
            *row[81] = xor_col81;
            let verify_bitwise_xor_9_inputs_23 =
                [op0_limb_23_col24, op1_limb_23_col53, xor_col81].unpack();
            *lookup_data.verify_bitwise_xor_9_23 =
                [op0_limb_23_col24, op1_limb_23_col53, xor_col81];

            let and_tmp_efb2a_51 =
                ((M31_1073741824) * (((op0_limb_23_col24) + (op1_limb_23_col53)) - (xor_col81)));

            // Bitwise Xor Num Bits 9.

            let xor_tmp_efb2a_52 = ((PackedUInt16::from_m31(op0_limb_24_col25))
                ^ (PackedUInt16::from_m31(op1_limb_24_col54)));
            let xor_col82 = xor_tmp_efb2a_52.as_m31();
            *row[82] = xor_col82;
            let verify_bitwise_xor_9_inputs_24 =
                [op0_limb_24_col25, op1_limb_24_col54, xor_col82].unpack();
            *lookup_data.verify_bitwise_xor_9_24 =
                [op0_limb_24_col25, op1_limb_24_col54, xor_col82];

            let and_tmp_efb2a_53 =
                ((M31_1073741824) * (((op0_limb_24_col25) + (op1_limb_24_col54)) - (xor_col82)));

            // Bitwise Xor Num Bits 9.

            let xor_tmp_efb2a_54 = ((PackedUInt16::from_m31(op0_limb_25_col26))
                ^ (PackedUInt16::from_m31(op1_limb_25_col55)));
            let xor_col83 = xor_tmp_efb2a_54.as_m31();
            *row[83] = xor_col83;
            let verify_bitwise_xor_9_inputs_25 =
                [op0_limb_25_col26, op1_limb_25_col55, xor_col83].unpack();
            *lookup_data.verify_bitwise_xor_9_25 =
                [op0_limb_25_col26, op1_limb_25_col55, xor_col83];

            let and_tmp_efb2a_55 =
                ((M31_1073741824) * (((op0_limb_25_col26) + (op1_limb_25_col55)) - (xor_col83)));

            // Bitwise Xor Num Bits 9.

            let xor_tmp_efb2a_56 = ((PackedUInt16::from_m31(op0_limb_26_col27))
                ^ (PackedUInt16::from_m31(op1_limb_26_col56)));
            let xor_col84 = xor_tmp_efb2a_56.as_m31();
            *row[84] = xor_col84;
            let verify_bitwise_xor_9_inputs_26 =
                [op0_limb_26_col27, op1_limb_26_col56, xor_col84].unpack();
            *lookup_data.verify_bitwise_xor_9_26 =
                [op0_limb_26_col27, op1_limb_26_col56, xor_col84];

            let and_tmp_efb2a_57 =
                ((M31_1073741824) * (((op0_limb_26_col27) + (op1_limb_26_col56)) - (xor_col84)));

            // Bitwise Xor Num Bits 9.

            let xor_tmp_efb2a_58 = ((PackedUInt16::from_m31(op0_limb_27_col28))
                ^ (PackedUInt16::from_m31(op1_limb_27_col57)));
            let xor_col85 = xor_tmp_efb2a_58.as_m31();
            *row[85] = xor_col85;
            let verify_bitwise_xor_9_inputs_27 =
                [op0_limb_27_col28, op1_limb_27_col57, xor_col85].unpack();
            *lookup_data.verify_bitwise_xor_9_27 =
                [op0_limb_27_col28, op1_limb_27_col57, xor_col85];

            let and_tmp_efb2a_59 =
                ((M31_1073741824) * (((op0_limb_27_col28) + (op1_limb_27_col57)) - (xor_col85)));

            // Mem Verify.

            let memory_address_to_id_value_tmp_efb2a_60 = memory_address_to_id_state.deduce_output(
                (((PackedM31::broadcast(M31::from(bitwise_builtin_segment_start)))
                    + ((seq) * (M31_5)))
                    + (M31_2)),
            );
            let and_id_col86 = memory_address_to_id_value_tmp_efb2a_60;
            *row[86] = and_id_col86;
            let memory_address_to_id_inputs_2 =
                (((PackedM31::broadcast(M31::from(bitwise_builtin_segment_start)))
                    + ((seq) * (M31_5)))
                    + (M31_2))
                    .unpack();
            *lookup_data.memory_address_to_id_2 = [
                (((PackedM31::broadcast(M31::from(bitwise_builtin_segment_start)))
                    + ((seq) * (M31_5)))
                    + (M31_2)),
                and_id_col86,
            ];
            let memory_id_to_big_inputs_2 = and_id_col86.unpack();
            *lookup_data.memory_id_to_big_2 = [
                and_id_col86,
                and_tmp_efb2a_5,
                and_tmp_efb2a_7,
                and_tmp_efb2a_9,
                and_tmp_efb2a_11,
                and_tmp_efb2a_13,
                and_tmp_efb2a_15,
                and_tmp_efb2a_17,
                and_tmp_efb2a_19,
                and_tmp_efb2a_21,
                and_tmp_efb2a_23,
                and_tmp_efb2a_25,
                and_tmp_efb2a_27,
                and_tmp_efb2a_29,
                and_tmp_efb2a_31,
                and_tmp_efb2a_33,
                and_tmp_efb2a_35,
                and_tmp_efb2a_37,
                and_tmp_efb2a_39,
                and_tmp_efb2a_41,
                and_tmp_efb2a_43,
                and_tmp_efb2a_45,
                and_tmp_efb2a_47,
                and_tmp_efb2a_49,
                and_tmp_efb2a_51,
                and_tmp_efb2a_53,
                and_tmp_efb2a_55,
                and_tmp_efb2a_57,
                and_tmp_efb2a_59,
            ];

            // Mem Verify.

            let memory_address_to_id_value_tmp_efb2a_61 = memory_address_to_id_state.deduce_output(
                (((PackedM31::broadcast(M31::from(bitwise_builtin_segment_start)))
                    + ((seq) * (M31_5)))
                    + (M31_3)),
            );
            let xor_id_col87 = memory_address_to_id_value_tmp_efb2a_61;
            *row[87] = xor_id_col87;
            let memory_address_to_id_inputs_3 =
                (((PackedM31::broadcast(M31::from(bitwise_builtin_segment_start)))
                    + ((seq) * (M31_5)))
                    + (M31_3))
                    .unpack();
            *lookup_data.memory_address_to_id_3 = [
                (((PackedM31::broadcast(M31::from(bitwise_builtin_segment_start)))
                    + ((seq) * (M31_5)))
                    + (M31_3)),
                xor_id_col87,
            ];
            let memory_id_to_big_inputs_3 = xor_id_col87.unpack();
            *lookup_data.memory_id_to_big_3 = [
                xor_id_col87,
                xor_col58,
                xor_col59,
                xor_col60,
                xor_col61,
                xor_col62,
                xor_col63,
                xor_col64,
                xor_col65,
                xor_col66,
                xor_col67,
                xor_col68,
                xor_col69,
                xor_col70,
                xor_col71,
                xor_col72,
                xor_col73,
                xor_col74,
                xor_col75,
                xor_col76,
                xor_col77,
                xor_col78,
                xor_col79,
                xor_col80,
                xor_col81,
                xor_col82,
                xor_col83,
                xor_col84,
                xor_col85,
            ];

            // Mem Verify.

            let memory_address_to_id_value_tmp_efb2a_62 = memory_address_to_id_state.deduce_output(
                (((PackedM31::broadcast(M31::from(bitwise_builtin_segment_start)))
                    + ((seq) * (M31_5)))
                    + (M31_4)),
            );
            let or_id_col88 = memory_address_to_id_value_tmp_efb2a_62;
            *row[88] = or_id_col88;
            let memory_address_to_id_inputs_4 =
                (((PackedM31::broadcast(M31::from(bitwise_builtin_segment_start)))
                    + ((seq) * (M31_5)))
                    + (M31_4))
                    .unpack();
            *lookup_data.memory_address_to_id_4 = [
                (((PackedM31::broadcast(M31::from(bitwise_builtin_segment_start)))
                    + ((seq) * (M31_5)))
                    + (M31_4)),
                or_id_col88,
            ];
            let memory_id_to_big_inputs_4 = or_id_col88.unpack();
            *lookup_data.memory_id_to_big_4 = [
                or_id_col88,
                ((and_tmp_efb2a_5) + (xor_col58)),
                ((and_tmp_efb2a_7) + (xor_col59)),
                ((and_tmp_efb2a_9) + (xor_col60)),
                ((and_tmp_efb2a_11) + (xor_col61)),
                ((and_tmp_efb2a_13) + (xor_col62)),
                ((and_tmp_efb2a_15) + (xor_col63)),
                ((and_tmp_efb2a_17) + (xor_col64)),
                ((and_tmp_efb2a_19) + (xor_col65)),
                ((and_tmp_efb2a_21) + (xor_col66)),
                ((and_tmp_efb2a_23) + (xor_col67)),
                ((and_tmp_efb2a_25) + (xor_col68)),
                ((and_tmp_efb2a_27) + (xor_col69)),
                ((and_tmp_efb2a_29) + (xor_col70)),
                ((and_tmp_efb2a_31) + (xor_col71)),
                ((and_tmp_efb2a_33) + (xor_col72)),
                ((and_tmp_efb2a_35) + (xor_col73)),
                ((and_tmp_efb2a_37) + (xor_col74)),
                ((and_tmp_efb2a_39) + (xor_col75)),
                ((and_tmp_efb2a_41) + (xor_col76)),
                ((and_tmp_efb2a_43) + (xor_col77)),
                ((and_tmp_efb2a_45) + (xor_col78)),
                ((and_tmp_efb2a_47) + (xor_col79)),
                ((and_tmp_efb2a_49) + (xor_col80)),
                ((and_tmp_efb2a_51) + (xor_col81)),
                ((and_tmp_efb2a_53) + (xor_col82)),
                ((and_tmp_efb2a_55) + (xor_col83)),
                ((and_tmp_efb2a_57) + (xor_col84)),
                ((and_tmp_efb2a_59) + (xor_col85)),
            ];

            // Add sub-components inputs.
            #[allow(clippy::needless_range_loop)]
            for i in 0..N_LANES {
                memory_address_to_id_state.add_input(&memory_address_to_id_inputs_0[i]);
                memory_id_to_big_state.add_input(&memory_id_to_big_inputs_0[i]);
                memory_address_to_id_state.add_input(&memory_address_to_id_inputs_1[i]);
                memory_id_to_big_state.add_input(&memory_id_to_big_inputs_1[i]);
                verify_bitwise_xor_9_state.add_input(&verify_bitwise_xor_9_inputs_0[i]);
                verify_bitwise_xor_9_state.add_input(&verify_bitwise_xor_9_inputs_1[i]);
                verify_bitwise_xor_9_state.add_input(&verify_bitwise_xor_9_inputs_2[i]);
                verify_bitwise_xor_9_state.add_input(&verify_bitwise_xor_9_inputs_3[i]);
                verify_bitwise_xor_9_state.add_input(&verify_bitwise_xor_9_inputs_4[i]);
                verify_bitwise_xor_9_state.add_input(&verify_bitwise_xor_9_inputs_5[i]);
                verify_bitwise_xor_9_state.add_input(&verify_bitwise_xor_9_inputs_6[i]);
                verify_bitwise_xor_9_state.add_input(&verify_bitwise_xor_9_inputs_7[i]);
                verify_bitwise_xor_9_state.add_input(&verify_bitwise_xor_9_inputs_8[i]);
                verify_bitwise_xor_9_state.add_input(&verify_bitwise_xor_9_inputs_9[i]);
                verify_bitwise_xor_9_state.add_input(&verify_bitwise_xor_9_inputs_10[i]);
                verify_bitwise_xor_9_state.add_input(&verify_bitwise_xor_9_inputs_11[i]);
                verify_bitwise_xor_9_state.add_input(&verify_bitwise_xor_9_inputs_12[i]);
                verify_bitwise_xor_9_state.add_input(&verify_bitwise_xor_9_inputs_13[i]);
                verify_bitwise_xor_9_state.add_input(&verify_bitwise_xor_9_inputs_14[i]);
                verify_bitwise_xor_9_state.add_input(&verify_bitwise_xor_9_inputs_15[i]);
                verify_bitwise_xor_9_state.add_input(&verify_bitwise_xor_9_inputs_16[i]);
                verify_bitwise_xor_9_state.add_input(&verify_bitwise_xor_9_inputs_17[i]);
                verify_bitwise_xor_9_state.add_input(&verify_bitwise_xor_9_inputs_18[i]);
                verify_bitwise_xor_9_state.add_input(&verify_bitwise_xor_9_inputs_19[i]);
                verify_bitwise_xor_9_state.add_input(&verify_bitwise_xor_9_inputs_20[i]);
                verify_bitwise_xor_9_state.add_input(&verify_bitwise_xor_9_inputs_21[i]);
                verify_bitwise_xor_9_state.add_input(&verify_bitwise_xor_9_inputs_22[i]);
                verify_bitwise_xor_9_state.add_input(&verify_bitwise_xor_9_inputs_23[i]);
                verify_bitwise_xor_9_state.add_input(&verify_bitwise_xor_9_inputs_24[i]);
                verify_bitwise_xor_9_state.add_input(&verify_bitwise_xor_9_inputs_25[i]);
                verify_bitwise_xor_9_state.add_input(&verify_bitwise_xor_9_inputs_26[i]);
                verify_bitwise_xor_9_state.add_input(&verify_bitwise_xor_9_inputs_27[i]);
                memory_address_to_id_state.add_input(&memory_address_to_id_inputs_2[i]);
                memory_id_to_big_state.add_input(&memory_id_to_big_inputs_2[i]);
                memory_address_to_id_state.add_input(&memory_address_to_id_inputs_3[i]);
                memory_id_to_big_state.add_input(&memory_id_to_big_inputs_3[i]);
                memory_address_to_id_state.add_input(&memory_address_to_id_inputs_4[i]);
                memory_id_to_big_state.add_input(&memory_id_to_big_inputs_4[i]);
            }
        });

    (trace, lookup_data)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    memory_address_to_id_0: Vec<[PackedM31; 2]>,
    memory_address_to_id_1: Vec<[PackedM31; 2]>,
    memory_address_to_id_2: Vec<[PackedM31; 2]>,
    memory_address_to_id_3: Vec<[PackedM31; 2]>,
    memory_address_to_id_4: Vec<[PackedM31; 2]>,
    memory_id_to_big_0: Vec<[PackedM31; 29]>,
    memory_id_to_big_1: Vec<[PackedM31; 29]>,
    memory_id_to_big_2: Vec<[PackedM31; 29]>,
    memory_id_to_big_3: Vec<[PackedM31; 29]>,
    memory_id_to_big_4: Vec<[PackedM31; 29]>,
    verify_bitwise_xor_9_0: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_9_1: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_9_2: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_9_3: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_9_4: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_9_5: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_9_6: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_9_7: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_9_8: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_9_9: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_9_10: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_9_11: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_9_12: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_9_13: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_9_14: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_9_15: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_9_16: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_9_17: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_9_18: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_9_19: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_9_20: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_9_21: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_9_22: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_9_23: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_9_24: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_9_25: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_9_26: Vec<[PackedM31; 3]>,
    verify_bitwise_xor_9_27: Vec<[PackedM31; 3]>,
}

pub struct InteractionClaimGenerator {
    log_size: u32,
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        memory_address_to_id: &relations::MemoryAddressToId,
        memory_id_to_big: &relations::MemoryIdToBig,
        verify_bitwise_xor_9: &relations::VerifyBitwiseXor_9,
    ) -> InteractionClaim
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_address_to_id_0,
            &self.lookup_data.memory_id_to_big_0,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_address_to_id.combine(values0);
            let denom1: PackedQM31 = memory_id_to_big.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_address_to_id_1,
            &self.lookup_data.memory_id_to_big_1,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_address_to_id.combine(values0);
            let denom1: PackedQM31 = memory_id_to_big.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.verify_bitwise_xor_9_0,
            &self.lookup_data.verify_bitwise_xor_9_1,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = verify_bitwise_xor_9.combine(values0);
            let denom1: PackedQM31 = verify_bitwise_xor_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.verify_bitwise_xor_9_2,
            &self.lookup_data.verify_bitwise_xor_9_3,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = verify_bitwise_xor_9.combine(values0);
            let denom1: PackedQM31 = verify_bitwise_xor_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.verify_bitwise_xor_9_4,
            &self.lookup_data.verify_bitwise_xor_9_5,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = verify_bitwise_xor_9.combine(values0);
            let denom1: PackedQM31 = verify_bitwise_xor_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.verify_bitwise_xor_9_6,
            &self.lookup_data.verify_bitwise_xor_9_7,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = verify_bitwise_xor_9.combine(values0);
            let denom1: PackedQM31 = verify_bitwise_xor_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.verify_bitwise_xor_9_8,
            &self.lookup_data.verify_bitwise_xor_9_9,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = verify_bitwise_xor_9.combine(values0);
            let denom1: PackedQM31 = verify_bitwise_xor_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.verify_bitwise_xor_9_10,
            &self.lookup_data.verify_bitwise_xor_9_11,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = verify_bitwise_xor_9.combine(values0);
            let denom1: PackedQM31 = verify_bitwise_xor_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.verify_bitwise_xor_9_12,
            &self.lookup_data.verify_bitwise_xor_9_13,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = verify_bitwise_xor_9.combine(values0);
            let denom1: PackedQM31 = verify_bitwise_xor_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.verify_bitwise_xor_9_14,
            &self.lookup_data.verify_bitwise_xor_9_15,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = verify_bitwise_xor_9.combine(values0);
            let denom1: PackedQM31 = verify_bitwise_xor_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.verify_bitwise_xor_9_16,
            &self.lookup_data.verify_bitwise_xor_9_17,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = verify_bitwise_xor_9.combine(values0);
            let denom1: PackedQM31 = verify_bitwise_xor_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.verify_bitwise_xor_9_18,
            &self.lookup_data.verify_bitwise_xor_9_19,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = verify_bitwise_xor_9.combine(values0);
            let denom1: PackedQM31 = verify_bitwise_xor_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.verify_bitwise_xor_9_20,
            &self.lookup_data.verify_bitwise_xor_9_21,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = verify_bitwise_xor_9.combine(values0);
            let denom1: PackedQM31 = verify_bitwise_xor_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.verify_bitwise_xor_9_22,
            &self.lookup_data.verify_bitwise_xor_9_23,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = verify_bitwise_xor_9.combine(values0);
            let denom1: PackedQM31 = verify_bitwise_xor_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.verify_bitwise_xor_9_24,
            &self.lookup_data.verify_bitwise_xor_9_25,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = verify_bitwise_xor_9.combine(values0);
            let denom1: PackedQM31 = verify_bitwise_xor_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.verify_bitwise_xor_9_26,
            &self.lookup_data.verify_bitwise_xor_9_27,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = verify_bitwise_xor_9.combine(values0);
            let denom1: PackedQM31 = verify_bitwise_xor_9.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_address_to_id_2,
            &self.lookup_data.memory_id_to_big_2,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_address_to_id.combine(values0);
            let denom1: PackedQM31 = memory_id_to_big.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_address_to_id_3,
            &self.lookup_data.memory_id_to_big_3,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_address_to_id.combine(values0);
            let denom1: PackedQM31 = memory_id_to_big.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_address_to_id_4,
            &self.lookup_data.memory_id_to_big_4,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_address_to_id.combine(values0);
            let denom1: PackedQM31 = memory_id_to_big.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}
