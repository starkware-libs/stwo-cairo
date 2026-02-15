use cairo_air::air::PublicData;
use cairo_air::cairo_components::{cairo_relation_entries, CairoComponents};
use itertools::Itertools;
use num_traits::{One, Zero};
use stwo::core::channel::MerkleChannel;
use stwo::core::fields::m31::{BaseField, M31};
use stwo::core::poly::circle::CanonicCoset;
use stwo::prover::backend::simd::SimdBackend;
use stwo::prover::backend::{BackendForChannel, Col, Column};
use stwo::prover::poly::circle::CircleCoefficients;
use stwo::prover::CommitmentSchemeProver;
use stwo_cairo_common::prover_types::felt::split_f252;
use stwo_constraint_framework::relation_tracker::{RelationSummary, RelationTrackerEntry};

pub fn track_and_summarize_cairo_relations<MC: MerkleChannel>(
    commitment_scheme: &CommitmentSchemeProver<'_, SimdBackend, MC>,
    components: &CairoComponents,
    public_data: &PublicData,
) -> RelationSummary
where
    SimdBackend: BackendForChannel<MC>,
{
    let entries = track_cairo_relations(commitment_scheme, components, public_data);
    RelationSummary::summarize_relations(&entries).cleaned()
}

pub fn track_cairo_relations<MC: MerkleChannel>(
    commitment_scheme: &CommitmentSchemeProver<'_, SimdBackend, MC>,
    components: &CairoComponents,
    public_data: &PublicData,
) -> Vec<RelationTrackerEntry>
where
    SimdBackend: BackendForChannel<MC>,
{
    // Cairo air aggregates interpolated polynomials. Evaluate to get the original trace.
    // NOTE: this process is slow, and should be only used for debugging.
    // TODO(Ohad): skip lde and merkle.
    let evals = commitment_scheme.trace().polys.map(|interaction_tree| {
        interaction_tree
            .iter()
            .map(|poly| {
                let coeffs = reduce_degree(poly.evals.clone().interpolate());
                coeffs
                    .evaluate(CanonicCoset::new(coeffs.log_size()).circle_domain())
                    .values
                    .to_cpu()
            })
            .collect_vec()
    });
    let evals = &evals.as_ref();
    let trace = &evals.into();

    let mut entries = cairo_relation_entries(components, trace);

    // Public data.
    let initial_pc = public_data.initial_state.pc.0;
    let initial_ap = public_data.initial_state.ap.0;
    let final_ap = public_data.final_state.ap.0;
    public_data
        .public_memory
        .get_entries(initial_pc, initial_ap, final_ap)
        .for_each(|(addr, id, val)| {
            entries.push(RelationTrackerEntry {
                relation: "MemoryAddressToId".to_string(),
                mult: M31::one(),
                values: vec![M31::from_u32_unchecked(addr), M31::from_u32_unchecked(id)],
            });
            entries.push(RelationTrackerEntry {
                relation: "MemoryIdToBig".to_string(),
                mult: M31::one(),
                values: [
                    [M31::from_u32_unchecked(id)].as_slice(),
                    split_f252(val).as_slice(),
                ]
                .concat(),
            });
        });
    entries.push(RelationTrackerEntry {
        relation: "Opcodes".to_string(),
        mult: M31::one(),
        values: public_data.final_state.values().to_vec(),
    });
    entries.push(RelationTrackerEntry {
        relation: "Opcodes".to_string(),
        mult: -M31::one(),
        values: public_data.initial_state.values().to_vec(),
    });

    entries
}

/// Reduces the polynomial to a minimal degree polynomial that evaluates to the same values.
pub fn reduce_degree(coeffs: CircleCoefficients<SimdBackend>) -> CircleCoefficients<SimdBackend> {
    let mut new_log_size = coeffs.log_size();
    while new_log_size > 1 {
        if ((1 << (new_log_size - 1))..(1 << new_log_size))
            .any(|i| coeffs.coeffs.at(i) != BaseField::zero())
        {
            break;
        }
        new_log_size -= 1;
    }
    CircleCoefficients::new(Col::<SimdBackend, BaseField>::from_iter(
        coeffs.coeffs.to_cpu()[..1 << new_log_size].iter().copied(),
    ))
}
