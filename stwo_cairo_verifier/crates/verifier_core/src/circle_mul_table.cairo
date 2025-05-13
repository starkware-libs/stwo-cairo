use crate::circle::CirclePoint;
use crate::fields::m31::M31BoundedInt;

/// Index `i` stores `M31_CIRCLE_GEN * i`.
pub const M31_CIRCLE_GEN_MUL_TABLE_BITS_0_TO_5: [CirclePoint<M31BoundedInt>; 64] = [
    CirclePoint { x: 1, y: 0 }, CirclePoint { x: 2, y: 1268011823 },
    CirclePoint { x: 7, y: 777079998 }, CirclePoint { x: 26, y: 1840308169 },
    CirclePoint { x: 97, y: 141701737 }, CirclePoint { x: 362, y: 873982426 },
    CirclePoint { x: 1351, y: 1206744320 }, CirclePoint { x: 5042, y: 1805511207 },
    CirclePoint { x: 18817, y: 1720333214 }, CirclePoint { x: 70226, y: 780854355 },
    CirclePoint { x: 262087, y: 1403084206 }, CirclePoint { x: 978122, y: 536515175 },
    CirclePoint { x: 3650401, y: 742976494 }, CirclePoint { x: 13623482, y: 287907154 },
    CirclePoint { x: 50843527, y: 408652122 }, CirclePoint { x: 189750626, y: 1346701334 },
    CirclePoint { x: 708158977, y: 683185920 }, CirclePoint { x: 495401635, y: 1386042346 },
    CirclePoint { x: 1273447563, y: 566016170 }, CirclePoint { x: 303421323, y: 878022334 },
    CirclePoint { x: 2087721376, y: 798589519 }, CirclePoint { x: 1605013240, y: 168852095 },
    CirclePoint { x: 37364290, y: 2024302508 }, CirclePoint { x: 691927567, y: 1485906996 },
    CirclePoint { x: 582862331, y: 1771841829 }, CirclePoint { x: 1639521757, y: 1306493026 },
    CirclePoint { x: 1680257403, y: 1306646628 }, CirclePoint { x: 786540561, y: 1772609839 },
    CirclePoint { x: 1465904841, y: 1488825434 }, CirclePoint { x: 782111509, y: 2035208250 },
    CirclePoint { x: 1662541195, y: 209556625 }, CirclePoint { x: 1573085977, y: 950501897 },
    CirclePoint { x: 334835419, y: 1444967316 }, CirclePoint { x: 1913739346, y: 534400073 },
    CirclePoint { x: 877671024, y: 692632976 }, CirclePoint { x: 1596944750, y: 88648184 },
    CirclePoint { x: 1215140682, y: 1809443407 }, CirclePoint { x: 1116134331, y: 706674503 },
    CirclePoint { x: 1101912995, y: 1017254605 }, CirclePoint { x: 1144034002, y: 1214860270 },
    CirclePoint { x: 1326739366, y: 1694702828 }, CirclePoint { x: 2015439815, y: 1268983748 },
    CirclePoint { x: 292568953, y: 1233748517 }, CirclePoint { x: 1302319644, y: 1518526673 },
    CirclePoint { x: 621742329, y: 545390881 }, CirclePoint { x: 1184649672, y: 663036851 },
    CirclePoint { x: 1969372712, y: 2106756523 }, CirclePoint { x: 250390235, y: 1321538300 },
    CirclePoint { x: 1179671875, y: 1031913030 }, CirclePoint { x: 173329971, y: 658630173 },
    CirclePoint { x: 1661131656, y: 1602607662 }, CirclePoint { x: 28745712, y: 1456833181 },
    CirclePoint { x: 601334839, y: 2077241415 }, CirclePoint { x: 229109997, y: 409681538 },
    CirclePoint { x: 315105149, y: 1708968384 }, CirclePoint { x: 1031310599, y: 2131224704 },
    CirclePoint { x: 1662653600, y: 373479491 }, CirclePoint { x: 1324336507, y: 1510176907 },
    CirclePoint { x: 1487208781, y: 1372260843 }, CirclePoint { x: 329531323, y: 1831382818 },
    CirclePoint { x: 1978400158, y: 1658303135 }, CirclePoint { x: 1141618368, y: 506862428 },
    CirclePoint { x: 440589667, y: 369146577 }, CirclePoint { x: 620740300, y: 969723880 },
];

