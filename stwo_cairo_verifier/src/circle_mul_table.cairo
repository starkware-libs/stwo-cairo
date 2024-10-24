use stwo_cairo_verifier::circle::CirclePoint;
use stwo_cairo_verifier::fields::m31::M31;

/// Index `i` stores `M31_CIRCLE_GEN * i`.
pub const M31_CIRCLE_GEN_MUL_TABLE_BITS_0_TO_5: [
    CirclePoint<M31>
    ; 64] = [
    CirclePoint { x: M31 { inner: 1 }, y: M31 { inner: 0 } },
    CirclePoint { x: M31 { inner: 2 }, y: M31 { inner: 1268011823 } },
    CirclePoint { x: M31 { inner: 7 }, y: M31 { inner: 777079998 } },
    CirclePoint { x: M31 { inner: 26 }, y: M31 { inner: 1840308169 } },
    CirclePoint { x: M31 { inner: 97 }, y: M31 { inner: 141701737 } },
    CirclePoint { x: M31 { inner: 362 }, y: M31 { inner: 873982426 } },
    CirclePoint { x: M31 { inner: 1351 }, y: M31 { inner: 1206744320 } },
    CirclePoint { x: M31 { inner: 5042 }, y: M31 { inner: 1805511207 } },
    CirclePoint { x: M31 { inner: 18817 }, y: M31 { inner: 1720333214 } },
    CirclePoint { x: M31 { inner: 70226 }, y: M31 { inner: 780854355 } },
    CirclePoint { x: M31 { inner: 262087 }, y: M31 { inner: 1403084206 } },
    CirclePoint { x: M31 { inner: 978122 }, y: M31 { inner: 536515175 } },
    CirclePoint { x: M31 { inner: 3650401 }, y: M31 { inner: 742976494 } },
    CirclePoint { x: M31 { inner: 13623482 }, y: M31 { inner: 287907154 } },
    CirclePoint { x: M31 { inner: 50843527 }, y: M31 { inner: 408652122 } },
    CirclePoint { x: M31 { inner: 189750626 }, y: M31 { inner: 1346701334 } },
    CirclePoint { x: M31 { inner: 708158977 }, y: M31 { inner: 683185920 } },
    CirclePoint { x: M31 { inner: 495401635 }, y: M31 { inner: 1386042346 } },
    CirclePoint { x: M31 { inner: 1273447563 }, y: M31 { inner: 566016170 } },
    CirclePoint { x: M31 { inner: 303421323 }, y: M31 { inner: 878022334 } },
    CirclePoint { x: M31 { inner: 2087721376 }, y: M31 { inner: 798589519 } },
    CirclePoint { x: M31 { inner: 1605013240 }, y: M31 { inner: 168852095 } },
    CirclePoint { x: M31 { inner: 37364290 }, y: M31 { inner: 2024302508 } },
    CirclePoint { x: M31 { inner: 691927567 }, y: M31 { inner: 1485906996 } },
    CirclePoint { x: M31 { inner: 582862331 }, y: M31 { inner: 1771841829 } },
    CirclePoint { x: M31 { inner: 1639521757 }, y: M31 { inner: 1306493026 } },
    CirclePoint { x: M31 { inner: 1680257403 }, y: M31 { inner: 1306646628 } },
    CirclePoint { x: M31 { inner: 786540561 }, y: M31 { inner: 1772609839 } },
    CirclePoint { x: M31 { inner: 1465904841 }, y: M31 { inner: 1488825434 } },
    CirclePoint { x: M31 { inner: 782111509 }, y: M31 { inner: 2035208250 } },
    CirclePoint { x: M31 { inner: 1662541195 }, y: M31 { inner: 209556625 } },
    CirclePoint { x: M31 { inner: 1573085977 }, y: M31 { inner: 950501897 } },
    CirclePoint { x: M31 { inner: 334835419 }, y: M31 { inner: 1444967316 } },
    CirclePoint { x: M31 { inner: 1913739346 }, y: M31 { inner: 534400073 } },
    CirclePoint { x: M31 { inner: 877671024 }, y: M31 { inner: 692632976 } },
    CirclePoint { x: M31 { inner: 1596944750 }, y: M31 { inner: 88648184 } },
    CirclePoint { x: M31 { inner: 1215140682 }, y: M31 { inner: 1809443407 } },
    CirclePoint { x: M31 { inner: 1116134331 }, y: M31 { inner: 706674503 } },
    CirclePoint { x: M31 { inner: 1101912995 }, y: M31 { inner: 1017254605 } },
    CirclePoint { x: M31 { inner: 1144034002 }, y: M31 { inner: 1214860270 } },
    CirclePoint { x: M31 { inner: 1326739366 }, y: M31 { inner: 1694702828 } },
    CirclePoint { x: M31 { inner: 2015439815 }, y: M31 { inner: 1268983748 } },
    CirclePoint { x: M31 { inner: 292568953 }, y: M31 { inner: 1233748517 } },
    CirclePoint { x: M31 { inner: 1302319644 }, y: M31 { inner: 1518526673 } },
    CirclePoint { x: M31 { inner: 621742329 }, y: M31 { inner: 545390881 } },
    CirclePoint { x: M31 { inner: 1184649672 }, y: M31 { inner: 663036851 } },
    CirclePoint { x: M31 { inner: 1969372712 }, y: M31 { inner: 2106756523 } },
    CirclePoint { x: M31 { inner: 250390235 }, y: M31 { inner: 1321538300 } },
    CirclePoint { x: M31 { inner: 1179671875 }, y: M31 { inner: 1031913030 } },
    CirclePoint { x: M31 { inner: 173329971 }, y: M31 { inner: 658630173 } },
    CirclePoint { x: M31 { inner: 1661131656 }, y: M31 { inner: 1602607662 } },
    CirclePoint { x: M31 { inner: 28745712 }, y: M31 { inner: 1456833181 } },
    CirclePoint { x: M31 { inner: 601334839 }, y: M31 { inner: 2077241415 } },
    CirclePoint { x: M31 { inner: 229109997 }, y: M31 { inner: 409681538 } },
    CirclePoint { x: M31 { inner: 315105149 }, y: M31 { inner: 1708968384 } },
    CirclePoint { x: M31 { inner: 1031310599 }, y: M31 { inner: 2131224704 } },
    CirclePoint { x: M31 { inner: 1662653600 }, y: M31 { inner: 373479491 } },
    CirclePoint { x: M31 { inner: 1324336507 }, y: M31 { inner: 1510176907 } },
    CirclePoint { x: M31 { inner: 1487208781 }, y: M31 { inner: 1372260843 } },
    CirclePoint { x: M31 { inner: 329531323 }, y: M31 { inner: 1831382818 } },
    CirclePoint { x: M31 { inner: 1978400158 }, y: M31 { inner: 1658303135 } },
    CirclePoint { x: M31 { inner: 1141618368 }, y: M31 { inner: 506862428 } },
    CirclePoint { x: M31 { inner: 440589667 }, y: M31 { inner: 369146577 } },
    CirclePoint { x: M31 { inner: 620740300 }, y: M31 { inner: 969723880 } },
];

