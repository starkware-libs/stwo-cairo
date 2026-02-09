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
        let mut pedersen_points_table_window_bits_9_sum_0: QM31 = Zero::zero();
        let seq_15 = preprocessed_mask_values.get_and_mark_used(SEQ_15_IDX);
        let pedersen_points_small_0 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_0_IDX);
        let pedersen_points_small_1 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_1_IDX);
        let pedersen_points_small_2 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_2_IDX);
        let pedersen_points_small_3 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_3_IDX);
        let pedersen_points_small_4 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_4_IDX);
        let pedersen_points_small_5 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_5_IDX);
        let pedersen_points_small_6 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_6_IDX);
        let pedersen_points_small_7 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_7_IDX);
        let pedersen_points_small_8 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_8_IDX);
        let pedersen_points_small_9 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_9_IDX);
        let pedersen_points_small_10 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_10_IDX);
        let pedersen_points_small_11 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_11_IDX);
        let pedersen_points_small_12 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_12_IDX);
        let pedersen_points_small_13 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_13_IDX);
        let pedersen_points_small_14 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_14_IDX);
        let pedersen_points_small_15 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_15_IDX);
        let pedersen_points_small_16 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_16_IDX);
        let pedersen_points_small_17 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_17_IDX);
        let pedersen_points_small_18 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_18_IDX);
        let pedersen_points_small_19 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_19_IDX);
        let pedersen_points_small_20 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_20_IDX);
        let pedersen_points_small_21 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_21_IDX);
        let pedersen_points_small_22 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_22_IDX);
        let pedersen_points_small_23 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_23_IDX);
        let pedersen_points_small_24 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_24_IDX);
        let pedersen_points_small_25 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_25_IDX);
        let pedersen_points_small_26 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_26_IDX);
        let pedersen_points_small_27 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_27_IDX);
        let pedersen_points_small_28 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_28_IDX);
        let pedersen_points_small_29 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_29_IDX);
        let pedersen_points_small_30 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_30_IDX);
        let pedersen_points_small_31 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_31_IDX);
        let pedersen_points_small_32 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_32_IDX);
        let pedersen_points_small_33 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_33_IDX);
        let pedersen_points_small_34 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_34_IDX);
        let pedersen_points_small_35 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_35_IDX);
        let pedersen_points_small_36 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_36_IDX);
        let pedersen_points_small_37 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_37_IDX);
        let pedersen_points_small_38 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_38_IDX);
        let pedersen_points_small_39 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_39_IDX);
        let pedersen_points_small_40 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_40_IDX);
        let pedersen_points_small_41 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_41_IDX);
        let pedersen_points_small_42 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_42_IDX);
        let pedersen_points_small_43 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_43_IDX);
        let pedersen_points_small_44 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_44_IDX);
        let pedersen_points_small_45 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_45_IDX);
        let pedersen_points_small_46 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_46_IDX);
        let pedersen_points_small_47 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_47_IDX);
        let pedersen_points_small_48 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_48_IDX);
        let pedersen_points_small_49 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_49_IDX);
        let pedersen_points_small_50 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_50_IDX);
        let pedersen_points_small_51 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_51_IDX);
        let pedersen_points_small_52 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_52_IDX);
        let pedersen_points_small_53 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_53_IDX);
        let pedersen_points_small_54 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_54_IDX);
        let pedersen_points_small_55 = preprocessed_mask_values
            .get_and_mark_used(PEDERSEN_POINTS_SMALL_55_IDX);

        let [pedersen_points_table_window_bits_9_multiplicity]: [Span<QM31>; 1] =
            (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [pedersen_points_table_window_bits_9_multiplicity]: [QM31; 1] =
            (*pedersen_points_table_window_bits_9_multiplicity
            .try_into()
            .unwrap())
            .unbox();

        core::internal::revoke_ap_tracking();

        pedersen_points_table_window_bits_9_sum_0 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<1791500038, 0, 0, 0>(), seq_15, pedersen_points_small_0,
                    pedersen_points_small_1, pedersen_points_small_2, pedersen_points_small_3,
                    pedersen_points_small_4, pedersen_points_small_5, pedersen_points_small_6,
                    pedersen_points_small_7, pedersen_points_small_8, pedersen_points_small_9,
                    pedersen_points_small_10, pedersen_points_small_11, pedersen_points_small_12,
                    pedersen_points_small_13, pedersen_points_small_14, pedersen_points_small_15,
                    pedersen_points_small_16, pedersen_points_small_17, pedersen_points_small_18,
                    pedersen_points_small_19, pedersen_points_small_20, pedersen_points_small_21,
                    pedersen_points_small_22, pedersen_points_small_23, pedersen_points_small_24,
                    pedersen_points_small_25, pedersen_points_small_26, pedersen_points_small_27,
                    pedersen_points_small_28, pedersen_points_small_29, pedersen_points_small_30,
                    pedersen_points_small_31, pedersen_points_small_32, pedersen_points_small_33,
                    pedersen_points_small_34, pedersen_points_small_35, pedersen_points_small_36,
                    pedersen_points_small_37, pedersen_points_small_38, pedersen_points_small_39,
                    pedersen_points_small_40, pedersen_points_small_41, pedersen_points_small_42,
                    pedersen_points_small_43, pedersen_points_small_44, pedersen_points_small_45,
                    pedersen_points_small_46, pedersen_points_small_47, pedersen_points_small_48,
                    pedersen_points_small_49, pedersen_points_small_50, pedersen_points_small_51,
                    pedersen_points_small_52, pedersen_points_small_53, pedersen_points_small_54,
                    pedersen_points_small_55,
                ]
                    .span(),
            );

        lookup_constraints(
            ref sum,
            random_coeff,
            claimed_sum,
            pedersen_points_table_window_bits_9_multiplicity,
            column_size,
            ref interaction_trace_mask_values,
            pedersen_points_table_window_bits_9_sum_0,
        );
    }
}


