%builtins output pedersen range_check ecdsa bitwise ec_op keccak poseidon range_check96 add_mod mul_mod

from starkware.cairo.common.alloc import alloc
from starkware.cairo.common.cairo_builtins import PoseidonBuiltin
from stwo_cairo_prover.test_data.test_prove_verify_poseidon_builtin.do_15_poseidon import do_15_poseidon

// The main function. Reads the number of usages for each builtin from the input,
// and calls each builtin accordingly.
func main{
    output_ptr,
    pedersen_ptr,
    range_check_ptr,
    ecdsa_ptr,
    bitwise_ptr,
    ec_op_ptr,
    keccak_ptr,
    poseidon_ptr: PoseidonBuiltin*,
    range_check96_ptr,
    add_mod_ptr,
    mul_mod_ptr,
}() {
    alloc_locals;
    local n_poseidon = 30;

    // Call poseidon builtin.
    do_15_poseidon(n_times=n_poseidon);

    return ();
}
