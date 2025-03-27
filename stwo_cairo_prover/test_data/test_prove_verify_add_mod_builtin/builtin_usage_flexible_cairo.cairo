%builtins output pedersen range_check ecdsa bitwise ec_op keccak poseidon range_check96 add_mod mul_mod


from starkware.cairo.common.alloc import alloc
from starkware.cairo.common.cairo_builtins import (
    HashBuiltin,
    ModBuiltin,
    PoseidonBuiltin,
    UInt384,
)
from starkware.cairo.common.modulo import BATCH_SIZE
from starkware.cairo.common.registers import get_label_location

func do_add_mod{add_mod_ptr: ModBuiltin*}(n_builtin_usages: felt) {
    if (n_builtin_usages == 0) {
        return ();
    }

    let (values_ptr: UInt384*) = alloc();

    assert values_ptr[0] = UInt384(
        d0=0x000000000000000000000006,
        d1=0x000000000000000000000000,
        d2=0x000000000000000000000000,
        d3=0x000000000000000000000000,
    );

    assert values_ptr[1] = UInt384(
        d0=0x000000000000000000000007,
        d1=0x000000000000000000000000,
        d2=0x000000000000000000000000,
        d3=0x000000000000000000000000,
    );

    assert values_ptr[2] = UInt384(
        d0=0x00000000000000000000000d,
        d1=0x000000000000000000000000,
        d2=0x000000000000000000000000,
        d3=0x000000000000000000000000,
    );

    let (add_mod_offsets_ptr) = get_label_location(add_offsets);

    // Apply the add_mod builtin.
    assert add_mod_ptr[0] = ModBuiltin(
        p=UInt384(
            d0=0xffffffff,
            d1=0xfffffffffffffffeffffffff,
            d2=0xffffffffffffffffffffffff,
            d3=0xffffffffffffffffffffffff,
        ),
        values_ptr=values_ptr,
        offsets_ptr=add_mod_offsets_ptr,
        n=BATCH_SIZE,
    );
    let add_mod_ptr = add_mod_ptr + ModBuiltin.SIZE;

    do_add_mod(n_builtin_usages=n_builtin_usages - 1);
    return ();

    add_offsets:
    dw 0;
    dw 4;
    dw 8;
}

// The main function. Reads the number of usages for each builtin from the input,
// and calls each builtin accordingly.
func main{
    output_ptr: felt*,
    pedersen_ptr: HashBuiltin*,
    range_check_ptr,
    ecdsa_ptr,
    bitwise_ptr,
    ec_op_ptr,
    keccak_ptr,
    poseidon_ptr: PoseidonBuiltin*,
    range_check96_ptr,
    add_mod_ptr : ModBuiltin*,
    mul_mod_ptr,
}() {
    alloc_locals;
    local n_add_mod = 50;

    // Call add_mod builtin.
    do_add_mod(n_builtin_usages=n_add_mod);

    return ();
}