/// Index `i` stores `M31_CIRCLE_GEN * i * 2^6`.
pub const M31_CIRCLE_GEN_MUL_TABLE_BITS_6_TO_11: [
    CirclePoint<M31>
    ; 64] = [
    CirclePoint { x: M31 { inner: 1 }, y: M31 { inner: 0 } },
    CirclePoint { x: M31 { inner: 2042371533 }, y: M31 { inner: 1362265296 } },
    CirclePoint { x: M31 { inner: 212706801 }, y: M31 { inner: 1223819887 } },
    CirclePoint { x: M31 { inner: 1343082982 }, y: M31 { inner: 1656388718 } },
    CirclePoint { x: M31 { inner: 421007138 }, y: M31 { inner: 256177860 } },
    CirclePoint { x: M31 { inner: 1637047685 }, y: M31 { inner: 109864373 } },
    CirclePoint { x: M31 { inner: 1594705154 }, y: M31 { inner: 1156423391 } },
    CirclePoint { x: M31 { inner: 1326191972 }, y: M31 { inner: 1554410474 } },
    CirclePoint { x: M31 { inner: 6346213 }, y: M31 { inner: 905523693 } },
    CirclePoint { x: M31 { inner: 1603309155 }, y: M31 { inner: 1317339952 } },
    CirclePoint { x: M31 { inner: 982766847 }, y: M31 { inner: 1369509443 } },
    CirclePoint { x: M31 { inner: 1130890301 }, y: M31 { inner: 1314091057 } },
    CirclePoint { x: M31 { inner: 784830374 }, y: M31 { inner: 912241638 } },
    CirclePoint { x: M31 { inner: 529167155 }, y: M31 { inner: 49384636 } },
    CirclePoint { x: M31 { inner: 1475671350 }, y: M31 { inner: 739745329 } },
    CirclePoint { x: M31 { inner: 418523497 }, y: M31 { inner: 1921984699 } },
    CirclePoint { x: M31 { inner: 1022251061 }, y: M31 { inner: 788094511 } },
    CirclePoint { x: M31 { inner: 1453585800 }, y: M31 { inner: 1686756607 } },
    CirclePoint { x: M31 { inner: 2144756169 }, y: M31 { inner: 1973977555 } },
    CirclePoint { x: M31 { inner: 222118537 }, y: M31 { inner: 1014253609 } },
    CirclePoint { x: M31 { inner: 1774187400 }, y: M31 { inner: 1713497086 } },
    CirclePoint { x: M31 { inner: 1447121614 }, y: M31 { inner: 1053732556 } },
    CirclePoint { x: M31 { inner: 105012991 }, y: M31 { inner: 261334420 } },
    CirclePoint { x: M31 { inner: 1509980434 }, y: M31 { inner: 1580004926 } },
    CirclePoint { x: M31 { inner: 999230472 }, y: M31 { inner: 1852479283 } },
    CirclePoint { x: M31 { inner: 517742188 }, y: M31 { inner: 968413651 } },
    CirclePoint { x: M31 { inner: 288424812 }, y: M31 { inner: 1959942454 } },
    CirclePoint { x: M31 { inner: 1136461567 }, y: M31 { inner: 2028297119 } },
    CirclePoint { x: M31 { inner: 344093444 }, y: M31 { inner: 1723950257 } },
    CirclePoint { x: M31 { inner: 2162587 }, y: M31 { inner: 1662102570 } },
    CirclePoint { x: M31 { inner: 203869561 }, y: M31 { inner: 313423390 } },
    CirclePoint { x: M31 { inner: 57053772 }, y: M31 { inner: 179506272 } },
    CirclePoint { x: M31 { inner: 1633461177 }, y: M31 { inner: 574296567 } },
    CirclePoint { x: M31 { inner: 2063130879 }, y: M31 { inner: 1272838818 } },
    CirclePoint { x: M31 { inner: 1655011431 }, y: M31 { inner: 2085598907 } },
    CirclePoint { x: M31 { inner: 1882985882 }, y: M31 { inner: 1884234849 } },
    CirclePoint { x: M31 { inner: 505774551 }, y: M31 { inner: 1187398701 } },
    CirclePoint { x: M31 { inner: 39852141 }, y: M31 { inner: 123936243 } },
    CirclePoint { x: M31 { inner: 1676081227 }, y: M31 { inner: 1135991864 } },
    CirclePoint { x: M31 { inner: 603512862 }, y: M31 { inner: 13573590 } },
    CirclePoint { x: M31 { inner: 1743980363 }, y: M31 { inner: 345791924 } },
    CirclePoint { x: M31 { inner: 1221120023 }, y: M31 { inner: 1584263814 } },
    CirclePoint { x: M31 { inner: 319360190 }, y: M31 { inner: 618255967 } },
    CirclePoint { x: M31 { inner: 937276350 }, y: M31 { inner: 1889866233 } },
    CirclePoint { x: M31 { inner: 638925477 }, y: M31 { inner: 1200786367 } },
    CirclePoint { x: M31 { inner: 2050207099 }, y: M31 { inner: 979596272 } },
    CirclePoint { x: M31 { inner: 1091919627 }, y: M31 { inner: 1535715623 } },
    CirclePoint { x: M31 { inner: 327335145 }, y: M31 { inner: 1879026140 } },
    CirclePoint { x: M31 { inner: 2054061671 }, y: M31 { inner: 949045631 } },
    CirclePoint { x: M31 { inner: 2017447582 }, y: M31 { inner: 2138555804 } },
    CirclePoint { x: M31 { inner: 1230785717 }, y: M31 { inner: 932947748 } },
    CirclePoint { x: M31 { inner: 1045835668 }, y: M31 { inner: 1112919339 } },
    CirclePoint { x: M31 { inner: 1399167609 }, y: M31 { inner: 1906303117 } },
    CirclePoint { x: M31 { inner: 968612238 }, y: M31 { inner: 1641022910 } },
    CirclePoint { x: M31 { inner: 1696585972 }, y: M31 { inner: 468920631 } },
    CirclePoint { x: M31 { inner: 19290124 }, y: M31 { inner: 887640976 } },
    CirclePoint { x: M31 { inner: 1309393381 }, y: M31 { inner: 53742718 } },
    CirclePoint { x: M31 { inner: 2120725548 }, y: M31 { inner: 846203964 } },
    CirclePoint { x: M31 { inner: 1273782452 }, y: M31 { inner: 924138332 } },
    CirclePoint { x: M31 { inner: 1759380646 }, y: M31 { inner: 1693051226 } },
    CirclePoint { x: M31 { inner: 2080156404 }, y: M31 { inner: 1988869649 } },
    CirclePoint { x: M31 { inner: 314388810 }, y: M31 { inner: 1013513450 } },
    CirclePoint { x: M31 { inner: 1619251001 }, y: M31 { inner: 830366565 } },
    CirclePoint { x: M31 { inner: 1385987562 }, y: M31 { inner: 2130936594 } },
];

