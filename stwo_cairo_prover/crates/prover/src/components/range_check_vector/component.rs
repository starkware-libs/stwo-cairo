#[macro_export]
macro_rules! range_check_eval{
    ($($log_range:expr),+) => {
        paste::paste! {
                use serde::{Deserialize, Serialize};
                use stwo_prover::constraint_framework::{EvalAtRow, FrameworkComponent};
                use stwo_prover::constraint_framework::FrameworkEval;
                use stwo_prover::constraint_framework::INTERACTION_TRACE_IDX;
                use stwo_prover::constraint_framework::logup::LogupAtRow;
                use stwo_prover::constraint_framework::logup::LookupElements;
                use stwo_prover::constraint_framework::preprocessed_columns::PreprocessedColumn;
                use stwo_prover::core::channel::Channel;
                use stwo_prover::core::fields::qm31::QM31;
                use stwo_prover::core::fields::secure_column::SECURE_EXTENSION_DEGREE;
                use stwo_prover::core::lookups::utils::Fraction;
                use stwo_prover::core::pcs::TreeVec;

                use $crate::components::memory::id_to_f252::component::N_MULTIPLICITY_COLUMNS;

                const N_RANGES:usize = $crate::count_elements!($($log_range),*);
                const RANGES : [u32; N_RANGES] = [$($log_range),+];
                pub type RelationElements = LookupElements<N_RANGES>;
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
                    pub lookup_elements: RelationElements,
                    pub claimed_sum: QM31,
                }
                impl Eval {
                    pub fn new(lookup_elements: RelationElements,
                         interaction_claim: InteractionClaim
                        ) -> Self {
                        Self {
                            lookup_elements,
                            claimed_sum: interaction_claim.claimed_sum,
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
                        let is_first = eval.get_preprocessed_column(
                            PreprocessedColumn::IsFirst(self.log_size())
                        );
                        let mut logup =
                            LogupAtRow::<E>::new(
                                INTERACTION_TRACE_IDX,
                                self.claimed_sum, None,
                                is_first
                            );
                        let rc_values: [E::F; N_RANGES] = std::array::from_fn(|_|
                             eval.next_trace_mask()
                            );
                        let multiplicity = eval.next_trace_mask();
                        logup.write_frac(
                            &mut eval,
                            Fraction::new(
                                E::EF::from(-multiplicity),
                                self.lookup_elements.combine(&rc_values),
                            ),
                        );
                        logup.finalize(&mut eval);

                        eval
                    }
                }
        }
    }
}
