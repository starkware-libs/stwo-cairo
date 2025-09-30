use serde::{Deserialize, Serialize};
use stwo::core::channel::Channel;
use stwo::core::fields::m31::M31;
use stwo::core::fields::qm31::{SecureField, SECURE_EXTENSION_DEGREE};
use stwo::core::pcs::TreeVec;
use stwo_cairo_common::preprocessed_columns::preprocessed_trace::{PreProcessedColumn, Seq};
use stwo_cairo_serialize::{CairoDeserialize, CairoSerialize};
use stwo_constraint_framework::{EvalAtRow, FrameworkComponent, FrameworkEval, RelationEntry};

use crate::relations;

/// Split the (ID , Multiplicity) columns to shorter chunks. This is done to improve the performance
/// during The merkle commitment and FRI, as this component is usually the tallest in the Cairo AIR.
///
/// 1. The ID and Multiplicity vectors are split to 'MEMORY_ADDRESS_TO_ID_SPLIT' chunks of size
///    `ids.len()`/`MEMORY_ADDRESS_TO_ID_SPLIT`.
/// 2. The chunks are padded with 0s to the next power of 2.
///
/// #  Example
/// ID = [id0..id10], MEMORY_ADDRESS_TO_ID_SPLIT = 4:
/// ID0 = [id0, id1, id2, 0]
/// ID1 = [id3, id4, id5, 0]
/// ID2 = [id6, id7, id8, 0]
/// ID3 = [id9, id10, 0, 0]
pub const MEMORY_ADDRESS_TO_ID_SPLIT: usize = 16;
pub const N_ID_AND_MULT_COLUMNS_PER_CHUNK: usize = 2;
pub const N_TRACE_COLUMNS: usize = MEMORY_ADDRESS_TO_ID_SPLIT * N_ID_AND_MULT_COLUMNS_PER_CHUNK;

pub type Component = FrameworkComponent<Eval>;

#[derive(Clone)]
pub struct Eval {
    // The log size of the component after split.
    pub log_size: u32,
    pub lookup_elements: relations::MemoryAddressToId,
}
impl Eval {
    pub fn new(claim: Claim, lookup_elements: relations::MemoryAddressToId) -> Self {
        Self {
            log_size: claim.log_size,
            lookup_elements,
        }
    }
}

impl FrameworkEval for Eval {
    fn log_size(&self) -> u32 {
        self.log_size
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_size() + 1
    }

    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        // Addresses are offsetted by 1, as 0 address is reserved.
        let seq_plus_one =
            eval.get_preprocessed_column(Seq::new(self.log_size()).id()) + E::F::from(M31(1));
        for i in 0..MEMORY_ADDRESS_TO_ID_SPLIT {
            let id = eval.next_trace_mask();
            let multiplicity = eval.next_trace_mask();
            let address =
                seq_plus_one.clone() + E::F::from(M31((i * (1 << self.log_size())) as u32));
            eval.add_to_relation(RelationEntry::new(
                &self.lookup_elements,
                E::EF::from(-multiplicity),
                &[address, id],
            ));
        }

        eval.finalize_logup_in_pairs();
        eval
    }
}

#[derive(Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct Claim {
    pub log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; N_TRACE_COLUMNS];
        let interaction_log_sizes =
            vec![self.log_size; SECURE_EXTENSION_DEGREE * MEMORY_ADDRESS_TO_ID_SPLIT.div_ceil(2)];
        TreeVec::new(vec![vec![], trace_log_sizes, interaction_log_sizes])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(self.log_size as u64);
    }
}

#[derive(Clone, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
pub struct InteractionClaim {
    pub claimed_sum: SecureField,
}
impl InteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_felts(&[self.claimed_sum]);
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
    use crate::components::constraints_regression_test_values::MEMORY_ADDRESS_TO_ID;

    #[test]
    fn memory_address_to_id_constraints_regression() {
        let mut rng = SmallRng::seed_from_u64(0);
        let eval = Eval {
            log_size: 4,
            lookup_elements: relations::MemoryAddressToId::dummy(),
        };

        let expr_eval = eval.evaluate(ExprEvaluator::new());
        let assignment = expr_eval.random_assignment();

        let mut sum = QM31::zero();
        for c in expr_eval.constraints {
            sum += c.assign(&assignment) * rng.gen::<QM31>();
        }

        assert_eq!(sum, MEMORY_ADDRESS_TO_ID);
    }
}
