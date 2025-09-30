%builtins output pedersen range_check ecdsa bitwise ec_op keccak poseidon range_check96 add_mod mul_mod

from starkware.cairo.common.alloc import alloc
from starkware.cairo.common.cairo_builtins import PoseidonBuiltin
from starkware.cairo.common.poseidon_state import PoseidonBuiltinState


func do_poseidon{poseidon_ptr: PoseidonBuiltin*}(n_builtin_usages: felt) {
    if (n_builtin_usages == 0) {
        return ();
    }

    assert poseidon_ptr.input = PoseidonBuiltinState(s0=0, s1=1, s2=2);
    assert poseidon_ptr.output = PoseidonBuiltinState(
        s0=0x5134197931125e849424475aa20cd6ca0ce8603b79177c3f76e2119c8f98c53,
        s1=0x30b51bb39c4e74544fc2576ac2a3cf44485ad135802c6ac1246659ad34f241f,
        s2=0x3241fe256bea8c2e2fa69098127e17e4020dc42158e61fd3e6dc236e0c0cac,
    );
    let poseidon_ptr = poseidon_ptr + PoseidonBuiltin.SIZE;

    do_poseidon(n_builtin_usages=n_builtin_usages - 1);
    return ();
}

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
    local n_poseidon = 50;

    // Call poseidon builtin.
    do_poseidon(n_builtin_usages=n_poseidon);

    return ();
}
