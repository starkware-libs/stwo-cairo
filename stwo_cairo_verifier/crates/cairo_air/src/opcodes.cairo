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
use stwo_cairo_air::claims::CairoClaim;
use stwo_cairo_air::components;
use stwo_constraint_framework::{
    AirComponent, CommonLookupElements, LookupElementsImpl, PreprocessedMaskValues,
    PreprocessedMaskValuesImpl, pop_claimed_sum,
};
use stwo_verifier_core::ColumnSpan;
use stwo_verifier_core::fields::qm31::QM31;
use stwo_verifier_core::pcs::verifier::CommitmentSchemeVerifierImpl;
use stwo_verifier_core::utils::{ArrayImpl, OptionImpl};
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
        cairo_claim: @CairoClaim,
        common_lookup_elements: @CommonLookupElements,
        ref claimed_sums: Span<QM31>,
    ) -> OpcodeComponents {
        let interaction_claim_add = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => Some(components::add_opcode::InteractionClaim { claimed_sum }),
            None => None,
        };
        let interaction_claim_add_small = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => {
                Some(components::add_opcode_small::InteractionClaim { claimed_sum })
            },
            None => None,
        };
        let interaction_claim_add_ap = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => Some(components::add_ap_opcode::InteractionClaim { claimed_sum }),
            None => None,
        };
        let interaction_claim_assert_eq = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => {
                Some(components::assert_eq_opcode::InteractionClaim { claimed_sum })
            },
            None => None,
        };
        let interaction_claim_assert_eq_imm = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => {
                Some(components::assert_eq_opcode_imm::InteractionClaim { claimed_sum })
            },
            None => None,
        };
        let interaction_claim_assert_eq_double_deref = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => {
                Some(
                    components::assert_eq_opcode_double_deref::InteractionClaim { claimed_sum },
                )
            },
            None => None,
        };
        let interaction_claim_blake = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => {
                Some(components::blake_compress_opcode::InteractionClaim { claimed_sum })
            },
            None => None,
        };
        let interaction_claim_call = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => {
                Some(components::call_opcode_abs::InteractionClaim { claimed_sum })
            },
            None => None,
        };
        let interaction_claim_call_rel_imm = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => {
                Some(components::call_opcode_rel_imm::InteractionClaim { claimed_sum })
            },
            None => None,
        };
        let interaction_claim_generic = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => {
                Some(components::generic_opcode::InteractionClaim { claimed_sum })
            },
            None => None,
        };
        let interaction_claim_jnz = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => {
                Some(components::jnz_opcode_non_taken::InteractionClaim { claimed_sum })
            },
            None => None,
        };
        let interaction_claim_jnz_taken = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => {
                Some(components::jnz_opcode_taken::InteractionClaim { claimed_sum })
            },
            None => None,
        };
        let interaction_claim_jump = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => {
                Some(components::jump_opcode_abs::InteractionClaim { claimed_sum })
            },
            None => None,
        };
        let interaction_claim_jump_double_deref = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => {
                Some(
                    components::jump_opcode_double_deref::InteractionClaim { claimed_sum },
                )
            },
            None => None,
        };
        let interaction_claim_jump_rel = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => {
                Some(components::jump_opcode_rel::InteractionClaim { claimed_sum })
            },
            None => None,
        };
        let interaction_claim_jump_rel_imm = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => {
                Some(components::jump_opcode_rel_imm::InteractionClaim { claimed_sum })
            },
            None => None,
        };
        let interaction_claim_mul = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => Some(components::mul_opcode::InteractionClaim { claimed_sum }),
            None => None,
        };
        let interaction_claim_mul_small = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => {
                Some(components::mul_opcode_small::InteractionClaim { claimed_sum })
            },
            None => None,
        };
        let interaction_claim_qm31 = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => {
                Some(components::qm_31_add_mul_opcode::InteractionClaim { claimed_sum })
            },
            None => None,
        };
        let interaction_claim_ret = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => Some(components::ret_opcode::InteractionClaim { claimed_sum }),
            None => None,
        };

        let add = components::add_opcode::NewComponentImpl::try_new(
            cairo_claim.add_opcode, @interaction_claim_add, common_lookup_elements,
        );
        let add_small = components::add_opcode_small::NewComponentImpl::try_new(
            cairo_claim.add_opcode_small,
            @interaction_claim_add_small,
            common_lookup_elements,
        );
        let add_ap = components::add_ap_opcode::NewComponentImpl::try_new(
            cairo_claim.add_ap_opcode, @interaction_claim_add_ap, common_lookup_elements,
        );
        let assert_eq = components::assert_eq_opcode::NewComponentImpl::try_new(
            cairo_claim.assert_eq_opcode,
            @interaction_claim_assert_eq,
            common_lookup_elements,
        );
        let assert_eq_imm = components::assert_eq_opcode_imm::NewComponentImpl::try_new(
            cairo_claim.assert_eq_opcode_imm,
            @interaction_claim_assert_eq_imm,
            common_lookup_elements,
        );
        let assert_eq_double_deref =
            components::assert_eq_opcode_double_deref::NewComponentImpl::try_new(
            cairo_claim.assert_eq_opcode_double_deref,
            @interaction_claim_assert_eq_double_deref,
            common_lookup_elements,
        );
        let blake = components::blake_compress_opcode::NewComponentImpl::try_new(
            cairo_claim.blake_compress_opcode,
            @interaction_claim_blake,
            common_lookup_elements,
        );
        let call = components::call_opcode_abs::NewComponentImpl::try_new(
            cairo_claim.call_opcode_abs, @interaction_claim_call, common_lookup_elements,
        );
        let call_rel_imm = components::call_opcode_rel_imm::NewComponentImpl::try_new(
            cairo_claim.call_opcode_rel_imm,
            @interaction_claim_call_rel_imm,
            common_lookup_elements,
        );
        let generic = components::generic_opcode::NewComponentImpl::try_new(
            cairo_claim.generic_opcode, @interaction_claim_generic, common_lookup_elements,
        );
        let jnz = components::jnz_opcode_non_taken::NewComponentImpl::try_new(
            cairo_claim.jnz_opcode_non_taken,
            @interaction_claim_jnz,
            common_lookup_elements,
        );
        let jnz_taken = components::jnz_opcode_taken::NewComponentImpl::try_new(
            cairo_claim.jnz_opcode_taken,
            @interaction_claim_jnz_taken,
            common_lookup_elements,
        );
        let jump = components::jump_opcode_abs::NewComponentImpl::try_new(
            cairo_claim.jump_opcode_abs, @interaction_claim_jump, common_lookup_elements,
        );
        let jump_double_deref = components::jump_opcode_double_deref::NewComponentImpl::try_new(
            cairo_claim.jump_opcode_double_deref,
            @interaction_claim_jump_double_deref,
            common_lookup_elements,
        );
        let jump_rel = components::jump_opcode_rel::NewComponentImpl::try_new(
            cairo_claim.jump_opcode_rel, @interaction_claim_jump_rel, common_lookup_elements,
        );
        let jump_rel_imm = components::jump_opcode_rel_imm::NewComponentImpl::try_new(
            cairo_claim.jump_opcode_rel_imm,
            @interaction_claim_jump_rel_imm,
            common_lookup_elements,
        );
        let mul = components::mul_opcode::NewComponentImpl::try_new(
            cairo_claim.mul_opcode, @interaction_claim_mul, common_lookup_elements,
        );
        let mul_small = components::mul_opcode_small::NewComponentImpl::try_new(
            cairo_claim.mul_opcode_small,
            @interaction_claim_mul_small,
            common_lookup_elements,
        );
        let qm31 = components::qm_31_add_mul_opcode::NewComponentImpl::try_new(
            cairo_claim.qm_31_add_mul_opcode,
            @interaction_claim_qm31,
            common_lookup_elements,
        );
        let ret = components::ret_opcode::NewComponentImpl::try_new(
            cairo_claim.ret_opcode, @interaction_claim_ret, common_lookup_elements,
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
                    [].span(),
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
                    [].span(),
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
                    [].span(),
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
                    [].span(),
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
                    [].span(),
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
                    [].span(),
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
                    [].span(),
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
                    [].span(),
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
                    [].span(),
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
                    [].span(),
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
                    [].span(),
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
                    [].span(),
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
                    [].span(),
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
                    [].span(),
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
                    [].span(),
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
                    [].span(),
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
                    [].span(),
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
                    [].span(),
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
                    [].span(),
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
                    [].span(),
                );
        };
    }
}

