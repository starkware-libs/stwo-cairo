%builtins output pedersen range_check ecdsa bitwise ec_op keccak poseidon range_check96 add_mod mul_mod

from starkware.cairo.common.alloc import alloc
from starkware.cairo.common.cairo_builtins import PoseidonBuiltin
from starkware.cairo.common.poseidon_state import PoseidonBuiltinState


func do_poseidon{poseidon_ptr: PoseidonBuiltin*}(n_builtin_usages: felt) {
    if (n_builtin_usages == 0) {
        return ();
    }
    // Instance 1
    assert poseidon_ptr[0].input = PoseidonBuiltinState(s0=0, s1=1, s2=2);
    assert poseidon_ptr[0].output = PoseidonBuiltinState(
        s0=0x5134197931125e849424475aa20cd6ca0ce8603b79177c3f76e2119c8f98c53,
        s1=0x30b51bb39c4e74544fc2576ac2a3cf44485ad135802c6ac1246659ad34f241f,
        s2=0x3241fe256bea8c2e2fa69098127e17e4020dc42158e61fd3e6dc236e0c0cac,
    );

    // Instance 2
    assert poseidon_ptr[1].input = PoseidonBuiltinState(s0=3, s1=4, s2=5);
    assert poseidon_ptr[1].output = PoseidonBuiltinState(
        s0=0x61025455C656D4F817B662069B67EBC472DF3C2BA6EA0CC322C2234A73ECC96,
        s1=0x3AE8A3AD4C9B6B74F3FD183BFBEBC3C45B959D6C43F957BDE3411DB7FCDB430,
        s2=0x245E86518EBCFEBE03E35D04A30BFA96DB6B291CFD1D93B4E41527C44F6453E,
    );

    // Instance 3
    assert poseidon_ptr[2].input = PoseidonBuiltinState(s0=6, s1=7, s2=8);
    assert poseidon_ptr[2].output = PoseidonBuiltinState(
        s0=0x195872647a9fbd55dded9d1802e33344ade7898cf26af098aa88aef27d73a7f,
        s1=0xe19339ab2130866421017cd1af8459581854615703d1be292c3c5079868cb,
        s2=0x455873bb0f768bc7ec85fe5750bda71a99dc28143539af7175c24c8ac185315,
    );

    // Instance 4
    assert poseidon_ptr[3].input = PoseidonBuiltinState(s0=9, s1=10, s2=11);
    assert poseidon_ptr[3].output = PoseidonBuiltinState(
        s0=0x40b8d76b2df6424898a9c0c9160485a2d8a39f3e415d85c4b22dbf93819fbfd,
        s1=0x750d380551c0488e6532edd99bb6f9c328b72af726b4d88e9f239b790e1f33a,
        s2=0x9e7a32f633ca2fc5c4d7423b41ae9e73ac42e69461d792e617719a32bfa1ec,
    );

    // Instance 5
    assert poseidon_ptr[4].input = PoseidonBuiltinState(s0=12, s1=13, s2=14);
    assert poseidon_ptr[4].output = PoseidonBuiltinState(
        s0=0x4c9587c972c20fc7111ee588f39dbd2fbc85c76e51e6c45b39677890ea85b9e,
        s1=0x20e211722df2b08232fe610f692e327a2222110a21edd53fae3e31314dccb23,
        s2=0x39a6a63327f30bf10ea1c78848bcd76c2133af7c5112444809ad9ee19a123db,
    );

    // Instance 6
    assert poseidon_ptr[5].input = PoseidonBuiltinState(s0=15, s1=16, s2=17);
    assert poseidon_ptr[5].output = PoseidonBuiltinState(
        s0=0x64c97848746b52f70d659cca988f9748e3a8ca6aead6f3110ecfd1379638a77,
        s1=0x7fc781c41c8404324fa5cb35196cc4fc387e3d52c71051a2bd89a6856f0901,
        s2=0x1fc8eb6a3b9f69f22d916719be0f1ca6077f7b5a531f50e8eafb000e0e6fbf8,
    );

    // Instance 7
    assert poseidon_ptr[6].input = PoseidonBuiltinState(s0=18, s1=19, s2=20);
    assert poseidon_ptr[6].output = PoseidonBuiltinState(
        s0=0x1165f523aab7c06e448bd41a86ecdc6379f87dd7630792114198aac92f0c2a6,
        s1=0x1542f463e21dabc6af427e5251ac733eeb1904a6b3602f4ea85d8684a9b4ec2,
        s2=0x39ca8d7baee9e2eeb8b265ad12790918b989ee29202f189bec8184ddfeebddc,
    );

    // Instance 8
    assert poseidon_ptr[7].input = PoseidonBuiltinState(s0=21, s1=22, s2=23);
    assert poseidon_ptr[7].output = PoseidonBuiltinState(
        s0=0x689bc531a1f318e5e52ba04f9db72094f98e3e040594f1bb7f8aa806832b43c,
        s1=0x6949a0ae95988432ded0f1578ae1eb634efdce53aad7a333d1d14ed3ccabebd,
        s2=0xd0969f13d005e5ba4da179310eb9949c8996265395b467bcdfb118d934d380,
    );

    // Instance 9
    assert poseidon_ptr[8].input = PoseidonBuiltinState(s0=24, s1=25, s2=26);
    assert poseidon_ptr[8].output = PoseidonBuiltinState(
        s0=0x2a40582431920c144a720f2c384fc546d919b3a96199857d5ffff3a8b347195,
        s1=0x64e4ff3371aff7bb403b357a05b66150e43fc6d78c88b5f036826536102a7f5,
        s2=0x4bb4e340c86a75ad5844f5a6a851a7e2789e98326aa1ccb418560e3da724073,
    );

    // Instance 10
    assert poseidon_ptr[9].input = PoseidonBuiltinState(s0=27, s1=28, s2=29);
    assert poseidon_ptr[9].output = PoseidonBuiltinState(
        s0=0x50bc8aab9a3fd3138b63e08b093a322f75833a099a84f4e6df97f14b0f2cb41,
        s1=0x7c20bf9c3efdf53ea67c2d52b6f007de12e055301e23933e9339e3c3118a775,
        s2=0x78e36374703b184ff95836eae6e46d002670b17a677ecf64cd4363ed662f82d,
    );

    // Instance 11
    assert poseidon_ptr[10].input = PoseidonBuiltinState(s0=30, s1=31, s2=32);
    assert poseidon_ptr[10].output = PoseidonBuiltinState(
        s0=0x2626b2ac93ebf47543b476797638772681b7607ff659c621e5560e111902974,
        s1=0x5331C7481CF824668F75E461E95191E9C8D014BB29EBF532B5FB0B7E29C3A22,
        s2=0x44FD2CAE666648675140D732E22428C4A8CEC13DCCE45EBCEDFFCAD824861E0,
    );

    // Instance 12
    assert poseidon_ptr[11].input = PoseidonBuiltinState(s0=33, s1=34, s2=35);
    assert poseidon_ptr[11].output = PoseidonBuiltinState(
        s0=0x45BD551EB3BD05396E3E47E2B25258B8F0D2D89918C456D1711B70BC07DB3C1,
        s1=0x5F238B95099AF0FCAE6EF11EFC7A8B189F1540006133253B4B7DB7B105923A7,
        s2=0x47D61811B97A561C69B3A5BE6D54A42FA887D85976C4DB684D06E803D68BB38,
    );

    // Instance 13
    assert poseidon_ptr[12].input = PoseidonBuiltinState(s0=36, s1=37, s2=38);
    assert poseidon_ptr[12].output = PoseidonBuiltinState(
        s0=0x129D4A2887CCE7EE01F2A65553401B77CBC572F32981903B32E0125F29F4A,
        s1=0x2879B0EAD85118E75BC88496E48567E9916D85C08223488C0226A2848C0FC78,
        s2=0x5EF2DED1068D20E6A4EF2D7F3A2B22DE483CA7268A28C5EB3B1D8F6FC9357E4,
    );

    // Instance 14
    assert poseidon_ptr[13].input = PoseidonBuiltinState(s0=39, s1=40, s2=41);
    assert poseidon_ptr[13].output = PoseidonBuiltinState(
        s0=0x3FA58A9B6095E3DF1F397CF6245CDB72DEF4259DCE93D44462079B600D07200,
        s1=0x29AEC75DECD07751667C744DD23A6795C1DFCF3CE59CD71861127F40DFADA65,
        s2=0x71C556672799EF9A999CAF985D3B247B000709D19CE15330CE48DB22F6417E7,
    );

    // Instance 15
    assert poseidon_ptr[14].input = PoseidonBuiltinState(s0=42, s1=43, s2=44);
    assert poseidon_ptr[14].output = PoseidonBuiltinState(
        s0=0x60F86EED3E6C51E8DA3381C9E080F6B1818E9EA8F4BB0736149187FE6E4FD2D,
        s1=0x3114D2BCF9B2AF4D7EA16995D92DF2887592F1C31869985D82F5CDD241C4C1E,
        s2=0x4DF69A871653BD347F09302C317043EB3EC2CA1B9E947B3B0DD50EB40CB1ACB,
    );

    // Instance 16
    assert poseidon_ptr[15].input = PoseidonBuiltinState(s0=45, s1=46, s2=47);
    assert poseidon_ptr[15].output = PoseidonBuiltinState(
        s0=0x7F8E6C0F1C75DAF638A0A72B874D91B4420D73DD7599093C36048DD57206664,
        s1=0x435B83D95E5E5C818ADD2389ECD47CBD56221884F66D992A2175D3E87AF5C36,
        s2=0x55FB0DC56684E462CF4C1AC5B1B05D18A26D04D265381C795A7B252356D1AE1,
    );

    let poseidon_ptr = poseidon_ptr + 16 * PoseidonBuiltin.SIZE;

    do_poseidon(n_builtin_usages=n_builtin_usages - 1);
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
    poseidon_ptr: PoseidonBuiltin*,
    range_check96_ptr,
    add_mod_ptr,
    mul_mod_ptr,
}() {
    alloc_locals;
    local n_poseidon = 50;

    // Call poseidon builtin.
    do_poseidon(n_builtin_usages=n_poseidon);

    return ();
}