/// Index `i` stores `M31_CIRCLE_GEN * i * 2^6`.
pub const M31_CIRCLE_GEN_MUL_TABLE_BITS_6_TO_11: [CirclePoint<M31BoundedInt>; 64] = [
    CirclePoint { x: 1, y: 0 }, CirclePoint { x: 2042371533, y: 1362265296 },
    CirclePoint { x: 212706801, y: 1223819887 }, CirclePoint { x: 1343082982, y: 1656388718 },
    CirclePoint { x: 421007138, y: 256177860 }, CirclePoint { x: 1637047685, y: 109864373 },
    CirclePoint { x: 1594705154, y: 1156423391 }, CirclePoint { x: 1326191972, y: 1554410474 },
    CirclePoint { x: 6346213, y: 905523693 }, CirclePoint { x: 1603309155, y: 1317339952 },
    CirclePoint { x: 982766847, y: 1369509443 }, CirclePoint { x: 1130890301, y: 1314091057 },
    CirclePoint { x: 784830374, y: 912241638 }, CirclePoint { x: 529167155, y: 49384636 },
    CirclePoint { x: 1475671350, y: 739745329 }, CirclePoint { x: 418523497, y: 1921984699 },
    CirclePoint { x: 1022251061, y: 788094511 }, CirclePoint { x: 1453585800, y: 1686756607 },
    CirclePoint { x: 2144756169, y: 1973977555 }, CirclePoint { x: 222118537, y: 1014253609 },
    CirclePoint { x: 1774187400, y: 1713497086 }, CirclePoint { x: 1447121614, y: 1053732556 },
    CirclePoint { x: 105012991, y: 261334420 }, CirclePoint { x: 1509980434, y: 1580004926 },
    CirclePoint { x: 999230472, y: 1852479283 }, CirclePoint { x: 517742188, y: 968413651 },
    CirclePoint { x: 288424812, y: 1959942454 }, CirclePoint { x: 1136461567, y: 2028297119 },
    CirclePoint { x: 344093444, y: 1723950257 }, CirclePoint { x: 2162587, y: 1662102570 },
    CirclePoint { x: 203869561, y: 313423390 }, CirclePoint { x: 57053772, y: 179506272 },
    CirclePoint { x: 1633461177, y: 574296567 }, CirclePoint { x: 2063130879, y: 1272838818 },
    CirclePoint { x: 1655011431, y: 2085598907 }, CirclePoint { x: 1882985882, y: 1884234849 },
    CirclePoint { x: 505774551, y: 1187398701 }, CirclePoint { x: 39852141, y: 123936243 },
    CirclePoint { x: 1676081227, y: 1135991864 }, CirclePoint { x: 603512862, y: 13573590 },
    CirclePoint { x: 1743980363, y: 345791924 }, CirclePoint { x: 1221120023, y: 1584263814 },
    CirclePoint { x: 319360190, y: 618255967 }, CirclePoint { x: 937276350, y: 1889866233 },
    CirclePoint { x: 638925477, y: 1200786367 }, CirclePoint { x: 2050207099, y: 979596272 },
    CirclePoint { x: 1091919627, y: 1535715623 }, CirclePoint { x: 327335145, y: 1879026140 },
    CirclePoint { x: 2054061671, y: 949045631 }, CirclePoint { x: 2017447582, y: 2138555804 },
    CirclePoint { x: 1230785717, y: 932947748 }, CirclePoint { x: 1045835668, y: 1112919339 },
    CirclePoint { x: 1399167609, y: 1906303117 }, CirclePoint { x: 968612238, y: 1641022910 },
    CirclePoint { x: 1696585972, y: 468920631 }, CirclePoint { x: 19290124, y: 887640976 },
    CirclePoint { x: 1309393381, y: 53742718 }, CirclePoint { x: 2120725548, y: 846203964 },
    CirclePoint { x: 1273782452, y: 924138332 }, CirclePoint { x: 1759380646, y: 1693051226 },
    CirclePoint { x: 2080156404, y: 1988869649 }, CirclePoint { x: 314388810, y: 1013513450 },
    CirclePoint { x: 1619251001, y: 830366565 }, CirclePoint { x: 1385987562, y: 2130936594 },
];

