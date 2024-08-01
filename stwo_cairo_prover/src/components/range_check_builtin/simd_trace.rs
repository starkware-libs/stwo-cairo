use itertools::{chain, Itertools};
use num_traits::Zero;
use stwo_prover::constraint_framework::logup::{LogupTraceGenerator, LookupElements};
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::qm31::PackedQM31;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::{Col, Column};
use stwo_prover::core::fields::m31::{BaseField, M31};
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;
use stwo_prover::core::ColumnVec;

use super::component::{ADDRESS_FELTS, N_BITS_PER_FELT, N_VALUES_FELTS};

// Memory addresses and the corresponding values, for the RangeCheck128Builtin segment.
pub struct RangeCheckInput {
    pub address: PackedM31,
    pub values: [u128; N_LANES],
}

pub struct RangeCheck128BuiltinLookupData {
    pub memory_lookups: [BaseColumn; ADDRESS_FELTS + N_VALUES_FELTS],
    pub rcunit2_lookups: [BaseColumn; 1],
}
impl RangeCheck128BuiltinLookupData {
    fn new(log_size: u32) -> Self {
        Self {
            memory_lookups: std::array::from_fn(|_| unsafe {
                BaseColumn::uninitialized(1 << log_size)
            }),
            rcunit2_lookups: std::array::from_fn(|_| unsafe {
                BaseColumn::uninitialized(1 << log_size)
            }),
        }
    }
}

pub fn generate_trace(
    log_size: u32,
    address_initial_offset: M31,
    inputs: &[RangeCheckInput],
) -> (
    ColumnVec<CircleEvaluation<SimdBackend, BaseField, BitReversedOrder>>,
    RangeCheck128BuiltinLookupData,
) {
    let mut lookup_data = RangeCheck128BuiltinLookupData::new(log_size);
    let mut trace = (0..ADDRESS_FELTS + N_VALUES_FELTS)
        .map(|_| unsafe { Col::<SimdBackend, M31>::uninitialized(1 << log_size) })
        .collect_vec();

    #[allow(clippy::needless_range_loop)]
    for vec_row in 0..(1 << (log_size - LOG_N_LANES)) {
        // TODO: remove address from the trace.
        let address = inputs[vec_row].address - PackedM31::broadcast(address_initial_offset);
        let splitted_values = split_u128(inputs[vec_row].values);
        trace[0].data[vec_row] = address;
        for (i, v) in splitted_values.iter().enumerate() {
            trace[i + 1].data[vec_row] = *v;
        }

        lookup_data.rcunit2_lookups[0].data[vec_row] = splitted_values[N_VALUES_FELTS - 1];

        chain![vec![inputs[vec_row].address], splitted_values]
            .enumerate()
            .for_each(|(i, val)| lookup_data.memory_lookups[i].data[vec_row] = val);
    }

    let domain = CanonicCoset::new(log_size).circle_domain();
    (
        trace
            .into_iter()
            .map(|eval| CircleEvaluation::<SimdBackend, _, BitReversedOrder>::new(domain, eval))
            .collect_vec(),
        lookup_data,
    )
}

pub fn gen_interaction_trace(
    log_size: u32,
    lookup_data: RangeCheck128BuiltinLookupData,
    memory_lookup_elements: &LookupElements,
    rcunit2_lookup_elements: &LookupElements,
) -> (
    ColumnVec<CircleEvaluation<SimdBackend, BaseField, BitReversedOrder>>,
    SecureField,
) {
    let mut logup_gen = LogupTraceGenerator::new(log_size);
    let mut col_gen = logup_gen.new_col();
    for vec_row in 0..(1 << (log_size - LOG_N_LANES)) {
        let p_mem: PackedQM31 = memory_lookup_elements.combine(
            &lookup_data
                .memory_lookups
                .each_ref()
                .map(|l| l.data[vec_row]),
        );
        let p_rc2: PackedQM31 =
            rcunit2_lookup_elements.combine(&[lookup_data.rcunit2_lookups[0].data[vec_row]]);
        col_gen.write_frac(vec_row, p_mem + p_rc2, p_mem * p_rc2);
    }
    col_gen.finalize_col();
    logup_gen.finalize()
}

