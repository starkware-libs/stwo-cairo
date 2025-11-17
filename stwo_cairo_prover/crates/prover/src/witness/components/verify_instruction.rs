// This file was created by the AIR team.

#![allow(unused_parens)]
use cairo_air::components::verify_instruction::{Claim, InteractionClaim, N_TRACE_COLUMNS};

use crate::witness::components::{
    memory_address_to_id, memory_id_to_big, range_check_4_3, range_check_7_2_5,
};
use crate::witness::prelude::*;

pub type InputType = (M31, [M31; 3], [M31; 2], M31);
pub type PackedInputType = (PackedM31, [PackedM31; 3], [PackedM31; 2], PackedM31);

#[derive(Default)]
pub struct ClaimGenerator {
    pub mults: DashMap<InputType, AtomicU32>,
}

impl ClaimGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.mults.is_empty()
    }

    pub fn write_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        range_check_7_2_5_state: &range_check_7_2_5::ClaimGenerator,
        range_check_4_3_state: &range_check_4_3::ClaimGenerator,
        memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
        memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator) {
        let mut inputs_mults = self
            .mults
            .iter()
            .map(|entry| (*entry.key(), M31(entry.value().load(Ordering::Relaxed))))
            .collect::<Vec<_>>();

        inputs_mults.sort_by_key(|(input, _)| input.0);

        let (mut inputs, mut mults) = inputs_mults.into_iter().unzip::<_, _, Vec<_>, Vec<_>>();

        let n_rows = inputs.len();
        assert_ne!(n_rows, 0);
        let size = std::cmp::max(n_rows.next_power_of_two(), N_LANES);
        let log_size = size.ilog2();

        inputs.resize(size, *inputs.first().unwrap());
        mults.resize(size, M31::zero());

        let packed_inputs = pack_values(&inputs);
        let packed_mults = pack_values(&mults);

        let (trace, lookup_data, sub_component_inputs) = write_trace_simd(
            packed_inputs,
            packed_mults,
            range_check_7_2_5_state,
            range_check_4_3_state,
            memory_address_to_id_state,
            memory_id_to_big_state,
        );
        sub_component_inputs
            .range_check_7_2_5
            .iter()
            .for_each(|inputs| {
                range_check_7_2_5_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_4_3
            .iter()
            .for_each(|inputs| {
                range_check_4_3_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .memory_address_to_id
            .iter()
            .for_each(|inputs| {
                memory_address_to_id_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .memory_id_to_big
            .iter()
            .for_each(|inputs| {
                memory_id_to_big_state.add_packed_inputs(inputs);
            });
        tree_builder.extend_evals(trace.to_evals());

        (
            Claim { log_size },
            InteractionClaimGenerator {
                log_size,
                lookup_data,
            },
        )
    }

    pub fn add_input(&self, input: &InputType) {
        self.mults
            .entry(*input)
            .or_insert_with(|| AtomicU32::new(0))
            .fetch_add(1, Ordering::Relaxed);
    }

    pub fn add_packed_inputs(&self, packed_inputs: &[PackedInputType]) {
        packed_inputs.into_par_iter().for_each(|packed_input| {
            packed_input.unpack().into_par_iter().for_each(|input| {
                self.add_input(&input);
            });
        });
    }
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct SubComponentInputs {
    range_check_7_2_5: [Vec<range_check_7_2_5::PackedInputType>; 1],
    range_check_4_3: [Vec<range_check_4_3::PackedInputType>; 1],
    memory_address_to_id: [Vec<memory_address_to_id::PackedInputType>; 1],
    memory_id_to_big: [Vec<memory_id_to_big::PackedInputType>; 1],
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    inputs: Vec<PackedInputType>,
    mults: Vec<PackedM31>,
    range_check_7_2_5_state: &range_check_7_2_5::ClaimGenerator,
    range_check_4_3_state: &range_check_4_3::ClaimGenerator,
    memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
    memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
) -> (
    ComponentTrace<N_TRACE_COLUMNS>,
    LookupData,
    SubComponentInputs,
) {
    let log_n_packed_rows = inputs.len().ilog2();
    let log_size = log_n_packed_rows + LOG_N_LANES;
    let (mut trace, mut lookup_data, mut sub_component_inputs) = unsafe {
        (
            ComponentTrace::<N_TRACE_COLUMNS>::uninitialized(log_size),
            LookupData::uninitialized(log_n_packed_rows),
            SubComponentInputs::uninitialized(log_n_packed_rows),
        )
    };

    let M31_0 = PackedM31::broadcast(M31::from(0));
    let M31_128 = PackedM31::broadcast(M31::from(128));
    let M31_32 = PackedM31::broadcast(M31::from(32));
    let UInt16_11 = PackedUInt16::broadcast(UInt16::from(11));
    let UInt16_13 = PackedUInt16::broadcast(UInt16::from(13));
    let UInt16_15 = PackedUInt16::broadcast(UInt16::from(15));
    let UInt16_2 = PackedUInt16::broadcast(UInt16::from(2));
    let UInt16_3 = PackedUInt16::broadcast(UInt16::from(3));
    let UInt16_4 = PackedUInt16::broadcast(UInt16::from(4));
    let UInt16_511 = PackedUInt16::broadcast(UInt16::from(511));
    let UInt16_9 = PackedUInt16::broadcast(UInt16::from(9));

    (
        trace.par_iter_mut(),
        lookup_data.par_iter_mut(),
        sub_component_inputs.par_iter_mut(),
        inputs.into_par_iter(),
    )
        .into_par_iter()
        .enumerate()
        .for_each(
            |(row_index, (row, lookup_data, sub_component_inputs, verify_instruction_input))| {
                let input_pc_col0 = verify_instruction_input.0;
                *row[0] = input_pc_col0;
                let input_offset0_col1 = verify_instruction_input.1[0];
                *row[1] = input_offset0_col1;
                let input_offset1_col2 = verify_instruction_input.1[1];
                *row[2] = input_offset1_col2;
                let input_offset2_col3 = verify_instruction_input.1[2];
                *row[3] = input_offset2_col3;
                let input_inst_felt5_high_col4 = verify_instruction_input.2[0];
                *row[4] = input_inst_felt5_high_col4;
                let input_inst_felt6_col5 = verify_instruction_input.2[1];
                *row[5] = input_inst_felt6_col5;
                let input_opcode_extension_col6 = verify_instruction_input.3;
                *row[6] = input_opcode_extension_col6;

                // Encode Offsets.

                let offset0_low_tmp_16a4f_0 =
                    ((PackedUInt16::from_m31(input_offset0_col1)) & (UInt16_511));
                let offset0_low_col7 = offset0_low_tmp_16a4f_0.as_m31();
                *row[7] = offset0_low_col7;
                let offset0_mid_tmp_16a4f_1 =
                    ((PackedUInt16::from_m31(input_offset0_col1)) >> (UInt16_9));
                let offset0_mid_col8 = offset0_mid_tmp_16a4f_1.as_m31();
                *row[8] = offset0_mid_col8;
                let offset1_low_tmp_16a4f_2 =
                    ((PackedUInt16::from_m31(input_offset1_col2)) & (UInt16_3));
                let offset1_low_col9 = offset1_low_tmp_16a4f_2.as_m31();
                *row[9] = offset1_low_col9;
                let offset1_mid_tmp_16a4f_3 =
                    (((PackedUInt16::from_m31(input_offset1_col2)) >> (UInt16_2)) & (UInt16_511));
                let offset1_mid_col10 = offset1_mid_tmp_16a4f_3.as_m31();
                *row[10] = offset1_mid_col10;
                let offset1_high_tmp_16a4f_4 =
                    ((PackedUInt16::from_m31(input_offset1_col2)) >> (UInt16_11));
                let offset1_high_col11 = offset1_high_tmp_16a4f_4.as_m31();
                *row[11] = offset1_high_col11;
                let offset2_low_tmp_16a4f_5 =
                    ((PackedUInt16::from_m31(input_offset2_col3)) & (UInt16_15));
                let offset2_low_col12 = offset2_low_tmp_16a4f_5.as_m31();
                *row[12] = offset2_low_col12;
                let offset2_mid_tmp_16a4f_6 =
                    (((PackedUInt16::from_m31(input_offset2_col3)) >> (UInt16_4)) & (UInt16_511));
                let offset2_mid_col13 = offset2_mid_tmp_16a4f_6.as_m31();
                *row[13] = offset2_mid_col13;
                let offset2_high_tmp_16a4f_7 =
                    ((PackedUInt16::from_m31(input_offset2_col3)) >> (UInt16_13));
                let offset2_high_col14 = offset2_high_tmp_16a4f_7.as_m31();
                *row[14] = offset2_high_col14;
                *sub_component_inputs.range_check_7_2_5[0] =
                    [offset0_mid_col8, offset1_low_col9, offset1_high_col11];
                *lookup_data.range_check_7_2_5_0 =
                    [offset0_mid_col8, offset1_low_col9, offset1_high_col11];
                *sub_component_inputs.range_check_4_3[0] = [offset2_low_col12, offset2_high_col14];
                *lookup_data.range_check_4_3_0 = [offset2_low_col12, offset2_high_col14];
                let encode_offsets_output_tmp_16a4f_8 = [
                    offset0_low_col7,
                    ((offset0_mid_col8) + ((offset1_low_col9) * (M31_128))),
                    offset1_mid_col10,
                    ((offset1_high_col11) + ((offset2_low_col12) * (M31_32))),
                    offset2_mid_col13,
                    offset2_high_col14,
                ];

                // Mem Verify.

                // Read Id.

                let memory_address_to_id_value_tmp_16a4f_9 =
                    memory_address_to_id_state.deduce_output(input_pc_col0);
                let instruction_id_col15 = memory_address_to_id_value_tmp_16a4f_9;
                *row[15] = instruction_id_col15;
                *sub_component_inputs.memory_address_to_id[0] = input_pc_col0;
                *lookup_data.memory_address_to_id_0 = [input_pc_col0, instruction_id_col15];

                *sub_component_inputs.memory_id_to_big[0] = instruction_id_col15;
                *lookup_data.memory_id_to_big_0 = [
                    instruction_id_col15,
                    offset0_low_col7,
                    encode_offsets_output_tmp_16a4f_8[1],
                    offset1_mid_col10,
                    encode_offsets_output_tmp_16a4f_8[3],
                    offset2_mid_col13,
                    ((offset2_high_col14) + (input_inst_felt5_high_col4)),
                    input_inst_felt6_col5,
                    input_opcode_extension_col6,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                ];

                *lookup_data.verify_instruction_0 = [
                    input_pc_col0,
                    input_offset0_col1,
                    input_offset1_col2,
                    input_offset2_col3,
                    input_inst_felt5_high_col4,
                    input_inst_felt6_col5,
                    input_opcode_extension_col6,
                ];
                let mult_at_row = *mults.get(row_index).unwrap_or(&PackedM31::zero());
                *row[16] = mult_at_row;
                *lookup_data.mults = mult_at_row;
            },
        );

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    memory_address_to_id_0: Vec<[PackedM31; 2]>,
    memory_id_to_big_0: Vec<[PackedM31; 29]>,
    range_check_4_3_0: Vec<[PackedM31; 2]>,
    range_check_7_2_5_0: Vec<[PackedM31; 3]>,
    verify_instruction_0: Vec<[PackedM31; 7]>,
    mults: Vec<PackedM31>,
}

pub struct InteractionClaimGenerator {
    log_size: u32,
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        range_check_7_2_5: &relations::RangeCheck_7_2_5,
        range_check_4_3: &relations::RangeCheck_4_3,
        memory_address_to_id: &relations::MemoryAddressToId,
        memory_id_to_big: &relations::MemoryIdToBig,
        verify_instruction: &relations::VerifyInstruction,
    ) -> InteractionClaim {
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_7_2_5_0,
            &self.lookup_data.range_check_4_3_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_7_2_5.combine(values0);
                let denom1: PackedQM31 = range_check_4_3.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_address_to_id_0,
            &self.lookup_data.memory_id_to_big_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_address_to_id.combine(values0);
                let denom1: PackedQM31 = memory_id_to_big.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.verify_instruction_0,
            self.lookup_data.mults,
        )
            .into_par_iter()
            .for_each(|(writer, values, mults)| {
                let denom = verify_instruction.combine(values);
                writer.write_frac(-PackedQM31::one() * mults, denom);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}
