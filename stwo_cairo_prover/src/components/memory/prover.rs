use itertools::{zip_eq, Itertools};
use stwo_prover::constraint_framework::logup::{LogupTraceGenerator, LookupElements};
use stwo_prover::core::backend::simd::m31::{PackedBaseField, PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::qm31::PackedQM31;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::{Col, Column};
use stwo_prover::core::fields::m31::{BaseField, M31};
use stwo_prover::core::pcs::TreeBuilder;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleHasher;

use super::component::{
    MemoryClaim, MemoryInteractionClaim, LOG_MEMORY_ADDRESS_BOUND, MULTIPLICITY_COLUMN_OFFSET,
    N_M31_IN_FELT252, N_MEMORY_COLUMNS,
};
use super::MemoryValues;
use crate::prover_types::PackedUInt32;

pub const LOG_SIMD_MEMORY_ADDRESS_BOUND: u32 = LOG_MEMORY_ADDRESS_BOUND + LOG_N_LANES;
pub const SIMD_MEMORY_ADDRESS_BOUND: usize = 1 << LOG_SIMD_MEMORY_ADDRESS_BOUND;

pub struct MemoryClaimProver {
    pub values: Vec<[PackedM31; N_M31_IN_FELT252]>,
    pub multiplicities: Vec<PackedUInt32>,
}
impl MemoryClaimProver {
    pub fn new(_path: String) -> Self {
        // TODO(AlonH): change to read from file.
        let values = (0..SIMD_MEMORY_ADDRESS_BOUND)
            .map(|i| M31::from_u32_unchecked(i as u32))
            .array_chunks::<N_LANES>()
            .map(|c| {
                let value = PackedM31::from_array(c);
                [value; N_M31_IN_FELT252]
            })
            .collect();
        let multiplicities = vec![PackedUInt32::broadcast(0); SIMD_MEMORY_ADDRESS_BOUND / N_LANES];
        Self {
            values,
            multiplicities,
        }
    }

    pub fn from_info(info: MemoryValues) -> Self {
        let values = info.values;
        let multiplicities = vec![PackedUInt32::broadcast(0); values.len()];
        Self {
            values,
            multiplicities,
        }
    }

    pub fn deduce_output(&self, input: PackedM31) -> [PackedM31; N_M31_IN_FELT252] {
        let indices = input.to_array().map(|i| i.0 as usize);
        std::array::from_fn(|j| {
            PackedM31::from_array(indices.map(|i| {
                let packed_res = self.values[i / N_LANES];
                packed_res.map(|v| v.to_array()[i % N_LANES])[j]
            }))
        })
    }

    pub fn add_inputs(&mut self, inputs: &PackedM31) {
        let addresses = inputs.to_array();
        for address in addresses {
            let idx = address.0 as usize / N_LANES;
            self.multiplicities[idx].simd[address.0 as usize % N_LANES] += 1;
        }
    }

    pub fn write_trace(
        &mut self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleHasher>,
    ) -> (MemoryClaim, InteractionClaimProver) {
        let mut trace = (0..N_M31_IN_FELT252 + 2)
            .map(|_| Col::<SimdBackend, BaseField>::zeros(SIMD_MEMORY_ADDRESS_BOUND))
            .collect_vec();

        let inc = PackedBaseField::from_array(std::array::from_fn(|i| {
            M31::from_u32_unchecked((i) as u32)
        }));
        for (i, (values, multiplicity)) in zip_eq(&self.values, &self.multiplicities).enumerate() {
            // TODO(AlonH): Either create a constant column for the addresses and remove it from
            // here or add constraints to the column here.
            trace[0].data[i] =
                PackedM31::broadcast(M31::from_u32_unchecked((i * N_LANES) as u32)) + inc;
            for (j, value) in values.iter().enumerate() {
                trace[j + 1].data[i] = *value;
            }
            assert!(multiplicity.in_m31_range());
            trace[MULTIPLICITY_COLUMN_OFFSET].data[i] = multiplicity.as_m31_unchecked();
        }

        // Lookup data.
        let adresses_and_values = std::array::from_fn(|i| trace[i].data.clone());
        let multiplicities = trace[MULTIPLICITY_COLUMN_OFFSET].data.clone();

        // Extend trace.
        let domain = CanonicCoset::new(LOG_MEMORY_ADDRESS_BOUND + LOG_N_LANES).circle_domain();
        let trace = trace
            .into_iter()
            .map(|eval| CircleEvaluation::<SimdBackend, _, BitReversedOrder>::new(domain, eval))
            .collect_vec();
        tree_builder.extend_evals(trace);

        (
            MemoryClaim {
                log_address_bound: LOG_SIMD_MEMORY_ADDRESS_BOUND,
            },
            InteractionClaimProver {
                adresses_and_values,
                multiplicities,
            },
        )
    }
}

#[derive(Debug)]
pub struct InteractionClaimProver {
    pub adresses_and_values: [Vec<PackedM31>; N_M31_IN_FELT252 + 1],
    pub multiplicities: Vec<PackedM31>,
}
impl InteractionClaimProver {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            adresses_and_values: std::array::from_fn(|_| Vec::with_capacity(capacity)),
            multiplicities: Vec::with_capacity(capacity),
        }
    }

    pub fn write_interaction_trace(
        &self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleHasher>,
        lookup_elements: &LookupElements<N_MEMORY_COLUMNS>,
    ) -> MemoryInteractionClaim {
        let mut logup_gen = LogupTraceGenerator::new(LOG_SIMD_MEMORY_ADDRESS_BOUND);
        let mut col_gen = logup_gen.new_col();

        // Lookup values columns.
        for vec_row in 0..1 << (LOG_SIMD_MEMORY_ADDRESS_BOUND - LOG_N_LANES) {
            let values: [PackedM31; N_M31_IN_FELT252 + 1] =
                std::array::from_fn(|i| self.adresses_and_values[i][vec_row]);
            let denom: PackedQM31 = lookup_elements.combine(&values);
            col_gen.write_frac(vec_row, (-self.multiplicities[vec_row]).into(), denom);
        }
        col_gen.finalize_col();
        let (trace, claimed_sum) = logup_gen.finalize();
        tree_builder.extend_evals(trace);

        MemoryInteractionClaim { claimed_sum }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use stwo_prover::core::backend::simd::m31::PackedM31;
    use stwo_prover::core::fields::m31::M31;

    use crate::components::memory::component::N_M31_IN_FELT252;

    #[test]
    fn test_deduct_output_simd() {
        let generator = super::MemoryClaimProver::new("".to_string());
        let arr = [0, 1, 2, 3, 4, 5, 6, 7, 15, 16, 17, 18, 0, 0, 0, 0].map(M31::from_u32_unchecked);
        let input = PackedM31::from_array(arr);
        let expected_output = arr.map(|i| [i; N_M31_IN_FELT252]);

        let output = generator.deduce_output(input);

        for (i, expected) in expected_output.into_iter().enumerate() {
            let value: [M31; N_M31_IN_FELT252] = (0..N_M31_IN_FELT252)
                .map(|j| output[j].to_array()[i])
                .collect_vec()
                .try_into()
                .unwrap();
            assert_eq!(value, expected);
        }
    }
}
