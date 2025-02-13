#![allow(unused_parens)]
#![allow(unused_imports)]
use std::iter::zip;

use itertools::{chain, zip_eq, Itertools};
use num_traits::{One, Zero};
use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};
use stwo_air_utils::trace::component_trace::ComponentTrace;
use stwo_air_utils_derive::{IterMut, ParIterMut, Uninitialized};
use stwo_cairo_common::prover_types::cpu::*;
use stwo_cairo_common::prover_types::simd::*;
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
use crate::components::utils::{pack_values, Enabler};
use crate::components::{memory_address_to_id, memory_id_to_big, verify_instruction};
use crate::relations;

pub type InputType = CasmState;
pub type PackedInputType = PackedCasmState;
const N_TRACE_COLUMNS: usize = 99;

#[derive(Default)]
pub struct ClaimGenerator {
    pub inputs: Vec<InputType>,
}
impl ClaimGenerator {
    pub fn new(inputs: Vec<InputType>) -> Self {
        Self { inputs }
    }

    pub fn write_trace<MC: MerkleChannel>(
        mut self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
        memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
        verify_instruction_state: &verify_instruction::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator)
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let n_rows = self.inputs.len();
        assert_ne!(n_rows, 0);
        let size = std::cmp::max(n_rows.next_power_of_two(), N_LANES);
        self.inputs.resize(size, *self.inputs.first().unwrap());
        let packed_inputs = pack_values(&self.inputs);

        let (trace, lookup_data) = write_trace_simd(
            n_rows,
            packed_inputs,
            memory_address_to_id_state,
            memory_id_to_big_state,
            verify_instruction_state,
        );
        tree_builder.extend_evals(trace.to_evals());

