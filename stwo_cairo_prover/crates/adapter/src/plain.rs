use std::fs;
use std::path::Path;

use cairo_vm::stdlib::collections::HashMap;
use cairo_vm::types::relocatable::MaybeRelocatable;
use cairo_vm::vm::runners::cairo_runner::{CairoRunner, ProverInputInfo};
use itertools::Itertools;
use tracing::{span, Level};

use super::memory::{MemoryBuilder, MemoryConfig};
use super::vm_import::VmImportError;
use super::ProverInput;
use crate::builtins::BuiltinSegments;
use crate::relocator::Relocator;
use crate::StateTransitions;

pub fn program_from_casm(
    casm: Vec<cairo_lang_casm::instructions::Instruction>,
) -> (cairo_vm::types::program::Program, usize) {
    let felt_code = casm
        .into_iter()
        .flat_map(|instruction| instruction.assemble().encode())
        .map(|felt| MaybeRelocatable::Int(felt.into()))
        .collect_vec();
    let program_len = felt_code.len();
    let program = cairo_vm::types::program::Program::new_for_proof(
        vec![],
        felt_code,
        0,
        1,
        HashMap::default(),
        Default::default(),
        Default::default(),
        Default::default(),
        Default::default(),
    )
    .expect("Program creation failed");
    (program, program_len)
}

/// Translates a CairoRunner that finished its run into a ProverInput by extracting the relevant
/// input to the adapter.
/// When dev mod is enabled, the opcodes generated from the plain casm will be mapped to the generic
/// component only.
pub fn adapt_finished_runner(runner: CairoRunner) -> Result<ProverInput, VmImportError> {
    let _span = tracing::info_span!("adapt_finished_runner").entered();

    let mut prover_input_info = runner
        .get_prover_input_info()
        .expect("Unable to get prover input info");

    prover_input_info_to_prover_input(&mut prover_input_info)
}

pub fn prover_input_info_to_prover_input(
    prover_input_info: &mut ProverInputInfo,
) -> Result<ProverInput, VmImportError> {
    BuiltinSegments::pad_relocatble_builtin_segments(
        &mut prover_input_info.relocatable_memory,
        prover_input_info.builtins_segments.clone(),
    );
    let relocator = Relocator::new(
        prover_input_info.relocatable_memory.clone(),
        prover_input_info.builtins_segments.clone(),
    );
    let mut memory =
        MemoryBuilder::from_iter(MemoryConfig::default(), relocator.get_relocated_memory());

    let state_transitions = StateTransitions::from_iter(
        relocator
            .relocate_trace(&prover_input_info.relocatable_trace)
            .into_iter(),
        &mut memory,
    );

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

fn read_prover_input_info_file(prover_input_info_path: &Path) -> ProverInputInfo {
    let _span: span::EnteredSpan = span!(Level::INFO, "read_prover_input_info_file").entered();

    let bytes = fs::read(prover_input_info_path).unwrap_or_else(|_| {
        panic!(
            "Unable to read prover input info at path {}",
            prover_input_info_path.display()
        )
    });
    let (prover_input_info, _): (ProverInputInfo, usize) =
        bincode::serde::decode_from_slice(&bytes, bincode::config::standard())
            .expect("Unable to decode prover input info");

    prover_input_info
}
pub fn prover_input_from_vm_output(
    prover_input_info_path: &Path,
) -> Result<ProverInput, VmImportError> {
    let _span: span::EnteredSpan = span!(Level::INFO, "adapter").entered();

    prover_input_info_to_prover_input(&mut read_prover_input_info_file(prover_input_info_path))
}

#[cfg(test)]
#[cfg(feature = "slow-tests")]
mod adapter_tests {
    use serde_json::to_value;

    use crate::plain::prover_input_from_vm_output;
    use crate::test_utils::{
        get_prover_input_info_path, get_prover_input_path,
        prover_input_from_compiled_cairo_program, read_json, write_json,
    };

    fn test_compare_prover_input_to_expected_file(test_name: &str) {
        let is_fix_mode = std::env::var("FIX") == Ok("1".to_string());
        let prover_input_a = to_value(prover_input_from_compiled_cairo_program(test_name))
            .expect("Unable to covert prover input to value");

        if is_fix_mode {
            write_json(&get_prover_input_path(test_name), &prover_input_a);
        }

        let prover_input_b = prover_input_from_vm_output(&get_prover_input_info_path(test_name))
            .expect("Failed to create prover input from vm output");

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
