#[macro_export]
macro_rules! range_check_eval{
    ($($log_range:expr),+) => {
        paste::paste! {
                use serde::{Deserialize, Serialize};
                use stwo_prover::constraint_framework::{EvalAtRow, FrameworkComponent};
                use stwo_prover::constraint_framework::FrameworkEval;
                use stwo_prover::core::channel::Channel;
                use stwo_prover::core::fields::qm31::QM31;
                use stwo_prover::core::fields::secure_column::SECURE_EXTENSION_DEGREE;
                use stwo_prover::constraint_framework::RelationEntry;
                use stwo_prover::core::pcs::TreeVec;

                use $crate::relations;
                use $crate::components::memory::memory_id_to_big::component::N_MULTIPLICITY_COLUMNS;

                const N_RANGES:usize = $crate::count_elements!($($log_range),*);
                const RANGES : [u32; N_RANGES] = [$($log_range),+];
                pub type Component = FrameworkComponent<[<Eval>]>;

                #[derive(Clone, Deserialize, Serialize)]
                pub struct Claim {
                    pub log_ranges: Vec<u32>,
                }
                impl Claim {
                    fn log_size(&self) -> u32 {
                        self.log_ranges.iter().sum()
                    }

                    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
                        TreeVec::new(vec![
                            vec![self.log_size(); 1],
                            vec![self.log_size(); self.log_ranges.len() + N_MULTIPLICITY_COLUMNS],
                            vec![self.log_size(); SECURE_EXTENSION_DEGREE],
                        ])
                    }

                    pub fn mix_into(&self, channel: &mut impl Channel) {
                        for &log_range in self.log_ranges.iter() {
                            channel.mix_u64(log_range as u64);
                        }
                    }
                }

                #[derive(Clone, Copy, Serialize, Deserialize)]
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
                        let rc_values: [E::F; N_RANGES] = std::array::from_fn(|_|
                             eval.next_trace_mask()
                            );
                        let multiplicity = eval.next_trace_mask();
                        eval.add_to_relation(RelationEntry::new(
                            &self.lookup_elements, E::EF::from(-multiplicity), &rc_values
                        ));

                        eval.finalize_logup();
                        eval
                    }
                }
        }
    }
}
