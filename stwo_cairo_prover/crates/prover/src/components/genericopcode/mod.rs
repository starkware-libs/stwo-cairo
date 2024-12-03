#![allow(clippy::too_many_arguments)]
pub mod component;
pub mod prover;

pub use component::{Claim, Component, Eval, InteractionClaim, RelationElements};
pub use prover::{ClaimGenerator, InputType, InteractionClaimGenerator};

#[cfg(test)]
mod tests {
    use stwo_prover::constraint_framework::expr::ExprEvaluator;
    use stwo_prover::constraint_framework::FrameworkEval;

    use crate::components::genericopcode;
    use crate::relations;

    #[test]
    fn test_generic_opcode_repr() {
        let eval = genericopcode::Eval {
            claim: genericopcode::Claim { n_calls: 0 },
            memoryaddresstoid_lookup_elements: relations::AddrToId::dummy(),
            memoryidtobig_lookup_elements: relations::IdToValue::dummy(),
            range_check_19_lookup_elements: relations::RangeCheck_19::dummy(),
            range_check_9_9_lookup_elements: relations::RangeCheck_9_9::dummy(),
            verifyinstruction_lookup_elements: relations::VerifyInstruction::dummy(),
            opcodes_lookup_elements: relations::Vm::dummy(),
        };

        let expected = "";
        let constraint_str = eval
            .evaluate(ExprEvaluator::new(16, false))
            .format_constraints();
        assert_eq!(constraint_str, expected);
    }
}
