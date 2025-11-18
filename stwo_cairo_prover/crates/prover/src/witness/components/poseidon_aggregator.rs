// This file was created by the AIR team.

#![allow(unused_parens)]
use cairo_air::components::poseidon_aggregator::{Claim, InteractionClaim, N_TRACE_COLUMNS};

use crate::witness::components::{
    cube_252, memory_id_to_big, poseidon_3_partial_rounds_chain, poseidon_full_round_chain,
    range_check_252_width_27, range_check_3_3_3_3_3, range_check_4_4, range_check_4_4_4_4,
};
use crate::witness::prelude::*;

pub type InputType = ([M31; 3], [M31; 3]);
pub type PackedInputType = ([PackedM31; 3], [PackedM31; 3]);

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
        memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
        poseidon_full_round_chain_state: &mut poseidon_full_round_chain::ClaimGenerator,
        range_check_252_width_27_state: &mut range_check_252_width_27::ClaimGenerator,
        cube_252_state: &mut cube_252::ClaimGenerator,
        range_check_3_3_3_3_3_state: &range_check_3_3_3_3_3::ClaimGenerator,
        range_check_4_4_4_4_state: &range_check_4_4_4_4::ClaimGenerator,
        range_check_4_4_state: &range_check_4_4::ClaimGenerator,
        poseidon_3_partial_rounds_chain_state: &mut poseidon_3_partial_rounds_chain::ClaimGenerator,
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
            memory_id_to_big_state,
            poseidon_full_round_chain_state,
            range_check_252_width_27_state,
            cube_252_state,
            range_check_3_3_3_3_3_state,
            range_check_4_4_4_4_state,
            range_check_4_4_state,
            poseidon_3_partial_rounds_chain_state,
        );
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
            .range_check_252_width_27
            .iter()
            .for_each(|inputs| {
                range_check_252_width_27_state.add_packed_inputs(inputs);
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
    memory_id_to_big: [Vec<memory_id_to_big::PackedInputType>; 6],
    poseidon_full_round_chain: [Vec<poseidon_full_round_chain::PackedInputType>; 8],
    range_check_252_width_27: [Vec<range_check_252_width_27::PackedInputType>; 2],
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
    inputs: Vec<PackedInputType>,
    mults: Vec<PackedM31>,
    memory_id_to_big_state: &memory_id_to_big::ClaimGenerator,
    poseidon_full_round_chain_state: &mut poseidon_full_round_chain::ClaimGenerator,
    range_check_252_width_27_state: &mut range_check_252_width_27::ClaimGenerator,
    cube_252_state: &mut cube_252::ClaimGenerator,
    range_check_3_3_3_3_3_state: &range_check_3_3_3_3_3::ClaimGenerator,
    range_check_4_4_4_4_state: &range_check_4_4_4_4::ClaimGenerator,
    range_check_4_4_state: &range_check_4_4::ClaimGenerator,
    poseidon_3_partial_rounds_chain_state: &mut poseidon_3_partial_rounds_chain::ClaimGenerator,
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
    lookup_data.par_iter_mut(),sub_component_inputs.par_iter_mut(),inputs.into_par_iter(),)
    .into_par_iter()
    .enumerate()
    .for_each(
        |(row_index,(row, lookup_data,sub_component_inputs,poseidon_aggregator_input,))| {
            let seq = seq.packed_at(row_index);let input_limb_0_col0 = poseidon_aggregator_input.0[0];
            *row[0] = input_limb_0_col0;let input_limb_1_col1 = poseidon_aggregator_input.0[1];
            *row[1] = input_limb_1_col1;let input_limb_2_col2 = poseidon_aggregator_input.0[2];
            *row[2] = input_limb_2_col2;let input_limb_3_col3 = poseidon_aggregator_input.1[0];
            *row[3] = input_limb_3_col3;let input_limb_4_col4 = poseidon_aggregator_input.1[1];
            *row[4] = input_limb_4_col4;let input_limb_5_col5 = poseidon_aggregator_input.1[2];
            *row[5] = input_limb_5_col5;

            // Read Positive Known Id Num Bits 252.

            let memory_id_to_big_value_tmp_34cc4_0 = memory_id_to_big_state.deduce_output(input_limb_0_col0);let value_limb_0_col6 = memory_id_to_big_value_tmp_34cc4_0.get_m31(0);
            *row[6] = value_limb_0_col6;let value_limb_1_col7 = memory_id_to_big_value_tmp_34cc4_0.get_m31(1);
            *row[7] = value_limb_1_col7;let value_limb_2_col8 = memory_id_to_big_value_tmp_34cc4_0.get_m31(2);
            *row[8] = value_limb_2_col8;let value_limb_3_col9 = memory_id_to_big_value_tmp_34cc4_0.get_m31(3);
            *row[9] = value_limb_3_col9;let value_limb_4_col10 = memory_id_to_big_value_tmp_34cc4_0.get_m31(4);
            *row[10] = value_limb_4_col10;let value_limb_5_col11 = memory_id_to_big_value_tmp_34cc4_0.get_m31(5);
            *row[11] = value_limb_5_col11;let value_limb_6_col12 = memory_id_to_big_value_tmp_34cc4_0.get_m31(6);
            *row[12] = value_limb_6_col12;let value_limb_7_col13 = memory_id_to_big_value_tmp_34cc4_0.get_m31(7);
            *row[13] = value_limb_7_col13;let value_limb_8_col14 = memory_id_to_big_value_tmp_34cc4_0.get_m31(8);
            *row[14] = value_limb_8_col14;let value_limb_9_col15 = memory_id_to_big_value_tmp_34cc4_0.get_m31(9);
            *row[15] = value_limb_9_col15;let value_limb_10_col16 = memory_id_to_big_value_tmp_34cc4_0.get_m31(10);
            *row[16] = value_limb_10_col16;let value_limb_11_col17 = memory_id_to_big_value_tmp_34cc4_0.get_m31(11);
            *row[17] = value_limb_11_col17;let value_limb_12_col18 = memory_id_to_big_value_tmp_34cc4_0.get_m31(12);
            *row[18] = value_limb_12_col18;let value_limb_13_col19 = memory_id_to_big_value_tmp_34cc4_0.get_m31(13);
            *row[19] = value_limb_13_col19;let value_limb_14_col20 = memory_id_to_big_value_tmp_34cc4_0.get_m31(14);
            *row[20] = value_limb_14_col20;let value_limb_15_col21 = memory_id_to_big_value_tmp_34cc4_0.get_m31(15);
            *row[21] = value_limb_15_col21;let value_limb_16_col22 = memory_id_to_big_value_tmp_34cc4_0.get_m31(16);
            *row[22] = value_limb_16_col22;let value_limb_17_col23 = memory_id_to_big_value_tmp_34cc4_0.get_m31(17);
            *row[23] = value_limb_17_col23;let value_limb_18_col24 = memory_id_to_big_value_tmp_34cc4_0.get_m31(18);
            *row[24] = value_limb_18_col24;let value_limb_19_col25 = memory_id_to_big_value_tmp_34cc4_0.get_m31(19);
            *row[25] = value_limb_19_col25;let value_limb_20_col26 = memory_id_to_big_value_tmp_34cc4_0.get_m31(20);
            *row[26] = value_limb_20_col26;let value_limb_21_col27 = memory_id_to_big_value_tmp_34cc4_0.get_m31(21);
            *row[27] = value_limb_21_col27;let value_limb_22_col28 = memory_id_to_big_value_tmp_34cc4_0.get_m31(22);
            *row[28] = value_limb_22_col28;let value_limb_23_col29 = memory_id_to_big_value_tmp_34cc4_0.get_m31(23);
            *row[29] = value_limb_23_col29;let value_limb_24_col30 = memory_id_to_big_value_tmp_34cc4_0.get_m31(24);
            *row[30] = value_limb_24_col30;let value_limb_25_col31 = memory_id_to_big_value_tmp_34cc4_0.get_m31(25);
            *row[31] = value_limb_25_col31;let value_limb_26_col32 = memory_id_to_big_value_tmp_34cc4_0.get_m31(26);
            *row[32] = value_limb_26_col32;let value_limb_27_col33 = memory_id_to_big_value_tmp_34cc4_0.get_m31(27);
            *row[33] = value_limb_27_col33;*sub_component_inputs.memory_id_to_big[0] =
                input_limb_0_col0;
            *lookup_data.memory_id_to_big_0 = [input_limb_0_col0, value_limb_0_col6, value_limb_1_col7, value_limb_2_col8, value_limb_3_col9, value_limb_4_col10, value_limb_5_col11, value_limb_6_col12, value_limb_7_col13, value_limb_8_col14, value_limb_9_col15, value_limb_10_col16, value_limb_11_col17, value_limb_12_col18, value_limb_13_col19, value_limb_14_col20, value_limb_15_col21, value_limb_16_col22, value_limb_17_col23, value_limb_18_col24, value_limb_19_col25, value_limb_20_col26, value_limb_21_col27, value_limb_22_col28, value_limb_23_col29, value_limb_24_col30, value_limb_25_col31, value_limb_26_col32, value_limb_27_col33];let read_positive_known_id_num_bits_252_output_tmp_34cc4_1 = PackedFelt252::from_limbs([value_limb_0_col6, value_limb_1_col7, value_limb_2_col8, value_limb_3_col9, value_limb_4_col10, value_limb_5_col11, value_limb_6_col12, value_limb_7_col13, value_limb_8_col14, value_limb_9_col15, value_limb_10_col16, value_limb_11_col17, value_limb_12_col18, value_limb_13_col19, value_limb_14_col20, value_limb_15_col21, value_limb_16_col22, value_limb_17_col23, value_limb_18_col24, value_limb_19_col25, value_limb_20_col26, value_limb_21_col27, value_limb_22_col28, value_limb_23_col29, value_limb_24_col30, value_limb_25_col31, value_limb_26_col32, value_limb_27_col33]);

            let packed_input_state_0_tmp_34cc4_2 = PackedFelt252Width27::from_limbs([((((value_limb_0_col6) + (((value_limb_1_col7) * (M31_512))))) + (((value_limb_2_col8) * (M31_262144)))), ((((value_limb_3_col9) + (((value_limb_4_col10) * (M31_512))))) + (((value_limb_5_col11) * (M31_262144)))), ((((value_limb_6_col12) + (((value_limb_7_col13) * (M31_512))))) + (((value_limb_8_col14) * (M31_262144)))), ((((value_limb_9_col15) + (((value_limb_10_col16) * (M31_512))))) + (((value_limb_11_col17) * (M31_262144)))), ((((value_limb_12_col18) + (((value_limb_13_col19) * (M31_512))))) + (((value_limb_14_col20) * (M31_262144)))), ((((value_limb_15_col21) + (((value_limb_16_col22) * (M31_512))))) + (((value_limb_17_col23) * (M31_262144)))), ((((value_limb_18_col24) + (((value_limb_19_col25) * (M31_512))))) + (((value_limb_20_col26) * (M31_262144)))), ((((value_limb_21_col27) + (((value_limb_22_col28) * (M31_512))))) + (((value_limb_23_col29) * (M31_262144)))), ((((value_limb_24_col30) + (((value_limb_25_col31) * (M31_512))))) + (((value_limb_26_col32) * (M31_262144)))), value_limb_27_col33]);

            // Read Positive Known Id Num Bits 252.

            let memory_id_to_big_value_tmp_34cc4_3 = memory_id_to_big_state.deduce_output(input_limb_1_col1);let value_limb_0_col34 = memory_id_to_big_value_tmp_34cc4_3.get_m31(0);
            *row[34] = value_limb_0_col34;let value_limb_1_col35 = memory_id_to_big_value_tmp_34cc4_3.get_m31(1);
            *row[35] = value_limb_1_col35;let value_limb_2_col36 = memory_id_to_big_value_tmp_34cc4_3.get_m31(2);
            *row[36] = value_limb_2_col36;let value_limb_3_col37 = memory_id_to_big_value_tmp_34cc4_3.get_m31(3);
            *row[37] = value_limb_3_col37;let value_limb_4_col38 = memory_id_to_big_value_tmp_34cc4_3.get_m31(4);
            *row[38] = value_limb_4_col38;let value_limb_5_col39 = memory_id_to_big_value_tmp_34cc4_3.get_m31(5);
            *row[39] = value_limb_5_col39;let value_limb_6_col40 = memory_id_to_big_value_tmp_34cc4_3.get_m31(6);
            *row[40] = value_limb_6_col40;let value_limb_7_col41 = memory_id_to_big_value_tmp_34cc4_3.get_m31(7);
            *row[41] = value_limb_7_col41;let value_limb_8_col42 = memory_id_to_big_value_tmp_34cc4_3.get_m31(8);
            *row[42] = value_limb_8_col42;let value_limb_9_col43 = memory_id_to_big_value_tmp_34cc4_3.get_m31(9);
            *row[43] = value_limb_9_col43;let value_limb_10_col44 = memory_id_to_big_value_tmp_34cc4_3.get_m31(10);
            *row[44] = value_limb_10_col44;let value_limb_11_col45 = memory_id_to_big_value_tmp_34cc4_3.get_m31(11);
            *row[45] = value_limb_11_col45;let value_limb_12_col46 = memory_id_to_big_value_tmp_34cc4_3.get_m31(12);
            *row[46] = value_limb_12_col46;let value_limb_13_col47 = memory_id_to_big_value_tmp_34cc4_3.get_m31(13);
            *row[47] = value_limb_13_col47;let value_limb_14_col48 = memory_id_to_big_value_tmp_34cc4_3.get_m31(14);
            *row[48] = value_limb_14_col48;let value_limb_15_col49 = memory_id_to_big_value_tmp_34cc4_3.get_m31(15);
            *row[49] = value_limb_15_col49;let value_limb_16_col50 = memory_id_to_big_value_tmp_34cc4_3.get_m31(16);
            *row[50] = value_limb_16_col50;let value_limb_17_col51 = memory_id_to_big_value_tmp_34cc4_3.get_m31(17);
            *row[51] = value_limb_17_col51;let value_limb_18_col52 = memory_id_to_big_value_tmp_34cc4_3.get_m31(18);
            *row[52] = value_limb_18_col52;let value_limb_19_col53 = memory_id_to_big_value_tmp_34cc4_3.get_m31(19);
            *row[53] = value_limb_19_col53;let value_limb_20_col54 = memory_id_to_big_value_tmp_34cc4_3.get_m31(20);
            *row[54] = value_limb_20_col54;let value_limb_21_col55 = memory_id_to_big_value_tmp_34cc4_3.get_m31(21);
            *row[55] = value_limb_21_col55;let value_limb_22_col56 = memory_id_to_big_value_tmp_34cc4_3.get_m31(22);
            *row[56] = value_limb_22_col56;let value_limb_23_col57 = memory_id_to_big_value_tmp_34cc4_3.get_m31(23);
            *row[57] = value_limb_23_col57;let value_limb_24_col58 = memory_id_to_big_value_tmp_34cc4_3.get_m31(24);
            *row[58] = value_limb_24_col58;let value_limb_25_col59 = memory_id_to_big_value_tmp_34cc4_3.get_m31(25);
            *row[59] = value_limb_25_col59;let value_limb_26_col60 = memory_id_to_big_value_tmp_34cc4_3.get_m31(26);
            *row[60] = value_limb_26_col60;let value_limb_27_col61 = memory_id_to_big_value_tmp_34cc4_3.get_m31(27);
            *row[61] = value_limb_27_col61;*sub_component_inputs.memory_id_to_big[1] =
                input_limb_1_col1;
            *lookup_data.memory_id_to_big_1 = [input_limb_1_col1, value_limb_0_col34, value_limb_1_col35, value_limb_2_col36, value_limb_3_col37, value_limb_4_col38, value_limb_5_col39, value_limb_6_col40, value_limb_7_col41, value_limb_8_col42, value_limb_9_col43, value_limb_10_col44, value_limb_11_col45, value_limb_12_col46, value_limb_13_col47, value_limb_14_col48, value_limb_15_col49, value_limb_16_col50, value_limb_17_col51, value_limb_18_col52, value_limb_19_col53, value_limb_20_col54, value_limb_21_col55, value_limb_22_col56, value_limb_23_col57, value_limb_24_col58, value_limb_25_col59, value_limb_26_col60, value_limb_27_col61];let read_positive_known_id_num_bits_252_output_tmp_34cc4_4 = PackedFelt252::from_limbs([value_limb_0_col34, value_limb_1_col35, value_limb_2_col36, value_limb_3_col37, value_limb_4_col38, value_limb_5_col39, value_limb_6_col40, value_limb_7_col41, value_limb_8_col42, value_limb_9_col43, value_limb_10_col44, value_limb_11_col45, value_limb_12_col46, value_limb_13_col47, value_limb_14_col48, value_limb_15_col49, value_limb_16_col50, value_limb_17_col51, value_limb_18_col52, value_limb_19_col53, value_limb_20_col54, value_limb_21_col55, value_limb_22_col56, value_limb_23_col57, value_limb_24_col58, value_limb_25_col59, value_limb_26_col60, value_limb_27_col61]);

            let packed_input_state_1_tmp_34cc4_5 = PackedFelt252Width27::from_limbs([((((value_limb_0_col34) + (((value_limb_1_col35) * (M31_512))))) + (((value_limb_2_col36) * (M31_262144)))), ((((value_limb_3_col37) + (((value_limb_4_col38) * (M31_512))))) + (((value_limb_5_col39) * (M31_262144)))), ((((value_limb_6_col40) + (((value_limb_7_col41) * (M31_512))))) + (((value_limb_8_col42) * (M31_262144)))), ((((value_limb_9_col43) + (((value_limb_10_col44) * (M31_512))))) + (((value_limb_11_col45) * (M31_262144)))), ((((value_limb_12_col46) + (((value_limb_13_col47) * (M31_512))))) + (((value_limb_14_col48) * (M31_262144)))), ((((value_limb_15_col49) + (((value_limb_16_col50) * (M31_512))))) + (((value_limb_17_col51) * (M31_262144)))), ((((value_limb_18_col52) + (((value_limb_19_col53) * (M31_512))))) + (((value_limb_20_col54) * (M31_262144)))), ((((value_limb_21_col55) + (((value_limb_22_col56) * (M31_512))))) + (((value_limb_23_col57) * (M31_262144)))), ((((value_limb_24_col58) + (((value_limb_25_col59) * (M31_512))))) + (((value_limb_26_col60) * (M31_262144)))), value_limb_27_col61]);

            // Read Positive Known Id Num Bits 252.

            let memory_id_to_big_value_tmp_34cc4_6 = memory_id_to_big_state.deduce_output(input_limb_2_col2);let value_limb_0_col62 = memory_id_to_big_value_tmp_34cc4_6.get_m31(0);
            *row[62] = value_limb_0_col62;let value_limb_1_col63 = memory_id_to_big_value_tmp_34cc4_6.get_m31(1);
            *row[63] = value_limb_1_col63;let value_limb_2_col64 = memory_id_to_big_value_tmp_34cc4_6.get_m31(2);
            *row[64] = value_limb_2_col64;let value_limb_3_col65 = memory_id_to_big_value_tmp_34cc4_6.get_m31(3);
            *row[65] = value_limb_3_col65;let value_limb_4_col66 = memory_id_to_big_value_tmp_34cc4_6.get_m31(4);
            *row[66] = value_limb_4_col66;let value_limb_5_col67 = memory_id_to_big_value_tmp_34cc4_6.get_m31(5);
            *row[67] = value_limb_5_col67;let value_limb_6_col68 = memory_id_to_big_value_tmp_34cc4_6.get_m31(6);
            *row[68] = value_limb_6_col68;let value_limb_7_col69 = memory_id_to_big_value_tmp_34cc4_6.get_m31(7);
            *row[69] = value_limb_7_col69;let value_limb_8_col70 = memory_id_to_big_value_tmp_34cc4_6.get_m31(8);
            *row[70] = value_limb_8_col70;let value_limb_9_col71 = memory_id_to_big_value_tmp_34cc4_6.get_m31(9);
            *row[71] = value_limb_9_col71;let value_limb_10_col72 = memory_id_to_big_value_tmp_34cc4_6.get_m31(10);
            *row[72] = value_limb_10_col72;let value_limb_11_col73 = memory_id_to_big_value_tmp_34cc4_6.get_m31(11);
            *row[73] = value_limb_11_col73;let value_limb_12_col74 = memory_id_to_big_value_tmp_34cc4_6.get_m31(12);
            *row[74] = value_limb_12_col74;let value_limb_13_col75 = memory_id_to_big_value_tmp_34cc4_6.get_m31(13);
            *row[75] = value_limb_13_col75;let value_limb_14_col76 = memory_id_to_big_value_tmp_34cc4_6.get_m31(14);
            *row[76] = value_limb_14_col76;let value_limb_15_col77 = memory_id_to_big_value_tmp_34cc4_6.get_m31(15);
            *row[77] = value_limb_15_col77;let value_limb_16_col78 = memory_id_to_big_value_tmp_34cc4_6.get_m31(16);
            *row[78] = value_limb_16_col78;let value_limb_17_col79 = memory_id_to_big_value_tmp_34cc4_6.get_m31(17);
            *row[79] = value_limb_17_col79;let value_limb_18_col80 = memory_id_to_big_value_tmp_34cc4_6.get_m31(18);
            *row[80] = value_limb_18_col80;let value_limb_19_col81 = memory_id_to_big_value_tmp_34cc4_6.get_m31(19);
            *row[81] = value_limb_19_col81;let value_limb_20_col82 = memory_id_to_big_value_tmp_34cc4_6.get_m31(20);
            *row[82] = value_limb_20_col82;let value_limb_21_col83 = memory_id_to_big_value_tmp_34cc4_6.get_m31(21);
            *row[83] = value_limb_21_col83;let value_limb_22_col84 = memory_id_to_big_value_tmp_34cc4_6.get_m31(22);
            *row[84] = value_limb_22_col84;let value_limb_23_col85 = memory_id_to_big_value_tmp_34cc4_6.get_m31(23);
            *row[85] = value_limb_23_col85;let value_limb_24_col86 = memory_id_to_big_value_tmp_34cc4_6.get_m31(24);
            *row[86] = value_limb_24_col86;let value_limb_25_col87 = memory_id_to_big_value_tmp_34cc4_6.get_m31(25);
            *row[87] = value_limb_25_col87;let value_limb_26_col88 = memory_id_to_big_value_tmp_34cc4_6.get_m31(26);
            *row[88] = value_limb_26_col88;let value_limb_27_col89 = memory_id_to_big_value_tmp_34cc4_6.get_m31(27);
            *row[89] = value_limb_27_col89;*sub_component_inputs.memory_id_to_big[2] =
                input_limb_2_col2;
            *lookup_data.memory_id_to_big_2 = [input_limb_2_col2, value_limb_0_col62, value_limb_1_col63, value_limb_2_col64, value_limb_3_col65, value_limb_4_col66, value_limb_5_col67, value_limb_6_col68, value_limb_7_col69, value_limb_8_col70, value_limb_9_col71, value_limb_10_col72, value_limb_11_col73, value_limb_12_col74, value_limb_13_col75, value_limb_14_col76, value_limb_15_col77, value_limb_16_col78, value_limb_17_col79, value_limb_18_col80, value_limb_19_col81, value_limb_20_col82, value_limb_21_col83, value_limb_22_col84, value_limb_23_col85, value_limb_24_col86, value_limb_25_col87, value_limb_26_col88, value_limb_27_col89];let read_positive_known_id_num_bits_252_output_tmp_34cc4_7 = PackedFelt252::from_limbs([value_limb_0_col62, value_limb_1_col63, value_limb_2_col64, value_limb_3_col65, value_limb_4_col66, value_limb_5_col67, value_limb_6_col68, value_limb_7_col69, value_limb_8_col70, value_limb_9_col71, value_limb_10_col72, value_limb_11_col73, value_limb_12_col74, value_limb_13_col75, value_limb_14_col76, value_limb_15_col77, value_limb_16_col78, value_limb_17_col79, value_limb_18_col80, value_limb_19_col81, value_limb_20_col82, value_limb_21_col83, value_limb_22_col84, value_limb_23_col85, value_limb_24_col86, value_limb_25_col87, value_limb_26_col88, value_limb_27_col89]);

            let packed_input_state_2_tmp_34cc4_8 = PackedFelt252Width27::from_limbs([((((value_limb_0_col62) + (((value_limb_1_col63) * (M31_512))))) + (((value_limb_2_col64) * (M31_262144)))), ((((value_limb_3_col65) + (((value_limb_4_col66) * (M31_512))))) + (((value_limb_5_col67) * (M31_262144)))), ((((value_limb_6_col68) + (((value_limb_7_col69) * (M31_512))))) + (((value_limb_8_col70) * (M31_262144)))), ((((value_limb_9_col71) + (((value_limb_10_col72) * (M31_512))))) + (((value_limb_11_col73) * (M31_262144)))), ((((value_limb_12_col74) + (((value_limb_13_col75) * (M31_512))))) + (((value_limb_14_col76) * (M31_262144)))), ((((value_limb_15_col77) + (((value_limb_16_col78) * (M31_512))))) + (((value_limb_17_col79) * (M31_262144)))), ((((value_limb_18_col80) + (((value_limb_19_col81) * (M31_512))))) + (((value_limb_20_col82) * (M31_262144)))), ((((value_limb_21_col83) + (((value_limb_22_col84) * (M31_512))))) + (((value_limb_23_col85) * (M31_262144)))), ((((value_limb_24_col86) + (((value_limb_25_col87) * (M31_512))))) + (((value_limb_26_col88) * (M31_262144)))), value_limb_27_col89]);

            // Poseidon Hades Permutation.

            // Linear Combination N 2 Coefs 1 1.

            let combination_tmp_34cc4_9 = PackedFelt252Width27::from_packed_felt252(((((Felt252_0_0_0_0) + (((Felt252_1_0_0_0) * (PackedFelt252::from_packed_felt252width27(packed_input_state_0_tmp_34cc4_2)))))) + (Felt252_9275160746813554287_16541205595039575623_4169650429605064889_470088886057789987)));let combination_limb_0_col90 = combination_tmp_34cc4_9.get_m31(0);
            *row[90] = combination_limb_0_col90;let combination_limb_1_col91 = combination_tmp_34cc4_9.get_m31(1);
            *row[91] = combination_limb_1_col91;let combination_limb_2_col92 = combination_tmp_34cc4_9.get_m31(2);
            *row[92] = combination_limb_2_col92;let combination_limb_3_col93 = combination_tmp_34cc4_9.get_m31(3);
            *row[93] = combination_limb_3_col93;let combination_limb_4_col94 = combination_tmp_34cc4_9.get_m31(4);
            *row[94] = combination_limb_4_col94;let combination_limb_5_col95 = combination_tmp_34cc4_9.get_m31(5);
            *row[95] = combination_limb_5_col95;let combination_limb_6_col96 = combination_tmp_34cc4_9.get_m31(6);
            *row[96] = combination_limb_6_col96;let combination_limb_7_col97 = combination_tmp_34cc4_9.get_m31(7);
            *row[97] = combination_limb_7_col97;let combination_limb_8_col98 = combination_tmp_34cc4_9.get_m31(8);
            *row[98] = combination_limb_8_col98;let combination_limb_9_col99 = combination_tmp_34cc4_9.get_m31(9);
            *row[99] = combination_limb_9_col99;let biased_limb_accumulator_u32_tmp_34cc4_10 = PackedUInt32::from_m31(((((((packed_input_state_0_tmp_34cc4_2.get_m31(0)) + (M31_74972783))) - (combination_limb_0_col90))) + (M31_134217729)));let p_coef_col100 = ((biased_limb_accumulator_u32_tmp_34cc4_10.low().as_m31()) - (M31_1));
            *row[100] = p_coef_col100;let carry_0_tmp_34cc4_11 = ((((((((packed_input_state_0_tmp_34cc4_2.get_m31(0)) + (M31_74972783))) - (combination_limb_0_col90))) - (p_coef_col100))) * (M31_16));let carry_1_tmp_34cc4_12 = ((((((((carry_0_tmp_34cc4_11) + (packed_input_state_0_tmp_34cc4_2.get_m31(1)))) + (M31_117420501))) - (combination_limb_1_col91))) * (M31_16));let carry_2_tmp_34cc4_13 = ((((((((carry_1_tmp_34cc4_12) + (packed_input_state_0_tmp_34cc4_2.get_m31(2)))) + (M31_112795138))) - (combination_limb_2_col92))) * (M31_16));let carry_3_tmp_34cc4_14 = ((((((((carry_2_tmp_34cc4_13) + (packed_input_state_0_tmp_34cc4_2.get_m31(3)))) + (M31_91013252))) - (combination_limb_3_col93))) * (M31_16));let carry_4_tmp_34cc4_15 = ((((((((carry_3_tmp_34cc4_14) + (packed_input_state_0_tmp_34cc4_2.get_m31(4)))) + (M31_60709090))) - (combination_limb_4_col94))) * (M31_16));let carry_5_tmp_34cc4_16 = ((((((((carry_4_tmp_34cc4_15) + (packed_input_state_0_tmp_34cc4_2.get_m31(5)))) + (M31_44848225))) - (combination_limb_5_col95))) * (M31_16));let carry_6_tmp_34cc4_17 = ((((((((carry_5_tmp_34cc4_16) + (packed_input_state_0_tmp_34cc4_2.get_m31(6)))) + (M31_108487870))) - (combination_limb_6_col96))) * (M31_16));let carry_7_tmp_34cc4_18 = ((((((((((carry_6_tmp_34cc4_17) + (packed_input_state_0_tmp_34cc4_2.get_m31(7)))) + (M31_44781849))) - (combination_limb_7_col97))) - (((p_coef_col100) * (M31_136))))) * (M31_16));let carry_8_tmp_34cc4_19 = ((((((((carry_7_tmp_34cc4_18) + (packed_input_state_0_tmp_34cc4_2.get_m31(8)))) + (M31_102193642))) - (combination_limb_8_col98))) * (M31_16));let linear_combination_n_2_coefs_1_1_output_tmp_34cc4_29 = combination_tmp_34cc4_9;

            // Linear Combination N 2 Coefs 1 1.

            let combination_tmp_34cc4_30 = PackedFelt252Width27::from_packed_felt252(((((Felt252_0_0_0_0) + (((Felt252_1_0_0_0) * (PackedFelt252::from_packed_felt252width27(packed_input_state_1_tmp_34cc4_5)))))) + (Felt252_16477292399064058052_4441744911417641572_18431044672185975386_252894828082060025)));let combination_limb_0_col101 = combination_tmp_34cc4_30.get_m31(0);
            *row[101] = combination_limb_0_col101;let combination_limb_1_col102 = combination_tmp_34cc4_30.get_m31(1);
            *row[102] = combination_limb_1_col102;let combination_limb_2_col103 = combination_tmp_34cc4_30.get_m31(2);
            *row[103] = combination_limb_2_col103;let combination_limb_3_col104 = combination_tmp_34cc4_30.get_m31(3);
            *row[104] = combination_limb_3_col104;let combination_limb_4_col105 = combination_tmp_34cc4_30.get_m31(4);
            *row[105] = combination_limb_4_col105;let combination_limb_5_col106 = combination_tmp_34cc4_30.get_m31(5);
            *row[106] = combination_limb_5_col106;let combination_limb_6_col107 = combination_tmp_34cc4_30.get_m31(6);
            *row[107] = combination_limb_6_col107;let combination_limb_7_col108 = combination_tmp_34cc4_30.get_m31(7);
            *row[108] = combination_limb_7_col108;let combination_limb_8_col109 = combination_tmp_34cc4_30.get_m31(8);
            *row[109] = combination_limb_8_col109;let combination_limb_9_col110 = combination_tmp_34cc4_30.get_m31(9);
            *row[110] = combination_limb_9_col110;let biased_limb_accumulator_u32_tmp_34cc4_31 = PackedUInt32::from_m31(((((((packed_input_state_1_tmp_34cc4_5.get_m31(0)) + (M31_41224388))) - (combination_limb_0_col101))) + (M31_134217729)));let p_coef_col111 = ((biased_limb_accumulator_u32_tmp_34cc4_31.low().as_m31()) - (M31_1));
            *row[111] = p_coef_col111;let carry_0_tmp_34cc4_32 = ((((((((packed_input_state_1_tmp_34cc4_5.get_m31(0)) + (M31_41224388))) - (combination_limb_0_col101))) - (p_coef_col111))) * (M31_16));let carry_1_tmp_34cc4_33 = ((((((((carry_0_tmp_34cc4_32) + (packed_input_state_1_tmp_34cc4_5.get_m31(1)))) + (M31_90391646))) - (combination_limb_1_col102))) * (M31_16));let carry_2_tmp_34cc4_34 = ((((((((carry_1_tmp_34cc4_33) + (packed_input_state_1_tmp_34cc4_5.get_m31(2)))) + (M31_36279186))) - (combination_limb_2_col103))) * (M31_16));let carry_3_tmp_34cc4_35 = ((((((((carry_2_tmp_34cc4_34) + (packed_input_state_1_tmp_34cc4_5.get_m31(3)))) + (M31_129717753))) - (combination_limb_3_col104))) * (M31_16));let carry_4_tmp_34cc4_36 = ((((((((carry_3_tmp_34cc4_35) + (packed_input_state_1_tmp_34cc4_5.get_m31(4)))) + (M31_94624323))) - (combination_limb_4_col105))) * (M31_16));let carry_5_tmp_34cc4_37 = ((((((((carry_4_tmp_34cc4_36) + (packed_input_state_1_tmp_34cc4_5.get_m31(5)))) + (M31_75104388))) - (combination_limb_5_col106))) * (M31_16));let carry_6_tmp_34cc4_38 = ((((((((carry_5_tmp_34cc4_37) + (packed_input_state_1_tmp_34cc4_5.get_m31(6)))) + (M31_133303902))) - (combination_limb_6_col107))) * (M31_16));let carry_7_tmp_34cc4_39 = ((((((((((carry_6_tmp_34cc4_38) + (packed_input_state_1_tmp_34cc4_5.get_m31(7)))) + (M31_48945103))) - (combination_limb_7_col108))) - (((p_coef_col111) * (M31_136))))) * (M31_16));let carry_8_tmp_34cc4_40 = ((((((((carry_7_tmp_34cc4_39) + (packed_input_state_1_tmp_34cc4_5.get_m31(8)))) + (M31_41320857))) - (combination_limb_8_col109))) * (M31_16));let linear_combination_n_2_coefs_1_1_output_tmp_34cc4_50 = combination_tmp_34cc4_30;

            // Linear Combination N 2 Coefs 1 1.

            let combination_tmp_34cc4_51 = PackedFelt252Width27::from_packed_felt252(((((Felt252_0_0_0_0) + (((Felt252_1_0_0_0) * (PackedFelt252::from_packed_felt252width27(packed_input_state_2_tmp_34cc4_8)))))) + (Felt252_8794894655201903369_3219077422080798056_16714934791572408267_262217499501479120)));let combination_limb_0_col112 = combination_tmp_34cc4_51.get_m31(0);
            *row[112] = combination_limb_0_col112;let combination_limb_1_col113 = combination_tmp_34cc4_51.get_m31(1);
            *row[113] = combination_limb_1_col113;let combination_limb_2_col114 = combination_tmp_34cc4_51.get_m31(2);
            *row[114] = combination_limb_2_col114;let combination_limb_3_col115 = combination_tmp_34cc4_51.get_m31(3);
            *row[115] = combination_limb_3_col115;let combination_limb_4_col116 = combination_tmp_34cc4_51.get_m31(4);
            *row[116] = combination_limb_4_col116;let combination_limb_5_col117 = combination_tmp_34cc4_51.get_m31(5);
            *row[117] = combination_limb_5_col117;let combination_limb_6_col118 = combination_tmp_34cc4_51.get_m31(6);
            *row[118] = combination_limb_6_col118;let combination_limb_7_col119 = combination_tmp_34cc4_51.get_m31(7);
            *row[119] = combination_limb_7_col119;let combination_limb_8_col120 = combination_tmp_34cc4_51.get_m31(8);
            *row[120] = combination_limb_8_col120;let combination_limb_9_col121 = combination_tmp_34cc4_51.get_m31(9);
            *row[121] = combination_limb_9_col121;let biased_limb_accumulator_u32_tmp_34cc4_52 = PackedUInt32::from_m31(((((((packed_input_state_2_tmp_34cc4_8.get_m31(0)) + (M31_4883209))) - (combination_limb_0_col112))) + (M31_134217729)));let p_coef_col122 = ((biased_limb_accumulator_u32_tmp_34cc4_52.low().as_m31()) - (M31_1));
            *row[122] = p_coef_col122;let carry_0_tmp_34cc4_53 = ((((((((packed_input_state_2_tmp_34cc4_8.get_m31(0)) + (M31_4883209))) - (combination_limb_0_col112))) - (p_coef_col122))) * (M31_16));let carry_1_tmp_34cc4_54 = ((((((((carry_0_tmp_34cc4_53) + (packed_input_state_2_tmp_34cc4_8.get_m31(1)))) + (M31_28820206))) - (combination_limb_1_col113))) * (M31_16));let carry_2_tmp_34cc4_55 = ((((((((carry_1_tmp_34cc4_54) + (packed_input_state_2_tmp_34cc4_8.get_m31(2)))) + (M31_79012328))) - (combination_limb_2_col114))) * (M31_16));let carry_3_tmp_34cc4_56 = ((((((((carry_2_tmp_34cc4_55) + (packed_input_state_2_tmp_34cc4_8.get_m31(3)))) + (M31_49157069))) - (combination_limb_3_col115))) * (M31_16));let carry_4_tmp_34cc4_57 = ((((((((carry_3_tmp_34cc4_56) + (packed_input_state_2_tmp_34cc4_8.get_m31(4)))) + (M31_78826183))) - (combination_limb_4_col116))) * (M31_16));let carry_5_tmp_34cc4_58 = ((((((((carry_4_tmp_34cc4_57) + (packed_input_state_2_tmp_34cc4_8.get_m31(5)))) + (M31_72285071))) - (combination_limb_5_col117))) * (M31_16));let carry_6_tmp_34cc4_59 = ((((((((carry_5_tmp_34cc4_58) + (packed_input_state_2_tmp_34cc4_8.get_m31(6)))) + (M31_33413160))) - (combination_limb_6_col118))) * (M31_16));let carry_7_tmp_34cc4_60 = ((((((((((carry_6_tmp_34cc4_59) + (packed_input_state_2_tmp_34cc4_8.get_m31(7)))) + (M31_90842759))) - (combination_limb_7_col119))) - (((p_coef_col122) * (M31_136))))) * (M31_16));let carry_8_tmp_34cc4_61 = ((((((((carry_7_tmp_34cc4_60) + (packed_input_state_2_tmp_34cc4_8.get_m31(8)))) + (M31_60124463))) - (combination_limb_8_col120))) * (M31_16));let linear_combination_n_2_coefs_1_1_output_tmp_34cc4_71 = combination_tmp_34cc4_51;

            let poseidon_full_round_chain_chain_tmp_tmp_34cc4_72 = ((seq) * (M31_2));*lookup_data.poseidon_full_round_chain_0 = [poseidon_full_round_chain_chain_tmp_tmp_34cc4_72, M31_0, combination_limb_0_col90, combination_limb_1_col91, combination_limb_2_col92, combination_limb_3_col93, combination_limb_4_col94, combination_limb_5_col95, combination_limb_6_col96, combination_limb_7_col97, combination_limb_8_col98, combination_limb_9_col99, combination_limb_0_col101, combination_limb_1_col102, combination_limb_2_col103, combination_limb_3_col104, combination_limb_4_col105, combination_limb_5_col106, combination_limb_6_col107, combination_limb_7_col108, combination_limb_8_col109, combination_limb_9_col110, combination_limb_0_col112, combination_limb_1_col113, combination_limb_2_col114, combination_limb_3_col115, combination_limb_4_col116, combination_limb_5_col117, combination_limb_6_col118, combination_limb_7_col119, combination_limb_8_col120, combination_limb_9_col121];*sub_component_inputs.poseidon_full_round_chain[0] =
                (poseidon_full_round_chain_chain_tmp_tmp_34cc4_72, M31_0, [linear_combination_n_2_coefs_1_1_output_tmp_34cc4_29, linear_combination_n_2_coefs_1_1_output_tmp_34cc4_50, linear_combination_n_2_coefs_1_1_output_tmp_34cc4_71]);
            let poseidon_full_round_chain_output_round_0_tmp_34cc4_73 = PackedPoseidonFullRoundChain::deduce_output((poseidon_full_round_chain_chain_tmp_tmp_34cc4_72, M31_0, [linear_combination_n_2_coefs_1_1_output_tmp_34cc4_29, linear_combination_n_2_coefs_1_1_output_tmp_34cc4_50, linear_combination_n_2_coefs_1_1_output_tmp_34cc4_71]));*sub_component_inputs.poseidon_full_round_chain[1] =
                (poseidon_full_round_chain_chain_tmp_tmp_34cc4_72, M31_1, [poseidon_full_round_chain_output_round_0_tmp_34cc4_73.2[0], poseidon_full_round_chain_output_round_0_tmp_34cc4_73.2[1], poseidon_full_round_chain_output_round_0_tmp_34cc4_73.2[2]]);
            let poseidon_full_round_chain_output_round_1_tmp_34cc4_74 = PackedPoseidonFullRoundChain::deduce_output((poseidon_full_round_chain_chain_tmp_tmp_34cc4_72, M31_1, [poseidon_full_round_chain_output_round_0_tmp_34cc4_73.2[0], poseidon_full_round_chain_output_round_0_tmp_34cc4_73.2[1], poseidon_full_round_chain_output_round_0_tmp_34cc4_73.2[2]]));*sub_component_inputs.poseidon_full_round_chain[2] =
                (poseidon_full_round_chain_chain_tmp_tmp_34cc4_72, M31_2, [poseidon_full_round_chain_output_round_1_tmp_34cc4_74.2[0], poseidon_full_round_chain_output_round_1_tmp_34cc4_74.2[1], poseidon_full_round_chain_output_round_1_tmp_34cc4_74.2[2]]);
            let poseidon_full_round_chain_output_round_2_tmp_34cc4_75 = PackedPoseidonFullRoundChain::deduce_output((poseidon_full_round_chain_chain_tmp_tmp_34cc4_72, M31_2, [poseidon_full_round_chain_output_round_1_tmp_34cc4_74.2[0], poseidon_full_round_chain_output_round_1_tmp_34cc4_74.2[1], poseidon_full_round_chain_output_round_1_tmp_34cc4_74.2[2]]));*sub_component_inputs.poseidon_full_round_chain[3] =
                (poseidon_full_round_chain_chain_tmp_tmp_34cc4_72, M31_3, [poseidon_full_round_chain_output_round_2_tmp_34cc4_75.2[0], poseidon_full_round_chain_output_round_2_tmp_34cc4_75.2[1], poseidon_full_round_chain_output_round_2_tmp_34cc4_75.2[2]]);
            let poseidon_full_round_chain_output_round_3_tmp_34cc4_76 = PackedPoseidonFullRoundChain::deduce_output((poseidon_full_round_chain_chain_tmp_tmp_34cc4_72, M31_3, [poseidon_full_round_chain_output_round_2_tmp_34cc4_75.2[0], poseidon_full_round_chain_output_round_2_tmp_34cc4_75.2[1], poseidon_full_round_chain_output_round_2_tmp_34cc4_75.2[2]]));let poseidon_full_round_chain_output_limb_0_col123 = poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[0].get_m31(0);
            *row[123] = poseidon_full_round_chain_output_limb_0_col123;let poseidon_full_round_chain_output_limb_1_col124 = poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[0].get_m31(1);
            *row[124] = poseidon_full_round_chain_output_limb_1_col124;let poseidon_full_round_chain_output_limb_2_col125 = poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[0].get_m31(2);
            *row[125] = poseidon_full_round_chain_output_limb_2_col125;let poseidon_full_round_chain_output_limb_3_col126 = poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[0].get_m31(3);
            *row[126] = poseidon_full_round_chain_output_limb_3_col126;let poseidon_full_round_chain_output_limb_4_col127 = poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[0].get_m31(4);
            *row[127] = poseidon_full_round_chain_output_limb_4_col127;let poseidon_full_round_chain_output_limb_5_col128 = poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[0].get_m31(5);
            *row[128] = poseidon_full_round_chain_output_limb_5_col128;let poseidon_full_round_chain_output_limb_6_col129 = poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[0].get_m31(6);
            *row[129] = poseidon_full_round_chain_output_limb_6_col129;let poseidon_full_round_chain_output_limb_7_col130 = poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[0].get_m31(7);
            *row[130] = poseidon_full_round_chain_output_limb_7_col130;let poseidon_full_round_chain_output_limb_8_col131 = poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[0].get_m31(8);
            *row[131] = poseidon_full_round_chain_output_limb_8_col131;let poseidon_full_round_chain_output_limb_9_col132 = poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[0].get_m31(9);
            *row[132] = poseidon_full_round_chain_output_limb_9_col132;let poseidon_full_round_chain_output_limb_10_col133 = poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[1].get_m31(0);
            *row[133] = poseidon_full_round_chain_output_limb_10_col133;let poseidon_full_round_chain_output_limb_11_col134 = poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[1].get_m31(1);
            *row[134] = poseidon_full_round_chain_output_limb_11_col134;let poseidon_full_round_chain_output_limb_12_col135 = poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[1].get_m31(2);
            *row[135] = poseidon_full_round_chain_output_limb_12_col135;let poseidon_full_round_chain_output_limb_13_col136 = poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[1].get_m31(3);
            *row[136] = poseidon_full_round_chain_output_limb_13_col136;let poseidon_full_round_chain_output_limb_14_col137 = poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[1].get_m31(4);
            *row[137] = poseidon_full_round_chain_output_limb_14_col137;let poseidon_full_round_chain_output_limb_15_col138 = poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[1].get_m31(5);
            *row[138] = poseidon_full_round_chain_output_limb_15_col138;let poseidon_full_round_chain_output_limb_16_col139 = poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[1].get_m31(6);
            *row[139] = poseidon_full_round_chain_output_limb_16_col139;let poseidon_full_round_chain_output_limb_17_col140 = poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[1].get_m31(7);
            *row[140] = poseidon_full_round_chain_output_limb_17_col140;let poseidon_full_round_chain_output_limb_18_col141 = poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[1].get_m31(8);
            *row[141] = poseidon_full_round_chain_output_limb_18_col141;let poseidon_full_round_chain_output_limb_19_col142 = poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[1].get_m31(9);
            *row[142] = poseidon_full_round_chain_output_limb_19_col142;let poseidon_full_round_chain_output_limb_20_col143 = poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[2].get_m31(0);
            *row[143] = poseidon_full_round_chain_output_limb_20_col143;let poseidon_full_round_chain_output_limb_21_col144 = poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[2].get_m31(1);
            *row[144] = poseidon_full_round_chain_output_limb_21_col144;let poseidon_full_round_chain_output_limb_22_col145 = poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[2].get_m31(2);
            *row[145] = poseidon_full_round_chain_output_limb_22_col145;let poseidon_full_round_chain_output_limb_23_col146 = poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[2].get_m31(3);
            *row[146] = poseidon_full_round_chain_output_limb_23_col146;let poseidon_full_round_chain_output_limb_24_col147 = poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[2].get_m31(4);
            *row[147] = poseidon_full_round_chain_output_limb_24_col147;let poseidon_full_round_chain_output_limb_25_col148 = poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[2].get_m31(5);
            *row[148] = poseidon_full_round_chain_output_limb_25_col148;let poseidon_full_round_chain_output_limb_26_col149 = poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[2].get_m31(6);
            *row[149] = poseidon_full_round_chain_output_limb_26_col149;let poseidon_full_round_chain_output_limb_27_col150 = poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[2].get_m31(7);
            *row[150] = poseidon_full_round_chain_output_limb_27_col150;let poseidon_full_round_chain_output_limb_28_col151 = poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[2].get_m31(8);
            *row[151] = poseidon_full_round_chain_output_limb_28_col151;let poseidon_full_round_chain_output_limb_29_col152 = poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[2].get_m31(9);
            *row[152] = poseidon_full_round_chain_output_limb_29_col152;*lookup_data.poseidon_full_round_chain_1 = [poseidon_full_round_chain_chain_tmp_tmp_34cc4_72, M31_4, poseidon_full_round_chain_output_limb_0_col123, poseidon_full_round_chain_output_limb_1_col124, poseidon_full_round_chain_output_limb_2_col125, poseidon_full_round_chain_output_limb_3_col126, poseidon_full_round_chain_output_limb_4_col127, poseidon_full_round_chain_output_limb_5_col128, poseidon_full_round_chain_output_limb_6_col129, poseidon_full_round_chain_output_limb_7_col130, poseidon_full_round_chain_output_limb_8_col131, poseidon_full_round_chain_output_limb_9_col132, poseidon_full_round_chain_output_limb_10_col133, poseidon_full_round_chain_output_limb_11_col134, poseidon_full_round_chain_output_limb_12_col135, poseidon_full_round_chain_output_limb_13_col136, poseidon_full_round_chain_output_limb_14_col137, poseidon_full_round_chain_output_limb_15_col138, poseidon_full_round_chain_output_limb_16_col139, poseidon_full_round_chain_output_limb_17_col140, poseidon_full_round_chain_output_limb_18_col141, poseidon_full_round_chain_output_limb_19_col142, poseidon_full_round_chain_output_limb_20_col143, poseidon_full_round_chain_output_limb_21_col144, poseidon_full_round_chain_output_limb_22_col145, poseidon_full_round_chain_output_limb_23_col146, poseidon_full_round_chain_output_limb_24_col147, poseidon_full_round_chain_output_limb_25_col148, poseidon_full_round_chain_output_limb_26_col149, poseidon_full_round_chain_output_limb_27_col150, poseidon_full_round_chain_output_limb_28_col151, poseidon_full_round_chain_output_limb_29_col152];*sub_component_inputs.range_check_252_width_27[0] =
                poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[0];
            *lookup_data.range_check_252_width_27_0 = [poseidon_full_round_chain_output_limb_0_col123, poseidon_full_round_chain_output_limb_1_col124, poseidon_full_round_chain_output_limb_2_col125, poseidon_full_round_chain_output_limb_3_col126, poseidon_full_round_chain_output_limb_4_col127, poseidon_full_round_chain_output_limb_5_col128, poseidon_full_round_chain_output_limb_6_col129, poseidon_full_round_chain_output_limb_7_col130, poseidon_full_round_chain_output_limb_8_col131, poseidon_full_round_chain_output_limb_9_col132];*sub_component_inputs.range_check_252_width_27[1] =
                poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[1];
            *lookup_data.range_check_252_width_27_1 = [poseidon_full_round_chain_output_limb_10_col133, poseidon_full_round_chain_output_limb_11_col134, poseidon_full_round_chain_output_limb_12_col135, poseidon_full_round_chain_output_limb_13_col136, poseidon_full_round_chain_output_limb_14_col137, poseidon_full_round_chain_output_limb_15_col138, poseidon_full_round_chain_output_limb_16_col139, poseidon_full_round_chain_output_limb_17_col140, poseidon_full_round_chain_output_limb_18_col141, poseidon_full_round_chain_output_limb_19_col142];*sub_component_inputs.cube_252[0] =
                poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[2];
            let cube_252_output_tmp_34cc4_77 = PackedCube252::deduce_output(poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[2]);let cube_252_output_limb_0_col153 = cube_252_output_tmp_34cc4_77.get_m31(0);
            *row[153] = cube_252_output_limb_0_col153;let cube_252_output_limb_1_col154 = cube_252_output_tmp_34cc4_77.get_m31(1);
            *row[154] = cube_252_output_limb_1_col154;let cube_252_output_limb_2_col155 = cube_252_output_tmp_34cc4_77.get_m31(2);
            *row[155] = cube_252_output_limb_2_col155;let cube_252_output_limb_3_col156 = cube_252_output_tmp_34cc4_77.get_m31(3);
            *row[156] = cube_252_output_limb_3_col156;let cube_252_output_limb_4_col157 = cube_252_output_tmp_34cc4_77.get_m31(4);
            *row[157] = cube_252_output_limb_4_col157;let cube_252_output_limb_5_col158 = cube_252_output_tmp_34cc4_77.get_m31(5);
            *row[158] = cube_252_output_limb_5_col158;let cube_252_output_limb_6_col159 = cube_252_output_tmp_34cc4_77.get_m31(6);
            *row[159] = cube_252_output_limb_6_col159;let cube_252_output_limb_7_col160 = cube_252_output_tmp_34cc4_77.get_m31(7);
            *row[160] = cube_252_output_limb_7_col160;let cube_252_output_limb_8_col161 = cube_252_output_tmp_34cc4_77.get_m31(8);
            *row[161] = cube_252_output_limb_8_col161;let cube_252_output_limb_9_col162 = cube_252_output_tmp_34cc4_77.get_m31(9);
            *row[162] = cube_252_output_limb_9_col162;*lookup_data.cube_252_0 = [poseidon_full_round_chain_output_limb_20_col143, poseidon_full_round_chain_output_limb_21_col144, poseidon_full_round_chain_output_limb_22_col145, poseidon_full_round_chain_output_limb_23_col146, poseidon_full_round_chain_output_limb_24_col147, poseidon_full_round_chain_output_limb_25_col148, poseidon_full_round_chain_output_limb_26_col149, poseidon_full_round_chain_output_limb_27_col150, poseidon_full_round_chain_output_limb_28_col151, poseidon_full_round_chain_output_limb_29_col152, cube_252_output_limb_0_col153, cube_252_output_limb_1_col154, cube_252_output_limb_2_col155, cube_252_output_limb_3_col156, cube_252_output_limb_4_col157, cube_252_output_limb_5_col158, cube_252_output_limb_6_col159, cube_252_output_limb_7_col160, cube_252_output_limb_8_col161, cube_252_output_limb_9_col162];

            // Linear Combination N 4 Coefs 1 1 M 2 1.

            let combination_tmp_34cc4_78 = PackedFelt252Width27::from_packed_felt252(((((((((Felt252_0_0_0_0) + (((Felt252_1_0_0_0) * (PackedFelt252::from_packed_felt252width27(poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[0])))))) + (((Felt252_1_0_0_0) * (PackedFelt252::from_packed_felt252width27(poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[1])))))) - (((Felt252_2_0_0_0) * (PackedFelt252::from_packed_felt252width27(cube_252_output_tmp_34cc4_77)))))) + (Felt252_11041071929982523380_7503192613203831446_4943121247101232560_560497091765764140)));let combination_limb_0_col163 = combination_tmp_34cc4_78.get_m31(0);
            *row[163] = combination_limb_0_col163;let combination_limb_1_col164 = combination_tmp_34cc4_78.get_m31(1);
            *row[164] = combination_limb_1_col164;let combination_limb_2_col165 = combination_tmp_34cc4_78.get_m31(2);
            *row[165] = combination_limb_2_col165;let combination_limb_3_col166 = combination_tmp_34cc4_78.get_m31(3);
            *row[166] = combination_limb_3_col166;let combination_limb_4_col167 = combination_tmp_34cc4_78.get_m31(4);
            *row[167] = combination_limb_4_col167;let combination_limb_5_col168 = combination_tmp_34cc4_78.get_m31(5);
            *row[168] = combination_limb_5_col168;let combination_limb_6_col169 = combination_tmp_34cc4_78.get_m31(6);
            *row[169] = combination_limb_6_col169;let combination_limb_7_col170 = combination_tmp_34cc4_78.get_m31(7);
            *row[170] = combination_limb_7_col170;let combination_limb_8_col171 = combination_tmp_34cc4_78.get_m31(8);
            *row[171] = combination_limb_8_col171;let combination_limb_9_col172 = combination_tmp_34cc4_78.get_m31(9);
            *row[172] = combination_limb_9_col172;let biased_limb_accumulator_u32_tmp_34cc4_79 = PackedUInt32::from_m31(((((((((((poseidon_full_round_chain_output_limb_0_col123) + (poseidon_full_round_chain_output_limb_10_col133))) - (((M31_2) * (cube_252_output_limb_0_col153))))) + (M31_103094260))) - (combination_limb_0_col163))) + (M31_402653187)));let p_coef_col173 = ((biased_limb_accumulator_u32_tmp_34cc4_79.low().as_m31()) - (M31_3));
            *row[173] = p_coef_col173;let carry_0_tmp_34cc4_80 = ((((((((((((poseidon_full_round_chain_output_limb_0_col123) + (poseidon_full_round_chain_output_limb_10_col133))) - (((M31_2) * (cube_252_output_limb_0_col153))))) + (M31_103094260))) - (combination_limb_0_col163))) - (p_coef_col173))) * (M31_16));let carry_1_tmp_34cc4_81 = ((((((((((((carry_0_tmp_34cc4_80) + (poseidon_full_round_chain_output_limb_1_col124))) + (poseidon_full_round_chain_output_limb_11_col134))) - (((M31_2) * (cube_252_output_limb_1_col154))))) + (M31_121146754))) - (combination_limb_1_col164))) * (M31_16));let carry_2_tmp_34cc4_82 = ((((((((((((carry_1_tmp_34cc4_81) + (poseidon_full_round_chain_output_limb_2_col125))) + (poseidon_full_round_chain_output_limb_12_col135))) - (((M31_2) * (cube_252_output_limb_2_col155))))) + (M31_95050340))) - (combination_limb_2_col165))) * (M31_16));let carry_3_tmp_34cc4_83 = ((((((((((((carry_2_tmp_34cc4_82) + (poseidon_full_round_chain_output_limb_3_col126))) + (poseidon_full_round_chain_output_limb_13_col136))) - (((M31_2) * (cube_252_output_limb_3_col156))))) + (M31_16173996))) - (combination_limb_3_col166))) * (M31_16));let carry_4_tmp_34cc4_84 = ((((((((((((carry_3_tmp_34cc4_83) + (poseidon_full_round_chain_output_limb_4_col127))) + (poseidon_full_round_chain_output_limb_14_col137))) - (((M31_2) * (cube_252_output_limb_4_col157))))) + (M31_50758155))) - (combination_limb_4_col167))) * (M31_16));let carry_5_tmp_34cc4_85 = ((((((((((((carry_4_tmp_34cc4_84) + (poseidon_full_round_chain_output_limb_5_col128))) + (poseidon_full_round_chain_output_limb_15_col138))) - (((M31_2) * (cube_252_output_limb_5_col158))))) + (M31_54415179))) - (combination_limb_5_col168))) * (M31_16));let carry_6_tmp_34cc4_86 = ((((((((((((carry_5_tmp_34cc4_85) + (poseidon_full_round_chain_output_limb_6_col129))) + (poseidon_full_round_chain_output_limb_16_col139))) - (((M31_2) * (cube_252_output_limb_6_col159))))) + (M31_19292069))) - (combination_limb_6_col169))) * (M31_16));let carry_7_tmp_34cc4_87 = ((((((((((((((carry_6_tmp_34cc4_86) + (poseidon_full_round_chain_output_limb_7_col130))) + (poseidon_full_round_chain_output_limb_17_col140))) - (((M31_2) * (cube_252_output_limb_7_col160))))) + (M31_45351266))) - (combination_limb_7_col170))) - (((p_coef_col173) * (M31_136))))) * (M31_16));let carry_8_tmp_34cc4_88 = ((((((((((((carry_7_tmp_34cc4_87) + (poseidon_full_round_chain_output_limb_8_col131))) + (poseidon_full_round_chain_output_limb_18_col141))) - (((M31_2) * (cube_252_output_limb_8_col161))))) + (M31_122233508))) - (combination_limb_8_col171))) * (M31_16));*sub_component_inputs.range_check_3_3_3_3_3[0] =
                [((p_coef_col173) + (M31_3)), ((carry_0_tmp_34cc4_80) + (M31_3)), ((carry_1_tmp_34cc4_81) + (M31_3)), ((carry_2_tmp_34cc4_82) + (M31_3)), ((carry_3_tmp_34cc4_83) + (M31_3))];
            *lookup_data.range_check_3_3_3_3_3_0 = [((p_coef_col173) + (M31_3)), ((carry_0_tmp_34cc4_80) + (M31_3)), ((carry_1_tmp_34cc4_81) + (M31_3)), ((carry_2_tmp_34cc4_82) + (M31_3)), ((carry_3_tmp_34cc4_83) + (M31_3))];*sub_component_inputs.range_check_3_3_3_3_3[1] =
                [((carry_4_tmp_34cc4_84) + (M31_3)), ((carry_5_tmp_34cc4_85) + (M31_3)), ((carry_6_tmp_34cc4_86) + (M31_3)), ((carry_7_tmp_34cc4_87) + (M31_3)), ((carry_8_tmp_34cc4_88) + (M31_3))];
            *lookup_data.range_check_3_3_3_3_3_1 = [((carry_4_tmp_34cc4_84) + (M31_3)), ((carry_5_tmp_34cc4_85) + (M31_3)), ((carry_6_tmp_34cc4_86) + (M31_3)), ((carry_7_tmp_34cc4_87) + (M31_3)), ((carry_8_tmp_34cc4_88) + (M31_3))];let linear_combination_n_4_coefs_1_1_m2_1_output_tmp_34cc4_89 = combination_tmp_34cc4_78;

            *sub_component_inputs.cube_252[1] =
                linear_combination_n_4_coefs_1_1_m2_1_output_tmp_34cc4_89;
            let cube_252_output_tmp_34cc4_90 = PackedCube252::deduce_output(linear_combination_n_4_coefs_1_1_m2_1_output_tmp_34cc4_89);let cube_252_output_limb_0_col174 = cube_252_output_tmp_34cc4_90.get_m31(0);
            *row[174] = cube_252_output_limb_0_col174;let cube_252_output_limb_1_col175 = cube_252_output_tmp_34cc4_90.get_m31(1);
            *row[175] = cube_252_output_limb_1_col175;let cube_252_output_limb_2_col176 = cube_252_output_tmp_34cc4_90.get_m31(2);
            *row[176] = cube_252_output_limb_2_col176;let cube_252_output_limb_3_col177 = cube_252_output_tmp_34cc4_90.get_m31(3);
            *row[177] = cube_252_output_limb_3_col177;let cube_252_output_limb_4_col178 = cube_252_output_tmp_34cc4_90.get_m31(4);
            *row[178] = cube_252_output_limb_4_col178;let cube_252_output_limb_5_col179 = cube_252_output_tmp_34cc4_90.get_m31(5);
            *row[179] = cube_252_output_limb_5_col179;let cube_252_output_limb_6_col180 = cube_252_output_tmp_34cc4_90.get_m31(6);
            *row[180] = cube_252_output_limb_6_col180;let cube_252_output_limb_7_col181 = cube_252_output_tmp_34cc4_90.get_m31(7);
            *row[181] = cube_252_output_limb_7_col181;let cube_252_output_limb_8_col182 = cube_252_output_tmp_34cc4_90.get_m31(8);
            *row[182] = cube_252_output_limb_8_col182;let cube_252_output_limb_9_col183 = cube_252_output_tmp_34cc4_90.get_m31(9);
            *row[183] = cube_252_output_limb_9_col183;*lookup_data.cube_252_1 = [combination_limb_0_col163, combination_limb_1_col164, combination_limb_2_col165, combination_limb_3_col166, combination_limb_4_col167, combination_limb_5_col168, combination_limb_6_col169, combination_limb_7_col170, combination_limb_8_col171, combination_limb_9_col172, cube_252_output_limb_0_col174, cube_252_output_limb_1_col175, cube_252_output_limb_2_col176, cube_252_output_limb_3_col177, cube_252_output_limb_4_col178, cube_252_output_limb_5_col179, cube_252_output_limb_6_col180, cube_252_output_limb_7_col181, cube_252_output_limb_8_col182, cube_252_output_limb_9_col183];

            // Linear Combination N 4 Coefs 4 2 M 2 1.

            let combination_tmp_34cc4_91 = PackedFelt252Width27::from_packed_felt252(((((((((Felt252_0_0_0_0) + (((Felt252_4_0_0_0) * (PackedFelt252::from_packed_felt252width27(poseidon_full_round_chain_output_round_3_tmp_34cc4_76.2[0])))))) + (((Felt252_2_0_0_0) * (PackedFelt252::from_packed_felt252width27(cube_252_output_tmp_34cc4_77)))))) - (((Felt252_2_0_0_0) * (PackedFelt252::from_packed_felt252width27(cube_252_output_tmp_34cc4_90)))))) + (Felt252_10931822301410252833_1475756362763989377_3378552166684303673_348229636055909092)));let combination_limb_0_col184 = combination_tmp_34cc4_91.get_m31(0);
            *row[184] = combination_limb_0_col184;let combination_limb_1_col185 = combination_tmp_34cc4_91.get_m31(1);
            *row[185] = combination_limb_1_col185;let combination_limb_2_col186 = combination_tmp_34cc4_91.get_m31(2);
            *row[186] = combination_limb_2_col186;let combination_limb_3_col187 = combination_tmp_34cc4_91.get_m31(3);
            *row[187] = combination_limb_3_col187;let combination_limb_4_col188 = combination_tmp_34cc4_91.get_m31(4);
            *row[188] = combination_limb_4_col188;let combination_limb_5_col189 = combination_tmp_34cc4_91.get_m31(5);
            *row[189] = combination_limb_5_col189;let combination_limb_6_col190 = combination_tmp_34cc4_91.get_m31(6);
            *row[190] = combination_limb_6_col190;let combination_limb_7_col191 = combination_tmp_34cc4_91.get_m31(7);
            *row[191] = combination_limb_7_col191;let combination_limb_8_col192 = combination_tmp_34cc4_91.get_m31(8);
            *row[192] = combination_limb_8_col192;let combination_limb_9_col193 = combination_tmp_34cc4_91.get_m31(9);
            *row[193] = combination_limb_9_col193;let biased_limb_accumulator_u32_tmp_34cc4_92 = PackedUInt32::from_m31(((((((((((((M31_4) * (poseidon_full_round_chain_output_limb_0_col123))) + (((M31_2) * (cube_252_output_limb_0_col153))))) - (((M31_2) * (cube_252_output_limb_0_col174))))) + (M31_121657377))) - (combination_limb_0_col184))) + (M31_402653187)));let p_coef_col194 = ((biased_limb_accumulator_u32_tmp_34cc4_92.low().as_m31()) - (M31_3));
            *row[194] = p_coef_col194;let carry_0_tmp_34cc4_93 = ((((((((((((((M31_4) * (poseidon_full_round_chain_output_limb_0_col123))) + (((M31_2) * (cube_252_output_limb_0_col153))))) - (((M31_2) * (cube_252_output_limb_0_col174))))) + (M31_121657377))) - (combination_limb_0_col184))) - (p_coef_col194))) * (M31_16));let carry_1_tmp_34cc4_94 = ((((((((((((carry_0_tmp_34cc4_93) + (((M31_4) * (poseidon_full_round_chain_output_limb_1_col124))))) + (((M31_2) * (cube_252_output_limb_1_col154))))) - (((M31_2) * (cube_252_output_limb_1_col175))))) + (M31_112479959))) - (combination_limb_1_col185))) * (M31_16));let carry_2_tmp_34cc4_95 = ((((((((((((carry_1_tmp_34cc4_94) + (((M31_4) * (poseidon_full_round_chain_output_limb_2_col125))))) + (((M31_2) * (cube_252_output_limb_2_col155))))) - (((M31_2) * (cube_252_output_limb_2_col176))))) + (M31_130418270))) - (combination_limb_2_col186))) * (M31_16));let carry_3_tmp_34cc4_96 = ((((((((((((carry_2_tmp_34cc4_95) + (((M31_4) * (poseidon_full_round_chain_output_limb_3_col126))))) + (((M31_2) * (cube_252_output_limb_3_col156))))) - (((M31_2) * (cube_252_output_limb_3_col177))))) + (M31_4974792))) - (combination_limb_3_col187))) * (M31_16));let carry_4_tmp_34cc4_97 = ((((((((((((carry_3_tmp_34cc4_96) + (((M31_4) * (poseidon_full_round_chain_output_limb_4_col127))))) + (((M31_2) * (cube_252_output_limb_4_col157))))) - (((M31_2) * (cube_252_output_limb_4_col178))))) + (M31_59852719))) - (combination_limb_4_col188))) * (M31_16));let carry_5_tmp_34cc4_98 = ((((((((((((carry_4_tmp_34cc4_97) + (((M31_4) * (poseidon_full_round_chain_output_limb_5_col128))))) + (((M31_2) * (cube_252_output_limb_5_col158))))) - (((M31_2) * (cube_252_output_limb_5_col179))))) + (M31_120369218))) - (combination_limb_5_col189))) * (M31_16));let carry_6_tmp_34cc4_99 = ((((((((((((carry_5_tmp_34cc4_98) + (((M31_4) * (poseidon_full_round_chain_output_limb_6_col129))))) + (((M31_2) * (cube_252_output_limb_6_col159))))) - (((M31_2) * (cube_252_output_limb_6_col180))))) + (M31_62439890))) - (combination_limb_6_col190))) * (M31_16));let carry_7_tmp_34cc4_100 = ((((((((((((((carry_6_tmp_34cc4_99) + (((M31_4) * (poseidon_full_round_chain_output_limb_7_col130))))) + (((M31_2) * (cube_252_output_limb_7_col160))))) - (((M31_2) * (cube_252_output_limb_7_col181))))) + (M31_50468641))) - (combination_limb_7_col191))) - (((p_coef_col194) * (M31_136))))) * (M31_16));let carry_8_tmp_34cc4_101 = ((((((((((((carry_7_tmp_34cc4_100) + (((M31_4) * (poseidon_full_round_chain_output_limb_8_col131))))) + (((M31_2) * (cube_252_output_limb_8_col161))))) - (((M31_2) * (cube_252_output_limb_8_col182))))) + (M31_86573645))) - (combination_limb_8_col192))) * (M31_16));*sub_component_inputs.range_check_4_4_4_4[0] =
                [((p_coef_col194) + (M31_3)), ((carry_0_tmp_34cc4_93) + (M31_3)), ((carry_1_tmp_34cc4_94) + (M31_3)), ((carry_2_tmp_34cc4_95) + (M31_3))];
            *lookup_data.range_check_4_4_4_4_0 = [((p_coef_col194) + (M31_3)), ((carry_0_tmp_34cc4_93) + (M31_3)), ((carry_1_tmp_34cc4_94) + (M31_3)), ((carry_2_tmp_34cc4_95) + (M31_3))];*sub_component_inputs.range_check_4_4_4_4[1] =
                [((carry_3_tmp_34cc4_96) + (M31_3)), ((carry_4_tmp_34cc4_97) + (M31_3)), ((carry_5_tmp_34cc4_98) + (M31_3)), ((carry_6_tmp_34cc4_99) + (M31_3))];
            *lookup_data.range_check_4_4_4_4_1 = [((carry_3_tmp_34cc4_96) + (M31_3)), ((carry_4_tmp_34cc4_97) + (M31_3)), ((carry_5_tmp_34cc4_98) + (M31_3)), ((carry_6_tmp_34cc4_99) + (M31_3))];*sub_component_inputs.range_check_4_4[0] =
                [((carry_7_tmp_34cc4_100) + (M31_3)), ((carry_8_tmp_34cc4_101) + (M31_3))];
            *lookup_data.range_check_4_4_0 = [((carry_7_tmp_34cc4_100) + (M31_3)), ((carry_8_tmp_34cc4_101) + (M31_3))];let linear_combination_n_4_coefs_4_2_m2_1_output_tmp_34cc4_102 = combination_tmp_34cc4_91;

            *lookup_data.poseidon_3_partial_rounds_chain_0 = [seq, M31_4, cube_252_output_limb_0_col153, cube_252_output_limb_1_col154, cube_252_output_limb_2_col155, cube_252_output_limb_3_col156, cube_252_output_limb_4_col157, cube_252_output_limb_5_col158, cube_252_output_limb_6_col159, cube_252_output_limb_7_col160, cube_252_output_limb_8_col161, cube_252_output_limb_9_col162, combination_limb_0_col163, combination_limb_1_col164, combination_limb_2_col165, combination_limb_3_col166, combination_limb_4_col167, combination_limb_5_col168, combination_limb_6_col169, combination_limb_7_col170, combination_limb_8_col171, combination_limb_9_col172, cube_252_output_limb_0_col174, cube_252_output_limb_1_col175, cube_252_output_limb_2_col176, cube_252_output_limb_3_col177, cube_252_output_limb_4_col178, cube_252_output_limb_5_col179, cube_252_output_limb_6_col180, cube_252_output_limb_7_col181, cube_252_output_limb_8_col182, cube_252_output_limb_9_col183, combination_limb_0_col184, combination_limb_1_col185, combination_limb_2_col186, combination_limb_3_col187, combination_limb_4_col188, combination_limb_5_col189, combination_limb_6_col190, combination_limb_7_col191, combination_limb_8_col192, combination_limb_9_col193];*sub_component_inputs.poseidon_3_partial_rounds_chain[0] =
                (seq, M31_4, [cube_252_output_tmp_34cc4_77, linear_combination_n_4_coefs_1_1_m2_1_output_tmp_34cc4_89, cube_252_output_tmp_34cc4_90, linear_combination_n_4_coefs_4_2_m2_1_output_tmp_34cc4_102]);
            let poseidon_3_partial_rounds_chain_output_round_4_tmp_34cc4_104 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_4, [cube_252_output_tmp_34cc4_77, linear_combination_n_4_coefs_1_1_m2_1_output_tmp_34cc4_89, cube_252_output_tmp_34cc4_90, linear_combination_n_4_coefs_4_2_m2_1_output_tmp_34cc4_102]));*sub_component_inputs.poseidon_3_partial_rounds_chain[1] =
                (seq, M31_5, [poseidon_3_partial_rounds_chain_output_round_4_tmp_34cc4_104.2[0], poseidon_3_partial_rounds_chain_output_round_4_tmp_34cc4_104.2[1], poseidon_3_partial_rounds_chain_output_round_4_tmp_34cc4_104.2[2], poseidon_3_partial_rounds_chain_output_round_4_tmp_34cc4_104.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_5_tmp_34cc4_105 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_5, [poseidon_3_partial_rounds_chain_output_round_4_tmp_34cc4_104.2[0], poseidon_3_partial_rounds_chain_output_round_4_tmp_34cc4_104.2[1], poseidon_3_partial_rounds_chain_output_round_4_tmp_34cc4_104.2[2], poseidon_3_partial_rounds_chain_output_round_4_tmp_34cc4_104.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[2] =
                (seq, M31_6, [poseidon_3_partial_rounds_chain_output_round_5_tmp_34cc4_105.2[0], poseidon_3_partial_rounds_chain_output_round_5_tmp_34cc4_105.2[1], poseidon_3_partial_rounds_chain_output_round_5_tmp_34cc4_105.2[2], poseidon_3_partial_rounds_chain_output_round_5_tmp_34cc4_105.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_6_tmp_34cc4_106 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_6, [poseidon_3_partial_rounds_chain_output_round_5_tmp_34cc4_105.2[0], poseidon_3_partial_rounds_chain_output_round_5_tmp_34cc4_105.2[1], poseidon_3_partial_rounds_chain_output_round_5_tmp_34cc4_105.2[2], poseidon_3_partial_rounds_chain_output_round_5_tmp_34cc4_105.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[3] =
                (seq, M31_7, [poseidon_3_partial_rounds_chain_output_round_6_tmp_34cc4_106.2[0], poseidon_3_partial_rounds_chain_output_round_6_tmp_34cc4_106.2[1], poseidon_3_partial_rounds_chain_output_round_6_tmp_34cc4_106.2[2], poseidon_3_partial_rounds_chain_output_round_6_tmp_34cc4_106.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_7_tmp_34cc4_107 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_7, [poseidon_3_partial_rounds_chain_output_round_6_tmp_34cc4_106.2[0], poseidon_3_partial_rounds_chain_output_round_6_tmp_34cc4_106.2[1], poseidon_3_partial_rounds_chain_output_round_6_tmp_34cc4_106.2[2], poseidon_3_partial_rounds_chain_output_round_6_tmp_34cc4_106.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[4] =
                (seq, M31_8, [poseidon_3_partial_rounds_chain_output_round_7_tmp_34cc4_107.2[0], poseidon_3_partial_rounds_chain_output_round_7_tmp_34cc4_107.2[1], poseidon_3_partial_rounds_chain_output_round_7_tmp_34cc4_107.2[2], poseidon_3_partial_rounds_chain_output_round_7_tmp_34cc4_107.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_8_tmp_34cc4_108 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_8, [poseidon_3_partial_rounds_chain_output_round_7_tmp_34cc4_107.2[0], poseidon_3_partial_rounds_chain_output_round_7_tmp_34cc4_107.2[1], poseidon_3_partial_rounds_chain_output_round_7_tmp_34cc4_107.2[2], poseidon_3_partial_rounds_chain_output_round_7_tmp_34cc4_107.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[5] =
                (seq, M31_9, [poseidon_3_partial_rounds_chain_output_round_8_tmp_34cc4_108.2[0], poseidon_3_partial_rounds_chain_output_round_8_tmp_34cc4_108.2[1], poseidon_3_partial_rounds_chain_output_round_8_tmp_34cc4_108.2[2], poseidon_3_partial_rounds_chain_output_round_8_tmp_34cc4_108.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_9_tmp_34cc4_109 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_9, [poseidon_3_partial_rounds_chain_output_round_8_tmp_34cc4_108.2[0], poseidon_3_partial_rounds_chain_output_round_8_tmp_34cc4_108.2[1], poseidon_3_partial_rounds_chain_output_round_8_tmp_34cc4_108.2[2], poseidon_3_partial_rounds_chain_output_round_8_tmp_34cc4_108.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[6] =
                (seq, M31_10, [poseidon_3_partial_rounds_chain_output_round_9_tmp_34cc4_109.2[0], poseidon_3_partial_rounds_chain_output_round_9_tmp_34cc4_109.2[1], poseidon_3_partial_rounds_chain_output_round_9_tmp_34cc4_109.2[2], poseidon_3_partial_rounds_chain_output_round_9_tmp_34cc4_109.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_10_tmp_34cc4_110 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_10, [poseidon_3_partial_rounds_chain_output_round_9_tmp_34cc4_109.2[0], poseidon_3_partial_rounds_chain_output_round_9_tmp_34cc4_109.2[1], poseidon_3_partial_rounds_chain_output_round_9_tmp_34cc4_109.2[2], poseidon_3_partial_rounds_chain_output_round_9_tmp_34cc4_109.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[7] =
                (seq, M31_11, [poseidon_3_partial_rounds_chain_output_round_10_tmp_34cc4_110.2[0], poseidon_3_partial_rounds_chain_output_round_10_tmp_34cc4_110.2[1], poseidon_3_partial_rounds_chain_output_round_10_tmp_34cc4_110.2[2], poseidon_3_partial_rounds_chain_output_round_10_tmp_34cc4_110.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_11_tmp_34cc4_111 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_11, [poseidon_3_partial_rounds_chain_output_round_10_tmp_34cc4_110.2[0], poseidon_3_partial_rounds_chain_output_round_10_tmp_34cc4_110.2[1], poseidon_3_partial_rounds_chain_output_round_10_tmp_34cc4_110.2[2], poseidon_3_partial_rounds_chain_output_round_10_tmp_34cc4_110.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[8] =
                (seq, M31_12, [poseidon_3_partial_rounds_chain_output_round_11_tmp_34cc4_111.2[0], poseidon_3_partial_rounds_chain_output_round_11_tmp_34cc4_111.2[1], poseidon_3_partial_rounds_chain_output_round_11_tmp_34cc4_111.2[2], poseidon_3_partial_rounds_chain_output_round_11_tmp_34cc4_111.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_12_tmp_34cc4_112 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_12, [poseidon_3_partial_rounds_chain_output_round_11_tmp_34cc4_111.2[0], poseidon_3_partial_rounds_chain_output_round_11_tmp_34cc4_111.2[1], poseidon_3_partial_rounds_chain_output_round_11_tmp_34cc4_111.2[2], poseidon_3_partial_rounds_chain_output_round_11_tmp_34cc4_111.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[9] =
                (seq, M31_13, [poseidon_3_partial_rounds_chain_output_round_12_tmp_34cc4_112.2[0], poseidon_3_partial_rounds_chain_output_round_12_tmp_34cc4_112.2[1], poseidon_3_partial_rounds_chain_output_round_12_tmp_34cc4_112.2[2], poseidon_3_partial_rounds_chain_output_round_12_tmp_34cc4_112.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_13_tmp_34cc4_113 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_13, [poseidon_3_partial_rounds_chain_output_round_12_tmp_34cc4_112.2[0], poseidon_3_partial_rounds_chain_output_round_12_tmp_34cc4_112.2[1], poseidon_3_partial_rounds_chain_output_round_12_tmp_34cc4_112.2[2], poseidon_3_partial_rounds_chain_output_round_12_tmp_34cc4_112.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[10] =
                (seq, M31_14, [poseidon_3_partial_rounds_chain_output_round_13_tmp_34cc4_113.2[0], poseidon_3_partial_rounds_chain_output_round_13_tmp_34cc4_113.2[1], poseidon_3_partial_rounds_chain_output_round_13_tmp_34cc4_113.2[2], poseidon_3_partial_rounds_chain_output_round_13_tmp_34cc4_113.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_14_tmp_34cc4_114 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_14, [poseidon_3_partial_rounds_chain_output_round_13_tmp_34cc4_113.2[0], poseidon_3_partial_rounds_chain_output_round_13_tmp_34cc4_113.2[1], poseidon_3_partial_rounds_chain_output_round_13_tmp_34cc4_113.2[2], poseidon_3_partial_rounds_chain_output_round_13_tmp_34cc4_113.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[11] =
                (seq, M31_15, [poseidon_3_partial_rounds_chain_output_round_14_tmp_34cc4_114.2[0], poseidon_3_partial_rounds_chain_output_round_14_tmp_34cc4_114.2[1], poseidon_3_partial_rounds_chain_output_round_14_tmp_34cc4_114.2[2], poseidon_3_partial_rounds_chain_output_round_14_tmp_34cc4_114.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_15_tmp_34cc4_115 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_15, [poseidon_3_partial_rounds_chain_output_round_14_tmp_34cc4_114.2[0], poseidon_3_partial_rounds_chain_output_round_14_tmp_34cc4_114.2[1], poseidon_3_partial_rounds_chain_output_round_14_tmp_34cc4_114.2[2], poseidon_3_partial_rounds_chain_output_round_14_tmp_34cc4_114.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[12] =
                (seq, M31_16, [poseidon_3_partial_rounds_chain_output_round_15_tmp_34cc4_115.2[0], poseidon_3_partial_rounds_chain_output_round_15_tmp_34cc4_115.2[1], poseidon_3_partial_rounds_chain_output_round_15_tmp_34cc4_115.2[2], poseidon_3_partial_rounds_chain_output_round_15_tmp_34cc4_115.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_16_tmp_34cc4_116 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_16, [poseidon_3_partial_rounds_chain_output_round_15_tmp_34cc4_115.2[0], poseidon_3_partial_rounds_chain_output_round_15_tmp_34cc4_115.2[1], poseidon_3_partial_rounds_chain_output_round_15_tmp_34cc4_115.2[2], poseidon_3_partial_rounds_chain_output_round_15_tmp_34cc4_115.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[13] =
                (seq, M31_17, [poseidon_3_partial_rounds_chain_output_round_16_tmp_34cc4_116.2[0], poseidon_3_partial_rounds_chain_output_round_16_tmp_34cc4_116.2[1], poseidon_3_partial_rounds_chain_output_round_16_tmp_34cc4_116.2[2], poseidon_3_partial_rounds_chain_output_round_16_tmp_34cc4_116.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_17_tmp_34cc4_117 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_17, [poseidon_3_partial_rounds_chain_output_round_16_tmp_34cc4_116.2[0], poseidon_3_partial_rounds_chain_output_round_16_tmp_34cc4_116.2[1], poseidon_3_partial_rounds_chain_output_round_16_tmp_34cc4_116.2[2], poseidon_3_partial_rounds_chain_output_round_16_tmp_34cc4_116.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[14] =
                (seq, M31_18, [poseidon_3_partial_rounds_chain_output_round_17_tmp_34cc4_117.2[0], poseidon_3_partial_rounds_chain_output_round_17_tmp_34cc4_117.2[1], poseidon_3_partial_rounds_chain_output_round_17_tmp_34cc4_117.2[2], poseidon_3_partial_rounds_chain_output_round_17_tmp_34cc4_117.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_18_tmp_34cc4_118 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_18, [poseidon_3_partial_rounds_chain_output_round_17_tmp_34cc4_117.2[0], poseidon_3_partial_rounds_chain_output_round_17_tmp_34cc4_117.2[1], poseidon_3_partial_rounds_chain_output_round_17_tmp_34cc4_117.2[2], poseidon_3_partial_rounds_chain_output_round_17_tmp_34cc4_117.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[15] =
                (seq, M31_19, [poseidon_3_partial_rounds_chain_output_round_18_tmp_34cc4_118.2[0], poseidon_3_partial_rounds_chain_output_round_18_tmp_34cc4_118.2[1], poseidon_3_partial_rounds_chain_output_round_18_tmp_34cc4_118.2[2], poseidon_3_partial_rounds_chain_output_round_18_tmp_34cc4_118.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_19_tmp_34cc4_119 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_19, [poseidon_3_partial_rounds_chain_output_round_18_tmp_34cc4_118.2[0], poseidon_3_partial_rounds_chain_output_round_18_tmp_34cc4_118.2[1], poseidon_3_partial_rounds_chain_output_round_18_tmp_34cc4_118.2[2], poseidon_3_partial_rounds_chain_output_round_18_tmp_34cc4_118.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[16] =
                (seq, M31_20, [poseidon_3_partial_rounds_chain_output_round_19_tmp_34cc4_119.2[0], poseidon_3_partial_rounds_chain_output_round_19_tmp_34cc4_119.2[1], poseidon_3_partial_rounds_chain_output_round_19_tmp_34cc4_119.2[2], poseidon_3_partial_rounds_chain_output_round_19_tmp_34cc4_119.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_20_tmp_34cc4_120 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_20, [poseidon_3_partial_rounds_chain_output_round_19_tmp_34cc4_119.2[0], poseidon_3_partial_rounds_chain_output_round_19_tmp_34cc4_119.2[1], poseidon_3_partial_rounds_chain_output_round_19_tmp_34cc4_119.2[2], poseidon_3_partial_rounds_chain_output_round_19_tmp_34cc4_119.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[17] =
                (seq, M31_21, [poseidon_3_partial_rounds_chain_output_round_20_tmp_34cc4_120.2[0], poseidon_3_partial_rounds_chain_output_round_20_tmp_34cc4_120.2[1], poseidon_3_partial_rounds_chain_output_round_20_tmp_34cc4_120.2[2], poseidon_3_partial_rounds_chain_output_round_20_tmp_34cc4_120.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_21_tmp_34cc4_121 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_21, [poseidon_3_partial_rounds_chain_output_round_20_tmp_34cc4_120.2[0], poseidon_3_partial_rounds_chain_output_round_20_tmp_34cc4_120.2[1], poseidon_3_partial_rounds_chain_output_round_20_tmp_34cc4_120.2[2], poseidon_3_partial_rounds_chain_output_round_20_tmp_34cc4_120.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[18] =
                (seq, M31_22, [poseidon_3_partial_rounds_chain_output_round_21_tmp_34cc4_121.2[0], poseidon_3_partial_rounds_chain_output_round_21_tmp_34cc4_121.2[1], poseidon_3_partial_rounds_chain_output_round_21_tmp_34cc4_121.2[2], poseidon_3_partial_rounds_chain_output_round_21_tmp_34cc4_121.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_22_tmp_34cc4_122 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_22, [poseidon_3_partial_rounds_chain_output_round_21_tmp_34cc4_121.2[0], poseidon_3_partial_rounds_chain_output_round_21_tmp_34cc4_121.2[1], poseidon_3_partial_rounds_chain_output_round_21_tmp_34cc4_121.2[2], poseidon_3_partial_rounds_chain_output_round_21_tmp_34cc4_121.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[19] =
                (seq, M31_23, [poseidon_3_partial_rounds_chain_output_round_22_tmp_34cc4_122.2[0], poseidon_3_partial_rounds_chain_output_round_22_tmp_34cc4_122.2[1], poseidon_3_partial_rounds_chain_output_round_22_tmp_34cc4_122.2[2], poseidon_3_partial_rounds_chain_output_round_22_tmp_34cc4_122.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_23_tmp_34cc4_123 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_23, [poseidon_3_partial_rounds_chain_output_round_22_tmp_34cc4_122.2[0], poseidon_3_partial_rounds_chain_output_round_22_tmp_34cc4_122.2[1], poseidon_3_partial_rounds_chain_output_round_22_tmp_34cc4_122.2[2], poseidon_3_partial_rounds_chain_output_round_22_tmp_34cc4_122.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[20] =
                (seq, M31_24, [poseidon_3_partial_rounds_chain_output_round_23_tmp_34cc4_123.2[0], poseidon_3_partial_rounds_chain_output_round_23_tmp_34cc4_123.2[1], poseidon_3_partial_rounds_chain_output_round_23_tmp_34cc4_123.2[2], poseidon_3_partial_rounds_chain_output_round_23_tmp_34cc4_123.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_24_tmp_34cc4_124 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_24, [poseidon_3_partial_rounds_chain_output_round_23_tmp_34cc4_123.2[0], poseidon_3_partial_rounds_chain_output_round_23_tmp_34cc4_123.2[1], poseidon_3_partial_rounds_chain_output_round_23_tmp_34cc4_123.2[2], poseidon_3_partial_rounds_chain_output_round_23_tmp_34cc4_123.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[21] =
                (seq, M31_25, [poseidon_3_partial_rounds_chain_output_round_24_tmp_34cc4_124.2[0], poseidon_3_partial_rounds_chain_output_round_24_tmp_34cc4_124.2[1], poseidon_3_partial_rounds_chain_output_round_24_tmp_34cc4_124.2[2], poseidon_3_partial_rounds_chain_output_round_24_tmp_34cc4_124.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_25_tmp_34cc4_125 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_25, [poseidon_3_partial_rounds_chain_output_round_24_tmp_34cc4_124.2[0], poseidon_3_partial_rounds_chain_output_round_24_tmp_34cc4_124.2[1], poseidon_3_partial_rounds_chain_output_round_24_tmp_34cc4_124.2[2], poseidon_3_partial_rounds_chain_output_round_24_tmp_34cc4_124.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[22] =
                (seq, M31_26, [poseidon_3_partial_rounds_chain_output_round_25_tmp_34cc4_125.2[0], poseidon_3_partial_rounds_chain_output_round_25_tmp_34cc4_125.2[1], poseidon_3_partial_rounds_chain_output_round_25_tmp_34cc4_125.2[2], poseidon_3_partial_rounds_chain_output_round_25_tmp_34cc4_125.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_26_tmp_34cc4_126 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_26, [poseidon_3_partial_rounds_chain_output_round_25_tmp_34cc4_125.2[0], poseidon_3_partial_rounds_chain_output_round_25_tmp_34cc4_125.2[1], poseidon_3_partial_rounds_chain_output_round_25_tmp_34cc4_125.2[2], poseidon_3_partial_rounds_chain_output_round_25_tmp_34cc4_125.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[23] =
                (seq, M31_27, [poseidon_3_partial_rounds_chain_output_round_26_tmp_34cc4_126.2[0], poseidon_3_partial_rounds_chain_output_round_26_tmp_34cc4_126.2[1], poseidon_3_partial_rounds_chain_output_round_26_tmp_34cc4_126.2[2], poseidon_3_partial_rounds_chain_output_round_26_tmp_34cc4_126.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_27_tmp_34cc4_127 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_27, [poseidon_3_partial_rounds_chain_output_round_26_tmp_34cc4_126.2[0], poseidon_3_partial_rounds_chain_output_round_26_tmp_34cc4_126.2[1], poseidon_3_partial_rounds_chain_output_round_26_tmp_34cc4_126.2[2], poseidon_3_partial_rounds_chain_output_round_26_tmp_34cc4_126.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[24] =
                (seq, M31_28, [poseidon_3_partial_rounds_chain_output_round_27_tmp_34cc4_127.2[0], poseidon_3_partial_rounds_chain_output_round_27_tmp_34cc4_127.2[1], poseidon_3_partial_rounds_chain_output_round_27_tmp_34cc4_127.2[2], poseidon_3_partial_rounds_chain_output_round_27_tmp_34cc4_127.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_28_tmp_34cc4_128 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_28, [poseidon_3_partial_rounds_chain_output_round_27_tmp_34cc4_127.2[0], poseidon_3_partial_rounds_chain_output_round_27_tmp_34cc4_127.2[1], poseidon_3_partial_rounds_chain_output_round_27_tmp_34cc4_127.2[2], poseidon_3_partial_rounds_chain_output_round_27_tmp_34cc4_127.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[25] =
                (seq, M31_29, [poseidon_3_partial_rounds_chain_output_round_28_tmp_34cc4_128.2[0], poseidon_3_partial_rounds_chain_output_round_28_tmp_34cc4_128.2[1], poseidon_3_partial_rounds_chain_output_round_28_tmp_34cc4_128.2[2], poseidon_3_partial_rounds_chain_output_round_28_tmp_34cc4_128.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_29_tmp_34cc4_129 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_29, [poseidon_3_partial_rounds_chain_output_round_28_tmp_34cc4_128.2[0], poseidon_3_partial_rounds_chain_output_round_28_tmp_34cc4_128.2[1], poseidon_3_partial_rounds_chain_output_round_28_tmp_34cc4_128.2[2], poseidon_3_partial_rounds_chain_output_round_28_tmp_34cc4_128.2[3]]));*sub_component_inputs.poseidon_3_partial_rounds_chain[26] =
                (seq, M31_30, [poseidon_3_partial_rounds_chain_output_round_29_tmp_34cc4_129.2[0], poseidon_3_partial_rounds_chain_output_round_29_tmp_34cc4_129.2[1], poseidon_3_partial_rounds_chain_output_round_29_tmp_34cc4_129.2[2], poseidon_3_partial_rounds_chain_output_round_29_tmp_34cc4_129.2[3]]);
            let poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130 = PackedPoseidon3PartialRoundsChain::deduce_output((seq, M31_30, [poseidon_3_partial_rounds_chain_output_round_29_tmp_34cc4_129.2[0], poseidon_3_partial_rounds_chain_output_round_29_tmp_34cc4_129.2[1], poseidon_3_partial_rounds_chain_output_round_29_tmp_34cc4_129.2[2], poseidon_3_partial_rounds_chain_output_round_29_tmp_34cc4_129.2[3]]));let poseidon_3_partial_rounds_chain_output_limb_0_col195 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[0].get_m31(0);
            *row[195] = poseidon_3_partial_rounds_chain_output_limb_0_col195;let poseidon_3_partial_rounds_chain_output_limb_1_col196 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[0].get_m31(1);
            *row[196] = poseidon_3_partial_rounds_chain_output_limb_1_col196;let poseidon_3_partial_rounds_chain_output_limb_2_col197 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[0].get_m31(2);
            *row[197] = poseidon_3_partial_rounds_chain_output_limb_2_col197;let poseidon_3_partial_rounds_chain_output_limb_3_col198 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[0].get_m31(3);
            *row[198] = poseidon_3_partial_rounds_chain_output_limb_3_col198;let poseidon_3_partial_rounds_chain_output_limb_4_col199 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[0].get_m31(4);
            *row[199] = poseidon_3_partial_rounds_chain_output_limb_4_col199;let poseidon_3_partial_rounds_chain_output_limb_5_col200 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[0].get_m31(5);
            *row[200] = poseidon_3_partial_rounds_chain_output_limb_5_col200;let poseidon_3_partial_rounds_chain_output_limb_6_col201 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[0].get_m31(6);
            *row[201] = poseidon_3_partial_rounds_chain_output_limb_6_col201;let poseidon_3_partial_rounds_chain_output_limb_7_col202 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[0].get_m31(7);
            *row[202] = poseidon_3_partial_rounds_chain_output_limb_7_col202;let poseidon_3_partial_rounds_chain_output_limb_8_col203 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[0].get_m31(8);
            *row[203] = poseidon_3_partial_rounds_chain_output_limb_8_col203;let poseidon_3_partial_rounds_chain_output_limb_9_col204 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[0].get_m31(9);
            *row[204] = poseidon_3_partial_rounds_chain_output_limb_9_col204;let poseidon_3_partial_rounds_chain_output_limb_10_col205 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[1].get_m31(0);
            *row[205] = poseidon_3_partial_rounds_chain_output_limb_10_col205;let poseidon_3_partial_rounds_chain_output_limb_11_col206 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[1].get_m31(1);
            *row[206] = poseidon_3_partial_rounds_chain_output_limb_11_col206;let poseidon_3_partial_rounds_chain_output_limb_12_col207 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[1].get_m31(2);
            *row[207] = poseidon_3_partial_rounds_chain_output_limb_12_col207;let poseidon_3_partial_rounds_chain_output_limb_13_col208 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[1].get_m31(3);
            *row[208] = poseidon_3_partial_rounds_chain_output_limb_13_col208;let poseidon_3_partial_rounds_chain_output_limb_14_col209 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[1].get_m31(4);
            *row[209] = poseidon_3_partial_rounds_chain_output_limb_14_col209;let poseidon_3_partial_rounds_chain_output_limb_15_col210 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[1].get_m31(5);
            *row[210] = poseidon_3_partial_rounds_chain_output_limb_15_col210;let poseidon_3_partial_rounds_chain_output_limb_16_col211 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[1].get_m31(6);
            *row[211] = poseidon_3_partial_rounds_chain_output_limb_16_col211;let poseidon_3_partial_rounds_chain_output_limb_17_col212 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[1].get_m31(7);
            *row[212] = poseidon_3_partial_rounds_chain_output_limb_17_col212;let poseidon_3_partial_rounds_chain_output_limb_18_col213 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[1].get_m31(8);
            *row[213] = poseidon_3_partial_rounds_chain_output_limb_18_col213;let poseidon_3_partial_rounds_chain_output_limb_19_col214 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[1].get_m31(9);
            *row[214] = poseidon_3_partial_rounds_chain_output_limb_19_col214;let poseidon_3_partial_rounds_chain_output_limb_20_col215 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[2].get_m31(0);
            *row[215] = poseidon_3_partial_rounds_chain_output_limb_20_col215;let poseidon_3_partial_rounds_chain_output_limb_21_col216 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[2].get_m31(1);
            *row[216] = poseidon_3_partial_rounds_chain_output_limb_21_col216;let poseidon_3_partial_rounds_chain_output_limb_22_col217 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[2].get_m31(2);
            *row[217] = poseidon_3_partial_rounds_chain_output_limb_22_col217;let poseidon_3_partial_rounds_chain_output_limb_23_col218 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[2].get_m31(3);
            *row[218] = poseidon_3_partial_rounds_chain_output_limb_23_col218;let poseidon_3_partial_rounds_chain_output_limb_24_col219 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[2].get_m31(4);
            *row[219] = poseidon_3_partial_rounds_chain_output_limb_24_col219;let poseidon_3_partial_rounds_chain_output_limb_25_col220 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[2].get_m31(5);
            *row[220] = poseidon_3_partial_rounds_chain_output_limb_25_col220;let poseidon_3_partial_rounds_chain_output_limb_26_col221 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[2].get_m31(6);
            *row[221] = poseidon_3_partial_rounds_chain_output_limb_26_col221;let poseidon_3_partial_rounds_chain_output_limb_27_col222 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[2].get_m31(7);
            *row[222] = poseidon_3_partial_rounds_chain_output_limb_27_col222;let poseidon_3_partial_rounds_chain_output_limb_28_col223 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[2].get_m31(8);
            *row[223] = poseidon_3_partial_rounds_chain_output_limb_28_col223;let poseidon_3_partial_rounds_chain_output_limb_29_col224 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[2].get_m31(9);
            *row[224] = poseidon_3_partial_rounds_chain_output_limb_29_col224;let poseidon_3_partial_rounds_chain_output_limb_30_col225 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[3].get_m31(0);
            *row[225] = poseidon_3_partial_rounds_chain_output_limb_30_col225;let poseidon_3_partial_rounds_chain_output_limb_31_col226 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[3].get_m31(1);
            *row[226] = poseidon_3_partial_rounds_chain_output_limb_31_col226;let poseidon_3_partial_rounds_chain_output_limb_32_col227 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[3].get_m31(2);
            *row[227] = poseidon_3_partial_rounds_chain_output_limb_32_col227;let poseidon_3_partial_rounds_chain_output_limb_33_col228 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[3].get_m31(3);
            *row[228] = poseidon_3_partial_rounds_chain_output_limb_33_col228;let poseidon_3_partial_rounds_chain_output_limb_34_col229 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[3].get_m31(4);
            *row[229] = poseidon_3_partial_rounds_chain_output_limb_34_col229;let poseidon_3_partial_rounds_chain_output_limb_35_col230 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[3].get_m31(5);
            *row[230] = poseidon_3_partial_rounds_chain_output_limb_35_col230;let poseidon_3_partial_rounds_chain_output_limb_36_col231 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[3].get_m31(6);
            *row[231] = poseidon_3_partial_rounds_chain_output_limb_36_col231;let poseidon_3_partial_rounds_chain_output_limb_37_col232 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[3].get_m31(7);
            *row[232] = poseidon_3_partial_rounds_chain_output_limb_37_col232;let poseidon_3_partial_rounds_chain_output_limb_38_col233 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[3].get_m31(8);
            *row[233] = poseidon_3_partial_rounds_chain_output_limb_38_col233;let poseidon_3_partial_rounds_chain_output_limb_39_col234 = poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[3].get_m31(9);
            *row[234] = poseidon_3_partial_rounds_chain_output_limb_39_col234;*lookup_data.poseidon_3_partial_rounds_chain_1 = [seq, M31_31, poseidon_3_partial_rounds_chain_output_limb_0_col195, poseidon_3_partial_rounds_chain_output_limb_1_col196, poseidon_3_partial_rounds_chain_output_limb_2_col197, poseidon_3_partial_rounds_chain_output_limb_3_col198, poseidon_3_partial_rounds_chain_output_limb_4_col199, poseidon_3_partial_rounds_chain_output_limb_5_col200, poseidon_3_partial_rounds_chain_output_limb_6_col201, poseidon_3_partial_rounds_chain_output_limb_7_col202, poseidon_3_partial_rounds_chain_output_limb_8_col203, poseidon_3_partial_rounds_chain_output_limb_9_col204, poseidon_3_partial_rounds_chain_output_limb_10_col205, poseidon_3_partial_rounds_chain_output_limb_11_col206, poseidon_3_partial_rounds_chain_output_limb_12_col207, poseidon_3_partial_rounds_chain_output_limb_13_col208, poseidon_3_partial_rounds_chain_output_limb_14_col209, poseidon_3_partial_rounds_chain_output_limb_15_col210, poseidon_3_partial_rounds_chain_output_limb_16_col211, poseidon_3_partial_rounds_chain_output_limb_17_col212, poseidon_3_partial_rounds_chain_output_limb_18_col213, poseidon_3_partial_rounds_chain_output_limb_19_col214, poseidon_3_partial_rounds_chain_output_limb_20_col215, poseidon_3_partial_rounds_chain_output_limb_21_col216, poseidon_3_partial_rounds_chain_output_limb_22_col217, poseidon_3_partial_rounds_chain_output_limb_23_col218, poseidon_3_partial_rounds_chain_output_limb_24_col219, poseidon_3_partial_rounds_chain_output_limb_25_col220, poseidon_3_partial_rounds_chain_output_limb_26_col221, poseidon_3_partial_rounds_chain_output_limb_27_col222, poseidon_3_partial_rounds_chain_output_limb_28_col223, poseidon_3_partial_rounds_chain_output_limb_29_col224, poseidon_3_partial_rounds_chain_output_limb_30_col225, poseidon_3_partial_rounds_chain_output_limb_31_col226, poseidon_3_partial_rounds_chain_output_limb_32_col227, poseidon_3_partial_rounds_chain_output_limb_33_col228, poseidon_3_partial_rounds_chain_output_limb_34_col229, poseidon_3_partial_rounds_chain_output_limb_35_col230, poseidon_3_partial_rounds_chain_output_limb_36_col231, poseidon_3_partial_rounds_chain_output_limb_37_col232, poseidon_3_partial_rounds_chain_output_limb_38_col233, poseidon_3_partial_rounds_chain_output_limb_39_col234];

            // Linear Combination N 4 Coefs 4 2 1 1.

            let combination_tmp_34cc4_131 = PackedFelt252Width27::from_packed_felt252(((((((((Felt252_0_0_0_0) + (((Felt252_4_0_0_0) * (PackedFelt252::from_packed_felt252width27(poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[0])))))) + (((Felt252_2_0_0_0) * (PackedFelt252::from_packed_felt252width27(poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[1])))))) + (((Felt252_1_0_0_0) * (PackedFelt252::from_packed_felt252width27(poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[2])))))) + (Felt252_3969818800901670911_10562874008078701503_14906396266795319764_223312371439046257)));let combination_limb_0_col235 = combination_tmp_34cc4_131.get_m31(0);
            *row[235] = combination_limb_0_col235;let combination_limb_1_col236 = combination_tmp_34cc4_131.get_m31(1);
            *row[236] = combination_limb_1_col236;let combination_limb_2_col237 = combination_tmp_34cc4_131.get_m31(2);
            *row[237] = combination_limb_2_col237;let combination_limb_3_col238 = combination_tmp_34cc4_131.get_m31(3);
            *row[238] = combination_limb_3_col238;let combination_limb_4_col239 = combination_tmp_34cc4_131.get_m31(4);
            *row[239] = combination_limb_4_col239;let combination_limb_5_col240 = combination_tmp_34cc4_131.get_m31(5);
            *row[240] = combination_limb_5_col240;let combination_limb_6_col241 = combination_tmp_34cc4_131.get_m31(6);
            *row[241] = combination_limb_6_col241;let combination_limb_7_col242 = combination_tmp_34cc4_131.get_m31(7);
            *row[242] = combination_limb_7_col242;let combination_limb_8_col243 = combination_tmp_34cc4_131.get_m31(8);
            *row[243] = combination_limb_8_col243;let combination_limb_9_col244 = combination_tmp_34cc4_131.get_m31(9);
            *row[244] = combination_limb_9_col244;let biased_limb_accumulator_u32_tmp_34cc4_132 = PackedUInt32::from_m31(((((((((((((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_0_col195))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_10_col205))))) + (poseidon_3_partial_rounds_chain_output_limb_20_col215))) + (M31_40454143))) - (combination_limb_0_col235))) + (M31_134217729)));let p_coef_col245 = ((biased_limb_accumulator_u32_tmp_34cc4_132.low().as_m31()) - (M31_1));
            *row[245] = p_coef_col245;let carry_0_tmp_34cc4_133 = ((((((((((((((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_0_col195))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_10_col205))))) + (poseidon_3_partial_rounds_chain_output_limb_20_col215))) + (M31_40454143))) - (combination_limb_0_col235))) - (p_coef_col245))) * (M31_16));let carry_1_tmp_34cc4_134 = ((((((((((((carry_0_tmp_34cc4_133) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_1_col196))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_11_col206))))) + (poseidon_3_partial_rounds_chain_output_limb_21_col216))) + (M31_49554771))) - (combination_limb_1_col236))) * (M31_16));let carry_2_tmp_34cc4_135 = ((((((((((((carry_1_tmp_34cc4_134) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_2_col197))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_12_col207))))) + (poseidon_3_partial_rounds_chain_output_limb_22_col217))) + (M31_55508188))) - (combination_limb_2_col237))) * (M31_16));let carry_3_tmp_34cc4_136 = ((((((((((((carry_2_tmp_34cc4_135) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_3_col198))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_13_col208))))) + (poseidon_3_partial_rounds_chain_output_limb_23_col218))) + (M31_116986206))) - (combination_limb_3_col238))) * (M31_16));let carry_4_tmp_34cc4_137 = ((((((((((((carry_3_tmp_34cc4_136) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_4_col199))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_14_col209))))) + (poseidon_3_partial_rounds_chain_output_limb_24_col219))) + (M31_88680813))) - (combination_limb_4_col239))) * (M31_16));let carry_5_tmp_34cc4_138 = ((((((((((((carry_4_tmp_34cc4_137) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_5_col200))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_15_col210))))) + (poseidon_3_partial_rounds_chain_output_limb_25_col220))) + (M31_45553283))) - (combination_limb_5_col240))) * (M31_16));let carry_6_tmp_34cc4_139 = ((((((((((((carry_5_tmp_34cc4_138) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_6_col201))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_16_col211))))) + (poseidon_3_partial_rounds_chain_output_limb_26_col221))) + (M31_62360091))) - (combination_limb_6_col241))) * (M31_16));let carry_7_tmp_34cc4_140 = ((((((((((((((carry_6_tmp_34cc4_139) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_7_col202))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_17_col212))))) + (poseidon_3_partial_rounds_chain_output_limb_27_col222))) + (M31_77099918))) - (combination_limb_7_col242))) - (((p_coef_col245) * (M31_136))))) * (M31_16));let carry_8_tmp_34cc4_141 = ((((((((((((carry_7_tmp_34cc4_140) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_8_col203))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_18_col213))))) + (poseidon_3_partial_rounds_chain_output_limb_28_col223))) + (M31_22899501))) - (combination_limb_8_col243))) * (M31_16));*sub_component_inputs.range_check_4_4_4_4[2] =
                [((p_coef_col245) + (M31_1)), ((carry_0_tmp_34cc4_133) + (M31_1)), ((carry_1_tmp_34cc4_134) + (M31_1)), ((carry_2_tmp_34cc4_135) + (M31_1))];
            *lookup_data.range_check_4_4_4_4_2 = [((p_coef_col245) + (M31_1)), ((carry_0_tmp_34cc4_133) + (M31_1)), ((carry_1_tmp_34cc4_134) + (M31_1)), ((carry_2_tmp_34cc4_135) + (M31_1))];*sub_component_inputs.range_check_4_4_4_4[3] =
                [((carry_3_tmp_34cc4_136) + (M31_1)), ((carry_4_tmp_34cc4_137) + (M31_1)), ((carry_5_tmp_34cc4_138) + (M31_1)), ((carry_6_tmp_34cc4_139) + (M31_1))];
            *lookup_data.range_check_4_4_4_4_3 = [((carry_3_tmp_34cc4_136) + (M31_1)), ((carry_4_tmp_34cc4_137) + (M31_1)), ((carry_5_tmp_34cc4_138) + (M31_1)), ((carry_6_tmp_34cc4_139) + (M31_1))];*sub_component_inputs.range_check_4_4[1] =
                [((carry_7_tmp_34cc4_140) + (M31_1)), ((carry_8_tmp_34cc4_141) + (M31_1))];
            *lookup_data.range_check_4_4_1 = [((carry_7_tmp_34cc4_140) + (M31_1)), ((carry_8_tmp_34cc4_141) + (M31_1))];let linear_combination_n_4_coefs_4_2_1_1_output_tmp_34cc4_142 = combination_tmp_34cc4_131;

            // Linear Combination N 4 Coefs 4 2 1 1.

            let combination_tmp_34cc4_143 = PackedFelt252Width27::from_packed_felt252(((((((((Felt252_0_0_0_0) + (((Felt252_4_0_0_0) * (PackedFelt252::from_packed_felt252width27(poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[2])))))) + (((Felt252_2_0_0_0) * (PackedFelt252::from_packed_felt252width27(poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[3])))))) + (((Felt252_1_0_0_0) * (PackedFelt252::from_packed_felt252width27(linear_combination_n_4_coefs_4_2_1_1_output_tmp_34cc4_142)))))) + (Felt252_10310704347937391837_5874215448258336115_2880320859071049537_45350836576946303)));let combination_limb_0_col246 = combination_tmp_34cc4_143.get_m31(0);
            *row[246] = combination_limb_0_col246;let combination_limb_1_col247 = combination_tmp_34cc4_143.get_m31(1);
            *row[247] = combination_limb_1_col247;let combination_limb_2_col248 = combination_tmp_34cc4_143.get_m31(2);
            *row[248] = combination_limb_2_col248;let combination_limb_3_col249 = combination_tmp_34cc4_143.get_m31(3);
            *row[249] = combination_limb_3_col249;let combination_limb_4_col250 = combination_tmp_34cc4_143.get_m31(4);
            *row[250] = combination_limb_4_col250;let combination_limb_5_col251 = combination_tmp_34cc4_143.get_m31(5);
            *row[251] = combination_limb_5_col251;let combination_limb_6_col252 = combination_tmp_34cc4_143.get_m31(6);
            *row[252] = combination_limb_6_col252;let combination_limb_7_col253 = combination_tmp_34cc4_143.get_m31(7);
            *row[253] = combination_limb_7_col253;let combination_limb_8_col254 = combination_tmp_34cc4_143.get_m31(8);
            *row[254] = combination_limb_8_col254;let combination_limb_9_col255 = combination_tmp_34cc4_143.get_m31(9);
            *row[255] = combination_limb_9_col255;let biased_limb_accumulator_u32_tmp_34cc4_144 = PackedUInt32::from_m31(((((((((((((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_20_col215))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_30_col225))))) + (combination_limb_0_col235))) + (M31_48383197))) - (combination_limb_0_col246))) + (M31_134217729)));let p_coef_col256 = ((biased_limb_accumulator_u32_tmp_34cc4_144.low().as_m31()) - (M31_1));
            *row[256] = p_coef_col256;let carry_0_tmp_34cc4_145 = ((((((((((((((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_20_col215))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_30_col225))))) + (combination_limb_0_col235))) + (M31_48383197))) - (combination_limb_0_col246))) - (p_coef_col256))) * (M31_16));let carry_1_tmp_34cc4_146 = ((((((((((((carry_0_tmp_34cc4_145) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_21_col216))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_31_col226))))) + (combination_limb_1_col236))) + (M31_48193339))) - (combination_limb_1_col247))) * (M31_16));let carry_2_tmp_34cc4_147 = ((((((((((((carry_1_tmp_34cc4_146) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_22_col217))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_32_col227))))) + (combination_limb_2_col237))) + (M31_55955004))) - (combination_limb_2_col248))) * (M31_16));let carry_3_tmp_34cc4_148 = ((((((((((((carry_2_tmp_34cc4_147) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_23_col218))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_33_col228))))) + (combination_limb_3_col238))) + (M31_65659846))) - (combination_limb_3_col249))) * (M31_16));let carry_4_tmp_34cc4_149 = ((((((((((((carry_3_tmp_34cc4_148) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_24_col219))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_34_col229))))) + (combination_limb_4_col239))) + (M31_68491350))) - (combination_limb_4_col250))) * (M31_16));let carry_5_tmp_34cc4_150 = ((((((((((((carry_4_tmp_34cc4_149) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_25_col220))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_35_col230))))) + (combination_limb_5_col240))) + (M31_119023582))) - (combination_limb_5_col251))) * (M31_16));let carry_6_tmp_34cc4_151 = ((((((((((((carry_5_tmp_34cc4_150) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_26_col221))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_36_col231))))) + (combination_limb_6_col241))) + (M31_33439011))) - (combination_limb_6_col252))) * (M31_16));let carry_7_tmp_34cc4_152 = ((((((((((((((carry_6_tmp_34cc4_151) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_27_col222))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_37_col232))))) + (combination_limb_7_col242))) + (M31_58475513))) - (combination_limb_7_col253))) - (((p_coef_col256) * (M31_136))))) * (M31_16));let carry_8_tmp_34cc4_153 = ((((((((((((carry_7_tmp_34cc4_152) + (((M31_4) * (poseidon_3_partial_rounds_chain_output_limb_28_col223))))) + (((M31_2) * (poseidon_3_partial_rounds_chain_output_limb_38_col233))))) + (combination_limb_8_col243))) + (M31_18765944))) - (combination_limb_8_col254))) * (M31_16));*sub_component_inputs.range_check_4_4_4_4[4] =
                [((p_coef_col256) + (M31_1)), ((carry_0_tmp_34cc4_145) + (M31_1)), ((carry_1_tmp_34cc4_146) + (M31_1)), ((carry_2_tmp_34cc4_147) + (M31_1))];
            *lookup_data.range_check_4_4_4_4_4 = [((p_coef_col256) + (M31_1)), ((carry_0_tmp_34cc4_145) + (M31_1)), ((carry_1_tmp_34cc4_146) + (M31_1)), ((carry_2_tmp_34cc4_147) + (M31_1))];*sub_component_inputs.range_check_4_4_4_4[5] =
                [((carry_3_tmp_34cc4_148) + (M31_1)), ((carry_4_tmp_34cc4_149) + (M31_1)), ((carry_5_tmp_34cc4_150) + (M31_1)), ((carry_6_tmp_34cc4_151) + (M31_1))];
            *lookup_data.range_check_4_4_4_4_5 = [((carry_3_tmp_34cc4_148) + (M31_1)), ((carry_4_tmp_34cc4_149) + (M31_1)), ((carry_5_tmp_34cc4_150) + (M31_1)), ((carry_6_tmp_34cc4_151) + (M31_1))];*sub_component_inputs.range_check_4_4[2] =
                [((carry_7_tmp_34cc4_152) + (M31_1)), ((carry_8_tmp_34cc4_153) + (M31_1))];
            *lookup_data.range_check_4_4_2 = [((carry_7_tmp_34cc4_152) + (M31_1)), ((carry_8_tmp_34cc4_153) + (M31_1))];let linear_combination_n_4_coefs_4_2_1_1_output_tmp_34cc4_154 = combination_tmp_34cc4_143;

            let poseidon_full_round_chain_chain_id_tmp_34cc4_155 = ((poseidon_full_round_chain_chain_tmp_tmp_34cc4_72) + (M31_1));*lookup_data.poseidon_full_round_chain_2 = [poseidon_full_round_chain_chain_id_tmp_34cc4_155, M31_31, combination_limb_0_col246, combination_limb_1_col247, combination_limb_2_col248, combination_limb_3_col249, combination_limb_4_col250, combination_limb_5_col251, combination_limb_6_col252, combination_limb_7_col253, combination_limb_8_col254, combination_limb_9_col255, combination_limb_0_col235, combination_limb_1_col236, combination_limb_2_col237, combination_limb_3_col238, combination_limb_4_col239, combination_limb_5_col240, combination_limb_6_col241, combination_limb_7_col242, combination_limb_8_col243, combination_limb_9_col244, poseidon_3_partial_rounds_chain_output_limb_30_col225, poseidon_3_partial_rounds_chain_output_limb_31_col226, poseidon_3_partial_rounds_chain_output_limb_32_col227, poseidon_3_partial_rounds_chain_output_limb_33_col228, poseidon_3_partial_rounds_chain_output_limb_34_col229, poseidon_3_partial_rounds_chain_output_limb_35_col230, poseidon_3_partial_rounds_chain_output_limb_36_col231, poseidon_3_partial_rounds_chain_output_limb_37_col232, poseidon_3_partial_rounds_chain_output_limb_38_col233, poseidon_3_partial_rounds_chain_output_limb_39_col234];*sub_component_inputs.poseidon_full_round_chain[4] =
                (poseidon_full_round_chain_chain_id_tmp_34cc4_155, M31_31, [linear_combination_n_4_coefs_4_2_1_1_output_tmp_34cc4_154, linear_combination_n_4_coefs_4_2_1_1_output_tmp_34cc4_142, poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[3]]);
            let poseidon_full_round_chain_output_round_31_tmp_34cc4_156 = PackedPoseidonFullRoundChain::deduce_output((poseidon_full_round_chain_chain_id_tmp_34cc4_155, M31_31, [linear_combination_n_4_coefs_4_2_1_1_output_tmp_34cc4_154, linear_combination_n_4_coefs_4_2_1_1_output_tmp_34cc4_142, poseidon_3_partial_rounds_chain_output_round_30_tmp_34cc4_130.2[3]]));*sub_component_inputs.poseidon_full_round_chain[5] =
                (poseidon_full_round_chain_chain_id_tmp_34cc4_155, M31_32, [poseidon_full_round_chain_output_round_31_tmp_34cc4_156.2[0], poseidon_full_round_chain_output_round_31_tmp_34cc4_156.2[1], poseidon_full_round_chain_output_round_31_tmp_34cc4_156.2[2]]);
            let poseidon_full_round_chain_output_round_32_tmp_34cc4_157 = PackedPoseidonFullRoundChain::deduce_output((poseidon_full_round_chain_chain_id_tmp_34cc4_155, M31_32, [poseidon_full_round_chain_output_round_31_tmp_34cc4_156.2[0], poseidon_full_round_chain_output_round_31_tmp_34cc4_156.2[1], poseidon_full_round_chain_output_round_31_tmp_34cc4_156.2[2]]));*sub_component_inputs.poseidon_full_round_chain[6] =
                (poseidon_full_round_chain_chain_id_tmp_34cc4_155, M31_33, [poseidon_full_round_chain_output_round_32_tmp_34cc4_157.2[0], poseidon_full_round_chain_output_round_32_tmp_34cc4_157.2[1], poseidon_full_round_chain_output_round_32_tmp_34cc4_157.2[2]]);
            let poseidon_full_round_chain_output_round_33_tmp_34cc4_158 = PackedPoseidonFullRoundChain::deduce_output((poseidon_full_round_chain_chain_id_tmp_34cc4_155, M31_33, [poseidon_full_round_chain_output_round_32_tmp_34cc4_157.2[0], poseidon_full_round_chain_output_round_32_tmp_34cc4_157.2[1], poseidon_full_round_chain_output_round_32_tmp_34cc4_157.2[2]]));*sub_component_inputs.poseidon_full_round_chain[7] =
                (poseidon_full_round_chain_chain_id_tmp_34cc4_155, M31_34, [poseidon_full_round_chain_output_round_33_tmp_34cc4_158.2[0], poseidon_full_round_chain_output_round_33_tmp_34cc4_158.2[1], poseidon_full_round_chain_output_round_33_tmp_34cc4_158.2[2]]);
            let poseidon_full_round_chain_output_round_34_tmp_34cc4_159 = PackedPoseidonFullRoundChain::deduce_output((poseidon_full_round_chain_chain_id_tmp_34cc4_155, M31_34, [poseidon_full_round_chain_output_round_33_tmp_34cc4_158.2[0], poseidon_full_round_chain_output_round_33_tmp_34cc4_158.2[1], poseidon_full_round_chain_output_round_33_tmp_34cc4_158.2[2]]));let poseidon_full_round_chain_output_limb_0_col257 = poseidon_full_round_chain_output_round_34_tmp_34cc4_159.2[0].get_m31(0);
            *row[257] = poseidon_full_round_chain_output_limb_0_col257;let poseidon_full_round_chain_output_limb_1_col258 = poseidon_full_round_chain_output_round_34_tmp_34cc4_159.2[0].get_m31(1);
            *row[258] = poseidon_full_round_chain_output_limb_1_col258;let poseidon_full_round_chain_output_limb_2_col259 = poseidon_full_round_chain_output_round_34_tmp_34cc4_159.2[0].get_m31(2);
            *row[259] = poseidon_full_round_chain_output_limb_2_col259;let poseidon_full_round_chain_output_limb_3_col260 = poseidon_full_round_chain_output_round_34_tmp_34cc4_159.2[0].get_m31(3);
            *row[260] = poseidon_full_round_chain_output_limb_3_col260;let poseidon_full_round_chain_output_limb_4_col261 = poseidon_full_round_chain_output_round_34_tmp_34cc4_159.2[0].get_m31(4);
            *row[261] = poseidon_full_round_chain_output_limb_4_col261;let poseidon_full_round_chain_output_limb_5_col262 = poseidon_full_round_chain_output_round_34_tmp_34cc4_159.2[0].get_m31(5);
            *row[262] = poseidon_full_round_chain_output_limb_5_col262;let poseidon_full_round_chain_output_limb_6_col263 = poseidon_full_round_chain_output_round_34_tmp_34cc4_159.2[0].get_m31(6);
            *row[263] = poseidon_full_round_chain_output_limb_6_col263;let poseidon_full_round_chain_output_limb_7_col264 = poseidon_full_round_chain_output_round_34_tmp_34cc4_159.2[0].get_m31(7);
            *row[264] = poseidon_full_round_chain_output_limb_7_col264;let poseidon_full_round_chain_output_limb_8_col265 = poseidon_full_round_chain_output_round_34_tmp_34cc4_159.2[0].get_m31(8);
            *row[265] = poseidon_full_round_chain_output_limb_8_col265;let poseidon_full_round_chain_output_limb_9_col266 = poseidon_full_round_chain_output_round_34_tmp_34cc4_159.2[0].get_m31(9);
            *row[266] = poseidon_full_round_chain_output_limb_9_col266;let poseidon_full_round_chain_output_limb_10_col267 = poseidon_full_round_chain_output_round_34_tmp_34cc4_159.2[1].get_m31(0);
            *row[267] = poseidon_full_round_chain_output_limb_10_col267;let poseidon_full_round_chain_output_limb_11_col268 = poseidon_full_round_chain_output_round_34_tmp_34cc4_159.2[1].get_m31(1);
            *row[268] = poseidon_full_round_chain_output_limb_11_col268;let poseidon_full_round_chain_output_limb_12_col269 = poseidon_full_round_chain_output_round_34_tmp_34cc4_159.2[1].get_m31(2);
            *row[269] = poseidon_full_round_chain_output_limb_12_col269;let poseidon_full_round_chain_output_limb_13_col270 = poseidon_full_round_chain_output_round_34_tmp_34cc4_159.2[1].get_m31(3);
            *row[270] = poseidon_full_round_chain_output_limb_13_col270;let poseidon_full_round_chain_output_limb_14_col271 = poseidon_full_round_chain_output_round_34_tmp_34cc4_159.2[1].get_m31(4);
            *row[271] = poseidon_full_round_chain_output_limb_14_col271;let poseidon_full_round_chain_output_limb_15_col272 = poseidon_full_round_chain_output_round_34_tmp_34cc4_159.2[1].get_m31(5);
            *row[272] = poseidon_full_round_chain_output_limb_15_col272;let poseidon_full_round_chain_output_limb_16_col273 = poseidon_full_round_chain_output_round_34_tmp_34cc4_159.2[1].get_m31(6);
            *row[273] = poseidon_full_round_chain_output_limb_16_col273;let poseidon_full_round_chain_output_limb_17_col274 = poseidon_full_round_chain_output_round_34_tmp_34cc4_159.2[1].get_m31(7);
            *row[274] = poseidon_full_round_chain_output_limb_17_col274;let poseidon_full_round_chain_output_limb_18_col275 = poseidon_full_round_chain_output_round_34_tmp_34cc4_159.2[1].get_m31(8);
            *row[275] = poseidon_full_round_chain_output_limb_18_col275;let poseidon_full_round_chain_output_limb_19_col276 = poseidon_full_round_chain_output_round_34_tmp_34cc4_159.2[1].get_m31(9);
            *row[276] = poseidon_full_round_chain_output_limb_19_col276;let poseidon_full_round_chain_output_limb_20_col277 = poseidon_full_round_chain_output_round_34_tmp_34cc4_159.2[2].get_m31(0);
            *row[277] = poseidon_full_round_chain_output_limb_20_col277;let poseidon_full_round_chain_output_limb_21_col278 = poseidon_full_round_chain_output_round_34_tmp_34cc4_159.2[2].get_m31(1);
            *row[278] = poseidon_full_round_chain_output_limb_21_col278;let poseidon_full_round_chain_output_limb_22_col279 = poseidon_full_round_chain_output_round_34_tmp_34cc4_159.2[2].get_m31(2);
            *row[279] = poseidon_full_round_chain_output_limb_22_col279;let poseidon_full_round_chain_output_limb_23_col280 = poseidon_full_round_chain_output_round_34_tmp_34cc4_159.2[2].get_m31(3);
            *row[280] = poseidon_full_round_chain_output_limb_23_col280;let poseidon_full_round_chain_output_limb_24_col281 = poseidon_full_round_chain_output_round_34_tmp_34cc4_159.2[2].get_m31(4);
            *row[281] = poseidon_full_round_chain_output_limb_24_col281;let poseidon_full_round_chain_output_limb_25_col282 = poseidon_full_round_chain_output_round_34_tmp_34cc4_159.2[2].get_m31(5);
            *row[282] = poseidon_full_round_chain_output_limb_25_col282;let poseidon_full_round_chain_output_limb_26_col283 = poseidon_full_round_chain_output_round_34_tmp_34cc4_159.2[2].get_m31(6);
            *row[283] = poseidon_full_round_chain_output_limb_26_col283;let poseidon_full_round_chain_output_limb_27_col284 = poseidon_full_round_chain_output_round_34_tmp_34cc4_159.2[2].get_m31(7);
            *row[284] = poseidon_full_round_chain_output_limb_27_col284;let poseidon_full_round_chain_output_limb_28_col285 = poseidon_full_round_chain_output_round_34_tmp_34cc4_159.2[2].get_m31(8);
            *row[285] = poseidon_full_round_chain_output_limb_28_col285;let poseidon_full_round_chain_output_limb_29_col286 = poseidon_full_round_chain_output_round_34_tmp_34cc4_159.2[2].get_m31(9);
            *row[286] = poseidon_full_round_chain_output_limb_29_col286;*lookup_data.poseidon_full_round_chain_3 = [poseidon_full_round_chain_chain_id_tmp_34cc4_155, M31_35, poseidon_full_round_chain_output_limb_0_col257, poseidon_full_round_chain_output_limb_1_col258, poseidon_full_round_chain_output_limb_2_col259, poseidon_full_round_chain_output_limb_3_col260, poseidon_full_round_chain_output_limb_4_col261, poseidon_full_round_chain_output_limb_5_col262, poseidon_full_round_chain_output_limb_6_col263, poseidon_full_round_chain_output_limb_7_col264, poseidon_full_round_chain_output_limb_8_col265, poseidon_full_round_chain_output_limb_9_col266, poseidon_full_round_chain_output_limb_10_col267, poseidon_full_round_chain_output_limb_11_col268, poseidon_full_round_chain_output_limb_12_col269, poseidon_full_round_chain_output_limb_13_col270, poseidon_full_round_chain_output_limb_14_col271, poseidon_full_round_chain_output_limb_15_col272, poseidon_full_round_chain_output_limb_16_col273, poseidon_full_round_chain_output_limb_17_col274, poseidon_full_round_chain_output_limb_18_col275, poseidon_full_round_chain_output_limb_19_col276, poseidon_full_round_chain_output_limb_20_col277, poseidon_full_round_chain_output_limb_21_col278, poseidon_full_round_chain_output_limb_22_col279, poseidon_full_round_chain_output_limb_23_col280, poseidon_full_round_chain_output_limb_24_col281, poseidon_full_round_chain_output_limb_25_col282, poseidon_full_round_chain_output_limb_26_col283, poseidon_full_round_chain_output_limb_27_col284, poseidon_full_round_chain_output_limb_28_col285, poseidon_full_round_chain_output_limb_29_col286];let poseidon_hades_permutation_output_tmp_34cc4_160 = [poseidon_full_round_chain_output_round_34_tmp_34cc4_159.2[0], poseidon_full_round_chain_output_round_34_tmp_34cc4_159.2[1], poseidon_full_round_chain_output_round_34_tmp_34cc4_159.2[2]];

            // Felt 252 Unpack From 27.

            let input_as_felt252_tmp_34cc4_161 = PackedFelt252::from_packed_felt252width27(poseidon_hades_permutation_output_tmp_34cc4_160[0]);let unpacked_limb_0_col287 = input_as_felt252_tmp_34cc4_161.get_m31(0);
            *row[287] = unpacked_limb_0_col287;let unpacked_limb_1_col288 = input_as_felt252_tmp_34cc4_161.get_m31(1);
            *row[288] = unpacked_limb_1_col288;let unpacked_limb_3_col289 = input_as_felt252_tmp_34cc4_161.get_m31(3);
            *row[289] = unpacked_limb_3_col289;let unpacked_limb_4_col290 = input_as_felt252_tmp_34cc4_161.get_m31(4);
            *row[290] = unpacked_limb_4_col290;let unpacked_limb_6_col291 = input_as_felt252_tmp_34cc4_161.get_m31(6);
            *row[291] = unpacked_limb_6_col291;let unpacked_limb_7_col292 = input_as_felt252_tmp_34cc4_161.get_m31(7);
            *row[292] = unpacked_limb_7_col292;let unpacked_limb_9_col293 = input_as_felt252_tmp_34cc4_161.get_m31(9);
            *row[293] = unpacked_limb_9_col293;let unpacked_limb_10_col294 = input_as_felt252_tmp_34cc4_161.get_m31(10);
            *row[294] = unpacked_limb_10_col294;let unpacked_limb_12_col295 = input_as_felt252_tmp_34cc4_161.get_m31(12);
            *row[295] = unpacked_limb_12_col295;let unpacked_limb_13_col296 = input_as_felt252_tmp_34cc4_161.get_m31(13);
            *row[296] = unpacked_limb_13_col296;let unpacked_limb_15_col297 = input_as_felt252_tmp_34cc4_161.get_m31(15);
            *row[297] = unpacked_limb_15_col297;let unpacked_limb_16_col298 = input_as_felt252_tmp_34cc4_161.get_m31(16);
            *row[298] = unpacked_limb_16_col298;let unpacked_limb_18_col299 = input_as_felt252_tmp_34cc4_161.get_m31(18);
            *row[299] = unpacked_limb_18_col299;let unpacked_limb_19_col300 = input_as_felt252_tmp_34cc4_161.get_m31(19);
            *row[300] = unpacked_limb_19_col300;let unpacked_limb_21_col301 = input_as_felt252_tmp_34cc4_161.get_m31(21);
            *row[301] = unpacked_limb_21_col301;let unpacked_limb_22_col302 = input_as_felt252_tmp_34cc4_161.get_m31(22);
            *row[302] = unpacked_limb_22_col302;let unpacked_limb_24_col303 = input_as_felt252_tmp_34cc4_161.get_m31(24);
            *row[303] = unpacked_limb_24_col303;let unpacked_limb_25_col304 = input_as_felt252_tmp_34cc4_161.get_m31(25);
            *row[304] = unpacked_limb_25_col304;let felt_252_unpack_from_27_output_tmp_34cc4_162 = PackedFelt252::from_limbs([unpacked_limb_0_col287, unpacked_limb_1_col288, ((((((poseidon_full_round_chain_output_limb_0_col257) - (unpacked_limb_0_col287))) - (((unpacked_limb_1_col288) * (M31_512))))) * (M31_8192)), unpacked_limb_3_col289, unpacked_limb_4_col290, ((((((poseidon_full_round_chain_output_limb_1_col258) - (unpacked_limb_3_col289))) - (((unpacked_limb_4_col290) * (M31_512))))) * (M31_8192)), unpacked_limb_6_col291, unpacked_limb_7_col292, ((((((poseidon_full_round_chain_output_limb_2_col259) - (unpacked_limb_6_col291))) - (((unpacked_limb_7_col292) * (M31_512))))) * (M31_8192)), unpacked_limb_9_col293, unpacked_limb_10_col294, ((((((poseidon_full_round_chain_output_limb_3_col260) - (unpacked_limb_9_col293))) - (((unpacked_limb_10_col294) * (M31_512))))) * (M31_8192)), unpacked_limb_12_col295, unpacked_limb_13_col296, ((((((poseidon_full_round_chain_output_limb_4_col261) - (unpacked_limb_12_col295))) - (((unpacked_limb_13_col296) * (M31_512))))) * (M31_8192)), unpacked_limb_15_col297, unpacked_limb_16_col298, ((((((poseidon_full_round_chain_output_limb_5_col262) - (unpacked_limb_15_col297))) - (((unpacked_limb_16_col298) * (M31_512))))) * (M31_8192)), unpacked_limb_18_col299, unpacked_limb_19_col300, ((((((poseidon_full_round_chain_output_limb_6_col263) - (unpacked_limb_18_col299))) - (((unpacked_limb_19_col300) * (M31_512))))) * (M31_8192)), unpacked_limb_21_col301, unpacked_limb_22_col302, ((((((poseidon_full_round_chain_output_limb_7_col264) - (unpacked_limb_21_col301))) - (((unpacked_limb_22_col302) * (M31_512))))) * (M31_8192)), unpacked_limb_24_col303, unpacked_limb_25_col304, ((((((poseidon_full_round_chain_output_limb_8_col265) - (unpacked_limb_24_col303))) - (((unpacked_limb_25_col304) * (M31_512))))) * (M31_8192)), poseidon_full_round_chain_output_limb_9_col266]);

            *sub_component_inputs.memory_id_to_big[3] =
                input_limb_3_col3;
            *lookup_data.memory_id_to_big_3 = [input_limb_3_col3, unpacked_limb_0_col287, unpacked_limb_1_col288, felt_252_unpack_from_27_output_tmp_34cc4_162.get_m31(2), unpacked_limb_3_col289, unpacked_limb_4_col290, felt_252_unpack_from_27_output_tmp_34cc4_162.get_m31(5), unpacked_limb_6_col291, unpacked_limb_7_col292, felt_252_unpack_from_27_output_tmp_34cc4_162.get_m31(8), unpacked_limb_9_col293, unpacked_limb_10_col294, felt_252_unpack_from_27_output_tmp_34cc4_162.get_m31(11), unpacked_limb_12_col295, unpacked_limb_13_col296, felt_252_unpack_from_27_output_tmp_34cc4_162.get_m31(14), unpacked_limb_15_col297, unpacked_limb_16_col298, felt_252_unpack_from_27_output_tmp_34cc4_162.get_m31(17), unpacked_limb_18_col299, unpacked_limb_19_col300, felt_252_unpack_from_27_output_tmp_34cc4_162.get_m31(20), unpacked_limb_21_col301, unpacked_limb_22_col302, felt_252_unpack_from_27_output_tmp_34cc4_162.get_m31(23), unpacked_limb_24_col303, unpacked_limb_25_col304, felt_252_unpack_from_27_output_tmp_34cc4_162.get_m31(26), poseidon_full_round_chain_output_limb_9_col266];

            // Felt 252 Unpack From 27.

            let input_as_felt252_tmp_34cc4_163 = PackedFelt252::from_packed_felt252width27(poseidon_hades_permutation_output_tmp_34cc4_160[1]);let unpacked_limb_0_col305 = input_as_felt252_tmp_34cc4_163.get_m31(0);
            *row[305] = unpacked_limb_0_col305;let unpacked_limb_1_col306 = input_as_felt252_tmp_34cc4_163.get_m31(1);
            *row[306] = unpacked_limb_1_col306;let unpacked_limb_3_col307 = input_as_felt252_tmp_34cc4_163.get_m31(3);
            *row[307] = unpacked_limb_3_col307;let unpacked_limb_4_col308 = input_as_felt252_tmp_34cc4_163.get_m31(4);
            *row[308] = unpacked_limb_4_col308;let unpacked_limb_6_col309 = input_as_felt252_tmp_34cc4_163.get_m31(6);
            *row[309] = unpacked_limb_6_col309;let unpacked_limb_7_col310 = input_as_felt252_tmp_34cc4_163.get_m31(7);
            *row[310] = unpacked_limb_7_col310;let unpacked_limb_9_col311 = input_as_felt252_tmp_34cc4_163.get_m31(9);
            *row[311] = unpacked_limb_9_col311;let unpacked_limb_10_col312 = input_as_felt252_tmp_34cc4_163.get_m31(10);
            *row[312] = unpacked_limb_10_col312;let unpacked_limb_12_col313 = input_as_felt252_tmp_34cc4_163.get_m31(12);
            *row[313] = unpacked_limb_12_col313;let unpacked_limb_13_col314 = input_as_felt252_tmp_34cc4_163.get_m31(13);
            *row[314] = unpacked_limb_13_col314;let unpacked_limb_15_col315 = input_as_felt252_tmp_34cc4_163.get_m31(15);
            *row[315] = unpacked_limb_15_col315;let unpacked_limb_16_col316 = input_as_felt252_tmp_34cc4_163.get_m31(16);
            *row[316] = unpacked_limb_16_col316;let unpacked_limb_18_col317 = input_as_felt252_tmp_34cc4_163.get_m31(18);
            *row[317] = unpacked_limb_18_col317;let unpacked_limb_19_col318 = input_as_felt252_tmp_34cc4_163.get_m31(19);
            *row[318] = unpacked_limb_19_col318;let unpacked_limb_21_col319 = input_as_felt252_tmp_34cc4_163.get_m31(21);
            *row[319] = unpacked_limb_21_col319;let unpacked_limb_22_col320 = input_as_felt252_tmp_34cc4_163.get_m31(22);
            *row[320] = unpacked_limb_22_col320;let unpacked_limb_24_col321 = input_as_felt252_tmp_34cc4_163.get_m31(24);
            *row[321] = unpacked_limb_24_col321;let unpacked_limb_25_col322 = input_as_felt252_tmp_34cc4_163.get_m31(25);
            *row[322] = unpacked_limb_25_col322;let felt_252_unpack_from_27_output_tmp_34cc4_164 = PackedFelt252::from_limbs([unpacked_limb_0_col305, unpacked_limb_1_col306, ((((((poseidon_full_round_chain_output_limb_10_col267) - (unpacked_limb_0_col305))) - (((unpacked_limb_1_col306) * (M31_512))))) * (M31_8192)), unpacked_limb_3_col307, unpacked_limb_4_col308, ((((((poseidon_full_round_chain_output_limb_11_col268) - (unpacked_limb_3_col307))) - (((unpacked_limb_4_col308) * (M31_512))))) * (M31_8192)), unpacked_limb_6_col309, unpacked_limb_7_col310, ((((((poseidon_full_round_chain_output_limb_12_col269) - (unpacked_limb_6_col309))) - (((unpacked_limb_7_col310) * (M31_512))))) * (M31_8192)), unpacked_limb_9_col311, unpacked_limb_10_col312, ((((((poseidon_full_round_chain_output_limb_13_col270) - (unpacked_limb_9_col311))) - (((unpacked_limb_10_col312) * (M31_512))))) * (M31_8192)), unpacked_limb_12_col313, unpacked_limb_13_col314, ((((((poseidon_full_round_chain_output_limb_14_col271) - (unpacked_limb_12_col313))) - (((unpacked_limb_13_col314) * (M31_512))))) * (M31_8192)), unpacked_limb_15_col315, unpacked_limb_16_col316, ((((((poseidon_full_round_chain_output_limb_15_col272) - (unpacked_limb_15_col315))) - (((unpacked_limb_16_col316) * (M31_512))))) * (M31_8192)), unpacked_limb_18_col317, unpacked_limb_19_col318, ((((((poseidon_full_round_chain_output_limb_16_col273) - (unpacked_limb_18_col317))) - (((unpacked_limb_19_col318) * (M31_512))))) * (M31_8192)), unpacked_limb_21_col319, unpacked_limb_22_col320, ((((((poseidon_full_round_chain_output_limb_17_col274) - (unpacked_limb_21_col319))) - (((unpacked_limb_22_col320) * (M31_512))))) * (M31_8192)), unpacked_limb_24_col321, unpacked_limb_25_col322, ((((((poseidon_full_round_chain_output_limb_18_col275) - (unpacked_limb_24_col321))) - (((unpacked_limb_25_col322) * (M31_512))))) * (M31_8192)), poseidon_full_round_chain_output_limb_19_col276]);

            *sub_component_inputs.memory_id_to_big[4] =
                input_limb_4_col4;
            *lookup_data.memory_id_to_big_4 = [input_limb_4_col4, unpacked_limb_0_col305, unpacked_limb_1_col306, felt_252_unpack_from_27_output_tmp_34cc4_164.get_m31(2), unpacked_limb_3_col307, unpacked_limb_4_col308, felt_252_unpack_from_27_output_tmp_34cc4_164.get_m31(5), unpacked_limb_6_col309, unpacked_limb_7_col310, felt_252_unpack_from_27_output_tmp_34cc4_164.get_m31(8), unpacked_limb_9_col311, unpacked_limb_10_col312, felt_252_unpack_from_27_output_tmp_34cc4_164.get_m31(11), unpacked_limb_12_col313, unpacked_limb_13_col314, felt_252_unpack_from_27_output_tmp_34cc4_164.get_m31(14), unpacked_limb_15_col315, unpacked_limb_16_col316, felt_252_unpack_from_27_output_tmp_34cc4_164.get_m31(17), unpacked_limb_18_col317, unpacked_limb_19_col318, felt_252_unpack_from_27_output_tmp_34cc4_164.get_m31(20), unpacked_limb_21_col319, unpacked_limb_22_col320, felt_252_unpack_from_27_output_tmp_34cc4_164.get_m31(23), unpacked_limb_24_col321, unpacked_limb_25_col322, felt_252_unpack_from_27_output_tmp_34cc4_164.get_m31(26), poseidon_full_round_chain_output_limb_19_col276];

            // Felt 252 Unpack From 27.

            let input_as_felt252_tmp_34cc4_165 = PackedFelt252::from_packed_felt252width27(poseidon_hades_permutation_output_tmp_34cc4_160[2]);let unpacked_limb_0_col323 = input_as_felt252_tmp_34cc4_165.get_m31(0);
            *row[323] = unpacked_limb_0_col323;let unpacked_limb_1_col324 = input_as_felt252_tmp_34cc4_165.get_m31(1);
            *row[324] = unpacked_limb_1_col324;let unpacked_limb_3_col325 = input_as_felt252_tmp_34cc4_165.get_m31(3);
            *row[325] = unpacked_limb_3_col325;let unpacked_limb_4_col326 = input_as_felt252_tmp_34cc4_165.get_m31(4);
            *row[326] = unpacked_limb_4_col326;let unpacked_limb_6_col327 = input_as_felt252_tmp_34cc4_165.get_m31(6);
            *row[327] = unpacked_limb_6_col327;let unpacked_limb_7_col328 = input_as_felt252_tmp_34cc4_165.get_m31(7);
            *row[328] = unpacked_limb_7_col328;let unpacked_limb_9_col329 = input_as_felt252_tmp_34cc4_165.get_m31(9);
            *row[329] = unpacked_limb_9_col329;let unpacked_limb_10_col330 = input_as_felt252_tmp_34cc4_165.get_m31(10);
            *row[330] = unpacked_limb_10_col330;let unpacked_limb_12_col331 = input_as_felt252_tmp_34cc4_165.get_m31(12);
            *row[331] = unpacked_limb_12_col331;let unpacked_limb_13_col332 = input_as_felt252_tmp_34cc4_165.get_m31(13);
            *row[332] = unpacked_limb_13_col332;let unpacked_limb_15_col333 = input_as_felt252_tmp_34cc4_165.get_m31(15);
            *row[333] = unpacked_limb_15_col333;let unpacked_limb_16_col334 = input_as_felt252_tmp_34cc4_165.get_m31(16);
            *row[334] = unpacked_limb_16_col334;let unpacked_limb_18_col335 = input_as_felt252_tmp_34cc4_165.get_m31(18);
            *row[335] = unpacked_limb_18_col335;let unpacked_limb_19_col336 = input_as_felt252_tmp_34cc4_165.get_m31(19);
            *row[336] = unpacked_limb_19_col336;let unpacked_limb_21_col337 = input_as_felt252_tmp_34cc4_165.get_m31(21);
            *row[337] = unpacked_limb_21_col337;let unpacked_limb_22_col338 = input_as_felt252_tmp_34cc4_165.get_m31(22);
            *row[338] = unpacked_limb_22_col338;let unpacked_limb_24_col339 = input_as_felt252_tmp_34cc4_165.get_m31(24);
            *row[339] = unpacked_limb_24_col339;let unpacked_limb_25_col340 = input_as_felt252_tmp_34cc4_165.get_m31(25);
            *row[340] = unpacked_limb_25_col340;let felt_252_unpack_from_27_output_tmp_34cc4_166 = PackedFelt252::from_limbs([unpacked_limb_0_col323, unpacked_limb_1_col324, ((((((poseidon_full_round_chain_output_limb_20_col277) - (unpacked_limb_0_col323))) - (((unpacked_limb_1_col324) * (M31_512))))) * (M31_8192)), unpacked_limb_3_col325, unpacked_limb_4_col326, ((((((poseidon_full_round_chain_output_limb_21_col278) - (unpacked_limb_3_col325))) - (((unpacked_limb_4_col326) * (M31_512))))) * (M31_8192)), unpacked_limb_6_col327, unpacked_limb_7_col328, ((((((poseidon_full_round_chain_output_limb_22_col279) - (unpacked_limb_6_col327))) - (((unpacked_limb_7_col328) * (M31_512))))) * (M31_8192)), unpacked_limb_9_col329, unpacked_limb_10_col330, ((((((poseidon_full_round_chain_output_limb_23_col280) - (unpacked_limb_9_col329))) - (((unpacked_limb_10_col330) * (M31_512))))) * (M31_8192)), unpacked_limb_12_col331, unpacked_limb_13_col332, ((((((poseidon_full_round_chain_output_limb_24_col281) - (unpacked_limb_12_col331))) - (((unpacked_limb_13_col332) * (M31_512))))) * (M31_8192)), unpacked_limb_15_col333, unpacked_limb_16_col334, ((((((poseidon_full_round_chain_output_limb_25_col282) - (unpacked_limb_15_col333))) - (((unpacked_limb_16_col334) * (M31_512))))) * (M31_8192)), unpacked_limb_18_col335, unpacked_limb_19_col336, ((((((poseidon_full_round_chain_output_limb_26_col283) - (unpacked_limb_18_col335))) - (((unpacked_limb_19_col336) * (M31_512))))) * (M31_8192)), unpacked_limb_21_col337, unpacked_limb_22_col338, ((((((poseidon_full_round_chain_output_limb_27_col284) - (unpacked_limb_21_col337))) - (((unpacked_limb_22_col338) * (M31_512))))) * (M31_8192)), unpacked_limb_24_col339, unpacked_limb_25_col340, ((((((poseidon_full_round_chain_output_limb_28_col285) - (unpacked_limb_24_col339))) - (((unpacked_limb_25_col340) * (M31_512))))) * (M31_8192)), poseidon_full_round_chain_output_limb_29_col286]);

            *sub_component_inputs.memory_id_to_big[5] =
                input_limb_5_col5;
            *lookup_data.memory_id_to_big_5 = [input_limb_5_col5, unpacked_limb_0_col323, unpacked_limb_1_col324, felt_252_unpack_from_27_output_tmp_34cc4_166.get_m31(2), unpacked_limb_3_col325, unpacked_limb_4_col326, felt_252_unpack_from_27_output_tmp_34cc4_166.get_m31(5), unpacked_limb_6_col327, unpacked_limb_7_col328, felt_252_unpack_from_27_output_tmp_34cc4_166.get_m31(8), unpacked_limb_9_col329, unpacked_limb_10_col330, felt_252_unpack_from_27_output_tmp_34cc4_166.get_m31(11), unpacked_limb_12_col331, unpacked_limb_13_col332, felt_252_unpack_from_27_output_tmp_34cc4_166.get_m31(14), unpacked_limb_15_col333, unpacked_limb_16_col334, felt_252_unpack_from_27_output_tmp_34cc4_166.get_m31(17), unpacked_limb_18_col335, unpacked_limb_19_col336, felt_252_unpack_from_27_output_tmp_34cc4_166.get_m31(20), unpacked_limb_21_col337, unpacked_limb_22_col338, felt_252_unpack_from_27_output_tmp_34cc4_166.get_m31(23), unpacked_limb_24_col339, unpacked_limb_25_col340, felt_252_unpack_from_27_output_tmp_34cc4_166.get_m31(26), poseidon_full_round_chain_output_limb_29_col286];*lookup_data.poseidon_aggregator_0 = [input_limb_0_col0, input_limb_1_col1, input_limb_2_col2, input_limb_3_col3, input_limb_4_col4, input_limb_5_col5];let mult_at_row = *mults.get(row_index).unwrap_or(&PackedM31::zero());
            *row[341] = mult_at_row;
            *lookup_data.mults = mult_at_row;
        });

    (trace, lookup_data, sub_component_inputs)
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    cube_252_0: Vec<[PackedM31; 20]>,
    cube_252_1: Vec<[PackedM31; 20]>,
    memory_id_to_big_0: Vec<[PackedM31; 29]>,
    memory_id_to_big_1: Vec<[PackedM31; 29]>,
    memory_id_to_big_2: Vec<[PackedM31; 29]>,
    memory_id_to_big_3: Vec<[PackedM31; 29]>,
    memory_id_to_big_4: Vec<[PackedM31; 29]>,
    memory_id_to_big_5: Vec<[PackedM31; 29]>,
    poseidon_3_partial_rounds_chain_0: Vec<[PackedM31; 42]>,
    poseidon_3_partial_rounds_chain_1: Vec<[PackedM31; 42]>,
    poseidon_aggregator_0: Vec<[PackedM31; 6]>,
    poseidon_full_round_chain_0: Vec<[PackedM31; 32]>,
    poseidon_full_round_chain_1: Vec<[PackedM31; 32]>,
    poseidon_full_round_chain_2: Vec<[PackedM31; 32]>,
    poseidon_full_round_chain_3: Vec<[PackedM31; 32]>,
    range_check_252_width_27_0: Vec<[PackedM31; 10]>,
    range_check_252_width_27_1: Vec<[PackedM31; 10]>,
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
        memory_id_to_big: &relations::MemoryIdToBig,
        poseidon_full_round_chain: &relations::PoseidonFullRoundChain,
        range_check_252_width_27: &relations::RangeCheck252Width27,
        cube_252: &relations::Cube252,
        range_check_3_3_3_3_3: &relations::RangeCheck_3_3_3_3_3,
        range_check_4_4_4_4: &relations::RangeCheck_4_4_4_4,
        range_check_4_4: &relations::RangeCheck_4_4,
        poseidon_3_partial_rounds_chain: &relations::Poseidon3PartialRoundsChain,
        poseidon_aggregator: &relations::PoseidonAggregator,
    ) -> InteractionClaim {
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);

        // Sum logup terms in pairs.
        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_id_to_big_0,
            &self.lookup_data.memory_id_to_big_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_id_to_big.combine(values0);
                let denom1: PackedQM31 = memory_id_to_big.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_id_to_big_2,
            &self.lookup_data.poseidon_full_round_chain_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_id_to_big.combine(values0);
                let denom1: PackedQM31 = poseidon_full_round_chain.combine(values1);
                writer.write_frac(denom1 - denom0, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.poseidon_full_round_chain_1,
            &self.lookup_data.range_check_252_width_27_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = poseidon_full_round_chain.combine(values0);
                let denom1: PackedQM31 = range_check_252_width_27.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_252_width_27_1,
            &self.lookup_data.cube_252_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_252_width_27.combine(values0);
                let denom1: PackedQM31 = cube_252.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_3_3_3_3_3_0,
            &self.lookup_data.range_check_3_3_3_3_3_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_3_3_3_3_3.combine(values0);
                let denom1: PackedQM31 = range_check_3_3_3_3_3.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.cube_252_1,
            &self.lookup_data.range_check_4_4_4_4_0,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = cube_252.combine(values0);
                let denom1: PackedQM31 = range_check_4_4_4_4.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_4_4_4_4_1,
            &self.lookup_data.range_check_4_4_0,
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
            &self.lookup_data.poseidon_3_partial_rounds_chain_0,
            &self.lookup_data.poseidon_3_partial_rounds_chain_1,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = poseidon_3_partial_rounds_chain.combine(values0);
                let denom1: PackedQM31 = poseidon_3_partial_rounds_chain.combine(values1);
                writer.write_frac(denom0 - denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_4_4_4_4_2,
            &self.lookup_data.range_check_4_4_4_4_3,
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
            &self.lookup_data.range_check_4_4_1,
            &self.lookup_data.range_check_4_4_4_4_4,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = range_check_4_4.combine(values0);
                let denom1: PackedQM31 = range_check_4_4_4_4.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.range_check_4_4_4_4_5,
            &self.lookup_data.range_check_4_4_2,
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
            &self.lookup_data.poseidon_full_round_chain_2,
            &self.lookup_data.poseidon_full_round_chain_3,
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
            &self.lookup_data.memory_id_to_big_3,
            &self.lookup_data.memory_id_to_big_4,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1)| {
                let denom0: PackedQM31 = memory_id_to_big.combine(values0);
                let denom1: PackedQM31 = memory_id_to_big.combine(values1);
                writer.write_frac(denom0 + denom1, denom0 * denom1);
            });
        col_gen.finalize_col();

        let mut col_gen = logup_gen.new_col();
        (
            col_gen.par_iter_mut(),
            &self.lookup_data.memory_id_to_big_5,
            &self.lookup_data.poseidon_aggregator_0,
            self.lookup_data.mults,
        )
            .into_par_iter()
            .for_each(|(writer, values0, values1, mults)| {
                let denom0: PackedQM31 = memory_id_to_big.combine(values0);
                let denom1: PackedQM31 = poseidon_aggregator.combine(values1);
                writer.write_frac(denom1 - denom0 * mults, denom0 * denom1);
            });
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}
