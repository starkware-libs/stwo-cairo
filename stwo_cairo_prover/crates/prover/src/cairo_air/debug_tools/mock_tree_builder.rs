use itertools::Itertools;
use stwo_prover::core::backend::{Backend, Column};
use stwo_prover::core::fields::m31::M31;
use stwo_prover::core::pcs::{TreeSubspan, TreeVec};
use stwo_prover::core::poly::circle::CircleEvaluation;
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::ColumnVec;

use crate::witness::utils::TreeBuilder;

/// A mock commitment scheme implementation used for testing purposes.
pub struct MockCommitmentScheme {
    pub trees: TreeVec<ColumnVec<Vec<M31>>>,
}

impl MockCommitmentScheme {
    pub fn new() -> Self {
        Self {
            trees: Default::default(),
        }
    }

    pub fn tree_builder(&mut self) -> MockTreeBuilder<'_> {
        MockTreeBuilder {
            tree_index: self.trees.len(),
            commitment_scheme: self,
            evals: Vec::default(),
        }
    }

    fn next_interaction(&mut self, evals: ColumnVec<Vec<M31>>) {
        self.trees.push(evals);
    }

    /// Returns the trace domain evaluations.
    /// Used for testing purposes.
    pub fn trace_domain_evaluations(&self) -> TreeVec<ColumnVec<&Vec<M31>>> {
        self.trees.as_cols_ref()
    }
}

/// A [`TreeBuilder`] used by [`MockCommitmentScheme`] to aggregate trace values.
/// This implementation avoids low degree extension and Merkle commitments.
pub struct MockTreeBuilder<'a> {
    tree_index: usize,
    evals: ColumnVec<Vec<M31>>,
    commitment_scheme: &'a mut MockCommitmentScheme,
}
impl MockTreeBuilder<'_> {
    pub fn extend_evals<B: Backend>(
        &mut self,
        columns: impl IntoIterator<Item = CircleEvaluation<B, M31, BitReversedOrder>>,
    ) {
        self.evals
            .extend(columns.into_iter().map(|s| s.to_cpu()).collect_vec());
    }

    pub fn finalize_interaction(self) {
        self.commitment_scheme.next_interaction(self.evals);
    }
}

impl<B: Backend> TreeBuilder<B> for MockTreeBuilder<'_> {
    fn extend_evals(
        &mut self,
        columns: impl IntoIterator<Item = CircleEvaluation<B, M31, BitReversedOrder>>,
    ) -> TreeSubspan {
        let col_start = self.evals.len();
        let tree_index = self.tree_index;
        self.extend_evals(columns);
        let col_end = self.evals.len();
        TreeSubspan {
            tree_index,
            col_start,
            col_end,
        }
    }
}

mod tests {
    use std::array;
    use std::simd::u32x16;

    use rand::rngs::SmallRng;
    use rand::{Rng, SeedableRng};
    use stwo_cairo_common::prover_types::simd::PackedUInt32;

    use super::MockCommitmentScheme;
    use crate::cairo_air::relations;
    use crate::witness::components::{triple_xor_32, verify_bitwise_xor_8};

    #[test]
    fn test_mock_commitment_scheme() {
        let mut rng = SmallRng::seed_from_u64(0);
        let input = array::from_fn(|_| array::from_fn(|_| rng.gen()))
            .map(u32x16::from_array)
            .map(PackedUInt32::from_simd);
        let veirfy_bitwise_xor_8_trace_gen = &verify_bitwise_xor_8::ClaimGenerator::new();
        let mut triple_xor_32_trace_gen = triple_xor_32::ClaimGenerator::new();
        triple_xor_32_trace_gen.add_packed_inputs(&[input]);
        let triple_xor_relation = relations::TripleXor32::dummy();
        let verify_bitwise_xor_8_relation = relations::VerifyBitwiseXor_8::dummy();

        let mut mock_commitment_scheme = MockCommitmentScheme::new();

        // Preprocessed.
        mock_commitment_scheme.tree_builder().finalize_interaction();
        let mut mock_tree_builder = mock_commitment_scheme.tree_builder();

        // Base trace.
        let (_, interaction_gen) = triple_xor_32_trace_gen
            .write_trace(&mut mock_tree_builder, veirfy_bitwise_xor_8_trace_gen);
        mock_tree_builder.finalize_interaction();
        let mut mock_tree_builder = mock_commitment_scheme.tree_builder();

        // Interaction trace.
        interaction_gen.write_interaction_trace(
            &mut mock_tree_builder,
            &triple_xor_relation,
            &verify_bitwise_xor_8_relation,
        );
        mock_tree_builder.finalize_interaction();
        let trace = mock_commitment_scheme.trace_domain_evaluations();

        assert_eq!(trace.len(), 3);
        assert_eq!(trace[0].len(), 0);
        assert_eq!(trace[1].len(), 20);
        assert_eq!(trace[2].len(), 20);
        assert_eq!(trace[1][0].len(), 16);
        assert_eq!(trace[2][0].len(), 16);
    }
}
