// This file was created by the AIR team.

use crate::components::subroutines::bitwise_xor_num_bits_8::bitwise_xor_num_bits_8_evaluate;
use crate::components::subroutines::bitwise_xor_num_bits_8_b::bitwise_xor_num_bits_8_b_evaluate;
use crate::components::subroutines::split_16_low_part_size_8::split_16_low_part_size_8_evaluate;
use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 21;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 2] = [
    ('VerifyBitwiseXor_8', 4), ('VerifyBitwiseXor_8_B', 4),
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
        let interaction_log_sizes = [log_size; 20].span();
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
    pub verify_bitwise_xor_8_lookup_elements: crate::VerifyBitwiseXor_8Elements,
    pub verify_bitwise_xor_8_b_lookup_elements: crate::VerifyBitwiseXor_8_BElements,
    pub triple_xor_32_lookup_elements: crate::TripleXor32Elements,
}

pub impl NewComponentImpl of NewComponent<Component> {
    type Claim = Claim;
    type InteractionClaim = InteractionClaim;

    fn new(
        claim: @Claim,
        interaction_claim: @InteractionClaim,
        interaction_elements: @CairoInteractionElements,
    ) -> Component {
        Component {
            claim: *claim,
            interaction_claim: *interaction_claim,
            verify_bitwise_xor_8_lookup_elements: interaction_elements.verify_bitwise_xor_8.clone(),
            verify_bitwise_xor_8_b_lookup_elements: interaction_elements
                .verify_bitwise_xor_8_b
                .clone(),
            triple_xor_32_lookup_elements: interaction_elements.triple_xor_32.clone(),
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
        point: CirclePoint<QM31>,
    ) {
        let log_size = *(self.claim.log_size);
        let trace_domain = CanonicCosetImpl::new(log_size);
        let domain_vanishing_eval_inv = trace_domain.eval_vanishing(point).inverse();
        let claimed_sum = *self.interaction_claim.claimed_sum;
        let column_size = m31(pow2(log_size));
        let mut verify_bitwise_xor_8_sum_0: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_sum_1: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_sum_2: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_sum_3: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_b_sum_4: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_b_sum_5: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_b_sum_6: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_b_sum_7: QM31 = Zero::zero();
        let mut triple_xor_32_sum_8: QM31 = Zero::zero();

        let [
            input_limb_0_col0,
            input_limb_1_col1,
            input_limb_2_col2,
            input_limb_3_col3,
            input_limb_4_col4,
            input_limb_5_col5,
            ms_8_bits_col6,
            ms_8_bits_col7,
            ms_8_bits_col8,
            ms_8_bits_col9,
            ms_8_bits_col10,
            ms_8_bits_col11,
            xor_col12,
            xor_col13,
            xor_col14,
            xor_col15,
            xor_col16,
            xor_col17,
            xor_col18,
            xor_col19,
            enabler,
        ]: [Span<QM31>; 21] =
            (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [input_limb_0_col0]: [QM31; 1] = (*input_limb_0_col0.try_into().unwrap()).unbox();
        let [input_limb_1_col1]: [QM31; 1] = (*input_limb_1_col1.try_into().unwrap()).unbox();
        let [input_limb_2_col2]: [QM31; 1] = (*input_limb_2_col2.try_into().unwrap()).unbox();
        let [input_limb_3_col3]: [QM31; 1] = (*input_limb_3_col3.try_into().unwrap()).unbox();
        let [input_limb_4_col4]: [QM31; 1] = (*input_limb_4_col4.try_into().unwrap()).unbox();
        let [input_limb_5_col5]: [QM31; 1] = (*input_limb_5_col5.try_into().unwrap()).unbox();
        let [ms_8_bits_col6]: [QM31; 1] = (*ms_8_bits_col6.try_into().unwrap()).unbox();
        let [ms_8_bits_col7]: [QM31; 1] = (*ms_8_bits_col7.try_into().unwrap()).unbox();
        let [ms_8_bits_col8]: [QM31; 1] = (*ms_8_bits_col8.try_into().unwrap()).unbox();
        let [ms_8_bits_col9]: [QM31; 1] = (*ms_8_bits_col9.try_into().unwrap()).unbox();
        let [ms_8_bits_col10]: [QM31; 1] = (*ms_8_bits_col10.try_into().unwrap()).unbox();
        let [ms_8_bits_col11]: [QM31; 1] = (*ms_8_bits_col11.try_into().unwrap()).unbox();
        let [xor_col12]: [QM31; 1] = (*xor_col12.try_into().unwrap()).unbox();
        let [xor_col13]: [QM31; 1] = (*xor_col13.try_into().unwrap()).unbox();
        let [xor_col14]: [QM31; 1] = (*xor_col14.try_into().unwrap()).unbox();
        let [xor_col15]: [QM31; 1] = (*xor_col15.try_into().unwrap()).unbox();
        let [xor_col16]: [QM31; 1] = (*xor_col16.try_into().unwrap()).unbox();
        let [xor_col17]: [QM31; 1] = (*xor_col17.try_into().unwrap()).unbox();
        let [xor_col18]: [QM31; 1] = (*xor_col18.try_into().unwrap()).unbox();
        let [xor_col19]: [QM31; 1] = (*xor_col19.try_into().unwrap()).unbox();
        let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        let constraint_quotient = (enabler * enabler - enabler) * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;
        let split_16_low_part_size_8_output_tmp_298db_1_limb_0: QM31 =
            split_16_low_part_size_8_evaluate(
            input_limb_0_col0, ms_8_bits_col6, ref sum, domain_vanishing_eval_inv, random_coeff,
        );
        let split_16_low_part_size_8_output_tmp_298db_3_limb_0: QM31 =
            split_16_low_part_size_8_evaluate(
            input_limb_1_col1, ms_8_bits_col7, ref sum, domain_vanishing_eval_inv, random_coeff,
        );
        let split_16_low_part_size_8_output_tmp_298db_5_limb_0: QM31 =
            split_16_low_part_size_8_evaluate(
            input_limb_2_col2, ms_8_bits_col8, ref sum, domain_vanishing_eval_inv, random_coeff,
        );
        let split_16_low_part_size_8_output_tmp_298db_7_limb_0: QM31 =
            split_16_low_part_size_8_evaluate(
            input_limb_3_col3, ms_8_bits_col9, ref sum, domain_vanishing_eval_inv, random_coeff,
        );
        let split_16_low_part_size_8_output_tmp_298db_9_limb_0: QM31 =
            split_16_low_part_size_8_evaluate(
            input_limb_4_col4, ms_8_bits_col10, ref sum, domain_vanishing_eval_inv, random_coeff,
        );
        let split_16_low_part_size_8_output_tmp_298db_11_limb_0: QM31 =
            split_16_low_part_size_8_evaluate(
            input_limb_5_col5, ms_8_bits_col11, ref sum, domain_vanishing_eval_inv, random_coeff,
        );
        bitwise_xor_num_bits_8_evaluate(
            [
                split_16_low_part_size_8_output_tmp_298db_1_limb_0,
                split_16_low_part_size_8_output_tmp_298db_5_limb_0,
            ],
            xor_col12,
            self.verify_bitwise_xor_8_lookup_elements,
            ref verify_bitwise_xor_8_sum_0,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        bitwise_xor_num_bits_8_evaluate(
            [xor_col12, split_16_low_part_size_8_output_tmp_298db_9_limb_0],
            xor_col13,
            self.verify_bitwise_xor_8_lookup_elements,
            ref verify_bitwise_xor_8_sum_1,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        bitwise_xor_num_bits_8_evaluate(
            [ms_8_bits_col6, ms_8_bits_col8],
            xor_col14,
            self.verify_bitwise_xor_8_lookup_elements,
            ref verify_bitwise_xor_8_sum_2,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        bitwise_xor_num_bits_8_evaluate(
            [xor_col14, ms_8_bits_col10],
            xor_col15,
            self.verify_bitwise_xor_8_lookup_elements,
            ref verify_bitwise_xor_8_sum_3,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        bitwise_xor_num_bits_8_b_evaluate(
            [
                split_16_low_part_size_8_output_tmp_298db_3_limb_0,
                split_16_low_part_size_8_output_tmp_298db_7_limb_0,
            ],
            xor_col16,
            self.verify_bitwise_xor_8_b_lookup_elements,
            ref verify_bitwise_xor_8_b_sum_4,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        bitwise_xor_num_bits_8_b_evaluate(
            [xor_col16, split_16_low_part_size_8_output_tmp_298db_11_limb_0],
            xor_col17,
            self.verify_bitwise_xor_8_b_lookup_elements,
            ref verify_bitwise_xor_8_b_sum_5,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        bitwise_xor_num_bits_8_b_evaluate(
            [ms_8_bits_col7, ms_8_bits_col9],
            xor_col18,
            self.verify_bitwise_xor_8_b_lookup_elements,
            ref verify_bitwise_xor_8_b_sum_6,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        bitwise_xor_num_bits_8_b_evaluate(
            [xor_col18, ms_8_bits_col11],
            xor_col19,
            self.verify_bitwise_xor_8_b_lookup_elements,
            ref verify_bitwise_xor_8_b_sum_7,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let triple_xor32_output_tmp_298db_28_limb_0: QM31 = (xor_col13
            + (xor_col15 * qm31_const::<256, 0, 0, 0>()));
        let triple_xor32_output_tmp_298db_28_limb_1: QM31 = (xor_col17
            + (xor_col19 * qm31_const::<256, 0, 0, 0>()));

        triple_xor_32_sum_8 = self
            .triple_xor_32_lookup_elements
            .combine_qm31(
                [
                    input_limb_0_col0, input_limb_1_col1, input_limb_2_col2, input_limb_3_col3,
                    input_limb_4_col4, input_limb_5_col5, triple_xor32_output_tmp_298db_28_limb_0,
                    triple_xor32_output_tmp_298db_28_limb_1,
                ],
            );

        lookup_constraints(
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
            claimed_sum,
            enabler,
            column_size,
            ref interaction_trace_mask_values,
            verify_bitwise_xor_8_sum_0,
            verify_bitwise_xor_8_sum_1,
            verify_bitwise_xor_8_sum_2,
            verify_bitwise_xor_8_sum_3,
            verify_bitwise_xor_8_b_sum_4,
            verify_bitwise_xor_8_b_sum_5,
            verify_bitwise_xor_8_b_sum_6,
            verify_bitwise_xor_8_b_sum_7,
            triple_xor_32_sum_8,
        );
    }
}


fn lookup_constraints(
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
    claimed_sum: QM31,
    enabler: QM31,
    column_size: M31,
    ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
    verify_bitwise_xor_8_sum_0: QM31,
    verify_bitwise_xor_8_sum_1: QM31,
    verify_bitwise_xor_8_sum_2: QM31,
    verify_bitwise_xor_8_sum_3: QM31,
    verify_bitwise_xor_8_b_sum_4: QM31,
    verify_bitwise_xor_8_b_sum_5: QM31,
    verify_bitwise_xor_8_b_sum_6: QM31,
    verify_bitwise_xor_8_b_sum_7: QM31,
    triple_xor_32_sum_8: QM31,
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
        trace_2_col16,
        trace_2_col17,
        trace_2_col18,
        trace_2_col19,
    ]: [Span<QM31>; 20] =
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
    let [trace_2_col12]: [QM31; 1] = (*trace_2_col12.try_into().unwrap()).unbox();
    let [trace_2_col13]: [QM31; 1] = (*trace_2_col13.try_into().unwrap()).unbox();
    let [trace_2_col14]: [QM31; 1] = (*trace_2_col14.try_into().unwrap()).unbox();
    let [trace_2_col15]: [QM31; 1] = (*trace_2_col15.try_into().unwrap()).unbox();
    let [trace_2_col16_neg1, trace_2_col16]: [QM31; 2] = (*trace_2_col16.try_into().unwrap())
        .unbox();
    let [trace_2_col17_neg1, trace_2_col17]: [QM31; 2] = (*trace_2_col17.try_into().unwrap())
        .unbox();
    let [trace_2_col18_neg1, trace_2_col18]: [QM31; 2] = (*trace_2_col18.try_into().unwrap())
        .unbox();
    let [trace_2_col19_neg1, trace_2_col19]: [QM31; 2] = (*trace_2_col19.try_into().unwrap())
        .unbox();

    core::internal::revoke_ap_tracking();

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3],
    ))
        * verify_bitwise_xor_8_sum_0
        * verify_bitwise_xor_8_sum_1)
        - verify_bitwise_xor_8_sum_0
        - verify_bitwise_xor_8_sum_1)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7],
    )
        - QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3]))
        * verify_bitwise_xor_8_sum_2
        * verify_bitwise_xor_8_sum_3)
        - verify_bitwise_xor_8_sum_2
        - verify_bitwise_xor_8_sum_3)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11],
    )
        - QM31Impl::from_partial_evals([trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7]))
        * verify_bitwise_xor_8_b_sum_4
        * verify_bitwise_xor_8_b_sum_5)
        - verify_bitwise_xor_8_b_sum_4
        - verify_bitwise_xor_8_b_sum_5)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
    )
        - QM31Impl::from_partial_evals([trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11]))
        * verify_bitwise_xor_8_b_sum_6
        * verify_bitwise_xor_8_b_sum_7)
        - verify_bitwise_xor_8_b_sum_6
        - verify_bitwise_xor_8_b_sum_7)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
    )
        - QM31Impl::from_partial_evals([trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15])
        - QM31Impl::from_partial_evals(
            [trace_2_col16_neg1, trace_2_col17_neg1, trace_2_col18_neg1, trace_2_col19_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * triple_xor_32_sum_8)
        + enabler)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
}
#[cfg(and(test, feature: "qm31_opcode"))]
mod tests {
    use core::array::ArrayImpl;
    use core::num::traits::Zero;
    #[allow(unused_imports)]
    use stwo_cairo_air::preprocessed_columns::{NUM_PREPROCESSED_COLUMNS, seq_column_idx};
    #[allow(unused_imports)]
    use stwo_constraint_framework::{
        LookupElements, PreprocessedMaskValues, PreprocessedMaskValuesTrait,
    };
    use stwo_verifier_core::circle::CirclePoint;
    use stwo_verifier_core::fields::qm31::{QM31, QM31Impl, QM31Trait, qm31_const};
    use crate::cairo_component::*;
    use crate::components::sample_evaluations::*;
    #[allow(unused_imports)]
    use crate::test_utils::{make_interaction_trace, make_lookup_elements, preprocessed_mask_add};
    use crate::utils::*;
    use super::{Claim, Component, InteractionClaim};

