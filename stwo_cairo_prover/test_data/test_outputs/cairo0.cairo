%builtins output pedersen range_check ecdsa bitwise ec_op keccak poseidon range_check96 add_mod mul_mod

from starkware.cairo.common.alloc import alloc
from starkware.cairo.common.cairo_builtins import (
    BitwiseBuiltin,
    EcOpBuiltin,
    HashBuiltin,
    KeccakBuiltin,
    ModBuiltin,
    PoseidonBuiltin,
    SignatureBuiltin,
    UInt384,
)
from starkware.cairo.common.keccak_state import KeccakBuiltinState
from starkware.cairo.common.memcpy import memcpy
from starkware.cairo.common.modulo import BATCH_SIZE
from starkware.cairo.common.poseidon_state import PoseidonBuiltinState
from starkware.cairo.common.registers import get_label_location

func do_output{output_ptr: felt*}(n_builtin_usages: felt) {
    if (n_builtin_usages == 0) {
        return ();
    }

    assert [output_ptr] = 1000 + n_builtin_usages;
    let output_ptr = output_ptr + 1;

    do_output(n_builtin_usages=n_builtin_usages - 1);
    return ();
}


// The main function. Reads the number of usages for each builtin from the input,
// and calls each builtin accordingly.
func main{
    output_ptr: felt*,
    pedersen_ptr: HashBuiltin*,
    range_check_ptr: felt*,
    ecdsa_ptr: SignatureBuiltin*,
    bitwise_ptr: BitwiseBuiltin*,
    ec_op_ptr: EcOpBuiltin*,
    keccak_ptr: KeccakBuiltin*,
    poseidon_ptr: PoseidonBuiltin*,
    range_check96_ptr: felt*,
    add_mod_ptr: ModBuiltin*,
    mul_mod_ptr: ModBuiltin*,
}() {
    alloc_locals;
    local n_output = 1000000;
    // Get number of calls to each builtin from the input and assign it to the local variables.


    local output_start: felt* = output_ptr;

    // Call output builtin.
    do_output(n_builtin_usages=n_output);

    return ();
}
