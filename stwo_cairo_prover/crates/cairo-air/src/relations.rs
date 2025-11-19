#![allow(non_camel_case_types)]
use stwo::core::fields::m31::M31;
use stwo_constraint_framework::relation;

pub const MEMORY_ADDRESS_TO_ID_RELATION_ID: M31 = M31::from_u32_unchecked(1444891767);
pub const MEMORY_ID_TO_BIG_RELATION_ID: M31 = M31::from_u32_unchecked(1662111297);
pub const OPCODES_RELATION_ID: M31 = M31::from_u32_unchecked(428564188);
pub const RANGE_CHECK_9_9_RELATION_ID: M31 = M31::from_u32_unchecked(517791011);
pub const RANGE_CHECK_9_9_B_RELATION_ID: M31 = M31::from_u32_unchecked(1897792095);
pub const RANGE_CHECK_9_9_C_RELATION_ID: M31 = M31::from_u32_unchecked(1881014476);
pub const RANGE_CHECK_9_9_D_RELATION_ID: M31 = M31::from_u32_unchecked(1864236857);
pub const RANGE_CHECK_9_9_E_RELATION_ID: M31 = M31::from_u32_unchecked(1847459238);
pub const RANGE_CHECK_9_9_F_RELATION_ID: M31 = M31::from_u32_unchecked(1830681619);
pub const RANGE_CHECK_9_9_G_RELATION_ID: M31 = M31::from_u32_unchecked(1813904000);
pub const RANGE_CHECK_9_9_H_RELATION_ID: M31 = M31::from_u32_unchecked(2065568285);
pub const VERIFY_BITWISE_XOR_12_RELATION_ID: M31 = M31::from_u32_unchecked(648362599);

relation!(CommonLookupElements, 128);
