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
}

#[generate_trait]
pub impl ClaimImpl of ClaimTrait {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = *self.log_size;
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = ArrayImpl::new_repeated(126, log_size).span();
        let interaction_log_sizes = ArrayImpl::new_repeated(QM31_EXTENSION_DEGREE * 6, log_size)
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
    pub range_check_3_3_3_3_3_lookup_elements: crate::RangeCheck_3_3_3_3_3Elements,
    pub poseidon_round_keys_lookup_elements: crate::PoseidonRoundKeysElements,
    pub poseidon_full_round_chain_lookup_elements: crate::PoseidonFullRoundChainElements,
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

        let log_size = *self.claim.log_size;

        let claimed_sum = *self.interaction_claim.claimed_sum;

        let params = constraints::ConstraintParams {
            column_size: pow2(log_size).try_into().unwrap(),
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
            RangeCheck_3_3_3_3_3_alpha0,
            RangeCheck_3_3_3_3_3_alpha1,
            RangeCheck_3_3_3_3_3_alpha2,
            RangeCheck_3_3_3_3_3_alpha3,
            RangeCheck_3_3_3_3_3_alpha4,
            RangeCheck_3_3_3_3_3_z,
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

