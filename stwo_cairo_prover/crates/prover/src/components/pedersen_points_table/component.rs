use crate::components::prelude::constraint_eval::*;

pub struct Eval {
    pub claim: Claim,
    pub pedersen_points_table_lookup_elements: relations::PedersenPointsTable,
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
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(0)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(1)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(10)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(11)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(12)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(13)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(14)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(15)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(16)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(17)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(18)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(19)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(2)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(20)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(21)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(22)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(23)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(24)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(25)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(26)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(27)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(28)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(29)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(3)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(30)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(31)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(32)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(33)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(34)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(35)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(36)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(37)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(38)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(39)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(4)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(40)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(41)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(42)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(43)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(44)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(45)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(46)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(47)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(48)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(49)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(5)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(50)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(51)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(52)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(53)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(54)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(55)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(6)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(7)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(8)).id());
        let pedersenpoints = eval.get_preprocessed_column((PedersenPoints::new(9)).id());
        let seq = eval.get_preprocessed_column(Seq::new(self.log_size()).id());
        eval.add_to_relation(RelationEntry::new(
            &self.pedersen_points_table_lookup_elements,
            -E::EF::one(),
            &[
                seq.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
                pedersenpoints.clone(),
            ],
        ));

        eval.finalize_logup_in_pairs();
        eval
    }
}