/// Index `i` stores `M31_CIRCLE_GEN * i * 2^12`.
pub const M31_CIRCLE_GEN_MUL_TABLE_BITS_12_TO_17: [CirclePoint<M31BoundedInt>; 64] = [
    CirclePoint { x: 1, y: 0 }, CirclePoint { x: 595037635, y: 2111542451 },
    CirclePoint { x: 1799120754, y: 343598868 }, CirclePoint { x: 1475016787, y: 1876816901 },
    CirclePoint { x: 438833264, y: 1327019128 }, CirclePoint { x: 433600440, y: 167145119 },
    CirclePoint { x: 503318431, y: 730224775 }, CirclePoint { x: 1124768046, y: 299494314 },
    CirclePoint { x: 1389168750, y: 838891026 }, CirclePoint { x: 1421883301, y: 1767600112 },
    CirclePoint { x: 263253631, y: 1948074535 }, CirclePoint { x: 1984370174, y: 548484416 },
    CirclePoint { x: 377761958, y: 572545120 }, CirclePoint { x: 528497962, y: 2048433423 },
    CirclePoint { x: 347749121, y: 1576860820 }, CirclePoint { x: 1168068815, y: 1631133982 },
    CirclePoint { x: 1543902459, y: 1632329423 }, CirclePoint { x: 1964662320, y: 1382435771 },
    CirclePoint { x: 124482577, y: 226221110 }, CirclePoint { x: 1214106985, y: 780900129 },
    CirclePoint { x: 1635360495, y: 1075839836 }, CirclePoint { x: 36975016, y: 821963373 },
    CirclePoint { x: 338018326, y: 944583937 }, CirclePoint { x: 1753543609, y: 1294275673 },
    CirclePoint { x: 13191618, y: 303576968 }, CirclePoint { x: 606906923, y: 1264738384 },
    CirclePoint { x: 1488671228, y: 939868393 }, CirclePoint { x: 854984042, y: 63484416 },
    CirclePoint { x: 1825664712, y: 501661179 }, CirclePoint { x: 845078094, y: 149898124 },
    CirclePoint { x: 1829052821, y: 673049321 }, CirclePoint { x: 1029759507, y: 977441374 },
    CirclePoint { x: 1330239767, y: 1446369578 }, CirclePoint { x: 798338207, y: 2110850421 },
    CirclePoint { x: 521340221, y: 1824173925 }, CirclePoint { x: 957870942, y: 905094216 },
    CirclePoint { x: 1384581133, y: 347627210 }, CirclePoint { x: 2001642678, y: 1520422911 },
    CirclePoint { x: 940196297, y: 1361581394 }, CirclePoint { x: 1924483244, y: 1528988896 },
    CirclePoint { x: 296310565, y: 109983397 }, CirclePoint { x: 729011542, y: 952009042 },
    CirclePoint { x: 735504938, y: 198004457 }, CirclePoint { x: 591304555, y: 68989614 },
    CirclePoint { x: 324121113, y: 147452506 }, CirclePoint { x: 1029245122, y: 1158447597 },
    CirclePoint { x: 1213375404, y: 771547637 }, CirclePoint { x: 1389630114, y: 73256692 },
    CirclePoint { x: 1338697498, y: 1881711368 }, CirclePoint { x: 1631496031, y: 481893475 },
    CirclePoint { x: 607954189, y: 1091734815 }, CirclePoint { x: 891287480, y: 1172290213 },
    CirclePoint { x: 1004021208, y: 1870425365 }, CirclePoint { x: 1876139646, y: 119923533 },
    CirclePoint { x: 1212070390, y: 338345678 }, CirclePoint { x: 122809504, y: 181977862 },
    CirclePoint { x: 1577483195, y: 846841358 }, CirclePoint { x: 953139263, y: 265503960 },
    CirclePoint { x: 474711937, y: 1020011657 }, CirclePoint { x: 1696657789, y: 2121958423 },
    CirclePoint { x: 640220223, y: 1154724018 }, CirclePoint { x: 887385898, y: 608769478 },
    CirclePoint { x: 44588143, y: 2058311453 }, CirclePoint { x: 404074859, y: 398369746 },
];

