// This file was created by the AIR team.

use crate::components::subroutines::decode_instruction_d2a10::decode_instruction_d2a10_evaluate;
use crate::components::subroutines::range_check_29::range_check_29_evaluate;
use crate::components::subroutines::read_small::read_small_evaluate;
use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 17;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 6] = [
    ('VerifyInstruction', 1), ('MemoryAddressToId', 1), ('MemoryIdToBig', 1), ('RangeCheck_18', 1),
    ('RangeCheck_11', 1), ('Opcodes', 1),
];

#[derive(Drop, Serde, Copy)]
pub struct Claim {
    pub log_size: u32,
}

pub impl ClaimImpl of ClaimTrait<Claim> {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = *(self.log_size);
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = [log_size; N_TRACE_COLUMNS].span();
        let interaction_log_sizes = [log_size; 16].span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        channel.mix_u64((*(self.log_size)).into());
    }

    fn accumulate_relation_uses(self: @Claim, ref relation_uses: RelationUsesDict) {
        accumulate_relation_uses(ref relation_uses, RELATION_USES_PER_ROW.span(), *self.log_size);
    }
}

#[derive(Drop, Serde, Copy)]
pub struct InteractionClaim {
    pub claimed_sum: QM31,
}

#[generate_trait]
pub impl InteractionClaimImpl of InteractionClaimTrait {
    fn mix_into(self: @InteractionClaim, ref channel: Channel) {
        channel.mix_felts([*self.claimed_sum].span());
    }
}


#[derive(Drop)]
pub struct Component {
    pub claim: Claim,
    pub interaction_claim: InteractionClaim,
    pub common_lookup_elements: CommonLookupElements,
}

pub impl NewComponentImpl of NewComponent<Component> {
    type Claim = Claim;
    type InteractionClaim = InteractionClaim;

    fn new(
        claim: @Claim,
        interaction_claim: @InteractionClaim,
        common_lookup_elements: @CommonLookupElements,
    ) -> Component {
        Component {
            claim: *claim,
            interaction_claim: *interaction_claim,
            common_lookup_elements: common_lookup_elements.clone(),
        }
    }
}

