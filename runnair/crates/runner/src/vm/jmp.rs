use paste::paste;
use stwo_prover::core::fields::m31::M31;

use crate::memory::relocatable::assert_and_project;
use crate::memory::{MaybeRelocatableValue, Memory};
use crate::vm::{InstructionArgs, State};

pub fn jmp_rel(state: State, operand: M31) -> State {
    State {
        ap: state.ap,
        fp: state.fp,
        pc: state.pc + operand,
    }
}

pub fn jmp_rel_appp(state: State, operand: M31) -> State {
    State {
        ap: state.ap + M31(1),
        fp: state.fp,
        pc: state.pc + operand,
    }
}

pub fn jmp_abs(state: State, operand: M31) -> State {
    State {
        ap: state.ap,
        fp: state.fp,
        pc: operand,
    }
}

pub fn jmp_abs_appp(state: State, operand: M31) -> State {
    State {
        ap: state.ap + M31(1),
        fp: state.fp,
        pc: operand,
    }
}

macro_rules! jmp_with_operand {
    ($operand:ident) => {
        paste! {
            pub fn [<jmp_rel_ $operand>](
                memory: &mut Memory,
                state: State,
                args: InstructionArgs,
            ) -> State {
                if let MaybeRelocatableValue::Absolute(offset) =
                    crate::vm::operand::$operand(memory, state, &args)
                {
                    jmp_rel(state, assert_and_project(offset))
                } else {
                    panic!("Can't jump to a relocatable.")
                }
            }

            pub fn [<jmp_rel_ $operand _appp>](
                memory: &mut Memory,
                state: State,
                args: InstructionArgs,
            ) -> State {
                if let MaybeRelocatableValue::Absolute(offset) =
                    crate::vm::operand::$operand(memory, state, &args)
                {
                    jmp_rel_appp(state, assert_and_project(offset))
                } else {
                    panic!("Can't jump to a relocatable.")
                }
            }

            pub fn [<jmp_abs_ $operand>](
                memory: &mut Memory,
                state: State,
                args: InstructionArgs,
            ) -> State {
                if let MaybeRelocatableValue::Absolute(offset) =
                    crate::vm::operand::$operand(memory, state, &args)
                {
                    jmp_abs(state, assert_and_project(offset))
                } else {
                    panic!("Can't jump to a relocatable.")
                }
            }

            pub fn [<jmp_abs_ $operand _appp>](
                memory: &mut Memory,
                state: State,
                args: InstructionArgs,
            ) -> State {
                if let MaybeRelocatableValue::Absolute(offset) =
                    crate::vm::operand::$operand(memory, state, &args)
                {
                    jmp_abs_appp(state, assert_and_project(offset))
                } else {
                    panic!("Can't jump to a relocatable.")
                }
            }

        }
    };
}

jmp_with_operand!(add_ap_ap);
jmp_with_operand!(add_ap_fp);
jmp_with_operand!(add_fp_ap);
jmp_with_operand!(add_fp_fp);
jmp_with_operand!(add_imm_ap);
jmp_with_operand!(add_imm_fp);
jmp_with_operand!(deref_ap);
jmp_with_operand!(deref_fp);
jmp_with_operand!(double_deref_ap);
jmp_with_operand!(double_deref_fp);
jmp_with_operand!(imm);
jmp_with_operand!(mul_ap_ap);
jmp_with_operand!(mul_ap_fp);
jmp_with_operand!(mul_fp_ap);
jmp_with_operand!(mul_fp_fp);
jmp_with_operand!(mul_imm_ap);
jmp_with_operand!(mul_imm_fp);