/// Index `i` stores `M31_CIRCLE_GEN * i * 2^18`.
pub const M31_CIRCLE_GEN_MUL_TABLE_BITS_18_TO_23: [CirclePoint<M31BoundedInt>; 64] = [
    CirclePoint { x: 1, y: 0 }, CirclePoint { x: 1420207432, y: 2023238517 },
    CirclePoint { x: 2015554631, y: 1088093947 }, CirclePoint { x: 1720479643, y: 177413607 },
    CirclePoint { x: 996212859, y: 1140996376 }, CirclePoint { x: 816861480, y: 962195439 },
    CirclePoint { x: 816867225, y: 1001272228 }, CirclePoint { x: 451378731, y: 1322552846 },
    CirclePoint { x: 1434706457, y: 1835793811 }, CirclePoint { x: 306425233, y: 1445476898 },
    CirclePoint { x: 108989434, y: 171027633 }, CirclePoint { x: 2071133460, y: 426561857 },
    CirclePoint { x: 497251457, y: 850319468 }, CirclePoint { x: 1261210341, y: 453089266 },
    CirclePoint { x: 1658851911, y: 228916482 }, CirclePoint { x: 2034814864, y: 1083531373 },
    CirclePoint { x: 13610297, y: 1064696601 }, CirclePoint { x: 1070779035, y: 1741827866 },
    CirclePoint { x: 1818030451, y: 1183201954 }, CirclePoint { x: 741042664, y: 44265284 },
    CirclePoint { x: 903851705, y: 1384342623 }, CirclePoint { x: 1074738377, y: 922667469 },
    CirclePoint { x: 505405055, y: 327810224 }, CirclePoint { x: 1396305186, y: 1953641782 },
    CirclePoint { x: 609228044, y: 827893883 }, CirclePoint { x: 814418619, y: 258209920 },
    CirclePoint { x: 1215555130, y: 798189752 }, CirclePoint { x: 2054815690, y: 101808551 },
    CirclePoint { x: 1272801243, y: 632096738 }, CirclePoint { x: 1716313062, y: 1918467562 },
    CirclePoint { x: 2020345262, y: 2075173999 }, CirclePoint { x: 1119352987, y: 277348337 },
    CirclePoint { x: 785043271, y: 1260750973 }, CirclePoint { x: 612772931, y: 67557103 },
    CirclePoint { x: 1626319062, y: 1695214127 }, CirclePoint { x: 933993899, y: 1989600800 },
    CirclePoint { x: 608020784, y: 1658232706 }, CirclePoint { x: 1095416042, y: 2064872520 },
    CirclePoint { x: 503059345, y: 472488427 }, CirclePoint { x: 1030627222, y: 1992293517 },
    CirclePoint { x: 477465227, y: 1464821634 }, CirclePoint { x: 1395612878, y: 23814109 },
    CirclePoint { x: 889125072, y: 992347850 }, CirclePoint { x: 483482869, y: 1961673230 },
    CirclePoint { x: 1584592502, y: 821845974 }, CirclePoint { x: 1132606208, y: 1726336308 },
    CirclePoint { x: 1842504837, y: 844472455 }, CirclePoint { x: 1848107793, y: 1406793602 },
    CirclePoint { x: 655387905, y: 752064346 }, CirclePoint { x: 1888484876, y: 944217688 },
    CirclePoint { x: 870057820, y: 841184532 }, CirclePoint { x: 211324925, y: 1758927877 },
    CirclePoint { x: 1616257580, y: 975311156 }, CirclePoint { x: 1550009808, y: 1892920908 },
    CirclePoint { x: 326246026, y: 459902653 }, CirclePoint { x: 1943811611, y: 1441409520 },
    CirclePoint { x: 1919800332, y: 1732277387 }, CirclePoint { x: 1949082454, y: 963118347 },
    CirclePoint { x: 1315700354, y: 133468241 }, CirclePoint { x: 653789815, y: 1573078736 },
    CirclePoint { x: 947277864, y: 1089459781 }, CirclePoint { x: 494332398, y: 1387925761 },
    CirclePoint { x: 1536875846, y: 1146921693 }, CirclePoint { x: 1792250398, y: 609879011 },
];

