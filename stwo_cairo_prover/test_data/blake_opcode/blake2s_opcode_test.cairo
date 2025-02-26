%builtins range_check bitwise

from starkware.cairo.common.alloc import alloc
from starkware.cairo.common.bool import FALSE, TRUE
from starkware.cairo.common.cairo_blake2s.blake2s import STATE_SIZE_FELTS, INPUT_BLOCK_FELTS, _get_sigma
from starkware.cairo.common.cairo_blake2s.packed_blake2s import N_PACKED_INSTANCES, blake2s_compress
from starkware.cairo.common.cairo_builtins import BitwiseBuiltin

const COUNTER = 64;
const U32_MASK = 0xffffffff;

// Tests the Blake2s and Blake2sLastBlock opcode runners using a preexisting implementation within the repo as reference.
// The initial state, a random message of 64 bytes and a counter are used as input.
// Both the opcode and the reference implementation are run on the same inputs and then their outputs are compared.
// Before comparing the outputs, it is verified that the opcode runner has written the output to the correct location.
func main{range_check_ptr, bitwise_ptr: BitwiseBuiltin*}() {
    run_blake_test(is_last_block=FALSE);
    run_blake_test(is_last_block=TRUE);
    return ();
}
func run_blake_test{range_check_ptr, bitwise_ptr: BitwiseBuiltin*}(is_last_block: felt) {
    alloc_locals;

    let (local random_message) = alloc();
    assert random_message[0] = 930933030;
    assert random_message[1] = 1766240503;
    assert random_message[2] = 3660871006;
    assert random_message[3] = 388409270;
    assert random_message[4] = 1948594622;
    assert random_message[5] = 3119396969;
    assert random_message[6] = 3924579183;
    assert random_message[7] = 2089920034;
    assert random_message[8] = 3857888532;
    assert random_message[9] = 929304360;
    assert random_message[10] = 1810891574;
    assert random_message[11] = 860971754;
    assert random_message[12] = 1822893775;
    assert random_message[13] = 2008495810;
    assert random_message[14] = 2958962335;
    assert random_message[15] = 2340515744;

    let (local input_state) = alloc();
    // Set the initial state to IV (IV[0] is modified).
    assert input_state[0] = 0x6B08E647;  // IV[0] ^ 0x01010020 (config: no key, 32 bytes output).
    assert input_state[1] = 0xBB67AE85;
    assert input_state[2] = 0x3C6EF372;
    assert input_state[3] = 0xA54FF53A;
    assert input_state[4] = 0x510E527F;
    assert input_state[5] = 0x9B05688C;
    assert input_state[6] = 0x1F83D9AB;
    assert input_state[7] = 0x5BE0CD19;
    static_assert STATE_SIZE_FELTS == 8;

    // Use the packed blake2s_compress to compute the output of the first instance.
    let (sigma) = _get_sigma();
    let (local cairo_output) = alloc();
    blake2s_compress(
        h=input_state,
        message=random_message,
        t0=COUNTER,
        f0=is_last_block * U32_MASK,
        sigma=sigma,
        output=cairo_output,
    );

    // Unpack the first instance of the blake2s_compress output (extract the first 32 bits).
    assert bitwise_ptr[0].x = cairo_output[0];
    assert bitwise_ptr[0].y = U32_MASK;
    assert bitwise_ptr[1].x = cairo_output[1];
    assert bitwise_ptr[1].y = U32_MASK;
    assert bitwise_ptr[2].x = cairo_output[2];
    assert bitwise_ptr[2].y = U32_MASK;
    assert bitwise_ptr[3].x = cairo_output[3];
    assert bitwise_ptr[3].y = U32_MASK;
    assert bitwise_ptr[4].x = cairo_output[4];
    assert bitwise_ptr[4].y = U32_MASK;
    assert bitwise_ptr[5].x = cairo_output[5];
    assert bitwise_ptr[5].y = U32_MASK;
    assert bitwise_ptr[6].x = cairo_output[6];
    assert bitwise_ptr[6].y = U32_MASK;
    assert bitwise_ptr[7].x = cairo_output[7];
    assert bitwise_ptr[7].y = U32_MASK;

    // Run the blake2s opcode runner on the same inputs and store its output.
    let vm_output = run_blake2s_opcode(
        is_last_block = is_last_block,
        dst=COUNTER,
        op0=input_state,
        op1=random_message,
    );

    // Verify that the opcode runner has written the 8 felts to the correct location.
    tempvar check_nonempty = vm_output[0];
    tempvar check_nonempty = vm_output[1];
    tempvar check_nonempty = vm_output[2];
    tempvar check_nonempty = vm_output[3];
    tempvar check_nonempty = vm_output[4];
    tempvar check_nonempty = vm_output[5];
    tempvar check_nonempty = vm_output[6];
    tempvar check_nonempty = vm_output[7];

    // Compare the vm_output to the blake2s_compress first instance output.
    assert vm_output[0] = bitwise_ptr[0].x_and_y;
    assert vm_output[1] = bitwise_ptr[1].x_and_y;
    assert vm_output[2] = bitwise_ptr[2].x_and_y;
    assert vm_output[3] = bitwise_ptr[3].x_and_y;
    assert vm_output[4] = bitwise_ptr[4].x_and_y;
    assert vm_output[5] = bitwise_ptr[5].x_and_y;
    assert vm_output[6] = bitwise_ptr[6].x_and_y;
    assert vm_output[7] = bitwise_ptr[7].x_and_y;

    let bitwise_ptr = bitwise_ptr + BitwiseBuiltin.SIZE * STATE_SIZE_FELTS;

    return ();
}

