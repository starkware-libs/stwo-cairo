use std::simd::Simd;

use itertools::{chain, Itertools};
use num_traits::One;
use stwo_prover::constraint_framework::logup::LookupElements;
use stwo_prover::core::backend::simd::m31::{PackedM31, N_LANES};
use stwo_prover::core::backend::simd::qm31::PackedQM31;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::{Col, Column};
use stwo_prover::core::fields::m31::{BaseField, M31};
use stwo_prover::core::lookups::utils::Fraction;
use stwo_prover::core::pcs::TreeBuilder;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;

use super::component::{AddrToIdClaim, N_IDS_PER_LINE};
use crate::components::utils::to_evals;
use crate::components::{LookupFunc, StandardInteractionProver, StandardLookupData};
use crate::input::mem::EncodedMemoryValueId;

pub struct AddrToIdProverBuilder {
    pub addr_to_id: Vec<Simd<u32, N_LANES>>,
    pub multiplicities: Vec<Simd<u32, N_LANES>>,
}
impl AddrToIdProverBuilder {
    pub fn new(addr_to_id: Vec<EncodedMemoryValueId>) -> Self {
        // Pack the ids.
        let packed_size = addr_to_id.len().div_ceil(N_LANES);
        let pad_size = packed_size * N_LANES - addr_to_id.len();

        let mut iter = chain![
            addr_to_id.into_iter().map(|x| x.0),
            std::iter::repeat(0).take(pad_size)
        ]
        .array_chunks::<N_LANES>();

        Self {
            addr_to_id: iter.map(|chunk| Simd::from_array(chunk)).collect_vec(),
            multiplicities: vec![Simd::splat(0); packed_size],
        }
    }
    pub fn add_inputs_simd(&mut self, inputs: Simd<u32, N_LANES>) -> Simd<u32, N_LANES> {
        let addresses = inputs.to_array();
        Simd::from_array(addresses.map(|addr| self.add_inputs(addr)))
    }
    pub fn add_inputs(&mut self, addr: u32) -> u32 {
        let addr = addr as usize;
        self.multiplicities[addr / N_LANES][addr % N_LANES] += 1;
        self.addr_to_id[addr / N_LANES][addr % N_LANES]
    }
    pub fn build(mut self) -> AddrToIdProver {
        // TODO: Split and return a Vec.
        let size = self
            .addr_to_id
            .len()
            .next_power_of_two()
            .max(64 * N_IDS_PER_LINE);
        self.addr_to_id.resize(size, Simd::splat(0));
        self.multiplicities.resize(size, Simd::splat(0));
        AddrToIdProver {
            addr_to_id: self.addr_to_id,
            multiplicities: self.multiplicities,
        }
    }
}

pub struct AddrToIdProver {
    pub addr_to_id: Vec<Simd<u32, N_LANES>>,
    pub multiplicities: Vec<Simd<u32, N_LANES>>,
}
impl AddrToIdProver {
    pub fn write_trace(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
    ) -> (AddrToIdClaim, StandardInteractionProver<AddrToIdLookupData>) {
        let log_size = (self.addr_to_id.len() * N_LANES / N_IDS_PER_LINE) as u32;
        let claim = AddrToIdClaim { log_size };
        let mut trace = (0..claim.log_sizes()[0].len())
            .map(|_| Col::<SimdBackend, BaseField>::zeros(1 << log_size))
            .collect_vec();
        let mut lookup_data = AddrToIdLookupData {
            addr_to_id: self.addr_to_id.clone(),
            multiplicities: self.multiplicities.clone(),
        };
        for row_vec in 0..(1 << log_size) {
            for i in 0..N_IDS_PER_LINE {
                trace[2 * i].data[row_vec] = unsafe {
                    PackedM31::from_simd_unchecked(self.addr_to_id[row_vec * N_IDS_PER_LINE + i])
                };
                trace[2 * i].data[row_vec] = unsafe {
                    PackedM31::from_simd_unchecked(
                        self.multiplicities[row_vec * N_IDS_PER_LINE + i],
                    )
                };
            }
        }

        tree_builder.extend_evals(to_evals(trace));
        (
            claim,
            StandardInteractionProver {
                log_size,
                lookup_data: vec![lookup_data],
            },
        )
    }
}

