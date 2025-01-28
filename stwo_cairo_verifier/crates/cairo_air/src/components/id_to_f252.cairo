use crate::components::CairoComponent;
use stwo_constraint_framework::{
    PreprocessedColumn, PreprocessedColumnSet, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::channel::{Channel, ChannelImpl};
use stwo_verifier_core::circle::CirclePoint;
use stwo_verifier_core::fields::qm31::{QM31, QM31Zero, QM31_EXTENSION_DEGREE};
use stwo_verifier_core::poly::circle::CanonicCosetImpl;
use stwo_verifier_core::utils::ArrayImpl;
use stwo_verifier_core::{ColumnArray, ColumnSpan, TreeArray};
use super::super::Invertible;
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
        for _ in 0..(QM31_EXTENSION_DEGREE * ((N_M31_IN_FELT252.div_ceil(2) + 1).div_ceil(2))) {
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
    pub lookup_elements: super::super::IdToValueElements,
    pub range_9_9_lookup_elements: super::super::RangeCheck9Bit9BitElements,
}

pub impl BigComponentImpl of CairoComponent<BigComponent> {
    fn mask_points(
        self: @BigComponent,
        ref preprocessed_column_set: PreprocessedColumnSet,
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

    fn evaluate_constraints_at_point(
        self: @BigComponent,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Array<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Array<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        let mut id_to_value_alpha_powers = self.lookup_elements.alpha_powers.span();
        let id_to_value_alpha_0 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_1 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_2 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_3 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_4 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_5 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_6 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_7 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_8 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_9 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_10 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_11 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_12 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_13 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_14 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_15 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_16 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_17 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_18 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_19 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_20 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_21 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_22 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_23 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_24 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_25 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_26 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_27 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_28 = *id_to_value_alpha_powers.pop_front().unwrap();

        let mut range_check_9_9_alpha_powers = self.range_9_9_lookup_elements.alpha_powers.span();
        let range_check_9_9_alpha_0 = *range_check_9_9_alpha_powers.pop_front().unwrap();
        let range_check_9_9_alpha_1 = *range_check_9_9_alpha_powers.pop_front().unwrap();

        let params = constraints_big::ConstraintParams {
            MemoryIdToBig_alpha0: id_to_value_alpha_0,
            MemoryIdToBig_alpha1: id_to_value_alpha_1,
            MemoryIdToBig_alpha10: id_to_value_alpha_10,
            MemoryIdToBig_alpha11: id_to_value_alpha_11,
            MemoryIdToBig_alpha12: id_to_value_alpha_12,
            MemoryIdToBig_alpha13: id_to_value_alpha_13,
            MemoryIdToBig_alpha14: id_to_value_alpha_14,
            MemoryIdToBig_alpha15: id_to_value_alpha_15,
            MemoryIdToBig_alpha16: id_to_value_alpha_16,
            MemoryIdToBig_alpha17: id_to_value_alpha_17,
            MemoryIdToBig_alpha18: id_to_value_alpha_18,
            MemoryIdToBig_alpha19: id_to_value_alpha_19,
            MemoryIdToBig_alpha2: id_to_value_alpha_2,
            MemoryIdToBig_alpha20: id_to_value_alpha_20,
            MemoryIdToBig_alpha21: id_to_value_alpha_21,
            MemoryIdToBig_alpha22: id_to_value_alpha_22,
            MemoryIdToBig_alpha23: id_to_value_alpha_23,
            MemoryIdToBig_alpha24: id_to_value_alpha_24,
            MemoryIdToBig_alpha25: id_to_value_alpha_25,
            MemoryIdToBig_alpha26: id_to_value_alpha_26,
            MemoryIdToBig_alpha27: id_to_value_alpha_27,
            MemoryIdToBig_alpha28: id_to_value_alpha_28,
            MemoryIdToBig_alpha3: id_to_value_alpha_3,
            MemoryIdToBig_alpha4: id_to_value_alpha_4,
            MemoryIdToBig_alpha5: id_to_value_alpha_5,
            MemoryIdToBig_alpha6: id_to_value_alpha_6,
            MemoryIdToBig_alpha7: id_to_value_alpha_7,
            MemoryIdToBig_alpha8: id_to_value_alpha_8,
            MemoryIdToBig_alpha9: id_to_value_alpha_9,
            MemoryIdToBig_z: *self.lookup_elements.z,
            RangeCheck_9_9_alpha0: range_check_9_9_alpha_0,
            RangeCheck_9_9_alpha1: range_check_9_9_alpha_1,
            RangeCheck_9_9_z: *self.range_9_9_lookup_elements.z,
            preprocessed_is_first: preprocessed_mask_values
                .get(PreprocessedColumn::IsFirst(*self.log_n_rows)),
            total_sum: *self.interaction_claim.big_claimed_sum,
        };

        let trace_domain = CanonicCosetImpl::new(*self.log_n_rows);
        let vanish_eval = trace_domain.eval_vanishing(point);

        constraints_big::evaluate_constraints_at_point(
            ref sum,
            ref trace_mask_values,
            ref interaction_trace_mask_values,
            params,
            random_coeff,
            vanish_eval.inverse(),
        );
    }
}


#[derive(Drop)]
pub struct SmallComponent {
    pub log_n_rows: u32,
    pub interaction_claim: InteractionClaim,
    pub lookup_elements: super::super::IdToValueElements,
    pub range_9_9_lookup_elements: super::super::RangeCheck9Bit9BitElements,
}

pub impl SmallComponentImpl of CairoComponent<SmallComponent> {
    fn mask_points(
        self: @SmallComponent,
        ref preprocessed_column_set: PreprocessedColumnSet,
        ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
        point: CirclePoint<QM31>,
    ) {
        let log_size = *self.log_n_rows;
        let trace_gen = CanonicCosetImpl::new(log_size).coset.step_size;
        constraints_small::mask_points(
            ref preprocessed_column_set,
            ref trace_mask_points,
            ref interaction_trace_mask_points,
            point,
            trace_gen,
            log_size,
        );
    }

    fn max_constraint_log_degree_bound(self: @SmallComponent) -> u32 {
        *self.log_n_rows + 1
    }

    fn evaluate_constraints_at_point(
        self: @SmallComponent,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Array<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Array<QM31>>,
        random_coeff: QM31,
        point: CirclePoint<QM31>,
    ) {
        let mut id_to_value_alpha_powers = self.lookup_elements.alpha_powers.span();
        let id_to_value_alpha_0 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_1 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_2 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_3 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_4 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_5 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_6 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_7 = *id_to_value_alpha_powers.pop_front().unwrap();
        let id_to_value_alpha_8 = *id_to_value_alpha_powers.pop_front().unwrap();

        let mut range_check_9_9_alpha_powers = self.range_9_9_lookup_elements.alpha_powers.span();
        let range_check_9_9_alpha_0 = *range_check_9_9_alpha_powers.pop_front().unwrap();
        let range_check_9_9_alpha_1 = *range_check_9_9_alpha_powers.pop_front().unwrap();

        let params = constraints_small::ConstraintParams {
            MemoryIdToBig_alpha0: id_to_value_alpha_0,
            MemoryIdToBig_alpha1: id_to_value_alpha_1,
            MemoryIdToBig_alpha2: id_to_value_alpha_2,
            MemoryIdToBig_alpha3: id_to_value_alpha_3,
            MemoryIdToBig_alpha4: id_to_value_alpha_4,
            MemoryIdToBig_alpha5: id_to_value_alpha_5,
            MemoryIdToBig_alpha6: id_to_value_alpha_6,
            MemoryIdToBig_alpha7: id_to_value_alpha_7,
            MemoryIdToBig_alpha8: id_to_value_alpha_8,
            MemoryIdToBig_z: *self.lookup_elements.z,
            RangeCheck_9_9_alpha0: range_check_9_9_alpha_0,
            RangeCheck_9_9_alpha1: range_check_9_9_alpha_1,
            RangeCheck_9_9_z: *self.range_9_9_lookup_elements.z,
            preprocessed_is_first: preprocessed_mask_values
                .get(PreprocessedColumn::IsFirst(*self.log_n_rows)),
            total_sum: *self.interaction_claim.small_claimed_sum,
        };

        let trace_domain = CanonicCosetImpl::new(*self.log_n_rows);
        let vanish_eval = trace_domain.eval_vanishing(point);

        constraints_small::evaluate_constraints_at_point(
            ref sum,
            ref trace_mask_values,
            ref interaction_trace_mask_values,
            params,
            random_coeff,
            vanish_eval.inverse(),
        );
    }
}
