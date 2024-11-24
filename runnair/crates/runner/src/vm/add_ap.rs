use stwo_prover::core::fields::m31::M31;

use super::{Instruction, State};
use crate::memory::relocatable::assert_and_project;
use crate::memory::{MaybeRelocatableValue, Memory};

pub fn addap(state: State, summand: M31) -> State {
    State {
        ap: state.ap + summand,
        fp: state.fp,
        pc: state.pc,
    }
}

macro_rules! addap_with_operand {
    ($instruction:ident, $operand:ident) => {
        pub fn $instruction(memory: &mut Memory, state: State, instruction: Instruction) -> State {
            if let MaybeRelocatableValue::Absolute(summand) =
                crate::vm::operand::$operand(memory, state, &instruction.args)
            {
                addap(state, assert_and_project(summand))
            } else {
                panic!("Can't addap by a relocatable.")
            }
        }
    };
}

addap_with_operand!(addap_add_ap_ap, add_ap_ap);
addap_with_operand!(addap_add_ap_fp, add_ap_fp);
addap_with_operand!(addap_add_fp_ap, add_fp_ap);
addap_with_operand!(addap_add_fp_fp, add_fp_fp);
addap_with_operand!(addap_add_imm_ap, add_imm_ap);
addap_with_operand!(addap_add_imm_fp, add_imm_fp);
addap_with_operand!(addap_deref_ap, deref_ap);
addap_with_operand!(addap_deref_fp, deref_fp);
addap_with_operand!(addap_double_deref_ap, double_deref_ap);
addap_with_operand!(addap_double_deref_fp, double_deref_fp);
addap_with_operand!(addap_imm, imm);
addap_with_operand!(addap_mul_ap_ap, mul_ap_ap);
addap_with_operand!(addap_mul_ap_fp, mul_ap_fp);
addap_with_operand!(addap_mul_fp_ap, mul_fp_ap);
addap_with_operand!(addap_mul_fp_fp, mul_fp_fp);
addap_with_operand!(addap_mul_imm_ap, mul_imm_ap);
addap_with_operand!(addap_mul_imm_fp, mul_imm_fp);
