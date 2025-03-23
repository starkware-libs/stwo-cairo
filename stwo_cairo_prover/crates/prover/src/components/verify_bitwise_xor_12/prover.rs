#![allow(unused_parens)]
// TODO(Ohad): remove allow unused.
#![allow(unused)]
use std::array;
use std::simd::u32x16;

use itertools::Itertools;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;

use super::component::{Claim, InteractionClaim};
use super::EXPAND_BITS;
use crate::cairo_air::preprocessed::BitwiseXor;
use crate::components::prelude::proving::*;
use crate::components::verify_bitwise_xor_12::LIMB_BITS;

pub type InputType = [M31; 3];

const PACKED_LOG_SIZE: u32 = super::LOG_SIZE - LOG_N_LANES;

pub struct ClaimGenerator {
    pub mults: [AtomicMultiplicityColumn; super::N_MULT_COLUMNS],
}
impl ClaimGenerator {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            mults: array::from_fn(|_| AtomicMultiplicityColumn::new(1 << super::LOG_SIZE)),
        }
    }

    pub fn write_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
    ) -> (Claim, InteractionClaimGenerator) {
        let mults = self.mults.map(AtomicMultiplicityColumn::into_simd_vec);

        let domain = CanonicCoset::new(super::LOG_SIZE).circle_domain();
        let trace = mults
            .iter()
            .cloned()
            .map(BaseColumn::from_simd)
            .map(|col| CircleEvaluation::new(domain, col))
            .collect_vec();

        let lookup_data = LookupData { mults };
        tree_builder.extend_evals(trace);

        (Claim {}, InteractionClaimGenerator { lookup_data })
    }

    pub fn add_input(&self, [M31(a), M31(b), ..]: &InputType) {
        let [[al, ah], [bl, bh]] = [*a, *b].map(|x| [x & ((1 << LIMB_BITS) - 1), x >> LIMB_BITS]);
        let column_index = (ah << EXPAND_BITS) + bh;
        let row_index = (al << LIMB_BITS) + bl;
        self.mults[column_index as usize].increase_at(row_index);
    }

    pub fn add_inputs(&self, inputs: &[InputType]) {
        for input in inputs {
            self.add_input(input);
        }
    }
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    mults: [Vec<PackedM31>; super::N_MULT_COLUMNS],
}

pub struct InteractionClaimGenerator {
    lookup_data: LookupData,
}
impl InteractionClaimGenerator {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
        verify_bitwise_xor_12: &relations::VerifyBitwiseXor_12,
    ) -> InteractionClaim {
        let mut logup_gen = LogupTraceGenerator::new(super::LOG_SIZE);
        let offsets_vec = u32x16::from_array(std::array::from_fn(|i| i as u32));
        const EXPAND_BITS_MASK: u32 = (1 << EXPAND_BITS) - 1;

        for ((i0, mults0), (i1, mults1)) in self.lookup_data.mults.into_iter().enumerate().tuples()
        {
            let mut col_gen = logup_gen.new_col();

            // Extract ah, bh from column index.
            let ah0 = i0 as u32 >> EXPAND_BITS;
            let bh0 = i0 as u32 & EXPAND_BITS_MASK;
            let ah1 = i1 as u32 >> EXPAND_BITS;
            let bh1 = i1 as u32 & EXPAND_BITS_MASK;

            for (vec_row, (mult0, mult1)) in zip(mults0, mults1).enumerate() {
                // vec_row is LIMB_BITS of al and LIMB_BITS - LOG_N_LANES of bl.
                // Extract al, and the high part of bl (blh) from vec_row.
                let al = vec_row as u32 >> (LIMB_BITS - LOG_N_LANES);
                let blh = vec_row as u32 & ((1 << (LIMB_BITS - LOG_N_LANES)) - 1);

                // Construct the 3 vectors a, b, c.
                let a0 = u32x16::splat((ah0 << LIMB_BITS) | al);
                let a1 = u32x16::splat((ah1 << LIMB_BITS) | al);
                // bll is just the consecutive numbers 0 .. N_LANES-1.
                let b0 = u32x16::splat((bh0 << LIMB_BITS) | (blh << LOG_N_LANES)) | offsets_vec;
                let b1 = u32x16::splat((bh1 << LIMB_BITS) | (blh << LOG_N_LANES)) | offsets_vec;

                let c0 = a0 ^ b0;
                let c1 = a1 ^ b1;

                let (v0, v1) = unsafe {
                    (
                        [a0, b0, c0].map(|v| PackedM31::from_simd_unchecked(v)),
                        [a1, b1, c1].map(|v| PackedM31::from_simd_unchecked(v)),
                    )
                };

                let p0: PackedQM31 = verify_bitwise_xor_12.combine(&v0);
                let p1: PackedQM31 = verify_bitwise_xor_12.combine(&v1);
                col_gen.write_frac(vec_row, p0 * (-mult1) + p1 * (-mult0), p1 * p0);
            }
            col_gen.finalize_col();
        }

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}
