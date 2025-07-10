// AIR version aca38612
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
use crate::PreprocessedColumnTrait;
use crate::cairo_component::CairoComponent;
use crate::components::subroutines::create_blake_output::create_blake_output_evaluate;
use crate::components::subroutines::create_blake_round_input::create_blake_round_input_evaluate;
use crate::components::subroutines::decode_blake_opcode::decode_blake_opcode_evaluate;
use crate::components::subroutines::verify_blake_word::verify_blake_word_evaluate;

pub const N_TRACE_COLUMNS: usize = 169;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 8] = [
    ('VerifyInstruction', 1), ('MemoryAddressToId', 20), ('MemoryIdToBig', 20),
    ('RangeCheck_7_2_5', 17), ('VerifyBitwiseXor_8', 4), ('BlakeRound', 1), ('TripleXor32', 8),
    ('Opcodes', 1),
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
        let trace_log_sizes = [log_size; N_TRACE_COLUMNS].span();
        let interaction_log_sizes = [log_size; 148].span();
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
    pub range_check_7_2_5_lookup_elements: crate::RangeCheck_7_2_5Elements,
    pub verify_bitwise_xor_8_lookup_elements: crate::VerifyBitwiseXor_8Elements,
    pub blake_round_lookup_elements: crate::BlakeRoundElements,
    pub triple_xor_32_lookup_elements: crate::TripleXor32Elements,
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
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
        interaction_trace_mask_points.append(array![point]);
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
        let mut verify_bitwise_xor_8_sum_34: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_sum_35: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_sum_36: QM31 = Zero::zero();
        let mut verify_bitwise_xor_8_sum_37: QM31 = Zero::zero();
        let mut blake_round_sum_38: QM31 = Zero::zero();
        let mut blake_round_sum_39: QM31 = Zero::zero();
        let mut triple_xor_32_sum_40: QM31 = Zero::zero();
        let mut triple_xor_32_sum_41: QM31 = Zero::zero();
        let mut triple_xor_32_sum_42: QM31 = Zero::zero();
        let mut triple_xor_32_sum_43: QM31 = Zero::zero();
        let mut triple_xor_32_sum_44: QM31 = Zero::zero();
        let mut triple_xor_32_sum_45: QM31 = Zero::zero();
        let mut triple_xor_32_sum_46: QM31 = Zero::zero();
        let mut triple_xor_32_sum_47: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_48: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_49: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_50: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_51: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_52: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_53: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_54: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_55: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_56: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_57: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_58: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_59: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_60: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_61: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_62: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_63: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_64: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_65: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_66: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_67: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_68: QM31 = Zero::zero();
        let mut range_check_7_2_5_sum_69: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_70: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_71: QM31 = Zero::zero();
        let mut opcodes_sum_72: QM31 = Zero::zero();
        let mut opcodes_sum_73: QM31 = Zero::zero();
        let seq = preprocessed_mask_values.get(PreprocessedColumn::Seq(*(self.claim.log_size)));

        let [
            input_pc_col0,
            input_ap_col1,
            input_fp_col2,
            offset0_col3,
            offset1_col4,
            offset2_col5,
            dst_base_fp_col6,
            op0_base_fp_col7,
            op1_base_fp_col8,
            op1_base_ap_col9,
            ap_update_add_1_col10,
            opcode_extension_col11,
            mem0_base_col12,
            op0_id_col13,
            op0_limb_0_col14,
            op0_limb_1_col15,
            op0_limb_2_col16,
            mem1_base_col17,
            op1_id_col18,
            op1_limb_0_col19,
            op1_limb_1_col20,
            op1_limb_2_col21,
            ap_id_col22,
            ap_limb_0_col23,
            ap_limb_1_col24,
            ap_limb_2_col25,
            mem_dst_base_col26,
            low_16_bits_col27,
            high_16_bits_col28,
            low_7_ms_bits_col29,
            high_14_ms_bits_col30,
            high_5_ms_bits_col31,
            dst_id_col32,
            low_16_bits_col33,
            high_16_bits_col34,
            low_7_ms_bits_col35,
            high_14_ms_bits_col36,
            high_5_ms_bits_col37,
            state_0_id_col38,
            low_16_bits_col39,
            high_16_bits_col40,
            low_7_ms_bits_col41,
            high_14_ms_bits_col42,
            high_5_ms_bits_col43,
            state_1_id_col44,
            low_16_bits_col45,
            high_16_bits_col46,
            low_7_ms_bits_col47,
            high_14_ms_bits_col48,
            high_5_ms_bits_col49,
            state_2_id_col50,
            low_16_bits_col51,
            high_16_bits_col52,
            low_7_ms_bits_col53,
            high_14_ms_bits_col54,
            high_5_ms_bits_col55,
            state_3_id_col56,
            low_16_bits_col57,
            high_16_bits_col58,
            low_7_ms_bits_col59,
            high_14_ms_bits_col60,
            high_5_ms_bits_col61,
            state_4_id_col62,
            low_16_bits_col63,
            high_16_bits_col64,
            low_7_ms_bits_col65,
            high_14_ms_bits_col66,
            high_5_ms_bits_col67,
            state_5_id_col68,
            low_16_bits_col69,
            high_16_bits_col70,
            low_7_ms_bits_col71,
            high_14_ms_bits_col72,
            high_5_ms_bits_col73,
            state_6_id_col74,
            low_16_bits_col75,
            high_16_bits_col76,
            low_7_ms_bits_col77,
            high_14_ms_bits_col78,
            high_5_ms_bits_col79,
            state_7_id_col80,
            ms_8_bits_col81,
            ms_8_bits_col82,
            xor_col83,
            xor_col84,
            xor_col85,
            xor_col86,
            blake_round_output_limb_0_col87,
            blake_round_output_limb_1_col88,
            blake_round_output_limb_2_col89,
            blake_round_output_limb_3_col90,
            blake_round_output_limb_4_col91,
            blake_round_output_limb_5_col92,
            blake_round_output_limb_6_col93,
            blake_round_output_limb_7_col94,
            blake_round_output_limb_8_col95,
            blake_round_output_limb_9_col96,
            blake_round_output_limb_10_col97,
            blake_round_output_limb_11_col98,
            blake_round_output_limb_12_col99,
            blake_round_output_limb_13_col100,
            blake_round_output_limb_14_col101,
            blake_round_output_limb_15_col102,
            blake_round_output_limb_16_col103,
            blake_round_output_limb_17_col104,
            blake_round_output_limb_18_col105,
            blake_round_output_limb_19_col106,
            blake_round_output_limb_20_col107,
            blake_round_output_limb_21_col108,
            blake_round_output_limb_22_col109,
            blake_round_output_limb_23_col110,
            blake_round_output_limb_24_col111,
            blake_round_output_limb_25_col112,
            blake_round_output_limb_26_col113,
            blake_round_output_limb_27_col114,
            blake_round_output_limb_28_col115,
            blake_round_output_limb_29_col116,
            blake_round_output_limb_30_col117,
            blake_round_output_limb_31_col118,
            blake_round_output_limb_32_col119,
            triple_xor_32_output_limb_0_col120,
            triple_xor_32_output_limb_1_col121,
            triple_xor_32_output_limb_0_col122,
            triple_xor_32_output_limb_1_col123,
            triple_xor_32_output_limb_0_col124,
            triple_xor_32_output_limb_1_col125,
            triple_xor_32_output_limb_0_col126,
            triple_xor_32_output_limb_1_col127,
            triple_xor_32_output_limb_0_col128,
            triple_xor_32_output_limb_1_col129,
            triple_xor_32_output_limb_0_col130,
            triple_xor_32_output_limb_1_col131,
            triple_xor_32_output_limb_0_col132,
            triple_xor_32_output_limb_1_col133,
            triple_xor_32_output_limb_0_col134,
            triple_xor_32_output_limb_1_col135,
            low_7_ms_bits_col136,
            high_14_ms_bits_col137,
            high_5_ms_bits_col138,
            new_state_0_id_col139,
            low_7_ms_bits_col140,
            high_14_ms_bits_col141,
            high_5_ms_bits_col142,
            new_state_1_id_col143,
            low_7_ms_bits_col144,
            high_14_ms_bits_col145,
            high_5_ms_bits_col146,
            new_state_2_id_col147,
            low_7_ms_bits_col148,
            high_14_ms_bits_col149,
            high_5_ms_bits_col150,
            new_state_3_id_col151,
            low_7_ms_bits_col152,
            high_14_ms_bits_col153,
            high_5_ms_bits_col154,
            new_state_4_id_col155,
            low_7_ms_bits_col156,
            high_14_ms_bits_col157,
            high_5_ms_bits_col158,
            new_state_5_id_col159,
            low_7_ms_bits_col160,
            high_14_ms_bits_col161,
            high_5_ms_bits_col162,
            new_state_6_id_col163,
            low_7_ms_bits_col164,
            high_14_ms_bits_col165,
            high_5_ms_bits_col166,
            new_state_7_id_col167,
            enabler,
        ]: [Span<QM31>; 169] =
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
        let [op1_base_fp_col8]: [QM31; 1] = (*op1_base_fp_col8.try_into().unwrap()).unbox();
        let [op1_base_ap_col9]: [QM31; 1] = (*op1_base_ap_col9.try_into().unwrap()).unbox();
        let [ap_update_add_1_col10]: [QM31; 1] = (*ap_update_add_1_col10.try_into().unwrap())
            .unbox();
        let [opcode_extension_col11]: [QM31; 1] = (*opcode_extension_col11.try_into().unwrap())
            .unbox();
        let [mem0_base_col12]: [QM31; 1] = (*mem0_base_col12.try_into().unwrap()).unbox();
        let [op0_id_col13]: [QM31; 1] = (*op0_id_col13.try_into().unwrap()).unbox();
        let [op0_limb_0_col14]: [QM31; 1] = (*op0_limb_0_col14.try_into().unwrap()).unbox();
        let [op0_limb_1_col15]: [QM31; 1] = (*op0_limb_1_col15.try_into().unwrap()).unbox();
        let [op0_limb_2_col16]: [QM31; 1] = (*op0_limb_2_col16.try_into().unwrap()).unbox();
        let [mem1_base_col17]: [QM31; 1] = (*mem1_base_col17.try_into().unwrap()).unbox();
        let [op1_id_col18]: [QM31; 1] = (*op1_id_col18.try_into().unwrap()).unbox();
        let [op1_limb_0_col19]: [QM31; 1] = (*op1_limb_0_col19.try_into().unwrap()).unbox();
        let [op1_limb_1_col20]: [QM31; 1] = (*op1_limb_1_col20.try_into().unwrap()).unbox();
        let [op1_limb_2_col21]: [QM31; 1] = (*op1_limb_2_col21.try_into().unwrap()).unbox();
        let [ap_id_col22]: [QM31; 1] = (*ap_id_col22.try_into().unwrap()).unbox();
        let [ap_limb_0_col23]: [QM31; 1] = (*ap_limb_0_col23.try_into().unwrap()).unbox();
        let [ap_limb_1_col24]: [QM31; 1] = (*ap_limb_1_col24.try_into().unwrap()).unbox();
        let [ap_limb_2_col25]: [QM31; 1] = (*ap_limb_2_col25.try_into().unwrap()).unbox();
        let [mem_dst_base_col26]: [QM31; 1] = (*mem_dst_base_col26.try_into().unwrap()).unbox();
        let [low_16_bits_col27]: [QM31; 1] = (*low_16_bits_col27.try_into().unwrap()).unbox();
        let [high_16_bits_col28]: [QM31; 1] = (*high_16_bits_col28.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col29]: [QM31; 1] = (*low_7_ms_bits_col29.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col30]: [QM31; 1] = (*high_14_ms_bits_col30.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col31]: [QM31; 1] = (*high_5_ms_bits_col31.try_into().unwrap()).unbox();
        let [dst_id_col32]: [QM31; 1] = (*dst_id_col32.try_into().unwrap()).unbox();
        let [low_16_bits_col33]: [QM31; 1] = (*low_16_bits_col33.try_into().unwrap()).unbox();
        let [high_16_bits_col34]: [QM31; 1] = (*high_16_bits_col34.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col35]: [QM31; 1] = (*low_7_ms_bits_col35.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col36]: [QM31; 1] = (*high_14_ms_bits_col36.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col37]: [QM31; 1] = (*high_5_ms_bits_col37.try_into().unwrap()).unbox();
        let [state_0_id_col38]: [QM31; 1] = (*state_0_id_col38.try_into().unwrap()).unbox();
        let [low_16_bits_col39]: [QM31; 1] = (*low_16_bits_col39.try_into().unwrap()).unbox();
        let [high_16_bits_col40]: [QM31; 1] = (*high_16_bits_col40.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col41]: [QM31; 1] = (*low_7_ms_bits_col41.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col42]: [QM31; 1] = (*high_14_ms_bits_col42.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col43]: [QM31; 1] = (*high_5_ms_bits_col43.try_into().unwrap()).unbox();
        let [state_1_id_col44]: [QM31; 1] = (*state_1_id_col44.try_into().unwrap()).unbox();
        let [low_16_bits_col45]: [QM31; 1] = (*low_16_bits_col45.try_into().unwrap()).unbox();
        let [high_16_bits_col46]: [QM31; 1] = (*high_16_bits_col46.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col47]: [QM31; 1] = (*low_7_ms_bits_col47.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col48]: [QM31; 1] = (*high_14_ms_bits_col48.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col49]: [QM31; 1] = (*high_5_ms_bits_col49.try_into().unwrap()).unbox();
        let [state_2_id_col50]: [QM31; 1] = (*state_2_id_col50.try_into().unwrap()).unbox();
        let [low_16_bits_col51]: [QM31; 1] = (*low_16_bits_col51.try_into().unwrap()).unbox();
        let [high_16_bits_col52]: [QM31; 1] = (*high_16_bits_col52.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col53]: [QM31; 1] = (*low_7_ms_bits_col53.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col54]: [QM31; 1] = (*high_14_ms_bits_col54.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col55]: [QM31; 1] = (*high_5_ms_bits_col55.try_into().unwrap()).unbox();
        let [state_3_id_col56]: [QM31; 1] = (*state_3_id_col56.try_into().unwrap()).unbox();
        let [low_16_bits_col57]: [QM31; 1] = (*low_16_bits_col57.try_into().unwrap()).unbox();
        let [high_16_bits_col58]: [QM31; 1] = (*high_16_bits_col58.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col59]: [QM31; 1] = (*low_7_ms_bits_col59.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col60]: [QM31; 1] = (*high_14_ms_bits_col60.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col61]: [QM31; 1] = (*high_5_ms_bits_col61.try_into().unwrap()).unbox();
        let [state_4_id_col62]: [QM31; 1] = (*state_4_id_col62.try_into().unwrap()).unbox();
        let [low_16_bits_col63]: [QM31; 1] = (*low_16_bits_col63.try_into().unwrap()).unbox();
        let [high_16_bits_col64]: [QM31; 1] = (*high_16_bits_col64.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col65]: [QM31; 1] = (*low_7_ms_bits_col65.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col66]: [QM31; 1] = (*high_14_ms_bits_col66.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col67]: [QM31; 1] = (*high_5_ms_bits_col67.try_into().unwrap()).unbox();
        let [state_5_id_col68]: [QM31; 1] = (*state_5_id_col68.try_into().unwrap()).unbox();
        let [low_16_bits_col69]: [QM31; 1] = (*low_16_bits_col69.try_into().unwrap()).unbox();
        let [high_16_bits_col70]: [QM31; 1] = (*high_16_bits_col70.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col71]: [QM31; 1] = (*low_7_ms_bits_col71.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col72]: [QM31; 1] = (*high_14_ms_bits_col72.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col73]: [QM31; 1] = (*high_5_ms_bits_col73.try_into().unwrap()).unbox();
        let [state_6_id_col74]: [QM31; 1] = (*state_6_id_col74.try_into().unwrap()).unbox();
        let [low_16_bits_col75]: [QM31; 1] = (*low_16_bits_col75.try_into().unwrap()).unbox();
        let [high_16_bits_col76]: [QM31; 1] = (*high_16_bits_col76.try_into().unwrap()).unbox();
        let [low_7_ms_bits_col77]: [QM31; 1] = (*low_7_ms_bits_col77.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col78]: [QM31; 1] = (*high_14_ms_bits_col78.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col79]: [QM31; 1] = (*high_5_ms_bits_col79.try_into().unwrap()).unbox();
        let [state_7_id_col80]: [QM31; 1] = (*state_7_id_col80.try_into().unwrap()).unbox();
        let [ms_8_bits_col81]: [QM31; 1] = (*ms_8_bits_col81.try_into().unwrap()).unbox();
        let [ms_8_bits_col82]: [QM31; 1] = (*ms_8_bits_col82.try_into().unwrap()).unbox();
        let [xor_col83]: [QM31; 1] = (*xor_col83.try_into().unwrap()).unbox();
        let [xor_col84]: [QM31; 1] = (*xor_col84.try_into().unwrap()).unbox();
        let [xor_col85]: [QM31; 1] = (*xor_col85.try_into().unwrap()).unbox();
        let [xor_col86]: [QM31; 1] = (*xor_col86.try_into().unwrap()).unbox();
        let [blake_round_output_limb_0_col87]: [QM31; 1] = (*blake_round_output_limb_0_col87
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_1_col88]: [QM31; 1] = (*blake_round_output_limb_1_col88
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_2_col89]: [QM31; 1] = (*blake_round_output_limb_2_col89
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_3_col90]: [QM31; 1] = (*blake_round_output_limb_3_col90
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_4_col91]: [QM31; 1] = (*blake_round_output_limb_4_col91
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_5_col92]: [QM31; 1] = (*blake_round_output_limb_5_col92
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_6_col93]: [QM31; 1] = (*blake_round_output_limb_6_col93
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_7_col94]: [QM31; 1] = (*blake_round_output_limb_7_col94
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_8_col95]: [QM31; 1] = (*blake_round_output_limb_8_col95
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_9_col96]: [QM31; 1] = (*blake_round_output_limb_9_col96
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_10_col97]: [QM31; 1] = (*blake_round_output_limb_10_col97
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_11_col98]: [QM31; 1] = (*blake_round_output_limb_11_col98
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_12_col99]: [QM31; 1] = (*blake_round_output_limb_12_col99
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_13_col100]: [QM31; 1] = (*blake_round_output_limb_13_col100
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_14_col101]: [QM31; 1] = (*blake_round_output_limb_14_col101
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_15_col102]: [QM31; 1] = (*blake_round_output_limb_15_col102
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_16_col103]: [QM31; 1] = (*blake_round_output_limb_16_col103
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_17_col104]: [QM31; 1] = (*blake_round_output_limb_17_col104
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_18_col105]: [QM31; 1] = (*blake_round_output_limb_18_col105
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_19_col106]: [QM31; 1] = (*blake_round_output_limb_19_col106
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_20_col107]: [QM31; 1] = (*blake_round_output_limb_20_col107
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_21_col108]: [QM31; 1] = (*blake_round_output_limb_21_col108
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_22_col109]: [QM31; 1] = (*blake_round_output_limb_22_col109
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_23_col110]: [QM31; 1] = (*blake_round_output_limb_23_col110
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_24_col111]: [QM31; 1] = (*blake_round_output_limb_24_col111
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_25_col112]: [QM31; 1] = (*blake_round_output_limb_25_col112
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_26_col113]: [QM31; 1] = (*blake_round_output_limb_26_col113
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_27_col114]: [QM31; 1] = (*blake_round_output_limb_27_col114
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_28_col115]: [QM31; 1] = (*blake_round_output_limb_28_col115
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_29_col116]: [QM31; 1] = (*blake_round_output_limb_29_col116
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_30_col117]: [QM31; 1] = (*blake_round_output_limb_30_col117
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_31_col118]: [QM31; 1] = (*blake_round_output_limb_31_col118
            .try_into()
            .unwrap())
            .unbox();
        let [blake_round_output_limb_32_col119]: [QM31; 1] = (*blake_round_output_limb_32_col119
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_0_col120]: [QM31; 1] = (*triple_xor_32_output_limb_0_col120
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_1_col121]: [QM31; 1] = (*triple_xor_32_output_limb_1_col121
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_0_col122]: [QM31; 1] = (*triple_xor_32_output_limb_0_col122
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_1_col123]: [QM31; 1] = (*triple_xor_32_output_limb_1_col123
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_0_col124]: [QM31; 1] = (*triple_xor_32_output_limb_0_col124
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_1_col125]: [QM31; 1] = (*triple_xor_32_output_limb_1_col125
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_0_col126]: [QM31; 1] = (*triple_xor_32_output_limb_0_col126
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_1_col127]: [QM31; 1] = (*triple_xor_32_output_limb_1_col127
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_0_col128]: [QM31; 1] = (*triple_xor_32_output_limb_0_col128
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_1_col129]: [QM31; 1] = (*triple_xor_32_output_limb_1_col129
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_0_col130]: [QM31; 1] = (*triple_xor_32_output_limb_0_col130
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_1_col131]: [QM31; 1] = (*triple_xor_32_output_limb_1_col131
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_0_col132]: [QM31; 1] = (*triple_xor_32_output_limb_0_col132
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_1_col133]: [QM31; 1] = (*triple_xor_32_output_limb_1_col133
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_0_col134]: [QM31; 1] = (*triple_xor_32_output_limb_0_col134
            .try_into()
            .unwrap())
            .unbox();
        let [triple_xor_32_output_limb_1_col135]: [QM31; 1] = (*triple_xor_32_output_limb_1_col135
            .try_into()
            .unwrap())
            .unbox();
        let [low_7_ms_bits_col136]: [QM31; 1] = (*low_7_ms_bits_col136.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col137]: [QM31; 1] = (*high_14_ms_bits_col137.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col138]: [QM31; 1] = (*high_5_ms_bits_col138.try_into().unwrap())
            .unbox();
        let [new_state_0_id_col139]: [QM31; 1] = (*new_state_0_id_col139.try_into().unwrap())
            .unbox();
        let [low_7_ms_bits_col140]: [QM31; 1] = (*low_7_ms_bits_col140.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col141]: [QM31; 1] = (*high_14_ms_bits_col141.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col142]: [QM31; 1] = (*high_5_ms_bits_col142.try_into().unwrap())
            .unbox();
        let [new_state_1_id_col143]: [QM31; 1] = (*new_state_1_id_col143.try_into().unwrap())
            .unbox();
        let [low_7_ms_bits_col144]: [QM31; 1] = (*low_7_ms_bits_col144.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col145]: [QM31; 1] = (*high_14_ms_bits_col145.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col146]: [QM31; 1] = (*high_5_ms_bits_col146.try_into().unwrap())
            .unbox();
        let [new_state_2_id_col147]: [QM31; 1] = (*new_state_2_id_col147.try_into().unwrap())
            .unbox();
        let [low_7_ms_bits_col148]: [QM31; 1] = (*low_7_ms_bits_col148.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col149]: [QM31; 1] = (*high_14_ms_bits_col149.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col150]: [QM31; 1] = (*high_5_ms_bits_col150.try_into().unwrap())
            .unbox();
        let [new_state_3_id_col151]: [QM31; 1] = (*new_state_3_id_col151.try_into().unwrap())
            .unbox();
        let [low_7_ms_bits_col152]: [QM31; 1] = (*low_7_ms_bits_col152.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col153]: [QM31; 1] = (*high_14_ms_bits_col153.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col154]: [QM31; 1] = (*high_5_ms_bits_col154.try_into().unwrap())
            .unbox();
        let [new_state_4_id_col155]: [QM31; 1] = (*new_state_4_id_col155.try_into().unwrap())
            .unbox();
        let [low_7_ms_bits_col156]: [QM31; 1] = (*low_7_ms_bits_col156.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col157]: [QM31; 1] = (*high_14_ms_bits_col157.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col158]: [QM31; 1] = (*high_5_ms_bits_col158.try_into().unwrap())
            .unbox();
        let [new_state_5_id_col159]: [QM31; 1] = (*new_state_5_id_col159.try_into().unwrap())
            .unbox();
        let [low_7_ms_bits_col160]: [QM31; 1] = (*low_7_ms_bits_col160.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col161]: [QM31; 1] = (*high_14_ms_bits_col161.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col162]: [QM31; 1] = (*high_5_ms_bits_col162.try_into().unwrap())
            .unbox();
        let [new_state_6_id_col163]: [QM31; 1] = (*new_state_6_id_col163.try_into().unwrap())
            .unbox();
        let [low_7_ms_bits_col164]: [QM31; 1] = (*low_7_ms_bits_col164.try_into().unwrap()).unbox();
        let [high_14_ms_bits_col165]: [QM31; 1] = (*high_14_ms_bits_col165.try_into().unwrap())
            .unbox();
        let [high_5_ms_bits_col166]: [QM31; 1] = (*high_5_ms_bits_col166.try_into().unwrap())
            .unbox();
        let [new_state_7_id_col167]: [QM31; 1] = (*new_state_7_id_col167.try_into().unwrap())
            .unbox();
        let [enabler]: [QM31; 1] = (*enabler.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        let constraint_quotient = (enabler * enabler - enabler) * domain_vanishing_eval_inv;
        sum = sum * random_coeff + constraint_quotient;

        let output: [QM31; 4] = decode_blake_opcode_evaluate(
            [input_pc_col0, input_ap_col1, input_fp_col2],
            offset0_col3,
            offset1_col4,
            offset2_col5,
            dst_base_fp_col6,
            op0_base_fp_col7,
            op1_base_fp_col8,
            op1_base_ap_col9,
            ap_update_add_1_col10,
            opcode_extension_col11,
            mem0_base_col12,
            op0_id_col13,
            op0_limb_0_col14,
            op0_limb_1_col15,
            op0_limb_2_col16,
            mem1_base_col17,
            op1_id_col18,
            op1_limb_0_col19,
            op1_limb_1_col20,
            op1_limb_2_col21,
            ap_id_col22,
            ap_limb_0_col23,
            ap_limb_1_col24,
            ap_limb_2_col25,
            mem_dst_base_col26,
            low_16_bits_col27,
            high_16_bits_col28,
            low_7_ms_bits_col29,
            high_14_ms_bits_col30,
            high_5_ms_bits_col31,
            dst_id_col32,
            self.verify_instruction_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            self.range_check_7_2_5_lookup_elements,
            ref verify_instruction_sum_0,
            ref memory_address_to_id_sum_1,
            ref memory_id_to_big_sum_2,
            ref memory_address_to_id_sum_3,
            ref memory_id_to_big_sum_4,
            ref memory_address_to_id_sum_5,
            ref memory_id_to_big_sum_6,
            ref range_check_7_2_5_sum_7,
            ref memory_address_to_id_sum_8,
            ref memory_id_to_big_sum_9,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let [
            decode_blake_opcode_output_tmp_53f39_29_limb_0,
            decode_blake_opcode_output_tmp_53f39_29_limb_1,
            decode_blake_opcode_output_tmp_53f39_29_limb_2,
            decode_blake_opcode_output_tmp_53f39_29_limb_6,
        ] =
            output;

        let output: [QM31; 4] = create_blake_round_input_evaluate(
            [
                decode_blake_opcode_output_tmp_53f39_29_limb_0, low_16_bits_col27,
                high_16_bits_col28, decode_blake_opcode_output_tmp_53f39_29_limb_6,
            ],
            low_16_bits_col33,
            high_16_bits_col34,
            low_7_ms_bits_col35,
            high_14_ms_bits_col36,
            high_5_ms_bits_col37,
            state_0_id_col38,
            low_16_bits_col39,
            high_16_bits_col40,
            low_7_ms_bits_col41,
            high_14_ms_bits_col42,
            high_5_ms_bits_col43,
            state_1_id_col44,
            low_16_bits_col45,
            high_16_bits_col46,
            low_7_ms_bits_col47,
            high_14_ms_bits_col48,
            high_5_ms_bits_col49,
            state_2_id_col50,
            low_16_bits_col51,
            high_16_bits_col52,
            low_7_ms_bits_col53,
            high_14_ms_bits_col54,
            high_5_ms_bits_col55,
            state_3_id_col56,
            low_16_bits_col57,
            high_16_bits_col58,
            low_7_ms_bits_col59,
            high_14_ms_bits_col60,
            high_5_ms_bits_col61,
            state_4_id_col62,
            low_16_bits_col63,
            high_16_bits_col64,
            low_7_ms_bits_col65,
            high_14_ms_bits_col66,
            high_5_ms_bits_col67,
            state_5_id_col68,
            low_16_bits_col69,
            high_16_bits_col70,
            low_7_ms_bits_col71,
            high_14_ms_bits_col72,
            high_5_ms_bits_col73,
            state_6_id_col74,
            low_16_bits_col75,
            high_16_bits_col76,
            low_7_ms_bits_col77,
            high_14_ms_bits_col78,
            high_5_ms_bits_col79,
            state_7_id_col80,
            ms_8_bits_col81,
            ms_8_bits_col82,
            xor_col83,
            xor_col84,
            xor_col85,
            xor_col86,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            self.verify_bitwise_xor_8_lookup_elements,
            ref range_check_7_2_5_sum_10,
            ref memory_address_to_id_sum_11,
            ref memory_id_to_big_sum_12,
            ref range_check_7_2_5_sum_13,
            ref memory_address_to_id_sum_14,
            ref memory_id_to_big_sum_15,
            ref range_check_7_2_5_sum_16,
            ref memory_address_to_id_sum_17,
            ref memory_id_to_big_sum_18,
            ref range_check_7_2_5_sum_19,
            ref memory_address_to_id_sum_20,
            ref memory_id_to_big_sum_21,
            ref range_check_7_2_5_sum_22,
            ref memory_address_to_id_sum_23,
            ref memory_id_to_big_sum_24,
            ref range_check_7_2_5_sum_25,
            ref memory_address_to_id_sum_26,
            ref memory_id_to_big_sum_27,
            ref range_check_7_2_5_sum_28,
            ref memory_address_to_id_sum_29,
            ref memory_id_to_big_sum_30,
            ref range_check_7_2_5_sum_31,
            ref memory_address_to_id_sum_32,
            ref memory_id_to_big_sum_33,
            ref verify_bitwise_xor_8_sum_34,
            ref verify_bitwise_xor_8_sum_35,
            ref verify_bitwise_xor_8_sum_36,
            ref verify_bitwise_xor_8_sum_37,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let [
            create_blake_round_input_output_tmp_53f39_114_limb_24,
            create_blake_round_input_output_tmp_53f39_114_limb_25,
            create_blake_round_input_output_tmp_53f39_114_limb_28,
            create_blake_round_input_output_tmp_53f39_114_limb_29,
        ] =
            output;

        blake_round_sum_38 = self
            .blake_round_lookup_elements
            .combine_qm31(
                [
                    seq, qm31_const::<0, 0, 0, 0>(), low_16_bits_col33, high_16_bits_col34,
                    low_16_bits_col39, high_16_bits_col40, low_16_bits_col45, high_16_bits_col46,
                    low_16_bits_col51, high_16_bits_col52, low_16_bits_col57, high_16_bits_col58,
                    low_16_bits_col63, high_16_bits_col64, low_16_bits_col69, high_16_bits_col70,
                    low_16_bits_col75, high_16_bits_col76, qm31_const::<58983, 0, 0, 0>(),
                    qm31_const::<27145, 0, 0, 0>(), qm31_const::<44677, 0, 0, 0>(),
                    qm31_const::<47975, 0, 0, 0>(), qm31_const::<62322, 0, 0, 0>(),
                    qm31_const::<15470, 0, 0, 0>(), qm31_const::<62778, 0, 0, 0>(),
                    qm31_const::<42319, 0, 0, 0>(),
                    create_blake_round_input_output_tmp_53f39_114_limb_24,
                    create_blake_round_input_output_tmp_53f39_114_limb_25,
                    qm31_const::<26764, 0, 0, 0>(), qm31_const::<39685, 0, 0, 0>(),
                    create_blake_round_input_output_tmp_53f39_114_limb_28,
                    create_blake_round_input_output_tmp_53f39_114_limb_29,
                    qm31_const::<52505, 0, 0, 0>(), qm31_const::<23520, 0, 0, 0>(),
                    decode_blake_opcode_output_tmp_53f39_29_limb_1,
                ],
            );

        blake_round_sum_39 = self
            .blake_round_lookup_elements
            .combine_qm31(
                [
                    seq, qm31_const::<10, 0, 0, 0>(), blake_round_output_limb_0_col87,
                    blake_round_output_limb_1_col88, blake_round_output_limb_2_col89,
                    blake_round_output_limb_3_col90, blake_round_output_limb_4_col91,
                    blake_round_output_limb_5_col92, blake_round_output_limb_6_col93,
                    blake_round_output_limb_7_col94, blake_round_output_limb_8_col95,
                    blake_round_output_limb_9_col96, blake_round_output_limb_10_col97,
                    blake_round_output_limb_11_col98, blake_round_output_limb_12_col99,
                    blake_round_output_limb_13_col100, blake_round_output_limb_14_col101,
                    blake_round_output_limb_15_col102, blake_round_output_limb_16_col103,
                    blake_round_output_limb_17_col104, blake_round_output_limb_18_col105,
                    blake_round_output_limb_19_col106, blake_round_output_limb_20_col107,
                    blake_round_output_limb_21_col108, blake_round_output_limb_22_col109,
                    blake_round_output_limb_23_col110, blake_round_output_limb_24_col111,
                    blake_round_output_limb_25_col112, blake_round_output_limb_26_col113,
                    blake_round_output_limb_27_col114, blake_round_output_limb_28_col115,
                    blake_round_output_limb_29_col116, blake_round_output_limb_30_col117,
                    blake_round_output_limb_31_col118, blake_round_output_limb_32_col119,
                ],
            );

        create_blake_output_evaluate(
            [
                low_16_bits_col33, high_16_bits_col34, low_16_bits_col39, high_16_bits_col40,
                low_16_bits_col45, high_16_bits_col46, low_16_bits_col51, high_16_bits_col52,
                low_16_bits_col57, high_16_bits_col58, low_16_bits_col63, high_16_bits_col64,
                low_16_bits_col69, high_16_bits_col70, low_16_bits_col75, high_16_bits_col76,
                blake_round_output_limb_0_col87, blake_round_output_limb_1_col88,
                blake_round_output_limb_2_col89, blake_round_output_limb_3_col90,
                blake_round_output_limb_4_col91, blake_round_output_limb_5_col92,
                blake_round_output_limb_6_col93, blake_round_output_limb_7_col94,
                blake_round_output_limb_8_col95, blake_round_output_limb_9_col96,
                blake_round_output_limb_10_col97, blake_round_output_limb_11_col98,
                blake_round_output_limb_12_col99, blake_round_output_limb_13_col100,
                blake_round_output_limb_14_col101, blake_round_output_limb_15_col102,
                blake_round_output_limb_16_col103, blake_round_output_limb_17_col104,
                blake_round_output_limb_18_col105, blake_round_output_limb_19_col106,
                blake_round_output_limb_20_col107, blake_round_output_limb_21_col108,
                blake_round_output_limb_22_col109, blake_round_output_limb_23_col110,
                blake_round_output_limb_24_col111, blake_round_output_limb_25_col112,
                blake_round_output_limb_26_col113, blake_round_output_limb_27_col114,
                blake_round_output_limb_28_col115, blake_round_output_limb_29_col116,
                blake_round_output_limb_30_col117, blake_round_output_limb_31_col118,
            ],
            triple_xor_32_output_limb_0_col120,
            triple_xor_32_output_limb_1_col121,
            triple_xor_32_output_limb_0_col122,
            triple_xor_32_output_limb_1_col123,
            triple_xor_32_output_limb_0_col124,
            triple_xor_32_output_limb_1_col125,
            triple_xor_32_output_limb_0_col126,
            triple_xor_32_output_limb_1_col127,
            triple_xor_32_output_limb_0_col128,
            triple_xor_32_output_limb_1_col129,
            triple_xor_32_output_limb_0_col130,
            triple_xor_32_output_limb_1_col131,
            triple_xor_32_output_limb_0_col132,
            triple_xor_32_output_limb_1_col133,
            triple_xor_32_output_limb_0_col134,
            triple_xor_32_output_limb_1_col135,
            self.triple_xor_32_lookup_elements,
            ref triple_xor_32_sum_40,
            ref triple_xor_32_sum_41,
            ref triple_xor_32_sum_42,
            ref triple_xor_32_sum_43,
            ref triple_xor_32_sum_44,
            ref triple_xor_32_sum_45,
            ref triple_xor_32_sum_46,
            ref triple_xor_32_sum_47,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        verify_blake_word_evaluate(
            [
                decode_blake_opcode_output_tmp_53f39_29_limb_2, triple_xor_32_output_limb_0_col120,
                triple_xor_32_output_limb_1_col121,
            ],
            low_7_ms_bits_col136,
            high_14_ms_bits_col137,
            high_5_ms_bits_col138,
            new_state_0_id_col139,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_48,
            ref memory_address_to_id_sum_49,
            ref memory_id_to_big_sum_50,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        verify_blake_word_evaluate(
            [
                (decode_blake_opcode_output_tmp_53f39_29_limb_2 + qm31_const::<1, 0, 0, 0>()),
                triple_xor_32_output_limb_0_col122, triple_xor_32_output_limb_1_col123,
            ],
            low_7_ms_bits_col140,
            high_14_ms_bits_col141,
            high_5_ms_bits_col142,
            new_state_1_id_col143,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_51,
            ref memory_address_to_id_sum_52,
            ref memory_id_to_big_sum_53,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        verify_blake_word_evaluate(
            [
                (decode_blake_opcode_output_tmp_53f39_29_limb_2 + qm31_const::<2, 0, 0, 0>()),
                triple_xor_32_output_limb_0_col124, triple_xor_32_output_limb_1_col125,
            ],
            low_7_ms_bits_col144,
            high_14_ms_bits_col145,
            high_5_ms_bits_col146,
            new_state_2_id_col147,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_54,
            ref memory_address_to_id_sum_55,
            ref memory_id_to_big_sum_56,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        verify_blake_word_evaluate(
            [
                (decode_blake_opcode_output_tmp_53f39_29_limb_2 + qm31_const::<3, 0, 0, 0>()),
                triple_xor_32_output_limb_0_col126, triple_xor_32_output_limb_1_col127,
            ],
            low_7_ms_bits_col148,
            high_14_ms_bits_col149,
            high_5_ms_bits_col150,
            new_state_3_id_col151,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_57,
            ref memory_address_to_id_sum_58,
            ref memory_id_to_big_sum_59,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        verify_blake_word_evaluate(
            [
                (decode_blake_opcode_output_tmp_53f39_29_limb_2 + qm31_const::<4, 0, 0, 0>()),
                triple_xor_32_output_limb_0_col128, triple_xor_32_output_limb_1_col129,
            ],
            low_7_ms_bits_col152,
            high_14_ms_bits_col153,
            high_5_ms_bits_col154,
            new_state_4_id_col155,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_60,
            ref memory_address_to_id_sum_61,
            ref memory_id_to_big_sum_62,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        verify_blake_word_evaluate(
            [
                (decode_blake_opcode_output_tmp_53f39_29_limb_2 + qm31_const::<5, 0, 0, 0>()),
                triple_xor_32_output_limb_0_col130, triple_xor_32_output_limb_1_col131,
            ],
            low_7_ms_bits_col156,
            high_14_ms_bits_col157,
            high_5_ms_bits_col158,
            new_state_5_id_col159,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_63,
            ref memory_address_to_id_sum_64,
            ref memory_id_to_big_sum_65,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        verify_blake_word_evaluate(
            [
                (decode_blake_opcode_output_tmp_53f39_29_limb_2 + qm31_const::<6, 0, 0, 0>()),
                triple_xor_32_output_limb_0_col132, triple_xor_32_output_limb_1_col133,
            ],
            low_7_ms_bits_col160,
            high_14_ms_bits_col161,
            high_5_ms_bits_col162,
            new_state_6_id_col163,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_66,
            ref memory_address_to_id_sum_67,
            ref memory_id_to_big_sum_68,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        verify_blake_word_evaluate(
            [
                (decode_blake_opcode_output_tmp_53f39_29_limb_2 + qm31_const::<7, 0, 0, 0>()),
                triple_xor_32_output_limb_0_col134, triple_xor_32_output_limb_1_col135,
            ],
            low_7_ms_bits_col164,
            high_14_ms_bits_col165,
            high_5_ms_bits_col166,
            new_state_7_id_col167,
            self.range_check_7_2_5_lookup_elements,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref range_check_7_2_5_sum_69,
            ref memory_address_to_id_sum_70,
            ref memory_id_to_big_sum_71,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        opcodes_sum_72 = self
            .opcodes_lookup_elements
            .combine_qm31([input_pc_col0, input_ap_col1, input_fp_col2]);

        opcodes_sum_73 = self
            .opcodes_lookup_elements
            .combine_qm31(
                [
                    (input_pc_col0 + qm31_const::<1, 0, 0, 0>()),
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
            verify_bitwise_xor_8_sum_34,
            verify_bitwise_xor_8_sum_35,
            verify_bitwise_xor_8_sum_36,
            verify_bitwise_xor_8_sum_37,
            blake_round_sum_38,
            blake_round_sum_39,
            triple_xor_32_sum_40,
            triple_xor_32_sum_41,
            triple_xor_32_sum_42,
            triple_xor_32_sum_43,
            triple_xor_32_sum_44,
            triple_xor_32_sum_45,
            triple_xor_32_sum_46,
            triple_xor_32_sum_47,
            range_check_7_2_5_sum_48,
            memory_address_to_id_sum_49,
            memory_id_to_big_sum_50,
            range_check_7_2_5_sum_51,
            memory_address_to_id_sum_52,
            memory_id_to_big_sum_53,
            range_check_7_2_5_sum_54,
            memory_address_to_id_sum_55,
            memory_id_to_big_sum_56,
            range_check_7_2_5_sum_57,
            memory_address_to_id_sum_58,
            memory_id_to_big_sum_59,
            range_check_7_2_5_sum_60,
            memory_address_to_id_sum_61,
            memory_id_to_big_sum_62,
            range_check_7_2_5_sum_63,
            memory_address_to_id_sum_64,
            memory_id_to_big_sum_65,
            range_check_7_2_5_sum_66,
            memory_address_to_id_sum_67,
            memory_id_to_big_sum_68,
            range_check_7_2_5_sum_69,
            memory_address_to_id_sum_70,
            memory_id_to_big_sum_71,
            opcodes_sum_72,
            opcodes_sum_73,
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
    verify_bitwise_xor_8_sum_34: QM31,
    verify_bitwise_xor_8_sum_35: QM31,
    verify_bitwise_xor_8_sum_36: QM31,
    verify_bitwise_xor_8_sum_37: QM31,
    blake_round_sum_38: QM31,
    blake_round_sum_39: QM31,
    triple_xor_32_sum_40: QM31,
    triple_xor_32_sum_41: QM31,
    triple_xor_32_sum_42: QM31,
    triple_xor_32_sum_43: QM31,
    triple_xor_32_sum_44: QM31,
    triple_xor_32_sum_45: QM31,
    triple_xor_32_sum_46: QM31,
    triple_xor_32_sum_47: QM31,
    range_check_7_2_5_sum_48: QM31,
    memory_address_to_id_sum_49: QM31,
    memory_id_to_big_sum_50: QM31,
    range_check_7_2_5_sum_51: QM31,
    memory_address_to_id_sum_52: QM31,
    memory_id_to_big_sum_53: QM31,
    range_check_7_2_5_sum_54: QM31,
    memory_address_to_id_sum_55: QM31,
    memory_id_to_big_sum_56: QM31,
    range_check_7_2_5_sum_57: QM31,
    memory_address_to_id_sum_58: QM31,
    memory_id_to_big_sum_59: QM31,
    range_check_7_2_5_sum_60: QM31,
    memory_address_to_id_sum_61: QM31,
    memory_id_to_big_sum_62: QM31,
    range_check_7_2_5_sum_63: QM31,
    memory_address_to_id_sum_64: QM31,
    memory_id_to_big_sum_65: QM31,
    range_check_7_2_5_sum_66: QM31,
    memory_address_to_id_sum_67: QM31,
    memory_id_to_big_sum_68: QM31,
    range_check_7_2_5_sum_69: QM31,
    memory_address_to_id_sum_70: QM31,
    memory_id_to_big_sum_71: QM31,
    opcodes_sum_72: QM31,
    opcodes_sum_73: QM31,
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
    ]: [Span<QM31>; 148] =
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
    let [trace_2_col144_neg1, trace_2_col144]: [QM31; 2] = (*trace_2_col144.try_into().unwrap())
        .unbox();
    let [trace_2_col145_neg1, trace_2_col145]: [QM31; 2] = (*trace_2_col145.try_into().unwrap())
        .unbox();
    let [trace_2_col146_neg1, trace_2_col146]: [QM31; 2] = (*trace_2_col146.try_into().unwrap())
        .unbox();
    let [trace_2_col147_neg1, trace_2_col147]: [QM31; 2] = (*trace_2_col147.try_into().unwrap())
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
        * verify_bitwise_xor_8_sum_34
        * verify_bitwise_xor_8_sum_35)
        - verify_bitwise_xor_8_sum_34
        - verify_bitwise_xor_8_sum_35)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col72, trace_2_col73, trace_2_col74, trace_2_col75],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col68, trace_2_col69, trace_2_col70, trace_2_col71],
        ))
        * verify_bitwise_xor_8_sum_36
        * verify_bitwise_xor_8_sum_37)
        - verify_bitwise_xor_8_sum_36
        - verify_bitwise_xor_8_sum_37)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col76, trace_2_col77, trace_2_col78, trace_2_col79],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col72, trace_2_col73, trace_2_col74, trace_2_col75],
        ))
        * blake_round_sum_38
        * blake_round_sum_39)
        - blake_round_sum_38
        + blake_round_sum_39)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col80, trace_2_col81, trace_2_col82, trace_2_col83],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col76, trace_2_col77, trace_2_col78, trace_2_col79],
        ))
        * triple_xor_32_sum_40
        * triple_xor_32_sum_41)
        - triple_xor_32_sum_40
        - triple_xor_32_sum_41)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col84, trace_2_col85, trace_2_col86, trace_2_col87],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col80, trace_2_col81, trace_2_col82, trace_2_col83],
        ))
        * triple_xor_32_sum_42
        * triple_xor_32_sum_43)
        - triple_xor_32_sum_42
        - triple_xor_32_sum_43)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col88, trace_2_col89, trace_2_col90, trace_2_col91],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col84, trace_2_col85, trace_2_col86, trace_2_col87],
        ))
        * triple_xor_32_sum_44
        * triple_xor_32_sum_45)
        - triple_xor_32_sum_44
        - triple_xor_32_sum_45)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col92, trace_2_col93, trace_2_col94, trace_2_col95],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col88, trace_2_col89, trace_2_col90, trace_2_col91],
        ))
        * triple_xor_32_sum_46
        * triple_xor_32_sum_47)
        - triple_xor_32_sum_46
        - triple_xor_32_sum_47)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col96, trace_2_col97, trace_2_col98, trace_2_col99],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col92, trace_2_col93, trace_2_col94, trace_2_col95],
        ))
        * range_check_7_2_5_sum_48
        * memory_address_to_id_sum_49)
        - range_check_7_2_5_sum_48
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
        * range_check_7_2_5_sum_51)
        - memory_id_to_big_sum_50
        - range_check_7_2_5_sum_51)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col104, trace_2_col105, trace_2_col106, trace_2_col107],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col100, trace_2_col101, trace_2_col102, trace_2_col103],
        ))
        * memory_address_to_id_sum_52
        * memory_id_to_big_sum_53)
        - memory_address_to_id_sum_52
        - memory_id_to_big_sum_53)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col108, trace_2_col109, trace_2_col110, trace_2_col111],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col104, trace_2_col105, trace_2_col106, trace_2_col107],
        ))
        * range_check_7_2_5_sum_54
        * memory_address_to_id_sum_55)
        - range_check_7_2_5_sum_54
        - memory_address_to_id_sum_55)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col112, trace_2_col113, trace_2_col114, trace_2_col115],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col108, trace_2_col109, trace_2_col110, trace_2_col111],
        ))
        * memory_id_to_big_sum_56
        * range_check_7_2_5_sum_57)
        - memory_id_to_big_sum_56
        - range_check_7_2_5_sum_57)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col116, trace_2_col117, trace_2_col118, trace_2_col119],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col112, trace_2_col113, trace_2_col114, trace_2_col115],
        ))
        * memory_address_to_id_sum_58
        * memory_id_to_big_sum_59)
        - memory_address_to_id_sum_58
        - memory_id_to_big_sum_59)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col120, trace_2_col121, trace_2_col122, trace_2_col123],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col116, trace_2_col117, trace_2_col118, trace_2_col119],
        ))
        * range_check_7_2_5_sum_60
        * memory_address_to_id_sum_61)
        - range_check_7_2_5_sum_60
        - memory_address_to_id_sum_61)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col124, trace_2_col125, trace_2_col126, trace_2_col127],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col120, trace_2_col121, trace_2_col122, trace_2_col123],
        ))
        * memory_id_to_big_sum_62
        * range_check_7_2_5_sum_63)
        - memory_id_to_big_sum_62
        - range_check_7_2_5_sum_63)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col128, trace_2_col129, trace_2_col130, trace_2_col131],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col124, trace_2_col125, trace_2_col126, trace_2_col127],
        ))
        * memory_address_to_id_sum_64
        * memory_id_to_big_sum_65)
        - memory_address_to_id_sum_64
        - memory_id_to_big_sum_65)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col132, trace_2_col133, trace_2_col134, trace_2_col135],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col128, trace_2_col129, trace_2_col130, trace_2_col131],
        ))
        * range_check_7_2_5_sum_66
        * memory_address_to_id_sum_67)
        - range_check_7_2_5_sum_66
        - memory_address_to_id_sum_67)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col136, trace_2_col137, trace_2_col138, trace_2_col139],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col132, trace_2_col133, trace_2_col134, trace_2_col135],
        ))
        * memory_id_to_big_sum_68
        * range_check_7_2_5_sum_69)
        - memory_id_to_big_sum_68
        - range_check_7_2_5_sum_69)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col140, trace_2_col141, trace_2_col142, trace_2_col143],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col136, trace_2_col137, trace_2_col138, trace_2_col139],
        ))
        * memory_address_to_id_sum_70
        * memory_id_to_big_sum_71)
        - memory_address_to_id_sum_70
        - memory_id_to_big_sum_71)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col144, trace_2_col145, trace_2_col146, trace_2_col147],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col140, trace_2_col141, trace_2_col142, trace_2_col143],
        )
        - QM31Impl::from_partial_evals(
            [trace_2_col144_neg1, trace_2_col145_neg1, trace_2_col146_neg1, trace_2_col147_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * opcodes_sum_72
        * opcodes_sum_73)
        + (opcodes_sum_72 * enabler)
        - (opcodes_sum_73 * enabler))
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
}
