pub use std::iter::zip;
pub use std::simd::Simd;

pub use cairo_air::preprocessed::Seq;
pub(crate) use cairo_air::relations;
pub use num_traits::{One, Zero};
pub use rayon::prelude::*;
pub use stwo_air_utils::trace::component_trace::ComponentTrace;
pub use stwo_air_utils_derive::{IterMut, ParIterMut, Uninitialized};
pub use stwo_cairo_common::prover_types::cpu::*;
pub use stwo_cairo_common::prover_types::simd::*;
pub use stwo_prover::constraint_framework::logup::LogupTraceGenerator;
pub use stwo_prover::constraint_framework::Relation;
pub use stwo_prover::core::backend::simd::column::BaseColumn;
pub use stwo_prover::core::backend::simd::conversion::Unpack;
pub use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES, N_LANES};
pub use stwo_prover::core::backend::simd::qm31::PackedQM31;
pub use stwo_prover::core::backend::simd::SimdBackend;
pub use stwo_prover::core::backend::Column;
pub use stwo_prover::core::fields::m31::M31;
pub use stwo_prover::core::fields::FieldExpOps;
pub use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
pub use stwo_prover::core::poly::BitReversedOrder;

pub use crate::witness::fast_deduction::blake::{BlakeG, BlakeRound, BlakeRoundSigma, TripleXor32};
pub use crate::witness::fast_deduction::poseidon::{
    PackedCube252, PackedPoseidon3PartialRoundsChain, PackedPoseidonFullRoundChain,
    PackedPoseidonRoundKeys,
};
pub use crate::witness::utils::*;
