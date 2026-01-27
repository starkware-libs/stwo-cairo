use cairo_air::range_checks_air::RangeChecksClaim;
use stwo::core::fields::m31::BaseField;
use stwo::prover::backend::simd::SimdBackend;
use stwo::prover::poly::circle::CircleEvaluation;
use stwo::prover::poly::BitReversedOrder;

use crate::witness::components::{
    range_check_11, range_check_12, range_check_18, range_check_20, range_check_3_3_3_3_3,
    range_check_3_6_6_3, range_check_4_3, range_check_4_4, range_check_4_4_4_4, range_check_6,
    range_check_7_2_5, range_check_8, range_check_9_9,
};
use crate::witness::utils::TreeBuilder;

/// Type alias for trace evaluations.
pub type TraceEval = Vec<CircleEvaluation<SimdBackend, BaseField, BitReversedOrder>>;

/// Holds generated traces for all range check components.
#[derive(Default)]
pub struct RangeChecksTraces {
    pub rc_6: Option<TraceEval>,
    pub rc_8: Option<TraceEval>,
    pub rc_11: Option<TraceEval>,
    pub rc_12: Option<TraceEval>,
    pub rc_18: Option<TraceEval>,
    pub rc_20: Option<TraceEval>,
    pub rc_4_3: Option<TraceEval>,
    pub rc_4_4: Option<TraceEval>,
    pub rc_9_9: Option<TraceEval>,
    pub rc_7_2_5: Option<TraceEval>,
    pub rc_3_6_6_3: Option<TraceEval>,
    pub rc_4_4_4_4: Option<TraceEval>,
    pub rc_3_3_3_3_3: Option<TraceEval>,
}

pub fn get_range_checks() -> Vec<&'static str> {
    vec![
        "range_check_6",
        "range_check_8",
        "range_check_11",
        "range_check_12",
        "range_check_18",
        "range_check_20",
        "range_check_4_3",
        "range_check_4_4",
        "range_check_9_9",
        "range_check_7_2_5",
        "range_check_3_6_6_3",
        "range_check_4_4_4_4",
        "range_check_3_3_3_3_3",
    ]
}

