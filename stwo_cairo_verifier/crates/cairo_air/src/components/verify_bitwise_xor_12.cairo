use crate::prelude::*;

mod constraints;

pub const ELEM_BITS: u32 = 12;

pub const EXPAND_BITS: u32 = 2;

pub const LOG_SIZE: u32 = (ELEM_BITS - EXPAND_BITS) * 2;


#[derive(Drop, Serde, Copy)]
pub struct Claim {}

pub impl ClaimImpl of ClaimTrait<Claim> {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let preprocessed_log_sizes = array![LOG_SIZE].span();
        let trace_log_sizes = [LOG_SIZE; 16].span();
        let interaction_log_sizes = [LOG_SIZE; QM31_EXTENSION_DEGREE * 8].span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {}

    fn accumulate_relation_uses(self: @Claim, ref relation_uses: RelationUsesDict) {}
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
    pub verify_bitwise_xor_12_lookup_elements: crate::VerifyBitwiseXor_12Elements,
}

pub impl NewComponentImpl of NewComponent<Component> {
    type Claim = Claim;
    type InteractionClaim = InteractionClaim;

    fn new(
        claim: @Claim,
        interaction_claim: @InteractionClaim,
        interaction_elements: @CairoInteractionElements,
    ) -> Component {
        Component {
            claim: *claim,
            interaction_claim: *interaction_claim,
            verify_bitwise_xor_12_lookup_elements: interaction_elements
                .verify_bitwise_xor_12
                .clone(),
        }
    }
}

pub impl CairoComponentImpl of CairoComponent<Component> {
    fn evaluate_constraints_at_point(
        self: @Component,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        let VerifyBitwiseXor_12_z = *self.verify_bitwise_xor_12_lookup_elements.z;
        let mut verify_bitwise_xor_12_alpha_powers = self
            .verify_bitwise_xor_12_lookup_elements
            .alpha_powers
            .span();
        let VerifyBitwiseXor_12_alpha0 = *verify_bitwise_xor_12_alpha_powers.pop_front().unwrap();
        let VerifyBitwiseXor_12_alpha1 = *verify_bitwise_xor_12_alpha_powers.pop_front().unwrap();
        let VerifyBitwiseXor_12_alpha2 = *verify_bitwise_xor_12_alpha_powers.pop_front().unwrap();

        let claimed_sum = *self.interaction_claim.claimed_sum;

        let params = constraints::ConstraintParams {
            VerifyBitwiseXor_12_alpha0,
            VerifyBitwiseXor_12_alpha1,
            VerifyBitwiseXor_12_alpha2,
            VerifyBitwiseXor_12_z,
            claimed_sum,
            bitwise_xor_10_0: preprocessed_mask_values
                .get_and_mark_used(preprocessed_columns::BITWISE_XOR_10_0_IDX),
            bitwise_xor_10_1: preprocessed_mask_values
                .get_and_mark_used(preprocessed_columns::BITWISE_XOR_10_1_IDX),
            bitwise_xor_10_2: preprocessed_mask_values
                .get_and_mark_used(preprocessed_columns::BITWISE_XOR_10_2_IDX),
            column_size: pow2(LOG_SIZE).try_into().unwrap(),
        };

        let trace_domain = CanonicCosetImpl::new(LOG_SIZE);
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

