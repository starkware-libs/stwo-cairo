%builtins output pedersen range_check ecdsa bitwise ec_op keccak poseidon range_check96 add_mod mul_mod

from starkware.cairo.common.alloc import alloc
from starkware.cairo.common.cairo_builtins import (
    ModBuiltin,
    UInt384,
)
from starkware.cairo.common.registers import get_label_location

// Note that this function implicitly assumes that BATCH_SIZE is 1.
func do_mul_mod{mul_mod_ptr: ModBuiltin*}(n_builtin_usages: felt) {
    if (n_builtin_usages == 0) {
        return ();
    }

    let p1 = UInt384(
        d0=50194602145281500560007247715,
        d1=76868189491771433761047631975,
        d2=76613164022382921729294390885,
        d3=46298721445880844022721490358,
    );

    let (values_ptr1: UInt384*) = alloc();

    assert values_ptr1[0] = UInt384(
        d0=75281414126059212790394219922,
        d1=60700966719373522919650757631,
        d2=43425698583645282082404301051,
        d3=42294050191407740231374671983,
    );

    assert values_ptr1[1] = UInt384(
        d0=34336884712738565810077902554,
        d1=37998742198487431679639436906,
        d2=53500273492292755219177107288,
        d3=42580083475966415457160365629,
    );

    assert values_ptr1[2] = UInt384(
        d0=6503285614362226567910515565,
        d1=42938128015207989754410481467,
        d2=47591058973097636376448903827,
        d3=4802857463010953968823214097,
    );

    assert values_ptr1[3] = UInt384(
        d0=23860321595689079427193147251,
        d1=4805495049297682669916770071,
        d2=22115680452665293227178802480,
        d3=30591144237717258677916270335,
    );

    assert values_ptr1[4] = UInt384(
        d0=21595802556169574972448102645,
        d1=35454383782553720163730347286,
        d2=70037672172643121947901803312,
        d3=5661799930819731137956045877,
    );

    assert values_ptr1[5] = UInt384(
        d0=16766459322852676516904855393,
        d1=24608253471469857381448040761,
        d2=56332221317264894720870780495,
        d3=41256838211496216491559769271,
    );

    let p2 = UInt384(
        d0=47215045371565379164322085426,
        d1=44041118390009378566376440305,
        d2=28253136013572507312084647372,
        d3=27591888424968618459251648517,
    );

    let (values_ptr2: UInt384*) = alloc();

    assert values_ptr2[0] = UInt384(
        d0=69926110382496690563359520275,
        d1=9308974168209872844197265502,
        d2=64424777588392514699752119546,
        d3=5394210003713718564961882557,
    );

    assert values_ptr2[1] = UInt384(
        d0=2845066666780114342938595450,
        d1=29780086105520932566160438017,
        d2=75213321032037721471593296313,
        d3=6611815142597078571092015255,
    );

    assert values_ptr2[2] = UInt384(
        d0=3661423089439934978425391126,
        d1=74210556980200465542649174620,
        d2=20874948564311674749699755538,
        d3=30694671709320083130870638516,
    );

    assert values_ptr2[3] = UInt384(
        d0=12227190523074426419090527623,
        d1=46712652064633827086171544646,
        d2=46055537532622719551869569698,
        d3=7385742337844151409533505471,
    );

    assert values_ptr2[4] = UInt384(
        d0=41502782606238570599766817469,
        d1=57894090910609718724726438504,
        d2=4497518638534961828729279062,
        d3=78278135709032233327300018245,
    );

    assert values_ptr2[5] = UInt384(
        d0=2670370366790608019707026465,
        d1=63111696278031769012703421894,
        d2=53813122877425523411516821874,
        d3=36224716698068674133590023427,
    );

    assert values_ptr2[6] = UInt384(
        d0=44390217778450106642788121428,
        d1=41707332170547363733210108810,
        d2=29530285266187886030904079407,
        d3=6804191761749355142818135605,
    );

    assert values_ptr2[7] = UInt384(
        d0=44618248458250066350674949180,
        d1=71517080126292087066788763066,
        d2=29803266097382181021625108294,
        d3=35684685716400274220441588037,
    );

    assert values_ptr2[8] = UInt384(
        d0=71651386031690951198142458052,
        d1=39078816024644767304426349767,
        d2=3265112471770797123113101935,
        d3=31403838889582149564923896506,
    );

    let (mul_mod_offsets_ptr1) = get_label_location(add_offsets1);
    let (mul_mod_offsets_ptr2) = get_label_location(add_offsets2);

    // Apply the mul_mod builtin.
    assert mul_mod_ptr[0] = ModBuiltin(
        p=p1,
        values_ptr=values_ptr1,
        offsets_ptr=mul_mod_offsets_ptr1,
        n=2,
    );
    assert mul_mod_ptr[1] = ModBuiltin(
        p=p1,
        values_ptr=values_ptr1,
        offsets_ptr=mul_mod_offsets_ptr1 + 3,
        n=1,
    );
    assert mul_mod_ptr[2] = ModBuiltin(
        p=p2,
        values_ptr=values_ptr2,
        offsets_ptr=mul_mod_offsets_ptr2,
        n=3,
    );
    assert mul_mod_ptr[3] = ModBuiltin(
        p=p2,
        values_ptr=values_ptr2,
        offsets_ptr=mul_mod_offsets_ptr2 + 3,
        n=2,
    );
    assert mul_mod_ptr[4] = ModBuiltin(
        p=p2,
        values_ptr=values_ptr2,
        offsets_ptr=mul_mod_offsets_ptr2 + 3*2,
        n=1,
    );
    let mul_mod_ptr = mul_mod_ptr + 5 * ModBuiltin.SIZE;

    do_mul_mod(n_builtin_usages=n_builtin_usages - 1);
    return ();

    add_offsets1:
    dw 12;
    dw 16;
    dw 20;
    dw 0;
    dw 4;
    dw 8;

    add_offsets2:
    dw 24;
    dw 28;
    dw 32;
    dw 0;
    dw 4;
    dw 8;
    dw 12;
    dw 16;
    dw 20;
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
    range_check96_ptr,
    add_mod_ptr,
    mul_mod_ptr: ModBuiltin*,
}() {
    alloc_locals;
    local n_mul_mod = 50;

    // Call mul_mod builtin.
    do_mul_mod(n_builtin_usages=n_mul_mod);

    return ();
}
