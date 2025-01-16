# Diff Summary

Date : 2025-01-23 11:41:09

Directory /home/alonf/workspace/stwo-cairo/stwo_cairo_verifier/crates/verifier_core

Total : 159 files,  -46907 codes, -1108 comments, -3246 blanks, all -51261 lines

[Summary](results.md) / [Details](details.md) / Diff Summary / [Diff Details](diff-details.md)

## Languages
| language | files | code | comment | blank | total |
| :--- | ---: | ---: | ---: | ---: | ---: |
| Shell Script | 3 | -29 | -8 | -14 | -51 |
| Markdown | 3 | -65 | 0 | -31 | -96 |
| YAML | 2 | -120 | -2 | -16 | -138 |
| JSON | 2 | -3,050 | 0 | -2 | -3,052 |
| Cairo | 26 | -5,243 | -125 | -519 | -5,887 |
| Rust | 123 | -38,400 | -973 | -2,664 | -42,037 |

## Directories
| path | files | code | comment | blank | total |
| :--- | ---: | ---: | ---: | ---: | ---: |
| . | 159 | -46,907 | -1,108 | -3,246 | -51,261 |
| .. | 159 | -46,907 | -1,108 | -3,246 | -51,261 |
| ../.. | 133 | -41,664 | -983 | -2,727 | -45,374 |
| ../.. (Files) | 1 | -41 | 0 | -18 | -59 |
| ../../.. | 132 | -41,623 | -983 | -2,709 | -45,315 |
| ../../.. (Files) | 1 | -1 | 0 | -1 | -2 |
| ../../../.github | 2 | -120 | -2 | -16 | -138 |
| ../../../.github/workflows | 2 | -120 | -2 | -16 | -138 |
| ../../../stwo_cairo_prover | 129 | -41,502 | -981 | -2,692 | -45,175 |
| ../../../stwo_cairo_prover/crates | 125 | -41,459 | -973 | -2,677 | -45,109 |
| ../../../stwo_cairo_prover/crates/adapted_prover | 1 | -67 | -8 | -12 | -87 |
| ../../../stwo_cairo_prover/crates/adapted_prover/src | 1 | -67 | -8 | -12 | -87 |
| ../../../stwo_cairo_prover/crates/air_structs_derive | 1 | -96 | -4 | -17 | -117 |
| ../../../stwo_cairo_prover/crates/air_structs_derive/src | 1 | -96 | -4 | -17 | -117 |
| ../../../stwo_cairo_prover/crates/cairo-serialize | 1 | -164 | -2 | -20 | -186 |
| ../../../stwo_cairo_prover/crates/cairo-serialize-derive | 1 | -44 | -4 | -7 | -55 |
| ../../../stwo_cairo_prover/crates/cairo-serialize-derive/src | 1 | -44 | -4 | -7 | -55 |
| ../../../stwo_cairo_prover/crates/cairo-serialize/src | 1 | -164 | -2 | -20 | -186 |
| ../../../stwo_cairo_prover/crates/prover | 109 | -39,665 | -885 | -2,397 | -42,947 |
| ../../../stwo_cairo_prover/crates/prover/src | 109 | -39,665 | -885 | -2,397 | -42,947 |
| ../../../stwo_cairo_prover/crates/prover/src (Files) | 2 | -61 | -15 | -12 | -88 |
| ../../../stwo_cairo_prover/crates/prover/src/cairo_air | 5 | -3,705 | -58 | -110 | -3,873 |
| ../../../stwo_cairo_prover/crates/prover/src/components | 90 | -30,982 | -662 | -2,072 | -33,716 |
| ../../../stwo_cairo_prover/crates/prover/src/components (Files) | 2 | -159 | -26 | -25 | -210 |
| ../../../stwo_cairo_prover/crates/prover/src/components/add_ap_opcode | 3 | -554 | -11 | -54 | -619 |
| ../../../stwo_cairo_prover/crates/prover/src/components/add_ap_opcode_imm | 3 | -529 | -11 | -54 | -594 |
| ../../../stwo_cairo_prover/crates/prover/src/components/add_ap_opcode_op_1_base_fp | 3 | -554 | -11 | -54 | -619 |
| ../../../stwo_cairo_prover/crates/prover/src/components/add_opcode | 3 | -1,421 | -17 | -67 | -1,505 |
| ../../../stwo_cairo_prover/crates/prover/src/components/add_opcode_imm | 3 | -1,369 | -15 | -67 | -1,451 |
| ../../../stwo_cairo_prover/crates/prover/src/components/add_opcode_small | 3 | -957 | -30 | -78 | -1,065 |
| ../../../stwo_cairo_prover/crates/prover/src/components/add_opcode_small_imm | 3 | -904 | -28 | -78 | -1,010 |
| ../../../stwo_cairo_prover/crates/prover/src/components/assert_eq_opcode | 3 | -545 | -9 | -51 | -605 |
| ../../../stwo_cairo_prover/crates/prover/src/components/assert_eq_opcode_double_deref | 3 | -658 | -10 | -57 | -725 |
| ../../../stwo_cairo_prover/crates/prover/src/components/assert_eq_opcode_imm | 3 | -488 | -7 | -51 | -546 |
| ../../../stwo_cairo_prover/crates/prover/src/components/call_opcode | 3 | -680 | -12 | -63 | -755 |
| ../../../stwo_cairo_prover/crates/prover/src/components/call_opcode_op_1_base_fp | 3 | -683 | -12 | -63 | -758 |
| ../../../stwo_cairo_prover/crates/prover/src/components/call_opcode_rel | 3 | -720 | -17 | -68 | -805 |
| ../../../stwo_cairo_prover/crates/prover/src/components/generic_opcode | 3 | -4,872 | -239 | -293 | -5,404 |
| ../../../stwo_cairo_prover/crates/prover/src/components/jnz_opcode | 3 | -647 | -7 | -49 | -703 |
| ../../../stwo_cairo_prover/crates/prover/src/components/jnz_opcode_dst_base_fp | 3 | -650 | -7 | -49 | -706 |
| ../../../stwo_cairo_prover/crates/prover/src/components/jnz_opcode_taken | 3 | -907 | -15 | -62 | -984 |
| ../../../stwo_cairo_prover/crates/prover/src/components/jnz_opcode_taken_dst_base_fp | 3 | -907 | -15 | -62 | -984 |
| ../../../stwo_cairo_prover/crates/prover/src/components/jump_opcode | 3 | -558 | -8 | -51 | -617 |
| ../../../stwo_cairo_prover/crates/prover/src/components/jump_opcode_double_deref | 3 | -667 | -9 | -57 | -733 |
| ../../../stwo_cairo_prover/crates/prover/src/components/jump_opcode_rel | 3 | -617 | -13 | -56 | -686 |
| ../../../stwo_cairo_prover/crates/prover/src/components/jump_opcode_rel_imm | 3 | -560 | -11 | -54 | -625 |
| ../../../stwo_cairo_prover/crates/prover/src/components/memory | 7 | -832 | -46 | -115 | -993 |
| ../../../stwo_cairo_prover/crates/prover/src/components/memory (Files) | 1 | -4 | -1 | -2 | -7 |
| ../../../stwo_cairo_prover/crates/prover/src/components/memory/memory_address_to_id | 3 | -269 | -17 | -43 | -329 |
| ../../../stwo_cairo_prover/crates/prover/src/components/memory/memory_id_to_big | 3 | -559 | -28 | -70 | -657 |
| ../../../stwo_cairo_prover/crates/prover/src/components/mul_opcode | 3 | -4,002 | -16 | -109 | -4,127 |
| ../../../stwo_cairo_prover/crates/prover/src/components/mul_opcode_imm | 3 | -3,945 | -14 | -109 | -4,068 |
| ../../../stwo_cairo_prover/crates/prover/src/components/range_check_vector | 3 | -361 | -12 | -64 | -437 |
| ../../../stwo_cairo_prover/crates/prover/src/components/ret_opcode | 3 | -566 | -8 | -55 | -629 |
| ../../../stwo_cairo_prover/crates/prover/src/components/verify_instruction | 3 | -670 | -26 | -57 | -753 |
| ../../../stwo_cairo_prover/crates/prover/src/input | 11 | -4,904 | -150 | -201 | -5,255 |
| ../../../stwo_cairo_prover/crates/prover/src/input (Files) | 8 | -1,590 | -133 | -166 | -1,889 |
| ../../../stwo_cairo_prover/crates/prover/src/input/test_builtins_segments | 1 | -3,036 | 0 | -1 | -3,037 |
| ../../../stwo_cairo_prover/crates/prover/src/input/vm_import | 2 | -278 | -17 | -34 | -329 |
| ../../../stwo_cairo_prover/crates/prover/src/relations | 1 | -13 | 0 | -2 | -15 |
| ../../../stwo_cairo_prover/crates/prover_types | 3 | -1,159 | -41 | -179 | -1,379 |
| ../../../stwo_cairo_prover/crates/prover_types/src | 3 | -1,159 | -41 | -179 | -1,379 |
| ../../../stwo_cairo_prover/crates/recursion_bridge | 2 | -25 | -1 | -8 | -34 |
| ../../../stwo_cairo_prover/crates/recursion_bridge/src | 2 | -25 | -1 | -8 | -34 |
| ../../../stwo_cairo_prover/crates/recursion_bridge/src (Files) | 1 | -1 | 0 | -1 | -2 |
| ../../../stwo_cairo_prover/crates/recursion_bridge/src/proof_generation | 1 | -24 | -1 | -7 | -32 |
| ../../../stwo_cairo_prover/crates/run_and_prove | 1 | -58 | -7 | -10 | -75 |
| ../../../stwo_cairo_prover/crates/run_and_prove/src | 1 | -58 | -7 | -10 | -75 |
| ../../../stwo_cairo_prover/crates/utils | 5 | -138 | -14 | -19 | -171 |
| ../../../stwo_cairo_prover/crates/utils/src | 5 | -138 | -14 | -19 | -171 |
| ../../../stwo_cairo_prover/crates/vm_runner | 1 | -43 | -7 | -8 | -58 |
| ../../../stwo_cairo_prover/crates/vm_runner/src | 1 | -43 | -7 | -8 | -58 |
| ../../../stwo_cairo_prover/scripts | 4 | -43 | -8 | -15 | -66 |
| ../bounded_int | 1 | -4 | -2 | -2 | -8 |
| ../bounded_int/src | 1 | -4 | -2 | -2 | -8 |
| ../cairo_air | 23 | -5,117 | -114 | -495 | -5,726 |
| ../cairo_air/src | 21 | -5,094 | -114 | -491 | -5,699 |
| ../cairo_air/src (Files) | 3 | -849 | -56 | -134 | -1,039 |
| ../cairo_air/src/components | 18 | -4,245 | -58 | -357 | -4,660 |
| ../cairo_air/src/components (Files) | 7 | -1,471 | -4 | -189 | -1,664 |
| ../cairo_air/src/components/addr_to_id | 1 | -229 | -4 | -17 | -250 |
| ../cairo_air/src/components/genericopcode | 1 | -458 | 0 | -5 | -463 |
| ../cairo_air/src/components/id_to_f252 | 2 | -731 | -13 | -47 | -791 |
| ../cairo_air/src/components/jump_t_t_f_opcode | 1 | -337 | -7 | -17 | -361 |
| ../cairo_air/src/components/range_check | 4 | -324 | -4 | -30 | -358 |
| ../cairo_air/src/components/ret_opcode | 1 | -302 | -5 | -19 | -326 |
| ../cairo_air/src/components/verify_instruction | 1 | -393 | -21 | -33 | -447 |
| ../cairo_air/tests | 2 | -23 | 0 | -4 | -27 |
| ../cairo_air/tests (Files) | 1 | -17 | 0 | -3 | -20 |
| ../cairo_air/tests/verify | 1 | -6 | 0 | -1 | -7 |
| ../cairo_verifier | 1 | -7 | 0 | -2 | -9 |
| ../cairo_verifier/src | 1 | -7 | 0 | -2 | -9 |
| ../constraint_framework | 1 | -115 | -9 | -20 | -144 |
| ../constraint_framework/src | 1 | -115 | -9 | -20 | -144 |

[Summary](results.md) / [Details](details.md) / Diff Summary / [Diff Details](diff-details.md)