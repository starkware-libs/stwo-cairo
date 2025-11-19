use std::simd::Simd;

use cairo_air::components::memory_id_to_big::{Claim, InteractionClaim, MEMORY_ID_SIZE};
use cairo_air::relations;
use itertools::{chain, Itertools};
use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
};
use stwo_cairo_adapter::memory::{u128_to_4_limbs, EncodedMemoryValueId, Memory, MemoryValueId};
use stwo_cairo_common::memory::{
    LARGE_MEMORY_VALUE_ID_BASE, N_M31_IN_FELT252, N_M31_IN_SMALL_FELT252,
};
use stwo_cairo_common::preprocessed_columns::preprocessed_utils::SIMD_ENUMERATION_0;
use stwo_cairo_common::prover_types::felt::split_f252_simd;
use stwo_cairo_common::prover_types::simd::PackedFelt252;

use crate::witness::components::{
    range_check_9_9, range_check_9_9_b, range_check_9_9_c, range_check_9_9_d, range_check_9_9_e,
    range_check_9_9_f, range_check_9_9_g, range_check_9_9_h,
};
use crate::witness::prelude::*;
use crate::witness::utils::{AtomicMultiplicityColumn, TreeBuilder};

pub type InputType = M31;
pub type PackedInputType = PackedM31;

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
        let mut big_values = mem.f252_values.clone();
        let simd_padded_big_size = big_values.len().next_multiple_of(N_LANES);
        big_values.resize(simd_padded_big_size, [0; 8]);
        let big_mults = AtomicMultiplicityColumn::new(simd_padded_big_size);

        let mut small_values = mem.small_values.clone();
        let simd_padded_small_size = small_values.len().next_multiple_of(N_LANES);
        small_values.resize(simd_padded_small_size, 0);
        let small_mults = AtomicMultiplicityColumn::new(simd_padded_small_size);

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
                        MemoryValueId::Empty => {
                            panic!("Attempted deduce_output on empty memory cell.")
                        }
                    }),
            )
        });

        PackedFelt252 {
            value: split_f252_simd(values),
        }
    }

    pub fn add_inputs(&self, inputs: &[InputType]) {
        for input in inputs {
            self.add_input(input);
        }
    }

    pub fn add_packed_inputs(&self, inputs: &[PackedInputType]) {
        inputs.into_par_iter().for_each(|input| {
            self.add_packed_m31(input);
        });
    }

    pub fn add_packed_m31(&self, inputs: &PackedM31) {
        let memory_ids = inputs.to_array();
        for memory_id in memory_ids {
            self.add_input(&memory_id);
        }
    }

    pub fn add_input(&self, encoded_memory_id: &M31) {
        match EncodedMemoryValueId(encoded_memory_id.0).decode() {
            MemoryValueId::F252(id) => {
                self.big_mults.increase_at(id);
            }
            MemoryValueId::Small(id) => {
                self.small_mults.increase_at(id);
            }
            MemoryValueId::Empty => panic!("Attempted add_input on empty memory cell."),
        }
    }

    pub fn write_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        range_check_9_9_trace_generator: &range_check_9_9::ClaimGenerator,
        range_check_9_9_b_trace_generator: &range_check_9_9_b::ClaimGenerator,
        range_check_9_9_c_trace_generator: &range_check_9_9_c::ClaimGenerator,
        range_check_9_9_d_trace_generator: &range_check_9_9_d::ClaimGenerator,
        range_check_9_9_e_trace_generator: &range_check_9_9_e::ClaimGenerator,
        range_check_9_9_f_trace_generator: &range_check_9_9_f::ClaimGenerator,
        range_check_9_9_g_trace_generator: &range_check_9_9_g::ClaimGenerator,
        range_check_9_9_h_trace_generator: &range_check_9_9_h::ClaimGenerator,
        log_max_big_size: u32,
    ) -> (Claim, InteractionClaimGenerator) {
        let big_table_traces = gen_big_memory_traces(
            self.big_values,
            self.big_mults.into_simd_vec(),
            log_max_big_size,
        );
        let small_table_trace =
            gen_small_memory_trace(self.small_values, self.small_mults.into_simd_vec());

        // Lookup data.
        let big_components_values: Vec<[_; N_M31_IN_FELT252]> = big_table_traces
            .iter()
            .map(|trace| std::array::from_fn(|i| trace[i].data.clone()))
            .collect_vec();
        let big_multiplicities = big_table_traces
            .iter()
            .map(|trace| trace.last().unwrap().data.clone())
            .collect_vec();
        let small_values: [_; N_M31_IN_SMALL_FELT252] =
            std::array::from_fn(|i| small_table_trace[i].data.clone());
        let small_multiplicities = small_table_trace.last().unwrap().data.clone();

        // Add inputs to range check that all the values are 9-bit felts.
        for values in &big_components_values {
            for (i, (col0, col1)) in values.iter().tuples().enumerate() {
                match i % 8 {
                    0 => col0
                        .par_iter()
                        .zip(col1.par_iter())
                        .for_each(|(val0, val1)| {
                            range_check_9_9_trace_generator.add_packed_inputs(&[[*val0, *val1]]);
                        }),
                    1 => col0
                        .par_iter()
                        .zip(col1.par_iter())
                        .for_each(|(val0, val1)| {
                            range_check_9_9_b_trace_generator.add_packed_inputs(&[[*val0, *val1]]);
                        }),
                    2 => col0
                        .par_iter()
                        .zip(col1.par_iter())
                        .for_each(|(val0, val1)| {
                            range_check_9_9_c_trace_generator.add_packed_inputs(&[[*val0, *val1]]);
                        }),
                    3 => col0
                        .par_iter()
                        .zip(col1.par_iter())
                        .for_each(|(val0, val1)| {
                            range_check_9_9_d_trace_generator.add_packed_inputs(&[[*val0, *val1]]);
                        }),
                    4 => col0
                        .par_iter()
                        .zip(col1.par_iter())
                        .for_each(|(val0, val1)| {
                            range_check_9_9_e_trace_generator.add_packed_inputs(&[[*val0, *val1]]);
                        }),
                    5 => col0
                        .par_iter()
                        .zip(col1.par_iter())
                        .for_each(|(val0, val1)| {
                            range_check_9_9_f_trace_generator.add_packed_inputs(&[[*val0, *val1]]);
                        }),
                    6 => col0
                        .par_iter()
                        .zip(col1.par_iter())
                        .for_each(|(val0, val1)| {
                            range_check_9_9_g_trace_generator.add_packed_inputs(&[[*val0, *val1]]);
                        }),
                    7 => col0
                        .par_iter()
                        .zip(col1.par_iter())
                        .for_each(|(val0, val1)| {
                            range_check_9_9_h_trace_generator.add_packed_inputs(&[[*val0, *val1]]);
                        }),
                    _ => {
                        unreachable!("There are only 8 possible values for i % 8.",)
                    }
                };
            }
        }

        for (i, (col0, col1)) in small_values.iter().tuples().enumerate() {
            match i % 4 {
                0 => col0
                    .par_iter()
                    .zip(col1.par_iter())
                    .for_each(|(val0, val1)| {
                        range_check_9_9_trace_generator.add_packed_inputs(&[[*val0, *val1]]);
                    }),
                1 => col0
                    .par_iter()
                    .zip(col1.par_iter())
                    .for_each(|(val0, val1)| {
                        range_check_9_9_b_trace_generator.add_packed_inputs(&[[*val0, *val1]]);
                    }),
                2 => col0
                    .par_iter()
                    .zip(col1.par_iter())
                    .for_each(|(val0, val1)| {
                        range_check_9_9_c_trace_generator.add_packed_inputs(&[[*val0, *val1]]);
                    }),
                3 => col0
                    .par_iter()
                    .zip(col1.par_iter())
                    .for_each(|(val0, val1)| {
                        range_check_9_9_d_trace_generator.add_packed_inputs(&[[*val0, *val1]]);
                    }),
                _ => {
                    unreachable!("There are only 4 possible values for i % 4.",)
                }
            };
        }

        // Extend trace.
        let mut big_log_sizes = vec![];
        for big_table_trace in big_table_traces {
            let big_log_size = big_table_trace[0].length.ilog2();
            big_log_sizes.push(big_log_size);
            let trace = big_table_trace
                .into_iter()
                .map(|eval| {
                    CircleEvaluation::<SimdBackend, M31, BitReversedOrder>::new(
                        CanonicCoset::new(big_log_size).circle_domain(),
                        eval,
                    )
                })
                .collect_vec();
            tree_builder.extend_evals(trace);
        }
        let small_log_size = small_table_trace[0].len().ilog2();
        let trace = small_table_trace
            .into_iter()
            .map(|eval| {
                CircleEvaluation::<SimdBackend, M31, BitReversedOrder>::new(
                    CanonicCoset::new(small_log_size).circle_domain(),
                    eval,
                )
            })
            .collect_vec();
        tree_builder.extend_evals(trace);

        (
            Claim {
                big_log_sizes,
                small_log_size,
            },
            InteractionClaimGenerator {
                big_components_values,
                big_multiplicities,
                small_values,
                small_multiplicities,
            },
        )
    }
}

