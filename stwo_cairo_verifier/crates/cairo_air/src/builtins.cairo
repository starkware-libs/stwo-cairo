use core::array::Span;
use core::box::BoxImpl;
use stwo_cairo_air::component_indices::*;
use stwo_cairo_air::{PublicData, components};
use stwo_constraint_framework::{
    AirComponent, CommonLookupElements, LookupElementsImpl, PreprocessedMaskValues,
    PreprocessedMaskValuesImpl,
};
use stwo_verifier_core::ColumnSpan;
use stwo_verifier_core::fields::qm31::QM31;
use stwo_verifier_core::pcs::verifier::CommitmentSchemeVerifierImpl;
use stwo_verifier_core::utils::{ArrayImpl, OptionImpl};
#[derive(Drop)]
#[cfg(not(feature: "poseidon252_verifier"))]
pub struct BuiltinComponents {
    pub add_mod_builtin: Option<components::add_mod_builtin::Component>,
    pub bitwise_builtin: Option<components::bitwise_builtin::Component>,
    pub mul_mod_builtin: Option<components::mul_mod_builtin::Component>,
    pub pedersen_builtin: Option<components::pedersen_builtin::Component>,
    pub poseidon_builtin: Option<components::poseidon_builtin::Component>,
    pub range_check_96_builtin: Option<components::range_check96_builtin::Component>,
    pub range_check_128_builtin: Option<components::range_check_builtin::Component>,
    pub ec_op_builtin: Option<components::ec_op_builtin::Component>,
}

#[derive(Drop)]
#[cfg(and(feature: "poseidon252_verifier", not(feature: "poseidon_outputs_packing")))]
pub struct BuiltinComponents {
    pub bitwise_builtin: Option<components::bitwise_builtin::Component>,
    pub range_check_128_builtin: Option<components::range_check_builtin::Component>,
}

#[derive(Drop)]
#[cfg(and(feature: "poseidon252_verifier", feature: "poseidon_outputs_packing"))]
pub struct BuiltinComponents {
    pub bitwise_builtin: Option<components::bitwise_builtin::Component>,
    pub poseidon_builtin: Option<components::poseidon_builtin::Component>,
    pub range_check_128_builtin: Option<components::range_check_builtin::Component>,
}


#[generate_trait]
#[cfg(not(feature: "poseidon252_verifier"))]
pub impl BuiltinComponentsImpl of BuiltinComponentsTrait {
    fn new(
        log_size_per_component: Span<Option<u32>>,
        claimed_sum_per_component: Span<Option<QM31>>,
        common_lookup_elements: @CommonLookupElements,
    ) -> BuiltinComponents {
        assert!(
            (*log_size_per_component.at(PEDERSEN_BUILTIN_NARROW_WINDOWS_COMPONENT_IDX)).is_none()
                && (*claimed_sum_per_component.at(PEDERSEN_BUILTIN_NARROW_WINDOWS_COMPONENT_IDX))
                    .is_none(),
        );

        BuiltinComponents {
            add_mod_builtin: components::add_mod_builtin::NewComponentImpl::try_new(
                log_size_per_component.at(ADD_MOD_BUILTIN_COMPONENT_IDX),
                claimed_sum_per_component.at(ADD_MOD_BUILTIN_COMPONENT_IDX),
                common_lookup_elements,
            ),
            bitwise_builtin: components::bitwise_builtin::NewComponentImpl::try_new(
                log_size_per_component.at(BITWISE_BUILTIN_COMPONENT_IDX),
                claimed_sum_per_component.at(BITWISE_BUILTIN_COMPONENT_IDX),
                common_lookup_elements,
            ),
            mul_mod_builtin: components::mul_mod_builtin::NewComponentImpl::try_new(
                log_size_per_component.at(MUL_MOD_BUILTIN_COMPONENT_IDX),
                claimed_sum_per_component.at(MUL_MOD_BUILTIN_COMPONENT_IDX),
                common_lookup_elements,
            ),
            pedersen_builtin: components::pedersen_builtin::NewComponentImpl::try_new(
                log_size_per_component.at(PEDERSEN_BUILTIN_COMPONENT_IDX),
                claimed_sum_per_component.at(PEDERSEN_BUILTIN_COMPONENT_IDX),
                common_lookup_elements,
            ),
            poseidon_builtin: components::poseidon_builtin::NewComponentImpl::try_new(
                log_size_per_component.at(POSEIDON_BUILTIN_COMPONENT_IDX),
                claimed_sum_per_component.at(POSEIDON_BUILTIN_COMPONENT_IDX),
                common_lookup_elements,
            ),
            range_check_96_builtin: components::range_check96_builtin::NewComponentImpl::try_new(
                log_size_per_component.at(RANGE_CHECK96_BUILTIN_COMPONENT_IDX),
                claimed_sum_per_component.at(RANGE_CHECK96_BUILTIN_COMPONENT_IDX),
                common_lookup_elements,
            ),
            range_check_128_builtin: components::range_check_builtin::NewComponentImpl::try_new(
                log_size_per_component.at(RANGE_CHECK_BUILTIN_COMPONENT_IDX),
                claimed_sum_per_component.at(RANGE_CHECK_BUILTIN_COMPONENT_IDX),
                common_lookup_elements,
            ),
            ec_op_builtin: components::ec_op_builtin::NewComponentImpl::try_new(
                log_size_per_component.at(EC_OP_BUILTIN_COMPONENT_IDX),
                claimed_sum_per_component.at(EC_OP_BUILTIN_COMPONENT_IDX),
                common_lookup_elements,
            ),
        }
    }

