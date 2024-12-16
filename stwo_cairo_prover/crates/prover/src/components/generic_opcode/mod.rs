#![allow(clippy::too_many_arguments)]
pub mod component;
pub mod prover;

pub use component::{Claim, Component, Eval, InteractionClaim};
pub use prover::{ClaimGenerator, InputType, InteractionClaimGenerator};

#[cfg(test)]
mod tests {
    use stwo_prover::constraint_framework::expr::ExprEvaluator;
    use stwo_prover::constraint_framework::FrameworkEval;

    use crate::components::generic_opcode;
    use crate::relations;

    #[test]
    fn test_generic_opcode_repr() {
        let eval = generic_opcode::Eval {
            claim: generic_opcode::Claim { n_calls: 0 },
            memoryaddresstoid_lookup_elements: relations::MemoryAddressToId::dummy(),
            memoryidtobig_lookup_elements: relations::MemoryIdToBig::dummy(),
            opcodes_lookup_elements: relations::Opcodes::dummy(),
            rangecheck_19_lookup_elements: relations::RangeCheck_19::dummy(),
            rangecheck_9_9_lookup_elements: relations::RangeCheck_9_9::dummy(),
            verifyinstruction_lookup_elements: relations::VerifyInstruction::dummy(),
        };

        let expected = "";
        let constraint_str = eval
            .evaluate(ExprEvaluator::new(16, true))
            .format_constraints();
        assert_eq!(constraint_str, expected);
    }
}
