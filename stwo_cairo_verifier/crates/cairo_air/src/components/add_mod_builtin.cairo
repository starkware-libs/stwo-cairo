// Constraints version: 9330aaaf

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
use crate::components::subroutines::mod_utils::mod_utils_evaluate;
use crate::utils::U32Impl;

pub const N_TRACE_COLUMNS: usize = 251;

#[derive(Drop, Serde, Copy)]
pub struct Claim {
    pub log_size: u32,
    pub add_mod_builtin_segment_start: u32,
}

#[generate_trait]
pub impl ClaimImpl of ClaimTrait {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = *(self.log_size);
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = ArrayImpl::new_repeated(N_TRACE_COLUMNS, log_size).span();
        let interaction_log_sizes = ArrayImpl::new_repeated(108, log_size).span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        channel.mix_u64((*(self.log_size)).into());
        channel.mix_u64((*self.add_mod_builtin_segment_start).into());
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
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
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
        let add_mod_builtin_segment_start: QM31 = (TryInto::<
            u32, M31,
        >::try_into((*(self.claim.add_mod_builtin_segment_start)))
            .unwrap())
            .into();
        let mut memory_address_to_id_sum_0: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_1: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_2: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_3: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_4: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_5: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_6: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_7: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_8: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_9: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_10: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_11: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_12: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_13: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_14: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_15: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_16: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_17: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_18: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_19: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_20: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_21: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_22: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_23: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_24: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_25: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_26: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_27: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_28: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_29: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_30: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_31: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_32: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_33: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_34: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_35: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_36: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_37: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_38: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_39: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_40: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_41: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_42: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_43: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_44: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_45: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_46: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_47: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_48: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_49: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_50: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_51: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_52: QM31 = Zero::zero();
        let seq = preprocessed_mask_values.get(PreprocessedColumn::Seq(*(self.claim.log_size)));