#[cfg(feature: "poseidon252_verifier")]
#[generate_trait]
pub impl OpcodeComponentsImpl of OpcodeComponentsTrait {
    fn new(
        cairo_claim: @CairoClaim,
        common_lookup_elements: @CommonLookupElements,
        ref claimed_sums: Span<QM31>,
    ) -> OpcodeComponents {
        assert!(cairo_claim.generic_opcode.is_none(), "The generic opcode is not supported.");

        let interaction_claim_add = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => Some(components::add_opcode::InteractionClaim { claimed_sum }),
            None => None,
        };
        let interaction_claim_add_small = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => {
                Some(components::add_opcode_small::InteractionClaim { claimed_sum })
            },
            None => None,
        };
        let interaction_claim_add_ap = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => Some(components::add_ap_opcode::InteractionClaim { claimed_sum }),
            None => None,
        };
        let interaction_claim_assert_eq = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => {
                Some(components::assert_eq_opcode::InteractionClaim { claimed_sum })
            },
            None => None,
        };
        let interaction_claim_assert_eq_imm = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => {
                Some(components::assert_eq_opcode_imm::InteractionClaim { claimed_sum })
            },
            None => None,
        };
        let interaction_claim_assert_eq_double_deref = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => {
                Some(
                    components::assert_eq_opcode_double_deref::InteractionClaim { claimed_sum },
                )
            },
            None => None,
        };
        let interaction_claim_blake = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => {
                Some(components::blake_compress_opcode::InteractionClaim { claimed_sum })
            },
            None => None,
        };
        let interaction_claim_call = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => {
                Some(components::call_opcode_abs::InteractionClaim { claimed_sum })
            },
            None => None,
        };
        let interaction_claim_call_rel_imm = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => {
                Some(components::call_opcode_rel_imm::InteractionClaim { claimed_sum })
            },
            None => None,
        };
        let interaction_claim_jnz = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => {
                Some(components::jnz_opcode_non_taken::InteractionClaim { claimed_sum })
            },
            None => None,
        };
        let interaction_claim_jnz_taken = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => {
                Some(components::jnz_opcode_taken::InteractionClaim { claimed_sum })
            },
            None => None,
        };
        let interaction_claim_jump = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => {
                Some(components::jump_opcode_abs::InteractionClaim { claimed_sum })
            },
            None => None,
        };
        let interaction_claim_jump_double_deref = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => {
                Some(
                    components::jump_opcode_double_deref::InteractionClaim { claimed_sum },
                )
            },
            None => None,
        };
        let interaction_claim_jump_rel = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => {
                Some(components::jump_opcode_rel::InteractionClaim { claimed_sum })
            },
            None => None,
        };
        let interaction_claim_jump_rel_imm = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => {
                Some(components::jump_opcode_rel_imm::InteractionClaim { claimed_sum })
            },
            None => None,
        };
        let interaction_claim_mul = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => Some(components::mul_opcode::InteractionClaim { claimed_sum }),
            None => None,
        };
        let interaction_claim_mul_small = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => {
                Some(components::mul_opcode_small::InteractionClaim { claimed_sum })
            },
            None => None,
        };
        let interaction_claim_qm31 = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => {
                Some(components::qm_31_add_mul_opcode::InteractionClaim { claimed_sum })
            },
            None => None,
        };
        let interaction_claim_ret = match pop_claimed_sum(ref claimed_sums) {
            Some(claimed_sum) => Some(components::ret_opcode::InteractionClaim { claimed_sum }),
            None => None,
        };

        let add = components::add_opcode::NewComponentImpl::try_new(
            cairo_claim.add_opcode, @interaction_claim_add, common_lookup_elements,
        );
        let add_small = components::add_opcode_small::NewComponentImpl::try_new(
            cairo_claim.add_opcode_small,
            @interaction_claim_add_small,
            common_lookup_elements,
        );
        let add_ap = components::add_ap_opcode::NewComponentImpl::try_new(
            cairo_claim.add_ap_opcode, @interaction_claim_add_ap, common_lookup_elements,
        );
        let assert_eq = components::assert_eq_opcode::NewComponentImpl::try_new(
            cairo_claim.assert_eq_opcode,
            @interaction_claim_assert_eq,
            common_lookup_elements,
        );
        let assert_eq_imm = components::assert_eq_opcode_imm::NewComponentImpl::try_new(
            cairo_claim.assert_eq_opcode_imm,
            @interaction_claim_assert_eq_imm,
            common_lookup_elements,
        );
        let assert_eq_double_deref =
            components::assert_eq_opcode_double_deref::NewComponentImpl::try_new(
            cairo_claim.assert_eq_opcode_double_deref,
            @interaction_claim_assert_eq_double_deref,
            common_lookup_elements,
        );
        let blake = components::blake_compress_opcode::NewComponentImpl::try_new(
            cairo_claim.blake_compress_opcode,
            @interaction_claim_blake,
            common_lookup_elements,
        );
        let call = components::call_opcode_abs::NewComponentImpl::try_new(
            cairo_claim.call_opcode_abs, @interaction_claim_call, common_lookup_elements,
        );
        let call_rel_imm = components::call_opcode_rel_imm::NewComponentImpl::try_new(
            cairo_claim.call_opcode_rel_imm,
            @interaction_claim_call_rel_imm,
            common_lookup_elements,
        );
        let jnz = components::jnz_opcode_non_taken::NewComponentImpl::try_new(
            cairo_claim.jnz_opcode_non_taken,
            @interaction_claim_jnz,
            common_lookup_elements,
        );
        let jnz_taken = components::jnz_opcode_taken::NewComponentImpl::try_new(
            cairo_claim.jnz_opcode_taken,
            @interaction_claim_jnz_taken,
            common_lookup_elements,
        );
        let jump = components::jump_opcode_abs::NewComponentImpl::try_new(
            cairo_claim.jump_opcode_abs, @interaction_claim_jump, common_lookup_elements,
        );
        let jump_double_deref = components::jump_opcode_double_deref::NewComponentImpl::try_new(
            cairo_claim.jump_opcode_double_deref,
            @interaction_claim_jump_double_deref,
            common_lookup_elements,
        );
        let jump_rel = components::jump_opcode_rel::NewComponentImpl::try_new(
            cairo_claim.jump_opcode_rel, @interaction_claim_jump_rel, common_lookup_elements,
        );
        let jump_rel_imm = components::jump_opcode_rel_imm::NewComponentImpl::try_new(
            cairo_claim.jump_opcode_rel_imm,
            @interaction_claim_jump_rel_imm,
            common_lookup_elements,
        );
        let mul = components::mul_opcode::NewComponentImpl::try_new(
            cairo_claim.mul_opcode, @interaction_claim_mul, common_lookup_elements,
        );
        let mul_small = components::mul_opcode_small::NewComponentImpl::try_new(
            cairo_claim.mul_opcode_small,
            @interaction_claim_mul_small,
            common_lookup_elements,
        );
        let qm31 = components::qm_31_add_mul_opcode::NewComponentImpl::try_new(
            cairo_claim.qm_31_add_mul_opcode,
            @interaction_claim_qm31,
            common_lookup_elements,
        );
        let ret = components::ret_opcode::NewComponentImpl::try_new(
            cairo_claim.ret_opcode, @interaction_claim_ret, common_lookup_elements,
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
                    [].span(),
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
                    [].span(),
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
                    [].span(),
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
                    [].span(),
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
                    [].span(),
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
                    [].span(),
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
                    [].span(),
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
                    [].span(),
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
                    [].span(),
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
                    [].span(),
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
                    [].span(),
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
                    [].span(),
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
                    [].span(),
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
                    [].span(),
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
                    [].span(),
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
                    [].span(),
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
                    [].span(),
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
                    [].span(),
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
                    [].span(),
                );
        };
    }
}
