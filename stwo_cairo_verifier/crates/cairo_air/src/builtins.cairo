use components::add_mod_builtin::InteractionClaimImpl as AddModBuiltinInteractionClaimImpl;
use components::bitwise_builtin::InteractionClaimImpl as BitwiseBuiltinInteractionClaimImpl;
use components::mul_mod_builtin::InteractionClaimImpl as MulModBuiltinInteractionClaimImpl;
use components::pedersen_builtin::InteractionClaimImpl as PedersenBuiltinInteractionClaimImpl;
use components::poseidon_builtin::InteractionClaimImpl as PoseidonBuiltinInteractionClaimImpl;
use components::range_check96_builtin::InteractionClaimImpl as RangeCheckBuiltinBits96InteractionClaimImpl;
use components::range_check_builtin::InteractionClaimImpl as RangeCheckBuiltinBits128InteractionClaimImpl;
use core::box::BoxImpl;
use core::num::traits::Zero;
use stwo_cairo_air::cairo_component::CairoComponent;
use stwo_cairo_air::claim::ClaimTrait;
use stwo_cairo_air::{RelationUsesDict, components, utils};
use stwo_constraint_framework::{
    CommonLookupElements, LookupElementsImpl, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelTrait};
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::qm31::QM31;
use stwo_verifier_core::pcs::verifier::CommitmentSchemeVerifierImpl;
use stwo_verifier_core::utils::{ArrayImpl, OptionImpl};
use stwo_verifier_core::{ColumnSpan, TreeArray};

#[derive(Drop, Serde, Copy)]
pub struct BuiltinsClaim {
    pub add_mod_builtin: Option<components::add_mod_builtin::Claim>,
    pub bitwise_builtin: Option<components::bitwise_builtin::Claim>,
    pub mul_mod_builtin: Option<components::mul_mod_builtin::Claim>,
    pub pedersen_builtin: Option<components::pedersen_builtin::Claim>,
    pub poseidon_builtin: Option<components::poseidon_builtin::Claim>,
    pub range_check_96_builtin: Option<components::range_check96_builtin::Claim>,
    pub range_check_128_builtin: Option<components::range_check_builtin::Claim>,
}

pub impl BuiltinsClaimImpl of ClaimTrait<BuiltinsClaim> {
    fn mix_into(self: @BuiltinsClaim, ref channel: Channel) {
        let BuiltinsClaim {
            add_mod_builtin,
            bitwise_builtin,
            mul_mod_builtin,
            pedersen_builtin,
            poseidon_builtin,
            range_check_96_builtin,
            range_check_128_builtin,
        } = self;
        if let Some(claim) = add_mod_builtin {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }
        if let Some(claim) = bitwise_builtin {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }
        if let Some(claim) = mul_mod_builtin {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }
        if let Some(claim) = pedersen_builtin {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }
        if let Some(claim) = poseidon_builtin {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }
        if let Some(claim) = range_check_96_builtin {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }
        if let Some(claim) = range_check_128_builtin {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }
    }

    fn log_sizes(self: @BuiltinsClaim) -> TreeArray<Span<u32>> {
        let BuiltinsClaim {
            add_mod_builtin,
            bitwise_builtin,
            mul_mod_builtin,
            pedersen_builtin,
            poseidon_builtin,
            range_check_96_builtin,
            range_check_128_builtin,
        } = self;
        let mut log_sizes = array![];

        if let Some(claim) = add_mod_builtin {
            log_sizes.append(claim.log_sizes());
        }

        if let Some(claim) = bitwise_builtin {
            log_sizes.append(claim.log_sizes());
        }

        if let Some(claim) = mul_mod_builtin {
            log_sizes.append(claim.log_sizes());
        }

        if let Some(claim) = pedersen_builtin {
            log_sizes.append(claim.log_sizes());
        }

        if let Some(claim) = poseidon_builtin {
            log_sizes.append(claim.log_sizes());
        }

        if let Some(claim) = range_check_96_builtin {
            log_sizes.append(claim.log_sizes());
        }

        if let Some(claim) = range_check_128_builtin {
            log_sizes.append(claim.log_sizes());
        }

        utils::tree_array_concat_cols(log_sizes)
    }

