use bytemuck::Zeroable;
use itertools::{zip_eq, Itertools};
use num_traits::Zero;
use stwo_prover::constraint_framework::logup::LogupTraceGenerator;
use stwo_prover::core::backend::simd::m31::{PackedBaseField, PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::qm31::PackedSecureField;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::{Col, Column};
use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::core::pcs::TreeBuilder;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;

use super::component::{AddrToIdClaim, AddrToIdInteractionClaim, N_ADDR_TO_ID_COLUMNS};
use super::AddrToIdLookupElements;
use crate::components::memory::MEMORY_ADDRESS_BOUND;
use crate::input::mem::Memory;
use crate::prover_types::PackedUInt32;

pub struct MemoryAddrToIdClaimProver {
    pub ids: Vec<PackedBaseField>,
    pub multiplicities: Vec<PackedUInt32>,
}
impl MemoryAddrToIdClaimProver {
    pub fn new(mem: Memory) -> Self {
        let mut ids = (0..mem.address_to_id.len())
            .map(|addr| BaseField::from_u32_unchecked(mem.get_id(addr as u32)))
            .collect_vec();
        let size = ids.len().next_power_of_two();
        assert!(size <= MEMORY_ADDRESS_BOUND);
        ids.resize(size, BaseField::zero());

        let packed_ids = ids
            .into_iter()
            .array_chunks::<N_LANES>()
            .map(PackedBaseField::from_array)
            .collect_vec();

        let multiplicities = vec![PackedUInt32::zeroed(); packed_ids.len()];
        Self {
            ids: packed_ids,
            multiplicities,
        }
    }

    pub fn deduce_output(&self, input: PackedBaseField) -> PackedBaseField {
        let indices = input.to_array().map(|i| i.0 as usize);
        let memory_ids = std::array::from_fn(|j| {
            self.ids[indices[j] / N_LANES].to_array()[indices[j] % N_LANES]
        });
        PackedBaseField::from_array(memory_ids)
    }

    pub fn add_inputs_simd(&mut self, inputs: &PackedBaseField) {
        let addresses = inputs.to_array();
        for address in addresses {
            self.add_inputs(address);
        }
    }

    pub fn add_inputs(&mut self, addr: BaseField) {
        let addr = addr.0 as usize;
        self.multiplicities[addr / N_LANES].simd[addr % N_LANES] += 1;
    }

    pub fn write_trace(
        &mut self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
    ) -> (AddrToIdClaim, AddrToIdInteractionClaimProver) {
        let size = self.ids.len() * N_LANES;
        let mut trace = (0..N_ADDR_TO_ID_COLUMNS)
            .map(|_| Col::<SimdBackend, BaseField>::zeros(size))
            .collect_vec();

        let inc = PackedBaseField::from_array(std::array::from_fn(|i| {
            BaseField::from_u32_unchecked((i) as u32)
        }));

        for (i, (id, multiplicity)) in zip_eq(&self.ids, &self.multiplicities).enumerate() {
            trace[0].data[i] =
                PackedM31::broadcast(BaseField::from_u32_unchecked((i * N_LANES) as u32)) + inc;
            trace[1].data[i] = *id;
            trace[2].data[i] = multiplicity.as_m31_unchecked();
        }

        // Lookup data.
        let addresses = trace[0].data.clone();
        let ids = trace[1].data.clone();
        let multiplicities = trace[2].data.clone();

        // Commit on trace.
        let log_address_bound = size.checked_ilog2().unwrap();
        let domain = CanonicCoset::new(log_address_bound).circle_domain();
        let trace = trace
            .into_iter()
            .map(|eval| CircleEvaluation::<SimdBackend, _, BitReversedOrder>::new(domain, eval))
            .collect_vec();
        tree_builder.extend_evals(trace);

        (
            AddrToIdClaim {
                log_size: log_address_bound,
            },
            AddrToIdInteractionClaimProver {
                addresses,
                ids,
                multiplicities,
            },
        )
    }
}

pub struct AddrToIdInteractionClaimProver {
    pub addresses: Vec<PackedM31>,
    pub ids: Vec<PackedM31>,
    pub multiplicities: Vec<PackedM31>,
}
impl AddrToIdInteractionClaimProver {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            addresses: Vec::with_capacity(capacity),
            ids: Vec::with_capacity(capacity),
            multiplicities: Vec::with_capacity(capacity),
        }
    }

    pub fn write_interaction_trace(
        &self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
        lookup_elements: &AddrToIdLookupElements,
    ) -> AddrToIdInteractionClaim {
        let log_size = self.addresses.len().ilog2() + LOG_N_LANES;
        let mut logup_gen = LogupTraceGenerator::new(log_size);

        let mut col_gen = logup_gen.new_col();
        for vec_row in 0..1 << (log_size - LOG_N_LANES) {
            let addr = self.addresses[vec_row];
            let id = self.ids[vec_row];
            let denom: PackedSecureField = lookup_elements.combine(&[addr, id]);
            col_gen.write_frac(vec_row, (-self.multiplicities[vec_row]).into(), denom);
        }
        col_gen.finalize_col();

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        AddrToIdInteractionClaim { claimed_sum }
    }
}
