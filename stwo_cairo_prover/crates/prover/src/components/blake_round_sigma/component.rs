use crate::components::prelude::constraint_eval::*;

pub struct Eval {
    pub claim: Claim,
    pub blake_round_sigma_lookup_elements: relations::BlakeRoundSigma,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Claim {
    pub log_size: u32,
}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![self.log_size; 0];
        let interaction_log_sizes = vec![self.log_size; SECURE_EXTENSION_DEGREE];
        TreeVec::new(vec![vec![], trace_log_sizes, interaction_log_sizes])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_u64(self.log_size as u64);
    }
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
        self.claim.log_size
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_size() + 1
    }

    #[allow(unused_parens)]
    #[allow(clippy::double_parens)]
    #[allow(non_snake_case)]
    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let blakesigma = eval.get_preprocessed_column((BlakeSigma::new(0)).id());
        let blakesigma = eval.get_preprocessed_column((BlakeSigma::new(1)).id());
        let blakesigma = eval.get_preprocessed_column((BlakeSigma::new(10)).id());
        let blakesigma = eval.get_preprocessed_column((BlakeSigma::new(11)).id());
        let blakesigma = eval.get_preprocessed_column((BlakeSigma::new(12)).id());
        let blakesigma = eval.get_preprocessed_column((BlakeSigma::new(13)).id());
        let blakesigma = eval.get_preprocessed_column((BlakeSigma::new(14)).id());
        let blakesigma = eval.get_preprocessed_column((BlakeSigma::new(15)).id());
        let blakesigma = eval.get_preprocessed_column((BlakeSigma::new(2)).id());
        let blakesigma = eval.get_preprocessed_column((BlakeSigma::new(3)).id());
        let blakesigma = eval.get_preprocessed_column((BlakeSigma::new(4)).id());
        let blakesigma = eval.get_preprocessed_column((BlakeSigma::new(5)).id());
        let blakesigma = eval.get_preprocessed_column((BlakeSigma::new(6)).id());
        let blakesigma = eval.get_preprocessed_column((BlakeSigma::new(7)).id());
        let blakesigma = eval.get_preprocessed_column((BlakeSigma::new(8)).id());
        let blakesigma = eval.get_preprocessed_column((BlakeSigma::new(9)).id());
        let seq = eval.get_preprocessed_column(Seq::new(self.log_size()).id());
        eval.add_to_relation(RelationEntry::new(
            &self.blake_round_sigma_lookup_elements,
            -E::EF::one(),
            &[
                seq.clone(),
                blakesigma.clone(),
                blakesigma.clone(),
                blakesigma.clone(),
                blakesigma.clone(),
                blakesigma.clone(),
                blakesigma.clone(),
                blakesigma.clone(),
                blakesigma.clone(),
                blakesigma.clone(),
                blakesigma.clone(),
                blakesigma.clone(),
                blakesigma.clone(),
                blakesigma.clone(),
                blakesigma.clone(),
                blakesigma.clone(),
                blakesigma.clone(),
            ],
        ));

        eval.finalize_logup_in_pairs();
        eval
    }
}
