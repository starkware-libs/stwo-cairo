// Constraints version: 9c495569

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
use crate::components::range_check_19::{RANGE_CHECK_19_RELATION_SIZE, range_check_19_sum};
use crate::components::range_check_9_9::{RANGE_CHECK_9_9_RELATION_SIZE, range_check_9_9_sum};
use crate::components::subroutines::felt_252_unpack_from_27_range_check_output::felt_252_unpack_from_27_range_check_output_evaluate;
use crate::components::subroutines::mul_252::mul_252_evaluate;
use crate::components::{CairoComponent, OPCODES_RELATION_SIZE, opcodes_sum};
use crate::utils::U32Impl;


pub const N_TRACE_COLUMNS: usize = 141;
pub const CUBE_252_RELATION_SIZE: usize = 20;


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
        let interaction_log_sizes = ArrayImpl::new_repeated(QM31_EXTENSION_DEGREE * 50, log_size)
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
    pub range_check_9_9_lookup_elements: crate::RangeCheck_9_9Elements,
    pub range_check_19_lookup_elements: crate::RangeCheck_19Elements,
    pub cube_252_lookup_elements: crate::Cube252Elements,
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
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
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
        let mut range_check_9_9_sum_0: QM31 = Zero::zero();
        let mut range_check_9_9_sum_1: QM31 = Zero::zero();
        let mut range_check_9_9_sum_2: QM31 = Zero::zero();
        let mut range_check_9_9_sum_3: QM31 = Zero::zero();
        let mut range_check_9_9_sum_4: QM31 = Zero::zero();
        let mut range_check_9_9_sum_5: QM31 = Zero::zero();
        let mut range_check_9_9_sum_6: QM31 = Zero::zero();
        let mut range_check_9_9_sum_7: QM31 = Zero::zero();
        let mut range_check_9_9_sum_8: QM31 = Zero::zero();
        let mut range_check_9_9_sum_9: QM31 = Zero::zero();
        let mut range_check_9_9_sum_10: QM31 = Zero::zero();
        let mut range_check_9_9_sum_11: QM31 = Zero::zero();
        let mut range_check_9_9_sum_12: QM31 = Zero::zero();
        let mut range_check_9_9_sum_13: QM31 = Zero::zero();
        let mut range_check_9_9_sum_14: QM31 = Zero::zero();
        let mut range_check_9_9_sum_15: QM31 = Zero::zero();
        let mut range_check_9_9_sum_16: QM31 = Zero::zero();
        let mut range_check_9_9_sum_17: QM31 = Zero::zero();
        let mut range_check_9_9_sum_18: QM31 = Zero::zero();
        let mut range_check_9_9_sum_19: QM31 = Zero::zero();
        let mut range_check_9_9_sum_20: QM31 = Zero::zero();
        let mut range_check_9_9_sum_21: QM31 = Zero::zero();
        let mut range_check_9_9_sum_22: QM31 = Zero::zero();
        let mut range_check_9_9_sum_23: QM31 = Zero::zero();
        let mut range_check_9_9_sum_24: QM31 = Zero::zero();
        let mut range_check_9_9_sum_25: QM31 = Zero::zero();
        let mut range_check_9_9_sum_26: QM31 = Zero::zero();
        let mut range_check_9_9_sum_27: QM31 = Zero::zero();
        let mut range_check_19_sum_28: QM31 = Zero::zero();
        let mut range_check_19_sum_29: QM31 = Zero::zero();
        let mut range_check_19_sum_30: QM31 = Zero::zero();
        let mut range_check_19_sum_31: QM31 = Zero::zero();
        let mut range_check_19_sum_32: QM31 = Zero::zero();
        let mut range_check_19_sum_33: QM31 = Zero::zero();
        let mut range_check_19_sum_34: QM31 = Zero::zero();
        let mut range_check_19_sum_35: QM31 = Zero::zero();
        let mut range_check_19_sum_36: QM31 = Zero::zero();
        let mut range_check_19_sum_37: QM31 = Zero::zero();
        let mut range_check_19_sum_38: QM31 = Zero::zero();
        let mut range_check_19_sum_39: QM31 = Zero::zero();
        let mut range_check_19_sum_40: QM31 = Zero::zero();
        let mut range_check_19_sum_41: QM31 = Zero::zero();
        let mut range_check_19_sum_42: QM31 = Zero::zero();
        let mut range_check_19_sum_43: QM31 = Zero::zero();
        let mut range_check_19_sum_44: QM31 = Zero::zero();
        let mut range_check_19_sum_45: QM31 = Zero::zero();
        let mut range_check_19_sum_46: QM31 = Zero::zero();
        let mut range_check_19_sum_47: QM31 = Zero::zero();
        let mut range_check_19_sum_48: QM31 = Zero::zero();
        let mut range_check_19_sum_49: QM31 = Zero::zero();
        let mut range_check_19_sum_50: QM31 = Zero::zero();
        let mut range_check_19_sum_51: QM31 = Zero::zero();
        let mut range_check_19_sum_52: QM31 = Zero::zero();
        let mut range_check_19_sum_53: QM31 = Zero::zero();
        let mut range_check_19_sum_54: QM31 = Zero::zero();
        let mut range_check_19_sum_55: QM31 = Zero::zero();
        let mut range_check_9_9_sum_56: QM31 = Zero::zero();
        let mut range_check_9_9_sum_57: QM31 = Zero::zero();
        let mut range_check_9_9_sum_58: QM31 = Zero::zero();
        let mut range_check_9_9_sum_59: QM31 = Zero::zero();
        let mut range_check_9_9_sum_60: QM31 = Zero::zero();
        let mut range_check_9_9_sum_61: QM31 = Zero::zero();
        let mut range_check_9_9_sum_62: QM31 = Zero::zero();
        let mut range_check_9_9_sum_63: QM31 = Zero::zero();
        let mut range_check_9_9_sum_64: QM31 = Zero::zero();
        let mut range_check_9_9_sum_65: QM31 = Zero::zero();
        let mut range_check_9_9_sum_66: QM31 = Zero::zero();
        let mut range_check_9_9_sum_67: QM31 = Zero::zero();
        let mut range_check_9_9_sum_68: QM31 = Zero::zero();
        let mut range_check_9_9_sum_69: QM31 = Zero::zero();
        let mut range_check_19_sum_70: QM31 = Zero::zero();
        let mut range_check_19_sum_71: QM31 = Zero::zero();
        let mut range_check_19_sum_72: QM31 = Zero::zero();
        let mut range_check_19_sum_73: QM31 = Zero::zero();
        let mut range_check_19_sum_74: QM31 = Zero::zero();
        let mut range_check_19_sum_75: QM31 = Zero::zero();
        let mut range_check_19_sum_76: QM31 = Zero::zero();
        let mut range_check_19_sum_77: QM31 = Zero::zero();
        let mut range_check_19_sum_78: QM31 = Zero::zero();
        let mut range_check_19_sum_79: QM31 = Zero::zero();
        let mut range_check_19_sum_80: QM31 = Zero::zero();
        let mut range_check_19_sum_81: QM31 = Zero::zero();
        let mut range_check_19_sum_82: QM31 = Zero::zero();
        let mut range_check_19_sum_83: QM31 = Zero::zero();
        let mut range_check_19_sum_84: QM31 = Zero::zero();
        let mut range_check_19_sum_85: QM31 = Zero::zero();
        let mut range_check_19_sum_86: QM31 = Zero::zero();
        let mut range_check_19_sum_87: QM31 = Zero::zero();
        let mut range_check_19_sum_88: QM31 = Zero::zero();
        let mut range_check_19_sum_89: QM31 = Zero::zero();
        let mut range_check_19_sum_90: QM31 = Zero::zero();
        let mut range_check_19_sum_91: QM31 = Zero::zero();
        let mut range_check_19_sum_92: QM31 = Zero::zero();
        let mut range_check_19_sum_93: QM31 = Zero::zero();
        let mut range_check_19_sum_94: QM31 = Zero::zero();
        let mut range_check_19_sum_95: QM31 = Zero::zero();
        let mut range_check_19_sum_96: QM31 = Zero::zero();
        let mut range_check_19_sum_97: QM31 = Zero::zero();
        let mut cube_252_sum_98: QM31 = Zero::zero();
        let range_check_9_9_alphas = self.range_check_9_9_lookup_elements.alpha_powers.span();
        let range_check_9_9_z = *self.range_check_9_9_lookup_elements.z;
        let range_check_19_alphas = self.range_check_19_lookup_elements.alpha_powers.span();
        let range_check_19_z = *self.range_check_19_lookup_elements.z;
        let cube_252_alphas = self.cube_252_lookup_elements.alpha_powers.span();
        let cube_252_z = *self.cube_252_lookup_elements.z;

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
            unpacked_limb_0_col10,
            unpacked_limb_1_col11,
            unpacked_limb_3_col12,
            unpacked_limb_4_col13,
            unpacked_limb_6_col14,
            unpacked_limb_7_col15,
            unpacked_limb_9_col16,
            unpacked_limb_10_col17,
            unpacked_limb_12_col18,
            unpacked_limb_13_col19,
            unpacked_limb_15_col20,
            unpacked_limb_16_col21,
            unpacked_limb_18_col22,
            unpacked_limb_19_col23,
            unpacked_limb_21_col24,
            unpacked_limb_22_col25,
            unpacked_limb_24_col26,
            unpacked_limb_25_col27,
            mul_res_limb_0_col28,
            mul_res_limb_1_col29,
            mul_res_limb_2_col30,
            mul_res_limb_3_col31,
            mul_res_limb_4_col32,
            mul_res_limb_5_col33,
            mul_res_limb_6_col34,
            mul_res_limb_7_col35,
            mul_res_limb_8_col36,
            mul_res_limb_9_col37,
            mul_res_limb_10_col38,
            mul_res_limb_11_col39,
            mul_res_limb_12_col40,
            mul_res_limb_13_col41,
            mul_res_limb_14_col42,
            mul_res_limb_15_col43,
            mul_res_limb_16_col44,
            mul_res_limb_17_col45,
            mul_res_limb_18_col46,
            mul_res_limb_19_col47,
            mul_res_limb_20_col48,
            mul_res_limb_21_col49,
            mul_res_limb_22_col50,
            mul_res_limb_23_col51,
            mul_res_limb_24_col52,
            mul_res_limb_25_col53,
            mul_res_limb_26_col54,
            mul_res_limb_27_col55,
            k_col56,
            carry_0_col57,
            carry_1_col58,
            carry_2_col59,
            carry_3_col60,
            carry_4_col61,
            carry_5_col62,
            carry_6_col63,
            carry_7_col64,
            carry_8_col65,
            carry_9_col66,
            carry_10_col67,
            carry_11_col68,
            carry_12_col69,
            carry_13_col70,
            carry_14_col71,
            carry_15_col72,
            carry_16_col73,
            carry_17_col74,
            carry_18_col75,
            carry_19_col76,
            carry_20_col77,
            carry_21_col78,
            carry_22_col79,
            carry_23_col80,
            carry_24_col81,
            carry_25_col82,
            carry_26_col83,
            mul_res_limb_0_col84,
            mul_res_limb_1_col85,
            mul_res_limb_2_col86,
            mul_res_limb_3_col87,
            mul_res_limb_4_col88,
            mul_res_limb_5_col89,
            mul_res_limb_6_col90,
            mul_res_limb_7_col91,
            mul_res_limb_8_col92,
            mul_res_limb_9_col93,
            mul_res_limb_10_col94,
            mul_res_limb_11_col95,
            mul_res_limb_12_col96,
            mul_res_limb_13_col97,
            mul_res_limb_14_col98,
            mul_res_limb_15_col99,
            mul_res_limb_16_col100,
            mul_res_limb_17_col101,
            mul_res_limb_18_col102,
            mul_res_limb_19_col103,
            mul_res_limb_20_col104,
            mul_res_limb_21_col105,
            mul_res_limb_22_col106,
            mul_res_limb_23_col107,
            mul_res_limb_24_col108,
            mul_res_limb_25_col109,
            mul_res_limb_26_col110,
            mul_res_limb_27_col111,
            k_col112,
            carry_0_col113,
            carry_1_col114,
            carry_2_col115,
            carry_3_col116,
            carry_4_col117,
            carry_5_col118,
            carry_6_col119,
            carry_7_col120,
            carry_8_col121,
            carry_9_col122,
            carry_10_col123,
            carry_11_col124,
            carry_12_col125,
            carry_13_col126,
            carry_14_col127,
            carry_15_col128,
            carry_16_col129,
            carry_17_col130,
            carry_18_col131,
            carry_19_col132,
            carry_20_col133,
            carry_21_col134,
            carry_22_col135,
            carry_23_col136,
            carry_24_col137,
            carry_25_col138,
            carry_26_col139,
            enabler,
        ]: [Span<QM31>; 141] =
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
        let [unpacked_limb_0_col10]: [QM31; 1] = (*unpacked_limb_0_col10.try_into().unwrap())
            .unbox();
        let [unpacked_limb_1_col11]: [QM31; 1] = (*unpacked_limb_1_col11.try_into().unwrap())
            .unbox();
        let [unpacked_limb_3_col12]: [QM31; 1] = (*unpacked_limb_3_col12.try_into().unwrap())
            .unbox();
        let [unpacked_limb_4_col13]: [QM31; 1] = (*unpacked_limb_4_col13.try_into().unwrap())
            .unbox();
        let [unpacked_limb_6_col14]: [QM31; 1] = (*unpacked_limb_6_col14.try_into().unwrap())
            .unbox();
        let [unpacked_limb_7_col15]: [QM31; 1] = (*unpacked_limb_7_col15.try_into().unwrap())
            .unbox();
        let [unpacked_limb_9_col16]: [QM31; 1] = (*unpacked_limb_9_col16.try_into().unwrap())
            .unbox();
        let [unpacked_limb_10_col17]: [QM31; 1] = (*unpacked_limb_10_col17.try_into().unwrap())
            .unbox();
        let [unpacked_limb_12_col18]: [QM31; 1] = (*unpacked_limb_12_col18.try_into().unwrap())
            .unbox();
        let [unpacked_limb_13_col19]: [QM31; 1] = (*unpacked_limb_13_col19.try_into().unwrap())
            .unbox();
        let [unpacked_limb_15_col20]: [QM31; 1] = (*unpacked_limb_15_col20.try_into().unwrap())
            .unbox();
        let [unpacked_limb_16_col21]: [QM31; 1] = (*unpacked_limb_16_col21.try_into().unwrap())
            .unbox();
        let [unpacked_limb_18_col22]: [QM31; 1] = (*unpacked_limb_18_col22.try_into().unwrap())
            .unbox();
        let [unpacked_limb_19_col23]: [QM31; 1] = (*unpacked_limb_19_col23.try_into().unwrap())
            .unbox();
        let [unpacked_limb_21_col24]: [QM31; 1] = (*unpacked_limb_21_col24.try_into().unwrap())
            .unbox();
        let [unpacked_limb_22_col25]: [QM31; 1] = (*unpacked_limb_22_col25.try_into().unwrap())
            .unbox();
        let [unpacked_limb_24_col26]: [QM31; 1] = (*unpacked_limb_24_col26.try_into().unwrap())
            .unbox();
        let [unpacked_limb_25_col27]: [QM31; 1] = (*unpacked_limb_25_col27.try_into().unwrap())
            .unbox();
        let [mul_res_limb_0_col28]: [QM31; 1] = (*mul_res_limb_0_col28.try_into().unwrap()).unbox();
        let [mul_res_limb_1_col29]: [QM31; 1] = (*mul_res_limb_1_col29.try_into().unwrap()).unbox();
        let [mul_res_limb_2_col30]: [QM31; 1] = (*mul_res_limb_2_col30.try_into().unwrap()).unbox();
        let [mul_res_limb_3_col31]: [QM31; 1] = (*mul_res_limb_3_col31.try_into().unwrap()).unbox();
        let [mul_res_limb_4_col32]: [QM31; 1] = (*mul_res_limb_4_col32.try_into().unwrap()).unbox();
        let [mul_res_limb_5_col33]: [QM31; 1] = (*mul_res_limb_5_col33.try_into().unwrap()).unbox();
        let [mul_res_limb_6_col34]: [QM31; 1] = (*mul_res_limb_6_col34.try_into().unwrap()).unbox();
        let [mul_res_limb_7_col35]: [QM31; 1] = (*mul_res_limb_7_col35.try_into().unwrap()).unbox();
        let [mul_res_limb_8_col36]: [QM31; 1] = (*mul_res_limb_8_col36.try_into().unwrap()).unbox();
        let [mul_res_limb_9_col37]: [QM31; 1] = (*mul_res_limb_9_col37.try_into().unwrap()).unbox();
        let [mul_res_limb_10_col38]: [QM31; 1] = (*mul_res_limb_10_col38.try_into().unwrap())
            .unbox();
        let [mul_res_limb_11_col39]: [QM31; 1] = (*mul_res_limb_11_col39.try_into().unwrap())
            .unbox();
        let [mul_res_limb_12_col40]: [QM31; 1] = (*mul_res_limb_12_col40.try_into().unwrap())
            .unbox();
        let [mul_res_limb_13_col41]: [QM31; 1] = (*mul_res_limb_13_col41.try_into().unwrap())
            .unbox();
        let [mul_res_limb_14_col42]: [QM31; 1] = (*mul_res_limb_14_col42.try_into().unwrap())
            .unbox();
        let [mul_res_limb_15_col43]: [QM31; 1] = (*mul_res_limb_15_col43.try_into().unwrap())
            .unbox();
        let [mul_res_limb_16_col44]: [QM31; 1] = (*mul_res_limb_16_col44.try_into().unwrap())
            .unbox();
        let [mul_res_limb_17_col45]: [QM31; 1] = (*mul_res_limb_17_col45.try_into().unwrap())
            .unbox();
        let [mul_res_limb_18_col46]: [QM31; 1] = (*mul_res_limb_18_col46.try_into().unwrap())
            .unbox();
        let [mul_res_limb_19_col47]: [QM31; 1] = (*mul_res_limb_19_col47.try_into().unwrap())
            .unbox();
        let [mul_res_limb_20_col48]: [QM31; 1] = (*mul_res_limb_20_col48.try_into().unwrap())
            .unbox();
        let [mul_res_limb_21_col49]: [QM31; 1] = (*mul_res_limb_21_col49.try_into().unwrap())
            .unbox();
        let [mul_res_limb_22_col50]: [QM31; 1] = (*mul_res_limb_22_col50.try_into().unwrap())
            .unbox();
        let [mul_res_limb_23_col51]: [QM31; 1] = (*mul_res_limb_23_col51.try_into().unwrap())
            .unbox();
        let [mul_res_limb_24_col52]: [QM31; 1] = (*mul_res_limb_24_col52.try_into().unwrap())
            .unbox();
        let [mul_res_limb_25_col53]: [QM31; 1] = (*mul_res_limb_25_col53.try_into().unwrap())
            .unbox();
        let [mul_res_limb_26_col54]: [QM31; 1] = (*mul_res_limb_26_col54.try_into().unwrap())
            .unbox();
        let [mul_res_limb_27_col55]: [QM31; 1] = (*mul_res_limb_27_col55.try_into().unwrap())
            .unbox();
        let [k_col56]: [QM31; 1] = (*k_col56.try_into().unwrap()).unbox();
        let [carry_0_col57]: [QM31; 1] = (*carry_0_col57.try_into().unwrap()).unbox();
        let [carry_1_col58]: [QM31; 1] = (*carry_1_col58.try_into().unwrap()).unbox();
        let [carry_2_col59]: [QM31; 1] = (*carry_2_col59.try_into().unwrap()).unbox();
        let [carry_3_col60]: [QM31; 1] = (*carry_3_col60.try_into().unwrap()).unbox();
        let [carry_4_col61]: [QM31; 1] = (*carry_4_col61.try_into().unwrap()).unbox();
        let [carry_5_col62]: [QM31; 1] = (*carry_5_col62.try_into().unwrap()).unbox();
        let [carry_6_col63]: [QM31; 1] = (*carry_6_col63.try_into().unwrap()).unbox();
        let [carry_7_col64]: [QM31; 1] = (*carry_7_col64.try_into().unwrap()).unbox();
        let [carry_8_col65]: [QM31; 1] = (*carry_8_col65.try_into().unwrap()).unbox();
        let [carry_9_col66]: [QM31; 1] = (*carry_9_col66.try_into().unwrap()).unbox();
        let [carry_10_col67]: [QM31; 1] = (*carry_10_col67.try_into().unwrap()).unbox();
        let [carry_11_col68]: [QM31; 1] = (*carry_11_col68.try_into().unwrap()).unbox();
        let [carry_12_col69]: [QM31; 1] = (*carry_12_col69.try_into().unwrap()).unbox();
        let [carry_13_col70]: [QM31; 1] = (*carry_13_col70.try_into().unwrap()).unbox();
        let [carry_14_col71]: [QM31; 1] = (*carry_14_col71.try_into().unwrap()).unbox();
        let [carry_15_col72]: [QM31; 1] = (*carry_15_col72.try_into().unwrap()).unbox();
        let [carry_16_col73]: [QM31; 1] = (*carry_16_col73.try_into().unwrap()).unbox();
        let [carry_17_col74]: [QM31; 1] = (*carry_17_col74.try_into().unwrap()).unbox();
        let [carry_18_col75]: [QM31; 1] = (*carry_18_col75.try_into().unwrap()).unbox();
        let [carry_19_col76]: [QM31; 1] = (*carry_19_col76.try_into().unwrap()).unbox();
        let [carry_20_col77]: [QM31; 1] = (*carry_20_col77.try_into().unwrap()).unbox();
        let [carry_21_col78]: [QM31; 1] = (*carry_21_col78.try_into().unwrap()).unbox();
        let [carry_22_col79]: [QM31; 1] = (*carry_22_col79.try_into().unwrap()).unbox();
        let [carry_23_col80]: [QM31; 1] = (*carry_23_col80.try_into().unwrap()).unbox();
        let [carry_24_col81]: [QM31; 1] = (*carry_24_col81.try_into().unwrap()).unbox();
        let [carry_25_col82]: [QM31; 1] = (*carry_25_col82.try_into().unwrap()).unbox();
        let [carry_26_col83]: [QM31; 1] = (*carry_26_col83.try_into().unwrap()).unbox();
        let [mul_res_limb_0_col84]: [QM31; 1] = (*mul_res_limb_0_col84.try_into().unwrap()).unbox();
        let [mul_res_limb_1_col85]: [QM31; 1] = (*mul_res_limb_1_col85.try_into().unwrap()).unbox();
        let [mul_res_limb_2_col86]: [QM31; 1] = (*mul_res_limb_2_col86.try_into().unwrap()).unbox();
        let [mul_res_limb_3_col87]: [QM31; 1] = (*mul_res_limb_3_col87.try_into().unwrap()).unbox();
        let [mul_res_limb_4_col88]: [QM31; 1] = (*mul_res_limb_4_col88.try_into().unwrap()).unbox();
        let [mul_res_limb_5_col89]: [QM31; 1] = (*mul_res_limb_5_col89.try_into().unwrap()).unbox();
        let [mul_res_limb_6_col90]: [QM31; 1] = (*mul_res_limb_6_col90.try_into().unwrap()).unbox();
        let [mul_res_limb_7_col91]: [QM31; 1] = (*mul_res_limb_7_col91.try_into().unwrap()).unbox();
        let [mul_res_limb_8_col92]: [QM31; 1] = (*mul_res_limb_8_col92.try_into().unwrap()).unbox();
        let [mul_res_limb_9_col93]: [QM31; 1] = (*mul_res_limb_9_col93.try_into().unwrap()).unbox();
        let [mul_res_limb_10_col94]: [QM31; 1] = (*mul_res_limb_10_col94.try_into().unwrap())
            .unbox();
        let [mul_res_limb_11_col95]: [QM31; 1] = (*mul_res_limb_11_col95.try_into().unwrap())
            .unbox();
        let [mul_res_limb_12_col96]: [QM31; 1] = (*mul_res_limb_12_col96.try_into().unwrap())
            .unbox();
        let [mul_res_limb_13_col97]: [QM31; 1] = (*mul_res_limb_13_col97.try_into().unwrap())
            .unbox();
        let [mul_res_limb_14_col98]: [QM31; 1] = (*mul_res_limb_14_col98.try_into().unwrap())
            .unbox();
        let [mul_res_limb_15_col99]: [QM31; 1] = (*mul_res_limb_15_col99.try_into().unwrap())
            .unbox();
        let [mul_res_limb_16_col100]: [QM31; 1] = (*mul_res_limb_16_col100.try_into().unwrap())
            .unbox();
        let [mul_res_limb_17_col101]: [QM31; 1] = (*mul_res_limb_17_col101.try_into().unwrap())
            .unbox();
        let [mul_res_limb_18_col102]: [QM31; 1] = (*mul_res_limb_18_col102.try_into().unwrap())
            .unbox();
        let [mul_res_limb_19_col103]: [QM31; 1] = (*mul_res_limb_19_col103.try_into().unwrap())
            .unbox();
        let [mul_res_limb_20_col104]: [QM31; 1] = (*mul_res_limb_20_col104.try_into().unwrap())
            .unbox();
        let [mul_res_limb_21_col105]: [QM31; 1] = (*mul_res_limb_21_col105.try_into().unwrap())
            .unbox();
        let [mul_res_limb_22_col106]: [QM31; 1] = (*mul_res_limb_22_col106.try_into().unwrap())
            .unbox();
        let [mul_res_limb_23_col107]: [QM31; 1] = (*mul_res_limb_23_col107.try_into().unwrap())
            .unbox();
        let [mul_res_limb_24_col108]: [QM31; 1] = (*mul_res_limb_24_col108.try_into().unwrap())
            .unbox();
        let [mul_res_limb_25_col109]: [QM31; 1] = (*mul_res_limb_25_col109.try_into().unwrap())
            .unbox();
        let [mul_res_limb_26_col110]: [QM31; 1] = (*mul_res_limb_26_col110.try_into().unwrap())
            .unbox();
        let [mul_res_limb_27_col111]: [QM31; 1] = (*mul_res_limb_27_col111.try_into().unwrap())
            .unbox();
        let [k_col112]: [QM31; 1] = (*k_col112.try_into().unwrap()).unbox();
        let [carry_0_col113]: [QM31; 1] = (*carry_0_col113.try_into().unwrap()).unbox();
        let [carry_1_col114]: [QM31; 1] = (*carry_1_col114.try_into().unwrap()).unbox();
        let [carry_2_col115]: [QM31; 1] = (*carry_2_col115.try_into().unwrap()).unbox();
        let [carry_3_col116]: [QM31; 1] = (*carry_3_col116.try_into().unwrap()).unbox();
        let [carry_4_col117]: [QM31; 1] = (*carry_4_col117.try_into().unwrap()).unbox();
        let [carry_5_col118]: [QM31; 1] = (*carry_5_col118.try_into().unwrap()).unbox();
        let [carry_6_col119]: [QM31; 1] = (*carry_6_col119.try_into().unwrap()).unbox();
        let [carry_7_col120]: [QM31; 1] = (*carry_7_col120.try_into().unwrap()).unbox();
        let [carry_8_col121]: [QM31; 1] = (*carry_8_col121.try_into().unwrap()).unbox();
        let [carry_9_col122]: [QM31; 1] = (*carry_9_col122.try_into().unwrap()).unbox();
        let [carry_10_col123]: [QM31; 1] = (*carry_10_col123.try_into().unwrap()).unbox();
        let [carry_11_col124]: [QM31; 1] = (*carry_11_col124.try_into().unwrap()).unbox();
        let [carry_12_col125]: [QM31; 1] = (*carry_12_col125.try_into().unwrap()).unbox();
        let [carry_13_col126]: [QM31; 1] = (*carry_13_col126.try_into().unwrap()).unbox();
        let [carry_14_col127]: [QM31; 1] = (*carry_14_col127.try_into().unwrap()).unbox();
        let [carry_15_col128]: [QM31; 1] = (*carry_15_col128.try_into().unwrap()).unbox();
        let [carry_16_col129]: [QM31; 1] = (*carry_16_col129.try_into().unwrap()).unbox();
        let [carry_17_col130]: [QM31; 1] = (*carry_17_col130.try_into().unwrap()).unbox();
        let [carry_18_col131]: [QM31; 1] = (*carry_18_col131.try_into().unwrap()).unbox();
        let [carry_19_col132]: [QM31; 1] = (*carry_19_col132.try_into().unwrap()).unbox();
        let [carry_20_col133]: [QM31; 1] = (*carry_20_col133.try_into().unwrap()).unbox();
        let [carry_21_col134]: [QM31; 1] = (*carry_21_col134.try_into().unwrap()).unbox();
        let [carry_22_col135]: [QM31; 1] = (*carry_22_col135.try_into().unwrap()).unbox();
        let [carry_23_col136]: [QM31; 1] = (*carry_23_col136.try_into().unwrap()).unbox();
        let [carry_24_col137]: [QM31; 1] = (*carry_24_col137.try_into().unwrap()).unbox();
        let [carry_25_col138]: [QM31; 1] = (*carry_25_col138.try_into().unwrap()).unbox();
        let [carry_26_col139]: [QM31; 1] = (*carry_26_col139.try_into().unwrap()).unbox();
        let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        let constraint_quotient = (enabler * enabler - enabler) * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        let output: [QM31; 10] = felt_252_unpack_from_27_range_check_output_evaluate(
            [
                input_limb_0_col0, input_limb_1_col1, input_limb_2_col2, input_limb_3_col3,
                input_limb_4_col4, input_limb_5_col5, input_limb_6_col6, input_limb_7_col7,
                input_limb_8_col8, input_limb_9_col9,
            ],
            unpacked_limb_0_col10,
            unpacked_limb_1_col11,
            unpacked_limb_3_col12,
            unpacked_limb_4_col13,
            unpacked_limb_6_col14,
            unpacked_limb_7_col15,
            unpacked_limb_9_col16,
            unpacked_limb_10_col17,
            unpacked_limb_12_col18,
            unpacked_limb_13_col19,
            unpacked_limb_15_col20,
            unpacked_limb_16_col21,
            unpacked_limb_18_col22,
            unpacked_limb_19_col23,
            unpacked_limb_21_col24,
            unpacked_limb_22_col25,
            unpacked_limb_24_col26,
            unpacked_limb_25_col27,
            range_check_9_9_alphas,
            range_check_9_9_z,
            ref range_check_9_9_sum_0,
            ref range_check_9_9_sum_1,
            ref range_check_9_9_sum_2,
            ref range_check_9_9_sum_3,
            ref range_check_9_9_sum_4,
            ref range_check_9_9_sum_5,
            ref range_check_9_9_sum_6,
            ref range_check_9_9_sum_7,
            ref range_check_9_9_sum_8,
            ref range_check_9_9_sum_9,
            ref range_check_9_9_sum_10,
            ref range_check_9_9_sum_11,
            ref range_check_9_9_sum_12,
            ref range_check_9_9_sum_13,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let [
            felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_2,
            felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_5,
            felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_8,
            felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_11,
            felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_14,
            felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_17,
            felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_20,
            felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_23,
            felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_26,
            felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_27,
        ] =
            output;

        mul_252_evaluate(
            [
                unpacked_limb_0_col10, unpacked_limb_1_col11,
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_2,
                unpacked_limb_3_col12, unpacked_limb_4_col13,
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_5,
                unpacked_limb_6_col14, unpacked_limb_7_col15,
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_8,
                unpacked_limb_9_col16, unpacked_limb_10_col17,
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_11,
                unpacked_limb_12_col18, unpacked_limb_13_col19,
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_14,
                unpacked_limb_15_col20, unpacked_limb_16_col21,
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_17,
                unpacked_limb_18_col22, unpacked_limb_19_col23,
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_20,
                unpacked_limb_21_col24, unpacked_limb_22_col25,
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_23,
                unpacked_limb_24_col26, unpacked_limb_25_col27,
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_26,
                input_limb_9_col9, unpacked_limb_0_col10, unpacked_limb_1_col11,
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_2,
                unpacked_limb_3_col12, unpacked_limb_4_col13,
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_5,
                unpacked_limb_6_col14, unpacked_limb_7_col15,
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_8,
                unpacked_limb_9_col16, unpacked_limb_10_col17,
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_11,
                unpacked_limb_12_col18, unpacked_limb_13_col19,
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_14,
                unpacked_limb_15_col20, unpacked_limb_16_col21,
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_17,
                unpacked_limb_18_col22, unpacked_limb_19_col23,
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_20,
                unpacked_limb_21_col24, unpacked_limb_22_col25,
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_23,
                unpacked_limb_24_col26, unpacked_limb_25_col27,
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_26,
                input_limb_9_col9,
            ],
            mul_res_limb_0_col28,
            mul_res_limb_1_col29,
            mul_res_limb_2_col30,
            mul_res_limb_3_col31,
            mul_res_limb_4_col32,
            mul_res_limb_5_col33,
            mul_res_limb_6_col34,
            mul_res_limb_7_col35,
            mul_res_limb_8_col36,
            mul_res_limb_9_col37,
            mul_res_limb_10_col38,
            mul_res_limb_11_col39,
            mul_res_limb_12_col40,
            mul_res_limb_13_col41,
            mul_res_limb_14_col42,
            mul_res_limb_15_col43,
            mul_res_limb_16_col44,
            mul_res_limb_17_col45,
            mul_res_limb_18_col46,
            mul_res_limb_19_col47,
            mul_res_limb_20_col48,
            mul_res_limb_21_col49,
            mul_res_limb_22_col50,
            mul_res_limb_23_col51,
            mul_res_limb_24_col52,
            mul_res_limb_25_col53,
            mul_res_limb_26_col54,
            mul_res_limb_27_col55,
            k_col56,
            carry_0_col57,
            carry_1_col58,
            carry_2_col59,
            carry_3_col60,
            carry_4_col61,
            carry_5_col62,
            carry_6_col63,
            carry_7_col64,
            carry_8_col65,
            carry_9_col66,
            carry_10_col67,
            carry_11_col68,
            carry_12_col69,
            carry_13_col70,
            carry_14_col71,
            carry_15_col72,
            carry_16_col73,
            carry_17_col74,
            carry_18_col75,
            carry_19_col76,
            carry_20_col77,
            carry_21_col78,
            carry_22_col79,
            carry_23_col80,
            carry_24_col81,
            carry_25_col82,
            carry_26_col83,
            range_check_9_9_alphas,
            range_check_9_9_z,
            range_check_19_alphas,
            range_check_19_z,
            ref range_check_9_9_sum_14,
            ref range_check_9_9_sum_15,
            ref range_check_9_9_sum_16,
            ref range_check_9_9_sum_17,
            ref range_check_9_9_sum_18,
            ref range_check_9_9_sum_19,
            ref range_check_9_9_sum_20,
            ref range_check_9_9_sum_21,
            ref range_check_9_9_sum_22,
            ref range_check_9_9_sum_23,
            ref range_check_9_9_sum_24,
            ref range_check_9_9_sum_25,
            ref range_check_9_9_sum_26,
            ref range_check_9_9_sum_27,
            ref range_check_19_sum_28,
            ref range_check_19_sum_29,
            ref range_check_19_sum_30,
            ref range_check_19_sum_31,
            ref range_check_19_sum_32,
            ref range_check_19_sum_33,
            ref range_check_19_sum_34,
            ref range_check_19_sum_35,
            ref range_check_19_sum_36,
            ref range_check_19_sum_37,
            ref range_check_19_sum_38,
            ref range_check_19_sum_39,
            ref range_check_19_sum_40,
            ref range_check_19_sum_41,
            ref range_check_19_sum_42,
            ref range_check_19_sum_43,
            ref range_check_19_sum_44,
            ref range_check_19_sum_45,
            ref range_check_19_sum_46,
            ref range_check_19_sum_47,
            ref range_check_19_sum_48,
            ref range_check_19_sum_49,
            ref range_check_19_sum_50,
            ref range_check_19_sum_51,
            ref range_check_19_sum_52,
            ref range_check_19_sum_53,
            ref range_check_19_sum_54,
            ref range_check_19_sum_55,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        mul_252_evaluate(
            [
                unpacked_limb_0_col10, unpacked_limb_1_col11,
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_2,
                unpacked_limb_3_col12, unpacked_limb_4_col13,
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_5,
                unpacked_limb_6_col14, unpacked_limb_7_col15,
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_8,
                unpacked_limb_9_col16, unpacked_limb_10_col17,
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_11,
                unpacked_limb_12_col18, unpacked_limb_13_col19,
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_14,
                unpacked_limb_15_col20, unpacked_limb_16_col21,
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_17,
                unpacked_limb_18_col22, unpacked_limb_19_col23,
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_20,
                unpacked_limb_21_col24, unpacked_limb_22_col25,
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_23,
                unpacked_limb_24_col26, unpacked_limb_25_col27,
                felt_252_unpack_from_27_range_check_output_output_tmp_fec87_2_limb_26,
                input_limb_9_col9, mul_res_limb_0_col28, mul_res_limb_1_col29, mul_res_limb_2_col30,
                mul_res_limb_3_col31, mul_res_limb_4_col32, mul_res_limb_5_col33,
                mul_res_limb_6_col34, mul_res_limb_7_col35, mul_res_limb_8_col36,
                mul_res_limb_9_col37, mul_res_limb_10_col38, mul_res_limb_11_col39,
                mul_res_limb_12_col40, mul_res_limb_13_col41, mul_res_limb_14_col42,
                mul_res_limb_15_col43, mul_res_limb_16_col44, mul_res_limb_17_col45,
                mul_res_limb_18_col46, mul_res_limb_19_col47, mul_res_limb_20_col48,
                mul_res_limb_21_col49, mul_res_limb_22_col50, mul_res_limb_23_col51,
                mul_res_limb_24_col52, mul_res_limb_25_col53, mul_res_limb_26_col54,
                mul_res_limb_27_col55,
            ],
            mul_res_limb_0_col84,
            mul_res_limb_1_col85,
            mul_res_limb_2_col86,
            mul_res_limb_3_col87,
            mul_res_limb_4_col88,
            mul_res_limb_5_col89,
            mul_res_limb_6_col90,
            mul_res_limb_7_col91,
            mul_res_limb_8_col92,
            mul_res_limb_9_col93,
            mul_res_limb_10_col94,
            mul_res_limb_11_col95,
            mul_res_limb_12_col96,
            mul_res_limb_13_col97,
            mul_res_limb_14_col98,
            mul_res_limb_15_col99,
            mul_res_limb_16_col100,
            mul_res_limb_17_col101,
            mul_res_limb_18_col102,
            mul_res_limb_19_col103,
            mul_res_limb_20_col104,
            mul_res_limb_21_col105,
            mul_res_limb_22_col106,
            mul_res_limb_23_col107,
            mul_res_limb_24_col108,
            mul_res_limb_25_col109,
            mul_res_limb_26_col110,
            mul_res_limb_27_col111,
            k_col112,
            carry_0_col113,
            carry_1_col114,
            carry_2_col115,
            carry_3_col116,
            carry_4_col117,
            carry_5_col118,
            carry_6_col119,
            carry_7_col120,
            carry_8_col121,
            carry_9_col122,
            carry_10_col123,
            carry_11_col124,
            carry_12_col125,
            carry_13_col126,
            carry_14_col127,
            carry_15_col128,
            carry_16_col129,
            carry_17_col130,
            carry_18_col131,
            carry_19_col132,
            carry_20_col133,
            carry_21_col134,
            carry_22_col135,
            carry_23_col136,
            carry_24_col137,
            carry_25_col138,
            carry_26_col139,
            range_check_9_9_alphas,
            range_check_9_9_z,
            range_check_19_alphas,
            range_check_19_z,
            ref range_check_9_9_sum_56,
            ref range_check_9_9_sum_57,
            ref range_check_9_9_sum_58,
            ref range_check_9_9_sum_59,
            ref range_check_9_9_sum_60,
            ref range_check_9_9_sum_61,
            ref range_check_9_9_sum_62,
            ref range_check_9_9_sum_63,
            ref range_check_9_9_sum_64,
            ref range_check_9_9_sum_65,
            ref range_check_9_9_sum_66,
            ref range_check_9_9_sum_67,
            ref range_check_9_9_sum_68,
            ref range_check_9_9_sum_69,
            ref range_check_19_sum_70,
            ref range_check_19_sum_71,
            ref range_check_19_sum_72,
            ref range_check_19_sum_73,
            ref range_check_19_sum_74,
            ref range_check_19_sum_75,
            ref range_check_19_sum_76,
            ref range_check_19_sum_77,
            ref range_check_19_sum_78,
            ref range_check_19_sum_79,
            ref range_check_19_sum_80,
            ref range_check_19_sum_81,
            ref range_check_19_sum_82,
            ref range_check_19_sum_83,
            ref range_check_19_sum_84,
            ref range_check_19_sum_85,
            ref range_check_19_sum_86,
            ref range_check_19_sum_87,
            ref range_check_19_sum_88,
            ref range_check_19_sum_89,
            ref range_check_19_sum_90,
            ref range_check_19_sum_91,
            ref range_check_19_sum_92,
            ref range_check_19_sum_93,
            ref range_check_19_sum_94,
            ref range_check_19_sum_95,
            ref range_check_19_sum_96,
            ref range_check_19_sum_97,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        cube_252_sum_98 =
            cube_252_sum(
                cube_252_alphas,
                cube_252_z,
                [
                    input_limb_0_col0, input_limb_1_col1, input_limb_2_col2, input_limb_3_col3,
                    input_limb_4_col4, input_limb_5_col5, input_limb_6_col6, input_limb_7_col7,
                    input_limb_8_col8, input_limb_9_col9,
                    ((mul_res_limb_0_col84 + (mul_res_limb_1_col85 * qm31_const::<512, 0, 0, 0>()))
                        + (mul_res_limb_2_col86 * qm31_const::<262144, 0, 0, 0>())),
                    ((mul_res_limb_3_col87 + (mul_res_limb_4_col88 * qm31_const::<512, 0, 0, 0>()))
                        + (mul_res_limb_5_col89 * qm31_const::<262144, 0, 0, 0>())),
                    ((mul_res_limb_6_col90 + (mul_res_limb_7_col91 * qm31_const::<512, 0, 0, 0>()))
                        + (mul_res_limb_8_col92 * qm31_const::<262144, 0, 0, 0>())),
                    ((mul_res_limb_9_col93 + (mul_res_limb_10_col94 * qm31_const::<512, 0, 0, 0>()))
                        + (mul_res_limb_11_col95 * qm31_const::<262144, 0, 0, 0>())),
                    ((mul_res_limb_12_col96
                        + (mul_res_limb_13_col97 * qm31_const::<512, 0, 0, 0>()))
                        + (mul_res_limb_14_col98 * qm31_const::<262144, 0, 0, 0>())),
                    ((mul_res_limb_15_col99
                        + (mul_res_limb_16_col100 * qm31_const::<512, 0, 0, 0>()))
                        + (mul_res_limb_17_col101 * qm31_const::<262144, 0, 0, 0>())),
                    ((mul_res_limb_18_col102
                        + (mul_res_limb_19_col103 * qm31_const::<512, 0, 0, 0>()))
                        + (mul_res_limb_20_col104 * qm31_const::<262144, 0, 0, 0>())),
                    ((mul_res_limb_21_col105
                        + (mul_res_limb_22_col106 * qm31_const::<512, 0, 0, 0>()))
                        + (mul_res_limb_23_col107 * qm31_const::<262144, 0, 0, 0>())),
                    ((mul_res_limb_24_col108
                        + (mul_res_limb_25_col109 * qm31_const::<512, 0, 0, 0>()))
                        + (mul_res_limb_26_col110 * qm31_const::<262144, 0, 0, 0>())),
                    mul_res_limb_27_col111,
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
            range_check_9_9_sum_0,
            range_check_9_9_sum_1,
            range_check_9_9_sum_2,
            range_check_9_9_sum_3,
            range_check_9_9_sum_4,
            range_check_9_9_sum_5,
            range_check_9_9_sum_6,
            range_check_9_9_sum_7,
            range_check_9_9_sum_8,
            range_check_9_9_sum_9,
            range_check_9_9_sum_10,
            range_check_9_9_sum_11,
            range_check_9_9_sum_12,
            range_check_9_9_sum_13,
            range_check_9_9_sum_14,
            range_check_9_9_sum_15,
            range_check_9_9_sum_16,
            range_check_9_9_sum_17,
            range_check_9_9_sum_18,
            range_check_9_9_sum_19,
            range_check_9_9_sum_20,
            range_check_9_9_sum_21,
            range_check_9_9_sum_22,
            range_check_9_9_sum_23,
            range_check_9_9_sum_24,
            range_check_9_9_sum_25,
            range_check_9_9_sum_26,
            range_check_9_9_sum_27,
            range_check_19_sum_28,
            range_check_19_sum_29,
            range_check_19_sum_30,
            range_check_19_sum_31,
            range_check_19_sum_32,
            range_check_19_sum_33,
            range_check_19_sum_34,
            range_check_19_sum_35,
            range_check_19_sum_36,
            range_check_19_sum_37,
            range_check_19_sum_38,
            range_check_19_sum_39,
            range_check_19_sum_40,
            range_check_19_sum_41,
            range_check_19_sum_42,
            range_check_19_sum_43,
            range_check_19_sum_44,
            range_check_19_sum_45,
            range_check_19_sum_46,
            range_check_19_sum_47,
            range_check_19_sum_48,
            range_check_19_sum_49,
            range_check_19_sum_50,
            range_check_19_sum_51,
            range_check_19_sum_52,
            range_check_19_sum_53,
            range_check_19_sum_54,
            range_check_19_sum_55,
            range_check_9_9_sum_56,
            range_check_9_9_sum_57,
            range_check_9_9_sum_58,
            range_check_9_9_sum_59,
            range_check_9_9_sum_60,
            range_check_9_9_sum_61,
            range_check_9_9_sum_62,
            range_check_9_9_sum_63,
            range_check_9_9_sum_64,
            range_check_9_9_sum_65,
            range_check_9_9_sum_66,
            range_check_9_9_sum_67,
            range_check_9_9_sum_68,
            range_check_9_9_sum_69,
            range_check_19_sum_70,
            range_check_19_sum_71,
            range_check_19_sum_72,
            range_check_19_sum_73,
            range_check_19_sum_74,
            range_check_19_sum_75,
            range_check_19_sum_76,
            range_check_19_sum_77,
            range_check_19_sum_78,
            range_check_19_sum_79,
            range_check_19_sum_80,
            range_check_19_sum_81,
            range_check_19_sum_82,
            range_check_19_sum_83,
            range_check_19_sum_84,
            range_check_19_sum_85,
            range_check_19_sum_86,
            range_check_19_sum_87,
            range_check_19_sum_88,
            range_check_19_sum_89,
            range_check_19_sum_90,
            range_check_19_sum_91,
            range_check_19_sum_92,
            range_check_19_sum_93,
            range_check_19_sum_94,
            range_check_19_sum_95,
            range_check_19_sum_96,
            range_check_19_sum_97,
            cube_252_sum_98,
        );
    }
}


pub fn cube_252_sum(mut alphas: Span<QM31>, z: QM31, values: [QM31; 20]) -> QM31 {
    let [
        alpha0,
        alpha1,
        alpha2,
        alpha3,
        alpha4,
        alpha5,
        alpha6,
        alpha7,
        alpha8,
        alpha9,
        alpha10,
        alpha11,
        alpha12,
        alpha13,
        alpha14,
        alpha15,
        alpha16,
        alpha17,
        alpha18,
        alpha19,
    ] =
        (*alphas
        .multi_pop_front()
        .unwrap())
        .unbox();
    let [
        val0,
        val1,
        val2,
        val3,
        val4,
        val5,
        val6,
        val7,
        val8,
        val9,
        val10,
        val11,
        val12,
        val13,
        val14,
        val15,
        val16,
        val17,
        val18,
        val19,
    ] =
        values;

    alpha0 * val0
        + alpha1 * val1
        + alpha2 * val2
        + alpha3 * val3
        + alpha4 * val4
        + alpha5 * val5
        + alpha6 * val6
        + alpha7 * val7
        + alpha8 * val8
        + alpha9 * val9
        + alpha10 * val10
        + alpha11 * val11
        + alpha12 * val12
        + alpha13 * val13
        + alpha14 * val14
        + alpha15 * val15
        + alpha16 * val16
        + alpha17 * val17
        + alpha18 * val18
        + alpha19 * val19
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
    range_check_9_9_sum_0: QM31,
    range_check_9_9_sum_1: QM31,
    range_check_9_9_sum_2: QM31,
    range_check_9_9_sum_3: QM31,
    range_check_9_9_sum_4: QM31,
    range_check_9_9_sum_5: QM31,
    range_check_9_9_sum_6: QM31,
    range_check_9_9_sum_7: QM31,
    range_check_9_9_sum_8: QM31,
    range_check_9_9_sum_9: QM31,
    range_check_9_9_sum_10: QM31,
    range_check_9_9_sum_11: QM31,
    range_check_9_9_sum_12: QM31,
    range_check_9_9_sum_13: QM31,
    range_check_9_9_sum_14: QM31,
    range_check_9_9_sum_15: QM31,
    range_check_9_9_sum_16: QM31,
    range_check_9_9_sum_17: QM31,
    range_check_9_9_sum_18: QM31,
    range_check_9_9_sum_19: QM31,
    range_check_9_9_sum_20: QM31,
    range_check_9_9_sum_21: QM31,
    range_check_9_9_sum_22: QM31,
    range_check_9_9_sum_23: QM31,
    range_check_9_9_sum_24: QM31,
    range_check_9_9_sum_25: QM31,
    range_check_9_9_sum_26: QM31,
    range_check_9_9_sum_27: QM31,
    range_check_19_sum_28: QM31,
    range_check_19_sum_29: QM31,
    range_check_19_sum_30: QM31,
    range_check_19_sum_31: QM31,
    range_check_19_sum_32: QM31,
    range_check_19_sum_33: QM31,
    range_check_19_sum_34: QM31,
    range_check_19_sum_35: QM31,
    range_check_19_sum_36: QM31,
    range_check_19_sum_37: QM31,
    range_check_19_sum_38: QM31,
    range_check_19_sum_39: QM31,
    range_check_19_sum_40: QM31,
    range_check_19_sum_41: QM31,
    range_check_19_sum_42: QM31,
    range_check_19_sum_43: QM31,
    range_check_19_sum_44: QM31,
    range_check_19_sum_45: QM31,
    range_check_19_sum_46: QM31,
    range_check_19_sum_47: QM31,
    range_check_19_sum_48: QM31,
    range_check_19_sum_49: QM31,
    range_check_19_sum_50: QM31,
    range_check_19_sum_51: QM31,
    range_check_19_sum_52: QM31,
    range_check_19_sum_53: QM31,
    range_check_19_sum_54: QM31,
    range_check_19_sum_55: QM31,
    range_check_9_9_sum_56: QM31,
    range_check_9_9_sum_57: QM31,
    range_check_9_9_sum_58: QM31,
    range_check_9_9_sum_59: QM31,
    range_check_9_9_sum_60: QM31,
    range_check_9_9_sum_61: QM31,
    range_check_9_9_sum_62: QM31,
    range_check_9_9_sum_63: QM31,
    range_check_9_9_sum_64: QM31,
    range_check_9_9_sum_65: QM31,
    range_check_9_9_sum_66: QM31,
    range_check_9_9_sum_67: QM31,
    range_check_9_9_sum_68: QM31,
    range_check_9_9_sum_69: QM31,
    range_check_19_sum_70: QM31,
    range_check_19_sum_71: QM31,
    range_check_19_sum_72: QM31,
    range_check_19_sum_73: QM31,
    range_check_19_sum_74: QM31,
    range_check_19_sum_75: QM31,
    range_check_19_sum_76: QM31,
    range_check_19_sum_77: QM31,
    range_check_19_sum_78: QM31,
    range_check_19_sum_79: QM31,
    range_check_19_sum_80: QM31,
    range_check_19_sum_81: QM31,
    range_check_19_sum_82: QM31,
    range_check_19_sum_83: QM31,
    range_check_19_sum_84: QM31,
    range_check_19_sum_85: QM31,
    range_check_19_sum_86: QM31,
    range_check_19_sum_87: QM31,
    range_check_19_sum_88: QM31,
    range_check_19_sum_89: QM31,
    range_check_19_sum_90: QM31,
    range_check_19_sum_91: QM31,
    range_check_19_sum_92: QM31,
    range_check_19_sum_93: QM31,
    range_check_19_sum_94: QM31,
    range_check_19_sum_95: QM31,
    range_check_19_sum_96: QM31,
    range_check_19_sum_97: QM31,
    cube_252_sum_98: QM31,
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
        trace_2_col120,
        trace_2_col121,
        trace_2_col122,
        trace_2_col123,
        trace_2_col124,
        trace_2_col125,
        trace_2_col126,
        trace_2_col127,
        trace_2_col128,
        trace_2_col129,
        trace_2_col130,
        trace_2_col131,
        trace_2_col132,
        trace_2_col133,
        trace_2_col134,
        trace_2_col135,
        trace_2_col136,
        trace_2_col137,
        trace_2_col138,
        trace_2_col139,
        trace_2_col140,
        trace_2_col141,
        trace_2_col142,
        trace_2_col143,
        trace_2_col144,
        trace_2_col145,
        trace_2_col146,
        trace_2_col147,
        trace_2_col148,
        trace_2_col149,
        trace_2_col150,
        trace_2_col151,
        trace_2_col152,
        trace_2_col153,
        trace_2_col154,
        trace_2_col155,
        trace_2_col156,
        trace_2_col157,
        trace_2_col158,
        trace_2_col159,
        trace_2_col160,
        trace_2_col161,
        trace_2_col162,
        trace_2_col163,
        trace_2_col164,
        trace_2_col165,
        trace_2_col166,
        trace_2_col167,
        trace_2_col168,
        trace_2_col169,
        trace_2_col170,
        trace_2_col171,
        trace_2_col172,
        trace_2_col173,
        trace_2_col174,
        trace_2_col175,
        trace_2_col176,
        trace_2_col177,
        trace_2_col178,
        trace_2_col179,
        trace_2_col180,
        trace_2_col181,
        trace_2_col182,
        trace_2_col183,
        trace_2_col184,
        trace_2_col185,
        trace_2_col186,
        trace_2_col187,
        trace_2_col188,
        trace_2_col189,
        trace_2_col190,
        trace_2_col191,
        trace_2_col192,
        trace_2_col193,
        trace_2_col194,
        trace_2_col195,
        trace_2_col196,
        trace_2_col197,
        trace_2_col198,
        trace_2_col199,
    ]: [Span<QM31>; 200] =
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
    let [trace_2_col116]: [QM31; 1] = (*trace_2_col116.try_into().unwrap()).unbox();
    let [trace_2_col117]: [QM31; 1] = (*trace_2_col117.try_into().unwrap()).unbox();
    let [trace_2_col118]: [QM31; 1] = (*trace_2_col118.try_into().unwrap()).unbox();
    let [trace_2_col119]: [QM31; 1] = (*trace_2_col119.try_into().unwrap()).unbox();
    let [trace_2_col120]: [QM31; 1] = (*trace_2_col120.try_into().unwrap()).unbox();
    let [trace_2_col121]: [QM31; 1] = (*trace_2_col121.try_into().unwrap()).unbox();
    let [trace_2_col122]: [QM31; 1] = (*trace_2_col122.try_into().unwrap()).unbox();
    let [trace_2_col123]: [QM31; 1] = (*trace_2_col123.try_into().unwrap()).unbox();
    let [trace_2_col124]: [QM31; 1] = (*trace_2_col124.try_into().unwrap()).unbox();
    let [trace_2_col125]: [QM31; 1] = (*trace_2_col125.try_into().unwrap()).unbox();
    let [trace_2_col126]: [QM31; 1] = (*trace_2_col126.try_into().unwrap()).unbox();
    let [trace_2_col127]: [QM31; 1] = (*trace_2_col127.try_into().unwrap()).unbox();
    let [trace_2_col128]: [QM31; 1] = (*trace_2_col128.try_into().unwrap()).unbox();
    let [trace_2_col129]: [QM31; 1] = (*trace_2_col129.try_into().unwrap()).unbox();
    let [trace_2_col130]: [QM31; 1] = (*trace_2_col130.try_into().unwrap()).unbox();
    let [trace_2_col131]: [QM31; 1] = (*trace_2_col131.try_into().unwrap()).unbox();
    let [trace_2_col132]: [QM31; 1] = (*trace_2_col132.try_into().unwrap()).unbox();
    let [trace_2_col133]: [QM31; 1] = (*trace_2_col133.try_into().unwrap()).unbox();
    let [trace_2_col134]: [QM31; 1] = (*trace_2_col134.try_into().unwrap()).unbox();
    let [trace_2_col135]: [QM31; 1] = (*trace_2_col135.try_into().unwrap()).unbox();
    let [trace_2_col136]: [QM31; 1] = (*trace_2_col136.try_into().unwrap()).unbox();
    let [trace_2_col137]: [QM31; 1] = (*trace_2_col137.try_into().unwrap()).unbox();
    let [trace_2_col138]: [QM31; 1] = (*trace_2_col138.try_into().unwrap()).unbox();
    let [trace_2_col139]: [QM31; 1] = (*trace_2_col139.try_into().unwrap()).unbox();
    let [trace_2_col140]: [QM31; 1] = (*trace_2_col140.try_into().unwrap()).unbox();
    let [trace_2_col141]: [QM31; 1] = (*trace_2_col141.try_into().unwrap()).unbox();
    let [trace_2_col142]: [QM31; 1] = (*trace_2_col142.try_into().unwrap()).unbox();
    let [trace_2_col143]: [QM31; 1] = (*trace_2_col143.try_into().unwrap()).unbox();
    let [trace_2_col144]: [QM31; 1] = (*trace_2_col144.try_into().unwrap()).unbox();
    let [trace_2_col145]: [QM31; 1] = (*trace_2_col145.try_into().unwrap()).unbox();
    let [trace_2_col146]: [QM31; 1] = (*trace_2_col146.try_into().unwrap()).unbox();
    let [trace_2_col147]: [QM31; 1] = (*trace_2_col147.try_into().unwrap()).unbox();
    let [trace_2_col148]: [QM31; 1] = (*trace_2_col148.try_into().unwrap()).unbox();
    let [trace_2_col149]: [QM31; 1] = (*trace_2_col149.try_into().unwrap()).unbox();
    let [trace_2_col150]: [QM31; 1] = (*trace_2_col150.try_into().unwrap()).unbox();
    let [trace_2_col151]: [QM31; 1] = (*trace_2_col151.try_into().unwrap()).unbox();
    let [trace_2_col152]: [QM31; 1] = (*trace_2_col152.try_into().unwrap()).unbox();
    let [trace_2_col153]: [QM31; 1] = (*trace_2_col153.try_into().unwrap()).unbox();
    let [trace_2_col154]: [QM31; 1] = (*trace_2_col154.try_into().unwrap()).unbox();
    let [trace_2_col155]: [QM31; 1] = (*trace_2_col155.try_into().unwrap()).unbox();
    let [trace_2_col156]: [QM31; 1] = (*trace_2_col156.try_into().unwrap()).unbox();
    let [trace_2_col157]: [QM31; 1] = (*trace_2_col157.try_into().unwrap()).unbox();
    let [trace_2_col158]: [QM31; 1] = (*trace_2_col158.try_into().unwrap()).unbox();
    let [trace_2_col159]: [QM31; 1] = (*trace_2_col159.try_into().unwrap()).unbox();
    let [trace_2_col160]: [QM31; 1] = (*trace_2_col160.try_into().unwrap()).unbox();
    let [trace_2_col161]: [QM31; 1] = (*trace_2_col161.try_into().unwrap()).unbox();
    let [trace_2_col162]: [QM31; 1] = (*trace_2_col162.try_into().unwrap()).unbox();
    let [trace_2_col163]: [QM31; 1] = (*trace_2_col163.try_into().unwrap()).unbox();
    let [trace_2_col164]: [QM31; 1] = (*trace_2_col164.try_into().unwrap()).unbox();
    let [trace_2_col165]: [QM31; 1] = (*trace_2_col165.try_into().unwrap()).unbox();
    let [trace_2_col166]: [QM31; 1] = (*trace_2_col166.try_into().unwrap()).unbox();
    let [trace_2_col167]: [QM31; 1] = (*trace_2_col167.try_into().unwrap()).unbox();
    let [trace_2_col168]: [QM31; 1] = (*trace_2_col168.try_into().unwrap()).unbox();
    let [trace_2_col169]: [QM31; 1] = (*trace_2_col169.try_into().unwrap()).unbox();
    let [trace_2_col170]: [QM31; 1] = (*trace_2_col170.try_into().unwrap()).unbox();
    let [trace_2_col171]: [QM31; 1] = (*trace_2_col171.try_into().unwrap()).unbox();
    let [trace_2_col172]: [QM31; 1] = (*trace_2_col172.try_into().unwrap()).unbox();
    let [trace_2_col173]: [QM31; 1] = (*trace_2_col173.try_into().unwrap()).unbox();
    let [trace_2_col174]: [QM31; 1] = (*trace_2_col174.try_into().unwrap()).unbox();
    let [trace_2_col175]: [QM31; 1] = (*trace_2_col175.try_into().unwrap()).unbox();
    let [trace_2_col176]: [QM31; 1] = (*trace_2_col176.try_into().unwrap()).unbox();
    let [trace_2_col177]: [QM31; 1] = (*trace_2_col177.try_into().unwrap()).unbox();
    let [trace_2_col178]: [QM31; 1] = (*trace_2_col178.try_into().unwrap()).unbox();
    let [trace_2_col179]: [QM31; 1] = (*trace_2_col179.try_into().unwrap()).unbox();
    let [trace_2_col180]: [QM31; 1] = (*trace_2_col180.try_into().unwrap()).unbox();
    let [trace_2_col181]: [QM31; 1] = (*trace_2_col181.try_into().unwrap()).unbox();
    let [trace_2_col182]: [QM31; 1] = (*trace_2_col182.try_into().unwrap()).unbox();
    let [trace_2_col183]: [QM31; 1] = (*trace_2_col183.try_into().unwrap()).unbox();
    let [trace_2_col184]: [QM31; 1] = (*trace_2_col184.try_into().unwrap()).unbox();
    let [trace_2_col185]: [QM31; 1] = (*trace_2_col185.try_into().unwrap()).unbox();
    let [trace_2_col186]: [QM31; 1] = (*trace_2_col186.try_into().unwrap()).unbox();
    let [trace_2_col187]: [QM31; 1] = (*trace_2_col187.try_into().unwrap()).unbox();
    let [trace_2_col188]: [QM31; 1] = (*trace_2_col188.try_into().unwrap()).unbox();
    let [trace_2_col189]: [QM31; 1] = (*trace_2_col189.try_into().unwrap()).unbox();
    let [trace_2_col190]: [QM31; 1] = (*trace_2_col190.try_into().unwrap()).unbox();
    let [trace_2_col191]: [QM31; 1] = (*trace_2_col191.try_into().unwrap()).unbox();
    let [trace_2_col192]: [QM31; 1] = (*trace_2_col192.try_into().unwrap()).unbox();
    let [trace_2_col193]: [QM31; 1] = (*trace_2_col193.try_into().unwrap()).unbox();
    let [trace_2_col194]: [QM31; 1] = (*trace_2_col194.try_into().unwrap()).unbox();
    let [trace_2_col195]: [QM31; 1] = (*trace_2_col195.try_into().unwrap()).unbox();
    let [trace_2_col196_neg1, trace_2_col196]: [QM31; 2] = (*trace_2_col196.try_into().unwrap())
        .unbox();
    let [trace_2_col197_neg1, trace_2_col197]: [QM31; 2] = (*trace_2_col197.try_into().unwrap())
        .unbox();
    let [trace_2_col198_neg1, trace_2_col198]: [QM31; 2] = (*trace_2_col198.try_into().unwrap())
        .unbox();
    let [trace_2_col199_neg1, trace_2_col199]: [QM31; 2] = (*trace_2_col199.try_into().unwrap())
        .unbox();

    core::internal::revoke_ap_tracking();

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3],
    ))
        * range_check_9_9_sum_0
        * range_check_9_9_sum_1)
        - range_check_9_9_sum_0
        - range_check_9_9_sum_1)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7],
    )
        - QM31Impl::from_partial_evals([trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3]))
        * range_check_9_9_sum_2
        * range_check_9_9_sum_3)
        - range_check_9_9_sum_2
        - range_check_9_9_sum_3)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11],
    )
        - QM31Impl::from_partial_evals([trace_2_col4, trace_2_col5, trace_2_col6, trace_2_col7]))
        * range_check_9_9_sum_4
        * range_check_9_9_sum_5)
        - range_check_9_9_sum_4
        - range_check_9_9_sum_5)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
    )
        - QM31Impl::from_partial_evals([trace_2_col8, trace_2_col9, trace_2_col10, trace_2_col11]))
        * range_check_9_9_sum_6
        * range_check_9_9_sum_7)
        - range_check_9_9_sum_6
        - range_check_9_9_sum_7)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
        ))
        * range_check_9_9_sum_8
        * range_check_9_9_sum_9)
        - range_check_9_9_sum_8
        - range_check_9_9_sum_9)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
        ))
        * range_check_9_9_sum_10
        * range_check_9_9_sum_11)
        - range_check_9_9_sum_10
        - range_check_9_9_sum_11)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23],
        ))
        * range_check_9_9_sum_12
        * range_check_9_9_sum_13)
        - range_check_9_9_sum_12
        - range_check_9_9_sum_13)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col28, trace_2_col29, trace_2_col30, trace_2_col31],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27],
        ))
        * range_check_9_9_sum_14
        * range_check_9_9_sum_15)
        - range_check_9_9_sum_14
        - range_check_9_9_sum_15)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col32, trace_2_col33, trace_2_col34, trace_2_col35],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col28, trace_2_col29, trace_2_col30, trace_2_col31],
        ))
        * range_check_9_9_sum_16
        * range_check_9_9_sum_17)
        - range_check_9_9_sum_16
        - range_check_9_9_sum_17)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col36, trace_2_col37, trace_2_col38, trace_2_col39],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col32, trace_2_col33, trace_2_col34, trace_2_col35],
        ))
        * range_check_9_9_sum_18
        * range_check_9_9_sum_19)
        - range_check_9_9_sum_18
        - range_check_9_9_sum_19)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col40, trace_2_col41, trace_2_col42, trace_2_col43],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col36, trace_2_col37, trace_2_col38, trace_2_col39],
        ))
        * range_check_9_9_sum_20
        * range_check_9_9_sum_21)
        - range_check_9_9_sum_20
        - range_check_9_9_sum_21)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col44, trace_2_col45, trace_2_col46, trace_2_col47],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col40, trace_2_col41, trace_2_col42, trace_2_col43],
        ))
        * range_check_9_9_sum_22
        * range_check_9_9_sum_23)
        - range_check_9_9_sum_22
        - range_check_9_9_sum_23)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col48, trace_2_col49, trace_2_col50, trace_2_col51],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col44, trace_2_col45, trace_2_col46, trace_2_col47],
        ))
        * range_check_9_9_sum_24
        * range_check_9_9_sum_25)
        - range_check_9_9_sum_24
        - range_check_9_9_sum_25)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col52, trace_2_col53, trace_2_col54, trace_2_col55],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col48, trace_2_col49, trace_2_col50, trace_2_col51],
        ))
        * range_check_9_9_sum_26
        * range_check_9_9_sum_27)
        - range_check_9_9_sum_26
        - range_check_9_9_sum_27)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col56, trace_2_col57, trace_2_col58, trace_2_col59],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col52, trace_2_col53, trace_2_col54, trace_2_col55],
        ))
        * range_check_19_sum_28
        * range_check_19_sum_29)
        - range_check_19_sum_28
        - range_check_19_sum_29)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col60, trace_2_col61, trace_2_col62, trace_2_col63],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col56, trace_2_col57, trace_2_col58, trace_2_col59],
        ))
        * range_check_19_sum_30
        * range_check_19_sum_31)
        - range_check_19_sum_30
        - range_check_19_sum_31)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col64, trace_2_col65, trace_2_col66, trace_2_col67],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col60, trace_2_col61, trace_2_col62, trace_2_col63],
        ))
        * range_check_19_sum_32
        * range_check_19_sum_33)
        - range_check_19_sum_32
        - range_check_19_sum_33)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col68, trace_2_col69, trace_2_col70, trace_2_col71],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col64, trace_2_col65, trace_2_col66, trace_2_col67],
        ))
        * range_check_19_sum_34
        * range_check_19_sum_35)
        - range_check_19_sum_34
        - range_check_19_sum_35)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col72, trace_2_col73, trace_2_col74, trace_2_col75],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col68, trace_2_col69, trace_2_col70, trace_2_col71],
        ))
        * range_check_19_sum_36
        * range_check_19_sum_37)
        - range_check_19_sum_36
        - range_check_19_sum_37)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col76, trace_2_col77, trace_2_col78, trace_2_col79],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col72, trace_2_col73, trace_2_col74, trace_2_col75],
        ))
        * range_check_19_sum_38
        * range_check_19_sum_39)
        - range_check_19_sum_38
        - range_check_19_sum_39)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col80, trace_2_col81, trace_2_col82, trace_2_col83],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col76, trace_2_col77, trace_2_col78, trace_2_col79],
        ))
        * range_check_19_sum_40
        * range_check_19_sum_41)
        - range_check_19_sum_40
        - range_check_19_sum_41)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col84, trace_2_col85, trace_2_col86, trace_2_col87],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col80, trace_2_col81, trace_2_col82, trace_2_col83],
        ))
        * range_check_19_sum_42
        * range_check_19_sum_43)
        - range_check_19_sum_42
        - range_check_19_sum_43)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col88, trace_2_col89, trace_2_col90, trace_2_col91],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col84, trace_2_col85, trace_2_col86, trace_2_col87],
        ))
        * range_check_19_sum_44
        * range_check_19_sum_45)
        - range_check_19_sum_44
        - range_check_19_sum_45)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col92, trace_2_col93, trace_2_col94, trace_2_col95],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col88, trace_2_col89, trace_2_col90, trace_2_col91],
        ))
        * range_check_19_sum_46
        * range_check_19_sum_47)
        - range_check_19_sum_46
        - range_check_19_sum_47)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col96, trace_2_col97, trace_2_col98, trace_2_col99],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col92, trace_2_col93, trace_2_col94, trace_2_col95],
        ))
        * range_check_19_sum_48
        * range_check_19_sum_49)
        - range_check_19_sum_48
        - range_check_19_sum_49)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col100, trace_2_col101, trace_2_col102, trace_2_col103],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col96, trace_2_col97, trace_2_col98, trace_2_col99],
        ))
        * range_check_19_sum_50
        * range_check_19_sum_51)
        - range_check_19_sum_50
        - range_check_19_sum_51)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col104, trace_2_col105, trace_2_col106, trace_2_col107],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col100, trace_2_col101, trace_2_col102, trace_2_col103],
        ))
        * range_check_19_sum_52
        * range_check_19_sum_53)
        - range_check_19_sum_52
        - range_check_19_sum_53)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col108, trace_2_col109, trace_2_col110, trace_2_col111],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col104, trace_2_col105, trace_2_col106, trace_2_col107],
        ))
        * range_check_19_sum_54
        * range_check_19_sum_55)
        - range_check_19_sum_54
        - range_check_19_sum_55)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col112, trace_2_col113, trace_2_col114, trace_2_col115],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col108, trace_2_col109, trace_2_col110, trace_2_col111],
        ))
        * range_check_9_9_sum_56
        * range_check_9_9_sum_57)
        - range_check_9_9_sum_56
        - range_check_9_9_sum_57)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col116, trace_2_col117, trace_2_col118, trace_2_col119],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col112, trace_2_col113, trace_2_col114, trace_2_col115],
        ))
        * range_check_9_9_sum_58
        * range_check_9_9_sum_59)
        - range_check_9_9_sum_58
        - range_check_9_9_sum_59)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col120, trace_2_col121, trace_2_col122, trace_2_col123],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col116, trace_2_col117, trace_2_col118, trace_2_col119],
        ))
        * range_check_9_9_sum_60
        * range_check_9_9_sum_61)
        - range_check_9_9_sum_60
        - range_check_9_9_sum_61)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col124, trace_2_col125, trace_2_col126, trace_2_col127],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col120, trace_2_col121, trace_2_col122, trace_2_col123],
        ))
        * range_check_9_9_sum_62
        * range_check_9_9_sum_63)
        - range_check_9_9_sum_62
        - range_check_9_9_sum_63)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col128, trace_2_col129, trace_2_col130, trace_2_col131],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col124, trace_2_col125, trace_2_col126, trace_2_col127],
        ))
        * range_check_9_9_sum_64
        * range_check_9_9_sum_65)
        - range_check_9_9_sum_64
        - range_check_9_9_sum_65)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col132, trace_2_col133, trace_2_col134, trace_2_col135],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col128, trace_2_col129, trace_2_col130, trace_2_col131],
        ))
        * range_check_9_9_sum_66
        * range_check_9_9_sum_67)
        - range_check_9_9_sum_66
        - range_check_9_9_sum_67)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col136, trace_2_col137, trace_2_col138, trace_2_col139],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col132, trace_2_col133, trace_2_col134, trace_2_col135],
        ))
        * range_check_9_9_sum_68
        * range_check_9_9_sum_69)
        - range_check_9_9_sum_68
        - range_check_9_9_sum_69)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col140, trace_2_col141, trace_2_col142, trace_2_col143],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col136, trace_2_col137, trace_2_col138, trace_2_col139],
        ))
        * range_check_19_sum_70
        * range_check_19_sum_71)
        - range_check_19_sum_70
        - range_check_19_sum_71)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col144, trace_2_col145, trace_2_col146, trace_2_col147],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col140, trace_2_col141, trace_2_col142, trace_2_col143],
        ))
        * range_check_19_sum_72
        * range_check_19_sum_73)
        - range_check_19_sum_72
        - range_check_19_sum_73)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col148, trace_2_col149, trace_2_col150, trace_2_col151],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col144, trace_2_col145, trace_2_col146, trace_2_col147],
        ))
        * range_check_19_sum_74
        * range_check_19_sum_75)
        - range_check_19_sum_74
        - range_check_19_sum_75)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col152, trace_2_col153, trace_2_col154, trace_2_col155],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col148, trace_2_col149, trace_2_col150, trace_2_col151],
        ))
        * range_check_19_sum_76
        * range_check_19_sum_77)
        - range_check_19_sum_76
        - range_check_19_sum_77)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col156, trace_2_col157, trace_2_col158, trace_2_col159],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col152, trace_2_col153, trace_2_col154, trace_2_col155],
        ))
        * range_check_19_sum_78
        * range_check_19_sum_79)
        - range_check_19_sum_78
        - range_check_19_sum_79)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col160, trace_2_col161, trace_2_col162, trace_2_col163],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col156, trace_2_col157, trace_2_col158, trace_2_col159],
        ))
        * range_check_19_sum_80
        * range_check_19_sum_81)
        - range_check_19_sum_80
        - range_check_19_sum_81)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col164, trace_2_col165, trace_2_col166, trace_2_col167],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col160, trace_2_col161, trace_2_col162, trace_2_col163],
        ))
        * range_check_19_sum_82
        * range_check_19_sum_83)
        - range_check_19_sum_82
        - range_check_19_sum_83)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col168, trace_2_col169, trace_2_col170, trace_2_col171],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col164, trace_2_col165, trace_2_col166, trace_2_col167],
        ))
        * range_check_19_sum_84
        * range_check_19_sum_85)
        - range_check_19_sum_84
        - range_check_19_sum_85)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col172, trace_2_col173, trace_2_col174, trace_2_col175],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col168, trace_2_col169, trace_2_col170, trace_2_col171],
        ))
        * range_check_19_sum_86
        * range_check_19_sum_87)
        - range_check_19_sum_86
        - range_check_19_sum_87)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col176, trace_2_col177, trace_2_col178, trace_2_col179],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col172, trace_2_col173, trace_2_col174, trace_2_col175],
        ))
        * range_check_19_sum_88
        * range_check_19_sum_89)
        - range_check_19_sum_88
        - range_check_19_sum_89)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col180, trace_2_col181, trace_2_col182, trace_2_col183],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col176, trace_2_col177, trace_2_col178, trace_2_col179],
        ))
        * range_check_19_sum_90
        * range_check_19_sum_91)
        - range_check_19_sum_90
        - range_check_19_sum_91)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col184, trace_2_col185, trace_2_col186, trace_2_col187],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col180, trace_2_col181, trace_2_col182, trace_2_col183],
        ))
        * range_check_19_sum_92
        * range_check_19_sum_93)
        - range_check_19_sum_92
        - range_check_19_sum_93)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col188, trace_2_col189, trace_2_col190, trace_2_col191],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col184, trace_2_col185, trace_2_col186, trace_2_col187],
        ))
        * range_check_19_sum_94
        * range_check_19_sum_95)
        - range_check_19_sum_94
        - range_check_19_sum_95)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col192, trace_2_col193, trace_2_col194, trace_2_col195],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col188, trace_2_col189, trace_2_col190, trace_2_col191],
        ))
        * range_check_19_sum_96
        * range_check_19_sum_97)
        - range_check_19_sum_96
        - range_check_19_sum_97)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col196, trace_2_col197, trace_2_col198, trace_2_col199],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col192, trace_2_col193, trace_2_col194, trace_2_col195],
        )
        - QM31Impl::from_partial_evals(
            [trace_2_col196_neg1, trace_2_col197_neg1, trace_2_col198_neg1, trace_2_col199_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * cube_252_sum_98)
        + enabler)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
}