        let [
            is_instance_0_col0,
            p0_id_col1,
            p0_limb_0_col2,
            p0_limb_1_col3,
            p0_limb_2_col4,
            p0_limb_3_col5,
            p0_limb_4_col6,
            p0_limb_5_col7,
            p0_limb_6_col8,
            p0_limb_7_col9,
            p0_limb_8_col10,
            p0_limb_9_col11,
            p0_limb_10_col12,
            p1_id_col13,
            p1_limb_0_col14,
            p1_limb_1_col15,
            p1_limb_2_col16,
            p1_limb_3_col17,
            p1_limb_4_col18,
            p1_limb_5_col19,
            p1_limb_6_col20,
            p1_limb_7_col21,
            p1_limb_8_col22,
            p1_limb_9_col23,
            p1_limb_10_col24,
            p2_id_col25,
            p2_limb_0_col26,
            p2_limb_1_col27,
            p2_limb_2_col28,
            p2_limb_3_col29,
            p2_limb_4_col30,
            p2_limb_5_col31,
            p2_limb_6_col32,
            p2_limb_7_col33,
            p2_limb_8_col34,
            p2_limb_9_col35,
            p2_limb_10_col36,
            p3_id_col37,
            p3_limb_0_col38,
            p3_limb_1_col39,
            p3_limb_2_col40,
            p3_limb_3_col41,
            p3_limb_4_col42,
            p3_limb_5_col43,
            p3_limb_6_col44,
            p3_limb_7_col45,
            p3_limb_8_col46,
            p3_limb_9_col47,
            p3_limb_10_col48,
            values_ptr_id_col49,
            values_ptr_limb_0_col50,
            values_ptr_limb_1_col51,
            values_ptr_limb_2_col52,
            offsets_ptr_id_col53,
            offsets_ptr_limb_0_col54,
            offsets_ptr_limb_1_col55,
            offsets_ptr_limb_2_col56,
            offsets_ptr_prev_id_col57,
            offsets_ptr_prev_limb_0_col58,
            offsets_ptr_prev_limb_1_col59,
            offsets_ptr_prev_limb_2_col60,
            n_id_col61,
            n_limb_0_col62,
            n_limb_1_col63,
            n_limb_2_col64,
            n_prev_id_col65,
            n_prev_limb_0_col66,
            n_prev_limb_1_col67,
            n_prev_limb_2_col68,
            values_ptr_prev_id_col69,
            p_prev0_id_col70,
            p_prev1_id_col71,
            p_prev2_id_col72,
            p_prev3_id_col73,
            offsets_a_id_col74,
            msb_col75,
            mid_limbs_set_col76,
            offsets_a_limb_0_col77,
            offsets_a_limb_1_col78,
            offsets_a_limb_2_col79,
            offsets_b_id_col80,
            msb_col81,
            mid_limbs_set_col82,
            offsets_b_limb_0_col83,
            offsets_b_limb_1_col84,
            offsets_b_limb_2_col85,
            offsets_c_id_col86,
            msb_col87,
            mid_limbs_set_col88,
            offsets_c_limb_0_col89,
            offsets_c_limb_1_col90,
            offsets_c_limb_2_col91,
            a0_id_col92,
            a0_limb_0_col93,
            a0_limb_1_col94,
            a0_limb_2_col95,
            a0_limb_3_col96,
            a0_limb_4_col97,
            a0_limb_5_col98,
            a0_limb_6_col99,
            a0_limb_7_col100,
            a0_limb_8_col101,
            a0_limb_9_col102,
            a0_limb_10_col103,
            a1_id_col104,
            a1_limb_0_col105,
            a1_limb_1_col106,
            a1_limb_2_col107,
            a1_limb_3_col108,
            a1_limb_4_col109,
            a1_limb_5_col110,
            a1_limb_6_col111,
            a1_limb_7_col112,
            a1_limb_8_col113,
            a1_limb_9_col114,
            a1_limb_10_col115,
            a2_id_col116,
            a2_limb_0_col117,
            a2_limb_1_col118,
            a2_limb_2_col119,
            a2_limb_3_col120,
            a2_limb_4_col121,
            a2_limb_5_col122,
            a2_limb_6_col123,
            a2_limb_7_col124,
            a2_limb_8_col125,
            a2_limb_9_col126,
            a2_limb_10_col127,
            a3_id_col128,
            a3_limb_0_col129,
            a3_limb_1_col130,
            a3_limb_2_col131,
            a3_limb_3_col132,
            a3_limb_4_col133,
            a3_limb_5_col134,
            a3_limb_6_col135,
            a3_limb_7_col136,
            a3_limb_8_col137,
            a3_limb_9_col138,
            a3_limb_10_col139,
            b0_id_col140,
            b0_limb_0_col141,
            b0_limb_1_col142,
            b0_limb_2_col143,
            b0_limb_3_col144,
            b0_limb_4_col145,
            b0_limb_5_col146,
            b0_limb_6_col147,
            b0_limb_7_col148,
            b0_limb_8_col149,
            b0_limb_9_col150,
            b0_limb_10_col151,
            b1_id_col152,
            b1_limb_0_col153,
            b1_limb_1_col154,
            b1_limb_2_col155,
            b1_limb_3_col156,
            b1_limb_4_col157,
            b1_limb_5_col158,
            b1_limb_6_col159,
            b1_limb_7_col160,
            b1_limb_8_col161,
            b1_limb_9_col162,
            b1_limb_10_col163,
            b2_id_col164,
            b2_limb_0_col165,
            b2_limb_1_col166,
            b2_limb_2_col167,
            b2_limb_3_col168,
            b2_limb_4_col169,
            b2_limb_5_col170,
            b2_limb_6_col171,
            b2_limb_7_col172,
            b2_limb_8_col173,
            b2_limb_9_col174,
            b2_limb_10_col175,
            b3_id_col176,
            b3_limb_0_col177,
            b3_limb_1_col178,
            b3_limb_2_col179,
            b3_limb_3_col180,
            b3_limb_4_col181,
            b3_limb_5_col182,
            b3_limb_6_col183,
            b3_limb_7_col184,
            b3_limb_8_col185,
            b3_limb_9_col186,
            b3_limb_10_col187,
            c0_id_col188,
            c0_limb_0_col189,
            c0_limb_1_col190,
            c0_limb_2_col191,
            c0_limb_3_col192,
            c0_limb_4_col193,
            c0_limb_5_col194,
            c0_limb_6_col195,
            c0_limb_7_col196,
            c0_limb_8_col197,
            c0_limb_9_col198,
            c0_limb_10_col199,
            c1_id_col200,
            c1_limb_0_col201,
            c1_limb_1_col202,
            c1_limb_2_col203,
            c1_limb_3_col204,
            c1_limb_4_col205,
            c1_limb_5_col206,
            c1_limb_6_col207,
            c1_limb_7_col208,
            c1_limb_8_col209,
            c1_limb_9_col210,
            c1_limb_10_col211,
            c2_id_col212,
            c2_limb_0_col213,
            c2_limb_1_col214,
            c2_limb_2_col215,
            c2_limb_3_col216,
            c2_limb_4_col217,
            c2_limb_5_col218,
            c2_limb_6_col219,
            c2_limb_7_col220,
            c2_limb_8_col221,
            c2_limb_9_col222,
            c2_limb_10_col223,
            c3_id_col224,
            c3_limb_0_col225,
            c3_limb_1_col226,
            c3_limb_2_col227,
            c3_limb_3_col228,
            c3_limb_4_col229,
            c3_limb_5_col230,
            c3_limb_6_col231,
            c3_limb_7_col232,
            c3_limb_8_col233,
            c3_limb_9_col234,
            c3_limb_10_col235,
            sub_p_bit_col236,
            carry_0_col237,
            carry_1_col238,
            carry_2_col239,
            carry_3_col240,
            carry_4_col241,
            carry_5_col242,
            carry_6_col243,
            carry_7_col244,
            carry_8_col245,
            carry_9_col246,
            carry_10_col247,
            carry_11_col248,
            carry_12_col249,
            carry_13_col250,
        ]: [Span<QM31>; 251] =
            (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [is_instance_0_col0]: [QM31; 1] = (*is_instance_0_col0.try_into().unwrap()).unbox();
        let [p0_id_col1]: [QM31; 1] = (*p0_id_col1.try_into().unwrap()).unbox();
        let [p0_limb_0_col2]: [QM31; 1] = (*p0_limb_0_col2.try_into().unwrap()).unbox();
        let [p0_limb_1_col3]: [QM31; 1] = (*p0_limb_1_col3.try_into().unwrap()).unbox();
        let [p0_limb_2_col4]: [QM31; 1] = (*p0_limb_2_col4.try_into().unwrap()).unbox();
        let [p0_limb_3_col5]: [QM31; 1] = (*p0_limb_3_col5.try_into().unwrap()).unbox();
        let [p0_limb_4_col6]: [QM31; 1] = (*p0_limb_4_col6.try_into().unwrap()).unbox();
        let [p0_limb_5_col7]: [QM31; 1] = (*p0_limb_5_col7.try_into().unwrap()).unbox();
        let [p0_limb_6_col8]: [QM31; 1] = (*p0_limb_6_col8.try_into().unwrap()).unbox();
        let [p0_limb_7_col9]: [QM31; 1] = (*p0_limb_7_col9.try_into().unwrap()).unbox();
        let [p0_limb_8_col10]: [QM31; 1] = (*p0_limb_8_col10.try_into().unwrap()).unbox();
        let [p0_limb_9_col11]: [QM31; 1] = (*p0_limb_9_col11.try_into().unwrap()).unbox();
        let [p0_limb_10_col12]: [QM31; 1] = (*p0_limb_10_col12.try_into().unwrap()).unbox();
        let [p1_id_col13]: [QM31; 1] = (*p1_id_col13.try_into().unwrap()).unbox();
        let [p1_limb_0_col14]: [QM31; 1] = (*p1_limb_0_col14.try_into().unwrap()).unbox();
        let [p1_limb_1_col15]: [QM31; 1] = (*p1_limb_1_col15.try_into().unwrap()).unbox();
        let [p1_limb_2_col16]: [QM31; 1] = (*p1_limb_2_col16.try_into().unwrap()).unbox();
        let [p1_limb_3_col17]: [QM31; 1] = (*p1_limb_3_col17.try_into().unwrap()).unbox();
        let [p1_limb_4_col18]: [QM31; 1] = (*p1_limb_4_col18.try_into().unwrap()).unbox();
        let [p1_limb_5_col19]: [QM31; 1] = (*p1_limb_5_col19.try_into().unwrap()).unbox();
        let [p1_limb_6_col20]: [QM31; 1] = (*p1_limb_6_col20.try_into().unwrap()).unbox();
        let [p1_limb_7_col21]: [QM31; 1] = (*p1_limb_7_col21.try_into().unwrap()).unbox();
        let [p1_limb_8_col22]: [QM31; 1] = (*p1_limb_8_col22.try_into().unwrap()).unbox();
        let [p1_limb_9_col23]: [QM31; 1] = (*p1_limb_9_col23.try_into().unwrap()).unbox();
        let [p1_limb_10_col24]: [QM31; 1] = (*p1_limb_10_col24.try_into().unwrap()).unbox();
        let [p2_id_col25]: [QM31; 1] = (*p2_id_col25.try_into().unwrap()).unbox();
        let [p2_limb_0_col26]: [QM31; 1] = (*p2_limb_0_col26.try_into().unwrap()).unbox();
        let [p2_limb_1_col27]: [QM31; 1] = (*p2_limb_1_col27.try_into().unwrap()).unbox();
        let [p2_limb_2_col28]: [QM31; 1] = (*p2_limb_2_col28.try_into().unwrap()).unbox();
        let [p2_limb_3_col29]: [QM31; 1] = (*p2_limb_3_col29.try_into().unwrap()).unbox();
        let [p2_limb_4_col30]: [QM31; 1] = (*p2_limb_4_col30.try_into().unwrap()).unbox();
        let [p2_limb_5_col31]: [QM31; 1] = (*p2_limb_5_col31.try_into().unwrap()).unbox();
        let [p2_limb_6_col32]: [QM31; 1] = (*p2_limb_6_col32.try_into().unwrap()).unbox();
        let [p2_limb_7_col33]: [QM31; 1] = (*p2_limb_7_col33.try_into().unwrap()).unbox();
        let [p2_limb_8_col34]: [QM31; 1] = (*p2_limb_8_col34.try_into().unwrap()).unbox();
        let [p2_limb_9_col35]: [QM31; 1] = (*p2_limb_9_col35.try_into().unwrap()).unbox();
        let [p2_limb_10_col36]: [QM31; 1] = (*p2_limb_10_col36.try_into().unwrap()).unbox();
        let [p3_id_col37]: [QM31; 1] = (*p3_id_col37.try_into().unwrap()).unbox();
        let [p3_limb_0_col38]: [QM31; 1] = (*p3_limb_0_col38.try_into().unwrap()).unbox();
        let [p3_limb_1_col39]: [QM31; 1] = (*p3_limb_1_col39.try_into().unwrap()).unbox();
        let [p3_limb_2_col40]: [QM31; 1] = (*p3_limb_2_col40.try_into().unwrap()).unbox();
        let [p3_limb_3_col41]: [QM31; 1] = (*p3_limb_3_col41.try_into().unwrap()).unbox();
        let [p3_limb_4_col42]: [QM31; 1] = (*p3_limb_4_col42.try_into().unwrap()).unbox();
        let [p3_limb_5_col43]: [QM31; 1] = (*p3_limb_5_col43.try_into().unwrap()).unbox();
        let [p3_limb_6_col44]: [QM31; 1] = (*p3_limb_6_col44.try_into().unwrap()).unbox();
        let [p3_limb_7_col45]: [QM31; 1] = (*p3_limb_7_col45.try_into().unwrap()).unbox();
        let [p3_limb_8_col46]: [QM31; 1] = (*p3_limb_8_col46.try_into().unwrap()).unbox();
        let [p3_limb_9_col47]: [QM31; 1] = (*p3_limb_9_col47.try_into().unwrap()).unbox();
        let [p3_limb_10_col48]: [QM31; 1] = (*p3_limb_10_col48.try_into().unwrap()).unbox();
        let [values_ptr_id_col49]: [QM31; 1] = (*values_ptr_id_col49.try_into().unwrap()).unbox();
        let [values_ptr_limb_0_col50]: [QM31; 1] = (*values_ptr_limb_0_col50.try_into().unwrap())
            .unbox();
        let [values_ptr_limb_1_col51]: [QM31; 1] = (*values_ptr_limb_1_col51.try_into().unwrap())
            .unbox();
        let [values_ptr_limb_2_col52]: [QM31; 1] = (*values_ptr_limb_2_col52.try_into().unwrap())
            .unbox();
        let [offsets_ptr_id_col53]: [QM31; 1] = (*offsets_ptr_id_col53.try_into().unwrap()).unbox();
        let [offsets_ptr_limb_0_col54]: [QM31; 1] = (*offsets_ptr_limb_0_col54.try_into().unwrap())
            .unbox();
        let [offsets_ptr_limb_1_col55]: [QM31; 1] = (*offsets_ptr_limb_1_col55.try_into().unwrap())
            .unbox();
        let [offsets_ptr_limb_2_col56]: [QM31; 1] = (*offsets_ptr_limb_2_col56.try_into().unwrap())
            .unbox();
        let [offsets_ptr_prev_id_col57]: [QM31; 1] = (*offsets_ptr_prev_id_col57
            .try_into()
            .unwrap())
            .unbox();
        let [offsets_ptr_prev_limb_0_col58]: [QM31; 1] = (*offsets_ptr_prev_limb_0_col58
            .try_into()
            .unwrap())
            .unbox();
        let [offsets_ptr_prev_limb_1_col59]: [QM31; 1] = (*offsets_ptr_prev_limb_1_col59
            .try_into()
            .unwrap())
            .unbox();
        let [offsets_ptr_prev_limb_2_col60]: [QM31; 1] = (*offsets_ptr_prev_limb_2_col60
            .try_into()
            .unwrap())
            .unbox();
        let [n_id_col61]: [QM31; 1] = (*n_id_col61.try_into().unwrap()).unbox();
        let [n_limb_0_col62]: [QM31; 1] = (*n_limb_0_col62.try_into().unwrap()).unbox();
        let [n_limb_1_col63]: [QM31; 1] = (*n_limb_1_col63.try_into().unwrap()).unbox();
        let [n_limb_2_col64]: [QM31; 1] = (*n_limb_2_col64.try_into().unwrap()).unbox();
        let [n_prev_id_col65]: [QM31; 1] = (*n_prev_id_col65.try_into().unwrap()).unbox();
        let [n_prev_limb_0_col66]: [QM31; 1] = (*n_prev_limb_0_col66.try_into().unwrap()).unbox();
        let [n_prev_limb_1_col67]: [QM31; 1] = (*n_prev_limb_1_col67.try_into().unwrap()).unbox();
        let [n_prev_limb_2_col68]: [QM31; 1] = (*n_prev_limb_2_col68.try_into().unwrap()).unbox();
        let [values_ptr_prev_id_col69]: [QM31; 1] = (*values_ptr_prev_id_col69.try_into().unwrap())
            .unbox();
        let [p_prev0_id_col70]: [QM31; 1] = (*p_prev0_id_col70.try_into().unwrap()).unbox();
        let [p_prev1_id_col71]: [QM31; 1] = (*p_prev1_id_col71.try_into().unwrap()).unbox();
        let [p_prev2_id_col72]: [QM31; 1] = (*p_prev2_id_col72.try_into().unwrap()).unbox();
        let [p_prev3_id_col73]: [QM31; 1] = (*p_prev3_id_col73.try_into().unwrap()).unbox();
        let [offsets_a_id_col74]: [QM31; 1] = (*offsets_a_id_col74.try_into().unwrap()).unbox();
        let [msb_col75]: [QM31; 1] = (*msb_col75.try_into().unwrap()).unbox();
        let [mid_limbs_set_col76]: [QM31; 1] = (*mid_limbs_set_col76.try_into().unwrap()).unbox();
        let [offsets_a_limb_0_col77]: [QM31; 1] = (*offsets_a_limb_0_col77.try_into().unwrap())
            .unbox();
        let [offsets_a_limb_1_col78]: [QM31; 1] = (*offsets_a_limb_1_col78.try_into().unwrap())
            .unbox();
        let [offsets_a_limb_2_col79]: [QM31; 1] = (*offsets_a_limb_2_col79.try_into().unwrap())
            .unbox();
        let [offsets_b_id_col80]: [QM31; 1] = (*offsets_b_id_col80.try_into().unwrap()).unbox();
        let [msb_col81]: [QM31; 1] = (*msb_col81.try_into().unwrap()).unbox();
        let [mid_limbs_set_col82]: [QM31; 1] = (*mid_limbs_set_col82.try_into().unwrap()).unbox();
        let [offsets_b_limb_0_col83]: [QM31; 1] = (*offsets_b_limb_0_col83.try_into().unwrap())
            .unbox();
        let [offsets_b_limb_1_col84]: [QM31; 1] = (*offsets_b_limb_1_col84.try_into().unwrap())
            .unbox();
        let [offsets_b_limb_2_col85]: [QM31; 1] = (*offsets_b_limb_2_col85.try_into().unwrap())
            .unbox();
        let [offsets_c_id_col86]: [QM31; 1] = (*offsets_c_id_col86.try_into().unwrap()).unbox();
        let [msb_col87]: [QM31; 1] = (*msb_col87.try_into().unwrap()).unbox();
        let [mid_limbs_set_col88]: [QM31; 1] = (*mid_limbs_set_col88.try_into().unwrap()).unbox();
        let [offsets_c_limb_0_col89]: [QM31; 1] = (*offsets_c_limb_0_col89.try_into().unwrap())
            .unbox();
        let [offsets_c_limb_1_col90]: [QM31; 1] = (*offsets_c_limb_1_col90.try_into().unwrap())
            .unbox();
        let [offsets_c_limb_2_col91]: [QM31; 1] = (*offsets_c_limb_2_col91.try_into().unwrap())
            .unbox();
        let [a0_id_col92]: [QM31; 1] = (*a0_id_col92.try_into().unwrap()).unbox();
        let [a0_limb_0_col93]: [QM31; 1] = (*a0_limb_0_col93.try_into().unwrap()).unbox();
        let [a0_limb_1_col94]: [QM31; 1] = (*a0_limb_1_col94.try_into().unwrap()).unbox();
        let [a0_limb_2_col95]: [QM31; 1] = (*a0_limb_2_col95.try_into().unwrap()).unbox();
        let [a0_limb_3_col96]: [QM31; 1] = (*a0_limb_3_col96.try_into().unwrap()).unbox();
        let [a0_limb_4_col97]: [QM31; 1] = (*a0_limb_4_col97.try_into().unwrap()).unbox();
        let [a0_limb_5_col98]: [QM31; 1] = (*a0_limb_5_col98.try_into().unwrap()).unbox();
        let [a0_limb_6_col99]: [QM31; 1] = (*a0_limb_6_col99.try_into().unwrap()).unbox();
        let [a0_limb_7_col100]: [QM31; 1] = (*a0_limb_7_col100.try_into().unwrap()).unbox();
        let [a0_limb_8_col101]: [QM31; 1] = (*a0_limb_8_col101.try_into().unwrap()).unbox();
        let [a0_limb_9_col102]: [QM31; 1] = (*a0_limb_9_col102.try_into().unwrap()).unbox();
        let [a0_limb_10_col103]: [QM31; 1] = (*a0_limb_10_col103.try_into().unwrap()).unbox();
        let [a1_id_col104]: [QM31; 1] = (*a1_id_col104.try_into().unwrap()).unbox();
        let [a1_limb_0_col105]: [QM31; 1] = (*a1_limb_0_col105.try_into().unwrap()).unbox();
        let [a1_limb_1_col106]: [QM31; 1] = (*a1_limb_1_col106.try_into().unwrap()).unbox();
        let [a1_limb_2_col107]: [QM31; 1] = (*a1_limb_2_col107.try_into().unwrap()).unbox();
        let [a1_limb_3_col108]: [QM31; 1] = (*a1_limb_3_col108.try_into().unwrap()).unbox();
        let [a1_limb_4_col109]: [QM31; 1] = (*a1_limb_4_col109.try_into().unwrap()).unbox();
        let [a1_limb_5_col110]: [QM31; 1] = (*a1_limb_5_col110.try_into().unwrap()).unbox();
        let [a1_limb_6_col111]: [QM31; 1] = (*a1_limb_6_col111.try_into().unwrap()).unbox();
        let [a1_limb_7_col112]: [QM31; 1] = (*a1_limb_7_col112.try_into().unwrap()).unbox();
        let [a1_limb_8_col113]: [QM31; 1] = (*a1_limb_8_col113.try_into().unwrap()).unbox();
        let [a1_limb_9_col114]: [QM31; 1] = (*a1_limb_9_col114.try_into().unwrap()).unbox();
        let [a1_limb_10_col115]: [QM31; 1] = (*a1_limb_10_col115.try_into().unwrap()).unbox();
        let [a2_id_col116]: [QM31; 1] = (*a2_id_col116.try_into().unwrap()).unbox();
        let [a2_limb_0_col117]: [QM31; 1] = (*a2_limb_0_col117.try_into().unwrap()).unbox();
        let [a2_limb_1_col118]: [QM31; 1] = (*a2_limb_1_col118.try_into().unwrap()).unbox();
        let [a2_limb_2_col119]: [QM31; 1] = (*a2_limb_2_col119.try_into().unwrap()).unbox();
        let [a2_limb_3_col120]: [QM31; 1] = (*a2_limb_3_col120.try_into().unwrap()).unbox();
        let [a2_limb_4_col121]: [QM31; 1] = (*a2_limb_4_col121.try_into().unwrap()).unbox();
        let [a2_limb_5_col122]: [QM31; 1] = (*a2_limb_5_col122.try_into().unwrap()).unbox();
        let [a2_limb_6_col123]: [QM31; 1] = (*a2_limb_6_col123.try_into().unwrap()).unbox();
        let [a2_limb_7_col124]: [QM31; 1] = (*a2_limb_7_col124.try_into().unwrap()).unbox();
        let [a2_limb_8_col125]: [QM31; 1] = (*a2_limb_8_col125.try_into().unwrap()).unbox();
        let [a2_limb_9_col126]: [QM31; 1] = (*a2_limb_9_col126.try_into().unwrap()).unbox();
        let [a2_limb_10_col127]: [QM31; 1] = (*a2_limb_10_col127.try_into().unwrap()).unbox();
        let [a3_id_col128]: [QM31; 1] = (*a3_id_col128.try_into().unwrap()).unbox();
        let [a3_limb_0_col129]: [QM31; 1] = (*a3_limb_0_col129.try_into().unwrap()).unbox();
        let [a3_limb_1_col130]: [QM31; 1] = (*a3_limb_1_col130.try_into().unwrap()).unbox();
        let [a3_limb_2_col131]: [QM31; 1] = (*a3_limb_2_col131.try_into().unwrap()).unbox();
        let [a3_limb_3_col132]: [QM31; 1] = (*a3_limb_3_col132.try_into().unwrap()).unbox();
        let [a3_limb_4_col133]: [QM31; 1] = (*a3_limb_4_col133.try_into().unwrap()).unbox();
        let [a3_limb_5_col134]: [QM31; 1] = (*a3_limb_5_col134.try_into().unwrap()).unbox();
        let [a3_limb_6_col135]: [QM31; 1] = (*a3_limb_6_col135.try_into().unwrap()).unbox();
        let [a3_limb_7_col136]: [QM31; 1] = (*a3_limb_7_col136.try_into().unwrap()).unbox();
        let [a3_limb_8_col137]: [QM31; 1] = (*a3_limb_8_col137.try_into().unwrap()).unbox();
        let [a3_limb_9_col138]: [QM31; 1] = (*a3_limb_9_col138.try_into().unwrap()).unbox();
        let [a3_limb_10_col139]: [QM31; 1] = (*a3_limb_10_col139.try_into().unwrap()).unbox();
        let [b0_id_col140]: [QM31; 1] = (*b0_id_col140.try_into().unwrap()).unbox();
        let [b0_limb_0_col141]: [QM31; 1] = (*b0_limb_0_col141.try_into().unwrap()).unbox();
        let [b0_limb_1_col142]: [QM31; 1] = (*b0_limb_1_col142.try_into().unwrap()).unbox();
        let [b0_limb_2_col143]: [QM31; 1] = (*b0_limb_2_col143.try_into().unwrap()).unbox();
        let [b0_limb_3_col144]: [QM31; 1] = (*b0_limb_3_col144.try_into().unwrap()).unbox();
        let [b0_limb_4_col145]: [QM31; 1] = (*b0_limb_4_col145.try_into().unwrap()).unbox();
        let [b0_limb_5_col146]: [QM31; 1] = (*b0_limb_5_col146.try_into().unwrap()).unbox();
        let [b0_limb_6_col147]: [QM31; 1] = (*b0_limb_6_col147.try_into().unwrap()).unbox();
        let [b0_limb_7_col148]: [QM31; 1] = (*b0_limb_7_col148.try_into().unwrap()).unbox();
        let [b0_limb_8_col149]: [QM31; 1] = (*b0_limb_8_col149.try_into().unwrap()).unbox();
        let [b0_limb_9_col150]: [QM31; 1] = (*b0_limb_9_col150.try_into().unwrap()).unbox();
        let [b0_limb_10_col151]: [QM31; 1] = (*b0_limb_10_col151.try_into().unwrap()).unbox();
        let [b1_id_col152]: [QM31; 1] = (*b1_id_col152.try_into().unwrap()).unbox();
        let [b1_limb_0_col153]: [QM31; 1] = (*b1_limb_0_col153.try_into().unwrap()).unbox();
        let [b1_limb_1_col154]: [QM31; 1] = (*b1_limb_1_col154.try_into().unwrap()).unbox();
        let [b1_limb_2_col155]: [QM31; 1] = (*b1_limb_2_col155.try_into().unwrap()).unbox();
        let [b1_limb_3_col156]: [QM31; 1] = (*b1_limb_3_col156.try_into().unwrap()).unbox();
        let [b1_limb_4_col157]: [QM31; 1] = (*b1_limb_4_col157.try_into().unwrap()).unbox();
        let [b1_limb_5_col158]: [QM31; 1] = (*b1_limb_5_col158.try_into().unwrap()).unbox();
        let [b1_limb_6_col159]: [QM31; 1] = (*b1_limb_6_col159.try_into().unwrap()).unbox();
        let [b1_limb_7_col160]: [QM31; 1] = (*b1_limb_7_col160.try_into().unwrap()).unbox();
        let [b1_limb_8_col161]: [QM31; 1] = (*b1_limb_8_col161.try_into().unwrap()).unbox();
        let [b1_limb_9_col162]: [QM31; 1] = (*b1_limb_9_col162.try_into().unwrap()).unbox();
        let [b1_limb_10_col163]: [QM31; 1] = (*b1_limb_10_col163.try_into().unwrap()).unbox();
        let [b2_id_col164]: [QM31; 1] = (*b2_id_col164.try_into().unwrap()).unbox();
        let [b2_limb_0_col165]: [QM31; 1] = (*b2_limb_0_col165.try_into().unwrap()).unbox();
        let [b2_limb_1_col166]: [QM31; 1] = (*b2_limb_1_col166.try_into().unwrap()).unbox();
        let [b2_limb_2_col167]: [QM31; 1] = (*b2_limb_2_col167.try_into().unwrap()).unbox();
        let [b2_limb_3_col168]: [QM31; 1] = (*b2_limb_3_col168.try_into().unwrap()).unbox();
        let [b2_limb_4_col169]: [QM31; 1] = (*b2_limb_4_col169.try_into().unwrap()).unbox();
        let [b2_limb_5_col170]: [QM31; 1] = (*b2_limb_5_col170.try_into().unwrap()).unbox();
        let [b2_limb_6_col171]: [QM31; 1] = (*b2_limb_6_col171.try_into().unwrap()).unbox();
        let [b2_limb_7_col172]: [QM31; 1] = (*b2_limb_7_col172.try_into().unwrap()).unbox();
        let [b2_limb_8_col173]: [QM31; 1] = (*b2_limb_8_col173.try_into().unwrap()).unbox();
        let [b2_limb_9_col174]: [QM31; 1] = (*b2_limb_9_col174.try_into().unwrap()).unbox();
        let [b2_limb_10_col175]: [QM31; 1] = (*b2_limb_10_col175.try_into().unwrap()).unbox();
        let [b3_id_col176]: [QM31; 1] = (*b3_id_col176.try_into().unwrap()).unbox();
        let [b3_limb_0_col177]: [QM31; 1] = (*b3_limb_0_col177.try_into().unwrap()).unbox();
        let [b3_limb_1_col178]: [QM31; 1] = (*b3_limb_1_col178.try_into().unwrap()).unbox();
        let [b3_limb_2_col179]: [QM31; 1] = (*b3_limb_2_col179.try_into().unwrap()).unbox();
        let [b3_limb_3_col180]: [QM31; 1] = (*b3_limb_3_col180.try_into().unwrap()).unbox();
        let [b3_limb_4_col181]: [QM31; 1] = (*b3_limb_4_col181.try_into().unwrap()).unbox();
        let [b3_limb_5_col182]: [QM31; 1] = (*b3_limb_5_col182.try_into().unwrap()).unbox();
        let [b3_limb_6_col183]: [QM31; 1] = (*b3_limb_6_col183.try_into().unwrap()).unbox();
        let [b3_limb_7_col184]: [QM31; 1] = (*b3_limb_7_col184.try_into().unwrap()).unbox();
        let [b3_limb_8_col185]: [QM31; 1] = (*b3_limb_8_col185.try_into().unwrap()).unbox();
        let [b3_limb_9_col186]: [QM31; 1] = (*b3_limb_9_col186.try_into().unwrap()).unbox();
        let [b3_limb_10_col187]: [QM31; 1] = (*b3_limb_10_col187.try_into().unwrap()).unbox();
        let [c0_id_col188]: [QM31; 1] = (*c0_id_col188.try_into().unwrap()).unbox();
        let [c0_limb_0_col189]: [QM31; 1] = (*c0_limb_0_col189.try_into().unwrap()).unbox();
        let [c0_limb_1_col190]: [QM31; 1] = (*c0_limb_1_col190.try_into().unwrap()).unbox();
        let [c0_limb_2_col191]: [QM31; 1] = (*c0_limb_2_col191.try_into().unwrap()).unbox();
        let [c0_limb_3_col192]: [QM31; 1] = (*c0_limb_3_col192.try_into().unwrap()).unbox();
        let [c0_limb_4_col193]: [QM31; 1] = (*c0_limb_4_col193.try_into().unwrap()).unbox();
        let [c0_limb_5_col194]: [QM31; 1] = (*c0_limb_5_col194.try_into().unwrap()).unbox();
        let [c0_limb_6_col195]: [QM31; 1] = (*c0_limb_6_col195.try_into().unwrap()).unbox();
        let [c0_limb_7_col196]: [QM31; 1] = (*c0_limb_7_col196.try_into().unwrap()).unbox();
        let [c0_limb_8_col197]: [QM31; 1] = (*c0_limb_8_col197.try_into().unwrap()).unbox();
        let [c0_limb_9_col198]: [QM31; 1] = (*c0_limb_9_col198.try_into().unwrap()).unbox();
        let [c0_limb_10_col199]: [QM31; 1] = (*c0_limb_10_col199.try_into().unwrap()).unbox();
        let [c1_id_col200]: [QM31; 1] = (*c1_id_col200.try_into().unwrap()).unbox();
        let [c1_limb_0_col201]: [QM31; 1] = (*c1_limb_0_col201.try_into().unwrap()).unbox();
        let [c1_limb_1_col202]: [QM31; 1] = (*c1_limb_1_col202.try_into().unwrap()).unbox();
        let [c1_limb_2_col203]: [QM31; 1] = (*c1_limb_2_col203.try_into().unwrap()).unbox();
        let [c1_limb_3_col204]: [QM31; 1] = (*c1_limb_3_col204.try_into().unwrap()).unbox();
        let [c1_limb_4_col205]: [QM31; 1] = (*c1_limb_4_col205.try_into().unwrap()).unbox();
        let [c1_limb_5_col206]: [QM31; 1] = (*c1_limb_5_col206.try_into().unwrap()).unbox();
        let [c1_limb_6_col207]: [QM31; 1] = (*c1_limb_6_col207.try_into().unwrap()).unbox();
        let [c1_limb_7_col208]: [QM31; 1] = (*c1_limb_7_col208.try_into().unwrap()).unbox();
        let [c1_limb_8_col209]: [QM31; 1] = (*c1_limb_8_col209.try_into().unwrap()).unbox();
        let [c1_limb_9_col210]: [QM31; 1] = (*c1_limb_9_col210.try_into().unwrap()).unbox();
        let [c1_limb_10_col211]: [QM31; 1] = (*c1_limb_10_col211.try_into().unwrap()).unbox();
        let [c2_id_col212]: [QM31; 1] = (*c2_id_col212.try_into().unwrap()).unbox();
        let [c2_limb_0_col213]: [QM31; 1] = (*c2_limb_0_col213.try_into().unwrap()).unbox();
        let [c2_limb_1_col214]: [QM31; 1] = (*c2_limb_1_col214.try_into().unwrap()).unbox();
        let [c2_limb_2_col215]: [QM31; 1] = (*c2_limb_2_col215.try_into().unwrap()).unbox();
        let [c2_limb_3_col216]: [QM31; 1] = (*c2_limb_3_col216.try_into().unwrap()).unbox();
        let [c2_limb_4_col217]: [QM31; 1] = (*c2_limb_4_col217.try_into().unwrap()).unbox();
        let [c2_limb_5_col218]: [QM31; 1] = (*c2_limb_5_col218.try_into().unwrap()).unbox();
        let [c2_limb_6_col219]: [QM31; 1] = (*c2_limb_6_col219.try_into().unwrap()).unbox();
        let [c2_limb_7_col220]: [QM31; 1] = (*c2_limb_7_col220.try_into().unwrap()).unbox();
        let [c2_limb_8_col221]: [QM31; 1] = (*c2_limb_8_col221.try_into().unwrap()).unbox();
        let [c2_limb_9_col222]: [QM31; 1] = (*c2_limb_9_col222.try_into().unwrap()).unbox();
        let [c2_limb_10_col223]: [QM31; 1] = (*c2_limb_10_col223.try_into().unwrap()).unbox();
        let [c3_id_col224]: [QM31; 1] = (*c3_id_col224.try_into().unwrap()).unbox();
        let [c3_limb_0_col225]: [QM31; 1] = (*c3_limb_0_col225.try_into().unwrap()).unbox();
        let [c3_limb_1_col226]: [QM31; 1] = (*c3_limb_1_col226.try_into().unwrap()).unbox();
        let [c3_limb_2_col227]: [QM31; 1] = (*c3_limb_2_col227.try_into().unwrap()).unbox();
        let [c3_limb_3_col228]: [QM31; 1] = (*c3_limb_3_col228.try_into().unwrap()).unbox();
        let [c3_limb_4_col229]: [QM31; 1] = (*c3_limb_4_col229.try_into().unwrap()).unbox();
        let [c3_limb_5_col230]: [QM31; 1] = (*c3_limb_5_col230.try_into().unwrap()).unbox();
        let [c3_limb_6_col231]: [QM31; 1] = (*c3_limb_6_col231.try_into().unwrap()).unbox();
        let [c3_limb_7_col232]: [QM31; 1] = (*c3_limb_7_col232.try_into().unwrap()).unbox();
        let [c3_limb_8_col233]: [QM31; 1] = (*c3_limb_8_col233.try_into().unwrap()).unbox();
        let [c3_limb_9_col234]: [QM31; 1] = (*c3_limb_9_col234.try_into().unwrap()).unbox();
        let [c3_limb_10_col235]: [QM31; 1] = (*c3_limb_10_col235.try_into().unwrap()).unbox();
        let [sub_p_bit_col236]: [QM31; 1] = (*sub_p_bit_col236.try_into().unwrap()).unbox();
        let [carry_0_col237]: [QM31; 1] = (*carry_0_col237.try_into().unwrap()).unbox();
        let [carry_1_col238]: [QM31; 1] = (*carry_1_col238.try_into().unwrap()).unbox();
        let [carry_2_col239]: [QM31; 1] = (*carry_2_col239.try_into().unwrap()).unbox();
        let [carry_3_col240]: [QM31; 1] = (*carry_3_col240.try_into().unwrap()).unbox();
        let [carry_4_col241]: [QM31; 1] = (*carry_4_col241.try_into().unwrap()).unbox();
        let [carry_5_col242]: [QM31; 1] = (*carry_5_col242.try_into().unwrap()).unbox();
        let [carry_6_col243]: [QM31; 1] = (*carry_6_col243.try_into().unwrap()).unbox();
        let [carry_7_col244]: [QM31; 1] = (*carry_7_col244.try_into().unwrap()).unbox();
        let [carry_8_col245]: [QM31; 1] = (*carry_8_col245.try_into().unwrap()).unbox();
        let [carry_9_col246]: [QM31; 1] = (*carry_9_col246.try_into().unwrap()).unbox();
        let [carry_10_col247]: [QM31; 1] = (*carry_10_col247.try_into().unwrap()).unbox();
        let [carry_11_col248]: [QM31; 1] = (*carry_11_col248.try_into().unwrap()).unbox();
        let [carry_12_col249]: [QM31; 1] = (*carry_12_col249.try_into().unwrap()).unbox();
        let [carry_13_col250]: [QM31; 1] = (*carry_13_col250.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        mod_utils_evaluate(
            [add_mod_builtin_segment_start, seq],
            is_instance_0_col0,
            p0_id_col1,
            p0_limb_0_col2,
            p0_limb_1_col3,
            p0_limb_2_col4,
            p0_limb_3_col5,
            p0_limb_4_col6,
            p0_limb_5_col7,
            p0_limb_6_col8,
            p0_limb_7_col9,
            p0_limb_8_col10,
            p0_limb_9_col11,
            p0_limb_10_col12,
            p1_id_col13,
            p1_limb_0_col14,
            p1_limb_1_col15,
            p1_limb_2_col16,
            p1_limb_3_col17,
            p1_limb_4_col18,
            p1_limb_5_col19,
            p1_limb_6_col20,
            p1_limb_7_col21,
            p1_limb_8_col22,
            p1_limb_9_col23,
            p1_limb_10_col24,
            p2_id_col25,
            p2_limb_0_col26,
            p2_limb_1_col27,
            p2_limb_2_col28,
            p2_limb_3_col29,
            p2_limb_4_col30,
            p2_limb_5_col31,
            p2_limb_6_col32,
            p2_limb_7_col33,
            p2_limb_8_col34,
            p2_limb_9_col35,
            p2_limb_10_col36,
            p3_id_col37,
            p3_limb_0_col38,
            p3_limb_1_col39,
            p3_limb_2_col40,
            p3_limb_3_col41,
            p3_limb_4_col42,
            p3_limb_5_col43,
            p3_limb_6_col44,
            p3_limb_7_col45,
            p3_limb_8_col46,
            p3_limb_9_col47,
            p3_limb_10_col48,
            values_ptr_id_col49,
            values_ptr_limb_0_col50,
            values_ptr_limb_1_col51,
            values_ptr_limb_2_col52,
            offsets_ptr_id_col53,
            offsets_ptr_limb_0_col54,
            offsets_ptr_limb_1_col55,
            offsets_ptr_limb_2_col56,
            offsets_ptr_prev_id_col57,
            offsets_ptr_prev_limb_0_col58,
            offsets_ptr_prev_limb_1_col59,
            offsets_ptr_prev_limb_2_col60,
            n_id_col61,
            n_limb_0_col62,
            n_limb_1_col63,
            n_limb_2_col64,
            n_prev_id_col65,
            n_prev_limb_0_col66,
            n_prev_limb_1_col67,
            n_prev_limb_2_col68,
            values_ptr_prev_id_col69,
            p_prev0_id_col70,
            p_prev1_id_col71,
            p_prev2_id_col72,
            p_prev3_id_col73,
            offsets_a_id_col74,
            msb_col75,
            mid_limbs_set_col76,
            offsets_a_limb_0_col77,
            offsets_a_limb_1_col78,
            offsets_a_limb_2_col79,
            offsets_b_id_col80,
            msb_col81,
            mid_limbs_set_col82,
            offsets_b_limb_0_col83,
            offsets_b_limb_1_col84,
            offsets_b_limb_2_col85,
            offsets_c_id_col86,
            msb_col87,
            mid_limbs_set_col88,
            offsets_c_limb_0_col89,
            offsets_c_limb_1_col90,
            offsets_c_limb_2_col91,
            a0_id_col92,
            a0_limb_0_col93,
            a0_limb_1_col94,
            a0_limb_2_col95,
            a0_limb_3_col96,
            a0_limb_4_col97,
            a0_limb_5_col98,
            a0_limb_6_col99,
            a0_limb_7_col100,
            a0_limb_8_col101,
            a0_limb_9_col102,
            a0_limb_10_col103,
            a1_id_col104,
            a1_limb_0_col105,
            a1_limb_1_col106,
            a1_limb_2_col107,
            a1_limb_3_col108,
            a1_limb_4_col109,
            a1_limb_5_col110,
            a1_limb_6_col111,
            a1_limb_7_col112,
            a1_limb_8_col113,
            a1_limb_9_col114,
            a1_limb_10_col115,
            a2_id_col116,
            a2_limb_0_col117,
            a2_limb_1_col118,
            a2_limb_2_col119,
            a2_limb_3_col120,
            a2_limb_4_col121,
            a2_limb_5_col122,
            a2_limb_6_col123,
            a2_limb_7_col124,
            a2_limb_8_col125,
            a2_limb_9_col126,
            a2_limb_10_col127,
            a3_id_col128,
            a3_limb_0_col129,
            a3_limb_1_col130,
            a3_limb_2_col131,
            a3_limb_3_col132,
            a3_limb_4_col133,
            a3_limb_5_col134,
            a3_limb_6_col135,
            a3_limb_7_col136,
            a3_limb_8_col137,
            a3_limb_9_col138,
            a3_limb_10_col139,
            b0_id_col140,
            b0_limb_0_col141,
            b0_limb_1_col142,
            b0_limb_2_col143,
            b0_limb_3_col144,
            b0_limb_4_col145,
            b0_limb_5_col146,
            b0_limb_6_col147,
            b0_limb_7_col148,
            b0_limb_8_col149,
            b0_limb_9_col150,
            b0_limb_10_col151,
            b1_id_col152,
            b1_limb_0_col153,
            b1_limb_1_col154,
            b1_limb_2_col155,
            b1_limb_3_col156,
            b1_limb_4_col157,
            b1_limb_5_col158,
            b1_limb_6_col159,
            b1_limb_7_col160,
            b1_limb_8_col161,
            b1_limb_9_col162,
            b1_limb_10_col163,
            b2_id_col164,
            b2_limb_0_col165,
            b2_limb_1_col166,
            b2_limb_2_col167,
            b2_limb_3_col168,
            b2_limb_4_col169,
            b2_limb_5_col170,
            b2_limb_6_col171,
            b2_limb_7_col172,
            b2_limb_8_col173,
            b2_limb_9_col174,
            b2_limb_10_col175,
            b3_id_col176,
            b3_limb_0_col177,
            b3_limb_1_col178,
            b3_limb_2_col179,
            b3_limb_3_col180,
            b3_limb_4_col181,
            b3_limb_5_col182,
            b3_limb_6_col183,
            b3_limb_7_col184,
            b3_limb_8_col185,
            b3_limb_9_col186,
            b3_limb_10_col187,
            c0_id_col188,
            c0_limb_0_col189,
            c0_limb_1_col190,
            c0_limb_2_col191,
            c0_limb_3_col192,
            c0_limb_4_col193,
            c0_limb_5_col194,
            c0_limb_6_col195,
            c0_limb_7_col196,
            c0_limb_8_col197,
            c0_limb_9_col198,
            c0_limb_10_col199,
            c1_id_col200,
            c1_limb_0_col201,
            c1_limb_1_col202,
            c1_limb_2_col203,
            c1_limb_3_col204,
            c1_limb_4_col205,
            c1_limb_5_col206,
            c1_limb_6_col207,
            c1_limb_7_col208,
            c1_limb_8_col209,
            c1_limb_9_col210,
            c1_limb_10_col211,
            c2_id_col212,
            c2_limb_0_col213,
            c2_limb_1_col214,
            c2_limb_2_col215,
            c2_limb_3_col216,
            c2_limb_4_col217,
            c2_limb_5_col218,
            c2_limb_6_col219,
            c2_limb_7_col220,
            c2_limb_8_col221,
            c2_limb_9_col222,
            c2_limb_10_col223,
            c3_id_col224,
            c3_limb_0_col225,
            c3_limb_1_col226,
            c3_limb_2_col227,
            c3_limb_3_col228,
            c3_limb_4_col229,
            c3_limb_5_col230,
            c3_limb_6_col231,
            c3_limb_7_col232,
            c3_limb_8_col233,
            c3_limb_9_col234,
            c3_limb_10_col235,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref memory_address_to_id_sum_0,
            ref memory_id_to_big_sum_1,
            ref memory_address_to_id_sum_2,
            ref memory_id_to_big_sum_3,
            ref memory_address_to_id_sum_4,
            ref memory_id_to_big_sum_5,
            ref memory_address_to_id_sum_6,
            ref memory_id_to_big_sum_7,
            ref memory_address_to_id_sum_8,
            ref memory_id_to_big_sum_9,
            ref memory_address_to_id_sum_10,
            ref memory_id_to_big_sum_11,
            ref memory_address_to_id_sum_12,
            ref memory_id_to_big_sum_13,
            ref memory_address_to_id_sum_14,
            ref memory_id_to_big_sum_15,
            ref memory_address_to_id_sum_16,
            ref memory_id_to_big_sum_17,
            ref memory_address_to_id_sum_18,
            ref memory_address_to_id_sum_19,
            ref memory_address_to_id_sum_20,
            ref memory_address_to_id_sum_21,
            ref memory_address_to_id_sum_22,
            ref memory_address_to_id_sum_23,
            ref memory_id_to_big_sum_24,
            ref memory_address_to_id_sum_25,
            ref memory_id_to_big_sum_26,
            ref memory_address_to_id_sum_27,
            ref memory_id_to_big_sum_28,
            ref memory_address_to_id_sum_29,
            ref memory_id_to_big_sum_30,
            ref memory_address_to_id_sum_31,
            ref memory_id_to_big_sum_32,
            ref memory_address_to_id_sum_33,
            ref memory_id_to_big_sum_34,
            ref memory_address_to_id_sum_35,
            ref memory_id_to_big_sum_36,
            ref memory_address_to_id_sum_37,
            ref memory_id_to_big_sum_38,
            ref memory_address_to_id_sum_39,
            ref memory_id_to_big_sum_40,
            ref memory_address_to_id_sum_41,
            ref memory_id_to_big_sum_42,
            ref memory_address_to_id_sum_43,
            ref memory_id_to_big_sum_44,
            ref memory_address_to_id_sum_45,
            ref memory_id_to_big_sum_46,
            ref memory_address_to_id_sum_47,
            ref memory_id_to_big_sum_48,
            ref memory_address_to_id_sum_49,
            ref memory_id_to_big_sum_50,
            ref memory_address_to_id_sum_51,
            ref memory_id_to_big_sum_52,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        // Constraint - make sure sub_p_bit is 0 or 1.
        let constraint_quotient = (((sub_p_bit_col236 - qm31_const::<1, 0, 0, 0>())
            * sub_p_bit_col236))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry is 0 or 1 or -1.
        let constraint_quotient = ((carry_0_col237
            * ((carry_0_col237 * carry_0_col237) - qm31_const::<1, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry is 0 or 1 or -1.
        let constraint_quotient = ((carry_1_col238
            * ((carry_1_col238 * carry_1_col238) - qm31_const::<1, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry is 0 or 1 or -1.
        let constraint_quotient = ((carry_2_col239
            * ((carry_2_col239 * carry_2_col239) - qm31_const::<1, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry is 0 or 1 or -1.
        let constraint_quotient = ((carry_3_col240
            * ((carry_3_col240 * carry_3_col240) - qm31_const::<1, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry is 0 or 1 or -1.
        let constraint_quotient = ((carry_4_col241
            * ((carry_4_col241 * carry_4_col241) - qm31_const::<1, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry is 0 or 1 or -1.
        let constraint_quotient = ((carry_5_col242
            * ((carry_5_col242 * carry_5_col242) - qm31_const::<1, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry is 0 or 1 or -1.
        let constraint_quotient = ((carry_6_col243
            * ((carry_6_col243 * carry_6_col243) - qm31_const::<1, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry is 0 or 1 or -1.
        let constraint_quotient = ((carry_7_col244
            * ((carry_7_col244 * carry_7_col244) - qm31_const::<1, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry is 0 or 1 or -1.
        let constraint_quotient = ((carry_8_col245
            * ((carry_8_col245 * carry_8_col245) - qm31_const::<1, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry is 0 or 1 or -1.
        let constraint_quotient = ((carry_9_col246
            * ((carry_9_col246 * carry_9_col246) - qm31_const::<1, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry is 0 or 1 or -1.
        let constraint_quotient = ((carry_10_col247
            * ((carry_10_col247 * carry_10_col247) - qm31_const::<1, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry is 0 or 1 or -1.
        let constraint_quotient = ((carry_11_col248
            * ((carry_11_col248 * carry_11_col248) - qm31_const::<1, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry is 0 or 1 or -1.
        let constraint_quotient = ((carry_12_col249
            * ((carry_12_col249 * carry_12_col249) - qm31_const::<1, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - carry is 0 or 1 or -1.
        let constraint_quotient = ((carry_13_col250
            * ((carry_13_col250 * carry_13_col250) - qm31_const::<1, 0, 0, 0>())))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        // Constraint - last carry needs to be 0.
        let constraint_quotient = (((carry_13_col250
            + (((a3_limb_9_col138 + b3_limb_9_col186) - c3_limb_9_col234)
                - (p3_limb_9_col47 * sub_p_bit_col236)))
            + (qm31_const::<512, 0, 0, 0>()
                * (((a3_limb_10_col139 + b3_limb_10_col187) - c3_limb_10_col235)
                    - (p3_limb_10_col48 * sub_p_bit_col236)))))
            * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        lookup_constraints(
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
            claimed_sum,
            column_size,
            ref interaction_trace_mask_values,
            memory_address_to_id_sum_0,
            memory_id_to_big_sum_1,
            memory_address_to_id_sum_2,
            memory_id_to_big_sum_3,
            memory_address_to_id_sum_4,
            memory_id_to_big_sum_5,
            memory_address_to_id_sum_6,
            memory_id_to_big_sum_7,
            memory_address_to_id_sum_8,
            memory_id_to_big_sum_9,
            memory_address_to_id_sum_10,
            memory_id_to_big_sum_11,
            memory_address_to_id_sum_12,
            memory_id_to_big_sum_13,
            memory_address_to_id_sum_14,
            memory_id_to_big_sum_15,
            memory_address_to_id_sum_16,
            memory_id_to_big_sum_17,
            memory_address_to_id_sum_18,
            memory_address_to_id_sum_19,
            memory_address_to_id_sum_20,
            memory_address_to_id_sum_21,
            memory_address_to_id_sum_22,
            memory_address_to_id_sum_23,
            memory_id_to_big_sum_24,
            memory_address_to_id_sum_25,
            memory_id_to_big_sum_26,
            memory_address_to_id_sum_27,
            memory_id_to_big_sum_28,
            memory_address_to_id_sum_29,
            memory_id_to_big_sum_30,
            memory_address_to_id_sum_31,
            memory_id_to_big_sum_32,
            memory_address_to_id_sum_33,
            memory_id_to_big_sum_34,
            memory_address_to_id_sum_35,
            memory_id_to_big_sum_36,
            memory_address_to_id_sum_37,
            memory_id_to_big_sum_38,
            memory_address_to_id_sum_39,
            memory_id_to_big_sum_40,
            memory_address_to_id_sum_41,
            memory_id_to_big_sum_42,
            memory_address_to_id_sum_43,
            memory_id_to_big_sum_44,
            memory_address_to_id_sum_45,
            memory_id_to_big_sum_46,
            memory_address_to_id_sum_47,
            memory_id_to_big_sum_48,
            memory_address_to_id_sum_49,
            memory_id_to_big_sum_50,
            memory_address_to_id_sum_51,
            memory_id_to_big_sum_52,
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
    memory_id_to_big_sum_1: QM31,
    memory_address_to_id_sum_2: QM31,
    memory_id_to_big_sum_3: QM31,
    memory_address_to_id_sum_4: QM31,
    memory_id_to_big_sum_5: QM31,
    memory_address_to_id_sum_6: QM31,
    memory_id_to_big_sum_7: QM31,
    memory_address_to_id_sum_8: QM31,
    memory_id_to_big_sum_9: QM31,
    memory_address_to_id_sum_10: QM31,
    memory_id_to_big_sum_11: QM31,
    memory_address_to_id_sum_12: QM31,
    memory_id_to_big_sum_13: QM31,
    memory_address_to_id_sum_14: QM31,
    memory_id_to_big_sum_15: QM31,
    memory_address_to_id_sum_16: QM31,
    memory_id_to_big_sum_17: QM31,
    memory_address_to_id_sum_18: QM31,
    memory_address_to_id_sum_19: QM31,
    memory_address_to_id_sum_20: QM31,
    memory_address_to_id_sum_21: QM31,
    memory_address_to_id_sum_22: QM31,
    memory_address_to_id_sum_23: QM31,
    memory_id_to_big_sum_24: QM31,
    memory_address_to_id_sum_25: QM31,
    memory_id_to_big_sum_26: QM31,
    memory_address_to_id_sum_27: QM31,
    memory_id_to_big_sum_28: QM31,
    memory_address_to_id_sum_29: QM31,
    memory_id_to_big_sum_30: QM31,
    memory_address_to_id_sum_31: QM31,
    memory_id_to_big_sum_32: QM31,
    memory_address_to_id_sum_33: QM31,
    memory_id_to_big_sum_34: QM31,
    memory_address_to_id_sum_35: QM31,
    memory_id_to_big_sum_36: QM31,
    memory_address_to_id_sum_37: QM31,
    memory_id_to_big_sum_38: QM31,
    memory_address_to_id_sum_39: QM31,
    memory_id_to_big_sum_40: QM31,
    memory_address_to_id_sum_41: QM31,
    memory_id_to_big_sum_42: QM31,
    memory_address_to_id_sum_43: QM31,
    memory_id_to_big_sum_44: QM31,
    memory_address_to_id_sum_45: QM31,
    memory_id_to_big_sum_46: QM31,
    memory_address_to_id_sum_47: QM31,
    memory_id_to_big_sum_48: QM31,
    memory_address_to_id_sum_49: QM31,
    memory_id_to_big_sum_50: QM31,
    memory_address_to_id_sum_51: QM31,
    memory_id_to_big_sum_52: QM31,
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
    ]: [Span<QM31>; 108] =
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
    let [trace_2_col104_neg1, trace_2_col104]: [QM31; 2] = (*trace_2_col104.try_into().unwrap())
        .unbox();
    let [trace_2_col105_neg1, trace_2_col105]: [QM31; 2] = (*trace_2_col105.try_into().unwrap())
        .unbox();
    let [trace_2_col106_neg1, trace_2_col106]: [QM31; 2] = (*trace_2_col106.try_into().unwrap())
        .unbox();
    let [trace_2_col107_neg1, trace_2_col107]: [QM31; 2] = (*trace_2_col107.try_into().unwrap())
        .unbox();

    core::internal::revoke_ap_tracking();

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3],
    ))
        * memory_address_to_id_sum_0
        * memory_id_to_big_sum_1)
        - memory_address_to_id_sum_0
        - memory_id_to_big_sum_1)
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
        * memory_address_to_id_sum_4
        * memory_id_to_big_sum_5)
        - memory_address_to_id_sum_4
        - memory_id_to_big_sum_5)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
    )
        - QM31Impl::from_partial_evals([trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11]))
        * memory_address_to_id_sum_6
        * memory_id_to_big_sum_7)
        - memory_address_to_id_sum_6
        - memory_id_to_big_sum_7)
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
        * memory_address_to_id_sum_10
        * memory_id_to_big_sum_11)
        - memory_address_to_id_sum_10
        - memory_id_to_big_sum_11)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23],
        ))
        * memory_address_to_id_sum_12
        * memory_id_to_big_sum_13)
        - memory_address_to_id_sum_12
        - memory_id_to_big_sum_13)
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
        * memory_address_to_id_sum_16
        * memory_id_to_big_sum_17)
        - memory_address_to_id_sum_16
        - memory_id_to_big_sum_17)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col36, trace_2_col37, trace_2_col38, trace_2_col39],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col32, trace_2_col33, trace_2_col34, trace_2_col35],
        ))
        * memory_address_to_id_sum_18
        * memory_address_to_id_sum_19)
        - memory_address_to_id_sum_18
        - memory_address_to_id_sum_19)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col40, trace_2_col41, trace_2_col42, trace_2_col43],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col36, trace_2_col37, trace_2_col38, trace_2_col39],
        ))
        * memory_address_to_id_sum_20
        * memory_address_to_id_sum_21)
        - memory_address_to_id_sum_20
        - memory_address_to_id_sum_21)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col44, trace_2_col45, trace_2_col46, trace_2_col47],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col40, trace_2_col41, trace_2_col42, trace_2_col43],
        ))
        * memory_address_to_id_sum_22
        * memory_address_to_id_sum_23)
        - memory_address_to_id_sum_22
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
        * memory_address_to_id_sum_25)
        - memory_id_to_big_sum_24
        - memory_address_to_id_sum_25)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col52, trace_2_col53, trace_2_col54, trace_2_col55],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col48, trace_2_col49, trace_2_col50, trace_2_col51],
        ))
        * memory_id_to_big_sum_26
        * memory_address_to_id_sum_27)
        - memory_id_to_big_sum_26
        - memory_address_to_id_sum_27)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col56, trace_2_col57, trace_2_col58, trace_2_col59],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col52, trace_2_col53, trace_2_col54, trace_2_col55],
        ))
        * memory_id_to_big_sum_28
        * memory_address_to_id_sum_29)
        - memory_id_to_big_sum_28
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
        * memory_address_to_id_sum_31)
        - memory_id_to_big_sum_30
        - memory_address_to_id_sum_31)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col64, trace_2_col65, trace_2_col66, trace_2_col67],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col60, trace_2_col61, trace_2_col62, trace_2_col63],
        ))
        * memory_id_to_big_sum_32
        * memory_address_to_id_sum_33)
        - memory_id_to_big_sum_32
        - memory_address_to_id_sum_33)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col68, trace_2_col69, trace_2_col70, trace_2_col71],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col64, trace_2_col65, trace_2_col66, trace_2_col67],
        ))
        * memory_id_to_big_sum_34
        * memory_address_to_id_sum_35)
        - memory_id_to_big_sum_34
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
        * memory_address_to_id_sum_37)
        - memory_id_to_big_sum_36
        - memory_address_to_id_sum_37)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col76, trace_2_col77, trace_2_col78, trace_2_col79],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col72, trace_2_col73, trace_2_col74, trace_2_col75],
        ))
        * memory_id_to_big_sum_38
        * memory_address_to_id_sum_39)
        - memory_id_to_big_sum_38
        - memory_address_to_id_sum_39)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col80, trace_2_col81, trace_2_col82, trace_2_col83],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col76, trace_2_col77, trace_2_col78, trace_2_col79],
        ))
        * memory_id_to_big_sum_40
        * memory_address_to_id_sum_41)
        - memory_id_to_big_sum_40
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
        * memory_address_to_id_sum_43)
        - memory_id_to_big_sum_42
        - memory_address_to_id_sum_43)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col88, trace_2_col89, trace_2_col90, trace_2_col91],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col84, trace_2_col85, trace_2_col86, trace_2_col87],
        ))
        * memory_id_to_big_sum_44
        * memory_address_to_id_sum_45)
        - memory_id_to_big_sum_44
        - memory_address_to_id_sum_45)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col92, trace_2_col93, trace_2_col94, trace_2_col95],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col88, trace_2_col89, trace_2_col90, trace_2_col91],
        ))
        * memory_id_to_big_sum_46
        * memory_address_to_id_sum_47)
        - memory_id_to_big_sum_46
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
        * memory_address_to_id_sum_49)
        - memory_id_to_big_sum_48
        - memory_address_to_id_sum_49)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col100, trace_2_col101, trace_2_col102, trace_2_col103],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col96, trace_2_col97, trace_2_col98, trace_2_col99],
        ))
        * memory_id_to_big_sum_50
        * memory_address_to_id_sum_51)
        - memory_id_to_big_sum_50
        - memory_address_to_id_sum_51)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col104, trace_2_col105, trace_2_col106, trace_2_col107],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col100, trace_2_col101, trace_2_col102, trace_2_col103],
        )
        - QM31Impl::from_partial_evals(
            [trace_2_col104_neg1, trace_2_col105_neg1, trace_2_col106_neg1, trace_2_col107_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * memory_id_to_big_sum_52)
        - qm31_const::<1, 0, 0, 0>())
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
}
