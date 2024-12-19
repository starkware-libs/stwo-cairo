use serde::{Deserialize, Serialize};
// TODO(Stav): delete this struct after making those fieldes public in
// `vm::air_private_input::AirPrivateInputSerializable.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrivateInput {
    pub trace_path: String,
    pub memory_path: String,
}
