// STUB: to be replaced by auto-generated output from stwo-air-infra.
// Mirrors stwo-circuits/crates/circuit_air/src/components/blake_round_sigma.rs.

use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 0;
pub const N_INTERACTION_COLUMNS: usize = 0;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 0] = [];

#[derive(Drop, Serde, Copy)]
pub struct Claim {
    pub log_size: u32,
}

pub impl ClaimImpl of ClaimTrait<Claim> {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        // TODO(circuit-verifier): real per-tree column lists come from the auto-generated
        // component. For now the stub declares no columns.
        array![array![*self.log_size].span(), array![].span(), array![].span()]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        channel.mix_u64((*(self.log_size)).into());
    }

    fn accumulate_relation_uses(self: @Claim, ref relation_uses: RelationUsesDict) {
        accumulate_relation_uses(ref relation_uses, RELATION_USES_PER_ROW.span(), *self.log_size);
    }
}

#[derive(Drop, Serde, Copy)]
pub struct InteractionClaim {
    pub claimed_sum: QM31,
}

pub impl InteractionClaimImpl of InteractionClaimTrait<InteractionClaim> {
    fn mix_into(self: @InteractionClaim, ref channel: Channel) {
        channel.mix_felts([*self.claimed_sum].span());
    }
}


#[derive(Drop)]
pub struct Component {
    pub claim: Claim,
    pub interaction_claim: InteractionClaim,
    pub common_lookup_elements: CommonLookupElements,
}

pub impl NewComponentImpl of NewComponent<Component> {
    type Claim = Claim;
    type InteractionClaim = InteractionClaim;

    fn new(
        claim: @Claim,
        interaction_claim: @InteractionClaim,
        common_lookup_elements: @CommonLookupElements,
    ) -> Component {
        Component {
            claim: *claim,
            interaction_claim: *interaction_claim,
            common_lookup_elements: common_lookup_elements.clone(),
        }
    }
}

pub impl CircuitComponentImpl of CircuitComponent<Component> {
    fn evaluate_constraints_at_point(
        self: @Component,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
    ) {
        // TODO(circuit-verifier): replaced by AIR-team generated evaluator.
        // The stub intentionally does nothing — no columns consumed, sum left unchanged.
        let _ = (self, random_coeff);
    }
}
