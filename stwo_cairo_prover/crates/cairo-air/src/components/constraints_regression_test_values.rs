use expect_test::{expect, Expect};

pub const ADD_AP_OPCODE: Expect = expect![[r#"
    (516897519 + 1850193407i) + (1806878744 + 187524219i)u
"#]];
pub const ADD_MOD_BUILTIN: Expect = expect![[r#"
    (962362941 + 2028338795i) + (1661967568 + 39048394i)u
"#]];
pub const ADD_OPCODE_SMALL: Expect = expect![[r#"
    (694474879 + 672709585i) + (499918043 + 1005895843i)u
"#]];
pub const ADD_OPCODE: Expect = expect![[r#"
    (1743563322 + 642474410i) + (1589520095 + 1661529002i)u
"#]];
pub const ASSERT_EQ_OPCODE_DOUBLE_DEREF: Expect = expect![[r#"
    (319295236 + 399115361i) + (449061143 + 958443158i)u
"#]];
pub const ASSERT_EQ_OPCODE_IMM: Expect = expect![[r#"
    (178653785 + 845156067i) + (155408132 + 1001002813i)u
"#]];
pub const ASSERT_EQ_OPCODE: Expect = expect![[r#"
    (49636834 + 1572439984i) + (878930484 + 2047808655i)u
"#]];
pub const BITWISE_BUILTIN: Expect = expect![[r#"
    (1824891900 + 955303235i) + (675524294 + 522539128i)u
"#]];
pub const BLAKE_COMPRESS_OPCODE: Expect = expect![[r#"
    (155666272 + 1449099244i) + (2129074048 + 98118208i)u
"#]];
pub const BLAKE_G: Expect = expect![[r#"
    (1764683956 + 1445328965i) + (1747106891 + 897069792i)u
"#]];
pub const BLAKE_ROUND_SIGMA: Expect = expect![[r#"
    (1097485784 + 777282969i) + (604469751 + 1994507613i)u
"#]];
pub const BLAKE_ROUND: Expect = expect![[r#"
    (1154303975 + 1817245278i) + (625110977 + 1345116621i)u
"#]];
pub const CALL_OPCODE_REL_IMM: Expect = expect![[r#"
    (760384448 + 1362812134i) + (115035496 + 485861047i)u
"#]];
pub const CALL_OPCODE_ABS: Expect = expect![[r#"
    (1421856702 + 2139630331i) + (803565856 + 823736821i)u
"#]];
pub const CUBE_252: Expect = expect![[r#"
    (593453477 + 1585603168i) + (1656051478 + 1832911948i)u
"#]];
pub const EC_OP_BUILTIN: Expect = expect![[r#"
    (791429500 + 1720267412i) + (1336427407 + 1936025114i)u
"#]];
pub const GENERIC_OPCODE: Expect = expect![[r#"
    (1394044458 + 1523620864i) + (2097664597 + 1506659344i)u
"#]];
pub const JNZ_OPCODE_TAKEN: Expect = expect![[r#"
    (234087556 + 763961986i) + (848830079 + 1170752701i)u
"#]];
pub const JNZ_OPCODE_NON_TAKEN: Expect = expect![[r#"
    (1600907259 + 1130535989i) + (1880812788 + 2095081929i)u
"#]];
pub const JUMP_OPCODE_DOUBLE_DEREF: Expect = expect![[r#"
    (633864483 + 1418769799i) + (933019036 + 1980262481i)u
"#]];
pub const JUMP_OPCODE_REL_IMM: Expect = expect![[r#"
    (1808938333 + 549767725i) + (2064733711 + 1013978252i)u
"#]];
pub const JUMP_OPCODE_REL: Expect = expect![[r#"
    (1905318571 + 1328129507i) + (668609582 + 975107412i)u
"#]];
pub const JUMP_OPCODE_ABS: Expect = expect![[r#"
    (998345091 + 2028413139i) + (1860260793 + 1836467596i)u
"#]];
pub const MEMORY_ADDRESS_TO_ID: Expect = expect![[r#"
    (1614147890 + 1911636193i) + (1028002099 + 432864212i)u
"#]];
pub const MEMORY_ID_TO_SMALL: Expect = expect![[r#"
    (281962446 + 2066139697i) + (922252316 + 81791388i)u
"#]];
pub const BIG_MEMORY_ID_TO_BIG: Expect = expect![[r#"
    (1721360873 + 1066712461i) + (1992693108 + 727836717i)u
"#]];
pub const MUL_MOD_BUILTIN: Expect = expect![[r#"
    (992304303 + 1797117466i) + (786535578 + 877514361i)u
"#]];
pub const MUL_OPCODE_SMALL: Expect = expect![[r#"
    (520318153 + 688909842i) + (1954113231 + 257430291i)u
"#]];
pub const MUL_OPCODE: Expect = expect![[r#"
    (1371509821 + 1944040430i) + (27801671 + 347173398i)u
"#]];
pub const PEDERSEN_AGGREGATOR_WINDOW_BITS_18: Expect = expect![[r#"
    (41079903 + 1702994602i) + (1706854790 + 402437045i)u
"#]];
pub const PARTIAL_EC_MUL_GENERIC: Expect = expect![[r#"
    (1126605007 + 1721691547i) + (1169832585 + 1979217839i)u
"#]];
pub const PARTIAL_EC_MUL_WINDOW_BITS_18: Expect = expect![[r#"
    (1108696589 + 251185570i) + (980553688 + 1810286524i)u
"#]];
pub const PEDERSEN_BUILTIN: Expect = expect![[r#"
    (969711901 + 1392199844i) + (1334358869 + 876891828i)u
"#]];
pub const PEDERSEN_POINTS_TABLE_WINDOW_BITS_18: Expect = expect![[r#"
    (1058665216 + 678067404i) + (1473240927 + 24440930i)u
"#]];
pub const PEDERSEN_AGGREGATOR_WINDOW_BITS_9: Expect = expect![[r#"
    (714921143 + 1954040354i) + (1440862981 + 1560205159i)u
"#]];
pub const PARTIAL_EC_MUL_WINDOW_BITS_9: Expect = expect![[r#"
    (1716820607 + 1350896267i) + (307663276 + 666050347i)u
"#]];
pub const PEDERSEN_BUILTIN_NARROW_WINDOWS: Expect = expect![[r#"
    (376299506 + 1084003754i) + (1941085737 + 849715726i)u
"#]];
pub const PEDERSEN_POINTS_TABLE_WINDOW_BITS_9: Expect = expect![[r#"
    (1826400922 + 886383461i) + (1136857239 + 182340749i)u
"#]];
pub const POSEIDON_3_PARTIAL_ROUNDS_CHAIN: Expect = expect![[r#"
    (1611922424 + 468575745i) + (2083468074 + 1292697274i)u
"#]];
pub const POSEIDON_BUILTIN: Expect = expect![[r#"
    (1759340471 + 1370483238i) + (545738155 + 358148773i)u
"#]];
pub const POSEIDON_AGGREGATOR: Expect = expect![[r#"
    (1285570206 + 458099145i) + (801615907 + 439650025i)u
"#]];
pub const POSEIDON_FULL_ROUND_CHAIN: Expect = expect![[r#"
    (1757641738 + 1279855357i) + (696757604 + 372294085i)u
"#]];
pub const POSEIDON_ROUND_KEYS: Expect = expect![[r#"
    (425854113 + 1624523814i) + (451994973 + 1589445042i)u
"#]];
pub const QM_31_ADD_MUL_OPCODE: Expect = expect![[r#"
    (1873354922 + 678955234i) + (1108874319 + 994914790i)u
"#]];
pub const RANGE_CHECK_96_BUILTIN: Expect = expect![[r#"
    (1978135976 + 1651636415i) + (1817070127 + 298075321i)u
"#]];
pub const RANGE_CHECK_BUILTIN: Expect = expect![[r#"
    (1552037947 + 2022771875i) + (221528730 + 1383803744i)u
"#]];
pub const RANGE_CHECK_252_WIDTH_27: Expect = expect![[r#"
    (692904064 + 1704385925i) + (2070358011 + 204298786i)u
"#]];
pub const RANGE_CHECK_6: Expect = expect![[r#"
    (1287392387 + 1489610743i) + (367080702 + 1808591126i)u
"#]];
pub const RANGE_CHECK_8: Expect = expect![[r#"
    (31228837 + 2029748033i) + (1187429124 + 254576204i)u
"#]];
pub const RANGE_CHECK_11: Expect = expect![[r#"
    (786977737 + 260135648i) + (883688678 + 1951986654i)u
"#]];
pub const RANGE_CHECK_12: Expect = expect![[r#"
    (2009720849 + 880709310i) + (209677118 + 264373981i)u
"#]];
pub const RANGE_CHECK_18: Expect = expect![[r#"
    (587048399 + 258344417i) + (236793901 + 1014992535i)u
"#]];
pub const RANGE_CHECK_20: Expect = expect![[r#"
    (326397975 + 82526200i) + (811088916 + 47367761i)u
"#]];
pub const RANGE_CHECK_4_3: Expect = expect![[r#"
    (414428100 + 1393115176i) + (1011872446 + 889157547i)u
"#]];
pub const RANGE_CHECK_4_4: Expect = expect![[r#"
    (350622436 + 359072798i) + (2110604343 + 684373928i)u
"#]];
pub const RANGE_CHECK_9_9: Expect = expect![[r#"
    (2013041554 + 2021790978i) + (1426679064 + 35205972i)u
"#]];
pub const RANGE_CHECK_7_2_5: Expect = expect![[r#"
    (706570058 + 1204581711i) + (1655517314 + 301571807i)u
"#]];
pub const RANGE_CHECK_3_6_6_3: Expect = expect![[r#"
    (1941445049 + 1624474806i) + (1148940352 + 550304864i)u
"#]];
pub const RANGE_CHECK_4_4_4_4: Expect = expect![[r#"
    (785268719 + 630372339i) + (378045690 + 1482705202i)u
"#]];
pub const RANGE_CHECK_3_3_3_3_3: Expect = expect![[r#"
    (55171861 + 1765766743i) + (1964482406 + 275096048i)u
"#]];
pub const RET_OPCODE: Expect = expect![[r#"
    (1403836458 + 2140993390i) + (549797753 + 526238080i)u
"#]];
pub const TRIPLE_XOR_32: Expect = expect![[r#"
    (1152376846 + 753053024i) + (1408555470 + 1523324023i)u
"#]];
pub const VERIFY_BITWISE_XOR_4: Expect = expect![[r#"
    (1408196330 + 613398876i) + (902685482 + 1598646629i)u
"#]];
pub const VERIFY_BITWISE_XOR_7: Expect = expect![[r#"
    (1148280325 + 1727535771i) + (231268374 + 1904995666i)u
"#]];
pub const VERIFY_BITWISE_XOR_8: Expect = expect![[r#"
    (2133776082 + 1230761515i) + (1058454893 + 1979176609i)u
"#]];
pub const VERIFY_BITWISE_XOR_9: Expect = expect![[r#"
    (841088250 + 1499053200i) + (1122584495 + 937798316i)u
"#]];
pub const VERIFY_BITWISE_XOR_12: Expect = expect![[r#"
    (1665543176 + 800367929i) + (1406064337 + 277486427i)u
"#]];
pub const VERIFY_INSTRUCTION: Expect = expect![[r#"
    (1271062552 + 1607124749i) + (712067765 + 354590374i)u
"#]];