/// Generates range check traces without extending to tree_builder.
/// Returns the traces, claims, and interaction generators.
pub fn range_checks_generate_traces(
    rc_6_trace_generator: Option<range_check_6::ClaimGenerator>,
    rc_8_trace_generator: Option<range_check_8::ClaimGenerator>,
    rc_11_trace_generator: Option<range_check_11::ClaimGenerator>,
    rc_12_trace_generator: Option<range_check_12::ClaimGenerator>,
    rc_18_trace_generator: Option<range_check_18::ClaimGenerator>,
    rc_20_trace_generator: Option<range_check_20::ClaimGenerator>,
    rc_4_3_trace_generator: Option<range_check_4_3::ClaimGenerator>,
    rc_4_4_trace_generator: Option<range_check_4_4::ClaimGenerator>,
    rc_9_9_trace_generator: Option<range_check_9_9::ClaimGenerator>,
    rc_7_2_5_trace_generator: Option<range_check_7_2_5::ClaimGenerator>,
    rc_3_6_6_3_trace_generator: Option<range_check_3_6_6_3::ClaimGenerator>,
    rc_4_4_4_4_trace_generator: Option<range_check_4_4_4_4::ClaimGenerator>,
    rc_3_3_3_3_3_trace_generator: Option<range_check_3_3_3_3_3::ClaimGenerator>,
) -> (
    RangeChecksTraces,
    RangeChecksClaim,
    RangeChecksInteractionClaimGenerator,
) {
    let (rc_6_trace, rc_6_claim, rc_6_interaction_gen) =
        rc_6_trace_generator.unwrap().write_trace();
    let (rc_8_trace, rc_8_claim, rc_8_interaction_gen) =
        rc_8_trace_generator.unwrap().write_trace();
    let (rc_11_trace, rc_11_claim, rc_11_interaction_gen) =
        rc_11_trace_generator.unwrap().write_trace();
    let (rc_12_trace, rc_12_claim, rc_12_interaction_gen) =
        rc_12_trace_generator.unwrap().write_trace();
    let (rc_18_trace, rc_18_claim, rc_18_interaction_gen) =
        rc_18_trace_generator.unwrap().write_trace();
    let (rc_20_trace, rc_20_claim, rc_20_interaction_gen) =
        rc_20_trace_generator.unwrap().write_trace();
    let (rc_4_3_trace, rc_4_3_claim, rc_4_3_interaction_gen) =
        rc_4_3_trace_generator.unwrap().write_trace();
    let (rc_4_4_trace, rc_4_4_claim, rc_4_4_interaction_gen) =
        rc_4_4_trace_generator.unwrap().write_trace();
    let (rc_9_9_trace, rc_9_9_claim, rc_9_9_interaction_gen) =
        rc_9_9_trace_generator.unwrap().write_trace();
    let (rc_7_2_5_trace, rc_7_2_5_claim, rc_7_2_5_interaction_gen) =
        rc_7_2_5_trace_generator.unwrap().write_trace();
    let (rc_3_6_6_3_trace, rc_3_6_6_3_claim, rc_3_6_6_3_interaction_gen) =
        rc_3_6_6_3_trace_generator.unwrap().write_trace();
    let (rc_4_4_4_4_trace, rc_4_4_4_4_claim, rc_4_4_4_4_interaction_gen) =
        rc_4_4_4_4_trace_generator.unwrap().write_trace();
    let (rc_3_3_3_3_3_trace, rc_3_3_3_3_3_claim, rc_3_3_3_3_3_interaction_gen) =
        rc_3_3_3_3_3_trace_generator.unwrap().write_trace();

    let traces = RangeChecksTraces {
        rc_6: Some(rc_6_trace.to_evals()),
        rc_8: Some(rc_8_trace.to_evals()),
        rc_11: Some(rc_11_trace.to_evals()),
        rc_12: Some(rc_12_trace.to_evals()),
        rc_18: Some(rc_18_trace.to_evals()),
        rc_20: Some(rc_20_trace.to_evals()),
        rc_4_3: Some(rc_4_3_trace.to_evals()),
        rc_4_4: Some(rc_4_4_trace.to_evals()),
        rc_9_9: Some(rc_9_9_trace.to_evals()),
        rc_7_2_5: Some(rc_7_2_5_trace.to_evals()),
        rc_3_6_6_3: Some(rc_3_6_6_3_trace.to_evals()),
        rc_4_4_4_4: Some(rc_4_4_4_4_trace.to_evals()),
        rc_3_3_3_3_3: Some(rc_3_3_3_3_3_trace.to_evals()),
    };

    let claim = RangeChecksClaim {
        rc_6: rc_6_claim,
        rc_8: rc_8_claim,
        rc_11: rc_11_claim,
        rc_12: rc_12_claim,
        rc_18: rc_18_claim,
        rc_20: rc_20_claim,
        rc_4_3: rc_4_3_claim,
        rc_4_4: rc_4_4_claim,
        rc_9_9: rc_9_9_claim,
        rc_7_2_5: rc_7_2_5_claim,
        rc_3_6_6_3: rc_3_6_6_3_claim,
        rc_4_4_4_4: rc_4_4_4_4_claim,
        rc_3_3_3_3_3: rc_3_3_3_3_3_claim,
    };

    let interaction_gen = RangeChecksInteractionClaimGenerator {
        rc_6_interaction_gen,
        rc_8_interaction_gen,
        rc_11_interaction_gen,
        rc_12_interaction_gen,
        rc_18_interaction_gen,
        rc_20_interaction_gen,
        rc_4_3_interaction_gen,
        rc_4_4_interaction_gen,
        rc_9_9_interaction_gen,
        rc_7_2_5_interaction_gen,
        rc_3_6_6_3_interaction_gen,
        rc_4_4_4_4_interaction_gen,
        rc_3_3_3_3_3_interaction_gen,
    };

    (traces, claim, interaction_gen)
}

