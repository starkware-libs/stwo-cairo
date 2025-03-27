%builtins mul_mod

from starkware.cairo.common.alloc import alloc
from starkware.cairo.common.cairo_builtins import (
    ModBuiltin,
    UInt384,
)
from starkware.cairo.common.modulo import BATCH_SIZE
from starkware.cairo.common.registers import get_label_location

func do_mul_mod{mul_mod_ptr: ModBuiltin*}(n_builtin_usages: felt) {
    if (n_builtin_usages == 0) {
        return ();
    }

    let (values_ptr: UInt384*) = alloc();

    assert values_ptr[0] = UInt384(
        d0=0x000000000000000000000007,
        d1=0x000000000000000000000000,
        d2=0x000000000000000000000000,
        d3=0x000000000000000000000000,
    );

    assert values_ptr[1] = UInt384(
        d0=0x000000000000000000000006,
        d1=0x000000000000000000000000,
        d2=0x000000000000000000000000,
        d3=0x000000000000000000000000,
    );

    assert values_ptr[2] = UInt384(
        d0=0x00000000000000000000002a,
        d1=0x000000000000000000000000,
        d2=0x000000000000000000000000,
        d3=0x000000000000000000000000,
    );

    let (mul_mod_offsets_ptr) = get_label_location(mul_offsets);

    // Apply the mul_mod builtin.
    assert mul_mod_ptr[0] = ModBuiltin(
        p=UInt384(
            d0=0xffffffff,
            d1=0xfffffffffffffffeffffffff,
            d2=0xffffffffffffffffffffffff,
            d3=0xffffffffffffffffffffffff,
        ),
        values_ptr=values_ptr,
        offsets_ptr=mul_mod_offsets_ptr,
        n=BATCH_SIZE,
    );

    let mul_mod_ptr = mul_mod_ptr + ModBuiltin.SIZE;

    do_mul_mod(n_builtin_usages=n_builtin_usages - 1);
    return ();

    mul_offsets:
    dw 0;
    dw 4;
    dw 8;
}

// The main function. Reads the number of usages for each builtin from the input,
// and calls each builtin accordingly.
func main{
    mul_mod_ptr: ModBuiltin*,
}() {
    alloc_locals;
    local n_mul_mod = 50;

    // Call mul_mod builtin.
    do_mul_mod(n_builtin_usages=n_mul_mod);

    return ();
}
