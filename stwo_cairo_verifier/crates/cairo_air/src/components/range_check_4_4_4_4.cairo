use core::num::traits::Zero;
use stwo_constraint_framework::{
    PreprocessedColumn, PreprocessedColumnSet, PreprocessedColumnSetImpl, PreprocessedMaskValues,
    PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::circle::{
    CirclePoint, CirclePointIndexTrait, CirclePointQM31AddCirclePointM31Trait,
};
use stwo_verifier_core::fields::Invertible;
use stwo_verifier_core::fields::m31::{M31, m31};
use stwo_verifier_core::fields::qm31::{
    QM31, QM31Impl, QM31Serde, QM31Zero, QM31_EXTENSION_DEGREE, qm31_const,
};
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::{ArrayImpl, pow2};
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray};
use crate::components::{
    CairoComponent, OPCODES_RELATION_SIZE, RANGE_CHECK_4_4_4_4_LOG_SIZE, opcodes_sum,
};
use crate::utils::U32Impl;


pub const N_TRACE_COLUMNS: usize = 1;
pub const LOG_SIZE: u32 = RANGE_CHECK_4_4_4_4_LOG_SIZE;
pub const RANGE_CHECK_4_4_4_4_RELATION_SIZE: usize = 4;


// #[derive(Drop, Serde, Copy)]
// pub struct Claim {}

// #[generate_trait]
// pub impl ClaimImpl of ClaimTrait {
//     fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
//         let log_size = LOG_SIZE;
//         let preprocessed_log_sizes = array![log_size].span();
//         let trace_log_sizes = ArrayImpl::new_repeated(N_TRACE_COLUMNS, log_size).span();
//         let interaction_log_sizes = ArrayImpl::new_repeated(QM31_EXTENSION_DEGREE,
//         log_size).span();
//         array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
//     }

//     fn mix_into(self: @Claim, ref channel: Channel) {
//         channel.mix_u64((LOG_SIZE).into());
//     }
// }

// #[derive(Drop, Serde, Copy)]
// pub struct InteractionClaim {
//     pub claimed_sum: QM31,
// }

// #[generate_trait]
// pub impl InteractionClaimImpl of InteractionClaimTrait {
//     fn mix_into(self: @InteractionClaim, ref channel: Channel) {
//         channel.mix_felts([*self.claimed_sum].span());
//     }
// }

// #[derive(Drop)]
// pub struct Component {
//     pub claim: Claim,
//     pub interaction_claim: InteractionClaim,
//     pub range_check_4_4_4_4_lookup_elements: crate::RangeCheck_4_4_4_4Elements,
// }

// pub impl ComponentImpl of CairoComponent<Component> {
//     fn mask_points(
//         self: @Component,
//         ref preprocessed_column_set: PreprocessedColumnSet,
//         ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
//         ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
//         point: CirclePoint<QM31>,
//     ) {
//         let log_size = LOG_SIZE;
//         let trace_gen = CanonicCosetImpl::new(log_size).coset.step_size;
//         let point_offset_neg_1 = point.add_circle_point_m31(-trace_gen.mul(1).to_point());

//         preprocessed_column_set.insert(PreprocessedColumn::RangeCheck4(([4,4,4,4], 0)));
//         preprocessed_column_set.insert(PreprocessedColumn::RangeCheck4(([4,4,4,4], 1)));
//         preprocessed_column_set.insert(PreprocessedColumn::RangeCheck4(([4,4,4,4], 2)));
//         preprocessed_column_set.insert(PreprocessedColumn::RangeCheck4(([4,4,4,4], 3)));
//         trace_mask_points.append(array![point]);
//         interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
//         interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
//         interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
//         interaction_trace_mask_points.append(array![point_offset_neg_1, point]);

//     }

//     fn max_constraint_log_degree_bound(self: @Component) -> u32 {
//         LOG_SIZE + 1
//     }

