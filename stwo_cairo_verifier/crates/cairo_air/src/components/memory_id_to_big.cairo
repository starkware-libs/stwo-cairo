use stwo_constraint_framework::{PreprocessedMaskValues, PreprocessedMaskValuesImpl};
use stwo_verifier_core::channel::Channel;
use stwo_verifier_core::fields::m31::P_U32;
use stwo_verifier_core::fields::qm31::{QM31, QM31Serde, QM31_EXTENSION_DEGREE};
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::{ArrayImpl, pow2};
use stwo_verifier_core::{ColumnSpan, TreeArray};
use crate::cairo_component::{CairoComponent, NewComponent};
use crate::claim::ClaimTrait;
use crate::prelude::*;
use crate::{RelationUsesDict, accumulate_relation_uses};
use super::super::utils::UsizeImpl;

mod constraints_big;

/// The smallest ID yielded by the id_to_big component.
pub const LARGE_MEMORY_VALUE_ID_BASE: u32 = 0x40000000; // 2^30.

pub const N_BITS_PER_FELT: usize = 9;

pub const MEMORY_ID_SIZE: usize = 1;

pub const N_M31_IN_FELT252: usize = 28;

pub const N_MULTIPLICITY_COLUMNS: usize = 1;

pub const BIG_MULTIPLICITY_COLUMN_OFFSET: usize = N_M31_IN_FELT252;

pub const BIG_N_COLUMNS: usize = N_M31_IN_FELT252 + N_MULTIPLICITY_COLUMNS;

pub const RELATION_USES_PER_ROW_BIG: [(felt252, u32); 8] = [
    ('RangeCheck_9_9', 2), ('RangeCheck_9_9_B', 2), ('RangeCheck_9_9_C', 2),
    ('RangeCheck_9_9_D', 2), ('RangeCheck_9_9_E', 2), ('RangeCheck_9_9_F', 2),
    ('RangeCheck_9_9_G', 1), ('RangeCheck_9_9_H', 1),
];

#[derive(Drop, Serde)]
pub struct Claim {
    pub big_log_sizes: Array<u32>,
}

pub impl ClaimImpl of ClaimTrait<Claim> {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let Claim { big_log_sizes } = self;

        let mut preprocessed_log_sizes = big_log_sizes.clone();

        let mut trace_log_sizes = array![];

        for big_log_size in big_log_sizes.span() {
            for _ in 0..BIG_N_COLUMNS {
                trace_log_sizes.append(*big_log_size);
            }
        }

        let mut interaction_log_sizes = array![];

        // A lookup for every pair of limbs, and a yield of the value.
        for big_log_size in big_log_sizes.span() {
            for _ in 0..(QM31_EXTENSION_DEGREE * ((N_M31_IN_FELT252.div_ceil(2) + 1).div_ceil(2))) {
                interaction_log_sizes.append(*big_log_size);
            }
        }

        array![preprocessed_log_sizes.span(), trace_log_sizes.span(), interaction_log_sizes.span()]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        for big_log_size in self.big_log_sizes.span() {
            channel.mix_u64((*big_log_size).into());
        }
    }

    fn accumulate_relation_uses(self: @Claim, ref relation_uses: RelationUsesDict) {
        for log_size in self.big_log_sizes.span() {
            accumulate_relation_uses(
                ref relation_uses, RELATION_USES_PER_ROW_BIG.span(), *log_size,
            );
        }
    }
}

#[derive(Drop, Serde)]
pub struct InteractionClaim {
    pub big_claimed_sums: Array<QM31>,
    pub claimed_sum: QM31,
}

#[generate_trait]
pub impl InteractionClaimImpl of InteractionClaimTrait {
    fn mix_into(self: @InteractionClaim, ref channel: Channel) {
        channel.mix_felts(self.big_claimed_sums.span());
    }
}


#[derive(Drop)]
pub struct BigComponent {
    log_n_rows: u32,
    offset: u32,
    claimed_sum: QM31,
    common_lookup_elements: CommonLookupElements,
}

impl CairoBigComponentImpl of CairoComponent<BigComponent> {
    fn evaluate_constraints_at_point(
        self: @BigComponent,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
    ) {
        let params = constraints_big::ConstraintParams {
            column_size: pow2(*self.log_n_rows).try_into().unwrap(),
            offset: (*self.offset).try_into().unwrap(),
            common_lookup_elements: self.common_lookup_elements.clone(),
            claimed_sum: *self.claimed_sum,
            seq: preprocessed_mask_values
                .get_and_mark_used(preprocessed_columns::seq_column_idx(*self.log_n_rows)),
        };

        constraints_big::evaluate_constraints_at_point(
            ref sum, ref trace_mask_values, ref interaction_trace_mask_values, params, random_coeff,
        );
    }
}

#[derive(Drop)]
pub struct Component {
    big_components: Array<BigComponent>,
}

pub impl NewComponentImpl of NewComponent<Component> {
    type Claim = Claim;
    type InteractionClaim = InteractionClaim;

    fn new(
        claim: @Claim,
        interaction_claim: @InteractionClaim,
        common_lookup_elements: @CommonLookupElements,
    ) -> Component {
        let big_log_sizes = claim.big_log_sizes;
        let big_claimed_sums = interaction_claim.big_claimed_sums;
        assert!(big_log_sizes.len() == big_claimed_sums.len());

        let mut big_components = array![];
        let mut offset: u32 = LARGE_MEMORY_VALUE_ID_BASE;
        for i in 0..big_log_sizes.len() {
            let log_size = big_log_sizes[i];
            let claimed_sum = big_claimed_sums[i];
            big_components
                .append(
                    BigComponent {
                        log_n_rows: *log_size,
                        offset: offset,
                        claimed_sum: *claimed_sum,
                        common_lookup_elements: common_lookup_elements.clone(),
                    },
                );
            offset = offset + pow2(*log_size);
        }
        // Check that IDs in (ID -> Value) do not overflow P.
        assert!(offset <= P_U32);

        Component { big_components }
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
    ) {
        for big_component in self.big_components.span() {
            big_component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }
    }
}
