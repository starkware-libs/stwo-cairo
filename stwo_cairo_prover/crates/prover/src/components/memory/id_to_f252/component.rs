use itertools::{chain, Itertools};
use num_traits::One;
use serde::{Deserialize, Serialize};
use stwo_prover::constraint_framework::logup::{LogupAtRow, LookupElements};
use stwo_prover::constraint_framework::preprocessed_columns::PreprocessedColumn;
use stwo_prover::constraint_framework::{
    EvalAtRow, FrameworkComponent, FrameworkEval, INTERACTION_TRACE_IDX,
};
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::qm31::{SecureField, QM31};
use stwo_prover::core::fields::secure_column::SECURE_EXTENSION_DEGREE;
use stwo_prover::core::lookups::utils::Fraction;
use stwo_prover::core::pcs::TreeVec;

use crate::components::range_check_vector::range_check_9_9;

// TODO(AlonH): Make memory size configurable.
pub const MEMORY_ID_SIZE: usize = 1;
pub const N_M31_IN_FELT252: usize = 28;
pub const N_M31_IN_SMALL_FELT252: usize = 8; // 72 bits.
pub const N_MULTIPLICITY_COLUMNS: usize = 1;
pub const BIG_MULTIPLICITY_COLUMN_OFFSET: usize = BIG_N_ID_AND_VALUE_COLUMNS;
pub const BIG_N_COLUMNS: usize = BIG_N_ID_AND_VALUE_COLUMNS + N_MULTIPLICITY_COLUMNS;
pub const BIG_N_ID_AND_VALUE_COLUMNS: usize = MEMORY_ID_SIZE + N_M31_IN_FELT252;
pub const SMALL_MULTIPLICITY_COLUMN_OFFSET: usize = SMALL_N_ID_AND_VALUE_COLUMNS;
pub const SMALL_N_COLUMNS: usize = SMALL_N_ID_AND_VALUE_COLUMNS + N_MULTIPLICITY_COLUMNS;
pub const SMALL_N_ID_AND_VALUE_COLUMNS: usize = MEMORY_ID_SIZE + N_M31_IN_SMALL_FELT252;

pub type BigComponent = FrameworkComponent<BigEval>;
pub type SmallComponent = FrameworkComponent<SmallEval>;

const N_LOGUP_POWERS: usize = MEMORY_ID_SIZE + N_M31_IN_FELT252;
pub type RelationElements = LookupElements<N_LOGUP_POWERS>;

/// IDs are continuous and start from 0.
/// Values are Felt252 stored as `N_M31_IN_FELT252` M31 values (each value containing 9 bits).
#[derive(Clone)]
pub struct BigEval {
    pub log_n_rows: u32,
    pub lookup_elements: RelationElements,
    pub range9_9_lookup_elements: range_check_9_9::RelationElements,
    pub claimed_sum: QM31,
}
impl BigEval {
    pub const fn n_columns(&self) -> usize {
        BIG_N_COLUMNS
    }
    pub fn new(
        claim: Claim,
        lookup_elements: RelationElements,
        range9_9_lookup_elements: range_check_9_9::RelationElements,
        interaction_claim: InteractionClaim,
    ) -> Self {
        Self {
            log_n_rows: claim.big_log_size,
            lookup_elements,
            range9_9_lookup_elements,
            claimed_sum: interaction_claim.big_claimed_sum,
        }
    }
}

impl FrameworkEval for BigEval {
    fn log_size(&self) -> u32 {
        self.log_n_rows
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_size() + 1
    }

    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let is_first = eval.get_preprocessed_column(PreprocessedColumn::IsFirst(self.log_size()));
        let mut logup =
            LogupAtRow::<E>::new(INTERACTION_TRACE_IDX, self.claimed_sum, None, is_first);

        let id_and_value: [E::F; MEMORY_ID_SIZE + N_M31_IN_FELT252] =
            std::array::from_fn(|_| eval.next_trace_mask());
        let multiplicity = eval.next_trace_mask();
        let frac = Fraction::new(
            E::EF::from(-multiplicity),
            self.lookup_elements.combine(&id_and_value),
        );
        logup.write_frac(&mut eval, frac);

        // Range check elements.
        for (l, r) in id_and_value[MEMORY_ID_SIZE..].iter().tuples() {
            let frac = Fraction::new(
                E::EF::one(),
                self.range9_9_lookup_elements
                    .combine(&[l.clone(), r.clone()]),
            );
            logup.write_frac(&mut eval, frac);
        }

        logup.finalize(&mut eval);

        eval
    }
}

pub struct SmallEval {
    pub log_n_rows: u32,
    pub lookup_elements: RelationElements,
    pub claimed_sum: QM31,
}
impl SmallEval {
    pub const fn n_columns(&self) -> usize {
        SMALL_N_COLUMNS
    }
    pub fn new(
        claim: Claim,
        lookup_elements: RelationElements,
        interaction_claim: InteractionClaim,
    ) -> Self {
        Self {
            log_n_rows: claim.small_log_size,
            lookup_elements,
            claimed_sum: interaction_claim.small_claimed_sum,
        }
    }
}
impl FrameworkEval for SmallEval {
    fn log_size(&self) -> u32 {
        self.log_n_rows
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_size() + 1
    }

    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let is_first = eval.get_preprocessed_column(PreprocessedColumn::IsFirst(self.log_size()));
        let mut logup =
            LogupAtRow::<E>::new(INTERACTION_TRACE_IDX, self.claimed_sum, None, is_first);

        let id_and_value: [E::F; SMALL_N_ID_AND_VALUE_COLUMNS] =
            std::array::from_fn(|_| eval.next_trace_mask());
        let multiplicity = eval.next_trace_mask();
        let frac = Fraction::new(
            E::EF::from(-multiplicity),
            self.lookup_elements.combine(&id_and_value),
        );
        logup.write_frac(&mut eval, frac);

        logup.finalize(&mut eval);

        eval
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Claim {
    pub big_log_size: u32,
    pub small_log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let preprocessed_log_sizes = vec![self.big_log_size, self.small_log_size];
        let trace_log_sizes = chain!(
            vec![self.big_log_size; BIG_N_COLUMNS],
            vec![self.small_log_size; SMALL_N_COLUMNS]
        )
        .collect();
        let interaction_log_sizes = chain!(
            // A lookup for every pair of limbs, and a yield of the value.
            vec![self.big_log_size; SECURE_EXTENSION_DEGREE * (N_M31_IN_FELT252.div_ceil(2) + 1)],
            vec![self.small_log_size; SECURE_EXTENSION_DEGREE]
        )
        .collect();

        TreeVec::new(vec![
            preprocessed_log_sizes,
            trace_log_sizes,
            interaction_log_sizes,
        ])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(self.big_log_size as u64);
        channel.mix_u64(self.small_log_size as u64);
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct InteractionClaim {
    pub big_claimed_sum: SecureField,
    pub small_claimed_sum: SecureField,
}
impl InteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_felts(&[self.big_claimed_sum]);
        channel.mix_felts(&[self.small_claimed_sum]);
    }
}
