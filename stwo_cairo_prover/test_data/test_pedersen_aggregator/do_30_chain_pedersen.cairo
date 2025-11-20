%builtins output pedersen range_check ecdsa bitwise ec_op keccak poseidon range_check96 add_mod mul_mod

from starkware.cairo.common.cairo_builtins import HashBuiltin
from stwo_cairo_prover.test_data.test_prove_verify_pedersen_builtin.do_chain_pedersen import do_chain_pedersen
from starkware.cairo.common.bool import TRUE, FALSE

func main{
    output_ptr,
    pedersen_ptr: HashBuiltin*,
    range_check_ptr,
    ecdsa_ptr,
    bitwise_ptr,
    ec_op_ptr,
    keccak_ptr,
    poseidon_ptr,
    range_check96_ptr,
    add_mod_ptr,
    mul_mod_ptr,
}() {

    // To test the aggregator we want to get identical Pedersen inputs; therefore, we duplicate the same chain 30 times.
    do_chains_pedersen(chain_len = 15, n_chains = 30);
    return ();
}

/// Do n_chains chains of pedersen, each one of chain_len size.
func do_chains_pedersen{pedersen_ptr: HashBuiltin*}(chain_len: felt, n_chains: felt) {
    if (n_chains == 0) {
        return ();
    }

    do_chain_pedersen(initialize=TRUE, chain_len = 15);

    do_chains_pedersen(chain_len = 15, n_chains = n_chains - 1);
    return ();
}
