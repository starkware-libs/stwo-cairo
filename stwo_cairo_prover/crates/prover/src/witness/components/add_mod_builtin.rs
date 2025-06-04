#![allow(unused_parens)]
use cairo_air::components::add_mod_builtin::{Claim, InteractionClaim, N_TRACE_COLUMNS};

use crate::witness::components::{memory_address_to_id, memory_id_to_big};
use crate::witness::prelude::*;
use stwo_cairo_common::prover_types::cpu::Relocatable;

#[derive(Default)]
pub struct ClaimGenerator {
    pub log_size: u32,
    pub add_mod_builtin_segment_start: Relocatable,
}
impl ClaimGenerator {
    pub fn new(log_size: u32, add_mod_builtin_segment_start: Relocatable) -> Self {
        assert!(log_size >= LOG_N_LANES);
        Self {
            log_size,
            add_mod_builtin_segment_start,
        }
    }

    pub fn write_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
        memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator) {
        let log_size = self.log_size;

        let (trace, lookup_data, sub_component_inputs) = write_trace_simd(
            log_size,
            self.add_mod_builtin_segment_start.segment_index,
            memory_address_to_id_state,
            memory_id_to_big_state,
        );
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
            Claim {
                log_size,
                add_mod_builtin_segment_start: self.add_mod_builtin_segment_start.segment_index
                    as u32,
            },
            InteractionClaimGenerator {
                log_size,
                lookup_data,
            },
        )
    }
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct SubComponentInputs {
    memory_address_to_id: [Vec<PackedRelocatable>; 29],
    memory_id_to_big: [Vec<memory_id_to_big::PackedInputType>; 24],
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    log_size: u32,
    segment_id: usize,
    memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
    memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
) -> (
    ComponentTrace<N_TRACE_COLUMNS>,
    LookupData,
    SubComponentInputs,
) {
    let log_n_packed_rows = log_size - LOG_N_LANES;
    let (mut trace, mut lookup_data, mut sub_component_inputs) = unsafe {
        (
            ComponentTrace::<N_TRACE_COLUMNS>::uninitialized(log_size),
            LookupData::uninitialized(log_n_packed_rows),
            SubComponentInputs::uninitialized(log_n_packed_rows),
        )
    };

    let BigUInt_384_6_32_0_0_0_0_0_0 =
        PackedBigUInt::<384, 6, 32>::broadcast(BigUInt::<384, 6, 32>::from([0, 0, 0, 0, 0, 0]));
    let M31_0 = PackedM31::broadcast(M31::from(0));
    let M31_1 = PackedM31::broadcast(M31::from(1));
    let M31_128 = PackedM31::broadcast(M31::from(128));
    let M31_134217728 = PackedM31::broadcast(M31::from(134217728));
    let M31_136 = PackedM31::broadcast(M31::from(136));
    let M31_16 = PackedM31::broadcast(M31::from(16));
    let M31_2 = PackedM31::broadcast(M31::from(2));
    let M31_256 = PackedM31::broadcast(M31::from(256));
    let M31_262144 = PackedM31::broadcast(M31::from(262144));
    let M31_3 = PackedM31::broadcast(M31::from(3));
    let M31_32768 = PackedM31::broadcast(M31::from(32768));
    let M31_4 = PackedM31::broadcast(M31::from(4));
    let M31_5 = PackedM31::broadcast(M31::from(5));
    let M31_511 = PackedM31::broadcast(M31::from(511));
    let M31_512 = PackedM31::broadcast(M31::from(512));
    let M31_6 = PackedM31::broadcast(M31::from(6));
    let M31_64 = PackedM31::broadcast(M31::from(64));
    let M31_7 = PackedM31::broadcast(M31::from(7));
    let UInt32_9 = PackedUInt32::broadcast(UInt32::from(9));
    let UInt32_18 = PackedUInt32::broadcast(UInt32::from(18));
    let UInt32_27 = PackedUInt32::broadcast(UInt32::from(27));
    let UInt32_31 = PackedUInt32::broadcast(UInt32::from(31));
    let UInt32_5 = PackedUInt32::broadcast(UInt32::from(5));
    let UInt32_4 = PackedUInt32::broadcast(UInt32::from(4));
    let UInt32_13 = PackedUInt32::broadcast(UInt32::from(13));
    let UInt32_22 = PackedUInt32::broadcast(UInt32::from(22));
    let UInt32_31 = PackedUInt32::broadcast(UInt32::from(31));
    let UInt32_1 = PackedUInt32::broadcast(UInt32::from(1));
    let segment_id_packed = PackedM31::broadcast(M31::from(segment_id));
    let seq = Seq::new(log_size);

    (
        trace.par_iter_mut(),
        lookup_data.par_iter_mut(),
        sub_component_inputs.par_iter_mut(),
    )
        .into_par_iter()
        .enumerate()
        .for_each(
            |(row_index, (mut row, lookup_data, sub_component_inputs))| {
                let seq = seq.packed_at(row_index);

                // Mod Utils.

                let is_instance_0_tmp_c1b19_0 = seq.eq(M31_0);
                let is_instance_0_col0 = is_instance_0_tmp_c1b19_0.as_m31();
                *row[0] = is_instance_0_col0;
                let prev_instance_addr_tmp_c1b19_1 =
                    ((M31_7) * (((seq) - (M31_1)) + (is_instance_0_col0)));
                let instance_addr_tmp_c1b19_2 = ((M31_7) * (seq));

                // Read Positive Num Bits 99.

                let memory_address_to_id_value_tmp_c1b19_3 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable {
                        segment_index: segment_id_packed,
                        offset: instance_addr_tmp_c1b19_2,
                    });
                let memory_id_to_big_value_tmp_c1b19_4 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_c1b19_3);
                let p0_id_col1 = memory_address_to_id_value_tmp_c1b19_3;
                *row[1] = p0_id_col1;
                *sub_component_inputs.memory_address_to_id[0] = PackedRelocatable {
                    segment_index: segment_id_packed,
                    offset: instance_addr_tmp_c1b19_2,
                };
                *lookup_data.memory_address_to_id_0 =
                    [segment_id_packed, instance_addr_tmp_c1b19_2, p0_id_col1];
                let p0_limb_0_col2 = memory_id_to_big_value_tmp_c1b19_4.get_m31(0);
                *row[2] = p0_limb_0_col2;
                let p0_limb_1_col3 = memory_id_to_big_value_tmp_c1b19_4.get_m31(1);
                *row[3] = p0_limb_1_col3;
                let p0_limb_2_col4 = memory_id_to_big_value_tmp_c1b19_4.get_m31(2);
                *row[4] = p0_limb_2_col4;
                let p0_limb_3_col5 = memory_id_to_big_value_tmp_c1b19_4.get_m31(3);
                *row[5] = p0_limb_3_col5;
                let p0_limb_4_col6 = memory_id_to_big_value_tmp_c1b19_4.get_m31(4);
                *row[6] = p0_limb_4_col6;
                let p0_limb_5_col7 = memory_id_to_big_value_tmp_c1b19_4.get_m31(5);
                *row[7] = p0_limb_5_col7;
                let p0_limb_6_col8 = memory_id_to_big_value_tmp_c1b19_4.get_m31(6);
                *row[8] = p0_limb_6_col8;
                let p0_limb_7_col9 = memory_id_to_big_value_tmp_c1b19_4.get_m31(7);
                *row[9] = p0_limb_7_col9;
                let p0_limb_8_col10 = memory_id_to_big_value_tmp_c1b19_4.get_m31(8);
                *row[10] = p0_limb_8_col10;
                let p0_limb_9_col11 = memory_id_to_big_value_tmp_c1b19_4.get_m31(9);
                *row[11] = p0_limb_9_col11;
                let p0_limb_10_col12 = memory_id_to_big_value_tmp_c1b19_4.get_m31(10);
                *row[12] = p0_limb_10_col12;
                *sub_component_inputs.memory_id_to_big[0] = p0_id_col1;
                *lookup_data.memory_id_to_big_0 = [
                    p0_id_col1,
                    p0_limb_0_col2,
                    p0_limb_1_col3,
                    p0_limb_2_col4,
                    p0_limb_3_col5,
                    p0_limb_4_col6,
                    p0_limb_5_col7,
                    p0_limb_6_col8,
                    p0_limb_7_col9,
                    p0_limb_8_col10,
                    p0_limb_9_col11,
                    p0_limb_10_col12,
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
                let read_positive_num_bits_99_output_tmp_c1b19_5 = (
                    PackedFelt252::from_limbs([
                        p0_limb_0_col2,
                        p0_limb_1_col3,
                        p0_limb_2_col4,
                        p0_limb_3_col5,
                        p0_limb_4_col6,
                        p0_limb_5_col7,
                        p0_limb_6_col8,
                        p0_limb_7_col9,
                        p0_limb_8_col10,
                        p0_limb_9_col11,
                        p0_limb_10_col12,
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
                    ]),
                    p0_id_col1,
                );

                // Read Positive Num Bits 99.

                let memory_address_to_id_value_tmp_c1b19_6 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable {
                        segment_index: segment_id_packed,
                        offset: ((instance_addr_tmp_c1b19_2) + (M31_1)),
                    });
                let memory_id_to_big_value_tmp_c1b19_7 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_c1b19_6);
                let p1_id_col13 = memory_address_to_id_value_tmp_c1b19_6;
                *row[13] = p1_id_col13;
                *sub_component_inputs.memory_address_to_id[1] = PackedRelocatable {
                    segment_index: segment_id_packed,
                    offset: ((instance_addr_tmp_c1b19_2) + (M31_1)),
                };
                *lookup_data.memory_address_to_id_1 = [
                    segment_id_packed,
                    ((instance_addr_tmp_c1b19_2) + (M31_1)),
                    p1_id_col13,
                ];
                let p1_limb_0_col14 = memory_id_to_big_value_tmp_c1b19_7.get_m31(0);
                *row[14] = p1_limb_0_col14;
                let p1_limb_1_col15 = memory_id_to_big_value_tmp_c1b19_7.get_m31(1);
                *row[15] = p1_limb_1_col15;
                let p1_limb_2_col16 = memory_id_to_big_value_tmp_c1b19_7.get_m31(2);
                *row[16] = p1_limb_2_col16;
                let p1_limb_3_col17 = memory_id_to_big_value_tmp_c1b19_7.get_m31(3);
                *row[17] = p1_limb_3_col17;
                let p1_limb_4_col18 = memory_id_to_big_value_tmp_c1b19_7.get_m31(4);
                *row[18] = p1_limb_4_col18;
                let p1_limb_5_col19 = memory_id_to_big_value_tmp_c1b19_7.get_m31(5);
                *row[19] = p1_limb_5_col19;
                let p1_limb_6_col20 = memory_id_to_big_value_tmp_c1b19_7.get_m31(6);
                *row[20] = p1_limb_6_col20;
                let p1_limb_7_col21 = memory_id_to_big_value_tmp_c1b19_7.get_m31(7);
                *row[21] = p1_limb_7_col21;
                let p1_limb_8_col22 = memory_id_to_big_value_tmp_c1b19_7.get_m31(8);
                *row[22] = p1_limb_8_col22;
                let p1_limb_9_col23 = memory_id_to_big_value_tmp_c1b19_7.get_m31(9);
                *row[23] = p1_limb_9_col23;
                let p1_limb_10_col24 = memory_id_to_big_value_tmp_c1b19_7.get_m31(10);
                *row[24] = p1_limb_10_col24;
                *sub_component_inputs.memory_id_to_big[1] = p1_id_col13;
                *lookup_data.memory_id_to_big_1 = [
                    p1_id_col13,
                    p1_limb_0_col14,
                    p1_limb_1_col15,
                    p1_limb_2_col16,
                    p1_limb_3_col17,
                    p1_limb_4_col18,
                    p1_limb_5_col19,
                    p1_limb_6_col20,
                    p1_limb_7_col21,
                    p1_limb_8_col22,
                    p1_limb_9_col23,
                    p1_limb_10_col24,
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
                let read_positive_num_bits_99_output_tmp_c1b19_8 = (
                    PackedFelt252::from_limbs([
                        p1_limb_0_col14,
                        p1_limb_1_col15,
                        p1_limb_2_col16,
                        p1_limb_3_col17,
                        p1_limb_4_col18,
                        p1_limb_5_col19,
                        p1_limb_6_col20,
                        p1_limb_7_col21,
                        p1_limb_8_col22,
                        p1_limb_9_col23,
                        p1_limb_10_col24,
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
                    ]),
                    p1_id_col13,
                );

                // Read Positive Num Bits 99.

                let memory_address_to_id_value_tmp_c1b19_9 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable {
                        segment_index: segment_id_packed,
                        offset: ((instance_addr_tmp_c1b19_2) + (M31_2)),
                    });
                let memory_id_to_big_value_tmp_c1b19_10 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_c1b19_9);
                let p2_id_col25 = memory_address_to_id_value_tmp_c1b19_9;
                *row[25] = p2_id_col25;
                *sub_component_inputs.memory_address_to_id[2] = PackedRelocatable {
                    segment_index: segment_id_packed,
                    offset: ((instance_addr_tmp_c1b19_2) + (M31_2)),
                };
                *lookup_data.memory_address_to_id_2 = [
                    segment_id_packed,
                    ((instance_addr_tmp_c1b19_2) + (M31_2)),
                    p2_id_col25,
                ];
                let p2_limb_0_col26 = memory_id_to_big_value_tmp_c1b19_10.get_m31(0);
                *row[26] = p2_limb_0_col26;
                let p2_limb_1_col27 = memory_id_to_big_value_tmp_c1b19_10.get_m31(1);
                *row[27] = p2_limb_1_col27;
                let p2_limb_2_col28 = memory_id_to_big_value_tmp_c1b19_10.get_m31(2);
                *row[28] = p2_limb_2_col28;
                let p2_limb_3_col29 = memory_id_to_big_value_tmp_c1b19_10.get_m31(3);
                *row[29] = p2_limb_3_col29;
                let p2_limb_4_col30 = memory_id_to_big_value_tmp_c1b19_10.get_m31(4);
                *row[30] = p2_limb_4_col30;
                let p2_limb_5_col31 = memory_id_to_big_value_tmp_c1b19_10.get_m31(5);
                *row[31] = p2_limb_5_col31;
                let p2_limb_6_col32 = memory_id_to_big_value_tmp_c1b19_10.get_m31(6);
                *row[32] = p2_limb_6_col32;
                let p2_limb_7_col33 = memory_id_to_big_value_tmp_c1b19_10.get_m31(7);
                *row[33] = p2_limb_7_col33;
                let p2_limb_8_col34 = memory_id_to_big_value_tmp_c1b19_10.get_m31(8);
                *row[34] = p2_limb_8_col34;
                let p2_limb_9_col35 = memory_id_to_big_value_tmp_c1b19_10.get_m31(9);
                *row[35] = p2_limb_9_col35;
                let p2_limb_10_col36 = memory_id_to_big_value_tmp_c1b19_10.get_m31(10);
                *row[36] = p2_limb_10_col36;
                *sub_component_inputs.memory_id_to_big[2] = p2_id_col25;
                *lookup_data.memory_id_to_big_2 = [
                    p2_id_col25,
                    p2_limb_0_col26,
                    p2_limb_1_col27,
                    p2_limb_2_col28,
                    p2_limb_3_col29,
                    p2_limb_4_col30,
                    p2_limb_5_col31,
                    p2_limb_6_col32,
                    p2_limb_7_col33,
                    p2_limb_8_col34,
                    p2_limb_9_col35,
                    p2_limb_10_col36,
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
                let read_positive_num_bits_99_output_tmp_c1b19_11 = (
                    PackedFelt252::from_limbs([
                        p2_limb_0_col26,
                        p2_limb_1_col27,
                        p2_limb_2_col28,
                        p2_limb_3_col29,
                        p2_limb_4_col30,
                        p2_limb_5_col31,
                        p2_limb_6_col32,
                        p2_limb_7_col33,
                        p2_limb_8_col34,
                        p2_limb_9_col35,
                        p2_limb_10_col36,
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
                    ]),
                    p2_id_col25,
                );

                // Read Positive Num Bits 99.

                let memory_address_to_id_value_tmp_c1b19_12 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable {
                        segment_index: segment_id_packed,
                        offset: ((instance_addr_tmp_c1b19_2) + (M31_3)),
                    });
                let memory_id_to_big_value_tmp_c1b19_13 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_c1b19_12);
                let p3_id_col37 = memory_address_to_id_value_tmp_c1b19_12;
                *row[37] = p3_id_col37;
                *sub_component_inputs.memory_address_to_id[3] = PackedRelocatable {
                    segment_index: segment_id_packed,
                    offset: ((instance_addr_tmp_c1b19_2) + (M31_3)),
                };
                *lookup_data.memory_address_to_id_3 = [
                    segment_id_packed,
                    ((instance_addr_tmp_c1b19_2) + (M31_3)),
                    p3_id_col37,
                ];
                let p3_limb_0_col38 = memory_id_to_big_value_tmp_c1b19_13.get_m31(0);
                *row[38] = p3_limb_0_col38;
                let p3_limb_1_col39 = memory_id_to_big_value_tmp_c1b19_13.get_m31(1);
                *row[39] = p3_limb_1_col39;
                let p3_limb_2_col40 = memory_id_to_big_value_tmp_c1b19_13.get_m31(2);
                *row[40] = p3_limb_2_col40;
                let p3_limb_3_col41 = memory_id_to_big_value_tmp_c1b19_13.get_m31(3);
                *row[41] = p3_limb_3_col41;
                let p3_limb_4_col42 = memory_id_to_big_value_tmp_c1b19_13.get_m31(4);
                *row[42] = p3_limb_4_col42;
                let p3_limb_5_col43 = memory_id_to_big_value_tmp_c1b19_13.get_m31(5);
                *row[43] = p3_limb_5_col43;
                let p3_limb_6_col44 = memory_id_to_big_value_tmp_c1b19_13.get_m31(6);
                *row[44] = p3_limb_6_col44;
                let p3_limb_7_col45 = memory_id_to_big_value_tmp_c1b19_13.get_m31(7);
                *row[45] = p3_limb_7_col45;
                let p3_limb_8_col46 = memory_id_to_big_value_tmp_c1b19_13.get_m31(8);
                *row[46] = p3_limb_8_col46;
                let p3_limb_9_col47 = memory_id_to_big_value_tmp_c1b19_13.get_m31(9);
                *row[47] = p3_limb_9_col47;
                let p3_limb_10_col48 = memory_id_to_big_value_tmp_c1b19_13.get_m31(10);
                *row[48] = p3_limb_10_col48;
                *sub_component_inputs.memory_id_to_big[3] = p3_id_col37;
                *lookup_data.memory_id_to_big_3 = [
                    p3_id_col37,
                    p3_limb_0_col38,
                    p3_limb_1_col39,
                    p3_limb_2_col40,
                    p3_limb_3_col41,
                    p3_limb_4_col42,
                    p3_limb_5_col43,
                    p3_limb_6_col44,
                    p3_limb_7_col45,
                    p3_limb_8_col46,
                    p3_limb_9_col47,
                    p3_limb_10_col48,
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
                let read_positive_num_bits_99_output_tmp_c1b19_14 = (
                    PackedFelt252::from_limbs([
                        p3_limb_0_col38,
                        p3_limb_1_col39,
                        p3_limb_2_col40,
                        p3_limb_3_col41,
                        p3_limb_4_col42,
                        p3_limb_5_col43,
                        p3_limb_6_col44,
                        p3_limb_7_col45,
                        p3_limb_8_col46,
                        p3_limb_9_col47,
                        p3_limb_10_col48,
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
                    ]),
                    p3_id_col37,
                );

                // Read Positive Num Bits 27.

                let memory_address_to_id_value_tmp_c1b19_15 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable {
                        segment_index: segment_id_packed,
                        offset: ((instance_addr_tmp_c1b19_2) + (M31_4)),
                    });
                let memory_id_to_big_value_tmp_c1b19_16 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_c1b19_15);
                let values_ptr_id_col49 = memory_address_to_id_value_tmp_c1b19_15;
                *row[49] = values_ptr_id_col49;
                *sub_component_inputs.memory_address_to_id[4] = PackedRelocatable {
                    segment_index: segment_id_packed,
                    offset: ((instance_addr_tmp_c1b19_2) + (M31_4)),
                };
                *lookup_data.memory_address_to_id_4 = [
                    segment_id_packed,
                    ((instance_addr_tmp_c1b19_2) + (M31_4)),
                    values_ptr_id_col49,
                ];
                let values_ptr_limb_0_col50 = memory_id_to_big_value_tmp_c1b19_16.get_m31(0);
                *row[50] = values_ptr_limb_0_col50;
                let values_ptr_limb_1_col51 = memory_id_to_big_value_tmp_c1b19_16.get_m31(1);
                *row[51] = values_ptr_limb_1_col51;
                let values_ptr_limb_2_col52 = memory_id_to_big_value_tmp_c1b19_16.get_m31(2);
                *row[52] = values_ptr_limb_2_col52;
                let values_ptr_limb_3_col252 = memory_id_to_big_value_tmp_c1b19_16.get_m31(3);
                *row[252] = values_ptr_limb_3_col252;
                let values_ptr_limb_4_col253 = memory_id_to_big_value_tmp_c1b19_16.get_m31(4);
                *row[253] = values_ptr_limb_4_col253;
                let values_ptr_limb_5_col254 = memory_id_to_big_value_tmp_c1b19_16.get_m31(5);
                *row[254] = values_ptr_limb_5_col254;
                let values_ptr_limb_6_col255 = memory_id_to_big_value_tmp_c1b19_16.get_m31(6);
                *row[255] = values_ptr_limb_6_col255;
                let values_ptr_limb_7_col256 = memory_id_to_big_value_tmp_c1b19_16.get_m31(7);
                *row[256] = values_ptr_limb_7_col256;
                *sub_component_inputs.memory_id_to_big[4] = values_ptr_id_col49;
                *lookup_data.memory_id_to_big_4 = [
                    values_ptr_id_col49,
                    values_ptr_limb_0_col50,
                    values_ptr_limb_1_col51,
                    values_ptr_limb_2_col52,
                    values_ptr_limb_3_col252,
                    values_ptr_limb_4_col253,
                    values_ptr_limb_5_col254,
                    values_ptr_limb_6_col255,
                    values_ptr_limb_7_col256,
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
                let read_positive_num_bits_27_output_tmp_c1b19_17 = (
                    PackedFelt252::from_limbs([
                        values_ptr_limb_0_col50,
                        values_ptr_limb_1_col51,
                        values_ptr_limb_2_col52,
                        values_ptr_limb_3_col252,
                        values_ptr_limb_4_col253,
                        values_ptr_limb_5_col254,
                        values_ptr_limb_6_col255,
                        values_ptr_limb_7_col256,
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
                    ]),
                    values_ptr_id_col49,
                );

                // Read Positive Num Bits 27.

                let memory_address_to_id_value_tmp_c1b19_18 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable {
                        segment_index: segment_id_packed,
                        offset: ((instance_addr_tmp_c1b19_2) + (M31_5)),
                    });
                let memory_id_to_big_value_tmp_c1b19_19 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_c1b19_18);
                let offsets_ptr_id_col53 = memory_address_to_id_value_tmp_c1b19_18;
                *row[53] = offsets_ptr_id_col53;
                *sub_component_inputs.memory_address_to_id[5] = PackedRelocatable {
                    segment_index: segment_id_packed,
                    offset: ((instance_addr_tmp_c1b19_2) + (M31_5)),
                };
                *lookup_data.memory_address_to_id_5 = [
                    segment_id_packed,
                    ((instance_addr_tmp_c1b19_2) + (M31_5)),
                    offsets_ptr_id_col53,
                ];
                let offsets_ptr_limb_0_col54 = memory_id_to_big_value_tmp_c1b19_19.get_m31(0);
                *row[54] = offsets_ptr_limb_0_col54;
                let offsets_ptr_limb_1_col55 = memory_id_to_big_value_tmp_c1b19_19.get_m31(1);
                *row[55] = offsets_ptr_limb_1_col55;
                let offsets_ptr_limb_2_col56 = memory_id_to_big_value_tmp_c1b19_19.get_m31(2);
                *row[56] = offsets_ptr_limb_2_col56;
                let offsets_ptr_limb_3_col257 = memory_id_to_big_value_tmp_c1b19_19.get_m31(3);
                *row[257] = offsets_ptr_limb_3_col257;
                let offsets_ptr_limb_4_col258 = memory_id_to_big_value_tmp_c1b19_19.get_m31(4);
                *row[258] = offsets_ptr_limb_4_col258;
                let offsets_ptr_limb_5_col259 = memory_id_to_big_value_tmp_c1b19_19.get_m31(5);
                *row[259] = offsets_ptr_limb_5_col259;
                let offsets_ptr_limb_6_col260 = memory_id_to_big_value_tmp_c1b19_19.get_m31(6);
                *row[260] = offsets_ptr_limb_6_col260;
                let offsets_ptr_limb_7_col261 = memory_id_to_big_value_tmp_c1b19_19.get_m31(7);
                *row[261] = offsets_ptr_limb_7_col261;
                *sub_component_inputs.memory_id_to_big[5] = offsets_ptr_id_col53;
                *lookup_data.memory_id_to_big_5 = [
                    offsets_ptr_id_col53,
                    offsets_ptr_limb_0_col54,
                    offsets_ptr_limb_1_col55,
                    offsets_ptr_limb_2_col56,
                    offsets_ptr_limb_3_col257,
                    offsets_ptr_limb_4_col258,
                    offsets_ptr_limb_5_col259,
                    offsets_ptr_limb_6_col260,
                    offsets_ptr_limb_7_col261,
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
                let read_positive_num_bits_27_output_tmp_c1b19_20 = (
                    PackedFelt252::from_limbs([
                        offsets_ptr_limb_0_col54,
                        offsets_ptr_limb_1_col55,
                        offsets_ptr_limb_2_col56,
                        offsets_ptr_limb_3_col257,
                        offsets_ptr_limb_4_col258,
                        offsets_ptr_limb_5_col259,
                        offsets_ptr_limb_6_col260,
                        offsets_ptr_limb_7_col261,
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
                    ]),
                    offsets_ptr_id_col53,
                );

                // Read Positive Num Bits 27.

                let memory_address_to_id_value_tmp_c1b19_21 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable {
                        segment_index: segment_id_packed,
                        offset: ((prev_instance_addr_tmp_c1b19_1) + (M31_5)),
                    });
                let memory_id_to_big_value_tmp_c1b19_22 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_c1b19_21);
                let offsets_ptr_prev_id_col57 = memory_address_to_id_value_tmp_c1b19_21;
                *row[57] = offsets_ptr_prev_id_col57;
                *sub_component_inputs.memory_address_to_id[6] = PackedRelocatable {
                    segment_index: segment_id_packed,
                    offset: ((prev_instance_addr_tmp_c1b19_1) + (M31_5)),
                };
                *lookup_data.memory_address_to_id_6 = [
                    segment_id_packed,
                    ((prev_instance_addr_tmp_c1b19_1) + (M31_5)),
                    offsets_ptr_prev_id_col57,
                ];
                let offsets_ptr_prev_limb_0_col58 = memory_id_to_big_value_tmp_c1b19_22.get_m31(0);
                *row[58] = offsets_ptr_prev_limb_0_col58;
                let offsets_ptr_prev_limb_1_col59 = memory_id_to_big_value_tmp_c1b19_22.get_m31(1);
                *row[59] = offsets_ptr_prev_limb_1_col59;
                let offsets_ptr_prev_limb_2_col60 = memory_id_to_big_value_tmp_c1b19_22.get_m31(2);
                *row[60] = offsets_ptr_prev_limb_2_col60;
                let offsets_ptr_prev_limb_3_col262 = memory_id_to_big_value_tmp_c1b19_22.get_m31(3);
                *row[262] = offsets_ptr_prev_limb_3_col262;
                let offsets_ptr_prev_limb_4_col263 = memory_id_to_big_value_tmp_c1b19_22.get_m31(4);
                *row[263] = offsets_ptr_prev_limb_4_col263;
                let offsets_ptr_prev_limb_5_col264 = memory_id_to_big_value_tmp_c1b19_22.get_m31(5);
                *row[264] = offsets_ptr_prev_limb_5_col264;
                let offsets_ptr_prev_limb_6_col265 = memory_id_to_big_value_tmp_c1b19_22.get_m31(6);
                *row[265] = offsets_ptr_prev_limb_6_col265;
                let offsets_ptr_prev_limb_7_col266 = memory_id_to_big_value_tmp_c1b19_22.get_m31(7);
                *row[266] = offsets_ptr_prev_limb_7_col266;
                *sub_component_inputs.memory_id_to_big[6] = offsets_ptr_prev_id_col57;
                *lookup_data.memory_id_to_big_6 = [
                    offsets_ptr_prev_id_col57,
                    offsets_ptr_prev_limb_0_col58,
                    offsets_ptr_prev_limb_1_col59,
                    offsets_ptr_prev_limb_2_col60,
                    offsets_ptr_prev_limb_3_col262,
                    offsets_ptr_prev_limb_4_col263,
                    offsets_ptr_prev_limb_5_col264,
                    offsets_ptr_prev_limb_6_col265,
                    offsets_ptr_prev_limb_7_col266,
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
                let read_positive_num_bits_27_output_tmp_c1b19_23 = (
                    PackedFelt252::from_limbs([
                        offsets_ptr_prev_limb_0_col58,
                        offsets_ptr_prev_limb_1_col59,
                        offsets_ptr_prev_limb_2_col60,
                        offsets_ptr_prev_limb_3_col262,
                        offsets_ptr_prev_limb_4_col263,
                        offsets_ptr_prev_limb_5_col264,
                        offsets_ptr_prev_limb_6_col265,
                        offsets_ptr_prev_limb_7_col266,
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
                    ]),
                    offsets_ptr_prev_id_col57,
                );

                let prev_offsets_ptr_packed_offset =
                    (PackedUInt32::from_m31(offsets_ptr_prev_limb_0_col58)
                        | (PackedUInt32::from_m31(offsets_ptr_prev_limb_1_col59) << UInt32_9)
                        | (PackedUInt32::from_m31(offsets_ptr_prev_limb_2_col60) << UInt32_18)
                        | ((PackedUInt32::from_m31(offsets_ptr_prev_limb_3_col262) & UInt32_31)
                            << UInt32_27))
                        .as_m31();
                let prev_offsets_ptr_packed_segment_id =
                    ((PackedUInt32::from_m31(offsets_ptr_prev_limb_3_col262) >> UInt32_5)
                        | (PackedUInt32::from_m31(offsets_ptr_prev_limb_4_col263) << UInt32_4)
                        | (PackedUInt32::from_m31(offsets_ptr_prev_limb_5_col264) << UInt32_13)
                        | (PackedUInt32::from_m31(offsets_ptr_prev_limb_6_col265) << UInt32_22)
                        | ((PackedUInt32::from_m31(offsets_ptr_prev_limb_7_col266) & UInt32_1)
                            << UInt32_31))
                        .as_m31();

                let prev_offsets_ptr_offset_final_word_col273 =
                    (PackedUInt32::from_m31(offsets_ptr_prev_limb_3_col262) & UInt32_31).as_m31();
                *row[273] = prev_offsets_ptr_offset_final_word_col273;
                let prev_offsets_ptr_segment_id_initial_word_col274 =
                    (PackedUInt32::from_m31(offsets_ptr_prev_limb_3_col262) >> UInt32_5).as_m31();
                *row[274] = prev_offsets_ptr_segment_id_initial_word_col274;
                let prev_offsets_ptr_segment_id_final_word_col275 =
                    (PackedUInt32::from_m31(offsets_ptr_prev_limb_7_col266) & UInt32_1).as_m31();
                *row[275] = prev_offsets_ptr_segment_id_final_word_col275;

                // Read Positive Num Bits 27.

                let memory_address_to_id_value_tmp_c1b19_24 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable {
                        segment_index: segment_id_packed,
                        offset: ((instance_addr_tmp_c1b19_2) + (M31_6)),
                    });
                let memory_id_to_big_value_tmp_c1b19_25 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_c1b19_24);
                let n_id_col61 = memory_address_to_id_value_tmp_c1b19_24;
                *row[61] = n_id_col61;
                *sub_component_inputs.memory_address_to_id[7] = PackedRelocatable {
                    segment_index: segment_id_packed,
                    offset: ((instance_addr_tmp_c1b19_2) + (M31_6)),
                };
                *lookup_data.memory_address_to_id_7 = [
                    segment_id_packed,
                    ((instance_addr_tmp_c1b19_2) + (M31_6)),
                    n_id_col61,
                ];
                let n_limb_0_col62 = memory_id_to_big_value_tmp_c1b19_25.get_m31(0);
                *row[62] = n_limb_0_col62;
                let n_limb_1_col63 = memory_id_to_big_value_tmp_c1b19_25.get_m31(1);
                *row[63] = n_limb_1_col63;
                let n_limb_2_col64 = memory_id_to_big_value_tmp_c1b19_25.get_m31(2);
                *row[64] = n_limb_2_col64;
                *sub_component_inputs.memory_id_to_big[7] = n_id_col61;
                *lookup_data.memory_id_to_big_7 = [
                    n_id_col61,
                    n_limb_0_col62,
                    n_limb_1_col63,
                    n_limb_2_col64,
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
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                ];
                let read_positive_num_bits_27_output_tmp_c1b19_26 = (
                    PackedFelt252::from_limbs([
                        n_limb_0_col62,
                        n_limb_1_col63,
                        n_limb_2_col64,
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
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                    ]),
                    n_id_col61,
                );

                // Read Positive Num Bits 27.

                let memory_address_to_id_value_tmp_c1b19_27 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable {
                        segment_index: segment_id_packed,
                        offset: ((prev_instance_addr_tmp_c1b19_1) + (M31_6)),
                    });
                let memory_id_to_big_value_tmp_c1b19_28 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_c1b19_27);
                let n_prev_id_col65 = memory_address_to_id_value_tmp_c1b19_27;
                *row[65] = n_prev_id_col65;
                *sub_component_inputs.memory_address_to_id[8] = PackedRelocatable {
                    segment_index: segment_id_packed,
                    offset: ((prev_instance_addr_tmp_c1b19_1) + (M31_6)),
                };
                *lookup_data.memory_address_to_id_8 = [
                    segment_id_packed,
                    ((prev_instance_addr_tmp_c1b19_1) + (M31_6)),
                    n_prev_id_col65,
                ];
                let n_prev_limb_0_col66 = memory_id_to_big_value_tmp_c1b19_28.get_m31(0);
                *row[66] = n_prev_limb_0_col66;
                let n_prev_limb_1_col67 = memory_id_to_big_value_tmp_c1b19_28.get_m31(1);
                *row[67] = n_prev_limb_1_col67;
                let n_prev_limb_2_col68 = memory_id_to_big_value_tmp_c1b19_28.get_m31(2);
                *row[68] = n_prev_limb_2_col68;
                *sub_component_inputs.memory_id_to_big[8] = n_prev_id_col65;
                *lookup_data.memory_id_to_big_8 = [
                    n_prev_id_col65,
                    n_prev_limb_0_col66,
                    n_prev_limb_1_col67,
                    n_prev_limb_2_col68,
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
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                ];
                let read_positive_num_bits_27_output_tmp_c1b19_29 = (
                    PackedFelt252::from_limbs([
                        n_prev_limb_0_col66,
                        n_prev_limb_1_col67,
                        n_prev_limb_2_col68,
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
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                        M31_0,
                    ]),
                    n_prev_id_col65,
                );

                let block_reset_condition_tmp_c1b19_30 = (((((n_prev_limb_0_col66)
                    + ((n_prev_limb_1_col67) * (M31_512)))
                    + ((n_prev_limb_2_col68) * (M31_262144)))
                    - (M31_1))
                    * ((is_instance_0_col0) - (M31_1)));

                // Mem Cond Verify Equal Known Id.

                let memory_address_to_id_value_tmp_c1b19_31 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable {
                        segment_index: segment_id_packed,
                        offset: ((prev_instance_addr_tmp_c1b19_1) + (M31_4)),
                    });
                let values_ptr_prev_id_col69 = memory_address_to_id_value_tmp_c1b19_31;
                *row[69] = values_ptr_prev_id_col69;
                *sub_component_inputs.memory_address_to_id[9] = PackedRelocatable {
                    segment_index: segment_id_packed,
                    offset: ((prev_instance_addr_tmp_c1b19_1) + (M31_4)),
                };
                *lookup_data.memory_address_to_id_9 = [
                    segment_id_packed,
                    ((prev_instance_addr_tmp_c1b19_1) + (M31_4)),
                    values_ptr_prev_id_col69,
                ];

                // Mem Cond Verify Equal Known Id.

                let memory_address_to_id_value_tmp_c1b19_32 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable {
                        segment_index: segment_id_packed,
                        offset: prev_instance_addr_tmp_c1b19_1,
                    });
                let p_prev0_id_col70 = memory_address_to_id_value_tmp_c1b19_32;
                *row[70] = p_prev0_id_col70;
                *sub_component_inputs.memory_address_to_id[10] = PackedRelocatable {
                    segment_index: segment_id_packed,
                    offset: prev_instance_addr_tmp_c1b19_1,
                };
                *lookup_data.memory_address_to_id_10 = [
                    segment_id_packed,
                    prev_instance_addr_tmp_c1b19_1,
                    p_prev0_id_col70,
                ];

                // Mem Cond Verify Equal Known Id.

                let memory_address_to_id_value_tmp_c1b19_33 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable {
                        segment_index: segment_id_packed,
                        offset: ((prev_instance_addr_tmp_c1b19_1) + (M31_1)),
                    });
                let p_prev1_id_col71 = memory_address_to_id_value_tmp_c1b19_33;
                *row[71] = p_prev1_id_col71;
                *sub_component_inputs.memory_address_to_id[11] = PackedRelocatable {
                    segment_index: segment_id_packed,
                    offset: ((prev_instance_addr_tmp_c1b19_1) + (M31_1)),
                };
                *lookup_data.memory_address_to_id_11 = [
                    segment_id_packed,
                    ((prev_instance_addr_tmp_c1b19_1) + (M31_1)),
                    p_prev1_id_col71,
                ];

                // Mem Cond Verify Equal Known Id.

                let memory_address_to_id_value_tmp_c1b19_34 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable {
                        segment_index: segment_id_packed,
                        offset: ((prev_instance_addr_tmp_c1b19_1) + (M31_2)),
                    });
                let p_prev2_id_col72 = memory_address_to_id_value_tmp_c1b19_34;
                *row[72] = p_prev2_id_col72;
                *sub_component_inputs.memory_address_to_id[12] = PackedRelocatable {
                    segment_index: segment_id_packed,
                    offset: ((prev_instance_addr_tmp_c1b19_1) + (M31_2)),
                };
                *lookup_data.memory_address_to_id_12 = [
                    segment_id_packed,
                    ((prev_instance_addr_tmp_c1b19_1) + (M31_2)),
                    p_prev2_id_col72,
                ];

                // Mem Cond Verify Equal Known Id.

                let memory_address_to_id_value_tmp_c1b19_35 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable {
                        segment_index: segment_id_packed,
                        offset: ((prev_instance_addr_tmp_c1b19_1) + (M31_3)),
                    });
                let p_prev3_id_col73 = memory_address_to_id_value_tmp_c1b19_35;
                *row[73] = p_prev3_id_col73;
                *sub_component_inputs.memory_address_to_id[13] = PackedRelocatable {
                    segment_index: segment_id_packed,
                    offset: ((prev_instance_addr_tmp_c1b19_1) + (M31_3)),
                };
                *lookup_data.memory_address_to_id_13 = [
                    segment_id_packed,
                    ((prev_instance_addr_tmp_c1b19_1) + (M31_3)),
                    p_prev3_id_col73,
                ];

                // Read Small.

                let offsets_ptr_packed_offset = (PackedUInt32::from_m31(offsets_ptr_limb_0_col54)
                    | (PackedUInt32::from_m31(offsets_ptr_limb_1_col55) << UInt32_9)
                    | (PackedUInt32::from_m31(offsets_ptr_limb_2_col56) << UInt32_18)
                    | ((PackedUInt32::from_m31(offsets_ptr_limb_3_col257) & UInt32_31)
                        << UInt32_27))
                    .as_m31();
                let offsets_ptr_packed_segment_id =
                    ((PackedUInt32::from_m31(offsets_ptr_limb_3_col257) >> UInt32_5)
                        | (PackedUInt32::from_m31(offsets_ptr_limb_4_col258) << UInt32_4)
                        | (PackedUInt32::from_m31(offsets_ptr_limb_5_col259) << UInt32_13)
                        | (PackedUInt32::from_m31(offsets_ptr_limb_6_col260) << UInt32_22)
                        | ((PackedUInt32::from_m31(offsets_ptr_limb_7_col261) & UInt32_1)
                            << UInt32_31))
                        .as_m31();

                let offsets_ptr_offset_final_word_col267 =
                    (PackedUInt32::from_m31(offsets_ptr_limb_3_col257) & UInt32_31).as_m31();
                *row[267] = offsets_ptr_offset_final_word_col267;
                let offsets_ptr_segment_id_initial_word_col268 =
                    (PackedUInt32::from_m31(offsets_ptr_limb_3_col257) >> UInt32_5).as_m31();
                *row[268] = offsets_ptr_segment_id_initial_word_col268;
                let offsets_ptr_segment_id_final_word_col269 =
                    (PackedUInt32::from_m31(offsets_ptr_limb_7_col261) & UInt32_1).as_m31();
                *row[269] = offsets_ptr_segment_id_final_word_col269;

                let memory_address_to_id_value_tmp_c1b19_36 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable {
                        segment_index: offsets_ptr_packed_segment_id,
                        offset: offsets_ptr_packed_offset,
                    });
                let memory_id_to_big_value_tmp_c1b19_37 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_c1b19_36);
                let offsets_a_id_col74 = memory_address_to_id_value_tmp_c1b19_36;
                *row[74] = offsets_a_id_col74;
                *sub_component_inputs.memory_address_to_id[14] = PackedRelocatable {
                    segment_index: offsets_ptr_packed_segment_id,
                    offset: offsets_ptr_packed_offset,
                };
                *lookup_data.memory_address_to_id_14 = [
                    offsets_ptr_packed_segment_id,
                    offsets_ptr_packed_offset,
                    offsets_a_id_col74,
                ];

                // Cond Decode Small Sign.

                let msb_tmp_c1b19_38 = memory_id_to_big_value_tmp_c1b19_37.get_m31(27).eq(M31_256);
                let msb_col75 = msb_tmp_c1b19_38.as_m31();
                *row[75] = msb_col75;
                let mid_limbs_set_tmp_c1b19_39 =
                    memory_id_to_big_value_tmp_c1b19_37.get_m31(20).eq(M31_511);
                let mid_limbs_set_col76 = mid_limbs_set_tmp_c1b19_39.as_m31();
                *row[76] = mid_limbs_set_col76;
                let cond_decode_small_sign_output_tmp_c1b19_40 = [msb_col75, mid_limbs_set_col76];

                let offsets_a_limb_0_col77 = memory_id_to_big_value_tmp_c1b19_37.get_m31(0);
                *row[77] = offsets_a_limb_0_col77;
                let offsets_a_limb_1_col78 = memory_id_to_big_value_tmp_c1b19_37.get_m31(1);
                *row[78] = offsets_a_limb_1_col78;
                let offsets_a_limb_2_col79 = memory_id_to_big_value_tmp_c1b19_37.get_m31(2);
                *row[79] = offsets_a_limb_2_col79;
                *sub_component_inputs.memory_id_to_big[9] = offsets_a_id_col74;
                *lookup_data.memory_id_to_big_9 = [
                    offsets_a_id_col74,
                    offsets_a_limb_0_col77,
                    offsets_a_limb_1_col78,
                    offsets_a_limb_2_col79,
                    ((mid_limbs_set_col76) * (M31_511)),
                    ((mid_limbs_set_col76) * (M31_511)),
                    ((mid_limbs_set_col76) * (M31_511)),
                    ((mid_limbs_set_col76) * (M31_511)),
                    ((mid_limbs_set_col76) * (M31_511)),
                    ((mid_limbs_set_col76) * (M31_511)),
                    ((mid_limbs_set_col76) * (M31_511)),
                    ((mid_limbs_set_col76) * (M31_511)),
                    ((mid_limbs_set_col76) * (M31_511)),
                    ((mid_limbs_set_col76) * (M31_511)),
                    ((mid_limbs_set_col76) * (M31_511)),
                    ((mid_limbs_set_col76) * (M31_511)),
                    ((mid_limbs_set_col76) * (M31_511)),
                    ((mid_limbs_set_col76) * (M31_511)),
                    ((mid_limbs_set_col76) * (M31_511)),
                    ((mid_limbs_set_col76) * (M31_511)),
                    ((mid_limbs_set_col76) * (M31_511)),
                    ((mid_limbs_set_col76) * (M31_511)),
                    (((M31_136) * (msb_col75)) - (mid_limbs_set_col76)),
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    ((msb_col75) * (M31_256)),
                ];
                let read_small_output_tmp_c1b19_41 = (
                    (((((offsets_a_limb_0_col77) + ((offsets_a_limb_1_col78) * (M31_512)))
                        + ((offsets_a_limb_2_col79) * (M31_262144)))
                        - (msb_col75))
                        - ((M31_134217728) * (mid_limbs_set_col76))),
                    offsets_a_id_col74,
                );

                // Read Small.

                let memory_address_to_id_value_tmp_c1b19_42 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable {
                        segment_index: offsets_ptr_packed_segment_id,
                        offset: offsets_ptr_packed_offset + (M31_1),
                    });
                let memory_id_to_big_value_tmp_c1b19_43 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_c1b19_42);
                let offsets_b_id_col80 = memory_address_to_id_value_tmp_c1b19_42;
                *row[80] = offsets_b_id_col80;
                *sub_component_inputs.memory_address_to_id[15] = PackedRelocatable {
                    segment_index: offsets_ptr_packed_segment_id,
                    offset: offsets_ptr_packed_offset + (M31_1),
                };
                *lookup_data.memory_address_to_id_15 = [
                    offsets_ptr_packed_segment_id,
                    offsets_ptr_packed_offset + (M31_1),
                    offsets_b_id_col80,
                ];

                // Cond Decode Small Sign.

                let msb_tmp_c1b19_44 = memory_id_to_big_value_tmp_c1b19_43.get_m31(27).eq(M31_256);
                let msb_col81 = msb_tmp_c1b19_44.as_m31();
                *row[81] = msb_col81;
                let mid_limbs_set_tmp_c1b19_45 =
                    memory_id_to_big_value_tmp_c1b19_43.get_m31(20).eq(M31_511);
                let mid_limbs_set_col82 = mid_limbs_set_tmp_c1b19_45.as_m31();
                *row[82] = mid_limbs_set_col82;
                let cond_decode_small_sign_output_tmp_c1b19_46 = [msb_col81, mid_limbs_set_col82];

                let offsets_b_limb_0_col83 = memory_id_to_big_value_tmp_c1b19_43.get_m31(0);
                *row[83] = offsets_b_limb_0_col83;
                let offsets_b_limb_1_col84 = memory_id_to_big_value_tmp_c1b19_43.get_m31(1);
                *row[84] = offsets_b_limb_1_col84;
                let offsets_b_limb_2_col85 = memory_id_to_big_value_tmp_c1b19_43.get_m31(2);
                *row[85] = offsets_b_limb_2_col85;
                *sub_component_inputs.memory_id_to_big[10] = offsets_b_id_col80;
                *lookup_data.memory_id_to_big_10 = [
                    offsets_b_id_col80,
                    offsets_b_limb_0_col83,
                    offsets_b_limb_1_col84,
                    offsets_b_limb_2_col85,
                    ((mid_limbs_set_col82) * (M31_511)),
                    ((mid_limbs_set_col82) * (M31_511)),
                    ((mid_limbs_set_col82) * (M31_511)),
                    ((mid_limbs_set_col82) * (M31_511)),
                    ((mid_limbs_set_col82) * (M31_511)),
                    ((mid_limbs_set_col82) * (M31_511)),
                    ((mid_limbs_set_col82) * (M31_511)),
                    ((mid_limbs_set_col82) * (M31_511)),
                    ((mid_limbs_set_col82) * (M31_511)),
                    ((mid_limbs_set_col82) * (M31_511)),
                    ((mid_limbs_set_col82) * (M31_511)),
                    ((mid_limbs_set_col82) * (M31_511)),
                    ((mid_limbs_set_col82) * (M31_511)),
                    ((mid_limbs_set_col82) * (M31_511)),
                    ((mid_limbs_set_col82) * (M31_511)),
                    ((mid_limbs_set_col82) * (M31_511)),
                    ((mid_limbs_set_col82) * (M31_511)),
                    ((mid_limbs_set_col82) * (M31_511)),
                    (((M31_136) * (msb_col81)) - (mid_limbs_set_col82)),
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    ((msb_col81) * (M31_256)),
                ];
                let read_small_output_tmp_c1b19_47 = (
                    (((((offsets_b_limb_0_col83) + ((offsets_b_limb_1_col84) * (M31_512)))
                        + ((offsets_b_limb_2_col85) * (M31_262144)))
                        - (msb_col81))
                        - ((M31_134217728) * (mid_limbs_set_col82))),
                    offsets_b_id_col80,
                );

                // Read Small.

                let memory_address_to_id_value_tmp_c1b19_48 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable {
                        segment_index: offsets_ptr_packed_segment_id,
                        offset: offsets_ptr_packed_offset + (M31_2),
                    });
                let memory_id_to_big_value_tmp_c1b19_49 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_c1b19_48);
                let offsets_c_id_col86 = memory_address_to_id_value_tmp_c1b19_48;
                *row[86] = offsets_c_id_col86;
                *sub_component_inputs.memory_address_to_id[16] = PackedRelocatable {
                    segment_index: offsets_ptr_packed_segment_id,
                    offset: offsets_ptr_packed_offset + (M31_2),
                };
                *lookup_data.memory_address_to_id_16 = [
                    offsets_ptr_packed_segment_id,
                    offsets_ptr_packed_offset + (M31_2),
                    offsets_c_id_col86,
                ];

                // Cond Decode Small Sign.

                let msb_tmp_c1b19_50 = memory_id_to_big_value_tmp_c1b19_49.get_m31(27).eq(M31_256);
                let msb_col87 = msb_tmp_c1b19_50.as_m31();
                *row[87] = msb_col87;
                let mid_limbs_set_tmp_c1b19_51 =
                    memory_id_to_big_value_tmp_c1b19_49.get_m31(20).eq(M31_511);
                let mid_limbs_set_col88 = mid_limbs_set_tmp_c1b19_51.as_m31();
                *row[88] = mid_limbs_set_col88;
                let cond_decode_small_sign_output_tmp_c1b19_52 = [msb_col87, mid_limbs_set_col88];

                let offsets_c_limb_0_col89 = memory_id_to_big_value_tmp_c1b19_49.get_m31(0);
                *row[89] = offsets_c_limb_0_col89;
                let offsets_c_limb_1_col90 = memory_id_to_big_value_tmp_c1b19_49.get_m31(1);
                *row[90] = offsets_c_limb_1_col90;
                let offsets_c_limb_2_col91 = memory_id_to_big_value_tmp_c1b19_49.get_m31(2);
                *row[91] = offsets_c_limb_2_col91;
                *sub_component_inputs.memory_id_to_big[11] = offsets_c_id_col86;
                *lookup_data.memory_id_to_big_11 = [
                    offsets_c_id_col86,
                    offsets_c_limb_0_col89,
                    offsets_c_limb_1_col90,
                    offsets_c_limb_2_col91,
                    ((mid_limbs_set_col88) * (M31_511)),
                    ((mid_limbs_set_col88) * (M31_511)),
                    ((mid_limbs_set_col88) * (M31_511)),
                    ((mid_limbs_set_col88) * (M31_511)),
                    ((mid_limbs_set_col88) * (M31_511)),
                    ((mid_limbs_set_col88) * (M31_511)),
                    ((mid_limbs_set_col88) * (M31_511)),
                    ((mid_limbs_set_col88) * (M31_511)),
                    ((mid_limbs_set_col88) * (M31_511)),
                    ((mid_limbs_set_col88) * (M31_511)),
                    ((mid_limbs_set_col88) * (M31_511)),
                    ((mid_limbs_set_col88) * (M31_511)),
                    ((mid_limbs_set_col88) * (M31_511)),
                    ((mid_limbs_set_col88) * (M31_511)),
                    ((mid_limbs_set_col88) * (M31_511)),
                    ((mid_limbs_set_col88) * (M31_511)),
                    ((mid_limbs_set_col88) * (M31_511)),
                    ((mid_limbs_set_col88) * (M31_511)),
                    (((M31_136) * (msb_col87)) - (mid_limbs_set_col88)),
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    M31_0,
                    ((msb_col87) * (M31_256)),
                ];
                let read_small_output_tmp_c1b19_53 = (
                    (((((offsets_c_limb_0_col89) + ((offsets_c_limb_1_col90) * (M31_512)))
                        + ((offsets_c_limb_2_col91) * (M31_262144)))
                        - (msb_col87))
                        - ((M31_134217728) * (mid_limbs_set_col88))),
                    offsets_c_id_col86,
                );

                let values_ptr_packed_offset = (PackedUInt32::from_m31(values_ptr_limb_0_col50)
                    | (PackedUInt32::from_m31(values_ptr_limb_1_col51) << UInt32_9)
                    | (PackedUInt32::from_m31(values_ptr_limb_2_col52) << UInt32_18)
                    | ((PackedUInt32::from_m31(values_ptr_limb_3_col252) & UInt32_31)
                        << UInt32_27))
                    .as_m31();
                let values_ptr_packed_segment_id =
                    ((PackedUInt32::from_m31(values_ptr_limb_3_col252) >> UInt32_5)
                        | (PackedUInt32::from_m31(values_ptr_limb_4_col253) << UInt32_4)
                        | (PackedUInt32::from_m31(values_ptr_limb_5_col254) << UInt32_13)
                        | (PackedUInt32::from_m31(values_ptr_limb_6_col255) << UInt32_22)
                        | ((PackedUInt32::from_m31(values_ptr_limb_7_col256) & UInt32_1)
                            << UInt32_31))
                        .as_m31();

                let values_ptr_offset_final_word_col270 =
                    (PackedUInt32::from_m31(values_ptr_limb_3_col252) & UInt32_31).as_m31();
                *row[270] = values_ptr_offset_final_word_col270;
                let values_ptr_segment_id_initial_word_col271 =
                    (PackedUInt32::from_m31(values_ptr_limb_3_col252) >> UInt32_5).as_m31();
                *row[271] = values_ptr_segment_id_initial_word_col271;
                let values_ptr_segment_id_final_word_col272 =
                    (PackedUInt32::from_m31(values_ptr_limb_7_col256) & UInt32_1).as_m31();
                *row[272] = values_ptr_segment_id_final_word_col272;

                // Read Positive Num Bits 99.

                let memory_address_to_id_value_tmp_c1b19_55 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable {
                        segment_index: values_ptr_packed_segment_id,
                        offset: values_ptr_packed_offset + (read_small_output_tmp_c1b19_41.0),
                    });
                let memory_id_to_big_value_tmp_c1b19_56 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_c1b19_55);
                let a0_id_col92 = memory_address_to_id_value_tmp_c1b19_55;
                *row[92] = a0_id_col92;
                *sub_component_inputs.memory_address_to_id[17] = PackedRelocatable {
                    segment_index: values_ptr_packed_segment_id,
                    offset: values_ptr_packed_offset + (read_small_output_tmp_c1b19_41.0),
                };
                *lookup_data.memory_address_to_id_17 = [
                    values_ptr_packed_segment_id,
                    values_ptr_packed_offset + (read_small_output_tmp_c1b19_41.0),
                    a0_id_col92,
                ];
                let a0_limb_0_col93 = memory_id_to_big_value_tmp_c1b19_56.get_m31(0);
                *row[93] = a0_limb_0_col93;
                let a0_limb_1_col94 = memory_id_to_big_value_tmp_c1b19_56.get_m31(1);
                *row[94] = a0_limb_1_col94;
                let a0_limb_2_col95 = memory_id_to_big_value_tmp_c1b19_56.get_m31(2);
                *row[95] = a0_limb_2_col95;
                let a0_limb_3_col96 = memory_id_to_big_value_tmp_c1b19_56.get_m31(3);
                *row[96] = a0_limb_3_col96;
                let a0_limb_4_col97 = memory_id_to_big_value_tmp_c1b19_56.get_m31(4);
                *row[97] = a0_limb_4_col97;
                let a0_limb_5_col98 = memory_id_to_big_value_tmp_c1b19_56.get_m31(5);
                *row[98] = a0_limb_5_col98;
                let a0_limb_6_col99 = memory_id_to_big_value_tmp_c1b19_56.get_m31(6);
                *row[99] = a0_limb_6_col99;
                let a0_limb_7_col100 = memory_id_to_big_value_tmp_c1b19_56.get_m31(7);
                *row[100] = a0_limb_7_col100;
                let a0_limb_8_col101 = memory_id_to_big_value_tmp_c1b19_56.get_m31(8);
                *row[101] = a0_limb_8_col101;
                let a0_limb_9_col102 = memory_id_to_big_value_tmp_c1b19_56.get_m31(9);
                *row[102] = a0_limb_9_col102;
                let a0_limb_10_col103 = memory_id_to_big_value_tmp_c1b19_56.get_m31(10);
                *row[103] = a0_limb_10_col103;
                *sub_component_inputs.memory_id_to_big[12] = a0_id_col92;
                *lookup_data.memory_id_to_big_12 = [
                    a0_id_col92,
                    a0_limb_0_col93,
                    a0_limb_1_col94,
                    a0_limb_2_col95,
                    a0_limb_3_col96,
                    a0_limb_4_col97,
                    a0_limb_5_col98,
                    a0_limb_6_col99,
                    a0_limb_7_col100,
                    a0_limb_8_col101,
                    a0_limb_9_col102,
                    a0_limb_10_col103,
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
                let read_positive_num_bits_99_output_tmp_c1b19_57 = (
                    PackedFelt252::from_limbs([
                        a0_limb_0_col93,
                        a0_limb_1_col94,
                        a0_limb_2_col95,
                        a0_limb_3_col96,
                        a0_limb_4_col97,
                        a0_limb_5_col98,
                        a0_limb_6_col99,
                        a0_limb_7_col100,
                        a0_limb_8_col101,
                        a0_limb_9_col102,
                        a0_limb_10_col103,
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
                    ]),
                    a0_id_col92,
                );

                // Read Positive Num Bits 99.

                let memory_address_to_id_value_tmp_c1b19_58 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable {
                        segment_index: values_ptr_packed_segment_id,
                        offset: values_ptr_packed_offset
                            + (read_small_output_tmp_c1b19_41.0)
                            + (M31_1),
                    });
                let memory_id_to_big_value_tmp_c1b19_59 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_c1b19_58);
                let a1_id_col104 = memory_address_to_id_value_tmp_c1b19_58;
                *row[104] = a1_id_col104;
                *sub_component_inputs.memory_address_to_id[18] = PackedRelocatable {
                    segment_index: values_ptr_packed_segment_id,
                    offset: values_ptr_packed_offset + (read_small_output_tmp_c1b19_41.0) + (M31_1),
                };
                *lookup_data.memory_address_to_id_18 = [
                    values_ptr_packed_segment_id,
                    values_ptr_packed_offset + (read_small_output_tmp_c1b19_41.0) + (M31_1),
                    a1_id_col104,
                ];
                let a1_limb_0_col105 = memory_id_to_big_value_tmp_c1b19_59.get_m31(0);
                *row[105] = a1_limb_0_col105;
                let a1_limb_1_col106 = memory_id_to_big_value_tmp_c1b19_59.get_m31(1);
                *row[106] = a1_limb_1_col106;
                let a1_limb_2_col107 = memory_id_to_big_value_tmp_c1b19_59.get_m31(2);
                *row[107] = a1_limb_2_col107;
                let a1_limb_3_col108 = memory_id_to_big_value_tmp_c1b19_59.get_m31(3);
                *row[108] = a1_limb_3_col108;
                let a1_limb_4_col109 = memory_id_to_big_value_tmp_c1b19_59.get_m31(4);
                *row[109] = a1_limb_4_col109;
                let a1_limb_5_col110 = memory_id_to_big_value_tmp_c1b19_59.get_m31(5);
                *row[110] = a1_limb_5_col110;
                let a1_limb_6_col111 = memory_id_to_big_value_tmp_c1b19_59.get_m31(6);
                *row[111] = a1_limb_6_col111;
                let a1_limb_7_col112 = memory_id_to_big_value_tmp_c1b19_59.get_m31(7);
                *row[112] = a1_limb_7_col112;
                let a1_limb_8_col113 = memory_id_to_big_value_tmp_c1b19_59.get_m31(8);
                *row[113] = a1_limb_8_col113;
                let a1_limb_9_col114 = memory_id_to_big_value_tmp_c1b19_59.get_m31(9);
                *row[114] = a1_limb_9_col114;
                let a1_limb_10_col115 = memory_id_to_big_value_tmp_c1b19_59.get_m31(10);
                *row[115] = a1_limb_10_col115;
                *sub_component_inputs.memory_id_to_big[13] = a1_id_col104;
                *lookup_data.memory_id_to_big_13 = [
                    a1_id_col104,
                    a1_limb_0_col105,
                    a1_limb_1_col106,
                    a1_limb_2_col107,
                    a1_limb_3_col108,
                    a1_limb_4_col109,
                    a1_limb_5_col110,
                    a1_limb_6_col111,
                    a1_limb_7_col112,
                    a1_limb_8_col113,
                    a1_limb_9_col114,
                    a1_limb_10_col115,
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
                let read_positive_num_bits_99_output_tmp_c1b19_60 = (
                    PackedFelt252::from_limbs([
                        a1_limb_0_col105,
                        a1_limb_1_col106,
                        a1_limb_2_col107,
                        a1_limb_3_col108,
                        a1_limb_4_col109,
                        a1_limb_5_col110,
                        a1_limb_6_col111,
                        a1_limb_7_col112,
                        a1_limb_8_col113,
                        a1_limb_9_col114,
                        a1_limb_10_col115,
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
                    ]),
                    a1_id_col104,
                );

                // Read Positive Num Bits 99.

                let memory_address_to_id_value_tmp_c1b19_61 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable {
                        segment_index: values_ptr_packed_segment_id,
                        offset: values_ptr_packed_offset
                            + (read_small_output_tmp_c1b19_41.0)
                            + (M31_2),
                    });
                let memory_id_to_big_value_tmp_c1b19_62 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_c1b19_61);
                let a2_id_col116 = memory_address_to_id_value_tmp_c1b19_61;
                *row[116] = a2_id_col116;
                *sub_component_inputs.memory_address_to_id[19] = PackedRelocatable {
                    segment_index: values_ptr_packed_segment_id,
                    offset: values_ptr_packed_offset + (read_small_output_tmp_c1b19_41.0) + (M31_2),
                };
                *lookup_data.memory_address_to_id_19 = [
                    values_ptr_packed_segment_id,
                    values_ptr_packed_offset + (read_small_output_tmp_c1b19_41.0) + (M31_2),
                    a2_id_col116,
                ];
                let a2_limb_0_col117 = memory_id_to_big_value_tmp_c1b19_62.get_m31(0);
                *row[117] = a2_limb_0_col117;
                let a2_limb_1_col118 = memory_id_to_big_value_tmp_c1b19_62.get_m31(1);
                *row[118] = a2_limb_1_col118;
                let a2_limb_2_col119 = memory_id_to_big_value_tmp_c1b19_62.get_m31(2);
                *row[119] = a2_limb_2_col119;
                let a2_limb_3_col120 = memory_id_to_big_value_tmp_c1b19_62.get_m31(3);
                *row[120] = a2_limb_3_col120;
                let a2_limb_4_col121 = memory_id_to_big_value_tmp_c1b19_62.get_m31(4);
                *row[121] = a2_limb_4_col121;
                let a2_limb_5_col122 = memory_id_to_big_value_tmp_c1b19_62.get_m31(5);
                *row[122] = a2_limb_5_col122;
                let a2_limb_6_col123 = memory_id_to_big_value_tmp_c1b19_62.get_m31(6);
                *row[123] = a2_limb_6_col123;
                let a2_limb_7_col124 = memory_id_to_big_value_tmp_c1b19_62.get_m31(7);
                *row[124] = a2_limb_7_col124;
                let a2_limb_8_col125 = memory_id_to_big_value_tmp_c1b19_62.get_m31(8);
                *row[125] = a2_limb_8_col125;
                let a2_limb_9_col126 = memory_id_to_big_value_tmp_c1b19_62.get_m31(9);
                *row[126] = a2_limb_9_col126;
                let a2_limb_10_col127 = memory_id_to_big_value_tmp_c1b19_62.get_m31(10);
                *row[127] = a2_limb_10_col127;
                *sub_component_inputs.memory_id_to_big[14] = a2_id_col116;
                *lookup_data.memory_id_to_big_14 = [
                    a2_id_col116,
                    a2_limb_0_col117,
                    a2_limb_1_col118,
                    a2_limb_2_col119,
                    a2_limb_3_col120,
                    a2_limb_4_col121,
                    a2_limb_5_col122,
                    a2_limb_6_col123,
                    a2_limb_7_col124,
                    a2_limb_8_col125,
                    a2_limb_9_col126,
                    a2_limb_10_col127,
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
                let read_positive_num_bits_99_output_tmp_c1b19_63 = (
                    PackedFelt252::from_limbs([
                        a2_limb_0_col117,
                        a2_limb_1_col118,
                        a2_limb_2_col119,
                        a2_limb_3_col120,
                        a2_limb_4_col121,
                        a2_limb_5_col122,
                        a2_limb_6_col123,
                        a2_limb_7_col124,
                        a2_limb_8_col125,
                        a2_limb_9_col126,
                        a2_limb_10_col127,
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
                    ]),
                    a2_id_col116,
                );

                // Read Positive Num Bits 99.

                let memory_address_to_id_value_tmp_c1b19_64 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable {
                        segment_index: values_ptr_packed_segment_id,
                        offset: values_ptr_packed_offset
                            + (read_small_output_tmp_c1b19_41.0)
                            + (M31_3),
                    });
                let memory_id_to_big_value_tmp_c1b19_65 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_c1b19_64);
                let a3_id_col128 = memory_address_to_id_value_tmp_c1b19_64;
                *row[128] = a3_id_col128;
                *sub_component_inputs.memory_address_to_id[20] = PackedRelocatable {
                    segment_index: values_ptr_packed_segment_id,
                    offset: values_ptr_packed_offset + (read_small_output_tmp_c1b19_41.0) + (M31_3),
                };
                *lookup_data.memory_address_to_id_20 = [
                    values_ptr_packed_segment_id,
                    values_ptr_packed_offset + (read_small_output_tmp_c1b19_41.0) + (M31_3),
                    a3_id_col128,
                ];
                let a3_limb_0_col129 = memory_id_to_big_value_tmp_c1b19_65.get_m31(0);
                *row[129] = a3_limb_0_col129;
                let a3_limb_1_col130 = memory_id_to_big_value_tmp_c1b19_65.get_m31(1);
                *row[130] = a3_limb_1_col130;
                let a3_limb_2_col131 = memory_id_to_big_value_tmp_c1b19_65.get_m31(2);
                *row[131] = a3_limb_2_col131;
                let a3_limb_3_col132 = memory_id_to_big_value_tmp_c1b19_65.get_m31(3);
                *row[132] = a3_limb_3_col132;
                let a3_limb_4_col133 = memory_id_to_big_value_tmp_c1b19_65.get_m31(4);
                *row[133] = a3_limb_4_col133;
                let a3_limb_5_col134 = memory_id_to_big_value_tmp_c1b19_65.get_m31(5);
                *row[134] = a3_limb_5_col134;
                let a3_limb_6_col135 = memory_id_to_big_value_tmp_c1b19_65.get_m31(6);
                *row[135] = a3_limb_6_col135;
                let a3_limb_7_col136 = memory_id_to_big_value_tmp_c1b19_65.get_m31(7);
                *row[136] = a3_limb_7_col136;
                let a3_limb_8_col137 = memory_id_to_big_value_tmp_c1b19_65.get_m31(8);
                *row[137] = a3_limb_8_col137;
                let a3_limb_9_col138 = memory_id_to_big_value_tmp_c1b19_65.get_m31(9);
                *row[138] = a3_limb_9_col138;
                let a3_limb_10_col139 = memory_id_to_big_value_tmp_c1b19_65.get_m31(10);
                *row[139] = a3_limb_10_col139;
                *sub_component_inputs.memory_id_to_big[15] = a3_id_col128;
                *lookup_data.memory_id_to_big_15 = [
                    a3_id_col128,
                    a3_limb_0_col129,
                    a3_limb_1_col130,
                    a3_limb_2_col131,
                    a3_limb_3_col132,
                    a3_limb_4_col133,
                    a3_limb_5_col134,
                    a3_limb_6_col135,
                    a3_limb_7_col136,
                    a3_limb_8_col137,
                    a3_limb_9_col138,
                    a3_limb_10_col139,
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
                let read_positive_num_bits_99_output_tmp_c1b19_66 = (
                    PackedFelt252::from_limbs([
                        a3_limb_0_col129,
                        a3_limb_1_col130,
                        a3_limb_2_col131,
                        a3_limb_3_col132,
                        a3_limb_4_col133,
                        a3_limb_5_col134,
                        a3_limb_6_col135,
                        a3_limb_7_col136,
                        a3_limb_8_col137,
                        a3_limb_9_col138,
                        a3_limb_10_col139,
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
                    ]),
                    a3_id_col128,
                );

                // Read Positive Num Bits 99.

                let memory_address_to_id_value_tmp_c1b19_67 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable {
                        segment_index: values_ptr_packed_segment_id,
                        offset: values_ptr_packed_offset + (read_small_output_tmp_c1b19_47.0),
                    });
                let memory_id_to_big_value_tmp_c1b19_68 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_c1b19_67);
                let b0_id_col140 = memory_address_to_id_value_tmp_c1b19_67;
                *row[140] = b0_id_col140;
                *sub_component_inputs.memory_address_to_id[21] = PackedRelocatable {
                    segment_index: values_ptr_packed_segment_id,
                    offset: values_ptr_packed_offset + (read_small_output_tmp_c1b19_47.0),
                };
                *lookup_data.memory_address_to_id_21 = [
                    values_ptr_packed_segment_id,
                    values_ptr_packed_offset + (read_small_output_tmp_c1b19_47.0),
                    b0_id_col140,
                ];
                let b0_limb_0_col141 = memory_id_to_big_value_tmp_c1b19_68.get_m31(0);
                *row[141] = b0_limb_0_col141;
                let b0_limb_1_col142 = memory_id_to_big_value_tmp_c1b19_68.get_m31(1);
                *row[142] = b0_limb_1_col142;
                let b0_limb_2_col143 = memory_id_to_big_value_tmp_c1b19_68.get_m31(2);
                *row[143] = b0_limb_2_col143;
                let b0_limb_3_col144 = memory_id_to_big_value_tmp_c1b19_68.get_m31(3);
                *row[144] = b0_limb_3_col144;
                let b0_limb_4_col145 = memory_id_to_big_value_tmp_c1b19_68.get_m31(4);
                *row[145] = b0_limb_4_col145;
                let b0_limb_5_col146 = memory_id_to_big_value_tmp_c1b19_68.get_m31(5);
                *row[146] = b0_limb_5_col146;
                let b0_limb_6_col147 = memory_id_to_big_value_tmp_c1b19_68.get_m31(6);
                *row[147] = b0_limb_6_col147;
                let b0_limb_7_col148 = memory_id_to_big_value_tmp_c1b19_68.get_m31(7);
                *row[148] = b0_limb_7_col148;
                let b0_limb_8_col149 = memory_id_to_big_value_tmp_c1b19_68.get_m31(8);
                *row[149] = b0_limb_8_col149;
                let b0_limb_9_col150 = memory_id_to_big_value_tmp_c1b19_68.get_m31(9);
                *row[150] = b0_limb_9_col150;
                let b0_limb_10_col151 = memory_id_to_big_value_tmp_c1b19_68.get_m31(10);
                *row[151] = b0_limb_10_col151;
                *sub_component_inputs.memory_id_to_big[16] = b0_id_col140;
                *lookup_data.memory_id_to_big_16 = [
                    b0_id_col140,
                    b0_limb_0_col141,
                    b0_limb_1_col142,
                    b0_limb_2_col143,
                    b0_limb_3_col144,
                    b0_limb_4_col145,
                    b0_limb_5_col146,
                    b0_limb_6_col147,
                    b0_limb_7_col148,
                    b0_limb_8_col149,
                    b0_limb_9_col150,
                    b0_limb_10_col151,
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
                let read_positive_num_bits_99_output_tmp_c1b19_69 = (
                    PackedFelt252::from_limbs([
                        b0_limb_0_col141,
                        b0_limb_1_col142,
                        b0_limb_2_col143,
                        b0_limb_3_col144,
                        b0_limb_4_col145,
                        b0_limb_5_col146,
                        b0_limb_6_col147,
                        b0_limb_7_col148,
                        b0_limb_8_col149,
                        b0_limb_9_col150,
                        b0_limb_10_col151,
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
                    ]),
                    b0_id_col140,
                );

                // Read Positive Num Bits 99.

                let memory_address_to_id_value_tmp_c1b19_70 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable {
                        segment_index: values_ptr_packed_segment_id,
                        offset: values_ptr_packed_offset
                            + (read_small_output_tmp_c1b19_47.0)
                            + (M31_1),
                    });
                let memory_id_to_big_value_tmp_c1b19_71 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_c1b19_70);
                let b1_id_col152 = memory_address_to_id_value_tmp_c1b19_70;
                *row[152] = b1_id_col152;
                *sub_component_inputs.memory_address_to_id[22] = PackedRelocatable {
                    segment_index: values_ptr_packed_segment_id,
                    offset: values_ptr_packed_offset + (read_small_output_tmp_c1b19_47.0) + (M31_1),
                };
                *lookup_data.memory_address_to_id_22 = [
                    values_ptr_packed_segment_id,
                    values_ptr_packed_offset + (read_small_output_tmp_c1b19_47.0) + (M31_1),
                    b1_id_col152,
                ];
                let b1_limb_0_col153 = memory_id_to_big_value_tmp_c1b19_71.get_m31(0);
                *row[153] = b1_limb_0_col153;
                let b1_limb_1_col154 = memory_id_to_big_value_tmp_c1b19_71.get_m31(1);
                *row[154] = b1_limb_1_col154;
                let b1_limb_2_col155 = memory_id_to_big_value_tmp_c1b19_71.get_m31(2);
                *row[155] = b1_limb_2_col155;
                let b1_limb_3_col156 = memory_id_to_big_value_tmp_c1b19_71.get_m31(3);
                *row[156] = b1_limb_3_col156;
                let b1_limb_4_col157 = memory_id_to_big_value_tmp_c1b19_71.get_m31(4);
                *row[157] = b1_limb_4_col157;
                let b1_limb_5_col158 = memory_id_to_big_value_tmp_c1b19_71.get_m31(5);
                *row[158] = b1_limb_5_col158;
                let b1_limb_6_col159 = memory_id_to_big_value_tmp_c1b19_71.get_m31(6);
                *row[159] = b1_limb_6_col159;
                let b1_limb_7_col160 = memory_id_to_big_value_tmp_c1b19_71.get_m31(7);
                *row[160] = b1_limb_7_col160;
                let b1_limb_8_col161 = memory_id_to_big_value_tmp_c1b19_71.get_m31(8);
                *row[161] = b1_limb_8_col161;
                let b1_limb_9_col162 = memory_id_to_big_value_tmp_c1b19_71.get_m31(9);
                *row[162] = b1_limb_9_col162;
                let b1_limb_10_col163 = memory_id_to_big_value_tmp_c1b19_71.get_m31(10);
                *row[163] = b1_limb_10_col163;
                *sub_component_inputs.memory_id_to_big[17] = b1_id_col152;
                *lookup_data.memory_id_to_big_17 = [
                    b1_id_col152,
                    b1_limb_0_col153,
                    b1_limb_1_col154,
                    b1_limb_2_col155,
                    b1_limb_3_col156,
                    b1_limb_4_col157,
                    b1_limb_5_col158,
                    b1_limb_6_col159,
                    b1_limb_7_col160,
                    b1_limb_8_col161,
                    b1_limb_9_col162,
                    b1_limb_10_col163,
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
                let read_positive_num_bits_99_output_tmp_c1b19_72 = (
                    PackedFelt252::from_limbs([
                        b1_limb_0_col153,
                        b1_limb_1_col154,
                        b1_limb_2_col155,
                        b1_limb_3_col156,
                        b1_limb_4_col157,
                        b1_limb_5_col158,
                        b1_limb_6_col159,
                        b1_limb_7_col160,
                        b1_limb_8_col161,
                        b1_limb_9_col162,
                        b1_limb_10_col163,
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
                    ]),
                    b1_id_col152,
                );

                // Read Positive Num Bits 99.

                let memory_address_to_id_value_tmp_c1b19_73 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable {
                        segment_index: values_ptr_packed_segment_id,
                        offset: values_ptr_packed_offset
                            + (read_small_output_tmp_c1b19_47.0)
                            + (M31_2),
                    });
                let memory_id_to_big_value_tmp_c1b19_74 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_c1b19_73);
                let b2_id_col164 = memory_address_to_id_value_tmp_c1b19_73;
                *row[164] = b2_id_col164;
                *sub_component_inputs.memory_address_to_id[23] = PackedRelocatable {
                    segment_index: values_ptr_packed_segment_id,
                    offset: values_ptr_packed_offset + (read_small_output_tmp_c1b19_47.0) + (M31_2),
                };
                *lookup_data.memory_address_to_id_23 = [
                    values_ptr_packed_segment_id,
                    values_ptr_packed_offset + (read_small_output_tmp_c1b19_47.0) + (M31_2),
                    b2_id_col164,
                ];
                let b2_limb_0_col165 = memory_id_to_big_value_tmp_c1b19_74.get_m31(0);
                *row[165] = b2_limb_0_col165;
                let b2_limb_1_col166 = memory_id_to_big_value_tmp_c1b19_74.get_m31(1);
                *row[166] = b2_limb_1_col166;
                let b2_limb_2_col167 = memory_id_to_big_value_tmp_c1b19_74.get_m31(2);
                *row[167] = b2_limb_2_col167;
                let b2_limb_3_col168 = memory_id_to_big_value_tmp_c1b19_74.get_m31(3);
                *row[168] = b2_limb_3_col168;
                let b2_limb_4_col169 = memory_id_to_big_value_tmp_c1b19_74.get_m31(4);
                *row[169] = b2_limb_4_col169;
                let b2_limb_5_col170 = memory_id_to_big_value_tmp_c1b19_74.get_m31(5);
                *row[170] = b2_limb_5_col170;
                let b2_limb_6_col171 = memory_id_to_big_value_tmp_c1b19_74.get_m31(6);
                *row[171] = b2_limb_6_col171;
                let b2_limb_7_col172 = memory_id_to_big_value_tmp_c1b19_74.get_m31(7);
                *row[172] = b2_limb_7_col172;
                let b2_limb_8_col173 = memory_id_to_big_value_tmp_c1b19_74.get_m31(8);
                *row[173] = b2_limb_8_col173;
                let b2_limb_9_col174 = memory_id_to_big_value_tmp_c1b19_74.get_m31(9);
                *row[174] = b2_limb_9_col174;
                let b2_limb_10_col175 = memory_id_to_big_value_tmp_c1b19_74.get_m31(10);
                *row[175] = b2_limb_10_col175;
                *sub_component_inputs.memory_id_to_big[18] = b2_id_col164;
                *lookup_data.memory_id_to_big_18 = [
                    b2_id_col164,
                    b2_limb_0_col165,
                    b2_limb_1_col166,
                    b2_limb_2_col167,
                    b2_limb_3_col168,
                    b2_limb_4_col169,
                    b2_limb_5_col170,
                    b2_limb_6_col171,
                    b2_limb_7_col172,
                    b2_limb_8_col173,
                    b2_limb_9_col174,
                    b2_limb_10_col175,
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
                let read_positive_num_bits_99_output_tmp_c1b19_75 = (
                    PackedFelt252::from_limbs([
                        b2_limb_0_col165,
                        b2_limb_1_col166,
                        b2_limb_2_col167,
                        b2_limb_3_col168,
                        b2_limb_4_col169,
                        b2_limb_5_col170,
                        b2_limb_6_col171,
                        b2_limb_7_col172,
                        b2_limb_8_col173,
                        b2_limb_9_col174,
                        b2_limb_10_col175,
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
                    ]),
                    b2_id_col164,
                );

                // Read Positive Num Bits 99.

                let memory_address_to_id_value_tmp_c1b19_76 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable {
                        segment_index: values_ptr_packed_segment_id,
                        offset: values_ptr_packed_offset
                            + (read_small_output_tmp_c1b19_47.0)
                            + (M31_3),
                    });
                let memory_id_to_big_value_tmp_c1b19_77 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_c1b19_76);
                let b3_id_col176 = memory_address_to_id_value_tmp_c1b19_76;
                *row[176] = b3_id_col176;
                *sub_component_inputs.memory_address_to_id[24] = PackedRelocatable {
                    segment_index: values_ptr_packed_segment_id,
                    offset: values_ptr_packed_offset + (read_small_output_tmp_c1b19_47.0) + (M31_3),
                };
                *lookup_data.memory_address_to_id_24 = [
                    values_ptr_packed_segment_id,
                    values_ptr_packed_offset + (read_small_output_tmp_c1b19_47.0) + (M31_3),
                    b3_id_col176,
                ];
                let b3_limb_0_col177 = memory_id_to_big_value_tmp_c1b19_77.get_m31(0);
                *row[177] = b3_limb_0_col177;
                let b3_limb_1_col178 = memory_id_to_big_value_tmp_c1b19_77.get_m31(1);
                *row[178] = b3_limb_1_col178;
                let b3_limb_2_col179 = memory_id_to_big_value_tmp_c1b19_77.get_m31(2);
                *row[179] = b3_limb_2_col179;
                let b3_limb_3_col180 = memory_id_to_big_value_tmp_c1b19_77.get_m31(3);
                *row[180] = b3_limb_3_col180;
                let b3_limb_4_col181 = memory_id_to_big_value_tmp_c1b19_77.get_m31(4);
                *row[181] = b3_limb_4_col181;
                let b3_limb_5_col182 = memory_id_to_big_value_tmp_c1b19_77.get_m31(5);
                *row[182] = b3_limb_5_col182;
                let b3_limb_6_col183 = memory_id_to_big_value_tmp_c1b19_77.get_m31(6);
                *row[183] = b3_limb_6_col183;
                let b3_limb_7_col184 = memory_id_to_big_value_tmp_c1b19_77.get_m31(7);
                *row[184] = b3_limb_7_col184;
                let b3_limb_8_col185 = memory_id_to_big_value_tmp_c1b19_77.get_m31(8);
                *row[185] = b3_limb_8_col185;
                let b3_limb_9_col186 = memory_id_to_big_value_tmp_c1b19_77.get_m31(9);
                *row[186] = b3_limb_9_col186;
                let b3_limb_10_col187 = memory_id_to_big_value_tmp_c1b19_77.get_m31(10);
                *row[187] = b3_limb_10_col187;
                *sub_component_inputs.memory_id_to_big[19] = b3_id_col176;
                *lookup_data.memory_id_to_big_19 = [
                    b3_id_col176,
                    b3_limb_0_col177,
                    b3_limb_1_col178,
                    b3_limb_2_col179,
                    b3_limb_3_col180,
                    b3_limb_4_col181,
                    b3_limb_5_col182,
                    b3_limb_6_col183,
                    b3_limb_7_col184,
                    b3_limb_8_col185,
                    b3_limb_9_col186,
                    b3_limb_10_col187,
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
                let read_positive_num_bits_99_output_tmp_c1b19_78 = (
                    PackedFelt252::from_limbs([
                        b3_limb_0_col177,
                        b3_limb_1_col178,
                        b3_limb_2_col179,
                        b3_limb_3_col180,
                        b3_limb_4_col181,
                        b3_limb_5_col182,
                        b3_limb_6_col183,
                        b3_limb_7_col184,
                        b3_limb_8_col185,
                        b3_limb_9_col186,
                        b3_limb_10_col187,
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
                    ]),
                    b3_id_col176,
                );

                // Read Positive Num Bits 99.

                let memory_address_to_id_value_tmp_c1b19_79 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable {
                        segment_index: values_ptr_packed_segment_id,
                        offset: values_ptr_packed_offset + (read_small_output_tmp_c1b19_53.0),
                    });
                let memory_id_to_big_value_tmp_c1b19_80 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_c1b19_79);
                let c0_id_col188 = memory_address_to_id_value_tmp_c1b19_79;
                *row[188] = c0_id_col188;
                *sub_component_inputs.memory_address_to_id[25] = PackedRelocatable {
                    segment_index: values_ptr_packed_segment_id,
                    offset: values_ptr_packed_offset + (read_small_output_tmp_c1b19_53.0),
                };
                *lookup_data.memory_address_to_id_25 = [
                    values_ptr_packed_segment_id,
                    values_ptr_packed_offset + (read_small_output_tmp_c1b19_53.0),
                    c0_id_col188,
                ];
                let c0_limb_0_col189 = memory_id_to_big_value_tmp_c1b19_80.get_m31(0);
                *row[189] = c0_limb_0_col189;
                let c0_limb_1_col190 = memory_id_to_big_value_tmp_c1b19_80.get_m31(1);
                *row[190] = c0_limb_1_col190;
                let c0_limb_2_col191 = memory_id_to_big_value_tmp_c1b19_80.get_m31(2);
                *row[191] = c0_limb_2_col191;
                let c0_limb_3_col192 = memory_id_to_big_value_tmp_c1b19_80.get_m31(3);
                *row[192] = c0_limb_3_col192;
                let c0_limb_4_col193 = memory_id_to_big_value_tmp_c1b19_80.get_m31(4);
                *row[193] = c0_limb_4_col193;
                let c0_limb_5_col194 = memory_id_to_big_value_tmp_c1b19_80.get_m31(5);
                *row[194] = c0_limb_5_col194;
                let c0_limb_6_col195 = memory_id_to_big_value_tmp_c1b19_80.get_m31(6);
                *row[195] = c0_limb_6_col195;
                let c0_limb_7_col196 = memory_id_to_big_value_tmp_c1b19_80.get_m31(7);
                *row[196] = c0_limb_7_col196;
                let c0_limb_8_col197 = memory_id_to_big_value_tmp_c1b19_80.get_m31(8);
                *row[197] = c0_limb_8_col197;
                let c0_limb_9_col198 = memory_id_to_big_value_tmp_c1b19_80.get_m31(9);
                *row[198] = c0_limb_9_col198;
                let c0_limb_10_col199 = memory_id_to_big_value_tmp_c1b19_80.get_m31(10);
                *row[199] = c0_limb_10_col199;
                *sub_component_inputs.memory_id_to_big[20] = c0_id_col188;
                *lookup_data.memory_id_to_big_20 = [
                    c0_id_col188,
                    c0_limb_0_col189,
                    c0_limb_1_col190,
                    c0_limb_2_col191,
                    c0_limb_3_col192,
                    c0_limb_4_col193,
                    c0_limb_5_col194,
                    c0_limb_6_col195,
                    c0_limb_7_col196,
                    c0_limb_8_col197,
                    c0_limb_9_col198,
                    c0_limb_10_col199,
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
                let read_positive_num_bits_99_output_tmp_c1b19_81 = (
                    PackedFelt252::from_limbs([
                        c0_limb_0_col189,
                        c0_limb_1_col190,
                        c0_limb_2_col191,
                        c0_limb_3_col192,
                        c0_limb_4_col193,
                        c0_limb_5_col194,
                        c0_limb_6_col195,
                        c0_limb_7_col196,
                        c0_limb_8_col197,
                        c0_limb_9_col198,
                        c0_limb_10_col199,
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
                    ]),
                    c0_id_col188,
                );

                // Read Positive Num Bits 99.

                let memory_address_to_id_value_tmp_c1b19_82 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable {
                        segment_index: values_ptr_packed_segment_id,
                        offset: values_ptr_packed_offset
                            + (read_small_output_tmp_c1b19_53.0)
                            + (M31_1),
                    });
                let memory_id_to_big_value_tmp_c1b19_83 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_c1b19_82);
                let c1_id_col200 = memory_address_to_id_value_tmp_c1b19_82;
                *row[200] = c1_id_col200;
                *sub_component_inputs.memory_address_to_id[26] = PackedRelocatable {
                    segment_index: values_ptr_packed_segment_id,
                    offset: values_ptr_packed_offset + (read_small_output_tmp_c1b19_53.0) + (M31_1),
                };
                *lookup_data.memory_address_to_id_26 = [
                    values_ptr_packed_segment_id,
                    values_ptr_packed_offset + (read_small_output_tmp_c1b19_53.0) + (M31_1),
                    c1_id_col200,
                ];
                let c1_limb_0_col201 = memory_id_to_big_value_tmp_c1b19_83.get_m31(0);
                *row[201] = c1_limb_0_col201;
                let c1_limb_1_col202 = memory_id_to_big_value_tmp_c1b19_83.get_m31(1);
                *row[202] = c1_limb_1_col202;
                let c1_limb_2_col203 = memory_id_to_big_value_tmp_c1b19_83.get_m31(2);
                *row[203] = c1_limb_2_col203;
                let c1_limb_3_col204 = memory_id_to_big_value_tmp_c1b19_83.get_m31(3);
                *row[204] = c1_limb_3_col204;
                let c1_limb_4_col205 = memory_id_to_big_value_tmp_c1b19_83.get_m31(4);
                *row[205] = c1_limb_4_col205;
                let c1_limb_5_col206 = memory_id_to_big_value_tmp_c1b19_83.get_m31(5);
                *row[206] = c1_limb_5_col206;
                let c1_limb_6_col207 = memory_id_to_big_value_tmp_c1b19_83.get_m31(6);
                *row[207] = c1_limb_6_col207;
                let c1_limb_7_col208 = memory_id_to_big_value_tmp_c1b19_83.get_m31(7);
                *row[208] = c1_limb_7_col208;
                let c1_limb_8_col209 = memory_id_to_big_value_tmp_c1b19_83.get_m31(8);
                *row[209] = c1_limb_8_col209;
                let c1_limb_9_col210 = memory_id_to_big_value_tmp_c1b19_83.get_m31(9);
                *row[210] = c1_limb_9_col210;
                let c1_limb_10_col211 = memory_id_to_big_value_tmp_c1b19_83.get_m31(10);
                *row[211] = c1_limb_10_col211;
                *sub_component_inputs.memory_id_to_big[21] = c1_id_col200;
                *lookup_data.memory_id_to_big_21 = [
                    c1_id_col200,
                    c1_limb_0_col201,
                    c1_limb_1_col202,
                    c1_limb_2_col203,
                    c1_limb_3_col204,
                    c1_limb_4_col205,
                    c1_limb_5_col206,
                    c1_limb_6_col207,
                    c1_limb_7_col208,
                    c1_limb_8_col209,
                    c1_limb_9_col210,
                    c1_limb_10_col211,
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
                let read_positive_num_bits_99_output_tmp_c1b19_84 = (
                    PackedFelt252::from_limbs([
                        c1_limb_0_col201,
                        c1_limb_1_col202,
                        c1_limb_2_col203,
                        c1_limb_3_col204,
                        c1_limb_4_col205,
                        c1_limb_5_col206,
                        c1_limb_6_col207,
                        c1_limb_7_col208,
                        c1_limb_8_col209,
                        c1_limb_9_col210,
                        c1_limb_10_col211,
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
                    ]),
                    c1_id_col200,
                );

                // Read Positive Num Bits 99.

                let memory_address_to_id_value_tmp_c1b19_85 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable {
                        segment_index: values_ptr_packed_segment_id,
                        offset: values_ptr_packed_offset
                            + (read_small_output_tmp_c1b19_53.0)
                            + (M31_2),
                    });
                let memory_id_to_big_value_tmp_c1b19_86 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_c1b19_85);
                let c2_id_col212 = memory_address_to_id_value_tmp_c1b19_85;
                *row[212] = c2_id_col212;
                *sub_component_inputs.memory_address_to_id[27] = PackedRelocatable {
                    segment_index: values_ptr_packed_segment_id,
                    offset: values_ptr_packed_offset + (read_small_output_tmp_c1b19_53.0) + (M31_2),
                };
                *lookup_data.memory_address_to_id_27 = [
                    values_ptr_packed_segment_id,
                    values_ptr_packed_offset + (read_small_output_tmp_c1b19_53.0) + (M31_2),
                    c2_id_col212,
                ];
                let c2_limb_0_col213 = memory_id_to_big_value_tmp_c1b19_86.get_m31(0);
                *row[213] = c2_limb_0_col213;
                let c2_limb_1_col214 = memory_id_to_big_value_tmp_c1b19_86.get_m31(1);
                *row[214] = c2_limb_1_col214;
                let c2_limb_2_col215 = memory_id_to_big_value_tmp_c1b19_86.get_m31(2);
                *row[215] = c2_limb_2_col215;
                let c2_limb_3_col216 = memory_id_to_big_value_tmp_c1b19_86.get_m31(3);
                *row[216] = c2_limb_3_col216;
                let c2_limb_4_col217 = memory_id_to_big_value_tmp_c1b19_86.get_m31(4);
                *row[217] = c2_limb_4_col217;
                let c2_limb_5_col218 = memory_id_to_big_value_tmp_c1b19_86.get_m31(5);
                *row[218] = c2_limb_5_col218;
                let c2_limb_6_col219 = memory_id_to_big_value_tmp_c1b19_86.get_m31(6);
                *row[219] = c2_limb_6_col219;
                let c2_limb_7_col220 = memory_id_to_big_value_tmp_c1b19_86.get_m31(7);
                *row[220] = c2_limb_7_col220;
                let c2_limb_8_col221 = memory_id_to_big_value_tmp_c1b19_86.get_m31(8);
                *row[221] = c2_limb_8_col221;
                let c2_limb_9_col222 = memory_id_to_big_value_tmp_c1b19_86.get_m31(9);
                *row[222] = c2_limb_9_col222;
                let c2_limb_10_col223 = memory_id_to_big_value_tmp_c1b19_86.get_m31(10);
                *row[223] = c2_limb_10_col223;
                *sub_component_inputs.memory_id_to_big[22] = c2_id_col212;
                *lookup_data.memory_id_to_big_22 = [
                    c2_id_col212,
                    c2_limb_0_col213,
                    c2_limb_1_col214,
                    c2_limb_2_col215,
                    c2_limb_3_col216,
                    c2_limb_4_col217,
                    c2_limb_5_col218,
                    c2_limb_6_col219,
                    c2_limb_7_col220,
                    c2_limb_8_col221,
                    c2_limb_9_col222,
                    c2_limb_10_col223,
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
                let read_positive_num_bits_99_output_tmp_c1b19_87 = (
                    PackedFelt252::from_limbs([
                        c2_limb_0_col213,
                        c2_limb_1_col214,
                        c2_limb_2_col215,
                        c2_limb_3_col216,
                        c2_limb_4_col217,
                        c2_limb_5_col218,
                        c2_limb_6_col219,
                        c2_limb_7_col220,
                        c2_limb_8_col221,
                        c2_limb_9_col222,
                        c2_limb_10_col223,
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
                    ]),
                    c2_id_col212,
                );

                // Read Positive Num Bits 99.

                let memory_address_to_id_value_tmp_c1b19_88 = memory_address_to_id_state
                    .deduce_output(PackedRelocatable {
                        segment_index: values_ptr_packed_segment_id,
                        offset: values_ptr_packed_offset
                            + (read_small_output_tmp_c1b19_53.0)
                            + (M31_3),
                    });
                let memory_id_to_big_value_tmp_c1b19_89 =
                    memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_c1b19_88);
                let c3_id_col224 = memory_address_to_id_value_tmp_c1b19_88;
                *row[224] = c3_id_col224;
                *sub_component_inputs.memory_address_to_id[28] = PackedRelocatable {
                    segment_index: values_ptr_packed_segment_id,
                    offset: values_ptr_packed_offset + (read_small_output_tmp_c1b19_53.0) + (M31_3),
                };
                *lookup_data.memory_address_to_id_28 = [
                    values_ptr_packed_segment_id,
                    values_ptr_packed_offset + (read_small_output_tmp_c1b19_53.0) + (M31_3),
                    c3_id_col224,
                ];
                let c3_limb_0_col225 = memory_id_to_big_value_tmp_c1b19_89.get_m31(0);
                *row[225] = c3_limb_0_col225;
                let c3_limb_1_col226 = memory_id_to_big_value_tmp_c1b19_89.get_m31(1);
                *row[226] = c3_limb_1_col226;
                let c3_limb_2_col227 = memory_id_to_big_value_tmp_c1b19_89.get_m31(2);
                *row[227] = c3_limb_2_col227;
                let c3_limb_3_col228 = memory_id_to_big_value_tmp_c1b19_89.get_m31(3);
                *row[228] = c3_limb_3_col228;
                let c3_limb_4_col229 = memory_id_to_big_value_tmp_c1b19_89.get_m31(4);
                *row[229] = c3_limb_4_col229;
                let c3_limb_5_col230 = memory_id_to_big_value_tmp_c1b19_89.get_m31(5);
                *row[230] = c3_limb_5_col230;
                let c3_limb_6_col231 = memory_id_to_big_value_tmp_c1b19_89.get_m31(6);
                *row[231] = c3_limb_6_col231;
                let c3_limb_7_col232 = memory_id_to_big_value_tmp_c1b19_89.get_m31(7);
                *row[232] = c3_limb_7_col232;
                let c3_limb_8_col233 = memory_id_to_big_value_tmp_c1b19_89.get_m31(8);
                *row[233] = c3_limb_8_col233;
                let c3_limb_9_col234 = memory_id_to_big_value_tmp_c1b19_89.get_m31(9);
                *row[234] = c3_limb_9_col234;
                let c3_limb_10_col235 = memory_id_to_big_value_tmp_c1b19_89.get_m31(10);
                *row[235] = c3_limb_10_col235;
                *sub_component_inputs.memory_id_to_big[23] = c3_id_col224;
                *lookup_data.memory_id_to_big_23 = [
                    c3_id_col224,
                    c3_limb_0_col225,
                    c3_limb_1_col226,
                    c3_limb_2_col227,
                    c3_limb_3_col228,
                    c3_limb_4_col229,
                    c3_limb_5_col230,
                    c3_limb_6_col231,
                    c3_limb_7_col232,
                    c3_limb_8_col233,
                    c3_limb_9_col234,
                    c3_limb_10_col235,
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
                let read_positive_num_bits_99_output_tmp_c1b19_90 = (
                    PackedFelt252::from_limbs([
                        c3_limb_0_col225,
                        c3_limb_1_col226,
                        c3_limb_2_col227,
                        c3_limb_3_col228,
                        c3_limb_4_col229,
                        c3_limb_5_col230,
                        c3_limb_6_col231,
                        c3_limb_7_col232,
                        c3_limb_8_col233,
                        c3_limb_9_col234,
                        c3_limb_10_col235,
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
                    ]),
                    c3_id_col224,
                );

                let mod_utils_output_tmp_c1b19_91 = [
                    [
                        read_positive_num_bits_99_output_tmp_c1b19_5.0,
                        read_positive_num_bits_99_output_tmp_c1b19_8.0,
                        read_positive_num_bits_99_output_tmp_c1b19_11.0,
                        read_positive_num_bits_99_output_tmp_c1b19_14.0,
                    ],
                    [
                        read_positive_num_bits_99_output_tmp_c1b19_57.0,
                        read_positive_num_bits_99_output_tmp_c1b19_60.0,
                        read_positive_num_bits_99_output_tmp_c1b19_63.0,
                        read_positive_num_bits_99_output_tmp_c1b19_66.0,
                    ],
                    [
                        read_positive_num_bits_99_output_tmp_c1b19_69.0,
                        read_positive_num_bits_99_output_tmp_c1b19_72.0,
                        read_positive_num_bits_99_output_tmp_c1b19_75.0,
                        read_positive_num_bits_99_output_tmp_c1b19_78.0,
                    ],
                    [
                        read_positive_num_bits_99_output_tmp_c1b19_81.0,
                        read_positive_num_bits_99_output_tmp_c1b19_84.0,
                        read_positive_num_bits_99_output_tmp_c1b19_87.0,
                        read_positive_num_bits_99_output_tmp_c1b19_90.0,
                    ],
                ];

                let diff_tmp_c1b19_92 =
                    (((PackedBigUInt::<384, 6, 32>::from_packed_felt252_array(&[
                        mod_utils_output_tmp_c1b19_91[1][0],
                        mod_utils_output_tmp_c1b19_91[1][1],
                        mod_utils_output_tmp_c1b19_91[1][2],
                        mod_utils_output_tmp_c1b19_91[1][3],
                    ])) + (PackedBigUInt::<384, 6, 32>::from_packed_felt252_array(&[
                        mod_utils_output_tmp_c1b19_91[2][0],
                        mod_utils_output_tmp_c1b19_91[2][1],
                        mod_utils_output_tmp_c1b19_91[2][2],
                        mod_utils_output_tmp_c1b19_91[2][3],
                    ]))) - (PackedBigUInt::<384, 6, 32>::from_packed_felt252_array(&[
                        mod_utils_output_tmp_c1b19_91[3][0],
                        mod_utils_output_tmp_c1b19_91[3][1],
                        mod_utils_output_tmp_c1b19_91[3][2],
                        mod_utils_output_tmp_c1b19_91[3][3],
                    ])));
                let is_diff_0_tmp_c1b19_93 = diff_tmp_c1b19_92.eq(BigUInt_384_6_32_0_0_0_0_0_0);
                let sub_p_bit_col236 = ((M31_1) - (is_diff_0_tmp_c1b19_93.as_m31()));
                *row[236] = sub_p_bit_col236;
                let carry_0_col237 = (((((((a0_limb_0_col93) + (b0_limb_0_col141))
                    - (c0_limb_0_col189))
                    - ((p0_limb_0_col2) * (sub_p_bit_col236)))
                    + ((M31_512)
                        * ((((a0_limb_1_col94) + (b0_limb_1_col142)) - (c0_limb_1_col190))
                            - ((p0_limb_1_col3) * (sub_p_bit_col236)))))
                    + ((M31_262144)
                        * ((((a0_limb_2_col95) + (b0_limb_2_col143)) - (c0_limb_2_col191))
                            - ((p0_limb_2_col4) * (sub_p_bit_col236)))))
                    * (M31_16));
                *row[237] = carry_0_col237;
                let carry_1_col238 = (((((carry_0_col237)
                    + ((((a0_limb_3_col96) + (b0_limb_3_col144)) - (c0_limb_3_col192))
                        - ((p0_limb_3_col5) * (sub_p_bit_col236))))
                    + ((M31_512)
                        * ((((a0_limb_4_col97) + (b0_limb_4_col145)) - (c0_limb_4_col193))
                            - ((p0_limb_4_col6) * (sub_p_bit_col236)))))
                    + ((M31_262144)
                        * ((((a0_limb_5_col98) + (b0_limb_5_col146)) - (c0_limb_5_col194))
                            - ((p0_limb_5_col7) * (sub_p_bit_col236)))))
                    * (M31_16));
                *row[238] = carry_1_col238;
                let carry_2_col239 = (((((carry_1_col238)
                    + ((((a0_limb_6_col99) + (b0_limb_6_col147)) - (c0_limb_6_col195))
                        - ((p0_limb_6_col8) * (sub_p_bit_col236))))
                    + ((M31_512)
                        * ((((a0_limb_7_col100) + (b0_limb_7_col148)) - (c0_limb_7_col196))
                            - ((p0_limb_7_col9) * (sub_p_bit_col236)))))
                    + ((M31_262144)
                        * ((((a0_limb_8_col101) + (b0_limb_8_col149)) - (c0_limb_8_col197))
                            - ((p0_limb_8_col10) * (sub_p_bit_col236)))))
                    * (M31_16));
                *row[239] = carry_2_col239;
                let carry_3_col240 = (((((carry_2_col239)
                    + ((((a0_limb_9_col102) + (b0_limb_9_col150)) - (c0_limb_9_col198))
                        - ((p0_limb_9_col11) * (sub_p_bit_col236))))
                    + ((M31_512)
                        * ((((a0_limb_10_col103) + (b0_limb_10_col151)) - (c0_limb_10_col199))
                            - ((p0_limb_10_col12) * (sub_p_bit_col236)))))
                    + ((M31_32768)
                        * ((((a1_limb_0_col105) + (b1_limb_0_col153)) - (c1_limb_0_col201))
                            - ((p1_limb_0_col14) * (sub_p_bit_col236)))))
                    * (M31_128));
                *row[240] = carry_3_col240;
                let carry_4_col241 = (((((carry_3_col240)
                    + ((((a1_limb_1_col106) + (b1_limb_1_col154)) - (c1_limb_1_col202))
                        - ((p1_limb_1_col15) * (sub_p_bit_col236))))
                    + ((M31_512)
                        * ((((a1_limb_2_col107) + (b1_limb_2_col155)) - (c1_limb_2_col203))
                            - ((p1_limb_2_col16) * (sub_p_bit_col236)))))
                    + ((M31_262144)
                        * ((((a1_limb_3_col108) + (b1_limb_3_col156)) - (c1_limb_3_col204))
                            - ((p1_limb_3_col17) * (sub_p_bit_col236)))))
                    * (M31_16));
                *row[241] = carry_4_col241;
                let carry_5_col242 = (((((carry_4_col241)
                    + ((((a1_limb_4_col109) + (b1_limb_4_col157)) - (c1_limb_4_col205))
                        - ((p1_limb_4_col18) * (sub_p_bit_col236))))
                    + ((M31_512)
                        * ((((a1_limb_5_col110) + (b1_limb_5_col158)) - (c1_limb_5_col206))
                            - ((p1_limb_5_col19) * (sub_p_bit_col236)))))
                    + ((M31_262144)
                        * ((((a1_limb_6_col111) + (b1_limb_6_col159)) - (c1_limb_6_col207))
                            - ((p1_limb_6_col20) * (sub_p_bit_col236)))))
                    * (M31_16));
                *row[242] = carry_5_col242;
                let carry_6_col243 = (((((carry_5_col242)
                    + ((((a1_limb_7_col112) + (b1_limb_7_col160)) - (c1_limb_7_col208))
                        - ((p1_limb_7_col21) * (sub_p_bit_col236))))
                    + ((M31_512)
                        * ((((a1_limb_8_col113) + (b1_limb_8_col161)) - (c1_limb_8_col209))
                            - ((p1_limb_8_col22) * (sub_p_bit_col236)))))
                    + ((M31_262144)
                        * ((((a1_limb_9_col114) + (b1_limb_9_col162)) - (c1_limb_9_col210))
                            - ((p1_limb_9_col23) * (sub_p_bit_col236)))))
                    * (M31_16));
                *row[243] = carry_6_col243;
                let carry_7_col244 = (((((carry_6_col243)
                    + ((((a1_limb_10_col115) + (b1_limb_10_col163)) - (c1_limb_10_col211))
                        - ((p1_limb_10_col24) * (sub_p_bit_col236))))
                    + ((M31_64)
                        * ((((a2_limb_0_col117) + (b2_limb_0_col165)) - (c2_limb_0_col213))
                            - ((p2_limb_0_col26) * (sub_p_bit_col236)))))
                    + ((M31_32768)
                        * ((((a2_limb_1_col118) + (b2_limb_1_col166)) - (c2_limb_1_col214))
                            - ((p2_limb_1_col27) * (sub_p_bit_col236)))))
                    * (M31_128));
                *row[244] = carry_7_col244;
                let carry_8_col245 = (((((carry_7_col244)
                    + ((((a2_limb_2_col119) + (b2_limb_2_col167)) - (c2_limb_2_col215))
                        - ((p2_limb_2_col28) * (sub_p_bit_col236))))
                    + ((M31_512)
                        * ((((a2_limb_3_col120) + (b2_limb_3_col168)) - (c2_limb_3_col216))
                            - ((p2_limb_3_col29) * (sub_p_bit_col236)))))
                    + ((M31_262144)
                        * ((((a2_limb_4_col121) + (b2_limb_4_col169)) - (c2_limb_4_col217))
                            - ((p2_limb_4_col30) * (sub_p_bit_col236)))))
                    * (M31_16));
                *row[245] = carry_8_col245;
                let carry_9_col246 = (((((carry_8_col245)
                    + ((((a2_limb_5_col122) + (b2_limb_5_col170)) - (c2_limb_5_col218))
                        - ((p2_limb_5_col31) * (sub_p_bit_col236))))
                    + ((M31_512)
                        * ((((a2_limb_6_col123) + (b2_limb_6_col171)) - (c2_limb_6_col219))
                            - ((p2_limb_6_col32) * (sub_p_bit_col236)))))
                    + ((M31_262144)
                        * ((((a2_limb_7_col124) + (b2_limb_7_col172)) - (c2_limb_7_col220))
                            - ((p2_limb_7_col33) * (sub_p_bit_col236)))))
                    * (M31_16));
                *row[246] = carry_9_col246;
                let carry_10_col247 = (((((carry_9_col246)
                    + ((((a2_limb_8_col125) + (b2_limb_8_col173)) - (c2_limb_8_col221))
                        - ((p2_limb_8_col34) * (sub_p_bit_col236))))
                    + ((M31_512)
                        * ((((a2_limb_9_col126) + (b2_limb_9_col174)) - (c2_limb_9_col222))
                            - ((p2_limb_9_col35) * (sub_p_bit_col236)))))
                    + ((M31_262144)
                        * ((((a2_limb_10_col127) + (b2_limb_10_col175)) - (c2_limb_10_col223))
                            - ((p2_limb_10_col36) * (sub_p_bit_col236)))))
                    * (M31_128));
                *row[247] = carry_10_col247;
                let carry_11_col248 = (((((carry_10_col247)
                    + ((((a3_limb_0_col129) + (b3_limb_0_col177)) - (c3_limb_0_col225))
                        - ((p3_limb_0_col38) * (sub_p_bit_col236))))
                    + ((M31_512)
                        * ((((a3_limb_1_col130) + (b3_limb_1_col178)) - (c3_limb_1_col226))
                            - ((p3_limb_1_col39) * (sub_p_bit_col236)))))
                    + ((M31_262144)
                        * ((((a3_limb_2_col131) + (b3_limb_2_col179)) - (c3_limb_2_col227))
                            - ((p3_limb_2_col40) * (sub_p_bit_col236)))))
                    * (M31_16));
                *row[248] = carry_11_col248;
                let carry_12_col249 = (((((carry_11_col248)
                    + ((((a3_limb_3_col132) + (b3_limb_3_col180)) - (c3_limb_3_col228))
                        - ((p3_limb_3_col41) * (sub_p_bit_col236))))
                    + ((M31_512)
                        * ((((a3_limb_4_col133) + (b3_limb_4_col181)) - (c3_limb_4_col229))
                            - ((p3_limb_4_col42) * (sub_p_bit_col236)))))
                    + ((M31_262144)
                        * ((((a3_limb_5_col134) + (b3_limb_5_col182)) - (c3_limb_5_col230))
                            - ((p3_limb_5_col43) * (sub_p_bit_col236)))))
                    * (M31_16));
                *row[249] = carry_12_col249;
                let carry_13_col250 = (((((carry_12_col249)
                    + ((((a3_limb_6_col135) + (b3_limb_6_col183)) - (c3_limb_6_col231))
                        - ((p3_limb_6_col44) * (sub_p_bit_col236))))
                    + ((M31_512)
                        * ((((a3_limb_7_col136) + (b3_limb_7_col184)) - (c3_limb_7_col232))
                            - ((p3_limb_7_col45) * (sub_p_bit_col236)))))
                    + ((M31_262144)
                        * ((((a3_limb_8_col137) + (b3_limb_8_col185)) - (c3_limb_8_col233))
                            - ((p3_limb_8_col46) * (sub_p_bit_col236)))))
                    * (M31_16));
                *row[250] = carry_13_col250;
                *row[251] = segment_id_packed;
            },
        );

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    memory_address_to_id_0: Vec<[PackedM31; 3]>,
    memory_address_to_id_1: Vec<[PackedM31; 3]>,
    memory_address_to_id_2: Vec<[PackedM31; 3]>,
    memory_address_to_id_3: Vec<[PackedM31; 3]>,
    memory_address_to_id_4: Vec<[PackedM31; 3]>,
    memory_address_to_id_5: Vec<[PackedM31; 3]>,
    memory_address_to_id_6: Vec<[PackedM31; 3]>,
    memory_address_to_id_7: Vec<[PackedM31; 3]>,
    memory_address_to_id_8: Vec<[PackedM31; 3]>,
    memory_address_to_id_9: Vec<[PackedM31; 3]>,
    memory_address_to_id_10: Vec<[PackedM31; 3]>,
    memory_address_to_id_11: Vec<[PackedM31; 3]>,
    memory_address_to_id_12: Vec<[PackedM31; 3]>,
    memory_address_to_id_13: Vec<[PackedM31; 3]>,
    memory_address_to_id_14: Vec<[PackedM31; 3]>,
    memory_address_to_id_15: Vec<[PackedM31; 3]>,
    memory_address_to_id_16: Vec<[PackedM31; 3]>,
    memory_address_to_id_17: Vec<[PackedM31; 3]>,
    memory_address_to_id_18: Vec<[PackedM31; 3]>,
    memory_address_to_id_19: Vec<[PackedM31; 3]>,
    memory_address_to_id_20: Vec<[PackedM31; 3]>,
    memory_address_to_id_21: Vec<[PackedM31; 3]>,
    memory_address_to_id_22: Vec<[PackedM31; 3]>,
    memory_address_to_id_23: Vec<[PackedM31; 3]>,
    memory_address_to_id_24: Vec<[PackedM31; 3]>,
    memory_address_to_id_25: Vec<[PackedM31; 3]>,
    memory_address_to_id_26: Vec<[PackedM31; 3]>,
    memory_address_to_id_27: Vec<[PackedM31; 3]>,
    memory_address_to_id_28: Vec<[PackedM31; 3]>,
    memory_id_to_big_0: Vec<[PackedM31; 29]>,
    memory_id_to_big_1: Vec<[PackedM31; 29]>,
    memory_id_to_big_2: Vec<[PackedM31; 29]>,
    memory_id_to_big_3: Vec<[PackedM31; 29]>,
    memory_id_to_big_4: Vec<[PackedM31; 29]>,
    memory_id_to_big_5: Vec<[PackedM31; 29]>,
    memory_id_to_big_6: Vec<[PackedM31; 29]>,
    memory_id_to_big_7: Vec<[PackedM31; 29]>,
    memory_id_to_big_8: Vec<[PackedM31; 29]>,
    memory_id_to_big_9: Vec<[PackedM31; 29]>,
    memory_id_to_big_10: Vec<[PackedM31; 29]>,
    memory_id_to_big_11: Vec<[PackedM31; 29]>,
    memory_id_to_big_12: Vec<[PackedM31; 29]>,
    memory_id_to_big_13: Vec<[PackedM31; 29]>,
    memory_id_to_big_14: Vec<[PackedM31; 29]>,
    memory_id_to_big_15: Vec<[PackedM31; 29]>,
    memory_id_to_big_16: Vec<[PackedM31; 29]>,
    memory_id_to_big_17: Vec<[PackedM31; 29]>,
    memory_id_to_big_18: Vec<[PackedM31; 29]>,
    memory_id_to_big_19: Vec<[PackedM31; 29]>,
    memory_id_to_big_20: Vec<[PackedM31; 29]>,
    memory_id_to_big_21: Vec<[PackedM31; 29]>,
    memory_id_to_big_22: Vec<[PackedM31; 29]>,
    memory_id_to_big_23: Vec<[PackedM31; 29]>,
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
    ) -> InteractionClaim {
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);

        // Sum logup terms in pairs.
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

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_address_to_id_1,
            &self.lookup_data.memory_id_to_big_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_address_to_id.combine(values0);
                let denom1: PackedQM31 = memory_id_to_big.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_address_to_id_2,
            &self.lookup_data.memory_id_to_big_2,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_address_to_id.combine(values0);
                let denom1: PackedQM31 = memory_id_to_big.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_address_to_id_3,
            &self.lookup_data.memory_id_to_big_3,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_address_to_id.combine(values0);
                let denom1: PackedQM31 = memory_id_to_big.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_address_to_id_4,
            &self.lookup_data.memory_id_to_big_4,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_address_to_id.combine(values0);
                let denom1: PackedQM31 = memory_id_to_big.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_address_to_id_5,
            &self.lookup_data.memory_id_to_big_5,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_address_to_id.combine(values0);
                let denom1: PackedQM31 = memory_id_to_big.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_address_to_id_6,
            &self.lookup_data.memory_id_to_big_6,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_address_to_id.combine(values0);
                let denom1: PackedQM31 = memory_id_to_big.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_address_to_id_7,
            &self.lookup_data.memory_id_to_big_7,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_address_to_id.combine(values0);
                let denom1: PackedQM31 = memory_id_to_big.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_address_to_id_8,
            &self.lookup_data.memory_id_to_big_8,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_address_to_id.combine(values0);
                let denom1: PackedQM31 = memory_id_to_big.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_address_to_id_9,
            &self.lookup_data.memory_address_to_id_10,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_address_to_id.combine(values0);
                let denom1: PackedQM31 = memory_address_to_id.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_address_to_id_11,
            &self.lookup_data.memory_address_to_id_12,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_address_to_id.combine(values0);
                let denom1: PackedQM31 = memory_address_to_id.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_address_to_id_13,
            &self.lookup_data.memory_address_to_id_14,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_address_to_id.combine(values0);
                let denom1: PackedQM31 = memory_address_to_id.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_id_to_big_9,
            &self.lookup_data.memory_address_to_id_15,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_id_to_big.combine(values0);
                let denom1: PackedQM31 = memory_address_to_id.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_id_to_big_10,
            &self.lookup_data.memory_address_to_id_16,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_id_to_big.combine(values0);
                let denom1: PackedQM31 = memory_address_to_id.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col(); // 48
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_id_to_big_11,
            &self.lookup_data.memory_address_to_id_17,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_id_to_big.combine(values0);
                let denom1: PackedQM31 = memory_address_to_id.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_id_to_big_12,
            &self.lookup_data.memory_address_to_id_18,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_id_to_big.combine(values0);
                let denom1: PackedQM31 = memory_address_to_id.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_id_to_big_13,
            &self.lookup_data.memory_address_to_id_19,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_id_to_big.combine(values0);
                let denom1: PackedQM31 = memory_address_to_id.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_id_to_big_14,
            &self.lookup_data.memory_address_to_id_20,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_id_to_big.combine(values0);
                let denom1: PackedQM31 = memory_address_to_id.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_id_to_big_15,
            &self.lookup_data.memory_address_to_id_21,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_id_to_big.combine(values0);
                let denom1: PackedQM31 = memory_address_to_id.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_id_to_big_16,
            &self.lookup_data.memory_address_to_id_22,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_id_to_big.combine(values0);
                let denom1: PackedQM31 = memory_address_to_id.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_id_to_big_17,
            &self.lookup_data.memory_address_to_id_23,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_id_to_big.combine(values0);
                let denom1: PackedQM31 = memory_address_to_id.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_id_to_big_18,
            &self.lookup_data.memory_address_to_id_24,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_id_to_big.combine(values0);
                let denom1: PackedQM31 = memory_address_to_id.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_id_to_big_19,
            &self.lookup_data.memory_address_to_id_25,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_id_to_big.combine(values0);
                let denom1: PackedQM31 = memory_address_to_id.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_id_to_big_20,
            &self.lookup_data.memory_address_to_id_26,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_id_to_big.combine(values0);
                let denom1: PackedQM31 = memory_address_to_id.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_id_to_big_21,
            &self.lookup_data.memory_address_to_id_27,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_id_to_big.combine(values0);
                let denom1: PackedQM31 = memory_address_to_id.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_id_to_big_22,
            &self.lookup_data.memory_address_to_id_28,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_id_to_big.combine(values0);
                let denom1: PackedQM31 = memory_address_to_id.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_id_to_big_23,
        )
            .into_par_iter()
            .for_each(|(writer, values)| {
                let denom = memory_id_to_big.combine(values);
                writer.write_frac(PackedQM31::one(), denom);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}