    fn evaluate_constraints_at_point(
        self: @BuiltinComponents,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        public_data: @PublicData,
    ) {
        let BuiltinComponents {
            add_mod_builtin,
            bitwise_builtin,
            mul_mod_builtin,
            pedersen_builtin,
            poseidon_builtin,
            range_check_96_builtin,
            range_check_128_builtin,
            ec_op_builtin,
        } = self;

        if let Option::Some(component) = add_mod_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    [*public_data.public_memory.public_segments.add_mod.start_ptr.value].span(),
                );
        }

        if let Option::Some(component) = bitwise_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    [*public_data.public_memory.public_segments.bitwise.start_ptr.value].span(),
                );
        }

        if let Option::Some(component) = mul_mod_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    [*public_data.public_memory.public_segments.mul_mod.start_ptr.value].span(),
                );
        }

        if let Option::Some(component) = pedersen_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    [*public_data.public_memory.public_segments.pedersen.start_ptr.value].span(),
                );
        }

        if let Option::Some(component) = poseidon_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    [*public_data.public_memory.public_segments.poseidon.start_ptr.value].span(),
                );
        }

        if let Option::Some(component) = range_check_96_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    [*public_data.public_memory.public_segments.range_check_96.start_ptr.value]
                        .span(),
                );
        }

        if let Option::Some(component) = range_check_128_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    [*public_data.public_memory.public_segments.range_check_128.start_ptr.value]
                        .span(),
                );
        }

        if let Option::Some(component) = ec_op_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    [*public_data.public_memory.public_segments.ec_op.start_ptr.value].span(),
                );
        }
    }
}

#[generate_trait]
#[cfg(and(feature: "poseidon252_verifier", not(feature: "poseidon_outputs_packing")))]
pub impl BuiltinComponentsImpl of BuiltinComponentsTrait {
    fn new(
        log_size_per_component: Span<Option<u32>>,
        claimed_sum_per_component: Span<Option<QM31>>,
        common_lookup_elements: @CommonLookupElements,
    ) -> BuiltinComponents {
        assert!(
            (*log_size_per_component.at(RANGE_CHECK96_BUILTIN_COMPONENT_IDX)).is_none()
                && (*claimed_sum_per_component.at(RANGE_CHECK96_BUILTIN_COMPONENT_IDX)).is_none(),
        );
        assert!(
            (*log_size_per_component.at(ADD_MOD_BUILTIN_COMPONENT_IDX)).is_none()
                && (*claimed_sum_per_component.at(ADD_MOD_BUILTIN_COMPONENT_IDX)).is_none(),
        );
        assert!(
            (*log_size_per_component.at(MUL_MOD_BUILTIN_COMPONENT_IDX)).is_none()
                && (*claimed_sum_per_component.at(MUL_MOD_BUILTIN_COMPONENT_IDX)).is_none(),
        );
        assert!(
            (*log_size_per_component.at(PEDERSEN_BUILTIN_COMPONENT_IDX)).is_none()
                && (*claimed_sum_per_component.at(PEDERSEN_BUILTIN_COMPONENT_IDX)).is_none(),
        );
        assert!(
            (*log_size_per_component.at(PEDERSEN_BUILTIN_NARROW_WINDOWS_COMPONENT_IDX)).is_none()
                && (*claimed_sum_per_component.at(PEDERSEN_BUILTIN_NARROW_WINDOWS_COMPONENT_IDX))
                    .is_none(),
        );
        assert!(
            (*log_size_per_component.at(POSEIDON_BUILTIN_COMPONENT_IDX)).is_none()
                && (*claimed_sum_per_component.at(POSEIDON_BUILTIN_COMPONENT_IDX)).is_none(),
        );
        assert!(
            (*log_size_per_component.at(EC_OP_BUILTIN_COMPONENT_IDX)).is_none()
                && (*claimed_sum_per_component.at(EC_OP_BUILTIN_COMPONENT_IDX)).is_none(),
        );

        BuiltinComponents {
            bitwise_builtin: components::bitwise_builtin::NewComponentImpl::try_new(
                log_size_per_component.at(BITWISE_BUILTIN_COMPONENT_IDX),
                claimed_sum_per_component.at(BITWISE_BUILTIN_COMPONENT_IDX),
                common_lookup_elements,
            ),
            range_check_128_builtin: components::range_check_builtin::NewComponentImpl::try_new(
                log_size_per_component.at(RANGE_CHECK_BUILTIN_COMPONENT_IDX),
                claimed_sum_per_component.at(RANGE_CHECK_BUILTIN_COMPONENT_IDX),
                common_lookup_elements,
            ),
        }
    }

