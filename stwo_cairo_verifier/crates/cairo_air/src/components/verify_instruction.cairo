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
use crate::components::range_check_4_3::{RANGE_CHECK_4_3_RELATION_SIZE, range_check_4_3_sum};
use crate::components::range_check_7_2_5::{RANGE_CHECK_7_2_5_RELATION_SIZE, range_check_7_2_5_sum};
use crate::components::subroutines::encode_offsets::encode_offsets_evaluate;
use crate::components::subroutines::mem_verify::mem_verify_evaluate;
use crate::components::{CairoComponent, OPCODES_RELATION_SIZE, opcodes_sum};
use crate::utils::U32Impl;


pub const N_TRACE_COLUMNS: usize = 17;
pub const VERIFY_INSTRUCTION_RELATION_SIZE: usize = 7;


#[derive(Drop, Serde, Copy)]
pub struct Claim {
    pub log_size: u32,
}

#[generate_trait]
pub impl ClaimImpl of ClaimTrait {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = *(self.log_size);
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = ArrayImpl::new_repeated(N_TRACE_COLUMNS, log_size).span();
        let interaction_log_sizes = ArrayImpl::new_repeated(QM31_EXTENSION_DEGREE * 3, log_size)
            .span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        channel.mix_u64((*(self.log_size)).into());
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
    pub range_check_7_2_5_lookup_elements: crate::RangeCheck_7_2_5Elements,
    pub range_check_4_3_lookup_elements: crate::RangeCheck_4_3Elements,
    pub memory_address_to_id_lookup_elements: crate::MemoryAddressToIdElements,
    pub memory_id_to_big_lookup_elements: crate::MemoryIdToBigElements,
    pub verify_instruction_lookup_elements: crate::VerifyInstructionElements,
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
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
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
        let mut range_check_7_2_5_sum_0: QM31 = Zero::zero();
        let mut range_check_4_3_sum_1: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_2: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_3: QM31 = Zero::zero();
        let mut verify_instruction_sum_4: QM31 = Zero::zero();
        let range_check_7_2_5_alphas = self.range_check_7_2_5_lookup_elements.alpha_powers.span();
        let range_check_7_2_5_z = *self.range_check_7_2_5_lookup_elements.z;
        let range_check_4_3_alphas = self.range_check_4_3_lookup_elements.alpha_powers.span();
        let range_check_4_3_z = *self.range_check_4_3_lookup_elements.z;
        let memory_address_to_id_alphas = self
            .memory_address_to_id_lookup_elements
            .alpha_powers
            .span();
        let memory_address_to_id_z = *self.memory_address_to_id_lookup_elements.z;
        let memory_id_to_big_alphas = self.memory_id_to_big_lookup_elements.alpha_powers.span();
        let memory_id_to_big_z = *self.memory_id_to_big_lookup_elements.z;
        let verify_instruction_alphas = self.verify_instruction_lookup_elements.alpha_powers.span();
        let verify_instruction_z = *self.verify_instruction_lookup_elements.z;

