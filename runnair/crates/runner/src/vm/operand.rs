use stwo_prover::core::fields::m31::M31;

use super::State;
use crate::memory::{MaybeRelocatableValue, Memory};

// Adds:
pub fn add_ap_ap(memory: &Memory, state: State, args: &[M31]) -> MaybeRelocatableValue {
    memory[state.ap + args[0]] + memory[state.ap + args[1]]
}

pub fn add_ap_fp(memory: &Memory, state: State, args: &[M31]) -> MaybeRelocatableValue {
    memory[state.ap + args[0]] + memory[state.fp + args[1]]
}

pub fn add_fp_ap(memory: &Memory, state: State, args: &[M31]) -> MaybeRelocatableValue {
    memory[state.fp + args[0]] + memory[state.ap + args[1]]
}

pub fn add_fp_fp(memory: &Memory, state: State, args: &[M31]) -> MaybeRelocatableValue {
    memory[state.fp + args[0]] + memory[state.fp + args[1]]
}

pub fn add_imm_ap(memory: &Memory, state: State, args: &[M31]) -> MaybeRelocatableValue {
    memory[state.ap + args[1]] + args[0]
}

pub fn add_imm_fp(memory: &Memory, state: State, args: &[M31]) -> MaybeRelocatableValue {
    memory[state.fp + args[1]] + args[0]
}

// Muls:
pub fn mul_ap_ap(memory: &Memory, state: State, args: &[M31]) -> MaybeRelocatableValue {
    memory[state.ap + args[0]] * memory[state.ap + args[1]]
}

pub fn mul_ap_fp(memory: &Memory, state: State, args: &[M31]) -> MaybeRelocatableValue {
    memory[state.ap + args[0]] * memory[state.fp + args[1]]
}

pub fn mul_fp_ap(memory: &Memory, state: State, args: &[M31]) -> MaybeRelocatableValue {
    memory[state.fp + args[0]] * memory[state.ap + args[1]]
}

pub fn mul_fp_fp(memory: &Memory, state: State, args: &[M31]) -> MaybeRelocatableValue {
    memory[state.fp + args[0]] * memory[state.fp + args[1]]
}

pub fn mul_imm_ap(memory: &Memory, state: State, args: &[M31]) -> MaybeRelocatableValue {
    memory[state.ap + args[1]] * args[0]
}

pub fn mul_imm_fp(memory: &Memory, state: State, args: &[M31]) -> MaybeRelocatableValue {
    memory[state.fp + args[1]] * args[0]
}

// Derefs:
pub fn imm(_memory: &Memory, _state: State, args: &[M31]) -> MaybeRelocatableValue {
    MaybeRelocatableValue::Absolute(args[0].into())
}

pub fn deref_ap(memory: &Memory, state: State, args: &[M31]) -> MaybeRelocatableValue {
    memory[state.ap + args[0]]
}

pub fn deref_fp(memory: &Memory, state: State, args: &[M31]) -> MaybeRelocatableValue {
    memory[state.fp + args[0]]
}

pub fn double_deref_ap(memory: &Memory, state: State, args: &[M31]) -> MaybeRelocatableValue {
    memory[memory[state.ap + args[0]] + args[1]]
}

pub fn double_deref_fp(memory: &Memory, state: State, args: &[M31]) -> MaybeRelocatableValue {
    memory[memory[state.fp + args[0]] + args[1]]
}
