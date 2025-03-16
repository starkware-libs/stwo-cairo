#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use crate::cairo_air::pedersen::const_columns::PEDERSEN_TABLE_N_COLUMNS;
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
        let trace_log_sizes = vec![self.log_size; 1];
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
        let pedersen_points_array: [_; PEDERSEN_TABLE_N_COLUMNS] =
            std::array::from_fn(|i| eval.get_preprocessed_column(PedersenPoints::new(i).id()));
        let seq = eval.get_preprocessed_column(Seq::new(self.log_size()).id());
        let multiplicity = eval.next_trace_mask();

        eval.add_to_relation(RelationEntry::new(
            &self.pedersen_points_table_lookup_elements,
            -E::EF::from(multiplicity),
            &[
                seq.clone(),
                pedersen_points_array[0].clone(),
                pedersen_points_array[1].clone(),
                pedersen_points_array[2].clone(),
                pedersen_points_array[3].clone(),
                pedersen_points_array[4].clone(),
                pedersen_points_array[5].clone(),
                pedersen_points_array[6].clone(),
                pedersen_points_array[7].clone(),
                pedersen_points_array[8].clone(),
                pedersen_points_array[9].clone(),
                pedersen_points_array[10].clone(),
                pedersen_points_array[11].clone(),
                pedersen_points_array[12].clone(),
                pedersen_points_array[13].clone(),
                pedersen_points_array[14].clone(),
                pedersen_points_array[15].clone(),
                pedersen_points_array[16].clone(),
                pedersen_points_array[17].clone(),
                pedersen_points_array[18].clone(),
                pedersen_points_array[19].clone(),
                pedersen_points_array[20].clone(),
                pedersen_points_array[21].clone(),
                pedersen_points_array[22].clone(),
                pedersen_points_array[23].clone(),
                pedersen_points_array[24].clone(),
                pedersen_points_array[25].clone(),
                pedersen_points_array[26].clone(),
                pedersen_points_array[27].clone(),
                pedersen_points_array[28].clone(),
                pedersen_points_array[29].clone(),
                pedersen_points_array[30].clone(),
                pedersen_points_array[31].clone(),
                pedersen_points_array[32].clone(),
                pedersen_points_array[33].clone(),
                pedersen_points_array[34].clone(),
                pedersen_points_array[35].clone(),
                pedersen_points_array[36].clone(),
                pedersen_points_array[37].clone(),
                pedersen_points_array[38].clone(),
                pedersen_points_array[39].clone(),
                pedersen_points_array[40].clone(),
                pedersen_points_array[41].clone(),
                pedersen_points_array[42].clone(),
                pedersen_points_array[43].clone(),
                pedersen_points_array[44].clone(),
                pedersen_points_array[45].clone(),
                pedersen_points_array[46].clone(),
                pedersen_points_array[47].clone(),
                pedersen_points_array[48].clone(),
                pedersen_points_array[49].clone(),
                pedersen_points_array[50].clone(),
                pedersen_points_array[51].clone(),
                pedersen_points_array[52].clone(),
                pedersen_points_array[53].clone(),
                pedersen_points_array[54].clone(),
                pedersen_points_array[55].clone(),
            ],
        ));

        eval.finalize_logup_in_pairs();
        eval
    }
}
