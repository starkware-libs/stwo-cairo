use cairo_vm::air_private_input::{PrivateInputPair, PrivateInputValue};
use serde::{Deserialize, Serialize};

type PedersenData = PrivateInputPair;
type RangeCheckData = PrivateInputValue;

// Can't use cairo_vm::air_private_input::AirPrivateInputSerializable since trace_path and mem_path
// are private.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrivateInput {
    pub trace_path: String,
    pub memory_path: String,
    pub pedersen: Vec<PedersenData>,
    pub range_check: Vec<RangeCheckData>,
}
