%builtins range_check96

from starkware.cairo.common.alloc import alloc

func do_range_check96{range_check96_ptr: felt*}(n_builtin_usages: felt) {
    if (n_builtin_usages == 0) {
        return ();
    }

    // Check that 0 <= n_builtin_usages < 2**96.
    assert [range_check96_ptr] = n_builtin_usages;
    let range_check96_ptr = range_check96_ptr + 1;

    do_range_check96(n_builtin_usages=n_builtin_usages - 1);
    return ();
}

// The main function. Reads the number of usages for each builtin from the input,
// and calls each builtin accordingly.
func main{
    range_check96_ptr: felt*,
}() {
    alloc_locals;
    local n_range_check96 = 50;

    // Call range_check96 builtin.
    do_range_check96(n_builtin_usages=n_range_check96);

    return ();
}