/// Index `i` stores `M31_CIRCLE_GEN * i * 2^24`.
pub const M31_CIRCLE_GEN_MUL_TABLE_BITS_24_TO_29: [CirclePoint<M31BoundedInt>; 64] = [
    CirclePoint { x: 1, y: 0 }, CirclePoint { x: 838195206, y: 1774253895 },
    CirclePoint { x: 579625837, y: 1690787918 }, CirclePoint { x: 2013328190, y: 1108537731 },
    CirclePoint { x: 1179735656, y: 1241207368 }, CirclePoint { x: 2140339328, y: 404685994 },
    CirclePoint { x: 1866536500, y: 1133522282 }, CirclePoint { x: 1263730590, y: 350742286 },
    CirclePoint { x: 590768354, y: 978592373 }, CirclePoint { x: 206059115, y: 212443077 },
    CirclePoint { x: 1952787376, y: 1580223790 }, CirclePoint { x: 2079025011, y: 2137679949 },
    CirclePoint { x: 34602070, y: 732393395 }, CirclePoint { x: 14530030, y: 228509164 },
    CirclePoint { x: 26164677, y: 505542828 }, CirclePoint { x: 1885292596, y: 408478793 },
    CirclePoint { x: 32768, y: 2147450879 }, CirclePoint { x: 1739004854, y: 262191051 },
    CirclePoint { x: 1641940819, y: 2121318970 }, CirclePoint { x: 1918974483, y: 2132953617 },
    CirclePoint { x: 1415090252, y: 2112881577 }, CirclePoint { x: 9803698, y: 68458636 },
    CirclePoint { x: 567259857, y: 194696271 }, CirclePoint { x: 1935040570, y: 1941424532 },
    CirclePoint { x: 1168891274, y: 1556715293 }, CirclePoint { x: 1796741361, y: 883753057 },
    CirclePoint { x: 1013961365, y: 280947147 }, CirclePoint { x: 1742797653, y: 7144319 },
    CirclePoint { x: 906276279, y: 967747991 }, CirclePoint { x: 1038945916, y: 134155457 },
    CirclePoint { x: 456695729, y: 1567857810 }, CirclePoint { x: 373229752, y: 1309288441 },
    CirclePoint { x: 0, y: 2147483646 }, CirclePoint { x: 1774253895, y: 1309288441 },
    CirclePoint { x: 1690787918, y: 1567857810 }, CirclePoint { x: 1108537731, y: 134155457 },
    CirclePoint { x: 1241207368, y: 967747991 }, CirclePoint { x: 404685994, y: 7144319 },
    CirclePoint { x: 1133522282, y: 280947147 }, CirclePoint { x: 350742286, y: 883753057 },
    CirclePoint { x: 978592373, y: 1556715293 }, CirclePoint { x: 212443077, y: 1941424532 },
    CirclePoint { x: 1580223790, y: 194696271 }, CirclePoint { x: 2137679949, y: 68458636 },
    CirclePoint { x: 732393395, y: 2112881577 }, CirclePoint { x: 228509164, y: 2132953617 },
    CirclePoint { x: 505542828, y: 2121318970 }, CirclePoint { x: 408478793, y: 262191051 },
    CirclePoint { x: 2147450879, y: 2147450879 }, CirclePoint { x: 262191051, y: 408478793 },
    CirclePoint { x: 2121318970, y: 505542828 }, CirclePoint { x: 2132953617, y: 228509164 },
    CirclePoint { x: 2112881577, y: 732393395 }, CirclePoint { x: 68458636, y: 2137679949 },
    CirclePoint { x: 194696271, y: 1580223790 }, CirclePoint { x: 1941424532, y: 212443077 },
    CirclePoint { x: 1556715293, y: 978592373 }, CirclePoint { x: 883753057, y: 350742286 },
    CirclePoint { x: 280947147, y: 1133522282 }, CirclePoint { x: 7144319, y: 404685994 },
    CirclePoint { x: 967747991, y: 1241207368 }, CirclePoint { x: 134155457, y: 1108537731 },
    CirclePoint { x: 1567857810, y: 1690787918 }, CirclePoint { x: 1309288441, y: 1774253895 },
];

