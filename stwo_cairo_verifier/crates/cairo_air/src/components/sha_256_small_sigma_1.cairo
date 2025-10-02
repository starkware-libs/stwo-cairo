// AIR version 98896da1
use crate::prelude::*;use crate::components::subroutines::bitwise_and_num_bits_16::bitwise_and_num_bits_16_evaluate;use crate::components::subroutines::bitwise_xor_num_bits_16::bitwise_xor_num_bits_16_evaluate;

pub const N_TRACE_COLUMNS: usize = 21;pub const RELATION_USES_PER_ROW: [(felt252, u32); 4] = [
    ('VerifyBitwiseAnd_16', 6), ('Sha256SmallSigma1O0', 1), ('Sha256SmallSigma1O1', 1), ('VerifyBitwiseXor_16', 2)
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
        let interaction_log_sizes = [log_size; 24].span();
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
    pub verify_bitwise_and_16_lookup_elements: crate::VerifyBitwiseAnd_16Elements,
pub sha_256_small_sigma_1_o_0_lookup_elements: crate::Sha256SmallSigma1O0Elements,
pub sha_256_small_sigma_1_o_1_lookup_elements: crate::Sha256SmallSigma1O1Elements,
pub verify_bitwise_xor_16_lookup_elements: crate::VerifyBitwiseXor_16Elements,
pub sha_256_small_sigma_1_lookup_elements: crate::Sha256SmallSigma1Elements,
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
            verify_bitwise_and_16_lookup_elements: interaction_elements.verify_bitwise_and_16.clone(),
sha_256_small_sigma_1_o_0_lookup_elements: interaction_elements.sha_256_small_sigma_1_o_0.clone(),
sha_256_small_sigma_1_o_1_lookup_elements: interaction_elements.sha_256_small_sigma_1_o_1.clone(),
verify_bitwise_xor_16_lookup_elements: interaction_elements.verify_bitwise_xor_16.clone(),
sha_256_small_sigma_1_lookup_elements: interaction_elements.sha_256_small_sigma_1.clone(),
        }
    }
}