        (
            Claim { n_rows },
            InteractionClaimGenerator {
                n_rows,
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
    n_rows: usize,
    inputs: Vec<PackedInputType>,
    memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
    memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
    verify_instruction_state: &verify_instruction::ClaimGenerator,
) -> (ComponentTrace<N_TRACE_COLUMNS>, LookupData) {
    let log_n_packed_rows = inputs.len().ilog2();
    let log_size = log_n_packed_rows + LOG_N_LANES;
    let (mut trace, mut lookup_data) = unsafe {
        (
            ComponentTrace::<N_TRACE_COLUMNS>::uninitialized(log_size),
            LookupData::uninitialized(log_n_packed_rows),
        )
    };

    let M31_0 = PackedM31::broadcast(M31::from(0));
    let M31_1 = PackedM31::broadcast(M31::from(1));
    let M31_2 = PackedM31::broadcast(M31::from(2));
    let M31_32768 = PackedM31::broadcast(M31::from(32768));
    let M31_32769 = PackedM31::broadcast(M31::from(32769));
    let UInt16_0 = PackedUInt16::broadcast(UInt16::from(0));
    let UInt16_1 = PackedUInt16::broadcast(UInt16::from(1));
    let UInt16_11 = PackedUInt16::broadcast(UInt16::from(11));
    let UInt16_127 = PackedUInt16::broadcast(UInt16::from(127));
    let UInt16_2 = PackedUInt16::broadcast(UInt16::from(2));
    let UInt16_3 = PackedUInt16::broadcast(UInt16::from(3));
    let UInt16_31 = PackedUInt16::broadcast(UInt16::from(31));
    let UInt16_6 = PackedUInt16::broadcast(UInt16::from(6));
    let UInt16_7 = PackedUInt16::broadcast(UInt16::from(7));
    let UInt16_9 = PackedUInt16::broadcast(UInt16::from(9));
    let padding_col = Enabler::new(n_rows);

    trace
        .par_iter_mut()
        .enumerate()
        .zip(inputs.into_par_iter())
        .zip(lookup_data.par_iter_mut())
        .for_each(|(((row_index, row), add_opcode_imm_input), lookup_data)| {
            let input_tmp_7e965_0 = add_opcode_imm_input;
            let input_pc_col0 = input_tmp_7e965_0.pc;
            *row[0] = input_pc_col0;
            let input_ap_col1 = input_tmp_7e965_0.ap;
            *row[1] = input_ap_col1;
            let input_fp_col2 = input_tmp_7e965_0.fp;
            *row[2] = input_fp_col2;

            // Decode Instruction.

            let memory_address_to_id_value_tmp_7e965_1 =
                memory_address_to_id_state.deduce_output(input_pc_col0);
            let memory_id_to_big_value_tmp_7e965_2 =
                memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_7e965_1);
            let offset0_tmp_7e965_3 =
                ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_7e965_2.get_m31(0)))
                    + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_7e965_2.get_m31(1)))
                        & (UInt16_127))
                        << (UInt16_9)));
            let offset0_col3 = offset0_tmp_7e965_3.as_m31();
            *row[3] = offset0_col3;
            let offset1_tmp_7e965_4 =
                ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_7e965_2.get_m31(1)))
                    >> (UInt16_7))
                    + ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_7e965_2.get_m31(2)))
                        << (UInt16_2)))
                    + (((PackedUInt16::from_m31(memory_id_to_big_value_tmp_7e965_2.get_m31(3)))
                        & (UInt16_31))
                        << (UInt16_11)));
            let offset1_col4 = offset1_tmp_7e965_4.as_m31();
            *row[4] = offset1_col4;
            let dst_base_fp_tmp_7e965_5 =
                (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_7e965_2.get_m31(5)))
                    >> (UInt16_3))
                    + ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_7e965_2.get_m31(6)))
                        << (UInt16_6)))
                    >> (UInt16_0))
                    & (UInt16_1));
            let dst_base_fp_col5 = dst_base_fp_tmp_7e965_5.as_m31();
            *row[5] = dst_base_fp_col5;
            let op0_base_fp_tmp_7e965_6 =
                (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_7e965_2.get_m31(5)))
                    >> (UInt16_3))
                    + ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_7e965_2.get_m31(6)))
                        << (UInt16_6)))
                    >> (UInt16_1))
                    & (UInt16_1));
            let op0_base_fp_col6 = op0_base_fp_tmp_7e965_6.as_m31();
            *row[6] = op0_base_fp_col6;
            let ap_update_add_1_tmp_7e965_7 =
                (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_7e965_2.get_m31(5)))
                    >> (UInt16_3))
                    + ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_7e965_2.get_m31(6)))
                        << (UInt16_6)))
                    >> (UInt16_11))
                    & (UInt16_1));
            let ap_update_add_1_col7 = ap_update_add_1_tmp_7e965_7.as_m31();
            *row[7] = ap_update_add_1_col7;
            let verify_instruction_inputs_0 = (
                input_pc_col0,
                [offset0_col3, offset1_col4, M31_32769],
                [
                    dst_base_fp_col5,
                    op0_base_fp_col6,
                    M31_1,
                    M31_0,
                    M31_0,
                    M31_1,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    ap_update_add_1_col7,
                    M31_0,
                    M31_0,
                    M31_1,
                ],
            )
                .unpack();
            *lookup_data.verify_instruction_0 = [
                input_pc_col0,
                offset0_col3,
                offset1_col4,
                M31_32769,
                dst_base_fp_col5,
                op0_base_fp_col6,
                M31_1,
                M31_0,
                M31_0,
                M31_1,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                M31_0,
                ap_update_add_1_col7,
                M31_0,
                M31_0,
                M31_1,
            ];

            let mem_dst_base_col8 = (((dst_base_fp_col5) * (input_fp_col2))
                + (((M31_1) - (dst_base_fp_col5)) * (input_ap_col1)));
            *row[8] = mem_dst_base_col8;
            let mem0_base_col9 = (((op0_base_fp_col6) * (input_fp_col2))
                + (((M31_1) - (op0_base_fp_col6)) * (input_ap_col1)));
            *row[9] = mem0_base_col9;

            // Read Positive Num Bits 252.

            let memory_address_to_id_value_tmp_7e965_8 = memory_address_to_id_state
                .deduce_output(((mem_dst_base_col8) + ((offset0_col3) - (M31_32768))));
            let memory_id_to_big_value_tmp_7e965_9 =
                memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_7e965_8);
            let dst_id_col10 = memory_address_to_id_value_tmp_7e965_8;
            *row[10] = dst_id_col10;
            let memory_address_to_id_inputs_0 =
                ((mem_dst_base_col8) + ((offset0_col3) - (M31_32768))).unpack();
            *lookup_data.memory_address_to_id_0 = [
                ((mem_dst_base_col8) + ((offset0_col3) - (M31_32768))),
                dst_id_col10,
            ];
            let dst_limb_0_col11 = memory_id_to_big_value_tmp_7e965_9.get_m31(0);
            *row[11] = dst_limb_0_col11;
            let dst_limb_1_col12 = memory_id_to_big_value_tmp_7e965_9.get_m31(1);
            *row[12] = dst_limb_1_col12;
            let dst_limb_2_col13 = memory_id_to_big_value_tmp_7e965_9.get_m31(2);
            *row[13] = dst_limb_2_col13;
            let dst_limb_3_col14 = memory_id_to_big_value_tmp_7e965_9.get_m31(3);
            *row[14] = dst_limb_3_col14;
            let dst_limb_4_col15 = memory_id_to_big_value_tmp_7e965_9.get_m31(4);
            *row[15] = dst_limb_4_col15;
            let dst_limb_5_col16 = memory_id_to_big_value_tmp_7e965_9.get_m31(5);
            *row[16] = dst_limb_5_col16;
            let dst_limb_6_col17 = memory_id_to_big_value_tmp_7e965_9.get_m31(6);
            *row[17] = dst_limb_6_col17;
            let dst_limb_7_col18 = memory_id_to_big_value_tmp_7e965_9.get_m31(7);
            *row[18] = dst_limb_7_col18;
            let dst_limb_8_col19 = memory_id_to_big_value_tmp_7e965_9.get_m31(8);
            *row[19] = dst_limb_8_col19;
            let dst_limb_9_col20 = memory_id_to_big_value_tmp_7e965_9.get_m31(9);
            *row[20] = dst_limb_9_col20;
            let dst_limb_10_col21 = memory_id_to_big_value_tmp_7e965_9.get_m31(10);
            *row[21] = dst_limb_10_col21;
            let dst_limb_11_col22 = memory_id_to_big_value_tmp_7e965_9.get_m31(11);
            *row[22] = dst_limb_11_col22;
            let dst_limb_12_col23 = memory_id_to_big_value_tmp_7e965_9.get_m31(12);
            *row[23] = dst_limb_12_col23;
            let dst_limb_13_col24 = memory_id_to_big_value_tmp_7e965_9.get_m31(13);
            *row[24] = dst_limb_13_col24;
            let dst_limb_14_col25 = memory_id_to_big_value_tmp_7e965_9.get_m31(14);
            *row[25] = dst_limb_14_col25;
            let dst_limb_15_col26 = memory_id_to_big_value_tmp_7e965_9.get_m31(15);
            *row[26] = dst_limb_15_col26;
            let dst_limb_16_col27 = memory_id_to_big_value_tmp_7e965_9.get_m31(16);
            *row[27] = dst_limb_16_col27;
            let dst_limb_17_col28 = memory_id_to_big_value_tmp_7e965_9.get_m31(17);
            *row[28] = dst_limb_17_col28;
            let dst_limb_18_col29 = memory_id_to_big_value_tmp_7e965_9.get_m31(18);
            *row[29] = dst_limb_18_col29;
            let dst_limb_19_col30 = memory_id_to_big_value_tmp_7e965_9.get_m31(19);
            *row[30] = dst_limb_19_col30;
            let dst_limb_20_col31 = memory_id_to_big_value_tmp_7e965_9.get_m31(20);
            *row[31] = dst_limb_20_col31;
            let dst_limb_21_col32 = memory_id_to_big_value_tmp_7e965_9.get_m31(21);
            *row[32] = dst_limb_21_col32;
            let dst_limb_22_col33 = memory_id_to_big_value_tmp_7e965_9.get_m31(22);
            *row[33] = dst_limb_22_col33;
            let dst_limb_23_col34 = memory_id_to_big_value_tmp_7e965_9.get_m31(23);
            *row[34] = dst_limb_23_col34;
            let dst_limb_24_col35 = memory_id_to_big_value_tmp_7e965_9.get_m31(24);
            *row[35] = dst_limb_24_col35;
            let dst_limb_25_col36 = memory_id_to_big_value_tmp_7e965_9.get_m31(25);
            *row[36] = dst_limb_25_col36;
            let dst_limb_26_col37 = memory_id_to_big_value_tmp_7e965_9.get_m31(26);
            *row[37] = dst_limb_26_col37;
            let dst_limb_27_col38 = memory_id_to_big_value_tmp_7e965_9.get_m31(27);
            *row[38] = dst_limb_27_col38;
            let memory_id_to_big_inputs_0 = dst_id_col10.unpack();
            *lookup_data.memory_id_to_big_0 = [
                dst_id_col10,
                dst_limb_0_col11,
                dst_limb_1_col12,
                dst_limb_2_col13,
                dst_limb_3_col14,
                dst_limb_4_col15,
                dst_limb_5_col16,
                dst_limb_6_col17,
                dst_limb_7_col18,
                dst_limb_8_col19,
                dst_limb_9_col20,
                dst_limb_10_col21,
                dst_limb_11_col22,
                dst_limb_12_col23,
                dst_limb_13_col24,
                dst_limb_14_col25,
                dst_limb_15_col26,
                dst_limb_16_col27,
                dst_limb_17_col28,
                dst_limb_18_col29,
                dst_limb_19_col30,
                dst_limb_20_col31,
                dst_limb_21_col32,
                dst_limb_22_col33,
                dst_limb_23_col34,
                dst_limb_24_col35,
                dst_limb_25_col36,
                dst_limb_26_col37,
                dst_limb_27_col38,
            ];

            // Read Positive Num Bits 252.

            let memory_address_to_id_value_tmp_7e965_10 = memory_address_to_id_state
                .deduce_output(((mem0_base_col9) + ((offset1_col4) - (M31_32768))));
            let memory_id_to_big_value_tmp_7e965_11 =
                memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_7e965_10);
            let op0_id_col39 = memory_address_to_id_value_tmp_7e965_10;
            *row[39] = op0_id_col39;
            let memory_address_to_id_inputs_1 =
                ((mem0_base_col9) + ((offset1_col4) - (M31_32768))).unpack();
            *lookup_data.memory_address_to_id_1 = [
                ((mem0_base_col9) + ((offset1_col4) - (M31_32768))),
                op0_id_col39,
            ];
            let op0_limb_0_col40 = memory_id_to_big_value_tmp_7e965_11.get_m31(0);
            *row[40] = op0_limb_0_col40;
            let op0_limb_1_col41 = memory_id_to_big_value_tmp_7e965_11.get_m31(1);
            *row[41] = op0_limb_1_col41;
            let op0_limb_2_col42 = memory_id_to_big_value_tmp_7e965_11.get_m31(2);
            *row[42] = op0_limb_2_col42;
            let op0_limb_3_col43 = memory_id_to_big_value_tmp_7e965_11.get_m31(3);
            *row[43] = op0_limb_3_col43;
            let op0_limb_4_col44 = memory_id_to_big_value_tmp_7e965_11.get_m31(4);
            *row[44] = op0_limb_4_col44;
            let op0_limb_5_col45 = memory_id_to_big_value_tmp_7e965_11.get_m31(5);
            *row[45] = op0_limb_5_col45;
            let op0_limb_6_col46 = memory_id_to_big_value_tmp_7e965_11.get_m31(6);
            *row[46] = op0_limb_6_col46;
            let op0_limb_7_col47 = memory_id_to_big_value_tmp_7e965_11.get_m31(7);
            *row[47] = op0_limb_7_col47;
            let op0_limb_8_col48 = memory_id_to_big_value_tmp_7e965_11.get_m31(8);
            *row[48] = op0_limb_8_col48;
            let op0_limb_9_col49 = memory_id_to_big_value_tmp_7e965_11.get_m31(9);
            *row[49] = op0_limb_9_col49;
            let op0_limb_10_col50 = memory_id_to_big_value_tmp_7e965_11.get_m31(10);
            *row[50] = op0_limb_10_col50;
            let op0_limb_11_col51 = memory_id_to_big_value_tmp_7e965_11.get_m31(11);
            *row[51] = op0_limb_11_col51;
            let op0_limb_12_col52 = memory_id_to_big_value_tmp_7e965_11.get_m31(12);
            *row[52] = op0_limb_12_col52;
            let op0_limb_13_col53 = memory_id_to_big_value_tmp_7e965_11.get_m31(13);
            *row[53] = op0_limb_13_col53;
            let op0_limb_14_col54 = memory_id_to_big_value_tmp_7e965_11.get_m31(14);
            *row[54] = op0_limb_14_col54;
            let op0_limb_15_col55 = memory_id_to_big_value_tmp_7e965_11.get_m31(15);
            *row[55] = op0_limb_15_col55;
            let op0_limb_16_col56 = memory_id_to_big_value_tmp_7e965_11.get_m31(16);
            *row[56] = op0_limb_16_col56;
            let op0_limb_17_col57 = memory_id_to_big_value_tmp_7e965_11.get_m31(17);
            *row[57] = op0_limb_17_col57;
            let op0_limb_18_col58 = memory_id_to_big_value_tmp_7e965_11.get_m31(18);
            *row[58] = op0_limb_18_col58;
            let op0_limb_19_col59 = memory_id_to_big_value_tmp_7e965_11.get_m31(19);
            *row[59] = op0_limb_19_col59;
            let op0_limb_20_col60 = memory_id_to_big_value_tmp_7e965_11.get_m31(20);
            *row[60] = op0_limb_20_col60;
            let op0_limb_21_col61 = memory_id_to_big_value_tmp_7e965_11.get_m31(21);
            *row[61] = op0_limb_21_col61;
            let op0_limb_22_col62 = memory_id_to_big_value_tmp_7e965_11.get_m31(22);
            *row[62] = op0_limb_22_col62;
            let op0_limb_23_col63 = memory_id_to_big_value_tmp_7e965_11.get_m31(23);
            *row[63] = op0_limb_23_col63;
            let op0_limb_24_col64 = memory_id_to_big_value_tmp_7e965_11.get_m31(24);
            *row[64] = op0_limb_24_col64;
            let op0_limb_25_col65 = memory_id_to_big_value_tmp_7e965_11.get_m31(25);
            *row[65] = op0_limb_25_col65;
            let op0_limb_26_col66 = memory_id_to_big_value_tmp_7e965_11.get_m31(26);
            *row[66] = op0_limb_26_col66;
            let op0_limb_27_col67 = memory_id_to_big_value_tmp_7e965_11.get_m31(27);
            *row[67] = op0_limb_27_col67;
            let memory_id_to_big_inputs_1 = op0_id_col39.unpack();
            *lookup_data.memory_id_to_big_1 = [
                op0_id_col39,
                op0_limb_0_col40,
                op0_limb_1_col41,
                op0_limb_2_col42,
                op0_limb_3_col43,
                op0_limb_4_col44,
                op0_limb_5_col45,
                op0_limb_6_col46,
                op0_limb_7_col47,
                op0_limb_8_col48,
                op0_limb_9_col49,
                op0_limb_10_col50,
                op0_limb_11_col51,
                op0_limb_12_col52,
                op0_limb_13_col53,
                op0_limb_14_col54,
                op0_limb_15_col55,
                op0_limb_16_col56,
                op0_limb_17_col57,
                op0_limb_18_col58,
                op0_limb_19_col59,
                op0_limb_20_col60,
                op0_limb_21_col61,
                op0_limb_22_col62,
                op0_limb_23_col63,
                op0_limb_24_col64,
                op0_limb_25_col65,
                op0_limb_26_col66,
                op0_limb_27_col67,
            ];

            // Read Positive Num Bits 252.

            let memory_address_to_id_value_tmp_7e965_12 =
                memory_address_to_id_state.deduce_output(((input_pc_col0) + (M31_1)));
            let memory_id_to_big_value_tmp_7e965_13 =
                memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_7e965_12);
            let op1_id_col68 = memory_address_to_id_value_tmp_7e965_12;
            *row[68] = op1_id_col68;
            let memory_address_to_id_inputs_2 = ((input_pc_col0) + (M31_1)).unpack();
            *lookup_data.memory_address_to_id_2 = [((input_pc_col0) + (M31_1)), op1_id_col68];
            let op1_limb_0_col69 = memory_id_to_big_value_tmp_7e965_13.get_m31(0);
            *row[69] = op1_limb_0_col69;
            let op1_limb_1_col70 = memory_id_to_big_value_tmp_7e965_13.get_m31(1);
            *row[70] = op1_limb_1_col70;
            let op1_limb_2_col71 = memory_id_to_big_value_tmp_7e965_13.get_m31(2);
            *row[71] = op1_limb_2_col71;
            let op1_limb_3_col72 = memory_id_to_big_value_tmp_7e965_13.get_m31(3);
            *row[72] = op1_limb_3_col72;
            let op1_limb_4_col73 = memory_id_to_big_value_tmp_7e965_13.get_m31(4);
            *row[73] = op1_limb_4_col73;
            let op1_limb_5_col74 = memory_id_to_big_value_tmp_7e965_13.get_m31(5);
            *row[74] = op1_limb_5_col74;
            let op1_limb_6_col75 = memory_id_to_big_value_tmp_7e965_13.get_m31(6);
            *row[75] = op1_limb_6_col75;
            let op1_limb_7_col76 = memory_id_to_big_value_tmp_7e965_13.get_m31(7);
            *row[76] = op1_limb_7_col76;
            let op1_limb_8_col77 = memory_id_to_big_value_tmp_7e965_13.get_m31(8);
            *row[77] = op1_limb_8_col77;
            let op1_limb_9_col78 = memory_id_to_big_value_tmp_7e965_13.get_m31(9);
            *row[78] = op1_limb_9_col78;
            let op1_limb_10_col79 = memory_id_to_big_value_tmp_7e965_13.get_m31(10);
            *row[79] = op1_limb_10_col79;
            let op1_limb_11_col80 = memory_id_to_big_value_tmp_7e965_13.get_m31(11);
            *row[80] = op1_limb_11_col80;
            let op1_limb_12_col81 = memory_id_to_big_value_tmp_7e965_13.get_m31(12);
            *row[81] = op1_limb_12_col81;
            let op1_limb_13_col82 = memory_id_to_big_value_tmp_7e965_13.get_m31(13);
            *row[82] = op1_limb_13_col82;
            let op1_limb_14_col83 = memory_id_to_big_value_tmp_7e965_13.get_m31(14);
            *row[83] = op1_limb_14_col83;
            let op1_limb_15_col84 = memory_id_to_big_value_tmp_7e965_13.get_m31(15);
            *row[84] = op1_limb_15_col84;
            let op1_limb_16_col85 = memory_id_to_big_value_tmp_7e965_13.get_m31(16);
            *row[85] = op1_limb_16_col85;
            let op1_limb_17_col86 = memory_id_to_big_value_tmp_7e965_13.get_m31(17);
            *row[86] = op1_limb_17_col86;
            let op1_limb_18_col87 = memory_id_to_big_value_tmp_7e965_13.get_m31(18);
            *row[87] = op1_limb_18_col87;
            let op1_limb_19_col88 = memory_id_to_big_value_tmp_7e965_13.get_m31(19);
            *row[88] = op1_limb_19_col88;
            let op1_limb_20_col89 = memory_id_to_big_value_tmp_7e965_13.get_m31(20);
            *row[89] = op1_limb_20_col89;
            let op1_limb_21_col90 = memory_id_to_big_value_tmp_7e965_13.get_m31(21);
            *row[90] = op1_limb_21_col90;
            let op1_limb_22_col91 = memory_id_to_big_value_tmp_7e965_13.get_m31(22);
            *row[91] = op1_limb_22_col91;
            let op1_limb_23_col92 = memory_id_to_big_value_tmp_7e965_13.get_m31(23);
            *row[92] = op1_limb_23_col92;
            let op1_limb_24_col93 = memory_id_to_big_value_tmp_7e965_13.get_m31(24);
            *row[93] = op1_limb_24_col93;
            let op1_limb_25_col94 = memory_id_to_big_value_tmp_7e965_13.get_m31(25);
            *row[94] = op1_limb_25_col94;
            let op1_limb_26_col95 = memory_id_to_big_value_tmp_7e965_13.get_m31(26);
            *row[95] = op1_limb_26_col95;
            let op1_limb_27_col96 = memory_id_to_big_value_tmp_7e965_13.get_m31(27);
            *row[96] = op1_limb_27_col96;
            let memory_id_to_big_inputs_2 = op1_id_col68.unpack();
            *lookup_data.memory_id_to_big_2 = [
                op1_id_col68,
                op1_limb_0_col69,
                op1_limb_1_col70,
                op1_limb_2_col71,
                op1_limb_3_col72,
                op1_limb_4_col73,
                op1_limb_5_col74,
                op1_limb_6_col75,
                op1_limb_7_col76,
                op1_limb_8_col77,
                op1_limb_9_col78,
                op1_limb_10_col79,
                op1_limb_11_col80,
                op1_limb_12_col81,
                op1_limb_13_col82,
                op1_limb_14_col83,
                op1_limb_15_col84,
                op1_limb_16_col85,
                op1_limb_17_col86,
                op1_limb_18_col87,
                op1_limb_19_col88,
                op1_limb_20_col89,
                op1_limb_21_col90,
                op1_limb_22_col91,
                op1_limb_23_col92,
                op1_limb_24_col93,
                op1_limb_25_col94,
                op1_limb_26_col95,
                op1_limb_27_col96,
            ];

            // Verify Add 252.

            let sub_p_bit_tmp_7e965_14 = ((UInt16_1)
                & (((PackedUInt16::from_m31(op0_limb_0_col40))
                    ^ (PackedUInt16::from_m31(op1_limb_0_col69)))
                    ^ (PackedUInt16::from_m31(dst_limb_0_col11))));
            let sub_p_bit_col97 = sub_p_bit_tmp_7e965_14.as_m31();
            *row[97] = sub_p_bit_col97;

            *lookup_data.opcodes_0 = [input_pc_col0, input_ap_col1, input_fp_col2];
            *lookup_data.opcodes_1 = [
                ((input_pc_col0) + (M31_2)),
                ((input_ap_col1) + (ap_update_add_1_col7)),
                input_fp_col2,
            ];
            *row[98] = padding_col.packed_at(row_index);

            // Add sub-components inputs.
            verify_instruction_state.add_inputs(&verify_instruction_inputs_0);
            memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_0);
            memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_0);
            memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_1);
            memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_1);
            memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_2);
            memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_2);
        });

    (trace, lookup_data)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    memory_address_to_id_0: Vec<[PackedM31; 2]>,
    memory_address_to_id_1: Vec<[PackedM31; 2]>,
    memory_address_to_id_2: Vec<[PackedM31; 2]>,
    memory_id_to_big_0: Vec<[PackedM31; 29]>,
    memory_id_to_big_1: Vec<[PackedM31; 29]>,
    memory_id_to_big_2: Vec<[PackedM31; 29]>,
    opcodes_0: Vec<[PackedM31; 3]>,
    opcodes_1: Vec<[PackedM31; 3]>,
    verify_instruction_0: Vec<[PackedM31; 19]>,
}

