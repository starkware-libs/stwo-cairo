use std::array::from_fn;
use std::rc::Rc;

use num_traits::Zero;
use prover_types::cpu::{Felt252, Felt252Packed27, FELT252PACKED27_N_WORDS, M31};
use starknet_ff::FieldElement;
use stwo_prover::constraint_framework::preprocessed_columns::PreProcessedColumnId;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::core::poly::circle::{CanonicCoset, CircleEvaluation};
use stwo_prover::core::poly::BitReversedOrder;

use super::preprocessed::table_id_to_col_id;

pub const POSEIDON_ROUND_KEYS_TABLE: &str = "poseidon_round_keys";
pub const POSEIDON_ROUND_NUMBER_COLUMN: &str = "poseidon_round_number";

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

pub fn round_keys(round: M31) -> [Felt252Packed27; 3] {
    POSEIDON_ROUND_KEYS[round.0 as usize].map(|k| Felt252Packed27 { limbs: k })
}

pub fn round_keys_field_elements(round: M31) -> [FieldElement; 3] {
    POSEIDON_ROUND_KEYS[round.0 as usize].map(FieldElement::from_mont)
}

pub fn round_keys_m31s(round: M31) -> [[M31; FELT252PACKED27_N_WORDS]; 3] {
    round_keys(round).map(|felt| from_fn(|i| felt.get_m31(i)))
}

#[derive(Debug)]
pub struct PoseidonRoundKeys {}
impl PoseidonRoundKeys {
    pub fn deduce_output(round: M31) -> [Felt252Packed27; 3] {
        round_keys(round)
    }
}

// This value is equal to 2**786 % PRIME, which compensates for the three Montgomery divisions
// by 2**256 performed in the three multiplications of a cubing.
const FELT252_MONT_CUBE_FACTOR: FieldElement = FieldElement::from_mont([
    14731687596718420366,
    8450283861232831494,
    17617383518939119640,
    256247204371237485,
]);

#[derive(Debug)]
pub struct Cube252 {}
impl Cube252 {
    pub fn cube(x: FieldElement) -> FieldElement {
        x * x * x * FELT252_MONT_CUBE_FACTOR
    }

    pub fn deduce_output(x: Felt252Packed27) -> Felt252Packed27 {
        let x: FieldElement = Felt252::from(x).into();
        let x = Self::cube(x);
        Felt252::from(x).into()
    }
}

#[derive(Debug)]
pub struct PoseidonFullRoundChain {}
impl PoseidonFullRoundChain {
    pub fn deduce_output(
        chain: M31,
        round: M31,
        state: [Felt252Packed27; 3],
    ) -> (M31, M31, [Felt252Packed27; 3]) {
        let [x, y, z] = state.map(|x| Cube252::cube(Felt252::from(x).into()));
        let keys = round_keys_field_elements(round);

        let y1_zm1 = y - z;
        let x1_ym1_z1 = x - y1_zm1;
        let x1_y1_zm1 = x + y1_zm1;
        let x1_y1 = x + y;
        let x2_y2 = x1_y1 + x1_y1;

        let new_x = x2_y2 + x1_ym1_z1 + keys[0];
        let new_y = x1_ym1_z1 + keys[1];
        let new_z = x1_y1_zm1 - z + keys[2];

        let state = [new_x, new_y, new_z].map(|x| Felt252::from(x).into());

        (chain, round + M31::from(1), state)
    }
}

#[derive(Debug)]
pub struct Poseidon3PartialRoundsChain {}
impl Poseidon3PartialRoundsChain {
    fn partial_round_field_elements(
        [z03, z1, z13, z2]: [FieldElement; 4],
        half_key: FieldElement,
    ) -> [FieldElement; 4] {
        let z23 = Cube252::cube(z2);

        let z03_z13 = z03 + z13;
        let z03_z13_z1 = z03_z13 + z1;
        let longsum = z03_z13_z1 + z2 - z23 + half_key;
        let half_z3 = longsum + z03_z13_z1 + z03_z13 + z03;
        let z3 = half_z3 + half_z3;

        [z13, z2, z23, z3]
    }

    pub fn deduce_output(
        chain: M31,
        round: M31,
        state: [Felt252Packed27; 4],
    ) -> (M31, M31, [Felt252Packed27; 4]) {
        let mut state = state.map(|x| Felt252::from(x).into());
        let keys = round_keys_field_elements(round);
        for half_key in keys {
            state = Self::partial_round_field_elements(state, half_key)
        }
        let state = state.map(|x| Felt252::from(x).into());

        (chain, round + M31::from(1), state)
    }
}

#[derive(Debug)]
pub struct PoseidonRoundKeysPackedM31(pub [[PackedM31; FELT252PACKED27_N_WORDS * 3]; 4]);

