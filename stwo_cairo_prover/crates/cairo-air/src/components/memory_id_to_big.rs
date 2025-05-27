use itertools::{chain, Itertools};
use num_traits::One;
use serde::{Deserialize, Serialize};
use starknet_ff::FieldElement;
use stwo_cairo_adapter::memory::LARGE_MEMORY_VALUE_ID_BASE;
use stwo_cairo_common::memory::{N_M31_IN_FELT252, N_M31_IN_SMALL_FELT252};
use stwo_cairo_serialize::CairoSerialize;
use stwo_prover::constraint_framework::{
    EvalAtRow, FrameworkComponent, FrameworkEval, RelationEntry, TraceLocationAllocator,
};
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::fields::secure_column::SECURE_EXTENSION_DEGREE;
use stwo_prover::core::pcs::TreeVec;
use stwo_prover::relation;

use super::prelude::RelationUse;
use crate::preprocessed::{PreProcessedColumn, Seq};
use crate::relations;

// TODO(AlonH): Make memory size configurable.
pub const MEMORY_ID_SIZE: usize = 1;
pub const N_MULTIPLICITY_COLUMNS: usize = 1;
pub const BIG_N_COLUMNS: usize = N_M31_IN_FELT252 + N_MULTIPLICITY_COLUMNS;
pub const SMALL_N_COLUMNS: usize = N_M31_IN_SMALL_FELT252 + N_MULTIPLICITY_COLUMNS;

pub type BigComponent = FrameworkComponent<BigEval>;
pub type SmallComponent = FrameworkComponent<SmallEval>;

const N_LOGUP_POWERS: usize = MEMORY_ID_SIZE + N_M31_IN_FELT252;

pub const RELATION_USES_PER_ROW_BIG: [RelationUse; 1] = [RelationUse {
    relation_id: "RangeCheck_9_9",
    uses: 14,
}];

pub const RELATION_USES_PER_ROW_SMALL: [RelationUse; 1] = [RelationUse {
    relation_id: "RangeCheck_9_9",
    uses: 4,
}];

relation!(RelationElements, N_LOGUP_POWERS);

/// IDs are continuous and start from 0.
/// Values are Felt252 stored as `N_M31_IN_FELT252` M31 values (each value containing 9 bits).
#[derive(Clone)]
pub struct BigEval {
    pub log_n_rows: u32,
    // Internal offset of the ids when there are multiple components.
    pub offset: u32,
    pub lookup_elements: relations::MemoryIdToBig,
    pub range9_9_lookup_elements: relations::RangeCheck_9_9,
}
impl BigEval {
    pub fn new(
        log_n_rows: u32,
        offset: u32,
        lookup_elements: relations::MemoryIdToBig,
        range9_9_lookup_elements: relations::RangeCheck_9_9,
    ) -> Self {
        Self {
            log_n_rows,
            offset,
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
        let seq = eval.get_preprocessed_column(Seq::new(self.log_size()).id());
        let value: [E::F; N_M31_IN_FELT252] = std::array::from_fn(|_| eval.next_trace_mask());
        let multiplicity = eval.next_trace_mask();

        // Range check limbs.
        for (l, r) in value.iter().tuples() {
            eval.add_to_relation(RelationEntry::new(
                &self.range9_9_lookup_elements,
                E::EF::one(),
                &[l.clone(), r.clone()],
            ));
        }

        // Yield the value.
        let id = seq
            + E::F::from(M31::from(LARGE_MEMORY_VALUE_ID_BASE))
            + E::F::from(M31::from(self.offset));
        eval.add_to_relation(RelationEntry::new(
            &self.lookup_elements,
            E::EF::from(-multiplicity),
            &chain!([id], value).collect_vec(),
        ));

        eval.finalize_logup_in_pairs();
        eval
    }
}

pub fn big_components_from_claim(
    log_sizes: &[u32],
    claimed_sums: &[SecureField],
    lookup_elements: &relations::MemoryIdToBig,
    range9_9_lookup_elements: &relations::RangeCheck_9_9,
    tree_span_provider: &mut TraceLocationAllocator,
) -> Vec<BigComponent> {
    // Every component is responsible for a range of memory ids. The ids must not overlap. Use an
    // offset to keep track of the previous component's range.
    let mut components = vec![];
    let mut offset = 0;
    for (&log_size, &claimed_sum) in log_sizes.iter().zip_eq(claimed_sums) {
        components.push(BigComponent::new(
            tree_span_provider,
            BigEval::new(
                log_size,
                offset,
                lookup_elements.clone(),
                range9_9_lookup_elements.clone(),
            ),
            claimed_sum,
        ));
        offset += 1 << log_size;
    }
    components
}

pub struct SmallEval {
    pub log_n_rows: u32,
    pub lookup_elements: relations::MemoryIdToBig,
    pub range_check_9_9_relation: relations::RangeCheck_9_9,
}
impl SmallEval {
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
        let seq = eval.get_preprocessed_column(Seq::new(self.log_size()).id());
        let value: [E::F; N_M31_IN_SMALL_FELT252] = std::array::from_fn(|_| eval.next_trace_mask());
        let multiplicity = eval.next_trace_mask();

