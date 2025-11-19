use expect_test::{expect, Expect};

pub const ADD_AP_OPCODE: Expect = expect![[r#"
    (589101622 + 1090351706i) + (104041107 + 1079270075i)u
"#]];
pub const ADD_MOD_BUILTIN: Expect = expect![[r#"
    (962362941 + 2028338795i) + (1661967568 + 39048394i)u
"#]];
pub const ADD_OPCODE_SMALL: Expect = expect![[r#"
    (1339251051 + 1030318460i) + (92043105 + 970547393i)u
"#]];
pub const ADD_OPCODE: Expect = expect![[r#"
    (156686535 + 1417938864i) + (2044502242 + 110290025i)u
"#]];
pub const ASSERT_EQ_OPCODE_DOUBLE_DEREF: Expect = expect![[r#"
    (1941588763 + 1305628079i) + (1576307763 + 1871835466i)u
"#]];
pub const ASSERT_EQ_OPCODE_IMM: Expect = expect![[r#"
    (1017210131 + 895264793i) + (941631259 + 1206146899i)u
"#]];
pub const ASSERT_EQ_OPCODE: Expect = expect![[r#"
    (2022057017 + 1231501795i) + (1700777730 + 1155144815i)u
"#]];
pub const BITWISE_BUILTIN: Expect = expect![[r#"
    (1824891900 + 955303235i) + (675524294 + 522539128i)u
"#]];
pub const BLAKE_COMPRESS_OPCODE: Expect = expect![[r#"
    (1544164973 + 1397357342i) + (1051760993 + 1504581700i)u
"#]];
pub const BLAKE_G: Expect = expect![[r#"
    (1214761540 + 515860879i) + (1444582138 + 786211397i)u
"#]];
pub const BLAKE_ROUND_SIGMA: Expect = expect![[r#"
    (1097485784 + 777282969i) + (604469751 + 1994507613i)u
"#]];
pub const BLAKE_ROUND: Expect = expect![[r#"
    (1154303975 + 1817245278i) + (625110977 + 1345116621i)u
"#]];
pub const CALL_OPCODE_REL_IMM: Expect = expect![[r#"
    (839564697 + 1650560296i) + (829769883 + 949062908i)u
"#]];
pub const CALL_OPCODE_ABS: Expect = expect![[r#"
    (1364047067 + 552701562i) + (828984219 + 1777527432i)u
"#]];
pub const CUBE_252: Expect = expect![[r#"
    (1037973223 + 1298605477i) + (275158739 + 558974209i)u
"#]];
pub const GENERIC_OPCODE: Expect = expect![[r#"
    (940022209 + 222333129i) + (209213476 + 1471105611i)u
"#]];
pub const JNZ_OPCODE_TAKEN: Expect = expect![[r#"
    (462697114 + 630599933i) + (43540185 + 2057603235i)u
"#]];
pub const JNZ_OPCODE_NON_TAKEN: Expect = expect![[r#"
    (291979958 + 1180644715i) + (519552268 + 152742368i)u
"#]];
pub const JUMP_OPCODE_DOUBLE_DEREF: Expect = expect![[r#"
    (380398535 + 510909784i) + (88322840 + 260410509i)u
"#]];
pub const JUMP_OPCODE_REL_IMM: Expect = expect![[r#"
    (1313896945 + 646454969i) + (727260 + 35977804i)u
"#]];
pub const JUMP_OPCODE_REL: Expect = expect![[r#"
    (576156250 + 2035043254i) + (103722753 + 1464111220i)u
"#]];
pub const JUMP_OPCODE_ABS: Expect = expect![[r#"
    (2102094881 + 1050068779i) + (646125376 + 524572413i)u
"#]];
pub const MEMORY_ADDRESS_TO_ID: Expect = expect![[r#"
    (1614147890 + 1911636193i) + (1028002099 + 432864212i)u
"#]];
pub const SMALL_MEMORY_ID_TO_BIG: Expect = expect![[r#"
    (271372123 + 1779135872i) + (852016794 + 1799561719i)u
"#]];
pub const BIG_MEMORY_ID_TO_BIG: Expect = expect![[r#"
    (1721360873 + 1066712461i) + (1992693108 + 727836717i)u
"#]];
pub const MUL_MOD_BUILTIN: Expect = expect![[r#"
    (992304303 + 1797117466i) + (786535578 + 877514361i)u
"#]];
pub const MUL_OPCODE_SMALL: Expect = expect![[r#"
    (231132536 + 1913752884i) + (1557558360 + 1093847801i)u
"#]];
pub const MUL_OPCODE: Expect = expect![[r#"
    (41815338 + 918136265i) + (630313769 + 273769618i)u
"#]];
pub const PEDERSEN_AGGREGATOR: Expect = expect![[r#"
    (74871017 + 1099415281i) + (530724843 + 120650302i)u
"#]];
pub const PARTIAL_EC_MUL: Expect = expect![[r#"
    (1589041280 + 854172237i) + (925272440 + 32929786i)u
"#]];
pub const PEDERSEN_BUILTIN: Expect = expect![[r#"
    (1910522058 + 899257381i) + (1325555920 + 1915826108i)u
"#]];
pub const PEDERSEN_POINTS_TABLE: Expect = expect![[r#"
    (528441308 + 1423130543i) + (1567555347 + 996009982i)u
"#]];
pub const POSEIDON_3_PARTIAL_ROUNDS_CHAIN: Expect = expect![[r#"
    (14043571 + 839875499i) + (1036178337 + 1236394025i)u
"#]];
pub const POSEIDON_BUILTIN: Expect = expect![[r#"
    (1759340471 + 1370483238i) + (545738155 + 358148773i)u
"#]];
pub const POSEIDON_AGGREGATOR: Expect = expect![[r#"
    (1285570206 + 458099145i) + (801615907 + 439650025i)u
"#]];
pub const POSEIDON_FULL_ROUND_CHAIN: Expect = expect![[r#"
    (2092145165 + 1503826994i) + (528511002 + 656705856i)u
"#]];
pub const POSEIDON_ROUND_KEYS: Expect = expect![[r#"
    (425854113 + 1624523814i) + (451994973 + 1589445042i)u
"#]];
pub const QM_31_ADD_MUL_OPCODE: Expect = expect![[r#"
    (717044148 + 283036480i) + (875932608 + 38741017i)u
"#]];
pub const RANGE_CHECK_BUILTIN_BITS_96: Expect = expect![[r#"
    (1978135976 + 1651636415i) + (1817070127 + 298075321i)u
"#]];
pub const RANGE_CHECK_BUILTIN_BITS_128: Expect = expect![[r#"
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
    (495561053 + 2048153392i) + (1118673067 + 31907605i)u
"#]];
pub const RANGE_CHECK_18_B: Expect = expect![[r#"
    (677513322 + 473157611i) + (529119915 + 1130986532i)u
"#]];
pub const RANGE_CHECK_20: Expect = expect![[r#"
    (1467326459 + 1371970780i) + (661691708 + 2008391618i)u
"#]];
pub const RANGE_CHECK_20_B: Expect = expect![[r#"
    (1518087362 + 1446943291i) + (660469348 + 1035604969i)u
"#]];
pub const RANGE_CHECK_20_C: Expect = expect![[r#"
    (32328132 + 712211156i) + (1442490953 + 409425566i)u
"#]];
pub const RANGE_CHECK_20_D: Expect = expect![[r#"
    (194638528 + 768923914i) + (1243909785 + 140480128i)u
"#]];
pub const RANGE_CHECK_20_E: Expect = expect![[r#"
    (856362945 + 34191779i) + (2025931390 + 1661784372i)u
"#]];
pub const RANGE_CHECK_20_F: Expect = expect![[r#"
    (1018673341 + 90904537i) + (1827350222 + 1392838934i)u
"#]];
pub const RANGE_CHECK_20_G: Expect = expect![[r#"
    (1680397758 + 1503656049i) + (461888180 + 766659531i)u
"#]];
pub const RANGE_CHECK_20_H: Expect = expect![[r#"
    (1692880591 + 542072882i) + (2038234457 + 1216261880i)u
"#]];
pub const RANGE_CHECK_4_3: Expect = expect![[r#"
    (414428100 + 1393115176i) + (1011872446 + 889157547i)u
"#]];
pub const RANGE_CHECK_4_4: Expect = expect![[r#"
    (350622436 + 359072798i) + (2110604343 + 684373928i)u
"#]];
pub const RANGE_CHECK_5_4: Expect = expect![[r#"
    (41059202 + 1690748204i) + (1347941681 + 1950724057i)u
"#]];
pub const RANGE_CHECK_9_9: Expect = expect![[r#"
    (1558108163 + 1022850570i) + (1322807602 + 1521370668i)u
"#]];
pub const RANGE_CHECK_9_9_B: Expect = expect![[r#"
    (1237444858 + 1810346995i) + (47465237 + 797600691i)u
"#]];
pub const RANGE_CHECK_9_9_C: Expect = expect![[r#"
    (575720441 + 397595483i) + (1412927279 + 1423780094i)u
"#]];
pub const RANGE_CHECK_9_9_D: Expect = expect![[r#"
    (2061479671 + 1132327618i) + (630905674 + 2049959497i)u
"#]];
pub const RANGE_CHECK_9_9_E: Expect = expect![[r#"
    (1399755254 + 1867059753i) + (1996367716 + 528655253i)u
"#]];
pub const RANGE_CHECK_9_9_F: Expect = expect![[r#"
    (738030837 + 454308241i) + (1214346111 + 1154834656i)u
"#]];
pub const RANGE_CHECK_9_9_G: Expect = expect![[r#"
    (76306420 + 1189040376i) + (432324506 + 1781014059i)u
"#]];
pub const RANGE_CHECK_9_9_H: Expect = expect![[r#"
    (1412238087 + 905476586i) + (1425230346 + 978257602i)u
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
    (6796407 + 249013208i) + (1955248469 + 1454505513i)u
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
    (52907118 + 1592735639i) + (2096201705 + 1163617544i)u
"#]];
pub const VERIFY_BITWISE_XOR_8_B: Expect = expect![[r#"
    (1484601147 + 453233476i) + (984197618 + 1233569099i)u
"#]];
pub const VERIFY_BITWISE_XOR_9: Expect = expect![[r#"
    (841088250 + 1499053200i) + (1122584495 + 937798316i)u
"#]];
pub const VERIFY_BITWISE_XOR_12: Expect = expect![[r#"
    (1441104943 + 1249176516i) + (1626298138 + 350241661i)u
"#]];
pub const VERIFY_INSTRUCTION: Expect = expect![[r#"
    (1271062552 + 1607124749i) + (712067765 + 354590374i)u
"#]];