impl PoseidonRoundKeysPackedM31 {
    pub fn new() -> Self {
        // Add the first row until we have 64 rows
        let first_row = round_keys_m31s(M31::zero());
        let keys: [[[M31; FELT252PACKED27_N_WORDS]; 3]; 64] = from_fn(|i| match i {
            1..35 => round_keys_m31s(M31::from(i)),
            _ => first_row,
        });

        // Pack every 16 rows into PackedM31
        let mut packed = [[[PackedM31::broadcast(M31::zero()); FELT252PACKED27_N_WORDS]; 3]; 4];
        for j in 0..3 {
            for k in 0..FELT252PACKED27_N_WORDS {
                for (i, r) in [0..16, 16..32, 32..48, 48..64].into_iter().enumerate() {
                    packed[i][j][k] = PackedM31::from_array(
                        keys.get(r)
                            .unwrap()
                            .iter()
                            .map(|arr| arr[j][k])
                            .collect::<Vec<_>>()
                            .try_into()
                            .unwrap(),
                    );
                }
            }
        }

        // Flatten the array
        let flat: [[PackedM31; FELT252PACKED27_N_WORDS * 3]; 4] = packed
            .into_iter()
            .map(|arr| {
                arr.into_iter()
                    .flatten()
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Self(flat)
    }
}

impl Default for PoseidonRoundKeysPackedM31 {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct PoseidonRoundKeysColumn {
    pub keys: Rc<PoseidonRoundKeysPackedM31>,
    pub index: usize,
}

impl PoseidonRoundKeysColumn {
    pub const fn new(keys: Rc<PoseidonRoundKeysPackedM31>, index: usize) -> Self {
        Self { keys, index }
    }

    pub const fn log_size(&self) -> u32 {
        4
    }

    pub fn packed_at(&self, vec_row: usize) -> PackedM31 {
        self.keys.0[vec_row][self.index]
    }

    pub fn gen_column_simd(&self) -> CircleEvaluation<SimdBackend, BaseField, BitReversedOrder> {
        CircleEvaluation::new(
            CanonicCoset::new(self.log_size()).circle_domain(),
            BaseColumn::from_simd(
                (0..(1 << (self.log_size() - LOG_N_LANES)))
                    .map(|i| self.packed_at(i))
                    .collect(),
            ),
        )
    }

    pub fn id(&self) -> PreProcessedColumnId {
        PreProcessedColumnId {
            id: table_id_to_col_id(POSEIDON_ROUND_KEYS_TABLE, self.index),
        }
    }
}

#[derive(Debug)]
pub struct PoseidonRoundNumber {}

impl PoseidonRoundNumber {
    pub const fn log_size(&self) -> u32 {
        4
    }

    pub fn packed_at(&self, vec_row: usize) -> PackedM31 {
        let nums: [M31; N_LANES] = (vec_row * N_LANES..(vec_row + 1) * N_LANES)
            .map(|i| {
                if i < 35 {
                    M31::from(i as u32)
                } else {
                    M31::zero()
                }
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        PackedM31::from_array(nums)
    }

    pub fn gen_column_simd(&self) -> CircleEvaluation<SimdBackend, BaseField, BitReversedOrder> {
        CircleEvaluation::new(
            CanonicCoset::new(self.log_size()).circle_domain(),
            BaseColumn::from_simd(
                (0..(1 << (self.log_size() - LOG_N_LANES)))
                    .map(|i| self.packed_at(i))
                    .collect(),
            ),
        )
    }

    pub fn id(&self) -> PreProcessedColumnId {
        PreProcessedColumnId {
            id: POSEIDON_ROUND_NUMBER_COLUMN.to_string(),
        }
    }
}

#[test]
fn test_packed_at_round_keys() {
    let keys_packed = Rc::new(PoseidonRoundKeysPackedM31::new());
    for vec_row in 0..4 {
        for i in 0..3 {
            let packed: [[M31; N_LANES]; FELT252PACKED27_N_WORDS] = from_fn(|c| {
                PoseidonRoundKeysColumn::new(keys_packed.clone(), (i * FELT252PACKED27_N_WORDS) + c)
                    .packed_at(vec_row)
                    .to_array()
            });
            for row_in_packed in 0..N_LANES {
                let felt_limbs: [M31; FELT252PACKED27_N_WORDS] = packed
                    .iter()
                    .map(|arr| arr[row_in_packed])
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                let row = (vec_row * N_LANES) + row_in_packed;
                if row < 35 {
                    assert_eq!(
                        Felt252Packed27::from_limbs(&felt_limbs),
                        round_keys(M31::from(row))[i]
                    );
                } else {
                    assert_eq!(
                        Felt252Packed27::from_limbs(&felt_limbs),
                        round_keys(M31::zero())[i]
                    );
                }
            }
        }
    }
}

#[test]
fn test_packed_at_round_number() {
    let nums = PoseidonRoundNumber {};
    let expected: [M31; 64] = from_fn(|i| {
        if i < 35 {
            M31::from(i as u32)
        } else {
            M31::zero()
        }
    });
    let packed: [[M31; 16]; 4] = from_fn(|i| nums.packed_at(i).to_array());
    let packed_flat: [M31; 64] = packed
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    assert_eq!(packed_flat, expected);
}

#[cfg(test)]
fn into_felt252((l, h): (u128, u128)) -> Felt252Packed27 {
    Felt252Packed27 {
        limbs: [
            (l & 0xffffffff_ffffffffu128) as u64,
            (l >> 64) as u64,
            (h & 0xffffffff_ffffffffu128) as u64,
            (h >> 64) as u64,
        ],
    }
}

#[test]
fn test_cube252() {
    assert_eq!(
        Cube252::deduce_output(into_felt252((0, 1u128 << (251 - 128)))),
        into_felt252((
            0x30ea8ae0ccefff980d98e2efa280dcu128,
            0xe7271cbef30e79ffffe8fb09f06c60u128
        ))
    );

    assert_eq!(
        Cube252::deduce_output(into_felt252((
            0xffffffff_ffffffff_ffffffff_ffffffffu128,
            0x07ffffff_ffffffff_ffffffff_ffffffffu128
        ))),
        into_felt252((
            0x30ea8adfe6a3ff980d98e4d90400d5u128,
            0xe7273d3e6c8de0ffffe8fb09f0d8c0u128
        ))
    );

    assert_eq!(
        Cube252::deduce_output(into_felt252((
            0x01234567_89abcdef_fedcba98_76543210u128,
            0x01234567_89abcdef_fedcba98_76543210u128
        ))),
        into_felt252((
            0x7f0d24086b967fd21d60203b176451d4u128,
            0x552b02c038f04d62a878b4b879c017du128
        ))
    );

    assert_eq!(
        Cube252::deduce_output(into_felt252((0xffffffffffffffffffffffffffffffu128, 0u128))),
        into_felt252((
            0x2ffe000000000000043ffffffffffffu128,
            0x4810000000000000000000000000000u128
        ))
    );
}

#[test]
fn test_full_round() {
    assert_eq!(
        PoseidonFullRoundChain::deduce_output(
            M31::zero(),
            M31::zero(),
            [
                into_felt252((
                    0xe58e2ad98109ae4780b7fd8eac77fe70u128,
                    0x6861759ea556a2339dd92f9562a30b9u128
                )),
                into_felt252((
                    0x3da43f76abf28a64e4ab1a22f27508c6u128,
                    0x3827681995d5af9ffc8397a3d00425au128
                )),
                into_felt252((
                    0x2cac75dc279b2d687a0dbe17704a830cu128,
                    0x3a3956d2fad44d0e7f760a2277dc7cbu128
                )),
            ]
        ),
        (
            M31::zero(),
            M31::from(1),
            [
                into_felt252((
                    0x182ac04678b725e9cd28a9a910551228u128,
                    0x7fe46959f384f2a87db105d7a8ef27u128
                )),
                into_felt252((
                    0xb9da7ad31613d5fc91092ddda81c074eu128,
                    0xbda37a4df11995b73b227acf31008cu128
                )),
                into_felt252((
                    0x269c04ea49158efba215b84ebe38196cu128,
                    0x2a756b59d27dc9eacc79f2ec1a3cbceu128
                )),
            ]
        )
    );

    assert_eq!(
        PoseidonFullRoundChain::deduce_output(
            M31::zero(),
            M31::from(3),
            [
                into_felt252((
                    0xcbafc1adb06e95d212132b0c0d0e5646u128,
                    0x208ff902dab0ef051b12fdb03b79a5bu128
                )),
                into_felt252((
                    0x12b2b2f261de39db323c24b6bb80a4e6u128,
                    0x1c2b35cc5f2ffb5c8949be8fdaa0a69u128
                )),
                into_felt252((
                    0x4ea222e01d5fc8d79794cd442dc7e2c7u128,
                    0x6bed17577108d9c3e7248bd95f32a21u128
                )),
            ]
        ),
        (
            M31::zero(),
            M31::from(4),
            [
                into_felt252((
                    0x44fa12d484b6715dda64b90368464504u128,
                    0x849be1221f8c38be2766fcadb3e4c8u128
                )),
                into_felt252((
                    0xfbd0349285a7795ebd6234122d513069u128,
                    0x4af998bff9c606f1b444844f9536292u128
                )),
                into_felt252((
                    0x27192be6b723923be05140f2d45c356eu128,
                    0x20aaaee2bc0e29c34a78721e2ca88abu128
                )),
            ]
        )
    );

    assert_eq!(
        PoseidonFullRoundChain::deduce_output(
            M31::zero(),
            M31::from(34),
            [
                into_felt252((
                    0x932437b264692f535842627ff2a74c19u128,
                    0x1c74c0faeb7590134bad3b2156e51e4u128
                )),
                into_felt252((
                    0xd8cdad6756c0158c4f560628e5035f40u128,
                    0x319010cf1fbc3a6a5cc8d74a7b6e0fbu128
                )),
                into_felt252((
                    0x35c9170386dbaafb0940ba8864dff049u128,
                    0xbcb539f18f2fc93a79d794f43dd10eu128
                )),
            ]
        ),
        (
            M31::zero(),
            M31::from(35),
            [
                into_felt252((
                    0xe932a9bf7456d009b1b174f36d558c5u128,
                    0xfa8c9b6742b6176139365833d001e3u128
                )),
                into_felt252((
                    0x2d16fba2151e4252a2e2111cde08bfe6u128,
                    0x4f04deca4cb7f9f2bd16b1d25b817cau128
                )),
                into_felt252((
                    0x72ab826e9bb5383a8018b59772964892u128,
                    0x58dde0a2a785b395ee2dc7b60b79e94u128
                )),
            ]
        )
    );
}

#[test]
fn test_partial_round() {
    assert_eq!(
        Poseidon3PartialRoundsChain::deduce_output(
            M31::zero(),
            M31::from(4),
            [
                into_felt252((
                    0xe3537dca9f28533cade50a1864816c70u128,
                    0x9b4e4d358f1423380738337bbf67c8u128
                )),
                into_felt252((
                    0xe243fdbf6366aed9d5369d50e2b9b480u128,
                    0x3c4e1f55af4447cadf6bd704eb01d79u128
                )),
                into_felt252((
                    0xa07a6ff0d70f8d1338144fd09791059du128,
                    0x243f648e2ecd9720bb2657aad0a79b5u128
                )),
                into_felt252((
                    0xae15579d749d434bece9fb13fa3a39d7u128,
                    0x396488f7b22e929802a48afea919080u128
                )),
            ]
        ),
        (
            M31::zero(),
            M31::from(5),
            [
                into_felt252((
                    0x8c0a712f0be18a3795c645569683af77u128,
                    0x6533e92d5aa8e4f6867494ec59573cbu128
                )),
                into_felt252((
                    0xc48f1f967e974e894689a7d15f60af10u128,
                    0x37292a0a739ee1dc8b6b27cc4e2a6a4u128
                )),
                into_felt252((
                    0x13a7f72654da48fb022be337f1179920u128,
                    0x6e43f5086c021c40056b2673a6a9a4cu128
                )),
                into_felt252((
                    0xef171642498d0042106c34abdf2c6b5du128,
                    0x63c745ffb88adea76b5f49ce227847fu128
                )),
            ]
        )
    );

    assert_eq!(
        Poseidon3PartialRoundsChain::deduce_output(
            M31::zero(),
            M31::from(30),
            [
                into_felt252((
                    0x1d810ad08ec7292a3e480d064e3c33fdu128,
                    0x24197cdb19d911de8c1aefd79a1e361u128
                )),
                into_felt252((
                    0x3df9d9c8f0ea31661422c910a953e90eu128,
                    0x79bc5f0ef7295007407214b31fc9c5du128
                )),
                into_felt252((
                    0x37f0afa4744515e8b6289c6e7088fa53u128,
                    0x5fede0e8c1d858b07208faaf87e2825u128
                )),
                into_felt252((
                    0xd990717ed1feae68b4ce9a884f5fa1dcu128,
                    0x124ab89fe982b447d1721cab0feeeddu128
                )),
            ]
        ),
        (
            M31::zero(),
            M31::from(31),
            [
                into_felt252((
                    0xb6f492c4e386fea4cae83dcb7d09dcfau128,
                    0x11737e2519e23a2efbeb74703e14806u128
                )),
                into_felt252((
                    0x19c06386a2f3c99c45a26f4bb3484bb7u128,
                    0x681bfdd9318d1399d41f9a200353addu128
                )),
                into_felt252((
                    0xc9509dbbe4d81cc3bda85be223cbab69u128,
                    0x6446e7317d7f5c81b361fa0a425b38fu128
                )),
                into_felt252((
                    0x9e0c2c87b68a3eb97700c355bfbfdf01u128,
                    0x3c04b83fb3af01feceeb140bbab0322u128
                )),
            ]
        )
    );
}
