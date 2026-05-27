use expect_test::{expect, Expect};

pub const ADD_AP_OPCODE: Expect = expect![[r#"
    (844891754 + 1248273321i) + (487761271 + 559655087i)u
"#]];
pub const ADD_MOD_BUILTIN: Expect = expect![[r#"
    (962362941 + 2028338795i) + (1661967568 + 39048394i)u
"#]];
pub const ADD_OPCODE_SMALL: Expect = expect![[r#"
    (83589279 + 46373560i) + (1867688469 + 1338192067i)u
"#]];
pub const ADD_OPCODE: Expect = expect![[r#"
    (991271144 + 596195883i) + (1020622550 + 58968609i)u
"#]];
pub const ASSERT_EQ_OPCODE_DOUBLE_DEREF: Expect = expect![[r#"
    (1895388078 + 275276163i) + (344348745 + 745522634i)u
"#]];
pub const ASSERT_EQ_OPCODE_IMM: Expect = expect![[r#"
    (1338329658 + 1846835987i) + (318125818 + 144897692i)u
"#]];
pub const ASSERT_EQ_OPCODE: Expect = expect![[r#"
    (64068087 + 104784564i) + (1391877320 + 1305287027i)u
"#]];
pub const BITWISE_BUILTIN: Expect = expect![[r#"
    (1824891900 + 955303235i) + (675524294 + 522539128i)u
"#]];
pub const BLAKE_COMPRESS_OPCODE: Expect = expect![[r#"
    (842682723 + 1342877184i) + (1898984024 + 8249957i)u
"#]];
pub const BLAKE_G: Expect = expect![[r#"
    (1077232792 + 142299444i) + (1142256924 + 1150624275i)u
"#]];
pub const BLAKE_ROUND_SIGMA: Expect = expect![[r#"
    (1097485784 + 777282969i) + (604469751 + 1994507613i)u
"#]];
pub const BLAKE_ROUND: Expect = expect![[r#"
    (1222032991 + 795558421i) + (1886760989 + 2105089880i)u
"#]];
pub const CALL_OPCODE_REL_IMM: Expect = expect![[r#"
    (793659323 + 938336724i) + (684905702 + 2122092579i)u
"#]];
pub const CALL_OPCODE_ABS: Expect = expect![[r#"
    (1859636802 + 878407725i) + (766837018 + 1814692688i)u
"#]];
pub const CUBE_252: Expect = expect![[r#"
    (1141077780 + 1298228451i) + (275693507 + 842544255i)u
"#]];
pub const EC_OP_BUILTIN: Expect = expect![[r#"
    (791429500 + 1720267412i) + (1336427407 + 1936025114i)u
"#]];
pub const GENERIC_OPCODE: Expect = expect![[r#"
    (37379640 + 1020701111i) + (470630996 + 1730808127i)u
"#]];
pub const JNZ_OPCODE_TAKEN: Expect = expect![[r#"
    (1900719451 + 1097522173i) + (1022669792 + 1328325644i)u
"#]];
pub const JNZ_OPCODE_NON_TAKEN: Expect = expect![[r#"
    (1912640657 + 1021173040i) + (1833884673 + 1617675460i)u
"#]];
pub const JUMP_OPCODE_DOUBLE_DEREF: Expect = expect![[r#"
    (21214181 + 733486179i) + (1811620667 + 1537789211i)u
"#]];
pub const JUMP_OPCODE_REL_IMM: Expect = expect![[r#"
    (1403714793 + 525345599i) + (1058890637 + 456242783i)u
"#]];
pub const JUMP_OPCODE_REL: Expect = expect![[r#"
    (839834172 + 404260212i) + (954115998 + 1464610821i)u
"#]];
pub const JUMP_OPCODE_ABS: Expect = expect![[r#"
    (1593333430 + 1688939133i) + (587593872 + 1952718266i)u
"#]];
pub const MEMORY_ADDRESS_TO_ID: Expect = expect![[r#"
    (1614147890 + 1911636193i) + (1028002099 + 432864212i)u
"#]];
pub const MEMORY_ID_TO_SMALL: Expect = expect![[r#"
    (166194830 + 682190417i) + (907224037 + 2084161304i)u
"#]];
pub const BIG_MEMORY_ID_TO_BIG: Expect = expect![[r#"
    (843927765 + 803429460i) + (141162125 + 959113326i)u
"#]];
pub const MUL_MOD_BUILTIN: Expect = expect![[r#"
    (992304303 + 1797117466i) + (786535578 + 877514361i)u
"#]];
pub const MUL_OPCODE_SMALL: Expect = expect![[r#"
    (499698058 + 1717570284i) + (752060252 + 109985084i)u
"#]];
pub const MUL_OPCODE: Expect = expect![[r#"
    (940184284 + 1307347244i) + (688282253 + 1737169019i)u
"#]];
pub const PEDERSEN_AGGREGATOR_WINDOW_BITS_18: Expect = expect![[r#"
    (1946266126 + 688917506i) + (266944292 + 1872124002i)u
"#]];
pub const PARTIAL_EC_MUL_GENERIC: Expect = expect![[r#"
    (1629922869 + 34560383i) + (1383769778 + 345148896i)u
"#]];
pub const PARTIAL_EC_MUL_WINDOW_BITS_18: Expect = expect![[r#"
    (547114742 + 518395969i) + (1195328526 + 1016712435i)u
"#]];
pub const PEDERSEN_BUILTIN: Expect = expect![[r#"
    (969711901 + 1392199844i) + (1334358869 + 876891828i)u
"#]];
pub const PEDERSEN_POINTS_TABLE_WINDOW_BITS_18: Expect = expect![[r#"
    (1058665216 + 678067404i) + (1473240927 + 24440930i)u
"#]];
pub const PEDERSEN_AGGREGATOR_WINDOW_BITS_9: Expect = expect![[r#"
    (148135514 + 2086914351i) + (1919771681 + 1707664610i)u
"#]];
pub const PARTIAL_EC_MUL_WINDOW_BITS_9: Expect = expect![[r#"
    (1539232588 + 200246866i) + (72726067 + 942842057i)u
"#]];
pub const PEDERSEN_BUILTIN_NARROW_WINDOWS: Expect = expect![[r#"
    (376299506 + 1084003754i) + (1941085737 + 849715726i)u
"#]];
pub const PEDERSEN_POINTS_TABLE_WINDOW_BITS_9: Expect = expect![[r#"
    (1826400922 + 886383461i) + (1136857239 + 182340749i)u
"#]];
pub const POSEIDON_3_PARTIAL_ROUNDS_CHAIN: Expect = expect![[r#"
    (1073436384 + 1610103553i) + (1820003228 + 1809887354i)u
"#]];
pub const POSEIDON_BUILTIN: Expect = expect![[r#"
    (1759340471 + 1370483238i) + (545738155 + 358148773i)u
"#]];
pub const POSEIDON_AGGREGATOR: Expect = expect![[r#"
    (2147065485 + 937856379i) + (124666645 + 1001800223i)u
"#]];
pub const POSEIDON_FULL_ROUND_CHAIN: Expect = expect![[r#"
    (322973250 + 1421587037i) + (2067450889 + 220220162i)u
"#]];
pub const POSEIDON_ROUND_KEYS: Expect = expect![[r#"
    (425854113 + 1624523814i) + (451994973 + 1589445042i)u
"#]];
pub const QM_31_ADD_MUL_OPCODE: Expect = expect![[r#"
    (584278681 + 542227592i) + (1791221227 + 1420875552i)u
"#]];
pub const RANGE_CHECK_96_BUILTIN: Expect = expect![[r#"
    (1978135976 + 1651636415i) + (1817070127 + 298075321i)u
"#]];
pub const RANGE_CHECK_BUILTIN: Expect = expect![[r#"
    (1552037947 + 2022771875i) + (221528730 + 1383803744i)u
"#]];
pub const RANGE_CHECK_252_WIDTH_27: Expect = expect![[r#"
    (1182672678 + 876288204i) + (1530718457 + 663123888i)u
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
    (1145096184 + 1882373280i) + (1718762272 + 1798225458i)u
"#]];
pub const TRIPLE_XOR_32: Expect = expect![[r#"
    (1414181606 + 1084784533i) + (2061057413 + 667172947i)u
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
    (1330808660 + 1073796176i) + (1094381481 + 1187873710i)u
"#]];
