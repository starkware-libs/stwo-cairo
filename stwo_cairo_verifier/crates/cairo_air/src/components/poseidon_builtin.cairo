use core::num::traits::Zero;
use stwo_constraint_framework::{
    PreprocessedColumn, PreprocessedColumnSet, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::Invertible;
use stwo_verifier_core::fields::m31::m31;
use stwo_verifier_core::fields::qm31::{QM31, QM31Serde, QM31_EXTENSION_DEGREE};
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::{ArrayImpl, pow2};
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray};
use crate::components::CairoComponent;
use crate::utils::U32Impl;

mod constraints;

#[derive(Drop, Serde, Copy)]
pub struct Claim {
    pub log_size: u32,
    pub poseidon_builtin_segment_start: u32,
}

#[generate_trait]
pub impl ClaimImpl of ClaimTrait {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = *self.log_size;
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = ArrayImpl::new_repeated(347, log_size).span();
        let interaction_log_sizes = ArrayImpl::new_repeated(QM31_EXTENSION_DEGREE * 17, log_size)
            .span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        channel.mix_u64((*self.log_size).into());
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
    pub cube_252_lookup_elements: crate::Cube252Elements,
    pub poseidon_3_partial_rounds_chain_lookup_elements: crate::Poseidon3PartialRoundsChainElements,
    pub range_check_3_3_3_3_3_lookup_elements: crate::RangeCheck3Bit3Bit3Bit3Bit3BitElements,
    pub range_check_4_4_4_4_lookup_elements: crate::RangeCheck4Bit4Bit4Bit4BitElements,
    pub range_check_4_4_lookup_elements: crate::RangeCheck4Bit4BitElements,
    pub poseidon_full_round_chain_lookup_elements: crate::PoseidonFullRoundChainElements,
    pub range_check_felt_252_width_27_lookup_elements: crate::RangeCheckFelt252Width27Elements,
}

pub impl CairoComponentImpl of CairoComponent<Component> {
    fn mask_points(
        self: @Component,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let log_size = *self.claim.log_size;
        let trace_gen = CanonicCosetImpl::new(log_size).coset.step_size;
        constraints::mask_points(
            ref preprocessed_column_set,
            ref trace_mask_points,
            ref interaction_trace_mask_points,
            point,
            trace_gen,
            log_size,
        );
    }

