use core::num::traits::Zero;
use stwo_constraint_framework::{
    PreprocessedColumn, PreprocessedColumnSet, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::Invertible;
use stwo_verifier_core::fields::m31::m31;
use stwo_verifier_core::fields::qm31::{QM31, QM31_EXTENSION_DEGREE};
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::{ArrayImpl, pow2};
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray};
use crate::components::CairoComponent;
use crate::utils::U32Impl;

mod constraints;

#[derive(Drop, Serde, Copy)]
pub struct Claim {
    pub log_size: u32,
}

#[generate_trait]
pub impl ClaimImpl of ClaimTrait {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = *self.log_size;
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = ArrayImpl::new_repeated(169, log_size).span();
        let interaction_log_sizes = ArrayImpl::new_repeated(QM31_EXTENSION_DEGREE * 9, log_size)
            .span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        channel.mix_u64((*self.log_size).into());
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
    pub cube_252_lookup_elements: crate::Cube252Elements,
    pub poseidon_3_partial_rounds_chain_lookup_elements: crate::Poseidon3PartialRoundsChainElements,
    pub range_check_4_4_4_4_lookup_elements: crate::RangeCheck4Bit4Bit4Bit4BitElements,
    pub poseidon_round_keys_lookup_elements: crate::PoseidonRoundKeysElements,
    pub range_check_4_4_lookup_elements: crate::RangeCheck4Bit4BitElements,
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

        let PoseidonRoundKeys_z = *self.poseidon_round_keys_lookup_elements.z;
        let mut poseidon_round_keys_alpha_powers = self
            .poseidon_round_keys_lookup_elements
            .alpha_powers
            .span();
        let PoseidonRoundKeys_alpha0 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha1 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha2 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha3 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha4 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha5 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha6 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha7 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha8 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha9 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha10 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha11 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha12 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha13 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha14 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha15 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha16 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha17 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha18 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha19 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha20 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha21 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha22 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha23 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha24 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha25 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha26 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha27 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha28 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha29 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();
        let PoseidonRoundKeys_alpha30 = *poseidon_round_keys_alpha_powers.pop_front().unwrap();

        let log_size = *self.claim.log_size;

        let claimed_sum = *self.interaction_claim.claimed_sum;

        let params = constraints::ConstraintParams {
            column_size: m31(pow2(log_size)),
            PoseidonRoundKeys_alpha0,
            PoseidonRoundKeys_alpha1,
            PoseidonRoundKeys_alpha2,
            PoseidonRoundKeys_alpha3,
            PoseidonRoundKeys_alpha4,
            PoseidonRoundKeys_alpha5,
            PoseidonRoundKeys_alpha6,
            PoseidonRoundKeys_alpha7,
            PoseidonRoundKeys_alpha8,
            PoseidonRoundKeys_alpha9,
            PoseidonRoundKeys_alpha10,
            PoseidonRoundKeys_alpha11,
            PoseidonRoundKeys_alpha12,
            PoseidonRoundKeys_alpha13,
            PoseidonRoundKeys_alpha14,
            PoseidonRoundKeys_alpha15,
            PoseidonRoundKeys_alpha16,
            PoseidonRoundKeys_alpha17,
            PoseidonRoundKeys_alpha18,
            PoseidonRoundKeys_alpha19,
            PoseidonRoundKeys_alpha20,
            PoseidonRoundKeys_alpha21,
            PoseidonRoundKeys_alpha22,
            PoseidonRoundKeys_alpha23,
            PoseidonRoundKeys_alpha24,
            PoseidonRoundKeys_alpha25,
            PoseidonRoundKeys_alpha26,
            PoseidonRoundKeys_alpha27,
            PoseidonRoundKeys_alpha28,
            PoseidonRoundKeys_alpha29,
            PoseidonRoundKeys_alpha30,
            PoseidonRoundKeys_z,
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
            claimed_sum,
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