#[cfg(test)]
mod tests {
    use crate::circle::{CirclePointM31Impl, M31_CIRCLE_GEN};
    use super::{
        M31_CIRCLE_GEN_MUL_TABLE_BITS_0_TO_5, M31_CIRCLE_GEN_MUL_TABLE_BITS_12_TO_17,
        M31_CIRCLE_GEN_MUL_TABLE_BITS_18_TO_23, M31_CIRCLE_GEN_MUL_TABLE_BITS_24_TO_29,
        M31_CIRCLE_GEN_MUL_TABLE_BITS_6_TO_11,
    };

    #[test]
    fn test_constants_valid() {
        let step_1 = M31_CIRCLE_GEN;
        let mut acc = CirclePointM31Impl::zero();
        for p in M31_CIRCLE_GEN_MUL_TABLE_BITS_0_TO_5.span() {
            assert_eq!((*p).into(), acc);
            acc = acc + step_1.into();
        }

        let step_2_pow_6 = acc;
        let mut acc = CirclePointM31Impl::zero();
        for p in M31_CIRCLE_GEN_MUL_TABLE_BITS_6_TO_11.span() {
            assert_eq!((*p).into(), acc);
            acc = acc + step_2_pow_6.into();
        }

        let step_2_pow_12 = acc;
        let mut acc = CirclePointM31Impl::zero();
        for p in M31_CIRCLE_GEN_MUL_TABLE_BITS_12_TO_17.span() {
            assert_eq!((*p).into(), acc);
            acc = acc + step_2_pow_12.into();
        }

        let step_2_pow_18 = acc;
        let mut acc = CirclePointM31Impl::zero();
        for p in M31_CIRCLE_GEN_MUL_TABLE_BITS_18_TO_23.span() {
            assert_eq!((*p).into(), acc);
            acc = acc + step_2_pow_18.into();
        }

        let step_2_pow_24 = acc;
        let mut acc = CirclePointM31Impl::zero();
        for p in M31_CIRCLE_GEN_MUL_TABLE_BITS_24_TO_29.span() {
            assert_eq!((*p).into(), acc);
            acc = acc + step_2_pow_24.into();
        };
    }
}
