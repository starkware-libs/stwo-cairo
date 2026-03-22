// This file was created by the AIR team.

use crate::components::subroutines::mem_verify::mem_verify_evaluate;
use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 29;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 3] = [
    ('ProgramComponent', 1), ('MemoryAddressToId', 1), ('MemoryIdToBig', 1),
];

#[derive(Drop, Serde, Copy)]
pub struct Claim {
    pub log_size: u32,
    pub verify_program_segment_start: u32,
}

pub impl ClaimImpl of ClaimTrait<Claim> {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = *(self.log_size);
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = [log_size; N_TRACE_COLUMNS].span();
        let interaction_log_sizes = [log_size; 8].span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        channel.mix_u64((*(self.log_size)).into());
        channel.mix_u64((*self.verify_program_segment_start).into());
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
        let verify_program_segment_start: QM31 = (TryInto::<
            u32, M31,
        >::try_into((*(self.claim.verify_program_segment_start)))
            .unwrap())
            .into();
        let mut program_component_sum_0: QM31 = Zero::zero();
        let mut numerator_0: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_1: QM31 = Zero::zero();
        let mut numerator_1: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_2: QM31 = Zero::zero();
        let mut numerator_2: QM31 = Zero::zero();
        let seq = preprocessed_mask_values
            .get_and_mark_used(seq_column_idx(*(self.claim.log_size)));

