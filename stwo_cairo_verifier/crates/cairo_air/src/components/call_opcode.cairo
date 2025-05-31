// Constraints version: 1d0330d7-dirty

use core::num::traits::Zero;
use stwo_constraint_framework::{
    PreprocessedColumn, PreprocessedColumnSet, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
    PreprocessedColumnSetImpl, LookupElementsImpl,
};
use stwo_verifier_core::circle::CirclePointQM31AddCirclePointM31Trait;
use stwo_verifier_core::circle::CirclePointIndexTrait;
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::Invertible;
use stwo_verifier_core::fields::m31::{m31, M31};
use stwo_verifier_core::fields::qm31::{qm31_const, QM31, QM31Impl, QM31Serde, QM31Zero};
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::{ArrayImpl, pow2};
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray};
use crate::components::CairoComponent;
use crate::utils::U32Impl;use crate::components::subroutines::decode_instruction_f1edd::decode_instruction_f1edd_evaluate;
use crate::components::subroutines::read_positive_num_bits_27::read_positive_num_bits_27_evaluate;

pub const N_TRACE_COLUMNS: usize = 19;

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
        let interaction_log_sizes = ArrayImpl::new_repeated(20, log_size).span();
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
        let trace_gen = CanonicCosetImpl::new(log_size).coset.step;
        let point_offset_neg_1 = point.add_circle_point_m31(-trace_gen.mul(1).to_point());
        trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);trace_mask_points.append(array![point]);
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


        let [
            input_pc_col0,
            input_ap_col1,
            input_fp_col2,
            offset2_col3,
            op1_base_fp_col4,
            stored_fp_id_col5,
            stored_fp_limb_0_col6,
            stored_fp_limb_1_col7,
            stored_fp_limb_2_col8,
            stored_ret_pc_id_col9,
            stored_ret_pc_limb_0_col10,
            stored_ret_pc_limb_1_col11,
            stored_ret_pc_limb_2_col12,
            mem1_base_col13,
            next_pc_id_col14,
            next_pc_limb_0_col15,
            next_pc_limb_1_col16,
            next_pc_limb_2_col17,
            enabler
        ]: [Span<QM31>; 19] =
            (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();

        let [input_pc_col0]: [QM31; 1] = (*input_pc_col0.try_into().unwrap()).unbox();
        let [input_ap_col1]: [QM31; 1] = (*input_ap_col1.try_into().unwrap()).unbox();
        let [input_fp_col2]: [QM31; 1] = (*input_fp_col2.try_into().unwrap()).unbox();
        let [offset2_col3]: [QM31; 1] = (*offset2_col3.try_into().unwrap()).unbox();
        let [op1_base_fp_col4]: [QM31; 1] = (*op1_base_fp_col4.try_into().unwrap()).unbox();
        let [stored_fp_id_col5]: [QM31; 1] = (*stored_fp_id_col5.try_into().unwrap()).unbox();
        let [stored_fp_limb_0_col6]: [QM31; 1] = (*stored_fp_limb_0_col6.try_into().unwrap()).unbox();
        let [stored_fp_limb_1_col7]: [QM31; 1] = (*stored_fp_limb_1_col7.try_into().unwrap()).unbox();
        let [stored_fp_limb_2_col8]: [QM31; 1] = (*stored_fp_limb_2_col8.try_into().unwrap()).unbox();
        let [stored_ret_pc_id_col9]: [QM31; 1] = (*stored_ret_pc_id_col9.try_into().unwrap()).unbox();
        let [stored_ret_pc_limb_0_col10]: [QM31; 1] = (*stored_ret_pc_limb_0_col10.try_into().unwrap()).unbox();
        let [stored_ret_pc_limb_1_col11]: [QM31; 1] = (*stored_ret_pc_limb_1_col11.try_into().unwrap()).unbox();
        let [stored_ret_pc_limb_2_col12]: [QM31; 1] = (*stored_ret_pc_limb_2_col12.try_into().unwrap()).unbox();
        let [mem1_base_col13]: [QM31; 1] = (*mem1_base_col13.try_into().unwrap()).unbox();
        let [next_pc_id_col14]: [QM31; 1] = (*next_pc_id_col14.try_into().unwrap()).unbox();
        let [next_pc_limb_0_col15]: [QM31; 1] = (*next_pc_limb_0_col15.try_into().unwrap()).unbox();
        let [next_pc_limb_1_col16]: [QM31; 1] = (*next_pc_limb_1_col16.try_into().unwrap()).unbox();
        let [next_pc_limb_2_col17]: [QM31; 1] = (*next_pc_limb_2_col17.try_into().unwrap()).unbox();
        let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();


    }
}