/// Generates the trace for the id -> f252 `big` tables. Splits the table to multiple traces
/// according to `log_max_big_size`.
fn gen_big_memory_traces(
    values: Vec<[u32; 8]>,
    mults: Vec<PackedM31>,
    log_max_big_size: u32,
) -> Vec<Vec<BaseColumn>> {
    assert!(log_max_big_size >= LOG_N_LANES);
    let max_big_size = 1 << log_max_big_size;
    assert_eq!(values.len() / N_LANES, mults.len());
    let mut traces = vec![];

    for (values, mults) in values
        .chunks(max_big_size)
        .zip(mults.chunks(max_big_size / N_LANES))
    {
        let trace = gen_single_big_memory_trace(values, mults);
        traces.push(trace);
    }

    traces
}

// Generates the trace of the large value memory table.
fn gen_single_big_memory_trace(values: &[[u32; 8]], mults: &[PackedM31]) -> Vec<BaseColumn> {
    assert_eq!(values.len(), mults.len() * N_LANES);
    let column_length = values.len().next_power_of_two();

    let mut mults = mults.to_vec();
    mults.resize(column_length / N_LANES, PackedM31::zero());
    let multiplicities = BaseColumn::from_simd(mults);

    let packed_values = values
        .iter()
        .chain(std::iter::repeat(&[0; 8]))
        .take(column_length)
        .array_chunks::<N_LANES>()
        .map(|chunk| {
            std::array::from_fn(|i| Simd::from_array(std::array::from_fn(|j| chunk[j][i])))
        })
        .collect_vec();

    let mut value_trace =
        std::iter::repeat_with(|| unsafe { BaseColumn::uninitialized(column_length) })
            .take(N_M31_IN_FELT252)
            .collect_vec();
    for (i, values) in packed_values.iter().enumerate() {
        let values = split_f252_simd(*values);
        for (j, value) in values.iter().enumerate() {
            value_trace[j].data[i] = *value;
        }
    }

    chain!(value_trace, [multiplicities]).collect_vec()
}

