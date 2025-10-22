%builtins output pedersen range_check ecdsa bitwise ec_op keccak poseidon range_check96 add_mod mul_mod

from starkware.cairo.common.alloc import alloc
from starkware.cairo.common.bool import FALSE, TRUE
from starkware.cairo.common.registers import get_fp_and_pc

func main{
    output_ptr: felt,
    pedersen_ptr,
    range_check_ptr,
    ecdsa_ptr,
    bitwise_ptr,
    ec_op_ptr,
    keccak_ptr,
    poseidon_ptr,
    range_check96_ptr,
    add_mod_ptr,
    mul_mod_ptr,
}() {
    blake2s();
    ap+=1;
    add_ap();
    ap+=1;
    jump_rel_imm();
    ap+=1;
    jump_abs();
    ap+=1;
    call_abs();
    ap+=2;
    call_abs_ap();
    ap+=2;
    jnz_not_taken_ap();
    ap+=1;
    jnz_not_taken_fp();
    ap+=1;
    jnz_taken_fp();
    ap+=1;
    jnz_taken_ap();
    ap+=1;
    assert_eq();
    ap+=2;
    add_small();
    ap+=1;
    add_big();
    ap+=1;
    mul_small();
    ap+=1;
    qm31();
    ap+=1;
    assert_eq_double_deref();
    ap+=1;
    mul_big();
    ap+=1;
    generic();
    ap+=1;
    jump_rel();
    ap+=1;
    jump_abs_double_deref();

    // jump_opcode_abs
    // call rel 2;
    // [ap] = [ap-1] + 3;
    // ap+=1;
    // jmp abs [ap-1];

    // //jump_opcode_rel_imm
    // // jmp rel 1;
    // // ap+=1;
    // ap+=1;
    // [ap] = 10, ap++;
    // [ap] = 12, ap++;
    // ap += [ap -2];
    // ap += [fp + 1];
    // ap += 1;
    // [ap] = 1, ap++;

    return ();
}

func add_ap() {
    [ap] = 38, ap++;
    [ap] = 12, ap++;
    ap += [ap -2];
    ap += [fp + 1];
    ap += 1;
    [ap] = 1, ap++;
    ret;
}

func jump_rel_imm(){
    jmp rel 2;
    [ap] = [ap-1] + 3, ap++;
    ret;
}

func jump_abs(){
    call rel 2;
    [ap] = [ap-1] + 3;
    jmp abs [ap];
    ret;
}

func call_abs(){
    alloc_locals;
    let (_, local __pc__) = get_fp_and_pc();
    local addr = cast(__pc__ + 4, felt);
    call abs addr;
    ret;
}

func call_abs_ap(){
    alloc_locals;
    let (_, local __pc__) = get_fp_and_pc();
    tempvar addr = cast(__pc__ + 4, felt);
    call abs addr;
    ret;
}

func jnz_not_taken_ap(){
    [ap] = 0, ap++;
    jmp rel 2 if [ap-1] != 0;
    ret;
}

func jnz_not_taken_fp(){
    call rel 2;
    [ap] = 0, ap++;
    jmp rel 2 if [fp] != 0;
    [ap] = 1, ap++;
    ret;
}

func jnz_taken_fp(){
    call rel 2;
    jmp rel 2 if [fp-1] != 0;
    [ap] = 1, ap++;
    ret;
}

func jnz_taken_ap(){
    [ap] = 5, ap++;
    jmp rel 2 if [ap-1] != 0;
    [ap] = 1, ap++;
    ret;
}

func assert_eq(){
    [ap] =  8, ap++;
    [ap] =  8, ap++;
    [ap+2] = [fp + 1];
    [ap] = 1, ap++;
    ret;
}

func add_small() {
    call rel 2;
    [ap] = 134217725, ap++;
    [ap] = 2, ap++;
    // 134217725 + 2= 2^27-1.
    [ap] = [fp] + [ap-1], ap++;
    // 134217724 + 3 = 2^27-1.
    [ap] = [fp-1] + 134217724, ap++;
    [ap] = 1, ap++;
    ret;
}

func add_big() {
    call rel 2;
    [ap] = 134217725, ap++;
    [ap] = 3, ap++;
    // 134217725 + 3 = is 2^27.
    [ap] = [fp] + [ap-1], ap++;
    [ap] = [ap-1] + 1, ap++;
    [ap] = 1, ap++;
    ret;
}