        let [
            program_component_output_limb_0_col0,
            program_component_output_limb_1_col1,
            program_component_output_limb_2_col2,
            program_component_output_limb_3_col3,
            program_component_output_limb_4_col4,
            program_component_output_limb_5_col5,
            program_component_output_limb_6_col6,
            program_component_output_limb_7_col7,
            program_component_output_limb_8_col8,
            program_component_output_limb_9_col9,
            program_component_output_limb_10_col10,
            program_component_output_limb_11_col11,
            program_component_output_limb_12_col12,
            program_component_output_limb_13_col13,
            program_component_output_limb_14_col14,
            program_component_output_limb_15_col15,
            program_component_output_limb_16_col16,
            program_component_output_limb_17_col17,
            program_component_output_limb_18_col18,
            program_component_output_limb_19_col19,
            program_component_output_limb_20_col20,
            program_component_output_limb_21_col21,
            program_component_output_limb_22_col22,
            program_component_output_limb_23_col23,
            program_component_output_limb_24_col24,
            program_component_output_limb_25_col25,
            program_component_output_limb_26_col26,
            program_component_output_limb_27_col27,
            address_id_col28,
        ]: [Span<QM31>; 29] =
            (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [program_component_output_limb_0_col0]: [QM31; 1] =
            (*program_component_output_limb_0_col0
            .try_into()
            .unwrap())
            .unbox();
        let [program_component_output_limb_1_col1]: [QM31; 1] =
            (*program_component_output_limb_1_col1
            .try_into()
            .unwrap())
            .unbox();
        let [program_component_output_limb_2_col2]: [QM31; 1] =
            (*program_component_output_limb_2_col2
            .try_into()
            .unwrap())
            .unbox();
        let [program_component_output_limb_3_col3]: [QM31; 1] =
            (*program_component_output_limb_3_col3
            .try_into()
            .unwrap())
            .unbox();
        let [program_component_output_limb_4_col4]: [QM31; 1] =
            (*program_component_output_limb_4_col4
            .try_into()
            .unwrap())
            .unbox();
        let [program_component_output_limb_5_col5]: [QM31; 1] =
            (*program_component_output_limb_5_col5
            .try_into()
            .unwrap())
            .unbox();
        let [program_component_output_limb_6_col6]: [QM31; 1] =
            (*program_component_output_limb_6_col6
            .try_into()
            .unwrap())
            .unbox();
        let [program_component_output_limb_7_col7]: [QM31; 1] =
            (*program_component_output_limb_7_col7
            .try_into()
            .unwrap())
            .unbox();
        let [program_component_output_limb_8_col8]: [QM31; 1] =
            (*program_component_output_limb_8_col8
            .try_into()
            .unwrap())
            .unbox();
        let [program_component_output_limb_9_col9]: [QM31; 1] =
            (*program_component_output_limb_9_col9
            .try_into()
            .unwrap())
            .unbox();
        let [program_component_output_limb_10_col10]: [QM31; 1] =
            (*program_component_output_limb_10_col10
            .try_into()
            .unwrap())
            .unbox();
        let [program_component_output_limb_11_col11]: [QM31; 1] =
            (*program_component_output_limb_11_col11
            .try_into()
            .unwrap())
            .unbox();
        let [program_component_output_limb_12_col12]: [QM31; 1] =
            (*program_component_output_limb_12_col12
            .try_into()
            .unwrap())
            .unbox();
        let [program_component_output_limb_13_col13]: [QM31; 1] =
            (*program_component_output_limb_13_col13
            .try_into()
            .unwrap())
            .unbox();
        let [program_component_output_limb_14_col14]: [QM31; 1] =
            (*program_component_output_limb_14_col14
            .try_into()
            .unwrap())
            .unbox();
        let [program_component_output_limb_15_col15]: [QM31; 1] =
            (*program_component_output_limb_15_col15
            .try_into()
            .unwrap())
            .unbox();
        let [program_component_output_limb_16_col16]: [QM31; 1] =
            (*program_component_output_limb_16_col16
            .try_into()
            .unwrap())
            .unbox();
        let [program_component_output_limb_17_col17]: [QM31; 1] =
            (*program_component_output_limb_17_col17
            .try_into()
            .unwrap())
            .unbox();
        let [program_component_output_limb_18_col18]: [QM31; 1] =
            (*program_component_output_limb_18_col18
            .try_into()
            .unwrap())
            .unbox();
        let [program_component_output_limb_19_col19]: [QM31; 1] =
            (*program_component_output_limb_19_col19
            .try_into()
            .unwrap())
            .unbox();
        let [program_component_output_limb_20_col20]: [QM31; 1] =
            (*program_component_output_limb_20_col20
            .try_into()
            .unwrap())
            .unbox();
        let [program_component_output_limb_21_col21]: [QM31; 1] =
            (*program_component_output_limb_21_col21
            .try_into()
            .unwrap())
            .unbox();
        let [program_component_output_limb_22_col22]: [QM31; 1] =
            (*program_component_output_limb_22_col22
            .try_into()
            .unwrap())
            .unbox();
        let [program_component_output_limb_23_col23]: [QM31; 1] =
            (*program_component_output_limb_23_col23
            .try_into()
            .unwrap())
            .unbox();
        let [program_component_output_limb_24_col24]: [QM31; 1] =
            (*program_component_output_limb_24_col24
            .try_into()
            .unwrap())
            .unbox();
        let [program_component_output_limb_25_col25]: [QM31; 1] =
            (*program_component_output_limb_25_col25
            .try_into()
            .unwrap())
            .unbox();
        let [program_component_output_limb_26_col26]: [QM31; 1] =
            (*program_component_output_limb_26_col26
            .try_into()
            .unwrap())
            .unbox();
        let [program_component_output_limb_27_col27]: [QM31; 1] =
            (*program_component_output_limb_27_col27
            .try_into()
            .unwrap())
            .unbox();
        let [address_id_col28]: [QM31; 1] = (*address_id_col28.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        program_component_sum_0 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<1942035206, 0, 0, 0>(), seq, program_component_output_limb_0_col0,
                    program_component_output_limb_1_col1, program_component_output_limb_2_col2,
                    program_component_output_limb_3_col3, program_component_output_limb_4_col4,
                    program_component_output_limb_5_col5, program_component_output_limb_6_col6,
                    program_component_output_limb_7_col7, program_component_output_limb_8_col8,
                    program_component_output_limb_9_col9, program_component_output_limb_10_col10,
                    program_component_output_limb_11_col11, program_component_output_limb_12_col12,
                    program_component_output_limb_13_col13, program_component_output_limb_14_col14,
                    program_component_output_limb_15_col15, program_component_output_limb_16_col16,
                    program_component_output_limb_17_col17, program_component_output_limb_18_col18,
                    program_component_output_limb_19_col19, program_component_output_limb_20_col20,
                    program_component_output_limb_21_col21, program_component_output_limb_22_col22,
                    program_component_output_limb_23_col23, program_component_output_limb_24_col24,
                    program_component_output_limb_25_col25, program_component_output_limb_26_col26,
                    program_component_output_limb_27_col27,
                ]
                    .span(),
            );
        numerator_0 = qm31_const::<1, 0, 0, 0>();
        mem_verify_evaluate(
            [
                (verify_program_segment_start + seq), program_component_output_limb_0_col0,
                program_component_output_limb_1_col1, program_component_output_limb_2_col2,
                program_component_output_limb_3_col3, program_component_output_limb_4_col4,
                program_component_output_limb_5_col5, program_component_output_limb_6_col6,
                program_component_output_limb_7_col7, program_component_output_limb_8_col8,
                program_component_output_limb_9_col9, program_component_output_limb_10_col10,
                program_component_output_limb_11_col11, program_component_output_limb_12_col12,
                program_component_output_limb_13_col13, program_component_output_limb_14_col14,
                program_component_output_limb_15_col15, program_component_output_limb_16_col16,
                program_component_output_limb_17_col17, program_component_output_limb_18_col18,
                program_component_output_limb_19_col19, program_component_output_limb_20_col20,
                program_component_output_limb_21_col21, program_component_output_limb_22_col22,
                program_component_output_limb_23_col23, program_component_output_limb_24_col24,
                program_component_output_limb_25_col25, program_component_output_limb_26_col26,
                program_component_output_limb_27_col27,
            ],
            address_id_col28,
            self.common_lookup_elements,
            ref memory_address_to_id_sum_1,
            ref numerator_1,
            ref memory_id_to_big_sum_2,
            ref numerator_2,
            ref sum,
            random_coeff,
        );