/// Index `i` stores `M31_CIRCLE_GEN * i * 2^12`.
pub const M31_CIRCLE_GEN_MUL_TABLE_BITS_12_TO_17: [
    CirclePoint<M31>
    ; 64] = [
    CirclePoint { x: M31 { inner: 1 }, y: M31 { inner: 0 } },
    CirclePoint { x: M31 { inner: 595037635 }, y: M31 { inner: 2111542451 } },
    CirclePoint { x: M31 { inner: 1799120754 }, y: M31 { inner: 343598868 } },
    CirclePoint { x: M31 { inner: 1475016787 }, y: M31 { inner: 1876816901 } },
    CirclePoint { x: M31 { inner: 438833264 }, y: M31 { inner: 1327019128 } },
    CirclePoint { x: M31 { inner: 433600440 }, y: M31 { inner: 167145119 } },
    CirclePoint { x: M31 { inner: 503318431 }, y: M31 { inner: 730224775 } },
    CirclePoint { x: M31 { inner: 1124768046 }, y: M31 { inner: 299494314 } },
    CirclePoint { x: M31 { inner: 1389168750 }, y: M31 { inner: 838891026 } },
    CirclePoint { x: M31 { inner: 1421883301 }, y: M31 { inner: 1767600112 } },
    CirclePoint { x: M31 { inner: 263253631 }, y: M31 { inner: 1948074535 } },
    CirclePoint { x: M31 { inner: 1984370174 }, y: M31 { inner: 548484416 } },
    CirclePoint { x: M31 { inner: 377761958 }, y: M31 { inner: 572545120 } },
    CirclePoint { x: M31 { inner: 528497962 }, y: M31 { inner: 2048433423 } },
    CirclePoint { x: M31 { inner: 347749121 }, y: M31 { inner: 1576860820 } },
    CirclePoint { x: M31 { inner: 1168068815 }, y: M31 { inner: 1631133982 } },
    CirclePoint { x: M31 { inner: 1543902459 }, y: M31 { inner: 1632329423 } },
    CirclePoint { x: M31 { inner: 1964662320 }, y: M31 { inner: 1382435771 } },
    CirclePoint { x: M31 { inner: 124482577 }, y: M31 { inner: 226221110 } },
    CirclePoint { x: M31 { inner: 1214106985 }, y: M31 { inner: 780900129 } },
    CirclePoint { x: M31 { inner: 1635360495 }, y: M31 { inner: 1075839836 } },
    CirclePoint { x: M31 { inner: 36975016 }, y: M31 { inner: 821963373 } },
    CirclePoint { x: M31 { inner: 338018326 }, y: M31 { inner: 944583937 } },
    CirclePoint { x: M31 { inner: 1753543609 }, y: M31 { inner: 1294275673 } },
    CirclePoint { x: M31 { inner: 13191618 }, y: M31 { inner: 303576968 } },
    CirclePoint { x: M31 { inner: 606906923 }, y: M31 { inner: 1264738384 } },
    CirclePoint { x: M31 { inner: 1488671228 }, y: M31 { inner: 939868393 } },
    CirclePoint { x: M31 { inner: 854984042 }, y: M31 { inner: 63484416 } },
    CirclePoint { x: M31 { inner: 1825664712 }, y: M31 { inner: 501661179 } },
    CirclePoint { x: M31 { inner: 845078094 }, y: M31 { inner: 149898124 } },
    CirclePoint { x: M31 { inner: 1829052821 }, y: M31 { inner: 673049321 } },
    CirclePoint { x: M31 { inner: 1029759507 }, y: M31 { inner: 977441374 } },
    CirclePoint { x: M31 { inner: 1330239767 }, y: M31 { inner: 1446369578 } },
    CirclePoint { x: M31 { inner: 798338207 }, y: M31 { inner: 2110850421 } },
    CirclePoint { x: M31 { inner: 521340221 }, y: M31 { inner: 1824173925 } },
    CirclePoint { x: M31 { inner: 957870942 }, y: M31 { inner: 905094216 } },
    CirclePoint { x: M31 { inner: 1384581133 }, y: M31 { inner: 347627210 } },
    CirclePoint { x: M31 { inner: 2001642678 }, y: M31 { inner: 1520422911 } },
    CirclePoint { x: M31 { inner: 940196297 }, y: M31 { inner: 1361581394 } },
    CirclePoint { x: M31 { inner: 1924483244 }, y: M31 { inner: 1528988896 } },
    CirclePoint { x: M31 { inner: 296310565 }, y: M31 { inner: 109983397 } },
    CirclePoint { x: M31 { inner: 729011542 }, y: M31 { inner: 952009042 } },
    CirclePoint { x: M31 { inner: 735504938 }, y: M31 { inner: 198004457 } },
    CirclePoint { x: M31 { inner: 591304555 }, y: M31 { inner: 68989614 } },
    CirclePoint { x: M31 { inner: 324121113 }, y: M31 { inner: 147452506 } },
    CirclePoint { x: M31 { inner: 1029245122 }, y: M31 { inner: 1158447597 } },
    CirclePoint { x: M31 { inner: 1213375404 }, y: M31 { inner: 771547637 } },
    CirclePoint { x: M31 { inner: 1389630114 }, y: M31 { inner: 73256692 } },
    CirclePoint { x: M31 { inner: 1338697498 }, y: M31 { inner: 1881711368 } },
    CirclePoint { x: M31 { inner: 1631496031 }, y: M31 { inner: 481893475 } },
    CirclePoint { x: M31 { inner: 607954189 }, y: M31 { inner: 1091734815 } },
    CirclePoint { x: M31 { inner: 891287480 }, y: M31 { inner: 1172290213 } },
    CirclePoint { x: M31 { inner: 1004021208 }, y: M31 { inner: 1870425365 } },
    CirclePoint { x: M31 { inner: 1876139646 }, y: M31 { inner: 119923533 } },
    CirclePoint { x: M31 { inner: 1212070390 }, y: M31 { inner: 338345678 } },
    CirclePoint { x: M31 { inner: 122809504 }, y: M31 { inner: 181977862 } },
    CirclePoint { x: M31 { inner: 1577483195 }, y: M31 { inner: 846841358 } },
    CirclePoint { x: M31 { inner: 953139263 }, y: M31 { inner: 265503960 } },
    CirclePoint { x: M31 { inner: 474711937 }, y: M31 { inner: 1020011657 } },
    CirclePoint { x: M31 { inner: 1696657789 }, y: M31 { inner: 2121958423 } },
    CirclePoint { x: M31 { inner: 640220223 }, y: M31 { inner: 1154724018 } },
    CirclePoint { x: M31 { inner: 887385898 }, y: M31 { inner: 608769478 } },
    CirclePoint { x: M31 { inner: 44588143 }, y: M31 { inner: 2058311453 } },
    CirclePoint { x: M31 { inner: 404074859 }, y: M31 { inner: 398369746 } },
];