/// Extends the tree builder with range check traces in the correct order.
pub fn extend_range_checks_traces(
    tree_builder: &mut impl TreeBuilder<SimdBackend>,
    traces: RangeChecksTraces,
) {
    if let Some(trace) = traces.rc_6 {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = traces.rc_8 {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = traces.rc_11 {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = traces.rc_12 {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = traces.rc_18 {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = traces.rc_20 {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = traces.rc_4_3 {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = traces.rc_4_4 {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = traces.rc_9_9 {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = traces.rc_7_2_5 {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = traces.rc_3_6_6_3 {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = traces.rc_4_4_4_4 {
        tree_builder.extend_evals(trace);
    }
    if let Some(trace) = traces.rc_3_3_3_3_3 {
        tree_builder.extend_evals(trace);
    }
}

/// Legacy function that generates range check traces and extends them to tree_builder.
/// Maintained for backward compatibility.
pub fn range_checks_write_trace(
    rc_6_trace_generator: Option<range_check_6::ClaimGenerator>,
    rc_8_trace_generator: Option<range_check_8::ClaimGenerator>,
    rc_11_trace_generator: Option<range_check_11::ClaimGenerator>,
    rc_12_trace_generator: Option<range_check_12::ClaimGenerator>,
    rc_18_trace_generator: Option<range_check_18::ClaimGenerator>,
    rc_20_trace_generator: Option<range_check_20::ClaimGenerator>,
    rc_4_3_trace_generator: Option<range_check_4_3::ClaimGenerator>,
    rc_4_4_trace_generator: Option<range_check_4_4::ClaimGenerator>,
    rc_9_9_trace_generator: Option<range_check_9_9::ClaimGenerator>,
    rc_7_2_5_trace_generator: Option<range_check_7_2_5::ClaimGenerator>,
    rc_3_6_6_3_trace_generator: Option<range_check_3_6_6_3::ClaimGenerator>,
    rc_4_4_4_4_trace_generator: Option<range_check_4_4_4_4::ClaimGenerator>,
    rc_3_3_3_3_3_trace_generator: Option<range_check_3_3_3_3_3::ClaimGenerator>,
    tree_builder: &mut impl TreeBuilder<SimdBackend>,
) -> (RangeChecksClaim, RangeChecksInteractionClaimGenerator) {
    let (traces, claim, interaction_gen) = range_checks_generate_traces(
        rc_6_trace_generator,
        rc_8_trace_generator,
        rc_11_trace_generator,
        rc_12_trace_generator,
        rc_18_trace_generator,
        rc_20_trace_generator,
        rc_4_3_trace_generator,
        rc_4_4_trace_generator,
        rc_9_9_trace_generator,
        rc_7_2_5_trace_generator,
        rc_3_6_6_3_trace_generator,
        rc_4_4_4_4_trace_generator,
        rc_3_3_3_3_3_trace_generator,
    );
    extend_range_checks_traces(tree_builder, traces);
    (claim, interaction_gen)
}

pub struct RangeChecksInteractionClaimGenerator {
    pub rc_6_interaction_gen: range_check_6::InteractionClaimGenerator,
    pub rc_8_interaction_gen: range_check_8::InteractionClaimGenerator,
    pub rc_11_interaction_gen: range_check_11::InteractionClaimGenerator,
    pub rc_12_interaction_gen: range_check_12::InteractionClaimGenerator,
    pub rc_18_interaction_gen: range_check_18::InteractionClaimGenerator,
    pub rc_20_interaction_gen: range_check_20::InteractionClaimGenerator,
    pub rc_4_3_interaction_gen: range_check_4_3::InteractionClaimGenerator,
    pub rc_4_4_interaction_gen: range_check_4_4::InteractionClaimGenerator,
    pub rc_9_9_interaction_gen: range_check_9_9::InteractionClaimGenerator,
    pub rc_7_2_5_interaction_gen: range_check_7_2_5::InteractionClaimGenerator,
    pub rc_3_6_6_3_interaction_gen: range_check_3_6_6_3::InteractionClaimGenerator,
    pub rc_4_4_4_4_interaction_gen: range_check_4_4_4_4::InteractionClaimGenerator,
    pub rc_3_3_3_3_3_interaction_gen: range_check_3_3_3_3_3::InteractionClaimGenerator,
}
