use itertools::Itertools;
use serde::{Deserialize, Serialize};
use stwo::core::channel::Channel;
use stwo::core::fields::m31::M31;
use stwo::core::fields::qm31::{SecureField, SECURE_EXTENSION_DEGREE};
use stwo::core::pcs::TreeVec;
use stwo_cairo_common::memory::LARGE_MEMORY_VALUE_ID_BASE;
use stwo_cairo_common::preprocessed_columns::preprocessed_trace::{PreProcessedColumn, Seq};
use stwo_cairo_common::prover_types::cpu::FELT252_N_WORDS;
use stwo_cairo_serialize::{CairoDeserialize, CairoSerialize};
use stwo_constraint_framework::{
    relation, EvalAtRow, FrameworkComponent, FrameworkEval, RelationEntry, TraceLocationAllocator,
};

use super::prelude::RelationUse;
use crate::components::subroutines::range_check_mem_value_n_28::RangeCheckMemValueN28;
use crate::relations;

// TODO(AlonH): Make memory size configurable.
pub const MEMORY_ID_SIZE: usize = 1;
pub const N_MULTIPLICITY_COLUMNS: usize = 1;
pub const BIG_N_COLUMNS: usize = FELT252_N_WORDS + N_MULTIPLICITY_COLUMNS;

pub type BigComponent = FrameworkComponent<BigEval>;

const N_LOGUP_POWERS: usize = MEMORY_ID_SIZE + FELT252_N_WORDS;

pub const RELATION_USES_PER_ROW: [RelationUse; 8] = [
    RelationUse {
        relation_id: "RangeCheck_9_9",
        uses: 2,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_B",
        uses: 2,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_C",
        uses: 2,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_D",
        uses: 2,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_E",
        uses: 2,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_F",
        uses: 2,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_G",
        uses: 1,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_H",
        uses: 1,
    },
];

relation!(RelationElements, N_LOGUP_POWERS);

