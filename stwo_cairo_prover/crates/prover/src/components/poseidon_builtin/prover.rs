#![allow(unused_parens)]
use itertools::Itertools;

use super::component::{Claim, InteractionClaim};
use crate::cairo_air::preprocessed::Seq;
use crate::components::prelude::proving::*;
use crate::components::range_check_vector::{
    range_check_3_3_3_3_3, range_check_4_4, range_check_4_4_4_4,
};
use crate::components::{
    cube_252, memory_address_to_id, memory_id_to_big, poseidon_3_partial_rounds_chain,
    poseidon_full_round_chain, range_check_felt_252_width_27,
};

const N_TRACE_COLUMNS: usize = 347;

#[derive(Default)]
pub struct ClaimGenerator {
    pub log_size: u32,
    pub poseidon_builtin_segment_start: u32,
}
impl ClaimGenerator {
    pub fn new(log_size: u32, poseidon_builtin_segment_start: u32) -> Self {
        Self {
            log_size,
            poseidon_builtin_segment_start,
        }
    }

    pub fn write_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        cube_252_state: &mut cube_252::ClaimGenerator,
        memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
        memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
        poseidon_3_partial_rounds_chain_state: &mut poseidon_3_partial_rounds_chain::ClaimGenerator,
        poseidon_full_round_chain_state: &mut poseidon_full_round_chain::ClaimGenerator,
        range_check_3_3_3_3_3_state: &range_check_3_3_3_3_3::ClaimGenerator,
        range_check_4_4_state: &range_check_4_4::ClaimGenerator,
        range_check_4_4_4_4_state: &range_check_4_4_4_4::ClaimGenerator,
        range_check_felt_252_width_27_state: &mut range_check_felt_252_width_27::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator)
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let log_size = self.log_size;

        let (trace, lookup_data) = write_trace_simd(
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

#[allow(clippy::useless_conversion)]
#[allow(unused_variables)]
#[allow(clippy::double_parens)]
#[allow(non_snake_case)]
fn write_trace_simd(
    log_size: u32,
    poseidon_builtin_segment_start: u32,
    cube_252_state: &mut cube_252::ClaimGenerator,
    memory_address_to_id_state: &memory_address_to_id::ClaimGenerator,
    memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
    poseidon_3_partial_rounds_chain_state: &mut poseidon_3_partial_rounds_chain::ClaimGenerator,
    poseidon_full_round_chain_state: &mut poseidon_full_round_chain::ClaimGenerator,
    range_check_3_3_3_3_3_state: &range_check_3_3_3_3_3::ClaimGenerator,
    range_check_4_4_state: &range_check_4_4::ClaimGenerator,
    range_check_4_4_4_4_state: &range_check_4_4_4_4::ClaimGenerator,
    range_check_felt_252_width_27_state: &mut range_check_felt_252_width_27::ClaimGenerator,
) -> (ComponentTrace<N_TRACE_COLUMNS>, LookupData) {
    let log_n_packed_rows = log_size - LOG_N_LANES;
    let (mut trace, mut lookup_data) = unsafe {
        (
            ComponentTrace::<N_TRACE_COLUMNS>::uninitialized(log_size),
            LookupData::uninitialized(log_n_packed_rows),
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

    let mut cube_252_inputs_vec =
        vec![[[Felt252Width27::default(); 16]; 2]; 1 << log_n_packed_rows];
    let mut range_check_felt_252_width_27_inputs_vec =
        vec![[[Felt252Width27::default(); 16]; 2]; 1 << log_n_packed_rows];
    let mut poseidon_full_round_chain_inputs_vec = vec![
        [[(
            M31::default(),
            M31::default(),
            [Felt252Width27::default(); 3]
        ); N_LANES]; 8];
        1 << log_n_packed_rows
    ];
    let mut poseidon_3_partial_rounds_chain_inputs_vec = vec![
        [[(
            M31::default(),
            M31::default(),
            [Felt252Width27::default(); 4]
        ); N_LANES]; 27];
        1 << log_n_packed_rows
    ];

    trace
    .par_iter_mut()
    .enumerate()
    .zip(lookup_data.par_iter_mut())
    .zip(cube_252_inputs_vec.par_iter_mut())
    .zip(range_check_felt_252_width_27_inputs_vec.par_iter_mut())
    .zip(poseidon_full_round_chain_inputs_vec.par_iter_mut())
    .zip(poseidon_3_partial_rounds_chain_inputs_vec.par_iter_mut())
    .for_each(
        |((((((row_index, mut row), lookup_data),cube_252_input),range_check_felt_252_width_27_input), poseidon_full_round_chain_input), poseidon_3_partial_rounds_chain_input)| {
            let seq = seq.packed_at(row_index);

            //Read Positive Num Bits 252.

            let memory_address_to_id_value_tmp_51986_0 = memory_address_to_id_state.deduce_output(((((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start))) + (((seq) * (M31_6))))) + (M31_0)));let memory_id_to_big_value_tmp_51986_1 = memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_51986_0);let input_state_0_id_col0 = memory_address_to_id_value_tmp_51986_0;
            *row[0] = input_state_0_id_col0;let memory_address_to_id_inputs_0 =
                ((((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start))) + (((seq) * (M31_6))))) + (M31_0)).unpack();
            *lookup_data.memory_address_to_id_0 = [((((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start))) + (((seq) * (M31_6))))) + (M31_0)), input_state_0_id_col0];let input_state_0_limb_0_col1 = memory_id_to_big_value_tmp_51986_1.get_m31(0);
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
            *row[28] = input_state_0_limb_27_col28;let memory_id_to_big_inputs_0 =
                input_state_0_id_col0.unpack();
            *lookup_data.memory_id_to_big_0 = [input_state_0_id_col0, input_state_0_limb_0_col1, input_state_0_limb_1_col2, input_state_0_limb_2_col3, input_state_0_limb_3_col4, input_state_0_limb_4_col5, input_state_0_limb_5_col6, input_state_0_limb_6_col7, input_state_0_limb_7_col8, input_state_0_limb_8_col9, input_state_0_limb_9_col10, input_state_0_limb_10_col11, input_state_0_limb_11_col12, input_state_0_limb_12_col13, input_state_0_limb_13_col14, input_state_0_limb_14_col15, input_state_0_limb_15_col16, input_state_0_limb_16_col17, input_state_0_limb_17_col18, input_state_0_limb_18_col19, input_state_0_limb_19_col20, input_state_0_limb_20_col21, input_state_0_limb_21_col22, input_state_0_limb_22_col23, input_state_0_limb_23_col24, input_state_0_limb_24_col25, input_state_0_limb_25_col26, input_state_0_limb_26_col27, input_state_0_limb_27_col28];

            //Read Positive Num Bits 252.

            let memory_address_to_id_value_tmp_51986_2 = memory_address_to_id_state.deduce_output(((((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start))) + (((seq) * (M31_6))))) + (M31_1)));let memory_id_to_big_value_tmp_51986_3 = memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_51986_2);let input_state_1_id_col29 = memory_address_to_id_value_tmp_51986_2;
            *row[29] = input_state_1_id_col29;let memory_address_to_id_inputs_1 =
                ((((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start))) + (((seq) * (M31_6))))) + (M31_1)).unpack();
            *lookup_data.memory_address_to_id_1 = [((((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start))) + (((seq) * (M31_6))))) + (M31_1)), input_state_1_id_col29];let input_state_1_limb_0_col30 = memory_id_to_big_value_tmp_51986_3.get_m31(0);
            *row[30] = input_state_1_limb_0_col30;let input_state_1_limb_1_col31 = memory_id_to_big_value_tmp_51986_3.get_m31(1);
            *row[31] = input_state_1_limb_1_col31;let input_state_1_limb_2_col32 = memory_id_to_big_value_tmp_51986_3.get_m31(2);
            *row[32] = input_state_1_limb_2_col32;let input_state_1_limb_3_col33 = memory_id_to_big_value_tmp_51986_3.get_m31(3);
            *row[33] = input_state_1_limb_3_col33;let input_state_1_limb_4_col34 = memory_id_to_big_value_tmp_51986_3.get_m31(4);
            *row[34] = input_state_1_limb_4_col34;let input_state_1_limb_5_col35 = memory_id_to_big_value_tmp_51986_3.get_m31(5);
            *row[35] = input_state_1_limb_5_col35;let input_state_1_limb_6_col36 = memory_id_to_big_value_tmp_51986_3.get_m31(6);
            *row[36] = input_state_1_limb_6_col36;let input_state_1_limb_7_col37 = memory_id_to_big_value_tmp_51986_3.get_m31(7);
            *row[37] = input_state_1_limb_7_col37;let input_state_1_limb_8_col38 = memory_id_to_big_value_tmp_51986_3.get_m31(8);
            *row[38] = input_state_1_limb_8_col38;let input_state_1_limb_9_col39 = memory_id_to_big_value_tmp_51986_3.get_m31(9);
            *row[39] = input_state_1_limb_9_col39;let input_state_1_limb_10_col40 = memory_id_to_big_value_tmp_51986_3.get_m31(10);
            *row[40] = input_state_1_limb_10_col40;let input_state_1_limb_11_col41 = memory_id_to_big_value_tmp_51986_3.get_m31(11);
            *row[41] = input_state_1_limb_11_col41;let input_state_1_limb_12_col42 = memory_id_to_big_value_tmp_51986_3.get_m31(12);
            *row[42] = input_state_1_limb_12_col42;let input_state_1_limb_13_col43 = memory_id_to_big_value_tmp_51986_3.get_m31(13);
            *row[43] = input_state_1_limb_13_col43;let input_state_1_limb_14_col44 = memory_id_to_big_value_tmp_51986_3.get_m31(14);
            *row[44] = input_state_1_limb_14_col44;let input_state_1_limb_15_col45 = memory_id_to_big_value_tmp_51986_3.get_m31(15);
            *row[45] = input_state_1_limb_15_col45;let input_state_1_limb_16_col46 = memory_id_to_big_value_tmp_51986_3.get_m31(16);
            *row[46] = input_state_1_limb_16_col46;let input_state_1_limb_17_col47 = memory_id_to_big_value_tmp_51986_3.get_m31(17);
            *row[47] = input_state_1_limb_17_col47;let input_state_1_limb_18_col48 = memory_id_to_big_value_tmp_51986_3.get_m31(18);
            *row[48] = input_state_1_limb_18_col48;let input_state_1_limb_19_col49 = memory_id_to_big_value_tmp_51986_3.get_m31(19);
            *row[49] = input_state_1_limb_19_col49;let input_state_1_limb_20_col50 = memory_id_to_big_value_tmp_51986_3.get_m31(20);
            *row[50] = input_state_1_limb_20_col50;let input_state_1_limb_21_col51 = memory_id_to_big_value_tmp_51986_3.get_m31(21);
            *row[51] = input_state_1_limb_21_col51;let input_state_1_limb_22_col52 = memory_id_to_big_value_tmp_51986_3.get_m31(22);
            *row[52] = input_state_1_limb_22_col52;let input_state_1_limb_23_col53 = memory_id_to_big_value_tmp_51986_3.get_m31(23);
            *row[53] = input_state_1_limb_23_col53;let input_state_1_limb_24_col54 = memory_id_to_big_value_tmp_51986_3.get_m31(24);
            *row[54] = input_state_1_limb_24_col54;let input_state_1_limb_25_col55 = memory_id_to_big_value_tmp_51986_3.get_m31(25);
            *row[55] = input_state_1_limb_25_col55;let input_state_1_limb_26_col56 = memory_id_to_big_value_tmp_51986_3.get_m31(26);
            *row[56] = input_state_1_limb_26_col56;let input_state_1_limb_27_col57 = memory_id_to_big_value_tmp_51986_3.get_m31(27);
            *row[57] = input_state_1_limb_27_col57;let memory_id_to_big_inputs_1 =
                input_state_1_id_col29.unpack();
            *lookup_data.memory_id_to_big_1 = [input_state_1_id_col29, input_state_1_limb_0_col30, input_state_1_limb_1_col31, input_state_1_limb_2_col32, input_state_1_limb_3_col33, input_state_1_limb_4_col34, input_state_1_limb_5_col35, input_state_1_limb_6_col36, input_state_1_limb_7_col37, input_state_1_limb_8_col38, input_state_1_limb_9_col39, input_state_1_limb_10_col40, input_state_1_limb_11_col41, input_state_1_limb_12_col42, input_state_1_limb_13_col43, input_state_1_limb_14_col44, input_state_1_limb_15_col45, input_state_1_limb_16_col46, input_state_1_limb_17_col47, input_state_1_limb_18_col48, input_state_1_limb_19_col49, input_state_1_limb_20_col50, input_state_1_limb_21_col51, input_state_1_limb_22_col52, input_state_1_limb_23_col53, input_state_1_limb_24_col54, input_state_1_limb_25_col55, input_state_1_limb_26_col56, input_state_1_limb_27_col57];

            //Read Positive Num Bits 252.

            let memory_address_to_id_value_tmp_51986_4 = memory_address_to_id_state.deduce_output(((((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start))) + (((seq) * (M31_6))))) + (M31_2)));let memory_id_to_big_value_tmp_51986_5 = memory_id_to_big_state.deduce_output(memory_address_to_id_value_tmp_51986_4);let input_state_2_id_col58 = memory_address_to_id_value_tmp_51986_4;
            *row[58] = input_state_2_id_col58;let memory_address_to_id_inputs_2 =
                ((((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start))) + (((seq) * (M31_6))))) + (M31_2)).unpack();
            *lookup_data.memory_address_to_id_2 = [((((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start))) + (((seq) * (M31_6))))) + (M31_2)), input_state_2_id_col58];let input_state_2_limb_0_col59 = memory_id_to_big_value_tmp_51986_5.get_m31(0);
            *row[59] = input_state_2_limb_0_col59;let input_state_2_limb_1_col60 = memory_id_to_big_value_tmp_51986_5.get_m31(1);
            *row[60] = input_state_2_limb_1_col60;let input_state_2_limb_2_col61 = memory_id_to_big_value_tmp_51986_5.get_m31(2);
            *row[61] = input_state_2_limb_2_col61;let input_state_2_limb_3_col62 = memory_id_to_big_value_tmp_51986_5.get_m31(3);
            *row[62] = input_state_2_limb_3_col62;let input_state_2_limb_4_col63 = memory_id_to_big_value_tmp_51986_5.get_m31(4);
            *row[63] = input_state_2_limb_4_col63;let input_state_2_limb_5_col64 = memory_id_to_big_value_tmp_51986_5.get_m31(5);
            *row[64] = input_state_2_limb_5_col64;let input_state_2_limb_6_col65 = memory_id_to_big_value_tmp_51986_5.get_m31(6);
            *row[65] = input_state_2_limb_6_col65;let input_state_2_limb_7_col66 = memory_id_to_big_value_tmp_51986_5.get_m31(7);
            *row[66] = input_state_2_limb_7_col66;let input_state_2_limb_8_col67 = memory_id_to_big_value_tmp_51986_5.get_m31(8);
            *row[67] = input_state_2_limb_8_col67;let input_state_2_limb_9_col68 = memory_id_to_big_value_tmp_51986_5.get_m31(9);
            *row[68] = input_state_2_limb_9_col68;let input_state_2_limb_10_col69 = memory_id_to_big_value_tmp_51986_5.get_m31(10);
            *row[69] = input_state_2_limb_10_col69;let input_state_2_limb_11_col70 = memory_id_to_big_value_tmp_51986_5.get_m31(11);
            *row[70] = input_state_2_limb_11_col70;let input_state_2_limb_12_col71 = memory_id_to_big_value_tmp_51986_5.get_m31(12);
            *row[71] = input_state_2_limb_12_col71;let input_state_2_limb_13_col72 = memory_id_to_big_value_tmp_51986_5.get_m31(13);
            *row[72] = input_state_2_limb_13_col72;let input_state_2_limb_14_col73 = memory_id_to_big_value_tmp_51986_5.get_m31(14);
            *row[73] = input_state_2_limb_14_col73;let input_state_2_limb_15_col74 = memory_id_to_big_value_tmp_51986_5.get_m31(15);
            *row[74] = input_state_2_limb_15_col74;let input_state_2_limb_16_col75 = memory_id_to_big_value_tmp_51986_5.get_m31(16);
            *row[75] = input_state_2_limb_16_col75;let input_state_2_limb_17_col76 = memory_id_to_big_value_tmp_51986_5.get_m31(17);
            *row[76] = input_state_2_limb_17_col76;let input_state_2_limb_18_col77 = memory_id_to_big_value_tmp_51986_5.get_m31(18);
            *row[77] = input_state_2_limb_18_col77;let input_state_2_limb_19_col78 = memory_id_to_big_value_tmp_51986_5.get_m31(19);
            *row[78] = input_state_2_limb_19_col78;let input_state_2_limb_20_col79 = memory_id_to_big_value_tmp_51986_5.get_m31(20);
            *row[79] = input_state_2_limb_20_col79;let input_state_2_limb_21_col80 = memory_id_to_big_value_tmp_51986_5.get_m31(21);
            *row[80] = input_state_2_limb_21_col80;let input_state_2_limb_22_col81 = memory_id_to_big_value_tmp_51986_5.get_m31(22);
            *row[81] = input_state_2_limb_22_col81;let input_state_2_limb_23_col82 = memory_id_to_big_value_tmp_51986_5.get_m31(23);
            *row[82] = input_state_2_limb_23_col82;let input_state_2_limb_24_col83 = memory_id_to_big_value_tmp_51986_5.get_m31(24);
            *row[83] = input_state_2_limb_24_col83;let input_state_2_limb_25_col84 = memory_id_to_big_value_tmp_51986_5.get_m31(25);
            *row[84] = input_state_2_limb_25_col84;let input_state_2_limb_26_col85 = memory_id_to_big_value_tmp_51986_5.get_m31(26);
            *row[85] = input_state_2_limb_26_col85;let input_state_2_limb_27_col86 = memory_id_to_big_value_tmp_51986_5.get_m31(27);
            *row[86] = input_state_2_limb_27_col86;let memory_id_to_big_inputs_2 =
                input_state_2_id_col58.unpack();
            *lookup_data.memory_id_to_big_2 = [input_state_2_id_col58, input_state_2_limb_0_col59, input_state_2_limb_1_col60, input_state_2_limb_2_col61, input_state_2_limb_3_col62, input_state_2_limb_4_col63, input_state_2_limb_5_col64, input_state_2_limb_6_col65, input_state_2_limb_7_col66, input_state_2_limb_8_col67, input_state_2_limb_9_col68, input_state_2_limb_10_col69, input_state_2_limb_11_col70, input_state_2_limb_12_col71, input_state_2_limb_13_col72, input_state_2_limb_14_col73, input_state_2_limb_15_col74, input_state_2_limb_16_col75, input_state_2_limb_17_col76, input_state_2_limb_18_col77, input_state_2_limb_19_col78, input_state_2_limb_20_col79, input_state_2_limb_21_col80, input_state_2_limb_22_col81, input_state_2_limb_23_col82, input_state_2_limb_24_col83, input_state_2_limb_25_col84, input_state_2_limb_26_col85, input_state_2_limb_27_col86];

            //Poseidon Hades Permutation.

            //Linear Combination N 2 Coefs 1 1.

            let combination_result_tmp_51986_6 = PackedFelt252Width27::from_packed_felt252(((((Felt252_0_0_0_0) + (((Felt252_1_0_0_0) * (PackedFelt252::from_packed_felt252width27(PackedFelt252Width27::from_limbs([((((input_state_0_limb_0_col1) + (((input_state_0_limb_1_col2) * (M31_512))))) + (((input_state_0_limb_2_col3) * (M31_262144)))), ((((input_state_0_limb_3_col4) + (((input_state_0_limb_4_col5) * (M31_512))))) + (((input_state_0_limb_5_col6) * (M31_262144)))), ((((input_state_0_limb_6_col7) + (((input_state_0_limb_7_col8) * (M31_512))))) + (((input_state_0_limb_8_col9) * (M31_262144)))), ((((input_state_0_limb_9_col10) + (((input_state_0_limb_10_col11) * (M31_512))))) + (((input_state_0_limb_11_col12) * (M31_262144)))), ((((input_state_0_limb_12_col13) + (((input_state_0_limb_13_col14) * (M31_512))))) + (((input_state_0_limb_14_col15) * (M31_262144)))), ((((input_state_0_limb_15_col16) + (((input_state_0_limb_16_col17) * (M31_512))))) + (((input_state_0_limb_17_col18) * (M31_262144)))), ((((input_state_0_limb_18_col19) + (((input_state_0_limb_19_col20) * (M31_512))))) + (((input_state_0_limb_20_col21) * (M31_262144)))), ((((input_state_0_limb_21_col22) + (((input_state_0_limb_22_col23) * (M31_512))))) + (((input_state_0_limb_23_col24) * (M31_262144)))), ((((input_state_0_limb_24_col25) + (((input_state_0_limb_25_col26) * (M31_512))))) + (((input_state_0_limb_26_col27) * (M31_262144)))), input_state_0_limb_27_col28]))))))) + (Felt252_9275160746813554287_16541205595039575623_4169650429605064889_470088886057789987)));let combination_limb_0_col87 = combination_result_tmp_51986_6.get_m31(0);
            *row[87] = combination_limb_0_col87;let combination_limb_1_col88 = combination_result_tmp_51986_6.get_m31(1);
            *row[88] = combination_limb_1_col88;let combination_limb_2_col89 = combination_result_tmp_51986_6.get_m31(2);
            *row[89] = combination_limb_2_col89;let combination_limb_3_col90 = combination_result_tmp_51986_6.get_m31(3);
            *row[90] = combination_limb_3_col90;let combination_limb_4_col91 = combination_result_tmp_51986_6.get_m31(4);
            *row[91] = combination_limb_4_col91;let combination_limb_5_col92 = combination_result_tmp_51986_6.get_m31(5);
            *row[92] = combination_limb_5_col92;let combination_limb_6_col93 = combination_result_tmp_51986_6.get_m31(6);
            *row[93] = combination_limb_6_col93;let combination_limb_7_col94 = combination_result_tmp_51986_6.get_m31(7);
            *row[94] = combination_limb_7_col94;let combination_limb_8_col95 = combination_result_tmp_51986_6.get_m31(8);
            *row[95] = combination_limb_8_col95;let combination_limb_9_col96 = combination_result_tmp_51986_6.get_m31(9);
            *row[96] = combination_limb_9_col96;let biased_limb_accumulator_u32_tmp_51986_7 = PackedUInt32::from_m31(((((((((M31_0) - (combination_limb_0_col87))) + (((M31_1) * (((((input_state_0_limb_0_col1) + (((input_state_0_limb_1_col2) * (M31_512))))) + (((input_state_0_limb_2_col3) * (M31_262144))))))))) + (M31_74972783))) + (M31_134217729)));let p_coef_col97 = ((biased_limb_accumulator_u32_tmp_51986_7.low().as_m31()) - (M31_1));
            *row[97] = p_coef_col97;let carry_0_tmp_51986_8 = ((((((((((M31_0) - (combination_limb_0_col87))) + (((M31_1) * (((((input_state_0_limb_0_col1) + (((input_state_0_limb_1_col2) * (M31_512))))) + (((input_state_0_limb_2_col3) * (M31_262144))))))))) + (M31_74972783))) - (((p_coef_col97) * (M31_1))))) * (M31_16));let carry_1_tmp_51986_9 = ((((((((((carry_0_tmp_51986_8) - (combination_limb_1_col88))) + (((M31_1) * (((((input_state_0_limb_3_col4) + (((input_state_0_limb_4_col5) * (M31_512))))) + (((input_state_0_limb_5_col6) * (M31_262144))))))))) + (M31_117420501))) - (((p_coef_col97) * (M31_0))))) * (M31_16));let carry_2_tmp_51986_10 = ((((((((((carry_1_tmp_51986_9) - (combination_limb_2_col89))) + (((M31_1) * (((((input_state_0_limb_6_col7) + (((input_state_0_limb_7_col8) * (M31_512))))) + (((input_state_0_limb_8_col9) * (M31_262144))))))))) + (M31_112795138))) - (((p_coef_col97) * (M31_0))))) * (M31_16));let carry_3_tmp_51986_11 = ((((((((((carry_2_tmp_51986_10) - (combination_limb_3_col90))) + (((M31_1) * (((((input_state_0_limb_9_col10) + (((input_state_0_limb_10_col11) * (M31_512))))) + (((input_state_0_limb_11_col12) * (M31_262144))))))))) + (M31_91013252))) - (((p_coef_col97) * (M31_0))))) * (M31_16));let carry_4_tmp_51986_12 = ((((((((((carry_3_tmp_51986_11) - (combination_limb_4_col91))) + (((M31_1) * (((((input_state_0_limb_12_col13) + (((input_state_0_limb_13_col14) * (M31_512))))) + (((input_state_0_limb_14_col15) * (M31_262144))))))))) + (M31_60709090))) - (((p_coef_col97) * (M31_0))))) * (M31_16));let carry_5_tmp_51986_13 = ((((((((((carry_4_tmp_51986_12) - (combination_limb_5_col92))) + (((M31_1) * (((((input_state_0_limb_15_col16) + (((input_state_0_limb_16_col17) * (M31_512))))) + (((input_state_0_limb_17_col18) * (M31_262144))))))))) + (M31_44848225))) - (((p_coef_col97) * (M31_0))))) * (M31_16));let carry_6_tmp_51986_14 = ((((((((((carry_5_tmp_51986_13) - (combination_limb_6_col93))) + (((M31_1) * (((((input_state_0_limb_18_col19) + (((input_state_0_limb_19_col20) * (M31_512))))) + (((input_state_0_limb_20_col21) * (M31_262144))))))))) + (M31_108487870))) - (((p_coef_col97) * (M31_0))))) * (M31_16));let carry_7_tmp_51986_15 = ((((((((((carry_6_tmp_51986_14) - (combination_limb_7_col94))) + (((M31_1) * (((((input_state_0_limb_21_col22) + (((input_state_0_limb_22_col23) * (M31_512))))) + (((input_state_0_limb_23_col24) * (M31_262144))))))))) + (M31_44781849))) - (((p_coef_col97) * (M31_136))))) * (M31_16));let carry_8_tmp_51986_16 = ((((((((((carry_7_tmp_51986_15) - (combination_limb_8_col95))) + (((M31_1) * (((((input_state_0_limb_24_col25) + (((input_state_0_limb_25_col26) * (M31_512))))) + (((input_state_0_limb_26_col27) * (M31_262144))))))))) + (M31_102193642))) - (((p_coef_col97) * (M31_0))))) * (M31_16));

            //Linear Combination N 2 Coefs 1 1.

            let combination_result_tmp_51986_27 = PackedFelt252Width27::from_packed_felt252(((((Felt252_0_0_0_0) + (((Felt252_1_0_0_0) * (PackedFelt252::from_packed_felt252width27(PackedFelt252Width27::from_limbs([((((input_state_1_limb_0_col30) + (((input_state_1_limb_1_col31) * (M31_512))))) + (((input_state_1_limb_2_col32) * (M31_262144)))), ((((input_state_1_limb_3_col33) + (((input_state_1_limb_4_col34) * (M31_512))))) + (((input_state_1_limb_5_col35) * (M31_262144)))), ((((input_state_1_limb_6_col36) + (((input_state_1_limb_7_col37) * (M31_512))))) + (((input_state_1_limb_8_col38) * (M31_262144)))), ((((input_state_1_limb_9_col39) + (((input_state_1_limb_10_col40) * (M31_512))))) + (((input_state_1_limb_11_col41) * (M31_262144)))), ((((input_state_1_limb_12_col42) + (((input_state_1_limb_13_col43) * (M31_512))))) + (((input_state_1_limb_14_col44) * (M31_262144)))), ((((input_state_1_limb_15_col45) + (((input_state_1_limb_16_col46) * (M31_512))))) + (((input_state_1_limb_17_col47) * (M31_262144)))), ((((input_state_1_limb_18_col48) + (((input_state_1_limb_19_col49) * (M31_512))))) + (((input_state_1_limb_20_col50) * (M31_262144)))), ((((input_state_1_limb_21_col51) + (((input_state_1_limb_22_col52) * (M31_512))))) + (((input_state_1_limb_23_col53) * (M31_262144)))), ((((input_state_1_limb_24_col54) + (((input_state_1_limb_25_col55) * (M31_512))))) + (((input_state_1_limb_26_col56) * (M31_262144)))), input_state_1_limb_27_col57]))))))) + (Felt252_16477292399064058052_4441744911417641572_18431044672185975386_252894828082060025)));let combination_limb_0_col98 = combination_result_tmp_51986_27.get_m31(0);
            *row[98] = combination_limb_0_col98;let combination_limb_1_col99 = combination_result_tmp_51986_27.get_m31(1);
            *row[99] = combination_limb_1_col99;let combination_limb_2_col100 = combination_result_tmp_51986_27.get_m31(2);
            *row[100] = combination_limb_2_col100;let combination_limb_3_col101 = combination_result_tmp_51986_27.get_m31(3);
            *row[101] = combination_limb_3_col101;let combination_limb_4_col102 = combination_result_tmp_51986_27.get_m31(4);
            *row[102] = combination_limb_4_col102;let combination_limb_5_col103 = combination_result_tmp_51986_27.get_m31(5);
            *row[103] = combination_limb_5_col103;let combination_limb_6_col104 = combination_result_tmp_51986_27.get_m31(6);
            *row[104] = combination_limb_6_col104;let combination_limb_7_col105 = combination_result_tmp_51986_27.get_m31(7);
            *row[105] = combination_limb_7_col105;let combination_limb_8_col106 = combination_result_tmp_51986_27.get_m31(8);
            *row[106] = combination_limb_8_col106;let combination_limb_9_col107 = combination_result_tmp_51986_27.get_m31(9);
            *row[107] = combination_limb_9_col107;let biased_limb_accumulator_u32_tmp_51986_28 = PackedUInt32::from_m31(((((((((M31_0) - (combination_limb_0_col98))) + (((M31_1) * (((((input_state_1_limb_0_col30) + (((input_state_1_limb_1_col31) * (M31_512))))) + (((input_state_1_limb_2_col32) * (M31_262144))))))))) + (M31_41224388))) + (M31_134217729)));let p_coef_col108 = ((biased_limb_accumulator_u32_tmp_51986_28.low().as_m31()) - (M31_1));
            *row[108] = p_coef_col108;let carry_0_tmp_51986_29 = ((((((((((M31_0) - (combination_limb_0_col98))) + (((M31_1) * (((((input_state_1_limb_0_col30) + (((input_state_1_limb_1_col31) * (M31_512))))) + (((input_state_1_limb_2_col32) * (M31_262144))))))))) + (M31_41224388))) - (((p_coef_col108) * (M31_1))))) * (M31_16));let carry_1_tmp_51986_30 = ((((((((((carry_0_tmp_51986_29) - (combination_limb_1_col99))) + (((M31_1) * (((((input_state_1_limb_3_col33) + (((input_state_1_limb_4_col34) * (M31_512))))) + (((input_state_1_limb_5_col35) * (M31_262144))))))))) + (M31_90391646))) - (((p_coef_col108) * (M31_0))))) * (M31_16));let carry_2_tmp_51986_31 = ((((((((((carry_1_tmp_51986_30) - (combination_limb_2_col100))) + (((M31_1) * (((((input_state_1_limb_6_col36) + (((input_state_1_limb_7_col37) * (M31_512))))) + (((input_state_1_limb_8_col38) * (M31_262144))))))))) + (M31_36279186))) - (((p_coef_col108) * (M31_0))))) * (M31_16));let carry_3_tmp_51986_32 = ((((((((((carry_2_tmp_51986_31) - (combination_limb_3_col101))) + (((M31_1) * (((((input_state_1_limb_9_col39) + (((input_state_1_limb_10_col40) * (M31_512))))) + (((input_state_1_limb_11_col41) * (M31_262144))))))))) + (M31_129717753))) - (((p_coef_col108) * (M31_0))))) * (M31_16));let carry_4_tmp_51986_33 = ((((((((((carry_3_tmp_51986_32) - (combination_limb_4_col102))) + (((M31_1) * (((((input_state_1_limb_12_col42) + (((input_state_1_limb_13_col43) * (M31_512))))) + (((input_state_1_limb_14_col44) * (M31_262144))))))))) + (M31_94624323))) - (((p_coef_col108) * (M31_0))))) * (M31_16));let carry_5_tmp_51986_34 = ((((((((((carry_4_tmp_51986_33) - (combination_limb_5_col103))) + (((M31_1) * (((((input_state_1_limb_15_col45) + (((input_state_1_limb_16_col46) * (M31_512))))) + (((input_state_1_limb_17_col47) * (M31_262144))))))))) + (M31_75104388))) - (((p_coef_col108) * (M31_0))))) * (M31_16));let carry_6_tmp_51986_35 = ((((((((((carry_5_tmp_51986_34) - (combination_limb_6_col104))) + (((M31_1) * (((((input_state_1_limb_18_col48) + (((input_state_1_limb_19_col49) * (M31_512))))) + (((input_state_1_limb_20_col50) * (M31_262144))))))))) + (M31_133303902))) - (((p_coef_col108) * (M31_0))))) * (M31_16));let carry_7_tmp_51986_36 = ((((((((((carry_6_tmp_51986_35) - (combination_limb_7_col105))) + (((M31_1) * (((((input_state_1_limb_21_col51) + (((input_state_1_limb_22_col52) * (M31_512))))) + (((input_state_1_limb_23_col53) * (M31_262144))))))))) + (M31_48945103))) - (((p_coef_col108) * (M31_136))))) * (M31_16));let carry_8_tmp_51986_37 = ((((((((((carry_7_tmp_51986_36) - (combination_limb_8_col106))) + (((M31_1) * (((((input_state_1_limb_24_col54) + (((input_state_1_limb_25_col55) * (M31_512))))) + (((input_state_1_limb_26_col56) * (M31_262144))))))))) + (M31_41320857))) - (((p_coef_col108) * (M31_0))))) * (M31_16));

            //Linear Combination N 2 Coefs 1 1.

            let combination_result_tmp_51986_48 = PackedFelt252Width27::from_packed_felt252(((((Felt252_0_0_0_0) + (((Felt252_1_0_0_0) * (PackedFelt252::from_packed_felt252width27(PackedFelt252Width27::from_limbs([((((input_state_2_limb_0_col59) + (((input_state_2_limb_1_col60) * (M31_512))))) + (((input_state_2_limb_2_col61) * (M31_262144)))), ((((input_state_2_limb_3_col62) + (((input_state_2_limb_4_col63) * (M31_512))))) + (((input_state_2_limb_5_col64) * (M31_262144)))), ((((input_state_2_limb_6_col65) + (((input_state_2_limb_7_col66) * (M31_512))))) + (((input_state_2_limb_8_col67) * (M31_262144)))), ((((input_state_2_limb_9_col68) + (((input_state_2_limb_10_col69) * (M31_512))))) + (((input_state_2_limb_11_col70) * (M31_262144)))), ((((input_state_2_limb_12_col71) + (((input_state_2_limb_13_col72) * (M31_512))))) + (((input_state_2_limb_14_col73) * (M31_262144)))), ((((input_state_2_limb_15_col74) + (((input_state_2_limb_16_col75) * (M31_512))))) + (((input_state_2_limb_17_col76) * (M31_262144)))), ((((input_state_2_limb_18_col77) + (((input_state_2_limb_19_col78) * (M31_512))))) + (((input_state_2_limb_20_col79) * (M31_262144)))), ((((input_state_2_limb_21_col80) + (((input_state_2_limb_22_col81) * (M31_512))))) + (((input_state_2_limb_23_col82) * (M31_262144)))), ((((input_state_2_limb_24_col83) + (((input_state_2_limb_25_col84) * (M31_512))))) + (((input_state_2_limb_26_col85) * (M31_262144)))), input_state_2_limb_27_col86]))))))) + (Felt252_8794894655201903369_3219077422080798056_16714934791572408267_262217499501479120)));let combination_limb_0_col109 = combination_result_tmp_51986_48.get_m31(0);
            *row[109] = combination_limb_0_col109;let combination_limb_1_col110 = combination_result_tmp_51986_48.get_m31(1);
            *row[110] = combination_limb_1_col110;let combination_limb_2_col111 = combination_result_tmp_51986_48.get_m31(2);
            *row[111] = combination_limb_2_col111;let combination_limb_3_col112 = combination_result_tmp_51986_48.get_m31(3);
            *row[112] = combination_limb_3_col112;let combination_limb_4_col113 = combination_result_tmp_51986_48.get_m31(4);
            *row[113] = combination_limb_4_col113;let combination_limb_5_col114 = combination_result_tmp_51986_48.get_m31(5);
            *row[114] = combination_limb_5_col114;let combination_limb_6_col115 = combination_result_tmp_51986_48.get_m31(6);
            *row[115] = combination_limb_6_col115;let combination_limb_7_col116 = combination_result_tmp_51986_48.get_m31(7);
            *row[116] = combination_limb_7_col116;let combination_limb_8_col117 = combination_result_tmp_51986_48.get_m31(8);
            *row[117] = combination_limb_8_col117;let combination_limb_9_col118 = combination_result_tmp_51986_48.get_m31(9);
            *row[118] = combination_limb_9_col118;let biased_limb_accumulator_u32_tmp_51986_49 = PackedUInt32::from_m31(((((((((M31_0) - (combination_limb_0_col109))) + (((M31_1) * (((((input_state_2_limb_0_col59) + (((input_state_2_limb_1_col60) * (M31_512))))) + (((input_state_2_limb_2_col61) * (M31_262144))))))))) + (M31_4883209))) + (M31_134217729)));let p_coef_col119 = ((biased_limb_accumulator_u32_tmp_51986_49.low().as_m31()) - (M31_1));
            *row[119] = p_coef_col119;let carry_0_tmp_51986_50 = ((((((((((M31_0) - (combination_limb_0_col109))) + (((M31_1) * (((((input_state_2_limb_0_col59) + (((input_state_2_limb_1_col60) * (M31_512))))) + (((input_state_2_limb_2_col61) * (M31_262144))))))))) + (M31_4883209))) - (((p_coef_col119) * (M31_1))))) * (M31_16));let carry_1_tmp_51986_51 = ((((((((((carry_0_tmp_51986_50) - (combination_limb_1_col110))) + (((M31_1) * (((((input_state_2_limb_3_col62) + (((input_state_2_limb_4_col63) * (M31_512))))) + (((input_state_2_limb_5_col64) * (M31_262144))))))))) + (M31_28820206))) - (((p_coef_col119) * (M31_0))))) * (M31_16));let carry_2_tmp_51986_52 = ((((((((((carry_1_tmp_51986_51) - (combination_limb_2_col111))) + (((M31_1) * (((((input_state_2_limb_6_col65) + (((input_state_2_limb_7_col66) * (M31_512))))) + (((input_state_2_limb_8_col67) * (M31_262144))))))))) + (M31_79012328))) - (((p_coef_col119) * (M31_0))))) * (M31_16));let carry_3_tmp_51986_53 = ((((((((((carry_2_tmp_51986_52) - (combination_limb_3_col112))) + (((M31_1) * (((((input_state_2_limb_9_col68) + (((input_state_2_limb_10_col69) * (M31_512))))) + (((input_state_2_limb_11_col70) * (M31_262144))))))))) + (M31_49157069))) - (((p_coef_col119) * (M31_0))))) * (M31_16));let carry_4_tmp_51986_54 = ((((((((((carry_3_tmp_51986_53) - (combination_limb_4_col113))) + (((M31_1) * (((((input_state_2_limb_12_col71) + (((input_state_2_limb_13_col72) * (M31_512))))) + (((input_state_2_limb_14_col73) * (M31_262144))))))))) + (M31_78826183))) - (((p_coef_col119) * (M31_0))))) * (M31_16));let carry_5_tmp_51986_55 = ((((((((((carry_4_tmp_51986_54) - (combination_limb_5_col114))) + (((M31_1) * (((((input_state_2_limb_15_col74) + (((input_state_2_limb_16_col75) * (M31_512))))) + (((input_state_2_limb_17_col76) * (M31_262144))))))))) + (M31_72285071))) - (((p_coef_col119) * (M31_0))))) * (M31_16));let carry_6_tmp_51986_56 = ((((((((((carry_5_tmp_51986_55) - (combination_limb_6_col115))) + (((M31_1) * (((((input_state_2_limb_18_col77) + (((input_state_2_limb_19_col78) * (M31_512))))) + (((input_state_2_limb_20_col79) * (M31_262144))))))))) + (M31_33413160))) - (((p_coef_col119) * (M31_0))))) * (M31_16));let carry_7_tmp_51986_57 = ((((((((((carry_6_tmp_51986_56) - (combination_limb_7_col116))) + (((M31_1) * (((((input_state_2_limb_21_col80) + (((input_state_2_limb_22_col81) * (M31_512))))) + (((input_state_2_limb_23_col82) * (M31_262144))))))))) + (M31_90842759))) - (((p_coef_col119) * (M31_136))))) * (M31_16));let carry_8_tmp_51986_58 = ((((((((((carry_7_tmp_51986_57) - (combination_limb_8_col117))) + (((M31_1) * (((((input_state_2_limb_24_col83) + (((input_state_2_limb_25_col84) * (M31_512))))) + (((input_state_2_limb_26_col85) * (M31_262144))))))))) + (M31_60124463))) - (((p_coef_col119) * (M31_0))))) * (M31_16));

            *lookup_data.poseidon_full_round_chain_0 = [((seq) * (M31_2)), M31_0, combination_limb_0_col87, combination_limb_1_col88, combination_limb_2_col89, combination_limb_3_col90, combination_limb_4_col91, combination_limb_5_col92, combination_limb_6_col93, combination_limb_7_col94, combination_limb_8_col95, combination_limb_9_col96, combination_limb_0_col98, combination_limb_1_col99, combination_limb_2_col100, combination_limb_3_col101, combination_limb_4_col102, combination_limb_5_col103, combination_limb_6_col104, combination_limb_7_col105, combination_limb_8_col106, combination_limb_9_col107, combination_limb_0_col109, combination_limb_1_col110, combination_limb_2_col111, combination_limb_3_col112, combination_limb_4_col113, combination_limb_5_col114, combination_limb_6_col115, combination_limb_7_col116, combination_limb_8_col117, combination_limb_9_col118];let poseidon_full_round_chain_inputs_0 =
                (((seq) * (M31_2)), M31_0, [combination_result_tmp_51986_6, combination_result_tmp_51986_27, combination_result_tmp_51986_48]).unpack();
            let poseidon_full_round_chain_output_round_0_tmp_51986_69 = poseidon_full_round_chain_state.deduce_output((((seq) * (M31_2)), M31_0, [combination_result_tmp_51986_6, combination_result_tmp_51986_27, combination_result_tmp_51986_48]));let poseidon_full_round_chain_inputs_1 =
                (((seq) * (M31_2)), M31_1, [poseidon_full_round_chain_output_round_0_tmp_51986_69.2[0], poseidon_full_round_chain_output_round_0_tmp_51986_69.2[1], poseidon_full_round_chain_output_round_0_tmp_51986_69.2[2]]).unpack();
            let poseidon_full_round_chain_output_round_1_tmp_51986_70 = poseidon_full_round_chain_state.deduce_output((((seq) * (M31_2)), M31_1, [poseidon_full_round_chain_output_round_0_tmp_51986_69.2[0], poseidon_full_round_chain_output_round_0_tmp_51986_69.2[1], poseidon_full_round_chain_output_round_0_tmp_51986_69.2[2]]));let poseidon_full_round_chain_inputs_2 =
                (((seq) * (M31_2)), M31_2, [poseidon_full_round_chain_output_round_1_tmp_51986_70.2[0], poseidon_full_round_chain_output_round_1_tmp_51986_70.2[1], poseidon_full_round_chain_output_round_1_tmp_51986_70.2[2]]).unpack();
            let poseidon_full_round_chain_output_round_2_tmp_51986_71 = poseidon_full_round_chain_state.deduce_output((((seq) * (M31_2)), M31_2, [poseidon_full_round_chain_output_round_1_tmp_51986_70.2[0], poseidon_full_round_chain_output_round_1_tmp_51986_70.2[1], poseidon_full_round_chain_output_round_1_tmp_51986_70.2[2]]));let poseidon_full_round_chain_inputs_3 =
                (((seq) * (M31_2)), M31_3, [poseidon_full_round_chain_output_round_2_tmp_51986_71.2[0], poseidon_full_round_chain_output_round_2_tmp_51986_71.2[1], poseidon_full_round_chain_output_round_2_tmp_51986_71.2[2]]).unpack();
            let poseidon_full_round_chain_output_round_3_tmp_51986_72 = poseidon_full_round_chain_state.deduce_output((((seq) * (M31_2)), M31_3, [poseidon_full_round_chain_output_round_2_tmp_51986_71.2[0], poseidon_full_round_chain_output_round_2_tmp_51986_71.2[1], poseidon_full_round_chain_output_round_2_tmp_51986_71.2[2]]));let poseidon_full_round_chain_output_limb_0_col120 = poseidon_full_round_chain_output_round_3_tmp_51986_72.0;
            *row[120] = poseidon_full_round_chain_output_limb_0_col120;let poseidon_full_round_chain_output_limb_1_col121 = poseidon_full_round_chain_output_round_3_tmp_51986_72.1;
            *row[121] = poseidon_full_round_chain_output_limb_1_col121;let poseidon_full_round_chain_output_limb_2_col122 = poseidon_full_round_chain_output_round_3_tmp_51986_72.2[0].get_m31(0);
            *row[122] = poseidon_full_round_chain_output_limb_2_col122;let poseidon_full_round_chain_output_limb_3_col123 = poseidon_full_round_chain_output_round_3_tmp_51986_72.2[0].get_m31(1);
            *row[123] = poseidon_full_round_chain_output_limb_3_col123;let poseidon_full_round_chain_output_limb_4_col124 = poseidon_full_round_chain_output_round_3_tmp_51986_72.2[0].get_m31(2);
            *row[124] = poseidon_full_round_chain_output_limb_4_col124;let poseidon_full_round_chain_output_limb_5_col125 = poseidon_full_round_chain_output_round_3_tmp_51986_72.2[0].get_m31(3);
            *row[125] = poseidon_full_round_chain_output_limb_5_col125;let poseidon_full_round_chain_output_limb_6_col126 = poseidon_full_round_chain_output_round_3_tmp_51986_72.2[0].get_m31(4);
            *row[126] = poseidon_full_round_chain_output_limb_6_col126;let poseidon_full_round_chain_output_limb_7_col127 = poseidon_full_round_chain_output_round_3_tmp_51986_72.2[0].get_m31(5);
            *row[127] = poseidon_full_round_chain_output_limb_7_col127;let poseidon_full_round_chain_output_limb_8_col128 = poseidon_full_round_chain_output_round_3_tmp_51986_72.2[0].get_m31(6);
            *row[128] = poseidon_full_round_chain_output_limb_8_col128;let poseidon_full_round_chain_output_limb_9_col129 = poseidon_full_round_chain_output_round_3_tmp_51986_72.2[0].get_m31(7);
            *row[129] = poseidon_full_round_chain_output_limb_9_col129;let poseidon_full_round_chain_output_limb_10_col130 = poseidon_full_round_chain_output_round_3_tmp_51986_72.2[0].get_m31(8);
            *row[130] = poseidon_full_round_chain_output_limb_10_col130;let poseidon_full_round_chain_output_limb_11_col131 = poseidon_full_round_chain_output_round_3_tmp_51986_72.2[0].get_m31(9);
            *row[131] = poseidon_full_round_chain_output_limb_11_col131;let poseidon_full_round_chain_output_limb_12_col132 = poseidon_full_round_chain_output_round_3_tmp_51986_72.2[1].get_m31(0);
            *row[132] = poseidon_full_round_chain_output_limb_12_col132;let poseidon_full_round_chain_output_limb_13_col133 = poseidon_full_round_chain_output_round_3_tmp_51986_72.2[1].get_m31(1);
            *row[133] = poseidon_full_round_chain_output_limb_13_col133;let poseidon_full_round_chain_output_limb_14_col134 = poseidon_full_round_chain_output_round_3_tmp_51986_72.2[1].get_m31(2);
            *row[134] = poseidon_full_round_chain_output_limb_14_col134;let poseidon_full_round_chain_output_limb_15_col135 = poseidon_full_round_chain_output_round_3_tmp_51986_72.2[1].get_m31(3);
            *row[135] = poseidon_full_round_chain_output_limb_15_col135;let poseidon_full_round_chain_output_limb_16_col136 = poseidon_full_round_chain_output_round_3_tmp_51986_72.2[1].get_m31(4);
            *row[136] = poseidon_full_round_chain_output_limb_16_col136;let poseidon_full_round_chain_output_limb_17_col137 = poseidon_full_round_chain_output_round_3_tmp_51986_72.2[1].get_m31(5);
            *row[137] = poseidon_full_round_chain_output_limb_17_col137;let poseidon_full_round_chain_output_limb_18_col138 = poseidon_full_round_chain_output_round_3_tmp_51986_72.2[1].get_m31(6);
            *row[138] = poseidon_full_round_chain_output_limb_18_col138;let poseidon_full_round_chain_output_limb_19_col139 = poseidon_full_round_chain_output_round_3_tmp_51986_72.2[1].get_m31(7);
            *row[139] = poseidon_full_round_chain_output_limb_19_col139;let poseidon_full_round_chain_output_limb_20_col140 = poseidon_full_round_chain_output_round_3_tmp_51986_72.2[1].get_m31(8);
            *row[140] = poseidon_full_round_chain_output_limb_20_col140;let poseidon_full_round_chain_output_limb_21_col141 = poseidon_full_round_chain_output_round_3_tmp_51986_72.2[1].get_m31(9);
            *row[141] = poseidon_full_round_chain_output_limb_21_col141;let poseidon_full_round_chain_output_limb_22_col142 = poseidon_full_round_chain_output_round_3_tmp_51986_72.2[2].get_m31(0);
            *row[142] = poseidon_full_round_chain_output_limb_22_col142;let poseidon_full_round_chain_output_limb_23_col143 = poseidon_full_round_chain_output_round_3_tmp_51986_72.2[2].get_m31(1);
            *row[143] = poseidon_full_round_chain_output_limb_23_col143;let poseidon_full_round_chain_output_limb_24_col144 = poseidon_full_round_chain_output_round_3_tmp_51986_72.2[2].get_m31(2);
            *row[144] = poseidon_full_round_chain_output_limb_24_col144;let poseidon_full_round_chain_output_limb_25_col145 = poseidon_full_round_chain_output_round_3_tmp_51986_72.2[2].get_m31(3);
            *row[145] = poseidon_full_round_chain_output_limb_25_col145;let poseidon_full_round_chain_output_limb_26_col146 = poseidon_full_round_chain_output_round_3_tmp_51986_72.2[2].get_m31(4);
            *row[146] = poseidon_full_round_chain_output_limb_26_col146;let poseidon_full_round_chain_output_limb_27_col147 = poseidon_full_round_chain_output_round_3_tmp_51986_72.2[2].get_m31(5);
            *row[147] = poseidon_full_round_chain_output_limb_27_col147;let poseidon_full_round_chain_output_limb_28_col148 = poseidon_full_round_chain_output_round_3_tmp_51986_72.2[2].get_m31(6);
            *row[148] = poseidon_full_round_chain_output_limb_28_col148;let poseidon_full_round_chain_output_limb_29_col149 = poseidon_full_round_chain_output_round_3_tmp_51986_72.2[2].get_m31(7);
            *row[149] = poseidon_full_round_chain_output_limb_29_col149;let poseidon_full_round_chain_output_limb_30_col150 = poseidon_full_round_chain_output_round_3_tmp_51986_72.2[2].get_m31(8);
            *row[150] = poseidon_full_round_chain_output_limb_30_col150;let poseidon_full_round_chain_output_limb_31_col151 = poseidon_full_round_chain_output_round_3_tmp_51986_72.2[2].get_m31(9);
            *row[151] = poseidon_full_round_chain_output_limb_31_col151;*lookup_data.poseidon_full_round_chain_1 = [poseidon_full_round_chain_output_limb_0_col120, poseidon_full_round_chain_output_limb_1_col121, poseidon_full_round_chain_output_limb_2_col122, poseidon_full_round_chain_output_limb_3_col123, poseidon_full_round_chain_output_limb_4_col124, poseidon_full_round_chain_output_limb_5_col125, poseidon_full_round_chain_output_limb_6_col126, poseidon_full_round_chain_output_limb_7_col127, poseidon_full_round_chain_output_limb_8_col128, poseidon_full_round_chain_output_limb_9_col129, poseidon_full_round_chain_output_limb_10_col130, poseidon_full_round_chain_output_limb_11_col131, poseidon_full_round_chain_output_limb_12_col132, poseidon_full_round_chain_output_limb_13_col133, poseidon_full_round_chain_output_limb_14_col134, poseidon_full_round_chain_output_limb_15_col135, poseidon_full_round_chain_output_limb_16_col136, poseidon_full_round_chain_output_limb_17_col137, poseidon_full_round_chain_output_limb_18_col138, poseidon_full_round_chain_output_limb_19_col139, poseidon_full_round_chain_output_limb_20_col140, poseidon_full_round_chain_output_limb_21_col141, poseidon_full_round_chain_output_limb_22_col142, poseidon_full_round_chain_output_limb_23_col143, poseidon_full_round_chain_output_limb_24_col144, poseidon_full_round_chain_output_limb_25_col145, poseidon_full_round_chain_output_limb_26_col146, poseidon_full_round_chain_output_limb_27_col147, poseidon_full_round_chain_output_limb_28_col148, poseidon_full_round_chain_output_limb_29_col149, poseidon_full_round_chain_output_limb_30_col150, poseidon_full_round_chain_output_limb_31_col151];let range_check_felt_252_width_27_inputs_0 =
                poseidon_full_round_chain_output_round_3_tmp_51986_72.2[0].unpack();
            *lookup_data.range_check_felt_252_width_27_0 = [poseidon_full_round_chain_output_limb_2_col122, poseidon_full_round_chain_output_limb_3_col123, poseidon_full_round_chain_output_limb_4_col124, poseidon_full_round_chain_output_limb_5_col125, poseidon_full_round_chain_output_limb_6_col126, poseidon_full_round_chain_output_limb_7_col127, poseidon_full_round_chain_output_limb_8_col128, poseidon_full_round_chain_output_limb_9_col129, poseidon_full_round_chain_output_limb_10_col130, poseidon_full_round_chain_output_limb_11_col131];let range_check_felt_252_width_27_inputs_1 =
                poseidon_full_round_chain_output_round_3_tmp_51986_72.2[1].unpack();
            *lookup_data.range_check_felt_252_width_27_1 = [poseidon_full_round_chain_output_limb_12_col132, poseidon_full_round_chain_output_limb_13_col133, poseidon_full_round_chain_output_limb_14_col134, poseidon_full_round_chain_output_limb_15_col135, poseidon_full_round_chain_output_limb_16_col136, poseidon_full_round_chain_output_limb_17_col137, poseidon_full_round_chain_output_limb_18_col138, poseidon_full_round_chain_output_limb_19_col139, poseidon_full_round_chain_output_limb_20_col140, poseidon_full_round_chain_output_limb_21_col141];let cube_252_inputs_0 =
                poseidon_full_round_chain_output_round_3_tmp_51986_72.2[2].unpack();
            let cube_252_output_tmp_51986_73 = cube_252_state.deduce_output(poseidon_full_round_chain_output_round_3_tmp_51986_72.2[2]);let cube_252_output_limb_0_col152 = cube_252_output_tmp_51986_73.get_m31(0);
            *row[152] = cube_252_output_limb_0_col152;let cube_252_output_limb_1_col153 = cube_252_output_tmp_51986_73.get_m31(1);
            *row[153] = cube_252_output_limb_1_col153;let cube_252_output_limb_2_col154 = cube_252_output_tmp_51986_73.get_m31(2);
            *row[154] = cube_252_output_limb_2_col154;let cube_252_output_limb_3_col155 = cube_252_output_tmp_51986_73.get_m31(3);
            *row[155] = cube_252_output_limb_3_col155;let cube_252_output_limb_4_col156 = cube_252_output_tmp_51986_73.get_m31(4);
            *row[156] = cube_252_output_limb_4_col156;let cube_252_output_limb_5_col157 = cube_252_output_tmp_51986_73.get_m31(5);
            *row[157] = cube_252_output_limb_5_col157;let cube_252_output_limb_6_col158 = cube_252_output_tmp_51986_73.get_m31(6);
            *row[158] = cube_252_output_limb_6_col158;let cube_252_output_limb_7_col159 = cube_252_output_tmp_51986_73.get_m31(7);
            *row[159] = cube_252_output_limb_7_col159;let cube_252_output_limb_8_col160 = cube_252_output_tmp_51986_73.get_m31(8);
            *row[160] = cube_252_output_limb_8_col160;let cube_252_output_limb_9_col161 = cube_252_output_tmp_51986_73.get_m31(9);
            *row[161] = cube_252_output_limb_9_col161;*lookup_data.cube_252_0 = [poseidon_full_round_chain_output_limb_22_col142, poseidon_full_round_chain_output_limb_23_col143, poseidon_full_round_chain_output_limb_24_col144, poseidon_full_round_chain_output_limb_25_col145, poseidon_full_round_chain_output_limb_26_col146, poseidon_full_round_chain_output_limb_27_col147, poseidon_full_round_chain_output_limb_28_col148, poseidon_full_round_chain_output_limb_29_col149, poseidon_full_round_chain_output_limb_30_col150, poseidon_full_round_chain_output_limb_31_col151, cube_252_output_limb_0_col152, cube_252_output_limb_1_col153, cube_252_output_limb_2_col154, cube_252_output_limb_3_col155, cube_252_output_limb_4_col156, cube_252_output_limb_5_col157, cube_252_output_limb_6_col158, cube_252_output_limb_7_col159, cube_252_output_limb_8_col160, cube_252_output_limb_9_col161];

            //Linear Combination N 4 Coefs 1 1 M 2 1.

            let combination_result_tmp_51986_74 = PackedFelt252Width27::from_packed_felt252(((((((((Felt252_0_0_0_0) + (((Felt252_1_0_0_0) * (PackedFelt252::from_packed_felt252width27(poseidon_full_round_chain_output_round_3_tmp_51986_72.2[0])))))) + (((Felt252_1_0_0_0) * (PackedFelt252::from_packed_felt252width27(poseidon_full_round_chain_output_round_3_tmp_51986_72.2[1])))))) - (((Felt252_2_0_0_0) * (PackedFelt252::from_packed_felt252width27(cube_252_output_tmp_51986_73)))))) + (Felt252_11041071929982523380_7503192613203831446_4943121247101232560_560497091765764140)));let combination_limb_0_col162 = combination_result_tmp_51986_74.get_m31(0);
            *row[162] = combination_limb_0_col162;let combination_limb_1_col163 = combination_result_tmp_51986_74.get_m31(1);
            *row[163] = combination_limb_1_col163;let combination_limb_2_col164 = combination_result_tmp_51986_74.get_m31(2);
            *row[164] = combination_limb_2_col164;let combination_limb_3_col165 = combination_result_tmp_51986_74.get_m31(3);
            *row[165] = combination_limb_3_col165;let combination_limb_4_col166 = combination_result_tmp_51986_74.get_m31(4);
            *row[166] = combination_limb_4_col166;let combination_limb_5_col167 = combination_result_tmp_51986_74.get_m31(5);
            *row[167] = combination_limb_5_col167;let combination_limb_6_col168 = combination_result_tmp_51986_74.get_m31(6);
            *row[168] = combination_limb_6_col168;let combination_limb_7_col169 = combination_result_tmp_51986_74.get_m31(7);
            *row[169] = combination_limb_7_col169;let combination_limb_8_col170 = combination_result_tmp_51986_74.get_m31(8);
            *row[170] = combination_limb_8_col170;let combination_limb_9_col171 = combination_result_tmp_51986_74.get_m31(9);
            *row[171] = combination_limb_9_col171;let biased_limb_accumulator_u32_tmp_51986_75 = PackedUInt32::from_m31(((((((((((((M31_0) - (combination_limb_0_col162))) + (((M31_1) * (poseidon_full_round_chain_output_limb_2_col122))))) + (((M31_1) * (poseidon_full_round_chain_output_limb_12_col132))))) - (((M31_2) * (cube_252_output_limb_0_col152))))) + (M31_103094260))) + (M31_402653187)));let p_coef_col172 = ((biased_limb_accumulator_u32_tmp_51986_75.low().as_m31()) - (M31_3));
            *row[172] = p_coef_col172;let carry_0_tmp_51986_76 = ((((((((((((((M31_0) - (combination_limb_0_col162))) + (((M31_1) * (poseidon_full_round_chain_output_limb_2_col122))))) + (((M31_1) * (poseidon_full_round_chain_output_limb_12_col132))))) - (((M31_2) * (cube_252_output_limb_0_col152))))) + (M31_103094260))) - (((p_coef_col172) * (M31_1))))) * (M31_16));let carry_1_tmp_51986_77 = ((((((((((((((carry_0_tmp_51986_76) - (combination_limb_1_col163))) + (((M31_1) * (poseidon_full_round_chain_output_limb_3_col123))))) + (((M31_1) * (poseidon_full_round_chain_output_limb_13_col133))))) - (((M31_2) * (cube_252_output_limb_1_col153))))) + (M31_121146754))) - (((p_coef_col172) * (M31_0))))) * (M31_16));let carry_2_tmp_51986_78 = ((((((((((((((carry_1_tmp_51986_77) - (combination_limb_2_col164))) + (((M31_1) * (poseidon_full_round_chain_output_limb_4_col124))))) + (((M31_1) * (poseidon_full_round_chain_output_limb_14_col134))))) - (((M31_2) * (cube_252_output_limb_2_col154))))) + (M31_95050340))) - (((p_coef_col172) * (M31_0))))) * (M31_16));let carry_3_tmp_51986_79 = ((((((((((((((carry_2_tmp_51986_78) - (combination_limb_3_col165))) + (((M31_1) * (poseidon_full_round_chain_output_limb_5_col125))))) + (((M31_1) * (poseidon_full_round_chain_output_limb_15_col135))))) - (((M31_2) * (cube_252_output_limb_3_col155))))) + (M31_16173996))) - (((p_coef_col172) * (M31_0))))) * (M31_16));let carry_4_tmp_51986_80 = ((((((((((((((carry_3_tmp_51986_79) - (combination_limb_4_col166))) + (((M31_1) * (poseidon_full_round_chain_output_limb_6_col126))))) + (((M31_1) * (poseidon_full_round_chain_output_limb_16_col136))))) - (((M31_2) * (cube_252_output_limb_4_col156))))) + (M31_50758155))) - (((p_coef_col172) * (M31_0))))) * (M31_16));let carry_5_tmp_51986_81 = ((((((((((((((carry_4_tmp_51986_80) - (combination_limb_5_col167))) + (((M31_1) * (poseidon_full_round_chain_output_limb_7_col127))))) + (((M31_1) * (poseidon_full_round_chain_output_limb_17_col137))))) - (((M31_2) * (cube_252_output_limb_5_col157))))) + (M31_54415179))) - (((p_coef_col172) * (M31_0))))) * (M31_16));let carry_6_tmp_51986_82 = ((((((((((((((carry_5_tmp_51986_81) - (combination_limb_6_col168))) + (((M31_1) * (poseidon_full_round_chain_output_limb_8_col128))))) + (((M31_1) * (poseidon_full_round_chain_output_limb_18_col138))))) - (((M31_2) * (cube_252_output_limb_6_col158))))) + (M31_19292069))) - (((p_coef_col172) * (M31_0))))) * (M31_16));let carry_7_tmp_51986_83 = ((((((((((((((carry_6_tmp_51986_82) - (combination_limb_7_col169))) + (((M31_1) * (poseidon_full_round_chain_output_limb_9_col129))))) + (((M31_1) * (poseidon_full_round_chain_output_limb_19_col139))))) - (((M31_2) * (cube_252_output_limb_7_col159))))) + (M31_45351266))) - (((p_coef_col172) * (M31_136))))) * (M31_16));let carry_8_tmp_51986_84 = ((((((((((((((carry_7_tmp_51986_83) - (combination_limb_8_col170))) + (((M31_1) * (poseidon_full_round_chain_output_limb_10_col130))))) + (((M31_1) * (poseidon_full_round_chain_output_limb_20_col140))))) - (((M31_2) * (cube_252_output_limb_8_col160))))) + (M31_122233508))) - (((p_coef_col172) * (M31_0))))) * (M31_16));let range_check_3_3_3_3_3_inputs_0 =
                [((p_coef_col172) + (M31_3)), ((carry_0_tmp_51986_76) + (M31_3)), ((carry_1_tmp_51986_77) + (M31_3)), ((carry_2_tmp_51986_78) + (M31_3)), ((carry_3_tmp_51986_79) + (M31_3))].unpack();
            *lookup_data.range_check_3_3_3_3_3_0 = [((p_coef_col172) + (M31_3)), ((carry_0_tmp_51986_76) + (M31_3)), ((carry_1_tmp_51986_77) + (M31_3)), ((carry_2_tmp_51986_78) + (M31_3)), ((carry_3_tmp_51986_79) + (M31_3))];let range_check_3_3_3_3_3_inputs_1 =
                [((carry_4_tmp_51986_80) + (M31_3)), ((carry_5_tmp_51986_81) + (M31_3)), ((carry_6_tmp_51986_82) + (M31_3)), ((carry_7_tmp_51986_83) + (M31_3)), ((carry_8_tmp_51986_84) + (M31_3))].unpack();
            *lookup_data.range_check_3_3_3_3_3_1 = [((carry_4_tmp_51986_80) + (M31_3)), ((carry_5_tmp_51986_81) + (M31_3)), ((carry_6_tmp_51986_82) + (M31_3)), ((carry_7_tmp_51986_83) + (M31_3)), ((carry_8_tmp_51986_84) + (M31_3))];

            let cube_252_inputs_1 =
                combination_result_tmp_51986_74.unpack();
            let cube_252_output_tmp_51986_85 = cube_252_state.deduce_output(combination_result_tmp_51986_74);let cube_252_output_limb_0_col173 = cube_252_output_tmp_51986_85.get_m31(0);
            *row[173] = cube_252_output_limb_0_col173;let cube_252_output_limb_1_col174 = cube_252_output_tmp_51986_85.get_m31(1);
            *row[174] = cube_252_output_limb_1_col174;let cube_252_output_limb_2_col175 = cube_252_output_tmp_51986_85.get_m31(2);
            *row[175] = cube_252_output_limb_2_col175;let cube_252_output_limb_3_col176 = cube_252_output_tmp_51986_85.get_m31(3);
            *row[176] = cube_252_output_limb_3_col176;let cube_252_output_limb_4_col177 = cube_252_output_tmp_51986_85.get_m31(4);
            *row[177] = cube_252_output_limb_4_col177;let cube_252_output_limb_5_col178 = cube_252_output_tmp_51986_85.get_m31(5);
            *row[178] = cube_252_output_limb_5_col178;let cube_252_output_limb_6_col179 = cube_252_output_tmp_51986_85.get_m31(6);
            *row[179] = cube_252_output_limb_6_col179;let cube_252_output_limb_7_col180 = cube_252_output_tmp_51986_85.get_m31(7);
            *row[180] = cube_252_output_limb_7_col180;let cube_252_output_limb_8_col181 = cube_252_output_tmp_51986_85.get_m31(8);
            *row[181] = cube_252_output_limb_8_col181;let cube_252_output_limb_9_col182 = cube_252_output_tmp_51986_85.get_m31(9);
            *row[182] = cube_252_output_limb_9_col182;*lookup_data.cube_252_1 = [combination_limb_0_col162, combination_limb_1_col163, combination_limb_2_col164, combination_limb_3_col165, combination_limb_4_col166, combination_limb_5_col167, combination_limb_6_col168, combination_limb_7_col169, combination_limb_8_col170, combination_limb_9_col171, cube_252_output_limb_0_col173, cube_252_output_limb_1_col174, cube_252_output_limb_2_col175, cube_252_output_limb_3_col176, cube_252_output_limb_4_col177, cube_252_output_limb_5_col178, cube_252_output_limb_6_col179, cube_252_output_limb_7_col180, cube_252_output_limb_8_col181, cube_252_output_limb_9_col182];

            //Linear Combination N 4 Coefs 4 2 M 2 1.

            let combination_result_tmp_51986_86 = PackedFelt252Width27::from_packed_felt252(((((((((Felt252_0_0_0_0) + (((Felt252_4_0_0_0) * (PackedFelt252::from_packed_felt252width27(poseidon_full_round_chain_output_round_3_tmp_51986_72.2[0])))))) + (((Felt252_2_0_0_0) * (PackedFelt252::from_packed_felt252width27(cube_252_output_tmp_51986_73)))))) - (((Felt252_2_0_0_0) * (PackedFelt252::from_packed_felt252width27(cube_252_output_tmp_51986_85)))))) + (Felt252_10931822301410252833_1475756362763989377_3378552166684303673_348229636055909092)));let combination_limb_0_col183 = combination_result_tmp_51986_86.get_m31(0);
            *row[183] = combination_limb_0_col183;let combination_limb_1_col184 = combination_result_tmp_51986_86.get_m31(1);
            *row[184] = combination_limb_1_col184;let combination_limb_2_col185 = combination_result_tmp_51986_86.get_m31(2);
            *row[185] = combination_limb_2_col185;let combination_limb_3_col186 = combination_result_tmp_51986_86.get_m31(3);
            *row[186] = combination_limb_3_col186;let combination_limb_4_col187 = combination_result_tmp_51986_86.get_m31(4);
            *row[187] = combination_limb_4_col187;let combination_limb_5_col188 = combination_result_tmp_51986_86.get_m31(5);
            *row[188] = combination_limb_5_col188;let combination_limb_6_col189 = combination_result_tmp_51986_86.get_m31(6);
            *row[189] = combination_limb_6_col189;let combination_limb_7_col190 = combination_result_tmp_51986_86.get_m31(7);
            *row[190] = combination_limb_7_col190;let combination_limb_8_col191 = combination_result_tmp_51986_86.get_m31(8);
            *row[191] = combination_limb_8_col191;let combination_limb_9_col192 = combination_result_tmp_51986_86.get_m31(9);
            *row[192] = combination_limb_9_col192;let biased_limb_accumulator_u32_tmp_51986_87 = PackedUInt32::from_m31(((((((((((((M31_0) - (combination_limb_0_col183))) + (((M31_4) * (poseidon_full_round_chain_output_limb_2_col122))))) + (((M31_2) * (cube_252_output_limb_0_col152))))) - (((M31_2) * (cube_252_output_limb_0_col173))))) + (M31_121657377))) + (M31_402653187)));let p_coef_col193 = ((biased_limb_accumulator_u32_tmp_51986_87.low().as_m31()) - (M31_3));
            *row[193] = p_coef_col193;let carry_0_tmp_51986_88 = ((((((((((((((M31_0) - (combination_limb_0_col183))) + (((M31_4) * (poseidon_full_round_chain_output_limb_2_col122))))) + (((M31_2) * (cube_252_output_limb_0_col152))))) - (((M31_2) * (cube_252_output_limb_0_col173))))) + (M31_121657377))) - (((p_coef_col193) * (M31_1))))) * (M31_16));let carry_1_tmp_51986_89 = ((((((((((((((carry_0_tmp_51986_88) - (combination_limb_1_col184))) + (((M31_4) * (poseidon_full_round_chain_output_limb_3_col123))))) + (((M31_2) * (cube_252_output_limb_1_col153))))) - (((M31_2) * (cube_252_output_limb_1_col174))))) + (M31_112479959))) - (((p_coef_col193) * (M31_0))))) * (M31_16));let carry_2_tmp_51986_90 = ((((((((((((((carry_1_tmp_51986_89) - (combination_limb_2_col185))) + (((M31_4) * (poseidon_full_round_chain_output_limb_4_col124))))) + (((M31_2) * (cube_252_output_limb_2_col154))))) - (((M31_2) * (cube_252_output_limb_2_col175))))) + (M31_130418270))) - (((p_coef_col193) * (M31_0))))) * (M31_16));let carry_3_tmp_51986_91 = ((((((((((((((carry_2_tmp_51986_90) - (combination_limb_3_col186))) + (((M31_4) * (poseidon_full_round_chain_output_limb_5_col125))))) + (((M31_2) * (cube_252_output_limb_3_col155))))) - (((M31_2) * (cube_252_output_limb_3_col176))))) + (M31_4974792))) - (((p_coef_col193) * (M31_0))))) * (M31_16));let carry_4_tmp_51986_92 = ((((((((((((((carry_3_tmp_51986_91) - (combination_limb_4_col187))) + (((M31_4) * (poseidon_full_round_chain_output_limb_6_col126))))) + (((M31_2) * (cube_252_output_limb_4_col156))))) - (((M31_2) * (cube_252_output_limb_4_col177))))) + (M31_59852719))) - (((p_coef_col193) * (M31_0))))) * (M31_16));let carry_5_tmp_51986_93 = ((((((((((((((carry_4_tmp_51986_92) - (combination_limb_5_col188))) + (((M31_4) * (poseidon_full_round_chain_output_limb_7_col127))))) + (((M31_2) * (cube_252_output_limb_5_col157))))) - (((M31_2) * (cube_252_output_limb_5_col178))))) + (M31_120369218))) - (((p_coef_col193) * (M31_0))))) * (M31_16));let carry_6_tmp_51986_94 = ((((((((((((((carry_5_tmp_51986_93) - (combination_limb_6_col189))) + (((M31_4) * (poseidon_full_round_chain_output_limb_8_col128))))) + (((M31_2) * (cube_252_output_limb_6_col158))))) - (((M31_2) * (cube_252_output_limb_6_col179))))) + (M31_62439890))) - (((p_coef_col193) * (M31_0))))) * (M31_16));let carry_7_tmp_51986_95 = ((((((((((((((carry_6_tmp_51986_94) - (combination_limb_7_col190))) + (((M31_4) * (poseidon_full_round_chain_output_limb_9_col129))))) + (((M31_2) * (cube_252_output_limb_7_col159))))) - (((M31_2) * (cube_252_output_limb_7_col180))))) + (M31_50468641))) - (((p_coef_col193) * (M31_136))))) * (M31_16));let carry_8_tmp_51986_96 = ((((((((((((((carry_7_tmp_51986_95) - (combination_limb_8_col191))) + (((M31_4) * (poseidon_full_round_chain_output_limb_10_col130))))) + (((M31_2) * (cube_252_output_limb_8_col160))))) - (((M31_2) * (cube_252_output_limb_8_col181))))) + (M31_86573645))) - (((p_coef_col193) * (M31_0))))) * (M31_16));let range_check_4_4_4_4_inputs_0 =
                [((p_coef_col193) + (M31_3)), ((carry_0_tmp_51986_88) + (M31_3)), ((carry_1_tmp_51986_89) + (M31_3)), ((carry_2_tmp_51986_90) + (M31_3))].unpack();
            *lookup_data.range_check_4_4_4_4_0 = [((p_coef_col193) + (M31_3)), ((carry_0_tmp_51986_88) + (M31_3)), ((carry_1_tmp_51986_89) + (M31_3)), ((carry_2_tmp_51986_90) + (M31_3))];let range_check_4_4_4_4_inputs_1 =
                [((carry_3_tmp_51986_91) + (M31_3)), ((carry_4_tmp_51986_92) + (M31_3)), ((carry_5_tmp_51986_93) + (M31_3)), ((carry_6_tmp_51986_94) + (M31_3))].unpack();
            *lookup_data.range_check_4_4_4_4_1 = [((carry_3_tmp_51986_91) + (M31_3)), ((carry_4_tmp_51986_92) + (M31_3)), ((carry_5_tmp_51986_93) + (M31_3)), ((carry_6_tmp_51986_94) + (M31_3))];let range_check_4_4_inputs_0 =
                [((carry_7_tmp_51986_95) + (M31_3)), ((carry_8_tmp_51986_96) + (M31_3))].unpack();
            *lookup_data.range_check_4_4_0 = [((carry_7_tmp_51986_95) + (M31_3)), ((carry_8_tmp_51986_96) + (M31_3))];

            *lookup_data.poseidon_3_partial_rounds_chain_0 = [((seq) * (M31_1)), M31_4, cube_252_output_limb_0_col152, cube_252_output_limb_1_col153, cube_252_output_limb_2_col154, cube_252_output_limb_3_col155, cube_252_output_limb_4_col156, cube_252_output_limb_5_col157, cube_252_output_limb_6_col158, cube_252_output_limb_7_col159, cube_252_output_limb_8_col160, cube_252_output_limb_9_col161, combination_limb_0_col162, combination_limb_1_col163, combination_limb_2_col164, combination_limb_3_col165, combination_limb_4_col166, combination_limb_5_col167, combination_limb_6_col168, combination_limb_7_col169, combination_limb_8_col170, combination_limb_9_col171, cube_252_output_limb_0_col173, cube_252_output_limb_1_col174, cube_252_output_limb_2_col175, cube_252_output_limb_3_col176, cube_252_output_limb_4_col177, cube_252_output_limb_5_col178, cube_252_output_limb_6_col179, cube_252_output_limb_7_col180, cube_252_output_limb_8_col181, cube_252_output_limb_9_col182, combination_limb_0_col183, combination_limb_1_col184, combination_limb_2_col185, combination_limb_3_col186, combination_limb_4_col187, combination_limb_5_col188, combination_limb_6_col189, combination_limb_7_col190, combination_limb_8_col191, combination_limb_9_col192];let poseidon_3_partial_rounds_chain_inputs_0 =
                (((seq) * (M31_1)), M31_4, [cube_252_output_tmp_51986_73, combination_result_tmp_51986_74, cube_252_output_tmp_51986_85, combination_result_tmp_51986_86]).unpack();
            let poseidon_3_partial_rounds_chain_output_round_4_tmp_51986_97 = poseidon_3_partial_rounds_chain_state.deduce_output((((seq) * (M31_1)), M31_4, [cube_252_output_tmp_51986_73, combination_result_tmp_51986_74, cube_252_output_tmp_51986_85, combination_result_tmp_51986_86]));let poseidon_3_partial_rounds_chain_inputs_1 =
                (((seq) * (M31_1)), M31_5, [poseidon_3_partial_rounds_chain_output_round_4_tmp_51986_97.2[0], poseidon_3_partial_rounds_chain_output_round_4_tmp_51986_97.2[1], poseidon_3_partial_rounds_chain_output_round_4_tmp_51986_97.2[2], poseidon_3_partial_rounds_chain_output_round_4_tmp_51986_97.2[3]]).unpack();
            let poseidon_3_partial_rounds_chain_output_round_5_tmp_51986_98 = poseidon_3_partial_rounds_chain_state.deduce_output((((seq) * (M31_1)), M31_5, [poseidon_3_partial_rounds_chain_output_round_4_tmp_51986_97.2[0], poseidon_3_partial_rounds_chain_output_round_4_tmp_51986_97.2[1], poseidon_3_partial_rounds_chain_output_round_4_tmp_51986_97.2[2], poseidon_3_partial_rounds_chain_output_round_4_tmp_51986_97.2[3]]));let poseidon_3_partial_rounds_chain_inputs_2 =
                (((seq) * (M31_1)), M31_6, [poseidon_3_partial_rounds_chain_output_round_5_tmp_51986_98.2[0], poseidon_3_partial_rounds_chain_output_round_5_tmp_51986_98.2[1], poseidon_3_partial_rounds_chain_output_round_5_tmp_51986_98.2[2], poseidon_3_partial_rounds_chain_output_round_5_tmp_51986_98.2[3]]).unpack();
            let poseidon_3_partial_rounds_chain_output_round_6_tmp_51986_99 = poseidon_3_partial_rounds_chain_state.deduce_output((((seq) * (M31_1)), M31_6, [poseidon_3_partial_rounds_chain_output_round_5_tmp_51986_98.2[0], poseidon_3_partial_rounds_chain_output_round_5_tmp_51986_98.2[1], poseidon_3_partial_rounds_chain_output_round_5_tmp_51986_98.2[2], poseidon_3_partial_rounds_chain_output_round_5_tmp_51986_98.2[3]]));let poseidon_3_partial_rounds_chain_inputs_3 =
                (((seq) * (M31_1)), M31_7, [poseidon_3_partial_rounds_chain_output_round_6_tmp_51986_99.2[0], poseidon_3_partial_rounds_chain_output_round_6_tmp_51986_99.2[1], poseidon_3_partial_rounds_chain_output_round_6_tmp_51986_99.2[2], poseidon_3_partial_rounds_chain_output_round_6_tmp_51986_99.2[3]]).unpack();
            let poseidon_3_partial_rounds_chain_output_round_7_tmp_51986_100 = poseidon_3_partial_rounds_chain_state.deduce_output((((seq) * (M31_1)), M31_7, [poseidon_3_partial_rounds_chain_output_round_6_tmp_51986_99.2[0], poseidon_3_partial_rounds_chain_output_round_6_tmp_51986_99.2[1], poseidon_3_partial_rounds_chain_output_round_6_tmp_51986_99.2[2], poseidon_3_partial_rounds_chain_output_round_6_tmp_51986_99.2[3]]));let poseidon_3_partial_rounds_chain_inputs_4 =
                (((seq) * (M31_1)), M31_8, [poseidon_3_partial_rounds_chain_output_round_7_tmp_51986_100.2[0], poseidon_3_partial_rounds_chain_output_round_7_tmp_51986_100.2[1], poseidon_3_partial_rounds_chain_output_round_7_tmp_51986_100.2[2], poseidon_3_partial_rounds_chain_output_round_7_tmp_51986_100.2[3]]).unpack();
            let poseidon_3_partial_rounds_chain_output_round_8_tmp_51986_101 = poseidon_3_partial_rounds_chain_state.deduce_output((((seq) * (M31_1)), M31_8, [poseidon_3_partial_rounds_chain_output_round_7_tmp_51986_100.2[0], poseidon_3_partial_rounds_chain_output_round_7_tmp_51986_100.2[1], poseidon_3_partial_rounds_chain_output_round_7_tmp_51986_100.2[2], poseidon_3_partial_rounds_chain_output_round_7_tmp_51986_100.2[3]]));let poseidon_3_partial_rounds_chain_inputs_5 =
                (((seq) * (M31_1)), M31_9, [poseidon_3_partial_rounds_chain_output_round_8_tmp_51986_101.2[0], poseidon_3_partial_rounds_chain_output_round_8_tmp_51986_101.2[1], poseidon_3_partial_rounds_chain_output_round_8_tmp_51986_101.2[2], poseidon_3_partial_rounds_chain_output_round_8_tmp_51986_101.2[3]]).unpack();
            let poseidon_3_partial_rounds_chain_output_round_9_tmp_51986_102 = poseidon_3_partial_rounds_chain_state.deduce_output((((seq) * (M31_1)), M31_9, [poseidon_3_partial_rounds_chain_output_round_8_tmp_51986_101.2[0], poseidon_3_partial_rounds_chain_output_round_8_tmp_51986_101.2[1], poseidon_3_partial_rounds_chain_output_round_8_tmp_51986_101.2[2], poseidon_3_partial_rounds_chain_output_round_8_tmp_51986_101.2[3]]));let poseidon_3_partial_rounds_chain_inputs_6 =
                (((seq) * (M31_1)), M31_10, [poseidon_3_partial_rounds_chain_output_round_9_tmp_51986_102.2[0], poseidon_3_partial_rounds_chain_output_round_9_tmp_51986_102.2[1], poseidon_3_partial_rounds_chain_output_round_9_tmp_51986_102.2[2], poseidon_3_partial_rounds_chain_output_round_9_tmp_51986_102.2[3]]).unpack();
            let poseidon_3_partial_rounds_chain_output_round_10_tmp_51986_103 = poseidon_3_partial_rounds_chain_state.deduce_output((((seq) * (M31_1)), M31_10, [poseidon_3_partial_rounds_chain_output_round_9_tmp_51986_102.2[0], poseidon_3_partial_rounds_chain_output_round_9_tmp_51986_102.2[1], poseidon_3_partial_rounds_chain_output_round_9_tmp_51986_102.2[2], poseidon_3_partial_rounds_chain_output_round_9_tmp_51986_102.2[3]]));let poseidon_3_partial_rounds_chain_inputs_7 =
                (((seq) * (M31_1)), M31_11, [poseidon_3_partial_rounds_chain_output_round_10_tmp_51986_103.2[0], poseidon_3_partial_rounds_chain_output_round_10_tmp_51986_103.2[1], poseidon_3_partial_rounds_chain_output_round_10_tmp_51986_103.2[2], poseidon_3_partial_rounds_chain_output_round_10_tmp_51986_103.2[3]]).unpack();
            let poseidon_3_partial_rounds_chain_output_round_11_tmp_51986_104 = poseidon_3_partial_rounds_chain_state.deduce_output((((seq) * (M31_1)), M31_11, [poseidon_3_partial_rounds_chain_output_round_10_tmp_51986_103.2[0], poseidon_3_partial_rounds_chain_output_round_10_tmp_51986_103.2[1], poseidon_3_partial_rounds_chain_output_round_10_tmp_51986_103.2[2], poseidon_3_partial_rounds_chain_output_round_10_tmp_51986_103.2[3]]));let poseidon_3_partial_rounds_chain_inputs_8 =
                (((seq) * (M31_1)), M31_12, [poseidon_3_partial_rounds_chain_output_round_11_tmp_51986_104.2[0], poseidon_3_partial_rounds_chain_output_round_11_tmp_51986_104.2[1], poseidon_3_partial_rounds_chain_output_round_11_tmp_51986_104.2[2], poseidon_3_partial_rounds_chain_output_round_11_tmp_51986_104.2[3]]).unpack();
            let poseidon_3_partial_rounds_chain_output_round_12_tmp_51986_105 = poseidon_3_partial_rounds_chain_state.deduce_output((((seq) * (M31_1)), M31_12, [poseidon_3_partial_rounds_chain_output_round_11_tmp_51986_104.2[0], poseidon_3_partial_rounds_chain_output_round_11_tmp_51986_104.2[1], poseidon_3_partial_rounds_chain_output_round_11_tmp_51986_104.2[2], poseidon_3_partial_rounds_chain_output_round_11_tmp_51986_104.2[3]]));let poseidon_3_partial_rounds_chain_inputs_9 =
                (((seq) * (M31_1)), M31_13, [poseidon_3_partial_rounds_chain_output_round_12_tmp_51986_105.2[0], poseidon_3_partial_rounds_chain_output_round_12_tmp_51986_105.2[1], poseidon_3_partial_rounds_chain_output_round_12_tmp_51986_105.2[2], poseidon_3_partial_rounds_chain_output_round_12_tmp_51986_105.2[3]]).unpack();
            let poseidon_3_partial_rounds_chain_output_round_13_tmp_51986_106 = poseidon_3_partial_rounds_chain_state.deduce_output((((seq) * (M31_1)), M31_13, [poseidon_3_partial_rounds_chain_output_round_12_tmp_51986_105.2[0], poseidon_3_partial_rounds_chain_output_round_12_tmp_51986_105.2[1], poseidon_3_partial_rounds_chain_output_round_12_tmp_51986_105.2[2], poseidon_3_partial_rounds_chain_output_round_12_tmp_51986_105.2[3]]));let poseidon_3_partial_rounds_chain_inputs_10 =
                (((seq) * (M31_1)), M31_14, [poseidon_3_partial_rounds_chain_output_round_13_tmp_51986_106.2[0], poseidon_3_partial_rounds_chain_output_round_13_tmp_51986_106.2[1], poseidon_3_partial_rounds_chain_output_round_13_tmp_51986_106.2[2], poseidon_3_partial_rounds_chain_output_round_13_tmp_51986_106.2[3]]).unpack();
            let poseidon_3_partial_rounds_chain_output_round_14_tmp_51986_107 = poseidon_3_partial_rounds_chain_state.deduce_output((((seq) * (M31_1)), M31_14, [poseidon_3_partial_rounds_chain_output_round_13_tmp_51986_106.2[0], poseidon_3_partial_rounds_chain_output_round_13_tmp_51986_106.2[1], poseidon_3_partial_rounds_chain_output_round_13_tmp_51986_106.2[2], poseidon_3_partial_rounds_chain_output_round_13_tmp_51986_106.2[3]]));let poseidon_3_partial_rounds_chain_inputs_11 =
                (((seq) * (M31_1)), M31_15, [poseidon_3_partial_rounds_chain_output_round_14_tmp_51986_107.2[0], poseidon_3_partial_rounds_chain_output_round_14_tmp_51986_107.2[1], poseidon_3_partial_rounds_chain_output_round_14_tmp_51986_107.2[2], poseidon_3_partial_rounds_chain_output_round_14_tmp_51986_107.2[3]]).unpack();
            let poseidon_3_partial_rounds_chain_output_round_15_tmp_51986_108 = poseidon_3_partial_rounds_chain_state.deduce_output((((seq) * (M31_1)), M31_15, [poseidon_3_partial_rounds_chain_output_round_14_tmp_51986_107.2[0], poseidon_3_partial_rounds_chain_output_round_14_tmp_51986_107.2[1], poseidon_3_partial_rounds_chain_output_round_14_tmp_51986_107.2[2], poseidon_3_partial_rounds_chain_output_round_14_tmp_51986_107.2[3]]));let poseidon_3_partial_rounds_chain_inputs_12 =
                (((seq) * (M31_1)), M31_16, [poseidon_3_partial_rounds_chain_output_round_15_tmp_51986_108.2[0], poseidon_3_partial_rounds_chain_output_round_15_tmp_51986_108.2[1], poseidon_3_partial_rounds_chain_output_round_15_tmp_51986_108.2[2], poseidon_3_partial_rounds_chain_output_round_15_tmp_51986_108.2[3]]).unpack();
            let poseidon_3_partial_rounds_chain_output_round_16_tmp_51986_109 = poseidon_3_partial_rounds_chain_state.deduce_output((((seq) * (M31_1)), M31_16, [poseidon_3_partial_rounds_chain_output_round_15_tmp_51986_108.2[0], poseidon_3_partial_rounds_chain_output_round_15_tmp_51986_108.2[1], poseidon_3_partial_rounds_chain_output_round_15_tmp_51986_108.2[2], poseidon_3_partial_rounds_chain_output_round_15_tmp_51986_108.2[3]]));let poseidon_3_partial_rounds_chain_inputs_13 =
                (((seq) * (M31_1)), M31_17, [poseidon_3_partial_rounds_chain_output_round_16_tmp_51986_109.2[0], poseidon_3_partial_rounds_chain_output_round_16_tmp_51986_109.2[1], poseidon_3_partial_rounds_chain_output_round_16_tmp_51986_109.2[2], poseidon_3_partial_rounds_chain_output_round_16_tmp_51986_109.2[3]]).unpack();
            let poseidon_3_partial_rounds_chain_output_round_17_tmp_51986_110 = poseidon_3_partial_rounds_chain_state.deduce_output((((seq) * (M31_1)), M31_17, [poseidon_3_partial_rounds_chain_output_round_16_tmp_51986_109.2[0], poseidon_3_partial_rounds_chain_output_round_16_tmp_51986_109.2[1], poseidon_3_partial_rounds_chain_output_round_16_tmp_51986_109.2[2], poseidon_3_partial_rounds_chain_output_round_16_tmp_51986_109.2[3]]));let poseidon_3_partial_rounds_chain_inputs_14 =
                (((seq) * (M31_1)), M31_18, [poseidon_3_partial_rounds_chain_output_round_17_tmp_51986_110.2[0], poseidon_3_partial_rounds_chain_output_round_17_tmp_51986_110.2[1], poseidon_3_partial_rounds_chain_output_round_17_tmp_51986_110.2[2], poseidon_3_partial_rounds_chain_output_round_17_tmp_51986_110.2[3]]).unpack();
            let poseidon_3_partial_rounds_chain_output_round_18_tmp_51986_111 = poseidon_3_partial_rounds_chain_state.deduce_output((((seq) * (M31_1)), M31_18, [poseidon_3_partial_rounds_chain_output_round_17_tmp_51986_110.2[0], poseidon_3_partial_rounds_chain_output_round_17_tmp_51986_110.2[1], poseidon_3_partial_rounds_chain_output_round_17_tmp_51986_110.2[2], poseidon_3_partial_rounds_chain_output_round_17_tmp_51986_110.2[3]]));let poseidon_3_partial_rounds_chain_inputs_15 =
                (((seq) * (M31_1)), M31_19, [poseidon_3_partial_rounds_chain_output_round_18_tmp_51986_111.2[0], poseidon_3_partial_rounds_chain_output_round_18_tmp_51986_111.2[1], poseidon_3_partial_rounds_chain_output_round_18_tmp_51986_111.2[2], poseidon_3_partial_rounds_chain_output_round_18_tmp_51986_111.2[3]]).unpack();
            let poseidon_3_partial_rounds_chain_output_round_19_tmp_51986_112 = poseidon_3_partial_rounds_chain_state.deduce_output((((seq) * (M31_1)), M31_19, [poseidon_3_partial_rounds_chain_output_round_18_tmp_51986_111.2[0], poseidon_3_partial_rounds_chain_output_round_18_tmp_51986_111.2[1], poseidon_3_partial_rounds_chain_output_round_18_tmp_51986_111.2[2], poseidon_3_partial_rounds_chain_output_round_18_tmp_51986_111.2[3]]));let poseidon_3_partial_rounds_chain_inputs_16 =
                (((seq) * (M31_1)), M31_20, [poseidon_3_partial_rounds_chain_output_round_19_tmp_51986_112.2[0], poseidon_3_partial_rounds_chain_output_round_19_tmp_51986_112.2[1], poseidon_3_partial_rounds_chain_output_round_19_tmp_51986_112.2[2], poseidon_3_partial_rounds_chain_output_round_19_tmp_51986_112.2[3]]).unpack();
            let poseidon_3_partial_rounds_chain_output_round_20_tmp_51986_113 = poseidon_3_partial_rounds_chain_state.deduce_output((((seq) * (M31_1)), M31_20, [poseidon_3_partial_rounds_chain_output_round_19_tmp_51986_112.2[0], poseidon_3_partial_rounds_chain_output_round_19_tmp_51986_112.2[1], poseidon_3_partial_rounds_chain_output_round_19_tmp_51986_112.2[2], poseidon_3_partial_rounds_chain_output_round_19_tmp_51986_112.2[3]]));let poseidon_3_partial_rounds_chain_inputs_17 =
                (((seq) * (M31_1)), M31_21, [poseidon_3_partial_rounds_chain_output_round_20_tmp_51986_113.2[0], poseidon_3_partial_rounds_chain_output_round_20_tmp_51986_113.2[1], poseidon_3_partial_rounds_chain_output_round_20_tmp_51986_113.2[2], poseidon_3_partial_rounds_chain_output_round_20_tmp_51986_113.2[3]]).unpack();
            let poseidon_3_partial_rounds_chain_output_round_21_tmp_51986_114 = poseidon_3_partial_rounds_chain_state.deduce_output((((seq) * (M31_1)), M31_21, [poseidon_3_partial_rounds_chain_output_round_20_tmp_51986_113.2[0], poseidon_3_partial_rounds_chain_output_round_20_tmp_51986_113.2[1], poseidon_3_partial_rounds_chain_output_round_20_tmp_51986_113.2[2], poseidon_3_partial_rounds_chain_output_round_20_tmp_51986_113.2[3]]));let poseidon_3_partial_rounds_chain_inputs_18 =
                (((seq) * (M31_1)), M31_22, [poseidon_3_partial_rounds_chain_output_round_21_tmp_51986_114.2[0], poseidon_3_partial_rounds_chain_output_round_21_tmp_51986_114.2[1], poseidon_3_partial_rounds_chain_output_round_21_tmp_51986_114.2[2], poseidon_3_partial_rounds_chain_output_round_21_tmp_51986_114.2[3]]).unpack();
            let poseidon_3_partial_rounds_chain_output_round_22_tmp_51986_115 = poseidon_3_partial_rounds_chain_state.deduce_output((((seq) * (M31_1)), M31_22, [poseidon_3_partial_rounds_chain_output_round_21_tmp_51986_114.2[0], poseidon_3_partial_rounds_chain_output_round_21_tmp_51986_114.2[1], poseidon_3_partial_rounds_chain_output_round_21_tmp_51986_114.2[2], poseidon_3_partial_rounds_chain_output_round_21_tmp_51986_114.2[3]]));let poseidon_3_partial_rounds_chain_inputs_19 =
                (((seq) * (M31_1)), M31_23, [poseidon_3_partial_rounds_chain_output_round_22_tmp_51986_115.2[0], poseidon_3_partial_rounds_chain_output_round_22_tmp_51986_115.2[1], poseidon_3_partial_rounds_chain_output_round_22_tmp_51986_115.2[2], poseidon_3_partial_rounds_chain_output_round_22_tmp_51986_115.2[3]]).unpack();
            let poseidon_3_partial_rounds_chain_output_round_23_tmp_51986_116 = poseidon_3_partial_rounds_chain_state.deduce_output((((seq) * (M31_1)), M31_23, [poseidon_3_partial_rounds_chain_output_round_22_tmp_51986_115.2[0], poseidon_3_partial_rounds_chain_output_round_22_tmp_51986_115.2[1], poseidon_3_partial_rounds_chain_output_round_22_tmp_51986_115.2[2], poseidon_3_partial_rounds_chain_output_round_22_tmp_51986_115.2[3]]));let poseidon_3_partial_rounds_chain_inputs_20 =
                (((seq) * (M31_1)), M31_24, [poseidon_3_partial_rounds_chain_output_round_23_tmp_51986_116.2[0], poseidon_3_partial_rounds_chain_output_round_23_tmp_51986_116.2[1], poseidon_3_partial_rounds_chain_output_round_23_tmp_51986_116.2[2], poseidon_3_partial_rounds_chain_output_round_23_tmp_51986_116.2[3]]).unpack();
            let poseidon_3_partial_rounds_chain_output_round_24_tmp_51986_117 = poseidon_3_partial_rounds_chain_state.deduce_output((((seq) * (M31_1)), M31_24, [poseidon_3_partial_rounds_chain_output_round_23_tmp_51986_116.2[0], poseidon_3_partial_rounds_chain_output_round_23_tmp_51986_116.2[1], poseidon_3_partial_rounds_chain_output_round_23_tmp_51986_116.2[2], poseidon_3_partial_rounds_chain_output_round_23_tmp_51986_116.2[3]]));let poseidon_3_partial_rounds_chain_inputs_21 =
                (((seq) * (M31_1)), M31_25, [poseidon_3_partial_rounds_chain_output_round_24_tmp_51986_117.2[0], poseidon_3_partial_rounds_chain_output_round_24_tmp_51986_117.2[1], poseidon_3_partial_rounds_chain_output_round_24_tmp_51986_117.2[2], poseidon_3_partial_rounds_chain_output_round_24_tmp_51986_117.2[3]]).unpack();
            let poseidon_3_partial_rounds_chain_output_round_25_tmp_51986_118 = poseidon_3_partial_rounds_chain_state.deduce_output((((seq) * (M31_1)), M31_25, [poseidon_3_partial_rounds_chain_output_round_24_tmp_51986_117.2[0], poseidon_3_partial_rounds_chain_output_round_24_tmp_51986_117.2[1], poseidon_3_partial_rounds_chain_output_round_24_tmp_51986_117.2[2], poseidon_3_partial_rounds_chain_output_round_24_tmp_51986_117.2[3]]));let poseidon_3_partial_rounds_chain_inputs_22 =
                (((seq) * (M31_1)), M31_26, [poseidon_3_partial_rounds_chain_output_round_25_tmp_51986_118.2[0], poseidon_3_partial_rounds_chain_output_round_25_tmp_51986_118.2[1], poseidon_3_partial_rounds_chain_output_round_25_tmp_51986_118.2[2], poseidon_3_partial_rounds_chain_output_round_25_tmp_51986_118.2[3]]).unpack();
            let poseidon_3_partial_rounds_chain_output_round_26_tmp_51986_119 = poseidon_3_partial_rounds_chain_state.deduce_output((((seq) * (M31_1)), M31_26, [poseidon_3_partial_rounds_chain_output_round_25_tmp_51986_118.2[0], poseidon_3_partial_rounds_chain_output_round_25_tmp_51986_118.2[1], poseidon_3_partial_rounds_chain_output_round_25_tmp_51986_118.2[2], poseidon_3_partial_rounds_chain_output_round_25_tmp_51986_118.2[3]]));let poseidon_3_partial_rounds_chain_inputs_23 =
                (((seq) * (M31_1)), M31_27, [poseidon_3_partial_rounds_chain_output_round_26_tmp_51986_119.2[0], poseidon_3_partial_rounds_chain_output_round_26_tmp_51986_119.2[1], poseidon_3_partial_rounds_chain_output_round_26_tmp_51986_119.2[2], poseidon_3_partial_rounds_chain_output_round_26_tmp_51986_119.2[3]]).unpack();
            let poseidon_3_partial_rounds_chain_output_round_27_tmp_51986_120 = poseidon_3_partial_rounds_chain_state.deduce_output((((seq) * (M31_1)), M31_27, [poseidon_3_partial_rounds_chain_output_round_26_tmp_51986_119.2[0], poseidon_3_partial_rounds_chain_output_round_26_tmp_51986_119.2[1], poseidon_3_partial_rounds_chain_output_round_26_tmp_51986_119.2[2], poseidon_3_partial_rounds_chain_output_round_26_tmp_51986_119.2[3]]));let poseidon_3_partial_rounds_chain_inputs_24 =
                (((seq) * (M31_1)), M31_28, [poseidon_3_partial_rounds_chain_output_round_27_tmp_51986_120.2[0], poseidon_3_partial_rounds_chain_output_round_27_tmp_51986_120.2[1], poseidon_3_partial_rounds_chain_output_round_27_tmp_51986_120.2[2], poseidon_3_partial_rounds_chain_output_round_27_tmp_51986_120.2[3]]).unpack();
            let poseidon_3_partial_rounds_chain_output_round_28_tmp_51986_121 = poseidon_3_partial_rounds_chain_state.deduce_output((((seq) * (M31_1)), M31_28, [poseidon_3_partial_rounds_chain_output_round_27_tmp_51986_120.2[0], poseidon_3_partial_rounds_chain_output_round_27_tmp_51986_120.2[1], poseidon_3_partial_rounds_chain_output_round_27_tmp_51986_120.2[2], poseidon_3_partial_rounds_chain_output_round_27_tmp_51986_120.2[3]]));let poseidon_3_partial_rounds_chain_inputs_25 =
                (((seq) * (M31_1)), M31_29, [poseidon_3_partial_rounds_chain_output_round_28_tmp_51986_121.2[0], poseidon_3_partial_rounds_chain_output_round_28_tmp_51986_121.2[1], poseidon_3_partial_rounds_chain_output_round_28_tmp_51986_121.2[2], poseidon_3_partial_rounds_chain_output_round_28_tmp_51986_121.2[3]]).unpack();
            let poseidon_3_partial_rounds_chain_output_round_29_tmp_51986_122 = poseidon_3_partial_rounds_chain_state.deduce_output((((seq) * (M31_1)), M31_29, [poseidon_3_partial_rounds_chain_output_round_28_tmp_51986_121.2[0], poseidon_3_partial_rounds_chain_output_round_28_tmp_51986_121.2[1], poseidon_3_partial_rounds_chain_output_round_28_tmp_51986_121.2[2], poseidon_3_partial_rounds_chain_output_round_28_tmp_51986_121.2[3]]));let poseidon_3_partial_rounds_chain_inputs_26 =
                (((seq) * (M31_1)), M31_30, [poseidon_3_partial_rounds_chain_output_round_29_tmp_51986_122.2[0], poseidon_3_partial_rounds_chain_output_round_29_tmp_51986_122.2[1], poseidon_3_partial_rounds_chain_output_round_29_tmp_51986_122.2[2], poseidon_3_partial_rounds_chain_output_round_29_tmp_51986_122.2[3]]).unpack();
            let poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123 = poseidon_3_partial_rounds_chain_state.deduce_output((((seq) * (M31_1)), M31_30, [poseidon_3_partial_rounds_chain_output_round_29_tmp_51986_122.2[0], poseidon_3_partial_rounds_chain_output_round_29_tmp_51986_122.2[1], poseidon_3_partial_rounds_chain_output_round_29_tmp_51986_122.2[2], poseidon_3_partial_rounds_chain_output_round_29_tmp_51986_122.2[3]]));let poseidon_3_partial_rounds_chain_output_limb_0_col194 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.0;
            *row[194] = poseidon_3_partial_rounds_chain_output_limb_0_col194;let poseidon_3_partial_rounds_chain_output_limb_1_col195 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.1;
            *row[195] = poseidon_3_partial_rounds_chain_output_limb_1_col195;let poseidon_3_partial_rounds_chain_output_limb_2_col196 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[0].get_m31(0);
            *row[196] = poseidon_3_partial_rounds_chain_output_limb_2_col196;let poseidon_3_partial_rounds_chain_output_limb_3_col197 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[0].get_m31(1);
            *row[197] = poseidon_3_partial_rounds_chain_output_limb_3_col197;let poseidon_3_partial_rounds_chain_output_limb_4_col198 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[0].get_m31(2);
            *row[198] = poseidon_3_partial_rounds_chain_output_limb_4_col198;let poseidon_3_partial_rounds_chain_output_limb_5_col199 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[0].get_m31(3);
            *row[199] = poseidon_3_partial_rounds_chain_output_limb_5_col199;let poseidon_3_partial_rounds_chain_output_limb_6_col200 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[0].get_m31(4);
            *row[200] = poseidon_3_partial_rounds_chain_output_limb_6_col200;let poseidon_3_partial_rounds_chain_output_limb_7_col201 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[0].get_m31(5);
            *row[201] = poseidon_3_partial_rounds_chain_output_limb_7_col201;let poseidon_3_partial_rounds_chain_output_limb_8_col202 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[0].get_m31(6);
            *row[202] = poseidon_3_partial_rounds_chain_output_limb_8_col202;let poseidon_3_partial_rounds_chain_output_limb_9_col203 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[0].get_m31(7);
            *row[203] = poseidon_3_partial_rounds_chain_output_limb_9_col203;let poseidon_3_partial_rounds_chain_output_limb_10_col204 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[0].get_m31(8);
            *row[204] = poseidon_3_partial_rounds_chain_output_limb_10_col204;let poseidon_3_partial_rounds_chain_output_limb_11_col205 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[0].get_m31(9);
            *row[205] = poseidon_3_partial_rounds_chain_output_limb_11_col205;let poseidon_3_partial_rounds_chain_output_limb_12_col206 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[1].get_m31(0);
            *row[206] = poseidon_3_partial_rounds_chain_output_limb_12_col206;let poseidon_3_partial_rounds_chain_output_limb_13_col207 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[1].get_m31(1);
            *row[207] = poseidon_3_partial_rounds_chain_output_limb_13_col207;let poseidon_3_partial_rounds_chain_output_limb_14_col208 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[1].get_m31(2);
            *row[208] = poseidon_3_partial_rounds_chain_output_limb_14_col208;let poseidon_3_partial_rounds_chain_output_limb_15_col209 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[1].get_m31(3);
            *row[209] = poseidon_3_partial_rounds_chain_output_limb_15_col209;let poseidon_3_partial_rounds_chain_output_limb_16_col210 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[1].get_m31(4);
            *row[210] = poseidon_3_partial_rounds_chain_output_limb_16_col210;let poseidon_3_partial_rounds_chain_output_limb_17_col211 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[1].get_m31(5);
            *row[211] = poseidon_3_partial_rounds_chain_output_limb_17_col211;let poseidon_3_partial_rounds_chain_output_limb_18_col212 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[1].get_m31(6);
            *row[212] = poseidon_3_partial_rounds_chain_output_limb_18_col212;let poseidon_3_partial_rounds_chain_output_limb_19_col213 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[1].get_m31(7);
            *row[213] = poseidon_3_partial_rounds_chain_output_limb_19_col213;let poseidon_3_partial_rounds_chain_output_limb_20_col214 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[1].get_m31(8);
            *row[214] = poseidon_3_partial_rounds_chain_output_limb_20_col214;let poseidon_3_partial_rounds_chain_output_limb_21_col215 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[1].get_m31(9);
            *row[215] = poseidon_3_partial_rounds_chain_output_limb_21_col215;let poseidon_3_partial_rounds_chain_output_limb_22_col216 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[2].get_m31(0);
            *row[216] = poseidon_3_partial_rounds_chain_output_limb_22_col216;let poseidon_3_partial_rounds_chain_output_limb_23_col217 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[2].get_m31(1);
            *row[217] = poseidon_3_partial_rounds_chain_output_limb_23_col217;let poseidon_3_partial_rounds_chain_output_limb_24_col218 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[2].get_m31(2);
            *row[218] = poseidon_3_partial_rounds_chain_output_limb_24_col218;let poseidon_3_partial_rounds_chain_output_limb_25_col219 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[2].get_m31(3);
            *row[219] = poseidon_3_partial_rounds_chain_output_limb_25_col219;let poseidon_3_partial_rounds_chain_output_limb_26_col220 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[2].get_m31(4);
            *row[220] = poseidon_3_partial_rounds_chain_output_limb_26_col220;let poseidon_3_partial_rounds_chain_output_limb_27_col221 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[2].get_m31(5);
            *row[221] = poseidon_3_partial_rounds_chain_output_limb_27_col221;let poseidon_3_partial_rounds_chain_output_limb_28_col222 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[2].get_m31(6);
            *row[222] = poseidon_3_partial_rounds_chain_output_limb_28_col222;let poseidon_3_partial_rounds_chain_output_limb_29_col223 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[2].get_m31(7);
            *row[223] = poseidon_3_partial_rounds_chain_output_limb_29_col223;let poseidon_3_partial_rounds_chain_output_limb_30_col224 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[2].get_m31(8);
            *row[224] = poseidon_3_partial_rounds_chain_output_limb_30_col224;let poseidon_3_partial_rounds_chain_output_limb_31_col225 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[2].get_m31(9);
            *row[225] = poseidon_3_partial_rounds_chain_output_limb_31_col225;let poseidon_3_partial_rounds_chain_output_limb_32_col226 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[3].get_m31(0);
            *row[226] = poseidon_3_partial_rounds_chain_output_limb_32_col226;let poseidon_3_partial_rounds_chain_output_limb_33_col227 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[3].get_m31(1);
            *row[227] = poseidon_3_partial_rounds_chain_output_limb_33_col227;let poseidon_3_partial_rounds_chain_output_limb_34_col228 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[3].get_m31(2);
            *row[228] = poseidon_3_partial_rounds_chain_output_limb_34_col228;let poseidon_3_partial_rounds_chain_output_limb_35_col229 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[3].get_m31(3);
            *row[229] = poseidon_3_partial_rounds_chain_output_limb_35_col229;let poseidon_3_partial_rounds_chain_output_limb_36_col230 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[3].get_m31(4);
            *row[230] = poseidon_3_partial_rounds_chain_output_limb_36_col230;let poseidon_3_partial_rounds_chain_output_limb_37_col231 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[3].get_m31(5);
            *row[231] = poseidon_3_partial_rounds_chain_output_limb_37_col231;let poseidon_3_partial_rounds_chain_output_limb_38_col232 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[3].get_m31(6);
            *row[232] = poseidon_3_partial_rounds_chain_output_limb_38_col232;let poseidon_3_partial_rounds_chain_output_limb_39_col233 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[3].get_m31(7);
            *row[233] = poseidon_3_partial_rounds_chain_output_limb_39_col233;let poseidon_3_partial_rounds_chain_output_limb_40_col234 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[3].get_m31(8);
            *row[234] = poseidon_3_partial_rounds_chain_output_limb_40_col234;let poseidon_3_partial_rounds_chain_output_limb_41_col235 = poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[3].get_m31(9);
            *row[235] = poseidon_3_partial_rounds_chain_output_limb_41_col235;*lookup_data.poseidon_3_partial_rounds_chain_1 = [poseidon_3_partial_rounds_chain_output_limb_0_col194, poseidon_3_partial_rounds_chain_output_limb_1_col195, poseidon_3_partial_rounds_chain_output_limb_2_col196, poseidon_3_partial_rounds_chain_output_limb_3_col197, poseidon_3_partial_rounds_chain_output_limb_4_col198, poseidon_3_partial_rounds_chain_output_limb_5_col199, poseidon_3_partial_rounds_chain_output_limb_6_col200, poseidon_3_partial_rounds_chain_output_limb_7_col201, poseidon_3_partial_rounds_chain_output_limb_8_col202, poseidon_3_partial_rounds_chain_output_limb_9_col203, poseidon_3_partial_rounds_chain_output_limb_10_col204, poseidon_3_partial_rounds_chain_output_limb_11_col205, poseidon_3_partial_rounds_chain_output_limb_12_col206, poseidon_3_partial_rounds_chain_output_limb_13_col207, poseidon_3_partial_rounds_chain_output_limb_14_col208, poseidon_3_partial_rounds_chain_output_limb_15_col209, poseidon_3_partial_rounds_chain_output_limb_16_col210, poseidon_3_partial_rounds_chain_output_limb_17_col211, poseidon_3_partial_rounds_chain_output_limb_18_col212, poseidon_3_partial_rounds_chain_output_limb_19_col213, poseidon_3_partial_rounds_chain_output_limb_20_col214, poseidon_3_partial_rounds_chain_output_limb_21_col215, poseidon_3_partial_rounds_chain_output_limb_22_col216, poseidon_3_partial_rounds_chain_output_limb_23_col217, poseidon_3_partial_rounds_chain_output_limb_24_col218, poseidon_3_partial_rounds_chain_output_limb_25_col219, poseidon_3_partial_rounds_chain_output_limb_26_col220, poseidon_3_partial_rounds_chain_output_limb_27_col221, poseidon_3_partial_rounds_chain_output_limb_28_col222, poseidon_3_partial_rounds_chain_output_limb_29_col223, poseidon_3_partial_rounds_chain_output_limb_30_col224, poseidon_3_partial_rounds_chain_output_limb_31_col225, poseidon_3_partial_rounds_chain_output_limb_32_col226, poseidon_3_partial_rounds_chain_output_limb_33_col227, poseidon_3_partial_rounds_chain_output_limb_34_col228, poseidon_3_partial_rounds_chain_output_limb_35_col229, poseidon_3_partial_rounds_chain_output_limb_36_col230, poseidon_3_partial_rounds_chain_output_limb_37_col231, poseidon_3_partial_rounds_chain_output_limb_38_col232, poseidon_3_partial_rounds_chain_output_limb_39_col233, poseidon_3_partial_rounds_chain_output_limb_40_col234, poseidon_3_partial_rounds_chain_output_limb_41_col235];

            //Linear Combination N 4 Coefs 4 2 1 1.

            let combination_result_tmp_51986_124 = PackedFelt252Width27::from_packed_felt252(((((((((Felt252_0_0_0_0) + (((Felt252_4_0_0_0) * (PackedFelt252::from_packed_felt252width27(poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[0])))))) + (((Felt252_2_0_0_0) * (PackedFelt252::from_packed_felt252width27(poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[1])))))) + (((Felt252_1_0_0_0) * (PackedFelt252::from_packed_felt252width27(poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[2])))))) + (Felt252_3969818800901670911_10562874008078701503_14906396266795319764_223312371439046257)));let combination_limb_0_col236 = combination_result_tmp_51986_124.get_m31(0);
            *row[236] = combination_limb_0_col236;let combination_limb_1_col237 = combination_result_tmp_51986_124.get_m31(1);
            *row[237] = combination_limb_1_col237;let combination_limb_2_col238 = combination_result_tmp_51986_124.get_m31(2);
            *row[238] = combination_limb_2_col238;let combination_limb_3_col239 = combination_result_tmp_51986_124.get_m31(3);
            *row[239] = combination_limb_3_col239;let combination_limb_4_col240 = combination_result_tmp_51986_124.get_m31(4);
            *row[240] = combination_limb_4_col240;let combination_limb_5_col241 = combination_result_tmp_51986_124.get_m31(5);
            *row[241] = combination_limb_5_col241;let combination_limb_6_col242 = combination_result_tmp_51986_124.get_m31(6);
            *row[242] = combination_limb_6_col242;let combination_limb_7_col243 = combination_result_tmp_51986_124.get_m31(7);
            *row[243] = combination_limb_7_col243;let combination_limb_8_col244 = combination_result_tmp_51986_124.get_m31(8);
            *row[244] = combination_limb_8_col244;let combination_limb_9_col245 = combination_result_tmp_51986_124.get_m31(9);
            *row[245] = combination_limb_9_col245;let biased_limb_accumulator_u32_tmp_51986_125 = PackedUInt32::from_m31(((((((((((((M31_0) - (combination_limb_0_col236))) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_2_col196))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_12_col206))))) + (((M31_1) * (poseidon_3_partial_rounds_chain_output_limb_22_col216))))) + (M31_40454143))) + (M31_134217729)));let p_coef_col246 = ((biased_limb_accumulator_u32_tmp_51986_125.low().as_m31()) - (M31_1));
            *row[246] = p_coef_col246;let carry_0_tmp_51986_126 = ((((((((((((((M31_0) - (combination_limb_0_col236))) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_2_col196))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_12_col206))))) + (((M31_1) * (poseidon_3_partial_rounds_chain_output_limb_22_col216))))) + (M31_40454143))) - (((p_coef_col246) * (M31_1))))) * (M31_16));let carry_1_tmp_51986_127 = ((((((((((((((carry_0_tmp_51986_126) - (combination_limb_1_col237))) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_3_col197))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_13_col207))))) + (((M31_1) * (poseidon_3_partial_rounds_chain_output_limb_23_col217))))) + (M31_49554771))) - (((p_coef_col246) * (M31_0))))) * (M31_16));let carry_2_tmp_51986_128 = ((((((((((((((carry_1_tmp_51986_127) - (combination_limb_2_col238))) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_4_col198))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_14_col208))))) + (((M31_1) * (poseidon_3_partial_rounds_chain_output_limb_24_col218))))) + (M31_55508188))) - (((p_coef_col246) * (M31_0))))) * (M31_16));let carry_3_tmp_51986_129 = ((((((((((((((carry_2_tmp_51986_128) - (combination_limb_3_col239))) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_5_col199))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_15_col209))))) + (((M31_1) * (poseidon_3_partial_rounds_chain_output_limb_25_col219))))) + (M31_116986206))) - (((p_coef_col246) * (M31_0))))) * (M31_16));let carry_4_tmp_51986_130 = ((((((((((((((carry_3_tmp_51986_129) - (combination_limb_4_col240))) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_6_col200))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_16_col210))))) + (((M31_1) * (poseidon_3_partial_rounds_chain_output_limb_26_col220))))) + (M31_88680813))) - (((p_coef_col246) * (M31_0))))) * (M31_16));let carry_5_tmp_51986_131 = ((((((((((((((carry_4_tmp_51986_130) - (combination_limb_5_col241))) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_7_col201))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_17_col211))))) + (((M31_1) * (poseidon_3_partial_rounds_chain_output_limb_27_col221))))) + (M31_45553283))) - (((p_coef_col246) * (M31_0))))) * (M31_16));let carry_6_tmp_51986_132 = ((((((((((((((carry_5_tmp_51986_131) - (combination_limb_6_col242))) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_8_col202))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_18_col212))))) + (((M31_1) * (poseidon_3_partial_rounds_chain_output_limb_28_col222))))) + (M31_62360091))) - (((p_coef_col246) * (M31_0))))) * (M31_16));let carry_7_tmp_51986_133 = ((((((((((((((carry_6_tmp_51986_132) - (combination_limb_7_col243))) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_9_col203))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_19_col213))))) + (((M31_1) * (poseidon_3_partial_rounds_chain_output_limb_29_col223))))) + (M31_77099918))) - (((p_coef_col246) * (M31_136))))) * (M31_16));let carry_8_tmp_51986_134 = ((((((((((((((carry_7_tmp_51986_133) - (combination_limb_8_col244))) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_10_col204))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_20_col214))))) + (((M31_1) * (poseidon_3_partial_rounds_chain_output_limb_30_col224))))) + (M31_22899501))) - (((p_coef_col246) * (M31_0))))) * (M31_16));let range_check_4_4_4_4_inputs_2 =
                [((p_coef_col246) + (M31_1)), ((carry_0_tmp_51986_126) + (M31_1)), ((carry_1_tmp_51986_127) + (M31_1)), ((carry_2_tmp_51986_128) + (M31_1))].unpack();
            *lookup_data.range_check_4_4_4_4_2 = [((p_coef_col246) + (M31_1)), ((carry_0_tmp_51986_126) + (M31_1)), ((carry_1_tmp_51986_127) + (M31_1)), ((carry_2_tmp_51986_128) + (M31_1))];let range_check_4_4_4_4_inputs_3 =
                [((carry_3_tmp_51986_129) + (M31_1)), ((carry_4_tmp_51986_130) + (M31_1)), ((carry_5_tmp_51986_131) + (M31_1)), ((carry_6_tmp_51986_132) + (M31_1))].unpack();
            *lookup_data.range_check_4_4_4_4_3 = [((carry_3_tmp_51986_129) + (M31_1)), ((carry_4_tmp_51986_130) + (M31_1)), ((carry_5_tmp_51986_131) + (M31_1)), ((carry_6_tmp_51986_132) + (M31_1))];let range_check_4_4_inputs_1 =
                [((carry_7_tmp_51986_133) + (M31_1)), ((carry_8_tmp_51986_134) + (M31_1))].unpack();
            *lookup_data.range_check_4_4_1 = [((carry_7_tmp_51986_133) + (M31_1)), ((carry_8_tmp_51986_134) + (M31_1))];

            //Linear Combination N 4 Coefs 4 2 1 1.

            let combination_result_tmp_51986_135 = PackedFelt252Width27::from_packed_felt252(((((((((Felt252_0_0_0_0) + (((Felt252_4_0_0_0) * (PackedFelt252::from_packed_felt252width27(poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[2])))))) + (((Felt252_2_0_0_0) * (PackedFelt252::from_packed_felt252width27(poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[3])))))) + (((Felt252_1_0_0_0) * (PackedFelt252::from_packed_felt252width27(combination_result_tmp_51986_124)))))) + (Felt252_10310704347937391837_5874215448258336115_2880320859071049537_45350836576946303)));let combination_limb_0_col247 = combination_result_tmp_51986_135.get_m31(0);
            *row[247] = combination_limb_0_col247;let combination_limb_1_col248 = combination_result_tmp_51986_135.get_m31(1);
            *row[248] = combination_limb_1_col248;let combination_limb_2_col249 = combination_result_tmp_51986_135.get_m31(2);
            *row[249] = combination_limb_2_col249;let combination_limb_3_col250 = combination_result_tmp_51986_135.get_m31(3);
            *row[250] = combination_limb_3_col250;let combination_limb_4_col251 = combination_result_tmp_51986_135.get_m31(4);
            *row[251] = combination_limb_4_col251;let combination_limb_5_col252 = combination_result_tmp_51986_135.get_m31(5);
            *row[252] = combination_limb_5_col252;let combination_limb_6_col253 = combination_result_tmp_51986_135.get_m31(6);
            *row[253] = combination_limb_6_col253;let combination_limb_7_col254 = combination_result_tmp_51986_135.get_m31(7);
            *row[254] = combination_limb_7_col254;let combination_limb_8_col255 = combination_result_tmp_51986_135.get_m31(8);
            *row[255] = combination_limb_8_col255;let combination_limb_9_col256 = combination_result_tmp_51986_135.get_m31(9);
            *row[256] = combination_limb_9_col256;let biased_limb_accumulator_u32_tmp_51986_136 = PackedUInt32::from_m31(((((((((((((M31_0) - (combination_limb_0_col247))) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_22_col216))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_32_col226))))) + (((M31_1) * (combination_limb_0_col236))))) + (M31_48383197))) + (M31_134217729)));let p_coef_col257 = ((biased_limb_accumulator_u32_tmp_51986_136.low().as_m31()) - (M31_1));
            *row[257] = p_coef_col257;let carry_0_tmp_51986_137 = ((((((((((((((M31_0) - (combination_limb_0_col247))) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_22_col216))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_32_col226))))) + (((M31_1) * (combination_limb_0_col236))))) + (M31_48383197))) - (((p_coef_col257) * (M31_1))))) * (M31_16));let carry_1_tmp_51986_138 = ((((((((((((((carry_0_tmp_51986_137) - (combination_limb_1_col248))) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_23_col217))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_33_col227))))) + (((M31_1) * (combination_limb_1_col237))))) + (M31_48193339))) - (((p_coef_col257) * (M31_0))))) * (M31_16));let carry_2_tmp_51986_139 = ((((((((((((((carry_1_tmp_51986_138) - (combination_limb_2_col249))) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_24_col218))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_34_col228))))) + (((M31_1) * (combination_limb_2_col238))))) + (M31_55955004))) - (((p_coef_col257) * (M31_0))))) * (M31_16));let carry_3_tmp_51986_140 = ((((((((((((((carry_2_tmp_51986_139) - (combination_limb_3_col250))) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_25_col219))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_35_col229))))) + (((M31_1) * (combination_limb_3_col239))))) + (M31_65659846))) - (((p_coef_col257) * (M31_0))))) * (M31_16));let carry_4_tmp_51986_141 = ((((((((((((((carry_3_tmp_51986_140) - (combination_limb_4_col251))) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_26_col220))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_36_col230))))) + (((M31_1) * (combination_limb_4_col240))))) + (M31_68491350))) - (((p_coef_col257) * (M31_0))))) * (M31_16));let carry_5_tmp_51986_142 = ((((((((((((((carry_4_tmp_51986_141) - (combination_limb_5_col252))) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_27_col221))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_37_col231))))) + (((M31_1) * (combination_limb_5_col241))))) + (M31_119023582))) - (((p_coef_col257) * (M31_0))))) * (M31_16));let carry_6_tmp_51986_143 = ((((((((((((((carry_5_tmp_51986_142) - (combination_limb_6_col253))) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_28_col222))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_38_col232))))) + (((M31_1) * (combination_limb_6_col242))))) + (M31_33439011))) - (((p_coef_col257) * (M31_0))))) * (M31_16));let carry_7_tmp_51986_144 = ((((((((((((((carry_6_tmp_51986_143) - (combination_limb_7_col254))) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_29_col223))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_39_col233))))) + (((M31_1) * (combination_limb_7_col243))))) + (M31_58475513))) - (((p_coef_col257) * (M31_136))))) * (M31_16));let carry_8_tmp_51986_145 = ((((((((((((((carry_7_tmp_51986_144) - (combination_limb_8_col255))) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_30_col224))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_40_col234))))) + (((M31_1) * (combination_limb_8_col244))))) + (M31_18765944))) - (((p_coef_col257) * (M31_0))))) * (M31_16));let range_check_4_4_4_4_inputs_4 =
                [((p_coef_col257) + (M31_1)), ((carry_0_tmp_51986_137) + (M31_1)), ((carry_1_tmp_51986_138) + (M31_1)), ((carry_2_tmp_51986_139) + (M31_1))].unpack();
            *lookup_data.range_check_4_4_4_4_4 = [((p_coef_col257) + (M31_1)), ((carry_0_tmp_51986_137) + (M31_1)), ((carry_1_tmp_51986_138) + (M31_1)), ((carry_2_tmp_51986_139) + (M31_1))];let range_check_4_4_4_4_inputs_5 =
                [((carry_3_tmp_51986_140) + (M31_1)), ((carry_4_tmp_51986_141) + (M31_1)), ((carry_5_tmp_51986_142) + (M31_1)), ((carry_6_tmp_51986_143) + (M31_1))].unpack();
            *lookup_data.range_check_4_4_4_4_5 = [((carry_3_tmp_51986_140) + (M31_1)), ((carry_4_tmp_51986_141) + (M31_1)), ((carry_5_tmp_51986_142) + (M31_1)), ((carry_6_tmp_51986_143) + (M31_1))];let range_check_4_4_inputs_2 =
                [((carry_7_tmp_51986_144) + (M31_1)), ((carry_8_tmp_51986_145) + (M31_1))].unpack();
            *lookup_data.range_check_4_4_2 = [((carry_7_tmp_51986_144) + (M31_1)), ((carry_8_tmp_51986_145) + (M31_1))];

            *lookup_data.poseidon_full_round_chain_2 = [((((seq) * (M31_2))) + (M31_1)), M31_31, combination_limb_0_col247, combination_limb_1_col248, combination_limb_2_col249, combination_limb_3_col250, combination_limb_4_col251, combination_limb_5_col252, combination_limb_6_col253, combination_limb_7_col254, combination_limb_8_col255, combination_limb_9_col256, combination_limb_0_col236, combination_limb_1_col237, combination_limb_2_col238, combination_limb_3_col239, combination_limb_4_col240, combination_limb_5_col241, combination_limb_6_col242, combination_limb_7_col243, combination_limb_8_col244, combination_limb_9_col245, poseidon_3_partial_rounds_chain_output_limb_32_col226, poseidon_3_partial_rounds_chain_output_limb_33_col227, poseidon_3_partial_rounds_chain_output_limb_34_col228, poseidon_3_partial_rounds_chain_output_limb_35_col229, poseidon_3_partial_rounds_chain_output_limb_36_col230, poseidon_3_partial_rounds_chain_output_limb_37_col231, poseidon_3_partial_rounds_chain_output_limb_38_col232, poseidon_3_partial_rounds_chain_output_limb_39_col233, poseidon_3_partial_rounds_chain_output_limb_40_col234, poseidon_3_partial_rounds_chain_output_limb_41_col235];let poseidon_full_round_chain_inputs_4 =
                (((((seq) * (M31_2))) + (M31_1)), M31_31, [combination_result_tmp_51986_135, combination_result_tmp_51986_124, poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[3]]).unpack();
            let poseidon_full_round_chain_output_round_31_tmp_51986_146 = poseidon_full_round_chain_state.deduce_output((((((seq) * (M31_2))) + (M31_1)), M31_31, [combination_result_tmp_51986_135, combination_result_tmp_51986_124, poseidon_3_partial_rounds_chain_output_round_30_tmp_51986_123.2[3]]));let poseidon_full_round_chain_inputs_5 =
                (((((seq) * (M31_2))) + (M31_1)), M31_32, [poseidon_full_round_chain_output_round_31_tmp_51986_146.2[0], poseidon_full_round_chain_output_round_31_tmp_51986_146.2[1], poseidon_full_round_chain_output_round_31_tmp_51986_146.2[2]]).unpack();
            let poseidon_full_round_chain_output_round_32_tmp_51986_147 = poseidon_full_round_chain_state.deduce_output((((((seq) * (M31_2))) + (M31_1)), M31_32, [poseidon_full_round_chain_output_round_31_tmp_51986_146.2[0], poseidon_full_round_chain_output_round_31_tmp_51986_146.2[1], poseidon_full_round_chain_output_round_31_tmp_51986_146.2[2]]));let poseidon_full_round_chain_inputs_6 =
                (((((seq) * (M31_2))) + (M31_1)), M31_33, [poseidon_full_round_chain_output_round_32_tmp_51986_147.2[0], poseidon_full_round_chain_output_round_32_tmp_51986_147.2[1], poseidon_full_round_chain_output_round_32_tmp_51986_147.2[2]]).unpack();
            let poseidon_full_round_chain_output_round_33_tmp_51986_148 = poseidon_full_round_chain_state.deduce_output((((((seq) * (M31_2))) + (M31_1)), M31_33, [poseidon_full_round_chain_output_round_32_tmp_51986_147.2[0], poseidon_full_round_chain_output_round_32_tmp_51986_147.2[1], poseidon_full_round_chain_output_round_32_tmp_51986_147.2[2]]));let poseidon_full_round_chain_inputs_7 =
                (((((seq) * (M31_2))) + (M31_1)), M31_34, [poseidon_full_round_chain_output_round_33_tmp_51986_148.2[0], poseidon_full_round_chain_output_round_33_tmp_51986_148.2[1], poseidon_full_round_chain_output_round_33_tmp_51986_148.2[2]]).unpack();
            let poseidon_full_round_chain_output_round_34_tmp_51986_149 = poseidon_full_round_chain_state.deduce_output((((((seq) * (M31_2))) + (M31_1)), M31_34, [poseidon_full_round_chain_output_round_33_tmp_51986_148.2[0], poseidon_full_round_chain_output_round_33_tmp_51986_148.2[1], poseidon_full_round_chain_output_round_33_tmp_51986_148.2[2]]));let poseidon_full_round_chain_output_limb_0_col258 = poseidon_full_round_chain_output_round_34_tmp_51986_149.0;
            *row[258] = poseidon_full_round_chain_output_limb_0_col258;let poseidon_full_round_chain_output_limb_1_col259 = poseidon_full_round_chain_output_round_34_tmp_51986_149.1;
            *row[259] = poseidon_full_round_chain_output_limb_1_col259;let poseidon_full_round_chain_output_limb_2_col260 = poseidon_full_round_chain_output_round_34_tmp_51986_149.2[0].get_m31(0);
            *row[260] = poseidon_full_round_chain_output_limb_2_col260;let poseidon_full_round_chain_output_limb_3_col261 = poseidon_full_round_chain_output_round_34_tmp_51986_149.2[0].get_m31(1);
            *row[261] = poseidon_full_round_chain_output_limb_3_col261;let poseidon_full_round_chain_output_limb_4_col262 = poseidon_full_round_chain_output_round_34_tmp_51986_149.2[0].get_m31(2);
            *row[262] = poseidon_full_round_chain_output_limb_4_col262;let poseidon_full_round_chain_output_limb_5_col263 = poseidon_full_round_chain_output_round_34_tmp_51986_149.2[0].get_m31(3);
            *row[263] = poseidon_full_round_chain_output_limb_5_col263;let poseidon_full_round_chain_output_limb_6_col264 = poseidon_full_round_chain_output_round_34_tmp_51986_149.2[0].get_m31(4);
            *row[264] = poseidon_full_round_chain_output_limb_6_col264;let poseidon_full_round_chain_output_limb_7_col265 = poseidon_full_round_chain_output_round_34_tmp_51986_149.2[0].get_m31(5);
            *row[265] = poseidon_full_round_chain_output_limb_7_col265;let poseidon_full_round_chain_output_limb_8_col266 = poseidon_full_round_chain_output_round_34_tmp_51986_149.2[0].get_m31(6);
            *row[266] = poseidon_full_round_chain_output_limb_8_col266;let poseidon_full_round_chain_output_limb_9_col267 = poseidon_full_round_chain_output_round_34_tmp_51986_149.2[0].get_m31(7);
            *row[267] = poseidon_full_round_chain_output_limb_9_col267;let poseidon_full_round_chain_output_limb_10_col268 = poseidon_full_round_chain_output_round_34_tmp_51986_149.2[0].get_m31(8);
            *row[268] = poseidon_full_round_chain_output_limb_10_col268;let poseidon_full_round_chain_output_limb_11_col269 = poseidon_full_round_chain_output_round_34_tmp_51986_149.2[0].get_m31(9);
            *row[269] = poseidon_full_round_chain_output_limb_11_col269;let poseidon_full_round_chain_output_limb_12_col270 = poseidon_full_round_chain_output_round_34_tmp_51986_149.2[1].get_m31(0);
            *row[270] = poseidon_full_round_chain_output_limb_12_col270;let poseidon_full_round_chain_output_limb_13_col271 = poseidon_full_round_chain_output_round_34_tmp_51986_149.2[1].get_m31(1);
            *row[271] = poseidon_full_round_chain_output_limb_13_col271;let poseidon_full_round_chain_output_limb_14_col272 = poseidon_full_round_chain_output_round_34_tmp_51986_149.2[1].get_m31(2);
            *row[272] = poseidon_full_round_chain_output_limb_14_col272;let poseidon_full_round_chain_output_limb_15_col273 = poseidon_full_round_chain_output_round_34_tmp_51986_149.2[1].get_m31(3);
            *row[273] = poseidon_full_round_chain_output_limb_15_col273;let poseidon_full_round_chain_output_limb_16_col274 = poseidon_full_round_chain_output_round_34_tmp_51986_149.2[1].get_m31(4);
            *row[274] = poseidon_full_round_chain_output_limb_16_col274;let poseidon_full_round_chain_output_limb_17_col275 = poseidon_full_round_chain_output_round_34_tmp_51986_149.2[1].get_m31(5);
            *row[275] = poseidon_full_round_chain_output_limb_17_col275;let poseidon_full_round_chain_output_limb_18_col276 = poseidon_full_round_chain_output_round_34_tmp_51986_149.2[1].get_m31(6);
            *row[276] = poseidon_full_round_chain_output_limb_18_col276;let poseidon_full_round_chain_output_limb_19_col277 = poseidon_full_round_chain_output_round_34_tmp_51986_149.2[1].get_m31(7);
            *row[277] = poseidon_full_round_chain_output_limb_19_col277;let poseidon_full_round_chain_output_limb_20_col278 = poseidon_full_round_chain_output_round_34_tmp_51986_149.2[1].get_m31(8);
            *row[278] = poseidon_full_round_chain_output_limb_20_col278;let poseidon_full_round_chain_output_limb_21_col279 = poseidon_full_round_chain_output_round_34_tmp_51986_149.2[1].get_m31(9);
            *row[279] = poseidon_full_round_chain_output_limb_21_col279;let poseidon_full_round_chain_output_limb_22_col280 = poseidon_full_round_chain_output_round_34_tmp_51986_149.2[2].get_m31(0);
            *row[280] = poseidon_full_round_chain_output_limb_22_col280;let poseidon_full_round_chain_output_limb_23_col281 = poseidon_full_round_chain_output_round_34_tmp_51986_149.2[2].get_m31(1);
            *row[281] = poseidon_full_round_chain_output_limb_23_col281;let poseidon_full_round_chain_output_limb_24_col282 = poseidon_full_round_chain_output_round_34_tmp_51986_149.2[2].get_m31(2);
            *row[282] = poseidon_full_round_chain_output_limb_24_col282;let poseidon_full_round_chain_output_limb_25_col283 = poseidon_full_round_chain_output_round_34_tmp_51986_149.2[2].get_m31(3);
            *row[283] = poseidon_full_round_chain_output_limb_25_col283;let poseidon_full_round_chain_output_limb_26_col284 = poseidon_full_round_chain_output_round_34_tmp_51986_149.2[2].get_m31(4);
            *row[284] = poseidon_full_round_chain_output_limb_26_col284;let poseidon_full_round_chain_output_limb_27_col285 = poseidon_full_round_chain_output_round_34_tmp_51986_149.2[2].get_m31(5);
            *row[285] = poseidon_full_round_chain_output_limb_27_col285;let poseidon_full_round_chain_output_limb_28_col286 = poseidon_full_round_chain_output_round_34_tmp_51986_149.2[2].get_m31(6);
            *row[286] = poseidon_full_round_chain_output_limb_28_col286;let poseidon_full_round_chain_output_limb_29_col287 = poseidon_full_round_chain_output_round_34_tmp_51986_149.2[2].get_m31(7);
            *row[287] = poseidon_full_round_chain_output_limb_29_col287;let poseidon_full_round_chain_output_limb_30_col288 = poseidon_full_round_chain_output_round_34_tmp_51986_149.2[2].get_m31(8);
            *row[288] = poseidon_full_round_chain_output_limb_30_col288;let poseidon_full_round_chain_output_limb_31_col289 = poseidon_full_round_chain_output_round_34_tmp_51986_149.2[2].get_m31(9);
            *row[289] = poseidon_full_round_chain_output_limb_31_col289;*lookup_data.poseidon_full_round_chain_3 = [poseidon_full_round_chain_output_limb_0_col258, poseidon_full_round_chain_output_limb_1_col259, poseidon_full_round_chain_output_limb_2_col260, poseidon_full_round_chain_output_limb_3_col261, poseidon_full_round_chain_output_limb_4_col262, poseidon_full_round_chain_output_limb_5_col263, poseidon_full_round_chain_output_limb_6_col264, poseidon_full_round_chain_output_limb_7_col265, poseidon_full_round_chain_output_limb_8_col266, poseidon_full_round_chain_output_limb_9_col267, poseidon_full_round_chain_output_limb_10_col268, poseidon_full_round_chain_output_limb_11_col269, poseidon_full_round_chain_output_limb_12_col270, poseidon_full_round_chain_output_limb_13_col271, poseidon_full_round_chain_output_limb_14_col272, poseidon_full_round_chain_output_limb_15_col273, poseidon_full_round_chain_output_limb_16_col274, poseidon_full_round_chain_output_limb_17_col275, poseidon_full_round_chain_output_limb_18_col276, poseidon_full_round_chain_output_limb_19_col277, poseidon_full_round_chain_output_limb_20_col278, poseidon_full_round_chain_output_limb_21_col279, poseidon_full_round_chain_output_limb_22_col280, poseidon_full_round_chain_output_limb_23_col281, poseidon_full_round_chain_output_limb_24_col282, poseidon_full_round_chain_output_limb_25_col283, poseidon_full_round_chain_output_limb_26_col284, poseidon_full_round_chain_output_limb_27_col285, poseidon_full_round_chain_output_limb_28_col286, poseidon_full_round_chain_output_limb_29_col287, poseidon_full_round_chain_output_limb_30_col288, poseidon_full_round_chain_output_limb_31_col289];

            //Felt 252 Unpack From 27.

            let input_as_felt252_tmp_51986_150 = PackedFelt252::from_packed_felt252width27(poseidon_full_round_chain_output_round_34_tmp_51986_149.2[0]);let unpacked_limb_0_col290 = input_as_felt252_tmp_51986_150.get_m31(0);
            *row[290] = unpacked_limb_0_col290;let unpacked_limb_1_col291 = input_as_felt252_tmp_51986_150.get_m31(1);
            *row[291] = unpacked_limb_1_col291;let unpacked_limb_3_col292 = input_as_felt252_tmp_51986_150.get_m31(3);
            *row[292] = unpacked_limb_3_col292;let unpacked_limb_4_col293 = input_as_felt252_tmp_51986_150.get_m31(4);
            *row[293] = unpacked_limb_4_col293;let unpacked_limb_6_col294 = input_as_felt252_tmp_51986_150.get_m31(6);
            *row[294] = unpacked_limb_6_col294;let unpacked_limb_7_col295 = input_as_felt252_tmp_51986_150.get_m31(7);
            *row[295] = unpacked_limb_7_col295;let unpacked_limb_9_col296 = input_as_felt252_tmp_51986_150.get_m31(9);
            *row[296] = unpacked_limb_9_col296;let unpacked_limb_10_col297 = input_as_felt252_tmp_51986_150.get_m31(10);
            *row[297] = unpacked_limb_10_col297;let unpacked_limb_12_col298 = input_as_felt252_tmp_51986_150.get_m31(12);
            *row[298] = unpacked_limb_12_col298;let unpacked_limb_13_col299 = input_as_felt252_tmp_51986_150.get_m31(13);
            *row[299] = unpacked_limb_13_col299;let unpacked_limb_15_col300 = input_as_felt252_tmp_51986_150.get_m31(15);
            *row[300] = unpacked_limb_15_col300;let unpacked_limb_16_col301 = input_as_felt252_tmp_51986_150.get_m31(16);
            *row[301] = unpacked_limb_16_col301;let unpacked_limb_18_col302 = input_as_felt252_tmp_51986_150.get_m31(18);
            *row[302] = unpacked_limb_18_col302;let unpacked_limb_19_col303 = input_as_felt252_tmp_51986_150.get_m31(19);
            *row[303] = unpacked_limb_19_col303;let unpacked_limb_21_col304 = input_as_felt252_tmp_51986_150.get_m31(21);
            *row[304] = unpacked_limb_21_col304;let unpacked_limb_22_col305 = input_as_felt252_tmp_51986_150.get_m31(22);
            *row[305] = unpacked_limb_22_col305;let unpacked_limb_24_col306 = input_as_felt252_tmp_51986_150.get_m31(24);
            *row[306] = unpacked_limb_24_col306;let unpacked_limb_25_col307 = input_as_felt252_tmp_51986_150.get_m31(25);
            *row[307] = unpacked_limb_25_col307;

            //Mem Verify.

            let memory_address_to_id_value_tmp_51986_151 = memory_address_to_id_state.deduce_output(((((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start))) + (((seq) * (M31_6))))) + (M31_3)));let output_state_0_id_col308 = memory_address_to_id_value_tmp_51986_151;
            *row[308] = output_state_0_id_col308;let memory_address_to_id_inputs_3 =
                ((((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start))) + (((seq) * (M31_6))))) + (M31_3)).unpack();
            *lookup_data.memory_address_to_id_3 = [((((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start))) + (((seq) * (M31_6))))) + (M31_3)), output_state_0_id_col308];let memory_id_to_big_inputs_3 =
                output_state_0_id_col308.unpack();
            *lookup_data.memory_id_to_big_3 = [output_state_0_id_col308, unpacked_limb_0_col290, unpacked_limb_1_col291, ((((((poseidon_full_round_chain_output_limb_2_col260) - (unpacked_limb_0_col290))) - (((unpacked_limb_1_col291) * (M31_512))))) * (M31_8192)), unpacked_limb_3_col292, unpacked_limb_4_col293, ((((((poseidon_full_round_chain_output_limb_3_col261) - (unpacked_limb_3_col292))) - (((unpacked_limb_4_col293) * (M31_512))))) * (M31_8192)), unpacked_limb_6_col294, unpacked_limb_7_col295, ((((((poseidon_full_round_chain_output_limb_4_col262) - (unpacked_limb_6_col294))) - (((unpacked_limb_7_col295) * (M31_512))))) * (M31_8192)), unpacked_limb_9_col296, unpacked_limb_10_col297, ((((((poseidon_full_round_chain_output_limb_5_col263) - (unpacked_limb_9_col296))) - (((unpacked_limb_10_col297) * (M31_512))))) * (M31_8192)), unpacked_limb_12_col298, unpacked_limb_13_col299, ((((((poseidon_full_round_chain_output_limb_6_col264) - (unpacked_limb_12_col298))) - (((unpacked_limb_13_col299) * (M31_512))))) * (M31_8192)), unpacked_limb_15_col300, unpacked_limb_16_col301, ((((((poseidon_full_round_chain_output_limb_7_col265) - (unpacked_limb_15_col300))) - (((unpacked_limb_16_col301) * (M31_512))))) * (M31_8192)), unpacked_limb_18_col302, unpacked_limb_19_col303, ((((((poseidon_full_round_chain_output_limb_8_col266) - (unpacked_limb_18_col302))) - (((unpacked_limb_19_col303) * (M31_512))))) * (M31_8192)), unpacked_limb_21_col304, unpacked_limb_22_col305, ((((((poseidon_full_round_chain_output_limb_9_col267) - (unpacked_limb_21_col304))) - (((unpacked_limb_22_col305) * (M31_512))))) * (M31_8192)), unpacked_limb_24_col306, unpacked_limb_25_col307, ((((((poseidon_full_round_chain_output_limb_10_col268) - (unpacked_limb_24_col306))) - (((unpacked_limb_25_col307) * (M31_512))))) * (M31_8192)), poseidon_full_round_chain_output_limb_11_col269];

            //Felt 252 Unpack From 27.

            let input_as_felt252_tmp_51986_152 = PackedFelt252::from_packed_felt252width27(poseidon_full_round_chain_output_round_34_tmp_51986_149.2[1]);let unpacked_limb_0_col309 = input_as_felt252_tmp_51986_152.get_m31(0);
            *row[309] = unpacked_limb_0_col309;let unpacked_limb_1_col310 = input_as_felt252_tmp_51986_152.get_m31(1);
            *row[310] = unpacked_limb_1_col310;let unpacked_limb_3_col311 = input_as_felt252_tmp_51986_152.get_m31(3);
            *row[311] = unpacked_limb_3_col311;let unpacked_limb_4_col312 = input_as_felt252_tmp_51986_152.get_m31(4);
            *row[312] = unpacked_limb_4_col312;let unpacked_limb_6_col313 = input_as_felt252_tmp_51986_152.get_m31(6);
            *row[313] = unpacked_limb_6_col313;let unpacked_limb_7_col314 = input_as_felt252_tmp_51986_152.get_m31(7);
            *row[314] = unpacked_limb_7_col314;let unpacked_limb_9_col315 = input_as_felt252_tmp_51986_152.get_m31(9);
            *row[315] = unpacked_limb_9_col315;let unpacked_limb_10_col316 = input_as_felt252_tmp_51986_152.get_m31(10);
            *row[316] = unpacked_limb_10_col316;let unpacked_limb_12_col317 = input_as_felt252_tmp_51986_152.get_m31(12);
            *row[317] = unpacked_limb_12_col317;let unpacked_limb_13_col318 = input_as_felt252_tmp_51986_152.get_m31(13);
            *row[318] = unpacked_limb_13_col318;let unpacked_limb_15_col319 = input_as_felt252_tmp_51986_152.get_m31(15);
            *row[319] = unpacked_limb_15_col319;let unpacked_limb_16_col320 = input_as_felt252_tmp_51986_152.get_m31(16);
            *row[320] = unpacked_limb_16_col320;let unpacked_limb_18_col321 = input_as_felt252_tmp_51986_152.get_m31(18);
            *row[321] = unpacked_limb_18_col321;let unpacked_limb_19_col322 = input_as_felt252_tmp_51986_152.get_m31(19);
            *row[322] = unpacked_limb_19_col322;let unpacked_limb_21_col323 = input_as_felt252_tmp_51986_152.get_m31(21);
            *row[323] = unpacked_limb_21_col323;let unpacked_limb_22_col324 = input_as_felt252_tmp_51986_152.get_m31(22);
            *row[324] = unpacked_limb_22_col324;let unpacked_limb_24_col325 = input_as_felt252_tmp_51986_152.get_m31(24);
            *row[325] = unpacked_limb_24_col325;let unpacked_limb_25_col326 = input_as_felt252_tmp_51986_152.get_m31(25);
            *row[326] = unpacked_limb_25_col326;

            //Mem Verify.

            let memory_address_to_id_value_tmp_51986_153 = memory_address_to_id_state.deduce_output(((((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start))) + (((seq) * (M31_6))))) + (M31_4)));let output_state_1_id_col327 = memory_address_to_id_value_tmp_51986_153;
            *row[327] = output_state_1_id_col327;let memory_address_to_id_inputs_4 =
                ((((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start))) + (((seq) * (M31_6))))) + (M31_4)).unpack();
            *lookup_data.memory_address_to_id_4 = [((((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start))) + (((seq) * (M31_6))))) + (M31_4)), output_state_1_id_col327];let memory_id_to_big_inputs_4 =
                output_state_1_id_col327.unpack();
            *lookup_data.memory_id_to_big_4 = [output_state_1_id_col327, unpacked_limb_0_col309, unpacked_limb_1_col310, ((((((poseidon_full_round_chain_output_limb_12_col270) - (unpacked_limb_0_col309))) - (((unpacked_limb_1_col310) * (M31_512))))) * (M31_8192)), unpacked_limb_3_col311, unpacked_limb_4_col312, ((((((poseidon_full_round_chain_output_limb_13_col271) - (unpacked_limb_3_col311))) - (((unpacked_limb_4_col312) * (M31_512))))) * (M31_8192)), unpacked_limb_6_col313, unpacked_limb_7_col314, ((((((poseidon_full_round_chain_output_limb_14_col272) - (unpacked_limb_6_col313))) - (((unpacked_limb_7_col314) * (M31_512))))) * (M31_8192)), unpacked_limb_9_col315, unpacked_limb_10_col316, ((((((poseidon_full_round_chain_output_limb_15_col273) - (unpacked_limb_9_col315))) - (((unpacked_limb_10_col316) * (M31_512))))) * (M31_8192)), unpacked_limb_12_col317, unpacked_limb_13_col318, ((((((poseidon_full_round_chain_output_limb_16_col274) - (unpacked_limb_12_col317))) - (((unpacked_limb_13_col318) * (M31_512))))) * (M31_8192)), unpacked_limb_15_col319, unpacked_limb_16_col320, ((((((poseidon_full_round_chain_output_limb_17_col275) - (unpacked_limb_15_col319))) - (((unpacked_limb_16_col320) * (M31_512))))) * (M31_8192)), unpacked_limb_18_col321, unpacked_limb_19_col322, ((((((poseidon_full_round_chain_output_limb_18_col276) - (unpacked_limb_18_col321))) - (((unpacked_limb_19_col322) * (M31_512))))) * (M31_8192)), unpacked_limb_21_col323, unpacked_limb_22_col324, ((((((poseidon_full_round_chain_output_limb_19_col277) - (unpacked_limb_21_col323))) - (((unpacked_limb_22_col324) * (M31_512))))) * (M31_8192)), unpacked_limb_24_col325, unpacked_limb_25_col326, ((((((poseidon_full_round_chain_output_limb_20_col278) - (unpacked_limb_24_col325))) - (((unpacked_limb_25_col326) * (M31_512))))) * (M31_8192)), poseidon_full_round_chain_output_limb_21_col279];

            //Felt 252 Unpack From 27.

            let input_as_felt252_tmp_51986_154 = PackedFelt252::from_packed_felt252width27(poseidon_full_round_chain_output_round_34_tmp_51986_149.2[2]);let unpacked_limb_0_col328 = input_as_felt252_tmp_51986_154.get_m31(0);
            *row[328] = unpacked_limb_0_col328;let unpacked_limb_1_col329 = input_as_felt252_tmp_51986_154.get_m31(1);
            *row[329] = unpacked_limb_1_col329;let unpacked_limb_3_col330 = input_as_felt252_tmp_51986_154.get_m31(3);
            *row[330] = unpacked_limb_3_col330;let unpacked_limb_4_col331 = input_as_felt252_tmp_51986_154.get_m31(4);
            *row[331] = unpacked_limb_4_col331;let unpacked_limb_6_col332 = input_as_felt252_tmp_51986_154.get_m31(6);
            *row[332] = unpacked_limb_6_col332;let unpacked_limb_7_col333 = input_as_felt252_tmp_51986_154.get_m31(7);
            *row[333] = unpacked_limb_7_col333;let unpacked_limb_9_col334 = input_as_felt252_tmp_51986_154.get_m31(9);
            *row[334] = unpacked_limb_9_col334;let unpacked_limb_10_col335 = input_as_felt252_tmp_51986_154.get_m31(10);
            *row[335] = unpacked_limb_10_col335;let unpacked_limb_12_col336 = input_as_felt252_tmp_51986_154.get_m31(12);
            *row[336] = unpacked_limb_12_col336;let unpacked_limb_13_col337 = input_as_felt252_tmp_51986_154.get_m31(13);
            *row[337] = unpacked_limb_13_col337;let unpacked_limb_15_col338 = input_as_felt252_tmp_51986_154.get_m31(15);
            *row[338] = unpacked_limb_15_col338;let unpacked_limb_16_col339 = input_as_felt252_tmp_51986_154.get_m31(16);
            *row[339] = unpacked_limb_16_col339;let unpacked_limb_18_col340 = input_as_felt252_tmp_51986_154.get_m31(18);
            *row[340] = unpacked_limb_18_col340;let unpacked_limb_19_col341 = input_as_felt252_tmp_51986_154.get_m31(19);
            *row[341] = unpacked_limb_19_col341;let unpacked_limb_21_col342 = input_as_felt252_tmp_51986_154.get_m31(21);
            *row[342] = unpacked_limb_21_col342;let unpacked_limb_22_col343 = input_as_felt252_tmp_51986_154.get_m31(22);
            *row[343] = unpacked_limb_22_col343;let unpacked_limb_24_col344 = input_as_felt252_tmp_51986_154.get_m31(24);
            *row[344] = unpacked_limb_24_col344;let unpacked_limb_25_col345 = input_as_felt252_tmp_51986_154.get_m31(25);
            *row[345] = unpacked_limb_25_col345;

            //Mem Verify.

            let memory_address_to_id_value_tmp_51986_155 = memory_address_to_id_state.deduce_output(((((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start))) + (((seq) * (M31_6))))) + (M31_5)));let output_state_2_id_col346 = memory_address_to_id_value_tmp_51986_155;
            *row[346] = output_state_2_id_col346;let memory_address_to_id_inputs_5 =
                ((((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start))) + (((seq) * (M31_6))))) + (M31_5)).unpack();
            *lookup_data.memory_address_to_id_5 = [((((PackedM31::broadcast(M31::from(poseidon_builtin_segment_start))) + (((seq) * (M31_6))))) + (M31_5)), output_state_2_id_col346];let memory_id_to_big_inputs_5 =
                output_state_2_id_col346.unpack();
            *lookup_data.memory_id_to_big_5 = [output_state_2_id_col346, unpacked_limb_0_col328, unpacked_limb_1_col329, ((((((poseidon_full_round_chain_output_limb_22_col280) - (unpacked_limb_0_col328))) - (((unpacked_limb_1_col329) * (M31_512))))) * (M31_8192)), unpacked_limb_3_col330, unpacked_limb_4_col331, ((((((poseidon_full_round_chain_output_limb_23_col281) - (unpacked_limb_3_col330))) - (((unpacked_limb_4_col331) * (M31_512))))) * (M31_8192)), unpacked_limb_6_col332, unpacked_limb_7_col333, ((((((poseidon_full_round_chain_output_limb_24_col282) - (unpacked_limb_6_col332))) - (((unpacked_limb_7_col333) * (M31_512))))) * (M31_8192)), unpacked_limb_9_col334, unpacked_limb_10_col335, ((((((poseidon_full_round_chain_output_limb_25_col283) - (unpacked_limb_9_col334))) - (((unpacked_limb_10_col335) * (M31_512))))) * (M31_8192)), unpacked_limb_12_col336, unpacked_limb_13_col337, ((((((poseidon_full_round_chain_output_limb_26_col284) - (unpacked_limb_12_col336))) - (((unpacked_limb_13_col337) * (M31_512))))) * (M31_8192)), unpacked_limb_15_col338, unpacked_limb_16_col339, ((((((poseidon_full_round_chain_output_limb_27_col285) - (unpacked_limb_15_col338))) - (((unpacked_limb_16_col339) * (M31_512))))) * (M31_8192)), unpacked_limb_18_col340, unpacked_limb_19_col341, ((((((poseidon_full_round_chain_output_limb_28_col286) - (unpacked_limb_18_col340))) - (((unpacked_limb_19_col341) * (M31_512))))) * (M31_8192)), unpacked_limb_21_col342, unpacked_limb_22_col343, ((((((poseidon_full_round_chain_output_limb_29_col287) - (unpacked_limb_21_col342))) - (((unpacked_limb_22_col343) * (M31_512))))) * (M31_8192)), unpacked_limb_24_col344, unpacked_limb_25_col345, ((((((poseidon_full_round_chain_output_limb_30_col288) - (unpacked_limb_24_col344))) - (((unpacked_limb_25_col345) * (M31_512))))) * (M31_8192)), poseidon_full_round_chain_output_limb_31_col289];

            // Add sub-components inputs.
            *cube_252_input = [cube_252_inputs_0, cube_252_inputs_1];
            *range_check_felt_252_width_27_input = [
                    range_check_felt_252_width_27_inputs_0,
                    range_check_felt_252_width_27_inputs_1,
                ];
            *poseidon_full_round_chain_input = [
                    poseidon_full_round_chain_inputs_0,
                    poseidon_full_round_chain_inputs_1,
                    poseidon_full_round_chain_inputs_2,
                    poseidon_full_round_chain_inputs_3,
                    poseidon_full_round_chain_inputs_4,
                    poseidon_full_round_chain_inputs_5,
                    poseidon_full_round_chain_inputs_6,
                    poseidon_full_round_chain_inputs_7,
                ];
            *poseidon_3_partial_rounds_chain_input = [
                    poseidon_3_partial_rounds_chain_inputs_0,
                    poseidon_3_partial_rounds_chain_inputs_1,
                    poseidon_3_partial_rounds_chain_inputs_2,
                    poseidon_3_partial_rounds_chain_inputs_3,
                    poseidon_3_partial_rounds_chain_inputs_4,
                    poseidon_3_partial_rounds_chain_inputs_5,
                    poseidon_3_partial_rounds_chain_inputs_6,
                    poseidon_3_partial_rounds_chain_inputs_7,
                    poseidon_3_partial_rounds_chain_inputs_8,
                    poseidon_3_partial_rounds_chain_inputs_9,
                    poseidon_3_partial_rounds_chain_inputs_10,
                    poseidon_3_partial_rounds_chain_inputs_11,
                    poseidon_3_partial_rounds_chain_inputs_12,
                    poseidon_3_partial_rounds_chain_inputs_13,
                    poseidon_3_partial_rounds_chain_inputs_14,
                    poseidon_3_partial_rounds_chain_inputs_15,
                    poseidon_3_partial_rounds_chain_inputs_16,
                    poseidon_3_partial_rounds_chain_inputs_17,
                    poseidon_3_partial_rounds_chain_inputs_18,
                    poseidon_3_partial_rounds_chain_inputs_19,
                    poseidon_3_partial_rounds_chain_inputs_20,
                    poseidon_3_partial_rounds_chain_inputs_21,
                    poseidon_3_partial_rounds_chain_inputs_22,
                    poseidon_3_partial_rounds_chain_inputs_23,
                    poseidon_3_partial_rounds_chain_inputs_24,
                    poseidon_3_partial_rounds_chain_inputs_25,
                    poseidon_3_partial_rounds_chain_inputs_26,
                ];
            memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_0);
            memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_0);
            memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_1);
            memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_1);
            memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_2);
            memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_2);
            range_check_3_3_3_3_3_state.add_inputs(&range_check_3_3_3_3_3_inputs_0);
            range_check_3_3_3_3_3_state.add_inputs(&range_check_3_3_3_3_3_inputs_1);
            range_check_4_4_4_4_state.add_inputs(&range_check_4_4_4_4_inputs_0);
            range_check_4_4_4_4_state.add_inputs(&range_check_4_4_4_4_inputs_1);
            range_check_4_4_state.add_inputs(&range_check_4_4_inputs_0);
            range_check_4_4_4_4_state.add_inputs(&range_check_4_4_4_4_inputs_2);
            range_check_4_4_4_4_state.add_inputs(&range_check_4_4_4_4_inputs_3);
            range_check_4_4_state.add_inputs(&range_check_4_4_inputs_1);
            range_check_4_4_4_4_state.add_inputs(&range_check_4_4_4_4_inputs_4);
            range_check_4_4_4_4_state.add_inputs(&range_check_4_4_4_4_inputs_5);
            range_check_4_4_state.add_inputs(&range_check_4_4_inputs_2);
            memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_3);
            memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_3);
            memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_4);
            memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_4);
            memory_address_to_id_state.add_inputs(&memory_address_to_id_inputs_5);
            memory_id_to_big_state.add_inputs(&memory_id_to_big_inputs_5);
        });

    cube_252_inputs_vec.iter().for_each(|inputs_array| {
        inputs_array
            .iter()
            .for_each(|inputs| cube_252_state.add_inputs(inputs))
    });
    range_check_felt_252_width_27_inputs_vec
        .iter()
        .for_each(|inputs_array| {
            inputs_array
                .iter()
                .for_each(|inputs| range_check_felt_252_width_27_state.add_inputs(inputs))
        });
    poseidon_full_round_chain_inputs_vec
        .iter()
        .for_each(|inputs_array| {
            inputs_array
                .iter()
                .for_each(|inputs| poseidon_full_round_chain_state.add_inputs(inputs))
        });
    poseidon_3_partial_rounds_chain_inputs_vec
        .iter()
        .for_each(|inputs_array| {
            inputs_array
                .iter()
                .for_each(|inputs| poseidon_3_partial_rounds_chain_state.add_inputs(inputs))
        });

    (trace, lookup_data)
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
    pub fn write_interaction_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        cube_252: &relations::Cube252,
        memory_address_to_id: &relations::MemoryAddressToId,
        memory_id_to_big: &relations::MemoryIdToBig,
        poseidon_3_partial_rounds_chain: &relations::Poseidon3PartialRoundsChain,
        poseidon_full_round_chain: &relations::PoseidonFullRoundChain,
        range_check_felt_252_width_27: &relations::RangeCheckFelt252Width27,
        range_check_3_3_3_3_3: &relations::RangeCheck_3_3_3_3_3,
        range_check_4_4: &relations::RangeCheck_4_4,
        range_check_4_4_4_4: &relations::RangeCheck_4_4_4_4,
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
            &self.lookup_data.poseidon_full_round_chain_0,
            &self.lookup_data.poseidon_full_round_chain_1,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = poseidon_full_round_chain.combine(values0);
            let denom1: PackedQM31 = poseidon_full_round_chain.combine(values1);
            col_gen.write_frac(i, denom0 - denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_felt_252_width_27_0,
            &self.lookup_data.range_check_felt_252_width_27_1,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_felt_252_width_27.combine(values0);
            let denom1: PackedQM31 = range_check_felt_252_width_27.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.cube_252_0,
            &self.lookup_data.range_check_3_3_3_3_3_0,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = cube_252.combine(values0);
            let denom1: PackedQM31 = range_check_3_3_3_3_3.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_3_3_3_3_3_1,
            &self.lookup_data.cube_252_1,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_3_3_3_3_3.combine(values0);
            let denom1: PackedQM31 = cube_252.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_4_4_4_4_0,
            &self.lookup_data.range_check_4_4_4_4_1,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_4_4_4_4.combine(values0);
            let denom1: PackedQM31 = range_check_4_4_4_4.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_4_4_0,
            &self.lookup_data.poseidon_3_partial_rounds_chain_0,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_4_4.combine(values0);
            let denom1: PackedQM31 = poseidon_3_partial_rounds_chain.combine(values1);
            col_gen.write_frac(i, denom1 - denom0, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.poseidon_3_partial_rounds_chain_1,
            &self.lookup_data.range_check_4_4_4_4_2,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = poseidon_3_partial_rounds_chain.combine(values0);
            let denom1: PackedQM31 = range_check_4_4_4_4.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_4_4_4_4_3,
            &self.lookup_data.range_check_4_4_1,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_4_4_4_4.combine(values0);
            let denom1: PackedQM31 = range_check_4_4.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_4_4_4_4_4,
            &self.lookup_data.range_check_4_4_4_4_5,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_4_4_4_4.combine(values0);
            let denom1: PackedQM31 = range_check_4_4_4_4.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.range_check_4_4_2,
            &self.lookup_data.poseidon_full_round_chain_2,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = range_check_4_4.combine(values0);
            let denom1: PackedQM31 = poseidon_full_round_chain.combine(values1);
            col_gen.write_frac(i, denom1 - denom0, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.poseidon_full_round_chain_3,
            &self.lookup_data.memory_address_to_id_3,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = poseidon_full_round_chain.combine(values0);
            let denom1: PackedQM31 = memory_address_to_id.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        for (i, (values0, values1)) in zip(
            &self.lookup_data.memory_id_to_big_3,
            &self.lookup_data.memory_address_to_id_4,
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
            &self.lookup_data.memory_id_to_big_4,
            &self.lookup_data.memory_address_to_id_5,
        )
        .enumerate()
        {
            let denom0: PackedQM31 = memory_id_to_big.combine(values0);
            let denom1: PackedQM31 = memory_address_to_id.combine(values1);
            col_gen.write_frac(i, denom0 + denom1, denom0 * denom1);
        }
        col_gen.finalize_col();

        // Sum last logup term.
        let mut col_gen = logup_gen.new_col();
        for (i, values) in self.lookup_data.memory_id_to_big_5.iter().enumerate() {
            let denom = memory_id_to_big.combine(values);
            col_gen.write_frac(i, PackedQM31::one(), denom);
        }
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}
