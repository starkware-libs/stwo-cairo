// Hand-ported to mirror
// `stwo-circuits/crates/circuit_air/src/components/verify_bitwise_xor_12.rs`.
//
// Unlike the cairo-CPU `verify_bitwise_xor_12` (1 trace column, single lookup), the
// circuit AIR variant builds the 12-bit XOR table by EXPANDing a 10-bit table 4×4 times:
// for each pair (i, j) in {0..4}², a multiplicity column counts how many times
// `(a_low + i·1024, b_low + j·1024, (a_low ^ b_low) + (i ^ j)·1024)` is yielded. There
// are 16 multiplicities — paired up via `finalize_logup_in_pairs` into 8 batches.

use crate::prelude::*;

pub const ELEM_BITS: u32 = 12;
pub const EXPAND_BITS: u32 = 2;
pub const LIMB_BITS: u32 = ELEM_BITS - EXPAND_BITS; // = 10
pub const LOG_SIZE: u32 = (ELEM_BITS - EXPAND_BITS) * 2; // = 20
pub const N_MULT_COLUMNS: usize = 16; // = 1 << (EXPAND_BITS * 2)
pub const N_TRACE_COLUMNS: usize = N_MULT_COLUMNS;
pub const N_INTERACTION_COLUMNS: usize = 32;

/// `1 << LIMB_BITS = 1024`. Used as the offset shift between expansion bands.
const LIMB_OFFSET: u32 = 1024;

#[derive(Drop, Serde, Copy)]
pub struct Claim {}

