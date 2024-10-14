// TODO(Ohad): fix and test the code.
// NOTE: W.I.P, exists to make things compile.
use std::cmp::max;
use std::simd::Simd;

use itertools::Itertools;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::{Col, Column};
use stwo_prover::core::fields::m31::{BaseField, M31};
use stwo_prover::core::pcs::TreeBuilder;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;

use super::component::{RangeCheckClaim, RangeCheckInteractionClaim};
use super::RangeCheckVectorElements;
use crate::prover_types::PackedUInt32;

// TODO(Ohad): rustdoc.
pub struct RangeCheckClaimProver<const N: usize> {
    log_ranges: [u32; N],
    pub multiplicities: Vec<PackedUInt32>,
}
impl<const N: usize> RangeCheckClaimProver<N> {
    pub fn new(log_ranges: [u32; N]) -> Self {
        let multiplicities = vec![PackedUInt32::broadcast(0); 1 << log_ranges.iter().sum::<u32>()];
        Self {
            log_ranges,
            multiplicities,
        }
    }

    fn log_size(&self) -> u32 {
        self.log_ranges.iter().sum()
    }

    fn dense_representation(&self, values: [M31; N]) -> u32 {
        values
            .iter()
            .zip_eq(self.log_ranges.iter())
            .fold(
                (0, self.log_size()),
                |(acc, trailing_bits), (&val, &n_bits)| {
                    let trailing_bits = trailing_bits - n_bits;
                    (acc + (val.0 << trailing_bits), trailing_bits)
                },
            )
            .0
    }

    fn _sparse_representation(&self, mut dense: u32) -> [M31; N] {
        let mut trailing_bits = self.log_size();
        self.log_ranges
            .iter()
            .map(|&n_bits| {
                trailing_bits -= n_bits;
                let value = (!((1 << trailing_bits) - 1) & dense) >> trailing_bits;
                dense &= (1 << trailing_bits) - 1;
                value.into()
            })
            .collect_vec()
            .try_into()
            .unwrap()
    }

    // TODO(Ohad): test.
    pub fn add_m31(&mut self, input: [M31; N]) {
        let num = self.dense_representation(input) as usize;
        let (input_row, input_lane) = (num / N_LANES, num % N_LANES);
        self.multiplicities[input_row].simd[input_lane] += 1;
    }

    // TODO(Ohad): test.
    pub fn add_packed_m31(&mut self, input: &[PackedM31; N]) {
        for i in 0..N_LANES {
            self.add_m31(std::array::from_fn(|j| input[j].to_array()[i]));
        }
    }

    fn write_fixed_columns(&self) -> [BaseColumn; N] {
        let mut fixed_columns: [BaseColumn; N] = std::array::from_fn(|_| unsafe {
            Col::<SimdBackend, BaseField>::uninitialized(1 << self.log_size())
        });

        let mut trailing_bits = self.log_size();
        for (&n_bits, col) in self.log_ranges.iter().zip_eq(fixed_columns.iter_mut()) {
            trailing_bits -= n_bits;
            let (mut val, step) = compute_iv_and_step(trailing_bits);
            let log_step_size = max(trailing_bits as i64 - LOG_N_LANES as i64, 0);

            let mask = Simd::splat((1 << n_bits) - 1);
            for step_chunk in col.data.chunks_mut(1 << log_step_size) {
                val = unsafe { PackedM31::from_simd_unchecked(val.into_simd() & mask) };
                step_chunk.iter_mut().for_each(|v| {
                    *v = val;
                });
                val += step;
            }
        }
        fixed_columns
    }

    pub fn write_trace(
        &mut self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
    ) -> (RangeCheckClaim<N>, RangeCheckInteractionClaimProver<N>) {
        let multiplicity_column = BaseColumn {
            data: self
                .multiplicities
                .iter()
                .map(|m| m.as_m31_unchecked())
                .collect(),
            length: 1 << self.log_size(),
        };

        let domain = CanonicCoset::new(self.log_size()).circle_domain();
        let trace = self
            .write_fixed_columns()
            .into_iter()
            .chain([multiplicity_column])
            .map(|col| CircleEvaluation::<SimdBackend, _, BitReversedOrder>::new(domain, col));

        tree_builder.extend_evals(trace);

        let claim = RangeCheckClaim {
            log_ranges: self.log_ranges,
        };

        let interaction_claim_prover = RangeCheckInteractionClaimProver {
            log_ranges: self.log_ranges,
            multiplicities: self
                .multiplicities
                .iter()
                .map(|m| m.as_m31_unchecked())
                .collect(),
        };

        (claim, interaction_claim_prover)
    }
}

