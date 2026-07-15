use expect_test::{Expect, expect};

pub const ADD_AP_OPCODE: Expect = expect![[r#"
    (458929235 + 5797669i) + (18630804 + 182006167i)u
"#]];
pub const ADD_MOD_BUILTIN: Expect = expect![[r#"
    (1651209292 + 1631949582i) + (1310358215 + 61448705i)u
"#]];
pub const ADD_OPCODE_SMALL: Expect = expect![[r#"
    (790330704 + 1890318687i) + (363257796 + 219355634i)u
"#]];
pub const ADD_OPCODE: Expect = expect![[r#"
    (1820335376 + 1606495929i) + (1095589787 + 302734037i)u
"#]];
pub const ASSERT_EQ_OPCODE_DOUBLE_DEREF: Expect = expect![[r#"
    (379106749 + 915890106i) + (1727457480 + 687607920i)u
"#]];
pub const ASSERT_EQ_OPCODE_IMM: Expect = expect![[r#"
    (1640528803 + 912593356i) + (1117804448 + 835292600i)u
"#]];
pub const ASSERT_EQ_OPCODE: Expect = expect![[r#"
    (1835825903 + 1814637104i) + (1898674174 + 1781006028i)u
"#]];
pub const BITWISE_BUILTIN: Expect = expect![[r#"
    (1665539429 + 987572849i) + (1533020782 + 36443731i)u
"#]];
pub const BLAKE_COMPRESS_OPCODE: Expect = expect![[r#"
    (190894734 + 538505112i) + (1669720451 + 1086013475i)u
"#]];
pub const BLAKE_G: Expect = expect![[r#"
    (246705092 + 1799430179i) + (1405795391 + 63642545i)u
"#]];
pub const BLAKE_ROUND_SIGMA: Expect = expect![[r#"
    (1919813204 + 303792194i) + (1850443067 + 1642783985i)u
"#]];
pub const BLAKE_ROUND: Expect = expect![[r#"
    (1609725911 + 218451019i) + (280517424 + 1450842762i)u
"#]];
pub const CALL_OPCODE_REL_IMM: Expect = expect![[r#"
    (48791559 + 1219950080i) + (1231557619 + 1552327986i)u
"#]];
pub const CALL_OPCODE_ABS: Expect = expect![[r#"
    (629696148 + 589550443i) + (877212980 + 749259896i)u
"#]];
pub const CUBE_252: Expect = expect![[r#"
    (773789263 + 166913065i) + (1423173062 + 1312340067i)u
"#]];
pub const EC_OP_BUILTIN: Expect = expect![[r#"
    (938201201 + 1421409269i) + (456805978 + 163110296i)u
"#]];
pub const GENERIC_OPCODE: Expect = expect![[r#"
    (1142199633 + 673648135i) + (191481650 + 334634890i)u
"#]];
pub const JNZ_OPCODE_TAKEN: Expect = expect![[r#"
    (2048750410 + 1197911359i) + (2132735811 + 1307132594i)u
"#]];
pub const JNZ_OPCODE_NON_TAKEN: Expect = expect![[r#"
    (167557020 + 1356344416i) + (1398484387 + 674171450i)u
"#]];
pub const JUMP_OPCODE_DOUBLE_DEREF: Expect = expect![[r#"
    (221447774 + 324212361i) + (1662679325 + 265449029i)u
"#]];
pub const JUMP_OPCODE_REL_IMM: Expect = expect![[r#"
    (868173947 + 1203005883i) + (1077493193 + 2033720819i)u
"#]];
pub const JUMP_OPCODE_REL: Expect = expect![[r#"
    (1898510355 + 954854253i) + (1217817264 + 497879742i)u
"#]];
pub const JUMP_OPCODE_ABS: Expect = expect![[r#"
    (1972441933 + 101320373i) + (1726947360 + 722287191i)u
"#]];
pub const MEMORY_ADDRESS_TO_ID: Expect = expect![[r#"
    (1460493255 + 551209186i) + (489757161 + 239600975i)u
"#]];
pub const MEMORY_ID_TO_SMALL: Expect = expect![[r#"
    (223728730 + 891909464i) + (568749162 + 1146460109i)u
"#]];
pub const BIG_MEMORY_ID_TO_BIG: Expect = expect![[r#"
    (1190406961 + 588712769i) + (1841624242 + 603808518i)u
"#]];
pub const MUL_MOD_BUILTIN: Expect = expect![[r#"
    (384451193 + 254860802i) + (664033154 + 629925775i)u
"#]];
pub const MUL_OPCODE_SMALL: Expect = expect![[r#"
    (1202721951 + 514785320i) + (700678412 + 1929021808i)u
"#]];
pub const MUL_OPCODE: Expect = expect![[r#"
    (1253368354 + 527331106i) + (770695646 + 1229932516i)u
"#]];
pub const PEDERSEN_AGGREGATOR_WINDOW_BITS_18: Expect = expect![[r#"
    (531857233 + 1312114580i) + (211978392 + 555560856i)u
"#]];
pub const PARTIAL_EC_MUL_GENERIC: Expect = expect![[r#"
    (132347477 + 1076767014i) + (2012787544 + 1536747291i)u
"#]];
pub const PARTIAL_EC_MUL_WINDOW_BITS_18: Expect = expect![[r#"
    (1423873954 + 1196562803i) + (1433032551 + 1980492714i)u
"#]];
pub const PEDERSEN_BUILTIN: Expect = expect![[r#"
    (1242032170 + 69048203i) + (1292682512 + 823080009i)u
"#]];
pub const PEDERSEN_POINTS_TABLE_WINDOW_BITS_18: Expect = expect![[r#"
    (443478850 + 1078221393i) + (1816336587 + 1977606i)u
"#]];
pub const PEDERSEN_AGGREGATOR_WINDOW_BITS_9: Expect = expect![[r#"
    (142061360 + 992199812i) + (1001354806 + 2128918742i)u
"#]];
pub const PARTIAL_EC_MUL_WINDOW_BITS_9: Expect = expect![[r#"
    (1049013921 + 539946614i) + (738932704 + 1735306474i)u
"#]];
pub const PEDERSEN_BUILTIN_NARROW_WINDOWS: Expect = expect![[r#"
    (1437549776 + 172256841i) + (430342274 + 1411266781i)u
"#]];
pub const PEDERSEN_POINTS_TABLE_WINDOW_BITS_9: Expect = expect![[r#"
    (781175284 + 584381295i) + (1223619317 + 2033838154i)u
"#]];
pub const POSEIDON_3_PARTIAL_ROUNDS_CHAIN: Expect = expect![[r#"
    (689933629 + 1471071159i) + (1448564401 + 1524407105i)u
"#]];
pub const POSEIDON_BUILTIN: Expect = expect![[r#"
    (589521837 + 1758201868i) + (1968190374 + 570793848i)u
"#]];
pub const POSEIDON_AGGREGATOR: Expect = expect![[r#"
    (1451888552 + 372173630i) + (849156612 + 1431439826i)u
"#]];
pub const POSEIDON_FULL_ROUND_CHAIN: Expect = expect![[r#"
    (1540453446 + 95805430i) + (1729545520 + 711197199i)u
"#]];
pub const POSEIDON_ROUND_KEYS: Expect = expect![[r#"
    (340984957 + 1979284349i) + (2007042925 + 765864348i)u
"#]];
pub const QM_31_ADD_MUL_OPCODE: Expect = expect![[r#"
    (961483204 + 1770219947i) + (307888555 + 2026660749i)u
"#]];
pub const RANGE_CHECK_96_BUILTIN: Expect = expect![[r#"
    (143198076 + 1103592890i) + (1786860753 + 1563298553i)u
"#]];
pub const RANGE_CHECK_BUILTIN: Expect = expect![[r#"
    (1433546586 + 1531512604i) + (1672013901 + 1829706850i)u
"#]];
pub const RANGE_CHECK_252_WIDTH_27: Expect = expect![[r#"
    (83991019 + 316908254i) + (230337280 + 132647684i)u
"#]];
pub const RANGE_CHECK_6: Expect = expect![[r#"
    (675291943 + 666099435i) + (2137271737 + 1940085756i)u
"#]];
pub const RANGE_CHECK_8: Expect = expect![[r#"
    (833098058 + 1415746034i) + (1372080731 + 200257697i)u
"#]];
pub const RANGE_CHECK_11: Expect = expect![[r#"
    (1290566202 + 594633350i) + (1710365581 + 1129557130i)u
"#]];
pub const RANGE_CHECK_12: Expect = expect![[r#"
    (1554275242 + 91332441i) + (1195447058 + 1966162234i)u
"#]];
pub const RANGE_CHECK_18: Expect = expect![[r#"
    (546996913 + 1900012880i) + (816304037 + 63381250i)u
"#]];
pub const RANGE_CHECK_20: Expect = expect![[r#"
    (138891558 + 304450862i) + (768166567 + 2023113240i)u
"#]];
pub const RANGE_CHECK_4_3: Expect = expect![[r#"
    (1670803808 + 1311328639i) + (1312470982 + 1373178518i)u
"#]];
pub const RANGE_CHECK_4_4: Expect = expect![[r#"
    (1478258889 + 190921212i) + (504984809 + 1160971054i)u
"#]];
pub const RANGE_CHECK_9_9: Expect = expect![[r#"
    (298991121 + 1997205637i) + (846906667 + 116155416i)u
"#]];
pub const RANGE_CHECK_7_2_5: Expect = expect![[r#"
    (1129389884 + 1598570625i) + (1334552802 + 710132322i)u
"#]];
pub const RANGE_CHECK_3_6_6_3: Expect = expect![[r#"
    (2091581501 + 334906348i) + (1223812852 + 519826026i)u
"#]];
pub const RANGE_CHECK_4_4_4_4: Expect = expect![[r#"
    (986813034 + 863959206i) + (239662028 + 336295673i)u
"#]];
pub const RANGE_CHECK_3_3_3_3_3: Expect = expect![[r#"
    (460049470 + 655306871i) + (389215895 + 99380060i)u
"#]];
pub const RET_OPCODE: Expect = expect![[r#"
    (552721435 + 972762329i) + (188626967 + 282442327i)u
"#]];
pub const TRIPLE_XOR_32: Expect = expect![[r#"
    (565903694 + 986807002i) + (1229965565 + 912419251i)u
"#]];
pub const VERIFY_BITWISE_XOR_4: Expect = expect![[r#"
    (1989442030 + 354737447i) + (1017224845 + 993909400i)u
"#]];
pub const VERIFY_BITWISE_XOR_7: Expect = expect![[r#"
    (144747908 + 545875633i) + (517815551 + 1019649583i)u
"#]];
pub const VERIFY_BITWISE_XOR_8: Expect = expect![[r#"
    (998149202 + 235538579i) + (1144376034 + 121409592i)u
"#]];
pub const VERIFY_BITWISE_XOR_9: Expect = expect![[r#"
    (1704264155 + 1827004954i) + (1982906079 + 397183327i)u
"#]];
pub const VERIFY_BITWISE_XOR_12: Expect = expect![[r#"
    (746364129 + 1925104988i) + (1372741568 + 1157223207i)u
"#]];
pub const VERIFY_INSTRUCTION: Expect = expect![[r#"
    (1889994456 + 1813476410i) + (1677938566 + 87262056i)u
"#]];