//     fn evaluate_constraints_at_point(
//         self: @Component,
//         ref sum: QM31,
//         ref preprocessed_mask_values: PreprocessedMaskValues,
//         ref trace_mask_values: ColumnSpan<Span<QM31>>,
//         ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
//         random_coeff: QM31,
//         point: CirclePoint<QM31>,
//     ) {
//         let log_size = LOG_SIZE;
//         let trace_domain = CanonicCosetImpl::new(log_size);
//         let domain_vanishing_eval_inv = trace_domain.eval_vanishing(point).inverse();
//         let claimed_sum = *self.interaction_claim.claimed_sum;
//         let column_size = m31(pow2(log_size));
//         let mut range_check_4_4_4_4_sum_0: QM31 = Zero::zero();
//         let rangecheck_4_4_4_4_0 =
//         preprocessed_mask_values.get(PreprocessedColumn::RangeCheck4(([4,4,4,4], 0)));
//         let rangecheck_4_4_4_4_1 =
//         preprocessed_mask_values.get(PreprocessedColumn::RangeCheck4(([4,4,4,4], 1)));
//         let rangecheck_4_4_4_4_2 =
//         preprocessed_mask_values.get(PreprocessedColumn::RangeCheck4(([4,4,4,4], 2)));
//         let rangecheck_4_4_4_4_3 =
//         preprocessed_mask_values.get(PreprocessedColumn::RangeCheck4(([4,4,4,4], 3)));
//         let range_check_4_4_4_4_alphas =
//         self.range_check_4_4_4_4_lookup_elements.alpha_powers.span();
//         let range_check_4_4_4_4_z = *self.range_check_4_4_4_4_lookup_elements.z;

//         let [enabler]: [Span<QM31>; 1] = (*trace_mask_values.multi_pop_front().unwrap()).unbox();
//         let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();

//         core::internal::revoke_ap_tracking();

//         range_check_4_4_4_4_sum_0 = range_check_4_4_4_4_sum(
//             range_check_4_4_4_4_alphas,
//             range_check_4_4_4_4_z,
//             [
//                 rangecheck_4_4_4_4_0,
//                 rangecheck_4_4_4_4_1,
//                 rangecheck_4_4_4_4_2,
//                 rangecheck_4_4_4_4_3
//             ],
//         );

//         lookup_constraints(
//             ref sum,
//             domain_vanishing_eval_inv,
//             random_coeff,
//             claimed_sum,
//             enabler,
//             column_size,
//             ref interaction_trace_mask_values,
//             range_check_4_4_4_4_sum_0
//         );
//     }
// }

pub fn range_check_4_4_4_4_sum(mut alphas: Span<QM31>, z: QM31, values: [QM31; 4]) -> QM31 {
    let [alpha0, alpha1, alpha2, alpha3] = (*alphas.multi_pop_front().unwrap()).unbox();
    let [val0, val1, val2, val3] = values;

    alpha0 * val0 + alpha1 * val1 + alpha2 * val2 + alpha3 * val3 - z
}

#[cairofmt::skip]
fn lookup_constraints(
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
    claimed_sum: QM31,
    enabler: QM31,
    column_size: M31,
    ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
    range_check_4_4_4_4_sum_0: QM31
) {
    let [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3]: [Span<QM31>; 4]
        = (*interaction_trace_mask_values.multi_pop_front().unwrap()).unbox();
    
    let [trace_2_col0_neg1, trace_2_col0]: [QM31; 2] = (*trace_2_col0.try_into().unwrap()).unbox();
    let [trace_2_col1_neg1, trace_2_col1]: [QM31; 2] = (*trace_2_col1.try_into().unwrap()).unbox();
    let [trace_2_col2_neg1, trace_2_col2]: [QM31; 2] = (*trace_2_col2.try_into().unwrap()).unbox();
    let [trace_2_col3_neg1, trace_2_col3]: [QM31; 2] = (*trace_2_col3.try_into().unwrap()).unbox();
    
    core::internal::revoke_ap_tracking();
    
    let constraint_quotient = (
        (
            (
                (QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3])) 
                    - (QM31Impl::from_partial_evals([trace_2_col0_neg1, trace_2_col1_neg1, trace_2_col2_neg1, trace_2_col3_neg1]))
                    + (claimed_sum * (column_size.inverse().into()))
            ) * range_check_4_4_4_4_sum_0
        ) + enabler
    ) * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

}
