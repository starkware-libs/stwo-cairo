#![allow(non_camel_case_types)]
use stwo_prover::relation;

relation!(MemoryAddressToId, 2);
relation!(MemoryIdToBig, 29);
relation!(Opcodes, 3);
relation!(RangeCheck_6, 1);
relation!(RangeCheck_11, 1);
relation!(RangeCheck_12, 1);
relation!(RangeCheck_18, 1);
relation!(RangeCheck_19, 1);
relation!(RangeCheck_3_6, 2);
relation!(RangeCheck_4_3, 2);
relation!(RangeCheck_4_4, 2);
relation!(RangeCheck_9_9, 2);
relation!(RangeCheck_7_2_5, 3);
relation!(RangeCheck_3_6_6_3, 4);
relation!(RangeCheck_4_4_4_4, 4);
relation!(RangeCheck_3_3_3_3_3, 5);
relation!(TripleXor32, 3);
relation!(VerifyInstruction, 29);
relation!(VerifyBitwiseXor_4, 3);
relation!(VerifyBitwiseXor_7, 3);
relation!(VerifyBitwiseXor_8, 3);
relation!(VerifyBitwiseXor_9, 3);
relation!(VerifyBitwiseXor_12, 3);
