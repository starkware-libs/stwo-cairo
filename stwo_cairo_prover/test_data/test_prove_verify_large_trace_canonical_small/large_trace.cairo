%builtins output pedersen range_check ecdsa bitwise ec_op keccak poseidon range_check96 add_mod mul_mod

// Performs many runtime adds that bounce a value in {1, 2}. The adds are real opcodes
// (deref + deref), so the add opcode count grows, while the set of distinct memory values stays tiny 
// (only {1, 2} plus the loop counter and frame pointers). This makes an opcode the largest component, 
// exceeding the CanonicalSmall preprocessed trace's max log size (20) => lifting_log_size > pp_log_size.
func add_routine(n: felt) {
    if (n == 0) {
        return ();
    }
    tempvar a = 1;
    tempvar b = 1;
    tempvar c0 = a + b;
    tempvar c1 = c0 - a;
    tempvar c2 = c1 + a;
    tempvar c3 = c2 - a;
    tempvar c4 = c3 + a;
    tempvar c5 = c4 - a;
    tempvar c6 = c5 + a;
    tempvar c7 = c6 - a;
    tempvar c8 = c7 + a;
    tempvar c9 = c8 - a;
    tempvar c10 = c9 + a;
    tempvar c11 = c10 - a;
    tempvar c12 = c11 + a;
    tempvar c13 = c12 - a;
    tempvar c14 = c13 + a;
    tempvar c15 = c14 - a;
    bounce_adds(n - 1);
    return ();
}

func main{
    output_ptr,
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
    add_routine(100000);
    return ();
}
