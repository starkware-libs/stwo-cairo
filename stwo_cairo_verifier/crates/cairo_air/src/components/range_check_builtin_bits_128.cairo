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
use crate::components::memory_address_to_id::{
    MEMORY_ADDRESS_TO_ID_RELATION_SIZE, memory_address_to_id_sum,
};
use crate::components::memory_id_to_big::{MEMORY_ID_TO_BIG_RELATION_SIZE, memory_id_to_big_sum};
use crate::components::subroutines::read_positive_num_bits_128::read_positive_num_bits_128_evaluate;
use crate::components::{CairoComponent, OPCODES_RELATION_SIZE, opcodes_sum};
use crate::utils::U32Impl;


pub const N_TRACE_COLUMNS: usize = 17;


#[derive(Drop, Serde, Copy)]
pub struct Claim {
    pub log_size: u32,
    pub range_check_builtin_segment_start: u32,
}

#[generate_trait]
pub impl ClaimImpl of ClaimTrait {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = *(self.log_size);
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = ArrayImpl::new_repeated(N_TRACE_COLUMNS, log_size).span();
        let interaction_log_sizes = ArrayImpl::new_repeated(QM31_EXTENSION_DEGREE, log_size).span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        channel.mix_u64((*(self.log_size)).into());
        channel.mix_u64((*self.range_check_builtin_segment_start).into());
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
        let trace_gen = CanonicCosetImpl::new(log_size).coset.step_size;
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
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
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
        let range_check_builtin_segment_start: QM31 = (TryInto::<
            u32, M31,
        >::try_into((*(self.claim.range_check_builtin_segment_start)))
            .unwrap())
            .into();
        let mut memory_address_to_id_sum_0: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_1: QM31 = Zero::zero();
        let seq = preprocessed_mask_values.get(PreprocessedColumn::Seq(*(self.claim.log_size)));
        let memory_address_to_id_alphas = self
            .memory_address_to_id_lookup_elements
            .alpha_powers
            .span();
        let memory_address_to_id_z = *self.memory_address_to_id_lookup_elements.z;
        let memory_id_to_big_alphas = self.memory_id_to_big_lookup_elements.alpha_powers.span();
        let memory_id_to_big_z = *self.memory_id_to_big_lookup_elements.z;

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
            value_limb_11_col12,
            value_limb_12_col13,
            value_limb_13_col14,
            value_limb_14_col15,
            msb_col16,
        ]: [Span<QM31>; 17] =
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
        let [value_limb_11_col12]: [QM31; 1] = (*value_limb_11_col12.try_into().unwrap()).unbox();
        let [value_limb_12_col13]: [QM31; 1] = (*value_limb_12_col13.try_into().unwrap()).unbox();
        let [value_limb_13_col14]: [QM31; 1] = (*value_limb_13_col14.try_into().unwrap()).unbox();
        let [value_limb_14_col15]: [QM31; 1] = (*value_limb_14_col15.try_into().unwrap()).unbox();
        let [msb_col16]: [QM31; 1] = (*msb_col16.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        let output: [QM31; 29] = read_positive_num_bits_128_evaluate(
            (range_check_builtin_segment_start + seq),
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
            value_limb_11_col12,
            value_limb_12_col13,
            value_limb_13_col14,
            value_limb_14_col15,
            msb_col16,
            memory_address_to_id_alphas,
            memory_address_to_id_z,
            memory_id_to_big_alphas,
            memory_id_to_big_z,
            ref memory_address_to_id_sum_0,
            ref memory_id_to_big_sum_1,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let [
            read_positive_num_bits_128_output_tmp_66b3a_4_limb_0,
            read_positive_num_bits_128_output_tmp_66b3a_4_limb_1,
            read_positive_num_bits_128_output_tmp_66b3a_4_limb_2,
            read_positive_num_bits_128_output_tmp_66b3a_4_limb_3,
            read_positive_num_bits_128_output_tmp_66b3a_4_limb_4,
            read_positive_num_bits_128_output_tmp_66b3a_4_limb_5,
            read_positive_num_bits_128_output_tmp_66b3a_4_limb_6,
            read_positive_num_bits_128_output_tmp_66b3a_4_limb_7,
            read_positive_num_bits_128_output_tmp_66b3a_4_limb_8,
            read_positive_num_bits_128_output_tmp_66b3a_4_limb_9,
            read_positive_num_bits_128_output_tmp_66b3a_4_limb_10,
            read_positive_num_bits_128_output_tmp_66b3a_4_limb_11,
            read_positive_num_bits_128_output_tmp_66b3a_4_limb_12,
            read_positive_num_bits_128_output_tmp_66b3a_4_limb_13,
            read_positive_num_bits_128_output_tmp_66b3a_4_limb_14,
            read_positive_num_bits_128_output_tmp_66b3a_4_limb_15,
            read_positive_num_bits_128_output_tmp_66b3a_4_limb_16,
            read_positive_num_bits_128_output_tmp_66b3a_4_limb_17,
            read_positive_num_bits_128_output_tmp_66b3a_4_limb_18,
            read_positive_num_bits_128_output_tmp_66b3a_4_limb_19,
            read_positive_num_bits_128_output_tmp_66b3a_4_limb_20,
            read_positive_num_bits_128_output_tmp_66b3a_4_limb_21,
            read_positive_num_bits_128_output_tmp_66b3a_4_limb_22,
            read_positive_num_bits_128_output_tmp_66b3a_4_limb_23,
            read_positive_num_bits_128_output_tmp_66b3a_4_limb_24,
            read_positive_num_bits_128_output_tmp_66b3a_4_limb_25,
            read_positive_num_bits_128_output_tmp_66b3a_4_limb_26,
            read_positive_num_bits_128_output_tmp_66b3a_4_limb_27,
            read_positive_num_bits_128_output_tmp_66b3a_4_limb_28,
        ] =
            output;

        lookup_constraints(
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
            claimed_sum,
            column_size,
            ref interaction_trace_mask_values,
            memory_address_to_id_sum_0,
            memory_id_to_big_sum_1,
        );
    }
}

#[cairofmt::skip]
fn lookup_constraints(
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
    claimed_sum: QM31,
    column_size: M31,
    ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
    memory_address_to_id_sum_0: QM31,
    memory_id_to_big_sum_1: QM31
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
            ) * memory_address_to_id_sum_0 * memory_id_to_big_sum_1
        ) - memory_address_to_id_sum_0 - memory_id_to_big_sum_1
    ) * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

}
