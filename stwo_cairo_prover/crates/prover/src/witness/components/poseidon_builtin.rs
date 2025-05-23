#![allow(unused_parens)]
use cairo_air::components::poseidon_builtin::{Claim, InteractionClaim, N_TRACE_COLUMNS};

use crate::witness::components::{
    cube_252, memory_address_to_id, memory_id_to_big, poseidon_3_partial_rounds_chain,
    poseidon_full_round_chain, range_check_3_3_3_3_3, range_check_4_4, range_check_4_4_4_4,
    range_check_felt_252_width_27,
};
use crate::witness::prelude::*;

#[derive(Default)]
pub struct ClaimGenerator {
    pub log_size: u32,
    pub poseidon_builtin_segment_start: u32,
}
impl ClaimGenerator {
    pub fn new(log_size: u32, poseidon_builtin_segment_start: u32) -> Self {
        assert!(log_size >= LOG_N_LANES);
        Self {
            log_size,
            poseidon_builtin_segment_start,
        }
    }

    pub fn write_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        cube_252_state: &mut cube_252::ClaimGenerator,
        memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
        memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
        poseidon_3_partial_rounds_chain_state: &mut poseidon_3_partial_rounds_chain::ClaimGenerator,
        poseidon_full_round_chain_state: &mut poseidon_full_round_chain::ClaimGenerator,
        range_check_3_3_3_3_3_state: &range_check_3_3_3_3_3::ClaimGenerator,
        range_check_4_4_state: &range_check_4_4::ClaimGenerator,
        range_check_4_4_4_4_state: &range_check_4_4_4_4::ClaimGenerator,
        range_check_felt_252_width_27_state: &mut range_check_felt_252_width_27::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator) {
        let log_size = self.log_size;

        let (trace, lookup_data, sub_component_inputs) = write_trace_simd(
            log_size,
            self.poseidon_builtin_segment_start,
            cube_252_state,
            memory_address_to_id_state,
            memory_id_to_big_state,
            poseidon_3_partial_rounds_chain_state,
            poseidon_full_round_chain_state,
            range_check_3_3_3_3_3_state,
            range_check_4_4_state,
            range_check_4_4_4_4_state,
            range_check_felt_252_width_27_state,
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
        sub_component_inputs
            .poseidon_full_round_chain
            .iter()
            .for_each(|inputs| {
                poseidon_full_round_chain_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_felt_252_width_27
            .iter()
            .for_each(|inputs| {
                range_check_felt_252_width_27_state.add_packed_inputs(inputs);
            });
        sub_component_inputs.cube_252.iter().for_each(|inputs| {
            cube_252_state.add_packed_inputs(inputs);
        });
        sub_component_inputs
            .range_check_3_3_3_3_3
            .iter()
            .for_each(|inputs| {
                range_check_3_3_3_3_3_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_4_4_4_4
            .iter()
            .for_each(|inputs| {
                range_check_4_4_4_4_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .range_check_4_4
            .iter()
            .for_each(|inputs| {
                range_check_4_4_state.add_packed_inputs(inputs);
            });
        sub_component_inputs
            .poseidon_3_partial_rounds_chain
            .iter()
            .for_each(|inputs| {
                poseidon_3_partial_rounds_chain_state.add_packed_inputs(inputs);
            });
        tree_builder.extend_evals(trace.to_evals());

        (
            Claim {
                log_size,
                poseidon_builtin_segment_start: self.poseidon_builtin_segment_start,
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
    memory_address_to_id: [Vec<memory_address_to_id::PackedInputType>; 6],
    memory_id_to_big: [Vec<memory_id_to_big::PackedInputType>; 6],
    poseidon_full_round_chain: [Vec<poseidon_full_round_chain::PackedInputType>; 8],
    range_check_felt_252_width_27: [Vec<range_check_felt_252_width_27::PackedInputType>; 2],
    cube_252: [Vec<cube_252::PackedInputType>; 2],
    range_check_3_3_3_3_3: [Vec<range_check_3_3_3_3_3::PackedInputType>; 2],
    range_check_4_4_4_4: [Vec<range_check_4_4_4_4::PackedInputType>; 6],
    range_check_4_4: [Vec<range_check_4_4::PackedInputType>; 3],
    poseidon_3_partial_rounds_chain: [Vec<poseidon_3_partial_rounds_chain::PackedInputType>; 27],
}

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    log_size: u32,
    poseidon_builtin_segment_start: u32,
    cube_252_state: &cube_252::ClaimGenerator,
    memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
    memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
    poseidon_3_partial_rounds_chain_state: &poseidon_3_partial_rounds_chain::ClaimGenerator,
    poseidon_full_round_chain_state: &poseidon_full_round_chain::ClaimGenerator,
    range_check_3_3_3_3_3_state: &range_check_3_3_3_3_3::ClaimGenerator,
    range_check_4_4_state: &range_check_4_4::ClaimGenerator,
    range_check_4_4_4_4_state: &range_check_4_4_4_4::ClaimGenerator,
    range_check_felt_252_width_27_state: &range_check_felt_252_width_27::ClaimGenerator,
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

    let Felt252_0_0_0_0 = PackedFelt252::broadcast(Felt252::from([0, 0, 0, 0]));
    let Felt252_1_0_0_0 = PackedFelt252::broadcast(Felt252::from([1, 0, 0, 0]));
    let Felt252_10310704347937391837_5874215448258336115_2880320859071049537_45350836576946303 =
        PackedFelt252::broadcast(Felt252::from([
            10310704347937391837,
            5874215448258336115,
            2880320859071049537,
            45350836576946303,
        ]));
    let Felt252_10931822301410252833_1475756362763989377_3378552166684303673_348229636055909092 =
        PackedFelt252::broadcast(Felt252::from([
            10931822301410252833,
            1475756362763989377,
            3378552166684303673,
            348229636055909092,
        ]));
    let Felt252_11041071929982523380_7503192613203831446_4943121247101232560_560497091765764140 =
        PackedFelt252::broadcast(Felt252::from([
            11041071929982523380,
            7503192613203831446,
            4943121247101232560,
            560497091765764140,
        ]));
    let Felt252_16477292399064058052_4441744911417641572_18431044672185975386_252894828082060025 =
        PackedFelt252::broadcast(Felt252::from([
            16477292399064058052,
            4441744911417641572,
            18431044672185975386,
            252894828082060025,
        ]));
    let Felt252_2_0_0_0 = PackedFelt252::broadcast(Felt252::from([2, 0, 0, 0]));
    let Felt252_3969818800901670911_10562874008078701503_14906396266795319764_223312371439046257 =
        PackedFelt252::broadcast(Felt252::from([
            3969818800901670911,
            10562874008078701503,
            14906396266795319764,
            223312371439046257,
        ]));
    let Felt252_4_0_0_0 = PackedFelt252::broadcast(Felt252::from([4, 0, 0, 0]));
    let Felt252_8794894655201903369_3219077422080798056_16714934791572408267_262217499501479120 =
        PackedFelt252::broadcast(Felt252::from([
            8794894655201903369,
            3219077422080798056,
            16714934791572408267,
            262217499501479120,
        ]));
    let Felt252_9275160746813554287_16541205595039575623_4169650429605064889_470088886057789987 =
        PackedFelt252::broadcast(Felt252::from([
            9275160746813554287,
            16541205595039575623,
            4169650429605064889,
            470088886057789987,
        ]));
    let M31_0 = PackedM31::broadcast(M31::from(0));
    let M31_1 = PackedM31::broadcast(M31::from(1));
    let M31_10 = PackedM31::broadcast(M31::from(10));
    let M31_102193642 = PackedM31::broadcast(M31::from(102193642));
    let M31_103094260 = PackedM31::broadcast(M31::from(103094260));
    let M31_108487870 = PackedM31::broadcast(M31::from(108487870));
    let M31_11 = PackedM31::broadcast(M31::from(11));
    let M31_112479959 = PackedM31::broadcast(M31::from(112479959));
    let M31_112795138 = PackedM31::broadcast(M31::from(112795138));
    let M31_116986206 = PackedM31::broadcast(M31::from(116986206));
    let M31_117420501 = PackedM31::broadcast(M31::from(117420501));
    let M31_119023582 = PackedM31::broadcast(M31::from(119023582));
    let M31_12 = PackedM31::broadcast(M31::from(12));
    let M31_120369218 = PackedM31::broadcast(M31::from(120369218));
    let M31_121146754 = PackedM31::broadcast(M31::from(121146754));
    let M31_121657377 = PackedM31::broadcast(M31::from(121657377));
    let M31_122233508 = PackedM31::broadcast(M31::from(122233508));
    let M31_129717753 = PackedM31::broadcast(M31::from(129717753));
    let M31_13 = PackedM31::broadcast(M31::from(13));
    let M31_130418270 = PackedM31::broadcast(M31::from(130418270));
    let M31_133303902 = PackedM31::broadcast(M31::from(133303902));
    let M31_134217729 = PackedM31::broadcast(M31::from(134217729));
    let M31_136 = PackedM31::broadcast(M31::from(136));
    let M31_14 = PackedM31::broadcast(M31::from(14));
    let M31_15 = PackedM31::broadcast(M31::from(15));
    let M31_16 = PackedM31::broadcast(M31::from(16));
    let M31_16173996 = PackedM31::broadcast(M31::from(16173996));
    let M31_17 = PackedM31::broadcast(M31::from(17));
    let M31_18 = PackedM31::broadcast(M31::from(18));
    let M31_18765944 = PackedM31::broadcast(M31::from(18765944));
    let M31_19 = PackedM31::broadcast(M31::from(19));
    let M31_19292069 = PackedM31::broadcast(M31::from(19292069));
    let M31_2 = PackedM31::broadcast(M31::from(2));
    let M31_20 = PackedM31::broadcast(M31::from(20));
    let M31_21 = PackedM31::broadcast(M31::from(21));
    let M31_22 = PackedM31::broadcast(M31::from(22));
    let M31_22899501 = PackedM31::broadcast(M31::from(22899501));
    let M31_23 = PackedM31::broadcast(M31::from(23));
    let M31_24 = PackedM31::broadcast(M31::from(24));
    let M31_25 = PackedM31::broadcast(M31::from(25));
    let M31_26 = PackedM31::broadcast(M31::from(26));
    let M31_262144 = PackedM31::broadcast(M31::from(262144));
    let M31_27 = PackedM31::broadcast(M31::from(27));
    let M31_28 = PackedM31::broadcast(M31::from(28));
    let M31_28820206 = PackedM31::broadcast(M31::from(28820206));
    let M31_29 = PackedM31::broadcast(M31::from(29));
    let M31_3 = PackedM31::broadcast(M31::from(3));
    let M31_30 = PackedM31::broadcast(M31::from(30));
    let M31_31 = PackedM31::broadcast(M31::from(31));
    let M31_32 = PackedM31::broadcast(M31::from(32));
    let M31_33 = PackedM31::broadcast(M31::from(33));
    let M31_33413160 = PackedM31::broadcast(M31::from(33413160));
    let M31_33439011 = PackedM31::broadcast(M31::from(33439011));
    let M31_34 = PackedM31::broadcast(M31::from(34));
    let M31_35 = PackedM31::broadcast(M31::from(35));
    let M31_36279186 = PackedM31::broadcast(M31::from(36279186));
    let M31_4 = PackedM31::broadcast(M31::from(4));
    let M31_402653187 = PackedM31::broadcast(M31::from(402653187));
    let M31_40454143 = PackedM31::broadcast(M31::from(40454143));
    let M31_41224388 = PackedM31::broadcast(M31::from(41224388));
    let M31_41320857 = PackedM31::broadcast(M31::from(41320857));
    let M31_44781849 = PackedM31::broadcast(M31::from(44781849));
    let M31_44848225 = PackedM31::broadcast(M31::from(44848225));
    let M31_45351266 = PackedM31::broadcast(M31::from(45351266));
    let M31_45553283 = PackedM31::broadcast(M31::from(45553283));
    let M31_48193339 = PackedM31::broadcast(M31::from(48193339));
    let M31_48383197 = PackedM31::broadcast(M31::from(48383197));
    let M31_4883209 = PackedM31::broadcast(M31::from(4883209));
    let M31_48945103 = PackedM31::broadcast(M31::from(48945103));
    let M31_49157069 = PackedM31::broadcast(M31::from(49157069));
    let M31_49554771 = PackedM31::broadcast(M31::from(49554771));
    let M31_4974792 = PackedM31::broadcast(M31::from(4974792));
    let M31_5 = PackedM31::broadcast(M31::from(5));
    let M31_50468641 = PackedM31::broadcast(M31::from(50468641));
    let M31_50758155 = PackedM31::broadcast(M31::from(50758155));
    let M31_512 = PackedM31::broadcast(M31::from(512));
    let M31_54415179 = PackedM31::broadcast(M31::from(54415179));
    let M31_55508188 = PackedM31::broadcast(M31::from(55508188));
    let M31_55955004 = PackedM31::broadcast(M31::from(55955004));
    let M31_58475513 = PackedM31::broadcast(M31::from(58475513));
    let M31_59852719 = PackedM31::broadcast(M31::from(59852719));
    let M31_6 = PackedM31::broadcast(M31::from(6));
    let M31_60124463 = PackedM31::broadcast(M31::from(60124463));
    let M31_60709090 = PackedM31::broadcast(M31::from(60709090));
    let M31_62360091 = PackedM31::broadcast(M31::from(62360091));
    let M31_62439890 = PackedM31::broadcast(M31::from(62439890));
    let M31_65659846 = PackedM31::broadcast(M31::from(65659846));
    let M31_68491350 = PackedM31::broadcast(M31::from(68491350));
    let M31_7 = PackedM31::broadcast(M31::from(7));
    let M31_72285071 = PackedM31::broadcast(M31::from(72285071));
    let M31_74972783 = PackedM31::broadcast(M31::from(74972783));
    let M31_75104388 = PackedM31::broadcast(M31::from(75104388));
    let M31_77099918 = PackedM31::broadcast(M31::from(77099918));
    let M31_78826183 = PackedM31::broadcast(M31::from(78826183));
    let M31_79012328 = PackedM31::broadcast(M31::from(79012328));
    let M31_8 = PackedM31::broadcast(M31::from(8));
    let M31_8192 = PackedM31::broadcast(M31::from(8192));
    let M31_86573645 = PackedM31::broadcast(M31::from(86573645));
    let M31_88680813 = PackedM31::broadcast(M31::from(88680813));
    let M31_9 = PackedM31::broadcast(M31::from(9));
    let M31_90391646 = PackedM31::broadcast(M31::from(90391646));
    let M31_90842759 = PackedM31::broadcast(M31::from(90842759));
    let M31_91013252 = PackedM31::broadcast(M31::from(91013252));
    let M31_94624323 = PackedM31::broadcast(M31::from(94624323));
    let M31_95050340 = PackedM31::broadcast(M31::from(95050340));
    let seq = Seq::new(log_size);

    (trace.par_iter_mut(),
    lookup_data.par_iter_mut(),sub_component_inputs.par_iter_mut(),)
    .into_par_iter()
    .enumerate()
    .for_each(
        |(row_index,(mut row, lookup_data,sub_component_inputs,))| {
            let seq = seq.packed_at(row_index);

            //Read Positive Num Bits 252.

            let memory_address_to_id_value_tmp_51986_0 = memory_address_to_id_state.deduce_output(((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start))) + (((seq) * (M31_6)))));let memory_id_to_big_value_tmp_51986_1 = memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_51986_0);let input_state_0_id_col0 = memory_address_to_id_value_tmp_51986_0;
            *row[0] = input_state_0_id_col0;*sub_component_inputs.memory_address_to_id[0] =
                ((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start))) + (((seq) * (M31_6))));
            *lookup_data.memory_address_to_id_0 = [((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start))) + (((seq) * (M31_6)))), input_state_0_id_col0];let input_state_0_limb_0_col1 = memory_id_to_big_value_tmp_51986_1.get_m31(0);
            *row[1] = input_state_0_limb_0_col1;let input_state_0_limb_1_col2 = memory_id_to_big_value_tmp_51986_1.get_m31(1);
            *row[2] = input_state_0_limb_1_col2;let input_state_0_limb_2_col3 = memory_id_to_big_value_tmp_51986_1.get_m31(2);
            *row[3] = input_state_0_limb_2_col3;let input_state_0_limb_3_col4 = memory_id_to_big_value_tmp_51986_1.get_m31(3);
            *row[4] = input_state_0_limb_3_col4;let input_state_0_limb_4_col5 = memory_id_to_big_value_tmp_51986_1.get_m31(4);
            *row[5] = input_state_0_limb_4_col5;let input_state_0_limb_5_col6 = memory_id_to_big_value_tmp_51986_1.get_m31(5);
            *row[6] = input_state_0_limb_5_col6;let input_state_0_limb_6_col7 = memory_id_to_big_value_tmp_51986_1.get_m31(6);
            *row[7] = input_state_0_limb_6_col7;let input_state_0_limb_7_col8 = memory_id_to_big_value_tmp_51986_1.get_m31(7);
            *row[8] = input_state_0_limb_7_col8;let input_state_0_limb_8_col9 = memory_id_to_big_value_tmp_51986_1.get_m31(8);
            *row[9] = input_state_0_limb_8_col9;let input_state_0_limb_9_col10 = memory_id_to_big_value_tmp_51986_1.get_m31(9);
            *row[10] = input_state_0_limb_9_col10;let input_state_0_limb_10_col11 = memory_id_to_big_value_tmp_51986_1.get_m31(10);
            *row[11] = input_state_0_limb_10_col11;let input_state_0_limb_11_col12 = memory_id_to_big_value_tmp_51986_1.get_m31(11);
            *row[12] = input_state_0_limb_11_col12;let input_state_0_limb_12_col13 = memory_id_to_big_value_tmp_51986_1.get_m31(12);
            *row[13] = input_state_0_limb_12_col13;let input_state_0_limb_13_col14 = memory_id_to_big_value_tmp_51986_1.get_m31(13);
            *row[14] = input_state_0_limb_13_col14;let input_state_0_limb_14_col15 = memory_id_to_big_value_tmp_51986_1.get_m31(14);
            *row[15] = input_state_0_limb_14_col15;let input_state_0_limb_15_col16 = memory_id_to_big_value_tmp_51986_1.get_m31(15);
            *row[16] = input_state_0_limb_15_col16;let input_state_0_limb_16_col17 = memory_id_to_big_value_tmp_51986_1.get_m31(16);
            *row[17] = input_state_0_limb_16_col17;let input_state_0_limb_17_col18 = memory_id_to_big_value_tmp_51986_1.get_m31(17);
            *row[18] = input_state_0_limb_17_col18;let input_state_0_limb_18_col19 = memory_id_to_big_value_tmp_51986_1.get_m31(18);
            *row[19] = input_state_0_limb_18_col19;let input_state_0_limb_19_col20 = memory_id_to_big_value_tmp_51986_1.get_m31(19);
            *row[20] = input_state_0_limb_19_col20;let input_state_0_limb_20_col21 = memory_id_to_big_value_tmp_51986_1.get_m31(20);
            *row[21] = input_state_0_limb_20_col21;let input_state_0_limb_21_col22 = memory_id_to_big_value_tmp_51986_1.get_m31(21);
            *row[22] = input_state_0_limb_21_col22;let input_state_0_limb_22_col23 = memory_id_to_big_value_tmp_51986_1.get_m31(22);
            *row[23] = input_state_0_limb_22_col23;let input_state_0_limb_23_col24 = memory_id_to_big_value_tmp_51986_1.get_m31(23);
            *row[24] = input_state_0_limb_23_col24;let input_state_0_limb_24_col25 = memory_id_to_big_value_tmp_51986_1.get_m31(24);
            *row[25] = input_state_0_limb_24_col25;let input_state_0_limb_25_col26 = memory_id_to_big_value_tmp_51986_1.get_m31(25);
            *row[26] = input_state_0_limb_25_col26;let input_state_0_limb_26_col27 = memory_id_to_big_value_tmp_51986_1.get_m31(26);
            *row[27] = input_state_0_limb_26_col27;let input_state_0_limb_27_col28 = memory_id_to_big_value_tmp_51986_1.get_m31(27);
            *row[28] = input_state_0_limb_27_col28;*sub_component_inputs.memory_id_to_big[0] =
                input_state_0_id_col0;
            *lookup_data.memory_id_to_big_0 = [input_state_0_id_col0, input_state_0_limb_0_col1, input_state_0_limb_1_col2, input_state_0_limb_2_col3, input_state_0_limb_3_col4, input_state_0_limb_4_col5, input_state_0_limb_5_col6, input_state_0_limb_6_col7, input_state_0_limb_7_col8, input_state_0_limb_8_col9, input_state_0_limb_9_col10, input_state_0_limb_10_col11, input_state_0_limb_11_col12, input_state_0_limb_12_col13, input_state_0_limb_13_col14, input_state_0_limb_14_col15, input_state_0_limb_15_col16, input_state_0_limb_16_col17, input_state_0_limb_17_col18, input_state_0_limb_18_col19, input_state_0_limb_19_col20, input_state_0_limb_20_col21, input_state_0_limb_21_col22, input_state_0_limb_22_col23, input_state_0_limb_23_col24, input_state_0_limb_24_col25, input_state_0_limb_25_col26, input_state_0_limb_26_col27, input_state_0_limb_27_col28];let read_positive_num_bits_252_output_tmp_51986_2 = (PackedFelt252::from_limbs([input_state_0_limb_0_col1, input_state_0_limb_1_col2, input_state_0_limb_2_col3, input_state_0_limb_3_col4, input_state_0_limb_4_col5, input_state_0_limb_5_col6, input_state_0_limb_6_col7, input_state_0_limb_7_col8, input_state_0_limb_8_col9, input_state_0_limb_9_col10, input_state_0_limb_10_col11, input_state_0_limb_11_col12, input_state_0_limb_12_col13, input_state_0_limb_13_col14, input_state_0_limb_14_col15, input_state_0_limb_15_col16, input_state_0_limb_16_col17, input_state_0_limb_17_col18, input_state_0_limb_18_col19, input_state_0_limb_19_col20, input_state_0_limb_20_col21, input_state_0_limb_21_col22, input_state_0_limb_22_col23, input_state_0_limb_23_col24, input_state_0_limb_24_col25, input_state_0_limb_25_col26, input_state_0_limb_26_col27, input_state_0_limb_27_col28]), input_state_0_id_col0);

            let packed_input_state_0_tmp_51986_3 = PackedFelt252Width27::from_limbs([((((input_state_0_limb_0_col1) + (((input_state_0_limb_1_col2) * (M31_512))))) + (((input_state_0_limb_2_col3) * (M31_262144)))), ((((input_state_0_limb_3_col4) + (((input_state_0_limb_4_col5) * (M31_512))))) + (((input_state_0_limb_5_col6) * (M31_262144)))), ((((input_state_0_limb_6_col7) + (((input_state_0_limb_7_col8) * (M31_512))))) + (((input_state_0_limb_8_col9) * (M31_262144)))), ((((input_state_0_limb_9_col10) + (((input_state_0_limb_10_col11) * (M31_512))))) + (((input_state_0_limb_11_col12) * (M31_262144)))), ((((input_state_0_limb_12_col13) + (((input_state_0_limb_13_col14) * (M31_512))))) + (((input_state_0_limb_14_col15) * (M31_262144)))), ((((input_state_0_limb_15_col16) + (((input_state_0_limb_16_col17) * (M31_512))))) + (((input_state_0_limb_17_col18) * (M31_262144)))), ((((input_state_0_limb_18_col19) + (((input_state_0_limb_19_col20) * (M31_512))))) + (((input_state_0_limb_20_col21) * (M31_262144)))), ((((input_state_0_limb_21_col22) + (((input_state_0_limb_22_col23) * (M31_512))))) + (((input_state_0_limb_23_col24) * (M31_262144)))), ((((input_state_0_limb_24_col25) + (((input_state_0_limb_25_col26) * (M31_512))))) + (((input_state_0_limb_26_col27) * (M31_262144)))), input_state_0_limb_27_col28]);

            //Read Positive Num Bits 252.

            let memory_address_to_id_value_tmp_51986_4 = memory_address_to_id_state.deduce_output(((((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start))) + (((seq) * (M31_6))))) + (M31_1)));let memory_id_to_big_value_tmp_51986_5 = memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_51986_4);let input_state_1_id_col29 = memory_address_to_id_value_tmp_51986_4;
            *row[29] = input_state_1_id_col29;*sub_component_inputs.memory_address_to_id[1] =
                ((((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start))) + (((seq) * (M31_6))))) + (M31_1));
            *lookup_data.memory_address_to_id_1 = [((((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start))) + (((seq) * (M31_6))))) + (M31_1)), input_state_1_id_col29];let input_state_1_limb_0_col30 = memory_id_to_big_value_tmp_51986_5.get_m31(0);
            *row[30] = input_state_1_limb_0_col30;let input_state_1_limb_1_col31 = memory_id_to_big_value_tmp_51986_5.get_m31(1);
            *row[31] = input_state_1_limb_1_col31;let input_state_1_limb_2_col32 = memory_id_to_big_value_tmp_51986_5.get_m31(2);
            *row[32] = input_state_1_limb_2_col32;let input_state_1_limb_3_col33 = memory_id_to_big_value_tmp_51986_5.get_m31(3);
            *row[33] = input_state_1_limb_3_col33;let input_state_1_limb_4_col34 = memory_id_to_big_value_tmp_51986_5.get_m31(4);
            *row[34] = input_state_1_limb_4_col34;let input_state_1_limb_5_col35 = memory_id_to_big_value_tmp_51986_5.get_m31(5);
            *row[35] = input_state_1_limb_5_col35;let input_state_1_limb_6_col36 = memory_id_to_big_value_tmp_51986_5.get_m31(6);
            *row[36] = input_state_1_limb_6_col36;let input_state_1_limb_7_col37 = memory_id_to_big_value_tmp_51986_5.get_m31(7);
            *row[37] = input_state_1_limb_7_col37;let input_state_1_limb_8_col38 = memory_id_to_big_value_tmp_51986_5.get_m31(8);
            *row[38] = input_state_1_limb_8_col38;let input_state_1_limb_9_col39 = memory_id_to_big_value_tmp_51986_5.get_m31(9);
            *row[39] = input_state_1_limb_9_col39;let input_state_1_limb_10_col40 = memory_id_to_big_value_tmp_51986_5.get_m31(10);
            *row[40] = input_state_1_limb_10_col40;let input_state_1_limb_11_col41 = memory_id_to_big_value_tmp_51986_5.get_m31(11);
            *row[41] = input_state_1_limb_11_col41;let input_state_1_limb_12_col42 = memory_id_to_big_value_tmp_51986_5.get_m31(12);
            *row[42] = input_state_1_limb_12_col42;let input_state_1_limb_13_col43 = memory_id_to_big_value_tmp_51986_5.get_m31(13);
            *row[43] = input_state_1_limb_13_col43;let input_state_1_limb_14_col44 = memory_id_to_big_value_tmp_51986_5.get_m31(14);
            *row[44] = input_state_1_limb_14_col44;let input_state_1_limb_15_col45 = memory_id_to_big_value_tmp_51986_5.get_m31(15);
            *row[45] = input_state_1_limb_15_col45;let input_state_1_limb_16_col46 = memory_id_to_big_value_tmp_51986_5.get_m31(16);
            *row[46] = input_state_1_limb_16_col46;let input_state_1_limb_17_col47 = memory_id_to_big_value_tmp_51986_5.get_m31(17);
            *row[47] = input_state_1_limb_17_col47;let input_state_1_limb_18_col48 = memory_id_to_big_value_tmp_51986_5.get_m31(18);
            *row[48] = input_state_1_limb_18_col48;let input_state_1_limb_19_col49 = memory_id_to_big_value_tmp_51986_5.get_m31(19);
            *row[49] = input_state_1_limb_19_col49;let input_state_1_limb_20_col50 = memory_id_to_big_value_tmp_51986_5.get_m31(20);
            *row[50] = input_state_1_limb_20_col50;let input_state_1_limb_21_col51 = memory_id_to_big_value_tmp_51986_5.get_m31(21);
            *row[51] = input_state_1_limb_21_col51;let input_state_1_limb_22_col52 = memory_id_to_big_value_tmp_51986_5.get_m31(22);
            *row[52] = input_state_1_limb_22_col52;let input_state_1_limb_23_col53 = memory_id_to_big_value_tmp_51986_5.get_m31(23);
            *row[53] = input_state_1_limb_23_col53;let input_state_1_limb_24_col54 = memory_id_to_big_value_tmp_51986_5.get_m31(24);
            *row[54] = input_state_1_limb_24_col54;let input_state_1_limb_25_col55 = memory_id_to_big_value_tmp_51986_5.get_m31(25);
            *row[55] = input_state_1_limb_25_col55;let input_state_1_limb_26_col56 = memory_id_to_big_value_tmp_51986_5.get_m31(26);
            *row[56] = input_state_1_limb_26_col56;let input_state_1_limb_27_col57 = memory_id_to_big_value_tmp_51986_5.get_m31(27);
            *row[57] = input_state_1_limb_27_col57;*sub_component_inputs.memory_id_to_big[1] =
                input_state_1_id_col29;
            *lookup_data.memory_id_to_big_1 = [input_state_1_id_col29, input_state_1_limb_0_col30, input_state_1_limb_1_col31, input_state_1_limb_2_col32, input_state_1_limb_3_col33, input_state_1_limb_4_col34, input_state_1_limb_5_col35, input_state_1_limb_6_col36, input_state_1_limb_7_col37, input_state_1_limb_8_col38, input_state_1_limb_9_col39, input_state_1_limb_10_col40, input_state_1_limb_11_col41, input_state_1_limb_12_col42, input_state_1_limb_13_col43, input_state_1_limb_14_col44, input_state_1_limb_15_col45, input_state_1_limb_16_col46, input_state_1_limb_17_col47, input_state_1_limb_18_col48, input_state_1_limb_19_col49, input_state_1_limb_20_col50, input_state_1_limb_21_col51, input_state_1_limb_22_col52, input_state_1_limb_23_col53, input_state_1_limb_24_col54, input_state_1_limb_25_col55, input_state_1_limb_26_col56, input_state_1_limb_27_col57];let read_positive_num_bits_252_output_tmp_51986_6 = (PackedFelt252::from_limbs([input_state_1_limb_0_col30, input_state_1_limb_1_col31, input_state_1_limb_2_col32, input_state_1_limb_3_col33, input_state_1_limb_4_col34, input_state_1_limb_5_col35, input_state_1_limb_6_col36, input_state_1_limb_7_col37, input_state_1_limb_8_col38, input_state_1_limb_9_col39, input_state_1_limb_10_col40, input_state_1_limb_11_col41, input_state_1_limb_12_col42, input_state_1_limb_13_col43, input_state_1_limb_14_col44, input_state_1_limb_15_col45, input_state_1_limb_16_col46, input_state_1_limb_17_col47, input_state_1_limb_18_col48, input_state_1_limb_19_col49, input_state_1_limb_20_col50, input_state_1_limb_21_col51, input_state_1_limb_22_col52, input_state_1_limb_23_col53, input_state_1_limb_24_col54, input_state_1_limb_25_col55, input_state_1_limb_26_col56, input_state_1_limb_27_col57]), input_state_1_id_col29);

            let packed_input_state_1_tmp_51986_7 = PackedFelt252Width27::from_limbs([((((input_state_1_limb_0_col30) + (((input_state_1_limb_1_col31) * (M31_512))))) + (((input_state_1_limb_2_col32) * (M31_262144)))), ((((input_state_1_limb_3_col33) + (((input_state_1_limb_4_col34) * (M31_512))))) + (((input_state_1_limb_5_col35) * (M31_262144)))), ((((input_state_1_limb_6_col36) + (((input_state_1_limb_7_col37) * (M31_512))))) + (((input_state_1_limb_8_col38) * (M31_262144)))), ((((input_state_1_limb_9_col39) + (((input_state_1_limb_10_col40) * (M31_512))))) + (((input_state_1_limb_11_col41) * (M31_262144)))), ((((input_state_1_limb_12_col42) + (((input_state_1_limb_13_col43) * (M31_512))))) + (((input_state_1_limb_14_col44) * (M31_262144)))), ((((input_state_1_limb_15_col45) + (((input_state_1_limb_16_col46) * (M31_512))))) + (((input_state_1_limb_17_col47) * (M31_262144)))), ((((input_state_1_limb_18_col48) + (((input_state_1_limb_19_col49) * (M31_512))))) + (((input_state_1_limb_20_col50) * (M31_262144)))), ((((input_state_1_limb_21_col51) + (((input_state_1_limb_22_col52) * (M31_512))))) + (((input_state_1_limb_23_col53) * (M31_262144)))), ((((input_state_1_limb_24_col54) + (((input_state_1_limb_25_col55) * (M31_512))))) + (((input_state_1_limb_26_col56) * (M31_262144)))), input_state_1_limb_27_col57]);

            //Read Positive Num Bits 252.

            let memory_address_to_id_value_tmp_51986_8 = memory_address_to_id_state.deduce_output(((((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start))) + (((seq) * (M31_6))))) + (M31_2)));let memory_id_to_big_value_tmp_51986_9 = memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_51986_8);let input_state_2_id_col58 = memory_address_to_id_value_tmp_51986_8;
            *row[58] = input_state_2_id_col58;*sub_component_inputs.memory_address_to_id[2] =
                ((((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start))) + (((seq) * (M31_6))))) + (M31_2));
            *lookup_data.memory_address_to_id_2 = [((((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start))) + (((seq) * (M31_6))))) + (M31_2)), input_state_2_id_col58];let input_state_2_limb_0_col59 = memory_id_to_big_value_tmp_51986_9.get_m31(0);
            *row[59] = input_state_2_limb_0_col59;let input_state_2_limb_1_col60 = memory_id_to_big_value_tmp_51986_9.get_m31(1);
            *row[60] = input_state_2_limb_1_col60;let input_state_2_limb_2_col61 = memory_id_to_big_value_tmp_51986_9.get_m31(2);
            *row[61] = input_state_2_limb_2_col61;let input_state_2_limb_3_col62 = memory_id_to_big_value_tmp_51986_9.get_m31(3);
            *row[62] = input_state_2_limb_3_col62;let input_state_2_limb_4_col63 = memory_id_to_big_value_tmp_51986_9.get_m31(4);
            *row[63] = input_state_2_limb_4_col63;let input_state_2_limb_5_col64 = memory_id_to_big_value_tmp_51986_9.get_m31(5);
            *row[64] = input_state_2_limb_5_col64;let input_state_2_limb_6_col65 = memory_id_to_big_value_tmp_51986_9.get_m31(6);
            *row[65] = input_state_2_limb_6_col65;let input_state_2_limb_7_col66 = memory_id_to_big_value_tmp_51986_9.get_m31(7);
            *row[66] = input_state_2_limb_7_col66;let input_state_2_limb_8_col67 = memory_id_to_big_value_tmp_51986_9.get_m31(8);
            *row[67] = input_state_2_limb_8_col67;let input_state_2_limb_9_col68 = memory_id_to_big_value_tmp_51986_9.get_m31(9);
            *row[68] = input_state_2_limb_9_col68;let input_state_2_limb_10_col69 = memory_id_to_big_value_tmp_51986_9.get_m31(10);
            *row[69] = input_state_2_limb_10_col69;let input_state_2_limb_11_col70 = memory_id_to_big_value_tmp_51986_9.get_m31(11);
            *row[70] = input_state_2_limb_11_col70;let input_state_2_limb_12_col71 = memory_id_to_big_value_tmp_51986_9.get_m31(12);
            *row[71] = input_state_2_limb_12_col71;let input_state_2_limb_13_col72 = memory_id_to_big_value_tmp_51986_9.get_m31(13);
            *row[72] = input_state_2_limb_13_col72;let input_state_2_limb_14_col73 = memory_id_to_big_value_tmp_51986_9.get_m31(14);
            *row[73] = input_state_2_limb_14_col73;let input_state_2_limb_15_col74 = memory_id_to_big_value_tmp_51986_9.get_m31(15);
            *row[74] = input_state_2_limb_15_col74;let input_state_2_limb_16_col75 = memory_id_to_big_value_tmp_51986_9.get_m31(16);
            *row[75] = input_state_2_limb_16_col75;let input_state_2_limb_17_col76 = memory_id_to_big_value_tmp_51986_9.get_m31(17);
            *row[76] = input_state_2_limb_17_col76;let input_state_2_limb_18_col77 = memory_id_to_big_value_tmp_51986_9.get_m31(18);
            *row[77] = input_state_2_limb_18_col77;let input_state_2_limb_19_col78 = memory_id_to_big_value_tmp_51986_9.get_m31(19);
            *row[78] = input_state_2_limb_19_col78;let input_state_2_limb_20_col79 = memory_id_to_big_value_tmp_51986_9.get_m31(20);
            *row[79] = input_state_2_limb_20_col79;let input_state_2_limb_21_col80 = memory_id_to_big_value_tmp_51986_9.get_m31(21);
            *row[80] = input_state_2_limb_21_col80;let input_state_2_limb_22_col81 = memory_id_to_big_value_tmp_51986_9.get_m31(22);
            *row[81] = input_state_2_limb_22_col81;let input_state_2_limb_23_col82 = memory_id_to_big_value_tmp_51986_9.get_m31(23);
            *row[82] = input_state_2_limb_23_col82;let input_state_2_limb_24_col83 = memory_id_to_big_value_tmp_51986_9.get_m31(24);
            *row[83] = input_state_2_limb_24_col83;let input_state_2_limb_25_col84 = memory_id_to_big_value_tmp_51986_9.get_m31(25);
            *row[84] = input_state_2_limb_25_col84;let input_state_2_limb_26_col85 = memory_id_to_big_value_tmp_51986_9.get_m31(26);
            *row[85] = input_state_2_limb_26_col85;let input_state_2_limb_27_col86 = memory_id_to_big_value_tmp_51986_9.get_m31(27);
            *row[86] = input_state_2_limb_27_col86;*sub_component_inputs.memory_id_to_big[2] =
                input_state_2_id_col58;
            *lookup_data.memory_id_to_big_2 = [input_state_2_id_col58, input_state_2_limb_0_col59, input_state_2_limb_1_col60, input_state_2_limb_2_col61, input_state_2_limb_3_col62, input_state_2_limb_4_col63, input_state_2_limb_5_col64, input_state_2_limb_6_col65, input_state_2_limb_7_col66, input_state_2_limb_8_col67, input_state_2_limb_9_col68, input_state_2_limb_10_col69, input_state_2_limb_11_col70, input_state_2_limb_12_col71, input_state_2_limb_13_col72, input_state_2_limb_14_col73, input_state_2_limb_15_col74, input_state_2_limb_16_col75, input_state_2_limb_17_col76, input_state_2_limb_18_col77, input_state_2_limb_19_col78, input_state_2_limb_20_col79, input_state_2_limb_21_col80, input_state_2_limb_22_col81, input_state_2_limb_23_col82, input_state_2_limb_24_col83, input_state_2_limb_25_col84, input_state_2_limb_26_col85, input_state_2_limb_27_col86];let read_positive_num_bits_252_output_tmp_51986_10 = (PackedFelt252::from_limbs([input_state_2_limb_0_col59, input_state_2_limb_1_col60, input_state_2_limb_2_col61, input_state_2_limb_3_col62, input_state_2_limb_4_col63, input_state_2_limb_5_col64, input_state_2_limb_6_col65, input_state_2_limb_7_col66, input_state_2_limb_8_col67, input_state_2_limb_9_col68, input_state_2_limb_10_col69, input_state_2_limb_11_col70, input_state_2_limb_12_col71, input_state_2_limb_13_col72, input_state_2_limb_14_col73, input_state_2_limb_15_col74, input_state_2_limb_16_col75, input_state_2_limb_17_col76, input_state_2_limb_18_col77, input_state_2_limb_19_col78, input_state_2_limb_20_col79, input_state_2_limb_21_col80, input_state_2_limb_22_col81, input_state_2_limb_23_col82, input_state_2_limb_24_col83, input_state_2_limb_25_col84, input_state_2_limb_26_col85, input_state_2_limb_27_col86]), input_state_2_id_col58);

            let packed_input_state_2_tmp_51986_11 = PackedFelt252Width27::from_limbs([((((input_state_2_limb_0_col59) + (((input_state_2_limb_1_col60) * (M31_512))))) + (((input_state_2_limb_2_col61) * (M31_262144)))), ((((input_state_2_limb_3_col62) + (((input_state_2_limb_4_col63) * (M31_512))))) + (((input_state_2_limb_5_col64) * (M31_262144)))), ((((input_state_2_limb_6_col65) + (((input_state_2_limb_7_col66) * (M31_512))))) + (((input_state_2_limb_8_col67) * (M31_262144)))), ((((input_state_2_limb_9_col68) + (((input_state_2_limb_10_col69) * (M31_512))))) + (((input_state_2_limb_11_col70) * (M31_262144)))), ((((input_state_2_limb_12_col71) + (((input_state_2_limb_13_col72) * (M31_512))))) + (((input_state_2_limb_14_col73) * (M31_262144)))), ((((input_state_2_limb_15_col74) + (((input_state_2_limb_16_col75) * (M31_512))))) + (((input_state_2_limb_17_col76) * (M31_262144)))), ((((input_state_2_limb_18_col77) + (((input_state_2_limb_19_col78) * (M31_512))))) + (((input_state_2_limb_20_col79) * (M31_262144)))), ((((input_state_2_limb_21_col80) + (((input_state_2_limb_22_col81) * (M31_512))))) + (((input_state_2_limb_23_col82) * (M31_262144)))), ((((input_state_2_limb_24_col83) + (((input_state_2_limb_25_col84) * (M31_512))))) + (((input_state_2_limb_26_col85) * (M31_262144)))), input_state_2_limb_27_col86]);

            //Poseidon Hades Permutation.

            //Linear Combination N 2 Coefs 1 1.

            let combination_tmp_51986_12 = PackedFelt252Width27::from_packed_felt252(((((Felt252_0_0_0_0) + (((Felt252_1_0_0_0) * (PackedFelt252::from_packed_felt252width27(packed_input_state_0_tmp_51986_3)))))) + (Felt252_9275160746813554287_16541205595039575623_4169650429605064889_470088886057789987)));let combination_limb_0_col87 = combination_tmp_51986_12.get_m31(0);
            *row[87] = combination_limb_0_col87;let combination_limb_1_col88 = combination_tmp_51986_12.get_m31(1);
            *row[88] = combination_limb_1_col88;let combination_limb_2_col89 = combination_tmp_51986_12.get_m31(2);
            *row[89] = combination_limb_2_col89;let combination_limb_3_col90 = combination_tmp_51986_12.get_m31(3);
            *row[90] = combination_limb_3_col90;let combination_limb_4_col91 = combination_tmp_51986_12.get_m31(4);
            *row[91] = combination_limb_4_col91;let combination_limb_5_col92 = combination_tmp_51986_12.get_m31(5);
            *row[92] = combination_limb_5_col92;let combination_limb_6_col93 = combination_tmp_51986_12.get_m31(6);
            *row[93] = combination_limb_6_col93;let combination_limb_7_col94 = combination_tmp_51986_12.get_m31(7);
            *row[94] = combination_limb_7_col94;let combination_limb_8_col95 = combination_tmp_51986_12.get_m31(8);
            *row[95] = combination_limb_8_col95;let combination_limb_9_col96 = combination_tmp_51986_12.get_m31(9);
            *row[96] = combination_limb_9_col96;let biased_limb_accumulator_u32_tmp_51986_13 = PackedUInt32::from_m31(((((((packed_input_state_0_tmp_51986_3.get_m31(0)) + (M31_74972783))) - (combination_limb_0_col87))) + (M31_134217729)));let p_coef_col97 = ((biased_limb_accumulator_u32_tmp_51986_13.low().as_m31()) - (M31_1));
            *row[97] = p_coef_col97;let carry_0_tmp_51986_14 = ((((((((packed_input_state_0_tmp_51986_3.get_m31(0)) + (M31_74972783))) - (combination_limb_0_col87))) - (p_coef_col97))) * (M31_16));let carry_1_tmp_51986_15 = ((((((((carry_0_tmp_51986_14) + (packed_input_state_0_tmp_51986_3.get_m31(1)))) + (M31_117420501))) - (combination_limb_1_col88))) * (M31_16));let carry_2_tmp_51986_16 = ((((((((carry_1_tmp_51986_15) + (packed_input_state_0_tmp_51986_3.get_m31(2)))) + (M31_112795138))) - (combination_limb_2_col89))) * (M31_16));let carry_3_tmp_51986_17 = ((((((((carry_2_tmp_51986_16) + (packed_input_state_0_tmp_51986_3.get_m31(3)))) + (M31_91013252))) - (combination_limb_3_col90))) * (M31_16));let carry_4_tmp_51986_18 = ((((((((carry_3_tmp_51986_17) + (packed_input_state_0_tmp_51986_3.get_m31(4)))) + (M31_60709090))) - (combination_limb_4_col91))) * (M31_16));let carry_5_tmp_51986_19 = ((((((((carry_4_tmp_51986_18) + (packed_input_state_0_tmp_51986_3.get_m31(5)))) + (M31_44848225))) - (combination_limb_5_col92))) * (M31_16));let carry_6_tmp_51986_20 = ((((((((carry_5_tmp_51986_19) + (packed_input_state_0_tmp_51986_3.get_m31(6)))) + (M31_108487870))) - (combination_limb_6_col93))) * (M31_16));let carry_7_tmp_51986_21 = ((((((((((carry_6_tmp_51986_20) + (packed_input_state_0_tmp_51986_3.get_m31(7)))) + (M31_44781849))) - (combination_limb_7_col94))) - (((p_coef_col97) * (M31_136))))) * (M31_16));let carry_8_tmp_51986_22 = ((((((((carry_7_tmp_51986_21) + (packed_input_state_0_tmp_51986_3.get_m31(8)))) + (M31_102193642))) - (combination_limb_8_col95))) * (M31_16));let linear_combination_n_2_coefs_1_1_output_tmp_51986_33 = combination_tmp_51986_12;

            //Linear Combination N 2 Coefs 1 1.

            let combination_tmp_51986_34 = PackedFelt252Width27::from_packed_felt252(((((Felt252_0_0_0_0) + (((Felt252_1_0_0_0) * (PackedFelt252::from_packed_felt252width27(packed_input_state_1_tmp_51986_7)))))) + (Felt252_16477292399064058052_4441744911417641572_18431044672185975386_252894828082060025)));let combination_limb_0_col98 = combination_tmp_51986_34.get_m31(0);
            *row[98] = combination_limb_0_col98;let combination_limb_1_col99 = combination_tmp_51986_34.get_m31(1);
            *row[99] = combination_limb_1_col99;let combination_limb_2_col100 = combination_tmp_51986_34.get_m31(2);
            *row[100] = combination_limb_2_col100;let combination_limb_3_col101 = combination_tmp_51986_34.get_m31(3);
            *row[101] = combination_limb_3_col101;let combination_limb_4_col102 = combination_tmp_51986_34.get_m31(4);
            *row[102] = combination_limb_4_col102;let combination_limb_5_col103 = combination_tmp_51986_34.get_m31(5);
            *row[103] = combination_limb_5_col103;let combination_limb_6_col104 = combination_tmp_51986_34.get_m31(6);
            *row[104] = combination_limb_6_col104;let combination_limb_7_col105 = combination_tmp_51986_34.get_m31(7);
            *row[105] = combination_limb_7_col105;let combination_limb_8_col106 = combination_tmp_51986_34.get_m31(8);
            *row[106] = combination_limb_8_col106;let combination_limb_9_col107 = combination_tmp_51986_34.get_m31(9);
            *row[107] = combination_limb_9_col107;let biased_limb_accumulator_u32_tmp_51986_35 = PackedUInt32::from_m31(((((((packed_input_state_1_tmp_51986_7.get_m31(0)) + (M31_41224388))) - (combination_limb_0_col98))) + (M31_134217729)));let p_coef_col108 = ((biased_limb_accumulator_u32_tmp_51986_35.low().as_m31()) - (M31_1));
            *row[108] = p_coef_col108;let carry_0_tmp_51986_36 = ((((((((packed_input_state_1_tmp_51986_7.get_m31(0)) + (M31_41224388))) - (combination_limb_0_col98))) - (p_coef_col108))) * (M31_16));let carry_1_tmp_51986_37 = ((((((((carry_0_tmp_51986_36) + (packed_input_state_1_tmp_51986_7.get_m31(1)))) + (M31_90391646))) - (combination_limb_1_col99))) * (M31_16));let carry_2_tmp_51986_38 = ((((((((carry_1_tmp_51986_37) + (packed_input_state_1_tmp_51986_7.get_m31(2)))) + (M31_36279186))) - (combination_limb_2_col100))) * (M31_16));let carry_3_tmp_51986_39 = ((((((((carry_2_tmp_51986_38) + (packed_input_state_1_tmp_51986_7.get_m31(3)))) + (M31_129717753))) - (combination_limb_3_col101))) * (M31_16));let carry_4_tmp_51986_40 = ((((((((carry_3_tmp_51986_39) + (packed_input_state_1_tmp_51986_7.get_m31(4)))) + (M31_94624323))) - (combination_limb_4_col102))) * (M31_16));let carry_5_tmp_51986_41 = ((((((((carry_4_tmp_51986_40) + (packed_input_state_1_tmp_51986_7.get_m31(5)))) + (M31_75104388))) - (combination_limb_5_col103))) * (M31_16));let carry_6_tmp_51986_42 = ((((((((carry_5_tmp_51986_41) + (packed_input_state_1_tmp_51986_7.get_m31(6)))) + (M31_133303902))) - (combination_limb_6_col104))) * (M31_16));let carry_7_tmp_51986_43 = ((((((((((carry_6_tmp_51986_42) + (packed_input_state_1_tmp_51986_7.get_m31(7)))) + (M31_48945103))) - (combination_limb_7_col105))) - (((p_coef_col108) * (M31_136))))) * (M31_16));let carry_8_tmp_51986_44 = ((((((((carry_7_tmp_51986_43) + (packed_input_state_1_tmp_51986_7.get_m31(8)))) + (M31_41320857))) - (combination_limb_8_col106))) * (M31_16));let linear_combination_n_2_coefs_1_1_output_tmp_51986_55 = combination_tmp_51986_34;

            //Linear Combination N 2 Coefs 1 1.

            let combination_tmp_51986_56 = PackedFelt252Width27::from_packed_felt252(((((Felt252_0_0_0_0) + (((Felt252_1_0_0_0) * (PackedFelt252::from_packed_felt252width27(packed_input_state_2_tmp_51986_11)))))) + (Felt252_8794894655201903369_3219077422080798056_16714934791572408267_262217499501479120)));let combination_limb_0_col109 = combination_tmp_51986_56.get_m31(0);
            *row[109] = combination_limb_0_col109;let combination_limb_1_col110 = combination_tmp_51986_56.get_m31(1);
            *row[110] = combination_limb_1_col110;let combination_limb_2_col111 = combination_tmp_51986_56.get_m31(2);
            *row[111] = combination_limb_2_col111;let combination_limb_3_col112 = combination_tmp_51986_56.get_m31(3);
            *row[112] = combination_limb_3_col112;let combination_limb_4_col113 = combination_tmp_51986_56.get_m31(4);
            *row[113] = combination_limb_4_col113;let combination_limb_5_col114 = combination_tmp_51986_56.get_m31(5);
            *row[114] = combination_limb_5_col114;let combination_limb_6_col115 = combination_tmp_51986_56.get_m31(6);
            *row[115] = combination_limb_6_col115;let combination_limb_7_col116 = combination_tmp_51986_56.get_m31(7);
            *row[116] = combination_limb_7_col116;let combination_limb_8_col117 = combination_tmp_51986_56.get_m31(8);
            *row[117] = combination_limb_8_col117;let combination_limb_9_col118 = combination_tmp_51986_56.get_m31(9);
            *row[118] = combination_limb_9_col118;let biased_limb_accumulator_u32_tmp_51986_57 = PackedUInt32::from_m31(((((((packed_input_state_2_tmp_51986_11.get_m31(0)) + (M31_4883209))) - (combination_limb_0_col109))) + (M31_134217729)));let p_coef_col119 = ((biased_limb_accumulator_u32_tmp_51986_57.low().as_m31()) - (M31_1));
            *row[119] = p_coef_col119;let carry_0_tmp_51986_58 = ((((((((packed_input_state_2_tmp_51986_11.get_m31(0)) + (M31_4883209))) - (combination_limb_0_col109))) - (p_coef_col119))) * (M31_16));let carry_1_tmp_51986_59 = ((((((((carry_0_tmp_51986_58) + (packed_input_state_2_tmp_51986_11.get_m31(1)))) + (M31_28820206))) - (combination_limb_1_col110))) * (M31_16));let carry_2_tmp_51986_60 = ((((((((carry_1_tmp_51986_59) + (packed_input_state_2_tmp_51986_11.get_m31(2)))) + (M31_79012328))) - (combination_limb_2_col111))) * (M31_16));let carry_3_tmp_51986_61 = ((((((((carry_2_tmp_51986_60) + (packed_input_state_2_tmp_51986_11.get_m31(3)))) + (M31_49157069))) - (combination_limb_3_col112))) * (M31_16));let carry_4_tmp_51986_62 = ((((((((carry_3_tmp_51986_61) + (packed_input_state_2_tmp_51986_11.get_m31(4)))) + (M31_78826183))) - (combination_limb_4_col113))) * (M31_16));let carry_5_tmp_51986_63 = ((((((((carry_4_tmp_51986_62) + (packed_input_state_2_tmp_51986_11.get_m31(5)))) + (M31_72285071))) - (combination_limb_5_col114))) * (M31_16));let carry_6_tmp_51986_64 = ((((((((carry_5_tmp_51986_63) + (packed_input_state_2_tmp_51986_11.get_m31(6)))) + (M31_33413160))) - (combination_limb_6_col115))) * (M31_16));let carry_7_tmp_51986_65 = ((((((((((carry_6_tmp_51986_64) + (packed_input_state_2_tmp_51986_11.get_m31(7)))) + (M31_90842759))) - (combination_limb_7_col116))) - (((p_coef_col119) * (M31_136))))) * (M31_16));let carry_8_tmp_51986_66 = ((((((((carry_7_tmp_51986_65) + (packed_input_state_2_tmp_51986_11.get_m31(8)))) + (M31_60124463))) - (combination_limb_8_col117))) * (M31_16));let linear_combination_n_2_coefs_1_1_output_tmp_51986_77 = combination_tmp_51986_56;

            *lookup_data.poseidon_full_round_chain_0 = [((seq) * (M31_2)), M31_0, combination_limb_0_col87, combination_limb_1_col88, combination_limb_2_col89, combination_limb_3_col90, combination_limb_4_col91, combination_limb_5_col92, combination_limb_6_col93, combination_limb_7_col94, combination_limb_8_col95, combination_limb_9_col96, combination_limb_0_col98, combination_limb_1_col99, combination_limb_2_col100, combination_limb_3_col101, combination_limb_4_col102, combination_limb_5_col103, combination_limb_6_col104, combination_limb_7_col105, combination_limb_8_col106, combination_limb_9_col107, combination_limb_0_col109, combination_limb_1_col110, combination_limb_2_col111, combination_limb_3_col112, combination_limb_4_col113, combination_limb_5_col114, combination_limb_6_col115, combination_limb_7_col116, combination_limb_8_col117, combination_limb_9_col118];*sub_component_inputs.poseidon_full_round_chain[0] =
                (((seq) * (M31_2)), M31_0, [linear_combination_n_2_coefs_1_1_output_tmp_51986_33, linear_combination_n_2_coefs_1_1_output_tmp_51986_55, linear_combination_n_2_coefs_1_1_output_tmp_51986_77]);
            let poseidon_full_round_chain_output_round_0_tmp_51986_78 = PackedPoseidonFullRoundChain::deduce_output((((seq) * (M31_2)), M31_0, [linear_combination_n_2_coefs_1_1_output_tmp_51986_33, linear_combination_n_2_coefs_1_1_output_tmp_51986_55, linear_combination_n_2_coefs_1_1_output_tmp_51986_77]));*sub_component_inputs.poseidon_full_round_chain[1] =
                (((seq) * (M31_2)), M31_1, [poseidon_full_round_chain_output_round_0_tmp_51986_78.2[0], poseidon_full_round_chain_output_round_0_tmp_51986_78.2[1], poseidon_full_round_chain_output_round_0_tmp_51986_78.2[2]]);
            let poseidon_full_round_chain_output_round_1_tmp_51986_79 = PackedPoseidonFullRoundChain::deduce_output((((seq) * (M31_2)), M31_1, [poseidon_full_round_chain_output_round_0_tmp_51986_78.2[0], poseidon_full_round_chain_output_round_0_tmp_51986_78.2[1], poseidon_full_round_chain_output_round_0_tmp_51986_78.2[2]]));*sub_component_inputs.poseidon_full_round_chain[2] =
                (((seq) * (M31_2)), M31_2, [poseidon_full_round_chain_output_round_1_tmp_51986_79.2[0], poseidon_full_round_chain_output_round_1_tmp_51986_79.2[1], poseidon_full_round_chain_output_round_1_tmp_51986_79.2[2]]);
            let poseidon_full_round_chain_output_round_2_tmp_51986_80 = PackedPoseidonFullRoundChain::deduce_output((((seq) * (M31_2)), M31_2, [poseidon_full_round_chain_output_round_1_tmp_51986_79.2[0], poseidon_full_round_chain_output_round_1_tmp_51986_79.2[1], poseidon_full_round_chain_output_round_1_tmp_51986_79.2[2]]));*sub_component_inputs.poseidon_full_round_chain[3] =
                (((seq) * (M31_2)), M31_3, [poseidon_full_round_chain_output_round_2_tmp_51986_80.2[0], poseidon_full_round_chain_output_round_2_tmp_51986_80.2[1], poseidon_full_round_chain_output_round_2_tmp_51986_80.2[2]]);
            let poseidon_full_round_chain_output_round_3_tmp_51986_81 = PackedPoseidonFullRoundChain::deduce_output((((seq) * (M31_2)), M31_3, [poseidon_full_round_chain_output_round_2_tmp_51986_80.2[0], poseidon_full_round_chain_output_round_2_tmp_51986_80.2[1], poseidon_full_round_chain_output_round_2_tmp_51986_80.2[2]]));let poseidon_full_round_chain_output_limb_0_col120 = poseidon_full_round_chain_output_round_3_tmp_51986_81.2[0].get_m31(0);
            *row[120] = poseidon_full_round_chain_output_limb_0_col120;let poseidon_full_round_chain_output_limb_1_col121 = poseidon_full_round_chain_output_round_3_tmp_51986_81.2[0].get_m31(1);
            *row[121] = poseidon_full_round_chain_output_limb_1_col121;let poseidon_full_round_chain_output_limb_2_col122 = poseidon_full_round_chain_output_round_3_tmp_51986_81.2[0].get_m31(2);
            *row[122] = poseidon_full_round_chain_output_limb_2_col122;let poseidon_full_round_chain_output_limb_3_col123 = poseidon_full_round_chain_output_round_3_tmp_51986_81.2[0].get_m31(3);
            *row[123] = poseidon_full_round_chain_output_limb_3_col123;let poseidon_full_round_chain_output_limb_4_col124 = poseidon_full_round_chain_output_round_3_tmp_51986_81.2[0].get_m31(4);
            *row[124] = poseidon_full_round_chain_output_limb_4_col124;let poseidon_full_round_chain_output_limb_5_col125 = poseidon_full_round_chain_output_round_3_tmp_51986_81.2[0].get_m31(5);
            *row[125] = poseidon_full_round_chain_output_limb_5_col125;let poseidon_full_round_chain_output_limb_6_col126 = poseidon_full_round_chain_output_round_3_tmp_51986_81.2[0].get_m31(6);
            *row[126] = poseidon_full_round_chain_output_limb_6_col126;let poseidon_full_round_chain_output_limb_7_col127 = poseidon_full_round_chain_output_round_3_tmp_51986_81.2[0].get_m31(7);
            *row[127] = poseidon_full_round_chain_output_limb_7_col127;let poseidon_full_round_chain_output_limb_8_col128 = poseidon_full_round_chain_output_round_3_tmp_51986_81.2[0].get_m31(8);
            *row[128] = poseidon_full_round_chain_output_limb_8_col128;let poseidon_full_round_chain_output_limb_9_col129 = poseidon_full_round_chain_output_round_3_tmp_51986_81.2[0].get_m31(9);
            *row[129] = poseidon_full_round_chain_output_limb_9_col129;let poseidon_full_round_chain_output_limb_10_col130 = poseidon_full_round_chain_output_round_3_tmp_51986_81.2[1].get_m31(0);
            *row[130] = poseidon_full_round_chain_output_limb_10_col130;let poseidon_full_round_chain_output_limb_11_col131 = poseidon_full_round_chain_output_round_3_tmp_51986_81.2[1].get_m31(1);
            *row[131] = poseidon_full_round_chain_output_limb_11_col131;let poseidon_full_round_chain_output_limb_12_col132 = poseidon_full_round_chain_output_round_3_tmp_51986_81.2[1].get_m31(2);
            *row[132] = poseidon_full_round_chain_output_limb_12_col132;let poseidon_full_round_chain_output_limb_13_col133 = poseidon_full_round_chain_output_round_3_tmp_51986_81.2[1].get_m31(3);
            *row[133] = poseidon_full_round_chain_output_limb_13_col133;let poseidon_full_round_chain_output_limb_14_col134 = poseidon_full_round_chain_output_round_3_tmp_51986_81.2[1].get_m31(4);
            *row[134] = poseidon_full_round_chain_output_limb_14_col134;let poseidon_full_round_chain_output_limb_15_col135 = poseidon_full_round_chain_output_round_3_tmp_51986_81.2[1].get_m31(5);
            *row[135] = poseidon_full_round_chain_output_limb_15_col135;let poseidon_full_round_chain_output_limb_16_col136 = poseidon_full_round_chain_output_round_3_tmp_51986_81.2[1].get_m31(6);
            *row[136] = poseidon_full_round_chain_output_limb_16_col136;let poseidon_full_round_chain_output_limb_17_col137 = poseidon_full_round_chain_output_round_3_tmp_51986_81.2[1].get_m31(7);
            *row[137] = poseidon_full_round_chain_output_limb_17_col137;let poseidon_full_round_chain_output_limb_18_col138 = poseidon_full_round_chain_output_round_3_tmp_51986_81.2[1].get_m31(8);
            *row[138] = poseidon_full_round_chain_output_limb_18_col138;let poseidon_full_round_chain_output_limb_19_col139 = poseidon_full_round_chain_output_round_3_tmp_51986_81.2[1].get_m31(9);
            *row[139] = poseidon_full_round_chain_output_limb_19_col139;let poseidon_full_round_chain_output_limb_20_col140 = poseidon_full_round_chain_output_round_3_tmp_51986_81.2[2].get_m31(0);
            *row[140] = poseidon_full_round_chain_output_limb_20_col140;let poseidon_full_round_chain_output_limb_21_col141 = poseidon_full_round_chain_output_round_3_tmp_51986_81.2[2].get_m31(1);
            *row[141] = poseidon_full_round_chain_output_limb_21_col141;let poseidon_full_round_chain_output_limb_22_col142 = poseidon_full_round_chain_output_round_3_tmp_51986_81.2[2].get_m31(2);
            *row[142] = poseidon_full_round_chain_output_limb_22_col142;let poseidon_full_round_chain_output_limb_23_col143 = poseidon_full_round_chain_output_round_3_tmp_51986_81.2[2].get_m31(3);
            *row[143] = poseidon_full_round_chain_output_limb_23_col143;let poseidon_full_round_chain_output_limb_24_col144 = poseidon_full_round_chain_output_round_3_tmp_51986_81.2[2].get_m31(4);
            *row[144] = poseidon_full_round_chain_output_limb_24_col144;let poseidon_full_round_chain_output_limb_25_col145 = poseidon_full_round_chain_output_round_3_tmp_51986_81.2[2].get_m31(5);
            *row[145] = poseidon_full_round_chain_output_limb_25_col145;let poseidon_full_round_chain_output_limb_26_col146 = poseidon_full_round_chain_output_round_3_tmp_51986_81.2[2].get_m31(6);
            *row[146] = poseidon_full_round_chain_output_limb_26_col146;let poseidon_full_round_chain_output_limb_27_col147 = poseidon_full_round_chain_output_round_3_tmp_51986_81.2[2].get_m31(7);
            *row[147] = poseidon_full_round_chain_output_limb_27_col147;let poseidon_full_round_chain_output_limb_28_col148 = poseidon_full_round_chain_output_round_3_tmp_51986_81.2[2].get_m31(8);
            *row[148] = poseidon_full_round_chain_output_limb_28_col148;let poseidon_full_round_chain_output_limb_29_col149 = poseidon_full_round_chain_output_round_3_tmp_51986_81.2[2].get_m31(9);
            *row[149] = poseidon_full_round_chain_output_limb_29_col149;*lookup_data.poseidon_full_round_chain_1 = [((seq) * (M31_2)), M31_4, poseidon_full_round_chain_output_limb_0_col120, poseidon_full_round_chain_output_limb_1_col121, poseidon_full_round_chain_output_limb_2_col122, poseidon_full_round_chain_output_limb_3_col123, poseidon_full_round_chain_output_limb_4_col124, poseidon_full_round_chain_output_limb_5_col125, poseidon_full_round_chain_output_limb_6_col126, poseidon_full_round_chain_output_limb_7_col127, poseidon_full_round_chain_output_limb_8_col128, poseidon_full_round_chain_output_limb_9_col129, poseidon_full_round_chain_output_limb_10_col130, poseidon_full_round_chain_output_limb_11_col131, poseidon_full_round_chain_output_limb_12_col132, poseidon_full_round_chain_output_limb_13_col133, poseidon_full_round_chain_output_limb_14_col134, poseidon_full_round_chain_output_limb_15_col135, poseidon_full_round_chain_output_limb_16_col136, poseidon_full_round_chain_output_limb_17_col137, poseidon_full_round_chain_output_limb_18_col138, poseidon_full_round_chain_output_limb_19_col139, poseidon_full_round_chain_output_limb_20_col140, poseidon_full_round_chain_output_limb_21_col141, poseidon_full_round_chain_output_limb_22_col142, poseidon_full_round_chain_output_limb_23_col143, poseidon_full_round_chain_output_limb_24_col144, poseidon_full_round_chain_output_limb_25_col145, poseidon_full_round_chain_output_limb_26_col146, poseidon_full_round_chain_output_limb_27_col147, poseidon_full_round_chain_output_limb_28_col148, poseidon_full_round_chain_output_limb_29_col149];*sub_component_inputs.range_check_felt_252_width_27[0] =
                poseidon_full_round_chain_output_round_3_tmp_51986_81.2[0];
            *lookup_data.range_check_felt_252_width_27_0 = [poseidon_full_round_chain_output_limb_0_col120, poseidon_full_round_chain_output_limb_1_col121, poseidon_full_round_chain_output_limb_2_col122, poseidon_full_round_chain_output_limb_3_col123, poseidon_full_round_chain_output_limb_4_col124, poseidon_full_round_chain_output_limb_5_col125, poseidon_full_round_chain_output_limb_6_col126, poseidon_full_round_chain_output_limb_7_col127, poseidon_full_round_chain_output_limb_8_col128, poseidon_full_round_chain_output_limb_9_col129];*sub_component_inputs.range_check_felt_252_width_27[1] =
                poseidon_full_round_chain_output_round_3_tmp_51986_81.2[1];
            *lookup_data.range_check_felt_252_width_27_1 = [poseidon_full_round_chain_output_limb_10_col130, poseidon_full_round_chain_output_limb_11_col131, poseidon_full_round_chain_output_limb_12_col132, poseidon_full_round_chain_output_limb_13_col133, poseidon_full_round_chain_output_limb_14_col134, poseidon_full_round_chain_output_limb_15_col135, poseidon_full_round_chain_output_limb_16_col136, poseidon_full_round_chain_output_limb_17_col137, poseidon_full_round_chain_output_limb_18_col138, poseidon_full_round_chain_output_limb_19_col139];*sub_component_inputs.cube_252[0] =
                poseidon_full_round_chain_output_round_3_tmp_51986_81.2[2];
            let cube_252_output_tmp_51986_82 = PackedCube252::deduce_output(poseidon_full_round_chain_output_round_3_tmp_51986_81.2[2]);let cube_252_output_limb_0_col150 = cube_252_output_tmp_51986_82.get_m31(0);
            *row[150] = cube_252_output_limb_0_col150;let cube_252_output_limb_1_col151 = cube_252_output_tmp_51986_82.get_m31(1);
            *row[151] = cube_252_output_limb_1_col151;let cube_252_output_limb_2_col152 = cube_252_output_tmp_51986_82.get_m31(2);
            *row[152] = cube_252_output_limb_2_col152;let cube_252_output_limb_3_col153 = cube_252_output_tmp_51986_82.get_m31(3);
            *row[153] = cube_252_output_limb_3_col153;let cube_252_output_limb_4_col154 = cube_252_output_tmp_51986_82.get_m31(4);
            *row[154] = cube_252_output_limb_4_col154;let cube_252_output_limb_5_col155 = cube_252_output_tmp_51986_82.get_m31(5);
            *row[155] = cube_252_output_limb_5_col155;let cube_252_output_limb_6_col156 = cube_252_output_tmp_51986_82.get_m31(6);
            *row[156] = cube_252_output_limb_6_col156;let cube_252_output_limb_7_col157 = cube_252_output_tmp_51986_82.get_m31(7);
            *row[157] = cube_252_output_limb_7_col157;let cube_252_output_limb_8_col158 = cube_252_output_tmp_51986_82.get_m31(8);
            *row[158] = cube_252_output_limb_8_col158;let cube_252_output_limb_9_col159 = cube_252_output_tmp_51986_82.get_m31(9);
            *row[159] = cube_252_output_limb_9_col159;*lookup_data.cube_252_0 = [poseidon_full_round_chain_output_limb_20_col140, poseidon_full_round_chain_output_limb_21_col141, poseidon_full_round_chain_output_limb_22_col142, poseidon_full_round_chain_output_limb_23_col143, poseidon_full_round_chain_output_limb_24_col144, poseidon_full_round_chain_output_limb_25_col145, poseidon_full_round_chain_output_limb_26_col146, poseidon_full_round_chain_output_limb_27_col147, poseidon_full_round_chain_output_limb_28_col148, poseidon_full_round_chain_output_limb_29_col149, cube_252_output_limb_0_col150, cube_252_output_limb_1_col151, cube_252_output_limb_2_col152, cube_252_output_limb_3_col153, cube_252_output_limb_4_col154, cube_252_output_limb_5_col155, cube_252_output_limb_6_col156, cube_252_output_limb_7_col157, cube_252_output_limb_8_col158, cube_252_output_limb_9_col159];

            // Linear Combination N 4 Coefs 1 1 M 2 1.

            let combination_tmp_51986_83 = PackedFelt252Width27::from_packed_felt252(((((((((Felt252_0_0_0_0) + (((Felt252_1_0_0_0) * (PackedFelt252::from_packed_felt252width27(poseidon_full_round_chain_output_round_3_tmp_51986_81.2[0])))))) + (((Felt252_1_0_0_0) * (PackedFelt252::from_packed_felt252width27(poseidon_full_round_chain_output_round_3_tmp_51986_81.2[1])))))) - (((Felt252_2_0_0_0) * (PackedFelt252::from_packed_felt252width27(cube_252_output_tmp_51986_82)))))) + (Felt252_11041071929982523380_7503192613203831446_4943121247101232560_560497091765764140)));let combination_limb_0_col160 = combination_tmp_51986_83.get_m31(0);
            *row[160] = combination_limb_0_col160;let combination_limb_1_col161 = combination_tmp_51986_83.get_m31(1);
            *row[161] = combination_limb_1_col161;let combination_limb_2_col162 = combination_tmp_51986_83.get_m31(2);
            *row[162] = combination_limb_2_col162;let combination_limb_3_col163 = combination_tmp_51986_83.get_m31(3);
            *row[163] = combination_limb_3_col163;let combination_limb_4_col164 = combination_tmp_51986_83.get_m31(4);
            *row[164] = combination_limb_4_col164;let combination_limb_5_col165 = combination_tmp_51986_83.get_m31(5);
            *row[165] = combination_limb_5_col165;let combination_limb_6_col166 = combination_tmp_51986_83.get_m31(6);
            *row[166] = combination_limb_6_col166;let combination_limb_7_col167 = combination_tmp_51986_83.get_m31(7);
            *row[167] = combination_limb_7_col167;let combination_limb_8_col168 = combination_tmp_51986_83.get_m31(8);
            *row[168] = combination_limb_8_col168;let combination_limb_9_col169 = combination_tmp_51986_83.get_m31(9);
            *row[169] = combination_limb_9_col169;let biased_limb_accumulator_u32_tmp_51986_84 = PackedUInt32::from_m31(((((((((((poseidon_full_round_chain_output_limb_0_col120) + (poseidon_full_round_chain_output_limb_10_col130))) - (((M31_2) * (cube_252_output_limb_0_col150))))) + (M31_103094260))) - (combination_limb_0_col160))) + (M31_402653187)));let p_coef_col170 = ((biased_limb_accumulator_u32_tmp_51986_84.low().as_m31()) - (M31_3));
            *row[170] = p_coef_col170;let carry_0_tmp_51986_85 = ((((((((((((poseidon_full_round_chain_output_limb_0_col120) + (poseidon_full_round_chain_output_limb_10_col130))) - (((M31_2) * (cube_252_output_limb_0_col150))))) + (M31_103094260))) - (combination_limb_0_col160))) - (p_coef_col170))) * (M31_16));let carry_1_tmp_51986_86 = ((((((((((((carry_0_tmp_51986_85) + (poseidon_full_round_chain_output_limb_1_col121))) + (poseidon_full_round_chain_output_limb_11_col131))) - (((M31_2) * (cube_252_output_limb_1_col151))))) + (M31_121146754))) - (combination_limb_1_col161))) * (M31_16));let carry_2_tmp_51986_87 = ((((((((((((carry_1_tmp_51986_86) + (poseidon_full_round_chain_output_limb_2_col122))) + (poseidon_full_round_chain_output_limb_12_col132))) - (((M31_2) * (cube_252_output_limb_2_col152))))) + (M31_95050340))) - (combination_limb_2_col162))) * (M31_16));let carry_3_tmp_51986_88 = ((((((((((((carry_2_tmp_51986_87) + (poseidon_full_round_chain_output_limb_3_col123))) + (poseidon_full_round_chain_output_limb_13_col133))) - (((M31_2) * (cube_252_output_limb_3_col153))))) + (M31_16173996))) - (combination_limb_3_col163))) * (M31_16));let carry_4_tmp_51986_89 = ((((((((((((carry_3_tmp_51986_88) + (poseidon_full_round_chain_output_limb_4_col124))) + (poseidon_full_round_chain_output_limb_14_col134))) - (((M31_2) * (cube_252_output_limb_4_col154))))) + (M31_50758155))) - (combination_limb_4_col164))) * (M31_16));let carry_5_tmp_51986_90 = ((((((((((((carry_4_tmp_51986_89) + (poseidon_full_round_chain_output_limb_5_col125))) + (poseidon_full_round_chain_output_limb_15_col135))) - (((M31_2) * (cube_252_output_limb_5_col155))))) + (M31_54415179))) - (combination_limb_5_col165))) * (M31_16));let carry_6_tmp_51986_91 = ((((((((((((carry_5_tmp_51986_90) + (poseidon_full_round_chain_output_limb_6_col126))) + (poseidon_full_round_chain_output_limb_16_col136))) - (((M31_2) * (cube_252_output_limb_6_col156))))) + (M31_19292069))) - (combination_limb_6_col166))) * (M31_16));let carry_7_tmp_51986_92 = ((((((((((((((carry_6_tmp_51986_91) + (poseidon_full_round_chain_output_limb_7_col127))) + (poseidon_full_round_chain_output_limb_17_col137))) - (((M31_2) * (cube_252_output_limb_7_col157))))) + (M31_45351266))) - (combination_limb_7_col167))) - (((p_coef_col170) * (M31_136))))) * (M31_16));let carry_8_tmp_51986_93 = ((((((((((((carry_7_tmp_51986_92) + (poseidon_full_round_chain_output_limb_8_col128))) + (poseidon_full_round_chain_output_limb_18_col138))) - (((M31_2) * (cube_252_output_limb_8_col158))))) + (M31_122233508))) - (combination_limb_8_col168))) * (M31_16));*sub_component_inputs.range_check_3_3_3_3_3[0] =
                [((p_coef_col170) + (M31_3)), ((carry_0_tmp_51986_85) + (M31_3)), ((carry_1_tmp_51986_86) + (M31_3)), ((carry_2_tmp_51986_87) + (M31_3)), ((carry_3_tmp_51986_88) + (M31_3))];
            *lookup_data.range_check_3_3_3_3_3_0 = [((p_coef_col170) + (M31_3)), ((carry_0_tmp_51986_85) + (M31_3)), ((carry_1_tmp_51986_86) + (M31_3)), ((carry_2_tmp_51986_87) + (M31_3)), ((carry_3_tmp_51986_88) + (M31_3))];*sub_component_inputs.range_check_3_3_3_3_3[1] =
                [((carry_4_tmp_51986_89) + (M31_3)), ((carry_5_tmp_51986_90) + (M31_3)), ((carry_6_tmp_51986_91) + (M31_3)), ((carry_7_tmp_51986_92) + (M31_3)), ((carry_8_tmp_51986_93) + (M31_3))];
            *lookup_data.range_check_3_3_3_3_3_1 = [((carry_4_tmp_51986_89) + (M31_3)), ((carry_5_tmp_51986_90) + (M31_3)), ((carry_6_tmp_51986_91) + (M31_3)), ((carry_7_tmp_51986_92) + (M31_3)), ((carry_8_tmp_51986_93) + (M31_3))];let linear_combination_n_4_coefs_1_1_m2_1_output_tmp_51986_94 = combination_tmp_51986_83;

            *sub_component_inputs.cube_252[1] =
                linear_combination_n_4_coefs_1_1_m2_1_output_tmp_51986_94;
            let cube_252_output_tmp_51986_95 = PackedCube252::deduce_output(linear_combination_n_4_coefs_1_1_m2_1_output_tmp_51986_94);let cube_252_output_limb_0_col171 = cube_252_output_tmp_51986_95.get_m31(0);
            *row[171] = cube_252_output_limb_0_col171;let cube_252_output_limb_1_col172 = cube_252_output_tmp_51986_95.get_m31(1);
            *row[172] = cube_252_output_limb_1_col172;let cube_252_output_limb_2_col173 = cube_252_output_tmp_51986_95.get_m31(2);
            *row[173] = cube_252_output_limb_2_col173;let cube_252_output_limb_3_col174 = cube_252_output_tmp_51986_95.get_m31(3);
            *row[174] = cube_252_output_limb_3_col174;let cube_252_output_limb_4_col175 = cube_252_output_tmp_51986_95.get_m31(4);
            *row[175] = cube_252_output_limb_4_col175;let cube_252_output_limb_5_col176 = cube_252_output_tmp_51986_95.get_m31(5);
            *row[176] = cube_252_output_limb_5_col176;let cube_252_output_limb_6_col177 = cube_252_output_tmp_51986_95.get_m31(6);
            *row[177] = cube_252_output_limb_6_col177;let cube_252_output_limb_7_col178 = cube_252_output_tmp_51986_95.get_m31(7);
            *row[178] = cube_252_output_limb_7_col178;let cube_252_output_limb_8_col179 = cube_252_output_tmp_51986_95.get_m31(8);
            *row[179] = cube_252_output_limb_8_col179;let cube_252_output_limb_9_col180 = cube_252_output_tmp_51986_95.get_m31(9);
            *row[180] = cube_252_output_limb_9_col180;*lookup_data.cube_252_1 = [combination_limb_0_col160, combination_limb_1_col161, combination_limb_2_col162, combination_limb_3_col163, combination_limb_4_col164, combination_limb_5_col165, combination_limb_6_col166, combination_limb_7_col167, combination_limb_8_col168, combination_limb_9_col169, cube_252_output_limb_0_col171, cube_252_output_limb_1_col172, cube_252_output_limb_2_col173, cube_252_output_limb_3_col174, cube_252_output_limb_4_col175, cube_252_output_limb_5_col176, cube_252_output_limb_6_col177, cube_252_output_limb_7_col178, cube_252_output_limb_8_col179, cube_252_output_limb_9_col180];

            // Linear Combination N 4 Coefs 4 2 M 2 1.

            let combination_tmp_51986_96 = PackedFelt252Width27::from_packed_felt252(((((((((Felt252_0_0_0_0) + (((Felt252_4_0_0_0) * (PackedFelt252::from_packed_felt252width27(poseidon_full_round_chain_output_round_3_tmp_51986_81.2[0])))))) + (((Felt252_2_0_0_0) * (PackedFelt252::from_packed_felt252width27(cube_252_output_tmp_51986_82)))))) - (((Felt252_2_0_0_0) * (PackedFelt252::from_packed_felt252width27(cube_252_output_tmp_51986_95)))))) + (Felt252_10931822301410252833_1475756362763989377_3378552166684303673_348229636055909092)));let combination_limb_0_col181 = combination_tmp_51986_96.get_m31(0);
            *row[181] = combination_limb_0_col181;let combination_limb_1_col182 = combination_tmp_51986_96.get_m31(1);
            *row[182] = combination_limb_1_col182;let combination_limb_2_col183 = combination_tmp_51986_96.get_m31(2);
            *row[183] = combination_limb_2_col183;let combination_limb_3_col184 = combination_tmp_51986_96.get_m31(3);
            *row[184] = combination_limb_3_col184;let combination_limb_4_col185 = combination_tmp_51986_96.get_m31(4);
            *row[185] = combination_limb_4_col185;let combination_limb_5_col186 = combination_tmp_51986_96.get_m31(5);
            *row[186] = combination_limb_5_col186;let combination_limb_6_col187 = combination_tmp_51986_96.get_m31(6);
            *row[187] = combination_limb_6_col187;let combination_limb_7_col188 = combination_tmp_51986_96.get_m31(7);
            *row[188] = combination_limb_7_col188;let combination_limb_8_col189 = combination_tmp_51986_96.get_m31(8);
            *row[189] = combination_limb_8_col189;let combination_limb_9_col190 = combination_tmp_51986_96.get_m31(9);
            *row[190] = combination_limb_9_col190;let biased_limb_accumulator_u32_tmp_51986_97 = PackedUInt32::from_m31(((((((((((((M31_4) * (poseidon_full_round_chain_output_limb_0_col120))) + (((M31_2) * (cube_252_output_limb_0_col150))))) - (((M31_2) * (cube_252_output_limb_0_col171))))) + (M31_121657377))) - (combination_limb_0_col181))) + (M31_402653187)));let p_coef_col191 = ((biased_limb_accumulator_u32_tmp_51986_97.low().as_m31()) - (M31_3));
            *row[191] = p_coef_col191;let carry_0_tmp_51986_98 = ((((((((((((((M31_4) * (poseidon_full_round_chain_output_limb_0_col120))) + (((M31_2) * (cube_252_output_limb_0_col150))))) - (((M31_2) * (cube_252_output_limb_0_col171))))) + (M31_121657377))) - (combination_limb_0_col181))) - (p_coef_col191))) * (M31_16));let carry_1_tmp_51986_99 = ((((((((((((carry_0_tmp_51986_98) + (((M31_4) * (poseidon_full_round_chain_output_limb_1_col121))))) + (((M31_2) * (cube_252_output_limb_1_col151))))) - (((M31_2) * (cube_252_output_limb_1_col172))))) + (M31_112479959))) - (combination_limb_1_col182))) * (M31_16));let carry_2_tmp_51986_100 = ((((((((((((carry_1_tmp_51986_99) + (((M31_4) * (poseidon_full_round_chain_output_limb_2_col122))))) + (((M31_2) * (cube_252_output_limb_2_col152))))) - (((M31_2) * (cube_252_output_limb_2_col173))))) + (M31_130418270))) - (combination_limb_2_col183))) * (M31_16));let carry_3_tmp_51986_101 = ((((((((((((carry_2_tmp_51986_100) + (((M31_4) * (poseidon_full_round_chain_output_limb_3_col123))))) + (((M31_2) * (cube_252_output_limb_3_col153))))) - (((M31_2) * (cube_252_output_limb_3_col174))))) + (M31_4974792))) - (combination_limb_3_col184))) * (M31_16));let carry_4_tmp_51986_102 = ((((((((((((carry_3_tmp_51986_101) + (((M31_4) * (poseidon_full_round_chain_output_limb_4_col124))))) + (((M31_2) * (cube_252_output_limb_4_col154))))) - (((M31_2) * (cube_252_output_limb_4_col175))))) + (M31_59852719))) - (combination_limb_4_col185))) * (M31_16));let carry_5_tmp_51986_103 = ((((((((((((carry_4_tmp_51986_102) + (((M31_4) * (poseidon_full_round_chain_output_limb_5_col125))))) + (((M31_2) * (cube_252_output_limb_5_col155))))) - (((M31_2) * (cube_252_output_limb_5_col176))))) + (M31_120369218))) - (combination_limb_5_col186))) * (M31_16));let carry_6_tmp_51986_104 = ((((((((((((carry_5_tmp_51986_103) + (((M31_4) * (poseidon_full_round_chain_output_limb_6_col126))))) + (((M31_2) * (cube_252_output_limb_6_col156))))) - (((M31_2) * (cube_252_output_limb_6_col177))))) + (M31_62439890))) - (combination_limb_6_col187))) * (M31_16));let carry_7_tmp_51986_105 = ((((((((((((((carry_6_tmp_51986_104) + (((M31_4) * (poseidon_full_round_chain_output_limb_7_col127))))) + (((M31_2) * (cube_252_output_limb_7_col157))))) - (((M31_2) * (cube_252_output_limb_7_col178))))) + (M31_50468641))) - (combination_limb_7_col188))) - (((p_coef_col191) * (M31_136))))) * (M31_16));let carry_8_tmp_51986_106 = ((((((((((((carry_7_tmp_51986_105) + (((M31_4) * (poseidon_full_round_chain_output_limb_8_col128))))) + (((M31_2) * (cube_252_output_limb_8_col158))))) - (((M31_2) * (cube_252_output_limb_8_col179))))) + (M31_86573645))) - (combination_limb_8_col189))) * (M31_16));*sub_component_inputs.range_check_4_4_4_4[0] =
                [((p_coef_col191) + (M31_3)), ((carry_0_tmp_51986_98) + (M31_3)), ((carry_1_tmp_51986_99) + (M31_3)), ((carry_2_tmp_51986_100) + (M31_3))];
            *lookup_data.range_check_4_4_4_4_0 = [((p_coef_col191) + (M31_3)), ((carry_0_tmp_51986_98) + (M31_3)), ((carry_1_tmp_51986_99) + (M31_3)), ((carry_2_tmp_51986_100) + (M31_3))];*sub_component_inputs.range_check_4_4_4_4[1] =
                [((carry_3_tmp_51986_101) + (M31_3)), ((carry_4_tmp_51986_102) + (M31_3)), ((carry_5_tmp_51986_103) + (M31_3)), ((carry_6_tmp_51986_104) + (M31_3))];
            *lookup_data.range_check_4_4_4_4_1 = [((carry_3_tmp_51986_101) + (M31_3)), ((carry_4_tmp_51986_102) + (M31_3)), ((carry_5_tmp_51986_103) + (M31_3)), ((carry_6_tmp_51986_104) + (M31_3))];*sub_component_inputs.range_check_4_4[0] =
                [((carry_7_tmp_51986_105) + (M31_3)), ((carry_8_tmp_51986_106) + (M31_3))];
            *lookup_data.range_check_4_4_0 = [((carry_7_tmp_51986_105) + (M31_3)), ((carry_8_tmp_51986_106) + (M31_3))];let linear_combination_n_4_coefs_4_2_m2_1_output_tmp_51986_107 = combination_tmp_51986_96;

            *lookup_data.poseidon_3_partial_rounds_chain_0 = [seq, M31_4, cube_252_output_limb_0_col150, cube_252_output_limb_1_col151, cube_252_output_limb_2_col152, cube_252_output_limb_3_col153, cube_252_output_limb_4_col154, cube_252_output_limb_5_col155, cube_252_output_limb_6_col156, cube_252_output_limb_7_col157, cube_252_output_limb_8_col158, cube_252_output_limb_9_col159, combination_limb_0_col160, combination_limb_1_col161, combination_limb_2_col162, combination_limb_3_col163, combination_limb_4_col164, combination_limb_5_col165, combination_limb_6_col166, combination_limb_7_col167, combination_limb_8_col168, combination_limb_9_col169, cube_252_output_limb_0_col171, cube_252_output_limb_1_col172, cube_252_output_limb_2_col173, cube_252_output_limb_3_col174, cube_252_output_limb_4_col175, cube_252_output_limb_5_col176, cube_252_output_limb_6_col177, cube_252_output_limb_7_col178, cube_252_output_limb_8_col179, cube_252_output_limb_9_col180, combination_limb_0_col181, combination_limb_1_col182, combination_limb_2_col183, combination_limb_3_col184, combination_limb_4_col185, combination_limb_5_col186, combination_limb_6_col187, combination_limb_7_col188, combination_limb_8_col189, combination_limb_9_col190];*sub_component_inputs.poseidon_3_partial_rounds_chain[0] =
                (seq, M31_4, [cube_252_output_tmp_51986_82, linear_combination_n_4_coefs_1_1_m2_1_output_tmp_51986_94, cube_252_output_tmp_51986_95, linear_combination_n_4_coefs_4_2_m2_1_output_tmp_51986_107]);
            let poseidon_3_partial_rounds_chain_output_round_4_tmp_51986_108 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_4, [cube_252_output_tmp_51986_82, linear_combination_n_4_coefs_1_1_m2_1_output_tmp_51986_94, cube_252_output_tmp_51986_95, linear_combination_n_4_coefs_4_2_m2_1_output_tmp_51986_107]));*sub_component_inputs.poseidon_3_partial_rounds_chain[1] =
                (seq, M31_5, [poseidon_3_partial_rounds_chain_output_round_4_tmp_51986_108.2[0], poseidon_3_partial_rounds_chain_output_round_4_tmp_51986_108.2[1], poseidon_3_partial_rounds_chain_output_round_4_tmp_51986_108.2[2], poseidon_3_partial_rounds_chain_output_round_4_tmp_51986_108.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_5_tmp_51986_109 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_5, [poseidon_3_partial_rounds_chain_output_round_4_tmp_51986_108.2[0], poseidon_3_partial_rounds_chain_output_round_4_tmp_51986_108.2[1], poseidon_3_partial_rounds_chain_output_round_4_tmp_51986_108.2[2], poseidon_3_partial_rounds_chain_output_round_4_tmp_51986_108.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[2] =
                (seq, M31_6, [poseidon_3_partial_rounds_chain_output_round_5_tmp_51986_109.2[0], poseidon_3_partial_rounds_chain_output_round_5_tmp_51986_109.2[1], poseidon_3_partial_rounds_chain_output_round_5_tmp_51986_109.2[2], poseidon_3_partial_rounds_chain_output_round_5_tmp_51986_109.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_6_tmp_51986_110 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_6, [poseidon_3_partial_rounds_chain_output_round_5_tmp_51986_109.2[0], poseidon_3_partial_rounds_chain_output_round_5_tmp_51986_109.2[1], poseidon_3_partial_rounds_chain_output_round_5_tmp_51986_109.2[2], poseidon_3_partial_rounds_chain_output_round_5_tmp_51986_109.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[3] =
                (seq, M31_7, [poseidon_3_partial_rounds_chain_output_round_6_tmp_51986_110.2[0], poseidon_3_partial_rounds_chain_output_round_6_tmp_51986_110.2[1], poseidon_3_partial_rounds_chain_output_round_6_tmp_51986_110.2[2], poseidon_3_partial_rounds_chain_output_round_6_tmp_51986_110.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_7_tmp_51986_111 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_7, [poseidon_3_partial_rounds_chain_output_round_6_tmp_51986_110.2[0], poseidon_3_partial_rounds_chain_output_round_6_tmp_51986_110.2[1], poseidon_3_partial_rounds_chain_output_round_6_tmp_51986_110.2[2], poseidon_3_partial_rounds_chain_output_round_6_tmp_51986_110.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[4] =
                (seq, M31_8, [poseidon_3_partial_rounds_chain_output_round_7_tmp_51986_111.2[0], poseidon_3_partial_rounds_chain_output_round_7_tmp_51986_111.2[1], poseidon_3_partial_rounds_chain_output_round_7_tmp_51986_111.2[2], poseidon_3_partial_rounds_chain_output_round_7_tmp_51986_111.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_8_tmp_51986_112 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_8, [poseidon_3_partial_rounds_chain_output_round_7_tmp_51986_111.2[0], poseidon_3_partial_rounds_chain_output_round_7_tmp_51986_111.2[1], poseidon_3_partial_rounds_chain_output_round_7_tmp_51986_111.2[2], poseidon_3_partial_rounds_chain_output_round_7_tmp_51986_111.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[5] =
                (seq, M31_9, [poseidon_3_partial_rounds_chain_output_round_8_tmp_51986_112.2[0], poseidon_3_partial_rounds_chain_output_round_8_tmp_51986_112.2[1], poseidon_3_partial_rounds_chain_output_round_8_tmp_51986_112.2[2], poseidon_3_partial_rounds_chain_output_round_8_tmp_51986_112.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_9_tmp_51986_113 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_9, [poseidon_3_partial_rounds_chain_output_round_8_tmp_51986_112.2[0], poseidon_3_partial_rounds_chain_output_round_8_tmp_51986_112.2[1], poseidon_3_partial_rounds_chain_output_round_8_tmp_51986_112.2[2], poseidon_3_partial_rounds_chain_output_round_8_tmp_51986_112.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[6] =
                (seq, M31_10, [poseidon_3_partial_rounds_chain_output_round_9_tmp_51986_113.2[0], poseidon_3_partial_rounds_chain_output_round_9_tmp_51986_113.2[1], poseidon_3_partial_rounds_chain_output_round_9_tmp_51986_113.2[2], poseidon_3_partial_rounds_chain_output_round_9_tmp_51986_113.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_10_tmp_51986_114 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_10, [poseidon_3_partial_rounds_chain_output_round_9_tmp_51986_113.2[0], poseidon_3_partial_rounds_chain_output_round_9_tmp_51986_113.2[1], poseidon_3_partial_rounds_chain_output_round_9_tmp_51986_113.2[2], poseidon_3_partial_rounds_chain_output_round_9_tmp_51986_113.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[7] =
                (seq, M31_11, [poseidon_3_partial_rounds_chain_output_round_10_tmp_51986_114.2[0], poseidon_3_partial_rounds_chain_output_round_10_tmp_51986_114.2[1], poseidon_3_partial_rounds_chain_output_round_10_tmp_51986_114.2[2], poseidon_3_partial_rounds_chain_output_round_10_tmp_51986_114.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_11_tmp_51986_115 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_11, [poseidon_3_partial_rounds_chain_output_round_10_tmp_51986_114.2[0], poseidon_3_partial_rounds_chain_output_round_10_tmp_51986_114.2[1], poseidon_3_partial_rounds_chain_output_round_10_tmp_51986_114.2[2], poseidon_3_partial_rounds_chain_output_round_10_tmp_51986_114.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[8] =
                (seq, M31_12, [poseidon_3_partial_rounds_chain_output_round_11_tmp_51986_115.2[0], poseidon_3_partial_rounds_chain_output_round_11_tmp_51986_115.2[1], poseidon_3_partial_rounds_chain_output_round_11_tmp_51986_115.2[2], poseidon_3_partial_rounds_chain_output_round_11_tmp_51986_115.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_12_tmp_51986_116 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_12, [poseidon_3_partial_rounds_chain_output_round_11_tmp_51986_115.2[0], poseidon_3_partial_rounds_chain_output_round_11_tmp_51986_115.2[1], poseidon_3_partial_rounds_chain_output_round_11_tmp_51986_115.2[2], poseidon_3_partial_rounds_chain_output_round_11_tmp_51986_115.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[9] =
                (seq, M31_13, [poseidon_3_partial_rounds_chain_output_round_12_tmp_51986_116.2[0], poseidon_3_partial_rounds_chain_output_round_12_tmp_51986_116.2[1], poseidon_3_partial_rounds_chain_output_round_12_tmp_51986_116.2[2], poseidon_3_partial_rounds_chain_output_round_12_tmp_51986_116.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_13_tmp_51986_117 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_13, [poseidon_3_partial_rounds_chain_output_round_12_tmp_51986_116.2[0], poseidon_3_partial_rounds_chain_output_round_12_tmp_51986_116.2[1], poseidon_3_partial_rounds_chain_output_round_12_tmp_51986_116.2[2], poseidon_3_partial_rounds_chain_output_round_12_tmp_51986_116.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[10] =
                (seq, M31_14, [poseidon_3_partial_rounds_chain_output_round_13_tmp_51986_117.2[0], poseidon_3_partial_rounds_chain_output_round_13_tmp_51986_117.2[1], poseidon_3_partial_rounds_chain_output_round_13_tmp_51986_117.2[2], poseidon_3_partial_rounds_chain_output_round_13_tmp_51986_117.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_14_tmp_51986_118 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_14, [poseidon_3_partial_rounds_chain_output_round_13_tmp_51986_117.2[0], poseidon_3_partial_rounds_chain_output_round_13_tmp_51986_117.2[1], poseidon_3_partial_rounds_chain_output_round_13_tmp_51986_117.2[2], poseidon_3_partial_rounds_chain_output_round_13_tmp_51986_117.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[11] =
                (seq, M31_15, [poseidon_3_partial_rounds_chain_output_round_14_tmp_51986_118.2[0], poseidon_3_partial_rounds_chain_output_round_14_tmp_51986_118.2[1], poseidon_3_partial_rounds_chain_output_round_14_tmp_51986_118.2[2], poseidon_3_partial_rounds_chain_output_round_14_tmp_51986_118.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_15_tmp_51986_119 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_15, [poseidon_3_partial_rounds_chain_output_round_14_tmp_51986_118.2[0], poseidon_3_partial_rounds_chain_output_round_14_tmp_51986_118.2[1], poseidon_3_partial_rounds_chain_output_round_14_tmp_51986_118.2[2], poseidon_3_partial_rounds_chain_output_round_14_tmp_51986_118.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[12] =
                (seq, M31_16, [poseidon_3_partial_rounds_chain_output_round_15_tmp_51986_119.2[0], poseidon_3_partial_rounds_chain_output_round_15_tmp_51986_119.2[1], poseidon_3_partial_rounds_chain_output_round_15_tmp_51986_119.2[2], poseidon_3_partial_rounds_chain_output_round_15_tmp_51986_119.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_16_tmp_51986_120 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_16, [poseidon_3_partial_rounds_chain_output_round_15_tmp_51986_119.2[0], poseidon_3_partial_rounds_chain_output_round_15_tmp_51986_119.2[1], poseidon_3_partial_rounds_chain_output_round_15_tmp_51986_119.2[2], poseidon_3_partial_rounds_chain_output_round_15_tmp_51986_119.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[13] =
                (seq, M31_17, [poseidon_3_partial_rounds_chain_output_round_16_tmp_51986_120.2[0], poseidon_3_partial_rounds_chain_output_round_16_tmp_51986_120.2[1], poseidon_3_partial_rounds_chain_output_round_16_tmp_51986_120.2[2], poseidon_3_partial_rounds_chain_output_round_16_tmp_51986_120.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_17_tmp_51986_121 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_17, [poseidon_3_partial_rounds_chain_output_round_16_tmp_51986_120.2[0], poseidon_3_partial_rounds_chain_output_round_16_tmp_51986_120.2[1], poseidon_3_partial_rounds_chain_output_round_16_tmp_51986_120.2[2], poseidon_3_partial_rounds_chain_output_round_16_tmp_51986_120.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[14] =
                (seq, M31_18, [poseidon_3_partial_rounds_chain_output_round_17_tmp_51986_121.2[0], poseidon_3_partial_rounds_chain_output_round_17_tmp_51986_121.2[1], poseidon_3_partial_rounds_chain_output_round_17_tmp_51986_121.2[2], poseidon_3_partial_rounds_chain_output_round_17_tmp_51986_121.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_18_tmp_51986_122 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_18, [poseidon_3_partial_rounds_chain_output_round_17_tmp_51986_121.2[0], poseidon_3_partial_rounds_chain_output_round_17_tmp_51986_121.2[1], poseidon_3_partial_rounds_chain_output_round_17_tmp_51986_121.2[2], poseidon_3_partial_rounds_chain_output_round_17_tmp_51986_121.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[15] =
                (seq, M31_19, [poseidon_3_partial_rounds_chain_output_round_18_tmp_51986_122.2[0], poseidon_3_partial_rounds_chain_output_round_18_tmp_51986_122.2[1], poseidon_3_partial_rounds_chain_output_round_18_tmp_51986_122.2[2], poseidon_3_partial_rounds_chain_output_round_18_tmp_51986_122.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_19_tmp_51986_123 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_19, [poseidon_3_partial_rounds_chain_output_round_18_tmp_51986_122.2[0], poseidon_3_partial_rounds_chain_output_round_18_tmp_51986_122.2[1], poseidon_3_partial_rounds_chain_output_round_18_tmp_51986_122.2[2], poseidon_3_partial_rounds_chain_output_round_18_tmp_51986_122.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[16] =
                (seq, M31_20, [poseidon_3_partial_rounds_chain_output_round_19_tmp_51986_123.2[0], poseidon_3_partial_rounds_chain_output_round_19_tmp_51986_123.2[1], poseidon_3_partial_rounds_chain_output_round_19_tmp_51986_123.2[2], poseidon_3_partial_rounds_chain_output_round_19_tmp_51986_123.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_20_tmp_51986_124 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_20, [poseidon_3_partial_rounds_chain_output_round_19_tmp_51986_123.2[0], poseidon_3_partial_rounds_chain_output_round_19_tmp_51986_123.2[1], poseidon_3_partial_rounds_chain_output_round_19_tmp_51986_123.2[2], poseidon_3_partial_rounds_chain_output_round_19_tmp_51986_123.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[17] =
                (seq, M31_21, [poseidon_3_partial_rounds_chain_output_round_20_tmp_51986_124.2[0], poseidon_3_partial_rounds_chain_output_round_20_tmp_51986_124.2[1], poseidon_3_partial_rounds_chain_output_round_20_tmp_51986_124.2[2], poseidon_3_partial_rounds_chain_output_round_20_tmp_51986_124.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_21_tmp_51986_125 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_21, [poseidon_3_partial_rounds_chain_output_round_20_tmp_51986_124.2[0], poseidon_3_partial_rounds_chain_output_round_20_tmp_51986_124.2[1], poseidon_3_partial_rounds_chain_output_round_20_tmp_51986_124.2[2], poseidon_3_partial_rounds_chain_output_round_20_tmp_51986_124.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[18] =
                (seq, M31_22, [poseidon_3_partial_rounds_chain_output_round_21_tmp_51986_125.2[0], poseidon_3_partial_rounds_chain_output_round_21_tmp_51986_125.2[1], poseidon_3_partial_rounds_chain_output_round_21_tmp_51986_125.2[2], poseidon_3_partial_rounds_chain_output_round_21_tmp_51986_125.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_22_tmp_51986_126 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_22, [poseidon_3_partial_rounds_chain_output_round_21_tmp_51986_125.2[0], poseidon_3_partial_rounds_chain_output_round_21_tmp_51986_125.2[1], poseidon_3_partial_rounds_chain_output_round_21_tmp_51986_125.2[2], poseidon_3_partial_rounds_chain_output_round_21_tmp_51986_125.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[19] =
                (seq, M31_23, [poseidon_3_partial_rounds_chain_output_round_22_tmp_51986_126.2[0], poseidon_3_partial_rounds_chain_output_round_22_tmp_51986_126.2[1], poseidon_3_partial_rounds_chain_output_round_22_tmp_51986_126.2[2], poseidon_3_partial_rounds_chain_output_round_22_tmp_51986_126.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_23_tmp_51986_127 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_23, [poseidon_3_partial_rounds_chain_output_round_22_tmp_51986_126.2[0], poseidon_3_partial_rounds_chain_output_round_22_tmp_51986_126.2[1], poseidon_3_partial_rounds_chain_output_round_22_tmp_51986_126.2[2], poseidon_3_partial_rounds_chain_output_round_22_tmp_51986_126.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[20] =
                (seq, M31_24, [poseidon_3_partial_rounds_chain_output_round_23_tmp_51986_127.2[0], poseidon_3_partial_rounds_chain_output_round_23_tmp_51986_127.2[1], poseidon_3_partial_rounds_chain_output_round_23_tmp_51986_127.2[2], poseidon_3_partial_rounds_chain_output_round_23_tmp_51986_127.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_24_tmp_51986_128 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_24, [poseidon_3_partial_rounds_chain_output_round_23_tmp_51986_127.2[0], poseidon_3_partial_rounds_chain_output_round_23_tmp_51986_127.2[1], poseidon_3_partial_rounds_chain_output_round_23_tmp_51986_127.2[2], poseidon_3_partial_rounds_chain_output_round_23_tmp_51986_127.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[21] =
                (seq, M31_25, [poseidon_3_partial_rounds_chain_output_round_24_tmp_51986_128.2[0], poseidon_3_partial_rounds_chain_output_round_24_tmp_51986_128.2[1], poseidon_3_partial_rounds_chain_output_round_24_tmp_51986_128.2[2], poseidon_3_partial_rounds_chain_output_round_24_tmp_51986_128.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_25_tmp_51986_129 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_25, [poseidon_3_partial_rounds_chain_output_round_24_tmp_51986_128.2[0], poseidon_3_partial_rounds_chain_output_round_24_tmp_51986_128.2[1], poseidon_3_partial_rounds_chain_output_round_24_tmp_51986_128.2[2], poseidon_3_partial_rounds_chain_output_round_24_tmp_51986_128.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[22] =
                (seq, M31_26, [poseidon_3_partial_rounds_chain_output_round_25_tmp_51986_129.2[0], poseidon_3_partial_rounds_chain_output_round_25_tmp_51986_129.2[1], poseidon_3_partial_rounds_chain_output_round_25_tmp_51986_129.2[2], poseidon_3_partial_rounds_chain_output_round_25_tmp_51986_129.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_26_tmp_51986_130 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_26, [poseidon_3_partial_rounds_chain_output_round_25_tmp_51986_129.2[0], poseidon_3_partial_rounds_chain_output_round_25_tmp_51986_129.2[1], poseidon_3_partial_rounds_chain_output_round_25_tmp_51986_129.2[2], poseidon_3_partial_rounds_chain_output_round_25_tmp_51986_129.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[23] =
                (seq, M31_27, [poseidon_3_partial_rounds_chain_output_round_26_tmp_51986_130.2[0], poseidon_3_partial_rounds_chain_output_round_26_tmp_51986_130.2[1], poseidon_3_partial_rounds_chain_output_round_26_tmp_51986_130.2[2], poseidon_3_partial_rounds_chain_output_round_26_tmp_51986_130.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_27_tmp_51986_131 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_27, [poseidon_3_partial_rounds_chain_output_round_26_tmp_51986_130.2[0], poseidon_3_partial_rounds_chain_output_round_26_tmp_51986_130.2[1], poseidon_3_partial_rounds_chain_output_round_26_tmp_51986_130.2[2], poseidon_3_partial_rounds_chain_output_round_26_tmp_51986_130.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[24] =
                (seq, M31_28, [poseidon_3_partial_rounds_chain_output_round_27_tmp_51986_131.2[0], poseidon_3_partial_rounds_chain_output_round_27_tmp_51986_131.2[1], poseidon_3_partial_rounds_chain_output_round_27_tmp_51986_131.2[2], poseidon_3_partial_rounds_chain_output_round_27_tmp_51986_131.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_28_tmp_51986_132 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_28, [poseidon_3_partial_rounds_chain_output_round_27_tmp_51986_131.2[0], poseidon_3_partial_rounds_chain_output_round_27_tmp_51986_131.2[1], poseidon_3_partial_rounds_chain_output_round_27_tmp_51986_131.2[2], poseidon_3_partial_rounds_chain_output_round_27_tmp_51986_131.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[25] =
                (seq, M31_29, [poseidon_3_partial_rounds_chain_output_round_28_tmp_51986_132.2[0], poseidon_3_partial_rounds_chain_output_round_28_tmp_51986_132.2[1], poseidon_3_partial_rounds_chain_output_round_28_tmp_51986_132.2[2], poseidon_3_partial_rounds_chain_output_round_28_tmp_51986_132.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_29_tmp_51986_133 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_29, [poseidon_3_partial_rounds_chain_output_round_28_tmp_51986_132.2[0], poseidon_3_partial_rounds_chain_output_round_28_tmp_51986_132.2[1], poseidon_3_partial_rounds_chain_output_round_28_tmp_51986_132.2[2], poseidon_3_partial_rounds_chain_output_round_28_tmp_51986_132.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[26] =
                (seq, M31_30, [poseidon_3_partial_rounds_chain_output_round_29_tmp_51986_133.2[0], poseidon_3_partial_rounds_chain_output_round_29_tmp_51986_133.2[1], poseidon_3_partial_rounds_chain_output_round_29_tmp_51986_133.2[2], poseidon_3_partial_rounds_chain_output_round_29_tmp_51986_133.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_30, [poseidon_3_partial_rounds_chain_output_round_29_tmp_51986_133.2[0], poseidon_3_partial_rounds_chain_output_round_29_tmp_51986_133.2[1], poseidon_3_partial_rounds_chain_output_round_29_tmp_51986_133.2[2], poseidon_3_partial_rounds_chain_output_round_29_tmp_51986_133.2[3]]));let poseidon_3_partial_rounds_chain_output_limb_0_col192 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[0].get_m31(0);
            *row[192] = poseidon_3_partial_rounds_chain_output_limb_0_col192;let poseidon_3_partial_rounds_chain_output_limb_1_col193 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[0].get_m31(1);
            *row[193] = poseidon_3_partial_rounds_chain_output_limb_1_col193;let poseidon_3_partial_rounds_chain_output_limb_2_col194 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[0].get_m31(2);
            *row[194] = poseidon_3_partial_rounds_chain_output_limb_2_col194;let poseidon_3_partial_rounds_chain_output_limb_3_col195 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[0].get_m31(3);
            *row[195] = poseidon_3_partial_rounds_chain_output_limb_3_col195;let poseidon_3_partial_rounds_chain_output_limb_4_col196 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[0].get_m31(4);
            *row[196] = poseidon_3_partial_rounds_chain_output_limb_4_col196;let poseidon_3_partial_rounds_chain_output_limb_5_col197 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[0].get_m31(5);
            *row[197] = poseidon_3_partial_rounds_chain_output_limb_5_col197;let poseidon_3_partial_rounds_chain_output_limb_6_col198 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[0].get_m31(6);
            *row[198] = poseidon_3_partial_rounds_chain_output_limb_6_col198;let poseidon_3_partial_rounds_chain_output_limb_7_col199 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[0].get_m31(7);
            *row[199] = poseidon_3_partial_rounds_chain_output_limb_7_col199;let poseidon_3_partial_rounds_chain_output_limb_8_col200 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[0].get_m31(8);
            *row[200] = poseidon_3_partial_rounds_chain_output_limb_8_col200;let poseidon_3_partial_rounds_chain_output_limb_9_col201 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[0].get_m31(9);
            *row[201] = poseidon_3_partial_rounds_chain_output_limb_9_col201;let poseidon_3_partial_rounds_chain_output_limb_10_col202 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[1].get_m31(0);
            *row[202] = poseidon_3_partial_rounds_chain_output_limb_10_col202;let poseidon_3_partial_rounds_chain_output_limb_11_col203 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[1].get_m31(1);
            *row[203] = poseidon_3_partial_rounds_chain_output_limb_11_col203;let poseidon_3_partial_rounds_chain_output_limb_12_col204 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[1].get_m31(2);
            *row[204] = poseidon_3_partial_rounds_chain_output_limb_12_col204;let poseidon_3_partial_rounds_chain_output_limb_13_col205 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[1].get_m31(3);
            *row[205] = poseidon_3_partial_rounds_chain_output_limb_13_col205;let poseidon_3_partial_rounds_chain_output_limb_14_col206 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[1].get_m31(4);
            *row[206] = poseidon_3_partial_rounds_chain_output_limb_14_col206;let poseidon_3_partial_rounds_chain_output_limb_15_col207 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[1].get_m31(5);
            *row[207] = poseidon_3_partial_rounds_chain_output_limb_15_col207;let poseidon_3_partial_rounds_chain_output_limb_16_col208 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[1].get_m31(6);
            *row[208] = poseidon_3_partial_rounds_chain_output_limb_16_col208;let poseidon_3_partial_rounds_chain_output_limb_17_col209 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[1].get_m31(7);
            *row[209] = poseidon_3_partial_rounds_chain_output_limb_17_col209;let poseidon_3_partial_rounds_chain_output_limb_18_col210 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[1].get_m31(8);
            *row[210] = poseidon_3_partial_rounds_chain_output_limb_18_col210;let poseidon_3_partial_rounds_chain_output_limb_19_col211 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[1].get_m31(9);
            *row[211] = poseidon_3_partial_rounds_chain_output_limb_19_col211;let poseidon_3_partial_rounds_chain_output_limb_20_col212 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[2].get_m31(0);
            *row[212] = poseidon_3_partial_rounds_chain_output_limb_20_col212;let poseidon_3_partial_rounds_chain_output_limb_21_col213 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[2].get_m31(1);
            *row[213] = poseidon_3_partial_rounds_chain_output_limb_21_col213;let poseidon_3_partial_rounds_chain_output_limb_22_col214 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[2].get_m31(2);
            *row[214] = poseidon_3_partial_rounds_chain_output_limb_22_col214;let poseidon_3_partial_rounds_chain_output_limb_23_col215 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[2].get_m31(3);
            *row[215] = poseidon_3_partial_rounds_chain_output_limb_23_col215;let poseidon_3_partial_rounds_chain_output_limb_24_col216 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[2].get_m31(4);
            *row[216] = poseidon_3_partial_rounds_chain_output_limb_24_col216;let poseidon_3_partial_rounds_chain_output_limb_25_col217 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[2].get_m31(5);
            *row[217] = poseidon_3_partial_rounds_chain_output_limb_25_col217;let poseidon_3_partial_rounds_chain_output_limb_26_col218 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[2].get_m31(6);
            *row[218] = poseidon_3_partial_rounds_chain_output_limb_26_col218;let poseidon_3_partial_rounds_chain_output_limb_27_col219 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[2].get_m31(7);
            *row[219] = poseidon_3_partial_rounds_chain_output_limb_27_col219;let poseidon_3_partial_rounds_chain_output_limb_28_col220 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[2].get_m31(8);
            *row[220] = poseidon_3_partial_rounds_chain_output_limb_28_col220;let poseidon_3_partial_rounds_chain_output_limb_29_col221 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[2].get_m31(9);
            *row[221] = poseidon_3_partial_rounds_chain_output_limb_29_col221;let poseidon_3_partial_rounds_chain_output_limb_30_col222 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[3].get_m31(0);
            *row[222] = poseidon_3_partial_rounds_chain_output_limb_30_col222;let poseidon_3_partial_rounds_chain_output_limb_31_col223 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[3].get_m31(1);
            *row[223] = poseidon_3_partial_rounds_chain_output_limb_31_col223;let poseidon_3_partial_rounds_chain_output_limb_32_col224 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[3].get_m31(2);
            *row[224] = poseidon_3_partial_rounds_chain_output_limb_32_col224;let poseidon_3_partial_rounds_chain_output_limb_33_col225 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[3].get_m31(3);
            *row[225] = poseidon_3_partial_rounds_chain_output_limb_33_col225;let poseidon_3_partial_rounds_chain_output_limb_34_col226 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[3].get_m31(4);
            *row[226] = poseidon_3_partial_rounds_chain_output_limb_34_col226;let poseidon_3_partial_rounds_chain_output_limb_35_col227 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[3].get_m31(5);
            *row[227] = poseidon_3_partial_rounds_chain_output_limb_35_col227;let poseidon_3_partial_rounds_chain_output_limb_36_col228 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[3].get_m31(6);
            *row[228] = poseidon_3_partial_rounds_chain_output_limb_36_col228;let poseidon_3_partial_rounds_chain_output_limb_37_col229 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[3].get_m31(7);
            *row[229] = poseidon_3_partial_rounds_chain_output_limb_37_col229;let poseidon_3_partial_rounds_chain_output_limb_38_col230 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[3].get_m31(8);
            *row[230] = poseidon_3_partial_rounds_chain_output_limb_38_col230;let poseidon_3_partial_rounds_chain_output_limb_39_col231 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[3].get_m31(9);
            *row[231] = poseidon_3_partial_rounds_chain_output_limb_39_col231;*lookup_data.poseidon_3_partial_rounds_chain_1 = [seq, M31_31, poseidon_3_partial_rounds_chain_output_limb_0_col192, poseidon_3_partial_rounds_chain_output_limb_1_col193, poseidon_3_partial_rounds_chain_output_limb_2_col194, poseidon_3_partial_rounds_chain_output_limb_3_col195, poseidon_3_partial_rounds_chain_output_limb_4_col196, poseidon_3_partial_rounds_chain_output_limb_5_col197, poseidon_3_partial_rounds_chain_output_limb_6_col198, poseidon_3_partial_rounds_chain_output_limb_7_col199, poseidon_3_partial_rounds_chain_output_limb_8_col200, poseidon_3_partial_rounds_chain_output_limb_9_col201, poseidon_3_partial_rounds_chain_output_limb_10_col202, poseidon_3_partial_rounds_chain_output_limb_11_col203, poseidon_3_partial_rounds_chain_output_limb_12_col204, poseidon_3_partial_rounds_chain_output_limb_13_col205, poseidon_3_partial_rounds_chain_output_limb_14_col206, poseidon_3_partial_rounds_chain_output_limb_15_col207, poseidon_3_partial_rounds_chain_output_limb_16_col208, poseidon_3_partial_rounds_chain_output_limb_17_col209, poseidon_3_partial_rounds_chain_output_limb_18_col210, poseidon_3_partial_rounds_chain_output_limb_19_col211, poseidon_3_partial_rounds_chain_output_limb_20_col212, poseidon_3_partial_rounds_chain_output_limb_21_col213, poseidon_3_partial_rounds_chain_output_limb_22_col214, poseidon_3_partial_rounds_chain_output_limb_23_col215, poseidon_3_partial_rounds_chain_output_limb_24_col216, poseidon_3_partial_rounds_chain_output_limb_25_col217, poseidon_3_partial_rounds_chain_output_limb_26_col218, poseidon_3_partial_rounds_chain_output_limb_27_col219, poseidon_3_partial_rounds_chain_output_limb_28_col220, poseidon_3_partial_rounds_chain_output_limb_29_col221, poseidon_3_partial_rounds_chain_output_limb_30_col222, poseidon_3_partial_rounds_chain_output_limb_31_col223, poseidon_3_partial_rounds_chain_output_limb_32_col224, poseidon_3_partial_rounds_chain_output_limb_33_col225, poseidon_3_partial_rounds_chain_output_limb_34_col226, poseidon_3_partial_rounds_chain_output_limb_35_col227, poseidon_3_partial_rounds_chain_output_limb_36_col228, poseidon_3_partial_rounds_chain_output_limb_37_col229, poseidon_3_partial_rounds_chain_output_limb_38_col230, poseidon_3_partial_rounds_chain_output_limb_39_col231];

            // Linear Combination N 4 Coefs 4 2 1 1.

            let combination_tmp_51986_135 = PackedFelt252Width27::from_packed_felt252(((((((((Felt252_0_0_0_0) + (((Felt252_4_0_0_0) * (PackedFelt252::from_packed_felt252width27(poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[0])))))) + (((Felt252_2_0_0_0) * (PackedFelt252::from_packed_felt252width27(poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[1])))))) + (((Felt252_1_0_0_0) * (PackedFelt252::from_packed_felt252width27(poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[2])))))) + (Felt252_3969818800901670911_10562874008078701503_14906396266795319764_223312371439046257)));let combination_limb_0_col232 = combination_tmp_51986_135.get_m31(0);
            *row[232] = combination_limb_0_col232;let combination_limb_1_col233 = combination_tmp_51986_135.get_m31(1);
            *row[233] = combination_limb_1_col233;let combination_limb_2_col234 = combination_tmp_51986_135.get_m31(2);
            *row[234] = combination_limb_2_col234;let combination_limb_3_col235 = combination_tmp_51986_135.get_m31(3);
            *row[235] = combination_limb_3_col235;let combination_limb_4_col236 = combination_tmp_51986_135.get_m31(4);
            *row[236] = combination_limb_4_col236;let combination_limb_5_col237 = combination_tmp_51986_135.get_m31(5);
            *row[237] = combination_limb_5_col237;let combination_limb_6_col238 = combination_tmp_51986_135.get_m31(6);
            *row[238] = combination_limb_6_col238;let combination_limb_7_col239 = combination_tmp_51986_135.get_m31(7);
            *row[239] = combination_limb_7_col239;let combination_limb_8_col240 = combination_tmp_51986_135.get_m31(8);
            *row[240] = combination_limb_8_col240;let combination_limb_9_col241 = combination_tmp_51986_135.get_m31(9);
            *row[241] = combination_limb_9_col241;let biased_limb_accumulator_u32_tmp_51986_136 = PackedUInt32::from_m31(((((((((((((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_0_col192))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_10_col202))))) + (poseidon_3_partial_rounds_chain_output_limb_20_col212))) + (M31_40454143))) - (combination_limb_0_col232))) + (M31_134217729)));let p_coef_col242 = ((biased_limb_accumulator_u32_tmp_51986_136.low().as_m31()) - (M31_1));
            *row[242] = p_coef_col242;let carry_0_tmp_51986_137 = ((((((((((((((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_0_col192))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_10_col202))))) + (poseidon_3_partial_rounds_chain_output_limb_20_col212))) + (M31_40454143))) - (combination_limb_0_col232))) - (p_coef_col242))) * (M31_16));let carry_1_tmp_51986_138 = ((((((((((((carry_0_tmp_51986_137) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_1_col193))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_11_col203))))) + (poseidon_3_partial_rounds_chain_output_limb_21_col213))) + (M31_49554771))) - (combination_limb_1_col233))) * (M31_16));let carry_2_tmp_51986_139 = ((((((((((((carry_1_tmp_51986_138) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_2_col194))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_12_col204))))) + (poseidon_3_partial_rounds_chain_output_limb_22_col214))) + (M31_55508188))) - (combination_limb_2_col234))) * (M31_16));let carry_3_tmp_51986_140 = ((((((((((((carry_2_tmp_51986_139) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_3_col195))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_13_col205))))) + (poseidon_3_partial_rounds_chain_output_limb_23_col215))) + (M31_116986206))) - (combination_limb_3_col235))) * (M31_16));let carry_4_tmp_51986_141 = ((((((((((((carry_3_tmp_51986_140) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_4_col196))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_14_col206))))) + (poseidon_3_partial_rounds_chain_output_limb_24_col216))) + (M31_88680813))) - (combination_limb_4_col236))) * (M31_16));let carry_5_tmp_51986_142 = ((((((((((((carry_4_tmp_51986_141) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_5_col197))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_15_col207))))) + (poseidon_3_partial_rounds_chain_output_limb_25_col217))) + (M31_45553283))) - (combination_limb_5_col237))) * (M31_16));let carry_6_tmp_51986_143 = ((((((((((((carry_5_tmp_51986_142) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_6_col198))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_16_col208))))) + (poseidon_3_partial_rounds_chain_output_limb_26_col218))) + (M31_62360091))) - (combination_limb_6_col238))) * (M31_16));let carry_7_tmp_51986_144 = ((((((((((((((carry_6_tmp_51986_143) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_7_col199))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_17_col209))))) + (poseidon_3_partial_rounds_chain_output_limb_27_col219))) + (M31_77099918))) - (combination_limb_7_col239))) - (((p_coef_col242) * (M31_136))))) * (M31_16));let carry_8_tmp_51986_145 = ((((((((((((carry_7_tmp_51986_144) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_8_col200))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_18_col210))))) + (poseidon_3_partial_rounds_chain_output_limb_28_col220))) + (M31_22899501))) - (combination_limb_8_col240))) * (M31_16));*sub_component_inputs.range_check_4_4_4_4[2] =
                [((p_coef_col242) + (M31_1)), ((carry_0_tmp_51986_137) + (M31_1)), ((carry_1_tmp_51986_138) + (M31_1)), ((carry_2_tmp_51986_139) + (M31_1))];
            *lookup_data.range_check_4_4_4_4_2 = [((p_coef_col242) + (M31_1)), ((carry_0_tmp_51986_137) + (M31_1)), ((carry_1_tmp_51986_138) + (M31_1)), ((carry_2_tmp_51986_139) + (M31_1))];*sub_component_inputs.range_check_4_4_4_4[3] =
                [((carry_3_tmp_51986_140) + (M31_1)), ((carry_4_tmp_51986_141) + (M31_1)), ((carry_5_tmp_51986_142) + (M31_1)), ((carry_6_tmp_51986_143) + (M31_1))];
            *lookup_data.range_check_4_4_4_4_3 = [((carry_3_tmp_51986_140) + (M31_1)), ((carry_4_tmp_51986_141) + (M31_1)), ((carry_5_tmp_51986_142) + (M31_1)), ((carry_6_tmp_51986_143) + (M31_1))];*sub_component_inputs.range_check_4_4[1] =
                [((carry_7_tmp_51986_144) + (M31_1)), ((carry_8_tmp_51986_145) + (M31_1))];
            *lookup_data.range_check_4_4_1 = [((carry_7_tmp_51986_144) + (M31_1)), ((carry_8_tmp_51986_145) + (M31_1))];let linear_combination_n_4_coefs_4_2_1_1_output_tmp_51986_146 = combination_tmp_51986_135;

            // Linear Combination N 4 Coefs 4 2 1 1.

            let combination_tmp_51986_147 = PackedFelt252Width27::from_packed_felt252(((((((((Felt252_0_0_0_0) + (((Felt252_4_0_0_0) * (PackedFelt252::from_packed_felt252width27(poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[2])))))) + (((Felt252_2_0_0_0) * (PackedFelt252::from_packed_felt252width27(poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[3])))))) + (((Felt252_1_0_0_0) * (PackedFelt252::from_packed_felt252width27(linear_combination_n_4_coefs_4_2_1_1_output_tmp_51986_146)))))) + (Felt252_10310704347937391837_5874215448258336115_2880320859071049537_45350836576946303)));let combination_limb_0_col243 = combination_tmp_51986_147.get_m31(0);
            *row[243] = combination_limb_0_col243;let combination_limb_1_col244 = combination_tmp_51986_147.get_m31(1);
            *row[244] = combination_limb_1_col244;let combination_limb_2_col245 = combination_tmp_51986_147.get_m31(2);
            *row[245] = combination_limb_2_col245;let combination_limb_3_col246 = combination_tmp_51986_147.get_m31(3);
            *row[246] = combination_limb_3_col246;let combination_limb_4_col247 = combination_tmp_51986_147.get_m31(4);
            *row[247] = combination_limb_4_col247;let combination_limb_5_col248 = combination_tmp_51986_147.get_m31(5);
            *row[248] = combination_limb_5_col248;let combination_limb_6_col249 = combination_tmp_51986_147.get_m31(6);
            *row[249] = combination_limb_6_col249;let combination_limb_7_col250 = combination_tmp_51986_147.get_m31(7);
            *row[250] = combination_limb_7_col250;let combination_limb_8_col251 = combination_tmp_51986_147.get_m31(8);
            *row[251] = combination_limb_8_col251;let combination_limb_9_col252 = combination_tmp_51986_147.get_m31(9);
            *row[252] = combination_limb_9_col252;let biased_limb_accumulator_u32_tmp_51986_148 = PackedUInt32::from_m31(((((((((((((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_20_col212))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_30_col222))))) + (combination_limb_0_col232))) + (M31_48383197))) - (combination_limb_0_col243))) + (M31_134217729)));let p_coef_col253 = ((biased_limb_accumulator_u32_tmp_51986_148.low().as_m31()) - (M31_1));
            *row[253] = p_coef_col253;let carry_0_tmp_51986_149 = ((((((((((((((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_20_col212))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_30_col222))))) + (combination_limb_0_col232))) + (M31_48383197))) - (combination_limb_0_col243))) - (p_coef_col253))) * (M31_16));let carry_1_tmp_51986_150 = ((((((((((((carry_0_tmp_51986_149) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_21_col213))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_31_col223))))) + (combination_limb_1_col233))) + (M31_48193339))) - (combination_limb_1_col244))) * (M31_16));let carry_2_tmp_51986_151 = ((((((((((((carry_1_tmp_51986_150) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_22_col214))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_32_col224))))) + (combination_limb_2_col234))) + (M31_55955004))) - (combination_limb_2_col245))) * (M31_16));let carry_3_tmp_51986_152 = ((((((((((((carry_2_tmp_51986_151) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_23_col215))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_33_col225))))) + (combination_limb_3_col235))) + (M31_65659846))) - (combination_limb_3_col246))) * (M31_16));let carry_4_tmp_51986_153 = ((((((((((((carry_3_tmp_51986_152) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_24_col216))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_34_col226))))) + (combination_limb_4_col236))) + (M31_68491350))) - (combination_limb_4_col247))) * (M31_16));let carry_5_tmp_51986_154 = ((((((((((((carry_4_tmp_51986_153) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_25_col217))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_35_col227))))) + (combination_limb_5_col237))) + (M31_119023582))) - (combination_limb_5_col248))) * (M31_16));let carry_6_tmp_51986_155 = ((((((((((((carry_5_tmp_51986_154) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_26_col218))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_36_col228))))) + (combination_limb_6_col238))) + (M31_33439011))) - (combination_limb_6_col249))) * (M31_16));let carry_7_tmp_51986_156 = ((((((((((((((carry_6_tmp_51986_155) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_27_col219))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_37_col229))))) + (combination_limb_7_col239))) + (M31_58475513))) - (combination_limb_7_col250))) - (((p_coef_col253) * (M31_136))))) * (M31_16));let carry_8_tmp_51986_157 = ((((((((((((carry_7_tmp_51986_156) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_28_col220))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_38_col230))))) + (combination_limb_8_col240))) + (M31_18765944))) - (combination_limb_8_col251))) * (M31_16));*sub_component_inputs.range_check_4_4_4_4[4] =
                [((p_coef_col253) + (M31_1)), ((carry_0_tmp_51986_149) + (M31_1)), ((carry_1_tmp_51986_150) + (M31_1)), ((carry_2_tmp_51986_151) + (M31_1))];
            *lookup_data.range_check_4_4_4_4_4 = [((p_coef_col253) + (M31_1)), ((carry_0_tmp_51986_149) + (M31_1)), ((carry_1_tmp_51986_150) + (M31_1)), ((carry_2_tmp_51986_151) + (M31_1))];*sub_component_inputs.range_check_4_4_4_4[5] =
                [((carry_3_tmp_51986_152) + (M31_1)), ((carry_4_tmp_51986_153) + (M31_1)), ((carry_5_tmp_51986_154) + (M31_1)), ((carry_6_tmp_51986_155) + (M31_1))];
            *lookup_data.range_check_4_4_4_4_5 = [((carry_3_tmp_51986_152) + (M31_1)), ((carry_4_tmp_51986_153) + (M31_1)), ((carry_5_tmp_51986_154) + (M31_1)), ((carry_6_tmp_51986_155) + (M31_1))];*sub_component_inputs.range_check_4_4[2] =
                [((carry_7_tmp_51986_156) + (M31_1)), ((carry_8_tmp_51986_157) + (M31_1))];
            *lookup_data.range_check_4_4_2 = [((carry_7_tmp_51986_156) + (M31_1)), ((carry_8_tmp_51986_157) + (M31_1))];let linear_combination_n_4_coefs_4_2_1_1_output_tmp_51986_158 = combination_tmp_51986_147;

            *lookup_data.poseidon_full_round_chain_2 = [((((seq) * (M31_2))) + (M31_1)), M31_31, combination_limb_0_col243, combination_limb_1_col244, combination_limb_2_col245, combination_limb_3_col246, combination_limb_4_col247, combination_limb_5_col248, combination_limb_6_col249, combination_limb_7_col250, combination_limb_8_col251, combination_limb_9_col252, combination_limb_0_col232, combination_limb_1_col233, combination_limb_2_col234, combination_limb_3_col235, combination_limb_4_col236, combination_limb_5_col237, combination_limb_6_col238, combination_limb_7_col239, combination_limb_8_col240, combination_limb_9_col241, poseidon_3_partial_rounds_chain_output_limb_30_col222, poseidon_3_partial_rounds_chain_output_limb_31_col223, poseidon_3_partial_rounds_chain_output_limb_32_col224, poseidon_3_partial_rounds_chain_output_limb_33_col225, poseidon_3_partial_rounds_chain_output_limb_34_col226, poseidon_3_partial_rounds_chain_output_limb_35_col227, poseidon_3_partial_rounds_chain_output_limb_36_col228, poseidon_3_partial_rounds_chain_output_limb_37_col229, poseidon_3_partial_rounds_chain_output_limb_38_col230, poseidon_3_partial_rounds_chain_output_limb_39_col231];*sub_component_inputs.poseidon_full_round_chain[4] =
                (((((seq) * (M31_2))) + (M31_1)), M31_31, [linear_combination_n_4_coefs_4_2_1_1_output_tmp_51986_158, linear_combination_n_4_coefs_4_2_1_1_output_tmp_51986_146, poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[3]]);
            let poseidon_full_round_chain_output_round_31_tmp_51986_159 = PackedPoseidonFullRoundChain::deduce_output((((((seq) * (M31_2))) + (M31_1)), M31_31, [linear_combination_n_4_coefs_4_2_1_1_output_tmp_51986_158, linear_combination_n_4_coefs_4_2_1_1_output_tmp_51986_146, poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_134.2[3]]));*sub_component_inputs.poseidon_full_round_chain[5] =
                (((((seq) * (M31_2))) + (M31_1)), M31_32, [poseidon_full_round_chain_output_round_31_tmp_51986_159.2[0], poseidon_full_round_chain_output_round_31_tmp_51986_159.2[1], poseidon_full_round_chain_output_round_31_tmp_51986_159.2[2]]);
            let poseidon_full_round_chain_output_round_32_tmp_51986_160 = PackedPoseidonFullRoundChain::deduce_output((((((seq) * (M31_2))) + (M31_1)), M31_32, [poseidon_full_round_chain_output_round_31_tmp_51986_159.2[0], poseidon_full_round_chain_output_round_31_tmp_51986_159.2[1], poseidon_full_round_chain_output_round_31_tmp_51986_159.2[2]]));*sub_component_inputs.poseidon_full_round_chain[6] =
                (((((seq) * (M31_2))) + (M31_1)), M31_33, [poseidon_full_round_chain_output_round_32_tmp_51986_160.2[0], poseidon_full_round_chain_output_round_32_tmp_51986_160.2[1], poseidon_full_round_chain_output_round_32_tmp_51986_160.2[2]]);
            let poseidon_full_round_chain_output_round_33_tmp_51986_161 = PackedPoseidonFullRoundChain::deduce_output((((((seq) * (M31_2))) + (M31_1)), M31_33, [poseidon_full_round_chain_output_round_32_tmp_51986_160.2[0], poseidon_full_round_chain_output_round_32_tmp_51986_160.2[1], poseidon_full_round_chain_output_round_32_tmp_51986_160.2[2]]));*sub_component_inputs.poseidon_full_round_chain[7] =
                (((((seq) * (M31_2))) + (M31_1)), M31_34, [poseidon_full_round_chain_output_round_33_tmp_51986_161.2[0], poseidon_full_round_chain_output_round_33_tmp_51986_161.2[1], poseidon_full_round_chain_output_round_33_tmp_51986_161.2[2]]);
            let poseidon_full_round_chain_output_round_34_tmp_51986_162 = PackedPoseidonFullRoundChain::deduce_output((((((seq) * (M31_2))) + (M31_1)), M31_34, [poseidon_full_round_chain_output_round_33_tmp_51986_161.2[0], poseidon_full_round_chain_output_round_33_tmp_51986_161.2[1], poseidon_full_round_chain_output_round_33_tmp_51986_161.2[2]]));let poseidon_full_round_chain_output_limb_0_col254 = poseidon_full_round_chain_output_round_34_tmp_51986_162.2[0].get_m31(0);
            *row[254] = poseidon_full_round_chain_output_limb_0_col254;let poseidon_full_round_chain_output_limb_1_col255 = poseidon_full_round_chain_output_round_34_tmp_51986_162.2[0].get_m31(1);
            *row[255] = poseidon_full_round_chain_output_limb_1_col255;let poseidon_full_round_chain_output_limb_2_col256 = poseidon_full_round_chain_output_round_34_tmp_51986_162.2[0].get_m31(2);
            *row[256] = poseidon_full_round_chain_output_limb_2_col256;let poseidon_full_round_chain_output_limb_3_col257 = poseidon_full_round_chain_output_round_34_tmp_51986_162.2[0].get_m31(3);
            *row[257] = poseidon_full_round_chain_output_limb_3_col257;let poseidon_full_round_chain_output_limb_4_col258 = poseidon_full_round_chain_output_round_34_tmp_51986_162.2[0].get_m31(4);
            *row[258] = poseidon_full_round_chain_output_limb_4_col258;let poseidon_full_round_chain_output_limb_5_col259 = poseidon_full_round_chain_output_round_34_tmp_51986_162.2[0].get_m31(5);
            *row[259] = poseidon_full_round_chain_output_limb_5_col259;let poseidon_full_round_chain_output_limb_6_col260 = poseidon_full_round_chain_output_round_34_tmp_51986_162.2[0].get_m31(6);
            *row[260] = poseidon_full_round_chain_output_limb_6_col260;let poseidon_full_round_chain_output_limb_7_col261 = poseidon_full_round_chain_output_round_34_tmp_51986_162.2[0].get_m31(7);
            *row[261] = poseidon_full_round_chain_output_limb_7_col261;let poseidon_full_round_chain_output_limb_8_col262 = poseidon_full_round_chain_output_round_34_tmp_51986_162.2[0].get_m31(8);
            *row[262] = poseidon_full_round_chain_output_limb_8_col262;let poseidon_full_round_chain_output_limb_9_col263 = poseidon_full_round_chain_output_round_34_tmp_51986_162.2[0].get_m31(9);
            *row[263] = poseidon_full_round_chain_output_limb_9_col263;let poseidon_full_round_chain_output_limb_10_col264 = poseidon_full_round_chain_output_round_34_tmp_51986_162.2[1].get_m31(0);
            *row[264] = poseidon_full_round_chain_output_limb_10_col264;let poseidon_full_round_chain_output_limb_11_col265 = poseidon_full_round_chain_output_round_34_tmp_51986_162.2[1].get_m31(1);
            *row[265] = poseidon_full_round_chain_output_limb_11_col265;let poseidon_full_round_chain_output_limb_12_col266 = poseidon_full_round_chain_output_round_34_tmp_51986_162.2[1].get_m31(2);
            *row[266] = poseidon_full_round_chain_output_limb_12_col266;let poseidon_full_round_chain_output_limb_13_col267 = poseidon_full_round_chain_output_round_34_tmp_51986_162.2[1].get_m31(3);
            *row[267] = poseidon_full_round_chain_output_limb_13_col267;let poseidon_full_round_chain_output_limb_14_col268 = poseidon_full_round_chain_output_round_34_tmp_51986_162.2[1].get_m31(4);
            *row[268] = poseidon_full_round_chain_output_limb_14_col268;let poseidon_full_round_chain_output_limb_15_col269 = poseidon_full_round_chain_output_round_34_tmp_51986_162.2[1].get_m31(5);
            *row[269] = poseidon_full_round_chain_output_limb_15_col269;let poseidon_full_round_chain_output_limb_16_col270 = poseidon_full_round_chain_output_round_34_tmp_51986_162.2[1].get_m31(6);
            *row[270] = poseidon_full_round_chain_output_limb_16_col270;let poseidon_full_round_chain_output_limb_17_col271 = poseidon_full_round_chain_output_round_34_tmp_51986_162.2[1].get_m31(7);
            *row[271] = poseidon_full_round_chain_output_limb_17_col271;let poseidon_full_round_chain_output_limb_18_col272 = poseidon_full_round_chain_output_round_34_tmp_51986_162.2[1].get_m31(8);
            *row[272] = poseidon_full_round_chain_output_limb_18_col272;let poseidon_full_round_chain_output_limb_19_col273 = poseidon_full_round_chain_output_round_34_tmp_51986_162.2[1].get_m31(9);
            *row[273] = poseidon_full_round_chain_output_limb_19_col273;let poseidon_full_round_chain_output_limb_20_col274 = poseidon_full_round_chain_output_round_34_tmp_51986_162.2[2].get_m31(0);
            *row[274] = poseidon_full_round_chain_output_limb_20_col274;let poseidon_full_round_chain_output_limb_21_col275 = poseidon_full_round_chain_output_round_34_tmp_51986_162.2[2].get_m31(1);
            *row[275] = poseidon_full_round_chain_output_limb_21_col275;let poseidon_full_round_chain_output_limb_22_col276 = poseidon_full_round_chain_output_round_34_tmp_51986_162.2[2].get_m31(2);
            *row[276] = poseidon_full_round_chain_output_limb_22_col276;let poseidon_full_round_chain_output_limb_23_col277 = poseidon_full_round_chain_output_round_34_tmp_51986_162.2[2].get_m31(3);
            *row[277] = poseidon_full_round_chain_output_limb_23_col277;let poseidon_full_round_chain_output_limb_24_col278 = poseidon_full_round_chain_output_round_34_tmp_51986_162.2[2].get_m31(4);
            *row[278] = poseidon_full_round_chain_output_limb_24_col278;let poseidon_full_round_chain_output_limb_25_col279 = poseidon_full_round_chain_output_round_34_tmp_51986_162.2[2].get_m31(5);
            *row[279] = poseidon_full_round_chain_output_limb_25_col279;let poseidon_full_round_chain_output_limb_26_col280 = poseidon_full_round_chain_output_round_34_tmp_51986_162.2[2].get_m31(6);
            *row[280] = poseidon_full_round_chain_output_limb_26_col280;let poseidon_full_round_chain_output_limb_27_col281 = poseidon_full_round_chain_output_round_34_tmp_51986_162.2[2].get_m31(7);
            *row[281] = poseidon_full_round_chain_output_limb_27_col281;let poseidon_full_round_chain_output_limb_28_col282 = poseidon_full_round_chain_output_round_34_tmp_51986_162.2[2].get_m31(8);
            *row[282] = poseidon_full_round_chain_output_limb_28_col282;let poseidon_full_round_chain_output_limb_29_col283 = poseidon_full_round_chain_output_round_34_tmp_51986_162.2[2].get_m31(9);
            *row[283] = poseidon_full_round_chain_output_limb_29_col283;*lookup_data.poseidon_full_round_chain_3 = [((((seq) * (M31_2))) + (M31_1)), M31_35, poseidon_full_round_chain_output_limb_0_col254, poseidon_full_round_chain_output_limb_1_col255, poseidon_full_round_chain_output_limb_2_col256, poseidon_full_round_chain_output_limb_3_col257, poseidon_full_round_chain_output_limb_4_col258, poseidon_full_round_chain_output_limb_5_col259, poseidon_full_round_chain_output_limb_6_col260, poseidon_full_round_chain_output_limb_7_col261, poseidon_full_round_chain_output_limb_8_col262, poseidon_full_round_chain_output_limb_9_col263, poseidon_full_round_chain_output_limb_10_col264, poseidon_full_round_chain_output_limb_11_col265, poseidon_full_round_chain_output_limb_12_col266, poseidon_full_round_chain_output_limb_13_col267, poseidon_full_round_chain_output_limb_14_col268, poseidon_full_round_chain_output_limb_15_col269, poseidon_full_round_chain_output_limb_16_col270, poseidon_full_round_chain_output_limb_17_col271, poseidon_full_round_chain_output_limb_18_col272, poseidon_full_round_chain_output_limb_19_col273, poseidon_full_round_chain_output_limb_20_col274, poseidon_full_round_chain_output_limb_21_col275, poseidon_full_round_chain_output_limb_22_col276, poseidon_full_round_chain_output_limb_23_col277, poseidon_full_round_chain_output_limb_24_col278, poseidon_full_round_chain_output_limb_25_col279, poseidon_full_round_chain_output_limb_26_col280, poseidon_full_round_chain_output_limb_27_col281, poseidon_full_round_chain_output_limb_28_col282, poseidon_full_round_chain_output_limb_29_col283];let poseidon_hades_permutation_output_tmp_51986_163 = [poseidon_full_round_chain_output_round_34_tmp_51986_162.2[0], poseidon_full_round_chain_output_round_34_tmp_51986_162.2[1], poseidon_full_round_chain_output_round_34_tmp_51986_162.2[2]];

            // Felt 252 Unpack From 27.

            let input_as_felt252_tmp_51986_164 = PackedFelt252::from_packed_felt252width27(poseidon_hades_permutation_output_tmp_51986_163[0]);let unpacked_limb_0_col284 = input_as_felt252_tmp_51986_164.get_m31(0);
            *row[284] = unpacked_limb_0_col284;let unpacked_limb_1_col285 = input_as_felt252_tmp_51986_164.get_m31(1);
            *row[285] = unpacked_limb_1_col285;let unpacked_limb_3_col286 = input_as_felt252_tmp_51986_164.get_m31(3);
            *row[286] = unpacked_limb_3_col286;let unpacked_limb_4_col287 = input_as_felt252_tmp_51986_164.get_m31(4);
            *row[287] = unpacked_limb_4_col287;let unpacked_limb_6_col288 = input_as_felt252_tmp_51986_164.get_m31(6);
            *row[288] = unpacked_limb_6_col288;let unpacked_limb_7_col289 = input_as_felt252_tmp_51986_164.get_m31(7);
            *row[289] = unpacked_limb_7_col289;let unpacked_limb_9_col290 = input_as_felt252_tmp_51986_164.get_m31(9);
            *row[290] = unpacked_limb_9_col290;let unpacked_limb_10_col291 = input_as_felt252_tmp_51986_164.get_m31(10);
            *row[291] = unpacked_limb_10_col291;let unpacked_limb_12_col292 = input_as_felt252_tmp_51986_164.get_m31(12);
            *row[292] = unpacked_limb_12_col292;let unpacked_limb_13_col293 = input_as_felt252_tmp_51986_164.get_m31(13);
            *row[293] = unpacked_limb_13_col293;let unpacked_limb_15_col294 = input_as_felt252_tmp_51986_164.get_m31(15);
            *row[294] = unpacked_limb_15_col294;let unpacked_limb_16_col295 = input_as_felt252_tmp_51986_164.get_m31(16);
            *row[295] = unpacked_limb_16_col295;let unpacked_limb_18_col296 = input_as_felt252_tmp_51986_164.get_m31(18);
            *row[296] = unpacked_limb_18_col296;let unpacked_limb_19_col297 = input_as_felt252_tmp_51986_164.get_m31(19);
            *row[297] = unpacked_limb_19_col297;let unpacked_limb_21_col298 = input_as_felt252_tmp_51986_164.get_m31(21);
            *row[298] = unpacked_limb_21_col298;let unpacked_limb_22_col299 = input_as_felt252_tmp_51986_164.get_m31(22);
            *row[299] = unpacked_limb_22_col299;let unpacked_limb_24_col300 = input_as_felt252_tmp_51986_164.get_m31(24);
            *row[300] = unpacked_limb_24_col300;let unpacked_limb_25_col301 = input_as_felt252_tmp_51986_164.get_m31(25);
            *row[301] = unpacked_limb_25_col301;let felt_252_unpack_from_27_output_tmp_51986_165 = PackedFelt252::from_limbs([unpacked_limb_0_col284, unpacked_limb_1_col285, ((((((poseidon_full_round_chain_output_limb_0_col254) - (unpacked_limb_0_col284))) - (((unpacked_limb_1_col285) * (M31_512))))) * (M31_8192)), unpacked_limb_3_col286, unpacked_limb_4_col287, ((((((poseidon_full_round_chain_output_limb_1_col255) - (unpacked_limb_3_col286))) - (((unpacked_limb_4_col287) * (M31_512))))) * (M31_8192)), unpacked_limb_6_col288, unpacked_limb_7_col289, ((((((poseidon_full_round_chain_output_limb_2_col256) - (unpacked_limb_6_col288))) - (((unpacked_limb_7_col289) * (M31_512))))) * (M31_8192)), unpacked_limb_9_col290, unpacked_limb_10_col291, ((((((poseidon_full_round_chain_output_limb_3_col257) - (unpacked_limb_9_col290))) - (((unpacked_limb_10_col291) * (M31_512))))) * (M31_8192)), unpacked_limb_12_col292, unpacked_limb_13_col293, ((((((poseidon_full_round_chain_output_limb_4_col258) - (unpacked_limb_12_col292))) - (((unpacked_limb_13_col293) * (M31_512))))) * (M31_8192)), unpacked_limb_15_col294, unpacked_limb_16_col295, ((((((poseidon_full_round_chain_output_limb_5_col259) - (unpacked_limb_15_col294))) - (((unpacked_limb_16_col295) * (M31_512))))) * (M31_8192)), unpacked_limb_18_col296, unpacked_limb_19_col297, ((((((poseidon_full_round_chain_output_limb_6_col260) - (unpacked_limb_18_col296))) - (((unpacked_limb_19_col297) * (M31_512))))) * (M31_8192)), unpacked_limb_21_col298, unpacked_limb_22_col299, ((((((poseidon_full_round_chain_output_limb_7_col261) - (unpacked_limb_21_col298))) - (((unpacked_limb_22_col299) * (M31_512))))) * (M31_8192)), unpacked_limb_24_col300, unpacked_limb_25_col301, ((((((poseidon_full_round_chain_output_limb_8_col262) - (unpacked_limb_24_col300))) - (((unpacked_limb_25_col301) * (M31_512))))) * (M31_8192)), poseidon_full_round_chain_output_limb_9_col263]);

            // Mem Verify.

            let memory_address_to_id_value_tmp_51986_166 = memory_address_to_id_state.deduce_output(((((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start))) + (((seq) * (M31_6))))) + (M31_3)));let output_state_0_id_col302 = memory_address_to_id_value_tmp_51986_166;
            *row[302] = output_state_0_id_col302;*sub_component_inputs.memory_address_to_id[3] =
                ((((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start))) + (((seq) * (M31_6))))) + (M31_3));
            *lookup_data.memory_address_to_id_3 = [((((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start))) + (((seq) * (M31_6))))) + (M31_3)), output_state_0_id_col302];*sub_component_inputs.memory_id_to_big[3] =
                output_state_0_id_col302;
            *lookup_data.memory_id_to_big_3 = [output_state_0_id_col302, unpacked_limb_0_col284, unpacked_limb_1_col285, felt_252_unpack_from_27_output_tmp_51986_165.get_m31(2), unpacked_limb_3_col286, unpacked_limb_4_col287, felt_252_unpack_from_27_output_tmp_51986_165.get_m31(5), unpacked_limb_6_col288, unpacked_limb_7_col289, felt_252_unpack_from_27_output_tmp_51986_165.get_m31(8), unpacked_limb_9_col290, unpacked_limb_10_col291, felt_252_unpack_from_27_output_tmp_51986_165.get_m31(11), unpacked_limb_12_col292, unpacked_limb_13_col293, felt_252_unpack_from_27_output_tmp_51986_165.get_m31(14), unpacked_limb_15_col294, unpacked_limb_16_col295, felt_252_unpack_from_27_output_tmp_51986_165.get_m31(17), unpacked_limb_18_col296, unpacked_limb_19_col297, felt_252_unpack_from_27_output_tmp_51986_165.get_m31(20), unpacked_limb_21_col298, unpacked_limb_22_col299, felt_252_unpack_from_27_output_tmp_51986_165.get_m31(23), unpacked_limb_24_col300, unpacked_limb_25_col301, felt_252_unpack_from_27_output_tmp_51986_165.get_m31(26), poseidon_full_round_chain_output_limb_9_col263];

            // Felt 252 Unpack From 27.

            let input_as_felt252_tmp_51986_167 = PackedFelt252::from_packed_felt252width27(poseidon_hades_permutation_output_tmp_51986_163[1]);let unpacked_limb_0_col303 = input_as_felt252_tmp_51986_167.get_m31(0);
            *row[303] = unpacked_limb_0_col303;let unpacked_limb_1_col304 = input_as_felt252_tmp_51986_167.get_m31(1);
            *row[304] = unpacked_limb_1_col304;let unpacked_limb_3_col305 = input_as_felt252_tmp_51986_167.get_m31(3);
            *row[305] = unpacked_limb_3_col305;let unpacked_limb_4_col306 = input_as_felt252_tmp_51986_167.get_m31(4);
            *row[306] = unpacked_limb_4_col306;let unpacked_limb_6_col307 = input_as_felt252_tmp_51986_167.get_m31(6);
            *row[307] = unpacked_limb_6_col307;let unpacked_limb_7_col308 = input_as_felt252_tmp_51986_167.get_m31(7);
            *row[308] = unpacked_limb_7_col308;let unpacked_limb_9_col309 = input_as_felt252_tmp_51986_167.get_m31(9);
            *row[309] = unpacked_limb_9_col309;let unpacked_limb_10_col310 = input_as_felt252_tmp_51986_167.get_m31(10);
            *row[310] = unpacked_limb_10_col310;let unpacked_limb_12_col311 = input_as_felt252_tmp_51986_167.get_m31(12);
            *row[311] = unpacked_limb_12_col311;let unpacked_limb_13_col312 = input_as_felt252_tmp_51986_167.get_m31(13);
            *row[312] = unpacked_limb_13_col312;let unpacked_limb_15_col313 = input_as_felt252_tmp_51986_167.get_m31(15);
            *row[313] = unpacked_limb_15_col313;let unpacked_limb_16_col314 = input_as_felt252_tmp_51986_167.get_m31(16);
            *row[314] = unpacked_limb_16_col314;let unpacked_limb_18_col315 = input_as_felt252_tmp_51986_167.get_m31(18);
            *row[315] = unpacked_limb_18_col315;let unpacked_limb_19_col316 = input_as_felt252_tmp_51986_167.get_m31(19);
            *row[316] = unpacked_limb_19_col316;let unpacked_limb_21_col317 = input_as_felt252_tmp_51986_167.get_m31(21);
            *row[317] = unpacked_limb_21_col317;let unpacked_limb_22_col318 = input_as_felt252_tmp_51986_167.get_m31(22);
            *row[318] = unpacked_limb_22_col318;let unpacked_limb_24_col319 = input_as_felt252_tmp_51986_167.get_m31(24);
            *row[319] = unpacked_limb_24_col319;let unpacked_limb_25_col320 = input_as_felt252_tmp_51986_167.get_m31(25);
            *row[320] = unpacked_limb_25_col320;let felt_252_unpack_from_27_output_tmp_51986_168 = PackedFelt252::from_limbs([unpacked_limb_0_col303, unpacked_limb_1_col304, ((((((poseidon_full_round_chain_output_limb_10_col264) - (unpacked_limb_0_col303))) - (((unpacked_limb_1_col304) * (M31_512))))) * (M31_8192)), unpacked_limb_3_col305, unpacked_limb_4_col306, ((((((poseidon_full_round_chain_output_limb_11_col265) - (unpacked_limb_3_col305))) - (((unpacked_limb_4_col306) * (M31_512))))) * (M31_8192)), unpacked_limb_6_col307, unpacked_limb_7_col308, ((((((poseidon_full_round_chain_output_limb_12_col266) - (unpacked_limb_6_col307))) - (((unpacked_limb_7_col308) * (M31_512))))) * (M31_8192)), unpacked_limb_9_col309, unpacked_limb_10_col310, ((((((poseidon_full_round_chain_output_limb_13_col267) - (unpacked_limb_9_col309))) - (((unpacked_limb_10_col310) * (M31_512))))) * (M31_8192)), unpacked_limb_12_col311, unpacked_limb_13_col312, ((((((poseidon_full_round_chain_output_limb_14_col268) - (unpacked_limb_12_col311))) - (((unpacked_limb_13_col312) * (M31_512))))) * (M31_8192)), unpacked_limb_15_col313, unpacked_limb_16_col314, ((((((poseidon_full_round_chain_output_limb_15_col269) - (unpacked_limb_15_col313))) - (((unpacked_limb_16_col314) * (M31_512))))) * (M31_8192)), unpacked_limb_18_col315, unpacked_limb_19_col316, ((((((poseidon_full_round_chain_output_limb_16_col270) - (unpacked_limb_18_col315))) - (((unpacked_limb_19_col316) * (M31_512))))) * (M31_8192)), unpacked_limb_21_col317, unpacked_limb_22_col318, ((((((poseidon_full_round_chain_output_limb_17_col271) - (unpacked_limb_21_col317))) - (((unpacked_limb_22_col318) * (M31_512))))) * (M31_8192)), unpacked_limb_24_col319, unpacked_limb_25_col320, ((((((poseidon_full_round_chain_output_limb_18_col272) - (unpacked_limb_24_col319))) - (((unpacked_limb_25_col320) * (M31_512))))) * (M31_8192)), poseidon_full_round_chain_output_limb_19_col273]);

            // Mem Verify.

            let memory_address_to_id_value_tmp_51986_169 = memory_address_to_id_state.deduce_output(((((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start))) + (((seq) * (M31_6))))) + (M31_4)));let output_state_1_id_col321 = memory_address_to_id_value_tmp_51986_169;
            *row[321] = output_state_1_id_col321;*sub_component_inputs.memory_address_to_id[4] =
                ((((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start))) + (((seq) * (M31_6))))) + (M31_4));
            *lookup_data.memory_address_to_id_4 = [((((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start))) + (((seq) * (M31_6))))) + (M31_4)), output_state_1_id_col321];*sub_component_inputs.memory_id_to_big[4] =
                output_state_1_id_col321;
            *lookup_data.memory_id_to_big_4 = [output_state_1_id_col321, unpacked_limb_0_col303, unpacked_limb_1_col304, felt_252_unpack_from_27_output_tmp_51986_168.get_m31(2), unpacked_limb_3_col305, unpacked_limb_4_col306, felt_252_unpack_from_27_output_tmp_51986_168.get_m31(5), unpacked_limb_6_col307, unpacked_limb_7_col308, felt_252_unpack_from_27_output_tmp_51986_168.get_m31(8), unpacked_limb_9_col309, unpacked_limb_10_col310, felt_252_unpack_from_27_output_tmp_51986_168.get_m31(11), unpacked_limb_12_col311, unpacked_limb_13_col312, felt_252_unpack_from_27_output_tmp_51986_168.get_m31(14), unpacked_limb_15_col313, unpacked_limb_16_col314, felt_252_unpack_from_27_output_tmp_51986_168.get_m31(17), unpacked_limb_18_col315, unpacked_limb_19_col316, felt_252_unpack_from_27_output_tmp_51986_168.get_m31(20), unpacked_limb_21_col317, unpacked_limb_22_col318, felt_252_unpack_from_27_output_tmp_51986_168.get_m31(23), unpacked_limb_24_col319, unpacked_limb_25_col320, felt_252_unpack_from_27_output_tmp_51986_168.get_m31(26), poseidon_full_round_chain_output_limb_19_col273];

            // Felt 252 Unpack From 27.

            let input_as_felt252_tmp_51986_170 = PackedFelt252::from_packed_felt252width27(poseidon_hades_permutation_output_tmp_51986_163[2]);let unpacked_limb_0_col322 = input_as_felt252_tmp_51986_170.get_m31(0);
            *row[322] = unpacked_limb_0_col322;let unpacked_limb_1_col323 = input_as_felt252_tmp_51986_170.get_m31(1);
            *row[323] = unpacked_limb_1_col323;let unpacked_limb_3_col324 = input_as_felt252_tmp_51986_170.get_m31(3);
            *row[324] = unpacked_limb_3_col324;let unpacked_limb_4_col325 = input_as_felt252_tmp_51986_170.get_m31(4);
            *row[325] = unpacked_limb_4_col325;let unpacked_limb_6_col326 = input_as_felt252_tmp_51986_170.get_m31(6);
            *row[326] = unpacked_limb_6_col326;let unpacked_limb_7_col327 = input_as_felt252_tmp_51986_170.get_m31(7);
            *row[327] = unpacked_limb_7_col327;let unpacked_limb_9_col328 = input_as_felt252_tmp_51986_170.get_m31(9);
            *row[328] = unpacked_limb_9_col328;let unpacked_limb_10_col329 = input_as_felt252_tmp_51986_170.get_m31(10);
            *row[329] = unpacked_limb_10_col329;let unpacked_limb_12_col330 = input_as_felt252_tmp_51986_170.get_m31(12);
            *row[330] = unpacked_limb_12_col330;let unpacked_limb_13_col331 = input_as_felt252_tmp_51986_170.get_m31(13);
            *row[331] = unpacked_limb_13_col331;let unpacked_limb_15_col332 = input_as_felt252_tmp_51986_170.get_m31(15);
            *row[332] = unpacked_limb_15_col332;let unpacked_limb_16_col333 = input_as_felt252_tmp_51986_170.get_m31(16);
            *row[333] = unpacked_limb_16_col333;let unpacked_limb_18_col334 = input_as_felt252_tmp_51986_170.get_m31(18);
            *row[334] = unpacked_limb_18_col334;let unpacked_limb_19_col335 = input_as_felt252_tmp_51986_170.get_m31(19);
            *row[335] = unpacked_limb_19_col335;let unpacked_limb_21_col336 = input_as_felt252_tmp_51986_170.get_m31(21);
            *row[336] = unpacked_limb_21_col336;let unpacked_limb_22_col337 = input_as_felt252_tmp_51986_170.get_m31(22);
            *row[337] = unpacked_limb_22_col337;let unpacked_limb_24_col338 = input_as_felt252_tmp_51986_170.get_m31(24);
            *row[338] = unpacked_limb_24_col338;let unpacked_limb_25_col339 = input_as_felt252_tmp_51986_170.get_m31(25);
            *row[339] = unpacked_limb_25_col339;let felt_252_unpack_from_27_output_tmp_51986_171 = PackedFelt252::from_limbs([unpacked_limb_0_col322, unpacked_limb_1_col323, ((((((poseidon_full_round_chain_output_limb_20_col274) - (unpacked_limb_0_col322))) - (((unpacked_limb_1_col323) * (M31_512))))) * (M31_8192)), unpacked_limb_3_col324, unpacked_limb_4_col325, ((((((poseidon_full_round_chain_output_limb_21_col275) - (unpacked_limb_3_col324))) - (((unpacked_limb_4_col325) * (M31_512))))) * (M31_8192)), unpacked_limb_6_col326, unpacked_limb_7_col327, ((((((poseidon_full_round_chain_output_limb_22_col276) - (unpacked_limb_6_col326))) - (((unpacked_limb_7_col327) * (M31_512))))) * (M31_8192)), unpacked_limb_9_col328, unpacked_limb_10_col329, ((((((poseidon_full_round_chain_output_limb_23_col277) - (unpacked_limb_9_col328))) - (((unpacked_limb_10_col329) * (M31_512))))) * (M31_8192)), unpacked_limb_12_col330, unpacked_limb_13_col331, ((((((poseidon_full_round_chain_output_limb_24_col278) - (unpacked_limb_12_col330))) - (((unpacked_limb_13_col331) * (M31_512))))) * (M31_8192)), unpacked_limb_15_col332, unpacked_limb_16_col333, ((((((poseidon_full_round_chain_output_limb_25_col279) - (unpacked_limb_15_col332))) - (((unpacked_limb_16_col333) * (M31_512))))) * (M31_8192)), unpacked_limb_18_col334, unpacked_limb_19_col335, ((((((poseidon_full_round_chain_output_limb_26_col280) - (unpacked_limb_18_col334))) - (((unpacked_limb_19_col335) * (M31_512))))) * (M31_8192)), unpacked_limb_21_col336, unpacked_limb_22_col337, ((((((poseidon_full_round_chain_output_limb_27_col281) - (unpacked_limb_21_col336))) - (((unpacked_limb_22_col337) * (M31_512))))) * (M31_8192)), unpacked_limb_24_col338, unpacked_limb_25_col339, ((((((poseidon_full_round_chain_output_limb_28_col282) - (unpacked_limb_24_col338))) - (((unpacked_limb_25_col339) * (M31_512))))) * (M31_8192)), poseidon_full_round_chain_output_limb_29_col283]);

            // Mem Verify.

            let memory_address_to_id_value_tmp_51986_172 = memory_address_to_id_state.deduce_output(((((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start))) + (((seq) * (M31_6))))) + (M31_5)));let output_state_2_id_col340 = memory_address_to_id_value_tmp_51986_172;
            *row[340] = output_state_2_id_col340;*sub_component_inputs.memory_address_to_id[5] =
                ((((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start))) + (((seq) * (M31_6))))) + (M31_5));
            *lookup_data.memory_address_to_id_5 = [((((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start))) + (((seq) * (M31_6))))) + (M31_5)), output_state_2_id_col340];*sub_component_inputs.memory_id_to_big[5] =
                output_state_2_id_col340;
            *lookup_data.memory_id_to_big_5 = [output_state_2_id_col340, unpacked_limb_0_col322, unpacked_limb_1_col323, felt_252_unpack_from_27_output_tmp_51986_171.get_m31(2), unpacked_limb_3_col324, unpacked_limb_4_col325, felt_252_unpack_from_27_output_tmp_51986_171.get_m31(5), unpacked_limb_6_col326, unpacked_limb_7_col327, felt_252_unpack_from_27_output_tmp_51986_171.get_m31(8), unpacked_limb_9_col328, unpacked_limb_10_col329, felt_252_unpack_from_27_output_tmp_51986_171.get_m31(11), unpacked_limb_12_col330, unpacked_limb_13_col331, felt_252_unpack_from_27_output_tmp_51986_171.get_m31(14), unpacked_limb_15_col332, unpacked_limb_16_col333, felt_252_unpack_from_27_output_tmp_51986_171.get_m31(17), unpacked_limb_18_col334, unpacked_limb_19_col335, felt_252_unpack_from_27_output_tmp_51986_171.get_m31(20), unpacked_limb_21_col336, unpacked_limb_22_col337, felt_252_unpack_from_27_output_tmp_51986_171.get_m31(23), unpacked_limb_24_col338, unpacked_limb_25_col339, felt_252_unpack_from_27_output_tmp_51986_171.get_m31(26), poseidon_full_round_chain_output_limb_29_col283];
        });

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    cube_252_0: Vec<[PackedM31; 20]>,
    cube_252_1: Vec<[PackedM31; 20]>,
    memory_address_to_id_0: Vec<[PackedM31; 2]>,
    memory_address_to_id_1: Vec<[PackedM31; 2]>,
    memory_address_to_id_2: Vec<[PackedM31; 2]>,
    memory_address_to_id_3: Vec<[PackedM31; 2]>,
    memory_address_to_id_4: Vec<[PackedM31; 2]>,
    memory_address_to_id_5: Vec<[PackedM31; 2]>,
    memory_id_to_big_0: Vec<[PackedM31; 29]>,
    memory_id_to_big_1: Vec<[PackedM31; 29]>,
    memory_id_to_big_2: Vec<[PackedM31; 29]>,
    memory_id_to_big_3: Vec<[PackedM31; 29]>,
    memory_id_to_big_4: Vec<[PackedM31; 29]>,
    memory_id_to_big_5: Vec<[PackedM31; 29]>,
    poseidon_3_partial_rounds_chain_0: Vec<[PackedM31; 42]>,
    poseidon_3_partial_rounds_chain_1: Vec<[PackedM31; 42]>,
    poseidon_full_round_chain_0: Vec<[PackedM31; 32]>,
    poseidon_full_round_chain_1: Vec<[PackedM31; 32]>,
    poseidon_full_round_chain_2: Vec<[PackedM31; 32]>,
    poseidon_full_round_chain_3: Vec<[PackedM31; 32]>,
    range_check_felt_252_width_27_0: Vec<[PackedM31; 10]>,
    range_check_felt_252_width_27_1: Vec<[PackedM31; 10]>,
    range_check_3_3_3_3_3_0: Vec<[PackedM31; 5]>,
    range_check_3_3_3_3_3_1: Vec<[PackedM31; 5]>,
    range_check_4_4_0: Vec<[PackedM31; 2]>,
    range_check_4_4_1: Vec<[PackedM31; 2]>,
    range_check_4_4_2: Vec<[PackedM31; 2]>,
    range_check_4_4_4_4_0: Vec<[PackedM31; 4]>,
    range_check_4_4_4_4_1: Vec<[PackedM31; 4]>,
    range_check_4_4_4_4_2: Vec<[PackedM31; 4]>,
    range_check_4_4_4_4_3: Vec<[PackedM31; 4]>,
    range_check_4_4_4_4_4: Vec<[PackedM31; 4]>,
    range_check_4_4_4_4_5: Vec<[PackedM31; 4]>,
}