func mul_small(){
   // 2^36-1 is the maximal factor value for a small mul.
   [ap] =  262145, ap++;
   [ap] =  [ap-1]*262143, ap++;
   // 2^36-1 is the maximal factor value for a small mul.
   [ap] = [ap-1], ap++;
   [ap] = [ap-1] * [ap-2], ap++;
   [ap] = [ap-2]*2147483647, ap++;
   [ap] = 1, ap++;
   ret; 
}

func mul_big(){
    [ap] =  8, ap++;
    // 2^36 is the minimal factor value for a big mul.
    [ap] = 262144, ap++;
    [ap] = [ap-1] * 262144, ap++;
    [ap] = [ap-1] * [ap-3], ap++;
    [ap] = [ap-2]* 2, ap++;
    [ap] = 1, ap++;
    ret; 
}

func assert_eq_double_deref(){
    call rel 2;
    ap += 2;
    [ap] = 100, ap++;
    [ap] = [[fp - 1] + 2], ap++;  // [fp - 2] is the old fp.
    [ap] = 5;
    ret;
}

func generic(){
    [ap]=1, ap++;
    [ap]=2, ap++;
    jmp rel [ap-2] if [ap-1] != 0;
    [ap]=1, ap++;
    ret;
}

func jump_rel(){
    [ap] = 1, ap++;
    jmp rel [ap-1];
    [ap] = 2, ap++;
    ret;
}
func jump_abs_double_deref(){
    alloc_locals;
    let (_, local __pc__) = get_fp_and_pc();
    local x = cast(__pc__ + 8, felt);
    call rel 2;
    jmp abs [[ap - 2] + 1];
    [ap] = 5;
    ret;
}

func qm31() {
    let qm31_op0_coordinates_a = 0x544b2fba;
    let qm31_op0_coordinates_b = 0x673cff77;
    let qm31_op0_coordinates_c = 0x60713d44;
    let qm31_op0_coordinates_d = 0x499602d2;
    let qm31_op0 = qm31_op0_coordinates_a + qm31_op0_coordinates_b*(2**36) + qm31_op0_coordinates_c*(2**72) + qm31_op0_coordinates_d*(2**108);

    let qm31_op1_coordinates_a = 0x4b18de99;
    let qm31_op1_coordinates_b = 0x55f6fb62;
    let qm31_op1_coordinates_c = 0x6e2290d9;
    let qm31_op1_coordinates_d = 0x7cd851b9;
    let qm31_op1 = qm31_op1_coordinates_a + qm31_op1_coordinates_b*(2**36) + qm31_op1_coordinates_c*(2**72) + qm31_op1_coordinates_d*(2**108);

    let qm31_add_dst_coordinates_a = 0x1f640e54;
    let qm31_add_dst_coordinates_b = 0x3d33fada;
    let qm31_add_dst_coordinates_c = 0x4e93ce1e;
    let qm31_add_dst_coordinates_d = 0x466e548c;
    let qm31_add_dst = qm31_add_dst_coordinates_a + qm31_add_dst_coordinates_b*(2**36) + qm31_add_dst_coordinates_c*(2**72) + qm31_add_dst_coordinates_d*(2**108);

    let qm31_mul_dst_coordinates_a = 0x38810ab4;
    let qm31_mul_dst_coordinates_b = 0x5a0fd30a;
    let qm31_mul_dst_coordinates_c = 0x2527b81e;
    let qm31_mul_dst_coordinates_d = 0x4b1ed1cd;
    let qm31_mul_dst = qm31_mul_dst_coordinates_a + qm31_mul_dst_coordinates_b*(2**36) + qm31_mul_dst_coordinates_c*(2**72) + qm31_mul_dst_coordinates_d*(2**108);

    let runner_output_mul_dst = run_qm31_operation(missing_operand=0, is_imm=FALSE, is_mul=TRUE, dst_or_op0=qm31_op0, op0_or_op1=qm31_op1);
    assert runner_output_mul_dst = qm31_mul_dst;
    let runner_output_add_dst = run_qm31_operation(missing_operand=0, is_imm=FALSE, is_mul=FALSE, dst_or_op0=qm31_op0, op0_or_op1=qm31_op1);
    assert runner_output_add_dst = qm31_add_dst;

    let runner_output_mul_op0 = run_qm31_operation(missing_operand=1, is_imm=FALSE, is_mul=TRUE, dst_or_op0=qm31_mul_dst, op0_or_op1=qm31_op1);
    assert runner_output_mul_op0 = qm31_op0;
    let runner_output_add_op0 = run_qm31_operation(missing_operand=1, is_imm=FALSE, is_mul=FALSE, dst_or_op0=qm31_add_dst, op0_or_op1=qm31_op1);
    assert runner_output_add_op0 = qm31_op0;

    let runner_output_mul_op1 = run_qm31_operation(missing_operand=2, is_imm=FALSE, is_mul=TRUE, dst_or_op0=qm31_mul_dst, op0_or_op1=qm31_op0);
    assert runner_output_mul_op1 = qm31_op1;
    let runner_output_add_op1 = run_qm31_operation(missing_operand=2, is_imm=FALSE, is_mul=FALSE, dst_or_op0=qm31_add_dst, op0_or_op1=qm31_op0);
    assert runner_output_add_op1 = qm31_op1;

    let runner_output_mul_dst = run_qm31_operation(missing_operand=0, is_imm=TRUE, is_mul=TRUE, dst_or_op0=qm31_op0, op0_or_op1=qm31_op1);
    assert runner_output_mul_dst = qm31_mul_dst;
    let runner_output_add_dst = run_qm31_operation(missing_operand=0, is_imm=TRUE, is_mul=FALSE, dst_or_op0=qm31_op0, op0_or_op1=qm31_op1);
    assert runner_output_add_dst = qm31_add_dst;

    let runner_output_mul_op0 = run_qm31_operation(missing_operand=1, is_imm=TRUE, is_mul=TRUE, dst_or_op0=qm31_mul_dst, op0_or_op1=qm31_op1);
    assert runner_output_mul_op0 = qm31_op0;
    let runner_output_add_op0 = run_qm31_operation(missing_operand=1, is_imm=TRUE, is_mul=FALSE, dst_or_op0=qm31_add_dst, op0_or_op1=qm31_op1);
    assert runner_output_add_op0 = qm31_op0;

    return ();
}

