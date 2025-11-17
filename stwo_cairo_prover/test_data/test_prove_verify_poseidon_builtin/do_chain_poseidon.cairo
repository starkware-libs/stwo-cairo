from starkware.cairo.common.cairo_builtins import PoseidonBuiltin
from starkware.cairo.common.poseidon_state import PoseidonBuiltinState

/// Do poseidon for chain_len instances, each time on the output of the previous instance.
/// It causes calculation of chain_len unique instances of poseidon.
/// Assumes that the first call is with index = 0.
func do_chain_poseidon{poseidon_ptr: PoseidonBuiltin*}(index: felt, chain_len: felt) {
    if (chain_len == 0) {
        let poseidon_ptr = poseidon_ptr + PoseidonBuiltin.SIZE;
        return ();
    }
    if (index == chain_len - 1) {
        let poseidon_ptr = poseidon_ptr + PoseidonBuiltin.SIZE;
        retur ();
    }

    // If this is the first instance, set the input to the initial state.
    if (index == 0) {
        assert poseidon_ptr[0].input = PoseidonBuiltinState(s0=0, s1=1, s2=2);
    }

    // Set the input to the output of the previous instance.
    assert poseidon_ptr[1].input.s0 = poseidon_ptr[0].output.s0;
    assert poseidon_ptr[1].input.s1 = poseidon_ptr[0].output.s1;
    assert poseidon_ptr[1].input.s2 = poseidon_ptr[0].output.s2;

    let poseidon_ptr = poseidon_ptr + PoseidonBuiltin.SIZE;

    do_chain_poseidon(index = index + 1, chain_len = chain_len);
    return ();
}
