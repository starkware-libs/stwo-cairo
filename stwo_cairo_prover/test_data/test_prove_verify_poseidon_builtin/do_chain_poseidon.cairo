from starkware.cairo.common.cairo_builtins import PoseidonBuiltin
from starkware.cairo.common.poseidon_state import PoseidonBuiltinState

/// Do poseidon for chain_len instances, each time on the output of the previous instance. It causes calculation of chain_len unique instances of poseidon.
/// This function assumes that the caller has already set the input for poseidon_ptr[0].
func do_chain_poseidon{poseidon_ptr: PoseidonBuiltin*}(chain_len: felt) {
    if (chain_len == 1) {
        let poseidon_ptr = poseidon_ptr + PoseidonBuiltin.SIZE;
        return ();
    }
    assert poseidon_ptr[1].input.s0 = poseidon_ptr[0].output.s0;
    assert poseidon_ptr[1].input.s1 = poseidon_ptr[0].output.s1;
    assert poseidon_ptr[1].input.s2 = poseidon_ptr[0].output.s2;
    let poseidon_ptr = poseidon_ptr + PoseidonBuiltin.SIZE;
    do_chain_poseidon(chain_len = chain_len - 1);
    return ();
}