/// IDs are continuous and start from 0.
/// Values are Felt252 stored as `FELT252_N_WORDS` M31 values (each value containing 9 bits).
#[derive(Clone)]
pub struct BigEval {
    pub log_n_rows: u32,
    // Internal offset of the ids when there are multiple components.
    pub offset: u32,
    pub common_lookup_elements: relations::CommonLookupElements,
}
impl BigEval {
    pub fn new(
        log_n_rows: u32,
        offset: u32,
        common_lookup_elements: relations::CommonLookupElements,
    ) -> Self {
        Self {
            log_n_rows,
            offset,
            common_lookup_elements,
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

    #[allow(non_snake_case)]
    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let M31_1662111297 = E::F::from(M31::from(1662111297));
        let seq = eval.get_preprocessed_column(Seq::new(self.log_size()).id());
        let memory_id_to_big_output_col0 = eval.next_trace_mask();
        let memory_id_to_big_output_col1 = eval.next_trace_mask();
        let memory_id_to_big_output_col2 = eval.next_trace_mask();
        let memory_id_to_big_output_col3 = eval.next_trace_mask();
        let memory_id_to_big_output_col4 = eval.next_trace_mask();
        let memory_id_to_big_output_col5 = eval.next_trace_mask();
        let memory_id_to_big_output_col6 = eval.next_trace_mask();
        let memory_id_to_big_output_col7 = eval.next_trace_mask();
        let memory_id_to_big_output_col8 = eval.next_trace_mask();
        let memory_id_to_big_output_col9 = eval.next_trace_mask();
        let memory_id_to_big_output_col10 = eval.next_trace_mask();
        let memory_id_to_big_output_col11 = eval.next_trace_mask();
        let memory_id_to_big_output_col12 = eval.next_trace_mask();
        let memory_id_to_big_output_col13 = eval.next_trace_mask();
        let memory_id_to_big_output_col14 = eval.next_trace_mask();
        let memory_id_to_big_output_col15 = eval.next_trace_mask();
        let memory_id_to_big_output_col16 = eval.next_trace_mask();
        let memory_id_to_big_output_col17 = eval.next_trace_mask();
        let memory_id_to_big_output_col18 = eval.next_trace_mask();
        let memory_id_to_big_output_col19 = eval.next_trace_mask();
        let memory_id_to_big_output_col20 = eval.next_trace_mask();
        let memory_id_to_big_output_col21 = eval.next_trace_mask();
        let memory_id_to_big_output_col22 = eval.next_trace_mask();
        let memory_id_to_big_output_col23 = eval.next_trace_mask();
        let memory_id_to_big_output_col24 = eval.next_trace_mask();
        let memory_id_to_big_output_col25 = eval.next_trace_mask();
        let memory_id_to_big_output_col26 = eval.next_trace_mask();
        let memory_id_to_big_output_col27 = eval.next_trace_mask();
        let multiplicity = eval.next_trace_mask();

        RangeCheckMemValueN28::evaluate(
            [
                memory_id_to_big_output_col0.clone(),
                memory_id_to_big_output_col1.clone(),
                memory_id_to_big_output_col2.clone(),
                memory_id_to_big_output_col3.clone(),
                memory_id_to_big_output_col4.clone(),
                memory_id_to_big_output_col5.clone(),
                memory_id_to_big_output_col6.clone(),
                memory_id_to_big_output_col7.clone(),
                memory_id_to_big_output_col8.clone(),
                memory_id_to_big_output_col9.clone(),
                memory_id_to_big_output_col10.clone(),
                memory_id_to_big_output_col11.clone(),
                memory_id_to_big_output_col12.clone(),
                memory_id_to_big_output_col13.clone(),
                memory_id_to_big_output_col14.clone(),
                memory_id_to_big_output_col15.clone(),
                memory_id_to_big_output_col16.clone(),
                memory_id_to_big_output_col17.clone(),
                memory_id_to_big_output_col18.clone(),
                memory_id_to_big_output_col19.clone(),
                memory_id_to_big_output_col20.clone(),
                memory_id_to_big_output_col21.clone(),
                memory_id_to_big_output_col22.clone(),
                memory_id_to_big_output_col23.clone(),
                memory_id_to_big_output_col24.clone(),
                memory_id_to_big_output_col25.clone(),
                memory_id_to_big_output_col26.clone(),
                memory_id_to_big_output_col27.clone(),
            ],
            &self.common_lookup_elements,
            &mut eval,
        );
        eval.add_to_relation(RelationEntry::new(
            &self.common_lookup_elements,
            -E::EF::from(multiplicity),
            &[
                M31_1662111297.clone(),
                seq.clone()
                    + E::F::from(M31::from(LARGE_MEMORY_VALUE_ID_BASE))
                    + E::F::from(M31::from(self.offset)),
                memory_id_to_big_output_col0.clone(),
                memory_id_to_big_output_col1.clone(),
                memory_id_to_big_output_col2.clone(),
                memory_id_to_big_output_col3.clone(),
                memory_id_to_big_output_col4.clone(),
                memory_id_to_big_output_col5.clone(),
                memory_id_to_big_output_col6.clone(),
                memory_id_to_big_output_col7.clone(),
                memory_id_to_big_output_col8.clone(),
                memory_id_to_big_output_col9.clone(),
                memory_id_to_big_output_col10.clone(),
                memory_id_to_big_output_col11.clone(),
                memory_id_to_big_output_col12.clone(),
                memory_id_to_big_output_col13.clone(),
                memory_id_to_big_output_col14.clone(),
                memory_id_to_big_output_col15.clone(),
                memory_id_to_big_output_col16.clone(),
                memory_id_to_big_output_col17.clone(),
                memory_id_to_big_output_col18.clone(),
                memory_id_to_big_output_col19.clone(),
                memory_id_to_big_output_col20.clone(),
                memory_id_to_big_output_col21.clone(),
                memory_id_to_big_output_col22.clone(),
                memory_id_to_big_output_col23.clone(),
                memory_id_to_big_output_col24.clone(),
                memory_id_to_big_output_col25.clone(),
                memory_id_to_big_output_col26.clone(),
                memory_id_to_big_output_col27.clone(),
            ],
        ));

        eval.finalize_logup_in_pairs();
        eval
    }
}

pub fn big_components_from_claim(
    log_sizes: &[u32],
    claimed_sums: &[SecureField],
    common_lookup_elements: &relations::CommonLookupElements,
    tree_span_provider: &mut TraceLocationAllocator,
) -> Vec<BigComponent> {
    // Every component is responsible for a range of memory ids. The ids must not overlap. Use an
    // offset to keep track of the previous component's range.
    let mut components = vec![];
    let mut offset = 0;
    for (&log_size, &claimed_sum) in log_sizes.iter().zip_eq(claimed_sums) {
        components.push(BigComponent::new(
            tree_span_provider,
            BigEval::new(log_size, offset, common_lookup_elements.clone()),
            claimed_sum,
        ));
        offset += 1 << log_size;
    }
    components
}

#[derive(Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct Claim {
    pub big_log_sizes: Vec<u32>,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        // Original trace.
        let big_trace_log_sizes = self
            .big_log_sizes
            .iter()
            .flat_map(|&log_size| vec![log_size; BIG_N_COLUMNS])
            .collect_vec();

        // Interaction trace.
        let big_interaction_log_sizes = self
            .big_log_sizes
            .iter()
            .flat_map(|&log_size| {
                // A range-check for every pair of limbs, batched in pairs.
                // And a yield of the value.
                vec![
                    log_size;
                    SECURE_EXTENSION_DEGREE * ((FELT252_N_WORDS.div_ceil(2) + 1).div_ceil(2))
                ]
            })
            .collect_vec();

        TreeVec::new(vec![vec![], big_trace_log_sizes, big_interaction_log_sizes])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        self.big_log_sizes
            .iter()
            .for_each(|&log_size| channel.mix_u64(log_size as u64));
    }
}

#[derive(Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct InteractionClaim {
    pub big_claimed_sums: Vec<SecureField>,
    pub claimed_sum: SecureField,
}
impl InteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_felts(&self.big_claimed_sums);
    }
}

#[cfg(test)]
mod tests {
    use num_traits::Zero;
    use rand::rngs::SmallRng;
    use rand::{Rng, SeedableRng};
    use stwo::core::fields::qm31::QM31;
    use stwo_constraint_framework::expr::ExprEvaluator;

    use super::*;
    use crate::components::constraints_regression_test_values::BIG_MEMORY_ID_TO_BIG;

    #[test]
    fn memory_id_to_big_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let big_eval = BigEval {
            log_n_rows: 4,
            offset: 0,
            common_lookup_elements: relations::CommonLookupElements::dummy(),
        };
        let big_expr_eval = big_eval.evaluate(ExprEvaluator::new());
        let big_assignment = big_expr_eval.random_assignment();

        let mut big_sum = QM31::zero();
        for c in big_expr_eval.constraints {
            big_sum += c.assign(&big_assignment) * rng.gen::<QM31>();
        }

        BIG_MEMORY_ID_TO_BIG.assert_debug_eq(&big_sum);
    }
}