func run_qm31_operation(
    missing_operand: felt,
    is_imm: felt,
    is_mul: felt,
    dst_or_op0: felt,
    op0_or_op1: felt,
) -> felt {
    alloc_locals;

    // Set flags and offsets.
    let (local offsets) = alloc();
    let (local flags) = alloc();

    assert offsets[missing_operand] = 2**15; // the missing operand will be written to [ap]

    assert flags[2] = is_imm; // flag_op1_imm = 0;
    assert flags[5] = 1-is_mul; // flag_res_add = 1-is_mul;
    assert flags[6] = is_mul; // flag_res_mul = is_mul;
    assert flags[7] = 0; // flag_PC_update_jump = 0;
    assert flags[8] = 0; // flag_PC_update_jump_rel = 0;
    assert flags[9] = 0; // flag_PC_update_jnz = 0;
    assert flags[10] = 0; // flag_ap_update_add = 0;
    assert flags[11] = 0; // flag_ap_update_add_1 = 0;
    assert flags[12] = 0; // flag_opcode_call = 0;
    assert flags[13] = 0; // flag_opcode_ret = 0;
    assert flags[14] = 1; // flag_opcode_assert_eq = 1;

    if (missing_operand == 0) {
        assert offsets[1] = 2**15 - 4;
        assert offsets[2] = 2**15 - 3 + 4 * is_imm;
        assert flags[0] = 0; // flag_dst_base_fp
        assert flags[1] = 1; // flag_op0_base_fp
    }
    if (missing_operand == 1) {
        assert offsets[0] = 2**15 - 4;
        assert offsets[2] = 2**15 - 3 + 4 * is_imm;
        assert flags[0] = 1; // flag_dst_base_fp
        assert flags[1] = 0; // flag_op0_base_fp
    }
    if (missing_operand == 2) {
        assert is_imm = FALSE;
        assert offsets[0] = 2**15 - 4;
        assert offsets[1] = 2**15 - 3;
        assert flags[0] = 1; // flag_dst_base_fp
        assert flags[1] = 1; // flag_op0_base_fp
    }
    assert flags[3] = (2 - flags[0] - flags[1]) * (1 - is_imm); // flag_op1_base_fp
    assert flags[4] = 1 - is_imm - flags[3]; // flag_op1_base_ap

    // Compute the instruction encoding.
    let flag_num = flags[0] + flags[1]*(2**1) + flags[2]*(2**2) + flags[3]*(2**3) + flags[4]*(2**4) + flags[5]*(2**5) + flags[6]*(2**6) + flags[14]*(2**14);
    let qm31_opcode_extension_num = 3;
    let instruction_encoding = offsets[0] + offsets[1]*(2**16) + offsets[2]*(2**32) + flag_num*(2**48) + qm31_opcode_extension_num*(2**63);

    // Run the instruction and return the result.
    if (is_imm == TRUE) {
        assert op0_or_op1 = 0x7cd851b906e2290d9055f6fb6204b18de99;
        if (missing_operand == 0) {
            if (is_mul == TRUE) {
                assert instruction_encoding=0x1c04680017ffc8000;
                dw 0x1c04680017ffc8000;
                dw 0x7cd851b906e2290d9055f6fb6204b18de99;
                return [ap];
            }
            assert instruction_encoding=0x1c02680017ffc8000;
            dw 0x1c02680017ffc8000;
            dw 0x7cd851b906e2290d9055f6fb6204b18de99;
            return [ap];
        }
        if (missing_operand == 1) {
            if (is_mul == TRUE) {
                assert instruction_encoding=0x1c045800180007ffc;
                dw 0x1c045800180007ffc;
                dw 0x7cd851b906e2290d9055f6fb6204b18de99;
                return [ap];
            }
            assert instruction_encoding=0x1c025800180007ffc;
            dw 0x1c025800180007ffc;
            dw 0x7cd851b906e2290d9055f6fb6204b18de99;
            return [ap];
        }
    }

    if (missing_operand == 0) {
        if (is_mul == TRUE) {
            assert instruction_encoding=0x1c04a7ffd7ffc8000;
            dw 0x1c04a7ffd7ffc8000;
            return [ap];
        }
        assert instruction_encoding=0x1c02a7ffd7ffc8000;
        dw 0x1c02a7ffd7ffc8000;
        return [ap];
    }
    if (missing_operand == 1) {
        if (is_mul == TRUE) {
            assert instruction_encoding=0x1c0497ffd80007ffc;
            dw 0x1c0497ffd80007ffc;
            return [ap];
        }
        assert instruction_encoding=0x1c0297ffd80007ffc;
        dw 0x1c0297ffd80007ffc;
        return [ap];
    }
    if (is_mul == TRUE) {
        assert instruction_encoding=0x1c05380007ffd7ffc;
        dw 0x1c05380007ffd7ffc;
        return [ap];
    }
    assert instruction_encoding=0x1c03380007ffd7ffc;
    dw 0x1c03380007ffd7ffc;
    return [ap];
}

