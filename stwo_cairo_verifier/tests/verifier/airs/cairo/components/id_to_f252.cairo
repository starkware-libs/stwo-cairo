use stwo_cairo_verifier::channel::{Channel, ChannelImpl};
use stwo_cairo_verifier::circle::CirclePoint;
use stwo_cairo_verifier::fields::qm31::{QM31, QM31_EXTENSION_DEGREE};
use stwo_cairo_verifier::poly::circle::CanonicCosetImpl;
use stwo_cairo_verifier::utils::ArrayImpl;
use stwo_cairo_verifier::{ColumnArray, TreeArray};
use super::super::utils::UsizeImpl;

mod constraints_big;
mod constraints_small;

pub const N_BITS_PER_FELT: usize = 9;

pub const MEMORY_ID_SIZE: usize = 1;

pub const N_M31_IN_FELT252: usize = 28;

pub const N_M31_IN_SMALL_FELT252: usize = 8; // 72 bits.

pub const N_MULTIPLICITY_COLUMNS: usize = 1;

pub const BIG_N_ID_AND_VALUE_COLUMNS: usize = MEMORY_ID_SIZE + N_M31_IN_FELT252;

pub const BIG_MULTIPLICITY_COLUMN_OFFSET: usize = BIG_N_ID_AND_VALUE_COLUMNS;

pub const BIG_N_COLUMNS: usize = BIG_N_ID_AND_VALUE_COLUMNS + N_MULTIPLICITY_COLUMNS;

pub const SMALL_MULTIPLICITY_COLUMN_OFFSET: usize = SMALL_N_ID_AND_VALUE_COLUMNS;

pub const SMALL_N_COLUMNS: usize = SMALL_N_ID_AND_VALUE_COLUMNS + N_MULTIPLICITY_COLUMNS;

pub const SMALL_N_ID_AND_VALUE_COLUMNS: usize = MEMORY_ID_SIZE + N_M31_IN_SMALL_FELT252;


#[derive(Drop, Serde, Copy)]
pub struct Claim {
    pub big_log_size: u32,
    pub small_log_size: u32,
}

#[generate_trait]
pub impl ClaimImpl of ClaimTrait {
    fn log_sizes(self: @Claim) -> TreeArray<Span<u32>> {
        let Claim { big_log_size, small_log_size } = *self;

        let preprocessed_log_sizes = array![big_log_size, small_log_size].span();

        let mut trace_log_sizes = array![];

        for _ in 0..BIG_N_COLUMNS {
            trace_log_sizes.append(big_log_size);
        };

        for _ in 0..SMALL_N_COLUMNS {
            trace_log_sizes.append(small_log_size);
        };

        let mut interaction_log_sizes = array![];

        // A lookup for every pair of limbs, and a yield of the value.
        for _ in 0..(QM31_EXTENSION_DEGREE * (N_M31_IN_FELT252.div_ceil(2) + 1)) {
            interaction_log_sizes.append(big_log_size);
        };

        for _ in 0..(QM31_EXTENSION_DEGREE * (N_M31_IN_SMALL_FELT252.div_ceil(2) + 1)) {
            interaction_log_sizes.append(small_log_size);
        };

        array![preprocessed_log_sizes, trace_log_sizes.span(), interaction_log_sizes.span()]
    }

    fn mix_into(self: @Claim, ref channel: Channel) {
        channel.mix_nonce((*self.big_log_size).into());
        channel.mix_nonce((*self.small_log_size).into());
    }
}

#[derive(Copy, Drop, Serde)]
pub struct InteractionClaim {
    pub big_claimed_sum: QM31,
    pub small_claimed_sum: QM31,
}

#[generate_trait]
pub impl InteractionClaimImpl of InteractionClaimTrait {
    fn mix_into(self: @InteractionClaim, ref channel: Channel) {
        channel.mix_felts([*self.big_claimed_sum].span());
        channel.mix_felts([*self.small_claimed_sum].span());
    }
}

#[derive(Drop)]
pub struct BigComponent {
    pub log_n_rows: u32,
    pub interaction_claim: InteractionClaim,
    pub lookup_elements: super::super::AddrToIdElements,
    pub range9_9_lookup_elements: super::super::RangeCheck9Bit9BitElements,
}

#[generate_trait]
pub impl BigComponentImpl of BigComponentTrait {
    fn mask_points(
        self: @BigComponent,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let trace_gen = CanonicCosetImpl::new(*self.log_n_rows).coset.step_size;
        constraints_big::mask_points(
            ref trace_mask_points, ref interaction_trace_mask_points, point, trace_gen,
        );
    }

    fn max_constraint_log_degree_bound(self: @BigComponent) -> u32 {
        *self.log_n_rows + 1
    }
}


#[derive(Drop)]
pub struct SmallComponent {
    pub log_n_rows: u32,
    pub interaction_claim: InteractionClaim,
    pub lookup_elements: super::super::AddrToIdElements,
    pub range9_9_lookup_elements: super::super::RangeCheck9Bit9BitElements,
}

#[generate_trait]
pub impl SmallComponentImpl of SmallComponentTrait {
    fn mask_points(
        self: @SmallComponent,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let trace_gen = CanonicCosetImpl::new(*self.log_n_rows).coset.step_size;
        constraints_small::mask_points(
            ref trace_mask_points, ref interaction_trace_mask_points, point, trace_gen,
        );
    }

    fn max_constraint_log_degree_bound(self: @SmallComponent) -> u32 {
        *self.log_n_rows + 1
    }
}
