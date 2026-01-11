use core::num::traits::Zero;
use stwo_constraint_framework::{PreprocessedMaskValues, PreprocessedMaskValuesImpl};
use stwo_verifier_core::channel::Channel;
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::qm31::{QM31, QM31Serde, QM31_EXTENSION_DEGREE};
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::{ArrayImpl, pow2};
use stwo_verifier_core::{ColumnSpan, TreeArray};
use crate::cairo_component::CairoComponent;
use crate::claim::ClaimTrait;
use crate::prelude::*;
use crate::{RelationUsesDict, accumulate_relation_uses};
use super::super::utils::UsizeImpl;

mod constraints_big;
mod constraints_small;

/// The smallest ID yielded by the id_to_big component.
pub const LARGE_MEMORY_VALUE_ID_BASE: u32 = 0x40000000; // 2^30.

pub const N_BITS_PER_FELT: usize = 9;

pub const MEMORY_ID_SIZE: usize = 1;

pub const N_M31_IN_FELT252: usize = 28;

pub const N_M31_IN_SMALL_FELT252: usize = 8; // 72 bits.

pub const N_MULTIPLICITY_COLUMNS: usize = 1;

pub const BIG_MULTIPLICITY_COLUMN_OFFSET: usize = N_M31_IN_FELT252;

pub const BIG_N_COLUMNS: usize = N_M31_IN_FELT252 + N_MULTIPLICITY_COLUMNS;

pub const SMALL_MULTIPLICITY_COLUMN_OFFSET: usize = N_M31_IN_SMALL_FELT252;

pub const SMALL_N_COLUMNS: usize = N_M31_IN_SMALL_FELT252 + N_MULTIPLICITY_COLUMNS;

pub const RELATION_USES_PER_ROW_BIG: [(felt252, u32); 8] = [
    ('RangeCheck_9_9', 2), ('RangeCheck_9_9_B', 2), ('RangeCheck_9_9_C', 2),
    ('RangeCheck_9_9_D', 2), ('RangeCheck_9_9_E', 2), ('RangeCheck_9_9_F', 2),
    ('RangeCheck_9_9_G', 1), ('RangeCheck_9_9_H', 1),
];
pub const RELATION_USES_PER_ROW_SMALL: [(felt252, u32); 1] = [('RangeCheck_9_9', 4)];

#[derive(Drop, Serde)]
pub struct Claim {
    pub big_log_sizes: Array<u32>,
    pub small_log_size: u32,
}

pub impl ClaimImpl of ClaimTrait<Claim> {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let Claim { big_log_sizes, small_log_size } = self;

        let mut preprocessed_log_sizes = big_log_sizes.clone();
        preprocessed_log_sizes.append(*small_log_size);

        let mut trace_log_sizes = array![];

        for big_log_size in big_log_sizes.span() {
            for _ in 0..BIG_N_COLUMNS {
                trace_log_sizes.append(*big_log_size);
            }
        }

        for _ in 0..SMALL_N_COLUMNS {
            trace_log_sizes.append(*small_log_size);
        }

        let mut interaction_log_sizes = array![];

        // A lookup for every pair of limbs, and a yield of the value.
        for big_log_size in big_log_sizes.span() {
            for _ in 0..(QM31_EXTENSION_DEGREE * ((N_M31_IN_FELT252.div_ceil(2) + 1).div_ceil(2))) {
                interaction_log_sizes.append(*big_log_size);
            }
        }

        for _ in 0..(QM31_EXTENSION_DEGREE * (N_M31_IN_SMALL_FELT252.div_ceil(2) + 1).div_ceil(2)) {
            interaction_log_sizes.append(*small_log_size);
        }

        array![preprocessed_log_sizes.span(), trace_log_sizes.span(), interaction_log_sizes.span()]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        for big_log_size in self.big_log_sizes.span() {
            channel.mix_u64((*big_log_size).into());
        }
        channel.mix_u64((*self.small_log_size).into());
    }

    fn accumulate_relation_uses(self: @Claim, ref relation_uses: RelationUsesDict) {
        for log_size in self.big_log_sizes.span() {
            accumulate_relation_uses(
                ref relation_uses, RELATION_USES_PER_ROW_BIG.span(), *log_size,
            );
        }
        accumulate_relation_uses(
            ref relation_uses, RELATION_USES_PER_ROW_SMALL.span(), *self.small_log_size,
        );
    }
}

#[derive(Drop, Serde)]
pub struct InteractionClaim {
    pub big_claimed_sums: Array<QM31>,
    pub small_claimed_sum: QM31,
}

#[generate_trait]
pub impl InteractionClaimImpl of InteractionClaimTrait {
    fn mix_into(self: @InteractionClaim, ref channel: Channel) {
        channel.mix_felts(self.big_claimed_sums.span());
        channel.mix_felts([*self.small_claimed_sum].span());
    }

    fn sum(self: @InteractionClaim) -> QM31 {
        let mut sum = Zero::zero();
        for big_claimed_sum in self.big_claimed_sums.span() {
            sum += *big_claimed_sum;
        }
        sum += *self.small_claimed_sum;
        sum
    }
}


#[derive(Drop)]
pub struct BigComponent {
    pub log_n_rows: u32,
    pub offset: u32,
    pub claimed_sum: QM31,
    pub common_lookup_elements: CommonLookupElements,
}

#[generate_trait]
pub impl NewBigComponentImpl of NewBigComponent {
    fn new(
        log_n_rows: u32,
        offset: u32,
        claimed_sum: QM31,
        common_lookup_elements: @CommonLookupElements,
    ) -> BigComponent {
        BigComponent {
            log_n_rows: log_n_rows,
            offset: offset,
            claimed_sum: claimed_sum,
            common_lookup_elements: common_lookup_elements.clone(),
        }
    }
}

pub impl CairoBigComponentImpl of CairoComponent<BigComponent> {
    fn evaluate_constraints_at_point(
        self: @BigComponent,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
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
pub struct SmallComponent {
    pub log_n_rows: u32,
    pub claimed_sum: QM31,
    pub common_lookup_elements: CommonLookupElements,
}

#[generate_trait]
pub impl NewSmallComponentImpl of NewSmallComponent {
    fn new(
        log_n_rows: u32, claimed_sum: QM31, common_lookup_elements: @CommonLookupElements,
    ) -> SmallComponent {
        SmallComponent {
            log_n_rows: log_n_rows,
            claimed_sum: claimed_sum,
            common_lookup_elements: common_lookup_elements.clone(),
        }
    }
}

pub impl CairoSmallComponentImpl of CairoComponent<SmallComponent> {
    fn evaluate_constraints_at_point(
        self: @SmallComponent,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        let params = constraints_small::ConstraintParams {
            column_size: pow2(*self.log_n_rows).try_into().unwrap(),
            common_lookup_elements: self.common_lookup_elements.clone(),
            claimed_sum: *self.claimed_sum,
            seq: preprocessed_mask_values
                .get_and_mark_used(preprocessed_columns::seq_column_idx(*self.log_n_rows)),
        };

        constraints_small::evaluate_constraints_at_point(
            ref sum, ref trace_mask_values, ref interaction_trace_mask_values, params, random_coeff,
        );
    }
}
