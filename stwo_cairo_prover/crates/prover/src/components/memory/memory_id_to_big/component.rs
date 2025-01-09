use itertools::{chain, Itertools};
use num_traits::One;
use serde::{Deserialize, Serialize};
use starknet_ff::FieldElement;
use stwo_cairo_serialize::CairoSerialize;
use stwo_prover::constraint_framework::{
    EvalAtRow, FrameworkComponent, FrameworkEval, RelationEntry,
};
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::fields::secure_column::SECURE_EXTENSION_DEGREE;
use stwo_prover::core::pcs::TreeVec;
use stwo_prover::relation;

use crate::relations;

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

relation!(RelationElements, N_LOGUP_POWERS);

/// IDs are continuous and start from 0.
/// Values are Felt252 stored as `N_M31_IN_FELT252` M31 values (each value containing 9 bits).
#[derive(Clone)]
pub struct BigEval {
    pub log_n_rows: u32,
    pub lookup_elements: relations::MemoryIdToBig,
    pub range9_9_lookup_elements: relations::RangeCheck_9_9,
}
impl BigEval {
    pub const fn n_columns(&self) -> usize {
        BIG_N_COLUMNS
    }
    pub fn new(
        claim: Claim,
        lookup_elements: relations::MemoryIdToBig,
        range9_9_lookup_elements: relations::RangeCheck_9_9,
    ) -> Self {
        Self {
            log_n_rows: claim.big_log_size,
            lookup_elements,
            range9_9_lookup_elements,
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
        let id_and_value: [E::F; MEMORY_ID_SIZE + N_M31_IN_FELT252] =
            std::array::from_fn(|_| eval.next_trace_mask());
        let multiplicity = eval.next_trace_mask();

        // Range check limbs.
        for (l, r) in id_and_value[MEMORY_ID_SIZE..].iter().tuples() {
            eval.add_to_relation(RelationEntry::new(
                &self.range9_9_lookup_elements,
                E::EF::one(),
                &[l.clone(), r.clone()],
            ));
        }

        // Yield the value.
        eval.add_to_relation(RelationEntry::new(
            &self.lookup_elements,
            E::EF::from(-multiplicity),
            &id_and_value,
        ));

        eval.finalize_logup_in_pairs();
        eval
    }
}

pub struct SmallEval {
    pub log_n_rows: u32,
    pub lookup_elements: relations::MemoryIdToBig,
    pub range_check_9_9_relation: relations::RangeCheck_9_9,
}
impl SmallEval {
    pub const fn n_columns(&self) -> usize {
        SMALL_N_COLUMNS
    }
    pub fn new(
        claim: Claim,
        lookup_elements: relations::MemoryIdToBig,
        range_check_9_9_relation: relations::RangeCheck_9_9,
    ) -> Self {
        Self {
            log_n_rows: claim.small_log_size,
            lookup_elements,
            range_check_9_9_relation,
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
        let id_and_value: [E::F; SMALL_N_ID_AND_VALUE_COLUMNS] =
            std::array::from_fn(|_| eval.next_trace_mask());
        let multiplicity = eval.next_trace_mask();

        // Range check limbs.
        for (l, r) in id_and_value[MEMORY_ID_SIZE..].iter().tuples() {
            eval.add_to_relation(RelationEntry::new(
                &self.range_check_9_9_relation,
                E::EF::one(),
                &[l.clone(), r.clone()],
            ));
        }

        // Yield the value.
        eval.add_to_relation(RelationEntry::new(
            &self.lookup_elements,
            E::EF::from(-multiplicity),
            &id_and_value,
        ));

        eval.finalize_logup();
        eval
    }
}

#[derive(Clone, Serialize, Deserialize, CairoSerialize)]
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
            // A range-check for every pair of limbs, batched in pairs.
            // And a yield of the value.
            vec![
                self.big_log_size;
                SECURE_EXTENSION_DEGREE * ((N_M31_IN_FELT252.div_ceil(2) + 1).div_ceil(2))
            ],
            // Not batched range-check.
            // TODO(Ohad): Batch.
            vec![
                self.small_log_size;
                SECURE_EXTENSION_DEGREE * (N_M31_IN_SMALL_FELT252.div_ceil(2) + 1)
            ]
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

impl CairoSerialize for InteractionClaim {
    fn serialize(&self, output: &mut Vec<FieldElement>) {
        let Self {
            big_claimed_sum,
            small_claimed_sum,
        } = self;
        CairoSerialize::serialize(big_claimed_sum, output);
        CairoSerialize::serialize(small_claimed_sum, output);
    }
}
