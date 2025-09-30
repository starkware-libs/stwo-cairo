#![allow(unused_parens)]
#![allow(dead_code)]
use std::simd::u32x16;

use cairo_air::components::verify_bitwise_xor_12::{
    Claim, InteractionClaim, EXPAND_BITS, LIMB_BITS, LOG_SIZE, N_MULT_COLUMNS,
};
use itertools::Itertools;

use crate::witness::prelude::*;

pub type InputType = [M31; 3];
pub type PackedInputType = [PackedM31; 3];

pub struct ClaimGenerator {
    pub mults: [AtomicMultiplicityColumn; N_MULT_COLUMNS],
}
impl ClaimGenerator {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            mults: std::array::from_fn(|_| AtomicMultiplicityColumn::new(1 << LOG_SIZE)),
        }
    }

    pub fn write_trace(
        self,
        tree_builder: &mut impl TreeBuilder<SimdBackend>,
    ) -> (Claim, InteractionClaimGenerator) {
        let mults = self.mults.map(AtomicMultiplicityColumn::into_simd_vec);

        let domain = CanonicCoset::new(LOG_SIZE).circle_domain();
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

    pub fn add_packed_inputs(&self, packed_inputs: &[PackedInputType]) {
        packed_inputs.into_par_iter().for_each(|packed_input| {
            packed_input.unpack().into_iter().for_each(|input| {
                self.add_input(&input);
            });
        });
    }
}

#[derive(Uninitialized, IterMut, ParIterMut)]
struct LookupData {
    mults: [Vec<PackedM31>; N_MULT_COLUMNS],
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
        let mut logup_gen = LogupTraceGenerator::new(LOG_SIZE);

        // [0, 1, 2, ..., N_LANES - 1].
        let zero_to_n_lanes = u32x16::from_array(std::array::from_fn(|i| i as u32));

        const EXPAND_BITS_MASK: u32 = (1 << EXPAND_BITS) - 1;

        // Batch two lookups at a time. `i0` and `i1` are the column indices of the two multiplicity
        // columns batched together.
        for ((i0, mults0), (i1, mults1)) in self.lookup_data.mults.into_iter().enumerate().tuples()
        {
            let mut col_gen = logup_gen.new_col();

            // Each multiplicity column represents a different combination of `EXPAND_BITS`
            // MSBs of each enumeration column. For example, if `EXPAND_BITS == 1`, then the
            // multiplicity columns are: 0b00, 0b01, 0b10, 0b11.
            // Extract ah, bh from column index, to be used as the high part of a and b.
            let ah0 = i0 as u32 >> EXPAND_BITS;
            let bh0 = i0 as u32 & EXPAND_BITS_MASK;

            // Repeat for the second lookup.
            let ah1 = i1 as u32 >> EXPAND_BITS;
            let bh1 = i1 as u32 & EXPAND_BITS_MASK;

            // Reconstruct the "expanded" values of a and b, and batch the lookups.
            (col_gen.par_iter_mut(), mults0, mults1)
                .into_par_iter()
                .enumerate()
                .for_each(|(vec_row, (writer, mults0, mults1))| {
                    // vec_row is LIMB_BITS of al and LIMB_BITS - LOG_N_LANES of bl. The low part of
                    // bl is just the consecutive numbers 0 .. N_LANES-1.
                    let al = vec_row as u32 >> (LIMB_BITS - LOG_N_LANES);
                    let blh = vec_row as u32 & ((1 << (LIMB_BITS - LOG_N_LANES)) - 1);

                    // Construct the 3 vectors a, b, c.
                    let a0 = u32x16::splat((ah0 << LIMB_BITS) | al);
                    let a1 = u32x16::splat((ah1 << LIMB_BITS) | al);
                    let b0 =
                        u32x16::splat((bh0 << LIMB_BITS) | (blh << LOG_N_LANES)) | zero_to_n_lanes;
                    let b1 =
                        u32x16::splat((bh1 << LIMB_BITS) | (blh << LOG_N_LANES)) | zero_to_n_lanes;

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
                    writer.write_frac(p0 * (-mults1) + p1 * (-mults0), p1 * p0);
                });
            col_gen.finalize_col();
        }

        let (trace, claimed_sum) = logup_gen.finalize_last();
        tree_builder.extend_evals(trace);

        InteractionClaim { claimed_sum }
    }
}
