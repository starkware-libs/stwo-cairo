use itertools::{chain, Itertools};
use num_traits::Zero;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::{Col, Column};
use stwo_prover::core::fields::m31::{BaseField, M31};
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

    #[test]
    fn test_generate_trace() {
        use super::*;

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
}
