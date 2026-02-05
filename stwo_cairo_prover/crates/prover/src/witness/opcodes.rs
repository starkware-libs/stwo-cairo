use stwo_cairo_adapter::opcodes::CasmStatesByOpcode;

pub fn get_opcodes(casm_states_by_opcode: &CasmStatesByOpcode) -> Vec<&'static str> {
    let mut opcodes = vec![];
    if !casm_states_by_opcode.add_opcode.is_empty() {
        opcodes.push("add_opcode");
    }
    if !casm_states_by_opcode.add_opcode_small.is_empty() {
        opcodes.push("add_opcode_small");
    }
    if !casm_states_by_opcode.add_ap_opcode.is_empty() {
        opcodes.push("add_ap_opcode");
    }
    if !casm_states_by_opcode.assert_eq_opcode.is_empty() {
        opcodes.push("assert_eq_opcode");
    }
    if !casm_states_by_opcode.assert_eq_opcode_imm.is_empty() {
        opcodes.push("assert_eq_opcode_imm");
    }
    if !casm_states_by_opcode
        .assert_eq_opcode_double_deref
        .is_empty()
    {
        opcodes.push("assert_eq_opcode_double_deref");
    }
    if !casm_states_by_opcode.blake_compress_opcode.is_empty() {
        opcodes.push("blake_compress_opcode");
    }
    if !casm_states_by_opcode.call_opcode_abs.is_empty() {
        opcodes.push("call_opcode_abs");
    }
    if !casm_states_by_opcode.call_opcode_rel_imm.is_empty() {
        opcodes.push("call_opcode_rel_imm");
    }
    if !casm_states_by_opcode.generic_opcode.is_empty() {
        opcodes.push("generic_opcode");
    }
    if !casm_states_by_opcode.jnz_opcode_non_taken.is_empty() {
        opcodes.push("jnz_opcode_non_taken");
    }
    if !casm_states_by_opcode.jnz_opcode_taken.is_empty() {
        opcodes.push("jnz_opcode_taken");
    }
    if !casm_states_by_opcode.jump_opcode_abs.is_empty() {
        opcodes.push("jump_opcode_abs");
    }
    if !casm_states_by_opcode.jump_opcode_double_deref.is_empty() {
        opcodes.push("jump_opcode_double_deref");
    }
    if !casm_states_by_opcode.jump_opcode_rel.is_empty() {
        opcodes.push("jump_opcode_rel");
    }
    if !casm_states_by_opcode.jump_opcode_rel_imm.is_empty() {
        opcodes.push("jump_opcode_rel_imm");
    }
    if !casm_states_by_opcode.mul_opcode.is_empty() {
        opcodes.push("mul_opcode");
    }
    if !casm_states_by_opcode.mul_opcode_small.is_empty() {
        opcodes.push("mul_opcode_small");
    }
    if !casm_states_by_opcode.qm_31_add_mul_opcode.is_empty() {
        opcodes.push("qm_31_add_mul_opcode");
    }
    if !casm_states_by_opcode.ret_opcode.is_empty() {
        opcodes.push("ret_opcode");
    }

    opcodes
}