pub impl CairoComponentImpl of CairoComponent<Component> {
    fn evaluate_constraints_at_point(
        self: @Component,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
    ) {
        let log_size = *(self.claim.log_size);
        let claimed_sum = *self.interaction_claim.claimed_sum;
        let column_size = m31(pow2(log_size));
        let mut verify_instruction_sum_0: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_1: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_2: QM31 = Zero::zero();
        let mut range_check_18_sum_3: QM31 = Zero::zero();
        let mut range_check_11_sum_4: QM31 = Zero::zero();
        let mut opcodes_sum_5: QM31 = Zero::zero();
        let mut opcodes_sum_6: QM31 = Zero::zero();

        let [
            input_pc_col0,
            input_ap_col1,
            input_fp_col2,
            offset2_col3,
            op1_imm_col4,
            op1_base_fp_col5,
            mem1_base_col6,
            op1_id_col7,
            msb_col8,
            mid_limbs_set_col9,
            op1_limb_0_col10,
            op1_limb_1_col11,
            op1_limb_2_col12,
            remainder_bits_col13,
            partial_limb_msb_col14,
            range_check_29_bot11bits_col15,
            opcodes_multiplicity,
        ]: [Span<QM31>; 17] =
            (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [input_pc_col0]: [QM31; 1] = (*input_pc_col0.try_into().unwrap()).unbox();
        let [input_ap_col1]: [QM31; 1] = (*input_ap_col1.try_into().unwrap()).unbox();
        let [input_fp_col2]: [QM31; 1] = (*input_fp_col2.try_into().unwrap()).unbox();
        let [offset2_col3]: [QM31; 1] = (*offset2_col3.try_into().unwrap()).unbox();
        let [op1_imm_col4]: [QM31; 1] = (*op1_imm_col4.try_into().unwrap()).unbox();
        let [op1_base_fp_col5]: [QM31; 1] = (*op1_base_fp_col5.try_into().unwrap()).unbox();
        let [mem1_base_col6]: [QM31; 1] = (*mem1_base_col6.try_into().unwrap()).unbox();
        let [op1_id_col7]: [QM31; 1] = (*op1_id_col7.try_into().unwrap()).unbox();
        let [msb_col8]: [QM31; 1] = (*msb_col8.try_into().unwrap()).unbox();
        let [mid_limbs_set_col9]: [QM31; 1] = (*mid_limbs_set_col9.try_into().unwrap()).unbox();
        let [op1_limb_0_col10]: [QM31; 1] = (*op1_limb_0_col10.try_into().unwrap()).unbox();
        let [op1_limb_1_col11]: [QM31; 1] = (*op1_limb_1_col11.try_into().unwrap()).unbox();
        let [op1_limb_2_col12]: [QM31; 1] = (*op1_limb_2_col12.try_into().unwrap()).unbox();
        let [remainder_bits_col13]: [QM31; 1] = (*remainder_bits_col13.try_into().unwrap()).unbox();
        let [partial_limb_msb_col14]: [QM31; 1] = (*partial_limb_msb_col14.try_into().unwrap())
            .unbox();
        let [range_check_29_bot11bits_col15]: [QM31; 1] = (*range_check_29_bot11bits_col15
            .try_into()
            .unwrap())
            .unbox();
        let [opcodes_multiplicity]: [QM31; 1] = (*opcodes_multiplicity.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        let constraint_quotient = (opcodes_multiplicity * opcodes_multiplicity
            - opcodes_multiplicity);
        sum = sum * random_coeff + constraint_quotient;
        let [
            decode_instruction_d2a10_output_tmp_c921e_6_offset2,
            decode_instruction_d2a10_output_tmp_c921e_6_op1_base_ap,
        ] =
            decode_instruction_d2a10_evaluate(
            input_pc_col0,
            offset2_col3,
            op1_imm_col4,
            op1_base_fp_col5,
            self.common_lookup_elements,
            ref verify_instruction_sum_0,
            ref sum,
            random_coeff,
        );

        // Constraint - if imm then offset2 is 1
        let constraint_quotient = ((op1_imm_col4
            * (qm31_const::<1, 0, 0, 0>() - decode_instruction_d2a10_output_tmp_c921e_6_offset2)));
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - mem1_base
        let constraint_quotient = ((mem1_base_col6
            - (((op1_imm_col4 * input_pc_col0) + (op1_base_fp_col5 * input_fp_col2))
                + (decode_instruction_d2a10_output_tmp_c921e_6_op1_base_ap * input_ap_col1))));
        sum = sum * random_coeff + constraint_quotient;
        let read_small_output_tmp_c921e_16_limb_0: QM31 = read_small_evaluate(
            (mem1_base_col6 + decode_instruction_d2a10_output_tmp_c921e_6_offset2),
            op1_id_col7,
            msb_col8,
            mid_limbs_set_col9,
            op1_limb_0_col10,
            op1_limb_1_col11,
            op1_limb_2_col12,
            remainder_bits_col13,
            partial_limb_msb_col14,
            self.common_lookup_elements,
            ref memory_address_to_id_sum_1,
            ref memory_id_to_big_sum_2,
            ref sum,
            random_coeff,
        );
        let next_ap_tmp_c921e_17: QM31 = (input_ap_col1 + read_small_output_tmp_c921e_16_limb_0);
        range_check_29_evaluate(
            next_ap_tmp_c921e_17,
            range_check_29_bot11bits_col15,
            self.common_lookup_elements,
            ref range_check_18_sum_3,
            ref range_check_11_sum_4,
            ref sum,
            random_coeff,
        );

        opcodes_sum_5 = self
            .common_lookup_elements
            .combine_qm31(
                [qm31_const::<428564188, 0, 0, 0>(), input_pc_col0, input_ap_col1, input_fp_col2]
                    .span(),
            );

        opcodes_sum_6 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<428564188, 0, 0, 0>(),
                    (input_pc_col0 + (qm31_const::<1, 0, 0, 0>() + op1_imm_col4)),
                    next_ap_tmp_c921e_17, input_fp_col2,
                ]
                    .span(),
            );

        lookup_constraints(
            ref sum,
            random_coeff,
            claimed_sum,
            opcodes_multiplicity,
            column_size,
            ref interaction_trace_mask_values,
            verify_instruction_sum_0,
            memory_address_to_id_sum_1,
            memory_id_to_big_sum_2,
            range_check_18_sum_3,
            range_check_11_sum_4,
            opcodes_sum_5,
            opcodes_sum_6,
        );
    }
}


