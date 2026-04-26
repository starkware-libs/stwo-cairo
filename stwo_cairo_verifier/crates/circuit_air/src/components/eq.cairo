// Hand-written mirror of stwo-circuits/crates/circuit_air/src/components/eq.rs.
// `eq` is not in stwo-air-infra's compiled JSONs (the Rust component is also handwritten),
// so this Cairo port follows the patterns produced by `cairo_code_gen` for paired-logup
// components — see
// stwo-air-infra/crates/air_code_gen/src/components_code_gen/cairo_constraints/lookups.rs for the
// exact emission rules.
//
// The component yields `[GATE_RELATION_ID, addr, c0, c1, c2, c3]` to the `Gate` relation
// twice per row (once with `addr = eq_in0_address`, once with `addr = eq_in1_address`),
// where `(c0, c1, c2, c3)` are the four trace columns. Both yields are paired into a
// single logup fraction (`finalize_logup_in_pairs` on the Rust side), occupying 4
// interaction columns.

use crate::prelude::*;

pub const N_TRACE_COLUMNS: usize = 4;
pub const N_INTERACTION_COLUMNS: usize = 4;
pub const RELATION_USES_PER_ROW: [(felt252, u32); 1] = [('Gate', 2)];

#[derive(Drop, Serde, Copy)]
pub struct Claim {
    pub log_size: u32,
}

pub impl ClaimImpl of ClaimTrait<Claim> {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = *(self.log_size);
        // 2 preprocessed columns (eq_in0_address, eq_in1_address), 4 trace cols, 4 interaction.
        let preprocessed_log_sizes = [log_size; 2].span();
        let trace_log_sizes = [log_size; N_TRACE_COLUMNS].span();
        let interaction_log_sizes = [log_size; N_INTERACTION_COLUMNS].span();
        array![preprocessed_log_sizes, trace_log_sizes, interaction_log_sizes]
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
        let log_size = *self.claim.log_size;
        let claimed_sum = *self.interaction_claim.claimed_sum;
        let column_size = m31(pow2(log_size));

        let eq_in0_address = preprocessed_mask_values.get_and_mark_used(EQ_IN0_ADDRESS_IDX);
        let eq_in1_address = preprocessed_mask_values.get_and_mark_used(EQ_IN1_ADDRESS_IDX);

        let [in_col0, in_col1, in_col2, in_col3]: [Span<QM31>; 4] = (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [in_col0]: [QM31; 1] = (*in_col0.try_into().unwrap()).unbox();
        let [in_col1]: [QM31; 1] = (*in_col1.try_into().unwrap()).unbox();
        let [in_col2]: [QM31; 1] = (*in_col2.try_into().unwrap()).unbox();
        let [in_col3]: [QM31; 1] = (*in_col3.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        // GATE_RELATION_ID = 378353459 (see stwo-circuits/crates/circuit_air/src/relations.rs).
        let gate_relation_id = qm31_const::<378353459, 0, 0, 0>();
        // The two yields differ only in the address slot.
        let eq_sum_0 = self
            .common_lookup_elements
            .combine_qm31(
                [gate_relation_id, eq_in0_address, in_col0, in_col1, in_col2, in_col3].span(),
            );
        let eq_sum_1 = self
            .common_lookup_elements
            .combine_qm31(
                [gate_relation_id, eq_in1_address, in_col0, in_col1, in_col2, in_col3].span(),
            );
        // Both numerators are 1 (`E::EF::one()` in eq.rs::evaluate).
        let one: QM31 = qm31_const::<1, 0, 0, 0>();
        let numerator_0 = one;
        let numerator_1 = one;

        lookup_constraints(
            ref sum,
            random_coeff,
            claimed_sum,
            numerator_0,
            numerator_1,
            column_size,
            ref interaction_trace_mask_values,
            eq_sum_0,
            eq_sum_1,
        );
    }
}


fn lookup_constraints(
    ref sum: QM31,
    random_coeff: QM31,
    claimed_sum: QM31,
    numerator_0: QM31,
    numerator_1: QM31,
    column_size: M31,
    ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
    eq_sum_0: QM31,
    eq_sum_1: QM31,
) {
    let [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3]: [Span<QM31>; 4] =
        (*interaction_trace_mask_values
        .multi_pop_front()
        .unwrap())
        .unbox();

    // Single chunk → all 4 columns are the LAST chunk and are read with both
    // _neg1 (previous-row) and current values, per
    // air_code_gen::cairo_constraints::lookups::get_interaction_trace_vars.
    let [trace_2_col0_neg1, trace_2_col0]: [QM31; 2] = (*trace_2_col0.try_into().unwrap()).unbox();
    let [trace_2_col1_neg1, trace_2_col1]: [QM31; 2] = (*trace_2_col1.try_into().unwrap()).unbox();
    let [trace_2_col2_neg1, trace_2_col2]: [QM31; 2] = (*trace_2_col2.try_into().unwrap()).unbox();
    let [trace_2_col3_neg1, trace_2_col3]: [QM31; 2] = (*trace_2_col3.try_into().unwrap()).unbox();

    core::internal::revoke_ap_tracking();

    // Paired-logup constraint for the LAST (and only) chunk. Both relation entries are
    // CONSUMES on the rust side (`E::EF::one()` numerator), so the prover writes
    // `(sum_a + sum_b) / (sum_a * sum_b)` per row. The verifier-side constraint is
    //   diff * denom - num = (diff + shift) * sum_a * sum_b - sum_a - sum_b
    // with `diff = cur - cur_neg1` and `shift = claimed_sum / column_size`.
    let constraint_quotient = (((QM31Impl::from_partial_evals(
        [trace_2_col0, trace_2_col1, trace_2_col2, trace_2_col3],
    )
        - QM31Impl::from_partial_evals(
            [trace_2_col0_neg1, trace_2_col1_neg1, trace_2_col2_neg1, trace_2_col3_neg1],
        )
        + (claimed_sum * (column_size.inverse().into())))
        * eq_sum_0
        * eq_sum_1)
        - (eq_sum_0 * numerator_1)
        - (eq_sum_1 * numerator_0));
    sum = sum * random_coeff + constraint_quotient;
}
