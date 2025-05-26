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
use crate::components::subroutines::felt_252_unpack_from_27::felt_252_unpack_from_27_evaluate;
use crate::components::subroutines::mem_verify::mem_verify_evaluate;
use crate::components::subroutines::poseidon_hades_permutation::poseidon_hades_permutation_evaluate;
use crate::components::subroutines::read_positive_num_bits_252::read_positive_num_bits_252_evaluate;
use crate::utils::U32Impl;

pub const N_TRACE_COLUMNS: usize = 341;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 9] = [
    ('MemoryAddressToId', 6), ('MemoryIdToBig', 6), ('PoseidonFullRoundChain', 2),
    ('RangeCheckFelt252Width27', 2), ('Cube252', 2), ('RangeCheck_3_3_3_3_3', 2),
    ('RangeCheck_4_4_4_4', 6), ('RangeCheck_4_4', 3), ('Poseidon3PartialRoundsChain', 1),
];

#[derive(Drop, Serde, Copy)]
pub struct Claim {
    pub log_size: u32,
    pub poseidon_builtin_segment_start: u32,
}

#[generate_trait]
pub impl ClaimImpl of ClaimTrait {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = *(self.log_size);
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = ArrayImpl::new_repeated(N_TRACE_COLUMNS, log_size).span();
        let interaction_log_sizes = ArrayImpl::new_repeated(68, log_size).span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        channel.mix_u64((*(self.log_size)).into());
        channel.mix_u64((*self.poseidon_builtin_segment_start).into());
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
    pub poseidon_full_round_chain_lookup_elements: crate::PoseidonFullRoundChainElements,
    pub range_check_felt_252_width_27_lookup_elements: crate::RangeCheckFelt252Width27Elements,
    pub cube_252_lookup_elements: crate::Cube252Elements,
    pub range_check_3_3_3_3_3_lookup_elements: crate::RangeCheck_3_3_3_3_3Elements,
    pub range_check_4_4_4_4_lookup_elements: crate::RangeCheck_4_4_4_4Elements,
    pub range_check_4_4_lookup_elements: crate::RangeCheck_4_4Elements,
    pub poseidon_3_partial_rounds_chain_lookup_elements: crate::Poseidon3PartialRoundsChainElements,
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
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
        trace_mask_points.append(array![point]);
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
        let poseidon_builtin_segment_start: QM31 = (TryInto::<
            u32, M31,
        >::try_into((*(self.claim.poseidon_builtin_segment_start)))
            .unwrap())
            .into();
        let mut memory_address_to_id_sum_0: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_1: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_2: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_3: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_4: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_5: QM31 = Zero::zero();
        let mut poseidon_full_round_chain_sum_6: QM31 = Zero::zero();
        let mut poseidon_full_round_chain_sum_7: QM31 = Zero::zero();
        let mut range_check_felt_252_width_27_sum_8: QM31 = Zero::zero();
        let mut range_check_felt_252_width_27_sum_9: QM31 = Zero::zero();
        let mut cube_252_sum_10: QM31 = Zero::zero();
        let mut range_check_3_3_3_3_3_sum_11: QM31 = Zero::zero();
        let mut range_check_3_3_3_3_3_sum_12: QM31 = Zero::zero();
        let mut cube_252_sum_13: QM31 = Zero::zero();
        let mut range_check_4_4_4_4_sum_14: QM31 = Zero::zero();
        let mut range_check_4_4_4_4_sum_15: QM31 = Zero::zero();
        let mut range_check_4_4_sum_16: QM31 = Zero::zero();
        let mut poseidon_3_partial_rounds_chain_sum_17: QM31 = Zero::zero();
        let mut poseidon_3_partial_rounds_chain_sum_18: QM31 = Zero::zero();
        let mut range_check_4_4_4_4_sum_19: QM31 = Zero::zero();
        let mut range_check_4_4_4_4_sum_20: QM31 = Zero::zero();
        let mut range_check_4_4_sum_21: QM31 = Zero::zero();
        let mut range_check_4_4_4_4_sum_22: QM31 = Zero::zero();
        let mut range_check_4_4_4_4_sum_23: QM31 = Zero::zero();
        let mut range_check_4_4_sum_24: QM31 = Zero::zero();
        let mut poseidon_full_round_chain_sum_25: QM31 = Zero::zero();
        let mut poseidon_full_round_chain_sum_26: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_27: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_28: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_29: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_30: QM31 = Zero::zero();
        let mut memory_address_to_id_sum_31: QM31 = Zero::zero();
        let mut memory_id_to_big_sum_32: QM31 = Zero::zero();
        let seq = preprocessed_mask_values.get(PreprocessedColumn::Seq(*(self.claim.log_size)));

