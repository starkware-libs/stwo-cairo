use crate::components::prelude::*;
pub const ELEM_BITS: u32 = 12;
pub const EXPAND_BITS: u32 = 2;
pub const LIMB_BITS: u32 = ELEM_BITS - EXPAND_BITS;
pub const LOG_SIZE: u32 = (ELEM_BITS - EXPAND_BITS) * 2;
pub const N_MULT_COLUMNS: usize = 1 << (EXPAND_BITS * 2);
pub const N_TRACE_COLUMNS: usize = N_MULT_COLUMNS;

pub struct Eval {
    pub verify_bitwise_xor_12_lookup_elements: relations::VerifyBitwiseXor_12,
}

#[derive(Copy, Clone, Serialize, Deserialize, CairoSerialize)]
pub struct Claim {}
impl Claim {
    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let trace_log_sizes = vec![LOG_SIZE; N_TRACE_COLUMNS];
        let interaction_log_sizes =
            vec![LOG_SIZE; SECURE_EXTENSION_DEGREE * N_MULT_COLUMNS.div_ceil(2)];
        TreeVec::new(vec![vec![], trace_log_sizes, interaction_log_sizes])
    }

    pub fn mix_into(&self, _channel: &mut impl Channel) {
        // TODO(Ohad): Think about this.
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
        LOG_SIZE
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_size() + 1
    }

    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        // al, bl are the constant columns for the inputs: All pairs of elements in [0,
        // 2^LIMB_BITS).
        // cl is the constant column for the xor: al ^ bl.
        let a_low = eval.get_preprocessed_column(BitwiseXor::new(LIMB_BITS, 0).id());
        let b_low = eval.get_preprocessed_column(BitwiseXor::new(LIMB_BITS, 1).id());
        let c_low = eval.get_preprocessed_column(BitwiseXor::new(LIMB_BITS, 2).id());

        for i in 0..1 << EXPAND_BITS {
            for j in 0..1 << EXPAND_BITS {
                let multiplicity = eval.next_trace_mask();

                let a = a_low.clone() + E::F::from(M31(i << LIMB_BITS));
                let b = b_low.clone() + E::F::from(M31(j << LIMB_BITS));
                let c = c_low.clone() + E::F::from(M31((i ^ j) << LIMB_BITS));

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