        // Range check limbs.
        for (l, r) in value.iter().tuples() {
            eval.add_to_relation(RelationEntry::new(
                &self.range_check_9_9_relation,
                E::EF::one(),
                &[l.clone(), r.clone()],
            ));
        }

        // Yield the value.
        let id = seq;
        eval.add_to_relation(RelationEntry::new(
            &self.lookup_elements,
            E::EF::from(-multiplicity),
            &chain!([id], value).collect_vec(),
        ));

        eval.finalize_logup();
        eval
    }
}

#[derive(Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Claim {
    pub big_log_sizes: Vec<u32>,
    pub small_log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        // Original trace.
        let big_trace_log_sizes = self
            .big_log_sizes
            .iter()
            .flat_map(|&log_size| vec![log_size; BIG_N_COLUMNS]);
        let small_trace_log_sizes = vec![self.small_log_size; SMALL_N_COLUMNS];
        let trace_log_sizes = chain!(big_trace_log_sizes, small_trace_log_sizes).collect_vec();

        // Interaction trace.
        let big_interaction_log_sizes = self.big_log_sizes.iter().flat_map(|&log_size| {
            // A range-check for every pair of limbs, batched in pairs.
            // And a yield of the value.
            vec![
                log_size;
                SECURE_EXTENSION_DEGREE * ((N_M31_IN_FELT252.div_ceil(2) + 1).div_ceil(2))
            ]
        });
        // TODO(Ohad): Batch.
        let small_interaction_log_sizes = vec![
            self.small_log_size;
            SECURE_EXTENSION_DEGREE
                * (N_M31_IN_SMALL_FELT252.div_ceil(2) + 1)
        ];
        let interaction_log_sizes =
            chain!(big_interaction_log_sizes, small_interaction_log_sizes).collect_vec();

        TreeVec::new(vec![vec![], trace_log_sizes, interaction_log_sizes])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        chain!(self.big_log_sizes.clone(), [self.small_log_size])
            .for_each(|log_size| channel.mix_u64(log_size as u64));
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct InteractionClaim {
    pub big_claimed_sums: Vec<SecureField>,
    pub small_claimed_sum: SecureField,
}
impl InteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_felts(&self.big_claimed_sums);
        channel.mix_felts(&[self.small_claimed_sum]);
    }

    pub fn claimed_sum(&self) -> SecureField {
        self.small_claimed_sum + self.big_claimed_sums.iter().sum::<SecureField>()
    }
}

impl CairoSerialize for InteractionClaim {
    fn serialize(&self, output: &mut Vec<FieldElement>) {
        let Self {
            big_claimed_sums,
            small_claimed_sum,
        } = self;
        CairoSerialize::serialize(big_claimed_sums, output);
        CairoSerialize::serialize(small_claimed_sum, output);
    }
}

#[cfg(test)]
mod tests {
    use num_traits::Zero;
    use rand::rngs::SmallRng;
    use rand::{Rng, SeedableRng};
    use stwo_prover::constraint_framework::expr::ExprEvaluator;
    use stwo_prover::core::fields::qm31::QM31;

    use super::*;
    use crate::components::constraints_regression_test_values::{
        BIG_MEMORY_ID_TO_BIG, SMALL_MEMORY_ID_TO_BIG,
    };

    #[test]
    fn memory_id_to_big_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let big_eval = BigEval {
            log_n_rows: 4,
            offset: 0,
            lookup_elements: relations::MemoryIdToBig::dummy(),
            range9_9_lookup_elements: relations::RangeCheck_9_9::dummy(),
        };
        let small_eval = SmallEval {
            log_n_rows: 4,
            lookup_elements: relations::MemoryIdToBig::dummy(),
            range_check_9_9_relation: relations::RangeCheck_9_9::dummy(),
        };

        let big_expr_eval = big_eval.evaluate(ExprEvaluator::new());
        let small_expr_eval = small_eval.evaluate(ExprEvaluator::new());
        let big_assignment = big_expr_eval.random_assignment();
        let small_assignment = small_expr_eval.random_assignment();

        let mut big_sum = QM31::zero();
        let mut small_sum = QM31::zero();
        for c in big_expr_eval.constraints {
            big_sum += c.assign(&big_assignment) * rng.gen::<QM31>();
        }
        for c in small_expr_eval.constraints {
            small_sum += c.assign(&small_assignment) * rng.gen::<QM31>();
        }

        assert_eq!(big_sum, BIG_MEMORY_ID_TO_BIG);
        assert_eq!(small_sum, SMALL_MEMORY_ID_TO_BIG);
    }
}