/// Index `i` stores `M31_CIRCLE_GEN * i * 2^18`.
pub const M31_CIRCLE_GEN_MUL_TABLE_BITS_18_TO_23: [
    CirclePoint<M31>
    ; 64] = [
    CirclePoint { x: M31 { inner: 1 }, y: M31 { inner: 0 } },
    CirclePoint { x: M31 { inner: 1420207432 }, y: M31 { inner: 2023238517 } },
    CirclePoint { x: M31 { inner: 2015554631 }, y: M31 { inner: 1088093947 } },
    CirclePoint { x: M31 { inner: 1720479643 }, y: M31 { inner: 177413607 } },
    CirclePoint { x: M31 { inner: 996212859 }, y: M31 { inner: 1140996376 } },
    CirclePoint { x: M31 { inner: 816861480 }, y: M31 { inner: 962195439 } },
    CirclePoint { x: M31 { inner: 816867225 }, y: M31 { inner: 1001272228 } },
    CirclePoint { x: M31 { inner: 451378731 }, y: M31 { inner: 1322552846 } },
    CirclePoint { x: M31 { inner: 1434706457 }, y: M31 { inner: 1835793811 } },
    CirclePoint { x: M31 { inner: 306425233 }, y: M31 { inner: 1445476898 } },
    CirclePoint { x: M31 { inner: 108989434 }, y: M31 { inner: 171027633 } },
    CirclePoint { x: M31 { inner: 2071133460 }, y: M31 { inner: 426561857 } },
    CirclePoint { x: M31 { inner: 497251457 }, y: M31 { inner: 850319468 } },
    CirclePoint { x: M31 { inner: 1261210341 }, y: M31 { inner: 453089266 } },
    CirclePoint { x: M31 { inner: 1658851911 }, y: M31 { inner: 228916482 } },
    CirclePoint { x: M31 { inner: 2034814864 }, y: M31 { inner: 1083531373 } },
    CirclePoint { x: M31 { inner: 13610297 }, y: M31 { inner: 1064696601 } },
    CirclePoint { x: M31 { inner: 1070779035 }, y: M31 { inner: 1741827866 } },
    CirclePoint { x: M31 { inner: 1818030451 }, y: M31 { inner: 1183201954 } },
    CirclePoint { x: M31 { inner: 741042664 }, y: M31 { inner: 44265284 } },
    CirclePoint { x: M31 { inner: 903851705 }, y: M31 { inner: 1384342623 } },
    CirclePoint { x: M31 { inner: 1074738377 }, y: M31 { inner: 922667469 } },
    CirclePoint { x: M31 { inner: 505405055 }, y: M31 { inner: 327810224 } },
    CirclePoint { x: M31 { inner: 1396305186 }, y: M31 { inner: 1953641782 } },
    CirclePoint { x: M31 { inner: 609228044 }, y: M31 { inner: 827893883 } },
    CirclePoint { x: M31 { inner: 814418619 }, y: M31 { inner: 258209920 } },
    CirclePoint { x: M31 { inner: 1215555130 }, y: M31 { inner: 798189752 } },
    CirclePoint { x: M31 { inner: 2054815690 }, y: M31 { inner: 101808551 } },
    CirclePoint { x: M31 { inner: 1272801243 }, y: M31 { inner: 632096738 } },
    CirclePoint { x: M31 { inner: 1716313062 }, y: M31 { inner: 1918467562 } },
    CirclePoint { x: M31 { inner: 2020345262 }, y: M31 { inner: 2075173999 } },
    CirclePoint { x: M31 { inner: 1119352987 }, y: M31 { inner: 277348337 } },
    CirclePoint { x: M31 { inner: 785043271 }, y: M31 { inner: 1260750973 } },
    CirclePoint { x: M31 { inner: 612772931 }, y: M31 { inner: 67557103 } },
    CirclePoint { x: M31 { inner: 1626319062 }, y: M31 { inner: 1695214127 } },
    CirclePoint { x: M31 { inner: 933993899 }, y: M31 { inner: 1989600800 } },
    CirclePoint { x: M31 { inner: 608020784 }, y: M31 { inner: 1658232706 } },
    CirclePoint { x: M31 { inner: 1095416042 }, y: M31 { inner: 2064872520 } },
    CirclePoint { x: M31 { inner: 503059345 }, y: M31 { inner: 472488427 } },
    CirclePoint { x: M31 { inner: 1030627222 }, y: M31 { inner: 1992293517 } },
    CirclePoint { x: M31 { inner: 477465227 }, y: M31 { inner: 1464821634 } },
    CirclePoint { x: M31 { inner: 1395612878 }, y: M31 { inner: 23814109 } },
    CirclePoint { x: M31 { inner: 889125072 }, y: M31 { inner: 992347850 } },
    CirclePoint { x: M31 { inner: 483482869 }, y: M31 { inner: 1961673230 } },
    CirclePoint { x: M31 { inner: 1584592502 }, y: M31 { inner: 821845974 } },
    CirclePoint { x: M31 { inner: 1132606208 }, y: M31 { inner: 1726336308 } },
    CirclePoint { x: M31 { inner: 1842504837 }, y: M31 { inner: 844472455 } },
    CirclePoint { x: M31 { inner: 1848107793 }, y: M31 { inner: 1406793602 } },
    CirclePoint { x: M31 { inner: 655387905 }, y: M31 { inner: 752064346 } },
    CirclePoint { x: M31 { inner: 1888484876 }, y: M31 { inner: 944217688 } },
    CirclePoint { x: M31 { inner: 870057820 }, y: M31 { inner: 841184532 } },
    CirclePoint { x: M31 { inner: 211324925 }, y: M31 { inner: 1758927877 } },
    CirclePoint { x: M31 { inner: 1616257580 }, y: M31 { inner: 975311156 } },
    CirclePoint { x: M31 { inner: 1550009808 }, y: M31 { inner: 1892920908 } },
    CirclePoint { x: M31 { inner: 326246026 }, y: M31 { inner: 459902653 } },
    CirclePoint { x: M31 { inner: 1943811611 }, y: M31 { inner: 1441409520 } },
    CirclePoint { x: M31 { inner: 1919800332 }, y: M31 { inner: 1732277387 } },
    CirclePoint { x: M31 { inner: 1949082454 }, y: M31 { inner: 963118347 } },
    CirclePoint { x: M31 { inner: 1315700354 }, y: M31 { inner: 133468241 } },
    CirclePoint { x: M31 { inner: 653789815 }, y: M31 { inner: 1573078736 } },
    CirclePoint { x: M31 { inner: 947277864 }, y: M31 { inner: 1089459781 } },
    CirclePoint { x: M31 { inner: 494332398 }, y: M31 { inner: 1387925761 } },
    CirclePoint { x: M31 { inner: 1536875846 }, y: M31 { inner: 1146921693 } },
    CirclePoint { x: M31 { inner: 1792250398 }, y: M31 { inner: 609879011 } },
];

