// This file was created by the AIR team.

use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 1;
pub const LOG_SIZE: u32 = 4;

#[derive(Drop, Serde, Copy)]
pub struct Claim {}

pub impl ClaimImpl of ClaimTrait<Claim> {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = LOG_SIZE;
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = [log_size; N_TRACE_COLUMNS].span();
        let interaction_log_sizes = [log_size; 4].span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {}

    fn accumulate_relation_uses(self: @Claim, ref relation_uses: RelationUsesDict) {}
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
        let log_size = LOG_SIZE;
        let claimed_sum = *self.interaction_claim.claimed_sum;
        let column_size = m31(pow2(log_size));
        let mut blake_round_sigma_sum_0: QM31 = Zero::zero();
        let mut numerator_0: QM31 = Zero::zero();
        let seq_4 = preprocessed_mask_values.get_and_mark_used(SEQ_4_IDX);
        let blake_sigma_0 = preprocessed_mask_values.get_and_mark_used(BLAKE_SIGMA_0_IDX);
        let blake_sigma_1 = preprocessed_mask_values.get_and_mark_used(BLAKE_SIGMA_1_IDX);
        let blake_sigma_2 = preprocessed_mask_values.get_and_mark_used(BLAKE_SIGMA_2_IDX);
        let blake_sigma_3 = preprocessed_mask_values.get_and_mark_used(BLAKE_SIGMA_3_IDX);
        let blake_sigma_4 = preprocessed_mask_values.get_and_mark_used(BLAKE_SIGMA_4_IDX);
        let blake_sigma_5 = preprocessed_mask_values.get_and_mark_used(BLAKE_SIGMA_5_IDX);
        let blake_sigma_6 = preprocessed_mask_values.get_and_mark_used(BLAKE_SIGMA_6_IDX);
        let blake_sigma_7 = preprocessed_mask_values.get_and_mark_used(BLAKE_SIGMA_7_IDX);
        let blake_sigma_8 = preprocessed_mask_values.get_and_mark_used(BLAKE_SIGMA_8_IDX);
        let blake_sigma_9 = preprocessed_mask_values.get_and_mark_used(BLAKE_SIGMA_9_IDX);
        let blake_sigma_10 = preprocessed_mask_values.get_and_mark_used(BLAKE_SIGMA_10_IDX);
        let blake_sigma_11 = preprocessed_mask_values.get_and_mark_used(BLAKE_SIGMA_11_IDX);
        let blake_sigma_12 = preprocessed_mask_values.get_and_mark_used(BLAKE_SIGMA_12_IDX);
        let blake_sigma_13 = preprocessed_mask_values.get_and_mark_used(BLAKE_SIGMA_13_IDX);
        let blake_sigma_14 = preprocessed_mask_values.get_and_mark_used(BLAKE_SIGMA_14_IDX);
        let blake_sigma_15 = preprocessed_mask_values.get_and_mark_used(BLAKE_SIGMA_15_IDX);

        let [multiplicity_0_col0]: [Span<QM31>; 1] = (*trace_mask_values.multi_pop_front().unwrap())
            .unbox();
        let [multiplicity_0_col0]: [QM31; 1] = (*multiplicity_0_col0.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        blake_round_sigma_sum_0 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<1805967942, 0, 0, 0>(), seq_4, blake_sigma_0, blake_sigma_1,
                    blake_sigma_2, blake_sigma_3, blake_sigma_4, blake_sigma_5, blake_sigma_6,
                    blake_sigma_7, blake_sigma_8, blake_sigma_9, blake_sigma_10, blake_sigma_11,
                    blake_sigma_12, blake_sigma_13, blake_sigma_14, blake_sigma_15,
                ]
                    .span(),
            );
        numerator_0 = multiplicity_0_col0;

        lookup_constraints(
            ref sum,
            random_coeff,
            claimed_sum,
            numerator_0,
            column_size,
            ref interaction_trace_mask_values,
            blake_round_sigma_sum_0,
        );
    }
}


