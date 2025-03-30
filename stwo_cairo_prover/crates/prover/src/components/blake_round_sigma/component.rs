use crate::components::prelude::*;

pub const BLAKE_SIGMA_LOG_SIZE: u32 = 4;
pub const N_TRACE_COLUMNS: usize = 1;

pub struct Eval {
    pub blake_round_sigma_lookup_elements: relations::BlakeRoundSigma,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Claim {}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![BLAKE_SIGMA_LOG_SIZE; N_TRACE_COLUMNS];
        let interaction_log_sizes = vec![BLAKE_SIGMA_LOG_SIZE; SECURE_EXTENSION_DEGREE];
        TreeVec::new(vec![vec![], trace_log_sizes, interaction_log_sizes])
    }

    pub fn mix_into(&self, _channel: &mut impl Channel) {}
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct InteractionClaim {
    pub claimed_sum: SecureField,
}
impl InteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_felts(&[self.claimed_sum]);
    }
}

pub type Component = FrameworkComponent<Eval>;

impl FrameworkEval for Eval {
    fn log_size(&self) -> u32 {
        BLAKE_SIGMA_LOG_SIZE
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_size() + 1
    }

    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let blakesigma_0 = eval.get_preprocessed_column((BlakeSigma::new(0)).id());
        let blakesigma_1 = eval.get_preprocessed_column((BlakeSigma::new(1)).id());
        let blakesigma_2 = eval.get_preprocessed_column((BlakeSigma::new(2)).id());
        let blakesigma_3 = eval.get_preprocessed_column((BlakeSigma::new(3)).id());
        let blakesigma_4 = eval.get_preprocessed_column((BlakeSigma::new(4)).id());
        let blakesigma_5 = eval.get_preprocessed_column((BlakeSigma::new(5)).id());
        let blakesigma_6 = eval.get_preprocessed_column((BlakeSigma::new(6)).id());
        let blakesigma_7 = eval.get_preprocessed_column((BlakeSigma::new(7)).id());
        let blakesigma_8 = eval.get_preprocessed_column((BlakeSigma::new(8)).id());
        let blakesigma_9 = eval.get_preprocessed_column((BlakeSigma::new(9)).id());
        let blakesigma_10 = eval.get_preprocessed_column((BlakeSigma::new(10)).id());
        let blakesigma_11 = eval.get_preprocessed_column((BlakeSigma::new(11)).id());
        let blakesigma_12 = eval.get_preprocessed_column((BlakeSigma::new(12)).id());
        let blakesigma_13 = eval.get_preprocessed_column((BlakeSigma::new(13)).id());
        let blakesigma_14 = eval.get_preprocessed_column((BlakeSigma::new(14)).id());
        let blakesigma_15 = eval.get_preprocessed_column((BlakeSigma::new(15)).id());
        let seq = eval.get_preprocessed_column(Seq::new(self.log_size()).id());
        let mult = eval.next_trace_mask();

        eval.add_to_relation(RelationEntry::new(
            &self.blake_round_sigma_lookup_elements,
            E::EF::from(-mult),
            &[
                seq.clone(),
                blakesigma_0.clone(),
                blakesigma_1.clone(),
                blakesigma_2.clone(),
                blakesigma_3.clone(),
                blakesigma_4.clone(),
                blakesigma_5.clone(),
                blakesigma_6.clone(),
                blakesigma_7.clone(),
                blakesigma_8.clone(),
                blakesigma_9.clone(),
                blakesigma_10.clone(),
                blakesigma_11.clone(),
                blakesigma_12.clone(),
                blakesigma_13.clone(),
                blakesigma_14.clone(),
                blakesigma_15.clone(),
            ],
        ));

        eval.finalize_logup_in_pairs();
        eval
    }
}