/// Index `i` stores `M31_CIRCLE_GEN * i * 2^24`.
pub const M31_CIRCLE_GEN_MUL_TABLE_BITS_24_TO_29: [
    CirclePoint<M31>
    ; 64] = [
    CirclePoint { x: M31 { inner: 1 }, y: M31 { inner: 0 } },
    CirclePoint { x: M31 { inner: 838195206 }, y: M31 { inner: 1774253895 } },
    CirclePoint { x: M31 { inner: 579625837 }, y: M31 { inner: 1690787918 } },
    CirclePoint { x: M31 { inner: 2013328190 }, y: M31 { inner: 1108537731 } },
    CirclePoint { x: M31 { inner: 1179735656 }, y: M31 { inner: 1241207368 } },
    CirclePoint { x: M31 { inner: 2140339328 }, y: M31 { inner: 404685994 } },
    CirclePoint { x: M31 { inner: 1866536500 }, y: M31 { inner: 1133522282 } },
    CirclePoint { x: M31 { inner: 1263730590 }, y: M31 { inner: 350742286 } },
    CirclePoint { x: M31 { inner: 590768354 }, y: M31 { inner: 978592373 } },
    CirclePoint { x: M31 { inner: 206059115 }, y: M31 { inner: 212443077 } },
    CirclePoint { x: M31 { inner: 1952787376 }, y: M31 { inner: 1580223790 } },
    CirclePoint { x: M31 { inner: 2079025011 }, y: M31 { inner: 2137679949 } },
    CirclePoint { x: M31 { inner: 34602070 }, y: M31 { inner: 732393395 } },
    CirclePoint { x: M31 { inner: 14530030 }, y: M31 { inner: 228509164 } },
    CirclePoint { x: M31 { inner: 26164677 }, y: M31 { inner: 505542828 } },
    CirclePoint { x: M31 { inner: 1885292596 }, y: M31 { inner: 408478793 } },
    CirclePoint { x: M31 { inner: 32768 }, y: M31 { inner: 2147450879 } },
    CirclePoint { x: M31 { inner: 1739004854 }, y: M31 { inner: 262191051 } },
    CirclePoint { x: M31 { inner: 1641940819 }, y: M31 { inner: 2121318970 } },
    CirclePoint { x: M31 { inner: 1918974483 }, y: M31 { inner: 2132953617 } },
    CirclePoint { x: M31 { inner: 1415090252 }, y: M31 { inner: 2112881577 } },
    CirclePoint { x: M31 { inner: 9803698 }, y: M31 { inner: 68458636 } },
    CirclePoint { x: M31 { inner: 567259857 }, y: M31 { inner: 194696271 } },
    CirclePoint { x: M31 { inner: 1935040570 }, y: M31 { inner: 1941424532 } },
    CirclePoint { x: M31 { inner: 1168891274 }, y: M31 { inner: 1556715293 } },
    CirclePoint { x: M31 { inner: 1796741361 }, y: M31 { inner: 883753057 } },
    CirclePoint { x: M31 { inner: 1013961365 }, y: M31 { inner: 280947147 } },
    CirclePoint { x: M31 { inner: 1742797653 }, y: M31 { inner: 7144319 } },
    CirclePoint { x: M31 { inner: 906276279 }, y: M31 { inner: 967747991 } },
    CirclePoint { x: M31 { inner: 1038945916 }, y: M31 { inner: 134155457 } },
    CirclePoint { x: M31 { inner: 456695729 }, y: M31 { inner: 1567857810 } },
    CirclePoint { x: M31 { inner: 373229752 }, y: M31 { inner: 1309288441 } },
    CirclePoint { x: M31 { inner: 0 }, y: M31 { inner: 2147483646 } },
    CirclePoint { x: M31 { inner: 1774253895 }, y: M31 { inner: 1309288441 } },
    CirclePoint { x: M31 { inner: 1690787918 }, y: M31 { inner: 1567857810 } },
    CirclePoint { x: M31 { inner: 1108537731 }, y: M31 { inner: 134155457 } },
    CirclePoint { x: M31 { inner: 1241207368 }, y: M31 { inner: 967747991 } },
    CirclePoint { x: M31 { inner: 404685994 }, y: M31 { inner: 7144319 } },
    CirclePoint { x: M31 { inner: 1133522282 }, y: M31 { inner: 280947147 } },
    CirclePoint { x: M31 { inner: 350742286 }, y: M31 { inner: 883753057 } },
    CirclePoint { x: M31 { inner: 978592373 }, y: M31 { inner: 1556715293 } },
    CirclePoint { x: M31 { inner: 212443077 }, y: M31 { inner: 1941424532 } },
    CirclePoint { x: M31 { inner: 1580223790 }, y: M31 { inner: 194696271 } },
    CirclePoint { x: M31 { inner: 2137679949 }, y: M31 { inner: 68458636 } },
    CirclePoint { x: M31 { inner: 732393395 }, y: M31 { inner: 2112881577 } },
    CirclePoint { x: M31 { inner: 228509164 }, y: M31 { inner: 2132953617 } },
    CirclePoint { x: M31 { inner: 505542828 }, y: M31 { inner: 2121318970 } },
    CirclePoint { x: M31 { inner: 408478793 }, y: M31 { inner: 262191051 } },
    CirclePoint { x: M31 { inner: 2147450879 }, y: M31 { inner: 2147450879 } },
    CirclePoint { x: M31 { inner: 262191051 }, y: M31 { inner: 408478793 } },
    CirclePoint { x: M31 { inner: 2121318970 }, y: M31 { inner: 505542828 } },
    CirclePoint { x: M31 { inner: 2132953617 }, y: M31 { inner: 228509164 } },
    CirclePoint { x: M31 { inner: 2112881577 }, y: M31 { inner: 732393395 } },
    CirclePoint { x: M31 { inner: 68458636 }, y: M31 { inner: 2137679949 } },
    CirclePoint { x: M31 { inner: 194696271 }, y: M31 { inner: 1580223790 } },
    CirclePoint { x: M31 { inner: 1941424532 }, y: M31 { inner: 212443077 } },
    CirclePoint { x: M31 { inner: 1556715293 }, y: M31 { inner: 978592373 } },
    CirclePoint { x: M31 { inner: 883753057 }, y: M31 { inner: 350742286 } },
    CirclePoint { x: M31 { inner: 280947147 }, y: M31 { inner: 1133522282 } },
    CirclePoint { x: M31 { inner: 7144319 }, y: M31 { inner: 404685994 } },
    CirclePoint { x: M31 { inner: 967747991 }, y: M31 { inner: 1241207368 } },
    CirclePoint { x: M31 { inner: 134155457 }, y: M31 { inner: 1108537731 } },
    CirclePoint { x: M31 { inner: 1567857810 }, y: M31 { inner: 1690787918 } },
    CirclePoint { x: M31 { inner: 1309288441 }, y: M31 { inner: 1774253895 } },
];

