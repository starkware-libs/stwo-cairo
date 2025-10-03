use itertools::{chain, Itertools};
use num_traits::One;
use serde::{Deserialize, Serialize};
use stwo::core::channel::Channel;
use stwo::core::fields::m31::M31;
use stwo::core::fields::qm31::{SecureField, SECURE_EXTENSION_DEGREE};
use stwo::core::pcs::TreeVec;
use stwo_cairo_common::memory::{
    LARGE_MEMORY_VALUE_ID_BASE, N_M31_IN_FELT252, N_M31_IN_SMALL_FELT252,
};
use stwo_cairo_common::preprocessed_columns::preprocessed_trace::{PreProcessedColumn, Seq};
use stwo_cairo_serialize::{CairoDeserialize, CairoSerialize};
use stwo_constraint_framework::{
    relation, EvalAtRow, FrameworkComponent, FrameworkEval, RelationEntry, TraceLocationAllocator,
};

use super::prelude::RelationUse;
use crate::relations;

// TODO(AlonH): Make memory size configurable.
pub const MEMORY_ID_SIZE: usize = 1;
pub const N_MULTIPLICITY_COLUMNS: usize = 1;
pub const BIG_N_COLUMNS: usize = N_M31_IN_FELT252 + N_MULTIPLICITY_COLUMNS;
pub const SMALL_N_COLUMNS: usize = N_M31_IN_SMALL_FELT252 + N_MULTIPLICITY_COLUMNS;

pub type BigComponent = FrameworkComponent<BigEval>;
pub type SmallComponent = FrameworkComponent<SmallEval>;

const N_LOGUP_POWERS: usize = MEMORY_ID_SIZE + N_M31_IN_FELT252;