pub impl ClaimImpl of ClaimTrait<Claim> {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let log_size = LOG_SIZE;
        let preprocessed_log_sizes = array![log_size].span();
        let trace_log_sizes = [log_size; N_TRACE_COLUMNS].span();
        // 8 pairs × QM31 (4 base columns each) = 32 interaction columns.
        let interaction_log_sizes = [log_size; N_INTERACTION_COLUMNS].span();
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

pub impl AirComponentImpl of AirComponent<Component> {
    fn evaluate_constraints_at_point(
        self: @Component,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
    ) {
        let log_size = LOG_SIZE;
        let claimed_sum = *self.interaction_claim.claimed_sum;
        let column_size = m31(pow2(log_size));

        // Preprocessed 10-bit XOR table columns (aliased as BITWISE_XOR_12_*_IDX in
        // `preprocessed_columns.cairo`).
        let bitwise_xor_10_0 = preprocessed_mask_values.get_and_mark_used(BITWISE_XOR_12_0_IDX);
        let bitwise_xor_10_1 = preprocessed_mask_values.get_and_mark_used(BITWISE_XOR_12_1_IDX);
        let bitwise_xor_10_2 = preprocessed_mask_values.get_and_mark_used(BITWISE_XOR_12_2_IDX);

        // 16 multiplicity columns, one per (i, j) ∈ {0..4}².
        let [
            multiplicity_col0,
            multiplicity_col1,
            multiplicity_col2,
            multiplicity_col3,
            multiplicity_col4,
            multiplicity_col5,
            multiplicity_col6,
            multiplicity_col7,
            multiplicity_col8,
            multiplicity_col9,
            multiplicity_col10,
            multiplicity_col11,
            multiplicity_col12,
            multiplicity_col13,
            multiplicity_col14,
            multiplicity_col15,
        ]: [Span<QM31>; 16] =
            (*trace_mask_values
            .multi_pop_front()
            .unwrap())
            .unbox();
        let [multiplicity_col0]: [QM31; 1] = (*multiplicity_col0.try_into().unwrap()).unbox();
        let [multiplicity_col1]: [QM31; 1] = (*multiplicity_col1.try_into().unwrap()).unbox();
        let [multiplicity_col2]: [QM31; 1] = (*multiplicity_col2.try_into().unwrap()).unbox();
        let [multiplicity_col3]: [QM31; 1] = (*multiplicity_col3.try_into().unwrap()).unbox();
        let [multiplicity_col4]: [QM31; 1] = (*multiplicity_col4.try_into().unwrap()).unbox();
        let [multiplicity_col5]: [QM31; 1] = (*multiplicity_col5.try_into().unwrap()).unbox();
        let [multiplicity_col6]: [QM31; 1] = (*multiplicity_col6.try_into().unwrap()).unbox();
        let [multiplicity_col7]: [QM31; 1] = (*multiplicity_col7.try_into().unwrap()).unbox();
        let [multiplicity_col8]: [QM31; 1] = (*multiplicity_col8.try_into().unwrap()).unbox();
        let [multiplicity_col9]: [QM31; 1] = (*multiplicity_col9.try_into().unwrap()).unbox();
        let [multiplicity_col10]: [QM31; 1] = (*multiplicity_col10.try_into().unwrap()).unbox();
        let [multiplicity_col11]: [QM31; 1] = (*multiplicity_col11.try_into().unwrap()).unbox();
        let [multiplicity_col12]: [QM31; 1] = (*multiplicity_col12.try_into().unwrap()).unbox();
        let [multiplicity_col13]: [QM31; 1] = (*multiplicity_col13.try_into().unwrap()).unbox();
        let [multiplicity_col14]: [QM31; 1] = (*multiplicity_col14.try_into().unwrap()).unbox();
        let [multiplicity_col15]: [QM31; 1] = (*multiplicity_col15.try_into().unwrap()).unbox();

        core::internal::revoke_ap_tracking();

        // For each lookup `n = 4*i + j`, the relation combines:
        //   (relation_id, a_low + i*1024, b_low + j*1024, c_low + (i^j)*1024)
        let relation_id = qm31_const::<648362599, 0, 0, 0>();
        let limb_off: QM31 = m31(LIMB_OFFSET).into();
        let limb_off_2: QM31 = m31(LIMB_OFFSET * 2).into();
        let limb_off_3: QM31 = m31(LIMB_OFFSET * 3).into();

        // Precompute the 4 a-side values (one per `i`) and 4 b-side values (one per `j`).
        let a_offs = [
            bitwise_xor_10_0, bitwise_xor_10_0 + limb_off, bitwise_xor_10_0 + limb_off_2,
            bitwise_xor_10_0 + limb_off_3,
        ];
        let b_offs = [
            bitwise_xor_10_1, bitwise_xor_10_1 + limb_off, bitwise_xor_10_1 + limb_off_2,
            bitwise_xor_10_1 + limb_off_3,
        ];
        // c offsets — XOR of i and j folded in.
        let c_offs = [
            bitwise_xor_10_2, bitwise_xor_10_2 + limb_off, bitwise_xor_10_2 + limb_off_2,
            bitwise_xor_10_2 + limb_off_3,
        ];

        // Build sum_n = combine(relation_id, a_offs[i], b_offs[j], c_offs[i^j]) for each
        // lookup n = 4*i + j.
        let common = self.common_lookup_elements;
        let [a0, a1, a2, a3] = a_offs;
        let [b0, b1, b2, b3] = b_offs;
        let [c0, c1, c2, c3] = c_offs;
        let sum_0 = common.combine_qm31([relation_id, a0, b0, c0].span());
        let sum_1 = common.combine_qm31([relation_id, a0, b1, c1].span());
        let sum_2 = common.combine_qm31([relation_id, a0, b2, c2].span());
        let sum_3 = common.combine_qm31([relation_id, a0, b3, c3].span());
        let sum_4 = common.combine_qm31([relation_id, a1, b0, c1].span());
        let sum_5 = common.combine_qm31([relation_id, a1, b1, c0].span());
        let sum_6 = common.combine_qm31([relation_id, a1, b2, c3].span());
        let sum_7 = common.combine_qm31([relation_id, a1, b3, c2].span());
        let sum_8 = common.combine_qm31([relation_id, a2, b0, c2].span());
        let sum_9 = common.combine_qm31([relation_id, a2, b1, c3].span());
        let sum_10 = common.combine_qm31([relation_id, a2, b2, c0].span());
        let sum_11 = common.combine_qm31([relation_id, a2, b3, c1].span());
        let sum_12 = common.combine_qm31([relation_id, a3, b0, c3].span());
        let sum_13 = common.combine_qm31([relation_id, a3, b1, c2].span());
        let sum_14 = common.combine_qm31([relation_id, a3, b2, c1].span());
        let sum_15 = common.combine_qm31([relation_id, a3, b3, c0].span());

        lookup_constraints(
            ref sum,
            random_coeff,
            claimed_sum,
            column_size,
            ref interaction_trace_mask_values,
            sum_0,
            sum_1,
            sum_2,
            sum_3,
            sum_4,
            sum_5,
            sum_6,
            sum_7,
            sum_8,
            sum_9,
            sum_10,
            sum_11,
            sum_12,
            sum_13,
            sum_14,
            sum_15,
            multiplicity_col0,
            multiplicity_col1,
            multiplicity_col2,
            multiplicity_col3,
            multiplicity_col4,
            multiplicity_col5,
            multiplicity_col6,
            multiplicity_col7,
            multiplicity_col8,
            multiplicity_col9,
            multiplicity_col10,
            multiplicity_col11,
            multiplicity_col12,
            multiplicity_col13,
            multiplicity_col14,
            multiplicity_col15,
        );
    }
}


/// Emits 8 batched logup constraints. `sum_n` is the lookup denominator for lookup n
/// (`combine(relation_id, a, b, c)`); `mult_n` is the multiplicity (positive). Pair k
/// fuses lookups (2k, 2k+1) into 4 interaction columns. Batches 0..6 are non-last
/// (cumulative-column constraints); batch 7 carries the `claimed_sum` shift.
///
/// The sign of `(sum * mult)` is `+` because this component is on the YIELD side of the
/// lookup (rust-side numerator is `-multiplicity`).
fn lookup_constraints(
    ref sum: QM31,
    random_coeff: QM31,
    claimed_sum: QM31,
    column_size: M31,
    ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
    sum_0: QM31,
    sum_1: QM31,
    sum_2: QM31,
    sum_3: QM31,
    sum_4: QM31,
    sum_5: QM31,
    sum_6: QM31,
    sum_7: QM31,
    sum_8: QM31,
    sum_9: QM31,
    sum_10: QM31,
    sum_11: QM31,
    sum_12: QM31,
    sum_13: QM31,
    sum_14: QM31,
    sum_15: QM31,
    num_0: QM31,
    num_1: QM31,
    num_2: QM31,
    num_3: QM31,
    num_4: QM31,
    num_5: QM31,
    num_6: QM31,
    num_7: QM31,
    num_8: QM31,
    num_9: QM31,
    num_10: QM31,
    num_11: QM31,
    num_12: QM31,
    num_13: QM31,
    num_14: QM31,
    num_15: QM31,
) {
    // 32 interaction columns: 8 batches × 4 (one QM31 per batch).
    let [
        c0,
        c1,
        c2,
        c3,
        c4,
        c5,
        c6,
        c7,
        c8,
        c9,
        c10,
        c11,
        c12,
        c13,
        c14,
        c15,
        c16,
        c17,
        c18,
        c19,
        c20,
        c21,
        c22,
        c23,
        c24,
        c25,
        c26,
        c27,
        c28,
        c29,
        c30,
        c31,
    ]: [Span<QM31>; 32] =
        (*interaction_trace_mask_values
        .multi_pop_front()
        .unwrap())
        .unbox();

    // Non-last batches (0..6) need only the current row's value.
    let [c0]: [QM31; 1] = (*c0.try_into().unwrap()).unbox();
    let [c1]: [QM31; 1] = (*c1.try_into().unwrap()).unbox();
    let [c2]: [QM31; 1] = (*c2.try_into().unwrap()).unbox();
    let [c3]: [QM31; 1] = (*c3.try_into().unwrap()).unbox();
    let [c4]: [QM31; 1] = (*c4.try_into().unwrap()).unbox();
    let [c5]: [QM31; 1] = (*c5.try_into().unwrap()).unbox();
    let [c6]: [QM31; 1] = (*c6.try_into().unwrap()).unbox();
    let [c7]: [QM31; 1] = (*c7.try_into().unwrap()).unbox();
    let [c8]: [QM31; 1] = (*c8.try_into().unwrap()).unbox();
    let [c9]: [QM31; 1] = (*c9.try_into().unwrap()).unbox();
    let [c10]: [QM31; 1] = (*c10.try_into().unwrap()).unbox();
    let [c11]: [QM31; 1] = (*c11.try_into().unwrap()).unbox();
    let [c12]: [QM31; 1] = (*c12.try_into().unwrap()).unbox();
    let [c13]: [QM31; 1] = (*c13.try_into().unwrap()).unbox();
    let [c14]: [QM31; 1] = (*c14.try_into().unwrap()).unbox();
    let [c15]: [QM31; 1] = (*c15.try_into().unwrap()).unbox();
    let [c16]: [QM31; 1] = (*c16.try_into().unwrap()).unbox();
    let [c17]: [QM31; 1] = (*c17.try_into().unwrap()).unbox();
    let [c18]: [QM31; 1] = (*c18.try_into().unwrap()).unbox();
    let [c19]: [QM31; 1] = (*c19.try_into().unwrap()).unbox();
    let [c20]: [QM31; 1] = (*c20.try_into().unwrap()).unbox();
    let [c21]: [QM31; 1] = (*c21.try_into().unwrap()).unbox();
    let [c22]: [QM31; 1] = (*c22.try_into().unwrap()).unbox();
    let [c23]: [QM31; 1] = (*c23.try_into().unwrap()).unbox();
    let [c24]: [QM31; 1] = (*c24.try_into().unwrap()).unbox();
    let [c25]: [QM31; 1] = (*c25.try_into().unwrap()).unbox();
    let [c26]: [QM31; 1] = (*c26.try_into().unwrap()).unbox();
    let [c27]: [QM31; 1] = (*c27.try_into().unwrap()).unbox();
    // Last batch (7) is a cumulative sum across rows — we need both `_neg1` and current.
    let [c28_neg1, c28]: [QM31; 2] = (*c28.try_into().unwrap()).unbox();
    let [c29_neg1, c29]: [QM31; 2] = (*c29.try_into().unwrap()).unbox();
    let [c30_neg1, c30]: [QM31; 2] = (*c30.try_into().unwrap()).unbox();
    let [c31_neg1, c31]: [QM31; 2] = (*c31.try_into().unwrap()).unbox();

    core::internal::revoke_ap_tracking();

    // Reusable: build the QM31 from 4 base evals.
    // Batch 0 (lookups 0,1) — first non-last, no `prev` term.
    let cur_0 = QM31Impl::from_partial_evals([c0, c1, c2, c3]);
    let q = ((cur_0) * sum_0 * sum_1) + (sum_0 * num_1) + (sum_1 * num_0);
    sum = sum * random_coeff + q;

    // Batch 1 (lookups 2,3).
    let cur_1 = QM31Impl::from_partial_evals([c4, c5, c6, c7]);
    let q = ((cur_1 - cur_0) * sum_2 * sum_3) + (sum_2 * num_3) + (sum_3 * num_2);
    sum = sum * random_coeff + q;

    // Batch 2 (lookups 4,5).
    let cur_2 = QM31Impl::from_partial_evals([c8, c9, c10, c11]);
    let q = ((cur_2 - cur_1) * sum_4 * sum_5) + (sum_4 * num_5) + (sum_5 * num_4);
    sum = sum * random_coeff + q;

    // Batch 3 (lookups 6,7).
    let cur_3 = QM31Impl::from_partial_evals([c12, c13, c14, c15]);
    let q = ((cur_3 - cur_2) * sum_6 * sum_7) + (sum_6 * num_7) + (sum_7 * num_6);
    sum = sum * random_coeff + q;

    // Batch 4 (lookups 8,9).
    let cur_4 = QM31Impl::from_partial_evals([c16, c17, c18, c19]);
    let q = ((cur_4 - cur_3) * sum_8 * sum_9) + (sum_8 * num_9) + (sum_9 * num_8);
    sum = sum * random_coeff + q;

    // Batch 5 (lookups 10,11).
    let cur_5 = QM31Impl::from_partial_evals([c20, c21, c22, c23]);
    let q = ((cur_5 - cur_4) * sum_10 * sum_11) + (sum_10 * num_11) + (sum_11 * num_10);
    sum = sum * random_coeff + q;

    // Batch 6 (lookups 12,13).
    let cur_6 = QM31Impl::from_partial_evals([c24, c25, c26, c27]);
    let q = ((cur_6 - cur_5) * sum_12 * sum_13) + (sum_12 * num_13) + (sum_13 * num_12);
    sum = sum * random_coeff + q;

    // Batch 7 (lookups 14,15) — last; uses `_neg1` for the cumulative-row diff and adds
    // the `claimed_sum / N` shift.
    let cur_7 = QM31Impl::from_partial_evals([c28, c29, c30, c31]);
    let cur_7_neg1 = QM31Impl::from_partial_evals([c28_neg1, c29_neg1, c30_neg1, c31_neg1]);
    let q = ((cur_7 - cur_6 - cur_7_neg1 + (claimed_sum * (column_size.inverse().into())))
        * sum_14
        * sum_15)
        + (sum_14 * num_15)
        + (sum_15 * num_14);
    sum = sum * random_coeff + q;
}
