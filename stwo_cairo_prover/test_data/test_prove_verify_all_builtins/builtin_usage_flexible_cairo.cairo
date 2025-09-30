%builtins output pedersen range_check ecdsa bitwise ec_op keccak poseidon range_check96 add_mod mul_mod

from starkware.cairo.common.alloc import alloc
from starkware.cairo.common.cairo_builtins import (
    BitwiseBuiltin,
    EcOpBuiltin,
    HashBuiltin,
    KeccakBuiltin,
    ModBuiltin,
    PoseidonBuiltin,
    SignatureBuiltin,
    UInt384,
)
from starkware.cairo.common.keccak_state import KeccakBuiltinState
from starkware.cairo.common.modulo import BATCH_SIZE
from starkware.cairo.common.poseidon_state import PoseidonBuiltinState
from starkware.cairo.common.registers import get_label_location

func do_output{output_ptr: felt*}(n_builtin_usages: felt) {
    if (n_builtin_usages == 0) {
        return ();
    }

    assert [output_ptr] = 1000 + n_builtin_usages;
    let output_ptr = output_ptr + 1;

    do_output(n_builtin_usages=n_builtin_usages - 1);
    return ();
}

func do_pedersen{pedersen_ptr: HashBuiltin*}(n_builtin_usages: felt) {
    if (n_builtin_usages == 0) {
        return ();
    }

    assert pedersen_ptr.x = 2000;
    assert pedersen_ptr.y = 3000;
    let expect_res = 2259827999605678368300255972310867999576917457292275299312258452858451849126;
    assert expect_res = pedersen_ptr.result;
    let pedersen_ptr = pedersen_ptr + HashBuiltin.SIZE;

    do_pedersen(n_builtin_usages=n_builtin_usages - 1);
    return ();
}

func do_range_check{range_check_ptr: felt*}(n_builtin_usages: felt) {
    if (n_builtin_usages == 0) {
        return ();
    }

    // Check that 0 <= n_builtin_usages < 2**128.
    assert [range_check_ptr] = n_builtin_usages;
    let range_check_ptr = range_check_ptr + 1;

    do_range_check(n_builtin_usages=n_builtin_usages - 1);
    return ();
}

func do_ecdsa{ecdsa_ptr: SignatureBuiltin*}(n_builtin_usages: felt) {
    if (n_builtin_usages == 0) {
        return ();
    }

    tempvar signature_r = 3086480810278599376317923499561306189851900463386393948998357832163236918254;
    tempvar signature_s = 598673427589502599949712887611119751108407514580626464031881322743364689811;
    %{
        ecdsa_builtin.add_signature(ids.ecdsa_ptr.address_, (ids.signature_r, ids.signature_s))
    %}
    tempvar exp_key = 1735102664668487605176656616876767369909409133946409161569774794110049207117;
    assert exp_key = ecdsa_ptr.pub_key;
    assert ecdsa_ptr.message = 2718;
    let ecdsa_ptr = ecdsa_ptr + SignatureBuiltin.SIZE;

    do_ecdsa(n_builtin_usages=n_builtin_usages - 1);
    return ();
}

func do_bitwise{bitwise_ptr: BitwiseBuiltin*}(n_builtin_usages: felt) {
    if (n_builtin_usages == 0) {
        return ();
    }

    assert bitwise_ptr.x = 3;  // Binary representation 0b011.
    assert bitwise_ptr.y = 6;  // Binary representation 0b110.
    // assert 0x2 = bitwise_ptr.x_and_y;  // Calculate 0b011 & 0b110 = 0b010 = 0x2.
    assert 0x5 = bitwise_ptr.x_xor_y;  // Calculate 0b011 ^ 0b110 = 0b101 = 0x5.
    assert 0x7 = bitwise_ptr.x_or_y;  // Calculate 0b011 & 0b110 = 0b111 = 0x7.
    let bitwise_ptr = bitwise_ptr + BitwiseBuiltin.SIZE;

    do_bitwise(n_builtin_usages=n_builtin_usages - 1);
    return ();
}

func do_ec_op{ec_op_ptr: EcOpBuiltin*}(n_builtin_usages: felt) {
    if (n_builtin_usages == 0) {
        return ();
    }

    assert ec_op_ptr.p.x = 0x49ee3eba8c1600700ee1b87eb599f16716b0b1022947733551fde4050ca6804;
    assert ec_op_ptr.p.y = 0x3ca0cfe4b3bc6ddf346d49d06ea0ed34e621062c0e056c1d0405d266e10268a;
    assert ec_op_ptr.q.x = 0x1ef15c18599971b7beced415a40f0c7deacfd9b0d1819e03d723d8bc943cfca;
    assert ec_op_ptr.q.y = 0x5668060aa49730b7be4801df46ec62de53ecd11abe43a32873000c36e8dc1f;
    assert ec_op_ptr.m = 3;
    assert 0x7e7981dbdcab7a12e82a71563265fe17d1e468def04dc824c342bd113b8a6ba = ec_op_ptr.r.x;
    assert 0x74af28209b54a0943e10972953ae3acc93ca2d74caf5b07c0a833fbb9aba0ff = ec_op_ptr.r.y;
    let ec_op_ptr = ec_op_ptr + EcOpBuiltin.SIZE;

    do_ec_op(n_builtin_usages=n_builtin_usages - 1);
    return ();
}

