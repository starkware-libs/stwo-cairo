// Constraints version: bc855610

use core::num::traits::Zero;
use stwo_constraint_framework::{
    LookupElementsImpl, PreprocessedColumn, PreprocessedColumnSet, PreprocessedColumnSetImpl,
    PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::circle::{
    CirclePoint, CirclePointIndexTrait, CirclePointQM31AddCirclePointM31Trait,
};
use stwo_verifier_core::fields::Invertible;
use stwo_verifier_core::fields::m31::{M31, m31};
use stwo_verifier_core::fields::qm31::{QM31, QM31Impl, QM31Serde, QM31Zero, qm31_const};
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::{ArrayImpl, pow2};
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray};
use crate::components::CairoComponent;
use crate::components::subroutines::read_positive_num_bits_96::read_positive_num_bits_96_evaluate;
use crate::utils::U32Impl;

pub const N_TRACE_COLUMNS: usize = 12;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 3] = [
    ('MemoryAddressToId', 1), ('RangeCheck_6', 1), ('MemoryIdToBig', 1),
];

#[derive(Drop, Serde, Copy)]
pub struct Claim {
    pub log_size: u32,
    pub range_check96_builtin_segment_start: u32,
}

#[generate_trait]
pub impl ClaimImpl of ClaimTrait {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = *(self.log_size);
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = ArrayImpl::new_repeated(N_TRACE_COLUMNS, log_size).span();
        let interaction_log_sizes = ArrayImpl::new_repeated(8, log_size).span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        channel.mix_u64((*(self.log_size)).into());
        channel.mix_u64((*self.range_check96_builtin_segment_start).into());
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
    pub memory_address_to_id_lookup_elements: crate::MemoryAddressToIdElements,
    pub range_check_6_lookup_elements: crate::RangeCheck_6Elements,
    pub memory_id_to_big_lookup_elements: crate::MemoryIdToBigElements,
}

pub impl ComponentImpl of CairoComponent<Component> {
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
        preprocessed_column_set.insert(PreprocessedColumn::Seq(*(self.claim.log_size)));
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point_offset_neg_1, point]);
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
        let range_check96_builtin_segment_start: QM31 = (TryInto::<
            u32, M31,
        >::try_into((*(self.claim.range_check96_builtin_segment_start)))
            .unwrap())
            .into();
        let mut memory_address_to_id_sum_0: QM31 = Zero::zero();
        let mut range_check_6_sum_1: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_2: QM31 = Zero::zero();
        let seq = preprocessed_mask_values.get(PreprocessedColumn::Seq(*(self.claim.log_size)));

        let [
            value_id_col0,
            value_limb_0_col1,
            value_limb_1_col2,
            value_limb_2_col3,
            value_limb_3_col4,
            value_limb_4_col5,
            value_limb_5_col6,
            value_limb_6_col7,
            value_limb_7_col8,
            value_limb_8_col9,
            value_limb_9_col10,
            value_limb_10_col11,
        ]: [Span<QM31>; 12] =
            (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [value_id_col0]: [QM31; 1] = (*value_id_col0.try_into().unwrap()).unbox();
        let [value_limb_0_col1]: [QM31; 1] = (*value_limb_0_col1.try_into().unwrap()).unbox();
        let [value_limb_1_col2]: [QM31; 1] = (*value_limb_1_col2.try_into().unwrap()).unbox();
        let [value_limb_2_col3]: [QM31; 1] = (*value_limb_2_col3.try_into().unwrap()).unbox();
        let [value_limb_3_col4]: [QM31; 1] = (*value_limb_3_col4.try_into().unwrap()).unbox();
        let [value_limb_4_col5]: [QM31; 1] = (*value_limb_4_col5.try_into().unwrap()).unbox();
        let [value_limb_5_col6]: [QM31; 1] = (*value_limb_5_col6.try_into().unwrap()).unbox();
        let [value_limb_6_col7]: [QM31; 1] = (*value_limb_6_col7.try_into().unwrap()).unbox();
        let [value_limb_7_col8]: [QM31; 1] = (*value_limb_7_col8.try_into().unwrap()).unbox();
        let [value_limb_8_col9]: [QM31; 1] = (*value_limb_8_col9.try_into().unwrap()).unbox();
        let [value_limb_9_col10]: [QM31; 1] = (*value_limb_9_col10.try_into().unwrap()).unbox();
        let [value_limb_10_col11]: [QM31; 1] = (*value_limb_10_col11.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        read_positive_num_bits_96_evaluate(
            [(range_check96_builtin_segment_start + seq)],
            value_id_col0,
            value_limb_0_col1,
            value_limb_1_col2,
            value_limb_2_col3,
            value_limb_3_col4,
            value_limb_4_col5,
            value_limb_5_col6,
            value_limb_6_col7,
            value_limb_7_col8,
            value_limb_8_col9,
            value_limb_9_col10,
            value_limb_10_col11,
            self.memory_address_to_id_lookup_elements,
            self.range_check_6_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref memory_address_to_id_sum_0,
            ref range_check_6_sum_1,
            ref memory_id_to_big_sum_2,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        lookup_constraints(
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
            claimed_sum,
            column_size,
            ref interaction_trace_mask_values,
            memory_address_to_id_sum_0,
            range_check_6_sum_1,
            memory_id_to_big_sum_2,
        );
    }
}


fn lookup_constraints(
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
    claimed_sum: QM31,
    column_size: M31,
    ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
    memory_address_to_id_sum_0: QM31,
    range_check_6_sum_1: QM31,
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
        * memory_address_to_id_sum_0
        * range_check_6_sum_1)
        - memory_address_to_id_sum_0
        - range_check_6_sum_1)
        * domain_vanishing_eval_inv;
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
        - qm31_const::<1, 0, 0, 0>())
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
}
