use components::add_mod_builtin::InteractionClaimImpl as AddModBuiltinInteractionClaimImpl;
use components::bitwise_builtin::InteractionClaimImpl as BitwiseBuiltinInteractionClaimImpl;
use components::mul_mod_builtin::InteractionClaimImpl as MulModBuiltinInteractionClaimImpl;
use components::pedersen_builtin::InteractionClaimImpl as PedersenBuiltinInteractionClaimImpl;
use components::poseidon_builtin::InteractionClaimImpl as PoseidonBuiltinInteractionClaimImpl;
use components::range_check_builtin_bits_128::InteractionClaimImpl as RangeCheckBuiltinBits128InteractionClaimImpl;
use components::range_check_builtin_bits_96::InteractionClaimImpl as RangeCheckBuiltinBits96InteractionClaimImpl;
use core::box::BoxImpl;
use core::num::traits::Zero;
use stwo_cairo_air::cairo_component::CairoComponent;
use stwo_cairo_air::claim::ClaimTrait;
use stwo_cairo_air::{CairoInteractionElements, RelationUsesDict, components, utils};
use stwo_constraint_framework::{
    LookupElementsImpl, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::Channel;
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
    pub range_check_96_builtin: Option<components::range_check_builtin_bits_96::Claim>,
    pub range_check_128_builtin: Option<components::range_check_builtin_bits_128::Claim>,
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
            claim.mix_into(ref channel);
        }
        if let Some(claim) = bitwise_builtin {
            claim.mix_into(ref channel);
        }
        if let Some(claim) = mul_mod_builtin {
            claim.mix_into(ref channel);
        }
        if let Some(claim) = pedersen_builtin {
            claim.mix_into(ref channel);
        }
        if let Some(claim) = poseidon_builtin {
            claim.mix_into(ref channel);
        }
        if let Some(claim) = range_check_96_builtin {
            claim.mix_into(ref channel);
        }
        if let Some(claim) = range_check_128_builtin {
            claim.mix_into(ref channel);
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
    pub range_check_96_builtin: Option<components::range_check_builtin_bits_96::InteractionClaim>,
    pub range_check_128_builtin: Option<components::range_check_builtin_bits_128::InteractionClaim>,
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
            claim.mix_into(ref channel);
        }
        if let Some(claim) = bitwise_builtin {
            claim.mix_into(ref channel);
        }
        if let Some(claim) = mul_mod_builtin {
            claim.mix_into(ref channel);
        }
        if let Some(claim) = pedersen_builtin {
            claim.mix_into(ref channel);
        }
        if let Some(claim) = poseidon_builtin {
            claim.mix_into(ref channel);
        }
        if let Some(claim) = range_check_96_builtin {
            claim.mix_into(ref channel);
        }
        if let Some(claim) = range_check_128_builtin {
            claim.mix_into(ref channel);
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
    pub range_check_96_builtin: Option<components::range_check_builtin_bits_96::Component>,
    pub range_check_128_builtin: Option<components::range_check_builtin_bits_128::Component>,
}

#[derive(Drop)]
#[cfg(and(feature: "poseidon252_verifier", not(feature: "poseidon_outputs_packing")))]
pub struct BuiltinComponents {
    pub bitwise_builtin: Option<components::bitwise_builtin::Component>,
    pub range_check_128_builtin: Option<components::range_check_builtin_bits_128::Component>,
}

#[derive(Drop)]
#[cfg(and(feature: "poseidon252_verifier", feature: "poseidon_outputs_packing"))]
pub struct BuiltinComponents {
    pub bitwise_builtin: Option<components::bitwise_builtin::Component>,
    pub poseidon_builtin: Option<components::poseidon_builtin::Component>,
    pub range_check_128_builtin: Option<components::range_check_builtin_bits_128::Component>,
}


