use stwo_prover::core::fields::qm31::QM31;

macro_rules! qm31 {
    ($m0:expr, $m1:expr, $m2:expr, $m3:expr) => {{
        QM31::from_u32_unchecked($m0, $m1, $m2, $m3)
    }};
}

pub const JNZ_OPCODE_TAKEN: QM31 = qm31!(1256383105, 2006201918, 765486323, 651355861);
pub const ADD_AP_OPCODE: QM31 = qm31!(1460514281, 281584274, 1454203206, 907116959);
pub const RANGE_CHECK_6: QM31 = qm31!(157343689, 1996779356, 1953195847, 1040522490);
pub const RANGE_CHECK_BUILTIN_BITS_128: QM31 = qm31!(162942568, 2060527078, 1843565602, 1617382771);
pub const VERIFY_INSTRUCTION: QM31 = qm31!(2113482325, 244639099, 958801514, 2052805714);
pub const ADD_OPCODE_IMM: QM31 = qm31!(411836725, 533311028, 2013255129, 642936946);
pub const ADD_OPCODE_SMALL_IMM: QM31 = qm31!(1978357203, 107244381, 506842868, 11952275);
pub const ADD_OPCODE_SMALL: QM31 = qm31!(1587549878, 504264399, 1192358411, 2109420200);
pub const ADD_OPCODE: QM31 = qm31!(818227840, 1829997736, 1837742466, 1751192896);
pub const ASSERT_EQ_OPCODE_DOUBLE_DEREF: QM31 =
    qm31!(1925096030, 1851573837, 277430168, 1121155226);
pub const ASSERT_EQ_OPCODE_IMM: QM31 = qm31!(1274804562, 1105793200, 578629990, 1204735693);
pub const ASSERT_EQ_OPCODE: QM31 = qm31!(644692727, 1252191971, 1844111828, 801630831);
pub const BLAKE_COMPRESS_OPCODE: QM31 = qm31!(1365434575, 2084558554, 103474294, 719397210);
pub const CALL_OPCODE_OP_1_BASE_FP: QM31 = qm31!(2066963646, 19095169, 785988351, 504505005);
pub const CALL_OPCODE_REL: QM31 = qm31!(1381906597, 814160643, 1383245686, 1243154220);
pub const CALL_OPCODE: QM31 = qm31!(2066963646, 19095169, 785988351, 504505005);
pub const GENERIC_OPCODE: QM31 = qm31!(792870104, 438067056, 1706034039, 2059359254);
pub const JNZ_OPCODE: QM31 = qm31!(1352028453, 447235709, 310070519, 69208790);
pub const JUMP_OPCODE_DOUBLE_DEREF: QM31 = qm31!(2101478830, 1630596249, 84122546, 308859785);
pub const JUMP_OPCODE_REL_IMM: QM31 = qm31!(349453824, 1866253793, 704705736, 462221098);
pub const JUMP_OPCODE_REL: QM31 = qm31!(790384557, 740590762, 1839176953, 613966341);
pub const JUMP_OPCODE: QM31 = qm31!(141780038, 165598605, 80878651, 973781448);
pub const MUL_OPCODE_IMM: QM31 = qm31!(1290518266, 1740571264, 494302837, 652552188);
pub const MUL_OPCODE_SMALL_IMM: QM31 = qm31!(443526177, 1398119028, 1045742085, 1941576278);
pub const MUL_OPCODE_SMALL: QM31 = qm31!(1629261365, 1410206860, 1055374917, 439519210);
pub const MUL_OPCODE: QM31 = qm31!(636784779, 1323630674, 1086718172, 2018690410);
pub const QM_31_ADD_MUL_OPCODE: QM31 = qm31!(403092206, 1178243798, 349886550, 1323494377);
pub const RET_OPCODE: QM31 = qm31!(2010170343, 1991181417, 62123824, 1982435275);