fn lookup_constraints(
    ref sum: QM31,
    random_coeff: QM31,
    claimed_sum: QM31,
    opcodes_multiplicity: QM31,
    column_size: M31,
    ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
    verify_instruction_sum_0: QM31,
    memory_address_to_id_sum_1: QM31,
    memory_id_to_big_sum_2: QM31,
    range_check_18_sum_3: QM31,
    range_check_11_sum_4: QM31,
    opcodes_sum_5: QM31,
    opcodes_sum_6: QM31,
) {
    let [
        trace_2_col0,
        trace_2_col1,
        trace_2_col2,
        trace_2_col3,
        trace_2_col4,
        trace_2_col5,
        trace_2_col6,
        trace_2_col7,
        trace_2_col8,
        trace_2_col9,
        trace_2_col10,
        trace_2_col11,
        trace_2_col12,
        trace_2_col13,
        trace_2_col14,
        trace_2_col15,
    ]: [Span<QM31>; 16] =
        (*interaction_trace_mask_values
        .multi_pop_front()
        .unwrap())
        .unbox();

    let [trace_2_col0]: [QM31; 1] = (*trace_2_col0.try_into().unwrap()).unbox();
    let [trace_2_col1]: [QM31; 1] = (*trace_2_col1.try_into().unwrap()).unbox();
    let [trace_2_col2]: [QM31; 1] = (*trace_2_col2.try_into().unwrap()).unbox();
    let [trace_2_col3]: [QM31; 1] = (*trace_2_col3.try_into().unwrap()).unbox();
    let [trace_2_col4]: [QM31; 1] = (*trace_2_col4.try_into().unwrap()).unbox();
    let [trace_2_col5]: [QM31; 1] = (*trace_2_col5.try_into().unwrap()).unbox();
    let [trace_2_col6]: [QM31; 1] = (*trace_2_col6.try_into().unwrap()).unbox();
    let [trace_2_col7]: [QM31; 1] = (*trace_2_col7.try_into().unwrap()).unbox();
    let [trace_2_col8]: [QM31; 1] = (*trace_2_col8.try_into().unwrap()).unbox();
    let [trace_2_col9]: [QM31; 1] = (*trace_2_col9.try_into().unwrap()).unbox();
    let [trace_2_col10]: [QM31; 1] = (*trace_2_col10.try_into().unwrap()).unbox();
    let [trace_2_col11]: [QM31; 1] = (*trace_2_col11.try_into().unwrap()).unbox();
    let [trace_2_col12_neg1, trace_2_col12]: [QM31; 2] = (*trace_2_col12.try_into().unwrap())
        .unbox();
    let [trace_2_col13_neg1, trace_2_col13]: [QM31; 2] = (*trace_2_col13.try_into().unwrap())
        .unbox();
    let [trace_2_col14_neg1, trace_2_col14]: [QM31; 2] = (*trace_2_col14.try_into().unwrap())
        .unbox();
    let [trace_2_col15_neg1, trace_2_col15]: [QM31; 2] = (*trace_2_col15.try_into().unwrap())
        .unbox();

    core::internal::revoke_ap_tracking();

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3],
    ))
        * verify_instruction_sum_0
        * memory_address_to_id_sum_1)
        - verify_instruction_sum_0
        - memory_address_to_id_sum_1);
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7],
    )
        - QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3]))
        * memory_id_to_big_sum_2
        * range_check_18_sum_3)
        - memory_id_to_big_sum_2
        - range_check_18_sum_3);
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11],
    )
        - QM31Impl::from_partial_evals([trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7]))
        * range_check_11_sum_4
        * opcodes_sum_5)
        - (range_check_11_sum_4 * opcodes_multiplicity)
        - opcodes_sum_5);
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
    )
        - QM31Impl::from_partial_evals([trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11])
        - QM31Impl::from_partial_evals(
            [trace_2_col12_neg1, trace_2_col13_neg1, trace_2_col14_neg1, trace_2_col15_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * opcodes_sum_6)
        + opcodes_multiplicity);
    sum = sum * random_coeff + constraint_quotient;
}
#[cfg(and(test, feature: "qm31_opcode"))]
mod tests {
    use core::array::ArrayImpl;
    use core::num::traits::Zero;
    #[allow(unused_imports)]
    use stwo_cairo_air::preprocessed_columns::*;
    #[allow(unused_imports)]
    use stwo_constraint_framework::{
        LookupElementsTrait, PreprocessedMaskValues, PreprocessedMaskValuesTrait,
    };
    use stwo_verifier_core::fields::qm31::{QM31, QM31Impl, QM31Trait, qm31_const};
    use crate::cairo_component::*;
    use crate::components::sample_evaluations::*;
    #[allow(unused_imports)]
    use crate::test_utils::{make_interaction_trace, preprocessed_mask_add};
    use crate::utils::*;
    use super::{Claim, Component, InteractionClaim};

