use expect_test::{expect, Expect};

pub const ADD_AP_OPCODE: Expect = expect![[r#"
    (1223906247 + 658066491i) + (698580117 + 1250932223i)u
"#]];
pub const ADD_MOD_BUILTIN: Expect = expect![[r#"
    (81145834 + 1353679277i) + (1261874316 + 1278937625i)u
"#]];
pub const ADD_OPCODE_SMALL: Expect = expect![[r#"
    (1607992135 + 761352859i) + (508319523 + 1440640565i)u
"#]];
pub const ADD_OPCODE: Expect = expect![[r#"
    (1081304484 + 162089784i) + (1049210556 + 1623282977i)u
"#]];
pub const ASSERT_EQ_OPCODE_DOUBLE_DEREF: Expect = expect![[r#"
    (2047540910 + 1491599532i) + (784436801 + 1515184530i)u
"#]];
pub const ASSERT_EQ_OPCODE_IMM: Expect = expect![[r#"
    (755919580 + 1956203874i) + (528911742 + 1152401247i)u
"#]];
pub const ASSERT_EQ_OPCODE: Expect = expect![[r#"
    (17704646 + 682295436i) + (545773498 + 1505312319i)u
"#]];
pub const BITWISE_BUILTIN: Expect = expect![[r#"
    (1938395388 + 1487496143i) + (173944255 + 1931767517i)u
"#]];
pub const BLAKE_COMPRESS_OPCODE: Expect = expect![[r#"
    (793949020 + 184521049i) + (1571061689 + 1699344846i)u
"#]];
pub const BLAKE_G: Expect = expect![[r#"
    (589517194 + 1714400972i) + (124778100 + 1709965715i)u
"#]];
pub const BLAKE_ROUND_SIGMA: Expect = expect![[r#"
    (1679325815 + 2032005875i) + (1831081253 + 999201494i)u
"#]];
pub const BLAKE_ROUND: Expect = expect![[r#"
    (689010793 + 1714718853i) + (1598356108 + 1862084735i)u
"#]];
pub const CALL_OPCODE_REL_IMM: Expect = expect![[r#"
    (1058958974 + 291699845i) + (1067131259 + 1385778936i)u
"#]];
pub const CALL_OPCODE_ABS: Expect = expect![[r#"
    (753588549 + 110829970i) + (382462386 + 1929508194i)u
"#]];
pub const CUBE_252: Expect = expect![[r#"
    (2138805547 + 791862161i) + (1496263445 + 1432185054i)u
"#]];
pub const GENERIC_OPCODE: Expect = expect![[r#"
    (7522005 + 143143560i) + (2147333686 + 272502067i)u
"#]];
pub const JNZ_OPCODE_TAKEN: Expect = expect![[r#"
    (863863896 + 1799155008i) + (2087434855 + 1577176263i)u
"#]];
pub const JNZ_OPCODE_NON_TAKEN: Expect = expect![[r#"
    (527000951 + 699354015i) + (1174877270 + 2099424613i)u
"#]];
pub const JUMP_OPCODE_DOUBLE_DEREF: Expect = expect![[r#"
    (1104346987 + 1319193194i) + (162676979 + 376017635i)u
"#]];
pub const JUMP_OPCODE_REL_IMM: Expect = expect![[r#"
    (79712695 + 631991609i) + (2034114495 + 1081569364i)u
"#]];
pub const JUMP_OPCODE_REL: Expect = expect![[r#"
    (1321760482 + 1745914118i) + (2079741091 + 369317996i)u
"#]];
pub const JUMP_OPCODE_ABS: Expect = expect![[r#"
    (1822069462 + 1014890554i) + (653917245 + 1477843164i)u
"#]];
pub const MEMORY_ADDRESS_TO_ID: Expect = expect![[r#"
    (1557081375 + 1030421885i) + (614292297 + 1358103414i)u
"#]];
pub const SMALL_MEMORY_ID_TO_BIG: Expect = expect![[r#"
    (1926563719 + 1031963008i) + (80105391 + 1734885910i)u
"#]];
pub const BIG_MEMORY_ID_TO_BIG: Expect = expect![[r#"
    (482978858 + 145449557i) + (1999339889 + 1114454455i)u
"#]];
pub const MUL_MOD_BUILTIN: Expect = expect![[r#"
    (1776953875 + 544868048i) + (742165102 + 828119419i)u
"#]];
pub const MUL_OPCODE_SMALL: Expect = expect![[r#"
    (31568448 + 1396831613i) + (391800554 + 787822387i)u
"#]];
pub const MUL_OPCODE: Expect = expect![[r#"
    (1765238355 + 567102279i) + (1303576643 + 132743394i)u
"#]];
pub const PEDERSEN_AGGREGATOR: Expect = expect![[r#"
    (1935068275 + 861457484i) + (903392461 + 608363705i)u
"#]];
pub const PARTIAL_EC_MUL: Expect = expect![[r#"
    (895051799 + 1568254972i) + (1038712019 + 2090679041i)u
"#]];
pub const PEDERSEN_BUILTIN: Expect = expect![[r#"
    (1510178630 + 1309842420i) + (2057566104 + 833830564i)u
"#]];
pub const PEDERSEN_POINTS_TABLE: Expect = expect![[r#"
    (2016504587 + 1568791972i) + (1116506128 + 884768463i)u
"#]];
pub const POSEIDON_3_PARTIAL_ROUNDS_CHAIN: Expect = expect![[r#"
    (2033262826 + 667964543i) + (319252363 + 1402641406i)u
"#]];
pub const POSEIDON_BUILTIN: Expect = expect![[r#"
    (628591667 + 1555110826i) + (854671776 + 1476141013i)u
"#]];
pub const POSEIDON_AGGREGATOR: Expect = expect![[r#"
    (2094581775 + 1775137527i) + (282111203 + 1239842877i)u
"#]];
pub const POSEIDON_FULL_ROUND_CHAIN: Expect = expect![[r#"
    (941127383 + 452604700i) + (507558048 + 839794501i)u
"#]];
pub const POSEIDON_ROUND_KEYS: Expect = expect![[r#"
    (1182501131 + 1048397214i) + (2019522910 + 988967040i)u
"#]];
pub const QM_31_ADD_MUL_OPCODE: Expect = expect![[r#"
    (989822133 + 613070772i) + (258458924 + 1917445389i)u
"#]];
pub const RANGE_CHECK_96_BUILTIN: Expect = expect![[r#"
    (1203114103 + 1251795602i) + (1884838449 + 1758114216i)u
"#]];
pub const RANGE_CHECK_BUILTIN: Expect = expect![[r#"
    (98318865 + 510635574i) + (1814943674 + 477505709i)u
"#]];
pub const RANGE_CHECK_252_WIDTH_27: Expect = expect![[r#"
    (432510888 + 1744417094i) + (1925158638 + 1069774182i)u
"#]];
pub const RANGE_CHECK_6: Expect = expect![[r#"
    (1839596244 + 1029136098i) + (314080530 + 1911163288i)u
"#]];
pub const RANGE_CHECK_8: Expect = expect![[r#"
    (619380186 + 51892308i) + (1959448152 + 27164417i)u
"#]];
pub const RANGE_CHECK_11: Expect = expect![[r#"
    (1797671535 + 1448714071i) + (655307656 + 610124137i)u
"#]];
pub const RANGE_CHECK_12: Expect = expect![[r#"
    (2029705150 + 1702041781i) + (1878593402 + 1237923315i)u
"#]];
pub const RANGE_CHECK_18: Expect = expect![[r#"
    (879202200 + 1041935930i) + (627046766 + 1207731371i)u
"#]];
pub const RANGE_CHECK_20: Expect = expect![[r#"
    (132433986 + 929791686i) + (719321861 + 603794429i)u
"#]];
pub const RANGE_CHECK_4_3: Expect = expect![[r#"
    (1196459453 + 1634762104i) + (400394686 + 2114180246i)u
"#]];
pub const RANGE_CHECK_4_4: Expect = expect![[r#"
    (1164138582 + 762844355i) + (1425321120 + 1242804536i)u
"#]];
pub const RANGE_CHECK_9_9: Expect = expect![[r#"
    (1982132292 + 2089235785i) + (358419016 + 1940121199i)u
"#]];
pub const RANGE_CHECK_7_2_5: Expect = expect![[r#"
    (2147119250 + 839921636i) + (589036134 + 149935499i)u
"#]];
pub const RANGE_CHECK_3_6_6_3: Expect = expect![[r#"
    (988141449 + 852070290i) + (1651734195 + 1092922573i)u
"#]];
pub const RANGE_CHECK_4_4_4_4: Expect = expect![[r#"
    (889035083 + 1848776877i) + (1035214492 + 587702437i)u
"#]];
pub const RANGE_CHECK_3_3_3_3_3: Expect = expect![[r#"
    (1008319034 + 1849060917i) + (2093248124 + 937701073i)u
"#]];
pub const RET_OPCODE: Expect = expect![[r#"
    (160301559 + 831378595i) + (27495844 + 126354001i)u
"#]];
pub const TRIPLE_XOR_32: Expect = expect![[r#"
    (294647251 + 721238886i) + (920521736 + 1654795886i)u
"#]];
pub const VERIFY_BITWISE_XOR_4: Expect = expect![[r#"
    (2073514027 + 254560307i) + (221445278 + 572607020i)u
"#]];
pub const VERIFY_BITWISE_XOR_7: Expect = expect![[r#"
    (1901363990 + 1932874508i) + (1322897748 + 1124138466i)u
"#]];
pub const VERIFY_BITWISE_XOR_8: Expect = expect![[r#"
    (603902742 + 1751375822i) + (1335533187 + 1410219767i)u
"#]];
pub const VERIFY_BITWISE_XOR_9: Expect = expect![[r#"
    (1084186033 + 1944126042i) + (1899018540 + 496255699i)u
"#]];
pub const VERIFY_BITWISE_XOR_12: Expect = expect![[r#"
    (271618984 + 716415572i) + (1162434803 + 168934829i)u
"#]];
pub const VERIFY_INSTRUCTION: Expect = expect![[r#"
    (1751946169 + 1684131734i) + (814276776 + 1152537782i)u
"#]];