func do_keccak{keccak_ptr: KeccakBuiltin*}(n_builtin_usages: felt) {
    if (n_builtin_usages == 0) {
        return ();
    }

    assert keccak_ptr.input = KeccakBuiltinState(s0=0, s1=1, s2=2, s3=3, s4=4, s5=5, s6=6, s7=7);
    tempvar keccak_output = keccak_ptr.output;
    assert keccak_output = KeccakBuiltinState(
        s0=0x39d703c98a1b2e1a2ddf0c93810df2d39b6dfecdee6832188d,
        s1=0x541c4683d434a407a3525e2f20fa9431b65cd58e995379146d,
        s2=0x66f2b6f9585469eef0f16447a1bc76adc5f3b602a698dfdc42,
        s3=0x16f13d5794d8770f73a01aa7e00accde43f4fa6a208a7f03a5,
        s4=0xfdf1ac3b6b45fdeee26ff23d7a5318a94dabb4efbba7ad35a1,
        s5=0x639b68738d3ebd70e1181f43ccfbfc0e5ba26fb99251069ae2,
        s6=0x50c5875966fe759e96419d03d1ff8c66e868d68a052651260d,
        s7=0x51611748d0540c05bd45cd46cdb6cdcdce7402d755893da7e0,
    );
    let keccak_ptr = keccak_ptr + KeccakBuiltin.SIZE;

    do_keccak(n_builtin_usages=n_builtin_usages - 1);
    return ();
}

func do_poseidon{poseidon_ptr: PoseidonBuiltin*}(n_builtin_usages: felt) {
    if (n_builtin_usages == 0) {
        return ();
    }

    assert poseidon_ptr.input = PoseidonBuiltinState(s0=0, s1=1, s2=2);
    assert poseidon_ptr.output = PoseidonBuiltinState(
        s0=0x5134197931125e849424475aa20cd6ca0ce8603b79177c3f76e2119c8f98c53,
        s1=0x30b51bb39c4e74544fc2576ac2a3cf44485ad135802c6ac1246659ad34f241f,
        s2=0x3241fe256bea8c2e2fa69098127e17e4020dc42158e61fd3e6dc236e0c0cac,
    );
    let poseidon_ptr = poseidon_ptr + PoseidonBuiltin.SIZE;

    do_poseidon(n_builtin_usages=n_builtin_usages - 1);
    return ();
}

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

func do_add_mod{add_mod_ptr: ModBuiltin*}(n_builtin_usages: felt) {
    if (n_builtin_usages == 0) {
        return ();
    }

    let (values_ptr: UInt384*) = alloc();

    assert values_ptr[0] = UInt384(
        d0=0x000000000000000000000006,
        d1=0x000000000000000000000000,
        d2=0x000000000000000000000000,
        d3=0x000000000000000000000000,
    );

    assert values_ptr[1] = UInt384(
        d0=0x000000000000000000000007,
        d1=0x000000000000000000000000,
        d2=0x000000000000000000000000,
        d3=0x000000000000000000000000,
    );

    assert values_ptr[2] = UInt384(
        d0=0x00000000000000000000000d,
        d1=0x000000000000000000000000,
        d2=0x000000000000000000000000,
        d3=0x000000000000000000000000,
    );

    let (add_mod_offsets_ptr) = get_label_location(add_offsets);

    // Apply the add_mod builtin.
    assert add_mod_ptr[0] = ModBuiltin(
        p=UInt384(
            d0=0xffffffff,
            d1=0xfffffffffffffffeffffffff,
            d2=0xffffffffffffffffffffffff,
            d3=0xffffffffffffffffffffffff,
        ),
        values_ptr=values_ptr,
        offsets_ptr=add_mod_offsets_ptr,
        n=BATCH_SIZE,
    );
    let add_mod_ptr = add_mod_ptr + ModBuiltin.SIZE;

    do_add_mod(n_builtin_usages=n_builtin_usages - 1);
    return ();

    add_offsets:
    dw 0;
    dw 4;
    dw 8;
}

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
    output_ptr: felt*,
    pedersen_ptr: HashBuiltin*,
    range_check_ptr: felt*,
    ecdsa_ptr: SignatureBuiltin*,
    bitwise_ptr: BitwiseBuiltin*,
    ec_op_ptr: EcOpBuiltin*,
    keccak_ptr: KeccakBuiltin*,
    poseidon_ptr: PoseidonBuiltin*,
    range_check96_ptr: felt*,
    add_mod_ptr: ModBuiltin*,
    mul_mod_ptr: ModBuiltin*,
}() {
    alloc_locals;
    local n_output = 50;
    local n_pedersen = 50;
    local n_range_check = 50;
    local n_ecdsa = 0;
    local n_bitwise = 50;
    local n_ec_op = 0;
    local n_keccak = 0;
    local n_poseidon = 50;
    local n_range_check96 = 50;
    local n_add_mod = 50;
    local n_mul_mod = 50;
    local n_memory_holes = 50;

    // Call output builtin.
    do_output(n_builtin_usages=n_output);

    // Call pedersen builtin.
    do_pedersen(n_builtin_usages=n_pedersen);

    // Call range_check builtin.
    do_range_check(n_builtin_usages=n_range_check);

    // Call ecdsa builtin.
    do_ecdsa(n_builtin_usages=n_ecdsa);

    // Call bitwise builtin.
    do_bitwise(n_builtin_usages=n_bitwise);

    // Call ec_op builtin.
    do_ec_op(n_builtin_usages=n_ec_op);

    // Call keccak builtin.
    do_keccak(n_builtin_usages=n_keccak);

    // Call poseidon builtin.
    do_poseidon(n_builtin_usages=n_poseidon);

    // Call range_check96 builtin.
    do_range_check96(n_builtin_usages=n_range_check96);

    // Call add_mod builtin.
    do_add_mod(n_builtin_usages=n_add_mod);

    // Call mul_mod builtin.
    do_mul_mod(n_builtin_usages=n_mul_mod);

    // Create memory holes.
    [ap] = 1, ap++;
    ap += n_memory_holes;
    [ap] = 1, ap++;

    return ();
}
