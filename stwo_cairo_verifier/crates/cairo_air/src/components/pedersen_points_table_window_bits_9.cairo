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
        let pedersen_points_0 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_0_IDX);
        let pedersen_points_1 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_1_IDX);
        let pedersen_points_2 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_2_IDX);
        let pedersen_points_3 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_3_IDX);
        let pedersen_points_4 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_4_IDX);
        let pedersen_points_5 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_5_IDX);
        let pedersen_points_6 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_6_IDX);
        let pedersen_points_7 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_7_IDX);
        let pedersen_points_8 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_8_IDX);
        let pedersen_points_9 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_9_IDX);
        let pedersen_points_10 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_10_IDX);
        let pedersen_points_11 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_11_IDX);
        let pedersen_points_12 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_12_IDX);
        let pedersen_points_13 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_13_IDX);
        let pedersen_points_14 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_14_IDX);
        let pedersen_points_15 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_15_IDX);
        let pedersen_points_16 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_16_IDX);
        let pedersen_points_17 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_17_IDX);
        let pedersen_points_18 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_18_IDX);
        let pedersen_points_19 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_19_IDX);
        let pedersen_points_20 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_20_IDX);
        let pedersen_points_21 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_21_IDX);
        let pedersen_points_22 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_22_IDX);
        let pedersen_points_23 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_23_IDX);
        let pedersen_points_24 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_24_IDX);
        let pedersen_points_25 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_25_IDX);
        let pedersen_points_26 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_26_IDX);
        let pedersen_points_27 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_27_IDX);
        let pedersen_points_28 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_28_IDX);
        let pedersen_points_29 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_29_IDX);
        let pedersen_points_30 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_30_IDX);
        let pedersen_points_31 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_31_IDX);
        let pedersen_points_32 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_32_IDX);
        let pedersen_points_33 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_33_IDX);
        let pedersen_points_34 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_34_IDX);
        let pedersen_points_35 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_35_IDX);
        let pedersen_points_36 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_36_IDX);
        let pedersen_points_37 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_37_IDX);
        let pedersen_points_38 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_38_IDX);
        let pedersen_points_39 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_39_IDX);
        let pedersen_points_40 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_40_IDX);
        let pedersen_points_41 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_41_IDX);
        let pedersen_points_42 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_42_IDX);
        let pedersen_points_43 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_43_IDX);
        let pedersen_points_44 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_44_IDX);
        let pedersen_points_45 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_45_IDX);
        let pedersen_points_46 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_46_IDX);
        let pedersen_points_47 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_47_IDX);
        let pedersen_points_48 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_48_IDX);
        let pedersen_points_49 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_49_IDX);
        let pedersen_points_50 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_50_IDX);
        let pedersen_points_51 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_51_IDX);
        let pedersen_points_52 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_52_IDX);
        let pedersen_points_53 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_53_IDX);
        let pedersen_points_54 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_54_IDX);
        let pedersen_points_55 = preprocessed_mask_values.get_and_mark_used(PEDERSEN_POINTS_55_IDX);

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
                    qm31_const::<1791500038, 0, 0, 0>(), seq_15, pedersen_points_0,
                    pedersen_points_1, pedersen_points_2, pedersen_points_3, pedersen_points_4,
                    pedersen_points_5, pedersen_points_6, pedersen_points_7, pedersen_points_8,
                    pedersen_points_9, pedersen_points_10, pedersen_points_11, pedersen_points_12,
                    pedersen_points_13, pedersen_points_14, pedersen_points_15, pedersen_points_16,
                    pedersen_points_17, pedersen_points_18, pedersen_points_19, pedersen_points_20,
                    pedersen_points_21, pedersen_points_22, pedersen_points_23, pedersen_points_24,
                    pedersen_points_25, pedersen_points_26, pedersen_points_27, pedersen_points_28,
                    pedersen_points_29, pedersen_points_30, pedersen_points_31, pedersen_points_32,
                    pedersen_points_33, pedersen_points_34, pedersen_points_35, pedersen_points_36,
                    pedersen_points_37, pedersen_points_38, pedersen_points_39, pedersen_points_40,
                    pedersen_points_41, pedersen_points_42, pedersen_points_43, pedersen_points_44,
                    pedersen_points_45, pedersen_points_46, pedersen_points_47, pedersen_points_48,
                    pedersen_points_49, pedersen_points_50, pedersen_points_51, pedersen_points_52,
                    pedersen_points_53, pedersen_points_54, pedersen_points_55,
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
            PEDERSEN_POINTS_0_IDX,
            qm31_const::<1498683260, 701261032, 9432172, 1479828962>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_1_IDX,
            qm31_const::<1565792439, 835478760, 76541036, 1479828962>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_2_IDX,
            qm31_const::<1364464902, 432825576, 2022698091, 1479828961>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_3_IDX,
            qm31_const::<1431574081, 567043304, 2089806955, 1479828961>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_4_IDX,
            qm31_const::<1767119976, 1238131944, 277867628, 1479828962>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_5_IDX,
            qm31_const::<1834229155, 1372349672, 344976492, 1479828962>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_6_IDX,
            qm31_const::<1632901618, 969696488, 143649900, 1479828962>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_7_IDX,
            qm31_const::<1700010797, 1103914216, 210758764, 1479828962>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_8_IDX,
            qm31_const::<961809828, 1775002855, 1620044906, 1479828961>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_9_IDX,
            qm31_const::<1028919007, 1909220583, 1687153770, 1479828961>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_10_IDX,
            qm31_const::<1362728643, 314406152, 1163211253, 1597535363>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_11_IDX,
            qm31_const::<1295619464, 180188424, 1096102389, 1597535363>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_12_IDX,
            qm31_const::<1496947001, 582841608, 1297428981, 1597535363>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_13_IDX,
            qm31_const::<1429837822, 448623880, 1230320117, 1597535363>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_14_IDX,
            qm31_const::<1631165359, 851277064, 1431646709, 1597535363>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_15_IDX,
            qm31_const::<1564056180, 717059336, 1364537845, 1597535363>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_16_IDX,
            qm31_const::<1765383717, 1119712520, 1565864437, 1597535363>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_17_IDX,
            qm31_const::<1698274538, 985494792, 1498755573, 1597535363>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_18_IDX,
            qm31_const::<825855211, 1388147975, 626340340, 1597535363>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_19_IDX,
            qm31_const::<758746032, 1253930247, 559231476, 1597535363>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_20_IDX,
            qm31_const::<1228203100, 45970516, 1028993345, 1597535303>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_21_IDX,
            qm31_const::<1295312279, 180188244, 1096102209, 1597535303>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_22_IDX,
            qm31_const::<1362421458, 314405972, 1163211073, 1597535303>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_23_IDX,
            qm31_const::<1429530637, 448623700, 1230319937, 1597535303>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_24_IDX,
            qm31_const::<1496639816, 582841428, 1297428801, 1597535303>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_25_IDX,
            qm31_const::<1563748995, 717059156, 1364537665, 1597535303>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_26_IDX,
            qm31_const::<1630858174, 851276884, 1431646529, 1597535303>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_27_IDX,
            qm31_const::<1697967353, 985494612, 1498755393, 1597535303>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_28_IDX,
            qm31_const::<691329668, 1119712339, 492122432, 1597535303>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_29_IDX,
            qm31_const::<758438847, 1253930067, 559231296, 1597535303>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_30_IDX,
            qm31_const::<557213705, 851276943, 357904764, 1597535323>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_31_IDX,
            qm31_const::<490104526, 717059215, 290795900, 1597535323>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_32_IDX,
            qm31_const::<422995347, 582841487, 223687036, 1597535323>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_33_IDX,
            qm31_const::<355886168, 448623759, 156578172, 1597535323>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_34_IDX,
            qm31_const::<825650421, 1388147855, 626340220, 1597535323>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_35_IDX,
            qm31_const::<758541242, 1253930127, 559231356, 1597535323>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_36_IDX,
            qm31_const::<691432063, 1119712399, 492122492, 1597535323>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_37_IDX,
            qm31_const::<624322884, 985494671, 425013628, 1597535323>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_38_IDX,
            qm31_const::<1094087137, 1925018767, 894775676, 1597535323>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_39_IDX,
            qm31_const::<1026977958, 1790801039, 827666812, 1597535323>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_40_IDX,
            qm31_const::<1497254186, 582841788, 1297429161, 1597535423>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_41_IDX,
            qm31_const::<1564363365, 717059516, 1364538025, 1597535423>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_42_IDX,
            qm31_const::<1363035828, 314406332, 1163211433, 1597535423>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_43_IDX,
            qm31_const::<1430145007, 448624060, 1230320297, 1597535423>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_44_IDX,
            qm31_const::<1765690902, 1119712700, 1565864617, 1597535423>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_45_IDX,
            qm31_const::<1832800081, 1253930428, 1632973481, 1597535423>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_46_IDX,
            qm31_const::<1631472544, 851277244, 1431646889, 1597535423>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_47_IDX,
            qm31_const::<1698581723, 985494972, 1498755753, 1597535423>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_48_IDX,
            qm31_const::<960380754, 1656583611, 760558248, 1597535423>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_49_IDX,
            qm31_const::<1027489933, 1790801339, 827667112, 1597535423>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_50_IDX,
            qm31_const::<826264791, 1388148215, 626340580, 1597535443>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_51_IDX,
            qm31_const::<759155612, 1253930487, 559231716, 1597535443>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_52_IDX,
            qm31_const::<960483149, 1656583671, 760558308, 1597535443>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_53_IDX,
            qm31_const::<893373970, 1522365943, 693449444, 1597535443>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_54_IDX,
            qm31_const::<557828075, 851277303, 357905124, 1597535443>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            PEDERSEN_POINTS_55_IDX,
            qm31_const::<490718896, 717059575, 290796260, 1597535443>(),
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
