use std::ops::Deref;
use std::sync::Arc;

use cairo_air::cairo_components::{assert_cairo_components, CairoComponents};
use cairo_air::relations::CommonLookupElements;
use itertools::Itertools;
use stwo::core::channel::Blake2sChannel;
use stwo::core::fields::m31::M31;
use stwo::core::pcs::TreeVec;
use stwo_cairo_adapter::ProverInput;
use stwo_cairo_common::preprocessed_columns::preprocessed_trace::PreProcessedTrace;
use stwo_constraint_framework::{
    assert_constraints_on_trace, FrameworkComponent, FrameworkEval, PREPROCESSED_TRACE_IDX,
};

use crate::debug_tools::mock_tree_builder::MockCommitmentScheme;
use crate::witness::cairo::create_cairo_claim_generator;
use crate::witness::preprocessed_trace::gen_trace;

pub fn assert_component<E: FrameworkEval + Sync>(
    component: &FrameworkComponent<E>,
    trace: &TreeVec<Vec<&Vec<M31>>>,
) {
    let mut component_trace = trace
        .sub_tree(component.trace_locations())
        .map(|tree| tree.into_iter().cloned().collect_vec());
    component_trace[PREPROCESSED_TRACE_IDX] = component
        .preprocessed_column_indices()
        .iter()
        .map(|idx| trace[PREPROCESSED_TRACE_IDX][*idx])
        .collect();

    let log_size = component.log_size();

    let component_eval = component.deref();
    assert_constraints_on_trace(
        &component_trace,
        log_size,
        |eval| {
            component_eval.evaluate(eval);
        },
        component.claimed_sum(),
    );
}

pub fn assert_cairo_constraints(input: ProverInput, preprocessed_trace: Arc<PreProcessedTrace>) {
    let mut commitment_scheme = MockCommitmentScheme::default();

    // Preprocessed trace.
    let mut tree_builder = commitment_scheme.tree_builder();
    tree_builder.extend_evals(gen_trace(preprocessed_trace.clone()));
    tree_builder.finalize_interaction();

    // Base trace.
    let cairo_claim_generator = create_cairo_claim_generator(input, preprocessed_trace.clone());
    let mut tree_builder = commitment_scheme.tree_builder();
    let (claim, interaction_generator) = cairo_claim_generator.write_trace(&mut tree_builder);
    tree_builder.finalize_interaction();

    // Interaction trace.
    let mut dummy_channel = Blake2sChannel::default();
    let interaction_elements = CommonLookupElements::draw(&mut dummy_channel);
    let mut tree_builder = commitment_scheme.tree_builder();
    let interaction_claim =
        interaction_generator.write_interaction_trace(&mut tree_builder, &interaction_elements);
    tree_builder.finalize_interaction();

    let components = CairoComponents::new(
        &claim,
        &interaction_elements,
        &interaction_claim,
        &preprocessed_trace.ids(),
    );

    assert_cairo_components(commitment_scheme.trace_domain_evaluations(), &components);
}
