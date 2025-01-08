use std::iter::zip;
use std::simd::Simd;

use itertools::{izip, Itertools};
use stwo_prover::constraint_framework::logup::LogupTraceGenerator;
use stwo_prover::constraint_framework::preprocessed_columns::PreprocessedColumn;
use stwo_prover::constraint_framework::Relation;
use stwo_prover::core::backend::simd::m31::{PackedBaseField, PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::qm31::PackedQM31;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::{BackendForChannel, Col, Column};
use stwo_prover::core::channel::MerkleChannel;
use stwo_prover::core::fields::m31::{BaseField, M31};
use stwo_prover::core::pcs::TreeBuilder;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;

use super::component::{Claim, InteractionClaim, N_SPLIT_CHUNKS};
use crate::components::memory_address_to_id::component::{
    N_ID_AND_MULT_COLUMNS_PER_CHUNK, N_TRACE_COLUMNS,
};
use crate::components::AtomicMultiplicityColumn;
use crate::input::memory::Memory;
use crate::relations;

pub type PackedInputType = PackedM31;
pub type InputType = M31;

pub struct ClaimGenerator {
    ids: Vec<u32>,
    multiplicities: AtomicMultiplicityColumn,
}
impl ClaimGenerator {
    pub fn new(memory: &Memory) -> Self {
        let ids = (0..memory.address_to_id.len())
            .map(|addr| memory.get_raw_id(addr as u32))
            .collect_vec();
        let multiplicities = AtomicMultiplicityColumn::new(ids.len());

        Self {
            ids,
            multiplicities,
        }
    }

    pub fn deduce_output(&self, input: PackedBaseField) -> PackedBaseField {
        let indices = input.to_array().map(|i| i.0);
        let memory_ids = std::array::from_fn(|j| self.get_id(M31(indices[j])));
        PackedBaseField::from_array(memory_ids)
    }

    pub fn get_id(&self, input: BaseField) -> M31 {
        M31(self.ids[input.0 as usize])
    }

    pub fn add_inputs(&self, inputs: &[InputType]) {
        for input in inputs {
            self.add_m31(*input);
        }
    }

    pub fn add_packed_m31(&self, inputs: &PackedBaseField) {
        let addresses = inputs.to_array();
        for address in addresses {
            self.add_m31(address);
        }
    }

    pub fn add_m31(&self, addr: BaseField) {
        self.multiplicities.increase_at(addr.0);
    }

    pub fn write_trace<MC: MerkleChannel>(
        mut self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
    ) -> (Claim, InteractionClaimGenerator)
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let size = std::cmp::max(
            (self.ids.len() / N_SPLIT_CHUNKS).next_power_of_two(),
            N_LANES,
        );
        let n_packed_rows = size.div_ceil(N_LANES);
        let mut trace: [_; N_TRACE_COLUMNS] =
            std::array::from_fn(|_| Col::<SimdBackend, M31>::zeros(size));

        // Pad to a multiple of `N_LANES`.
        let next_multiple_of_16 = self.ids.len().next_multiple_of(16);
        self.ids.resize(next_multiple_of_16, 0);

        let id_it = self
            .ids
            .array_chunks::<N_LANES>()
            .map(|&chunk| unsafe { PackedM31::from_simd_unchecked(Simd::from_array(chunk)) });
        let multiplicities = self.multiplicities.into_simd_vec();

        for (i, (id, multiplicity)) in zip(id_it, multiplicities).enumerate() {
            let chunk_idx = i / n_packed_rows;
            let i = i % n_packed_rows;
            trace[chunk_idx * N_ID_AND_MULT_COLUMNS_PER_CHUNK].data[i] = id;
            trace[1 + chunk_idx * N_ID_AND_MULT_COLUMNS_PER_CHUNK].data[i] = multiplicity;
        }

        // Lookup data.
        let ids: [_; N_SPLIT_CHUNKS] =
            std::array::from_fn(|i| trace[i * N_ID_AND_MULT_COLUMNS_PER_CHUNK].data.clone());
        let multiplicities: [_; N_SPLIT_CHUNKS] =
            std::array::from_fn(|i| trace[1 + i * N_ID_AND_MULT_COLUMNS_PER_CHUNK].data.clone());

        // Commit on trace.
        let log_size = size.checked_ilog2().unwrap();
        let domain = CanonicCoset::new(log_size).circle_domain();
        let trace = trace
            .into_iter()
            .map(|eval| {
                CircleEvaluation::<SimdBackend, BaseField, BitReversedOrder>::new(domain, eval)
            })
            .collect_vec();
        tree_builder.extend_evals(trace);

        (
            Claim { log_size },
            InteractionClaimGenerator {
                ids,
                multiplicities,
            },
        )
    }
}

pub struct InteractionClaimGenerator {
    pub ids: [Vec<PackedM31>; N_SPLIT_CHUNKS],
    pub multiplicities: [Vec<PackedM31>; N_SPLIT_CHUNKS],
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        lookup_elements: &relations::MemoryAddressToId,
    ) -> InteractionClaim
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let packed_size = self.ids[0].len();
        let log_size = packed_size.ilog2() + LOG_N_LANES;
        let n_rows = 1 << log_size;
        let mut logup_gen = LogupTraceGenerator::new(log_size);

        for (i, ((ids0, mults0), (ids1, mults1))) in
            izip!(&self.ids, &self.multiplicities).tuples().enumerate()
        {
            let mut col_gen = logup_gen.new_col();
            for (vec_row, (&id0, &id1, &mult0, &mult1)) in
                izip!(ids0, ids1, mults0, mults1).enumerate()
            {
                let addr = PreprocessedColumn::Seq(log_size).packed_at(vec_row);
                let addr0 = addr + PackedM31::broadcast(M31(((i * 2) * n_rows) as u32));
                let addr1 = addr + PackedM31::broadcast(M31(((i * 2 + 1) * n_rows) as u32));
                let p0: PackedQM31 = lookup_elements.combine(&[addr0, id0]);
                let p1: PackedQM31 = lookup_elements.combine(&[addr1, id1]);
                col_gen.write_frac(vec_row, p0 * (-mult1) + p1 * (-mult0), p1 * p0);
            }
            col_gen.finalize_col();
        }

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use stwo_prover::core::fields::m31::{BaseField, M31};

    use crate::components::memory::memory_address_to_id;
    use crate::input::memory::{MemoryBuilder, MemoryConfig};
    use crate::input::vm_import::MemoryEntry;

    #[test]
    fn test_memory_multiplicities() {
        const N_ENTRIES: u64 = 10;
        let memory = MemoryBuilder::from_iter(
            MemoryConfig::default(),
            (0..N_ENTRIES).map(|i| MemoryEntry {
                address: i,
                value: [i as u32; 8],
            }),
        )
        .build();
        let memory_address_to_id_gen = memory_address_to_id::ClaimGenerator::new(&memory);
        let address_usages = [0, 1, 1, 2, 2, 2]
            .into_iter()
            .map(BaseField::from)
            .collect_vec();
        let expected_mults = [1, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].map(M31);

        address_usages.iter().for_each(|addr| {
            memory_address_to_id_gen.add_m31(*addr);
        });
        let actual_mults = memory_address_to_id_gen.multiplicities.into_simd_vec();

        assert_eq!(actual_mults.len(), 1);
        assert_eq!(actual_mults[0].to_array(), expected_mults);
    }
}
