use std::iter::zip;
use std::simd::Simd;

use itertools::{chain, izip, Itertools};
use prover_types::simd::PackedFelt252;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use stwo_prover::constraint_framework::logup::LogupTraceGenerator;
use stwo_prover::constraint_framework::Relation;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::m31::{PackedBaseField, PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::qm31::PackedQM31;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::{BackendForChannel, Column};
use stwo_prover::core::channel::MerkleChannel;
use stwo_prover::core::fields::m31::{BaseField, M31};
use stwo_prover::core::fields::qm31::QM31;
use stwo_prover::core::pcs::TreeBuilder;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;

use super::component::{
    Claim, InteractionClaim, BIG_N_ID_AND_VALUE_COLUMNS, MEMORY_ID_SIZE, N_M31_IN_SMALL_FELT252,
    SMALL_N_ID_AND_VALUE_COLUMNS,
};
use crate::components::memory::MEMORY_ADDRESS_BOUND;
use crate::components::range_check_vector::range_check_9_9;
use crate::components::AtomicMultiplicityColumn;
use crate::felt::split_f252_simd;
use crate::input::memory::{
    u128_to_4_limbs, EncodedMemoryValueId, Memory, MemoryValueId, LARGE_MEMORY_VALUE_ID_TAG,
};
use crate::relations;

pub type PackedInputType = PackedBaseField;
pub type InputType = BaseField;

/// Generates the trace and the claim for the id -> f252 memory table.
/// Generates 2 table, one for large values and one for small values. A large value is a full 28
/// limb Felt252. The small values are currently 8 limbs, for a maximum of 72 bits.
/// The separation is done to reduce zeroed out ('unused') trace cells.
pub struct ClaimGenerator {
    big_values: Vec<[u32; 8]>,
    big_mults: AtomicMultiplicityColumn,
    small_values: Vec<u128>,
    small_mults: AtomicMultiplicityColumn,
}
impl ClaimGenerator {
    pub fn new(mem: &Memory) -> Self {
        // TODO(spapini): More repetitions, for efficiency.
        let mut big_values = mem.f252_values.clone();
        let big_size = std::cmp::max(big_values.len().next_power_of_two(), N_LANES);
        big_values.resize(big_size, [0; 8]);
        let big_mults = AtomicMultiplicityColumn::new(big_size);

        let mut small_values = mem.small_values.clone();
        let small_size = std::cmp::max(small_values.len().next_power_of_two(), N_LANES);
        small_values.resize(small_size, 0);
        let small_mults = AtomicMultiplicityColumn::new(small_size);
        assert!(big_size + small_size <= MEMORY_ADDRESS_BOUND);

        Self {
            small_values,
            big_values,
            small_mults,
            big_mults,
        }
    }

    pub fn deduce_output(&self, ids: PackedM31) -> PackedFelt252 {
        let values = std::array::from_fn(|j| {
            Simd::from_array(
                ids.to_array()
                    .map(|M31(i)| match EncodedMemoryValueId(i).decode() {
                        MemoryValueId::F252(id) => self.big_values[id as usize][j],
                        MemoryValueId::Small(id) => {
                            if j >= 4 {
                                0
                            } else {
                                let small = self.small_values[id as usize];
                                u128_to_4_limbs(small)[j]
                            }
                        }
                    }),
            )
        });

        PackedFelt252 {
            value: split_f252_simd(values),
        }
    }

    pub fn add_inputs(&self, inputs: &[InputType]) {
        for &input in inputs {
            self.add_m31(input);
        }
    }

    pub fn add_packed_m31(&self, inputs: &PackedM31) {
        let memory_ids = inputs.to_array();
        for memory_id in memory_ids {
            self.add_m31(memory_id);
        }
    }

    pub fn add_m31(&self, M31(encoded_memory_id): M31) {
        match EncodedMemoryValueId(encoded_memory_id).decode() {
            MemoryValueId::F252(id) => {
                self.big_mults.increase_at(id);
            }
            MemoryValueId::Small(id) => {
                self.small_mults.increase_at(id);
            }
        }
    }

    pub fn write_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        range_check_9_9_trace_generator: &range_check_9_9::ClaimGenerator,
    ) -> (Claim, InteractionClaimGenerator)
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let big_table_trace = gen_big_memory_trace(self.big_values, self.big_mults.into_simd_vec());
        let small_table_trace =
            gen_small_memory_trace(self.small_values, self.small_mults.into_simd_vec());

        // Lookup data.
        let big_ids_and_values: [_; BIG_N_ID_AND_VALUE_COLUMNS] =
            std::array::from_fn(|i| big_table_trace[i].data.clone());
        let big_multiplicities = big_table_trace.last().unwrap().data.clone();
        let small_ids_and_values: [_; SMALL_N_ID_AND_VALUE_COLUMNS] =
            std::array::from_fn(|i| small_table_trace[i].data.clone());
        let small_multiplicities = small_table_trace.last().unwrap().data.clone();

        // Add inputs to range check that all the values are 9-bit felts.
        for (col0, col1) in big_ids_and_values[MEMORY_ID_SIZE..].iter().tuples() {
            col0.par_iter()
                .zip(col1.par_iter())
                .for_each(|(val0, val1)| {
                    range_check_9_9_trace_generator.add_packed_m31(&[*val0, *val1]);
                });
        }
        for (col0, col1) in small_ids_and_values[MEMORY_ID_SIZE..].iter().tuples() {
            col0.par_iter()
                .zip(col1.par_iter())
                .for_each(|(val0, val1)| {
                    range_check_9_9_trace_generator.add_packed_m31(&[*val0, *val1]);
                });
        }

        // Extend trace.
        let big_log_size = big_table_trace[0].len().ilog2();
        let trace = big_table_trace
            .into_iter()
            .map(|eval| {
                CircleEvaluation::<SimdBackend, BaseField, BitReversedOrder>::new(
                    CanonicCoset::new(big_log_size).circle_domain(),
                    eval,
                )
            })
            .collect_vec();
        tree_builder.extend_evals(trace);
        let small_log_size = small_table_trace[0].len().ilog2();
        let trace = small_table_trace
            .into_iter()
            .map(|eval| {
                CircleEvaluation::<SimdBackend, BaseField, BitReversedOrder>::new(
                    CanonicCoset::new(small_log_size).circle_domain(),
                    eval,
                )
            })
            .collect_vec();
        tree_builder.extend_evals(trace);

        (
            Claim {
                big_log_size,
                small_log_size,
            },
            InteractionClaimGenerator {
                big_ids_and_values,
                big_multiplicities,
                small_ids_and_values,
                small_multiplicities,
            },
        )
    }
}

