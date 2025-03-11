#![allow(unused_parens)]
use super::component::{Claim, InteractionClaim};
use crate::components::prelude::proving::*;
use crate::components::{memory_address_to_id, memory_id_to_big, verify_instruction};

pub type InputType = CasmState;
pub type PackedInputType = PackedCasmState;
const N_TRACE_COLUMNS: usize = 11;

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
    let M31_134217728 = PackedM31::broadcast(M31::from(134217728));
    let M31_136 = PackedM31::broadcast(M31::from(136));
    let M31_152 = PackedM31::broadcast(M31::from(152));
    let M31_16 = PackedM31::broadcast(M31::from(16));
    let M31_256 = PackedM31::broadcast(M31::from(256));
    let M31_262144 = PackedM31::broadcast(M31::from(262144));
    let M31_32767 = PackedM31::broadcast(M31::from(32767));
    let M31_32768 = PackedM31::broadcast(M31::from(32768));
    let M31_511 = PackedM31::broadcast(M31::from(511));
    let M31_512 = PackedM31::broadcast(M31::from(512));
    let UInt16_13 = PackedUInt16::broadcast(UInt16::from(13));
    let UInt16_4 = PackedUInt16::broadcast(UInt16::from(4));
    let UInt16_5 = PackedUInt16::broadcast(UInt16::from(5));
    let UInt16_7 = PackedUInt16::broadcast(UInt16::from(7));
    let padding_col = Enabler::new(n_rows);

    trace
        .par_iter_mut()
        .enumerate()
        .zip(inputs.into_par_iter())
        .zip(lookup_data.par_iter_mut())
        .for_each(
            |(((row_index, mut row), add_ap_opcode_input), lookup_data)| {
                let input_tmp_c921e_0 = add_ap_opcode_input;
                let input_pc_col0 = input_tmp_c921e_0.pc;
                *row[0] = input_pc_col0;
                let input_ap_col1 = input_tmp_c921e_0.ap;
                *row[1] = input_ap_col1;
                let input_fp_col2 = input_tmp_c921e_0.fp;
                *row[2] = input_fp_col2;

                // Decode Instruction.

                let memory_address_to_id_value_tmp_c921e_1 =
                    memory_address_to_id_state.deduce_output(input_pc_col0);
                let memory_id_to_big_value_tmp_c921e_2 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_c921e_1);
                let offset2_tmp_c921e_3 =
                    ((((PackedUInt16::from_m31(memory_id_to_big_value_tmp_c921e_2.get_m31(3)))
                        >> (UInt16_5))
                        + ((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_c921e_2.get_m31(4),
                        )) << (UInt16_4)))
                        + (((PackedUInt16::from_m31(
                            memory_id_to_big_value_tmp_c921e_2.get_m31(5),
                        )) & (UInt16_7))
                            << (UInt16_13)));
                let offset2_col3 = offset2_tmp_c921e_3.as_m31();
                *row[3] = offset2_col3;
                let verify_instruction_inputs_0 = (
                    input_pc_col0,
                    [M31_32767, M31_32767, offset2_col3],
                    [M31_152, M31_16],
                    M31_0,
                )
                    .unpack();
                *lookup_data.verify_instruction_0 = [
                    input_pc_col0,
                    M31_32767,
                    M31_32767,
                    offset2_col3,
                    M31_152,
                    M31_16,
                    M31_0,
                ];

                // Read Small.

                let memory_address_to_id_value_tmp_c921e_4 = memory_address_to_id_state
                    .deduce_output(((input_ap_col1) + ((offset2_col3) - (M31_32768))));
                let memory_id_to_big_value_tmp_c921e_5 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_c921e_4);
                let op1_id_col4 = memory_address_to_id_value_tmp_c921e_4;
                *row[4] = op1_id_col4;
                let memory_address_to_id_inputs_0 =
                    ((input_ap_col1) + ((offset2_col3) - (M31_32768))).unpack();
                *lookup_data.memory_address_to_id_0 = [
                    ((input_ap_col1) + ((offset2_col3) - (M31_32768))),
                    op1_id_col4,
                ];

                // Cond Decode Small Sign.

                let msb_tmp_c921e_6 = memory_id_to_big_value_tmp_c921e_5.get_m31(27).eq(M31_256);
                let msb_col5 = msb_tmp_c921e_6.as_m31();
                *row[5] = msb_col5;
                let mid_limbs_set_tmp_c921e_7 =
                    memory_id_to_big_value_tmp_c921e_5.get_m31(20).eq(M31_511);
                let mid_limbs_set_col6 = mid_limbs_set_tmp_c921e_7.as_m31();
                *row[6] = mid_limbs_set_col6;

                let op1_limb_0_col7 = memory_id_to_big_value_tmp_c921e_5.get_m31(0);
                *row[7] = op1_limb_0_col7;
                let op1_limb_1_col8 = memory_id_to_big_value_tmp_c921e_5.get_m31(1);
                *row[8] = op1_limb_1_col8;
                let op1_limb_2_col9 = memory_id_to_big_value_tmp_c921e_5.get_m31(2);
                *row[9] = op1_limb_2_col9;
                let memory_id_to_big_inputs_0 = op1_id_col4.unpack();
                *lookup_data.memory_id_to_big_0 = [
                    op1_id_col4,
                    op1_limb_0_col7,
                    op1_limb_1_col8,
                    op1_limb_2_col9,
                    ((mid_limbs_set_col6) * (M31_511)),
                    ((mid_limbs_set_col6) * (M31_511)),
                    ((mid_limbs_set_col6) * (M31_511)),
                    ((mid_limbs_set_col6) * (M31_511)),
                    ((mid_limbs_set_col6) * (M31_511)),
                    ((mid_limbs_set_col6) * (M31_511)),
                    ((mid_limbs_set_col6) * (M31_511)),
                    ((mid_limbs_set_col6) * (M31_511)),
                    ((mid_limbs_set_col6) * (M31_511)),
                    ((mid_limbs_set_col6) * (M31_511)),
                    ((mid_limbs_set_col6) * (M31_511)),
                    ((mid_limbs_set_col6) * (M31_511)),
                    ((mid_limbs_set_col6) * (M31_511)),
                    ((mid_limbs_set_col6) * (M31_511)),
                    ((mid_limbs_set_col6) * (M31_511)),
                    ((mid_limbs_set_col6) * (M31_511)),
                    ((mid_limbs_set_col6) * (M31_511)),
                    ((mid_limbs_set_col6) * (M31_511)),
                    (((M31_136) * (msb_col5)) - (mid_limbs_set_col6)),
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    ((msb_col5) * (M31_256)),
                ];

                *lookup_data.opcodes_0 = [input_pc_col0, input_ap_col1, input_fp_col2];
                *lookup_data.opcodes_1 = [
                    ((input_pc_col0) + (M31_1)),
                    ((input_ap_col1)
                        + (((((op1_limb_0_col7) + ((op1_limb_1_col8) * (M31_512)))
                            + ((op1_limb_2_col9) * (M31_262144)))
                            - (msb_col5))
                            - ((M31_134217728) * (mid_limbs_set_col6)))),
                    input_fp_col2,
                ];
                *row[10] = padding_col.packed_at(row_index);

                // Add sub-components inputs.
                verify_instruction_state.add_inputs(&verify_instruction_inputs_0);
                memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_0);
                memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_0);
            },
        );

    (trace, lookup_data)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    memory_address_to_id_0: Vec<[PackedM31; 2]>,
    memory_id_to_big_0: Vec<[PackedM31; 29]>,
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
