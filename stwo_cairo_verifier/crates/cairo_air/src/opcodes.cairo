use core::array::Span;
use core::box::BoxImpl;
use core::iter::{IntoIterator, Iterator};
use stwo_cairo_air::claims::CairoClaim;
use stwo_cairo_air::components;
use stwo_constraint_framework::{
    AirComponent, CommonLookupElements, LookupElementsImpl, PreprocessedMaskValues,
    PreprocessedMaskValuesImpl,
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
        let add = components::add_opcode::NewComponentImpl::try_new(
            cairo_claim.add_opcode, ref claimed_sums, common_lookup_elements,
        );
        let add_small = components::add_opcode_small::NewComponentImpl::try_new(
            cairo_claim.add_opcode_small, ref claimed_sums, common_lookup_elements,
        );
        let add_ap = components::add_ap_opcode::NewComponentImpl::try_new(
            cairo_claim.add_ap_opcode, ref claimed_sums, common_lookup_elements,
        );
        let assert_eq = components::assert_eq_opcode::NewComponentImpl::try_new(
            cairo_claim.assert_eq_opcode, ref claimed_sums, common_lookup_elements,
        );
        let assert_eq_imm = components::assert_eq_opcode_imm::NewComponentImpl::try_new(
            cairo_claim.assert_eq_opcode_imm, ref claimed_sums, common_lookup_elements,
        );
        let assert_eq_double_deref =
            components::assert_eq_opcode_double_deref::NewComponentImpl::try_new(
            cairo_claim.assert_eq_opcode_double_deref, ref claimed_sums, common_lookup_elements,
        );
        let blake = components::blake_compress_opcode::NewComponentImpl::try_new(
            cairo_claim.blake_compress_opcode, ref claimed_sums, common_lookup_elements,
        );
        let call = components::call_opcode_abs::NewComponentImpl::try_new(
            cairo_claim.call_opcode_abs, ref claimed_sums, common_lookup_elements,
        );
        let call_rel_imm = components::call_opcode_rel_imm::NewComponentImpl::try_new(
            cairo_claim.call_opcode_rel_imm, ref claimed_sums, common_lookup_elements,
        );
        let generic = components::generic_opcode::NewComponentImpl::try_new(
            cairo_claim.generic_opcode, ref claimed_sums, common_lookup_elements,
        );
        let jnz = components::jnz_opcode_non_taken::NewComponentImpl::try_new(
            cairo_claim.jnz_opcode_non_taken, ref claimed_sums, common_lookup_elements,
        );
        let jnz_taken = components::jnz_opcode_taken::NewComponentImpl::try_new(
            cairo_claim.jnz_opcode_taken, ref claimed_sums, common_lookup_elements,
        );
        let jump = components::jump_opcode_abs::NewComponentImpl::try_new(
            cairo_claim.jump_opcode_abs, ref claimed_sums, common_lookup_elements,
        );
        let jump_double_deref = components::jump_opcode_double_deref::NewComponentImpl::try_new(
            cairo_claim.jump_opcode_double_deref, ref claimed_sums, common_lookup_elements,
        );
        let jump_rel = components::jump_opcode_rel::NewComponentImpl::try_new(
            cairo_claim.jump_opcode_rel, ref claimed_sums, common_lookup_elements,
        );
        let jump_rel_imm = components::jump_opcode_rel_imm::NewComponentImpl::try_new(
            cairo_claim.jump_opcode_rel_imm, ref claimed_sums, common_lookup_elements,
        );
        let mul = components::mul_opcode::NewComponentImpl::try_new(
            cairo_claim.mul_opcode, ref claimed_sums, common_lookup_elements,
        );
        let mul_small = components::mul_opcode_small::NewComponentImpl::try_new(
            cairo_claim.mul_opcode_small, ref claimed_sums, common_lookup_elements,
        );
        let qm31 = components::qm_31_add_mul_opcode::NewComponentImpl::try_new(
            cairo_claim.qm_31_add_mul_opcode, ref claimed_sums, common_lookup_elements,
        );
        let ret = components::ret_opcode::NewComponentImpl::try_new(
            cairo_claim.ret_opcode, ref claimed_sums, common_lookup_elements,
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

        let add = components::add_opcode::NewComponentImpl::try_new(
            cairo_claim.add_opcode, ref claimed_sums, common_lookup_elements,
        );
        let add_small = components::add_opcode_small::NewComponentImpl::try_new(
            cairo_claim.add_opcode_small, ref claimed_sums, common_lookup_elements,
        );
        let add_ap = components::add_ap_opcode::NewComponentImpl::try_new(
            cairo_claim.add_ap_opcode, ref claimed_sums, common_lookup_elements,
        );
        let assert_eq = components::assert_eq_opcode::NewComponentImpl::try_new(
            cairo_claim.assert_eq_opcode, ref claimed_sums, common_lookup_elements,
        );
        let assert_eq_imm = components::assert_eq_opcode_imm::NewComponentImpl::try_new(
            cairo_claim.assert_eq_opcode_imm, ref claimed_sums, common_lookup_elements,
        );
        let assert_eq_double_deref =
            components::assert_eq_opcode_double_deref::NewComponentImpl::try_new(
            cairo_claim.assert_eq_opcode_double_deref, ref claimed_sums, common_lookup_elements,
        );
        let blake = components::blake_compress_opcode::NewComponentImpl::try_new(
            cairo_claim.blake_compress_opcode, ref claimed_sums, common_lookup_elements,
        );
        let call = components::call_opcode_abs::NewComponentImpl::try_new(
            cairo_claim.call_opcode_abs, ref claimed_sums, common_lookup_elements,
        );
        let call_rel_imm = components::call_opcode_rel_imm::NewComponentImpl::try_new(
            cairo_claim.call_opcode_rel_imm, ref claimed_sums, common_lookup_elements,
        );
        let jnz = components::jnz_opcode_non_taken::NewComponentImpl::try_new(
            cairo_claim.jnz_opcode_non_taken, ref claimed_sums, common_lookup_elements,
        );
        let jnz_taken = components::jnz_opcode_taken::NewComponentImpl::try_new(
            cairo_claim.jnz_opcode_taken, ref claimed_sums, common_lookup_elements,
        );
        let jump = components::jump_opcode_abs::NewComponentImpl::try_new(
            cairo_claim.jump_opcode_abs, ref claimed_sums, common_lookup_elements,
        );
        let jump_double_deref = components::jump_opcode_double_deref::NewComponentImpl::try_new(
            cairo_claim.jump_opcode_double_deref, ref claimed_sums, common_lookup_elements,
        );
        let jump_rel = components::jump_opcode_rel::NewComponentImpl::try_new(
            cairo_claim.jump_opcode_rel, ref claimed_sums, common_lookup_elements,
        );
        let jump_rel_imm = components::jump_opcode_rel_imm::NewComponentImpl::try_new(
            cairo_claim.jump_opcode_rel_imm, ref claimed_sums, common_lookup_elements,
        );
        let mul = components::mul_opcode::NewComponentImpl::try_new(
            cairo_claim.mul_opcode, ref claimed_sums, common_lookup_elements,
        );
        let mul_small = components::mul_opcode_small::NewComponentImpl::try_new(
            cairo_claim.mul_opcode_small, ref claimed_sums, common_lookup_elements,
        );
        let qm31 = components::qm_31_add_mul_opcode::NewComponentImpl::try_new(
            cairo_claim.qm_31_add_mul_opcode, ref claimed_sums, common_lookup_elements,
        );
        let ret = components::ret_opcode::NewComponentImpl::try_new(
            cairo_claim.ret_opcode, ref claimed_sums, common_lookup_elements,
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
