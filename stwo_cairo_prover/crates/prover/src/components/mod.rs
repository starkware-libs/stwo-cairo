use itertools::{chain, Itertools};
use num_traits::One;
use stwo_prover::constraint_framework::logup::LogupTraceGenerator;
use stwo_prover::core::backend::simd::column::BaseColumn;
use stwo_prover::core::backend::simd::m31::{PackedM31, LOG_N_LANES, N_LANES};
use stwo_prover::core::backend::simd::qm31::PackedQM31;
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::backend::Column;
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

use stwo_prover::constraint_framework::logup::LogupAtRow;
use stwo_prover::constraint_framework::{EvalAtRow, FrameworkComponent, InfoEvaluator};
use stwo_prover::core::fields::secure_column::SECURE_EXTENSION_DEGREE;
use stwo_prover::core::pcs::TreeVec;
use utils::to_evals;

// Trait.
pub trait Standard: Clone {
    type LookupElements;
    type Context<'a>;
    type PackedInput;
    type LookupData: StandardLookupData;

    const N_REPETITIONS: usize;

    fn n_columns() -> usize {
        Self::info().mask_offsets[0].len()
    }
    fn info() -> InfoEvaluator {
        let mut logup = LogupAtRow::<2, InfoEvaluator>::new(1, SecureField::one(), 1);
        let mut eval = InfoEvaluator::default();
        Self::evaluate(&mut eval, &mut logup, &Self::dummy_elements());
        logup.finalize(&mut eval);
        eval
    }
    fn dummy_elements() -> Self::LookupElements;
    fn new_lookup_data(log_size: u32) -> Self::LookupData;
    fn evaluate<E: EvalAtRow>(
        eval: &mut E,
        logup: &mut LogupAtRow<2, E>,
        elements: &Self::LookupElements,
    );
    fn write_trace_row(
        dst: &mut [BaseColumn],
        input: &Self::PackedInput,
        row_index: usize,
        ctx: &mut Self::Context<'_>,
        lookup_data: &mut Self::LookupData,
    );
}