from starkware.cairo.common.cairo_blake2s.blake2s import STATE_SIZE_FELTS, INPUT_BLOCK_FELTS, _get_sigma
from starkware.cairo.common.cairo_blake2s.packed_blake2s import N_PACKED_INSTANCES

const COUNTER = 64;
const U32_MASK = 0xffffffff;

// Tests the Blake2s and Blake2sLastBlock opcode runners using a preexisting implementation within the repo as reference.
// The initial state, a random message of 64 bytes and a counter are used as input.
// Both the opcode and the reference implementation are run on the same inputs and then their outputs are compared.
// Before comparing the outputs, it is verified that the opcode runner has written the output to the correct location.
func blake2s{}() {
    run_blake_test(is_last_block=FALSE);
    run_blake_test(is_last_block=TRUE);
    return ();
}
func run_blake_test{}(is_last_block: felt) {
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


    // Run the blake2s opcode runner on the same inputs and store its output.
    let vm_output = run_blake_compress_opcode(
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

    return ();
}

// Forces the runner to execute the Blake2s or Blake2sLastBlock opcode with the given operands.
// op0 is a pointer to an array of 8 felts as u32 integers of the state.
// op1 is a pointer to an array of 16 felts as u32 integers of the message.
// dst is a felt representing a u32 of the counter.
// ap contains a pointer to an array of 8 felts as u32 integers of the output state.
// Those values are stored within addresses fp-5, fp-4 and fp-3 respectively.
// An instruction encoding is built from offsets -5, -4, -3 and flags which are all 0 except for
// those denoting uses of fp as the base for operand addresses and flag_opcode_blake (16th flag).
// The instruction is then written to [pc] and the runner is forced to execute Blake2s.
func run_blake_compress_opcode(
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
    let blake_compress_opcode_extension_num = 1;
    let blake_compress_last_block_opcode_extension_num = 2;
    let blake_compress_instruction_num = offset0 + offset1*(2**16) + offset2*(2**32) + flag_num*(2**48) + blake_compress_opcode_extension_num*(2**63);
    let blake_compress_last_block_instruction_num = offset0 + offset1*(2**16) + offset2*(2**32) + flag_num*(2**48) + blake_compress_last_block_opcode_extension_num*(2**63);
    static_assert blake_compress_instruction_num==9226608988349300731;
    static_assert blake_compress_last_block_instruction_num==18449981025204076539;

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
