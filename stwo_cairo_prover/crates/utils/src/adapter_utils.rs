use cairo_vm::vm::runners::cairo_runner::CairoRunner;
use stwo_cairo_prover::input::plain::input_from_finished_runner;
use stwo_cairo_prover::input::CairoInput;

// TODO(yuval): consider directly calling `input_from_finished_runner` (and add `span` there) and
// removing this function+module.
/// Adapts the Cairo VM output to the input of Stwo.
/// Assumes memory and trace are already relocated. Otherwise panics.
pub fn adapt_vm_output_to_stwo(runner: CairoRunner) -> CairoInput {
    let _span = tracing::info_span!("adapt_vm_output_to_stwo").entered();
    input_from_finished_runner(runner, false)
}
