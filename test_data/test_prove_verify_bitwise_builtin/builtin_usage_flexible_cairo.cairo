%builtins output pedersen range_check ecdsa bitwise ec_op keccak poseidon range_check96 add_mod mul_mod

from starkware.cairo.common.alloc import alloc
from starkware.cairo.common.cairo_builtins import (
    BitwiseBuiltin,
)

func do_bitwise{bitwise_ptr: BitwiseBuiltin*}(n_builtin_usages: felt) {
    if (n_builtin_usages == 0) {
        return ();
    }

    assert bitwise_ptr.x = 3;  // Binary representation 0b011.
    assert bitwise_ptr.y = 6;  // Binary representation 0b110.
    // assert 0x2 = bitwise_ptr.x_and_y;  // Calculate 0b011 & 0b110 = 0b010 = 0x2.
    assert 0x5 = bitwise_ptr.x_xor_y;  // Calculate 0b011 ^ 0b110 = 0b101 = 0x5.
    assert 0x7 = bitwise_ptr.x_or_y;  // Calculate 0b011 & 0b110 = 0b111 = 0x7.
    let bitwise_ptr = bitwise_ptr + BitwiseBuiltin.SIZE;

    do_bitwise(n_builtin_usages=n_builtin_usages - 1);
    return ();
}

// The main function. Reads the number of usages for each builtin from the input,
// and calls each builtin accordingly.
func main{
    output_ptr,
    pedersen_ptr,
    range_check_ptr,
    ecdsa_ptr,
    bitwise_ptr: BitwiseBuiltin*,
    ec_op_ptr,
    keccak_ptr,
    poseidon_ptr,
    range_check96_ptr,
    add_mod_ptr,
    mul_mod_ptr,
}() {
    alloc_locals;
    local n_bitwise = 50;

    // Call bitwise builtin.
    do_bitwise(n_builtin_usages=n_bitwise);

    return ();
}
