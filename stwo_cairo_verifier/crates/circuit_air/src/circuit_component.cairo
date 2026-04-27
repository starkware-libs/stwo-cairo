// Generic AIR component traits live in `stwo_constraint_framework`.
// Re-exported under their local names so the (to-be-generated) component modules under
// `crate::components::*` can keep using `use crate::circuit_component::CircuitComponent`.
pub use stwo_constraint_framework::{AirComponent as CircuitComponent, NewComponent};
