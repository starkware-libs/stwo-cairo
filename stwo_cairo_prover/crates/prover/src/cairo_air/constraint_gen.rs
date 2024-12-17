mod tests {
    use std::collections::{BTreeMap, BTreeSet};
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::Path;

    use itertools::{chain, Itertools};
    use stwo_prover::constraint_framework::expr::{BaseExpr, ColumnExpr, ExprEvaluator, ExtExpr};
    use stwo_prover::constraint_framework::FrameworkEval;

    use crate::components::{
        generic_opcode, jump_opcode_is_rel_t_is_imm_t_is_double_deref_f, memory_address_to_id,
        memory_id_to_big, range_check_19, range_check_4_3, range_check_7_2_5, range_check_9_9,
        ret_opcode, verify_instruction,
    };
    use crate::relations;

    #[test]
    fn gen_code() {
        let path = "";
        gen_memory_id_to_big(path);
        gen_memory_id_to_small(path);
        gen_memory_addr_to_id(path);

        gen_jmp_code(path);
        gen_verify_instruction_code(path);
        gen_ret_constraint_code(path);
        gen_rc_4_3_code(path);
        gen_rc_7_2_5_code(path);
        gen_rc_9_9_code(path);
        gen_rc_19_code(path);

        // TODO(ilya): does not compie
        gen_generic_opcode_code(path);
    }

    fn gen_memory_id_to_big(path: &str) {
        let componenet_eval = memory_id_to_big::BigEval {
            log_n_rows: 4,
            lookup_elements: relations::MemoryIdToBig::dummy(),
            range9_9_lookup_elements: relations::RangeCheck_9_9::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new(16, false)),
            path.to_string() + "components/id_to_f252/constraints_big.cairo",
        )
        .unwrap();
    }

    fn gen_memory_id_to_small(path: &str) {
        let componenet_eval = memory_id_to_big::SmallEval {
            log_n_rows: 4,
            lookup_elements: relations::MemoryIdToBig::dummy(),
            range_check_9_9_relation: relations::RangeCheck_9_9::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new(16, false)),
            path.to_string() + "components/id_to_f252/constraints_small.cairo",
        )
        .unwrap();
    }

    fn gen_memory_addr_to_id(path: &str) {
        let componenet_eval = memory_address_to_id::Eval {
            log_size: 4,
            lookup_elements: relations::MemoryAddressToId::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new(16, false)),
            path.to_string() + "components/addr_to_id/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_jmp_code(path: &str) {
        let componenet_eval = jump_opcode_is_rel_t_is_imm_t_is_double_deref_f::Eval {
            claim: jump_opcode_is_rel_t_is_imm_t_is_double_deref_f::Claim { n_calls: 0 },
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
            opcodes_lookup_elements: relations::Opcodes::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new(16, true)),
            path.to_string() + "components/jump_t_t_f_opcode/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_verify_instruction_code(path: &str) {
        let componenet_eval = verify_instruction::Eval {
            claim: verify_instruction::Claim { log_size: 0 },
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
            range_check_4_3_lookup_elements: relations::RangeCheck_4_3::dummy(),
            range_check_7_2_5_lookup_elements: relations::RangeCheck_7_2_5::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new(16, false)),
            path.to_string() + "components/verify_instruction/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_ret_constraint_code(path: &str) {
        let componenet_eval = ret_opcode::Eval {
            claim: ret_opcode::Claim { n_calls: 0 },
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
            opcodes_lookup_elements: relations::Opcodes::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new(16, true)),
            path.to_string() + "components/ret_opcode/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_generic_opcode_code(path: &str) {
        let componenet_eval = generic_opcode::Eval {
            claim: generic_opcode::Claim { n_calls: 0 },
            memory_address_to_id_lookup_elements: relations::MemoryAddressToId::dummy(),
            memory_id_to_big_lookup_elements: relations::MemoryIdToBig::dummy(),
            range_check_19_lookup_elements: relations::RangeCheck_19::dummy(),
            range_check_9_9_lookup_elements: relations::RangeCheck_9_9::dummy(),
            verify_instruction_lookup_elements: relations::VerifyInstruction::dummy(),
            opcodes_lookup_elements: relations::Opcodes::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new(16, true)),
            path.to_string() + "components/generic_opcode/constraints.cairo",
        )
        .unwrap();
    }

    fn gen_rc_4_3_code(path: &str) {
        let componenet_eval = range_check_4_3::Eval {
            lookup_elements: relations::RangeCheck_4_3::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new(16, false)),
            path.to_string() + "components/range_check/rc_4_3_constraints.cairo",
        )
        .unwrap();
    }

    fn gen_rc_7_2_5_code(path: &str) {
        let componenet_eval = range_check_7_2_5::Eval {
            lookup_elements: relations::RangeCheck_7_2_5::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new(16, false)),
            path.to_string() + "components/range_check/rc_7_2_5_constraints.cairo",
        )
        .unwrap();
    }

    fn gen_rc_9_9_code(path: &str) {
        let componenet_eval = range_check_9_9::Eval {
            lookup_elements: relations::RangeCheck_9_9::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new(16, false)),
            path.to_string() + "components/range_check/rc_9_9_constraints.cairo",
        )
        .unwrap();
    }

    fn gen_rc_19_code(path: &str) {
        let componenet_eval = range_check_19::Eval {
            lookup_elements: relations::RangeCheck_19::dummy(),
        };

        gen_cairo_constraint_code(
            componenet_eval.evaluate(ExprEvaluator::new(16, false)),
            path.to_string() + "components/range_check/rc_19_constraints.cairo",
        )
        .unwrap();
    }

    fn gen_cairo_constraint_code(
        constraint_expr: ExprEvaluator,
        output_path: impl AsRef<Path>,
    ) -> std::io::Result<()> {
        if let Some(parent_path) = output_path.as_ref().parent() {
            fs::create_dir_all(parent_path)?;
        }

        let mut file = File::create(&output_path)?;
        file.write_all(component_constraint_code(constraint_expr).as_bytes());

        Ok(())
    }

    #[derive(Clone, Copy)]
    pub struct ColumnExprPub {
        pub interaction: usize,
        pub idx: usize,
        pub offset: isize,
    }

    const CLAIMED_SUM_DUMMY_OFFSET: usize = stwo_prover::core::fields::m31::P as usize;

    fn component_constraint_code(expression: ExprEvaluator) -> String {
        // Offsets relative to the component.
        let mut preprocessed_column_indices = BTreeSet::new();
        let mut trace_column_offsets = BTreeMap::<usize, BTreeSet<isize>>::new();
        let mut interaction_column_offsets = BTreeMap::<usize, BTreeSet<isize>>::new();

        let collect_cols = &mut |node: &BaseExpr| {
            if let BaseExpr::Col(column_expr) = node {
                let ColumnExprPub {
                    interaction,
                    idx,
                    offset,
                } = *unsafe { core::mem::transmute::<&ColumnExpr, &ColumnExprPub>(column_expr) };

                match interaction {
                    // Preprocessed column.
                    0 => {
                        assert!(offset == 0);
                        preprocessed_column_indices.insert(idx);
                    }
                    // Base trace.
                    1 => {
                        trace_column_offsets.entry(idx).or_default().insert(offset);
                    }
                    // Interaction trace.
                    2 => {
                        interaction_column_offsets
                            .entry(idx)
                            .or_default()
                            .insert(offset);
                    }
                    _ => panic!(),
                }
            }
        };

        let mut parameters = BTreeSet::new();

        for intermediate in expression.intermediates.iter().map(|(_, expr)| expr) {
            intermediate.traverse(collect_cols);
            intermediate.traverse(&mut |base_expr| {
                if let BaseExpr::Param(param) = base_expr {
                    parameters.insert(param.clone());
                }
            });
        }

        for constraint in chain![
            expression.ext_intermediates.iter().map(|(_, expr)| expr),
            &expression.constraints
        ] {
            constraint.traverse(&mut |node| {
                if let ExtExpr::SecureCol(base_exprs) = node {
                    base_exprs.iter().for_each(|base_expr| {
                        base_expr.traverse(collect_cols);
                        base_expr.traverse(&mut |base_expr| {
                            if let BaseExpr::Param(param) = base_expr {
                                parameters.insert(param.clone());
                            }
                        });
                    });
                }

                if let ExtExpr::Param(param) = node {
                    parameters.insert(param.clone());
                }
            });
        }

        assert!(preprocessed_column_indices.is_empty());

        // Sanity check uses all columns.
        let trace_col_min = *trace_column_offsets.keys().min().unwrap_or(&0);
        let trace_col_max = *trace_column_offsets.keys().max().unwrap_or(&0);
        (trace_col_min..=trace_col_max)
            .for_each(|i| assert!(trace_column_offsets.get(&i).is_some()));
        let interaction_col_min = *interaction_column_offsets
            .keys()
            .min()
            .unwrap_or(&trace_col_max);
        let interaction_col_max = *interaction_column_offsets
            .keys()
            .max()
            .unwrap_or(&interaction_col_min);
        (interaction_col_min..=interaction_col_max)
            .for_each(|i| assert!(interaction_column_offsets.get(&i).is_some()));

        let unique_offsets = chain![
            trace_column_offsets.values(),
            interaction_column_offsets.values()
        ]
        .fold(BTreeSet::new(), |a, b| &a | b);

        let has_partial_sum = expression.logup.claimed_sum.is_some();

        let claimed_sum_offset_parameter = if has_partial_sum {
            "claimed_sum_offset: usize,"
        } else {
            ""
        };

        let offset_var_name = |offset: isize| {
            if offset == 0 {
                return "point".to_string();
            }

            if offset == CLAIMED_SUM_DUMMY_OFFSET as isize {
                return "point_offset_claimed_sum".to_string();
            }

            let offset_abs = offset.abs();

            if offset > 0 {
                format!("point_offset_{offset_abs}")
            } else {
                format!("point_offset_neg_{offset_abs}")
            }
        };

        let offset_variables = unique_offsets.iter().filter_map(|offset| {
            if *offset == 0 {
                return None;
            }

            if *offset == CLAIMED_SUM_DUMMY_OFFSET as isize {
                assert!(has_partial_sum);
                return Some("let point_offset_claimed_sum = point.add_circle_point_m31(trace_gen.mul(claimed_sum_offset).to_point());".to_string());
            }

            let offset_abs = offset.abs();

            if *offset > 0 {
                Some(format!("let point_offset_{offset_abs} = point.add_circle_point_m31(trace_gen.mul({offset_abs}).to_point());"))
            } else {
                Some(format!("let point_offset_neg_{offset_abs} = point.add_circle_point_m31(-trace_gen.mul({offset_abs}).to_point());"))
            }
        }).join("\n");

        let trace_mask_points = (trace_col_min..=trace_col_max)
            .map(|i| {
                let column_offsets = &trace_column_offsets[&i];
                let column_mask_points = column_offsets
                    .iter()
                    .map(|&offset| offset_var_name(offset))
                    .join(",");
                format!("trace_mask_points.append(array![{column_mask_points}]);")
            })
            .join("\n");

        let interaction_mask_points = (interaction_col_min..=interaction_col_max)
            .map(|i| {
                let column_offsets = &interaction_column_offsets[&i];
                let column_mask_points = column_offsets
                    .iter()
                    .map(|&offset| offset_var_name(offset))
                    .join(",");
                format!("interaction_trace_mask_points.append(array![{column_mask_points}]);")
            })
            .join("\n");

        let mask_function = format!(
            r#"
        pub fn mask_points(
            ref trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
            ref interaction_trace_mask_points: ColumnArray<Array<CirclePoint<QM31>>>,
            point: CirclePoint<QM31>,
            trace_gen: CirclePointIndex,
            {claimed_sum_offset_parameter}
        ) {{
            {offset_variables}
            {trace_mask_points}
            {interaction_mask_points}
        }}"#
        );

        let trace_mask_columns_and_offsets = (trace_col_min..=trace_col_max)
            .map(|i| {
                let offsets = &trace_column_offsets[&i].iter().map(|&offset| {
                    if offset == CLAIMED_SUM_DUMMY_OFFSET as isize {
                        format!("let trace_1_column_{i}_offset_claimed_sum = *trace_1_column_{i}.pop_front().unwrap();")
                    } else if offset >= 0 {
                        format!("let trace_1_column_{i}_offset_{offset} = *trace_1_column_{i}.pop_front().unwrap();")
                    } else {
                        let offset_abs = offset.abs();
                        format!("let trace_1_column_{i}_offset_neg_{offset_abs} = *trace_1_column_{i}.pop_front().unwrap();")
                    }
                }).join("\n");

                format!("let mut trace_1_column_{i} = trace_mask_values.pop_front().unwrap().span();\n{offsets}")
            })
            .join("\n");

        let interaction_mask_columns_and_offsets = (interaction_col_min..=interaction_col_max)
            .map(|i| {
                let offsets = &interaction_column_offsets[&i].iter().map(|&offset| {
                    if offset == CLAIMED_SUM_DUMMY_OFFSET as isize {
                        format!("let trace_2_column_{i}_offset_claimed_sum = *trace_2_column_{i}.pop_front().unwrap();")
                    } else if offset >= 0 {
                        format!("let trace_2_column_{i}_offset_{offset} = *trace_2_column_{i}.pop_front().unwrap();")
                    } else {
                        let offset_abs = offset.abs();
                        format!("let trace_2_column_{i}_offset_neg_{offset_abs} = *trace_2_column_{i}.pop_front().unwrap();")
                    }
                }).join("\n");

                format!("let mut trace_2_column_{i} = interaction_mask_values.pop_front().unwrap().span();\n{offsets}")
            })
            .join("\n");

        let constraint_str = expression.format_constraints();

        let constraints_sum_str = (0..expression.constraints.len())
            .map(|i| {
                format!("// TODO: Batch `domain_vanish_at_point_inv` multiplication.\nsum = sum * random_coeff + constraint_{i} * domain_vanish_at_point_inv;")
            })
            .join("\n");

        let constraint_parameters = parameters
            .iter()
            .filter_map(|parameter| {
                if parameter.starts_with("intermediate") {
                    return None;
                }

                Some(format!("pub {parameter}: QM31,"))
            })
            .join("\n");

        let constraint_parameter_keys = parameters
            .iter()
            .filter_map(|parameter| {
                if parameter.starts_with("intermediate") {
                    return None;
                }

                Some(format!("{parameter},"))
            })
            .join("\n");

        let constraint_eval_function = format!(
            r#"
        #[derive(Drop)]
        pub struct ConstraintParams {{
            {constraint_parameters}
        }}

        pub fn evaluate_constraints_at_point(
            ref sum: QM31,
            ref trace_mask_values: ColumnSpan<Array<QM31>>,
            ref interaction_mask_values: ColumnSpan<Array<QM31>>,
            params: ConstraintParams,
            random_coeff: QM31,
            domain_vanish_at_point_inv: QM31,
        ) {{
            let ConstraintParams {{
                {constraint_parameter_keys}
            }} = params;

            {trace_mask_columns_and_offsets}
            {interaction_mask_columns_and_offsets}
            {constraint_str}
            {constraints_sum_str}
        }}
        "#
        );

        format!(
            r#"
            use stwo_verifier_core::{{ColumnSpan, ColumnArray}};
            use stwo_verifier_core::circle::{{CirclePoint, CirclePointIndex, CirclePointIndexImpl, CirclePointQM31AddCirclePointM31Impl}};
            use stwo_verifier_core::fields::m31::{{m31, M31}};
            use stwo_verifier_core::fields::qm31::{{QM31, QM31Impl, qm31}};

        {mask_function}
        {constraint_eval_function}
        "#
        )
    }

    trait Traversable {
        fn traverse(&self, f: &mut impl FnMut(&Self));
    }

    impl Traversable for ExtExpr {
        fn traverse(&self, f: &mut impl FnMut(&Self)) {
            match self {
                // Tree types are recursed first
                ExtExpr::Neg(a) => a.traverse(f),
                ExtExpr::SecureCol(_) | ExtExpr::Param(_) | ExtExpr::Const(_) => {}
                ExtExpr::Sub(a, b) | ExtExpr::Add(a, b) | ExtExpr::Mul(a, b) => {
                    a.traverse(f);
                    b.traverse(f);
                }
            };

            f(self);
        }
    }

    impl Traversable for BaseExpr {
        fn traverse(&self, f: &mut impl FnMut(&Self)) {
            match self {
                // Tree types are recursed first
                BaseExpr::Neg(a) | BaseExpr::Inv(a) => a.traverse(f),
                BaseExpr::Col(_) | BaseExpr::Param(_) | BaseExpr::Const(_) => {}
                BaseExpr::Sub(a, b) | BaseExpr::Add(a, b) | BaseExpr::Mul(a, b) => {
                    a.traverse(f);
                    b.traverse(f);
                }
            };

            f(self);
        }
    }
}
