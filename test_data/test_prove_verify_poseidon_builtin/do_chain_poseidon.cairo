from starkware.cairo.common.cairo_builtins import PoseidonBuiltin
from starkware.cairo.common.poseidon_state import PoseidonBuiltinState
from starkware.cairo.common.bool import TRUE, FALSE

/// Do poseidon for chain_len instances, each time on the output of the previous instance.
/// It causes calculation of chain_len unique instances of poseidon.
/// Assumes that only the first call is with initialize = True.
func do_chain_poseidon{poseidon_ptr: PoseidonBuiltin*}(initialize: felt, chain_len: felt) {
    if (chain_len == 0) {
        return ();
    }

    // If this is the first instance, set the input to the initial state.
    if (initialize == TRUE) {
        assert poseidon_ptr[0].input = PoseidonBuiltinState(s0=0, s1=1, s2=2);
        assert poseidon_ptr.output = PoseidonBuiltinState(
            s0=0x5134197931125e849424475aa20cd6ca0ce8603b79177c3f76e2119c8f98c53,
            s1=0x30b51bb39c4e74544fc2576ac2a3cf44485ad135802c6ac1246659ad34f241f,
            s2=0x3241fe256bea8c2e2fa69098127e17e4020dc42158e61fd3e6dc236e0c0cac,
        );
    } else {
        // Set the input to the output of the previous instance.
        assert poseidon_ptr[0].input.s0 = poseidon_ptr[-1].output.s0;
        assert poseidon_ptr[0].input.s1 = poseidon_ptr[-1].output.s1;
        assert poseidon_ptr[0].input.s2 = poseidon_ptr[-1].output.s2;
    }

    let poseidon_ptr = poseidon_ptr + PoseidonBuiltin.SIZE;
    do_chain_poseidon(initialize = FALSE, chain_len = chain_len - 1);
    return ();
}