#[generate_trait]
#[cfg(not(feature: "poseidon252_verifier"))]
pub impl BuiltinComponentsImpl of BuiltinComponentsTrait {
    fn new(
        claim: @BuiltinsClaim,
        interaction_elements: @CairoInteractionElements,
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

        let mut add_mod_builtin_component = Option::None;
        if let Option::Some(claim) = add_mod_builtin {
            add_mod_builtin_component =
                Option::Some(
                    components::add_mod_builtin::NewComponentImpl::new(
                        claim, @interaction_claim.add_mod_builtin.unwrap(), interaction_elements,
                    ),
                );
        }

        let mut bitwise_builtin_component = Option::None;
        if let Option::Some(claim) = bitwise_builtin {
            bitwise_builtin_component =
                Option::Some(
                    components::bitwise_builtin::NewComponentImpl::new(
                        claim, @interaction_claim.bitwise_builtin.unwrap(), interaction_elements,
                    ),
                );
        }

        let mut mul_mod_builtin_component = Option::None;
        if let Option::Some(claim) = mul_mod_builtin {
            mul_mod_builtin_component =
                Option::Some(
                    components::mul_mod_builtin::NewComponentImpl::new(
                        claim, @interaction_claim.mul_mod_builtin.unwrap(), interaction_elements,
                    ),
                );
        }

        let mut pedersen_builtin_component = Option::None;
        if let Option::Some(claim) = pedersen_builtin {
            pedersen_builtin_component =
                Option::Some(
                    components::pedersen_builtin::NewComponentImpl::new(
                        claim, @interaction_claim.pedersen_builtin.unwrap(), interaction_elements,
                    ),
                );
        }

        let mut poseidon_builtin_component = Option::None;
        if let Option::Some(claim) = poseidon_builtin {
            poseidon_builtin_component =
                Option::Some(
                    components::poseidon_builtin::NewComponentImpl::new(
                        claim, @interaction_claim.poseidon_builtin.unwrap(), interaction_elements,
                    ),
                );
        }

        let mut range_check_96_builtin_component = Option::None;
        if let Option::Some(claim) = range_check_96_builtin {
            range_check_96_builtin_component =
                Option::Some(
                    components::range_check_builtin_bits_96::NewComponentImpl::new(
                        claim,
                        @interaction_claim.range_check_96_builtin.unwrap(),
                        interaction_elements,
                    ),
                );
        }

        let mut range_check_128_builtin_component = Option::None;
        if let Option::Some(claim) = range_check_128_builtin {
            range_check_128_builtin_component =
                Option::Some(
                    components::range_check_builtin_bits_128::NewComponentImpl::new(
                        claim,
                        @interaction_claim.range_check_128_builtin.unwrap(),
                        interaction_elements,
                    ),
                );
        }

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
        interaction_elements: @CairoInteractionElements,
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
        assert!(range_check_96_builtin.is_none());
        assert!(add_mod_builtin.is_none());
        assert!(mul_mod_builtin.is_none());
        assert!(pedersen_builtin.is_none());
        assert!(poseidon_builtin.is_none());

        let mut bitwise_builtin_component = Option::None;
        if let Option::Some(claim) = bitwise_builtin {
            bitwise_builtin_component =
                Option::Some(
                    components::bitwise_builtin::NewComponentImpl::new(
                        claim, @interaction_claim.bitwise_builtin.unwrap(), interaction_elements,
                    ),
                );
        }

        let mut range_check_128_builtin_component = Option::None;
        if let Option::Some(claim) = range_check_128_builtin {
            range_check_128_builtin_component =
                Option::Some(
                    components::range_check_builtin_bits_128::NewComponentImpl::new(
                        claim,
                        @interaction_claim.range_check_128_builtin.unwrap(),
                        interaction_elements,
                    ),
                );
        }

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
        interaction_elements: @CairoInteractionElements,
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
        assert!(range_check_96_builtin.is_none());
        assert!(add_mod_builtin.is_none());
        assert!(mul_mod_builtin.is_none());
        assert!(pedersen_builtin.is_none());

        let mut bitwise_builtin_component = Option::None;
        if let Option::Some(claim) = bitwise_builtin {
            bitwise_builtin_component =
                Option::Some(
                    components::bitwise_builtin::NewComponentImpl::new(
                        claim, @interaction_claim.bitwise_builtin.unwrap(), interaction_elements,
                    ),
                );
        }

        let mut poseidon_builtin_component = Option::None;
        if let Option::Some(claim) = poseidon_builtin {
            poseidon_builtin_component =
                Option::Some(
                    components::poseidon_builtin::NewComponentImpl::new(
                        claim, @interaction_claim.poseidon_builtin.unwrap(), interaction_elements,
                    ),
                );
        }

        let mut range_check_128_builtin_component = Option::None;
        if let Option::Some(claim) = range_check_128_builtin {
            range_check_128_builtin_component =
                Option::Some(
                    components::range_check_builtin_bits_128::NewComponentImpl::new(
                        claim,
                        @interaction_claim.range_check_128_builtin.unwrap(),
                        interaction_elements,
                    ),
                );
        }

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

