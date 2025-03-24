%builtins pedersen

from starkware.cairo.common.alloc import alloc
from starkware.cairo.common.cairo_builtins import HashBuiltin


func do_pedersen{pedersen_ptr: HashBuiltin*}(n_builtin_usages: felt) {
    if (n_builtin_usages == 0) {
        return ();
    }

    assert pedersen_ptr.x = 2687178544003432653678040075961940334401287682740059621576273116296938255681;
    assert pedersen_ptr.y = 2796488826891666520311933296728647689545053236330041407839701137298719673254;
    let expect_res = 416500234377612030369920205736914591480298524620437441093603534871773441085;
    assert expect_res = pedersen_ptr.result;
    let pedersen_ptr = pedersen_ptr + HashBuiltin.SIZE;

    assert pedersen_ptr.x = 2000;
    assert pedersen_ptr.y = 3000;
    let expect_res = 2259827999605678368300255972310867999576917457292275299312258452858451849126;
    assert expect_res = pedersen_ptr.result;
    let pedersen_ptr = pedersen_ptr + HashBuiltin.SIZE;

    do_pedersen(n_builtin_usages=n_builtin_usages - 1);
    return ();
}

// The main function. Reads the number of usages for each builtin from the input,
// and calls each builtin accordingly.
func main{
    pedersen_ptr: HashBuiltin*,
}() {
    alloc_locals;
    local n_pedersen = 50;

    // Call pedersen builtin.
    do_pedersen(n_builtin_usages=n_pedersen);

    return ();
}