        let [
            input_pc_col0,
            input_offset0_col1,
            input_offset1_col2,
            input_offset2_col3,
            input_inst_felt5_high_col4,
            input_inst_felt6_col5,
            input_opcode_extension_col6,
            offset0_low_col7,
            offset0_mid_col8,
            offset1_low_col9,
            offset1_mid_col10,
            offset1_high_col11,
            offset2_low_col12,
            offset2_mid_col13,
            offset2_high_col14,
            instruction_id_col15,
            enabler,
        ]: [Span<QM31>; 17] =
            (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();

        let [input_pc_col0]: [QM31; 1] = (*input_pc_col0.try_into().unwrap()).unbox();
        let [input_offset0_col1]: [QM31; 1] = (*input_offset0_col1.try_into().unwrap()).unbox();
        let [input_offset1_col2]: [QM31; 1] = (*input_offset1_col2.try_into().unwrap()).unbox();
        let [input_offset2_col3]: [QM31; 1] = (*input_offset2_col3.try_into().unwrap()).unbox();
        let [input_inst_felt5_high_col4]: [QM31; 1] = (*input_inst_felt5_high_col4
            .try_into()
            .unwrap())
            .unbox();
        let [input_inst_felt6_col5]: [QM31; 1] = (*input_inst_felt6_col5.try_into().unwrap())
            .unbox();
        let [input_opcode_extension_col6]: [QM31; 1] = (*input_opcode_extension_col6
            .try_into()
            .unwrap())
            .unbox();
        let [offset0_low_col7]: [QM31; 1] = (*offset0_low_col7.try_into().unwrap()).unbox();
        let [offset0_mid_col8]: [QM31; 1] = (*offset0_mid_col8.try_into().unwrap()).unbox();
        let [offset1_low_col9]: [QM31; 1] = (*offset1_low_col9.try_into().unwrap()).unbox();
        let [offset1_mid_col10]: [QM31; 1] = (*offset1_mid_col10.try_into().unwrap()).unbox();
        let [offset1_high_col11]: [QM31; 1] = (*offset1_high_col11.try_into().unwrap()).unbox();
        let [offset2_low_col12]: [QM31; 1] = (*offset2_low_col12.try_into().unwrap()).unbox();
        let [offset2_mid_col13]: [QM31; 1] = (*offset2_mid_col13.try_into().unwrap()).unbox();
        let [offset2_high_col14]: [QM31; 1] = (*offset2_high_col14.try_into().unwrap()).unbox();
        let [instruction_id_col15]: [QM31; 1] = (*instruction_id_col15.try_into().unwrap()).unbox();
        let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        let output: [QM31; 6] = encode_offsets_evaluate(
            [input_offset0_col1, input_offset1_col2, input_offset2_col3],
            offset0_low_col7,
            offset0_mid_col8,
            offset1_low_col9,
            offset1_mid_col10,
            offset1_high_col11,
            offset2_low_col12,
            offset2_mid_col13,
            offset2_high_col14,
            range_check_7_2_5_alphas,
            range_check_7_2_5_z,
            range_check_4_3_alphas,
            range_check_4_3_z,
            ref range_check_7_2_5_sum_0,
            ref range_check_4_3_sum_1,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let [
            encode_offsets_output_tmp_16a4f_8_limb_0,
            encode_offsets_output_tmp_16a4f_8_limb_1,
            encode_offsets_output_tmp_16a4f_8_limb_2,
            encode_offsets_output_tmp_16a4f_8_limb_3,
            encode_offsets_output_tmp_16a4f_8_limb_4,
            encode_offsets_output_tmp_16a4f_8_limb_5,
        ] =
            output;

        mem_verify_evaluate(
            [
                input_pc_col0, offset0_low_col7, encode_offsets_output_tmp_16a4f_8_limb_1,
                offset1_mid_col10, encode_offsets_output_tmp_16a4f_8_limb_3, offset2_mid_col13,
                (offset2_high_col14 + input_inst_felt5_high_col4), input_inst_felt6_col5,
                input_opcode_extension_col6, qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
                qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
                qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
                qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
                qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
                qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
                qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(), qm31_const::<0, 0, 0, 0>(),
            ],
            instruction_id_col15,
            memory_address_to_id_alphas,
            memory_address_to_id_z,
            memory_id_to_big_alphas,
            memory_id_to_big_z,
            ref memory_address_to_id_sum_2,
            ref memory_id_to_big_sum_3,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        verify_instruction_sum_4 =
            verify_instruction_sum(
                verify_instruction_alphas,
                verify_instruction_z,
                [
                    input_pc_col0, input_offset0_col1, input_offset1_col2, input_offset2_col3,
                    input_inst_felt5_high_col4, input_inst_felt6_col5, input_opcode_extension_col6,
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
            range_check_7_2_5_sum_0,
            range_check_4_3_sum_1,
            memory_address_to_id_sum_2,
            memory_id_to_big_sum_3,
            verify_instruction_sum_4,
        );
    }
}


pub fn verify_instruction_sum(mut alphas: Span<QM31>, z: QM31, values: [QM31; 7]) -> QM31 {
    let [alpha0, alpha1, alpha2, alpha3, alpha4, alpha5, alpha6] = (*alphas
        .multi_pop_front()
        .unwrap())
        .unbox();
    let [val0, val1, val2, val3, val4, val5, val6] = values;

    alpha0 * val0
        + alpha1 * val1
        + alpha2 * val2
        + alpha3 * val3
        + alpha4 * val4
        + alpha5 * val5
        + alpha6 * val6
        - z
}

fn lookup_constraints(
    ref sum: QM31,
    domain_vanishing_eval_inv: QM31,
    random_coeff: QM31,
    claimed_sum: QM31,
    enabler: QM31,
    column_size: M31,
    ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
    range_check_7_2_5_sum_0: QM31,
    range_check_4_3_sum_1: QM31,
    memory_address_to_id_sum_2: QM31,
    memory_id_to_big_sum_3: QM31,
    verify_instruction_sum_4: QM31,
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
    ]: [Span<QM31>; 12] =
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
    let [trace_2_col8_neg1, trace_2_col8]: [QM31; 2] = (*trace_2_col8.try_into().unwrap()).unbox();
    let [trace_2_col9_neg1, trace_2_col9]: [QM31; 2] = (*trace_2_col9.try_into().unwrap()).unbox();
    let [trace_2_col10_neg1, trace_2_col10]: [QM31; 2] = (*trace_2_col10.try_into().unwrap())
        .unbox();
    let [trace_2_col11_neg1, trace_2_col11]: [QM31; 2] = (*trace_2_col11.try_into().unwrap())
        .unbox();

    core::internal::revoke_ap_tracking();

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3],
    ))
        * range_check_7_2_5_sum_0
        * range_check_4_3_sum_1)
        - range_check_7_2_5_sum_0
        - range_check_4_3_sum_1)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7],
    )
        - QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3]))
        * memory_address_to_id_sum_2
        * memory_id_to_big_sum_3)
        - memory_address_to_id_sum_2
        - memory_id_to_big_sum_3)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11],
    )
        - QM31Impl::from_partial_evals([trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7])
        - QM31Impl::from_partial_evals(
            [trace_2_col8_neg1, trace_2_col9_neg1, trace_2_col10_neg1, trace_2_col11_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * verify_instruction_sum_4)
        + enabler)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
}
