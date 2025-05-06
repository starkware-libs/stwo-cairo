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
use crate::components::subroutines::mod_utils::mod_utils_evaluate;
use crate::components::{CairoComponent, OPCODES_RELATION_SIZE, opcodes_sum};
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
        let interaction_log_sizes = ArrayImpl::new_repeated(QM31_EXTENSION_DEGREE * 27, log_size)
            .span();
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
        let memory_address_to_id_alphas = self
            .memory_address_to_id_lookup_elements
            .alpha_powers
            .span();
        let memory_address_to_id_z = *self.memory_address_to_id_lookup_elements.z;
        let memory_id_to_big_alphas = self.memory_id_to_big_lookup_elements.alpha_powers.span();
        let memory_id_to_big_z = *self.memory_id_to_big_lookup_elements.z;

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

        let output: [QM31; 448] = mod_utils_evaluate(
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
            memory_address_to_id_alphas,
            memory_address_to_id_z,
            memory_id_to_big_alphas,
            memory_id_to_big_z,
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
        let [
            mod_utils_output_tmp_c1b19_91_limb_0,
            mod_utils_output_tmp_c1b19_91_limb_1,
            mod_utils_output_tmp_c1b19_91_limb_2,
            mod_utils_output_tmp_c1b19_91_limb_3,
            mod_utils_output_tmp_c1b19_91_limb_4,
            mod_utils_output_tmp_c1b19_91_limb_5,
            mod_utils_output_tmp_c1b19_91_limb_6,
            mod_utils_output_tmp_c1b19_91_limb_7,
            mod_utils_output_tmp_c1b19_91_limb_8,
            mod_utils_output_tmp_c1b19_91_limb_9,
            mod_utils_output_tmp_c1b19_91_limb_10,
            mod_utils_output_tmp_c1b19_91_limb_11,
            mod_utils_output_tmp_c1b19_91_limb_12,
            mod_utils_output_tmp_c1b19_91_limb_13,
            mod_utils_output_tmp_c1b19_91_limb_14,
            mod_utils_output_tmp_c1b19_91_limb_15,
            mod_utils_output_tmp_c1b19_91_limb_16,
            mod_utils_output_tmp_c1b19_91_limb_17,
            mod_utils_output_tmp_c1b19_91_limb_18,
            mod_utils_output_tmp_c1b19_91_limb_19,
            mod_utils_output_tmp_c1b19_91_limb_20,
            mod_utils_output_tmp_c1b19_91_limb_21,
            mod_utils_output_tmp_c1b19_91_limb_22,
            mod_utils_output_tmp_c1b19_91_limb_23,
            mod_utils_output_tmp_c1b19_91_limb_24,
            mod_utils_output_tmp_c1b19_91_limb_25,
            mod_utils_output_tmp_c1b19_91_limb_26,
            mod_utils_output_tmp_c1b19_91_limb_27,
            mod_utils_output_tmp_c1b19_91_limb_28,
            mod_utils_output_tmp_c1b19_91_limb_29,
            mod_utils_output_tmp_c1b19_91_limb_30,
            mod_utils_output_tmp_c1b19_91_limb_31,
            mod_utils_output_tmp_c1b19_91_limb_32,
            mod_utils_output_tmp_c1b19_91_limb_33,
            mod_utils_output_tmp_c1b19_91_limb_34,
            mod_utils_output_tmp_c1b19_91_limb_35,
            mod_utils_output_tmp_c1b19_91_limb_36,
            mod_utils_output_tmp_c1b19_91_limb_37,
            mod_utils_output_tmp_c1b19_91_limb_38,
            mod_utils_output_tmp_c1b19_91_limb_39,
            mod_utils_output_tmp_c1b19_91_limb_40,
            mod_utils_output_tmp_c1b19_91_limb_41,
            mod_utils_output_tmp_c1b19_91_limb_42,
            mod_utils_output_tmp_c1b19_91_limb_43,
            mod_utils_output_tmp_c1b19_91_limb_44,
            mod_utils_output_tmp_c1b19_91_limb_45,
            mod_utils_output_tmp_c1b19_91_limb_46,
            mod_utils_output_tmp_c1b19_91_limb_47,
            mod_utils_output_tmp_c1b19_91_limb_48,
            mod_utils_output_tmp_c1b19_91_limb_49,
            mod_utils_output_tmp_c1b19_91_limb_50,
            mod_utils_output_tmp_c1b19_91_limb_51,
            mod_utils_output_tmp_c1b19_91_limb_52,
            mod_utils_output_tmp_c1b19_91_limb_53,
            mod_utils_output_tmp_c1b19_91_limb_54,
            mod_utils_output_tmp_c1b19_91_limb_55,
            mod_utils_output_tmp_c1b19_91_limb_56,
            mod_utils_output_tmp_c1b19_91_limb_57,
            mod_utils_output_tmp_c1b19_91_limb_58,
            mod_utils_output_tmp_c1b19_91_limb_59,
            mod_utils_output_tmp_c1b19_91_limb_60,
            mod_utils_output_tmp_c1b19_91_limb_61,
            mod_utils_output_tmp_c1b19_91_limb_62,
            mod_utils_output_tmp_c1b19_91_limb_63,
            mod_utils_output_tmp_c1b19_91_limb_64,
            mod_utils_output_tmp_c1b19_91_limb_65,
            mod_utils_output_tmp_c1b19_91_limb_66,
            mod_utils_output_tmp_c1b19_91_limb_67,
            mod_utils_output_tmp_c1b19_91_limb_68,
            mod_utils_output_tmp_c1b19_91_limb_69,
            mod_utils_output_tmp_c1b19_91_limb_70,
            mod_utils_output_tmp_c1b19_91_limb_71,
            mod_utils_output_tmp_c1b19_91_limb_72,
            mod_utils_output_tmp_c1b19_91_limb_73,
            mod_utils_output_tmp_c1b19_91_limb_74,
            mod_utils_output_tmp_c1b19_91_limb_75,
            mod_utils_output_tmp_c1b19_91_limb_76,
            mod_utils_output_tmp_c1b19_91_limb_77,
            mod_utils_output_tmp_c1b19_91_limb_78,
            mod_utils_output_tmp_c1b19_91_limb_79,
            mod_utils_output_tmp_c1b19_91_limb_80,
            mod_utils_output_tmp_c1b19_91_limb_81,
            mod_utils_output_tmp_c1b19_91_limb_82,
            mod_utils_output_tmp_c1b19_91_limb_83,
            mod_utils_output_tmp_c1b19_91_limb_84,
            mod_utils_output_tmp_c1b19_91_limb_85,
            mod_utils_output_tmp_c1b19_91_limb_86,
            mod_utils_output_tmp_c1b19_91_limb_87,
            mod_utils_output_tmp_c1b19_91_limb_88,
            mod_utils_output_tmp_c1b19_91_limb_89,
            mod_utils_output_tmp_c1b19_91_limb_90,
            mod_utils_output_tmp_c1b19_91_limb_91,
            mod_utils_output_tmp_c1b19_91_limb_92,
            mod_utils_output_tmp_c1b19_91_limb_93,
            mod_utils_output_tmp_c1b19_91_limb_94,
            mod_utils_output_tmp_c1b19_91_limb_95,
            mod_utils_output_tmp_c1b19_91_limb_96,
            mod_utils_output_tmp_c1b19_91_limb_97,
            mod_utils_output_tmp_c1b19_91_limb_98,
            mod_utils_output_tmp_c1b19_91_limb_99,
            mod_utils_output_tmp_c1b19_91_limb_100,
            mod_utils_output_tmp_c1b19_91_limb_101,
            mod_utils_output_tmp_c1b19_91_limb_102,
            mod_utils_output_tmp_c1b19_91_limb_103,
            mod_utils_output_tmp_c1b19_91_limb_104,
            mod_utils_output_tmp_c1b19_91_limb_105,
            mod_utils_output_tmp_c1b19_91_limb_106,
            mod_utils_output_tmp_c1b19_91_limb_107,
            mod_utils_output_tmp_c1b19_91_limb_108,
            mod_utils_output_tmp_c1b19_91_limb_109,
            mod_utils_output_tmp_c1b19_91_limb_110,
            mod_utils_output_tmp_c1b19_91_limb_111,
            mod_utils_output_tmp_c1b19_91_limb_112,
            mod_utils_output_tmp_c1b19_91_limb_113,
            mod_utils_output_tmp_c1b19_91_limb_114,
            mod_utils_output_tmp_c1b19_91_limb_115,
            mod_utils_output_tmp_c1b19_91_limb_116,
            mod_utils_output_tmp_c1b19_91_limb_117,
            mod_utils_output_tmp_c1b19_91_limb_118,
            mod_utils_output_tmp_c1b19_91_limb_119,
            mod_utils_output_tmp_c1b19_91_limb_120,
            mod_utils_output_tmp_c1b19_91_limb_121,
            mod_utils_output_tmp_c1b19_91_limb_122,
            mod_utils_output_tmp_c1b19_91_limb_123,
            mod_utils_output_tmp_c1b19_91_limb_124,
            mod_utils_output_tmp_c1b19_91_limb_125,
            mod_utils_output_tmp_c1b19_91_limb_126,
            mod_utils_output_tmp_c1b19_91_limb_127,
            mod_utils_output_tmp_c1b19_91_limb_128,
            mod_utils_output_tmp_c1b19_91_limb_129,
            mod_utils_output_tmp_c1b19_91_limb_130,
            mod_utils_output_tmp_c1b19_91_limb_131,
            mod_utils_output_tmp_c1b19_91_limb_132,
            mod_utils_output_tmp_c1b19_91_limb_133,
            mod_utils_output_tmp_c1b19_91_limb_134,
            mod_utils_output_tmp_c1b19_91_limb_135,
            mod_utils_output_tmp_c1b19_91_limb_136,
            mod_utils_output_tmp_c1b19_91_limb_137,
            mod_utils_output_tmp_c1b19_91_limb_138,
            mod_utils_output_tmp_c1b19_91_limb_139,
            mod_utils_output_tmp_c1b19_91_limb_140,
            mod_utils_output_tmp_c1b19_91_limb_141,
            mod_utils_output_tmp_c1b19_91_limb_142,
            mod_utils_output_tmp_c1b19_91_limb_143,
            mod_utils_output_tmp_c1b19_91_limb_144,
            mod_utils_output_tmp_c1b19_91_limb_145,
            mod_utils_output_tmp_c1b19_91_limb_146,
            mod_utils_output_tmp_c1b19_91_limb_147,
            mod_utils_output_tmp_c1b19_91_limb_148,
            mod_utils_output_tmp_c1b19_91_limb_149,
            mod_utils_output_tmp_c1b19_91_limb_150,
            mod_utils_output_tmp_c1b19_91_limb_151,
            mod_utils_output_tmp_c1b19_91_limb_152,
            mod_utils_output_tmp_c1b19_91_limb_153,
            mod_utils_output_tmp_c1b19_91_limb_154,
            mod_utils_output_tmp_c1b19_91_limb_155,
            mod_utils_output_tmp_c1b19_91_limb_156,
            mod_utils_output_tmp_c1b19_91_limb_157,
            mod_utils_output_tmp_c1b19_91_limb_158,
            mod_utils_output_tmp_c1b19_91_limb_159,
            mod_utils_output_tmp_c1b19_91_limb_160,
            mod_utils_output_tmp_c1b19_91_limb_161,
            mod_utils_output_tmp_c1b19_91_limb_162,
            mod_utils_output_tmp_c1b19_91_limb_163,
            mod_utils_output_tmp_c1b19_91_limb_164,
            mod_utils_output_tmp_c1b19_91_limb_165,
            mod_utils_output_tmp_c1b19_91_limb_166,
            mod_utils_output_tmp_c1b19_91_limb_167,
            mod_utils_output_tmp_c1b19_91_limb_168,
            mod_utils_output_tmp_c1b19_91_limb_169,
            mod_utils_output_tmp_c1b19_91_limb_170,
            mod_utils_output_tmp_c1b19_91_limb_171,
            mod_utils_output_tmp_c1b19_91_limb_172,
            mod_utils_output_tmp_c1b19_91_limb_173,
            mod_utils_output_tmp_c1b19_91_limb_174,
            mod_utils_output_tmp_c1b19_91_limb_175,
            mod_utils_output_tmp_c1b19_91_limb_176,
            mod_utils_output_tmp_c1b19_91_limb_177,
            mod_utils_output_tmp_c1b19_91_limb_178,
            mod_utils_output_tmp_c1b19_91_limb_179,
            mod_utils_output_tmp_c1b19_91_limb_180,
            mod_utils_output_tmp_c1b19_91_limb_181,
            mod_utils_output_tmp_c1b19_91_limb_182,
            mod_utils_output_tmp_c1b19_91_limb_183,
            mod_utils_output_tmp_c1b19_91_limb_184,
            mod_utils_output_tmp_c1b19_91_limb_185,
            mod_utils_output_tmp_c1b19_91_limb_186,
            mod_utils_output_tmp_c1b19_91_limb_187,
            mod_utils_output_tmp_c1b19_91_limb_188,
            mod_utils_output_tmp_c1b19_91_limb_189,
            mod_utils_output_tmp_c1b19_91_limb_190,
            mod_utils_output_tmp_c1b19_91_limb_191,
            mod_utils_output_tmp_c1b19_91_limb_192,
            mod_utils_output_tmp_c1b19_91_limb_193,
            mod_utils_output_tmp_c1b19_91_limb_194,
            mod_utils_output_tmp_c1b19_91_limb_195,
            mod_utils_output_tmp_c1b19_91_limb_196,
            mod_utils_output_tmp_c1b19_91_limb_197,
            mod_utils_output_tmp_c1b19_91_limb_198,
            mod_utils_output_tmp_c1b19_91_limb_199,
            mod_utils_output_tmp_c1b19_91_limb_200,
            mod_utils_output_tmp_c1b19_91_limb_201,
            mod_utils_output_tmp_c1b19_91_limb_202,
            mod_utils_output_tmp_c1b19_91_limb_203,
            mod_utils_output_tmp_c1b19_91_limb_204,
            mod_utils_output_tmp_c1b19_91_limb_205,
            mod_utils_output_tmp_c1b19_91_limb_206,
            mod_utils_output_tmp_c1b19_91_limb_207,
            mod_utils_output_tmp_c1b19_91_limb_208,
            mod_utils_output_tmp_c1b19_91_limb_209,
            mod_utils_output_tmp_c1b19_91_limb_210,
            mod_utils_output_tmp_c1b19_91_limb_211,
            mod_utils_output_tmp_c1b19_91_limb_212,
            mod_utils_output_tmp_c1b19_91_limb_213,
            mod_utils_output_tmp_c1b19_91_limb_214,
            mod_utils_output_tmp_c1b19_91_limb_215,
            mod_utils_output_tmp_c1b19_91_limb_216,
            mod_utils_output_tmp_c1b19_91_limb_217,
            mod_utils_output_tmp_c1b19_91_limb_218,
            mod_utils_output_tmp_c1b19_91_limb_219,
            mod_utils_output_tmp_c1b19_91_limb_220,
            mod_utils_output_tmp_c1b19_91_limb_221,
            mod_utils_output_tmp_c1b19_91_limb_222,
            mod_utils_output_tmp_c1b19_91_limb_223,
            mod_utils_output_tmp_c1b19_91_limb_224,
            mod_utils_output_tmp_c1b19_91_limb_225,
            mod_utils_output_tmp_c1b19_91_limb_226,
            mod_utils_output_tmp_c1b19_91_limb_227,
            mod_utils_output_tmp_c1b19_91_limb_228,
            mod_utils_output_tmp_c1b19_91_limb_229,
            mod_utils_output_tmp_c1b19_91_limb_230,
            mod_utils_output_tmp_c1b19_91_limb_231,
            mod_utils_output_tmp_c1b19_91_limb_232,
            mod_utils_output_tmp_c1b19_91_limb_233,
            mod_utils_output_tmp_c1b19_91_limb_234,
            mod_utils_output_tmp_c1b19_91_limb_235,
            mod_utils_output_tmp_c1b19_91_limb_236,
            mod_utils_output_tmp_c1b19_91_limb_237,
            mod_utils_output_tmp_c1b19_91_limb_238,
            mod_utils_output_tmp_c1b19_91_limb_239,
            mod_utils_output_tmp_c1b19_91_limb_240,
            mod_utils_output_tmp_c1b19_91_limb_241,
            mod_utils_output_tmp_c1b19_91_limb_242,
            mod_utils_output_tmp_c1b19_91_limb_243,
            mod_utils_output_tmp_c1b19_91_limb_244,
            mod_utils_output_tmp_c1b19_91_limb_245,
            mod_utils_output_tmp_c1b19_91_limb_246,
            mod_utils_output_tmp_c1b19_91_limb_247,
            mod_utils_output_tmp_c1b19_91_limb_248,
            mod_utils_output_tmp_c1b19_91_limb_249,
            mod_utils_output_tmp_c1b19_91_limb_250,
            mod_utils_output_tmp_c1b19_91_limb_251,
            mod_utils_output_tmp_c1b19_91_limb_252,
            mod_utils_output_tmp_c1b19_91_limb_253,
            mod_utils_output_tmp_c1b19_91_limb_254,
            mod_utils_output_tmp_c1b19_91_limb_255,
            mod_utils_output_tmp_c1b19_91_limb_256,
            mod_utils_output_tmp_c1b19_91_limb_257,
            mod_utils_output_tmp_c1b19_91_limb_258,
            mod_utils_output_tmp_c1b19_91_limb_259,
            mod_utils_output_tmp_c1b19_91_limb_260,
            mod_utils_output_tmp_c1b19_91_limb_261,
            mod_utils_output_tmp_c1b19_91_limb_262,
            mod_utils_output_tmp_c1b19_91_limb_263,
            mod_utils_output_tmp_c1b19_91_limb_264,
            mod_utils_output_tmp_c1b19_91_limb_265,
            mod_utils_output_tmp_c1b19_91_limb_266,
            mod_utils_output_tmp_c1b19_91_limb_267,
            mod_utils_output_tmp_c1b19_91_limb_268,
            mod_utils_output_tmp_c1b19_91_limb_269,
            mod_utils_output_tmp_c1b19_91_limb_270,
            mod_utils_output_tmp_c1b19_91_limb_271,
            mod_utils_output_tmp_c1b19_91_limb_272,
            mod_utils_output_tmp_c1b19_91_limb_273,
            mod_utils_output_tmp_c1b19_91_limb_274,
            mod_utils_output_tmp_c1b19_91_limb_275,
            mod_utils_output_tmp_c1b19_91_limb_276,
            mod_utils_output_tmp_c1b19_91_limb_277,
            mod_utils_output_tmp_c1b19_91_limb_278,
            mod_utils_output_tmp_c1b19_91_limb_279,
            mod_utils_output_tmp_c1b19_91_limb_280,
            mod_utils_output_tmp_c1b19_91_limb_281,
            mod_utils_output_tmp_c1b19_91_limb_282,
            mod_utils_output_tmp_c1b19_91_limb_283,
            mod_utils_output_tmp_c1b19_91_limb_284,
            mod_utils_output_tmp_c1b19_91_limb_285,
            mod_utils_output_tmp_c1b19_91_limb_286,
            mod_utils_output_tmp_c1b19_91_limb_287,
            mod_utils_output_tmp_c1b19_91_limb_288,
            mod_utils_output_tmp_c1b19_91_limb_289,
            mod_utils_output_tmp_c1b19_91_limb_290,
            mod_utils_output_tmp_c1b19_91_limb_291,
            mod_utils_output_tmp_c1b19_91_limb_292,
            mod_utils_output_tmp_c1b19_91_limb_293,
            mod_utils_output_tmp_c1b19_91_limb_294,
            mod_utils_output_tmp_c1b19_91_limb_295,
            mod_utils_output_tmp_c1b19_91_limb_296,
            mod_utils_output_tmp_c1b19_91_limb_297,
            mod_utils_output_tmp_c1b19_91_limb_298,
            mod_utils_output_tmp_c1b19_91_limb_299,
            mod_utils_output_tmp_c1b19_91_limb_300,
            mod_utils_output_tmp_c1b19_91_limb_301,
            mod_utils_output_tmp_c1b19_91_limb_302,
            mod_utils_output_tmp_c1b19_91_limb_303,
            mod_utils_output_tmp_c1b19_91_limb_304,
            mod_utils_output_tmp_c1b19_91_limb_305,
            mod_utils_output_tmp_c1b19_91_limb_306,
            mod_utils_output_tmp_c1b19_91_limb_307,
            mod_utils_output_tmp_c1b19_91_limb_308,
            mod_utils_output_tmp_c1b19_91_limb_309,
            mod_utils_output_tmp_c1b19_91_limb_310,
            mod_utils_output_tmp_c1b19_91_limb_311,
            mod_utils_output_tmp_c1b19_91_limb_312,
            mod_utils_output_tmp_c1b19_91_limb_313,
            mod_utils_output_tmp_c1b19_91_limb_314,
            mod_utils_output_tmp_c1b19_91_limb_315,
            mod_utils_output_tmp_c1b19_91_limb_316,
            mod_utils_output_tmp_c1b19_91_limb_317,
            mod_utils_output_tmp_c1b19_91_limb_318,
            mod_utils_output_tmp_c1b19_91_limb_319,
            mod_utils_output_tmp_c1b19_91_limb_320,
            mod_utils_output_tmp_c1b19_91_limb_321,
            mod_utils_output_tmp_c1b19_91_limb_322,
            mod_utils_output_tmp_c1b19_91_limb_323,
            mod_utils_output_tmp_c1b19_91_limb_324,
            mod_utils_output_tmp_c1b19_91_limb_325,
            mod_utils_output_tmp_c1b19_91_limb_326,
            mod_utils_output_tmp_c1b19_91_limb_327,
            mod_utils_output_tmp_c1b19_91_limb_328,
            mod_utils_output_tmp_c1b19_91_limb_329,
            mod_utils_output_tmp_c1b19_91_limb_330,
            mod_utils_output_tmp_c1b19_91_limb_331,
            mod_utils_output_tmp_c1b19_91_limb_332,
            mod_utils_output_tmp_c1b19_91_limb_333,
            mod_utils_output_tmp_c1b19_91_limb_334,
            mod_utils_output_tmp_c1b19_91_limb_335,
            mod_utils_output_tmp_c1b19_91_limb_336,
            mod_utils_output_tmp_c1b19_91_limb_337,
            mod_utils_output_tmp_c1b19_91_limb_338,
            mod_utils_output_tmp_c1b19_91_limb_339,
            mod_utils_output_tmp_c1b19_91_limb_340,
            mod_utils_output_tmp_c1b19_91_limb_341,
            mod_utils_output_tmp_c1b19_91_limb_342,
            mod_utils_output_tmp_c1b19_91_limb_343,
            mod_utils_output_tmp_c1b19_91_limb_344,
            mod_utils_output_tmp_c1b19_91_limb_345,
            mod_utils_output_tmp_c1b19_91_limb_346,
            mod_utils_output_tmp_c1b19_91_limb_347,
            mod_utils_output_tmp_c1b19_91_limb_348,
            mod_utils_output_tmp_c1b19_91_limb_349,
            mod_utils_output_tmp_c1b19_91_limb_350,
            mod_utils_output_tmp_c1b19_91_limb_351,
            mod_utils_output_tmp_c1b19_91_limb_352,
            mod_utils_output_tmp_c1b19_91_limb_353,
            mod_utils_output_tmp_c1b19_91_limb_354,
            mod_utils_output_tmp_c1b19_91_limb_355,
            mod_utils_output_tmp_c1b19_91_limb_356,
            mod_utils_output_tmp_c1b19_91_limb_357,
            mod_utils_output_tmp_c1b19_91_limb_358,
            mod_utils_output_tmp_c1b19_91_limb_359,
            mod_utils_output_tmp_c1b19_91_limb_360,
            mod_utils_output_tmp_c1b19_91_limb_361,
            mod_utils_output_tmp_c1b19_91_limb_362,
            mod_utils_output_tmp_c1b19_91_limb_363,
            mod_utils_output_tmp_c1b19_91_limb_364,
            mod_utils_output_tmp_c1b19_91_limb_365,
            mod_utils_output_tmp_c1b19_91_limb_366,
            mod_utils_output_tmp_c1b19_91_limb_367,
            mod_utils_output_tmp_c1b19_91_limb_368,
            mod_utils_output_tmp_c1b19_91_limb_369,
            mod_utils_output_tmp_c1b19_91_limb_370,
            mod_utils_output_tmp_c1b19_91_limb_371,
            mod_utils_output_tmp_c1b19_91_limb_372,
            mod_utils_output_tmp_c1b19_91_limb_373,
            mod_utils_output_tmp_c1b19_91_limb_374,
            mod_utils_output_tmp_c1b19_91_limb_375,
            mod_utils_output_tmp_c1b19_91_limb_376,
            mod_utils_output_tmp_c1b19_91_limb_377,
            mod_utils_output_tmp_c1b19_91_limb_378,
            mod_utils_output_tmp_c1b19_91_limb_379,
            mod_utils_output_tmp_c1b19_91_limb_380,
            mod_utils_output_tmp_c1b19_91_limb_381,
            mod_utils_output_tmp_c1b19_91_limb_382,
            mod_utils_output_tmp_c1b19_91_limb_383,
            mod_utils_output_tmp_c1b19_91_limb_384,
            mod_utils_output_tmp_c1b19_91_limb_385,
            mod_utils_output_tmp_c1b19_91_limb_386,
            mod_utils_output_tmp_c1b19_91_limb_387,
            mod_utils_output_tmp_c1b19_91_limb_388,
            mod_utils_output_tmp_c1b19_91_limb_389,
            mod_utils_output_tmp_c1b19_91_limb_390,
            mod_utils_output_tmp_c1b19_91_limb_391,
            mod_utils_output_tmp_c1b19_91_limb_392,
            mod_utils_output_tmp_c1b19_91_limb_393,
            mod_utils_output_tmp_c1b19_91_limb_394,
            mod_utils_output_tmp_c1b19_91_limb_395,
            mod_utils_output_tmp_c1b19_91_limb_396,
            mod_utils_output_tmp_c1b19_91_limb_397,
            mod_utils_output_tmp_c1b19_91_limb_398,
            mod_utils_output_tmp_c1b19_91_limb_399,
            mod_utils_output_tmp_c1b19_91_limb_400,
            mod_utils_output_tmp_c1b19_91_limb_401,
            mod_utils_output_tmp_c1b19_91_limb_402,
            mod_utils_output_tmp_c1b19_91_limb_403,
            mod_utils_output_tmp_c1b19_91_limb_404,
            mod_utils_output_tmp_c1b19_91_limb_405,
            mod_utils_output_tmp_c1b19_91_limb_406,
            mod_utils_output_tmp_c1b19_91_limb_407,
            mod_utils_output_tmp_c1b19_91_limb_408,
            mod_utils_output_tmp_c1b19_91_limb_409,
            mod_utils_output_tmp_c1b19_91_limb_410,
            mod_utils_output_tmp_c1b19_91_limb_411,
            mod_utils_output_tmp_c1b19_91_limb_412,
            mod_utils_output_tmp_c1b19_91_limb_413,
            mod_utils_output_tmp_c1b19_91_limb_414,
            mod_utils_output_tmp_c1b19_91_limb_415,
            mod_utils_output_tmp_c1b19_91_limb_416,
            mod_utils_output_tmp_c1b19_91_limb_417,
            mod_utils_output_tmp_c1b19_91_limb_418,
            mod_utils_output_tmp_c1b19_91_limb_419,
            mod_utils_output_tmp_c1b19_91_limb_420,
            mod_utils_output_tmp_c1b19_91_limb_421,
            mod_utils_output_tmp_c1b19_91_limb_422,
            mod_utils_output_tmp_c1b19_91_limb_423,
            mod_utils_output_tmp_c1b19_91_limb_424,
            mod_utils_output_tmp_c1b19_91_limb_425,
            mod_utils_output_tmp_c1b19_91_limb_426,
            mod_utils_output_tmp_c1b19_91_limb_427,
            mod_utils_output_tmp_c1b19_91_limb_428,
            mod_utils_output_tmp_c1b19_91_limb_429,
            mod_utils_output_tmp_c1b19_91_limb_430,
            mod_utils_output_tmp_c1b19_91_limb_431,
            mod_utils_output_tmp_c1b19_91_limb_432,
            mod_utils_output_tmp_c1b19_91_limb_433,
            mod_utils_output_tmp_c1b19_91_limb_434,
            mod_utils_output_tmp_c1b19_91_limb_435,
            mod_utils_output_tmp_c1b19_91_limb_436,
            mod_utils_output_tmp_c1b19_91_limb_437,
            mod_utils_output_tmp_c1b19_91_limb_438,
            mod_utils_output_tmp_c1b19_91_limb_439,
            mod_utils_output_tmp_c1b19_91_limb_440,
            mod_utils_output_tmp_c1b19_91_limb_441,
            mod_utils_output_tmp_c1b19_91_limb_442,
            mod_utils_output_tmp_c1b19_91_limb_443,
            mod_utils_output_tmp_c1b19_91_limb_444,
            mod_utils_output_tmp_c1b19_91_limb_445,
            mod_utils_output_tmp_c1b19_91_limb_446,
            mod_utils_output_tmp_c1b19_91_limb_447,
        ] =
            output;

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
