use anyhow::Result;
use cairo_vm::vm::runners::cairo_runner::CairoRunner;
use tracing::{info, span, Level};

use super::memory::{MemoryBuilder, MemoryConfig};
use super::ProverInput;
use crate::builtins::BuiltinSegments;
use crate::relocator::Relocator;
use crate::{PublicSegmentContext, StateTransitions};

pub fn adapt(runner: &CairoRunner) -> Result<ProverInput> {
    let _span = span!(Level::INFO, "adapt").entered();

    // Extract the relevant information from the Runner.
    let relocatable_trace = runner.get_relocatable_trace()?;
    info!("Num steps: {:?}", relocatable_trace.len());
    let mut relocatable_memory = runner.get_relocatable_memory();
    let public_memory_offsets = &runner.vm.segments.public_memory_offsets;
    let builtin_segments = runner.get_builtin_segments();

    // Relocation part.
    BuiltinSegments::pad_relocatble_builtin_segments(&mut relocatable_memory, &builtin_segments);
    let relocator = Relocator::new(&relocatable_memory);
    let relocated_memory = relocator.relocate_memory(&relocatable_memory);

    #[cfg(feature = "extract-mem-trace")]
    let relocated_memory_clone = relocated_memory.clone();

    let relocated_trace = relocator.relocate_trace(relocatable_trace);
    let builtin_segments = relocator.relocate_builtin_segments(&builtin_segments);
    info!("Builtin segments: {:?}", builtin_segments);
    let public_memory_addresses = relocator.relocate_public_addresses(public_memory_offsets);

    let memory = MemoryBuilder::from_iter(MemoryConfig::default(), relocated_memory);
    let state_transitions = StateTransitions::from_slice_parallel(&relocated_trace, &memory);
    info!(
        "Opcode counts: {:?}",
        state_transitions.casm_states_by_opcode.counts()
    );

    // TODO(spapini): Add output builtin to public memory.
    let (memory, inst_cache) = memory.build();

    // TODO(Ohad): take this from the input.
    let public_segment_context = PublicSegmentContext::bootloader_context();

    Ok(ProverInput {
        state_transitions,
        memory,
        pc_count: inst_cache.len(),
        public_memory_addresses,
        builtin_segments,
        public_segment_context,
        #[cfg(feature = "extract-mem-trace")]
        relocated_mem: relocated_memory_clone,
        #[cfg(feature = "extract-mem-trace")]
        relocated_trace: relocated_trace.clone(),
    })
}

#[cfg(test)]
#[cfg(feature = "slow-tests")]
mod tests {
    use std::fs::{read_to_string, File};
    use std::io::Write;

    use dev_utils::utils::get_compiled_cairo_program_path;
    use serde_json::{to_string_pretty, to_value};
    use stwo_cairo_utils::vm_utils::{run_and_adapt, ProgramType};

    use crate::test_utils::get_prover_input_path;

    fn test_compare_prover_input_to_expected_file(test_name: &str) {
        let is_fix_mode = std::env::var("FIX") == Ok("1".to_string());

        let compiled_program = get_compiled_cairo_program_path(test_name);
        let mut prover_input = run_and_adapt(&compiled_program, ProgramType::Json, None).unwrap();
        // Public memory addresses are not deterministic, sort them.
        prover_input.public_memory_addresses.sort();

        let prover_input_value =
            to_value(&prover_input).expect("Unable to convert prover input to value");

        let expected_prover_input_path = get_prover_input_path(test_name);
        if is_fix_mode {
            let mut file = File::create(&expected_prover_input_path).unwrap();
            write!(file, "{}", &to_string_pretty(&prover_input_value).unwrap()).unwrap();
        }
        let expected_prover_input: serde_json::Value =
            serde_json::from_str(&read_to_string(&expected_prover_input_path).unwrap()).unwrap();

        assert_eq!(
            prover_input_value,
            expected_prover_input,
            "Prover input from compiled cairo program: {test_name} doesn't match the expected prover input. To update prover input file, run the test with FIX=1."
        );
    }

    #[test]
    fn test_compare_prover_input_to_expected_file_all_opcodes() {
        test_compare_prover_input_to_expected_file("test_prove_verify_all_opcode_components");
    }

    #[test]
    fn test_compare_prover_input_to_expected_file_all_builtins() {
        test_compare_prover_input_to_expected_file("test_prove_verify_all_builtins");
    }
}
