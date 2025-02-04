use prover_types::cpu::*;
use starknet_ff::FieldElement;

// This value is equal to 2**786 % PRIME, which compensates for the three Montgomery divisions
// by 2**256 performed in the three multiplications of a cubing.
const FELT252_MONT_CUBE_FACTOR: FieldElement = FieldElement::from_mont([
    14731687596718420366,
    8450283861232831494,
    17617383518939119640,
    256247204371237485,
]);

trait Cube {
    fn cube(self: Self) -> Self;
}

impl Cube for FieldElement {
    /// This method returns the cube of the FieldElement, as if it represents an element directly
    /// rather than in Montgomery form. While normal cubing of an element x = a * R (representing a
    /// in Montgomenry form) results in a**3 * R, this function instead returns x**3 = a**3 * R**3.
    fn cube(self: FieldElement) -> FieldElement {
        self * self * self * FELT252_MONT_CUBE_FACTOR
    }
}

pub fn cube252(x: Felt252Packed27) -> Felt252Packed27 {
    let x: FieldElement = x.into();
    x.cube().into()
}

const POSEIDON_ROUND_KEYS: [[[u64; 4]; 3]; 35] = [
    [
        [
            9808894619969057997,
            2962375666393338310,
            17382841788414994265,
            443257643709112289,
        ],
        [
            12537484503666775718,
            3256805997184644908,
            6617722259049010207,
            543112534054733059,
        ],
        [
            1454046077829682943,
            14331133962181073949,
            2327346812919484995,
            379005027604567203,
        ],
    ],
    [
        [
            316912300309518807,
            9546737057323600779,
            4990939959663297477,
            409555158710929193,
        ],
        [
            14375050875883784322,
            3258765518491372314,
            6123276414968091301,
            564945574506516589,
        ],
        [
            16399159056375946558,
            12401617009203210820,
            11251954111719545950,
            433429710746163456,
        ],
    ],
    [
        [
            7006359817426385501,
            17203056170488800107,
            10266463410669146573,
            302632258003414824,
        ],
        [
            14549101708442237159,
            5447808788302094550,
            5985460154360671870,
            377474158904626280,
        ],
        [
            2016536305309843132,
            7819086366821881070,
            6549900492473011498,
            375041666190951811,
        ],
    ],
    [
        [
            10958522786453251491,
            9697564334799485296,
            12061515884908749995,
            382992757552468126,
        ],
        [
            3123128533353263104,
            1101320594306927398,
            12277506650622088974,
            151394834303922635,
        ],
        [
            10558209943415701402,
            3761550961988184469,
            3770582263098070207,
            337917216919135628,
        ],
    ],
    [
        [
            11059652905750625380,
            13475195141561210865,
            13294395003503408798,
            543485850395037306,
        ],
        [
            2130930448912281502,
            11333634387982439184,
            8850610548639699306,
            457475955817445493,
        ],
        [
            13832015370839427617,
            3536623570151876469,
            6528270901734940966,
            1727329127918258,
        ],
    ],
    [
        [
            8115407971155582787,
            9978560128345434391,
            5056408649803520810,
            262112615523232333,
        ],
        [
            15132830981034848104,
            221062278661831986,
            642558393488344280,
            294435853161867218,
        ],
        [
            17720328485765873457,
            906259302840864515,
            9886887042042513701,
            91167054851387838,
        ],
    ],
    [
        [
            17686069520976319126,
            357140690021361429,
            4698816318705416205,
            393981709058899502,
        ],
        [
            11422699654326280778,
            16059236267229938280,
            14304086719964370791,
            408074902120160445,
        ],
        [
            10098039853740591407,
            18346213706869023683,
            9856189649941491293,
            184406899276982606,
        ],
    ],
    [
        [
            4111625807920905640,
            13925198121954558587,
            12310073145155562618,
            235927056615592132,
        ],
        [
            6384587362686501122,
            8879249956632890389,
            16548116661510213280,
            336779148206005613,
        ],
        [
            16031192111936359076,
            5992351855224207619,
            12627781605286612627,
            62096344547068532,
        ],
    ],
    [
        [
            7132509749599922320,
            6162523224461213171,
            8812310904075333603,
            485641259025949528,
        ],
        [
            9608543551775247566,
            5196481376567266799,
            4241060526105290574,
            127878632248644222,
        ],
        [
            5678830138638110916,
            17150803417208083936,
            3818159621611526901,
            334934582699708306,
        ],
    ],
    [
        [
            2411984091808734148,
            4676885686514770974,
            16024545775274701230,
            35144855618233700,
        ],
        [
            3764367625527066390,
            16203185340231970163,
            3091249709657140605,
            246768161927392025,
        ],
        [
            14145295330358598665,
            8189373517343951529,
            9185965492588609992,
            84513112494072013,
        ],
    ],
    [
        [
            11207056279882910057,
            8343632314543039012,
            11825707279115803854,
            290028553136950168,
        ],
        [
            8324430733814960957,
            1295757100659713237,
            13793768882092694703,
            170786252004278897,
        ],
        [
            3470195965229650799,
            7014342387715054610,
            8068066979310954905,
            29052085287755578,
        ],
    ],
    [
        [
            16182857524744847037,
            13508798581767021717,
            11609421482229546503,
            516546051298708222,
        ],
        [
            15033308840249833218,
            2943419432621774691,
            12721848833217608341,
            495114643959326355,
        ],
        [
            15900588831390415422,
            13235287741266617180,
            199968676028566291,
            31464852864192972,
        ],
    ],
    [
        [
            1620930607287324279,
            5691881440575675201,
            14029655266804088527,
            85281494397439074,
        ],
        [
            14358430905948328990,
            8174075507093900204,
            4259719420728355987,
            198633356278451635,
        ],
        [
            9809328616759644792,
            9452995911559605327,
            14571337138054143278,
            30595350758349726,
        ],
    ],
    [
        [
            5052814846190951527,
            2564319269931470445,
            11667947324380057882,
            381968969372291382,
        ],
        [
            10270535945835093416,
            7013539859298536233,
            12880625276280589921,
            423512085516371736,
        ],
        [
            9247593602616013019,
            9800080834126373385,
            15154714092547675637,
            85949681409890944,
        ],
    ],
    [
        [
            16691816165224508898,
            7632563883998222450,
            7283702476287088318,
            122927053010244988,
        ],
        [
            5281932702559195717,
            3912411525754476767,
            2751980448518808692,
            449246095011271218,
        ],
        [
            2154533012881281233,
            12108066475824676498,
            3101185982842383519,
            23082295823839220,
        ],
    ],
    [
        [
            7605923575436520679,
            17775553940505278137,
            12354955681295648587,
            509503557137574051,
        ],
        [
            6859516056437622059,
            15185460371714151768,
            11379739398280558941,
            467020453759984910,
        ],
        [
            16906035870412618946,
            2048289172670790831,
            3913835398798558993,
            247123888097172708,
        ],
    ],
    [
        [
            14861675641163720274,
            13490102368184285242,
            7347027430097237399,
            12293023119986689,
        ],
        [
            17015804524472763158,
            2030415039026408622,
            17809691364575575612,
            373903064080638948,
        ],
        [
            8022416475434186219,
            17815483025186149958,
            17841645611508634712,
            214671237987350594,
        ],
    ],
    [
        [
            12836252995927977384,
            12965348847163767059,
            16404178258733598267,
            90570121994582570,
        ],
        [
            3307613700375182919,
            6181136657427428089,
            13131983874186228376,
            111501226499533359,
        ],
        [
            17976156798761747365,
            5762323702974965097,
            2597451573586851781,
            505236697901381580,
        ],
    ],
    [
        [
            10381335841935625777,
            14975760611930379479,
            14435427058050060920,
            398310795157470149,
        ],
        [
            7159397558615159963,
            7188734421404393410,
            1719787959693584840,
            314760383409924924,
        ],
        [
            2143638283124439842,
            13645456387251540292,
            13644498560249152495,
            40707004462247922,
        ],
    ],
    [
        [
            4076286098075094248,
            6047377110922245529,
            10310252161423143831,
            4203916682452144,
        ],
        [
            16974446701450911254,
            13817121135003214195,
            2110477587661043379,
            408404362623939579,
        ],
        [
            2551943238291520253,
            7863747909853014700,
            10038172819555036178,
            498557445795487158,
        ],
    ],
    [
        [
            2539457924220004741,
            6798810574668326903,
            734801439896130781,
            197318323104987578,
        ],
        [
            5630865015115736926,
            6924395279250128121,
            6087898613499446423,
            97920604124542022,
        ],
        [
            8238671520399847831,
            17200586436505834896,
            18050188643002787777,
            522418299476559161,
        ],
    ],
    [
        [
            16671769425050870867,
            9818859487908268561,
            14628982416326270968,
            105495391240150891,
        ],
        [
            15264270982597349937,
            3214172504508351054,
            96620451664846254,
            107082329324411929,
        ],
        [
            14217429574808978483,
            128541115086728122,
            9630653827036234478,
            337586787343095831,
        ],
    ],
    [
        [
            8212059728564478324,
            2347043101088709486,
            6567058597747348925,
            136303161555124818,
        ],
        [
            9215571201366006957,
            18390624930749960250,
            12318590206736769157,
            94289926799047171,
        ],
        [
            2681199449952507734,
            2490827916210922767,
            17337862272405306868,
            167761531076143152,
        ],
    ],
    [
        [
            12798085655163047736,
            13696387070792973059,
            5787352356986496426,
            499982807322000917,
        ],
        [
            11741293120501172298,
            2334843635281516844,
            168280946537445205,
            83199885793358504,
        ],
        [
            12631247450401696760,
            1333500347809883553,
            7960218164236031817,
            545118190714259783,
        ],
    ],
    [
        [
            10647924774016649377,
            7324763009135962770,
            16081867801897155361,
            428325268027940951,
        ],
        [
            9916218426490841056,
            1030092679665695813,
            1263314736050787503,
            189382575143994932,
        ],
        [
            13675857909985031082,
            12446306110735075056,
            5033197056310105549,
            437182141684164304,
        ],
    ],
    [
        [
            16017091423340680337,
            16608552316205419099,
            8224269358142049653,
            421369926480120239,
        ],
        [
            9595998208446022457,
            16134937223929573489,
            14473485045201252070,
            492654444776713988,
        ],
        [
            5219139941060837063,
            15853778231351784921,
            17809532253019979818,
            417855011179639500,
        ],
    ],
    [
        [
            10403233932030073012,
            16730279770404598818,
            5644362027553147371,
            71735369062479113,
        ],
        [
            9631935352839043273,
            17054291492818491992,
            17392960852478338690,
            460451922811752815,
        ],
        [
            8477235527901859940,
            9814894372396933310,
            4799990685272759189,
            400282443455003622,
        ],
    ],
    [
        [
            1576843956219139158,
            702430080765383770,
            17965431827539789120,
            396822748194700979,
        ],
        [
            6534711024274141327,
            7130143603302325050,
            16132773823416316657,
            291127484699867608,
        ],
        [
            4689783592352337689,
            13240213583589885272,
            9538724043191226625,
            53501278744087361,
        ],
    ],
    [
        [
            5303664022074446245,
            11845428113003974855,
            12846417839211241712,
            261206892901600457,
        ],
        [
            7213274845438153874,
            3182604331988734929,
            3825798403644353861,
            498424027422114180,
        ],
        [
            16374080072108357107,
            12602249521880983826,
            16789038177262469212,
            434844701481670312,
        ],
    ],
    [
        [
            5424818379886973048,
            12246700522389374047,
            16105275706111921973,
            171092744151981801,
        ],
        [
            6597251438734065845,
            2269299153703490182,
            1681550773894047556,
            497581200702741603,
        ],
        [
            448123443688089620,
            2093069703231428951,
            10690354368775868807,
            47970718237487928,
        ],
    ],
    [
        [
            5663590943682895948,
            1657980836047728369,
            14473564132866859515,
            63033233261729097,
        ],
        [
            10253436943656002480,
            496762026162837186,
            4416882861358244138,
            410250456421992694,
        ],
        [
            13092039789419326359,
            12701778245175598192,
            10053832990213334033,
            99263799527290004,
        ],
    ],
    [
        [
            17612850421305358241,
            8270527177451683327,
            3824004781155827930,
            23420660055616514,
        ],
        [
            14792590694225191985,
            10340526750113228103,
            13907692663639317222,
            419027522602786902,
        ],
        [
            6537420399978421861,
            13000095247594509242,
            4689550804574686720,
            216508944422249833,
        ],
    ],
    [
        [
            5284804903132760421,
            6021193890942533180,
            12919709475177128988,
            388552658857864838,
        ],
        [
            14487504387541221402,
            8671715521049733970,
            11505630672145478718,
            340569955046273223,
        ],
        [
            15605227016987613715,
            4467780628446399859,
            1916547247173479880,
            360408062797054196,
        ],
    ],
    [
        [
            2318724554222443904,
            12462857660673509117,
            1043912215626002694,
            444903370426104614,
        ],
        [
            13771333397279872933,
            8504629457688506196,
            17402104297977249580,
            365482958936297625,
        ],
        [
            9663191740751744091,
            17588211649412389869,
            8849849772044756264,
            441288247586026977,
        ],
    ],
    [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
];

pub fn round_keys_field_elements(round: M31) -> [FieldElement; 3] {
    POSEIDON_ROUND_KEYS[round.0 as usize].map(|data| FieldElement::from_mont(data))
}

pub fn poseidon_round_keys(round: M31) -> [Felt252Packed27; 3] {
    POSEIDON_ROUND_KEYS[round.0 as usize].map(|limbs| Felt252Packed27 { limbs })
}

pub fn poseidon_full_round_chain(
    chain: M31,
    round: M31,
    state: [Felt252Packed27; 3],
) -> (M31, M31, [Felt252Packed27; 3]) {
    let [x, y, z] = state.map(|x| FieldElement::from(x).cube());
    let keys = round_keys_field_elements(round);

    // An implementation of the MDS [[3, 1, 1], [1, -1, 1], [1, 1, -2]] using only
    // 7 field adds/subs (plus 3 more additions for the round keys) and no muls.
    let y1_zm1 = y - z;
    let x1_ym1_z1 = x - y1_zm1;
    let x1_y1_zm1 = x + y1_zm1;
    let x1_y1 = x + y;
    let x2_y2 = x1_y1 + x1_y1;

    let new_x = x2_y2 + x1_ym1_z1 + keys[0];
    let new_y = x1_ym1_z1 + keys[1];
    let new_z = x1_y1_zm1 - z + keys[2];
    let state = [new_x, new_y, new_z].map(Felt252Packed27::from);

    (chain, round + M31::from_u32_unchecked(1), state)
}

pub fn poseidon_partial_round_field_elements(
    [z03, z1, z13, z2]: [FieldElement; 4],
    half_key: FieldElement,
) -> [FieldElement; 4] {
    let z23 = z2.cube();

    // An implementation of the linear combination
    //   z3 = 8*z03 + 4*z1 + 6*z13 + 2*z2 - 2*z23 + 2*half_key
    // using only 9 field adds/subs, and no muls.
    let z03_z13 = z03 + z13;
    let z03_z13_z1 = z03_z13 + z1;
    let longsum = z03_z13_z1 + z2 - z23 + half_key;
    let half_z3 = longsum + z03_z13_z1 + z03_z13 + z03;
    let z3 = half_z3 + half_z3;

    [z13, z2, z23, z3]
}

pub fn poseidon_3_partial_rounds_chain(
    chain: M31,
    round: M31,
    state: [Felt252Packed27; 4],
) -> (M31, M31, [Felt252Packed27; 4]) {
    let mut state = state.map(FieldElement::from);
    let keys = round_keys_field_elements(round);
    for half_key in keys {
        state = poseidon_partial_round_field_elements(state, half_key)
    }
    let state = state.map(Felt252Packed27::from);

    (chain, round + M31::from_u32_unchecked(1), state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube() {
        let (input, output) = (
            Felt252Packed27 {
                limbs: [3, 0, 0, 0],
            },
            Felt252Packed27 {
                limbs: [27, 0, 0, 0],
            },
        );
        assert_eq!(cube252(input), output);

        let (input, output) = (
            Felt252Packed27 {
                limbs: [0, 1, 0, 0],
            },
            Felt252Packed27 {
                limbs: [0, 0, 0, 1],
            },
        );
        assert_eq!(cube252(input), output);

        let (input, output) = (
            Felt252Packed27 {
                limbs: [0, 0, 1, 0],
            },
            Felt252Packed27 {
                limbs: [
                    18446744073700081665,
                    17407,
                    18446744073709551584,
                    576460752142434320,
                ],
            },
        );
        assert_eq!(cube252(input), output);
    }

    #[test]
    fn test_full_round() {
        let input = [
            Felt252Packed27 {
                limbs: [
                    9275160746813554288,
                    16541205595039575623,
                    4169650429605064889,
                    470088886057789987,
                ],
            },
            Felt252Packed27 {
                limbs: [
                    16477292399064058054,
                    4441744911417641572,
                    18431044672185975386,
                    252894828082060025,
                ],
            },
            Felt252Packed27 {
                limbs: [
                    8794894655201903372,
                    3219077422080798056,
                    16714934791572408267,
                    262217499501479120,
                ],
            },
        ];
        let output = [
            Felt252Packed27 {
                limbs: [
                    14783252320432230952,
                    1741415614837630441,
                    12141054809111850791,
                    35998463174083826,
                ],
            },
            Felt252Packed27 {
                limbs: [
                    10450934840321378126,
                    13392151488966678012,
                    13203184643424190604,
                    53378516287297941,
                ],
            },
            Felt252Packed27 {
                limbs: [
                    11679443856993556844,
                    2782104074103131899,
                    12450094718055074766,
                    191216847212240030,
                ],
            },
        ];

        assert_eq!(
            poseidon_full_round_chain(
                M31::from_u32_unchecked(0),
                M31::from_u32_unchecked(0),
                input
            ),
            (
                M31::from_u32_unchecked(0),
                M31::from_u32_unchecked(1),
                output
            )
        );

        let input = [
            Felt252Packed27 {
                limbs: [
                    6359753925471980569,
                    10602660662065311571,
                    3799581997457625572,
                    128154744642820353,
                ],
            },
            Felt252Packed27 {
                limbs: [
                    5716763549704085312,
                    15622333341766784396,
                    11947079443676586235,
                    223210811642594214,
                ],
            },
            Felt252Packed27 {
                limbs: [
                    666737839821549641,
                    3875654258243906299,
                    4213635961134633230,
                    53116556092059593,
                ],
            },
        ];
        let output = [
            Felt252Packed27 {
                limbs: [
                    11176552529239824581,
                    1050229987464277248,
                    7005690447805809123,
                    70523343257908759,
                ],
            },
            Felt252Packed27 {
                limbs: [
                    11736962394562805734,
                    3249060854751969874,
                    3157422586694473674,
                    355870049334689695,
                ],
            },
            Felt252Packed27 {
                limbs: [
                    9230327098362972306,
                    8262841352911665210,
                    6837269606763568788,
                    400220077149412153,
                ],
            },
        ];

        assert_eq!(
            poseidon_full_round_chain(
                M31::from_u32_unchecked(0),
                M31::from_u32_unchecked(34),
                input
            ),
            (
                M31::from_u32_unchecked(0),
                M31::from_u32_unchecked(35),
                output
            )
        );
    }

    #[test]
    fn test_partial_round() {
        let input = [
            Felt252Packed27 {
                limbs: [
                    12530432638109969520,
                    16380574578883711804,
                    4037257384731568072,
                    43714714908169251,
                ],
            },
            Felt252Packed27 {
                limbs: [
                    15363640149547791488,
                    16304154074459582169,
                    12535414902886505849,
                    271590321458267260,
                ],
            },
            Felt252Packed27 {
                limbs: [
                    4040942522972833181,
                    11563678073418124563,
                    842847657835264437,
                    163244804420721010,
                ],
            },
            Felt252Packed27 {
                limbs: [
                    17071451925745646039,
                    12544028670888330059,
                    9235273906268508288,
                    258473809703790889,
                ],
            },
        ];
        let output = [
            Felt252Packed27 {
                limbs: [
                    10792389795234688887,
                    10091002361953749559,
                    7523062305216885707,
                    455776787665423951,
                ],
            },
            Felt252Packed27 {
                limbs: [
                    5082778172154490640,
                    14163574084332899977,
                    14462943502295803556,
                    248422148156812829,
                ],
            },
            Felt252Packed27 {
                limbs: [
                    156468441481976096,
                    1416372351835719931,
                    24403004428556876,
                    496591474008269252,
                ],
            },
            Felt252Packed27 {
                limbs: [
                    1183378714879880029,
                    17228263373486751810,
                    8554012021906441343,
                    449362018420960746,
                ],
            },
        ];
        assert_eq!(
            poseidon_3_partial_rounds_chain(
                M31::from_u32_unchecked(0),
                M31::from_u32_unchecked(4),
                input
            ),
            (
                M31::from_u32_unchecked(0),
                M31::from_u32_unchecked(5),
                output
            )
        );

        let input = [
            Felt252Packed27 {
                limbs: [
                    4487851349408232445,
                    2125992389960476970,
                    16771878890995049313,
                    162577971266031901,
                ],
            },
            Felt252Packed27 {
                limbs: [
                    1450943103364229390,
                    4465839962573123942,
                    8360687840081517661,
                    548249418255734016,
                ],
            },
            Felt252Packed27 {
                limbs: [
                    13125913112119671379,
                    4030914787356775912,
                    513568321996466213,
                    432026768335799691,
                ],
            },
            Felt252Packed27 {
                limbs: [
                    13028520682267320796,
                    15677155092379643496,
                    9013710333641944797,
                    82379302369766212,
                ],
            },
        ];
        let output = [
            Felt252Packed27 {
                limbs: [
                    14621004134443769082,
                    13183323383548804772,
                    17275446736275458054,
                    78592963673727906,
                ],
            },
            Felt252Packed27 {
                limbs: [
                    5017695305765243831,
                    1855592476387297692,
                    11331612611624385245,
                    468866794599797049,
                ],
            },
            Felt252Packed27 {
                limbs: [
                    13666274096077056873,
                    14506267830084181187,
                    1960789462575592335,
                    451607303244215752,
                ],
            },
            Felt252Packed27 {
                limbs: [
                    8575068463570083585,
                    11388526519107862201,
                    17072778128992699170,
                    270299007869972511,
                ],
            },
        ];
        assert_eq!(
            poseidon_3_partial_rounds_chain(
                M31::from_u32_unchecked(0),
                M31::from_u32_unchecked(30),
                input
            ),
            (
                M31::from_u32_unchecked(0),
                M31::from_u32_unchecked(31),
                output
            )
        );
    }
}