    #[test]
    fn test_evaluation_result() {
        let component = Component {
            claim: Claim { log_size: 15 },
            interaction_claim: InteractionClaim {
                claimed_sum: qm31_const::<1398335417, 314974026, 1722107152, 821933968>(),
            },
            triple_xor_32_lookup_elements: make_lookup_elements(
                qm31_const::<1306465622, 1475920612, 435786988, 143056699>(),
                qm31_const::<1864509813, 1662621571, 448425708, 599804019>(),
            ),
            verify_bitwise_xor_8_lookup_elements: make_lookup_elements(
                qm31_const::<390097169, 1715941348, 958959293, 1227669969>(),
                qm31_const::<105167513, 476596518, 1027059816, 1879697407>(),
            ),
            verify_bitwise_xor_8_b_lookup_elements: make_lookup_elements(
                qm31_const::<281609569, 2020003995, 58077116, 764105642>(),
                qm31_const::<797062783, 1701269078, 1114861254, 2119266818>(),
            ),
        };
        let mut sum: QM31 = Zero::zero();
        let point = CirclePoint {
            x: qm31_const::<461666434, 38651694, 1083586041, 510305943>(),
            y: qm31_const::<817798294, 862569777, 2091320744, 1178484122>(),
        };

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
            [qm31_const::<1718117409, 376642659, 1925018455, 343880121>()].span(),
            [qm31_const::<1785226588, 510860387, 1992127319, 343880121>()].span(),
            [qm31_const::<1315462335, 1718819938, 1522365270, 343880121>()].span(),
            [qm31_const::<1382571514, 1853037666, 1589474134, 343880121>()].span(),
            [qm31_const::<179325277, 825275894, 97341591, 1357105975>()].span(),
        ]
            .span();
        let interaction_values = array![
            qm31_const::<1005168032, 79980996, 1847888101, 1941984119>(),
            qm31_const::<1072277211, 214198724, 1914996965, 1941984119>(),
            qm31_const::<1139386390, 348416452, 1982105829, 1941984119>(),
            qm31_const::<1206495569, 482634180, 2049214693, 1941984119>(),
            qm31_const::<736731316, 1690593731, 1579452644, 1941984119>(),
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
                point,
            );
        preprocessed_trace.validate_usage();
        assert_eq!(sum, QM31Trait::from_fixed_array(TRIPLE_XOR_32_SAMPLE_EVAL_RESULT))
    }
}
