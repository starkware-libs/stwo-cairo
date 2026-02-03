// This file was created by the AIR team.

use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 2;
pub const LOG_SIZE: u32 = 16;

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
        let mut verify_bitwise_xor_8_sum_0: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_b_sum_1: QM31 = Zero::zero();
        let bitwise_xor_8_0 = preprocessed_mask_values.get_and_mark_used(BITWISE_XOR_8_0_IDX);
        let bitwise_xor_8_1 = preprocessed_mask_values.get_and_mark_used(BITWISE_XOR_8_1_IDX);
        let bitwise_xor_8_2 = preprocessed_mask_values.get_and_mark_used(BITWISE_XOR_8_2_IDX);

        let [
            verify_bitwise_xor_8_multiplicity, verify_bitwise_xor_8_b_multiplicity,
        ]: [Span<QM31>; 2] =
            (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [verify_bitwise_xor_8_multiplicity]: [QM31; 1] = (*verify_bitwise_xor_8_multiplicity
            .try_into()
            .unwrap())
            .unbox();
        let [verify_bitwise_xor_8_b_multiplicity]: [QM31; 1] = (*verify_bitwise_xor_8_b_multiplicity
            .try_into()
            .unwrap())
            .unbox();

        core::internal::revoke_ap_tracking();

        verify_bitwise_xor_8_sum_0 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<112558620, 0, 0, 0>(), bitwise_xor_8_0, bitwise_xor_8_1,
                    bitwise_xor_8_2,
                ]
                    .span(),
            );

        verify_bitwise_xor_8_b_sum_1 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<521092554, 0, 0, 0>(), bitwise_xor_8_0, bitwise_xor_8_1,
                    bitwise_xor_8_2,
                ]
                    .span(),
            );

        lookup_constraints(
            ref sum,
            random_coeff,
            claimed_sum,
            verify_bitwise_xor_8_multiplicity,
            verify_bitwise_xor_8_b_multiplicity,
            column_size,
            ref interaction_trace_mask_values,
            verify_bitwise_xor_8_sum_0,
            verify_bitwise_xor_8_b_sum_1,
        );
    }
}


fn lookup_constraints(
    ref sum: QM31,
    random_coeff: QM31,
    claimed_sum: QM31,
    verify_bitwise_xor_8_multiplicity: QM31,
    verify_bitwise_xor_8_b_multiplicity: QM31,
    column_size: M31,
    ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
    verify_bitwise_xor_8_sum_0: QM31,
    verify_bitwise_xor_8_b_sum_1: QM31,
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
        * verify_bitwise_xor_8_sum_0
        * verify_bitwise_xor_8_b_sum_1)
        + (verify_bitwise_xor_8_sum_0 * verify_bitwise_xor_8_b_multiplicity)
        + (verify_bitwise_xor_8_b_sum_1 * verify_bitwise_xor_8_multiplicity));
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
            BITWISE_XOR_8_0_IDX,
            qm31_const::<1541997479, 760368373, 967211141, 1503140347>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            BITWISE_XOR_8_1_IDX,
            qm31_const::<1609106658, 894586101, 1034320005, 1503140347>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            BITWISE_XOR_8_2_IDX,
            qm31_const::<1407779121, 491932917, 832993413, 1503140347>(),
        );

        let mut trace_columns = [
            [qm31_const::<700269555, 307766862, 1685683780, 745982081>()].span(),
            [qm31_const::<633160376, 173549134, 1618574916, 745982081>()].span(),
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
        assert_eq!(sum, QM31Trait::from_fixed_array(VERIFY_BITWISE_XOR_8_SAMPLE_EVAL_RESULT))
    }
}
