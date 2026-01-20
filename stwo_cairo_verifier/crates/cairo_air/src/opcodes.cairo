use components::add_ap_opcode::InteractionClaimImpl as AddApOpcodeInteractionClaimImpl;
use components::add_opcode::InteractionClaimImpl as AddOpcodeInteractionClaimImpl;
use components::add_opcode_small::InteractionClaimImpl as AddOpcodeSmallInteractionClaimImpl;
use components::assert_eq_opcode::InteractionClaimImpl as AssertEqOpcodeInteractionClaimImpl;
use components::assert_eq_opcode_double_deref::InteractionClaimImpl as AssertEqOpcodeDoubleDerefInteractionClaimImpl;
use components::assert_eq_opcode_imm::InteractionClaimImpl as AssertEqOpcodeImmInteractionClaimImpl;
use components::blake_compress_opcode::InteractionClaimImpl as BlakeCompressOpcodeInteractionClaimImpl;
use components::call_opcode_abs::InteractionClaimImpl as CallOpcodeInteractionClaimImpl;
use components::call_opcode_rel_imm::InteractionClaimImpl as CallOpcodeRelInteractionClaimImpl;
use components::generic_opcode::InteractionClaimImpl as GenericOpcodeInteractionClaimImpl;
use components::jnz_opcode_non_taken::InteractionClaimImpl as JnzOpcodeInteractionClaimImpl;
use components::jnz_opcode_taken::InteractionClaimImpl as JnzOpcodeTakenInteractionClaimImpl;
use components::jump_opcode_abs::InteractionClaimImpl as JumpOpcodeInteractionClaimImpl;
use components::jump_opcode_double_deref::InteractionClaimImpl as JumpOpcodeDoubleDerefInteractionClaimImpl;
use components::jump_opcode_rel::InteractionClaimImpl as JumpOpcodeRelInteractionClaimImpl;
use components::jump_opcode_rel_imm::InteractionClaimImpl as JumpOpcodeRelImmInteractionClaimImpl;
use components::mul_opcode::InteractionClaimImpl as MulOpcodeInteractionClaimImpl;
use components::mul_opcode_small::InteractionClaimImpl as MulOpcodeSmallInteractionClaimImpl;
use components::qm_31_add_mul_opcode::InteractionClaimImpl as Qm31AddMulOpcodeInteractionClaimImpl;
use components::ret_opcode::InteractionClaimImpl as RetOpcodeInteractionClaimImpl;
use core::array::Span;
use core::box::BoxImpl;
use core::iter::{IntoIterator, Iterator};
use stwo_cairo_air::cairo_component::CairoComponent;
use stwo_cairo_air::claim::ClaimTrait;
use stwo_cairo_air::claims::CairoClaim;
use stwo_cairo_air::components;
use stwo_constraint_framework::{
    CommonLookupElements, LookupElementsImpl, PreprocessedMaskValues, PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::fields::qm31::QM31;
use stwo_verifier_core::pcs::verifier::CommitmentSchemeVerifierImpl;
use stwo_verifier_core::utils::{ArrayImpl, OptionImpl};
use stwo_verifier_core::{ColumnSpan, TreeArray};
use crate::utils;

pub fn opcodes_log_sizes(claim: @CairoClaim) -> TreeArray<Span<u32>> {
    let mut log_sizes = array![];

    if let Some(add) = claim.add_opcode {
        log_sizes.append(add.log_sizes());
    }
    if let Some(add_small) = claim.add_opcode_small {
        log_sizes.append(add_small.log_sizes());
    }
    if let Some(add_ap) = claim.add_ap_opcode {
        log_sizes.append(add_ap.log_sizes());
    }
    if let Some(assert_eq) = claim.assert_eq_opcode {
        log_sizes.append(assert_eq.log_sizes());
    }
    if let Some(assert_eq_imm) = claim.assert_eq_opcode_imm {
        log_sizes.append(assert_eq_imm.log_sizes());
    }
    if let Some(assert_eq_double_deref) = claim.assert_eq_opcode_double_deref {
        log_sizes.append(assert_eq_double_deref.log_sizes());
    }
    if let Some(blake) = claim.blake_compress_opcode {
        log_sizes.append(blake.log_sizes());
    }
    if let Some(call) = claim.call_opcode_abs {
        log_sizes.append(call.log_sizes());
    }
    if let Some(call_rel_imm) = claim.call_opcode_rel_imm {
        log_sizes.append(call_rel_imm.log_sizes());
    }
    if let Some(generic) = claim.generic_opcode {
        log_sizes.append(generic.log_sizes());
    }
    if let Some(jnz) = claim.jnz_opcode_non_taken {
        log_sizes.append(jnz.log_sizes());
    }
    if let Some(jnz_taken) = claim.jnz_opcode_taken {
        log_sizes.append(jnz_taken.log_sizes());
    }
    if let Some(jump) = claim.jump_opcode_abs {
        log_sizes.append(jump.log_sizes());
    }
    if let Some(jump_double_deref) = claim.jump_opcode_double_deref {
        log_sizes.append(jump_double_deref.log_sizes());
    }
    if let Some(jump_rel) = claim.jump_opcode_rel {
        log_sizes.append(jump_rel.log_sizes());
    }
    if let Some(jump_rel_imm) = claim.jump_opcode_rel_imm {
        log_sizes.append(jump_rel_imm.log_sizes());
    }
    if let Some(mul) = claim.mul_opcode {
        log_sizes.append(mul.log_sizes());
    }
    if let Some(mul_small) = claim.mul_opcode_small {
        log_sizes.append(mul_small.log_sizes());
    }
    if let Some(qm31) = claim.qm_31_add_mul_opcode {
        log_sizes.append(qm31.log_sizes());
    }
    if let Some(ret) = claim.ret_opcode {
        log_sizes.append(ret.log_sizes());
    }

    utils::tree_array_concat_cols(log_sizes)
}

#[cfg(not(feature: "poseidon252_verifier"))]
#[derive(Drop)]
pub struct OpcodeComponents {
    add: Array<components::add_opcode::Component>,
    add_small: Array<components::add_opcode_small::Component>,
    add_ap: Array<components::add_ap_opcode::Component>,
    assert_eq: Array<components::assert_eq_opcode::Component>,
    assert_eq_imm: Array<components::assert_eq_opcode_imm::Component>,
    assert_eq_double_deref: Array<components::assert_eq_opcode_double_deref::Component>,
    blake: Array<components::blake_compress_opcode::Component>,
    call: Array<components::call_opcode_abs::Component>,
    call_rel_imm: Array<components::call_opcode_rel_imm::Component>,
    generic: Array<components::generic_opcode::Component>,
    jnz: Array<components::jnz_opcode_non_taken::Component>,
    jnz_taken: Array<components::jnz_opcode_taken::Component>,
    jump: Array<components::jump_opcode_abs::Component>,
    jump_double_deref: Array<components::jump_opcode_double_deref::Component>,
    jump_rel: Array<components::jump_opcode_rel::Component>,
    jump_rel_imm: Array<components::jump_opcode_rel_imm::Component>,
    mul: Array<components::mul_opcode::Component>,
    mul_small: Array<components::mul_opcode_small::Component>,
    qm31: Array<components::qm_31_add_mul_opcode::Component>,
    ret: Array<components::ret_opcode::Component>,
}

#[cfg(feature: "poseidon252_verifier")]
#[derive(Drop)]
pub struct OpcodeComponents {
    add: Array<components::add_opcode::Component>,
    add_small: Array<components::add_opcode_small::Component>,
    add_ap: Array<components::add_ap_opcode::Component>,
    assert_eq: Array<components::assert_eq_opcode::Component>,
    assert_eq_imm: Array<components::assert_eq_opcode_imm::Component>,
    assert_eq_double_deref: Array<components::assert_eq_opcode_double_deref::Component>,
    blake: Array<components::blake_compress_opcode::Component>,
    call: Array<components::call_opcode_abs::Component>,
    call_rel_imm: Array<components::call_opcode_rel_imm::Component>,
    jnz: Array<components::jnz_opcode_non_taken::Component>,
    jnz_taken: Array<components::jnz_opcode_taken::Component>,
    jump: Array<components::jump_opcode_abs::Component>,
    jump_double_deref: Array<components::jump_opcode_double_deref::Component>,
    jump_rel: Array<components::jump_opcode_rel::Component>,
    jump_rel_imm: Array<components::jump_opcode_rel_imm::Component>,
    mul: Array<components::mul_opcode::Component>,
    mul_small: Array<components::mul_opcode_small::Component>,
    qm31: Array<components::qm_31_add_mul_opcode::Component>,
    ret: Array<components::ret_opcode::Component>,
}

#[cfg(not(feature: "poseidon252_verifier"))]
#[generate_trait]
pub impl OpcodeComponentsImpl of OpcodeComponentsTrait {
    fn new(
        add_claim: @Option<components::add_opcode::Claim>,
        add_small_claim: @Option<components::add_opcode_small::Claim>,
        add_ap_claim: @Option<components::add_ap_opcode::Claim>,
        assert_eq_claim: @Option<components::assert_eq_opcode::Claim>,
        assert_eq_imm_claim: @Option<components::assert_eq_opcode_imm::Claim>,
        assert_eq_double_deref_claim: @Option<components::assert_eq_opcode_double_deref::Claim>,
        blake_claim: @Option<components::blake_compress_opcode::Claim>,
        call_claim: @Option<components::call_opcode_abs::Claim>,
        call_rel_imm_claim: @Option<components::call_opcode_rel_imm::Claim>,
        generic_claim: @Option<components::generic_opcode::Claim>,
        jnz_claim: @Option<components::jnz_opcode_non_taken::Claim>,
        jnz_taken_claim: @Option<components::jnz_opcode_taken::Claim>,
        jump_claim: @Option<components::jump_opcode_abs::Claim>,
        jump_double_deref_claim: @Option<components::jump_opcode_double_deref::Claim>,
        jump_rel_claim: @Option<components::jump_opcode_rel::Claim>,
        jump_rel_imm_claim: @Option<components::jump_opcode_rel_imm::Claim>,
        mul_claim: @Option<components::mul_opcode::Claim>,
        mul_small_claim: @Option<components::mul_opcode_small::Claim>,
        qm31_claim: @Option<components::qm_31_add_mul_opcode::Claim>,
        ret_claim: @Option<components::ret_opcode::Claim>,
        common_lookup_elements: @CommonLookupElements,
        add_interaction_claim: @Option<components::add_opcode::InteractionClaim>,
        add_small_interaction_claim: @Option<components::add_opcode_small::InteractionClaim>,
        add_ap_interaction_claim: @Option<components::add_ap_opcode::InteractionClaim>,
        assert_eq_interaction_claim: @Option<components::assert_eq_opcode::InteractionClaim>,
        assert_eq_imm_interaction_claim: @Option<
            components::assert_eq_opcode_imm::InteractionClaim,
        >,
        assert_eq_double_deref_interaction_claim: @Option<
            components::assert_eq_opcode_double_deref::InteractionClaim,
        >,
        blake_interaction_claim: @Option<components::blake_compress_opcode::InteractionClaim>,
        call_interaction_claim: @Option<components::call_opcode_abs::InteractionClaim>,
        call_rel_imm_interaction_claim: @Option<components::call_opcode_rel_imm::InteractionClaim>,
        generic_interaction_claim: @Option<components::generic_opcode::InteractionClaim>,
        jnz_interaction_claim: @Option<components::jnz_opcode_non_taken::InteractionClaim>,
        jnz_taken_interaction_claim: @Option<components::jnz_opcode_taken::InteractionClaim>,
        jump_interaction_claim: @Option<components::jump_opcode_abs::InteractionClaim>,
        jump_double_deref_interaction_claim: @Option<
            components::jump_opcode_double_deref::InteractionClaim,
        >,
        jump_rel_interaction_claim: @Option<components::jump_opcode_rel::InteractionClaim>,
        jump_rel_imm_interaction_claim: @Option<components::jump_opcode_rel_imm::InteractionClaim>,
        mul_interaction_claim: @Option<components::mul_opcode::InteractionClaim>,
        mul_small_interaction_claim: @Option<components::mul_opcode_small::InteractionClaim>,
        qm31_interaction_claim: @Option<components::qm_31_add_mul_opcode::InteractionClaim>,
        ret_interaction_claim: @Option<components::ret_opcode::InteractionClaim>,
    ) -> OpcodeComponents {
        // Add components
        let add = components::add_opcode::NewComponentImpl::try_new(
            add_claim, add_interaction_claim, common_lookup_elements,
        );

        // Add Small components
        let add_small = components::add_opcode_small::NewComponentImpl::try_new(
            add_small_claim, add_small_interaction_claim, common_lookup_elements,
        );

        // Add AP components
        let add_ap = components::add_ap_opcode::NewComponentImpl::try_new(
            add_ap_claim, add_ap_interaction_claim, common_lookup_elements,
        );

        // Assert Eq components
        let assert_eq = components::assert_eq_opcode::NewComponentImpl::try_new(
            assert_eq_claim, assert_eq_interaction_claim, common_lookup_elements,
        );

        // Assert Eq Imm components
        let assert_eq_imm = components::assert_eq_opcode_imm::NewComponentImpl::try_new(
            assert_eq_imm_claim, assert_eq_imm_interaction_claim, common_lookup_elements,
        );

        // Assert Eq Double Deref components
        let assert_eq_double_deref =
            components::assert_eq_opcode_double_deref::NewComponentImpl::try_new(
            assert_eq_double_deref_claim,
            assert_eq_double_deref_interaction_claim,
            common_lookup_elements,
        );

        let blake = components::blake_compress_opcode::NewComponentImpl::try_new(
            blake_claim, blake_interaction_claim, common_lookup_elements,
        );

        // Call components
        let call = components::call_opcode_abs::NewComponentImpl::try_new(
            call_claim, call_interaction_claim, common_lookup_elements,
        );

        // Call Rel_imm components
        let call_rel_imm = components::call_opcode_rel_imm::NewComponentImpl::try_new(
            call_rel_imm_claim, call_rel_imm_interaction_claim, common_lookup_elements,
        );

        // Generic components
        let generic = components::generic_opcode::NewComponentImpl::try_new(
            generic_claim, generic_interaction_claim, common_lookup_elements,
        );

        // Jnz components
        let jnz = components::jnz_opcode_non_taken::NewComponentImpl::try_new(
            jnz_claim, jnz_interaction_claim, common_lookup_elements,
        );

        // Jnz Taken components
        let jnz_taken = components::jnz_opcode_taken::NewComponentImpl::try_new(
            jnz_taken_claim, jnz_taken_interaction_claim, common_lookup_elements,
        );

        // Jump components
        let jump = components::jump_opcode_abs::NewComponentImpl::try_new(
            jump_claim, jump_interaction_claim, common_lookup_elements,
        );

        // Jump Double Deref components
        let jump_double_deref = components::jump_opcode_double_deref::NewComponentImpl::try_new(
            jump_double_deref_claim, jump_double_deref_interaction_claim, common_lookup_elements,
        );

        // Jump Rel components
        let jump_rel = components::jump_opcode_rel::NewComponentImpl::try_new(
            jump_rel_claim, jump_rel_interaction_claim, common_lookup_elements,
        );

        // Jump Rel Imm components
        let jump_rel_imm = components::jump_opcode_rel_imm::NewComponentImpl::try_new(
            jump_rel_imm_claim, jump_rel_imm_interaction_claim, common_lookup_elements,
        );

        // Mul components
        let mul = components::mul_opcode::NewComponentImpl::try_new(
            mul_claim, mul_interaction_claim, common_lookup_elements,
        );

        // Mul Small components
        let mul_small = components::mul_opcode_small::NewComponentImpl::try_new(
            mul_small_claim, mul_small_interaction_claim, common_lookup_elements,
        );

        // QM31 components
        let qm31 = components::qm_31_add_mul_opcode::NewComponentImpl::try_new(
            qm31_claim, qm31_interaction_claim, common_lookup_elements,
        );

        // Ret components
        let ret = components::ret_opcode::NewComponentImpl::try_new(
            ret_claim, ret_interaction_claim, common_lookup_elements,
        );

        OpcodeComponents {
            add: add.into_iter().collect::<Array<_>>(),
            add_small: add_small.into_iter().collect::<Array<_>>(),
            add_ap: add_ap.into_iter().collect::<Array<_>>(),
            assert_eq: assert_eq.into_iter().collect::<Array<_>>(),
            assert_eq_imm: assert_eq_imm.into_iter().collect::<Array<_>>(),
            assert_eq_double_deref: assert_eq_double_deref.into_iter().collect::<Array<_>>(),
            blake: blake.into_iter().collect::<Array<_>>(),
            call: call.into_iter().collect::<Array<_>>(),
            call_rel_imm: call_rel_imm.into_iter().collect::<Array<_>>(),
            generic: generic.into_iter().collect::<Array<_>>(),
            jnz: jnz.into_iter().collect::<Array<_>>(),
            jnz_taken: jnz_taken.into_iter().collect::<Array<_>>(),
            jump: jump.into_iter().collect::<Array<_>>(),
            jump_double_deref: jump_double_deref.into_iter().collect::<Array<_>>(),
            jump_rel: jump_rel.into_iter().collect::<Array<_>>(),
            jump_rel_imm: jump_rel_imm.into_iter().collect::<Array<_>>(),
            mul: mul.into_iter().collect::<Array<_>>(),
            mul_small: mul_small.into_iter().collect::<Array<_>>(),
            qm31: qm31.into_iter().collect::<Array<_>>(),
            ret: ret.into_iter().collect::<Array<_>>(),
        }
    }

    fn evaluate_constraints_at_point(
        self: @OpcodeComponents,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
    ) {
        for component in self.add.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }
        for component in self.add_small.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }
        for component in self.add_ap.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }
        for component in self.assert_eq.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }
        for component in self.assert_eq_imm.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }
        for component in self.assert_eq_double_deref.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }

        for component in self.blake.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }

        for component in self.call.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }

        for component in self.call_rel_imm.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }

        for component in self.generic.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }

        for component in self.jnz.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }

        for component in self.jnz_taken.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }

        for component in self.jump.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }

        for component in self.jump_double_deref.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }

        for component in self.jump_rel.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }

        for component in self.jump_rel_imm.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }

        for component in self.mul.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }
        for component in self.mul_small.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }

        for component in self.qm31.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }

        for component in self.ret.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        };
    }
}

