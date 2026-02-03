// This file was created by the AIR team.

use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 1;
pub const LOG_SIZE: u32 = 6;

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
        let mut poseidon_round_keys_sum_0: QM31 = Zero::zero();
        let seq_6 = preprocessed_mask_values.get_and_mark_used(SEQ_6_IDX);
        let poseidon_round_keys_0 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_0_IDX);
        let poseidon_round_keys_1 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_1_IDX);
        let poseidon_round_keys_2 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_2_IDX);
        let poseidon_round_keys_3 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_3_IDX);
        let poseidon_round_keys_4 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_4_IDX);
        let poseidon_round_keys_5 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_5_IDX);
        let poseidon_round_keys_6 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_6_IDX);
        let poseidon_round_keys_7 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_7_IDX);
        let poseidon_round_keys_8 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_8_IDX);
        let poseidon_round_keys_9 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_9_IDX);
        let poseidon_round_keys_10 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_10_IDX);
        let poseidon_round_keys_11 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_11_IDX);
        let poseidon_round_keys_12 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_12_IDX);
        let poseidon_round_keys_13 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_13_IDX);
        let poseidon_round_keys_14 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_14_IDX);
        let poseidon_round_keys_15 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_15_IDX);
        let poseidon_round_keys_16 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_16_IDX);
        let poseidon_round_keys_17 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_17_IDX);
        let poseidon_round_keys_18 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_18_IDX);
        let poseidon_round_keys_19 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_19_IDX);
        let poseidon_round_keys_20 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_20_IDX);
        let poseidon_round_keys_21 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_21_IDX);
        let poseidon_round_keys_22 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_22_IDX);
        let poseidon_round_keys_23 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_23_IDX);
        let poseidon_round_keys_24 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_24_IDX);
        let poseidon_round_keys_25 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_25_IDX);
        let poseidon_round_keys_26 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_26_IDX);
        let poseidon_round_keys_27 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_27_IDX);
        let poseidon_round_keys_28 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_28_IDX);
        let poseidon_round_keys_29 = preprocessed_mask_values
            .get_and_mark_used(POSEIDON_ROUND_KEYS_29_IDX);

        let [poseidon_round_keys_multiplicity]: [Span<QM31>; 1] = (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [poseidon_round_keys_multiplicity]: [QM31; 1] = (*poseidon_round_keys_multiplicity
            .try_into()
            .unwrap())
            .unbox();

        core::internal::revoke_ap_tracking();

        poseidon_round_keys_sum_0 = self
            .common_lookup_elements
            .combine_qm31(
                [
                    qm31_const::<1024310512, 0, 0, 0>(), seq_6, poseidon_round_keys_0,
                    poseidon_round_keys_1, poseidon_round_keys_2, poseidon_round_keys_3,
                    poseidon_round_keys_4, poseidon_round_keys_5, poseidon_round_keys_6,
                    poseidon_round_keys_7, poseidon_round_keys_8, poseidon_round_keys_9,
                    poseidon_round_keys_10, poseidon_round_keys_11, poseidon_round_keys_12,
                    poseidon_round_keys_13, poseidon_round_keys_14, poseidon_round_keys_15,
                    poseidon_round_keys_16, poseidon_round_keys_17, poseidon_round_keys_18,
                    poseidon_round_keys_19, poseidon_round_keys_20, poseidon_round_keys_21,
                    poseidon_round_keys_22, poseidon_round_keys_23, poseidon_round_keys_24,
                    poseidon_round_keys_25, poseidon_round_keys_26, poseidon_round_keys_27,
                    poseidon_round_keys_28, poseidon_round_keys_29,
                ]
                    .span(),
            );

        lookup_constraints(
            ref sum,
            random_coeff,
            claimed_sum,
            poseidon_round_keys_multiplicity,
            column_size,
            ref interaction_trace_mask_values,
            poseidon_round_keys_sum_0,
        );
    }
}


