#![allow(unused_parens)]
use std::sync::atomic::{AtomicU32, Ordering};

use itertools::{zip_eq, Itertools};
use stwo_cairo_adapter::decode::deconstruct_instruction;
use stwo_cairo_adapter::HashMap;

use crate::cairo_air::components::verify_instruction::{Claim, InteractionClaim};
use crate::witness::components::{
    memory_address_to_id, memory_id_to_big, range_check_4_3, range_check_7_2_5,
};
use crate::witness::prelude::*;
pub type InputType = (M31, [M31; 3], [M31; 2], M31);
pub type PackedInputType = (PackedM31, [PackedM31; 3], [PackedM31; 2], PackedM31);
const N_MULTIPLICITY_COLUMNS: usize = 1;
const N_TRACE_COLUMNS: usize = 16 + N_MULTIPLICITY_COLUMNS;

#[derive(Default)]
pub struct ClaimGenerator {
    /// pc -> encoded instruction.
    instructions: HashMap<M31, u128>,

    /// pc -> multiplicity.
    multiplicities: HashMap<M31, AtomicU32>,
}
impl ClaimGenerator {
    pub fn new(instructions: HashMap<M31, u128>) -> Self {
        let keys = instructions.keys().copied();
        let mut multiplicities = HashMap::with_capacity(keys.len());
        multiplicities.extend(keys.zip(std::iter::repeat_with(|| AtomicU32::new(0))));

        Self {
            multiplicities,
            instructions,
        }
    }

    pub fn write_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
        memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
        range_check_4_3_state: &range_check_4_3::ClaimGenerator,
        range_check_7_2_5_state: &range_check_7_2_5::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator) {
        // TODO(Ohad): use opcode_extension as it is used in stwo-air-cairo.
        let (mut inputs, mut mults) = self
            .multiplicities
            .into_iter()
            .sorted_by_key(|(pc, _)| *pc)
            .map(|(pc, multiplicity)| {
                let (offsets, flags, opcode_extension) =
                    deconstruct_instruction(*self.instructions.get(&pc).unwrap());
                let multiplicity = M31(multiplicity.into_inner());
                ((pc, offsets, flags, opcode_extension), multiplicity)
            })
            .unzip::<_, _, Vec<_>, Vec<_>>();
        let n_rows = inputs.len();
        assert_ne!(n_rows, 0);
        let size = std::cmp::max(n_rows.next_power_of_two(), N_LANES);
        let log_size = size.ilog2();
        let need_padding = n_rows != size;

        if need_padding {
            inputs.resize(size, *inputs.first().unwrap());
            mults.resize(size, M31::zero());
        }

        let packed_inputs = pack_values(&inputs);
        let packed_mults = pack_values(&mults);
        let (trace, lookup_data) = write_trace_simd(
            packed_inputs,
            packed_mults,
            memory_address_to_id_state,
            memory_id_to_big_state,
            range_check_4_3_state,
            range_check_7_2_5_state,
        );

        tree_builder.extend_evals(trace.to_evals());

        (
            Claim { log_size },
            InteractionClaimGenerator {
                log_size,
                lookup_data,
            },
        )
    }

    pub fn add_packed_inputs(&self, packed_inputs: &[PackedInputType]) {
        packed_inputs.into_par_iter().for_each(|packed_input| {
            packed_input.unpack().into_par_iter().for_each(|input| {
                self.add_input(&input);
            });
        });
    }

    // Instruction is determined by PC.
    pub fn add_input(&self, (pc, ..): &InputType) {
        self.multiplicities
            .get(pc)
            .unwrap()
            .fetch_add(1, Ordering::Relaxed);
    }
}