        let [
            input_state_0_id_col0,
            input_state_0_limb_0_col1,
            input_state_0_limb_1_col2,
            input_state_0_limb_2_col3,
            input_state_0_limb_3_col4,
            input_state_0_limb_4_col5,
            input_state_0_limb_5_col6,
            input_state_0_limb_6_col7,
            input_state_0_limb_7_col8,
            input_state_0_limb_8_col9,
            input_state_0_limb_9_col10,
            input_state_0_limb_10_col11,
            input_state_0_limb_11_col12,
            input_state_0_limb_12_col13,
            input_state_0_limb_13_col14,
            input_state_0_limb_14_col15,
            input_state_0_limb_15_col16,
            input_state_0_limb_16_col17,
            input_state_0_limb_17_col18,
            input_state_0_limb_18_col19,
            input_state_0_limb_19_col20,
            input_state_0_limb_20_col21,
            input_state_0_limb_21_col22,
            input_state_0_limb_22_col23,
            input_state_0_limb_23_col24,
            input_state_0_limb_24_col25,
            input_state_0_limb_25_col26,
            input_state_0_limb_26_col27,
            input_state_0_limb_27_col28,
            input_state_1_id_col29,
            input_state_1_limb_0_col30,
            input_state_1_limb_1_col31,
            input_state_1_limb_2_col32,
            input_state_1_limb_3_col33,
            input_state_1_limb_4_col34,
            input_state_1_limb_5_col35,
            input_state_1_limb_6_col36,
            input_state_1_limb_7_col37,
            input_state_1_limb_8_col38,
            input_state_1_limb_9_col39,
            input_state_1_limb_10_col40,
            input_state_1_limb_11_col41,
            input_state_1_limb_12_col42,
            input_state_1_limb_13_col43,
            input_state_1_limb_14_col44,
            input_state_1_limb_15_col45,
            input_state_1_limb_16_col46,
            input_state_1_limb_17_col47,
            input_state_1_limb_18_col48,
            input_state_1_limb_19_col49,
            input_state_1_limb_20_col50,
            input_state_1_limb_21_col51,
            input_state_1_limb_22_col52,
            input_state_1_limb_23_col53,
            input_state_1_limb_24_col54,
            input_state_1_limb_25_col55,
            input_state_1_limb_26_col56,
            input_state_1_limb_27_col57,
            input_state_2_id_col58,
            input_state_2_limb_0_col59,
            input_state_2_limb_1_col60,
            input_state_2_limb_2_col61,
            input_state_2_limb_3_col62,
            input_state_2_limb_4_col63,
            input_state_2_limb_5_col64,
            input_state_2_limb_6_col65,
            input_state_2_limb_7_col66,
            input_state_2_limb_8_col67,
            input_state_2_limb_9_col68,
            input_state_2_limb_10_col69,
            input_state_2_limb_11_col70,
            input_state_2_limb_12_col71,
            input_state_2_limb_13_col72,
            input_state_2_limb_14_col73,
            input_state_2_limb_15_col74,
            input_state_2_limb_16_col75,
            input_state_2_limb_17_col76,
            input_state_2_limb_18_col77,
            input_state_2_limb_19_col78,
            input_state_2_limb_20_col79,
            input_state_2_limb_21_col80,
            input_state_2_limb_22_col81,
            input_state_2_limb_23_col82,
            input_state_2_limb_24_col83,
            input_state_2_limb_25_col84,
            input_state_2_limb_26_col85,
            input_state_2_limb_27_col86,
            combination_limb_0_col87,
            combination_limb_1_col88,
            combination_limb_2_col89,
            combination_limb_3_col90,
            combination_limb_4_col91,
            combination_limb_5_col92,
            combination_limb_6_col93,
            combination_limb_7_col94,
            combination_limb_8_col95,
            combination_limb_9_col96,
            p_coef_col97,
            combination_limb_0_col98,
            combination_limb_1_col99,
            combination_limb_2_col100,
            combination_limb_3_col101,
            combination_limb_4_col102,
            combination_limb_5_col103,
            combination_limb_6_col104,
            combination_limb_7_col105,
            combination_limb_8_col106,
            combination_limb_9_col107,
            p_coef_col108,
            combination_limb_0_col109,
            combination_limb_1_col110,
            combination_limb_2_col111,
            combination_limb_3_col112,
            combination_limb_4_col113,
            combination_limb_5_col114,
            combination_limb_6_col115,
            combination_limb_7_col116,
            combination_limb_8_col117,
            combination_limb_9_col118,
            p_coef_col119,
            poseidon_full_round_chain_output_limb_0_col120,
            poseidon_full_round_chain_output_limb_1_col121,
            poseidon_full_round_chain_output_limb_2_col122,
            poseidon_full_round_chain_output_limb_3_col123,
            poseidon_full_round_chain_output_limb_4_col124,
            poseidon_full_round_chain_output_limb_5_col125,
            poseidon_full_round_chain_output_limb_6_col126,
            poseidon_full_round_chain_output_limb_7_col127,
            poseidon_full_round_chain_output_limb_8_col128,
            poseidon_full_round_chain_output_limb_9_col129,
            poseidon_full_round_chain_output_limb_10_col130,
            poseidon_full_round_chain_output_limb_11_col131,
            poseidon_full_round_chain_output_limb_12_col132,
            poseidon_full_round_chain_output_limb_13_col133,
            poseidon_full_round_chain_output_limb_14_col134,
            poseidon_full_round_chain_output_limb_15_col135,
            poseidon_full_round_chain_output_limb_16_col136,
            poseidon_full_round_chain_output_limb_17_col137,
            poseidon_full_round_chain_output_limb_18_col138,
            poseidon_full_round_chain_output_limb_19_col139,
            poseidon_full_round_chain_output_limb_20_col140,
            poseidon_full_round_chain_output_limb_21_col141,
            poseidon_full_round_chain_output_limb_22_col142,
            poseidon_full_round_chain_output_limb_23_col143,
            poseidon_full_round_chain_output_limb_24_col144,
            poseidon_full_round_chain_output_limb_25_col145,
            poseidon_full_round_chain_output_limb_26_col146,
            poseidon_full_round_chain_output_limb_27_col147,
            poseidon_full_round_chain_output_limb_28_col148,
            poseidon_full_round_chain_output_limb_29_col149,
            cube_252_output_limb_0_col150,
            cube_252_output_limb_1_col151,
            cube_252_output_limb_2_col152,
            cube_252_output_limb_3_col153,
            cube_252_output_limb_4_col154,
            cube_252_output_limb_5_col155,
            cube_252_output_limb_6_col156,
            cube_252_output_limb_7_col157,
            cube_252_output_limb_8_col158,
            cube_252_output_limb_9_col159,
            combination_limb_0_col160,
            combination_limb_1_col161,
            combination_limb_2_col162,
            combination_limb_3_col163,
            combination_limb_4_col164,
            combination_limb_5_col165,
            combination_limb_6_col166,
            combination_limb_7_col167,
            combination_limb_8_col168,
            combination_limb_9_col169,
            p_coef_col170,
            cube_252_output_limb_0_col171,
            cube_252_output_limb_1_col172,
            cube_252_output_limb_2_col173,
            cube_252_output_limb_3_col174,
            cube_252_output_limb_4_col175,
            cube_252_output_limb_5_col176,
            cube_252_output_limb_6_col177,
            cube_252_output_limb_7_col178,
            cube_252_output_limb_8_col179,
            cube_252_output_limb_9_col180,
            combination_limb_0_col181,
            combination_limb_1_col182,
            combination_limb_2_col183,
            combination_limb_3_col184,
            combination_limb_4_col185,
            combination_limb_5_col186,
            combination_limb_6_col187,
            combination_limb_7_col188,
            combination_limb_8_col189,
            combination_limb_9_col190,
            p_coef_col191,
            poseidon_3_partial_rounds_chain_output_limb_0_col192,
            poseidon_3_partial_rounds_chain_output_limb_1_col193,
            poseidon_3_partial_rounds_chain_output_limb_2_col194,
            poseidon_3_partial_rounds_chain_output_limb_3_col195,
            poseidon_3_partial_rounds_chain_output_limb_4_col196,
            poseidon_3_partial_rounds_chain_output_limb_5_col197,
            poseidon_3_partial_rounds_chain_output_limb_6_col198,
            poseidon_3_partial_rounds_chain_output_limb_7_col199,
            poseidon_3_partial_rounds_chain_output_limb_8_col200,
            poseidon_3_partial_rounds_chain_output_limb_9_col201,
            poseidon_3_partial_rounds_chain_output_limb_10_col202,
            poseidon_3_partial_rounds_chain_output_limb_11_col203,
            poseidon_3_partial_rounds_chain_output_limb_12_col204,
            poseidon_3_partial_rounds_chain_output_limb_13_col205,
            poseidon_3_partial_rounds_chain_output_limb_14_col206,
            poseidon_3_partial_rounds_chain_output_limb_15_col207,
            poseidon_3_partial_rounds_chain_output_limb_16_col208,
            poseidon_3_partial_rounds_chain_output_limb_17_col209,
            poseidon_3_partial_rounds_chain_output_limb_18_col210,
            poseidon_3_partial_rounds_chain_output_limb_19_col211,
            poseidon_3_partial_rounds_chain_output_limb_20_col212,
            poseidon_3_partial_rounds_chain_output_limb_21_col213,
            poseidon_3_partial_rounds_chain_output_limb_22_col214,
            poseidon_3_partial_rounds_chain_output_limb_23_col215,
            poseidon_3_partial_rounds_chain_output_limb_24_col216,
            poseidon_3_partial_rounds_chain_output_limb_25_col217,
            poseidon_3_partial_rounds_chain_output_limb_26_col218,
            poseidon_3_partial_rounds_chain_output_limb_27_col219,
            poseidon_3_partial_rounds_chain_output_limb_28_col220,
            poseidon_3_partial_rounds_chain_output_limb_29_col221,
            poseidon_3_partial_rounds_chain_output_limb_30_col222,
            poseidon_3_partial_rounds_chain_output_limb_31_col223,
            poseidon_3_partial_rounds_chain_output_limb_32_col224,
            poseidon_3_partial_rounds_chain_output_limb_33_col225,
            poseidon_3_partial_rounds_chain_output_limb_34_col226,
            poseidon_3_partial_rounds_chain_output_limb_35_col227,
            poseidon_3_partial_rounds_chain_output_limb_36_col228,
            poseidon_3_partial_rounds_chain_output_limb_37_col229,
            poseidon_3_partial_rounds_chain_output_limb_38_col230,
            poseidon_3_partial_rounds_chain_output_limb_39_col231,
            combination_limb_0_col232,
            combination_limb_1_col233,
            combination_limb_2_col234,
            combination_limb_3_col235,
            combination_limb_4_col236,
            combination_limb_5_col237,
            combination_limb_6_col238,
            combination_limb_7_col239,
            combination_limb_8_col240,
            combination_limb_9_col241,
            p_coef_col242,
            combination_limb_0_col243,
            combination_limb_1_col244,
            combination_limb_2_col245,
            combination_limb_3_col246,
            combination_limb_4_col247,
            combination_limb_5_col248,
            combination_limb_6_col249,
            combination_limb_7_col250,
            combination_limb_8_col251,
            combination_limb_9_col252,
            p_coef_col253,
            poseidon_full_round_chain_output_limb_0_col254,
            poseidon_full_round_chain_output_limb_1_col255,
            poseidon_full_round_chain_output_limb_2_col256,
            poseidon_full_round_chain_output_limb_3_col257,
            poseidon_full_round_chain_output_limb_4_col258,
            poseidon_full_round_chain_output_limb_5_col259,
            poseidon_full_round_chain_output_limb_6_col260,
            poseidon_full_round_chain_output_limb_7_col261,
            poseidon_full_round_chain_output_limb_8_col262,
            poseidon_full_round_chain_output_limb_9_col263,
            poseidon_full_round_chain_output_limb_10_col264,
            poseidon_full_round_chain_output_limb_11_col265,
            poseidon_full_round_chain_output_limb_12_col266,
            poseidon_full_round_chain_output_limb_13_col267,
            poseidon_full_round_chain_output_limb_14_col268,
            poseidon_full_round_chain_output_limb_15_col269,
            poseidon_full_round_chain_output_limb_16_col270,
            poseidon_full_round_chain_output_limb_17_col271,
            poseidon_full_round_chain_output_limb_18_col272,
            poseidon_full_round_chain_output_limb_19_col273,
            poseidon_full_round_chain_output_limb_20_col274,
            poseidon_full_round_chain_output_limb_21_col275,
            poseidon_full_round_chain_output_limb_22_col276,
            poseidon_full_round_chain_output_limb_23_col277,
            poseidon_full_round_chain_output_limb_24_col278,
            poseidon_full_round_chain_output_limb_25_col279,
            poseidon_full_round_chain_output_limb_26_col280,
            poseidon_full_round_chain_output_limb_27_col281,
            poseidon_full_round_chain_output_limb_28_col282,
            poseidon_full_round_chain_output_limb_29_col283,
            unpacked_limb_0_col284,
            unpacked_limb_1_col285,
            unpacked_limb_3_col286,
            unpacked_limb_4_col287,
            unpacked_limb_6_col288,
            unpacked_limb_7_col289,
            unpacked_limb_9_col290,
            unpacked_limb_10_col291,
            unpacked_limb_12_col292,
            unpacked_limb_13_col293,
            unpacked_limb_15_col294,
            unpacked_limb_16_col295,
            unpacked_limb_18_col296,
            unpacked_limb_19_col297,
            unpacked_limb_21_col298,
            unpacked_limb_22_col299,
            unpacked_limb_24_col300,
            unpacked_limb_25_col301,
            output_state_0_id_col302,
            unpacked_limb_0_col303,
            unpacked_limb_1_col304,
            unpacked_limb_3_col305,
            unpacked_limb_4_col306,
            unpacked_limb_6_col307,
            unpacked_limb_7_col308,
            unpacked_limb_9_col309,
            unpacked_limb_10_col310,
            unpacked_limb_12_col311,
            unpacked_limb_13_col312,
            unpacked_limb_15_col313,
            unpacked_limb_16_col314,
            unpacked_limb_18_col315,
            unpacked_limb_19_col316,
            unpacked_limb_21_col317,
            unpacked_limb_22_col318,
            unpacked_limb_24_col319,
            unpacked_limb_25_col320,
            output_state_1_id_col321,
            unpacked_limb_0_col322,
            unpacked_limb_1_col323,
            unpacked_limb_3_col324,
            unpacked_limb_4_col325,
            unpacked_limb_6_col326,
            unpacked_limb_7_col327,
            unpacked_limb_9_col328,
            unpacked_limb_10_col329,
            unpacked_limb_12_col330,
            unpacked_limb_13_col331,
            unpacked_limb_15_col332,
            unpacked_limb_16_col333,
            unpacked_limb_18_col334,
            unpacked_limb_19_col335,
            unpacked_limb_21_col336,
            unpacked_limb_22_col337,
            unpacked_limb_24_col338,
            unpacked_limb_25_col339,
            output_state_2_id_col340,
        ]: [Span<QM31>; 341] =
            (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [input_state_0_id_col0]: [QM31; 1] = (*input_state_0_id_col0.try_into().unwrap())
            .unbox();
        let [input_state_0_limb_0_col1]: [QM31; 1] = (*input_state_0_limb_0_col1
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_0_limb_1_col2]: [QM31; 1] = (*input_state_0_limb_1_col2
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_0_limb_2_col3]: [QM31; 1] = (*input_state_0_limb_2_col3
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_0_limb_3_col4]: [QM31; 1] = (*input_state_0_limb_3_col4
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_0_limb_4_col5]: [QM31; 1] = (*input_state_0_limb_4_col5
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_0_limb_5_col6]: [QM31; 1] = (*input_state_0_limb_5_col6
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_0_limb_6_col7]: [QM31; 1] = (*input_state_0_limb_6_col7
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_0_limb_7_col8]: [QM31; 1] = (*input_state_0_limb_7_col8
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_0_limb_8_col9]: [QM31; 1] = (*input_state_0_limb_8_col9
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_0_limb_9_col10]: [QM31; 1] = (*input_state_0_limb_9_col10
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_0_limb_10_col11]: [QM31; 1] = (*input_state_0_limb_10_col11
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_0_limb_11_col12]: [QM31; 1] = (*input_state_0_limb_11_col12
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_0_limb_12_col13]: [QM31; 1] = (*input_state_0_limb_12_col13
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_0_limb_13_col14]: [QM31; 1] = (*input_state_0_limb_13_col14
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_0_limb_14_col15]: [QM31; 1] = (*input_state_0_limb_14_col15
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_0_limb_15_col16]: [QM31; 1] = (*input_state_0_limb_15_col16
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_0_limb_16_col17]: [QM31; 1] = (*input_state_0_limb_16_col17
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_0_limb_17_col18]: [QM31; 1] = (*input_state_0_limb_17_col18
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_0_limb_18_col19]: [QM31; 1] = (*input_state_0_limb_18_col19
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_0_limb_19_col20]: [QM31; 1] = (*input_state_0_limb_19_col20
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_0_limb_20_col21]: [QM31; 1] = (*input_state_0_limb_20_col21
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_0_limb_21_col22]: [QM31; 1] = (*input_state_0_limb_21_col22
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_0_limb_22_col23]: [QM31; 1] = (*input_state_0_limb_22_col23
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_0_limb_23_col24]: [QM31; 1] = (*input_state_0_limb_23_col24
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_0_limb_24_col25]: [QM31; 1] = (*input_state_0_limb_24_col25
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_0_limb_25_col26]: [QM31; 1] = (*input_state_0_limb_25_col26
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_0_limb_26_col27]: [QM31; 1] = (*input_state_0_limb_26_col27
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_0_limb_27_col28]: [QM31; 1] = (*input_state_0_limb_27_col28
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_1_id_col29]: [QM31; 1] = (*input_state_1_id_col29.try_into().unwrap())
            .unbox();
        let [input_state_1_limb_0_col30]: [QM31; 1] = (*input_state_1_limb_0_col30
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_1_limb_1_col31]: [QM31; 1] = (*input_state_1_limb_1_col31
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_1_limb_2_col32]: [QM31; 1] = (*input_state_1_limb_2_col32
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_1_limb_3_col33]: [QM31; 1] = (*input_state_1_limb_3_col33
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_1_limb_4_col34]: [QM31; 1] = (*input_state_1_limb_4_col34
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_1_limb_5_col35]: [QM31; 1] = (*input_state_1_limb_5_col35
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_1_limb_6_col36]: [QM31; 1] = (*input_state_1_limb_6_col36
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_1_limb_7_col37]: [QM31; 1] = (*input_state_1_limb_7_col37
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_1_limb_8_col38]: [QM31; 1] = (*input_state_1_limb_8_col38
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_1_limb_9_col39]: [QM31; 1] = (*input_state_1_limb_9_col39
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_1_limb_10_col40]: [QM31; 1] = (*input_state_1_limb_10_col40
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_1_limb_11_col41]: [QM31; 1] = (*input_state_1_limb_11_col41
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_1_limb_12_col42]: [QM31; 1] = (*input_state_1_limb_12_col42
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_1_limb_13_col43]: [QM31; 1] = (*input_state_1_limb_13_col43
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_1_limb_14_col44]: [QM31; 1] = (*input_state_1_limb_14_col44
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_1_limb_15_col45]: [QM31; 1] = (*input_state_1_limb_15_col45
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_1_limb_16_col46]: [QM31; 1] = (*input_state_1_limb_16_col46
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_1_limb_17_col47]: [QM31; 1] = (*input_state_1_limb_17_col47
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_1_limb_18_col48]: [QM31; 1] = (*input_state_1_limb_18_col48
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_1_limb_19_col49]: [QM31; 1] = (*input_state_1_limb_19_col49
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_1_limb_20_col50]: [QM31; 1] = (*input_state_1_limb_20_col50
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_1_limb_21_col51]: [QM31; 1] = (*input_state_1_limb_21_col51
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_1_limb_22_col52]: [QM31; 1] = (*input_state_1_limb_22_col52
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_1_limb_23_col53]: [QM31; 1] = (*input_state_1_limb_23_col53
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_1_limb_24_col54]: [QM31; 1] = (*input_state_1_limb_24_col54
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_1_limb_25_col55]: [QM31; 1] = (*input_state_1_limb_25_col55
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_1_limb_26_col56]: [QM31; 1] = (*input_state_1_limb_26_col56
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_1_limb_27_col57]: [QM31; 1] = (*input_state_1_limb_27_col57
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_2_id_col58]: [QM31; 1] = (*input_state_2_id_col58.try_into().unwrap())
            .unbox();
        let [input_state_2_limb_0_col59]: [QM31; 1] = (*input_state_2_limb_0_col59
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_2_limb_1_col60]: [QM31; 1] = (*input_state_2_limb_1_col60
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_2_limb_2_col61]: [QM31; 1] = (*input_state_2_limb_2_col61
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_2_limb_3_col62]: [QM31; 1] = (*input_state_2_limb_3_col62
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_2_limb_4_col63]: [QM31; 1] = (*input_state_2_limb_4_col63
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_2_limb_5_col64]: [QM31; 1] = (*input_state_2_limb_5_col64
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_2_limb_6_col65]: [QM31; 1] = (*input_state_2_limb_6_col65
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_2_limb_7_col66]: [QM31; 1] = (*input_state_2_limb_7_col66
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_2_limb_8_col67]: [QM31; 1] = (*input_state_2_limb_8_col67
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_2_limb_9_col68]: [QM31; 1] = (*input_state_2_limb_9_col68
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_2_limb_10_col69]: [QM31; 1] = (*input_state_2_limb_10_col69
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_2_limb_11_col70]: [QM31; 1] = (*input_state_2_limb_11_col70
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_2_limb_12_col71]: [QM31; 1] = (*input_state_2_limb_12_col71
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_2_limb_13_col72]: [QM31; 1] = (*input_state_2_limb_13_col72
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_2_limb_14_col73]: [QM31; 1] = (*input_state_2_limb_14_col73
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_2_limb_15_col74]: [QM31; 1] = (*input_state_2_limb_15_col74
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_2_limb_16_col75]: [QM31; 1] = (*input_state_2_limb_16_col75
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_2_limb_17_col76]: [QM31; 1] = (*input_state_2_limb_17_col76
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_2_limb_18_col77]: [QM31; 1] = (*input_state_2_limb_18_col77
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_2_limb_19_col78]: [QM31; 1] = (*input_state_2_limb_19_col78
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_2_limb_20_col79]: [QM31; 1] = (*input_state_2_limb_20_col79
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_2_limb_21_col80]: [QM31; 1] = (*input_state_2_limb_21_col80
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_2_limb_22_col81]: [QM31; 1] = (*input_state_2_limb_22_col81
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_2_limb_23_col82]: [QM31; 1] = (*input_state_2_limb_23_col82
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_2_limb_24_col83]: [QM31; 1] = (*input_state_2_limb_24_col83
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_2_limb_25_col84]: [QM31; 1] = (*input_state_2_limb_25_col84
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_2_limb_26_col85]: [QM31; 1] = (*input_state_2_limb_26_col85
            .try_into()
            .unwrap())
            .unbox();
        let [input_state_2_limb_27_col86]: [QM31; 1] = (*input_state_2_limb_27_col86
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_0_col87]: [QM31; 1] = (*combination_limb_0_col87.try_into().unwrap())
            .unbox();
        let [combination_limb_1_col88]: [QM31; 1] = (*combination_limb_1_col88.try_into().unwrap())
            .unbox();
        let [combination_limb_2_col89]: [QM31; 1] = (*combination_limb_2_col89.try_into().unwrap())
            .unbox();
        let [combination_limb_3_col90]: [QM31; 1] = (*combination_limb_3_col90.try_into().unwrap())
            .unbox();
        let [combination_limb_4_col91]: [QM31; 1] = (*combination_limb_4_col91.try_into().unwrap())
            .unbox();
        let [combination_limb_5_col92]: [QM31; 1] = (*combination_limb_5_col92.try_into().unwrap())
            .unbox();
        let [combination_limb_6_col93]: [QM31; 1] = (*combination_limb_6_col93.try_into().unwrap())
            .unbox();
        let [combination_limb_7_col94]: [QM31; 1] = (*combination_limb_7_col94.try_into().unwrap())
            .unbox();
        let [combination_limb_8_col95]: [QM31; 1] = (*combination_limb_8_col95.try_into().unwrap())
            .unbox();
        let [combination_limb_9_col96]: [QM31; 1] = (*combination_limb_9_col96.try_into().unwrap())
            .unbox();
        let [p_coef_col97]: [QM31; 1] = (*p_coef_col97.try_into().unwrap()).unbox();
        let [combination_limb_0_col98]: [QM31; 1] = (*combination_limb_0_col98.try_into().unwrap())
            .unbox();
        let [combination_limb_1_col99]: [QM31; 1] = (*combination_limb_1_col99.try_into().unwrap())
            .unbox();
        let [combination_limb_2_col100]: [QM31; 1] = (*combination_limb_2_col100
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_3_col101]: [QM31; 1] = (*combination_limb_3_col101
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_4_col102]: [QM31; 1] = (*combination_limb_4_col102
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_5_col103]: [QM31; 1] = (*combination_limb_5_col103
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_6_col104]: [QM31; 1] = (*combination_limb_6_col104
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_7_col105]: [QM31; 1] = (*combination_limb_7_col105
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_8_col106]: [QM31; 1] = (*combination_limb_8_col106
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_9_col107]: [QM31; 1] = (*combination_limb_9_col107
            .try_into()
            .unwrap())
            .unbox();
        let [p_coef_col108]: [QM31; 1] = (*p_coef_col108.try_into().unwrap()).unbox();
        let [combination_limb_0_col109]: [QM31; 1] = (*combination_limb_0_col109
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_1_col110]: [QM31; 1] = (*combination_limb_1_col110
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_2_col111]: [QM31; 1] = (*combination_limb_2_col111
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_3_col112]: [QM31; 1] = (*combination_limb_3_col112
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_4_col113]: [QM31; 1] = (*combination_limb_4_col113
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_5_col114]: [QM31; 1] = (*combination_limb_5_col114
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_6_col115]: [QM31; 1] = (*combination_limb_6_col115
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_7_col116]: [QM31; 1] = (*combination_limb_7_col116
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_8_col117]: [QM31; 1] = (*combination_limb_8_col117
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_9_col118]: [QM31; 1] = (*combination_limb_9_col118
            .try_into()
            .unwrap())
            .unbox();
        let [p_coef_col119]: [QM31; 1] = (*p_coef_col119.try_into().unwrap()).unbox();
        let [poseidon_full_round_chain_output_limb_0_col120]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_0_col120
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_1_col121]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_1_col121
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_2_col122]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_2_col122
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_3_col123]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_3_col123
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_4_col124]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_4_col124
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_5_col125]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_5_col125
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_6_col126]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_6_col126
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_7_col127]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_7_col127
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_8_col128]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_8_col128
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_9_col129]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_9_col129
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_10_col130]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_10_col130
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_11_col131]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_11_col131
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_12_col132]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_12_col132
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_13_col133]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_13_col133
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_14_col134]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_14_col134
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_15_col135]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_15_col135
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_16_col136]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_16_col136
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_17_col137]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_17_col137
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_18_col138]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_18_col138
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_19_col139]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_19_col139
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_20_col140]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_20_col140
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_21_col141]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_21_col141
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_22_col142]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_22_col142
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_23_col143]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_23_col143
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_24_col144]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_24_col144
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_25_col145]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_25_col145
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_26_col146]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_26_col146
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_27_col147]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_27_col147
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_28_col148]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_28_col148
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_29_col149]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_29_col149
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_0_col150]: [QM31; 1] = (*cube_252_output_limb_0_col150
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_1_col151]: [QM31; 1] = (*cube_252_output_limb_1_col151
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_2_col152]: [QM31; 1] = (*cube_252_output_limb_2_col152
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_3_col153]: [QM31; 1] = (*cube_252_output_limb_3_col153
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_4_col154]: [QM31; 1] = (*cube_252_output_limb_4_col154
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_5_col155]: [QM31; 1] = (*cube_252_output_limb_5_col155
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_6_col156]: [QM31; 1] = (*cube_252_output_limb_6_col156
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_7_col157]: [QM31; 1] = (*cube_252_output_limb_7_col157
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_8_col158]: [QM31; 1] = (*cube_252_output_limb_8_col158
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_9_col159]: [QM31; 1] = (*cube_252_output_limb_9_col159
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_0_col160]: [QM31; 1] = (*combination_limb_0_col160
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_1_col161]: [QM31; 1] = (*combination_limb_1_col161
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_2_col162]: [QM31; 1] = (*combination_limb_2_col162
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_3_col163]: [QM31; 1] = (*combination_limb_3_col163
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_4_col164]: [QM31; 1] = (*combination_limb_4_col164
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_5_col165]: [QM31; 1] = (*combination_limb_5_col165
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_6_col166]: [QM31; 1] = (*combination_limb_6_col166
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_7_col167]: [QM31; 1] = (*combination_limb_7_col167
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_8_col168]: [QM31; 1] = (*combination_limb_8_col168
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_9_col169]: [QM31; 1] = (*combination_limb_9_col169
            .try_into()
            .unwrap())
            .unbox();
        let [p_coef_col170]: [QM31; 1] = (*p_coef_col170.try_into().unwrap()).unbox();
        let [cube_252_output_limb_0_col171]: [QM31; 1] = (*cube_252_output_limb_0_col171
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_1_col172]: [QM31; 1] = (*cube_252_output_limb_1_col172
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_2_col173]: [QM31; 1] = (*cube_252_output_limb_2_col173
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_3_col174]: [QM31; 1] = (*cube_252_output_limb_3_col174
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_4_col175]: [QM31; 1] = (*cube_252_output_limb_4_col175
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_5_col176]: [QM31; 1] = (*cube_252_output_limb_5_col176
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_6_col177]: [QM31; 1] = (*cube_252_output_limb_6_col177
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_7_col178]: [QM31; 1] = (*cube_252_output_limb_7_col178
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_8_col179]: [QM31; 1] = (*cube_252_output_limb_8_col179
            .try_into()
            .unwrap())
            .unbox();
        let [cube_252_output_limb_9_col180]: [QM31; 1] = (*cube_252_output_limb_9_col180
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_0_col181]: [QM31; 1] = (*combination_limb_0_col181
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_1_col182]: [QM31; 1] = (*combination_limb_1_col182
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_2_col183]: [QM31; 1] = (*combination_limb_2_col183
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_3_col184]: [QM31; 1] = (*combination_limb_3_col184
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_4_col185]: [QM31; 1] = (*combination_limb_4_col185
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_5_col186]: [QM31; 1] = (*combination_limb_5_col186
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_6_col187]: [QM31; 1] = (*combination_limb_6_col187
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_7_col188]: [QM31; 1] = (*combination_limb_7_col188
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_8_col189]: [QM31; 1] = (*combination_limb_8_col189
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_9_col190]: [QM31; 1] = (*combination_limb_9_col190
            .try_into()
            .unwrap())
            .unbox();
        let [p_coef_col191]: [QM31; 1] = (*p_coef_col191.try_into().unwrap()).unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_0_col192]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_0_col192
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_1_col193]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_1_col193
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_2_col194]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_2_col194
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_3_col195]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_3_col195
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_4_col196]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_4_col196
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_5_col197]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_5_col197
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_6_col198]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_6_col198
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_7_col199]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_7_col199
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_8_col200]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_8_col200
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_9_col201]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_9_col201
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_10_col202]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_10_col202
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_11_col203]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_11_col203
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_12_col204]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_12_col204
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_13_col205]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_13_col205
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_14_col206]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_14_col206
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_15_col207]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_15_col207
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_16_col208]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_16_col208
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_17_col209]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_17_col209
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_18_col210]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_18_col210
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_19_col211]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_19_col211
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_20_col212]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_20_col212
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_21_col213]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_21_col213
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_22_col214]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_22_col214
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_23_col215]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_23_col215
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_24_col216]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_24_col216
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_25_col217]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_25_col217
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_26_col218]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_26_col218
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_27_col219]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_27_col219
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_28_col220]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_28_col220
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_29_col221]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_29_col221
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_30_col222]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_30_col222
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_31_col223]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_31_col223
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_32_col224]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_32_col224
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_33_col225]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_33_col225
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_34_col226]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_34_col226
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_35_col227]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_35_col227
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_36_col228]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_36_col228
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_37_col229]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_37_col229
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_38_col230]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_38_col230
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_3_partial_rounds_chain_output_limb_39_col231]: [QM31; 1] =
            (*poseidon_3_partial_rounds_chain_output_limb_39_col231
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_0_col232]: [QM31; 1] = (*combination_limb_0_col232
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_1_col233]: [QM31; 1] = (*combination_limb_1_col233
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_2_col234]: [QM31; 1] = (*combination_limb_2_col234
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_3_col235]: [QM31; 1] = (*combination_limb_3_col235
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_4_col236]: [QM31; 1] = (*combination_limb_4_col236
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_5_col237]: [QM31; 1] = (*combination_limb_5_col237
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_6_col238]: [QM31; 1] = (*combination_limb_6_col238
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_7_col239]: [QM31; 1] = (*combination_limb_7_col239
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_8_col240]: [QM31; 1] = (*combination_limb_8_col240
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_9_col241]: [QM31; 1] = (*combination_limb_9_col241
            .try_into()
            .unwrap())
            .unbox();
        let [p_coef_col242]: [QM31; 1] = (*p_coef_col242.try_into().unwrap()).unbox();
        let [combination_limb_0_col243]: [QM31; 1] = (*combination_limb_0_col243
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_1_col244]: [QM31; 1] = (*combination_limb_1_col244
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_2_col245]: [QM31; 1] = (*combination_limb_2_col245
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_3_col246]: [QM31; 1] = (*combination_limb_3_col246
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_4_col247]: [QM31; 1] = (*combination_limb_4_col247
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_5_col248]: [QM31; 1] = (*combination_limb_5_col248
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_6_col249]: [QM31; 1] = (*combination_limb_6_col249
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_7_col250]: [QM31; 1] = (*combination_limb_7_col250
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_8_col251]: [QM31; 1] = (*combination_limb_8_col251
            .try_into()
            .unwrap())
            .unbox();
        let [combination_limb_9_col252]: [QM31; 1] = (*combination_limb_9_col252
            .try_into()
            .unwrap())
            .unbox();
        let [p_coef_col253]: [QM31; 1] = (*p_coef_col253.try_into().unwrap()).unbox();
        let [poseidon_full_round_chain_output_limb_0_col254]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_0_col254
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_1_col255]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_1_col255
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_2_col256]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_2_col256
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_3_col257]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_3_col257
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_4_col258]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_4_col258
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_5_col259]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_5_col259
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_6_col260]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_6_col260
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_7_col261]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_7_col261
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_8_col262]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_8_col262
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_9_col263]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_9_col263
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_10_col264]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_10_col264
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_11_col265]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_11_col265
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_12_col266]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_12_col266
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_13_col267]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_13_col267
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_14_col268]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_14_col268
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_15_col269]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_15_col269
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_16_col270]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_16_col270
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_17_col271]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_17_col271
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_18_col272]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_18_col272
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_19_col273]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_19_col273
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_20_col274]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_20_col274
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_21_col275]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_21_col275
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_22_col276]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_22_col276
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_23_col277]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_23_col277
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_24_col278]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_24_col278
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_25_col279]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_25_col279
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_26_col280]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_26_col280
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_27_col281]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_27_col281
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_28_col282]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_28_col282
            .try_into()
            .unwrap())
            .unbox();
        let [poseidon_full_round_chain_output_limb_29_col283]: [QM31; 1] =
            (*poseidon_full_round_chain_output_limb_29_col283
            .try_into()
            .unwrap())
            .unbox();
        let [unpacked_limb_0_col284]: [QM31; 1] = (*unpacked_limb_0_col284.try_into().unwrap())
            .unbox();
        let [unpacked_limb_1_col285]: [QM31; 1] = (*unpacked_limb_1_col285.try_into().unwrap())
            .unbox();
        let [unpacked_limb_3_col286]: [QM31; 1] = (*unpacked_limb_3_col286.try_into().unwrap())
            .unbox();
        let [unpacked_limb_4_col287]: [QM31; 1] = (*unpacked_limb_4_col287.try_into().unwrap())
            .unbox();
        let [unpacked_limb_6_col288]: [QM31; 1] = (*unpacked_limb_6_col288.try_into().unwrap())
            .unbox();
        let [unpacked_limb_7_col289]: [QM31; 1] = (*unpacked_limb_7_col289.try_into().unwrap())
            .unbox();
        let [unpacked_limb_9_col290]: [QM31; 1] = (*unpacked_limb_9_col290.try_into().unwrap())
            .unbox();
        let [unpacked_limb_10_col291]: [QM31; 1] = (*unpacked_limb_10_col291.try_into().unwrap())
            .unbox();
        let [unpacked_limb_12_col292]: [QM31; 1] = (*unpacked_limb_12_col292.try_into().unwrap())
            .unbox();
        let [unpacked_limb_13_col293]: [QM31; 1] = (*unpacked_limb_13_col293.try_into().unwrap())
            .unbox();
        let [unpacked_limb_15_col294]: [QM31; 1] = (*unpacked_limb_15_col294.try_into().unwrap())
            .unbox();
        let [unpacked_limb_16_col295]: [QM31; 1] = (*unpacked_limb_16_col295.try_into().unwrap())
            .unbox();
        let [unpacked_limb_18_col296]: [QM31; 1] = (*unpacked_limb_18_col296.try_into().unwrap())
            .unbox();
        let [unpacked_limb_19_col297]: [QM31; 1] = (*unpacked_limb_19_col297.try_into().unwrap())
            .unbox();
        let [unpacked_limb_21_col298]: [QM31; 1] = (*unpacked_limb_21_col298.try_into().unwrap())
            .unbox();
        let [unpacked_limb_22_col299]: [QM31; 1] = (*unpacked_limb_22_col299.try_into().unwrap())
            .unbox();
        let [unpacked_limb_24_col300]: [QM31; 1] = (*unpacked_limb_24_col300.try_into().unwrap())
            .unbox();
        let [unpacked_limb_25_col301]: [QM31; 1] = (*unpacked_limb_25_col301.try_into().unwrap())
            .unbox();
        let [output_state_0_id_col302]: [QM31; 1] = (*output_state_0_id_col302.try_into().unwrap())
            .unbox();
        let [unpacked_limb_0_col303]: [QM31; 1] = (*unpacked_limb_0_col303.try_into().unwrap())
            .unbox();
        let [unpacked_limb_1_col304]: [QM31; 1] = (*unpacked_limb_1_col304.try_into().unwrap())
            .unbox();
        let [unpacked_limb_3_col305]: [QM31; 1] = (*unpacked_limb_3_col305.try_into().unwrap())
            .unbox();
        let [unpacked_limb_4_col306]: [QM31; 1] = (*unpacked_limb_4_col306.try_into().unwrap())
            .unbox();
        let [unpacked_limb_6_col307]: [QM31; 1] = (*unpacked_limb_6_col307.try_into().unwrap())
            .unbox();
        let [unpacked_limb_7_col308]: [QM31; 1] = (*unpacked_limb_7_col308.try_into().unwrap())
            .unbox();
        let [unpacked_limb_9_col309]: [QM31; 1] = (*unpacked_limb_9_col309.try_into().unwrap())
            .unbox();
        let [unpacked_limb_10_col310]: [QM31; 1] = (*unpacked_limb_10_col310.try_into().unwrap())
            .unbox();
        let [unpacked_limb_12_col311]: [QM31; 1] = (*unpacked_limb_12_col311.try_into().unwrap())
            .unbox();
        let [unpacked_limb_13_col312]: [QM31; 1] = (*unpacked_limb_13_col312.try_into().unwrap())
            .unbox();
        let [unpacked_limb_15_col313]: [QM31; 1] = (*unpacked_limb_15_col313.try_into().unwrap())
            .unbox();
        let [unpacked_limb_16_col314]: [QM31; 1] = (*unpacked_limb_16_col314.try_into().unwrap())
            .unbox();
        let [unpacked_limb_18_col315]: [QM31; 1] = (*unpacked_limb_18_col315.try_into().unwrap())
            .unbox();
        let [unpacked_limb_19_col316]: [QM31; 1] = (*unpacked_limb_19_col316.try_into().unwrap())
            .unbox();
        let [unpacked_limb_21_col317]: [QM31; 1] = (*unpacked_limb_21_col317.try_into().unwrap())
            .unbox();
        let [unpacked_limb_22_col318]: [QM31; 1] = (*unpacked_limb_22_col318.try_into().unwrap())
            .unbox();
        let [unpacked_limb_24_col319]: [QM31; 1] = (*unpacked_limb_24_col319.try_into().unwrap())
            .unbox();
        let [unpacked_limb_25_col320]: [QM31; 1] = (*unpacked_limb_25_col320.try_into().unwrap())
            .unbox();
        let [output_state_1_id_col321]: [QM31; 1] = (*output_state_1_id_col321.try_into().unwrap())
            .unbox();
        let [unpacked_limb_0_col322]: [QM31; 1] = (*unpacked_limb_0_col322.try_into().unwrap())
            .unbox();
        let [unpacked_limb_1_col323]: [QM31; 1] = (*unpacked_limb_1_col323.try_into().unwrap())
            .unbox();
        let [unpacked_limb_3_col324]: [QM31; 1] = (*unpacked_limb_3_col324.try_into().unwrap())
            .unbox();
        let [unpacked_limb_4_col325]: [QM31; 1] = (*unpacked_limb_4_col325.try_into().unwrap())
            .unbox();
        let [unpacked_limb_6_col326]: [QM31; 1] = (*unpacked_limb_6_col326.try_into().unwrap())
            .unbox();
        let [unpacked_limb_7_col327]: [QM31; 1] = (*unpacked_limb_7_col327.try_into().unwrap())
            .unbox();
        let [unpacked_limb_9_col328]: [QM31; 1] = (*unpacked_limb_9_col328.try_into().unwrap())
            .unbox();
        let [unpacked_limb_10_col329]: [QM31; 1] = (*unpacked_limb_10_col329.try_into().unwrap())
            .unbox();
        let [unpacked_limb_12_col330]: [QM31; 1] = (*unpacked_limb_12_col330.try_into().unwrap())
            .unbox();
        let [unpacked_limb_13_col331]: [QM31; 1] = (*unpacked_limb_13_col331.try_into().unwrap())
            .unbox();
        let [unpacked_limb_15_col332]: [QM31; 1] = (*unpacked_limb_15_col332.try_into().unwrap())
            .unbox();
        let [unpacked_limb_16_col333]: [QM31; 1] = (*unpacked_limb_16_col333.try_into().unwrap())
            .unbox();
        let [unpacked_limb_18_col334]: [QM31; 1] = (*unpacked_limb_18_col334.try_into().unwrap())
            .unbox();
        let [unpacked_limb_19_col335]: [QM31; 1] = (*unpacked_limb_19_col335.try_into().unwrap())
            .unbox();
        let [unpacked_limb_21_col336]: [QM31; 1] = (*unpacked_limb_21_col336.try_into().unwrap())
            .unbox();
        let [unpacked_limb_22_col337]: [QM31; 1] = (*unpacked_limb_22_col337.try_into().unwrap())
            .unbox();
        let [unpacked_limb_24_col338]: [QM31; 1] = (*unpacked_limb_24_col338.try_into().unwrap())
            .unbox();
        let [unpacked_limb_25_col339]: [QM31; 1] = (*unpacked_limb_25_col339.try_into().unwrap())
            .unbox();
        let [output_state_2_id_col340]: [QM31; 1] = (*output_state_2_id_col340.try_into().unwrap())
            .unbox();

        core::internal::revoke_ap_tracking();

        read_positive_num_bits_252_evaluate(
            [(poseidon_builtin_segment_start + (seq * qm31_const::<6, 0, 0, 0>()))],
            input_state_0_id_col0,
            input_state_0_limb_0_col1,
            input_state_0_limb_1_col2,
            input_state_0_limb_2_col3,
            input_state_0_limb_3_col4,
            input_state_0_limb_4_col5,
            input_state_0_limb_5_col6,
            input_state_0_limb_6_col7,
            input_state_0_limb_7_col8,
            input_state_0_limb_8_col9,
            input_state_0_limb_9_col10,
            input_state_0_limb_10_col11,
            input_state_0_limb_11_col12,
            input_state_0_limb_12_col13,
            input_state_0_limb_13_col14,
            input_state_0_limb_14_col15,
            input_state_0_limb_15_col16,
            input_state_0_limb_16_col17,
            input_state_0_limb_17_col18,
            input_state_0_limb_18_col19,
            input_state_0_limb_19_col20,
            input_state_0_limb_20_col21,
            input_state_0_limb_21_col22,
            input_state_0_limb_22_col23,
            input_state_0_limb_23_col24,
            input_state_0_limb_24_col25,
            input_state_0_limb_25_col26,
            input_state_0_limb_26_col27,
            input_state_0_limb_27_col28,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref memory_address_to_id_sum_0,
            ref memory_id_to_big_sum_1,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let packed_input_state_0_tmp_51986_3_limb_0: QM31 = ((input_state_0_limb_0_col1
            + (input_state_0_limb_1_col2 * qm31_const::<512, 0, 0, 0>()))
            + (input_state_0_limb_2_col3 * qm31_const::<262144, 0, 0, 0>()));
        let packed_input_state_0_tmp_51986_3_limb_1: QM31 = ((input_state_0_limb_3_col4
            + (input_state_0_limb_4_col5 * qm31_const::<512, 0, 0, 0>()))
            + (input_state_0_limb_5_col6 * qm31_const::<262144, 0, 0, 0>()));
        let packed_input_state_0_tmp_51986_3_limb_2: QM31 = ((input_state_0_limb_6_col7
            + (input_state_0_limb_7_col8 * qm31_const::<512, 0, 0, 0>()))
            + (input_state_0_limb_8_col9 * qm31_const::<262144, 0, 0, 0>()));
        let packed_input_state_0_tmp_51986_3_limb_3: QM31 = ((input_state_0_limb_9_col10
            + (input_state_0_limb_10_col11 * qm31_const::<512, 0, 0, 0>()))
            + (input_state_0_limb_11_col12 * qm31_const::<262144, 0, 0, 0>()));
        let packed_input_state_0_tmp_51986_3_limb_4: QM31 = ((input_state_0_limb_12_col13
            + (input_state_0_limb_13_col14 * qm31_const::<512, 0, 0, 0>()))
            + (input_state_0_limb_14_col15 * qm31_const::<262144, 0, 0, 0>()));
        let packed_input_state_0_tmp_51986_3_limb_5: QM31 = ((input_state_0_limb_15_col16
            + (input_state_0_limb_16_col17 * qm31_const::<512, 0, 0, 0>()))
            + (input_state_0_limb_17_col18 * qm31_const::<262144, 0, 0, 0>()));
        let packed_input_state_0_tmp_51986_3_limb_6: QM31 = ((input_state_0_limb_18_col19
            + (input_state_0_limb_19_col20 * qm31_const::<512, 0, 0, 0>()))
            + (input_state_0_limb_20_col21 * qm31_const::<262144, 0, 0, 0>()));
        let packed_input_state_0_tmp_51986_3_limb_7: QM31 = ((input_state_0_limb_21_col22
            + (input_state_0_limb_22_col23 * qm31_const::<512, 0, 0, 0>()))
            + (input_state_0_limb_23_col24 * qm31_const::<262144, 0, 0, 0>()));
        let packed_input_state_0_tmp_51986_3_limb_8: QM31 = ((input_state_0_limb_24_col25
            + (input_state_0_limb_25_col26 * qm31_const::<512, 0, 0, 0>()))
            + (input_state_0_limb_26_col27 * qm31_const::<262144, 0, 0, 0>()));

        read_positive_num_bits_252_evaluate(
            [
                ((poseidon_builtin_segment_start + (seq * qm31_const::<6, 0, 0, 0>()))
                    + qm31_const::<1, 0, 0, 0>())
            ],
            input_state_1_id_col29,
            input_state_1_limb_0_col30,
            input_state_1_limb_1_col31,
            input_state_1_limb_2_col32,
            input_state_1_limb_3_col33,
            input_state_1_limb_4_col34,
            input_state_1_limb_5_col35,
            input_state_1_limb_6_col36,
            input_state_1_limb_7_col37,
            input_state_1_limb_8_col38,
            input_state_1_limb_9_col39,
            input_state_1_limb_10_col40,
            input_state_1_limb_11_col41,
            input_state_1_limb_12_col42,
            input_state_1_limb_13_col43,
            input_state_1_limb_14_col44,
            input_state_1_limb_15_col45,
            input_state_1_limb_16_col46,
            input_state_1_limb_17_col47,
            input_state_1_limb_18_col48,
            input_state_1_limb_19_col49,
            input_state_1_limb_20_col50,
            input_state_1_limb_21_col51,
            input_state_1_limb_22_col52,
            input_state_1_limb_23_col53,
            input_state_1_limb_24_col54,
            input_state_1_limb_25_col55,
            input_state_1_limb_26_col56,
            input_state_1_limb_27_col57,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref memory_address_to_id_sum_2,
            ref memory_id_to_big_sum_3,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let packed_input_state_1_tmp_51986_7_limb_0: QM31 = ((input_state_1_limb_0_col30
            + (input_state_1_limb_1_col31 * qm31_const::<512, 0, 0, 0>()))
            + (input_state_1_limb_2_col32 * qm31_const::<262144, 0, 0, 0>()));
        let packed_input_state_1_tmp_51986_7_limb_1: QM31 = ((input_state_1_limb_3_col33
            + (input_state_1_limb_4_col34 * qm31_const::<512, 0, 0, 0>()))
            + (input_state_1_limb_5_col35 * qm31_const::<262144, 0, 0, 0>()));
        let packed_input_state_1_tmp_51986_7_limb_2: QM31 = ((input_state_1_limb_6_col36
            + (input_state_1_limb_7_col37 * qm31_const::<512, 0, 0, 0>()))
            + (input_state_1_limb_8_col38 * qm31_const::<262144, 0, 0, 0>()));
        let packed_input_state_1_tmp_51986_7_limb_3: QM31 = ((input_state_1_limb_9_col39
            + (input_state_1_limb_10_col40 * qm31_const::<512, 0, 0, 0>()))
            + (input_state_1_limb_11_col41 * qm31_const::<262144, 0, 0, 0>()));
        let packed_input_state_1_tmp_51986_7_limb_4: QM31 = ((input_state_1_limb_12_col42
            + (input_state_1_limb_13_col43 * qm31_const::<512, 0, 0, 0>()))
            + (input_state_1_limb_14_col44 * qm31_const::<262144, 0, 0, 0>()));
        let packed_input_state_1_tmp_51986_7_limb_5: QM31 = ((input_state_1_limb_15_col45
            + (input_state_1_limb_16_col46 * qm31_const::<512, 0, 0, 0>()))
            + (input_state_1_limb_17_col47 * qm31_const::<262144, 0, 0, 0>()));
        let packed_input_state_1_tmp_51986_7_limb_6: QM31 = ((input_state_1_limb_18_col48
            + (input_state_1_limb_19_col49 * qm31_const::<512, 0, 0, 0>()))
            + (input_state_1_limb_20_col50 * qm31_const::<262144, 0, 0, 0>()));
        let packed_input_state_1_tmp_51986_7_limb_7: QM31 = ((input_state_1_limb_21_col51
            + (input_state_1_limb_22_col52 * qm31_const::<512, 0, 0, 0>()))
            + (input_state_1_limb_23_col53 * qm31_const::<262144, 0, 0, 0>()));
        let packed_input_state_1_tmp_51986_7_limb_8: QM31 = ((input_state_1_limb_24_col54
            + (input_state_1_limb_25_col55 * qm31_const::<512, 0, 0, 0>()))
            + (input_state_1_limb_26_col56 * qm31_const::<262144, 0, 0, 0>()));

        read_positive_num_bits_252_evaluate(
            [
                ((poseidon_builtin_segment_start + (seq * qm31_const::<6, 0, 0, 0>()))
                    + qm31_const::<2, 0, 0, 0>())
            ],
            input_state_2_id_col58,
            input_state_2_limb_0_col59,
            input_state_2_limb_1_col60,
            input_state_2_limb_2_col61,
            input_state_2_limb_3_col62,
            input_state_2_limb_4_col63,
            input_state_2_limb_5_col64,
            input_state_2_limb_6_col65,
            input_state_2_limb_7_col66,
            input_state_2_limb_8_col67,
            input_state_2_limb_9_col68,
            input_state_2_limb_10_col69,
            input_state_2_limb_11_col70,
            input_state_2_limb_12_col71,
            input_state_2_limb_13_col72,
            input_state_2_limb_14_col73,
            input_state_2_limb_15_col74,
            input_state_2_limb_16_col75,
            input_state_2_limb_17_col76,
            input_state_2_limb_18_col77,
            input_state_2_limb_19_col78,
            input_state_2_limb_20_col79,
            input_state_2_limb_21_col80,
            input_state_2_limb_22_col81,
            input_state_2_limb_23_col82,
            input_state_2_limb_24_col83,
            input_state_2_limb_25_col84,
            input_state_2_limb_26_col85,
            input_state_2_limb_27_col86,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref memory_address_to_id_sum_4,
            ref memory_id_to_big_sum_5,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let packed_input_state_2_tmp_51986_11_limb_0: QM31 = ((input_state_2_limb_0_col59
            + (input_state_2_limb_1_col60 * qm31_const::<512, 0, 0, 0>()))
            + (input_state_2_limb_2_col61 * qm31_const::<262144, 0, 0, 0>()));
        let packed_input_state_2_tmp_51986_11_limb_1: QM31 = ((input_state_2_limb_3_col62
            + (input_state_2_limb_4_col63 * qm31_const::<512, 0, 0, 0>()))
            + (input_state_2_limb_5_col64 * qm31_const::<262144, 0, 0, 0>()));
        let packed_input_state_2_tmp_51986_11_limb_2: QM31 = ((input_state_2_limb_6_col65
            + (input_state_2_limb_7_col66 * qm31_const::<512, 0, 0, 0>()))
            + (input_state_2_limb_8_col67 * qm31_const::<262144, 0, 0, 0>()));
        let packed_input_state_2_tmp_51986_11_limb_3: QM31 = ((input_state_2_limb_9_col68
            + (input_state_2_limb_10_col69 * qm31_const::<512, 0, 0, 0>()))
            + (input_state_2_limb_11_col70 * qm31_const::<262144, 0, 0, 0>()));
        let packed_input_state_2_tmp_51986_11_limb_4: QM31 = ((input_state_2_limb_12_col71
            + (input_state_2_limb_13_col72 * qm31_const::<512, 0, 0, 0>()))
            + (input_state_2_limb_14_col73 * qm31_const::<262144, 0, 0, 0>()));
        let packed_input_state_2_tmp_51986_11_limb_5: QM31 = ((input_state_2_limb_15_col74
            + (input_state_2_limb_16_col75 * qm31_const::<512, 0, 0, 0>()))
            + (input_state_2_limb_17_col76 * qm31_const::<262144, 0, 0, 0>()));
        let packed_input_state_2_tmp_51986_11_limb_6: QM31 = ((input_state_2_limb_18_col77
            + (input_state_2_limb_19_col78 * qm31_const::<512, 0, 0, 0>()))
            + (input_state_2_limb_20_col79 * qm31_const::<262144, 0, 0, 0>()));
        let packed_input_state_2_tmp_51986_11_limb_7: QM31 = ((input_state_2_limb_21_col80
            + (input_state_2_limb_22_col81 * qm31_const::<512, 0, 0, 0>()))
            + (input_state_2_limb_23_col82 * qm31_const::<262144, 0, 0, 0>()));
        let packed_input_state_2_tmp_51986_11_limb_8: QM31 = ((input_state_2_limb_24_col83
            + (input_state_2_limb_25_col84 * qm31_const::<512, 0, 0, 0>()))
            + (input_state_2_limb_26_col85 * qm31_const::<262144, 0, 0, 0>()));

        poseidon_hades_permutation_evaluate(
            [
                packed_input_state_0_tmp_51986_3_limb_0, packed_input_state_0_tmp_51986_3_limb_1,
                packed_input_state_0_tmp_51986_3_limb_2, packed_input_state_0_tmp_51986_3_limb_3,
                packed_input_state_0_tmp_51986_3_limb_4, packed_input_state_0_tmp_51986_3_limb_5,
                packed_input_state_0_tmp_51986_3_limb_6, packed_input_state_0_tmp_51986_3_limb_7,
                packed_input_state_0_tmp_51986_3_limb_8, input_state_0_limb_27_col28,
                packed_input_state_1_tmp_51986_7_limb_0, packed_input_state_1_tmp_51986_7_limb_1,
                packed_input_state_1_tmp_51986_7_limb_2, packed_input_state_1_tmp_51986_7_limb_3,
                packed_input_state_1_tmp_51986_7_limb_4, packed_input_state_1_tmp_51986_7_limb_5,
                packed_input_state_1_tmp_51986_7_limb_6, packed_input_state_1_tmp_51986_7_limb_7,
                packed_input_state_1_tmp_51986_7_limb_8, input_state_1_limb_27_col57,
                packed_input_state_2_tmp_51986_11_limb_0, packed_input_state_2_tmp_51986_11_limb_1,
                packed_input_state_2_tmp_51986_11_limb_2, packed_input_state_2_tmp_51986_11_limb_3,
                packed_input_state_2_tmp_51986_11_limb_4, packed_input_state_2_tmp_51986_11_limb_5,
                packed_input_state_2_tmp_51986_11_limb_6, packed_input_state_2_tmp_51986_11_limb_7,
                packed_input_state_2_tmp_51986_11_limb_8, input_state_2_limb_27_col86,
            ],
            combination_limb_0_col87,
            combination_limb_1_col88,
            combination_limb_2_col89,
            combination_limb_3_col90,
            combination_limb_4_col91,
            combination_limb_5_col92,
            combination_limb_6_col93,
            combination_limb_7_col94,
            combination_limb_8_col95,
            combination_limb_9_col96,
            p_coef_col97,
            combination_limb_0_col98,
            combination_limb_1_col99,
            combination_limb_2_col100,
            combination_limb_3_col101,
            combination_limb_4_col102,
            combination_limb_5_col103,
            combination_limb_6_col104,
            combination_limb_7_col105,
            combination_limb_8_col106,
            combination_limb_9_col107,
            p_coef_col108,
            combination_limb_0_col109,
            combination_limb_1_col110,
            combination_limb_2_col111,
            combination_limb_3_col112,
            combination_limb_4_col113,
            combination_limb_5_col114,
            combination_limb_6_col115,
            combination_limb_7_col116,
            combination_limb_8_col117,
            combination_limb_9_col118,
            p_coef_col119,
            poseidon_full_round_chain_output_limb_0_col120,
            poseidon_full_round_chain_output_limb_1_col121,
            poseidon_full_round_chain_output_limb_2_col122,
            poseidon_full_round_chain_output_limb_3_col123,
            poseidon_full_round_chain_output_limb_4_col124,
            poseidon_full_round_chain_output_limb_5_col125,
            poseidon_full_round_chain_output_limb_6_col126,
            poseidon_full_round_chain_output_limb_7_col127,
            poseidon_full_round_chain_output_limb_8_col128,
            poseidon_full_round_chain_output_limb_9_col129,
            poseidon_full_round_chain_output_limb_10_col130,
            poseidon_full_round_chain_output_limb_11_col131,
            poseidon_full_round_chain_output_limb_12_col132,
            poseidon_full_round_chain_output_limb_13_col133,
            poseidon_full_round_chain_output_limb_14_col134,
            poseidon_full_round_chain_output_limb_15_col135,
            poseidon_full_round_chain_output_limb_16_col136,
            poseidon_full_round_chain_output_limb_17_col137,
            poseidon_full_round_chain_output_limb_18_col138,
            poseidon_full_round_chain_output_limb_19_col139,
            poseidon_full_round_chain_output_limb_20_col140,
            poseidon_full_round_chain_output_limb_21_col141,
            poseidon_full_round_chain_output_limb_22_col142,
            poseidon_full_round_chain_output_limb_23_col143,
            poseidon_full_round_chain_output_limb_24_col144,
            poseidon_full_round_chain_output_limb_25_col145,
            poseidon_full_round_chain_output_limb_26_col146,
            poseidon_full_round_chain_output_limb_27_col147,
            poseidon_full_round_chain_output_limb_28_col148,
            poseidon_full_round_chain_output_limb_29_col149,
            cube_252_output_limb_0_col150,
            cube_252_output_limb_1_col151,
            cube_252_output_limb_2_col152,
            cube_252_output_limb_3_col153,
            cube_252_output_limb_4_col154,
            cube_252_output_limb_5_col155,
            cube_252_output_limb_6_col156,
            cube_252_output_limb_7_col157,
            cube_252_output_limb_8_col158,
            cube_252_output_limb_9_col159,
            combination_limb_0_col160,
            combination_limb_1_col161,
            combination_limb_2_col162,
            combination_limb_3_col163,
            combination_limb_4_col164,
            combination_limb_5_col165,
            combination_limb_6_col166,
            combination_limb_7_col167,
            combination_limb_8_col168,
            combination_limb_9_col169,
            p_coef_col170,
            cube_252_output_limb_0_col171,
            cube_252_output_limb_1_col172,
            cube_252_output_limb_2_col173,
            cube_252_output_limb_3_col174,
            cube_252_output_limb_4_col175,
            cube_252_output_limb_5_col176,
            cube_252_output_limb_6_col177,
            cube_252_output_limb_7_col178,
            cube_252_output_limb_8_col179,
            cube_252_output_limb_9_col180,
            combination_limb_0_col181,
            combination_limb_1_col182,
            combination_limb_2_col183,
            combination_limb_3_col184,
            combination_limb_4_col185,
            combination_limb_5_col186,
            combination_limb_6_col187,
            combination_limb_7_col188,
            combination_limb_8_col189,
            combination_limb_9_col190,
            p_coef_col191,
            poseidon_3_partial_rounds_chain_output_limb_0_col192,
            poseidon_3_partial_rounds_chain_output_limb_1_col193,
            poseidon_3_partial_rounds_chain_output_limb_2_col194,
            poseidon_3_partial_rounds_chain_output_limb_3_col195,
            poseidon_3_partial_rounds_chain_output_limb_4_col196,
            poseidon_3_partial_rounds_chain_output_limb_5_col197,
            poseidon_3_partial_rounds_chain_output_limb_6_col198,
            poseidon_3_partial_rounds_chain_output_limb_7_col199,
            poseidon_3_partial_rounds_chain_output_limb_8_col200,
            poseidon_3_partial_rounds_chain_output_limb_9_col201,
            poseidon_3_partial_rounds_chain_output_limb_10_col202,
            poseidon_3_partial_rounds_chain_output_limb_11_col203,
            poseidon_3_partial_rounds_chain_output_limb_12_col204,
            poseidon_3_partial_rounds_chain_output_limb_13_col205,
            poseidon_3_partial_rounds_chain_output_limb_14_col206,
            poseidon_3_partial_rounds_chain_output_limb_15_col207,
            poseidon_3_partial_rounds_chain_output_limb_16_col208,
            poseidon_3_partial_rounds_chain_output_limb_17_col209,
            poseidon_3_partial_rounds_chain_output_limb_18_col210,
            poseidon_3_partial_rounds_chain_output_limb_19_col211,
            poseidon_3_partial_rounds_chain_output_limb_20_col212,
            poseidon_3_partial_rounds_chain_output_limb_21_col213,
            poseidon_3_partial_rounds_chain_output_limb_22_col214,
            poseidon_3_partial_rounds_chain_output_limb_23_col215,
            poseidon_3_partial_rounds_chain_output_limb_24_col216,
            poseidon_3_partial_rounds_chain_output_limb_25_col217,
            poseidon_3_partial_rounds_chain_output_limb_26_col218,
            poseidon_3_partial_rounds_chain_output_limb_27_col219,
            poseidon_3_partial_rounds_chain_output_limb_28_col220,
            poseidon_3_partial_rounds_chain_output_limb_29_col221,
            poseidon_3_partial_rounds_chain_output_limb_30_col222,
            poseidon_3_partial_rounds_chain_output_limb_31_col223,
            poseidon_3_partial_rounds_chain_output_limb_32_col224,
            poseidon_3_partial_rounds_chain_output_limb_33_col225,
            poseidon_3_partial_rounds_chain_output_limb_34_col226,
            poseidon_3_partial_rounds_chain_output_limb_35_col227,
            poseidon_3_partial_rounds_chain_output_limb_36_col228,
            poseidon_3_partial_rounds_chain_output_limb_37_col229,
            poseidon_3_partial_rounds_chain_output_limb_38_col230,
            poseidon_3_partial_rounds_chain_output_limb_39_col231,
            combination_limb_0_col232,
            combination_limb_1_col233,
            combination_limb_2_col234,
            combination_limb_3_col235,
            combination_limb_4_col236,
            combination_limb_5_col237,
            combination_limb_6_col238,
            combination_limb_7_col239,
            combination_limb_8_col240,
            combination_limb_9_col241,
            p_coef_col242,
            combination_limb_0_col243,
            combination_limb_1_col244,
            combination_limb_2_col245,
            combination_limb_3_col246,
            combination_limb_4_col247,
            combination_limb_5_col248,
            combination_limb_6_col249,
            combination_limb_7_col250,
            combination_limb_8_col251,
            combination_limb_9_col252,
            p_coef_col253,
            poseidon_full_round_chain_output_limb_0_col254,
            poseidon_full_round_chain_output_limb_1_col255,
            poseidon_full_round_chain_output_limb_2_col256,
            poseidon_full_round_chain_output_limb_3_col257,
            poseidon_full_round_chain_output_limb_4_col258,
            poseidon_full_round_chain_output_limb_5_col259,
            poseidon_full_round_chain_output_limb_6_col260,
            poseidon_full_round_chain_output_limb_7_col261,
            poseidon_full_round_chain_output_limb_8_col262,
            poseidon_full_round_chain_output_limb_9_col263,
            poseidon_full_round_chain_output_limb_10_col264,
            poseidon_full_round_chain_output_limb_11_col265,
            poseidon_full_round_chain_output_limb_12_col266,
            poseidon_full_round_chain_output_limb_13_col267,
            poseidon_full_round_chain_output_limb_14_col268,
            poseidon_full_round_chain_output_limb_15_col269,
            poseidon_full_round_chain_output_limb_16_col270,
            poseidon_full_round_chain_output_limb_17_col271,
            poseidon_full_round_chain_output_limb_18_col272,
            poseidon_full_round_chain_output_limb_19_col273,
            poseidon_full_round_chain_output_limb_20_col274,
            poseidon_full_round_chain_output_limb_21_col275,
            poseidon_full_round_chain_output_limb_22_col276,
            poseidon_full_round_chain_output_limb_23_col277,
            poseidon_full_round_chain_output_limb_24_col278,
            poseidon_full_round_chain_output_limb_25_col279,
            poseidon_full_round_chain_output_limb_26_col280,
            poseidon_full_round_chain_output_limb_27_col281,
            poseidon_full_round_chain_output_limb_28_col282,
            poseidon_full_round_chain_output_limb_29_col283,
            self.poseidon_full_round_chain_lookup_elements,
            self.range_check_felt_252_width_27_lookup_elements,
            self.cube_252_lookup_elements,
            self.range_check_3_3_3_3_3_lookup_elements,
            self.range_check_4_4_4_4_lookup_elements,
            self.range_check_4_4_lookup_elements,
            self.poseidon_3_partial_rounds_chain_lookup_elements,
            seq,
            ref poseidon_full_round_chain_sum_6,
            ref poseidon_full_round_chain_sum_7,
            ref range_check_felt_252_width_27_sum_8,
            ref range_check_felt_252_width_27_sum_9,
            ref cube_252_sum_10,
            ref range_check_3_3_3_3_3_sum_11,
            ref range_check_3_3_3_3_3_sum_12,
            ref cube_252_sum_13,
            ref range_check_4_4_4_4_sum_14,
            ref range_check_4_4_4_4_sum_15,
            ref range_check_4_4_sum_16,
            ref poseidon_3_partial_rounds_chain_sum_17,
            ref poseidon_3_partial_rounds_chain_sum_18,
            ref range_check_4_4_4_4_sum_19,
            ref range_check_4_4_4_4_sum_20,
            ref range_check_4_4_sum_21,
            ref range_check_4_4_4_4_sum_22,
            ref range_check_4_4_4_4_sum_23,
            ref range_check_4_4_sum_24,
            ref poseidon_full_round_chain_sum_25,
            ref poseidon_full_round_chain_sum_26,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        let output: [QM31; 10] = felt_252_unpack_from_27_evaluate(
            [
                poseidon_full_round_chain_output_limb_0_col254,
                poseidon_full_round_chain_output_limb_1_col255,
                poseidon_full_round_chain_output_limb_2_col256,
                poseidon_full_round_chain_output_limb_3_col257,
                poseidon_full_round_chain_output_limb_4_col258,
                poseidon_full_round_chain_output_limb_5_col259,
                poseidon_full_round_chain_output_limb_6_col260,
                poseidon_full_round_chain_output_limb_7_col261,
                poseidon_full_round_chain_output_limb_8_col262,
                poseidon_full_round_chain_output_limb_9_col263,
            ],
            unpacked_limb_0_col284,
            unpacked_limb_1_col285,
            unpacked_limb_3_col286,
            unpacked_limb_4_col287,
            unpacked_limb_6_col288,
            unpacked_limb_7_col289,
            unpacked_limb_9_col290,
            unpacked_limb_10_col291,
            unpacked_limb_12_col292,
            unpacked_limb_13_col293,
            unpacked_limb_15_col294,
            unpacked_limb_16_col295,
            unpacked_limb_18_col296,
            unpacked_limb_19_col297,
            unpacked_limb_21_col298,
            unpacked_limb_22_col299,
            unpacked_limb_24_col300,
            unpacked_limb_25_col301,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let [
            felt_252_unpack_from_27_output_tmp_51986_168_limb_2,
            felt_252_unpack_from_27_output_tmp_51986_168_limb_5,
            felt_252_unpack_from_27_output_tmp_51986_168_limb_8,
            felt_252_unpack_from_27_output_tmp_51986_168_limb_11,
            felt_252_unpack_from_27_output_tmp_51986_168_limb_14,
            felt_252_unpack_from_27_output_tmp_51986_168_limb_17,
            felt_252_unpack_from_27_output_tmp_51986_168_limb_20,
            felt_252_unpack_from_27_output_tmp_51986_168_limb_23,
            felt_252_unpack_from_27_output_tmp_51986_168_limb_26,
            felt_252_unpack_from_27_output_tmp_51986_168_limb_27,
        ] =
            output;

        mem_verify_evaluate(
            [
                ((poseidon_builtin_segment_start + (seq * qm31_const::<6, 0, 0, 0>()))
                    + qm31_const::<3, 0, 0, 0>()),
                unpacked_limb_0_col284, unpacked_limb_1_col285,
                felt_252_unpack_from_27_output_tmp_51986_168_limb_2, unpacked_limb_3_col286,
                unpacked_limb_4_col287, felt_252_unpack_from_27_output_tmp_51986_168_limb_5,
                unpacked_limb_6_col288, unpacked_limb_7_col289,
                felt_252_unpack_from_27_output_tmp_51986_168_limb_8, unpacked_limb_9_col290,
                unpacked_limb_10_col291, felt_252_unpack_from_27_output_tmp_51986_168_limb_11,
                unpacked_limb_12_col292, unpacked_limb_13_col293,
                felt_252_unpack_from_27_output_tmp_51986_168_limb_14, unpacked_limb_15_col294,
                unpacked_limb_16_col295, felt_252_unpack_from_27_output_tmp_51986_168_limb_17,
                unpacked_limb_18_col296, unpacked_limb_19_col297,
                felt_252_unpack_from_27_output_tmp_51986_168_limb_20, unpacked_limb_21_col298,
                unpacked_limb_22_col299, felt_252_unpack_from_27_output_tmp_51986_168_limb_23,
                unpacked_limb_24_col300, unpacked_limb_25_col301,
                felt_252_unpack_from_27_output_tmp_51986_168_limb_26,
                poseidon_full_round_chain_output_limb_9_col263,
            ],
            output_state_0_id_col302,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref memory_address_to_id_sum_27,
            ref memory_id_to_big_sum_28,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        let output: [QM31; 10] = felt_252_unpack_from_27_evaluate(
            [
                poseidon_full_round_chain_output_limb_10_col264,
                poseidon_full_round_chain_output_limb_11_col265,
                poseidon_full_round_chain_output_limb_12_col266,
                poseidon_full_round_chain_output_limb_13_col267,
                poseidon_full_round_chain_output_limb_14_col268,
                poseidon_full_round_chain_output_limb_15_col269,
                poseidon_full_round_chain_output_limb_16_col270,
                poseidon_full_round_chain_output_limb_17_col271,
                poseidon_full_round_chain_output_limb_18_col272,
                poseidon_full_round_chain_output_limb_19_col273,
            ],
            unpacked_limb_0_col303,
            unpacked_limb_1_col304,
            unpacked_limb_3_col305,
            unpacked_limb_4_col306,
            unpacked_limb_6_col307,
            unpacked_limb_7_col308,
            unpacked_limb_9_col309,
            unpacked_limb_10_col310,
            unpacked_limb_12_col311,
            unpacked_limb_13_col312,
            unpacked_limb_15_col313,
            unpacked_limb_16_col314,
            unpacked_limb_18_col315,
            unpacked_limb_19_col316,
            unpacked_limb_21_col317,
            unpacked_limb_22_col318,
            unpacked_limb_24_col319,
            unpacked_limb_25_col320,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let [
            felt_252_unpack_from_27_output_tmp_51986_171_limb_2,
            felt_252_unpack_from_27_output_tmp_51986_171_limb_5,
            felt_252_unpack_from_27_output_tmp_51986_171_limb_8,
            felt_252_unpack_from_27_output_tmp_51986_171_limb_11,
            felt_252_unpack_from_27_output_tmp_51986_171_limb_14,
            felt_252_unpack_from_27_output_tmp_51986_171_limb_17,
            felt_252_unpack_from_27_output_tmp_51986_171_limb_20,
            felt_252_unpack_from_27_output_tmp_51986_171_limb_23,
            felt_252_unpack_from_27_output_tmp_51986_171_limb_26,
            felt_252_unpack_from_27_output_tmp_51986_171_limb_27,
        ] =
            output;

        mem_verify_evaluate(
            [
                ((poseidon_builtin_segment_start + (seq * qm31_const::<6, 0, 0, 0>()))
                    + qm31_const::<4, 0, 0, 0>()),
                unpacked_limb_0_col303, unpacked_limb_1_col304,
                felt_252_unpack_from_27_output_tmp_51986_171_limb_2, unpacked_limb_3_col305,
                unpacked_limb_4_col306, felt_252_unpack_from_27_output_tmp_51986_171_limb_5,
                unpacked_limb_6_col307, unpacked_limb_7_col308,
                felt_252_unpack_from_27_output_tmp_51986_171_limb_8, unpacked_limb_9_col309,
                unpacked_limb_10_col310, felt_252_unpack_from_27_output_tmp_51986_171_limb_11,
                unpacked_limb_12_col311, unpacked_limb_13_col312,
                felt_252_unpack_from_27_output_tmp_51986_171_limb_14, unpacked_limb_15_col313,
                unpacked_limb_16_col314, felt_252_unpack_from_27_output_tmp_51986_171_limb_17,
                unpacked_limb_18_col315, unpacked_limb_19_col316,
                felt_252_unpack_from_27_output_tmp_51986_171_limb_20, unpacked_limb_21_col317,
                unpacked_limb_22_col318, felt_252_unpack_from_27_output_tmp_51986_171_limb_23,
                unpacked_limb_24_col319, unpacked_limb_25_col320,
                felt_252_unpack_from_27_output_tmp_51986_171_limb_26,
                poseidon_full_round_chain_output_limb_19_col273,
            ],
            output_state_1_id_col321,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref memory_address_to_id_sum_29,
            ref memory_id_to_big_sum_30,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

        let output: [QM31; 10] = felt_252_unpack_from_27_evaluate(
            [
                poseidon_full_round_chain_output_limb_20_col274,
                poseidon_full_round_chain_output_limb_21_col275,
                poseidon_full_round_chain_output_limb_22_col276,
                poseidon_full_round_chain_output_limb_23_col277,
                poseidon_full_round_chain_output_limb_24_col278,
                poseidon_full_round_chain_output_limb_25_col279,
                poseidon_full_round_chain_output_limb_26_col280,
                poseidon_full_round_chain_output_limb_27_col281,
                poseidon_full_round_chain_output_limb_28_col282,
                poseidon_full_round_chain_output_limb_29_col283,
            ],
            unpacked_limb_0_col322,
            unpacked_limb_1_col323,
            unpacked_limb_3_col324,
            unpacked_limb_4_col325,
            unpacked_limb_6_col326,
            unpacked_limb_7_col327,
            unpacked_limb_9_col328,
            unpacked_limb_10_col329,
            unpacked_limb_12_col330,
            unpacked_limb_13_col331,
            unpacked_limb_15_col332,
            unpacked_limb_16_col333,
            unpacked_limb_18_col334,
            unpacked_limb_19_col335,
            unpacked_limb_21_col336,
            unpacked_limb_22_col337,
            unpacked_limb_24_col338,
            unpacked_limb_25_col339,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );
        let [
            felt_252_unpack_from_27_output_tmp_51986_174_limb_2,
            felt_252_unpack_from_27_output_tmp_51986_174_limb_5,
            felt_252_unpack_from_27_output_tmp_51986_174_limb_8,
            felt_252_unpack_from_27_output_tmp_51986_174_limb_11,
            felt_252_unpack_from_27_output_tmp_51986_174_limb_14,
            felt_252_unpack_from_27_output_tmp_51986_174_limb_17,
            felt_252_unpack_from_27_output_tmp_51986_174_limb_20,
            felt_252_unpack_from_27_output_tmp_51986_174_limb_23,
            felt_252_unpack_from_27_output_tmp_51986_174_limb_26,
            felt_252_unpack_from_27_output_tmp_51986_174_limb_27,
        ] =
            output;

        mem_verify_evaluate(
            [
                ((poseidon_builtin_segment_start + (seq * qm31_const::<6, 0, 0, 0>()))
                    + qm31_const::<5, 0, 0, 0>()),
                unpacked_limb_0_col322, unpacked_limb_1_col323,
                felt_252_unpack_from_27_output_tmp_51986_174_limb_2, unpacked_limb_3_col324,
                unpacked_limb_4_col325, felt_252_unpack_from_27_output_tmp_51986_174_limb_5,
                unpacked_limb_6_col326, unpacked_limb_7_col327,
                felt_252_unpack_from_27_output_tmp_51986_174_limb_8, unpacked_limb_9_col328,
                unpacked_limb_10_col329, felt_252_unpack_from_27_output_tmp_51986_174_limb_11,
                unpacked_limb_12_col330, unpacked_limb_13_col331,
                felt_252_unpack_from_27_output_tmp_51986_174_limb_14, unpacked_limb_15_col332,
                unpacked_limb_16_col333, felt_252_unpack_from_27_output_tmp_51986_174_limb_17,
                unpacked_limb_18_col334, unpacked_limb_19_col335,
                felt_252_unpack_from_27_output_tmp_51986_174_limb_20, unpacked_limb_21_col336,
                unpacked_limb_22_col337, felt_252_unpack_from_27_output_tmp_51986_174_limb_23,
                unpacked_limb_24_col338, unpacked_limb_25_col339,
                felt_252_unpack_from_27_output_tmp_51986_174_limb_26,
                poseidon_full_round_chain_output_limb_29_col283,
            ],
            output_state_2_id_col340,
            self.memory_address_to_id_lookup_elements,
            self.memory_id_to_big_lookup_elements,
            ref memory_address_to_id_sum_31,
            ref memory_id_to_big_sum_32,
            ref sum,
            domain_vanishing_eval_inv,
            random_coeff,
        );

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
            poseidon_full_round_chain_sum_6,
            poseidon_full_round_chain_sum_7,
            range_check_felt_252_width_27_sum_8,
            range_check_felt_252_width_27_sum_9,
            cube_252_sum_10,
            range_check_3_3_3_3_3_sum_11,
            range_check_3_3_3_3_3_sum_12,
            cube_252_sum_13,
            range_check_4_4_4_4_sum_14,
            range_check_4_4_4_4_sum_15,
            range_check_4_4_sum_16,
            poseidon_3_partial_rounds_chain_sum_17,
            poseidon_3_partial_rounds_chain_sum_18,
            range_check_4_4_4_4_sum_19,
            range_check_4_4_4_4_sum_20,
            range_check_4_4_sum_21,
            range_check_4_4_4_4_sum_22,
            range_check_4_4_4_4_sum_23,
            range_check_4_4_sum_24,
            poseidon_full_round_chain_sum_25,
            poseidon_full_round_chain_sum_26,
            memory_address_to_id_sum_27,
            memory_id_to_big_sum_28,
            memory_address_to_id_sum_29,
            memory_id_to_big_sum_30,
            memory_address_to_id_sum_31,
            memory_id_to_big_sum_32,
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
    poseidon_full_round_chain_sum_6: QM31,
    poseidon_full_round_chain_sum_7: QM31,
    range_check_felt_252_width_27_sum_8: QM31,
    range_check_felt_252_width_27_sum_9: QM31,
    cube_252_sum_10: QM31,
    range_check_3_3_3_3_3_sum_11: QM31,
    range_check_3_3_3_3_3_sum_12: QM31,
    cube_252_sum_13: QM31,
    range_check_4_4_4_4_sum_14: QM31,
    range_check_4_4_4_4_sum_15: QM31,
    range_check_4_4_sum_16: QM31,
    poseidon_3_partial_rounds_chain_sum_17: QM31,
    poseidon_3_partial_rounds_chain_sum_18: QM31,
    range_check_4_4_4_4_sum_19: QM31,
    range_check_4_4_4_4_sum_20: QM31,
    range_check_4_4_sum_21: QM31,
    range_check_4_4_4_4_sum_22: QM31,
    range_check_4_4_4_4_sum_23: QM31,
    range_check_4_4_sum_24: QM31,
    poseidon_full_round_chain_sum_25: QM31,
    poseidon_full_round_chain_sum_26: QM31,
    memory_address_to_id_sum_27: QM31,
    memory_id_to_big_sum_28: QM31,
    memory_address_to_id_sum_29: QM31,
    memory_id_to_big_sum_30: QM31,
    memory_address_to_id_sum_31: QM31,
    memory_id_to_big_sum_32: QM31,
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
    ]: [Span<QM31>; 68] =
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
    let [trace_2_col64_neg1, trace_2_col64]: [QM31; 2] = (*trace_2_col64.try_into().unwrap())
        .unbox();
    let [trace_2_col65_neg1, trace_2_col65]: [QM31; 2] = (*trace_2_col65.try_into().unwrap())
        .unbox();
    let [trace_2_col66_neg1, trace_2_col66]: [QM31; 2] = (*trace_2_col66.try_into().unwrap())
        .unbox();
    let [trace_2_col67_neg1, trace_2_col67]: [QM31; 2] = (*trace_2_col67.try_into().unwrap())
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
        * poseidon_full_round_chain_sum_6
        * poseidon_full_round_chain_sum_7)
        - poseidon_full_round_chain_sum_6
        + poseidon_full_round_chain_sum_7)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col12, trace_2_col13, trace_2_col14, trace_2_col15],
        ))
        * range_check_felt_252_width_27_sum_8
        * range_check_felt_252_width_27_sum_9)
        - range_check_felt_252_width_27_sum_8
        - range_check_felt_252_width_27_sum_9)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col16, trace_2_col17, trace_2_col18, trace_2_col19],
        ))
        * cube_252_sum_10
        * range_check_3_3_3_3_3_sum_11)
        - cube_252_sum_10
        - range_check_3_3_3_3_3_sum_11)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col20, trace_2_col21, trace_2_col22, trace_2_col23],
        ))
        * range_check_3_3_3_3_3_sum_12
        * cube_252_sum_13)
        - range_check_3_3_3_3_3_sum_12
        - cube_252_sum_13)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col28, trace_2_col29, trace_2_col30, trace_2_col31],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col24, trace_2_col25, trace_2_col26, trace_2_col27],
        ))
        * range_check_4_4_4_4_sum_14
        * range_check_4_4_4_4_sum_15)
        - range_check_4_4_4_4_sum_14
        - range_check_4_4_4_4_sum_15)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col32, trace_2_col33, trace_2_col34, trace_2_col35],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col28, trace_2_col29, trace_2_col30, trace_2_col31],
        ))
        * range_check_4_4_sum_16
        * poseidon_3_partial_rounds_chain_sum_17)
        + range_check_4_4_sum_16
        - poseidon_3_partial_rounds_chain_sum_17)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col36, trace_2_col37, trace_2_col38, trace_2_col39],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col32, trace_2_col33, trace_2_col34, trace_2_col35],
        ))
        * poseidon_3_partial_rounds_chain_sum_18
        * range_check_4_4_4_4_sum_19)
        - poseidon_3_partial_rounds_chain_sum_18
        - range_check_4_4_4_4_sum_19)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col40, trace_2_col41, trace_2_col42, trace_2_col43],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col36, trace_2_col37, trace_2_col38, trace_2_col39],
        ))
        * range_check_4_4_4_4_sum_20
        * range_check_4_4_sum_21)
        - range_check_4_4_4_4_sum_20
        - range_check_4_4_sum_21)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col44, trace_2_col45, trace_2_col46, trace_2_col47],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col40, trace_2_col41, trace_2_col42, trace_2_col43],
        ))
        * range_check_4_4_4_4_sum_22
        * range_check_4_4_4_4_sum_23)
        - range_check_4_4_4_4_sum_22
        - range_check_4_4_4_4_sum_23)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col48, trace_2_col49, trace_2_col50, trace_2_col51],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col44, trace_2_col45, trace_2_col46, trace_2_col47],
        ))
        * range_check_4_4_sum_24
        * poseidon_full_round_chain_sum_25)
        + range_check_4_4_sum_24
        - poseidon_full_round_chain_sum_25)
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;

    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col52, trace_2_col53, trace_2_col54, trace_2_col55],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col48, trace_2_col49, trace_2_col50, trace_2_col51],
        ))
        * poseidon_full_round_chain_sum_26
        * memory_address_to_id_sum_27)
        - poseidon_full_round_chain_sum_26
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
        - QM31Impl::from_partial_evals([trace_2_col60, trace_2_col61, trace_2_col62, trace_2_col63])
        - QM31Impl::from_partial_evals(
            [trace_2_col64_neg1, trace_2_col65_neg1, trace_2_col66_neg1, trace_2_col67_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * memory_id_to_big_sum_32)
        - qm31_const::<1, 0, 0, 0>())
        * domain_vanishing_eval_inv;
    sum = sum * random_coeff + constraint_quotient;
}
