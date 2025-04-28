// Constraints version: 9c495569

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
use stwo_verifier_core::fields::qm31::{
    QM31, QM31Impl, QM31Serde, QM31Zero, QM31_EXTENSION_DEGREE, qm31_const,
};
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::{ArrayImpl, pow2};
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray};
use crate::components::blake_g::BLAKE_G_RELATION_SIZE;
use crate::components::blake_round_sigma::BLAKE_ROUND_SIGMA_RELATION_SIZE;
use crate::components::memory_address_to_id::MEMORY_ADDRESS_TO_ID_RELATION_SIZE;
use crate::components::memory_id_to_big::MEMORY_ID_TO_BIG_RELATION_SIZE;
use crate::components::range_check_7_2_5::RANGE_CHECK_7_2_5_RELATION_SIZE;
use crate::components::subroutines::read_blake_word::read_blake_word_evaluate;
use crate::components::{CairoComponent, OPCODES_RELATION_SIZE};
use crate::utils::U32Impl;


pub const N_TRACE_COLUMNS: usize = 212;
pub const BLAKE_ROUND_RELATION_SIZE: usize = 35;


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
        let interaction_log_sizes = ArrayImpl::new_repeated(QM31_EXTENSION_DEGREE * 30, log_size)
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
    pub blake_round_sigma_lookup_elements: crate::BlakeRoundSigmaElements,
    pub range_check_7_2_5_lookup_elements: crate::RangeCheck_7_2_5Elements,
    pub memory_address_to_id_lookup_elements: crate::MemoryAddressToIdElements,
    pub memory_id_to_big_lookup_elements: crate::MemoryIdToBigElements,
    pub blake_g_lookup_elements: crate::BlakeGElements,
    pub blake_round_lookup_elements: crate::BlakeRoundElements,
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
        let mut blake_round_sigma_sum_0: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_1: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_2: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_3: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_4: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_5: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_6: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_7: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_8: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_9: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_10: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_11: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_12: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_13: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_14: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_15: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_16: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_17: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_18: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_19: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_20: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_21: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_22: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_23: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_24: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_25: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_26: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_27: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_28: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_29: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_30: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_31: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_32: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_33: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_34: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_35: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_36: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_37: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_38: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_39: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_40: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_41: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_42: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_43: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_44: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_45: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_46: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_47: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_48: QM31 = Zero::zero();
        let mut blake_g_sum_49: QM31 = Zero::zero();
        let mut blake_g_sum_50: QM31 = Zero::zero();
        let mut blake_g_sum_51: QM31 = Zero::zero();
        let mut blake_g_sum_52: QM31 = Zero::zero();
        let mut blake_g_sum_53: QM31 = Zero::zero();
        let mut blake_g_sum_54: QM31 = Zero::zero();
        let mut blake_g_sum_55: QM31 = Zero::zero();
        let mut blake_g_sum_56: QM31 = Zero::zero();
        let mut blake_round_sum_57: QM31 = Zero::zero();
        let mut blake_round_sum_58: QM31 = Zero::zero();

        let [
            input_limb_0_col0,
            input_limb_1_col1,
            input_limb_2_col2,
            input_limb_3_col3,
            input_limb_4_col4,
            input_limb_5_col5,
            input_limb_6_col6,
            input_limb_7_col7,
            input_limb_8_col8,
            input_limb_9_col9,
            input_limb_10_col10,
            input_limb_11_col11,
            input_limb_12_col12,
            input_limb_13_col13,
            input_limb_14_col14,
            input_limb_15_col15,
            input_limb_16_col16,
            input_limb_17_col17,
            input_limb_18_col18,
            input_limb_19_col19,
            input_limb_20_col20,
            input_limb_21_col21,
            input_limb_22_col22,
            input_limb_23_col23,
            input_limb_24_col24,
            input_limb_25_col25,
            input_limb_26_col26,
            input_limb_27_col27,
            input_limb_28_col28,
            input_limb_29_col29,
            input_limb_30_col30,
            input_limb_31_col31,
            input_limb_32_col32,
            input_limb_33_col33,
            input_limb_34_col34,
            blake_round_sigma_output_limb_0_col35,
            blake_round_sigma_output_limb_1_col36,
            blake_round_sigma_output_limb_2_col37,
            blake_round_sigma_output_limb_3_col38,
            blake_round_sigma_output_limb_4_col39,
            blake_round_sigma_output_limb_5_col40,
            blake_round_sigma_output_limb_6_col41,
            blake_round_sigma_output_limb_7_col42,
            blake_round_sigma_output_limb_8_col43,
            blake_round_sigma_output_limb_9_col44,
            blake_round_sigma_output_limb_10_col45,
            blake_round_sigma_output_limb_11_col46,
            blake_round_sigma_output_limb_12_col47,
            blake_round_sigma_output_limb_13_col48,
            blake_round_sigma_output_limb_14_col49,
            blake_round_sigma_output_limb_15_col50,
            low_16_bits_col51,
            high_16_bits_col52,
            low_7_ms_bits_col53,
            high_14_ms_bits_col54,
            high_5_ms_bits_col55,
            message_word_0_id_col56,
            low_16_bits_col57,
            high_16_bits_col58,
            low_7_ms_bits_col59,
            high_14_ms_bits_col60,
            high_5_ms_bits_col61,
            message_word_1_id_col62,
            low_16_bits_col63,
            high_16_bits_col64,
            low_7_ms_bits_col65,
            high_14_ms_bits_col66,
            high_5_ms_bits_col67,
            message_word_2_id_col68,
            low_16_bits_col69,
            high_16_bits_col70,
            low_7_ms_bits_col71,
            high_14_ms_bits_col72,
            high_5_ms_bits_col73,
            message_word_3_id_col74,
            low_16_bits_col75,
            high_16_bits_col76,
            low_7_ms_bits_col77,
            high_14_ms_bits_col78,
            high_5_ms_bits_col79,
            message_word_4_id_col80,
            low_16_bits_col81,
            high_16_bits_col82,
            low_7_ms_bits_col83,
            high_14_ms_bits_col84,
            high_5_ms_bits_col85,
            message_word_5_id_col86,
            low_16_bits_col87,
            high_16_bits_col88,
            low_7_ms_bits_col89,
            high_14_ms_bits_col90,
            high_5_ms_bits_col91,
            message_word_6_id_col92,
            low_16_bits_col93,
            high_16_bits_col94,
            low_7_ms_bits_col95,
            high_14_ms_bits_col96,
            high_5_ms_bits_col97,
            message_word_7_id_col98,
            low_16_bits_col99,
            high_16_bits_col100,
            low_7_ms_bits_col101,
            high_14_ms_bits_col102,
            high_5_ms_bits_col103,
            message_word_8_id_col104,
            low_16_bits_col105,
            high_16_bits_col106,
            low_7_ms_bits_col107,
            high_14_ms_bits_col108,
            high_5_ms_bits_col109,
            message_word_9_id_col110,
            low_16_bits_col111,
            high_16_bits_col112,
            low_7_ms_bits_col113,
            high_14_ms_bits_col114,
            high_5_ms_bits_col115,
            message_word_10_id_col116,
            low_16_bits_col117,
            high_16_bits_col118,
            low_7_ms_bits_col119,
            high_14_ms_bits_col120,
            high_5_ms_bits_col121,
            message_word_11_id_col122,
            low_16_bits_col123,
            high_16_bits_col124,
            low_7_ms_bits_col125,
            high_14_ms_bits_col126,
            high_5_ms_bits_col127,
            message_word_12_id_col128,
            low_16_bits_col129,
            high_16_bits_col130,
            low_7_ms_bits_col131,
            high_14_ms_bits_col132,
            high_5_ms_bits_col133,
            message_word_13_id_col134,
            low_16_bits_col135,
            high_16_bits_col136,
            low_7_ms_bits_col137,
            high_14_ms_bits_col138,
            high_5_ms_bits_col139,
            message_word_14_id_col140,
            low_16_bits_col141,
            high_16_bits_col142,
            low_7_ms_bits_col143,
            high_14_ms_bits_col144,
            high_5_ms_bits_col145,
            message_word_15_id_col146,
            blake_g_output_limb_0_col147,
            blake_g_output_limb_1_col148,
            blake_g_output_limb_2_col149,
            blake_g_output_limb_3_col150,
            blake_g_output_limb_4_col151,
            blake_g_output_limb_5_col152,
            blake_g_output_limb_6_col153,
            blake_g_output_limb_7_col154,
            blake_g_output_limb_0_col155,
            blake_g_output_limb_1_col156,
            blake_g_output_limb_2_col157,
            blake_g_output_limb_3_col158,
            blake_g_output_limb_4_col159,
            blake_g_output_limb_5_col160,
            blake_g_output_limb_6_col161,
            blake_g_output_limb_7_col162,
            blake_g_output_limb_0_col163,
            blake_g_output_limb_1_col164,
            blake_g_output_limb_2_col165,
            blake_g_output_limb_3_col166,
            blake_g_output_limb_4_col167,
            blake_g_output_limb_5_col168,
            blake_g_output_limb_6_col169,
            blake_g_output_limb_7_col170,
            blake_g_output_limb_0_col171,
            blake_g_output_limb_1_col172,
            blake_g_output_limb_2_col173,
            blake_g_output_limb_3_col174,
            blake_g_output_limb_4_col175,
            blake_g_output_limb_5_col176,
            blake_g_output_limb_6_col177,
            blake_g_output_limb_7_col178,
            blake_g_output_limb_0_col179,
            blake_g_output_limb_1_col180,
            blake_g_output_limb_2_col181,
            blake_g_output_limb_3_col182,
            blake_g_output_limb_4_col183,
            blake_g_output_limb_5_col184,
            blake_g_output_limb_6_col185,
            blake_g_output_limb_7_col186,
            blake_g_output_limb_0_col187,
            blake_g_output_limb_1_col188,
            blake_g_output_limb_2_col189,
            blake_g_output_limb_3_col190,
            blake_g_output_limb_4_col191,
            blake_g_output_limb_5_col192,
            blake_g_output_limb_6_col193,
            blake_g_output_limb_7_col194,
            blake_g_output_limb_0_col195,
            blake_g_output_limb_1_col196,
            blake_g_output_limb_2_col197,
            blake_g_output_limb_3_col198,
            blake_g_output_limb_4_col199,
            blake_g_output_limb_5_col200,
            blake_g_output_limb_6_col201,
            blake_g_output_limb_7_col202,
            blake_g_output_limb_0_col203,
            blake_g_output_limb_1_col204,
            blake_g_output_limb_2_col205,
            blake_g_output_limb_3_col206,
            blake_g_output_limb_4_col207,
            blake_g_output_limb_5_col208,
            blake_g_output_limb_6_col209,
            blake_g_output_limb_7_col210,
            enabler,
        ]: [Span<QM31>; 212] =
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
        let [input_limb_6_col6]: [QM31; 1] = (*input_limb_6_col6.try_into().unwrap()).unbox();
        let [input_limb_7_col7]: [QM31; 1] = (*input_limb_7_col7.try_into().unwrap()).unbox();
        let [input_limb_8_col8]: [QM31; 1] = (*input_limb_8_col8.try_into().unwrap()).unbox();
        let [input_limb_9_col9]: [QM31; 1] = (*input_limb_9_col9.try_into().unwrap()).unbox();
        let [input_limb_10_col10]: [QM31; 1] = (*input_limb_10_col10.try_into().unwrap()).unbox();
        let [input_limb_11_col11]: [QM31; 1] = (*input_limb_11_col11.try_into().unwrap()).unbox();
        let [input_limb_12_col12]: [QM31; 1] = (*input_limb_12_col12.try_into().unwrap()).unbox();
        let [input_limb_13_col13]: [QM31; 1] = (*input_limb_13_col13.try_into().unwrap()).unbox();
        let [input_limb_14_col14]: [QM31; 1] = (*input_limb_14_col14.try_into().unwrap()).unbox();
        let [input_limb_15_col15]: [QM31; 1] = (*input_limb_15_col15.try_into().unwrap()).unbox();
        let [input_limb_16_col16]: [QM31; 1] = (*input_limb_16_col16.try_into().unwrap()).unbox();
        let [input_limb_17_col17]: [QM31; 1] = (*input_limb_17_col17.try_into().unwrap()).unbox();
        let [input_limb_18_col18]: [QM31; 1] = (*input_limb_18_col18.try_into().unwrap()).unbox();
        let [input_limb_19_col19]: [QM31; 1] = (*input_limb_19_col19.try_into().unwrap()).unbox();
        let [input_limb_20_col20]: [QM31; 1] = (*input_limb_20_col20.try_into().unwrap()).unbox();
        let [input_limb_21_col21]: [QM31; 1] = (*input_limb_21_col21.try_into().unwrap()).unbox();
        let [input_limb_22_col22]: [QM31; 1] = (*input_limb_22_col22.try_into().unwrap()).unbox();
        let [input_limb_23_col23]: [QM31; 1] = (*input_limb_23_col23.try_into().unwrap()).unbox();
        let [input_limb_24_col24]: [QM31; 1] = (*input_limb_24_col24.try_into().unwrap()).unbox();
        let [input_limb_25_col25]: [QM31; 1] = (*input_limb_25_col25.try_into().unwrap()).unbox();
        let [input_limb_26_col26]: [QM31; 1] = (*input_limb_26_col26.try_into().unwrap()).unbox();
        let [input_limb_27_col27]: [QM31; 1] = (*input_limb_27_col27.try_into().unwrap()).unbox();
        let [input_limb_28_col28]: [QM31; 1] = (*input_limb_28_col28.try_into().unwrap()).unbox();
        let [input_limb_29_col29]: [QM31; 1] = (*input_limb_29_col29.try_into().unwrap()).unbox();
        let [input_limb_30_col30]: [QM31; 1] = (*input_limb_30_col30.try_into().unwrap()).unbox();
        let [input_limb_31_col31]: [QM31; 1] = (*input_limb_31_col31.try_into().unwrap()).unbox();
        let [input_limb_32_col32]: [QM31; 1] = (*input_limb_32_col32.try_into().unwrap()).unbox();
        let [input_limb_33_col33]: [QM31; 1] = (*input_limb_33_col33.try_into().unwrap()).unbox();
        let [input_limb_34_col34]: [QM31; 1] = (*input_limb_34_col34.try_into().unwrap()).unbox();
        let [blake_round_sigma_output_limb_0_col35]: [QM31; 1] =
            (*blake_round_sigma_output_limb_0_col35
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_sigma_output_limb_1_col36]: [QM31; 1] =
            (*blake_round_sigma_output_limb_1_col36
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_sigma_output_limb_2_col37]: [QM31; 1] =
            (*blake_round_sigma_output_limb_2_col37
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_sigma_output_limb_3_col38]: [QM31; 1] =
            (*blake_round_sigma_output_limb_3_col38
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_sigma_output_limb_4_col39]: [QM31; 1] =
            (*blake_round_sigma_output_limb_4_col39
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_sigma_output_limb_5_col40]: [QM31; 1] =
            (*blake_round_sigma_output_limb_5_col40
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_sigma_output_limb_6_col41]: [QM31; 1] =
            (*blake_round_sigma_output_limb_6_col41
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_sigma_output_limb_7_col42]: [QM31; 1] =
            (*blake_round_sigma_output_limb_7_col42
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_sigma_output_limb_8_col43]: [QM31; 1] =
            (*blake_round_sigma_output_limb_8_col43
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_sigma_output_limb_9_col44]: [QM31; 1] =
            (*blake_round_sigma_output_limb_9_col44
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_sigma_output_limb_10_col45]: [QM31; 1] =
            (*blake_round_sigma_output_limb_10_col45
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_sigma_output_limb_11_col46]: [QM31; 1] =
            (*blake_round_sigma_output_limb_11_col46
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_sigma_output_limb_12_col47]: [QM31; 1] =
            (*blake_round_sigma_output_limb_12_col47
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_sigma_output_limb_13_col48]: [QM31; 1] =
            (*blake_round_sigma_output_limb_13_col48
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_sigma_output_limb_14_col49]: [QM31; 1] =
            (*blake_round_sigma_output_limb_14_col49
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_sigma_output_limb_15_col50]: [QM31; 1] =
            (*blake_round_sigma_output_limb_15_col50
            .try_into()
            .unwrap())
            .unbox();
        let [low_16_bits_col51]: [QM31; 1] = (*low_16_bits_col51.try_into().unwrap()).unbox();
        let [high_16_bits_col52]: [QM31; 1] = (*high_16_bits_col52.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col53]: [QM31; 1] = (*low_7_ms_bits_col53.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col54]: [QM31; 1] = (*high_14_ms_bits_col54.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col55]: [QM31; 1] = (*high_5_ms_bits_col55.try_into().unwrap()).unbox();
        let [message_word_0_id_col56]: [QM31; 1] = (*message_word_0_id_col56.try_into().unwrap())
            .unbox();
        let [low_16_bits_col57]: [QM31; 1] = (*low_16_bits_col57.try_into().unwrap()).unbox();
        let [high_16_bits_col58]: [QM31; 1] = (*high_16_bits_col58.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col59]: [QM31; 1] = (*low_7_ms_bits_col59.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col60]: [QM31; 1] = (*high_14_ms_bits_col60.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col61]: [QM31; 1] = (*high_5_ms_bits_col61.try_into().unwrap()).unbox();
        let [message_word_1_id_col62]: [QM31; 1] = (*message_word_1_id_col62.try_into().unwrap())
            .unbox();
        let [low_16_bits_col63]: [QM31; 1] = (*low_16_bits_col63.try_into().unwrap()).unbox();
        let [high_16_bits_col64]: [QM31; 1] = (*high_16_bits_col64.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col65]: [QM31; 1] = (*low_7_ms_bits_col65.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col66]: [QM31; 1] = (*high_14_ms_bits_col66.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col67]: [QM31; 1] = (*high_5_ms_bits_col67.try_into().unwrap()).unbox();
        let [message_word_2_id_col68]: [QM31; 1] = (*message_word_2_id_col68.try_into().unwrap())
            .unbox();
        let [low_16_bits_col69]: [QM31; 1] = (*low_16_bits_col69.try_into().unwrap()).unbox();
        let [high_16_bits_col70]: [QM31; 1] = (*high_16_bits_col70.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col71]: [QM31; 1] = (*low_7_ms_bits_col71.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col72]: [QM31; 1] = (*high_14_ms_bits_col72.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col73]: [QM31; 1] = (*high_5_ms_bits_col73.try_into().unwrap()).unbox();
        let [message_word_3_id_col74]: [QM31; 1] = (*message_word_3_id_col74.try_into().unwrap())
            .unbox();
        let [low_16_bits_col75]: [QM31; 1] = (*low_16_bits_col75.try_into().unwrap()).unbox();
        let [high_16_bits_col76]: [QM31; 1] = (*high_16_bits_col76.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col77]: [QM31; 1] = (*low_7_ms_bits_col77.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col78]: [QM31; 1] = (*high_14_ms_bits_col78.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col79]: [QM31; 1] = (*high_5_ms_bits_col79.try_into().unwrap()).unbox();
        let [message_word_4_id_col80]: [QM31; 1] = (*message_word_4_id_col80.try_into().unwrap())
            .unbox();
        let [low_16_bits_col81]: [QM31; 1] = (*low_16_bits_col81.try_into().unwrap()).unbox();
        let [high_16_bits_col82]: [QM31; 1] = (*high_16_bits_col82.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col83]: [QM31; 1] = (*low_7_ms_bits_col83.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col84]: [QM31; 1] = (*high_14_ms_bits_col84.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col85]: [QM31; 1] = (*high_5_ms_bits_col85.try_into().unwrap()).unbox();
        let [message_word_5_id_col86]: [QM31; 1] = (*message_word_5_id_col86.try_into().unwrap())
            .unbox();
        let [low_16_bits_col87]: [QM31; 1] = (*low_16_bits_col87.try_into().unwrap()).unbox();
        let [high_16_bits_col88]: [QM31; 1] = (*high_16_bits_col88.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col89]: [QM31; 1] = (*low_7_ms_bits_col89.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col90]: [QM31; 1] = (*high_14_ms_bits_col90.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col91]: [QM31; 1] = (*high_5_ms_bits_col91.try_into().unwrap()).unbox();
        let [message_word_6_id_col92]: [QM31; 1] = (*message_word_6_id_col92.try_into().unwrap())
            .unbox();
        let [low_16_bits_col93]: [QM31; 1] = (*low_16_bits_col93.try_into().unwrap()).unbox();
        let [high_16_bits_col94]: [QM31; 1] = (*high_16_bits_col94.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col95]: [QM31; 1] = (*low_7_ms_bits_col95.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col96]: [QM31; 1] = (*high_14_ms_bits_col96.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col97]: [QM31; 1] = (*high_5_ms_bits_col97.try_into().unwrap()).unbox();
        let [message_word_7_id_col98]: [QM31; 1] = (*message_word_7_id_col98.try_into().unwrap())
            .unbox();
        let [low_16_bits_col99]: [QM31; 1] = (*low_16_bits_col99.try_into().unwrap()).unbox();
        let [high_16_bits_col100]: [QM31; 1] = (*high_16_bits_col100.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col101]: [QM31; 1] = (*low_7_ms_bits_col101.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col102]: [QM31; 1] = (*high_14_ms_bits_col102.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col103]: [QM31; 1] = (*high_5_ms_bits_col103.try_into().unwrap())
            .unbox();
        let [message_word_8_id_col104]: [QM31; 1] = (*message_word_8_id_col104.try_into().unwrap())
            .unbox();
        let [low_16_bits_col105]: [QM31; 1] = (*low_16_bits_col105.try_into().unwrap()).unbox();
        let [high_16_bits_col106]: [QM31; 1] = (*high_16_bits_col106.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col107]: [QM31; 1] = (*low_7_ms_bits_col107.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col108]: [QM31; 1] = (*high_14_ms_bits_col108.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col109]: [QM31; 1] = (*high_5_ms_bits_col109.try_into().unwrap())
            .unbox();
        let [message_word_9_id_col110]: [QM31; 1] = (*message_word_9_id_col110.try_into().unwrap())
            .unbox();
        let [low_16_bits_col111]: [QM31; 1] = (*low_16_bits_col111.try_into().unwrap()).unbox();
        let [high_16_bits_col112]: [QM31; 1] = (*high_16_bits_col112.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col113]: [QM31; 1] = (*low_7_ms_bits_col113.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col114]: [QM31; 1] = (*high_14_ms_bits_col114.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col115]: [QM31; 1] = (*high_5_ms_bits_col115.try_into().unwrap())
            .unbox();
        let [message_word_10_id_col116]: [QM31; 1] = (*message_word_10_id_col116
            .try_into()
            .unwrap())
            .unbox();
        let [low_16_bits_col117]: [QM31; 1] = (*low_16_bits_col117.try_into().unwrap()).unbox();
        let [high_16_bits_col118]: [QM31; 1] = (*high_16_bits_col118.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col119]: [QM31; 1] = (*low_7_ms_bits_col119.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col120]: [QM31; 1] = (*high_14_ms_bits_col120.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col121]: [QM31; 1] = (*high_5_ms_bits_col121.try_into().unwrap())
            .unbox();
        let [message_word_11_id_col122]: [QM31; 1] = (*message_word_11_id_col122
            .try_into()
            .unwrap())
            .unbox();
        let [low_16_bits_col123]: [QM31; 1] = (*low_16_bits_col123.try_into().unwrap()).unbox();
        let [high_16_bits_col124]: [QM31; 1] = (*high_16_bits_col124.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col125]: [QM31; 1] = (*low_7_ms_bits_col125.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col126]: [QM31; 1] = (*high_14_ms_bits_col126.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col127]: [QM31; 1] = (*high_5_ms_bits_col127.try_into().unwrap())
            .unbox();
        let [message_word_12_id_col128]: [QM31; 1] = (*message_word_12_id_col128
            .try_into()
            .unwrap())
            .unbox();
        let [low_16_bits_col129]: [QM31; 1] = (*low_16_bits_col129.try_into().unwrap()).unbox();
        let [high_16_bits_col130]: [QM31; 1] = (*high_16_bits_col130.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col131]: [QM31; 1] = (*low_7_ms_bits_col131.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col132]: [QM31; 1] = (*high_14_ms_bits_col132.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col133]: [QM31; 1] = (*high_5_ms_bits_col133.try_into().unwrap())
            .unbox();
        let [message_word_13_id_col134]: [QM31; 1] = (*message_word_13_id_col134
            .try_into()
            .unwrap())
            .unbox();
        let [low_16_bits_col135]: [QM31; 1] = (*low_16_bits_col135.try_into().unwrap()).unbox();
        let [high_16_bits_col136]: [QM31; 1] = (*high_16_bits_col136.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col137]: [QM31; 1] = (*low_7_ms_bits_col137.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col138]: [QM31; 1] = (*high_14_ms_bits_col138.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col139]: [QM31; 1] = (*high_5_ms_bits_col139.try_into().unwrap())
            .unbox();
        let [message_word_14_id_col140]: [QM31; 1] = (*message_word_14_id_col140
            .try_into()
            .unwrap())
            .unbox();
        let [low_16_bits_col141]: [QM31; 1] = (*low_16_bits_col141.try_into().unwrap()).unbox();
        let [high_16_bits_col142]: [QM31; 1] = (*high_16_bits_col142.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col143]: [QM31; 1] = (*low_7_ms_bits_col143.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col144]: [QM31; 1] = (*high_14_ms_bits_col144.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col145]: [QM31; 1] = (*high_5_ms_bits_col145.try_into().unwrap())
            .unbox();
        let [message_word_15_id_col146]: [QM31; 1] = (*message_word_15_id_col146
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_0_col147]: [QM31; 1] = (*blake_g_output_limb_0_col147
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_1_col148]: [QM31; 1] = (*blake_g_output_limb_1_col148
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_2_col149]: [QM31; 1] = (*blake_g_output_limb_2_col149
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_3_col150]: [QM31; 1] = (*blake_g_output_limb_3_col150
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_4_col151]: [QM31; 1] = (*blake_g_output_limb_4_col151
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_5_col152]: [QM31; 1] = (*blake_g_output_limb_5_col152
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_6_col153]: [QM31; 1] = (*blake_g_output_limb_6_col153
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_7_col154]: [QM31; 1] = (*blake_g_output_limb_7_col154
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_0_col155]: [QM31; 1] = (*blake_g_output_limb_0_col155
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_1_col156]: [QM31; 1] = (*blake_g_output_limb_1_col156
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_2_col157]: [QM31; 1] = (*blake_g_output_limb_2_col157
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_3_col158]: [QM31; 1] = (*blake_g_output_limb_3_col158
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_4_col159]: [QM31; 1] = (*blake_g_output_limb_4_col159
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_5_col160]: [QM31; 1] = (*blake_g_output_limb_5_col160
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_6_col161]: [QM31; 1] = (*blake_g_output_limb_6_col161
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_7_col162]: [QM31; 1] = (*blake_g_output_limb_7_col162
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_0_col163]: [QM31; 1] = (*blake_g_output_limb_0_col163
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_1_col164]: [QM31; 1] = (*blake_g_output_limb_1_col164
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_2_col165]: [QM31; 1] = (*blake_g_output_limb_2_col165
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_3_col166]: [QM31; 1] = (*blake_g_output_limb_3_col166
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_4_col167]: [QM31; 1] = (*blake_g_output_limb_4_col167
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_5_col168]: [QM31; 1] = (*blake_g_output_limb_5_col168
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_6_col169]: [QM31; 1] = (*blake_g_output_limb_6_col169
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_7_col170]: [QM31; 1] = (*blake_g_output_limb_7_col170
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_0_col171]: [QM31; 1] = (*blake_g_output_limb_0_col171
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_1_col172]: [QM31; 1] = (*blake_g_output_limb_1_col172
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_2_col173]: [QM31; 1] = (*blake_g_output_limb_2_col173
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_3_col174]: [QM31; 1] = (*blake_g_output_limb_3_col174
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_4_col175]: [QM31; 1] = (*blake_g_output_limb_4_col175
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_5_col176]: [QM31; 1] = (*blake_g_output_limb_5_col176
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_6_col177]: [QM31; 1] = (*blake_g_output_limb_6_col177
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_7_col178]: [QM31; 1] = (*blake_g_output_limb_7_col178
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_0_col179]: [QM31; 1] = (*blake_g_output_limb_0_col179
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_1_col180]: [QM31; 1] = (*blake_g_output_limb_1_col180
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_2_col181]: [QM31; 1] = (*blake_g_output_limb_2_col181
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_3_col182]: [QM31; 1] = (*blake_g_output_limb_3_col182
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_4_col183]: [QM31; 1] = (*blake_g_output_limb_4_col183
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_5_col184]: [QM31; 1] = (*blake_g_output_limb_5_col184
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_6_col185]: [QM31; 1] = (*blake_g_output_limb_6_col185
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_7_col186]: [QM31; 1] = (*blake_g_output_limb_7_col186
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_0_col187]: [QM31; 1] = (*blake_g_output_limb_0_col187
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_1_col188]: [QM31; 1] = (*blake_g_output_limb_1_col188
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_2_col189]: [QM31; 1] = (*blake_g_output_limb_2_col189
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_3_col190]: [QM31; 1] = (*blake_g_output_limb_3_col190
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_4_col191]: [QM31; 1] = (*blake_g_output_limb_4_col191
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_5_col192]: [QM31; 1] = (*blake_g_output_limb_5_col192
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_6_col193]: [QM31; 1] = (*blake_g_output_limb_6_col193
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_7_col194]: [QM31; 1] = (*blake_g_output_limb_7_col194
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_0_col195]: [QM31; 1] = (*blake_g_output_limb_0_col195
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_1_col196]: [QM31; 1] = (*blake_g_output_limb_1_col196
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_2_col197]: [QM31; 1] = (*blake_g_output_limb_2_col197
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_3_col198]: [QM31; 1] = (*blake_g_output_limb_3_col198
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_4_col199]: [QM31; 1] = (*blake_g_output_limb_4_col199
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_5_col200]: [QM31; 1] = (*blake_g_output_limb_5_col200
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_6_col201]: [QM31; 1] = (*blake_g_output_limb_6_col201
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_7_col202]: [QM31; 1] = (*blake_g_output_limb_7_col202
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_0_col203]: [QM31; 1] = (*blake_g_output_limb_0_col203
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_1_col204]: [QM31; 1] = (*blake_g_output_limb_1_col204
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_2_col205]: [QM31; 1] = (*blake_g_output_limb_2_col205
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_3_col206]: [QM31; 1] = (*blake_g_output_limb_3_col206
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_4_col207]: [QM31; 1] = (*blake_g_output_limb_4_col207
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_5_col208]: [QM31; 1] = (*blake_g_output_limb_5_col208
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_6_col209]: [QM31; 1] = (*blake_g_output_limb_6_col209
            .try_into()
            .unwrap())
            .unbox();
        let [blake_g_output_limb_7_col210]: [QM31; 1] = (*blake_g_output_limb_7_col210
            .try_into()
            .unwrap())
            .unbox();
        let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        let constraint_quotient = (enabler * enabler - enabler) * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        blake_round_sigma_sum_0 = self
            .blake_round_sigma_lookup_elements
            .combine_qm31(
                [
                    input_limb_1_col1, blake_round_sigma_output_limb_0_col35,
                    blake_round_sigma_output_limb_1_col36, blake_round_sigma_output_limb_2_col37,
                    blake_round_sigma_output_limb_3_col38, blake_round_sigma_output_limb_4_col39,
                    blake_round_sigma_output_limb_5_col40, blake_round_sigma_output_limb_6_col41,
                    blake_round_sigma_output_limb_7_col42, blake_round_sigma_output_limb_8_col43,
                    blake_round_sigma_output_limb_9_col44, blake_round_sigma_output_limb_10_col45,
                    blake_round_sigma_output_limb_11_col46, blake_round_sigma_output_limb_12_col47,
                    blake_round_sigma_output_limb_13_col48, blake_round_sigma_output_limb_14_col49,
                    blake_round_sigma_output_limb_15_col50,
                ],
            );

        read_blake_word_evaluate(
            [(input_limb_34_col34 + blake_round_sigma_output_limb_0_col35)],
            low_16_bits_col51,
            high_16_bits_col52,
            low_7_ms_bits_col53,
            high_14_ms_bits_col54,
            high_5_ms_bits_col55,
            message_word_0_id_col56,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_1,
            ref memory_address_to_id_sum_2,
            ref memory_id_to_big_sum_3,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        read_blake_word_evaluate(
            [(input_limb_34_col34 + blake_round_sigma_output_limb_1_col36)],
            low_16_bits_col57,
            high_16_bits_col58,
            low_7_ms_bits_col59,
            high_14_ms_bits_col60,
            high_5_ms_bits_col61,
            message_word_1_id_col62,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_4,
            ref memory_address_to_id_sum_5,
            ref memory_id_to_big_sum_6,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        read_blake_word_evaluate(
            [(input_limb_34_col34 + blake_round_sigma_output_limb_2_col37)],
            low_16_bits_col63,
            high_16_bits_col64,
            low_7_ms_bits_col65,
            high_14_ms_bits_col66,
            high_5_ms_bits_col67,
            message_word_2_id_col68,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_7,
            ref memory_address_to_id_sum_8,
            ref memory_id_to_big_sum_9,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        read_blake_word_evaluate(
            [(input_limb_34_col34 + blake_round_sigma_output_limb_3_col38)],
            low_16_bits_col69,
            high_16_bits_col70,
            low_7_ms_bits_col71,
            high_14_ms_bits_col72,
            high_5_ms_bits_col73,
            message_word_3_id_col74,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_10,
            ref memory_address_to_id_sum_11,
            ref memory_id_to_big_sum_12,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        read_blake_word_evaluate(
            [(input_limb_34_col34 + blake_round_sigma_output_limb_4_col39)],
            low_16_bits_col75,
            high_16_bits_col76,
            low_7_ms_bits_col77,
            high_14_ms_bits_col78,
            high_5_ms_bits_col79,
            message_word_4_id_col80,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_13,
            ref memory_address_to_id_sum_14,
            ref memory_id_to_big_sum_15,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        read_blake_word_evaluate(
            [(input_limb_34_col34 + blake_round_sigma_output_limb_5_col40)],
            low_16_bits_col81,
            high_16_bits_col82,
            low_7_ms_bits_col83,
            high_14_ms_bits_col84,
            high_5_ms_bits_col85,
            message_word_5_id_col86,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_16,
            ref memory_address_to_id_sum_17,
            ref memory_id_to_big_sum_18,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        read_blake_word_evaluate(
            [(input_limb_34_col34 + blake_round_sigma_output_limb_6_col41)],
            low_16_bits_col87,
            high_16_bits_col88,
            low_7_ms_bits_col89,
            high_14_ms_bits_col90,
            high_5_ms_bits_col91,
            message_word_6_id_col92,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_19,
            ref memory_address_to_id_sum_20,
            ref memory_id_to_big_sum_21,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        read_blake_word_evaluate(
            [(input_limb_34_col34 + blake_round_sigma_output_limb_7_col42)],
            low_16_bits_col93,
            high_16_bits_col94,
            low_7_ms_bits_col95,
            high_14_ms_bits_col96,
            high_5_ms_bits_col97,
            message_word_7_id_col98,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_22,
            ref memory_address_to_id_sum_23,
            ref memory_id_to_big_sum_24,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        read_blake_word_evaluate(
            [(input_limb_34_col34 + blake_round_sigma_output_limb_8_col43)],
            low_16_bits_col99,
            high_16_bits_col100,
            low_7_ms_bits_col101,
            high_14_ms_bits_col102,
            high_5_ms_bits_col103,
            message_word_8_id_col104,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_25,
            ref memory_address_to_id_sum_26,
            ref memory_id_to_big_sum_27,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        read_blake_word_evaluate(
            [(input_limb_34_col34 + blake_round_sigma_output_limb_9_col44)],
            low_16_bits_col105,
            high_16_bits_col106,
            low_7_ms_bits_col107,
            high_14_ms_bits_col108,
            high_5_ms_bits_col109,
            message_word_9_id_col110,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_28,
            ref memory_address_to_id_sum_29,
            ref memory_id_to_big_sum_30,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        read_blake_word_evaluate(
            [(input_limb_34_col34 + blake_round_sigma_output_limb_10_col45)],
            low_16_bits_col111,
            high_16_bits_col112,
            low_7_ms_bits_col113,
            high_14_ms_bits_col114,
            high_5_ms_bits_col115,
            message_word_10_id_col116,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_31,
            ref memory_address_to_id_sum_32,
            ref memory_id_to_big_sum_33,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        read_blake_word_evaluate(
            [(input_limb_34_col34 + blake_round_sigma_output_limb_11_col46)],
            low_16_bits_col117,
            high_16_bits_col118,
            low_7_ms_bits_col119,
            high_14_ms_bits_col120,
            high_5_ms_bits_col121,
            message_word_11_id_col122,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_34,
            ref memory_address_to_id_sum_35,
            ref memory_id_to_big_sum_36,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        read_blake_word_evaluate(
            [(input_limb_34_col34 + blake_round_sigma_output_limb_12_col47)],
            low_16_bits_col123,
            high_16_bits_col124,
            low_7_ms_bits_col125,
            high_14_ms_bits_col126,
            high_5_ms_bits_col127,
            message_word_12_id_col128,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_37,
            ref memory_address_to_id_sum_38,
            ref memory_id_to_big_sum_39,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        read_blake_word_evaluate(
            [(input_limb_34_col34 + blake_round_sigma_output_limb_13_col48)],
            low_16_bits_col129,
            high_16_bits_col130,
            low_7_ms_bits_col131,
            high_14_ms_bits_col132,
            high_5_ms_bits_col133,
            message_word_13_id_col134,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_40,
            ref memory_address_to_id_sum_41,
            ref memory_id_to_big_sum_42,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        read_blake_word_evaluate(
            [(input_limb_34_col34 + blake_round_sigma_output_limb_14_col49)],
            low_16_bits_col135,
            high_16_bits_col136,
            low_7_ms_bits_col137,
            high_14_ms_bits_col138,
            high_5_ms_bits_col139,
            message_word_14_id_col140,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_43,
            ref memory_address_to_id_sum_44,
            ref memory_id_to_big_sum_45,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        read_blake_word_evaluate(
            [(input_limb_34_col34 + blake_round_sigma_output_limb_15_col50)],
            low_16_bits_col141,
            high_16_bits_col142,
            low_7_ms_bits_col143,
            high_14_ms_bits_col144,
            high_5_ms_bits_col145,
            message_word_15_id_col146,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_46,
            ref memory_address_to_id_sum_47,
            ref memory_id_to_big_sum_48,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        blake_g_sum_49 = self
            .blake_g_lookup_elements
            .combine_qm31(
                [
                    input_limb_2_col2, input_limb_3_col3, input_limb_10_col10, input_limb_11_col11,
                    input_limb_18_col18, input_limb_19_col19, input_limb_26_col26,
                    input_limb_27_col27, low_16_bits_col51, high_16_bits_col52, low_16_bits_col57,
                    high_16_bits_col58, blake_g_output_limb_0_col147, blake_g_output_limb_1_col148,
                    blake_g_output_limb_2_col149, blake_g_output_limb_3_col150,
                    blake_g_output_limb_4_col151, blake_g_output_limb_5_col152,
                    blake_g_output_limb_6_col153, blake_g_output_limb_7_col154,
                ],
            );

        blake_g_sum_50 = self
            .blake_g_lookup_elements
            .combine_qm31(
                [
                    input_limb_4_col4, input_limb_5_col5, input_limb_12_col12, input_limb_13_col13,
                    input_limb_20_col20, input_limb_21_col21, input_limb_28_col28,
                    input_limb_29_col29, low_16_bits_col63, high_16_bits_col64, low_16_bits_col69,
                    high_16_bits_col70, blake_g_output_limb_0_col155, blake_g_output_limb_1_col156,
                    blake_g_output_limb_2_col157, blake_g_output_limb_3_col158,
                    blake_g_output_limb_4_col159, blake_g_output_limb_5_col160,
                    blake_g_output_limb_6_col161, blake_g_output_limb_7_col162,
                ],
            );

        blake_g_sum_51 = self
            .blake_g_lookup_elements
            .combine_qm31(
                [
                    input_limb_6_col6, input_limb_7_col7, input_limb_14_col14, input_limb_15_col15,
                    input_limb_22_col22, input_limb_23_col23, input_limb_30_col30,
                    input_limb_31_col31, low_16_bits_col75, high_16_bits_col76, low_16_bits_col81,
                    high_16_bits_col82, blake_g_output_limb_0_col163, blake_g_output_limb_1_col164,
                    blake_g_output_limb_2_col165, blake_g_output_limb_3_col166,
                    blake_g_output_limb_4_col167, blake_g_output_limb_5_col168,
                    blake_g_output_limb_6_col169, blake_g_output_limb_7_col170,
                ],
            );

        blake_g_sum_52 = self
            .blake_g_lookup_elements
            .combine_qm31(
                [
                    input_limb_8_col8, input_limb_9_col9, input_limb_16_col16, input_limb_17_col17,
                    input_limb_24_col24, input_limb_25_col25, input_limb_32_col32,
                    input_limb_33_col33, low_16_bits_col87, high_16_bits_col88, low_16_bits_col93,
                    high_16_bits_col94, blake_g_output_limb_0_col171, blake_g_output_limb_1_col172,
                    blake_g_output_limb_2_col173, blake_g_output_limb_3_col174,
                    blake_g_output_limb_4_col175, blake_g_output_limb_5_col176,
                    blake_g_output_limb_6_col177, blake_g_output_limb_7_col178,
                ],
            );

        blake_g_sum_53 = self
            .blake_g_lookup_elements
            .combine_qm31(
                [
                    blake_g_output_limb_0_col147, blake_g_output_limb_1_col148,
                    blake_g_output_limb_2_col157, blake_g_output_limb_3_col158,
                    blake_g_output_limb_4_col167, blake_g_output_limb_5_col168,
                    blake_g_output_limb_6_col177, blake_g_output_limb_7_col178, low_16_bits_col99,
                    high_16_bits_col100, low_16_bits_col105, high_16_bits_col106,
                    blake_g_output_limb_0_col179, blake_g_output_limb_1_col180,
                    blake_g_output_limb_2_col181, blake_g_output_limb_3_col182,
                    blake_g_output_limb_4_col183, blake_g_output_limb_5_col184,
                    blake_g_output_limb_6_col185, blake_g_output_limb_7_col186,
                ],
            );

        blake_g_sum_54 = self
            .blake_g_lookup_elements
            .combine_qm31(
                [
                    blake_g_output_limb_0_col155, blake_g_output_limb_1_col156,
                    blake_g_output_limb_2_col165, blake_g_output_limb_3_col166,
                    blake_g_output_limb_4_col175, blake_g_output_limb_5_col176,
                    blake_g_output_limb_6_col153, blake_g_output_limb_7_col154, low_16_bits_col111,
                    high_16_bits_col112, low_16_bits_col117, high_16_bits_col118,
                    blake_g_output_limb_0_col187, blake_g_output_limb_1_col188,
                    blake_g_output_limb_2_col189, blake_g_output_limb_3_col190,
                    blake_g_output_limb_4_col191, blake_g_output_limb_5_col192,
                    blake_g_output_limb_6_col193, blake_g_output_limb_7_col194,
                ],
            );

        blake_g_sum_55 = self
            .blake_g_lookup_elements
            .combine_qm31(
                [
                    blake_g_output_limb_0_col163, blake_g_output_limb_1_col164,
                    blake_g_output_limb_2_col173, blake_g_output_limb_3_col174,
                    blake_g_output_limb_4_col151, blake_g_output_limb_5_col152,
                    blake_g_output_limb_6_col161, blake_g_output_limb_7_col162, low_16_bits_col123,
                    high_16_bits_col124, low_16_bits_col129, high_16_bits_col130,
                    blake_g_output_limb_0_col195, blake_g_output_limb_1_col196,
                    blake_g_output_limb_2_col197, blake_g_output_limb_3_col198,
                    blake_g_output_limb_4_col199, blake_g_output_limb_5_col200,
                    blake_g_output_limb_6_col201, blake_g_output_limb_7_col202,
                ],
            );

        blake_g_sum_56 = self
            .blake_g_lookup_elements
            .combine_qm31(
                [
                    blake_g_output_limb_0_col171, blake_g_output_limb_1_col172,
                    blake_g_output_limb_2_col149, blake_g_output_limb_3_col150,
                    blake_g_output_limb_4_col159, blake_g_output_limb_5_col160,
                    blake_g_output_limb_6_col169, blake_g_output_limb_7_col170, low_16_bits_col135,
                    high_16_bits_col136, low_16_bits_col141, high_16_bits_col142,
                    blake_g_output_limb_0_col203, blake_g_output_limb_1_col204,
                    blake_g_output_limb_2_col205, blake_g_output_limb_3_col206,
                    blake_g_output_limb_4_col207, blake_g_output_limb_5_col208,
                    blake_g_output_limb_6_col209, blake_g_output_limb_7_col210,
                ],
            );

        blake_round_sum_57 = self
            .blake_round_lookup_elements
            .combine_qm31(
                [
                    input_limb_0_col0, input_limb_1_col1, input_limb_2_col2, input_limb_3_col3,
                    input_limb_4_col4, input_limb_5_col5, input_limb_6_col6, input_limb_7_col7,
                    input_limb_8_col8, input_limb_9_col9, input_limb_10_col10, input_limb_11_col11,
                    input_limb_12_col12, input_limb_13_col13, input_limb_14_col14,
                    input_limb_15_col15, input_limb_16_col16, input_limb_17_col17,
                    input_limb_18_col18, input_limb_19_col19, input_limb_20_col20,
                    input_limb_21_col21, input_limb_22_col22, input_limb_23_col23,
                    input_limb_24_col24, input_limb_25_col25, input_limb_26_col26,
                    input_limb_27_col27, input_limb_28_col28, input_limb_29_col29,
                    input_limb_30_col30, input_limb_31_col31, input_limb_32_col32,
                    input_limb_33_col33, input_limb_34_col34,
                ],
            );

        blake_round_sum_58 = self
            .blake_round_lookup_elements
            .combine_qm31(
                [
                    input_limb_0_col0, (input_limb_1_col1 + qm31_const::<1, 0, 0, 0>()),
                    blake_g_output_limb_0_col179, blake_g_output_limb_1_col180,
                    blake_g_output_limb_0_col187, blake_g_output_limb_1_col188,
                    blake_g_output_limb_0_col195, blake_g_output_limb_1_col196,
                    blake_g_output_limb_0_col203, blake_g_output_limb_1_col204,
                    blake_g_output_limb_2_col205, blake_g_output_limb_3_col206,
                    blake_g_output_limb_2_col181, blake_g_output_limb_3_col182,
                    blake_g_output_limb_2_col189, blake_g_output_limb_3_col190,
                    blake_g_output_limb_2_col197, blake_g_output_limb_3_col198,
                    blake_g_output_limb_4_col199, blake_g_output_limb_5_col200,
                    blake_g_output_limb_4_col207, blake_g_output_limb_5_col208,
                    blake_g_output_limb_4_col183, blake_g_output_limb_5_col184,
                    blake_g_output_limb_4_col191, blake_g_output_limb_5_col192,
                    blake_g_output_limb_6_col193, blake_g_output_limb_7_col194,
                    blake_g_output_limb_6_col201, blake_g_output_limb_7_col202,
                    blake_g_output_limb_6_col209, blake_g_output_limb_7_col210,
                    blake_g_output_limb_6_col185, blake_g_output_limb_7_col186, input_limb_34_col34,
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
            blake_round_sigma_sum_0,
            range_check_7_2_5_sum_1,
            memory_address_to_id_sum_2,
            memory_id_to_big_sum_3,
            range_check_7_2_5_sum_4,
            memory_address_to_id_sum_5,
            memory_id_to_big_sum_6,
            range_check_7_2_5_sum_7,
            memory_address_to_id_sum_8,
            memory_id_to_big_sum_9,
            range_check_7_2_5_sum_10,
            memory_address_to_id_sum_11,
            memory_id_to_big_sum_12,
            range_check_7_2_5_sum_13,
            memory_address_to_id_sum_14,
            memory_id_to_big_sum_15,
            range_check_7_2_5_sum_16,
            memory_address_to_id_sum_17,
            memory_id_to_big_sum_18,
            range_check_7_2_5_sum_19,
            memory_address_to_id_sum_20,
            memory_id_to_big_sum_21,
            range_check_7_2_5_sum_22,
            memory_address_to_id_sum_23,
            memory_id_to_big_sum_24,
            range_check_7_2_5_sum_25,
            memory_address_to_id_sum_26,
            memory_id_to_big_sum_27,
            range_check_7_2_5_sum_28,
            memory_address_to_id_sum_29,
            memory_id_to_big_sum_30,
            range_check_7_2_5_sum_31,
            memory_address_to_id_sum_32,
            memory_id_to_big_sum_33,
            range_check_7_2_5_sum_34,
            memory_address_to_id_sum_35,
            memory_id_to_big_sum_36,
            range_check_7_2_5_sum_37,
            memory_address_to_id_sum_38,
            memory_id_to_big_sum_39,
            range_check_7_2_5_sum_40,
            memory_address_to_id_sum_41,
            memory_id_to_big_sum_42,
            range_check_7_2_5_sum_43,
            memory_address_to_id_sum_44,
            memory_id_to_big_sum_45,
            range_check_7_2_5_sum_46,
            memory_address_to_id_sum_47,
            memory_id_to_big_sum_48,
            blake_g_sum_49,
            blake_g_sum_50,
            blake_g_sum_51,
            blake_g_sum_52,
            blake_g_sum_53,
            blake_g_sum_54,
            blake_g_sum_55,
            blake_g_sum_56,
            blake_round_sum_57,
            blake_round_sum_58,
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
    blake_round_sigma_sum_0: QM31,
    range_check_7_2_5_sum_1: QM31,
    memory_address_to_id_sum_2: QM31,
    memory_id_to_big_sum_3: QM31,
    range_check_7_2_5_sum_4: QM31,
    memory_address_to_id_sum_5: QM31,
    memory_id_to_big_sum_6: QM31,
    range_check_7_2_5_sum_7: QM31,
    memory_address_to_id_sum_8: QM31,
    memory_id_to_big_sum_9: QM31,
    range_check_7_2_5_sum_10: QM31,
    memory_address_to_id_sum_11: QM31,
    memory_id_to_big_sum_12: QM31,
    range_check_7_2_5_sum_13: QM31,
    memory_address_to_id_sum_14: QM31,
    memory_id_to_big_sum_15: QM31,
    range_check_7_2_5_sum_16: QM31,
    memory_address_to_id_sum_17: QM31,
    memory_id_to_big_sum_18: QM31,
    range_check_7_2_5_sum_19: QM31,
    memory_address_to_id_sum_20: QM31,
    memory_id_to_big_sum_21: QM31,
    range_check_7_2_5_sum_22: QM31,
    memory_address_to_id_sum_23: QM31,
    memory_id_to_big_sum_24: QM31,
    range_check_7_2_5_sum_25: QM31,
    memory_address_to_id_sum_26: QM31,
    memory_id_to_big_sum_27: QM31,
    range_check_7_2_5_sum_28: QM31,
    memory_address_to_id_sum_29: QM31,
    memory_id_to_big_sum_30: QM31,
    range_check_7_2_5_sum_31: QM31,
    memory_address_to_id_sum_32: QM31,
    memory_id_to_big_sum_33: QM31,
    range_check_7_2_5_sum_34: QM31,
    memory_address_to_id_sum_35: QM31,
    memory_id_to_big_sum_36: QM31,
    range_check_7_2_5_sum_37: QM31,
    memory_address_to_id_sum_38: QM31,
    memory_id_to_big_sum_39: QM31,
    range_check_7_2_5_sum_40: QM31,
    memory_address_to_id_sum_41: QM31,
    memory_id_to_big_sum_42: QM31,
    range_check_7_2_5_sum_43: QM31,
    memory_address_to_id_sum_44: QM31,
    memory_id_to_big_sum_45: QM31,
    range_check_7_2_5_sum_46: QM31,
    memory_address_to_id_sum_47: QM31,
    memory_id_to_big_sum_48: QM31,
    blake_g_sum_49: QM31,
    blake_g_sum_50: QM31,
    blake_g_sum_51: QM31,
    blake_g_sum_52: QM31,
    blake_g_sum_53: QM31,
    blake_g_sum_54: QM31,
    blake_g_sum_55: QM31,
    blake_g_sum_56: QM31,
    blake_round_sum_57: QM31,
    blake_round_sum_58: QM31,
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
        trace_2_col20,
        trace_2_col21,
        trace_2_col22,
        trace_2_col23,
        trace_2_col24,
        trace_2_col25,
        trace_2_col26,
        trace_2_col27,
        trace_2_col28,
        trace_2_col29,
        trace_2_col30,
        trace_2_col31,
        trace_2_col32,
        trace_2_col33,
        trace_2_col34,
        trace_2_col35,
        trace_2_col36,
        trace_2_col37,
        trace_2_col38,
        trace_2_col39,
        trace_2_col40,
        trace_2_col41,
        trace_2_col42,
        trace_2_col43,
        trace_2_col44,
        trace_2_col45,
        trace_2_col46,
        trace_2_col47,
        trace_2_col48,
        trace_2_col49,
        trace_2_col50,
        trace_2_col51,
        trace_2_col52,
        trace_2_col53,
        trace_2_col54,
        trace_2_col55,
        trace_2_col56,
        trace_2_col57,
        trace_2_col58,
        trace_2_col59,
        trace_2_col60,
        trace_2_col61,
        trace_2_col62,
        trace_2_col63,
        trace_2_col64,
        trace_2_col65,
        trace_2_col66,
        trace_2_col67,
        trace_2_col68,
        trace_2_col69,
        trace_2_col70,
        trace_2_col71,
        trace_2_col72,
        trace_2_col73,
        trace_2_col74,
        trace_2_col75,
        trace_2_col76,
        trace_2_col77,
        trace_2_col78,
        trace_2_col79,
        trace_2_col80,
        trace_2_col81,
        trace_2_col82,
        trace_2_col83,
        trace_2_col84,
        trace_2_col85,
        trace_2_col86,
        trace_2_col87,
        trace_2_col88,
        trace_2_col89,
        trace_2_col90,
        trace_2_col91,
        trace_2_col92,
        trace_2_col93,
        trace_2_col94,
        trace_2_col95,
        trace_2_col96,
        trace_2_col97,
        trace_2_col98,
        trace_2_col99,
        trace_2_col100,
        trace_2_col101,
        trace_2_col102,
        trace_2_col103,
        trace_2_col104,
        trace_2_col105,
        trace_2_col106,
        trace_2_col107,
        trace_2_col108,
        trace_2_col109,
        trace_2_col110,
        trace_2_col111,
        trace_2_col112,
        trace_2_col113,
        trace_2_col114,
        trace_2_col115,
        trace_2_col116,
        trace_2_col117,
        trace_2_col118,
        trace_2_col119,
    ]: [Span<QM31>; 120] =
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
    let [trace_2_col16]: [QM31; 1] = (*trace_2_col16.try_into().unwrap()).unbox();
    let [trace_2_col17]: [QM31; 1] = (*trace_2_col17.try_into().unwrap()).unbox();
    let [trace_2_col18]: [QM31; 1] = (*trace_2_col18.try_into().unwrap()).unbox();
    let [trace_2_col19]: [QM31; 1] = (*trace_2_col19.try_into().unwrap()).unbox();
    let [trace_2_col20]: [QM31; 1] = (*trace_2_col20.try_into().unwrap()).unbox();
    let [trace_2_col21]: [QM31; 1] = (*trace_2_col21.try_into().unwrap()).unbox();
    let [trace_2_col22]: [QM31; 1] = (*trace_2_col22.try_into().unwrap()).unbox();
    let [trace_2_col23]: [QM31; 1] = (*trace_2_col23.try_into().unwrap()).unbox();
    let [trace_2_col24]: [QM31; 1] = (*trace_2_col24.try_into().unwrap()).unbox();
    let [trace_2_col25]: [QM31; 1] = (*trace_2_col25.try_into().unwrap()).unbox();
    let [trace_2_col26]: [QM31; 1] = (*trace_2_col26.try_into().unwrap()).unbox();
    let [trace_2_col27]: [QM31; 1] = (*trace_2_col27.try_into().unwrap()).unbox();
    let [trace_2_col28]: [QM31; 1] = (*trace_2_col28.try_into().unwrap()).unbox();
    let [trace_2_col29]: [QM31; 1] = (*trace_2_col29.try_into().unwrap()).unbox();
    let [trace_2_col30]: [QM31; 1] = (*trace_2_col30.try_into().unwrap()).unbox();
    let [trace_2_col31]: [QM31; 1] = (*trace_2_col31.try_into().unwrap()).unbox();
    let [trace_2_col32]: [QM31; 1] = (*trace_2_col32.try_into().unwrap()).unbox();
    let [trace_2_col33]: [QM31; 1] = (*trace_2_col33.try_into().unwrap()).unbox();
    let [trace_2_col34]: [QM31; 1] = (*trace_2_col34.try_into().unwrap()).unbox();
    let [trace_2_col35]: [QM31; 1] = (*trace_2_col35.try_into().unwrap()).unbox();
    let [trace_2_col36]: [QM31; 1] = (*trace_2_col36.try_into().unwrap()).unbox();
    let [trace_2_col37]: [QM31; 1] = (*trace_2_col37.try_into().unwrap()).unbox();
    let [trace_2_col38]: [QM31; 1] = (*trace_2_col38.try_into().unwrap()).unbox();
    let [trace_2_col39]: [QM31; 1] = (*trace_2_col39.try_into().unwrap()).unbox();
    let [trace_2_col40]: [QM31; 1] = (*trace_2_col40.try_into().unwrap()).unbox();
    let [trace_2_col41]: [QM31; 1] = (*trace_2_col41.try_into().unwrap()).unbox();
    let [trace_2_col42]: [QM31; 1] = (*trace_2_col42.try_into().unwrap()).unbox();
    let [trace_2_col43]: [QM31; 1] = (*trace_2_col43.try_into().unwrap()).unbox();
    let [trace_2_col44]: [QM31; 1] = (*trace_2_col44.try_into().unwrap()).unbox();
    let [trace_2_col45]: [QM31; 1] = (*trace_2_col45.try_into().unwrap()).unbox();
    let [trace_2_col46]: [QM31; 1] = (*trace_2_col46.try_into().unwrap()).unbox();
    let [trace_2_col47]: [QM31; 1] = (*trace_2_col47.try_into().unwrap()).unbox();
    let [trace_2_col48]: [QM31; 1] = (*trace_2_col48.try_into().unwrap()).unbox();
    let [trace_2_col49]: [QM31; 1] = (*trace_2_col49.try_into().unwrap()).unbox();
    let [trace_2_col50]: [QM31; 1] = (*trace_2_col50.try_into().unwrap()).unbox();
    let [trace_2_col51]: [QM31; 1] = (*trace_2_col51.try_into().unwrap()).unbox();
    let [trace_2_col52]: [QM31; 1] = (*trace_2_col52.try_into().unwrap()).unbox();
    let [trace_2_col53]: [QM31; 1] = (*trace_2_col53.try_into().unwrap()).unbox();
    let [trace_2_col54]: [QM31; 1] = (*trace_2_col54.try_into().unwrap()).unbox();
    let [trace_2_col55]: [QM31; 1] = (*trace_2_col55.try_into().unwrap()).unbox();
    let [trace_2_col56]: [QM31; 1] = (*trace_2_col56.try_into().unwrap()).unbox();
    let [trace_2_col57]: [QM31; 1] = (*trace_2_col57.try_into().unwrap()).unbox();
    let [trace_2_col58]: [QM31; 1] = (*trace_2_col58.try_into().unwrap()).unbox();
    let [trace_2_col59]: [QM31; 1] = (*trace_2_col59.try_into().unwrap()).unbox();
    let [trace_2_col60]: [QM31; 1] = (*trace_2_col60.try_into().unwrap()).unbox();
    let [trace_2_col61]: [QM31; 1] = (*trace_2_col61.try_into().unwrap()).unbox();
    let [trace_2_col62]: [QM31; 1] = (*trace_2_col62.try_into().unwrap()).unbox();
    let [trace_2_col63]: [QM31; 1] = (*trace_2_col63.try_into().unwrap()).unbox();
    let [trace_2_col64]: [QM31; 1] = (*trace_2_col64.try_into().unwrap()).unbox();
    let [trace_2_col65]: [QM31; 1] = (*trace_2_col65.try_into().unwrap()).unbox();
    let [trace_2_col66]: [QM31; 1] = (*trace_2_col66.try_into().unwrap()).unbox();
    let [trace_2_col67]: [QM31; 1] = (*trace_2_col67.try_into().unwrap()).unbox();
    let [trace_2_col68]: [QM31; 1] = (*trace_2_col68.try_into().unwrap()).unbox();
    let [trace_2_col69]: [QM31; 1] = (*trace_2_col69.try_into().unwrap()).unbox();
    let [trace_2_col70]: [QM31; 1] = (*trace_2_col70.try_into().unwrap()).unbox();
    let [trace_2_col71]: [QM31; 1] = (*trace_2_col71.try_into().unwrap()).unbox();
    let [trace_2_col72]: [QM31; 1] = (*trace_2_col72.try_into().unwrap()).unbox();
    let [trace_2_col73]: [QM31; 1] = (*trace_2_col73.try_into().unwrap()).unbox();
    let [trace_2_col74]: [QM31; 1] = (*trace_2_col74.try_into().unwrap()).unbox();
    let [trace_2_col75]: [QM31; 1] = (*trace_2_col75.try_into().unwrap()).unbox();
    let [trace_2_col76]: [QM31; 1] = (*trace_2_col76.try_into().unwrap()).unbox();
    let [trace_2_col77]: [QM31; 1] = (*trace_2_col77.try_into().unwrap()).unbox();
    let [trace_2_col78]: [QM31; 1] = (*trace_2_col78.try_into().unwrap()).unbox();
    let [trace_2_col79]: [QM31; 1] = (*trace_2_col79.try_into().unwrap()).unbox();
    let [trace_2_col80]: [QM31; 1] = (*trace_2_col80.try_into().unwrap()).unbox();
    let [trace_2_col81]: [QM31; 1] = (*trace_2_col81.try_into().unwrap()).unbox();
    let [trace_2_col82]: [QM31; 1] = (*trace_2_col82.try_into().unwrap()).unbox();
    let [trace_2_col83]: [QM31; 1] = (*trace_2_col83.try_into().unwrap()).unbox();
    let [trace_2_col84]: [QM31; 1] = (*trace_2_col84.try_into().unwrap()).unbox();
    let [trace_2_col85]: [QM31; 1] = (*trace_2_col85.try_into().unwrap()).unbox();
    let [trace_2_col86]: [QM31; 1] = (*trace_2_col86.try_into().unwrap()).unbox();
    let [trace_2_col87]: [QM31; 1] = (*trace_2_col87.try_into().unwrap()).unbox();
    let [trace_2_col88]: [QM31; 1] = (*trace_2_col88.try_into().unwrap()).unbox();
    let [trace_2_col89]: [QM31; 1] = (*trace_2_col89.try_into().unwrap()).unbox();
    let [trace_2_col90]: [QM31; 1] = (*trace_2_col90.try_into().unwrap()).unbox();
    let [trace_2_col91]: [QM31; 1] = (*trace_2_col91.try_into().unwrap()).unbox();
    let [trace_2_col92]: [QM31; 1] = (*trace_2_col92.try_into().unwrap()).unbox();
    let [trace_2_col93]: [QM31; 1] = (*trace_2_col93.try_into().unwrap()).unbox();
    let [trace_2_col94]: [QM31; 1] = (*trace_2_col94.try_into().unwrap()).unbox();
    let [trace_2_col95]: [QM31; 1] = (*trace_2_col95.try_into().unwrap()).unbox();
    let [trace_2_col96]: [QM31; 1] = (*trace_2_col96.try_into().unwrap()).unbox();
    let [trace_2_col97]: [QM31; 1] = (*trace_2_col97.try_into().unwrap()).unbox();
    let [trace_2_col98]: [QM31; 1] = (*trace_2_col98.try_into().unwrap()).unbox();
    let [trace_2_col99]: [QM31; 1] = (*trace_2_col99.try_into().unwrap()).unbox();
    let [trace_2_col100]: [QM31; 1] = (*trace_2_col100.try_into().unwrap()).unbox();
    let [trace_2_col101]: [QM31; 1] = (*trace_2_col101.try_into().unwrap()).unbox();
    let [trace_2_col102]: [QM31; 1] = (*trace_2_col102.try_into().unwrap()).unbox();
    let [trace_2_col103]: [QM31; 1] = (*trace_2_col103.try_into().unwrap()).unbox();
    let [trace_2_col104]: [QM31; 1] = (*trace_2_col104.try_into().unwrap()).unbox();
    let [trace_2_col105]: [QM31; 1] = (*trace_2_col105.try_into().unwrap()).unbox();
    let [trace_2_col106]: [QM31; 1] = (*trace_2_col106.try_into().unwrap()).unbox();
    let [trace_2_col107]: [QM31; 1] = (*trace_2_col107.try_into().unwrap()).unbox();
    let [trace_2_col108]: [QM31; 1] = (*trace_2_col108.try_into().unwrap()).unbox();
    let [trace_2_col109]: [QM31; 1] = (*trace_2_col109.try_into().unwrap()).unbox();
    let [trace_2_col110]: [QM31; 1] = (*trace_2_col110.try_into().unwrap()).unbox();
    let [trace_2_col111]: [QM31; 1] = (*trace_2_col111.try_into().unwrap()).unbox();
    let [trace_2_col112]: [QM31; 1] = (*trace_2_col112.try_into().unwrap()).unbox();
    let [trace_2_col113]: [QM31; 1] = (*trace_2_col113.try_into().unwrap()).unbox();
    let [trace_2_col114]: [QM31; 1] = (*trace_2_col114.try_into().unwrap()).unbox();
    let [trace_2_col115]: [QM31; 1] = (*trace_2_col115.try_into().unwrap()).unbox();
    let [trace_2_col116_neg1, trace_2_col116]: [QM31; 2] = (*trace_2_col116.try_into().unwrap())
        .unbox();
    let [trace_2_col117_neg1, trace_2_col117]: [QM31; 2] = (*trace_2_col117.try_into().unwrap())
        .unbox();
    let [trace_2_col118_neg1, trace_2_col118]: [QM31; 2] = (*trace_2_col118.try_into().unwrap())
        .unbox();
    let [trace_2_col119_neg1, trace_2_col119]: [QM31; 2] = (*trace_2_col119.try_into().unwrap())
        .unbox();

    core::internal::revoke_ap_tracking();

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3],
    ))
        * blake_round_sigma_sum_0
        * range_check_7_2_5_sum_1)
        - blake_round_sigma_sum_0
        - range_check_7_2_5_sum_1)
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
        - QM31Impl::from_partial_evals([trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7]))
        * range_check_7_2_5_sum_4
        * memory_address_to_id_sum_5)
        - range_check_7_2_5_sum_4
        - memory_address_to_id_sum_5)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
    )
        - QM31Impl::from_partial_evals([trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11]))
        * memory_id_to_big_sum_6
        * range_check_7_2_5_sum_7)
        - memory_id_to_big_sum_6
        - range_check_7_2_5_sum_7)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
        ))
        * memory_address_to_id_sum_8
        * memory_id_to_big_sum_9)
        - memory_address_to_id_sum_8
        - memory_id_to_big_sum_9)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
        ))
        * range_check_7_2_5_sum_10
        * memory_address_to_id_sum_11)
        - range_check_7_2_5_sum_10
        - memory_address_to_id_sum_11)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23],
        ))
        * memory_id_to_big_sum_12
        * range_check_7_2_5_sum_13)
        - memory_id_to_big_sum_12
        - range_check_7_2_5_sum_13)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col28, trace_2_col29, trace_2_col30, trace_2_col31],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27],
        ))
        * memory_address_to_id_sum_14
        * memory_id_to_big_sum_15)
        - memory_address_to_id_sum_14
        - memory_id_to_big_sum_15)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col32, trace_2_col33, trace_2_col34, trace_2_col35],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col28, trace_2_col29, trace_2_col30, trace_2_col31],
        ))
        * range_check_7_2_5_sum_16
        * memory_address_to_id_sum_17)
        - range_check_7_2_5_sum_16
        - memory_address_to_id_sum_17)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col36, trace_2_col37, trace_2_col38, trace_2_col39],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col32, trace_2_col33, trace_2_col34, trace_2_col35],
        ))
        * memory_id_to_big_sum_18
        * range_check_7_2_5_sum_19)
        - memory_id_to_big_sum_18
        - range_check_7_2_5_sum_19)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col40, trace_2_col41, trace_2_col42, trace_2_col43],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col36, trace_2_col37, trace_2_col38, trace_2_col39],
        ))
        * memory_address_to_id_sum_20
        * memory_id_to_big_sum_21)
        - memory_address_to_id_sum_20
        - memory_id_to_big_sum_21)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col44, trace_2_col45, trace_2_col46, trace_2_col47],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col40, trace_2_col41, trace_2_col42, trace_2_col43],
        ))
        * range_check_7_2_5_sum_22
        * memory_address_to_id_sum_23)
        - range_check_7_2_5_sum_22
        - memory_address_to_id_sum_23)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col48, trace_2_col49, trace_2_col50, trace_2_col51],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col44, trace_2_col45, trace_2_col46, trace_2_col47],
        ))
        * memory_id_to_big_sum_24
        * range_check_7_2_5_sum_25)
        - memory_id_to_big_sum_24
        - range_check_7_2_5_sum_25)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col52, trace_2_col53, trace_2_col54, trace_2_col55],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col48, trace_2_col49, trace_2_col50, trace_2_col51],
        ))
        * memory_address_to_id_sum_26
        * memory_id_to_big_sum_27)
        - memory_address_to_id_sum_26
        - memory_id_to_big_sum_27)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col56, trace_2_col57, trace_2_col58, trace_2_col59],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col52, trace_2_col53, trace_2_col54, trace_2_col55],
        ))
        * range_check_7_2_5_sum_28
        * memory_address_to_id_sum_29)
        - range_check_7_2_5_sum_28
        - memory_address_to_id_sum_29)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col60, trace_2_col61, trace_2_col62, trace_2_col63],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col56, trace_2_col57, trace_2_col58, trace_2_col59],
        ))
        * memory_id_to_big_sum_30
        * range_check_7_2_5_sum_31)
        - memory_id_to_big_sum_30
        - range_check_7_2_5_sum_31)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col64, trace_2_col65, trace_2_col66, trace_2_col67],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col60, trace_2_col61, trace_2_col62, trace_2_col63],
        ))
        * memory_address_to_id_sum_32
        * memory_id_to_big_sum_33)
        - memory_address_to_id_sum_32
        - memory_id_to_big_sum_33)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col68, trace_2_col69, trace_2_col70, trace_2_col71],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col64, trace_2_col65, trace_2_col66, trace_2_col67],
        ))
        * range_check_7_2_5_sum_34
        * memory_address_to_id_sum_35)
        - range_check_7_2_5_sum_34
        - memory_address_to_id_sum_35)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col72, trace_2_col73, trace_2_col74, trace_2_col75],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col68, trace_2_col69, trace_2_col70, trace_2_col71],
        ))
        * memory_id_to_big_sum_36
        * range_check_7_2_5_sum_37)
        - memory_id_to_big_sum_36
        - range_check_7_2_5_sum_37)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col76, trace_2_col77, trace_2_col78, trace_2_col79],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col72, trace_2_col73, trace_2_col74, trace_2_col75],
        ))
        * memory_address_to_id_sum_38
        * memory_id_to_big_sum_39)
        - memory_address_to_id_sum_38
        - memory_id_to_big_sum_39)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col80, trace_2_col81, trace_2_col82, trace_2_col83],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col76, trace_2_col77, trace_2_col78, trace_2_col79],
        ))
        * range_check_7_2_5_sum_40
        * memory_address_to_id_sum_41)
        - range_check_7_2_5_sum_40
        - memory_address_to_id_sum_41)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col84, trace_2_col85, trace_2_col86, trace_2_col87],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col80, trace_2_col81, trace_2_col82, trace_2_col83],
        ))
        * memory_id_to_big_sum_42
        * range_check_7_2_5_sum_43)
        - memory_id_to_big_sum_42
        - range_check_7_2_5_sum_43)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col88, trace_2_col89, trace_2_col90, trace_2_col91],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col84, trace_2_col85, trace_2_col86, trace_2_col87],
        ))
        * memory_address_to_id_sum_44
        * memory_id_to_big_sum_45)
        - memory_address_to_id_sum_44
        - memory_id_to_big_sum_45)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col92, trace_2_col93, trace_2_col94, trace_2_col95],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col88, trace_2_col89, trace_2_col90, trace_2_col91],
        ))
        * range_check_7_2_5_sum_46
        * memory_address_to_id_sum_47)
        - range_check_7_2_5_sum_46
        - memory_address_to_id_sum_47)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col96, trace_2_col97, trace_2_col98, trace_2_col99],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col92, trace_2_col93, trace_2_col94, trace_2_col95],
        ))
        * memory_id_to_big_sum_48
        * blake_g_sum_49)
        - memory_id_to_big_sum_48
        - blake_g_sum_49)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col100, trace_2_col101, trace_2_col102, trace_2_col103],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col96, trace_2_col97, trace_2_col98, trace_2_col99],
        ))
        * blake_g_sum_50
        * blake_g_sum_51)
        - blake_g_sum_50
        - blake_g_sum_51)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col104, trace_2_col105, trace_2_col106, trace_2_col107],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col100, trace_2_col101, trace_2_col102, trace_2_col103],
        ))
        * blake_g_sum_52
        * blake_g_sum_53)
        - blake_g_sum_52
        - blake_g_sum_53)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col108, trace_2_col109, trace_2_col110, trace_2_col111],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col104, trace_2_col105, trace_2_col106, trace_2_col107],
        ))
        * blake_g_sum_54
        * blake_g_sum_55)
        - blake_g_sum_54
        - blake_g_sum_55)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col112, trace_2_col113, trace_2_col114, trace_2_col115],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col108, trace_2_col109, trace_2_col110, trace_2_col111],
        ))
        * blake_g_sum_56
        * blake_round_sum_57)
        - (blake_g_sum_56 * enabler)
        - blake_round_sum_57)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col116, trace_2_col117, trace_2_col118, trace_2_col119],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col112, trace_2_col113, trace_2_col114, trace_2_col115],
        )
        - QM31Impl::from_partial_evals(
            [trace_2_col116_neg1, trace_2_col117_neg1, trace_2_col118_neg1, trace_2_col119_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * blake_round_sum_58)
        + enabler)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
}