// Forces the runner to execute the Blake2s or Blake2sLastBlock opcode with the given operands.
// op0 is a pointer to an array of 8 felts as u32 integers of the state.
// op1 is a pointer to an array of 16 felts as u32 integers of the messsage.
// dst is a felt representing a u32 of the counter.
// ap contains a pointer to an array of 8 felts as u32 integers of the output state.
// Those values are stored within addresses fp-5, fp-4 and fp-3 respectively.
// An instruction encoding is built from offsets -5, -4, -3 and flags which are all 0 except for
// those denoting uses of fp as the base for operand addresses and flag_opcode_blake (16th flag).
// The instruction is then written to [pc] and the runner is forced to execute Blake2s.
func run_blake2s_opcode(
    is_last_block: felt,
    dst: felt,
    op0: felt*,
    op1: felt*,
) -> felt* {
    alloc_locals;

    // Set the offsets for the operands.
    let offset0 = (2**15)-5;
    let offset1 = (2**15)-4;
    let offset2 = (2**15)-3;
    static_assert dst == [fp - 5];
    static_assert op0 == [fp - 4];
    static_assert op1 == [fp - 3];

    // Set the flags for the instruction.
    let flag_dst_base_fp = 1;
    let flag_op0_base_fp = 1;
    let flag_op1_imm = 0;
    let flag_op1_base_fp = 1;
    let flag_op1_base_ap = 0;
    let flag_res_add = 0;
    let flag_res_mul = 0;
    let flag_PC_update_jump = 0;
    let flag_PC_update_jump_rel = 0;
    let flag_PC_update_jnz = 0;
    let flag_ap_update_add = 0;
    let flag_ap_update_add_1 = 0;
    let flag_opcode_call = 0;
    let flag_opcode_ret = 0;
    let flag_opcode_assert_eq = 0;

    let flag_num = flag_dst_base_fp+flag_op0_base_fp*(2**1)+flag_op1_imm*(2**2)+flag_op1_base_fp*(2**3);
    let blake2s_opcode_extension_num = 1;
    let blake2s_last_block_opcode_extension_num = 2;
    let blake2s_instruction_num = offset0 + offset1*(2**16) + offset2*(2**32) + flag_num*(2**48) + blake2s_opcode_extension_num*(2**63);
    let blake2s_last_block_instruction_num = offset0 + offset1*(2**16) + offset2*(2**32) + flag_num*(2**48) + blake2s_last_block_opcode_extension_num*(2**63);
    static_assert blake2s_instruction_num==9226608988349300731;
    static_assert blake2s_last_block_instruction_num==18449981025204076539;

    // Write the instruction to [pc] and point [ap] to the designated output.
    let (local vm_output) = alloc();
    assert [ap] = cast(vm_output, felt);

    jmp last_block if is_last_block!=0;
    dw 9226608988349300731;
    return cast([ap], felt*);

    last_block:
    dw 18449981025204076539;
    return cast([ap], felt*);
}
