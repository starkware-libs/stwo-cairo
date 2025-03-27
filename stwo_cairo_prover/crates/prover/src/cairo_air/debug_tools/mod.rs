use itertools::Itertools;
use stwo_prover::constraint_framework::{FrameworkComponent, FrameworkEval};

#[cfg(test)]
pub mod assert_constraints;

#[cfg(test)]
pub mod mock_tree_builder;

#[cfg(feature = "relation-tracker")]
pub mod relation_tracker;

pub(crate) fn indented_component_display<E: FrameworkEval>(
    component: &FrameworkComponent<E>,
) -> String {
    let component_display = &format!("\n{}", component);
    component_display
        .lines()
        .map(|line| format!("\t{}", line))
        .join("\n")
}

pub(crate) fn display_components<E: FrameworkEval>(components: &[FrameworkComponent<E>]) -> String {
    components
        .iter()
        .map(|component| indented_component_display(component))
        .join("\n")
}
