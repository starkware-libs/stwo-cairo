use itertools::Itertools;
use stwo_prover::constraint_framework::logup::LogupTraceGenerator;
use stwo_prover::constraint_framework::Relation;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::m31::{PackedBaseField, PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::qm31::PackedSecureField;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::{Col, Column};
use stwo_prover::core::fields::m31::{BaseField, M31};
use stwo_prover::core::pcs::TreeBuilder;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;

use super::component::{Claim, InteractionClaim, N_ADDR_TO_ID_COLUMNS};
use crate::components::memory::MEMORY_ADDRESS_BOUND;
use crate::input::mem::Memory;
use crate::relations::AddrToIdRelation;

pub type PackedInputType = PackedM31;
pub type InputType = M31;

pub struct ClaimGenerator {
    pub ids: Vec<u32>,
    pub multiplicities: Vec<u32>,
}
impl ClaimGenerator {
    pub fn new(mem: &Memory) -> Self {
        let mut ids = (0..mem.address_to_id.len())
            .map(|addr| mem.get_raw_id(addr as u32))
            .collect_vec();
        let size = ids.len().next_power_of_two();
        assert!(size <= MEMORY_ADDRESS_BOUND);
        ids.resize(size, 0);

        let multiplicities = vec![0; size];
        Self {
            ids,
            multiplicities,
        }
    }

    pub fn deduce_output(&self, input: PackedBaseField) -> PackedBaseField {
        let indices = input.to_array().map(|i| i.0);
        let memory_ids = std::array::from_fn(|j| self.deduce_output_cpu(M31(indices[j])));
        PackedBaseField::from_array(memory_ids)
    }

    fn deduce_output_cpu(&self, input: BaseField) -> M31 {
        M31(self.ids[input.0 as usize])
    }

    pub fn add_inputs(&mut self, inputs: &[InputType]) {
        for input in inputs {
            self.add_m31(*input);
        }
    }

    pub fn add_packed_m31(&mut self, inputs: &PackedBaseField) {
        let addresses = inputs.to_array();
        for address in addresses {
            self.add_m31(address);
        }
    }

    pub fn add_m31(&mut self, addr: BaseField) {
        let addr = addr.0 as usize;
        self.multiplicities[addr] += 1;
    }

    pub fn write_trace(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
    ) -> (Claim, InteractionClaimGenerator) {
        let size = self.ids.len();
        let mut trace = (0..N_ADDR_TO_ID_COLUMNS)
            .map(|_| Col::<SimdBackend, BaseField>::zeros(size))
            .collect_vec();

        let inc = PackedBaseField::from_array(std::array::from_fn(|i| {
            BaseField::from_u32_unchecked((i) as u32)
        }));

        // TODO(Ohad): avoid copy.
        let packed_ids = self
            .ids
            .array_chunks::<N_LANES>()
            .map(|chunk| {
                PackedM31::from_array(std::array::from_fn(|i| M31::from_u32_unchecked(chunk[i])))
            })
            .collect_vec();

        // Replace with seq.
        for (i, id) in packed_ids.iter().enumerate() {
            trace[0].data[i] =
                PackedM31::broadcast(BaseField::from_u32_unchecked((i * N_LANES) as u32)) + inc;
            trace[1].data[i] = *id;
        }

        // TODO(Ohad): avoid copy
        trace[2] = BaseColumn::from_iter(
            self.multiplicities
                .clone()
                .into_iter()
                .map(BaseField::from_u32_unchecked),
        );

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
            Claim {
                log_size: log_address_bound,
            },
            InteractionClaimGenerator {
                addresses,
                ids,
                multiplicities,
            },
        )
    }
}

pub struct InteractionClaimGenerator {
    pub addresses: Vec<PackedM31>,
    pub ids: Vec<PackedM31>,
    pub multiplicities: Vec<PackedM31>,
}
impl InteractionClaimGenerator {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            addresses: Vec::with_capacity(capacity),
            ids: Vec::with_capacity(capacity),
            multiplicities: Vec::with_capacity(capacity),
        }
    }

    pub fn write_interaction_trace(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
        lookup_elements: &AddrToIdRelation,
    ) -> InteractionClaim {
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

        InteractionClaim { claimed_sum }
    }
}
