%builtins output pedersen range_check ecdsa bitwise ec_op keccak poseidon sha256 range_check96 add_mod mul_mod 

from starkware.cairo.common.alloc import alloc
from starkware.cairo.common.cairo_builtins import Sha256Builtin
from starkware.cairo.common.sha256_state import Sha256Input, Sha256State


func do_sha256{sha256_ptr: Sha256Builtin*}(n_builtin_usages: felt) {
    if (n_builtin_usages == 0) {
        return ();
    }

    assert sha256_ptr.input = Sha256Input(s0=1214606444, s1=1870659584, s2=0, s3=0, s4=0, s5=0, s6=0, s7=0, s8=0, s9=0, s10=0, s11=0, s12=0, s13=0, s14=0, s15=40);
    assert sha256_ptr.output = Sha256State(
        s0=2924848972,
        s1=1728728992,
        s2=3102913418,
        s3=3996858604,
        s4=4059601329,
        s5=3017074932,
        s6=3897400477,
        s7=3394718800,
    );
    let sha256_ptr = sha256_ptr + Sha256Builtin.SIZE;

    do_sha256(n_builtin_usages=n_builtin_usages - 1);
    return ();
}

// The main function. Reads the number of usages for each builtin from the input,
// and calls each builtin accordingly.
func main{
    output_ptr,
    pedersen_ptr,
    range_check_ptr,
    ecdsa_ptr,
    bitwise_ptr,
    ec_op_ptr,
    keccak_ptr,
    poseidon_ptr,
    sha256_ptr: Sha256Builtin*,
    range_check96_ptr,
    add_mod_ptr,
    mul_mod_ptr,
}() {
    alloc_locals;
    local n_sha256 = 50;

    do_sha256(n_builtin_usages=n_sha256);


    return ();
}
