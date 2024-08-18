use itertools::Itertools;
use stwo_prover::constraint_framework::logup::LogupTraceGenerator;
use stwo_prover::core::backend::simd::m31::{PackedBaseField, PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::qm31::PackedQM31;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::{Col, Column};
use stwo_prover::core::fields::m31::{BaseField, M31};
use stwo_prover::core::pcs::TreeBuilder;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleHasher;

use super::component::{RangeCheckClaim, RangeCheckInteractionClaim};
use super::RangeCheckElements;
use crate::prover_types::PackedUInt32;

const TRACE_INDICES_OFFSET: usize = 0;
const TRACE_MULTIPLICITIES_OFFSET: usize = 1;

pub struct RangeCheckClaimProver<const RC_TRACE_LEN: usize> {
    pub multiplicities: [PackedUInt32; RC_TRACE_LEN],
}
impl<const RC_TRACE_LEN: usize> RangeCheckClaimProver<RC_TRACE_LEN> {
    pub fn new() -> Self {
        let multiplicities = [PackedUInt32::broadcast(0); RC_TRACE_LEN];
        Self { multiplicities }
    }

    pub fn add_inputs(&mut self, inputs: &PackedM31) {
        for input in inputs.to_array() {
            let num = input.0 as usize;
            let (input_row, input_lane) = (num / N_LANES, num % N_LANES);
            self.multiplicities[input_row].simd[input_lane] += 1;
        }
    }

    pub fn write_trace(
        &mut self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleHasher>,
    ) -> (
        RangeCheckClaim,
        RangeCheckInteractionClaimProver<RC_TRACE_LEN>,
    ) {
        assert!(RC_TRACE_LEN.is_power_of_two());

        let mut trace = (0..2)
            .map(|_| unsafe {
                Col::<SimdBackend, BaseField>::uninitialized(RC_TRACE_LEN / N_LANES)
            })
            .collect_vec();

        // TODO(alont) Change this to constant column when supported.
        for (vec_row, multiplicity) in self.multiplicities.into_iter().enumerate() {
            trace[TRACE_INDICES_OFFSET].data[vec_row] =
                PackedBaseField::from_array(std::array::from_fn(|i| {
                    M31::from_u32_unchecked(((vec_row << LOG_N_LANES) + i) as u32)
                }));
            trace[TRACE_MULTIPLICITIES_OFFSET].data[vec_row] = multiplicity.as_m31_unchecked();
        }

        // TODO(alont): Can this clone be avoided?
        let multiplicities = trace[TRACE_MULTIPLICITIES_OFFSET].data.clone();

        // Extend trace.
        let domain = CanonicCoset::new(RC_TRACE_LEN.ilog2()).circle_domain();
        let trace = trace
            .into_iter()
            .map(|eval| CircleEvaluation::<SimdBackend, _, BitReversedOrder>::new(domain, eval))
            .collect_vec();
        tree_builder.extend_evals(trace);

        (
            RangeCheckClaim {
                log_rc_max: RC_TRACE_LEN.ilog2() + LOG_N_LANES,
            },
            RangeCheckInteractionClaimProver { multiplicities },
        )
    }
}

impl<const RC_TRACE_LEN: usize> Default for RangeCheckClaimProver<RC_TRACE_LEN> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct RangeCheckInteractionClaimProver<const RC_TRACE_LEN: usize> {
    pub multiplicities: Vec<PackedM31>,
}
impl<const RC_TRACE_LEN: usize> RangeCheckInteractionClaimProver<RC_TRACE_LEN> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            multiplicities: Vec::with_capacity(capacity),
        }
    }

    pub fn write_interaction_trace(
        &self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleHasher>,
        lookup_elements: &RangeCheckElements,
    ) -> RangeCheckInteractionClaim {
        let mut logup_gen = LogupTraceGenerator::new(1 << RC_TRACE_LEN);
        let mut col_gen = logup_gen.new_col();

        // Lookup values columns.
        for vec_row in 0..(RC_TRACE_LEN >> LOG_N_LANES) {
            let value = PackedBaseField::from_array(std::array::from_fn(|i| {
                M31(((vec_row << LOG_N_LANES) + i) as u32)
            }));
            let denom: PackedQM31 = lookup_elements.combine(&[value]);
            col_gen.write_frac(vec_row, (-self.multiplicities[vec_row]).into(), denom);
        }
        col_gen.finalize_col();
        let (trace, claimed_sum) = logup_gen.finalize();
        tree_builder.extend_evals(trace);

        RangeCheckInteractionClaim { claimed_sum }
    }
}
