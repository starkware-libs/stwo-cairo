use stwo_prover::constraint_framework::logup::LookupElements;

pub mod component;
pub mod component_prover;

// TODO(Ohad): figure out n_alpha_powers.
pub type RangeCheckVectorElements = LookupElements<1>;