#[derive(ParIterMut, IterMut, Uninitialized)]
struct LookupData {
    memory_address_to_id_0: Vec<[PackedM31; 2]>,
    memory_id_to_big_0: Vec<[PackedM31; 29]>,
    range_check_4_3_0: Vec<[PackedM31; 2]>,
    range_check_7_2_5_0: Vec<[PackedM31; 3]>,
    verify_instruction_0: Vec<[PackedM31; 7]>,
    mults: Vec<PackedM31>,
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    inputs: Vec<PackedInputType>,
    mults: Vec<PackedM31>,
    memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
    memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
    range_check_4_3_state: &range_check_4_3::ClaimGenerator,
    range_check_7_2_5_state: &range_check_7_2_5::ClaimGenerator,
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
    let M31_16 = PackedM31::broadcast(M31::from(16));
    let M31_2 = PackedM31::broadcast(M31::from(2));
    let M31_256 = PackedM31::broadcast(M31::from(256));
    let M31_32 = PackedM31::broadcast(M31::from(32));
    let M31_4 = PackedM31::broadcast(M31::from(4));
    let M31_64 = PackedM31::broadcast(M31::from(64));
    let M31_8 = PackedM31::broadcast(M31::from(8));
    let UInt16_11 = PackedUInt16::broadcast(UInt16::from(11));
    let UInt16_13 = PackedUInt16::broadcast(UInt16::from(13));
    let UInt16_15 = PackedUInt16::broadcast(UInt16::from(15));
    let UInt16_2 = PackedUInt16::broadcast(UInt16::from(2));
    let UInt16_3 = PackedUInt16::broadcast(UInt16::from(3));
    let UInt16_4 = PackedUInt16::broadcast(UInt16::from(4));
    let UInt16_511 = PackedUInt16::broadcast(UInt16::from(511));
    let UInt16_9 = PackedUInt16::broadcast(UInt16::from(9));