pub struct AddrToIdLookupData {
    pub addr_to_id: Vec<Simd<u32, N_LANES>>,
    pub multiplicities: Vec<Simd<u32, N_LANES>>,
}
impl StandardLookupData for AddrToIdLookupData {
    const N_LOOKUPS: usize = 2 * N_IDS_PER_LINE;
    type Elements = LookupElements<2>;
    fn lookups<'a, 'b>(&'a self, elements: &'a Self::Elements) -> Vec<LookupFunc<'a, 'b>> {
        (0..N_IDS_PER_LINE)
            .flat_map(|i| {
                vec![
                    // Address lookup.
                    Box::new(
                        move |initial, dst: &mut [Fraction<PackedM31, PackedQM31>]| {
                            let offset = PackedM31::from_array(std::array::from_fn(|i| {
                                M31::from_u32_unchecked(i as u32)
                            }));
                            for (j, dst) in dst.iter_mut().enumerate() {
                                let row = initial + j;
                                let index: usize = row * N_IDS_PER_LINE + i;
                                let addr = PackedM31::broadcast(M31::from_u32_unchecked(
                                    (index * N_LANES) as u32,
                                )) + offset;
                                let id = unsafe {
                                    PackedM31::from_simd_unchecked(self.addr_to_id[index as usize])
                                };
                                let denom = unsafe { elements.combine(&[addr, id]) };
                                *dst = Fraction::new(PackedM31::one(), denom);
                            }
                        },
                    )
                        as Box<dyn Fn(usize, &mut [Fraction<PackedM31, PackedQM31>])>,
                ]
            })
            .collect_vec()
    }
}

//     pub fn write_trace(
//         &mut self,
//         tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleHasher>,
//     ) -> (AddrToIdClaim, InteractionClaimProver) {
//         let mut trace = (0..N_M31_IN_FELT252 + 2)
//             .map(|_| Col::<SimdBackend, BaseField>::zeros(MEMORY_ADDRESS_BOUND))
//             .collect_vec();

//         let inc = PackedBaseField::from_array(std::array::from_fn(|i| {
//             M31::from_u32_unchecked((i) as u32)
//         }));
//         for (i, (values, multiplicity)) in zip_eq(&self.values, &self.multiplicities).enumerate()
// {             let values = split_f252_simd(*values);
//             // TODO(AlonH): Either create a constant column for the addresses and remove it from
//             // here or add constraints to the column here.
//             trace[0].data[i] =
//                 PackedM31::broadcast(M31::from_u32_unchecked((i * N_LANES) as u32)) + inc;
//             for (j, value) in values.iter().enumerate() {
//                 trace[j + 1].data[i] = *value;
//             }
//             assert!(multiplicity.in_m31_range());
//             trace[MULTIPLICITY_COLUMN_OFFSET].data[i] = multiplicity.as_m31_unchecked();
//         }

//         // Lookup data.
//         let addresses_and_values = std::array::from_fn(|i| trace[i].data.clone());
//         let multiplicities = trace[MULTIPLICITY_COLUMN_OFFSET].data.clone();

//         // Extend trace.
//         let domain = CanonicCoset::new(LOG_MEMORY_ADDRESS_BOUND).circle_domain();
//         let trace = trace
//             .into_iter()
//             .map(|eval| CircleEvaluation::<SimdBackend, _, BitReversedOrder>::new(domain, eval))
//             .collect_vec();
//         tree_builder.extend_evals(trace);

//         (
//             AddrToIdClaim {
//                 log_address_bound: LOG_MEMORY_ADDRESS_BOUND,
//             },
//             InteractionClaimProver {
//                 adresses_and_values: addresses_and_values,
//                 multiplicities,
//             },
//         )
//     }
// }
