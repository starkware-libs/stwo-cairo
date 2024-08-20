use itertools::Itertools;
use stwo_prover::constraint_framework::logup::LogupTraceGenerator;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::m31::{PackedBaseField, PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::qm31::PackedQM31;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::{Col, Column};
use stwo_prover::core::fields::m31::{BaseField, M31};
use stwo_prover::core::pcs::TreeBuilder;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;

use super::component::{RangeCheckClaim, RangeCheckInteractionClaim};
use super::RangeCheckElements;
use crate::prover_types::PackedUInt32;

const TRACE_INDICES_OFFSET: usize = 0;
const TRACE_MULTIPLICITIES_OFFSET: usize = 1;

/// Range check unit claim prover.
/// `multiplicities` are a nested vector of SIMD vectors, where the first dimension is the
/// repetition and the second dimension is the offset within the repetition.
/// Thus, `multiplicities[rep][off].simd[i]` refers to the multiplicity of
/// rep * RC_LOG_HEIGHT + off * N_LANES + i.
pub struct RangeCheckClaimProver<const RC_LOG_HEIGHT: u32, const N_REPETITIONS: usize> {
    pub multiplicities: [Vec<PackedUInt32>; N_REPETITIONS],
}
impl<const RC_LOG_HEIGHT: u32, const N_REPETITIONS: usize>
    RangeCheckClaimProver<RC_LOG_HEIGHT, N_REPETITIONS>
{
    pub fn new() -> Self {
        let multiplicities: [Vec<PackedUInt32>; N_REPETITIONS] = std::array::from_fn(|_| {
            vec![PackedUInt32::broadcast(0); 1 << (RC_LOG_HEIGHT - LOG_N_LANES) as usize]
        });
        Self { multiplicities }
    }

    pub fn add_input(&mut self, input: M31) {
        let num = input.0 as usize;
        let (input_repetition, input_offset) =
            (num >> RC_LOG_HEIGHT, num % (1 << (RC_LOG_HEIGHT as usize)));
        let (input_row, input_lane) = (input_offset / N_LANES, input_offset % N_LANES);

        self.multiplicities[input_repetition][input_row].simd[input_lane] += 1;
    }

    pub fn add_inputs(&mut self, inputs: &[PackedM31]) {
        for input in inputs {
            self.add_m31s(&input.to_array())
        }
    }

    pub fn add_m31s(&mut self, inputs: &[M31]) {
        for input in inputs {
            self.add_input(*input)
        }
    }

    pub fn generate_trace(&mut self) -> Vec<BaseColumn> {
        let mut trace = (0..(N_REPETITIONS + 1))
            .map(|_| unsafe { Col::<SimdBackend, BaseField>::uninitialized(1 << RC_LOG_HEIGHT) })
            .collect_vec();

        for vec_row in 0..(1 << (RC_LOG_HEIGHT - LOG_N_LANES)) {
            trace[TRACE_INDICES_OFFSET].data[vec_row] =
                PackedBaseField::from_array(std::array::from_fn(|i| {
                    M31::from_u32_unchecked(((vec_row << LOG_N_LANES) + i) as u32)
                }));
        }

        for rep in 0..N_REPETITIONS {
            for (vec_row, multiplicity) in self.multiplicities[rep].iter().enumerate() {
                trace[TRACE_MULTIPLICITIES_OFFSET + rep].data[vec_row] =
                    multiplicity.as_m31_unchecked();
            }
        }
        trace
    }

    pub fn write_trace(
        &mut self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
    ) -> (
        RangeCheckClaim<N_REPETITIONS>,
        RangeCheckInteractionClaimProver<RC_LOG_HEIGHT, N_REPETITIONS>,
    ) {
        let trace = self.generate_trace();

        let multiplicities: [Vec<PackedUInt32>; N_REPETITIONS] = std::array::from_fn(|i| {
            trace[TRACE_MULTIPLICITIES_OFFSET + i]
                .data
                .iter()
                .map(|x| PackedUInt32::from_array(x.to_array().map(|c| c.0)))
                .collect_vec()
        });

        let domain = CanonicCoset::new(RC_LOG_HEIGHT).circle_domain();
        tree_builder.extend_evals(
            trace
                .into_iter()
                .map(|eval| CircleEvaluation::<SimdBackend, _, BitReversedOrder>::new(domain, eval))
                .collect_vec(),
        );

        (
            RangeCheckClaim::<N_REPETITIONS> {
                log_rc_height: RC_LOG_HEIGHT,
            },
            RangeCheckInteractionClaimProver { multiplicities },
        )
    }
}

impl<const RC_TRACE_LEN: u32, const N_REPETITIONS: usize> Default
    for RangeCheckClaimProver<RC_TRACE_LEN, N_REPETITIONS>
{
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct RangeCheckInteractionClaimProver<const RC_LOG_HEIGHT: u32, const N_REPETITIONS: usize> {
    pub multiplicities: [Vec<PackedUInt32>; N_REPETITIONS],
}
impl<const RC_LOG_HEIGHT: u32, const N_REPETITIONS: usize>
    RangeCheckInteractionClaimProver<RC_LOG_HEIGHT, N_REPETITIONS>
{
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            multiplicities: std::array::from_fn(|_| Vec::with_capacity(capacity)),
        }
    }

    pub fn generate_interaction_trace(
        &self,
        lookup_elements: &RangeCheckElements,
    ) -> LogupTraceGenerator {
        let mut logup_gen = LogupTraceGenerator::new(RC_LOG_HEIGHT);
        let mut col_gen = logup_gen.new_col();

        // Lookup values columns.
        for vec_row in 0..(1 << (RC_LOG_HEIGHT - LOG_N_LANES)) {
            let mut cum_num = PackedQM31::broadcast(M31(0).into());
            let mut cum_denom = PackedQM31::broadcast(M31(1).into());
            for rep in 0..N_REPETITIONS {
                let value = PackedBaseField::from_array(std::array::from_fn(|i| {
                    M31(((vec_row << LOG_N_LANES) + (rep << RC_LOG_HEIGHT) + i) as u32)
                }));
                let num: PackedQM31 =
                    (-self.multiplicities[rep][vec_row].as_m31_unchecked()).into();
                let denom: PackedQM31 = lookup_elements.combine(&[value]);
                (cum_num, cum_denom) = (cum_num * denom + num * cum_denom, cum_denom * denom);
            }
            col_gen.write_frac(vec_row, cum_num, cum_denom);
        }
        col_gen.finalize_col();
        logup_gen
    }

    pub fn write_interaction_trace(
        &self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
        lookup_elements: &RangeCheckElements,
    ) -> RangeCheckInteractionClaim<N_REPETITIONS> {
        let logup_gen = self.generate_interaction_trace(lookup_elements);
        let (trace, claimed_sum) = logup_gen.finalize();
        tree_builder.extend_evals(trace);

        RangeCheckInteractionClaim::<N_REPETITIONS> { claimed_sum }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use rand::Rng;
    use stwo_prover::constraint_framework::FrameworkEval;
    use stwo_prover::core::backend::simd::column::BaseColumn;
    use stwo_prover::core::backend::simd::SimdBackend;
    use stwo_prover::core::backend::Column;
    use stwo_prover::core::channel::Blake2sChannel;
    use stwo_prover::core::fields::m31::M31;
    use stwo_prover::core::pcs::{CommitmentSchemeProver, PcsConfig};
    use stwo_prover::core::poly::circle::{CanonicCoset, PolyOps};

    use super::RangeCheckClaimProver;
    use crate::components::range_check_unit::component::RangeCheckUnitComponent;
    use crate::components::range_check_unit::RangeCheckElements;

    #[test]
    fn test_generate_trace() {
        // 9 times 0..3; 8 times 3..17.
        let inputs: [M31; 17 * 8 + 3] =
            std::array::from_fn(|i| M31::from_u32_unchecked((i % 17) as u32));

        let mut rc_prover = RangeCheckClaimProver::<4, 4>::new();

        rc_prover.add_m31s(&inputs);

        let trace = rc_prover.generate_trace();

        let expected_trace = [
            // Indices:
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
            // Multiplicities:
            [9, 9, 9, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8],
            [8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0; 16],
            [0; 16],
        ]
        .map(|c| BaseColumn::from_iter(c.map(M31::from_u32_unchecked)));

        assert_eq!(trace.len(), 5);
        for i in 0..5 {
            assert_eq!(trace[i].to_cpu(), expected_trace[i].to_cpu());
        }
    }

    #[test]
    fn test_generate_interaction_trace() {
        let mut rng = rand::thread_rng();
        const LOG_HEIGHT: u32 = 6;
        const LOG_BLOWUP_FACTOR: u32 = 1;
        const N_REPS: u32 = 4;

        let twiddles = SimdBackend::precompute_twiddles(
            CanonicCoset::new(LOG_HEIGHT + LOG_BLOWUP_FACTOR)
                .circle_domain()
                .half_coset,
        );

        let channel = &mut Blake2sChannel::default();
        let config = PcsConfig::default();
        let commitment_scheme = &mut CommitmentSchemeProver::new(config, &twiddles);

        let inputs: [M31; 100] = std::array::from_fn(|_| {
            M31::from_u32_unchecked(rng.gen::<u32>() % (N_REPS << LOG_HEIGHT))
        });

        let mut rc_prover = RangeCheckClaimProver::<LOG_HEIGHT, { N_REPS as usize }>::new();
        rc_prover.add_m31s(&inputs);

        let mut tree_builder = commitment_scheme.tree_builder();
        let (_, interaction_claim_prover) = rc_prover.write_trace(&mut tree_builder);

        tree_builder.commit(channel);
        let mut tree_builder = commitment_scheme.tree_builder();

        let lookup_elements = RangeCheckElements::draw(channel);
        let interaction_claim =
            interaction_claim_prover.write_interaction_trace(&mut tree_builder, &lookup_elements);
        tree_builder.commit(channel);

        let component = RangeCheckUnitComponent::<{ N_REPS as usize }> {
            log_n_rows: LOG_HEIGHT,
            lookup_elements,
            claimed_sum: interaction_claim.claimed_sum,
        };

        let trace_polys = commitment_scheme
            .trees
            .as_ref()
            .map(|t| t.polynomials.iter().cloned().collect_vec());

        stwo_prover::constraint_framework::assert_constraints(
            &trace_polys,
            CanonicCoset::new(LOG_HEIGHT),
            |eval| {
                component.evaluate(eval);
            },
        )
    }
}