    trace
        .par_iter_mut()
        .zip(inputs.into_par_iter())
        .zip(lookup_data.par_iter_mut())
        .zip(mults.into_par_iter())
        .for_each(
            |(((mut row, verify_instruction_input), lookup_data), multiplicity)| {
                let input_tmp_16a4f_0 = (
                    verify_instruction_input.0,
                    [
                        verify_instruction_input.1[0],
                        verify_instruction_input.1[1],
                        verify_instruction_input.1[2],
                    ],
                    [verify_instruction_input.2[0], verify_instruction_input.2[1]],
                    verify_instruction_input.3,
                );
                let input_limb_0_col0 = input_tmp_16a4f_0.0;
                *row[0] = input_limb_0_col0;
                let input_limb_1_col1 = input_tmp_16a4f_0.1[0];
                *row[1] = input_limb_1_col1;
                let input_limb_2_col2 = input_tmp_16a4f_0.1[1];
                *row[2] = input_limb_2_col2;
                let input_limb_3_col3 = input_tmp_16a4f_0.1[2];
                *row[3] = input_limb_3_col3;
                let input_limb_4_col4 = input_tmp_16a4f_0.2[0];
                *row[4] = input_limb_4_col4;
                let input_limb_5_col5 = input_tmp_16a4f_0.2[1];
                *row[5] = input_limb_5_col5;
                let input_limb_6_col6 = input_tmp_16a4f_0.3;
                *row[6] = input_limb_6_col6;

                // Encode Offsets.

                let offset0_low_tmp_16a4f_1 =
                    ((PackedUInt16::from_m31(input_limb_1_col1)) & (UInt16_511));
                let offset0_low_col7 = offset0_low_tmp_16a4f_1.as_m31();
                *row[7] = offset0_low_col7;
                let offset0_mid_tmp_16a4f_2 =
                    ((PackedUInt16::from_m31(input_limb_1_col1)) >> (UInt16_9));
                let offset0_mid_col8 = offset0_mid_tmp_16a4f_2.as_m31();
                *row[8] = offset0_mid_col8;
                let offset1_low_tmp_16a4f_3 =
                    ((PackedUInt16::from_m31(input_limb_2_col2)) & (UInt16_3));
                let offset1_low_col9 = offset1_low_tmp_16a4f_3.as_m31();
                *row[9] = offset1_low_col9;
                let offset1_mid_tmp_16a4f_4 =
                    (((PackedUInt16::from_m31(input_limb_2_col2)) >> (UInt16_2)) & (UInt16_511));
                let offset1_mid_col10 = offset1_mid_tmp_16a4f_4.as_m31();
                *row[10] = offset1_mid_col10;
                let offset1_high_tmp_16a4f_5 =
                    ((PackedUInt16::from_m31(input_limb_2_col2)) >> (UInt16_11));
                let offset1_high_col11 = offset1_high_tmp_16a4f_5.as_m31();
                *row[11] = offset1_high_col11;
                let offset2_low_tmp_16a4f_6 =
                    ((PackedUInt16::from_m31(input_limb_3_col3)) & (UInt16_15));
                let offset2_low_col12 = offset2_low_tmp_16a4f_6.as_m31();
                *row[12] = offset2_low_col12;
                let offset2_mid_tmp_16a4f_7 =
                    (((PackedUInt16::from_m31(input_limb_3_col3)) >> (UInt16_4)) & (UInt16_511));
                let offset2_mid_col13 = offset2_mid_tmp_16a4f_7.as_m31();
                *row[13] = offset2_mid_col13;
                let offset2_high_tmp_16a4f_8 =
                    ((PackedUInt16::from_m31(input_limb_3_col3)) >> (UInt16_13));
                let offset2_high_col14 = offset2_high_tmp_16a4f_8.as_m31();
                *row[14] = offset2_high_col14;
                let range_check_7_2_5_inputs_0 =
                    [offset0_mid_col8, offset1_low_col9, offset1_high_col11].unpack();
                *lookup_data.range_check_7_2_5_0 =
                    [offset0_mid_col8, offset1_low_col9, offset1_high_col11];
                let range_check_4_3_inputs_0 = [offset2_low_col12, offset2_high_col14].unpack();
                *lookup_data.range_check_4_3_0 = [offset2_low_col12, offset2_high_col14];

                // Mem Verify.

                let memory_address_to_id_value_tmp_16a4f_9 =
                    memory_address_to_id_state.deduce_output(input_limb_0_col0);
                let instruction_id_col15 = memory_address_to_id_value_tmp_16a4f_9;
                *row[15] = instruction_id_col15;
                let memory_address_to_id_inputs_0 = input_limb_0_col0.unpack();
                *lookup_data.memory_address_to_id_0 = [input_limb_0_col0, instruction_id_col15];
                let memory_id_to_big_inputs_0 = instruction_id_col15.unpack();
                *lookup_data.memory_id_to_big_0 = [
                    instruction_id_col15,
                    offset0_low_col7,
                    ((offset0_mid_col8) + ((offset1_low_col9) * (M31_128))),
                    offset1_mid_col10,
                    ((offset1_high_col11) + ((offset2_low_col12) * (M31_32))),
                    offset2_mid_col13,
                    ((offset2_high_col14) + (input_limb_4_col4)),
                    input_limb_5_col5,
                    input_limb_6_col6,
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
                    input_limb_0_col0,
                    input_limb_1_col1,
                    input_limb_2_col2,
                    input_limb_3_col3,
                    input_limb_4_col4,
                    input_limb_5_col5,
                    input_limb_6_col6,
                ];

                *row[16] = multiplicity;
                *lookup_data.mults = multiplicity;

                // Add sub-components inputs.
                range_check_7_2_5_state.add_inputs(&range_check_7_2_5_inputs_0);
                range_check_4_3_state.add_inputs(&range_check_4_3_inputs_0);
                memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_0);
                memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_0);
            },
        );

    (trace, lookup_data)
}

pub struct InteractionClaimGenerator {
    log_size: u32,
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        memory_address_to_id: &relations::MemoryAddressToId,
        memory_id_to_big: &relations::MemoryIdToBig,
        range_check_4_3: &relations::RangeCheck_4_3,
        range_check_7_2_5: &relations::RangeCheck_7_2_5,
        verify_instruction: &relations::VerifyInstruction,
    ) -> InteractionClaim {
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_7_2_5_0,
            &self.lookup_data.range_check_4_3_0,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_7_2_5.combine(values0);
            let denom1: PackedQM31 = range_check_4_3.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

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
        let lookup_row = &self.lookup_data.verify_instruction_0;
        let mults = &self.lookup_data.mults;
        for (i, (values, &mults)) in zip_eq(lookup_row, mults).enumerate() {
            let denom = verify_instruction.combine(values);
            col_gen.write_frac(i, (-mults).into(), denom);
        }
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();

        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}
