use super::{LIMB_BITS, N_MULT_COLUMNS};
use crate::components::prelude::constraint_eval::*;

pub struct Eval {
    pub verify_bitwise_xor_12_lookup_elements: relations::VerifyBitwiseXor_12,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Claim {}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![super::LOG_SIZE; super::N_TRACE_COLUMNS];
        let interaction_log_sizes =
            vec![super::LOG_SIZE; SECURE_EXTENSION_DEGREE * N_MULT_COLUMNS.div_ceil(2)];
        TreeVec::new(vec![vec![], trace_log_sizes, interaction_log_sizes])
    }

    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_felts(&[]);
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
        super::LOG_SIZE
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_size() + 1
    }

    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        // al, bl are the constant columns for the inputs: All pairs of elements in [0,
        // 2^LIMB_BITS).
        // cl is the constant column for the xor: al ^ bl.
        let al = eval.get_preprocessed_column(BitwiseXor::new(LIMB_BITS, 0).id());
        let bl = eval.get_preprocessed_column(BitwiseXor::new(LIMB_BITS, 1).id());
        let cl = eval.get_preprocessed_column(BitwiseXor::new(LIMB_BITS, 2).id());

        for i in 0..1 << super::EXPAND_BITS {
            for j in 0..1 << super::EXPAND_BITS {
                let multiplicity = eval.next_trace_mask();

                let a = al.clone() + E::F::from(M31(i << LIMB_BITS));
                let b = bl.clone() + E::F::from(M31(j << LIMB_BITS));
                let c = cl.clone() + E::F::from(M31((i ^ j) << LIMB_BITS));

                eval.add_to_relation(RelationEntry::new(
                    &self.verify_bitwise_xor_12_lookup_elements,
                    -E::EF::from(multiplicity),
                    &[a, b, c],
                ));
            }
        }

        eval.finalize_logup_in_pairs();
        eval
    }
}
