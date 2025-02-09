use serde::{Deserialize, Serialize};

// TODO(Stav): Replace with original struct once fields are public.
/// Struct to store Cairo private input.
/// Replicated from `cairo_vm::air_private_input::AirPrivateInputSerializable`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrivateInput {
    pub trace_path: String,
    pub memory_path: String,
}