// TODO: replace this code with prover_types when available.
fn split_u128(mut x: [u128; N_LANES]) -> [PackedM31; N_VALUES_FELTS] {
    let mut res = [PackedM31::zero(); N_VALUES_FELTS];
    for e in res.iter_mut().take(N_VALUES_FELTS) {
        let x_low =
            x.map(|x| BaseField::from_u32_unchecked((x & ((1 << N_BITS_PER_FELT) - 1)) as u32));
        *e = PackedM31::from_array(x_low);
        x = x.map(|x| x >> N_BITS_PER_FELT);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::array;

    use rand::Rng;
    use stwo_prover::constraint_framework::constant_columns::gen_is_first;
    use stwo_prover::constraint_framework::FrameworkComponent;
    use stwo_prover::core::fields::qm31::SecureField;
    use stwo_prover::core::pcs::TreeVec;

    use super::*;
    use crate::components::range_check_builtin::component::RangeCheck128BuiltinComponent;

    #[test]
    fn test_generate_trace() {
        let log_size = 8;
        let address_initial_offset = 1000;
        let mut rng = rand::thread_rng();
        let inputs = (0..32)
            .map(|i| RangeCheckInput {
                address: PackedM31::from_array(array::from_fn(|j| {
                    M31::from_u32_unchecked(i * N_LANES as u32 + j as u32 + address_initial_offset)
                })),
                values: array::from_fn(|_| rng.gen()),
            })
            .collect_vec();

        let (trace, lookup_data) =
            generate_trace(log_size, M31::from(address_initial_offset), &inputs);

        assert_eq!(trace.len(), ADDRESS_FELTS + N_VALUES_FELTS);
        assert_eq!(
            trace[0].values.clone().into_cpu_vec(),
            (0..1 << log_size).map(M31::from).collect_vec()
        );
        for row_offset in 0..(1 << log_size) {
            let mut input = inputs[row_offset / N_LANES].values[row_offset % N_LANES];
            for col in trace.iter().skip(1) {
                assert_eq!(
                    input % (1 << N_BITS_PER_FELT),
                    col.values.at(row_offset).0.into()
                );
                input >>= N_BITS_PER_FELT;
            }
        }
        // Assert rcunit2_lookups are in range [0, 4).
        for rc2_col in lookup_data.rcunit2_lookups.iter() {
            for row in rc2_col.data.iter() {
                assert!(row.to_array().iter().all(|&x| x.0 < 4));
            }
        }

        // Assert memory addresses are sequential, offseted by `address_initial_offset`.
        assert_eq!(
            lookup_data.memory_lookups[0].data.len() * N_LANES,
            1 << log_size
        );
        for (i, addresses) in lookup_data.memory_lookups[0].data.iter().enumerate() {
            assert_eq!(
                addresses.to_array(),
                array::from_fn(|j| {
                    M31::from_u32_unchecked((i * N_LANES + j) as u32 + address_initial_offset)
                })
            );
        }
    }

    #[test]
    fn test_interaction_trace() {
        let log_size = 8;
        let address_initial_offset = 10032;
        let inputs = (0..32)
            .map(|i| RangeCheckInput {
                address: PackedM31::from_array(array::from_fn(|j| {
                    M31::from_u32_unchecked(i * N_LANES as u32 + j as u32 + address_initial_offset)
                })),
                values: array::from_fn(|_| 85237070383327051513330625736333328897_u128),
            })
            .collect_vec();

        let (trace, lookup_data) =
            generate_trace(log_size, M31::from(address_initial_offset), &inputs);

        let memory_lookup_elements = LookupElements {
            z: SecureField::from_m31_array(array::from_fn(|i| M31::from_u32_unchecked(i as u32))),
            alpha: SecureField::from_m31_array(array::from_fn(|i| {
                M31::from_u32_unchecked(i as u32)
            })),
        };
        let rc_lookup_elements = LookupElements {
            z: SecureField::from_m31_array(array::from_fn(|i| {
                M31::from_u32_unchecked(3 * i as u32)
            })),
            alpha: SecureField::from_m31_array(array::from_fn(|i| {
                M31::from_u32_unchecked(4 * i as u32)
            })),
        };

        let (interaction_trace, claimed_sum) = gen_interaction_trace(
            log_size,
            lookup_data,
            &memory_lookup_elements,
            &rc_lookup_elements,
        );

        let trace = TreeVec::new(vec![trace, interaction_trace, vec![gen_is_first(log_size)]]);
        let trace_polys = trace.map_cols(|c| c.interpolate());

        let component = RangeCheck128BuiltinComponent {
            log_size,
            initial_memory_address: M31::from(address_initial_offset),
            memory_lookup_elements,
            rcunit2_lookup_elements: rc_lookup_elements,
            claimed_sum,
        };

        stwo_prover::constraint_framework::assert_constraints(
            &trace_polys,
            CanonicCoset::new(log_size),
            |eval| {
                component.evaluate(eval);
            },
        )
    }
}
