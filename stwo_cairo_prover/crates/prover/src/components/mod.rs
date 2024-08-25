use num_traits::Zero;
use stwo_prover::constraint_framework::logup::LogupTraceGenerator;
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES};
use stwo_prover::core::backend::simd::qm31::PackedQM31;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::qm31::SecureField;
use stwo_prover::core::lookups::utils::Fraction;
use stwo_prover::core::pcs::TreeBuilder;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleChannel;

pub mod memory;
pub mod opcode;
pub mod range_check;
pub mod ret_opcode;
pub mod utils;

// TODO(ShaharS): Move to a common file.
pub const LOOKUP_INTERACTION_PHASE: usize = 1;

pub trait StandardLookupData {
    const N_LOOKUPS: usize;
    type Elements;
    fn lookups<'a, 'b>(&'a self, elements: &'a Self::Elements) -> Vec<LookupFunc<'a, 'b>>;
}
pub type LookupFunc<'a, 'b> = Box<dyn Fn(usize, &'b mut [Fraction<PackedM31, PackedQM31>]) + 'a>;

pub struct StandardInteractionProver<LD: StandardLookupData> {
    pub log_size: u32,
    pub lookup_data: Vec<LD>,
}
impl<LD: StandardLookupData> StandardInteractionProver<LD> {
    pub fn write_interaction_trace(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
        elements: &LD::Elements,
    ) -> StandardInteractionClaim {
        let mut logup_gen = LogupTraceGenerator::new(self.log_size);
        let mut lookups = self
            .lookup_data
            .into_iter()
            .flat_map(|ld| ld.lookups(elements));
        let it = lookups.array_chunks();

        // TODO: Do this in chunks.
        let mut fracs0 = vec![
            Fraction::new(PackedM31::zero(), PackedQM31::zero());
            1 << (self.log_size - LOG_N_LANES)
        ];
        let mut fracs1 = vec![
            Fraction::new(PackedM31::zero(), PackedQM31::zero());
            1 << (self.log_size - LOG_N_LANES)
        ];

        for [l0, l1] in it {
            let mut col_gen = logup_gen.new_col();
            l0(0, &mut fracs0);
            l1(1, &mut fracs1);
            for vec_row in 0..(1 << (self.log_size - LOG_N_LANES)) {
                let numerator = fracs1[vec_row].denominator * fracs0[vec_row].numerator
                    + fracs0[vec_row].denominator * fracs1[vec_row].numerator;
                let denom = fracs0[vec_row].denominator * fracs1[vec_row].denominator;
                col_gen.write_frac(vec_row, numerator, denom);
            }
            col_gen.finalize_col();
        }
        // TODO: Remainder.
        assert!(it.into_remainder().is_none());

        let (trace, claimed_sum) = logup_gen.finalize();
        tree_builder.extend_evals(trace);

        StandardInteractionClaim {
            log_size: self.log_size,
            claimed_sum,
        }
    }
}

#[derive(Clone)]
pub struct StandardInteractionClaim {
    pub log_size: u32,
    pub claimed_sum: SecureField,
}
impl StandardInteractionClaim {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_felts(&[self.claimed_sum]);
    }
}