    #[test]
    fn test_evaluation_result() {
        let component = Component {
            claim: Claim { log_size: 15 },
            interaction_claim: InteractionClaim {
                claimed_sum: qm31_const::<1398335417, 314974026, 1722107152, 821933968>(),
            },
            common_lookup_elements: LookupElementsTrait::from_z_alpha(
                qm31_const::<445623802, 202571636, 1360224996, 131355117>(),
                qm31_const::<476823935, 939223384, 62486082, 122423602>(),
            ),
        };
        let mut sum: QM31 = Zero::zero();

        let mut preprocessed_trace = PreprocessedMaskValues { values: Default::default() };

        let mut trace_columns = [
            [qm31_const::<1659099300, 905558730, 651199673, 1375009625>()].span(),
            [qm31_const::<1591990121, 771341002, 584090809, 1375009625>()].span(),
            [qm31_const::<1793317658, 1173994186, 785417401, 1375009625>()].span(),
            [qm31_const::<1726208479, 1039776458, 718308537, 1375009625>()].span(),
            [qm31_const::<1390662584, 368687818, 382764217, 1375009625>()].span(),
            [qm31_const::<1323553405, 234470090, 315655353, 1375009625>()].span(),
            [qm31_const::<1524880942, 637123274, 516981945, 1375009625>()].span(),
            [qm31_const::<1457771763, 502905546, 449873081, 1375009625>()].span(),
            [qm31_const::<48489085, 1979300555, 1188070585, 1375009625>()].span(),
            [qm31_const::<2128863553, 1845082826, 1120961721, 1375009625>()].span(),
            [qm31_const::<1852335767, 645078115, 2059236183, 343880121>()].span(),
            [qm31_const::<1919444946, 779295843, 2126345047, 343880121>()].span(),
            [qm31_const::<1986554125, 913513571, 45970264, 343880122>()].span(),
            [qm31_const::<2053663304, 1047731299, 113079128, 343880122>()].span(),
            [qm31_const::<1583899051, 108207203, 1790800727, 343880121>()].span(),
            [qm31_const::<1651008230, 242424931, 1857909591, 343880121>()].span(),
            [qm31_const::<902525010, 1115155995, 130434373, 2116865290>()].span(),
        ]
            .span();
        let interaction_values = array![
            qm31_const::<1005168032, 79980996, 1847888101, 1941984119>(),
            qm31_const::<1072277211, 214198724, 1914996965, 1941984119>(),
            qm31_const::<1139386390, 348416452, 1982105829, 1941984119>(),
            qm31_const::<1206495569, 482634180, 2049214693, 1941984119>(),
        ];
        let mut interaction_columns = make_interaction_trace(
            interaction_values, qm31_const::<1115374022, 1127856551, 489657863, 643630026>(),
        );
        component
            .evaluate_constraints_at_point(
                ref sum,
                ref preprocessed_trace,
                ref trace_columns,
                ref interaction_columns,
                qm31_const::<474642921, 876336632, 1911695779, 974600512>(),
            );
        preprocessed_trace.validate_usage();
        assert_eq!(sum, QM31Trait::from_fixed_array(ADD_AP_OPCODE_SAMPLE_EVAL_RESULT))
    }
}
