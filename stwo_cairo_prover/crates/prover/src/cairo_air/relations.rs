#![allow(non_camel_case_types)]
use stwo_prover::relation;

relation!(Cube252, 20);
relation!(MemoryAddressToId, 2);
relation!(MemoryIdToBig, 29);
relation!(Opcodes, 3);
relation!(PoseidonRoundKeys, 31);
relation!(RangeCheck_6, 1);
relation!(RangeCheck_8, 1);
relation!(RangeCheck_11, 1);
relation!(RangeCheck_12, 1);
relation!(RangeCheck_18, 1);
relation!(RangeCheck_19, 1);
relation!(RangeCheck_3_6, 2);
relation!(RangeCheck_4_3, 2);
relation!(RangeCheck_4_4, 2);
relation!(RangeCheck_5_4, 2);
relation!(RangeCheck_9_9, 2);
relation!(RangeCheck_7_2_5, 3);
relation!(RangeCheck_3_6_6_3, 4);
relation!(RangeCheck_4_4_4_4, 4);
relation!(RangeCheck_3_3_3_3_3, 5);
relation!(RangeCheckFelt252Width27, 10);
relation!(VerifyInstruction, 29);
relation!(VerifyBitwiseXor_4, 3);
relation!(VerifyBitwiseXor_7, 3);
relation!(VerifyBitwiseXor_8, 3);
relation!(VerifyBitwiseXor_9, 3);
relation!(VerifyBitwiseXor_12, 3);
relation!(BlakeG, 20);
relation!(TripleXor32, 3);
relation!(BlakeRound, 35);
relation!(BlakeRoundSigma, 99);
relation!(PartialEcMul, 73); //
relation!(PedersenPointsTable, 57); //
