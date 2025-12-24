/**
  Cairo VM to Stwo Prover Adapter.

  This module provides functions to convert Cairo VM execution artifacts into
  the format required by the Stwo prover. It handles relocation of memory
  addresses, padding of builtin segments, and construction of state transitions.

  # Memory Optimization

  The adapter provides two approaches for converting VM data to prover input:

  1. **Direct adaptation** via [`adapt()`]: Convenient but memory-intensive.
     The CairoRunner remains alive during adaptation, causing peak memory of ~27.5GB.

  2. **Extracted data adaptation** via [`adapt_from_extracted_data()`]: Memory-optimized.
     Data is extracted from the runner, which is then dropped before adaptation begins.
     This reduces peak memory to ~16-18GB by preventing VM memory (~11GB) and adapter
     memory (~9GB) from coexisting.

  # Usage Example

  For memory-constrained environments, use the extraction pattern:

  ```rust
  use stwo_cairo_adapter::adapter::adapt_from_extracted_data;

  // Extract data from runner before adaptation.
  let (relocatable_trace, relocatable_memory, public_memory_offsets, builtin_segments) = {
      let cairo_runner = cairo_run_program(...)?;

      let relocatable_trace = cairo_runner.get_relocatable_trace()?.to_vec();
      let relocatable_memory = cairo_runner.get_relocatable_memory();
      let public_memory_offsets = cairo_runner.vm.segments.public_memory_offsets.clone();
      let builtin_segments = cairo_runner.get_builtin_segments();

      // Runner is dropped here, freeing ~11GB.
      (relocatable_trace, relocatable_memory, public_memory_offsets, builtin_segments)
  };

  // Perform adaptation with extracted data.
  let prover_input = adapt_from_extracted_data(
      relocatable_trace,
      relocatable_memory,
      public_memory_offsets,
      builtin_segments,
  )?;
  ```

  # Performance Considerations

  The main cost of the extraction pattern is cloning `public_memory_offsets`,
  which is typically small (< 100MB) compared to the ~11GB memory savings from
  dropping the runner early.
*/

use std::collections::BTreeMap;

use anyhow::Result;
use cairo_vm::stdlib::collections::HashMap;
use cairo_vm::types::builtin_name::BuiltinName;
use cairo_vm::types::relocatable::MaybeRelocatable;
use cairo_vm::vm::runners::cairo_runner::CairoRunner;
use cairo_vm::vm::trace::trace_entry::TraceEntry;
use tracing::{info, span, Level};

use super::memory::{MemoryBuilder, MemoryConfig};
use super::ProverInput;
use crate::builtins::BuiltinSegments;
use crate::relocator::Relocator;
use crate::{PublicSegmentContext, StateTransitions};

/// Adapts Cairo VM execution data into Stwo prover input format.
///
/// This function operates on owned data structures extracted from a CairoRunner,
/// allowing the runner to be dropped before adaptation begins. This pattern reduces
/// peak memory usage by approximately 9-11 GB, as the VM memory (~11GB) and adapter
/// memory (~9GB) do not need to coexist.
///
/// # Memory Optimization Strategy
///
/// The adaptation process involves expensive operations (relocation, builtin padding)
/// that allocate significant memory. By accepting owned data instead of borrowing
/// from the runner, callers can drop the runner before these operations begin,
/// reducing peak memory from ~27.5GB to ~16-18GB.
///
/// # Arguments
///
/// * `relocatable_trace` - The execution trace with relocatable addresses.
/// * `relocatable_memory` - The VM memory segments with relocatable addresses.
/// * `public_memory_offsets` - Mapping of segment indices to public memory offsets.
/// * `builtin_segments` - Mapping of segment indices to builtin names.
///
/// # Returns
///
/// The prover input structure ready for STARK proof generation.
pub fn adapt_from_extracted_data(
    relocatable_trace: Vec<TraceEntry>,
    mut relocatable_memory: Vec<Vec<Option<MaybeRelocatable>>>,
    public_memory_offsets: HashMap<usize, Vec<(usize, usize)>>,
    builtin_segments: BTreeMap<usize, BuiltinName>,
) -> Result<ProverInput> {
    let _span = span!(Level::INFO, "adapt_from_extracted_data").entered();

    info!("Num steps: {:?}", relocatable_trace.len());

    // Relocation part.
    BuiltinSegments::pad_relocatble_builtin_segments(&mut relocatable_memory, &builtin_segments);
    let relocator = Relocator::new(&relocatable_memory);
    let relocated_memory = relocator.relocate_memory(&relocatable_memory);

    #[cfg(feature = "extract-mem-trace")]
    let relocated_memory_clone = relocated_memory.clone();

    let relocated_trace = relocator.relocate_trace(&relocatable_trace);
    let builtin_segments = relocator.relocate_builtin_segments(&builtin_segments);
    info!("Builtin segments: {:?}", builtin_segments);
    let public_memory_addresses = relocator.relocate_public_addresses(&public_memory_offsets);

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

/// Adapts Cairo VM runner output to Stwo prover input format.
///
/// This is a convenience wrapper around [`adapt_from_extracted_data`] that extracts
/// data from the runner and performs adaptation. For memory-constrained environments,
/// prefer extracting data manually and calling [`adapt_from_extracted_data`] directly
/// after dropping the runner.
///
/// # Backward Compatibility
///
/// This function maintains the original API for existing callers. It extracts and
/// clones data from the runner, then delegates to [`adapt_from_extracted_data`].
///
/// # Arguments
///
/// * `runner` - Reference to a finished Cairo VM runner.
///
/// # Returns
///
/// The prover input structure ready for STARK proof generation.
pub fn adapt(runner: &CairoRunner) -> Result<ProverInput> {
    let _span = span!(Level::INFO, "adapt").entered();

    // Extract the relevant information from the Runner.
    let relocatable_trace = runner.get_relocatable_trace()?.to_vec();
    let relocatable_memory = runner.get_relocatable_memory();
    let public_memory_offsets = runner.vm.segments.public_memory_offsets.clone();
    let builtin_segments = runner.get_builtin_segments();

    // Delegate to the extracted data function.
    adapt_from_extracted_data(
        relocatable_trace,
        relocatable_memory,
        public_memory_offsets,
        builtin_segments,
    )
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