#[cfg(feature: "poseidon252_verifier")]
#[generate_trait]
pub impl OpcodeComponentsImpl of OpcodeComponentsTrait {
    fn new(
        add_claim: @Option<components::add_opcode::Claim>,
        add_small_claim: @Option<components::add_opcode_small::Claim>,
        add_ap_claim: @Option<components::add_ap_opcode::Claim>,
        assert_eq_claim: @Option<components::assert_eq_opcode::Claim>,
        assert_eq_imm_claim: @Option<components::assert_eq_opcode_imm::Claim>,
        assert_eq_double_deref_claim: @Option<components::assert_eq_opcode_double_deref::Claim>,
        blake_claim: @Option<components::blake_compress_opcode::Claim>,
        call_claim: @Option<components::call_opcode_abs::Claim>,
        call_rel_imm_claim: @Option<components::call_opcode_rel_imm::Claim>,
        generic_claim: @Option<components::generic_opcode::Claim>,
        jnz_claim: @Option<components::jnz_opcode_non_taken::Claim>,
        jnz_taken_claim: @Option<components::jnz_opcode_taken::Claim>,
        jump_claim: @Option<components::jump_opcode_abs::Claim>,
        jump_double_deref_claim: @Option<components::jump_opcode_double_deref::Claim>,
        jump_rel_claim: @Option<components::jump_opcode_rel::Claim>,
        jump_rel_imm_claim: @Option<components::jump_opcode_rel_imm::Claim>,
        mul_claim: @Option<components::mul_opcode::Claim>,
        mul_small_claim: @Option<components::mul_opcode_small::Claim>,
        qm31_claim: @Option<components::qm_31_add_mul_opcode::Claim>,
        ret_claim: @Option<components::ret_opcode::Claim>,
        common_lookup_elements: @CommonLookupElements,
        add_interaction_claim: @Option<components::add_opcode::InteractionClaim>,
        add_small_interaction_claim: @Option<components::add_opcode_small::InteractionClaim>,
        add_ap_interaction_claim: @Option<components::add_ap_opcode::InteractionClaim>,
        assert_eq_interaction_claim: @Option<components::assert_eq_opcode::InteractionClaim>,
        assert_eq_imm_interaction_claim: @Option<
            components::assert_eq_opcode_imm::InteractionClaim,
        >,
        assert_eq_double_deref_interaction_claim: @Option<
            components::assert_eq_opcode_double_deref::InteractionClaim,
        >,
        blake_interaction_claim: @Option<components::blake_compress_opcode::InteractionClaim>,
        call_interaction_claim: @Option<components::call_opcode_abs::InteractionClaim>,
        call_rel_imm_interaction_claim: @Option<components::call_opcode_rel_imm::InteractionClaim>,
        generic_interaction_claim: @Option<components::generic_opcode::InteractionClaim>,
        jnz_interaction_claim: @Option<components::jnz_opcode_non_taken::InteractionClaim>,
        jnz_taken_interaction_claim: @Option<components::jnz_opcode_taken::InteractionClaim>,
        jump_interaction_claim: @Option<components::jump_opcode_abs::InteractionClaim>,
        jump_double_deref_interaction_claim: @Option<
            components::jump_opcode_double_deref::InteractionClaim,
        >,
        jump_rel_interaction_claim: @Option<components::jump_opcode_rel::InteractionClaim>,
        jump_rel_imm_interaction_claim: @Option<components::jump_opcode_rel_imm::InteractionClaim>,
        mul_interaction_claim: @Option<components::mul_opcode::InteractionClaim>,
        mul_small_interaction_claim: @Option<components::mul_opcode_small::InteractionClaim>,
        qm31_interaction_claim: @Option<components::qm_31_add_mul_opcode::InteractionClaim>,
        ret_interaction_claim: @Option<components::ret_opcode::InteractionClaim>,
    ) -> OpcodeComponents {
        assert!(generic_claim.is_none(), "The generic opcode is not supported.");
        assert!(generic_interaction_claim.is_none(), "The generic opcode is not supported.");

        // Add components
        let add = components::add_opcode::NewComponentImpl::try_new(
            add_claim, add_interaction_claim, common_lookup_elements,
        );

        // Add Small components
        let add_small = components::add_opcode_small::NewComponentImpl::try_new(
            add_small_claim, add_small_interaction_claim, common_lookup_elements,
        );

        // Add AP components
        let add_ap = components::add_ap_opcode::NewComponentImpl::try_new(
            add_ap_claim, add_ap_interaction_claim, common_lookup_elements,
        );

        // Assert Eq components
        let assert_eq = components::assert_eq_opcode::NewComponentImpl::try_new(
            assert_eq_claim, assert_eq_interaction_claim, common_lookup_elements,
        );

        // Assert Eq Imm components
        let assert_eq_imm = components::assert_eq_opcode_imm::NewComponentImpl::try_new(
            assert_eq_imm_claim, assert_eq_imm_interaction_claim, common_lookup_elements,
        );

        // Assert Eq Double Deref components
        let assert_eq_double_deref =
            components::assert_eq_opcode_double_deref::NewComponentImpl::try_new(
            assert_eq_double_deref_claim,
            assert_eq_double_deref_interaction_claim,
            common_lookup_elements,
        );

        let blake = components::blake_compress_opcode::NewComponentImpl::try_new(
            blake_claim, blake_interaction_claim, common_lookup_elements,
        );

        // Call components
        let call = components::call_opcode_abs::NewComponentImpl::try_new(
            call_claim, call_interaction_claim, common_lookup_elements,
        );

        // Call Rel_imm components
        let call_rel_imm = components::call_opcode_rel_imm::NewComponentImpl::try_new(
            call_rel_imm_claim, call_rel_imm_interaction_claim, common_lookup_elements,
        );

        // Jnz components
        let jnz = components::jnz_opcode_non_taken::NewComponentImpl::try_new(
            jnz_claim, jnz_interaction_claim, common_lookup_elements,
        );

        // Jnz Taken components
        let jnz_taken = components::jnz_opcode_taken::NewComponentImpl::try_new(
            jnz_taken_claim, jnz_taken_interaction_claim, common_lookup_elements,
        );

        // Jump components
        let jump = components::jump_opcode_abs::NewComponentImpl::try_new(
            jump_claim, jump_interaction_claim, common_lookup_elements,
        );

        // Jump Double Deref components
        let jump_double_deref = components::jump_opcode_double_deref::NewComponentImpl::try_new(
            jump_double_deref_claim, jump_double_deref_interaction_claim, common_lookup_elements,
        );

        // Jump Rel components
        let jump_rel = components::jump_opcode_rel::NewComponentImpl::try_new(
            jump_rel_claim, jump_rel_interaction_claim, common_lookup_elements,
        );

        // Jump Rel Imm components
        let jump_rel_imm = components::jump_opcode_rel_imm::NewComponentImpl::try_new(
            jump_rel_imm_claim, jump_rel_imm_interaction_claim, common_lookup_elements,
        );

        // Mul components
        let mul = components::mul_opcode::NewComponentImpl::try_new(
            mul_claim, mul_interaction_claim, common_lookup_elements,
        );

        // Mul Small components
        let mul_small = components::mul_opcode_small::NewComponentImpl::try_new(
            mul_small_claim, mul_small_interaction_claim, common_lookup_elements,
        );

        // QM31 components
        let qm31 = components::qm_31_add_mul_opcode::NewComponentImpl::try_new(
            qm31_claim, qm31_interaction_claim, common_lookup_elements,
        );

        // Ret components
        let ret = components::ret_opcode::NewComponentImpl::try_new(
            ret_claim, ret_interaction_claim, common_lookup_elements,
        );

        OpcodeComponents {
            add: add.into_iter().collect::<Array<_>>(),
            add_small: add_small.into_iter().collect::<Array<_>>(),
            add_ap: add_ap.into_iter().collect::<Array<_>>(),
            assert_eq: assert_eq.into_iter().collect::<Array<_>>(),
            assert_eq_imm: assert_eq_imm.into_iter().collect::<Array<_>>(),
            assert_eq_double_deref: assert_eq_double_deref.into_iter().collect::<Array<_>>(),
            blake: blake.into_iter().collect::<Array<_>>(),
            call: call.into_iter().collect::<Array<_>>(),
            call_rel_imm: call_rel_imm.into_iter().collect::<Array<_>>(),
            jnz: jnz.into_iter().collect::<Array<_>>(),
            jnz_taken: jnz_taken.into_iter().collect::<Array<_>>(),
            jump: jump.into_iter().collect::<Array<_>>(),
            jump_double_deref: jump_double_deref.into_iter().collect::<Array<_>>(),
            jump_rel: jump_rel.into_iter().collect::<Array<_>>(),
            jump_rel_imm: jump_rel_imm.into_iter().collect::<Array<_>>(),
            mul: mul.into_iter().collect::<Array<_>>(),
            mul_small: mul_small.into_iter().collect::<Array<_>>(),
            qm31: qm31.into_iter().collect::<Array<_>>(),
            ret: ret.into_iter().collect::<Array<_>>(),
        }
    }

    fn evaluate_constraints_at_point(
        self: @OpcodeComponents,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
    ) {
        for component in self.add.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }
        for component in self.add_small.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }
        for component in self.add_ap.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }
        for component in self.assert_eq.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }
        for component in self.assert_eq_imm.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }
        for component in self.assert_eq_double_deref.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }

        for component in self.blake.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }

        for component in self.call.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }

        for component in self.call_rel_imm.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }

        for component in self.jnz.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }

        for component in self.jnz_taken.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }

        for component in self.jump.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }

        for component in self.jump_double_deref.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }

        for component in self.jump_rel.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }

        for component in self.jump_rel_imm.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }

        for component in self.mul.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }
        for component in self.mul_small.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }

        for component in self.qm31.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        }

        for component in self.ret.span() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                );
        };
    }
}