// Generates the trace of the small value memory table.
fn gen_small_memory_trace(values: Vec<u128>, mut mults: Vec<PackedM31>) -> Vec<BaseColumn> {
    assert_eq!(values.len(), mults.len() * N_LANES);
    let column_length = values.len().next_power_of_two();

    mults.resize(column_length / N_LANES, PackedM31::zero());
    let multiplicities = BaseColumn::from_simd(mults);

    let packed_values: Vec<[Simd<u32, N_LANES>; 4]> = values
        .into_iter()
        .chain(std::iter::repeat(0))
        .take(column_length)
        .map(u128_to_4_limbs)
        .array_chunks::<N_LANES>()
        .map(|chunk| {
            std::array::from_fn(|i| Simd::from_array(std::array::from_fn(|j| chunk[j][i])))
        })
        .collect_vec();

    let mut values_trace =
        std::iter::repeat_with(|| unsafe { BaseColumn::uninitialized(column_length) })
            .take(N_M31_IN_SMALL_FELT252)
            .collect_vec();
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
        for (j, value) in values[..N_M31_IN_SMALL_FELT252].iter().enumerate() {
            values_trace[j].data[i] = *value;
        }
    }

    chain!(values_trace, [multiplicities]).collect_vec()
}

#[derive(Debug)]
pub struct InteractionClaimGenerator {
    pub big_components_values: Vec<[Vec<PackedM31>; N_M31_IN_FELT252]>,
    pub big_multiplicities: Vec<Vec<PackedM31>>,
    pub small_values: [Vec<PackedM31>; N_M31_IN_SMALL_FELT252],
    pub small_multiplicities: Vec<PackedM31>,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        lookup_elements: &relations::MemoryIdToBig,
        range9_9_lookup_elements: &relations::RangeCheck_9_9,
        range9_9_b_lookup_elements: &relations::RangeCheck_9_9_B,
        range9_9_c_lookup_elements: &relations::RangeCheck_9_9_C,
        range9_9_d_lookup_elements: &relations::RangeCheck_9_9_D,
        range9_9_e_lookup_elements: &relations::RangeCheck_9_9_E,
        range9_9_f_lookup_elements: &relations::RangeCheck_9_9_F,
        range9_9_g_lookup_elements: &relations::RangeCheck_9_9_G,
        range9_9_h_lookup_elements: &relations::RangeCheck_9_9_H,
    ) -> InteractionClaim {
        let mut offset = 0;
        let (big_traces, big_claimed_sums): (Vec<_>, Vec<_>) = self
            .big_components_values
            .iter()
            .zip(self.big_multiplicities.iter())
            .map(|(big_components_values, big_multiplicities)| {
                let res = Self::gen_big_memory_interaction_trace(
                    big_components_values,
                    big_multiplicities,
                    offset,
                    lookup_elements,
                    range9_9_lookup_elements,
                    range9_9_b_lookup_elements,
                    range9_9_c_lookup_elements,
                    range9_9_d_lookup_elements,
                    range9_9_e_lookup_elements,
                    range9_9_f_lookup_elements,
                    range9_9_g_lookup_elements,
                    range9_9_h_lookup_elements,
                );
                offset += big_multiplicities.len() as u32 * N_LANES as u32;
                res
            })
            .unzip();
        for big_trace in big_traces {
            tree_builder.extend_evals(big_trace);
        }

        let (small_trace, small_claimed_sum) = self.gen_small_memory_interaction_trace(
            lookup_elements,
            range9_9_lookup_elements,
            range9_9_b_lookup_elements,
            range9_9_c_lookup_elements,
            range9_9_d_lookup_elements,
        );
        tree_builder.extend_evals(small_trace);

        InteractionClaim {
            small_claimed_sum,
            big_claimed_sums,
        }
    }

    fn gen_big_memory_interaction_trace(
        big_components_values: &[Vec<PackedM31>; N_M31_IN_FELT252],
        big_multiplicities: &[PackedM31],
        offset: u32,
        lookup_elements: &relations::MemoryIdToBig,
        range9_9_lookup_elements: &relations::RangeCheck_9_9,
        range9_9_b_lookup_elements: &relations::RangeCheck_9_9_B,
        range9_9_c_lookup_elements: &relations::RangeCheck_9_9_C,
        range9_9_d_lookup_elements: &relations::RangeCheck_9_9_D,
        range9_9_e_lookup_elements: &relations::RangeCheck_9_9_E,
        range9_9_f_lookup_elements: &relations::RangeCheck_9_9_F,
        range9_9_g_lookup_elements: &relations::RangeCheck_9_9_G,
        range9_9_h_lookup_elements: &relations::RangeCheck_9_9_H,
    ) -> (
        Vec<CircleEvaluation<SimdBackend, M31, BitReversedOrder>>,
        QM31,
    ) {
        assert!(big_components_values
            .iter()
            .all(|v| v.len() == big_multiplicities.len()));
        let big_table_log_size = big_components_values[0].len().ilog2() + LOG_N_LANES;
        let mut big_values_logup_gen = LogupTraceGenerator::new(big_table_log_size);

        // Every element is 9-bit.
        for (i, (limb0, limb1, limb2, limb3)) in big_components_values.iter().tuples().enumerate() {
            let mut col_gen = big_values_logup_gen.new_col();
            (col_gen.par_iter_mut(), limb0, limb1, limb2, limb3)
                .into_par_iter()
                .for_each(|(writer, limb0, limb1, limb2, limb3)| {
                    let (denom0, denom1): (PackedQM31, PackedQM31) = match i % 4 {
                        0 => (
                            range9_9_lookup_elements.combine(&[*limb0, *limb1]),
                            range9_9_b_lookup_elements.combine(&[*limb2, *limb3]),
                        ),
                        1 => (
                            range9_9_c_lookup_elements.combine(&[*limb0, *limb1]),
                            range9_9_d_lookup_elements.combine(&[*limb2, *limb3]),
                        ),
                        2 => (
                            range9_9_e_lookup_elements.combine(&[*limb0, *limb1]),
                            range9_9_f_lookup_elements.combine(&[*limb2, *limb3]),
                        ),
                        3 => (
                            range9_9_g_lookup_elements.combine(&[*limb0, *limb1]),
                            range9_9_h_lookup_elements.combine(&[*limb2, *limb3]),
                        ),
                        _ => {
                            unreachable!("There are only 4 possible values for i % 4.",)
                        }
                    };
                    writer.write_frac(denom0 + denom1, denom0 * denom1);
                });
            col_gen.finalize_col();
        }

        // Yield large values.
        let mut col_gen = big_values_logup_gen.new_col();
        let large_memory_value_id_tag = Simd::splat(LARGE_MEMORY_VALUE_ID_BASE);
        #[allow(clippy::needless_range_loop)]
        for vec_row in 0..1 << (big_table_log_size - LOG_N_LANES) {
            let id_and_value: [_; N_M31_IN_FELT252 + MEMORY_ID_SIZE] = std::array::from_fn(|i| {
                if i == 0 {
                    unsafe {
                        PackedM31::from_simd_unchecked(
                            (SIMD_ENUMERATION_0 + Simd::splat((vec_row * N_LANES) as u32 + offset))
                                | large_memory_value_id_tag,
                        )
                    }
                } else {
                    big_components_values[i - 1][vec_row]
                }
            });
            let denom: PackedQM31 = lookup_elements.combine(&id_and_value);
            col_gen.write_frac(vec_row, (-big_multiplicities[vec_row]).into(), denom);
        }
        col_gen.finalize_col();

        big_values_logup_gen.finalize_last()
    }

    fn gen_small_memory_interaction_trace(
        &self,
        lookup_elements: &relations::MemoryIdToBig,
        range9_9_lookup_elements: &relations::RangeCheck_9_9,
        range9_9_b_lookup_elements: &relations::RangeCheck_9_9_B,
        range9_9_c_lookup_elements: &relations::RangeCheck_9_9_C,
        range9_9_d_lookup_elements: &relations::RangeCheck_9_9_D,
    ) -> (
        Vec<CircleEvaluation<SimdBackend, M31, BitReversedOrder>>,
        QM31,
    ) {
        let small_table_log_size = self.small_values[0].len().ilog2() + LOG_N_LANES;
        let mut small_values_logup_gen = LogupTraceGenerator::new(small_table_log_size);

        // Every element is 9-bit.
        for (i, (limb0, limb1, limb2, limb3)) in self.small_values.iter().tuples().enumerate() {
            let mut col_gen = small_values_logup_gen.new_col();
            (col_gen.par_iter_mut(), limb0, limb1, limb2, limb3)
                .into_par_iter()
                .for_each(|(writer, limb0, limb1, limb2, limb3)| {
                    let (denom0, denom1): (PackedQM31, PackedQM31) = match i % 2 {
                        0 => (
                            range9_9_lookup_elements.combine(&[*limb0, *limb1]),
                            range9_9_b_lookup_elements.combine(&[*limb2, *limb3]),
                        ),
                        1 => (
                            range9_9_c_lookup_elements.combine(&[*limb0, *limb1]),
                            range9_9_d_lookup_elements.combine(&[*limb2, *limb3]),
                        ),
                        _ => {
                            unreachable!()
                        }
                    };
                    writer.write_frac(denom0 + denom1, denom0 * denom1);
                });
            col_gen.finalize_col();
        }

        // Yield small values.
        let mut col_gen = small_values_logup_gen.new_col();
        for vec_row in 0..1 << (small_table_log_size - LOG_N_LANES) {
            let id_and_value: [_; N_M31_IN_SMALL_FELT252 + MEMORY_ID_SIZE] =
                std::array::from_fn(|i| {
                    if i == 0 {
                        unsafe {
                            PackedM31::from_simd_unchecked(
                                SIMD_ENUMERATION_0 + Simd::splat((vec_row * N_LANES) as u32),
                            )
                        }
                    } else {
                        self.small_values[i - 1][vec_row]
                    }
                });
            let denom: PackedQM31 = lookup_elements.combine(&id_and_value);
            col_gen.write_frac(vec_row, (-self.small_multiplicities[vec_row]).into(), denom);
        }
        col_gen.finalize_col();

        small_values_logup_gen.finalize_last()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use cairo_air::air::CairoInteractionElements;
    use cairo_air::components::memory_id_to_big::{self, SmallEval};
    use cairo_air::PreProcessedTraceVariant;
    use itertools::Itertools;
    use rand::rngs::SmallRng;
    use rand::{Rng, SeedableRng};
    use stwo::core::channel::Blake2sChannel;
    use stwo::core::fields::m31::M31;
    use stwo::prover::backend::simd::m31::PackedM31;
    use stwo_cairo_adapter::memory::{
        value_from_felt252, MemoryBuilder, MemoryConfig, MemoryValue,
    };
    use stwo_cairo_common::memory::N_M31_IN_FELT252;
    use stwo_cairo_common::preprocessed_columns::preprocessed_trace::PreProcessedTrace;
    use stwo_cairo_common::prover_types::felt::split_f252;
    use stwo_constraint_framework::TraceLocationAllocator;

    use crate::debug_tools::assert_constraints::assert_component;
    use crate::debug_tools::mock_tree_builder::MockCommitmentScheme;
    use crate::witness::components::{
        memory_address_to_id, range_check_9_9, range_check_9_9_b, range_check_9_9_c,
        range_check_9_9_d, range_check_9_9_e, range_check_9_9_f, range_check_9_9_g,
        range_check_9_9_h,
    };

    #[test]
    fn test_memory_constraints() {
        let log_size = 10;
        let log_max_seq_size = 8;
        let n_values = 1 << log_size;
        let mut rng = SmallRng::seed_from_u64(1152);
        let mut mem = MemoryBuilder::new(MemoryConfig {
            log_small_value_capacity: log_max_seq_size,
            ..Default::default()
        });
        for i in 1..n_values {
            mem.set(i, MemoryValue::F252(rng.gen()));
        }
        for i in n_values..n_values * 2 {
            mem.set(i, MemoryValue::Small(rng.gen()));
        }
        let memory = mem.build().0;

        let mut commitment_scheme = MockCommitmentScheme::default();

        // Preprocessed trace.
        let mut tree_builder = commitment_scheme.tree_builder();
        tree_builder.extend_evals(
            PreProcessedTraceVariant::CanonicalWithoutPedersen
                .to_preprocessed_trace()
                .gen_trace(),
        );
        tree_builder.finalize_interaction();

        // Base trace.
        let mut tree_builder = commitment_scheme.tree_builder();
        let preprocessed_trace = Arc::new(PreProcessedTrace::canonical());
        let id_to_big = super::ClaimGenerator::new(&memory);
        let range_check_9_9 = range_check_9_9::ClaimGenerator::new(Arc::clone(&preprocessed_trace));
        let range_check_9_9_b =
            range_check_9_9_b::ClaimGenerator::new(Arc::clone(&preprocessed_trace));
        let range_check_9_9_c =
            range_check_9_9_c::ClaimGenerator::new(Arc::clone(&preprocessed_trace));
        let range_check_9_9_d =
            range_check_9_9_d::ClaimGenerator::new(Arc::clone(&preprocessed_trace));
        let range_check_9_9_e =
            range_check_9_9_e::ClaimGenerator::new(Arc::clone(&preprocessed_trace));
        let range_check_9_9_f =
            range_check_9_9_f::ClaimGenerator::new(Arc::clone(&preprocessed_trace));
        let range_check_9_9_g =
            range_check_9_9_g::ClaimGenerator::new(Arc::clone(&preprocessed_trace));
        let range_check_9_9_h =
            range_check_9_9_h::ClaimGenerator::new(Arc::clone(&preprocessed_trace));
        let (claim, interaction_generator) = id_to_big.write_trace(
            &mut tree_builder,
            &range_check_9_9,
            &range_check_9_9_b,
            &range_check_9_9_c,
            &range_check_9_9_d,
            &range_check_9_9_e,
            &range_check_9_9_f,
            &range_check_9_9_g,
            &range_check_9_9_h,
            log_max_seq_size,
        );
        tree_builder.finalize_interaction();

        // Interaction trace.
        let mut dummy_channel = Blake2sChannel::default();
        let interaction_elements = CairoInteractionElements::draw(&mut dummy_channel);
        let mut tree_builder = commitment_scheme.tree_builder();
        let interaction_claim = interaction_generator.write_interaction_trace(
            &mut tree_builder,
            &interaction_elements.memory_id_to_value,
            &interaction_elements.range_checks.rc_9_9,
            &interaction_elements.range_checks.rc_9_9_b,
            &interaction_elements.range_checks.rc_9_9_c,
            &interaction_elements.range_checks.rc_9_9_d,
            &interaction_elements.range_checks.rc_9_9_e,
            &interaction_elements.range_checks.rc_9_9_f,
            &interaction_elements.range_checks.rc_9_9_g,
            &interaction_elements.range_checks.rc_9_9_h,
        );
        tree_builder.finalize_interaction();

        let mut location_allocator = TraceLocationAllocator::default();
        let big_components = memory_id_to_big::big_components_from_claim(
            &claim.big_log_sizes,
            &interaction_claim.big_claimed_sums,
            &interaction_elements.memory_id_to_value,
            &interaction_elements.range_checks.rc_9_9,
            &interaction_elements.range_checks.rc_9_9_b,
            &interaction_elements.range_checks.rc_9_9_c,
            &interaction_elements.range_checks.rc_9_9_d,
            &interaction_elements.range_checks.rc_9_9_e,
            &interaction_elements.range_checks.rc_9_9_f,
            &interaction_elements.range_checks.rc_9_9_g,
            &interaction_elements.range_checks.rc_9_9_h,
            &mut location_allocator,
        );

        let small_component = memory_id_to_big::SmallComponent::new(
            &mut location_allocator,
            SmallEval {
                log_n_rows: claim.small_log_size,
                lookup_elements: interaction_elements.memory_id_to_value.clone(),
                range_check_9_9_relation: interaction_elements.range_checks.rc_9_9.clone(),
                range_check_9_9_b_relation: interaction_elements.range_checks.rc_9_9_b.clone(),
                range_check_9_9_c_relation: interaction_elements.range_checks.rc_9_9_c.clone(),
                range_check_9_9_d_relation: interaction_elements.range_checks.rc_9_9_d.clone(),
            },
            interaction_claim.small_claimed_sum,
        );

        let trace_domain_evaluations = commitment_scheme.trace_domain_evaluations();

        for component in big_components {
            assert_component(&component, &trace_domain_evaluations);
        }
        assert_component(&small_component, &trace_domain_evaluations);
    }

    #[test]
    fn test_memory_splits_correctly() {
        let n_large_values = 300;
        let n_small_values = 300;
        let log_max_seq_size = 8;
        let mut rng = SmallRng::seed_from_u64(1152);
        let mut mem = MemoryBuilder::new(MemoryConfig {
            log_small_value_capacity: log_max_seq_size,
            ..Default::default()
        });
        for i in 1..n_large_values {
            mem.set(i, MemoryValue::F252(rng.gen()));
        }
        for i in n_large_values..n_large_values + n_small_values {
            mem.set(i, MemoryValue::Small(rng.gen()));
        }
        let memory = mem.build().0;
        let mut commitment_scheme = MockCommitmentScheme::default();
        let mut tree_builder = commitment_scheme.tree_builder();
        let preprocessed_trace = Arc::new(PreProcessedTrace::canonical());
        let id_to_big = super::ClaimGenerator::new(&memory);
        let range_check_9_9 = range_check_9_9::ClaimGenerator::new(Arc::clone(&preprocessed_trace));
        let range_check_9_9_b =
            range_check_9_9_b::ClaimGenerator::new(Arc::clone(&preprocessed_trace));
        let range_check_9_9_c =
            range_check_9_9_c::ClaimGenerator::new(Arc::clone(&preprocessed_trace));
        let range_check_9_9_d =
            range_check_9_9_d::ClaimGenerator::new(Arc::clone(&preprocessed_trace));
        let range_check_9_9_e =
            range_check_9_9_e::ClaimGenerator::new(Arc::clone(&preprocessed_trace));
        let range_check_9_9_f =
            range_check_9_9_f::ClaimGenerator::new(Arc::clone(&preprocessed_trace));
        let range_check_9_9_g =
            range_check_9_9_g::ClaimGenerator::new(Arc::clone(&preprocessed_trace));
        let range_check_9_9_h =
            range_check_9_9_h::ClaimGenerator::new(Arc::clone(&preprocessed_trace));
        let expected_small_log_size = log_max_seq_size;
        let expected_first_big_log_size = log_max_seq_size;
        let big_value_overflow = n_large_values - (1 << log_max_seq_size);
        let small_value_overflow = n_small_values - (1 << log_max_seq_size);
        let expected_second_big_log_size = (big_value_overflow + small_value_overflow)
            .next_power_of_two()
            .ilog2();
        let expected_big_log_sizes =
            vec![expected_first_big_log_size, expected_second_big_log_size];

        let (claim, ..) = id_to_big.write_trace(
            &mut tree_builder,
            &range_check_9_9,
            &range_check_9_9_b,
            &range_check_9_9_c,
            &range_check_9_9_d,
            &range_check_9_9_e,
            &range_check_9_9_f,
            &range_check_9_9_g,
            &range_check_9_9_h,
            log_max_seq_size,
        );

        assert_eq!(claim.small_log_size, expected_small_log_size);
        assert_eq!(claim.big_log_sizes, expected_big_log_sizes);
    }

    #[test]
    fn test_deduce_output_simd() {
        // Set up memory addresses, padded by ones at the end.
        let memory_addresses = [1, 2, 3, 4, 5, 6, 7, 8, 15, 16, 17, 18, 1, 1, 1, 1];
        let input = PackedM31::from_array(memory_addresses.map(M31::from_u32_unchecked));

        // The expected values will alternate between small felts and big felts.
        let mut expected = (0..memory_addresses.len() as u32)
            .map(|i| {
                let arr: [u32; 8] = if i.is_multiple_of(2) {
                    [i, 0, 0, 0, 0, 0, 0, 0]
                } else {
                    [i; 8]
                };
                arr
            })
            .collect_vec();

        // Correct the padded-by-ones area at the end to the correct value.
        let value_at_one = expected[0];
        expected[12..].fill(value_at_one);

        // Create memory.
        let mut mem = MemoryBuilder::new(MemoryConfig::default());
        for (j, a) in memory_addresses.iter().enumerate() {
            mem.set(*a, value_from_felt252(expected[j]));
        }
        let (mem, ..) = mem.build();
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