    fn evaluate_constraints_at_point(
        self: @BuiltinComponents,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        public_data: @PublicData,
    ) {
        let BuiltinComponents { bitwise_builtin, range_check_128_builtin } = self;

        if let Option::Some(component) = bitwise_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    [*public_data.public_memory.public_segments.bitwise.start_ptr.value].span(),
                );
        }

        if let Option::Some(component) = range_check_128_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    [*public_data.public_memory.public_segments.range_check_128.start_ptr.value]
                        .span(),
                );
        }
    }
}

#[generate_trait]
#[cfg(and(feature: "poseidon252_verifier", feature: "poseidon_outputs_packing"))]
pub impl BuiltinComponentsImpl of BuiltinComponentsTrait {
    fn new(
        log_size_per_component: Span<Option<u32>>,
        claimed_sum_per_component: Span<Option<QM31>>,
        common_lookup_elements: @CommonLookupElements,
    ) -> BuiltinComponents {
        assert!(
            (*log_size_per_component.at(RANGE_CHECK96_BUILTIN_COMPONENT_IDX)).is_none()
                && (*claimed_sum_per_component.at(RANGE_CHECK96_BUILTIN_COMPONENT_IDX)).is_none(),
        );
        assert!(
            (*log_size_per_component.at(ADD_MOD_BUILTIN_COMPONENT_IDX)).is_none()
                && (*claimed_sum_per_component.at(ADD_MOD_BUILTIN_COMPONENT_IDX)).is_none(),
        );
        assert!(
            (*log_size_per_component.at(MUL_MOD_BUILTIN_COMPONENT_IDX)).is_none()
                && (*claimed_sum_per_component.at(MUL_MOD_BUILTIN_COMPONENT_IDX)).is_none(),
        );
        assert!(
            (*log_size_per_component.at(PEDERSEN_BUILTIN_COMPONENT_IDX)).is_none()
                && (*claimed_sum_per_component.at(PEDERSEN_BUILTIN_COMPONENT_IDX)).is_none(),
        );
        assert!(
            (*log_size_per_component.at(PEDERSEN_BUILTIN_NARROW_WINDOWS_COMPONENT_IDX)).is_none()
                && (*claimed_sum_per_component.at(PEDERSEN_BUILTIN_NARROW_WINDOWS_COMPONENT_IDX))
                    .is_none(),
        );
        assert!(
            (*log_size_per_component.at(EC_OP_BUILTIN_COMPONENT_IDX)).is_none()
                && (*claimed_sum_per_component.at(EC_OP_BUILTIN_COMPONENT_IDX)).is_none(),
        );

        BuiltinComponents {
            bitwise_builtin: components::bitwise_builtin::NewComponentImpl::try_new(
                log_size_per_component.at(BITWISE_BUILTIN_COMPONENT_IDX),
                claimed_sum_per_component.at(BITWISE_BUILTIN_COMPONENT_IDX),
                common_lookup_elements,
            ),
            poseidon_builtin: components::poseidon_builtin::NewComponentImpl::try_new(
                log_size_per_component.at(POSEIDON_BUILTIN_COMPONENT_IDX),
                claimed_sum_per_component.at(POSEIDON_BUILTIN_COMPONENT_IDX),
                common_lookup_elements,
            ),
            range_check_128_builtin: components::range_check_builtin::NewComponentImpl::try_new(
                log_size_per_component.at(RANGE_CHECK_BUILTIN_COMPONENT_IDX),
                claimed_sum_per_component.at(RANGE_CHECK_BUILTIN_COMPONENT_IDX),
                common_lookup_elements,
            ),
        }
    }

    fn evaluate_constraints_at_point(
        self: @BuiltinComponents,
        ref sum: QM31,
        ref preprocessed_mask_values: PreprocessedMaskValues,
        ref trace_mask_values: ColumnSpan<Span<QM31>>,
        ref interaction_trace_mask_values: ColumnSpan<Span<QM31>>,
        random_coeff: QM31,
        public_data: @PublicData,
    ) {
        let BuiltinComponents { bitwise_builtin, poseidon_builtin, range_check_128_builtin } = self;

        if let Option::Some(component) = bitwise_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    [*public_data.public_memory.public_segments.bitwise.start_ptr.value].span(),
                );
        }

        if let Option::Some(component) = poseidon_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    [*public_data.public_memory.public_segments.poseidon.start_ptr.value].span(),
                );
        }

        if let Option::Some(component) = range_check_128_builtin.as_snap() {
            component
                .evaluate_constraints_at_point(
                    ref sum,
                    ref preprocessed_mask_values,
                    ref trace_mask_values,
                    ref interaction_trace_mask_values,
                    random_coeff,
                    [*public_data.public_memory.public_segments.range_check_128.start_ptr.value]
                        .span(),
                );
        }
    }
}
