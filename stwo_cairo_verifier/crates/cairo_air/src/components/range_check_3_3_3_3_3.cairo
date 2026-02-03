// This file was created by the AIR team.

use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 1;
pub const LOG_SIZE: u32 = 15;

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
        let mut range_check_3_3_3_3_3_sum_0: QM31 = Zero::zero();
        let range_check_3_3_3_3_3_column_0 = preprocessed_mask_values
            .get_and_mark_used(RANGE_CHECK_3_3_3_3_3_COLUMN_0_IDX);
        let range_check_3_3_3_3_3_column_1 = preprocessed_mask_values
            .get_and_mark_used(RANGE_CHECK_3_3_3_3_3_COLUMN_1_IDX);
        let range_check_3_3_3_3_3_column_2 = preprocessed_mask_values
            .get_and_mark_used(RANGE_CHECK_3_3_3_3_3_COLUMN_2_IDX);
        let range_check_3_3_3_3_3_column_3 = preprocessed_mask_values
            .get_and_mark_used(RANGE_CHECK_3_3_3_3_3_COLUMN_3_IDX);
        let range_check_3_3_3_3_3_column_4 = preprocessed_mask_values
            .get_and_mark_used(RANGE_CHECK_3_3_3_3_3_COLUMN_4_IDX);

        let [range_check_3_3_3_3_3_multiplicity]: [Span<QM31>; 1] = (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [range_check_3_3_3_3_3_multiplicity]: [QM31; 1] = (*range_check_3_3_3_3_3_multiplicity
            .try_into()
            .unwrap())
            .unbox();

        core::internal::revoke_ap_tracking();

        range_check_3_3_3_3_3_sum_0 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<502259093, 0, 0, 0>(), range_check_3_3_3_3_3_column_0,
                    range_check_3_3_3_3_3_column_1, range_check_3_3_3_3_3_column_2,
                    range_check_3_3_3_3_3_column_3, range_check_3_3_3_3_3_column_4,
                ]
                    .span(),
            );

        lookup_constraints(
            ref sum,
            random_coeff,
            claimed_sum,
            range_check_3_3_3_3_3_multiplicity,
            column_size,
            ref interaction_trace_mask_values,
            range_check_3_3_3_3_3_sum_0,
        );
    }
}


fn lookup_constraints(
    ref sum: QM31,
    random_coeff: QM31,
    claimed_sum: QM31,
    range_check_3_3_3_3_3_multiplicity: QM31,
    column_size: M31,
    ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
    range_check_3_3_3_3_3_sum_0: QM31,
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
        * range_check_3_3_3_3_3_sum_0)
        + range_check_3_3_3_3_3_multiplicity);
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
            RANGE_CHECK_3_3_3_3_3_COLUMN_0_IDX,
            qm31_const::<489832784, 37569125, 1395818842, 1438798122>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            RANGE_CHECK_3_3_3_3_3_COLUMN_1_IDX,
            qm31_const::<422723605, 2050835044, 1328709977, 1438798122>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            RANGE_CHECK_3_3_3_3_3_COLUMN_2_IDX,
            qm31_const::<355614426, 1916617316, 1261601113, 1438798122>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            RANGE_CHECK_3_3_3_3_3_COLUMN_3_IDX,
            qm31_const::<288505247, 1782399588, 1194492249, 1438798122>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            RANGE_CHECK_3_3_3_3_3_COLUMN_4_IDX,
            qm31_const::<758269500, 574440037, 1664254298, 1438798122>(),
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
        assert_eq!(sum, QM31Trait::from_fixed_array(RANGE_CHECK_3_3_3_3_3_SAMPLE_EVAL_RESULT))
    }
}
