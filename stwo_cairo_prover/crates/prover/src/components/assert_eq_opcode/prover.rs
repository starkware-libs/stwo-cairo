#![allow(unused_parens)]
use super::component::{Claim, InteractionClaim};
use crate::components::prelude::proving::*;
use crate::components::{memory_address_to_id, memory_id_to_big, verify_instruction};

pub type InputType = CasmState;
pub type PackedInputType = PackedCasmState;
const N_TRACE_COLUMNS: usize = 13;

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
    let M31_32767 = PackedM31::broadcast(M31::from(32767));
    let M31_32768 = PackedM31::broadcast(M31::from(32768));
    let UInt16_0 = PackedUInt16::broadcast(UInt16::from(0));
    let UInt16_1 = PackedUInt16::broadcast(UInt16::from(1));
    let UInt16_11 = PackedUInt16::broadcast(UInt16::from(11));
    let UInt16_127 = PackedUInt16::broadcast(UInt16::from(127));
    let UInt16_13 = PackedUInt16::broadcast(UInt16::from(13));
    let UInt16_3 = PackedUInt16::broadcast(UInt16::from(3));
    let UInt16_4 = PackedUInt16::broadcast(UInt16::from(4));
    let UInt16_5 = PackedUInt16::broadcast(UInt16::from(5));
    let UInt16_6 = PackedUInt16::broadcast(UInt16::from(6));
    let UInt16_7 = PackedUInt16::broadcast(UInt16::from(7));
    let UInt16_9 = PackedUInt16::broadcast(UInt16::from(9));
    let padding_col = Enabler::new(n_rows);

    trace
        .par_iter_mut()
        .enumerate()
        .zip(inputs.into_par_iter())
        .zip(lookup_data.par_iter_mut())
        .for_each(
            |(((row_index, mut row), assert_eq_opcode_input), lookup_data)| {
                let input_tmp_d6f03_0 = assert_eq_opcode_input;
                let input_pc_col0 = input_tmp_d6f03_0.pc;
                *row[0] = input_pc_col0;
                let input_ap_col1 = input_tmp_d6f03_0.ap;
                *row[1] = input_ap_col1;
                let input_fp_col2 = input_tmp_d6f03_0.fp;
                *row[2] = input_fp_col2;

                // Decode Instruction.

                let memory_address_to_id_value_tmp_d6f03_1 =
                    memory_address_to_id_state.deduce_output(input_pc_col0);
                let memory_id_to_big_value_tmp_d6f03_2 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_d6f03_1);
                let offset0_tmp_d6f03_3 =
                    ((PackedUInt16::from_m31(memory_id_to_big_value_tmp_d6f03_2.get_m31(0)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_d6f03_2.get_m31(1),
                        )) & (UInt16_127))
                            << (UInt16_9)));
                let offset0_col3 = offset0_tmp_d6f03_3.as_m31();
                *row[3] = offset0_col3;
                let offset2_tmp_d6f03_4 =
                    ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_d6f03_2.get_m31(3)))
                        >> (UInt16_5))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_d6f03_2.get_m31(4),
                        )) << (UInt16_4)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_d6f03_2.get_m31(5),
                        )) & (UInt16_7))
                            << (UInt16_13)));
                let offset2_col4 = offset2_tmp_d6f03_4.as_m31();
                *row[4] = offset2_col4;
                let dst_base_fp_tmp_d6f03_5 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_d6f03_2.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_d6f03_2.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_0))
                        & (UInt16_1));
                let dst_base_fp_col5 = dst_base_fp_tmp_d6f03_5.as_m31();
                *row[5] = dst_base_fp_col5;
                let op1_base_fp_tmp_d6f03_6 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_d6f03_2.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_d6f03_2.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_3))
                        & (UInt16_1));
                let op1_base_fp_col6 = op1_base_fp_tmp_d6f03_6.as_m31();
                *row[6] = op1_base_fp_col6;
                let op1_base_ap_tmp_d6f03_7 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_d6f03_2.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_d6f03_2.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_4))
                        & (UInt16_1));
                let op1_base_ap_col7 = op1_base_ap_tmp_d6f03_7.as_m31();
                *row[7] = op1_base_ap_col7;
                let ap_update_add_1_tmp_d6f03_8 =
                    (((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_d6f03_2.get_m31(5)))
                        >> (UInt16_3))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_d6f03_2.get_m31(6),
                        )) << (UInt16_6)))
                        >> (UInt16_11))
                        & (UInt16_1));
                let ap_update_add_1_col8 = ap_update_add_1_tmp_d6f03_8.as_m31();
                *row[8] = ap_update_add_1_col8;
                let verify_instruction_inputs_0 = (
                    input_pc_col0,
                    [offset0_col3, M31_32767, offset2_col4],
                    [
                        dst_base_fp_col5,
                        M31_1,
                        M31_0,
                        op1_base_fp_col6,
                        op1_base_ap_col7,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        ap_update_add_1_col8,
                        M31_0,
                        M31_0,
                        M31_1,
                    ],
                )
                    .unpack();
                *lookup_data.verify_instruction_0 = [
                    input_pc_col0,
                    offset0_col3,
                    M31_32767,
                    offset2_col4,
                    dst_base_fp_col5,
                    M31_1,
                    M31_0,
                    op1_base_fp_col6,
                    op1_base_ap_col7,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    ap_update_add_1_col8,
                    M31_0,
                    M31_0,
                    M31_1,
                ];

                let mem_dst_base_col9 = (((dst_base_fp_col5) * (input_fp_col2))
                    + (((M31_1) - (dst_base_fp_col5)) * (input_ap_col1)));
                *row[9] = mem_dst_base_col9;
                let mem1_base_col10 = (((op1_base_fp_col6) * (input_fp_col2))
                    + ((op1_base_ap_col7) * (input_ap_col1)));
                *row[10] = mem1_base_col10;

                // Mem Verify Equal.

                let memory_address_to_id_value_tmp_d6f03_9 = memory_address_to_id_state
                    .deduce_output(((mem_dst_base_col9) + ((offset0_col3) - (M31_32768))));
                let dst_id_col11 = memory_address_to_id_value_tmp_d6f03_9;
                *row[11] = dst_id_col11;
                let memory_address_to_id_inputs_0 =
                    ((mem_dst_base_col9) + ((offset0_col3) - (M31_32768))).unpack();
                *lookup_data.memory_address_to_id_0 = [
                    ((mem_dst_base_col9) + ((offset0_col3) - (M31_32768))),
                    dst_id_col11,
                ];
                let memory_address_to_id_inputs_1 =
                    ((mem1_base_col10) + ((offset2_col4) - (M31_32768))).unpack();
                *lookup_data.memory_address_to_id_1 = [
                    ((mem1_base_col10) + ((offset2_col4) - (M31_32768))),
                    dst_id_col11,
                ];

                *lookup_data.opcodes_0 = [input_pc_col0, input_ap_col1, input_fp_col2];
                *lookup_data.opcodes_1 = [
                    ((input_pc_col0) + (M31_1)),
                    ((input_ap_col1) + (ap_update_add_1_col8)),
                    input_fp_col2,
                ];
                *row[12] = padding_col.packed_at(row_index);

                // Add sub-components inputs.
                verify_instruction_state.add_inputs(&verify_instruction_inputs_0);
                memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_0);
                memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_1);
            },
        );

    (trace, lookup_data)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    memory_address_to_id_0: Vec<[PackedM31; 2]>,
    memory_address_to_id_1: Vec<[PackedM31; 2]>,
    opcodes_0: Vec<[PackedM31; 3]>,
    opcodes_1: Vec<[PackedM31; 3]>,
    verify_instruction_0: Vec<[PackedM31; 19]>,
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
            &self.lookup_data.memory_address_to_id_1,
            &self.lookup_data.opcodes_0,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_address_to_id.combine(values0);
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