// Component.
#[derive(Clone)]
pub struct StandardComponent<C: Standard> {
    pub log_size: u32,
    pub opcode_elements: C::LookupElements,
    pub claimed_sum: SecureField,
    phantom: std::marker::PhantomData<C>,
}
impl<C: Standard> StandardComponent<C> {
    pub fn new(
        ret_claim: StandardClaim<C>,
        opcode_elements: C::LookupElements,
        interaction_claim: StandardInteractionClaim,
    ) -> Self {
        Self {
            log_size: ret_claim.log_size,
            opcode_elements,
            claimed_sum: interaction_claim.claimed_sum,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<C: Standard> FrameworkComponent for StandardComponent<C> {
    fn log_size(&self) -> u32 {
        self.log_size
    }

    fn max_constraint_log_degree_bound(&self) -> u32 {
        self.log_size + 1
    }

    fn evaluate<E: EvalAtRow>(&self, mut eval: E) -> E {
        let mut logup = LogupAtRow::<2, E>::new(1, self.claimed_sum, self.log_size);

        for _ in 0..C::N_REPETITIONS {
            C::evaluate(&mut eval, &mut logup, &self.opcode_elements);
        }

        logup.finalize(&mut eval);
        eval
    }
}

#[derive(Clone)]
pub struct StandardClaim<C: Standard> {
    pub log_size: u32,
    pub n_instances: usize,
    pub phantom: std::marker::PhantomData<C>,
}
impl<C: Standard> StandardClaim<C> {
    pub fn mix_into(&self, channel: &mut impl Channel) {
        channel.mix_nonce(self.log_size as u64);
        channel.mix_nonce(self.n_instances as u64);
    }

    pub fn log_sizes(&self) -> TreeVec<Vec<u32>> {
        let interaction_0_log_sizes = vec![self.log_size; C::n_columns() * C::N_REPETITIONS];
        let interaction_1_log_sizes =
            vec![
                self.log_size;
                SECURE_EXTENSION_DEGREE * (C::LookupData::N_LOOKUPS * C::N_REPETITIONS).div_ceil(2)
            ];
        let log_sizes = TreeVec::new(vec![interaction_0_log_sizes, interaction_1_log_sizes]);

        debug_assert_eq!(
            C::info().mask_offsets.map(|x| x.len())[..2],
            log_sizes.as_ref().map(|x| x.len())[..2]
        );

        log_sizes
    }
}

// Prover.
pub struct StandardProver<C: Standard> {
    pub inputs: Vec<C::PackedInput>,
    pub n_instances: usize,
    pub phantom: std::marker::PhantomData<C>,
}
impl<C: Standard> StandardProver<C> {
    pub fn new<Input: Clone>(inputs: impl ExactSizeIterator<Item = Input>) -> Vec<Self>
    where
        C::PackedInput: From<[Input; N_LANES]>,
    {
        // TODO(spapini): Split to multiple components.
        let n_instances = inputs.len();
        let n_rows = n_instances
            .div_ceil(C::N_REPETITIONS)
            .next_power_of_two()
            .max(64);

        // Pad the input vector to a power of 2.
        let new_size = n_rows * C::N_REPETITIONS;
        let mut inputs = inputs.peekable();
        let pad = inputs.peek().unwrap().clone();
        let packed_inputs = chain![inputs, std::iter::repeat(pad).take(new_size - n_instances)]
            .array_chunks::<N_LANES>()
            .map(C::PackedInput::from)
            .collect();

        vec![Self {
            inputs: packed_inputs,
            n_instances,
            phantom: std::marker::PhantomData,
        }]
    }
    pub fn write_trace(
        self,
        tree_builder: &mut TreeBuilder<'_, '_, SimdBackend, Blake2sMerkleChannel>,
        ctx: &mut C::Context<'_>,
    ) -> (StandardClaim<C>, StandardInteractionProver<C::LookupData>) {
        let inputs = self.inputs;
        let n_cols = C::n_columns();
        assert_eq!(inputs.len() % C::N_REPETITIONS, 0);
        let n_rows = inputs.len() / C::N_REPETITIONS * N_LANES;
        let mut trace_values = (0..(n_cols * C::N_REPETITIONS))
            .map(|_| BaseColumn::zeros(n_rows))
            .collect_vec();

        let log_size = n_rows.ilog2();
        let mut lookup_data = (0..C::N_REPETITIONS)
            .map(|_| C::new_lookup_data(log_size))
            .collect_vec();

        for (row_index, inputs) in inputs.chunks(C::N_REPETITIONS).enumerate() {
            for i in 0..C::N_REPETITIONS {
                let offset = n_cols * i;
                // Write opcode columns.
                C::write_trace_row(
                    &mut trace_values[offset..(offset + n_cols)],
                    &inputs[i],
                    row_index,
                    ctx,
                    &mut lookup_data[i],
                );
            }
        }

        tree_builder.extend_evals(to_evals(trace_values));
        let claim = StandardClaim {
            log_size: inputs.len().ilog2() + LOG_N_LANES,
            n_instances: self.n_instances,
            phantom: std::marker::PhantomData,
        };

        (
            claim,
            StandardInteractionProver {
                log_size,
                lookup_data: lookup_data.into_iter().collect_vec(),
            },
        )
    }
}

pub trait StandardLookupData {
    const N_LOOKUPS: usize;
    type Elements;
    fn lookups<'a>(&'a self, elements: &'a Self::Elements) -> Vec<LookupFunc<'a>>;
}
pub type LookupFunc<'a> = Box<dyn Iterator<Item = Fraction<PackedM31, PackedQM31>> + 'a>;

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

        let lookups = self.lookup_data.iter().flat_map(|ld| {
            let lookups = ld.lookups(elements);
            assert_eq!(lookups.len(), LD::N_LOOKUPS);
            lookups
        });
        let mut it = lookups.array_chunks();
        for [l0, l1] in &mut it {
            let mut col_gen = logup_gen.new_col();
            for (vec_row, (f0, f1)) in l0.zip(l1).enumerate() {
                let f = f0 + f1;
                col_gen.write_frac(vec_row, f.numerator, f.denominator);
            }
            col_gen.finalize_col();
        }
        // TODO: Remainder.
        let remainder = it.into_remainder();
        assert!(remainder.map(|x| x.count() == 0).unwrap_or(true));

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