    fn accumulate_relation_uses(self: @BuiltinsClaim, ref relation_uses: RelationUsesDict) {
        let BuiltinsClaim {
            add_mod_builtin,
            bitwise_builtin,
            mul_mod_builtin,
            pedersen_builtin,
            poseidon_builtin,
            range_check_96_builtin,
            range_check_128_builtin,
        } = self;

        if let Some(claim) = add_mod_builtin {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = bitwise_builtin {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = mul_mod_builtin {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = pedersen_builtin {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = poseidon_builtin {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = range_check_96_builtin {
            claim.accumulate_relation_uses(ref relation_uses);
        }
        if let Some(claim) = range_check_128_builtin {
            claim.accumulate_relation_uses(ref relation_uses);
        }
    }
}

#[derive(Drop, Serde, Copy)]
pub struct BuiltinsInteractionClaim {
    pub add_mod_builtin: Option<components::add_mod_builtin::InteractionClaim>,
    pub bitwise_builtin: Option<components::bitwise_builtin::InteractionClaim>,
    pub mul_mod_builtin: Option<components::mul_mod_builtin::InteractionClaim>,
    pub pedersen_builtin: Option<components::pedersen_builtin::InteractionClaim>,
    pub poseidon_builtin: Option<components::poseidon_builtin::InteractionClaim>,
    pub range_check_96_builtin: Option<components::range_check96_builtin::InteractionClaim>,
    pub range_check_128_builtin: Option<components::range_check_builtin::InteractionClaim>,
}

#[generate_trait]
pub impl BuiltinsInteractionClaimImpl of BuiltinsInteractionClaimTrait {
    fn mix_into(self: @BuiltinsInteractionClaim, ref channel: Channel) {
        let BuiltinsInteractionClaim {
            add_mod_builtin,
            bitwise_builtin,
            mul_mod_builtin,
            pedersen_builtin,
            poseidon_builtin,
            range_check_96_builtin,
            range_check_128_builtin,
        } = self;

        if let Some(claim) = add_mod_builtin {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }
        if let Some(claim) = bitwise_builtin {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }
        if let Some(claim) = mul_mod_builtin {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }
        if let Some(claim) = pedersen_builtin {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }
        if let Some(claim) = poseidon_builtin {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }
        if let Some(claim) = range_check_96_builtin {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }
        if let Some(claim) = range_check_128_builtin {
            channel.mix_u64(1);
            claim.mix_into(ref channel);
        } else {
            channel.mix_u64(0);
        }
    }

    fn sum(self: @BuiltinsInteractionClaim) -> QM31 {
        let BuiltinsInteractionClaim {
            add_mod_builtin,
            bitwise_builtin,
            mul_mod_builtin,
            pedersen_builtin,
            poseidon_builtin,
            range_check_96_builtin,
            range_check_128_builtin,
        } = self;
        let mut sum = Zero::zero();

        if let Some(claim) = add_mod_builtin {
            sum += *claim.claimed_sum;
        }

        if let Some(claim) = bitwise_builtin {
            sum += *claim.claimed_sum;
        }

        if let Some(claim) = mul_mod_builtin {
            sum += *claim.claimed_sum;
        }

        if let Some(claim) = pedersen_builtin {
            sum += *claim.claimed_sum;
        }

        if let Some(claim) = poseidon_builtin {
            sum += *claim.claimed_sum;
        }

        if let Some(claim) = range_check_96_builtin {
            sum += *claim.claimed_sum;
        }

        if let Some(claim) = range_check_128_builtin {
            sum += *claim.claimed_sum;
        }

        sum
    }
}


#[derive(Drop)]
#[cfg(not(feature: "poseidon252_verifier"))]
pub struct BuiltinComponents {
    pub add_mod_builtin: Option<components::add_mod_builtin::Component>,
    pub bitwise_builtin: Option<components::bitwise_builtin::Component>,
    pub mul_mod_builtin: Option<components::mul_mod_builtin::Component>,
    pub pedersen_builtin: Option<components::pedersen_builtin::Component>,
    pub poseidon_builtin: Option<components::poseidon_builtin::Component>,
    pub range_check_96_builtin: Option<components::range_check96_builtin::Component>,
    pub range_check_128_builtin: Option<components::range_check_builtin::Component>,
}

#[derive(Drop)]
#[cfg(and(feature: "poseidon252_verifier", not(feature: "poseidon_outputs_packing")))]
pub struct BuiltinComponents {
    pub bitwise_builtin: Option<components::bitwise_builtin::Component>,
    pub range_check_128_builtin: Option<components::range_check_builtin::Component>,
}

#[derive(Drop)]
#[cfg(and(feature: "poseidon252_verifier", feature: "poseidon_outputs_packing"))]
pub struct BuiltinComponents {
    pub bitwise_builtin: Option<components::bitwise_builtin::Component>,
    pub poseidon_builtin: Option<components::poseidon_builtin::Component>,
    pub range_check_128_builtin: Option<components::range_check_builtin::Component>,
}


#[generate_trait]
#[cfg(not(feature: "poseidon252_verifier"))]
pub impl BuiltinComponentsImpl of BuiltinComponentsTrait {
    fn new(
        claim: @BuiltinsClaim,
        common_lookup_elements: @CommonLookupElements,
        interaction_claim: @BuiltinsInteractionClaim,
    ) -> BuiltinComponents {
        let BuiltinsClaim {
            range_check_128_builtin,
            range_check_96_builtin,
            bitwise_builtin,
            add_mod_builtin,
            mul_mod_builtin,
            pedersen_builtin,
            poseidon_builtin,
        } = claim;

        let add_mod_builtin_component = components::add_mod_builtin::NewComponentImpl::try_new(
            add_mod_builtin, interaction_claim.add_mod_builtin, common_lookup_elements,
        );

        let bitwise_builtin_component = components::bitwise_builtin::NewComponentImpl::try_new(
            bitwise_builtin, interaction_claim.bitwise_builtin, common_lookup_elements,
        );

        let mul_mod_builtin_component = components::mul_mod_builtin::NewComponentImpl::try_new(
            mul_mod_builtin, interaction_claim.mul_mod_builtin, common_lookup_elements,
        );

        let pedersen_builtin_component = components::pedersen_builtin::NewComponentImpl::try_new(
            pedersen_builtin, interaction_claim.pedersen_builtin, common_lookup_elements,
        );

        let poseidon_builtin_component = components::poseidon_builtin::NewComponentImpl::try_new(
            poseidon_builtin, interaction_claim.poseidon_builtin, common_lookup_elements,
        );

        let range_check_96_builtin_component =
            components::range_check96_builtin::NewComponentImpl::try_new(
            range_check_96_builtin,
            interaction_claim.range_check_96_builtin,
            common_lookup_elements,
        );

        let range_check_128_builtin_component =
            components::range_check_builtin::NewComponentImpl::try_new(
            range_check_128_builtin,
            interaction_claim.range_check_128_builtin,
            common_lookup_elements,
        );

        BuiltinComponents {
            add_mod_builtin: add_mod_builtin_component,
            bitwise_builtin: bitwise_builtin_component,
            mul_mod_builtin: mul_mod_builtin_component,
            pedersen_builtin: pedersen_builtin_component,
            poseidon_builtin: poseidon_builtin_component,
            range_check_96_builtin: range_check_96_builtin_component,
            range_check_128_builtin: range_check_128_builtin_component,
        }
    }

    fn evaluate_constraints_at_point(
        self: @BuiltinComponents,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        let BuiltinComponents {
            add_mod_builtin,
            bitwise_builtin,
            mul_mod_builtin,
            pedersen_builtin,
            poseidon_builtin,
            range_check_96_builtin,
            range_check_128_builtin,
        } = self;

        if let Option::Some(component) = add_mod_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        if let Option::Some(component) = bitwise_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        if let Option::Some(component) = mul_mod_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        if let Option::Some(component) = pedersen_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        if let Option::Some(component) = poseidon_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        if let Option::Some(component) = range_check_96_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        if let Option::Some(component) = range_check_128_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
    }
}

#[generate_trait]
#[cfg(and(feature: "poseidon252_verifier", not(feature: "poseidon_outputs_packing")))]
pub impl BuiltinComponentsImpl of BuiltinComponentsTrait {
    fn new(
        claim: @BuiltinsClaim,
        common_lookup_elements: @CommonLookupElements,
        interaction_claim: @BuiltinsInteractionClaim,
    ) -> BuiltinComponents {
        let BuiltinsClaim {
            range_check_128_builtin,
            range_check_96_builtin,
            bitwise_builtin,
            add_mod_builtin,
            mul_mod_builtin,
            pedersen_builtin,
            poseidon_builtin,
        } = claim;
        assert!(
            range_check_96_builtin.is_none() && interaction_claim.range_check_96_builtin.is_none(),
        );
        assert!(add_mod_builtin.is_none() && interaction_claim.add_mod_builtin.is_none());
        assert!(mul_mod_builtin.is_none() && interaction_claim.mul_mod_builtin.is_none());
        assert!(pedersen_builtin.is_none() && interaction_claim.pedersen_builtin.is_none());
        assert!(poseidon_builtin.is_none() && interaction_claim.poseidon_builtin.is_none());

        let bitwise_builtin_component = components::bitwise_builtin::NewComponentImpl::try_new(
            bitwise_builtin, interaction_claim.bitwise_builtin, common_lookup_elements,
        );

        let range_check_128_builtin_component =
            components::range_check_builtin::NewComponentImpl::try_new(
            range_check_128_builtin,
            interaction_claim.range_check_128_builtin,
            common_lookup_elements,
        );

        BuiltinComponents {
            bitwise_builtin: bitwise_builtin_component,
            range_check_128_builtin: range_check_128_builtin_component,
        }
    }

    fn evaluate_constraints_at_point(
        self: @BuiltinComponents,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        let BuiltinComponents { bitwise_builtin, range_check_128_builtin } = self;

        if let Option::Some(component) = bitwise_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        if let Option::Some(component) = range_check_128_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
    }
}

#[generate_trait]
#[cfg(and(feature: "poseidon252_verifier", feature: "poseidon_outputs_packing"))]
pub impl BuiltinComponentsImpl of BuiltinComponentsTrait {
    fn new(
        claim: @BuiltinsClaim,
        common_lookup_elements: @CommonLookupElements,
        interaction_claim: @BuiltinsInteractionClaim,
    ) -> BuiltinComponents {
        let BuiltinsClaim {
            range_check_128_builtin,
            range_check_96_builtin,
            bitwise_builtin,
            add_mod_builtin,
            mul_mod_builtin,
            pedersen_builtin,
            poseidon_builtin,
        } = claim;
        assert!(
            range_check_96_builtin.is_none() && interaction_claim.range_check_96_builtin.is_none(),
        );
        assert!(add_mod_builtin.is_none() && interaction_claim.add_mod_builtin.is_none());
        assert!(mul_mod_builtin.is_none() && interaction_claim.mul_mod_builtin.is_none());
        assert!(pedersen_builtin.is_none() && interaction_claim.pedersen_builtin.is_none());

        let bitwise_builtin_component = components::bitwise_builtin::NewComponentImpl::try_new(
            bitwise_builtin, interaction_claim.bitwise_builtin, common_lookup_elements,
        );

        let poseidon_builtin_component = components::poseidon_builtin::NewComponentImpl::try_new(
            poseidon_builtin, interaction_claim.poseidon_builtin, common_lookup_elements,
        );

        let range_check_128_builtin_component =
            components::range_check_builtin::NewComponentImpl::try_new(
            range_check_128_builtin,
            interaction_claim.range_check_128_builtin,
            common_lookup_elements,
        );

        BuiltinComponents {
            bitwise_builtin: bitwise_builtin_component,
            poseidon_builtin: poseidon_builtin_component,
            range_check_128_builtin: range_check_128_builtin_component,
        }
    }

    fn evaluate_constraints_at_point(
        self: @BuiltinComponents,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        let BuiltinComponents { bitwise_builtin, poseidon_builtin, range_check_128_builtin } = self;

        if let Option::Some(component) = bitwise_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        if let Option::Some(component) = poseidon_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }

        if let Option::Some(component) = range_check_128_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    point,
                );
        }
    }
}
