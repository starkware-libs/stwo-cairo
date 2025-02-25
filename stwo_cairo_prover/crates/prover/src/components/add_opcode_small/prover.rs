#![allow(unused_parens)]
#![allow(unused_imports)]
use super::component::{Claim, InteractionClaim};
use crate::components::prelude::proving::*;
use crate::components::{memory_address_to_id, memory_id_to_big, verify_instruction};

pub type InputType = CasmState;
pub type PackedInputType = PackedCasmState;
const N_TRACE_COLUMNS: usize = 33;

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
        let log_size = size.ilog2();
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
            Claim { log_size },
            InteractionClaimGenerator {
                n_rows,
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
    let M31_128 = PackedM31::broadcast(M31::from(128));
    let M31_136 = PackedM31::broadcast(M31::from(136));
    let M31_16 = PackedM31::broadcast(M31::from(16));
    let M31_256 = PackedM31::broadcast(M31::from(256));
    let M31_32 = PackedM31::broadcast(M31::from(32));
    let M31_32768 = PackedM31::broadcast(M31::from(32768));
    let M31_511 = PackedM31::broadcast(M31::from(511));
    let M31_64 = PackedM31::broadcast(M31::from(64));
    let M31_8 = PackedM31::broadcast(M31::from(8));
    let UInt16_0 = PackedUInt16::broadcast(UInt16::from(0));
    let UInt16_1 = PackedUInt16::broadcast(UInt16::from(1));
    let UInt16_11 = PackedUInt16::broadcast(UInt16::from(11));
    let UInt16_127 = PackedUInt16::broadcast(UInt16::from(127));
    let UInt16_13 = PackedUInt16::broadcast(UInt16::from(13));
    let UInt16_2 = PackedUInt16::broadcast(UInt16::from(2));
    let UInt16_3 = PackedUInt16::broadcast(UInt16::from(3));
    let UInt16_31 = PackedUInt16::broadcast(UInt16::from(31));
    let UInt16_4 = PackedUInt16::broadcast(UInt16::from(4));
    let UInt16_5 = PackedUInt16::broadcast(UInt16::from(5));
    let UInt16_6 = PackedUInt16::broadcast(UInt16::from(6));
    let UInt16_7 = PackedUInt16::broadcast(UInt16::from(7));
    let UInt16_9 = PackedUInt16::broadcast(UInt16::from(9));

    let padding = Enabler::new(n_rows);

    trace
        .par_iter_mut()
        .enumerate()
        .zip(inputs.into_par_iter())
        .zip(lookup_data.par_iter_mut())
        .for_each(
            |(((row_index, row), add_opcode_small_input), lookup_data)| {
                let input_tmp_8d553_0 = add_opcode_small_input;
                let input_pc_col0 = input_tmp_8d553_0.pc;
                *row[0] = input_pc_col0;
                let input_ap_col1 = input_tmp_8d553_0.ap;
                *row[1] = input_ap_col1;
                let input_fp_col2 = input_tmp_8d553_0.fp;
                *row[2] = input_fp_col2;

                // Decode Instruction.

                let memory_address_to_id_value_tmp_8d553_1 =
                    memory_address_to_id_state.deduce_output(input_pc_col0);
                let memory_id_to_big_value_tmp_8d553_2 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_8d553_1);
                let offset0_tmp_8d553_3 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_8d553_2.get_m31(0)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_8d553_2.get_m31(1),
                        )) & (UInt16_127))
                            << (UInt16_9)));
                let offset0_col3 = offset0_tmp_8d553_3.as_m31();
                *row[3] = offset0_col3;
                let offset1_tmp_8d553_4 =
                    ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_8d553_2.get_m31(1)))
                        >> (UInt16_7))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_8d553_2.get_m31(2),
                        )) << (UInt16_2)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_8d553_2.get_m31(3),
                        )) & (UInt16_31))
                            << (UInt16_11)));
                let offset1_col4 = offset1_tmp_8d553_4.as_m31();
                *row[4] = offset1_col4;
                let offset2_tmp_8d553_5 =
                    ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_8d553_2.get_m31(3)))
                        >> (UInt16_5))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_8d553_2.get_m31(4),
                        )) << (UInt16_4)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_8d553_2.get_m31(5),
                        )) & (UInt16_7))
                            << (UInt16_13)));
                let offset2_col5 = offset2_tmp_8d553_5.as_m31();
                *row[5] = offset2_col5;
                let dst_base_fp_tmp_8d553_6 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_8d553_2.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_8d553_2.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_0))
                        & (UInt16_1));
                let dst_base_fp_col6 = dst_base_fp_tmp_8d553_6.as_m31();
                *row[6] = dst_base_fp_col6;
                let op0_base_fp_tmp_8d553_7 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_8d553_2.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_8d553_2.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_1))
                        & (UInt16_1));
                let op0_base_fp_col7 = op0_base_fp_tmp_8d553_7.as_m31();
                *row[7] = op0_base_fp_col7;
                let op1_base_fp_tmp_8d553_8 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_8d553_2.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_8d553_2.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_3))
                        & (UInt16_1));
                let op1_base_fp_col8 = op1_base_fp_tmp_8d553_8.as_m31();
                *row[8] = op1_base_fp_col8;
                let op1_base_ap_tmp_8d553_9 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_8d553_2.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_8d553_2.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_4))
                        & (UInt16_1));
                let op1_base_ap_col9 = op1_base_ap_tmp_8d553_9.as_m31();
                *row[9] = op1_base_ap_col9;
                let ap_update_add_1_tmp_8d553_10 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_8d553_2.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_8d553_2.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_11))
                        & (UInt16_1));
                let ap_update_add_1_col10 = ap_update_add_1_tmp_8d553_10.as_m31();
                *row[10] = ap_update_add_1_col10;
                let verify_instruction_inputs_0 = (
                    input_pc_col0,
                    [offset0_col3, offset1_col4, offset2_col5],
                    [
                        (((((((M31_0) + ((dst_base_fp_col6) * (M31_8)))
                            + ((op0_base_fp_col7) * (M31_16)))
                            + (M31_0))
                            + ((op1_base_fp_col8) * (M31_64)))
                            + ((op1_base_ap_col9) * (M31_128)))
                            + (M31_256)),
                        (((((M31_0) + ((ap_update_add_1_col10) * (M31_32))) + (M31_0)) + (M31_0))
                            + (M31_256)),
                    ],
                    M31_0,
                )
                    .unpack();
                *lookup_data.verify_instruction_0 = [
                    input_pc_col0,
                    offset0_col3,
                    offset1_col4,
                    offset2_col5,
                    (((((((M31_0) + ((dst_base_fp_col6) * (M31_8)))
                        + ((op0_base_fp_col7) * (M31_16)))
                        + (M31_0))
                        + ((op1_base_fp_col8) * (M31_64)))
                        + ((op1_base_ap_col9) * (M31_128)))
                        + (M31_256)),
                    (((((M31_0) + ((ap_update_add_1_col10) * (M31_32))) + (M31_0)) + (M31_0))
                        + (M31_256)),
                    M31_0,
                ];

                let mem_dst_base_col11 = (((dst_base_fp_col6) * (input_fp_col2))
                    + (((M31_1) - (dst_base_fp_col6)) * (input_ap_col1)));
                *row[11] = mem_dst_base_col11;
                let mem0_base_col12 = (((op0_base_fp_col7) * (input_fp_col2))
                    + (((M31_1) - (op0_base_fp_col7)) * (input_ap_col1)));
                *row[12] = mem0_base_col12;
                let mem1_base_col13 = (((op1_base_fp_col8) * (input_fp_col2))
                    + ((op1_base_ap_col9) * (input_ap_col1)));
                *row[13] = mem1_base_col13;

                // Read Small.

                let memory_address_to_id_value_tmp_8d553_11 = memory_address_to_id_state
                    .deduce_output(((mem_dst_base_col11) + ((offset0_col3) - (M31_32768))));
                let memory_id_to_big_value_tmp_8d553_12 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_8d553_11);
                let dst_id_col14 = memory_address_to_id_value_tmp_8d553_11;
                *row[14] = dst_id_col14;
                let memory_address_to_id_inputs_0 =
                    ((mem_dst_base_col11) + ((offset0_col3) - (M31_32768))).unpack();
                *lookup_data.memory_address_to_id_0 = [
                    ((mem_dst_base_col11) + ((offset0_col3) - (M31_32768))),
                    dst_id_col14,
                ];

                // Cond Decode Small Sign.

                let msb_tmp_8d553_13 = memory_id_to_big_value_tmp_8d553_12.get_m31(27).eq(M31_256);
                let msb_col15 = msb_tmp_8d553_13.as_m31();
                *row[15] = msb_col15;
                let mid_limbs_set_tmp_8d553_14 =
                    memory_id_to_big_value_tmp_8d553_12.get_m31(20).eq(M31_511);
                let mid_limbs_set_col16 = mid_limbs_set_tmp_8d553_14.as_m31();
                *row[16] = mid_limbs_set_col16;

                let dst_limb_0_col17 = memory_id_to_big_value_tmp_8d553_12.get_m31(0);
                *row[17] = dst_limb_0_col17;
                let dst_limb_1_col18 = memory_id_to_big_value_tmp_8d553_12.get_m31(1);
                *row[18] = dst_limb_1_col18;
                let dst_limb_2_col19 = memory_id_to_big_value_tmp_8d553_12.get_m31(2);
                *row[19] = dst_limb_2_col19;
                let memory_id_to_big_inputs_0 = dst_id_col14.unpack();
                *lookup_data.memory_id_to_big_0 = [
                    dst_id_col14,
                    dst_limb_0_col17,
                    dst_limb_1_col18,
                    dst_limb_2_col19,
                    ((mid_limbs_set_col16) * (M31_511)),
                    ((mid_limbs_set_col16) * (M31_511)),
                    ((mid_limbs_set_col16) * (M31_511)),
                    ((mid_limbs_set_col16) * (M31_511)),
                    ((mid_limbs_set_col16) * (M31_511)),
                    ((mid_limbs_set_col16) * (M31_511)),
                    ((mid_limbs_set_col16) * (M31_511)),
                    ((mid_limbs_set_col16) * (M31_511)),
                    ((mid_limbs_set_col16) * (M31_511)),
                    ((mid_limbs_set_col16) * (M31_511)),
                    ((mid_limbs_set_col16) * (M31_511)),
                    ((mid_limbs_set_col16) * (M31_511)),
                    ((mid_limbs_set_col16) * (M31_511)),
                    ((mid_limbs_set_col16) * (M31_511)),
                    ((mid_limbs_set_col16) * (M31_511)),
                    ((mid_limbs_set_col16) * (M31_511)),
                    ((mid_limbs_set_col16) * (M31_511)),
                    ((mid_limbs_set_col16) * (M31_511)),
                    (((M31_136) * (msb_col15)) - (mid_limbs_set_col16)),
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    ((msb_col15) * (M31_256)),
                ];

                // Read Small.

                let memory_address_to_id_value_tmp_8d553_15 = memory_address_to_id_state
                    .deduce_output(((mem0_base_col12) + ((offset1_col4) - (M31_32768))));
                let memory_id_to_big_value_tmp_8d553_16 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_8d553_15);
                let op0_id_col20 = memory_address_to_id_value_tmp_8d553_15;
                *row[20] = op0_id_col20;
                let memory_address_to_id_inputs_1 =
                    ((mem0_base_col12) + ((offset1_col4) - (M31_32768))).unpack();
                *lookup_data.memory_address_to_id_1 = [
                    ((mem0_base_col12) + ((offset1_col4) - (M31_32768))),
                    op0_id_col20,
                ];

                // Cond Decode Small Sign.

                let msb_tmp_8d553_17 = memory_id_to_big_value_tmp_8d553_16.get_m31(27).eq(M31_256);
                let msb_col21 = msb_tmp_8d553_17.as_m31();
                *row[21] = msb_col21;
                let mid_limbs_set_tmp_8d553_18 =
                    memory_id_to_big_value_tmp_8d553_16.get_m31(20).eq(M31_511);
                let mid_limbs_set_col22 = mid_limbs_set_tmp_8d553_18.as_m31();
                *row[22] = mid_limbs_set_col22;

                let op0_limb_0_col23 = memory_id_to_big_value_tmp_8d553_16.get_m31(0);
                *row[23] = op0_limb_0_col23;
                let op0_limb_1_col24 = memory_id_to_big_value_tmp_8d553_16.get_m31(1);
                *row[24] = op0_limb_1_col24;
                let op0_limb_2_col25 = memory_id_to_big_value_tmp_8d553_16.get_m31(2);
                *row[25] = op0_limb_2_col25;
                let memory_id_to_big_inputs_1 = op0_id_col20.unpack();
                *lookup_data.memory_id_to_big_1 = [
                    op0_id_col20,
                    op0_limb_0_col23,
                    op0_limb_1_col24,
                    op0_limb_2_col25,
                    ((mid_limbs_set_col22) * (M31_511)),
                    ((mid_limbs_set_col22) * (M31_511)),
                    ((mid_limbs_set_col22) * (M31_511)),
                    ((mid_limbs_set_col22) * (M31_511)),
                    ((mid_limbs_set_col22) * (M31_511)),
                    ((mid_limbs_set_col22) * (M31_511)),
                    ((mid_limbs_set_col22) * (M31_511)),
                    ((mid_limbs_set_col22) * (M31_511)),
                    ((mid_limbs_set_col22) * (M31_511)),
                    ((mid_limbs_set_col22) * (M31_511)),
                    ((mid_limbs_set_col22) * (M31_511)),
                    ((mid_limbs_set_col22) * (M31_511)),
                    ((mid_limbs_set_col22) * (M31_511)),
                    ((mid_limbs_set_col22) * (M31_511)),
                    ((mid_limbs_set_col22) * (M31_511)),
                    ((mid_limbs_set_col22) * (M31_511)),
                    ((mid_limbs_set_col22) * (M31_511)),
                    ((mid_limbs_set_col22) * (M31_511)),
                    (((M31_136) * (msb_col21)) - (mid_limbs_set_col22)),
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    ((msb_col21) * (M31_256)),
                ];

                // Read Small.

                let memory_address_to_id_value_tmp_8d553_19 = memory_address_to_id_state
                    .deduce_output(((mem1_base_col13) + ((offset2_col5) - (M31_32768))));
                let memory_id_to_big_value_tmp_8d553_20 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_8d553_19);
                let op1_id_col26 = memory_address_to_id_value_tmp_8d553_19;
                *row[26] = op1_id_col26;
                let memory_address_to_id_inputs_2 =
                    ((mem1_base_col13) + ((offset2_col5) - (M31_32768))).unpack();
                *lookup_data.memory_address_to_id_2 = [
                    ((mem1_base_col13) + ((offset2_col5) - (M31_32768))),
                    op1_id_col26,
                ];

                // Cond Decode Small Sign.

                let msb_tmp_8d553_21 = memory_id_to_big_value_tmp_8d553_20.get_m31(27).eq(M31_256);
                let msb_col27 = msb_tmp_8d553_21.as_m31();
                *row[27] = msb_col27;
                let mid_limbs_set_tmp_8d553_22 =
                    memory_id_to_big_value_tmp_8d553_20.get_m31(20).eq(M31_511);
                let mid_limbs_set_col28 = mid_limbs_set_tmp_8d553_22.as_m31();
                *row[28] = mid_limbs_set_col28;

                let op1_limb_0_col29 = memory_id_to_big_value_tmp_8d553_20.get_m31(0);
                *row[29] = op1_limb_0_col29;
                let op1_limb_1_col30 = memory_id_to_big_value_tmp_8d553_20.get_m31(1);
                *row[30] = op1_limb_1_col30;
                let op1_limb_2_col31 = memory_id_to_big_value_tmp_8d553_20.get_m31(2);
                *row[31] = op1_limb_2_col31;
                let memory_id_to_big_inputs_2 = op1_id_col26.unpack();
                *lookup_data.memory_id_to_big_2 = [
                    op1_id_col26,
                    op1_limb_0_col29,
                    op1_limb_1_col30,
                    op1_limb_2_col31,
                    ((mid_limbs_set_col28) * (M31_511)),
                    ((mid_limbs_set_col28) * (M31_511)),
                    ((mid_limbs_set_col28) * (M31_511)),
                    ((mid_limbs_set_col28) * (M31_511)),
                    ((mid_limbs_set_col28) * (M31_511)),
                    ((mid_limbs_set_col28) * (M31_511)),
                    ((mid_limbs_set_col28) * (M31_511)),
                    ((mid_limbs_set_col28) * (M31_511)),
                    ((mid_limbs_set_col28) * (M31_511)),
                    ((mid_limbs_set_col28) * (M31_511)),
                    ((mid_limbs_set_col28) * (M31_511)),
                    ((mid_limbs_set_col28) * (M31_511)),
                    ((mid_limbs_set_col28) * (M31_511)),
                    ((mid_limbs_set_col28) * (M31_511)),
                    ((mid_limbs_set_col28) * (M31_511)),
                    ((mid_limbs_set_col28) * (M31_511)),
                    ((mid_limbs_set_col28) * (M31_511)),
                    ((mid_limbs_set_col28) * (M31_511)),
                    (((M31_136) * (msb_col27)) - (mid_limbs_set_col28)),
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    ((msb_col27) * (M31_256)),
                ];

                *lookup_data.opcodes_0 = [input_pc_col0, input_ap_col1, input_fp_col2];
                *lookup_data.opcodes_1 = [
                    ((input_pc_col0) + (M31_1)),
                    ((input_ap_col1) + (ap_update_add_1_col10)),
                    input_fp_col2,
                ];
                *row[32] = padding.packed_at(row_index);

                // Add sub-components inputs.
                verify_instruction_state.add_inputs(&verify_instruction_inputs_0);
                memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_0);
                memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_0);
                memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_1);
                memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_1);
                memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_2);
                memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_2);
            },
        );

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
    verify_instruction_0: Vec<[PackedM31; 7]>,
}

pub struct InteractionClaimGenerator {
    n_rows: usize,
    log_size: u32,
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
        let padding_col = Enabler::new(self.n_rows);
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);

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