fn lookup_constraints(
    ref sum: QM31,
    random_coeff: QM31,
    claimed_sum: QM31,
    poseidon_round_keys_multiplicity: QM31,
    column_size: M31,
    ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
    poseidon_round_keys_sum_0: QM31,
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
        * poseidon_round_keys_sum_0)
        + poseidon_round_keys_multiplicity);
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
            SEQ_6_IDX,
            qm31_const::<897701151, 670658310, 1894193071, 865942395>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            POSEIDON_ROUND_KEYS_0_IDX,
            qm31_const::<876887147, 1637604496, 697065836, 1600795770>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            POSEIDON_ROUND_KEYS_1_IDX,
            qm31_const::<809777968, 1503386768, 629956972, 1600795770>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            POSEIDON_ROUND_KEYS_2_IDX,
            qm31_const::<1011105505, 1906039952, 831283564, 1600795770>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            POSEIDON_ROUND_KEYS_3_IDX,
            qm31_const::<943996326, 1771822224, 764174700, 1600795770>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            POSEIDON_ROUND_KEYS_4_IDX,
            qm31_const::<1145323863, 26991761, 965501293, 1600795770>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            POSEIDON_ROUND_KEYS_5_IDX,
            qm31_const::<1078214684, 2040257680, 898392428, 1600795770>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            POSEIDON_ROUND_KEYS_6_IDX,
            qm31_const::<1279542221, 295427217, 1099719021, 1600795770>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            POSEIDON_ROUND_KEYS_7_IDX,
            qm31_const::<1212433042, 161209489, 1032610157, 1600795770>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            POSEIDON_ROUND_KEYS_8_IDX,
            qm31_const::<340013715, 563862672, 160194924, 1600795770>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            POSEIDON_ROUND_KEYS_9_IDX,
            qm31_const::<272904536, 429644944, 93086060, 1600795770>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            POSEIDON_ROUND_KEYS_10_IDX,
            qm31_const::<953889868, 561726076, 1919855750, 479512227>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            POSEIDON_ROUND_KEYS_11_IDX,
            qm31_const::<1020999047, 695943804, 1986964614, 479512227>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            POSEIDON_ROUND_KEYS_12_IDX,
            qm31_const::<1088108226, 830161532, 2054073478, 479512227>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            POSEIDON_ROUND_KEYS_13_IDX,
            qm31_const::<1155217405, 964379260, 2121182342, 479512227>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            POSEIDON_ROUND_KEYS_14_IDX,
            qm31_const::<1222326584, 1098596988, 40807559, 479512228>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            POSEIDON_ROUND_KEYS_15_IDX,
            qm31_const::<1289435763, 1232814716, 107916423, 479512228>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            POSEIDON_ROUND_KEYS_16_IDX,
            qm31_const::<1356544942, 1367032444, 175025287, 479512228>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            POSEIDON_ROUND_KEYS_17_IDX,
            qm31_const::<1423654121, 1501250172, 242134151, 479512228>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            POSEIDON_ROUND_KEYS_18_IDX,
            qm31_const::<417016436, 1635467899, 1382984837, 479512227>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            POSEIDON_ROUND_KEYS_19_IDX,
            qm31_const::<484125615, 1769685627, 1450093701, 479512227>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            POSEIDON_ROUND_KEYS_20_IDX,
            qm31_const::<1088415411, 830161712, 2054073658, 479512287>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            POSEIDON_ROUND_KEYS_21_IDX,
            qm31_const::<1021306232, 695943984, 1986964794, 479512287>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            POSEIDON_ROUND_KEYS_22_IDX,
            qm31_const::<1222633769, 1098597168, 40807739, 479512288>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            POSEIDON_ROUND_KEYS_23_IDX,
            qm31_const::<1155524590, 964379440, 2121182522, 479512287>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            POSEIDON_ROUND_KEYS_24_IDX,
            qm31_const::<1356852127, 1367032624, 175025467, 479512288>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            POSEIDON_ROUND_KEYS_25_IDX,
            qm31_const::<1289742948, 1232814896, 107916603, 479512288>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            POSEIDON_ROUND_KEYS_26_IDX,
            qm31_const::<1491070485, 1635468080, 309243195, 479512288>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            POSEIDON_ROUND_KEYS_27_IDX,
            qm31_const::<1423961306, 1501250352, 242134331, 479512288>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            POSEIDON_ROUND_KEYS_28_IDX,
            qm31_const::<551541979, 1903903535, 1517202745, 479512287>(),
        );
        let mut preprocessed_trace = preprocessed_mask_add(
            preprocessed_trace,
            POSEIDON_ROUND_KEYS_29_IDX,
            qm31_const::<484432800, 1769685807, 1450093881, 479512287>(),
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
        assert_eq!(sum, QM31Trait::from_fixed_array(POSEIDON_ROUND_KEYS_SAMPLE_EVAL_RESULT))
    }
}
