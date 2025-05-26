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
use crate::components::subroutines::decode_generic_instruction::decode_generic_instruction_evaluate;
use crate::components::subroutines::eval_operands::eval_operands_evaluate;
use crate::components::subroutines::handle_opcodes::handle_opcodes_evaluate;
use crate::components::subroutines::update_registers::update_registers_evaluate;
use crate::utils::U32Impl;

pub const N_TRACE_COLUMNS: usize = 236;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 6] = [
    ('VerifyInstruction', 1), ('MemoryAddressToId', 3), ('MemoryIdToBig', 3),
    ('RangeCheck_9_9', 28), ('RangeCheck_19', 28), ('Opcodes', 1),
];

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
        let interaction_log_sizes = ArrayImpl::new_repeated(132, log_size).span();
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
    pub range_check_9_9_lookup_elements: crate::RangeCheck_9_9Elements,
    pub range_check_19_lookup_elements: crate::RangeCheck_19Elements,
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
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
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
        let mut range_check_9_9_sum_28: QM31 = Zero::zero();
        let mut range_check_9_9_sum_29: QM31 = Zero::zero();
        let mut range_check_9_9_sum_30: QM31 = Zero::zero();
        let mut range_check_9_9_sum_31: QM31 = Zero::zero();
        let mut range_check_9_9_sum_32: QM31 = Zero::zero();
        let mut range_check_9_9_sum_33: QM31 = Zero::zero();
        let mut range_check_9_9_sum_34: QM31 = Zero::zero();
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
        let mut range_check_19_sum_56: QM31 = Zero::zero();
        let mut range_check_19_sum_57: QM31 = Zero::zero();
        let mut range_check_19_sum_58: QM31 = Zero::zero();
        let mut range_check_19_sum_59: QM31 = Zero::zero();
        let mut range_check_19_sum_60: QM31 = Zero::zero();
        let mut range_check_19_sum_61: QM31 = Zero::zero();
        let mut range_check_19_sum_62: QM31 = Zero::zero();
        let mut opcodes_sum_63: QM31 = Zero::zero();
        let mut opcodes_sum_64: QM31 = Zero::zero();

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
            op1_base_ap_col10,
            res_add_col11,
            res_mul_col12,
            pc_update_jump_col13,
            pc_update_jump_rel_col14,
            pc_update_jnz_col15,
            ap_update_add_col16,
            ap_update_add_1_col17,
            opcode_call_col18,
            opcode_ret_col19,
            opcode_assert_eq_col20,
            dst_src_col21,
            dst_id_col22,
            dst_limb_0_col23,
            dst_limb_1_col24,
            dst_limb_2_col25,
            dst_limb_3_col26,
            dst_limb_4_col27,
            dst_limb_5_col28,
            dst_limb_6_col29,
            dst_limb_7_col30,
            dst_limb_8_col31,
            dst_limb_9_col32,
            dst_limb_10_col33,
            dst_limb_11_col34,
            dst_limb_12_col35,
            dst_limb_13_col36,
            dst_limb_14_col37,
            dst_limb_15_col38,
            dst_limb_16_col39,
            dst_limb_17_col40,
            dst_limb_18_col41,
            dst_limb_19_col42,
            dst_limb_20_col43,
            dst_limb_21_col44,
            dst_limb_22_col45,
            dst_limb_23_col46,
            dst_limb_24_col47,
            dst_limb_25_col48,
            dst_limb_26_col49,
            dst_limb_27_col50,
            op0_src_col51,
            op0_id_col52,
            op0_limb_0_col53,
            op0_limb_1_col54,
            op0_limb_2_col55,
            op0_limb_3_col56,
            op0_limb_4_col57,
            op0_limb_5_col58,
            op0_limb_6_col59,
            op0_limb_7_col60,
            op0_limb_8_col61,
            op0_limb_9_col62,
            op0_limb_10_col63,
            op0_limb_11_col64,
            op0_limb_12_col65,
            op0_limb_13_col66,
            op0_limb_14_col67,
            op0_limb_15_col68,
            op0_limb_16_col69,
            op0_limb_17_col70,
            op0_limb_18_col71,
            op0_limb_19_col72,
            op0_limb_20_col73,
            op0_limb_21_col74,
            op0_limb_22_col75,
            op0_limb_23_col76,
            op0_limb_24_col77,
            op0_limb_25_col78,
            op0_limb_26_col79,
            op0_limb_27_col80,
            op1_src_col81,
            op1_id_col82,
            op1_limb_0_col83,
            op1_limb_1_col84,
            op1_limb_2_col85,
            op1_limb_3_col86,
            op1_limb_4_col87,
            op1_limb_5_col88,
            op1_limb_6_col89,
            op1_limb_7_col90,
            op1_limb_8_col91,
            op1_limb_9_col92,
            op1_limb_10_col93,
            op1_limb_11_col94,
            op1_limb_12_col95,
            op1_limb_13_col96,
            op1_limb_14_col97,
            op1_limb_15_col98,
            op1_limb_16_col99,
            op1_limb_17_col100,
            op1_limb_18_col101,
            op1_limb_19_col102,
            op1_limb_20_col103,
            op1_limb_21_col104,
            op1_limb_22_col105,
            op1_limb_23_col106,
            op1_limb_24_col107,
            op1_limb_25_col108,
            op1_limb_26_col109,
            op1_limb_27_col110,
            add_res_limb_0_col111,
            add_res_limb_1_col112,
            add_res_limb_2_col113,
            add_res_limb_3_col114,
            add_res_limb_4_col115,
            add_res_limb_5_col116,
            add_res_limb_6_col117,
            add_res_limb_7_col118,
            add_res_limb_8_col119,
            add_res_limb_9_col120,
            add_res_limb_10_col121,
            add_res_limb_11_col122,
            add_res_limb_12_col123,
            add_res_limb_13_col124,
            add_res_limb_14_col125,
            add_res_limb_15_col126,
            add_res_limb_16_col127,
            add_res_limb_17_col128,
            add_res_limb_18_col129,
            add_res_limb_19_col130,
            add_res_limb_20_col131,
            add_res_limb_21_col132,
            add_res_limb_22_col133,
            add_res_limb_23_col134,
            add_res_limb_24_col135,
            add_res_limb_25_col136,
            add_res_limb_26_col137,
            add_res_limb_27_col138,
            sub_p_bit_col139,
            mul_res_limb_0_col140,
            mul_res_limb_1_col141,
            mul_res_limb_2_col142,
            mul_res_limb_3_col143,
            mul_res_limb_4_col144,
            mul_res_limb_5_col145,
            mul_res_limb_6_col146,
            mul_res_limb_7_col147,
            mul_res_limb_8_col148,
            mul_res_limb_9_col149,
            mul_res_limb_10_col150,
            mul_res_limb_11_col151,
            mul_res_limb_12_col152,
            mul_res_limb_13_col153,
            mul_res_limb_14_col154,
            mul_res_limb_15_col155,
            mul_res_limb_16_col156,
            mul_res_limb_17_col157,
            mul_res_limb_18_col158,
            mul_res_limb_19_col159,
            mul_res_limb_20_col160,
            mul_res_limb_21_col161,
            mul_res_limb_22_col162,
            mul_res_limb_23_col163,
            mul_res_limb_24_col164,
            mul_res_limb_25_col165,
            mul_res_limb_26_col166,
            mul_res_limb_27_col167,
            k_col168,
            carry_0_col169,
            carry_1_col170,
            carry_2_col171,
            carry_3_col172,
            carry_4_col173,
            carry_5_col174,
            carry_6_col175,
            carry_7_col176,
            carry_8_col177,
            carry_9_col178,
            carry_10_col179,
            carry_11_col180,
            carry_12_col181,
            carry_13_col182,
            carry_14_col183,
            carry_15_col184,
            carry_16_col185,
            carry_17_col186,
            carry_18_col187,
            carry_19_col188,
            carry_20_col189,
            carry_21_col190,
            carry_22_col191,
            carry_23_col192,
            carry_24_col193,
            carry_25_col194,
            carry_26_col195,
            res_limb_0_col196,
            res_limb_1_col197,
            res_limb_2_col198,
            res_limb_3_col199,
            res_limb_4_col200,
            res_limb_5_col201,
            res_limb_6_col202,
            res_limb_7_col203,
            res_limb_8_col204,
            res_limb_9_col205,
            res_limb_10_col206,
            res_limb_11_col207,
            res_limb_12_col208,
            res_limb_13_col209,
            res_limb_14_col210,
            res_limb_15_col211,
            res_limb_16_col212,
            res_limb_17_col213,
            res_limb_18_col214,
            res_limb_19_col215,
            res_limb_20_col216,
            res_limb_21_col217,
            res_limb_22_col218,
            res_limb_23_col219,
            res_limb_24_col220,
            res_limb_25_col221,
            res_limb_26_col222,
            res_limb_27_col223,
            msb_col224,
            mid_limbs_set_col225,
            dst_sum_squares_inv_col226,
            dst_sum_inv_col227,
            op1_as_rel_imm_cond_col228,
            msb_col229,
            mid_limbs_set_col230,
            next_pc_jnz_col231,
            next_pc_col232,
            next_ap_col233,
            next_fp_col234,
            enabler,
        ]: [Span<QM31>; 236] =
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
        let [op1_base_ap_col10]: [QM31; 1] = (*op1_base_ap_col10.try_into().unwrap()).unbox();
        let [res_add_col11]: [QM31; 1] = (*res_add_col11.try_into().unwrap()).unbox();
        let [res_mul_col12]: [QM31; 1] = (*res_mul_col12.try_into().unwrap()).unbox();
        let [pc_update_jump_col13]: [QM31; 1] = (*pc_update_jump_col13.try_into().unwrap()).unbox();
        let [pc_update_jump_rel_col14]: [QM31; 1] = (*pc_update_jump_rel_col14.try_into().unwrap())
            .unbox();
        let [pc_update_jnz_col15]: [QM31; 1] = (*pc_update_jnz_col15.try_into().unwrap()).unbox();
        let [ap_update_add_col16]: [QM31; 1] = (*ap_update_add_col16.try_into().unwrap()).unbox();
        let [ap_update_add_1_col17]: [QM31; 1] = (*ap_update_add_1_col17.try_into().unwrap())
            .unbox();
        let [opcode_call_col18]: [QM31; 1] = (*opcode_call_col18.try_into().unwrap()).unbox();
        let [opcode_ret_col19]: [QM31; 1] = (*opcode_ret_col19.try_into().unwrap()).unbox();
        let [opcode_assert_eq_col20]: [QM31; 1] = (*opcode_assert_eq_col20.try_into().unwrap())
            .unbox();
        let [dst_src_col21]: [QM31; 1] = (*dst_src_col21.try_into().unwrap()).unbox();
        let [dst_id_col22]: [QM31; 1] = (*dst_id_col22.try_into().unwrap()).unbox();
        let [dst_limb_0_col23]: [QM31; 1] = (*dst_limb_0_col23.try_into().unwrap()).unbox();
        let [dst_limb_1_col24]: [QM31; 1] = (*dst_limb_1_col24.try_into().unwrap()).unbox();
        let [dst_limb_2_col25]: [QM31; 1] = (*dst_limb_2_col25.try_into().unwrap()).unbox();
        let [dst_limb_3_col26]: [QM31; 1] = (*dst_limb_3_col26.try_into().unwrap()).unbox();
        let [dst_limb_4_col27]: [QM31; 1] = (*dst_limb_4_col27.try_into().unwrap()).unbox();
        let [dst_limb_5_col28]: [QM31; 1] = (*dst_limb_5_col28.try_into().unwrap()).unbox();
        let [dst_limb_6_col29]: [QM31; 1] = (*dst_limb_6_col29.try_into().unwrap()).unbox();
        let [dst_limb_7_col30]: [QM31; 1] = (*dst_limb_7_col30.try_into().unwrap()).unbox();
        let [dst_limb_8_col31]: [QM31; 1] = (*dst_limb_8_col31.try_into().unwrap()).unbox();
        let [dst_limb_9_col32]: [QM31; 1] = (*dst_limb_9_col32.try_into().unwrap()).unbox();
        let [dst_limb_10_col33]: [QM31; 1] = (*dst_limb_10_col33.try_into().unwrap()).unbox();
        let [dst_limb_11_col34]: [QM31; 1] = (*dst_limb_11_col34.try_into().unwrap()).unbox();
        let [dst_limb_12_col35]: [QM31; 1] = (*dst_limb_12_col35.try_into().unwrap()).unbox();
        let [dst_limb_13_col36]: [QM31; 1] = (*dst_limb_13_col36.try_into().unwrap()).unbox();
        let [dst_limb_14_col37]: [QM31; 1] = (*dst_limb_14_col37.try_into().unwrap()).unbox();
        let [dst_limb_15_col38]: [QM31; 1] = (*dst_limb_15_col38.try_into().unwrap()).unbox();
        let [dst_limb_16_col39]: [QM31; 1] = (*dst_limb_16_col39.try_into().unwrap()).unbox();
        let [dst_limb_17_col40]: [QM31; 1] = (*dst_limb_17_col40.try_into().unwrap()).unbox();
        let [dst_limb_18_col41]: [QM31; 1] = (*dst_limb_18_col41.try_into().unwrap()).unbox();
        let [dst_limb_19_col42]: [QM31; 1] = (*dst_limb_19_col42.try_into().unwrap()).unbox();
        let [dst_limb_20_col43]: [QM31; 1] = (*dst_limb_20_col43.try_into().unwrap()).unbox();
        let [dst_limb_21_col44]: [QM31; 1] = (*dst_limb_21_col44.try_into().unwrap()).unbox();
        let [dst_limb_22_col45]: [QM31; 1] = (*dst_limb_22_col45.try_into().unwrap()).unbox();
        let [dst_limb_23_col46]: [QM31; 1] = (*dst_limb_23_col46.try_into().unwrap()).unbox();
        let [dst_limb_24_col47]: [QM31; 1] = (*dst_limb_24_col47.try_into().unwrap()).unbox();
        let [dst_limb_25_col48]: [QM31; 1] = (*dst_limb_25_col48.try_into().unwrap()).unbox();
        let [dst_limb_26_col49]: [QM31; 1] = (*dst_limb_26_col49.try_into().unwrap()).unbox();
        let [dst_limb_27_col50]: [QM31; 1] = (*dst_limb_27_col50.try_into().unwrap()).unbox();
        let [op0_src_col51]: [QM31; 1] = (*op0_src_col51.try_into().unwrap()).unbox();
        let [op0_id_col52]: [QM31; 1] = (*op0_id_col52.try_into().unwrap()).unbox();
        let [op0_limb_0_col53]: [QM31; 1] = (*op0_limb_0_col53.try_into().unwrap()).unbox();
        let [op0_limb_1_col54]: [QM31; 1] = (*op0_limb_1_col54.try_into().unwrap()).unbox();
        let [op0_limb_2_col55]: [QM31; 1] = (*op0_limb_2_col55.try_into().unwrap()).unbox();
        let [op0_limb_3_col56]: [QM31; 1] = (*op0_limb_3_col56.try_into().unwrap()).unbox();
        let [op0_limb_4_col57]: [QM31; 1] = (*op0_limb_4_col57.try_into().unwrap()).unbox();
        let [op0_limb_5_col58]: [QM31; 1] = (*op0_limb_5_col58.try_into().unwrap()).unbox();
        let [op0_limb_6_col59]: [QM31; 1] = (*op0_limb_6_col59.try_into().unwrap()).unbox();
        let [op0_limb_7_col60]: [QM31; 1] = (*op0_limb_7_col60.try_into().unwrap()).unbox();
        let [op0_limb_8_col61]: [QM31; 1] = (*op0_limb_8_col61.try_into().unwrap()).unbox();
        let [op0_limb_9_col62]: [QM31; 1] = (*op0_limb_9_col62.try_into().unwrap()).unbox();
        let [op0_limb_10_col63]: [QM31; 1] = (*op0_limb_10_col63.try_into().unwrap()).unbox();
        let [op0_limb_11_col64]: [QM31; 1] = (*op0_limb_11_col64.try_into().unwrap()).unbox();
        let [op0_limb_12_col65]: [QM31; 1] = (*op0_limb_12_col65.try_into().unwrap()).unbox();
        let [op0_limb_13_col66]: [QM31; 1] = (*op0_limb_13_col66.try_into().unwrap()).unbox();
        let [op0_limb_14_col67]: [QM31; 1] = (*op0_limb_14_col67.try_into().unwrap()).unbox();
        let [op0_limb_15_col68]: [QM31; 1] = (*op0_limb_15_col68.try_into().unwrap()).unbox();
        let [op0_limb_16_col69]: [QM31; 1] = (*op0_limb_16_col69.try_into().unwrap()).unbox();
        let [op0_limb_17_col70]: [QM31; 1] = (*op0_limb_17_col70.try_into().unwrap()).unbox();
        let [op0_limb_18_col71]: [QM31; 1] = (*op0_limb_18_col71.try_into().unwrap()).unbox();
        let [op0_limb_19_col72]: [QM31; 1] = (*op0_limb_19_col72.try_into().unwrap()).unbox();
        let [op0_limb_20_col73]: [QM31; 1] = (*op0_limb_20_col73.try_into().unwrap()).unbox();
        let [op0_limb_21_col74]: [QM31; 1] = (*op0_limb_21_col74.try_into().unwrap()).unbox();
        let [op0_limb_22_col75]: [QM31; 1] = (*op0_limb_22_col75.try_into().unwrap()).unbox();
        let [op0_limb_23_col76]: [QM31; 1] = (*op0_limb_23_col76.try_into().unwrap()).unbox();
        let [op0_limb_24_col77]: [QM31; 1] = (*op0_limb_24_col77.try_into().unwrap()).unbox();
        let [op0_limb_25_col78]: [QM31; 1] = (*op0_limb_25_col78.try_into().unwrap()).unbox();
        let [op0_limb_26_col79]: [QM31; 1] = (*op0_limb_26_col79.try_into().unwrap()).unbox();
        let [op0_limb_27_col80]: [QM31; 1] = (*op0_limb_27_col80.try_into().unwrap()).unbox();
        let [op1_src_col81]: [QM31; 1] = (*op1_src_col81.try_into().unwrap()).unbox();
        let [op1_id_col82]: [QM31; 1] = (*op1_id_col82.try_into().unwrap()).unbox();
        let [op1_limb_0_col83]: [QM31; 1] = (*op1_limb_0_col83.try_into().unwrap()).unbox();
        let [op1_limb_1_col84]: [QM31; 1] = (*op1_limb_1_col84.try_into().unwrap()).unbox();
        let [op1_limb_2_col85]: [QM31; 1] = (*op1_limb_2_col85.try_into().unwrap()).unbox();
        let [op1_limb_3_col86]: [QM31; 1] = (*op1_limb_3_col86.try_into().unwrap()).unbox();
        let [op1_limb_4_col87]: [QM31; 1] = (*op1_limb_4_col87.try_into().unwrap()).unbox();
        let [op1_limb_5_col88]: [QM31; 1] = (*op1_limb_5_col88.try_into().unwrap()).unbox();
        let [op1_limb_6_col89]: [QM31; 1] = (*op1_limb_6_col89.try_into().unwrap()).unbox();
        let [op1_limb_7_col90]: [QM31; 1] = (*op1_limb_7_col90.try_into().unwrap()).unbox();
        let [op1_limb_8_col91]: [QM31; 1] = (*op1_limb_8_col91.try_into().unwrap()).unbox();
        let [op1_limb_9_col92]: [QM31; 1] = (*op1_limb_9_col92.try_into().unwrap()).unbox();
        let [op1_limb_10_col93]: [QM31; 1] = (*op1_limb_10_col93.try_into().unwrap()).unbox();
        let [op1_limb_11_col94]: [QM31; 1] = (*op1_limb_11_col94.try_into().unwrap()).unbox();
        let [op1_limb_12_col95]: [QM31; 1] = (*op1_limb_12_col95.try_into().unwrap()).unbox();
        let [op1_limb_13_col96]: [QM31; 1] = (*op1_limb_13_col96.try_into().unwrap()).unbox();
        let [op1_limb_14_col97]: [QM31; 1] = (*op1_limb_14_col97.try_into().unwrap()).unbox();
        let [op1_limb_15_col98]: [QM31; 1] = (*op1_limb_15_col98.try_into().unwrap()).unbox();
        let [op1_limb_16_col99]: [QM31; 1] = (*op1_limb_16_col99.try_into().unwrap()).unbox();
        let [op1_limb_17_col100]: [QM31; 1] = (*op1_limb_17_col100.try_into().unwrap()).unbox();
        let [op1_limb_18_col101]: [QM31; 1] = (*op1_limb_18_col101.try_into().unwrap()).unbox();
        let [op1_limb_19_col102]: [QM31; 1] = (*op1_limb_19_col102.try_into().unwrap()).unbox();
        let [op1_limb_20_col103]: [QM31; 1] = (*op1_limb_20_col103.try_into().unwrap()).unbox();
        let [op1_limb_21_col104]: [QM31; 1] = (*op1_limb_21_col104.try_into().unwrap()).unbox();
        let [op1_limb_22_col105]: [QM31; 1] = (*op1_limb_22_col105.try_into().unwrap()).unbox();
        let [op1_limb_23_col106]: [QM31; 1] = (*op1_limb_23_col106.try_into().unwrap()).unbox();
        let [op1_limb_24_col107]: [QM31; 1] = (*op1_limb_24_col107.try_into().unwrap()).unbox();
        let [op1_limb_25_col108]: [QM31; 1] = (*op1_limb_25_col108.try_into().unwrap()).unbox();
        let [op1_limb_26_col109]: [QM31; 1] = (*op1_limb_26_col109.try_into().unwrap()).unbox();
        let [op1_limb_27_col110]: [QM31; 1] = (*op1_limb_27_col110.try_into().unwrap()).unbox();
        let [add_res_limb_0_col111]: [QM31; 1] = (*add_res_limb_0_col111.try_into().unwrap())
            .unbox();
        let [add_res_limb_1_col112]: [QM31; 1] = (*add_res_limb_1_col112.try_into().unwrap())
            .unbox();
        let [add_res_limb_2_col113]: [QM31; 1] = (*add_res_limb_2_col113.try_into().unwrap())
            .unbox();
        let [add_res_limb_3_col114]: [QM31; 1] = (*add_res_limb_3_col114.try_into().unwrap())
            .unbox();
        let [add_res_limb_4_col115]: [QM31; 1] = (*add_res_limb_4_col115.try_into().unwrap())
            .unbox();
        let [add_res_limb_5_col116]: [QM31; 1] = (*add_res_limb_5_col116.try_into().unwrap())
            .unbox();
        let [add_res_limb_6_col117]: [QM31; 1] = (*add_res_limb_6_col117.try_into().unwrap())
            .unbox();
        let [add_res_limb_7_col118]: [QM31; 1] = (*add_res_limb_7_col118.try_into().unwrap())
            .unbox();
        let [add_res_limb_8_col119]: [QM31; 1] = (*add_res_limb_8_col119.try_into().unwrap())
            .unbox();
        let [add_res_limb_9_col120]: [QM31; 1] = (*add_res_limb_9_col120.try_into().unwrap())
            .unbox();
        let [add_res_limb_10_col121]: [QM31; 1] = (*add_res_limb_10_col121.try_into().unwrap())
            .unbox();
        let [add_res_limb_11_col122]: [QM31; 1] = (*add_res_limb_11_col122.try_into().unwrap())
            .unbox();
        let [add_res_limb_12_col123]: [QM31; 1] = (*add_res_limb_12_col123.try_into().unwrap())
            .unbox();
        let [add_res_limb_13_col124]: [QM31; 1] = (*add_res_limb_13_col124.try_into().unwrap())
            .unbox();
        let [add_res_limb_14_col125]: [QM31; 1] = (*add_res_limb_14_col125.try_into().unwrap())
            .unbox();
        let [add_res_limb_15_col126]: [QM31; 1] = (*add_res_limb_15_col126.try_into().unwrap())
            .unbox();
        let [add_res_limb_16_col127]: [QM31; 1] = (*add_res_limb_16_col127.try_into().unwrap())
            .unbox();
        let [add_res_limb_17_col128]: [QM31; 1] = (*add_res_limb_17_col128.try_into().unwrap())
            .unbox();
        let [add_res_limb_18_col129]: [QM31; 1] = (*add_res_limb_18_col129.try_into().unwrap())
            .unbox();
        let [add_res_limb_19_col130]: [QM31; 1] = (*add_res_limb_19_col130.try_into().unwrap())
            .unbox();
        let [add_res_limb_20_col131]: [QM31; 1] = (*add_res_limb_20_col131.try_into().unwrap())
            .unbox();
        let [add_res_limb_21_col132]: [QM31; 1] = (*add_res_limb_21_col132.try_into().unwrap())
            .unbox();
        let [add_res_limb_22_col133]: [QM31; 1] = (*add_res_limb_22_col133.try_into().unwrap())
            .unbox();
        let [add_res_limb_23_col134]: [QM31; 1] = (*add_res_limb_23_col134.try_into().unwrap())
            .unbox();
        let [add_res_limb_24_col135]: [QM31; 1] = (*add_res_limb_24_col135.try_into().unwrap())
            .unbox();
        let [add_res_limb_25_col136]: [QM31; 1] = (*add_res_limb_25_col136.try_into().unwrap())
            .unbox();
        let [add_res_limb_26_col137]: [QM31; 1] = (*add_res_limb_26_col137.try_into().unwrap())
            .unbox();
        let [add_res_limb_27_col138]: [QM31; 1] = (*add_res_limb_27_col138.try_into().unwrap())
            .unbox();
        let [sub_p_bit_col139]: [QM31; 1] = (*sub_p_bit_col139.try_into().unwrap()).unbox();
        let [mul_res_limb_0_col140]: [QM31; 1] = (*mul_res_limb_0_col140.try_into().unwrap())
            .unbox();
        let [mul_res_limb_1_col141]: [QM31; 1] = (*mul_res_limb_1_col141.try_into().unwrap())
            .unbox();
        let [mul_res_limb_2_col142]: [QM31; 1] = (*mul_res_limb_2_col142.try_into().unwrap())
            .unbox();
        let [mul_res_limb_3_col143]: [QM31; 1] = (*mul_res_limb_3_col143.try_into().unwrap())
            .unbox();
        let [mul_res_limb_4_col144]: [QM31; 1] = (*mul_res_limb_4_col144.try_into().unwrap())
            .unbox();
        let [mul_res_limb_5_col145]: [QM31; 1] = (*mul_res_limb_5_col145.try_into().unwrap())
            .unbox();
        let [mul_res_limb_6_col146]: [QM31; 1] = (*mul_res_limb_6_col146.try_into().unwrap())
            .unbox();
        let [mul_res_limb_7_col147]: [QM31; 1] = (*mul_res_limb_7_col147.try_into().unwrap())
            .unbox();
        let [mul_res_limb_8_col148]: [QM31; 1] = (*mul_res_limb_8_col148.try_into().unwrap())
            .unbox();
        let [mul_res_limb_9_col149]: [QM31; 1] = (*mul_res_limb_9_col149.try_into().unwrap())
            .unbox();
        let [mul_res_limb_10_col150]: [QM31; 1] = (*mul_res_limb_10_col150.try_into().unwrap())
            .unbox();
        let [mul_res_limb_11_col151]: [QM31; 1] = (*mul_res_limb_11_col151.try_into().unwrap())
            .unbox();
        let [mul_res_limb_12_col152]: [QM31; 1] = (*mul_res_limb_12_col152.try_into().unwrap())
            .unbox();
        let [mul_res_limb_13_col153]: [QM31; 1] = (*mul_res_limb_13_col153.try_into().unwrap())
            .unbox();
        let [mul_res_limb_14_col154]: [QM31; 1] = (*mul_res_limb_14_col154.try_into().unwrap())
            .unbox();
        let [mul_res_limb_15_col155]: [QM31; 1] = (*mul_res_limb_15_col155.try_into().unwrap())
            .unbox();
        let [mul_res_limb_16_col156]: [QM31; 1] = (*mul_res_limb_16_col156.try_into().unwrap())
            .unbox();
        let [mul_res_limb_17_col157]: [QM31; 1] = (*mul_res_limb_17_col157.try_into().unwrap())
            .unbox();
        let [mul_res_limb_18_col158]: [QM31; 1] = (*mul_res_limb_18_col158.try_into().unwrap())
            .unbox();
        let [mul_res_limb_19_col159]: [QM31; 1] = (*mul_res_limb_19_col159.try_into().unwrap())
            .unbox();
        let [mul_res_limb_20_col160]: [QM31; 1] = (*mul_res_limb_20_col160.try_into().unwrap())
            .unbox();
        let [mul_res_limb_21_col161]: [QM31; 1] = (*mul_res_limb_21_col161.try_into().unwrap())
            .unbox();
        let [mul_res_limb_22_col162]: [QM31; 1] = (*mul_res_limb_22_col162.try_into().unwrap())
            .unbox();
        let [mul_res_limb_23_col163]: [QM31; 1] = (*mul_res_limb_23_col163.try_into().unwrap())
            .unbox();
        let [mul_res_limb_24_col164]: [QM31; 1] = (*mul_res_limb_24_col164.try_into().unwrap())
            .unbox();
        let [mul_res_limb_25_col165]: [QM31; 1] = (*mul_res_limb_25_col165.try_into().unwrap())
            .unbox();
        let [mul_res_limb_26_col166]: [QM31; 1] = (*mul_res_limb_26_col166.try_into().unwrap())
            .unbox();
        let [mul_res_limb_27_col167]: [QM31; 1] = (*mul_res_limb_27_col167.try_into().unwrap())
            .unbox();
        let [k_col168]: [QM31; 1] = (*k_col168.try_into().unwrap()).unbox();
        let [carry_0_col169]: [QM31; 1] = (*carry_0_col169.try_into().unwrap()).unbox();
        let [carry_1_col170]: [QM31; 1] = (*carry_1_col170.try_into().unwrap()).unbox();
        let [carry_2_col171]: [QM31; 1] = (*carry_2_col171.try_into().unwrap()).unbox();
        let [carry_3_col172]: [QM31; 1] = (*carry_3_col172.try_into().unwrap()).unbox();
        let [carry_4_col173]: [QM31; 1] = (*carry_4_col173.try_into().unwrap()).unbox();
        let [carry_5_col174]: [QM31; 1] = (*carry_5_col174.try_into().unwrap()).unbox();
        let [carry_6_col175]: [QM31; 1] = (*carry_6_col175.try_into().unwrap()).unbox();
        let [carry_7_col176]: [QM31; 1] = (*carry_7_col176.try_into().unwrap()).unbox();
        let [carry_8_col177]: [QM31; 1] = (*carry_8_col177.try_into().unwrap()).unbox();
        let [carry_9_col178]: [QM31; 1] = (*carry_9_col178.try_into().unwrap()).unbox();
        let [carry_10_col179]: [QM31; 1] = (*carry_10_col179.try_into().unwrap()).unbox();
        let [carry_11_col180]: [QM31; 1] = (*carry_11_col180.try_into().unwrap()).unbox();
        let [carry_12_col181]: [QM31; 1] = (*carry_12_col181.try_into().unwrap()).unbox();
        let [carry_13_col182]: [QM31; 1] = (*carry_13_col182.try_into().unwrap()).unbox();
        let [carry_14_col183]: [QM31; 1] = (*carry_14_col183.try_into().unwrap()).unbox();
        let [carry_15_col184]: [QM31; 1] = (*carry_15_col184.try_into().unwrap()).unbox();
        let [carry_16_col185]: [QM31; 1] = (*carry_16_col185.try_into().unwrap()).unbox();
        let [carry_17_col186]: [QM31; 1] = (*carry_17_col186.try_into().unwrap()).unbox();
        let [carry_18_col187]: [QM31; 1] = (*carry_18_col187.try_into().unwrap()).unbox();
        let [carry_19_col188]: [QM31; 1] = (*carry_19_col188.try_into().unwrap()).unbox();
        let [carry_20_col189]: [QM31; 1] = (*carry_20_col189.try_into().unwrap()).unbox();
        let [carry_21_col190]: [QM31; 1] = (*carry_21_col190.try_into().unwrap()).unbox();
        let [carry_22_col191]: [QM31; 1] = (*carry_22_col191.try_into().unwrap()).unbox();
        let [carry_23_col192]: [QM31; 1] = (*carry_23_col192.try_into().unwrap()).unbox();
        let [carry_24_col193]: [QM31; 1] = (*carry_24_col193.try_into().unwrap()).unbox();
        let [carry_25_col194]: [QM31; 1] = (*carry_25_col194.try_into().unwrap()).unbox();
        let [carry_26_col195]: [QM31; 1] = (*carry_26_col195.try_into().unwrap()).unbox();
        let [res_limb_0_col196]: [QM31; 1] = (*res_limb_0_col196.try_into().unwrap()).unbox();
        let [res_limb_1_col197]: [QM31; 1] = (*res_limb_1_col197.try_into().unwrap()).unbox();
        let [res_limb_2_col198]: [QM31; 1] = (*res_limb_2_col198.try_into().unwrap()).unbox();
        let [res_limb_3_col199]: [QM31; 1] = (*res_limb_3_col199.try_into().unwrap()).unbox();
        let [res_limb_4_col200]: [QM31; 1] = (*res_limb_4_col200.try_into().unwrap()).unbox();
        let [res_limb_5_col201]: [QM31; 1] = (*res_limb_5_col201.try_into().unwrap()).unbox();
        let [res_limb_6_col202]: [QM31; 1] = (*res_limb_6_col202.try_into().unwrap()).unbox();
        let [res_limb_7_col203]: [QM31; 1] = (*res_limb_7_col203.try_into().unwrap()).unbox();
        let [res_limb_8_col204]: [QM31; 1] = (*res_limb_8_col204.try_into().unwrap()).unbox();
        let [res_limb_9_col205]: [QM31; 1] = (*res_limb_9_col205.try_into().unwrap()).unbox();
        let [res_limb_10_col206]: [QM31; 1] = (*res_limb_10_col206.try_into().unwrap()).unbox();
        let [res_limb_11_col207]: [QM31; 1] = (*res_limb_11_col207.try_into().unwrap()).unbox();
        let [res_limb_12_col208]: [QM31; 1] = (*res_limb_12_col208.try_into().unwrap()).unbox();
        let [res_limb_13_col209]: [QM31; 1] = (*res_limb_13_col209.try_into().unwrap()).unbox();
        let [res_limb_14_col210]: [QM31; 1] = (*res_limb_14_col210.try_into().unwrap()).unbox();
        let [res_limb_15_col211]: [QM31; 1] = (*res_limb_15_col211.try_into().unwrap()).unbox();
        let [res_limb_16_col212]: [QM31; 1] = (*res_limb_16_col212.try_into().unwrap()).unbox();
        let [res_limb_17_col213]: [QM31; 1] = (*res_limb_17_col213.try_into().unwrap()).unbox();
        let [res_limb_18_col214]: [QM31; 1] = (*res_limb_18_col214.try_into().unwrap()).unbox();
        let [res_limb_19_col215]: [QM31; 1] = (*res_limb_19_col215.try_into().unwrap()).unbox();
        let [res_limb_20_col216]: [QM31; 1] = (*res_limb_20_col216.try_into().unwrap()).unbox();
        let [res_limb_21_col217]: [QM31; 1] = (*res_limb_21_col217.try_into().unwrap()).unbox();
        let [res_limb_22_col218]: [QM31; 1] = (*res_limb_22_col218.try_into().unwrap()).unbox();
        let [res_limb_23_col219]: [QM31; 1] = (*res_limb_23_col219.try_into().unwrap()).unbox();
        let [res_limb_24_col220]: [QM31; 1] = (*res_limb_24_col220.try_into().unwrap()).unbox();
        let [res_limb_25_col221]: [QM31; 1] = (*res_limb_25_col221.try_into().unwrap()).unbox();
        let [res_limb_26_col222]: [QM31; 1] = (*res_limb_26_col222.try_into().unwrap()).unbox();
        let [res_limb_27_col223]: [QM31; 1] = (*res_limb_27_col223.try_into().unwrap()).unbox();
        let [msb_col224]: [QM31; 1] = (*msb_col224.try_into().unwrap()).unbox();
        let [mid_limbs_set_col225]: [QM31; 1] = (*mid_limbs_set_col225.try_into().unwrap()).unbox();
        let [dst_sum_squares_inv_col226]: [QM31; 1] = (*dst_sum_squares_inv_col226
            .try_into()
            .unwrap())
            .unbox();
        let [dst_sum_inv_col227]: [QM31; 1] = (*dst_sum_inv_col227.try_into().unwrap()).unbox();
        let [op1_as_rel_imm_cond_col228]: [QM31; 1] = (*op1_as_rel_imm_cond_col228
            .try_into()
            .unwrap())
            .unbox();
        let [msb_col229]: [QM31; 1] = (*msb_col229.try_into().unwrap()).unbox();
        let [mid_limbs_set_col230]: [QM31; 1] = (*mid_limbs_set_col230.try_into().unwrap()).unbox();
        let [next_pc_jnz_col231]: [QM31; 1] = (*next_pc_jnz_col231.try_into().unwrap()).unbox();
        let [next_pc_col232]: [QM31; 1] = (*next_pc_col232.try_into().unwrap()).unbox();
        let [next_ap_col233]: [QM31; 1] = (*next_ap_col233.try_into().unwrap()).unbox();
        let [next_fp_col234]: [QM31; 1] = (*next_fp_col234.try_into().unwrap()).unbox();
        let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        let constraint_quotient = (enabler * enabler - enabler) * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        let output: [QM31; 8] = decode_generic_instruction_evaluate(
            [input_pc_col0],
            offset0_col3,
            offset1_col4,
            offset2_col5,
            dst_base_fp_col6,
            op0_base_fp_col7,
            op1_imm_col8,
            op1_base_fp_col9,
            op1_base_ap_col10,
            res_add_col11,
            res_mul_col12,
            pc_update_jump_col13,
            pc_update_jump_rel_col14,
            pc_update_jnz_col15,
            ap_update_add_col16,
            ap_update_add_1_col17,
            opcode_call_col18,
            opcode_ret_col19,
            opcode_assert_eq_col20,
            self.verify_instruction_lookup_elements,
            ref verify_instruction_sum_0,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let [
            decode_generic_instruction_output_tmp_57455_26_limb_15,
            decode_generic_instruction_output_tmp_57455_26_limb_16,
            decode_generic_instruction_output_tmp_57455_26_limb_17,
            decode_generic_instruction_output_tmp_57455_26_limb_18,
            decode_generic_instruction_output_tmp_57455_26_limb_19,
            decode_generic_instruction_output_tmp_57455_26_limb_20,
            decode_generic_instruction_output_tmp_57455_26_limb_21,
            decode_generic_instruction_output_tmp_57455_26_limb_22,
        ] =
            output;

        eval_operands_evaluate(
            [
                input_pc_col0, input_ap_col1, input_fp_col2, dst_base_fp_col6, op0_base_fp_col7,
                op1_imm_col8, op1_base_fp_col9, op1_base_ap_col10, res_add_col11, res_mul_col12,
                pc_update_jnz_col15, decode_generic_instruction_output_tmp_57455_26_limb_15,
                decode_generic_instruction_output_tmp_57455_26_limb_16,
                decode_generic_instruction_output_tmp_57455_26_limb_20,
                decode_generic_instruction_output_tmp_57455_26_limb_21,
                decode_generic_instruction_output_tmp_57455_26_limb_22,
            ],
            dst_src_col21,
            dst_id_col22,
            dst_limb_0_col23,
            dst_limb_1_col24,
            dst_limb_2_col25,
            dst_limb_3_col26,
            dst_limb_4_col27,
            dst_limb_5_col28,
            dst_limb_6_col29,
            dst_limb_7_col30,
            dst_limb_8_col31,
            dst_limb_9_col32,
            dst_limb_10_col33,
            dst_limb_11_col34,
            dst_limb_12_col35,
            dst_limb_13_col36,
            dst_limb_14_col37,
            dst_limb_15_col38,
            dst_limb_16_col39,
            dst_limb_17_col40,
            dst_limb_18_col41,
            dst_limb_19_col42,
            dst_limb_20_col43,
            dst_limb_21_col44,
            dst_limb_22_col45,
            dst_limb_23_col46,
            dst_limb_24_col47,
            dst_limb_25_col48,
            dst_limb_26_col49,
            dst_limb_27_col50,
            op0_src_col51,
            op0_id_col52,
            op0_limb_0_col53,
            op0_limb_1_col54,
            op0_limb_2_col55,
            op0_limb_3_col56,
            op0_limb_4_col57,
            op0_limb_5_col58,
            op0_limb_6_col59,
            op0_limb_7_col60,
            op0_limb_8_col61,
            op0_limb_9_col62,
            op0_limb_10_col63,
            op0_limb_11_col64,
            op0_limb_12_col65,
            op0_limb_13_col66,
            op0_limb_14_col67,
            op0_limb_15_col68,
            op0_limb_16_col69,
            op0_limb_17_col70,
            op0_limb_18_col71,
            op0_limb_19_col72,
            op0_limb_20_col73,
            op0_limb_21_col74,
            op0_limb_22_col75,
            op0_limb_23_col76,
            op0_limb_24_col77,
            op0_limb_25_col78,
            op0_limb_26_col79,
            op0_limb_27_col80,
            op1_src_col81,
            op1_id_col82,
            op1_limb_0_col83,
            op1_limb_1_col84,
            op1_limb_2_col85,
            op1_limb_3_col86,
            op1_limb_4_col87,
            op1_limb_5_col88,
            op1_limb_6_col89,
            op1_limb_7_col90,
            op1_limb_8_col91,
            op1_limb_9_col92,
            op1_limb_10_col93,
            op1_limb_11_col94,
            op1_limb_12_col95,
            op1_limb_13_col96,
            op1_limb_14_col97,
            op1_limb_15_col98,
            op1_limb_16_col99,
            op1_limb_17_col100,
            op1_limb_18_col101,
            op1_limb_19_col102,
            op1_limb_20_col103,
            op1_limb_21_col104,
            op1_limb_22_col105,
            op1_limb_23_col106,
            op1_limb_24_col107,
            op1_limb_25_col108,
            op1_limb_26_col109,
            op1_limb_27_col110,
            add_res_limb_0_col111,
            add_res_limb_1_col112,
            add_res_limb_2_col113,
            add_res_limb_3_col114,
            add_res_limb_4_col115,
            add_res_limb_5_col116,
            add_res_limb_6_col117,
            add_res_limb_7_col118,
            add_res_limb_8_col119,
            add_res_limb_9_col120,
            add_res_limb_10_col121,
            add_res_limb_11_col122,
            add_res_limb_12_col123,
            add_res_limb_13_col124,
            add_res_limb_14_col125,
            add_res_limb_15_col126,
            add_res_limb_16_col127,
            add_res_limb_17_col128,
            add_res_limb_18_col129,
            add_res_limb_19_col130,
            add_res_limb_20_col131,
            add_res_limb_21_col132,
            add_res_limb_22_col133,
            add_res_limb_23_col134,
            add_res_limb_24_col135,
            add_res_limb_25_col136,
            add_res_limb_26_col137,
            add_res_limb_27_col138,
            sub_p_bit_col139,
            mul_res_limb_0_col140,
            mul_res_limb_1_col141,
            mul_res_limb_2_col142,
            mul_res_limb_3_col143,
            mul_res_limb_4_col144,
            mul_res_limb_5_col145,
            mul_res_limb_6_col146,
            mul_res_limb_7_col147,
            mul_res_limb_8_col148,
            mul_res_limb_9_col149,
            mul_res_limb_10_col150,
            mul_res_limb_11_col151,
            mul_res_limb_12_col152,
            mul_res_limb_13_col153,
            mul_res_limb_14_col154,
            mul_res_limb_15_col155,
            mul_res_limb_16_col156,
            mul_res_limb_17_col157,
            mul_res_limb_18_col158,
            mul_res_limb_19_col159,
            mul_res_limb_20_col160,
            mul_res_limb_21_col161,
            mul_res_limb_22_col162,
            mul_res_limb_23_col163,
            mul_res_limb_24_col164,
            mul_res_limb_25_col165,
            mul_res_limb_26_col166,
            mul_res_limb_27_col167,
            k_col168,
            carry_0_col169,
            carry_1_col170,
            carry_2_col171,
            carry_3_col172,
            carry_4_col173,
            carry_5_col174,
            carry_6_col175,
            carry_7_col176,
            carry_8_col177,
            carry_9_col178,
            carry_10_col179,
            carry_11_col180,
            carry_12_col181,
            carry_13_col182,
            carry_14_col183,
            carry_15_col184,
            carry_16_col185,
            carry_17_col186,
            carry_18_col187,
            carry_19_col188,
            carry_20_col189,
            carry_21_col190,
            carry_22_col191,
            carry_23_col192,
            carry_24_col193,
            carry_25_col194,
            carry_26_col195,
            res_limb_0_col196,
            res_limb_1_col197,
            res_limb_2_col198,
            res_limb_3_col199,
            res_limb_4_col200,
            res_limb_5_col201,
            res_limb_6_col202,
            res_limb_7_col203,
            res_limb_8_col204,
            res_limb_9_col205,
            res_limb_10_col206,
            res_limb_11_col207,
            res_limb_12_col208,
            res_limb_13_col209,
            res_limb_14_col210,
            res_limb_15_col211,
            res_limb_16_col212,
            res_limb_17_col213,
            res_limb_18_col214,
            res_limb_19_col215,
            res_limb_20_col216,
            res_limb_21_col217,
            res_limb_22_col218,
            res_limb_23_col219,
            res_limb_24_col220,
            res_limb_25_col221,
            res_limb_26_col222,
            res_limb_27_col223,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            self.range_check_9_9_lookup_elements,
            self.range_check_19_lookup_elements,
            ref memory_address_to_id_sum_1,
            ref memory_id_to_big_sum_2,
            ref memory_address_to_id_sum_3,
            ref memory_id_to_big_sum_4,
            ref memory_address_to_id_sum_5,
            ref memory_id_to_big_sum_6,
            ref range_check_9_9_sum_7,
            ref range_check_9_9_sum_8,
            ref range_check_9_9_sum_9,
            ref range_check_9_9_sum_10,
            ref range_check_9_9_sum_11,
            ref range_check_9_9_sum_12,
            ref range_check_9_9_sum_13,
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
            ref range_check_9_9_sum_28,
            ref range_check_9_9_sum_29,
            ref range_check_9_9_sum_30,
            ref range_check_9_9_sum_31,
            ref range_check_9_9_sum_32,
            ref range_check_9_9_sum_33,
            ref range_check_9_9_sum_34,
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
            ref range_check_19_sum_56,
            ref range_check_19_sum_57,
            ref range_check_19_sum_58,
            ref range_check_19_sum_59,
            ref range_check_19_sum_60,
            ref range_check_19_sum_61,
            ref range_check_19_sum_62,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        handle_opcodes_evaluate(
            [
                input_pc_col0, input_fp_col2, dst_base_fp_col6, op0_base_fp_col7, op1_base_fp_col9,
                pc_update_jump_col13, opcode_call_col18, opcode_ret_col19, opcode_assert_eq_col20,
                decode_generic_instruction_output_tmp_57455_26_limb_16,
                decode_generic_instruction_output_tmp_57455_26_limb_19,
                decode_generic_instruction_output_tmp_57455_26_limb_20,
                decode_generic_instruction_output_tmp_57455_26_limb_21,
                decode_generic_instruction_output_tmp_57455_26_limb_22, dst_limb_0_col23,
                dst_limb_1_col24, dst_limb_2_col25, dst_limb_3_col26, dst_limb_4_col27,
                dst_limb_5_col28, dst_limb_6_col29, dst_limb_7_col30, dst_limb_8_col31,
                dst_limb_9_col32, dst_limb_10_col33, dst_limb_11_col34, dst_limb_12_col35,
                dst_limb_13_col36, dst_limb_14_col37, dst_limb_15_col38, dst_limb_16_col39,
                dst_limb_17_col40, dst_limb_18_col41, dst_limb_19_col42, dst_limb_20_col43,
                dst_limb_21_col44, dst_limb_22_col45, dst_limb_23_col46, dst_limb_24_col47,
                dst_limb_25_col48, dst_limb_26_col49, dst_limb_27_col50, op0_limb_0_col53,
                op0_limb_1_col54, op0_limb_2_col55, op0_limb_3_col56, op0_limb_4_col57,
                op0_limb_5_col58, op0_limb_6_col59, op0_limb_7_col60, op0_limb_8_col61,
                op0_limb_9_col62, op0_limb_10_col63, op0_limb_11_col64, op0_limb_12_col65,
                op0_limb_13_col66, op0_limb_14_col67, op0_limb_15_col68, op0_limb_16_col69,
                op0_limb_17_col70, op0_limb_18_col71, op0_limb_19_col72, op0_limb_20_col73,
                op0_limb_21_col74, op0_limb_22_col75, op0_limb_23_col76, op0_limb_24_col77,
                op0_limb_25_col78, op0_limb_26_col79, op0_limb_27_col80, res_limb_0_col196,
                res_limb_1_col197, res_limb_2_col198, res_limb_3_col199, res_limb_4_col200,
                res_limb_5_col201, res_limb_6_col202, res_limb_7_col203, res_limb_8_col204,
                res_limb_9_col205, res_limb_10_col206, res_limb_11_col207, res_limb_12_col208,
                res_limb_13_col209, res_limb_14_col210, res_limb_15_col211, res_limb_16_col212,
                res_limb_17_col213, res_limb_18_col214, res_limb_19_col215, res_limb_20_col216,
                res_limb_21_col217, res_limb_22_col218, res_limb_23_col219, res_limb_24_col220,
                res_limb_25_col221, res_limb_26_col222, res_limb_27_col223,
            ],
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        update_registers_evaluate(
            [
                input_pc_col0, input_ap_col1, input_fp_col2, pc_update_jump_col13,
                pc_update_jump_rel_col14, pc_update_jnz_col15, ap_update_add_col16,
                ap_update_add_1_col17, opcode_call_col18, opcode_ret_col19,
                decode_generic_instruction_output_tmp_57455_26_limb_17,
                decode_generic_instruction_output_tmp_57455_26_limb_18,
                decode_generic_instruction_output_tmp_57455_26_limb_19, dst_limb_0_col23,
                dst_limb_1_col24, dst_limb_2_col25, dst_limb_3_col26, dst_limb_4_col27,
                dst_limb_5_col28, dst_limb_6_col29, dst_limb_7_col30, dst_limb_8_col31,
                dst_limb_9_col32, dst_limb_10_col33, dst_limb_11_col34, dst_limb_12_col35,
                dst_limb_13_col36, dst_limb_14_col37, dst_limb_15_col38, dst_limb_16_col39,
                dst_limb_17_col40, dst_limb_18_col41, dst_limb_19_col42, dst_limb_20_col43,
                dst_limb_21_col44, dst_limb_22_col45, dst_limb_23_col46, dst_limb_24_col47,
                dst_limb_25_col48, dst_limb_26_col49, dst_limb_27_col50, op1_limb_0_col83,
                op1_limb_1_col84, op1_limb_2_col85, op1_limb_3_col86, op1_limb_4_col87,
                op1_limb_5_col88, op1_limb_6_col89, op1_limb_7_col90, op1_limb_8_col91,
                op1_limb_9_col92, op1_limb_10_col93, op1_limb_11_col94, op1_limb_12_col95,
                op1_limb_13_col96, op1_limb_14_col97, op1_limb_15_col98, op1_limb_16_col99,
                op1_limb_17_col100, op1_limb_18_col101, op1_limb_19_col102, op1_limb_20_col103,
                op1_limb_21_col104, op1_limb_22_col105, op1_limb_23_col106, op1_limb_24_col107,
                op1_limb_25_col108, op1_limb_26_col109, op1_limb_27_col110, res_limb_0_col196,
                res_limb_1_col197, res_limb_2_col198, res_limb_3_col199, res_limb_4_col200,
                res_limb_5_col201, res_limb_6_col202, res_limb_7_col203, res_limb_8_col204,
                res_limb_9_col205, res_limb_10_col206, res_limb_11_col207, res_limb_12_col208,
                res_limb_13_col209, res_limb_14_col210, res_limb_15_col211, res_limb_16_col212,
                res_limb_17_col213, res_limb_18_col214, res_limb_19_col215, res_limb_20_col216,
                res_limb_21_col217, res_limb_22_col218, res_limb_23_col219, res_limb_24_col220,
                res_limb_25_col221, res_limb_26_col222, res_limb_27_col223,
            ],
            msb_col224,
            mid_limbs_set_col225,
            dst_sum_squares_inv_col226,
            dst_sum_inv_col227,
            op1_as_rel_imm_cond_col228,
            msb_col229,
            mid_limbs_set_col230,
            next_pc_jnz_col231,
            next_pc_col232,
            next_ap_col233,
            next_fp_col234,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        opcodes_sum_63 = self
            .opcodes_lookup_elements
            .combine_qm31([input_pc_col0, input_ap_col1, input_fp_col2]);

        opcodes_sum_64 = self
            .opcodes_lookup_elements
            .combine_qm31([next_pc_col232, next_ap_col233, next_fp_col234]);

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
            range_check_9_9_sum_28,
            range_check_9_9_sum_29,
            range_check_9_9_sum_30,
            range_check_9_9_sum_31,
            range_check_9_9_sum_32,
            range_check_9_9_sum_33,
            range_check_9_9_sum_34,
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
            range_check_19_sum_56,
            range_check_19_sum_57,
            range_check_19_sum_58,
            range_check_19_sum_59,
            range_check_19_sum_60,
            range_check_19_sum_61,
            range_check_19_sum_62,
            opcodes_sum_63,
            opcodes_sum_64,
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
    range_check_9_9_sum_28: QM31,
    range_check_9_9_sum_29: QM31,
    range_check_9_9_sum_30: QM31,
    range_check_9_9_sum_31: QM31,
    range_check_9_9_sum_32: QM31,
    range_check_9_9_sum_33: QM31,
    range_check_9_9_sum_34: QM31,
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
    range_check_19_sum_56: QM31,
    range_check_19_sum_57: QM31,
    range_check_19_sum_58: QM31,
    range_check_19_sum_59: QM31,
    range_check_19_sum_60: QM31,
    range_check_19_sum_61: QM31,
    range_check_19_sum_62: QM31,
    opcodes_sum_63: QM31,
    opcodes_sum_64: QM31,
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
    ]: [Span<QM31>; 132] =
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
    let [trace_2_col128_neg1, trace_2_col128]: [QM31; 2] = (*trace_2_col128.try_into().unwrap())
        .unbox();
    let [trace_2_col129_neg1, trace_2_col129]: [QM31; 2] = (*trace_2_col129.try_into().unwrap())
        .unbox();
    let [trace_2_col130_neg1, trace_2_col130]: [QM31; 2] = (*trace_2_col130.try_into().unwrap())
        .unbox();
    let [trace_2_col131_neg1, trace_2_col131]: [QM31; 2] = (*trace_2_col131.try_into().unwrap())
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
        * range_check_9_9_sum_7)
        - memory_id_to_big_sum_6
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
        * range_check_9_9_sum_28
        * range_check_9_9_sum_29)
        - range_check_9_9_sum_28
        - range_check_9_9_sum_29)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col60, trace_2_col61, trace_2_col62, trace_2_col63],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col56, trace_2_col57, trace_2_col58, trace_2_col59],
        ))
        * range_check_9_9_sum_30
        * range_check_9_9_sum_31)
        - range_check_9_9_sum_30
        - range_check_9_9_sum_31)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col64, trace_2_col65, trace_2_col66, trace_2_col67],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col60, trace_2_col61, trace_2_col62, trace_2_col63],
        ))
        * range_check_9_9_sum_32
        * range_check_9_9_sum_33)
        - range_check_9_9_sum_32
        - range_check_9_9_sum_33)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col68, trace_2_col69, trace_2_col70, trace_2_col71],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col64, trace_2_col65, trace_2_col66, trace_2_col67],
        ))
        * range_check_9_9_sum_34
        * range_check_19_sum_35)
        - range_check_9_9_sum_34
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
        * range_check_19_sum_56
        * range_check_19_sum_57)
        - range_check_19_sum_56
        - range_check_19_sum_57)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col116, trace_2_col117, trace_2_col118, trace_2_col119],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col112, trace_2_col113, trace_2_col114, trace_2_col115],
        ))
        * range_check_19_sum_58
        * range_check_19_sum_59)
        - range_check_19_sum_58
        - range_check_19_sum_59)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col120, trace_2_col121, trace_2_col122, trace_2_col123],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col116, trace_2_col117, trace_2_col118, trace_2_col119],
        ))
        * range_check_19_sum_60
        * range_check_19_sum_61)
        - range_check_19_sum_60
        - range_check_19_sum_61)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col124, trace_2_col125, trace_2_col126, trace_2_col127],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col120, trace_2_col121, trace_2_col122, trace_2_col123],
        ))
        * range_check_19_sum_62
        * opcodes_sum_63)
        - (range_check_19_sum_62 * enabler)
        - opcodes_sum_63)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col128, trace_2_col129, trace_2_col130, trace_2_col131],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col124, trace_2_col125, trace_2_col126, trace_2_col127],
        )
        - QM31Impl::from_partial_evals(
            [trace_2_col128_neg1, trace_2_col129_neg1, trace_2_col130_neg1, trace_2_col131_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * opcodes_sum_64)
        + enabler)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
}
