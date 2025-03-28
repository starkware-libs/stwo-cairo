%builtins output pedersen range_check ecdsa bitwise ec_op keccak poseidon range_check96 add_mod mul_mod

from starkware.cairo.common.alloc import alloc
from starkware.cairo.common.cairo_builtins import (
    ModBuiltin,
    UInt384,
)
from starkware.cairo.common.registers import get_label_location

// Note that this function implicitly assumes that BATCH_SIZE is 1.
func do_add_mod{add_mod_ptr: ModBuiltin*}(n_builtin_usages: felt) {
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
        d0=29091876654323145151174848554,
        d1=26007157788849058493269479829,
        d2=38481985774736263908649278847,
        d3=57744057286257910072133617967,
    );

    assert values_ptr1[1] = UInt384(
        d0=30637063040695702493959780639,
        d1=49380517041604201135167969030,
        d2=29641425794583440171879443246,
        d3=19614206973152481265685278126,
    );

    assert values_ptr1[2] = UInt384(
        d0=9534337549737347085127381478,
        d1=77747647852946163460933767220,
        d2=70738410061201119944778281543,
        d3=31059542813529547315097405734,
    );

    assert values_ptr1[3] = UInt384(
        d0=25364755434755755068608619509,
        d1=46375831507992734287877107353,
        d2=51670430756152926393861588720,
        d3=52429320694060955929043194660,
    );

    assert values_ptr1[4] = UInt384(
        d0=78945914572755310267435273962,
        d1=27712413850847538634239209051,
        d2=65428338942922928372768070484,
        d3=28052795324322022483964753387,
    );

    assert values_ptr1[5] = UInt384(
        d0=54116067862229564776036645756,
        d1=76448218381333176754612634765,
        d2=40485605676692933037335268318,
        d3=34183394572502134390286457689,
    );

    assert values_ptr1[6] = UInt384(
        d0=35968495518893490208806073068,
        d1=49102443134823650461092222214,
        d2=35464705345053458822988979448,
        d3=65168685965630356325848133284,
    );

    assert values_ptr1[7] = UInt384(
        d0=18826251302685506831345344084,
        d1=77274307093685476183699352459,
        d2=15618203299442742321497109198,
        d3=35250365537155797686444094446,
    );

    assert values_ptr1[8] = UInt384(
        d0=4600144676297496480144169437,
        d1=49508560736737692883743942698,
        d2=53697907136377617008735648097,
        d3=54120330056905309989570737371,
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
        d0=72771177049276804906298115725,
        d1=39089060273730805410357703519,
        d2=60409936106165898577801465523,
        d3=12006025146310797136053897813,
    );

    assert values_ptr2[3] = UInt384(
        d0=76598284407932491796650762642,
        d1=66981425700865223582276481415,
        d2=25416153705972621336493592301,
        d3=44488314473329323761662510398,
    );

    assert values_ptr2[4] = UInt384(
        d0=28309971518945781801054774815,
        d1=35138656155715921972802013577,
        d2=33567915509971885704499946320,
        d3=56383220350781727457838908033,
    );

    assert values_ptr2[5] = UInt384(
        d0=57693210555312894433383452031,
        d1=58078963466571766988702054687,
        d2=30730933202371999728908891249,
        d3=73279646399142432760249769914,
    );

    let (add_mod_offsets_ptr1) = get_label_location(add_offsets1);
    let (add_mod_offsets_ptr2) = get_label_location(add_offsets2);

    // Apply the add_mod builtin.
    assert add_mod_ptr[0] = ModBuiltin(
        p=p1,
        values_ptr=values_ptr1,
        offsets_ptr=add_mod_offsets_ptr1,
        n=3,
    );
    assert add_mod_ptr[1] = ModBuiltin(
        p=p1,
        values_ptr=values_ptr1,
        offsets_ptr=add_mod_offsets_ptr1 + 3,
        n=2,
    );
    assert add_mod_ptr[2] = ModBuiltin(
        p=p1,
        values_ptr=values_ptr1,
        offsets_ptr=add_mod_offsets_ptr1 + 3*2,
        n=1,
    );
    assert add_mod_ptr[3] = ModBuiltin(
        p=p2,
        values_ptr=values_ptr2,
        offsets_ptr=add_mod_offsets_ptr2,
        n=2,
    );
    assert add_mod_ptr[4] = ModBuiltin(
        p=p2,
        values_ptr=values_ptr2,
        offsets_ptr=add_mod_offsets_ptr2 + 3,
        n=1,
    );
    let add_mod_ptr = add_mod_ptr + 5 * ModBuiltin.SIZE;

    do_add_mod(n_builtin_usages=n_builtin_usages - 1);
    return ();

    add_offsets1:
    dw 0;
    dw 4;
    dw 8;
    dw 24;
    dw 28;
    dw 32;
    dw 12;
    dw 16;
    dw 20;

    add_offsets2:
    dw 12;
    dw 16;
    dw 20;
    dw 0;
    dw 4;
    dw 8;
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
    add_mod_ptr : ModBuiltin*,
    mul_mod_ptr,
}() {
    alloc_locals;
    local n_add_mod = 50;

    // Call add_mod builtin.
    do_add_mod(n_builtin_usages=n_add_mod);

    return ();
}