    fn max_constraint_log_degree_bound(self: @Component) -> u32 {
        *self.claim.log_size + 1
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
        let MemoryAddressToId_z = *self.memory_address_to_id_lookup_elements.z;
        let mut memory_address_to_id_alpha_powers = self
            .memory_address_to_id_lookup_elements
            .alpha_powers
            .span();
        let MemoryAddressToId_alpha0 = *memory_address_to_id_alpha_powers.pop_front().unwrap();
        let MemoryAddressToId_alpha1 = *memory_address_to_id_alpha_powers.pop_front().unwrap();

        let MemoryIdToBig_z = *self.memory_id_to_big_lookup_elements.z;
        let mut memory_id_to_big_alpha_powers = self
            .memory_id_to_big_lookup_elements
            .alpha_powers
            .span();
        let MemoryIdToBig_alpha0 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha1 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha2 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha3 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha4 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha5 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha6 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha7 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha8 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha9 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha10 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha11 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha12 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha13 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha14 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha15 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha16 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha17 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha18 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha19 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha20 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha21 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha22 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha23 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha24 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha25 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha26 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha27 = *memory_id_to_big_alpha_powers.pop_front().unwrap();
        let MemoryIdToBig_alpha28 = *memory_id_to_big_alpha_powers.pop_front().unwrap();

        let Cube252_z = *self.cube_252_lookup_elements.z;
        let mut cube_252_alpha_powers = self.cube_252_lookup_elements.alpha_powers.span();
        let Cube252_alpha0 = *cube_252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha1 = *cube_252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha2 = *cube_252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha3 = *cube_252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha4 = *cube_252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha5 = *cube_252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha6 = *cube_252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha7 = *cube_252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha8 = *cube_252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha9 = *cube_252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha10 = *cube_252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha11 = *cube_252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha12 = *cube_252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha13 = *cube_252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha14 = *cube_252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha15 = *cube_252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha16 = *cube_252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha17 = *cube_252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha18 = *cube_252_alpha_powers.pop_front().unwrap();
        let Cube252_alpha19 = *cube_252_alpha_powers.pop_front().unwrap();

        let Poseidon3PartialRoundsChain_z = *self.poseidon_3_partial_rounds_chain_lookup_elements.z;
        let mut poseidon_3_partial_rounds_chain_alpha_powers = self
            .poseidon_3_partial_rounds_chain_lookup_elements
            .alpha_powers
            .span();
        let Poseidon3PartialRoundsChain_alpha0 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha1 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha2 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha3 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha4 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha5 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha6 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha7 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha8 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha9 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha10 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha11 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha12 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha13 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha14 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha15 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha16 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha17 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha18 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha19 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha20 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha21 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha22 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha23 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha24 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha25 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha26 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha27 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha28 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha29 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha30 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha31 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha32 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha33 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha34 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha35 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha36 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha37 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha38 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha39 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha40 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();
        let Poseidon3PartialRoundsChain_alpha41 = *poseidon_3_partial_rounds_chain_alpha_powers
            .pop_front()
            .unwrap();

        let RangeCheck_3_3_3_3_3_z = *self.range_check_3_3_3_3_3_lookup_elements.z;
        let mut range_check_3_3_3_3_3_alpha_powers = self
            .range_check_3_3_3_3_3_lookup_elements
            .alpha_powers
            .span();
        let RangeCheck_3_3_3_3_3_alpha0 = *range_check_3_3_3_3_3_alpha_powers.pop_front().unwrap();
        let RangeCheck_3_3_3_3_3_alpha1 = *range_check_3_3_3_3_3_alpha_powers.pop_front().unwrap();
        let RangeCheck_3_3_3_3_3_alpha2 = *range_check_3_3_3_3_3_alpha_powers.pop_front().unwrap();
        let RangeCheck_3_3_3_3_3_alpha3 = *range_check_3_3_3_3_3_alpha_powers.pop_front().unwrap();
        let RangeCheck_3_3_3_3_3_alpha4 = *range_check_3_3_3_3_3_alpha_powers.pop_front().unwrap();

        let RangeCheck_4_4_4_4_z = *self.range_check_4_4_4_4_lookup_elements.z;
        let mut range_check_4_4_4_4_alpha_powers = self
            .range_check_4_4_4_4_lookup_elements
            .alpha_powers
            .span();
        let RangeCheck_4_4_4_4_alpha0 = *range_check_4_4_4_4_alpha_powers.pop_front().unwrap();
        let RangeCheck_4_4_4_4_alpha1 = *range_check_4_4_4_4_alpha_powers.pop_front().unwrap();
        let RangeCheck_4_4_4_4_alpha2 = *range_check_4_4_4_4_alpha_powers.pop_front().unwrap();
        let RangeCheck_4_4_4_4_alpha3 = *range_check_4_4_4_4_alpha_powers.pop_front().unwrap();

        let RangeCheck_4_4_z = *self.range_check_4_4_lookup_elements.z;
        let mut range_check_4_4_alpha_powers = self
            .range_check_4_4_lookup_elements
            .alpha_powers
            .span();
        let RangeCheck_4_4_alpha0 = *range_check_4_4_alpha_powers.pop_front().unwrap();
        let RangeCheck_4_4_alpha1 = *range_check_4_4_alpha_powers.pop_front().unwrap();

        let PoseidonFullRoundChain_z = *self.poseidon_full_round_chain_lookup_elements.z;
        let mut poseidon_full_round_chain_alpha_powers = self
            .poseidon_full_round_chain_lookup_elements
            .alpha_powers
            .span();
        let PoseidonFullRoundChain_alpha0 = *poseidon_full_round_chain_alpha_powers
            .pop_front()
            .unwrap();
        let PoseidonFullRoundChain_alpha1 = *poseidon_full_round_chain_alpha_powers
            .pop_front()
            .unwrap();
        let PoseidonFullRoundChain_alpha2 = *poseidon_full_round_chain_alpha_powers
            .pop_front()
            .unwrap();
        let PoseidonFullRoundChain_alpha3 = *poseidon_full_round_chain_alpha_powers
            .pop_front()
            .unwrap();
        let PoseidonFullRoundChain_alpha4 = *poseidon_full_round_chain_alpha_powers
            .pop_front()
            .unwrap();
        let PoseidonFullRoundChain_alpha5 = *poseidon_full_round_chain_alpha_powers
            .pop_front()
            .unwrap();
        let PoseidonFullRoundChain_alpha6 = *poseidon_full_round_chain_alpha_powers
            .pop_front()
            .unwrap();
        let PoseidonFullRoundChain_alpha7 = *poseidon_full_round_chain_alpha_powers
            .pop_front()
            .unwrap();
        let PoseidonFullRoundChain_alpha8 = *poseidon_full_round_chain_alpha_powers
            .pop_front()
            .unwrap();
        let PoseidonFullRoundChain_alpha9 = *poseidon_full_round_chain_alpha_powers
            .pop_front()
            .unwrap();
        let PoseidonFullRoundChain_alpha10 = *poseidon_full_round_chain_alpha_powers
            .pop_front()
            .unwrap();
        let PoseidonFullRoundChain_alpha11 = *poseidon_full_round_chain_alpha_powers
            .pop_front()
            .unwrap();
        let PoseidonFullRoundChain_alpha12 = *poseidon_full_round_chain_alpha_powers
            .pop_front()
            .unwrap();
        let PoseidonFullRoundChain_alpha13 = *poseidon_full_round_chain_alpha_powers
            .pop_front()
            .unwrap();
        let PoseidonFullRoundChain_alpha14 = *poseidon_full_round_chain_alpha_powers
            .pop_front()
            .unwrap();
        let PoseidonFullRoundChain_alpha15 = *poseidon_full_round_chain_alpha_powers
            .pop_front()
            .unwrap();
        let PoseidonFullRoundChain_alpha16 = *poseidon_full_round_chain_alpha_powers
            .pop_front()
            .unwrap();
        let PoseidonFullRoundChain_alpha17 = *poseidon_full_round_chain_alpha_powers
            .pop_front()
            .unwrap();
        let PoseidonFullRoundChain_alpha18 = *poseidon_full_round_chain_alpha_powers
            .pop_front()
            .unwrap();
        let PoseidonFullRoundChain_alpha19 = *poseidon_full_round_chain_alpha_powers
            .pop_front()
            .unwrap();
        let PoseidonFullRoundChain_alpha20 = *poseidon_full_round_chain_alpha_powers
            .pop_front()
            .unwrap();
        let PoseidonFullRoundChain_alpha21 = *poseidon_full_round_chain_alpha_powers
            .pop_front()
            .unwrap();
        let PoseidonFullRoundChain_alpha22 = *poseidon_full_round_chain_alpha_powers
            .pop_front()
            .unwrap();
        let PoseidonFullRoundChain_alpha23 = *poseidon_full_round_chain_alpha_powers
            .pop_front()
            .unwrap();
        let PoseidonFullRoundChain_alpha24 = *poseidon_full_round_chain_alpha_powers
            .pop_front()
            .unwrap();
        let PoseidonFullRoundChain_alpha25 = *poseidon_full_round_chain_alpha_powers
            .pop_front()
            .unwrap();
        let PoseidonFullRoundChain_alpha26 = *poseidon_full_round_chain_alpha_powers
            .pop_front()
            .unwrap();
        let PoseidonFullRoundChain_alpha27 = *poseidon_full_round_chain_alpha_powers
            .pop_front()
            .unwrap();
        let PoseidonFullRoundChain_alpha28 = *poseidon_full_round_chain_alpha_powers
            .pop_front()
            .unwrap();
        let PoseidonFullRoundChain_alpha29 = *poseidon_full_round_chain_alpha_powers
            .pop_front()
            .unwrap();
        let PoseidonFullRoundChain_alpha30 = *poseidon_full_round_chain_alpha_powers
            .pop_front()
            .unwrap();
        let PoseidonFullRoundChain_alpha31 = *poseidon_full_round_chain_alpha_powers
            .pop_front()
            .unwrap();

        let RangeCheckFelt252Width27_z = *self.range_check_felt_252_width_27_lookup_elements.z;
        let mut range_check_felt_252_width_27_alpha_powers = self
            .range_check_felt_252_width_27_lookup_elements
            .alpha_powers
            .span();
        let RangeCheckFelt252Width27_alpha0 = *range_check_felt_252_width_27_alpha_powers
            .pop_front()
            .unwrap();
        let RangeCheckFelt252Width27_alpha1 = *range_check_felt_252_width_27_alpha_powers
            .pop_front()
            .unwrap();
        let RangeCheckFelt252Width27_alpha2 = *range_check_felt_252_width_27_alpha_powers
            .pop_front()
            .unwrap();
        let RangeCheckFelt252Width27_alpha3 = *range_check_felt_252_width_27_alpha_powers
            .pop_front()
            .unwrap();
        let RangeCheckFelt252Width27_alpha4 = *range_check_felt_252_width_27_alpha_powers
            .pop_front()
            .unwrap();
        let RangeCheckFelt252Width27_alpha5 = *range_check_felt_252_width_27_alpha_powers
            .pop_front()
            .unwrap();
        let RangeCheckFelt252Width27_alpha6 = *range_check_felt_252_width_27_alpha_powers
            .pop_front()
            .unwrap();
        let RangeCheckFelt252Width27_alpha7 = *range_check_felt_252_width_27_alpha_powers
            .pop_front()
            .unwrap();
        let RangeCheckFelt252Width27_alpha8 = *range_check_felt_252_width_27_alpha_powers
            .pop_front()
            .unwrap();
        let RangeCheckFelt252Width27_alpha9 = *range_check_felt_252_width_27_alpha_powers
            .pop_front()
            .unwrap();

        let log_size = *self.claim.log_size;

        let claimed_sum = *self.interaction_claim.claimed_sum;

        let poseidon_builtin_segment_start = (*self.claim.poseidon_builtin_segment_start)
            .try_into()
            .unwrap();

        let params = constraints::ConstraintParams {
            column_size: pow2(log_size).try_into().unwrap(),
            builtin_segment_start: poseidon_builtin_segment_start,
            RangeCheck_3_3_3_3_3_alpha0,
            RangeCheck_3_3_3_3_3_alpha1,
            RangeCheck_3_3_3_3_3_alpha2,
            RangeCheck_3_3_3_3_3_alpha3,
            RangeCheck_3_3_3_3_3_alpha4,
            RangeCheck_3_3_3_3_3_z,
            RangeCheck_4_4_4_4_alpha0,
            RangeCheck_4_4_4_4_alpha1,
            RangeCheck_4_4_4_4_alpha2,
            RangeCheck_4_4_4_4_alpha3,
            RangeCheck_4_4_4_4_z,
            RangeCheck_4_4_alpha0,
            RangeCheck_4_4_alpha1,
            RangeCheck_4_4_z,
            RangeCheckFelt252Width27_alpha0,
            RangeCheckFelt252Width27_alpha1,
            RangeCheckFelt252Width27_alpha2,
            RangeCheckFelt252Width27_alpha3,
            RangeCheckFelt252Width27_alpha4,
            RangeCheckFelt252Width27_alpha5,
            RangeCheckFelt252Width27_alpha6,
            RangeCheckFelt252Width27_alpha7,
            RangeCheckFelt252Width27_alpha8,
            RangeCheckFelt252Width27_alpha9,
            RangeCheckFelt252Width27_z,
            PoseidonFullRoundChain_alpha0,
            PoseidonFullRoundChain_alpha1,
            PoseidonFullRoundChain_alpha2,
            PoseidonFullRoundChain_alpha3,
            PoseidonFullRoundChain_alpha4,
            PoseidonFullRoundChain_alpha5,
            PoseidonFullRoundChain_alpha6,
            PoseidonFullRoundChain_alpha7,
            PoseidonFullRoundChain_alpha8,
            PoseidonFullRoundChain_alpha9,
            PoseidonFullRoundChain_alpha10,
            PoseidonFullRoundChain_alpha11,
            PoseidonFullRoundChain_alpha12,
            PoseidonFullRoundChain_alpha13,
            PoseidonFullRoundChain_alpha14,
            PoseidonFullRoundChain_alpha15,
            PoseidonFullRoundChain_alpha16,
            PoseidonFullRoundChain_alpha17,
            PoseidonFullRoundChain_alpha18,
            PoseidonFullRoundChain_alpha19,
            PoseidonFullRoundChain_alpha20,
            PoseidonFullRoundChain_alpha21,
            PoseidonFullRoundChain_alpha22,
            PoseidonFullRoundChain_alpha23,
            PoseidonFullRoundChain_alpha24,
            PoseidonFullRoundChain_alpha25,
            PoseidonFullRoundChain_alpha26,
            PoseidonFullRoundChain_alpha27,
            PoseidonFullRoundChain_alpha28,
            PoseidonFullRoundChain_alpha29,
            PoseidonFullRoundChain_alpha30,
            PoseidonFullRoundChain_alpha31,
            PoseidonFullRoundChain_z,
            Poseidon3PartialRoundsChain_alpha0,
            Poseidon3PartialRoundsChain_alpha1,
            Poseidon3PartialRoundsChain_alpha2,
            Poseidon3PartialRoundsChain_alpha3,
            Poseidon3PartialRoundsChain_alpha4,
            Poseidon3PartialRoundsChain_alpha5,
            Poseidon3PartialRoundsChain_alpha6,
            Poseidon3PartialRoundsChain_alpha7,
            Poseidon3PartialRoundsChain_alpha8,
            Poseidon3PartialRoundsChain_alpha9,
            Poseidon3PartialRoundsChain_alpha10,
            Poseidon3PartialRoundsChain_alpha11,
            Poseidon3PartialRoundsChain_alpha12,
            Poseidon3PartialRoundsChain_alpha13,
            Poseidon3PartialRoundsChain_alpha14,
            Poseidon3PartialRoundsChain_alpha15,
            Poseidon3PartialRoundsChain_alpha16,
            Poseidon3PartialRoundsChain_alpha17,
            Poseidon3PartialRoundsChain_alpha18,
            Poseidon3PartialRoundsChain_alpha19,
            Poseidon3PartialRoundsChain_alpha20,
            Poseidon3PartialRoundsChain_alpha21,
            Poseidon3PartialRoundsChain_alpha22,
            Poseidon3PartialRoundsChain_alpha23,
            Poseidon3PartialRoundsChain_alpha24,
            Poseidon3PartialRoundsChain_alpha25,
            Poseidon3PartialRoundsChain_alpha26,
            Poseidon3PartialRoundsChain_alpha27,
            Poseidon3PartialRoundsChain_alpha28,
            Poseidon3PartialRoundsChain_alpha29,
            Poseidon3PartialRoundsChain_alpha30,
            Poseidon3PartialRoundsChain_alpha31,
            Poseidon3PartialRoundsChain_alpha32,
            Poseidon3PartialRoundsChain_alpha33,
            Poseidon3PartialRoundsChain_alpha34,
            Poseidon3PartialRoundsChain_alpha35,
            Poseidon3PartialRoundsChain_alpha36,
            Poseidon3PartialRoundsChain_alpha37,
            Poseidon3PartialRoundsChain_alpha38,
            Poseidon3PartialRoundsChain_alpha39,
            Poseidon3PartialRoundsChain_alpha40,
            Poseidon3PartialRoundsChain_alpha41,
            Poseidon3PartialRoundsChain_z,
            Cube252_alpha0,
            Cube252_alpha1,
            Cube252_alpha2,
            Cube252_alpha3,
            Cube252_alpha4,
            Cube252_alpha5,
            Cube252_alpha6,
            Cube252_alpha7,
            Cube252_alpha8,
            Cube252_alpha9,
            Cube252_alpha10,
            Cube252_alpha11,
            Cube252_alpha12,
            Cube252_alpha13,
            Cube252_alpha14,
            Cube252_alpha15,
            Cube252_alpha16,
            Cube252_alpha17,
            Cube252_alpha18,
            Cube252_alpha19,
            Cube252_z,
            MemoryAddressToId_alpha0,
            MemoryAddressToId_alpha1,
            MemoryAddressToId_z,
            MemoryIdToBig_alpha0,
            MemoryIdToBig_alpha1,
            MemoryIdToBig_alpha10,
            MemoryIdToBig_alpha11,
            MemoryIdToBig_alpha12,
            MemoryIdToBig_alpha13,
            MemoryIdToBig_alpha14,
            MemoryIdToBig_alpha15,
            MemoryIdToBig_alpha16,
            MemoryIdToBig_alpha17,
            MemoryIdToBig_alpha18,
            MemoryIdToBig_alpha19,
            MemoryIdToBig_alpha20,
            MemoryIdToBig_alpha21,
            MemoryIdToBig_alpha22,
            MemoryIdToBig_alpha23,
            MemoryIdToBig_alpha24,
            MemoryIdToBig_alpha25,
            MemoryIdToBig_alpha26,
            MemoryIdToBig_alpha27,
            MemoryIdToBig_alpha28,
            MemoryIdToBig_alpha2,
            MemoryIdToBig_alpha3,
            MemoryIdToBig_alpha4,
            MemoryIdToBig_alpha5,
            MemoryIdToBig_alpha6,
            MemoryIdToBig_alpha7,
            MemoryIdToBig_alpha8,
            MemoryIdToBig_alpha9,
            MemoryIdToBig_z,
            claimed_sum,
            // TODO
            // poseidon_builtin_segment_start,
            seq: preprocessed_mask_values.get(PreprocessedColumn::Seq(log_size)),
        };

        let trace_domain = CanonicCosetImpl::new(log_size);
        let vanish_eval = trace_domain.eval_vanishing(point);

        constraints::evaluate_constraints_at_point(
            ref sum,
            ref trace_mask_values,
            ref interaction_trace_mask_values,
            params,
            random_coeff,
            vanish_eval.inverse(),
        );
    }
}