pub struct InteractionClaimGenerator {
    log_size: u32,
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        cube_252: &relations::Cube252,
        memory_address_to_id: &relations::MemoryAddressToId,
        memory_id_to_big: &relations::MemoryIdToBig,
        poseidon_3_partial_rounds_chain: &relations::Poseidon3PartialRoundsChain,
        poseidon_full_round_chain: &relations::PoseidonFullRoundChain,
        range_check_felt_252_width_27: &relations::RangeCheckFelt252Width27,
        range_check_3_3_3_3_3: &relations::RangeCheck_3_3_3_3_3,
        range_check_4_4: &relations::RangeCheck_4_4,
        range_check_4_4_4_4: &relations::RangeCheck_4_4_4_4,
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
            &self.lookup_data.poseidon_full_round_chain_0,
            &self.lookup_data.poseidon_full_round_chain_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = poseidon_full_round_chain.combine(values0);
                let denom1: PackedQM31 = poseidon_full_round_chain.combine(values1);
                writer.write_frac(denom0 - denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_felt_252_width_27_0,
            &self.lookup_data.range_check_felt_252_width_27_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_felt_252_width_27.combine(values0);
                let denom1: PackedQM31 = range_check_felt_252_width_27.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.cube_252_0,
            &self.lookup_data.range_check_3_3_3_3_3_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = cube_252.combine(values0);
                let denom1: PackedQM31 = range_check_3_3_3_3_3.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_3_3_3_3_3_1,
            &self.lookup_data.cube_252_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_3_3_3_3_3.combine(values0);
                let denom1: PackedQM31 = cube_252.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_4_4_4_4_0,
            &self.lookup_data.range_check_4_4_4_4_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_4_4_4_4.combine(values0);
                let denom1: PackedQM31 = range_check_4_4_4_4.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_4_4_0,
            &self.lookup_data.poseidon_3_partial_rounds_chain_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_4_4.combine(values0);
                let denom1: PackedQM31 = poseidon_3_partial_rounds_chain.combine(values1);
                writer.write_frac(denom1 - denom0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.poseidon_3_partial_rounds_chain_1,
            &self.lookup_data.range_check_4_4_4_4_2,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = poseidon_3_partial_rounds_chain.combine(values0);
                let denom1: PackedQM31 = range_check_4_4_4_4.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_4_4_4_4_3,
            &self.lookup_data.range_check_4_4_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_4_4_4_4.combine(values0);
                let denom1: PackedQM31 = range_check_4_4.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_4_4_4_4_4,
            &self.lookup_data.range_check_4_4_4_4_5,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_4_4_4_4.combine(values0);
                let denom1: PackedQM31 = range_check_4_4_4_4.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_4_4_2,
            &self.lookup_data.poseidon_full_round_chain_2,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_4_4.combine(values0);
                let denom1: PackedQM31 = poseidon_full_round_chain.combine(values1);
                writer.write_frac(denom1 - denom0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.poseidon_full_round_chain_3,
            &self.lookup_data.memory_address_to_id_3,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = poseidon_full_round_chain.combine(values0);
                let denom1: PackedQM31 = memory_address_to_id.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_id_to_big_3,
            &self.lookup_data.memory_address_to_id_4,
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
            &self.lookup_data.memory_id_to_big_4,
            &self.lookup_data.memory_address_to_id_5,
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
        (col_gen.par_iter_mut(), &self.lookup_data.memory_id_to_big_5)
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