// Generates the trace of the large value memory table.
fn gen_big_memory_trace(values: Vec<[u32; 8]>, mults: Vec<PackedM31>) -> Vec<BaseColumn> {
    let column_length = values.len();
    let packed_values = values
        .into_iter()
        .array_chunks::<N_LANES>()
        .map(|chunk| {
            std::array::from_fn(|i| Simd::from_array(std::array::from_fn(|j| chunk[j][i])))
        })
        .collect_vec();

    let mut id_and_value_trace =
        std::iter::repeat_with(|| unsafe { BaseColumn::uninitialized(column_length) })
            .take(BIG_N_ID_AND_VALUE_COLUMNS)
            .collect_vec();
    let inc = Simd::from_array(std::array::from_fn(|i| i as u32));
    for (i, values) in packed_values.iter().enumerate() {
        let values = split_f252_simd(*values);
        id_and_value_trace[0].data[i] = unsafe {
            PackedM31::from_simd_unchecked(
                Simd::splat((i * N_LANES) as u32 + LARGE_MEMORY_VALUE_ID_TAG) + inc,
            )
        };
        for (j, value) in values.iter().enumerate() {
            id_and_value_trace[j + 1].data[i] = *value;
        }
    }

    let multiplicities = BaseColumn::from_simd(mults);

    chain!(id_and_value_trace, [multiplicities]).collect_vec()
}