fn lookup_constraints(
    ref sum: QM31,
    random_coeff: QM31,
    claimed_sum: QM31,
    numerator_0: QM31,
    column_size: M31,
    ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
    blake_round_sigma_sum_0: QM31,
) {
    let [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3]: [Span<QM31>; 4] =
        (*interaction_trace_mask_values
        .multi_pop_front()
        .unwrap())
        .unbox();

    let [trace_2_col0_neg1, trace_2_col0]: [QM31; 2] = (*trace_2_col0.try_into().unwrap()).unbox();
    let [trace_2_col1_neg1, trace_2_col1]: [QM31; 2] = (*trace_2_col1.try_into().unwrap()).unbox();
    let [trace_2_col2_neg1, trace_2_col2]: [QM31; 2] = (*trace_2_col2.try_into().unwrap()).unbox();
    let [trace_2_col3_neg1, trace_2_col3]: [QM31; 2] = (*trace_2_col3.try_into().unwrap()).unbox();

    core::internal::revoke_ap_tracking();

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col0_neg1, trace_2_col1_neg1, trace_2_col2_neg1, trace_2_col3_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * blake_round_sigma_sum_0)
        + numerator_0);
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
            claim: Claim {},
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
            SEQ_4_IDX,
            qm31_const::<763482793, 402222854, 1759975343, 865942395>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            BLAKE_SIGMA_0_IDX,
            qm31_const::<1541575468, 910566768, 1277642954, 337722398>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            BLAKE_SIGMA_1_IDX,
            qm31_const::<1474466289, 776349040, 1210534090, 337722398>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            BLAKE_SIGMA_2_IDX,
            qm31_const::<1407357110, 642131312, 1143425226, 337722398>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            BLAKE_SIGMA_3_IDX,
            qm31_const::<1340247931, 507913584, 1076316362, 337722398>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            BLAKE_SIGMA_4_IDX,
            qm31_const::<1810012184, 1447437680, 1546078410, 337722398>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            BLAKE_SIGMA_5_IDX,
            qm31_const::<1742903005, 1313219952, 1478969546, 337722398>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            BLAKE_SIGMA_6_IDX,
            qm31_const::<1675793826, 1179002224, 1411860682, 337722398>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            BLAKE_SIGMA_7_IDX,
            qm31_const::<1608684647, 1044784496, 1344751818, 337722398>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            BLAKE_SIGMA_8_IDX,
            qm31_const::<2078448900, 1984308592, 1814513866, 337722398>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            BLAKE_SIGMA_9_IDX,
            qm31_const::<2011339721, 1850090864, 1747405002, 337722398>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            BLAKE_SIGMA_10_IDX,
            qm31_const::<112615900, 18292853, 1092454797, 265412759>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            BLAKE_SIGMA_11_IDX,
            qm31_const::<179725079, 152510581, 1159563661, 265412759>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            BLAKE_SIGMA_12_IDX,
            qm31_const::<2125881189, 1897341043, 958237068, 265412759>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            BLAKE_SIGMA_13_IDX,
            qm31_const::<45506721, 2031558772, 1025345932, 265412759>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            BLAKE_SIGMA_14_IDX,
            qm31_const::<1991662831, 1628905587, 824019340, 265412759>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            BLAKE_SIGMA_15_IDX,
            qm31_const::<2058772010, 1763123315, 891128204, 265412759>(),
        );

        let mut trace_columns = [
            [qm31_const::<700269555, 307766862, 1685683780, 745982081>()].span(),
        ]
            .span();
        let interaction_values = array![
            qm31_const::<1005168032, 79980996, 1847888101, 1941984119>(),
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
        assert_eq!(sum, QM31Trait::from_fixed_array(BLAKE_ROUND_SIGMA_SAMPLE_EVAL_RESULT))
    }
}
