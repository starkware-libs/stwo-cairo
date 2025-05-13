use std::path::Path;

use cairo_vm::vm::runners::cairo_runner::{CairoRunner, ProverInputInfo};
use tracing::{span, Level};

use super::memory::{MemoryBuilder, MemoryConfig};
use super::vm_import::VmImportError;
use super::ProverInput;
use crate::builtins::BuiltinSegments;
use crate::relocator::Relocator;
use crate::test_utils::read_prover_input_info_file;
use crate::StateTransitions;

/// Translates a CairoRunner that finished its run into a ProverInput by extracting the relevant
/// input to the adapter.
/// When dev mod is enabled, the opcodes generated from the plain casm will be mapped to the generic
/// component only.
pub fn adapt_finished_runner(runner: CairoRunner) -> Result<ProverInput, VmImportError> {
    let _span = tracing::info_span!("adapt_finished_runner").entered();

    let mut prover_input_info = runner
        .get_prover_input_info()
        .expect("Unable to get prover input info");

    adapter(&mut prover_input_info)
}

pub fn adapter(prover_input_info: &mut ProverInputInfo) -> Result<ProverInput, VmImportError> {
    BuiltinSegments::pad_relocatble_builtin_segments(
        &mut prover_input_info.relocatable_memory,
        prover_input_info.builtins_segments.clone(),
    );
    let relocator = Relocator::new(
        prover_input_info.relocatable_memory.clone(),
        prover_input_info.builtins_segments.clone(),
    );

    let relocated_memory = relocator.get_relocated_memory();
    let relocated_trace = relocator.relocate_trace(&prover_input_info.relocatable_trace);

    let memory = MemoryBuilder::from_iter(MemoryConfig::default(), relocated_memory);
    let state_transitions = StateTransitions::from_slice_parallel(&relocated_trace, &memory);

    let builtins_segments = relocator.get_builtin_segments();

    // TODO(spapini): Add output builtin to public memory.
    let (memory, inst_cache) = memory.build();
    Ok(ProverInput {
        state_transitions,
        memory,
        inst_cache,
        public_memory_addresses: relocator
            .relocate_public_addresses(prover_input_info.public_memory_offsets.clone()),
        builtins_segments,
    })
}

pub fn read_and_adapt_prover_input_info_file(
    prover_input_info_path: &Path,
) -> Result<ProverInput, VmImportError> {
    let _span: span::EnteredSpan = span!(Level::INFO, "adapter").entered();

    adapter(&mut read_prover_input_info_file(prover_input_info_path))
}

#[cfg(test)]
#[cfg(feature = "slow-tests")]
mod tests {
    use serde_json::to_value;

    use crate::adapter::read_and_adapt_prover_input_info_file;
    use crate::test_utils::{
        get_prover_input_info_path, get_prover_input_path, get_test_program, read_json,
        run_program_and_adapter, write_json,
    };

    fn test_compare_prover_input_to_expected_file(test_name: &str) {
        let is_fix_mode = std::env::var("FIX") == Ok("1".to_string());

        let compiled_program = get_test_program(test_name);
        let mut prover_input = run_program_and_adapter(&compiled_program);
        // Instruction cache is not deterministic, sort it.
        prover_input.inst_cache.sort_by_key(|(addr, _)| *addr);

        let prover_input_a =
            to_value(prover_input).expect("Unable to covert prover input to value");

        if is_fix_mode {
            write_json(&get_prover_input_path(test_name), &prover_input_a);
        }

        let mut prover_input_b =
            read_and_adapt_prover_input_info_file(&get_prover_input_info_path(test_name))
                .expect("Failed to create prover input from vm output");
        prover_input_b.inst_cache.sort_by_key(|(addr, _)| *addr);

        let expected_prover_input_path = get_prover_input_path(test_name);
        let expected_prover_input = read_json(&expected_prover_input_path);

        assert_eq!(
            prover_input_a,
            expected_prover_input,
            "Prover input from compiled cairo program: {} doesn't match the expected prover input. To update prover input file, run the test with FIX=1.",
            test_name
        );

        assert_eq!(
            prover_input_a,
            to_value(prover_input_b).expect("Unable to covert prover input to value"),
            "Prover input from vm output: {} doesn't match the expected prover inpu",
            test_name,
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
