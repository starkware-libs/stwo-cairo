#![allow(non_camel_case_types)]
use stwo::core::fields::m31::M31;
use stwo_constraint_framework::relation;

pub const OPCODES_RELATION_ID: M31 = M31::from_u32_unchecked(428564188);

relation!(CommonLookupElements, 64);