        lookup_constraints(
            ref sum,
            random_coeff,
            claimed_sum,
            numerator_0,
            numerator_1,
            numerator_2,
            column_size,
            ref interaction_trace_mask_values,
            program_component_sum_0,
            memory_address_to_id_sum_1,
            memory_id_to_big_sum_2,
        );
    }
}


fn lookup_constraints(
    ref sum: QM31,
    random_coeff: QM31,
    claimed_sum: QM31,
    numerator_0: QM31,
    numerator_1: QM31,
    numerator_2: QM31,
    column_size: M31,
    ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
    program_component_sum_0: QM31,
    memory_address_to_id_sum_1: QM31,
    memory_id_to_big_sum_2: QM31,
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
    ]: [Span<QM31>; 8] =
        (*interaction_trace_mask_values
        .multi_pop_front()
        .unwrap())
        .unbox();

    let [trace_2_col0]: [QM31; 1] = (*trace_2_col0.try_into().unwrap()).unbox();
    let [trace_2_col1]: [QM31; 1] = (*trace_2_col1.try_into().unwrap()).unbox();
    let [trace_2_col2]: [QM31; 1] = (*trace_2_col2.try_into().unwrap()).unbox();
    let [trace_2_col3]: [QM31; 1] = (*trace_2_col3.try_into().unwrap()).unbox();
    let [trace_2_col4_neg1, trace_2_col4]: [QM31; 2] = (*trace_2_col4.try_into().unwrap()).unbox();
    let [trace_2_col5_neg1, trace_2_col5]: [QM31; 2] = (*trace_2_col5.try_into().unwrap()).unbox();
    let [trace_2_col6_neg1, trace_2_col6]: [QM31; 2] = (*trace_2_col6.try_into().unwrap()).unbox();
    let [trace_2_col7_neg1, trace_2_col7]: [QM31; 2] = (*trace_2_col7.try_into().unwrap()).unbox();

    core::internal::revoke_ap_tracking();

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3],
    ))
        * program_component_sum_0
        * memory_address_to_id_sum_1)
        - (program_component_sum_0 * numerator_1)
        - (memory_address_to_id_sum_1 * numerator_0));
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7],
    )
        - QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3])
        - QM31Impl::from_partial_evals(
            [trace_2_col4_neg1, trace_2_col5_neg1, trace_2_col6_neg1, trace_2_col7_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * memory_id_to_big_sum_2)
        - numerator_2);
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
            claim: Claim { log_size: 15, verify_program_segment_start: 1315548426 },
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
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            seq_column_idx(component.claim.log_size),
            qm31_const::<735272696, 1215403647, 795393303, 879304430>(),
        );

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
            [qm31_const::<1718117409, 376642659, 1925018455, 343880121>()].span(),
            [qm31_const::<1785226588, 510860387, 1992127319, 343880121>()].span(),
            [qm31_const::<1315462335, 1718819938, 1522365270, 343880121>()].span(),
            [qm31_const::<1382571514, 1853037666, 1589474134, 343880121>()].span(),
            [qm31_const::<1986820986, 913513739, 45970432, 343880178>()].span(),
            [qm31_const::<1919711807, 779296011, 2126345215, 343880177>()].span(),
            [qm31_const::<2121039344, 1181949195, 180188160, 343880178>()].span(),
            [qm31_const::<2053930165, 1047731467, 113079296, 343880178>()].span(),
            [qm31_const::<1718384270, 376642827, 1925018623, 343880177>()].span(),
            [qm31_const::<1651275091, 242425099, 1857909759, 343880177>()].span(),
            [qm31_const::<1852602628, 645078283, 2059236351, 343880177>()].span(),
            [qm31_const::<1785493449, 510860555, 1992127487, 343880177>()].span(),
            [qm31_const::<1449947554, 1987255562, 1656583166, 343880177>()].span(),
        ]
            .span();
        let interaction_values = array![
            qm31_const::<1005168032, 79980996, 1847888101, 1941984119>(),
            qm31_const::<1072277211, 214198724, 1914996965, 1941984119>(),
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
        assert_eq!(sum, QM31Trait::from_fixed_array(VERIFY_PROGRAM_SAMPLE_EVAL_RESULT))
    }
}
