use std::simd::Simd;

use itertools::{zip_eq, Itertools};
use stwo_prover::constraint_framework::logup::LogupTraceGenerator;
use stwo_prover::core::backend::simd::m31::{PackedBaseField, PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::qm31::PackedQM31;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::{Col, Column};
use stwo_prover::core::fields::m31::{BaseField, M31};
use stwo_prover::core::pcs::TreeBuilder;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;

use super::component::{
    MemoryClaim, MemoryInteractionClaim, MEMORY_ADDRESS_BOUND, MEMORY_ADDRESS_SIZE,
    MULTIPLICITY_COLUMN_OFFSET, N_M31_IN_FELT252,
};
use super::MemoryLookupElements;
use crate::components::range_check_unit::RangeCheckElements;
use crate::felt::split_f252_simd;
use crate::input::mem::{Memory, MemoryValue};
use crate::prover_types::PackedUInt32;

pub struct MemoryClaimProver {
    pub values: Vec<[Simd<u32, N_LANES>; 8]>,
    pub multiplicities: Vec<PackedUInt32>,
}
impl MemoryClaimProver {
    pub fn new(mem: Memory) -> Self {
        // TODO(spapini): Split to multiple components.
        // TODO(spapini): More repetitions, for efficiency.
        let mut values = (0..mem.address_to_id.len())
            .map(|addr| mem.get(addr as u32).as_u256())
            .collect_vec();

        let size = values.len().next_power_of_two();
        assert!(size <= MEMORY_ADDRESS_BOUND);
        values.resize(size, MemoryValue::U64(0).as_u256());

        let values = values
            .into_iter()
            .array_chunks::<N_LANES>()
            .map(|chunk| {
                std::array::from_fn(|i| Simd::from_array(std::array::from_fn(|j| chunk[j][i])))
            })
            .collect_vec();

        let multiplicities = vec![PackedUInt32::broadcast(0); values.len()];
        Self {
            values,
            multiplicities,
        }
    }

    pub fn deduce_output(&self, input: PackedM31) -> [PackedM31; N_M31_IN_FELT252] {
        let indices = input.to_array().map(|i| i.0 as usize);
        let values = std::array::from_fn(|j| {
            Simd::from_array(indices.map(|i| {
                let packed_res = self.values[i / N_LANES];
                packed_res.map(|v| v.to_array()[i % N_LANES])[j]
            }))
        });
        split_f252_simd(values)
    }

    pub fn add_inputs_simd(&mut self, inputs: &PackedM31) {
        let addresses = inputs.to_array();
        for address in addresses {
            self.add_inputs(address);
        }
    }

    pub fn add_inputs(&mut self, addr: M31) {
        let addr = addr.0 as usize;
        self.multiplicities[addr / N_LANES].simd[addr % N_LANES] += 1;
    }

    pub fn write_trace(
        &mut self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
    ) -> (MemoryClaim, InteractionClaimProver) {
        let size = self.values.len() * N_LANES;
        let mut trace = (0..N_M31_IN_FELT252 + 2)
            .map(|_| Col::<SimdBackend, BaseField>::zeros(size))
            .collect_vec();

        let inc = PackedBaseField::from_array(std::array::from_fn(|i| {
            M31::from_u32_unchecked((i) as u32)
        }));
        for (i, (values, multiplicity)) in zip_eq(&self.values, &self.multiplicities).enumerate() {
            let values = split_f252_simd(*values);
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
        let addresses_and_values = std::array::from_fn(|i| trace[i].data.clone());
        let multiplicities = trace[MULTIPLICITY_COLUMN_OFFSET].data.clone();

        // Extend trace.
        let log_address_bound = size.checked_ilog2().unwrap();
        let domain = CanonicCoset::new(log_address_bound).circle_domain();
        let trace = trace
            .into_iter()
            .map(|eval| CircleEvaluation::<SimdBackend, _, BitReversedOrder>::new(domain, eval))
            .collect_vec();
        tree_builder.extend_evals(trace);

        (
            MemoryClaim { log_address_bound },
            InteractionClaimProver {
                addresses_and_values,
                multiplicities,
            },
        )
    }
}

#[derive(Debug)]
pub struct InteractionClaimProver {
    pub addresses_and_values: [Vec<PackedM31>; N_M31_IN_FELT252 + 1],
    pub multiplicities: Vec<PackedM31>,
}
impl InteractionClaimProver {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            addresses_and_values: std::array::from_fn(|_| Vec::with_capacity(capacity)),
            multiplicities: Vec::with_capacity(capacity),
        }
    }

    pub fn write_interaction_trace(
        &self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
        lookup_elements: &MemoryLookupElements,
        range9_lookup_elements: &RangeCheckElements,
    ) -> MemoryInteractionClaim {
        let log_size = self.addresses_and_values[0].len().ilog2() + LOG_N_LANES;
        let mut logup_gen = LogupTraceGenerator::new(log_size);
        let mut col_gen = logup_gen.new_col();

        // Lookup values columns.
        for vec_row in 0..1 << (log_size - LOG_N_LANES) {
            let values: [PackedM31; N_M31_IN_FELT252 + 1] =
                std::array::from_fn(|i| self.addresses_and_values[i][vec_row]);
            let denom: PackedQM31 = lookup_elements.combine(&values);
            col_gen.write_frac(vec_row, (-self.multiplicities[vec_row]).into(), denom);
        }
        col_gen.finalize_col();

        for value_col in self.addresses_and_values.iter().skip(MEMORY_ADDRESS_SIZE) {
            let mut col_gen = logup_gen.new_col();
            for (vec_row, value) in value_col.iter().enumerate() {
                // TOOD(alont) Add 2-batching.
                col_gen.write_frac(
                    vec_row,
                    PackedQM31::broadcast(M31(1).into()),
                    range9_lookup_elements.combine(&[PackedQM31::from(*value)]),
                );
            }
            col_gen.finalize_col();
        }
        let (trace, claimed_sum) = logup_gen.finalize();
        tree_builder.extend_evals(trace);

        MemoryInteractionClaim { claimed_sum }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use num_traits::Zero;
    use stwo_prover::core::backend::simd::m31::PackedM31;
    use stwo_prover::core::fields::m31::M31;

    use crate::components::memory::component::N_M31_IN_FELT252;
    use crate::input::mem::{MemConfig, MemoryBuilder};
    use crate::input::range_check_unit::RangeCheckUnitInput;

    #[test]
    fn test_deduce_output_simd() {
        // Set up data.
        let addr = [0, 1, 2, 3, 4, 5, 6, 7, 15, 16, 17, 18, 0, 0, 0, 0];
        let input = PackedM31::from_array(addr.map(M31::from_u32_unchecked));
        let expected_output = input
            .to_array()
            .map(|v| std::array::from_fn(|i| if i == 0 { v } else { M31::zero() }));

        // Create memory.
        let mut range_check9 = RangeCheckUnitInput::new();
        let mut mem = MemoryBuilder::new(MemConfig::default(), &mut range_check9);
        for a in &addr {
            let arr = std::array::from_fn(|i| if i == 0 { *a } else { 0 });
            mem.set(*a as u64, mem.value_from_felt252(arr));
        }
        let generator = super::MemoryClaimProver::new(mem.build());
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