// Generates the trace of the small value memory table.
fn gen_small_memory_trace(values: Vec<u128>, mults: Vec<PackedM31>) -> Vec<BaseColumn> {
    let column_length = values.len();
    let packed_values: Vec<[Simd<u32, N_LANES>; 4]> = values
        .into_iter()
        .map(u128_to_4_limbs)
        .array_chunks::<N_LANES>()
        .map(|chunk| {
            std::array::from_fn(|i| Simd::from_array(std::array::from_fn(|j| chunk[j][i])))
        })
        .collect_vec();

    let mut id_and_value_trace =
        std::iter::repeat_with(|| unsafe { BaseColumn::uninitialized(column_length) })
            .take(SMALL_N_ID_AND_VALUE_COLUMNS)
            .collect_vec();
    let inc = Simd::from_array(std::array::from_fn(|i| i as u32));
    for (i, values) in packed_values.iter().enumerate() {
        let values = split_f252_simd([
            values[0],
            values[1],
            values[2],
            values[3],
            Simd::splat(0),
            Simd::splat(0),
            Simd::splat(0),
            Simd::splat(0),
        ]);
        id_and_value_trace[0].data[i] =
            unsafe { PackedM31::from_simd_unchecked(Simd::splat((i * N_LANES) as u32) + inc) };
        for (j, value) in values[..N_M31_IN_SMALL_FELT252].iter().enumerate() {
            id_and_value_trace[j + 1].data[i] = *value;
        }
    }

    let multiplicities = BaseColumn::from_simd(mults);

    chain!(id_and_value_trace, [multiplicities]).collect_vec()
}

