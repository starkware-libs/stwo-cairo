use std::fs::File;
use std::path::PathBuf;

use cairo_air::utils::{serialize_proof_to_file, ProofFormat};
use cairo_air::verifier::{verify_cairo, CairoVerificationError};
use cairo_air::PreProcessedTraceVariant;
use cairo_lang_executable::executable::{EntryPointKind, Executable};
use cairo_lang_runner::{build_hints_dict, Arg, CairoHintProcessor};
use cairo_lang_utils::bigint::BigUintAsHex;
use cairo_vm::cairo_run::{cairo_run_program_with_initial_scope, CairoRunConfig};
use cairo_vm::hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor;
use cairo_vm::types::exec_scope::ExecutionScopes;
use cairo_vm::types::layout_name::LayoutName;
use cairo_vm::types::program::Program;
use cairo_vm::types::relocatable::MaybeRelocatable;
use cairo_vm::vm::runners::cairo_runner::CairoRunner;
use cairo_vm::Felt252;
use serde::Serialize;
use serde_json::from_reader;
use stwo::core::channel::MerkleChannel;
use stwo::core::pcs::PcsConfig;
use stwo::core::vcs::blake2_merkle::Blake2sMerkleChannel;
use stwo::core::vcs::poseidon252_merkle::Poseidon252MerkleChannel;
use stwo::core::vcs::MerkleHasher;
use stwo::prover::backend::simd::SimdBackend;
use stwo::prover::backend::BackendForChannel;
use stwo::prover::ProvingError;
use stwo_cairo_adapter::adapter::adapter;
use stwo_cairo_adapter::vm_import::VmImportError;
use stwo_cairo_adapter::ProverInput;
use stwo_cairo_prover::prover::{
    default_prod_prover_parameters, prove_cairo, ChannelHash, ProverParameters,
};
use stwo_cairo_serialize::CairoSerialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid arguments: {0}")]
    Cli(#[from] clap::Error),
    #[error("IO failed: {0}")]
    IO(#[from] std::io::Error),
    #[error("Proving failed: {0}")]
    Proving(#[from] ProvingError),
    #[error("Serialization failed: {0}")]
    Serializing(#[from] sonic_rs::error::Error),
    #[error("Verification failed: {0}")]
    Verification(#[from] CairoVerificationError),
    #[error("VM import failed: {0}")]
    VmImport(#[from] VmImportError),
}

/// Generates proof given the Cairo VM output and prover config/parameters.
/// Serializes the proof as JSON and write to the output path.
/// Verifies the proof in case the respective flag is set.
pub fn create_and_serialize_generic_proof<MC: MerkleChannel>(
    input: ProverInput,
    pcs_config: PcsConfig,
    preprocessed_trace: PreProcessedTraceVariant,
    verify: bool,
    proof_path: PathBuf,
    proof_format: ProofFormat,
) -> Result<(), Error>
where
    SimdBackend: BackendForChannel<MC>,
    MC::H: Serialize,
    <MC::H as MerkleHasher>::Hash: CairoSerialize,
{
    let proof = prove_cairo::<MC>(input, pcs_config, preprocessed_trace)?;

    serialize_proof_to_file::<MC::H>(&proof, &proof_path, proof_format)?;

    if verify {
        verify_cairo::<MC>(proof, preprocessed_trace)?;
        log::info!("Proof verified successfully");
    }

    Ok(())
}

pub fn create_and_serialize_proof(
    input: ProverInput,
    verify: bool,
    proof_path: PathBuf,
    proof_format: ProofFormat,
    proof_params_json: Option<PathBuf>,
) -> Result<(), Error> {
    let ProverParameters {
        channel_hash,
        pcs_config,
        preprocessed_trace,
    } = match proof_params_json {
        Some(path) => sonic_rs::from_str(&std::fs::read_to_string(&path)?)?,
        None => default_prod_prover_parameters(),
    };

    let create_and_serialize_generic_proof: fn(
        ProverInput,
        PcsConfig,
        PreProcessedTraceVariant,
        bool,
        PathBuf,
        ProofFormat,
    ) -> Result<(), Error> = match channel_hash {
        ChannelHash::Blake2s => create_and_serialize_generic_proof::<Blake2sMerkleChannel>,
        ChannelHash::Poseidon252 => create_and_serialize_generic_proof::<Poseidon252MerkleChannel>,
    };

    create_and_serialize_generic_proof(
        input,
        pcs_config,
        preprocessed_trace,
        verify,
        proof_path,
        proof_format,
    )?;

    Ok(())
}

pub fn run_program(
    program_path: &PathBuf,
    executable: bool,
    args: Option<&PathBuf>,
) -> CairoRunner {
    let cairo_run_config = CairoRunConfig {
        trace_enabled: true,
        relocate_trace: false,
        layout: LayoutName::all_cairo_stwo,
        proof_mode: true,
        disable_trace_padding: true,
        ..Default::default()
    };

    if executable {
        let executable: Executable =
            from_reader(File::open(program_path).expect("Unable to open executable"))
                .expect("Failed to read executable");
        let args = args.map(read_cairo_arguments_from_file).unwrap_or_default();
        let (program, mut hints) = get_program_and_hints_from_executable(&executable, args);
        cairo_run_program_with_initial_scope(
            &program,
            &cairo_run_config,
            &mut hints,
            ExecutionScopes::new(),
        )
        .expect("Failed to run program")
    } else {
        let program = read_compiled_cairo_program(program_path);

        let mut exec_scopes = ExecutionScopes::new();

        if let Some(args) = args {
            let program_input_contents =
                std::fs::read_to_string(args).expect("Failed to read program input");
            // Insert the program input into the execution scopes if exists
            exec_scopes.insert_value("program_input", program_input_contents);
        }
        // Insert the program object into the execution scopes
        exec_scopes.insert_value("program_object", program.clone());
        cairo_run_program_with_initial_scope(
            &program,
            &cairo_run_config,
            &mut BuiltinHintProcessor::new_empty(),
            exec_scopes,
        )
        .expect("Failed to run program")
    }
}

pub fn get_program_and_hints_from_executable<'a>(
    executable: &'a Executable,
    args: Vec<cairo_lang_runner::Arg>,
) -> (Program, CairoHintProcessor<'a>) {
    let data: Vec<MaybeRelocatable> = executable
        .program
        .bytecode
        .iter()
        .map(Felt252::from)
        .map(MaybeRelocatable::from)
        .collect();
    let (hints, string_to_hint) = build_hints_dict(&executable.program.hints);
    let entrypoint = executable
        .entrypoints
        .iter()
        .find(|e| matches!(e.kind, EntryPointKind::Standalone))
        .expect("Failed to find entrypoint");
    let program = Program::new_for_proof(
        entrypoint.builtins.clone(),
        data,
        entrypoint.offset,
        entrypoint.offset + 4,
        hints,
        Default::default(),
        Default::default(),
        vec![],
        None,
    )
    .unwrap();

    let hint_processor = CairoHintProcessor {
        runner: None,
        user_args: vec![vec![Arg::Array(args)]],
        string_to_hint,
        starknet_state: Default::default(),
        run_resources: Default::default(),
        syscalls_used_resources: Default::default(),
        no_temporary_segments: false,
        markers: Default::default(),
        panic_traceback: Default::default(),
    };

    (program, hint_processor)
}

pub fn read_cairo_arguments_from_file(path: &PathBuf) -> Vec<Arg> {
    let file = std::fs::File::open(path).unwrap();
    let as_vec: Vec<BigUintAsHex> = serde_json::from_reader(file).unwrap();
    as_vec
        .into_iter()
        .map(|v| Arg::Value(v.value.into()))
        .collect()
}

pub fn run_program_and_adapter(
    program_path: &PathBuf,
    executable: bool,
    args: Option<&PathBuf>,
) -> ProverInput {
    let runner = run_program(program_path, executable, args);
    adapter(&runner)
}

fn read_compiled_cairo_program(program_path: &PathBuf) -> Program {
    let bytes =
        std::fs::read(program_path).unwrap_or_else(|e| panic!("Failed to read program: {e:?}"));
    Program::from_bytes(&bytes, Some("main")).expect("Failed to create program from bytes")
}

pub fn get_compiled_cairo_program_path(test_name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../test_data/")
        .join(test_name)
        .join("compiled.json")
}

pub fn get_proof_file_path(test_name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../test_data/")
        .join(test_name)
        .join("proof.json")
}
