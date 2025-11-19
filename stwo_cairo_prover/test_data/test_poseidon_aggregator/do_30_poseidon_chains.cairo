%builtins output pedersen range_check ecdsa bitwise ec_op keccak poseidon range_check96 add_mod mul_mod

from starkware.cairo.common.cairo_builtins import PoseidonBuiltin
from starkware.cairo.common.poseidon_state import PoseidonBuiltinState
from starkware.cairo.common.bool import TRUE, FALSE
from stwo_cairo_prover.test_data.test_prove_verify_poseidon_builtin.do_chain_poseidon import do_chain_poseidon

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

    // To test the aggregator we want to get identical Poseidon inputs; therefore, we duplicate the same chain 30 times.
    do_chains_poseidon(chain_len = 15, n_chains = 30);
    return ();
}

/// Do n_chains chains of poseidon, each one of chain_len size.
func do_chains_poseidon{poseidon_ptr: PoseidonBuiltin*}(chain_len: felt, n_chains: felt) {
    if (n_chains == 0) {
        return ();
    }

    do_chain_poseidon(initialize = TRUE, chain_len = 15);

    do_chains_poseidon(chain_len = 15, n_chains = n_chains - 1);
    return ();
}
