use std::iter::zip;
use std::ops::Index;
use std::simd::Simd;

use cairo_air::components::memory_address_to_id::{
    Claim, InteractionClaim, MEMORY_ADDRESS_TO_ID_SPLIT, N_ID_AND_MULT_COLUMNS_PER_CHUNK,
    N_TRACE_COLUMNS,
};
use cairo_air::preprocessed::Seq;
use cairo_air::relations;
use itertools::{izip, Itertools};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use stwo_cairo_adapter::memory::Memory;
use stwo_prover::constraint_framework::logup::LogupTraceGenerator;
use stwo_prover::constraint_framework::Relation;
use stwo_prover::core::backend::simd::m31::{PackedBaseField, PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::qm31::PackedQM31;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::{Col, Column};
use stwo_prover::core::fields::m31::{BaseField, M31};
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_cairo_common::prover_types::cpu::Relocatable;
use stwo_cairo_common::prover_types::simd::PackedRelocatable;
use crate::witness::utils::{TreeBuilder, AtomicMultiplicityColumn2D};

pub type InputType = M31;
pub type PackedInputType = PackedM31;

/// A struct that represents a mapping from Address to ID. Zero address is not allowed.
pub struct RelocatableToId {
    /// Since zero address is reserved, the vector holding the data is offset by 1, i.e. the ID of
    /// address 1 is stored at index 0, and so on.
    pub data: Vec<Vec<u32>>,
    pub flattened_data: Option<Vec<u32>>,
}
impl RelocatableToId {
    pub fn new(data: Vec<Vec<u32>>) -> Self {
        Self { data, flattened_data: None }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn resize(&mut self, new_len: Vec<usize>, value: u32) {
        self.data.iter_mut().zip(new_len).for_each(|(segment, new_len)| {
            segment.resize(new_len, value);
        });
    }

    pub fn flatten(&mut self) {
        self.flattened_data = Some(self.data.iter().flatten().copied().collect());
    }

    pub fn array_chunks<const N: usize>(&self) -> impl Iterator<Item = &[u32; N]> {
        match &self.flattened_data {
            Some(flattened_data) => flattened_data.array_chunks::<N>(),
            None => panic!("Flattened data is not set"),
        }
    }
}

impl Index<Relocatable> for RelocatableToId {
    type Output = u32;

    fn index(&self, relocatable: Relocatable) -> &Self::Output {
        &self.data[relocatable.segment_index][relocatable.offset as usize]
    }
}

/// A struct to generate the memory address to ID trace.
pub struct ClaimGenerator {
    relocatable_to_id: RelocatableToId,
    multiplicities: AtomicMultiplicityColumn2D,
}
impl ClaimGenerator {
    pub fn new(memory: &Memory) -> Self {
        // Note that while `memory.address_to_id` starts from address 0, the memory component can
        // only yield addresses starting from 1.
        let relocatable_to_id = RelocatableToId::new(
            memory
                .relocatable_to_id
                .iter()
                .enumerate()
                .map(|(segment_index, segment)| {
                    (0..segment.len()).map(|offset| memory.get_raw_id(Relocatable {
                        segment_index,
                        offset: offset as u32,
                    })).collect_vec()
                }).collect_vec(),
        );
        let multiplicities = AtomicMultiplicityColumn2D::new(&relocatable_to_id);

        Self {
            relocatable_to_id,
            multiplicities,
        }
    }

    pub fn deduce_output(&self, input: PackedRelocatable) -> PackedBaseField {
        let indices = input.to_array();
        let memory_ids = std::array::from_fn(|j| self.get_id(indices[j]));
        PackedBaseField::from_array(memory_ids)
    }

    pub fn get_id(&self, input: Relocatable) -> M31 {
        M31(self.relocatable_to_id[input])
    }

    pub fn add_inputs(&self, inputs: &[Relocatable]) {
        for input in inputs {
            self.add_input(input);
        }
    }

    pub fn add_packed_inputs(&self, inputs: &[PackedRelocatable]) {
        inputs.into_par_iter().for_each(|input| {
            self.add_packed_m31(input);
        });
    }

    pub fn add_packed_m31(&self, inputs: &PackedRelocatable) {
        let addresses = inputs.to_array();
        for address in addresses {
            self.add_input(&address);
        }
    }

    pub fn add_input(&self, addr: &Relocatable) {
        // Addresses are offset by 1.
        self.multiplicities.increase_at(addr);
    }

    pub fn write_trace(
        mut self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
    ) -> (Claim, InteractionClaimGenerator) {
        let size = std::cmp::max(
            (self
                .relocatable_to_id
                .len()
                .div_ceil(MEMORY_ADDRESS_TO_ID_SPLIT))
            .next_power_of_two(),
            N_LANES,
        );
        let n_packed_rows = size.div_ceil(N_LANES);
        let mut trace: [_; N_TRACE_COLUMNS] =
            std::array::from_fn(|_| Col::<SimdBackend, M31>::zeros(size));

        // Pad to a multiple of `N_LANES`.
        let next_multiples_of_16 = self.relocatable_to_id.data.iter().map(|segment| segment.len().next_multiple_of(16)).collect_vec();
        self.relocatable_to_id.resize(next_multiples_of_16, 0);
        self.relocatable_to_id.flatten();

        let id_it = self
            .relocatable_to_id
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
        let ids: [_; MEMORY_ADDRESS_TO_ID_SPLIT] =
            std::array::from_fn(|i| trace[i * N_ID_AND_MULT_COLUMNS_PER_CHUNK].data.clone());
        let multiplicities: [_; MEMORY_ADDRESS_TO_ID_SPLIT] =
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
    pub ids: [Vec<PackedM31>; MEMORY_ADDRESS_TO_ID_SPLIT],
    pub multiplicities: [Vec<PackedM31>; MEMORY_ADDRESS_TO_ID_SPLIT],
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        lookup_elements: &relations::MemoryAddressToId,
    ) -> InteractionClaim {
        let packed_size = self.ids[0].len();
        let log_size = packed_size.ilog2() + LOG_N_LANES;
        let n_rows = 1 << log_size;
        let mut logup_gen = LogupTraceGenerator::new(log_size);

        for (i, ((ids0, mults0), (ids1, mults1))) in
            izip!(&self.ids, &self.multiplicities).tuples().enumerate()
        {
            let mut col_gen = logup_gen.new_col();
            (col_gen.par_iter_mut(), ids0, ids1, mults0, mults1)
                .into_par_iter()
                .enumerate()
                .for_each(|(vec_row, (writer, &id0, &id1, &mult0, &mult1))| {
                    let addr = Seq::new(log_size).packed_at(vec_row) + PackedM31::broadcast(M31(1));
                    let addr0 = addr + PackedM31::broadcast(M31(((i * 2) * n_rows) as u32));
                    let addr1 = addr + PackedM31::broadcast(M31(((i * 2 + 1) * n_rows) as u32));
                    let p0: PackedQM31 = lookup_elements.combine(&[addr0, id0]);
                    let p1: PackedQM31 = lookup_elements.combine(&[addr1, id1]);
                    writer.write_frac(p0 * (-mult1) + p1 * (-mult0), p1 * p0);
                });
            col_gen.finalize_col();
        }

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}

// #[cfg(test)]
// mod tests {
//     use itertools::Itertools;
//     use stwo_cairo_adapter::memory::{MemoryBuilder, MemoryConfig, MemoryEntry};
//     use stwo_prover::core::fields::m31::{BaseField, M31};

//     use crate::witness::components::memory_address_to_id;

//     #[test]
//     fn test_memory_multiplicities() {
//         const N_ENTRIES: u32 = 10;
//         let (memory, ..) = MemoryBuilder::from_iter(
//             MemoryConfig::default(),
//             (0..N_ENTRIES).map(|i| MemoryEntry {
//                 address: i as u64,
//                 value: [i; 8],
//             }),
//         )
//         .build();
//         let memory_address_to_id_gen = memory_address_to_id::ClaimGenerator::new(&memory);
//         let address_usages = [1, 1, 2, 2, 2, 3]
//             .into_iter()
//             .map(BaseField::from)
//             .collect_vec();
//         // Multiplicites are of addresses offseted by 1.
//         let expected_mults = [2, 3, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].map(M31);

//         address_usages.iter().for_each(|addr| {
//             memory_address_to_id_gen.add_input(addr);
//         });
//         let actual_mults = memory_address_to_id_gen.multiplicities.into_simd_vec();

//         assert_eq!(actual_mults.len(), 1);
//         assert_eq!(actual_mults[0].to_array(), expected_mults);
//     }
// }