#[cfg(test)]
mod tests {
    use stwo_cairo_verifier::circle::{M31_CIRCLE_GEN, CirclePointM31Impl};
    use super::{
        M31_CIRCLE_GEN_MUL_TABLE_BITS_0_TO_5, M31_CIRCLE_GEN_MUL_TABLE_BITS_6_TO_11,
        M31_CIRCLE_GEN_MUL_TABLE_BITS_12_TO_17, M31_CIRCLE_GEN_MUL_TABLE_BITS_18_TO_23,
        M31_CIRCLE_GEN_MUL_TABLE_BITS_24_TO_29,
    };

    #[test]
    fn test_constants_valid() {
        let step_1 = M31_CIRCLE_GEN;
        let mut acc = CirclePointM31Impl::zero();
        for p in M31_CIRCLE_GEN_MUL_TABLE_BITS_0_TO_5
            .span() {
                assert_eq!(*p, acc);
                acc = acc + step_1;
            };

        let step_2_pow_6 = acc;
        let mut acc = CirclePointM31Impl::zero();
        for p in M31_CIRCLE_GEN_MUL_TABLE_BITS_6_TO_11
            .span() {
                assert_eq!(*p, acc);
                acc = acc + step_2_pow_6;
            };

        let step_2_pow_12 = acc;
        let mut acc = CirclePointM31Impl::zero();
        for p in M31_CIRCLE_GEN_MUL_TABLE_BITS_12_TO_17
            .span() {
                assert_eq!(*p, acc);
                acc = acc + step_2_pow_12;
            };

        let step_2_pow_18 = acc;
        let mut acc = CirclePointM31Impl::zero();
        for p in M31_CIRCLE_GEN_MUL_TABLE_BITS_18_TO_23
            .span() {
                assert_eq!(*p, acc);
                acc = acc + step_2_pow_18;
            };

        let step_2_pow_24 = acc;
        let mut acc = CirclePointM31Impl::zero();
        for p in M31_CIRCLE_GEN_MUL_TABLE_BITS_24_TO_29
            .span() {
                assert_eq!(*p, acc);
                acc = acc + step_2_pow_24;
            };
    }
}
