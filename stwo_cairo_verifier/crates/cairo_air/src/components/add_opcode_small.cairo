// Constraints version: 252b9d8a

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
use crate::components::subroutines::decode_instruction_bc3cd::decode_instruction_bc3cd_evaluate;
use crate::components::subroutines::read_small::read_small_evaluate;
use crate::components::verify_instruction::{
    VERIFY_INSTRUCTION_RELATION_SIZE, verify_instruction_sum,
};
use crate::components::{CairoComponent, OPCODES_RELATION_SIZE, opcodes_sum};
use crate::utils::U32Impl;


pub const N_TRACE_COLUMNS: usize = 33;


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
        let interaction_log_sizes = ArrayImpl::new_repeated(QM31_EXTENSION_DEGREE * 5, log_size)
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
        let mut memory_address_to_id_sum_5: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_6: QM31 = Zero::zero();
        let mut opcodes_sum_7: QM31 = Zero::zero();
        let mut opcodes_sum_8: QM31 = Zero::zero();
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
            offset0_col3,
            offset1_col4,
            offset2_col5,
            dst_base_fp_col6,
            op0_base_fp_col7,
            op1_imm_col8,
            op1_base_fp_col9,
            ap_update_add_1_col10,
            mem_dst_base_col11,
            mem0_base_col12,
            mem1_base_col13,
            dst_id_col14,
            msb_col15,
            mid_limbs_set_col16,
            dst_limb_0_col17,
            dst_limb_1_col18,
            dst_limb_2_col19,
            op0_id_col20,
            msb_col21,
            mid_limbs_set_col22,
            op0_limb_0_col23,
            op0_limb_1_col24,
            op0_limb_2_col25,
            op1_id_col26,
            msb_col27,
            mid_limbs_set_col28,
            op1_limb_0_col29,
            op1_limb_1_col30,
            op1_limb_2_col31,
            enabler,
        ]: [Span<QM31>; 33] =
            (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();

        let [input_pc_col0]: [QM31; 1] = (*input_pc_col0.try_into().unwrap()).unbox();
        let [input_ap_col1]: [QM31; 1] = (*input_ap_col1.try_into().unwrap()).unbox();
        let [input_fp_col2]: [QM31; 1] = (*input_fp_col2.try_into().unwrap()).unbox();
        let [offset0_col3]: [QM31; 1] = (*offset0_col3.try_into().unwrap()).unbox();
        let [offset1_col4]: [QM31; 1] = (*offset1_col4.try_into().unwrap()).unbox();
        let [offset2_col5]: [QM31; 1] = (*offset2_col5.try_into().unwrap()).unbox();
        let [dst_base_fp_col6]: [QM31; 1] = (*dst_base_fp_col6.try_into().unwrap()).unbox();
        let [op0_base_fp_col7]: [QM31; 1] = (*op0_base_fp_col7.try_into().unwrap()).unbox();
        let [op1_imm_col8]: [QM31; 1] = (*op1_imm_col8.try_into().unwrap()).unbox();
        let [op1_base_fp_col9]: [QM31; 1] = (*op1_base_fp_col9.try_into().unwrap()).unbox();
        let [ap_update_add_1_col10]: [QM31; 1] = (*ap_update_add_1_col10.try_into().unwrap())
            .unbox();
        let [mem_dst_base_col11]: [QM31; 1] = (*mem_dst_base_col11.try_into().unwrap()).unbox();
        let [mem0_base_col12]: [QM31; 1] = (*mem0_base_col12.try_into().unwrap()).unbox();
        let [mem1_base_col13]: [QM31; 1] = (*mem1_base_col13.try_into().unwrap()).unbox();
        let [dst_id_col14]: [QM31; 1] = (*dst_id_col14.try_into().unwrap()).unbox();
        let [msb_col15]: [QM31; 1] = (*msb_col15.try_into().unwrap()).unbox();
        let [mid_limbs_set_col16]: [QM31; 1] = (*mid_limbs_set_col16.try_into().unwrap()).unbox();
        let [dst_limb_0_col17]: [QM31; 1] = (*dst_limb_0_col17.try_into().unwrap()).unbox();
        let [dst_limb_1_col18]: [QM31; 1] = (*dst_limb_1_col18.try_into().unwrap()).unbox();
        let [dst_limb_2_col19]: [QM31; 1] = (*dst_limb_2_col19.try_into().unwrap()).unbox();
        let [op0_id_col20]: [QM31; 1] = (*op0_id_col20.try_into().unwrap()).unbox();
        let [msb_col21]: [QM31; 1] = (*msb_col21.try_into().unwrap()).unbox();
        let [mid_limbs_set_col22]: [QM31; 1] = (*mid_limbs_set_col22.try_into().unwrap()).unbox();
        let [op0_limb_0_col23]: [QM31; 1] = (*op0_limb_0_col23.try_into().unwrap()).unbox();
        let [op0_limb_1_col24]: [QM31; 1] = (*op0_limb_1_col24.try_into().unwrap()).unbox();
        let [op0_limb_2_col25]: [QM31; 1] = (*op0_limb_2_col25.try_into().unwrap()).unbox();
        let [op1_id_col26]: [QM31; 1] = (*op1_id_col26.try_into().unwrap()).unbox();
        let [msb_col27]: [QM31; 1] = (*msb_col27.try_into().unwrap()).unbox();
        let [mid_limbs_set_col28]: [QM31; 1] = (*mid_limbs_set_col28.try_into().unwrap()).unbox();
        let [op1_limb_0_col29]: [QM31; 1] = (*op1_limb_0_col29.try_into().unwrap()).unbox();
        let [op1_limb_1_col30]: [QM31; 1] = (*op1_limb_1_col30.try_into().unwrap()).unbox();
        let [op1_limb_2_col31]: [QM31; 1] = (*op1_limb_2_col31.try_into().unwrap()).unbox();
        let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        let constraint_quotient = (enabler * enabler - enabler) * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        let output: [QM31; 19] = decode_instruction_bc3cd_evaluate(
            input_pc_col0,
            offset0_col3,
            offset1_col4,
            offset2_col5,
            dst_base_fp_col6,
            op0_base_fp_col7,
            op1_imm_col8,
            op1_base_fp_col9,
            ap_update_add_1_col10,
            verify_instruction_alphas,
            verify_instruction_z,
            ref verify_instruction_sum_0,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let [
            decode_instruction_bc3cd_output_tmp_756b7_10_offset0,
            decode_instruction_bc3cd_output_tmp_756b7_10_offset1,
            decode_instruction_bc3cd_output_tmp_756b7_10_offset2,
            decode_instruction_bc3cd_output_tmp_756b7_10_dst_base_fp,
            decode_instruction_bc3cd_output_tmp_756b7_10_op0_base_fp,
            decode_instruction_bc3cd_output_tmp_756b7_10_op1_imm,
            decode_instruction_bc3cd_output_tmp_756b7_10_op1_base_fp,
            decode_instruction_bc3cd_output_tmp_756b7_10_op1_base_ap,
            decode_instruction_bc3cd_output_tmp_756b7_10_res_add,
            decode_instruction_bc3cd_output_tmp_756b7_10_res_mul,
            decode_instruction_bc3cd_output_tmp_756b7_10_pc_update_jump,
            decode_instruction_bc3cd_output_tmp_756b7_10_pc_update_jump_rel,
            decode_instruction_bc3cd_output_tmp_756b7_10_pc_update_jnz,
            decode_instruction_bc3cd_output_tmp_756b7_10_ap_update_add,
            decode_instruction_bc3cd_output_tmp_756b7_10_ap_update_add_1,
            decode_instruction_bc3cd_output_tmp_756b7_10_opcode_call,
            decode_instruction_bc3cd_output_tmp_756b7_10_opcode_ret,
            decode_instruction_bc3cd_output_tmp_756b7_10_opcode_assert_eq,
            decode_instruction_bc3cd_output_tmp_756b7_10_opcode_extension,
        ] =
            output;

        // Constraint - if imm then offset2 is 1
        let constraint_quotient = ((op1_imm_col8
            * (qm31_const::<1, 0, 0, 0>() - decode_instruction_bc3cd_output_tmp_756b7_10_offset2)))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - mem_dst_base
        let constraint_quotient = ((mem_dst_base_col11
            - ((dst_base_fp_col6 * input_fp_col2)
                + ((qm31_const::<1, 0, 0, 0>() - dst_base_fp_col6) * input_ap_col1))))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - mem0_base
        let constraint_quotient = ((mem0_base_col12
            - ((op0_base_fp_col7 * input_fp_col2)
                + ((qm31_const::<1, 0, 0, 0>() - op0_base_fp_col7) * input_ap_col1))))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - mem1_base
        let constraint_quotient = ((mem1_base_col13
            - (((op1_imm_col8 * input_pc_col0) + (op1_base_fp_col9 * input_fp_col2))
                + (decode_instruction_bc3cd_output_tmp_756b7_10_op1_base_ap * input_ap_col1))))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        let output: [QM31; 2] = read_small_evaluate(
            (mem_dst_base_col11 + decode_instruction_bc3cd_output_tmp_756b7_10_offset0),
            dst_id_col14,
            msb_col15,
            mid_limbs_set_col16,
            dst_limb_0_col17,
            dst_limb_1_col18,
            dst_limb_2_col19,
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
        let [read_small_output_tmp_756b7_16_limb_0, read_small_output_tmp_756b7_16_limb_1] = output;

        let output: [QM31; 2] = read_small_evaluate(
            (mem0_base_col12 + decode_instruction_bc3cd_output_tmp_756b7_10_offset1),
            op0_id_col20,
            msb_col21,
            mid_limbs_set_col22,
            op0_limb_0_col23,
            op0_limb_1_col24,
            op0_limb_2_col25,
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
        let [read_small_output_tmp_756b7_22_limb_0, read_small_output_tmp_756b7_22_limb_1] = output;

        let output: [QM31; 2] = read_small_evaluate(
            (mem1_base_col13 + decode_instruction_bc3cd_output_tmp_756b7_10_offset2),
            op1_id_col26,
            msb_col27,
            mid_limbs_set_col28,
            op1_limb_0_col29,
            op1_limb_1_col30,
            op1_limb_2_col31,
            memory_address_to_id_alphas,
            memory_address_to_id_z,
            memory_id_to_big_alphas,
            memory_id_to_big_z,
            ref memory_address_to_id_sum_5,
            ref memory_id_to_big_sum_6,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let [read_small_output_tmp_756b7_28_limb_0, read_small_output_tmp_756b7_28_limb_1] = output;

        // Constraint - dst equals op0 + op1
        let constraint_quotient = ((read_small_output_tmp_756b7_16_limb_0
            - (read_small_output_tmp_756b7_22_limb_0 + read_small_output_tmp_756b7_28_limb_0)))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        opcodes_sum_7 =
            opcodes_sum(opcodes_alphas, opcodes_z, [input_pc_col0, input_ap_col1, input_fp_col2]);

        opcodes_sum_8 =
            opcodes_sum(
                opcodes_alphas,
                opcodes_z,
                [
                    ((input_pc_col0 + qm31_const::<1, 0, 0, 0>()) + op1_imm_col8),
                    (input_ap_col1 + ap_update_add_1_col10), input_fp_col2,
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
            memory_address_to_id_sum_5,
            memory_id_to_big_sum_6,
            opcodes_sum_7,
            opcodes_sum_8,
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
    memory_address_to_id_sum_5: QM31,
    memory_id_to_big_sum_6: QM31,
    opcodes_sum_7: QM31,
    opcodes_sum_8: QM31,
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
        * memory_address_to_id_sum_5)
        - memory_id_to_big_sum_4
        - memory_address_to_id_sum_5)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
    )
        - QM31Impl::from_partial_evals([trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11]))
        * memory_id_to_big_sum_6
        * opcodes_sum_7)
        - (memory_id_to_big_sum_6 * enabler)
        - opcodes_sum_7)
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
        * opcodes_sum_8)
        + enabler)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
}