pub impl CairoComponentImpl of CairoComponent<Component> {
    fn mask_points(
        self: @Component,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let log_size = *(self.claim.log_size);
        let trace_gen = CanonicCosetImpl::new(log_size).coset.step;
        let point_offset_neg_1 = point.add_circle_point_m31(-trace_gen.mul(1).to_point());
        trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point]);interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
        interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
        interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
        interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
    }

    fn max_constraint_log_degree_bound(self: @Component) -> u32 {
        *(self.claim.log_size) + 1
    }

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
        let mut verify_bitwise_and_16_sum_0: QM31 = Zero::zero();let mut verify_bitwise_and_16_sum_1: QM31 = Zero::zero();let mut verify_bitwise_and_16_sum_2: QM31 = Zero::zero();let mut verify_bitwise_and_16_sum_3: QM31 = Zero::zero();let mut verify_bitwise_and_16_sum_4: QM31 = Zero::zero();let mut verify_bitwise_and_16_sum_5: QM31 = Zero::zero();let mut sha_256_small_sigma_1_o_0_sum_6: QM31 = Zero::zero();let mut sha_256_small_sigma_1_o_1_sum_7: QM31 = Zero::zero();let mut verify_bitwise_xor_16_sum_8: QM31 = Zero::zero();let mut verify_bitwise_xor_16_sum_9: QM31 = Zero::zero();let mut sha_256_small_sigma_1_sum_10: QM31 = Zero::zero();

        let [input_limb_0_col0, input_limb_1_col1, and_col2, and_col3, and_col4, and_col5, and_col6, and_col7, sigma_O0_L_col8, sigma_O0_H_col9, sigma_O1_L_col10, sigma_O1_H_col11, sigma_O2_L_col12, sigma_O2_H_col13, sigma_O2_prime_L_col14, sigma_O2_prime_H_col15, xor_col16, xor_col17, output_low_col18, output_high_col19, enabler]: [Span<QM31>; 21]
            = (*trace_mask_values.multi_pop_front().unwrap()).unbox();
        let [input_limb_0_col0]: [QM31; 1] = (*input_limb_0_col0.try_into().unwrap()).unbox();let [input_limb_1_col1]: [QM31; 1] = (*input_limb_1_col1.try_into().unwrap()).unbox();let [and_col2]: [QM31; 1] = (*and_col2.try_into().unwrap()).unbox();let [and_col3]: [QM31; 1] = (*and_col3.try_into().unwrap()).unbox();let [and_col4]: [QM31; 1] = (*and_col4.try_into().unwrap()).unbox();let [and_col5]: [QM31; 1] = (*and_col5.try_into().unwrap()).unbox();let [and_col6]: [QM31; 1] = (*and_col6.try_into().unwrap()).unbox();let [and_col7]: [QM31; 1] = (*and_col7.try_into().unwrap()).unbox();let [sigma_O0_L_col8]: [QM31; 1] = (*sigma_O0_L_col8.try_into().unwrap()).unbox();let [sigma_O0_H_col9]: [QM31; 1] = (*sigma_O0_H_col9.try_into().unwrap()).unbox();let [sigma_O1_L_col10]: [QM31; 1] = (*sigma_O1_L_col10.try_into().unwrap()).unbox();let [sigma_O1_H_col11]: [QM31; 1] = (*sigma_O1_H_col11.try_into().unwrap()).unbox();let [sigma_O2_L_col12]: [QM31; 1] = (*sigma_O2_L_col12.try_into().unwrap()).unbox();let [sigma_O2_H_col13]: [QM31; 1] = (*sigma_O2_H_col13.try_into().unwrap()).unbox();let [sigma_O2_prime_L_col14]: [QM31; 1] = (*sigma_O2_prime_L_col14.try_into().unwrap()).unbox();let [sigma_O2_prime_H_col15]: [QM31; 1] = (*sigma_O2_prime_H_col15.try_into().unwrap()).unbox();let [xor_col16]: [QM31; 1] = (*xor_col16.try_into().unwrap()).unbox();let [xor_col17]: [QM31; 1] = (*xor_col17.try_into().unwrap()).unbox();let [output_low_col18]: [QM31; 1] = (*output_low_col18.try_into().unwrap()).unbox();let [output_high_col19]: [QM31; 1] = (*output_high_col19.try_into().unwrap()).unbox();let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();


        core::internal::revoke_ap_tracking();

        let constraint_quotient = (enabler * enabler - enabler) * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;bitwise_and_num_bits_16_evaluate(
            [input_limb_0_col0, qm31_const::<17029, 0, 0, 0>()],
and_col2,
self.verify_bitwise_and_16_lookup_elements,
ref verify_bitwise_and_16_sum_0,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_and_num_bits_16_evaluate(
            [input_limb_0_col0, qm31_const::<122, 0, 0, 0>()],
and_col3,
self.verify_bitwise_and_16_lookup_elements,
ref verify_bitwise_and_16_sum_1,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_and_num_bits_16_evaluate(
            [input_limb_0_col0, qm31_const::<48384, 0, 0, 0>()],
and_col4,
self.verify_bitwise_and_16_lookup_elements,
ref verify_bitwise_and_16_sum_2,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_and_num_bits_16_evaluate(
            [input_limb_1_col1, qm31_const::<19109, 0, 0, 0>()],
and_col5,
self.verify_bitwise_and_16_lookup_elements,
ref verify_bitwise_and_16_sum_3,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_and_num_bits_16_evaluate(
            [input_limb_1_col1, qm31_const::<346, 0, 0, 0>()],
and_col6,
self.verify_bitwise_and_16_lookup_elements,
ref verify_bitwise_and_16_sum_4,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_and_num_bits_16_evaluate(
            [input_limb_1_col1, qm31_const::<46080, 0, 0, 0>()],
and_col7,
self.verify_bitwise_and_16_lookup_elements,
ref verify_bitwise_and_16_sum_5,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );

        sha_256_small_sigma_1_o_0_sum_6 = self.sha_256_small_sigma_1_o_0_lookup_elements.combine_qm31(
            [
                and_col2,
and_col5,
sigma_O0_L_col8,
sigma_O0_H_col9,
sigma_O2_L_col12,
sigma_O2_H_col13
            ],
        );

        sha_256_small_sigma_1_o_1_sum_7 = self.sha_256_small_sigma_1_o_1_lookup_elements.combine_qm31(
            [
                (and_col3 + and_col4),
(and_col6 + and_col7),
sigma_O1_L_col10,
sigma_O1_H_col11,
sigma_O2_prime_L_col14,
sigma_O2_prime_H_col15
            ],
        );bitwise_xor_num_bits_16_evaluate(
            [sigma_O2_prime_L_col14, sigma_O2_L_col12],
xor_col16,
self.verify_bitwise_xor_16_lookup_elements,
ref verify_bitwise_xor_16_sum_8,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );bitwise_xor_num_bits_16_evaluate(
            [sigma_O2_prime_H_col15, sigma_O2_H_col13],
xor_col17,
self.verify_bitwise_xor_16_lookup_elements,
ref verify_bitwise_xor_16_sum_9,
ref sum,
domain_vanishing_eval_inv,
random_coeff
        );

        sha_256_small_sigma_1_sum_10 = self.sha_256_small_sigma_1_lookup_elements.combine_qm31(
            [
                input_limb_0_col0,
input_limb_1_col1,
output_low_col18,
output_high_col19
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
            verify_bitwise_and_16_sum_0,
verify_bitwise_and_16_sum_1,
verify_bitwise_and_16_sum_2,
verify_bitwise_and_16_sum_3,
verify_bitwise_and_16_sum_4,
verify_bitwise_and_16_sum_5,
sha_256_small_sigma_1_o_0_sum_6,
sha_256_small_sigma_1_o_1_sum_7,
verify_bitwise_xor_16_sum_8,
verify_bitwise_xor_16_sum_9,
sha_256_small_sigma_1_sum_10
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
    verify_bitwise_and_16_sum_0: QM31,
verify_bitwise_and_16_sum_1: QM31,
verify_bitwise_and_16_sum_2: QM31,
verify_bitwise_and_16_sum_3: QM31,
verify_bitwise_and_16_sum_4: QM31,
verify_bitwise_and_16_sum_5: QM31,
sha_256_small_sigma_1_o_0_sum_6: QM31,
sha_256_small_sigma_1_o_1_sum_7: QM31,
verify_bitwise_xor_16_sum_8: QM31,
verify_bitwise_xor_16_sum_9: QM31,
sha_256_small_sigma_1_sum_10: QM31
) {
    let [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3, trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7, trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11, trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15, trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19, trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23]: [Span<QM31>; 24]
        = (*interaction_trace_mask_values.multi_pop_front().unwrap()).unbox();

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
let [trace_2_col16]: [QM31; 1] = (*trace_2_col16.try_into().unwrap()).unbox();
let [trace_2_col17]: [QM31; 1] = (*trace_2_col17.try_into().unwrap()).unbox();
let [trace_2_col18]: [QM31; 1] = (*trace_2_col18.try_into().unwrap()).unbox();
let [trace_2_col19]: [QM31; 1] = (*trace_2_col19.try_into().unwrap()).unbox();
let [trace_2_col20_neg1, trace_2_col20]: [QM31; 2] = (*trace_2_col20.try_into().unwrap()).unbox();
let [trace_2_col21_neg1, trace_2_col21]: [QM31; 2] = (*trace_2_col21.try_into().unwrap()).unbox();
let [trace_2_col22_neg1, trace_2_col22]: [QM31; 2] = (*trace_2_col22.try_into().unwrap()).unbox();
let [trace_2_col23_neg1, trace_2_col23]: [QM31; 2] = (*trace_2_col23.try_into().unwrap()).unbox();


    core::internal::revoke_ap_tracking();

    
let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3])
            ) * verify_bitwise_and_16_sum_0 * verify_bitwise_and_16_sum_1
        ) - verify_bitwise_and_16_sum_0 - verify_bitwise_and_16_sum_1
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7]) 
                - QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3])
            ) * verify_bitwise_and_16_sum_2 * verify_bitwise_and_16_sum_3
        ) - verify_bitwise_and_16_sum_2 - verify_bitwise_and_16_sum_3
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11]) 
                - QM31Impl::from_partial_evals([trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7])
            ) * verify_bitwise_and_16_sum_4 * verify_bitwise_and_16_sum_5
        ) - verify_bitwise_and_16_sum_4 - verify_bitwise_and_16_sum_5
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15]) 
                - QM31Impl::from_partial_evals([trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11])
            ) * sha_256_small_sigma_1_o_0_sum_6 * sha_256_small_sigma_1_o_1_sum_7
        ) - sha_256_small_sigma_1_o_0_sum_6 - sha_256_small_sigma_1_o_1_sum_7
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19]) 
                - QM31Impl::from_partial_evals([trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15])
            ) * verify_bitwise_xor_16_sum_8 * verify_bitwise_xor_16_sum_9
        ) - verify_bitwise_xor_16_sum_8 - verify_bitwise_xor_16_sum_9
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

let constraint_quotient = (
        (
            (
                QM31Impl::from_partial_evals([trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23]) 
                - QM31Impl::from_partial_evals([trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19]) 
                - QM31Impl::from_partial_evals([trace_2_col20_neg1, trace_2_col21_neg1, trace_2_col22_neg1, trace_2_col23_neg1])
                + (claimed_sum * (column_size.inverse().into()))
            ) * sha_256_small_sigma_1_sum_10
        ) + enabler
    ) * domain_vanishing_eval_inv;
sum = sum * random_coeff + constraint_quotient;

}