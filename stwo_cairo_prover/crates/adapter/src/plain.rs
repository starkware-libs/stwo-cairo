
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

use std::fs::File;
use std::io::Read;
use cairo_vm::vm::trace::trace_entry::TraceEntry;
use cairo_vm::types::relocatable::Relocatable;
use cairo_vm::Felt252; 
use bincode::decode_from_slice;
use std::io;

pub fn read_encoded_prover_input_info(path: &Path) -> io::Result<ProverInputInfo> {
    let mut file = File::open(path)?;
    let mut buf = [0u8; 4];

    // --- Read Header ---
    file.read_exact(&mut buf)?;
    let trace_len = u32::from_le_bytes(buf);
 


    file.read_exact(&mut buf)?;
    let num_memory_segments = u32::from_le_bytes(buf);

    // --- Read Memory ---
    let mut relocatable_memory = Vec::new();

    for j in 0..num_memory_segments {
        // Read segment length
        file.read_exact(&mut buf)?;
        let segment_len = u32::from_le_bytes(buf);

        let mut segment = Vec::with_capacity(segment_len as usize);

        for i in 0..segment_len {
            let mut tag = [0u8; 1];
            file.read_exact(&mut tag)?;

            match tag[0] {
                0u8 => {
                    let mut felt_bytes = [0u8; 32];
                    file.read_exact(&mut felt_bytes)?;
                    segment.push(Some(MaybeRelocatable::Int(Felt252::from_bytes_le(
                        &felt_bytes,
                    ))));
                }
                1u8 => {
                    let mut seg_buf = [0u8; 4];
                    let mut off_buf = [0u8; 4];
                    file.read_exact(&mut seg_buf)?;
                    file.read_exact(&mut off_buf)?;
                    segment.push(Some(MaybeRelocatable::RelocatableValue(Relocatable {
                        segment_index: u32::from_le_bytes(seg_buf) as isize,
                        offset: u32::from_le_bytes(off_buf) as usize,
                    })))
                }
                _ => panic!("invalid tag: {}", tag[0]),
            };
        }
        relocatable_memory.push(segment);
    }
 
    // --- Read Trace ---
    let mut relocatable_trace = Vec::with_capacity(trace_len as usize);
    for _ in 0..trace_len {
        let mut seg_buf = [0u8; 4];
        let mut off_buf = [0u8; 4];
        let mut fp_buf = [0u8; 4];
        let mut ap_buf = [0u8; 4];

        file.read_exact(&mut seg_buf)?;
        file.read_exact(&mut off_buf)?;
        file.read_exact(&mut fp_buf)?;
        file.read_exact(&mut ap_buf)?;

        relocatable_trace.push(TraceEntry {
            pc: Relocatable {
                segment_index: u32::from_le_bytes(seg_buf) as isize,
                offset: u32::from_le_bytes(off_buf) as usize,
            },
            fp: u32::from_le_bytes(fp_buf) as usize,
            ap: u32::from_le_bytes(ap_buf) as usize,
        });
    }

    // --- Read Remaining as Bincode Maps ---
    let mut remaining_bytes = Vec::new();

    file.read_to_end(&mut remaining_bytes)?;
    
 
    let config = bincode::config::standard();

    let (builtins_segments, bytes_read) =
        bincode::serde::decode_from_slice(&remaining_bytes, config.clone()).expect("");

     

    let (public_memory_offsets, _) = decode_from_slice(&remaining_bytes[bytes_read..], config)
        .expect("");

    Ok(ProverInputInfo {
        relocatable_trace,
        relocatable_memory,
        public_memory_offsets,
        builtins_segments,
    })
}

// use std::fs::read;
fn read_prover_input_info_file(prover_input_info_path: &Path) -> ProverInputInfo {
    let _span: span::EnteredSpan = span!(Level::INFO, "read_prover_input_info_file").entered();

    // let bytes = read(prover_input_info_path).unwrap_or_else(|_| {
    //     panic!(
    //         "Unable to read prover input info at path {}",
    //         prover_input_info_path.display()
    //     )
    // });
    // let (prover_input_info, _): (ProverInputInfo, usize) =
    //     bincode::serde::decode_from_slice(&bytes, bincode::config::standard())
    //         .expect("Unable to decode prover input info");

    // prover_input_info
    read_encoded_prover_input_info(prover_input_info_path)
        .unwrap_or_else(|_| panic!("Failed to read prover input info from {:?}", prover_input_info_path))


}

pub fn prover_input_from_vm_output(
    prover_input_info_path: &Path,
) -> Result<ProverInput, VmImportError> {
    let _span: span::EnteredSpan = span!(Level::INFO, "adapter").entered();

    prover_input_info_to_prover_input(&mut read_prover_input_info_file(prover_input_info_path))
}