#[derive(Debug)]
pub struct InteractionClaimGenerator {
    pub big_ids_and_values: [Vec<PackedM31>; BIG_N_ID_AND_VALUE_COLUMNS],
    pub big_multiplicities: Vec<PackedM31>,
    pub small_ids_and_values: [Vec<PackedM31>; SMALL_N_ID_AND_VALUE_COLUMNS],
    pub small_multiplicities: Vec<PackedM31>,
}
impl InteractionClaimGenerator {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            big_ids_and_values: std::array::from_fn(|_| Vec::with_capacity(capacity)),
            big_multiplicities: Vec::with_capacity(capacity),
            small_ids_and_values: std::array::from_fn(|_| Vec::with_capacity(capacity)),
            small_multiplicities: Vec::with_capacity(capacity),
        }
    }

    pub fn write_interaction_trace<MC: MerkleChannel>(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, MC>,
        lookup_elements: &relations::MemoryIdToBig,
        range9_9_lookup_elements: &relations::RangeCheck_9_9,
    ) -> InteractionClaim
    where
        SimdBackend: BackendForChannel<MC>,
    {
        let (big_trace, big_claimed_sum) =
            self.gen_big_memory_interaction_trace(lookup_elements, range9_9_lookup_elements);
        tree_builder.extend_evals(big_trace);

        let (small_trace, small_claimed_sum) =
            self.gen_small_memory_interaction_trace(lookup_elements, range9_9_lookup_elements);
        tree_builder.extend_evals(small_trace);

        InteractionClaim {
            small_claimed_sum,
            big_claimed_sum,
        }
    }

    fn gen_big_memory_interaction_trace(
        &self,
        lookup_elements: &relations::MemoryIdToBig,
        range9_9_lookup_elements: &relations::RangeCheck_9_9,
    ) -> (
        Vec<CircleEvaluation<SimdBackend, M31, BitReversedOrder>>,
        QM31,
    ) {
        let big_table_log_size = self.big_ids_and_values[0].len().ilog2() + LOG_N_LANES;
        let mut big_values_logup_gen = LogupTraceGenerator::new(big_table_log_size);

        // Every element is 9-bit.
        for (limb0, limb1, limb2, lim3) in self.big_ids_and_values[MEMORY_ID_SIZE..].iter().tuples()
        {
            let mut col_gen = big_values_logup_gen.new_col();
            for (vec_row, (limb0, limb1, limb2, limb3)) in
                izip!(limb0, limb1, limb2, lim3).enumerate()
            {
                let denom0: PackedQM31 = range9_9_lookup_elements.combine(&[*limb0, *limb1]);
                let denom1: PackedQM31 = range9_9_lookup_elements.combine(&[*limb2, *limb3]);
                col_gen.write_frac(vec_row, denom0 + denom1, denom0 * denom1);
            }
            col_gen.finalize_col();
        }

        // Yield large values.
        let mut col_gen = big_values_logup_gen.new_col();
        for vec_row in 0..1 << (big_table_log_size - LOG_N_LANES) {
            let values: [_; BIG_N_ID_AND_VALUE_COLUMNS] =
                std::array::from_fn(|i| self.big_ids_and_values[i][vec_row]);
            let denom: PackedQM31 = lookup_elements.combine(&values);
            col_gen.write_frac(vec_row, (-self.big_multiplicities[vec_row]).into(), denom);
        }
        col_gen.finalize_col();

        big_values_logup_gen.finalize_last()
    }

    fn gen_small_memory_interaction_trace(
        &self,
        lookup_elements: &relations::MemoryIdToBig,
        range9_9_lookup_elements: &relations::RangeCheck_9_9,
    ) -> (
        Vec<CircleEvaluation<SimdBackend, M31, BitReversedOrder>>,
        QM31,
    ) {
        let small_table_log_size = self.small_ids_and_values[0].len().ilog2() + LOG_N_LANES;
        let mut small_values_logup_gen = LogupTraceGenerator::new(small_table_log_size);

        // Every element is 9-bit.
        for (l, r) in self.small_ids_and_values[MEMORY_ID_SIZE..].iter().tuples() {
            let mut col_gen = small_values_logup_gen.new_col();
            for (vec_row, (l1, l2)) in zip(l, r).enumerate() {
                // TOOD(alont) Add 2-batching.
                col_gen.write_frac(
                    vec_row,
                    PackedQM31::broadcast(M31(1).into()),
                    range9_9_lookup_elements.combine(&[*l1, *l2]),
                );
            }
            col_gen.finalize_col();
        }

        // Yield small values.
        let mut col_gen = small_values_logup_gen.new_col();
        for vec_row in 0..1 << (small_table_log_size - LOG_N_LANES) {
            let values: [_; SMALL_N_ID_AND_VALUE_COLUMNS] =
                std::array::from_fn(|i| self.small_ids_and_values[i][vec_row]);
            let denom: PackedQM31 = lookup_elements.combine(&values);
            col_gen.write_frac(vec_row, (-self.small_multiplicities[vec_row]).into(), denom);
        }
        col_gen.finalize_col();

        small_values_logup_gen.finalize_last()
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use stwo_prover::core::backend::simd::m31::PackedM31;
    use stwo_prover::core::fields::m31::M31;

    use crate::components::memory::memory_address_to_id;
    use crate::components::memory::memory_id_to_big::component::N_M31_IN_FELT252;
    use crate::felt::split_f252;
    use crate::input::memory::{MemoryBuilder, MemoryConfig};

    #[test]
    fn test_deduce_output_simd() {
        // Set up data.
        let memory_addreses = [0, 1, 2, 3, 4, 5, 6, 7, 15, 16, 17, 18, 0, 0, 0, 0];
        let expected = memory_addreses
            .iter()
            .enumerate()
            .map(|(j, addr)| {
                let arr: [_; 8] =
                    std::array::from_fn(|i| if i > 0 && j % 2 == 0 { 0 } else { *addr });
                arr
            })
            .collect_vec();
        let input = PackedM31::from_array(memory_addreses.map(M31::from_u32_unchecked));

        // Create memory.
        let mut mem = MemoryBuilder::new(MemoryConfig::default());
        for (j, a) in memory_addreses.iter().enumerate() {
            mem.set(*a as u64, mem.value_from_felt252(expected[j]));
        }
        let mem = mem.build();
        let memory_address_to_id = memory_address_to_id::ClaimGenerator::new(&mem);
        let id_to_felt = super::ClaimGenerator::new(&mem);

        let id = memory_address_to_id.deduce_output(input);
        let output = id_to_felt.deduce_output(id).value;

        for (i, expected) in expected.into_iter().enumerate() {
            let expected = split_f252(expected);
            let value: [M31; N_M31_IN_FELT252] = (0..N_M31_IN_FELT252)
                .map(|j| output[j].to_array()[i])
                .collect_vec()
                .try_into()
                .unwrap();
            assert_eq!(value, expected);
        }
    }
}