pub const RELATION_USES_PER_ROW_BIG: [RelationUse; 8] = [
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

pub const RELATION_USES_PER_ROW_SMALL: [RelationUse; 4] = [
    RelationUse {
        relation_id: "RangeCheck_9_9",
        uses: 1,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_B",
        uses: 1,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_C",
        uses: 1,
    },
    RelationUse {
        relation_id: "RangeCheck_9_9_D",
        uses: 1,
    },
];

relation!(RelationElements, N_LOGUP_POWERS);

/// IDs are continuous and start from 0.
/// Values are Felt252 stored as `N_M31_IN_FELT252` M31 values (each value containing 9 bits).
#[derive(Clone)]
pub struct BigEval {
    pub log_n_rows: u32,
    // Internal offset of the ids when there are multiple components.
    pub offset: u32,
    pub lookup_elements: relations::MemoryIdToBig,
    pub range_check_9_9_lookup_elements: relations::RangeCheck_9_9,
    pub range_check_9_9_b_lookup_elements: relations::RangeCheck_9_9_B,
    pub range_check_9_9_c_lookup_elements: relations::RangeCheck_9_9_C,
    pub range_check_9_9_d_lookup_elements: relations::RangeCheck_9_9_D,
    pub range_check_9_9_e_lookup_elements: relations::RangeCheck_9_9_E,
    pub range_check_9_9_f_lookup_elements: relations::RangeCheck_9_9_F,
    pub range_check_9_9_g_lookup_elements: relations::RangeCheck_9_9_G,
    pub range_check_9_9_h_lookup_elements: relations::RangeCheck_9_9_H,
}
#[allow(clippy::too_many_arguments)]
impl BigEval {
    pub fn new(
        log_n_rows: u32,
        offset: u32,
        lookup_elements: relations::MemoryIdToBig,
        range_check_9_9_lookup_elements: relations::RangeCheck_9_9,
        range_check_9_9_b_lookup_elements: relations::RangeCheck_9_9_B,
        range_check_9_9_c_lookup_elements: relations::RangeCheck_9_9_C,
        range_check_9_9_d_lookup_elements: relations::RangeCheck_9_9_D,
        range_check_9_9_e_lookup_elements: relations::RangeCheck_9_9_E,
        range_check_9_9_f_lookup_elements: relations::RangeCheck_9_9_F,
        range_check_9_9_g_lookup_elements: relations::RangeCheck_9_9_G,
        range_check_9_9_h_lookup_elements: relations::RangeCheck_9_9_H,
    ) -> Self {
        Self {
            log_n_rows,
            offset,
            lookup_elements,
            range_check_9_9_lookup_elements,
            range_check_9_9_b_lookup_elements,
            range_check_9_9_c_lookup_elements,
            range_check_9_9_d_lookup_elements,
            range_check_9_9_e_lookup_elements,
            range_check_9_9_f_lookup_elements,
            range_check_9_9_g_lookup_elements,
            range_check_9_9_h_lookup_elements,
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
        for (i, (l, r)) in value.iter().tuples().enumerate() {
            let limb_pair = [l.clone(), r.clone()];
            match i % 8 {
                0 => eval.add_to_relation(RelationEntry::new(
                    &self.range_check_9_9_lookup_elements,
                    E::EF::one(),
                    &limb_pair,
                )),
                1 => eval.add_to_relation(RelationEntry::new(
                    &self.range_check_9_9_b_lookup_elements,
                    E::EF::one(),
                    &limb_pair,
                )),
                2 => eval.add_to_relation(RelationEntry::new(
                    &self.range_check_9_9_c_lookup_elements,
                    E::EF::one(),
                    &limb_pair,
                )),
                3 => eval.add_to_relation(RelationEntry::new(
                    &self.range_check_9_9_d_lookup_elements,
                    E::EF::one(),
                    &limb_pair,
                )),
                4 => eval.add_to_relation(RelationEntry::new(
                    &self.range_check_9_9_e_lookup_elements,
                    E::EF::one(),
                    &limb_pair,
                )),
                5 => eval.add_to_relation(RelationEntry::new(
                    &self.range_check_9_9_f_lookup_elements,
                    E::EF::one(),
                    &limb_pair,
                )),
                6 => eval.add_to_relation(RelationEntry::new(
                    &self.range_check_9_9_g_lookup_elements,
                    E::EF::one(),
                    &limb_pair,
                )),
                7 => eval.add_to_relation(RelationEntry::new(
                    &self.range_check_9_9_h_lookup_elements,
                    E::EF::one(),
                    &limb_pair,
                )),
                _ => {
                    unreachable!("There are only 8 possible values for i % 8.",)
                }
            };
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

#[allow(clippy::too_many_arguments)]
pub fn big_components_from_claim(
    log_sizes: &[u32],
    claimed_sums: &[SecureField],
    lookup_elements: &relations::MemoryIdToBig,
    range_check_9_9_lookup_elements: &relations::RangeCheck_9_9,
    range_check_9_9_b_lookup_elements: &relations::RangeCheck_9_9_B,
    range_check_9_9_c_lookup_elements: &relations::RangeCheck_9_9_C,
    range_check_9_9_d_lookup_elements: &relations::RangeCheck_9_9_D,
    range_check_9_9_e_lookup_elements: &relations::RangeCheck_9_9_E,
    range_check_9_9_f_lookup_elements: &relations::RangeCheck_9_9_F,
    range_check_9_9_g_lookup_elements: &relations::RangeCheck_9_9_G,
    range_check_9_9_h_lookup_elements: &relations::RangeCheck_9_9_H,
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
                range_check_9_9_lookup_elements.clone(),
                range_check_9_9_b_lookup_elements.clone(),
                range_check_9_9_c_lookup_elements.clone(),
                range_check_9_9_d_lookup_elements.clone(),
                range_check_9_9_e_lookup_elements.clone(),
                range_check_9_9_f_lookup_elements.clone(),
                range_check_9_9_g_lookup_elements.clone(),
                range_check_9_9_h_lookup_elements.clone(),
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
    pub range_check_9_9_b_relation: relations::RangeCheck_9_9_B,
    pub range_check_9_9_c_relation: relations::RangeCheck_9_9_C,
    pub range_check_9_9_d_relation: relations::RangeCheck_9_9_D,
}
impl SmallEval {
    pub fn new(
        claim: Claim,
        lookup_elements: relations::MemoryIdToBig,
        range_check_9_9_relation: relations::RangeCheck_9_9,
        range_check_9_9_b_relation: relations::RangeCheck_9_9_B,
        range_check_9_9_c_relation: relations::RangeCheck_9_9_C,
        range_check_9_9_d_relation: relations::RangeCheck_9_9_D,
    ) -> Self {
        Self {
            log_n_rows: claim.small_log_size,
            lookup_elements,
            range_check_9_9_relation,
            range_check_9_9_b_relation,
            range_check_9_9_c_relation,
            range_check_9_9_d_relation,
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
        for (i, (l, r)) in value.iter().tuples().enumerate() {
            let limb_pair = [l.clone(), r.clone()];
            match i % 4 {
                0 => eval.add_to_relation(RelationEntry::new(
                    &self.range_check_9_9_relation,
                    E::EF::one(),
                    &limb_pair,
                )),
                1 => eval.add_to_relation(RelationEntry::new(
                    &self.range_check_9_9_b_relation,
                    E::EF::one(),
                    &limb_pair,
                )),
                2 => eval.add_to_relation(RelationEntry::new(
                    &self.range_check_9_9_c_relation,
                    E::EF::one(),
                    &limb_pair,
                )),
                3 => eval.add_to_relation(RelationEntry::new(
                    &self.range_check_9_9_d_relation,
                    E::EF::one(),
                    &limb_pair,
                )),
                _ => {
                    unreachable!("There are only 4 possible values for i % 4.",)
                }
            };
        }

        // Yield the value.
        let id = seq;
        eval.add_to_relation(RelationEntry::new(
            &self.lookup_elements,
            E::EF::from(-multiplicity),
            &chain!([id], value).collect_vec(),
        ));

        eval.finalize_logup_in_pairs();
        eval
    }
}

#[derive(Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
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
        let small_interaction_log_sizes =
            vec![
                self.small_log_size;
                SECURE_EXTENSION_DEGREE * (N_M31_IN_SMALL_FELT252.div_ceil(2) + 1).div_ceil(2)
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

#[derive(Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
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

#[cfg(test)]
mod tests {
    use num_traits::Zero;
    use rand::rngs::SmallRng;
    use rand::{Rng, SeedableRng};
    use stwo::core::fields::qm31::QM31;
    use stwo_constraint_framework::expr::ExprEvaluator;

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
            range_check_9_9_lookup_elements: relations::RangeCheck_9_9::dummy(),
            range_check_9_9_b_lookup_elements: relations::RangeCheck_9_9_B::dummy(),
            range_check_9_9_c_lookup_elements: relations::RangeCheck_9_9_C::dummy(),
            range_check_9_9_d_lookup_elements: relations::RangeCheck_9_9_D::dummy(),
            range_check_9_9_e_lookup_elements: relations::RangeCheck_9_9_E::dummy(),
            range_check_9_9_f_lookup_elements: relations::RangeCheck_9_9_F::dummy(),
            range_check_9_9_g_lookup_elements: relations::RangeCheck_9_9_G::dummy(),
            range_check_9_9_h_lookup_elements: relations::RangeCheck_9_9_H::dummy(),
        };
        let small_eval = SmallEval {
            log_n_rows: 4,
            lookup_elements: relations::MemoryIdToBig::dummy(),
            range_check_9_9_relation: relations::RangeCheck_9_9::dummy(),
            range_check_9_9_b_relation: relations::RangeCheck_9_9_B::dummy(),
            range_check_9_9_c_relation: relations::RangeCheck_9_9_C::dummy(),
            range_check_9_9_d_relation: relations::RangeCheck_9_9_D::dummy(),
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