fn compute_iv_and_step(trailing_bits: u32) -> (PackedM31, PackedM31) {
    let (simd_iv, simd_step) = if trailing_bits >= LOG_N_LANES {
        (
            Simd::<u32, N_LANES>::splat(0),
            Simd::<u32, N_LANES>::splat(1),
        )
    } else {
        let log_m = LOG_N_LANES - trailing_bits;
        let iv = Simd::<u32, N_LANES>::from_array(
            (0..(1 << log_m))
                .map(|x| vec![x; 1 << (LOG_N_LANES - log_m)])
                .concat()
                .try_into()
                .unwrap(),
        );
        let step = Simd::<u32, N_LANES>::splat(1 << log_m);
        (iv, step)
    };

    (unsafe { PackedM31::from_simd_unchecked(simd_iv) }, unsafe {
        PackedM31::from_simd_unchecked(simd_step)
    })
}

#[derive(Debug)]
pub struct RangeCheckInteractionClaimProver<const N: usize> {
    pub log_ranges: [u32; N],
    pub multiplicities: Vec<PackedM31>,
}
impl<const N: usize> RangeCheckInteractionClaimProver<N> {
    pub fn with_capacity(capacity: usize, log_ranges: [u32; N]) -> Self {
        Self {
            multiplicities: Vec::with_capacity(capacity),
            log_ranges,
        }
    }

    pub fn write_interaction_trace(
        &self,
        _tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
        _lookup_elements: &RangeCheckVectorElements,
    ) -> RangeCheckInteractionClaim {
        // TODO(Ohad): implement.
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use rand::rngs::SmallRng;
    use rand::{Rng, SeedableRng};
    use stwo_prover::core::backend::Column;
    use stwo_prover::core::fields::m31::M31;

    use super::RangeCheckClaimProver;

    #[test]
    fn test_dense_representation() {
        let log_ranges = [8, 4, 3];
        let claim_prover = RangeCheckClaimProver::new(log_ranges);
        let mut rng = SmallRng::seed_from_u64(0);
        for _ in 0..10 {
            let rand = [
                rng.gen::<u32>() % (1 << 8),
                rng.gen::<u32>() % (1 << 4),
                rng.gen::<u32>() % (1 << 3),
            ];
            assert_eq!(
                claim_prover.dense_representation([M31(rand[0]), M31(rand[1]), M31(rand[2])]),
                rand[0] << 7 | rand[1] << 3 | rand[2]
            )
        }
    }

    #[test]
    fn test_sparse_representation() {
        let log_ranges = [8, 4, 3];
        let claim_prover = RangeCheckClaimProver::new(log_ranges);
        let mut rng = SmallRng::seed_from_u64(1);
        for _ in 0..10 {
            let rand = [
                rng.gen::<u32>() % (1 << 8),
                rng.gen::<u32>() % (1 << 4),
                rng.gen::<u32>() % (1 << 3),
            ];
            let dense = rand[0] << 7 | rand[1] << 3 | rand[2];
            assert_eq!(
                claim_prover._sparse_representation(dense),
                [M31(rand[0]), M31(rand[1]), M31(rand[2])]
            );
        }
    }

    #[test]
    fn test_write_trace() {
        let log_ranges = [5, 3, 3];
        let claim_prover = RangeCheckClaimProver::new(log_ranges);
        let trace = claim_prover
            .write_fixed_columns()
            .into_iter()
            .map(|col| col.to_cpu())
            .collect_vec();

        for i in 0..1 << claim_prover.log_size() {
            let vals = [trace[0][i], trace[1][i], trace[2][i]];
            assert_eq!(claim_prover.dense_representation(vals), i as u32);
        }
    }
}
