use std::iter::zip;
use std::simd::Simd;

use itertools::{izip, Itertools};
use stwo_prover::constraint_framework::logup::LogupTraceGenerator;
use stwo_prover::constraint_framework::Relation;
use stwo_prover::core::backend::simd::m31::{PackedBaseField, PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::{Col, Column};
use stwo_prover::core::fields::m31::{BaseField, M31};
use stwo_prover::core::pcs::TreeBuilder;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;

use super::component::{Claim, InteractionClaim, SPLIT_SIZE};
use crate::components::memory_address_to_id::component::{
    N_ID_AND_MULT_COLUMNS_PER_CHUNK, N_TRACE_COLUMNS,
};
use crate::input::mem::Memory;
use crate::relations;

pub type PackedInputType = PackedM31;
pub type InputType = M31;

pub struct ClaimGenerator {
    pub ids: Vec<u32>,
    pub multiplicities: Vec<u32>,
}
impl ClaimGenerator {
    pub fn new(mem: &Memory) -> Self {
        let ids = (0..mem.address_to_id.len())
            .map(|addr| mem.get_raw_id(addr as u32))
            .collect_vec();

        let multiplicities = vec![0; ids.len()];
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
        mut self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
    ) -> (Claim, InteractionClaimGenerator) {
        let size = (self.ids.len() / SPLIT_SIZE).next_power_of_two();
        let packed_size = size.div_ceil(N_LANES);
        let mut trace: [_; N_TRACE_COLUMNS] =
            std::array::from_fn(|_| Col::<SimdBackend, M31>::zeros(size));

        // Pad to a multiple of `N_LANES`.
        let next_multiple_of_16 = ((self.ids.len() + 15) >> 4) << 4;
        self.ids.resize(next_multiple_of_16, 0);
        self.multiplicities.resize(next_multiple_of_16, 0);

        // TODO(Ohad): avoid copy.
        let id_it = self
            .ids
            .array_chunks::<N_LANES>()
            .map(|&chunk| unsafe { PackedM31::from_simd_unchecked(Simd::from_array(chunk)) });
        let multiplicities_it = self
            .multiplicities
            .array_chunks::<N_LANES>()
            .map(|&chunk| unsafe { PackedM31::from_simd_unchecked(Simd::from_array(chunk)) });

        let inc =
            PackedM31::from_array(std::array::from_fn(|i| M31::from_u32_unchecked((i) as u32)));
        for i in 0..packed_size {
            trace[0].data[i] =
                inc + PackedM31::broadcast(M31::from_u32_unchecked((i * N_LANES) as u32));
        }

        // TODO(Ohad): Replace with seq.
        for (i, (id, multiplicity)) in zip(id_it, multiplicities_it).enumerate() {
            let chunk_idx = i / packed_size;
            let i = i % packed_size;
            trace[1 + chunk_idx * N_ID_AND_MULT_COLUMNS_PER_CHUNK].data[i] = id;
            trace[2 + chunk_idx * N_ID_AND_MULT_COLUMNS_PER_CHUNK].data[i] = multiplicity;
        }

        // Lookup data.
        let addresses = trace[0].data.clone();
        let ids: [_; SPLIT_SIZE] =
            std::array::from_fn(|i| trace[1 + i * N_ID_AND_MULT_COLUMNS_PER_CHUNK].data.clone());
        let multiplicities: [_; SPLIT_SIZE] =
            std::array::from_fn(|i| trace[2 + i * N_ID_AND_MULT_COLUMNS_PER_CHUNK].data.clone());

        // Commit on trace.
        let log_size = size.checked_ilog2().unwrap();
        let domain = CanonicCoset::new(log_size).circle_domain();
        let trace = trace
            .into_iter()
            .map(|eval| CircleEvaluation::<SimdBackend, _, BitReversedOrder>::new(domain, eval))
            .collect_vec();
        tree_builder.extend_evals(trace);

        (
            Claim { log_size },
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
    pub ids: [Vec<PackedM31>; SPLIT_SIZE],
    pub multiplicities: [Vec<PackedM31>; SPLIT_SIZE],
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
        lookup_elements: &relations::MemoryAddressToId,
    ) -> InteractionClaim {
        let packed_size = self.addresses.len();
        let log_size = packed_size.ilog2() + LOG_N_LANES;
        let mut logup_gen = LogupTraceGenerator::new(log_size);

        for (i, (ids, multiplicities)) in izip!(&self.ids, &self.multiplicities).enumerate() {
            let mut col_gen = logup_gen.new_col();
            for (vec_row, (&addr, &id, &mult)) in
                izip!(&self.addresses, ids, multiplicities).enumerate()
            {
                let addr = addr + PackedM31::broadcast(M31((i * (1 << log_size)) as u32));
                let denom = lookup_elements.combine(&[addr, id]);
                col_gen.write_frac(vec_row, (-mult).into(), denom);
            }
            col_gen.finalize_col();
        }

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}