pub struct InteractionClaimGenerator {
    n_rows: usize,
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        memory_address_to_id: &relations::MemoryAddressToId,
        memory_id_to_big: &relations::MemoryIdToBig,
        opcodes: &relations::Opcodes,
        verify_instruction: &relations::VerifyInstruction,
    ) -> InteractionClaim
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let log_size = std::cmp::max(self.n_rows.next_power_of_two().ilog2(), LOG_N_LANES);
        let padding_col = Enabler::new(self.n_rows);
        let mut logup_gen = LogupTraceGenerator::new(log_size);

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.verify_instruction_0,
            &self.lookup_data.memory_address_to_id_0,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = verify_instruction.combine(values0);
            let denom1: PackedQM31 = memory_address_to_id.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_id_to_big_0,
            &self.lookup_data.memory_address_to_id_1,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_id_to_big.combine(values0);
            let denom1: PackedQM31 = memory_address_to_id.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_id_to_big_1,
            &self.lookup_data.memory_address_to_id_2,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_id_to_big.combine(values0);
            let denom1: PackedQM31 = memory_address_to_id.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_id_to_big_2,
            &self.lookup_data.opcodes_0,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_id_to_big.combine(values0);
            let denom1: PackedQM31 = opcodes.combine(values1);
            col_gen.write_frac(
                i,
                denom0 * padding_col.packed_at(i) + denom1,
                denom0 * denom1,
            );
        }
        col_gen.finalize_col();

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        for (i, values) in self.lookup_data.opcodes_1.iter().enumerate() {
            let denom = opcodes.combine(values);
            col_gen.write_frac(i, -PackedQM31::one() * padding_col.packed_at(i), denom);
        }
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}
