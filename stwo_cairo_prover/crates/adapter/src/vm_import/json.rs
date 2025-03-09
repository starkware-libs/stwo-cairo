use std::collections::HashMap;

use cairo_vm::types::builtin_name::BuiltinName;
use cairo_vm::types::relocatable::MaybeRelocatable;
use cairo_vm::vm::trace::trace_entry::TraceEntry;
use serde::{Deserialize, Serialize};

// TODO(Stav): Replace with original struct once fields are public.
/// Struct to store Cairo private input.
/// Replicated from `cairo_vm::air_private_input::AirPrivateInputSerializable`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrivateInput {
    pub trace_path: String,
    pub memory_path: String,
}

//* ----------------------
//*   ProverInfo
//* ----------------------
/// This struct contains all relevant data for the prover.
/// All addresses are relocatable.
/// TODO(Stav): Remove after merged to cairo-vm.
#[derive(Deserialize, Serialize)]
pub struct ProverInfo {
    // A vector of trace entries, i.e. pc, ap, fp, where pc is relocatable and ap & fp are
    // represented as their offsets, as their indexes will always be 1.
    pub relocatable_trace: Vec<TraceEntry>,
    // A vector of segments, where each segment is a vector of maybe relocatble values.
    pub relocatable_memory: Vec<Vec<MaybeRelocatable>>,
    // A map from segment index to a vector of offsets within the segment, representing the public
    // memory addresses.
    pub public_memory_offsets: HashMap<usize, Vec<usize>>,
    // A map from the builtin name into its segment index.
    pub builtins_segments: HashMap<BuiltinName, isize>,
}