fn lookup_constraints(
    ref sum: QM31,
    random_coeff: QM31,
    claimed_sum: QM31,
    pedersen_points_table_window_bits_9_multiplicity: QM31,
    column_size: M31,
    ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
    pedersen_points_table_window_bits_9_sum_0: QM31,
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
        * pedersen_points_table_window_bits_9_sum_0)
        + pedersen_points_table_window_bits_9_multiplicity);
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
            SEQ_15_IDX,
            qm31_const::<1561133224, 586108960, 1063114695, 1758380471>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_0_IDX,
            qm31_const::<1071331976, 281835572, 1927130496, 1936455919>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_1_IDX,
            qm31_const::<1138441155, 416053300, 1994239360, 1936455919>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_2_IDX,
            qm31_const::<937113618, 13400116, 1792912768, 1936455919>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_3_IDX,
            qm31_const::<1004222797, 147617844, 1860021632, 1936455919>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_4_IDX,
            qm31_const::<1339768692, 818706484, 48082305, 1936455920>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_5_IDX,
            qm31_const::<1406877871, 952924212, 115191169, 1936455920>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_6_IDX,
            qm31_const::<1205550334, 550271028, 2061348224, 1936455919>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_7_IDX,
            qm31_const::<1272659513, 684488756, 2128457088, 1936455919>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_8_IDX,
            qm31_const::<1608205408, 1355577396, 316517761, 1936455920>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_9_IDX,
            qm31_const::<1675314587, 1489795124, 383626625, 1936455920>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_10_IDX,
            qm31_const::<943093592, 1811147488, 1730181953, 2068794068>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_11_IDX,
            qm31_const::<875984413, 1676929760, 1663073089, 2068794068>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_12_IDX,
            qm31_const::<1077311950, 2079582944, 1864399681, 2068794068>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_13_IDX,
            qm31_const::<1010202771, 1945365216, 1797290817, 2068794068>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_14_IDX,
            qm31_const::<1211530308, 200534753, 1998617410, 2068794068>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_15_IDX,
            qm31_const::<1144421129, 66317025, 1931508546, 2068794068>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_16_IDX,
            qm31_const::<1345748666, 468970209, 2132835138, 2068794068>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_17_IDX,
            qm31_const::<1278639487, 334752481, 2065726274, 2068794068>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_18_IDX,
            qm31_const::<1479967024, 737405665, 119569219, 2068794069>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_19_IDX,
            qm31_const::<1412857845, 603187937, 52460355, 2068794069>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_20_IDX,
            qm31_const::<808568049, 1542711852, 1595964045, 2068794008>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_21_IDX,
            qm31_const::<875677228, 1676929580, 1663072909, 2068794008>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_22_IDX,
            qm31_const::<942786407, 1811147308, 1730181773, 2068794008>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_23_IDX,
            qm31_const::<1009895586, 1945365036, 1797290637, 2068794008>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_24_IDX,
            qm31_const::<1077004765, 2079582764, 1864399501, 2068794008>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_25_IDX,
            qm31_const::<1144113944, 66316845, 1931508366, 2068794008>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_26_IDX,
            qm31_const::<1211223123, 200534573, 1998617230, 2068794008>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_27_IDX,
            qm31_const::<1278332302, 334752301, 2065726094, 2068794008>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_28_IDX,
            qm31_const::<1345441481, 468970029, 2132834958, 2068794008>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_29_IDX,
            qm31_const::<1412550660, 603187757, 52460175, 2068794009>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_30_IDX,
            qm31_const::<137588735, 200534635, 924875468, 2068794029>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_31_IDX,
            qm31_const::<70479556, 66316907, 857766604, 2068794029>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_32_IDX,
            qm31_const::<3370377, 2079582826, 790657739, 2068794029>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_33_IDX,
            qm31_const::<2083744845, 1945365097, 723548875, 2068794029>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_34_IDX,
            qm31_const::<406025451, 737405547, 1193310924, 2068794029>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_35_IDX,
            qm31_const::<338916272, 603187819, 1126202060, 2068794029>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_36_IDX,
            qm31_const::<271807093, 468970091, 1059093196, 2068794029>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_37_IDX,
            qm31_const::<204697914, 334752363, 991984332, 2068794029>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_38_IDX,
            qm31_const::<1748198950, 1274276457, 388004555, 2068794029>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_39_IDX,
            qm31_const::<1681089771, 1140058729, 320895691, 2068794029>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_40_IDX,
            qm31_const::<1077619135, 2079583124, 1864399861, 2068794128>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_41_IDX,
            qm31_const::<1144728314, 66317205, 1931508726, 2068794128>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_42_IDX,
            qm31_const::<943400777, 1811147668, 1730182133, 2068794128>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_43_IDX,
            qm31_const::<1010509956, 1945365396, 1797290997, 2068794128>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_44_IDX,
            qm31_const::<1346055851, 468970389, 2132835318, 2068794128>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_45_IDX,
            qm31_const::<1413165030, 603188117, 52460535, 2068794129>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_46_IDX,
            qm31_const::<1211837493, 200534933, 1998617590, 2068794128>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_47_IDX,
            qm31_const::<1278946672, 334752661, 2065726454, 2068794128>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_48_IDX,
            qm31_const::<1614492567, 1005841301, 253787127, 2068794129>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_49_IDX,
            qm31_const::<1681601746, 1140059029, 320895991, 2068794129>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_50_IDX,
            qm31_const::<406639821, 737405907, 1193311284, 2068794149>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_51_IDX,
            qm31_const::<339530642, 603188179, 1126202420, 2068794149>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_52_IDX,
            qm31_const::<540858179, 1005841363, 1327529012, 2068794149>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_53_IDX,
            qm31_const::<473749000, 871623635, 1260420148, 2068794149>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_54_IDX,
            qm31_const::<138203105, 200534995, 924875828, 2068794149>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_SMALL_55_IDX,
            qm31_const::<71093926, 66317267, 857766964, 2068794149>(),
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
        assert_eq!(
            sum,
            QM31Trait::from_fixed_array(PEDERSEN_POINTS_TABLE_WINDOW_BITS_9_SAMPLE_EVAL_RESULT),
        )
    }
}
