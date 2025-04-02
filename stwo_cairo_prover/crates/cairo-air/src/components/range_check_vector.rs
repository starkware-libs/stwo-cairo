#[macro_export]
macro_rules! count_elements {
    ($x:expr) => (1);
    ($x:expr, $($xs:expr),*) => (1 + $crate::count_elements!($($xs),*));
}

#[macro_export]
macro_rules! generate_range_check_constraints {
    ([$($log_range:expr),+]) => {
        paste::paste!{
            pub mod [<range_check_$($log_range)_*>] {
                $crate::range_check_eval!($($log_range),+);
            }
        }
    };
}

generate_range_check_constraints!([6]);
generate_range_check_constraints!([8]);
generate_range_check_constraints!([11]);
generate_range_check_constraints!([12]);
generate_range_check_constraints!([18]);
generate_range_check_constraints!([19]);
generate_range_check_constraints!([3, 6]);
generate_range_check_constraints!([4, 3]);
generate_range_check_constraints!([4, 4]);
generate_range_check_constraints!([5, 4]);
generate_range_check_constraints!([9, 9]);
generate_range_check_constraints!([7, 2, 5]);
generate_range_check_constraints!([3, 6, 6, 3]);
generate_range_check_constraints!([4, 4, 4, 4]);
generate_range_check_constraints!([3, 3, 3, 3, 3]);

#[macro_export]
macro_rules! range_check_eval{
    ($($log_range:expr),+) => {
        paste::paste! {
                use serde::{Deserialize, Serialize};
                use stwo_cairo_serialize::CairoSerialize;
                use stwo_prover::constraint_framework::{EvalAtRow, FrameworkComponent};
                use stwo_prover::constraint_framework::FrameworkEval;
                use stwo_prover::core::channel::Channel;
                use stwo_prover::core::fields::qm31::QM31;
                use stwo_prover::core::fields::secure_column::SECURE_EXTENSION_DEGREE;
                use stwo_prover::constraint_framework::RelationEntry;
                use stwo_prover::core::pcs::TreeVec;

                use $crate::preprocessed::RangeCheck;
                use $crate::preprocessed::PreProcessedColumn;
                use $crate::relations;

                const N_MULTIPLICITY_COLUMNS: usize = 1;
                const N_RANGES:usize = $crate::count_elements!($($log_range),*);
                const RANGES : [u32; N_RANGES] = [$($log_range),+];
                pub type Component = FrameworkComponent<[<Eval>]>;

                #[derive(Clone, Deserialize, Serialize, CairoSerialize)]
                pub struct Claim {
                    pub log_ranges: Vec<u32>,
                }
                impl Claim {
                    fn log_size(&self) -> u32 {
                        self.log_ranges.iter().sum()
                    }

                    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
                        TreeVec::new(vec![
                            vec![],
                            vec![self.log_size(); N_MULTIPLICITY_COLUMNS],
                            vec![self.log_size(); SECURE_EXTENSION_DEGREE],
                        ])
                    }

                    pub fn mix_into(&self, channel: &mut impl Channel) {
                        for &log_range in self.log_ranges.iter() {
                            channel.mix_u64(log_range as u64);
                        }
                    }
                }

                #[derive(Clone, Copy, Serialize, Deserialize, CairoSerialize)]
                pub struct InteractionClaim{
                    pub claimed_sum: QM31,
                }
                impl InteractionClaim {
                    pub fn mix_into(&self, channel: &mut impl Channel) {
                        channel.mix_felts(&[self.claimed_sum]);
                    }
                }

                pub struct Eval {
                    pub lookup_elements: relations::[<RangeCheck_$($log_range)_*>],
                }
                impl Eval {
                    pub fn new(lookup_elements: relations::[<RangeCheck_$($log_range)_*>],) -> Self {
                        Self {
                            lookup_elements,
                        }
                    }
                }

                impl FrameworkEval for Eval {
                    fn log_size(&self) -> u32 {
                        RANGES.iter().sum()
                    }
                    fn max_constraint_log_degree_bound(&self) -> u32 {
                        self.log_size() + 1
                    }
                    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
                        let rc_values = (0..N_RANGES)
                            .map(|i| eval.get_preprocessed_column(RangeCheck::new(RANGES, i).id()))
                            .collect::<Vec<_>>();
                        let multiplicity = eval.next_trace_mask();
                        eval.add_to_relation(RelationEntry::new(
                            &self.lookup_elements, E::EF::from(-multiplicity), &rc_values
                        ));

                        eval.finalize_logup();
                        eval
                    }
                }

                #[cfg(test)]
                mod tests {
                    use num_traits::Zero;
                    use rand::rngs::SmallRng;
                    use rand::{Rng, SeedableRng};
                    use stwo_prover::constraint_framework::expr::ExprEvaluator;
                    use stwo_prover::core::fields::qm31::QM31;

                    use super::*;
                    use $crate::components::constraints_regression_test_values::RANGE_CHECK_VECTOR;

                    #[test]
                    fn range_check_vector_constraints_regression() {
                        let eval = Eval {
                            lookup_elements: relations::[<RangeCheck_$($log_range)_*>]::dummy(),
                        };

                        let expr_eval = eval.evaluate(ExprEvaluator::new());
                        let mut rng = SmallRng::seed_from_u64(0);
                        let mut sum = QM31::zero();
                        for c in expr_eval.constraints {
                            sum += c.random_eval() * rng.gen::<QM31>();
                        }

                        assert_eq!(sum, RANGE_CHECK_VECTOR);
                    }
                }
        }
    }
}
