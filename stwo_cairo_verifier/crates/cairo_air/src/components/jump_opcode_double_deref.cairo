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
use crate::components::subroutines::decode_instruction_9bd86::decode_instruction_9bd86_evaluate;
use crate::components::subroutines::read_positive_num_bits_27::read_positive_num_bits_27_evaluate;
use crate::components::verify_instruction::{
    VERIFY_INSTRUCTION_RELATION_SIZE, verify_instruction_sum,
};
use crate::components::{CairoComponent, OPCODES_RELATION_SIZE, opcodes_sum};
use crate::utils::U32Impl;


pub const N_TRACE_COLUMNS: usize = 17;


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
        let interaction_log_sizes = ArrayImpl::new_repeated(QM31_EXTENSION_DEGREE * 4, log_size)
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
    pub verify_instruction_lookup_elements: crate::VerifyInstructionElements,
    pub memory_address_to_id_lookup_elements: crate::MemoryAddressToIdElements,
    pub memory_id_to_big_lookup_elements: crate::MemoryIdToBigElements,
    pub opcodes_lookup_elements: crate::OpcodesElements,
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
        let mut verify_instruction_sum_0: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_1: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_2: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_3: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_4: QM31 = Zero::zero();
        let mut opcodes_sum_5: QM31 = Zero::zero();
        let mut opcodes_sum_6: QM31 = Zero::zero();
        let verify_instruction_alphas = self.verify_instruction_lookup_elements.alpha_powers.span();
        let verify_instruction_z = *self.verify_instruction_lookup_elements.z;
        let memory_address_to_id_alphas = self
            .memory_address_to_id_lookup_elements
            .alpha_powers
            .span();
        let memory_address_to_id_z = *self.memory_address_to_id_lookup_elements.z;
        let memory_id_to_big_alphas = self.memory_id_to_big_lookup_elements.alpha_powers.span();
        let memory_id_to_big_z = *self.memory_id_to_big_lookup_elements.z;
        let opcodes_alphas = self.opcodes_lookup_elements.alpha_powers.span();
        let opcodes_z = *self.opcodes_lookup_elements.z;

        let [
            input_pc_col0,
            input_ap_col1,
            input_fp_col2,
            offset1_col3,
            offset2_col4,
            op0_base_fp_col5,
            ap_update_add_1_col6,
            mem0_base_col7,
            mem1_base_id_col8,
            mem1_base_limb_0_col9,
            mem1_base_limb_1_col10,
            mem1_base_limb_2_col11,
            next_pc_id_col12,
            next_pc_limb_0_col13,
            next_pc_limb_1_col14,
            next_pc_limb_2_col15,
            enabler,
        ]: [Span<QM31>; 17] =
            (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();

        let [input_pc_col0]: [QM31; 1] = (*input_pc_col0.try_into().unwrap()).unbox();
        let [input_ap_col1]: [QM31; 1] = (*input_ap_col1.try_into().unwrap()).unbox();
        let [input_fp_col2]: [QM31; 1] = (*input_fp_col2.try_into().unwrap()).unbox();
        let [offset1_col3]: [QM31; 1] = (*offset1_col3.try_into().unwrap()).unbox();
        let [offset2_col4]: [QM31; 1] = (*offset2_col4.try_into().unwrap()).unbox();
        let [op0_base_fp_col5]: [QM31; 1] = (*op0_base_fp_col5.try_into().unwrap()).unbox();
        let [ap_update_add_1_col6]: [QM31; 1] = (*ap_update_add_1_col6.try_into().unwrap()).unbox();
        let [mem0_base_col7]: [QM31; 1] = (*mem0_base_col7.try_into().unwrap()).unbox();
        let [mem1_base_id_col8]: [QM31; 1] = (*mem1_base_id_col8.try_into().unwrap()).unbox();
        let [mem1_base_limb_0_col9]: [QM31; 1] = (*mem1_base_limb_0_col9.try_into().unwrap())
            .unbox();
        let [mem1_base_limb_1_col10]: [QM31; 1] = (*mem1_base_limb_1_col10.try_into().unwrap())
            .unbox();
        let [mem1_base_limb_2_col11]: [QM31; 1] = (*mem1_base_limb_2_col11.try_into().unwrap())
            .unbox();
        let [next_pc_id_col12]: [QM31; 1] = (*next_pc_id_col12.try_into().unwrap()).unbox();
        let [next_pc_limb_0_col13]: [QM31; 1] = (*next_pc_limb_0_col13.try_into().unwrap()).unbox();
        let [next_pc_limb_1_col14]: [QM31; 1] = (*next_pc_limb_1_col14.try_into().unwrap()).unbox();
        let [next_pc_limb_2_col15]: [QM31; 1] = (*next_pc_limb_2_col15.try_into().unwrap()).unbox();
        let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        let constraint_quotient = (enabler * enabler - enabler) * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        let output: [QM31; 19] = decode_instruction_9bd86_evaluate(
            input_pc_col0,
            offset1_col3,
            offset2_col4,
            op0_base_fp_col5,
            ap_update_add_1_col6,
            verify_instruction_alphas,
            verify_instruction_z,
            ref verify_instruction_sum_0,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let [
            decode_instruction_9bd86_output_tmp_22134_6_offset0,
            decode_instruction_9bd86_output_tmp_22134_6_offset1,
            decode_instruction_9bd86_output_tmp_22134_6_offset2,
            decode_instruction_9bd86_output_tmp_22134_6_dst_base_fp,
            decode_instruction_9bd86_output_tmp_22134_6_op0_base_fp,
            decode_instruction_9bd86_output_tmp_22134_6_op1_imm,
            decode_instruction_9bd86_output_tmp_22134_6_op1_base_fp,
            decode_instruction_9bd86_output_tmp_22134_6_op1_base_ap,
            decode_instruction_9bd86_output_tmp_22134_6_res_add,
            decode_instruction_9bd86_output_tmp_22134_6_res_mul,
            decode_instruction_9bd86_output_tmp_22134_6_pc_update_jump,
            decode_instruction_9bd86_output_tmp_22134_6_pc_update_jump_rel,
            decode_instruction_9bd86_output_tmp_22134_6_pc_update_jnz,
            decode_instruction_9bd86_output_tmp_22134_6_ap_update_add,
            decode_instruction_9bd86_output_tmp_22134_6_ap_update_add_1,
            decode_instruction_9bd86_output_tmp_22134_6_opcode_call,
            decode_instruction_9bd86_output_tmp_22134_6_opcode_ret,
            decode_instruction_9bd86_output_tmp_22134_6_opcode_assert_eq,
            decode_instruction_9bd86_output_tmp_22134_6_opcode_extension,
        ] =
            output;

        // Constraint - mem0_base
        let constraint_quotient = ((mem0_base_col7
            - ((op0_base_fp_col5 * input_fp_col2)
                + ((qm31_const::<1, 0, 0, 0>() - op0_base_fp_col5) * input_ap_col1))))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        let output: [QM31; 29] = read_positive_num_bits_27_evaluate(
            (mem0_base_col7 + decode_instruction_9bd86_output_tmp_22134_6_offset1),
            mem1_base_id_col8,
            mem1_base_limb_0_col9,
            mem1_base_limb_1_col10,
            mem1_base_limb_2_col11,
            memory_address_to_id_alphas,
            memory_address_to_id_z,
            memory_id_to_big_alphas,
            memory_id_to_big_z,
            ref memory_address_to_id_sum_1,
            ref memory_id_to_big_sum_2,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let [
            read_positive_num_bits_27_output_tmp_22134_9_limb_0,
            read_positive_num_bits_27_output_tmp_22134_9_limb_1,
            read_positive_num_bits_27_output_tmp_22134_9_limb_2,
            read_positive_num_bits_27_output_tmp_22134_9_limb_3,
            read_positive_num_bits_27_output_tmp_22134_9_limb_4,
            read_positive_num_bits_27_output_tmp_22134_9_limb_5,
            read_positive_num_bits_27_output_tmp_22134_9_limb_6,
            read_positive_num_bits_27_output_tmp_22134_9_limb_7,
            read_positive_num_bits_27_output_tmp_22134_9_limb_8,
            read_positive_num_bits_27_output_tmp_22134_9_limb_9,
            read_positive_num_bits_27_output_tmp_22134_9_limb_10,
            read_positive_num_bits_27_output_tmp_22134_9_limb_11,
            read_positive_num_bits_27_output_tmp_22134_9_limb_12,
            read_positive_num_bits_27_output_tmp_22134_9_limb_13,
            read_positive_num_bits_27_output_tmp_22134_9_limb_14,
            read_positive_num_bits_27_output_tmp_22134_9_limb_15,
            read_positive_num_bits_27_output_tmp_22134_9_limb_16,
            read_positive_num_bits_27_output_tmp_22134_9_limb_17,
            read_positive_num_bits_27_output_tmp_22134_9_limb_18,
            read_positive_num_bits_27_output_tmp_22134_9_limb_19,
            read_positive_num_bits_27_output_tmp_22134_9_limb_20,
            read_positive_num_bits_27_output_tmp_22134_9_limb_21,
            read_positive_num_bits_27_output_tmp_22134_9_limb_22,
            read_positive_num_bits_27_output_tmp_22134_9_limb_23,
            read_positive_num_bits_27_output_tmp_22134_9_limb_24,
            read_positive_num_bits_27_output_tmp_22134_9_limb_25,
            read_positive_num_bits_27_output_tmp_22134_9_limb_26,
            read_positive_num_bits_27_output_tmp_22134_9_limb_27,
            read_positive_num_bits_27_output_tmp_22134_9_limb_28,
        ] =
            output;

        let output: [QM31; 29] = read_positive_num_bits_27_evaluate(
            (((mem1_base_limb_0_col9 + (mem1_base_limb_1_col10 * qm31_const::<512, 0, 0, 0>()))
                + (mem1_base_limb_2_col11 * qm31_const::<262144, 0, 0, 0>()))
                + decode_instruction_9bd86_output_tmp_22134_6_offset2),
            next_pc_id_col12,
            next_pc_limb_0_col13,
            next_pc_limb_1_col14,
            next_pc_limb_2_col15,
            memory_address_to_id_alphas,
            memory_address_to_id_z,
            memory_id_to_big_alphas,
            memory_id_to_big_z,
            ref memory_address_to_id_sum_3,
            ref memory_id_to_big_sum_4,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let [
            read_positive_num_bits_27_output_tmp_22134_12_limb_0,
            read_positive_num_bits_27_output_tmp_22134_12_limb_1,
            read_positive_num_bits_27_output_tmp_22134_12_limb_2,
            read_positive_num_bits_27_output_tmp_22134_12_limb_3,
            read_positive_num_bits_27_output_tmp_22134_12_limb_4,
            read_positive_num_bits_27_output_tmp_22134_12_limb_5,
            read_positive_num_bits_27_output_tmp_22134_12_limb_6,
            read_positive_num_bits_27_output_tmp_22134_12_limb_7,
            read_positive_num_bits_27_output_tmp_22134_12_limb_8,
            read_positive_num_bits_27_output_tmp_22134_12_limb_9,
            read_positive_num_bits_27_output_tmp_22134_12_limb_10,
            read_positive_num_bits_27_output_tmp_22134_12_limb_11,
            read_positive_num_bits_27_output_tmp_22134_12_limb_12,
            read_positive_num_bits_27_output_tmp_22134_12_limb_13,
            read_positive_num_bits_27_output_tmp_22134_12_limb_14,
            read_positive_num_bits_27_output_tmp_22134_12_limb_15,
            read_positive_num_bits_27_output_tmp_22134_12_limb_16,
            read_positive_num_bits_27_output_tmp_22134_12_limb_17,
            read_positive_num_bits_27_output_tmp_22134_12_limb_18,
            read_positive_num_bits_27_output_tmp_22134_12_limb_19,
            read_positive_num_bits_27_output_tmp_22134_12_limb_20,
            read_positive_num_bits_27_output_tmp_22134_12_limb_21,
            read_positive_num_bits_27_output_tmp_22134_12_limb_22,
            read_positive_num_bits_27_output_tmp_22134_12_limb_23,
            read_positive_num_bits_27_output_tmp_22134_12_limb_24,
            read_positive_num_bits_27_output_tmp_22134_12_limb_25,
            read_positive_num_bits_27_output_tmp_22134_12_limb_26,
            read_positive_num_bits_27_output_tmp_22134_12_limb_27,
            read_positive_num_bits_27_output_tmp_22134_12_limb_28,
        ] =
            output;

        opcodes_sum_5 =
            opcodes_sum(opcodes_alphas, opcodes_z, [input_pc_col0, input_ap_col1, input_fp_col2]);

        opcodes_sum_6 =
            opcodes_sum(
                opcodes_alphas,
                opcodes_z,
                [
                    ((next_pc_limb_0_col13 + (next_pc_limb_1_col14 * qm31_const::<512, 0, 0, 0>()))
                        + (next_pc_limb_2_col15 * qm31_const::<262144, 0, 0, 0>())),
                    (input_ap_col1 + ap_update_add_1_col6), input_fp_col2,
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
            verify_instruction_sum_0,
            memory_address_to_id_sum_1,
            memory_id_to_big_sum_2,
            memory_address_to_id_sum_3,
            memory_id_to_big_sum_4,
            opcodes_sum_5,
            opcodes_sum_6,
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
    verify_instruction_sum_0: QM31,
    memory_address_to_id_sum_1: QM31,
    memory_id_to_big_sum_2: QM31,
    memory_address_to_id_sum_3: QM31,
    memory_id_to_big_sum_4: QM31,
    opcodes_sum_5: QM31,
    opcodes_sum_6: QM31,
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
    ]: [Span<QM31>; 16] =
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
    let [trace_2_col12_neg1, trace_2_col12]: [QM31; 2] = (*trace_2_col12.try_into().unwrap())
        .unbox();
    let [trace_2_col13_neg1, trace_2_col13]: [QM31; 2] = (*trace_2_col13.try_into().unwrap())
        .unbox();
    let [trace_2_col14_neg1, trace_2_col14]: [QM31; 2] = (*trace_2_col14.try_into().unwrap())
        .unbox();
    let [trace_2_col15_neg1, trace_2_col15]: [QM31; 2] = (*trace_2_col15.try_into().unwrap())
        .unbox();

    core::internal::revoke_ap_tracking();

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3],
    ))
        * verify_instruction_sum_0
        * memory_address_to_id_sum_1)
        - verify_instruction_sum_0
        - memory_address_to_id_sum_1)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7],
    )
        - QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3]))
        * memory_id_to_big_sum_2
        * memory_address_to_id_sum_3)
        - memory_id_to_big_sum_2
        - memory_address_to_id_sum_3)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11],
    )
        - QM31Impl::from_partial_evals([trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7]))
        * memory_id_to_big_sum_4
        * opcodes_sum_5)
        - (memory_id_to_big_sum_4 * enabler)
        - opcodes_sum_5)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
    )
        - QM31Impl::from_partial_evals([trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11])
        - QM31Impl::from_partial_evals(
            [trace_2_col12_neg1, trace_2_col13_neg1, trace_2_col14_neg1, trace_2_col15_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * opcodes_sum_6)
        + enabler)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
}
