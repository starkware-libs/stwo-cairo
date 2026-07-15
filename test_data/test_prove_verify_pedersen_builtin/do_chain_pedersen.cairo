from starkware.cairo.common.cairo_builtins import HashBuiltin
from starkware.cairo.common.bool import TRUE, FALSE

/// Do pedersen for chain_len instances, each time on the output of the previous instance.
/// It causes calculation of chain_len unique instances of pedersen.
/// Assumes that only the first call is with initialize = True.
func do_chain_pedersen{pedersen_ptr: HashBuiltin*}(initialize: felt, chain_len: felt) {
    if (chain_len == 0) {
        return ();
    }

    // If this is the first instance, set the input to the initial state.
    if (initialize == TRUE) {
        assert pedersen_ptr[0].x = 2687178544003432653678040075961940334401287682740059621576273116296938255681;
        assert pedersen_ptr[0].y = 2796488826891666520311933296728647689545053236330041407839701137298719673254;
        let expect_res = 416500234377612030369920205736914591480298524620437441093603534871773441085;
        assert expect_res = pedersen_ptr.result;
    } else {
        // Set the input to the output of the previous instance.
        assert pedersen_ptr[0].x = pedersen_ptr[-1].y;
        assert pedersen_ptr[0].y = pedersen_ptr[-1].result;
    }

    let pedersen_ptr = pedersen_ptr + HashBuiltin.SIZE;
    do_chain_pedersen(initialize = FALSE, chain_len = chain_len - 1);
    return ();
}
