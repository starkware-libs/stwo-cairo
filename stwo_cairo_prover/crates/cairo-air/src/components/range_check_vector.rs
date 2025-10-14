#[macro_export]
macro_rules! count_elements {
    ($x:expr) => (1);
    ($x:expr, $($xs:expr),*) => (1 + $crate::count_elements!($($xs),*));
}

#[macro_export]
// Macro for generating a module of a range_check component for a given vector of logs.
// If only a vector [l1,...,lk] is provided, the module will be named `range_check_l1..._lk`.
// If a suffix is provided, the module will be named `range_check_l1..._lk_suffix`.
// The suffix should be a lowercase letter and the uppercase letter should also be provided.
// TODO(ohadn): unite rangechecks with the same log size into one component.
macro_rules! generate_range_check_constraints {
    ([$($log_range:expr),+]) => {
        paste::paste!{
            pub mod [<range_check_$($log_range)_*>] {
                $crate::range_check_eval!([<range_check_$($log_range)_*>], [<_$($log_range)_*>], $($log_range),+);
            }
        }
    };
    ([$($log_range:expr),+], $suffix:ident, $suffix_upper:ident) => {
        paste::paste! {
            pub mod [<range_check_$($log_range)_*_$suffix>] {
                $crate::range_check_eval!([<range_check_$($log_range)_*_$suffix>], [<_$($log_range)_*_$suffix_upper>], $($log_range),+);
            }
        }
    };
}

generate_range_check_constraints!([6]);
generate_range_check_constraints!([8]);
generate_range_check_constraints!([11]);
generate_range_check_constraints!([12]);
generate_range_check_constraints!([18]);
generate_range_check_constraints!([18], b, B);
generate_range_check_constraints!([20]);
generate_range_check_constraints!([20], b, B);
generate_range_check_constraints!([20], c, C);
generate_range_check_constraints!([20], d, D);
generate_range_check_constraints!([20], e, E);
generate_range_check_constraints!([20], f, F);
generate_range_check_constraints!([20], g, G);
generate_range_check_constraints!([20], h, H);
generate_range_check_constraints!([4, 3]);
generate_range_check_constraints!([4, 4]);
generate_range_check_constraints!([5, 4]);
generate_range_check_constraints!([9, 9]);
generate_range_check_constraints!([9, 9], b, B);
generate_range_check_constraints!([9, 9], c, C);
generate_range_check_constraints!([9, 9], d, D);
generate_range_check_constraints!([9, 9], e, E);
generate_range_check_constraints!([9, 9], f, F);
generate_range_check_constraints!([9, 9], g, G);
generate_range_check_constraints!([9, 9], h, H);
generate_range_check_constraints!([7, 2, 5]);
generate_range_check_constraints!([3, 6, 6, 3]);
generate_range_check_constraints!([4, 4, 4, 4]);
generate_range_check_constraints!([3, 3, 3, 3, 3]);

#[macro_export]
macro_rules! range_check_eval{
    ($name:ident, $suffix_upper:ident, $($log_range:expr),+) => {
        paste::paste! {
            use serde::{Deserialize, Serialize};
            use stwo_cairo_serialize::{CairoDeserialize, CairoSerialize};
            use stwo_constraint_framework::{EvalAtRow, FrameworkComponent};
            use stwo_constraint_framework::FrameworkEval;
            use stwo::core::channel::Channel;
            use stwo::core::fields::qm31::{QM31, SECURE_EXTENSION_DEGREE};
            use stwo_constraint_framework::RelationEntry;
            use stwo::core::pcs::TreeVec;

            use stwo_cairo_common::preprocessed_columns::preprocessed_trace::RangeCheck;
            use stwo_cairo_common::preprocessed_columns::preprocessed_trace::Seq;
            use stwo_cairo_common::preprocessed_columns::preprocessed_trace::PreProcessedColumn;
            use $crate::relations;

            const N_MULTIPLICITY_COLUMNS: usize = 1;
            const N_RANGES:usize = $crate::count_elements!($($log_range),*);
            const RANGES : [u32; N_RANGES] = [$($log_range),+];
            pub type Component = FrameworkComponent<[<Eval>]>;

            #[derive(Clone, Deserialize, Serialize, CairoSerialize, CairoDeserialize)]
            pub struct Claim {}

            impl Claim {
                fn log_size(&self) -> u32 {
                    RANGES.iter().sum()
                }

                pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
                    TreeVec::new(vec![
                        vec![],
                        vec![self.log_size(); N_MULTIPLICITY_COLUMNS],
                        vec![self.log_size(); SECURE_EXTENSION_DEGREE],
                    ])
                }

                pub fn mix_into(&self, _channel: &mut impl Channel) {}
            }

            #[derive(Clone, Copy, Serialize, Deserialize, CairoSerialize, CairoDeserialize)]
            pub struct InteractionClaim{
                pub claimed_sum: QM31,
            }
            impl InteractionClaim {
                pub fn mix_into(&self, channel: &mut impl Channel) {
                    channel.mix_felts(&[self.claimed_sum]);
                }
            }

            pub struct Eval {
                pub lookup_elements: relations::[<RangeCheck$suffix_upper>]
            }
            impl Eval {
                pub fn new(lookup_elements: relations::[<RangeCheck$suffix_upper>],) -> Self {
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
                    let values = if N_RANGES == 1 {
                        vec![eval.get_preprocessed_column(Seq::new(self.log_size()).id())]
                    } else {
                        (0..N_RANGES)
                        .map(|i| eval.get_preprocessed_column(RangeCheck::new(RANGES, i).id()))
                        .collect::<Vec<_>>()
                    };
                    let multiplicity = eval.next_trace_mask();
                    eval.add_to_relation(RelationEntry::new(
                        &self.lookup_elements, E::EF::from(-multiplicity), &values
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
                use stwo_constraint_framework::expr::ExprEvaluator;
                use stwo::core::fields::qm31::QM31;

                use super::*;
                use $crate::components::constraints_regression_test_values::[<RANGE_CHECK$suffix_upper>];

                #[test]
                fn [<constraints_regression_$name>]() {
                    let mut rng = SmallRng::seed_from_u64(0);
                    let eval = Eval {
                        lookup_elements: relations::[<RangeCheck$suffix_upper>]::dummy(),
                    };

                    let expr_eval = eval.evaluate(ExprEvaluator::new());
                    let assignment = expr_eval.random_assignment();

                    let mut sum = QM31::zero();
                    for c in expr_eval.constraints {
                        sum += c.assign(&assignment) * rng.gen::<QM31>();
                    }

                    assert_eq!(sum, [<RANGE_CHECK$suffix_upper>]);
                }
            }
        }
    };
}
